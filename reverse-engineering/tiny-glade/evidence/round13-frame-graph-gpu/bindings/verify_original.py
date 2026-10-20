from pathlib import Path
import hashlib, json, struct, subprocess

root = Path('D:/game')
out = root / 'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/bindings'
paths = [root / 'ljxsj_92385/Tiny Glade/tiny-glade.rne', root / 'ljxsj_92385/Tiny Glade/tiny-glade.exe']
blobs = [p.read_bytes() for p in paths]
expected = ['f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03',
            'cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323']
identities = []
for path, blob, want in zip(paths, blobs, expected):
    sha = hashlib.sha256(blob).hexdigest()
    assert sha == want, (str(path), sha)
    identities.append({'path': str(path), 'sha256': sha})

def section_offset(blob, address):
    pe = struct.unpack_from('<I', blob, 0x3c)[0]
    n = struct.unpack_from('<H', blob, pe + 6)[0]
    optional_size = struct.unpack_from('<H', blob, pe + 20)[0]
    for i in range(n):
        pos = pe + 24 + optional_size + i * 40
        virtual_size, rva, raw_size, raw = struct.unpack_from('<IIII', blob, pos + 8)
        wanted = address - 0x140000000
        if rva <= wanted < rva + max(virtual_size, raw_size):
            return raw + wanted - rva
    raise ValueError(hex(address))

table = (root / 'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').read_text(encoding='utf-8').splitlines()
rows = []
for start in [0x1408ee800, 0x140867970, 0x140bd6c00, 0x140bd6c30]:
    row = next(row for row in table if row.startswith(hex(start) + '\t'))
    fields = row.split('\t')
    size = int(fields[1])
    chunks = [blob[section_offset(blob, start):section_offset(blob, start) + size] for blob in blobs]
    assert chunks[0] == chunks[1]
    name = hex(start)[2:]
    (out / (name + '.bin')).write_bytes(chunks[0])
    proc = subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe', '--disassemble',
        '--start-address=' + hex(start), '--stop-address=' + hex(start + size), str(paths[0])],
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=True)
    assert not proc.stderr, proc.stderr
    (out / (name + '.asm')).write_bytes(proc.stdout)
    rows.append({'address': hex(start), 'size': size, 'pdb_record': row,
                 'range_sha256': hashlib.sha256(chunks[0]).hexdigest(),
                 'rne_patched_equal': True, 'disassembly': name + '.asm'})

report = {'artifacts': identities, 'ranges': rows, 'original_executed': False,
          'mcp_success': False, 'mcp_failure': 'Ghidra LaunchSupport cannot write java_home.save outside sandbox; no overlay started',
          'scope': 'Static original binding/attachment producer contract and exact native bytes only; no GPU equivalence claim'}
(out / 'original-evidence.json').write_text(json.dumps(report, indent=2), encoding='utf-8')
print(json.dumps(report, indent=2))
