use crate::domain::test_config::TestConfig;
use crate::engine::rest::send_request;
use crate::engine::metrics::SharedMetrics;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::task;
use tokio::time::sleep;
use log::{info, error};

pub async fn run(config: &TestConfig, metrics: SharedMetrics) {
    info!("🚨 Ejecutando Stress Test...");

    let config = Arc::new(config.clone());
    let start = Instant::now();
    let delay = Duration::from_millis(1000 / config.rps.unwrap_or(100).max(1));

    while start.elapsed().as_secs() < config.duration {
        let cfg = config.clone();
        let metrics = metrics.clone();

        task::spawn(async move {
            match send_request(&cfg).await {
                Ok(status) => {
                    let mut m = metrics.lock().await;
                    m.total_requests += 1;
                    *m.status_codes.entry(status.as_u16()).or_insert(0) += 1;
                }
                Err(e) => {
                    error!("Error: {}", e);
                    let mut m = metrics.lock().await;
                    m.errors += 1;
                }
            }
        });

        sleep(delay).await;
    }

    sleep(Duration::from_secs(2)).await;
}
