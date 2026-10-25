import asyncio,json
from pathlib import Path
from mcp.client.session import ClientSession
from mcp.client.streamable_http import streamable_http_client
async def main():
 async with streamable_http_client('http://127.0.0.1:8089/mcp/message') as streams:
  async with ClientSession(streams[0],streams[1]) as s:
   info=await s.initialize(); print(info.model_dump_json())
   catalog=await s.list_tools()
   Path(__file__).with_name('tool-catalog.json').write_text(catalog.model_dump_json(indent=2))
   print('\n'.join(t.name for t in catalog.tools))
asyncio.run(main())
