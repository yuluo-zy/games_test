"""Bounded original CPU owners with dependency-only allocator/math bridges."""
import ctypes,hashlib,pathlib,struct
class NativeCpuImage:
    def __init__(self):
        self.data=pathlib.Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
        self.sha=hashlib.sha256(self.data).hexdigest();assert self.sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
        self.pe=struct.unpack_from('<I',self.data,60)[0];self.opt=self.pe+24
        sh=self.opt+struct.unpack_from('<H',self.data,self.pe+20)[0]
        self.sections=[struct.unpack_from('<4I',self.data,sh+i*40+8) for i in range(struct.unpack_from('<H',self.data,self.pe+6)[0])]
        self.k=ctypes.WinDLL('kernel32',use_last_error=True)
        self.k.VirtualAlloc.restype=ctypes.c_void_p;self.k.VirtualAlloc.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_uint32,ctypes.c_uint32]
        self.k.VirtualFree.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_uint32]
        size=struct.unpack_from('<I',self.data,self.opt+56)[0];headers=struct.unpack_from('<I',self.data,self.opt+60)[0]
        self.base=self.k.VirtualAlloc(None,size,0x3000,0x40);assert self.base;ctypes.memmove(self.base,self.data,headers)
        for vs,vr,rs,ro in self.sections:
            if rs:ctypes.memmove(self.base+vr,self.data[ro:ro+rs],rs)
        self.live={};self.callbacks=[];self.bridges=[];self.libraries=[]
    def address(self,va):return self.base+va-0x140000000
    def raw(self,va,n):return ctypes.string_at(self.address(va),n)
    def jump(self,va,target):ctypes.memmove(self.address(va),b'\x48\xb8'+struct.pack('<Q',target)+b'\xff\xe0',12)
    def dependency_callback(self,va,signature,fn,name):
        callback=signature(fn);self.callbacks.append(callback);self.jump(va,ctypes.cast(callback,ctypes.c_void_p).value);self.bridges.append({'entry':hex(va),'dependency':name})
    def bridge_allocator(self):
        def allocate(size,align):
            assert size<64*1024*1024 and align and align&(align-1)==0,(size,align)
            buf=ctypes.create_string_buffer(size+align);p=(ctypes.addressof(buf)+align-1)&-align;self.live[p]=buf;return p
        def deallocate(p,size,align):self.live.pop(p,None)
        def reallocate(p,old,align,new):
            q=allocate(new,align);ctypes.memmove(q,p,min(old,new));deallocate(p,old,align);return q
        self.deallocate=deallocate
        self.dependency_callback(0x140613c10,ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_size_t,ctypes.c_size_t),allocate,'std::alloc::__rust_alloc')
        self.dependency_callback(0x140613c20,ctypes.WINFUNCTYPE(None,ctypes.c_void_p,ctypes.c_size_t,ctypes.c_size_t),deallocate,'std::alloc::__rust_dealloc')
        self.dependency_callback(0x140613c30,ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_void_p,ctypes.c_size_t,ctypes.c_size_t,ctypes.c_size_t),reallocate,'std::alloc::__rust_realloc')
        self.dependency_callback(0x140613c40,ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_size_t,ctypes.c_size_t),allocate,'std::alloc::__rust_alloc_zeroed')
    def resolve_math_import(self,thunk):
        code=self.raw(thunk,6);assert code[:2]==b'\xff\x25',code.hex()
        iat=thunk+6+struct.unpack_from('<i',code,2)[0]-0x140000000
        imports=struct.unpack_from('<I',self.data,self.opt+112+8)[0]
        def z(rva):return ctypes.string_at(self.base+rva).decode()
        descriptor=self.base+imports
        while True:
            lookup,_,_,name,first=struct.unpack('<5I',ctypes.string_at(descriptor,20))
            assert first,'import slot not found'
            count=0
            while struct.unpack('<Q',ctypes.string_at(self.base+(lookup or first)+count*8,8))[0]:count+=1
            index=(iat-first)//8
            if first<=iat<first+count*8 and index>=0 and (iat-first)%8==0:
                named=struct.unpack('<Q',ctypes.string_at(self.base+(lookup or first)+index*8,8))[0]
                if named:
                    library=ctypes.WinDLL(z(name));symbol=z(named+2)
                    fn=getattr(library,symbol);self.libraries.append(library)
                    ctypes.memmove(self.base+iat,struct.pack('<Q',ctypes.cast(fn,ctypes.c_void_p).value),8)
                    self.bridges.append({'thunk':hex(thunk),'dependency':z(name)+'!'+symbol});return symbol
            descriptor+=20
    def close(self):
        self.live.clear();assert self.k.VirtualFree(self.base,0,0x8000)
