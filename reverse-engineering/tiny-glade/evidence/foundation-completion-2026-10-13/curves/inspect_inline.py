from pathlib import Path
import re,json,struct,hashlib
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
OUT=Path(__file__).resolve().parent
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',raw,0x3c)[0];size=struct.unpack_from('<H',raw,pe+20)[0];sections=[]
for i in range(struct.unpack_from('<H',raw,pe+6)[0]):
 o=pe+24+size+i*40;_,rva,count,offset=struct.unpack_from('<IIII',raw,o+8);sections.append((rva,count,offset))
def read(va,length):
 rva=va-0x140000000
 for start,count,offset in sections:
  if start<=rva<start+count:return raw[offset+rva-start:offset+rva-start+length]
 raise ValueError(hex(va))
IDS={0x113ee:'threshold_metric',0x6077:'remove_duplicates',0x11465:'remove_duplicate_predicate',0x17e83:'central_tangent',0x622c:'distance_coord',0x6227:'wall_coord',0x6d78:'deserialize_scalar',0x1372:'to_curve2',0x257c:'reinterpret_curve2',0x6d74:'curve_u_clamp',0x6d70:'curve_u_max',0x6d71:'curve_u_min'}
records=[];md=Cs(CS_ARCH_X86,CS_MODE_64)
for path in OUT.glob('module-*.txt'):
 text=path.read_text(encoding='utf-8-sig');ms=list(re.finditer(r'^\s*(\d+) \| (S_\w+) \[size = (\d+)\]',text,re.M));rows={}
 for i,m in enumerate(ms):
  block=text[m.end():ms[i+1].start()if i+1<len(ms)else len(text)];row={'record':int(m[1]),'kind':m[2],'text':block};rows[row['record']]=row
  if row['kind']in {'S_GPROC32','S_LPROC32'}:
   loc=re.search(r'addr = (\d+):(\d+), code size = (\d+)',block);name=re.search('`([^`]+)`',block)
   if loc:row.update({'base':0x140001000+int(loc[2]),'size':int(loc[3]),'name':name[1]if name else None})
  elif row['kind']=='S_INLINESITE':
   info=re.search(r'inlinee = 0x([0-9A-Fa-f]+).*parent = (\d+), end = (\d+)',block)
   if info:row.update({'inlinee':int(info[1],16),'parent':int(info[2])})
 for row in rows.values():
  if row.get('inlinee')not in IDS:continue
  owner=row;seen=set()
  while owner.get('parent')and owner['record']not in seen:seen.add(owner['record']);owner=rows[owner['parent']]
  if not owner.get('base'):continue
  begin=None;ranges=[]
  for line in row['text'].splitlines():
   m=re.search(r'code(?: end)? 0x([0-9A-Fa-f]+)',line)
   if not m:continue
   offset=int(m[1],16)
   if 'code end'in line:
    if begin is not None:
     a=owner['base']+begin;n=offset-begin;b=read(a,n);ranges.append({'address':hex(a),'size':n,'bytes_sha256':hashlib.sha256(b).hexdigest(),'asm':['%x: %s %s'%(x.address,x.mnemonic,x.op_str)for x in md.disasm(b,a)]})
    begin=None
   elif begin is None:begin=offset
  records.append({'method':IDS[row['inlinee']],'module':path.name,'record':row['record'],'owner':owner['name'],'owner_address':hex(owner['base']),'ranges':ranges})
(OUT/'inline-ranges.json').write_text(json.dumps({'records':records},indent=2)+'\n')
for r in records:
 print(r['method'],r['owner'],r['owner_address'])
 for x in r['ranges']:print('\n'.join(x['asm']))
for a in [0x14295a0d8,0x14295a0dc,0x14295a0e4,0x14295a0e8,0x142925870,0x1429258e0,0x1429258e4]:print(hex(a),struct.unpack('<f',read(a,4))[0])
