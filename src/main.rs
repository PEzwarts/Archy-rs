// Archy-rs 1.0.0 alpha!

#[allow(unused_imports)]
use std::{fs, process::Command};

fn main() {
    println!(
        r"

               ___     ___     ___    _  _   __   __ 
              /   \   | _ \   / __|  | || |  \ \ / / 
              | - |   |   /  | (__   | __ |   \ V /  
              |_|_|   |_|_\   \___|  |_||_|   _|_|_  

              FORK ME! Customize your archy-rs program.

            ## Its mandatory to run this in an bash shell; /usr/bin/bash ##

            ## Its mandatory to partition an drive in this order; 1GB Boot, [...]GB Swap, [...]GB Root to install arch ##

            ./archy -drive=[...] -de=[...] -gr=[...] -root=[...] -user=[...] -pass=[...]

            Use -drive to install arch linux on any particular drive, such as nvme[]n[], vda[], or sda[].
            Use -de to install an Desktop enviroment onto arch linux, such as plasma, gnome, xfce4, or lxqt.
            Use -gr to install greeter session, such as sddm, lightdm, gdm(More suited for gnome), or lxdm.
            Use -root to setup an root password for installing software and/or managing the arch linux operating system.
            Use -user to setup an user account.
            Use -pass to setup the user's password.

            Features:
            ~Beginner friendly;
            Privacy in mind;
            Blackarch & chaotic repos;
            Software/Scripts pre-installed, $HOME/../..;

            TODO: Encryption AES-256, LVM, Multi-user option.
        "
    );

    let mut _semibool: bool = false;

    let mut drive: String = String::from("");
    let mut de: String = String::from("");
    let mut gr: String = String::from("");
    let mut root: String = String::from("");
    let mut user: String = String::from("");
    let mut pass: String = String::from("");

    let mut _x = 0;

    for arg in std::env::args() {
        if arg.contains("-drive=") {
            _x = 1;
        }

        if arg.contains("-de=") {
            _x = 2;
        }

        if arg.contains("-gr=") {
            _x = 3;
        }

        if arg.contains("-root=") {
            _x = 4;
        }

        if arg.contains("-user=") {
            _x = 5;
        }

        if arg.contains("-pass=") {
            _x = 6;
        }

        for _char in arg.chars() {
            match _char {
                '[' => _semibool = true,
                ']' => _semibool = false,
                _ => print!(""),
            }

            if _semibool == true && _char != '[' {
                if _x == 1 {
                    drive.extend([_char]);
                    println!("{_char}");
                }

                if _x == 2 {
                    de.extend([_char]);
                    println!("{_char}");
                }

                if _x == 3 {
                    gr.extend([_char]);
                    println!("{_char}");
                }

                if _x == 4 {
                    root.extend([_char]);
                    println!("{_char}");
                }

                if _x == 5 {
                    user.extend([_char]);
                    println!("{_char}");
                }

                if _x == 6 {
                    pass.extend([_char]);
                    println!("{_char}");
                }
            }
        }
    }

    println!("--------");
    println!("{drive}");
    println!("{de}");
    println!("{gr}");
    println!("{root}");
    println!("{user}");
    println!("{pass}");

    let _ = fs::File::create_new("./arch.sh");
    let _ = fs::write(
        "./arch.sh",
        format!(
            r####"

cfdisk /dev/{drive}

echo "## FORMATTING... ##"
echo "## MOUNTING... ##"

if echo {drive} | grep -e "nvme"; then
	mkfs.fat -F32 /dev/{drive}p1
	mkswap /dev/{drive}p2
	swapon /dev/{drive}p2
	mkfs.ext4 /dev/{drive}p3

    mount /dev/{drive}p3 /mnt
	mkdir -p /mnt/boot/efi
	mount /dev/{drive}p1 /mnt/boot/efi
fi

if echo {drive} | grep -e "vda"; then
	mkfs.fat -F32 /dev/{drive}1
	mkswap /dev/{drive}2
	swapon /dev/{drive}2
	mkfs.ext4 /dev/{drive}3

    mount /dev/{drive}3 /mnt
	mkdir -p /mnt/boot/efi
	mount /dev/{drive}1 /mnt/boot/efi
fi

if echo {drive} | grep -e "sda"; then
	mkfs.fat -F32 /dev/{drive}1
	mkswap /dev/{drive}2
	swapon /dev/{drive}2
	mkfs.ext4 /dev/{drive}3

    mount /dev/{drive}3 /mnt
	mkdir -p /mnt/boot/efi
	mount /dev/{drive}1 /mnt/boot/efi
fi

pacstrap -K /mnt base base-devel linux linux-firmware linux-headers amd-ucode intel-ucode grub efibootmgr xorg-server xorg-xinit libx11 {de} {gr} fish dhcpcd iwd networkmanager reflector kitty vim git wget curl

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

grub-install /dev/{drive}
grub-mkconfig -o /boot/grub/grub.cfg
mkinitcpio -P

echo "## CREATING USER(S) ##"

useradd -m -g users -G wheel,power,storage -s /bin/bash {user}

echo {user}:{pass} | chpasswd
echo root:{root} | chpasswd

echo "%wheel ALL=(ALL:ALL) ALL" >> /etc/sudoers
echo "Defaults rootpw" >> /etc/sudoers

echo "## DESKTOP ENVIROMENT CONFIGURATION ##"

systemctl enable {gr} NetworkManager

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
sudo chsh -s /usr/bin/fish {user}

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
sudo pacman -S --noconfirm vscodium vscodium-marketplace rustup python2 python3 neovim msf-mpc evilpdf evilclippy
# Compiling tools
sudo pacman -S --noconfirm cmake gcc clang llvm
# Bruteforcing/dictatornary tools
sudo pacman -S --noconfirm hydra medusa legba hashcat hashcat-utils cracken bopscrk
# Fuzzing tools
sudo pacman -S --noconfirm aflplusplus wfuzz
# Debugging tools
sudo pacman -S --noconfirm gdb
# Forensic tools
sudo pacman -S --noconfirm volatility3 volatility-extra virustotal
# Reverse-Engineering tools & decompilers
sudo pacman -S --noconfirm rizin rz-cutter rz-ghidra retdec

# Useful for cracking hashes

mkdir -p $HOME/Desktop/wordlist
git clone https://github.com/kkrypt0nn/wordlists $HOME/Desktop/wordlist/krp-wordlist
git clone https://github.com/kennyn510/wpa2-wordlists $HOME/Desktop/wordlist/ken-wordlist

# OSINT
sudo pacman -S --noconfirm 

# OPSEC
sudo pacman -S --noconfirm proxychains-ng torctl onionshare

# Networking
sudo pacman -S --noconfirm nmap arp-scan aircrack-ng

# Web
sudo pacman -S --noconfirm burpsuite zaproxy nikto amass evilginx

rustup default stable

# Fish shell
sudo echo /usr/bin/fish | sudo tee -a /etc/shells
sudo chsh -s /usr/bin/fish {user}

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
sudo chsh -s /usr/bin/fish {user}

" > ./dev.sh

echo "## SUCCESFULLY CONFIGURED ##"

exit

EOF

umount -a
reboot
            "####
        ),
    );
    println!("{_x}");
    if _x != 0 {
        let _ = Command::new("/usr/bin/bash").arg("./arch.sh").status();
    } else {
        println!("You're missing arguments!");
    }

    let _ = fs::remove_file("./arch.sh");
}
