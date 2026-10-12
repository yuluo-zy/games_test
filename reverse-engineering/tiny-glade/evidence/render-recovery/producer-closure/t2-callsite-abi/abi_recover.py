#!/usr/bin/env python3
# Read-only forensic tool (t2-callsite-abi).
# Recover MSVC x64 *machine-argument* shape of rhapsody functions from their
# call sites, using only the shipped PE bytes + workgraph edges.
#
# For each target VA: find all incoming direct call sites (from calls.json
# edges), disassemble the CALLER from the PE, locate the `call` instruction,
# then backward-slice the register/stack writes that feed RCX/RDX/R8/R9
# (and XMM0-3, and [rsp+disp] stores) to bound the number of arguments and
# their sources.
#
# Usage:
#   python -I abi_recover.py <exe> <repo_root> <out_json> [--all | --targets va,va,...]
import struct, sys, json, io, os, collections
import site as _site
_usp = _site.getusersitepackages()
if _usp and _usp not in sys.path:
    sys.path.insert(0, _usp)

IMAGEBASE = 0x140000000
ARGREGS = ['rcx', 'rdx', 'r8', 'r9']
XMMREGS = ['xmm0', 'xmm1', 'xmm2', 'xmm3']
HOOKS = ['0x140530430', '0x1409dca90', '0x140bebdf0', '0x140bf0b90', '0x1409dd4d0']


def load_sections(d):
    e = struct.unpack_from('<I', d, 0x3c)[0]
    nsec = struct.unpack_from('<H', d, e + 6)[0]
    optsz = struct.unpack_from('<H', d, e + 20)[0]
    secs = []
    off = e + 24 + optsz
    for _ in range(nsec):
        nm = d[off:off+8].rstrip(b'\x00').decode('latin1')
        vsz, va, rsz, ra = struct.unpack_from('<IIII', d, off + 8)
        ch = struct.unpack_from('<I', d, off + 36)[0]
        secs.append((nm, va, vsz, ra, rsz, ch))
        off += 40
    return secs


def data_dirs(d):
    e = struct.unpack_from('<I', d, 0x3c)[0]
    # optional header starts at e+24; number of data dirs at opt+108 (PE32+)
    opt = e + 24
    ndd = struct.unpack_from('<I', d, opt + 108)[0]
    dd = []
    base = opt + 112
    for i in range(ndd):
        rva, sz = struct.unpack_from('<II', d, base + i * 8)
        dd.append((rva, sz))
    return dd


def parse_pdata(d, secs, off_of):
    """Return sorted list of (begin_rva, end_rva) from the x64 exception directory."""
    dd = data_dirs(d)
    if len(dd) <= 3:
        return []
    rva, sz = dd[3]
    if rva == 0 or sz == 0:
        return []
    fo = off_of(rva)
    if fo is None:
        return []
    out = []
    for i in range(0, sz, 12):
        b, e2, u = struct.unpack_from('<III', d, fo + i)
        if b == 0:
            continue
        out.append((b, e2))
    out.sort()
    return out


def caller_start(ranges, site_rva):
    # ranges are RVA based; site passed as RVA
    import bisect
    starts = [r[0] for r in ranges]
    i = bisect.bisect_right(starts, site_rva) - 1
    if i >= 0 and ranges[i][0] <= site_rva < ranges[i][1]:
        return ranges[i][0], ranges[i][1]
    return None, None


def scan_call_sites(d, secs, off_of, target_set):
    """Find every `E8 rel32` in executable sections whose target is in target_set.
    Returns dict target_va -> [(site_va, caller_va_hint_or_None)]."""
    res = collections.defaultdict(list)
    for nm, va, vsz, ra, rsz, ch in secs:
        if not (ch & 0x20000000):   # IMAGE_SCN_MEM_EXECUTE
            continue
        buf = d[ra:ra + rsz]
        pos = 0
        while True:
            p = buf.find(b'\xe8', pos)
            if p < 0:
                break
            pos = p + 1
            if p + 5 > len(buf):
                break
            rel = struct.unpack_from('<i', buf, p + 1)[0]
            site_rva = va + p
            tgt = (site_rva + 5 + rel)
            if tgt in target_set:
                res[tgt].append((site_rva + IMAGEBASE, None))
    return res



def make_off_of(secs):
    def off_of(rva):
        for nm, va, vsz, ra, rsz, ch in secs:
            if va <= rva < va + max(vsz, rsz):
                return ra + (rva - va)
        return None
    return off_of


def build_md():
    from capstone import Cs, CS_ARCH_X86, CS_MODE_64
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    md.detail = True
    return md


def reg_name(md, ins, reg):
    return ins.reg_name(reg)


