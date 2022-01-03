# Statisch

A staticly generated front for your server

# Installation

To build the project, clone it and compile it using
[cargo](https://doc.rust-lang.org/cargo/).

```
https://github.com/nils-degroot/statisch
cd statisch
cargo build --release
sudo ln -s `dir-to-statisch`/target/release/statisch /bin/statisch
```

# Usage

For a overview of options, use `statisch -h`.

Sites can be generated in two ways. You can use 
`statisch --config <CONFIG_FILE> --dump-html` to push to output to stdout. Or
to put all the required files into one directory, use 
`statisch --config <CONFIG_FILE> -o <OUTPUT_DIR>`.

# Configuration

Configuration is done via a config file in YAML. An example configuration is
shown down here:

```
# Title of the page
title: Statisch
# Theme to use
theme: gruvbox
# Font to use
font: some-font.ttf
# Favicon
favicon: some-icon.ico

# Applications section, must be a list
applications:
    # Name of the application
  - name: Public git
    # Link to the application
    link: https://git.peeko.nl
    # Icon for the application
    icon: fa-brands:git-alt
  - name: Github
    link: https://github.com/nils-degroot
    icon: fa-brands:github

# Bookmarks, a list of mark sections
bookmarks:
  # Name of the section
  - name: Media
    marks:
        # Name of the section
      - name: Whatsapp
        # Link to the bookmark
        link: https://web.whatsapp.com
      - name: Telegram
        link: https://web.telegram.org/z
```
