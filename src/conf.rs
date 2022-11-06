use clap::Parser;
use lazy_static::lazy_static;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Configuration {
    /// Telegram bot token
    #[clap(short = 't', long = "token", env = "LEO_TG_TOKEN")]
    pub telegram_token: String,

    /// Listening host
    #[clap(short = 'H', long = "host", env = "LEO_HOST")]
    pub host: String,

    /// DB link in format `$DB_DRIVER://$USERNAME:$PASSWORD@$HOST:$PORT/$DB_NAME`
    #[clap(short = 'd', long = "db", env = "LEO_DB_URL")]
    pub db: String,

    #[clap(short = 'T', long = "secret-token", env = "LEO_TG_SECRET_TOKEN")]
    pub telegram_secret_token: Option<String>,

    /// Listening port
    #[clap(short = 'P', long = "port", env = "LEO_PORT", default_value = "8888")]
    pub port: u16,

    /// Number of workers
    #[clap(long = "workers", env = "LEO_WORKERS", default_value = "4")]
    pub workers: usize,

    #[clap(
        long = "quiz-round-time",
        env = "LEO_QUIZ_ROUND_TIME",
        default_value = "15"
    )]
    pub quiz_round_time: usize,

    #[clap(
        long = "quiz-rounds-count",
        env = "LEO_QUIZ_ROUNDS_COUNT",
        default_value = "5"
    )]
    pub quiz_rounds_count: usize,
}

pub fn get_config() -> &'static Configuration {
    lazy_static! {
        static ref CONFIG: Configuration = Configuration::parse();
    };
    &CONFIG
}
