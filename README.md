![YetAnotherBar Baner](https://i.imgur.com/jqUkGuA.png)

## Why do we need yet another status bar?
The answer is really simple... we don't,  
but I really like the idea of native status bar with CSS support so here you go anyway

# Installation
On Arch linux you can install it from AUR [yetanotherbar-git](https://aur.archlinux.org/packages/yetanotherbar-git)

#### Wayland
for Wayland you need `gtk-layer-shell` on your system, and YAB binary built with `wayland` feature flag

# Example Config
`` ~/.config/YetAnotherBar/config.ron``
```rust
Config(
    bars: {
        "bar-left": (
            monitor: "DP-1",
            pos_x: 0,
            pos_y: 1000,
            modules_left: [
                I3,
                Custom((
                    name: "custom-ram",
                    exec: ["sh","/path/to/ram.sh"],
                    interval: 1000,
                ))
            ],
            modules_right: [
                Cpu,
                Mpris,
                Alsa,
                Clock,
            ],
        ),
        "bar-right": (
            monitor: "HDMI-0",
            pos_x: 1920,
            pos_y: 1000,
            modules_left: [
                I3,
            ],
            modules_right: [
                Cpu,
                Mpris,
                Alsa,
                Clock,           
            ],
        ),
    },
)
```
`` ~/.config/YetAnotherBar/style.css`` [css](https://github.com/PolyMeilex/YetAnotherBar/blob/master/src/style.css)  
For more info about CSS selectors visit [wiki](https://github.com/PolyMeilex/YetAnotherBar/wiki)
## Result
![img](https://i.imgur.com/GJ71oye.png)

![img](https://i.imgur.com/ECgAEZj.png)
