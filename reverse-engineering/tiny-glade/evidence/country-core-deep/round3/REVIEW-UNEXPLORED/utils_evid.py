import json, io
rows = json.load(open('round3/REVIEW-UNEXPLORED/_rows_evid.json', encoding='utf-8'))
out = io.open('round3/REVIEW-UNEXPLORED/_utils_evid.txt','w',encoding='utf-8')
for r in rows:
    f = r['file'].replace('\\','/')
    if f.startswith('chapters/utils/'):
        out.write('%s | %s | %s | %s\n' % (f.split('/')[-1], r['name'][:55], r['addr'], r['evid'][:180]))
out.close()
print('done')
