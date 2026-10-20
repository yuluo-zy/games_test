from pathlib import Path
import os,json
Path('D:/game/tmp/phase2-stairs-decompile.pid').write_text(str(os.getpid()))
os.environ['JAVA_HOME_OVERRIDE']='D:/tools/rea/jdk/jdk-21.0.12.1+1'
from pyghidra.launcher import HeadlessPyGhidraLauncher
launcher=HeadlessPyGhidraLauncher(install_dir=Path('D:/tools/rea/ghidra/ghidra_12.1.4_PUBLIC'))
launcher.java_home=Path(os.environ['JAVA_HOME_OVERRIDE'])
launcher.add_vmargs('-Duser.home=D:/game/tmp/phase2-ghidra-home','-Dapplication.settingsdir=D:/game/tmp/phase2-ghidra-settings','-Dapplication.cachedir=D:/game/tmp/phase2-ghidra-cache','-Xmx3G')
launcher.start()
import pyghidra
project=pyghidra.open_project('D:/game/tools/reverse-runtime/B-seed/project','TinyGlade')

from java.lang import Object
from ghidra.util.task import TaskMonitor
from ghidra.app.decompiler import DecompInterface
import jpype,hashlib
here=Path('D:/game/reverse-engineering/tiny-glade/progress-20261020/phase2-decorator-visual/stairs')
here.mkdir(exist_ok=True)

proc={}
for line in Path('D:/game/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').open(encoding='utf8'):
 a,size,n,obj,name=line.rstrip('\n').split('\t',4);proc[int(a,16)]=(int(size),name)
program=None;consumer=Object()
try:
 for df in project.getProjectData().getRootFolder().getFiles():
  test=df.getReadOnlyDomainObject(consumer,-1,TaskMonitor.DUMMY)
  address=test.getAddressFactory().getDefaultAddressSpace().getAddress(0x140276e80)
  n=proc[0x140276e80][0];data=jpype.JArray(jpype.JByte)(n)
  try:
   test.getMemory().getBytes(address,data)
   actual=bytes((int(x)&255 for x in data));expected=(here/'140276e80.bin').read_bytes()
   matches=actual==expected
  except Exception:matches=False
  print('PROGRAM',str(df.getPathname()),'request bytes equal original:',matches,flush=True)
  if matches:program=test;break
  test.release(consumer)
 if program is None:raise RuntimeError('No seed program has exact original request bytes')
 found=[]
 for ty in program.getDataTypeManager().getAllDataTypes():
  if any(k in str(ty.getName()) for k in ['PlayermadeStairsAssembly','AdjustedStairEdge','SampledStairsPoints','StairSubSegment']):
   found.append({'name':str(ty.getPathName()),'length':ty.getLength(),'fields':[(c.getOffset(),str(c.getFieldName()),str(c.getDataType().getPathName()),c.getLength()) for c in ty.getDefinedComponents()] if hasattr(ty,'getDefinedComponents') else []})
 (here/'original-types.json').write_text(json.dumps(found,indent=2),encoding='utf8')
 print('TYPES',len(found),flush=True)
 interface=DecompInterface();interface.openProgram(program);manifest=[]
 for a in [0x140276e80,0x14027e470,0x1407cf500,0x140807090,0x14080c640,0x1407d8fe0]:
  n,name=proc[a];address=program.getAddressFactory().getDefaultAddressSpace().getAddress(a)
  data=jpype.JArray(jpype.JByte)(n);program.getMemory().getBytes(address,data)
  actual=bytes((int(x)&255 for x in data));assert actual==(here/f'{a:x}.bin').read_bytes(),'bytes differ'
  fn=program.getFunctionManager().getFunctionAt(address)
  if fn is None:print('NO FUNCTION',hex(a),flush=True);continue
  result=interface.decompileFunction(fn,45,TaskMonitor.DUMMY)
  record={'address':hex(a),'bytes':n,'sha256':hashlib.sha256(actual).hexdigest(),'pdb_symbol':name,'decompile_completed':bool(result.decompileCompleted()),'error':str(result.getErrorMessage())}
  if result.decompileCompleted():
   c=str(result.getDecompiledFunction().getC());(here/f'{a:x}.c').write_text(c,encoding='utf8');record['c_sha256']=hashlib.sha256(c.encode()).hexdigest();record['c_bytes']=len(c)
  manifest.append(record);(here/'decompile-manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf8')
  print(record,flush=True)
 interface.dispose()
finally:
 if program is not None:program.release(consumer)
 project.close()
 os._exit(0)

