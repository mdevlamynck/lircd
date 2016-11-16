use irc::message::{Message, reply, reply as rep, error as err};
use irc::{IrcHandle, Connection, Client};
use error::{NetResult, NetError};
use config::Config;
use std::io::Write;

pub fn dispatch_command<Output>(handle: &IrcHandle<Output>, request: String) -> NetResult
    where Output: Write
{
    match request.parse::<Message>() {
        Ok(message)    => match message.command.as_ref() {
            "PASS"     => pass(handle, &message),
            "NICK"     => nick(handle, &message),
            "USER"     => user(handle, &message),
            "OPER"     => oper(handle, &message),
            "MODE"     => mode(handle, &message),
            "SERVICE"  => service(handle, &message),
            "QUIT"     => quit(handle, &message),
            "SQUIT"    => squit(handle, &message),
            "JOIN"     => join(handle, &message),
            "PART"     => part(handle, &message),
            "TOPIC"    => topic(handle, &message),
            "NAMES"    => names(handle, &message),
            "LIST"     => list(handle, &message),
            "INVITE"   => invite(handle, &message),
            "KICK"     => kick(handle, &message),
            "PRIVMSG"  => privmsg(handle, &message),
            "NOTICE"   => notice(handle, &message),
            "MOTD"     => motd(handle, &message),
            "LUSERS"   => lusers(handle, &message),
            "VERSION"  => version(handle, &message),
            "STATS"    => stats(handle, &message),
            "LINKS"    => links(handle, &message),
            "TIME"     => time(handle, &message),
            "CONNECT"  => connect(handle, &message),
            "TRACE"    => trace(handle, &message),
            "ADMIN"    => admin(handle, &message),
            "INFO"     => info(handle, &message),
            "SERVLIST" => servlist(handle, &message),
            "SQUERY"   => squery(handle, &message),
            "WHO"      => who(handle, &message),
            "WHOIS"    => whois(handle, &message),
            "WHOWAS"   => whowas(handle, &message),
            "KILL"     => kill(handle, &message),
            "PING"     => ping(handle, &message),
            "PONG"     => pong(handle, &message),
            "ERROR"    => error(handle, &message),
            "AWAY"     => away(handle, &message),
            "REHASH"   => rehash(handle, &message),
            "DIE"      => die(handle, &message),
            "RESTART"  => restart(handle, &message),
            "SUMMON"   => summon(handle, &message),
            "USERS"    => users(handle, &message),
            "WALLOPS"  => wallops(handle, &message),
            "USERHOST" => userhost(handle, &message),
            "ISON"     => ison(handle, &message),
            _          => unknown_command(handle, &message),
        },
        _              => no_command(handle),
    }
}

fn pass<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    let mut connection = handle.connection.lock().unwrap();

    match *connection {
        Connection::Unknown(_) => Ok(()),
        _                      => connection.write_all(format!("{r}\r\n", r=err::ALREADY_REGISTERED).as_bytes()),
    }?;

    Ok(())
}

fn nick<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    let ref arguments = message.arguments;

    if arguments.len() >= 1 {
        let ref new_nickname = arguments[0];

        let mut connection   = handle.connection.lock().unwrap();
        if let Connection::Unknown(_) = *connection {
            *connection = Connection::Client(Client::new(handle.output.clone()));
        }

        if let Connection::Client(ref mut client) = *connection {
            client.nickname = new_nickname.clone();
        }
    } else {
        need_more_params(handle, message);
    }

    Ok(())
}

fn user<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    let ref arguments = message.arguments;

    if arguments.len() >= 4 {
        let ref username   = arguments[0];
        let ref mode       = arguments[1];
        let ref realname   = arguments[3];

        let mut connection = handle.connection.lock().unwrap();
        if let Connection::Client(ref mut client) = *connection {
            client.username = username.clone();
            client.realname = realname.clone();

            client.write_all(format!("{r} :Welcome to LIrcD\r\n", r=rep::WELCOME).as_bytes())?;
        }
    } else {
        need_more_params(handle, message);
    }

    Ok(())
}

fn oper<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn mode<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    if message.command.starts_with('#') {
        mode_channel(handle, message)
    } else {
        mode_user(handle, message)
    }
}

fn mode_user<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn mode_channel<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn service<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn quit<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    let mut connection = handle.connection.lock().unwrap();
    connection.write_all("ERROR\r\n".as_bytes())?;

    Err(NetError::CloseConnection)
}

fn squit<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn join<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn part<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn topic<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn names<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn list<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn invite<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn kick<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn privmsg<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn notice<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn motd<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn lusers<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn version<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn stats<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn links<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn time<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn connect<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn trace<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn admin<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn info<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn servlist<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn squery<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn who<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn whois<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn whowas<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn kill<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn ping<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn pong<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn error<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn away<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn rehash<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn die<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn restart<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn summon<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn users<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn wallops<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn userhost<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn ison<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    unimplemented_command(handle, message)
}

fn unimplemented_command<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    let mut connection = handle.connection.lock().unwrap();

    connection.write_all(format!("{e} {c} :Unknown command, not implemented yet\r\n", e=err::UNKNOWN_COMMAND, c=message.command).as_bytes())?;

    Ok(())
}

fn unknown_command<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    let mut connection = handle.connection.lock().unwrap();

    connection.write_all(format!("{e} {c} :Unknown command\r\n", e=err::UNKNOWN_COMMAND, c=message.command).as_bytes())?;

    Ok(())
}

fn no_command<Output>(handle: &IrcHandle<Output>) -> NetResult
    where Output: Write
{
    Ok(())
}

fn need_more_params<Output>(handle: &IrcHandle<Output>, message: &Message) -> NetResult
    where Output: Write
{
    let mut connection = handle.connection.lock().unwrap();

    connection.write_all(format!("{e} {c} :Not enough parameters\r\n", e=err::NEED_MORE_PARAMS, c=message.command).as_bytes())?;

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
    fn unknown_command_writes_back_unknown_command()
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

        assert_eq!("421 some_gibberish :Unknown command\r\n", &String::from_utf8_lossy(&buffer));
    }

    #[test]
    fn unimplemented_command_writes_back_unknown_command_not_implemented_yet()
    {
        let mut buffer = Vec::<u8>::new();
        let config     = Config::default();

        {
            let mut state  = Arc::new(RwLock::new(Irc::new(config)));
            let mut handle = IrcHandle::new(state.clone(), &mut buffer);

            let message    = "ISON".to_string();

            let result = dispatch_command(&handle, message);
            assert!(result.is_ok());
        }

        assert_eq!("421 ISON :Unknown command, not implemented yet\r\n", &String::from_utf8_lossy(&buffer));
    }

    #[test]
    fn no_command_no_reaction()
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
