![YetAnotherBar Baner](https://i.imgur.com/jqUkGuA.png)
YetAnotherBar is simple gtk based statusbar for i3 with support for CSS

## Why do we need yet another status bar?
The answer is really simple... we don't,  
but I really like the idea of native status bar with CSS support so here you go anyway

# Example Config
`` ~/.config/YetAnotherBar/config.toml``
```toml
[[bars]]
name = "bar-left"
monitor = "DP-1"
pos_x = 0
pos_y = 1000

modules_left = ["I3"]
modules_right = ["Cpu","Mpris","Alsa","Clock"]

[[bars]]
name = "bar-right"
monitor = "HDMI-0"
pos_x = 1920
pos_y = 1000

modules_left = ["I3"]
modules_right = ["Cpu","Mpris","Alsa","Clock"]
```
`` ~/.config/YetAnotherBar/style.css`` [css](https://github.com/PolyMeilex/YetAnotherBar/blob/master/src/style.css)
## Result
![img](https://i.imgur.com/GJ71oye.png)
