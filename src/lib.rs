#[derive(Debug, Clone)]
pub struct UserConfig {
    pub theme_name: String,
    pub font_size: u8,
}

impl Default for UserConfig {
    fn default() -> Self { Self { theme_name: "sigma-dark".into(), font_size: 11 } }
}

pub struct ConfigManager;
impl ConfigManager {
    pub fn get_default_config_path() -> &'static str { "~/.config/sigmaos/config.toml" }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_config_defaults() {
        let cfg = UserConfig::default();
        assert_eq!(cfg.theme_name, "sigma-dark");
        assert_eq!(ConfigManager::get_default_config_path(), "~/.config/sigmaos/config.toml");
    }
}
