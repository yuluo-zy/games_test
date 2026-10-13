"""Bounded transport diagnosis; never prints backend credentials."""
import json
import sys
import urllib.request
from common import worker_dir
s = json.loads((worker_dir(sys.argv[1]) / 'state.json').read_text(encoding='utf-8'))
data = json.dumps({'jsonrpc':'2.0','id':1,'method':'initialize','params':{
    'protocolVersion':sys.argv[2] if len(sys.argv)>2 else '2025-06-18','capabilities':{},'clientInfo':{'name':'repair-probe','version':'1'}}}).encode()
request = urllib.request.Request(f'http://127.0.0.1:{s["port"]}/mcp/message', data=data,
    headers={'X-API-Key':s['api_key'],'Content-Type':'application/json','Accept':'application/json,text/event-stream'})
with urllib.request.urlopen(request, timeout=5) as response:
    print(response.status, response.headers.get('Content-Type'))
    print(response.read(800).decode())
