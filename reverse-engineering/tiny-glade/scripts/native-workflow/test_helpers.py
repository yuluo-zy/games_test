from pathlib import Path
import subprocess,json,tempfile,hashlib,sys
root=Path(__file__).resolve().parents[2]
import argparse
p=argparse.ArgumentParser();p.add_argument('--output',type=Path,required=True);args=p.parse_args()
base=args.output.resolve();base.mkdir(parents=True,exist_ok=True)
art=base/'test-artifact.bin';art.write_bytes(b'fixture-only')
op=base/'fake-operator.py';op.write_text('''import sys,json
from pathlib import Path
steps=json.loads(Path(sys.argv[2]).read_text())
results=[]
for step in steps:
 payload={"executable_path":step["args"].get("path","fixture")}
 if step["tool"]=="get_functions":payload={"functions":{ref:{"name":"fixture","size":1,"signature":"void f(void)","decompiled_code":"void f(void){int x=1; return;}"} for ref in step["args"]["functions"].split(",")}}
 if step["args"].get("nested_failure"):payload={"success":False,"error":"fixture nested rejection"}
 results.append({"tool":step["tool"],"args":step["args"],"result":{"isError":False,"content":[{"type":"text","text":json.dumps(payload)}]}})
Path(sys.argv[3]).write_text(json.dumps({"ok":True,"actual_mcp_connection":True,"results":results}))
''')
plan={'interpreter':sys.executable,'operator':str(op),'backend':'ghidra','worker':'shared','artifact':{'path':str(art),'sha256':hashlib.sha256(art.read_bytes()).hexdigest()},'metadata_guard':{'step':0,'field':'executable_path','equals':str(art)},'steps':[{'tool':'get_metadata','args':{'path':str(art),'program':'fixture'}},{'tool':'get_functions','args':{'program':'fixture','functions':'0x1,0x2','fields':'signature,decompiled_code'}}]}
runner='/Users/yuluo/.codex/skills/native-tool-batch/scripts/run_batch.py'
cases=[]
for name,mutate,expected in [('success',lambda p:p['steps'].append(p['steps'][-1]),0),('nested-error',lambda p:p['steps'][1]['args'].update(nested_failure=True),1),('identity-mismatch',lambda p:p['artifact'].update(sha256='0'*64),1),('write-rejected',lambda p:p['steps'].append({'tool':'set_comment','args':{}}),1),('oversize-rejected',lambda p:p['steps'][1]['args'].update(functions=','.join('0x'+str(i) for i in range(21))),1)]:
 p=json.loads(json.dumps(plan));mutate(p);file=base/(name+'.json');file.write_text(json.dumps(p));out=base/name
 cp=subprocess.run([sys.executable,runner,str(file),str(out)],capture_output=True,text=True)
 report=json.loads((out/'summary.json').read_text());cases.append({'case':name,'expected_exit':expected,'exit':cp.returncode,'pass':cp.returncode==expected,'errors':report['errors'],'operator_processes':report['operator_processes'],'mcp_steps':report['mcp_steps'],'deduplicated_steps':report.get('deduplicated_steps')})
ledger={'target_version':'C','module':'fixture','versions':{'C':{'artifact_sha256':'1'*64}},'facts':[{'id':'fact','version':'C','kind':'behavior','evidence':str(art.relative_to(root))}],'links':[{'name':'pet','status':'implemented_unverified','fact_ids':['fact']}],'dependencies':[],'rust_files':[]}
for name,status,expected in [('ledger-unverified','implemented_unverified',0),('ledger-false-verified','verified',1)]:
 ledger['links'][0]['status']=status;file=base/(name+'.json');file.write_text(json.dumps(ledger));out=base/(name+'-report.json')
 cp=subprocess.run([sys.executable,'/Users/yuluo/.codex/skills/native-module-handoff/scripts/audit_handoff.py',str(file),str(root),str(out)],capture_output=True,text=True)
 cases.append({'case':name,'expected_exit':expected,'exit':cp.returncode,'pass':cp.returncode==expected,'errors':json.loads(out.read_text())['errors']})
(base/'summary.json').write_text(json.dumps({'fixture_only':True,'all_pass':all(c['pass'] for c in cases),'cases':cases},indent=2));print(json.dumps(cases));assert all(c['pass'] for c in cases)
