use clap::{Clap, AppSettings};

/// BuzzWord Generator Server
#[derive(Clap)]
#[clap(version = "1.0", author = "Aleksei T. <avrong@avrong.me>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Settings {
    /// Address to bind to
    #[clap(short, long, default_value = "127.0.0.1")]
    pub host: String,

    /// Port to bind to
    #[clap(short, long, default_value = "3000")]
    pub port: u16,

    /// Buzzword dictionary to use
    #[clap(short, long, default_value = "dictionary.toml")]
    pub dictionary: String,
}

impl Settings {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}