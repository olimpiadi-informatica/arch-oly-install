use color_eyre::eyre::{Ok, Result, bail};

use crate::args::Args;
use crate::util::{iso_root, script};

pub fn backgrounds(args: &Args) -> Result<()> {
    let Args {
        backgrounds,
        default_background,
        orga_account,
        contestant_account,
        ..
    } = args;
    if !*backgrounds {
        return Ok(());
    }
    let Some(default_background) = default_background else {
        bail!("missing default background")
    };

    let bg_install = iso_root().join("install/default_background.png");
    std::fs::copy(default_background, bg_install)?;

    script!(
        "86-backgrounds",
        r#"
pacman -S --noconfirm wget
mkdir -p /opt/background
chmod ugo+rwX /opt/background
chown {orga_account}:{orga_account} /opt/background
cp -v /install/default_background.png /opt/background

cat > /etc/systemd/system/get-background.service << EOF
[Unit]
Description=get background
Requires=wait-hostname.service
After=wait-hostname.service
Before=gdm.service

[Service]
User={orga_account}
Group={orga_account}
Type=oneshot
WorkingDirectory=/opt/background
ExecStart=/usr/local/bin/get_background.sh

[Install]
WantedBy=multi-user.target
EOF

cat > /usr/local/bin/get_background.sh << EOF
#!/bin/bash
cp -v default_background.png background.png
wget http://backgrounds.olympiads-server/\$(hostnamectl hostname)/background.png -O correct_background.png
mv correct_background.png background.png
EOF

chmod +x /usr/local/bin/get_background.sh

sudo -u {contestant_account} -g {contestant_account} dbus-launch bash << EOF
gsettings set org.gnome.desktop.background picture-uri 'file:///opt/background/background.png'
gsettings set org.gnome.desktop.background picture-uri-dark 'file:///opt/background/background.png'
EOF

systemctl enable get-background.service
"#
    );

    Ok(())
}

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
sudo -u {contestant_account} -g {contestant_account} dbus-launch bash << EOF
set -xe

mkdir -p ~{contestant_account}/Desktop

# No dynamic workspaces
gsettings set org.gnome.mutter dynamic-workspaces false
gsettings set org.gnome.desktop.wm.preferences num-workspaces 4

# Power settings
gsettings set org.gnome.desktop.session idle-delay 0
gsettings set org.gnome.desktop.lockdown disable-lock-screen true
gsettings set org.gnome.settings-daemon.plugins.power sleep-inactive-ac-type nothing
gsettings set org.gnome.settings-daemon.plugins.power sleep-inactive-battery-type nothing
gsettings set org.gnome.settings-daemon.plugins.power power-button-action nothing

# Enable desktop icons
gnome-extensions enable ding@rastersoft.com

# ctrl-alt-t -> open terminal
dconf write /org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/command "'gnome-terminal'"
dconf write /org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/binding "'<Primary><Alt>t'"
dconf write /org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/name "'gnome-terminal'"
gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings "['/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/']"

# Misc
gsettings set org.gnome.desktop.interface clock-format 24h
gsettings set org.gnome.desktop.wm.preferences button-layout appmenu:minimize,maximize,close
gsettings set org.gnome.desktop.wm.preferences audible-bell false
EOF
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
