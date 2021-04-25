use crate::error::ApathyError;

use hyper::{
	body::{self, Buf},
	client::HttpConnector,
	http::HeaderValue,
	Body,
	Request,
};
use hyper_tls::HttpsConnector;
use std::str::FromStr;

fn create_client() -> hyper::Client<HttpsConnector<HttpConnector>> {
	let https = HttpsConnector::new();

	hyper::Client::builder().build::<_, Body>(https)
}

async fn request_header(request: Request<Body>, header: &str) -> Result<HeaderValue, ApathyError> {
	let response = create_client().request(request).await?;
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

async fn get_auth_ticket(cookie: String) -> Result<HeaderValue, ApathyError> {
	let csrf_token = request_header(
		Request::builder()
			.method(hyper::Method::POST)
			.uri("https://auth.roblox.com/v2/logout/")
			.header("Content-Length", 0)
			.header("Cookie", &cookie)
			.body(Body::empty())?,
		"X-CSRF-Token",
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

pub async fn get_launch_info(token: String) -> Result<String, ApathyError> {
	let uri = format!("https://eclipsis-alt-gen.herokuapp.com/getToken/{}", token);
	let response = create_client().get(hyper::Uri::from_str(&uri)?).await?;

	let content = body::aggregate(response).await?;
	let result: Popcorn = serde_json::from_reader(content.reader())?;

	if !result.success {
		return Err(ApathyError::PopcornError(result.error));
	}

	let auth_ticket = get_auth_ticket(format!(".ROBLOSECURITY={}", result.cookie)).await?;
	let url_components = vec![
        String::from("roblox-player:1"),
        String::from("launchmode:play"),
        format!("gameinfo:{}", auth_ticket.to_str()?),
        format!("launchtime:{}", chrono::Utc::now().timestamp_millis()),
        format!("placelauncherurl:https://assetgame.roblox.com/game/PlaceLauncher.ashx?request=RequestGame&browserTrackerId=1147338376&placeId={}&isPlayTogetherGame=true", result.place_id),
        String::from("browsertrackerid:1480614826"),
        String::from("robloxLocale:en_us"),
        String::from("gameLocale:en_us"),
        String::from("channel:")
    ];

	Ok(url_components.join("+"))
}
