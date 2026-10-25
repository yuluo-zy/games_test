from pathlib import Path
from pyghidra.launcher import PyGhidraLauncher
_original_java_home = PyGhidraLauncher.java_home
PyGhidraLauncher.java_home = property(lambda self: Path("D:/tools/rea/jdk/jdk-21.0.12.1+1"), _original_java_home.fset)
