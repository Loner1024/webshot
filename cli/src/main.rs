use ::web_shot::web_shot::Captureshot;
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    /// you want capture website
    #[arg(short, long)]
    url: String,

    /// set capture height
    #[arg(long, default_value_t = 1080)]
    height: u32,

    /// set capture width
    #[arg(long, default_value_t = 1280)]
    width: u32,

    /// output image quality
    #[arg(short, long, default_value_t = 75)]
    quality: u32,

    /// capture full size page
    #[arg(short, long, default_value_t = false)]
    full: bool,

    /// output file
    #[arg(short, long, default_value_t = String::from("./screen.png"))]
    out_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let captureshot = Captureshot::new(args.url, args.width, args.height, args.quality, args.full);
    captureshot
        .shot()
        .await?
        .write_to_file(&args.out_file)
        .await?;
    Ok(())
}
