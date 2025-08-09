//! Módulo que interpreta argumentos CLI o YAML y genera la configuración central.

use crate::cli::CliArgs;
use crate::domain::test_config::TestConfig;
use std::fs;

/// Carga configuración desde YAML o CLI
pub fn load_config(args: &CliArgs) -> Result<TestConfig, String> {
    if let Some(path) = &args.config {
        let content = fs::read_to_string(path).map_err(|e| format!("Error leyendo archivo: {}", e))?;
        let mut config: TestConfig = serde_yaml::from_str(&content).map_err(|e| format!("YAML inválido: {}", e))?;

        // CLI sobreescribe valores YAML si están presentes
        if let Some(rps) = args.rps {
            config.rps = Some(rps);
        }

        Ok(config)
    } else {
        Ok(TestConfig {
            mode: args.mode.clone(),
            endpoint: args.endpoint.clone().ok_or("Falta el parámetro --endpoint")?,
            concurrency: args.concurrency.unwrap_or(10),
            duration: args.duration.unwrap_or(10),
            method: args.method.clone().unwrap_or_else(|| "GET".to_string()),
            body: args.body.clone(),
            headers: args.headers.clone(),
            rps: args.rps, // ✅ CLI sin YAML
        })
    }
}
