use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct Args {
    #[arg(long, default_value = "Europe/Zurich")]
    pub timezone: String,

    #[arg(long, default_value = "en_US")]
    pub locale: String,

    #[arg(long, default_value = "olympiads")]
    pub contestant_account: String,

    #[arg(long, default_value = "orga")]
    pub orga_account: String,

    #[arg(long)]
    pub password: String,

    #[clap(long, value_parser)]
    pub github: Vec<String>,

    #[clap(long, value_parser)]
    pub ssh: Vec<String>,

    #[clap(long, default_value_t = false)]
    pub virtualbox: bool,

    #[clap(long, default_value_t = false)]
    pub disable_wayland: bool,

    #[clap(long, value_parser)]
    pub additional_user_files: Vec<PathBuf>,

    #[clap(long)]
    pub nowait: bool,

    /// YYYY/MM/DD
    #[clap(long)]
    pub pin_packages: Option<String>,

    #[clap(long)]
    pub homepage: Option<String>,

    #[clap(long, default_value_t = false)]
    pub pycharm: bool,

    #[clap(long, default_value_t = false)]
    pub codeblocks: bool,
}
