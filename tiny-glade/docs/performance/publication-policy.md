# Remote publication condition

The user authorized publication only after implementation and acceptance are fully complete. Destination: `git@github.com:yuluo-zy/games_test.git`, proposed new branch `codex/perf-windows-20261026`, continuing work sourced from `codex/render-cpu-cleanup-20261026`. This authorization replaces the original delivery boundary of keeping everything local; it does not waive the acceptance gates.

The source copy was commit `6957077f905ab1ef1bb9d16ca3521ce555fa488e`; its upstream branch parent was `8fd2a3c2c90d4f241332bee1a595e7f038a76546`. The independent `C:/game` Git root is a source-only import. Publication must replay its reviewed changes under the upstream `tiny-glade/` prefix, preserve upstream history and every unrelated path, and verify final file modes and blobs. It must not replace the upstream branch with the independent import root.

No force push, mirror push, deletion or overwrite of the source branch is authorized. The dedicated publication preparation and pre-push guard are staged outside the project, with tests and manifests; they have not been installed or executed against GitHub. Before eventual publication, refresh remote refs, validate the replay and guard, and push only the verified new branch. A changed or existing target ref requires re-evaluation rather than overwriting it.

Publication remains pending: release four-scenario CPU acceptance, probe overhead, strict water/ice pixels, pressure/stability/lifecycle acceptance and platform verification are still open. Windows WSI capability work was deferred by the user. Headless GPU tests do not satisfy the production-window or macOS gates.
