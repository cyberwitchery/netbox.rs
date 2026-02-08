//! Configuration file support for netbox-cli.
//!
//! Config file location: `~/.config/netbox-cli/config.toml`
//!
//! Example config:
//! ```toml
//! [default]
//! url = "https://netbox.example.com"
//! token_env = "NETBOX_TOKEN"
//!
//! [prod]
//! url = "https://netbox.prod.example.com"
//! token_command = "pass show netbox/prod-token"
//! timeout = 60
//! ssl_verify = true
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

/// A single profile configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Profile {
    /// NetBox instance URL.
    pub url: Option<String>,
    /// Plain token (not recommended - use token_env or token_command).
    pub token: Option<String>,
    /// Environment variable containing the token.
    pub token_env: Option<String>,
    /// Command to execute to get the token.
    pub token_command: Option<String>,
    /// Request timeout in seconds.
    pub timeout: Option<u64>,
    /// Maximum retry attempts.
    pub retries: Option<u32>,
    /// Whether to verify SSL certificates.
    pub ssl_verify: Option<bool>,
    /// Default output format (json, yaml, table).
    pub output: Option<String>,
}

impl Profile {
    /// Resolve the token from the configured source.
    pub fn resolve_token(&self) -> Result<Option<String>, ConfigError> {
        // Priority: token_command > token_env > token
        if let Some(cmd) = &self.token_command {
            return resolve_token_command(cmd).map(Some);
        }
        if let Some(env_var) = &self.token_env {
            return Ok(std::env::var(env_var).ok());
        }
        Ok(self.token.clone())
    }
}

/// Configuration file containing multiple profiles.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfigFile {
    #[serde(flatten)]
    pub profiles: HashMap<String, Profile>,
}

impl ConfigFile {
    /// Get a profile by name, falling back to "default" if not found.
    pub fn get_profile(&self, name: &str) -> Option<&Profile> {
        self.profiles.get(name)
    }

    /// List all profile names.
    pub fn profile_names(&self) -> Vec<&str> {
        self.profiles.keys().map(String::as_str).collect()
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Parse(toml::de::Error),
    TokenCommand { command: String, message: String },
    MissingField(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(err) => write!(f, "config file error: {}", err),
            ConfigError::Parse(err) => write!(f, "config parse error: {}", err),
            ConfigError::TokenCommand { command, message } => {
                write!(f, "token command '{}' failed: {}", command, message)
            }
            ConfigError::MissingField(field) => {
                write!(f, "missing required field: {}", field)
            }
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::Io(err)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::Parse(err)
    }
}

/// Get the default config file path.
pub fn config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("netbox-cli").join("config.toml"))
}

/// Load config file from the default location.
pub fn load_config() -> Result<Option<ConfigFile>, ConfigError> {
    let Some(path) = config_path() else {
        return Ok(None);
    };
    if !path.exists() {
        return Ok(None);
    }
    let contents = std::fs::read_to_string(&path)?;
    let config: ConfigFile = toml::from_str(&contents)?;
    Ok(Some(config))
}

/// Execute a command and return its stdout as the token.
fn resolve_token_command(command: &str) -> Result<String, ConfigError> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", command]).output()
    } else {
        Command::new("sh").args(["-c", command]).output()
    };

    match output {
        Ok(output) if output.status.success() => {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        }
        Ok(output) => Err(ConfigError::TokenCommand {
            command: command.to_string(),
            message: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        }),
        Err(err) => Err(ConfigError::TokenCommand {
            command: command.to_string(),
            message: err.to_string(),
        }),
    }
}

/// Validate a profile has all required fields for API access.
pub fn validate_profile(profile: &Profile) -> Result<(), ConfigError> {
    if profile.url.is_none() {
        return Err(ConfigError::MissingField("url".to_string()));
    }
    if profile.token.is_none() && profile.token_env.is_none() && profile.token_command.is_none() {
        return Err(ConfigError::MissingField(
            "token, token_env, or token_command".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config_file() {
        let toml = r#"
[default]
url = "https://netbox.example.com"
token_env = "NETBOX_TOKEN"

[prod]
url = "https://netbox.prod.example.com"
token_command = "pass show netbox/token"
timeout = 60
ssl_verify = true
output = "table"
"#;
        let config: ConfigFile = toml::from_str(toml).unwrap();
        assert_eq!(config.profiles.len(), 2);

        let default = config.get_profile("default").unwrap();
        assert_eq!(default.url.as_deref(), Some("https://netbox.example.com"));
        assert_eq!(default.token_env.as_deref(), Some("NETBOX_TOKEN"));

        let prod = config.get_profile("prod").unwrap();
        assert_eq!(prod.timeout, Some(60));
        assert_eq!(prod.ssl_verify, Some(true));
        assert_eq!(prod.output.as_deref(), Some("table"));
    }

    #[test]
    fn validate_profile_requires_url() {
        let profile = Profile {
            token: Some("tok".to_string()),
            ..Default::default()
        };
        let result = validate_profile(&profile);
        assert!(matches!(result, Err(ConfigError::MissingField(_))));
    }

    #[test]
    fn validate_profile_requires_token_source() {
        let profile = Profile {
            url: Some("https://example.com".to_string()),
            ..Default::default()
        };
        let result = validate_profile(&profile);
        assert!(matches!(result, Err(ConfigError::MissingField(_))));
    }

    #[test]
    fn validate_profile_accepts_token_env() {
        let profile = Profile {
            url: Some("https://example.com".to_string()),
            token_env: Some("MY_TOKEN".to_string()),
            ..Default::default()
        };
        assert!(validate_profile(&profile).is_ok());
    }
}
