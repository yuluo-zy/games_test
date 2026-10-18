exec((__import__('pathlib').Path(__file__).resolve().parent/'extract_chain.py').read_text())
md.detail=True
cfg=[]
for va in [0x140ee2560,0x140652050]:
 for i in md.disasm((P/f'{va:x}.bin').read_bytes(),va):
  if i.mnemonic=='call' and i.operands[0].type==capstone.x86.X86_OP_IMM:
   target=i.operands[0].imm;alias=A.get(str(target-BASE),[])
   if alias and (alias[0]['name'].startswith('bevy_ecs::schedule::config::impl$4::into_configs') or target in [0x140ed7b00,0x140ee3870,0x140ee1d50]):
    cfg.append({'site':hex(i.address),'target':hex(target),'aliases':alias});extract(target)
(P/'specialized-config-targets.json').write_text(json.dumps(cfg,indent=2)+'\n')
va=0x142b97c40;b=pe.get_data(va-BASE,72);q=struct.unpack('<9Q',b);(P/'set-vtable.json').write_text(json.dumps({'va':hex(va),'bytes':b.hex(),'system_type':A.get(str(q[4]-BASE),[])},indent=2)+'\n')
rec=json.loads((P/'140ee2560.pdb.json').read_text());s=rec['name'].split('<',1)[1];dep=0
for i,c in enumerate(s):
 if c in '<(':dep+=1
 elif c in '>)':dep-=1
 elif c==',' and dep==0:s=s[:i];break
body=s[len('tuple$<'):-1];dep=0;pos=0;tu=[]
for i,c in enumerate(body):
 if c in '<(':dep+=1
 elif c in '>)':dep-=1
 elif c==',' and dep==0:tu.append(body[pos:i]);pos=i+1
tu.append(body[pos:]);(P/'typed-chain.json').write_text(json.dumps({'api':'chain_ignore_deferred','tuple':tu},indent=2)+'\n')
# FunctionSystem metadata names with explicit length literals.
names=[]
for f in P.glob('*.bin'):
 ins=list(md.disasm(f.read_bytes(),int(f.stem,16)))
 for k,i in enumerate(ins):
  for op in i.operands:
   if op.type==capstone.x86.X86_OP_MEM and op.mem.base==capstone.x86.X86_REG_RIP:
    v=i.address+i.size+op.mem.disp;data=pe.get_data(v-BASE,220)
    if data.startswith(b'system_clutter::'):
     for nxt in ins[k+1:k+6]:
      if nxt.mnemonic=='mov' and len(nxt.operands)==2 and nxt.operands[0].type==capstone.x86.X86_OP_MEM and nxt.operands[1].type==capstone.x86.X86_OP_IMM:
       n=nxt.operands[1].imm
       if 20<n<210 and all(32<=c<127 for c in data[:n]):names.append({'helper':f.stem,'lea':hex(i.address),'data':hex(v),'length':n,'name':data[:n].decode(),'bytes':data[:n].hex()});break
(P/'native-function-names.json').write_text(json.dumps(names,indent=2)+'\n')
print('tuple count',len(tu))
for r in names:print(r['helper'],r['name'])
for r in cfg:
 if r['target']=='0x140ed7b00':print('ADD',r['site'],r['aliases'][0]['name'][:150])
