from pathlib import Path
import hashlib,json,re,struct,subprocess
ROOT=Path('D:/game'); OUT=Path(__file__).parent
EXE=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne'; raw=EXE.read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
nt=struct.unpack_from('<I',raw,0x3c)[0]; n=struct.unpack_from('<H',raw,nt+6)[0]; opt=struct.unpack_from('<H',raw,nt+20)[0]
sects=[struct.unpack_from('<IIII',raw,nt+24+opt+i*40+8) for i in range(n)]
def at(a,n):
 r=a-0x140000000
 for vs,va,rs,rp in sects:
  if va<=r and r+n<=va+rs:return raw[rp+r-va:rp+r-va+n]
 raise ValueError(hex(a))
symbols={}; functions={}
for line in (ROOT/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').read_text(encoding='utf8').splitlines():
 p=line.split('\t')
 try:a=int(p[0],16); size=int(p[1])
 except:continue
 symbols[a]=p[-1]
 if 'tiny_glade::gui::gui_escape_menu' in p[-1]:functions[a]=(size,p[-1])
calls={}; refs={}; identity={}
for a,(size,symbol) in functions.items():
 m=re.search(r'closure_env\$(\d+)',symbol); leaf=re.search(r'::closure\$(\d+)$',symbol)
 name=('slot-'+m[1]) if m else ('closure-'+leaf[1]) if leaf else symbol.split('::')[-1]
 result=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d','--no-show-raw-insn','--start-address='+hex(a),'--stop-address='+hex(a+size),str(EXE)],capture_output=True,text=True,check=True).stdout
 (OUT/(name+'.asm')).write_text(result,encoding='utf8')
 calls[name]=[dict(site='0x'+site,target='0x'+target,symbol=symbols.get(int(target,16))) for site,target in re.findall(r'^\s*([a-f0-9]+):\s+(?:callq?|jmp)\s+0x([a-f0-9]+)',result,re.M)]
 for ptr in re.findall(r'# (0x[0-9a-f]+)',result):
  addr=int(ptr,16)
  if 0x142925000<=addr<0x143600000:
   try:b=at(addr,180)
   except:continue
   refs[ptr]=dict(hex=b.hex(),text=b.decode('utf8',errors='backslashreplace'))
 identity[name]=dict(address=hex(a),size=size,symbol=symbol,sha256=hashlib.sha256(at(a,size)).hexdigest())
(OUT/'call-targets.json').write_text(json.dumps(calls,ensure_ascii=False,indent=2),encoding='utf8')
(OUT/'rip-data.json').write_text(json.dumps(refs,ensure_ascii=False,indent=2),encoding='utf8')
(OUT/'identity.json').write_text(json.dumps(dict(exe=str(EXE),sha256=hashlib.sha256(raw).hexdigest(),functions=identity),indent=2),encoding='utf8')
print(json.dumps({'count':len(functions),'names':list(identity)},indent=2))
