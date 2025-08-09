use serde::{Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use chrono::Utc;

#[derive(Serialize)]
pub struct TestResult {
    pub mode: String,
    pub duration: u64,
    pub total_requests: u64,
    pub status_codes: HashMap<u16, u64>,
    pub errors: u64,
    pub min_time_ms: Option<u64>,
    pub max_time_ms: Option<u64>,
    pub avg_time_ms: Option<f64>,
}

pub fn save_as_json(result: &TestResult, path: Option<&str>) -> Result<(), String> {
    let filename = path.unwrap_or_else(|| {
        let ts = Utc::now().format("%Y%m%d%H%M%S");
        Box::leak(format!("results_{}.json", ts).into_boxed_str())
    });
    
    let json = serde_json::to_string_pretty(result).map_err(|e| e.to_string())?;
    std::fs::write(filename, json).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn save_as_csv(result: &TestResult, path: Option<&str>) -> Result<(), String> {
    let filename = path.unwrap_or_else(|| {
        let ts = Utc::now().format("%Y%m%d%H%M%S");
        Box::leak(format!("results_{}.csv", ts).into_boxed_str())
    });

    let mut file = File::create(filename).map_err(|e| e.to_string())?;
    writeln!(file, "Status Code,Count").map_err(|e| e.to_string())?;
    for (code, count) in &result.status_codes {
        writeln!(file, "{},{}", code, count).map_err(|e| e.to_string())?;
    }
    writeln!(file, "Errors,{}", result.errors).map_err(|e| e.to_string())?;
    writeln!(file, "Total Requests,{}", result.total_requests).map_err(|e| e.to_string())?;
    if let Some(min) = result.min_time_ms {
        writeln!(file, "Min Time (ms),{}", min).map_err(|e| e.to_string())?;
    }
    if let Some(max) = result.max_time_ms {
        writeln!(file, "Max Time (ms),{}", max).map_err(|e| e.to_string())?;
    }
    if let Some(avg) = result.avg_time_ms {
        writeln!(file, "Avg Time (ms),{:.2}", avg).map_err(|e| e.to_string())?;
    }

    Ok(())
}
