#!/usr/bin/env python3
"""Compare public dependency provenance, not native code or fork behavior."""
import argparse
import hashlib
import json
from pathlib import Path
import tomllib


def compare(original,candidate):
    a=tomllib.loads(original.read_text())['package'];b=tomllib.loads(candidate.read_text())['package']
    index={(p['name'],p['version'],p.get('source')):p for p in a};rows=[];private=[]
    for p in b:
        if not p.get('source'):
            private.append({'name':p['name'],'version':p['version']});continue
        q=index.get((p['name'],p['version'],p['source']))
        rows.append({'name':p['name'],'version':p['version'],'source':p['source'],'present_original':q is not None,'checksum_match':q is not None and q.get('checksum')==p.get('checksum'),'original_choices':[{'version':x['version'],'source':x.get('source')} for x in a if x['name']==p['name']] if q is None else []})
    mismatches=[x for x in rows if not x['checksum_match']]
    return {'ok':not mismatches,'scope':'Public lock provenance only; source-less local entries require classification and matching packages do not prove native fork behavior equivalence','original_lock_sha256':hashlib.sha256(original.read_bytes()).hexdigest(),'candidate_lock_sha256':hashlib.sha256(candidate.read_bytes()).hexdigest(),'public_packages':len(rows),'matched':len(rows)-len(mismatches),'mismatches':mismatches,'local_entries_unclassified':private,'rows':rows}

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('original',type=Path);p.add_argument('candidate',type=Path);p.add_argument('--output',type=Path,required=True);a=p.parse_args()
    try: result=compare(a.original,a.candidate)
    except Exception as e: result={'ok':False,'error':str(e)}
    a.output.parent.mkdir(parents=True,exist_ok=True);a.output.write_text(json.dumps(result,ensure_ascii=False,indent=2))
    print(json.dumps({k:v for k,v in result.items() if k not in ('rows','mismatches','local_entries_unclassified')},ensure_ascii=False))
    raise SystemExit(0 if result['ok'] else 1)
