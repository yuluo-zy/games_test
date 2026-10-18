"""Independent boundary additions to the frozen original B one-shot probe."""
import json, runpy
from pathlib import Path
out = Path(__file__).resolve().parent
ns = runpy.run_path(str(out / 'native_one_shot_oracle.py'))
inputs = [('zero_over_zero', 4, 0.4, 0.0, 0.0), ('nan_magnitude', 4, 0.4, float('nan'), 1.0), ('positive_over_zero', 4, 0.4, 0.1, 0.0), ('negative_over_zero', 4, 0.4, -0.1, 0.0), ('negative_zero_magnitude', 4, 0.4, -0.0, 1.0), ('nan_height', 5, float('nan'), 0.0, 0.0)]
cases = [dict(name=name, **ns['case'](tag, value, magnitude, speed)) for name,tag,value,magnitude,speed in inputs]
(out / 'one-shot-boundaries.json').write_text(json.dumps({'target':'B','binary_sha256':ns['hashlib'].sha256(ns['raw']).hexdigest(),'method':'same original branch runner, API boundaries only; added reviewer boundary inputs','cases':cases}, indent=2) + '\n')
for case in cases:
 print(case['name'], [(c.get('name'), hex(c.get('bits',0))) for c in case['calls'] if c['op']=='parameter'])
