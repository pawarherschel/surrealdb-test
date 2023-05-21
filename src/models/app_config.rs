use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use config::Config;

/// This is the application configuration.
///
/// # Values
///
/// - `url` - The url to the surrealdb database.
/// - `username` - The username to use for surrealdb.
/// - `password` - The password to use for surrealdb.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct AppConfig {
    pub url: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub verbose: bool,
}

impl Default for AppConfig {
    /// Create a new `AppConfig`.
    ///
    /// # Examples
    ///
    /// ```
    /// use surrealdb_test::models::app_config::AppConfig;
    ///
    /// let app_config = AppConfig::default();
    /// ```
    ///
    /// # Default Values
    ///
    /// - `vrcx_sqlite` - The path to the VRCX sqlite3 file, which is `%APPDATA%\VRCX\vrcx.sqlite`.
    fn default() -> AppConfig {
        AppConfig {
            url: None,
            username: None,
            password: None,
            verbose: false,
        }
    }
}

impl AppConfig {
    /// Create a new `AppConfig`.
    pub fn new() -> Self {
        AppConfig::default()
    }

    /// Build the `AppConfig`.
    ///
    /// # What it does
    ///
    ///
    /// - Returns the `AppConfig`.
    pub async fn build<'a>(self) -> Result<AppConfig, Box<dyn Error>> {
        if self.url.is_none() {
            return Err("url is not set".into());
        }
        if self.username.is_none() {
            return Err("username is not set".into());
        }
        if self.password.is_none() {
            return Err("password is not set".into());
        }

        Ok(AppConfig {
            url: self.url,
            username: self.username,
            password: self.password,
            verbose: self.verbose,
        })
    }

    /// Get the `AppConfig` from the `Settings.toml` file.
    pub fn get() -> Self {
        let settings = Config::builder()
            .add_source(config::File::with_name("src/Settings.toml"))
            .build()
            .unwrap_or_default();

        let settings: AppConfig = settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
            .into();

        settings
    }
}

impl From<HashMap<String, String>> for AppConfig {
    /// Create a new `AppConfig` from a `HashMap<String, String>`.
    fn from(map: HashMap<String, String>) -> Self {
        let mut config = AppConfig::new();

        for (key, value) in map {
            match key.as_str() {
                "url" => config.url = Some(value),
                "username" => config.username = Some(value),
                "password" => config.password = Some(value),
                "verbose" => config.verbose = value.parse::<bool>().unwrap(),
                _ => {}
            }
        }

        config
    }
}
