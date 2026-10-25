import json, glob, io, os

# Scan oa2-results for rows whose addresses/names touch the six modules
mod_markers = ['asset_librar', 'utils/', 'startup', 'input', 'components', 'geometry', 'texture_watch', 'meshwatch', 'configwatch', 'dirty_tiles', 'float_grid', 'load_json', 'ssbo', 'color_librar', 'mesh_librar', 'shader_librar', 'vao_librar', 'placement_data', 'audio_trigger', 'action_set', 'input_stream', 'MainActionSet', 'transient_mesh', 'instanced_wall', 'sphere', 'cube', 'plane', 'drawable', 'prefab', 'nani_meshes', 'atlas']
out = io.open('round3/REVIEW-UNEXPLORED/_oa2_six.txt', 'w', encoding='utf-8')
for p in sorted(glob.glob('round3/oa2-results/*.json')):
    try:
        d = json.load(open(p, encoding='utf-8'))
    except Exception as e:
        out.write('%s: ERR %s\n' % (p, e))
        continue
    # discover structure
    def walk(o, path=''):
        hits = []
        if isinstance(o, dict):
            for k, v in o.items():
                hits += walk(v, path + '/' + str(k))
        elif isinstance(o, list):
            for i, v in enumerate(o):
                hits += walk(v, path + '/%d' % i)
        elif isinstance(o, str):
            for m in mod_markers:
                if m in o:
                    hits.append((path, o[:160]))
                    break
        return hits
    hits = walk(d)
    out.write('===== %s (%d hits) =====\n' % (os.path.basename(p), len(hits)))
    for path, s in hits[:30]:
        out.write('  %s | %s\n' % (path, s))
out.close()
print('done')
