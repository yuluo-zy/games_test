#!/usr/bin/env python3
"""Record and compare per-test results of `cargo test`, so "test results unchanged" is checkable.

    # record: runs the tests (no-fail-fast, lavapipe) and writes one row per test
    python3 tools/rhapsody_test_baseline.py record --out crates/rhapsody/baseline/test_results.tsv [-p rhapsody] [-p prism-gfx]

    # parse an existing cargo output instead of running (for tests / offline analysis)
    python3 tools/rhapsody_test_baseline.py record --from-log run.txt --out results.tsv

    # compare two recordings; exit status 1 if anything regressed or changed
    python3 tools/rhapsody_test_baseline.py compare baseline.tsv current.tsv [--alias prism_gfx=rhapsody]

When code moves between packages its unit tests keep their module path but run in another test binary.
`--alias NEW=OLD` renames binary NEW to OLD on the `current` side before comparing (and the same for the
`doctest:` binaries), so a moved test is matched with its old self. If the alias makes two current tests
collide on one key, that is an error, never a silent merge.

Rows are `binary<TAB>test<TAB>result` with result ok | FAILED | ignored. `cargo test` stops at the first
failing test binary unless `--no-fail-fast` is given, which `record` always passes; a baseline recorded
without it silently omits every binary after the first failure.

`compare` reports: tests that now fail but passed before (regressions), tests that failed before and
pass now (fixed), tests added, tests removed, and ok<->ignored flips. A removed test counts as a change:
a test that disappears would otherwise look like "no regression". Known failures that are already in the
baseline are not regressions; they are listed so they stay visible.
"""
import argparse
import os
import re
import subprocess
import sys

RUNNING_ANY = re.compile(r'^\s+Running (.+?)(?: \((.+?)\))?\s*$')
DOCTEST = re.compile(r'^\s+Doc-tests (\S+)')
TEST = re.compile(r'^test (.+?) \.\.\. (ok|FAILED|ignored)(?:, .*)?$')


def binary_name(path, detail):
    """Stable name for a test binary: the deps name without its hash, else the source path."""
    if detail:
        match = re.search(r'deps/([A-Za-z0-9_\-]+?)-[0-9a-f]{8,}', detail)
        if match:
            return match.group(1)
    return path


def parse(text):
    rows = []
    binary = '?'
    for line in text.splitlines():
        doc = DOCTEST.match(line)
        if doc:
            binary = 'doctest:' + doc.group(1)
            continue
        run = RUNNING_ANY.match(line)
        if run:
            binary = binary_name(run.group(1), run.group(2))
            continue
        test = TEST.match(line)
        if test:
            rows.append((binary, test.group(1), test.group(2)))
    return sorted(rows)


def write_tsv(rows, path):
    os.makedirs(os.path.dirname(os.path.abspath(path)), exist_ok=True)
    with open(path, 'w', encoding='utf-8') as handle:
        handle.write('binary\ttest\tresult\n')
        for row in rows:
            handle.write('\t'.join(row) + '\n')


def read_tsv(path):
    with open(path, encoding='utf-8') as handle:
        lines = handle.read().splitlines()[1:]
    result = {}
    for line in lines:
        binary, test, outcome = line.split('\t')
        result[(binary, test)] = outcome
    return result


def apply_aliases(table, aliases):
    """Rename binaries on one side. Raises ValueError if two rows would share a key afterwards."""
    if not aliases:
        return table
    mapping = {}
    for new, old in aliases:
        mapping[new] = old
        mapping['doctest:' + new] = 'doctest:' + old
    result = {}
    for (binary, test), outcome in table.items():
        key = (mapping.get(binary, binary), test)
        if key in result:
            raise ValueError(f'alias makes two tests collide on {key[0]} :: {key[1]}')
        result[key] = outcome
    return result


