# kotofetch

kotofetch is a small, configurable CLI tool that displays Japanese quotes in the terminal. It comes with built-in quotes and allows users to customize display options such as padding, width, translation display, and text styles.

![image](./images/01.png)

## Installation

### Arch Linux / AUR
You can install the stable release from the AUR:

```bash
yay -S kotofetch
```

> When prompted, choose All to clean-build the package from the downloaded PKGBUILD.

Or clone and build manually from [here](https://aur.archlinux.org/packages/kotofetch):
```bash
git clone https://aur.archlinux.org/kotofetch.git
cd kotofetch
makepkg -si
```

### From Source
Requires **Rust** and **Cargo**:

```bash
git clone https://github.com/hxpe-dev/kotofetch.git
cd kotofetch
cargo install --path .
```

After installation, you can run `kotofetch` from anywhere in your terminal.

## Configuration

### Config File

User configuration lives in:

```
~/.config/kotofetch/config.toml
```

Here you can customize:
- `horizontal_padding` / `vertical_padding` - spacing around quotes
- `width` - max width for text wrapping
- `show_translation` - display translations (true/false)
- `translation_color` - ANSI color for translations
- `font_size` - small, medium, or large
- `bold` - bold Japanese text
- `border` - show a box border
- `source` - show the quote source
- `mode` - default quote mode
- `seed` - RNG seed for random quotes

Example `config.toml`:
```toml
[display]
horizontal_padding = 3
vertical_padding = 1
width = 50
show_translation = true
translation_color = "#888888"
font_size = "medium"
bold = true
border = true
source = true
mode = "proverb"
seed = 0
```

### Custom quotes
Built-in quotes are embedded in the binary. To add your own quotes, create:
```
~/.config/kotofetch/quotes/
```

Place `.toml` files there with the same structure as the built-in ones (`proverbs.toml`, `haiku.toml`, `anime.toml`, see [this](https://github.com/hxpe-dev/kotofetch/tree/main/quotes)). These will automatically merge with the built-in quotes.

## Usage
```bash
kotofetch                           # display a random quote
kotofetch --mode haiku              # choose a specific mode
kotofetch --horizontal-padding 3    # override config temporarily
```

---

Made with ❤️ by [hxpe](https://github.com/hxpe-dev)  
If you enjoy **kotofetch**, consider starring the [GitHub repository](https://github.com/hxpe-dev/kotofetch)!
