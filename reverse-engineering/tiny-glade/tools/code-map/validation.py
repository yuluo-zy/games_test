"""Independent byte/relationship checks and honest paired query measurements."""
from __future__ import annotations
import collections
import hashlib
import json
import math
from pathlib import Path
import re
import resource
import statistics
import subprocess
import time
import zipfile

from artifacts import (RUNTIME, PROJECT, WORKSPACE, LLVM, atomic, baseline, canonical,
                       db, digest, load, pack_module, read_blob, coverage, resolve_evidence)
from graph import Graph, query_tool

CHAIN = Path(__file__).with_name('chain.json')
REPORTS = WORKSPACE/'tiny-glade/reports/module-recovery-2026-10-23'

def evidence_status():
    errors=[];stale=[]
    for row in load(CHAIN)['evidence']:
        p=resolve_evidence(row['path'])
        if not p.exists() or digest(p)!=row['sha256']:
            (stale if row['role'].startswith('current_implementation') else errors).append(str(p))
    return {'original_evidence_errors':errors,'stale_implementation_bindings':stale,
            'current_implementation_revalidated':False}

def verification_ok(failures,checks):
    return not failures and all(row.get('ok',False)
        for check in checks if check.get('kind')=='independent_objdump' for row in check['ranges'])

def verify(full=False):
    began=time.monotonic();con=db(True);checks=[];failures=[]
    b=baseline('B');pe=pack_module().PE(b['binary']['path'])
    memory={s['rva']:pe.read_rva(s['rva'],s['raw_size']) for s in pe.sections if s['executable']}
    def original_bytes(at,length):
        s=pe.section_for(at,length,executable=True)
        return memory[s['rva']][at-s['rva']:at-s['rva']+length]
    archives=collections.OrderedDict()
    def blob(row):
        name=row['blob']
        if name not in archives:
            if len(archives)>=8:archives.popitem(last=False)[1].close()
            archives[name]=zipfile.ZipFile(RUNTIME/name)
        archive=archives.pop(name);archives[name]=archive
        raw=archive.read(row['member'])
        if hashlib.sha256(raw).hexdigest()!=row['sha']:raise ValueError('Assembly blob integrity failure')
        return json.loads(raw)
    rows=con.execute("SELECT * FROM assemblies WHERE version='B' ORDER BY start,end").fetchall()
    if not full:
        rows=[r for r in rows if r['start'] in (0x8cc870,0x8cdc60,0x8ce4a0,0x8cd430,0x8d0490,0x21a3220)]
    for row in rows:
        try:
            doc=blob(row)
            if doc['binary_sha256']!=b['binary']['sha256']:raise ValueError('Binary identity mismatch')
            raw=original_bytes(row['start'],row['end']-row['start'])
            if hashlib.sha256(raw).hexdigest()!=doc['range_sha256']:raise ValueError('Range SHA mismatch')
            end=row['start'];covered=set()
            for at,h,text in doc['instructions']:
                ins=bytes.fromhex(h)
                if at<end or at+len(ins)>row['end']:raise ValueError('Overlap/out-of-range instruction')
                if raw[at-row['start']:at-row['start']+len(ins)]!=ins:raise ValueError('Instruction mismatch')
                end=at+len(ins);covered.update(range(at,end))
            covered.update(doc['undecoded'])
            if len(covered)!=len(raw):raise ValueError('Unreported coverage gap')
        except Exception as e:failures.append({'start':row['start'],'error':str(e)})
    checks.append({'kind':'original_bytes','ranges':len(rows),'failures':len(failures)})
    # Separate objdump executable, explicit known PDB ranges, no graph-derived expected output.
    independent=[]
    # Roof span is 14692 in the retained B PDB procedure index; an older narrative
    # said 12048. Freeze the actual PDB range, not that historical prose count.
    fixed=[(0x8cc870,39),(0x8cdc60,786),(0x8ce4a0,1398),(0x8cd430,383),(0x8d0490,31293),(0x21a3220,14692)]
    pattern=re.compile(r'^\s*([0-9a-fA-F]+):\s+((?:[0-9a-fA-F]{2}[ \t]+)+)')
    for start,length in fixed:
        result=subprocess.run([str(LLVM/'bin/llvm-objdump'),'-d','--start-address='+hex(pe.image_base+start),
                               '--stop-address='+hex(pe.image_base+start+length),b['binary']['path']],capture_output=True,text=True,check=True)
        observed={}
        for line in result.stdout.splitlines():
            match=pattern.match(line)
            if match:observed[int(match[1],16)-pe.image_base]=bytes.fromhex(match[2]).hex()
        row=con.execute("SELECT * FROM assemblies WHERE version='B' AND start=? AND end=?",(start,start+length)).fetchone()
        if not row:
            independent.append({'start':start,'ok':False,'error':'Exact PDB range absent'})
            failures.append({'start':start,'error':'Exact independent PDB range absent'})
            continue
        actual={at:h for at,h,text in blob(row)['instructions']}
        good=observed==actual
        independent.append({'start':start,'instructions':len(observed),'ok':good})
        if not good:failures.append({'start':start,'error':'objdump instruction sequence differs'})
    checks.append({'kind':'independent_objdump','ranges':independent})
    state=evidence_status()
    failures.extend({'error':'Original evidence changed','path':p} for p in state['original_evidence_errors'])
    con.close()
    graph=Graph();snapshot=graph.current()
    # Validate semantic import preservation against the pre-import reviewed fixture.
    fixture=load(CHAIN);missing=[]
    for edge in fixture['edges']:
        if edge['status']!='confirmed':continue
        found=graph.query('''MATCH (a:Entity {snapshot:$s,key:$a})-[r:LINK]->(b:Entity {snapshot:$s,key:$b})
                WHERE r.kind=$kind AND r.status='confirmed' RETURN r.evidence AS evidence''',
                {'s':snapshot,'a':edge['source'],'b':edge['target'],'kind':edge['kind']})
        if not found or edge['evidence'] not in [x['evidence'] for x in found]:missing.append(edge)
    failures.extend({'error':'Missing curated semantic edge','edge':e} for e in missing)
    keys=sorted({edge[x] for edge in fixture['edges'] for x in ('source','target')})
    actual=graph.query('''MATCH (a:Entity {snapshot:$s})-[r:LINK]->(b:Entity {snapshot:$s})
        WHERE a.key IN $keys AND b.key IN $keys AND r.status='confirmed'
        RETURN a.key AS source,b.key AS target,r.kind AS kind,r.evidence AS evidence''',{'s':snapshot,'keys':keys})
    expected={(e['source'],e['target'],e['kind'],e['evidence']) for e in fixture['edges'] if e['status']=='confirmed'}
    unexpected=[row for row in actual if (row['source'],row['target'],row['kind'],row['evidence']) not in expected]
    failures.extend({'error':'Unexpected confirmed edge within curated chain','edge':row} for row in unexpected)
    checks.append({'kind':'curated_chain_preservation','missing':len(missing),
                   'unexpected_confirmed':len(unexpected),
                   'scope':'Preservation of reviewed annotations, not automatic discovery or new behavioral proof'})
    # A direct CALL is a machine-level reference; verify stored target against original encoding.
    con=db(True);direct=0
    for row in con.execute("SELECT * FROM refs WHERE version='B' AND kind='CALLS'"):
        raw=original_bytes(row['site'],5)
        if raw[0]!=0xe8 or row['site']+5+int.from_bytes(raw[1:],'little',signed=True)!=row['target']:
            failures.append({'error':'Direct CALL target mismatch','site':row['site']})
        direct+=1
    con.close()
    for archive in archives.values():archive.close()
    report={'ok':verification_ok(failures,checks),'full_blob_check':full,'checks':checks,'failures':failures,
            'direct_call_records_checked':direct,'snapshot':snapshot,'seconds':time.monotonic()-began,
            'source_binding':state,'coverage':coverage(),'semantic_complete':False}
    atomic(RUNTIME/('verification-full.json' if full else 'verification-chain.json'),report)
    return report

