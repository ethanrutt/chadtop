# Chadtop
This is yet another `top` clone. The purpose of this project is to learn more
about the linux operating system and learn the rust programming language, so
I'm fine with solving a problem that's been solved before.

Change of focus. Originally this was going to be a `top` clone, but since I
don't have certain hardware and won't be able to handle some edge cases, I
think instead i'm going to just make an in depth process inspector.

Scrapping hwinfo

## TODO
- fix calculations, they are tripping rn
- fix spacing for new table
- fix bug on pane split
- improve performance of parsing processes
- sending signals menu
- user instead of just uid
- better cpu %
- fuzzy filter
- tree would be cool


## Known Issues
- time is just a string so sorting is not correct
- single threaded, so tui will redraw whenever a key is pressed or every
second, whichever comes first. This means that whenever scrolling up or down
the list will update on every key press, and when nothing is pressed it will
update every second. It would be nicer to have a separate thread that updated
the list and redraw everything else on key presses so that everything updates
every second or other set time period instead of both
- better info for processes such as how much memory is used, better cpu instead
of just raw "time" which is kind of ambiguous, maybe use `sysinfo` crate or do
more in depth parsing from `/proc`
- when splitting pane in tmux it panics and fails on `proc.rs:23`
