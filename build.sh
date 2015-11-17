set -x
set -u
set -e

if [ -d "rust" ]; then
    echo "Rust has already been cloned, updating";
    cd rust
    git pull
    cd -
else 
    git clone --single-branch --depth 500 https://github.com/rust-lang/rust
fi

hash=`rustc -v -V | grep commit-hash | sed 's/^commit-hash: //'`

cd rust
git checkout $hash
cd -

rm -f libcore
rm -f liballoc

ln -s rust/src/libcore libcore
ln -s rust/src/liballoc liballoc
cp Cargo.toml.core libcore/Cargo.toml
cp Cargo.toml.alloc liballoc/Cargo.toml
cp Cargo.toml.alloc_system liballoc_system/Cargo.toml
