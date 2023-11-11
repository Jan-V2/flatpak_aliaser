# flatpak_aliaser
This project allows you to automatically generate aliases for installed flatpak packages, to make them easy to run from the terminal.

For example alias `flatpak run com.discordapp.Discord` gets turned into `discord`.

It has a config file, that allows for ommisson, or customisaton of specific commands.
For example, by default `flatpak run com.sublimetext.three` (sublime text) will get turned into `three`.
But by adding `['com.sublimetext.three', 'sublime', '']` to the special aliases section of the config file, it get turned into `sublime` instead.
The 3rd string is for configuring options
