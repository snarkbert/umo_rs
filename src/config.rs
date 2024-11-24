use std::fs::{self, File};
use std::env;
use std::sync::atomic::AtomicBool;
use directories::BaseDirs;
use serde::{Deserialize, Serialize};


/* lazy_static! {
    pub static ref CONFIG: Config = Config::new();
} */

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub app_ver_header: String,
    pub app_ver: String,
    pub app_name_header: String,
    pub app_name: String,
    pub user_agent: String,
    pub curl_header: Vec<String>,
    pub req_header: std::collections::HashMap<String, String>,
    pub writing_stuff: AtomicBool,
    pub morrowind_dir: String,
    pub tes3cmd: String,
    pub socket: (String, u16),
    pub socket_server: Option<()>, // Placeholder for socket server implementation
    pub modding_openmw_host: String,
    pub nexus_api_key: Option<String>,
    pub basepath: String,
    pub cache_dir: String,
    pub verbose: bool,
    pub no_gui: bool,
    pub no_notifications: bool,
    pub nexus_premium: bool,
    pub download_only: bool,
    pub modlist_file: bool,
    pub modlist: Option<String>,
    pub use_7z: bool,
    pub max_threads: usize,
    pub json_output: bool,
    pub use_dev_mods: bool,
    pub bin_7z: String,
}

impl Config {
    pub fn new() -> Self {
        let app_name = "umo".to_string();
        let app_ver = "0.5.7".to_string();
        let user_agent = format!("{}/{}", app_name, app_ver);

        let curl_header = vec![
            format!("Application-Version: {}", app_ver),
            format!("Application-Name: {}", app_name),
            format!("User-Agent: {}", user_agent),
        ];

        let mut req_header = std::collections::HashMap::new();
        req_header.insert("Application-Version".to_string(), app_ver.to_string());
        req_header.insert("Application-Name".to_string(), app_name.to_string());
        req_header.insert("User-Agent".to_string(), user_agent.clone());

        let cache_dir = BaseDirs::new()
            .and_then(|dirs| dirs.cache_dir().to_str().map(String::from))
            .unwrap_or_else(|| "./cache".to_string());

        Config {
            app_ver_header: "Application-Version".to_string(),
            app_ver,
            app_name_header: "Application-Name".to_string(),
            app_name,
            user_agent,
            curl_header,
            req_header,
            writing_stuff: AtomicBool::new(false),
            morrowind_dir: ".".to_string(),
            tes3cmd: "tes3cmd".to_string(),
            socket: ("127.0.0.1".to_string(), 6666),
            socket_server: None,
            modding_openmw_host: env::var("MODDING_OPENMW_HOST")
                .unwrap_or_else(|_| "https://modding-openmw.com".to_string()),
            nexus_api_key: None,
            basepath: "./OpenMWMods".to_string(),
            cache_dir,
            verbose: false,
            no_gui: false,
            no_notifications: env::var("UMO_NO_NOTIFICATIONS").is_ok(),
            nexus_premium: false,
            download_only: false,
            modlist_file: false,
            modlist: None,
            use_7z: false,
            max_threads: 10,
            json_output: env::var("YAMZ").is_ok(),
            use_dev_mods: false,
            bin_7z: "7z".to_string(),
        }
    }

    pub fn save(&self, filename: &str) {
        let output_file = File::create(filename)
            .expect("Could not create config file");
        serde_json::to_writer_pretty(output_file, &self)
            .expect("Error serializing config");
    }

    pub fn load(filename: &str) -> Self {
        let contents = fs::read_to_string(filename)
            .expect("could not read config file");
        let cfg: Config =  serde_json::from_str(&contents)
            .expect("Error deserializing config");
        return cfg;
    }
}
