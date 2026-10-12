import asyncio,json,time,os
from pathlib import Path
from mcp import ClientSession,StdioServerParameters
from mcp.client.stdio import stdio_client
OUT=Path(__file__).parent
params=StdioServerParameters(command='/Users/yuluo/Desktop/game/tools/mcp-servers/re-mcp/.venv/bin/re-mcp-ghidra',args=['stdio'],env={'GHIDRA_INSTALL_DIR':'/Users/yuluo/Desktop/game/tools/ghidra_12.1.4_PUBLIC','JAVA_HOME':'/Users/yuluo/Library/Java/JavaVirtualMachines/corretto-21.0.5/Contents/Home','PATH':os.environ['PATH']})
report={'calls':[],'analysis':False,'cloned_database':True}
async def main():
 async with stdio_client(params) as (r,w):
  async with ClientSession(r,w) as s:
   await s.initialize()
   c=await s.list_tools();(OUT/'initial-catalog.json').write_text(c.model_dump_json(indent=2));print('initial tools',len(c.tools),flush=True)
   async def call(n,a):
    t=time.monotonic();res=await asyncio.wait_for(s.call_tool(n,a),timeout=150)
    rec={'name':n,'args':a,'duration':time.monotonic()-t,'result':res.model_dump(mode='json')};report['calls'].append(rec);(OUT/'discovery-session.json').write_text(json.dumps(report,indent=2));print(n,json.dumps(rec)[:4000],flush=True);return res
   for w in ['a','b']:
    await call('open_database',{'file_path':f'/Users/yuluo/Desktop/game/tools/mcp-servers/re-mcp/test-workers/worker-{w}/tiny-glade.exe','database_id':f'b_{w}','run_auto_analysis':False,'force_new':False,'keep_open':True})
   await asyncio.gather(*[call('wait_for_analysis',{'database':f'b_{w}'}) for w in ['a','b']])
   c=await s.list_tools();(OUT/'live-catalog.json').write_text(c.model_dump_json(indent=2));print('live tools',len(c.tools),flush=True)
   await asyncio.gather(*[call('get_database_info',{'database':f'b_{w}'}) for w in ['a','b']])
   await call('list_databases',{})
if __name__=='__main__':asyncio.run(main())
