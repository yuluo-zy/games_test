"""Render the saved full exhibition and nine category cameras."""
from pathlib import Path
import bpy, json, sys, time
ROOT=Path(__file__).resolve().parents[1]
plan=json.loads((ROOT/'reports/exhibition-placement.json').read_text())['renders']
args=sys.argv[sys.argv.index('--')+1:] if '--' in sys.argv else []
if '--draft' in args:
    plan=[next(p for p in plan if p['category']=='01_Vegetation')]
elif args:
    plan=[p for p in plan if p['category'] in args]
results=[]
for p in plan:
    s=bpy.data.scenes[p['scene']];bpy.context.window.scene=s
    s.render.resolution_percentage=40 if '--draft' in args else 100
    s.render.filepath=str(ROOT/p['file']) if '--draft' not in args else str(ROOT/'renders/draft-vegetation.png')
    started=time.monotonic();print('RENDER_BEGIN',p['category'],s.render.filepath,flush=True)
    bpy.ops.render.render(write_still=True,scene=s.name)
    results.append({'category':p['category'],'file':s.render.filepath,'seconds':round(time.monotonic()-started,2)})
    print('RENDER_DONE',results[-1],flush=True)
(ROOT/'reports'/('exhibition-draft.json' if '--draft' in args else 'exhibition-render-log.json')).write_text(json.dumps(results,indent=2))
