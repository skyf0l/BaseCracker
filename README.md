# BaseCracker

Encoder, Decoder and Cracker for many Bases
An awesome tool to crack some random bases in CTF

## Supported bases

- base2 / binary
- base16 / hexadecimal
- base32
- base64
- base85

## Installation

You just need to clone this repo and run `basecracker.py`

**No library is required !**

## Usage

### Crack:
Usage
```
$ ./basecracker.py cipher

try to crack cipher
```

Exemple
```
$ ./basecracker.py TkRjMU5UTTBOVFEwWkRWaE5USTFPRFEzTlRVMVlUUTBOREUwWlRVeU5UUTBOelU1TlRrMU5EUmtOV0UwWVRVek5EYzBNVE16TkRjME56UmxOVEkxTmpRM016UXpNalEwTkRVMFpEUXlOVGMwTnpRMU5XRTBORFF4TkdVMVlUVTBORGN6TkRNeU5EUTBaRFJrTkdFMU9EUTNORGsxWVRRME5ERTBaRE16TkRJME56VXhNekkwTVROa00yUXpaRE5r
Cipher: TkRjMU5UTTBOVFEwWkRWaE5USTFPRFEzTlRVMVlUUTBOREUwWlRVeU5UUTBOelU1TlRrMU5EUmtOV0UwWVRVek5EYzBNVE16TkRjME56UmxOVEkxTmpRM016UXpNalEwTkRVMFpEUXlOVGMwTnpRMU5XRTBORFF4TkdVMVlUVTBORGN6TkRNeU5EUTBaRFJrTkdFMU9EUTNORGsxWVRRME5ERTBaRE16TkRJME56VXhNekkwTVROa00yUXpaRE5r
Apply base64: NDc1NTM0NTQ0ZDVhNTI1ODQ3NTU1YTQ0NDE0ZTUyNTQ0NzU5NTk1NDRkNWE0YTUzNDc0MTMzNDc0NzRlNTI1NjQ3MzQzMjQ0NDU0ZDQyNTc0NzQ1NWE0NDQxNGU1YTU0NDczNDMyNDQ0ZDRkNGE1ODQ3NDk1YTQ0NDE0ZDMzNDI0NzUxMzI0MTNkM2QzZDNk
Apply base64: 475534544d5a525847555a44414e5254475959544d5a4a5347413347474e525647343244454d425747455a44414e5a54473432444d4d4a5847495a44414d3342475132413d3d3d3d
Apply base16: GU4TMZRXGUZDANRTGYYTMZJSGA3GGNRVG42DEMBWGEZDANZTG42DMMJXGIZDAM3BGQ2A====
Apply base32: 596f752063616e206c657420612073746172203a44
Apply base16: You can let a star :D
Decode order: base64,base64,base16,base32,base16
Plaintext: You can let a star :D
```

### Encode:

Usage
```
$ ./basecracker.py -e/-E plaintext base_names

encode cipher in bases
    -e display details
    -E display only result
```

Exemple
```
$ ./basecracker.py -E basecracker '64 base16'
596d467a5a574e7959574e725a58493d
$ ./basecracker.py -e basecracker '64 base16'
Plaintext: basecracker

Apply base64: YmFzZWNyYWNrZXI=
Apply base16: 596d467a5a574e7959574e725a58493d

Cipher: 596d467a5a574e7959574e725a58493d
```

### Decode:
Usage
```
$ ./basecracker.py -d/-D plaintext base_names

decode cipher from bases
    -d display details
    -D display only result
```

Exemple
```
$ ./basecracker.py -D 596d467a5a574e7959574e725a58493d '16,base64'
basecracker
$ ./basecracker.py -d 596d467a5a574e7959574e725a58493d '16,base64'
Cipher: 596d467a5a574e7959574e725a58493d

Apply base16: YmFzZWNyYWNrZXI=
Apply base64: basecracker

Plaintext: basecracker
```

