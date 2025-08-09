//! Módulo CLI: define los argumentos de línea de comandos usando `clap`.

use clap::Parser;

/// Argumentos que puede recibir StressTestBlue desde CLI.
#[derive(Parser, Debug)]
#[command(
    name = "StressTestBlue",
    version = "0.1.0",
    about = "🧪 Herramienta profesional para pruebas de carga y rendimiento de APIs REST",
    long_about = None
)]
pub struct CliArgs {
    /// Ruta a un archivo de configuración YAML
    #[arg(long)]
    pub config: Option<String>,

    /// Endpoint a probar
    #[arg(long)]
    pub endpoint: Option<String>,

    /// Número de tareas concurrentes
    #[arg(long)]
    pub concurrency: Option<usize>,

    /// Duración en segundos
    #[arg(long)]
    pub duration: Option<u64>,

    /// Método HTTP (GET, POST, PUT, etc.)
    #[arg(long)]
    pub method: Option<String>,

    /// Cuerpo de la petición (JSON string)
    #[arg(long)]
    pub body: Option<String>,

    /// Headers personalizados separados por coma (ej. Authorization: Bearer abc, X-Key: 123)
    #[arg(long)]
    pub headers: Option<String>,

    /// Modo de prueba: load, stress, spike, soak, concurrency
    #[arg(long, default_value = "load")]
    pub mode: String,

    /// Requests por segundo (RPS) - opcional
    #[arg(long)]
    pub rps: Option<u64>,
}
