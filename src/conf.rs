use clap::Parser;
use lazy_static::lazy_static;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Configuration {
    /* REQUIRED PARAMS */
    /// Telegram bot token
    #[clap(short = 't', long = "token", env = "LEO_TG_TOKEN")]
    pub telegram_token: String,

    /// Listening host
    #[clap(short = 'H', long = "host", env = "LEO_HOST")]
    pub host: String,

    /// DB link in format `$DB_DRIVER://$USERNAME:$PASSWORD@$HOST:$PORT/$DB_NAME`
    #[clap(short = 'd', long = "db", env = "LEO_DB_URL")]
    pub db: String,

    /* OPTIONAL PARAMS */
    /// Listening port
    #[clap(short = 'P', long = "port", env = "LEO_PORT", default_value = "8888")]
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    pub port: u16,

    /// Number of workers
    #[clap(long = "workers", env = "LEO_WORKERS", default_value = "4")]
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    pub workers: u16,

    #[clap(
        long = "quiz-round-time",
        env = "LEO_QUIZ_ROUND_TIME",
        default_value = "15"
    )]
    #[arg(value_parser = clap::value_parser!(u16).range(5..=600))]
    pub quiz_round_time: u16,

    #[clap(
        long = "quiz-rounds-count",
        env = "LEO_QUIZ_ROUNDS_COUNT",
        default_value = "5"
    )]
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    pub quiz_rounds_count: u16,

    #[clap(short = 'T', long = "secret-token", env = "LEO_TG_SECRET_TOKEN")]
    pub telegram_secret_token: Option<String>,

    #[clap(env = "LEO_TG_MAX_CONNECTION", default_value = "40")]
    #[arg(value_parser = clap::value_parser!(u8).range(1..=100))]
    pub telegram_max_connections: u8,

    #[clap(env = "LEO_CSV_PATH", default_value = "questions/questions.csv")]
    pub csv_path: String,
}

pub fn get_config() -> &'static Configuration {
    lazy_static! {
        static ref CONFIG: Configuration = Configuration::parse();
    };
    &CONFIG
}
