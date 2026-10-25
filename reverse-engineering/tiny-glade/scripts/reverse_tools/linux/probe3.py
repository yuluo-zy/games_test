import asyncio, sys, json
from mcp.client.streamable_http import streamablehttp_client
from mcp import ClientSession
async def main():
    async with streamablehttp_client(f"http://localhost:{sys.argv[1]}/mcp/message") as (r, w, _):
        async with ClientSession(r, w) as s:
            await s.initialize()
            for name, args in [("get-function-count",{"programPath":"/tiny-glade.exe"}),
                               ("get-decompilation",{"programPath":"/tiny-glade.exe","functionNameOrAddress":"0x14200c7f0","limit":25})]:
                res = await s.call_tool(name, args); print(name,res.isError,res.content[0].text[:1500],"\n")
asyncio.run(main())
