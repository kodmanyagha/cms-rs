use anyhow::Result;
use axum::extract::{Path, Query};
use axum::http::{Method, Uri};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Json, Router};
use bootstrap::app::{self, app};
use dotenvy::dotenv;
use log::error;
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod bootstrap;

#[tokio::main]
async fn main() -> Result<()> {
	dotenv().expect(".env file not found");

	env_logger::init();

	app().await.map_err(|err| {
		error!("An error occured: {:?}", err);

		err
	})
}
