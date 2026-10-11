import asyncio,json
from pathlib import Path
from mcp import ClientSession,StdioServerParameters
from mcp.client.stdio import stdio_client
OUT=Path(__file__).parent
async def main():
 p=StdioServerParameters(command='/Users/yuluo/Desktop/game/tools/venv/bin/python',args=['/Users/yuluo/.codex/mcp-tools/ghidra/launch.py'])
 async with stdio_client(p) as (r,w):
  async with ClientSession(r,w) as s:
   await s.initialize()
   t=await s.list_tools()
   (OUT/'catalog.json').write_text(t.model_dump_json(indent=2))
   for n,a in [('get_metadata',{'program':'tiny-glade.exe'}),('find_functions',{'program':'tiny-glade.exe','name_contains':'tiny_glade','limit':6})]:
    z=await s.call_tool(n,a)
    (OUT/(n+'.json')).write_text(z.model_dump_json(indent=2))
    print(n,z.model_dump_json()[:6000])
asyncio.run(main())
