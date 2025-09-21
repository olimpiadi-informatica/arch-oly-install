use color_eyre::eyre::Result;

use crate::args::Args;
use crate::util::script;

pub fn base_setup(args: &Args) -> Result<()> {
    let Args {
        timezone,
        locale,
        gnome_locale,
        server_ip,
        pixie,
        node_exporter,
        orga_account,
        additional_server_names,
        ..
    } = args;

    if *pixie {
        script!(
            "00-pixie-ping",
            r#"
pacman -S --noconfirm openbsd-netcat
cat > /etc/systemd/system/pixie-ping.timer << EOF
[Unit]
Description=Pixie ping timer

[Timer]
OnBootSec=30s
OnUnitActiveSec=30s

[Install]
WantedBy=timers.target
EOF

cat > /etc/systemd/system/pixie-ping.service << EOF
[Unit]
Description=Pixie ping

[Service]
Type=oneshot
User={orga_account}
Group={orga_account}
ExecStart=/bin/bash -c "echo 'linux' | nc -w 0 -u olympiads-server 25643"

[Install]
WantedBy=multi-user.target
EOF

systemctl enable pixie-ping.timer
"#
        );
    }

    script!(
        "00-wait-online-unit",
        r#"
cat > /etc/systemd/system/wait-hostname.service << EOF
[Unit]
Description=wait hostname
Wants=network-online.target
After=network-online.target
After=dhcpcd.service
Before=gdm.service

[Service]
Type=oneshot
ExecStart=/usr/local/bin/wait-hostname.sh
StartLimitBurst=100

[Install]
WantedBy=multi-user.target
EOF


cat > /usr/local/bin/wait-hostname.sh << EOF
#!/bin/bash
while true
do
        if [[ "\$(hostnamectl hostname)" = "archlinux" ]]
        then
            echo Waiting for hostname update
            sleep 1
        else
            break
        fi
done
EOF
chmod +x /usr/local/bin/wait-hostname.sh
systemctl enable wait-hostname.service
"#
    );

    let mut hline = "olympiads-server".to_string();
    for s in additional_server_names.iter() {
        hline = format!("{hline} {s}");
    }

    script!(
        "00-hosts",
        r#"echo -e "127.0.0.1 localhost\n{server_ip} {hline}" > /etc/hosts"#
    );

    if *node_exporter {
        script!(
            "00-node-exporter",
            r#"
pacman -S --noconfirm prometheus-node-exporter
systemctl enable prometheus-node-exporter
"#
        );
    }

    script!(
        "00-time",
        "ln -sf /usr/share/zoneinfo/{timezone} /etc/localtime\nhwclock --systohc"
    );

    let gnome_locale = gnome_locale
        .as_deref()
        .map(|s| format!(r#"echo "{s}.UTF-8 UTF-8" >> /etc/locale.gen"#))
        .unwrap_or_default();

    script!(
        "00-locale",
        r#"
echo "{locale}.UTF-8 UTF-8" > /etc/locale.gen
{gnome_locale}
locale-gen
echo LANG={locale}.UTF-8 > /etc/locale.conf"#
    );

    script!(
        "00-misc",
        "pacman -S --noconfirm less bash-completion screen tmux nano"
    );

    script!("97-mkinitcpio", "mkinitcpio -P linux");
    script!(
        "98-grub",
        r#"
pacman -S --noconfirm grub efibootmgr
grub-install --target=x86_64-efi --efi-directory=/boot
grub-mkconfig -o /boot/grub/grub.cfg
"#
    );

    script!(
        "99-networkd",
        r#"
systemctl enable systemd-networkd
echo -e "[Match]\nName=en*\nName=eth*\n[Network]\nDHCP=yes" >> /etc/systemd/network/20-ethernet.network
"#
    );
    script!(
        "99-timesyncd",
        r#"
systemctl enable systemd-timesyncd
echo -e "[Time]\nNTP=olympiads-server\nRootDistanceMaxSec=30" > /etc/systemd/timesyncd.conf
"#
    );
    script!(
        "99-resolved",
        r#"
systemctl enable systemd-resolved
umount /etc/resolv.conf
mkdir -p /etc/systemd/resolved.conf.d/
cat > /etc/systemd/resolved.conf.d/disable_stub.conf << EOF
[Resolve]
DNSStubListener=no
EOF
ln -sf /run/systemd/resolve/resolv.conf /etc/resolv.conf
sed -i "s/^hosts:.*/hosts: mymachines files myhostname dns/g" /etc/nsswitch.conf
"#
    );
    script!("99-clean-cache", "yes | pacman -Scc");

    Ok(())
}
