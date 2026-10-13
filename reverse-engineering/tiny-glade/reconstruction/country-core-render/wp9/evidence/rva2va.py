#!/usr/bin/env python3
"""Convert rva->va for B patched exe (image base 0x140000000) and print a VA list for ghidra batch jobs.
Input: pairs "name:rva" one per line on stdin, or args as name=rva.
"""
import sys

BASE = 0x140000000

def main():
    entries = []
    for arg in sys.argv[1:]:
        name, _, r = arg.partition('=')
        rva = int(r, 16)
        va = BASE + rva
        entries.append((name, rva, va))
    for name, rva, va in entries:
        print(f"{name} rva=0x{rva:x} va=0x{va:x}")

if __name__ == '__main__':
    main()
