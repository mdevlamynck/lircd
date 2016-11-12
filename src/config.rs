use std::default::Default;

#[derive(Debug, Clone, PartialEq)]
pub struct Config
{
    // Network
    pub listen_address: String,
    pub use_async:      bool,
    pub is_unix:        bool,

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

            password:       "".to_string(),
            timeout:        240,
            welcome:        "Welcome to LIrcD".to_string(),
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

        assert_eq!("", &config.password);
        assert_eq!(240, config.timeout);
        assert_eq!("Welcome to LIrcD", &config.welcome);
    }
}
