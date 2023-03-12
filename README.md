# mc-map-tools
mc-map-tools is a program that is capable of reading Minecraft save games and analyzing them. It is meant for server owners and administrators. At this point, there is only a CLI Program available.

## Features
### Searching for item stashes of duped items
mc-map-tools allows administrators to quickly find stashes of duplicated items. This is done by reading the entire map and scanning for high concentrations of items. Thresholds for items can be freely configured in the configuration file.
### Fast custom map parser
Maps are read by a custom map parser written in Rust. The reason to implement a custom solution was that a purpose build parser can be optimized much better than a more general implementation. Using Rust not only results in a short runtime but also eliminates many preventable crashes.

## Configuration
The configuration is stored in the file `config.json`. The location of this file depends on the operating system:
* Windows: `%APPDATA%\mc-map-tools\config.json`
* Linux: `$XDG_CONFIG_HOME/.config/mc-map-tools/config.json` or `~/.config/mc-map-tools/config.json`
* macOS: `$HOME/Library/Application Support/mc-map-tools/config.json`

Example configuration:
```json
{
    "search_dupe_stashes": {
        "groups": {
            "diamond": {
                "items": [{
                    "id": "minecraft:diamond"
                },{
                    "id": "minecraft:diamond_block",
                    "multiplier": 9
                }],
                "threshold": 50000
            },
        }
    }
}
```

### search_dupe_stashes
This section contains the configuration for the dupe stash search. 
The `groups` section contains a list of item groups. 
Each group has a list of items and a threshold. 
The threshold is the minimum number of items that have to be found in a single chunk for the chunk to be reported. 
The `items` section contains a list of items. 
Each item has an `id` and an optional `multiplier` and a optional `nbt`. 
The `id` is the item ID as used by minecraft. 
You can use wildcards inside of ids `?` matches one character while `*` matches any number of characters. 
The `multiplier` is the number is used to count a single item multiple times. For example, a diamond block has a multiplier of 9 because it contains 9 diamonds. 
This way not the actual amount of diamonds and diamond blocks is counted but the theoretical amount of diamonds.
The `nbt` section is used to match items with specific NBT data. 
The NBT data is specified as a JSON object. 
The following example matches a diamond sword with the `Unbreakable` tag set to `1b`:
```json
...
{
    "id": "minecraft:diamond_sword",
    "nbt": {
        "Unbreakable": true
    }
}
...
```
`nbt` does not support arrays or lists.

## Usage
Every command requires a path to a Minecraft world directory. This is allways the first argument.
| Argument | Description | Optional | Values | Default |
| --- | --- | --- | --- | --- |
| <SAVE_DIRECTORY> | The path to the Minecraft world directory | No | A valid path | |

### search_dupe_stashes
This command searches for item stashes of duped items.
```bash
mc-map-tools <SAVE_DIRECTORY> search_dupe_stashes [OPTIONS] <MODE>
```

| Option | Description | Optional | Values | Default |
| --- | --- | --- | --- | --- |
| -a, --area | The area to search in | Yes | A string in the format `x1,z1;x2,z2` | The entire map |

| Argument | Description | Optional | Values | Default |
| --- | --- | --- | --- | --- |
| <MODE> | The mode used to find stashes. Currently not used | Yes | `absolute` or `groth-rate` | `absolute` |



## Installation

### From source
To install mc-map-tools from source, you need to have git and Rust installed. 
You can install Rust using [rustup](https://rustup.rs/). 
Once you have Rust installed, you can install mc-map-tools by running the following command:
```bash
cargo install --git https://github.com/Julian-Alberts/mc-map-tools.git mc-map-tools
```
Make sure that `~/.cargo/bin` is in your `PATH` environment variable.

### From binaries
Binaries for Windows and Linux are available on the [releases page](https://github.com/Julian-Alberts/mc-map-tools/releases). 
Download the binary for your operating system and extract it. 
Make sure that the binary is in your `PATH` environment variable. (Not yet available)
