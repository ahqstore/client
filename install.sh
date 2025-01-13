#!/usr/bin/bash
tag="%%tag%%"
rpm="%%rpm%%"
deb="%%deb%%"
base="https://github.com/ahqsoftwares/tauri-ahq-store/releases/download/$tag"
arch=$(uname -m)

if [ "$arch" = "x86_64" ]; then
  echo "Downloading AHQ Store for x86_64 architecture"
elif [ "$arch" = "aarch64" ]; then
  echo "AHQ Store is not supported for arm64 architecture"
else
  echo "Unsupported architecture $arch"
fi

echo "Please wait while we download the required packages..."

service="$base/ahqstore_setup_linux_amd64"

PS3='Which package manager shall we use?: '
options=("apt" "rpm")

select opt in "${options[@]}"
do
  case $opt in
    "apt")
      echo "Installing flatpak"
      sudo add-apt-repository ppa:flatpak/stable
      sudo apt update
      sudo apt install flatpak

      download="$tag/$deb"
      break
      ;;
    "rpm")
      echo "Please enter the command to install flatpak"

      read command

      sudo $command

      download="$tag/$rpm"

      break
      ;;
    *) echo "invalid option $REPLY";;
  esac
done

echo "Installing FlatHub repository"
flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo