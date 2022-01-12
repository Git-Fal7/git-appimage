# git-appimage
grabs appimages from github's releases, and installs them in /opt/appimages/

# what it actually does

> send a request to github's api using ``curl``.
> grab everything needed from the request.
> grab an appimage from the repo releases.
> make it executable.
> copy it to ``/opt/appimages``.
> extract it using ``--appimage-extract`` in a folder called ``squashfs-root/``.
> get every``.desktop``, ``.png`` and ``.svg`` in ``squashfs-root/``.
> copy the images to ``/usr/share/pixmaps``.
> modify the original ``.desktop`` file to execute from ``/opt/appimages``.
> put the modified ``.desktop`` file to ``/usr/share/applications/`` with the appimage's name.

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

# usage
requires root permissions.

``git-appimage [name of repo]``

displays appimages in the repo's releases, it also display the numbers to install.

``git-appimage [name of repo] [numbers from what it displayed]``

installs the appimage depending on the number. (0 is the first number in the list)

# example 

to get the list

```
git-appimage srevinsaju/discord-appimage

0
Discord-0.0.16-x86_64.AppImage
stable
https://github.com/srevinsaju/discord-appimage/releases/download/stable/Discord-0.0.16-x86_64.AppImage

1
Discord-0.0.26-x86_64.AppImage
canary
https://github.com/srevinsaju/discord-appimage/releases/download/canary/Discord-0.0.26-x86_64.AppImage
```

to install one from the list

```
git-appimage srevinsaju/discord-appimage 1

0
Discord-0.0.16-x86_64.AppImage
stable
https://github.com/srevinsaju/discord-appimage/releases/download/stable/Discord-0.0.16-x86_64.AppImage

1
Discord-0.0.26-x86_64.AppImage
canary
https://github.com/srevinsaju/discord-appimage/releases/download/canary/Discord-0.0.26-x86_64.AppImage
https://github.com/srevinsaju/discord-appimage/releases/download/canary/Discord-0.0.26-x86_64.AppImage
Fi;le exists
chmoding appimage
extracted
```
