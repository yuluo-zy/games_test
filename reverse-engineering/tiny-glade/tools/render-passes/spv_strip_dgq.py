#!/usr/bin/env python3
"""Strip DerivativeGroupQuadsKHR (ExecutionMode 5289) and ComputeDerivativeGroupQuadsKHR (Capability 5088)
from a SPIR-V module.

dxc >= 1.9 adds them to every compute shader that uses QuadRead*/quad ops. They redefine which four
invocations form a quad (2x2 block of local x/y). The original shaders were compiled without the mode and
build their quads from gl_LocalInvocationIndex themselves (morton decode), so the extra mode changes the
result (seen in dof/analyze: oracle matches an independent model, the dxc 1.9 build does not). Stripping the
mode restores the original behaviour. Usage: spv_strip_dgq.py IN.spv OUT.spv  (prints how many instructions were dropped)
"""
import struct, sys


def strip(data):
    w = list(struct.unpack('<%dI' % (len(data) // 4), data))
    out, i, dropped = w[:5], 5, 0
    while i < len(w):
        wc, op = w[i] >> 16, w[i] & 0xffff
        if (op == 16 and w[i + 2] == 5289) or (op == 17 and w[i + 1] == 5088):  # OpExecutionMode / OpCapability
            dropped += 1
        else:
            out += w[i:i + wc]
        i += wc
    return struct.pack('<%dI' % len(out), *out), dropped


if __name__ == '__main__':
    data, n = strip(open(sys.argv[1], 'rb').read())
    open(sys.argv[2], 'wb').write(data)
    print('dropped', n)
