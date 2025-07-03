use color_eyre::eyre::Result;

use std::{
    fs::{File, create_dir_all},
    io::Write,
    path::Path,
};

fn add_script(dir: &Path, name: &str, contents: &str) -> Result<()> {
    create_dir_all(&dir)?;
    let mut f = File::create(dir.join(name))?;
    writeln!(f, "set -e")?;
    writeln!(f, "{}", contents)?;
    Ok(())
}

pub fn iso_root() -> &'static Path {
    Path::new("archiso-profile/airootfs")
}

pub fn add_pre_script(name: &str, contents: &str) -> Result<()> {
    add_script(&iso_root().join("pre_install"), name, contents)
}

pub fn add_post_script(name: &str, contents: &str) -> Result<()> {
    add_script(&iso_root().join("install/scripts"), name, contents)
}

macro_rules! script {
    (internal $fun: path | $name: literal, $tpl: literal) => {
        $fun($name, &format!($tpl))?;
    };
    (pre $name: literal, $tpl: literal) => {
        crate::util::script!(internal crate::util::add_pre_script | $name, $tpl)
    };
    ($name: literal, $tpl: literal) => {
        crate::util::script!(internal crate::util::add_post_script | $name, $tpl)
    };
}

pub(crate) use script;

pub fn ensure_paru() -> Result<()> {
    script!(
        "00-paru",
        r#"
set -x
pacman -S --noconfirm --needed base-devel git sudo
useradd -m paruuser
passwd -l paruuser
echo "paruuser ALL=(ALL:ALL) NOPASSWD: ALL" > /etc/sudoers.d/paruuser
TMP=$(mktemp -d /tmp/tmp.XXXXXXXX)
chown paruuser:paruuser $TMP
pushd $TMP
sudo -u paruuser git clone https://aur.archlinux.org/paru-bin.git
popd
pushd $TMP/paru-bin
sudo -u paruuser makepkg -si --noconfirm
popd
"#
    );
    Ok(())
}
