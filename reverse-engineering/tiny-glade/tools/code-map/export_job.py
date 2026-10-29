#!/usr/bin/env python3
"""Prepare an exclusive identity-bound ReVa export job for a caller-owned worker."""
import argparse
from pathlib import Path
import re
import subprocess
import json
from artifacts import RUNTIME,WORKSPACE,canonical,load

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--worker',required=True);a=p.parse_args()
    if not re.fullmatch('[A-Za-z0-9][A-Za-z0-9_-]{0,63}',a.worker) or a.worker=='shared':
        raise ValueError('A unique caller-owned worker name is required')
    RUNTIME.mkdir(parents=True,exist_ok=True)
    output=RUNTIME/'ghidra-functions.jsonl.gz'
    if output.exists():raise RuntimeError('Export already exists; validate/reuse it instead of overwriting')
    job=RUNTIME/f'{a.worker}-job.json';result=RUNTIME/f'{a.worker}-result.json'
    code='OUTPUT = '+repr(str(output))+'\n'+Path(__file__).with_name('export_ghidra.py').read_text()
    with job.open('xb') as f:f.write(canonical([{'tool':'run-script','args':{'programPath':'/tiny-glade.exe','code':code,'timeoutSeconds':300},'timeout_seconds':360}]))
    command=[str(WORKSPACE/'tools/mcp-servers/ghidra-mcp/.venv/bin/python'),
             '/Users/yuluo/.codex/mcp-tools/reverse-operator.py','reva',str(job),str(result),'--worker',a.worker,'--write']
    subprocess.run(command,check=True)
    report=load(result)
    if not report.get('ok'):raise RuntimeError('Export operator reported failure')
    for step in report['results']:
        if step['result'].get('isError'):raise RuntimeError('MCP export failed')
        for block in step['result'].get('content',[]):
            if block.get('type')=='text':
                data=json.loads(block['text'])
                if not data.get('success') or data.get('timedOut') or data.get('stdoutTruncated'):
                    raise RuntimeError('Export script failed or was incomplete')
    print(json.dumps({'job':str(job),'result':str(result),'export':str(output),
                      'worker':a.worker,'next':'Normally stop only this worker after export verification'}))

if __name__=='__main__':main()
