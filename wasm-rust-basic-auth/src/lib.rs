use http_auth_basic::Credentials;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use serde::{Deserialize, Serialize};

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(BasicAuthRoot {
            user: User {
                username: String::new(),
                password: String::new(),
            }
        })
    });
}}

#[derive(Clone, Serialize, Deserialize)]
struct User {
    pub username: String,
    pub password: String,
}

struct BasicAuthRoot {
    pub user: User,
}

impl Context for BasicAuthRoot {}

impl RootContext for BasicAuthRoot {
    fn on_configure(&mut self, _: usize) -> bool {
        if let Some(config_bytes) = self.get_plugin_configuration() {
            match serde_json::from_slice(config_bytes.as_slice()) {
                Ok(user) => self.user = user,
                Err(_) => panic!("Failed to parse plugin configuration"),
            }
        }
        true
    }

    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(BasicAuthHttp {
            user: self.user.clone(),
        }))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}

struct BasicAuthHttp {
    pub user: User,
}

impl Context for BasicAuthHttp {}

impl HttpContext for BasicAuthHttp {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        match self.get_http_request_header("Authorization") {
            Some(auth) => {
                let (username, password, ok) = parse_basic_auth(auth);
                if ok {
                    if username == self.user.username && password == self.user.password {
                        return Action::Continue;
                    }
                }
            }
            _ => {}
        }
        self.send_http_response(
            403,
            vec![("Powered-By", "proxy-wasm-rust")],
            Some(b"Access forbidden.\n"),
        );
        Action::Pause
    }
}

fn parse_basic_auth(auth: String) -> (String, String, bool) {
    match Credentials::from_header(auth) {
        Ok(c) => (c.user_id, c.password, true),
        Err(_) => (String::new(), String::new(), false)
    }
}

