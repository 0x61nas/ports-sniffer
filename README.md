## A simple port sniffer(scanner) implementation with 🦀


[![crates.io](https://img.shields.io/crates/v/ports-sniffer?style=for-the-badge)](https://crates.io/crates/ports-sniffer)

[![Codacy Badge](https://api.codacy.com/project/badge/Grade/3e0d24aa2c1441e484622b8540193cdf)](https://app.codacy.com/gh/anas-elgarhy/ports-sniffer?utm_source=github.com&utm_medium=referral&utm_content=Anas-Elgarhy/cmus-rpc&utm_campaign=Badge_Grade_Settings)
[![CodeFactor](https://www.codefactor.io/repository/github/anas-elgarhy/ports-sniffer/badge)](https://www.codefactor.io/repository/github/anas-elgarhy/cmus-rpc)

## Install

- from crates.io
    ```bash
    crago install ports-sniffer 
    ```
- From aur: `yay -S ports-sniffer`

### Arguments

| Argument            | Description                          |
|---------------------|--------------------------------------|
| `-h` or `--help`    | Show help                            |
| `-v` or `--version` | Show version                         |
| `-t` or `--threads` | Set number of threads (4 by default) |

### Usage

```bash
ports-sniffer <ip> [-t <threads>]
```

### Examples

```bash
ports-sniffer 192.168.1.1
```
```bash
ports-sniffer 192.168.1.1 -t 1000
```


[![License MIT](https://img.shields.io/badge/license-MIT-green.svg)](https://spdx.org/licenses/MIT.html)
