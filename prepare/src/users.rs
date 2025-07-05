use std::fs::create_dir_all;

use crate::args::Args;
use crate::util::{iso_root, script};
use color_eyre::eyre::Result;

pub fn additional_user_files(args: &Args) -> Result<()> {
    let Args {
        contestant_account,
        additional_user_files,
        ..
    } = args;

    let file_dir = iso_root().join("install/additional_user_files");
    create_dir_all(&file_dir)?;

    for f in additional_user_files.iter() {
        std::fs::copy(f, file_dir.join(f.file_name().unwrap()))?;
    }

    if !additional_user_files.is_empty() {
        script!(
            "81-additional-user-files",
            "cp -av /install/additional_user_files/* ~{contestant_account}"
        );
    }
    Ok(())
}

pub fn setup_users(args: &Args) -> Result<()> {
    let Args {
        contestant_account,
        orga_account,
        password,
        github,
        ssh,
        ..
    } = args;

    let mut gen_authorized_keys = String::new();
    for gh in github.iter() {
        gen_authorized_keys = format!("{gen_authorized_keys}\ncurl https://github.com/{gh}.keys");
    }
    for key in ssh.iter() {
        gen_authorized_keys = format!("{gen_authorized_keys}\necho {key}");
    }

    script!(
        "70-sshd",
        r#"
pacman -S --noconfirm openssh rsync
systemctl enable sshd
gen_authorized_keys() {{
    {gen_authorized_keys}
}}
mkdir -p /root/.ssh /etc/skel/.ssh
gen_authorized_keys > /root/.ssh/authorized_keys
cp /root/.ssh/authorized_keys /etc/skel/.ssh/authorized_keys
"#
    );

    script!(
        "80-contestant",
        "useradd -m {contestant_account}\necho {contestant_account} | passwd -s {contestant_account}"
    );

    script!(
        "80-orga",
        r#"
pacman -S --noconfirm sudo
useradd -m {orga_account}
echo "{password}" | passwd -s {orga_account}
echo "{password}" | passwd -s root
echo "{orga_account} ALL=(ALL:ALL) NOPASSWD: ALL" > /etc/sudoers.d/orga_account
"#
    );
    Ok(())
}
