"""Verify filled metadata, explicit mapping gaps and identity-bound references."""
import gzip
import hashlib
import json
from pathlib import Path
import struct

ROOT=Path(__file__).resolve().parents[1]
PROJECT=ROOT.parent

def load_gz(p):
    with gzip.open(p,'rt')as f:return json.load(f)

def main():
    reports=[]
    for v in 'ABC':
        out=ROOT/'_pdb'/v/'enrichment'
        report=json.loads((out/'report.json').read_text());files=json.loads((out/'source-files.json').read_text())
        mappings=json.loads((out/'missing-function-mappings.json').read_text());catalog=load_gz(out/'candidate-interfaces.json.gz')
        aliases=load_gz(out/'address-aliases.json.gz');procs={p['procedure_id']:p for p in load_gz(out/'procedure-index.json.gz')}
        native=json.loads((out/'native-reference-report.json').read_text())
        assert native['native_record_cache_reused']
        functions=direct=inline=closures=0;seen_paths=set();sample_refs=0
        base_sources=json.loads((ROOT/'_pdb'/v/'source-files.json').read_text())
        for item in files:
            path=item['path'];seen_paths.add(path);source=ROOT/v/path
            text=source.read_text();assert text.count('// === PDB ENRICHMENT V1 BEGIN ===')==1
            assert text.count('// === PDB ENRICHMENT V1 END ===')==1
            assert '参数名 = unknown'in text or item['functions']==0
            payload=load_gz(ROOT/item['metadata_file']);fs=payload['functions'];assert len(fs)==item['functions']
            assert payload['binding']['matching_rsds']and not payload['binding']['target_executed']
            for c in payload['file_interface_candidates']:
                assert c['candidate_id']in catalog;assert c['parameter_type_spellings']==catalog[c['candidate_id']]['parameter_type_spellings']
                assert c['actual_function_binding']=='unknown'
            for fn in fs:
                functions+=1;direct+=len(fn['reference_callers']);inline+=len(fn['inline_hosts']);closures+=bool(fn['closure'])
                assert fn['unknowns']['author_parameter_names']=='unknown'
                assert fn['unknowns']['indirect_callers']=='unknown'
                assert all(i in catalog for i in fn['candidate_interface_ids'])
                if fn['closure']:assert fn['closure']['captures']=='unknown'
                entries={p['rva']for p in fn['compiled_instances']}
                for p in fn['compiled_instances']:assert p['procedure_id']in procs
                for r in fn['reference_callers']:
                    assert r['target_rva']in entries
                    assert str(r['caller_alias_group_rva'])in aliases and str(r['target_alias_group_rva'])in aliases
                    assert not r['runtime_observed']
                    if sample_refs<128:
                        raw=bytes.fromhex(r['bytes']);width=1 if r['kind']=='direct_jump_outside_procedure'and r['llvm_opcode'].startswith('JMP_1')else 4
                        assert r['instruction_rva']+len(raw)+int.from_bytes(raw[-width:],'little',signed=True)==r['target_rva'];sample_refs+=1
                for r in fn['inline_hosts']:
                    assert r['kind']=='inlined_into'and not r['is_standalone_call']
                    if r['host_procedure_id']is not None:assert r['host_procedure_id']in procs
        assert {s['path']for s in base_sources}<=seen_paths
        assert functions==report['function_entries']and direct==report['direct_reference_records_attached']and inline==report['inline_hosts_attached']
        for m in mappings:
            assert m['procedure_id']in procs and m['source_files']
            for path in m['source_files']:assert path in seen_paths
            if m['status']=='unknown_source_preserved_in_module_bucket':assert all(p.startswith('_unmapped/')for p in m['source_files'])
            if 'candidate'in m['status']:assert m['author_definition_file_confirmed']is False
        assert len(mappings)==report['missing_function_records_routed']
        assert all(c['rust_parameter_names']=='unknown'and c['binding_to_game_function']=='unknown'for c in catalog.values())
        reports.append({'version':v,'ok':True,'source_files':len(files),'function_entries':functions,'candidate_function_types':len(catalog),
            'direct_reference_records':direct,'inline_host_records':inline,'closure_entries':closures,'missing_mappings_checked':len(mappings),
            'all_original_source_files_enriched':True,'caller_aliases_resolvable':True,'candidate_bindings_stay_explicit':True,'sample_reference_operands_checked':sample_refs,
            'native_scan_decode_gaps':native['ranges_with_decode_gaps'],'native_scan_indirect_jump_ranges':native['ranges_with_indirect_jumps']})
    before=json.loads((ROOT/'_pdb/production-source-snapshot-before-enrichment.json').read_text())
    changed=[]
    for rel,wanted in before.items():
        p=PROJECT/rel
        if not p.exists()or hashlib.sha256(p.read_bytes()).hexdigest()!=wanted:changed.append(rel)
    assert not changed,changed
    final={'ok':True,'versions':reports,'production_source_files_unchanged':len(before),'scope':'Metadata, candidate/source distinctions and decoded static references; not game behavior or complete indirect call graph'}
    (ROOT/'pdb-enrichment-verification.json').write_text(json.dumps(final,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps(final,ensure_ascii=False))

if __name__=='__main__':main()
