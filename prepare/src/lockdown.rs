use color_eyre::eyre::Result;

use crate::{args::Args, util::script};

pub fn lockdown(args: &Args) -> Result<()> {
    let Args {
        no_lockdown,
        orga_net,
        ..
    } = args;
    if *no_lockdown {
        return Ok(());
    }
    script!(
        "80-lockdown",
        r#"
pacman -S --noconfirm nftables
cat > /etc/systemd/system/lockdown.service << EOF
[Unit]
Description=lockdown
Requires=wait-hostname.service
After=wait-hostname.service
Before=gdm.service

[Service]
Type=oneshot
RemainAfterExit=true
ExecStart=/usr/local/bin/lockdown.sh
ExecStop=/usr/local/bin/unlock.sh

[Install]
WantedBy=multi-user.target
EOF

cat > /usr/local/bin/lockdown.sh << EOF
#!/bin/bash

nft -f - << NFT
destroy table inet lockdown

define server_ip = 10.0.0.1
define orga_net = {orga_net}

table inet lockdown {{
  chain input {{
    type filter hook input priority filter
    policy drop

    iif lo accept

    ip saddr {{ \\\$server_ip, \\\$orga_net }} accept

    counter reject with icmpx admin-prohibited
  }}

  chain output {{
    type filter hook output priority filter
    policy drop

    oif lo accept

    ip daddr {{ \\\$server_ip, \\\$orga_net }} ct state {{ related, established }} accept

    ip daddr \\\$server_ip meta l4proto . th dport {{
      tcp . 80,
      tcp . 443,
      udp . 53,
      udp . 123,
      udp . 67,
      udp . 68,
      udp . 25643
    }} accept

    counter reject with icmpx admin-prohibited
  }}
}}
NFT

# USB
KER=\$(uname -r)
rmmod uas
rmmod usb_storage
mv /lib/modules/\$KER/kernel/drivers/usb/storage/uas.ko.zst{{,.blacklist}}
mv /lib/modules/\$KER/kernel/drivers/usb/storage/usb-storage.ko.zst{{,.blacklist}}
EOF

cat > /usr/local/bin/unlock.sh << EOF
#!/bin/bash

nft destroy table inet lockdown

# USB
KER=\$(uname -r)
udevadm control --reload-rules
udevadm trigger
mv /lib/modules/\$KER/kernel/drivers/usb/storage/uas.ko.zst{{.blacklist,}}
mv /lib/modules/\$KER/kernel/drivers/usb/storage/usb-storage.ko.zst{{.blacklist,}}
modprobe uas
modprobe usb_storage
EOF

chmod +x /usr/local/bin/lockdown.sh /usr/local/bin/unlock.sh

systemctl enable lockdown
"#
    );
    Ok(())
}
