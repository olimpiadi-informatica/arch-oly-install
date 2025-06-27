use std::{
    fs::{File, create_dir_all},
    io::Write,
    path::{Path, PathBuf},
};

use clap::Parser;
use color_eyre::eyre::Result;

#[derive(clap::Parser)]
struct Args {
    #[arg(long, default_value = "Europe/Zurich")]
    timezone: String,

    #[arg(long, default_value = "en_US")]
    locale: String,

    #[arg(long, default_value = "olympiads")]
    contestant_account: String,

    #[arg(long, default_value = "orga")]
    orga_account: String,

    #[arg(long)]
    password: String,

    #[clap(long, value_parser)]
    github: Vec<String>,

    #[clap(long, value_parser)]
    ssh: Vec<String>,

    #[clap(long, default_value_t = false)]
    virtualbox: bool,

    #[clap(long, default_value_t = false)]
    disable_wayland: bool,

    #[clap(long, value_parser)]
    additional_user_files: Vec<PathBuf>,
}

const DIR: &str = "archiso-profile/airootfs/install";

fn add_script(name: &str, contents: &str) -> Result<()> {
    let dir = Path::new(DIR).join("scripts");
    create_dir_all(&dir)?;
    let mut f = File::create(dir.join(name))?;
    writeln!(f, "set -e")?;
    writeln!(f, "{}", contents)?;
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let file_dir = Path::new(DIR).join("additional_user_files");
    create_dir_all(&file_dir)?;

    for f in args.additional_user_files.iter() {
        std::fs::copy(f, file_dir.join(f.file_name().unwrap()))?;
    }

    add_script(
        "00-time",
        &format!(
            "ln -sf /usr/share/zoneinfo/{} /etc/localtime\nhwclock --systohc",
            args.timezone
        ),
    )?;

    add_script(
        "00-locale",
        &format!(
            r#"
echo "{}.UTF-8 UTF-8" > /etc/locale.gen
locale-gen
echo LANG={}.UTF-8 > /etc/locale.conf"#,
            args.locale, args.locale
        ),
    )?;

    let mut gen_authorized_keys = String::new();
    for gh in args.github.iter() {
        gen_authorized_keys = format!("{gen_authorized_keys}\ncurl https://github.com/{gh}.keys");
    }
    for key in args.ssh.iter() {
        gen_authorized_keys = format!("{gen_authorized_keys}\necho {key}");
    }

    add_script(
        "70-sshd",
        &format!(
            r#"
pacman -S --noconfirm openssh
systemctl enable sshd
gen_authorized_keys() {{
    {gen_authorized_keys}
}}
mkdir -p /root/.ssh /etc/skel/.ssh
gen_authorized_keys > /root/.ssh/authorized_keys
cp /root/.ssh/authorized_keys /etc/skel/.ssh/authorized_keys
"#
        ),
    )?;

    add_script(
        "80-contestant",
        &format!(
            "useradd -m {}\necho {} | passwd -s {}",
            args.contestant_account, args.contestant_account, args.contestant_account
        ),
    )?;

    add_script(
        "80-orga",
        &format!(
            r#"
pacman -S --noconfirm sudo
useradd -m {}
echo "{}" | passwd -s {}
echo "{}" | passwd -s root
echo "{} ALL=(ALL:ALL) NOPASSWD: ALL" > /etc/sudoers.d/orga_account
"#,
            args.orga_account, args.password, args.orga_account, args.password, args.orga_account
        ),
    )?;

    add_script(
        "81-additional-user-files",
        &format!(
            "cp -av /install/additional_user_files/* ~{}",
            args.contestant_account
        ),
    )?;

    add_script(
        "85-gnome",
        &format!(
            r#"
pacman -S --noconfirm gnome gnome-extra gdm
pacman -R --noconfirm gnome-tour
systemctl enable gdm
echo -e "[daemon]\nAutomaticLoginEnable=True\nAutomaticLogin={}{}" > /etc/gdm/custom.conf
"#,
            args.contestant_account,
            if args.disable_wayland {
                "\nWaylandEnable=false"
            } else {
                ""
            }
        ),
    )?;

    if args.disable_wayland {
        add_script("85-xorg", "pacman -S --noconfirm xorg-server")?;
    }

    if args.virtualbox {
        add_script(
            "85-virtualbox",
            "pacman -S --noconfirm virtualbox virtualbox-host-modules-arch",
        )?;
    }

    add_script(
        "90-networkd",
        r#"
systemctl enable systemd-networkd
echo -e "[Match]\nName=en*\nName=eth*\n[Network]\nDHCP=yes" >> /etc/systemd/network/20-ethernet.network
"#,
    )?;

    add_script("98-mkinitcpio", "mkinitcpio -P linux")?;
    add_script(
        "99-grub",
        r#"
pacman -S --noconfirm grub efibootmgr
grub-install --target=x86_64-efi --efi-directory=/boot
grub-mkconfig -o /boot/grub/grub.cfg
"#,
    )?;

    Ok(())
}
