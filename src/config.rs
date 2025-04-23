pub struct Config {
    pub md_path: String,
    pub output_path: String
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip path of executable

        // Read program arguments
        let md_path = match args.next() {
            Some(path) => path,
            None => return Err("No markdown file specified"),
        };

        let output_path = match args.next() {
            Some(path) => path,
            None => "output.html".to_string()
        };

        // TODO: Error check arguments <md_path>, <output_path>
        // ...

        Ok(Config { md_path, output_path })
    }
}