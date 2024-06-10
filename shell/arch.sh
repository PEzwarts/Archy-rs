cfdisk /dev/drive

echo "## FORMATTING... ##"
echo "## MOUNTING... ##"

if echo drive | grep -e "nvme"; then
	mkfs.fat -F32 /dev/drivep1
	mkswap /dev/drivep2
	swapon /dev/drivep2
	mkfs.ext4 /dev/drivep3

    mount /dev/drivep3 /mnt
	mkdir -p /mnt/boot/efi
	mount /dev/drivep1 /mnt/boot/efi
fi

if echo drive | grep -e "vda"; then
	mkfs.fat -F32 /dev/drive1
	mkswap /dev/drive2
	swapon /dev/drive2
	mkfs.ext4 /dev/drive3

    mount /dev/drive3 /mnt
	mkdir -p /mnt/boot/efi
	mount /dev/drive1 /mnt/boot/efi
fi

if echo drive | grep -e "sda"; then
	mkfs.fat -F32 /dev/drive1
	mkswap /dev/drive2
	swapon /dev/drive2
	mkfs.ext4 /dev/drive3

    mount /dev/drive3 /mnt
	mkdir -p /mnt/boot/efi
	mount /dev/drive1 /mnt/boot/efi
fi

pacstrap -K /mnt base base-devel linux linux-firmware linux-headers amd-ucode intel-ucode grub efibootmgr xorg-server xorg-xinit libx11 de gr fish dhcpcd iwd networkmanager reflector kitty vim git wget curl

genfstab -U -p /mnt >> /mnt/etc/fstab

arch-chroot /mnt /bin/bash << EOF

echo "## ARCH CONFIGURATION HAS STARTED ##"

echo "## TIMEZONES & MIRRORLIST ##"

reflector --sort rate -l 10 -p resync --save /etc/pacman.d/mirrorlist
ln -sf /usr/share/zoneinfo/Etc/UTC /etc/localtime
hwclock --systohc --utc

echo "## LANGUAGE & KEYMAP CONFIGURATION ##"

echo "en_US.UTF-8 UTF-8" >> /etc/locale.gen
locale-gen
echo "LANG=en_US.UTF-8" > /etc/locale.conf
echo "KEYMAP=us" > /etc/vconsole.conf

echo "## HOSTNAME CONFIGURATION ##"

echo "computer" > /etc/hostname

echo "## PACMAN AUR ##"

echo "

[multilib]
Include = /etc/pacman.d/mirrorlist
" >> /etc/pacman.conf

echo "## BOOTLOADER CONFIGURARTION ##"

grub-install /dev/drive
grub-mkconfig -o /boot/grub/grub.cfg
mkinitcpio -P

echo "## CREATING USER(S) ##"

useradd -m -g users -G wheel,power,storage -s /bin/bash user

echo user:pass | chpasswd
echo root:root | chpasswd

echo "%wheel ALL=(ALL:ALL) ALL" >> /etc/sudoers
echo "Defaults rootpw" >> /etc/sudoers

echo "## DESKTOP ENVIROMENT CONFIGURATION ##"

systemctl enable gr NetworkManager

echo "## SUCCESSFULLY CONFIGURED ##"
echo "## CUSTOM CONFIGURATION HAS STARTED ##"

touch host.sh
echo -e "

echo '## HOST CONFIG ##'

if [ -f /etc/pacman.d/chaotic-mirrorlist/ ]; then
    echo '## REPO'S ALREADY INSTALLED ##'
else
    echo '## CHAOTIC AUR ##'
    pacman-key --recv-key 3056513887B78AEB --keyserver keyserver.ubuntu.com
    pacman-key --lsign-key 3056513887B78AEB
    pacman -U 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-keyring.pkg.tar.zst'
    pacman -U 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-mirrorlist.pkg.tar.zst'
    echo '

    [chaotic-aur]
    Include = /etc/pacman.d/chaotic-mirrorlist
    ' >> /etc/pacman.conf

    echo '## BLACKARCH AUR ##'

    curl -O https://blackarch.org/strap.sh
    echo 26849980b35a42e6e192c6d9ed8c46f0d6d06047 strap.sh | sha1sum -c
    chmod +x strap.sh
    ./strap.sh
    pacman -Syy
fi

echo '## CONFIG ##'

pacman -Syu

