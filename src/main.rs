pub mod error;
mod utility;

use utility::{multiple_roblox, request};
use warp::Filter;

async fn launch_roblox(token: String) -> Result<impl warp::Reply, warp::Rejection> {
	println!("Getting authentication ticket...");

	Ok(match request::launch_roblox(token).await {
		Ok(_) => {
			println!("Roblox has launched successfully!");

			"Success! You may now close this tab."
		},
		Err(_) =>
			"Something went wrong when trying to open Roblox. Please see the console for more \
			 information.",
	})
}

#[tokio::main]
#[allow(unused_must_use)]
async fn main() -> Result<(), warp::Rejection> {
	multiple_roblox::start();

	let favicon = warp::path!("favicon.ico").map(|| "");
	let default = warp::get().and(warp::path::param()).and_then(launch_roblox);
	let handler =
		warp::any().map(|| "You're in the wrong place; please use the URL provided by the bot.");

	println!("Apathy Client listening on 127.0.0.1:3000");
	warp::serve(favicon.or(default).or(handler))
		.run(([127, 0, 0, 1], 3000))
		.await;

	Ok(())
}
