use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct App {
    #[arg(short, long)]
    pub dir: String,

    #[arg(short, long)]
    pub file: String,
}