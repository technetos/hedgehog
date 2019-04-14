#![feature(async_await, futures_api)]

use hedgehog::ContextExt;
use tide::{response::IntoResponse, Context, EndpointResult};

async fn echo_path(cx: Context<()>) -> EndpointResult {
    cx.hedgehog().protect()?;

    let path: String = cx
        .param("path")
        .map_err(|_| "Expected path param".into_response())?;
    Ok(format!("Your path is: {}", path).into_response())
}

fn main() {
    let mut app = tide::App::new(());
    app.at("/echo_path/*path").get(echo_path);
    app.serve("127.0.0.1:8000").unwrap();
}
