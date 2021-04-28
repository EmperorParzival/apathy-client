use crate::error::ApathyError;

use hyper::{
	body::{self, Buf},
	http::HeaderValue,
	Body,
	Request,
};
use std::{process::Command, str::FromStr};

type Client = hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>;

async fn request_header(
	request: Request<Body>,
	header: &str,
	client: Client,
) -> Result<HeaderValue, ApathyError> {
	let response = client.request(request).await?;
	let headers = response.headers();

	if headers.contains_key(header) {
		Ok(headers[header].clone())
	} else {
		Err(ApathyError::PopcornError(format!(
			"Could not read Roblox header {}",
			header
		)))
	}
}

async fn get_auth_ticket(cookie: String, client: Client) -> Result<HeaderValue, ApathyError> {
	let csrf_token = request_header(
		Request::builder()
			.method(hyper::Method::POST)
			.uri("https://auth.roblox.com/v2/logout/")
			.header("Content-Length", 0)
			.header("Cookie", &cookie)
			.body(Body::empty())?,
		"X-CSRF-Token",
		client.clone(),
	)
	.await?;

	request_header(
		Request::builder()
			.method(hyper::Method::POST)
			.uri("https://auth.roblox.com/v1/authentication-ticket")
			.header("Referer", "https://www.roblox.com")
			.header("X-CSRF-Token", csrf_token)
			.header("Content-Length", 0)
			.header("Cookie", cookie)
			.body(Body::empty())?,
		"rbx-authentication-ticket",
		client,
	)
	.await
}

#[derive(serde::Deserialize)]
struct Popcorn {
	success: bool,
	place_id: u32,
	error: String,
	cookie: String,
}

pub async fn launch_roblox(token: String) -> Result<(), ApathyError> {
	let https = hyper_tls::HttpsConnector::new();
	let client = hyper::Client::builder().build::<_, Body>(https);

	let token_uri = format!("https://eclipsis-alt-gen.herokuapp.com/getToken/{}", token);
	let token_resp = client.get(hyper::Uri::from_str(&token_uri)?).await?;
	let token: Popcorn = serde_json::from_reader(body::aggregate(token_resp).await?.reader())?;

	// NOTE: assert!() causes unnecessary errors when  multiple requests?? rust?????
	// assert!(token.success, "{}", token.error);

	if !token.success {
		return Err(ApathyError::PopcornError(token.error));
	}

	let version_uri = "http://setup.roblox.com/version.txt";
	let version_resp = client.get(hyper::Uri::from_static(version_uri)).await?;
	let version = String::from_utf8(body::to_bytes(version_resp).await?.to_vec())?;

	let mut roblox_dir = dirs::data_local_dir().expect("Failed to read AppData\\Local");
	roblox_dir.push(format!(
		"Roblox\\Versions\\{}\\RobloxPlayerBeta.exe",
		version
	));

	assert!(
		roblox_dir.is_file(),
		"Failed to find launcher - is your roblox updated?"
	);

	Command::new(roblox_dir.into_os_string())
		.args(&[
			"--play",
			"-a",
			"https://auth.roblox.com/v1/authentication-ticket/redeem",
			"-t",
			get_auth_ticket(format!(".ROBLOSECURITY={}", token.cookie), client).await?.to_str()?,
			"-j",
			&format!("https://assetgame.roblox.com/game/PlaceLauncher.ashx?request=RequestGame&browserTrackerId=1147338376&placeId={}&isPlayTogetherGame=true", token.place_id),
			"-b",
			"1147338376",
			"--launchtime",
			&chrono::Utc::now().timestamp_millis().to_string(),
			"--rloc",
			"en_us",
			"--gloc",
			"en_us",
		])
		.spawn()
		.expect("Failed to launch RobloxPlayerBeta.exe");

	Ok(())
}
