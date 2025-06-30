use color_eyre::eyre::Result;

use crate::args::Args;
use crate::util::script;

pub fn base_gui_setup(args: &Args) -> Result<()> {
    let Args {
        contestant_account,
        virtualbox,
        disable_wayland,
        ..
    } = args;

    let gdm_wayland = if *disable_wayland {
        "\nWaylandEnable=false"
    } else {
        ""
    };

    script!(
        "85-gnome",
        r#"
pacman -S --noconfirm gnome gnome-extra gdm
pacman -R --noconfirm gnome-tour
systemctl enable gdm
echo -e "[daemon]\nAutomaticLoginEnable=True\nAutomaticLogin={contestant_account}{gdm_wayland}" > /etc/gdm/custom.conf
"#
    );

    if *disable_wayland {
        script!("85-xorg", "pacman -S --noconfirm xorg-server");
    }

    if *virtualbox {
        script!(
            "85-virtualbox",
            "pacman -S --noconfirm virtualbox virtualbox-host-modules-arch"
        );
    }

    Ok(())
}
