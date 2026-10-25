"""Bounded original RNE probe; never run the negative candidate to exhaustion."""
import importlib.util,json,sys
from pathlib import Path
OUT=Path(__file__).parent
prior=OUT.parents[1]/'shared-foundation-2026-10-13/utils-base/native_spline_rdp.py'
spec=importlib.util.spec_from_file_location('native_spline_rdp',prior)
original=importlib.util.module_from_spec(spec);spec.loader.exec_module(original)
rows=[]
cases=[([[0,0,0],[1,0,0],[2,0,0]],-1,0),
       ([[0,0,0],[1,1,0],[2,0,0]],.1,10),
       ([[0,0,0],[1,1,0],[2,0,0]],-1,10),
       ([[0,0,0],[0,0,0],[0,0,0]],0,0)]
for points,epsilon,spacing in cases:
    row={'points':points,'epsilon':epsilon,'spacing':spacing}
    try:
        row.update(status='returned',output=original.rdp(points,epsilon,spacing))
    except Exception as error:
        row.update(status='did_not_return_within_budget',exception=repr(error))
    rows.append(row)
report={'original_sha256':original.hashlib.sha256(original.raw).hexdigest(),
        'entry':'0x141fe0b90','instruction_budget_per_case':3000000,
        'method':'Original business entry; existing allocation/copy hooks reused. No game algorithm hook.',
        'rows':rows,
        'limit':'Budget exhaustion is not a mathematical proof of nontermination. Original PC and candidate split=0 recurrence support the hazard. Original consumer precondition/negative epsilon reachability unverified.'}
(OUT/'rdp-boundary-probe.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
print([(r['epsilon'],r['spacing'],r['status']) for r in rows])
