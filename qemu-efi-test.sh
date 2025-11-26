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
riscv64-linux-gnu-objcopy -O efi-app-riscv64 target/riscv64gc-unknown-none-elf/release/scraper_efi BOOTRISCV64.EFI --subsystem=10
sudo cp BOOTRISCV64.EFI efi_image/EFI/BOOT/BOOTRISCV64.EFI
sudo umount efi_image

# Run QEMU with the directory as the FAT filesystem
qemu-system-riscv64 \
 -M virt,pflash0=pflash0,pflash1=pflash1,acpi=off \
 -m 4096 -smp 2 \
 -serial mon:stdio \
 -device virtio-gpu-pci \
 -device qemu-xhci \
 -device usb-kbd \
 -device virtio-rng-pci \
 -blockdev node-name=pflash0,driver=file,read-only=on,filename=RISCV_VIRT_CODE.fd \
 -blockdev node-name=pflash1,driver=file,filename=RISCV_VIRT_VARS.fd \
 -netdev user,id=net0 \
 -device virtio-net-pci,netdev=net0 \
 -device virtio-blk-device,drive=hd0 \
 -drive file=uefi.img,format=raw,id=hd0,if=none


