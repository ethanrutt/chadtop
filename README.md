# Chadtop
![gigachad](/img/gigachad.png)
![rust](https://img.shields.io/static/v1?logo=rust&label=&message=rust&color=FFF&logoColor=000000&style=flat-square)
![arch](https://img.shields.io/static/v1?logo=archlinux&label=&message=arch&color=3776AB&logoColor=1793D1&style=flat-square)
![macos](https://img.shields.io/static/v1?logo=apple&label=&message=macos&color=FFF&logoColor=000000&style=flat-square)

> [!NOTE]
> This `README.md` is *mostly* satire

A process information tui written for the chad devs in rust (btw) using neovim
(btw) developed on arch linux (btw)

Have you ever needed to kill a process but thought that opening up a GUI
process monitor was too slow for you?

Ever thought that using `top` or `ps -ef | grep && kill` or `pkill` was too
cringe?

Ever wanted to get disk info but `df` is too hard to read? Didn't want to parse
`/proc`?

Well lucky for you I made **yet another** `top` clone!

## Images
![chadtop sysinfo](/img/chadtop_sysinfo.png)
![chadtop rice](/img/rice.png)
![chadtop keybinds](/img/keybinds.png)

## Installation
### Arch Linux Manually
- [how to install aur packages](https://wiki.archlinux.org/title/Arch_User_Repository#Installing_and_upgrading_packages)
- [package link](https://aur.archlinux.org/packages/chadtop)
### AUR Helper
i.e. yay
```bash
$ yay -S chadtop
```
### Homebrew
*coming soon*

## Motivation
- I use TUI's a lot (neovim, fzf, nmtui, and much more), so I wanted to write
one just to see what goes into it.
- I wanted to learn some stuff about the `/proc` filesystem, but ended up using
the `sysinfo` crate in rust since I want this to be able to run on `macos` as
well.
- I wanted to have fun colors and art since I will probably use this in a Linux
rice in the near future.
- I wanted it to be performant, both for me to practice optimizing and also
because I don't like slow software. While single threaded it is as efficient as
I feel like it needs to be, and you can see how the performance is by using the
`debug` flag i.e. `chadtop -d`. I am open to further performance improvements,
please give me some tips and/or submit a PR. This is my first rust project so I
am still a noob rustacean.

## Features
- see processes
- sort by various things including cpu usage %, memory, name, pid, etc.
- filter by process name and pid
- kill processes
- see system information
- gigachad art
- able to say "I use chadtop (btw)"

> [!WARNING]
> You might start growing a neckbeard if you type `chadtop` in your terminal

## References
- [ratatui.rs](https://ratatui.rs/)
- [rust sysinfo](https://docs.rs/sysinfo/latest/sysinfo/)
- [ascii art generator (specifically tmplr)](https://patorjk.com/software/taag/)
- [gigachad art](https://emojicombos.com/gigachad-ascii-art)
