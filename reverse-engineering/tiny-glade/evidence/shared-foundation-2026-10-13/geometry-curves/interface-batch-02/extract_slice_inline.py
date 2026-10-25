from pathlib import Path
import json,re,struct,hashlib
OUT=Path(__file__).resolve().parent
IDS={0x114A2:'is_empty',0x114A5:'points_iter',0x5F75:'points_len',0x114A3:'loop_points_iter',0x6228:'from_curve_range',0x5F73:'from_point_range',0x5F6C:'from_coord_range',0x5F90:'remap',0x5F8F:'remap_2d',0x5F91:'extend_right',0x6037:'extend_right_until',0x114A0:'extend_left_until',0x5F76:'start',0x5F77:'end',0x5F70:'iter_coords',0x5F6F:'measure_length'}
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',raw,0x3c)[0];size=struct.unpack_from('<H',raw,pe+20)[0];sections=[]
for i in range(struct.unpack_from('<H',raw,pe+6)[0]):
 o=pe+24+size+i*40;virtual,rva,count,offset=struct.unpack_from('<IIII',raw,o+8);sections.append((rva,count,offset))
def read(va,length):
 rva=va-0x140000000
 for start,count,offset in sections:
  if start<=rva<start+count:return raw[offset+rva-start:offset+rva-start+length]
 raise ValueError(hex(va))
records=[]
for path in OUT.glob('module-*.txt'):
 text=path.read_text(encoding='utf-8');matches=list(re.finditer(r'^\s*(\d+) \| (S_\w+) \[size = (\d+)\]',text,re.M));rows={}
 for i,m in enumerate(matches):
  block=text[m.end():matches[i+1].start()if i+1<len(matches)else len(text)];row={'record':int(m[1]),'kind':m[2],'text':block,'line':text.count('\n',0,m.start())+1};rows[row['record']]=row
  if row['kind']in {'S_GPROC32','S_LPROC32'}:
   loc=re.search(r'addr = (\d+):(\d+), code size = (\d+)',block)
   name=re.search('`([^`]+)`',block)
   if loc:row.update({'section':int(loc[1]),'base':0x140001000+int(loc[2]),'size':int(loc[3]),'name':name[1]if name else None})
  elif row['kind']=='S_INLINESITE':
   info=re.search(r'inlinee = 0x([0-9A-Fa-f]+).*parent = (\d+), end = (\d+)',block)
   if info:row.update({'inlinee':int(info[1],16),'parent':int(info[2]),'end_record':int(info[3])})
 for row in rows.values():
  if row.get('inlinee')not in IDS:continue
  root=row;seen=set()
  while root.get('parent')and root['record']not in seen:
   seen.add(root['record']);root=rows[root['parent']]
  if not root.get('base'):continue
  ranges=[];begin=None
  for line in row['text'].splitlines():
   m=re.search(r'code(?: end)? 0x([0-9A-Fa-f]+)',line)
   if not m:continue
   offset=int(m[1],16)
   if 'code end'in line:
    if begin is not None:
     a=root['base']+begin;n=offset-begin;b=read(a,n);ranges.append({'address':hex(a),'size':n,'bytes_sha256':hashlib.sha256(b).hexdigest(),'hex':b.hex(),'range_basis':'LLVM PDB inline annotation range within original compiled procedure'})
    begin=None
   elif begin is None:begin=offset
  if not ranges:continue
  records.append({'module':int(path.stem.split('-')[1]),'record':row['record'],'inlinee':hex(row['inlinee']),'method':IDS[row['inlinee']],'parent':row['parent'],'root_record':root['record'],'root_name':root.get('name'),'root_address':hex(root['base']),'root_size':root['size'],'ranges':ranges,'evidence':path.name,'evidence_line':row['line']})
(OUT/'slice-inline-ranges.json').write_text(json.dumps({'records':records,'limits':['Inline ranges are not standalone Rust ABI/function boundaries','Registers/results must be determined from actual instructions/caller context','Source definitions restricted to original CurveUCircular IPI IDs']},indent=2)+'\n',encoding='utf-8')
for method in IDS.values():
 rr=[r for r in records if r['method']==method];print(method,len(rr),[(r['root_name'],[(x['address'],x['size'])for x in r['ranges']])for r in sorted(rr,key=lambda r:sum(x['size']for x in r['ranges']))[:2]])
