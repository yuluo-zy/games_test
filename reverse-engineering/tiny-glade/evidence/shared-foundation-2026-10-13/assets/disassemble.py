import json, pathlib, subprocess, struct, re
p=pathlib.Path(__file__).parent
binary=pathlib.Path(r'D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne')
b=binary.read_bytes(); pe=struct.unpack_from('<I',b,0x3c)[0]; n=struct.unpack_from('<H',b,pe+6)[0]; opt=struct.unpack_from('<H',b,pe+20)[0]; base=struct.unpack_from('<Q',b,pe+24+24)[0]
sections=[]
for i in range(n):
 s=pe+24+opt+i*40; vsize,rva,size,off=struct.unpack_from('<IIII',b,s+8); sections.append((base+rva,base+rva+size,off))
def read(a,n=150):
 for lo,hi,off in sections:
  if lo<=a<hi:return b[off+a-lo:off+a-lo+n]
 return b''
rows=json.loads((p/'functions.json').read_text()); names={r['va']:r['name'] for r in rows}
for r in rows:
 if not r['name'].startswith(('asset::','asset_pipe::')):continue
 a=int(r['va'],16)
 s=subprocess.check_output([r'C:\Program Files\LLVM\bin\llvm-objdump.exe','-d','--no-show-raw-insn',f'--start-address={a}',f'--stop-address={a+r["size"]}',str(binary)],text=True,encoding='utf8')
 out=[]
 for line in s.splitlines():
  m=re.search(r'# (0x[\da-f]+)',line)
  if m:
   q=read(int(m[1],16)); z=q.split(b'\0')[0]
   try:
    text=z.decode('utf8')
    if len(text)>2 and text.isprintable():line+=' ; '+repr(text[:100])
   except UnicodeError:pass
  m=re.search(r'callq\s+(0x[\da-f]+)',line)
  if m and m[1] in names:line+=' ; '+names[m[1]]
  out.append(line)
 (p/('asm-'+r['va']+'.txt')).write_text('\n'.join(out),encoding='utf8')
print('saved annotated original disassembly')
