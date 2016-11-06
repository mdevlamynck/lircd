extern crate mioco;

use std::str::FromStr;

struct Message
{
    pub prefix:    Option<String>, // servername the message originates from
    pub command:   Command,
}

#[derive(Debug, PartialEq)]
enum Command
{
    // Init connection
    Pass   {password: String}, // Password
    Nick   {nickname: String, hopcount: Option<u32>},
    User   {username: String, hostname: String, servername: String, realname: String},
    Server {severname: String, hopcount: u32, info: String},

    // Server managment
    Oper   {user: String, password: String},

    // Closing connection
    Quit   {quit_message: Option<String>},
    SQuit  {server: String, comment: String},

    // Channel
    Join   {channels: Vec<JoinContent>},
    Part   {channels: Vec<String>},

    //Mode   {channel: String, limit: Option, user: Option, ban_mask: Option},

    Unknown,
    __NonExhaustive,
}

#[derive(Debug, PartialEq)]
struct JoinContent
{
    chan: String,
    key:  Option<String>
}

#[derive(Debug, PartialEq)]
enum ChannelMode
{
    o,
    p,
    s,
    i,
    t,
    n,
    m,
    l,
    b,
    v,
    k,

    __NonExhaustive,
}

#[derive(Debug, PartialEq)]
enum UserMode
{
    i,
    s,
    w,
    o,

    __NonExhaustive,
}

mod reply
{
    const WELCOME            : &'static str = "001";
    const YOUR_HOST          : &'static str = "002";
    const CREATED            : &'static str = "003";
    const MY_INFO            : &'static str = "004";
    const BOUNCE             : &'static str = "005";
    const USER_HOST          : &'static str = "302";
    const IS_ON              : &'static str = "303";
    const AWAY               : &'static str = "301";
    const UN_AWAY            : &'static str = "305";
    const NOW_AWAY           : &'static str = "306";
    const WHO_IS_USER        : &'static str = "311";
    const WHO_IS_SERVER      : &'static str = "312";
    const WHO_IS_OPERATOR    : &'static str = "313";
    const WHO_IS_IDLE        : &'static str = "317";
    const END_OF_WHO_IS      : &'static str = "318";
    const WHO_IS_CHANNELS    : &'static str = "319";
    const WHO_WAS_USER       : &'static str = "314";
    const END_OF_WHO_WAS     : &'static str = "369";
    const LIST_START         : &'static str = "321";
    const LIST               : &'static str = "322";
    const LIST_END           : &'static str = "323";
    const UNIQ_OP_IS         : &'static str = "325";
    const CHANNEL_MODE_IS    : &'static str = "324";
    const NO_TOPIC           : &'static str = "331";
    const TOPIC              : &'static str = "332";
    const INVITING           : &'static str = "341";
    const SUMMONING          : &'static str = "342";
    const INVITE_LIST        : &'static str = "346";
    const END_OF_INVITE_LIST : &'static str = "347";
    const EXCEPT_LIST        : &'static str = "348";
    const END_OF_EXCEPT_LIST : &'static str = "349";
    const VERSION            : &'static str = "351";
    const WHO_REPLY          : &'static str = "352";
    const END_OF_WHO         : &'static str = "315";
    const NAM_REPLY          : &'static str = "353";
    const END_OF_NAMES       : &'static str = "366";
    const LINKS              : &'static str = "364";
    const END_OF_LINKS       : &'static str = "365";
    const BAN_LIST           : &'static str = "367";
    const END_OF_BAN_LIST    : &'static str = "368";
    const INFO               : &'static str = "371";
    const END_INFO           : &'static str = "374";
    const MOTD_START         : &'static str = "375";
    const MOTD               : &'static str = "372";
    const END_OF_MOTD        : &'static str = "376";
    const YOU_RE_OPER        : &'static str = "381";
    const RE_HASHING         : &'static str = "382";
    const YOU_RE_SERVICE     : &'static str = "383";
    const TIME               : &'static str = "391";
    const USER_START         : &'static str = "392";
    const USERS              : &'static str = "393";
    const END_OF_USERS       : &'static str = "394";
    const NO_USERS           : &'static str = "395";
    const TRACE_LINK         : &'static str = "200";
    const TRACE_CONNECTING   : &'static str = "201";
    const TRACE_HANDSHAKE    : &'static str = "202";
    const TRACE_UNKNOWN      : &'static str = "203";
    const TRACE_OPERATOR     : &'static str = "204";
    const TRACE_USER         : &'static str = "205";
    const TRACE_SERVER       : &'static str = "206";
    const TRACE_SERVICE      : &'static str = "207";
    const TRACE_NEW_TYPE     : &'static str = "208";
    const TRACE_CLASS        : &'static str = "209";
    const TRACE_CONNECT      : &'static str = "210";
    const TRACE_LOG          : &'static str = "261";
    const TRACE_END          : &'static str = "262";
    const STATS_LINK_INFO    : &'static str = "211";
    const STATS_COMMANDS     : &'static str = "212";
    const END_OF_STATS       : &'static str = "219";
    const STATS_UPTIME       : &'static str = "242";
    const STATS_O_LINE       : &'static str = "243";
    const UMODE_IS           : &'static str = "221";
    const SERV_LIST          : &'static str = "234";
    const SERV_LIST_END      : &'static str = "235";
    const L_USER_CLIENT      : &'static str = "251";
    const L_USER_OP          : &'static str = "252";
    const L_USER_UNKNOWN     : &'static str = "253";
    const L_USER_CHANNELS    : &'static str = "254";
    const L_USER_ME          : &'static str = "255";
    const ADMIN_ME           : &'static str = "256";
    const ADMIN_LOC_1        : &'static str = "257";
    const ADMIN_LOC_2        : &'static str = "258";
    const ADMIN_EMAIL        : &'static str = "259";
    const TRY_AGAIN          : &'static str = "263";
}

