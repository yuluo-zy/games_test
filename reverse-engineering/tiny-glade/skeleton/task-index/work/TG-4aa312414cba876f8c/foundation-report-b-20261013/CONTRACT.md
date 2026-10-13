# B package verification report consumer

Observed: original B RNE SHA f507a6ce...b7d03; patched shared metadata identifies tiny-glade.exe at expected B path. Fresh 0x1402552b0 decompilation and original machine instructions are in companion TG-7b281633ac9256cad6 work directory. PDB ABI void() remains placeholder.

Input: chosen package root (candidate integration parameter; original path discovery is not recovered). Producer: existing tg_package_integrity::verify_package_manifest. All variants, including success, continue into window/report creation. Result is moved into Controls without pre-normalizing its error or re-verifying. GUI consumer: existing run_integrity_report_gui, external GuiBackend implements event loop.

Native window policy: title `Tiny Glade file verification report`. 0x140255dde compares result tag against 0x8000000000000004, 0x140255e04 sets the success byte, 0x140256075 shifts it by 3. Double tables 0x1429a2fd0/0x1429a2fe0 provide error 1000×800 and success 500×300. Both paths run GUI; no early return for success.

Unknown: original exact root discovery, event-loop error ABI, logging sink and complete native GUI lifetime equivalence. Original logs success with elapsed time or category/list errors; this candidate does not claim those diagnostic effects. Existing Controls deletion/normalization remains the editor consumer; no historical/replay semantics inferred. Package hashing, serde, iced/winit remain reused dependencies.

Candidate delivery: append report_consumer.rs to tg-integrity-report/src/lib.rs; tiny-glade companion module reexports this callable game entry. Parent must integrate; no shared source modified. This is implemented_unverified B projection, not C recovery.

Validation: bounded tests run against actual existing package verifier and report/backend entry; manifest read failure is presented exactly once, backend errors propagate, native success dimensions checked. Full original entry runtime is unverified. Dependency comparison from shared lock seed initially found 6 transitive mismatches; report retained, reseeded with original lock. Final projection is 42/42 public package identities/checksums matched; local path implementations remain recovered candidates.
