exec(open('reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/review/verify_original.py').read().split('report=')[0])
for va,n in [(0x142aceca9,9),(0x142acecb2,13)]:print(hex(va),repr(b[off(va-0x140000000):off(va-0x140000000)+n]))
