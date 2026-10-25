from pathlib import Path
import hashlib, json, struct
import subprocess
root=Path('D:/game')
p=root/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe'
d=p.read_bytes(); sha=hashlib.sha256(d).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
nt=struct.unpack_from('<I',d,0x3c)[0]
count=struct.unpack_from('<H',d,nt+6)[0]
optional_size=struct.unpack_from('<H',d,nt+20)[0]
base=struct.unpack_from('<Q',d,nt+24+24)[0]
sections=[]
for n in range(count):
 off=nt+24+optional_size+n*40
 vsize,rva,size,ptr=struct.unpack_from('<IIII',d,off+8)
 flags=struct.unpack_from('<I',d,off+36)[0]
 sections.append((rva,size,ptr,flags))
def read(va,size):
 for rva,sz,ptr,_ in sections:
  if rva<=va-base<rva+sz: return d[ptr+va-base-rva:ptr+va-base-rva+size]
 raise ValueError(hex(va))
out=root/'tiny-glade/reports/business/2026-10-21-business-module-recovery/toolbar-input-evidence'
out.mkdir(parents=True,exist_ok=True)
def dump(name,va,size):
 b=read(va,size)
 result=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d','--x86-asm-syntax=intel','--start-address='+str(va),'--stop-address='+str(va+size),str(p)],capture_output=True,text=True,check=True)
 (out/(name+'.asm')).write_text(result.stdout)
 return {'name':name,'va':hex(va),'size':size,'sha256':hashlib.sha256(b).hexdigest()}
records=[dump('hide-ui',0x14233eb50,894),dump('screen-insert',0x14011d170,1341),
         dump('ui-update',0x140b2de60,7577),dump('get-ui-volume',0x140b2dc60,503),
         dump('toolbar-widget-producer',0x142338990,5496),
         dump('toolbar-replay-signifier',0x142339f10,604)]
assert records[2]['sha256']=='78070209e56873b445bcecd65c7b1ca67fe64a8e82065a3beff6b3b264c5b5c2'
assert records[3]['sha256']=='b0bd8122e4c3ccc3a8f30e201794c1b784f8ffdea8efd624bb7b6ab8e9adf955'
calls=[]
for rva,size,ptr,flags in sections:
 if not flags & 0x20000000: continue
 b=d[ptr:ptr+size]; sva=base+rva
 for off in range(len(b)-5):
  if b[off]==0xe8 and sva+off+5+struct.unpack_from('<i',b,off+1)[0]==0x14011d170:
   calls.append(hex(sva+off))
   records.append(dump('insert-call-'+hex(sva+off),sva+off-80,110))
(out/'manifest.json').write_text(json.dumps({'original':str(p),'sha256':sha,'kind':'static only; no original execution','insert_call_candidates':calls,'bodies':records},indent=2))
print(json.dumps(calls))
