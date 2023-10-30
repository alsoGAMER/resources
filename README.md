# Resources

<a href='https://flathub.org/apps/net.nokyan.Resources'><img width='240' alt='Download on Flathub' src='https://dl.flathub.org/assets/badges/flathub-badge-en.png'/></a>

[![Please do not theme this app](https://stopthemingmy.app/badge.svg)](https://stopthemingmy.app)

Resources is a simple yet powerful monitor for your system resources and processes, written in Rust and using GTK 4 and libadwaita for its GUI. It's capable of displaying usage and details of your CPU, memory, GPUs (AMD and NVIDIA only currently), network interfaces and block devices. It's also capable of listing and terminating running graphical applications as well as processes.

<details>
  <summary><b>Click me for screenshots!</b></summary>

  ![Applications View of Resources](data/resources/screenshots/1.png?raw=true "Applications View of Resources")

  ![Applications View of Resources](data/resources/screenshots/2.png?raw=true "Processor View of Resources")

  ![Applications View of Resources](data/resources/screenshots/3.png?raw=true "GPU View of Resources")

  ![Applications View of Resources](data/resources/screenshots/4.png?raw=true "Network Interface View of Resources")
  
</details>

## Dependencies

- `glib-2.0`
- `gio-2.0`
- `gtk-4`
- `libadwaita-1`
- `systemd`
- `polkit`
- `cargo`

Other dependencies are handled by `cargo`.

## Installing

The **offical** and **only supported** way of installing Resources is using Flatpak. Simply use your graphical software manager like GNOME Software or Discover to install Resources from Flathub or type ``flatpak install flathub net.nokyan.Resources`` in your terminal.
Please keep in mind that you need to have Flathub set up on your device. You can find out how to set up Flathub [here](https://flathub.org/setup).

### Fedora

**Unofficially** packaged in [COPR](https://copr.fedorainfracloud.org/coprs/atim/resources/) for Fedora 39 and newer.

```sh
dnf copr enable atim/resources
dnf install resources
```

## Building

If you prefer to build Resources yourself, you can do so using its build system Meson.
You can either build and install Resources natively on your system like this:

```sh
meson . build --prefix=/usr/local
ninja -C build install
```

Or, even better, use the Flatpak CLI to build:

```sh
flatpak install org.gnome.Sdk//45 org.freedesktop.Sdk.Extension.rust-stable//23.08 org.gnome.Platform//45 org.freedesktop.Sdk.Extension.llvm16//23.08
flatpak-builder --user flatpak_app build-aux/net.nokyan.Resources.Devel.json
```

If you use [GNOME Builder](https://apps.gnome.org/app/org.gnome.Builder/) or Visual Studio Code with the [Flatpak extension](https://marketplace.visualstudio.com/items?itemName=bilelmoussaoui.flatpak-vscode), Resources can be built and run automatically.

## Running

Running Resources is as simple as typing `flatpak run net.nokyan.Resources` into a terminal or running it from your application launcher.
If you've built Resources natively or installed it from a traditional package manager such as `apt` or `dnf`, or if you've built Resources yourself, typing `resources` in a terminal will start Resources.
If you've built Resources as a Flatpak, type `flatpak-builder --run flatpak_app build-aux/net.nokyan.Resources.Devel.json resources` into your terminal or use one of the afforementioned IDEs to do that automatically.

## To-do

The following list is *roughly* in order of their importance with the most important item being first in the list.

- Support reading statistics of Intel GPUs
- Translations
- Battery usage and details

## Contributing

If you have an idea, bug report, question or something else, don't hesitate to [open an issue](https://github.com/nokyan/resources/issues)! Translations are always welcome.
