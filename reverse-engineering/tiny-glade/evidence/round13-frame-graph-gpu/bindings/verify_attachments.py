from pathlib import Path
import hashlib, json, struct, subprocess

root = Path('D:/game')
out = root / 'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/bindings/attachments-v2'
out.mkdir(parents=True, exist_ok=True)
paths = [root / 'ljxsj_92385/Tiny Glade/tiny-glade.rne', root / 'ljxsj_92385/Tiny Glade/tiny-glade.exe']
blobs = [path.read_bytes() for path in paths]
expected = ['f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03',
            'cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323']
assert [hashlib.sha256(blob).hexdigest() for blob in blobs] == expected

def offset(blob, address):
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
for address in [0x14157b150, 0x141697ab0, 0x1416f6e90, 0x140bd5920, 0x140bd5960, 0x140bd59a0]:
    row = next(row for row in table if row.startswith(hex(address) + '\t'))
    size = int(row.split('\t')[1])
    chunks = [blob[offset(blob, address):offset(blob, address) + size] for blob in blobs]
    assert chunks[0] == chunks[1]
    name = hex(address)[2:]
    (out / (name + '.bin')).write_bytes(chunks[0])
    proc = subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe', '--disassemble',
        '--start-address=' + hex(address), '--stop-address=' + hex(address + size), str(paths[0])],
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=True)
    assert not proc.stderr, proc.stderr
    (out / (name + '.asm')).write_bytes(proc.stdout)
    rows.append({'address': hex(address), 'size': size, 'pdb_record': row,
                 'range_sha256': hashlib.sha256(chunks[0]).hexdigest(), 'rne_patched_equal': True})
(out / 'original-evidence.json').write_text(json.dumps({
    'artifact_sha256': expected, 'ranges': rows, 'original_executed': False,
    'prior_decompilation': 'evidence/country-core-deep/rhapsody/domains/rg/decomp/decomp-0x1416f6e90.txt'
}, indent=2), encoding='utf-8')
print(json.dumps(rows, indent=2))
