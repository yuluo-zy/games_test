# Independent review: B Delete's separate height

Accepted for the bounded schema/factory/Some-source replay increment, pending root compilation and current ECS regression execution. No blocking findings. This review made no production changes and ran neither Cargo nor original native instructions.

Input freeze: `TG-eb56a08d047af785ea/core-wall-height-20261018/frozen-inputs.json`, SHA256 `e073912c8b0d3de80ebefa6779c9c4dc6b7f583a207002ab1cb730179ddbead8`. All34 input sizes/hashes were checked independently. The six native packs' 381 instructions/2080 decoded bytes (including declared3-byte stop overhang) match B PE byte for byte, and all five serializer RIP keys match the original image. `verify_candidate.py` is rerunnable and records the result.

Original facts read before the patch:

* Producer `140a81077` captures progress+94 into xmm0. `140a81129` independently stores it to record+80, while the previous WallState block is copied to record+8; wall-index qword and color dword remain separate. Discriminator at `140a81199` is MIN+1. The candidate correctly refrains from mutating the snapshot's source.extra to store the independent value.
* Serializer `1414009b8/9e6/a0d/a31` maps record+78/+80/+8/+84 to wall_index/wall_height/before/color_id. The original strings were checked at the actual RIP target bytes. Candidate custom serde preserves existing saved names and the existing undo_instant_state alias, and now transports the previously rejected unequal height losslessly.
* Delete operation's undo gate is `140ac04b3`; None sentinel branches to `140ac1322`. Some loads source height from Arc+6c at `140ac12be`, keeps xmm1 across branch, and stores it to rebuilt WallState+54 at `140ac14c0`. The Some path reaches common new_wall_with_id through `140ac13e9..1461`. Independent Arc+90 is loaded only by the None fallback at1322. The separate Create overwrite at1252 is real and must not be applied to Delete.
* Existing generic deleted-wall factory reads source+54 into xmm6 at `140a87706`, then writes both the snapshot height and record+80 at `140a87886/7863`. Keeping the existing three-argument factory and delegating its default from source.source.extra is correct; the explicit factory is needed for the actual continuation exception.

Candidate review:

* WallHistoryEdit::Delete gains one scalar; its existing variant/discriminator and mandatory snapshot architecture remain. Some-source apply changes only its match pattern and leaves Create's override branch exclusive to Create.
* RecordWallHistory adds a wrapper method instead of parallel history/state storage. The existing API remains. Direct Delete literals in replay fixture, two eraser archivist producers and bulldoze are all adapted with their preexisting captured height. Existing nonconstructing matches use `..`; no other current direct literals were found in country-core/systems.
* New internal regressions check independent7/-0/NaN bits, preserve old3.9 during undo, remove during redo, and preserve the default factory. Serde regression checks finite old3.9/independent7 and legacy alias; it does not claim JSON can preserve NaN payloads. Actual ECS regression goes through existing replay_wall and checks source height, restore-color event and failure events. These tests are meaningful source assertions but have not been executed by this reviewer.
* Both patches apply together to all six captured bases and reproduce all six candidate files byte for byte. At review time all six current production files still exactly matched those bases.

Boundary: current WallAuthoritySnapshot is mandatory. The original before:null/None fallback still cannot be deserialized or replayed by this model. The candidate explicitly preserves this gap rather than fabricating a snapshot; retaining the scalar is a necessary lossless schema fix, not closure of the None producer/consumer. Continuation's explicit desired-height producer lives in its separate candidate and must be integrated in the same batch when claiming that chain. Eraser algorithms, other wall variants, actual host scheduling and GPU publication are outside this review.
