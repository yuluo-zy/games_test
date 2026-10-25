#!/usr/bin/env python3
"""Minimal xxh3_64 (seed=0, default secret) to verify SSBO key hashes."""
import sys

P64_1 = 0x9E3779B185EBCA87
P64_2 = 0xC2B2AE3D27D4EB4F
P64_3 = 0x165667B19E3779F9
P64_4 = 0x85EBCA77C2B2AE63
P64_5 = 0x27D4EB2F165667C5
MASK = (1 << 64) - 1

SECRET = bytes([
0xb8,0xfe,0x6c,0x39,0x23,0xa4,0x4b,0xbe,0x7c,0x01,0x81,0x2c,0xf7,0x21,0xad,0x1c,
0xde,0xd4,0x6d,0xe9,0x83,0x90,0x97,0xdb,0x72,0x40,0xa4,0xa4,0xb7,0xb3,0x67,0x1f,
0xcb,0x79,0xe6,0x4e,0xcc,0xc0,0xe5,0x78,0x82,0x5a,0xd0,0x7d,0xcc,0xff,0x72,0x21,
0xb8,0x08,0x46,0x74,0xf7,0x43,0x24,0x8e,0xe0,0x35,0x90,0xe6,0x81,0x3a,0x26,0x4c,
0x3c,0x28,0x52,0xbb,0x91,0xc3,0x00,0xcb,0x88,0xd0,0x65,0x8b,0x1b,0x53,0x2e,0xa3,
0x71,0x64,0x48,0x97,0xa2,0x0d,0xf9,0x4e,0x38,0x19,0xef,0x46,0xa9,0xde,0xac,0xd8,
0xa8,0xfa,0x76,0x3f,0xe3,0x9c,0x34,0x3f,0xf9,0xdc,0xbb,0xc7,0xc7,0x0b,0x4f,0x1d,
0x8a,0x51,0xe0,0x4b,0xcd,0xb4,0x59,0x31,0xc8,0x9f,0x7e,0xc9,0xd9,0x78,0x73,0x64,
0xea,0xc5,0xac,0x83,0x34,0xd3,0xeb,0xc3,0xc5,0x81,0xa0,0xff,0xfa,0x13,0x63,0xeb,
0x17,0x0d,0xdd,0x51,0xb7,0xf0,0xda,0x49,0xd3,0x16,0x55,0x26,0x29,0xd4,0x68,0x9e,
0x2b,0x16,0xbe,0x58,0x7d,0x47,0xa1,0xfc,0x8f,0xf8,0xb8,0xd1,0x7a,0xd0,0x31,0xce,
0x45,0xcb,0x3a,0x8f,0x95,0x16,0x04,0x28,0xaf,0xd7,0xfb,0xca,0xbb,0x4b,0x40,0x7e])

def rotl(x, r): return ((x << r) | (x >> (64 - r))) & MASK
def ror(x, r): return ((x >> r) | (x << (64 - r))) & MASK

def rnd(acc, input_, secret):
    a = (input_ ^ secret) & MASK
    return (acc + input_ + rotl(a * P64_2, 31) * P64_1) & MASK

def merge(acc, secret):
    a = rnd(acc & MASK, acc, secret)
    b = rnd(acc >> 32, acc, secret + 4)
    return (a ^ b) & MASK

def avalanche(h):
    h ^= h >> 37
    h = (h * 0x165667919E3779F9) & MASK
    h ^= h >> 32
    return h

def from_u64le(b): return int.from_bytes(b, 'little')
def to_u64le(x): return (x & MASK).to_bytes(8, 'little')

def short_round(acc, input_):
    return (acc + (input_ * P64_2) & MASK)

def short_mix(input_, length, secret, seed):
    h = 0
    if length >= 8:
        h = short_round(0, from_u64le(input_[:8]))
        h = (h ^ rotl(h, 42)) & MASK
        h = (h * P64_1) & MASK
        h ^= short_round(0, from_u64le(input_[length - 8:]))
        h = (h ^ rotl(h, 34)) & MASK
        h = (h * P64_2) & MASK
        h ^= short_round(0, from_u64le(input_[length // 2 - 4: length // 2 + 4]))
        h = (h ^ rotl(h, 24)) & MASK
        h = (h * P64_3) & MASK
        h ^= (from_u64le(secret) ^ (seed & MASK))
        h = (h ^ rotl(h, 26)) & MASK
        h = (h * P64_4) & MASK
        h = (h ^ rotl(h, 32)) & MASK
        h ^= (seed & MASK)
        return avalanche(h)
    if length >= 4:
        h ^= (from_u64le(input_[:4]) * P64_1) & MASK
        h = (h ^ rotl(h, 45)) & MASK
        h = (h * P64_2) & MASK
        h ^= (from_u64le(input_[length - 4:]) * P64_2) & MASK
        h = (h ^ rotl(h, 23)) & MASK
        h = (h * P64_3) & MASK
    if length > 0:
        h ^= (input_[0] * P64_5) & MASK
        h = (h ^ rotl(h, 25)) & MASK
        h = (h * P64_4) & MASK
    h ^= length
    return avalanche(h)

def medium_mix(acc, input_, secret, length):
    acc = (acc ^ (from_u64le(input_) * P64_2) & MASK) & MASK
    acc = (rotl(acc, 27) + P64_1) & MASK
    acc = (acc ^ (from_u64le(input_[8:]) * P64_1) & MASK) & MASK
    acc = (rotl(acc, 31) + P64_2) & MASK
    acc = (acc ^ (from_u64le(secret) + from_u64le(input_[16:]) * P64_3) & MASK) & MASK
    acc = (rotl(acc, 23) + P64_3) & MASK
    acc = (acc ^ (from_u64le(secret[8:]) + from_u64le(input_[length - 16:]) * P64_4) & MASK) & MASK
    acc = (rotl(acc, 33) + P64_4) & MASK
    acc = (acc ^ (from_u64le(secret[16:]) + from_u64le(input_[length - 8:]) * P64_5) & MASK) & MASK
    acc = (rotl(acc, 11) + P64_5) & MASK
    return acc

def xxh3_64(data, seed=0):
    data = bytes(data)
    length = len(data)
    if length <= 16:
        return short_mix(data, length, SECRET, seed)
    # 17..128
    acc = (length * P64_1) & MASK
    acc = (acc + ((seed ^ P64_5) & MASK)) & MASK
    n = (length - 1) // 32
    for i in range(n + 1):
        acc = medium_mix(acc, data[i * 32:], SECRET, length)
    acc = (acc ^ ((seed & MASK) ^ (SECRET[7] & 0xFF))) & MASK
    return avalanche(acc)

if __name__ == '__main__':
    for name in sys.argv[1:]:
        h = xxh3_64(name.encode('utf-8'))
        print(f'{name}\n  -> {h:#018x} ({h})')
