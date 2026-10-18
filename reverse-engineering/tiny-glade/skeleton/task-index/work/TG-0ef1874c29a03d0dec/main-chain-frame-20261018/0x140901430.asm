0x140901430 push rsi
0x140901431 push rdi
0x140901432 push rbx
0x140901433 sub rsp, 0x40
0x140901437 mov rax, qword ptr [rcx]
0x14090143a mov rsi, qword ptr [rax]
0x14090143d add rsi, 0x10
0x140901441 mov rdi, qword ptr [r8]
0x140901444 mov rbx, qword ptr [rdx]
0x140901447 mov qword ptr [rsp + 0x30], rbx
0x14090144c mov qword ptr [rsp + 0x28], rdi
0x140901451 mov qword ptr [rsp + 0x20], 0x1b
0x14090145a lea rdx, [rip + 0x21a894f] ; '/blit/resolve_to_swapchain.raster.hlslraster_resolve_to_swapchain/blit/resolve_m'
0x140901461 lea r9, [rip + 0x21a896e] ; 'raster_resolve_to_swapchain/blit/resolve_msaa_to_swapchain.raster.hlslraster_res'
0x140901468 mov r8d, 0x26
0x14090146e mov rcx, rsi
0x140901471 call 0x140b3fe40
0x140901476 mov qword ptr [rsp + 0x30], rbx
0x14090147b mov qword ptr [rsp + 0x28], rdi
0x140901480 mov qword ptr [rsp + 0x20], 0x20
0x140901489 lea rdx, [rip + 0x21a8961] ; '/blit/resolve_msaa_to_swapchain.raster.hlslraster_resolve_msaa_to_swapchain/blit'
0x140901490 lea r9, [rip + 0x21a8985] ; 'raster_resolve_msaa_to_swapchain/blit/blit_stretch_decode_srgb.raster.hlslblit_s'
0x140901497 mov r8d, 0x2b
0x14090149d mov rcx, rsi
0x1409014a0 call 0x140b3fe40
0x1409014a5 mov qword ptr [rsp + 0x30], rbx
0x1409014aa mov qword ptr [rsp + 0x28], rdi
0x1409014af mov qword ptr [rsp + 0x20], 0x18
0x1409014b8 lea rdx, [rip + 0x21a897d] ; '/blit/blit_stretch_decode_srgb.raster.hlslblit_stretch_decode_srgbcrates/country'
0x1409014bf lea r9, [rip + 0x21a89a0] ; 'blit_stretch_decode_srgbcrates/country-core/src/systems/render_loop/compute_pass'
0x1409014c6 mov r8d, 0x2a
0x1409014cc mov rcx, rsi
0x1409014cf call 0x140b3fe40
0x1409014d4 nop 
0x1409014d5 add rsp, 0x40
0x1409014d9 pop rbx
0x1409014da pop rdi
0x1409014db pop rsi
0x1409014dc ret 