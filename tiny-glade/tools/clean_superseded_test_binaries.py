#!/usr/bin/env python3
"""Delete superseded copies of test/bench executables in target/<profile>/deps.

Every code change gives each test binary a new hash and a fresh multi-megabyte executable; the old ones are
never used again (cargo rebuilds what it needs) but pile up: ~95 rhapsody integration tests relinked a dozen
times filled a disk during the prism-gfx extraction. For each executable name the newest copy is kept and
the older ones are deleted. Dry run by default.

    python3 tools/clean_superseded_test_binaries.py [--target-dir target] [--profile debug] [--apply]
    CARGO_INCREMENTAL=0 cargo ...      # also avoids target/debug/incremental growing back (it reached 15 GB)

Only files named `<name>-<16 hex digits>` with no extension and at least --min-mb megabytes are considered,
so `.d`, `.rlib`, `.rmeta` and `.o` files are never touched. Nothing outside <target-dir>/<profile>/deps is read
or removed.
"""
import argparse
import collections
import os
import re
import sys

NAME = re.compile(r'(.+)-([0-9a-f]{16})')


def superseded(deps_dir, min_bytes):
    groups = collections.defaultdict(list)
    for entry in os.scandir(deps_dir):
        if '.' in entry.name or not entry.is_file(follow_symlinks=False):
            continue
        match = NAME.fullmatch(entry.name)
        if not match:
            continue
        stat = entry.stat(follow_symlinks=False)
        if stat.st_size >= min_bytes:
            groups[match.group(1)].append((stat.st_mtime, stat.st_size, entry.path))
    stale = []
    for items in groups.values():
        items.sort(reverse=True)
        stale.extend(items[1:])
    return len(groups), stale


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument('--target-dir', default=os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'target'))
    parser.add_argument('--profile', default='debug')
    parser.add_argument('--min-mb', type=float, default=5.0)
    parser.add_argument('--apply', action='store_true', help='actually delete (default: report only)')
    args = parser.parse_args(argv)
    deps = os.path.join(args.target_dir, args.profile, 'deps')
    if not os.path.isdir(deps):
        print(f'{deps} does not exist', file=sys.stderr)
        return 2
    distinct, stale = superseded(deps, int(args.min_mb * 1_000_000))
    total = sum(size for _, size, _ in stale)
    print(f'{distinct} distinct executables; {len(stale)} superseded copies, {total / 1e9:.2f} GB')
    if not args.apply:
        print('dry run; pass --apply to delete')
        return 0
    for _, _, path in stale:
        os.remove(path)
    print(f'deleted {len(stale)} files')
    return 0


if __name__ == '__main__':
    sys.exit(main())
