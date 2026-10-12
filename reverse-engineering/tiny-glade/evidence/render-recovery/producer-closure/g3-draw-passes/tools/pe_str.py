# Resolve RVA/VA -> file offset via PE sections (read-only forensic helper).
import struct, sys

def load_exe(path):
    with open(path,'rb') as f:
        return f.read()

def pe_sections(data):
    e_lfanew = struct.unpack_from('<I', data, 0x3c)[0]
    assert data[e_lfanew:e_lfanew+4]==b'PE\x00\x00'
    coff = e_lfanew+4
    nsec = struct.unpack_from('<H', data, coff+2)[0]
    optsize = struct.unpack_from('<H', data, coff+16)[0]
    opt = coff+20
    magic = struct.unpack_from('<H', data, opt)[0]
    imgbase = struct.unpack_from('<Q', data, opt+24)[0] if magic==0x20b else struct.unpack_from('<I', data, opt+28)[0]
    secoff = opt+optsize
    secs=[]
    for i in range(nsec):
        o=secoff+i*40
        name=data[o:o+8].rstrip(b'\x00').decode('latin1')
        vsize,va,rsize,roff=struct.unpack_from('<IIII', data, o+8)
        secs.append((name,va,vsize,roff,rsize))
    return imgbase, secs

def va_to_off(imgbase, secs, va):
    rva=va-imgbase
    for name,sva,vsize,roff,rsize in secs:
        if sva <= rva < sva+max(vsize,rsize):
            return roff+(rva-sva)
    return None

def read_str(data, off, maxlen=120):
    end=data.find(b'\x00', off, off+maxlen)
    if end<0: end=off+maxlen
    return data[off:end]

if __name__=='__main__':
    data=load_exe(sys.argv[1])
    imgbase,secs=pe_sections(data)
    print('imagebase',hex(imgbase))
    for name,sva,vsize,roff,rsize in secs:
        print('  sec',name,hex(sva),hex(vsize),hex(roff))
    for a in sys.argv[2:]:
        va=int(a,16)
        off=va_to_off(imgbase,secs,va)
        s=read_str(data,off) if off is not None else b'<nooff>'
        print(hex(va),'->',repr(s.decode('latin1')))
