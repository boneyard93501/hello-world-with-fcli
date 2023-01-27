#![allow(non_snake_case)]
use marine_rs_sdk::marine;

pub fn main() {}

#[marine]
pub fn hello_world() -> String {
    format!("Hello, Fluence!")
}

#[marine]
pub fn greeting(name: String) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {

    use marine_rs_sdk_test::marine_test;
    
    #[marine_test(
        config_path = "../../../.fluence/tmp/Config.toml",
        modules_dir = "../../services/modules/hello_world/target/wasm32-wasi/release/"
    )]
    fn test_hello_world(greeter: marine_test_env::hello_world::ModuleInterface) {
        let res = greeter.hello_world();
        assert_eq!(res, "Hello, World!".to_string());
    }

    /*
    #[marine_test(
        config_path = "../../configs/Config.toml",
        modules_dir = "../target/wasm32-wasi/release/"
    )]
    fn test_greeting(greeter: marine_test_env::hello_world::ModuleInterface) {
        let name = "Fluence";
        let res = greeter.greeting(name.to_string());
        assert_eq!(res, format!("Hello, {}!", name));
    }
    */
}