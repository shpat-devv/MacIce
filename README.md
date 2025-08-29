# MACICE
======================

A CLI tool for automatically syncing settings across various applications.

## Table of Contents

1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Features](#features)
4. [Usage](#usage)
5. [Contributing](#contributing)

## Introduction
-------------

MacIce is a Rust-based CLI tool designed to simplify the process of
syncing settings across multiple applications. Currently, it supports Yabai,
SketchyBar, Alacritty, VS Code, Firefox, and Spicetify.

## Getting Started
---------------

To get started with MacIce, you'll need:

*   Rust installed on your system (version 1.57.0 or higher)
*   Yabai, a window manager
*   Sketchybar, a top bar for mac os
*   Alacritty, cross platform terminal emulator
*   Vs Code, a code editor
*   Firefox, the browser
*   Spicetify, a spotify tweaking tool

### Clone the Repository

```bash
git clone https://github.com/shpat-devv/MacIce.git
```

Navigate into the newly cloned directory and run `cargo build`.

## Features
------------

*   **Application Support**: Currently supports Yabai, SketchyBar, Alacritty, VS
Code, Firefox, and Spicetify.
*   **Sync Settings Across Apps**: Automatically syncs settings between
applications.
*   **Flexible Configuration**: Allows for customizing which settings
to sync and the order of operations.

### Future Development

*   **GUI Option**: A graphical user interface is being planned to make
configuration easier.

## Usage
-----

To use MacIce, simply run it with the following command:

```bash
cargo run --config {theme}
```

Replace `theme` with what one of the following themes:

Dark
Light

The tool will automatically sync settings between applications. 
## Contributing
------------

Contributions to MacIce are welcome. Please submit pull requests or
issues on the [GitHub
repository](https://github.com/your-username/auto-sync-settings).

### Code Contribution Guidelines

*   Ensure all changes follow Rust coding standards.
*   Write unit tests for added functionality.
*   Maintain documentation and comments for code readability.

## License
--------

MacIce is released under the MIT License. See the `LICENSE` file for more
details.

[License](https://github.com/your-username/auto-sync-settings/blob/main/LICENSE)


## How the code works
--------------------
verify the paths in the csv file.
if not correct try checking for the possible paths.
if possible paths are not correct ask user for the paths
save paths onto the csv file
manipulate data