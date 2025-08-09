use axum::{
    routing::get,
    response::IntoResponse,
    Router,
};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use serde::Serialize;

pub type SharedMetrics = Arc<Mutex<Metrics>>;

#[derive(Default, Clone, Serialize)]
pub struct Metrics {
    pub total_requests: u64,
    pub errors: u64,
    pub status_codes: HashMap<u16, u64>,
}

pub fn with_metrics_router(metrics: SharedMetrics) -> Router {
    Router::new().route("/metrics", get(move || {
        let metrics = metrics.clone();
        async move { get_metrics(metrics).await }
    }))
}

async fn get_metrics(shared: SharedMetrics) -> impl IntoResponse {
    let m = shared.lock().await;
    let mut output = String::new();

    output += &format!("http_requests_total {}\n", m.total_requests);
    output += &format!("http_errors_total {}\n", m.errors);
    for (code, count) in &m.status_codes {
        output += &format!("http_response_code_{} {}\n", code, count);
    }

    output
}

pub async fn run_metrics_server(metrics: SharedMetrics, port: u16) -> Result<(), std::io::Error> {
    let app = with_metrics_router(metrics);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("📊 Metrics server corriendo en http://{}/metrics", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}



