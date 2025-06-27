set -e

for script in $(ls /install/scripts/*)
do
  echo -e "\033[31;1m$script\033[;m"
  bash $script
done
