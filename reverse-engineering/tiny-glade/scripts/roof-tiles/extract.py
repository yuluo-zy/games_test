from pathlib import Path
import re,json,subprocess,os
R=Path('D:/game/reverse-engineering/tiny-glade'); O=R/'evidence/roof-tiles'
P=Path('D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb')
L='C:/Program Files/LLVM/bin/llvm-pdbutil.exe'; B=0x140001000
addresses=[int(x,16) for x in __import__('sys').argv[1:]] or [0x141a36700]
con=[]
s=subprocess.run([L,'dump','--section-contribs',str(P)],capture_output=True,text=True,check=True).stdout
for line in s.splitlines():
 m=re.search(r'mod = (\d+), (\d+):(\d+), size = (\d+)',line)
 if m:
  mod,sec,off,size=map(int,m.groups())
  if sec==1 and any(B+off<=a<B+off+size for a in addresses):con.append(mod)
targets=[]
for mod in set(con):
 s=subprocess.run([L,'dump','--symbols',f'--modi={mod}',str(P)],capture_output=True,text=True,check=True).stdout
 (O/f'module{mod}.txt').write_text(s,encoding='utf-8'); lines=s.splitlines()
 for i,line in enumerate(lines):
  if 'S_GPROC32 ' not in line and 'S_LPROC32 ' not in line:continue
  name=re.search(r'`([^`]+)`',line); a=re.search(r'addr = (\d+):(\d+), code size = (\d+)',lines[i+1])
  if name and a:
   sec,off,size=map(int,a.groups())
   if sec==1 and B+off in addresses:targets.append(dict(name=name[1],va=hex(B+off),code_size=size,end_exclusive=hex(B+off+size),module=mod))
prior=json.loads((O/'procedures.json').read_text(encoding='utf-8')) if (O/'procedures.json').exists() else []
(O/'procedures.json').write_text(json.dumps(list({x['va']:x for x in prior+targets}.values()),indent=2),encoding='utf-8')
os.environ['GHIDRA_INSTALL_DIR']='D:/tools/rea/ghidra/ghidra_12.1.4_PUBLIC';os.environ['JAVA_HOME']='D:/tools/rea/jdk/jdk-21.0.12.1+1'
from pyghidra.launcher import HeadlessPyGhidraLauncher
l=HeadlessPyGhidraLauncher(); l.vm_args=[x for x in l.vm_args if not x.startswith('-Xmx')];l.add_vmargs('-Xmx3G');l.start()
from ghidra.base.project import GhidraProject
from ghidra.app.cmd.disassemble import DisassembleCommand
from ghidra.app.decompiler import DecompInterface
from ghidra.app.script import GhidraScriptUtil
from ghidra.program.model.address import AddressSet
from ghidra.program.model.symbol import SourceType
from ghidra.util.task import ConsoleTaskMonitor
from java.io import File
D=R/'workspace/ghidra-tiles';D.mkdir(exist_ok=True);GhidraScriptUtil.acquireBundleHostReference();project=None
try:
 if (D/'tiles.gpr').exists():project=GhidraProject.openProject(str(D),'tiles');p=project.openProgram('/','tiny-glade.exe',False)
 else:
  project=GhidraProject.createProject(str(D),'tiles',False);p=project.importProgram(File(str(R/'workspace/input/tiny-glade.exe')));project.saveAs(p,'/','tiny-glade.exe',True)
 mon=ConsoleTaskMonitor();sp=p.getAddressFactory().getDefaultAddressSpace();fm=p.getFunctionManager();de=DecompInterface();de.openProgram(p)
 for t in targets:
  start=sp.getAddress(t['va']);body=AddressSet(start,sp.getAddress(hex(int(t['end_exclusive'],16)-1)));DisassembleCommand(start,body,True).applyTo(p,mon)
  f=fm.getFunctionAt(start)
  for nested in fm.getFunctions(body,True):
   if nested.getEntryPoint()!=start:fm.removeFunction(nested.getEntryPoint())
  if f is None:f=fm.createFunction('tiles',start,body,SourceType.IMPORTED)
  else:f.setBody(body)
  ds=[str(i.getAddress())+': '+str(i) for i in p.getListing().getInstructions(body,True)]
  label=t['va'];(O/(label+'-disassembly.txt')).write_text('\n'.join(ds),encoding='utf-8');res=de.decompileFunction(f,120,mon)
  if res.decompileCompleted():(O/(label+'.c')).write_text(str(res.getDecompiledFunction().getC()).replace('\r',''),encoding='utf-8')
  print(t,res.decompileCompleted(),flush=True)
 de.dispose();project.save(p)
finally:
 if project:project.close()
 GhidraScriptUtil.releaseBundleHostReference()
