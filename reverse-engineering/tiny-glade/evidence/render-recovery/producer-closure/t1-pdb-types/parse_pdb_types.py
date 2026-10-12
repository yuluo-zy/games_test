"""Read-only MSF / CodeView parser for Tiny Glade's tiny_glade.pdb.

Answers, independently of llvm-pdbutil text output, whether the PDB carries
real function parameter/return types for Rust ("rhapsody") code.

Parses directly from the MSF container:
  - SuperBlock + stream directory
  - Stream 1  (PDB Info: GUID/age)
  - Stream 2  (TPI: global type records)     -> leaf-kind histogram + LF_PROCEDURE decode
  - Stream 4  (IPI: item IDs, Rust func IDs) -> leaf-kind histogram + FUNC_ID->type map
  - Stream 3  (DBI: modules, section contribs, per-module symbol streams)
  - the PE's section headers (for RVA->module->function mapping)

Reads the original PDB/exe read-only. Writes nothing. Usage:
  python -I parse_pdb_types.py <pdb> <exe> [--rva 0x140530430 ...]
"""
from __future__ import annotations
import struct, sys, json, os, re
from collections import Counter, defaultdict

# --- CodeView leaf kinds (cvinfo.h values) --------------------------------
LEAF = {
    0x000A: "LF_VTSHAPE", 0x000E: "LF_LABEL",
    0x1001: "LF_MODIFIER", 0x1002: "LF_POINTER", 0x1008: "LF_PROCEDURE",
    0x1009: "LF_MFUNCTION", 0x1201: "LF_ARGLIST", 0x1203: "LF_FIELDLIST",
    0x1205: "LF_METHODLIST", 0x1206: "LF_DIMCONU", 0x1207: "LF_DIMCONLU",
    0x1503: "LF_ARRAY", 0x1504: "LF_CLASS", 0x1505: "LF_STRUCTURE",
    0x1506: "LF_UNION", 0x1507: "LF_ENUM", 0x151D: "LF_VFTABLE",
    0x1601: "LF_FUNC_ID", 0x1602: "LF_MFUNC_ID", 0x1603: "LF_BUILDINFO",
    0x1604: "LF_SUBSTR_LIST", 0x1605: "LF_STRING_ID", 0x1606: "LF_UDT_SRC_LINE",
    0x1607: "LF_UDT_MOD_SRC_LINE",
}
SYMKIND = {
    0x1101: "S_REGISTER", 0x1102: "S_CONSTANT", 0x1103: "S_UDT",
    0x1105: "S_MANYREG", 0x1106: "S_BPREL32", 0x1107: "S_LDATA32",
    0x1108: "S_GDATA32", 0x1109: "S_PUB32", 0x110A: "S_LPROC32",
    0x110B: "S_GPROC32", 0x110C: "S_REGREL32", 0x110D: "S_LTHREAD32",
    0x110E: "S_GTHREAD32", 0x1111: "S_PROCREF", 0x1112: "S_LPROCREF",
    0x1125: "S_FRAMEPROC", 0x1128: "S_BUILDINFO", 0x1129: "S_INLINEES",
    0x112A: "S_INLINESITE", 0x112B: "S_INLINESITE_END", 0x112C: "S_PROC_ID_END",
    0x1130: "S_OBJNAME", 0x1131: "S_COMPILE3", 0x1132: "S_ENVBLOCK",
    0x1134: "S_LOCAL", 0x113C: "S_LPROC32_ID", 0x113D: "S_GPROC32_ID",
    0x113E: "S_INLINESITE2", 0x1142: "S_END", 0x1147: "S_ARMSWITCHTABLE",
    0x114C: "S_POGODATA", 0x1153: "S_CALLSITEINFO",
}
def u16(b, o): return struct.unpack_from("<H", b, o)[0]
def u32(b, o): return struct.unpack_from("<I", b, o)[0]
def i32(b, o): return struct.unpack_from("<i", b, o)[0]
def align4(n): return (n + 3) & ~3
def cstr(b, o):
    e = b.find(b"\x00", o)
    if e < 0: e = len(b)
    return b[o:e].decode("utf-8", "replace")


