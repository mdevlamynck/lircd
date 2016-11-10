extern crate mioco;

use std::str::FromStr;

pub struct Message
{
    pub prefix:    Option<String>, // servername the message originates from
    pub command:   String,         // command
    pub arguments: Vec<String>,    // iterator over arguments
}

pub mod reply
{
    pub const WELCOME            : &'static str = "001";
    pub const YOUR_HOST          : &'static str = "002";
    pub const CREATED            : &'static str = "003";
    pub const MY_INFO            : &'static str = "004";
    pub const BOUNCE             : &'static str = "005";
    pub const USER_HOST          : &'static str = "302";
    pub const IS_ON              : &'static str = "303";
    pub const AWAY               : &'static str = "301";
    pub const UN_AWAY            : &'static str = "305";
    pub const NOW_AWAY           : &'static str = "306";
    pub const WHO_IS_USER        : &'static str = "311";
    pub const WHO_IS_SERVER      : &'static str = "312";
    pub const WHO_IS_OPERATOR    : &'static str = "313";
    pub const WHO_IS_IDLE        : &'static str = "317";
    pub const END_OF_WHO_IS      : &'static str = "318";
    pub const WHO_IS_CHANNELS    : &'static str = "319";
    pub const WHO_WAS_USER       : &'static str = "314";
    pub const END_OF_WHO_WAS     : &'static str = "369";
    pub const LIST_START         : &'static str = "321";
    pub const LIST               : &'static str = "322";
    pub const LIST_END           : &'static str = "323";
    pub const UNIQ_OP_IS         : &'static str = "325";
    pub const CHANNEL_MODE_IS    : &'static str = "324";
    pub const NO_TOPIC           : &'static str = "331";
    pub const TOPIC              : &'static str = "332";
    pub const INVITING           : &'static str = "341";
    pub const SUMMONING          : &'static str = "342";
    pub const INVITE_LIST        : &'static str = "346";
    pub const END_OF_INVITE_LIST : &'static str = "347";
    pub const EXCEPT_LIST        : &'static str = "348";
    pub const END_OF_EXCEPT_LIST : &'static str = "349";
    pub const VERSION            : &'static str = "351";
    pub const WHO_REPLY          : &'static str = "352";
    pub const END_OF_WHO         : &'static str = "315";
    pub const NAM_REPLY          : &'static str = "353";
    pub const END_OF_NAMES       : &'static str = "366";
    pub const LINKS              : &'static str = "364";
    pub const END_OF_LINKS       : &'static str = "365";
    pub const BAN_LIST           : &'static str = "367";
    pub const END_OF_BAN_LIST    : &'static str = "368";
    pub const INFO               : &'static str = "371";
    pub const END_INFO           : &'static str = "374";
    pub const MOTD_START         : &'static str = "375";
    pub const MOTD               : &'static str = "372";
    pub const END_OF_MOTD        : &'static str = "376";
    pub const YOU_RE_OPER        : &'static str = "381";
    pub const RE_HASHING         : &'static str = "382";
    pub const YOU_RE_SERVICE     : &'static str = "383";
    pub const TIME               : &'static str = "391";
    pub const USER_START         : &'static str = "392";
    pub const USERS              : &'static str = "393";
    pub const END_OF_USERS       : &'static str = "394";
    pub const NO_USERS           : &'static str = "395";
    pub const TRACE_LINK         : &'static str = "200";
    pub const TRACE_CONNECTING   : &'static str = "201";
    pub const TRACE_HANDSHAKE    : &'static str = "202";
    pub const TRACE_UNKNOWN      : &'static str = "203";
    pub const TRACE_OPERATOR     : &'static str = "204";
    pub const TRACE_USER         : &'static str = "205";
    pub const TRACE_SERVER       : &'static str = "206";
    pub const TRACE_SERVICE      : &'static str = "207";
    pub const TRACE_NEW_TYPE     : &'static str = "208";
    pub const TRACE_CLASS        : &'static str = "209";
    pub const TRACE_CONNECT      : &'static str = "210";
    pub const TRACE_LOG          : &'static str = "261";
    pub const TRACE_END          : &'static str = "262";
    pub const STATS_LINK_INFO    : &'static str = "211";
    pub const STATS_COMMANDS     : &'static str = "212";
    pub const END_OF_STATS       : &'static str = "219";
    pub const STATS_UPTIME       : &'static str = "242";
    pub const STATS_O_LINE       : &'static str = "243";
    pub const UMODE_IS           : &'static str = "221";
    pub const SERV_LIST          : &'static str = "234";
    pub const SERV_LIST_END      : &'static str = "235";
    pub const L_USER_CLIENT      : &'static str = "251";
    pub const L_USER_OP          : &'static str = "252";
    pub const L_USER_UNKNOWN     : &'static str = "253";
    pub const L_USER_CHANNELS    : &'static str = "254";
    pub const L_USER_ME          : &'static str = "255";
    pub const ADMIN_ME           : &'static str = "256";
    pub const ADMIN_LOC_1        : &'static str = "257";
    pub const ADMIN_LOC_2        : &'static str = "258";
    pub const ADMIN_EMAIL        : &'static str = "259";
    pub const TRY_AGAIN          : &'static str = "263";
}

