from pathlib import Path
import pefile,capstone,json,hashlib
ROOT=Path('/Users/yuluo/Desktop/game')
f=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne'
data=f.read_bytes()
assert hashlib.sha256(data).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=data,fast_load=True)
syms={}
for line in (ROOT/'games_test/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').read_text().splitlines():
 ps=line.split('\t')
 if len(ps)>=5:
  try: syms[int(ps[0],16)]=ps[4]
  except ValueError: pass
for x in json.loads((ROOT/'games_test/reverse-engineering/tiny-glade/evidence/render-architecture/public-symbols.json').read_text()):
 if isinstance(x.get('section'),int) and 0<x['section']<=len(pe.sections):
  va=0x140000000+pe.sections[x['section']-1].VirtualAddress+x['offset']
  syms.setdefault(va,x.get('demangled',''))
cs=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);cs.detail=True
out=Path(__file__).parent/'evidence';out.mkdir(exist_ok=True)
for start,end,name in [(0x1404e8010,0x1404e8010+10114,'render'),(0x1401647b0,0x1401647b0+7519,'compile_loading'),(0x140161cd0,0x140161cd0+10961,'compile_main'),(0x140166510,0x140166510+4867,'compile_present')]:
 off=pe.get_offset_from_rva(start-0x140000000)
 lines=[];calls=[]
 for i in cs.disasm(data[off:off+end-start],start):
  target=None
  if i.mnemonic=='call' and i.operands[0].type==capstone.CS_OP_IMM: target=i.operands[0].imm
  suffix=' ; '+syms.get(target,'') if target else ''
  lines.append(f'{i.address:#x} {i.bytes.hex()} {i.mnemonic} {i.op_str}{suffix}')
  if target: calls.append({'site':hex(i.address),'target':hex(target),'symbol':syms.get(target,'')})
 (out/(name+'.asm')).write_text('\n'.join(lines))
 (out/(name+'-calls.json')).write_text(json.dumps(calls,indent=2))
 print(name,len(lines),len(calls))
 for c in calls:
  if c['symbol'].startswith(('tiny_glade::','country_core::')) and 'drop' not in c['symbol']:
   print(c['site'],c['symbol'][:175])
