#!/usr/bin/env python3
"""X1b follow-up: what descriptor layout does the generated MSL assume?

Reads the .metal files written by x1_msl_translate.py for one profile and reports how the
argument buffer is laid out: which descriptor sets exist, which `[[id(N)]]` values are used,
how many members overlap, and which descriptor kinds are declared. This is the evidence for the
heap-layout requirement the Metal backend has to meet (see crates/rhapsody/METAL_DESIGN.md D8).

    python3 tools/metal_experiments/x1/x1_msl_layout_check.py <translate_out>/<profile> [--markdown out.md]

It reads text only; it proves nothing about whether the Metal compiler accepts the MSL.
"""
import argparse
import collections
import glob
import os
import re
import sys

SET_STRUCT = re.compile(r'struct spvDescriptorSetBuffer(\d+)\s*\{(.*?)\};', re.S)
ID = re.compile(r'\[\[id\((\d+)\)\]\]')


def kind_of(line):
    if 'sampler>' in line:
        return 'sampler'
    if 'texture' in line or 'depth' in line:
        return 'texture'
    if 'device' in line:
        return 'buffer pointer'
    return 'other'


def analyse(directory):
    files = sorted(glob.glob(os.path.join(directory, '*', '*.metal')))
    sets = collections.Counter()
    ids = collections.Counter()
    members = collections.Counter()
    overlapping_files = 0
    kinds = collections.Counter()
    types_per_struct = collections.Counter()
    with_heap = 0
    for path in files:
        with open(path, encoding='utf-8', errors='ignore') as handle:
            text = handle.read()
        found = SET_STRUCT.findall(text)
        if not found:
            continue
        with_heap += 1
        if 'Overlapping binding' in text:
            overlapping_files += 1
        for set_index, body in found:
            sets[set_index] += 1
            lines = [line for line in body.split('\n') if line.strip()]
            real = [line for line in lines if not line.strip().startswith('//')]
            members[len(real)] += 1
            struct_kinds = set()
            for line in lines:
                for value in ID.findall(line):
                    ids[value] += 1
                kind = kind_of(line)
                kinds[kind] += 1
                struct_kinds.add(kind)
            types_per_struct[len(struct_kinds)] += 1
    return {
        'files': len(files), 'with_heap': with_heap, 'overlapping_files': overlapping_files,
        'sets': dict(sets), 'ids': dict(ids), 'real_members_per_struct': dict(members),
        'kinds': dict(kinds), 'kinds_per_struct': dict(types_per_struct),
    }


def render(result, directory):
    return '\n'.join([
        '# X1b generated-MSL layout check (generated)', '',
        f'Directory: `{directory}`', '',
        f"- MSL files: {result['files']}; with a descriptor-set argument buffer: {result['with_heap']}; "
        f"with overlapping members: {result['overlapping_files']}",
        f"- descriptor sets used (set -> structs): {result['sets']}",
        f"- `[[id(N)]]` values used (value -> declarations): {result['ids']}",
        f"- non-overlapping members per argument buffer struct: {result['real_members_per_struct']}",
        f"- descriptor kinds declared: {result['kinds']}",
        f"- distinct kinds in one struct (1..3) -> structs: {result['kinds_per_struct']}", '',
        'Reading: if every declaration uses id 0 and every struct has exactly one non-overlapping member, the '
        'translator treats the whole set as ONE flat table of descriptors that every kind indexes into.', ''])


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument('directory')
    parser.add_argument('--markdown')
    args = parser.parse_args()
    text = render(analyse(args.directory), args.directory)
    if args.markdown:
        with open(args.markdown, 'w', encoding='utf-8') as handle:
            handle.write(text)
    sys.stdout.write(text)


if __name__ == '__main__':
    main()
