import re, io

checks = {
 'startup/ALGORITHMS.md': ['0x1408efd40','0x14082c000','0x140b0e0f0','0x1408be360','0x1408bf9c0',
   '0x140a08680','0x140a08d30','0x140aaaf80','0x140aab0c0','0x14089d5a0','0x140901430','0x1409d49a0',
   '0x140a97a30','0x1415f87c0','0x140a21620','0x140a21a30','0x1414547d0','0x140864430','0x140a92670',
   '0x1408f1a20','0x140a21620','0x1409e6b60','0x14089d6f0'],
 'asset-libraries/ALGORITHMS.md': ['0x14082bd70','0x140a06b00','0x140a05530','0x140a06370'],
 'components-geometry/ALGORITHMS.md': ['0x14082bd70','0x140a06b00','0x140a05530','0x140a06370','0x140858790','0x140858800'],
 'input/ALGORITHMS.md': ['0x1413fe800','0x1416f92d0','0x1409499c0','0x1402e0060'],
 'utils/ALGORITHMS.md': ['0x140a59220','0x140b024c0','0x140b02500','0x140b04640','0x140b04eb0',
   '0x14157f130','0x14157eda0','0x14157ef70','0x1402b4910','0x1402b4aa0','0x1402b4c40','0x1402b47f0',
   '0x140ade620','0x140ade8e0','0x1412945c0'],
}
out = io.open('round3/REVIEW-UNEXPLORED/_addr_scan.txt','w',encoding='utf-8')
for path, addrs in checks.items():
    text = open(path, encoding='utf-8').read()
    for a in addrs:
        # lowercase hex compare
        la = a.lower()
        hit = la in text.lower()
        out.write('%s %s %s\n' % (path.split('/')[0], a, 'FOUND' if hit else 'MISS'))
out.close()
print('done')
