# BaseCracker

Encoder, Decoder and Cracker for many Bases

An awesome tool to crack some random bases in CTF

## Supported bases

- base2 / binary
- base16 / hexadecimal
- base32
- base58
- base62
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
$ ./basecracker.py PiI8WFFHPT5xMkJtKktnQTJAaDk6M0NWWTplRzd1QjE1dVw6aiJYMTwsUUpIOUpvQGNBTiI9SERiLChPRmE1YGRAV2pKSkg7JCwqOzBAIlk7YCRkT0YqKVkrQk1xOS4xY1I3LzIrVnBJQkwtbzlGKXR0TEA7ZiVYN1NsaW42PEAnKEJKRDg9MkgiOjIyRkRAX0VkMVZeRkQrJ0I3UkMtNkQtOS5xR1paZUVAVCMoKkZdX2RpOWVvajExST5NQkFqZU90RStOY1BHWTJMRDY4VjtFQ0lpYlNDTUBvYQ==
Cipher: PiI8WFFHPT5xMkJtKktnQTJAaDk6M0NWWTplRzd1QjE1dVw6aiJYMTwsUUpIOUpvQGNBTiI9SERiLChPRmE1YGRAV2pKSkg7JCwqOzBAIlk7YCRkT0YqKVkrQk1xOS4xY1I3LzIrVnBJQkwtbzlGKXR0TEA7ZiVYN1NsaW42PEAnKEJKRDg9MkgiOjIyRkRAX0VkMVZeRkQrJ0I3UkMtNkQtOS5xR1paZUVAVCMoKkZdX2RpOWVvajExST5NQkFqZU90RStOY1BHWTJMRDY4VjtFQ0lpYlNDTUBvYQ==
Apply base64: >"<XQG=>q2Bm*KgA2@h9:3CVY:eG7uB15u\:j"X1<,QJH9Jo@cAN"=HDb,(OFa5`d@WjJJH;$,*;0@"Y;`$dOF*)Y+BMq9.1cR7/2+VpIBL-o9F)ttL@;f%X7Slin6<@'(BJD8=2H":22FD@_Ed1V^FD+'B7RC-6D-9.qGZZeE@T#(*F]_di9eoj11I>MBAjeOtE+NcPGY2LD68V;ECIibSCM@oa
Apply base85: ZGspwEaHivUfd3foNusVPJdUgEZ4Pu5jTnkNL5Hxe6kuoHwTuy2wbv1tzL3dQuuQS4AestxThREb482o5Hw3hAyJssg2aoRiFPNhBWRph12P6Rjs6CnVrxVQthDUFBV6mYAHxX4tbR5tuXYvM2Y73BaJfD6rpejDxJdQB4JckHzYkiuK
Apply base58: Gty9ZxMdGhOU1HjuVrXqyRr5eUwfPPCA0XUMlBqCvZUTDeiUJAPDQ4v5zo9BKF3nTMR5Dl8OQSEWS7Noak2Y4SMWHaqvlztjoTDcENJdAjxAZKL7gh2EPXmQb1uAizHaf
Apply base62: JZKGWMS2NJRTCTLKIEZE26SZPBHG2VLZJVCFU2SONJKTGTSEJF3U42SFPFGUIY32JZ5FCMSNKRRXSTLKIF5FSVCRGA======
Apply base32: NTk2Zjc1MjA2MzYxNmUyMDZjNjU3NDIwNjEyMDczNzQ2MTcyMjAzYTQ0
Apply base64: 596f752063616e206c657420612073746172203a44
Apply base16: You can let a star :D
Decode order: base64,base85,base58,base62,base32,base64,base16
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

