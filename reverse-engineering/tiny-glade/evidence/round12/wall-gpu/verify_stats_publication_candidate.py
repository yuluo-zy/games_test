from pathlib import Path
import json,subprocess
here=Path(__file__).parent
cases=json.loads((here/'material-stats-publication-native-cases.json').read_text())['cases']
material=here/'candidates/crates/country-core/src/resources/walls/material_stats.rs'
lines=['#[path = '+json.dumps(str(material).replace('\\','/'))+'] mod material_stats;','use material_stats::MaterialStats;','fn main() {']
def array(xs, ty):return '['+','.join(str(x)+ty for x in xs)+']'
for i,c in enumerate(cases):
    b=c['before'];a=c['after']
    lines.append('{ let mut old=MaterialStats::default();')
    lines.append('old.lengths='+array(b['lengths'],'u32')+'.map(f32::from_bits);')
    lines.append('old.counts='+array(b['counts'],'u32')+';old.fences_segments_seed='+str(b['seed'])+'u64;')
    lines.append('old.pillar_placements=(0..'+str(b['placements'][2])+').map(|i|[i as f32,-(i as f32)]).collect();')
    lines.append('let old_pointer=old.pillar_placements.as_ptr();let old_points=old.pillar_placements.clone();')
    lines.append('let new=MaterialStats::for_new_publication(&mut old);')
    lines.append('assert_eq!(new.previous_lengths.map(f32::to_bits),'+array(a['previous_lengths'],'u32')+');')
    lines.append('assert_eq!(new.previous_counts,'+array(a['previous_counts'],'u32')+');')
    lines.append('assert_eq!(new.previous_fences_segments_seed,'+str(a['previous_seed'])+'u64);')
    lines.append('assert_eq!(new.previous_pillar_placements,old_points);assert_eq!(new.previous_pillar_placements.as_ptr(),old_pointer);assert!(old.pillar_placements.is_empty());')
    lines.append('assert_eq!(new.lengths.map(f32::to_bits),[0;7]);assert_eq!(new.counts,[0;8]);assert_eq!(new.fences_segments_seed,0);assert!(new.pillar_placements.is_empty());}')
lines.append('println!("128 original publisher statistics move/reset fixtures matched; original Vec storage remains dependency");}')
driver=here/'material-stats-publication-check.rs';driver.write_text('\n'.join(lines))
rustc='C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustc.exe'
exe=here/'material-stats-publication-check.exe'
subprocess.run([rustc,'--edition','2024','-O','-A','dead_code',str(driver),'-o',str(exe)],check=True)
r=subprocess.run([str(exe)],check=True,text=True,capture_output=True)
(here/'material-stats-publication-check.log').write_text(r.stdout)
print(r.stdout,end='')
