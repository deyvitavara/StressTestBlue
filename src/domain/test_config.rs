//! Modelo principal de configuración de prueba.

use serde::Deserialize;

/// Representa la configuración completa para ejecutar una prueba.
/// Puede ser obtenida vía CLI o archivo YAML.
#[derive(Debug, Clone, Deserialize)]
pub struct TestConfig {
    /// URL completa del endpoint a probar (ej. https://api.ejemplo.com/data)
    pub endpoint: String,

    /// Número de tareas concurrentes que simulan usuarios simultáneos
    pub concurrency: usize,

    /// Duración total de la prueba (en segundos)
    pub duration: u64,

    /// Método HTTP: GET, POST, PUT, etc.
    pub method: String,

    /// Cuerpo opcional de la solicitud (JSON u otro formato)
    pub body: Option<String>,

    /// Headers personalizados en formato string (ej. Authorization: Bearer ABC123)
    pub headers: Option<String>,

    /// Modo de prueba: load, stress, spike, soak, concurrency
    pub mode: String,

    /// Requests por segundo deseados (opcional)
    pub rps: Option<u64>,
}
