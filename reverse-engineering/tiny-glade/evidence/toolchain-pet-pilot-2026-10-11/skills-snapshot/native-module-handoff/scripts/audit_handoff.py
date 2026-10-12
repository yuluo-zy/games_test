#!/usr/bin/env python3
"""Check a recovery ledger's references, never attest native semantic equivalence."""
import argparse
import hashlib
import json
from pathlib import Path
import re

KINDS={'source_owner','instruction','behavior','dependency','runtime','verification'}
STATUSES={'unknown','evidence_ready','implemented_unverified','verified'}

def audit(path,root):
    j=json.loads(path.read_text());errs=[];warnings=[];facts={};versions=j['versions'];target=j['target_version']
    if target not in versions: errs.append('Target version missing')
    for v,data in versions.items():
        if not re.fullmatch('[0-9a-f]{64}',data.get('artifact_sha256','')): errs.append('Invalid artifact identity: '+v)
    def local(file):
        q=(root/file).resolve()
        if not q.is_relative_to(root.resolve()): raise ValueError('Evidence/source path escapes project: '+file)
        return q
    for f in j['facts']:
        if f['id'] in facts: errs.append('Duplicate fact: '+f['id'])
        facts[f['id']]=f
        if f['version'] not in versions or f['kind'] not in KINDS: errs.append('Invalid fact: '+f['id'])
        if not local(f['evidence']).is_file(): errs.append('Missing evidence: '+f['id'])
    unresolved=[]
    for link in j['links']:
        status=link['status'];refs=link.get('fact_ids',[])
        if status not in STATUSES: errs.append('Invalid link status: '+link['name'])
        if status!='unknown' and not refs: errs.append('Missing evidence references: '+link['name'])
        for ref in refs:
            if ref not in facts: errs.append('Unknown fact reference: '+ref)
        if status=='verified':
            if link.get('gaps'): errs.append('Verified link still declares gaps: '+link['name'])
            kinds={facts[r]['kind'] for r in refs if r in facts and facts[r]['version']==target}
            if not {'runtime','verification'}<=kinds: errs.append('Unsupported target-version verified label: '+link['name'])
        else: unresolved.append({'name':link['name'],'status':status,'gaps':link.get('gaps',[])})
        if refs and not any(r in facts and facts[r]['version']==target for r in refs): warnings.append('Other-version-only evidence: '+link['name'])
    files={}
    for file in j.get('rust_files',[]):
        q=local(file)
        if not q.is_file(): errs.append('Missing Rust source: '+file)
        else: files[file]=hashlib.sha256(q.read_bytes()).hexdigest()
    return {'ok':not errs,'semantic_acceptance':False,'identity_scope':'artifact hashes are ledger declarations with format checks, not binary-file rehashes','module':j['module'],'target_version':target,'errors':errs,'warnings':warnings,'ledger_sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'auditor_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'fact_count':len(facts),'link_count':len(j['links']),'unresolved_links':unresolved,'rust_sha256':files,'dependencies':j.get('dependencies',[])}

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('ledger',type=Path);p.add_argument('root',type=Path);p.add_argument('output',type=Path);a=p.parse_args()
    try: out=audit(a.ledger,a.root)
    except Exception as e: out={'ok':False,'semantic_acceptance':False,'errors':[str(e)]}
    a.output.parent.mkdir(parents=True,exist_ok=True);a.output.write_text(json.dumps(out,ensure_ascii=False,indent=2));print(json.dumps(out,ensure_ascii=False))
    raise SystemExit(0 if out['ok'] else 1)
