"""Real backend checks: native bytes/code, retained output, failures, durable workers."""
import hashlib
import json
from pathlib import Path
import subprocess
import time
from common import PYTHON, ROOT, SHA256, atomic_json

OUT=ROOT/'evidence/toolchain-repair-2026-10-13'
OUT.mkdir(parents=True,exist_ok=True)
OP=Path(__file__).with_name('reverse_operator.py')
checks=[]

def job(label, backend, worker, steps, *, write=False, expected=0):
    jp=OUT/(label+'.job.json'); rp=OUT/(label+'.json')
    if rp.exists():
        history=OUT/'history'; history.mkdir(exist_ok=True)
        old=rp.read_bytes(); digest=hashlib.sha256(old).hexdigest()[:16]
        (history/(label+'-'+digest+'.json')).write_bytes(old)
        old_log=OUT/(label+'.log')
        if old_log.exists(): (history/(label+'-'+digest+'.log')).write_bytes(old_log.read_bytes())
    atomic_json(jp,steps)
    command=[str(PYTHON),str(OP),backend,str(jp),str(rp),'--worker',worker]
    if write:command+=['--write']
    tick=time.monotonic()
    run=subprocess.run(command,capture_output=True,text=True,encoding='utf-8',timeout=240)
    (OUT/(label+'.log')).write_text(run.stdout+run.stderr,encoding='utf-8')
    report=json.loads(rp.read_text(encoding='utf-8'))
    assert run.returncode==expected,(label,run.returncode,run.stdout,run.stderr)
    assert report['ok']==(expected==0),(label,report)
    checks.append({'label':label,'passed':True,'exit_code':run.returncode,'wall_seconds':round(time.monotonic()-tick,3),'report':rp.name})
    print(json.dumps(checks[-1]),flush=True)
    return report

def control(worker,action):
    run=subprocess.run([str(PYTHON),str(OP),'reva','--worker',worker,action],capture_output=True,text=True,timeout=80)
    assert run.returncode==0,(worker,action,run.stderr)
    return json.loads(run.stdout)

def native_stdout(report,index=0):
    return json.loads(report['results'][index]['stdout'])

