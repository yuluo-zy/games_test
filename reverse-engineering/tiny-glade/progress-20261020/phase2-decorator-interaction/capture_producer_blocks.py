"""Execute the original two event-construction blocks, not a reimplementation.

Stops immediately before EventWriter::write. Poison makes unwritten enum
padding visible. UI +4c is the existing semantic intersection position.
"""
from pathlib import Path
import hashlib, json, struct
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_MEM_WRITE
from unicorn.x86_const import *

p = Path(__file__).parent
raw = Path('D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe').read_bytes()
assert hashlib.sha256(raw).hexdigest() == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = pefile.PE(data=raw)
base = pe.OPTIONAL_HEADER.ImageBase
image = pe.get_memory_mapped_image()
rows = []
for name, start, end, length in [('rotate', 0x1409aaa78, 0x1409aab69, 88), ('move', 0x1409ab003, 0x1409ab0d6, 80)]:
    # Move starts at the mov ecx,[rax+5c] that precedes its identical UI-copy.
    if name == 'move':
        start = 0x1409aafff
    uc = Uc(UC_ARCH_X86, UC_MODE_64)
    uc.mem_map(base, (len(image)+4095)&~4095)
    uc.mem_write(base, image)
    uc.mem_map(0x20000000, 0x10000)
    frame, ui, info = 0x20001000, 0x20004000, 0x20005000
    uc.mem_write(frame, bytes([0xcd])*0x400)
    uc.mem_write(ui, bytes(range(0x70)))
    uc.mem_write(ui+0x4c, struct.pack('<3f', 11.25, 7.5, -2.75))
    uc.mem_write(frame+0x158, struct.pack('<QQ', 0, 0xabcdef))
    uc.mem_write(frame+0x168, struct.pack('<Q',info+2))
    uc.mem_write(info, bytes([0x3e]))
    uc.mem_write(frame+0x188, struct.pack('<2f',1.25,4.75))
    uc.mem_write(frame+0x218,struct.pack('<Q',0x3e))
    uc.mem_write(frame+0x360,struct.pack('<I',0x87654321))
    uc.reg_write(UC_X86_REG_RBP,frame)
    uc.reg_write(UC_X86_REG_RAX,ui)
    uc.reg_write(UC_X86_REG_RDX,0x1122334455667788)
    written=set()
    def on_write(machine,access,address,size,value,unused):
        first=max(address,frame+0x290)
        last=min(address+size,frame+0x290+length)
        written.update(range(first-(frame+0x290),last-(frame+0x290)))
    uc.hook_add(UC_HOOK_MEM_WRITE,on_write)
    uc.emu_start(start,end,count=1000)
    event=bytes(uc.mem_read(frame+0x290,length))
    rows.append({'kind':name,'start':hex(start),'end':hex(end),'storage_address':{'tag':0,'payload':0xabcdef},'event_value':event.hex(),
                 'hit':event[8:72].hex(),'unwritten_byte_offsets':[i for i in range(length) if i not in written]})
(p/'producer-block-native.json').write_text(json.dumps(rows,indent=2))
print(json.dumps(rows,indent=2))
