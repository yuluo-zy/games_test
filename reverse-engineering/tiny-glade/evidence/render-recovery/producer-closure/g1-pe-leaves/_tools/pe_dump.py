#!/usr/bin/env python3
# Read-only forensic tool: disassemble micro-leaf functions straight from the PE image.
# Usage: python -I pe_dump.py <exe> <repo_root> <out_json>
import struct, sys, json, io, os, glob, re
import site as _site
_usp = _site.getusersitepackages()
if _usp and _usp not in sys.path:
    sys.path.insert(0, _usp)  # capstone is installed in user site; -I disables it

def main():
    exe, root, outjson = sys.argv[1], sys.argv[2], sys.argv[3]
    d = open(exe, 'rb').read()
    e = struct.unpack_from('<I', d, 0x3c)[0]
    nsec = struct.unpack_from('<H', d, e + 6)[0]
    optsz = struct.unpack_from('<H', d, e + 20)[0]
    secs = []
    off = e + 24 + optsz
    for i in range(nsec):
        nm = d[off:off+8].rstrip(b'\x00').decode('latin1')
        vsz, va, rsz, ra = struct.unpack_from('<IIII', d, off + 8)
        secs.append((nm, va, vsz, ra, rsz))
        off += 40
    IMAGEBASE = 0x140000000
    def off_of(rva):
        for nm, va, vsz, ra, rsz in secs:
            if va <= rva < va + max(vsz, rsz):
                return ra + (rva - va)
        return None

    # ---- symbol map (va int -> name) ----
    syms = {}
    def add(v, n):
        if v is None: return
        syms.setdefault(v, n)
    wg = json.load(io.open(os.path.join(root, 'evidence/render-recovery/workgraph/functions.json'), encoding='utf-8'))
    for f in wg:
        add(int(f['va'], 16), f['names'][0])
    for p in [os.path.join(root, x) for x in [
        'evidence/render-architecture/symbol-seeds.json',
        'evidence/render-recovery/producer-closure/c1-country-core/country_core_render_symbols.json']]:
        try:
            data = json.load(io.open(p, encoding='utf-8'))
        except Exception:
            continue
        it = data.items() if isinstance(data, dict) else [(x.get('va'), x.get('demangled') or x.get('symbol')) for x in data]
        for k, v in it:
            if not k: continue
            try: va = int(k, 16) if isinstance(k, str) else int(k)
            except Exception: continue
            if va < IMAGEBASE: va += IMAGEBASE
            add(va, v)
    # public-symbols with section/offset -> derive va
    try:
        ps = json.load(io.open(os.path.join(root, 'evidence/render-architecture/public-symbols.json'), encoding='utf-8'))
        # build section va table index (1-based sections)
        secva = [s[1] for s in secs]
        for x in ps:
            va = x.get('va')
            if va:
                try: add(int(va, 16) if isinstance(va, str) else int(va), x.get('demangled') or x.get('symbol'))
                except Exception: pass
    except Exception:
        pass

    # ---- missing functions ----
    pat = re.compile(r'0x([0-9a-fA-F]{6,16})'); have = set()
    for p in glob.glob(os.path.join(root, 'evidence/render-recovery/producer-closure/**/*.c'), recursive=True) + \
             glob.glob(os.path.join(root, 'evidence/render-architecture/*.c')):
        m = pat.search(os.path.basename(p))
        if m: have.add(int(m.group(1), 16))
    miss = [f for f in wg if int(f['va'], 16) not in have]

    from capstone import Cs, CS_ARCH_X86, CS_MODE_64
    from capstone.x86 import X86_OP_MEM, X86_OP_IMM, X86_REG_RIP
    md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True

    out = []
    for f in sorted(miss, key=lambda f: -f['code_size']):
        va = int(f['va'], 16); size = f['code_size']
        fo = off_of(va - IMAGEBASE)
        if fo is None:
            out.append({'va': f['va'], 'size': size, 'name': f['names'][0], 'error': 'no section for rva'})
            continue
        raw = d[fo:fo+size]
        insns = []
        for ins in md.disasm(raw, va):
            ann = ''
            try:
                for op in ins.operands:
                    if op.type == X86_OP_MEM and op.mem.base == X86_REG_RIP:
                        tgt = ins.address + ins.size + op.mem.disp
                        t = syms.get(tgt)
                        ann += (' -> %s' % (t or hex(tgt)))
                    elif op.type == X86_OP_IMM and op.imm in syms:
                        ann += (' -> %s' % syms[op.imm])
            except Exception:
                pass
            insns.append({'a': hex(ins.address), 'b': ins.bytes.hex(), 'm': ins.mnemonic, 'o': ins.op_str, 'ann': ann})
        out.append({'va': f['va'], 'size': size, 'end': f.get('end_exclusive'), 'name': f['names'][0],
                    'module': f.get('modules'), 'bytes': raw.hex(), 'insns': insns,
                    'n_decoded': len(insns), 'n_bytes_decoded': sum(len(i['b'])//2 for i in insns)})
    json.dump(out, io.open(outjson, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
    print('wrote', outjson, 'funcs', len(out))

if __name__ == '__main__':
    main()
