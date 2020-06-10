#!/usr/bin/python3

import sys

name = 'basecracker'

# help
def print_help():
    print('Usage:')
    print('    ' + name + ' cipher')
    print('     ->try to crack cipher')
    print('')
    print('    ' + name + ' -d base_names plaintext')
    print('     -> decode cipher from bases')
    print('    ' + name + ' -e base_names plaintext')
    print('     -> encode cipher in bases')
    print('')
    print('    base_names supported are 16,64')
    print('    base_names can be stacked and are applied in order (space and coma are delimiters)')
    print('')
    print('Exemple:')
    print('    $ ' + name + ' -e \'64 16\' basecracker')
    print('     -> 596d467a5a574e7959574e725a58493d')
    print('    $ ' + name + ' -d \'16,64\' 596d467a5a574e7959574e725a58493d')
    print('     -> basecracker')
    exit(0)

# main
def basecracker(args):
    if '-h' in args or '--help' in args:
        print_help()

if __name__ == '__main__':
    basecracker(sys.argv[1:])