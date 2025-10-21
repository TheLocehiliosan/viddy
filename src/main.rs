#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod action;
pub mod app;
mod bytes;
pub mod cli;
pub mod components;
pub mod config;
mod diff;
mod exec;
pub mod mode;
mod old_config;
mod runner;
mod search;
mod store;
mod termtext;
pub mod tui;
mod types;
pub mod utils;
mod widget;

use std::path::PathBuf;

use chrono::Duration;
use clap::Parser;
use cli::Cli;
use color_eyre::eyre::{eyre, Result};
use directories::ProjectDirs;
use store::Store;

use crate::{
    app::App,
    utils::{initialize_logging, initialize_panic_handler, version},
};

async fn tokio_main() -> Result<()> {
    initialize_logging()?;

    initialize_panic_handler()?;

    let args = Cli::parse();
    let interval = Duration::from(args.interval);

    if args.load.is_none() && args.command.is_empty() {
        return Err(eyre!("No command provided"));
    }
    if args.load.is_some() && args.command.len() > 1 {
        return Err(eyre!("Can not use --load with command"));
    }

    if let Some(l) = &args.load {
        let store = store::sqlite::SQLiteStore::new(l.clone(), false)?;
        let mut app = App::new(args.clone(), store, true)?;
        app.run().await?;
    } else if let Some(b) = &args.save {
        let store = store::sqlite::SQLiteStore::new(b.clone(), true)?;
        let mut app = App::new(args.clone(), store, false)?;
        app.run().await?;
    } else {
        let store = store::memory::MemoryStore::new();
        let mut app = App::new(args, store, false)?;
        app.run().await?;
    }

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
