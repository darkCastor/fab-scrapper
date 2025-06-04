# FAB Card Scraper

A Rust-based tool for collecting card data from the Flesh and Blood Trading Card Game (FAB TCG) official API.

## Features

- Fetches card data for specific set codes
- Saves card data in JSON format (as .txt files)
- Handles API rate limiting with polite delays
- Error handling and logging
- Supports batch processing of multiple set codes

## Prerequisites

- Rust (edition 2024)
- Cargo (Rust's package manager)
- Internet connection to access the FAB TCG API

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
CRU
MON
```

3. Run the program:
```bash
cargo run --release
```

The program will:
- Read set codes from `sets_codes.txt`
- Create a `set_data_json_txt` directory
- Fetch data for each set code
- Save the results as `{SET_CODE}_cards.txt` in the output directory

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
- HER: History Pack 1
- DTD: Dusk till Dawn
- THS: The High Seas

## Error Handling

The program includes robust error handling for common issues:
- Missing input file
- Network connectivity problems
- API rate limiting
- File system operations

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