pub mod error
{
    pub const NO_SUCH_NICK         : &'static str = "401";
    pub const NO_SUCH_SERVER       : &'static str = "402";
    pub const NO_SUCH_CHANNEL      : &'static str = "403";
    pub const CAN_NOT_SEND_TO_CHAN : &'static str = "404";
    pub const TOO_MANY_CHANNELS    : &'static str = "405";
    pub const WAS_NO_SUCH_NICK     : &'static str = "406";
    pub const TOO_MANY_TARGETS     : &'static str = "407";
    pub const NO_SUCH_SERVICE      : &'static str = "408";
    pub const NO_ORIGIN            : &'static str = "409";
    pub const NO_RECIPIENT         : &'static str = "411";
    pub const NO_TEXT_TO_SEND      : &'static str = "412";
    pub const NO_TOP_LEVEL         : &'static str = "413";
    pub const WILD_TOP_LEVEL       : &'static str = "414";
    pub const BAD_MASK             : &'static str = "415";
    pub const UNKNOWN_COMMAND      : &'static str = "421";
    pub const NO_MOTD              : &'static str = "421";
    pub const NO_ADMIN_INFO        : &'static str = "423";
    pub const FILE_ERROR           : &'static str = "424";
    pub const NO_NICKNAME_GIVEN    : &'static str = "431";
    pub const ERRONEUS_NICKNAME    : &'static str = "432";
    pub const NICKNAME_IN_USE      : &'static str = "433";
    pub const NICK_COLLISION       : &'static str = "436";
    pub const UNAVAIL_RESOURCE     : &'static str = "437";
    pub const USER_NOT_IN_CHANNEL  : &'static str = "441";
    pub const NOT_ON_CHANNEL       : &'static str = "442";
    pub const USER_ON_CHANNEL      : &'static str = "443";
    pub const NO_LOGIN             : &'static str = "444";
    pub const SUMMON_DISABLED      : &'static str = "445";
    pub const USER_DISABLED        : &'static str = "446";
    pub const NOT_REGISTERED       : &'static str = "451";
    pub const NEED_MORE_PARAMS     : &'static str = "461";
    pub const ALREADY_REGISTERED   : &'static str = "462";
    pub const NO_PERM_FOR_HOST     : &'static str = "463";
    pub const PASSWD_MISMATCH      : &'static str = "464";
    pub const YOU_RE_BANNED_CREEP  : &'static str = "465";
    pub const YOU_WILL_BE_BANNED   : &'static str = "466";
    pub const KEYSET               : &'static str = "467";
    pub const CHANNEL_IS_FULL      : &'static str = "471";
    pub const UNKOWN_MODE          : &'static str = "472";
    pub const INVITE_ONLY_CHAN     : &'static str = "473";
    pub const BANNED_FROM_CHAN     : &'static str = "474";
    pub const BAD_CHANNEL_KEY      : &'static str = "475";
    pub const BAD_CHAN_MASK        : &'static str = "476";
    pub const NO_CHAN_MODES        : &'static str = "477";
    pub const BAN_LIST_FULL        : &'static str = "478";
    pub const NO_PRIVILEGES        : &'static str = "481";
    pub const CHAN_O_PRIVS_NEEDED  : &'static str = "482";
    pub const CANT_KILL_SERVER     : &'static str = "483";
    pub const RESTRICTED           : &'static str = "484";
    pub const UNIQ_O_PRIVIS_NEEDED : &'static str = "485";
    pub const NO_OPER_HOST         : &'static str = "491";
    pub const UMODE_UNKNOWN_FLAG   : &'static str = "501";
    pub const USER_DONT_MATCH      : &'static str = "502";
}

