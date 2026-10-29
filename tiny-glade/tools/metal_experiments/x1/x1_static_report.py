#!/usr/bin/env python3
"""X1 static report: facts about the production-lowered SPIR-V, no translator needed.

Input: the directory written by
    cargo run -p shader-compiler --example x1_lower_all -- ... --out <dir>
Output (stdout or --markdown): per-capability stage counts and the shaders using the
risky ones, plus the binding-aliasing statistics that decide how hard the SPIRV-Cross
step (decision D8/D9 in crates/rhapsody/METAL_DESIGN.md) will be.

    python3 tools/metal_experiments/x1/x1_static_report.py <dir> [--markdown out.md]
"""
import argparse
import collections
import glob
import json
import os
import re
import sys

# Capabilities that need explicit attention on Metal, with why.
WATCH = {
    'PhysicalStorageBufferAddresses': 'buffer GPU addresses (D23); needs Metal 3',
    'Int64': '64-bit integers',
    'GroupNonUniformQuad': 'quad operations (incl. fragment stage)',
    'GroupNonUniformShuffle': 'simd shuffle',
    'GroupNonUniformVote': 'simd vote',
    'GroupNonUniformArithmetic': 'simd reductions',
    'GroupNonUniformBallot': 'simd ballot',
    'DemoteToHelperInvocationEXT': 'discard semantics (demote vs terminate)',
    'ComputeDerivativeGroupQuadsNV': 'compute-stage derivatives',
    'StorageImageWriteWithoutFormat': 'typeless storage writes',
    'StorageImageReadWithoutFormat': 'typeless storage reads',
    'StorageImageExtendedFormats': 'extended storage formats',
    'ShaderNonUniform': 'non-uniform resource indexing',
}


def capability(text):
    match = re.search(r'Capability\((\w+)\)', text)
    return match.group(1) if match else text


def load(directory):
    for path in sorted(glob.glob(os.path.join(directory, '*', 'meta.json'))):
        with open(path, encoding='utf-8') as handle:
            yield json.load(handle)


def report(directory):
    pipelines = list(load(directory))
    stages = 0
    caps = collections.Counter()
    who = collections.defaultdict(set)
    aliased_stages = 0
    alias_types = collections.Counter()
    alias_slots = collections.Counter()
    worst = []
    per_pipeline_alias = 0
    for meta in pipelines:
        pipeline_aliased = False
        for stage in meta['stages']:
            stages += 1
            lowered = stage['lowered']
            for raw in lowered.get('capabilities', []):
                cap = capability(raw)
                caps[cap] += 1
                if cap in WATCH:
                    who[cap].add(meta['file'])
            binding = lowered.get('binding_types', {})
            groups = binding.get('aliased', [])
            if groups:
                aliased_stages += 1
                pipeline_aliased = True
            for group in groups:
                count = len(group['types'])
                alias_types[count] += 1
                alias_slots[(group['set'], group['binding'])] += 1
                worst.append((count, meta['file'], stage['stage'], group['binding'], group['types']))
        per_pipeline_alias += pipeline_aliased
    worst.sort(key=lambda row: -row[0])
    out = [f'# X1 static report (generated)', '',
           f'Pipelines: {len(pipelines)}; lowered stages: {stages}; runtime-array limit as built.', '',
           '## Capabilities in the lowered SPIR-V', '',
           '| capability | stages | shaders | note |', '| --- | ---: | ---: | --- |']
    for cap, count in caps.most_common():
        out.append(f"| {cap} | {count} | {len(who[cap]) if cap in WATCH else '-'} | {WATCH.get(cap, '')} |")
    out += ['', '## Shaders using the watched capabilities', '']
    for cap in WATCH:
        if who[cap]:
            names = sorted(who[cap])
            out.append(f"- **{cap}** ({len(names)}): " + ', '.join(names[:40]) + (' …' if len(names) > 40 else ''))
    out += ['', '## Binding aliasing after the production lowering', '',
            'The lowering puts every descriptor array of one Vulkan descriptor type on one binding, so a '
            'binding carrying more than one resource type is an *aliased array*. SPIRV-Cross has to turn each '
            'such binding into valid MSL, which is the highest-risk step of decision D8.', '',
            f'- stages with at least one aliased binding: **{aliased_stages} / {stages}** '
            f'({100 * aliased_stages / max(stages, 1):.0f}%)',
            f'- pipelines with at least one aliased stage: {per_pipeline_alias} / {len(pipelines)}',
            '- aliased bindings by (set, binding): ' + ', '.join(f'{k}: {v}' for k, v in alias_slots.most_common()),
            '- distinct types on one aliased binding -> occurrences: '
            + ', '.join(f'{k}: {v}' for k, v in sorted(alias_types.items())), '',
            'Widest aliasing (first 8):', '']
    for count, file, stage, binding, types in worst[:8]:
        out.append(f'- {file} [{stage}] binding {binding}: {count} types')
        for typ in types:
            out.append(f'  - `{typ}`')
    return '\n'.join(out) + '\n'


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument('directory')
    parser.add_argument('--markdown')
    args = parser.parse_args()
    text = report(args.directory)
    if args.markdown:
        with open(args.markdown, 'w', encoding='utf-8') as handle:
            handle.write(text)
    sys.stdout.write(text)


if __name__ == '__main__':
    main()
