"""Bounded, reproducible source ownership and original C guard comparison."""
import hashlib,json,struct
from pathlib import Path
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
ROOT=Path(__file__).resolve().parents[2]
OUT=Path(__file__).resolve().parent
baselines=json.loads((ROOT/'tracking/baselines.json').read_text())['baselines']
sources={}
paths=['crates/country-core/src/resources/history/mod.rs',
       'crates/country-core/src/resources/history/roof_edit.rs',
       'crates/country-core/src/systems/history/history_process.rs',
       'crates/country-core/src/systems/history/history_recording.rs']
for v in 'ABC':
    metadata=json.loads((ROOT/f'evidence/multi-version/{v}-sources.json').read_text())
    sources[v]={'pdb':metadata['source'],
        'game_sources':[row for row in metadata['files'] if row['path'] in paths],
        'dependency_examples':[p for p in metadata['dependency_source_paths']
             if '/bevy_ecs' in p or '/fastrand-' in p][:4]}
raw=Path(baselines['C']['binary']['path']).read_bytes()
assert hashlib.sha256(raw).hexdigest()==baselines['C']['binary']['sha256']
pe=struct.unpack_from('<I',raw,60)[0];opt=pe+24
base=struct.unpack_from('<Q',raw,opt+24)[0]
section_table=opt+struct.unpack_from('<H',raw,pe+20)[0]
def read(va,n):
    for i in range(struct.unpack_from('<H',raw,pe+6)[0]):
        _,rva,sz,off=struct.unpack_from('<IIII',raw,section_table+i*40+8)
        if rva<=va-base<rva+sz:return raw[off+va-base-rva:off+va-base-rva+n]
    raise ValueError(hex(va))
guard_const=read(0x140b4ae26+0x24f773a,4)
guard_message=read(0x140b4af26+0x24f7652,47).decode()
guard_text=['Replay static bytes: '+guard_const.hex(),'panic string: '+guard_message]
for insn in Cs(CS_ARCH_X86,CS_MODE_64).disasm(read(0x140b4d1b0,128),0x140b4d1b0):
    guard_text.append(f'{insn.address:x} {insn.mnemonic} {insn.op_str}')
(OUT/'C-start-guard.asm.txt').write_text('\n'.join(guard_text)+'\n')
report={'scope':'Original PDB source ownership plus bounded A/B/C history policy comparison; B only runtime transaction acceptance',
 'binary_sha256':{v:baselines[v]['binary']['sha256'] for v in 'ABC'},'sources':sources,
 'methods':json.loads((OUT/'version-methods.json').read_text()),
 'layout_differences':{
  'A':{'MultiEdit_stride':32,'cursor_offset':'0x50','empty_checks':2,
       'finish_nonce_write':'not present in inspected A finish body'},
  'B':{'MultiEdit_stride':40,'cursor_offset':'0x68','nonce_offset':'0x60','empty_checks':4,'Replay_guard_tag':13},
  'C':{'MultiEdit_stride':40,'cursor_offset':'0x68','nonce_offset':'0x60','empty_checks':4,
       'Replay_guard_tag':guard_const[0],'AppMode_representation':'byte tag; B uses dword tag'}},
 'policy_findings':[
  'A/B/C finish takes pending Option first, panics on None, skips empty groups, increments cursor and requires committed tail.',
  'A pending group has one auxiliary Arc and 32-byte chain entry; B/C have decorator and stairs auxiliaries and 40-byte entries.',
  'B/C first edit after cursor falls behind chain calls nuke_future_states; start alone does not truncate redo.',
  'B/C finish_potential_mutation active flag clears snapshots and refreshes fastrand nonce; it is distinct from group commit.',
  'C recording mode guard changed to byte Replay tag 9; B dword Replay tag 13 must not be reused as C ABI.',
  'All four inspected business source checksums differ across A/B/C. Shared policy must be established from instructions, not same paths.'],
 'dependency_decisions':[
  'country-core resources/history and systems/history remain game-owned even under core::ptr::drop_in_place aliases.',
  'HistoryMutationToken Drop contains the game forgotten-mutation panic, so only its guard is recovered.',
  'Vec growth/Arc release/HashMap RandomState/Bevy event append and fastrand TLS mechanics are dependency code; use Rust/Bevy/fastrand APIs.',
  'No recreation of auxiliary owner payloads, snapshot loading, allocator internals, ECS descriptors, or TLS RNG implementation.'],
 'limits':['C policy is static comparison, not C transaction oracle acceptance',
           'A Rim edit expansion not matched; no transplant of B Rim tag/layout to A',
           'No original startup ledger constructor proof','No complete MultiEdit auxiliary or timed replay scheduler']}
(OUT/'version-comparison.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({'C_Replay_guard_tag':guard_const[0],'C_guard_message':guard_message}))
