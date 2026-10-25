"""Existing persistent DB read-only view. No import/analysis/transaction/save."""
from pathlib import Path
import json,sys,subprocess,hashlib
import pyghidra
ROOT=Path(__file__).resolve().parents[2];GAME=ROOT.parents[2];OUT=ROOT/'evidence/parallel-camera/B-decompile';OUT.mkdir(exist_ok=True)
pyghidra.start(install_dir=GAME/'tools/ghidra_12.1.4_PUBLIC')
from ghidra.framework.model import ProjectLocator,DomainFile
from ghidra.framework.data import DefaultProjectData
from ghidra.util.task import TaskMonitor
from ghidra.app.decompiler import DecompInterface
from java.lang import Object
locator=ProjectLocator(str(GAME/'tools/ghidra-projects'),'TinyGlade');project=DefaultProjectData(locator,False,False)
consumer=Object();program=None;decomp=DecompInterface()
report={'database':str(GAME/'tools/ghidra-projects/TinyGlade.gpr'),'program':'tiny-glade.exe','writable':False,'domain_api':'getReadOnlyDomainObject','auto_analysis':False,'target_execution':False,'results':[]}
try:
 file=project.getFile('/tiny-glade.exe')
 if file is None:raise RuntimeError('Bound program missing')
 program=file.getReadOnlyDomainObject(consumer,DomainFile.DEFAULT_VERSION,TaskMonitor.DUMMY)
 report['executable_sha256']=str(program.getExecutableSHA256());report['changed_before']=bool(program.isChanged());assert not report['changed_before']
 assert report['executable_sha256']=='cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323'
 assert decomp.openProgram(program)
 inv=json.loads((ROOT/'evidence/parallel-camera/B-inventory.json').read_text());selected=[];seen=set()
 for x in inv['game_candidate_public_records']+inv['glue_records']:
  business= x['name'].startswith('country_core::systems::') and 'Serialize' not in x['name'] and 'Sequence' not in x['name'] or x['name'].startswith('_$LT$country_core..systems..camera_rig..') and '..CameraBody$GT$' in x['name']
  if not x['function'] or not business or x['va'] in seen:continue
  if sys.argv[1:] and not any(t in x['name'] or t==x['va'] for t in sys.argv[1:]):continue
  selected.append(x);seen.add(x['va'])
 fm=program.getFunctionManager();am=program.getAddressFactory();refs=program.getReferenceManager()
 for x in selected:
  a=am.getAddress(x['va']);fn=fm.getFunctionAt(a);row={'va':x['va'],'pdb_name':x['name'],'exact_function':fn is not None}
  if fn is not None:
   row['database_name']=str(fn.getName(True));row['body_bytes']=int(fn.getBody().getNumAddresses())
   row['incoming_refs']=[{'from':str(rr.getFromAddress()),'type':str(rr.getReferenceType()),'caller_va':str(fm.getFunctionContaining(rr.getFromAddress()).getEntryPoint()) if fm.getFunctionContaining(rr.getFromAddress()) else None,'caller':str(fm.getFunctionContaining(rr.getFromAddress()).getName(True)) if fm.getFunctionContaining(rr.getFromAddress()) else None} for rr in refs.getReferencesTo(a)]
   row['callees']=[{'va':str(c.getEntryPoint()),'name':str(c.getName(True))} for c in fn.getCalledFunctions(TaskMonitor.DUMMY)]
   result=decomp.decompileFunction(fn,180,TaskMonitor.DUMMY);row['decompiled']=bool(result.decompileCompleted());row['error']=str(result.getErrorMessage())
   if row['decompiled']:(OUT/(x['va'][2:]+'.c')).write_text(str(result.getDecompiledFunction().getC()))
   asm=[str(ins.getAddress())+' '+str(ins) for ins in program.getListing().getInstructions(fn.getBody(),True)];(OUT/(x['va'][2:]+'.asm.txt')).write_text('\n'.join(asm)+'\n')
  report['results'].append(row);(OUT/'report.json').write_text(json.dumps(report,indent=2)+'\n')
  print(json.dumps({'va':row['va'],'name':row['pdb_name'],'decompiled':row.get('decompiled',False),'body':row.get('body_bytes'),'error':row.get('error')}),flush=True)
 report['changed_after']=bool(program.isChanged());assert not report['changed_after'];report['ok']=all(x.get('decompiled') for x in report['results'])
finally:
 if program is not None:program.release(consumer)
 project.close();(OUT/'report.json').write_text(json.dumps(report,indent=2)+'\n')
