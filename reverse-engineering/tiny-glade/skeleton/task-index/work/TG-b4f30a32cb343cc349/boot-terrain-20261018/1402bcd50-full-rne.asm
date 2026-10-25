1402bcd50: push rbp
1402bcd51: push r15
1402bcd53: push r14
1402bcd55: push r13
1402bcd57: push r12
1402bcd59: push rsi
1402bcd5a: push rdi
1402bcd5b: push rbx
1402bcd5c: sub rsp, 0x88
1402bcd63: lea rbp, [rsp + 0x80]
1402bcd6b: movaps xmmword ptr [rbp - 0x10], xmm7
1402bcd6f: movaps xmmword ptr [rbp - 0x20], xmm6
1402bcd73: mov qword ptr [rbp - 0x28], 0xfffffffffffffffe
1402bcd7b: mov rdi, rdx
1402bcd7e: mov r9, qword ptr [rdx]
1402bcd81: test r9, r9
1402bcd84: je 0x1402bcd96
1402bcd86: mov rax, qword ptr [rdi + 8]
1402bcd8a: xor esi, esi
1402bcd8c: cmp r9, rax
1402bcd8f: setne al
1402bcd92: jne 0x1402bcd9c
1402bcd94: jmp 0x1402bcda7 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::sse2::vec4::Vec4> >,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$26<false> > >
1402bcd96: xor esi, esi
1402bcd98: xor eax, eax
1402bcd9a: jmp 0x1402bcdaa ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::sse2::vec4::Vec4> >,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$26<false> > >
1402bcd9c: movaps xmm6, xmmword ptr [r9]
1402bcda0: add r9, 0x10
1402bcda4: mov rsi, r9
1402bcda7: mov qword ptr [rdi], rsi
1402bcdaa: test al, 1
1402bcdac: jne 0x1402bce9e
1402bcdb2: mov rax, qword ptr [rdi + 0x20]
1402bcdb6: test rax, rax
1402bcdb9: je 0x1402bce6f
1402bcdbf: mov r10, qword ptr [rdi + 0x40]
1402bcdc3: test r10, r10
1402bcdc6: je 0x1402bce6f
1402bcdcc: movzx edx, word ptr [rdi + 0x38]
1402bcdd0: test dx, dx
1402bcdd3: je 0x1402bcdde
1402bcdd5: lea r11d, [rdx - 1]
1402bcdd9: and r11d, edx
1402bcddc: jmp 0x1402bce24 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::sse2::vec4::Vec4> >,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$26<false> > >
1402bcdde: mov r9, qword ptr [rdi + 0x28]
1402bcde2: nop word ptr cs:[rax + rax]
1402bcdf0: movdqa xmm0, xmmword ptr [r9]
1402bcdf5: pmovmskb edx, xmm0
1402bcdf9: add rax, -0x1100
1402bcdff: add r9, 0x10
1402bce03: cmp edx, 0xffff
1402bce09: je 0x1402bcdf0
1402bce0b: mov qword ptr [rdi + 0x28], r9
1402bce0f: mov qword ptr [rdi + 0x20], rax
1402bce13: mov r9d, 0xfffffffe
1402bce19: sub r9d, edx
1402bce1c: not edx
1402bce1e: mov r11d, edx
1402bce21: and r11d, r9d
1402bce24: mov word ptr [rdi + 0x38], r11w
1402bce29: dec r10
1402bce2c: mov qword ptr [rdi + 0x40], r10
1402bce30: tzcnt edx, edx
1402bce34: neg rdx
1402bce37: imul rdx, rdx, 0x110
1402bce3e: mov r9, qword ptr [rax + rdx - 0xe8]
1402bce46: mov rax, qword ptr [rax + rdx - 0xe0]
1402bce4e: shl rax, 4
1402bce52: add rax, r9
1402bce55: mov qword ptr [rdi], r9
1402bce58: mov qword ptr [rdi + 8], rax
1402bce5c: xor esi, esi
1402bce5e: cmp r9, rax
1402bce61: setne al
1402bce64: jne 0x1402bcd9c
1402bce6a: jmp 0x1402bcda7 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::sse2::vec4::Vec4> >,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$26<false> > >
1402bce6f: mov rax, qword ptr [rdi + 0x10]
1402bce73: test rax, rax
1402bce76: je 0x1402bd133
1402bce7c: mov r11, qword ptr [rdi + 0x18]
1402bce80: cmp rax, r11
1402bce83: je 0x1402bd12b
1402bce89: mov qword ptr [rbp - 0x30], r8
1402bce8d: mov qword ptr [rbp - 0x50], rcx
1402bce91: lea r10, [rax + 0x10]
1402bce95: mov qword ptr [rdi + 0x10], r10
1402bce99: movaps xmm6, xmmword ptr [rax]
1402bce9c: jmp 0x1402bceae ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::sse2::vec4::Vec4> >,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$26<false> > >
1402bce9e: mov qword ptr [rbp - 0x30], r8
1402bcea2: mov qword ptr [rbp - 0x50], rcx
1402bcea6: mov r10, qword ptr [rdi + 0x10]
1402bceaa: mov r11, qword ptr [rdi + 0x18]
1402bceae: mov r15, qword ptr [rdi + 8]
1402bceb2: mov rax, r15
1402bceb5: sub rax, rsi
1402bceb8: shr rax, 4
1402bcebc: test rsi, rsi
1402bcebf: cmove rax, rsi
1402bcec3: mov rcx, r11
1402bcec6: sub rcx, r10
1402bcec9: shr rcx, 4
1402bcecd: test r10, r10
1402bced0: cmove rcx, r10
1402bced4: add rcx, rax
1402bced7: cmp rcx, 4
1402bcedb: mov r12d, 3
1402bcee1: cmovae r12, rcx
1402bcee5: inc r12
1402bcee8: mov r14, r12
1402bceeb: shl r14, 4
1402bceef: xor ebx, ebx
1402bcef1: movabs rax, 0xffffffffffffffe
1402bcefb: cmp rcx, rax
1402bcefe: ja 0x1402bd169
1402bcf04: movabs rax, 0x7ffffffffffffff0
1402bcf0e: cmp r14, rax
1402bcf11: ja 0x1402bd169
1402bcf17: test r14, r14
1402bcf1a: mov qword ptr [rbp - 0x58], r11
1402bcf1e: je 0x1402bcf4e
1402bcf20: movzx eax, byte ptr [rip + 0x3377ce2]
1402bcf27: mov ebx, 0x10
1402bcf2c: mov edx, 0x10
1402bcf31: mov rcx, r14
1402bcf34: mov r13, r10
1402bcf37: call 0x140613c10 ; ?
1402bcf3c: mov r11, qword ptr [rbp - 0x58]
1402bcf40: test rax, rax
1402bcf43: je 0x1402bd169
1402bcf49: mov r10, r13
1402bcf4c: jmp 0x1402bcf56 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::sse2::vec4::Vec4> >,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$26<false> > >
1402bcf4e: mov eax, 0x10
1402bcf53: xor r12d, r12d
1402bcf56: movaps xmmword ptr [rax], xmm6
1402bcf59: mov qword ptr [rbp - 0x48], r12
1402bcf5d: mov qword ptr [rbp - 0x40], rax
1402bcf61: mov qword ptr [rbp - 0x38], 1
1402bcf69: mov r14, qword ptr [rdi + 0x20]
1402bcf6d: mov rbx, qword ptr [rdi + 0x28]
1402bcf71: movzx r12d, word ptr [rdi + 0x38]
1402bcf76: mov r9, qword ptr [rdi + 0x40]
1402bcf7a: mov edi, 1
1402bcf7f: jmp 0x1402bcfa2 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::sse2::vec4::Vec4> >,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$26<false> > >
1402bcf81: nop word ptr cs:[rax + rax]
1402bcf90: mov rcx, rdi
1402bcf93: shl rcx, 4
1402bcf97: movaps xmmword ptr [rax + rcx], xmm7
1402bcf9b: inc rdi
1402bcf9e: mov qword ptr [rbp - 0x38], rdi
1402bcfa2: test rsi, rsi
1402bcfa5: jne 0x1402bd059
1402bcfab: xor esi, esi
1402bcfad: xor ecx, ecx
1402bcfaf: test cl, 1
1402bcfb2: je 0x1402bcfd0
1402bcfb4: movaps xmm7, xmm6
1402bcfb7: cmp rdi, qword ptr [rbp - 0x48]
1402bcfbb: jne 0x1402bcf90
1402bcfbd: jmp 0x1402bd09e ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::sse2::vec4::Vec4> >,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$26<false> > >
1402bcfc2: nop word ptr cs:[rax + rax]
1402bcfd0: test r14, r14
1402bcfd3: je 0x1402bd07a
1402bcfd9: test r9, r9
1402bcfdc: je 0x1402bd077
1402bcfe2: test r12w, r12w
1402bcfe6: je 0x1402bd000
1402bcfe8: lea ecx, [r12 - 1]
1402bcfed: and ecx, r12d
1402bcff0: jmp 0x1402bd02d ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::sse2::vec4::Vec4> >,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$26<false> > >
1402bcff2: nop word ptr cs:[rax + rax]
1402bd000: movdqa xmm0, xmmword ptr [rbx]
1402bd004: pmovmskb r12d, xmm0
1402bd009: add r14, -0x1100
1402bd010: add rbx, 0x10
1402bd014: cmp r12d, 0xffff
1402bd01b: je 0x1402bd000
1402bd01d: mov edx, 0xfffffffe
1402bd022: sub edx, r12d
1402bd025: not r12d
1402bd028: mov ecx, r12d
1402bd02b: and ecx, edx
1402bd02d: tzcnt edx, r12d
1402bd032: neg rdx
1402bd035: imul rdx, rdx, 0x110
1402bd03c: dec r9
1402bd03f: mov rsi, qword ptr [r14 + rdx - 0xe8]
1402bd047: mov r15, qword ptr [r14 + rdx - 0xe0]
1402bd04f: shl r15, 4
1402bd053: add r15, rsi
1402bd056: mov r12d, ecx
1402bd059: cmp rsi, r15
1402bd05c: setne cl
1402bd05f: je 0x1402bd070
1402bd061: movaps xmm6, xmmword ptr [rsi]
1402bd064: add rsi, 0x10
1402bd068: jmp 0x1402bcfaf ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::sse2::vec4::Vec4> >,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$26<false> > >
1402bd06d: nop dword ptr [rax]
1402bd070: xor esi, esi
1402bd072: jmp 0x1402bcfaf ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::sse2::vec4::Vec4> >,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$26<false> > >
1402bd077: xor r9d, r9d
1402bd07a: test r10, r10
1402bd07d: je 0x1402bd114
1402bd083: cmp r10, r11
1402bd086: je 0x1402bd114
1402bd08c: movaps xmm7, xmmword ptr [r10]
1402bd090: add r10, 0x10
1402bd094: cmp rdi, qword ptr [rbp - 0x48]
1402bd098: jne 0x1402bcf90
1402bd09e: test rsi, rsi
1402bd0a1: je 0x1402bd0c8
1402bd0a3: mov rax, r15
1402bd0a6: sub rax, rsi
1402bd0a9: shr rax, 4
1402bd0ad: mov qword ptr [rbp - 0x30], r15
1402bd0b1: test r10, r10
1402bd0b4: je 0x1402bd0d3
1402bd0b6: mov r15, r9
1402bd0b9: mov rcx, r11
1402bd0bc: mov r13, r10
1402bd0bf: sub rcx, r10
1402bd0c2: shr rcx, 4
1402bd0c6: jmp 0x1402bd0db ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::sse2::vec4::Vec4> >,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$26<false> > >
1402bd0c8: xor eax, eax
1402bd0ca: mov qword ptr [rbp - 0x30], r15
1402bd0ce: test r10, r10
1402bd0d1: jne 0x1402bd0b6
1402bd0d3: mov r13, r10
1402bd0d6: mov r15, r9
1402bd0d9: xor ecx, ecx
1402bd0db: lea r8, [rcx + rax]
1402bd0df: inc r8
1402bd0e2: mov qword ptr [rsp + 0x20], 0x10
1402bd0eb: mov r9d, 0x10
1402bd0f1: lea rcx, [rbp - 0x48]
1402bd0f5: mov rdx, rdi
1402bd0f8: call 0x1428a29e0 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1402bd0fd: mov rax, qword ptr [rbp - 0x40]
1402bd101: mov r11, qword ptr [rbp - 0x58]
1402bd105: mov r9, r15
1402bd108: mov r15, qword ptr [rbp - 0x30]
1402bd10c: mov r10, r13
1402bd10f: jmp 0x1402bcf90 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::sse2::vec4::Vec4> >,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$26<false> > >
1402bd114: mov rax, qword ptr [rbp - 0x38]
1402bd118: mov rcx, qword ptr [rbp - 0x50]
1402bd11c: mov qword ptr [rcx + 0x10], rax
1402bd120: movdqu xmm0, xmmword ptr [rbp - 0x48]
1402bd125: movdqu xmmword ptr [rcx], xmm0
1402bd129: jmp 0x1402bd14a ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::sse2::vec4::Vec4> >,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$26<false> > >
1402bd12b: mov qword ptr [rdi + 0x10], 0
1402bd133: mov qword ptr [rcx], 0
1402bd13a: mov qword ptr [rcx + 8], 0x10
1402bd142: mov qword ptr [rcx + 0x10], 0
1402bd14a: mov rax, rcx
1402bd14d: movaps xmm6, xmmword ptr [rbp - 0x20]
1402bd151: movaps xmm7, xmmword ptr [rbp - 0x10]
1402bd155: add rsp, 0x88
1402bd15c: pop rbx
1402bd15d: pop rdi
1402bd15e: pop rsi
1402bd15f: pop r12
1402bd161: pop r13
1402bd163: pop r14
1402bd165: pop r15
1402bd167: pop rbp
1402bd168: ret 
1402bd169: mov rcx, rbx
1402bd16c: mov rdx, r14
1402bd16f: mov r8, qword ptr [rbp - 0x30]
1402bd173: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1402bd178: int3 
1402bd179: nop dword ptr [rax]
1402bd180: mov qword ptr [rsp + 0x10], rdx
1402bd185: push rbp
1402bd186: push r15
1402bd188: push r14
1402bd18a: push r13
1402bd18c: push r12
1402bd18e: push rsi
1402bd18f: push rdi
1402bd190: push rbx
1402bd191: sub rsp, 0x48
1402bd195: lea rbp, [rdx + 0x80]
1402bd19c: movaps xmmword ptr [rsp + 0x20], xmm7
1402bd1a1: movaps xmmword ptr [rsp + 0x30], xmm6
1402bd1a6: mov rdx, qword ptr [rbp - 0x48]
1402bd1aa: test rdx, rdx
1402bd1ad: je 0x1402bd1c2
1402bd1af: mov rcx, qword ptr [rbp - 0x40]
1402bd1b3: shl rdx, 4
1402bd1b7: mov r8d, 0x10
1402bd1bd: call 0x140613c20 ; ?
1402bd1c2: movaps xmm6, xmmword ptr [rsp + 0x30]
1402bd1c7: movaps xmm7, xmmword ptr [rsp + 0x20]
1402bd1cc: add rsp, 0x48
1402bd1d0: pop rbx
1402bd1d1: pop rdi
1402bd1d2: pop rsi
1402bd1d3: pop r12
1402bd1d5: pop r13
1402bd1d7: pop r14
1402bd1d9: pop r15
1402bd1db: pop rbp
1402bd1dc: ret 