def backward_slice(md, insns, call_idx, fn_start):
    """Return dict describing the arg-register defs feeding the call at call_idx."""
    md_ = md
    from capstone import CS_ARCH_X86, CS_AC_WRITE
    from capstone.x86 import X86_OP_REG, X86_OP_MEM, X86_OP_IMM, X86_REG_RSP, X86_REG_RBP
    GA = [0]  # placeholder
    defs = {}          # reg -> dict(source=..., addr=...)
    stack_stores = []  # (disp, src)
    xmm_defs = {}
    barrier = None
    lo = max(0, call_idx - 200)
    i = call_idx - 1
    while i >= lo:
        ins = insns[i]
        mn = ins.mnemonic
        if ins.address < fn_start:
            break
        if mn in ('ret', 'retn', 'jmp', 'ljmp'):
            barrier = (hex(ins.address), mn)
            break
        # stop at a prior call: caller-saved arg regs are clobbered there
        if mn == 'call' or mn.startswith('call'):
            barrier = (hex(ins.address), 'call')
            break
        # conditional branch -> control-flow merge upstream, unreliable
        if mn.startswith('j') and mn not in ('jmp',):
            barrier_soft = (hex(ins.address), mn)
            # don't hard-stop; note it
            barrier = barrier or barrier_soft
        ops = ins.operands
        if ops:
            d0 = ops[0]
            if d0.type == X86_OP_REG and (d0.access & CS_AC_WRITE):
                rn = ins.reg_name(d0.reg)
                if rn in ARGREGS and rn not in defs:
                    defs[rn] = {'source': _src_desc(md, ins), 'addr': hex(ins.address)}
                elif rn in XMMREGS and rn not in xmm_defs:
                    xmm_defs[rn] = {'source': _src_desc(md, ins), 'addr': hex(ins.address)}
                elif d0.type == X86_OP_MEM:
                    pass
            # stack store: mov [rsp/rbp+disp], src
            if d0.type == X86_OP_MEM and (d0.access & CS_AC_WRITE):
                base = d0.mem.base
                if base in (X86_REG_RSP, X86_REG_RBP):
                    s = _src_desc(md, ins, src_index=1)
                    stack_stores.append({'base': 'rsp' if base == X86_REG_RSP else 'rbp',
                                         'disp': d0.mem.disp, 'src': s, 'addr': hex(ins.address)})
        i -= 1
    return defs, xmm_defs, stack_stores, barrier


def _src_desc(md, ins, src_index=1):
    from capstone.x86 import X86_OP_REG, X86_OP_MEM, X86_OP_IMM, X86_REG_RIP
    ops = ins.operands
    if len(ops) <= src_index:
        return 'unary/' + ins.mnemonic
    s = ops[src_index]
    if s.type == X86_OP_REG:
        return 'reg:' + ins.reg_name(s.reg)
    if s.type == X86_OP_IMM:
        return 'imm:0x%x' % (s.imm & 0xffffffffffffffff)
    if s.type == X86_OP_MEM:
        b = ins.reg_name(s.mem.base) if s.mem.base else ''
        if s.mem.base == X86_REG_RIP:
            tgt = ins.address + ins.size + s.mem.disp
            return 'imm:0x%x(rip)' % tgt
        return 'mem:[%s%+d]' % (b, s.mem.disp)
    return '?'


def analyze_target(target, sites, ranges, fmap, off_of, d, md):
    per = []
    for site_va, _ in sites:
        sva = int(site_va) if isinstance(site_va, int) else int(site_va, 16)
        site_rva = sva - IMAGEBASE
        begin, end = caller_start(ranges, site_rva)
        if begin is None:
            # fall back to the 450-function map
            for va, info in fmap.items():
                iv = int(va, 16)
                if iv <= sva < iv + info['code_size']:
                    begin, end = iv - IMAGEBASE, iv + info['code_size'] - IMAGEBASE
                    break
        if begin is None:
            per.append({'site': hex(sva), 'error': 'no containing function (not in .pdata)'})
            continue
        cva = begin + IMAGEBASE
        size = end - begin
        fo = off_of(begin)
        if fo is None:
            per.append({'site': hex(sva), 'error': 'no section for caller'})
            continue
        raw = d[fo:fo + size]
        insns = list(md.disasm(raw, cva))
        cidx = None
        for k, ins in enumerate(insns):
            if ins.address == sva and ins.mnemonic.startswith('call'):
                cidx = k
                break
        if cidx is None:
            for k, ins in enumerate(insns):
                if abs(ins.address - sva) <= 6 and ins.mnemonic.startswith('call'):
                    cidx = k
                    break
        caller_info = fmap.get(hex(cva))
        cname = caller_info['names'][0] if caller_info else None
        if cidx is None:
            per.append({'site': hex(sva), 'caller': hex(cva), 'caller_name': cname,
                        'error': 'call instruction not found'})
            continue
        defs, xmm, stk, barrier = backward_slice(md, insns, cidx, cva)
        observed = [r for r in ARGREGS if r in defs]
        prefix = 0
        for r in ARGREGS:
            if r in defs:
                prefix += 1
            else:
                break
        gap = (prefix != len(observed))
        # MSVC x64: args 5+ live at [rsp+0x20], [rsp+0x28], ... ([rsp+disp] only;
        # rbp-relative stores are frame locals, not arguments).
        stack_args = [s for s in stk if s['base'] == 'rsp' and 0x20 <= s['disp'] < 0x400][:12]
        # confidence: complete prefix, and any barrier is further back than the
        # earliest argument definition (so it does not invalidate the values).
        conf = 'low'
        if observed and not gap and prefix >= 1:
            min_def = min(int(v['addr'], 16) for v in defs.values())
            if barrier is None:
                conf = 'high'
            else:
                bar_addr = int(barrier[0], 16)
                conf = 'high' if bar_addr < min_def else 'medium'
        elif observed:
            conf = 'medium'
        per.append({
            'site': hex(sva), 'caller': hex(cva), 'caller_name': cname,
            'arg_regs_observed': observed, 'prefix_count': prefix, 'gap': gap,
            'defs': defs, 'xmm_observed': list(xmm.keys()),
            'stack_arg_stores': stack_args, 'barrier': barrier, 'confidence': conf,
        })
    return per


