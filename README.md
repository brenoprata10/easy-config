## Easy Config

A glorified multi-thread shell executor.

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

#### Configuration

Easy Config expects a `.toml` file with the following template:

```
[[library]]
name = "NAME"
install_script = "INSTALL SCRIPT"
id = "ID" # Optional
allow_async = true # Optional 
group = "GROUP" # Optional
```

| Property | Type | Default Value | Description |
|---|---|---|---|
| name | String |  | Label that will be used while running the command |
| install_script | String |  | Script that will be run |
| id | String |  | Id that is needed if you'd like to run a single command instead of the whole config file |
| allow_async | bool | false | Single command that will be run in a separate thread |
| group | String |  | Commands that will be run sequentially in a separate thread |
