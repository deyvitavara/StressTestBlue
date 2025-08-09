use once_cell::sync::Lazy;
use reqwest::Client;
use std::time::Duration;

/// Cliente HTTP global reutilizable con configuración estándar
pub static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Error creando cliente HTTP")
});
