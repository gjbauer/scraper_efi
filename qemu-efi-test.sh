LOOP_DEVICE=$(sudo losetup -f)
dd if=/dev/zero of=uefi.img bs=1M count=4096 status=progress
sudo losetup "$LOOP_DEVICE" uefi.img
sudo sgdisk -Z "$LOOP_DEVICE" && sudo sgdisk -n 0:0:0 "$LOOP_DEVICE"
sudo partprobe "$LOOP_DEVICE"
sudo mkfs.fat -F32 "$LOOP_DEVICE"p1
if [ ! -d efi_image ]; then
	# Create a directory for the FAT filesystem
	mkdir efi_image
fi
sudo mount "$LOOP_DEVICE"p1 efi_image
sudo mkdir -p efi_image/EFI/BOOT
# Copy your EFI application to the correct location
sudo cp target/x86_64-unknown-uefi/release/scraper_efi.efi efi_image/EFI/BOOT/BOOTX64.EFI
sudo umount efi_image

# Run QEMU with the directory as the FAT filesystem
qemu-system-x86_64 -m 512M -bios /usr/share/edk2/x64/OVMF.4m.fd -drive format=raw,file=uefi.img

qemu-system-x86_64 -m 2G --enable-kvm -cpu host -smp 4 -cdrom ~/Downloads/ubuntu*.iso -boot order=d -drive format=raw,file=uefi.img -net nic,model=virtio -net user -display sdl,gl=on

