from pathlib import Path
import hashlib,json,difflib,datetime
root=Path('D:/game/tiny-glade');here=Path(__file__).parent;out=here/'skip-splash-candidate';rows={};patch=[]
def save(rel,new):
 p=root/rel;old=p.read_bytes() if p.exists() else b'';dst=out/rel;dst.parent.mkdir(parents=True,exist_ok=True);dst.write_bytes(new)
 rows[rel]={'base_sha256':hashlib.sha256(old).hexdigest() if old else None,'candidate_sha256':hashlib.sha256(new).hexdigest()}
 patch.append(''.join(difflib.unified_diff(old.decode().splitlines(True),new.decode().splitlines(True),'a/'+rel if old else '/dev/null','b/'+rel)))
rel='crates/tiny-glade/src/game_loop.rs';raw=(root/rel).read_bytes();needle=b'        let mut preparation = None;';assert raw.count(needle)==1;pos=raw.index(needle);nl=b'\r\n' if raw[raw.index(b'\n',pos)-1:raw.index(b'\n',pos)]==b'\r' else b'\n'
prefix='''        // Original run_game_thread14059f0a9 inserts the GUI context passed
        // from the OS owner as Pending before either startup path. The public
        // host owns Context creation here; both Apps retain that one identity.
        use country_core::resources::gui::{EguiContextResource, PendingEguiContextResource};
        let shared_gui = app.world().get_resource::<PendingEguiContextResource>()
            .map(|value| value.0.clone())
            .or_else(|| app.world().get_resource::<EguiContextResource>().map(|value| value.0.clone()))
            .unwrap_or_default();
        if !app.world().contains_resource::<PendingEguiContextResource>() {
            app.insert_resource(PendingEguiContextResource(shared_gui.clone()));
        }
'''.encode().replace(b'\n',nl)
raw=raw[:pos]+prefix+raw[pos:];needle=b'            let mut loading = crate::run_game::make_loading_app(&app)?;';assert raw.count(needle)==1
raw=raw.replace(needle,needle+nl+b'            loading.insert_resource(EguiContextResource(shared_gui));',1)
needle=b'        let context = app';pos=raw.index(needle,raw.index(b'        let mut preparation = None;'));end=raw.rfind(b'        }',0,pos)
assert raw[end:pos].strip()==b'}'
raw=raw[:end]+b'        } else {'+nl+b'            crate::run_game::finish_skipped_splash(app.world_mut())?;'+nl+b'        }'+raw[end+9:]
save(rel,raw)
rel='crates/tiny-glade/src/run_game.rs';raw=(root/rel).read_bytes();needle=b'fn transfer_loading_resources(';pos=raw.index(needle);nl=b'\r\n' if raw[raw.index(b'\n',pos)-1:raw.index(b'\n',pos)]==b'\r' else b'\n'
helper='''/// Original skip_splash=true branch1405a4194..439d. There is no loading
/// App, async preparation, warmup or Started event on this native path.
pub(crate) fn finish_skipped_splash(world: &mut World) -> anyhow::Result<()> {
    use country_core::resources::gui::PendingEguiContextResource;
    let pending = world.remove_resource::<PendingEguiContextResource>()
        .context("B skipped splash: PendingEguiContextResource missing")?;
    world.insert_resource(pending.into_egui_context_resource());
    world.send_event(country_core::systems::save::load_game::StartupRevealFinished);
    Ok(())
}

'''.encode().replace(b'\n',nl)
tests=(here/'startup-handoff.tests.rs').read_bytes().replace(b'\r\n',b'\n').replace(b'\n',nl)
save(rel,raw[:pos]+helper+raw[pos:]+nl+tests)
(here/'skip-splash-candidate.patch').write_bytes(''.join(patch).encode())
(here/'skip-splash-manifest.json').write_text(json.dumps({'created':datetime.datetime.now().isoformat(),'scope':'evidence only; no production or Cargo/LLVM','files':rows,'tests':'full source rustfmt parse only; Cargo and runtime pending central owner'},indent=2),encoding='utf-8')
print(json.dumps(rows,indent=2))
