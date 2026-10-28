import ast,asyncio,importlib.util,json,tempfile
from pathlib import Path
from contextlib import asynccontextmanager
root=Path('/Users/yuluo/.codex/mcp-tools')
code=ast.parse((root/'ghidra/bridge_entry.py').read_text());fn=next(x for x in code.body if isinstance(x,ast.FunctionDef) and x.name=='pinned_instances')
checks=[]
with tempfile.TemporaryDirectory(prefix='tool-repair-route-',dir='/tmp') as temporary:
 state=Path(temporary)/'state.json'
 valid={'project':'/verified/worker/TinyGlade.gpr','program':'tiny-glade.exe','database_sha256':'cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323','pid':22}
 ns={'json':json,'Path':Path,'state_path':str(state),'bound_project':valid['project'],'expected_pid':11,'original_discover':lambda:[{'pid':11},{'pid':22},{'pid':33}]}
 exec(compile(ast.Module(body=[fn],type_ignores=[]),'<bridge route guard>','exec'),ns)
 for key,value in [('valid',None),('project','/another/worker/TinyGlade.gpr'),('database_sha256','wrong'),('program','another.exe')]:
  candidate=dict(valid)
  if key!='valid':candidate[key]=value
  state.write_text(json.dumps(candidate))
  try:r=ns['pinned_instances']()
  except RuntimeError:assert key!='valid';checks.append({'case':key,'rejected':True})
  else:assert key=='valid' and r==[{'pid':22}];checks.append({'case':'same_worker_new_pid_only','passed':True})
import sys
sys.path.insert(0,str(root/'reva'))
spec=importlib.util.spec_from_file_location('repair_reva_proxy',root/'reva/proxy.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
calls=0
@asynccontextmanager
async def failing_client(url):
 global calls
 calls+=1
 raise OSError('injected connection loss')
 yield
m.streamablehttp_client=failing_client;m.ensure=lambda:{'url':'http://127.0.0.1/test'}
r=asyncio.run(m.call_upstream('set-function-prototype',{}))
assert calls==1 and r.isError and 'not replayed' in r.content[0].text
checks.append({'case':'write_transport_failure_not_replayed','attempts':calls,'error_explained':True})
Path('/tmp/tool-repair-routing-guards.json').write_text(json.dumps({'ok':True,'checks':checks},indent=2)+'\n')
print(json.dumps(checks))
