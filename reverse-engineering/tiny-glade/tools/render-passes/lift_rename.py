#!/usr/bin/env python3
"""Mechanical, word-boundary variable rename inside a file's lifted sections.

Guarantees arithmetic/control-flow preservation: only identifiers change.
Usage:
    lift_rename.py <src.hlsl> <dst.hlsl> \
        --rename "section_start:section_end:_89=uv,_92=uv_aspect,..." \
        [--rename ...]
Sections are 1-based inclusive line ranges; a rename map applies only within
that range. Multiple --rename options may target different sections.
"""
import argparse
import re


def apply_rename(src_text, start, end, rename_map):
    lines = src_text.split('\n')
    if start > len(lines) or end > len(lines):
        raise SystemExit('section %d..%d out of range (%d lines)' % (start, end, len(lines)))
    pat = re.compile(r'\b(%s)\b' % '|'.join(map(re.escape, sorted(rename_map, key=len, reverse=True))))
    for i in range(start - 1, end):
        lines[i] = pat.sub(lambda m: rename_map[m.group(1)], lines[i])
    return '\n'.join(lines)


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('src')
    ap.add_argument('dst')
    ap.add_argument('--rename', action='append', default=[],
                    help='section_start:section_end:_old1=new1,_old2=new2')
    ap.add_argument('--catchall', action='append', default=[],
                    help='section_start:section_end:prefix  (any remaining _NNN -> prefixNNN)')
    args = ap.parse_args()

    text = open(args.src).read()
    for spec in args.rename:
        start_s, _, rest = spec.partition(':')
        end_s, _, body = rest.partition(':')
        rm = dict(pair.split('=', 1) for pair in body.split(',') if '=' in pair)
        text = apply_rename(text, int(start_s), int(end_s), rm)
    for spec in args.catchall:
        start_s, _, rest = spec.partition(':')
        end_s, _, prefix = rest.partition(':')
        rm = {}
        m = re.compile(r'\b_(\d+)\b')
        lines = text.split('\n')
        for i in range(int(start_s) - 1, int(end_s)):
            for num in set(m.findall(lines[i])):
                rm.setdefault('_%s' % num, '%s%s' % (prefix, num))
        text = apply_rename(text, int(start_s), int(end_s), rm)
    open(args.dst, 'w').write(text)
    print('wrote %s' % args.dst)


if __name__ == '__main__':
    main()