#[derive(Debug, PartialEq)]
pub enum MessageParseError
{
    NeedMoreParams,
    UnknownCommand,
    SyntaxError,
}

impl FromStr for Message
{
    type Err = MessageParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let mut prefix = None;

        let mut words  = s.split_whitespace().map(|s| s.to_string());
        let mut word   = words.next().unwrap_or(String::new());

        if word.starts_with(':') {
            word.remove(0);
            prefix = Some(word);
            word   = words.next().unwrap_or(String::new());
        }

        if word.is_empty() {
            return Err(MessageParseError::SyntaxError);
        }

        Ok(Message {
            prefix:    prefix,
            command:   word,
            arguments: words.fold_trailing().collect(),
        })
    }
}

trait ArgumentsIterator<Iter>
    where Iter: Iterator<Item=String>
{
    fn fold_trailing(self) -> FoldTrailing<Iter>;
}

impl<Iter> ArgumentsIterator<Iter> for Iter
    where Iter: Iterator<Item=String>
{
    fn fold_trailing(self) -> FoldTrailing<Iter>
    {
        FoldTrailing { iter: self }
    }
}

pub struct FoldTrailing<Iter>
    where Iter: Iterator<Item=String>
{
    iter: Iter,
}

impl<Iter> Iterator for FoldTrailing<Iter>
    where Iter: Iterator<Item=String>
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item>
    {
        match self.iter.next() {
            Some(mut value) => {
                if value.starts_with(':') {
                    value.remove(0);
                    for next in self.iter.by_ref() {
                        value.push(' ');
                        value.push_str(&next);
                    }
                }
                Some(value)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod test
{
    use super::Message;
    use super::MessageParseError;

    #[test]
    fn parse_empty_line()
    {
        let message = "".parse::<Message>();

        assert!(message.is_err());
        assert_eq!(MessageParseError::SyntaxError, message.err().unwrap());
    }

    #[test]
    fn parse_prefix_then_empty_line()
    {
        let message = ":some_prefix".parse::<Message>();

        assert!(message.is_err());
        assert_eq!(MessageParseError::SyntaxError, message.err().unwrap());
    }

    #[test]
    fn parse_with_prefix()
    {
        let message = ":some_prefix some_command some arguments".parse::<Message>();
        assert!(message.is_ok());

        let prefix = message.ok().unwrap().prefix;
        assert!(prefix.is_some());
        assert_eq!("some_prefix", &prefix.unwrap());
    }

    #[test]
    fn parse_without_prefix()
    {
        let message = "not_a_prefix some arguments".parse::<Message>();
        assert!(message.is_ok());

        let prefix = message.ok().unwrap().prefix;
        assert!(prefix.is_none());
    }

    #[test]
    fn parse_command()
    {
        let message = "not_a_prefix some arguments".parse::<Message>();
        assert!(message.is_ok());

        let command = message.ok().unwrap().command;
        assert_eq!("not_a_prefix", &command);
    }

    #[test]
    fn parse_arguments_fold_trailing_groups_after_colon()
    {
        let message = "some_command some argument :some trailing".parse::<Message>();
        assert!(message.is_ok());

        let arguments = message.ok().unwrap().arguments;
        assert_eq!(vec!["some".to_string(), "argument".to_string(), "some trailing".to_string()], arguments);
    }

    #[test]
    fn parse_arguments_fold_trailing_keeps_identical_if_no_colon()
    {
        let message = "some_command some argument no trailing".parse::<Message>();
        assert!(message.is_ok());

        let arguments = message.ok().unwrap().arguments;
        assert_eq!(vec!["some".to_string(), "argument".to_string(), "no".to_string(), "trailing".to_string()], arguments);
    }

    #[test]
    fn parse_arguments_fold_trailing_no_content()
    {
        let message = "some_command".parse::<Message>();
        assert!(message.is_ok());

        let arguments = message.ok().unwrap().arguments;
        assert!(arguments.is_empty());
    }
}
