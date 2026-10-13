import os, runpy, sys
from pathlib import Path
root = Path(r"D:/game/tools/reverse-runtime/shader-compute-gap-1013")
for name, folder in [("APPDATA", "appdata"), ("LOCALAPPDATA", "localappdata")]:
    directory = root / folder
    directory.mkdir(parents=True, exist_ok=True)
    os.environ[name] = str(directory)
operator = Path(r"D:/game/reverse-engineering/tiny-glade/scripts/reverse_tools/reverse_operator.py")
sys.path.insert(0, str(operator.parent))
sys.argv = [str(operator)] + sys.argv[1:]
runpy.run_path(str(operator), run_name="__main__")
