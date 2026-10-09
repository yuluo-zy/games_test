"""Original warm-stone kit. Blender background only; never overwrites a revision.

blender --background --python art-source/tools/build-house-kit.py -- --revision v1
GLB and kit.json use the SAME evaluated triangles, meters, Y-up and +Z front.
No downloaded art, fonts or third-party model inputs.
"""
from pathlib import Path
import argparse
import json
import math
import sys
import bpy
import bmesh
from mathutils import Vector


def linear(v):
    v /= 255.0
    return v / 12.92 if v <= 0.04045 else ((v + 0.055) / 1.055) ** 2.4


def rgb(v):
    return tuple(linear(c) for c in v) + (1.0,)


def collection(name):
    c = bpy.data.collections.new(name)
    bpy.context.scene.collection.children.link(c)
    return c


def move(obj, col):
    for c in list(obj.users_collection):
        c.objects.unlink(obj)
    col.objects.link(obj)


def box(name, center, size, role, col, bevel=0.012):
    bpy.ops.mesh.primitive_cube_add(size=1, location=center)
    obj = bpy.context.object
    obj.name = 'SM_' + name
    obj.dimensions = size
    bpy.ops.object.transform_apply(location=True, rotation=True, scale=True)
    move(obj, col)
    obj.data.materials.append(MATERIALS[role])
    if bevel:
        m = obj.modifiers.new('ReadableEdges', 'BEVEL')
        m.width = min(bevel, min(size) * 0.2)
        m.segments = 1
        bpy.ops.object.modifier_apply(modifier=m.name)
    # World/meter projection, intentional tiling; construction seams on box edges.
    obj.data.update()
    uv = obj.data.uv_layers.active or obj.data.uv_layers.new(name='UV_MeterTile')
    for face in obj.data.polygons:
        axis = max(range(3), key=lambda a: abs(face.normal[a]))
        for li in face.loop_indices:
            p = obj.data.vertices[obj.data.loops[li].vertex_index].co
            uv.data[li].uv = (p.x, p.z) if axis == 1 else ((p.y, p.z) if axis == 0 else (p.x, p.y))
    obj['material_role'] = role
    return obj


def material(role, base):
    m = bpy.data.materials.new('MAT_WarmStone_' + role.title())
    m.use_nodes = True
    bsdf = m.node_tree.nodes.get('Principled BSDF')
    bsdf.inputs['Base Color'].default_value = rgb(base)
    bsdf.inputs['Roughness'].default_value = .82 if role != 'window' else .3
    m.diffuse_color = rgb(base)
    return m


