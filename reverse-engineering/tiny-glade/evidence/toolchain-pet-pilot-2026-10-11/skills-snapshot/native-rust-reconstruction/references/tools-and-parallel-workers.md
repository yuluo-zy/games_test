# Analysis tools and parallel workers

For an installed project toolchain, follow its applicable `AGENTS.md` and AI tool guide before inventing a client. On this host, Tiny Glade's executable commands, bounded JSON jobs and schema snapshots are in [AI_REVERSE_TOOLS.md](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/AI_REVERSE_TOOLS.md). Read only the needed backend section and example. Keep target paths, addresses and layouts in that project guide/evidence rather than this general skill.

Choose a backend by evidence needed, not whether it appeared in an old configuration. Initialization and tool discovery do not establish function recovery, correct target binding, or durable writes. Reuse a matching analyzed database and explicit artifact identity.

- Ghidra MCP: focused/batched function bundles, references, memory and confirmed signature/type overlays. Use selected fields; do not enumerate a whole large program to answer one function question.
- ReVa: PyGhidra inside the retained program, High P-code and batched custom predicates; focused decompilation and signature repair. Avoid global function/caller cache construction for a narrow query. Request incoming reference context only when needed. Script result success, stderr and timeout flags matter independently of the MCP error flag.
- RE-MCP: supervisor with independent database workers, useful for isolated multi-target or cross-version work. Explicit database selectors are mandatory. Check installed-version save/close compatibility and actual persistence. Stage copies before a backend that creates projects alongside its input; keep release trees immutable. A shared supervisor can dispatch independent workers concurrently, while each worker's Ghidra API executor may serialize operations.
- REA: independent artifact and native analysis on an ephemeral session. Do not expect it to inherit annotations from a user Ghidra project or preserve session edits after close. Prefer an optional SDK job over repeatedly loading a full catalog.
- radare2: original bytes, local disassembly, calls and an independent decompiler when its plugin is installed. PDB recognition, public symbols and TPI/variable recovery are separate capabilities. Use LLVM when the installed reader cannot handle the actual PDB type stream.

Parallel clients can share reads. Concurrent writes need distinct owned analysis copies or a single integration writer; a database lock alone does not prevent one agent from overwriting another agent's interpretation. Keep binary identity, worker identity, program/database selector and revision on every result. Recheck a revision after an accepted annotation change.

For the local reverse operator, use a distinct `TG_MCP_WORKER` or `--worker` name per modifying agent and select `TG_MCP_ACCESS=write` or `--write` only for that worker. Shared profiles are read-only. Writable workers retain their own project clones. Schema and overlays from one baseline do not apply to another by address coincidence.

ReVa inline scripts do not supply an automatic transaction: confirmed mutations require `startTransaction`/`endTransaction` with rollback on failure. Long passes must yield to the monitor. Isolate named script files by worker as well as database writes.

Test concurrent requests against different functions and byte ranges, and same-address requests against different targets. Assert identity, distinct outputs, lack of cross-session stdout, durable save/reopen, and unchanged source artifacts. Separate request overlap from measured CPU parallelism; do not invent a speed multiplier.

Individual worker overlays remain candidates until review and integration. Preserve source/revision hashes and export a patch or annotation ledger; never merge analysis databases or replace another worker's project wholesale merely because both copies compile or decompile successfully.
