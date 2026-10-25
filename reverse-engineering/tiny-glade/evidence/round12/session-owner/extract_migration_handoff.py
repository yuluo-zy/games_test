from pathlib import Path
import re, json, hashlib, struct

root = Path('D:/game/reverse-engineering/tiny-glade')
out = Path(__file__).resolve().parent
asm = (root / 'evidence/round12/startup/parse-saved-body.asm').read_text(encoding='utf-8')
procs = {}
for line in (root / 'evidence/country-core-audit/data/procs.tsv').open(encoding='utf-8'):
    cols = line.rstrip().split('\t')
    if len(cols) >= 5 and cols[0].startswith('0x'):
        procs.setdefault(cols[0].lower(), []).append({'bytes':int(cols[1]), 'symbol':cols[-1]})
calls = []
branches = []
for line in asm.splitlines():
    m = re.match(r'([0-9a-f]+):.*?callq\s+(0x[0-9a-f]+)', line)
    if m and 0x141471d3c <= int(m[1],16) < 0x14147319b:
        symbols = procs.get(m[2],[])
        calls.append({'at':'0x'+m[1], 'target':m[2],
                      'icf_alias_count':len(symbols), 'representative_symbols':symbols[:8],
                      'identity_warning':'First PDB name is not proof of a game operation when ICF aliases exist.'})
    m = re.match(r'([0-9a-f]+):.*?\b(cmpl|setb|setl|orb|cmpb|movb)\b(.*)', line)
    if m and 0x141471d3c <= int(m[1],16) < 0x14147319b:
        if any(x in m[3] for x in ['%r13d','0x336(%rbp)','0x304(%rbp)','0x2f0(%rbp)']):
            branches.append({'at':'0x'+m[1], 'instruction':m[2]+m[3]})
exe = root/'workspace/input/tiny-glade.exe'
blob = exe.read_bytes()
h = struct.unpack_from('<I',blob,0x3c)[0]
n = struct.unpack_from('<H',blob,h+6)[0]
opt_size = struct.unpack_from('<H',blob,h+20)[0]
base = struct.unpack_from('<Q',blob,h+48)[0]
def read_va(va,size):
    for i in range(n):
        sec=h+24+opt_size+i*40
        vs,rva,raw_size,offset=struct.unpack_from('<IIII',blob,sec+8)
        if rva<=va-base<rva+max(vs,raw_size):
            start=offset+va-base-rva
            return blob[start:start+size]
    raise ValueError(hex(va))
items={
    'original':{'path':str(exe), 'sha256':hashlib.sha256(blob).hexdigest()},
    'parser_procedure':procs.get('0x141470d00'),
    'migration_range':{'start':'0x141471d3c','end_exclusive':'0x14147319b','bytes':0x14147319b-0x141471d3c,
       'sha256':hashlib.sha256(read_va(0x141471d3c,0x14147319b-0x141471d3c)).hexdigest()},
    'calls_in_migration_range':calls,
    'version_and_flag_instructions':branches,
    'return_wrapper':{'body_pointer':0,'target_chain_index_i32':8,'backup_if_upgraded':12,
      'snapshot_window_colors_and_snapshot_reference_invalidation':13,
      'snapshot_arrow_slit_colors':14,'legacy_version_lt_19_notice_or_err_niche2':15},
    'verified_default_colors':{'native_constant_142c63d10_u32':list(struct.unpack('<4I',read_va(0x142c63d10,16))),
       'lantern_brightness_constant_142925904':struct.unpack('<f',read_va(0x142925904,4))[0],
       'settings_reads':[{'at':'0x14147191a','native_settings_offset':20,'field':'default_building_color_id'},
         {'at':'0x141471924','native_settings_offset':24,'field':'default_roof_color_id'},
         {'at':'0x14147192a','native_settings_offset':16,'field':'default_freeform_wall_color_id'}]},
    'tool_state':'retained original full-range LLVM assembly reused; no MCP worker mutation; no Cargo'}
(out/'migration-handoff.json').write_text(json.dumps(items,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps({'range_bytes':items['migration_range']['bytes'],'callee_sites':len(calls),'version_flag_instructions':len(branches)}))
