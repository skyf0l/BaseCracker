# BaseCracker

[![Tests](https://github.com/skyf0l/BaseCracker/actions/workflows/tests.yml/badge.svg)](https://github.com/skyf0l/BaseCracker/actions/workflows/tests.yml)
[![Help Wanted](https://img.shields.io/github/issues/skyf0l/BaseCracker/help%20wanted?color=green)](https://github.com/skyf0l/BaseCracker/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)
[![Lines Of Code](https://tokei.rs/b1/github/skyf0l/BaseCracker?category=code)](https://github.com/skyf0l/BaseCracker)

BaseCracker is a tool to encode, decode and crack encoded data. It can be really useful to crack some random encoded strings in CTFs.

## Installation

You can build and install it from source using cargo:

```console
cargo install --git https://github.com/skyf0l/BaseCracker.git basecracker
```

## Current supported encodings

- base2 / binary (padding by 7, 8, 9, 10)
- base10 / decimal
- base16 / hexadecimal
- base32
- base36
- base58
- base62
- base64
- base85

## Options

```
basecracker 0.1.0
Encode, decode and crack encoded data

USAGE:
    basecracker [OPTIONS]

OPTIONS:
    -b, --bases <BASES>...      Set base to use (can be separated by comma or space)
    -c, --crack <CIPHER>        Crack the cipher/file using the specified bases (default: all)
    -d, --decode <CIPHER>       Decode given cipher/file using the specified bases
    -e, --encode <PLAINTEXT>    Encode given plaintext/file using the specified bases
    -h, --help                  Print help information
    -j, --json                  Output cracker results in json format
    -l, --list                  List supported bases
    -q, --quiet                 Quiet mode, don't print anything except results
    -r, --reversed              Reverse bases order (default: false)
    -v, --verbose               Verbose mode, print encoding/decoding steps
    -V, --version               Print version information
```

## Example

```console
$ basecracker -e "Awsome CTF tool" -b b64,b85,hex,b32,b62,b58
2SYnX25ZB1torUZ1AmsobB58ESDjzHzb2dJTZuwt22KfynQP1eRjxPoumGWiA45iGpRw2sx5LVB1D8K8xaLTTetafmPs3a44oiaFxrg3s4d4fkWJ36UzrSFWuLL6WbdQ5nbQSBCV7gC2DCzrxpaj
$ basecracker -d 2SYnX25ZB1torUZ1AmsobB58ESDjzHzb2dJTZuwt22KfynQP1eRjxPoumGWiA45iGpRw2sx5LVB1D8K8xaLTTetafmPs3a44oiaFxrg3s4d4fkWJ36UzrSFWuLL6WbdQ5nbQSBCV7gC2DCzrxpaj -b b64,b85,hex,b32,b62,b58 -r
Awsome CTF tool
$ basecracker -c 2SYnX25ZB1torUZ1AmsobB58ESDjzHzb2dJTZuwt22KfynQP1eRjxPoumGWiA45iGpRw2sx5LVB1D8K8xaLTTetafmPs3a44oiaFxrg3s4d4fkWJ36UzrSFWuLL6WbdQ5nbQSBCV7gC2DCzrxpaj
Recipe: base58 -> base62 -> base32 -> hex -> base85 -> base64
Result: Awsome CTF tool
```

## JSON output

BaseCracker can output results in JSON format. This is useful for other tools that can parse JSON output.

```console
$ basecracker -c 2SYnX25ZB1torUZ1AmsobB58ESDjzHzb2dJTZuwt22KfynQP1eRjxPoumGWiA45iGpRw2sx5LVB1D8K8xaLTTetafmPs3a44oiaFxrg3s4d4fkWJ36UzrSFWuLL6WbdQ5nbQSBCV7gC2DCzrxpaj -j | jq
{
  "cipher": "2SYnX25ZB1torUZ1AmsobB58ESDjzHzb2dJTZuwt22KfynQP1eRjxPoumGWiA45iGpRw2sx5LVB1D8K8xaLTTetafmPs3a44oiaFxrg3s4d4fkWJ36UzrSFWuLL6WbdQ5nbQSBCV7gC2DCzrxpaj",
  "plaintexts": [
    {
      "bases": [
        "base58",
        "base62",
        "base32",
        "hex",
        "base85",
        "base64"
      ],
      "plaintext": "Awsome CTF tool"
    }
  ]
}
```
