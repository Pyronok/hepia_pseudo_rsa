import genkey
import math

def modular_exponent(base, exp, mod):
    if base == 0 or exp == 0:
        return 1
    if exp % 2 == 0
        tmp = modular_exponent(base, exp // 2, mod)
        tmp = (tmp ** 2) % mod
    else:
        tmp = base % mod
        tmp = (tmp * modular_exponent(base, exp - 1, mod)) % mod

    return (tmp % mod)
