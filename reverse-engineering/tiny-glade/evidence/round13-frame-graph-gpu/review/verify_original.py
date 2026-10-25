import struct,json,hashlib,pathlib
root=pathlib.Path('D:/game');out=root/'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/review';p=root/'ljxsj_92385/Tiny Glade/tiny-glade.rne';b=p.read_bytes();pe=struct.unpack_from('<I',b,0x3c)[0];opt=pe+24;n=struct.unpack_from('<H',b,pe+6)[0];oh=struct.unpack_from('<H',b,pe+20)[0];sections=[]
for i in range(n):
 o=opt+oh+i*40;name=b[o:o+8].rstrip(b'\0').decode();vs,va,rs,raw=struct.unpack_from('<IIII',b,o+8);sections.append((name,va,vs,raw,rs))
def off(rva):
 for name,va,vs,raw,rs in sections:
  if va<=rva<va+max(vs,rs):return raw+rva-va
 raise ValueError(hex(rva))
rva,size=struct.unpack_from('<II',b,opt+112+3*8);rows=[]
for i in range(size//12):
 a,z,u=struct.unpack_from('<III',b,off(rva)+i*12)
 if a<=0x161cd0<z:rows.append({'start':hex(0x140000000+a),'end':hex(0x140000000+z),'size':z-a,'unwind':hex(u),'sha256':hashlib.sha256(b[off(a):off(a)+(z-a)]).hexdigest()})
old=json.loads((root/'reverse-engineering/tiny-glade/skeleton/task-index/work/TG-379258a80656c7937d/main-chain-frame-20261018/os-evidence/native-world/compile-main.json').read_text());m=[]
for ins in old['instructions']:
 r=int(ins['rva'],16);v=bytes.fromhex(ins['bytes'])
 if b[off(r):off(r)+len(v)]!=v:m.append(ins['va'])
report={'artifact':str(p),'sha256':hashlib.sha256(b).hexdigest(),'unwind_cover':rows,'old_instruction_count':len(old['instructions']),'old_instruction_mismatch':m,'old_range':old['range'],'target_executed':False};(out/'original-identity-range.json').write_text(json.dumps(report,indent=2));print(json.dumps(report,indent=2))
