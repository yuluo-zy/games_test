# Root execution boundary

Do not run Cargo in template/. Copy that directory to a fresh runtime location, independently record the copy and input hashes, and run the seven source tests there with the actual locked fastrand1.8/glam0.29.3 packages. Use --offline --locked first. Any Cargo lock materialization must occur only in that runtime copy, with the final lock/package versions/checksums recorded and checked against LOCK_CONSTRAINTS_V1.json. Frozen template hashes remain unchanged.

This harness compiles one byte-exact source slice containing the three current helper functions and their original seven tests. It has no mocks, no alternative implementation, no old utils rlib and no new test matrix. Successful execution proves this numerical slice under its recorded target/features; it does not typecheck the full utils crate, restore sampling, build the App, prove the Cloth consumer/canonical Nani payload chain or exercise GPU.

Root remains the sole compiler/test runner. This package was prepared and hash-checked without executing Cargo, rustc, tests, the original executable or GPU.