def texture_set(role, output):
    """Bake periodic shader detail, not lighting/AO, onto a unit UV tile."""
    m = MATERIALS[role]
    tree = m.node_tree
    nodes, links = tree.nodes, tree.links
    bsdf = nodes.get('Principled BSDF')
    out = nodes.get('Material Output')
    coords = nodes.new('ShaderNodeTexCoord')
    noise = nodes.new('ShaderNodeTexNoise')
    noise.inputs['Scale'].default_value = 18
    noise.inputs['Detail'].default_value = 2
    links.new(coords.outputs['UV'], noise.inputs['Vector'])
    base = COLORS[role]
    if role == 'plaster':
        color = nodes.new('ShaderNodeMixRGB')
        color.inputs[1].default_value = rgb(tuple(max(0, c - 9) for c in base))
        color.inputs[2].default_value = rgb(tuple(min(255, c + 4) for c in base))
        links.new(noise.outputs['Fac'], color.inputs[0])
        color_out, height = color.outputs[0], noise.outputs['Fac']
    elif role == 'timber':
        separate = nodes.new('ShaderNodeSeparateXYZ')
        links.new(coords.outputs['UV'], separate.inputs[0])
        repeat = nodes.new('ShaderNodeMath')
        repeat.operation = 'MULTIPLY'
        repeat.inputs[1].default_value = 5
        links.new(separate.outputs['X'], repeat.inputs[0])
        fraction = nodes.new('ShaderNodeMath')
        fraction.operation = 'FRACT'
        links.new(repeat.outputs[0], fraction.inputs[0])
        seam = nodes.new('ShaderNodeMath')
        seam.operation = 'LESS_THAN'
        seam.inputs[1].default_value = .035
        links.new(fraction.outputs[0], seam.inputs[0])
        color = nodes.new('ShaderNodeMixRGB')
        color.inputs[1].default_value = rgb(base)
        color.inputs[2].default_value = rgb(tuple(max(0,c-24) for c in base))
        links.new(seam.outputs[0], color.inputs[0])
        color_out, height = color.outputs[0], seam.outputs[0]
    else:
        brick = nodes.new('ShaderNodeTexBrick')
        links.new(coords.outputs['UV'], brick.inputs['Vector'])
        brick.inputs['Scale'].default_value = 1
        brick.inputs['Color1'].default_value = rgb(tuple(min(255, c + 10) for c in base))
        brick.inputs['Color2'].default_value = rgb(tuple(max(0, c - 13) for c in base))
        brick.inputs['Mortar'].default_value = rgb(tuple(max(0, c - 30) for c in base))
        brick.inputs['Mortar Size'].default_value = .012 if role != 'timber' else .006
        brick.inputs['Mortar Smooth'].default_value = .008
        brick.inputs['Brick Width'].default_value = {'stone': .5, 'roof': .25, 'timber': 1}[role]
        brick.inputs['Row Height'].default_value = {'stone': .25, 'roof': .2, 'timber': .18}[role]
        brick.offset = 0 if role == 'timber' else .5
        color_out, height = brick.outputs['Color'], brick.outputs['Fac']
    bump = nodes.new('ShaderNodeBump')
    bump.inputs['Strength'].default_value = .32
    bump.inputs['Distance'].default_value = .008 if role == 'plaster' else .022
    links.new(height, bump.inputs['Height'])
    links.new(bump.outputs['Normal'], bsdf.inputs['Normal'])
    links.new(color_out, bsdf.inputs['Base Color'])
    bpy.ops.mesh.primitive_plane_add(size=2)
    plane = bpy.context.object
    plane.data.materials.append(m)
    # Plane primitive UV is exactly 0..1; bake 1m material tile.
    bake_node = nodes.new('ShaderNodeTexImage')
    generated = {}
    for kind, size in [('BC', 1024), ('N', 512), ('ORM', 512)]:
        image = bpy.data.images.new(f'T_WarmStone_{role.title()}_{kind}', width=size, height=size, alpha=False)
        image.colorspace_settings.name = 'sRGB' if kind == 'BC' else 'Non-Color'
        bake_node.image = image
        nodes.active = bake_node
        if kind == 'BC':
            bpy.ops.object.bake(type='DIFFUSE', pass_filter={'COLOR'}, margin=16)
        elif kind == 'N':
            bpy.ops.object.bake(type='NORMAL', margin=16)
        else:
            emission = nodes.new('ShaderNodeEmission')
            emission.inputs['Color'].default_value = (1, .82, 0, 1)
            links.new(emission.outputs[0], out.inputs['Surface'])
            bpy.ops.object.bake(type='EMIT', margin=16)
            nodes.remove(emission)
            links.new(bsdf.outputs[0], out.inputs['Surface'])
        image.filepath_raw = str(output / (image.name + '.png'))
        image.file_format = 'PNG'
        image.save()
        generated[kind] = image
    bpy.data.objects.remove(plane, do_unlink=True)
    # Replace unsupported procedural shader with actual baked engine textures.
    nodes.clear()
    shader = nodes.new('ShaderNodeBsdfPrincipled')
    surface = nodes.new('ShaderNodeOutputMaterial')
    links.new(shader.outputs[0], surface.inputs[0])
    textures = {}
    for kind, image in generated.items():
        t = nodes.new('ShaderNodeTexImage')
        t.image, t.extension = image, 'REPEAT'
        textures[kind] = t
    links.new(textures['BC'].outputs['Color'], shader.inputs['Base Color'])
    normal = nodes.new('ShaderNodeNormalMap')
    links.new(textures['N'].outputs['Color'], normal.inputs['Color'])
    links.new(normal.outputs[0], shader.inputs['Normal'])
    separate = nodes.new('ShaderNodeSeparateColor')
    links.new(textures['ORM'].outputs['Color'], separate.inputs[0])
    links.new(separate.outputs['Green'], shader.inputs['Roughness'])
    links.new(separate.outputs['Blue'], shader.inputs['Metallic'])


def window(col, shutters=False):
    w, h = .9, 1.2
    for x in [-w / 2 - .055, w / 2 + .055]:
        box('Window_Jamb', (x, -.065, h/2), (.11, .16, h+.22), 'timber', col)
    for z in [-.055, h+.055]:
        box('Window_Rail', (0, -.065, z), (w, .16, .11), 'timber', col)
    box('Window_Mullion', (0, -.085, h/2), (.042, .095, h), 'timber', col, .005)
    box('Window_Crossbar', (0, -.085, h*.58), (w, .095, .04), 'timber', col, .005)
    box('Window_Recess', (0, .205, h/2), (w, .02, h), 'window', col, 0)
    box('Window_Sill', (0, -.135, -.085), (w+.32, .34, .075), 'timber', col)
    if shutters:
        for sign in [-1, 1]:
            x = sign * (w/2 + .27)
            box('Window_Shutter', (x, -.045, h/2), (.32, .065, h+.03), 'timber', col)
            for z in [.2, .6, 1.0]:
                box('Window_ShutterRail', (x, -.085, z), (.32, .035, .045), 'timber', col, 0)


