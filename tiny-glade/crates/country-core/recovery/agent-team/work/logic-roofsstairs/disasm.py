#!/usr/bin/env python3
"""Disassemble a memory dump of a function and resolve RIP-relative f32 constants."""
import json, sys
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import X86_OP_MEM, X86_OP_IMM

def main(addr_hex, data_list, base_hex):
    base = int(base_hex, 16)
    code = bytes(data_list)
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    md.detail = True
    # read 4 bytes at absolute addresses
    memory = bytearray(0x1000000)  # fake big space, we only resolve the immediate float pool within function
    # constants loaded via RIP-relative refer to addresses outside the function; resolve from the hex dump float constants
    print(f"function base 0x{base:x} len {len(code)}")
    for insn in md.disasm(code, base):
        extra = ""
        for op in insn.operands:
            if op.type == X86_OP_MEM and op.mem.base == 0 and op.mem.disp != 0:
                # direct addressing with displacement (rip-relative shows base=rip in capstone)
                pass
        if insn.mnemonic.startswith(("movss", "mov", "lea")):
            ops = insn.op_str
            # capstone shows rip-relative as [rip + 0x...]
            if "rip" in ops:
                extra = "  <- RIP-REL"
        print(f"0x{insn.address:x}: {insn.mnemonic}\t{insn.op_str}{extra}")

if __name__ == "__main__":
    main(sys.argv[1], json.loads(sys.argv[2]), sys.argv[1])
