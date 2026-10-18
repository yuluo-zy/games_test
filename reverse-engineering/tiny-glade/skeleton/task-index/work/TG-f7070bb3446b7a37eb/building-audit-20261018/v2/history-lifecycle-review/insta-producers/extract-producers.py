from pathlib import Path
import gzip,json,hashlib
import pefile
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
root=Path('/Users/yuluo/Desktop/game');out=root/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-f7070bb3446b7a37eb/building-audit-20261018/v2/history-lifecycle-review/insta-producers'
alias=json.loads(gzip.decompress((root/'games_test/reverse-engineering/tiny-glade/skeleton/_pdb/B/enrichment/address-aliases.json.gz').read_bytes()))
pe=pefile.PE(str(root/'ljxsj_92385/Tiny Glade/tiny-glade.rne'),fast_load=True)
for rel,name in [('crates/country-core/src/systems/ui_clutter/rmb_autoclutter.rs','handle_rmb_autoclutter'),('crates/systems/decorator/src/ui_systems/ui_focus_stairs.rs','ui_focus_stairs'),('crates/systems/decorator/src/ui_systems/ui_place_decorator.rs','ui_place_decorator')]:
 p=root/'games_test/reverse-engineering/tiny-glade/skeleton/B'/rel
 rec=[json.loads(l)for l in gzip.decompress(p.with_name(p.name+'.pdb.jsonl.gz').read_bytes()).decode().splitlines()]
 f=next(d for d in rec if d.get('class')=='procedure'and d.get('name','').endswith('::'+name))
 start=f['va'];raw=pe.get_data(start-pe.OPTIONAL_HEADER.ImageBase,f['code_size']);ins=list(Cs(CS_ARCH_X86,CS_MODE_64).disasm(raw,start))
 (out/(name+'.bin')).write_bytes(raw)
 (out/(name+'.asm')).write_text('\n'.join(f'{i.address:x}: {i.bytes.hex():28} {i.mnemonic} {i.op_str}'for i in ins)+'\n')
 calls=[]
 for i in ins:
  if i.mnemonic=='call'and i.op_str.startswith('0x'):
   va=int(i.op_str,16);a=alias.get(str(va-0x140000000),[]);calls.append({'at':hex(i.address),'target':hex(va),'aliases':a})
 (out/(name+'-calls.json')).write_text(json.dumps(calls,indent=2)+'\n')
 print(name,hex(start),f['code_size'],len(ins))
 for c in calls:
  a=str(c['aliases'])
  if 'InstaCreate' in a or name=='handle_rmb_autoclutter': print(c['at'],c['target'],a[:1200])
