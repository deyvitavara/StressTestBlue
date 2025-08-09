use std::sync::Arc;
use tokio::sync::Mutex;

use crate::domain::test_config::TestConfig;
use crate::engine::{load, soak, spike, stress, concurrency, metrics::Metrics};

/// Alias para compartir métricas entre hilos/tasks
pub type SharedMetrics = Arc<Mutex<Metrics>>;

/// Ejecuta el modo de prueba especificado en la configuración
pub async fn ejecutar_modo(config: TestConfig, metrics: SharedMetrics) {
    let modo = config.mode.to_lowercase();

    match modo.as_str() {
        "load" => load::run(&config, metrics.clone()).await,
        "spike" => spike::run(&config, metrics.clone()).await,
        "soak" => soak::run(&config, metrics.clone()).await,
        "stress" => stress::run(&config, metrics.clone()).await,
        "concurrency" => concurrency::run(&config, metrics.clone()).await,
        _ => eprintln!("❌ Modo desconocido: {}", modo),
    }
}
