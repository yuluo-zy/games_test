"""Original native base ready/value stores and initialized getter only."""
import ctypes,hashlib,json,pathlib,random,struct
root=pathlib.Path('D:/game');data=(root/'ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(data).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=struct.unpack_from('<I',data,60)[0];opt=p+24;sh=opt+struct.unpack_from('<H',data,p+20)[0]
def raw(va,n):
    rva=va-0x140000000
    for i in range(struct.unpack_from('<H',data,p+6)[0]):
        vs,vr,rs,ro=struct.unpack_from('<4I',data,sh+i*40+8)
        if vr<=rva<vr+rs:return data[ro+rva-vr:ro+rva-vr+n]
    assert False
k=ctypes.WinDLL('kernel32',use_last_error=True);k.VirtualAlloc.restype=ctypes.c_void_p
k.VirtualAlloc.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_uint32,ctypes.c_uint32]
k.VirtualFree.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_uint32]
size=struct.unpack_from('<I',data,opt+56)[0];headers=struct.unpack_from('<I',data,opt+60)[0]
base=k.VirtualAlloc(None,size,0x3000,0x40);assert base;ctypes.memmove(base,data,headers)
for i in range(struct.unpack_from('<H',data,p+6)[0]):
    vs,vr,rs,ro=struct.unpack_from('<4I',data,sh+i*40+8)
    if rs:ctypes.memmove(base+vr,data[ro:ro+rs],rs)
# Only caller setup preserves native nonvolatile RBX/XMM15; original stores
#14078bc70..bc96 are copied unchanged, with no branches or replaced helper.
code=bytes.fromhex('534883ec10f3440f7f3c24488d99a002000066440f6efa')+raw(0x14078bc70,0x26)+bytes.fromhex('f3440f6f3c244883c4105bc3')
entry=k.VirtualAlloc(None,len(code),0x3000,0x40);assert entry;ctypes.memmove(entry,code,len(code))
initialize=ctypes.WINFUNCTYPE(None,ctypes.c_void_p,ctypes.c_uint32)(entry)
getter=ctypes.WINFUNCTYPE(ctypes.c_float,ctypes.c_void_p)(base+0xb2a680)
rng=random.Random(0x78bc70);cases=[]
for i in range(132):
    bits=[0,0x80000000,0x7f800000,0xff800000][i] if i<4 else rng.getrandbits(32)
    state=ctypes.create_string_buffer(0x2a0)
    initialize(ctypes.addressof(state),bits)
    assert struct.unpack_from('<IIII',state,0x108)==(1,bits,1,bits)
    # ctypes may quiet signaling NaNs during float conversion; keep direct
    # native output bits from state authoritative. Getter value is checked for
    # finite/non-NaN cases only, avoiding a language conversion claim.
    value=getter(ctypes.addressof(state))
    if bits&0x7f800000!=0x7f800000:
        assert struct.unpack('<I',struct.pack('<f',value))[0]==bits
    cases.append(bits)
out=pathlib.Path(__file__).with_name('gable-base-native-cases.json')
out.write_text(json.dumps({'source_sha256':hashlib.sha256(data).hexdigest(),'store_range':['14078bc70','14078bc96'],'getter':'140b2a680','base_bits':cases,'scope':'Exact original base readiness/value stores and initialized getter. Before-init native panic branch inspected, not invoked across ctypes. Rectangle/gable conditional path excluded.'},indent=2))
assert k.VirtualFree(entry,0,0x8000);assert k.VirtualFree(base,0,0x8000)
print(json.dumps({'cases':len(cases),'output':str(out)}))
