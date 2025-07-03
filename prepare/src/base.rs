use color_eyre::eyre::Result;

use crate::args::Args;
use crate::util::script;

pub fn base_setup(args: &Args) -> Result<()> {
    let Args {
        timezone, locale, ..
    } = args;

    script!(
        "00-time",
        "ln -sf /usr/share/zoneinfo/{timezone} /etc/localtime\nhwclock --systohc"
    );

    script!(
        "00-locale",
        r#"
echo "{locale}.UTF-8 UTF-8" > /etc/locale.gen
locale-gen
echo LANG={locale}.UTF-8 > /etc/locale.conf"#
    );

    script!(
        "00-misc",
        "pacman -S --noconfirm less bash-completion screen tmux nano"
    );

    script!(
        "90-networkd",
        r#"
systemctl enable systemd-networkd
echo -e "[Match]\nName=en*\nName=eth*\n[Network]\nDHCP=yes" >> /etc/systemd/network/20-ethernet.network
"#
    );
    script!("90-resolved", "systemctl enable systemd-resolved");

    script!("97-mkinitcpio", "mkinitcpio -P linux");
    script!(
        "98-grub",
        r#"
pacman -S --noconfirm grub efibootmgr
grub-install --target=x86_64-efi --efi-directory=/boot
grub-mkconfig -o /boot/grub/grub.cfg
"#
    );

    script!("99-clean-cache", "yes | pacman -Scc");

    Ok(())
}
