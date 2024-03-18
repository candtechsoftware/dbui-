#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod action;
pub mod app;
pub mod cli;
pub mod config;
pub mod components;
pub mod service;
pub mod tui;
pub mod utils;

use clap::Parser;
use cli::Cli;
use color_eyre::eyre::Result;
use components::{home::Home, Component};

use crate::{
  app::App,
  utils::{initialize_logging, initialize_panic_handler, version},
};

async fn tokio_main() -> Result<()> {
  initialize_logging()?;

  initialize_panic_handler()?;

  let args = Cli::parse();
  let mut components: Vec<Box<dyn Component>> = Vec::new();
  let home = Home::new().await;
  components.push(Box::new(home));

  let mut app = App::new(args.tick_rate, args.frame_rate, components)?;
  app.run().await?;

  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
  if let Err(e) = tokio_main().await {
    eprintln!("{} error: Something went wrong", env!("CARGO_PKG_NAME"));
    Err(e)
  } else {
    Ok(())
  }
}