def audit_receipts():
    """Audit old validation file bindings; never promote historical passes to current."""
    root=WORKSPACE/'tiny-glade';reports=[]
    for name in ['m01-global-ray-validation.json','m01-rapier-producer-final-validation.json']:
        path=REPORTS/name;doc=load(path);files=[]
        for record in doc['files']:
            candidate=resolve_evidence(record['path'].replace('\\','/')) if Path(record['path']).is_absolute() else Path(record['path'].replace('\\','/'))
            candidate=(candidate if candidate.is_absolute() else root/candidate).resolve()
            status='missing'
            if candidate.is_relative_to(WORKSPACE) and candidate.is_file():
                status='match' if digest(candidate)==record['sha256'] else 'changed'
            files.append({'path':str(candidate),'expected_sha256':record['sha256'],'status':status})
        reports.append({'receipt':str(path),'historical_result':doc.get('result'),
                        'current_bindings_match':all(f['status']=='match' for f in files),'files':files})
    result={'audit_completed':True,'receipts':reports,
            'all_receipts_match_current_workspace':all(r['current_bindings_match'] for r in reports),
            'current_game_behavior_revalidated':False}
    atomic(RUNTIME/'receipt-audit.json',result)
    return {'audit_completed':True,'all_receipts_match_current_workspace':result['all_receipts_match_current_workspace'],
            'counts':[{k:sum(f['status']==k for f in r['files']) for k in ('match','changed','missing')} for r in reports],
            'report':str(RUNTIME/'receipt-audit.json')}

