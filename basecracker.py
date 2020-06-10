#!/usr/bin/python3

import sys
import re

name = 'basecracker'

all_bases = [
    ['16', 'hexadecimal'],
    ['64', 'base64']
]

# base functions

# get base functions
def get_encoder(base):
    return None

def get_decoder(base):
    return None

# main encoder
def main_encoder(plaintext, bases):
    cipher = plaintext

    for base in bases:
        base_encoder = get_encoder(base)
        if base_encoder is None:
            print('unknown base: ' + base + ' (ignored)')
            continue

        cipher = base_encoder(cipher)
        if cipher is None:
            return None

    return cipher

# main decoder
def main_decoder(cipher, bases):
    plaintext = cipher

    for base in bases:
        base_decoder = get_decoder(base)
        if base_decoder is None:
            print('unknown base: ' + base + ' (ignored)')
            continue

        plaintext = base_decoder(plaintext)
        if plaintext is None:
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