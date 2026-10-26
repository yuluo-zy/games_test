from pathlib import Path
exec(Path(__file__).with_name("review_focus_height_target_native.py").read_text().split("rng=random.Random")[0])
stop_at=[0]
def bounded_stop(machine,address,size,opaque):
 if address==stop_at[0]:machine.emu_stop()
u.hook_add(UC_HOOK_CODE,bounded_stop)
rng=random.Random(0x464f435553415252)
rows=[]
for case in range(512):
 inv=identity.copy();inv[:3]=[rng.randrange(-2000,2001)/37. for _ in range(3)]
 if case<8:inv[:3]=[[1.,0.,0.],[0.,0.,1.],[0.,1.,0.],[-1.,0.,0.],[0.,0.,-1.],[0.,0.,0.],[1.,1.,1.],[-0.,0.,-1.]][case]
 u.mem_write(camera+0xc0,struct.pack('<16f',*inv));u.reg_write(UC_X86_REG_RSI,camera)
 u.reg_write(UC_X86_REG_XMM6,0x3f800000);u.reg_write(UC_X86_REG_RBP,frame);u.reg_write(UC_X86_REG_RSP,frame-0x80)
 # 含原eye_right完整依赖，旋转输入取该函数实际输出。
 stop_at[0]=0x140a62c2f
 u.emu_start(0x140a62c23,0x140a62c2f,count=10000);assert u.reg_read(UC_X86_REG_RIP)==0x140a62c2f
 right=list(struct.unpack('<III',u.mem_read(frame+0x70,12)))
 stop_at[0]=0x140a62c2f
 stop_at[0]=0x140a62c73
 u.emu_start(0x140a62c2f,0x140a62c73,count=10000);assert u.reg_read(UC_X86_REG_RIP)==0x140a62c73
 up=list(struct.unpack('<IIII',int(u.reg_read(UC_X86_REG_XMM15)).to_bytes(16,'little')))
 stop_at[0]=0x140a62d95
 u.emu_start(0x140a62d32,0x140a62d95,count=1000);assert u.reg_read(UC_X86_REG_RIP)==0x140a62d95
 down=list(struct.unpack('<IIII',int(u.reg_read(UC_X86_REG_XMM15)).to_bytes(16,'little')))
 rows.append({'right':right,'up':up,'down':down})
p=root/'tiny-glade/tests/fixtures/native/toolbar-focus-height-rotation.json';p.write_text(json.dumps(rows,separators=(',',':'))+'\n')
print(json.dumps({'cases':len(rows),'unique_inputs':len({tuple(row['right']) for row in rows}),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'ranges':[['0x140a62c23','0x140a62c73'],['0x140a62d32','0x140a62d95']],'original_full_callees':['0x140c90fd0..0x140c911c8','0x140c96df0 eye_right'],'bridges':[]}))