def freeze_evidence():
    folder=RUNTIME/'proof-cas';folder.mkdir(exist_ok=True);copied=0
    for record in load(CHAIN)['evidence']:
        raw=resolve_evidence(record['path']).read_bytes()
        if hashlib.sha256(raw).hexdigest()!=record['sha256']:
            raise ValueError('Evidence changed before freeze: '+record['path'])
        p=folder/record['sha256']
        if p.exists():
            if digest(p)!=record['sha256']:raise ValueError('Proof CAS integrity failure')
        else:
            with p.open('xb') as f:f.write(raw)
            copied+=1
    return {'proof_files_added':copied,'folder':str(folder)}

def tasks():
    """Manually selected questions, addresses and original-report anchors."""
    global_report=REPORTS/'m01-global-ray-recovery-2026-10-27.md'
    producer=REPORTS/'m01-rapier-producer-recovery-2026-10-27.md'
    roof=REPORTS/'m01-roof-trigger-floor-recovery-2026-10-27.md'
    def t(name,tool,args,expected,file,anchor):return dict(name=name,tool=tool,args=args,expected=expected,file=str(file),anchor=anchor)
    return [
      t('physical_address','search',{'text':'0x1408cdc60'},'function:B:8cdc60',global_report,'Rapier::raycast@1408cdc60'),
      t('wrapper_name','search',{'text':'RaycastWorld::raycast_w_terrain'},'symbol:B:pdb:496:38088',global_report,'raycast_w_terrain@1408cd430'),
      t('producer_state','neighbors',{'key':'function:B:8d0490','kinds':['WRITES']},'state:B:WorldRoofTable',producer,'+3df8'),
      t('state_consumers','state_users',{'key':'state:B:WorldRoofTable'},'function:B:8cdc60',global_report,'原17槽'),
      t('receipt_readers','state_users',{'key':'state:B:ColliderReceipt'},'function:B:8ce4a0',global_report,'receipt'),
      t('winner_contract','neighbors',{'key':'function:B:8cdc60','kinds':['HAS_CONTRACT']},'contract:B:winner_after_loop',global_report,'循环结束'),
      t('uv_terrain_contract','neighbors',{'key':'function:B:8cd430','kinds':['HAS_CONTRACT']},'contract:B:uv_hit_limits_terrain',global_report,'0/1'),
      t('option_contract','neighbors',{'key':'function:B:8ce4a0','kinds':['HAS_CONTRACT']},'contract:B:option_presence',global_report,'Some(empty)'),
      t('generation_contract','neighbors',{'key':'function:B:8cdc60','kinds':['HAS_CONTRACT']},'contract:B:generation_or_one',global_report,'raw_generation | 1'),
      t('release_contract','neighbors',{'key':'function:B:8d0490','kinds':['HAS_CONTRACT']},'contract:B:remove_release',producer,'Reload'),
      t('cross_module_state_path','paths',{'key':'function:B:21a3220','target':'function:B:8cdc60','depth':4,
            'kinds':['PRODUCES','CONSUMES','WRITES','READS']},'function:B:8cdc60',roof,'update_rapier_colliders1408d0490'),
      t('consumer_evidence_pack','evidence_pack',{'key':'function:B:8cc2e0','depth':2,'kinds':['CALLS']},'function:B:8cd430',
          WORKSPACE/'tiny-glade/crates/country-core/src/systems/collision/world.rs','0x1408cc2e0'),
    ]

