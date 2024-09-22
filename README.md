
## Archy-rs is an blazingly 🔥🚀 fast rust 🦀 based arch install script.

# Features:
    
    ~Beginner friendly;

    Privacy in mind;

    Blackarch & chaotic repos;

    Software/Scripts pre-installed, such as ./dev.sh ./pen.sh ./host.sh


# TODO:

    Encryption AES256, LVM, Multi-user option.

# Usage:

    git clone https://github.com/PEzwarts/Archy-rs
    cd ./Archy-rs
    RUSTFLAGS='-C target-feature=+crt-static' cargo build --release

    1. Put the ELF binary executable inside of an USB stick of your choice
    2. Boot into the Arch linux enviroment
    3. Create an folder, such as ./mnt/
    4. Mount the USB stick onto the folder
    5. Run ./Archy-rs ELF binary executable inside of your USB stick
