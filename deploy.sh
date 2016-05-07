git clone https://github.com/modulus-os/modulus-os.github.io.git
cargo doc
cd modulus-os.github.io
rm -rf doc
cp -r ../target/doc .
git config user.name "DocBot"
git config user.email "epicpotatopro@gmail.com"
git add .
git commit -m "Docs"
git push
