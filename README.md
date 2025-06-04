# FAB Card Scraper

A Rust-based tool for collecting card data from the Flesh and Blood Trading Card Game (FAB TCG) official API.

## Features

- Fetches card data for specific set codes
- Saves card data in both JSON and TXT formats in separate folders
- Creates a combined file with all sets data
- Generates metadata file with script execution details
- Handles API rate limiting with polite delays
- Error handling and logging
- Supports batch processing of multiple set codes

## Prerequisites

- Rust (edition 2024)
- Cargo (Rust's package manager)
- Internet connection to access the FAB TCG API

## Dependencies

- `reqwest` - For HTTP requests to the FAB TCG API
- `chrono` - For timestamp generation in metadata files

## Installation

1. Clone this repository:
```bash
git clone https://github.com/yourusername/fab-scrapper.git
cd fab-scrapper
```

2. Build the project:
```bash
cargo build --release
```

## Usage

1. Create a `sets_codes.txt` file in the project root directory
2. Add one set code per line in the file. For example:
```
WTR
ARC
MON
ELE
UPR
DYN
OUT
DTD
EVO
HVY
MST
ROS
HNT
SEA
```

3. Run the program:
```bash
cargo run --release
```

The program will:
- Read set codes from `sets_codes.txt`
- Create a `script_generated_card_data` directory with two subdirectories:
  - `txt/` - Contains .txt versions of all files
  - `json/` - Contains .json versions of all files
- Fetch data for each set code
- Save individual results as `{SET_CODE}_cards.txt` and `{SET_CODE}_cards.json`
- Create combined files `all_sets_combined.txt` and `all_sets_combined.json`
- Generate a `script_metadata.txt` file with execution details and latest set information

## Set Codes

Common set codes include:
- WTR: Welcome to Rathe
- ARC: Arcane Rising
- CRU: Crucible of War
- MON: Monarch
- ELE: Tales of Aria
- EVR: Everfest
- UPR: Uprising
- DYN: Dynasty
- OUT: Outsiders
- HVY: Heavy Hitters
- DTD: Dusk till Dawn
- EVO: Bright Lights
- HVY: Heavy Hitters
- MST: Part the Mistveil
- ROS: Rosetta
- HNT: The Hunted
- SEA: Battles of Legend

## Output Structure

After running the script, you'll find the following structure:

```
script_generated_card_data/
├── script_metadata.txt          # Execution metadata and latest set info
├── txt/
│   ├── WTR_cards.txt           # Individual set files
│   ├── ARC_cards.txt
│   ├── ...
│   └── all_sets_combined.txt   # Combined data from all sets
└── json/
    ├── WTR_cards.json          # Same data as JSON files
    ├── ARC_cards.json
    ├── ...
    └── all_sets_combined.json  # Combined JSON data
```

## Error Handling

The program includes robust error handling for common issues:
- Missing input file
- Network connectivity problems
- API rate limiting
- File system operations
- Partial failures (continues processing other sets if one fails)

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Legend Story Studios](https://legendstory.com/) for providing the FAB TCG API
- The FAB TCG community for their support and feedback

## Disclaimer

This tool is not affiliated with, endorsed, sponsored, or specifically approved by Legend Story Studios. This is an unofficial tool and is not guaranteed to be complete or accurate.