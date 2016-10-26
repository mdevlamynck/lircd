pub fn some_function() -> bool
{
    true
}

#[cfg(test)]
mod test {

    #[test]
    fn test()
    {
        assert_eq!(true, super::some_function())
    }
}
