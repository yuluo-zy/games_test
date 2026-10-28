import asyncio,json,os,subprocess,time
from pathlib import Path
from mcp import ClientSession,StdioServerParameters
from mcp.client.stdio import stdio_client
ROOT=Path('/Users/yuluo/Desktop/game');HOME_TOOLS=Path('/Users/yuluo/.codex/mcp-tools')
PYTHON=ROOT/'tools/mcp-servers/ghidra-mcp/.venv/bin/python'
async def command(argv,env):
 r=await asyncio.to_thread(subprocess.run,argv,env=env,capture_output=True,text=True)
 if r.returncode:raise RuntimeError((argv,r.stderr[-1500:]))
 return r
async def wait_stopped(pid):
 for _ in range(40):
  r=await asyncio.to_thread(subprocess.run,['/bin/ps','-p',str(pid),'-o','stat='],capture_output=True,text=True)
  if r.returncode or r.stdout.strip().startswith('Z'):return
  await asyncio.sleep(.25)
 raise RuntimeError('Own backend did not exit')
async def verify(backend,worker):
 env={**os.environ,'TG_MCP_WORKER':worker,'TG_MCP_ACCESS':'read'}
 launcher=HOME_TOOLS/('ghidra/launch.py' if backend=='ghidra' else 'reva/proxy.py')
 manager=HOME_TOOLS/('ghidra/launch.py' if backend=='ghidra' else 'reva/manage.py')
 state_path=HOME_TOOLS/backend/'runtimes'/worker/'state.json'
 call='get_metadata' if backend=='ghidra' else 'get-function-count'
 args={'program':'tiny-glade.exe'} if backend=='ghidra' else {'programPath':'/tiny-glade.exe','filterDefaultNames':False}
 async with stdio_client(StdioServerParameters(command=str(PYTHON),args=[str(launcher)],env=env)) as (read,write):
  async with ClientSession(read,write) as session:
   await session.initialize()
   before=await session.call_tool(call,args);assert not before.isError,before
   old=json.loads(state_path.read_text())
   await command([str(PYTHON),str(manager),'--stop'],env)
   await wait_stopped(old['pid'])
   if backend=='ghidra':await command([str(PYTHON),str(manager),'--check'],env)
   # 同一条 stdio 会话在后端重启后必须恢复；不重建客户端、不改路由到其他项目。
   after=await session.call_tool(call,args);assert not after.isError,after
   new=json.loads(state_path.read_text());assert old['pid']!=new['pid']
   if backend=='ghidra':
    probe=await session.call_tool('get_functions',{'program':'tiny-glade.exe','functions':'0x14090e510','fields':'signature,decompiled_code','include_call_context':False})
   else:probe=await session.call_tool('get-decompilation',{'programPath':'/tiny-glade.exe','functionNameOrAddress':'0x140949c20','offset':1,'limit':10,'includeIncomingReferences':False,'includeReferenceContext':False})
   assert not probe.isError,probe
   result={'backend':backend,'worker':worker,'old_pid':old['pid'],'new_pid':new['pid'],'same_stdio_client':True,'metadata_before':before.model_dump(mode='json'),'metadata_after':after.model_dump(mode='json'),'query_after_reconnect':probe.model_dump(mode='json')}
 await command([str(PYTHON),str(manager),'--stop'],env)
 return result
async def main():
 results=[]
 for backend,worker in [('ghidra','tool-repair-20261024'),('reva','tool-repair-pcode-20261024')]:
  results.append(await verify(backend,worker))
  print(json.dumps({k:v for k,v in results[-1].items() if k not in ['metadata_before','metadata_after','query_after_reconnect']}),flush=True)
 Path('/tmp/tool-repair-live-client-reconnect.json').write_text(json.dumps({'ok':True,'results':results},indent=2)+'\n')
asyncio.run(main())