def door(col):
    for x in [-.54, .54]:
        box('Door_Jamb', (x, -.095, 1), (.18, .22, 2.18), 'timber', col)
    box('Door_Lintel', (0, -.095, 2.045), (.9, .22, .18), 'timber', col)
    for i in range(6):
        box('Door_Plank', (-.375+i*.15, -.035, .98), (.143, .065, 1.96), 'timber', col, 0)
    for z in [.28, 1.62]:
        box('Door_Strap', (0, -.08, z), (.83, .02, .045), 'window', col, 0)
    box('Door_Handle', (.30, -.11, 1.0), (.055, .075, .12), 'window', col, .01)
    box('Door_Threshold', (0, -.13, .045), (1.25, .4, .09), 'timber', col)


def chimney(col):
    for x in [-.24, .24]:
        box('Chimney_ShaftSide', (x, 0, .75), (.12, .60, 1.5), 'stone', col)
    for y in [-.24, .24]:
        box('Chimney_ShaftEnd', (0, y, .75), (.36, .12, 1.5), 'stone', col)
    for x in [-.30, .30]:
        box('Chimney_CapSide', (x, 0, 1.45), (.16, .76, .16), 'stone', col)
    for y in [-.30, .30]:
        box('Chimney_CapEnd', (0, y, 1.45), (.44, .16, .16), 'stone', col)


def audit_and_compile(col, max_triangles, max_materials):
    batches = {}
    count = 0
    for obj in col.objects:
        assert obj.name.startswith('SM_') and obj.type == 'MESH'
        assert tuple(obj.scale) == (1, 1, 1) and max(abs(v) for v in obj.rotation_euler) < .0001
        bm = bmesh.new()
        bm.from_mesh(obj.data)
        assert all(e.is_manifold for e in bm.edges), obj.name + ': non-manifold component'
        bm.free()
        mesh = obj.data
        mesh.calc_loop_triangles()
        role = obj['material_role']
        data = batches.setdefault(role, dict(positions=[], normals=[], uvs=[], indices=[], fit=[]))
        for tri in mesh.loop_triangles:
            p = [mesh.vertices[mesh.loops[li].vertex_index].co for li in tri.loops]
            assert (p[1]-p[0]).cross(p[2]-p[0]).length > 1e-9
            for li in tri.loops:
                position = mesh.vertices[mesh.loops[li].vertex_index].co
                normal = mesh.polygons[tri.polygon_index].normal
                data['indices'].append(len(data['positions']))
                data['positions'].append([round(position.x, 6), round(position.z, 6), round(-position.y, 6)])
                data['normals'].append([round(normal.x, 6), round(normal.z, 6), round(-normal.y, 6)])
                u, v = mesh.uv_layers.active.data[li].uv
                data['uvs'].append([round(u, 6), round(1-v, 6)])
                # Pane & mullion are proportional; outside corners retain thickness.
                data['fit'].append('center' if any(s in obj.name for s in ['Recess', 'Mullion', 'Crossbar']) else 'border')
            count += 1
    assert count <= max_triangles, (col.name, count, max_triangles)
    assert len(batches) <= max_materials
    return {'batches': batches}, {'triangles': count, 'materials': len(batches), 'closed_components': True, 'applied_transforms': True, 'uv': 'intentional meter tiling'}


