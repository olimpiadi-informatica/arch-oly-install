use args::Args;
use base::base_setup;
use base_gui::base_gui_setup;
use clap::Parser;
use color_eyre::eyre::Result;
use contest_software::{browser, compilers, debuggers, editors};
use lockdown::lockdown;
use pre::pre_install_scripts;
use users::{additional_user_files, setup_users};

use crate::base_gui::backgrounds;

mod args;
mod base;
mod base_gui;
mod contest_software;
mod lockdown;
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
    backgrounds(&args)?;

    browser(&args)?;
    compilers()?;
    debuggers()?;
    editors(&args)?;

    lockdown(&args)?;

    Ok(())
}
