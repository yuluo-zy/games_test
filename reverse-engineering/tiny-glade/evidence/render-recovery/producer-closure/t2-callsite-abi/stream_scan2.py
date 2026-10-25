#!/usr/bin/env python3
# Read-only forensic tool (t2-callsite-abi).
# Per-module symbol analysis: for every CodeView module stream, read its
# S_OBJNAME (object file path) and count S_LOCAL / S_REGREL32 / S_FRAMEPROC /
# S_GPROC32 records, splitting by whether the module belongs to the GAME's own
# Rust code (obj path mentions tiny_glade / rhapsody / country-slice) vs system.
#
# Usage: python -I stream_scan2.py <pdb> <out_json>
import struct, sys, json, io, collections, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pdb_scan as P

def find_start(buf):
    for st in (0, 4, 8):
        if st + 4 > len(buf): continue
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

GAME_KEYS = ('tiny_glade', 'rhapsody', 'country-slice', 'pouncelight')

def main():
    pdb, outj = sys.argv[1], sys.argv[2]
    d = open(pdb, 'rb').read()
    msf = P.Msf(d)
    per = []
    glob = collections.Counter()
    total = collections.Counter()
    for si in range(msf.num_streams):
        buf = msf.stream(si)
        if len(buf) < 8: continue
        st = find_start(buf)
        if st is None: continue
        obj = None
        c = collections.Counter()
        sample_local = []
        for kind, off2, rlen, data in walk(buf, st, len(buf)):
            c[kind] += 1; total[kind] += 1
            if kind == 0x1101 and obj is None:      # S_OBJNAME
                obj = P.cstr(data[4:])
            elif kind == 0x113e:                    # S_LOCAL
                lflags = struct.unpack_from('<H', data, 4)[0] if len(data) >= 6 else 0
                nm = P.cstr(data[6:])
                if len(sample_local) < 5:
                    sample_local.append({'name': nm, 'flags': lflags,
                                         'typeidx': struct.unpack_from('<I', data, 0)[0]})
        if c.get(0x1110, 0) + c.get(0x110f, 0) == 0 and c.get(0x113e, 0) == 0:
            continue
        is_game = bool(obj) and any(k in obj for k in GAME_KEYS)
        per.append(dict(stream=si, obj=obj, is_game=is_game,
                        n_gproc32=c.get(0x1110,0), n_lproc32=c.get(0x110f,0),
                        n_local=c.get(0x113e,0), n_regrel32=c.get(0x1111,0),
                        n_register=c.get(0x1106,0), n_bprel32=c.get(0x110b,0),
                        n_frameproc=c.get(0x1012,0),
                        sample_local=sample_local))
    game = [p for p in per if p['is_game']]
    with_local = [p for p in per if p['n_local'] > 0]
    game_with_local = [p for p in game if p['n_local'] > 0]
    out = dict(pdb=pdb,
               module_streams_with_syms=len(per),
               game_modules=len(game), game_modules_with_local=len(game_with_local),
               modules_with_local=len(with_local),
               total_counts={P.KINDS.get(k,'?'): v for k,v in sorted(total.items(), key=lambda x:-x[1])},
               game_module_list=[{'stream':p['stream'],'obj':p['obj'],'local':p['n_local'],
                                  'gproc':p['n_gproc32']+p['n_lproc32'],'frameproc':p['n_frameproc']}
                                 for p in game][:200],
               game_local_samples=[p['sample_local'] for p in game_with_local][:20],
               modules_with_local_sample=[{'stream':p['stream'],'obj':p['obj'],'local':p['n_local']}
                                          for p in with_local[:40]])
    json.dump(out, io.open(outj,'w',encoding='utf-8'), ensure_ascii=False, indent=1)
    print('module streams with syms:', len(per))
    print('game modules (by obj keyword):', len(game), ' of which with S_LOCAL:', len(game_with_local))
    print('any modules with S_LOCAL:', len(with_local))
    print('per-kind totals:', dict(sorted(total.items(), key=lambda x:-x[1])[:15]))
    print('game module objs sample:')
    for p in game[:15]:
        print('  stream', p['stream'], 'local', p['n_local'], 'gproc', p['n_gproc32']+p['n_lproc32'], p['obj'][:90])
    print('wrote', outj)

if __name__ == '__main__':
    main()
