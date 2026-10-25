from pathlib import Path
exec(Path(__file__).with_name("review_focus_height_target_native.py").read_text().split("rng=random.Random")[0])
rows=[]
widget,wall=0xabcddcba,0x123456789
for hover in [0,1,2,255]:
 for matching in [False,True]:
  for phase in range(256):
   u.mem_write(data+0x5000+0x18,bytes([hover,phase]));u.mem_write(data+0x5000+0x38,struct.pack('<Q',widget if matching else widget+1));u.mem_write(frame+0x18,struct.pack('<Q',widget));u.mem_write(frame+0x160,struct.pack('<Q',wall))
   for index,stackoff in enumerate([0x298,0x2a0,0x2a8]):
    writer,events,storage,tick=[data+off+index*0x100 for off in [0x1000,0x2000,0x3000,0x4000]]
    u.mem_write(frame+stackoff,struct.pack('<Q',writer));u.mem_write(writer,struct.pack('<Q',storage));u.mem_write(writer+0x10,struct.pack('<Q',tick));u.mem_write(writer+0x1c,struct.pack('<I',22));u.mem_write(storage+0x20,struct.pack('<QQQ',4,events,0));u.mem_write(storage+0x40,struct.pack('<Q',17));u.mem_write(events,bytes(64))
   for reg,value in [(UC_X86_REG_R14,data+0x5000),(UC_X86_REG_RBP,frame),(UC_X86_REG_RSP,frame-0x80)]:u.reg_write(reg,value)
   u.emu_start(0x140a63294,0x140a63405,count=1000);assert u.reg_read(UC_X86_REG_RIP)==0x140a63405
   lengths=[struct.unpack('<Q',u.mem_read(data+0x3000+i*0x100+0x30,8))[0] for i in range(3)]
   for index,count in enumerate(lengths):
    if count:
     event=u.mem_read(data+0x2000+index*0x100,16);assert struct.unpack('<Q',event[:8])[0]==17
     if index<2:assert struct.unpack('<Q',event[8:])[0]==wall
   rows.append({'hover':hover,'matching':matching,'phase':phase,'finish':lengths[0],'adjust':lengths[1],'init':lengths[2]})
p=root/'tiny-glade/tests/fixtures/native/toolbar-focus-height-events.json';p.write_text(json.dumps(rows,separators=(',',':'))+'\n')
print(json.dumps({'cases':len(rows),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'range':['0x140a63294','0x140a63405'],'bridges':[],'preconditions':'有界容量4事件队列，grow分支自然不执行；EventId17与WallId0x123456789真实载荷均核对','strides':{'finish':16,'adjust':16,'init':8}}))
