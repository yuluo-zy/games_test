14130d8e0 55 push rbp
14130d8e1 4157 push r15
14130d8e3 4156 push r14
14130d8e5 4155 push r13
14130d8e7 4154 push r12
14130d8e9 56 push rsi
14130d8ea 57 push rdi
14130d8eb 53 push rbx
14130d8ec 4881ec48020000 sub rsp, 0x248
14130d8f3 488dac2480000000 lea rbp, [rsp + 0x80]
14130d8fb 48c785c0010000feffffff mov qword ptr [rbp + 0x1c0], 0xfffffffffffffffe
14130d906 4c89c7 mov rdi, r8
14130d909 4889ce mov rsi, rcx
14130d90c c685bf01000001 mov byte ptr [rbp + 0x1bf], 1
14130d913 488d4db0 lea rcx, [rbp - 0x50]
14130d917 488995b0010000 mov qword ptr [rbp + 0x1b0], rdx
14130d91e e87d44feff call 0x1412f1da0 ; core::iter::adapters::flatten::impl$3::next<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0>
14130d923 f645b001 test byte ptr [rbp - 0x50], 1
14130d927 7465 je 0x14130d98e
14130d929 4c8badb0010000 mov r13, qword ptr [rbp + 0x1b0]
14130d930 41837d0001 cmp dword ptr [r13], 1
14130d935 757f jne 0x14130d9b6
14130d937 498b4530 mov rax, qword ptr [r13 + 0x30]
14130d93b 498b4d48 mov rcx, qword ptr [r13 + 0x48]
14130d93f 492b4d38 sub rcx, qword ptr [r13 + 0x38]
14130d943 48c1e903 shr rcx, 3
14130d947 4885c0 test rax, rax
14130d94a 480f44c8 cmove rcx, rax
14130d94e 498b5568 mov rdx, qword ptr [r13 + 0x68]
14130d952 498b8580000000 mov rax, qword ptr [r13 + 0x80]
14130d959 492b4570 sub rax, qword ptr [r13 + 0x70]
14130d95d 48c1e803 shr rax, 3
14130d961 4885d2 test rdx, rdx
14130d964 480f44c2 cmove rax, rdx
14130d968 4801c8 add rax, rcx
14130d96b 4183bda000000001 cmp dword ptr [r13 + 0xa0], 1
14130d973 744d je 0x14130d9c2
14130d975 4531c0 xor r8d, r8d
14130d978 410fb68dc8010000 movzx ecx, byte ptr [r13 + 0x1c8]
14130d980 80f902 cmp cl, 2
14130d983 0f858d000000 jne 0x14130da16
14130d989 e917010000 jmp 0x14130daa5
14130d98e 48c70600000000 mov qword ptr [rsi], 0
14130d995 48c7460804000000 mov qword ptr [rsi + 8], 4
14130d99d 48c7461000000000 mov qword ptr [rsi + 0x10], 0
14130d9a5 488b8db0010000 mov rcx, qword ptr [rbp + 0x1b0]
14130d9ac e8ffdb56ff call 0x14087b5b0 ; core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0> >
14130d9b1 e9a4030000 jmp 0x14130dd5a
14130d9b6 31c0 xor eax, eax
14130d9b8 4183bda000000001 cmp dword ptr [r13 + 0xa0], 1
14130d9c0 75b3 jne 0x14130d975
14130d9c2 498b8dd0000000 mov rcx, qword ptr [r13 + 0xd0]
14130d9c9 498b95e8000000 mov rdx, qword ptr [r13 + 0xe8]
14130d9d0 492b95d8000000 sub rdx, qword ptr [r13 + 0xd8]
14130d9d7 48c1ea03 shr rdx, 3
14130d9db 4885c9 test rcx, rcx
14130d9de 480f44d1 cmove rdx, rcx
14130d9e2 498b8d08010000 mov rcx, qword ptr [r13 + 0x108]
14130d9e9 4d8b8520010000 mov r8, qword ptr [r13 + 0x120]
14130d9f0 4d2b8510010000 sub r8, qword ptr [r13 + 0x110]
14130d9f7 49c1e803 shr r8, 3
14130d9fb 4885c9 test rcx, rcx
14130d9fe 4c0f44c1 cmove r8, rcx
14130da02 4901d0 add r8, rdx
14130da05 410fb68dc8010000 movzx ecx, byte ptr [r13 + 0x1c8]
14130da0d 80f902 cmp cl, 2
14130da10 0f848f000000 je 0x14130daa5
14130da16 4d8b8d68010000 mov r9, qword ptr [r13 + 0x168]
14130da1d 4d8b9570010000 mov r10, qword ptr [r13 + 0x170]
14130da24 f6c101 test cl, 1
14130da27 743e je 0x14130da67
14130da29 4d39d1 cmp r9, r10
14130da2c 7477 je 0x14130daa5
14130da2e 498b8d50010000 mov rcx, qword ptr [r13 + 0x150]
14130da35 488b5110 mov rdx, qword ptr [rcx + 0x10]
14130da39 4d29ca sub r10, r9
14130da3c 49c1ea02 shr r10, 2
14130da40 4531db xor r11d, r11d
14130da43 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14130da50 438b0c99 mov ecx, dword ptr [r9 + r11*4]
14130da54 4839ca cmp rdx, rcx
14130da57 0f8630030000 jbe 0x14130dd8d
14130da5d 49ffc3 inc r11
14130da60 4d39da cmp r10, r11
14130da63 75eb jne 0x14130da50
14130da65 eb3e jmp 0x14130daa5
14130da67 4d39d1 cmp r9, r10
14130da6a 7439 je 0x14130daa5
14130da6c 498b8d58010000 mov rcx, qword ptr [r13 + 0x158]
14130da73 488b5110 mov rdx, qword ptr [rcx + 0x10]
14130da77 4d29ca sub r10, r9
14130da7a 49c1ea02 shr r10, 2
14130da7e 4531db xor r11d, r11d
14130da81 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14130da90 438b0c99 mov ecx, dword ptr [r9 + r11*4]
14130da94 4839ca cmp rdx, rcx
14130da97 0f8605030000 jbe 0x14130dda2
14130da9d 49ffc3 inc r11
14130daa0 4d39da cmp r10, r11
14130daa3 75eb jne 0x14130da90
14130daa5 4901c0 add r8, rax
14130daa8 4983f804 cmp r8, 4
14130daac bb03000000 mov ebx, 3
14130dab1 490f43d8 cmovae rbx, r8
14130dab5 48ffc3 inc rbx
14130dab8 4531ff xor r15d, r15d
14130dabb b914000000 mov ecx, 0x14
14130dac0 4889d8 mov rax, rbx
14130dac3 48f7e1 mul rcx
14130dac6 4989c6 mov r14, rax
14130dac9 0f80e8020000 jo 0x14130ddb7
14130dacf 48b8fcffffffffffff7f movabs rax, 0x7ffffffffffffffc
14130dad9 4939c6 cmp r14, rax
14130dadc 0f87d5020000 ja 0x14130ddb7
14130dae2 4d85f6 test r14, r14
14130dae5 7428 je 0x14130db0f
14130dae7 0fb6051b713202 movzx eax, byte ptr [rip + 0x232711b]
14130daee 41bf04000000 mov r15d, 4
14130daf4 ba04000000 mov edx, 4
14130daf9 4c89f1 mov rcx, r14
14130dafc e80f6130ff call 0x140613c10 ; 
14130db01 4885c0 test rax, rax
14130db04 0f84ad020000 je 0x14130ddb7
14130db0a 4989c4 mov r12, rax
14130db0d eb08 jmp 0x14130db17
14130db0f 41bc04000000 mov r12d, 4
14130db15 31db xor ebx, ebx
14130db17 488d45b4 lea rax, [rbp - 0x4c]
14130db1b 8b4810 mov ecx, dword ptr [rax + 0x10]
14130db1e 41894c2410 mov dword ptr [r12 + 0x10], ecx
14130db23 0f1000 movups xmm0, xmmword ptr [rax]
14130db26 410f110424 movups xmmword ptr [r12], xmm0
14130db2b 48899d98010000 mov qword ptr [rbp + 0x198], rbx
14130db32 4c89a5a0010000 mov qword ptr [rbp + 0x1a0], r12
14130db39 48c785a801000001000000 mov qword ptr [rbp + 0x1a8], 1
14130db44 488d7db0 lea rdi, [rbp - 0x50]
14130db48 41b8d0010000 mov r8d, 0x1d0
14130db4e 4889f9 mov rcx, rdi
14130db51 4c89ea mov rdx, r13
14130db54 e8a7606101 call 0x142923c00 ; 
14130db59 bb01000000 mov ebx, 1
14130db5e 4c8dad84010000 lea r13, [rbp + 0x184]
14130db65 4c8db580010000 lea r14, [rbp + 0x180]
14130db6c 4c8dbd98010000 lea r15, [rbp + 0x198]
14130db73 eb2c jmp 0x14130dba1
14130db75 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
14130db80 488d049b lea rax, [rbx + rbx*4]
14130db84 418b4d10 mov ecx, dword ptr [r13 + 0x10]
14130db88 41894c8410 mov dword ptr [r12 + rax*4 + 0x10], ecx
14130db8d 410f104500 movups xmm0, xmmword ptr [r13]
14130db92 410f110484 movups xmmword ptr [r12 + rax*4], xmm0
14130db97 48ffc3 inc rbx
14130db9a 48899da8010000 mov qword ptr [rbp + 0x1a8], rbx
14130dba1 4c89f1 mov rcx, r14
14130dba4 4889fa mov rdx, rdi
14130dba7 e8f441feff call 0x1412f1da0 ; core::iter::adapters::flatten::impl$3::next<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0>
14130dbac 83bd8001000001 cmp dword ptr [rbp + 0x180], 1
14130dbb3 0f8583010000 jne 0x14130dd3c
14130dbb9 483b9d98010000 cmp rbx, qword ptr [rbp + 0x198]
14130dbc0 75be jne 0x14130db80
14130dbc2 837db001 cmp dword ptr [rbp - 0x50], 1
14130dbc6 7558 jne 0x14130dc20
14130dbc8 488b45e0 mov rax, qword ptr [rbp - 0x20]
14130dbcc 488b4df8 mov rcx, qword ptr [rbp - 8]
14130dbd0 482b4de8 sub rcx, qword ptr [rbp - 0x18]
14130dbd4 48c1e903 shr rcx, 3
14130dbd8 4885c0 test rax, rax
14130dbdb 480f44c8 cmove rcx, rax
14130dbdf 488b5518 mov rdx, qword ptr [rbp + 0x18]
14130dbe3 488b4530 mov rax, qword ptr [rbp + 0x30]
14130dbe7 482b4520 sub rax, qword ptr [rbp + 0x20]
14130dbeb 48c1e803 shr rax, 3
14130dbef 4885d2 test rdx, rdx
14130dbf2 480f44c2 cmove rax, rdx
14130dbf6 4801c8 add rax, rcx
14130dbf9 837d5001 cmp dword ptr [rbp + 0x50], 1
14130dbfd 7429 je 0x14130dc28
14130dbff 4531c0 xor r8d, r8d
14130dc02 0fb68d78010000 movzx ecx, byte ptr [rbp + 0x178]
14130dc09 80f902 cmp cl, 2
14130dc0c 756d jne 0x14130dc7b
14130dc0e e9fd000000 jmp 0x14130dd10
14130dc13 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14130dc20 31c0 xor eax, eax
14130dc22 837d5001 cmp dword ptr [rbp + 0x50], 1
14130dc26 75d7 jne 0x14130dbff
14130dc28 488b8d80000000 mov rcx, qword ptr [rbp + 0x80]
14130dc2f 488b9598000000 mov rdx, qword ptr [rbp + 0x98]
14130dc36 482b9588000000 sub rdx, qword ptr [rbp + 0x88]
14130dc3d 48c1ea03 shr rdx, 3
14130dc41 4885c9 test rcx, rcx
14130dc44 480f44d1 cmove rdx, rcx
14130dc48 488b8db8000000 mov rcx, qword ptr [rbp + 0xb8]
14130dc4f 4c8b85d0000000 mov r8, qword ptr [rbp + 0xd0]
14130dc56 4c2b85c0000000 sub r8, qword ptr [rbp + 0xc0]
14130dc5d 49c1e803 shr r8, 3
14130dc61 4885c9 test rcx, rcx
14130dc64 4c0f44c1 cmove r8, rcx
14130dc68 4901d0 add r8, rdx
14130dc6b 0fb68d78010000 movzx ecx, byte ptr [rbp + 0x178]
14130dc72 80f902 cmp cl, 2
14130dc75 0f8495000000 je 0x14130dd10
14130dc7b 4c8b8d18010000 mov r9, qword ptr [rbp + 0x118]
14130dc82 4c8b9520010000 mov r10, qword ptr [rbp + 0x120]
14130dc89 f6c101 test cl, 1
14130dc8c 7439 je 0x14130dcc7
14130dc8e 4d39d1 cmp r9, r10
14130dc91 747d je 0x14130dd10
14130dc93 488b8d00010000 mov rcx, qword ptr [rbp + 0x100]
14130dc9a 488b5110 mov rdx, qword ptr [rcx + 0x10]
14130dc9e 4d29ca sub r10, r9
14130dca1 49c1ea02 shr r10, 2
14130dca5 4531db xor r11d, r11d
14130dca8 0f1f840000000000 nop dword ptr [rax + rax]
14130dcb0 438b0c99 mov ecx, dword ptr [r9 + r11*4]
14130dcb4 4839ca cmp rdx, rcx
14130dcb7 0f86b4000000 jbe 0x14130dd71
14130dcbd 49ffc3 inc r11
14130dcc0 4d39da cmp r10, r11
14130dcc3 75eb jne 0x14130dcb0
14130dcc5 eb49 jmp 0x14130dd10
14130dcc7 4d39d1 cmp r9, r10
14130dcca 7444 je 0x14130dd10
14130dccc 488b8d08010000 mov rcx, qword ptr [rbp + 0x108]
14130dcd3 488b5110 mov rdx, qword ptr [rcx + 0x10]
14130dcd7 4d29ca sub r10, r9
14130dcda 49c1ea02 shr r10, 2
14130dcde 4531db xor r11d, r11d
14130dce1 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14130dcf0 438b0c99 mov ecx, dword ptr [r9 + r11*4]
14130dcf4 4839ca cmp rdx, rcx
14130dcf7 0f8682000000 jbe 0x14130dd7f
14130dcfd 49ffc3 inc r11
14130dd00 4d39da cmp r10, r11
14130dd03 75eb jne 0x14130dcf0
14130dd05 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
14130dd10 4901c0 add r8, rax
14130dd13 49ffc0 inc r8
14130dd16 48c744242014000000 mov qword ptr [rsp + 0x20], 0x14
14130dd1f 41b904000000 mov r9d, 4
14130dd25 4c89f9 mov rcx, r15
14130dd28 4889da mov rdx, rbx
14130dd2b e830bd5901 call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
14130dd30 4c8ba5a0010000 mov r12, qword ptr [rbp + 0x1a0]
14130dd37 e944feffff jmp 0x14130db80
14130dd3c 488d4db0 lea rcx, [rbp - 0x50]
14130dd40 e86bd856ff call 0x14087b5b0 ; core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0> >
14130dd45 488b85a8010000 mov rax, qword ptr [rbp + 0x1a8]
14130dd4c 48894610 mov qword ptr [rsi + 0x10], rax
14130dd50 0f108598010000 movups xmm0, xmmword ptr [rbp + 0x198]
14130dd57 0f1106 movups xmmword ptr [rsi], xmm0
14130dd5a 4889f0 mov rax, rsi
14130dd5d 4881c448020000 add rsp, 0x248
14130dd64 5b pop rbx
14130dd65 5f pop rdi
14130dd66 5e pop rsi
14130dd67 415c pop r12
14130dd69 415d pop r13
14130dd6b 415e pop r14
14130dd6d 415f pop r15
14130dd6f 5d pop rbp
14130dd70 c3 ret 
14130dd71 4c8d0590b69c01 lea r8, [rip + 0x19cb690]
14130dd78 e89bb75c01 call 0x1428d9518 ; core::panicking::panic_bounds_check
14130dd7d eb4d jmp 0x14130ddcc
14130dd7f 4c8d059ab69c01 lea r8, [rip + 0x19cb69a]
14130dd86 e88db75c01 call 0x1428d9518 ; core::panicking::panic_bounds_check
14130dd8b eb3f jmp 0x14130ddcc
14130dd8d c685bf01000001 mov byte ptr [rbp + 0x1bf], 1
14130dd94 4c8d056db69c01 lea r8, [rip + 0x19cb66d]
14130dd9b e878b75c01 call 0x1428d9518 ; core::panicking::panic_bounds_check
14130dda0 eb2a jmp 0x14130ddcc
14130dda2 c685bf01000001 mov byte ptr [rbp + 0x1bf], 1
14130dda9 4c8d0570b69c01 lea r8, [rip + 0x19cb670]
14130ddb0 e863b75c01 call 0x1428d9518 ; core::panicking::panic_bounds_check
14130ddb5 eb15 jmp 0x14130ddcc
14130ddb7 c685bf01000001 mov byte ptr [rbp + 0x1bf], 1
14130ddbe 4c89f9 mov rcx, r15
14130ddc1 4c89f2 mov rdx, r14
14130ddc4 4989f8 mov r8, rdi
14130ddc7 e817b25c01 call 0x1428d8fe3 ; alloc::raw_vec::handle_error
14130ddcc 0f0b ud2 
14130ddce 6690 nop 
14130ddd0 4889542410 mov qword ptr [rsp + 0x10], rdx
14130ddd5 55 push rbp
14130ddd6 4157 push r15
14130ddd8 4156 push r14
14130ddda 4155 push r13
14130dddc 4154 push r12
14130ddde 56 push rsi
14130dddf 57 push rdi
14130dde0 53 push rbx
14130dde1 4883ec28 sub rsp, 0x28
14130dde5 488daa80000000 lea rbp, [rdx + 0x80]
14130ddec 488d4db0 lea rcx, [rbp - 0x50]
14130ddf0 e8bbd756ff call 0x14087b5b0 ; core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0> >
14130ddf5 90 nop 
14130ddf6 4883c428 add rsp, 0x28
14130ddfa 5b pop rbx
14130ddfb 5f pop rdi
14130ddfc 5e pop rsi
14130ddfd 415c pop r12
14130ddff 415d pop r13
14130de01 415e pop r14
14130de03 415f pop r15
14130de05 5d pop rbp
14130de06 c3 ret 
14130de07 660f1f840000000000 nop word ptr [rax + rax]
14130de10 4889542410 mov qword ptr [rsp + 0x10], rdx
14130de15 55 push rbp
14130de16 4157 push r15
14130de18 4156 push r14
14130de1a 4155 push r13
14130de1c 4154 push r12
14130de1e 56 push rsi
14130de1f 57 push rdi
14130de20 53 push rbx
14130de21 4883ec28 sub rsp, 0x28
14130de25 488daa80000000 lea rbp, [rdx + 0x80]
14130de2c 488b8598010000 mov rax, qword ptr [rbp + 0x198]
14130de33 4885c0 test rax, rax
14130de36 741a je 0x14130de52
14130de38 488b8da0010000 mov rcx, qword ptr [rbp + 0x1a0]
14130de3f 48c1e002 shl rax, 2
14130de43 488d1480 lea rdx, [rax + rax*4]
14130de47 41b804000000 mov r8d, 4
14130de4d e8ce5d30ff call 0x140613c20 ; 
14130de52 c685bf01000000 mov byte ptr [rbp + 0x1bf], 0
14130de59 4883c428 add rsp, 0x28
14130de5d 5b pop rbx
14130de5e 5f pop rdi
14130de5f 5e pop rsi
14130de60 415c pop r12
14130de62 415d pop r13
14130de64 415e pop r14
14130de66 415f pop r15
14130de68 5d pop rbp
14130de69 c3 ret 
14130de6a 660f1f440000 nop word ptr [rax + rax]
14130de70 4889542410 mov qword ptr [rsp + 0x10], rdx
14130de75 55 push rbp
14130de76 4157 push r15
14130de78 4156 push r14
14130de7a 4155 push r13
14130de7c 4154 push r12
14130de7e 56 push rsi
14130de7f 57 push rdi
14130de80 53 push rbx
14130de81 4883ec28 sub rsp, 0x28
14130de85 488daa80000000 lea rbp, [rdx + 0x80]
14130de8c 80bdbf01000000 cmp byte ptr [rbp + 0x1bf], 0
14130de93 7511 jne 0x14130dea6
14130de95 4883c428 add rsp, 0x28
14130de99 5b pop rbx
14130de9a 5f pop rdi
14130de9b 5e pop rsi
14130de9c 415c pop r12
14130de9e 415d pop r13
14130dea0 415e pop r14
14130dea2 415f pop r15
14130dea4 5d pop rbp
14130dea5 c3 ret 
14130dea6 488b8db0010000 mov rcx, qword ptr [rbp + 0x1b0]
14130dead e8fed656ff call 0x14087b5b0 ; core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0> >
14130deb2 ebe1 jmp 0x14130de95
