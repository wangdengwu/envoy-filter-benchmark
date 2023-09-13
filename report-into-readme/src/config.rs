use std::env;
use std::sync::Arc;

static DEFAULT_REPORT_PASSTHROUGH_DIR: &str = "../report-passthrough";
static DEFAULT_REPORT_BASIC_AUTH_DIR: &str = "../report-basic-auth";
static DEFAULT_REPORT_SVG_DIR: &str = "../assets";
static DEFAULT_TEMPLATE_DIR: &str = "./template";
static DEFAULT_README_MD: &str = "../README.md";

pub struct EnvConfig {
    pub report_passthrough_dir: String,
    pub report_basic_auth_dir: String,
    pub report_svg_dir: String,
    pub template_dir: String,
    pub report_readme_md: String,
}

impl EnvConfig {
    pub fn get_instance() -> Arc<EnvConfig> {
        static mut INSTANCE: Option<Arc<EnvConfig>> = None;
        unsafe {
            INSTANCE.get_or_insert_with(|| {
                let report_passthrough_dir = env::var("DEFAULT_REPORT_PASSTHROUGH_DIR").unwrap_or(DEFAULT_REPORT_PASSTHROUGH_DIR.to_string());
                let report_basic_auth_dir = env::var("DEFAULT_REPORT_BASIC_AUTH_DIR").unwrap_or(DEFAULT_REPORT_BASIC_AUTH_DIR.to_string());
                let report_svg_dir = env::var("DEFAULT_REPORT_SVG_DIR").unwrap_or(DEFAULT_REPORT_SVG_DIR.to_string());
                let template_dir = env::var("DEFAULT_TEMPLATE_DIR").unwrap_or(DEFAULT_TEMPLATE_DIR.to_string());
                let report_readme_md = env::var("DEFAULT_README_MD").unwrap_or(DEFAULT_README_MD.to_string());
                Arc::new(EnvConfig {
                    report_passthrough_dir,
                    report_basic_auth_dir,
                    report_svg_dir,
                    template_dir,
                    report_readme_md,
                })
            }).clone()
        }
    }
}
