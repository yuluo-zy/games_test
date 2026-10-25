import asyncio,json,time,os,subprocess
from pathlib import Path
from mcp import ClientSession,StdioServerParameters
from mcp.client.stdio import stdio_client
from discover import params
OUT=Path(__file__).parent
STATE=Path.home()/'Library/Application Support/re-mcp-ghidra'
env=dict(params.env)
proxy=StdioServerParameters(command=params.command,args=['proxy'],env=env)
report={'calls':[],'mode':'two independent stdio proxy clients, one test-owned persistent daemon','state_directory':str(STATE),'auto_analysis':False,'daemon_identity_per_client':[]}
barrier=asyncio.Barrier(2)
def unwrap(z):
 return z.structuredContent if z.structuredContent is not None else json.loads('\n'.join(c.text for c in z.content if c.type=='text'))
async def agent(w):
 async with stdio_client(proxy) as (r,writer):
  async with ClientSession(r,writer) as s:
   await s.initialize();catalog=await s.list_tools()
   state=json.loads((STATE/'daemon.json').read_text())
   report['daemon_identity_per_client'].append({'agent':w,'pid':state['pid'],'port':state['port'],'version':state.get('version')})
   async def call(n,a):
    t=time.perf_counter();z=await asyncio.wait_for(s.call_tool(n,a),timeout=150);v=unwrap(z)
    report['calls'].append({'agent':w,'tool':n,'args':a,'start':t,'end':time.perf_counter(),'isError':z.isError,'decoded':v})
    (OUT/'proxy-agents-session.json').write_text(json.dumps(report,indent=2))
    if z.isError:raise RuntimeError(str(v))
    return v
   db=f'b_{w}'
   await barrier.wait();await call('open_database',{'file_path':f'/Users/yuluo/Desktop/game/tools/mcp-servers/re-mcp/test-workers/worker-{w}/tiny-glade.exe','database_id':db,'run_auto_analysis':False,'force_new':False,'keep_open':True})
   await call('wait_for_analysis',{'database':db})
   await barrier.wait();listing=await call('list_databases',{})
   assert listing['database_count']==2,listing
   report.setdefault('catalog_counts',[]).append(len(catalog.tools))
   address={'a':'0x14090e510','b':'0x14090e6f0'}[w]
   await barrier.wait();decomp=await call('decompile_function',{'database':db,'address':address})
   assert decomp['database']==db and int(decomp['address'],16)==int(address,16),decomp
   await barrier.wait();await call('set_comment',{'database':db,'address':'0x14090e510','comment_type':'plate','comment':f're-mcp shared daemon agent {w} proof'})
   await barrier.wait();comment=await call('call',{'database':db,'tool':'get_comment','arguments':{'address':'0x14090e510'}})
   assert comment['result']['plate_comment']==f're-mcp shared daemon agent {w} proof',comment
   await barrier.wait();await call('save_database',{'database':db})
   await barrier.wait();await call('close_database',{'database':db,'save':True})
   await barrier.wait();await call('open_database',{'file_path':f'/Users/yuluo/Desktop/game/tools/mcp-servers/re-mcp/test-workers/worker-{w}/tiny-glade.exe','database_id':db,'run_auto_analysis':False,'force_new':False})
   await call('wait_for_analysis',{'database':db})
   await barrier.wait();persisted=await call('call',{'database':db,'tool':'get_comment','arguments':{'address':'0x14090e510'}})
   assert persisted['result']['plate_comment']==f're-mcp shared daemon agent {w} proof',persisted
   await barrier.wait();await call('close_database',{'database':db,'save':True})
   await barrier.wait()
   empty=await call('list_databases',{})
   assert empty['database_count']==0,empty
   return w
async def main():
 try:
  STATE.mkdir(parents=True,exist_ok=True)
  assert not (STATE/'daemon.json').exists(), 'An existing unrelated daemon must not be stopped by this test'
  await asyncio.gather(agent('a'),agent('b'))
  # Private XDG path is used only by this test. Do not record daemon auth token.
  metadata=[]
  for f in STATE.rglob('*.json'):
   try:
    j=json.loads(f.read_text())
    metadata.append({'path':str(f),'pid':j.get('pid'),'port':j.get('port'),'keys_only':list(j)})
   except:pass
  report['daemon_state_metadata']=metadata
  report['same_daemon_pid']=len({x['pid'] for x in report['daemon_identity_per_client']})==1
  assert report['same_daemon_pid']
  report['ok']=True
 except BaseException as e:
  report['ok']=False;report['error']=repr(e);raise
 finally:
  owned={x['pid'] for x in report['daemon_identity_per_client']}
  active=json.loads((STATE/'daemon.json').read_text()) if (STATE/'daemon.json').exists() else {}
  if len(owned)==1 and active.get('pid') in owned:
   process=await asyncio.create_subprocess_exec(params.command,'stop',env={**os.environ,**env},stdout=asyncio.subprocess.PIPE,stderr=asyncio.subprocess.PIPE)
   out,err=await process.communicate();report['stop']={'exit_code':process.returncode,'stdout':out.decode(),'stderr':err.decode()}
  (OUT/'proxy-agents-session.json').write_text(json.dumps(report,indent=2))
asyncio.run(main())
