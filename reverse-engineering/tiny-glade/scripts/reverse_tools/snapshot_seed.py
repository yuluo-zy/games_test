"""Freeze a closed, verified worker as a reusable B import seed."""
import argparse
import hashlib
import json
import shutil
from pathlib import Path
from common import SEED, SHA256, WorkerLock, atomic_json, owned_alive, worker_dir

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('worker')
    parser.add_argument('verified_report',type=Path)
    args=parser.parse_args()
    directory=worker_dir(args.worker)
    with WorkerLock(directory):
        state=json.loads((directory/'state.json').read_text(encoding='utf-8'))
        if owned_alive(state) or state['status']!='stopped': raise ValueError('Seed source must be gracefully closed')
        report=json.loads(args.verified_report.read_text(encoding='utf-8'))
        if not report['ok'] or report['worker']!=args.worker or report['identity']['sha256']!=SHA256:
            raise ValueError('Source report must pass and identify this exact B worker')
        if Path(report['project']).resolve()!= (directory/'project/TinyGlade.gpr').resolve():
            raise ValueError('Source report project mismatch')
        if SEED.exists(): raise ValueError('Seed already exists; retained baselines are never overwritten')
        SEED.mkdir(parents=True)
        shutil.copytree(directory/'project',SEED/'project')
        rows=[]
        for path in sorted((SEED/'project').rglob('*')):
            if path.is_file():
                with path.open('rb') as stream: digest=hashlib.file_digest(stream,'sha256').hexdigest()
                rows.append({'path':path.relative_to(SEED/'project').as_posix(),'sha256':digest,'bytes':path.stat().st_size})
        atomic_json(SEED/'manifest.json',{'program_sha256':SHA256,'source_worker':args.worker,
            'source_report':str(args.verified_report.resolve()),'source_report_sha256':hashlib.sha256(args.verified_report.read_bytes()).hexdigest(),
            'files':rows,'historical_mac_overlays':False})
        print(json.dumps({'files':len(rows),'bytes':sum(r['bytes'] for r in rows),'seed':str(SEED)}))

if __name__=='__main__':main()
