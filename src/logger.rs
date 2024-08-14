use colored::*;
use env_logger::Builder;
use log::{Level, LevelFilter, Record};
use std::env;
use std::fmt::Formatter;
use std::io::Write;
use serde::Serialize;
use serde_json::Value;
use csv::Writer;
use std::error::Error;
use std::fs;

#[derive(Serialize, Debug, Default)]
pub struct ProverPerformanceMetrics {
    // pub num_rows: usize,
    // pub log_rows: u32,
    // pub n: usize,
    pub params_k: u32,
    pub params_n: u64,
    pub k: u32,
    pub extended_k: u32,
    pub quotient_poly_degree: usize,
    pub max_gate_degree: usize,
    pub degee: usize,
    pub num_fixed_columns: usize,
    pub num_advice_columns: usize,
    pub num_instance_columns: usize,
    pub num_selectors: usize,
    pub num_challenges: usize,
    pub minimum_rows: usize,
    pub blinding_factors: usize,
    pub num_ffts: usize,
    pub num_msms: usize,
    pub max_fft_size: usize,
    pub max_msm_size: usize,
    pub total_fft_time: String,
    pub total_msm_time: String,
    // pub check_mode: str,

    pub proof_time: String,
    pub verify_time: String,
}

// pub fn write_perf_metrics_to_csv(settings: &ProverPerformanceMetrics, file_path: &str) -> Result<(), Box<dyn Error>> {
//     let mut wtr = Writer::from_path(file_path)?;

//     // Serialize the struct to a JSON Value
//     let json_value = serde_json::to_value(settings)?;

//     // Ensure the value is an object
//     if let Value::Object(ref obj) = json_value {
//         // Write headers (field names)
//         let headers: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();
//         wtr.write_record(&headers)?;

//         // Write values
//         let values: Vec<String> = headers.iter()
//             .map(|&header| obj.get(header).unwrap_or(&Value::Null).to_string())
//             .collect();
//         wtr.write_record(&values)?;
//     } else {
//         return Err("Expected JSON object".into());
//     }

//     wtr.flush()?;
//     Ok(())
// }


pub fn write_perf_metrics_to_csv(
    settings: &ProverPerformanceMetrics,
    file_path: &str,
    override_headers: bool
) -> Result<(), Box<dyn Error>> {
    // Check if the file exists
    let file_exists = fs::metadata(file_path).is_ok();
    
    let mut wtr = if file_exists && !override_headers {
        // Open the file in append mode if it exists and headers should not be overridden
        Writer::from_path(file_path)?
    } else {
        // Create a new file or overwrite the existing file with headers
        Writer::from_path(file_path)?
    };

    // Serialize the struct to a JSON Value
    let json_value = serde_json::to_value(settings)?;

    // Ensure the value is an object
    if let Value::Object(ref obj) = json_value {
        
        let headers: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();

        if !file_exists || override_headers {
            // Write headers (field names) if file does not exist or headers should be overridden
            // let headers: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();
            wtr.write_record(&headers)?;
        }

        let values: Vec<String> = headers.iter()
            .map(|&header| obj.get(header).unwrap_or(&Value::Null).to_string())
            .collect();

        wtr.write_record(&values)?;
    } else {
        return Err("Expected JSON object".into());
    }

    wtr.flush()?;
    Ok(())
}



/// sets the log level color
#[allow(dead_code)]
pub fn level_color(level: &log::Level, msg: &str) -> String {
    match level {
        Level::Error => msg.red(),
        Level::Warn => msg.yellow(),
        Level::Info => msg.blue(),
        Level::Debug => msg.green(),
        Level::Trace => msg.magenta(),
    }
    .bold()
    .to_string()
}

/// sets the log level text color
pub fn level_text_color(level: &log::Level, msg: &str) -> String {
    match level {
        Level::Error => msg.red(),
        Level::Warn => msg.yellow(),
        Level::Info => msg.white(),
        Level::Debug => msg.white(),
        Level::Trace => msg.white(),
    }
    .bold()
    .to_string()
}

/// sets the log level token
fn level_token(level: &Level) -> &str {
    match *level {
        Level::Error => "E",
        Level::Warn => "W",
        Level::Info => "*",
        Level::Debug => "D",
        Level::Trace => "T",
    }
}

/// sets the log level prefix token
fn prefix_token(level: &Level) -> String {
    format!(
        "{}{}{}",
        "[".blue().bold(),
        level_color(level, level_token(level)),
        "]".blue().bold()
    )
}

/// formats the log
pub fn format(buf: &mut Formatter, record: &Record<'_>) -> Result<(), std::fmt::Error> {
    let sep = format!("\n{} ", " | ".white().bold());
    let level = record.level();
    writeln!(
        buf,
        "{} {}",
        prefix_token(&level),
        level_color(&level, record.args().as_str().unwrap()).replace('\n', &sep),
    )
}

/// initializes the logger
pub fn init_logger() {
    let mut builder = Builder::new();

    builder.format(move |buf, record| {
        writeln!(
            buf,
            "{} [{}, {}] - {}",
            prefix_token(&record.level()),
            //    pretty print UTC time
            chrono::Utc::now()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
                .bright_magenta(),
            record.metadata().target(),
            level_text_color(&record.level(), &format!("{}", record.args()))
                .replace('\n', &format!("\n{} ", " | ".white().bold()))
        )
    });
    builder.target(env_logger::Target::Stdout);
    builder.filter(None, LevelFilter::Info);
    if env::var("RUST_LOG").is_ok() {
        builder.parse_filters(&env::var("RUST_LOG").unwrap());
    }
    builder.init();
}
