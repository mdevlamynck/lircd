use std::default::Default;

#[derive(Clone)]
pub struct Config
{
    pub listen_address: String,
    pub use_async:      bool,
    pub is_unix:        bool,
}

impl Config
{
    pub fn new() -> Config
    {
        Config {
            listen_address: String::new(),
            use_async:      true,
            is_unix:        false,
        }
    }
}

impl Default for Config
{
    fn default() -> Config
    {
        let mut config        = Config::new();
        config.listen_address = "0.0.0.0:6667".to_string();
        config.use_async      = true;
        config.is_unix        = false;

        config
    }
}

#[cfg(test)]
mod test
{
    #[test]
    fn config_new()
    {
        let config = super::Config::new();

        assert_eq!("", &config.listen_address);
        assert_eq!(true, config.use_async);
        assert_eq!(false, config.is_unix);
    }

    #[test]
    fn config_default()
    {
        let config = super::Config::default();

        assert_eq!("0.0.0.0:6667", &config.listen_address);
        assert_eq!(true, config.use_async);
        assert_eq!(false, config.is_unix);
    }
}
