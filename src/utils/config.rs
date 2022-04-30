use std::{
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    token: String,
    prefix: String,
}

impl Config {
    /// Returns a new Config struct from a TOML file.
    ///
    /// # Examples
    /// ```
    /// use utils::config::Config;
    /// let config = Config::from_toml("config.toml");
    /// ```
    /// # Panics
    /// If the file could not be read or parsed.
    fn from_toml(path: &str) -> Result<Self, toml::de::Error> {
        let config = match File::open(path) {
            Ok(mut file) => {
                let mut contents = String::new();

                match file.read_to_string(&mut contents).is_err() {
                    true => panic!("Failed to read the configuation file."),
                    false => &contents,
                };

                toml::from_str::<Config>(contents.as_str())
            }
            Err(why) => {
                panic!("Failed to open the configuration file: {}", why);
            }
        };

        config
    }

    /// Generates a config.toml file from a Config struct.
    /// # Examples
    /// ```
    /// use utils::config::Config;
    /// let config = Config {
    ///    token: "token".to_string(),
    ///   prefix: "prefix".to_string(),
    /// };
    /// config.to_toml("config.toml");
    /// ```
    /// # Panics
    /// If the file could not be written.
    fn generate_toml(path: &str) {
        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(why) => panic!("Failed to create the configuration file: {}", why),
        };

        let contents = toml::to_string::<Config>(&Config::default()).unwrap();

        match file.write_all(contents.as_bytes()) {
            Ok(_) => (),
            Err(why) => panic!("Failed to write the default configuration file: {}", why),
        };
    }

    /// Returns the current config or a default config if the config file does not exist.
    /// # Examples
    /// ```
    /// use utils::config::Config;
    /// let config = Config::get_config("config.toml");
    /// ```
    /// # Panics
    /// If the file could not be read or parsed.
    /// If the file could not be written.
    pub fn get_config(path: &str) -> Self {
        // Check if the config.toml file exists
        if !std::path::Path::new(path).exists() {
            Config::generate_toml(path);
        }

        // Read the config.toml file
        let config = Config::from_toml("config.toml").unwrap();

        if config.token.is_empty() {
            panic!("\"token\" is a required field in the config.toml file, please change it.");
        }

        config
    }

    /// Returns the bot's prefix.
    /// # Examples
    /// ```
    /// let prefix = config.prefix();
    /// ```
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Returns the bot's token.
    /// # Examples
    /// ```
    /// let token = config.token();
    /// ```
    pub fn token(&self) -> &str {
        &self.token
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            token: String::new(),
            prefix: String::from("~"),
        }
    }
}