mod error
{
    const NO_SUCH_NICK         : &'static str = "401";
    const NO_SUCH_SERVER       : &'static str = "402";
    const NO_SUCH_CHANNEL      : &'static str = "403";
    const CAN_NOT_SEND_TO_CHAN : &'static str = "404";
    const TOO_MANY_CHANNELS    : &'static str = "405";
    const WAS_NO_SUCH_NICK     : &'static str = "406";
    const TOO_MANY_TARGETS     : &'static str = "407";
    const NO_SUCH_SERVICE      : &'static str = "408";
    const NO_ORIGIN            : &'static str = "409";
    const NO_RECIPIENT         : &'static str = "411";
    const NO_TEXT_TO_SEND      : &'static str = "412";
    const NO_TOP_LEVEL         : &'static str = "413";
    const WILD_TOP_LEVEL       : &'static str = "414";
    const BAD_MASK             : &'static str = "415";
    const UNKNOWN_COMMAND      : &'static str = "421";
    const NO_MOTD              : &'static str = "421";
    const NO_ADMIN_INFO        : &'static str = "423";
    const FILE_ERROR           : &'static str = "424";
    const NO_NICKNAME_GIVEN    : &'static str = "431";
    const ERRONEUS_NICKNAME    : &'static str = "432";
    const NICKNAME_IN_USE      : &'static str = "433";
    const NICK_COLLISION       : &'static str = "436";
    const UNAVAIL_RESOURCE     : &'static str = "437";
    const USER_NOT_IN_CHANNEL  : &'static str = "441";
    const NOT_ON_CHANNEL       : &'static str = "442";
    const USER_ON_CHANNEL      : &'static str = "443";
    const NO_LOGIN             : &'static str = "444";
    const SUMMON_DISABLED      : &'static str = "445";
    const USER_DISABLED        : &'static str = "446";
    const NOT_REGISTERED       : &'static str = "451";
    const NEED_MORE_PARAMS     : &'static str = "461";
    const ALREADY_REGISTERED   : &'static str = "462";
    const NO_PERM_FOR_HOST     : &'static str = "463";
    const PASSWD_MISMATCH      : &'static str = "464";
    const YOU_RE_BANNED_CREEP  : &'static str = "465";
    const YOU_WILL_BE_BANNED   : &'static str = "466";
    const KEYSET               : &'static str = "467";
    const CHANNEL_IS_FULL      : &'static str = "471";
    const UNKOWN_MODE          : &'static str = "472";
    const INVITE_ONLY_CHAN     : &'static str = "473";
    const BANNED_FROM_CHAN     : &'static str = "474";
    const BAD_CHANNEL_KEY      : &'static str = "475";
    const BAD_CHAN_MASK        : &'static str = "476";
    const NO_CHAN_MODES        : &'static str = "477";
    const BAN_LIST_FULL        : &'static str = "478";
    const NO_PRIVILEGES        : &'static str = "481";
    const CHAN_O_PRIVS_NEEDED  : &'static str = "482";
    const CANT_KILL_SERVER     : &'static str = "483";
    const RESTRICTED           : &'static str = "484";
    const UNIQ_O_PRIVIS_NEEDED : &'static str = "485";
    const NO_OPER_HOST         : &'static str = "491";
    const UMODE_UNKNOWN_FLAG   : &'static str = "501";
    const USER_DONT_MATCH      : &'static str = "502";
}

