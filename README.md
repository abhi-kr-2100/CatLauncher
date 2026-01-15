# CatLauncher

An opinionated cross-platform launcher for Cataclysm games with modern social features.

## Screenshots

![Main Screen](/screenshots/main-screen.png?raw=true "Main Screen")

## Features

- [x] Supports Dark Days Ahead, Bright Nights, and The Last Generation.
- [x] Works on Windows, macOS, and Linux.
- [x] Supports third-party mods, soundpacks, and tilesets.
- [x] Automatic migration of saves and configs.
- [x] Backup and restore saves.
- [x] Track play time.
- [ ] Share saves with friends and strangers online.

## Installation

[Download](https://github.com/abhi-kr-2100/CatLauncher/releases/latest) the appropriate file for your operating system:

- Windows: .exe
- macOS: .dmg
- Linux: .deb, .rpm, .AppImage

### Windows

Windows may warn about downloading unidentified .exe files. To download, click on the three-dots menu and select "Keep". When you run the .exe file, you'll receive another blocking screen. To proceed with the installation, you must click on "More info" and then select "Run anyway".

CatLauncher is 100% open-source and the .exe is built directly from the source code via GitHub Actions. This ensures there is no opportunity for malicious actors to tamper with the executable. CatLauncher is safe to use.

### macOS

By default, macOS does not allow running applications from unidentified developers. To allow CatLauncher, open System Preferences > Security & Privacy > General and click on the lock icon to make changes. Then, click on the "Open Anyway" button next to CatLauncher. You should do this after trying to run the .dmg file and failing.

Ensure that you download the correct .dmg file depending on your macOS architecture. You can check your macOS architecture by running the command `uname -m` in the terminal. If the output is `x86_64`, download the `x64.dmg` file. If the output is `arm64`, download the `aarch64.dmg` file.

Another way to accomplish this is by disabling Gatekeeper. See https://disable-gatekeeper.github.io/.

CatLauncher is 100% open-source and the .dmg is built directly from the source code via GitHub Actions. This ensures there is no opportunity for malicious actors to tamper with the executable. CatLauncher is safe to use.

### Linux

Downloading the AppImage is the easiest way to install CatLauncher and keep it automatically up-to-date. However, the AppImage may not work on all Linux distros. It's been tested to work on Ubuntu 24.04.

To use the AppImage, download it, and make it executable by running `chmod a+x cat-launcher_0.14.0_amd64.AppImage`, and then run it like you would any other executable.

If you encounter issues with the AppImage, you can try installing CatLauncher using the .deb or the .rpm package. It would also help if you can open an issue on GitHub with details of your distro.

## License

CatLauncher © 2025 by Abhishek Kumar is licensed under CC BY-NC-ND 4.0. To view a copy of this license, visit https://creativecommons.org/licenses/by-nc-nd/4.0/

<div align="center">
  <img src="cat-launcher/src-tauri/icons/icon.png?raw=true" width="256" alt="CatLauncher icon">
</div>
