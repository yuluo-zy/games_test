"""Record actual candidate checks and LLVM-confirmed original call boundaries."""
from pathlib import Path
import hashlib, json, os, re, subprocess

OUT=Path(__file__).resolve().parent
ROOT=OUT.parents[2]
workspace=ROOT/'reconstruction/shared-foundation'
inventory=json.loads((OUT/'inventory.json').read_text())
native=json.loads((OUT/'native-result.json').read_text())
assert native['ok'] and native['actual_mcp_connection']
functions=json.loads(native['results'][1]['stdout'])['functions']
assert len(functions)==8 and all(f['decompiler_completed'] for f in functions)
checks=[]
env=os.environ.copy();env.pop('TG_DISABLE_LANG_FALLBACK',None)
commands=[
    ('tests',['cargo','test','--locked','--offline','-p','lang-lib']),
    ('clippy',['cargo','clippy','--locked','--offline','-p','lang-lib','--all-targets','--','-D','warnings']),
    ('locale-policy',['cargo','run','--locked','--offline','-p','lang-lib','--example','verify_locale_policy','--',str(OUT/'probe-policy')]),
    ('original-assets',['cargo','run','--locked','--offline','-p','lang-lib','--example','verify_original_assets','--','D:/game/ljxsj_92385/Tiny Glade',str(OUT/'probe-original-assets')]),
]
for name,command in commands:
    process=subprocess.run(command,cwd=workspace,env=env,capture_output=True,text=True,encoding='utf-8',errors='replace')
    (OUT/f'{name}.log').write_text(process.stdout+process.stderr,encoding='utf-8')
    checks.append({'name':name,'command':command,'exit_code':process.returncode,'passed':process.returncode==0,'log':f'{name}.log'})
    print(name,process.returncode,flush=True)

# Validate representative consumer CALLs at real LLVM-decoded boundaries. The
# remaining raw E8 hits stay explicitly unverified, rather than counted as xrefs.
chosen=[];seen=set()
for call in inventory['raw_call_candidates']:
    owner=call['owner_public'] or ''
    key=(call['target'],tuple(call['owner_unwind'] or []))
    if not call['owner_unwind'] or key in seen:continue
    if not any(s in owner for s in ['lang_lib::','tiny_glade::','tg_gpu_selector','country_core..render','toolbar::mei::config']):continue
    if call['target_name']=='lang_lib::lookup' and sum(c['target_name']=='lang_lib::lookup' for c in chosen)>=8:continue
    begin,end=map(lambda x:int(x,16),call['owner_unwind'])
    if end-begin>30000:continue
    chosen.append(call);seen.add(key)
    if len(chosen)>=20:break
confirmed=[]
for i,call in enumerate(chosen):
    begin,end=call['owner_unwind']
    command=['C:/Program Files/LLVM/bin/llvm-objdump.exe','--disassemble','--start-address='+begin,'--stop-address='+end,'D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne']
    process=subprocess.run(command,capture_output=True,text=True,encoding='utf-8',errors='replace')
    pattern=rf"^{call['call_address'][2:]}:.*callq\s+{call['target']}\b"
    passed=process.returncode==0 and bool(re.search(pattern,process.stdout,re.M))
    (OUT/f'consumer-{i:02}.asm.txt').write_text(process.stdout,encoding='utf-8')
    confirmed.append({**call,'status':'llvm_original_instruction_boundary_confirmed' if passed else 'unconfirmed','passed':passed,'assembly':f'consumer-{i:02}.asm.txt'})
assert confirmed and all(c['passed'] for c in confirmed)
result={'baseline':'B','checks':checks,'passed':all(c['passed'] for c in checks),
        'native_mcp_batch_verified':True,'native_decompiled_functions':[{k:f.get(k) for k in ['address','body_bytes','prototype_is_not_verified_Rust_ABI']} for f in functions],
        'confirmed_representative_consumers':confirmed,'raw_call_candidates_total':inventory['call_candidate_count'],
        'original_game_native_behavior_executed':False,'full_original_abi_verified':False,
        'candidate_sources':[{ 'path':str(p.relative_to(ROOT)).replace('\\','/'),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in sorted((workspace/'crates/lang-lib').rglob('*.rs'))],
        'lock_sha256':hashlib.sha256((workspace/'Cargo.lock').read_bytes()).hexdigest()}
(OUT/'validation.json').write_text(json.dumps(result,ensure_ascii=False,indent=2),encoding='utf-8')
if not result['passed']:raise SystemExit(1)
print('confirmed consumers:',len(confirmed))
