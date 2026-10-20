from pathlib import Path
import struct,json,hashlib,subprocess
ROOT=Path('D:/game')
OUT=Path(__file__).resolve().parent
exe=ROOT/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe'
blob=exe.read_bytes(); h=struct.unpack_from('<I',blob,60)[0]
ns=struct.unpack_from('<H',blob,h+6)[0]; os=struct.unpack_from('<H',blob,h+20)[0]
def read(va,size):
    for i in range(ns):
        s=h+24+os+40*i;vs,rva,n,off=struct.unpack_from('<IIII',blob,s+8)
        if rva<=va-0x140000000<rva+max(vs,n):
            at=off+va-0x140000000-rva
            return blob[at:at+size]
    raise ValueError(hex(va))
procs={}
for line in (ROOT/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').open(encoding='utf8'):
    p=line.strip().split('\t')
    if len(p)>=5:procs.setdefault(p[0],[]).append({'bytes':int(p[1]),'symbol':p[-1]})
ranges=[]
for start,size,name in [(0x141673140,704,'identity-fallback'),(0x14145a8e0,396,'identity-pair-hash'),
 (0x14145aa70,424,'identity-basename-hash'),(0x140d59e60,23,'metadata-modified'),
 (0x1412f50f0,549,'migration-retain-edits'),(0x1402de600,procs['0x1402de600'][0]['bytes'],'decorator-type-serialize'),
 (0x140d6a960,procs['0x140d6a960'][0]['bytes'],'metadata-result-construction')]:
    path=OUT/(name+'.asm')
    r=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',f'--start-address={start:#x}',f'--stop-address={start+size:#x}',str(exe)],capture_output=True,text=True,check=True)
    path.write_text(r.stdout,encoding='utf8')
    ranges.append({'start':hex(start),'bytes':size,'sha256':hashlib.sha256(read(start,size)).hexdigest(),'symbols':procs.get(hex(start),[]),'assembly':path.name})
strings={}
for addr in [0x142c64a90,0x142c64af0]:
    pointer,length=struct.unpack('<QQ',read(addr,16))
    strings[hex(addr)]={'pointer':hex(pointer),'length':length,'text':read(pointer,length).decode()}
type_rows=[]
for tag in range(0x38,0x3b):
    target=0x1429b3460+struct.unpack('<i',read(0x1429b3460+(tag-0x38)*4,4))[0]
    code=read(target,32)
    # Each unit case is lea(rdx), lea(r8,string), mov(r9d,length), jmp.
    lea=code.index(b'\x4c\x8d\x05'); ptr=target+lea+7+struct.unpack_from('<i',code,lea+3)[0]
    mov=code.index(b'\x41\xb9');length=struct.unpack_from('<I',code,mov+2)[0]
    type_rows.append({'tag':tag,'jump_target':hex(target),'string_pointer':hex(ptr),'length':length,'text':read(ptr,length).decode()})
constants={hex(v):{'bits':hex(struct.unpack('<I',read(v,4))[0])} for v in [0x142a82410,0x142960674,0x14294ca10,0x142960690,0x1429cd060,0x1429345a8,0x142a7fa30]}
table={
 'original':{'path':str(exe),'sha256':hashlib.sha256(blob).hexdigest()},
 'ranges':ranges,'strings':strings,'decorator_serializer_cases':type_rows,'float_constants':constants,
 'byte_flow':[
  {'producer':'141472734..14147279f','condition':'version<20; inner 0x58 record byte[-6] in 0x38..=0x3a','rewrite':'record u32[-0x28] through 9-way table; 999 retained; >8 except999 ->1','flag_store':'stack+0x2f0=1','wrapper_write':'141472eab: wrapper+0xd=stack+0x2f0 & 1','consumer':'14146ec9c: bt flags,0x28; +0x910 is successful wrapper second word','snapshot_write':'14146ed83..14146ee0a: Vec at stack+0x240/248, 16B records +8, same color map','log_fragment_pointer':'0x142c64af0'},
  {'producer':'141472a9f..141472aac','condition':'version<21; inner 0x58 record byte[-6]==0x3a','rewrite':'record u32[-0x28]=999','flag_store':'stack+0x304=1','wrapper_write':'141472eaf: wrapper+0xe=stack+0x304 & 1','consumer':'14146eb4e: bt flags,0x30','snapshot_write':'14146ec35..14146ec9a: outer 0x28 rows, inner0x58 rows, byte+0x52==0x3a ->u32+0x30=999','log_fragment_pointer':'0x142c64a90'}],
 'identity_flow':[
  '141673164 Path::file_name, 169/16d stores Option<&OsStr>',
  '141673175 calculate_hash<Option<&OsStr>>, 17a keeps hash in rsi',
  '141673187 std::fs::metadata/stat; error returns current rsi after dropping error',
  '141673207 loads Metadata+0x38 u64; 21f calculate_hash<(old_hash,&u64)>',
  '14167322f Metadata::modified; error returns current rsi after dropping error',
  '141673275 reads successful SystemTime raw8B; 28d hashes (old_hash,&SystemTime)',
  '14145a8e0 ICF has (u64,&u64)/(u64,&SystemTime): hashes exactly two8B words',
  'No clock query and no second Identity parser call exist in full704B procedure'],
 'history_identity_load_flow':[
  'parser141472d26 writes parsed body+0x48 = r15, successful Identity or fallback',
  'load14146dd98/9c takes parsed+0x48 to stack+0x8b8',
  'load14146de8e/95 moves same stack+0x8b8 to canonical History+0x60',
  'finish140a86832 and finish_potential140a86a33 mutate same History+0x60',
  'current ledger.mutation_nonce is same value, save_game publish_save_sections already serializes it'],
 'metadata_length_evidence':{
  'pdb_rust_commit':'05f9846f893b09a1be1fc8560e33fc3c815cfecb',
  'rust_source':'https://raw.githubusercontent.com/rust-lang/rust/1.86.0/library/std/src/sys/pal/windows/fs.rs',
  'source_observations':'FileAttr::size returns file_size; From<WIN32_FIND_DATAW> creates file_size as (nFileSizeHigh<<32)|nFileSizeLow. Metadata::modified returns SystemTime::from(last_write_time).',
  'original_construction':'140d6abb2/abb6 loads FindFirstFileExW output+32/+28; abed/abf1 combines high<<32|low; ac11 stores result at Metadata+0x38. Original modified@140d59e67 reads +0x30, independently matching last_write_time copied from Find data+20 at140d6abad/ac0d.',
  'no_std_reimplementation':'Only the game leaf calling policy is restored; canonical std metadata,len,modified and existing calculate_hash are reused.'},
 'scope':'read-only original RNE; no database overlay, no Cargo, no production changes'}
(OUT/'native-flow.json').write_text(json.dumps(table,ensure_ascii=False,indent=2),encoding='utf8')
print(json.dumps({'ranges':len(ranges),'strings':strings}))
