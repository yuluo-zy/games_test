//! Restored CPU roof generator source chain. No ECS, replay or GPU execution.
use crate::{rectangular_edges, rectangular_caps, rectangular_faces, tiles, surface};

#[derive(Debug)]
pub enum RoofBuildError { MissingRectangle, Curve(surface::CurveError) }
#[derive(Debug)]
pub struct RoofTileBatch {
    pub records:Vec<tiles::RoofTileRecord>,
    pub edges:usize,
    pub ridge_caps:usize,
    pub faces_and_fillers:usize,
    /// Original mutable RNG/seed lifecycle of the rectangular generator.
    pub rectangular_final_state:Option<(u64,u32)>,
}

pub fn assemble_roof_tiles_observed(roof:&[u8;88],rectangle:Option<&[u8;24]>,
                                     special_mode:bool)->Result<RoofTileBatch,RoofBuildError> {
    if roof[8]&1==0 {
        let records=tiles::assemble_circular_from_observed(roof,special_mode).map_err(RoofBuildError::Curve)?;
        let count=records.len();
        return Ok(RoofTileBatch{records,edges:0,ridge_caps:0,faces_and_fillers:count,rectangular_final_state:None});
    }
    let rectangle=rectangle.ok_or(RoofBuildError::MissingRectangle)?;
    let mut rng=tiles::TileRng(0);
    let mut ordinal=0;
    let mut records=rectangular_edges::assemble_edges_from_observed(rectangle,roof,&mut rng,&mut ordinal);
    let edges=records.len();
    let caps=rectangular_caps::assemble_caps_from_observed(rectangle,roof,special_mode,&mut rng,&mut ordinal);
    let ridge_caps=caps.len();records.extend(caps);
    let faces=rectangular_faces::assemble_faces_from_observed(rectangle,roof,special_mode,&mut rng,&mut ordinal).map_err(RoofBuildError::Curve)?;
    let faces_and_fillers=faces.len();records.extend(faces);
    Ok(RoofTileBatch{records,edges,ridge_caps,faces_and_fillers,rectangular_final_state:Some((rng.0,ordinal))})
}