def compare(before, after):
    keys_before, keys_after = set(before), set(after)
    regress = sorted(k for k in keys_before & keys_after if before[k] == 'ok' and after[k] == 'FAILED')
    fixed = sorted(k for k in keys_before & keys_after if before[k] == 'FAILED' and after[k] == 'ok')
    flips = sorted(k for k in keys_before & keys_after
                   if {before[k], after[k]} == {'ok', 'ignored'})
    still_failing = sorted(k for k in keys_before & keys_after if before[k] == 'FAILED' and after[k] == 'FAILED')
    added = sorted(keys_after - keys_before)
    removed = sorted(keys_before - keys_after)
    return {'regress': regress, 'fixed': fixed, 'flips': flips, 'still_failing': still_failing,
            'added': added, 'removed': removed}


def summarise(rows):
    count = {'ok': 0, 'FAILED': 0, 'ignored': 0}
    for _, _, result in rows:
        count[result] += 1
    return count


def cmd_record(args):
    if args.from_log:
        with open(args.from_log, encoding='utf-8', errors='ignore') as handle:
            text = handle.read()
    else:
        env = dict(os.environ)
        env.setdefault('VK_ICD_FILENAMES', '/usr/share/vulkan/icd.d/lvp_icd.json')
        packages = [flag for package in (args.package or ['rhapsody']) for flag in ('-p', package)]
        command = ['cargo', 'test', *packages, '--all-features', '--offline', '--no-fail-fast']
        # One merged stream: cargo prints `Running <binary>` on stderr and the test lines on stdout, and the
        # parser attributes each test to the most recent `Running` line, so the interleaving must survive.
        proc = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, env=env, cwd=args.cwd)
        text = proc.stdout
        if 'could not compile' in text:
            print('build failed; not recording', file=sys.stderr)
            return 2
    rows = parse(text)
    if not rows:
        print('no test results found', file=sys.stderr)
        return 2
    write_tsv(rows, args.out)
    print(f'{len(rows)} tests {summarise(rows)} -> {args.out}')
    return 0


def cmd_compare(args):
    aliases = []
    for item in args.alias:
        if '=' not in item:
            print(f'bad --alias {item!r}, expected NEW=OLD', file=sys.stderr)
            return 2
        aliases.append(tuple(item.split('=', 1)))
    try:
        after = apply_aliases(read_tsv(args.after), aliases)
    except ValueError as error:
        print(error, file=sys.stderr)
        return 2
    result = compare(read_tsv(args.before), after)
    label = {'regress': 'REGRESSED (ok -> FAILED)', 'fixed': 'fixed (FAILED -> ok)',
             'flips': 'ok <-> ignored', 'still_failing': 'still failing (in baseline)',
             'added': 'added', 'removed': 'REMOVED'}
    for key in ('regress', 'removed', 'flips', 'fixed', 'added', 'still_failing'):
        items = result[key]
        print(f'{label[key]}: {len(items)}')
        for item in items[:args.limit]:
            print('   ', ' :: '.join(item))
        if len(items) > args.limit:
            print(f'    ... {len(items) - args.limit} more')
    bad = result['regress'] or result['removed'] or result['flips']
    print('RESULT:', 'CHANGED' if bad else 'no regressions, none removed')
    return 1 if bad else 0


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    sub = parser.add_subparsers(dest='command', required=True)
    rec = sub.add_parser('record')
    rec.add_argument('--out', required=True)
    rec.add_argument('-p', '--package', action='append', help='package to test; repeatable (default: rhapsody)')
    rec.add_argument('--from-log')
    rec.add_argument('--cwd', default=os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
    rec.set_defaults(func=cmd_record)
    cmp_ = sub.add_parser('compare')
    cmp_.add_argument('before')
    cmp_.add_argument('after')
    cmp_.add_argument('--limit', type=int, default=20)
    cmp_.add_argument('--alias', action='append', default=[], help='NEW=OLD binary rename for the current side')
    cmp_.set_defaults(func=cmd_compare)
    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == '__main__':
    sys.exit(main())
