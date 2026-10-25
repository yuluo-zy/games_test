"""Trace initialize only; prints no authentication headers."""
import asyncio
import json
import sys
import httpx
from mcp import ClientSession
from mcp.client.streamable_http import streamable_http_client
from common import worker_dir

async def main():
    s=json.loads((worker_dir(sys.argv[1])/'state.json').read_text(encoding='utf-8'))
    async def request(r):
        print('REQUEST', r.method, r.content.decode(), flush=True)
    async def response(r):
        print('RESPONSE', r.status_code, r.headers.get('content-type'), flush=True)
    async with httpx.AsyncClient(headers={'X-API-Key':s['api_key']},timeout=int(sys.argv[2]) if len(sys.argv)>2 else 10,
            event_hooks={'request':[request],'response':[response]},
            limits=httpx.Limits(max_keepalive_connections=0)) as client:
        async with streamable_http_client(f'http://127.0.0.1:{s["port"]}/mcp/message', http_client=client) as (read,write,_):
            async with ClientSession(read,write) as session:
                print(await asyncio.wait_for(session.initialize(), 10), flush=True)

asyncio.run(main())
