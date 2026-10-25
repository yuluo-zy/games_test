import asyncio, sys, json
from mcp.client.streamable_http import streamablehttp_client
from mcp import ClientSession
port = sys.argv[1]
async def main():
    async with streamablehttp_client(f"http://localhost:{port}/mcp/message") as (r, w, _):
        async with ClientSession(r, w) as s:
            await s.initialize()
            tools = await s.list_tools()
            print(len(tools.tools), [t.name for t in tools.tools][:60])
            for name, args in [("list-project-files", {"folderPath":"/"}), ("get-function-count", {"programPath":"/tiny-glade.exe"}), ("get-memory-blocks", {"programPath":"/tiny-glade.exe"}), ("get-decompilation", {"programPath":"/tiny-glade.exe","functionNameOrAddress":"0x14090e510"})]:
                try:
                    res = await s.call_tool(name, args)
                    print(name, res.isError, str(res.content[0].text)[:500])
                except Exception as e:
                    print(name, "EXC", e)
asyncio.run(main())
