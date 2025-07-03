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

    script!(
        "84-lid",
        r#"
mkdir -p /etc/systemd/logind.conf.d/
echo -e "[Login]\nHandleLidSwitch=ignore\nHandlePowerKey=ignore" >> /etc/systemd/logind.conf.d/do_not_suspend.conf
"#
    );

    let gdm_wayland = if *disable_wayland {
        "\nWaylandEnable=false"
    } else {
        ""
    };

    script!(
        "85-gnome",
        r#"
pacman -S --noconfirm gnome gdm gnome-terminal gnome-shell-extension-desktop-icons-ng
pacman -R --noconfirm gnome-tour
systemctl enable gdm
echo -e "[daemon]\nAutomaticLoginEnable=True\nAutomaticLogin={contestant_account}{gdm_wayland}" > /etc/gdm/custom.conf
sudo -u {contestant_account} mkdir -p ~{contestant_account}/Desktop
sudo -u {contestant_account} -g {contestant_account} dbus-launch gsettings set org.gnome.settings-daemon.plugins.power sleep-inactive-ac-type nothing
sudo -u {contestant_account} -g {contestant_account} dbus-launch gsettings set org.gnome.settings-daemon.plugins.power sleep-inactive-battery-type nothing
sudo -u {contestant_account} -g {contestant_account} dbus-launch gsettings set org.gnome.settings-daemon.plugins.power power-button-action nothing
sudo -u {contestant_account} -g {contestant_account} dbus-launch gsettings set org.gnome.desktop.lockdown disable-lock-screen true
sudo -u {contestant_account} -g {contestant_account} dbus-launch gsettings set org.gnome.desktop.interface clock-format 24h
sudo -u {contestant_account} -g {contestant_account} dbus-launch gsettings set org.gnome.desktop.wm.preferences button-layout appmenu:minimize,maximize,close
sudo -u {contestant_account} -g {contestant_account} dbus-launch gsettings set org.gnome.desktop.wm.preferences audible-bell false
sudo -u {contestant_account} -g {contestant_account} dbus-launch gnome-extensions enable ding@rastersoft.com
"#
    );

    script!(
        "87-disable-keyring",
        r#"
chmod 0700 /usr/bin/gnome-keyring-daemon
mkdir -p ~{contestant_account}/.config/autostart
echo -e "[Desktop Entry]\nHidden=true" >> ~{contestant_account}/.config/autostart/gnome-keyring-pkcs11.desktop
echo -e "[Desktop Entry]\nHidden=true" >> ~{contestant_account}/.config/autostart/gnome-keyring-secrets.desktop
echo -e "[Desktop Entry]\nHidden=true" >> ~{contestant_account}/.config/autostart/gnome-keyring-ssh.desktop
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

    script!(
        "89-desktop-files",
        r#"
for dsk in ~{contestant_account}/Desktop/*.desktop
do
    sudo -u {contestant_account} -g {contestant_account} dbus-launch gio set $dsk metadata::trusted true
    chmod +x $dsk
done
sudo -u {contestant_account} -g {contestant_account} dbus-launch gsettings set org.gnome.shell favorite-apps \
    "[$((cd ~{contestant_account}/Desktop; echo org.gnome.Nautilus.desktop; ls *.desktop) | sed "s/^/'/g;s/$/'/g" | paste -sd,)]"
"#
    );

    Ok(())
}
