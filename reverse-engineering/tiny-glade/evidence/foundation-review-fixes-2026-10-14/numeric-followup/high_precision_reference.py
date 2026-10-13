"""Independent numerical reference; requires mpmath, never modifies fixtures."""
import struct
import mpmath as mp
mp.mp.dps = 80

def f(bits):
    return struct.unpack('<f', struct.pack('<I', bits))[0]

def bits(value):
    return struct.unpack('<I', struct.pack('<f', float(value)))[0]

t = f(1044372184)
scaled = f(bits(20 * t))
phase = f(bits(f(bits(scaled - 11.125)) * f(0x3fb2b8c3)))
exponent = f(bits(scaled - 10))
wave = f(bits(mp.sin(mp.mpf(phase))))
power = f(bits(mp.power(2, mp.mpf(exponent))))
result = bits(f(bits(wave * power)) * -0.5)
assert bits(wave) == 0x3f44c1da
assert bits(power) == 0x3c56d8cf
assert result == 3148161199
print(f'wave={bits(wave):#x}, power={bits(power):#x}, result={result}, Windows fixture=3148161200')
