"""Blender batch-only starter. Creates an editable prototype, not approved art.

Run: blender --background --python art-source/tools/build-window-blockout.py
Never run in an artist's active Blender session. Existing outputs are preserved.
"""
from pathlib import Path
import json
import bpy


def linear_channel(value):
    value = value / 255.0
    return value / 12.92 if value <= 0.04045 else ((value + 0.055) / 1.055) ** 2.4


def material(name, rgb, roughness):
    result = bpy.data.materials.new(name)
    result.use_nodes = True
    shader = result.node_tree.nodes.get("Principled BSDF")
    color = tuple(linear_channel(value) for value in rgb) + (1.0,)
    shader.inputs["Base Color"].default_value = color
    shader.inputs["Roughness"].default_value = roughness
    shader.inputs["Metallic"].default_value = 0.0
    result.diffuse_color = color
    return result


def box(name, center, dimensions, surface, root, bevel=False):
    bpy.ops.mesh.primitive_cube_add(size=1.0, location=center)
    obj = bpy.context.object
    obj.name = name
    obj.dimensions = dimensions
    bpy.ops.object.transform_apply(location=False, rotation=False, scale=True)
    obj.data.materials.append(surface)
    obj.parent = root
    if bevel:
        modifier = obj.modifiers.new("edge-readability", "BEVEL")
        modifier.width = 0.008
        modifier.segments = 1
    return obj


def main():
    if not bpy.app.background:
        raise RuntimeError("Batch-only tool: refusing to clear an interactive scene")
    project = Path(__file__).resolve().parents[2]
    source = project / "art-source/models/window-basic-blockout.blend"
    exported = project / "assets/themes/warm-stone/modules/window-basic-blockout.glb"
    if source.exists() or exported.exists():
        raise FileExistsError("Prototype outputs exist; keep artist edits and use another revision")
    theme = json.loads((project / "assets/themes/warm-stone/theme.json").read_text(encoding="utf-8"))
    bpy.ops.wm.read_factory_settings(use_empty=True)
    bpy.context.scene.unit_settings.system = "METRIC"
    bpy.context.scene.unit_settings.scale_length = 1.0
    wood_data = theme["materials"]["timber"]
    wood = material("warm-stone-timber", wood_data["base_color_srgb"], wood_data["roughness"])
    dark_data = theme["materials"]["window"]
    dark = material("warm-stone-window", dark_data["base_color_srgb"], dark_data["roughness"])
    root = bpy.data.objects.new("window-basic", None)
    bpy.context.collection.objects.link(root)
    root["asset_id"] = "window-basic"
    root["fit_mode"] = "fixed-corners-repeat-center"
    root["stage"] = "blockout"
    # Blender Z-up -> glTF Y-up. Blender -Y front -> glTF +Z front.
    width, height, frame = 0.9, 1.2, 0.09
    for x_name, x in [("left", -width / 2 - frame / 2), ("right", width / 2 + frame / 2)]:
        for z_name, z in [("bottom", frame / 2), ("top", height + frame * 1.5)]:
            box(f"corner-{z_name}-{x_name}", (x, -0.04, z), (frame, 0.10, frame), wood, root, True)
        rail = box(f"rail-{x_name}", (x, -0.04, frame + height / 2), (frame, 0.10, height), wood, root, True)
        rail["fit_axis"] = "height"
    for name, z in [("bottom", frame / 2), ("top", height + frame * 1.5)]:
        rail = box(f"rail-{name}", (0, -0.04, z), (width, 0.10, frame), wood, root, True)
        rail["fit_axis"] = "width"
    box("mullion", (0, -0.04, frame + height / 2), (0.035, 0.08, height), wood, root)
    box("opaque-recess", (0, 0.08, frame + height / 2), (width, 0.015, height), dark, root)
    anchor = bpy.data.objects.new("opening-bottom", None)
    bpy.context.collection.objects.link(anchor)
    anchor.parent = root
    anchor.location = (0, 0, frame)
    source.parent.mkdir(parents=True, exist_ok=True)
    exported.parent.mkdir(parents=True, exist_ok=True)
    bpy.ops.wm.save_as_mainfile(filepath=str(source))
    bpy.ops.export_scene.gltf(filepath=str(exported), export_format="GLB", export_yup=True,
                              export_animations=False, export_extras=True, export_apply=True)
    print(f"BLOCKOUT ONLY: {source}\n{exported}")
    print("Review axes, triangle/material budget and engine attachment before updating manifest status.")


if __name__ == "__main__":
    main()
