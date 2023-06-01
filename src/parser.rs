use std::collections::HashMap;
use crate::ap_error::ApError;

pub struct Parser {
    args: Vec<Arg>
}

impl Parser {

    pub fn new() -> Self {
        Self { args: vec!() }
    }

    pub fn arg(&mut self, arg: &str, arg_short: char, policy: Policy) -> &mut Self {
        let arg = Arg::new(arg, arg_short, policy);
        self.args.push(arg);
        self
    }

    pub fn run(&self, env_args: &Vec<String>) -> Result<Options, ApError> {
        let mut options = Options::new();
        let args = self.get_args(false)?;
        let args_short = self.get_args(true)?;

        for pair in env_args.chunks(2) {
            if let [key, value] = pair {
                let k = key.as_str();
                if let Some(arg) = args.get(k).or(args_short.get(k)) {
                    options.add(arg.arg.as_str(), arg.arg_short.as_str(), value);
                } else {
                    return Err(ApError::new(format!("Unknown argument {}", key)));
                }
            } else {
                return Err(ApError::new(format!("Invalid argument {}", pair[0])));
            }
        }

        // Set default values
        for arg in self.args.iter() {
            if options.get(&arg.arg).is_err() {
                if let Policy::Default(default) = &arg.policy {
                    options.add(&arg.arg, &arg.arg_short, default);
                } else {
                    return Err(ApError::new(format!("Missing argument {}", arg.arg)));
                }
            }
        }

        Ok(options)
    }

    /// Build a view of Args where the lookup key is Arg
    fn get_args(&self, arg_short: bool) -> Result<HashMap<&str, &Arg>, ApError> {
        let mut args: HashMap<&str, &Arg> = HashMap::new();
        for arg in self.args.iter() {
            let key = if arg_short { arg.arg_short.as_str() } else { arg.arg.as_str() };
            if args.contains_key(key) {
                return Err(ApError::new(format!("Duplicate argument {}", key)));
            }
            args.insert(key, arg);
        }
        Ok(args)
    }
}

#[derive(Debug)]
pub struct Options {
    options: HashMap<String, String>,
    options_short: HashMap<String, String>,
}

impl Options {
    pub fn new() -> Self {
        Self { options: HashMap::new(), options_short: HashMap::new() }
    }

    pub fn add(&mut self, option: &str, option_short: &str, value: &str) -> &mut Self {
        self.options.insert(String::from(option), String::from(value));
        self.options_short.insert(String::from(option_short), String::from(value));
        self
    }

    pub fn get(&self, option: &str) -> Result<&String, ApError> {
        let key = if option.starts_with("--") { String::from(option) } else { format!("--{}", option) };
        if let Some(value) = self.options.get(&key) {
            Ok(value)
        } else {
            Err(ApError::new(format!("Option {} not found", option)))
        }
    }

    pub fn get_short(&self, option_short: char) -> Result<&String, ApError> {
        let key = format!("-{}", option_short);
        if let Some(value) = self.options_short.get(&key) {
            Ok(value)
        } else {
            Err(ApError::new(format!("Short option {} not found", key)))
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct Arg {
    pub arg: String,
    pub arg_short: String,
    pub policy: Policy,
}

impl Arg {
    pub fn new(arg: &str, arg_short: char, policy: Policy) -> Self {
        Arg { arg: format!("--{}", arg), arg_short: format!("-{}", arg_short), policy }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Policy {
    Required,
    Default(String)
}