mkdir -p iso/boot/grub
cp ../scripts/grub.cfg iso/boot/grub
cp modulus iso/boot
cp -r ../fs/* iso
grub-mkrescue -o modulus.iso iso --directory /usr/lib/grub/i386-pc

#