from pathlib import Path
import json,struct,subprocess,sys
here=Path(__file__).parent
fixture=json.loads((here/'arch-stats-native-cases.json').read_text())
source=here/'candidates-v23/crates/country-core/src/systems/wall/construction/arch_stats.rs'
lines=['mod structures {#[derive(Clone)] pub struct MaterializedWallSegment {pub positions:Vec<[f32;3]>}}', '#[path='+json.dumps(str(source).replace('\\','/'))+'] mod arch_stats;', 'fn main(){']
for c in fixture['cases']:
    points=','.join('[0.,f32::from_bits('+str(struct.unpack('<I',struct.pack('<f',y))[0])+'u32),0.]' for y in c['points_y'])
    lines.append('{let segments=vec![structures::MaterializedWallSegment{positions:vec!['+points+']};'+str(c['segments'])+'];let mut s=arch_stats::WallArchMaterialStats::default();')
    lines.append('s.observe_mask(f32::from_bits('+str(c['mask_length_bits'])+'u32),&segments,'+str(c['has_roof']).lower()+');')
    lines.append('assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],['+','.join(str(v)+'u32' for v in c['lengths'])+']);')
    lines.append('assert_eq!(s.gates_count,'+str(c['gates'])+'u64);assert_eq!(s.sewer_count,'+str(c['sewer'])+'u64);}')
lines.append('println!("168 original arch material classification cases matched bitwise lengths and gate/sewer counters");}')
driver=here/'arch-stats-check.rs';driver.write_text('\n'.join(lines))
if '--prepare-only' in sys.argv:print(json.dumps({'driver':str(driver),'compiled':False}));raise SystemExit(0)
exe=here/'arch-stats-check.exe'
subprocess.run(['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustc.exe','--edition','2024','-O',str(driver),'-o',str(exe)],check=True)
r=subprocess.run([str(exe)],check=True,text=True,capture_output=True);(here/'arch-stats-check.log').write_text(r.stdout);print(r.stdout,end='')
