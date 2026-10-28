exec(open('D:/game/tiny-glade/work/m01-scratch/rotate_fragment_fixture.py').read().split('cases=')[0])
import ctypes
from unicorn import UC_HOOK_CODE
crt=ctypes.CDLL('ucrtbase.dll')
for name in ['_hypotf','atan2f']:
 f=getattr(crt,name);f.argtypes=[ctypes.c_float,ctypes.c_float];f.restype=ctypes.c_float
def polar(x,z):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(size+4095)&~4095);u.mem_write(base,bytes(image));u.mem_map(0x100000,0x10000);u.reg_write(UC_X86_REG_RSP,0x108008);u.mem_write(0x108008,struct.pack('<Q',0x140c90e6a));u.reg_write(UC_X86_REG_XMM0,vec(x,0,0,0));u.reg_write(UC_X86_REG_XMM1,vec(z,0,0,0))
 def hook(uc,address,n,data):
  name={0x142923cf0:'_hypotf',0x142923e20:'atan2f'}.get(address)
  if name:
   a,b=[struct.unpack('<f',struct.pack('<I',uc.reg_read(r)&0xffffffff))[0] for r in [UC_X86_REG_XMM0,UC_X86_REG_XMM1]]
   result=getattr(crt,name)(a,b);uc.reg_write(UC_X86_REG_XMM0,vec(result,0,0,0));sp=uc.reg_read(UC_X86_REG_RSP);ret=struct.unpack('<Q',uc.mem_read(sp,8))[0];uc.reg_write(UC_X86_REG_RSP,sp+8);uc.reg_write(UC_X86_REG_RIP,ret)
 u.hook_add(UC_HOOK_CODE,hook);u.emu_start(0x140c90e20,0x140c90e6a)
 return [f'{u.reg_read(r)&0xffffffff:08x}' for r in [UC_X86_REG_XMM0,UC_X86_REG_XMM1]]
cases=[(1.,0.),(0.,1.),(-1.,0.),(0.,-1.),(0.,0.),(-0.,0.),(0.,-0.),(-0.,-0.),(float('nan'),1.),(float('inf'),1.)]
out={'scope':'Original PE polar wrapper 140c90e20..e69 in Unicorn; imported _hypotf and atan2f bridged to host ucrtbase.dll, not original shipped CRT or full game execution','pe_sha256':hashlib.sha256(raw).hexdigest(),'cases':[{'x':str(x),'z':str(z),'radius_bits':polar(x,z)[0],'angle_bits':polar(x,z)[1]} for x,z in cases]}
Path('D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/wall/m01-polar-fragment-fixture.json').write_text(json.dumps(out,indent=2));print(json.dumps(out,indent=2))
