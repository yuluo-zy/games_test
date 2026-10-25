import sys, time, pyghidra
pyghidra.start()
from reva.headless import RevaHeadlessLauncher
from java.io import File
l = RevaHeadlessLauncher(None, True, True, File(""+__import__("os").path.expanduser("~/tools")+"/reverse-runtime/main/project"), "TinyGlade", None)
l.start()
assert l.waitForServer(120000), "server not ready"
print("PORT", l.getPort(), flush=True)
while True: time.sleep(60)
