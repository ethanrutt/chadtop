# Chadtop
A process information tui

written for the chad devs in rust (btw) using neovim (btw) developed on arch linux (btw)

<!-- put images here -->
<!-- meme and put disclaimer that i am meming hella -->

## Features
- see processes
- see thorough information about a process
- sort by various things including cpu usage %, memory, name, pid, etc.
- filter by process name
- gigachad art

## TODO
- add helpful keybind messages
- move keybinds to (h) and appear as a popup instead of always on main screen
- add kill w shift + k
- optimize using sysinfo package i.e. only refresh what we need. constants can be refreshed once on startup
- smaller chadtop art if width/height is too
small
- see if we can use ratatui init stuff instead of
raw
- meme and beautify ui with more colors and borders
- clean up code (ui)
- write simple tests without `TestBackend` for now
- tests using test backend
- handle cmdline options for debug mode and help. look at btop for other useful cmdline options
- benchmark and improve performance where possible with debug mode
- make readme chad af
- add pictures
- release v0.0.0
- write man page
- add package files for aur
- release v0.0.1
- add package files for homebrew tap
- release v0.0.2

## References
- [ratatui.rs](https://ratatui.rs/)
- [ascii art generator (specifically tmplr)](https://patorjk.com/software/taag/)
