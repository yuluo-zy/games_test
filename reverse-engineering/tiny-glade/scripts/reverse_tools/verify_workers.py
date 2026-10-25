"""Verify seed clone startup and simultaneous clients with real MCP calls."""
import json
from pathlib import Path
import subprocess
import time
from concurrent.futures import ThreadPoolExecutor
from common import PYTHON, ROOT, atomic_json, worker_dir
from verify_runtime import OP, OUT, checks, control, job, native_stdout

def main():
    before=time.monotonic()
    with ThreadPoolExecutor(max_workers=2) as pool:
        futures=[pool.submit(job,'seed-clone-'+str(i),'ghidra','tool-repair-clone-'+str(i),
            [{'tool':'get_metadata','args':{'program':'tiny-glade.exe'}}]) for i in range(2)]
        reports=[f.result() for f in futures]
    assert reports[0]['pid']!=reports[1]['pid']
    assert reports[0]['project']!=reports[1]['project']
    for i,report in enumerate(reports):
        assert 'bootstrap' not in report
        assert (worker_dir('tool-repair-clone-'+str(i))/'seed-origin.json').exists()
    parallel_wall=round(time.monotonic()-before,3)
    source='import json,time\na=time.time()\ntime.sleep(1)\nprint(json.dumps({"begin":a,"end":time.time()}))'
    with ThreadPoolExecutor(max_workers=2) as pool:
        futures=[pool.submit(job,'same-worker-'+str(i),'reva','tool-repair-clone-0',
            [{'tool':'run-script','args':{'programPath':'/tiny-glade.exe','code':source}}],write=True) for i in range(2)]
        serialized=[native_stdout(f.result()) for f in futures]
    first,second=sorted(serialized,key=lambda x:x['begin'])
    assert first['end']<=second['begin'],serialized
    for i in range(2):control('tool-repair-clone-'+str(i),'--stop')
    atomic_json(OUT/'workers-summary.json',{'ok':True,'checks':checks,'seed_clone_parallel_wall_seconds':parallel_wall,
        'same_worker_intervals':serialized,'limits':['Request overlap proves isolation, not a speedup multiplier.']})
    print('Worker verification passed',flush=True)

if __name__=='__main__':main()
