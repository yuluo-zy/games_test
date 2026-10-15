//! 原版 oracle 重放：`ResourceRegistry::populate_resources`（0x140be7ba0，97 例，
//! `parallel-batch-03/barrier-record/population-validation.json`）。
//! 输入 origins=[(tag,kind,initial,usage)]（tag 0 Create/1 ImportUnowned/2 ImportOwned；kind 0 图像/1 缓冲）、
//! lifetime events、pool 命中、backend 创建错误 tag。
//! 其中 `synthetic_local_reuse=true` 的 2 例是原版 oracle 自己声明的“只测消费者、未证实 reuse map 写入”的合成夹具，
//! 重建没有对应的 local reuse map，跳过并计数。
use rhapsody::rg::graph::{
    BackendFailure, Descriptor, EventBackend, OriginSource, Pool, RegistryResource, ResourceDeclaration,
    ResourceInfo, Texture, Buffer, TextureDesc, BufferDesc, populate_resources,
};
use serde_json::Value;
use std::path::PathBuf;

fn tag_of(r: &RegistryResource<u64, u64, ()>) -> u64 {
    match r {
        RegistryResource::OwnedTexture(_) => 0,
        RegistryResource::UnownedTexture(_) => 1,
        RegistryResource::OwnedBuffer(_) => 2,
        RegistryResource::UnownedBuffer(_) => 3,
        RegistryResource::Other(_) => 4,
        RegistryResource::Empty => 5,
    }
}

#[test]
fn populate_resources_matches_original() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../../../reverse-engineering/tiny-glade/evidence/render-recovery/parallel-batch-03/barrier-record/population-validation.json",
    );
    let Ok(text) = std::fs::read_to_string(path) else {
        eprintln!("skip population: evidence missing");
        return;
    };
    let report: Value = serde_json::from_str(&text).unwrap();
    let observations = report["observations"].as_array().unwrap();
    assert_eq!(observations.len(), 97);
    let (mut replayed, mut skipped) = (0, 0);
    for obs in observations {
        let input = &obs["input"];
        if input["synthetic_local_reuse"].as_bool().unwrap() {
            skipped += 1;
            continue;
        }
        let origins: Vec<Vec<u64>> = input["origins"]
            .as_array()
            .unwrap()
            .iter()
            .map(|o| o.as_array().unwrap().iter().map(|v| v.as_u64().unwrap()).collect())
            .collect();
        let events: Vec<u32> = input["events"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u32).collect();
        let pool_hit = input["pool_hit"].as_bool().unwrap();
        let error_tag = input["error_tag"].as_u64();

        let mut info = ResourceInfo::default();
        let mut declarations = Vec::new();
        let mut pool = Pool::default();
        for (i, o) in origins.iter().enumerate() {
            let (tag, kind, initial, usage) = (o[0], o[1], o[2] as u8, o[3] as u32);
            info.image_usage_flags.push(usage);
            info.buffer_usage_flags.push(usage);
            let base = if kind == 1 {
                Descriptor::Buffer(BufferDesc { word_at_00: 0, word_at_08: 0, bytes_at_0c_through_0e: [0; 3] })
            } else {
                Descriptor::Image(TextureDesc { words_at_00_through_18: [0; 7], bytes_at_1c_through_1e: [0; 3] })
            };
            let source = match tag {
                0 => OriginSource::Create,
                1 => OriginSource::ImportUnowned { token: 100 + i as u64, initial_access: initial },
                _ => OriginSource::ImportOwned { token: 100 + i as u64, initial_access: initial },
            };
            if pool_hit && tag == 0 && usage != 0 {
                match base.with_usage(usage) {
                    Descriptor::Image(desc) => pool.insert_texture(Texture { handle: 900 + i as u64, desc }),
                    Descriptor::Buffer(desc) => pool.insert_buffer(Buffer { handle: 900 + i as u64, desc }),
                }
            }
            declarations.push(ResourceDeclaration { name: format!("r{i}"), descriptor: base, source });
        }
        info.lifetime_events = events;
        let mut backend = EventBackend::default();
        if let Some(tag) = error_tag {
            backend.fail_at_creation = Some((0, tag as u8));
        }
        // 9/10 为原版 fatal 类（"result-tag" panic）。
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            populate_resources(&info, &declarations, &mut pool, &mut backend)
        }));
        let expected_panic = obs["panic"].as_str().unwrap();
        let population = match outcome {
            Ok(Ok(p)) => {
                assert_eq!(expected_panic, "ok", "{input}");
                assert_eq!(obs["returned_failure"].as_u64(), Some(0), "{input}");
                p
            }
            Ok(Err(failure)) => {
                assert_eq!(expected_panic, "ok", "{input}");
                assert_eq!(obs["returned_failure"].as_u64(), Some(1), "{input}");
                let _: &BackendFailure = &failure.error;
                *failure.partial
            }
            Err(_) => {
                assert_eq!(expected_panic, "result-tag", "{input}");
                replayed += 1;
                continue;
            }
        };
        let origin_len = population.remaining_origins.len() as u64;
        assert_eq!(origin_len, obs["origin_length"].as_u64().unwrap(), "origin_length {input}");
        if origin_len > 0 {
            let tags: Vec<u64> = population
                .remaining_origins
                .iter()
                .zip(&origins)
                .map(|(r, o)| if r.is_none() { 3 } else { o[0] })
                .collect();
            let want: Vec<u64> = obs["origin_tags"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap()).collect();
            assert_eq!(tags, want, "origin_tags {input}");
        }
        let reg = &population.registry;
        let reuse: Vec<[u64; 2]> = reg
            .resources
            .iter()
            .map(|r| [u64::from(r.backing_index), u64::from(r.reused)])
            .collect();
        let want_reuse: Vec<[u64; 2]> = obs["reuse"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| [p[0].as_u64().unwrap(), p[1].as_u64().unwrap()])
            .collect();
        assert_eq!(reuse, want_reuse, "reuse {input}");
        let want_initial: Vec<u8> = obs["initial"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u8).collect();
        assert_eq!(reg.resource_initial_access, want_initial, "initial {input}");
        assert_eq!(reg.backing_resources.len() as u64, obs["physical_count"].as_u64().unwrap(), "{input}");
        let backing: Vec<u64> = reg.backing_resources.iter().map(tag_of).collect();
        let want_backing: Vec<u64> = obs["backing_tags"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap()).collect();
        assert_eq!(backing, want_backing, "backing_tags {input}");
        // 边界计数：池命中不创建；创建时 pool_acquire/create 为 1（只在创建失败夹具里出现）。
        let b = &obs["boundaries"];
        let created = backend
            .events
            .iter()
            .filter(|e| matches!(e, rhapsody::rg::graph::BackendEvent::CreateWithoutInitialData { .. }))
            .count() as u64;
        // 原版 hook 把每次 create 调用计数（失败夹具里失败的那次也算），重建的 EventBackend 只记录成功创建。
        let attempted = created + u64::from(error_tag.is_some());
        assert_eq!(b["create"].as_u64().unwrap(), attempted, "create {input}");
        replayed += 1;
    }
    assert_eq!((replayed, skipped), (95, 2));
}
