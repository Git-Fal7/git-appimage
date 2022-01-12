# git-appimage
grabs appimages from github's releases, and installs them in /opt/appimages/

# building
you should have rustup installed and fully working

```rustup default stable```

then clone the repo and build it using ``cargo``.
```
git clone https://github.com/git-fal7/git-appimage
cd git-appimage/
cargo build --release
cd target/release/
```
