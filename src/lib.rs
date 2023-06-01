pub mod ap_error;
pub mod parser;

#[cfg(test)]
mod tests {
    use std::error::Error;
    use crate::parser::{Parser, Policy};

    #[test]
    fn create_config() -> Result<(), Box<dyn Error>> {
        let env_args = vec!(String::from("--hostname"), String::from("localhost"));
        let options = Parser::new()
            .arg("hostname", 'h', Policy::Required)
            .arg("port", 'p', Policy::Default(String::from("8080")))
            .run(&env_args)?;

        let hostname = options.get("hostname")?;
        assert_eq!(hostname, "localhost");
        let hostname = options.get_short('h')?;
        assert_eq!(hostname, "localhost");

        let port = options.get("port")?;
        assert_eq!(port, "8080");
        Ok(())
    }
}
