exec(open('reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/review/verify_original.py').read().split('report=')[0])
for va,n in [(0x14295d6a8,10),(0x14295d798,10),(0x14295d8a8,3),(0x14295d718,24),(0x14295d748,20),(0x14295d75c,31)]:print(hex(va),repr(b[off(va-0x140000000):off(va-0x140000000)+n]))
