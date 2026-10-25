from pathlib import Path
import hashlib,json,struct
recipe=Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py')
source=recipe.read_text().split('def spline(')[0].replace("Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')", "Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')")
exec(compile(source,str(recipe),'exec'),globals())
root=Path(__file__).resolve().parent
cases=[]
for capacity,expected in [(0x8000000000000000,'null'),(0,'{"sheep":[]}'),(2,'{"sheep":[]}')]:
    reset()
    u.mem_write(INPUT,struct.pack('<3Q',capacity,INPUT+0x4000,0))
    u.reg_write(UC_X86_REG_RCX,OUTPUT)
    u.reg_write(UC_X86_REG_RDX,INPUT)
    run(0x140a51d40)
    cap,ptr,n=struct.unpack('<3Q',u.mem_read(OUTPUT,24))
    actual=bytes(u.mem_read(ptr,n)).decode('utf8')
    assert actual==expected,(capacity,actual,expected)
    cases.append({'input_capacity':hex(capacity),'row_count':0,'json':actual})
(root/'sheep-option-native-fixtures.json').write_text(json.dumps({'binary_sha256':hashlib.sha256(raw).hexdigest(),'entry':'0x140a51d40','cases':cases,'boundary':'Resolved Option/Vec header; original serde snapshot/Option/string code executes; allocator/CRT bridged. Does not prove live save caller capture.'},indent=2))
print({'original_option_cases':len(cases),'passed':len(cases)})
