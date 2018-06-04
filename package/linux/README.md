This file contains a udev rules file that makes using _joycon_ easier. It:

* Enables user access to the Joy-Cons' raw HID interface, allowing _joycon_ to be run as a regular user, and
* Disables the /dev/js# files created by the Linux kernel for each individual Joy-Con, so virtual devices will occupy the correct player slots

To use this file, copy it into `/etc/udev/rules.d` and reload your udev rules:

    udevadm control --reload-rules

Package maintainers are asked to include this file while distributing _joycon_.
