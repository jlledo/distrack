# distrack
macOS daemon for removing URL trackers on copy, inspired by [TrackerZapper](https://github.com/rknightuk/TrackerZapper).

## Caveats
This daemon will remove the `source` query parameter from URLs, since it is used by [Medium](https://medium.com).

This might impact legitimate URLs, so keep it in mind.
