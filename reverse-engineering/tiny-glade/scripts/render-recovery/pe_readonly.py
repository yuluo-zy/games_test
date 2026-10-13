"""只读 PE 工具：VA->文件偏移、反汇编（objdump）。用法: pe.py <pe> dis <va_hex> <len>"""
import struct, subprocess, sys
class PE:
    def __init__(s, path):
        s.d = open(path, 'rb').read()   # 只读
        e = struct.unpack_from('<I', s.d, 0x3c)[0]
        n = struct.unpack_from('<H', s.d, e+6)[0]
        opt = struct.unpack_from('<H', s.d, e+20)[0]
        s.base = struct.unpack_from('<Q', s.d, e+24+24)[0]
        so = e+24+opt
        s.sec = []
        for i in range(n):
            nm, vs, va, rs, ro = struct.unpack_from('<8sIIII', s.d, so+40*i)
            s.sec.append((nm.rstrip(b'\0').decode(), va, max(vs, rs), ro, rs))
    def off(s, va):
        r = va - s.base
        for nm, sva, sz, ro, rs in s.sec:
            if sva <= r < sva+sz: return ro + r - sva
        raise ValueError(hex(va))
    def read(s, va, n): o = s.off(va); return s.d[o:o+n]
if __name__ == '__main__':
    p = PE(sys.argv[1]); print('base', hex(p.base), [(x[0], hex(x[1])) for x in p.sec])

def func_range(p, va):
    """用 .pdata RUNTIME_FUNCTION 查 va 所在函数 [begin,end)"""
    nm, sva, sz, ro, rs = [x for x in p.sec if x[0]=='.pdata'][0]
    rva = va - p.base; lo, hi = 0, rs//12 - 1
    while lo <= hi:
        m = (lo+hi)//2; b, e, _ = struct.unpack_from('<III', p.d, ro+12*m)
        if rva < b: hi = m-1
        elif rva >= e: lo = m+1
        else: return p.base+b, p.base+e
    return None

def dis(p, va, n=None):
    if n is None:
        r = func_range(p, va); n = r[1]-va
    open('/tmp/_pe_blob','wb').write(p.read(va, n))
    out = subprocess.run(['objdump','-D','-b','binary','-mi386:x86-64','-M','intel','--adjust-vma=%#x'%va,'/tmp/_pe_blob'],capture_output=True,text=True).stdout
    return out.split('<.data>:\n')[-1]
