#!/usr/bin/env python3
# Read-only forensic tool (t2-callsite-abi).
# Goal: determine whether the shipped PDB carries per-function parameter evidence:
#   S_LOCAL / S_REGREL32 / S_REGISTER / S_BPREL32 / S_FRAMEPROC records
# inside module symbol streams. If present, these directly give parameter
# name + register/stack location (i.e. a signature), better than ABI inference.
#
# Usage: python -I pdb_scan.py <pdb> <out_json> [max_modules]
import struct, sys, json, io, collections

def read_superblock(d):
    magic = d[:32]
    assert magic.startswith(b'Microsoft C/C++ MSF 7.00'), magic
    bs, fbm, nblocks, dirbytes, unk, bmaddr = struct.unpack_from('<IIIIII', d, 32)
    return bs, fbm, nblocks, dirbytes, unk, bmaddr

def read_dir_blocks(d, bs, dirbytes, bmaddr):
    ndirblocks = (dirbytes + bs - 1) // bs
    # block bmaddr holds an array of u32 block numbers, one per dir block
    arr = d[bmaddr*bs: bmaddr*bs + ndirblocks*4]
    blocks = struct.unpack_from('<%dI' % ndirblocks, arr, 0)
    return list(blocks)

def read_block(d, bs, b):
    return d[b*bs:(b+1)*bs]

class Msf:
    def __init__(self, d):
        self.d = d
        self.bs, fbm, nblocks, dirbytes, unk, bmaddr = read_superblock(d)
        dirblocks = read_dir_blocks(d, self.bs, dirbytes, bmaddr)
        dirbuf = b''.join(read_block(d, self.bs, b) for b in dirblocks)
        self.dirbuf = dirbuf[:dirbytes]
        self.num_streams = struct.unpack_from('<I', self.dirbuf, 0)[0]
        self.sizes = list(struct.unpack_from('<%dI' % self.num_streams, self.dirbuf, 4))
        self.blocks = []
        off = 4 + 4*self.num_streams
        for sz in self.sizes:
            if sz == 0xFFFFFFFF:
                self.blocks.append([])
                continue
            nb = (sz + self.bs - 1)//self.bs
            bl = list(struct.unpack_from('<%dI' % nb, self.dirbuf, off))
            off += nb*4
            self.blocks.append(bl)
    def stream(self, i):
        if i >= self.num_streams or self.sizes[i] == 0xFFFFFFFF:
            return b''
        buf = b''.join(read_block(self.d, self.bs, b) for b in self.blocks[i])
        return buf[:self.sizes[i]]

CV_REGS = {
    0:'NONE',1:'AL',2:'CL',3:'DL',4:'BL',5:'AH',6:'CH',7:'DH',8:'BH',
    9:'AX',10:'CX',11:'DX',12:'BX',13:'SP',14:'BP',15:'SI',16:'DI',
    17:'EAX',18:'ECX',19:'EDX',20:'EBX',21:'ESP',22:'EBP',23:'ESI',24:'EDI',
    328:'RAX',329:'RBX',330:'RCX',331:'RDX',332:'RSI',333:'RDI',334:'RBP',335:'RSP',
    336:'R8',337:'R9',338:'R10',339:'R11',340:'R12',341:'R13',342:'R14',343:'R15',
    344:'R8B',345:'R9B',346:'R10B',347:'R11B',348:'R12B',349:'R13B',350:'R14B',351:'R15B',
    352:'R8W',353:'R9W',
    122:'XMM0',123:'XMM1',124:'XMM2',125:'XMM3',126:'XMM4',127:'XMM5',
    128:'XMM6',129:'XMM7',130:'XMM8',131:'XMM9',132:'XMM10',133:'XMM11',
    134:'XMM12',135:'XMM13',136:'XMM14',137:'XMM15',
}
KINDS = {
    0x1101:'S_OBJNAME',0x1102:'S_THUNK32',0x1103:'S_BLOCK32',0x1104:'S_WITH32',
    0x1105:'S_LABEL32',0x1106:'S_REGISTER',0x1107:'S_CONSTANT',0x1108:'S_UDT',
    0x110a:'S_MANYREG',0x110b:'S_BPREL32',0x110c:'S_LDATA32',0x110d:'S_GDATA32',
    0x110e:'S_PUB32',0x110f:'S_LPROC32',0x1110:'S_GPROC32',0x1111:'S_REGREL32',
    0x1112:'S_LTHREAD32',0x1113:'S_GTHREAD32',0x1116:'S_COMPILE2',
    0x1012:'S_FRAMEPROC',0x1147:'S_PROC_ID_END',0x1109:'S_COBOLUDT',
    0x113c:'S_COMPILE3',0x113d:'S_ENVBLOCK',0x113e:'S_LOCAL',
}

