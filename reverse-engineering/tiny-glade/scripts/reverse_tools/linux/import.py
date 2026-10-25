import sys, pyghidra
pyghidra.start(install_dir=None)
from ghidra.base.project import GhidraProject
proj = GhidraProject.createProject(""+__import__("os").path.expanduser("~/tools")+"/reverse-runtime/main/project", "TinyGlade", False)
from java.io import File
from ghidra.program.util import GhidraProgramUtilities
prog = proj.importProgram(File("/home/ubuntu/Downloads/tiny-glade.exe"))
print("imported", prog.getName(), prog.getLanguageID(), prog.getImageBase())
proj.saveAs(prog, "/", "tiny-glade.exe", True)
proj.close(prog); proj.close()
print("done")
