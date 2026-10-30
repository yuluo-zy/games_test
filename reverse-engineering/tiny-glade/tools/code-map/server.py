"""MCP stdio: fixed read tools only, with the existing project's MCP runtime."""
import asyncio
import json
from mcp.server import Server
from mcp.server.stdio import stdio_server
from mcp.types import TextContent, Tool
from api import query_tool
from validation import evidence_status

TOOLS=['search','neighbors','paths','state_users','assembly','evidence_pack','module_map','coverage','unresolved']

def serve():
    app=Server('tiny-glade-code-map')
    @app.list_tools()
    async def list_tools():
        props={'text':{'type':'string'},'key':{'type':'string'},'target':{'type':'string'},
               'version':{'type':'string','enum':['A','B','C','unbound']},'snapshot':{'type':'string'},
               'limit':{'type':'integer','minimum':1,'maximum':200},'offset':{'type':'integer','minimum':0,'maximum':1000000},
               'depth':{'type':'integer','minimum':1,'maximum':6},'kinds':{'type':'array','items':{'type':'string'}}}
        props['direction']={'type':'string','enum':['out','in','both']}
        return [Tool(name=name,description='Read-only '+name+' with identity and evidence references. No arbitrary Cypher.',
                     inputSchema={'type':'object','properties':props,'additionalProperties':False}) for name in TOOLS]
    @app.call_tool()
    async def call_tool(name,arguments):
        if name not in TOOLS:raise ValueError('Unknown read tool')
        result=await asyncio.to_thread(query_tool,name,arguments or {})
        result['source_binding']=evidence_status()
        return [TextContent(type='text',text=json.dumps(result,ensure_ascii=False))]
    async def run():
        async with stdio_server() as (read,write):
            await app.run(read,write,app.create_initialization_options())
    asyncio.run(run())
