"""格式化、保存指纹并释放本轮 claim；不编译、测试或执行游戏原码。"""
import hashlib
import json
import subprocess
from datetime import datetime, timezone
from pathlib import Path

OUT = Path(__file__).resolve().parent
PROJECT = OUT.parents[1]
INDEX = PROJECT / 'skeleton/task-index'

def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def write(path, obj):
    path.write_text(json.dumps(obj, ensure_ascii=False, indent=2) + '\n')

def run(argv):
    result = subprocess.run(argv, capture_output=True, text=True)
    if result.returncode:
        raise RuntimeError(result.stdout + result.stderr)
    return result.stdout

def main():
    mapping = json.loads((OUT / 'mapping.json').read_text())
    files = list(dict.fromkeys(x['candidate_path'] for x in mapping))
    run(['/Users/yuluo/.cargo/bin/rustfmt', '--edition', '2021', '--config', 'skip_children=true', *files])
    for item in mapping:
        code = Path(item['candidate_path']).read_text()
        item['candidate_sha256'] = digest(item['candidate_path'])
        item['source_marker_line'] = code[:code.index('// ORIGINAL: ' + item['original_symbol'] + '\n')].count('\n') + 1
    write(OUT / 'mapping.json', mapping)
    claims, receipts = [], []
    for task in dict.fromkeys(x['task_id'] for x in mapping):
        entries = [x for x in mapping if x['task_id'] == task]
        claim = json.loads((INDEX / 'claims' / (task + '.json')).read_text())
        assert claim['owner'] == 'abc-tools-52'
        claims.append(claim)
        work = Path(claim['work_dir'])
        write(work / 'mapping.json', entries)
        refs = {}
        for entry in entries:
            for version in 'ABC':
                for name in entry['variants'][version]['evidence']:
                    for path in [Path(name), Path(name).with_suffix('.asm.txt')]:
                        refs[str(path)] = digest(path)
        for path in [PROJECT / 'tracking/dependency-exclusions.json', PROJECT / 'tracking/baselines.json']:
            refs[str(path)] = digest(path)
        write(work / 'evidence-refs.json', {'scope': 'identity-bound raw evidence; references are not runtime acceptance', 'files_sha256': refs})
        write(work / 'source.sha256.json', {str(Path(x['candidate_path']).relative_to(work)): x['candidate_sha256'] for x in entries})
        flow = PROJECT / 'skeleton/workflow-design/tiny-glade-recovery/scripts/workflow.py'
        manifest = work / 'delivery.json'
        if manifest.exists():
            archive = work / ('delivery-superseded-' + datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%S%fZ') + '.json')
            manifest.rename(archive)
        cmd = ['python3', str(flow), 'freeze', '--root', str(work), '--output', str(manifest)]
        for name in ['mapping.json', 'CONTRACT.md', 'obligations.md', 'evidence-refs.json', 'source.sha256.json', entries[0]['original_source_path']]:
            cmd += ['--file', name]
        run(cmd)
        run(['python3', str(flow), 'check', '--manifest', str(manifest)])
        # 核对冻结引用不会把候选升级为 verified，也不是原码或游戏行为测试。
        receipts.append({'task_id': task, 'manifest': str(manifest), 'status': 'frozen_unverified'})
    write(OUT / 'claims.json', claims)
    write(OUT / 'deliveries.json', receipts)
    releases = []
    for claim in claims:
        releases.append(json.loads(run(['python3', str(PROJECT / 'skeleton/_tools/task_queue.py'), 'release', claim['task_id'], '--agent', 'abc-tools-52', '--reason', '52组共享工具候选已保存并冻结；验证继续暂停，未独立审查/未整合，见 evidence/abc-tools-52-2026-10-11/README.md'])))
    write(OUT / 'releases.json', releases)
    write(OUT / 'summary.json', {
        'completed_at_utc': datetime.now(timezone.utc).isoformat(),
        'logical_function_candidates': len(mapping), 'original_source_file_groups': len(files),
        'version_mappings': sum(len(x['variants'][v]['native_instances']) for x in mapping for v in 'ABC'),
        'status': 'implemented_unverified', 'formatting_completed': True,
        'frozen_reference_integrity_checked': True, 'build_run': False, 'tests_run': False,
        'native_target_executed': False, 'independent_review': False,
        'production_integration': False, 'signature_and_abi_gaps_recorded': True,
        'overview': str(OUT / 'README.md'), 'mapping': str(OUT / 'mapping.json'),
    })
    print(json.dumps({'functions': len(mapping), 'files': len(files), 'frozen': len(receipts), 'released': len(releases)}))

if __name__ == '__main__':
    main()
