"""Read-only full-PDB native ranges needed for the typed W35..43 contract."""
from pathlib import Path
import hashlib
import json

import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

ROOT = Path('/Users/yuluo/Desktop/game')
OUT = Path(__file__).parent
RNE = ROOT / 'ljxsj_92385/Tiny Glade/tiny-glade.rne'
EXPECTED = 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
PROCS = ROOT / 'games_test/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv'
WANTED = {0x1404ec110, 0x140a01cf0, 0x1409e7770, 0x1409d20b0, 0x140a97a30,
          0x1404f7450, 0x1404f6e50, 0x1408f6180}
CALLEES = {0x1408f6180, 0x140a0d190, 0x1408743b0, 0x140a01cf0,
           0x140a44cd0, 0x1409e7770, 0x140a6fec0, 0x140a39e70, 0x1409d20b0}


def main():
    data = RNE.read_bytes()
    assert hashlib.sha256(data).hexdigest() == EXPECTED
    pe = pefile.PE(data=data, fast_load=True)
    rows, callees = {}, {}
    for line in PROCS.read_text().splitlines():
        cols = line.split('\t')
        try:
            va = int(cols[0], 0)
        except ValueError:
            continue
        if va in WANTED:
            rows[va] = (int(cols[1]), cols[4])
        if va in CALLEES:
            callees[hex(va)] = {'full_pdb_length': int(cols[1]), 'symbol': cols[4]}
    assert rows.keys() == WANTED
    assert len(callees) == len(CALLEES)
    cs = Cs(CS_ARCH_X86, CS_MODE_64)
    result = []
    for va, (length, symbol) in sorted(rows.items()):
        code = pe.get_data(va - pe.OPTIONAL_HEADER.ImageBase, length)
        insns = list(cs.disasm(code, va))
        assert sum(ins.size for ins in insns) == length
        path = OUT / f'{va:x}.asm'
        path.write_text(''.join(f'{ins.address:x}: {bytes(ins.bytes).hex()} {ins.mnemonic} {ins.op_str}\n' for ins in insns))
        result.append({'entry': hex(va), 'full_pdb_length': length, 'symbol': symbol,
                       'range_sha256': hashlib.sha256(code).hexdigest(),
                       'instructions': len(insns), 'path': path.name})
    manifest = {'scope': 'Original B native evidence only. No execution, Cargo, GPU or production edits.',
                'rne': str(RNE), 'rne_sha256': EXPECTED,
                'procs_sha256': hashlib.sha256(PROCS.read_bytes()).hexdigest(),
                'functions': result, 'w35_43_original_callees': callees}
    (OUT / 'original-manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
    print(json.dumps(manifest, indent=2))


if __name__ == '__main__':
    main()
