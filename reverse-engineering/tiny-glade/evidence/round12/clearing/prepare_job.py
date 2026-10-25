from pathlib import Path
import json
out=Path(__file__).resolve().parent
code=r'''
import json,hashlib
from pathlib import Path
from ghidra.program.flatapi import FlatProgramAPI
from ghidra.program.disassemble import Disassembler
from ghidra.program.model.address import AddressSet
from ghidra.program.model.symbol import SourceType
from ghidra.app.decompiler import DecompInterface
from ghidra.app.util.cparser.C import CParser
from ghidra.app.cmd.function import ApplyFunctionSignatureCmd
root=Path('D:/game/reverse-engineering/tiny-glade/evidence/round12/clearing')
p=currentProgram;api=FlatProgramAPI(p,monitor);fm=p.getFunctionManager();af=p.getAddressFactory()
def addr(v):return af.getAddress(hex(v)[2:])
plans=[('init_clearing',0x140a78aa0,6913,11),('startup_olden',0x1415539d0,3588,8),('startup_flowery',0x141553120,2224,7),('particle_transform',0x140a123a0,521,3),('forest_placement_extend',0x1412fa010,500,3)]
rows=[]
for name,va,size,count in plans:
 a=addr(va);region=AddressSet(a,a.add(size-1))
 raw=bytes(x&255 for x in api.getBytes(a,size))
 body=AddressSet(region)
 Disassembler.getDisassembler(p,monitor,None).disassemble(a,region,True)
 f=fm.getFunctionAt(a)
 if f is None:f=api.createFunction(a,name)
 if f is None:raise ValueError(name)
 for other in fm.getFunctionsOverlapping(body):
  if other.getEntryPoint()!=a:body.delete(other.getBody())
 f.setBody(body)
 proto='void '+name+'('+','.join('void *p'+str(i) for i in range(count))+');'
 cmd=ApplyFunctionSignatureCmd(a,CParser().parse(proto),SourceType.USER_DEFINED)
 if not cmd.applyTo(p,monitor):raise ValueError('signature '+name)
 rows.append({'name':name,'va':hex(va),'size':size,'sha256':hashlib.sha256(raw).hexdigest(),'prototype':proto,'bounded_bytes':body.getNumAddresses()})
di=DecompInterface();di.openProgram(p)
try:
 for row in rows:
  result=di.decompileFunction(fm.getFunctionAt(addr(int(row['va'],16))),120,monitor)
  row['completed']=bool(result.decompileCompleted());row['error']=str(result.getErrorMessage())
  if not result.decompileCompleted():raise ValueError(row)
  (root/(row['name']+'.c')).write_text(str(result.getDecompiledFunction().getC()).replace('\r\n','\n'),encoding='utf8')
finally:di.dispose()
(root/'overlay-revision.json').write_text(json.dumps({'worker':'color-menu-full','project':str(p.getDomainFile().getPathname()),'program_sha256':str(p.getExecutableSHA256()),'functions':rows},indent=2),encoding='utf8')
print(json.dumps(rows))
'''
(out/'prepare-job.json').write_text(json.dumps([{'tool':'run-script','args':{'programPath':'/tiny-glade.exe','code':code,'timeoutSeconds':600},'timeout_seconds':620}]),encoding='utf8')
