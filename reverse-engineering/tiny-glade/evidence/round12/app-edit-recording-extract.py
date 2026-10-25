import pathlib,struct,re,json,subprocess,hashlib
r=pathlib.Path('D:/game'); out=r/'reverse-engineering/tiny-glade/evidence/round12/app-edit-recording';out.mkdir(exist_ok=True)
b=(r/'ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();p=struct.unpack_from('<I',b,60)[0];s=p+24+struct.unpack_from('<H',b,p+20)[0];sects=[]
for i in range(struct.unpack_from('<H',b,p+6)[0]):
 a=s+40*i;sz,va,num,off=struct.unpack_from('<IIII',b,a+8);sects.append((b[a:a+8].rstrip(b'\0'),va,off,num))
def read(va,n):
 t=va-0x140000000;z=next(x for x in sects if x[1]<=t<x[1]+x[3]);return b[z[2]+t-z[1]:z[2]+t-z[1]+n]
pd=next(x for x in sects if x[0]==b'.pdata');fm={a:(z,u) for a,z,u in (struct.unpack_from('<III',b,pd[2]+i) for i in range(0,pd[3]-11,12))}
procs={}
for l in (r/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').read_text().splitlines():
 c=l.split('\t');
 try:a=int(c[0],16)
 except:continue
 procs.setdefault(a,[]).append(c[-1])
helpers=[0x1409438a0,0x14050d290,0x140335b30,0x140112d30,0x140322d40,0x14054e960,0x1400cbd40,0x140272c20]
rows=[]
for a in helpers:
 z=0x140000000+fm[a-0x140000000][0];asm=subprocess.check_output(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d','--no-show-raw-insn',f'--start-address={hex(a)}',f'--stop-address={hex(z)}',str(r/'ljxsj_92385/Tiny Glade/tiny-glade.rne')],text=True);(out/f'{a:x}.asm').write_text(asm)
 refs=[]
 for ref in re.findall(r'# (0x[0-9a-f]+)',asm):
  va=int(ref,16)
  try:v=read(va,240)
  except:continue
  ss=v.split(b'\0')[0]
  if b'::' in ss:refs.append({'va':ref,'string':ss.decode(errors='replace')})
  elif va<0x143000000 and va>=0x142900000:
   vs=struct.unpack('<'+'Q'*20,read(va,160));methods=[]
   for j,q in enumerate(vs):
    ns=procs.get(q,[]);interesting=[n for n in ns if not n.startswith('core::')]
    if interesting:methods.append({'slot':j,'va':hex(q),'names':interesting})
   if methods:refs.append({'va':ref,'vtable':methods})
 calls=[{'va':x,'names':procs.get(int(x,16),[])} for x in re.findall(r'callq\s+(0x[0-9a-f]+)',asm)]
 rows.append(dict(va=hex(a),size=z-a,sha256=hashlib.sha256(read(a,z-a)).hexdigest(),names=procs.get(a,[]),refs=refs,calls=calls))
(out/'native-registration.json').write_text(json.dumps(rows,indent=2))
for row in rows:
 print(row['va'],row['size'])
 for q in row['refs']:
  if 'string' in q:print(' ',q['va'],q['string'][:160])
 print('calls',','.join(x['va'] for x in row['calls']))
