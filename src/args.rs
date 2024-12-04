use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub priv_key: String,

    #[arg(long)]
    pub pub_key: String,
}
