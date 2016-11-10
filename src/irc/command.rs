
use irc::message::Message;
use irc::IrcHandle;
use error::NetResult;
use config::Config;
use std::io::Write;

pub fn dispatch_command<Output>(handle: &IrcHandle<Output>, message: String) -> NetResult
    where Output: Write + Send + 'static
{
    let parsed_message: Message = message.parse().unwrap();

    match parsed_message.command.as_ref() {
        "PASS" => pass(handle, parsed_message),
        _      => unknown_command(handle, parsed_message),
    }
}

fn pass<Output>(handle: &IrcHandle<Output>, message: Message) -> NetResult
    where Output: Write + Send + 'static
{
    Ok(())
}


fn unknown_command<Output>(handle: &IrcHandle<Output>, message: Message) -> NetResult
    where Output: Write + Send + 'static
{
    Ok(())
}

#[cfg(test)]
mod test
{
    extern crate mioco;

    use self::mioco::sync::RwLock;
    use std::sync::Arc;
    use irc::IrcHandle;
    use irc::IrcState;
    use irc::Irc;
    use config::Config;
    use irc::message::Message;
    use super::dispatch_command;

    #[test]
    fn unknown_command()
    {
        let mut buffer = Vec::<u8>::new();
        let config     = Config::default();

        let mut state  = Arc::new(RwLock::new(Irc::new(config)));
        let mut handle = IrcHandle::new(&state, &mut buffer);

        let message    = "".to_string();

        let result = dispatch_command(&handle, message);
        assert!(result.is_ok());
    }
}
