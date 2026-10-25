"""Three independent MCP sessions, barrier-synchronised read calls and byte oracle."""
import asyncio, hashlib, json, time, struct, sys
from pathlib import Path
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client
OUT=Path(__file__).parent
PARAMS=StdioServerParameters(command='/Users/yuluo/Desktop/game/tools/venv/bin/python',args=['/Users/yuluo/.codex/mcp-tools/ghidra/launch.py'])
FUNCS=['0x14090e510','0x14090e6f0','0x140911520']
PROGRAM='tiny-glade.exe'
report={'actual_mcp':True,'sessions':3,'functions':FUNCS,'writes':False,'rounds':[],'session_catalogs':[],'discovered_instances':[]}
barrier=asyncio.Barrier(3)
def unwrap(z):
 if z.isError: raise RuntimeError(z.model_dump_json())
 texts=[b.text for b in z.content if b.type=='text']
 try:return json.loads('\n'.join(texts))
 except:return '\n'.join(texts)
def pe_bytes(address,length):
 p=Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.exe')
 with p.open('rb') as f:
  data=f.read()
 nt=struct.unpack_from('<I',data,0x3c)[0]
 count=struct.unpack_from('<H',data,nt+6)[0]
 opt=struct.unpack_from('<H',data,nt+20)[0]
 image=struct.unpack_from('<Q',data,nt+24+24)[0]
 rva=int(address,16)-image
 for i in range(count):
  off=nt+24+opt+i*40
  vs,va,size,raw=struct.unpack_from('<IIII',data,off+8)
  if va<=rva<va+max(vs,size): return data[raw+rva-va:raw+rva-va+length].hex()
 raise RuntimeError('PE section not found')
async def call(s,name,args,tag):
 t=time.perf_counter(); z=await asyncio.wait_for(s.call_tool(name,args),timeout=60)
 rec={'tag':tag,'tool':name,'args':args,'start':t,'end':time.perf_counter(),'result':z.model_dump(mode='json')}
 rec['duration_s']=rec['end']-t; rec['decoded']=unwrap(z)
 report['rounds'].append(rec)
 return rec['decoded']
async def worker(i):
 async with stdio_client(PARAMS) as (r,w):
  async with ClientSession(r,w) as s:
   await s.initialize()
   catalog=await s.list_tools()
   report['session_catalogs'].append({'session':i,'tools':[t.name for t in catalog.tools]})
   instances=await s.call_tool('list_instances',{})
   report['discovered_instances'].append({'session':i,'result':instances.model_dump(mode='json'),'decoded':unwrap(instances)})
   addr=FUNCS[i]
   steps=[('get_metadata',{'program':PROGRAM}),('get_functions',{'program':PROGRAM,'function':addr,'fields':'entry_point,signature,body_start,body_end,decompiled_code','include_call_context':False}),('get_xrefs_to',{'program':PROGRAM,'address':addr,'limit':8}),('get_xrefs_from',{'program':PROGRAM,'address':addr,'limit':8}),('read_memory',{'program':PROGRAM,'address':addr,'length':32})]
   last_incoming=[]
   # Baseline runs are also synchronised; repeated runs detect state cross-talk.
   for round_no in range(3):
    for n,a in steps:
     await barrier.wait()
     if n=='get_xrefs_from' and last_incoming:
      a={**a,'address':last_incoming[0]['from_address']}
     decoded=await call(s,n,a,f'session-{i}/round-{round_no}')
     if n=='get_xrefs_to':
      last_incoming=sorted(decoded.get('references',[]),key=lambda x:x['type']=='DATA')
   return i
async def checks():
 procs=await asyncio.gather(*[asyncio.create_subprocess_exec('/Users/yuluo/Desktop/game/tools/venv/bin/python','/Users/yuluo/.codex/mcp-tools/ghidra/launch.py','--check',stdout=asyncio.subprocess.PIPE,stderr=asyncio.subprocess.PIPE) for _ in range(4)])
 zs=await asyncio.gather(*[p.communicate() for p in procs])
 report['launcher_checks']=[{'exit':p.returncode,'stdout':o.decode(),'stderr':e.decode()} for p,(o,e) in zip(procs,zs)]
 report['launcher_states']=[json.loads(x['stdout']) for x in report['launcher_checks'] if x['exit']==0]
 report['same_backend_pid']=len(report['launcher_states'])==4 and len({x['pid'] for x in report['launcher_states']})==1
async def main():
 try:
  await checks(); await asyncio.gather(*[worker(i) for i in range(3)])
  report['pe_byte_oracles']={a:pe_bytes(a,32) for a in FUNCS}
  validations=[]
  for i,addr in enumerate(FUNCS):
   rs=[r for r in report['rounds'] if r['tag'].startswith(f'session-{i}/')]
   metadata=[r['decoded'] for r in rs if r['tool']=='get_metadata']
   bundles=[r['decoded'] for r in rs if r['tool']=='get_functions']
   mem=[r['decoded'] for r in rs if r['tool']=='read_memory']
   outgoing=[r['decoded'] for r in rs if r['tool']=='get_xrefs_from']
   valid={'function':addr,'metadata_identity':all(x['program_name']==PROGRAM and '/ljxsj_92385/' in x['executable_path'] for x in metadata),'bundle_identity':all(int(x['entry_point'],16)==int(addr,16) for x in bundles),'nonempty_decompilation':all(x.get('decompiled_code','').strip() for x in bundles),'xref_reciprocal':all(any(int(ref.split()[1],16)==int(addr,16) for ref in x.get('references',[])) for x in outgoing),'pe_bytes_match':all(x['hex']==report['pe_byte_oracles'][addr] for x in mem),'stable_results':all(len({json.dumps(r['decoded'],sort_keys=True) for r in rs if r['tool']==name})==1 for name in ['get_functions','get_xrefs_to','get_xrefs_from','read_memory'])}
   validations.append(valid)
  batches=[]
  for round_no in range(3):
   for name in ['get_metadata','get_functions','get_xrefs_to','get_xrefs_from','read_memory']:
    rs=[r for r in report['rounds'] if r['tag'].endswith(f'/round-{round_no}') and r['tool']==name]
    batches.append({'round':round_no,'tool':name,'start_spread_s':max(x['start'] for x in rs)-min(x['start'] for x in rs),'all_three_outstanding_overlap_s':min(x['end'] for x in rs)-max(x['start'] for x in rs)})
  report['validation']=validations
  report['default_route_single_instance']=all(len(x['decoded']['instances'])==1 and x['decoded']['instances'][0].get('pid')==report['launcher_states'][0]['pid'] for x in report['discovered_instances'])
  report['batches']=batches
  report['distinct_decompiled_results']=len({r['decoded']['decompiled_code'] for r in report['rounds'] if r['tool']=='get_functions'})==3
  report['ok']=report['default_route_single_instance'] and report['same_backend_pid'] and report['distinct_decompiled_results'] and all(all(v for k,v in d.items() if k!='function') for d in validations) and all(b['all_three_outstanding_overlap_s']>0 for b in batches)
  if not report['ok']:raise AssertionError('See validation report')
 except BaseException as e:
  report['ok']=False;report['error']=repr(e); raise
 finally:
  dest=OUT/(sys.argv[1] if len(sys.argv)>1 else 'parallel-default.json')
  dest.write_text(json.dumps(report,indent=2)); print(json.dumps({'ok':report['ok'],'output':str(dest),'requests':len(report['rounds'])}),flush=True)
asyncio.run(main())
