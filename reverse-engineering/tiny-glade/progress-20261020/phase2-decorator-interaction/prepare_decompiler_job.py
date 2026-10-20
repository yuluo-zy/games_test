from pathlib import Path
import json,hashlib,struct
import pefile
root=Path('D:/game')
here=Path(__file__).parent
p=pefile.PE(str(root/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe'),fast_load=True)
base=p.OPTIONAL_HEADER.ImageBase
ranges=[(0x1407a8580,3596),(0x1407af340,9340)]
tables=[(0x1407a8764,0x142a82be8,4),(0x1407af699,0x142a83790,6),
        (0x1407afd6e,0x142a837a8,6),(0x1407b0143,0x142a837c0,17),
        (0x1407b0160,0x142a837c0,17),(0x1407b09b3,0x142a83804,4)]
spec=[]
for va,size in ranges:
    code=p.get_data(va-base,size)
    (here/(hex(va)+'.bin')).write_bytes(code)
    spec.append({'address':hex(va),'size':size,'sha256':hashlib.sha256(code).hexdigest()})
switches=[]
for site,table,count in tables:
    values=struct.unpack('<'+'i'*count,p.get_data(table-base,count*4))
    switches.append({'site':hex(site),'table':hex(table),'destinations':[hex(table+x) for x in values]})
code='''import json,hashlib
from ghidra.program.flatapi import FlatProgramAPI
from ghidra.program.disassemble import Disassembler
from ghidra.program.model.address import AddressSet
from ghidra.program.model.symbol import SourceType,RefType
from ghidra.program.model.pcode import JumpTable
from ghidra.app.decompiler import DecompInterface
from java.util import ArrayList
p=currentProgram
api=FlatProgramAPI(p,monitor)
fm=p.getFunctionManager()
def a(s):return p.getAddressFactory().getAddress(s.removeprefix('0x'))
spec=SPEC
switches=SWITCHES
tx=p.startTransaction('Original PDB bounded function and explicit switch destinations')
commit=False
try:
    for row in spec:
        start=a(row['address']);body=AddressSet(start,start.add(row['size']-1))
        raw=bytes(x&255 for x in api.getBytes(start,row['size']))
        assert hashlib.sha256(raw).hexdigest()==row['sha256']
        Disassembler.getDisassembler(p,monitor,None).disassemble(start,body,True)
        for other in list(fm.getFunctionsOverlapping(body)):
            if other.getEntryPoint()!=start:fm.removeFunction(other.getEntryPoint())
        f=fm.getFunctionAt(start)
        if f is None:f=api.createFunction(start,None)
        f.setBody(body)
    for row in switches:
        site=a(row['site']);f=fm.getFunctionContaining(site)
        destinations=ArrayList()
        ins=p.getListing().getInstructionAt(site)
        if ins is None:
            Disassembler.getDisassembler(p,monitor,None).disassemble(site,AddressSet(site,site.add(1)),False)
            ins=p.getListing().getInstructionAt(site)
        for destination in row['destinations']:
            target=a(destination);destinations.add(target)
            Disassembler.getDisassembler(p,monitor,None).disassemble(target,f.getBody(),True)
            ins.addOperandReference(0,target,RefType.COMPUTED_JUMP,SourceType.USER_DEFINED)
        JumpTable(site,destinations,True,0).writeOverride(f)
    commit=True
finally:p.endTransaction(tx,commit)
di=DecompInterface();di.openProgram(p)
try:
    for row in spec:
        f=fm.getFunctionAt(a(row['address']))
        result=di.decompileFunction(f,90,monitor)
        print(json.dumps({'address':row['address'],'bytes':row['size'],'completed':result.decompileCompleted(),'error':str(result.getErrorMessage()),'c':result.getDecompiledFunction().getC() if result.decompileCompleted() else None}))
finally:di.dispose()
'''.replace('SPEC',repr(spec)).replace('SWITCHES',repr(switches))
(here/'decompile-job.json').write_text(json.dumps([{'tool':'run-script','args':{'programPath':'/tiny-glade.exe','timeoutSeconds':240,'code':code},'timeout_seconds':300}],indent=2))
(here/'original-ranges.json').write_text(json.dumps({'ranges':spec,'switches':switches,'basis':'original PDB procs.tsv full code_size includes cold-tail; target B RNE'},indent=2))
print(json.dumps({'ranges':spec,'switches':switches}))
