import ctypes as c, hashlib, json, struct
from pathlib import Path

ROOT = Path(__file__).resolve().parent
TARGET = Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
raw = TARGET.read_bytes()
pe = struct.unpack_from('<I', raw, 0x3c)[0]
n = struct.unpack_from('<H', raw, pe + 6)[0]
sz = struct.unpack_from('<H', raw, pe + 20)[0]
sections = [struct.unpack_from('<IIII', raw, pe+24+sz+i*40+8) for i in range(n)]
def offset(rva):
    for vsize, base, rsize, ptr in sections:
        if base <= rva < base+rsize: return ptr+rva-base
    raise ValueError(hex(rva))
code = bytearray(raw[offset(0x61e660):offset(0x61e660)+334])
assert hashlib.sha256(code).hexdigest() == '7e65d927491e32dc7340398ebdb49d1881fae034940a12037550bff992b82281'
k = c.WinDLL('kernel32', use_last_error=True)
k.VirtualAlloc.argtypes = [c.c_void_p,c.c_size_t,c.c_ulong,c.c_ulong]
k.VirtualAlloc.restype = c.c_void_p
base = k.VirtualAlloc(None, 0x4000, 0x3000, 0x40)
if not base: raise c.WinError(c.get_last_error())
trace = []
scenario = {}
callbacks = []
SUCCESS = 0x8000000000000011
CB4 = c.WINFUNCTYPE(c.c_uint64,c.c_void_p,c.c_void_p,c.c_uint64,c.c_uint64)
CB1 = c.WINFUNCTYPE(c.c_int32,c.c_int32)
CB0 = c.WINFUNCTYPE(c.c_int32)

def write_result(out, payload):
    c.memmove(out, payload, len(payload))
def failure_payload(stage):
    return struct.pack('<QQQQ',0x12340000+stage,0x123456+stage,0x77770000+stage,0x55550000+stage)
def load(out, name, length, flags):
    trace.append(['load',c.string_at(name,length).decode(),flags])
    if scenario.get('fail') == 'load': write_result(out,failure_payload(1))
    else: write_result(out,struct.pack('<QQQQ',SUCCESS,0x9988,0,0))
    return out
def symbol(out, library, name, length):
    name = c.string_at(name,length).decode()
    trace.append(['symbol',name])
    names=['gfnInitializeRuntimeSdk','gfnShutdownRuntimeSdk','gfnIsRunningInCloud']
    stage=names.index(name)+2
    if scenario.get('fail') == name: write_result(out,failure_payload(stage))
    else: write_result(out,struct.pack('<QQQQ',SUCCESS,c.cast(exports[name],c.c_void_p).value,0,0))
    return out
def drop(library,*unused): trace.append(['unload']); return 0
def initialize(language): trace.append(['initialize',language]); return scenario['code']
def cloud(): trace.append(['query']); return int(scenario['cloud'])
def shutdown(): trace.append(['shutdown']); return -99
exports={'gfnInitializeRuntimeSdk':CB1(initialize),'gfnShutdownRuntimeSdk':CB0(shutdown),'gfnIsRunningInCloud':CB0(cloud)}
callbacks.extend(exports.values())
patches={0x140db8900:CB4(load),0x140db8ab0:CB4(symbol),0x140c8fe30:CB4(drop)}
callbacks.extend(patches.values())
# Read-only strings are copied; only the three external library boundaries
# are replaced with fixture callbacks. All original private branches run.
strings = {0x142a0b020:b'GfnRuntimeSdk.dll',0x142a0b031:b'gfnInitializeRuntimeSdk',0x142a0b048:b'gfnShutdownRuntimeSdk',0x142a0b05d:b'gfnIsRunningInCloud'}
positions={}
for i,(original,value) in enumerate(strings.items()):
    positions[original]=base+0x1000+i*0x100
    c.memmove(positions[original],value,len(value))
for i,(target,callback) in enumerate(patches.items()):
    stub=base+0x2000+i*0x40
    body=b'\x48\xb8'+struct.pack('<Q',c.cast(callback,c.c_void_p).value)+b'\xff\xe0'
    c.memmove(stub,body,len(body))
    original_call_offsets=[0x30] if target==0x140db8900 else ([0x69,0x90,0xb7] if target==0x140db8ab0 else [0x139])
    for at in original_call_offsets:
        assert code[at]==0xe8
        original=0x14061e660+at+5+struct.unpack_from('<i',code,at+1)[0]
        assert original==target,(at,hex(original),hex(target))
        struct.pack_into('<i',code,at+1,stub-(base+at+5))
for at,original in [(0x1c,0x142a0b020),(0x54,0x142a0b031),(0x7b,0x142a0b048),(0xa2,0x142a0b05d)]:
    assert code[at:at+3] in (b'\x48\x8d\x15',b'\x4c\x8d\x05')
    struct.pack_into('<i',code,at+3,positions[original]-(base+at+7))
c.memmove(base,bytes(code),len(code))
probe=c.WINFUNCTYPE(c.c_void_p,c.c_void_p)(base)
results=[]
for init in [0,1,-1,-2,-17,-2147483648,2147483647]:
    for state in [False,True]:
        scenario.clear();scenario.update(code=init,cloud=state)
        trace.clear();out=c.create_string_buffer(32)
        assert probe(out)==c.addressof(out)
        if init==0:
            assert struct.unpack_from('<Q',out.raw)[0]==0x8000000000000012
            assert out.raw[8]==int(state)
            assert trace[-4:]==[['initialize',0],['query'],['shutdown'],['unload']]
        else:
            assert struct.unpack_from('<Q',out.raw)[0]==SUCCESS
            assert struct.unpack_from('<i',out.raw,8)[0]==init
            assert trace[-2:]==[['initialize',0],['unload']]
        results.append({'code':init,'cloud':state,'result':out.raw.hex(),'calls':trace.copy()})
for fail in ['load','gfnInitializeRuntimeSdk','gfnShutdownRuntimeSdk','gfnIsRunningInCloud']:
    scenario.clear();scenario.update(code=0,cloud=True,fail=fail)
    trace.clear();out=c.create_string_buffer(32);probe(out)
    stage=['load','gfnInitializeRuntimeSdk','gfnShutdownRuntimeSdk','gfnIsRunningInCloud'].index(fail)+1
    assert out.raw==failure_payload(stage)
    assert not any(x[0] in ('initialize','query','shutdown') for x in trace)
    assert (trace[-1][0]=='unload') == (fail!='load')
    results.append({'failure':fail,'result':out.raw.hex(),'calls':trace.copy()})
report={'passed':True,'cases':len(results),'original_rva':'0x61e660','original_function_sha256':'7e65d927491e32dc7340398ebdb49d1881fae034940a12037550bff992b82281','target_sha256':hashlib.sha256(raw).hexdigest(),'scope':'original private code with injected DLL loader/symbol/drop and SDK callbacks; actual vendor SDK not loaded','results':results}
(ROOT/'gfn-native-validation.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({'passed':True,'cases':len(results)}))
