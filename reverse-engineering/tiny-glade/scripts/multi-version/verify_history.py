"""Compare Rust decoding against actual A/B/C history JSON, without replay."""
from pathlib import Path
from collections import Counter
import hashlib, json, subprocess
from verification_provenance import candidate_provenance

ROOT = Path(__file__).resolve().parents[2]

def main():
    baselines = json.loads((ROOT/'tracking/baselines.json').read_text())['baselines']
    runner = ROOT/'reconstruction/target/release/tg-reconstruction'
    rows, totals, failures = [], {}, []
    for version, baseline in baselines.items():
        counts, files = Counter(), 0
        source_root = Path(baseline['root'])
        for path in sorted((source_root/'assets/starting-builds').rglob('history.json')):
            raw = path.read_bytes()
            data = json.loads(raw)
            expected, roofs = Counter(), Counter()
            for entry in data['History']['edit_chain']:
                for edit in entry['edits']:
                    expected.update(edit.keys())
                    if 'Roof' in edit:
                        roofs.update(edit['Roof'].keys())
            process = subprocess.run([str(runner),'history-summary',str(path)], capture_output=True, text=True)
            if process.returncode:
                failures.append({'baseline':version,'path':str(path),'error':process.stderr})
                continue
            actual = json.loads(process.stdout)
            ok = (actual['command_counts'] == dict(expected)
                  and actual['roof_command_counts'] == dict(roofs)
                  and actual['version'] == data['Version']
                  and actual['chain_index'] == data['History']['chain_index']
                  and actual['entries'] == len(data['History']['edit_chain'])
                  and not actual['unknown_roof_commands'])
            if not ok:
                failures.append({'baseline':version,'path':str(path),'actual':actual})
            counts.update(roofs)
            files += 1
            rows.append({'baseline':version,'path':path.relative_to(source_root).as_posix(),
                         'sha256':hashlib.sha256(raw).hexdigest(),'ok':ok,'summary':actual})
        totals[version] = {'history_files':files,'roof_commands':dict(counts)}
    report = {'ok':not failures,
        'scope':'Three-version packaged history decode, typed roof payload validation and exact header/count comparison; replay/undo/GPU not verified',
        'baselines':totals,'failures':failures,'files':rows,
        'cargo_lock_sha256':hashlib.sha256((ROOT/'reconstruction/Cargo.lock').read_bytes()).hexdigest()}
    report.update(candidate_provenance())
    (ROOT/'evidence/multi-version/history-rust-verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps({'ok':report['ok'],'totals':totals,'failure_count':len(failures)},ensure_ascii=False))
    if failures:
        print(json.dumps(failures[:2],ensure_ascii=False))
        raise SystemExit(1)

if __name__ == '__main__':
    main()
