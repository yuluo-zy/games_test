from pathlib import Path
import bpy, json, re, sys
from mathutils import Vector

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'scripts'))
import build_library as builder

def architecture_kits():
    path=ROOT/'blends/03_Architecture.blend';bpy.ops.wm.open_mainfile(filepath=str(path),load_ui=False)
    bpy.context.window.scene=bpy.data.scenes['AssetDefinitions'];defs=bpy.data.collections['COL_EditableDefinitions']
    entries={c['source_name']:c for c in defs.children if 'source_name' in c and '__variant_' not in c.get('asset_id','')}
    kits=[];rows=[]
    bases=[n for n in entries if re.fullmatch(r'decorators/window_(?:cottage|gothic)_\dx\d',n)]
    for base in bases:
        parts=[entries[n] for n in [base,base+'_glass',base+'_hat'] if n in entries]
        if len(parts)<2:continue
        col=bpy.data.collections.new('KIT_'+base.replace('/','_'));defs.children.link(col)
        root=bpy.data.objects.new('ROOT_KIT_'+base.replace('/','_'),None);col.objects.link(root)
        root['asset_id']='kit/'+base;root['editing_level']='分离窗框/玻璃/帽檐 + 共用形状笼'
        meshes=[]
        for part in parts:
            src=bpy.data.objects.get(part['editable_mesh']);o=src.copy();o.data=src.data.copy();o.name='SM_KIT_'+src.name
            col.objects.link(o);o.parent=root
            for mod in list(o.modifiers):o.modifiers.remove(mod)
            meshes.append(o)
        # One cage across all parts, so the frame and glass retain their shared shape.
        verts=[v.co[:] for o in meshes for v in o.data.vertices]
        temp=bpy.data.meshes.new('TMP_Bounds');temp.from_pydata(verts,[],[])
        host=bpy.data.objects.new('TMP_Bounds',temp);col.objects.link(host)
        cage=builder.shape_cage(host,col,root)
        for o in meshes:
            m=o.modifiers.new('EDIT_SharedWindowShape','LATTICE');m.object=cage
        bpy.data.objects.remove(host,do_unlink=True);bpy.data.meshes.remove(temp)
        col['asset_id']='kit/'+base;col['editable_mesh']=meshes[0].name;col['shape_controller']=cage.name
        col['assembly_status']='Name-matched parts with shared local coordinates; game deformation/attachment not reproduced'
        col.asset_mark();col.asset_data.catalog_id=builder.CAT_UUID['03_Architecture']
        col.asset_data.description='框/玻璃/帽檐分离。共用 Lattice 保持形状关联；可分别编辑每个部件。装配按同名局部坐标匹配。'
        col.asset_data.tags.new('WindowKit');kits.append((col,meshes[0],None))
        rows.append({'id':'kit/'+base,'category':'03_Architecture','collection':col.name,'assembly':True,
                     'parts':len(meshes),'vertices':sum(len(o.data.vertices) for o in meshes),'lattice':True,
                     'material_status':'Preview, shared cage; assembly inferred from matching part names'})
    scene=bpy.data.scenes.new('WindowKits');builder.add_gallery(scene,kits)
    bpy.context.window.scene=scene;bpy.ops.wm.save_as_mainfile(filepath=str(path),compress=True)
    report=json.loads((ROOT/'reports/build-report.json').read_text());report['assets']+=rows
    report['summary']['window_kits']=len(kits);report['summary']['asset_entries']+=len(kits);report['summary']['shape_cages']+=len(kits)
    (ROOT/'reports/build-report.json').write_text(json.dumps(report,ensure_ascii=False,indent=2))
    print('WINDOW_KITS',len(kits),flush=True)

def animation_clip():
    path=ROOT/'blends/08_AnimationFrames.blend';bpy.ops.wm.open_mainfile(filepath=str(path),load_ui=False)
    scene=bpy.data.scenes.new('Sheep_MeshSequence_Preview');bpy.context.window.scene=scene;scene.render.fps=24
    scene.frame_start=1;scene.frame_end=30
    meshes=[]
    for number in range(1,31):
        col=next(c for c in bpy.data.collections if c.get('asset_id')==f'sheep_animation/{number}')
        src=bpy.data.objects[col['editable_mesh']];o=src.copy();o.data=src.data.copy();o.name=f'FRAME_Sheep_{number:02d}'
        scene.collection.objects.link(o);o.parent=None
        for f in [0,number,number+1]:
            o.hide_render=(f!=number);o.hide_viewport=(f!=number)
            o.keyframe_insert(data_path='hide_render',frame=f);o.keyframe_insert(data_path='hide_viewport',frame=f)
        if o.animation_data and o.animation_data.action:
            # Boolean channels are discrete; no topology correspondence is assumed.
            o['frame_source']=f'sheep_animation/{number}'
        meshes.append(o)
    scene.frame_set(1);scene['timing_status']='仅演示编号帧顺序；24fps 是预览设置，不是恢复的原作帧率。delighted 为独立姿态。'
    bpy.ops.wm.save_as_mainfile(filepath=str(path),compress=True)
    print('SHEEP_SEQUENCE',len(meshes),flush=True)

if __name__=='__main__':architecture_kits();animation_clip()
