from pathlib import Path
import json,gzip,hashlib
import pefile
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
root=Path('/Users/yuluo/Desktop/game');out=Path(__file__).resolve().parent
p=root/'games_test/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/systems/ui_clutter/ui_autoclutter.rs.enriched.json.gz';d=json.loads(gzip.decompress(p.read_bytes()))
raw=(root/'ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();pe=pefile.PE(data=raw,fast_load=True)
start=0x14099ca60;code=pe.get_data(start-pe.OPTIONAL_HEADER.ImageBase,8136);asm=list(Cs(CS_ARCH_X86,CS_MODE_64).disasm(code,start))
(out/'14099ca60-ui-convert-B.asm').write_text('\n'.join(f'{i.address:x}: {i.bytes.hex():28} {i.mnemonic} {i.op_str}'for i in asm)+'\n')
(out/'raw-ui-convert.bin').write_bytes(code)
selected=[x for x in d['file_interface_candidates'] if 'InstaCreateDeco' in x.get('function_type_spelling','')]
(out/'ui-convert-interface.json').write_text(json.dumps({'source':str(p),'source_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'binding':d['binding'],'interfaces':selected,'functions':[{k:x.get(k)for k in['name','compiled_instances','candidate_interface_evidence','reference_callers']}for x in d['functions']if x.get('name','').endswith('::ui_convert_autoclutter_to_decoclutter')]},indent=2)+'\n')
for i,ins in enumerate(asm):
 if ins.mnemonic=='inc'and'0x40]'in ins.op_str:
  print('\n'.join(f'{j.address:x}: {j.mnemonic} {j.op_str}'for j in asm[max(0,i-12):i+2]))
print({'instructions':len(asm),'code_bytes':len(code),'selected_interfaces':len(selected)})
