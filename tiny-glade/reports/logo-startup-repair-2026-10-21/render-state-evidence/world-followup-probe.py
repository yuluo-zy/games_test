"""Reuse the already verified bounded PE reader; do not execute original code."""
from pathlib import Path
prefix = Path(__file__).with_name('probe.py').read_text(encoding='utf-8').split('records = []')[0]
exec(compile(prefix, str(Path(__file__).with_name('probe.py')), 'exec'))
out = Path(__file__).parent / 'world-followup'
out.mkdir(exist_ok=True)
records = []
for name, address in [
    ('shadow', 0x1409cc270), ('main', 0x140833bc0), ('main-oc2', 0x140835a40),
    ('late', 0x1408373b0), ('deferred', 0x1409d20b0), ('nani-draw', 0x140904ae0),
    ('grass-lod0', 0x140b4af70), ('grass-lod1', 0x140b4b1d0),
    ('roof-tile', 0x1409d9070), ('roof-spire', 0x1409d96a0), ('roof-beam', 0x1409d98a0),
    ('rocky-terrain', 0x1409cb4e0), ('bricks-lod0', 0x1408a53d0), ('bricks-lod1', 0x1408a4ca0),
]:
    size, symbol = procedures[address]
    body = body_at(address, size)
    assembly = '\n'.join(f'{ins.address:#x}: {ins.mnemonic} {ins.op_str}'
                         for ins in decoder.disasm(body, address)) + '\n'
    (out / (name + '.asm')).write_text(assembly, encoding='utf-8')
    records.append(dict(name=name, address=hex(address), size=size, symbol=symbol,
                        body_sha256=hashlib.sha256(body).hexdigest(),
                        assembly_sha256=hashlib.sha256(assembly.encode()).hexdigest()))
sites = []
for address, size, meaning in [
    (0x1404ec648, 8, 'W34 caller passes scene color r15 in r9'),
    (0x1408373d6, 3, 'save original r9'),
    (0x140837400, 7, 'save original r9 before Deferred reuses r14'),
    (0x140837416, 3, 'Deferred r14 becomes GBuffer r8'),
    (0x14083742f, 7, 'Deferred attachment pointer'),
    (0x14083773e, 7, 'restore original r9 for Forward'),
    (0x14083777d, 17, 'Forward LOAD tag and original r9 attachment pointer'),
    (0x1408377da, 18, 'Forward actual new_impl color spec pointer'),
]:
    sites.append(dict(address=hex(address), bytes=body_at(address,size).hex(), meaning=meaning))
(out / 'manifest.json').write_text(json.dumps(dict(original=str(original),
    original_sha256=identity, evidence='PDB-bounded static decode; no execution',
    functions=records, instruction_sites=sites), indent=2) + '\n', encoding='utf-8')
print(f'Bounded decoded {len(records)} world procedures')
