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

    #[clap(long, value_parser)]
    pub additional_server_names: Vec<String>,

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

    #[clap(long, default_value = "10.0.0.1")]
    pub server_ip: String,

    #[clap(long, default_value_t = false)]
    pub pixie: bool,

    #[clap(long, default_value_t = false)]
    pub node_exporter: bool,

    #[clap(long, default_value_t = false)]
    pub no_lockdown: bool,

    #[clap(long, default_value = "172.26.0.0/16")]
    pub orga_net: String,

    #[clap(long, default_value_t = false)]
    pub backgrounds: bool,

    #[clap(long)]
    pub default_background: Option<PathBuf>,
}
