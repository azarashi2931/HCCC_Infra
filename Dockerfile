FROM archlinux

RUN pacman-key --init
RUN pacman -Sy --noconfirm archlinux-keyring
RUN pacman -Syyu --noconfirm base-devel neovim vim emacs



