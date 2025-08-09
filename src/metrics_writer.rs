use crate::engine::metrics::Metrics;
use std::{fs::File, io::Write};
use chrono::Utc;

pub fn save_metrics(metrics: &Metrics) -> std::io::Result<()> {
    let timestamp = Utc::now().format("%Y%m%d%H%M%S");
    let json_path = format!("results_{}.json", timestamp);
    let csv_path = format!("results_{}.csv", timestamp);

    // Guardar JSON
    let json = serde_json::to_string_pretty(&metrics)?;
    let mut json_file = File::create(json_path)?;
    json_file.write_all(json.as_bytes())?;

    // Guardar CSV simple
    let mut csv_file = File::create(csv_path)?;
    writeln!(csv_file, "code,count")?;
    for (code, count) in &metrics.status_codes {
        writeln!(csv_file, "{},{}", code, count)?;
    }

    Ok(())
}
