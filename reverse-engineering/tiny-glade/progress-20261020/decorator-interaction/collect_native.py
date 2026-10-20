"""Bounded B instructions; hooks only dependency boundaries, never game branches."""
from pathlib import Path
import hashlib, json, struct
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *

ROOT = Path('D:/game')
OUT = Path(__file__).parent
INPUT = ROOT/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe'
raw = INPUT.read_bytes()
digest = hashlib.sha256(raw).hexdigest()
assert digest == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = pefile.PE(data=raw)
base = pe.OPTIONAL_HEADER.ImageBase
image = pe.get_memory_mapped_image()
uc = Uc(UC_ARCH_X86, UC_MODE_64)
uc.mem_map(base, (len(image)+4095)&~4095)
uc.mem_write(base, image)
mem, stack, stop = 0x20000000, 0x21000000, 0x22000000
uc.mem_map(mem, 0x10000)
uc.mem_map(stack, 0x10000)
uc.mem_map(stop, 0x1000)

def p64(value): return struct.pack('<Q', value)
def f32(value): return struct.pack('<f', value)
def ret():
    sp=uc.reg_read(UC_X86_REG_RSP)
    uc.reg_write(UC_X86_REG_RIP, struct.unpack('<Q',uc.mem_read(sp,8))[0])
    uc.reg_write(UC_X86_REG_RSP,sp+8)

mode, receipt, height, position = None, None, 2.25, (3.5,7.25,-4.0)
grab_hit = None
offset_state = {}
def read_floats(address,n): return struct.unpack('<'+'f'*n,uc.mem_read(address,n*4))
def hook(machine, address, size, unused):
    global receipt
    if address == stop:
        uc.emu_stop(); return
    if address == 0x142923c00: # CRT memcpy, not game policy.
        target=uc.reg_read(UC_X86_REG_RCX)
        uc.mem_write(target,bytes(uc.mem_read(uc.reg_read(UC_X86_REG_RDX),uc.reg_read(UC_X86_REG_R8))))
        uc.reg_write(UC_X86_REG_RAX,target); ret()
    elif mode == 'widget' and address in (0x1409236d0,0x14125d180):
        receipt = bytes(uc.mem_read(uc.reg_read(UC_X86_REG_R9),0xb0))
        uc.reg_write(UC_X86_REG_RAX,0); uc.reg_write(UC_X86_REG_RDX,0); ret()
    elif mode == 'widget' and address == 0x1406868b0:
        ret()
    elif mode == 'grab' and address == 0x140949c10:
        uc.reg_write(UC_X86_REG_XMM0,int.from_bytes(f32(200.),'little'))
        uc.reg_write(UC_X86_REG_XMM1,int.from_bytes(f32(100.),'little')); ret()
    elif mode == 'grab' and address == 0x140ac8780:
        x,y,z=read_floats(uc.reg_read(UC_X86_REG_RCX),3)
        uc.reg_write(UC_X86_REG_XMM0,int.from_bytes(f32(2*x+z),'little'))
        uc.reg_write(UC_X86_REG_XMM1,int.from_bytes(f32(3*y),'little')); ret()
    elif mode == 'grab' and address == 0x141278050:
        uc.mem_write(uc.reg_read(UC_X86_REG_RCX),grab_hit); ret()
    elif mode == 'grab' and address == 0x140a1c240:
        ptr=uc.reg_read(UC_X86_REG_RCX)
        tag,wall=struct.unpack('<QQ',uc.mem_read(ptr,16))
        assert tag==0
        uc.reg_write(UC_X86_REG_RAX,wall); ret()
    elif mode == 'grab' and address == 0x140a91ef0:
        uc.reg_write(UC_X86_REG_RAX,mem+0x5000); ret()
    elif mode == 'offset' and address == 0x140949c10:
        uc.reg_write(UC_X86_REG_XMM0,int.from_bytes(f32(200.),'little'))
        uc.reg_write(UC_X86_REG_XMM1,int.from_bytes(f32(100.),'little')); ret()
    elif mode == 'offset' and address == 0x14127cab0:
        uc.mem_write(uc.reg_read(UC_X86_REG_RCX),struct.pack('<6f',*offset_state['ray'])); ret()
    elif mode == 'offset' and address == 0x1408cc2e0:
        sp=uc.reg_read(UC_X86_REG_RSP)
        offset_state['mask']=struct.unpack('<I',uc.mem_read(sp+0x28,4))[0]
        result=bytearray(64)
        struct.pack_into('<Q',result,0,0)
        struct.pack_into('<Q',result,8,0x9876)
        struct.pack_into('<fff',result,0x18,17.,18.,19.)
        struct.pack_into('<fff',result,0x24,.25,.5,.75)
        struct.pack_into('<I',result,0x34,0 if offset_state['physical'] else 2)
        uc.mem_write(uc.reg_read(UC_X86_REG_RCX),bytes(result)); ret()
    elif mode == 'offset' and address == 0x1412773f0:
        offset_state['terrain_calls']+=1
        result=bytearray(24)
        struct.pack_into('<I',result,0,int(offset_state['terrain']))
        struct.pack_into('<fff',result,8,7.,8.,9.)
        uc.mem_write(uc.reg_read(UC_X86_REG_RCX),bytes(result)); ret()
    elif mode == 'move' and address == 0x140a1c290:
        ptr=uc.reg_read(UC_X86_REG_RCX)
        tag, wall=struct.unpack('<QQ',uc.mem_read(ptr,16))
        uc.reg_write(UC_X86_REG_RAX, int(tag==0)); uc.reg_write(UC_X86_REG_RDX,wall); ret()
    elif mode == 'move' and address == 0x1405b0d70:
        uc.reg_write(UC_X86_REG_RAX,0); ret()
    elif mode == 'move' and address == 0x140b2a890:
        uc.reg_write(UC_X86_REG_XMM0,int.from_bytes(f32(height),'little')); ret()
    elif mode == 'move' and address == 0x140c92090:
        uc.mem_write(uc.reg_read(UC_X86_REG_RCX),struct.pack('<fff',*position)); ret()
    elif mode == 'move' and address in (0x1428d9330,0x1428d9310):
        raise RuntimeError('Original wall candidate panicked')
