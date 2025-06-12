# rust-keys

## Dev Setup
### Init
Prevent bad builds getting pushed
`cp ./pre-push .git/hooks/`

### Run
The build will check for necessary deps in build.rs
`cargo run`

### Debug
tldr;
- use built-in `debug!` macro.
- debug.log is generated
- monitor with `tail -f debug.log`

At time of writing, `debug!(msg)` is configured to log to a `debug.log` file but others (macros) could be configured via simplelog. Monitor this file in another terminal using `tail -f debug.log`. This is needed because of the nature of a TUI (ratatui) takes over the main console.