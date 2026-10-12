"""Record locked workspace builds and compare dependency identities with B."""
from datetime import datetime, timezone
from pathlib import Path
import hashlib, json, shutil, subprocess, tomllib
from verification_provenance import ROOT, candidate_provenance

def main():
    cargo, rustc = shutil.which('cargo'), shutil.which('rustc')
    if not cargo or not rustc: raise RuntimeError('cargo and rustc required in PATH')
    out = ROOT / 'evidence/multi-version'
    commands = []
    for action in ['test', 'build']:
        args = [cargo, action]
        if action == 'build': args += ['--release']
        args += ['--workspace', '--locked', '--manifest-path', str(ROOT/'reconstruction/Cargo.toml')]
        result = subprocess.run(args, cwd=ROOT, capture_output=True, text=True)
        log = out / f'workspace-{action}.log'
        log.write_text(result.stdout + result.stderr)
        commands.append({'args':args, 'exit_code':result.returncode, 'log':log.relative_to(ROOT).as_posix()})
    ok = all(command['exit_code'] == 0 for command in commands)
    original = tomllib.loads((out/'B-original-Cargo.lock').read_text())['package']
    current = tomllib.loads((ROOT/'reconstruction/Cargo.lock').read_text())['package']
    rows = []
    for package in current:
        if not package.get('source', '').startswith('registry+'): continue
        matched = any(all(package.get(key) == candidate.get(key) for key in ['name','version','source','checksum']) for candidate in original)
        rows.append({key:package[key] for key in ['name','version','source','checksum']} | {'original_version_and_checksum_match':matched})
    deps_ok = all(row['original_version_and_checksum_match'] for row in rows)
    report = {'ok':ok and deps_ok, 'recorded_at_utc':datetime.now(timezone.utc).isoformat(),
        'scope':'Current locked Rust workspace tests/build and B registry dependency identities; no full-game acceptance',
        'commands':commands, 'rustc':subprocess.check_output([rustc,'--version'],text=True).strip(),
        'cargo':subprocess.check_output([cargo,'--version'],text=True).strip(), **candidate_provenance()}
    (out/'workspace-build-verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
    dependency_report = {'ok':deps_ok, 'baseline':'B', 'matched_registry_dependencies':rows,
        'candidate_cargo_lock_sha256':report['candidate_cargo_lock_sha256'],
        'original_lock_sha256':hashlib.sha256((out/'B-original-Cargo.lock').read_bytes()).hexdigest(),
        'features_identical_to_original':None,
        'scope':'Name/version/source/checksum of every current registry package; original features and engine fork unverified',
        'previous_reports':'Pre-history workspace reports remain historical evidence; not current build acceptance'}
    (out/'workspace-dependency-verification.json').write_text(json.dumps(dependency_report,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps({'ok':report['ok'], 'registry_dependencies':len(rows), 'commands':[r['exit_code'] for r in commands]}))
    if not report['ok']: raise SystemExit(1)

if __name__ == '__main__': main()
