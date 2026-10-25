import json, hashlib, time
from pathlib import Path
import jpype
from ghidra.app.decompiler import DecompInterface
from ghidra.program.model.data import PointerDataType, VoidDataType
from ghidra.program.model.listing import Function, ParameterImpl
from ghidra.program.model.symbol import SourceType
from java.util import ArrayList
from ghidra.program.model.pcode import JumpTable
from ghidra.program.model.address import AddressSet
outdir=Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-0ae341593508bff12c/core-duck-building-20261021/direct-main-helper-20261021')
addr=currentProgram.getAddressFactory().getAddress("1407f98e0")
buf=jpype.JArray(jpype.JByte)(13201)
read=currentProgram.getMemory().getBytes(addr,buf)
body_sha=hashlib.sha256(bytes((int(b)&255) for b in buf)).hexdigest()
assert read==13201 and body_sha=='c8e2ed61fdbc155c4dfa84e1248e266eb1da64a913e0764ecb9b2be3911fe23d', (read,body_sha)
f=currentProgram.getFunctionManager().getFunctionAt(addr)
assert f is not None
before=str(f.getSignature())
body_before=int(f.getBody().getNumAddresses())
tx=currentProgram.startTransaction("duck-mesh rails seventeen-flat ABI private overlay")
try:
    dt=PointerDataType(VoidDataType.dataType,8,currentProgram.getDataTypeManager())
    params=ArrayList()
    for name in ["raw_reset_output","raw_widths","raw_tangents","raw_positions","raw_rng","raw_info28","raw_extra08","raw_arg8","raw_pair_flags","raw_railing_style","raw_flag11","raw_flag12","raw_flag13","raw_flag14","raw_roof_options","raw_samples_ptr","raw_samples_len"]:
        params.add(ParameterImpl(name,dt,currentProgram))
    f.setReturnType(VoidDataType.dataType,SourceType.USER_DEFINED)
    f.replaceParameters(params,Function.FunctionUpdateType.DYNAMIC_STORAGE_ALL_PARAMS,True,SourceType.USER_DEFINED)
    exact_body=AddressSet(addr,addr.add(13201-1))
    f.setBody(exact_body)
    for switch, cases in [("1407f9bc6", ["1407f9c2f","1407f9bc8","1407f9c00","1407f9c20"]), ("1407f9bec", ["1407f9c2f","1407f9bee","1407fbe6a","1407f9c2f"])]:
        destinations=ArrayList()
        for case in cases:
            destination=currentProgram.getAddressFactory().getAddress(case)
            assert f.getBody().contains(destination), case
            if currentProgram.getListing().getInstructionAt(destination) is None:
                disassemble(destination)
            destinations.add(destination)
        JumpTable(currentProgram.getAddressFactory().getAddress(switch),destinations,True,0).writeOverride(f)
    currentProgram.endTransaction(tx,True)
except:
    currentProgram.endTransaction(tx,False)
    raise

interface=DecompInterface()
assert interface.openProgram(currentProgram)
start=time.time()
r=interface.decompileFunction(f,180,monitor)
row={"program":str(currentProgram.getName()),"program_executable_sha256":str(currentProgram.getExecutableSHA256()),"original_body_sha256":body_sha,"original_native_size":13201,"ghidra_body_num_addresses":f.getBody().getNumAddresses(),"ghidra_body_before":body_before,"signature_before":before,"signature_after":str(f.getSignature()),"completed":bool(r.decompileCompleted()),"error":str(r.getErrorMessage()),"elapsed_seconds":time.time()-start,"private_saved_overlay":False,"private_overlay_readback_verified":True,"save_boundary":"Private overlay; explicit save/reopen not verified; runtime has active transaction"}
if r.decompileCompleted():
    text=str(r.getDecompiledFunction().getC())
    (outdir/"rails-private17-v2.c").write_text(text)
    row["c_sha256"]=hashlib.sha256(text.encode()).hexdigest()
    row["c_chars"]=len(text)
(outdir/"rails-private17-v2-meta.json").write_text(json.dumps(row,ensure_ascii=False,indent=2))
interface.dispose()
print(json.dumps(row,ensure_ascii=False))
