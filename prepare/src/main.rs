use args::Args;
use base::base_setup;
use base_gui::base_gui_setup;
use clap::Parser;
use color_eyre::eyre::Result;
use pre::pre_install_scripts;
use users::{additional_user_files, setup_users};

mod args;
mod base;
mod base_gui;
mod pre;
mod users;
mod util;

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    pre_install_scripts(&args)?;
    setup_users(&args)?;
    additional_user_files(&args)?;
    base_setup(&args)?;
    base_gui_setup(&args)?;

    Ok(())
}
