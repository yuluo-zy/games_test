"""Serve already analyzed B database through ReVa; no import or auto-analysis."""
import os, sys, signal, time, json
from pathlib import Path
ROOT=Path('/Users/yuluo/Desktop/game')
os.environ['JAVA_HOME']='/Users/yuluo/Library/Java/JavaVirtualMachines/corretto-21.0.5/Contents/Home'
os.environ['GHIDRA_INSTALL_DIR']=str(ROOT/'tools/ghidra_12.1.4_PUBLIC')
import pyghidra
pyghidra.start()
from ghidra.base.project import GhidraProject
from ghidra.framework.main import AppInfo
from ghidra.util.task import TaskMonitor
from reva.plugin import RevaProgramManager
from reva.headless import RevaHeadlessLauncher
from java.io import File
project=GhidraProject.openProject(str(ROOT/'tools/ghidra-projects/reva-test'), 'TinyGlade', True)
AppInfo.setActiveProject(project.getProject())
root=project.getProject().getProjectData().getRootFolder()
files=[]
def walk(folder):
 for f in folder.getFiles():
  files.append(str(f.getPathname()))
 for d in folder.getFolders(): walk(d)
walk(root)
print('FILES '+json.dumps(files), flush=True)
path='/tiny-glade.exe'
file=project.getProject().getProjectData().getFile(path)
consumer=object()
# JPype requires a Java consumer.
from java.lang import Object
consumer=Object()
program=file.getDomainObject(consumer, False, False, TaskMonitor.DUMMY)
RevaProgramManager.registerProgram(program)
config=ROOT/'games_test/reverse-engineering/tiny-glade/evidence/toolchain-revalidation/reva/server.properties'
launcher=RevaHeadlessLauncher(File(str(config)),False,False)
launcher.start()
assert launcher.waitForServer(30000)
print('READY '+json.dumps({'port':launcher.getPort(),'programPath':str(program.getDomainFile().getPathname()),'functions':program.getFunctionManager().getFunctionCount(),'imageBase':str(program.getImageBase())}),flush=True)
stop=False
def handler(*args):
 global stop
 stop=True
signal.signal(signal.SIGTERM,handler)
signal.signal(signal.SIGINT,handler)
while not stop: time.sleep(.2)
launcher.stop()
RevaProgramManager.unregisterProgram(program)
# Preserve explicitly committed MCP changes in the isolated clone.
program.save('ReVa validation',TaskMonitor.DUMMY)
program.release(consumer)
project.close()
