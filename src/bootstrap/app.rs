use anyhow::{anyhow, Result};
use axum::{middleware, Router};
use log::info;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

pub async fn app() -> anyhow::Result<()> {
	let routes = Router::new().layer(CookieManagerLayer::new());

	// TODO Make this configurable.
	let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
	info!("Server started on {:?}", listener.local_addr());

	axum::serve(listener, routes.into_make_service())
		.await
		.map_err(|_| anyhow!("Error occured when running axum server"))
}
