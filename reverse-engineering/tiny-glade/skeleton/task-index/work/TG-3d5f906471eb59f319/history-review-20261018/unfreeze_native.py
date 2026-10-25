from pathlib import Path
import hashlib
import json
import struct

recipe = Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py')
source = recipe.read_text().split('def spline(')[0].replace("Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')", "Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')")
exec(compile(source, str(recipe), 'exec'), globals())
root = Path(__file__).resolve().parent
control = INPUT + 0x4000
reader = INPUT + 0x5000
cursor = INPUT + 0x6000
events = INPUT + 0x7000
rows = INPUT + 0x8000
cases = []
for capacity in [0x8000000000000000, 0, 2]:
    for old_len, new_len, consumed in [(0,0,0),(0,1,0),(1,2,1),(1,2,3),(2,1,5)]:
        reset()
        for address in [control,reader,cursor,events,rows]:
            u.mem_write(address, bytes(256))
        u.mem_write(control, struct.pack('<3Q',capacity,rows,0 if capacity != 2 else 2))
        u.mem_write(reader, struct.pack('<QQ',cursor,events))
        u.mem_write(cursor, struct.pack('<Q',consumed))
        u.mem_write(events+0x10, struct.pack('<QQ',old_len,0))
        u.mem_write(events+0x30, struct.pack('<QQQ',new_len,old_len,old_len+new_len))
        u.reg_write(UC_X86_REG_RSI, reader)
        u.reg_write(UC_X86_REG_R13, control)
        u.emu_start(0x1416f19e8,0x1416f1a70,count=100000)
        assert u.reg_read(UC_X86_REG_RIP) == 0x1416f1a70
        result = struct.unpack('<Q',u.mem_read(control,8))[0]
        after_cursor = struct.unpack('<Q',u.mem_read(cursor,8))[0]
        unread = max(old_len-consumed,0)+max(new_len-max(consumed-old_len,0),0)
        expected = 0x8000000000000000 if unread else capacity
        assert result == expected,(capacity,old_len,new_len,consumed,result,expected)
        assert after_cursor == old_len+new_len
        cases.append({'before_cap':hex(capacity),'old_events':old_len,'new_events':new_len,'consumed_before':consumed,'has_unread':bool(unread),'after_cap':hex(result),'consumed_after':after_cursor})
report={'binary_sha256':hashlib.sha256(raw).hexdigest(),'entry':'0x1416f19e8','exit':'0x1416f1a70','cases':cases,'bridge':'original inline EventReader bookkeeping and snapshot clear/drop executes; allocator deallocation is existing CRT bridge; ECS query and UI excluded'}
(root/'unfreeze-native-fixtures.json').write_text(json.dumps(report,indent=2))
print({'original_cases':len(cases),'passed':len(cases)})
