#!/usr/bin/env python3
# Read-only forensic tool for Tiny Glade RE.
# Classifies indirect call instructions across the whole image and resolves
# rip-relative indirect call targets, using PE .pdata to bound function bodies.
#
# Usage: python -I analyze_indirect.py <exe> <repo_root> <out_dir>
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


def sec_of_rva(secs, rva):
    for s in secs:
        if s['rva'] <= rva < s['rva'] + max(s['vsz'], s['rsz']):
            return s
    return None


def rva_to_off(secs, rva):
    s = sec_of_rva(secs, rva)
    if not s:
        return None
    return s['ra'] + (rva - s['rva'])


def main():
    exe, root, outdir = sys.argv[1], sys.argv[2], sys.argv[3]
    d = open(exe, 'rb').read()
    secs = parse_pe(d)
    secmap = {s['name']: s for s in secs}
    text = secmap['.text']
    text_lo = text['rva']
    text_hi = text['rva'] + text['vsz']
    text_va_lo = IB + text_lo
    text_va_hi = IB + text_hi

    # ---- .pdata -> function ranges (RUNTIME_FUNCTION: Begin/End/Unwind RVAs) ----
    pdata = secmap['.pdata']
    pranges = []
    pb = pdata['ra']
    n = pdata['vsz'] // 12
    for i in range(n):
        b, e, u = struct.unpack_from('<III', d, pb + i * 12)
        if b < e and text_lo <= b < text_hi:
            pranges.append((b, min(e, text_hi)))
    func_begins = set()
    for b, e in pranges:
        func_begins.add(b)
    cov = sum(e - b for b, e in pranges)
    print('pdata funcs %d, code bytes covered %d (text vsz %d, ratio %.3f)'
          % (len(pranges), cov, text['vsz'], cov / text['vsz']))

    from capstone import Cs, CS_ARCH_X86, CS_MODE_64
    from capstone.x86 import (X86_OP_MEM, X86_OP_IMM, X86_OP_REG,
                              X86_REG_RIP, X86_REG_RBP, X86_REG_ESP, X86_REG_RSP,
                              X86_REG_EBP, X86_REG_EIP)
    md = Cs(CS_ARCH_X86, CS_MODE_64)          # no detail: fast pass to find call/jmp sites
    mdd = Cs(CS_ARCH_X86, CS_MODE_64)
    mdd.detail = True                          # detail: only used on call/jmp bytes

    # Every byte offset -> whether it starts a pdata function (for vtable validation)
    begin_rvas = func_begins

    # ---- indirect call / jump classification (only 0xFF-encoded forms) ----
    # FF /2 = call r/m64   FF /3 = call m16:64 (far)
    # FF /4 = jmp  r/m64   FF /5 = jmp  m16:64 (far)
    classes = {'rip': [], 'reg_disp': [], 'reg': [], 'reg_index': [],
               'abs': [], 'far': []}
    jmp_classes = {'rip': [], 'reg_disp': [], 'reg': [], 'reg_index': [], 'abs': [], 'far': []}
    e8_direct = 0
    ff2 = ff3 = ff4 = ff5 = 0
    other_ff = 0
    decoded_bytes = 0
    decoded_insns = 0
    call_sites = []   # (insn_va, bytes)

    # memory read helper
    def read_u64(va):
        o = rva_to_off(secs, va - IB)
        if o is None or o + 8 > len(d):
            return None
        return struct.unpack_from('<Q', d, o)[0]

    for b, e in pranges:
        o = rva_to_off(secs, b)
        if o is None:
            continue
        raw = d[o:o + (e - b)]
        decoded_bytes += len(raw)
        for ins in md.disasm(raw, IB + b):
            decoded_insns += 1
            m = ins.mnemonic
            if m in ('call', 'jmp', 'lcall', 'ljmp'):
                call_sites.append((ins.address, ins.bytes))

    print('decoded %d insns / %d bytes; call/jmp candidates %d'
          % (decoded_insns, decoded_bytes, len(call_sites)))

    disp_hist = {}
    base_hist = {}
    direct_imm = 0
    for va, by in call_sites:
        for ins in mdd.disasm(by, va):
            ops = ins.operands
            if not ops:
                break
            op = ops[0]
            mn = ins.mnemonic
            is_jmp = mn in ('jmp', 'ljmp')
            is_far = mn in ('lcall', 'ljmp')
            if op.type == X86_OP_IMM:
                # E8/E9/EB relative direct transfer
                direct_imm += 1
                break
            bucket = jmp_classes if is_jmp else classes
            kind = None
            if op.type == X86_OP_REG:
                kind = 'reg'
            elif op.type == X86_OP_MEM:
                mm = op.mem
                if mm.base == X86_REG_RIP:
                    kind = 'rip'
                elif mm.index != 0:
                    kind = 'reg_index'
                elif mm.base != 0:
                    kind = 'reg_disp'
                else:
                    kind = 'abs'
            if kind is None:
                break
            if is_far:
                kind = 'far'
            if kind in ('reg_disp', 'reg_index'):
                key = 'disp=%#x' % op.mem.disp
                disp_hist[key] = disp_hist.get(key, 0) + 1
                base_hist[hex(op.mem.base)] = base_hist.get(hex(op.mem.base), 0) + 1
            bucket[kind].append({'site': hex(va), 'op': ins.op_str,
                                 'bytes': by.hex()})
            if not is_jmp and kind == 'rip':
                slot = ins.address + ins.size + op.mem.disp
                tgt = read_u64(slot)
                if tgt is not None:
                    bucket['rip'][-1]['slot'] = hex(slot)
                    bucket['rip'][-1]['ptr'] = hex(tgt)
                    s = sec_of_rva(secs, tgt - IB) if tgt >= IB else None
                    bucket['rip'][-1]['ptr_section'] = s['name'] if s else (hex(tgt))
                    if text_va_lo <= tgt < text_va_hi:
                        bucket['rip'][-1]['target_text'] = hex(tgt)
                        bucket['rip'][-1]['target_is_func_begin'] = (tgt - IB) in begin_rvas
            break

    ff2 = len(classes['rip']) + len(classes['reg_disp']) + len(classes['reg']) + \
        len(classes['reg_index']) + len(classes['abs'])
    ff4 = len(jmp_classes['rip']) + len(jmp_classes['reg_disp']) + len(jmp_classes['reg']) + \
        len(jmp_classes['reg_index']) + len(jmp_classes['abs'])
    summary = {
        'direct_imm_transfers': direct_imm,
        'indirect_calls_total': ff2 + len(classes['far']),
        'indirect_jmps_total': ff4 + len(jmp_classes['far']),
        'call_rip': len(classes['rip']),
        'call_reg_disp': len(classes['reg_disp']),
        'call_reg': len(classes['reg']),
        'call_reg_index': len(classes['reg_index']),
        'call_abs': len(classes['abs']),
        'call_far': len(classes['far']),
        'jmp_rip': len(jmp_classes['rip']),
        'jmp_reg_disp': len(jmp_classes['reg_disp']),
        'jmp_reg': len(jmp_classes['reg']),
        'jmp_reg_index': len(jmp_classes['reg_index']),
        'jmp_abs': len(jmp_classes['abs']),
        'jmp_far': len(jmp_classes['far']),
    }
    # rip resolution outcome
    n_rip = len(classes['rip'])
    n_text = sum(1 for x in classes['rip'] if 'target_text' in x)
    n_begin = sum(1 for x in classes['rip'] if x.get('target_is_func_begin'))
    sec_dist = {}
    for x in classes['rip']:
        s = x.get('ptr_section')
        sec_dist[s] = sec_dist.get(s, 0) + 1
    summary['rip_resolved_to_text'] = n_text
    summary['rip_resolved_to_func_begin'] = n_begin
    summary['rip_total'] = n_rip
    summary['rip_ptr_section_dist'] = sec_dist
    print(json.dumps(summary, indent=1))

    os.makedirs(outdir, exist_ok=True)
    json.dump({'summary': summary,
               'classes': classes,
               'jmp_classes': jmp_classes,
               'disp_hist': disp_hist,
               'base_hist': base_hist,
               'func_count': len(pranges),
               'decoded_insns': decoded_insns},
              io.open(os.path.join(outdir, 'indirect_raw.json'), 'w'), indent=1)
    print('wrote indirect_raw.json')


if __name__ == '__main__':
    main()
