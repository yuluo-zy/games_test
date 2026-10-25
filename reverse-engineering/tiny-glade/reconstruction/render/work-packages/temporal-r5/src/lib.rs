use std::collections::HashMap;
use ustr::Ustr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextureDesc {
    pub fixed_words: [u32; 3],
    pub extent: [u32; 3],
    pub fixed_word_18: u32,
    pub fixed_bytes: [u8; 3],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourceDesc {
    Texture(TextureDesc),
    Buffer { opaque: Vec<u8> },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RecordState<L> {
    Texture(L),
    Buffer(L),
    Exported { access: u8 },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TemporalRecord<P, L> {
    pub physical: P,
    pub descriptor: ResourceDesc,
    pub state: RecordState<L>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lookup<L> {
    pub logical: L,
    pub reused: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pair<L> {
    pub first: L,
    pub second: L,
    pub parity: u32,
    pub both_reused: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LookupError<E> {
    AlreadyRequested,
    WrongResourceKind,
    Backend(E),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Created(Ustr),
    Imported { key: Ustr, access: u8 },
    Replaced(Ustr),
    Exported { key: Ustr, access: u8 },
    PairSelected { key: Ustr, parity: u32 },
}

pub trait Backend {
    type Physical;
    type Logical: Clone;
    type Error;
    fn create(
        &mut self,
        key: Ustr,
        descriptor: &ResourceDesc,
    ) -> Result<Self::Physical, Self::Error>;
    fn import(&mut self, physical: &Self::Physical, access: u8) -> Self::Logical;
    fn export(&mut self, logical: &Self::Logical, access: u8);
}

pub struct TemporalState<P, L> {
    pub records: HashMap<Ustr, TemporalRecord<P, L>>,
    pub parity: HashMap<Ustr, u32>,
    pub events: Vec<Event>,
}

impl<P, L> Default for TemporalState<P, L> {
    fn default() -> Self {
        Self {
            records: HashMap::new(),
            parity: HashMap::new(),
            events: Vec::new(),
        }
    }
}

impl<P, L: Clone> TemporalState<P, L> {
    pub fn get_or_create<B: Backend<Physical = P, Logical = L>>(
        &mut self,
        key: Ustr,
        descriptor: ResourceDesc,
        backend: &mut B,
    ) -> Result<Lookup<L>, LookupError<B::Error>> {
        if let Some(record) = self.records.get_mut(&key) {
            let access = match record.state {
                RecordState::Exported { access } => access,
                _ => return Err(LookupError::AlreadyRequested),
            };
            let recreate = match (&record.descriptor, &descriptor) {
                (ResourceDesc::Buffer { .. }, ResourceDesc::Buffer { .. }) => false,
                (ResourceDesc::Texture(old), ResourceDesc::Texture(new)) => {
                    if old.extent != new.extent {
                        true
                    } else {
                        assert_eq!(old, new, "temporal texture descriptor mismatch");
                        false
                    }
                }
                _ => return Err(LookupError::WrongResourceKind),
            };
            if !recreate {
                let logical = backend.import(&record.physical, access);
                record.state = match descriptor {
                    ResourceDesc::Texture(_) => RecordState::Texture(logical.clone()),
                    ResourceDesc::Buffer { .. } => RecordState::Buffer(logical.clone()),
                };
                self.events.push(Event::Imported { key, access });
                return Ok(Lookup {
                    logical,
                    reused: true,
                });
            }
        }
        let physical = backend
            .create(key, &descriptor)
            .map_err(LookupError::Backend)?;
        let logical = backend.import(&physical, 0);
        let state = match descriptor {
            ResourceDesc::Texture(_) => RecordState::Texture(logical.clone()),
            ResourceDesc::Buffer { .. } => RecordState::Buffer(logical.clone()),
        };
        let old = self.records.insert(
            key,
            TemporalRecord {
                physical,
                descriptor,
                state,
            },
        );
        if old.is_some() {
            self.events.push(Event::Replaced(key));
        } else {
            self.events.push(Event::Created(key));
        }
        Ok(Lookup {
            logical,
            reused: false,
        })
    }

    pub fn get_or_create_pair<B: Backend<Physical = P, Logical = L>>(
        &mut self,
        key: Ustr,
        descriptor: TextureDesc,
        backend: &mut B,
    ) -> Result<Pair<L>, LookupError<B::Error>> {
        let first = self.get_or_create(
            Ustr::from(format!("{key}:0").as_str()),
            ResourceDesc::Texture(descriptor.clone()),
            backend,
        )?;
        let second = self.get_or_create(
            Ustr::from(format!("{key}:1").as_str()),
            ResourceDesc::Texture(descriptor),
            backend,
        )?;
        let parity = self.get_and_flip_parity(key);
        self.events.push(Event::PairSelected { key, parity });
        let both_reused = first.reused && second.reused;
        let (first, second) = if parity == 0 {
            (first.logical, second.logical)
        } else {
            (second.logical, first.logical)
        };
        Ok(Pair {
            first,
            second,
            parity,
            both_reused,
        })
    }

    pub fn get_and_flip_parity(&mut self, key: Ustr) -> u32 {
        let new = self
            .parity
            .get(&key)
            .copied()
            .map(|old| 1u32.wrapping_sub(old))
            .unwrap_or(0);
        self.parity.insert(key, new);
        new
    }

    pub fn export_temporal<B: Backend<Physical = P, Logical = L>>(&mut self, backend: &mut B) {
        for (&key, record) in &mut self.records {
            let logical = match &record.state {
                RecordState::Texture(logical) | RecordState::Buffer(logical) => logical,
                RecordState::Exported { .. } => continue,
            };
            backend.export(logical, 0x37);
            record.state = RecordState::Exported { access: 0x37 };
            self.events.push(Event::Exported { key, access: 0x37 });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    #[derive(Default)]
    struct TestBackend {
        next: u32,
        imports: Vec<(u32, u8)>,
        exports: Vec<(u32, u8)>,
        fail_on: Option<u32>,
    }
    impl Backend for TestBackend {
        type Physical = u32;
        type Logical = u32;
        type Error = &'static str;
        fn create(&mut self, _: Ustr, _: &ResourceDesc) -> Result<u32, Self::Error> {
            self.next += 1;
            if self.fail_on == Some(self.next) {
                return Err("creation failed");
            }
            Ok(self.next)
        }
        fn import(&mut self, physical: &u32, access: u8) -> u32 {
            self.imports.push((*physical, access));
            *physical
        }
        fn export(&mut self, logical: &u32, access: u8) {
            self.exports.push((*logical, access));
        }
    }
    fn desc() -> TextureDesc {
        TextureDesc {
            fixed_words: [1, 2, 3],
            extent: [8, 8, 1],
            fixed_word_18: 4,
            fixed_bytes: [5, 6, 2],
        }
    }
    #[test]
    fn multi_frame_texture_create_export_import() {
        let mut state = TemporalState::default();
        let mut backend = TestBackend::default();
        let key = Ustr::from("texture");
        assert_eq!(
            state
                .get_or_create(key, ResourceDesc::Texture(desc()), &mut backend)
                .unwrap(),
            Lookup {
                logical: 1,
                reused: false
            }
        );
        assert_eq!(
            state.get_or_create(key, ResourceDesc::Texture(desc()), &mut backend),
            Err(LookupError::AlreadyRequested)
        );
        state.export_temporal(&mut backend);
        state.export_temporal(&mut backend);
        assert_eq!(backend.exports, vec![(1, 0x37)]);
        for _ in 0..4 {
            assert_eq!(
                state
                    .get_or_create(key, ResourceDesc::Texture(desc()), &mut backend)
                    .unwrap(),
                Lookup {
                    logical: 1,
                    reused: true
                }
            );
            state.export_temporal(&mut backend);
        }
        assert_eq!(backend.next, 1);
        assert_eq!(
            backend.imports,
            vec![(1, 0), (1, 0x37), (1, 0x37), (1, 0x37), (1, 0x37)]
        );
    }
    #[test]
    fn buffer_descriptor_change_is_not_compared() {
        let mut state = TemporalState::default();
        let mut backend = TestBackend::default();
        let key = Ustr::from("buffer");
        state
            .get_or_create(key, ResourceDesc::Buffer { opaque: vec![1] }, &mut backend)
            .unwrap();
        state.export_temporal(&mut backend);
        let result = state
            .get_or_create(
                key,
                ResourceDesc::Buffer { opaque: vec![2, 3] },
                &mut backend,
            )
            .unwrap();
        assert!(result.reused);
        assert_eq!(backend.next, 1);
        assert_eq!(
            state.get_or_create(key, ResourceDesc::Buffer { opaque: vec![] }, &mut backend),
            Err(LookupError::AlreadyRequested)
        );
    }
    #[test]
    fn texture_extent_recreates_and_replacement_failure_preserves_old() {
        let mut state = TemporalState::default();
        let mut backend = TestBackend::default();
        let key = Ustr::from("resize");
        state
            .get_or_create(key, ResourceDesc::Texture(desc()), &mut backend)
            .unwrap();
        state.export_temporal(&mut backend);
        let mut changed = desc();
        changed.extent[0] = 16;
        backend.fail_on = Some(2);
        assert!(matches!(
            state.get_or_create(key, ResourceDesc::Texture(changed.clone()), &mut backend),
            Err(LookupError::Backend(_))
        ));
        assert_eq!(state.records[&key].physical, 1);
        assert!(matches!(
            state.records[&key].state,
            RecordState::Exported { .. }
        ));
        backend.fail_on = None;
        assert!(
            !state
                .get_or_create(key, ResourceDesc::Texture(changed), &mut backend)
                .unwrap()
                .reused
        );
        assert_eq!(state.records[&key].physical, 3);
    }
    #[test]
    fn fixed_descriptor_mismatch_panics_and_kind_mismatch_returns_error() {
        let mut state = TemporalState::default();
        let mut backend = TestBackend::default();
        let key = Ustr::from("strict");
        state
            .get_or_create(key, ResourceDesc::Texture(desc()), &mut backend)
            .unwrap();
        state.export_temporal(&mut backend);
        let mut changed = desc();
        changed.fixed_words[0] = 99;
        assert!(catch_unwind(AssertUnwindSafe(|| state.get_or_create(
            key,
            ResourceDesc::Texture(changed),
            &mut backend
        )))
        .is_err());
        assert_eq!(
            state.get_or_create(key, ResourceDesc::Buffer { opaque: vec![] }, &mut backend),
            Err(LookupError::WrongResourceKind)
        );
        assert_eq!(state.records[&key].physical, 1);
    }
    #[test]
    fn pair_alternates_per_key_over_frames() {
        let mut state = TemporalState::default();
        let mut backend = TestBackend::default();
        let key = Ustr::from("pair");
        for frame in 0..6 {
            let pair = state.get_or_create_pair(key, desc(), &mut backend).unwrap();
            assert_eq!(pair.parity, frame % 2);
            assert_eq!(
                (pair.first, pair.second),
                if frame % 2 == 0 { (1, 2) } else { (2, 1) }
            );
            assert_eq!(pair.both_reused, frame != 0);
            state.export_temporal(&mut backend);
        }
        assert_eq!(state.get_and_flip_parity(Ustr::from("other")), 0);
        assert_eq!(backend.next, 2);
    }
    #[test]
    fn failed_pair_keeps_first_creation_and_does_not_flip() {
        let mut state = TemporalState::default();
        let mut backend = TestBackend {
            fail_on: Some(2),
            ..Default::default()
        };
        let key = Ustr::from("failed");
        assert!(state.get_or_create_pair(key, desc(), &mut backend).is_err());
        assert!(state.records.contains_key(&Ustr::from("failed:0")));
        assert!(!state.records.contains_key(&Ustr::from("failed:1")));
        assert!(!state.parity.contains_key(&key));
        backend.fail_on = None;
        assert_eq!(
            state.get_or_create_pair(key, desc(), &mut backend),
            Err(LookupError::AlreadyRequested)
        );
        state.export_temporal(&mut backend);
        let pair = state.get_or_create_pair(key, desc(), &mut backend).unwrap();
        assert!(!pair.both_reused);
        assert_eq!(pair.parity, 0);
    }
    #[test]
    fn missing_creation_failure_does_not_insert() {
        let mut state = TemporalState::default();
        let mut backend = TestBackend {
            fail_on: Some(1),
            ..Default::default()
        };
        let key = Ustr::from("missing");
        assert!(state
            .get_or_create(key, ResourceDesc::Texture(desc()), &mut backend)
            .is_err());
        assert!(state.records.is_empty());
    }
}
