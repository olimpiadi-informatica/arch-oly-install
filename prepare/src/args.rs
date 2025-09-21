use std::{net::IpAddr, path::PathBuf};

#[derive(clap::Parser)]
#[command(term_width = 0)]
pub struct Args {
    /// The system-wide timezone
    #[arg(long, default_value = "UTC")]
    pub timezone: String,

    /// The system-wide locale
    #[arg(long, default_value = "en_US")]
    pub locale: String,

    /// The GNOME locale
    #[arg(long)]
    pub gnome_locale: Option<String>,

    /// The contestant account name
    #[arg(long, default_value = "olympiads")]
    pub contestant_account: String,

    /// The organizer account name
    #[arg(long, default_value = "orga")]
    pub orga_account: String,

    /// Password for root and orga account
    #[arg(long)]
    pub password: String,

    /// GitHub usernames to fetch SSH keys from
    #[clap(long, value_parser)]
    pub ssh_github: Vec<String>,

    /// Additional SSH public keys
    #[clap(long, value_parser)]
    pub ssh: Vec<String>,

    /// Install VirtualBox
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

    /// Pin packages to a specific date, format YYYY/MM/DD
    #[clap(long)]
    pub pin_packages: Option<String>,

    /// The browser's homepage
    #[clap(long)]
    pub homepage: Option<String>,

    /// Install PyCharm
    #[clap(long, default_value_t = false)]
    pub pycharm: bool,

    /// Install Code::Blocks
    #[clap(long, default_value_t = false)]
    pub codeblocks: bool,

    /// The server IP address
    #[clap(long, default_value = "10.0.0.1")]
    pub server_ip: IpAddr,

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

    /// GNOME keyboard layout
    #[clap(long, value_parser)]
    pub keyboard_layout: Vec<String>,

    #[clap(long, value_parser)]
    pub ca_certificate: Vec<PathBuf>,

    #[clap(long)]
    pub firefox_policies: Option<PathBuf>,
}
