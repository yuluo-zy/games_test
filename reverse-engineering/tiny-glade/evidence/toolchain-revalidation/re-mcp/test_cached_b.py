import asyncio,json,time,os,hashlib
from pathlib import Path
from mcp import ClientSession
from mcp.client.stdio import stdio_client
from discover import params
OUT=Path(__file__).parent
report={'calls':[],'auto_analysis':False,'worker_ids':['b_a','b_b'],'queries':[]}
def unwrap(z):
 if z.structuredContent is not None:return z.structuredContent
 try:return json.loads('\n'.join(c.text for c in z.content if c.type=='text'))
 except:return z.model_dump(mode='json')
async def main():
 try:
  async with stdio_client(params) as (r,w):
   async with ClientSession(r,w) as s:
    await s.initialize();await s.list_tools()
    async def call(n,a,allow_error=False):
     t=time.perf_counter();z=await asyncio.wait_for(s.call_tool(n,a),timeout=150)
     v=unwrap(z);rec={'tool':n,'args':a,'start':t,'end':time.perf_counter(),'isError':z.isError,'decoded':v,'raw':z.model_dump(mode='json')};report['calls'].append(rec)
     (OUT/'cached-b-after-fix.json').write_text(json.dumps(report,indent=2));print(n,a,z.isError,flush=True)
     if z.isError and not allow_error:raise RuntimeError(str(v))
     return v,z.isError
    async def open_worker(w):
     await call('open_database',{'file_path':f'/Users/yuluo/Desktop/game/tools/mcp-servers/re-mcp/test-workers/worker-{w}/tiny-glade.exe','database_id':f'b_{w}','run_auto_analysis':False,'force_new':False})
    await asyncio.gather(*[open_worker(w) for w in ['a','b']])
    await asyncio.gather(*[call('wait_for_analysis',{'database':f'b_{w}'}) for w in ['a','b']])
    await call('get_schema',{'tools':['get_comment','set_comment','get_type_info','decompile_function','get_xrefs_to'],'detail':'full'})
    barrier=asyncio.Barrier(2)
    async def reads(w):
     db=f'b_{w}';addr={'a':'0x14090e510','b':'0x14090e6f0'}[w]
     for n,a in [('get_database_info',{}),('decompile_function',{'address':addr}),('get_xrefs_to',{'address':addr,'limit':8}),('get_type_info',{'address':addr})]:
      await barrier.wait();await call(n,{**a,'database':db})
     await barrier.wait();await call('set_comment',{'database':db,'address':'0x14090e510','comment':f're-mcp independent worker {w} patched persistence proof','comment_type':'plate'})
     await barrier.wait();await call('call',{'database':db,'tool':'get_comment','arguments':{'address':'0x14090e510'}})
    await asyncio.gather(*[reads(w) for w in ['a','b']])
    save_results=await asyncio.gather(*[call('save_database',{'database':f'b_{w}'},allow_error=True) for w in ['a','b']])
    report['save_ok']=not any(err for _,err in save_results)
    await asyncio.gather(*[call('close_database',{'database':f'b_{w}','save':report['save_ok']},allow_error=True) for w in ['a','b']])
    if report['save_ok']:
     await asyncio.gather(*[open_worker(w) for w in ['a','b']]);await asyncio.gather(*[call('wait_for_analysis',{'database':f'b_{w}'}) for w in ['a','b']])
     await asyncio.gather(*[call('call',{'database':f'b_{w}','tool':'get_comment','arguments':{'address':'0x14090e510'}}) for w in ['a','b']])
     await asyncio.gather(*[call('close_database',{'database':f'b_{w}','save':True}) for w in ['a','b']])
    report['ok']=report['save_ok']
 except BaseException as e:
  report['ok']=False;report['error']=repr(e);raise
 finally:
  (OUT/'cached-b-after-fix.json').write_text(json.dumps(report,indent=2))
asyncio.run(main())
