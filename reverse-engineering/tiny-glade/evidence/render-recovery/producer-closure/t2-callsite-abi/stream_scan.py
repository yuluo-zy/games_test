#!/usr/bin/env python3
# Read-only forensic tool (t2-callsite-abi).
# Bypass DBI ModInfo entirely: scan EVERY stream, detect CodeView module symbol
# streams (they begin with an S_OBJNAME record), walk length-prefixed records,
# and report the global kind histogram + any S_LOCAL / S_REGREL32 / S_REGISTER /
# S_BPREL32 / S_FRAMEPROC records (which would directly encode parameters).
#
# Usage: python -I stream_scan.py <pdb> <out_json>
import struct, sys, json, io, collections, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pdb_scan as P

def find_start(buf):
    # try offsets 0,4,8: first record must be S_OBJNAME (0x1101) and chain valid
    for st in (0, 4, 8):
        if st + 4 > len(buf):
            continue
        rlen = struct.unpack_from('<H', buf, st)[0]
        kind = struct.unpack_from('<H', buf, st+2)[0]
        if kind == 0x1101 and 2 <= rlen <= 4096 and st+2+rlen <= len(buf):
            return st
    return None

def walk(buf, st, limit):
    off = st
    while off + 4 <= limit:
        rlen = struct.unpack_from('<H', buf, off)[0]
        if rlen < 2 or off + 2 + rlen > limit:
            return
        kind = struct.unpack_from('<H', buf, off+2)[0]
        yield kind, off, rlen, buf[off+4: off+2+rlen]
        off += 2 + rlen

def main():
    pdb, outj = sys.argv[1], sys.argv[2]
    d = open(pdb, 'rb').read()
    msf = P.Msf(d)
    hist = collections.Counter()
    mod_streams = 0
    total_recs = 0
    funcs = []
    examples = collections.defaultdict(list)   # kind -> list of parsed dicts
    param_for_func = collections.Counter()
    for si in range(msf.num_streams):
        buf = msf.stream(si)
        if len(buf) < 8:
            continue
        st = find_start(buf)
        if st is None:
            continue
        mod_streams += 1
        cur = None
        for kind, off2, rlen, data in walk(buf, st, len(buf)):
            hist[kind] += 1
            total_recs += 1
            if kind in (0x110f, 0x1110):  # S_LPROC32 / S_GPROC32
                if cur: funcs.append(cur)
                code_off = struct.unpack_from('<I', data, 0)[0] if len(data) >= 4 else 0
                typind = struct.unpack_from('<I', data, 6)[0] if len(data) >= 10 else 0
                nlen = struct.unpack_from('<I', data, 22)[0] if len(data) >= 26 else 0
                name = P.cstr(data[26:26+nlen]) if len(data) >= 26 and nlen < 100000 else ''
                cur = dict(stream=si, code_off=code_off, typind=typind, name=name,
                           params=[], frame=None)
            elif kind == 0x113e and cur is not None:   # S_LOCAL
                typind = struct.unpack_from('<I', data, 0)[0]
                lflags = struct.unpack_from('<H', data, 4)[0]
                name = P.cstr(data[6:])
                r = dict(stream=si, func=cur['name'], kind='S_LOCAL', name=name,
                         typeidx=typind, flags=lflags)
                cur['params'].append(r)
                if len(examples['S_LOCAL']) < 8: examples['S_LOCAL'].append(r)
            elif kind == 0x1111 and cur is not None:   # S_REGREL32
                r_off = struct.unpack_from('<i', data, 0)[0]
                typind = struct.unpack_from('<I', data, 4)[0]
                reg = struct.unpack_from('<H', data, 8)[0]
                name = P.cstr(data[10:])
                r = dict(stream=si, func=cur['name'], kind='S_REGREL32', name=name,
                         typeidx=typind, reg=reg, regname=P.CV_REGS.get(reg, hex(reg)), off=r_off)
                cur['params'].append(r)
                if len(examples['S_REGREL32']) < 8: examples['S_REGREL32'].append(r)
            elif kind == 0x1106 and cur is not None:   # S_REGISTER
                typind = struct.unpack_from('<I', data, 0)[0]
                reg = struct.unpack_from('<H', data, 4)[0]
                name = P.cstr(data[6:])
                r = dict(stream=si, func=cur['name'], kind='S_REGISTER', name=name,
                         typeidx=typind, reg=reg, regname=P.CV_REGS.get(reg, hex(reg)))
                cur['params'].append(r)
                if len(examples['S_REGISTER']) < 8: examples['S_REGISTER'].append(r)
            elif kind == 0x110b and cur is not None:   # S_BPREL32
                bp_off = struct.unpack_from('<i', data, 0)[0]
                typind = struct.unpack_from('<I', data, 4)[0]
                name = P.cstr(data[8:])
                r = dict(stream=si, func=cur['name'], kind='S_BPREL32', name=name,
                         typeidx=typind, off=bp_off)
                cur['params'].append(r)
                if len(examples['S_BPREL32']) < 8: examples['S_BPREL32'].append(r)
            elif kind == 0x1012 and cur is not None:   # S_FRAMEPROC
                vals = list(struct.unpack_from('<8I', data, 0)) if len(data) >= 32 else None
                cur['frame'] = vals
        if cur: funcs.append(cur)

    with_params = [f for f in funcs if f['params']]
    out = dict(
        pdb=pdb, num_streams=msf.num_streams, module_symbol_streams=mod_streams,
        total_records=total_recs, funcs=len(funcs), funcs_with_param_records=len(with_params),
        kind_histogram={('%s' % P.KINDS.get(k, '?')) + ('(0x%04x)' % k): v
                        for k, v in hist.most_common()},
        counts=dict(S_FRAMEPROC=hist.get(0x1012, 0), S_LOCAL=hist.get(0x113e, 0),
                    S_REGREL32=hist.get(0x1111, 0), S_REGISTER=hist.get(0x1106, 0),
                    S_BPREL32=hist.get(0x110b, 0), S_GPROC32=hist.get(0x1110, 0),
                    S_LPROC32=hist.get(0x110f, 0), S_OBJNAME=hist.get(0x1101, 0),
                    S_PUB32=hist.get(0x110e, 0)),
        examples=examples,
        funcs_sample=with_params[:300],
    )
    json.dump(out, io.open(outj, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
    print('module_symbol_streams=%d total_records=%d funcs=%d funcs_with_params=%d'
          % (mod_streams, total_recs, len(funcs), len(with_params)))
    print('counts:', out['counts'])
    print('top kinds:')
    for k, v in hist.most_common(20):
        print('  %-14s 0x%04x %d' % (P.KINDS.get(k, '?'), k, v))
    print('wrote', outj)

if __name__ == '__main__':
    main()
