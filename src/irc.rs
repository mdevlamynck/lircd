use std::io::Write;

#[derive(Debug, PartialEq)]
pub struct Irc<T>
    where T: Write
{
    pub users: Vec<Client<T>>,
}

impl<T> Irc<T>
    where T: Write
{
    pub fn new() -> Irc<T>
    {
        Irc::<T> {
            users: Vec::new()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Client<T>
    where T: Write
{
    pub socket: T,
}

impl<T> Client<T>
    where T: Write
{
    pub fn new(socket: T) -> Client<T>
    {
        Client::<T> {
            socket: socket,
        }
    }
}

#[cfg(test)]
mod test
{
    #[test]
    fn test_irc_new()
    {
        let irc = super::Irc::<Vec<u8>>::new();

        assert_eq!(Vec::<super::Client<Vec<u8>>>::new(), irc.users);
    }

    #[test]
    fn test_client_new()
    {
        let write_buffer = Vec::new();
        let client = super::Client::<Vec<u8>>::new(write_buffer);

        assert_eq!(Vec::<u8>::new(), client.socket);
    }
}
