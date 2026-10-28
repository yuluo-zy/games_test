# Committed resource version index

RenderGraph maintains one version per logical resource, updated only when a completed PassBuilder publishes its declaration. Abandoned builders do not advance versions. Adopted retained passes populate the same index; conversion from publicly editable BuiltGraph declarations rebuilds it once. Graph scope, stale version checks, automatic writes, explicit writes, read-modify-write and sticky overflow retain their previous semantics.

Validation: random 1,024-pass declarations compared against the previous full scan after each commit/abandon; explicit overflow test; resource_contracts 21/21. Additional replay/roundtrip checks are included for the public declaration boundary. Diagnostic debug run: 100,000 queries, 877 committed passes, indexed 5,302,500ns versus scan 3,793,447,200ns. This is a local query diagnostic, not an end-to-end frame speedup. CPU metadata overhead is one Option<u32> per graph resource.
