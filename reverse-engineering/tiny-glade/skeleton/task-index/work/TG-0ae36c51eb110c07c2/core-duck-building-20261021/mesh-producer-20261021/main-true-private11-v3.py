import json, hashlib, time
from pathlib import Path
import jpype
from ghidra.app.decompiler import DecompInterface
from ghidra.program.model.data import PointerDataType, VoidDataType
from ghidra.program.model.listing import Function, ParameterImpl
from ghidra.program.model.symbol import SourceType
from java.util import ArrayList
outdir=Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-0ae36c51eb110c07c2/core-duck-building-20261021/mesh-producer-20261021')
addr=currentProgram.getAddressFactory().getAddress("140008c90")
buf=jpype.JArray(jpype.JByte)(62374)
read=currentProgram.getMemory().getBytes(addr,buf)
body_sha=hashlib.sha256(bytes((int(b)&255) for b in buf)).hexdigest()
assert read==62374 and body_sha=='dd4ce03b05b0604834781d8d1dec668a3a9561f4f1b4bd1d4c64e33c1e5e9bbb', (read,body_sha)
f=currentProgram.getFunctionManager().getFunctionAt(addr)
assert f is not None
before=str(f.getSignature())
assert len(f.getParameters()) == 11, str(f.getSignature())
interface=DecompInterface()
assert interface.openProgram(currentProgram)
start=time.time()
r=interface.decompileFunction(f,180,monitor)
row={"program":str(currentProgram.getName()),"program_executable_sha256":str(currentProgram.getExecutableSHA256()),"original_body_sha256":body_sha,"original_native_size":62374,"ghidra_body_num_addresses":f.getBody().getNumAddresses(),"signature_before":before,"signature_after":str(f.getSignature()),"completed":bool(r.decompileCompleted()),"error":str(r.getErrorMessage()),"elapsed_seconds":time.time()-start,"private_saved_overlay":False,"private_overlay_readback_verified":True,"save_boundary":"ReVa runtime active transaction prevented explicit save in v2; persistent reopen not verified"}
if r.decompileCompleted():
    text=str(r.getDecompiledFunction().getC())
    (outdir/"main-true-private11.c").write_text(text)
    row["c_sha256"]=hashlib.sha256(text.encode()).hexdigest()
    row["c_chars"]=len(text)
(outdir/"main-true-private11-meta.json").write_text(json.dumps(row,ensure_ascii=False,indent=2))
interface.dispose()
print(json.dumps(row,ensure_ascii=False))
