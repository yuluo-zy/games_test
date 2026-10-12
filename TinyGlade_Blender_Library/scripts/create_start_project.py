from pathlib import Path
import bpy, json, math
from mathutils import Vector
ROOT=Path(__file__).resolve().parents[1]
REPORT=json.loads((ROOT/'reports/build-report.json').read_text());byid={a['id']:a for a in REPORT['assets']}
PICKS=['assembled/mid_tree_v4','assembled/mid_distance_tree_v1','naked_tree_1_no_mushrooms',
       'kit/decorators/window_gothic_2x1','kit/decorators/window_cottage_2x1','decorators/door',
       'clutter/bench','clutter/watering_can','clutter/plant_pot_v1','garden_stone','roof_tile','small_bird']
PICKS=[p for p in PICKS if p in byid]
bpy.ops.wm.read_factory_settings(use_empty=True);scene=bpy.context.scene;scene.name='StartHere_素材库入口'
linked={}
for cat in sorted({byid[p]['category'] for p in PICKS}):
    names=[byid[p]['collection'] for p in PICKS if byid[p]['category']==cat]
    with bpy.data.libraries.load(str(ROOT/'blends'/(cat+'.blend')),link=True) as (source,target):target.collections=names
    for c in target.collections:linked[c.name]=c

for i,p in enumerate(PICKS):
    row=byid[p];col=linked[row['collection']];o=bpy.data.objects.new('PREVIEW_'+p.replace('/','_'),None)
    o.instance_type='COLLECTION';o.instance_collection=col;scene.collection.objects.link(o)
    coords=[v.co[:] for obj in col.all_objects if obj.type=='MESH' for v in obj.data.vertices]
    low=[min(v[k] for v in coords) for k in range(3)];high=[max(v[k] for v in coords) for k in range(3)]
    scale=2.8/max(max(high[k]-low[k] for k in range(3)),.1);o.scale=(scale,)*3
    o.location=((i%4)*4,(i//4)*4,-low[2]*scale+.1);o['asset_id']=p;o['usage']='N → TG 素材库 → 制作可编辑副本'
    c=bpy.data.curves.new('Label_'+str(i),'FONT');c.body=p.split('/')[-1][:26];c.size=.18
    text=bpy.data.objects.new('LABEL_'+str(i),c);scene.collection.objects.link(text);text.location=(o.location.x-1.4,o.location.y-1.4,.04)
    mat=bpy.data.materials.get('MAT_Label') or bpy.data.materials.new('MAT_Label');mat.diffuse_color=(.78,.84,.90,1);c.materials.append(mat)

bpy.ops.mesh.primitive_plane_add(size=200,location=(6,4,-.01));floor=bpy.context.object;floor.name='DISPLAY_Ground'
mat=bpy.data.materials.new('MAT_DisplayGround');mat.diffuse_color=(.065,.08,.105,1);mat.use_nodes=True
mat.node_tree.nodes.get('Principled BSDF').inputs['Base Color'].default_value=mat.diffuse_color;floor.data.materials.append(mat)
camera=bpy.data.cameras.new('Camera_Overview');o=bpy.data.objects.new('Camera_Overview',camera);scene.collection.objects.link(o)
o.location=(20,-25,26);target=Vector((6,4,1));o.rotation_euler=(target-o.location).to_track_quat('-Z','Y').to_euler();camera.type='ORTHO';camera.ortho_scale=20;scene.camera=o
for name,loc,power,size in [('Key',(3,-6,17),2800,10),('Fill',(15,5,15),2200,9),('Rim',(-5,10,13),1800,8)]:
    data=bpy.data.lights.new(name,'AREA');data.energy=power;data.shape='DISK';data.size=size
    light=bpy.data.objects.new(name,data);scene.collection.objects.link(light);light.location=loc;light.rotation_euler=(target-light.location).to_track_quat('-Z','Y').to_euler()
scene.world=bpy.data.worlds.new('World_Display');scene.world.use_nodes=True;scene.world.node_tree.nodes.get('Background').inputs[0].default_value=(.18,.22,.28,1)
scene.world.node_tree.nodes.get('Background').inputs[1].default_value=.5;scene.render.engine='BLENDER_EEVEE'
scene.render.resolution_x=1600;scene.render.resolution_y=1200;scene.render.resolution_percentage=100
scene.view_settings.view_transform='AgX';scene.unit_settings.system='METRIC'
text=bpy.data.texts.new('阅读我_README');text.write('''这是素材库入口工程，包含跨分类资产的预览实例。

开始编辑：选一个 PREVIEW 对象 → N 侧栏 → TG 素材库 → 制作可编辑副本。
复制后网格、控制笼、骨架独立，原始资产不受影响。
整体形状：编辑整体形状笼 → Tab → 移动 Lattice 控制点。
树枝：调整现有枝条 → Pose Mode → 移动/旋转骨骼。
新增枝条：选骨骼 → 新增可编辑枝条 → Tab 编辑曲线；修改器 Branch Radius 调整粗细。
删除已有枝条：查看影响的顶点并核对后手工编辑；推断权重不是准确的枝条分区。

完整素材：Asset Browser → Tiny Glade Editable；可拖入任意工程。
原始模型：分类文件的 SourceArchive 场景，原坐标与属性未改。
编辑模板：分类文件的 AssetDefinitions 场景。
Gallery：分类预览；WindowKits：组合门窗预览；Sheep_MeshSequence_Preview：编号帧预览。

树骨架、叶冠权重、部分装配关系属于重建推断。材质为编辑预览，原作着色器未完整复现。
所有模型来源、差异、验证和原始数据位于此文件夹的 reports 与 source 中。
''')
for screen in bpy.data.screens:
    for area in screen.areas:
        if area.type=='VIEW_3D':
            area.spaces.active.region_3d.view_perspective='CAMERA';area.spaces.active.shading.color_type='MATERIAL'
            area.spaces.active.overlay.show_extras=False
# Render before assigning an editable mesh selection; keep the opener focused on tree editing.
bpy.ops.object.select_all(action='DESELECT');active=next(o for o in scene.objects if o.get('asset_id')=='assembled/mid_tree_v4')
active.select_set(True);bpy.context.view_layer.objects.active=active
out=ROOT/'blends/00_StartHere.blend';bpy.ops.wm.save_as_mainfile(filepath=str(out),compress=True)
for lib in bpy.data.libraries:lib.filepath=bpy.path.relpath(lib.filepath,start=str(ROOT/'blends'))
bpy.ops.wm.save_as_mainfile(filepath=str(out),compress=True)
scene.render.filepath=str(ROOT/'previews/library-overview.png');bpy.ops.render.render(write_still=True)
print('START_PROJECT_CREATED',len(PICKS),flush=True)
