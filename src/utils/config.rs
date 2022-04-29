use std::{fs::File, io::Read};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
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
    pub fn from_toml(path: &str) -> Result<Self, toml::de::Error> {
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
                panic!("Could not open config: {}", why);
            }
        };

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
