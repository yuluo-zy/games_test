from pathlib import Path
import difflib, hashlib, json, shutil
root=Path('/Users/yuluo/Desktop/game');out=Path(__file__).resolve().parent
sha=lambda b:hashlib.sha256(b).hexdigest()
(out/'evidence').mkdir(exist_ok=True)
review=root/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-f7070bb3446b7a37eb/building-audit-20261018/v2/terrain-audio-review'
for name in ['140a59430-original-B.asm','stop-original-identity.json','extract_native_stop.py']:
 shutil.copy2(review/name,out/'evidence'/name)
skeleton=root/'games_test/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/utils/audio_utils.rs'
lines=skeleton.read_text().splitlines()
(out/'evidence'/'B-pdb-stop-clock-records.txt').write_text('\n'.join(lines[91:136])+'\n')
rel='games_test/tiny-glade/crates/country-core/src/utils/audio_utils.rs'
p=root/rel;base=p.read_text();candidate=base.replace('use core::time::Duration;','use core::time::Duration;\nuse std::{sync::OnceLock, time::Instant};',1)
needle='    /// 首次调用记录时间，严格超过阈值后停止；时间倒退按零耗时处理。'
assert needle in candidate
adapter='''    /// 使用单调实时时钟请求停止；显式时钟调用方仍可使用 [`Self::stop`]。
    pub fn stop_now(&mut self) -> Result<(), E::Error> {
        static CLOCK_ORIGIN: OnceLock<Instant> = OnceLock::new();
        self.stop(CLOCK_ORIGIN.get_or_init(Instant::now).elapsed())
    }

'''
candidate=candidate.replace(needle,adapter+needle,1)
needle='    #[test]\n    fn first_stop_only_records_the_time()'
assert needle in candidate
regression='''    #[test]
    fn live_stop_clock_advances_and_releases_a_started_loop() {
        let mut player = AudioLoop::new(vec![entry(true), entry(true)], 0, false, false);
        player.start_if_not_playing().unwrap();
        player.stop_now().unwrap();
        assert!(player.start.is_some());
        assert!(player.started);
        assert_eq!(player.entries[0].log, ["start"]);

        // A zero delay still requires a later instant, just like stop(now).
        std::thread::sleep(ms(2));
        player.stop_now().unwrap();
        assert_eq!(player.entries[0].log, ["start", "stop"]);
        assert_eq!(player.cursor, 1);
        assert!(!player.started);
        assert_eq!(player.start, None);
    }

'''
candidate=candidate.replace(needle,regression+needle,1)
pout=out/'candidate'/rel;pout.parent.mkdir(parents=True,exist_ok=True);pout.write_text(candidate)
owned=''.join(difflib.unified_diff(base.splitlines(True),candidate.splitlines(True),fromfile='a/'+rel,tofile='b/'+rel))
(out/'owned-audio-utils.patch').write_text(owned)
manifest={'target':'B','claim':'TG-8716b23eed86181558','production_edited':False,'cargo_run':False,'files':[{'scope':'owned','path':rel,'base_sha256':sha(base.encode()),'candidate_sha256':sha(candidate.encode())}],'adapter':'OnceLock<Instant> process origin elapsed Duration, delegates unchanged stop(now)','source_call_count':0}
patches=[]
for p in sorted((root/'games_test/tiny-glade/crates/country-core/src/systems/audio').rglob('*.rs')):
 base=p.read_text();count=base.count('.stop(std::time::Duration::ZERO)')
 if not count:continue
 candidate=base.replace('.stop(std::time::Duration::ZERO)', '.stop_now()')
 rel=str(p.relative_to(root));pout=out/'candidate'/rel;pout.parent.mkdir(parents=True,exist_ok=True);pout.write_text(candidate)
 patches.append(''.join(difflib.unified_diff(base.splitlines(True),candidate.splitlines(True),fromfile='a/'+rel,tofile='b/'+rel)))
 manifest['files'].append({'scope':'root_integration_only','path':rel,'replacement_count':count,'base_sha256':sha(base.encode()),'candidate_sha256':sha(candidate.encode())})
 manifest['source_call_count']+=count
(out/'integration-owner.patch').write_text(''.join(patches))
manifest['consumer_source_count']=len(patches)
manifest['patches']={name:sha((out/name).read_bytes())for name in['owned-audio-utils.patch','integration-owner.patch']}
manifest['pdb_source']={'path':str(skeleton),'sha256':sha(skeleton.read_bytes())}
(out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print({'source_call_count':manifest['source_call_count'],'consumer_source_count':len(patches),'patches':manifest['patches']})
