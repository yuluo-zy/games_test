# rhapsody PDB project scaffold

This crate materializes the current Tiny Glade build's PDB-observed `rhapsody` source hierarchy as an isolated Cargo project.

The PDB establishes 55 compiled contributing paths: two crate-root files, 29 direct `raw` files, nine `raw/managed` files, and 15 `rg` files. Their existing comments retain original paths, source checksums, compilands, native addresses and code sizes.

`src/raw/mod.rs`, `src/raw/managed/mod.rs`, and `src/rg/mod.rs` are synthetic reconstruction glue. The PDB did not enumerate those files. Public module declarations are a scaffold choice so later recovered APIs can be addressed; they do not establish the original visibility.

`Cargo.toml` uses the lock-confirmed package name/version and records the original direct dependency names as metadata. It intentionally does not activate those dependencies yet because the original manifest's dependency kinds, features, target conditions and several local/fork sources are not recovered.

Passing `cargo check` proves only that the module tree is internally complete. It does not prove any function, type, ABI, ownership rule or renderer behavior has been recovered. Machine-readable scope and limitations are in `structure.json`.
