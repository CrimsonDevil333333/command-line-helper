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
    // Expanded whitelist of allowed commands
    let allowed_commands = vec![
        // System
        "system-info",
        "cpu-info",
        "memory-info",
        "disk-info",
        "network-info",
        // Env
        "env-list",
        "env-get",
        "env-set",
        "env-load",
        "env-export",
        // Hash
        "hash-file",
        "hash-string",
        "hash-verify",
        "hash-all",
        // Text
        "text-stats",
        "base64-encode",
        "base64-decode",
        "url-encode",
        "url-decode",
        "text-case",
        // Git
        "git-status",
        "git-clone",
        "git-branch",
        "git-add",
        "git-commit",
        "git-branches",
        "git-log",
        // Archive
        "zip-create",
        "zip-extract",
        "zip-list",
        "tar-create",
        "tar-extract",
        "tar-list",
        // Network
        "port-check",
        "ping",
        "public-ip",
        "http-get",
        "http-post",
        "dns-lookup",
        // Format
        "json-format",
        "json-minify",
        "json-validate",
        "yaml-format",
        "yaml-validate",
        "json-to-yaml",
        "yaml-to-json",
        "json-query",
    ];

    // Basic validation
    let cmd = payload.command.trim_start_matches("--");
    if !allowed_commands.contains(&cmd) {
        return Ok(Json(CommandResponse {
            success: false,
            output: String::new(),
            error: Some(format!(
                "Command '{}' not allowed via web interface",
                payload.command
            )),
        }));
    }

    // Get current executable path
    let current_exe = match std::env::current_exe() {
        Ok(exe) => exe,
        Err(e) => {
            return Ok(Json(CommandResponse {
                success: false,
                output: String::new(),
                error: Some(format!("Failed to get current executable: {}", e)),
            }))
        }
    };

    // Construct arguments
    let mut args = Vec::new();
    // Add the main command flag (e.g., --system-info or --git-status)
    if !payload.command.starts_with("--") {
        args.push(format!("--{}", payload.command));
    } else {
        args.push(payload.command.clone());
    }

    // Add any additional arguments
    args.extend(payload.args.clone());

    // Execute the command
    use std::process::Command;
    let output = Command::new(current_exe).args(&args).output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            let success = output.status.success();
            let result_output = if success {
                stdout
            } else {
                format!("{}\n{}", stdout, stderr)
            };
            let error = if !success { Some(stderr) } else { None };

            Ok(Json(CommandResponse {
                success,
                output: result_output.trim().to_string(),
                error,
            }))
        }
        Err(e) => Ok(Json(CommandResponse {
            success: false,
            output: String::new(),
            error: Some(format!("Failed to execute command: {}", e)),
        })),
    }
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
