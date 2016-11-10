use irc::message::{Message, MessageParseError, reply, error as err};
use irc::IrcHandle;
use error::NetResult;
use config::Config;
use std::io::Write;

pub fn dispatch_command<Output>(handle: &IrcHandle<Output>, request: String) -> NetResult
    where Output: Write
{
    match request.parse::<Message>() {
        Ok(message) => match message.command.as_ref() {
            "PASS"  => pass(handle, message),
            _       => unknown_command(handle, message),
        },
        _           => no_command(handle),
    }
}

fn pass<Output>(handle: &IrcHandle<Output>, message: Message) -> NetResult
    where Output: Write
{
    Ok(())
}

fn unknown_command<Output>(handle: &IrcHandle<Output>, message: Message) -> NetResult
    where Output: Write
{
    let mut connection = handle.connection.lock().unwrap();

    connection.write(format!("{} {} :Unknown command\n", err::UNKNOWN_COMMAND, message.command).as_bytes());
    connection.flush();

    Ok(())
}

fn no_command<Output>(handle: &IrcHandle<Output>) -> NetResult
    where Output: Write
{
    Ok(())
}

#[cfg(test)]
mod test
{
    extern crate mioco;

    use self::mioco::sync::RwLock;
    use std::sync::Arc;
    use std::str;
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

        {
            let mut state  = Arc::new(RwLock::new(Irc::new(config)));
            let mut handle = IrcHandle::new(state.clone(), &mut buffer);

            let message    = "some_gibberish".to_string();

            let result = dispatch_command(&handle, message);
            assert!(result.is_ok());
        }

        assert_eq!("421 some_gibberish :Unknown command\n", &String::from_utf8_lossy(&buffer));
    }

    #[test]
    fn no_command()
    {
        let mut buffer = Vec::<u8>::new();
        let config     = Config::default();

        {
            let mut state  = Arc::new(RwLock::new(Irc::new(config)));
            let mut handle = IrcHandle::new(state.clone(), &mut buffer);

            let message    = "".to_string();

            let result = dispatch_command(&handle, message);
            assert!(result.is_ok());
        }

        assert_eq!("", &String::from_utf8_lossy(&buffer));
    }
}
