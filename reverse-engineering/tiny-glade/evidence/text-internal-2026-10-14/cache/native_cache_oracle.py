"""Execute B instruction slices and complete warm-render paths; no game/GPU startup.

Continuation markers classify branches after the untouched validation prefix.
They are harness instrumentation, not a production adapter or restored algorithm.
"""
import ctypes as c
import hashlib
import json
from pathlib import Path
import struct as s

out = Path(__file__).resolve().parent
source = Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
raw = source.read_bytes()
binary_sha = hashlib.sha256(raw).hexdigest()
assert binary_sha == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = s.unpack_from('<I',raw,60)[0]
opt = pe+24
sections = opt+s.unpack_from('<H',raw,pe+20)[0]
def bytes_at(address,length):
    rva = address-0x140000000
    for i in range(s.unpack_from('<H',raw,pe+6)[0]):
        virtual_size,virtual,physical,offset = s.unpack_from('<IIII',raw,sections+i*40+8)
        if virtual <= rva < virtual+max(physical,virtual_size):
            return raw[offset+rva-virtual:offset+rva-virtual+length]
    raise ValueError(hex(address))

k = c.WinDLL('kernel32',use_last_error=True)
k.VirtualAlloc.argtypes = [c.c_void_p,c.c_size_t,c.c_uint32,c.c_uint32]
k.VirtualAlloc.restype = c.c_void_p
k.VirtualFree.argtypes = [c.c_void_p,c.c_size_t,c.c_uint32]
k.VirtualFree.restype = c.c_int
allocations = []
def executable(code):
    code=bytes(code)
    p = k.VirtualAlloc(None,max(4096,len(code)),0x3000,0x40)
    assert p
    c.memmove(p,code,len(code))
    allocations.append(p)
    return p
def wrapper(prefix,address,suffix):
    return executable(bytes.fromhex(prefix)+bytes.fromhex('48 b8')+s.pack('<Q',address)+bytes.fromhex('ff d0')+bytes.fromhex(suffix))
