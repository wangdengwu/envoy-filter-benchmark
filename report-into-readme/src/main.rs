use crate::config::EnvConfig;

mod parse;
mod convert;
mod generate;
mod config;


fn main() {
    let passthrough_dir = EnvConfig::get_instance().report_passthrough_dir.clone();
    let ref passthrough_fortio_reports = parse::parse_report_files(passthrough_dir);
    convert::convert_to_chart_data(passthrough_fortio_reports, "passthrough".to_string());
    let basic_auth_dir = EnvConfig::get_instance().report_basic_auth_dir.clone();
    let ref basic_auth_fortio_reports = parse::parse_report_files(basic_auth_dir);
    convert::convert_to_chart_data(basic_auth_fortio_reports, "basic_auth".to_string());
    generate::generate_readme(passthrough_fortio_reports, basic_auth_fortio_reports);
}