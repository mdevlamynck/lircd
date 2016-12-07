extern crate toml;

use std::default::Default;
use std::io::{Read, Write, Result, Error, ErrorKind};
use std::mem;
use std::path::Path;
use std::fs::File;

const DEFAULT_CONFIG_PATH: &'static str = "~/.lircd.conf";

#[derive(Debug, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Config
{
    // Network
    pub listen_address: String,
    pub use_async:      bool,
    pub is_unix:        bool,
    pub hostname:       String,

    // Irc
    pub password:       String,
    pub timeout:        i32,
    pub welcome:        String,
}

impl Config
{
    pub fn new() -> Config
    {
        Config {
            listen_address: "0.0.0.0:6667".to_string(),
            use_async:      true,
            is_unix:        false,
            hostname:       "localhost".to_string(),

            password:       "Ch4ng3Th1sP4ssw0rd".to_string(),
            timeout:        240,
            welcome:        "Welcome to lircd".to_string(),
        }
    }

    pub fn load() -> Config
    {
        File::open(&Path::new(DEFAULT_CONFIG_PATH))
            .and_then(|mut file| {
                let mut file_content = String::new();
                let _                = file.read_to_string(&mut file_content)?;
                let config           = toml::decode_str(&file_content)
                    .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Can't read configuration file"))?;

                Ok(config)
            }).unwrap_or(Config::new())
    }

    pub fn reload(&mut self)
    {
        let new_config = Config::load();
        mem::replace(self, new_config);
    }

    pub fn save(&self) -> Result<()>
    {
        File::create(&Path::new(DEFAULT_CONFIG_PATH))
            .and_then(|mut file| file.write_all(toml::encode_str(self).as_bytes()))
            .unwrap_or_else(|err| error!("Unable to save configuration: {}", err) );

        Ok(())
    }
}

impl Default for Config
{
    fn default() -> Config
    {
        Config::new()
    }
}

#[cfg(test)]
mod test
{
    #[test]
    fn config_new()
    {
        let new     = super::Config::new();
        let default = super::Config::default();

        assert_eq!(new, default);
    }

    #[test]
    fn config_default()
    {
        let config = super::Config::default();

        assert_eq!("0.0.0.0:6667", &config.listen_address);
        assert_eq!(true, config.use_async);
        assert_eq!(false, config.is_unix);
        assert_eq!("localhost", &config.hostname);

        assert_eq!("Ch4ng3Th1sP4ssw0rd", &config.password);
        assert_eq!(240, config.timeout);
        assert_eq!("Welcome to lircd", &config.welcome);
    }

    #[test]
    fn load_valid_data()
    {
    }

    #[test]
    fn load_invalid_data()
    {
    }

    #[test]
    fn write_success()
    {
    }

    #[test]
    fn write_error()
    {
    }

    #[test]
    fn reload()
    {
    }
}
