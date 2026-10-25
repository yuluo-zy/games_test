from pathlib import Path
import json,struct
r=json.loads(Path('reverse-engineering/tiny-glade/evidence/round12/wall-gpu/sheep-spawn-native-policy.json').read_text())
for n,row in enumerate(r['sheep']):
 b=bytes.fromhex(row['bytes']);print(n,'transform',struct.unpack('<10f',b[:40]));print('sheep',[(hex(i),hex(struct.unpack_from('<I',b,0xa8+i)[0]))for i in range(0,104,4)]);print('critter tail',[(hex(i),hex(struct.unpack_from('<I',b,i)[0]))for i in range(0x110,0x180,4)])
print(r['control']);print(r['debug_commands']);print('id',r['bird_next_id'],'loans',r['pool_loan_count'])
