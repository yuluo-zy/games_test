//! B forwards UTF-8 bytes directly to xxh3, seed=0/default 192-byte secret.
#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
pub struct NameHash(pub u64);
impl NameHash {
    pub fn compute(name:&str)->Self {Self(xxhash_rust::xxh3::xxh3_64(name.as_bytes()))}
    pub fn debug_name(&self)->&'static str {"(NameHash)"}
}
