1412fde80: push rbp
1412fde81: push r15
1412fde83: push r14
1412fde85: push r13
1412fde87: push r12
1412fde89: push rsi
1412fde8a: push rdi
1412fde8b: push rbx
1412fde8c: sub rsp, 0x128
1412fde93: lea rbp, [rsp + 0x80]
1412fde9b: movaps xmmword ptr [rbp + 0x90], xmm6
1412fdea2: mov qword ptr [rbp + 0x88], 0xfffffffffffffffe
1412fdead: mov rbx, r8
1412fdeb0: mov rdi, rdx
1412fdeb3: mov rsi, rcx
1412fdeb6: lea rcx, [rbp - 0x30]
1412fdeba: call 0x1412f0c80 ; core::iter::adapters::chain::impl$1::next<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> >
1412fdebf: test byte ptr [rbp - 0x30], 1
1412fdec3: je 0x1412fdef2
1412fdec5: movaps xmm6, xmmword ptr [rbp - 0x20]
1412fdec9: movzx ecx, byte ptr [rdi + 0x90]
1412fded0: mov rax, qword ptr [rdi]
1412fded3: mov rdx, qword ptr [rdi + 8]
1412fded7: cmp cl, 2
1412fdeda: jne 0x1412fdf0e
1412fdedc: mov rcx, rax
1412fdedf: xor rcx, 2
1412fdee3: xor r8d, r8d
1412fdee6: or rcx, rdx
1412fdee9: cmovne r8, rax
1412fdeed: jmp 0x1412fe0a6 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >
1412fdef2: mov qword ptr [rsi], 0
1412fdef9: mov qword ptr [rsi + 8], 0x10
1412fdf01: mov qword ptr [rsi + 0x10], 0
1412fdf09: jmp 0x1412fe37f ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >
1412fdf0e: mov r8, rax
1412fdf11: xor r8, 2
1412fdf15: or r8, rdx
1412fdf18: jne 0x1412fdf73
1412fdf1a: mov rax, qword ptr [rdi + 0x40]
1412fdf1e: mov r9, qword ptr [rdi + 0x48]
1412fdf22: test cl, 1
1412fdf25: je 0x1412fdfd3
1412fdf2b: cmp rax, r9
1412fdf2e: je 0x1412fe074
1412fdf34: mov rcx, qword ptr [rdi + 0x28]
1412fdf38: mov r10, qword ptr [rcx + 8]
1412fdf3c: mov rdx, qword ptr [rcx + 0x10]
1412fdf40: sub r9, rax
1412fdf43: shr r9, 2
1412fdf47: xor r8d, r8d
1412fdf4a: xor r11d, r11d
1412fdf4d: nop dword ptr [rax]
1412fdf50: mov ecx, dword ptr [rax + r11*4]
1412fdf54: cmp rdx, rcx
1412fdf57: jbe 0x1412fe3d5
1412fdf5d: lea rcx, [rcx + rcx*8]
1412fdf61: add r8, qword ptr [r10 + rcx*8 + 0x10]
1412fdf66: inc r11
1412fdf69: cmp r9, r11
1412fdf6c: jne 0x1412fdf50
1412fdf6e: jmp 0x1412fe077 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >
1412fdf73: mov r8, qword ptr [rdi + 0x40]
1412fdf77: mov r9, qword ptr [rdi + 0x48]
1412fdf7b: test cl, 1
1412fdf7e: je 0x1412fe024
1412fdf84: cmp r8, r9
1412fdf87: je 0x1412fe087
1412fdf8d: mov rcx, qword ptr [rdi + 0x28]
1412fdf91: mov r11, qword ptr [rcx + 8]
1412fdf95: mov rdx, qword ptr [rcx + 0x10]
1412fdf99: sub r9, r8
1412fdf9c: shr r9, 2
1412fdfa0: xor r10d, r10d
1412fdfa3: xor r14d, r14d
1412fdfa6: nop word ptr cs:[rax + rax]
1412fdfb0: mov ecx, dword ptr [r8 + r14*4]
1412fdfb4: cmp rdx, rcx
1412fdfb7: jbe 0x1412fe3d5
1412fdfbd: lea rcx, [rcx + rcx*8]
1412fdfc1: add r10, qword ptr [r11 + rcx*8 + 0x10]
1412fdfc6: inc r14
1412fdfc9: cmp r9, r14
1412fdfcc: jne 0x1412fdfb0
1412fdfce: jmp 0x1412fe08a ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >
1412fdfd3: cmp rax, r9
1412fdfd6: je 0x1412fe074
1412fdfdc: mov rcx, qword ptr [rdi + 0x30]
1412fdfe0: mov r10, qword ptr [rcx + 8]
1412fdfe4: mov rdx, qword ptr [rcx + 0x10]
1412fdfe8: sub r9, rax
1412fdfeb: shr r9, 2
1412fdfef: xor r8d, r8d
1412fdff2: xor r11d, r11d
1412fdff5: nop word ptr cs:[rax + rax]
1412fe000: mov ecx, dword ptr [rax + r11*4]
1412fe004: cmp rdx, rcx
1412fe007: jbe 0x1412fe3e1
1412fe00d: lea rcx, [rcx + rcx*4]
1412fe011: shl rcx, 5
1412fe015: add r8, qword ptr [r10 + rcx + 0x58]
1412fe01a: inc r11
1412fe01d: cmp r9, r11
1412fe020: jne 0x1412fe000
1412fe022: jmp 0x1412fe077 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >
1412fe024: cmp r8, r9
1412fe027: je 0x1412fe087
1412fe029: mov rcx, qword ptr [rdi + 0x30]
1412fe02d: mov r11, qword ptr [rcx + 8]
1412fe031: mov rdx, qword ptr [rcx + 0x10]
1412fe035: sub r9, r8
1412fe038: shr r9, 2
1412fe03c: xor r10d, r10d
1412fe03f: xor r14d, r14d
1412fe042: nop word ptr cs:[rax + rax]
1412fe050: mov ecx, dword ptr [r8 + r14*4]
1412fe054: cmp rdx, rcx
1412fe057: jbe 0x1412fe3e1
1412fe05d: lea rcx, [rcx + rcx*4]
1412fe061: shl rcx, 5
1412fe065: add r10, qword ptr [r11 + rcx + 0x58]
1412fe06a: inc r14
1412fe06d: cmp r9, r14
1412fe070: jne 0x1412fe050
1412fe072: jmp 0x1412fe08a ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >
1412fe074: xor r8d, r8d
1412fe077: add r8, qword ptr [rdi + 0x80]
1412fe07e: sub r8, qword ptr [rdi + 0x88]
1412fe085: jmp 0x1412fe0a6 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >
1412fe087: xor r10d, r10d
1412fe08a: add r10, qword ptr [rdi + 0x80]
1412fe091: sub r10, qword ptr [rdi + 0x88]
1412fe098: add r10, rax
1412fe09b: mov r8, 0xffffffffffffffff
1412fe0a2: cmovae r8, r10
1412fe0a6: inc r8
1412fe0a9: mov rax, 0xffffffffffffffff
1412fe0b0: cmovne rax, r8
1412fe0b4: cmp rax, 5
1412fe0b8: mov r13d, 4
1412fe0be: cmovae r13, rax
1412fe0c2: mov r14, r13
1412fe0c5: shl r14, 4
1412fe0c9: xor r15d, r15d
1412fe0cc: shr rax, 0x3c
1412fe0d0: jne 0x1412fe3ed
1412fe0d6: movabs rax, 0x7ffffffffffffff0
1412fe0e0: cmp r14, rax
1412fe0e3: ja 0x1412fe3ed
1412fe0e9: movzx eax, byte ptr [rip + 0x2336b19]
1412fe0f0: mov r15d, 0x10
1412fe0f6: mov edx, 0x10
1412fe0fb: mov rcx, r14
1412fe0fe: call 0x140613c10 ; ?
1412fe103: test rax, rax
1412fe106: je 0x1412fe3ed
1412fe10c: mov r12, rax
1412fe10f: movaps xmmword ptr [rax], xmm6
1412fe112: mov qword ptr [rbp + 0x70], r13
1412fe116: mov qword ptr [rbp + 0x78], rax
1412fe11a: mov qword ptr [rbp + 0x80], 1
1412fe125: lea rbx, [rbp - 0x30]
1412fe129: mov r8d, 0xa0
1412fe12f: mov rcx, rbx
1412fe132: mov rdx, rdi
1412fe135: call 0x142923c00 ; ?
1412fe13a: mov edi, 1
1412fe13f: lea r14, [rbp - 0x50]
1412fe143: mov r13, 0xffffffffffffffff
1412fe14a: lea r15, [rbp + 0x70]
1412fe14e: jmp 0x1412fe166 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >
1412fe150: mov rax, rdi
1412fe153: shl rax, 4
1412fe157: movaps xmmword ptr [r12 + rax], xmm6
1412fe15c: inc rdi
1412fe15f: mov qword ptr [rbp + 0x80], rdi
1412fe166: mov rcx, r14
1412fe169: mov rdx, rbx
1412fe16c: call 0x1412f0c80 ; core::iter::adapters::chain::impl$1::next<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> >
1412fe171: test byte ptr [rbp - 0x50], 1
1412fe175: je 0x1412fe36d
1412fe17b: movaps xmm6, xmmword ptr [rbp - 0x40]
1412fe17f: cmp rdi, qword ptr [rbp + 0x70]
1412fe183: jne 0x1412fe150
1412fe185: movzx ecx, byte ptr [rbp + 0x60]
1412fe189: mov rax, qword ptr [rbp - 0x30]
1412fe18d: mov rdx, qword ptr [rbp - 0x28]
1412fe191: cmp cl, 2
1412fe194: jne 0x1412fe1c0
1412fe196: mov rcx, rax
1412fe199: xor rcx, 2
1412fe19d: or rcx, rdx
1412fe1a0: mov ecx, 0
1412fe1a5: cmove rax, rcx
1412fe1a9: mov r8, rax
1412fe1ac: jmp 0x1412fe343 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >
1412fe1b1: nop word ptr cs:[rax + rax]
1412fe1c0: mov r8, rax
1412fe1c3: xor r8, 2
1412fe1c7: or r8, rdx
1412fe1ca: jne 0x1412fe223
1412fe1cc: mov rax, qword ptr [rbp + 0x10]
1412fe1d0: mov r9, qword ptr [rbp + 0x18]
1412fe1d4: test cl, 1
1412fe1d7: je 0x1412fe283
1412fe1dd: cmp rax, r9
1412fe1e0: je 0x1412fe324
1412fe1e6: mov rcx, qword ptr [rbp - 8]
1412fe1ea: mov r10, qword ptr [rcx + 8]
1412fe1ee: mov rdx, qword ptr [rcx + 0x10]
1412fe1f2: sub r9, rax
1412fe1f5: shr r9, 2
1412fe1f9: xor r8d, r8d
1412fe1fc: xor r11d, r11d
1412fe1ff: nop 
1412fe200: mov ecx, dword ptr [rax + r11*4]
1412fe204: cmp rdx, rcx
1412fe207: jbe 0x1412fe3b9
1412fe20d: lea rcx, [rcx + rcx*8]
1412fe211: add r8, qword ptr [r10 + rcx*8 + 0x10]
1412fe216: inc r11
1412fe219: cmp r9, r11
1412fe21c: jne 0x1412fe200
1412fe21e: jmp 0x1412fe327 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >
1412fe223: mov r9, qword ptr [rbp + 0x10]
1412fe227: mov r10, qword ptr [rbp + 0x18]
1412fe22b: test cl, 1
1412fe22e: je 0x1412fe2d4
1412fe234: cmp r9, r10
1412fe237: je 0x1412fe331
1412fe23d: mov rcx, qword ptr [rbp - 8]
1412fe241: mov r11, qword ptr [rcx + 8]
1412fe245: mov rdx, qword ptr [rcx + 0x10]
1412fe249: sub r10, r9
1412fe24c: shr r10, 2
1412fe250: xor r8d, r8d
1412fe253: xor r12d, r12d
1412fe256: nop word ptr cs:[rax + rax]
1412fe260: mov ecx, dword ptr [r9 + r12*4]
1412fe264: cmp rdx, rcx
1412fe267: jbe 0x1412fe39d
1412fe26d: lea rcx, [rcx + rcx*8]
1412fe271: add r8, qword ptr [r11 + rcx*8 + 0x10]
1412fe276: inc r12
1412fe279: cmp r10, r12
1412fe27c: jne 0x1412fe260
1412fe27e: jmp 0x1412fe334 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >
1412fe283: cmp rax, r9
1412fe286: je 0x1412fe324
1412fe28c: mov rcx, qword ptr [rbp]
1412fe290: mov r10, qword ptr [rcx + 8]
1412fe294: mov rdx, qword ptr [rcx + 0x10]
1412fe298: sub r9, rax
1412fe29b: shr r9, 2
1412fe29f: xor r8d, r8d
1412fe2a2: xor r11d, r11d
1412fe2a5: nop word ptr cs:[rax + rax]
1412fe2b0: mov ecx, dword ptr [rax + r11*4]
1412fe2b4: cmp rdx, rcx
1412fe2b7: jbe 0x1412fe3ab
1412fe2bd: lea rcx, [rcx + rcx*4]
1412fe2c1: shl rcx, 5
1412fe2c5: add r8, qword ptr [r10 + rcx + 0x58]
1412fe2ca: inc r11
1412fe2cd: cmp r9, r11
1412fe2d0: jne 0x1412fe2b0
1412fe2d2: jmp 0x1412fe327 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >
1412fe2d4: cmp r9, r10
1412fe2d7: je 0x1412fe331
1412fe2d9: mov rcx, qword ptr [rbp]
1412fe2dd: mov r11, qword ptr [rcx + 8]
1412fe2e1: mov rdx, qword ptr [rcx + 0x10]
1412fe2e5: sub r10, r9
1412fe2e8: shr r10, 2
1412fe2ec: xor r8d, r8d
1412fe2ef: xor r12d, r12d
1412fe2f2: nop word ptr cs:[rax + rax]
1412fe300: mov ecx, dword ptr [r9 + r12*4]
1412fe304: cmp rdx, rcx
1412fe307: jbe 0x1412fe3c7
1412fe30d: lea rcx, [rcx + rcx*4]
1412fe311: shl rcx, 5
1412fe315: add r8, qword ptr [r11 + rcx + 0x58]
1412fe31a: inc r12
1412fe31d: cmp r10, r12
1412fe320: jne 0x1412fe300
1412fe322: jmp 0x1412fe334 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >
1412fe324: xor r8d, r8d
1412fe327: add r8, qword ptr [rbp + 0x50]
1412fe32b: sub r8, qword ptr [rbp + 0x58]
1412fe32f: jmp 0x1412fe343 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >
1412fe331: xor r8d, r8d
1412fe334: add r8, qword ptr [rbp + 0x50]
1412fe338: sub r8, qword ptr [rbp + 0x58]
1412fe33c: add r8, rax
1412fe33f: cmovb r8, r13
1412fe343: inc r8
1412fe346: cmove r8, r13
1412fe34a: mov qword ptr [rsp + 0x20], 0x10
1412fe353: mov r9d, 0x10
1412fe359: mov rcx, r15
1412fe35c: mov rdx, rdi
1412fe35f: call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412fe364: mov r12, qword ptr [rbp + 0x78]
1412fe368: jmp 0x1412fe150 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >
1412fe36d: mov rax, qword ptr [rbp + 0x80]
1412fe374: mov qword ptr [rsi + 0x10], rax
1412fe378: movups xmm0, xmmword ptr [rbp + 0x70]
1412fe37c: movups xmmword ptr [rsi], xmm0
1412fe37f: mov rax, rsi
1412fe382: movaps xmm6, xmmword ptr [rbp + 0x90]
1412fe389: add rsp, 0x128
1412fe390: pop rbx
1412fe391: pop rdi
1412fe392: pop rsi
1412fe393: pop r12
1412fe395: pop r13
1412fe397: pop r14
1412fe399: pop r15
1412fe39b: pop rbp
1412fe39c: ret 
1412fe39d: lea r8, [rip + 0x19db064]
1412fe3a4: call 0x1428d9518 ; core::panicking::panic_bounds_check
1412fe3a9: jmp 0x1412fe3d3 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >
1412fe3ab: lea r8, [rip + 0x19db06e]
1412fe3b2: call 0x1428d9518 ; core::panicking::panic_bounds_check
1412fe3b7: jmp 0x1412fe3d3 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >
1412fe3b9: lea r8, [rip + 0x19db048]
1412fe3c0: call 0x1428d9518 ; core::panicking::panic_bounds_check
1412fe3c5: jmp 0x1412fe3d3 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<utils::components::transform::Transform>,ref$<country_core::systems::render_loop::compute_passes::grass::ObjectGrassPush> >,tuple$<> >,country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass::closure_env$0>,core::option::IntoIter<glam::f32::sse2::vec4::Vec4> > >
1412fe3c7: lea r8, [rip + 0x19db052]
1412fe3ce: call 0x1428d9518 ; core::panicking::panic_bounds_check
1412fe3d3: ud2 
1412fe3d5: lea r8, [rip + 0x19db02c]
1412fe3dc: call 0x1428d9518 ; core::panicking::panic_bounds_check
1412fe3e1: lea r8, [rip + 0x19db038]
1412fe3e8: call 0x1428d9518 ; core::panicking::panic_bounds_check
1412fe3ed: mov rcx, r15
1412fe3f0: mov rdx, r14
1412fe3f3: mov r8, rbx
1412fe3f6: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1412fe3fb: int3 
1412fe3fc: nop dword ptr [rax]
1412fe400: mov qword ptr [rsp + 0x10], rdx
1412fe405: push rbp
1412fe406: push r15
1412fe408: push r14
1412fe40a: push r13
1412fe40c: push r12
1412fe40e: push rsi
1412fe40f: push rdi
1412fe410: push rbx
1412fe411: sub rsp, 0x38
1412fe415: lea rbp, [rdx + 0x80]
1412fe41c: movaps xmmword ptr [rsp + 0x20], xmm6
1412fe421: mov rdx, qword ptr [rbp + 0x70]
1412fe425: test rdx, rdx
1412fe428: je 0x1412fe43d
1412fe42a: mov rcx, qword ptr [rbp + 0x78]
1412fe42e: shl rdx, 4
1412fe432: mov r8d, 0x10
1412fe438: call 0x140613c20 ; ?
1412fe43d: movaps xmm6, xmmword ptr [rsp + 0x20]
1412fe442: add rsp, 0x38
1412fe446: pop rbx
1412fe447: pop rdi
1412fe448: pop rsi
1412fe449: pop r12
1412fe44b: pop r13
1412fe44d: pop r14
1412fe44f: pop r15
1412fe451: pop rbp
1412fe452: ret 
