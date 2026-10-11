import asyncio,json
from pathlib import Path
from mcp.client.session import ClientSession
from mcp.client.streamable_http import streamable_http_client
OUT=Path(__file__).parent
async def call(s,label,name,args):
 r=await s.call_tool(name,args);d=r.model_dump(mode='json',by_alias=True);(OUT/(label+'.json')).write_text(json.dumps({'tool':name,'arguments':args,'result':d},indent=2));print(label,d.get('isError'),flush=True);return json.loads(d['content'][0]['text'])
async def main():
 name='ReVaToolchainValidation_20261011.py'
 async with streamable_http_client('http://127.0.0.1:8089/mcp/message') as st:
  async with ClientSession(*st) as s:
   await s.initialize()
   await call(s,'persisted-signature','get-decompilation',{'programPath':'/tiny-glade.exe','functionNameOrAddress':'0x140949c20','includeIncomingReferences':False,'includeReferenceContext':False,'signatureOnly':True})
   created=await call(s,'script-write','write-script',{'scriptName':name,'code':'# @runtime PyGhidra\nprint("LIFECYCLE_BEFORE")\n'})
   await call(s,'script-list','list-scripts',{'nameFilter':name,'maxCount':5})
   await call(s,'script-read','read-script',{'scriptName':name,'limit':5})
   await call(s,'script-edit','edit-script',{'scriptName':name,'old_string':'LIFECYCLE_BEFORE','new_string':'LIFECYCLE_AFTER'})
   await call(s,'script-run-name','run-script',{'programPath':'/tiny-glade.exe','scriptName':name,'timeoutSeconds':15})
   print(created,flush=True)
   path=created.get('absolutePath') or created.get('scriptPath') or created.get('path')
   if path:
    f=Path(path)
    assert f.name==name
    f.unlink()
    print('removed',path,flush=True)
asyncio.run(main())
