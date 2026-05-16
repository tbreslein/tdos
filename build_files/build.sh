#!/bin/bash

set -ouex pipefail

# Use a COPR Example:
#
# dnf5 -y copr enable ublue-os/staging
# dnf5 -y install package
# Disable COPRs so they don't end up enabled on the final image:
# dnf5 -y copr disable ublue-os/staging

dnf5 -y copr enable ashbuk/Hyprland-Fedora

### Install packages
#
# Packages can be installed from any enabled yum repo on the image.
# RPMfusion repos are available by default in ublue main images
# List of rpmfusion packages can be found here:
# https://mirrors.rpmfusion.org/mirrorlist?path=free/fedora/updates/43/x86_64/repoview/index.html&protocol=https&redirect=1

dnf5 install -y syncthing \
    alacritty \
    foot \
    emacs mupdf mupdf-devel \
    hyprland xdg-desktop-portal-hyprland \
    hyprlock hypridle \
    swaybg gammastep dunst rofi waybar \
    xorg-x11-server-Xwayland \
    playerctl brightnessctl

#### Example for enabling a System Unit File

systemctl enable podman.socket
