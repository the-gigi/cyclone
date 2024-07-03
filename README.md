# Cyclone

Cyclone🌀 is a little Rust program that saves you time
when you clone a git repository via the console.

# Rationale

To clone a repository from GitHub or GitLab you do the following:

1. Copy the repository's URL to your clipboard
2. Go to your console
3. Type "git clone "
4. Paste the URL from your clipboard

WHAT IF YOU DIDN'T HAVE TO TYPE "git clone " EVER AGAIN ?!?!?! 🤯

Can you even start to imagine the improvement in your quality of life? ✨

I know, it sounds too good to be true, but trust me, it is possible. According to ChatGPT millions
of repositories are cloned every day on GitHub, and typing "git clone " takes 2-3 seconds on
average.

This means if somehow typing "git clone " could be avoided then we as an industry
could save millions of seconds every single day!

This is where Cyclone🌀 comes in. Cyclone🌀 watches your clipboard. Whenever it sees that clipboard
contains a string that has the prefix "git@" (which is common to all GitHub and Gitlab SSH URIs) it
prepends "git clone " to it and writes it back
to the clipboard.

When you paste your clipboard contents to the console, the "git clone " will already
be there, and you never need to type it!

Namaste 🙏

# Usage

You can just build and run Cyclone yourself on any operating system:

```
$ cargo b -r
$ ./target/release/cyclone
```

## The macOS White-Glove Service

If you want the white-glove service on macOS you should run Cyclone🌀 automatically whenever you
start your
computer, so it's always there ready to shave those seconds from your git clones...

First, install Cyclone (it will go to ~/.cargo/bin)

```
$ cargo install --path .
  Installing cyclone v0.1.0 (/Users/gigi.sayfan/git/cyclone)
    Updating crates.io index
    Finished release [optimized] target(s) in 1.53s
  Installing /Users/gigi.sayfan/.cargo/bin/cyclone
   Installed package `cyclone v0.1.0 (/Users/gigi.sayfan/git/cyclone)` (executable `cyclone`)
```

Then, use Launchd to launch it automatically on reboot by running the following commands:

```
$ cat local.cyclone.plist | sed s/{whoami}/$(whoami)/g > $TMPDIR/local.cyclone.plist
$ sudo cp $TMPDIR/cyclone.plist ~/Library/LaunchAgents
```

From now on, Cyclone🌀 will monitor your clipboard even if you reboot.

For the first time (if you don't want to reboot) you can launch it directly:

```
$ launchctl load ~/Library/LaunchAgents/local.cyclone.plist
```

To uninstall Cyclone🌀 run the following command:

```
$ launchctl unload ~/Library/LaunchAgents/local.cyclone.plist
$ rm ~/Library/LaunchAgenets/local.cyclone.plist
```

Enjoy! 🥳 
