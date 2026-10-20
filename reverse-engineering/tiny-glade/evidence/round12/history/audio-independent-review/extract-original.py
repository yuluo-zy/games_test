from pathlib import Path
import struct,json,hashlib,subprocess
ROOT=Path('D:/game');OUT=Path(__file__).resolve().parent;image=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne';raw=image.read_bytes();pe=struct.unpack_from('<I',raw,0x3c)[0];opt=pe+24;base=struct.unpack_from('<Q',raw,opt+24)[0]
def read(va,size):
 for i in range(struct.unpack_from('<H',raw,pe+6)[0]):
  off=opt+struct.unpack_from('<H',raw,pe+20)[0]+40*i;vs,rva,rs,rp=struct.unpack_from('<IIII',raw,off+8)
  if rva<=va-base<rva+max(vs,rs):return raw[rp+va-base-rva:rp+va-base-rva+size]
 raise ValueError(hex(va))
ranges={0x140a59220:459,0x140b0e0f0:642,0x1412dccf0:60,0x1412d91b0:183,0x1412df330:368,0x1412e0fa0:405};aliases={a:[] for a in ranges}
for line in (ROOT/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').open(encoding='utf8'):
 p=line.rstrip().split('\t',4)
 if len(p)==5 and int(p[0],16) in aliases:aliases[int(p[0],16)].append({'size':int(p[1]),'module':p[3],'name':p[4]})
rows={}
for va,n in ranges.items():
 b=read(va,n);rows[hex(va)]={'size':n,'sha256':hashlib.sha256(b).hexdigest(),'raw_hex':b.hex(),'pdb_aliases':aliases[va]}
 asm=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',f'--start-address={hex(va)}',f'--stop-address={hex(va+n)}',str(image)],capture_output=True,text=True,check=True).stdout
 (OUT/(hex(va)+'.original.asm')).write_text(asm,encoding='utf8')
(OUT/'original-byte-evidence.json').write_text(json.dumps({'original_sha256':hashlib.sha256(raw).hexdigest(),'functions':rows},indent=2),encoding='utf8')
print('fresh original functions',len(rows))
