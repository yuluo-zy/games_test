"""Original B scalar clone/select instructions, all 400x400 valid joint tag pairs."""
import itertools,json,struct,subprocess,hashlib
from original_code import BASE,SIZE,SECTIONS,raw,SHA,OUT,read
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import UC_X86_REG_RBP,UC_X86_REG_RDI,UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RIP
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(BASE,(SIZE+4095)&~4095)
for virtual,rva,n,off in SECTIONS:cpu.mem_write(BASE+rva,raw[off:off+n])
frame=0x30000000;parent=0x30002000;cpu.mem_map(frame,0x10000)
def packed(v):return struct.pack('<IIIIIBBxx',*v)
def normalized(v):
    r,w,h,m,n,g,c=v
    return [r,w if r==1 else 0,h if r==1 else 0,m,n if m==2 else 0,g,c]
states=list(itertools.product(range(4),range(5),range(4),range(5)))
inputs=[];expected=[]
for a,b in itertools.product(states,repeat=2):
    child=[a[0],11,19,a[1],7,a[2],a[3]];par=[b[0],31,47,b[1],23,b[2],b[3]]
    cpu.mem_write(frame+0x860,packed(child));cpu.mem_write(parent,packed(par))
    cpu.reg_write(UC_X86_REG_RBP,frame);cpu.reg_write(UC_X86_REG_RDI,parent)
    cpu.reg_write(UC_X86_REG_RAX,child[0]);cpu.reg_write(UC_X86_REG_RBX,child[3])
    cpu.emu_start(0x1420d9039,0x1420d92bc,count=160)
    assert cpu.reg_read(UC_X86_REG_RIP)==0x1420d92bc
    expected.append(normalized(struct.unpack('<IIIIIBBxx',cpu.mem_read(frame+0x860,24))))
    inputs.append(child+par)
runner=OUT.parents[3]/'reconstruction/shared-foundation/target/debug/examples/shallow_runner.exe'
result=subprocess.run([str(runner)],input=''.join(' '.join(map(str,v))+'\n'for v in inputs),text=True,capture_output=True,check=True)
actual=[list(map(int,line.split()))for line in result.stdout.splitlines()]
diff=[{'input':v,'original':x,'candidate':y}for v,x,y in zip(inputs,expected,actual)if x!=y]
report={'schema':1,'ok':len(actual)==len(expected)and not diff,'cases':len(inputs),'mismatches':diff[:20],
 'original_RNE_sha256':SHA,'original_inlinee':'0x1a669','parent_procedure':'asset::texture::scan::scan_asset_tree_walker 0x1420d8d00',
 'start':'0x1420d9039','stop':'0x1420d92bc','instruction_ranges':[
 {'start':hex(s),'end':hex(e),'sha256':hashlib.sha256(read(s,e-s)).hexdigest()}for s,e in [(0x1420d8eec,0x1420d8f20),(0x1420d9039,0x1420d90bc),(0x1420d922a,0x1420d92bc)]],
 'oracle':'Original inline caller register/frame state; original RNE instructions; no external callbacks or rewritten library routine',
 'candidate_exe_sha256':hashlib.sha256(runner.read_bytes()).hexdigest(),
 'limits':['Four scalar/array private specializations only; not arbitrary T::Clone ABI','All valid joint state tags; dimensions/mip payloads use distinct child/parent samples','Uninitialized inactive payload bytes ignored','No full game runtime or C-version claim']}
(OUT/'shallow-native-comparison.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf8')
print(json.dumps({'ok':report['ok'],'cases':len(inputs),'mismatches':len(diff)}))
if not report['ok']:raise SystemExit(1)
