# ap

Minimal Argument Parsing Rust Lib

## Setup

Add the dependency to your project

```bash
# Cargo.toml
[dependencies]
ap = { git = "https://github.com/dberg/ap.git" }
```

## Usage

```rust
let env_args = env::args().skip(1).collect();
// for example
// let env_args = vec!(String::from("--hostname"), String::from("localhost"));
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
```