def walk_records(buf, off0, limit, hist):
    """Yield (kind, rec_start, rec_len, data) walking length-prefixed CV records."""
    off = off0
    while off + 4 <= limit:
        rlen = struct.unpack_from('<H', buf, off)[0]
        if rlen < 2 or off + 2 + rlen > limit:
            break
        kind = struct.unpack_from('<H', buf, off+2)[0]
        hist[kind] += 1
        yield kind, off, rlen, buf[off+4: off+2+rlen]
        off += 2 + rlen

def cstr(b):
    z = b.find(b'\x00')
    return b[:z].decode('latin1', 'replace') if z >= 0 else b.decode('latin1','replace')

def main():
    pdb, outj = sys.argv[1], sys.argv[2]
    maxmod = int(sys.argv[3]) if len(sys.argv) > 3 else 10**9
    d = open(pdb, 'rb').read()
    msf = Msf(d)
    print('streams:', msf.num_streams)
    dbi = msf.stream(3)
    (ver_sig, ver_hdr, age, gsi, build, psi, pdbrev, symrec_stream, rbold,
     modinfosz, seccontrsz, secmapsz, srcinfosz, tsmapsz, mfcidx, optdbghdrsz,
     ecsubsz) = struct.unpack_from('<iIIHHHHHHiiiiiIii', dbi, 0)
    flags, machine = struct.unpack_from('<HH', dbi, 56)
    print('DBI: ver_sig=%x gsi=%d symrec_stream=%d modinfosz=%d machine=%x flags=%x'
          % (ver_sig, gsi, symrec_stream, modinfosz, machine, flags))

    # parse ModInfo substream
    mods = []
    off = 64
    end = 64 + modinfosz
    while off < end and len(mods) < maxmod:
        (u1, sec, sec_off, mflags, modsym, symbytesz, c11sz, c13sz,
         srcfcount, pad, u2, srcnameidx, pdbfileidx) = struct.unpack_from('<IHHHHIIIHHIII', dbi, off)
        o2 = off + 40
        z = dbi.find(b'\x00', o2, end)
        modname = cstr(dbi[o2:z])
        o2 = z + 1
        z2 = dbi.find(b'\x00', o2, end)
        objname = cstr(dbi[o2:z2])
        o2 = z2 + 1
        o2 = (o2 + 3) & ~3
        mods.append(dict(mod=modname, obj=objname, modsym=modsym,
                         symbytesz=symbytesz, c11sz=c11sz, c13sz=c13sz))
        off = o2
    print('modules parsed:', len(mods))

    hist = collections.Counter()
    # per-function aggregates keyed by (module, proc_off)
    funcs = []
    local_kinds = {0x1106, 0x110b, 0x1111, 0x113e}
    n_with_params = 0
    n_mods_scanned = 0
    total_recs = 0
    example = None
    for mi, m in enumerate(mods):
        modsym = m['modsym']
        if modsym in (0xFFFF,) or modsym >= msf.num_streams:
            continue
        sbuf = msf.stream(modsym)
        if not sbuf:
            continue
        n_mods_scanned += 1
        # module symbol substream = first symbytesz bytes, possibly after 4-byte sig
        # find the correct start: first record should be S_OBJNAME (0x1101)
        start = 0
        if len(sbuf) >= 4 and struct.unpack_from('<H', sbuf, 2)[0] not in KINDS:
            start = 4
        limit = min(len(sbuf), start + m['symbytesz'] + 8)
        cur = None
        recs = []
        for kind, off2, rlen, data in walk_records(sbuf, start, limit, hist):
            total_recs += 1
            if kind in (0x110f, 0x1110):  # S_LPROC32 / S_GPROC32
                if cur:
                    funcs.append(cur)
                # LPROC32/GPROC32 payload: ulCodeOffset(4) seg(2) typind(4) off(4) ...
                code_off = struct.unpack_from('<I', data, 0)[0]
                typind = struct.unpack_from('<I', data, 6)[0]
                nlen = struct.unpack_from('<I', data, 22)[0] if len(data) >= 26 else 0
                name = cstr(data[26:26+nlen]) if len(data) >= 26 else ''
                cur = dict(mod=m['mod'], code_off=code_off, typind=typind,
                           name=name, params=[], frame=None, nlocal=0)
            elif kind == 0x113e and cur is not None:  # S_LOCAL
                typind = struct.unpack_from('<I', data, 0)[0]
                lflags = struct.unpack_from('<H', data, 4)[0]
                name = cstr(data[6:])
                cur['params'].append(dict(kind='S_LOCAL', name=name,
                                          typeidx=typind, flags=lflags, reg=None, off=None))
            elif kind == 0x1111 and cur is not None:  # S_REGREL32
                r_off = struct.unpack_from('<i', data, 0)[0]
                typind = struct.unpack_from('<I', data, 4)[0]
                reg = struct.unpack_from('<H', data, 8)[0]
                name = cstr(data[10:])
                cur['params'].append(dict(kind='S_REGREL32', name=name,
                                          typeidx=typind, reg=reg, regname=CV_REGS.get(reg,hex(reg)),
                                          off=r_off))
            elif kind in (0x1106, 0x110b) and cur is not None:  # S_REGISTER/S_BPREL32
                if kind == 0x110b:
                    bp_off = struct.unpack_from('<i', data, 0)[0]
                    typind = struct.unpack_from('<I', data, 4)[0]
                    name = cstr(data[8:])
                    cur['params'].append(dict(kind='S_BPREL32', name=name,
                                              typeidx=typind, off=bp_off))
                else:
                    typind = struct.unpack_from('<I', data, 0)[0]
                    reg = struct.unpack_from('<H', data, 4)[0]
                    name = cstr(data[6:])
                    cur['params'].append(dict(kind='S_REGISTER', name=name,
                                              typeidx=typind, reg=reg,
                                              regname=CV_REGS.get(reg,hex(reg))))
            elif kind == 0x1012 and cur is not None:  # S_FRAMEPROC
                vals = struct.unpack_from('<8I', data, 0) if len(data) >= 32 else None
                cur['frame'] = vals
            elif kind == 0x1147 and cur is not None:  # S_PROC_ID_END? (0x1147)
                funcs.append(cur); cur = None
        if cur:
            funcs.append(cur)
        if mi == 0:
            print('mod0 symbytesz=%d c11=%d c13=%d streamlen=%d start=%d'
                  % (m['symbytesz'], m['c11sz'], m['c13sz'], len(sbuf), start))

    with_params = [f for f in funcs if f['params']]
    print('total_records=%d funcs=%d funcs_with_param_records=%d' %
          (total_recs, len(funcs), len(with_params)))
    print('kind histogram (top 25):')
    for k, c in hist.most_common(25):
        print('  0x%04x %-14s %d' % (k, KINDS.get(k, '?'), c))

    out = dict(
        pdb=pdb, num_streams=msf.num_streams, modules=len(mods),
        modules_scanned=n_mods_scanned, total_records=total_recs,
        funcs=len(funcs), funcs_with_param_records=len(with_params),
        kind_histogram={('%s(0x%04x)' % (KINDS.get(k,'?'), k)): v for k, v in hist.most_common()},
        frameproc_count=hist.get(0x1012,0), local_count=hist.get(0x113e,0),
        regrel32_count=hist.get(0x1111,0), register_count=hist.get(0x1106,0),
        bprel32_count=hist.get(0x110b,0),
        funcs_sample=with_params[:400],
    )
    json.dump(out, io.open(outj, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
    print('wrote', outj)

if __name__ == '__main__':
    main()
