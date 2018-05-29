# weekit

A tiny user interface toolkit for Rust applications.

## Raspberry Pi

On Raspberry Pi, requires clang and libjpeg-dev.

```
% sudo apt-get install clang libjpeg-dev -y
```

On Raspberry Pi, the touchscreen is assumed to be at /dev/input/touchscreen
and the keyboard is at /dev/input/keyboard.

To map the 7" touchscreen, create the following file:

```
$ cat /etc/udev/rules.d/70-touchscreen-raspberrypi.rules 
KERNEL=="event*",ATTRS{name}=="FT5406 memory based driver",SYMLINK+="input/touchscreen",MODE="0440"
```

To map the keyboard, create the following file:

```
$ cat /etc/udev/rules.d/80-keyboard.rules 
KERNEL=="event*",ENV{ID_INPUT_KEYBOARD}=="?*",SYMLINK+="input/keyboard",MODE="0440"
```

After rebooting, you should find /dev/input/touchscreen and /dev/input/keyboard.

## Mac OS X

Mac users, please clone the [AmanithVG SDK](https://github.com/Mazatech/amanithvg-sdk.git) to your Desktop.

