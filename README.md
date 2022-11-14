# circadianlight

## What It Is

Circadian Light is a program, currently only working on Linux with X, that
controls the color spectrum of your screen according to the current day time
in order to improve the quality of your sleep.
The more night it becomes, the more red your computer screen
will emit (actually, it is more correct to say that it will emit less green and
blue light).

## How It Works

The program deals with three day phases: day, dusk and night. In the default
configuration, during the "day" phase, your computer will emit full intensity
for all of red, green and blue color channels. During the "dusk" phase, it will
slowly make the colors contain more red than green or blue. And during the
"night" phase, your screen will emit the maximum configured redness, which is
`red=1.0 green=0.65 blue=0.45`.

## How To Use It

The program can be used as a service and can be managed by `systemd`, but it
can also apply the gamma color spectrum to your screen one time. Also, it can be
simply used to print the gamma color spectrum (without applying it) for the
current day hour (or a given hour), even though you're not on Linux.

The program can be configured, please run `circadianlight --help`.

## How To Install/Uninstall 

### On Linux, With Systemd

Before installing, you may want to customize the systemd unit file, namely
`circadianlight.service`, especially you might want to change CLI arguments.

To install it on linux, targeting X and systemd, simply run:

```sh
make install
```

To uninstall:

```sh
make uninstall
```

### Other Platforms (Including Linux Without Systemd)

This can be simply a cargo install:
```sh
cargo install --path .
```
