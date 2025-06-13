# Chadtop
A process information tui written in rust

<!-- put images here -->

## Features
- see processes
- see thorough information about a process
- sort by various things including cpu usage %, memory, name, pid, etc.
- filter by process name
- gigachad art

## TODO
- thread for process updating using channels and close with arc mutex bool
- change refresh proc in state to only sort
- fuzzy process filter by name
- beautify ui with more colors and borders
- clean up and write simple tests without `TestBackend` for now
- make readme chad af
- add pictures
- release v0.0.1
- add other stuff from sysinfo
- add signals to signals menu
- tests using test backend
- release v1.0.0

## References
- [ratatui.rs](https://ratatui.rs/)
- [ascii art generator (specifically tmplr)](https://patorjk.com/software/taag/)
