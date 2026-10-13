"""Bind exact original PDB inline code ranges to their physical B consumer."""
from pathlib import Path
import hashlib,json,re
import pefile
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
OUT=Path(__file__).parent;raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
pe=pefile.PE(data=raw);base=pe.OPTIONAL_HEADER.ImageBase
md=Cs(CS_ARCH_X86,CS_MODE_64);result=[]
for module,record_id,label in [(540,1668,'IndexCondenser::get'),(523,8032,'IndexCondenser::get consumer'),(523,15836,'Condex::last_known_index'),(554,21768,'IndexCondenser::clear')]:
    text=(OUT/f'B-module-{module}.txt').read_text(encoding='utf8')
    matches=list(re.finditer(r'^\s*(\d+) \| (S_[A-Z0-9_]+)[^\n]*',text,re.M));records={}
    for i,m in enumerate(matches):records[int(m[1])]=(m[2],text[m.start():matches[i+1].start() if i+1<len(matches) else len(text)])
    kind,body=records[record_id];parent=int(re.search(r'parent = (\d+)',body)[1]);chain=[record_id]
    while records[parent][0] not in ['S_GPROC32','S_LPROC32']:
        chain.append(parent);parent=int(re.search(r'parent = (\d+)',records[parent][1])[1])
    chain.append(parent);proc=records[parent][1];section,offset,size=map(int,re.search(r'addr = (\d+):(\d+), code size = (\d+)',proc).groups())
    address=base+pe.sections[section-1].VirtualAddress+offset
    offsets=[int(x,16) for x in re.findall(r'code (?:end )?0x([0-9A-Fa-f]+)',body)]
    first,last=min(offsets),max(offsets)
    data=pe.get_data(address-base+first,last-first)
    (OUT/f'{label.replace("::","-").replace(" ","-")}-inline.asm.txt').write_text('\n'.join(f'{i.address:#x}: {i.mnemonic} {i.op_str}' for i in md.disasm(data,address+first))+'\n',encoding='utf8')
    result.append({'interface':label,'module':module,'inline_record':record_id,'ancestor_records':chain,'physical_procedure_record':parent,'physical_name':re.search(r'`([^`]+)`',proc)[1],'physical_address':hex(address),'physical_size':size,'inline_annotation':body.strip(),'enclosing_code_range':[hex(address+first),hex(address+last)],'enclosing_bytes_sha256':hashlib.sha256(data).hexdigest(),'range_limit':'annotations may have discontiguous intervals; enclosing range includes library/consumer instructions; count only game field/state policy'})
(OUT/'inline-bindings.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf8')
print([(r['interface'],r['physical_address'],r['enclosing_code_range']) for r in result])
