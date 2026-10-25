from pathlib import Path
import json,struct,hashlib
R=Path('D:/game');O=Path(__file__).resolve().parent;C=O/'candidate'
def edit(rel,before,after):
 p=R/rel;s=p.read_text(encoding='utf8');assert s.count(before)==1,(rel,before)
 q=C/rel;q.parent.mkdir(parents=True,exist_ok=True);q.write_text(s.replace(before,after),encoding='utf8',newline='\n')
edit('tiny-glade/crates/utils/src/lib.rs','pub mod rng;','pub mod rng;\npub mod sampling;')
edit('tiny-glade/crates/country-core/src/resources/critters/mod.rs','pub use butterfly::Butterfly;','pub use butterfly::Butterfly;\n\n/// Original shared zero-sized critter marker, registered at140b22db0 and\n/// included in both original startup species bundles.\n#[derive(bevy_ecs::component::Component,Clone,Copy,Debug,Default,PartialEq,Eq)]\npub struct Critter;')
edit('tiny-glade/crates/systems/critters/src/components.rs','pub use country_core::resources::critters::CritterGoal;','pub use country_core::resources::critters::{Critter,CritterGoal};')
edit('tiny-glade/crates/systems/critters/src/lib.rs','pub mod shared;','pub mod shared;\npub mod startup_critters;')
rel='tiny-glade/crates/systems/critters/src/firefly.rs';s=(R/rel).read_text(encoding='utf8')
repls=[('    Sheep, Transform, Velocity,','    Transform, Velocity,'),
('Query<(&Transform, &mut Firefly, &Velocity, &mut CritterGoal, &Sheep)>','Query<(&Transform, &mut Firefly, &Velocity, &mut CritterGoal)>'),
('for (transform, mut firefly, velocity, mut goal, _sheep) in &mut query','for (transform, mut firefly, velocity, mut goal) in &mut query'),
('/// 查询（O）：`(Transform, Sheep, CritterGoal, VelocityTarget, Multiplier)`。','/// Original PDB: `(Transform, Firefly, CritterGoal, VelocityTarget, Multiplier)`.'),
('        &Sheep,','        &Firefly,'),
('for (transform, _sheep, goal, mut target, mut multiplier) in &mut query','for (transform, _firefly, goal, mut target, mut multiplier) in &mut query')]
for a,b in repls:assert s.count(a)==1,(rel,a);s=s.replace(a,b)
p=C/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(s,encoding='utf8',newline='\n')
j=json.loads((O/'original-policy-fixtures.json').read_text(encoding='utf8'));case=j['cases'][-1]
bp=[bytes.fromhex(v) for v in case['butterfly_bundles']]
bf=[list(struct.unpack_from('<3I',v,0x50)) for v in bp];bt=[struct.unpack_from('<I',v,0x84)[0] for v in bp]
fp=[bytes.fromhex(v['raw']) for v in case['command_records'] if v['executor']=='0x1412d7e60']
ff=[list(struct.unpack_from('<3I',v,0x28)) for v in fp]
def array(name,rows):return f'const {name}:[[u32;3];{len(rows)}]=[\n'+''.join('    ['+','.join(hex(v) for v in row)+'],\n' for row in rows)+'];\n'
fixture='// Generated only from full original-RNE startup execution.\n'+array('BUTTERFLIES',bf)+'const BUTTERFLY_TIMERS:[u32;24]=['+','.join(hex(v) for v in bt)+'];\n'+array('FIREFLIES',ff)
p=C/'tiny-glade/crates/systems/critters/tests/fixtures/startup_original_b.rs';p.parent.mkdir(parents=True,exist_ok=True);p.write_text(fixture,encoding='utf8',newline='\n')
(O/'candidate-base-hashes.json').write_text(json.dumps({str(p.relative_to(C)).replace('\\','/'):hashlib.sha256((R/p.relative_to(C)).read_bytes()).hexdigest() if (R/p.relative_to(C)).exists() else None for p in C.rglob('*.rs')},indent=2),encoding='utf8')
print('prepared',len(list(C.rglob('*.rs'))),'candidate files')
