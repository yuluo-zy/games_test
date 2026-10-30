# M3 B PE audit — 2026-10-30

Binary `D:/game/ljxsj_92385/Tiny Glade/tiny-glade.exe`, SHA-256
`cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`.
Run `D:/tools/reverse-engineering/venvs/reverse-engineering-assistant/Scripts/python.exe m3-pe-audit.py`.
The script reads B and production source, disassembles with LLVM, executes a
copied original leaf in memory and compiles the extracted Rust method with
`rustc`; it does not launch the game or run Cargo.

* `m3-pe-instruction-index.json`: 3,895 decoded instructions and 127 call / indirect
  jump records in five explicit windows. Memory operands are raw instruction
  operands, not an alias-complete read/write classification. Windows may include
  compiler funclets; they are not proof of a source function's entire CFG.
* `m3-pe-direct-targets.json`: 29 unique direct targets, 26 bounded by PE x64
  runtime-function metadata, the `0x140c97540` leaf bounded by its ret, and
  two unresolved targets `0x142923bf0`, `0x142923c20`. A runtime-function
  range is not a complete transitive call graph or a semantic proof.
* `m3-commit-event-index.json`: each of four instances' point-count branch and
  queue/sequence store addresses, cross-checked as instruction boundaries. Event
  names depend on earlier TypeId bindings; this table does not prove the absence
  of other callback effects.
* `m3-drawstate-native-diff.json`: B `0x140ae4bd0` bytes copied directly from the
  verified PE and called via Windows ABI; all 256 values of each of the state,
  mode and raw bytes, compared bytewise against the actual extracted
  `DrawState::update` method compiled by rustc. 16,777,216/16,777,216 match. Input
  matrix, output hashes, code and source hashes and first mismatch
  are recorded. This does not test draw_path, scheduling or UI.

The current two target command words and callback semantics remain open. Raw
`0x140ae4f34 -> 0x140c97540` defines shared `rsp+0x9c` for drawing modes;
target `rsp+0x8c` and eraser `rsp+0x9c` lack a proven source. In particular,
the four cross-variant functions receive a pointer to the latter; use the
instruction and target indexes to verify their argument aliases, indirect
dispatch and return paths before declaring preservation or callback effects.
The two missing target ranges and indirect call/jump targets are not excluded.
On begin, the four explicit second-word reads are Path `0x140ab3b87`, C1
`0x140ab291f`, C2 `0x140ab73bf`, and C3 `0x140ab866f`; each is forwarded to
`ActiveRasterStroke::start_new@0x140868d00`. Each instance also has two
indirect vtable calls, respectively Path `0x140ab3b59/0x140ab46de`, C1
`0x140ab28f1/0x140ab3575`, C2 `0x140ab7391/0x140ab8015`, C3
`0x140ab8641/0x140ab92c5`; they are not resolved by the raw index itself.

Search of `tiny-glade/crates/tiny-glade/src` and `country-core/src` found
`draw_path::draw_path` and `path_brush_preview_size::path_brush_preview_size`
only in local path tests, not a production registration by those names. Prior
PathPlugin evidence identifies original set 16 and draw-path then preview order;
formal App resource/event registration, scheduling, and end-to-end acceptance
are separate unverified work. No production source was changed by this audit.
