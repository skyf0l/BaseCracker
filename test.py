#!/usr/bin/python3

import unittest
import basecracker as bc

base2_expected = [
    ['qwerty', '011100010111011101100101011100100111010001111001'],
    ['123456 randouum!', '00110001001100100011001100110100001101010011011000100000011100100110000101101110011001000110111101110101011101010110110100100001']
]

base16_expected = [
    ['qwerty', '717765727479'],
    ['123456 randouum!', '3132333435362072616e646f75756d21']
]

base32_expected = [
    ['123456 randouum!', 'GEZDGNBVGYQHEYLOMRXXK5LNEE======'],
    ['a', 'ME======'],
    ['ab', 'MFRA===='],
    ['abc', 'MFRGG==='],
    ['abcd', 'MFRGGZA='],
    ['abcde', 'MFRGGZDF'],
    ['abcdef', 'MFRGGZDFMY======']
]

base58_expected = [
    ['123456 randouum!', '75M7MfQsAnc4wKJkVQdjwn'],
    ['a', '2g'],
    ['ab', '8Qq'],
    ['abc', 'ZiCa'],
    ['abcd', '3VNr6P'],
    ['abcde', 'BzFRgmr'],
    ['abcdef', 'qVgfxYy3']
]

base62_expected = [
    ['123456 randouum!', '1UpZ3kvt16KIrvCp8CtL4D'],
    ['a', '1Z'],
    ['ab', '6U6'],
    ['abc', 'QmIN'],
    ['abcd', '1mZPsa'],
    ['abcde', '7MYErOH'],
    ['abcdef', 'UP7NMSFq']
]

base64_expected = [
    ['123456 randouum!', 'MTIzNDU2IHJhbmRvdXVtIQ=='],
    ['a', 'YQ=='],
    ['ab', 'YWI='],
    ['abc', 'YWJj'],
    ['abcd', 'YWJjZA==']
]

base85_expected = [
    ['123456 randouum!', '0etOA2)ZRt@;]UoF`hOE'],
    ['a', '@/'],
    ['ab', '@:B'],
    ['abc', '@:E^'],
    ['abcd', '@:E_W'],
    ['abcde', '@:E_WAH'],
    ['abcdef', '@:E_WAS(']
]

class TestEncoderDecoder(unittest.TestCase):

    # test base2
    def test_base2_encoder(self):
        global base2_expected
        for expected in base2_expected:
            cipher = bc.base2_encoder(expected[0])
            self.assertEqual(cipher, expected[1])

    def test_base2_decoder(self):
        global base2_expected
        for expected in base2_expected:
            plaintext = bc.base2_decoder(expected[1])
            self.assertEqual(plaintext, expected[0])

    # test base16
    def test_base16_encoder(self):
        global base16_expected
        for expected in base16_expected:
            cipher = bc.base16_encoder(expected[0])
            self.assertEqual(cipher, expected[1])

    def test_base16_decoder(self):
        global base16_expected
        for expected in base16_expected:
            plaintext = bc.base16_decoder(expected[1])
            self.assertEqual(plaintext, expected[0])

    # test base32
    def test_base32_encoder(self):
        global base32_expected
        for expected in base32_expected:
            cipher = bc.base32_encoder(expected[0])
            self.assertEqual(cipher, expected[1])

    def test_base32_decoder(self):
        global base32_expected
        for expected in base32_expected:
            plaintext = bc.base32_decoder(expected[1])
            self.assertEqual(plaintext, expected[0])

    # test base58
    def test_base58_encoder(self):
        global base58_expected
        for expected in base58_expected:
            cipher = bc.base58_encoder(expected[0])
            self.assertEqual(cipher, expected[1])

    def test_base58_decoder(self):
        global base58_expected
        for expected in base58_expected:
            plaintext = bc.base58_decoder(expected[1])
            self.assertEqual(plaintext, expected[0])

    # test base62
    """
    def test_base62_encoder(self):
        global base62_expected
        for expected in base62_expected:
            cipher = bc.base62_encoder(expected[0])
            self.assertEqual(cipher, expected[1])

    def test_base62_decoder(self):
        global base62_expected
        for expected in base62_expected:
            plaintext = bc.base62_decoder(expected[1])
            self.assertEqual(plaintext, expected[0])
    """
    # test base64
    def test_base64_encoder(self):
        global base64_expected
        for expected in base64_expected:
            cipher = bc.base64_encoder(expected[0])
            self.assertEqual(cipher, expected[1])

    def test_base64_decoder(self):
        global base64_expected
        for expected in base64_expected:
            plaintext = bc.base64_decoder(expected[1])
            self.assertEqual(plaintext, expected[0])

    # test base85
    def test_base85_encoder(self):
        global base85_expected
        for expected in base85_expected:
            cipher = bc.base85_encoder(expected[0])
            self.assertEqual(cipher, expected[1])

    def test_base85_decoder(self):
        global base85_expected
        for expected in base85_expected:
            plaintext = bc.base85_decoder(expected[1])
            self.assertEqual(plaintext, expected[0])

if __name__ == '__main__':
    unittest.main()