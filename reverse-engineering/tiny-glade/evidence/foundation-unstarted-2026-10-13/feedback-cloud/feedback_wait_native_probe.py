import ctypes as c, hashlib, json, struct
from pathlib import Path
ROOT=Path(__file__).resolve().parent
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
# B .text RVA4096/raw1024; only this section is used.
def code_at(va,n): return raw[va-0x140000000-0x1000+0x400:va-0x140000000-0x1000+0x400+n]
start=0x140690a2a
branch=bytearray(code_at(start,0x140690aae-start))
wait=code_at(0x140691937,8)
k=c.WinDLL('kernel32',use_last_error=True)
k.VirtualAlloc.argtypes=[c.c_void_p,c.c_size_t,c.c_ulong,c.c_ulong];k.VirtualAlloc.restype=c.c_void_p
base=k.VirtualAlloc(None,0x4000,0x3000,0x40)
if not base: raise c.WinError(c.get_last_error())
wrapper=bytes.fromhex('55 48 81 ec 00 0c 00 00 48 8d ac 24 80 00 00 00 48 89 ca 48 8d 81 80 00 00 00 48 89 85 a0 00 00 00 48 8d 81 a0 00 00 00 48 89 85 a8 00 00 00 48 89 8d b8 0a 00 00')
body=base+len(wrapper);wait_dest=base+0x800;epilogue=base+0x900
CB=c.WINFUNCTYPE(c.c_uint64,c.c_void_p)
scenario={};trace=[]
def deref(pointer): return pointer
def is_file(pointer):
    index=0 if pointer==scenario['first'] else 1
    trace.append(index)
    return int(scenario['ready'][index])
callbacks=[CB(deref),CB(is_file)]
stubs={}
for target,callback,index in [(0x140d76ed0,callbacks[0],0),(0x140d62cf0,callbacks[1],1)]:
    stubs[target]=base+0x2000+index*0x40
    b=b'\x48\xb8'+struct.pack('<Q',c.cast(callback,c.c_void_p).value)+b'\xff\xe0'
    c.memmove(stubs[target],b,len(b))
for va in [0x140690a47,0x140690a56,0x140690a6d,0x140690a7c]:
    at=va-start;assert branch[at]==0xe8
    target=va+5+struct.unpack_from('<i',branch,at+1)[0]
    struct.pack_into('<i',branch,at+1,stubs[target]-(body+at+5))
# Original conditional jump to waiting increment slice. Internal branches stay
# relative to the same copied private block.
at=0x140690aa0-start;assert branch[at:at+2]==b'\x0f\x86'
struct.pack_into('<i',branch,at+2,wait_dest-(body+at+6))
exit_send=b'\xb8\x01\x00\x00\x00\xe9'+struct.pack('<i',epilogue-(body+len(branch)+10))
exit_wait=wait+b'\x31\xc0\xe9'+struct.pack('<i',epilogue-(wait_dest+len(wait)+7))
end=bytes.fromhex('48 81 c4 00 0c 00 00 5d c3')
for ptr,b in [(base,wrapper),(body,bytes(branch)+exit_send),(wait_dest,exit_wait),(epilogue,end)]:c.memmove(ptr,b,len(b))
probe=c.WINFUNCTYPE(c.c_uint32,c.c_void_p)(base)
results=[]
for present in [False,True]:
    for readiness in [(False,False),(False,True),(True,False),(True,True)]:
        for counter in [0,599,600,601,602,4294967295]:
            pending=c.create_string_buffer(0xe0)
            p=c.addressof(pending)
            struct.pack_into('<Q',pending,0x80,1 if present else 0x8000000000000000)
            struct.pack_into('<I',pending,0xd8,counter)
            scenario.update(first=p+0x80,ready=readiness)
            trace.clear();send=probe(p)
            expected=not present or all(readiness) or counter>600
            assert bool(send)==expected,(present,readiness,counter,send)
            after=struct.unpack_from('<I',pending,0xd8)[0]
            assert after==(counter+1)&0xffffffff
            expected_calls=[] if not present else ([0,1] if readiness[0] else [0])
            assert trace==expected_calls
            results.append({'save_present':present,'ready':readiness,'before':counter,'after':after,'send':bool(send),'file_query_order':trace.copy()})
report={'passed':True,'cases':len(results),'target_sha256':hashlib.sha256(raw).hexdigest(),'scope':'original B readiness/count branch slices with injected Path::deref/is_file; sender and Bevy containers excluded','branch':{'start':hex(start),'bytes':len(branch),'sha256':hashlib.sha256(code_at(start,len(branch))).hexdigest()},'results':results}
(ROOT/'feedback-wait-native-validation.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({'passed':True,'cases':len(results)}))
