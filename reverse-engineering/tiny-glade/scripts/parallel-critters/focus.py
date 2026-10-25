"""Existing DB view only; no import, analysis, transaction, save, type or comment write."""
from pathlib import Path
import json,sys,time,hashlib
import pyghidra
ROOT=Path(__file__).resolve().parents[2]
GAME=ROOT.parents[2]
OUT=ROOT/'evidence/parallel-critters'/sys.argv[2]
OUT.mkdir(exist_ok=False)
pyghidra.start(install_dir=GAME/'tools/ghidra_12.1.4_PUBLIC')
from ghidra.framework.model import ProjectLocator,DomainFile
from ghidra.framework.data import DefaultProjectData
from ghidra.util.task import TaskMonitor
from ghidra.app.decompiler import DecompInterface
from java.lang import Object
locator=ProjectLocator(str(GAME/'tools/ghidra-projects'),'TinyGlade')
# Inspected Ghidra 12.1.4 bytecode: second constructor flag is isWritable;
# false skips initLock, opens local/user file systems read-only. Third false does not reset owner.
project=DefaultProjectData(locator,False,False)
consumer=Object();program=None;decomp=DecompInterface()
report={'database':'TinyGlade/tiny-glade.exe','database_view_writable':False,'domain_api':'getReadOnlyDomainObject','auto_analysis':False,'target_execution':False,'results':[]}
try:
 file=project.getFile('/tiny-glade.exe')
 if file is None:raise RuntimeError('Original bound program not found')
 program=file.getReadOnlyDomainObject(consumer,DomainFile.DEFAULT_VERSION,TaskMonitor.DUMMY)
 report['executable_sha256']=str(program.getExecutableSHA256());report['changed_before']=bool(program.isChanged())
 assert report['executable_sha256']=='cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323'
 if not decomp.openProgram(program):raise RuntimeError('Decompiler openProgram failed')
 requests=json.loads(Path(sys.argv[1]).read_text())
 fm=program.getFunctionManager();am=program.getAddressFactory();refs=program.getReferenceManager()
 if 'search' in requests:
  matches=[]
  for fn in fm.getFunctions(True):
   name=str(fn.getName(True))
   if any(t.lower() in name.lower() for t in requests['search']):matches.append({'va':'0x'+str(fn.getEntryPoint()),'name':name,'body_bytes':int(fn.getBody().getNumAddresses())})
  report['matches']=matches
 for request in requests.get('functions',[]):
  a=am.getAddress(request);fn=fm.getFunctionAt(a);row={'va':request,'exact_function':fn is not None}
  if fn is not None:
   row['database_name']=str(fn.getName(True));row['body_bytes']=int(fn.getBody().getNumAddresses())
   row['incoming_refs']=[{'from':str(rr.getFromAddress()),'type':str(rr.getReferenceType()),'caller':str(fm.getFunctionContaining(rr.getFromAddress()).getName(True)) if fm.getFunctionContaining(rr.getFromAddress()) else None} for rr in refs.getReferencesTo(a)]
   row['callees']=[{'va':'0x'+str(c.getEntryPoint()),'name':str(c.getName(True))} for c in fn.getCalledFunctions(TaskMonitor.DUMMY)]
   result=decomp.decompileFunction(fn,120,TaskMonitor.DUMMY);row['decompiled']=bool(result.decompileCompleted());row['error']=str(result.getErrorMessage())
   if row['decompiled']:(OUT/(request[2:]+'.c')).write_text(str(result.getDecompiledFunction().getC()))
   (OUT/(request[2:]+'.asm.txt')).write_text('\n'.join(str(ins.getAddress())+' '+str(ins) for ins in program.getListing().getInstructions(fn.getBody(),True))+'\n')
  report['results'].append(row)
  print(json.dumps({'va':request,'decompiled':row.get('decompiled'),'body':row.get('body_bytes')}),flush=True)
 for request in requests.get('memory',[]):
  import jpype
  addr=am.getAddress(request['va']);buf=jpype.JArray(jpype.JByte)(request['size']);program.getMemory().getBytes(addr,buf)
  (OUT/(request['va'][2:]+'.bin')).write_bytes(bytes((int(b)&255 for b in buf)))
 report['changed_after']=bool(program.isChanged());assert not report['changed_after']
 report['ok']=all(x.get('decompiled') for x in report['results'])
finally:
 decomp.dispose()
 if program is not None:program.release(consumer)
 project.close()
 (OUT/'report.json').write_text(json.dumps(report,indent=2)+'\n')
