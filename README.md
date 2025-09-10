# kotofetch

kotofetch is a small, configurable CLI that displays Japanese quotes in the terminal. It ships built-in quotes and allows user configuration for padding, width, translation display, and styles.

## Config
User config lives in `~/.config/kotofetch/config.toml` and controls display options. Built-in quotes are embedded in the binary; users may place custom quotes in `~/.config/kotofetch/quotes.toml`.

## Usage
`kotofetch` - default
`kotofetch --mode haiku --horizontal-padding 3` 