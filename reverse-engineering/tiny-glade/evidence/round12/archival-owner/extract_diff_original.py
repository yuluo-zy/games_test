from pathlib import Path
import subprocess,hashlib,json,re,struct
O=Path(__file__).resolve().parent;R=Path('D:/game');exe=R/'ljxsj_92385/Tiny Glade/tiny-glade.rne'
data=exe.read_bytes();pe=struct.unpack_from('<I',data,60)[0];base=struct.unpack_from('<Q',data,pe+48)[0]
opt=struct.unpack_from('<H',data,pe+20)[0];secs=[struct.unpack_from('<IIII',data,pe+24+opt+i*40+8) for i in range(struct.unpack_from('<H',data,pe+6)[0])]
def read(a,n):
 for _,start,size,raw in secs:
  if start<=a-base and a-base+n<=start+size:return data[raw+a-base-start:raw+a-base-start+n]
 raise ValueError(hex(a))
out=[]
for a,n in [(0x140b2aa50,967),(0x1414761a0,714),(0x1414745e0,1130),(0x140b2be70,1589),(0x140b2cb30,1106),(0x1415237c0,2696),(0x141257900,690)]:
 saved=O/f'{a:x}.asm'
 s=saved.read_bytes() if saved.exists() else subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',f'--start-address={a:#x}',f'--stop-address={a+n:#x}',str(exe)],check=True,stdout=subprocess.PIPE).stdout
 (O/f'{a:x}.asm').write_bytes(s);out.append({'va':hex(a),'size':n,'sha256':hashlib.sha256(read(a,n)).hexdigest(),'calls':re.findall(r'callq\s+(0x[0-9a-f]+)',s.decode())})
strings={}
for p in [0x142aeb540,0x142aeb620]:
 vals=[]
 for i in range(3):
  a,n=struct.unpack('<QQ',read(p+i*16,16))
  try:vals.append(read(a,n).decode('utf-8'))
  except:vals.append(None)
 strings[hex(p)]=vals
table=0x142c6c070
targets=[table+struct.unpack('<i',read(table+i*4,4))[0] for i in range(4)]
(O/'extract-diff-original.json').write_text(json.dumps({'original_sha256':hashlib.sha256(data).hexdigest(),'ranges':out,'panic_literals':strings,'state_eq_dispatch':[hex(x) for x in targets]},indent=2))
print(json.dumps(out,indent=2));print(strings);print([hex(x) for x in targets])
