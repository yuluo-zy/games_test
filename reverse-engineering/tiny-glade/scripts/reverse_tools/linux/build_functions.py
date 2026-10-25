import struct, json, sys, time, os, pyghidra
pyghidra.start()
from ghidra.base.project import GhidraProject
from ghidra.program.model.address import AddressSet
from ghidra.program.model.symbol import SourceType
from ghidra.app.cmd.disassemble import DisassembleCommand
from ghidra.util.task import ConsoleTaskMonitor
EXE='/home/ubuntu/Downloads/tiny-glade.exe'
SYM='/home/ubuntu/test/games_test/reverse-engineering/tiny-glade/evidence/render-architecture/public-symbols.json'
d=open(EXE,'rb').read()
pe=struct.unpack_from('<I',d,0x3c)[0]; nsec=struct.unpack_from('<H',d,pe+6)[0]; opt=struct.unpack_from('<H',d,pe+20)[0]
sec=pe+24+opt; pd=None
for i in range(nsec):
    n,vs,va,rs,ro=struct.unpack_from('<8sIIII',d,sec+40*i)
    if n.startswith(b'.pdata'): pd=(vs,ro)
ents=[struct.unpack_from('<III',d,pd[1]+12*i)[:2] for i in range(pd[0]//12)]
names={}
for s in json.load(open(SYM)):
    if s['va']: names.setdefault(int(s['va'],16), s['demangled'])
proj=GhidraProject.openProject("/home/ubuntu/tools/reverse-runtime/main/project","TinyGlade",False)
prog=proj.openProgram("/","tiny-glade.exe",False)
fm=prog.getFunctionManager(); af=prog.getAddressFactory().getDefaultAddressSpace()
mon=ConsoleTaskMonitor(); t0=time.time(); made=named=0; B=0x140000000
CH=1000
for i in range(0,len(ents),CH):
    tx=prog.startTransaction("pdata functions")
    try:
        for b,e in ents[i:i+CH]:
            a=af.getAddress(B+b); body=AddressSet(a,af.getAddress(B+e-1))
            DisassembleCommand(a,body,True).applyTo(prog,mon)
            f=fm.getFunctionAt(a)
            nm=names.get(B+b)
            if f is None:
                f=fm.createFunction(None,a,body,SourceType.IMPORTED)
            elif f.getBody().getNumAddresses()!=body.getNumAddresses():
                f.setBody(body)
            if f is not None:
                made+=1
                if nm:
                    try:
                        f.setName(nm.replace(' ','_')[:200],SourceType.IMPORTED); named+=1
                    except Exception:
                        try:
                            f.setName(''.join(c if (c.isalnum() or c in '_:.$<>~@') else '_' for c in nm)[:200],SourceType.IMPORTED); named+=1
                        except Exception as ex: print('nameerr',hex(B+b),ex,flush=True)
    finally:
        prog.endTransaction(tx,True)
    if (i//CH)%10==0:
        print(f"{i}/{len(ents)} made={made} named={named} {time.time()-t0:.0f}s",flush=True)
        proj.save(prog)
proj.save(prog)
print("FINAL made",made,"named",named,"count",fm.getFunctionCount(),flush=True)
proj.close(prog); proj.close()
