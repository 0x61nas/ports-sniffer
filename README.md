## A simple port sniffer(scanner) implementation with 🦀

### Arguments

| Argument            | Description                          |
|---------------------|--------------------------------------|
| `-h`                | Show help                            |
| `-v`                | Show version                         |
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
