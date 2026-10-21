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

host=Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-10dd22b74858bd4c2e/decorator-chain-root-20261021/window-host-contract-v1/validation-native-pack')
checks=[(host/'validate-blueprints-system.asm.txt',0x140827130,539),
 (base/'callback-native-pack/validation-game-callback-fold-host.asm.txt',0x1412b84f0,8161),
 (host/'validate-blueprints-business-closure.asm.txt',0x1412ba810,473),
 (base/'max-wall-native-pack/show-max-wall-y-full.asm.txt',0x140827350,434),
 (base/'max-wall-native-pack/show-max-wall-y-map-full.asm.txt',0x141261700,445),
 (base/'scribble-name-native-pack/scribble-decorator-id-debug-formatter.asm.txt',0x1412ba7d0,56)]
results=[]
for path,start,n in checks:
 decoded=[]
 for line in path.read_text().splitlines():
  m=re.match(r'\s*([0-9a-f]+):\s+((?:[0-9a-f]{2}(?:[ \t]+|$))+)',line)
  if m:decoded.append((int(m[1],16),bytes.fromhex(m[2])))
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
def jump_table(va,count):return [hex(va+struct.unpack_from('<i',read(va+4*i,4))[0]) for i in range(count)]
assert read(0x142c2c250,18)==b'despawn_decorator_'
assert read(0x142c2c262,1)==b'_'
assert read(0x142a91c00,24)
report={'binary_sha256':hashlib.sha256(raw).hexdigest(),'ranges':results,
 'failure_reason_table':{'va':'0x142c2c390','targets':jump_table(0x142c2c390,11)},
 'decorator_type_accessor_table':{'va':'0x142a91c00','targets':jump_table(0x142a91c00,6),
  'semantic_payload_offsets':[105,92,96,92,92,None]},
 'scribble_format_pieces':['despawn_decorator_','_'],
 'scalar_literals':{hex(va):{'bits':read(va,4).hex(),'value':struct.unpack('<f',read(va,4))[0]} for va in [0x142925870,0x142925904,0x1429cd078]},
 'execution':'read-only PE/assembly verification; Rust/target not executed'}
assert report['binary_sha256']=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
(base/'NATIVE_BYTE_PROOF_V1.json').write_text(json.dumps(report,indent=2)+'\n')
print([(r['start_va'],r['code_size'],r['instruction_count']) for r in results])
