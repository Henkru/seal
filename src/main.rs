use anyhow::Result;
use app::App;
use clap::Parser;

mod app;
mod args;

fn main() -> Result<()> {
    let args = args::Args::parse();
    App::run(args)
}
