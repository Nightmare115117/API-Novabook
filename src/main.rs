mod config;
mod db;
mod error;
mod middleware;
pub mod productos;
pub mod services;
pub mod usuarios;

use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::{
    config::Config,
    db::{create_pool, AppState},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env();
    println!("Iniciando API REST Librería NovaBook...");

    // Inicializar pool de base de datos MySQL con SQLx
    let pool = create_pool(&config.database_url).await?;
    println!("Conexión a MySQL establecida exitosamente en: {}", config.database_url);

    let state = AppState {
        pool,
        config: config.clone(),
    };

    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    // Ensamblaje modular de routers según la arquitectura del proyecto:
    // - usuarios/controller: /api/auth/login, /api/auth/perfil, /api/usuarios (CRUD)
    // - productos/controller: /api/bodega/*, /api/vendedor/*, /api/jefe/*
    let app = Router::new()
        .merge(usuarios::controller::router(state.clone()))
        .merge(productos::controller::router(state.clone()))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::new(
        config.server_host.parse().unwrap_or([0, 0, 0, 0].into()),
        config.server_port,
    );

    println!("Servidor REST NovaBook ejecutándose en http://{}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
