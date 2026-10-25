#!/usr/bin/env python3
# Read-only forensic tool for Tiny Glade RE (companion to analyze_indirect.py).
#   * resolves call [rip+disp] pointers to import names / internal targets
#   * local backward data-flow for call [reg+disp] and call reg  -> internal edges
#   * scans .rdata/.data for function-pointer tables (vtables), both
#     32-bit image-relative RVA and 64-bit absolute VA encodings
#   * emits indirect-callgraph.json (calls.json-compatible) + vtables.json
# Usage: python -I resolve_and_vtables.py <exe> <repo_root> <out_dir>
import struct, sys, json, io, os
import site as _site
_usp = _site.getusersitepackages()
if _usp and _usp not in sys.path:
    sys.path.insert(0, _usp)

IB = 0x140000000


def parse_pe(d):
    e = struct.unpack_from('<I', d, 0x3c)[0]
    nsec = struct.unpack_from('<H', d, e + 6)[0]
    optsz = struct.unpack_from('<H', d, e + 20)[0]
    off = e + 24 + optsz
    secs = []
    for i in range(nsec):
        nm = d[off:off + 8].rstrip(b'\x00').decode('latin1')
        vsz, va, rsz, ra = struct.unpack_from('<IIII', d, off + 8)
        secs.append({'name': nm, 'rva': va, 'vsz': vsz, 'ra': ra, 'rsz': rsz})
        off += 40
    return secs


