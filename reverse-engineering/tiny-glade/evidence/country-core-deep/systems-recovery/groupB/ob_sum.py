import json, io, re, sys
def get(va):
    txt = io.open(f'decomp-{va}.txt', encoding='utf-8').read()
    return json.loads(txt[txt.index('{'):])['decompiled_code']
def body(va, maxlen=2000):
    code = get(va)
    out = []
    for l in code.splitlines():
        s = l.strip()
        if not s: continue
        if re.match(r'^(longlong|undefined|float|char|byte|uint|int|bool|ulonglong|undefined1|undefined4|undefined8|code \*|void|longlong \*|float \*|uint \*|byte \*|undefined4 \*|undefined8 \*|bool \*|ulonglong \*|byte \*)[ *]+[a-zA-Z_]', s) and '=' not in s:
            continue
        out.append(s)
    return chr(10).join(out)
for va in sys.argv[1:]:
    b = body(va)
    print('='*10, va, 'len', len(b))
    print(b[:2000])
