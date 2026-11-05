/// User configuration following Single Responsibility principle
/// - Theme name for UI theming
/// - Font size for accessibility
#[derive(Debug, Clone)]
pub struct UserConfig {
    pub theme_name: String,
    pub font_size: u8,
}

impl Default for UserConfig {
    // Follows Secure Defaults: sensible default theme and readable font size
    fn default() -> Self { 
        Self { 
            theme_name: "sigma-dark".into(), 
            font_size: 11 
        } 
    }
}

/// Configuration manager following Single Responsibility principle
/// - Manages configuration file paths
/// - Encapsulates configuration-related utilities
pub struct ConfigManager;

impl ConfigManager {
    /// Returns the default configuration path
    /// Follows Convention over Configuration: standard XDG config location
    pub fn get_default_config_path() -> &'static str { 
        "~/.config/sigmaos/config.toml" 
    }
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
