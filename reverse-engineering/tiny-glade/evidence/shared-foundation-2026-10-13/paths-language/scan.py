import pathlib,struct,re,json,subprocess
out=pathlib.Path(__file__).parent
b=pathlib.Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes(); pe=struct.unpack_from('<I',b,0x3c)[0];ns=struct.unpack_from('<H',b,pe+6)[0];os=struct.unpack_from('<H',b,pe+20)[0];sections=[]
for i in range(ns):
 x=pe+24+os+40*i;vs,rva,sz,raw=struct.unpack_from('<IIII',b,x+8);sections.append((rva,max(vs,sz),raw))
def read(va,n=160):
 rva=va-0x140000000
 for r,s,o in sections:
  if r<=rva<r+s:return b[o+rva-r:o+rva-r+n]
 return b''
refs=[]
for f in out.glob('*.asm.txt'):
 for h in dict.fromkeys(re.findall(r'# (0x[0-9a-f]+)',f.read_text(encoding='utf-8-sig'))):
  data=read(int(h,16));refs.append(dict(file=f.name,va=h,ascii=data.decode('utf-8',errors='replace').replace('\x00','\\0')))
(out/'rdata-refs.json').write_text(json.dumps(refs,indent=2,ensure_ascii=False),encoding='utf-8')
p=subprocess.Popen(['llvm-pdbutil','dump','--publics','D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb'],stdout=subprocess.PIPE,encoding='utf-8',errors='replace');active=None;found=[]
for line in p.stdout:
 if 'S_PUB32' in line:active=line.strip()
 elif active and 'addr =' in line:
  if any(k in active for k in ['theme_defs','lang_lib','_ZN5paths']):
   m=re.search(r'addr = (\d+):(\d+)',line);found.append(dict(record=active,va=hex(0x140001000+int(m.group(2))) if m and m.group(1)=='0001' else None))
  active=None
p.wait();(out/'publics.json').write_text(json.dumps(found,indent=2),encoding='utf-8')
print(json.dumps(refs,indent=2,ensure_ascii=False));print('publics',len(found))
