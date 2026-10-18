# B ColorLibrary startup producer

Image SHA-256: `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.
MCP database: patched `tiny-glade.exe`, revision `1791719479858:b17d372d:3`; raw bytes independently checked against original B RNE.
Only B behavior is used; the A/B/C file-family claim is mutual exclusion, not evidence about version C.

## Original sequence

- `14082c027`: `ColorLibrarySsbo::default` at `14093f900`, 245-byte full body. It writes 37 equal 16-byte entries (last offset +0x240), a 592-byte palette.
- `14082c04a`: copy 0x250 bytes; `14082c04f` sets element count 1; `14082c05e` calls `AssetSsboLibrary::add_new_with_data<ColorLibrarySsbo>` at `1412bbcf0`.
- `14082c08a..0b7`: append the real Commands insert-resource closure and palette to the queue; entry consumes 600 bytes including closure pointer.
- `14082c0e2..100`: copy the 18-byte path `assets/colors.json`; `14082c111` calls `ConfigWatch::watch` at `1408ba290`.
- No JSON read, theme selection, update loop, or LoadingGameComplete event exists here.

## Source mapping and limits

`country-core/startup/init_color_library.rs` retains this creation and registration behavior and returns the original deferred-insertion payload to the application owner. `tiny-glade/loading_graph.rs` inserts it into the same World using that App's actual `HostTables.buffers`; it cannot use another App's same-numbered handle table.
Loading ConfigWatch(false) is confirmed at original `run_game_thread` 1405a4d93..1405a4dab. Gameplay watcher enablement has a distinct DevSettings::live_link producer and must retain that separate branch.
The local host invokes the bounded producer during GPU setup. It does not claim that the complete original startup tuple or the complete gameplay palette/theme flow has been restored.

## Validation

Pending strict local GPU execution of the original packaged loading shaders. The original-loading test uses this producer, not a manually inserted dummy SSBO.
