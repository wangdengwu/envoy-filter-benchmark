use proxy_wasm::traits::*;
use proxy_wasm::types::*;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> { Box::new(Passthroughs) });
}}

struct Passthroughs;

impl Context for Passthroughs {}

impl HttpContext for Passthroughs {}
