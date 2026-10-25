"""Original publisher's identified statistics move blocks, not whole publisher.

Only straight-line original instructions are copied unchanged. Trampoline sets
RBP/R8 to the documented caller stack, saves/restores RBP and returns. No game
helper, allocator, TLS or business condition is substituted or executed.
"""
import ctypes, hashlib, json, pathlib, random, struct
root=pathlib.Path('D:/game')
src=root/'ljxsj_92385/Tiny Glade/tiny-glade.rne'
data=src.read_bytes()
assert hashlib.sha256(data).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',data,60)[0]; opt=pe+24
sections=opt+struct.unpack_from('<H',data,pe+20)[0]
def native_bytes(va,end):
    rva=va-0x140000000
    for i in range(struct.unpack_from('<H',data,pe+6)[0]):
        vs,vr,rs,raw=struct.unpack_from('<4I',data,sections+i*40+8)
        if vr<=rva and rva+(end-va)<=vr+rs:return data[raw+rva-vr:raw+rva-vr+end-va]
    raise AssertionError(hex(va))
ranges=[(0x140a8c85a,0x140a8c8ee),(0x140a8cb01,0x140a8cb47),
        (0x140a8d6b8,0x140a8d74c),(0x140a8d7d6,0x140a8d7fb),
        (0x140a8da9d,0x140a8db58)]
k=ctypes.WinDLL('kernel32',use_last_error=True)
k.VirtualAlloc.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_uint32,ctypes.c_uint32];k.VirtualAlloc.restype=ctypes.c_void_p
k.VirtualFree.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_uint32]
allocations=[];calls=[]
for va,end in ranges:
    # push rbp;mov rbp,rcx;lea r8,[rbp+708];original;pop rbp;ret
    code=bytes.fromhex('554889cd4c8d8508070000')+native_bytes(va,end)+bytes.fromhex('5dc3')
    p=k.VirtualAlloc(None,len(code),0x3000,0x40);assert p
    ctypes.memmove(p,code,len(code));allocations.append(p)
    calls.append(ctypes.WINFUNCTYPE(None,ctypes.c_void_p)(p))
rng=random.Random(0x1481b068)
cases=[]
for i in range(128):
    allocation=ctypes.create_string_buffer(0x1020)
    frame=(ctypes.addressof(allocation)+15)&~15
    ctypes.memset(frame,0xa5,0x1000)
    current=bytearray(rng.randbytes(0x68))
    current[0:8]=bytes(8) # uncontended dependency lock state, not executed
    cap=3;ptr=0x12345000+i*64;length=i%4
    struct.pack_into('<3Q',current,8,cap,ptr,length)
    ctypes.memmove(frame+0x1b0,bytes(current),len(current))
    for call in calls:call(frame)
    observed=ctypes.string_at(frame+0x688,0xd0)
    # Only identified meaningful fields; lock flags and native padding excluded.
    before=dict(lengths=list(struct.unpack_from('<7I',current,0x28)),counts=list(struct.unpack_from('<8I',current,0x44)),seed=struct.unpack_from('<Q',current,0x20)[0],placements=[cap,ptr,length])
    after=dict(previous_lengths=list(struct.unpack_from('<7I',observed,0x28)),previous_counts=list(struct.unpack_from('<8I',observed,0x44)),previous_seed=struct.unpack_from('<Q',observed,0x20)[0],previous_placements=list(struct.unpack_from('<3Q',observed,8)),current_lengths=list(struct.unpack_from('<7I',observed,0x90)),current_counts=list(struct.unpack_from('<8I',observed,0xac)),current_seed=struct.unpack_from('<Q',observed,0x88)[0],current_placements=list(struct.unpack_from('<3Q',observed,0x70)))
    assert before['lengths']==after['previous_lengths']
    assert before['counts']==after['previous_counts']
    assert before['seed']==after['previous_seed']
    assert before['placements']==after['previous_placements']
    assert after['current_lengths']==[0]*7 and after['current_counts']==[0]*8
    assert after['current_seed']==0 and after['current_placements']==[0,4,0]
    cases.append({'before':before,'after':after})
for p in allocations:assert k.VirtualFree(p,0,0x8000)
out=pathlib.Path(__file__).with_name('material-stats-publication-native-cases.json')
out.write_text(json.dumps({'source_sha256':hashlib.sha256(data).hexdigest(),'ranges':[[hex(a),hex(b)] for a,b in ranges],'cases':cases,'scope':'Original straight-line publisher move/reset blocks only. Old-present branch resolved by fixture. Full publisher/controlflow/ECS/allocator excluded.'},indent=2))
print(json.dumps({'cases':len(cases),'native_blocks':len(calls),'output':str(out)}))
