import asyncio,json,time
from pathlib import Path
from mcp import ClientSession
from mcp.client.stdio import stdio_client
from discover import params
OUT=Path(__file__).parent
report={'calls':[],'auto_analysis':False,'manual_save_in_first_session':False}
async def cycle(write):
 async with stdio_client(params) as (r,w):
  async with ClientSession(r,w) as s:
   await s.initialize();await s.list_tools()
   async def call(n,a):
    t=time.perf_counter();z=await asyncio.wait_for(s.call_tool(n,a),timeout=150)
    v=z.structuredContent or json.loads('\n'.join(c.text for c in z.content if c.type=='text'))
    report['calls'].append({'cycle':'write_disconnect' if write else 'reopen_verify','tool':n,'args':a,'duration':time.perf_counter()-t,'decoded':v,'isError':z.isError})
    (OUT/'automatic-close-session.json').write_text(json.dumps(report,indent=2))
    if z.isError:raise RuntimeError(str(v))
    return v
   for x in ['a','b']:
    await call('open_database',{'file_path':f'/Users/yuluo/Desktop/game/tools/mcp-servers/re-mcp/test-workers/worker-{x}/tiny-glade.exe','database_id':f'b_{x}','run_auto_analysis':False})
   await asyncio.gather(*[call('wait_for_analysis',{'database':f'b_{x}'}) for x in ['a','b']])
   if write:
    await asyncio.gather(*[call('set_comment',{'database':f'b_{x}','address':'0x14090e510','comment_type':'plate','comment':f're-mcp automatic disconnect persisted worker {x}'}) for x in ['a','b']])
    # Closing stdio supervisor invokes its automatic database save/close path.
   else:
    result=await asyncio.gather(*[call('call',{'database':f'b_{x}','tool':'get_comment','arguments':{'address':'0x14090e510'}}) for x in ['a','b']])
    for x,v in zip(['a','b'],result):
     assert v['result']['plate_comment']==f're-mcp automatic disconnect persisted worker {x}',v
    report['automatic_save_persisted_and_isolated']=True
    await asyncio.gather(*[call('close_database',{'database':f'b_{x}','save':True}) for x in ['a','b']])
async def main():
 try:
  await cycle(True);await cycle(False);report['ok']=True
 except BaseException as e:
  report['ok']=False;report['error']=repr(e);raise
 finally:(OUT/'automatic-close-session.json').write_text(json.dumps(report,indent=2))
asyncio.run(main())
