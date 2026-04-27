use icu::{
    experimental::messageformat::{MessageFormatter, OwnedInputs},
    locale::locale,
};

pub mod fatty_acids;
pub mod math;

mod macros;

https://github.com/JustFly1984/icu4x/tree/feature/messageformat-v2/tutorials

#[cfg(test)]
mod test {
    use super::*;
    use std::error::Error;

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        let formatter = MessageFormatter::builder().source("Hello, {$user}!").locale(locale!("en")).build().expect("valid MF2 source");
        let inputs = OwnedInputs::new().with_str("user", "Ada");
        let (text, _errors) = formatter.format_to_string(&inputs);
        assert_eq!(text, "Hello, Ada!");

        Ok(())
    }
}
