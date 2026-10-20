import re,glob,json,struct,pyghidra
pyghidra.start()
from ghidra.base.project import GhidraProject
from ghidra.program.model.symbol import SourceType
from ghidra.app.cmd.disassemble import DisassembleCommand
from ghidra.app.cmd.function import CreateFunctionCmd
from ghidra.util.task import ConsoleTaskMonitor
B=0x140000000
rv=set()
for f in glob.glob('/home/ubuntu/test/games_test/reverse-engineering/tiny-glade/skeleton/B/crates/**/*.rs',recursive=True):
    for m in re.finditer(r'"rva":(\d+)',open(f,errors='ignore').read()): rv.add(int(m.group(1)))
names={}
for s in json.load(open('/home/ubuntu/test/games_test/reverse-engineering/tiny-glade/evidence/render-architecture/public-symbols.json')):
    if s['va']: names.setdefault(int(s['va'],16), s['demangled'])
proj=GhidraProject.openProject("/home/ubuntu/tools/reverse-runtime/main/project","TinyGlade",False)
prog=proj.openProgram("/","tiny-glade.exe",False)
fm=prog.getFunctionManager(); af=prog.getAddressFactory().getDefaultAddressSpace(); mon=ConsoleTaskMonitor()
made=failed=named=0; tx=prog.startTransaction("leaf")
try:
    for r in sorted(rv):
        a=af.getAddress(B+r)
        if fm.getFunctionAt(a) is not None: continue
        DisassembleCommand(a,None,True).applyTo(prog,mon)
        CreateFunctionCmd(a).applyTo(prog,mon)
        f=fm.getFunctionAt(a)
        if f is None: failed+=1; print('fail',hex(B+r)); continue
        made+=1
        nm=names.get(B+r)
        if nm:
            try: f.setName(''.join(c if (c.isalnum() or c in '_:.$<>~@') else '_' for c in nm)[:200],SourceType.IMPORTED); named+=1
            except Exception as ex: pass
finally: prog.endTransaction(tx,True)
proj.save(prog); print("made",made,"failed",failed,"named",named,"total",fm.getFunctionCount())
proj.close(prog); proj.close()
