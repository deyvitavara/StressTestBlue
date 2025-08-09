use crate::domain::test_config::TestConfig;
use crate::engine::{load, stress, spike, soak, concurrency, metrics::SharedMetrics};

/// Despacha al módulo correspondiente según el modo.
/// Todas las funciones `run` deben ser async y llamadas con `.await`.
pub async fn dispatch(config: TestConfig, metrics: SharedMetrics) -> Result<(), String> {
    match config.mode.to_lowercase().as_str() {
        "load" => {
            load::run(&config, metrics.clone()).await;
            Ok(())
        }
        "stress" => {
            stress::run(&config, metrics.clone()).await;
            Ok(())
        }
        "spike" => {
            spike::run(&config, metrics.clone()).await;
            Ok(())
        }
        "soak" => {
            soak::run(&config, metrics.clone()).await;
            Ok(())
        }
        "concurrency" => {
            concurrency::run(&config, metrics.clone()).await;
            Ok(())
        }
        _ => Err(format!("❌ Modo desconocido: {}", config.mode)),
    }
}
