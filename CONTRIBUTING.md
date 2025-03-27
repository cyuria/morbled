# Contributing

Contributors are very welcome, morbled is in a very early state and needs work
done.

## Philosophy

### QoL changes

Quality of life features is the entire reason this project even exists, and
they are the most important part of it. Even the smallest qol fixes are
important, so please don't hesitate to open issues, PRs or whatever else in
order to fix these.

### Features

Useful features are always welcome, but please try to avoid scope creep. This
project is first and foremost backlight/brightness control program, so please
don't give it discord rich presence.

### Dependencies

Some effort has been put into the project to reduce dependencies. If you bring
in a new dependency, please make sure you aren't bringing in random other
projects. For example, we don't need json support. It's not something that's
inherently required for writing a single number to a file or sending a dbus
message, so if you think you need json support, think again.

### Configuration

Morbled doesn't really need a configuration file. At best you could argue that
the brightness transitions should be configurable, but that's the kind of thing
where there really is a one size fits all solution, and we should be using
whatever that is. If someone really, really wants to change something like
that, an environment variable or command line flag would do just fine and not
draw in a dependency on whatever file format parsing crate ends up being used.

### What to do

So far this document has just been "don't do this" and "don't do that". Here
are a few examples of what you *should* be doing.

Reducing dependencies. Provided there are not qol impacts, this is a great PR.
Reducing dependencies reduces compile times, and those are one of the biggest
issues with rust at the moment.

Improving or adding qol features. This could be a fix for some specific use
case to *just work ™*, or it could be a nice quality of life feature. Morbled
aims to have the best out of the box experience for backlight control, meaning
someone can just install it, change two lines of a window manager config, and
forget about it.

Lastly, changing and fixing up documentation is always appreciated.
Morbled desperately needs better man pages.

Also a better command line interface would be pretty nice.

## Decisions

A list of current design decisions and their justifications.

### Daemon Process

A daemonised process is used primarily because it returns control to the user
immediatelyinstead of waiting for the brightness to change fully before control
is returned.

Another important effect is that multiple changes happening within a short
enough time frame of each other will potentially conflict with each other
instead of being additive. Using a client/server model resolves this issue.

### Async

A large part of the daemon codebase is async. This is to ensure there are no
weird shenanigans while the brightness is being modified. If a user were to
spam brightness changes ten times a second, morbled should handle all of those
just as smoothly as if they were one bigger change.

### Systemd Dependency

Adding systemd as a dependency is a pretty big decision, however it enables
morbled to set brightness via a system dbus session, which doesn't require
special priveleges during eecution or a custom udev rule and user group, which
usually aren't set up. This is the same thing
[brightnessctl](https://github.com/Hummer12007/brightnessctl) does.

It is of course an optional dependency, because you can still do everything
through sysfs, but it does impact quality of life.