def main():
    global MATERIALS, COLORS
    if not bpy.app.background:
        raise RuntimeError('Background only: preserving interactive artist scenes')
    parser = argparse.ArgumentParser()
    parser.add_argument('--revision', default='v1')
    args = parser.parse_args(sys.argv[sys.argv.index('--')+1:] if '--' in sys.argv else [])
    if not args.revision.isalnum():
        raise ValueError('Revision must be alphanumeric')
    project = Path(__file__).resolve().parents[2]
    output = project / 'assets/themes/warm-stone' / ('house-kit-' + args.revision)
    source = project / 'art-source/models' / ('house-kit-' + args.revision + '.blend')
    if output.exists() or source.exists():
        raise FileExistsError('Revision exists; choose a new revision, do not overwrite artist work')
    output.mkdir(parents=True)
    source.parent.mkdir(parents=True, exist_ok=True)
    bpy.ops.wm.read_factory_settings(use_empty=True)
    scene = bpy.context.scene
    scene.unit_settings.system = 'METRIC'
    scene.unit_settings.scale_length = 1
    scene.render.engine = 'CYCLES'
    scene.cycles.samples = 8
    COLORS = dict(stone=(205,193,165), plaster=(232,218,191), timber=(139,102,68), roof=(113,134,143), window=(44,60,61))
    MATERIALS = {k: material(k, v) for k, v in COLORS.items()}
    for role in ['stone', 'plaster', 'timber', 'roof']:
        texture_set(role, output)
    modules, report = {}, {}
    for name, build, budget, mats in [('window-basic', lambda c: window(c), 500, 2), ('window-shutter', lambda c: window(c, True), 800, 2), ('door-timber', door, 500, 2), ('chimney-stone', chimney, 500, 1)]:
        col = collection('COL_' + name.replace('-', '_'))
        build(col)
        modules[name], report[name] = audit_and_compile(col, budget, mats)
        bpy.ops.object.select_all(action='DESELECT')
        for obj in col.objects:
            obj.select_set(True)
        bpy.context.view_layer.objects.active = list(col.objects)[0]
        bpy.ops.export_scene.gltf(filepath=str(output / (name+'.glb')), export_format='GLB', use_selection=True, export_yup=True, export_animations=False, export_extras=True, export_apply=True, export_texcoords=True, export_normals=True, export_tangents=True)
        # Low-cost editable LOD proxy in source; runtime selection is NOT claimed.
        lod = collection('COL_LOD_' + name.replace('-', '_'))
        for obj in col.objects:
            proxy = obj.copy()
            proxy.data = obj.data.copy()
            proxy.name = obj.name + '_LOD1'
            lod.objects.link(proxy)
            decimate = proxy.modifiers.new('LOD_Reduction', 'DECIMATE')
            decimate.ratio = .5
        lod.hide_render = True
        lod.hide_viewport = True
    compiled = dict(schema_version=1, units='meter', up_axis='+Y', front_axis='+Z', window_opening=[.9, 1.2], modules=modules)
    (output / 'kit.json').write_text(json.dumps(compiled, separators=(',', ':')), encoding='utf-8')
    (output / 'validation.json').write_text(json.dumps(dict(blender=bpy.app.version_string, assets=report, textures={'BC':1024, 'N':512, 'ORM':512}, source='project-original, no third-party inputs'), indent=2), encoding='utf-8')
    # Source preview array separated from export data; origin of exported modules stays zero.
    reference = collection('COL_Reference')
    box('Scale_180cm', (4, 0, .9), (.4, .3, 1.8), 'plaster', reference, .08)
    for index, name in enumerate(modules):
        original = bpy.data.collections['COL_' + name.replace('-', '_')]
        original.hide_render = True
        original.hide_viewport = True
        preview = collection('COL_Preview_' + name.replace('-', '_'))
        for obj in original.objects:
            duplicate = obj.copy()
            duplicate.data = obj.data
            duplicate.location = (-3 + index*2, 0, 0)
            preview.objects.link(duplicate)
    # Render a genuine Blender inspection sheet, not a generated concept image.
    scene.render.engine = 'CYCLES'
    scene.cycles.samples = 32
    scene.world = bpy.data.worlds.new('WarmStudio')
    scene.world.use_nodes = True
    scene.world.node_tree.nodes['Background'].inputs[0].default_value = (.30, .36, .40, 1)
    lights = collection('COL_Lights')
    for loc, energy, size in [((0,-4,6), 900, 5), ((3,3,4), 650, 4)]:
        bpy.ops.object.light_add(type='AREA', location=loc)
        light = bpy.context.object
        light.name = 'LGT_Studio'
        light.data.energy, light.data.shape, light.data.size = energy, 'DISK', size
        light.rotation_euler = (Vector((0,0,.7))-light.location).to_track_quat('-Z','Y').to_euler()
        move(light, lights)
    cameras = collection('COL_Cameras')
    bpy.ops.object.camera_add(location=(5,-10,5))
    camera = bpy.context.object
    camera.name = 'CAM_KitReview'
    camera.rotation_euler = (Vector((.5,0,.9))-camera.location).to_track_quat('-Z','Y').to_euler()
    camera.data.type, camera.data.ortho_scale = 'ORTHO', 10
    move(camera, cameras)
    scene.camera = camera
    scene.render.resolution_x, scene.render.resolution_y, scene.render.resolution_percentage = 1200, 650, 100
    scene.render.filepath = str(project / 'art-source/reference' / ('house-kit-'+args.revision+'-blender.png'))
    for image in bpy.data.images:
        if image.filepath:
            image.filepath = bpy.path.relpath(image.filepath, start=str(source.parent))
    bpy.ops.wm.save_as_mainfile(filepath=str(source))
    bpy.ops.render.render(write_still=True)
    print('KIT_READY ' + json.dumps(report))


if __name__ == '__main__':
    main()
