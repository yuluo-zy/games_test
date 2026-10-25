import json,struct,hashlib
from pathlib import Path
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
ROOT=Path(__file__).resolve().parents[2];b=json.loads((ROOT/'tracking/baselines.json').read_text())['baselines']['B'];binary=Path(b['binary']['path']).read_bytes();assert hashlib.sha256(binary).hexdigest()==b['binary']['sha256'];base=b['pe']['image_base'];sections=b['pe']['sections']
def bytes_at(rva,n):
 for s in sections:
  if s['rva']<=rva<s['rva']+s['raw_size']:return binary[s['raw_pointer']+rva-s['rva']:s['raw_pointer']+rva-s['rva']+n]
 raise ValueError(hex(rva))
pdata=next(s for s in sections if s['name']=='.pdata');unwind=[struct.unpack_from('<3I',binary,pdata['raw_pointer']+i)for i in range(0,pdata['raw_size']-11,12)];funcs=json.loads((ROOT/'evidence/multi-version/B-functions.json').read_text())['functions'];names={}
for f in funcs:names.setdefault(int(f['va'],16),[]).append(f['name'])
cs=Cs(CS_ARCH_X86,CS_MODE_64);entries=[0x140755690,0x140756370,0x141210110];result=[]
for start in entries:
 rows=[r for r in unwind if r[0]==start-base];assert len(rows)==1
 end=base+rows[0][1];instructions=list(cs.disasm(bytes_at(start-base,end-start),start));calls=[dict(address=hex(i.address),target=i.op_str,names=names.get(int(i.op_str,16),[]))for i in instructions if i.mnemonic=='call'and i.op_str.startswith('0x')]
 out=ROOT/f'evidence/building-gates/B-{start:x}-native.asm';out.write_text('\n'.join(f'{i.address:x} {i.mnemonic} {i.op_str}'for i in instructions)+'\n');result.append(dict(entry=hex(start),end=hex(end),unwind_body_bytes=end-start,names=names.get(start,[]),calls=calls,assembly_path=str(out.relative_to(ROOT))))
(ROOT/'evidence/building-gates/B-native-call-map.json').write_text(json.dumps(dict(binary_sha256=b['binary']['sha256'],functions=result),indent=2)+'\n')
print(json.dumps([dict(entry=f['entry'],bytes=f['unwind_body_bytes'],calls=len(f['calls']))for f in result]))