uc.hook_add(UC_HOOK_CODE,hook)

def start(va):
    uc.mem_write(stack,bytes(0x10000))
    sp=stack+0x8008
    uc.mem_write(sp,p64(stop))
    uc.reg_write(UC_X86_REG_RSP,sp)
    return sp
def run(va): uc.emu_start(va,stop,count=200000)

ranges=[('calculate_move_dst',0x1412801f0,512),('create_widget_handle',0x141282300,187),
        ('create_widget_str',0x1412823c0,187),('handle_hash',0x14128c8e0,392),
        ('grab_offset',0x141277bb0,1174),('raycast_offset',0x141278050,499)]
manifest={'input':str(INPUT),'sha256':digest,'ranges':[],'scope':'B only; no live gameplay or whole UI completion claim'}
cs=Cs(CS_ARCH_X86,CS_MODE_64)
for name,va,length in ranges:
    code=pe.get_data(va-base,length)
    (OUT/(name+'.bin')).write_bytes(code)
    (OUT/(name+'.asm')).write_text('\n'.join(f'{i.address:#x}: {i.mnemonic} {i.op_str}' for i in cs.disasm(code,va))+'\n')
    unwind=[{'begin':hex(e.struct.BeginAddress+base),'end':hex(e.struct.EndAddress+base)} for e in pe.DIRECTORY_ENTRY_EXCEPTION if e.struct.BeginAddress+base==va]
    manifest['ranges'].append({'name':name,'va':hex(va),'bytes':length,'sha256':hashlib.sha256(code).hexdigest(),'unwind':unwind})

