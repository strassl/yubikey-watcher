# yubikey-watcher

A simple daemon for MacOS that (very sloppily) detects whether the gpg-agent is waiting for touch input from the YubiKey and posts a notification for the user.

## Concept

It works by creating a UNIX socket to which the gpg-agent logs are redirected and simply checking for log messages containing "detected card with S/N".
Once such a message is detected it posts a notification using [notify-rust](https://crates.io/crates/notify-rust).
In theory this should be Linux-compatible, however I have not tested it and I recommend you use the more stable [yubikey-touch-detector](https://github.com/maximbaz/yubikey-touch-detector).

## Disclaimer
The gpg-agent documentation explicitly states that these logs are intended for debugging and may change without further notice.
However this was the easiest way I could figure out, that did not involve calling gpg and trying to guess if it is stuck waiting for a smartcard.
At the time of this writing it is working pretty well, but use at your own risk.

If you happen to have a better idea as to how to implement it please reach out to me, I'd love to hear it!

## Build

Make sure rust and cargo are installed on your system.
Build using

```bash
cargo build --release
```

## Configuration

Pass the path of the UNIX socket to be created via the `--socket` argument e.g.:

```
yubikey-watcher --socket /tmp/yubikey-watcher.sock
```

Adjust your `~/.gnupg/gpg-agent.conf` to redirect the log files to the socket by adding:

```
log-file socket:///tmp/yubikey-watcher.sock
```

## Installation

Copy the resulting binary (located in `target/release/yubikey-watcher`) to a location of your choice.
You can either run the binary on its own or create an OS X launch agent configuration to run it in the background.
A template for the OS X launch agent can be found in `daemon/at.sigmoid.yubikey-watcher.plist`.
Adjust the paths as required for your setup, copy the launch agent plist to `~/Library/LaunchAgents` and load it via `launchctl load -w ~/Library/LaunchAgents/at.sigmoid.yubikey-watcher.plist`.
