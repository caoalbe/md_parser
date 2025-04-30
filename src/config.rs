pub struct Config {
    pub md_path: String,
    pub output_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip path of executable

        // Read program arguments
        let md_path: String = match args.next() {
            Some(path) => path,
            None => return Err("No markdown file specified"),
        };

        let output_path: String = match args.next() {
            Some(path) => path,
            None => {
                let mut fallback_name: String = match md_path.split('.').nth(0) {
                    Some(text) => text.to_string(),
                    None => "output".to_string(),
                };
                fallback_name.push_str(".html");
                fallback_name
            }
        };

        Ok(Config {
            md_path,
            output_path,
        })
    }
}