class MSF:
    def __init__(self, path):
        self.raw = open(path, "rb").read()
        b = self.raw
        assert b[:32].startswith(b"Microsoft C/C++ MSF 7.00"), "bad MSF magic"
        self.block_size = u32(b, 0x20)
        self.num_blocks = u32(b, 0x28)
        self.num_dir_bytes = u32(b, 0x2C)
        self.block_map_addr = u32(b, 0x34)
        nblk = (self.num_dir_bytes + self.block_size - 1) // self.block_size
        dbptr = self.block_map_addr * self.block_size
        dblocks = [u32(b, dbptr + 4 * i) for i in range(nblk)]
        content = b"".join(b[x*self.block_size:x*self.block_size+self.block_size]
                           for x in dblocks)[:self.num_dir_bytes]
        self.num_streams = u32(content, 0)
        sizes_off = 4
        self.stream_sizes = [u32(content, sizes_off + 4*i) for i in range(self.num_streams)]
        off = sizes_off + 4 * self.num_streams
        self.stream_blocks = []
        for i in range(self.num_streams):
            s = self.stream_sizes[i]
            c = 0 if s in (0, 0xFFFFFFFF) else (s + self.block_size - 1)//self.block_size
            self.stream_blocks.append([u32(content, off + 4*j) for j in range(c)])
            off += 4 * c

    def stream(self, i):
        if self.stream_sizes[i] == 0xFFFFFFFF: return b""
        data = b"".join(self.raw[x*self.block_size:x*self.block_size+self.block_size]
                        for x in self.stream_blocks[i])
        return data[:self.stream_sizes[i]]


def walk_types(data, index_begin):
    header_size = u32(data, 4)
    pos = header_size; idx = index_begin
    out = []; kinds = Counter(); n = len(data)
    while pos + 4 <= n:
        length = u16(data, pos); kind = u16(data, pos + 2)
        out.append((idx, kind, pos + 4, pos + 2 + length)); kinds[kind] += 1
        idx += 1; pos = align4(pos + 2 + length)
    return out, kinds


def pe_sections(exe_path):
    b = open(exe_path, "rb").read(0x4000)
    e_lfanew = u32(b, 0x3C)
    nsec = u16(b, e_lfanew + 6)
    size_opt = u16(b, e_lfanew + 20)
    opt_off = e_lfanew + 24
    magic = u16(b, opt_off)
    image_base = struct.unpack_from("<Q", b, opt_off + 24)[0] if magic == 0x20B \
        else u32(b, opt_off + 28)
    sec_off = e_lfanew + 24 + size_opt
    secs = []
    for i in range(nsec):
        o = sec_off + 40 * i
        name = b[o:o+8].rstrip(b"\x00").decode("latin1")
        vsize = u32(b, o + 8); va = u32(b, o + 12)
        secs.append({"name": name, "rva": va, "vsize": vsize, "num": i + 1})
    return secs, image_base


