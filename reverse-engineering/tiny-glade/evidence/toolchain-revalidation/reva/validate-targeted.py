import asyncio, json, time
from pathlib import Path
from mcp.client.session import ClientSession
from mcp.client.streamable_http import streamable_http_client
OUT=Path(__file__).parent
URL='http://127.0.0.1:8089/mcp/message'
P='/tiny-glade.exe'
records=[]
async def call(session,label,name,args):
 start=time.perf_counter()
 result=await session.call_tool(name,args)
 data=result.model_dump(mode='json',by_alias=True)
 rec={'label':label,'tool':name,'arguments':args,'elapsed':time.perf_counter()-start,'result':data}
 records.append(rec); (OUT/(label+'.json')).write_text(json.dumps(rec,indent=2))
 print(label,'error=',data.get('isError'),'seconds=',round(rec['elapsed'],3),flush=True)
 return rec
PCODE='''import json
from ghidra.app.decompiler import DecompInterface
from ghidra.util.task import TaskMonitor
dec=DecompInterface()
dec.openProgram(currentProgram)
rows=[]
for raw in ['140949c20','1421aaea0']:
 addr=currentProgram.getAddressFactory().getAddress(raw)
 fn=currentProgram.getFunctionManager().getFunctionAt(addr)
 r=dec.decompileFunction(fn,30,monitor)
 high=r.getHighFunction()
 ops=[]
 if high is not None:
  it=high.getPcodeOps()
  while it.hasNext() and len(ops)<30:
   op=it.next()
   ops.append({'address':str(op.getSeqnum().getTarget()),'opcode':str(op.getMnemonic()),'text':str(op)})
 rows.append({'address':str(addr),'name':str(fn.getName()),'signature':str(fn.getSignature()),'completed':bool(r.decompileCompleted()),'pcode':ops})
dec.dispose()
print(json.dumps({'program':str(currentProgram.getName()),'executableSHA256':str(currentProgram.getExecutableSHA256()),'rows':rows}))
'''
async def one(idx,kind):
 async with streamable_http_client(URL) as st:
  async with ClientSession(st[0],st[1]) as s:
   await s.initialize()
   if kind=='read':
    return await call(s,'parallel-read-'+str(idx),'get-decompilation',{'programPath':P,'functionNameOrAddress':['0x140949c20','0x1421aaea0'][idx%2],'limit':25,'includeReferenceContext':False})
   marker='CLIENT_'+str(idx)
   code="import time\nfor i in range(8):\n print('"+marker+"_'+str(i),flush=True)\n time.sleep(0.15)\n"
   return await call(s,'parallel-script-'+str(idx),'run-script',{'programPath':P,'code':code,'timeoutSeconds':20})
async def main():
 async with streamable_http_client(URL) as st:
  async with ClientSession(st[0],st[1]) as s:
   await s.initialize()
   await call(s,'aspect-decomp','get-decompilation',{'programPath':P,'functionNameOrAddress':'0x140949c20','limit':30,'includeCallers':True,'includeCallees':True,'includeReferenceContext':False})
   await call(s,'roof-xrefs','find-cross-references',{'programPath':P,'location':'0x1421aaea0','direction':'to','limit':5})
   await call(s,'type-float','get-data-type-by-string',{'programPath':P,'dataTypeString':'float'})
   await call(s,'pcode-two-functions','run-script',{'programPath':P,'code':PCODE,'timeoutSeconds':60})
 start=time.perf_counter(); reads=await asyncio.gather(*(one(i,'read') for i in range(4))); read_wall=time.perf_counter()-start
 start=time.perf_counter(); scripts=await asyncio.gather(*(one(i,'script') for i in range(3))); script_wall=time.perf_counter()-start
 (OUT/'parallel-summary.json').write_text(json.dumps({'readWall':read_wall,'scriptWall':script_wall,'readLabels':[r['label'] for r in reads],'scriptLabels':[r['label'] for r in scripts]},indent=2))
 (OUT/'targeted-validation-records.json').write_text(json.dumps(records,indent=2))
asyncio.run(main())
