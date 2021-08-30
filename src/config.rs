use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default, with = "json::def_uvec2")]
    pub resolution: UVec2,
    #[serde(default)]
    pub fullscreen: bool,
    #[serde(default)]
    pub gui_scale: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_processing: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            resolution: uvec2(1920, 1080),
            fullscreen: true,
            gui_scale: 1.0,
            post_processing: None,
        }
    }
}

impl Config {
    #[cfg(any(target_family = "unix", target_family = "windows"))]
    pub fn load(path: &str) -> Self {
        let config = if let Ok(json) = fs::read_to_string(path) {
            let mut config: Config = serde_json::from_str(&json)
                .expect(&format!("Unable to parse config file '{}'!", path));
            config.gui_scale = config.gui_scale.clamp(0.25, 5.0);
            config
        } else {
            Default::default()
        };
        storage::store(config.clone());
        config
    }

    #[cfg(target_family = "wasm")]
    pub fn load(key: &str) -> Self {
        let web_storage = &mut quad_storage::STORAGE.lock().unwrap();
        let config = if let Some(json) = web_storage.get(key) {
            let config: Config = serde_json::from_str(&json)
                .expect("Unable to parse config from web storage!");
            storage::store(config.clone());
            config
        } else {
            Default::default()
        };
        storage::store(config.clone());
        config
    }

    #[cfg(any(target_family = "unix", target_family = "windows"))]
    pub fn save(&self, path: &str) {
        let json = serde_json::to_string_pretty(self)
            .expect("Error parsing config!");
        fs::write(path, &json).expect("Error saving config to file!");
    }

    #[cfg(target_family = "wasm")]
    pub fn save(&self, key: &str) {
        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        let json = serde_json::to_string_pretty(self)
            .expect("Error parsing config!");
        storage.set(key, &json)
    }
}
