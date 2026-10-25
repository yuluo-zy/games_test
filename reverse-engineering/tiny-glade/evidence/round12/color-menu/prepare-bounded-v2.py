import json,hashlib
from pathlib import Path
from ghidra.program.flatapi import FlatProgramAPI
from ghidra.program.disassemble import Disassembler
from ghidra.program.model.address import AddressSet
from ghidra.program.model.symbol import SourceType
from ghidra.program.model.pcode import JumpTable
from ghidra.app.decompiler import DecompInterface
from ghidra.app.util.cparser.C import CParser
from ghidra.app.cmd.function import ApplyFunctionSignatureCmd
from java.util import ArrayList
p=currentProgram
api=FlatProgramAPI(p,monitor)
fm=p.getFunctionManager()
af=p.getAddressFactory()
def addr(v): return af.getAddress(str(v).removeprefix('0x'))
root=Path('D:/game/reverse-engineering/tiny-glade/evidence/round12/color-menu')
meta=json.loads((root/'native-identity-and-tables.json').read_text())
rows=[]
plans=[('create',[('0x1425f2d31','0x143136f20',12),('0x1425f2f65','0x143136f50',12)]),('show_color_options',[('0x14229aa20','0x14303ca70',13),('0x14229aa6c','0x14303ca70',13),('0x14229ea79','0x14303caa4',13)]),('color_menu',[('0x142295e00','0x14303ba18',22)]),('hide_color_menu',[('0x1422ae3f7','0x14303cad8',13)])]
tx=p.startTransaction('color-menu-full original range and jump boundaries')
commit=False
try:
 for name,tables in plans:
  row=meta['functions'][name]
  a=addr(row['address']);size=row['size'];end=a.add(size-1)
  raw=bytes(x&255 for x in api.getBytes(a,size))
  if hashlib.sha256(raw).hexdigest()!=row['sha256']: raise ValueError('Original bytes differ: '+name)
  region=AddressSet(a,end)
  decoded=Disassembler.getDisassembler(p,monitor,None).disassemble(a,region,True)
  f=fm.getFunctionAt(a)
  if f is None:f=api.createFunction(a,name)
  if f is None:raise ValueError('No function '+name)
  body=AddressSet(decoded)
  for site,base,count in tables:
   targets=ArrayList()
   table=addr(base)
   for i in range(count):
    raw=bytes(x&255 for x in api.getBytes(table.add(i*4),4))
    target=table.add(int.from_bytes(raw,'little',signed=True));targets.add(target)
    if region.contains(target): body.add(Disassembler.getDisassembler(p,monitor,None).disassemble(target,region,True))
   JumpTable(addr(site),targets,True,0).writeOverride(f)
  preserved=[]
  for other in fm.getFunctionsOverlapping(body):
   if other.getEntryPoint()!=a:
    preserved.append({'address':str(other.getEntryPoint()),'body_bytes':other.getBody().getNumAddresses(),'name':str(other.getName())})
    body.delete(other.getBody())
  f.setBody(body)
  if name=='create':
   proto='void color_create(void *out,float center_x,float center_y,float radius,float start_angle,float icon_size,float spacing,float appearance_divisor,void *icons,void *target,void *initial,void *opening_eye,void *commands);'
  else:proto='void '+name+'('+','.join('void *p'+str(i) for i in range(15 if name in ['show_color_options','color_menu'] else 9))+');'
  definition=CParser().parse(proto)
  cmd=ApplyFunctionSignatureCmd(a,definition,SourceType.USER_DEFINED)
  if not cmd.applyTo(p,monitor):raise ValueError('Prototype apply failed '+name)
  rows.append({'name':name,'range':row,'bounded_bytes':f.getBody().getNumAddresses(),'overrides':tables,'prototype':proto,'preserved_functions':preserved})
 commit=True
finally:p.endTransaction(tx,commit)
di=DecompInterface();di.openProgram(p)
try:
 for row in rows:
  f=fm.getFunctionAt(addr(row['range']['address']))
  result=di.decompileFunction(f,120,monitor)
  row['completed']=bool(result.decompileCompleted());row['error']=str(result.getErrorMessage())
  if result.decompileCompleted():
   code=result.getDecompiledFunction().getC();(root/(row['name']+'-bounded.c')).write_text(str(code),encoding='utf-8');row['decomp_chars']=len(code)
finally:di.dispose()
(root/'bounded-overlay-revision.json').write_text(json.dumps({'worker':'color-menu-full','project':str(p.getDomainFile().getPathname()),'program_sha256':str(p.getExecutableSHA256()),'rows':rows},indent=2),encoding='utf-8')
print(json.dumps(rows))
