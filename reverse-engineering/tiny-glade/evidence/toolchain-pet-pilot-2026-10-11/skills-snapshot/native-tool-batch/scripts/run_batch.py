#!/usr/bin/env python3
"""Bounded, read-only operator batch with retained semantic-error summaries."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess
import time

READS = {'get_metadata','get_functions','find_functions','get_xrefs_to','get_xrefs_from','read_memory','disassemble_function','get_struct_layout','find_data_types','get-function-count','get-decompilation','find-cross-references','get-data-type-by-string','procedure_info','procedure_pseudo_code','read_bytes','xrefs','batch_decompile','inspect_native_instruction','trace_native_values','show_info','list_sections','list_decompilers','disassemble','decompile_function','hexdump','list_functions','get_database_info','get_type_info','get_comment'}

def digest(p):
    with Path(p).open('rb') as f:
        return hashlib.file_digest(f,'sha256').hexdigest()

def decode(result):
    items=[]
    if result.get('structuredContent') is not None: items.append(result['structuredContent'])
    for c in result.get('content',[]):
        if c.get('type')=='text':
            try: items.append(json.loads(c['text']))
            except json.JSONDecodeError: items.append(c['text'])
    unique=[];seen=set()
    for item in items:
        key=json.dumps(item,sort_keys=True,ensure_ascii=False)
        if key not in seen: unique.append(item);seen.add(key)
    return unique

def errors(value, pointer=''):
    found=[]
    if isinstance(value,dict):
        for k,v in value.items():
            p=pointer+'/'+k
            if (k in ('error','errors') and v) or (k in ('success','ok') and v is False) or (k=='timedOut' and v is True): found.append(p+': '+str(v)[:220])
            found.extend(errors(v,p))
    elif isinstance(value,list):
        for i,v in enumerate(value): found.extend(errors(v,pointer+'/'+str(i)))
    return found

def run(plan_path,out):
    start=time.monotonic(); out.mkdir(parents=True,exist_ok=True)
    if (out/'summary.json').exists(): raise ValueError('Choose a fresh output directory')
    summary={'ok':False,'target_execution':False,'operator_processes':0,'mcp_steps':0,'errors':[],'identity_scope':'not checked','semantic_acceptance':False,'warnings':[]}
    try:
        plan=json.loads(plan_path.read_text());summary['plan_sha256']=digest(plan_path);summary['runner_sha256']=digest(__file__);summary['operator_sha256']=digest(plan['operator'])
        art=plan['artifact'];expected=art['sha256']
        if not re.fullmatch('[0-9a-f]{64}',expected): raise ValueError('Invalid artifact SHA256')
        actual=digest(art['path']);summary['artifact_sha256']=actual
        if actual!=expected: raise ValueError('Artifact identity mismatch')
        if plan['backend'] not in ('ghidra','reva'): raise ValueError('This helper supports persistent Ghidra/ReVa read sessions; use the operator directly for other backends')
        unique=[];seen=set()
        for original_step in plan['steps']:
            s=json.loads(json.dumps(original_step))
            if s.get('tool') not in READS or s.get('allow_error'): raise ValueError('Only bounded read queries without allow_error are supported')
            if not 0<float(s.get('timeout_seconds',300))<=300: raise ValueError('Invalid timeout')
            args=s.get('args',{})
            selector='program' if plan['backend']=='ghidra' else 'programPath'
            if not args.get(selector): raise ValueError('Explicit '+selector+' selector is required')
            if s['tool'] in ('get_xrefs_to','get_xrefs_from','find-cross-references','get-decompilation') and not 0<int(args.get('limit',0))<=100: raise ValueError('Bound references/decompilation to 1..100')
            if s['tool']=='get_functions':
                if args.get('function') and not args.get('functions'): args['functions']=args.pop('function')
                n=len([f for f in args.get('functions','').split(',') if f])
                if not 1<=n<=20 or not args.get('fields'): raise ValueError('Select 1..20 functions and explicit fields')
                fields=args['fields'].split(',')
                if 'call_context' in fields and (not 0<int(args.get('call_context_limit',0))<=6 or not 1<=int(args.get('call_context_lines',0))<=21): raise ValueError('Bound call context explicitly to <=6 callers and <=21 lines')
                if 'call_context' not in fields: args['include_call_context']=False
            if s['tool'] in ('find_functions','list_functions') and not 0<int(args.get('limit',0))<=100: raise ValueError('Bound enumeration to 1..100')
            if s['tool']=='read_memory' and not 0<int(args.get('length',0))<=4096: raise ValueError('Memory request must be 1..4096 bytes')
            key=json.dumps(s,sort_keys=True)
            if key not in seen: unique.append(s);seen.add(key)
        if not 1<=len(unique)<=60: raise ValueError('Select 1..60 unique steps')
        job=out/'job.json';job.write_text(json.dumps(unique,indent=2))
        summary.update(requested_steps=len(plan['steps']),unique_steps=len(unique),deduplicated_steps=len(plan['steps'])-len(unique))
        report_path=out/'operator-report.json'
        argv=[plan['interpreter'],plan['operator'],plan['backend'],str(job.resolve()),str(report_path.resolve()),'--worker',plan.get('worker','shared')]
        summary['operator_processes']=1
        with (out/'operator.log').open('w') as log:
            proc=subprocess.run(argv,stdout=log,stderr=subprocess.STDOUT,timeout=120+sum(float(s.get('timeout_seconds',300)) for s in unique))
        summary['exit_code']=proc.returncode
        if not report_path.exists(): raise ValueError('Operator did not preserve a report')
        report=json.loads(report_path.read_text());summary['mcp_steps']=len(report.get('results',[]))
        summary['connected']=report.get('actual_mcp_connection',False)
        if proc.returncode!=0 or report.get('ok') is not True or not summary['connected']: summary['errors'].append('Operator failed or did not confirm connection: '+str(report.get('error','')))
        payloads=out/'payloads';payloads.mkdir(exist_ok=True);decoded=[]
        for i,step in enumerate(report.get('results',[])):
            result=step['result'];items=decode(result);decoded.append(items)
            (payloads/f'{i:03d}-{step["tool"]}.json').write_text(json.dumps(items,ensure_ascii=False,indent=2))
            if result.get('isError'): summary['errors'].append(f'Step {i}: MCP isError')
            for e in errors(items): summary['errors'].append(f'Step {i}: '+e)
            if step['tool']=='get_functions':
                if not any(isinstance(x,dict) and isinstance(x.get('functions'),dict) for x in items): summary['errors'].append(f'Step {i}: unrecognized function bundle shape')
                for item in items:
                    if isinstance(item,dict) and isinstance(item.get('functions'),dict):
                        req=step['args'].get('functions',step['args'].get('function','')).split(',')
                        for ref in req:
                            if ref not in item['functions']: summary['errors'].append(f'Step {i}: missing function {ref}')
                            elif not isinstance(item['functions'][ref],dict): summary['errors'].append(f'Step {i}: invalid function {ref}')
                            elif 'decompiled_code' in step['args'].get('fields','').split(',') and not item['functions'][ref].get('decompiled_code'): summary['errors'].append(f'Step {i}: no decompilation for {ref}')
                        summary.setdefault('functions',[]).extend({'reference':ref,'name':f.get('name'),'size':f.get('size'),'revision':f.get('revision'),'placeholder_signature':bool(re.search(r'\bvoid\b[^ (]*(?:[^()]*)\(\s*(?:void)?\s*\)',f.get('signature',''))),'code_bytes':len(f.get('decompiled_code','').encode())} for ref,f in item['functions'].items() if isinstance(f,dict))
        for i,step in enumerate(report.get('results',[])):
            if step['tool']=='get_functions':
                for item in decoded[i]:
                    if isinstance(item,dict):
                        for ref,f in item.get('functions',{}).items():
                            if isinstance(f,dict):
                                code=re.sub(r'/\*.*?\*/|//[^\n]*','',f.get('decompiled_code',''),flags=re.S).strip()
                                if re.search(r'\bvoid\b[^()]*\(\s*(?:void)?\s*\)',f.get('signature','')) and re.search(r'\{\s*(?:return\s*;)?\s*\}\s*$',code): summary['errors'].append('Unresolved placeholder-only decompilation for '+ref)
        for f in summary.get('functions',[]):
            if f['placeholder_signature']: summary['warnings'].append('Possible placeholder ABI for '+f['reference']+'; inspect original call convention and CFG')
        guard=plan.get('metadata_guard')
        if guard:
            idx=int(guard['step']);candidates=[x.get(guard['field']) for x in decoded[idx] if isinstance(x,dict)]
            if guard['equals'] not in candidates: summary['errors'].append('Selected program metadata guard mismatch')
            else: summary['identity_scope']='local file hash + returned metadata field; database import hash unverified'
        else: summary['identity_scope']='local file hash only; selected database identity unverified'
        if summary['mcp_steps']!=len(unique): summary['errors'].append('Incomplete step coverage')
        summary['raw_report_bytes']=report_path.stat().st_size
        summary['payload_bytes']=sum(p.stat().st_size for p in payloads.glob('*.json'))
        summary['ok']=not summary['errors']
    except Exception as e: summary['errors'].append(str(e))
    finally:
        summary['elapsed_seconds']=round(time.monotonic()-start,6)
        (out/'summary.json').write_text(json.dumps(summary,ensure_ascii=False,indent=2))
    print(json.dumps(summary,ensure_ascii=False));return 0 if summary['ok'] else 1

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('plan',type=Path);p.add_argument('output',type=Path);a=p.parse_args()
    raise SystemExit(run(a.plan,a.output))
