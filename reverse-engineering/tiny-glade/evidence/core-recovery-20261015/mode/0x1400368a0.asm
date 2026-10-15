0x1400368a0 push     r15
0x1400368a2 push     r14
0x1400368a4 push     rsi
0x1400368a5 push     rdi
0x1400368a6 push     rbp
0x1400368a7 push     rbx
0x1400368a8 sub      rsp, 0x58
0x1400368ac mov      r15d, dword ptr [rcx]
0x1400368af lea      eax, [r15 - 6]
0x1400368b3 mov      r8, r15
0x1400368b6 add      r8, -5
0x1400368ba xor      r9d, r9d
0x1400368bd cmp      eax, 0xf
0x1400368c0 cmovb    r9, r8
0x1400368c4 mov      rsi, rdx
0x1400368c7 lea      rax, [rip + 0x28f409a]
0x1400368ce movsxd   rdx, dword ptr [rax + r9*4]
0x1400368d2 add      rdx, rax
0x1400368d5 jmp      rdx
0x1400368d7 lea      rdi, [rsi + 8]
0x1400368db mov      rbx, qword ptr [rsi]
0x1400368de mov      rdx, qword ptr [rbx + 0x10]
0x1400368e2 cmp      qword ptr [rbx], rdx
0x1400368e5 je       0x140037198
0x1400368eb mov      r14, rcx
0x1400368ee mov      rax, qword ptr [rbx + 8]
0x1400368f2 mov      byte ptr [rax + rdx], 0x7b
0x1400368f6 inc      rdx
0x1400368f9 mov      qword ptr [rbx + 0x10], rdx
0x1400368fd lea      r8, [rip + 0x28f3cce] ; @0x14292a5d2 'EditingCobblestoneEraserDecoratorColorReplayLoadGameFencesPhotomodeCloneClutterFreeformCircleRectang'
0x140036904 mov      r9d, 7
0x14003690a mov      rcx, rsi
0x14003690d mov      rdx, rdi
0x140036910 call     0x1404b6100 ; serde_json::ser::format_escaped_str::hc891560d12372fd0
0x140036915 test     rax, rax
0x140036918 jne      0x140036d51
0x14003691e mov      rbx, qword ptr [rsi]
0x140036921 mov      rdx, qword ptr [rbx + 0x10]
0x140036925 cmp      qword ptr [rbx], rdx
0x140036928 je       0x140037303
0x14003692e mov      rax, qword ptr [rbx + 8]
0x140036932 mov      byte ptr [rax + rdx], 0x3a
0x140036936 inc      rdx
0x140036939 mov      qword ptr [rbx + 0x10], rdx
0x14003693d mov      eax, r15d
0x140036940 lea      rcx, [rip + 0x28f407d]
0x140036947 movsxd   rax, dword ptr [rcx + rax*4]
0x14003694b add      rax, rcx
0x14003694e jmp      rax
0x140036950 lea      r8, [rip + 0x28f3c29] ; @0x14292a580 'NoneWallRoofDecoClutterdecorator_idStairPointTerrainAllPathBulldozeWaterGardenTreeEditingCobblestone'
0x140036957 jmp      0x1400370a3 ; 
0x14003695c lea      rdx, [rsi + 8]
0x140036960 lea      r8, [rip + 0x28f3ca5] ; @0x14292a60c 'PhotomodeCloneClutterFreeformCircleRectangle'
0x140036967 mov      r9d, 9
0x14003696d jmp      0x140036d40 ; 
0x140036972 lea      rdx, [rsi + 8]
0x140036976 lea      r8, [rip + 0x28f3c51] ; @0x14292a5ce 'TreeEditingCobblestoneEraserDecoratorColorReplayLoadGameFencesPhotomodeCloneClutterFreeformCircleRec'
0x14003697d jmp      0x140036ac4 ; 
0x140036982 movzx    ebp, byte ptr [rcx + 4]
0x140036986 lea      rdi, [rsi + 8]
0x14003698a mov      rbx, qword ptr [rsi]
0x14003698d mov      rdx, qword ptr [rbx + 0x10]
0x140036991 cmp      qword ptr [rbx], rdx
0x140036994 je       0x140037114
0x14003699a mov      rax, qword ptr [rbx + 8]
0x14003699e mov      byte ptr [rax + rdx], 0x7b
0x1400369a2 inc      rdx
0x1400369a5 mov      qword ptr [rbx + 0x10], rdx
0x1400369a9 lea      r8, [rip + 0x28f3c34] ; @0x14292a5e4 'EraserDecoratorColorReplayLoadGameFencesPhotomodeCloneClutterFreeformCircleRectangle'
0x1400369b0 mov      r9d, 6
0x1400369b6 mov      rcx, rsi
0x1400369b9 mov      rdx, rdi
0x1400369bc call     0x1404b6100 ; serde_json::ser::format_escaped_str::hc891560d12372fd0
0x1400369c1 test     rax, rax
0x1400369c4 jne      0x140036d51
0x1400369ca mov      rbx, qword ptr [rsi]
0x1400369cd mov      rdx, qword ptr [rbx + 0x10]
0x1400369d1 cmp      qword ptr [rbx], rdx
0x1400369d4 je       0x140037219
0x1400369da mov      rax, qword ptr [rbx + 8]
0x1400369de mov      byte ptr [rax + rdx], 0x3a
0x1400369e2 inc      rdx
0x1400369e5 mov      qword ptr [rbx + 0x10], rdx
0x1400369e9 mov      eax, ebp
0x1400369eb lea      rcx, [rip + 0x28f3fb6]
0x1400369f2 movsxd   rax, dword ptr [rcx + rax*4]
0x1400369f6 add      rax, rcx
0x1400369f9 jmp      rax
0x1400369fb lea      r8, [rip + 0x28f3bb2] ; @0x14292a5b4 'AllPathBulldozeWaterGardenTreeEditingCobblestoneEraserDecoratorColorReplayLoadGameFencesPhotomodeClo'
0x140036a02 mov      r9d, 3
0x140036a08 jmp      0x1400370a9 ; 
0x140036a0d lea      rdx, [rsi + 8]
0x140036a11 lea      r8, [rip + 0x28f3bb0] ; @0x14292a5c8 'GardenTreeEditingCobblestoneEraserDecoratorColorReplayLoadGameFencesPhotomodeCloneClutterFreeformCir'
0x140036a18 jmp      0x140036d3a ; 
0x140036a1d lea      rdx, [rsi + 8]
0x140036a21 mov      rdi, qword ptr [rsi]
0x140036a24 mov      rax, qword ptr [rdi + 0x10]
0x140036a28 cmp      qword ptr [rdi], rax
0x140036a2b je       0x14003713a
0x140036a31 mov      rcx, qword ptr [rdi + 8]
0x140036a35 mov      byte ptr [rcx + rax], 0x7b
0x140036a39 inc      rax
0x140036a3c mov      qword ptr [rdi + 0x10], rax
0x140036a40 lea      r8, [rip + 0x28f3bb7] ; @0x14292a5fe 'LoadGameFencesPhotomodeCloneClutterFreeformCircleRectangle'
0x140036a47 mov      r9d, 8
0x140036a4d mov      rcx, rsi
0x140036a50 call     0x1404b6100 ; serde_json::ser::format_escaped_str::hc891560d12372fd0
0x140036a55 test     rax, rax
0x140036a58 jne      0x140036d51
0x140036a5e mov      rdi, qword ptr [rsi]
0x140036a61 mov      rdx, qword ptr [rdi + 0x10]
0x140036a65 cmp      qword ptr [rdi], rdx
0x140036a68 je       0x14003723f
0x140036a6e mov      rax, qword ptr [rdi + 8]
0x140036a72 mov      byte ptr [rax + rdx], 0x3a
0x140036a76 inc      rdx
0x140036a79 mov      qword ptr [rdi + 0x10], rdx
0x140036a7d mov      rdi, qword ptr [rsi]
0x140036a80 mov      rdx, qword ptr [rdi + 0x10]
0x140036a84 cmp      qword ptr [rdi], rdx
0x140036a87 je       0x140037265
0x140036a8d mov      rax, qword ptr [rdi + 8]
0x140036a91 mov      byte ptr [rax + rdx], 0x7b
0x140036a95 inc      rdx
0x140036a98 mov      qword ptr [rdi + 0x10], rdx
0x140036a9c cmp      qword ptr [rdi], rdx
0x140036a9f je       0x14003728b
0x140036aa5 mov      rax, qword ptr [rdi + 8]
0x140036aa9 mov      byte ptr [rax + rdx], 0x7d
0x140036aad inc      rdx
0x140036ab0 mov      qword ptr [rdi + 0x10], rdx
0x140036ab4 jmp      0x1400370bd ; 
0x140036ab9 lea      rdx, [rsi + 8]
0x140036abd lea      r8, [rip + 0x28f3af3] ; @0x14292a5b7 'PathBulldozeWaterGardenTreeEditingCobblestoneEraserDecoratorColorReplayLoadGameFencesPhotomodeCloneC'
0x140036ac4 mov      r9d, 4
0x140036aca jmp      0x140036d40 ; 
0x140036acf lea      rdx, [rsi + 8]
0x140036ad3 lea      r8, [rip + 0x28f3b3b] ; @0x14292a615 'CloneClutterFreeformCircleRectangle'
0x140036ada mov      r9d, 0xc
0x140036ae0 jmp      0x140036d40 ; 
0x140036ae5 mov      rdi, qword ptr [rcx + 8]
0x140036ae9 lea      rdx, [rsi + 8]
0x140036aed mov      rbx, qword ptr [rsi]
0x140036af0 mov      rax, qword ptr [rbx + 0x10]
0x140036af4 cmp      qword ptr [rbx], rax
0x140036af7 je       0x140037169
0x140036afd mov      rcx, qword ptr [rbx + 8]
0x140036b01 mov      byte ptr [rcx + rax], 0x7b
0x140036b05 inc      rax
0x140036b08 mov      qword ptr [rbx + 0x10], rax
0x140036b0c lea      r8, [rip + 0x28f3ac6] ; @0x14292a5d9 'CobblestoneEraserDecoratorColorReplayLoadGameFencesPhotomodeCloneClutterFreeformCircleRectangle'
0x140036b13 mov      r9d, 0xb
0x140036b19 mov      rcx, rsi
0x140036b1c call     0x1404b6100 ; serde_json::ser::format_escaped_str::hc891560d12372fd0
0x140036b21 test     rax, rax
0x140036b24 jne      0x140036d51
0x140036b2a mov      rbx, qword ptr [rsi]
0x140036b2d mov      rdx, qword ptr [rbx + 0x10]
0x140036b31 cmp      qword ptr [rbx], rdx
0x140036b34 je       0x1400372b1
0x140036b3a mov      rax, qword ptr [rbx + 8]
0x140036b3e mov      byte ptr [rax + rdx], 0x3a
0x140036b42 inc      rdx
0x140036b45 mov      qword ptr [rbx + 0x10], rdx
0x140036b49 mov      eax, 0x14
0x140036b4e lea      rcx, [rip + 0x28f3cbb] ; @0x14292a810 '0001020304050607080910111213141516171819202122232425262728293031323334353637383940414243444546474849'
0x140036b55 cmp      rdi, 0x2710
0x140036b5c jb       0x140036d65
0x140036b62 mov      r9d, 0x14
0x140036b68 movabs   r8, 0x346dc5d63886594b
0x140036b72 nop      word ptr cs:[rax + rax]
0x140036b80 mov      rax, rdi
0x140036b83 mul      r8
0x140036b86 shr      rdx, 0xb
0x140036b8a imul     eax, edx, 0x2710
0x140036b90 mov      r10d, edi
0x140036b93 sub      r10d, eax
0x140036b96 imul     r11d, r10d, 0x147b
0x140036b9d shr      r11d, 0x13
0x140036ba1 imul     eax, r11d, 0x64
0x140036ba5 sub      r10d, eax
0x140036ba8 movzx    r10d, r10w
0x140036bac lea      rax, [r9 - 4]
0x140036bb0 movzx    r11d, word ptr [rcx + r11*2]
0x140036bb5 mov      word ptr [rsp + r9 + 0x2c], r11w
0x140036bbb movzx    r10d, word ptr [rcx + r10*2]
0x140036bc0 mov      word ptr [rsp + r9 + 0x2e], r10w
0x140036bc6 mov      r9, rax
0x140036bc9 cmp      rdi, 0x5f5e0ff
0x140036bd0 mov      rdi, rdx
0x140036bd3 ja       0x140036b80
0x140036bd5 jmp      0x140036d68 ; 
0x140036bda lea      rdx, [rsi + 8]
0x140036bde lea      r8, [rip + 0x28f3a0e] ; @0x14292a5f3 'ColorReplayLoadGameFencesPhotomodeCloneClutterFreeformCircleRectangle'
0x140036be5 mov      r9d, 5
0x140036beb jmp      0x140036d40 ; 
0x140036bf0 movzx    ebp, byte ptr [rcx + 4]
0x140036bf4 lea      rdi, [rsi + 8]
0x140036bf8 mov      rbx, qword ptr [rsi]
0x140036bfb mov      rdx, qword ptr [rbx + 0x10]
0x140036bff cmp      qword ptr [rbx], rdx
0x140036c02 je       0x1400371c1
0x140036c08 mov      rax, qword ptr [rbx + 8]
0x140036c0c mov      byte ptr [rax + rdx], 0x7b
0x140036c10 inc      rdx
0x140036c13 mov      qword ptr [rbx + 0x10], rdx
0x140036c17 lea      r8, [rip + 0x28f3966] ; @0x14292a584 'WallRoofDecoClutterdecorator_idStairPointTerrainAllPathBulldozeWaterGardenTreeEditingCobblestoneEras'
0x140036c1e mov      r9d, 4
0x140036c24 mov      rcx, rsi
0x140036c27 mov      rdx, rdi
0x140036c2a call     0x1404b6100 ; serde_json::ser::format_escaped_str::hc891560d12372fd0
0x140036c2f test     rax, rax
0x140036c32 jne      0x140036d51
0x140036c38 mov      rbx, qword ptr [rsi]
0x140036c3b mov      rdx, qword ptr [rbx + 0x10]
0x140036c3f cmp      qword ptr [rbx], rdx
0x140036c42 je       0x140037329
0x140036c48 mov      rax, qword ptr [rbx + 8]
0x140036c4c mov      byte ptr [rax + rdx], 0x3a
0x140036c50 inc      rdx
0x140036c53 mov      qword ptr [rbx + 0x10], rdx
0x140036c57 test     ebp, ebp
0x140036c59 je       0x140036dfb
0x140036c5f cmp      ebp, 1
0x140036c62 jne      0x140036e0d
0x140036c68 lea      r8, [rip + 0x28f39ba] ; @0x14292a629 'CircleRectangle'
0x140036c6f mov      r9d, 6
0x140036c75 jmp      0x1400370a9 ; 
0x140036c7a lea      rdx, [rsi + 8]
0x140036c7e lea      r8, [rip + 0x28f3928] ; @0x14292a5ad 'TerrainAllPathBulldozeWaterGardenTreeEditingCobblestoneEraserDecoratorColorReplayLoadGameFencesPhoto'
0x140036c85 mov      r9d, 7
0x140036c8b jmp      0x140036d40 ; 
0x140036c90 lea      rdx, [rsi + 8]
0x140036c94 lea      r8, [rip + 0x28f396b] ; @0x14292a606 'FencesPhotomodeCloneClutterFreeformCircleRectangle'
0x140036c9b jmp      0x140036d3a ; 
0x140036ca0 lea      rdx, [rsi + 8]
0x140036ca4 mov      rdi, qword ptr [rsi]
0x140036ca7 mov      rax, qword ptr [rdi + 0x10]
0x140036cab cmp      qword ptr [rdi], rax
0x140036cae je       0x1400371e7
0x140036cb4 mov      rbx, rcx
0x140036cb7 mov      rcx, qword ptr [rdi + 8]
0x140036cbb mov      byte ptr [rcx + rax], 0x7b
0x140036cbf inc      rax
0x140036cc2 mov      qword ptr [rdi + 0x10], rax
0x140036cc6 lea      r8, [rip + 0x28f391d] ; @0x14292a5ea 'DecoratorColorReplayLoadGameFencesPhotomodeCloneClutterFreeformCircleRectangle'
0x140036ccd mov      r9d, 9
0x140036cd3 mov      rcx, rsi
0x140036cd6 call     0x1404b6100 ; serde_json::ser::format_escaped_str::hc891560d12372fd0
0x140036cdb test     rax, rax
0x140036cde jne      0x140036d51
0x140036ce0 mov      rcx, rbx
0x140036ce3 add      rcx, 4
0x140036ce7 mov      rdi, qword ptr [rsi]
0x140036cea mov      rdx, qword ptr [rdi + 0x10]
0x140036cee cmp      qword ptr [rdi], rdx
0x140036cf1 je       0x14003734f
0x140036cf7 mov      rax, qword ptr [rdi + 8]
0x140036cfb mov      byte ptr [rax + rdx], 0x3a
0x140036cff inc      rdx
0x140036d02 mov      qword ptr [rdi + 0x10], rdx
0x140036d06 mov      rdx, rsi
0x140036d09 call     0x1402de600 ; country_core::resources::walls::decorator::_::_$LT$impl$u20$serde..ser..Serialize$u20$for$u20$country_core..resources..walls..decorator..DecoratorType$GT$::serialize::h16fab12297f9a7f7
0x140036d0e test     rax, rax
0x140036d11 jne      0x1400370da
0x140036d17 jmp      0x1400370bd ; 
0x140036d1c lea      rdx, [rsi + 8]
0x140036d20 lea      r8, [rip + 0x28f389c] ; @0x14292a5c3 'WaterGardenTreeEditingCobblestoneEraserDecoratorColorReplayLoadGameFencesPhotomodeCloneClutterFreefo'
0x140036d27 mov      r9d, 5
0x140036d2d jmp      0x140036d40 ; 
0x140036d2f lea      rdx, [rsi + 8]
0x140036d33 lea      r8, [rip + 0x28f38be] ; @0x14292a5f8 'ReplayLoadGameFencesPhotomodeCloneClutterFreeformCircleRectangle'
0x140036d3a mov      r9d, 6
0x140036d40 mov      rcx, rsi
0x140036d43 call     0x1404b6100 ; serde_json::ser::format_escaped_str::hc891560d12372fd0
0x140036d48 test     rax, rax
0x140036d4b je       0x1400370d8
0x140036d51 mov      rcx, rax
0x140036d54 add      rsp, 0x58
0x140036d58 pop      rbx
0x140036d59 pop      rbp
0x140036d5a pop      rdi
0x140036d5b pop      rsi
0x140036d5c pop      r14
0x140036d5e pop      r15
0x140036d60 jmp      0x1428c98a0 ; serde_json::error::Error::io::hfcf27584d0d6aa9f
0x140036d65 mov      rdx, rdi
0x140036d68 cmp      rdx, 0x63
0x140036d6c jbe      0x140036d9b
0x140036d6e movzx    r8d, dx
0x140036d72 shr      r8d, 2
0x140036d76 imul     r8d, r8d, 0x147b
0x140036d7d shr      r8d, 0x11
0x140036d81 imul     r9d, r8d, 0x64
0x140036d85 sub      edx, r9d
0x140036d88 movzx    edx, dx
0x140036d8b movzx    edx, word ptr [rcx + rdx*2]
0x140036d8f mov      word ptr [rsp + rax + 0x2e], dx
0x140036d94 add      rax, -2
0x140036d98 mov      rdx, r8
0x140036d9b mov      rdi, qword ptr [rsi]
0x140036d9e cmp      rdx, 0xa
0x140036da2 jae      0x140036db0
0x140036da4 or       dl, 0x30
0x140036da7 mov      byte ptr [rsp + rax + 0x2f], dl
0x140036dab dec      rax
0x140036dae jmp      0x140036dbd ; 
0x140036db0 movzx    ecx, word ptr [rcx + rdx*2]
0x140036db4 mov      word ptr [rsp + rax + 0x2e], cx
0x140036db9 add      rax, -2
0x140036dbd mov      ebx, 0x14
0x140036dc2 sub      rbx, rax
0x140036dc5 lea      rdx, [rsp + rax]
0x140036dc9 add      rdx, 0x30
0x140036dcd mov      rax, qword ptr [rdi]
0x140036dd0 mov      r14, qword ptr [rdi + 0x10]
0x140036dd4 sub      rax, r14
0x140036dd7 cmp      rax, rbx
0x140036dda jb       0x1400372d7
0x140036de0 mov      rcx, qword ptr [rdi + 8]
0x140036de4 add      rcx, r14
0x140036de7 mov      r8, rbx
0x140036dea call     0x142923c00 ; memcpy
0x140036def add      r14, rbx
0x140036df2 mov      qword ptr [rdi + 0x10], r14
0x140036df6 jmp      0x1400370bd ; 
0x140036dfb lea      r8, [rip + 0x28f381f] ; @0x14292a621 'FreeformCircleRectangle'
0x140036e02 mov      r9d, 8
0x140036e08 jmp      0x1400370a9 ; 
0x140036e0d lea      r8, [rip + 0x28f381b] ; @0x14292a62f 'Rectangle'
0x140036e14 mov      r9d, 9
0x140036e1a jmp      0x1400370a9 ; 
0x140036e1f mov      rcx, qword ptr [rsi]
0x140036e22 lea      rdx, [rip + 0x28f35b7] ; @0x14292a3e0 '{:System built with a different world than the one it was added to.'
0x140036e29 lea      r9, [rip + 0x28f39c8]
0x140036e30 mov      r8d, 1
0x140036e36 call     0x1402b5870 ; alloc::vec::Vec$LT$T$C$A$GT$::extend_from_slice::ha670c95bd7cb2770
0x140036e3b lea      r8, [rip + 0x28f3761] ; @0x14292a5a3 'StairPointTerrainAllPathBulldozeWaterGardenTreeEditingCobblestoneEraserDecoratorColorReplayLoadGameF'
0x140036e42 mov      r9d, 0xa
0x140036e48 jmp      0x140036e9b ; 
0x140036e4a mov      rcx, qword ptr [rsi]
0x140036e4d lea      rdx, [rip + 0x28f358c] ; @0x14292a3e0 '{:System built with a different world than the one it was added to.'
0x140036e54 lea      r9, [rip + 0x28f399d]
0x140036e5b mov      r8d, 1
0x140036e61 call     0x1402b5870 ; alloc::vec::Vec$LT$T$C$A$GT$::extend_from_slice::ha670c95bd7cb2770
0x140036e66 lea      r8, [rip + 0x28f371b] ; @0x14292a588 'RoofDecoClutterdecorator_idStairPointTerrainAllPathBulldozeWaterGardenTreeEditingCobblestoneEraserDe'
0x140036e6d jmp      0x140036f81 ; 
0x140036e72 mov      rcx, qword ptr [rsi]
0x140036e75 lea      rdx, [rip + 0x28f3564] ; @0x14292a3e0 '{:System built with a different world than the one it was added to.'
0x140036e7c lea      r9, [rip + 0x28f3975]
0x140036e83 mov      r8d, 1
0x140036e89 call     0x1402b5870 ; alloc::vec::Vec$LT$T$C$A$GT$::extend_from_slice::ha670c95bd7cb2770
0x140036e8e lea      r8, [rip + 0x28f36f7] ; @0x14292a58c 'DecoClutterdecorator_idStairPointTerrainAllPathBulldozeWaterGardenTreeEditingCobblestoneEraserDecora'
0x140036e95 mov      r9d, 0xb
0x140036e9b mov      rcx, rsi
0x140036e9e mov      rdx, rdi
0x140036ea1 call     0x1404b6100 ; serde_json::ser::format_escaped_str::hc891560d12372fd0
0x140036ea6 test     rax, rax
0x140036ea9 jne      0x1400370e7
0x140036eaf mov      rcx, qword ptr [rsi]
0x140036eb2 lea      rdx, [rip + 0x28f3528] ; @0x14292a3e1 ':System built with a different world than the one it was added to.'
0x140036eb9 lea      rdi, [rip + 0x28f3938]
0x140036ec0 mov      r8d, 1
0x140036ec6 mov      r9, rdi
0x140036ec9 call     0x1402b5870 ; alloc::vec::Vec$LT$T$C$A$GT$::extend_from_slice::ha670c95bd7cb2770
0x140036ece mov      rcx, qword ptr [rsi]
0x140036ed1 lea      rdx, [rip + 0x28f3508] ; @0x14292a3e0 '{:System built with a different world than the one it was added to.'
0x140036ed8 mov      r8d, 1
0x140036ede mov      r9, rdi
0x140036ee1 call     0x1402b5870 ; alloc::vec::Vec$LT$T$C$A$GT$::extend_from_slice::ha670c95bd7cb2770
0x140036ee6 mov      r9, r14
0x140036ee9 add      r9, 4
0x140036eed mov      qword ptr [rsp + 0x30], rsi
0x140036ef2 mov      byte ptr [rsp + 0x38], 1
0x140036ef7 lea      rdx, [rip + 0x28f3699] ; @0x14292a597 'decorator_idStairPointTerrainAllPathBulldozeWaterGardenTreeEditingCobblestoneEraserDecoratorColorRep'
0x140036efe lea      rcx, [rsp + 0x30]
0x140036f03 mov      r8d, 0xc
0x140036f09 call     0x1404baf70 ; serde::ser::SerializeMap::serialize_entry::h3f9bcfb11b341a8d
0x140036f0e test     rax, rax
0x140036f11 jne      0x1400370da
0x140036f17 mov      rax, qword ptr [rsp + 0x30]
0x140036f1c mov      rdi, qword ptr [rax]
0x140036f1f cmp      byte ptr [rsp + 0x38], 0
0x140036f24 je       0x140036f42
0x140036f26 lea      rdx, [rip + 0x28f34b2] ; @0x14292a3df '}{:System built with a different world than the one it was added to.'
0x140036f2d lea      r9, [rip + 0x28f38c4]
0x140036f34 mov      r8d, 1
0x140036f3a mov      rcx, rdi
0x140036f3d call     0x1402b5870 ; alloc::vec::Vec$LT$T$C$A$GT$::extend_from_slice::ha670c95bd7cb2770
0x140036f42 lea      rdx, [rip + 0x28f3496] ; @0x14292a3df '}{:System built with a different world than the one it was added to.'
0x140036f49 lea      r9, [rip + 0x28f38a8]
0x140036f50 mov      r8d, 1
0x140036f56 mov      rcx, rdi
0x140036f59 jmp      0x140037056 ; 
0x140036f5e mov      rcx, qword ptr [rsi]
0x140036f61 lea      rdx, [rip + 0x28f3478] ; @0x14292a3e0 '{:System built with a different world than the one it was added to.'
0x140036f68 lea      r9, [rip + 0x28f3889]
0x140036f6f mov      r8d, 1
0x140036f75 call     0x1402b5870 ; alloc::vec::Vec$LT$T$C$A$GT$::extend_from_slice::ha670c95bd7cb2770
0x140036f7a lea      r8, [rip + 0x28f3603] ; @0x14292a584 'WallRoofDecoClutterdecorator_idStairPointTerrainAllPathBulldozeWaterGardenTreeEditingCobblestoneEras'
0x140036f81 mov      r9d, 4
0x140036f87 mov      rcx, rsi
0x140036f8a mov      rdx, rdi
0x140036f8d call     0x1404b6100 ; serde_json::ser::format_escaped_str::hc891560d12372fd0
0x140036f92 test     rax, rax
0x140036f95 jne      0x140036d51
0x140036f9b mov      rdi, r14
0x140036f9e add      rdi, 8
0x140036fa2 mov      rcx, qword ptr [rsi]
0x140036fa5 lea      rdx, [rip + 0x28f3435] ; @0x14292a3e1 ':System built with a different world than the one it was added to.'
0x140036fac lea      r9, [rip + 0x28f3845]
0x140036fb3 mov      r8d, 1
0x140036fb9 call     0x1402b5870 ; alloc::vec::Vec$LT$T$C$A$GT$::extend_from_slice::ha670c95bd7cb2770
0x140036fbe mov      rcx, rdi
0x140036fc1 mov      rdx, rsi
0x140036fc4 call     0x140080160 ; country_core::resources::walls::_::_$LT$impl$u20$serde..ser..Serialize$u20$for$u20$country_core..resources..walls..WallId$GT$::serialize::h0827e0a799466a17 / country_core::resources::time_of_day::_::_$LT$impl$u20$serde..ser..Serialize$u20$for$u20$country_core..resources..time_of_day..TempUiId$GT$::serialize::h0b77d994db303a86
0x140036fc9 jmp      0x140037036 ; 
0x140036fcb mov      rcx, qword ptr [rsi]
0x140036fce lea      rdx, [rip + 0x28f340b] ; @0x14292a3e0 '{:System built with a different world than the one it was added to.'
0x140036fd5 lea      r9, [rip + 0x28f381c]
0x140036fdc mov      r8d, 1
0x140036fe2 call     0x1402b5870 ; alloc::vec::Vec$LT$T$C$A$GT$::extend_from_slice::ha670c95bd7cb2770
0x140036fe7 lea      r8, [rip + 0x28f35bf] ; @0x14292a5ad 'TerrainAllPathBulldozeWaterGardenTreeEditingCobblestoneEraserDecoratorColorReplayLoadGameFencesPhoto'
0x140036fee mov      r9d, 7
0x140036ff4 mov      rcx, rsi
0x140036ff7 mov      rdx, rdi
0x140036ffa call     0x1404b6100 ; serde_json::ser::format_escaped_str::hc891560d12372fd0
0x140036fff test     rax, rax
0x140037002 jne      0x140036d51
0x140037008 mov      rdi, r14
0x14003700b add      rdi, 4
0x14003700f mov      rcx, qword ptr [rsi]
0x140037012 lea      rdx, [rip + 0x28f33c8] ; @0x14292a3e1 ':System built with a different world than the one it was added to.'
0x140037019 lea      r9, [rip + 0x28f37d8]
0x140037020 mov      r8d, 1
0x140037026 call     0x1402b5870 ; alloc::vec::Vec$LT$T$C$A$GT$::extend_from_slice::ha670c95bd7cb2770
0x14003702b mov      rcx, rdi
0x14003702e mov      rdx, rsi
0x140037031 call     0x1400e7480 ; country_core::resources::terrain_editing::_::_$LT$impl$u20$serde..ser..Serialize$u20$for$u20$country_core..resources..terrain_editing..TerrainStrokeId$GT$::serialize::hebb55cb7ed2caf9a / country_core::resources::color_id::_::_$LT$impl$u20$serde..ser..Serialize$u20$for$u20$country_core..resources..color_id..ColorId$GT$::serialize::h72a997dbece6cfd2
0x140037036 test     rax, rax
0x140037039 jne      0x1400370da
0x14003703f mov      rcx, qword ptr [rsi]
0x140037042 lea      rdx, [rip + 0x28f3396] ; @0x14292a3df '}{:System built with a different world than the one it was added to.'
0x140037049 lea      r9, [rip + 0x28f37a8]
0x140037050 mov      r8d, 1
0x140037056 call     0x1402b5870 ; alloc::vec::Vec$LT$T$C$A$GT$::extend_from_slice::ha670c95bd7cb2770
0x14003705b jmp      0x1400370bd ; 
0x14003705d lea      r8, [rip + 0x28f355f] ; @0x14292a5c3 'WaterGardenTreeEditingCobblestoneEraserDecoratorColorReplayLoadGameFencesPhotomodeCloneClutterFreefo'
0x140037064 mov      r9d, 5
0x14003706a jmp      0x1400370a9 ; 
0x14003706c lea      r8, [rip + 0x28f3544] ; @0x14292a5b7 'PathBulldozeWaterGardenTreeEditingCobblestoneEraserDecoratorColorReplayLoadGameFencesPhotomodeCloneC'
0x140037073 jmp      0x1400370a3 ; 
0x140037075 lea      r8, [rip + 0x28f353f] ; @0x14292a5bb 'BulldozeWaterGardenTreeEditingCobblestoneEraserDecoratorColorReplayLoadGameFencesPhotomodeCloneClutt'
0x14003707c mov      r9d, 8
0x140037082 jmp      0x1400370a9 ; 
0x140037084 lea      r8, [rip + 0x28f34f9] ; @0x14292a584 'WallRoofDecoClutterdecorator_idStairPointTerrainAllPathBulldozeWaterGardenTreeEditingCobblestoneEras'
0x14003708b jmp      0x1400370a3 ; 
0x14003708d lea      r8, [rip + 0x28f3534] ; @0x14292a5c8 'GardenTreeEditingCobblestoneEraserDecoratorColorReplayLoadGameFencesPhotomodeCloneClutterFreeformCir'
0x140037094 mov      r9d, 6
0x14003709a jmp      0x1400370a9 ; 
0x14003709c lea      r8, [rip + 0x28f352b] ; @0x14292a5ce 'TreeEditingCobblestoneEraserDecoratorColorReplayLoadGameFencesPhotomodeCloneClutterFreeformCircleRec'
0x1400370a3 mov      r9d, 4
0x1400370a9 mov      rcx, rsi
0x1400370ac mov      rdx, rdi
0x1400370af call     0x1404b6100 ; serde_json::ser::format_escaped_str::hc891560d12372fd0
0x1400370b4 test     rax, rax
0x1400370b7 jne      0x140036d51
0x1400370bd mov      rsi, qword ptr [rsi]
0x1400370c0 mov      rdx, qword ptr [rsi + 0x10]
0x1400370c4 cmp      qword ptr [rsi], rdx
0x1400370c7 je       0x1400370f1
0x1400370c9 mov      rax, qword ptr [rsi + 8]
0x1400370cd mov      byte ptr [rax + rdx], 0x7d
0x1400370d1 inc      rdx
0x1400370d4 mov      qword ptr [rsi + 0x10], rdx
0x1400370d8 xor      eax, eax
0x1400370da add      rsp, 0x58
0x1400370de pop      rbx
0x1400370df pop      rbp
0x1400370e0 pop      rdi
0x1400370e1 pop      rsi
0x1400370e2 pop      r14
0x1400370e4 pop      r15
0x1400370e6 ret      
0x1400370e7 mov      rcx, rax
0x1400370ea call     0x1428c98a0 ; serde_json::error::Error::io::hfcf27584d0d6aa9f
0x1400370ef jmp      0x1400370da ; 
0x1400370f1 mov      qword ptr [rsp + 0x20], 1
0x1400370fa mov      r8d, 1
0x140037100 mov      r9d, 1
0x140037106 mov      rcx, rsi
0x140037109 call     0x1428a29e0 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::h27486e0671e8b840
0x14003710e mov      rdx, qword ptr [rsi + 0x10]
0x140037112 jmp      0x1400370c9 ; 
0x140037114 mov      qword ptr [rsp + 0x20], 1
0x14003711d mov      r8d, 1
0x140037123 mov      r9d, 1
0x140037129 mov      rcx, rbx
0x14003712c call     0x1428a29e0 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::h27486e0671e8b840
0x140037131 mov      rdx, qword ptr [rbx + 0x10]
0x140037135 jmp      0x14003699a ; 
0x14003713a mov      qword ptr [rsp + 0x20], 1
0x140037143 mov      r8d, 1
0x140037149 mov      r9d, 1
0x14003714f mov      rcx, rdi
0x140037152 mov      rbx, rdx
0x140037155 mov      rdx, rax
0x140037158 call     0x1428a29e0 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::h27486e0671e8b840
0x14003715d mov      rdx, rbx
0x140037160 mov      rax, qword ptr [rdi + 0x10]
0x140037164 jmp      0x140036a31 ; 
0x140037169 mov      qword ptr [rsp + 0x20], 1
0x140037172 mov      r8d, 1
0x140037178 mov      r9d, 1
0x14003717e mov      rcx, rbx
0x140037181 mov      r14, rdx
0x140037184 mov      rdx, rax
0x140037187 call     0x1428a29e0 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::h27486e0671e8b840
0x14003718c mov      rdx, r14
0x14003718f mov      rax, qword ptr [rbx + 0x10]
0x140037193 jmp      0x140036afd ; 
0x140037198 mov      r14, rcx
0x14003719b mov      qword ptr [rsp + 0x20], 1
0x1400371a4 mov      r8d, 1
0x1400371aa mov      r9d, 1
0x1400371b0 mov      rcx, rbx
0x1400371b3 call     0x1428a29e0 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::h27486e0671e8b840
0x1400371b8 mov      rdx, qword ptr [rbx + 0x10]
0x1400371bc jmp      0x1400368ee ; 
0x1400371c1 mov      qword ptr [rsp + 0x20], 1
0x1400371ca mov      r8d, 1
0x1400371d0 mov      r9d, 1
0x1400371d6 mov      rcx, rbx
0x1400371d9 call     0x1428a29e0 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::h27486e0671e8b840
0x1400371de mov      rdx, qword ptr [rbx + 0x10]
0x1400371e2 jmp      0x140036c08 ; 
0x1400371e7 mov      rbx, rcx
0x1400371ea mov      qword ptr [rsp + 0x20], 1
0x1400371f3 mov      r8d, 1
0x1400371f9 mov      r9d, 1
0x1400371ff mov      rcx, rdi
0x140037202 mov      r14, rdx
0x140037205 mov      rdx, rax
0x140037208 call     0x1428a29e0 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::h27486e0671e8b840
0x14003720d mov      rdx, r14
0x140037210 mov      rax, qword ptr [rdi + 0x10]
0x140037214 jmp      0x140036cb7 ; 
0x140037219 mov      qword ptr [rsp + 0x20], 1
0x140037222 mov      r8d, 1
0x140037228 mov      r9d, 1
0x14003722e mov      rcx, rbx
0x140037231 call     0x1428a29e0 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::h27486e0671e8b840
0x140037236 mov      rdx, qword ptr [rbx + 0x10]
0x14003723a jmp      0x1400369da ; 
0x14003723f mov      qword ptr [rsp + 0x20], 1
0x140037248 mov      r8d, 1
0x14003724e mov      r9d, 1
0x140037254 mov      rcx, rdi
0x140037257 call     0x1428a29e0 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::h27486e0671e8b840
0x14003725c mov      rdx, qword ptr [rdi + 0x10]
0x140037260 jmp      0x140036a6e ; 
0x140037265 mov      qword ptr [rsp + 0x20], 1
0x14003726e mov      r8d, 1
0x140037274 mov      r9d, 1
0x14003727a mov      rcx, rdi
0x14003727d call     0x1428a29e0 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::h27486e0671e8b840
0x140037282 mov      rdx, qword ptr [rdi + 0x10]
0x140037286 jmp      0x140036a8d ; 
0x14003728b mov      qword ptr [rsp + 0x20], 1
0x140037294 mov      r8d, 1
0x14003729a mov      r9d, 1
0x1400372a0 mov      rcx, rdi
0x1400372a3 call     0x1428a29e0 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::h27486e0671e8b840
0x1400372a8 mov      rdx, qword ptr [rdi + 0x10]
0x1400372ac jmp      0x140036aa5 ; 
0x1400372b1 mov      qword ptr [rsp + 0x20], 1
0x1400372ba mov      r8d, 1
0x1400372c0 mov      r9d, 1
0x1400372c6 mov      rcx, rbx
0x1400372c9 call     0x1428a29e0 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::h27486e0671e8b840
0x1400372ce mov      rdx, qword ptr [rbx + 0x10]
0x1400372d2 jmp      0x140036b3a ; 
0x1400372d7 mov      qword ptr [rsp + 0x20], 1
0x1400372e0 mov      r9d, 1
0x1400372e6 mov      rcx, rdi
0x1400372e9 mov      r15, rdx
0x1400372ec mov      rdx, r14
0x1400372ef mov      r8, rbx
0x1400372f2 call     0x1428a29e0 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::h27486e0671e8b840
0x1400372f7 mov      rdx, r15
0x1400372fa mov      r14, qword ptr [rdi + 0x10]
0x1400372fe jmp      0x140036de0 ; 
0x140037303 mov      qword ptr [rsp + 0x20], 1
0x14003730c mov      r8d, 1
0x140037312 mov      r9d, 1
0x140037318 mov      rcx, rbx
0x14003731b call     0x1428a29e0 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::h27486e0671e8b840
0x140037320 mov      rdx, qword ptr [rbx + 0x10]
0x140037324 jmp      0x14003692e ; 
0x140037329 mov      qword ptr [rsp + 0x20], 1
0x140037332 mov      r8d, 1
0x140037338 mov      r9d, 1
0x14003733e mov      rcx, rbx
0x140037341 call     0x1428a29e0 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::h27486e0671e8b840
0x140037346 mov      rdx, qword ptr [rbx + 0x10]
0x14003734a jmp      0x140036c48 ; 
0x14003734f mov      qword ptr [rsp + 0x20], 1
0x140037358 mov      r8d, 1
0x14003735e mov      r9d, 1
0x140037364 mov      rbx, rcx
0x140037367 mov      rcx, rdi
0x14003736a call     0x1428a29e0 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::h27486e0671e8b840
0x14003736f mov      rcx, rbx
0x140037372 mov      rdx, qword ptr [rdi + 0x10]
0x140037376 jmp      0x140036cf7 ; 