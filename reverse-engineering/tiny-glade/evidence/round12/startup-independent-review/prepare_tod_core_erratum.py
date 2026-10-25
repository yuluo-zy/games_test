from pathlib import Path
import difflib,hashlib,json
here=Path(__file__).resolve().parent;root=here.parents[4]
relative='tiny-glade/crates/country-core/src/startup/run_game.rs'
source=root/relative;before=source.read_bytes();text=before.decode().replace('\r\n','\n')
anchor='    world.init_resource::<GameConfig>();\n'
assert text.count(anchor)==1
text=text.replace(anchor,anchor+'    world.init_resource::<crate::resources::time_of_day::TodSequence>();\n')
anchor='        music_audio_init_system::<B>.run_if(not(resource_exists::<MusicAudioState<B::Event>>)),\n'
assert text.count(anchor)==1
text=text.replace(anchor,anchor+'        super::startup_time_of_day::startup_time_of_day.run_if(run_once),\n')
block='''    // batch4 的默认 ToD 先于会话载入写入，避免覆盖会话自己的 ToD。
    schedule.add_systems(
        super::startup_time_of_day::startup_time_of_day
            .before(startup_session::load_game_session)
            .run_if(run_once),
    );
'''
assert block in text
text=text.replace(block,'')
text=text.replace('pub fn register_session_systems(schedule: &mut Schedule) {\n    use bevy_ecs::schedule::common_conditions::{resource_exists, run_once};', 'pub fn register_session_systems(schedule: &mut Schedule) {\n    use bevy_ecs::schedule::common_conditions::resource_exists;')
after=text.replace('\n','\r\n').encode() if b'\r\n' in before else text.encode()
dst=here/'candidate'/relative;dst.parent.mkdir(parents=True,exist_ok=True);dst.write_bytes(after)
patch=''.join(difflib.unified_diff(before.decode().splitlines(True),after.decode().splitlines(True),'a/'+relative,'b/'+relative))
(here/'tod-core-registration-erratum.patch').write_bytes(patch.encode())
(here/'tod-core-registration-erratum.json').write_text(json.dumps({'file':relative,'base_sha256':hashlib.sha256(before).hexdigest(),'candidate_sha256':hashlib.sha256(after).hexdigest(),'scope':'Only alternate core assembler registration and required resource; TOD producer is unchanged'},indent=2),encoding='utf8')
