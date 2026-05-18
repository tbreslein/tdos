#!/bin/bash

set -ouex pipefail

# Use a COPR Example:
#
# dnf5 -y copr enable ublue-os/staging
# dnf5 -y install package
# Disable COPRs so they don't end up enabled on the final image:
# dnf5 -y copr disable ublue-os/staging

dnf5 -y copr enable ashbuk/Hyprland-Fedora

# librewolf
dnf5 -y config-manager addrepo --from-repofile=https://repo.librewolf.net/librewolf.repo
dnf5 -y install librewolf

# brave browser
dnf5 -y install dnf-plugins-core
dnf5 -y config-manager addrepo --from-repofile=https://brave-browser-rpm-release.s3.brave.com/brave-browser.repo
dnf5 -y install brave-browser

### Install packages
#
# Packages can be installed from any enabled yum repo on the image.
# RPMfusion repos are available by default in ublue main images
# List of rpmfusion packages can be found here:
# https://mirrors.rpmfusion.org/mirrorlist?path=free/fedora/updates/43/x86_64/repoview/index.html&protocol=https&redirect=1

# desktop
dnf5 install -y syncthing \
    alacritty \
    emacs mupdf mupdf-devel tree-sitter-cli \
    hyprland xdg-desktop-portal-hyprland \
    hyprlock hypridle \
    swaybg gammastep dunst rofi waybar \
    xorg-x11-server-Xwayland \
    wl-clipboard \
    flameshot grim slurp \
    imv mpv vlc \
    playerctl brightnessctl

# common (i don't want to rely on homebrew to have these)
dnf5 -y install vim git \
    bash-completion

# coding
dnf5 -y install rustup \
    zig \
    golang gopls \
    cmake ninja-build bear clang clang-tools-extra \
    nodejs

# emacs build deps
dnf5 -y group install c-development --with-optional
dnf5 -y group install development-tools --with-optional
dnf5 -y builddep emacs
dnf5 -y install libgccjit libgccjit-devel \
    gtk3 gtk3-devel gtk4 gtk4-devel \
    libtree-sitter libtree-sitter-devel \
    jansson-devel libvterm-devel \
    webkit2gtk4.1-devel webkitgtk6.0-devel gnutls-devel

#### Example for enabling a System Unit File

systemctl enable podman.socket
