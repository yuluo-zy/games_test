import hashlib,json
from pathlib import Path
import struct
root=Path('D:/game')
out=root/'reverse-engineering/tiny-glade/evidence/theme-static-2026-10-14/integration'
exe=root/'ljxsj_92385/Tiny Glade/tiny-glade.exe'
b=exe.read_bytes()
pe=struct.unpack_from('<I',b,0x3c)[0]
count=struct.unpack_from('<H',b,pe+6)[0]
optional_size=struct.unpack_from('<H',b,pe+20)[0]
image_base=struct.unpack_from('<Q',b,pe+24+24)[0]
section_table=pe+24+optional_size
def file_offset(address):
 rva=address-image_base
 for i in range(count):
  s=section_table+i*40
  virtual_size,virtual_address,raw_size,raw_offset=struct.unpack_from('<IIII',b,s+8)
  if virtual_address <= rva < virtual_address+min(virtual_size,raw_size):
   return raw_offset+rva-virtual_address
 raise ValueError(hex(address))
targets=[]
for address,size,name in [(0x1403d0f70,2002,'tiny_glade::theme_fetcher_plugin::build'),(0x1403d1750,4247,'tiny_glade::theme_fetcher_plugin::update_daily_theme')]:
 offset=file_offset(address)
 targets.append(dict(address=hex(address),size=size,names=[name],range_basis='LLVM B PDB module 170 S_GPROC32 code size',bytes_sha256=hashlib.sha256(b[offset:offset+size]).hexdigest()))
job=[dict(tool='prepare_functions',args=dict(program='tiny-glade.exe',targets=targets)),dict(tool='get_functions',args=dict(program='tiny-glade.exe',functions=','.join(t['address'] for t in targets),fields='signature,decompiled_code,callees'))]
(out/'consumer-prepare.job.json').write_text(json.dumps(job,indent=2),encoding='utf-8')
