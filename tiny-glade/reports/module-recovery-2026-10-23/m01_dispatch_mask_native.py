"""复用已验证的原入口布置，执行空StartPlace流到真实全部命中查询，捕获mask参数。"""
from pathlib import Path
import json,itertools
source=Path(__file__).with_name('m01_dispatch_start_place_native.py').read_text()
source=source[:source.index('\nrows=[]')]
a=source.index(' u.emu_start(0x1407af340');b=source.index('\n return {',a);c=source.find('\n',b+1);c=len(source)if c<0 else c
source=source[:a]+''' q(mode_data+4,mode_type)
 q(dispatch_data,state)
 for offset in [0x5c,0x60,0x69]:u.mem_write(dispatch_data+offset,bytes([active_ty]))
 captures.clear()
 u.emu_start(0x1407af340,STOP,count=30000)
 assert captures, (mode,mode_type,state,hex(r(UC_X86_REG_RIP)))
 return captures[0]
'''+source[c:]
exec(compile(source,'retained-start-place-setup','exec'))
captures=[]
def extra_hook(uc,a,size,_):
 if a==0x1408cc8a0:
  sp=r(UC_X86_REG_RSP);captures.append({'mask':uw(sp+0x30),'max_toi_bits':uw(sp+0x28)});uc.emu_stop()
u.hook_add(UC_HOOK_CODE,extra_hook)
rows=[]
for mode,mode_type,state,active_ty in itertools.product([0,4,11],[27,63],range(7),[27,63]):
 result=run(mode,False,(0,0),0,27)
 rows.append({'mode':mode,'mode_type':mode_type,'state':state,'active_ty':active_ty,**result})
Path(__file__).with_name('m01-dispatch-mask-native.json').write_text(json.dumps({'binary_sha256':sha,'entry':'0x1407af340','capture':'0x1408cc8a0 before executing raycaster','rows':rows},indent=2)+'\n')
print(json.dumps({'cases':len(rows),'distinct_masks':sorted(set(r['mask']for r in rows))}))