# Basic Apps/Utils
pacman -S --noconfirm librewolf qbittorrent
# Virtualization tools
pacman -S --noconfirm qemu-full libvirt virt-manager dnsmasq bridge-utils vde2 openbsd-netcat

systemctl enable --now libvirtd
virsh net-autostart default

# Fish shell
sudo echo /usr/bin/fish | sudo tee -a /etc/shells
sudo chsh -s /usr/bin/fish user

" > ./host.sh

touch pen.sh
echo -e "

echo '## PENTEST CONFIG ##'

if [ -f /etc/pacman.d/chaotic-mirrorlist/ ]; then
    echo '## REPO'S ALREADY INSTALLED ##'
else
    echo '## CHAOTIC AUR ##'
    pacman-key --recv-key 3056513887B78AEB --keyserver keyserver.ubuntu.com
    pacman-key --lsign-key 3056513887B78AEB
    pacman -U 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-keyring.pkg.tar.zst'
    pacman -U 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-mirrorlist.pkg.tar.zst'
    echo '

    [chaotic-aur]
    Include = /etc/pacman.d/chaotic-mirrorlist
    ' >> /etc/pacman.conf

    echo '## BLACKARCH AUR ##'

    curl -O https://blackarch.org/strap.sh
    echo 26849980b35a42e6e192c6d9ed8c46f0d6d06047 strap.sh | sha1sum -c
    chmod +x strap.sh
    ./strap.sh
    pacman -Syy
fi

echo '## CONFIG ##'

pacman -Syu

# Basic Apps/Utils
sudo pacman -S --noconfirm librewolf tor-browser qbittorrent

# Software/Exploit tools & development
sudo pacman -S --noconfirm vscodium vscodium-marketplace rustup python msf-mpc
# Compiling tools
sudo pacman -S --noconfirm cmake gcc clang llvm
# Bruteforcing/dictatornary tools
sudo pacman -S --noconfirm hydra cracken
# Fuzzing tools
sudo pacman -S --noconfirm valgrind aflplusplus
# Debugging tools
sudo pacman -S --noconfirm gdb pwndbg rr
# Forensic tools
sudo pacman -S --noconfirm volatility3 volatility-extra virustotal
# Reverse-Engineering tools & decompilers
sudo pacman -S --noconfirm rizin rz-cutter rz-ghidra retdec

# OSINT
sudo pacman -S --noconfirm sherlock udork

# OPSEC
sudo pacman -S --noconfirm torctl onionshare darkdump usb-canary

# Networking
sudo pacman -S --noconfirm bettercap beef

rustup default stable

# Fish shell
sudo echo /usr/bin/fish | sudo tee -a /etc/shells
sudo chsh -s /usr/bin/fish user

" > ./pen.sh

touch dev.sh
echo -e "

echo '## DEV CONFIG ##'

if [ -f /etc/pacman.d/chaotic-mirrorlist/ ]; then
    echo '## REPO'S ALREADY INSTALLED ##'
else
    echo '## CHAOTIC AUR ##'
    pacman-key --recv-key 3056513887B78AEB --keyserver keyserver.ubuntu.com
    pacman-key --lsign-key 3056513887B78AEB
    pacman -U 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-keyring.pkg.tar.zst'
    pacman -U 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-mirrorlist.pkg.tar.zst'
    echo '

    [chaotic-aur]
    Include = /etc/pacman.d/chaotic-mirrorlist
    ' >> /etc/pacman.conf

    echo '## BLACKARCH AUR ##'

    curl -O https://blackarch.org/strap.sh
    echo 26849980b35a42e6e192c6d9ed8c46f0d6d06047 strap.sh | sha1sum -c
    chmod +x strap.sh
    ./strap.sh
    pacman -Syy
fi

echo '## CONFIG ##'

pacman -Syu

# Basic Apps/Utils
sudo pacman -S --noconfirm librewolf tor-browser qbittorrent

# Software tools & development
sudo pacman -S --noconfirm vscodium vscodium-marketplace rustup python
# Compiling tools
sudo pacman -S --noconfirm cmake gcc clang llvm

rustup default stable

# Fish shell
sudo echo /usr/bin/fish | sudo tee -a /etc/shells
sudo chsh -s /usr/bin/fish user

" > ./dev.sh

echo "## SUCCESFULLY CONFIGURED ##"

exit

EOF

umount -a
reboot