0x140aab0c0 push r14
0x140aab0c2 push rsi
0x140aab0c3 push rdi
0x140aab0c4 push rbx
0x140aab0c5 sub rsp, 0x38
0x140aab0c9 mov rdi, r9
0x140aab0cc mov rax, qword ptr [rcx]
0x140aab0cf mov rsi, qword ptr [rax]
0x140aab0d2 add rsi, 0x10
0x140aab0d6 mov rbx, qword ptr [r8]
0x140aab0d9 mov r14, qword ptr [rdx]
0x140aab0dc mov qword ptr [rsp + 0x30], r14
0x140aab0e1 mov qword ptr [rsp + 0x28], rbx
0x140aab0e6 mov qword ptr [rsp + 0x20], 0xd
0x140aab0ef lea rdx, [rip + 0x203248f] ; '/clearing/meadow_flower.raster.hlslmeadow_flower/vertex_color.raster.hlslvertex_'
0x140aab0f6 lea r9, [rip + 0x20324ab] ; 'meadow_flower/vertex_color.raster.hlslvertex_color_shader/dashed_line.raster.hls'
0x140aab0fd mov r8d, 0x23
0x140aab103 mov rcx, rsi
0x140aab106 call 0x140b3fe40
0x140aab10b mov qword ptr [rsp + 0x30], r14
0x140aab110 mov qword ptr [rsp + 0x28], rbx
0x140aab115 mov qword ptr [rsp + 0x20], 0x13
0x140aab11e lea rdx, [rip + 0x2032490] ; '/vertex_color.raster.hlslvertex_color_shader/dashed_line.raster.hlsldashed_line/'
0x140aab125 lea r9, [rip + 0x20324a2] ; 'vertex_color_shader/dashed_line.raster.hlsldashed_line/timer_circle.raster.hlslt'
0x140aab12c mov r8d, 0x19
0x140aab132 mov rcx, rsi
0x140aab135 call 0x140b3fe40
0x140aab13a mov qword ptr [rsp + 0x30], r14
0x140aab13f mov qword ptr [rsp + 0x28], rbx
0x140aab144 mov qword ptr [rsp + 0x20], 0xb
0x140aab14d lea rdx, [rip + 0x203248d] ; '/dashed_line.raster.hlsldashed_line/timer_circle.raster.hlsltimer_circle/final_p'
0x140aab154 lea r9, [rip + 0x203249e] ; 'dashed_line/timer_circle.raster.hlsltimer_circle/final_post.raster.hlslfinal_pos'
0x140aab15b mov r8d, 0x18
0x140aab161 mov rcx, rsi
0x140aab164 call 0x140b3fe40
0x140aab169 mov qword ptr [rsp + 0x30], r14
0x140aab16e mov qword ptr [rsp + 0x28], rbx
0x140aab173 mov qword ptr [rsp + 0x20], 0xc
0x140aab17c lea rdx, [rip + 0x2032481] ; '/timer_circle.raster.hlsltimer_circle/final_post.raster.hlslfinal_post/egui.rast'
0x140aab183 lea r9, [rip + 0x2032493] ; 'timer_circle/final_post.raster.hlslfinal_post/egui.raster.hlslegui/solid_vertex_'
0x140aab18a mov r8d, 0x19
0x140aab190 mov rcx, rsi
0x140aab193 call 0x140b3fe40
0x140aab198 mov qword ptr [rsp + 0x30], r14
0x140aab19d mov qword ptr [rsp + 0x28], rbx
0x140aab1a2 mov qword ptr [rsp + 0x20], 0xa
0x140aab1ab lea rdx, [rip + 0x2032477] ; '/final_post.raster.hlslfinal_post/egui.raster.hlslegui/solid_vertex_color.raster'
0x140aab1b2 lea r9, [rip + 0x2032487] ; 'final_post/egui.raster.hlslegui/solid_vertex_color.raster.hlslsolid_vertex_color'
0x140aab1b9 mov r8d, 0x17
0x140aab1bf mov rcx, rsi
0x140aab1c2 call 0x140b3fe40
0x140aab1c7 mov ecx, dword ptr [rdi + 0x1c]
0x140aab1ca mov rdx, qword ptr [rdi + 0x10]
0x140aab1ce mov dword ptr [rdx], ecx
0x140aab1d0 mov rcx, qword ptr [rdi]
0x140aab1d3 mov qword ptr [rcx], 1
0x140aab1da mov qword ptr [rcx + 8], rax
0x140aab1de mov qword ptr [rsp + 0x30], r14
0x140aab1e3 mov qword ptr [rsp + 0x28], rbx
0x140aab1e8 mov qword ptr [rsp + 0x20], 4
0x140aab1f1 lea rdx, [rip + 0x2032452] ; '/egui.raster.hlslegui/solid_vertex_color.raster.hlslsolid_vertex_color/solid_ver'
0x140aab1f8 lea r9, [rip + 0x203245c] ; 'egui/solid_vertex_color.raster.hlslsolid_vertex_color/solid_vertex_color_albedo_'
0x140aab1ff mov r8d, 0x11
0x140aab205 mov rcx, rsi
0x140aab208 call 0x140b3fe40
0x140aab20d mov qword ptr [rsp + 0x30], r14
0x140aab212 mov qword ptr [rsp + 0x28], rbx
0x140aab217 mov qword ptr [rsp + 0x20], 0x12
0x140aab220 lea rdx, [rip + 0x2032438] ; '/solid_vertex_color.raster.hlslsolid_vertex_color/solid_vertex_color_albedo_map.'
0x140aab227 lea r9, [rip + 0x2032450] ; 'solid_vertex_color/solid_vertex_color_albedo_map.raster.hlslsolid_vertex_color_a'
0x140aab22e mov r8d, 0x1f
0x140aab234 mov rcx, rsi
0x140aab237 call 0x140b3fe40
0x140aab23c mov qword ptr [rsp + 0x30], r14
0x140aab241 mov qword ptr [rsp + 0x28], rbx
0x140aab246 mov qword ptr [rsp + 0x20], 0x1d
0x140aab24f lea rdx, [rip + 0x203243a] ; '/solid_vertex_color_albedo_map.raster.hlslsolid_vertex_color_albedo_map/solid_ve'
0x140aab256 lea r9, [rip + 0x203245d] ; 'solid_vertex_color_albedo_map/solid_vertex_color_wooden_gate.raster.hlslsolid_ve'
0x140aab25d mov r8d, 0x2a
0x140aab263 mov rcx, rsi
0x140aab266 call 0x140b3fe40
0x140aab26b mov qword ptr [rsp + 0x30], r14
0x140aab270 mov qword ptr [rsp + 0x28], rbx
0x140aab275 mov qword ptr [rsp + 0x20], 0x1e
0x140aab27e lea rdx, [rip + 0x2032452] ; '/solid_vertex_color_wooden_gate.raster.hlslsolid_vertex_color_wooden_gate/flag_r'
0x140aab285 lea r9, [rip + 0x2032476] ; 'solid_vertex_color_wooden_gate/flag_roof_cloth.raster.hlslflag_roof_cloth/flag_w'
0x140aab28c mov r8d, 0x2b
0x140aab292 mov rcx, rsi
0x140aab295 call 0x140b3fe40
0x140aab29a mov qword ptr [rsp + 0x30], r14
0x140aab29f mov qword ptr [rsp + 0x28], rbx
0x140aab2a4 mov qword ptr [rsp + 0x20], 0xf
0x140aab2ad lea rdx, [rip + 0x203246c] ; '/flag_roof_cloth.raster.hlslflag_roof_cloth/flag_wall_cloth.raster.hlslflag_wall'
0x140aab2b4 lea r9, [rip + 0x2032481] ; 'flag_roof_cloth/flag_wall_cloth.raster.hlslflag_wall_cloth/flag_small.raster.hls'
0x140aab2bb mov r8d, 0x1c
0x140aab2c1 mov rcx, rsi
0x140aab2c4 call 0x140b3fe40
0x140aab2c9 mov qword ptr [rsp + 0x30], r14
0x140aab2ce mov qword ptr [rsp + 0x28], rbx
0x140aab2d3 mov qword ptr [rsp + 0x20], 0xf
0x140aab2dc lea rdx, [rip + 0x2032468] ; '/flag_wall_cloth.raster.hlslflag_wall_cloth/flag_small.raster.hlslflag_small/sta'
0x140aab2e3 lea r9, [rip + 0x203247d] ; 'flag_wall_cloth/flag_small.raster.hlslflag_small/stairs_foliage_exclusion.raster'
0x140aab2ea mov r8d, 0x1c
0x140aab2f0 mov rcx, rsi
0x140aab2f3 call 0x140b3fe40
0x140aab2f8 mov qword ptr [rsp + 0x30], r14
0x140aab2fd mov qword ptr [rsp + 0x28], rbx
0x140aab302 mov qword ptr [rsp + 0x20], 0xa
0x140aab30b lea rdx, [rip + 0x2032464] ; '/flag_small.raster.hlslflag_small/stairs_foliage_exclusion.raster.hlslstairs_fol'
0x140aab312 lea r9, [rip + 0x2032474] ; 'flag_small/stairs_foliage_exclusion.raster.hlslstairs_foliage_exclusioncrates/co'
0x140aab319 mov r8d, 0x17
0x140aab31f mov rcx, rsi
0x140aab322 call 0x140b3fe40
0x140aab327 mov qword ptr [rsp + 0x30], r14
0x140aab32c mov qword ptr [rsp + 0x28], rbx
0x140aab331 mov qword ptr [rsp + 0x20], 0x18
0x140aab33a lea rdx, [rip + 0x2032456] ; '/stairs_foliage_exclusion.raster.hlslstairs_foliage_exclusioncrates/country-core'
0x140aab341 lea r9, [rip + 0x2032474] ; 'stairs_foliage_exclusioncrates/country-core/src/systems/backend/texture_watch.rs'
0x140aab348 mov r8d, 0x25
0x140aab34e mov rcx, rsi
0x140aab351 call 0x140b3fe40
0x140aab356 nop 
0x140aab357 add rsp, 0x38
0x140aab35b pop rbx
0x140aab35c pop rdi
0x140aab35d pop rsi
0x140aab35e pop r14
0x140aab360 ret 