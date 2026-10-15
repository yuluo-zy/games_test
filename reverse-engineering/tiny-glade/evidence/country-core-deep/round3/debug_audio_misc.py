import os
CH = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters"
KEYS = ['audio', 'steam', 'debug', 'shutdown', 'feedback_report', 'gpu_lut', 'grading_lut',
        'color_icon', 'color_id', 'gothic_window_profile', 'glade_color_customization',
        'image_formation', 'placement_data', 'texture_presets', 'render_ssbo',
        'shader_refactoring_state', 'window', 'editing_highlight', 'app_loading_state',
        'frozen_map', 'meta_commander', 'name_hash']


def dom(f):
    f = f.replace("\\", "/")
    for k in KEYS:
        if k in f:
            return 'audio-misc'
    return None


cnt = 0
for dirpath, _d, fns in os.walk(CH):
    for fn in fns:
        if not fn.endswith('.md'):
            continue
        rel = os.path.relpath(os.path.join(dirpath, fn), CH).replace("\\", "/")
        if dom(rel) != 'audio-misc':
            continue
        for i, line in enumerate(open(os.path.join(dirpath, fn), encoding='utf-8', errors='replace'), 1):
            if not line.startswith('|') or line.startswith('|---'):
                continue
            cells = [c.strip() for c in line.strip('|').split('|')]
            if len(cells) < 5 or not cells[0].startswith('`'):
                continue
            concl = cells[4]
            if concl.lower().startswith('o'):
                cnt += 1
                print(f'{rel}:{i} | addr={cells[1]!r} | concl={concl[:60]!r}')
print('total', cnt)
