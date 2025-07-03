set -e

for script in $(ls /install/scripts/*)
do
  echo -e "\033[31;1m$script\033[;m"
  bash $script || (echo -e "\033[31;1mInstall script in chroot failed!\033[;m" && bash && exit 1)
done
