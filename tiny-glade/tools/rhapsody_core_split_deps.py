#!/usr/bin/env python3
"""Dependency report for splitting backend-neutral code out of crates/rhapsody.

Prints, for non-test code only:
  1. per top-level module: lines, direct `vk::`/`ash::`/`gpu_allocator` uses,
     and `crate::<module>` references to other modules;
  2. for `rg` submodules: the transitive closure over references to backend
     modules, i.e. which submodules can move to a neutral crate as-is.

Items listed in MOVABLE are treated as already moved (the access vocabulary,
SubmitScratch, QueueClass, the CPU arena), matching step S2 of
crates/rhapsody/CORE_SPLIT_DESIGN.md. Re-run after each split step.

Heuristic: references are matched textually, inline `#[cfg(test)] mod` bodies
and `tests.rs`/`*_tests.rs` files are skipped. The compiler is the final word.
"""
import argparse
import collections
import os
import re

BACKEND = ('device', 'command', 'native', 'present', 'pipeline', 'transfer', 'bindless')
MOVABLE = re.compile(
    r'crate::native::(access_table|AccessScope|AccessSpec|is_write|resource_usage|submit::SubmitScratch)'
    r'|crate::device::(QueueClass|queue_config::QueueClass)'
    r'|crate::command::(cpu_arena|CpuArena)')
TEST_MOD = re.compile(r'#\[cfg\(test\)\]\s*\n\s*(pub\s+)?mod\s+\w+\s*\{')


def is_test_file(path):
    name = os.path.basename(path)
    return name == 'tests.rs' or name.endswith('_tests.rs')


def source(path):
    with open(path, encoding='utf-8', errors='ignore') as handle:
        text = handle.read()
    match = TEST_MOD.search(text)
    return text[:match.start()] if match else text


def rust_files(root):
    for directory, _, names in os.walk(root):
        for name in names:
            path = os.path.join(directory, name)
            if name.endswith('.rs') and not is_test_file(path):
                yield path


def module_report(src):
    modules = sorted(d for d in os.listdir(src) if os.path.isdir(os.path.join(src, d)))
    print(f"{'module':12s}{'loc':>7}{'vk':>6}{'ash':>5}{'ga':>4}  crate deps")
    for module in modules:
        loc = vk = ash = ga = 0
        deps = collections.Counter()
        for path in rust_files(os.path.join(src, module)):
            text = source(path)
            loc += text.count('\n') + 1
            vk += len(re.findall(r'\bvk::', text))
            ash += len(re.findall(r'\bash::', text))
            ga += text.count('gpu_allocator')
            deps.update(m for m in re.findall(r'crate::([a-z_]+)', text) if m != module and m in modules)
        print(f"{module:12s}{loc:7d}{vk:6d}{ash:5d}{ga:4d}  "
              + ', '.join(f'{k}:{v}' for k, v in deps.most_common()))


def rg_closure(src, treat_moved):
    rg = os.path.join(src, 'rg')
    subs = {}
    for entry in os.listdir(rg):
        name = entry[:-3] if entry.endswith('.rs') else entry
        if name in ('mod', 'test_image_copy'):
            continue
        full = os.path.join(rg, entry)
        subs[name] = [full] if entry.endswith('.rs') else list(rust_files(full))
    names = set(subs)
    internal = {name: set() for name in names}
    external = {name: collections.Counter() for name in names}
    for name, paths in subs.items():
        for path in paths:
            text = source(path)
            if treat_moved:
                text = MOVABLE.sub('', text)
            for ref in re.findall(r'crate::rg::([a-z_]+)', text) + re.findall(r'(?:super::)+([a-z_]+)', text):
                if ref in names and ref != name:
                    internal[name].add(ref)
            external[name].update(m for m in re.findall(r'crate::([a-z_]+)', text) if m in BACKEND)
    tainted = {name: 'direct' for name in names if external[name]}
    changed = True
    while changed:
        changed = False
        for name in names:
            if name not in tainted and internal[name] & set(tainted):
                tainted[name] = 'via ' + ','.join(sorted(internal[name] & set(tainted)))
                changed = True
    lines = {name: sum(source(p).count('\n') for p in subs[name]) for name in names}
    clean = sorted(names - set(tainted))
    print(f"\nrg submodules with a clean closure: {sum(lines[n] for n in clean)} lines")
    for name in clean:
        print(f'   {name:24s}{lines[name]:6d}')
    print(f"tainted: {sum(lines[n] for n in tainted)} lines")
    for name in sorted(tainted):
        print(f'   {name:24s}{lines[name]:6d}  {tainted[name]}  ext={dict(external[name])}')


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument('--src', default=os.path.join(os.path.dirname(__file__), '..', 'crates', 'rhapsody', 'src'))
    parser.add_argument('--no-moved', action='store_true', help='do not treat MOVABLE items as already moved')
    args = parser.parse_args()
    module_report(args.src)
    rg_closure(args.src, not args.no_moved)


if __name__ == '__main__':
    main()
