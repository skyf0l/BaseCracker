#!/usr/bin/python3

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

def cipher_padding(cipher):
    cipher = cipher.replace(' ', '')
    cipher = cipher.replace('\n', '')
    return cipher

# plaintext can is encoded in base x
def is_base(cipher, base_data):
    if base_data[0] == '16':
        return is_base16(cipher)
    cipher = cipher_padding(cipher)

    for k in range(0, len(cipher)):
        if cipher[k] not in base_data[ALPHABET]:
            break
    if base_data[COMPLEMENT] is None:
        if k == len(cipher) - 1:
            return True
        return False
    for k in range(k, len(cipher)):
        if cipher[k] not in base_data[COMPLEMENT]:
            return False
    return True

# base2
base2_alphabet = '01'
def base2_encoder(plaintext):
    cipher = ''
    for c in plaintext:
        cipher += int_to_base(ord(c), base2_alphabet, 8)
    return cipher

def base2_decoder(cipher):
    plaintext = ''
    cipher = cipher_padding(cipher)
    tokens = split_by_size(cipher, 8)
    for token in tokens:
        plaintext += chr(int(token, 2))
    return plaintext

# base16
base16_alphabet = '0123456789abcdef'
def base16_encoder(plaintext):
    cipher = ''
    for c in plaintext:
        cipher += base16_alphabet[ord(c) // 16] + base16_alphabet[ord(c) % 16]
    return cipher

def base16_decoder(cipher):
    plaintext = ''
    cipher = cipher.lower()
    cipher = cipher_padding(cipher)
    tokens = split_by_size(cipher, 2)
    for token in tokens:
        plaintext += chr(int(token, 16))
    return plaintext
def is_base16(cipher):
    cipher = cipher.lower()
    cipher = cipher_padding(cipher)
    for c in cipher:
        if c not in base16_alphabet:
            return False
    return True

# base32
base32_alphabet = string.ascii_uppercase + '234567'
base32_complement = '='
base32_nb_complements = [0, 6, 4, 3, 1]
base32_padding_size = 5
def base32_encoder(plaintext):
    cipher = ''

    # padding
    nb_tokens = len(plaintext) // base32_padding_size * 8
    if len(plaintext) % base32_padding_size != 0:
        nb_tokens += 8 - base32_nb_complements[len(plaintext) % base32_padding_size]
        plaintext += '\x00' * (base32_padding_size - (len(plaintext) % base32_padding_size))
    base2_cipher = base2_encoder(plaintext)

    tokens = split_by_size(base2_cipher, 5)
    token_id = 0
    for token in tokens:
        if token_id >= nb_tokens:
            cipher += base32_complement
        elif len(token) == 5:
            cipher += base32_alphabet[int(token, 2)]
        else:
            complement = base32_complement * ((5 - len(token)) // 2)
            token += base2_alphabet[0] * (len(complement) * 2)
            cipher += base32_alphabet[int(token, 2)]
            cipher += complement
        token_id += 1
    return cipher

def base32_decoder(cipher):
    base2_plaintext = ''
    nb_complements = 0
    cipher = cipher_padding(cipher)
    for c in cipher:
        if c in base32_alphabet:
            base2_plaintext += int_to_base(base32_alphabet.index(c), base2_alphabet, 5)
        elif c in base32_complement:
            base2_plaintext += '00000'
            nb_complements += 1
        else:
            return None

    plaintext = base2_decoder(base2_plaintext)
    if nb_complements in base32_nb_complements:
        plaintext = plaintext[:base32_nb_complements.index(nb_complements) - len(base32_nb_complements)]
    return plaintext

# base64
base64_alphabet = string.ascii_uppercase + string.ascii_lowercase + string.digits + '+/'
base64_complement = '='
def base64_encoder(plaintext):
    cipher = ''
    base2_cipher = base2_encoder(plaintext)

    tokens = split_by_size(base2_cipher, 6)
    for token in tokens:
        if len(token) == 6:
            cipher += base64_alphabet[int(token, 2)]
        else:
            complement = base64_complement * ((6 - len(token)) // 2)
            token += base2_alphabet[0] * (len(complement) * 2)
            cipher += base64_alphabet[int(token, 2)]
            cipher += complement
    return cipher

def base64_decoder(cipher):
    base2_plaintext = ''
    cipher = cipher_padding(cipher)
    for c in cipher:
        if c in base64_alphabet:
            base2_plaintext += int_to_base(base64_alphabet.index(c), base2_alphabet, 6)
        elif c in base64_complement:
            base2_plaintext = base2_plaintext[:-2]
        else:
            return None

    plaintext = base2_decoder(base2_plaintext)
    return plaintext

# base tab
all_bases = [
    ['2',  'base2',  base2_encoder,  base2_decoder,  base2_alphabet,  None],
    ['16', 'base16', base16_encoder, base16_decoder, base16_alphabet, None],
    ['32', 'base32', base32_encoder, base32_decoder, base32_alphabet, base32_complement],
    ['64', 'base64', base64_encoder, base64_decoder, base64_alphabet, base64_complement]
]
ENCODER = 2
DECODER = 3
ALPHABET = 4
COMPLEMENT = 5

# get base functions
def get_base_data(base_name):
    for base in all_bases:
        if base_name == base[0] or base_name == base[1]:
            return base
    return None

# main encoder
def main_encoder(plaintext, bases, display):
    cipher = plaintext

    if display == 1:
        print('Plaintext: ' + plaintext)
        print()

    for base in bases:
        base_data = get_base_data(base)
        if base_data is None:
            if display == 1:
                print('Unknown base: ' + base + ' (ignored)')
            continue

        cipher = base_data[ENCODER](cipher)
        if cipher is None:
            if display == 1:
                print('Error while encoding in ' + base_data[1])
            return None

        if display == 1:
            print('Apply ' + base_data[1] + ': ' + cipher)

    if display == 1:
        print()
        print('Cipher: ' + cipher)
    else:
        print(cipher)
    return cipher

# main decoder
def main_decoder(cipher, bases, display):
    plaintext = cipher

    if display == 1:
        print('Cipher: ' + cipher)
        print()

    for base in bases:
        base_data = get_base_data(base)
        if base_data is None:
            if display == 1:
                print('Unknown base: ' + base + ' (ignored)')
            continue

        plaintext = base_data[DECODER](plaintext)
        if plaintext is None:
            if display == 1:
                print('Error while decoding in base ' + base_data[0] + ' (' + base_data[1] + ')')
            return None
        if display == 1:
            print('Apply ' + base_data[1] + ': ' + plaintext)

    if display == 1:
        print()
        print('Plaintext: ' + plaintext)
    else:
        print(plaintext)
    return plaintext

# main cracker
def main_cracker(cipher):
    if len(cipher) == 0:
        return ''
    for base_data in all_bases:
        if not is_base(cipher, base_data):
            print('Unknown char ' + base_data[1])
            continue
        try:
            plaintext = base_data[DECODER](cipher)
            print('Apply ' + base_data[1] + ': ' + plaintext)
            main_cracker(plaintext)
        except:
            print('Crash ' + base_data[1])

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
            main_encoder(args[1], bases, 1)
        elif args[0] == '-E':
            main_encoder(args[1], bases, 0)
        elif args[0] == '-d':
            main_decoder(args[1], bases, 1)
        elif args[0] == '-D':
            main_decoder(args[1], bases, 0)
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
    print('    ' + name + ' -e/-E plaintext base_names')
    print('     -> encode cipher in bases')
    print('         -e display details')
    print('         -E display only result')
    print('    ' + name + ' -d/-D plaintext base_names')
    print('     -> decode cipher from bases')
    print('         -d display details')
    print('         -D display only result')
    print()
    print('    base_names can be stacked and are applied in order (space and coma are delimiters)')
    print('    base_names supported are:')
    for base in all_bases:
        print('        ' + base[0] + '\talias ' + base[1])
    print()
    print('Exemple:')
    print('    $ ' + name + ' -E basecracker \'64 base16\'')
    print('      596d467a5a574e7959574e725a58493d')
    print('    $ ' + name + ' -D 596d467a5a574e7959574e725a58493d \'16,base64\'')
    print('      basecracker')
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