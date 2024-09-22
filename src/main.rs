// Archy-rs 1.3.0

use std::{env::args, fs, process::{exit, Command}};
use text_io::read;

fn main() {
    let mut _args_t: Vec<String> = args().collect();

    for arg in _args_t.iter() {
        if arg.contains("-h") {
            println!(
                r"
               ___     ___     ___    _  _   __   __ 
              /   \   | _ \   / __|  | || |  \ \ / / 
              | - |   |   /  | (__   | __ |   \ V /  
              |_|_|   |_|_\   \___|  |_||_|   _|_|_  

              FORK ME! Customize your archy-rs program

            ## Its mandatory to partition an drive in this order; 1GB Boot, [...]GB Swap, [...]GB Root to install arch ##

            drive=[...]
            root_passwd=[...]
            user=[...]
            user_passwd=[...]
            desktop_enviroment=[...]
            greeter=[...]

            Use drive to install Arch linux on any particular drive, such as nvme[]n[], vda[], or sda[]
            Use root_passwd to setup an root password
            Use username to setup an user account
            Use user_passwd to setup the user's password
            Use desktop_enviroment to install an Desktop enviroment, such as plasma, gnome, xfce4
            Use greeter to install greeter session, such as sddm, lightdm, gdm

            Features:
            ~Beginner friendly;
            Privacy in mind;
            Multilib, Blackarch & Chaotic repos;
            Scripts pre-installed, $HOME/../..;

        "
            );
            exit(0)
        }
    }

    print!("DRIVE: ");
    let drive: String = read!();
    print!("ROOT_PASSWD: ");
    let root: String = read!();
    print!("USER: ");
    let user: String = read!();
    print!("USER_PASSWD: ");
    let pass: String = read!();
    print!("DESKTOP_ENVIROMENT: ");
    let de: String = read!();
    print!("GREETER: ");
    let gr: String = read!();

    let chaotic = String::from(format!(r####"
echo '## CHAOTIC AUR ##'
pacman-key --recv-key 3056513887B78AEB --keyserver keyserver.ubuntu.com
pacman-key --lsign-key 3056513887B78AEB
pacman -U 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-keyring.pkg.tar.zst'
pacman -U 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-mirrorlist.pkg.tar.zst'
echo '

[chaotic-aur]
Include = /etc/pacman.d/chaotic-mirrorlist
' >> /etc/pacman.conf
    "####));

    let blackarch = String::from(format!(r####"
echo '## BLACKARCH AUR ##'

curl -O https://blackarch.org/strap.sh
echo 26849990b35a42e6e192c6d9ed8c46f0d6d06047 strap.sh | sha1sum -c
chmod u+x strap.sh
./strap.sh
    "####));

    let dev = String::from(format!(r####"
echo '## DEV CONFIG ##'

if [ -f /etc/pacman.d/chaotic-mirrorlist/ ]; then
    echo '## CHAOTIC REPO INSTALLED ##'
else
    {chaotic}
fi

if [ -f /etc/pacman.d/blackarch-mirrorlist ]; then
    echo '## BLACKARCH REPO INSTALLED ##'
else
    {blackarch}
fi

pacman -Syyu

# Basic Apps/Utils
sudo pacman -S --noconfirm librewolf tor-browser qbittorrent

# Software tools & development
sudo pacman -S --noconfirm vscodium vscodium-marketplace neovim rustup python go
# Compiling tools
sudo pacman -S --noconfirm cmake ninja gcc clang llvm

rustup install stable

# Fish shell
sudo echo /usr/bin/fish | sudo tee -a /etc/shells
sudo chsh -s /usr/bin/fish {user}
    "####));

    let min = String::from(format!(r####"
echo '## MINIMAL CONFIG ##'

if [ -f /etc/pacman.d/chaotic-mirrorlist/ ]; then
    echo '## CHAOTIC REPO INSTALLED ##'
else
    {chaotic}
fi

if [ -f /etc/pacman.d/blackarch-mirrorlist ]; then
    echo '## BLACKARCH REPO INSTALLED ##'
else
    {blackarch}
fi

pacman -Syyu

# Basic Apps/Utils
sudo pacman -S --noconfirm librewolf tor-browser qbittorrent

# Fish shell
sudo echo /usr/bin/fish | sudo tee -a /etc/shells
sudo chsh -s /usr/bin/fish {user}
    "####));

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

pacstrap -K /mnt base base-devel linux linux-firmware linux-headers amd-ucode intel-ucode grub efibootmgr xorg-server xorg-xinit libx11 {de} {gr} fish dhcpcd iwd networkmanager reflector vim git wget curl

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

echo "## CREATING USER ##"

useradd -m -g users -G wheel,power,storage -s /bin/bash {user}

echo {user}:{pass} | chpasswd
echo root:{root} | chpasswd

echo "%wheel ALL=(ALL:ALL) ALL" >> /etc/sudoers
echo "Defaults rootpw" >> /etc/sudoers

echo "## DESKTOP ENVIROMENT CONFIGURATION ##"

systemctl enable {gr} NetworkManager

echo "## SUCCESSFULLY CONFIGURED ##"
echo "## CUSTOM CONFIGURATION HAS STARTED ##"

touch dev.sh
echo -e "
{dev}
" > ./dev.sh

touch min.sh
echo -e "
{min}
" > ./min.sh

echo "## SUCCESFULLY CONFIGURED ##"

exit

EOF

umount -a
reboot
            "####
        ),
    );
    Command::new("/usr/bin/bash")
        .arg("./arch.sh")
        .status()
        .unwrap();

    fs::remove_file("./arch.sh").unwrap();
}
