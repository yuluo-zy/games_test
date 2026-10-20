"""Independent original-PE static review; no reconstruction host as evidence."""
from pathlib import Path
import hashlib,json,struct
import pefile
from capstone import Cs,CS_ARCH_X86,CS_MODE_64

here=Path(__file__).parent
path=Path('D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe')
raw=path.read_bytes();digest=hashlib.sha256(raw).hexdigest()
assert digest=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw)
base=pe.OPTIONAL_HEADER.ImageBase
read=lambda va,n: pe.get_data(va-base,n)
cs=Cs(CS_ARCH_X86,CS_MODE_64)
ranges=[('parent-set14',0x14059fc77,0x15),('parent-tuple-transfer',0x14059fed8,0xb0),
 ('upload-converter',0x1405c0400,0x21c),('add-systems-tuple-prefix',0x140327490,0x130),
 ('update-schedule-selector',0x140172580,0x6d),('update-fmt',0x140526ef0,0x15),
 ('game-set-fmt',0x140037760,0x27)]
for name,va in [('run-unsafe',0x1405bef90),('run-without-apply',0x14045dc60),('upload-leaf',0x140957bf0)]:
    match=next(e.struct for e in pe.DIRECTORY_ENTRY_EXCEPTION if e.struct.BeginAddress+base==va)
    ranges.append((name,va,match.EndAddress-match.BeginAddress))
manifest={'binary':str(path),'binary_sha256':digest,'evidence_kind':'original static PE bytes/disassembly, not runtime','ranges':[]}
for name,va,n in ranges:
    code=read(va,n)
    (here/('raster-review-'+name+'.asm')).write_text('\n'.join(f'{x.address:x}: {x.mnemonic} {x.op_str}' for x in cs.disasm(code,va))+'\n',encoding='utf-8')
    manifest['ranges'].append({'name':name,'va':hex(va),'size':n,'sha256':hashlib.sha256(code).hexdigest()})

def table(va,n):
    code=read(va,n*8)
    return {'va':hex(va),'size':n*8,'sha256':hashlib.sha256(code).hexdigest(),
      'words':[hex(x) for x in struct.unpack('<'+'Q'*n,code)]}
manifest['upload_system_vtable']=table(0x142a05448,18)
manifest['game_system_set_vtable']=table(0x14292af98,9)
manifest['update_schedule_vtable']=table(0x1429f1e78,7)
assert manifest['upload_system_vtable']['words'][10]=='0x1405bef90'
assert manifest['upload_system_vtable']['words'][12]=='0x14045dc60'
assert manifest['game_system_set_vtable']['words'][3]=='0x140037760'
assert manifest['update_schedule_vtable']['words'][3]=='0x140526ef0'
labels={}
for index in [14,36,44]:
    n=struct.unpack('<Q',read(0x14292b070+index*8,8))[0]
    offset=struct.unpack('<i',read(0x14292b1d8+index*4,4))[0]
    target=0x14292b1d8+offset
    text=read(target,n).decode('utf-8')
    labels[index]={'length_entry':hex(0x14292b070+index*8),'offset_entry':hex(0x14292b1d8+index*4),
      'text_va':hex(target),'text':text}
assert labels[14]['text']=='HistoryStart'
manifest['game_set_labels']=labels
manifest['update_label']={'va':'0x1429f1cb0','length':6,'bytes_hex':read(0x1429f1cb0,6).hex(),
 'text':read(0x1429f1cb0,6).decode('utf-8')}
assert manifest['update_label']['text']=='Update'
(here/'raster-registration-independent-manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf-8')
print(json.dumps({'original_binary_sha256':digest,'set14':labels[14]['text'],'schedule':manifest['update_label']['text'],'original_ranges':len(ranges)}))
