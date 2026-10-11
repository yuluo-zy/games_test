import asyncio,json
from pathlib import Path
from mcp.client.session import ClientSession
from mcp.client.streamable_http import streamable_http_client
OUT=Path(__file__).parent
P='/tiny-glade.exe'
async def call(s,label,name,args):
 r=await s.call_tool(name,args);d=r.model_dump(mode='json',by_alias=True)
 (OUT/(label+'.json')).write_text(json.dumps({'tool':name,'arguments':args,'result':d},indent=2)); print(label,d.get('isError'),flush=True);return d
async def main():
 async with streamable_http_client('http://127.0.0.1:8089/mcp/message') as st:
  async with ClientSession(*st) as s:
   await s.initialize()
   code="""import json
addr=currentProgram.getAddressFactory().getAddress('140949c20')
f=currentProgram.getFunctionManager().getFunctionAt(addr)
rows=[]
it=currentProgram.getListing().getInstructions(f.getBody(),True)
while it.hasNext():
 i=it.next()
 rows.append({'address':str(i.getAddress()),'instruction':str(i),'pcode':[str(op) for op in i.getPcode()]})
print(json.dumps(rows))
"""
   await call(s,'aspect-instructions','run-script',{'programPath':P,'code':code})
   await call(s,'aspect-set-prototype','set-function-prototype',{'programPath':P,'location':'0x140949c20','signature':'float aspect_ratio(uint *window_size)','createIfNotExists':False})
   await call(s,'aspect-fixed-decomp','get-decompilation',{'programPath':P,'functionNameOrAddress':'0x140949c20','limit':30,'includeIncomingReferences':False,'includeReferenceContext':False})
   code="""import json
from ghidra.app.decompiler import DecompInterface
addr=currentProgram.getAddressFactory().getAddress('140949c20')
f=currentProgram.getFunctionManager().getFunctionAt(addr)
d=DecompInterface();d.openProgram(currentProgram)
r=d.decompileFunction(f,20,monitor)
ops=[];it=r.getHighFunction().getPcodeOps()
while it.hasNext():
 op=it.next();ops.append(str(op))
d.dispose()
print(json.dumps({'signature':str(f.getSignature()),'returnType':str(f.getReturnType()),'parameters':[str(p) for p in f.getParameters()],'pcode':ops}))
"""
   await call(s,'aspect-fixed-pcode','run-script',{'programPath':P,'code':code})
asyncio.run(main())
