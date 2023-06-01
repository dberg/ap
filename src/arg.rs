#[derive(Debug)]
pub struct Arg {
    pub arg: String,
    pub arg_short: String,
    pub policy: Policy,
}

impl Arg {
    pub fn new(arg: &str, arg_short: char, policy: Policy) -> Self {
        Arg { arg: format!("--{}", arg), arg_short: format!("-{}", arg_short), policy }
    }
}

#[derive(Debug, PartialEq)]
pub enum Policy {
    Optional,
    Required,
    Default(String)
}
