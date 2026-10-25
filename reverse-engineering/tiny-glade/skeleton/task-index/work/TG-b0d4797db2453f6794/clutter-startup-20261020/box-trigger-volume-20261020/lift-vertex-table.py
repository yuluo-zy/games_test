"""Bounded symbolic/numeric lane lift of retained1302B pre-map construction. Not native execution."""
from pathlib import Path
import json,re,struct
w=Path(__file__).parent
pack=json.loads((w/'native-pack/actual-box-trigger-volume.json').read_text())
assert pack['status']=='ok'
reg={'xmm1':[2,None,None,None],'xmm2':[4,None,None,None],'xmm3':[6,None,None,None]}
constants={0x142aa5d60:[-.5,-.5,.5,.5],0x142a8cc80:[.5,-.5,0,0]}
mem={};sites={}
def vals(name):return reg[name.strip().lstrip('%')][:]
def store(addr,v,pc):
 for i,a in enumerate(v):mem[addr+4*i]=a;sites[addr+4*i]=pc
for ins in pack['instructions']:
 pc=int(ins['va'],16)
 if pc<0x140ada128 or pc>0x140ada2bc:continue
 asm=ins['assembly'].split('#')[0].strip()
 mnemonic,_,args=asm.partition('\t')
 if mnemonic.startswith('mov') or mnemonic in ['unpcklps','unpckhpd','shufps','mulps']:
  # RIP data is the two retained constant vectors, not arbitrary process memory.
  if mnemonic=='mulps' and '(%rip)' in args:
   ref=int(re.search(r'# (0x[0-9a-f]+)',ins['assembly']).group(1),16)
   dst=args.rsplit(',',1)[1].strip().lstrip('%');reg[dst]=[a*b if a is not None else None for a,b in zip(reg[dst],constants[ref])];continue
  if '(%rsp)' in args:
   if not args.startswith('%xmm'):continue
   m=re.search(r'(0x[0-9a-f]+)\(%rsp\)',args)
   if not m:continue
   addr=int(m.group(1),16);src=vals(args.split(',')[0]);n={'movaps':4,'movups':4,'movss':1,'movlpd':2,'movlps':2}.get(mnemonic)
   if n:store(addr,src[:n],ins['va'])
   continue
  if mnemonic=='movq':continue
  parts=[x.strip() for x in args.split(',')]
  if mnemonic=='shufps':
   imm=int(parts[0].lstrip('$'),16);a=vals(parts[2]);b=vals(parts[1]);reg[parts[2].lstrip('%')]=[a[imm&3],a[(imm>>2)&3],b[(imm>>4)&3],b[(imm>>6)&3]];continue
  if len(parts)!=2 or not parts[0].startswith('%xmm') or not parts[1].startswith('%xmm'):continue
  src=vals(parts[0]);dst=parts[1].lstrip('%');old=reg.get(dst,[None]*4)
  if mnemonic in ['movaps','movups']:reg[dst]=src
  elif mnemonic=='movshdup':reg[dst]=[src[1],src[1],src[3],src[3]]
  elif mnemonic=='movlhps':reg[dst]=old[:2]+src[:2]
  elif mnemonic=='unpcklps':reg[dst]=[old[0],src[0],old[1],src[1]]
  elif mnemonic=='unpckhpd':reg[dst]=old[2:]+src[2:]
  elif mnemonic=='movsd':reg[dst]=src[:2]+old[2:]
  else:raise ValueError((ins['va'],asm))
rows=[]
for i in range(24):
 addr=0x170+i*12;v=[mem[addr+j*4] for j in range(3)]
 assert all(x is not None for x in v),i
 names=[{1:'width*.5',-1:'width*-.5',2:'height*.5',-2:'height*-.5',3:'depth*.5',-3:'depth*-.5'}[x] for x in v]
 rows.append({'vertex':i,'stack':hex(addr),'xyz':v,'symbolic_xyz':names,'store_sites':[sites[addr+j*4] for j in range(3)]})
indices=[]
for ins in pack['instructions']:
 pc=int(ins['va'],16)
 if 0x140ada2c5<=pc<=0x140ada457:
  m=re.search(r'movq\s+\$(0x[0-9a-f]+), (0x[0-9a-f]+)\(%rsp\)',ins['assembly'])
  if m:indices.append((int(m.group(2),16),int(m.group(1),16),ins['va']))
indices.sort();assert len(indices)==36
assert [i[0] for i in indices]==list(range(0x50,0x170,8))
result={'scope':'Scalar lane lift of fully retained native instruction records with dimensions2/4/6 to identify exact face vertices and literal indices; not Rust/native execution. F32 dimension scaling factors are exact+.5/-.5.','vertices':rows,'indices':[i[1] for i in indices],'index_sites':[i[2] for i in indices]}
(w/'vertex-index-lift.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'vertices':[r['symbolic_xyz'] for r in rows],'indices':result['indices']},indent=2))
