import bpy, addon_utils
addon_utils.enable("blender_mcp", default_set=True, persistent=True)
prefs = bpy.context.preferences.addons.get("blender_mcp")
if prefs is not None:
    prefs.preferences.telemetry_consent = False
bpy.ops.wm.save_userpref()
print("BLENDER_MCP_ENABLED", bpy.app.version_string, bool(prefs))