def main():
    targets=json.loads((ROOT/'evidence/shared-foundation-2026-10-13/geometry/targets.json').read_text(encoding='utf-8'))['targets']
    rows=[r for r in targets if r['range_basis']=='pe_unwind_contiguous_range' and 60<=r['size']<=250][:2]
    assert len(rows)==2
    prepared=job('native-batch','ghidra','tool-repair',[
        {'tool':'get_metadata','args':{'program':'tiny-glade.exe'}},
        {'tool':'find_functions','args':{'program':'tiny-glade.exe','pattern':'utils::geometry::circle','limit':4}},
        {'tool':'prepare_functions','args':{'program':'tiny-glade.exe','targets':rows}},
        {'tool':'get_functions','args':{'program':'tiny-glade.exe','functions':','.join(r['address'] for r in rows),'fields':'signature,decompiled_code,callees'}},
        {'tool':'read_memory','args':{'program':'tiny-glade.exe','address':rows[0]['address'],'length':rows[0]['size']}}
    ],write=True)
    assert prepared['identity']['sha256']==SHA256
    data=native_stdout(prepared,3)['functions']
    assert all(r.get('decompiler_completed') and r['body_bytes']>1 and len(r.get('decompiled_code',''))>80 for r in data),data
    memory=native_stdout(prepared,4)
    assert hashlib.sha256(bytes.fromhex(memory['hex'])).hexdigest()==rows[0]['bytes_sha256']
    long=job('large-output','reva','tool-repair',[{'tool':'run-script','args':{'programPath':'/tiny-glade.exe','code':'print("X"*200000)','timeoutSeconds':30}}],write=True)
    assert long['results'][0]['stdout']=='X'*200000+'\n'
    payload=json.loads(long['results'][0]['result']['content'][0]['text'])
    assert not payload.get('stdoutTruncated')
    failure=job('nested-failure','reva','tool-repair',[{'tool':'run-script','args':{'programPath':'/tiny-glade.exe','code':'print("saved-before-failure")\nraise RuntimeError("intentional validation failure")','timeoutSeconds':30}}],write=True,expected=1)
    assert failure['results'][0]['stdout']=='saved-before-failure\n'
    truncated=job('stderr-truncation','reva','tool-repair',[{'tool':'run-script','args':{'programPath':'/tiny-glade.exe','code':'raise RuntimeError("E"*100000)','timeoutSeconds':30}}],write=True,expected=1)
    assert json.loads(truncated['results'][0]['result']['content'][0]['text'])['stderrTruncated']
    job('read-profile-rejection','reva','tool-repair',[{'tool':'run-script','args':{'programPath':'/tiny-glade.exe','code':'print("must not execute")'}}],expected=1)
    job('selector-rejection','reva','tool-repair',[{'tool':'read-memory','args':{'address':rows[0]['address'],'length':16}}],expected=1)
    address=rows[0]['address'].removeprefix('0x')
    marker='toolchain-repair durability probe 2026-10-13'
    source=f'''import json
from ghidra.program.model.listing import CodeUnit
a=currentProgram.getAddressFactory().getAddress({address!r})
tx=currentProgram.startTransaction("Durability probe")
try:
    currentProgram.getListing().setComment(a,CodeUnit.EOL_COMMENT,{marker!r})
finally:
    currentProgram.endTransaction(tx,True)
print(json.dumps({{"comment":currentProgram.getListing().getComment(CodeUnit.EOL_COMMENT,a)}}))
'''
    job('write-checkpoint','reva','tool-repair',[{'tool':'run-script','args':{'programPath':'/tiny-glade.exe','code':source}}],write=True)
    old_pid=prepared['pid']
    stopped=control('tool-repair','--stop')
    assert stopped['status']=='stopped' and not stopped['running'],stopped
    read_source=f'''import json
from ghidra.program.model.listing import CodeUnit
a=currentProgram.getAddressFactory().getAddress({address!r})
f=currentProgram.getFunctionManager().getFunctionAt(a)
print(json.dumps({{"comment":currentProgram.getListing().getComment(CodeUnit.EOL_COMMENT,a),"body_bytes":f.getBody().getNumAddresses()}}))
'''
    reopened=job('save-reopen','reva','tool-repair',[{'tool':'run-script','args':{'programPath':'/tiny-glade.exe','code':read_source}}],write=True)
    assert reopened['pid']!=old_pid and native_stdout(reopened)['comment']==marker
    assert native_stdout(reopened)['body_bytes']>1
    independent=job('worker-isolation','reva','tool-repair-isolated',[{'tool':'run-script','args':{'programPath':'/tiny-glade.exe','code':read_source}}],write=True)
    assert independent['project']!=reopened['project'] and independent['pid']!=reopened['pid']
    assert native_stdout(independent)['comment']!=marker
    assert native_stdout(independent)['body_bytes']==1
    warm=job('warm-reuse','ghidra','tool-repair',[{'tool':'get_metadata','args':{'program':'tiny-glade.exe'}}])
    assert warm['backend_reused'] and warm['pid']==reopened['pid']
    # Clean up only the verification annotation, retaining genuinely decoded code.
    cleanup=source.replace(repr(marker),'None')
    job('probe-cleanup','reva','tool-repair',[{'tool':'run-script','args':{'programPath':'/tiny-glade.exe','code':cleanup}}],write=True)
    control('tool-repair-isolated','--stop')
    atomic_json(OUT/'summary.json',{'ok':True,'checks':checks,'native_addresses':rows,
        'limits':['Windows owned projects are fresh B imports, not the historical Mac DB/overlays.',
                  'No original game runtime or business acceptance was performed.',
                  'No whole-program analysis or guessed prototype repair was performed.']})
    print('Runtime verification passed',flush=True)

if __name__=='__main__':main()
