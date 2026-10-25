from pathlib import Path
import struct,re,hashlib,json
base=Path(__file__).parent
root=Path('/Users/yuluo/Desktop/game/games_test')
raw=Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
pe=struct.unpack_from('<I',raw,0x3c)[0];count=struct.unpack_from('<H',raw,pe+6)[0]
size=struct.unpack_from('<H',raw,pe+20)[0];opt=pe+24
image=struct.unpack_from('<Q',raw,opt+24)[0]
sections=[]
for i in range(count):
 row=opt+size+i*40
 virtual_size,rva,n,offset=struct.unpack_from('<IIII',raw,row+8)
 sections.append((rva,virtual_size,n,offset))
def read(va,n):
 rva=va-image
 for start,virtual,rawsize,offset in sections:
  if start<=rva and rva+n<=start+rawsize:return raw[offset+rva-start:offset+rva-start+n]
 raise ValueError(hex(va))
old=root/'reverse-engineering/tiny-glade/skeleton/task-index/work/TG-719dcf28a173a4b9b1/clutter-startup-20261020/location-result-20261020'
checks=[(base/'native-pack/is-decorator-location-valid-full.asm.txt',0x14080f470,8143),
 (old/'native-pack/dormer-ridge-bounds.asm.txt',0x140811440,1203),
 (old/'corner-native-pack/should-snap-to-corner.asm.txt',0x140811900,200),
 (base/'circle-closest-native-pack/circle-closest-required.asm.txt',0x140c97b40,84)]
# Use actual filename retained by the complete original corner pack.
if not checks[2][0].exists():checks[2]=(next((old/'corner-native-pack').glob('*.asm.txt')),0x140811900,200)
results=[]
for path,start,n in checks:
 decoded=[]
 for line in path.read_text().splitlines():
  m=re.match(r'\s*([0-9a-f]+):\s+((?:[0-9a-f]{2}(?:[ \t]+|$))+)',line)
  if m:
   address=int(m[1],16);bs=bytes.fromhex(m[2]);decoded.append((address,bs))
 assert decoded and decoded[0][0]==start,(path,'bad start')
 end=start
 for address,bs in decoded:
  assert address==end,(path,hex(address),hex(end))
  assert read(address,len(bs))==bs,(path,hex(address),'bytes mismatch')
  end+=len(bs)
 assert end==start+n,(path,hex(end),hex(start+n))
 results.append({'asm':str(path),'asm_sha256':hashlib.sha256(path.read_bytes()).hexdigest(),
  'start_va':hex(start),'code_size':n,'instruction_count':len(decoded),'all_bytes_match_original_rne':True,
  'range_sha256':hashlib.sha256(read(start,n)).hexdigest()})
report={'binary_sha256':hashlib.sha256(raw).hexdigest(),'ranges':results,
 'execution':'read-only PE/assembly verification; Rust/target not executed'}
(base/'NATIVE_BYTE_PROOF_V1.json').write_text(json.dumps(report,indent=2)+'\n')
print([(r['start_va'],r['code_size'],r['instruction_count']) for r in results])
