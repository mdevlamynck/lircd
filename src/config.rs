use std::default::Default;

#[derive(Clone)]
pub struct Config
{
    pub listen_addr: String,
}

impl Config
{
    pub fn new() -> Config
    {
        Config {
            listen_addr: String::new()
        }
    }
}

impl Default for Config
{
    fn default() -> Config
    {
        let mut config = Config::new();
        config.listen_addr = "0.0.0.0:6667".to_string();

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
    }

    #[test]
    fn config_default()
    {
        let config = super::Config::default();

        assert_eq!("0.0.0.0:6667", &config.listen_addr);
    }
}
