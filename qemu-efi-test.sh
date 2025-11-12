if [ ! -d efi_image/EFI/BOOT ]; then
	# Create a directory for the FAT filesystem
	mkdir -p efi_image/EFI/BOOT
fi

# Copy your EFI application to the correct location
cp target/x86_64-unknown-uefi/release/scraper_efi.efi efi_image/EFI/BOOT/BOOTX64.EFI

# Run QEMU with the directory as the FAT filesystem
qemu-system-x86_64 -m 512M -device virtio-balloon -bios /usr/share/edk2/x64/OVMF.4m.fd -drive format=raw,file=fat:rw:efi_image

