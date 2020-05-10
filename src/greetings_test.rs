#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn hello_world_basic_greeting() {
        assert_eq!(basic_greeting(), "Hello World!");
    }

    #[test]
    fn hello_german_greeting_name() {
        assert_eq!(greeting_name("German"), String::from("Hello German!"));
    }
}
