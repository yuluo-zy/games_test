"""Audit recovered small-library dependency identities against original B Cargo.lock."""
import argparse
import hashlib
import json
from pathlib import Path
import tomllib

ROOT = Path(__file__).resolve().parents[3]
ROOTS = ('theme-fetcher','theme-server-proto','feedback-reporter','geforcenow','tg-iced','tg-integrity-report','crash-reporter-gui','tg-gpu-selector')

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def source_identity(source):
    if source and source.startswith('git+'):
        return source.split('?', 1)[0].split('#', 1)[0], source.rsplit('#', 1)[-1]
    return source

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    original_path = ROOT / 'tracking/original-Cargo.lock'
    current_path = ROOT / 'reconstruction/shared-foundation/Cargo.lock'
    original = tomllib.loads(original_path.read_text(encoding='utf-8'))['package']
    current = tomllib.loads(current_path.read_text(encoding='utf-8'))['package']
    by_name = {}
    for row in current:
        by_name.setdefault(row['name'], []).append(row)
    ambiguities = []
    def resolve(dep):
        parts = dep.split(' (', 1)[0].split()
        rows = by_name.get(parts[0], [])
        if len(parts) > 1:
            rows = [p for p in rows if p['version'] == parts[1]]
        if len(rows) != 1:
            ambiguities.append({'dependency': dep, 'candidates': len(rows)})
            return None
        return rows[0]
    def key(row):
        return row['name'], row['version'], row.get('source', '')
    seen, pending, missing_roots = {}, [], []
    for name in ROOTS:
        rows = by_name.get(name, [])
        if len(rows) != 1:
            missing_roots.append(name)
        else:
            pending.extend(rows)
    while pending:
        row = pending.pop()
        if key(row) in seen:
            continue
        seen[key(row)] = row
        for dep in row.get('dependencies', []):
            resolved = resolve(dep)
            if resolved is not None:
                pending.append(resolved)
    records, mismatches = [], []
    for row in sorted(seen.values(), key=key):
        if not row.get('source'):
            records.append({'name': row['name'], 'version': row['version'], 'kind': 'local_recovery',
                            'original_version_observed': any(o['name'] == row['name'] and o['version'] == row['version'] for o in original)})
            continue
        matches = [o for o in original if o['name'] == row['name'] and o['version'] == row['version']
                   and source_identity(o.get('source')) == source_identity(row.get('source'))]
        same = len(matches) == 1 and matches[0].get('checksum') == row.get('checksum')
        record = {'name': row['name'], 'version': row['version'], 'source': row['source'],
                  'checksum': row.get('checksum'), 'original_identity_match': same,
                  'exact_source_text_match': bool(matches and matches[0].get('source') == row['source'])}
        records.append(record)
        if not same:
            mismatches.append(record)
    report = {'schema_version': 1, 'baseline': 'B', 'roots': ROOTS,
              'original_lock_sha256': sha(original_path), 'current_lock_sha256': sha(current_path),
              'scope': 'Public dependency closure of eight B foundation libraries; dependency code is reused.',
              'git_query_policy': 'Same repository and exact commit define identity; source text differences stay visible.',
              'packages': records, 'public_packages': sum('source' in r for r in records),
              'missing_roots': missing_roots, 'ambiguous_dependencies': ambiguities, 'mismatches': mismatches,
              'ok': not missing_roots and not ambiguities and not mismatches}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps({'ok': report['ok'], 'packages': len(records), 'public_packages': report['public_packages'],
                      'mismatches': len(mismatches), 'missing_roots': missing_roots, 'ambiguities': len(ambiguities)}))
    raise SystemExit(0 if report['ok'] else 1)

if __name__ == '__main__':
    main()
