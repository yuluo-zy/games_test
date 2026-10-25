"""Additional B raw-ID reuse/generation-zero/usage-gap cases; frozen 791 file is read-only."""
import ctypes as c
import hashlib
import json
from pathlib import Path
import struct as s

out=Path(__file__).resolve().parent
frozen=out/'native-cache-oracle.json'
frozen_sha=hashlib.sha256(frozen.read_bytes()).hexdigest()
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
binary_sha=hashlib.sha256(raw).hexdigest()
assert binary_sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=s.unpack_from('<I',raw,60)[0]
opt=pe+24
sections=opt+s.unpack_from('<H',raw,pe+20)[0]
def original(address,length):
    rva=address-0x140000000
    for i in range(s.unpack_from('<H',raw,pe+6)[0]):
        vs,va,rs,ro=s.unpack_from('<IIII',raw,sections+i*40+8)
        if va<=rva<va+max(vs,rs): return raw[ro+rva-va:ro+rva-va+length]
    raise ValueError(hex(address))

k=c.WinDLL('kernel32',use_last_error=True)
k.VirtualAlloc.argtypes=[c.c_void_p,c.c_size_t,c.c_uint32,c.c_uint32]
k.VirtualAlloc.restype=c.c_void_p
k.VirtualFree.argtypes=[c.c_void_p,c.c_size_t,c.c_uint32]
k.VirtualFree.restype=c.c_int
owned=[]
def executable(code):
    p=k.VirtualAlloc(None,4096,0x3000,0x40)
    assert p
    c.memmove(p,bytes(code),len(code))
    owned.append(p)
    return p
def field(obj,offset): return s.unpack_from('<I',obj,offset)[0]
def setfield(obj,offset,value): s.pack_into('<I',obj,offset,value)
def values(slots): return [s.unpack_from('<IH',slots,i*8) for i in range(len(slots)//8)]

render_start=0x140b91120
render_code=original(render_start,1028)
# Compare the complete code against the already validated frozen PDB range.
range_row=next(r for r in json.loads(frozen.read_text(encoding='utf-8'))['ranges'] if r['address']=='0x140b91120')
assert hashlib.sha256(render_code).hexdigest()==range_row['bytes_sha256']
probe_code=bytearray(render_code)
hooks=[]
for address,value in [(0x140b91265,1),(0x140b914df,0),(0x140b91517,2)]:
    marker=b'\xb8'+s.pack('<I',value)+b'\xc3'
    probe_code[address-render_start:address-render_start+len(marker)]=marker
    hooks.append({'address':hex(address),'original':original(address,len(marker)).hex(),'replacement':marker.hex()})
probe_base=executable(probe_code)
thunk=(bytes.fromhex('41 57 57 53 55 41 55 49 89 cf 48 89 d7 48 81 ec a0 00 00 00 48 b8')
       +s.pack('<Q',probe_base+0x98)
       +bytes.fromhex('ff d0 48 81 c4 a0 00 00 00 41 5d 5d 5b 5f 41 5f c3'))
probe=c.WINFUNCTYPE(c.c_uint32,c.c_void_p,c.c_void_p)(executable(thunk))
usage_code=original(0x141800040,0x2c7)
usage=c.WINFUNCTYPE(None,c.c_void_p,c.c_uint64,c.c_uint32,c.c_uint32)(executable(usage_code))

validation=[]
for frame in [1,2,6,0xffffffff]:
    for pattern in ['same-id-zero-reused','requested-gen-one-mismatch','stored-gen-one-mismatch','valid-prefix-then-gen-one-mismatch']:
        renderer=c.create_string_buffer(0x4e0)
        fragment=c.create_string_buffer(0x260)
        slots=c.create_string_buffer(b'\xa5'*16,16)
        s.pack_into('<IH',slots,0,0,0)
        s.pack_into('<IH',slots,8,9,0)
        vector=(c.c_uint64*3)(2,c.addressof(slots),2)
        # Slot was logically vacant, then the same raw id 0 is reallocated.
        # There is no hidden epoch/counter in the original slot or validator.
        reused_at=frame-1 if frame>1 else 0xffffffff
        usage(vector,0,reused_at,1 if pattern=='stored-gen-one-mismatch' else 0)
        ids=[0x10000] if pattern=='requested-gen-one-mismatch' else [0]
        if pattern=='valid-prefix-then-gen-one-mismatch': ids.append(0x10001)
        allocation_ids=(c.c_uint32*len(ids))(*ids)
        s.pack_into('<QQQ',fragment,0x1d0,len(ids),c.addressof(allocation_ids),len(ids))
        setfield(fragment,0x24c,1)
        setfield(fragment,0x250,0)
        s.pack_into('<QQQ',renderer,0x3f8,2,c.addressof(slots),2)
        setfield(renderer,0x4c8,frame)
        setfield(renderer,0x4cc,1)
        before=values(slots)
        branch=probe(fragment,renderer)
        expected_branch=0 if pattern=='same-id-zero-reused' else 1
        assert branch==expected_branch
        assert field(fragment,0x24c)==(1 if expected_branch==0 else 0)
        assert field(fragment,0x250)==(frame if expected_branch==0 else 0)
        prefix_valid=pattern in ['same-id-zero-reused','valid-prefix-then-gen-one-mismatch']
        assert s.unpack_from('<IH',slots,0)==(frame if prefix_valid else reused_at,1 if pattern=='stored-gen-one-mismatch' else 0)
        assert s.unpack_from('<IH',slots,8)==(9,0)
        validation.append({'frame':frame,'reused_at':reused_at,'pattern':pattern,'ids':ids,'usage_before':before,
                           'branch':branch,'fragment_stamp':field(fragment,0x24c),
                           'validated_at':field(fragment,0x250),'usage_after':values(slots)})

gaps=[]
for frame in [0,1,2,6,0xffffffff]:
    slots=c.create_string_buffer(b'\xa5'*64,64)
    for index in range(3): s.pack_into('<IH',slots,index*8,10+index,index)
    before=slots.raw
    vector=(c.c_uint64*3)(8,c.addressof(slots),3)
    usage(vector,5,frame,0)
    assert list(vector)==[8,c.addressof(slots),6]
    for index in [3,4]: assert s.unpack_from('<IH',slots,index*8)==(0,0)
    assert s.unpack_from('<IH',slots,5*8)==(frame,0)
    assert slots.raw[:24]==before[:24] and slots.raw[48:]==before[48:]
    assert all(slots.raw[index*8+6:index*8+8]==b'\xa5\xa5' for index in [3,4,5])
    gaps.append({'before_len':3,'capacity':8,'requested_index':5,'after_len':6,
                 'frame':frame,'generation':0,'after_usage':values(slots),
                 'padding_untouched':True,'no_allocation':True})

assert hashlib.sha256(frozen.read_bytes()).hexdigest()==frozen_sha
report={'binary_sha256':binary_sha,'frozen_791_sha256':frozen_sha,'frozen_791_unchanged':True,
        'scope':'original validator prefix plus complete original usage helper, capacity sufficient; no etagere allocation or GPU executed',
        'same_id_conclusion':'raw id 0 can be reused and accepted when slot generation remains 0 and last_used != 0; no extra identity epoch',
        'hooks':hooks,'usage_helper_sha256':hashlib.sha256(usage_code).hexdigest(),
        'validation_cases':validation,'gap_cases':gaps,'passed':len(validation)+len(gaps),'mismatches':0}
(out/'native-same-id-oracle.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
for p in owned: assert k.VirtualFree(p,0,0x8000)
print(json.dumps({'passed':report['passed'],'mismatches':0,'frozen_791_unchanged':True}))
