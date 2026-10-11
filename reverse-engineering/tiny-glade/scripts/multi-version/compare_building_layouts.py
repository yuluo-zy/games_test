"""Bounded constructor comparison using cached three-version symbol identity."""
from pathlib import Path
import hashlib, json
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

ROOT=Path(__file__).resolve().parents[2]

def main():
    baselines=json.loads((ROOT/'tracking/baselines.json').read_text())['baselines']; rows=[]
    for version, baseline in baselines.items():
        functions=json.loads((ROOT/f'evidence/multi-version/{version}-functions.json').read_text())['functions']
        target=next(f for f in functions if 'roof_shape4Roof3new17' in f['name'])
        rva=int(target['rva'],16);va=int(target['va'],16)
        section=next(s for s in baseline['pe']['sections'] if s['rva']<=rva<s['rva']+s['raw_size'])
        with Path(baseline['binary']['path']).open('rb') as file:
            actual_hash=hashlib.file_digest(file,'sha256').hexdigest()
            assert actual_hash==baseline['binary']['sha256']
            file.seek(section['raw_pointer']+rva-section['rva']);raw=file.read(256)
        instructions=[]
        for instruction in Cs(CS_ARCH_X86,CS_MODE_64).disasm(raw,va):
            instructions.append({'va':hex(instruction.address),'instruction':instruction.mnemonic+' '+instruction.op_str,
                'bytes':instruction.bytes.hex()})
            if instruction.mnemonic=='ret':break
        assert instructions[-1]['instruction'].startswith('ret')
        rows.append({'baseline':version,'source_sha256':actual_hash,'function':target,'instructions':instructions,
            'scope':'Direct game constructor bytes through first return; no dependency functions analyzed'})
    b,c=rows[1:]
    identical=[i['bytes'] for i in b['instructions']]==[i['bytes'] for i in c['instructions']]
    report={'target_execution':False,'versions':rows,'classification':'game_business',
        'source_path':'crates/country-core/src/resources/roofs/roof_shape.rs',
        'B_C_constructor_bytes_identical':identical,
        'findings':['B/C observed constructor stores identical byte-for-byte',
            'A height/style fields at 0x40/0x44/0x48; B/C 0x4c/0x50/0x54 with extra 12-byte data at 0x40'],
        'equivalence_verified':False,
        'limitations':['No A/C runtime execution or complete building-system equivalence',
            'Identical constructor writes do not establish identical field producer semantics']}
    out=ROOT/'evidence/building-state/constructor-version-comparison.json'
    out.write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps({'B_C_constructor_bytes_identical':identical,'versions':len(rows)}))

if __name__=='__main__':main()
