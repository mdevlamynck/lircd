use std::default::Default;

#[derive(Clone)]
pub struct Config
{
    pub listen_addr: String,
    pub use_async:   bool,
}

impl Config
{
    pub fn new() -> Config
    {
        Config {
            listen_addr: String::new(),
            use_async:   true,
        }
    }
}

impl Default for Config
{
    fn default() -> Config
    {
        let mut config     = Config::new();
        config.listen_addr = "0.0.0.0:6667".to_string();
        config.use_async   = true;

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

        assert_eq!("", &config.listen_addr);
        assert_eq!(true, &config.use_async);
    }

    #[test]
    fn config_default()
    {
        let config = super::Config::default();

        assert_eq!("0.0.0.0:6667", &config.listen_addr);
        assert_eq!(true, &config.use_async);
    }
}
