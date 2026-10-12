"""Add the full 573-asset exhibition to the existing entrance project.

Run: Blender --background blends/00_StartHere.blend --python this_file
Source collection assets are linked, never changed. Display proxies are separate.
"""
from pathlib import Path
import bpy, json, math, shutil, csv
import numpy as np
from mathutils import Vector

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'blends/00_StartHere.blend'
REPORT = json.loads((ROOT / 'reports/build-report.json').read_text())
ROWS = REPORT['assets']
SPECS = {
    '01_Vegetation': ('TREES & SHRUBS', 12, 0, 60, (.32, .55, .20, 1)),
    '02_Plants': ('PLANTS & FLOWERS', 12, 58, 60, (.49, .67, .24, 1)),
    '06_Animals': ('ANIMALS', 12, 0, 86, (.85, .64, .35, 1)),
    '08_AnimationFrames': ('SHEEP / MESH FRAMES', 12, 116, 74, (.86, .71, .44, 1)),
    '03_Architecture': ('ARCHITECTURE', 12, 58, 0, (.81, .56, .34, 1)),
    '07_Other': ('OTHER RESOURCES', 12, 58, 82, (.53, .64, .78, 1)),
    '04_Props': ('PROPS & FURNITURE', 12, 116, 0, (.73, .46, .29, 1)),
    '05_Terrain': ('TERRAIN & ROCKS', 12, 116, 48, (.44, .57, .43, 1)),
    '09_Helpers': ('HELPERS / TECHNICAL', 12, 0, 0, (.21, .69, .78, 1)),
}

def mat(name, color, emission=0):
    m = bpy.data.materials.get(name) or bpy.data.materials.new(name)
    m.use_nodes = True; m.diffuse_color = color
    bs = m.node_tree.nodes.get('Principled BSDF')
    bs.inputs['Base Color'].default_value = color
    bs.inputs['Roughness'].default_value = .85
    if emission:
        bs.inputs['Emission Color'].default_value = color
        bs.inputs['Emission Strength'].default_value = emission
    return m

def box(name, low, high, col, material):
    lo, hi = low, high
    verts = [(x,y,z) for z in (lo[2],hi[2]) for y in (lo[1],hi[1]) for x in (lo[0],hi[0])]
    mesh = bpy.data.meshes.new(name)
    mesh.from_pydata(verts, [], [(0,2,3,1),(4,5,7,6),(0,1,5,4),(2,6,7,3),(0,4,6,2),(1,3,7,5)])
    o = bpy.data.objects.new(name, mesh); col.objects.link(o); mesh.materials.append(material)
    return o

def label(name, text, loc, size, col, material):
    c = bpy.data.curves.new(name, 'FONT'); c.body = text; c.size = size
    c.align_x = 'LEFT'; c.space_character = 1.05
    o = bpy.data.objects.new(name, c); col.objects.link(o); o.location = loc
    c.materials.append(material); return o

def bounds(col):
    arrays = []
    for o in col.all_objects:
        if o.type != 'MESH' or not len(o.data.vertices): continue
        v = np.empty(len(o.data.vertices) * 3, dtype=np.float32)
        o.data.vertices.foreach_get('co', v); v = v.reshape(-1, 3)
        matrix = np.array(o.matrix_world)
        arrays.append(v @ matrix[:3,:3].T + matrix[:3,3])
    a = np.concatenate(arrays)
    return a.min(axis=0), a.max(axis=0)

