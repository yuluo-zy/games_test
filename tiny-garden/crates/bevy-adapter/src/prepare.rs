//! CPU tangent preparation on the generation pool, never in the upload system.
//! The same MikkTSpace implementation/handedness as Bevy's Mesh path is used.
use garden_generation::incremental::{Cancellation, PreparedBuilding};
use garden_generation::mesh::{MeshData, MeshError};
use std::sync::Arc;
struct Geometry<'a> {
    mesh: &'a MeshData,
    tangents: Vec<[f32; 4]>,
}
impl Geometry<'_> {
    fn index(&self, face: usize, vertex: usize) -> usize {
        self.mesh.indices[face * 3 + vertex] as usize
    }
}
impl bevy_mikktspace::Geometry for Geometry<'_> {
    fn num_faces(&self) -> usize {
        self.mesh.triangles()
    }
    fn num_vertices_of_face(&self, _: usize) -> usize {
        3
    }
    fn position(&self, f: usize, v: usize) -> [f32; 3] {
        self.mesh.positions[self.index(f, v)]
    }
    fn normal(&self, f: usize, v: usize) -> [f32; 3] {
        self.mesh.normals[self.index(f, v)]
    }
    fn tex_coord(&self, f: usize, v: usize) -> [f32; 2] {
        self.mesh.uvs[self.index(f, v)]
    }
    fn set_tangent(&mut self, tangent: Option<bevy_mikktspace::TangentSpace>, f: usize, v: usize) {
        let i = self.index(f, v);
        self.tangents[i] = tangent.unwrap_or_default().tangent_encoded();
    }
}
pub fn prepare(building: &mut PreparedBuilding, cancel: &Cancellation) -> Result<(), MeshError> {
    for batch in &mut building.batches {
        cancel.check()?;
        if !batch.tangents.is_empty() {
            continue;
        }
        batch.data.validate()?;
        let mut geometry = Geometry {
            mesh: &batch.data,
            tangents: vec![[0.; 4]; batch.data.positions.len()],
        };
        bevy_mikktspace::generate_tangents(&mut geometry)
            .map_err(|_| MeshError("invalid tangent geometry"))?;
        for t in &mut geometry.tangents {
            t[3] = -t[3];
        }
        if geometry.tangents.iter().flatten().any(|v| !v.is_finite()) {
            return Err(MeshError("non-finite tangent"));
        }
        batch.tangents = Arc::new(geometry.tangents);
    }
    cancel.check()
}
