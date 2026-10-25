from pathlib import Path
import struct,subprocess,hashlib,json
out=Path('D:/game/reverse-engineering/tiny-glade/evidence/round12/menu-widget');exe=Path('D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe');raw=exe.read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
nt=struct.unpack_from('<I',raw,0x3c)[0];secs=nt+24+struct.unpack_from('<H',raw,nt+20)[0]
def at(addr,size):
 for i in range(struct.unpack_from('<H',raw,nt+6)[0]):
  v,va,s,p=struct.unpack_from('<IIII',raw,secs+40*i+8);r=addr-0x140000000
  if va<=r and r+size<=va+s:return raw[p+r-va:p+r-va+size]
 raise ValueError(hex(addr))
fs=[('pill-button',0x140fa0130,950),('label-str',0x14068bf00,187),('hash-value-typeid',0x1405bdf20,393),('back-button',0x1406811c0,813),('label-init',0x140f9d930,619),('pill-init',0x140f9de80,683)]
for name,a,s in fs:
 r=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d','--no-show-raw-insn','--start-address='+hex(a),'--stop-address='+hex(a+s),str(exe)],capture_output=True,text=True,check=True);(out/(name+'.asm')).write_text(r.stdout)
(out/'literal.json').write_text(json.dumps({'142a16e40':{'bytes':at(0x142a16e40,4).hex(),'f32':struct.unpack('<f',at(0x142a16e40,4))[0]},'sha256':hashlib.sha256(raw).hexdigest(),'functions':{n:{'address':hex(a),'size':s,'sha256':hashlib.sha256(at(a,s)).hexdigest()} for n,a,s in fs}},indent=2))

