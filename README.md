# Morbled

The phonetic respelling of more backlight daemons.

A lighting control daemon written in rust.

Uses either systemd logind or sysfs directly.

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
