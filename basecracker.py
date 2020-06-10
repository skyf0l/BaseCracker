#!/usr/bin/python3

import base64
import sys
import re
import string

name = 'basecracker'

def split_by_size(string, size):
    splited = []
    for k in range(0, len(string), size):
        splited.append(string[0 + k:size + k])
    return splited

# base functions
def int_to_base(num, base, size):
    encode = ''
    while num:
        encode += base[num % len(base)]
        num //= len(base)
    encode += '0' * (size - len(encode))
    return encode[::-1]

# base2
def base2_encoder(plaintext):
    cipher = ''
    for c in plaintext:
        cipher += int_to_base(ord(c), '01', 8)
    return cipher
def base2_decoder(cipher):
    plaintext = ''
    tokens = split_by_size(cipher, 8)
    for token in tokens:
        plaintext += chr(int(token, 2))
    return plaintext

# base16
def base16_encoder(plaintext):
    return plaintext
def base16_decoder(cipher):
    return cipher

# base64
base64_alphabet = string.ascii_uppercase + string.ascii_lowercase + string.digits + '+/'
def base64_encoder(plaintext):
    return base64.b64encode(bytearray(plaintext, 'utf8')).decode('utf-8')
def base64_decoder(cipher):
    return base64.b64decode(cipher).decode('utf-8')

# base tab
all_bases = [
    ['2', 'base2', base2_encoder, base2_decoder],
    ['16', 'base16', base16_encoder, base16_decoder],
    ['64', 'base64', base64_encoder, base64_decoder]
]
ENCODER = 2
DECODER = 3

# get base functions
def get_base_data(base_name):
    for base in all_bases:
        if base_name == base[0] or base_name == base[1]:
            return base
    return None

# main encoder
def main_encoder(plaintext, bases):
    cipher = plaintext
    print('Plaintext: ' + plaintext)
    print()

    for base in bases:
        base_data = get_base_data(base)
        if base_data is None:
            print('Unknown base: ' + base + ' (ignored)')
            continue

        cipher = base_data[ENCODER](cipher)
        if cipher is None:
            print('Error while encoding in ' + base_data[1])
            return None

        print('Apply ' + base_data[1] + ': ' + cipher)

    print()
    print('Cipher: ' + cipher)
    return cipher

# main decoder
def main_decoder(cipher, bases):
    plaintext = cipher
    print('Cipher: ' + cipher)
    print()

    for base in bases:
        base_data = get_base_data(base)
        if base_data is None:
            print('Unknown base: ' + base + ' (ignored)')
            continue

        plaintext = base_data[DECODER](plaintext)
        if plaintext is None:
            print('Error while decoding in base ' + base_data[0] + ' (' + base_data[1] + ')')
            return None

        print('Apply ' + base_data[1] + ': ' + plaintext)

    print()
    print('Plaintext: ' + plaintext)
    return plaintext

# main cracker
def main_cracker(cipher):
    print('cracker')

# parse bases
def parse_bases(bases_str):
    bases = re.split(' |,',bases_str)
    bases = list(filter(('').__ne__, bases)) # remove empty elements
    return bases

# main
def main(args):
    if '-h' in args or '--help' in args:
        print_help()
    if len(args) == 0:
        print_miss_args()

    if len(args) == 1:
        main_cracker(args[0])
    elif len(args) == 3:
        bases = parse_bases(args[2])

        if args[0] == '-e':
            main_encoder(args[1], bases)
        elif args[0] == '-d':
            main_decoder(args[1], bases)
        else:
            print_invalid_instruction()

    else:
        print_miss_args()

# display
def print_help():
    print('Usage:')
    print('    ' + name + ' cipher')
    print('     ->try to crack cipher')
    print()
    print('    ' + name + ' -e plaintext base_names')
    print('     -> encode cipher in bases')
    print('    ' + name + ' -d plaintext base_names')
    print('     -> decode cipher from bases')
    print()
    print('    base_names can be stacked and are applied in order (space and coma are delimiters)')
    print('    base_names supported are:')
    for base in all_bases:
        print('        ' + base[0] + '\talias ' + base[1])
    print()
    print('Exemple:')
    print('    $ ' + name + ' -e basecracker \'64 base16\'')
    print('     -> 596d467a5a574e7959574e725a58493d')
    print('    $ ' + name + ' -d 596d467a5a574e7959574e725a58493d \'16,base64\'')
    print('     -> basecracker')
    exit(0)

def print_miss_args():
    print(name + ': miss arguments')
    print('    try --help for help')
    exit(0)

def print_invalid_instruction():
    print(name + ': invalid instruction')
    print('    try --help for help')
    exit(0)

if __name__ == '__main__':
    main(sys.argv[1:])