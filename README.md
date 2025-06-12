# Chadtop
This is yet another `top` clone. The purpose of this project is to learn more
about the linux operating system and learn the rust programming language, so
I'm fine with solving a problem that's been solved before.

Change of focus. Originally this was going to be a `top` clone, but since I
don't have certain hardware and won't be able to handle some edge cases, I
think instead i'm going to just make an in depth process inspector.

Scrapping hwinfo

## TODO
- switch back to sysinfo so we can also work on macos
- table should be pid, name, mem, cpy usage, uid, ppid
- sending signals menu
- on signal menu also include start time, run time, disk usage, open files, open files limit, cwd, exe
- fix spacing for new table
- user instead of just uid
- fuzzy filter
- keep simple, single threaded, just get this out so we can do js to get familiar w it before work
