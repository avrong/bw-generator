use clap::{Arg, App};

pub struct Settings {
    pub host: String,
    pub port: u16,
    pub dictionary_name: String
}

impl Settings {
    pub fn parse() -> Settings {
        let matches = App::new("BuzzWord Generator Server")
            .version("1.0")
            .arg(Arg::with_name("host")
                .short("h")
                .long("host")
                .value_name("TEXT")
                .help("The interface to bind to (default: 127.0.0.1)")
                .takes_value(true))
            .arg(Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("INTEGER")
                .help("The port to bind to (default: 3000)")
                .takes_value(true))
            .arg(Arg::with_name("dictionary")
                .short("d")
                .long("dict")
                .value_name("FILE")
                .help("The buzzword dictionary to use (default: dictionary.toml)")
                .takes_value(true))
            .get_matches();

        Settings {
            host: matches.value_of("host").unwrap_or("127.0.0.1").to_owned(),
            port: matches.value_of("port").unwrap_or("3000").parse().expect("Cannot parse port to number"),
            dictionary_name: matches.value_of("dictionary").unwrap_or("dictionary.toml").to_owned()
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}