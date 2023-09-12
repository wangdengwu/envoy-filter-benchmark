use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Percentiles {
    pub Percentile: f64,
    pub Value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct DurationHistogram {
    pub Min: f64,
    pub Max: f64,
    pub Avg: f64,
    pub Percentiles: Vec<Percentiles>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct FortioReport {
    pub Labels: String,
    pub ActualQPS: f64,
    pub DurationHistogram: DurationHistogram,
}

pub(crate) fn parse_report_files(dir: String) -> Vec<FortioReport> {
    let mut fortio_reports = vec![];
    if let Ok(files) = fs::read_dir(dir) {
        for file in files {
            if let Ok(file) = file {
                let ref file_name = file.file_name();
                if let Some(file_name) = file_name.to_str() {
                    if file_name.ends_with(".json") {
                        if let Ok(file_content) = fs::read_to_string(file.path()) {
                            if let Ok(fortio_report) = serde_json::from_str::<FortioReport>(&file_content) {
                                fortio_reports.push(fortio_report);
                            }
                        }
                    }
                }
            }
        }
    }
    fortio_reports
}