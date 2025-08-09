// main.rs
mod cli;
mod config;
mod domain;
mod engine;
mod client;
mod metrics_writer;

use clap::Parser;
use env_logger::Env;
use cli::CliArgs;
use config::load_config;
use engine::modos::ejecutar_modo;
use engine::metrics::{Metrics, run_metrics_server};
use std::sync::Arc;
use tokio::sync::Mutex;
use metrics_writer::save_metrics;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args = CliArgs::parse();
    let config = load_config(&args).expect("❌ No se pudo cargar la configuración");

    println!("✅ Configuración cargada: {:#?}", config);
    println!("🚀 Iniciando StressTestBlue con modo: {}", config.mode);

    let metrics = Arc::new(Mutex::new(Metrics::default()));
    let metrics_clone = metrics.clone();
    tokio::spawn(async move {
        if let Err(e) = run_metrics_server(metrics_clone, 9090).await {
            eprintln!("❌ Error al iniciar servidor de métricas: {:?}", e);
        }
    });

    ejecutar_modo(config, metrics.clone()).await;

    println!("🧪 Finalizando test. Guardando resultados...");
    let final_metrics = metrics.lock().await.clone();
    save_metrics(&final_metrics).expect("❌ No se pudo guardar resultados");
}
