use crate::domain::test_config::TestConfig;
use crate::engine::rest::send_request;
use crate::engine::metrics::SharedMetrics;
//use std::sync::{Arc, Mutex};
use std::sync::Arc;
//use tokio::sync::Mutex;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tokio::task;
use tokio::time::sleep;
use log::{info, error};

pub async fn run(config: &TestConfig, metrics: SharedMetrics) {
    info!("🔄 Ejecutando Concurrency Test...");

    let config = Arc::new(config.clone());
    let start = Instant::now();
    let semaphore = Arc::new(Semaphore::new(config.concurrency));
    let delay = Duration::from_millis(1000 / config.rps.unwrap_or(50).max(1));

    while start.elapsed().as_secs() < config.duration {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
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
                    error!("Request fallido: {}", e);
                    let mut m = metrics.lock().await;
                    m.errors += 1;
                }
            }
            drop(permit);
        });

        sleep(delay).await;
    }

    sleep(Duration::from_secs(2)).await;
}
