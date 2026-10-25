"""Original B terrain UI ports; names bound from full SystemParam, not ICF aliases."""
from pathlib import Path
saved_out=Path(__file__).resolve().parent
previous=saved_out.parent/'real-edit-producer/native_ui_edit_oracle.py'
exec(compile(previous.read_text().split('jumps=')[0],str(previous),'exec'),globals());OUT=saved_out
CAPTURE=INPUT+0x11000;CAPTUREBUF=INPUT+0x12000;CACHE=INPUT+0x13000;ARCHIVE=INPUT+0x14000;CURSOR=INPUT+0x15000;PENDING=INPUT+0x16000;HIGHLIGHT=INPUT+0x17000;HIGHLIGHTBUF=INPUT+0x18000;NOTIFY=INPUT+0x19000;NOTIFYBUF=INPUT+0x1a000;NOTIFYWRITER=INPUT+0x1b000
frame=STACK+0x10008

def queue(q,b):u.mem_write(q,bytes(128));u.mem_write(q+0x20,struct.pack('<3Q',32,b,0));u.mem_write(q+0x40,struct.pack('<Q',77))
def count(q):return struct.unpack('<Q',u.mem_read(q+0x30,8))[0]
def seed():
 case(0,0,2.5)
 for p in[CAPTURE,CAPTUREBUF,CACHE,ARCHIVE,CURSOR,PENDING,HIGHLIGHT,HIGHLIGHTBUF,NOTIFY,NOTIFYBUF,NOTIFYWRITER]:u.mem_write(p,bytes(512))
 for q,b in[(CAPTURE,CAPTUREBUF),(FOCUS,FOCUSBUF),(HIGHLIGHT,HIGHLIGHTBUF),(NOTIFY,NOTIFYBUF)]:queue(q,b)
 u.mem_write(CACHE,pack([1.5,.2,1,2,3]));u.mem_write(CURSOR,struct.pack('<I',1)+pack([3,4,5]));u.mem_write(PENDING,struct.pack('<Q',0x8000000000000002));u.mem_write(BUCKET+32,pack([.4]));
 for offset,value in[(0x50,CAPTURE),(0xd8,TICK),(0xe0,TICK),(0xe8,CACHE),(0xc0,CURSOR),(0x1b8,CURSOR+4),(0x68,CACHE+8),(0x1e8,ARCHIVE),(0x1f0,TICK),(0x200,PENDING),(0x1f8,TICK),(0xb0,HIGHLIGHT),(0x1d0,TICK)]:u.mem_write(frame+offset,struct.pack('<Q',value))
 u.reg_write(UC_X86_REG_R13,CONTROL);u.reg_write(UC_X86_REG_RDI,1)

def begin():
 seed();u.emu_start(0x140aef9c5,STOP,count=1000)
 return {'capture_count':count(CAPTURE),'capture_byte':u.mem_read(CAPTUREBUF+8,1)[0],'cache_bits':list(struct.unpack('<5I',u.mem_read(CACHE,20))),'archivist_allow_any':u.mem_read(ARCHIVE+0x1e0,1)[0]}
def release():
 seed();u.emu_start(0x140aefbc0,STOP,count=1000)
 return {'capture_count':count(CAPTURE),'capture_byte':u.mem_read(CAPTUREBUF+8,1)[0],'steepness_count':count(FOCUS),'steepness_byte':u.mem_read(FOCUSBUF+8,1)[0],'pending_bytes':bytes(u.mem_read(PENDING,28)).hex()}
def hovering():
 seed();u.mem_write(frame+0x378,struct.pack('<Q',NOTIFYWRITER));u.mem_write(NOTIFYWRITER,struct.pack('<Q',NOTIFY));u.mem_write(NOTIFYWRITER+0x10,struct.pack('<Q',TICK));u.emu_start(0x140aef3b4,0x140aef405,count=1000)
 return {'terrain_hint_count':count(NOTIFY),'terrain_hint_byte':u.mem_read(NOTIFYBUF+8,1)[0]}
def highlight(camera_moved):
 seed();ui=INPUT+0x1c000;u.mem_write(ui,bytes(128));u.mem_write(ui+0x18,bytes([1,1]));u.mem_write(frame+0x2b,bytes([camera_moved]));u.mem_write(frame+0x2c,struct.pack('<i',17));u.mem_write(frame+0x1c8,struct.pack('<Q',17));u.reg_write(UC_X86_REG_R15,ui);u.emu_start(0x140aef8d5,0x140aef9a0,count=1000)
 return {'camera_moved':camera_moved,'highlight_count':count(HIGHLIGHT),'highlight_id':struct.unpack('<i',u.mem_read(HIGHLIGHTBUF+8,4))[0]if count(HIGHLIGHT)else None}
def continuing(dx):
 out=case(dx,0,2.5);out.update({'arrow_count':count(UNIT),'steepness_count':count(FOCUS),'steepness_byte':u.mem_read(FOCUSBUF+8,1)[0]if count(FOCUS)else None});return out
result={'binary_sha256':hashlib.sha256(raw).hexdigest(),'method':'Original branch bytes with resolved SystemParam pointers and preallocated native event queues; types bound separately from exact PDB tuple and stack origins. No ECS/GPU/allocator growth.','begin':begin(),'release':release(),'hover':hovering(),'highlight':[highlight(0),highlight(1)],'continued':[continuing(10),continuing(200)]}
(OUT/'native-ui-edit-ports.json').write_text(json.dumps(result,indent=2)+'\n');print(result)
