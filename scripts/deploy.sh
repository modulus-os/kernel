git clone https://github.com/modulus-os/modulus-os.github.io.git
cargo doc
cd modulus-os.github.io
rm -rf doc
cp -r ../target/doc .
git config user.name "DocBot"
git config user.email "epicpotatopro@gmail.com"
git add .
git commit -m "Docs"

cp ~/.ssh/config ~/.ssh/config_old
rm ~/.ssh/config
echo "Host github.com\n IdentitiesOnly yes\n IdentityFile /home/ubuntu/.ssh/id_voxl" | cat > ~/.ssh/config
git push
rm ~/.ssh/config
mv ~/.ssh/config_old ~/.ssh/config