#[derive(Debug, PartialEq)]
enum MessageParseError
{
    NeedMoreParams,
    UnknownCommand,
}

impl FromStr for Message
{
    type Err = MessageParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let mut prefix    = None;
        let mut command   = Command::Unknown;

        let mut arguments = s.split_whitespace().map(|s| s.to_string());
        let mut argument  = arguments.next().unwrap_or(String::new());

        if argument.starts_with(':') {
            argument.remove(0);
            prefix   = Some(argument);
            argument = arguments.next().unwrap_or(String::new());
        }

        if argument.is_empty() {
            return Err(MessageParseError::UnknownCommand);
        }

        let parse_result = match argument.as_ref() {
            "PASS" => parse_pass(arguments),
            _      => Err(MessageParseError::UnknownCommand),
        };

        match parse_result {
            Ok(command) => {
                Ok(Message {
                    prefix:  prefix,
                    command: command,
                })
            },
            Err(err) => Err(err)
        }
    }
}

fn parse_pass<Iter>(mut arguments: Iter) -> Result<Command, MessageParseError>
    where Iter: Iterator<Item=String>
{
    let password = match arguments.next() {
        Some(arg) => arg,
        None      => return Err(MessageParseError::NeedMoreParams),
    };

    Ok(Command::Pass{password: password})
}

#[cfg(test)]
mod test
{
    use super::Message;
    use super::Command;
    use super::MessageParseError;

    #[test]
    fn parse_with_prefix()
    {
        let message = ":some_prefix PASS args".parse::<Message>();
        assert!(message.is_ok());

        let prefix = message.ok().unwrap().prefix;
        assert!(prefix.is_some());
        assert_eq!("some_prefix", &prefix.unwrap());
    }

    #[test]
    fn parse_without_prefix()
    {
        let message = "PASS args".parse::<Message>();
        assert!(message.is_ok());

        let prefix = message.ok().unwrap().prefix;
        assert!(prefix.is_none());
    }

    #[test]
    fn parse_empty_line()
    {
        let message = "".parse::<Message>();

        assert!(message.is_err());
        assert_eq!(MessageParseError::UnknownCommand, message.err().unwrap());
    }

    #[test]
    fn parse_prefix_then_empty_line()
    {
        let message = ":some_prefix".parse::<Message>();

        assert!(message.is_err());
        assert_eq!(MessageParseError::UnknownCommand, message.err().unwrap());
    }

    #[test]
    fn parse_unknown_command()
    {
        let message = "something args".parse::<Message>();

        assert!(message.is_err());
        assert_eq!(MessageParseError::UnknownCommand, message.err().unwrap());
    }

    #[test]
    fn parse_command_pass()
    {
        let message = "PASS args".parse::<Message>();
        assert!(message.is_ok());

        let command = message.ok().unwrap().command;
        assert_eq!(Command::Pass{password: "args".to_string()}, command);
    }

    #[test]
    fn parse_command_pass_missing_arg()
    {
        let message = "PASS".parse::<Message>();

        assert!(message.is_err());
        assert_eq!(MessageParseError::NeedMoreParams, message.err().unwrap());
    }
}
