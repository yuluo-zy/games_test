import json, hashlib, time
from pathlib import Path
import jpype
from ghidra.app.decompiler import DecompInterface
from ghidra.program.model.data import PointerDataType, VoidDataType
from ghidra.program.model.listing import Function, ParameterImpl
from ghidra.program.model.symbol import SourceType
from java.util import ArrayList
outdir=Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-0ae36c51eb110c07c2/core-duck-building-20261021/mesh-producer-20261021')
addr=currentProgram.getAddressFactory().getAddress("1407dc170")
buf=jpype.JArray(jpype.JByte)(9864)
read=currentProgram.getMemory().getBytes(addr,buf)
body_sha=hashlib.sha256(bytes((int(b)&255) for b in buf)).hexdigest()
assert read==9864 and body_sha=='0499ed317437d28bce89e9270c327984fe8e90a064bf7e3208b1d16a33ed0be3', (read,body_sha)
f=currentProgram.getFunctionManager().getFunctionAt(addr)
assert f is not None
before=str(f.getSignature())
tx=currentProgram.startTransaction("duck-mesh genfloor thirteen-flat ABI private overlay")
try:
    dt=PointerDataType(VoidDataType.dataType,8,currentProgram.getDataTypeManager())
    params=ArrayList()
    for name in ["raw_device","raw_widths_ptr","raw_widths_len","raw_tangents_ptr","raw_tangents_len","raw_positions_ptr","raw_positions_len","raw_reset_output","raw_commands","raw_info28","raw_roof_options","raw_aux_ptr","raw_aux_len"]:
        params.add(ParameterImpl(name,dt,currentProgram))
    f.setReturnType(VoidDataType.dataType,SourceType.USER_DEFINED)
    f.replaceParameters(params,Function.FunctionUpdateType.DYNAMIC_STORAGE_ALL_PARAMS,True,SourceType.USER_DEFINED)
    currentProgram.endTransaction(tx,True)
except:
    currentProgram.endTransaction(tx,False)
    raise

interface=DecompInterface()
assert interface.openProgram(currentProgram)
start=time.time()
r=interface.decompileFunction(f,180,monitor)
row={"program":str(currentProgram.getName()),"program_executable_sha256":str(currentProgram.getExecutableSHA256()),"original_body_sha256":body_sha,"original_native_size":9864,"ghidra_body_num_addresses":f.getBody().getNumAddresses(),"signature_before":before,"signature_after":str(f.getSignature()),"completed":bool(r.decompileCompleted()),"error":str(r.getErrorMessage()),"elapsed_seconds":time.time()-start,"private_saved_overlay":False,"private_overlay_readback_verified":True,"save_boundary":"Private overlay; explicit save/reopen not verified; runtime has active transaction"}
if r.decompileCompleted():
    text=str(r.getDecompiledFunction().getC())
    (outdir/"floor-private13-v2.c").write_text(text)
    row["c_sha256"]=hashlib.sha256(text.encode()).hexdigest()
    row["c_chars"]=len(text)
(outdir/"floor-private13-v2-meta.json").write_text(json.dumps(row,ensure_ascii=False,indent=2))
interface.dispose()
print(json.dumps(row,ensure_ascii=False))
