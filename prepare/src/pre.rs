use color_eyre::eyre::Result;

use crate::args::Args;
use crate::util::script;

pub fn pre_install_scripts(args: &Args) -> Result<()> {
    let Args {
        nowait,
        pin_packages,
        ..
    } = args;

    if !*nowait {
        script!(
            pre "00-wait",
            r#"
for i in 60 50 40 30 20 10
do
  echo -n "$i... "
  sleep 10
done"#
        );
    }

    script!(
        pre "10-partitions",
        r#"
echo -e "label: gpt\n- 500MiB U -\n- - L -" | sfdisk $DISK
"#
    );

    script!(
        pre "20-fs",
        r#"
# Wait for partitions to be online.
while ! [ -e $DISK*1 ] || ! [ -e $DISK*2 ]
do
  sleep 5
done

mkfs -t vfat $DISK*1
mkfs -F -t ext4 $DISK*2

mount $DISK*2 /mnt
mount --mkdir $DISK*1 /mnt/boot
"#
    );

    script!(
        pre "25-wait-online",
        r#"
while ! curl gstatic.com/generate_204
do
    sleep 5
done"#
    );

    if let Some(pin_packages) = pin_packages {
        script!(
            pre "30-fix-mirrors",
            r#"echo 'Server=https://archive.archlinux.org/repos/{pin_packages}/$repo/os/$arch' > /etc/pacman.d/mirrorlist"#
        );
    } else {
        script!(
            pre "30-mirrors",
            r#"reflector --score 5 --fastest 5 -c "Switzerland,Italy" --save /etc/pacman.d/mirrorlist"#
        );
    }

    script!(
        pre "40-install-base",
        r#"
pacman-key --init
pacman-key --populate
pacstrap -K /mnt base linux linux-firmware
"#
    );

    script!(
        pre "50-configure-inner",
        r#"
genfstab -U /mnt >> /mnt/etc/fstab
mount --bind --mkdir /install /mnt/install
arch-chroot /mnt bash /install/run_install_scripts.sh
umount -l /mnt/install
rm -rf /mnt/install
"#
    );
    Ok(())
}
