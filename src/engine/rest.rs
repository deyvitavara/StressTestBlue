use crate::domain::test_config::TestConfig;
use crate::client::HTTP_CLIENT;
use reqwest::{Method, StatusCode};
use std::time::Instant;
use log::{info, warn, error};

/// Envía una solicitud HTTP según la configuración proporcionada.
pub async fn send_request(config: &TestConfig) -> Result<StatusCode, String> {
    // Determinar método HTTP
    let method = match config.method.to_uppercase().as_str() {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        other => {
            warn!("Método HTTP no soportado: {}. Usando GET por defecto.", other);
            Method::GET
        }
    };

    // Construir solicitud
    let mut request = HTTP_CLIENT.request(method, &config.endpoint);

    // Agregar headers si existen
    if let Some(headers_str) = &config.headers {
        for header in headers_str.split(',') {
            if let Some((key, value)) = header.trim().split_once(':') {
                request = request.header(key.trim(), value.trim());
            }
        }
    }

    // Agregar body si aplica
    if let Some(body) = &config.body {
        request = request.body(body.clone());
    }

    // Enviar y medir tiempo
    let start = Instant::now();
    let response = request.send().await.map_err(|e| {
        error!("Fallo al enviar request: {}", e);
        e.to_string()
    })?;
    let duration = start.elapsed();

    info!(
        "[{}] {} - {} ({}ms)",
        config.method.to_uppercase(),
        config.endpoint,
        response.status(),
        duration.as_millis()
    );

    Ok(response.status())
}