def helper_proxy(source, inst, col, material):
    """Render original edges and isolated points as separate tubes / octahedra."""
    curves = bpy.data.curves.new('DISPLAY_Edges_' + inst.name, 'CURVE')
    curves.dimensions = '3D'; curves.bevel_depth = .009
    curves.bevel_resolution = 0; curves.resolution_u = 1
    isolated = []
    for o in source.all_objects:
        if o.type != 'MESH': continue
        coords = [inst.matrix_world @ (o.matrix_world @ v.co) for v in o.data.vertices]
        used = set()
        for e in o.data.edges:
            a, b = e.vertices; used.update((a,b))
            if (coords[a]-coords[b]).length < 1e-7: continue
            s = curves.splines.new('POLY'); s.points.add(1)
            s.points[0].co = (*coords[a],1); s.points[1].co = (*coords[b],1)
        isolated.extend(coords[i] for i in range(len(coords)) if i not in used)
    if len(curves.splines):
        obj = bpy.data.objects.new('DISPLAY_Wire_' + inst.name, curves)
        col.objects.link(obj); curves.materials.append(material)
        obj['display_only'] = True; obj['asset_id'] = inst['asset_id']
    else: bpy.data.curves.remove(curves)
    if isolated:
        verts, faces = [], []
        offsets = [(0,0,.035),(0,0,-.035),(.035,0,0),(0,.035,0),(-.035,0,0),(0,-.035,0)]
        for p in isolated:
            start = len(verts); verts.extend([tuple(p+Vector(q)) for q in offsets])
            faces.extend([tuple(start+k for k in f) for f in [(0,2,3),(0,3,4),(0,4,5),(0,5,2),(1,3,2),(1,4,3),(1,5,4),(1,2,5)]])
        mesh = bpy.data.meshes.new('DISPLAY_Points'); mesh.from_pydata(verts,[],faces)
        obj = bpy.data.objects.new('DISPLAY_Points_' + inst.name,mesh);col.objects.link(obj)
        mesh.materials.append(material);obj['display_only']=True;obj['asset_id']=inst['asset_id']

def setup_scene(scene, environment, zone=None):
    scene.collection.children.link(environment)
    if zone: scene.collection.children.link(zone)
    scene.world = bpy.data.worlds.get('World_Exhibition')
    scene.render.engine='BLENDER_EEVEE'
    scene.render.image_settings.file_format='PNG';scene.render.image_settings.color_mode='RGBA'
    scene.render.resolution_percentage=100
    scene.view_settings.view_transform='AgX'
    scene.render.film_transparent=False
    scene.unit_settings.system='METRIC'
    scene.render.image_settings.color_depth='8'

def camera(scene, name, low, high, width, height):
    center = (Vector(low)+Vector(high))/2
    data = bpy.data.cameras.new(name);data.type='ORTHO';data.clip_end=3000
    o=bpy.data.objects.new(name,data);scene.collection.objects.link(o)
    o.location=center+Vector((0,-150,260))
    o.rotation_euler=(center-o.location).to_track_quat('-Z','Y').to_euler()
    rot=o.rotation_euler.to_matrix().inverted()
    pts=[rot @ (Vector((x,y,z))-center) for x in (low[0],high[0]) for y in (low[1],high[1]) for z in (low[2],high[2])]
    xmin,xmax=min(p.x for p in pts),max(p.x for p in pts)
    ymin,ymax=min(p.y for p in pts),max(p.y for p in pts)
    o.location+=o.rotation_euler.to_matrix() @ Vector(((xmin+xmax)/2,(ymin+ymax)/2,0))
    # Blender orthographic scale is the width at landscape resolution.
    aspect=width/height;data.ortho_scale=max(xmax-xmin,(ymax-ymin)*aspect)*1.07
    scene.camera=o;scene.render.resolution_x=width;scene.render.resolution_y=height
    return o

