# BaseCracker

[![Build](https://github.com/skyf0l/BaseCracker/actions/workflows/ci.yml/badge.svg)](https://github.com/skyf0l/BaseCracker/actions/workflows/ci.yml)
[![](https://img.shields.io/crates/v/basecracker.svg)](https://crates.io/crates/basecracker)
[![Help Wanted](https://img.shields.io/github/issues/skyf0l/BaseCracker/help%20wanted?color=green)](https://github.com/skyf0l/BaseCracker/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)
[![codecov](https://codecov.io/gh/skyf0l/basecracker/branch/master/graph/badge.svg)](https://codecov.io/gh/skyf0l/basecracker)

BaseCracker is a tool to encode, decode and crack encoded data. It can be really useful to crack some random encoded strings in CTFs.

## Installation

From crates.io:

```console
cargo install basecracker
```

## Current supported encodings

- base2 / binary (padded by 8)
- base10 / decimal
- hexadecimal
- base32 (RFC4648)
- base36
- base58
- base62
- base64
- base85

## Options

```
Encode, Decode and Crack encoded data, useful to crack some random encoded strings in CTFs.

Usage: basecracker [OPTIONS] <COMMAND>

Commands:
  encode  Encode given plaintext/file using the specified bases
  decode  Decode given cipher/file using the specified bases
  crack   Crack given cipher/file
  help    Print this message or the help of the given subcommand(s)

Options:
  -q, --quiet
          Quiet mode, don't print anything except results
  -v, --verbose
          Verbose mode
  -m, --min-printable-percentage <MIN_PRINTABLE_PERCENTAGE>
          Minimum printable percentage to consider a result valid [default: 0.9]
  -n, --no-newline
          Do not output the trailing newline
  -h, --help
          Print help
  -V, --version
          Print version
```

## Example

```console
$ basecracker encode "Awsome CTF tool" b64,b85,hex,b32,b62,b58
2eSHB3WFgFiySPWP47oyrMrT6Vb4WXTEv5ZyWdmWWJNJ4H65n2auRW4ZFutQPtXegrNimoCAeUfiQwMAnb4UYg6grcK2WUCTL9LquGa4564JBJK2jAbRfPVjKx9sCgUVdrsUfyMuMR6MipKYERRr
$ basecracker decode 2eSHB3WFgFiySPWP47oyrMrT6Vb4WXTEv5ZyWdmWWJNJ4H65n2auRW4ZFutQPtXegrNimoCAeUfiQwMAnb4UYg6grcK2WUCTL9LquGa4564JBJK2jAbRfPVjKx9sCgUVdrsUfyMuMR6MipKYERRr b64,b85,hex,b32,b62,b58 -r
Awsome CTF tool
$ basecracker crack 2eSHB3WFgFiySPWP47oyrMrT6Vb4WXTEv5ZyWdmWWJNJ4H65n2auRW4ZFutQPtXegrNimoCAeUfiQwMAnb4UYg6grcK2WUCTL9LquGa4564JBJK2jAbRfPVjKx9sCgUVdrsUfyMuMR6MipKYERRr
Recipe: base58,base62,base32,hex,base85,base64
Awsome CTF tool
```

## Verbose mode

Useful if you want to see the steps of encoding/decoding/cracking

```console
$ basecracker -v crack 2eSHB3WFgFiySPWP47oyrMrT6Vb4WXTEv5ZyWdmWWJNJ4H65n2auRW4ZFutQPtXegrNimoCAeUfiQwMAnb4UYg6grcK2WUCTL9LquGa4564JBJK2jAbRf
PVjKx9sCgUVdrsUfyMuMR6MipKYERRr
Recipe: base58,base62,base32,hex,base85,base64
Applying base58:  9Y91a8AfMC1fYZFb6THWx0VBVu1R6BPhFsVhmAksMcKNLIibCXXnDGACS9woBiiuUhmwYgcEHrO4ZjPlvMVUTBxuOkLovyLgGTL2MOCZml9y
Applying base62:  GUYTIMZUMQ2TMNZQGU3DMYZXGA3TMNTGGRSTMYRXGY2TQNTGGUYTMNRVG43DINTCGU3DMYZXGA3WEMRV
Applying base32:  51434d5670566c70766f4e6b76586f516657646b566c707b25
Applying hex:     QCMVpVlpvoNkvXoQfWdkVlp{%
Applying base85:  QXdzb21lIENURiB0b29s
Applying base64:  Awsome CTF tool

Awsome CTF tool
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
