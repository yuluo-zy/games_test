import json, io
d = json.load(open('round3/oa2-results/w9_audio_backend.json', encoding='utf-8'))
out = io.open('round3/REVIEW-UNEXPLORED/_w9_detail.txt','w',encoding='utf-8')
def walk(o, path=''):
    if isinstance(o, dict):
        for k, v in o.items():
            walk(v, path+'/'+str(k))
    elif isinstance(o, list):
        for i, v in enumerate(o):
            walk(v, path+'/%d'%i)
    elif isinstance(o, str):
        if 'texture_watch' in o or 'debug' in o or '0x14003bb80' in o or '0x1408efd40' in o:
            out.write('%s | %s\n' % (path, o))
walk(d)
out.close()
print('done')
