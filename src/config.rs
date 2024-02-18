use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    #[clap(env, long)]
    pub database_url: String,
}