def main():
    exe, root, outj = sys.argv[1], sys.argv[2], sys.argv[3]
    rest = sys.argv[4:]
    targets = HOOKS
    if '--all' in rest:
        targets = None
    elif '--targets' in rest:
        targets = rest[rest.index('--targets') + 1].split(',')
    d = open(exe, 'rb').read()
    secs = load_sections(d)
    off_of = make_off_of(secs)
    md = build_md()
    funcs = json.load(io.open(os.path.join(root, 'evidence/render-recovery/workgraph/functions.json'), encoding='utf-8'))
    fmap = {}
    for f in funcs:
        fmap[f['va']] = {'code_size': f['code_size'], 'names': f.get('names', []),
                         'va': f['va'], 'modules': f.get('modules')}
    calls = json.load(io.open(os.path.join(root, 'evidence/render-recovery/workgraph/calls.json'), encoding='utf-8'))
    edges_by_target = collections.defaultdict(list)
    # per-function direct lists carry {site,target} (call-site address). We ALSO do
    # a full .text scan below so callers outside the tracked 450 are not missed.
    for srcva, blk in calls['functions'].items():
        for de in blk.get('direct', []):
            edges_by_target[de['target']].append((de['site'], srcva))

    ranges = parse_pdata(d, secs, off_of)
    if targets is None:
        targets = [f['va'] for f in funcs]
    targets = list(dict.fromkeys(targets))
    tset = set(int(t, 16) for t in targets)
    tset |= set(int(k, 16) for k in edges_by_target.keys())
    scanned = scan_call_sites(d, secs, off_of, tset)
    # union: workgraph edge sites + full-text scan sites
    def sites_for(t):
        s = set()
        for site, _hint in edges_by_target.get(t, []):
            s.add(int(site, 16))
        for site, _h in scanned.get(int(t, 16), []):
            s.add(site)
        return sorted(s)

    out = {'exe': exe, 'targets_analyzed': len(targets), 'pdata_functions': len(ranges),
           'functions': {}}
    n_with_site = 0
    n_high = 0
    n_resolved = 0
    for t in targets:
        sites = [(s, None) for s in sites_for(t)]
        per = analyze_target(t, sites, ranges, fmap, off_of, d, md)
        resolvable = [p for p in per if 'defs' in p]
        if per:
            n_with_site += 1
        if resolvable:
            n_resolved += 1
        # consensus: max prefix count; sources per position from the best call site
        best = None
        for p in resolvable:
            if best is None or p['prefix_count'] > best['prefix_count']:
                best = p
        consensus = None
        if best:
            sources = {}
            for r in ARGREGS[:best['prefix_count']] if not best['gap'] else best['arg_regs_observed']:
                sources[r] = best['defs'][r]['source']
            consensus = {'observed_arg_regs': best['arg_regs_observed'],
                         'prefix_count': best['prefix_count'],
                         'sources': sources,
                         'stack_arg_stores': best['stack_arg_stores'],
                         'confidence': best['confidence'],
                         'from_site': best['site']}
            if best['confidence'] == 'high':
                n_high += 1
        info = fmap.get(t)
        out['functions'][t] = {
            'va': t,
            'name': info['names'][0] if info else None,
            'modules': info['modules'] if info else None,
            'n_incoming_direct_callsites': len(per),
            'n_resolved_callsites': len(resolvable),
            'consensus': consensus,
            'callsites': per,
        }
    N = len(targets)
    out['coverage'] = {
        'targets': N,
        'targets_with_incoming_callsite': n_with_site,
        'targets_with_resolved_callsite': n_resolved,
        'targets_high_confidence': n_high,
        'targets_no_incoming_callsite': N - n_with_site,
        'targets_undecidable': N - n_resolved,
    }
    json.dump(out, io.open(outj, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
    print('analyzed', N, 'coverage', out['coverage'])
    print('wrote', outj)


if __name__ == '__main__':
    main()
