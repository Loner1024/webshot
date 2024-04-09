use anyhow::Result;
use clap::Parser;
use web_shot::web_shot;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    /// you want capture website
    #[arg(short, long)]
    url: String,

    /// set capture height
    #[arg(long)]
    height: Option<u32>,

    /// set capture width
    #[arg(long)]
    width: Option<u32>,

    /// capture full size page
    #[arg(short, long, default_value_t = false)]
    full: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    web_shot::shot(&args.url)?;
    Ok(())
}