def field(obj,offset): return s.unpack_from('<I',obj,offset)[0]
def setfield(obj,offset,value): s.pack_into('<I',obj,offset,value)
def slot_values(slots): return [s.unpack_from('<IH',slots,i*8) for i in range(len(slots)//8)]

ranges = json.loads(Path('D:/game/reverse-engineering/tiny-glade/evidence/tg-text-full-2026-10-13/ranges.json').read_text(encoding='utf-8'))
bound = []
for name in ['new','set_pixels_per_point','upload_pending_glyphs','get_glyph','render']:
    candidates = [r for r in ranges if r['name'].endswith('::'+name)]
    for row in candidates:
        code = bytes_at(int(row['address'],16),row['size'])
        assert hashlib.sha256(code).hexdigest()==row['bytes_sha256']
        bound.append(row)

# Full unmodified render function: only warm paths with empty primitive stream.
render_start = 0x140b91120
render_raw = bytes_at(render_start,1028)
render_mem = executable(render_raw)
warm = c.WINFUNCTYPE(None,c.c_void_p,c.c_uint64,c.c_uint64,c.c_uint32,c.c_void_p)(render_mem)

# Same original validation code with three instrumented outgoing continuations.
render_probe = bytearray(render_raw)
instrumentation = []
for address,value in [(0x140b91265,1),(0x140b914df,0),(0x140b91517,2)]:
    marker = b'\xb8'+s.pack('<I',value)+b'\xc3'
    render_probe[address-render_start:address-render_start+len(marker)] = marker
    instrumentation.append({'address':hex(address),'role':{0:'reuse',1:'regenerate',2:'bounds-panic'}[value],
                            'original':bytes_at(address,len(marker)).hex(),'replacement':marker.hex()})
render_probe_mem = executable(render_probe)
render_wrapper = wrapper('41 57 57 53 55 41 55 49 89 cf 48 89 d7 48 81 ec a0 00 00 00',
                         render_probe_mem+0x98,'48 81 c4 a0 00 00 00 41 5d 5d 5b 5f 41 5f c3')
probe = c.WINFUNCTYPE(c.c_uint32,c.c_void_p,c.c_void_p)(render_wrapper)

frames = [0,1,2,0xfffffffe,0xffffffff]
stamps = [0,1,0xffffffff]
patterns = ['empty','valid','generation-mismatch','zero-sentinel','out-of-bounds']
prefix_cases, warm_cases = [],[]
for frame in frames:
    for stamp in stamps:
        for fragment_stamp in stamps:
            for validated in stamps:
                for pattern in patterns:
                    fragment = c.create_string_buffer(0x260)
                    renderer = c.create_string_buffer(0x4e0)
                    slots = c.create_string_buffer(16)
                    s.pack_into('<IH',slots,0,9,0xffff)
                    s.pack_into('<IH',slots,8,0 if pattern=='zero-sentinel' else 9,0xffff)
                    ids = [] if pattern=='empty' else [0xffff0000]
                    if pattern in ['generation-mismatch','zero-sentinel']:
                        ids.append(0xfffe0001 if pattern=='generation-mismatch' else 0xffff0001)
                    elif pattern=='out-of-bounds': ids.append(0xffff0002)
                    ids_array = (c.c_uint32*len(ids))(*ids)
                    s.pack_into('<QQQ',fragment,0x1d0,len(ids),c.addressof(ids_array),len(ids))
                    setfield(fragment,0x24c,fragment_stamp)
                    setfield(fragment,0x250,validated)
                    s.pack_into('<QQQ',renderer,0x3f8,2,c.addressof(slots),2)
                    setfield(renderer,0x4c8,frame)
                    setfield(renderer,0x4cc,stamp)
                    before_fragment, before_slots = fragment.raw,slots.raw
                    expected_slots = bytearray(before_slots)
                    expected_stamp,expected_validated = fragment_stamp,validated
                    reason = 1 if fragment_stamp<stamp else 0
                    if not reason and validated<frame:
                        for allocation in ids:
                            index,generation = allocation&0xffff,allocation>>16
                            if index>=2: reason=2; break
                            last,stored_generation = s.unpack_from('<IH',expected_slots,index*8)
                            if stored_generation != generation or last==0:
                                expected_stamp=0
                                reason=int(0<stamp)
                                break
                            s.pack_into('<I',expected_slots,index*8,frame)
                        else:
                            expected_validated=frame
                    got = probe(fragment,renderer)
                    assert got==reason
                    assert field(fragment,0x24c)==expected_stamp
                    assert field(fragment,0x250)==expected_validated
                    assert slots.raw==bytes(expected_slots)
                    row = {'frame':frame,'renderer_stamp':stamp,'fragment_stamp':fragment_stamp,
                           'validated_frame':validated,'pattern':pattern,'branch':got,
                           'after_fragment_stamp':expected_stamp,'after_validated_frame':expected_validated,
                           'after_usage':slot_values(slots)}
                    prefix_cases.append(row)
                    if got==0 and len(warm_cases)<40:
                        c.memmove(fragment,before_fragment,len(before_fragment))
                        c.memmove(slots,before_slots,len(before_slots))
                        warm(fragment,0,0,0xffffffff,renderer)
                        assert field(fragment,0x24c)==expected_stamp
                        assert field(fragment,0x250)==expected_validated
                        assert slots.raw==bytes(expected_slots)
                        warm_cases.append(row)

# Setter prefix: exact float compare/store + current-frame stamp copy; stop
# before deallocation and etagere.clear. All those operations stay unexecuted.
setter_start=0x140b8f3f0
setter_probe=bytearray(bytes_at(setter_start,288))
epilogue=bytes.fromhex('48 83 c4 20 5b 5f 5e c3')
for address,value in [(0x140b8f421,1),(0x140b8f508,0)]:
    marker=b'\xb8'+s.pack('<I',value)+epilogue
    at=address-setter_start
    setter_probe[at:at+len(marker)]=marker
    instrumentation.append({'address':hex(address),'role':'setter-successor',
                            'replacement':marker.hex(),'original':bytes_at(address,len(marker)).hex()})
setter=c.WINFUNCTYPE(c.c_uint32,c.c_void_p,c.c_float)(executable(setter_probe))
setter_cases=[]
for frame in frames:
    for old,new in [(1.,1.),(1.,2.),(2.,3.),(0.,-0.),(float('nan'),float('nan'))]:
        obj=c.create_string_buffer(0x4e0)
        setfield(obj,0x4c8,frame)
        setfield(obj,0x4cc,7)
        s.pack_into('<f',obj,0x4d0,old)
        obj[0x4d4]=b'\x01'
        changed=setter(obj,new)
        assert changed==int(old!=new)
        assert field(obj,0x4cc)==(frame if old!=new else 7)
        assert obj.raw[0x4d4]==1
        setter_cases.append({'frame':frame,'old_bits':s.unpack('<I',s.pack('<f',old))[0],
                             'new_bits':s.unpack('<I',s.pack('<f',new))[0],
                             'changed':changed,'stamp':field(obj,0x4cc),'overflow_preserved':True})

same_frame_cases=[]
for frame in frames:
    renderer=c.create_string_buffer(0x4e0)
    fragment=c.create_string_buffer(0x260)
    setfield(renderer,0x4c8,frame)
    setfield(renderer,0x4cc,frame)
    s.pack_into('<f',renderer,0x4d0,1.)
    setfield(fragment,0x24c,frame)
    setfield(fragment,0x250,frame)
    assert setter(renderer,2.)==1
    assert probe(fragment,renderer)==0
    assert setter(renderer,3.)==1
    assert probe(fragment,renderer)==0
    assert field(renderer,0x4cc)==frame
    same_frame_cases.append({'frame':frame,'scale_changes':[2.,3.],
                             'fragment_stamp':frame,'renderer_stamp_after_both':frame,
                             'validated_frame':frame,'both_branches':'reuse',
                             'scope':'setter stamp slice and render validation prefix; redraw/metrics remain clear'})

# Original inline expiry predicate, instrument only its two successors.
expiry_start=0x140b90300
expiry_code=bytearray(bytes_at(expiry_start,0x29))
expiry_code[0:6]=bytes.fromhex('b8 00 00 00 00 c3')
expiry_code[0x23:0x29]=bytes.fromhex('b8 01 00 00 00 c3')
for address,value in [(0x140b90300,0),(0x140b90323,1)]:
    instrumentation.append({'address':hex(address),'role':'expiry-successor',
                            'original':bytes_at(address,6).hex(),
                            'replacement':(b'\xb8'+s.pack('<I',value)+b'\xc3').hex()})
expiry_base=executable(expiry_code)
expiry_wrapper=wrapper('56 53 48 89 ce 48 89 d3 48 83 ec 28',expiry_base+0x13,
                       '48 83 c4 28 5b 5e c3')
expiry=c.WINFUNCTYPE(c.c_uint32,c.c_void_p,c.c_void_p)(expiry_wrapper)
expiry_cases=[]
for frame in frames:
    for last in frames:
        renderer=c.create_string_buffer(0x4e0)
        slot=c.c_uint32(last)
        setfield(renderer,0x4c8,frame)
        got=expiry(c.byref(slot),renderer)
        expected=last!=0 and ((last+1)&0xffffffff)<frame
        assert got==int(expected)
        expiry_cases.append({'last':last,'frame':frame,'expired':bool(got)})

# Native usage helper existing-slot/append-with-capacity paths: no allocation.
usage_code=bytes_at(0x141800040,0x2c7)  # original PE RUNTIME_FUNCTION extent
usage=c.WINFUNCTYPE(None,c.c_void_p,c.c_uint64,c.c_uint32,c.c_uint32)(executable(usage_code))
usage_cases=[]
for frame in frames:
    for index in [0,2,3]:
        slots=c.create_string_buffer(b'\xa5'*32,32)
        vector=(c.c_uint64*3)(4,c.addressof(slots),3)
        usage(vector,index,frame,0xffff)
        assert s.unpack_from('<IH',slots,index*8)==(frame,0xffff)
        assert slots.raw[index*8+6:index*8+8]==b'\xa5\xa5'
        assert vector[2]==max(3,index+1)
        usage_cases.append({'frame':frame,'index':index,'generation':65535,'vector_len':vector[2],'pad_preserved':True})

def rsi_leaf(address,length):
    body=executable(bytes_at(address,length)+b'\xc3')
    thunk=wrapper('56 48 89 ce 48 83 ec 20',body,'48 83 c4 20 5e c3')
    return c.WINFUNCTYPE(None,c.c_void_p)(thunk)
construct=rsi_leaf(0x140b8ec5b,17)
copy_stamp=rsi_leaf(0x140b8fc21,12)
advance=rsi_leaf(0x140fac968,6)
sequence_cases=[]
for frame in frames:
    obj=c.create_string_buffer(0x4e0)
    setfield(obj,0x4c8,frame)
    copy_stamp(obj)
    assert field(obj,0x4cc)==frame
    advance(obj)
    assert field(obj,0x4c8)==((frame+1)&0xffffffff)
    sequence_cases.append({'before_frame':frame,'overflow_stamp':field(obj,0x4cc),'after_consume_frame':field(obj,0x4c8)})
obj=c.create_string_buffer(0x4e0)
construct(obj)
assert field(obj,0x4c8)==field(obj,0x4cc)==1

report={'binary_sha256':binary_sha,'full_game_or_gpu_execution':False,'production_adapter':False,
        'scope':'unmodified warm-render leaf paths plus original bounded instruction slices with successor markers',
        'ranges':bound,'instrumentation':instrumentation,
        'slices':[{'address':hex(address),'size':length,'sha256':hashlib.sha256(bytes_at(address,length)).hexdigest()}
                  for address,length in [(0x140b911b8,0xad),(0x140b8f3f0,0x31),(0x140b90313,0x10),
                                         (0x141800040,0x2c7),(0x140b8ec5b,17),(0x140b8fc21,12),(0x140fac968,6)]],
        'prefix_cases':prefix_cases,'warm_render_cases':warm_cases,'setter_prefix_cases':setter_cases,
        'expiry_cases':expiry_cases,'usage_helper_cases':usage_cases,'overflow_consume_store_cases':sequence_cases,
        'same_frame_scale_cases':same_frame_cases,
        'constructor_frame':1,'constructor_stamp':1,'mismatches':0,
        'passed':len(prefix_cases)+len(warm_cases)+len(setter_cases)+len(expiry_cases)+len(usage_cases)+len(sequence_cases)+len(same_frame_cases)+1}
(out/'native-cache-oracle.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
for p in allocations: assert k.VirtualFree(p,0,0x8000)
print(json.dumps({'passed':report['passed'],'prefix':len(prefix_cases),'warm_complete_function':len(warm_cases),'mismatches':0}))
