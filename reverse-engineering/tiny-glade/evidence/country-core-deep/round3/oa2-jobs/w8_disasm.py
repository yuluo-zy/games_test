#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Disassemble raw bins with capstone."""
import sys

try:
    from capstone import Cs, CS_ARCH_X86, CS_MODE_64
except Exception as e:
    print("capstone import fail:", repr(e))
    sys.exit(1)

for path, base in [("w8_from_cols.bin", 0x14089d500), ("w8_add_y.bin", 0x14089d550)]:
    print("=== %s ===" % path)
    b = open(path, "rb").read()
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    try:
        for i in md.disasm(b, base):
            print("0x%x: %s %s" % (i.address, i.mnemonic, i.op_str))
    except Exception as e:
        print("disasm fail:", repr(e))
