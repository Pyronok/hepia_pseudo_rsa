#!/usr/bin/env python
# -*- coding: utf-8 -*-

import rabin_miller
import math

def gcd_euclid(a, b):
    while b != 0:
        f = b
        b = a % b
        a = f

    return a

def lcm_euclid(a, b):
    return a * b / gcd_euclid(a, b)

def ext_euclid(a, b):
    z0, z1, zz0, zz1 = 1, 0, 0, 1

    while b != 0:
        q, a, b = a // b, b, a % b
        z0, z1 = z1, z0 - q * z1
        zz0, zz1 = zz1, zz0 - q * zz1

    return a, z0, zz0

def mult_inv_mod(b, n):
    x, y, z = ext_euclid(b, n)
    if x == 1:
        return y % n

def pick_e(lambda_n):
    # TODO: Not PKCS-compliant!
    while True:
        candidate = rabin_miller.generatePrimeBetween(2**14,2**16)
        if gcd_euclid(candidate, lambda_n) == 1:
            return candidate


def get_private_key(e, phi):
    a, d, c = ext_euclid(e, phi)
    return {'e' : e, 'd' : d % phi}

def gen_keys(size):
    p = rabin_miller.generateLargePrime(size)
    q = rabin_miller.generateLargePrime(size)

    n = p * q

    phi = (p - 1) * (q - 1)
    lambda_n = lcm_euclid(p - 1, q - 1)

    e = pick_e(lambda_n)

    print e

    return {'p' : p, 'q' : q, 'phi' : phi, 'n' : n, 'e' : e}