def main():
    if 'AllAssets_573_全素材展览' in bpy.data.scenes:
        raise RuntimeError('Exhibition already exists; use render_exhibition.py to render again.')
    backup=ROOT/'reports/backups/00_StartHere.before-exhibition.blend'
    if not backup.exists():shutil.copy2(OUT,backup)
    allscene=bpy.data.scenes.new('AllAssets_573_全素材展览')
    bpy.context.window.scene=allscene
    environment=bpy.data.collections.new('EXHIBITION_Environment')
    world=bpy.data.worlds.new('World_Exhibition');world.use_nodes=True
    world.node_tree.nodes.get('Background').inputs[0].default_value=(.36,.42,.5,1)
    world.node_tree.nodes.get('Background').inputs[1].default_value=.65
    setup_scene(allscene,environment)
    floor_mat=mat('EX_MAT_Floor',(.075,.095,.12,1))
    panel_mat=mat('EX_MAT_Panel',(.027,.043,.065,1))
    tile_mat=mat('EX_MAT_Tile',(.060,.079,.105,1))
    white=mat('EX_MAT_Label',(.82,.89,.97,1),.6)
    muted=mat('EX_MAT_Muted',(.42,.52,.65,1),.35)
    box('EX_Ground',(-30,-30,-.22),(185,170,-.18),environment,floor_mat)
    for name,loc,energy,angle in [('EX_Key',(-70,-100,160),2.5,.18),('EX_Fill',(140,70,140),.9,.3)]:
        d=bpy.data.lights.new(name,'SUN');d.energy=energy;d.angle=angle
        o=bpy.data.objects.new(name,d);environment.objects.link(o);o.location=loc
        o.rotation_euler=(Vector((65,60,0))-o.location).to_track_quat('-Z','Y').to_euler()
    placements=[];plan=[];all_bounds=[];number=0
    for cat in sorted(SPECS):
        title,cols,origin_x,origin_y,color=SPECS[cat]
        assets=sorted([r for r in ROWS if r['category']==cat],key=lambda r:(not r.get('assembly',False),r['id']))
        # Numeric sheep frames retain their meaningful sequence ordering.
        if cat=='08_AnimationFrames':
            assets.sort(key=lambda r: int(r['id'].split('/')[-1]) if r['id'].split('/')[-1].isdigit() else 999)
        with bpy.data.libraries.load(str(ROOT/'blends'/(cat+'.blend')),link=True) as (source,target):
            target.collections=[r['collection'] for r in assets]
        linked={c.name:c for c in target.collections}
        zone=bpy.data.collections.new('EXHIBITION_'+cat);allscene.collection.children.link(zone)
        col_assets=bpy.data.collections.new('EX_Instances_'+cat);zone.children.link(col_assets)
        col_display=bpy.data.collections.new('EX_Display_'+cat);zone.children.link(col_display)
        accent=mat('EX_Accent_'+cat,color,.3)
        rows=math.ceil(len(assets)/cols);w=cols*4.3+2;h=rows*4.3+5
        x,y=origin_x,origin_y
        box('EX_Panel_'+cat,(x,y,0),(x+w,y+h,.15),col_display,panel_mat)
        box('EX_Stripe_'+cat,(x+.6,y+.6,.16),(x+w-.6,y+.7,.17),col_display,accent)
        label('EX_Title_'+cat,title,(x+1,y+1.2,.18),.8,col_display,white)
        label('EX_Subtitle_'+cat,f'{len(assets):03d} ASSETS  /  SOURCE GEOMETRY + EDIT CONTROLS',(x+1,y+2.5,.18),.22,col_display,muted)
        for i,r in enumerate(assets):
            number+=1;col=linked[r['collection']]
            lo,hi=bounds(col);span=np.maximum(hi-lo,1e-6)
            scale=2.65/max(float(max(span)),.10)
            cx=x+3+(i%cols)*4.3;cy=y+5.5+(i//cols)*4.3
            inst=bpy.data.objects.new(f'EX_ASSET_{number:03d}_'+r['id'].replace('/','_'),None)
            inst.instance_type='COLLECTION';inst.instance_collection=col;col_assets.objects.link(inst)
            inst.scale=(scale,)*3
            inst.location=(cx-(lo[0]+hi[0])*scale/2,cy-(lo[1]+hi[1])*scale/2,.26-lo[2]*scale)
            inst['asset_id']=r['id'];inst['exhibition_number']=number
            inst['display_scale']=scale;inst['usage']='N → TG 素材库 → 制作可编辑副本'
            box(f'EX_Tile_{number:03d}',(cx-1.8,cy-1.85,.16),(cx+1.8,cy+1.8,.20),col_display,tile_mat)
            name=r['id'].split('/')[-1];short=name[:29]
            label(f'EX_Label_{number:03d}',f'{number:03d}  {short}',(cx-1.72,cy-1.64,.215),.145,col_display,white)
            # Linked helpers remain available for editing; render only their display wires.
            if cat=='09_Helpers':
                inst.hide_render=True;bpy.context.view_layer.update()
                helper_proxy(col,inst,col_display,accent)
            placements.append({'number':number,'id':r['id'],'category':cat,'object':inst.name,'collection':col.name,'scale':scale,'location':list(inst.location),'helper_display_proxy':cat=='09_Helpers'})
        scene=bpy.data.scenes.new('Exhibit_'+cat);setup_scene(scene,environment,zone)
        low=(x-.3,y-.3,0);high=(x+w+.3,y+h+.3,3.2)
        width=max(1800,min(4000,cols*320))
        height=max(1300,round(width*(h*.87+2)/w))
        camera(scene,'CAM_'+cat,low,high,width,height)
        filename=cat+'.png';scene.render.filepath=str(ROOT/'renders'/filename)
        plan.append({'scene':scene.name,'category':cat,'count':len(assets),'file':'renders/'+filename,'width':width,'height':height})
        all_bounds.extend([low,high]);print('ZONE_READY',cat,len(assets),flush=True)
    lo=np.min(all_bounds,axis=0);hi=np.max(all_bounds,axis=0)
    camera(allscene,'CAM_AllAssets_Overview',lo,hi,5200,3000)
    allscene.render.filepath=str(ROOT/'renders/all-assets-overview.png')
    plan.insert(0,{'scene':allscene.name,'category':'ALL','count':len(placements),'file':'renders/all-assets-overview.png','width':5200,'height':3000})
    actual=[o['asset_id'] for o in allscene.objects if o.instance_type=='COLLECTION' and o.get('exhibition_number')]
    expected=[r['id'] for r in ROWS]
    assert sorted(actual)==sorted(expected), 'Asset IDs missing or duplicated'
    allscene['asset_count']=len(actual);allscene['display_note']='Uniform display normalization. Original asset geometry unchanged. Helpers rendered as separate wires/points.'
    bpy.context.window.scene=allscene
    for screen in bpy.data.screens:
        for area in screen.areas:
            if area.type=='VIEW_3D':
                area.spaces.active.region_3d.view_perspective='CAMERA'
                area.spaces.active.shading.color_type='MATERIAL';area.spaces.active.overlay.show_extras=False
    for lib in bpy.data.libraries:lib.filepath=bpy.path.relpath(lib.filepath,start=str(ROOT/'blends'))
    text=bpy.data.texts.new('全素材展览_README')
    text.write('全部 573 项素材以集合实例分为 9 个展区。\n当前场景：AllAssets_573_全素材展览。\nExhibit_01 至 Exhibit_09 为分类视图，均已设置相机。\n选 EX_ASSET 对象 → N → TG 素材库 → 制作可编辑副本。\n展览为了便于观察统一了显示大小；原始资产的尺寸与结构保持不变。\n辅助资源以展示用线框/点标记渲染，原始点/边/面未补改。\n编号、资源 ID、位置见 reports/exhibition-placement.json。\n高清图见 renders/，原入口示例场景保留。\n')
    bpy.ops.wm.save_as_mainfile(filepath=str(OUT),compress=True)
    (ROOT/'reports/exhibition-placement.json').write_text(json.dumps({'count':len(actual),'preserved_scene':'StartHere_素材库入口','assets':placements,'renders':plan},indent=2,ensure_ascii=False))
    with (ROOT/'reports/exhibition-index.csv').open('w',newline='') as f:
        writer=csv.DictWriter(f,fieldnames=['number','id','category','object']);writer.writeheader()
        writer.writerows({k:r[k] for k in writer.fieldnames} for r in placements)
    print('EXHIBITION_SAVED',len(actual),str(OUT),flush=True)

if __name__=='__main__':main()
