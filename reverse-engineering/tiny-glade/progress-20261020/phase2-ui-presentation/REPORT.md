# UI presentation: three complete consumers

Original B binary SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03.

## Restored algorithms and owners

* ui_focus_animation14233ba40/PDB1490B: real Query(AnimationState,mutTaffyNode,OptionSlider,OptionReset,OptionCustomSize). Target1.2 for slider/reset else1.5; custom.kind1 overrides target with its exact value. CubicOut(select), native weighted blend target*ease+(1-ease), multiplied into existing scale. No progress clamp invented.
* ui_appear_animation14233c020/PDB730B: real Query(AnimationState,mutTaffyNode); CubicOut(appear), original weighted zero expression ((1-ease)*0+ease)*existingScale preserved, including nonfinite behavior.
* ui_sounds14233a7f0/PDB3060B: all nine original SystemParams, focus stream then click stream then drag stream; no deduplication, no guessed event adapter. Focus locked10 else15. Click priority: slider suppress; locked11; PhotoButton9→29; App/Category suppress; else14. Drag only actual UiSlider members: state3→26, state2/4 none, every other raw byte→27. These priorities apply even when marker components coexist. Invalid/stale entity focus/click produces generic sounds, drag skips. Real EventReader ownership retains exactly-once stream consumption.

Production module src/ui_presentation.rs SHA256627cb6c6de301dd48f86281895b4c6a5413795d3b8098edc468955760cea3980. Host/module exports owned by central coordinator. Old ui_interaction placeholder is not used by new host.

## Native oracle and validation limits

animations-original.asm and sounds-original.asm cover full PDB function ranges, including cold tails. Native animation execution calls original EasingFunction::apply and covers1848focus/66appear cases, all optional combinations, custom kind0/1, signedzero/NaN/Inf inputs. No hooks. Full original sound business loops14233aa60..14233b271 execute without hooks:578focus+578click+147968drag events across both native old/new buffers, all32nonexclusive marker combinations, all18phototags, all256dragbytes, stalegeneration/outofrangeentities. Original output73915sound events; all3native read cursors consumed full stream. Only tracing span enter/exit is outside the selected executable oracle range; no game branch or query membership behavior is replaced.

build_actual_presentation.py extracts actual production math/classification method bodies without substitutions and actual dependency easing source. Every nonNaN output bit and all73915sound tags/order match original. All NaN results agree as NaN;29arithmeticNaNpayload/sign bits differ because Rust floating arithmetic may select another NaN operand/sign. This is explicitly not claimed as fullNaNbitidentity. It does not alter finite algorithm branches or NaN/nonNaN classification.

New actual Cargo integration target tb_i_ui_presentation exercises the real two animation Queries and real nine-param sound system. It compares1848+66native animation cases (nonNaNbits/NaNclassification),73915original sound outputs generated from149124input events, actualBevy component membership, stalegeneration, and successive scheduled runs for once-only readers and repeated clicks. Central coordinator owns Cargo; no Cargo invoked by this agent.

Test source SHA256b9c9bb48352236c02f3e7f8470d771e57ff89f80d86ec67915d1aadfe9e1eecb.
