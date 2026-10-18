140a21620: push rbp
140a21621: push r15
140a21623: push r14
140a21625: push r13
140a21627: push r12
140a21629: push rsi
140a2162a: push rdi
140a2162b: push rbx
140a2162c: sub rsp, 0x108
140a21633: lea rbp, [rsp + 0x80]
140a2163b: movaps xmmword ptr [rbp + 0x70], xmm6
140a2163f: mov qword ptr [rbp + 0x68], 0xfffffffffffffffe
140a21647: mov rbx, r8
140a2164a: mov r14, rdx
140a2164d: mov rax, qword ptr [rbp + 0xf0]
140a21654: mov r13, qword ptr [rcx]
140a21657: mov rcx, qword ptr [r13]
140a2165b: add rcx, 0x10
140a2165f: mov r12, qword ptr [rax]
140a21662: mov rsi, qword ptr [r9]
140a21665: mov qword ptr [rsp + 0x30], rsi
140a2166a: mov qword ptr [rsp + 0x28], r12
140a2166f: mov qword ptr [rsp + 0x20], 0x11
140a21678: lea rdx, [rip + 0x20a8019]
140a2167f: lea r9, [rip + 0x20a8030]
140a21686: mov r8d, 0x1e
140a2168c: call 0x140b3fe40 ; country_core::startup::load_shader_into_library<ref$<str$> >
140a21691: mov qword ptr [rbp - 0x40], 0
140a21699: mov qword ptr [rbp - 0x10], 0
140a216a1: mov dword ptr [rbp + 0x20], 0
140a216a8: movabs rax, 0x500000000
140a216b2: mov qword ptr [rbp + 0x2c], rax
140a216b6: mov byte ptr [rbp + 0x34], 0
140a216ba: mov qword ptr [rbp + 0x38], 0x1b
140a216c2: lea r8, [rip + 0x20a73bf]
140a216c9: lea rcx, [rbp + 0x48]
140a216cd: lea rdx, [rbp - 0x40]
140a216d1: call 0x1412fff70 ; alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::take::Take<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::ops::range::RangeInclusive<u32>,tuple$<u32,u32> >,array$<u32,6>,country_core::startup::startup_grass::startup_grass::closure_env$0> > >
140a216d6: call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140a216db: mov rdi, rax
140a216de: test byte ptr [rax], 1
140a216e1: je 0x140a216f4
140a216e3: mov rcx, rdi
140a216e6: add rcx, 8
140a216ea: mov rax, qword ptr [rdi + 8]
140a216ee: mov rdx, qword ptr [rdi + 0x10]
140a216f2: jmp 0x140a2170f
140a216f4: call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140a216f9: mov rcx, rdi
140a216fc: add rcx, 8
140a21700: mov qword ptr [rdi], 1
140a21707: mov qword ptr [rdi + 8], rax
140a2170b: mov qword ptr [rdi + 0x10], rdx
140a2170f: mov rdi, qword ptr [rbp + 0xf8]
140a21716: lea r8, [rax + 1]
140a2171a: mov qword ptr [rcx], r8
140a2171d: mov rcx, qword ptr [rbp + 0x58]
140a21721: mov qword ptr [rbp - 0x30], rcx
140a21725: movups xmm0, xmmword ptr [rbp + 0x48]
140a21729: movaps xmmword ptr [rbp - 0x40], xmm0
140a2172d: movabs rcx, 0x8000000000000000
140a21737: mov qword ptr [rbp - 0x28], rcx
140a2173b: movups xmm0, xmmword ptr [rip + 0x20a72ce]
140a21742: movups xmmword ptr [rbp - 8], xmm0
140a21746: movups xmm0, xmmword ptr [rip + 0x20a72d3]
140a2174d: movups xmmword ptr [rbp + 8], xmm0
140a21751: mov qword ptr [rbp + 0x18], rax
140a21755: mov qword ptr [rbp + 0x20], rdx
140a21759: lea rax, [rip + 0x20a7f67]
140a21760: mov qword ptr [rbp + 0x28], rax
140a21764: mov qword ptr [rbp + 0x30], 0xb
140a2176c: mov r9, qword ptr [rbx]
140a2176f: mov rax, qword ptr [r14]
140a21772: mov qword ptr [rsp + 0x20], rax
140a21777: lea rdx, [rip + 0x20a7f54]
140a2177e: lea rcx, [rbp - 0x40]
140a21782: mov r8d, 0xb
140a21788: call 0x140b1aca0 ; country_core::startup::load_mesh_into_library
140a2178d: mov qword ptr [rbp + 0x60], rax
140a21791: mov rcx, qword ptr [r13]
140a21795: add rcx, 0x10
140a21799: mov qword ptr [rsp + 0x30], rsi
140a2179e: mov qword ptr [rsp + 0x28], r12
140a217a3: mov qword ptr [rsp + 0x20], 0xc
140a217ac: lea rdx, [rip + 0x20a7f2a]
140a217b3: lea r9, [rip + 0x20a7f35]
140a217ba: mov r8d, 0x12
140a217c0: call 0x140b3fe40 ; country_core::startup::load_shader_into_library<ref$<str$> >
140a217c5: mov rbx, rax
140a217c8: lea rcx, [rbp - 0x40]
140a217cc: mov rdx, rdi
140a217cf: call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
140a217d4: mov r14, qword ptr [rbp - 0x38]
140a217d8: test r14, r14
140a217db: jne 0x140a217e1
140a217dd: mov r14, qword ptr [rbp - 0x30]
140a217e1: mov rdi, qword ptr [rbp - 0x20]
140a217e5: mov rax, qword ptr [r14]
140a217e8: mov r15, qword ptr [r14 + 0x10]
140a217ec: sub rax, r15
140a217ef: cmp rax, 0x87
140a217f5: jbe 0x140a21978
140a217fb: mov rax, qword ptr [r14 + 8]
140a217ff: lea rcx, [rip + 0x8bac6a]
140a21806: mov qword ptr [rax + r15], rcx
140a2180a: movaps xmm6, xmmword ptr [rip + 0x1f1c8af]
140a21811: movups xmmword ptr [rax + r15 + 8], xmm6
140a21817: mov qword ptr [rax + r15 + 0x18], 0
140a21820: mov dword ptr [rax + r15 + 0x20], 0
140a21829: movabs rcx, 0x3f8000003f800000
140a21833: mov qword ptr [rax + r15 + 0x24], rcx
140a21838: mov dword ptr [rax + r15 + 0x2c], 0x3f800000
140a21841: mov rcx, qword ptr [rbp + 0x60]
140a21845: mov qword ptr [rax + r15 + 0x38], rcx
140a2184a: mov qword ptr [rax + r15 + 0x40], rbx
140a2184f: mov dword ptr [rax + r15 + 0x48], 7
140a21858: mov byte ptr [rax + r15 + 0x58], 0
140a2185e: mov qword ptr [rax + r15 + 0x68], rdi
140a21863: lea rcx, [rip + 0xb434c6]
140a2186a: mov qword ptr [rax + r15 + 0x78], rcx
140a2186f: add r15, 0x88
140a21876: mov qword ptr [r14 + 0x10], r15
140a2187a: mov rcx, qword ptr [r13]
140a2187e: add rcx, 0x10
140a21882: mov qword ptr [rsp + 0x30], rsi
140a21887: mov qword ptr [rsp + 0x28], r12
140a2188c: mov qword ptr [rsp + 0x20], 0x11
140a21895: lea rdx, [rip + 0x20a7e5f]
140a2189c: lea r9, [rip + 0x20a7e6f]
140a218a3: mov r8d, 0x17
140a218a9: call 0x140b3fe40 ; country_core::startup::load_shader_into_library<ref$<str$> >
140a218ae: mov rbx, rax
140a218b1: lea rcx, [rbp - 0x40]
140a218b5: mov rdx, qword ptr [rbp + 0xf8]
140a218bc: call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
140a218c1: mov rdi, qword ptr [rbp - 0x38]
140a218c5: test rdi, rdi
140a218c8: jne 0x140a218ce
140a218ca: mov rdi, qword ptr [rbp - 0x30]
140a218ce: mov rsi, qword ptr [rbp - 0x20]
140a218d2: mov rax, qword ptr [rdi]
140a218d5: mov r14, qword ptr [rdi + 0x10]
140a218d9: sub rax, r14
140a218dc: cmp rax, 0x87
140a218e2: jbe 0x140a2199d
140a218e8: mov rax, qword ptr [rdi + 8]
140a218ec: lea rcx, [rip + 0x8bab7d]
140a218f3: mov qword ptr [rax + r14], rcx
140a218f7: movups xmmword ptr [rax + r14 + 8], xmm6
140a218fd: mov qword ptr [rax + r14 + 0x18], 0
140a21906: mov dword ptr [rax + r14 + 0x20], 0
140a2190f: movabs rcx, 0x3f8000003f800000
140a21919: mov qword ptr [rax + r14 + 0x24], rcx
140a2191e: mov dword ptr [rax + r14 + 0x2c], 0x3f800000
140a21927: mov rcx, qword ptr [rbp + 0x60]
140a2192b: mov qword ptr [rax + r14 + 0x38], rcx
140a21930: mov qword ptr [rax + r14 + 0x40], rbx
140a21935: mov dword ptr [rax + r14 + 0x48], 7
140a2193e: mov byte ptr [rax + r14 + 0x58], 0
140a21944: mov qword ptr [rax + r14 + 0x68], rsi
140a21949: lea rcx, [rip + 0xb433e0]
140a21950: mov qword ptr [rax + r14 + 0x78], rcx
140a21955: add r14, 0x88
140a2195c: mov qword ptr [rdi + 0x10], r14
140a21960: movaps xmm6, xmmword ptr [rbp + 0x70]
140a21964: add rsp, 0x108
140a2196b: pop rbx
140a2196c: pop rdi
140a2196d: pop rsi
140a2196e: pop r12
140a21970: pop r13
140a21972: pop r14
140a21974: pop r15
140a21976: pop rbp
140a21977: ret 
140a21978: mov qword ptr [rsp + 0x20], 1
140a21981: mov r8d, 0x88
140a21987: mov r9d, 1
140a2198d: mov rcx, r14
140a21990: mov rdx, r15
140a21993: call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140a21998: jmp 0x140a217fb
140a2199d: mov qword ptr [rsp + 0x20], 1
140a219a6: mov r8d, 0x88
140a219ac: mov r9d, 1
140a219b2: mov rcx, rdi
140a219b5: mov rdx, r14
140a219b8: call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140a219bd: jmp 0x140a218e8
140a219c2: nop word ptr cs:[rax + rax]
140a219d0: mov qword ptr [rsp + 0x10], rdx
140a219d5: push rbp
140a219d6: push r15
140a219d8: push r14
140a219da: push r13
140a219dc: push r12
140a219de: push rsi
140a219df: push rdi
140a219e0: push rbx
140a219e1: sub rsp, 0x48
140a219e5: lea rbp, [rdx + 0x80]
140a219ec: movaps xmmword ptr [rsp + 0x30], xmm6
140a219f1: mov rdx, qword ptr [rbp + 0x48]
140a219f5: test rdx, rdx
140a219f8: je 0x140a21a0d
140a219fa: mov rcx, qword ptr [rbp + 0x50]
140a219fe: shl rdx, 2
140a21a02: mov r8d, 4
140a21a08: call 0x140613c20
140a21a0d: movaps xmm6, xmmword ptr [rsp + 0x30]
140a21a12: add rsp, 0x48
140a21a16: pop rbx
140a21a17: pop rdi
140a21a18: pop rsi
140a21a19: pop r12
140a21a1b: pop r13
140a21a1d: pop r14
140a21a1f: pop r15
140a21a21: pop rbp
140a21a22: ret 
