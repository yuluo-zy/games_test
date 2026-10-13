"""验证 AssetSsboLibrary 的 SsboId = xxh3_64(T::NAME 短名)。
只实现 XXH3 的 17..128 字节分支（短名足够），secret 直接从原 EXE 读取（NameHash::compute 0x140c9cd80 传入 0x142b2ebb0）。
期望值来自 get_rg_buffer<Cobblestone3RasterStagingSsbo> 与 add_new_with_data<NaniNextVisibilityBitmaskSsbo> 里的立即数。
用法: python3 -I verify_ssbo_id.py"""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rdva import rd

SEC = rd(0x142b2ebb0, 192)
M = (1 << 64) - 1


def r64(b, o):
    return int.from_bytes(b[o:o + 8], 'little')


def fold(a, b):
    p = a * b
    return (p & M) ^ (p >> 64)


def mix16(i, o, so):
    return fold(r64(i, o) ^ r64(SEC, so), r64(i, o + 8) ^ r64(SEC, so + 8))


def xxh3_17_128(i):
    n = len(i)
    assert 17 <= n <= 128
    acc = (n * 0x9E3779B185EBCA87) & M
    if n > 32:
        if n > 64:
            if n > 96:
                acc += mix16(i, 48, 96)
                acc += mix16(i, n - 64, 112)
            acc += mix16(i, 32, 64)
            acc += mix16(i, n - 48, 80)
        acc += mix16(i, 16, 32)
        acc += mix16(i, n - 32, 48)
    acc += mix16(i, 0, 0)
    acc += mix16(i, n - 16, 16)
    acc &= M
    acc ^= acc >> 37
    acc = (acc * 0x165667919E3779F9) & M
    acc ^= acc >> 32
    return acc


if __name__ == '__main__':
    cases = [(b'Cobblestone3RasterStagingSsbo', 0x802ff23dee42ab47), (b'NaniNextVisibilityBitmaskSsbo', 0x69cdc275a605bad6)]
    ok = True
    for name, want in cases:
        got = xxh3_17_128(name)
        print(name.decode(), hex(got), 'OK' if got == want else f'MISMATCH want {hex(want)}')
        ok &= got == want
    sys.exit(0 if ok else 1)
