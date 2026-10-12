"""Validate generated Cargo dependency identities and offline workspace parsing."""
from collections import Counter, defaultdict
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tomllib
from urllib.parse import urlsplit, urlunsplit

ROOT = Path(__file__).resolve().parents[1]
PROJECT = ROOT.parent

def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def key(p): return (p['name'], p.get('version'), p.get('source', 'local_or_path'))

def main():
    checks = []; reports = []; workspace_checks = []
    scope = ROOT / 'dependency-analysis-scope.json'
    versions = sys.argv[1:] or (json.loads(scope.read_text())['active_versions'] if scope.exists() else ['B', 'C'])
    for v in versions:
        report = json.loads((ROOT / '_cargo' / f'{v}-report.json').read_text()); reports.append(report)
        lock = json.loads((PROJECT / 'evidence/multi-version' / f'{v}-dependencies.json').read_text())
        packages = defaultdict(list)
        for p in lock['packages']: packages[p['name']].append(p)
        def resolve(token):
            parts = token.split(); rows = packages[parts[0]]
            if len(parts) > 1: rows = [p for p in rows if p['version'] == parts[1]]
            if len(parts) > 2: rows = [p for p in rows if p.get('source') == ' '.join(parts[2:]).strip('()')]
            return rows[0] if len(rows) == 1 else None
        count = 0
        for entry in report['manifests']:
            path = ROOT / entry['path']; parsed = tomllib.loads(path.read_text())
            changed_existing = sha(path) != entry['manifest_sha256']
            if changed_existing: assert entry.get('reused_existing_manifest'), 'Generated manifest changed unexpectedly'
            assert parsed['package']['name'] == entry['package_name']
            version = entry['package_identity'].get('version')
            if version: assert parsed['package']['version'] == version
            else: assert 'version' not in parsed['package']
            assert len(parsed.get('dependencies', {})) == entry['active_dependency_count']
            actual = Counter()
            for dep in entry['active_dependencies']:
                fields = dep['manifest_spec']; pkg = dep['package']
                current = parsed['dependencies'][dep['key']]
                if isinstance(current, dict) and current.get('workspace'):
                    workspace = tomllib.loads((ROOT / entry['independent_workspace'] / 'Cargo.toml').read_text())
                    current = workspace['workspace']['dependencies'][dep['key']]
                if isinstance(current, str): current = {'version': current}
                assert current == fields
                if pkg.get('version'): assert fields['version'] == '=' + pkg['version']
                elif v == 'A': assert 'version' not in fields and 'path' in fields
                if 'git' in fields:
                    src = urlsplit(pkg['source'][4:])
                    assert fields['git'] == urlunsplit((src.scheme, src.netloc, src.path, '', ''))
                    assert fields['rev'] == src.fragment and len(fields['rev']) == 40
                if 'path' in fields:
                    base = ROOT / entry['independent_workspace'] if entry.get('reused_existing_manifest') else path.parent
                    target = (base / fields['path']).resolve()
                    assert target.is_relative_to((ROOT / v).resolve())
                    linked = tomllib.loads((target / 'Cargo.toml').read_text())
                    assert linked['package']['name'] == pkg['name']
                    if pkg.get('version'): assert linked['package']['version'] == pkg['version']
                assert pkg['name'] not in ('std', 'core', 'alloc')
                if v != 'A': actual[key(resolve(dep['lock_token']))] += 1
            if v != 'A':
                for dep in entry['unresolved_dependencies']:
                    bound = resolve(dep['lock_token'])
                    if bound: actual[key(bound)] += 1
                    else: actual[('unresolved', dep['lock_token'])] += 1
                expected = Counter(key(resolve(t)) if resolve(t) else ('unresolved', t) for t in entry['locked_dependencies'])
                assert actual == expected
            if changed_existing:
                # Another reconstruction pass can update comments/metadata in an
                # independent workspace. Rebind only after all package/edge checks.
                entry['manifest_sha256'] = sha(path)
                path.with_suffix('.toml.evidence.json').write_text(json.dumps(entry, ensure_ascii=False, indent=2) + '\n')
            count += 1
        assert count == report['module_manifests']
        (ROOT / '_cargo' / f'{v}-report.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n')
        if v in 'BC':
            source = PROJECT / 'evidence/multi-version' / f'{v}-original-Cargo.lock'
            assert (ROOT / v / 'Cargo.lock').read_bytes() == source.read_bytes()
        checks.append({'version': v, 'ok': True, 'module_manifests': count, 'active_dependencies': report['active_dependencies'],
            'unresolved_dependencies_retained': report['unresolved_dependencies'], 'known_module_versions': report['known_package_versions'],
            'all_locked_edges_preserved': v != 'A', 'versions_and_git_commits_verified': True, 'A_unknown_versions_not_borrowed_from_other_versions': v == 'A'})
        print(json.dumps(checks[-1]), flush=True)
    roots = [ROOT / v / 'Cargo.toml' for v in versions]
    if 'C' in versions: roots.append(ROOT / 'C/rhapsody2/Cargo.toml')
    for manifest in roots:
        label = manifest.parent.relative_to(ROOT).as_posix().replace('/', '-')
        out = ROOT / '_cargo' / f'{label}-cargo-metadata.json'
        log = ROOT / '_cargo' / f'{label}-cargo-metadata.log'
        before = {p: sha(p) for p in ROOT.rglob('Cargo.lock')}
        env = {**os.environ, 'CARGO_NET_OFFLINE': 'true'}
        proc = subprocess.run(['cargo', 'metadata', '--no-deps', '--offline', '--format-version', '1', '--manifest-path', str(manifest)], capture_output=True, env=env)
        out.write_bytes(proc.stdout); log.write_bytes(proc.stderr)
        assert proc.returncode == 0, proc.stderr.decode(errors='replace')
        data = json.loads(proc.stdout)
        for p, checksum in before.items(): assert sha(p) == checksum
        workspace_checks.append({'manifest': manifest.relative_to(ROOT).as_posix(), 'ok': True, 'offline_no_deps': True,
            'workspace_packages': len(data['packages']), 'lockfiles_unchanged': True})
        print(json.dumps(workspace_checks[-1]), flush=True)
    result = {'ok': True, 'scope_versions': versions, 'manifest_files': sum(r['module_manifests'] for r in reports) + len(versions),
        'checks': checks, 'workspace_checks': workspace_checks,
        'scope': 'TOML/Cargo parser, dependency package/version/source identities and workspace topology; not full build, feature or original manifest recovery'}
    (ROOT / 'cargo-restoration-verification.json').write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n')

if __name__ == '__main__': main()
