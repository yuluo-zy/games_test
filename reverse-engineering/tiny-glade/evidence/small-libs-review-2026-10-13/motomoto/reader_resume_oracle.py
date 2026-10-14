"""Bounded execution of the original B RNE reader leaf, never game startup.
Only inputs whose paths do not call external panic routines are executed.
This is analysis tooling, not reconstructed game code.
"""
import ctypes, hashlib, json, random, struct
from pathlib import Path
ROOT = Path(__file__).parent
source = Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
raw = source.read_bytes()
sha = hashlib.sha256(raw).hexdigest()
assert sha == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = struct.unpack_from('<I', raw, 0x3c)[0]
sections = struct.unpack_from('<H', raw, pe + 6)[0]
opt_size = struct.unpack_from('<H', raw, pe + 20)[0]
section_table = pe + 24 + opt_size
rva = 0x2080d70
for index in range(sections):
    off = section_table + 40 * index
    virtual_size, virtual_address, raw_size, raw_offset = struct.unpack_from('<IIII', raw, off+8)
    if virtual_address <= rva < virtual_address + max(virtual_size, raw_size):
        code = raw[raw_offset+rva-virtual_address:raw_offset+rva-virtual_address+0xdd]
        break
else: raise ValueError('Reader RVA missing')
kernel = ctypes.WinDLL('kernel32', use_last_error=True)
kernel.VirtualAlloc.argtypes = [ctypes.c_void_p, ctypes.c_size_t, ctypes.c_uint32, ctypes.c_uint32]
kernel.VirtualAlloc.restype = ctypes.c_void_p
kernel.VirtualFree.argtypes = [ctypes.c_void_p, ctypes.c_size_t, ctypes.c_uint32]
mem = kernel.VirtualAlloc(None, len(code), 0x3000, 0x40)
if not mem: raise ctypes.WinError(ctypes.get_last_error())
ctypes.memmove(mem, code, len(code))
Word = ctypes.c_uint64
Leaf = ctypes.WINFUNCTYPE(ctypes.c_void_p, ctypes.POINTER(Word), ctypes.POINTER(Word))
leaf = Leaf(mem)
def one(leaf,cursor):
 result=(Word*4)(0,0,0,0);leaf(result,cursor)
 if result[0]==0:return {'none':True,'position':cursor[2]}
 if result[1]==0:return {'error':'UnexpectedEof' if result[2]==0x2500000003 else 'InvalidInput','position':cursor[2]}
 return {'tag':result[3],'data_hex':ctypes.string_at(result[1],result[2]).hex(),'position':cursor[2]}
cases=[]
for declared in [2**63,2**64-17,100]:
 data=struct.pack('<QQ',7,declared)+bytes(64)
 backing=ctypes.create_string_buffer(data);cursor=(Word*3)(ctypes.addressof(backing),len(data),0)
 items=[one(leaf,cursor) for i in range(5)]
 cases.append({'declared':declared,'input_hex':data.hex(),'items':items})
(ROOT/'reader-resume-oracle.json').write_text(json.dumps({'source_sha256':sha,'original_instruction_execution':True,'external_calls_executed':False,'cases':cases},indent=2)+'\n')
kernel.VirtualFree(mem,0,0x8000)
print(json.dumps(cases))
