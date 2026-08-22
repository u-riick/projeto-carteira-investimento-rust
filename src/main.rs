mod routes;
mod app;
mod models;
mod auth;
mod error;
mod repository;
use crate::app::App;


#[tokio::main]

async fn main() -> color_eyre::Result<()> {
    App::start().await
}

