"""Bounded original arch material-classification block760483..760628.

Original business instructions and RIP constants execute unchanged. The two
continuation addresses760628/7606f5 exit through a caller register, delimiting
this observed block before profile execution. No private helper is replaced.
"""
import ctypes,hashlib,json,pathlib,struct
root=pathlib.Path('D:/game');data=(root/'ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(data).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=struct.unpack_from('<I',data,60)[0];opt=p+24;sh=opt+struct.unpack_from('<H',data,p+20)[0]
k=ctypes.WinDLL('kernel32',use_last_error=True);k.VirtualAlloc.restype=ctypes.c_void_p
k.VirtualAlloc.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_uint32,ctypes.c_uint32]
k.VirtualFree.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_uint32]
size=struct.unpack_from('<I',data,opt+56)[0];headers=struct.unpack_from('<I',data,opt+60)[0]
base=k.VirtualAlloc(None,size,0x3000,0x40);assert base;ctypes.memmove(base,data,headers)
for i in range(struct.unpack_from('<H',data,p+6)[0]):
    vs,vr,rs,ro=struct.unpack_from('<4I',data,sh+i*40+8)
    if rs:ctypes.memmove(base+vr,data[ro:ro+rs],rs)
prefix=bytes.fromhex('555356574155415641574883ec10f30f7f34244889cd4889d64d89c54c89cf4531ffba0000803f660f6ef249bb')
epilogue=bytes.fromhex('f30f6f34244883c410415f415e415d5f5e5b5dc3')
code=k.VirtualAlloc(None,512,0x3000,0x40);assert code
exit_at=code+len(prefix)+8+2+8+2
trampoline=prefix+struct.pack('<Q',exit_at)+bytes.fromhex('48b8')+struct.pack('<Q',base+0x760483)+bytes.fromhex('ffe0')+epilogue
ctypes.memmove(code,trampoline,len(trampoline))
for va in [0x140760628,0x1407606f5]:ctypes.memmove(base+va-0x140000000,bytes.fromhex('41ffe3'),3)
call=ctypes.WINFUNCTYPE(None,ctypes.c_void_p,ctypes.c_uint64,ctypes.c_uint64,ctypes.c_void_p)(code)
cases=[]
for has_roof in [False,True]:
    for count in [0,1,2,3]:
        for ys in [[],[1.],[-1.],[-1.,1.],[-1.,-1.,1.],[-1.,1.,1.],[0.,-0.,-1.,1.]]:
            for length in [0.,-0.,3.5]:
                backing=ctypes.create_string_buffer(0x410);frame=(ctypes.addressof(backing)+15)&~15
                mask=ctypes.create_string_buffer(136);struct.pack_into('<f',mask,132,length)
                points=ctypes.create_string_buffer(max(1,len(ys)*12))
                for i,y in enumerate(ys):struct.pack_into('<3f',points,i*12,float(i),y,0.)
                segment=ctypes.create_string_buffer(104);struct.pack_into('<QQ',segment,32,ctypes.addressof(points),len(ys))
                public=ctypes.create_string_buffer(16)
                struct.pack_into('<Q',backing,frame-ctypes.addressof(backing)+0x2e8,ctypes.addressof(mask))
                struct.pack_into('<Q',backing,frame-ctypes.addressof(backing)+0x290,ctypes.addressof(public)+12)
                call(frame,count,int(has_roof),ctypes.addressof(segment))
                lengths=list(struct.unpack('<3I',ctypes.string_at(frame+0x2a4,12)))
                gate=struct.unpack('<Q',ctypes.string_at(frame+0x188,8))[0]
                sewer=struct.unpack('<Q',ctypes.string_at(frame+0x140,8))[0]
                cases.append({'has_roof':has_roof,'segments':count,'points_y':ys,'mask_length_bits':struct.unpack('<I',struct.pack('<f',length))[0],'lengths':lengths,'gates':gate,'sewer':sewer})
assert k.VirtualFree(code,0,0x8000);assert k.VirtualFree(base,0,0x8000)
out=pathlib.Path(__file__).with_name('arch-stats-native-cases.json');out.write_text(json.dumps({'source_sha256':hashlib.sha256(data).hexdigest(),'entry':'140760483','continuation_boundaries':['140760628','1407606f5'],'scope':'Original classification only, resolved materialized-count/positions/mask length/has_roof. Full materializer/profile/event/pool excluded. Empty segment position boundary included as raw native contract, not asserted valid game input.','cases':cases},indent=2))
print(json.dumps({'cases':len(cases),'output':str(out)}))
