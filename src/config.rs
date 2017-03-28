// IRC server in Rust
// Copyright (C) 2016, Matthias Devlamynck
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
// 
// For any questions, feture request or bug reports please contact me
// at matthias.devlamynck@mailoo.org. The official repository for this
// project is https://github.com/mdevlamynck/lircd.

extern crate toml;

use std::default::Default;
use std::io::{Read, Write, Error, ErrorKind};
use std::mem;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::env::home_dir;

use std::sync::RwLock;

lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new(Config::default());
}

pub fn get() -> &'static RwLock<Config>
{
    &CONFIG
}

pub fn load()
{
    let path = Config::default_path();
    load_from(&path);
}

pub fn load_from(path: &Path)
{
    let mut config = get().write().unwrap();
    let new_config = Config::load_from(path);
    mem::replace(&mut *config, new_config);
}

pub fn reload()
{
    let mut config = get().write().unwrap();
    config.reload();
}

pub fn save()
{
    let config = get().read().unwrap();
    config.save();
}

pub fn create_if_doesnt_exist()
{
    let config = get().read().unwrap();
    config.create_if_doesnt_exist();
}

pub fn create_or_load_from_path(path: String)
{
    if !path.is_empty() {
        config::load_from(Path::new(&path))
    } else {
        config::load()
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct Config
{
    pub inner: InnerConfig,
    pub path:  PathBuf,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerConfig
{
    pub network: Network,
    pub irc:     Irc,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Network
{
    pub listen_address: String,
    pub hostname:       String,
    pub use_tls:        bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Irc
{
    pub password:       String,
    pub timeout:        i32,
    pub welcome:        String,
}

impl InnerConfig
{
    pub fn new() -> InnerConfig
    {
        InnerConfig {
            network: Network::new(),
            irc:     Irc::new(),
        }
    }
}

impl Config
{
    fn new() -> Config
    {
        Config {
            inner: InnerConfig::new(),
            path:  Config::default_path()
        }
    }

    fn load_from(path: &Path) -> Config
    {
        let config = File::open(&path)
            .and_then(|mut file| {
                let mut file_content = String::new();
                let _                = file.read_to_string(&mut file_content)?;
                let config           = toml::from_str(&file_content)
                    .or_else(|err| {
                        error!("Can't read configuration file: {}", err);
                        Err(Error::new(ErrorKind::InvalidData, "Can't read configuration file"))
                    })?;

                Ok(config)
            }).unwrap_or_else(|err| {
                error!("Error: {}, using default values instead.", err);
                InnerConfig::new()
            });

        Config {
            inner: config,
            path:  path.to_path_buf(),
        }
    }

    fn reload(&mut self)
    {
        let new_config = Config::load_from(&self.path);
        mem::replace(self, new_config);
    }

    fn save(&self)
    {
        File::create(&self.path)
            .and_then(|mut file| {
                toml::to_string(&self.inner)
                    .map_err(|err| Error::new(ErrorKind::InvalidData, format!("Can't serialize configuration: {}", err)))
                    .and_then(move |toml| file.write_all(toml.as_bytes()))
            })
            .unwrap_or_else(|err| error!("Unable to save configuration: {}", err));
    }

    fn create_if_doesnt_exist(&self)
    {
        if !self.path.is_file() {
            self.save();
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
    fn new() -> Network
    {
        Network {
            listen_address: "0.0.0.0:6667".to_string(),
            hostname:       "localhost".to_string(),
            use_tls:        false,
        }
    }
}

impl Irc
{
    fn new() -> Irc
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
    extern crate tempdir;

    use unindent::unindent;
    use std::fs::File;
    use std::path::Path;
    use std::io::{Read, Write};
    use std::panic;
    use super::*;

    fn in_tmp_dir<T>(test: T) -> ()
        where T: FnOnce(&Path) -> () + panic::UnwindSafe
    {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();

        let result = panic::catch_unwind(|| {
            test(tmp_dir.path());
        });

        tmp_dir.close().unwrap();

        assert!(result.is_ok())
    }

    #[test]
    fn new()
    {
        let new     = super::Config::new();
        let default = super::Config::default();

        assert_eq!(new, default);
    }

    #[test]
    fn default()
    {
        let config = super::Config::default();

        assert_eq!("0.0.0.0:6667", &config.inner.network.listen_address);
        assert_eq!("localhost", &config.inner.network.hostname);
        assert_eq!(false, config.inner.network.use_tls);

        assert_eq!("Ch4ng3Th1sP4ssw0rd", &config.inner.irc.password);
        assert_eq!(240, config.inner.irc.timeout);
        assert_eq!("Welcome to lircd", &config.inner.irc.welcome);
    }

    #[test]
    fn load_from_valid_data()
    {
        in_tmp_dir(|tmp_dir| {
            let path = tmp_dir.join("config.toml");

            File::create(&path)
                .and_then(|mut f| f.write_all(&unindent(r#"
                        [irc]
                        password = "somepassword"
                        timeout = 42
                        welcome = "some welcome"

                        [network]
                        hostname = "somehost"
                        listen_address = "0.0.0.0:42"
                        use_tls = true
                    "#).as_bytes()))
                .unwrap();

            let config = Config::load_from(&path);

            assert_eq!("0.0.0.0:42", &config.inner.network.listen_address);
            assert_eq!("somehost", &config.inner.network.hostname);
            assert_eq!(true, config.inner.network.use_tls);

            assert_eq!("somepassword", &config.inner.irc.password);
            assert_eq!(42, config.inner.irc.timeout);
            assert_eq!("some welcome", &config.inner.irc.welcome);

            assert_eq!(path, config.path);
        });
    }

    #[test]
    fn load_from_invalid_data_load_default_data()
    {
        in_tmp_dir(|tmp_dir| {
            let path = tmp_dir.join("config.toml");

            File::create(&path)
                .and_then(|mut f| f.write_all(&unindent(r#"
                        [invalid]
                        invalid = "invalid"
                    "#).as_bytes()))
                .unwrap();

            let config = Config::load_from(&path);

            let mut expected = Config::default();
            expected.path    = path;
            assert_eq!(expected, config);
        });
    }

    #[test]
    fn save()
    {
        in_tmp_dir(|tmp_dir| {
            let path        = tmp_dir.join("config.toml");
            let mut config  = Config::default();
            config.path     = path.clone();

            assert!(!path.is_file());

            config.save();
            
            assert!(path.is_file());

            let mut file = File::open(&path).unwrap();
            let mut file_content = String::new();
            let _                = file.read_to_string(&mut file_content).unwrap();

            assert_eq!(unindent(r#"
                    [irc]
                    password = "Ch4ng3Th1sP4ssw0rd"
                    timeout = 240
                    welcome = "Welcome to lircd"

                    [network]
                    hostname = "localhost"
                    listen_address = "0.0.0.0:6667"
                    use_tls = false
                "#),
                file_content
            );
        });
    }

    #[test]
    fn reload()
    {
        in_tmp_dir(|tmp_dir| {
            let path       = tmp_dir.join("config.toml");
            let mut config = Config::load_from(&path);

            let mut expected = Config::default();
            expected.path    = path.clone();
            assert_eq!(expected, config);

            File::create(&path)
                .and_then(|mut f| f.write_all(&unindent(r#"
                        [irc]
                        password = "somepassword"
                        timeout = 42
                        welcome = "some welcome"

                        [network]
                        hostname = "somehost"
                        listen_address = "0.0.0.0:42"
                        use_tls = true
                    "#).as_bytes()))
                .unwrap();

            config.reload();

            assert_eq!("0.0.0.0:42", &config.inner.network.listen_address);
            assert_eq!("somehost", &config.inner.network.hostname);
            assert_eq!(true, config.inner.network.use_tls);

            assert_eq!("somepassword", &config.inner.irc.password);
            assert_eq!(42, config.inner.irc.timeout);
            assert_eq!("some welcome", &config.inner.irc.welcome);

            assert_eq!(path, config.path);
        });
    }
}
