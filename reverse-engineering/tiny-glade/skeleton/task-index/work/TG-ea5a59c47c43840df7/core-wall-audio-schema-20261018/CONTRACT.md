# B wall material request schema integration

Necessary shared adaptation for original draw_wall producer. Existing audio orchestrator algorithm stays unchanged. Original typed SystemParam slot6 proves EventWriter<RequestWallMaterialAudio>; B140a7ed56..ed9c writes wall u64, Vec3, kind u8=2, flag u8=0, EventInstance stride32 / payload24. Original helper141327750. Evidence: TG-772e8f649ffb32fc5a/terrain-curve-20261018/native-pack/draw-wall.{json,asm.txt}, original-draw-ports.json, report.json.

Current struct adds an unproved value:f32 and lacks the actual flag. No current production constructor uses that scalar, and the current orchestrator only reads id before delegating its existing host-change abstraction. Minimal correction removes value, retains id/position/kind and adds flag as raw u8 until the complete consumer discriminant is mined. Rust event layout is not copied from native memory.

This is a schema/port correction, not a recovered independent business function or acceptance of the complete original material-statistics consumer. Status: candidate; root applies only after current source-bound verification finishes.
