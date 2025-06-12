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
- fuzzy process filter by name
- beautify ui with more colors and borders
- clean up and write simple tests without `TestBackend` for now

## Future Work
- Have process list refresh in a separate thread for performance gains
- add other stuff from `sysinfo` like total mem, total cpu usage, etc. in
another popup
- also send signals from process info popup menu (use centered_rect from ratatui json_editor)
- ratatui tests using `TestBackend`

## References
- [ratatui.rs](https://ratatui.rs/)
- [ascii art generator (specifically tmplr)](https://patorjk.com/software/taag/)
