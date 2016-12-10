extern crate toml;

use std::default::Default;
use std::io::{Read, Write, Error, ErrorKind};
use std::mem;
use std::path::PathBuf;
use std::fs::File;
use std::env::home_dir;

#[derive(Debug, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Config
{
    pub network: Network,
    pub irc:     Irc,
}

#[derive(Debug, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Network
{
    pub listen_address: String,
    pub use_async:      bool,
    pub hostname:       String,
}

#[derive(Debug, Clone, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Irc
{
    pub password:       String,
    pub timeout:        i32,
    pub welcome:        String,
}

impl Config
{
    pub fn new() -> Config
    {
        Config {
            network: Network::new(),
            irc:     Irc::new(),
        }
    }

    pub fn load() -> Config
    {
        File::open(&Config::default_path())
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

    pub fn save(&self)
    {
        File::create(&Config::default_path())
            .and_then(|mut file| file.write_all(toml::encode_str(self).as_bytes()))
            .unwrap_or_else(|err| error!("Unable to save configuration: {}", err));
    }

    pub fn create_if_doesnt_exist()
    {
        let path = Config::default_path();

        if !path.is_file() {
            let config = Config::default();
            config.save();
        }
    }

    fn default_path() -> PathBuf
    {
        let mut path = home_dir().unwrap_or(PathBuf::from("/etc"));
        path.push(".lircd.conf");

        path
    }
}

impl Network
{
    pub fn new() -> Network
    {
        Network {
            listen_address: "0.0.0.0:6667".to_string(),
            use_async:      true,
            hostname:       "localhost".to_string(),
        }
    }
}

impl Irc
{
    pub fn new() -> Irc
    {
        Irc {
            password: "Ch4ng3Th1sP4ssw0rd".to_string(),
            timeout:  240,
            welcome:  "Welcome to lircd".to_string(),
        }
    }
}

impl Default for Config
{
    fn default() -> Config
    {
        Config::new()
    }
}

impl Default for Network
{
    fn default() -> Network
    {
        Network::new()
    }
}

impl Default for Irc
{
    fn default() -> Irc
    {
        Irc::new()
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

        assert_eq!("0.0.0.0:6667", &config.network.listen_address);
        assert_eq!(true, config.network.use_async);
        assert_eq!("localhost", &config.network.hostname);

        assert_eq!("Ch4ng3Th1sP4ssw0rd", &config.irc.password);
        assert_eq!(240, config.timeout);
        assert_eq!("Welcome to lircd", &config.irc.welcome);
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
