"""Read-only B PE instruction index and native DrawState differential. No Cargo/game."""
import bisect, ctypes, hashlib, json, re, struct, subprocess, tempfile
from pathlib import Path

ROOT = Path(__file__).parent
PE = Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.exe')
SOURCE = Path('D:/game/tiny-glade/crates/country-core/src/systems/paths.rs')
OBJDUMP = r'C:/Program Files/LLVM/bin/llvm-objdump.exe'
EXPECTED = 'cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323'
data = PE.read_bytes()
sha = hashlib.sha256(data).hexdigest()
if sha != EXPECTED: raise RuntimeError('unexpected B SHA: ' + sha)
p = struct.unpack_from('<I', data, 0x3c)[0]
n = struct.unpack_from('<H', data, p+6)[0]
optsize = struct.unpack_from('<H', data, p+20)[0]
opt = p+24
base = struct.unpack_from('<Q', data, opt+24)[0]
sections = []
for i in range(n):
    name, size, va, rawsize, raw = struct.unpack_from('<8sIIII',data,opt+optsize+40*i)
    sections.append((va, max(size,rawsize), raw, rawsize))
def at(va, size):
    rva = va-base
    for start, span, raw, rawsize in sections:
        if start <= rva and rva+size <= start+rawsize:
            return data[raw+rva-start:raw+rva-start+size]
    raise ValueError(hex(va))

funcs = {'draw_path':(0x140ae4d00,0x140ae5826), 'path':(0x140ab35f0,0x140ab4750),
         'c1':(0x140ab2340,0x140ab35f0), 'c2':(0x140ab6de0,0x140ab8090),
         'c3':(0x140ab8090,0x140ab9340)}
result = {'binary':str(PE),'sha256':sha,'tool':OBJDUMP,'functions':{},
          'classification':'Raw AT&T operands only. Memory direction and aliases require manual analysis; indirect calls are unresolved.'}
line = re.compile(r'^\s*([0-9a-f]+):\s*((?:[0-9a-f]{2}\s+)+)\s*([^\s]+)\s*(.*)$')
for name,(start,end) in funcs.items():
    cmd=[OBJDUMP,'-d',f'--start-address={start:#x}',f'--stop-address={end:#x}',str(PE)]
    raw=subprocess.check_output(cmd,text=True)
    ins=[]; calls=[]; mem=[]
    for row in raw.splitlines():
        m=line.match(row)
        if not m: continue
        addr=int(m.group(1),16); op=m.group(3); operand=m.group(4).split('#')[0].strip()
        item={'address':hex(addr),'bytes':m.group(2).split(),'op':op,'operands':operand}
        ins.append(item)
        if op.startswith(('call','jmp')):
            dest=re.match(r'(0x[0-9a-f]+)',operand)
            if op.startswith('call') or not dest: calls.append({**item,'target':dest.group(1) if dest else None})
        if '(' in operand and not op.startswith(('call','jmp','nop')): mem.append(item)
    result['functions'][name]={'start':hex(start),'end_exclusive':hex(end),'range_note':'explicit audit window; may include compiler funclets',
                              'instructions':ins,'calls':calls,'memory_operands':mem}
out=ROOT/'m3-pe-instruction-index.json'
out.write_text(json.dumps(result,indent=2),encoding='utf8')
print('index',out,'instructions',sum(len(f['instructions']) for f in result['functions'].values()),'calls',sum(len(f['calls']) for f in result['functions'].values()))

