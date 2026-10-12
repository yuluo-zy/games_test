#!/usr/bin/env python3
# Read-only: disassemble given VAs straight from the PE image.
# Usage: python -I pe_dis.py <exe> <repo_root> <va,va,...>
import struct, sys, json, io, os
sys.path.insert(0, r"C:\Users\liyu\AppData\Local\Packages\PythonSoftwareFoundation.Python.3.12_qbz5n2kfra8p0\LocalCache\local-packages\Python312\site-packages")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import X86_OP_MEM, X86_OP_IMM, X86_REG_RIP

def load_secs(d):
    e = struct.unpack_from('<I', d, 0x3c)[0]
    nsec = struct.unpack_from('<H', d, e + 6)[0]
    optsz = struct.unpack_from('<H', d, e + 20)[0]
    secs = []; off = e + 24 + optsz
    for i in range(nsec):
        nm = d[off:off+8].rstrip(b'\x00').decode('latin1')
        vsz, va, rsz, ra = struct.unpack_from('<IIII', d, off + 8)
        secs.append((nm, va, vsz, ra, rsz)); off += 40
    return secs

def main():
    exe, root, vas = sys.argv[1], sys.argv[2], sys.argv[3]
    d = open(exe,'rb').read()
    secs = load_secs(d)
    IB = 0x140000000
    def off_of(rva):
        for nm,va,vsz,ra,rsz in secs:
            if va <= rva < va+max(vsz,rsz): return ra+(rva-va)
    # symbol map
    syms = {}
    wgpath = os.path.join(root,'evidence/render-recovery/workgraph/functions.json')
    for f in json.load(io.open(wgpath,encoding='utf-8')):
        syms.setdefault(int(f['va'],16), f['names'][0])
    md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail=True
    for vs in vas.split(','):
        va = int(vs,16)
        fo = off_of(va-IB)
        print('==== %s  file_off=0x%x' % (vs, fo))
        # size: distance to next known symbol or default 0x400
        nxts = [v for v in syms if v>va]
        size = (min(nxts)-va) if nxts else 0x600
        size = min(size, 4096)
        raw = d[fo:fo+size]
        n=0
        for ins in md.disasm(raw, va):
            ann=''
            for op in ins.operands:
                if op.type==X86_OP_MEM and op.mem.base==X86_REG_RIP:
                    tgt=ins.address+ins.size+op.mem.disp
                    ann+=' -> %s' % (syms.get(tgt) or hex(tgt))
                elif op.type==X86_OP_IMM and op.imm in syms:
                    ann+=' -> %s' % syms[op.imm]
            print('  %08x  %-24s %s %s%s' % (ins.address, ins.bytes.hex(), ins.mnemonic, ins.op_str, ann))
            n+=1
            if n>400: break

if __name__=='__main__': main()
