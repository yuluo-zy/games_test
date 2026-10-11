#!/usr/bin/env python3
"""Reproduce the pinned sheep-pet evidence workflow; no target or building execution."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import time


def hashfile(path):
    with path.open('rb') as f: return hashlib.file_digest(f,'sha256').hexdigest()


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('recipe',type=Path);p.add_argument('--output',required=True,type=Path);p.add_argument('--skip-mcp',action='store_true')
    a=p.parse_args();start=time.monotonic();out=a.output.resolve()
    if out.exists(): p.error('Use a new output directory; previous receipts remain intact')
    out.mkdir(parents=True);r=json.loads(a.recipe.read_text());project=Path(r['project_root']).resolve();skills=Path(r['skill_root']).resolve()
    def resolved(key):
        value=Path(r[key]);return value.resolve() if value.is_absolute() else (project/value).resolve()
    stages=[]
    if not a.skip_mcp:
        stages.append(('mcp-batch',[sys.executable,str(skills/'native-tool-batch/scripts/run_batch.py'),str(resolved('mcp_plan')),str(out/'mcp-batch')],out/'mcp-batch/summary.json','ok',True))
    for phase in ('cold','warm'):
        stages.append(('pack-'+phase,[sys.executable,str(skills/'native-evidence-pack/scripts/collect.py'),str(resolved('pack_manifest')),'--output',str(out/('pack-'+phase)),'--cache',str(out/'cache')],out/('pack-'+phase)/'report.json','status','ok'))
    if r.get('original_lock') and r.get('candidate_lock'):
        stages.append(('dependency-lock',[sys.executable,str(skills/'native-rust-reconstruction/scripts/compare_cargo_locks.py'),str(resolved('original_lock')),str(resolved('candidate_lock')),'--output',str(out/'dependency-lock.json')],out/'dependency-lock.json','ok',True))
    stages.append(('handoff',[sys.executable,str(skills/'native-module-handoff/scripts/audit_handoff.py'),str(resolved('ledger')),str(project),str(out/'handoff.json')],out/'handoff.json','ok',True))
    receipt={'scope':r['scope'],'semantic_acceptance':False,'target_execution':False,'recipe_sha256':hashfile(a.recipe),'runner_sha256':hashfile(Path(__file__)),'skip_mcp':a.skip_mcp,'stages':[],'ok':False}
    try:
        for phase,argv,report,key,expected in stages:
            begin=time.monotonic()
            with (out/(phase+'.log')).open('w') as log:
                cp=subprocess.run(argv,stdout=log,stderr=subprocess.STDOUT,shell=False,timeout=1800)
            row={'id':phase,'argv':argv,'exit_code':cp.returncode,'elapsed_seconds':round(time.monotonic()-begin,6),'script_sha256':hashfile(Path(argv[1])),'report':str(report)}
            if report.is_file():
                data=json.loads(report.read_text());row['report_sha256']=hashfile(report);row['metrics']=data.get('stats', {k:data[k] for k in ('operator_processes','mcp_steps','requested_steps','unique_steps','deduplicated_steps','raw_report_bytes','payload_bytes') if k in data});row['ok']=cp.returncode==0 and data.get(key)==expected
            else: row['ok']=False
            receipt['stages'].append(row)
            if not row['ok']: raise RuntimeError('Stage failed: '+phase+'; inspect its retained report/log')
        receipt['ok']=True
    except Exception as e:receipt['error']=str(e)
    receipt['elapsed_seconds']=round(time.monotonic()-start,6)
    (out/'receipt.json').write_text(json.dumps(receipt,ensure_ascii=False,indent=2))
    print(json.dumps({'ok':receipt['ok'],'receipt':str(out/'receipt.json'),'elapsed_seconds':receipt['elapsed_seconds'],'stages':[{'id':s['id'],'ok':s['ok'],'metrics':s.get('metrics')} for s in receipt['stages']]},ensure_ascii=False))
    return 0 if receipt['ok'] else 1

if __name__=='__main__':raise SystemExit(main())