def main():
    exe, root, outdir = sys.argv[1], sys.argv[2], sys.argv[3]
    d = open(exe, 'rb').read()
    secs = parse_pe(d)
    secmap = {s['name']: s for s in secs}
    text = secmap['.text']
    t_lo, t_hi = text['rva'], text['rva'] + text['vsz']
    tv_lo, tv_hi = IB + t_lo, IB + t_hi

    def fo(rva):
        for s in secs:
            if s['rva'] <= rva < s['rva'] + max(s['vsz'], s['rsz']):
                return s['ra'] + (rva - s['rva'])
        return None

    def sec_of(rva):
        for s in secs:
            if s['rva'] <= rva < s['rva'] + max(s['vsz'], s['rsz']):
                return s['name']
        return None

    def rd_u64(va):
        o = fo(va - IB)
        if o is None or o + 8 > len(d):
            return None
        return struct.unpack_from('<Q', d, o)[0]

    def rd_u32(rva):
        o = fo(rva)
        if o is None or o + 4 > len(d):
            return None
        return struct.unpack_from('<I', d, o)[0]

    # ---------------- pdata function ranges ----------------
    pdata = secmap['.pdata']
    ranges = []
    for i in range(pdata['vsz'] // 12):
        b, e, u = struct.unpack_from('<III', d, pdata['ra'] + i * 12)
        if b < e and t_lo <= b < t_hi:
            ranges.append((b, min(e, t_hi)))
    ranges.sort()
    begins = set(b for b, _ in ranges)
    # site -> function range lookup
    def func_of(site_rva):
        lo, hi = 0, len(ranges) - 1
        while lo <= hi:
            m = (lo + hi) // 2
            b, e = ranges[m]
            if b <= site_rva < e:
                return m
            if site_rva < b:
                hi = m - 1
            else:
                lo = m + 1
        return None

    # ---------------- import name map ----------------
    def rd(rva, n):
        o = fo(rva)
        return d[o:o + n] if o is not None else b''
    imp_names = {}
    desc_rva = 0x3191200
    i = 0
    while True:
        ofthunk, ts, dmy, nm, ft = struct.unpack_from('<IIIII', rd(desc_rva + i * 20, 20), 0)
        if ofthunk == 0 and nm == 0 and ft == 0:
            break
        tr = ofthunk or ft
        j = 0
        while True:
            t = struct.unpack_from('<Q', rd(tr + j * 8, 8), 0)[0]
            if t == 0:
                break
            if not (t >> 63):
                s = rd(t, 2 + 64)
                imp_names[t] = s[2:].split(b'\x00')[0].decode('latin1')
            j += 1
        i += 1

    # ---------------- load indirect raw ----------------
    raw = json.load(io.open(os.path.join(outdir, 'indirect_raw.json')))
    classes = raw['classes']

    # ---------- 1. classify rip sites ----------
    rip_resolved = []
    rip_stats = {'import': 0, 'text': 0, 'data_slot': 0, 'zero': 0, 'other': 0}
    rip_internal_edges = []
    for x in classes['rip']:
        p = x.get('ptr')
        rec = dict(x)
        if p is None:
            rip_stats['other'] += 1
            rip_resolved.append(rec)
            continue
        pv = int(p, 16)
        if pv == 0:
            rip_stats['zero'] += 1
            rec['kind'] = 'unresolved_zero'
        elif pv in imp_names:
            rip_stats['import'] += 1
            rec['kind'] = 'import'
            rec['import_name'] = imp_names[pv]
        elif tv_lo <= pv < tv_hi:
            rip_stats['text'] += 1
            rec['kind'] = 'internal'
            rip_internal_edges.append({'site': x['site'], 'slot': x['slot'],
                                       'target': hex(pv), 'via': 'call [rip+disp]'})
        else:
            s = sec_of(pv - IB) if pv >= IB else None
            if s:
                rip_stats['data_slot'] += 1
                rec['kind'] = 'data_ptr_in_' + s
            else:
                rip_stats['other'] += 1
                rec['kind'] = 'unresolved_other'
        rip_resolved.append(rec)

    # ---------- 2. resolve call [reg+disp] and call reg via backward scan ----------
    from capstone import Cs, CS_ARCH_X86, CS_MODE_64
    from capstone.x86 import X86_OP_REG, X86_OP_MEM, X86_OP_IMM, X86_REG_RIP
    mdd = Cs(CS_ARCH_X86, CS_MODE_64)
    mdd.detail = True

    CF = set()
    for mn in ('jmp', 'call', 'ret', 'jne', 'je', 'jg', 'jl', 'jge', 'jle', 'ja',
               'jb', 'jae', 'jbe', 'js', 'jns', 'jp', 'jnp', 'jo', 'jno',
               'loop', 'jrcxz', 'jecxz'):
        CF.add(mn)

    def decode_func(rng_idx):
        b, e = ranges[rng_idx]
        o = fo(b)
        insns = list(mdd.disasm(d[o:o + (e - b)], IB + b))
        return insns

    def def_of_reg(insns, idx, reg, depth=0):
        """nearest straight-line definition of reg before insns[idx]."""
        if depth > 3:
            return None
        for j in range(idx - 1, max(-1, idx - 90), -1):
            ins = insns[j]
            if ins.mnemonic in CF:
                return None
            ops = ins.operands
            if not ops or ops[0].type != X86_OP_REG or ops[0].reg != reg:
                continue
            mn = ins.mnemonic
            # lea reg,[rip+X]  -> address of X
            if mn == 'lea' and len(ops) > 1 and ops[1].type == X86_OP_MEM and ops[1].mem.base == X86_REG_RIP:
                return ('addr', ins.address + ins.size + ops[1].mem.disp)
            # mov reg,[rip+X] -> pointer value stored at X
            if mn == 'mov' and len(ops) > 1 and ops[1].type == X86_OP_MEM and ops[1].mem.base == X86_REG_RIP:
                return ('mem', ins.address + ins.size + ops[1].mem.disp)
            # mov reg,[base+disp]
            if mn == 'mov' and len(ops) > 1 and ops[1].type == X86_OP_MEM and ops[1].mem.base != 0:
                mm = ops[1].mem
                base = def_of_reg(insns, j, mm.base, depth + 1)
                if base and base[0] == 'addr':
                    return ('mem', base[1] + mm.disp)
                if base and base[0] == 'mem':
                    return ('mem', base[1])  # loaded value, unknown slot
                return None
            # mov reg, reg2 (alias)
            if mn == 'mov' and len(ops) > 1 and ops[1].type == X86_OP_REG:
                sub = def_of_reg(insns, j, ops[1].reg, depth + 1)
                if sub:
                    return sub
                return None
            if mn == 'mov' and len(ops) > 1 and ops[1].type == X86_OP_IMM:
                return ('imm', ops[1].imm)
            return None
        return None

    reg_edges = []
    reg_stats = {'reg_disp_total': 0, 'reg_disp_resolved': 0,
                 'reg_total': 0, 'reg_resolved': 0,
                 'unresolved_no_def': 0, 'unresolved_def_not_vtable': 0,
                 'unresolved_slot_not_text': 0, 'unresolved_slot_empty': 0}
    cache = {}
    all_indirect = []
    for kind in ('reg_disp', 'reg', 'reg_index'):
        for x in classes[kind]:
            all_indirect.append((kind, x))

    for kind, x in all_indirect:
        site = int(x['site'], 16)
        fi = func_of(site - IB)
        if fi is None:
            continue
        if fi not in cache:
            cache[fi] = decode_func(fi)
        insns = cache[fi]
        # find this instruction index
        idx = None
        for j, ins in enumerate(insns):
            if ins.address == site:
                idx = j
                break
        if idx is None:
            continue
        ins = insns[idx]
        op = ins.operands[0]
        if kind == 'reg':
            reg_stats['reg_total'] += 1
        else:
            reg_stats['reg_disp_total'] += 1
        tgt_slot = None
        if op.type == X86_OP_REG:
            dv = def_of_reg(insns, idx, op.reg)
            # if def is ('mem',S): value = *(S) -> if *(S) is a vtable addr we'd need another hop;
            # here just try direct deref of the pointer slot value
            if dv is None:
                reg_stats['unresolved_no_def'] += 1
                continue
            if dv[0] == 'mem':
                tgt_slot = dv[1]
                tgt = rd_u64(dv[1])
                if tgt is None:
                    reg_stats['unresolved_slot_empty'] += 1
                    continue
                # tgt might itself be a vtable; call reg targets *(S) directly
                if tv_lo <= tgt < tv_hi:
                    reg_stats['reg_resolved'] += 1
                    reg_edges.append({'site': x['site'], 'target': hex(tgt),
                                      'via': 'call reg <- [slot]', 'slot': hex(dv[1])})
                else:
                    reg_stats['unresolved_slot_not_text'] += 1
            elif dv[0] == 'addr':
                tgt = rd_u64(dv[1])
                if tgt and tv_lo <= tgt < tv_hi:
                    reg_stats['reg_resolved'] += 1
                    reg_edges.append({'site': x['site'], 'target': hex(tgt),
                                      'via': 'call reg <- lea+[slot]', 'slot': hex(dv[1])})
                else:
                    reg_stats['unresolved_slot_not_text'] += 1
            else:
                reg_stats['unresolved_def_not_vtable'] += 1
        else:  # reg_disp
            mm = op.mem
            dv = def_of_reg(insns, idx, mm.base)
            if dv is None:
                reg_stats['unresolved_no_def'] += 1
                continue
            slot = None
            if dv[0] == 'addr':
                slot = dv[1] + mm.disp
            elif dv[0] == 'mem':
                P = rd_u64(dv[1])
                if P is not None and P >= IB:
                    slot = P + mm.disp   # reg = *(global ptr) = vtable base
            if slot is None:
                reg_stats['unresolved_def_not_vtable'] += 1
                continue
            tgt = rd_u64(slot)
            if tgt is None:
                reg_stats['unresolved_slot_empty'] += 1
                continue
            if tv_lo <= tgt < tv_hi:
                reg_stats['reg_disp_resolved'] += 1
                reg_edges.append({'site': x['site'], 'target': hex(tgt),
                                  'via': 'call [reg+disp]',
                                  'slot': hex(slot)})
            else:
                reg_stats['unresolved_slot_not_text'] += 1

    # ---------- 3. vtable / function-pointer table scan ----------
    def scan_tables():
        out = {'u32_rva_runs': [], 'u64_va_runs': []}
        for s in secs:
            if s['name'] not in ('.rdata', '.data'):
                continue
            ra, rva, vsz = s['ra'], s['rva'], s['vsz']
            # u32 RVA runs
            n = vsz // 4
            vals = struct.unpack_from('<%dI' % n, d, ra)
            i = 0
            while i < n:
                v = vals[i]
                if t_lo <= v < t_hi:
                    j = i
                    while j < n and t_lo <= vals[j] < t_hi:
                        j += 1
                    if j - i >= 2:
                        out['u32_rva_runs'].append({
                            'section': s['name'], 'run_va': hex(IB + rva + i * 4),
                            'slots': j - i,
                            'targets_are_func_begin': sum(
                                1 for k in range(i, j) if vals[k] in begins),
                            'sample_targets': [hex(IB + vals[k]) for k in range(i, min(j, i + 4))],
                        })
                    i = j
                else:
                    i += 1
            # u64 VA runs
            n8 = vsz // 8
            vals8 = struct.unpack_from('<%dQ' % n8, d, ra)
            i = 0
            while i < n8:
                v = vals8[i]
                if tv_lo <= v < tv_hi:
                    j = i
                    while j < n8 and tv_lo <= vals8[j] < tv_hi:
                        j += 1
                    if j - i >= 2:
                        out['u64_va_runs'].append({
                            'section': s['name'], 'run_va': hex(IB + rva + i * 8),
                            'slots': j - i,
                            'targets_are_func_begin': sum(
                                1 for k in range(i, j) if (vals8[k] - IB) in begins),
                            'sample_targets': [hex(vals8[k]) for k in range(i, min(j, i + 4))],
                        })
                    i = j
                else:
                    i += 1
        return out

    vt = scan_tables()

    # ---------- 4. symbol names ----------
    names = {}
    wg = json.load(io.open(os.path.join(root, 'evidence/render-recovery/workgraph/functions.json'), encoding='utf-8'))
    for f in wg:
        names[int(f['va'], 16)] = f['names'][0]
    try:
        ps = json.load(io.open(os.path.join(root, 'evidence/render-architecture/public-symbols.json'), encoding='utf-8'))
        secrva = [0] + [s['rva'] for s in secs]
        for x in ps:
            sec = x.get('section')
            off = x.get('offset')
            if not sec or off is None:
                continue
            try:
                va = IB + secrva[sec] + off
            except Exception:
                continue
            nm = x.get('demangled') or x.get('symbol')
            if nm and va not in names:
                names[va] = nm
    except Exception as e:
        print('public-symbols load warn', e)

    def nm(va):
        return names.get(va)

    # ---------- 5b. filtered vtables ----------
    def build_vtables():
        vt_rows = []
        for r in vt['u64_va_runs']:
            run_va = int(r['run_va'], 16)
            slots = r['slots']
            tgt_ok = r['targets_are_func_begin']
            # read slots
            targets = []
            for k in range(slots):
                v = rd_u64(run_va + k * 8)
                targets.append(hex(v) if v is not None else None)
            frac = tgt_ok / slots if slots else 0
            distinct = len(set(targets))
            base = run_va - 0x18          # Rust dyn vtable header: [drop,size,align]
            drop = rd_u64(base)
            size = rd_u64(base + 8)
            align = rd_u64(base + 16)
            vt_rows.append({
                'method_array_va': r['run_va'], 'vtable_base': hex(base),
                'section': r['section'], 'slots': slots,
                'func_begin_frac': round(frac, 3),
                'distinct_targets': distinct,
                'diversity': round(distinct / slots, 2) if slots else 0,
                'header_drop_in_place': hex(drop) if drop else None,
                'header_drop_is_text': bool(drop and tv_lo <= drop < tv_hi),
                'header_size': size, 'header_align': align,
                'targets': targets,
                'target_names': [nm(int(t, 16)) if t else None for t in targets],
            })
        return vt_rows
    vtables = build_vtables()
    # high-confidence: >=3 slots and mostly function entries
    hc = [v for v in vtables if v['slots'] >= 3 and v['func_begin_frac'] >= 0.6]

    # ---------- 6. cross-validation ----------
    xcheck = {}
    known = {
        'immediate_empty_bucket': 0x140bf92f0,
        'begin_frame': 0x140beabd0,
        'queue_submit': 0x140c04e20,
    }
    # which known fns appear as slots of any vtable?
    slot_index = {}
    for v in vtables:
        for t in v['targets']:
            if t:
                slot_index.setdefault(int(t, 16), []).append(v['vtable_base'])
    for name, va in known.items():
        xcheck[name] = {'va': hex(va), 'as_vtable_slot_in': slot_index.get(va, [])}
    # u32 region check 0x1436A55xx and 0x143310b40
    def region_info(rva, nbytes=32):
        return {'rva': hex(rva), 'va': hex(IB + rva), 'section': sec_of(rva),
                'u32_hex': ' '.join('%08x' % v for v in struct.unpack_from('<%dI' % (nbytes // 4), d, fo(rva)))}
    region_checks = {
        '0x1436A55xx': region_info(0x36A55C8),
        'rdata_table_0x143310b40': region_info(0x3310b40),
    }
    # rhapsody (functions.json 450) functions that ARE vtable slots
    rhap_names = {}
    wg0 = json.load(io.open(os.path.join(root, 'evidence/render-recovery/workgraph/functions.json'), encoding='utf-8'))
    for f in wg0:
        rhap_names[int(f['va'], 16)] = f['names'][0]
    rhap_slots = {}
    for va, vl in slot_index.items():
        if va in rhap_names:
            rhap_slots[hex(va)] = {'name': rhap_names[va], 'vtables': vl}

    # ---------- 7. emit ----------
    os.makedirs(outdir, exist_ok=True)
    json.dump({'rip_sites': rip_resolved, 'rip_stats': rip_stats,
               'reg_edges': reg_edges, 'reg_stats': reg_stats},
              io.open(os.path.join(outdir, 'indirect_resolution.json'), 'w'), indent=1)
    json.dump(vt, io.open(os.path.join(outdir, 'vtables_raw.json'), 'w'), indent=1)
    json.dump({'vtables_all': vtables, 'vtables_high_conf': hc,
               'rhapsody_functions_used_as_slots': rhap_slots,
               'region_checks': region_checks},
              io.open(os.path.join(outdir, 'vtables.json'), 'w'), indent=1)

    # calls.json-compatible callgraph
    def src_of(site_va):
        fi = func_of(site_va - IB)
        if fi is None:
            return None, None
        sva = IB + ranges[fi][0]
        return hex(sva), nm(sva)
    edges = []
    fmap = {}
    for e in reg_edges:
        tv = int(e['target'], 16)
        sva, snm = src_of(int(e['site'], 16))
        edges.append({'source': sva, 'source_names': [snm], 'target': e['target'],
                      'target_names': [nm(tv)], 'kind': 'indirect_call',
                      'via': e['via'], 'site': e['site'], 'slot': e.get('slot')})
        if sva:
            fe = fmap.setdefault(sva, {'names': [snm], 'indirect_calls': []})
            fe['indirect_calls'].append({'site': e['site'], 'target': e['target'],
                                         'slot': e.get('slot'), 'via': e['via']})
    for e in rip_internal_edges:
        tv = int(e['target'], 16)
        sva, snm = src_of(int(e['site'], 16))
        edges.append({'source': sva, 'source_names': [snm], 'target': e['target'],
                      'target_names': [nm(tv)], 'kind': 'indirect_call',
                      'via': e['via'], 'site': e['site'], 'slot': e.get('slot')})
        if sva:
            fe = fmap.setdefault(sva, {'names': [snm], 'indirect_calls': []})
            fe['indirect_calls'].append({'site': e['site'], 'target': e['target'],
                                         'slot': e.get('slot'), 'via': e['via']})

    summary = {
        'rip_total': len(classes['rip']),
        'rip_stats': rip_stats,
        'reg_stats': reg_stats,
        'internal_indirect_edges': len(reg_edges) + len(rip_internal_edges),
        'u32_rva_table_runs': len(vt['u32_rva_runs']),
        'u32_rva_total_slots': sum(r['slots'] for r in vt['u32_rva_runs']),
        'u64_va_table_runs': len(vt['u64_va_runs']),
        'u64_va_total_slots': sum(r['slots'] for r in vt['u64_va_runs']),
        'vtables_all_runs': len(vtables),
        'vtables_high_conf': len(hc),
        'hc_vtable_like_diversity_ge_06': sum(1 for v in hc if v['diversity'] >= 0.6),
        'hc_jumptable_like_diversity_le_03': sum(1 for v in hc if v['diversity'] <= 0.3),
        'hc_rvt_header_confirmed': sum(1 for v in hc
                                       if v['header_drop_is_text']
                                       and isinstance(v['header_size'], int)
                                       and 0 < v['header_size'] < 0x100000
                                       and v['header_align'] in (1, 2, 4, 8, 16)),
        'rhap_functions_as_vtable_slots': len(rhap_slots),
    }
    json.dump({'summary': summary, 'functions': fmap, 'edges': edges},
              io.open(os.path.join(outdir, 'indirect-callgraph.json'), 'w'), indent=1)
    print(json.dumps(summary, indent=1))
    print('region 0x1436A55xx', region_checks['0x1436A55xx'])
    print('region 0x143310b40', region_checks['rdata_table_0x143310b40'])
    print('known cross-check', json.dumps(xcheck, indent=1))


if __name__ == '__main__':
    main()
