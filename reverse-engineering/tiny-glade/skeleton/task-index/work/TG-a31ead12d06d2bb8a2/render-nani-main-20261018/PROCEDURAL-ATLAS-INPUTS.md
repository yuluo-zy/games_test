# B procedural atlas producer — read-only handoff

Owner of load_mesh_atlases is main-chain-boot-20261018 / TG-1a0d194affaf7f0d17. No source in that claim edited. Retained original load_mesh_atlases@14129f400/5138B and call map reused from its owner. Independent fresh LLVM ranges: roof FlatMap@141302d60/2000B, curved FlatMap@1413018b0/1214B, curved per-row Map@1414783b0/239B. Scalar literals read directly from immutable B RNE, bits in procedural-mesh-constants.json. No Ghidra database opened.

RoofFloor kind9: six levels 0..=5, n=1<<level. Current grid_gen_cw(indices,0,1,n,1) gives real native index order. For i=0..=n, t=(i as f32)/(n as f32)-0.5; append [-0.5,0,t],[0.5,0,t]. Only level!=0, transform each existing position in order: z=sin(z*PI_f32)*0.5; x=sqrt(clamp(1-(z+z)*(z+z),0,1))*signum(old_x)*0.5. PI bits40490fdb. Register via actual atlas.add(RoofFloor,format!("stone_floor_{level}"),mesh), Vertex_Position Float3. Native signum implementation handles NaN explicitly, but generated x always finite +/-0.5; no numerical equivalence beyond actual instruction formula claimed.

CurvedHalfTimber kind24: six levels, same n. Current grid_gen_ccw(indices,0,6,n,6). For each i=0..=n, append [x,0,(i as f32)/(n as f32)] for x=[0,1/6,1/3,0.5,2/3,5/6,1]. Native literal bits3e2aaaab/3eaaaaab/3f000000/3f2aaaab/3f555555. Key timber_{level}. These positions are a flat parameter grid, NOT CPU circular/sine geometry; real cull/shader consumers bend them using actual instance data.

Plaster kind19: one mesh label plaster, no vertex attribute table; for i in0..32 append [i|0x10000,i,i+1,i+1,i+0x10001,i|0x10000] =>192 actual encoded indices. Key plaster_strip. No added dummy vertices. Empty attributes produce radius0 in original MeshAtlas::add; shader pulls encoded geometry.

StairsFloor kind26: five mesh chunks n=4,8,16,32,64. Label stairs floor. No vertex attributes; for i in0..n append[3*i,3*i+1,3*i+2]. Key stairs_floor_chunk_{n}. Same native encoded/generated geometry policy.

Full producer also includes Smoke kind21 after those four: label grid cw, no vertex attributes; grid_gen_cw(indices,0,12,50,12), key smoke, then actual MeshAtlasLibrary::register_meshes. Smoke is not a sixth imported prefab or fake empty mesh. All keys/labels were read from PE literal bytes or actual formatting fragment pointers.

Current APIs suffice for all five procedural families: Mesh::new, set_indices, set_attribute(Vertex_Position,Vec<Vec3>), existing grid_gen_cw/ccw and MeshAtlasLibrary::add/register_meshes. Caller still needs the true prefab mesh owner/source fields; no geometry can be substituted for absent original RON/prebuilt mesh data. This handoff is original producer evidence, not runtime or whole-game readiness.

2026-10-18 independent caller/import correction: boot_chain resolved original call 0x142923fd0 through the PE import table to **sinf**, replacing this handoff's earlier unsupported cos interpretation. Full evidence: TG-1a0d194affaf7f0d17/main-chain-boot-20261018/roof-operation-constants.json. The radial square-root consumes the already transformed z. Original retained assembly and constants are unchanged.
