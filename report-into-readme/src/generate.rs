use std::fs::File;

use handlebars::{Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError, to_json};
use serde::{Deserialize, Serialize};
use serde_json::Map;

use crate::config::EnvConfig;
use crate::parse::FortioReport;

#[derive(Debug, Serialize, Deserialize)]
struct ReadmeData {
    name: String,
    min: f64,
    avg: f64,
    max: f64,
    p50: f64,
    p75: f64,
    p90: f64,
    p99: f64,
    p999: f64,
    qps: f64,
}

pub fn generate_readme(passthrough_fortio_reports: &Vec<FortioReport>, basic_auth_fortio_reports: &Vec<FortioReport>) {
    let passthrough_data = convert_to_readme_data(passthrough_fortio_reports);
    let basic_auth_data = convert_to_readme_data(basic_auth_fortio_reports);
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("format", Box::new(map_to_ms));
    let mut data = Map::new();
    data.insert("passthrough_data".to_string(), to_json(passthrough_data));
    data.insert("basic_auth_data".to_string(), to_json(basic_auth_data));
    handlebars
        .register_template_file("template", "./template/readme.hbs")
        .unwrap();
    let readme_md = EnvConfig::get_instance().report_readme_md.clone();
    let mut output_file = File::create(readme_md).unwrap();
    handlebars.render_to_write("template", &data, &mut output_file).unwrap();
}

fn convert_to_readme_data(fortio_reports: &Vec<FortioReport>) -> Vec<ReadmeData> {
    let mut data: Vec<ReadmeData> = vec![];
    for fortio_report in fortio_reports {
        let qps = fortio_report.ActualQPS;
        let name = fortio_report.Labels.to_string();
        let min = fortio_report.DurationHistogram.Min;
        let avg = fortio_report.DurationHistogram.Avg;
        let max = fortio_report.DurationHistogram.Max;
        let p50 = fortio_report.DurationHistogram.Percentiles[0].Value;
        let p75 = fortio_report.DurationHistogram.Percentiles[1].Value;
        let p90 = fortio_report.DurationHistogram.Percentiles[2].Value;
        let p99 = fortio_report.DurationHistogram.Percentiles[3].Value;
        let p999 = fortio_report.DurationHistogram.Percentiles[4].Value;
        let readme_data = ReadmeData { name, min, avg, max, p50, p75, p90, p99, p999, qps };
        data.push(readme_data);
    }
    data
}

fn map_to_ms(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    // get parameter from helper or throw an error
    let param = h
        .param(0)
        .ok_or(RenderError::new("missing parameter"))?;
    if param.relative_path().unwrap().ends_with(".qps") {
        let v: f64 = param.value().render().parse().unwrap();
        write!(out, "{:.0} qps", v)?;
    } else if param.value().is_f64() {
        let v: f64 = param.value().render().parse().unwrap();
        write!(out, "{:.4} ms", v * 1000f64)?;
    } else {
        write!(out, "{}", param.value().render())?;
    }
    Ok(())
}