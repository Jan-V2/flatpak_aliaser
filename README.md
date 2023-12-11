# flatpak_aliaser
This project allows you to automatically generate aliases for installed flatpak packages, to make them easier to run from the terminal.

For example instead of typing `flatpak run com.discordapp.Discord` you can just type `discord`.

### Installation
1. [install flatpak](https://flatpak.org/setup/) and install any package.
2. [install cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
3. Run `cargo install flatpak_aliaser` to install this program.
4. Done! you should be able to run the program, by running `flatpak_aliaser` in the terminal.

### Usage
Let's assume you'd want to create an alias, for the discord flatpak.

First you'd need to install the discord flatpak by running `flatpak install flathub com.discordapp.Discord` in the terminal.
Once installation has completed, run this program by running `flatpak_aliaser`.

If the program runs succesfully, it will create a new file at `~/.flatpak_aliases`.
This file should contain the line: `alias discord='flatpak run com.discordapp.Discord'.

To add the alias to your enviorment, first open your bashrc by running `nano ~/.bashrc`, and adding `. ~/.flatpak_aliases` to the end of that file.
Finally, restart your terminal, to load the new aliases.


You should now be able to start the discord flatpak, by typing `discord` instead of `flatpak run com.discordapp.Discord`. 

### Configuration
Sometimes the default way of generating aliases, does not produce the alias you want, or perhaps you want to run a flatpak with options.
In that case you can change the config file to alter the aliases that are generated.

The configuration file is located at `~/.config/flatpak_aliaser/config.toml` and should be created, when you run the program for the first time.

##### Ommitting a flatpak
If you don't want a package to get an alias, add the full package name tot the `do_not_alias` section of the config file.
For example, if you wanted to ommit `'app.example.org'`, you would alter the line to look like this `do_not_alias = ['app.example.org']`

##### Providing an alternate alias, or running a flatpak with options
Sometimes the default way of generating an alias, does not create the alias you need, or you want to run a program with options.
This can be done, by adding another variable, to the special_aliases section of the config file.

For example, by default `flatpak run com.sublimetext.three` (sublime text) will get turned into `three`.
But by adding `['com.sublimetext.three', 'sublime', '-n']` to the special aliases section of the config file, it get turned into `sublime` instead.

The 1st string `'com.sublimetext.three'` is the package you want to alias.
The 2nd string `'sublime'` is the command to run this package.
The 3rd string `'-n'` is for configuring options used to run the flatpak. this string may be left empty, if you don't want to use any options.

In this case the command `sublime` will alias `flatpak run com.sublimetext.three -n`

##### Changing the default output file
By default, this program will create an output file at `~/.flatpak_aliases`.
To change this, open the config file, and change `destination_path`, to whatever you require.

##### preserving capitalisation
By default this program generates aliases, that are all lowercase.
So `com.discordapp.Discord` will get aliased as `discord` instead of `Discord`.
To disable this feature, open the config, and change `aliases_all_lowercase` from `true` to `false`


