from pathlib import Path
import bpy, json, numpy as np
ROOT=Path(__file__).resolve().parents[1]
index=json.loads((ROOT/'reports/thumbnail-index.json').read_text())
for path in sorted((ROOT/'blends').glob('0[1-9]_*.blend')):
    bpy.ops.wm.open_mainfile(filepath=str(path),load_ui=False)
    bpy.context.window.scene=bpy.data.scenes['AssetDefinitions']
    if path.stem=='01_Vegetation':
        for col in bpy.data.collections:
            if not col.get('asset_id','').startswith('assembled/'):continue
            cage=next(o for o in col.objects if o.type=='LATTICE');meshes=[o for o in col.objects if o.type=='MESH']
            p=np.array([v.co[:] for o in meshes for v in o.data.vertices]);cage.location=(p.min(axis=0)+p.max(axis=0))/2;cage.scale=np.maximum(np.ptp(p,axis=0),.05)*1.03
            for obj in meshes:
                if not any(m.type=='LATTICE' for m in obj.modifiers):
                    m=obj.modifiers.new('EDIT_SharedTreeShape','LATTICE');m.object=cage
                    while obj.modifiers.find(m.name)>0:
                        bpy.context.view_layer.objects.active=obj;bpy.ops.object.modifier_move_up(modifier=m.name)
    count=0
    for col in bpy.data.collections:
        if col.name in index and col.asset_data:
            with bpy.context.temp_override(id=col):bpy.ops.ed.lib_id_load_custom_preview(filepath=str(ROOT/index[col.name]))
            count+=1
    bpy.context.window.scene=bpy.data.scenes['Gallery']
    bpy.ops.wm.save_as_mainfile(filepath=str(path),compress=True)
    print('FINALIZED',path.name,count,flush=True)
