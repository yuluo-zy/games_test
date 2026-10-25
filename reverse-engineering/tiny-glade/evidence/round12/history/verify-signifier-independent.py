"""Independent bounded original-byte/query/clock audit, without source mutation."""
from pathlib import Path
import hashlib,json,struct,subprocess
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import UC_X86_REG_RSI,UC_X86_REG_RBP,UC_X86_REG_RAX,UC_X86_REG_XMM6,UC_X86_REG_XMM7,UC_X86_REG_XMM8

HERE=Path(__file__).resolve().parent
TARGET=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
blob=TARGET.read_bytes();sha=hashlib.sha256(blob).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=blob);base=pe.OPTIONAL_HEADER.ImageBase
def read(va,size):return pe.get_data(va-base,size)
result={'original_sha256':sha,'functions':{},'typed_query_records':[]}
procs={}
for line in Path('D:/game/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').read_text(encoding='utf8').splitlines():
    f=line.split('\t')
    if len(f)>4:
        procs[int(f[0],16)]=(int(f[1]),f[-1])
        if ('run_unsafe<' in f[-1] or 'into_configs<' in f[-1]) and 'SignifierMarkedForDeletion' in f[-1]:
            result['typed_query_records'].append({'va':f[0],'size':int(f[1]),'name':f[-1]})
with (HERE/'signifier-animation-independent-original.asm').open('w',encoding='utf8') as out:
    for address in [0x140a69340,0x140a69c30,0x140929160,0x140929a90]:
        size,name=procs[address]
        result['functions'][hex(address)]={'size':size,'sha256':hashlib.sha256(read(address,size)).hexdigest(),'pdb_name':name}
        out.write(subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',
            f'--start-address={hex(address)}',f'--stop-address={hex(address+size)}',str(TARGET)],
            stdout=subprocess.PIPE,check=True).stdout.decode('utf8'))

# Execute the actual original branch and SSE clamp instructions up to easing.
# No re-expression of the arithmetic is used to generate the expected bits.
cpu=Uc(UC_ARCH_X86,UC_MODE_64)
cpu.mem_map(0x140a69000,0x2000);cpu.mem_write(0x140a69000,read(0x140a69000,0x2000))
cpu.mem_map(0x200000,0x4000)
cpu.reg_write(UC_X86_REG_RSI,0x200000);cpu.reg_write(UC_X86_REG_RBP,0x202800)
cpu.mem_write(0x2027f0,struct.pack('<Q',0x201000))
cpu.mem_write(0x20283c,struct.pack('<I',17));cpu.mem_write(0x202840,struct.pack('<Q',0x203000))
cases=[]
durations=[0,0x80000000,0x3f800000,0xbf800000,0x40000000,0x3e4ccccd,0x7f800000,0xff800000,0x7fc01234,0x7f801234,0x00800000,1]
times=[0,0x80000000,0x3f000000,0x3f800000,0x7fc12345,0x7f800000]
ticks=[0,0x80000000,0x3c83126f,0x3e4ccccd,0xbe800000,0x7f800000,0x7fc34567]
for duration in durations:
    for t in times:
        for tick in ticks:
            for deleted in [False,True]:
                cpu.mem_write(0x200010,struct.pack('<II',duration,t))
                cpu.mem_write(0x201028,struct.pack('<I',tick))
                cpu.reg_write(UC_X86_REG_RAX,int(deleted))
                cpu.reg_write(UC_X86_REG_XMM6,0);cpu.reg_write(UC_X86_REG_XMM7,0x3f800000)
                cpu.reg_write(UC_X86_REG_XMM8,0x80000000)
                cpu.emu_start(0x140a69998,0x140a699fe)
                value=struct.unpack('<I',cpu.mem_read(0x200014,4))[0]
                cases.append([duration,t,tick,deleted,value])
result['native_t_cases']={'segment_start':'0x140a69998','segment_end_exclusive':'0x140a699fe',
    'segment_sha256':hashlib.sha256(read(0x140a69998,0x66)).hexdigest(),
    'columns':['duration_bits','old_t_bits','delta_bits','deleted','native_new_t_bits'],
    'cases':cases,'count':len(cases),'scope':'Native time branch/clamp only; no easing/query/schedule runtime acceptance'}
(HERE/'signifier-animation-independent-original.json').write_text(json.dumps(result,indent=2),encoding='utf8')
print(json.dumps({'functions':result['functions'],'typed_query_record_count':len(result['typed_query_records']),
    'native_t_case_count':len(cases)},indent=2))
