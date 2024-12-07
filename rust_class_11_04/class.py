import random
import math
import hashlib

die = random.SystemRandom() # Secure random generator
##################
## Miller Rabin ##
##################

'''
Probabilistic primarly test
'''
def single_test(n, a):
    exp = n - 1
    while not exp & 1:
        exp >>= 1

    if pow(a, exp, n) == 1:
        return True

    while exp < n - 1:
        if pow(a, exp, n) == n - 1:
            return True
        exp <<= 1
    return False


def miller_rabin(n, k=40):
    for i in range(k):
        a = die.randrange(2, n-1)
        if not single_test(n, a):
            return False
    return True

'''
Generate prime number randomly
'''
def gen_prime(bits=1024):
    mask = (1 << (bits - 1)) | 1
    while True:
        p = die.getrandbits(bits)
        p |= mask
        if miller_rabin(p, 40):
            return p

#########
## RSA ##
#########
'''
Ref: https://gist.github.com/JekaDeka/c9b0f5da16625e3c7bd1033356354579
​
Euclid's extended algorithm for finding the multiplicative inverse of two numbers
'''
def extended_euclidean_algorithm(a, b):
    """
    Returns a three-tuple (gcd, x, y) such that
    a * x + b * y == gcd, where gcd is the greatest
    common divisor of a and b.
​
    This function implements the extended Euclidean
    algorithm and runs in O(log b) in the worst case.
    """
    s, old_s = 0, 1
    t, old_t = 1, 0
    r, old_r = b, a

    while r != 0:
        quotient = old_r // r
        old_r, r = r, old_r - quotient * r
        old_s, s = s, old_s - quotient * s
        old_t, t = t, old_t - quotient * t

    return old_r, old_s, old_t


def is_coprime(x, y):
    return math.gcd(x,y) == 1


class KeyPair:
    def __init__(self, n, e, d):
        self.n = n # Product of two big primes
        self.e = e # Public Key (encryption)
        self.d = d # Private Key (decryption)

    def encrypt(self, message):
        if isinstance(message, (bytes, bytearray)):
            message = int.from_bytes(message, byteorder='big')
        assert message < self.n
        return pow(message, self.e, self.n)

    def decrypt(self, encrypted):
        if isinstance(encrypted, (bytes, bytearray)):
            encrypted = int.from_bytes(encrypted, byteorder='big')
        assert encrypted < self.n
        return pow(encrypted, self.d, self.n)

    def sign_message(self, message):
        assert isinstance(message, (bytes, bytearray))
        message_hash = int.from_bytes(hashlib.sha256(message).digest(), byteorder='big')
        return pow(message_hash, self.d, self.n)

    def validate_signature(self, message, signature):
        assert isinstance(message, (bytes, bytearray))
        message_hash = int.from_bytes(hashlib.sha256(message).digest(), byteorder='big')
        decrypted = pow(signature, self.e, self.n)
        return decrypted == message_hash

'''
Create a new RSA Key Pair
'''
def new_keypair(bits = 1024):
    # p and q are two big prime numbers generated randomly
    p = gen_prime(bits)
    q = gen_prime(bits)

    n = p * q
    qn = n - p - q + 1
    e = 65537  # ref: https://en.wikipedia.org/wiki/65,537
    while not is_coprime(e, qn):
        e = die.randrange(3, qn - 1)

    (_, d, _) = extended_euclidean_algorithm(e, qn)
    d = d % qn  # mod qn in case D is negative
    return KeyPair(n, e, d)

################
## Encryption ##
################
keypair = KeyPair(0x96419608bf7210d5, 0x10001, 0x145a2be52aa7b6e1)
print(f"e = {hex(keypair.e)}")
print(f"d = {hex(keypair.d)}")
print(f"n = {hex(keypair.n)}")
bloco = 0xdcd884e68602ee03a538e57a3a1065c241e588e9224aa6e5cf42775043bdaa85
exit(0)
