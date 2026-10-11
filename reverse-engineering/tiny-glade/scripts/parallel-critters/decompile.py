"""Existing DB view only; no import, analysis, transaction, save, type or comment write."""
from pathlib import Path
import json,sys,time,hashlib
import pyghidra
ROOT=Path(__file__).resolve().parents[2]
GAME=ROOT.parents[2]
OUT=ROOT/'evidence/parallel-critters/B-decompile'
OUT.mkdir(exist_ok=True)
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
 inventory=json.loads((ROOT/'evidence/parallel-critters/B-inventory.json').read_text())['game_owned_public_records']
 selected=[x for x in inventory if x['function'] and (not sys.argv[1:] or any(t in x['name'] for t in sys.argv[1:]))]
 fm=program.getFunctionManager();am=program.getAddressFactory();refs=program.getReferenceManager()
 for x in selected:
  a=am.getAddress(x['va']);fn=fm.getFunctionAt(a);row={'va':x['va'],'pdb_name':x['name'],'exact_function':fn is not None}
  if fn is not None:
   row['database_name']=str(fn.getName(True));row['body_bytes']=int(fn.getBody().getNumAddresses())
   row['incoming_refs']=[{'from':str(rr.getFromAddress()),'type':str(rr.getReferenceType()),'caller':str(fm.getFunctionContaining(rr.getFromAddress()).getName(True)) if fm.getFunctionContaining(rr.getFromAddress()) else None} for rr in refs.getReferencesTo(a)]
   row['callees']=[{'va':str(c.getEntryPoint()),'name':str(c.getName(True))} for c in fn.getCalledFunctions(TaskMonitor.DUMMY)]
   result=decomp.decompileFunction(fn,120,TaskMonitor.DUMMY)
   row['decompiled']=bool(result.decompileCompleted());row['error']=str(result.getErrorMessage())
   if row['decompiled']:(OUT/(x['va'][2:]+'.c')).write_text(str(result.getDecompiledFunction().getC()))
   listing=program.getListing();asm=[]
   for ins in listing.getInstructions(fn.getBody(),True):asm.append(str(ins.getAddress())+' '+str(ins))
   (OUT/(x['va'][2:]+'.asm.txt')).write_text('\n'.join(asm)+'\n')
  report['results'].append(row);(OUT/'report.json').write_text(json.dumps(report,indent=2)+'\n')
  print(json.dumps({'va':row['va'],'name':row['pdb_name'],'decompiled':row.get('decompiled',False),'body':row.get('body_bytes'),'error':row.get('error')}),flush=True)
 report['changed_after']=bool(program.isChanged());assert not report['changed_after']
 report['ok']=all(x.get('decompiled') for x in report['results'])
finally:
 decomp.dispose()
 if program is not None:program.release(consumer)
 project.close()
 (OUT/'report.json').write_text(json.dumps(report,indent=2)+'\n')
