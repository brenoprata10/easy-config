## Easy Config

A glorified multi-thread shell executor

### About

This project was built with the purpose to solve "copy paste" hell while configuring a new setup or setting up a dev environment based on a list of terminal commands that need to be executed.

### Installation

#### Github Release

1. Download the [latest release](https://github.com/brenoprata10/easy-config/releases/latest);
2. Run `sudo chmod +x easy-config` to make it an executable;
3. Run the binary like: `./easy-config your_config.toml`.

#### From Source

The project is quite small and fast to compile. It should take less than a minute to have everything ready to go:

1. Clone the repo;
2. Run `cargo run -- your_config.toml`
