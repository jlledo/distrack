# distrack

macOS daemon for removing URL trackers on copy, inspired by [TrackerZapper](https://github.com/rknightuk/TrackerZapper).

## Caveats

This daemon will remove the `source` query parameter from URLs, since it is used by [Medium](https://medium.com).

This might impact legitimate URLs, so keep it in mind.

## Installation

1. `cargo build`
2. `cp ./target/debug/distrack <your-binary-dir>`
3. Setup run-at-login with launchd or similar, see this [StackOverflow answer](https://stackoverflow.com/a/22872222)
