#!/usr/bin/env python
"""Dump the decompiled_code field of a RE-MCP single-line JSON .c file to stdout.
Usage: python dump-decomp.py <file.c> [line_range]
Line range like '1-200' or '200-400'; omit for whole file.
Prints line-numbered C text with all \r stripped.
"""
import json, sys

def main():
    path = sys.argv[1]
    lo, hi = 0, None
    if len(sys.argv) > 2:
        rng = sys.argv[2]
        if '-' in rng:
            a, b = rng.split('-')
            lo = int(a) if a else 0
            hi = int(b) if b else None
        else:
            lo = int(rng); hi = lo
    raw = open(path, 'rb').read()
    text = raw.decode('utf-8', errors='replace')
    code = text
    try:
        obj = json.loads(text)
        if isinstance(obj, dict):
            code = obj.get('decompiled_code', '')
    except json.JSONDecodeError:
        pass  # plain C text with CRLF
    lines = code.replace('\r', '').split('\n')
    if hi is None:
        hi = len(lines)
    for i in range(lo - 1, min(hi, len(lines))):
        print(f"{i+1:6d}| {lines[i]}")

if __name__ == '__main__':
    main()