# Execute the actual complete hash helper and its std SipHasher write calls.
mode='hash'
hashes=[]
for key in (0,1,0xffffffffffffffff,0x123456789abcdef0,0x0000000100000002):
    for kind in (0,1,2,0x3f,0xff):
        start(0x14128c8e0)
        uc.mem_write(mem,p64(key)+bytes([kind])+bytes(7))
        uc.reg_write(UC_X86_REG_RCX,mem)
        run(0x14128c8e0)
        hashes.append({'key':key,'kind':kind,'hash':uc.reg_read(UC_X86_REG_RAX)})
(OUT/'handle-hash-goldens.json').write_text(json.dumps(hashes,indent=2)+'\n')

mode='widget'
widgets=[]
for entry in (0x141282300,0x1412823c0):
    for distance in (0.,-0.,1.,-4.5,125.,float('inf')):
        for question in (False,True):
            sp=start(entry)
            sounds=bytes((i*7+3)&255 for i in range(96))
            uc.mem_write(mem+0x100,sounds)
            uc.mem_write(sp+0x28,bytes([0x3f if question else 0x38]))
            uc.mem_write(sp+0x30,p64(mem+0x100))
            uc.reg_write(UC_X86_REG_RCX,mem+0x200)
            uc.reg_write(UC_X86_REG_RDX,mem+0x300)
            uc.reg_write(UC_X86_REG_R8,7)
            uc.reg_write(UC_X86_REG_XMM3,int.from_bytes(f32(distance),'little'))
            receipt=None
            run(entry)
            assert receipt is not None
            widgets.append({'entry':hex(entry),'distance_bits':int.from_bytes(f32(distance),'little'),'question':question,
                            'volume_tag':receipt[0x88],'volume_distance_bits':int.from_bytes(receipt[0x8c:0x90],'little'),
                            'occlude':receipt[0xa8],'flags':receipt[0xa9],'sounds_hex':receipt[0x18:0x78].hex()})
(OUT/'widget-goldens.json').write_text(json.dumps(widgets,indent=2)+'\n')

mode='move'
moves=[]
for tag in range(4):
    for side in (0,1):
        sp=start(0x1412801f0)
        old=bytes((i*13+11)&255 for i in range(72))
        intent=bytearray(64)
        struct.pack_into('<I',intent,0x20,tag)
        struct.pack_into('<f',intent,0x24,-.75)
        struct.pack_into('<ff',intent,0x28,1.25,7.25)
        struct.pack_into('<Q',intent,0x30,0x1234)
        if tag==3: struct.pack_into('<Q',intent,0x28,0x0000000500000003)
        if tag==1:
            struct.pack_into('<QQ',intent,0x10,0,0x9876)
            intent[0x38:0x3b]=bytes([3,1,side])
        uc.mem_write(mem,old)
        uc.mem_write(mem+0x100,bytes(intent))
        # One valid hashbrown bucket. Library hashing is a fixed boundary here.
        ctrl=mem+0x3000
        uc.mem_write(ctrl-0x2a8,p64(0x9876)+bytes(0x2a0))
        uc.mem_write(ctrl,bytes([0])+bytes([0xff])*31)
        uc.mem_write(mem+0x200,p64(ctrl)+p64(15)+p64(14)+p64(1)+bytes(32))
        uc.reg_write(UC_X86_REG_RCX,mem)
        uc.reg_write(UC_X86_REG_RDX,mem+0x100)
        uc.reg_write(UC_X86_REG_R8,mem+0x200)
        run(0x1412801f0)
        moves.append({'tag':tag,'side':side,'before_hex':old.hex(),'intent_hex':intent.hex(),'after_hex':bytes(uc.mem_read(mem,72)).hex(),
                      'boundary_height':height,'boundary_position':position})
(OUT/'move-destination-goldens.json').write_text(json.dumps(moves,indent=2)+'\n')

