use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<RwLock<crate::config::Config>>,
}

#[derive(Serialize, Deserialize)]
pub struct CommandRequest {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Serialize)]
pub struct CommandResponse {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let config = crate::config::Config::load().unwrap_or_default();
    let state = AppState {
        config: Arc::new(RwLock::new(config)),
    };

    let app = Router::new()
        .route("/", get(dashboard_handler))
        .route("/api/system-info", get(api_system_info))
        .route("/api/env-vars", get(api_env_vars))
        .route("/api/execute", post(api_execute_command))
        .route("/api/config", get(api_get_config))
        .route("/api/config", post(api_update_config))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("\n{}", "=".repeat(80).cyan());
    println!("{}", "  Command Line Helper - Web Dashboard".cyan().bold());
    println!("{}", "=".repeat(80).cyan());
    println!(
        "\n{} Server starting on {}",
        "→".cyan(),
        format!("http://localhost:{}", port).green().bold()
    );
    println!("\n{}", "Available endpoints:".yellow().bold());
    println!(
        "  {} Dashboard",
        format!("http://localhost:{}", port).green()
    );
    println!(
        "  {} System Info API",
        format!("http://localhost:{}/api/system-info", port).green()
    );
    println!(
        "  {} Environment Variables API",
        format!("http://localhost:{}/api/env-vars", port).green()
    );
    println!(
        "\n{} Press Ctrl+C to stop the server\n",
        "Info:".yellow().bold()
    );

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn dashboard_handler() -> Html<String> {
    let html = include_str!("../../web/index.html");
    Html(html.to_string())
}

async fn api_system_info() -> Json<serde_json::Value> {
    use sysinfo::System;

    let mut sys = System::new_all();
    sys.refresh_all();

    let info = serde_json::json!({
        "os": {
            "name": System::name().unwrap_or_else(|| "Unknown".to_string()),
            "version": System::os_version().unwrap_or_else(|| "Unknown".to_string()),
            "kernel": System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
        },
        "cpu": {
            "count": sys.cpus().len(),
            "brand": sys.cpus().first().map(|cpu| cpu.brand()).unwrap_or("Unknown"),
        },
        "memory": {
            "total": sys.total_memory(),
            "used": sys.used_memory(),
            "available": sys.available_memory(),
        }
    });

    Json(info)
}

async fn api_env_vars() -> Json<serde_json::Value> {
    use std::env;

    let vars: std::collections::HashMap<String, String> = env::vars().collect();
    Json(serde_json::json!(vars))
}

async fn api_execute_command(
    Json(payload): Json<CommandRequest>,
) -> Result<Json<CommandResponse>, StatusCode> {
    // For security, we'll only allow specific safe commands
    let allowed_commands = vec!["system-info", "env-list", "hash-file", "text-stats"];

    if !allowed_commands.contains(&payload.command.as_str()) {
        return Ok(Json(CommandResponse {
            success: false,
            output: String::new(),
            error: Some("Command not allowed via web interface".to_string()),
        }));
    }

    // Execute command logic here
    Ok(Json(CommandResponse {
        success: true,
        output: format!(
            "Executed: {} with args: {:?}",
            payload.command, payload.args
        ),
        error: None,
    }))
}

async fn api_get_config(State(state): State<AppState>) -> Json<crate::config::Config> {
    let config = state.config.read().await;
    Json(config.clone())
}

async fn api_update_config(
    State(state): State<AppState>,
    Json(new_config): Json<crate::config::Config>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut config = state.config.write().await;
    *config = new_config.clone();

    if let Err(e) = config.save() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json(serde_json::json!({"success": true})))
}
