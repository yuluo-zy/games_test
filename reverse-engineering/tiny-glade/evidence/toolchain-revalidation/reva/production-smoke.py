import asyncio,json,time
from pathlib import Path
from mcp.client.session import ClientSession
from mcp.client.streamable_http import streamable_http_client
async def main():
 async with streamable_http_client('http://127.0.0.1:8089/mcp/message') as st:
  async with ClientSession(*st) as s:
   info=await s.initialize()
   start=time.perf_counter();r=await s.call_tool('get-decompilation',{'programPath':'/tiny-glade.exe','functionNameOrAddress':'0x140949c20','limit':20,'includeIncomingReferences':False,'includeReferenceContext':False})
   Path(__file__).with_name('production-smoke.json').write_text(json.dumps({'initialize':info.model_dump(mode='json',by_alias=True),'elapsed':time.perf_counter()-start,'result':r.model_dump(mode='json',by_alias=True)},indent=2))
asyncio.run(main())
