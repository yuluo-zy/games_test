"""Restore only one generated enrichment block from its verified companion data.

Preserves the current base/dependency annotations and all global reports.
"""
import importlib.util
import gzip
import json
from pathlib import Path
import sys

ROOT=Path(__file__).resolve().parents[1]
def read(p):
    with gzip.open(p,'rt')as f:return json.load(f)

def main():
    v,path=sys.argv[1:];source=ROOT/v/path
    payload=read(source.with_suffix('.rs.enriched.json.gz'))
    spec=importlib.util.spec_from_file_location('enrich',ROOT/'_tools/enrich_skeleton.py');module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    baseline=json.loads((ROOT.parent/'tracking/baselines.json').read_text())['baselines'][v]
    e=module.Enrichment(v,baseline);e.binding=payload['binding']
    e.sources={path:payload['source']};e.original_sources={path}
    e.functions={path:{f['id']:f for f in payload['functions']}}
    e.file_candidates={path:{c['candidate_id']:c for c in payload['file_interface_candidates']}}
    e.wrapper_types=read(e.out/'candidate-interfaces.json.gz')
    e.procs={p['procedure_id']:p for p in read(e.out/'procedure-index.json.gz')}
    aliases=read(e.out/'address-aliases.json.gz');e.by_rva={int(k):[x['procedure_id']for x in a]for k,a in aliases.items()}
    save=module.write_json
    def local_only(p,data):
        if p.parent==e.out:return
        save(p,data)
    module.write_json=local_only;e.render()
    assert source.read_text().count(module.START)==1
    print('Restored enrichment block:',source)

if __name__=='__main__':main()