def main():
    pdb_path = sys.argv[1]; exe_path = sys.argv[2]
    rvas = []
    if "--rva" in sys.argv:
        i = sys.argv.index("--rva") + 1
        while i < len(sys.argv) and not sys.argv[i].startswith("--"):
            rvas.append(int(sys.argv[i], 16)); i += 1
    out = {}
    m = MSF(pdb_path)
    out["container"] = {"block_size": m.block_size, "num_blocks": m.num_blocks,
                        "num_streams": m.num_streams, "file_size": os.path.getsize(pdb_path)}
    info = m.stream(1)
    out["pdb_info"] = {"version": u32(info,0), "signature": u32(info,4), "age": u32(info,8),
                       "guid_hex": info[12:28].hex()}

    # ---- TPI ----
    tpi = m.stream(2); tb = u32(tpi, 8)
    trecs, tkinds = walk_types(tpi, tb)
    proc = {}; mfunc = {}
    for (idx, kind, po, pe) in trecs:
        if kind == 0x1008:  # LF_PROCEDURE: ret(4) cc(1) opt(1) argc(2) arglist(4)
            proc[idx] = ("LF_PROCEDURE", u32(tpi,po), u16(tpi,po+6), u32(tpi,po+8))
        elif kind == 0x1009:  # LF_MFUNCTION: ret(4) class(4) this(4) cc(1) opt(1) argc(2) arglist(4)
            mfunc[idx] = (u32(tpi,po), u32(tpi,po+4), u16(tpi,po+14), u32(tpi,po+16))
    void_ph = [i for i,(t,r,a,l) in proc.items() if r == 0x0003 and a == 0]
    out["TPI"] = {"num_records": len(trecs), "index_begin": hex(tb), "index_end": hex(u32(tpi,12)),
                  "kind_histogram": {LEAF.get(k, hex(k)): v for k,v in sorted(tkinds.items(), key=lambda x:-x[1])},
                  "lf_procedure_total": len(proc), "lf_mfunction_total": len(mfunc),
                  "lf_procedure_void_placeholder": len(void_ph),
                  "lf_procedure_nonvoid_sample": [[hex(i), hex(proc[i][1]), proc[i][2], hex(proc[i][3])]
                                                  for i in sorted(proc) if proc[i][1] != 3 or proc[i][2] != 0][:10]}

    # ---- IPI ----
    ipi = m.stream(4); ib = u32(ipi, 8)
    irecs, ikinds = walk_types(ipi, ib)
    strings = {}; funcids = []; mfuncids = []
    for (idx, kind, po, pe) in irecs:
        if kind == 0x1605: strings[idx] = cstr(ipi, po + 4)
        elif kind == 0x1601: funcids.append((idx, u32(ipi,po), u32(ipi,po+4), cstr(ipi,po+8)))
        elif kind == 0x1602: mfuncids.append((idx, u32(ipi,po), u32(ipi,po+4), cstr(ipi,po+8)))
    def scope(sid):
        # STRING_ID whose own record may chain via 'ID' field; here scopes are single strings
        return strings.get(sid, f"<{hex(sid)}>")
    tdist = Counter(f[2] for f in funcids)
    rh = [f for f in funcids if "rhapsody" in scope(f[1])]
    out["IPI"] = {"num_records": len(irecs), "index_begin": hex(ib),
                  "kind_histogram": {LEAF.get(k, hex(k)): v for k,v in sorted(ikinds.items(), key=lambda x:-x[1])},
                  "func_id_total": len(funcids), "mfunc_id_total": len(mfuncids),
                  "string_id_total": len(strings),
                  "func_id_type_dist_top": [[hex(t), c] for t,c in tdist.most_common(12)],
                  "func_id_pointing_to_void_placeholder_0x1001": tdist.get(0x1001, 0),
                  "rhapsody_func_id_total": len(rh),
                  "rhapsody_func_id_type_dist": [[hex(t), c] for t,c in Counter(f[2] for f in rh).most_common()],
                  "rhapsody_func_id_samples": [
                      {"id": hex(f[0]), "scope": scope(f[1]), "name": f[3], "func_type": hex(f[2]),
                       "tpi_kind": proc.get(f[2], mfunc.get(f[2], ("<none>",)))[0],
                       "resolved": (lambda p: {"ret": hex(p[1]), "argc": p[2], "arglist": hex(p[3])}
                                    if p[0] == "LF_PROCEDURE" else None)(proc.get(f[2], ("<none>",)))
                       } for f in rh[:20]]}

    # ---- DBI ----
    dbi = m.stream(3)
    modi_size = i32(dbi, 24); secc_size = i32(dbi, 28)
    mod_info = dbi[64:64+modi_size]
    nmods = u32(mod_info, 0)
    # Because the DBI mod_info count byte reads 0 in this file, module stream
    # indices are taken from llvm-pdbutil --sym-stats (see --symstats arg).
    out["DBI"] = {"modi_substream_size": modi_size, "seccontrib_substream_size": secc_size,
                  "mod_info_first_u32": nmods, "symstats_used": "--symstats" in sys.argv}

    # symbol-kind histogram across all modules + procedure list for RVA mapping
    sym_kinds = Counter(); total_syms = 0
    gproc = []   # (module, seg, off, size, name, kind)
    mod_streams = []
    if "--symstats" in sys.argv:
        ss_path = sys.argv[sys.argv.index("--symstats") + 1]
        cur = None; want_sym = False
        for line in open(ss_path, encoding="utf-8", errors="replace"):
            mm = re.match(r"Mod (\d+) \|", line)
            if mm: cur = int(mm.group(1)); continue
            ms = re.search(r"Stream (\d+), (\d+) bytes", line)
            if ms and cur is not None:
                mod_streams.append([cur, int(ms.group(1)), int(ms.group(2)), None]); continue
            if re.match(r"\s*Symbols\s*$", line): want_sym = True; continue
            mt = re.search(r"Total:\s+(\d+) entries \(\s*([\d,]+) bytes\)", line)
            if mt and want_sym and mod_streams and mod_streams[-1][3] is None:
                mod_streams[-1][3] = int(mt.group(2).replace(",", "")); want_sym = False
    out["DBI"]["modules_from_symstats"] = len(mod_streams)
    for (mi, ss, sbytes, symbytes) in mod_streams:
        if ss in (0, 0xFFFF) or ss >= m.num_streams: continue
        data = m.stream(ss)
        limit = (4 + symbytes) if symbytes else len(data)
        pos = 4; n = min(len(data), limit)
        while pos + 4 <= n:
            length = u16(data, pos); kind = u16(data, pos + 2)
            if length < 2: break
            sym_kinds[kind] += 1; total_syms += 1
            if kind in (0x110B, 0x110A, 0x113D, 0x113C):  # *PROC32* (ProcSym)
                base = pos + 4
                cs = u32(data, base + 12); co = u32(data, base + 28)
                seg = u16(data, base + 32); nm = cstr(data, base + 35)
                gproc.append((mi, seg, co, cs, nm, kind))
            pos = align4(pos + 2 + length)
    out["DBI"]["module_symbol_kind_histogram"] = {SYMKIND.get(k, hex(k)): v
                                                  for k,v in sorted(sym_kinds.items(), key=lambda x:-x[1])}
    out["DBI"]["module_symbols_total"] = total_syms
    out["DBI"]["procedure_records_total"] = len(gproc)

    # ---- RVA -> module -> function ----
    secs, image_base = pe_sections(exe_path)
    out["PE_sections"] = secs
    out["image_base"] = hex(image_base)
    rva_out = []
    for va in rvas:
        rva = va - image_base if va >= image_base else va
        sec = next((s for s in secs if s["rva"] <= rva < s["rva"] + s["vsize"]), None)
        hit = {"va": hex(va), "rva": hex(rva), "section": sec["name"] if sec else None,
               "section_num": sec["num"] if sec else None}
        if sec:
            soff = rva - sec["rva"]
            cands = []
            for (mod, seg, off_, csz, nm, kind) in gproc:
                if seg == sec["num"] and off_ <= soff < off_ + csz:
                    cands.append({"module": mod, "seg": seg, "off": hex(off_), "code_size": csz,
                                  "name": nm, "kind": SYMKIND.get(kind, hex(kind))})
            hit["candidates"] = cands[:6]
        rva_out.append(hit)
    out["rva_lookup"] = rva_out

    json.dump(out, sys.stdout, indent=2)


if __name__ == "__main__":
    main()
