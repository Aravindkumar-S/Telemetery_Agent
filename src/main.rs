use axum::{routing::get, Json, Router};
use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
struct SystemMetrics {
    os_name: String,
    total_memory_kb: u64,
    used_memory_kb: u64,
    cpu_usage_percentage: f32,
    total_processes: usize,
}

// Asynchronous handler to pull cross-platform telemetry natively
async fn get_metrics() -> Json<SystemMetrics> {
    let mut sys = System::new_all();
    
    // Refresh system metrics inside an async context
    sys.refresh_all();
    
    // Sleep momentarily to let sysinfo calculate a realistic delta differential for CPU usage
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    sys.refresh_cpu_usage();

    let metrics = SystemMetrics {
        os_name: System::name().unwrap_or_else(|| "Unknown OS".to_string()),
        total_memory_kb: sys.total_memory(),
        used_memory_kb: sys.used_memory(),
        cpu_usage_percentage: sys.global_cpu_info().cpu_usage(),
        total_processes: sys.processes().len(),
    };

    Json(metrics)
}

#[tokio::main]
async fn main() {
    // 1. Set up routing layers
    let app = Router::new()
        .route("/api/v1/telemetry", get(get_metrics));

    // 2. Bind the server internally to localhost (Least-privilege concept)
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();
        
    println!("🛡️ Citadel Telemetry Agent running on http://127.0.0.1:4000");
    println!("🔒 Security Baseline: Enforcing secure local REST loopback access.");

    // 3. Initialize the server runtime
    axum::serve(listener, app).await.unwrap();
}
