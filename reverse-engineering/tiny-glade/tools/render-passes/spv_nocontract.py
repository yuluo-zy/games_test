#!/usr/bin/env python3
"""Decorate every floating-point arithmetic result of a SPIR-V module with NoContraction.

lavapipe (LLVM) fuses FMul+FAdd into fma when the module allows it; the fused result differs from the
separately rounded one in the last bit. Two modules that compute the same algorithm but pair the
multiplies and adds differently (dxc re-association vs the original compiler) can then differ by a few
ulps. Injecting NoContraction into BOTH modules removes that degree of freedom: if the outputs are
bit-identical afterwards, the remaining difference was pure contraction, not a different algorithm.
Usage: spv_nocontract.py IN.spv OUT.spv   (prints how many results were decorated)
"""
import struct, sys

NO_CONTRACTION = 42
# OpFNegate, OpFAdd, OpFSub, OpFMul, OpFDiv, OpFRem, OpFMod, OpVectorTimesScalar, OpMatrixTimesScalar,
# OpVectorTimesMatrix, OpMatrixTimesVector, OpMatrixTimesMatrix, OpOuterProduct, OpDot
FP_ARITH = {127, 129, 131, 133, 136, 140, 141, 142, 143, 144, 145, 146, 147, 148}
TYPE_OR_GLOBAL = set(range(19, 40)) | {43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60}  # OpType*/OpConstant*/OpVariable...


def inject(data):
    w = list(struct.unpack('<%dI' % (len(data) // 4), data))
    ids, already, first_global, i = [], set(), None, 5
    while i < len(w):
        wc, op = w[i] >> 16, w[i] & 0xffff
        if op == 71 and w[i + 2] == NO_CONTRACTION:  # OpDecorate already present
            already.add(w[i + 1])
        if first_global is None and op in TYPE_OR_GLOBAL:
            first_global = i
        if op in FP_ARITH and w[i + 2] not in already:
            ids.append(w[i + 2])
        i += wc
    extra = []
    for r in ids:
        extra += [(3 << 16) | 71, r, NO_CONTRACTION]
    out = w[:first_global] + extra + w[first_global:]
    return struct.pack('<%dI' % len(out), *out), len(ids)


if __name__ == '__main__':
    d, n = inject(open(sys.argv[1], 'rb').read())
    open(sys.argv[2], 'wb').write(d)
    print('decorated', n)
