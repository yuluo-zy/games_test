import pathlib,struct,bisect,json,subprocess,re
r=pathlib.Path('D:/game');b=(r/'ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();p=struct.unpack_from('<I',b,60)[0];n=struct.unpack_from('<H',b,p+6)[0];s=p+24+struct.unpack_from('<H',b,p+20)[0];sects=[]
for i in range(n):
 off=s+40*i;vsize,rva,num,raw=struct.unpack_from('<IIII',b,off+8);sects.append((b[off:off+8].rstrip(b'\0').decode(),rva,raw,num))
pd=next(x for x in sects if x[0]=='.pdata');fs=[struct.unpack_from('<III',b,pd[2]+i) for i in range(0,pd[3]-11,12)];fm={a:(z,u) for a,z,u in fs}
helpers=[0x1405389b0,0x140335b30,0x1400cbb50,0x14015ccb0,0x140112b10,0x1405b0b70,0x14025fbd0,0x140531f10,0x140295030,0x1402dfe10,0x140252490,0x1405c6fb0,0x1404e6e90,0x140174db0,0x140538010,0x1404e7060,0x140176fe0,0x140293720,0x1402cfa60,0x14033def0,0x14033a430,0x140342790,0x14033a260,0x140002c90,0x14031a8a0,0x1402ad500,0x14050f480,0x140369050,0x140368e90,0x1400f50e0,0x140369210]
report=[]
for va in helpers:
 end=0x140000000+fm[va-0x140000000][0]
 text=subprocess.check_output(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d','--no-show-raw-insn',f'--start-address={hex(va)}',f'--stop-address={hex(end)}',str(r/'ljxsj_92385/Tiny Glade/tiny-glade.rne')],text=True)
 (r/f'reverse-engineering/tiny-glade/evidence/round12/wall-gpu/register-{va:x}.asm').write_text(text)
 strings=[]
 for a in re.findall(r'# (0x[0-9a-f]+)',text):
  target=int(a,16)-0x140000000
  sec=next((x for x in sects if x[1]<=target<x[1]+x[3]),None)
  if not sec:continue
  at=sec[2]+target-sec[1];v=b[at:at+250].split(b'\0')[0]
  if b'::' in v:
   value=v.decode(errors='replace')
   if len(value)>180:value=value[:180]
   strings.append(dict(address=a,text=value))
 report.append(dict(va=hex(va),end=hex(end),strings=strings,calls=re.findall(r'callq\s+(0x[0-9a-f]+)',text)))
print(json.dumps(report,indent=2));(r/'reverse-engineering/tiny-glade/evidence/round12/wall-gpu/register-helpers.json').write_text(json.dumps(report,indent=2))
