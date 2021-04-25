pub mod error;
mod utility;

use utility::{multiple_roblox, request};
use warp::Filter;

async fn launch_roblox(token: String) -> Result<impl warp::Reply, warp::Rejection> {
	let page = format!(
		r#"
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8" />
                <meta http-equiv="X-UA-Compatible" content="IE=edge" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />

                <title>Apathy Popcorn Generator</title>
                <script async>
                    window.location.href = "{launch_info}"
                </script>
            </head>
            <body>
                <p>If you are not redirected, please click <a href="{launch_info}">here</a>.</p>
            </body>
        </html>
        "#,
		launch_info = match request::get_launch_info(token).await {
			Ok(launch_info) => launch_info,
			Err(err) => err.to_string(),
		}
	);

	Ok(warp::reply::html(page))
}

#[tokio::main]
#[allow(unused_must_use)]
async fn main() -> Result<(), warp::Rejection> {
	// multiple_roblox::setup();

	let default = warp::get().and(warp::path::param()).and_then(launch_roblox);
	let handler =
		warp::any().map(|| "You're in the wrong place; please use the URL provided by the bot.");

	warp::serve(default.or(handler))
		.run(([127, 0, 0, 1], 3000))
		.await;

	Ok(())
}