mode='grab'
grabs=[]
for tag in (0,1,2,3,4):
    for ty in range(0x37,0x41):
        for hit_tag,wall,first,second in [(0,0x9876,3,5),(2,0x9876,3,5),(0,0x1234,3,5),(11,0,0,0),(13,0,3,5),(13,0,4,5),(13,0,3,6),(9,0,0,0)]:
            sp=start(0x141277bb0)
            info=bytearray(72)
            struct.pack_into('<Q',info,0,0x0000000500000003)
            info[0x22]=tag; info[0x46]=ty
            hit=bytearray(64)
            struct.pack_into('<Q',hit,0,hit_tag|(first<<32))
            struct.pack_into('<Q',hit,8,second if hit_tag==13 else wall)
            struct.pack_into('<fff',hit,0x18,1.25,2.5,3.75)
            grab_hit=bytes(hit)
            uc.mem_write(mem,bytes(16))
            uc.mem_write(mem+0x100,struct.pack('<fffffff',0.,0.,0.,1.,1.,2.,3.))
            uc.mem_write(mem+0x200,bytes(info))
            uc.mem_write(mem+0x300,struct.pack('<QQ',0,0x9876))
            uc.mem_write(mem+0x400,p64(mem+0x600)+bytes(24)+p64(mem+0x700))
            uc.mem_write(mem+0x5000+0x258,struct.pack('<I',2))
            for off,value in [(0x30,mem+0x300),(0x50,mem+0x400),(0x58,mem+0x800)]:uc.mem_write(sp+off,p64(value))
            uc.mem_write(sp+0x40,f32(4.));uc.mem_write(sp+0x48,f32(5.))
            uc.reg_write(UC_X86_REG_RCX,mem)
            uc.reg_write(UC_X86_REG_RDX,mem+0x100)
            uc.reg_write(UC_X86_REG_R8,mem+0x200)
            uc.reg_write(UC_X86_REG_R9,mem+0x900)
            run(0x141277bb0)
            result=bytes(uc.mem_read(mem,12))
            present=struct.unpack_from('<I',result)[0]==1
            grabs.append({'destination_tag':tag,'type':ty,'hit_hex':hit.hex(),
                          'present':present,'offset_bits':list(struct.unpack_from('<II',result,4)) if present else None})
(OUT/'grab-offset-goldens.json').write_text(json.dumps(grabs,indent=2)+'\n')

mode='offset'
offsets=[]
for ty in (0x38,0x3f):
    for physical in (False,True):
        for terrain in (False,True):
            for dy in (-1.,0.,1.):
                sp=start(0x141278050)
                offset_state.clear()
                offset_state.update(physical=physical,terrain=terrain,ray=[1.,2.,3.,2.,dy,4.],terrain_calls=0)
                uc.mem_write(mem,bytes(64))
                uc.mem_write(mem+0x400,p64(mem+0x600)+bytes(24)+p64(mem+0x700))
                uc.mem_write(sp+0x30,bytes([ty]))
                uc.mem_write(sp+0x38,p64(mem+0x400));uc.mem_write(sp+0x40,p64(mem+0x800))
                uc.reg_write(UC_X86_REG_RCX,mem)
                for reg in [UC_X86_REG_XMM1,UC_X86_REG_XMM2,UC_X86_REG_XMM3]:uc.reg_write(reg,0)
                uc.mem_write(sp+0x28,f32(0.))
                run(0x141278050)
                output=bytes(uc.mem_read(mem,64))
                offsets.append({'type':ty,'physical':physical,'terrain':terrain,'direction_y':dy,
                                'mask':offset_state['mask'],'terrain_calls':offset_state['terrain_calls'],
                                'source_tag':output[0],'position_bits':list(struct.unpack_from('<III',output,0x18)),
                                'normal_bits':list(struct.unpack_from('<III',output,0x24))})
(OUT/'offset-ray-goldens.json').write_text(json.dumps(offsets,indent=2)+'\n')
(OUT/'native-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print(json.dumps({'hash_cases':len(hashes),'widget_cases':len(widgets),'move_cases':len(moves),'grab_cases':len(grabs),'offset_cases':len(offsets),'manifest':str(OUT/'native-manifest.json')}))