def percentile(values,p):return sorted(values)[max(0,math.ceil(len(values)*p)-1)]

def benchmark(repetitions=30):
    from api import query_tool
    if repetitions<1:raise ValueError('Positive repetitions required')
    snapshot=Graph().current()
    if evidence_status()['original_evidence_errors']:raise ValueError('Original benchmark evidence changed')
    results=[];graph_total=[];baseline_total=[];graph_bytes=baseline_bytes=0
    for task in tasks():
        arguments=dict(task['args'],snapshot=snapshot)
        # Warm both sides first; this intentionally measures hot lookup, not reasoning time.
        first_began=time.perf_counter();first=query_tool(task['tool'],arguments);first_seconds=time.perf_counter()-first_began
        graph_ok=task['expected'] in canonical(first).decode()
        command=['rg','-n','-F','--',task['anchor'],task['file']]
        base=subprocess.run(command,capture_output=True,check=False)
        baseline_ok=base.returncode==0
        gs=[];bs=[];gb=bb=0
        for _ in range(repetitions):
            began=time.perf_counter();response=query_tool(task['tool'],arguments);gs.append(time.perf_counter()-began)
            gb=len(canonical(response))
            began=time.perf_counter();r=subprocess.run(command,capture_output=True,check=False);bs.append(time.perf_counter()-began)
            bb=len(r.stdout)
            if r.returncode!=0:baseline_ok=False
            if task['expected'] not in canonical(response).decode():graph_ok=False
        results.append({'name':task['name'],'graph_correct':graph_ok,'baseline_anchor_found':baseline_ok,
                        'first_graph_seconds':first_seconds,'graph_p50':statistics.median(gs),'graph_p95':percentile(gs,.95),
                        'baseline_p50':statistics.median(bs),'baseline_p95':percentile(bs,.95),
                        'graph_output_bytes':gb,'baseline_output_bytes':bb,'tool':task['tool']})
        graph_total.append(statistics.median(gs));baseline_total.append(statistics.median(bs));graph_bytes+=gb;baseline_bytes+=bb
    thresholds=all(r['graph_p95'] <= (3 if r['tool']=='paths' else 5 if r['tool']=='evidence_pack' else 1) for r in results)
    speedup=1-statistics.median(graph_total)/max(statistics.median(baseline_total),1e-9)
    report={'snapshot':snapshot,'task_definition_sha256':hashlib.sha256(canonical(tasks())).hexdigest(),
            'tasks':results,'repetitions':repetitions,'query_latency_targets_met':thresholds,
            'graph_correct':all(r['graph_correct'] for r in results),'baseline_anchors_found':all(r['baseline_anchor_found'] for r in results),
            'median_lookup_time_reduction':speedup,'graph_output_bytes':graph_bytes,'baseline_output_bytes':baseline_bytes,
            'end_to_end_agent_efficiency_acceptance':'not_established',
            'limitations':['Baseline is hot indexed report-anchor lookup, not an end-to-end reverse-engineering session.',
                          'Graph includes curated business annotations; this benchmark does not measure automatic semantic recovery.',
                          'No actual LLM token, manual step or conversational tool-roundtrip measurements were made.',
                          'First query is not an operating-system cold-cache measurement.'],
            'all_plan_efficiency_criteria_passed':False}
    atomic(RUNTIME/'benchmark.json',report)
    return report
