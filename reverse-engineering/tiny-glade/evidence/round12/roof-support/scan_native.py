from pathlib import Path
import struct, json, hashlib, bisect
OUT=Path(__file__).parent
data=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(data).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
nt=struct.unpack_from('<I',data,60)[0];n=struct.unpack_from('<H',data,nt+6)[0];osz=struct.unpack_from('<H',data,nt+20)[0];base=struct.unpack_from('<Q',data,nt+24+24)[0]
sections=[]
for i in range(n):
 p=nt+24+osz+40*i;name=data[p:p+8].split(b'\0')[0].decode();vs,va,sz,off=struct.unpack_from('<4I',data,p+8);sections.append(dict(name=name,va=base+va,offset=off,size=sz))
procs=[]
for line in Path('D:/game/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').read_text(encoding='utf8').splitlines():
 f=line.split('\t')
 try:a=int(f[0],16);sz=int(f[1])
 except (ValueError,IndexError):continue
 procs.append((a,sz,f[-1]))
procs.sort();starts=[r[0] for r in procs]
def hosts(a):
 i=bisect.bisect_right(starts,a);out=[]
 for x,sz,name in procs[max(0,i-120):i]:
  if x<=a<x+sz:out.append(dict(address=hex(x),size=sz,name=name))
 return out
def allmatch(needle):
 out=[]
 for sec in sections:
  p=sec['offset'];end=p+sec['size']
  while True:
   q=data.find(needle,p,end)
   if q<0:break
   a=sec['va']+q-sec['offset'];out.append(dict(address=hex(a),section=sec['name'],hosts=hosts(a) if sec['name']=='.text' else []));p=q+1
 return out
key=0x83773598f5ecfead
matches=allmatch(struct.pack('<Q',key))
rips=[];calls=[];text=next(s for s in sections if s['name']=='.text');t=data[text['offset']:text['offset']+text['size']]
for i in range(len(t)-7):
 # Common 64-bit RIP-relative LEA/MOV opcodes; candidate references, not a complete disassembler.
 if t[i] in (0x48,0x4c) and t[i+1] in (0x8d,0x8b) and t[i+2]&0xc7==5:
  a=text['va']+i;dst=a+7+struct.unpack_from('<i',t,i+3)[0]
  if dst in (0x142a9da0e,0x142c2e320):rips.append(dict(address=hex(a),target=hex(dst),hosts=hosts(a)))
 if t[i]==0xe8:
  a=text['va']+i;dst=a+5+struct.unpack_from('<i',t,i+1)[0]
  if dst in (0x1412c2650,0x14161a590,0x1412cb510,0x14162ef90):calls.append(dict(address=hex(a),target=hex(dst),hosts=hosts(a)))
identities={hex(v):allmatch(struct.pack('<Q',v)) for v in (0x75f56cda763c3d18,0x83756ac551763a0d,0x142c2ce40,0x142c2e320,0x142a9da0e)}
r=dict(original_sha256=sha,sections=sections,ssbo_key=hex(key),exact_key_occurrences=matches,name_and_descriptor_rip_candidates=rips,direct_call_candidates=calls,type_identity_and_pointer_occurrences=identities,limitations=['Raw immediate and common RIP encoding candidate scans are not full xref analysis; function hosts from all PROC ranges may include aliases.','Absence of a name/key is not proof no runtime producer exists.'])
(OUT/'native-references.json').write_text(json.dumps(r,indent=2),encoding='utf8')
print('key occurrences',len(matches),'rip candidates',len(rips),'call candidates',len(calls))
for x in matches+rips+calls:print(x['address'],x.get('target',''),[h['name'] for h in x['hosts']][:2])
