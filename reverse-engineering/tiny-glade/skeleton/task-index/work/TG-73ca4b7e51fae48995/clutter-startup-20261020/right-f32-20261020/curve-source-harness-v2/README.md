# Exact complete-source curve test driver V2

Root-only Cargo/compiler. Frozen template is copied to runtime-v1 before any Cargo operation. Seed runtime Cargo.lock from preserved actual workspace lock, run offline metadata, compare actual lock public identities using native-rust-reconstruction compare script, then locked offline Cargo test --lib --test curve_properties --test curve_review. Capture any errors without source/template edits or fallback; freeze a new version for repairs. Retain each actual test executable immediately and bind source/lock/log hashes.

The synthetic lib only declares eight actual existing modules; all their complete module source and cfg tests, include/path dependencies and two integration tests are byte-exact current utils files. No business-function copy/rewrite/mock/stub, missing sampling or old utils rlib is used. Scope is selected complete modules plus current tests/fixtures; no full utils crate/core/App/native executable/scene/GPU/performance acceptance. The missing canonical sampling declaration is unchanged and still blocks full workspace build.

V1 retained with changes_required: actual include fixture needed utils::components::transform. V2 adds complete actual canonical utils components/mod.rs and transform.rs byte-exactly, with only a module declaration in driver. No fake core module/type or test deletion/ignore was added.
