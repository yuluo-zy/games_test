"""Independent original448B TOD wrapper + original JSON parser execution.
The filesystem read is an explicit fixture boundary; no game/serde hook.
"""
from pathlib import Path
_template=Path(__file__).resolve().parent.parent/'color-startup/native_grading_oracle.py'
exec(_template.read_text(encoding='utf8').split('try:\n',1)[0])
try:
    put(base,raw[:struct.unpack_from('<I',raw,opt+60)[0]])
    sec=opt+struct.unpack_from('<H',raw,pe+20)[0]
    for i in range(struct.unpack_from('<H',raw,pe+6)[0]):
        size,rva,n,ptr=struct.unpack_from('<IIII',raw,sec+i*40+8)
        if n:put(base+rva,raw[ptr:ptr+n])
    assert hashlib.sha256(read(0x14089d6f0,448)).hexdigest()==json.loads((OUT/'native-review-identity.json').read_text())['ranges'][4]['sha256']
    crt=C.CDLL('ucrtbase')
    deallocated=[]
    @C.CFUNCTYPE(None,C.c_void_p,C.c_uint64,C.c_uint64)
    def deallocate_probe(pointer,size,alignment):deallocated.append((pointer,size,alignment))
    @C.CFUNCTYPE(C.c_void_p,C.c_void_p,C.c_uint64,C.c_uint64,C.c_uint64)
    def reallocate(pointer,old,alignment,new):
        result=allocate(new,alignment);C.memmove(result,pointer,min(old,new));return result
    for va,fn in [(0x140613c10,allocate),(0x140613c20,deallocate_probe),(0x140613c30,reallocate),(0x140613c40,allocate),(0x142923c00,memory_move),(0x142923c10,memory_move),(0x142923bf0,crt.memcmp)]:
        put(va,b'\x48\xb8'+struct.pack('<Q',C.cast(fn,C.c_void_p).value)+b'\xff\xe0')
    blob=(ROOT/'ljxsj_92385/Tiny Glade/assets/tod/default.json').read_bytes()
    text_owner=C.create_string_buffer(blob)
    seen=[]
    @C.CFUNCTYPE(C.c_void_p,C.c_void_p,C.c_void_p,C.c_uint64)
    def read_file(out,path,n):
        seen.append(read(path,n).decode())
        put(out,struct.pack('<QQQ',len(blob),C.addressof(text_owner),len(blob)))
        return out
    put(0x140d59140,b'\x48\xb8'+struct.pack('<Q',C.cast(read_file,C.c_void_p).value)+b'\xff\xe0')
    previous=allocate(3*80,8)
    owner=C.create_string_buffer(struct.pack('<QQQ',3,previous,3))
    tick=C.c_uint32(0);param=C.create_string_buffer(32)
    put(C.addressof(param),struct.pack('<Q',C.addressof(owner)))
    put(C.addressof(param)+0x10,struct.pack('<Q',C.addressof(tick)))
    put(C.addressof(param)+0x1c,struct.pack('<I',81))
    fn=C.CFUNCTYPE(None,C.c_void_p)(0x14089d6f0)
    fn(param)
    cap,ptr,n=struct.unpack_from('<QQQ',owner.raw)
    assert n==15 and tick.value==81 and seen==['assets/tod/default.json']
    entries=[]
    for i in range(n):
        b=read(ptr+i*80,80)
        entries.append({'index':i,'raw_hex':b.hex(),'temp_ui_id':struct.unpack_from('<Q',b)[0],
          'time_of_day_bits':struct.unpack_from('<I',b,8)[0],'snapshot_f32_bits':list(struct.unpack_from('<17I',b,12))})
    expected=json.loads(blob)
    assert [struct.unpack('<f',struct.pack('<I',r['time_of_day_bits']))[0] for r in entries]==[
      struct.unpack('<f',struct.pack('<f',e['time_of_day']))[0] for e in expected['entries']]
    for result,entry in zip(entries,expected['entries']):
        snapshot=entry['snapshot'];colors=snapshot['sun_sky']
        values=sum([colors[k] for k in ['sun_tint_color','moon_tint_color','sky_low_color','sky_high_color']],[])
        values+=[snapshot['base_fog_thickness'],*snapshot['fog_tint'],snapshot['speed_bias']]
        bits=list(struct.unpack('<17I',struct.pack('<17f',*values)))
        assert result['temp_ui_id']==entry['temp_ui_id'] and result['snapshot_f32_bits']==bits
    assert (previous,240,8) in deallocated
    (OUT/'original-tod-wrapper-execution.json').write_text(json.dumps({
      'original_pe_sha256':hashlib.sha256(raw).hexdigest(),'asset_sha256':hashlib.sha256(blob).hexdigest(),
      'read_paths':seen,'changed_tick':tick.value,'entry_stride':80,'entries':entries,
      'all17_snapshot_floats_and_ids_matched':True,'nonempty_previous_owner_deallocated':{'entries':3,'bytes':240,'alignment':8},
      'scope':'Original full wrapper and original JSON parser; generic allocator/CRT bridges plus fixture filesystem read only. No game/serde replacement.'},indent=2),encoding='utf8')
    print('original448B TOD wrapper + original parser:15 entries, required file path, actual owner+change tick verified')
finally:k.VirtualFree(base,0,0x8000)
