"""Native64B SparseWriter push/clear, bounded finite-pool fixture.

The ONLY bridges are parking_lot deadlock observer acquire/release, Cargo
dependency functions140c81c70/140c81e10. No private game or sparse algorithm
body is replaced. All vectors are pre-reserved, no allocator call is needed.
"""
import ctypes,hashlib,json,pathlib,random,struct,sys
resume_tail='--resume-tail' in sys.argv
brick='--bricks' in sys.argv
element_size=96 if brick else 64
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
for va in [0x140c81c70,0x140c81e10]:ctypes.memmove(base+va-0x140000000,b'\xc3',1)
push_address=0x14120c120 if brick else 0x14120c430
push=ctypes.WINFUNCTYPE(ctypes.c_uint8,ctypes.c_void_p,ctypes.c_void_p)(base+push_address-0x140000000)
clear=ctypes.WINFUNCTYPE(None,ctypes.c_void_p)(base+0x120c9d0)
def addr(buf):return ctypes.addressof(buf)
class Pool:
    def __init__(self,n):
        self.n=n;self.counts=ctypes.create_string_buffer(n+1);self.counts[n]=bytes([32])
        self.active=ctypes.create_string_buffer(8);self.dirty=ctypes.create_string_buffer(8)
        self.data=ctypes.create_string_buffer(n*32*element_size)
        self.freeids=ctypes.create_string_buffer(1024)
        struct.pack_into('<'+'I'*n,self.freeids,0,*reversed(range(n)))
        self.free=ctypes.create_string_buffer(32);struct.pack_into('<4Q',self.free,0,0,256,addr(self.freeids),n)
        self.alloc=[];self.ids=[]
        for _ in range(2):
            ids=ctypes.create_string_buffer(1024);alloc=ctypes.create_string_buffer(32)
            struct.pack_into('<3Q',alloc,0,256,addr(ids),0);self.ids.append(ids);self.alloc.append(alloc)
        self.writer=ctypes.create_string_buffer(88)
    def begin(self,wall):
        length=struct.unpack_from('<Q',self.alloc[wall],16)[0]
        current=struct.unpack_from('<I',self.ids[wall],(length-1)*4)[0] if resume_tail and length else self.n
        struct.pack_into('<10QI',self.writer,0,addr(self.counts),self.n+1,addr(self.active),1,addr(self.alloc[wall]),addr(self.data),self.n*32,addr(self.free),addr(self.dirty),1,current)
    def snapshot(self):
        free_n=struct.unpack_from('<Q',self.free,24)[0]
        lengths=[struct.unpack_from('<Q',a,16)[0] for a in self.alloc]
        return {'counts':list(bytes(self.counts)),'active':struct.unpack_from('<Q',self.active)[0],'dirty':struct.unpack_from('<Q',self.dirty)[0],
                'free':list(struct.unpack_from('<'+'I'*free_n,self.freeids)),'allocations':[list(struct.unpack_from('<'+'I'*n,ids)) for n,ids in zip(lengths,self.ids)],
                'data':bytes(self.data).hex()}
rng=random.Random(0x120c430);cases=[]
for n in (1,2,3):
    for scenario in range(8):
        pool=Pool(n);steps=[];seed=1
        operations=[(0,'clear',0),(0,'push',rng.randrange(1,n*32+34)),(0,'push',rng.randrange(1,42)),(1,'push',rng.randrange(1,42)),(0,'clear',0),(1,'push',rng.randrange(1,n*32+18)),(1,'clear',0),(0,'push',n*32+3)]
        for wall,op,count in operations:
            pool.begin(wall);failures=[]
            if op=='clear':clear(addr(pool.writer))
            else:
                for _ in range(count):
                    words=element_size//4
                    raw=struct.pack('<'+'I'*words,*[seed*words+j for j in range(words)])
                    record=ctypes.create_string_buffer(raw)
                    failures.append(bool(push(addr(pool.writer),addr(record))))
                    seed+=1
            steps.append({'wall':wall,'operation':op,'count':count,'failures':failures,'state':pool.snapshot()})
        cases.append({'chunks':n,'steps':steps})
assert k.VirtualFree(base,0,0x8000)
out=pathlib.Path(__file__).with_name(('brick' if brick else 'plank')+('-pool-tail-native-cases.json' if resume_tail else '-pool-native-cases.json'))
out.write_text(json.dumps({'source_sha256':hashlib.sha256(data).hexdigest(),'entries':[hex(push_address),'14120c9d0'],'element_size':element_size,'resume_allocation_tail':resume_tail,'dependency_bridges':['parking_lot deadlock acquire_resource140c81c70','parking_lot deadlock release_resource140c81e10'],'cases':cases,'scope':'Uncontended native sparse push/clear and fully reserved Vec storage. Geometry production and GPU upload excluded. Every push is attempted, including after capacity failure.'},indent=2))
print(json.dumps({'cases':len(cases),'steps':sum(len(c['steps']) for c in cases),'output':str(out)}))
