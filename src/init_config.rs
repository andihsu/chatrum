pub mod init_config {

    use serde_json::Value;
    use std::fs;

    pub struct Config {
        pub addr: String,
        pub name: String,
    }

    impl Config {
        pub fn new() -> Self {
            let config = fs::read_to_string("./src/config.json")
                .expect("Should have been able to read the file");
            println!("Using default configurations in config.json.");

            let parsed_config: Value = read_json(&config);

            let port = parsed_config["port"].clone().to_string();
            let addr = format!("0.0.0.0:{}", port);
            let name = parsed_config["name"].clone().to_string();

            println!("Running on {}", port.clone());

            Config {
                addr,
                name,
            }
        }
    }


    fn read_json(raw_json: &str) -> Value {
        let parsed: Value = serde_json::from_str(raw_json)
            .expect("Cannot read the configuration file or the json file has not been formatted!");
        parsed
    }

}