# Resolve direct targets using the PE x64 exception directory, recording the
# containing runtime-function range and raw instructions rather than guessing
# function ends from nearby symbols. Imports/leaf functions may have no entry.
exc_rva,exc_size=struct.unpack_from('<II',data,opt+112+3*8)
exc=at(base+exc_rva,exc_size)
entries=sorted(struct.iter_unpack('<III',exc[:len(exc)//12*12]))
starts=[x[0] for x in entries]
targets=sorted({int(x['target'],16) for f in result['functions'].values() for x in f['calls'] if x['target']})
direct={}
for target in targets:
    rva=target-base; i=bisect.bisect_right(starts,rva)-1
    if i<0 or not (entries[i][0]<=rva<entries[i][1]):
        if target==0x140c97540:
            text=subprocess.check_output([OBJDUMP,'-d','--start-address=0x140c97540','--stop-address=0x140c97559',str(PE)],text=True)
            rows=[]
            for row in text.splitlines():
                m=line.match(row)
                if m: rows.append({'address':hex(int(m.group(1),16)),'bytes':m.group(2).split(),'op':m.group(3),'operands':m.group(4).split('#')[0].strip()})
            direct[hex(target)]={'status':'verified leaf ending in ret','start':hex(target),'end_exclusive':'0x140c97559','instructions':rows}
            continue
        direct[hex(target)]={'status':'no PE runtime-function range; not disassembled'}; continue
    lo,hi,_=entries[i]; size=hi-lo
    if size>0x20000:
        direct[hex(target)]={'status':'range exceeds audit bound','start':hex(base+lo),'end_exclusive':hex(base+hi)}; continue
    text=subprocess.check_output([OBJDUMP,'-d',f'--start-address={base+lo:#x}',f'--stop-address={base+hi:#x}',str(PE)],text=True)
    rows=[]
    for row in text.splitlines():
        m=line.match(row)
        if m: rows.append({'address':hex(int(m.group(1),16)),'bytes':m.group(2).split(),'op':m.group(3),'operands':m.group(4).split('#')[0].strip()})
    direct[hex(target)]={'status':'PE runtime-function range','start':hex(base+lo),'end_exclusive':hex(base+hi),'instructions':rows}
(ROOT/'m3-pe-direct-targets.json').write_text(json.dumps({'binary_sha256':sha,'targets':direct},indent=2),encoding='utf8')
print('direct targets',len(direct),'unresolved',sum(v['status'] not in ('PE runtime-function range','verified leaf ending in ret') for v in direct.values()))

events={'binary_sha256':sha,'scope':'commit, >1 point; queue identity per previous TypeId binding; instruction addresses below',
        'instances':{n:{'entry':hex(funcs[n][0]),'condition':cond,'debug_history_queue':first,'tool_used_queue':second,
                         'sequence_stores':[store1,store2]}
        for n,cond,first,second,store1,store2 in [
          ('path','0x140ab374d','0x140ab3758','0x140ab379f','0x140ab3790','0x140ab37d7'),
          ('c1','0x140ab24bc','0x140ab24ca','0x140ab2511','0x140ab2502','0x140ab2549'),
          ('c2','0x140ab6f5c','0x140ab6f6a','0x140ab6fb1','0x140ab6fa2','0x140ab6fe9'),
          ('c3','0x140ab820c','0x140ab821a','0x140ab8261','0x140ab8252','0x140ab8299')]},
        'caveat':'event binding depends on prior TypeId evidence; not a proof of callback absence; manually verify all stores against raw index'}
for name, event in events['instances'].items():
    by_address={row['address']:row for row in result['functions'][name]['instructions']}
    for address in [event['condition'],event['debug_history_queue'],event['tool_used_queue'],*event['sequence_stores']]:
        if address not in by_address: raise RuntimeError('event address not at instruction boundary: '+address)
    for address in event['sequence_stores']:
        if not by_address[address]['op'].startswith('mov'): raise RuntimeError('event store opcode: '+address)
(ROOT/'m3-commit-event-index.json').write_text(json.dumps(events,indent=2),encoding='utf8')

# Execute the original leaf bytes, then compare against the actual checked-in Rust method.
source=SOURCE.read_text(encoding='utf8')
begin=source.index('#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]')
end=source.index('/// `draw_path` 里 `eraser`',begin)
snippet=source[begin:end]
program=snippet+'''\nuse std::io::{Read, Write};
fn main() { let mut b=Vec::new(); std::io::stdin().read_to_end(&mut b).unwrap();
 let mut out=Vec::with_capacity(b.len()/3); for t in b.chunks_exact(3) {
 let mut s=DrawState(t[0]); s.update(t[1],t[2]); out.push(s.0); }
 std::io::stdout().write_all(&out).unwrap(); }\n'''
code=at(0x140ae4bd0,0x4a)
k=ctypes.WinDLL('kernel32',use_last_error=True)
k.VirtualAlloc.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_ulong,ctypes.c_ulong]
k.VirtualAlloc.restype=ctypes.c_void_p
k.VirtualFree.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_ulong]
address=k.VirtualAlloc(None,len(code),0x3000,0x40)
if not address: raise ctypes.WinError(ctypes.get_last_error())
ctypes.memmove(address,code,len(code))
native=ctypes.CFUNCTYPE(None,ctypes.POINTER(ctypes.c_uint8),ctypes.c_uint8,ctypes.c_uint8)(address)
inputs=bytearray(); expected=bytearray()
try:
    for state in range(256):
        for mode in range(256):
            for raw in range(256):
                v=ctypes.c_uint8(state); native(ctypes.byref(v),mode,raw)
                inputs.extend((state,mode,raw)); expected.append(v.value)
finally: k.VirtualFree(address,0,0x8000)
with tempfile.TemporaryDirectory(prefix='m3-drawstate-') as td:
    rs=Path(td)/'fixture.rs'; exe=Path(td)/'fixture.exe'; rs.write_text(program,encoding='utf8')
    cp=subprocess.run(['rustc','--edition=2021','-O',str(rs),'-o',str(exe)],capture_output=True,text=True)
    if cp.returncode: raise RuntimeError('rustc failed: '+cp.stderr)
    proc=subprocess.run([str(exe)],input=bytes(inputs),capture_output=True)
    if proc.returncode: raise RuntimeError('fixture runner failed: '+repr(proc.stderr))
    actual=proc.stdout
mismatch=next((i for i,(x,y) in enumerate(zip(expected,actual)) if x!=y),None)
report={'binary_sha256':sha,'entry':'0x140ae4bd0','code_sha256':hashlib.sha256(code).hexdigest(),
        'production_source':str(SOURCE),'source_sha256':hashlib.sha256(SOURCE.read_bytes()).hexdigest(),
        'coverage':'all 256 values of state, mode and raw','case_count':len(expected),
        'native_output_sha256':hashlib.sha256(expected).hexdigest(),'rust_output_sha256':hashlib.sha256(actual).hexdigest(),
        'output_length':len(actual),'first_mismatch':None if mismatch is None else {
            'index':mismatch,'state':inputs[mismatch*3],'mode':inputs[mismatch*3+1],
            'raw':inputs[mismatch*3+2],'native':expected[mismatch],'rust':actual[mismatch]}}
(ROOT/'m3-drawstate-native-diff.json').write_text(json.dumps(report,indent=2),encoding='utf8')
print('native differential',len(expected),'mismatch',mismatch,'length',len(actual))
if mismatch is not None or len(expected)!=len(actual): raise SystemExit(1)
