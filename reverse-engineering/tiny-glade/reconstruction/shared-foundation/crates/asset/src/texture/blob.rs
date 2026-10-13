//! Private archived schema. Numeric `kind` is preserved because its original Rust enum name is unknown.
//! Root layout is evidenced by B rkyv producer and every shipped `.texture` root.
#[derive(Clone, Debug, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[archive_attr(repr(C), derive(bytecheck::CheckBytes))]
pub struct TextureAssetBlob {
    pub data: Vec<u8>,
    pub dimensions: [u32; 3],
    pub mips: u16,
    pub layers: u16,
    pub format: u8,
    pub kind: u8,
}
pub const TEXTURE_MAGIC: u32 = 0x48502183;
pub const TEXTURE_VERSION: u32 = 8;
impl TextureAssetBlob {
    /// Ownership-safe private file loader: public rkyv performs validation and decoding.
    pub fn load(bytes: &[u8]) -> anyhow::Result<Self> {
        anyhow::ensure!(bytes.len() >= 8, "Texture file is truncated");
        anyhow::ensure!(
            u32::from_le_bytes(bytes[..4].try_into()?) == TEXTURE_MAGIC,
            "Texture magic mismatch"
        );
        anyhow::ensure!(
            u32::from_le_bytes(bytes[4..8].try_into()?) == TEXTURE_VERSION,
            "Texture pipeline version mismatch"
        );
        let mut aligned = rkyv::AlignedVec::new();
        aligned.extend_from_slice(&bytes[8..]);
        let archived = rkyv::check_archived_root::<Self>(&aligned)
            .map_err(|e| anyhow::anyhow!("Invalid texture archive: {e}"))?;
        Ok(rkyv::Deserialize::deserialize(archived, &mut rkyv::Infallible).unwrap())
    }
    pub fn to_file_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let payload = rkyv::to_bytes::<_, 256>(self)
            .map_err(|e| anyhow::anyhow!("Texture serialization: {e}"))?;
        let mut bytes = Vec::with_capacity(payload.len() + 8);
        bytes.extend(TEXTURE_MAGIC.to_le_bytes());
        bytes.extend(TEXTURE_VERSION.to_le_bytes());
        bytes.extend_from_slice(&payload);
        Ok(bytes)
    }
}
