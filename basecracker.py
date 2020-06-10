#!/usr/bin/python3

import sys
import re

name = 'basecracker'

# base functions
def base16_encoder(plaintext):
    return plaintext
def base16_decoder(cipher):
    return cipher
def base64_encoder(plaintext):
    return plaintext
def base64_decoder(cipher):
    return cipher

# base tab
all_bases = [
    ['16', 'hexadecimal', base16_encoder, base16_decoder],
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

    for base in bases:
        base_data = get_base_data(base)
        if base_data is None:
            print('unknown base: ' + base + ' (ignored)')
            continue

        cipher = base_data[ENCODER](cipher)
        if cipher is None:
            print('error while encoding in base ' + base_data[0] + ' (' + base_data[1] + ')')
            return None

    return cipher

# main decoder
def main_decoder(cipher, bases):
    plaintext = cipher

    for base in bases:
        base_data = get_base_data(base)
        if base_data is None:
            print('unknown base: ' + base + ' (ignored)')
            continue

        plaintext = base_data[DECODER](plaintext)
        if plaintext is None:
            print('error while decoding in base ' + base_data[0] + ' (' + base_data[1] + ')')
            return None

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
    print('')
    print('    ' + name + ' -e plaintext base_names')
    print('     -> encode cipher in bases')
    print('    ' + name + ' -d plaintext base_names')
    print('     -> decode cipher from bases')
    print('')
    print('    base_names can be stacked and are applied in order (space and coma are delimiters)')
    print('    base_names supported are:')
    for base in all_bases:
        print('        ' + base[0] + '\talias ' + base[1])
    print('')
    print('Exemple:')
    print('    $ ' + name + ' -e basecracker \'64 hexadecimal\'')
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