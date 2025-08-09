use crate::domain::test_config::TestConfig;
use crate::engine::rest::send_request;
use crate::engine::metrics::SharedMetrics;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::task;
use tokio::time::sleep;
use log::{info, error};

pub async fn run(config: &TestConfig, metrics: SharedMetrics) {
    info!("⚙️ Ejecutando Spike Test...");

    let config = Arc::new(config.clone());
    let steps = 10;
    let max_concurrency = config.concurrency;
    let delay = Duration::from_millis(1000 / config.rps.unwrap_or(100).max(1));
    let inicio = Instant::now();

    for i in 1..=steps {
        if inicio.elapsed().as_secs() >= config.duration {
            break;
        }

        let users = max_concurrency * i / steps;
        info!("🚀 Spike nivel {i}/{steps}: {users} usuarios");

        let mut handles = Vec::new();
        for _ in 0..users {
            let cfg = config.clone();
            let metrics = metrics.clone();

            handles.push(task::spawn(async move {
                match send_request(&cfg).await {
                    Ok(status) => {
                        let mut m = metrics.lock().await;
                        m.total_requests += 1;
                        *m.status_codes.entry(status.as_u16()).or_insert(0) += 1;
                    }
                    Err(e) => {
                        error!("Falló request: {}", e);
                        let mut m = metrics.lock().await;
                        m.errors += 1;
                    }
                }
            }));

            sleep(delay).await;
        }

        for h in handles {
            let _ = h.await;
        }
    }

    info!("✅ Spike Test finalizado.");
}
