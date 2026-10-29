#!/usr/bin/env python3
"""X3: which tests are touched when Vulkan-typed fields become neutral types.

For each watched Rust token (see TYPES) this counts, in test code only
(`tests/`, `*_tests.rs`, `tests.rs`, and the body of inline `#[cfg(test)] mod`
blocks): occurrences, files, and the `#[test]` functions that live in those
files. The per-file `#[test]` count is an upper bound on affected tests -- a
test function in a touched file may not use the token itself.

It also reports, per crate and per top-level module, how many test files and
`#[test]` functions are touched by the union of the type-family tokens, which
is the number the migration steps in crates/rhapsody/CORE_SPLIT_DESIGN.md need.

Text matching only; the compiler is the final authority. Usage:
    python3 tools/metal_experiments/x3_test_impact.py [--root <repo>] [--json out.json] [--markdown out.md]
"""
import argparse
import collections
import json
import os
import re
import sys

# token family -> regex. Families are what the neutral types replace.
TYPES = {
    'stage': r'vk::PipelineStageFlags|PipelineStageFlags',
    'access': r'vk::AccessFlags\b|\bAccessFlags\b',
    'layout': r'vk::ImageLayout|\bImageLayout\b',
    'aspect': r'vk::ImageAspectFlags|\bImageAspectFlags\b',
    'queue_family': r'QUEUE_FAMILY_IGNORED|queue_family|source_family|destination_family',
    'access_scope': r'\bAccessScope\b',
    'native_state': r'\bNativeState\b',
    'vk_result': r'vk::Result',
    'error_vulkan': r'Error::Vulkan|ErrorKind::Vulkan',
    'timeline_api': r'\bTimelineApi\b',
    'vk_format': r'vk::Format',
    'usage_flags': r'vk::(Buffer|Image)UsageFlags',
    'semaphore': r'vk::Semaphore',
    'command_buffer': r'vk::CommandBuffer',
}
# Families that CORE_SPLIT_DESIGN steps S1-S4 / P2 actually change (K1 aliases keep them compiling
# unchanged; this is the number of test files that would need edits WITHOUT the aliases).
CORE_SPLIT_FAMILIES = ('stage', 'access', 'layout', 'aspect', 'queue_family', 'access_scope',
                       'native_state', 'vk_result', 'error_vulkan', 'timeline_api', 'semaphore')
CRATES = ('rhapsody', 'country-core', 'tiny-glade')
TEST_MOD = re.compile(r'#\[cfg\(test\)\]\s*\n\s*(pub\s+)?mod\s+\w+\s*\{')
TEST_ATTR = re.compile(r'#\[(?:tokio::)?test\b')


def is_test_path(path):
    name = os.path.basename(path)
    parts = path.replace('\\', '/').split('/')
    return 'tests' in parts or name == 'tests.rs' or name.endswith('_tests.rs')


def test_body(path, text):
    """Return the part of the file that is test code, or None."""
    if is_test_path(path):
        return text
    match = TEST_MOD.search(text)
    return text[match.start():] if match else None


def rust_files(root):
    for directory, dirs, names in os.walk(root):
        dirs[:] = [d for d in dirs if d not in ('target', '.git', 'node_modules')]
        for name in names:
            if name.endswith('.rs'):
                yield os.path.join(directory, name)


def top_module(crate, rel):
    parts = rel.replace('\\', '/').split('/')
    if parts[0] == 'src' and len(parts) > 2:
        return parts[1]
    if parts[0] in ('tests', 'examples', 'benches'):
        return parts[0]
    return parts[0] if len(parts) == 1 else parts[0]


def analyse(root):
    compiled = {k: re.compile(v) for k, v in TYPES.items()}
    per_type = {k: {'occurrences': 0, 'files': set(), 'tests': 0} for k in TYPES}
    union_files = {}
    core_files = {}
    by_crate_module = collections.defaultdict(lambda: {'files': 0, 'tests': 0})
    total_tests = collections.Counter()
    for crate in CRATES:
        base = os.path.join(root, 'crates', crate)
        if not os.path.isdir(base):
            continue
        for path in rust_files(base):
            rel = os.path.relpath(path, base)
            with open(path, encoding='utf-8', errors='ignore') as handle:
                text = handle.read()
            body = test_body(rel, text)
            if body is None:
                continue
            tests = len(TEST_ATTR.findall(body))
            total_tests[crate] += tests
            hit = False
            for key, rx in compiled.items():
                count = len(rx.findall(body))
                if count:
                    hit = True
                    if key in CORE_SPLIT_FAMILIES:
                        core_files[(crate, rel)] = tests
                    per_type[key]['occurrences'] += count
                    per_type[key]['files'].add((crate, rel))
            if hit:
                union_files[(crate, rel)] = tests
    for key in per_type:
        per_type[key]['tests'] = sum(union_files.get(f, 0) for f in per_type[key]['files'])
    for (crate, rel), tests in union_files.items():
        slot = by_crate_module[(crate, top_module(crate, rel))]
        slot['files'] += 1
        slot['tests'] += tests
    return per_type, union_files, by_crate_module, total_tests, core_files


def render(per_type, union_files, by_crate_module, total_tests, core_files):
    lines = ['# X3 test impact (generated)', '',
             'Counts are test code only. "#[test] in files" is an upper bound: a test in a touched file may not use the token.', '',
             '| family | occurrences | files | #[test] in those files |', '| --- | ---: | ---: | ---: |']
    for key, data in per_type.items():
        lines.append(f"| {key} | {data['occurrences']} | {len(data['files'])} | {data['tests']} |")
    core_by_crate = collections.Counter()
    for (crate, _), tests in core_files.items():
        core_by_crate[crate] += tests
    lines += ['', f"Union of all families: {len(union_files)} files, {sum(union_files.values())} #[test] functions.",
              f"Union of the CORE_SPLIT families ({', '.join(CORE_SPLIT_FAMILIES)}): {len(core_files)} files, "
              f"{sum(core_files.values())} #[test] functions "
              f"({', '.join(f'{c} {core_by_crate[c]}' for c in CRATES)}).", '',
              'Total #[test] per crate: ' + ', '.join(f'{c} {total_tests[c]}' for c in CRATES), '',
              '| crate | module | touched files | #[test] in those files |', '| --- | --- | ---: | ---: |']
    for (crate, module), data in sorted(by_crate_module.items(), key=lambda kv: -kv[1]['tests']):
        lines.append(f"| {crate} | {module} | {data['files']} | {data['tests']} |")
    return '\n'.join(lines) + '\n'


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument('--root', default=os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..'))
    parser.add_argument('--json')
    parser.add_argument('--markdown')
    args = parser.parse_args()
    per_type, union_files, by_cm, total, core_files = analyse(os.path.abspath(args.root))
    markdown = render(per_type, union_files, by_cm, total, core_files)
    if args.json:
        payload = {
            'families': {k: {'occurrences': v['occurrences'], 'files': sorted(f'{c}/{p}' for c, p in v['files']),
                             'tests_in_files': v['tests']} for k, v in per_type.items()},
            'union': {f'{c}/{p}': t for (c, p), t in sorted(union_files.items())},
            'core_split_union': {f'{c}/{p}': t for (c, p), t in sorted(core_files.items())},
            'total_tests': dict(total),
        }
        with open(args.json, 'w', encoding='utf-8') as handle:
            json.dump(payload, handle, indent=1, ensure_ascii=False)
    if args.markdown:
        with open(args.markdown, 'w', encoding='utf-8') as handle:
            handle.write(markdown)
    sys.stdout.write(markdown)


if __name__ == '__main__':
    main()
