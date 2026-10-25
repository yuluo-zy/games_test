from pathlib import Path
import hashlib,json,shutil,subprocess,runpy,contextlib,io
O=Path(__file__).resolve().parent;R=Path('D:/game');C=O/'runtime-candidate';C.mkdir(exist_ok=True)
manifest=[]
def edit(rel,fn):
 src=R/rel;before=src.read_bytes();s=before.decode('utf8').replace('\r','');after=fn(s);dst=C/rel;dst.parent.mkdir(parents=True,exist_ok=True);dst.write_text(after,encoding='utf8');manifest.append({'path':rel,'base_sha256':hashlib.sha256(before).hexdigest()})
def new(rel,s):
 dst=C/rel;dst.parent.mkdir(parents=True,exist_ok=True);dst.write_text(s,encoding='utf8');manifest.append({'path':rel,'base_sha256':None})
core='tiny-glade/crates/country-core/src/'
sys='tiny-glade/crates/systems/critters/src/'
edit(core+'resources/critters/mod.rs',lambda s:s+'''
/// Original shared control resource (default140f85b90; updater140f7df84).
#[repr(C)]
#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug, PartialEq)]
pub struct CritterControl {
    pub intensity: f32,
    pub enabled: bool,
    pub birds_enabled: bool,
}
impl Default for CritterControl {
    fn default() -> Self { Self { intensity: 0.0, enabled: false, birds_enabled: true } }
}
''')
def components(s):
 a=s.index('/// 小动物总开关/强度资源。');b=s.index('/// `DuckSpawnLocations`',a)
 return s[:a]+'/// One identity for the behavior systems and firefly rendering callbacks.\npub use country_core::resources::critters::CritterControl as CritterControlState;\n\n'+s[b:]
edit(sys+'components.rs',components)
def callback(s):
 a=s.index('/// `resources::critters::CritterControl`');b=s.index('}',a)+1
 s=s[:a]+'pub use crate::resources::critters::CritterControl;'+s[b:]
 s=s.replace('CritterControl { intensity: 0.0 }','CritterControl { intensity: 0.0, ..Default::default() }').replace('CritterControl { intensity: 0.5 }','CritterControl { intensity: 0.5, ..Default::default() }')
 return s
edit(core+'systems/render_loop/draw_passes/draw_callbacks_b.rs',callback)
edit(sys+'lib.rs',lambda s:s.replace('pub mod components;','pub mod components;\npub mod control_critters;'))
new(sys+'control_critters.rs','''//! Original inline system140f7de90, business span140f7df84..140f7dfef.
use bevy_ecs::prelude::*;
use country_core::resources::{critters::CritterControl, time::GameTime, time_of_day::TimeOfDay};

pub fn control_critters(tod: Res<TimeOfDay>, time: Res<GameTime>, mut control: ResMut<CritterControl>) {
    update(&mut control, tod.day_night_factor, time.delta);
}

fn update(control: &mut CritterControl, factor: f32, delta: f32) {
    control.enabled = factor >= 0.05 && factor < 0.8;
    let rate = if control.enabled { 0.2 } else { -0.3 };
    let value = control.intensity + rate * delta;
    // MAXSS(0,value), MINSS(1,lower): the source operand wins unordered/equal.
    let lower = if value < 0.0 { 0.0 } else { value };
    control.intensity = if 1.0 < lower { 1.0 } else { lower };
}

#[cfg(test)]
mod tests {
    use super::*;
    include!("tests/fixtures/control_original_b.rs");
    #[test]
    fn original_inline_control_arithmetic_324_cases() {
        for [factor, delta, before, expected, enabled, birds] in CONTROL_CASES {
            let mut control = CritterControl { intensity: f32::from_bits(before), enabled: true, birds_enabled: birds != 0 };
            update(&mut control, f32::from_bits(factor), f32::from_bits(delta));
            let expected = f32::from_bits(expected);
            if expected.is_nan() { assert!(control.intensity.is_nan()); }
            else { assert_eq!(control.intensity.to_bits(), expected.to_bits(), "factor={factor:x},dt={delta:x},before={before:x}"); }
            assert_eq!(control.enabled, enabled != 0);
            assert_eq!(control.birds_enabled, birds != 0);
        }
        assert!(CritterControl::default().birds_enabled);
    }
}
''')
new(sys+'tests/fixtures/control_original_b.rs',(O/'control_original_b.rs').read_text())
for e in manifest:
 p=C/e['path'];subprocess.run(['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe','--edition','2024','--config','skip_children=true',str(p)],check=True)
 e['proposed_sha256']=hashlib.sha256(p.read_bytes()).hexdigest()
(C/'manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf8')

# Independent original bounded upload bodies and cpu_write, not projection-only wrappers.
with contextlib.redirect_stdout(io.StringIO()):v=runpy.run_path(str(O/'native_extract.py'))
targets=[]
for a in [0x141d87140,0x142316760,0x14230edb0,0x14230f1e0]:
 n=v['procs'][hex(a)][0]['size'];targets.append({'address':hex(a),'size':n,'bytes_sha256':hashlib.sha256(v['read'](a,n)).hexdigest(),'range_basis':'Exact original PDB author range'})
 (O/f'{a:x}.original.asm').write_bytes(subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',f'--start-address={a:#x}',f'--stop-address={a+n:#x}',str(v['X'])],check=True,stdout=subprocess.PIPE).stdout)
job=[{'tool':'prepare_functions','args':{'program':'tiny-glade.exe','targets':targets}},{'tool':'get_functions','args':{'program':'tiny-glade.exe','functions':','.join(t['address'] for t in targets),'fields':'signature,decompiled_code,callees'}}]
(O/'upload-job.json').write_text(json.dumps(job),encoding='utf8')
print('prepared canonical control candidate and4 original upload bodies')
