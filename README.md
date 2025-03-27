# Morbled

The phonetic respelling of more backlight daemons.

A lighting control daemon written in rust targetting low bloat and the best
possible out of the box, quality of life experience.

Uses either the systemd/logind dbus or sysfs directly to control brightness.

## Compiling and Installing

Morbled has a dependency on libsystemd (which may be provided by elogind).

This dependency can be disabled by disabling the `"sd_dbus"` feature.

You are pretty much guaranteed to have the runtime dependencies, but for
compilation you might need to install something.
```sh
# To compile
apt install libsystemd-dev    # debian
dnf install systemd-devel     # fedora
xbps install -S elogind-devel # void
apk add elogind-dev           # alpine
```

Everything else is handled automatically by cargo.

```sh
cargo install --path .

# For no systemd dbus
cargo install --no-default-features --path .
```

### Services and udev

Some sample service files and udev rules are provided in the
[service/](./service/) directory.

If you are running morbled without systemd/logind, you will want to install
the udev rules.

Otherwise, a service file for systemd has also been provided.

It is also possible to manually start the daemon by running the `morbled`
executable.

## Running

Ensure the morble dameon (`morbled`) is running, directly or via a service
manager of your choice.

Run `morblectl` to communicate with the daemon and set the backlight.

For example:

```sh
morblectl 100      # set to raw 100 brightness
morblectl + 100    # increase by 100 raw
morblectl - 100    # decrease by 100 raw
morblectl % 50     # set to 50% brightness
morblectl + % 5    # inrease by 5%
morblectl - % 5    # decrease by 5%
```

## State

Morbled is very much in alpha right now, with a lot of ways it can improve.
Contributors are very welcome, please see [CONTRIBUTING.md](./CONTRIBUTING.md)
for more information.

## Credits

Copyright (C) 2025 Cyuria.
