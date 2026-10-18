0x140161cd0 55 push rbp
0x140161cd1 4157 push r15
0x140161cd3 4156 push r14
0x140161cd5 4155 push r13
0x140161cd7 4154 push r12
0x140161cd9 56 push rsi
0x140161cda 57 push rdi
0x140161cdb 53 push rbx
0x140161cdc b8381b0000 mov eax, 0x1b38
0x140161ce1 e85a8d7302 call 0x14289aa40 ; __chkstk
0x140161ce6 4829c4 sub rsp, rax
0x140161ce9 488dac2480000000 lea rbp, [rsp + 0x80]
0x140161cf1 0f29b5a01a0000 movaps xmmword ptr [rbp + 0x1aa0], xmm6
0x140161cf8 48c785981a0000feffffff mov qword ptr [rbp + 0x1a98], 0xfffffffffffffffe
0x140161d03 4d89c4 mov r12, r8
0x140161d06 4889d6 mov rsi, rdx
0x140161d09 488b02 mov rax, qword ptr [rdx]
0x140161d0c 49b80000000000000080 movabs r8, 0x8000000000000000
0x140161d16 4c8985081a0000 mov qword ptr [rbp + 0x1a08], r8
0x140161d1d 4c8902 mov qword ptr [rdx], r8
0x140161d20 4889c2 mov rdx, rax
0x140161d23 48f7da neg rdx
0x140161d26 0f8031230000 jo 0x14016405d
0x140161d2c 4c898d98170000 mov qword ptr [rbp + 0x1798], r9
0x140161d33 48898d90170000 mov qword ptr [rbp + 0x1790], rcx
0x140161d3a 488d5608 lea rdx, [rsi + 8]
0x140161d3e 48898598110000 mov qword ptr [rbp + 0x1198], rax
0x140161d45 488d8da0110000 lea rcx, [rbp + 0x11a0]
0x140161d4c 41b888000000 mov r8d, 0x88
0x140161d52 e8a91e7c02 call 0x142923c00 ; memcpy
0x140161d57 4c8bb690000000 mov r14, qword ptr [rsi + 0x90]
0x140161d5e 48c7869000000000000000 mov qword ptr [rsi + 0x90], 0
0x140161d69 4d85f6 test r14, r14
0x140161d6c 4c89b5a8170000 mov qword ptr [rbp + 0x17a8], r14
0x140161d73 0f84f0220000 je 0x140164069
0x140161d79 488b9d201b0000 mov rbx, qword ptr [rbp + 0x1b20]
0x140161d80 488b86e8000000 mov rax, qword ptr [rsi + 0xe8]
0x140161d87 48898530130000 mov qword ptr [rbp + 0x1330], rax
0x140161d8e 0f1086d8000000 movups xmm0, xmmword ptr [rsi + 0xd8]
0x140161d95 0f118520130000 movups xmmword ptr [rbp + 0x1320], xmm0
0x140161d9c 0f108698000000 movups xmm0, xmmword ptr [rsi + 0x98]
0x140161da3 0f108ea8000000 movups xmm1, xmmword ptr [rsi + 0xa8]
0x140161daa 0f1096b8000000 movups xmm2, xmmword ptr [rsi + 0xb8]
0x140161db1 4889b5b0190000 mov qword ptr [rbp + 0x19b0], rsi
0x140161db8 0f109ec8000000 movups xmm3, xmmword ptr [rsi + 0xc8]
0x140161dbf 0f119d10130000 movups xmmword ptr [rbp + 0x1310], xmm3
0x140161dc6 0f119500130000 movups xmmword ptr [rbp + 0x1300], xmm2
0x140161dcd 0f118df0120000 movups xmmword ptr [rbp + 0x12f0], xmm1
0x140161dd4 0f1185e0120000 movups xmmword ptr [rbp + 0x12e0], xmm0
0x140161ddb 488d8d28120000 lea rcx, [rbp + 0x1228]
0x140161de2 488d9598110000 lea rdx, [rbp + 0x1198]
0x140161de9 41b890000000 mov r8d, 0x90
0x140161def e80c1e7c02 call 0x142923c00 ; memcpy
0x140161df4 4c8b3b mov r15, qword ptr [rbx]
0x140161df7 488b7308 mov rsi, qword ptr [rbx + 8]
0x140161dfb 488b4310 mov rax, qword ptr [rbx + 0x10]
0x140161dff 488985481a0000 mov qword ptr [rbp + 0x1a48], rax
0x140161e06 488b7b18 mov rdi, qword ptr [rbx + 0x18]
0x140161e0a 488b4320 mov rax, qword ptr [rbx + 0x20]
0x140161e0e 488985001a0000 mov qword ptr [rbp + 0x1a00], rax
0x140161e15 488b4328 mov rax, qword ptr [rbx + 0x28]
0x140161e19 488985f0190000 mov qword ptr [rbp + 0x19f0], rax
0x140161e20 488b4330 mov rax, qword ptr [rbx + 0x30]
0x140161e24 48898580170000 mov qword ptr [rbp + 0x1780], rax
0x140161e2b 488b4338 mov rax, qword ptr [rbx + 0x38]
0x140161e2f 488985b8190000 mov qword ptr [rbp + 0x19b8], rax
0x140161e36 48c785b812000000000000 mov qword ptr [rbp + 0x12b8], 0
0x140161e41 48c785c012000008000000 mov qword ptr [rbp + 0x12c0], 8
0x140161e4c 48c785c812000000000000 mov qword ptr [rbp + 0x12c8], 0
0x140161e57 4c89a588170000 mov qword ptr [rbp + 0x1788], r12
0x140161e5e 4c89a5d0120000 mov qword ptr [rbp + 0x12d0], r12
0x140161e65 4c89b5d8120000 mov qword ptr [rbp + 0x12d8], r14
0x140161e6c 4d8d6738 lea r12, [r15 + 0x38]
0x140161e70 c685931a000001 mov byte ptr [rbp + 0x1a93], 1
0x140161e77 48baaa4e7896b36185a6 movabs rdx, 0xa68561b396784eaa
0x140161e81 49b814edf4fae674fdf5 movabs r8, 0xf5fd74e6faf4ed14
0x140161e8b 4c89e1 mov rcx, r12
0x140161e8e e85db8f7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x140161e93 4885c0 test rax, rax
0x140161e96 0f8472200000 je 0x140163f0e
0x140161e9c 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x140161ea3 0f8665200000 jbe 0x140163f0e
0x140161ea9 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x140161eb0 488b04d0 mov rax, qword ptr [rax + rdx*8]
0x140161eb4 4885c0 test rax, rax
0x140161eb7 0f8451200000 je 0x140163f0e
0x140161ebd 4d8bafe0010000 mov r13, qword ptr [r15 + 0x1e0]
0x140161ec4 48f7d0 not rax
0x140161ec7 488d0440 lea rax, [rax + rax*2]
0x140161ecb 48c1e005 shl rax, 5
0x140161ecf 49837c052000 cmp qword ptr [r13 + rax + 0x20], 0
0x140161ed5 0f8433200000 je 0x140163f0e
0x140161edb 4901c5 add r13, rax
0x140161ede 498b5d10 mov rbx, qword ptr [r13 + 0x10]
0x140161ee2 458bb7b0050000 mov r14d, dword ptr [r15 + 0x5b0]
0x140161ee9 4589755c mov dword ptr [r13 + 0x5c], r14d
0x140161eed c685931a000001 mov byte ptr [rbp + 0x1a93], 1
0x140161ef4 488d9528120000 lea rdx, [rbp + 0x1228]
0x140161efb 4889d9 mov rcx, rbx
0x140161efe e82dae7d00 call 0x14093cd30 ; country_core::asset_libraries::texture_library::AssetTextureLibrary::import_texture_presets_to_rg
0x140161f03 4589755c mov dword ptr [r13 + 0x5c], r14d
0x140161f07 c685931a000001 mov byte ptr [rbp + 0x1a93], 1
0x140161f0e 4889d9 mov rcx, rbx
0x140161f11 4889f2 mov rdx, rsi
0x140161f14 e827b17d00 call 0x14093d040 ; country_core::asset_libraries::texture_library::AssetTextureLibrary::bindless_textures_dset
0x140161f19 c685931a000001 mov byte ptr [rbp + 0x1a93], 1
0x140161f20 488d8d68150000 lea rcx, [rbp + 0x1568]
0x140161f27 4889c2 mov rdx, rax
0x140161f2a e8d14ba700 call 0x140bd6b00 ; rhapsody::raw::managed::handle::impl$5::clone
0x140161f2f c685941a000001 mov byte ptr [rbp + 0x1a94], 1
0x140161f36 48baefa5998645782966 movabs rdx, 0x662978458699a5ef
0x140161f40 49b8469b4162c157c03e movabs r8, 0x3ec057c162419b46
0x140161f4a 4c89e1 mov rcx, r12
0x140161f4d e89eb7f7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x140161f52 4885c0 test rax, rax
0x140161f55 0f84cb1f0000 je 0x140163f26
0x140161f5b 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x140161f62 0f86be1f0000 jbe 0x140163f26
0x140161f68 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x140161f6f 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140161f73 4885c9 test rcx, rcx
0x140161f76 0f84aa1f0000 je 0x140163f26
0x140161f7c 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x140161f83 48f7d1 not rcx
0x140161f86 488d0c49 lea rcx, [rcx + rcx*2]
0x140161f8a 48c1e105 shl rcx, 5
0x140161f8e 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x140161f94 0f848c1f0000 je 0x140163f26
0x140161f9a 4989fe mov r14, rdi
0x140161f9d 4801c8 add rax, rcx
0x140161fa0 488b4810 mov rcx, qword ptr [rax + 0x10]
0x140161fa4 418b97b0050000 mov edx, dword ptr [r15 + 0x5b0]
0x140161fab 89505c mov dword ptr [rax + 0x5c], edx
0x140161fae c685941a000001 mov byte ptr [rbp + 0x1a94], 1
0x140161fb5 488d9528120000 lea rdx, [rbp + 0x1228]
0x140161fbc e8ff018300 call 0x1409921c0 ; country_core::render::nani::gpu_world::GpuWorld::import_to_rg
0x140161fc1 c685941a000001 mov byte ptr [rbp + 0x1a94], 1
0x140161fc8 48baca3b97c719dcd98f movabs rdx, 0x8fd9dc19c7973bca
0x140161fd2 49b8fd459bebc5f41f36 movabs r8, 0x361ff4c5eb9b45fd
0x140161fdc 4c89e1 mov rcx, r12
0x140161fdf e80cb7f7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x140161fe4 4885c0 test rax, rax
0x140161fe7 0f84511f0000 je 0x140163f3e
0x140161fed 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x140161ff4 0f86441f0000 jbe 0x140163f3e
0x140161ffa 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x140162001 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140162005 4885c9 test rcx, rcx
0x140162008 0f84301f0000 je 0x140163f3e
0x14016200e 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x140162015 48f7d1 not rcx
0x140162018 488d0c49 lea rcx, [rcx + rcx*2]
0x14016201c 48c1e105 shl rcx, 5
0x140162020 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x140162026 0f84121f0000 je 0x140163f3e
0x14016202c 4801c8 add rax, rcx
0x14016202f 488b5010 mov rdx, qword ptr [rax + 0x10]
0x140162033 488d8de0000000 lea rcx, [rbp + 0xe0]
0x14016203a 41b890100000 mov r8d, 0x1090
0x140162040 e8bb1b7c02 call 0x142923c00 ; memcpy
0x140162045 c685941a000001 mov byte ptr [rbp + 0x1a94], 1
0x14016204c 48ba2ea78382b74cc947 movabs rdx, 0x47c94cb78283a72e
0x140162056 49b89d3e3297ed3d178c movabs r8, 0x8c173ded97323e9d
0x140162060 4c89e1 mov rcx, r12
0x140162063 e888b6f7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x140162068 4885c0 test rax, rax
0x14016206b 0f84e51e0000 je 0x140163f56
0x140162071 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x140162078 0f86d81e0000 jbe 0x140163f56
0x14016207e 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x140162085 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140162089 4885c9 test rcx, rcx
0x14016208c 0f84c41e0000 je 0x140163f56
0x140162092 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x140162099 48f7d1 not rcx
0x14016209c 488d0c49 lea rcx, [rcx + rcx*2]
0x1401620a0 48c1e105 shl rcx, 5
0x1401620a4 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1401620aa 0f84a61e0000 je 0x140163f56
0x1401620b0 4801c8 add rax, rcx
0x1401620b3 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1401620b7 c685941a000001 mov byte ptr [rbp + 0x1a94], 1
0x1401620be e8bdbc5100 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x1401620c3 4889c7 mov rdi, rax
0x1401620c6 b101 mov cl, 1
0x1401620c8 31c0 xor eax, eax
0x1401620ca f00fb00f lock cmpxchg byte ptr [rdi], cl
0x1401620ce 0f85bc1d0000 jne 0x140163e90
0x1401620d4 c685941a000001 mov byte ptr [rbp + 0x1a94], 1
0x1401620db 4889f9 mov rcx, rdi
0x1401620de e88dfbb100 call 0x140c81c70 ; parking_lot_core::parking_lot::deadlock_impl::acquire_resource
0x1401620e3 4889bd801a0000 mov qword ptr [rbp + 0x1a80], rdi
0x1401620ea 4883c708 add rdi, 8
0x1401620ee 48c744242000000000 mov qword ptr [rsp + 0x20], 0
0x1401620f7 488d8d10180000 lea rcx, [rbp + 0x1810]
0x1401620fe 41b990100000 mov r9d, 0x1090
0x140162104 4889fa mov rdx, rdi
0x140162107 4989f0 mov r8, rsi
0x14016210a e83106aa00 call 0x140c02740 ; rhapsody::raw::dynamic_constants::DynamicConstants::allocate
0x14016210f 488b8d18180000 mov rcx, qword ptr [rbp + 0x1818]
0x140162116 4881f990100000 cmp rcx, 0x1090
0x14016211d 0f85871d0000 jne 0x140163eaa
0x140162123 488b8d10180000 mov rcx, qword ptr [rbp + 0x1810]
0x14016212a 488d95e0000000 lea rdx, [rbp + 0xe0]
0x140162131 41b890100000 mov r8d, 0x1090
0x140162137 e8c41a7c02 call 0x142923c00 ; memcpy
0x14016213c 0f108510180000 movups xmm0, xmmword ptr [rbp + 0x1810]
0x140162143 0f108d20180000 movups xmm1, xmmword ptr [rbp + 0x1820]
0x14016214a 0f109530180000 movups xmm2, xmmword ptr [rbp + 0x1830]
0x140162151 0f109d40180000 movups xmm3, xmmword ptr [rbp + 0x1840]
0x140162158 0f2985e0150000 movaps xmmword ptr [rbp + 0x15e0], xmm0
0x14016215f 0f298df0150000 movaps xmmword ptr [rbp + 0x15f0], xmm1
0x140162166 0f299500160000 movaps xmmword ptr [rbp + 0x1600], xmm2
0x14016216d 0f299d10160000 movaps xmmword ptr [rbp + 0x1610], xmm3
0x140162174 488b8550180000 mov rax, qword ptr [rbp + 0x1850]
0x14016217b 48898520160000 mov qword ptr [rbp + 0x1620], rax
0x140162182 488d8d38130000 lea rcx, [rbp + 0x1338]
0x140162189 488d95e0150000 lea rdx, [rbp + 0x15e0]
0x140162190 4989f0 mov r8, rsi
0x140162193 e8d801aa00 call 0x140c02370 ; rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_and_storage_buffer_binding
0x140162198 c6858e1a000001 mov byte ptr [rbp + 0x1a8e], 1
0x14016219f 488bbd801a0000 mov rdi, qword ptr [rbp + 0x1a80]
0x1401621a6 4889f9 mov rcx, rdi
0x1401621a9 e862fcb100 call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
0x1401621ae 31c9 xor ecx, ecx
0x1401621b0 b001 mov al, 1
0x1401621b2 f00fb00f lock cmpxchg byte ptr [rdi], cl
0x1401621b6 0f85041d0000 jne 0x140163ec0
0x1401621bc 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x1401621c3 488d9528120000 lea rdx, [rbp + 0x1228]
0x1401621ca 41b810010000 mov r8d, 0x110
0x1401621d0 e82b1a7c02 call 0x142923c00 ; memcpy
0x1401621d5 0f108538130000 movups xmm0, xmmword ptr [rbp + 0x1338]
0x1401621dc 0f108d48130000 movups xmm1, xmmword ptr [rbp + 0x1348]
0x1401621e3 0f109558130000 movups xmm2, xmmword ptr [rbp + 0x1358]
0x1401621ea 0f109d68130000 movups xmm3, xmmword ptr [rbp + 0x1368]
0x1401621f1 0f1185f0160000 movups xmmword ptr [rbp + 0x16f0], xmm0
0x1401621f8 0f118d00170000 movups xmmword ptr [rbp + 0x1700], xmm1
0x1401621ff 0f119510170000 movups xmmword ptr [rbp + 0x1710], xmm2
0x140162206 0f119d20170000 movups xmmword ptr [rbp + 0x1720], xmm3
0x14016220d 0f108578130000 movups xmm0, xmmword ptr [rbp + 0x1378]
0x140162214 0f118530170000 movups xmmword ptr [rbp + 0x1730], xmm0
0x14016221b 0f108588130000 movups xmm0, xmmword ptr [rbp + 0x1388]
0x140162222 0f118540170000 movups xmmword ptr [rbp + 0x1740], xmm0
0x140162229 0f108568150000 movups xmm0, xmmword ptr [rbp + 0x1568]
0x140162230 0f118550170000 movups xmmword ptr [rbp + 0x1750], xmm0
0x140162237 488b8578150000 mov rax, qword ptr [rbp + 0x1578]
0x14016223e 48898560170000 mov qword ptr [rbp + 0x1760], rax
0x140162245 488b85481a0000 mov rax, qword ptr [rbp + 0x1a48]
0x14016224c 48898568170000 mov qword ptr [rbp + 0x1768], rax
0x140162253 4889b570170000 mov qword ptr [rbp + 0x1770], rsi
0x14016225a c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162261 48ba5a46f243f4452d16 movabs rdx, 0x162d45f443f2465a
0x14016226b 49b80ffc085559d9b07d movabs r8, 0x7db0d9595508fc0f
0x140162275 4c89e1 mov rcx, r12
0x140162278 e873b4f7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x14016227d 4885c0 test rax, rax
0x140162280 0f84e81c0000 je 0x140163f6e
0x140162286 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x14016228d 0f86db1c0000 jbe 0x140163f6e
0x140162293 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x14016229a 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x14016229e 4885c9 test rcx, rcx
0x1401622a1 0f84c71c0000 je 0x140163f6e
0x1401622a7 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x1401622ae 48f7d1 not rcx
0x1401622b1 488d0c49 lea rcx, [rcx + rcx*2]
0x1401622b5 48c1e105 shl rcx, 5
0x1401622b9 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1401622bf 0f84a91c0000 je 0x140163f6e
0x1401622c5 4801c8 add rax, rcx
0x1401622c8 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1401622cc 418b97b0050000 mov edx, dword ptr [r15 + 0x5b0]
0x1401622d3 89505c mov dword ptr [rax + 0x5c], edx
0x1401622d6 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x1401622dd 488d95e0150000 lea rdx, [rbp + 0x15e0]
0x1401622e4 4d89f8 mov r8, r15
0x1401622e7 e894bf7a00 call 0x14090e280 ; country_core::resources::gpu_lut::GpuLutTracker::render_luts
0x1401622ec c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x1401622f3 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x1401622fa 4c89fa mov rdx, r15
0x1401622fd e86e857a00 call 0x14090a870 ; country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args
0x140162302 498b16 mov rdx, qword ptr [r14]
0x140162305 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x14016230c 488d8d10180000 lea rcx, [rbp + 0x1810]
0x140162313 41b87a000000 mov r8d, 0x7a
0x140162319 e8822fa700 call 0x140bd52a0 ; rhapsody::raw::texture::TextureDesc::new_2d
0x14016231e c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162325 4c8dad101a0000 lea r13, [rbp + 0x1a10]
0x14016232c 488d9510180000 lea rdx, [rbp + 0x1810]
0x140162333 4c89e9 mov rcx, r13
0x140162336 41b006 mov r8b, 6
0x140162339 e82231a700 call 0x140bd5460 ; rhapsody::raw::texture::TextureDesc::mip_levels
0x14016233e 0f1085101a0000 movups xmm0, xmmword ptr [rbp + 0x1a10]
0x140162345 0f108d201a0000 movups xmm1, xmmword ptr [rbp + 0x1a20]
0x14016234c 0f298d60140000 movaps xmmword ptr [rbp + 0x1460], xmm1
0x140162353 0f298550140000 movaps xmmword ptr [rbp + 0x1450], xmm0
0x14016235a 488d8d20180000 lea rcx, [rbp + 0x1820]
0x140162361 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162368 488d9550140000 lea rdx, [rbp + 0x1450]
0x14016236f e86c49a700 call 0x140bd6ce0 ; rhapsody::rg::resource::impl$4::from
0x140162374 488d052db37f02 lea rax, [rip + 0x27fb32d]
0x14016237b 48898510180000 mov qword ptr [rbp + 0x1810], rax
0x140162382 48c785181800000a000000 mov qword ptr [rbp + 0x1818], 0xa
0x14016238d c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162394 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x14016239b 488d9510180000 lea rdx, [rbp + 0x1810]
0x1401623a2 e82971a800 call 0x140be94d0 ; rhapsody::rg::graph::RenderGraph::create_raw_resource
0x1401623a7 0f1085101a0000 movups xmm0, xmmword ptr [rbp + 0x1a10]
0x1401623ae 0f108d201a0000 movups xmm1, xmmword ptr [rbp + 0x1a20]
0x1401623b5 0f1185c4190000 movups xmmword ptr [rbp + 0x19c4], xmm0
0x1401623bc 0f118dd4190000 movups xmmword ptr [rbp + 0x19d4], xmm1
0x1401623c3 8985c0190000 mov dword ptr [rbp + 0x19c0], eax
0x1401623c9 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x1401623d0 48ba7a9cb77f61ae8dd3 movabs rdx, 0xd38dae617fb79c7a
0x1401623da 49b8ae1ead5d5cdfaa22 movabs r8, 0x22aadf5c5dad1eae
0x1401623e4 4c89e1 mov rcx, r12
0x1401623e7 e804b3f7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1401623ec 4885c0 test rax, rax
0x1401623ef 0f84911b0000 je 0x140163f86
0x1401623f5 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x1401623fc 0f86841b0000 jbe 0x140163f86
0x140162402 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x140162409 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x14016240d 4885c9 test rcx, rcx
0x140162410 0f84701b0000 je 0x140163f86
0x140162416 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x14016241d 48f7d1 not rcx
0x140162420 488d0c49 lea rcx, [rcx + rcx*2]
0x140162424 48c1e105 shl rcx, 5
0x140162428 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x14016242e 0f84521b0000 je 0x140163f86
0x140162434 4801c8 add rax, rcx
0x140162437 488b7010 mov rsi, qword ptr [rax + 0x10]
0x14016243b c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162442 48bae978f5b6455c3835 movabs rdx, 0x35385c45b6f578e9
0x14016244c 49b89f0cddd5813d62f1 movabs r8, 0xf1623d81d5dd0c9f
0x140162456 4c89e1 mov rcx, r12
0x140162459 e892b2f7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x14016245e 4885c0 test rax, rax
0x140162461 0f84371b0000 je 0x140163f9e
0x140162467 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x14016246e 0f862a1b0000 jbe 0x140163f9e
0x140162474 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x14016247b 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x14016247f 4885c9 test rcx, rcx
0x140162482 0f84161b0000 je 0x140163f9e
0x140162488 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x14016248f 48f7d1 not rcx
0x140162492 488d0c49 lea rcx, [rcx + rcx*2]
0x140162496 48c1e105 shl rcx, 5
0x14016249a 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1401624a0 0f84f81a0000 je 0x140163f9e
0x1401624a6 4801c8 add rax, rcx
0x1401624a9 488b4010 mov rax, qword ptr [rax + 0x10]
0x1401624ad 803801 cmp byte ptr [rax], 1
0x1401624b0 0f85fe020000 jne 0x1401627b4
0x1401624b6 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x1401624bd 4889f1 mov rcx, rsi
0x1401624c0 e85bd77d00 call 0x14093fc20 ; country_core::resources::gui::EscMenuState::should_show_world
0x1401624c5 84c0 test al, al
0x1401624c7 0f84e7020000 je 0x1401627b4
0x1401624cd c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x1401624d4 48ba7a9cb77f61ae8dd3 movabs rdx, 0xd38dae617fb79c7a
0x1401624de 49b8ae1ead5d5cdfaa22 movabs r8, 0x22aadf5c5dad1eae
0x1401624e8 4c89e1 mov rcx, r12
0x1401624eb e800b2f7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1401624f0 4885c0 test rax, rax
0x1401624f3 0f847e1b0000 je 0x140164077
0x1401624f9 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x140162500 0f86711b0000 jbe 0x140164077
0x140162506 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x14016250d 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140162511 4885c9 test rcx, rcx
0x140162514 0f845d1b0000 je 0x140164077
0x14016251a 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x140162521 48f7d1 not rcx
0x140162524 488d0c49 lea rcx, [rcx + rcx*2]
0x140162528 48c1e105 shl rcx, 5
0x14016252c 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x140162532 0f843f1b0000 je 0x140164077
0x140162538 4801c8 add rax, rcx
0x14016253b 488b4810 mov rcx, qword ptr [rax + 0x10]
0x14016253f c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162546 e875d67d00 call 0x14093fbc0 ; country_core::resources::gui::EscMenuState::is_open
0x14016254b 8985481a0000 mov dword ptr [rbp + 0x1a48], eax
0x140162551 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162558 4889f1 mov rcx, rsi
0x14016255b e880d67d00 call 0x14093fbe0 ; country_core::resources::gui::EscMenuState::is_closed
0x140162560 84c0 test al, al
0x140162562 0f85d8020000 jne 0x140162840
0x140162568 80bd481a000000 cmp byte ptr [rbp + 0x1a48], 0
0x14016256f 0f849f000000 je 0x140162614
0x140162575 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x14016257c 48ba606cbabdc9d9181f movabs rdx, 0x1f18d9c9bdba6c60
0x140162586 49b8670be94866984785 movabs r8, 0x8547986648e90b67
0x140162590 4c89e1 mov rcx, r12
0x140162593 e858b1f7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x140162598 4885c0 test rax, rax
0x14016259b 0f840e1b0000 je 0x1401640af
0x1401625a1 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x1401625a8 0f86011b0000 jbe 0x1401640af
0x1401625ae 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x1401625b5 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1401625b9 4885c9 test rcx, rcx
0x1401625bc 0f84ed1a0000 je 0x1401640af
0x1401625c2 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x1401625c9 48f7d1 not rcx
0x1401625cc 488d0c49 lea rcx, [rcx + rcx*2]
0x1401625d0 48c1e105 shl rcx, 5
0x1401625d4 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1401625da 0f84cf1a0000 je 0x1401640af
0x1401625e0 4801c8 add rax, rcx
0x1401625e3 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1401625e7 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x1401625ee e8dd3d7400 call 0x1408a63d0 ; country_core::systems::render_loop::painterly::PainterlyState::has_target
0x1401625f3 488d0d1eb17f02 lea rcx, [rip + 0x27fb11e]
0x1401625fa 48898d101a0000 mov qword ptr [rbp + 0x1a10], rcx
0x140162601 48c785181a000018000000 mov qword ptr [rbp + 0x1a18], 0x18
0x14016260c 84c0 test al, al
0x14016260e 0f847b170000 je 0x140163d8f
0x140162614 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x14016261b 48baca3b97c719dcd98f movabs rdx, 0x8fd9dc19c7973bca
0x140162625 49b8fd459bebc5f41f36 movabs r8, 0x361ff4c5eb9b45fd
0x14016262f 4c89e1 mov rcx, r12
0x140162632 e8b9b0f7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x140162637 4885c0 test rax, rax
0x14016263a 0f845a1a0000 je 0x14016409a
0x140162640 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x140162647 0f864d1a0000 jbe 0x14016409a
0x14016264d 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x140162654 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140162658 4885c9 test rcx, rcx
0x14016265b 0f84391a0000 je 0x14016409a
0x140162661 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x140162668 48f7d1 not rcx
0x14016266b 488d0c49 lea rcx, [rcx + rcx*2]
0x14016266f 48c1e105 shl rcx, 5
0x140162673 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x140162679 0f841b1a0000 je 0x14016409a
0x14016267f 4801c8 add rax, rcx
0x140162682 488b4810 mov rcx, qword ptr [rax + 0x10]
0x140162686 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x14016268d e8ee418100 call 0x140976880 ; country_core::render::frame_constants::FrameConstants::should_reset_temporal
0x140162692 488d0dafb07f02 lea rcx, [rip + 0x27fb0af]
0x140162699 48898d101a0000 mov qword ptr [rbp + 0x1a10], rcx
0x1401626a0 48c785181a000014000000 mov qword ptr [rbp + 0x1a18], 0x14
0x1401626ab 84c0 test al, al
0x1401626ad 0f84ab150000 je 0x140163c5e
0x1401626b3 488b05ee9c4c03 mov rax, qword ptr [rip + 0x34c9cee]
0x1401626ba 488b00 mov rax, qword ptr [rax]
0x1401626bd 4883f805 cmp rax, 5
0x1401626c1 0f8279010000 jb 0x140162840
0x1401626c7 4c89ad50140000 mov qword ptr [rbp + 0x1450], r13
0x1401626ce 488d05bb77faff lea rax, [rip - 0x58845]
0x1401626d5 48898558140000 mov qword ptr [rbp + 0x1458], rax
0x1401626dc c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x1401626e3 488d0d6e948802 lea rcx, [rip + 0x288946e]
0x1401626ea e891b65100 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x1401626ef 0f1000 movups xmm0, xmmword ptr [rax]
0x1401626f2 8b4010 mov eax, dword ptr [rax + 0x10]
0x1401626f5 48c7854018000005000000 mov qword ptr [rbp + 0x1840], 5
0x140162700 488d0d69948802 lea rcx, [rip + 0x2889469]
0x140162707 48898d48180000 mov qword ptr [rbp + 0x1848], rcx
0x14016270e 48c7855018000017000000 mov qword ptr [rbp + 0x1850], 0x17
0x140162719 488d1500948802 lea rdx, [rip + 0x2889400]
0x140162720 48899560180000 mov qword ptr [rbp + 0x1860], rdx
0x140162727 48c7856818000001000000 mov qword ptr [rbp + 0x1868], 1
0x140162732 488d9550140000 lea rdx, [rbp + 0x1450]
0x140162739 48899570180000 mov qword ptr [rbp + 0x1870], rdx
0x140162740 48c7857818000001000000 mov qword ptr [rbp + 0x1878], 1
0x14016274b 48c7858018000000000000 mov qword ptr [rbp + 0x1880], 0
0x140162756 48c7851018000000000000 mov qword ptr [rbp + 0x1810], 0
0x140162761 48898d18180000 mov qword ptr [rbp + 0x1818], rcx
0x140162768 48c7852018000017000000 mov qword ptr [rbp + 0x1820], 0x17
0x140162773 48c7852818000000000000 mov qword ptr [rbp + 0x1828], 0
0x14016277e 0f118530180000 movups xmmword ptr [rbp + 0x1830], xmm0
0x140162785 c7855818000001000000 mov dword ptr [rbp + 0x1858], 1
0x14016278f 89855c180000 mov dword ptr [rbp + 0x185c], eax
0x140162795 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x14016279c 488d8d501a0000 lea rcx, [rbp + 0x1a50]
0x1401627a3 488d9510180000 lea rdx, [rbp + 0x1810]
0x1401627aa e8817bbe00 call 0x140d4a330 ; log::__private_api::impl$2::log
0x1401627af e98c000000 jmp 0x140162840
0x1401627b4 c785481a000000000000 mov dword ptr [rbp + 0x1a48], 0
0x1401627be c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x1401627c5 48ba5951918cf9e83168 movabs rdx, 0x6831e8f98c915159
0x1401627cf 49b8250700e479c415d5 movabs r8, 0xd515c479e4000725
0x1401627d9 4c89e1 mov rcx, r12
0x1401627dc e80faff7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1401627e1 4885c0 test rax, rax
0x1401627e4 0f845e180000 je 0x140164048
0x1401627ea 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x1401627f1 0f8651180000 jbe 0x140164048
0x1401627f7 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x1401627fe 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140162802 4885c9 test rcx, rcx
0x140162805 0f843d180000 je 0x140164048
0x14016280b 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x140162812 48f7d1 not rcx
0x140162815 488d0c49 lea rcx, [rcx + rcx*2]
0x140162819 48c1e105 shl rcx, 5
0x14016281d 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x140162823 0f841f180000 je 0x140164048
0x140162829 4801c8 add rax, rcx
0x14016282c 488b4810 mov rcx, qword ptr [rax + 0x10]
0x140162830 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162837 e8b4c87b00 call 0x14091f0f0 ; country_core::meta_commander::MetaCommander::is_profiling
0x14016283c 84c0 test al, al
0x14016283e 7434 je 0x140162874
0x140162840 803d49234d0301 cmp byte ptr [rip + 0x34d2349], 1
0x140162847 0f8596010000 jne 0x1401629e3
0x14016284d c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162854 b001 mov al, 1
0x140162856 8985801a0000 mov dword ptr [rbp + 0x1a80], eax
0x14016285c 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x140162863 4c8d85c0190000 lea r8, [rbp + 0x19c0]
0x14016286a 4c89fa mov rdx, r15
0x14016286d e82ec48500 call 0x1409beca0 ; country_core::systems::render_loop::compute_passes::primary_rt_debug::primary_rt_debug
0x140162872 eb0a jmp 0x14016287e
0x140162874 c785801a000000000000 mov dword ptr [rbp + 0x1a80], 0
0x14016287e b307 mov bl, 7
0x140162880 498b16 mov rdx, qword ptr [r14]
0x140162883 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x14016288a 488d8d101a0000 lea rcx, [rbp + 0x1a10]
0x140162891 41b882000000 mov r8d, 0x82
0x140162897 e8042aa700 call 0x140bd52a0 ; rhapsody::raw::texture::TextureDesc::new_2d
0x14016289c 0f1085101a0000 movups xmm0, xmmword ptr [rbp + 0x1a10]
0x1401628a3 0f108d201a0000 movups xmm1, xmmword ptr [rbp + 0x1a20]
0x1401628aa 0f298d60140000 movaps xmmword ptr [rbp + 0x1460], xmm1
0x1401628b1 0f298550140000 movaps xmmword ptr [rbp + 0x1450], xmm0
0x1401628b8 488d8d20180000 lea rcx, [rbp + 0x1820]
0x1401628bf c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x1401628c6 488d9550140000 lea rdx, [rbp + 0x1450]
0x1401628cd e80e44a700 call 0x140bd6ce0 ; rhapsody::rg::resource::impl$4::from
0x1401628d2 488d05bfae7f02 lea rax, [rip + 0x27faebf]
0x1401628d9 48898510180000 mov qword ptr [rbp + 0x1810], rax
0x1401628e0 48c785181800000a000000 mov qword ptr [rbp + 0x1818], 0xa
0x1401628eb c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x1401628f2 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x1401628f9 488d9510180000 lea rdx, [rbp + 0x1810]
0x140162900 e8cb6ba800 call 0x140be94d0 ; rhapsody::rg::graph::RenderGraph::create_raw_resource
0x140162905 0f1085101a0000 movups xmm0, xmmword ptr [rbp + 0x1a10]
0x14016290c 0f108d201a0000 movups xmm1, xmmword ptr [rbp + 0x1a20]
0x140162913 0f1185e4170000 movups xmmword ptr [rbp + 0x17e4], xmm0
0x14016291a 0f118df4170000 movups xmmword ptr [rbp + 0x17f4], xmm1
0x140162921 8985e0170000 mov dword ptr [rbp + 0x17e0], eax
0x140162927 80fb07 cmp bl, 7
0x14016292a 0f85bd020000 jne 0x140162bed
0x140162930 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162937 488d8d101a0000 lea rcx, [rbp + 0x1a10]
0x14016293e 48ba0100000001000000 movabs rdx, 0x100000001
0x140162948 41b809000000 mov r8d, 9
0x14016294e e84d29a700 call 0x140bd52a0 ; rhapsody::raw::texture::TextureDesc::new_2d
0x140162953 0f1085101a0000 movups xmm0, xmmword ptr [rbp + 0x1a10]
0x14016295a 0f108d201a0000 movups xmm1, xmmword ptr [rbp + 0x1a20]
0x140162961 0f298d60140000 movaps xmmword ptr [rbp + 0x1460], xmm1
0x140162968 0f298550140000 movaps xmmword ptr [rbp + 0x1450], xmm0
0x14016296f 488d8d20180000 lea rcx, [rbp + 0x1820]
0x140162976 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x14016297d 488d9550140000 lea rdx, [rbp + 0x1450]
0x140162984 e85743a700 call 0x140bd6ce0 ; rhapsody::rg::resource::impl$4::from
0x140162989 488d0518af7f02 lea rax, [rip + 0x27faf18]
0x140162990 48898510180000 mov qword ptr [rbp + 0x1810], rax
0x140162997 48c7851818000003000000 mov qword ptr [rbp + 0x1818], 3
0x1401629a2 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x1401629a9 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x1401629b0 488d9510180000 lea rdx, [rbp + 0x1810]
0x1401629b7 e8146ba800 call 0x140be94d0 ; rhapsody::rg::graph::RenderGraph::create_raw_resource
0x1401629bc 0f1085101a0000 movups xmm0, xmmword ptr [rbp + 0x1a10]
0x1401629c3 0f108d201a0000 movups xmm1, xmmword ptr [rbp + 0x1a20]
0x1401629ca 0f1185b4170000 movups xmmword ptr [rbp + 0x17b4], xmm0
0x1401629d1 0f118dc4170000 movups xmmword ptr [rbp + 0x17c4], xmm1
0x1401629d8 8985b0170000 mov dword ptr [rbp + 0x17b0], eax
0x1401629de e941020000 jmp 0x140162c24
0x1401629e3 498b16 mov rdx, qword ptr [r14]
0x1401629e6 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x1401629ed 488d8d101a0000 lea rcx, [rbp + 0x1a10]
0x1401629f4 41b882000000 mov r8d, 0x82
0x1401629fa e8a128a700 call 0x140bd52a0 ; rhapsody::raw::texture::TextureDesc::new_2d
0x1401629ff 0f1085101a0000 movups xmm0, xmmword ptr [rbp + 0x1a10]
0x140162a06 0f108d201a0000 movups xmm1, xmmword ptr [rbp + 0x1a20]
0x140162a0d 0f298d60140000 movaps xmmword ptr [rbp + 0x1460], xmm1
0x140162a14 0f298550140000 movaps xmmword ptr [rbp + 0x1450], xmm0
0x140162a1b 488d8d20180000 lea rcx, [rbp + 0x1820]
0x140162a22 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162a29 4c8dad50140000 lea r13, [rbp + 0x1450]
0x140162a30 4c89ea mov rdx, r13
0x140162a33 e8a842a700 call 0x140bd6ce0 ; rhapsody::rg::resource::impl$4::from
0x140162a38 488d0559ad7f02 lea rax, [rip + 0x27fad59]
0x140162a3f 48898510180000 mov qword ptr [rbp + 0x1810], rax
0x140162a46 48c785181800000a000000 mov qword ptr [rbp + 0x1818], 0xa
0x140162a51 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162a58 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x140162a5f 488d9510180000 lea rdx, [rbp + 0x1810]
0x140162a66 e8656aa800 call 0x140be94d0 ; rhapsody::rg::graph::RenderGraph::create_raw_resource
0x140162a6b 0f1085101a0000 movups xmm0, xmmword ptr [rbp + 0x1a10]
0x140162a72 0f108d201a0000 movups xmm1, xmmword ptr [rbp + 0x1a20]
0x140162a79 0f1185d0130000 movups xmmword ptr [rbp + 0x13d0], xmm0
0x140162a80 0f118de0130000 movups xmmword ptr [rbp + 0x13e0], xmm1
0x140162a87 8985cc130000 mov dword ptr [rbp + 0x13cc], eax
0x140162a8d 498b06 mov rax, qword ptr [r14]
0x140162a90 8b8de0190000 mov ecx, dword ptr [rbp + 0x19e0]
0x140162a96 898d70140000 mov dword ptr [rbp + 0x1470], ecx
0x140162a9c 0f1085c0190000 movups xmm0, xmmword ptr [rbp + 0x19c0]
0x140162aa3 0f108dd0190000 movups xmm1, xmmword ptr [rbp + 0x19d0]
0x140162aaa 0f298d60140000 movaps xmmword ptr [rbp + 0x1460], xmm1
0x140162ab1 0f298550140000 movaps xmmword ptr [rbp + 0x1450], xmm0
0x140162ab8 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162abf 4c896c2438 mov qword ptr [rsp + 0x38], r13
0x140162ac4 488d8dcc130000 lea rcx, [rbp + 0x13cc]
0x140162acb 48894c2430 mov qword ptr [rsp + 0x30], rcx
0x140162ad0 488b8df0190000 mov rcx, qword ptr [rbp + 0x19f0]
0x140162ad7 48894c2428 mov qword ptr [rsp + 0x28], rcx
0x140162adc 4889442420 mov qword ptr [rsp + 0x20], rax
0x140162ae1 488d8d10180000 lea rcx, [rbp + 0x1810]
0x140162ae8 488d95e0150000 lea rdx, [rbp + 0x15e0]
0x140162aef 4d89f8 mov r8, r15
0x140162af2 4c8b8d001a0000 mov r9, qword ptr [rbp + 0x1a00]
0x140162af9 e812963800 call 0x1404ec110 ; tiny_glade::render_loop::render_world
0x140162afe 0f108510180000 movups xmm0, xmmword ptr [rbp + 0x1810]
0x140162b05 0f108d20180000 movups xmm1, xmmword ptr [rbp + 0x1820]
0x140162b0c 0f2985b0150000 movaps xmmword ptr [rbp + 0x15b0], xmm0
0x140162b13 0f298dc0150000 movaps xmmword ptr [rbp + 0x15c0], xmm1
0x140162b1a 0fb78530180000 movzx eax, word ptr [rbp + 0x1830]
0x140162b21 668985d0150000 mov word ptr [rbp + 0x15d0], ax
0x140162b28 0fb68532180000 movzx eax, byte ptr [rbp + 0x1832]
0x140162b2f 0fb68d33180000 movzx ecx, byte ptr [rbp + 0x1833]
0x140162b36 0f108534180000 movups xmm0, xmmword ptr [rbp + 0x1834]
0x140162b3d 0f298580150000 movaps xmmword ptr [rbp + 0x1580], xmm0
0x140162b44 0f108544180000 movups xmm0, xmmword ptr [rbp + 0x1844]
0x140162b4b 0f298590150000 movaps xmmword ptr [rbp + 0x1590], xmm0
0x140162b52 0fb79554180000 movzx edx, word ptr [rbp + 0x1854]
0x140162b59 668995a0150000 mov word ptr [rbp + 0x15a0], dx
0x140162b60 0fb69d56180000 movzx ebx, byte ptr [rbp + 0x1856]
0x140162b67 0fb6bd57180000 movzx edi, byte ptr [rbp + 0x1857]
0x140162b6e 8b9578180000 mov edx, dword ptr [rbp + 0x1878]
0x140162b74 8995e0190000 mov dword ptr [rbp + 0x19e0], edx
0x140162b7a 0f108568180000 movups xmm0, xmmword ptr [rbp + 0x1868]
0x140162b81 0f2985d0190000 movaps xmmword ptr [rbp + 0x19d0], xmm0
0x140162b88 0f108558180000 movups xmm0, xmmword ptr [rbp + 0x1858]
0x140162b8f 0f2985c0190000 movaps xmmword ptr [rbp + 0x19c0], xmm0
0x140162b96 b201 mov dl, 1
0x140162b98 8995801a0000 mov dword ptr [rbp + 0x1a80], edx
0x140162b9e 3c07 cmp al, 7
0x140162ba0 0f84dafcffff je 0x140162880
0x140162ba6 0fb795d0150000 movzx edx, word ptr [rbp + 0x15d0]
0x140162bad 66899500180000 mov word ptr [rbp + 0x1800], dx
0x140162bb4 0f2885b0150000 movaps xmm0, xmmword ptr [rbp + 0x15b0]
0x140162bbb 0f288dc0150000 movaps xmm1, xmmword ptr [rbp + 0x15c0]
0x140162bc2 0f298df0170000 movaps xmmword ptr [rbp + 0x17f0], xmm1
0x140162bc9 0f2985e0170000 movaps xmmword ptr [rbp + 0x17e0], xmm0
0x140162bd0 888502180000 mov byte ptr [rbp + 0x1802], al
0x140162bd6 888d03180000 mov byte ptr [rbp + 0x1803], cl
0x140162bdc b001 mov al, 1
0x140162bde 8985801a0000 mov dword ptr [rbp + 0x1a80], eax
0x140162be4 80fb07 cmp bl, 7
0x140162be7 0f8443fdffff je 0x140162930
0x140162bed 0fb785a0150000 movzx eax, word ptr [rbp + 0x15a0]
0x140162bf4 668985d0170000 mov word ptr [rbp + 0x17d0], ax
0x140162bfb 0f288580150000 movaps xmm0, xmmword ptr [rbp + 0x1580]
0x140162c02 0f288d90150000 movaps xmm1, xmmword ptr [rbp + 0x1590]
0x140162c09 0f298dc0170000 movaps xmmword ptr [rbp + 0x17c0], xmm1
0x140162c10 0f2985b0170000 movaps xmmword ptr [rbp + 0x17b0], xmm0
0x140162c17 889dd2170000 mov byte ptr [rbp + 0x17d2], bl
0x140162c1d 4088bdd3170000 mov byte ptr [rbp + 0x17d3], dil
0x140162c24 b101 mov cl, 1
0x140162c26 31c0 xor eax, eax
0x140162c28 f00fb00d20844c03 lock cmpxchg byte ptr [rip + 0x34c8420], cl
0x140162c30 0f85a0120000 jne 0x140163ed6
0x140162c36 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162c3d 488d0d0c844c03 lea rcx, [rip + 0x34c840c]
0x140162c44 e827f0b100 call 0x140c81c70 ; parking_lot_core::parking_lot::deadlock_impl::acquire_resource
0x140162c49 488b85b0190000 mov rax, qword ptr [rbp + 0x19b0]
0x140162c50 488d9890000000 lea rbx, [rax + 0x90]
0x140162c57 488b0502844c03 mov rax, qword ptr [rip + 0x34c8402]
0x140162c5e 486915ff834c0398000000 imul rdx, qword ptr [rip + 0x34c83ff], 0x98
0x140162c69 31c9 xor ecx, ecx
0x140162c6b 4989c0 mov r8, rax
0x140162c6e 6690 nop 
0x140162c70 4839ca cmp rdx, rcx
0x140162c73 7443 je 0x140162cb8
0x140162c75 4881c198000000 add rcx, 0x98
0x140162c7c 4180b89600000006 cmp byte ptr [r8 + 0x96], 6
0x140162c84 4d8d8098000000 lea r8, [r8 + 0x98]
0x140162c8b 75e3 jne 0x140162c70
0x140162c8d 0fb67c08b4 movzx edi, byte ptr [rax + rcx - 0x4c]
0x140162c92 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162c99 488d0db0834c03 lea rcx, [rip + 0x34c83b0]
0x140162ca0 e86bf1b100 call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
0x140162ca5 31c9 xor ecx, ecx
0x140162ca7 b001 mov al, 1
0x140162ca9 f00fb00d9f834c03 lock cmpxchg byte ptr [rip + 0x34c839f], cl
0x140162cb1 742d je 0x140162ce0
0x140162cb3 e93c120000 jmp 0x140163ef4
0x140162cb8 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162cbf 488d0d8a834c03 lea rcx, [rip + 0x34c838a]
0x140162cc6 e845f1b100 call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
0x140162ccb 31c9 xor ecx, ecx
0x140162ccd b001 mov al, 1
0x140162ccf f00fb00d79834c03 lock cmpxchg byte ptr [rip + 0x34c8379], cl
0x140162cd7 40b707 mov dil, 7
0x140162cda 0f8514120000 jne 0x140163ef4
0x140162ce0 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162ce7 48bab9752364092e144a movabs rdx, 0x4a142e09642375b9
0x140162cf1 49b87f848cfa4c15f643 movabs r8, 0x43f6154cfa8c847f
0x140162cfb 4c89e1 mov rcx, r12
0x140162cfe e8eda9f7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x140162d03 4885c0 test rax, rax
0x140162d06 0f84aa120000 je 0x140163fb6
0x140162d0c 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x140162d13 0f869d120000 jbe 0x140163fb6
0x140162d19 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x140162d20 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140162d24 4885c9 test rcx, rcx
0x140162d27 0f8489120000 je 0x140163fb6
0x140162d2d 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x140162d34 48f7d1 not rcx
0x140162d37 488d0c49 lea rcx, [rcx + rcx*2]
0x140162d3b 48c1e105 shl rcx, 5
0x140162d3f 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x140162d45 0f846b120000 je 0x140163fb6
0x140162d4b 4801c8 add rax, rcx
0x140162d4e 488b4810 mov rcx, qword ptr [rax + 0x10]
0x140162d52 4080ff07 cmp dil, 7
0x140162d56 0f8434010000 je 0x140162e90
0x140162d5c 8d47fe lea eax, [rdi - 2]
0x140162d5f 3c05 cmp al, 5
0x140162d61 0f93c2 setae dl
0x140162d64 3c03 cmp al, 3
0x140162d66 0f94c0 sete al
0x140162d69 08d0 or al, dl
0x140162d6b 4084f8 test al, dil
0x140162d6e 0f841c010000 je 0x140162e90
0x140162d74 488b052d964c03 mov rax, qword ptr [rip + 0x34c962d]
0x140162d7b 488b00 mov rax, qword ptr [rax]
0x140162d7e 4883f803 cmp rax, 3
0x140162d82 0f82fa000000 jb 0x140162e82
0x140162d88 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162d8f 4989cd mov r13, rcx
0x140162d92 e8e9af5100 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x140162d97 48898550140000 mov qword ptr [rbp + 0x1450], rax
0x140162d9e 488d052b8dedff lea rax, [rip - 0x1272d5]
0x140162da5 48898558140000 mov qword ptr [rbp + 0x1458], rax
0x140162dac c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162db3 488d0d46aa7f02 lea rcx, [rip + 0x27faa46]
0x140162dba e8c1af5100 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x140162dbf 0f1000 movups xmm0, xmmword ptr [rax]
0x140162dc2 8b4010 mov eax, dword ptr [rax + 0x10]
0x140162dc5 48c7854018000003000000 mov qword ptr [rbp + 0x1840], 3
0x140162dd0 488d0d41aa7f02 lea rcx, [rip + 0x27faa41]
0x140162dd7 48898d48180000 mov qword ptr [rbp + 0x1848], rcx
0x140162dde 48c7855018000017000000 mov qword ptr [rbp + 0x1850], 0x17
0x140162de9 488d1500aa7f02 lea rdx, [rip + 0x27faa00]
0x140162df0 48899560180000 mov qword ptr [rbp + 0x1860], rdx
0x140162df7 48c7856818000001000000 mov qword ptr [rbp + 0x1868], 1
0x140162e02 488d9550140000 lea rdx, [rbp + 0x1450]
0x140162e09 48899570180000 mov qword ptr [rbp + 0x1870], rdx
0x140162e10 48c7857818000001000000 mov qword ptr [rbp + 0x1878], 1
0x140162e1b 48c7858018000000000000 mov qword ptr [rbp + 0x1880], 0
0x140162e26 48c7851018000000000000 mov qword ptr [rbp + 0x1810], 0
0x140162e31 48898d18180000 mov qword ptr [rbp + 0x1818], rcx
0x140162e38 48c7852018000017000000 mov qword ptr [rbp + 0x1820], 0x17
0x140162e43 48c7852818000000000000 mov qword ptr [rbp + 0x1828], 0
0x140162e4e 0f118530180000 movups xmmword ptr [rbp + 0x1830], xmm0
0x140162e55 c7855818000001000000 mov dword ptr [rbp + 0x1858], 1
0x140162e5f 89855c180000 mov dword ptr [rbp + 0x185c], eax
0x140162e65 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162e6c 488d8d501a0000 lea rcx, [rbp + 0x1a50]
0x140162e73 488d9510180000 lea rdx, [rbp + 0x1810]
0x140162e7a e8b174be00 call 0x140d4a330 ; log::__private_api::impl$2::log
0x140162e7f 4c89e9 mov rcx, r13
0x140162e82 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162e89 e8f2ae5100 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x140162e8e eb0c jmp 0x140162e9c
0x140162e90 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162e97 e8b4bb7400 call 0x1408aea50 ; country_core::resources::image_formation::ImageFormationConfigs::current_mut
0x140162e9c c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140162ea3 488d8d101a0000 lea rcx, [rbp + 0x1a10]
0x140162eaa 4889c2 mov rdx, rax
0x140162ead e86edc9900 call 0x140b00b20 ; country_core::resources::grading_lut::ColorGradingState::flattened
0x140162eb2 4080ff07 cmp dil, 7
0x140162eb6 4889b5a0170000 mov qword ptr [rbp + 0x17a0], rsi
0x140162ebd 7421 je 0x140162ee0
0x140162ebf 8d47fe lea eax, [rdi - 2]
0x140162ec2 3c05 cmp al, 5
0x140162ec4 0f93c1 setae cl
0x140162ec7 3c03 cmp al, 3
0x140162ec9 0f94c0 sete al
0x140162ecc 08c8 or al, cl
0x140162ece 7410 je 0x140162ee0
0x140162ed0 89f8 mov eax, edi
0x140162ed2 2401 and al, 1
0x140162ed4 0fb6c0 movzx eax, al
0x140162ed7 488985f8190000 mov qword ptr [rbp + 0x19f8], rax
0x140162ede eb45 jmp 0x140162f25
0x140162ee0 c6858d1a000001 mov byte ptr [rbp + 0x1a8d], 1
0x140162ee7 488d8d10180000 lea rcx, [rbp + 0x1810]
0x140162eee 488d95101a0000 lea rdx, [rbp + 0x1a10]
0x140162ef5 e8968ec100 call 0x140d7bd90 ; alloc::string::impl$6::clone
0x140162efa f30f10b5281a0000 movss xmm6, dword ptr [rbp + 0x1a28]
0x140162f02 488b8510180000 mov rax, qword ptr [rbp + 0x1810]
0x140162f09 488985081a0000 mov qword ptr [rbp + 0x1a08], rax
0x140162f10 488b8518180000 mov rax, qword ptr [rbp + 0x1818]
0x140162f17 488985f8190000 mov qword ptr [rbp + 0x19f8], rax
0x140162f1e 488bb520180000 mov rsi, qword ptr [rbp + 0x1820]
0x140162f25 c685961a000001 mov byte ptr [rbp + 0x1a96], 1
0x140162f2c c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x140162f33 4c8dad74110000 lea r13, [rbp + 0x1174]
0x140162f3a 488d95e0150000 lea rdx, [rbp + 0x15e0]
0x140162f41 4c8d8dc0190000 lea r9, [rbp + 0x19c0]
0x140162f48 4c89e9 mov rcx, r13
0x140162f4b 4d89f8 mov r8, r15
0x140162f4e e80da98800 call 0x1409ed860 ; country_core::systems::render_loop::compute_passes::blur_pyramid::bidirectional_blur_pyramid
0x140162f53 488b8d80170000 mov rcx, qword ptr [rbp + 0x1780]
0x140162f5a 488d4108 lea rax, [rcx + 8]
0x140162f5e f60101 test byte ptr [rcx], 1
0x140162f61 4c0f45f0 cmovne r14, rax
0x140162f65 4d8b0e mov r9, qword ptr [r14]
0x140162f68 8b85e0190000 mov eax, dword ptr [rbp + 0x19e0]
0x140162f6e 898530180000 mov dword ptr [rbp + 0x1830], eax
0x140162f74 0f2885c0190000 movaps xmm0, xmmword ptr [rbp + 0x19c0]
0x140162f7b 0f288dd0190000 movaps xmm1, xmmword ptr [rbp + 0x19d0]
0x140162f82 0f298d20180000 movaps xmmword ptr [rbp + 0x1820], xmm1
0x140162f89 0f298510180000 movaps xmmword ptr [rbp + 0x1810], xmm0
0x140162f90 488b85081a0000 mov rax, qword ptr [rbp + 0x1a08]
0x140162f97 48898550140000 mov qword ptr [rbp + 0x1450], rax
0x140162f9e 488b85f8190000 mov rax, qword ptr [rbp + 0x19f8]
0x140162fa5 48898558140000 mov qword ptr [rbp + 0x1458], rax
0x140162fac 4889b560140000 mov qword ptr [rbp + 0x1460], rsi
0x140162fb3 f30f11b568140000 movss dword ptr [rbp + 0x1468], xmm6
0x140162fbb c6856c14000000 mov byte ptr [rbp + 0x146c], 0
0x140162fc2 488b85f0190000 mov rax, qword ptr [rbp + 0x19f0]
0x140162fc9 f30f10809c000000 movss xmm0, dword ptr [rax + 0x9c]
0x140162fd1 f30f1088a4000000 movss xmm1, dword ptr [rax + 0xa4]
0x140162fd9 c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x140162fe0 c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x140162fe7 f30f114c2448 movss dword ptr [rsp + 0x48], xmm1
0x140162fed f30f11442440 movss dword ptr [rsp + 0x40], xmm0
0x140162ff3 488d8550140000 lea rax, [rbp + 0x1450]
0x140162ffa 4889442438 mov qword ptr [rsp + 0x38], rax
0x140162fff 8b85801a0000 mov eax, dword ptr [rbp + 0x1a80]
0x140163005 88442430 mov byte ptr [rsp + 0x30], al
0x140163009 4c896c2428 mov qword ptr [rsp + 0x28], r13
0x14016300e 488d8510180000 lea rax, [rbp + 0x1810]
0x140163015 4889442420 mov qword ptr [rsp + 0x20], rax
0x14016301a 488d8d501a0000 lea rcx, [rbp + 0x1a50]
0x140163021 488d95e0150000 lea rdx, [rbp + 0x15e0]
0x140163028 4d89f8 mov r8, r15
0x14016302b e8d09f7a00 call 0x14090d000 ; country_core::systems::render_loop::draw_passes::final_post::final_post
0x140163030 48c7851818000002000000 mov qword ptr [rbp + 0x1818], 2
0x14016303b 66c785201800000001 mov word ptr [rbp + 0x1820], 0x100
0x140163044 31c0 xor eax, eax
0x140163046 4c8dade0150000 lea r13, [rbp + 0x15e0]
0x14016304d 4c8db5501a0000 lea r14, [rbp + 0x1a50]
0x140163054 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140163060 4883f802 cmp rax, 2
0x140163064 744b je 0x1401630b1
0x140163066 488d7001 lea rsi, [rax + 1]
0x14016306a 4889b510180000 mov qword ptr [rbp + 0x1810], rsi
0x140163071 440fb6840520180000 movzx r8d, byte ptr [rbp + rax + 0x1820]
0x14016307a c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x140163081 c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x140163088 4c89e9 mov rcx, r13
0x14016308b 4c89fa mov rdx, r15
0x14016308e 4d89f1 mov r9, r14
0x140163091 e8aaa01b00 call 0x14031d140 ; tiny_glade::screenshot::handle_screenshot_opportunity
0x140163096 c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x14016309d c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x1401630a4 4889c1 mov rcx, rax
0x1401630a7 e8048f3800 call 0x1404ebfb0 ; tiny_glade::render_loop::handle_screenshot_error
0x1401630ac 4889f0 mov rax, rsi
0x1401630af ebaf jmp 0x140163060
0x1401630b1 c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x1401630b8 c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x1401630bf 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x1401630c6 4c8d8d501a0000 lea r9, [rbp + 0x1a50]
0x1401630cd 4c89fa mov rdx, r15
0x1401630d0 41b002 mov r8b, 2
0x1401630d3 e868a01b00 call 0x14031d140 ; tiny_glade::screenshot::handle_screenshot_opportunity
0x1401630d8 c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x1401630df c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x1401630e6 4889c1 mov rcx, rax
0x1401630e9 e8c28e3800 call 0x1404ebfb0 ; tiny_glade::render_loop::handle_screenshot_error
0x1401630ee c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x1401630f5 c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x1401630fc 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x140163103 4c8d85501a0000 lea r8, [rbp + 0x1a50]
0x14016310a 4c89fa mov rdx, r15
0x14016310d e81e697600 call 0x1408c9a30 ; country_core::systems::render_loop::menu_background::render_menu_bg
0x140163112 c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x140163119 c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x140163120 8b85481a0000 mov eax, dword ptr [rbp + 0x1a48]
0x140163126 88442428 mov byte ptr [rsp + 0x28], al
0x14016312a 488d85b0170000 lea rax, [rbp + 0x17b0]
0x140163131 4889442420 mov qword ptr [rsp + 0x20], rax
0x140163136 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x14016313d 4c8d85501a0000 lea r8, [rbp + 0x1a50]
0x140163144 4c8d8de0170000 lea r9, [rbp + 0x17e0]
0x14016314b 4c89fa mov rdx, r15
0x14016314e e8bd327400 call 0x1408a6410 ; country_core::systems::render_loop::painterly::painterly_pass
0x140163153 c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x14016315a c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x140163161 48ba28641ac941e9b8c2 movabs rdx, 0xc2b8e941c91a6428
0x14016316b 49b8a043dad3d1c6f359 movabs r8, 0x59f3c6d1d3da43a0
0x140163175 4c89e1 mov rcx, r12
0x140163178 e873a5f7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x14016317d 4885c0 test rax, rax
0x140163180 0f84540e0000 je 0x140163fda
0x140163186 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x14016318d 0f86470e0000 jbe 0x140163fda
0x140163193 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x14016319a 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x14016319e 4885c9 test rcx, rcx
0x1401631a1 0f84330e0000 je 0x140163fda
0x1401631a7 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x1401631ae 48f7d1 not rcx
0x1401631b1 488d0c49 lea rcx, [rcx + rcx*2]
0x1401631b5 48c1e105 shl rcx, 5
0x1401631b9 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1401631bf 0f84150e0000 je 0x140163fda
0x1401631c5 4801c8 add rax, rcx
0x1401631c8 488b4010 mov rax, qword ptr [rax + 0x10]
0x1401631cc 448b30 mov r14d, dword ptr [rax]
0x1401631cf c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x1401631d6 c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x1401631dd 48ba8b047be66126c0a4 movabs rdx, 0xa4c02661e67b048b
0x1401631e7 49b8ec672aaae661e7ea movabs r8, 0xeae761e6aa2a67ec
0x1401631f1 4c89e1 mov rcx, r12
0x1401631f4 e8f7a4f7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1401631f9 4885c0 test rax, rax
0x1401631fc 0f84f70d0000 je 0x140163ff9
0x140163202 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x140163209 0f86ea0d0000 jbe 0x140163ff9
0x14016320f 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x140163216 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x14016321a 4885c9 test rcx, rcx
0x14016321d 0f84d60d0000 je 0x140163ff9
0x140163223 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x14016322a 48f7d1 not rcx
0x14016322d 488d0c49 lea rcx, [rcx + rcx*2]
0x140163231 48c1e105 shl rcx, 5
0x140163235 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x14016323b 0f84b80d0000 je 0x140163ff9
0x140163241 4801c8 add rax, rcx
0x140163244 4183fe13 cmp r14d, 0x13
0x140163248 410f94c1 sete r9b
0x14016324c 488b4010 mov rax, qword ptr [rax + 0x10]
0x140163250 0fb64020 movzx eax, byte ptr [rax + 0x20]
0x140163254 c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x14016325b c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x140163262 88442420 mov byte ptr [rsp + 0x20], al
0x140163266 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x14016326d 4c8d85501a0000 lea r8, [rbp + 0x1a50]
0x140163274 4c89fa mov rdx, r15
0x140163277 e874507400 call 0x1408a82f0 ; country_core::systems::render_loop::vermeer::vermeer_pass
0x14016327c 89c6 mov esi, eax
0x14016327e 4183fe13 cmp r14d, 0x13
0x140163282 410f95c6 setne r14b
0x140163286 488b050b8e4c03 mov rax, qword ptr [rip + 0x34c8e0b]
0x14016328d 803801 cmp byte ptr [rax], 1
0x140163290 750c jne 0x14016329e
0x140163292 4080c7fd add dil, 0xfd
0x140163296 4080ff02 cmp dil, 2
0x14016329a 7218 jb 0x1401632b4
0x14016329c eb77 jmp 0x140163315
0x14016329e 4080ff03 cmp dil, 3
0x1401632a2 7571 jne 0x140163315
0x1401632a4 488b85f0190000 mov rax, qword ptr [rbp + 0x19f0]
0x1401632ab 80b8f300000001 cmp byte ptr [rax + 0xf3], 1
0x1401632b2 7524 jne 0x1401632d8
0x1401632b4 c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x1401632bb c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x1401632c2 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x1401632c9 4c8d85501a0000 lea r8, [rbp + 0x1a50]
0x1401632d0 4c89fa mov rdx, r15
0x1401632d3 e8782c7400 call 0x1408a5f50 ; country_core::systems::render_loop::draw_passes::logo_overlay::render_logo_overlay
0x1401632d8 c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x1401632df c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x1401632e6 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x1401632ed 4c8d8d501a0000 lea r9, [rbp + 0x1a50]
0x1401632f4 4c89fa mov rdx, r15
0x1401632f7 41b003 mov r8b, 3
0x1401632fa e8419e1b00 call 0x14031d140 ; tiny_glade::screenshot::handle_screenshot_opportunity
0x1401632ff c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x140163306 c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x14016330d 4889c1 mov rcx, rax
0x140163310 e89b8c3800 call 0x1404ebfb0 ; tiny_glade::render_loop::handle_screenshot_error
0x140163315 4420f6 and sil, r14b
0x140163318 0f8591010000 jne 0x1401634af
0x14016331e c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x140163325 c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x14016332c 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x140163333 4c8d85501a0000 lea r8, [rbp + 0x1a50]
0x14016333a 4c89fa mov rdx, r15
0x14016333d e8be387900 call 0x1408f6c00 ; country_core::systems::render_loop::session_selection::render_session_selection_previews
0x140163342 c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x140163349 c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x140163350 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x140163357 4c8d85501a0000 lea r8, [rbp + 0x1a50]
0x14016335e 4c89fa mov rdx, r15
0x140163361 e8fad17400 call 0x1408b0560 ; country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery
0x140163366 c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x14016336d c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x140163374 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x14016337b 4c8d85501a0000 lea r8, [rbp + 0x1a50]
0x140163382 4c89fa mov rdx, r15
0x140163385 e8e6267400 call 0x1408a5a70 ; country_core::systems::render_loop::draw_passes::feedback_report_bg::render_feedback_report_bg
0x14016338a 80bd801a000000 cmp byte ptr [rbp + 0x1a80], 0
0x140163391 7456 je 0x1401633e9
0x140163393 c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x14016339a c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x1401633a1 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x1401633a8 4c8d85501a0000 lea r8, [rbp + 0x1a50]
0x1401633af 4c89fa mov rdx, r15
0x1401633b2 4c8b8d001a0000 mov r9, qword ptr [rbp + 0x1a00]
0x1401633b9 e862978600 call 0x1409ccb20 ; country_core::systems::render_loop::draw_passes::wireframe_pass::wireframe_pass
0x1401633be c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x1401633c5 c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x1401633cc 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x1401633d3 4c8d85501a0000 lea r8, [rbp + 0x1a50]
0x1401633da 4c89fa mov rdx, r15
0x1401633dd 4c8b8d001a0000 mov r9, qword ptr [rbp + 0x1a00]
0x1401633e4 e857486d00 call 0x140837c40 ; country_core::systems::render_loop::draw_passes::ui_pass::ui_pass
0x1401633e9 c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x1401633f0 c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x1401633f7 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x1401633fe 4c8d85501a0000 lea r8, [rbp + 0x1a50]
0x140163405 4c89fa mov rdx, r15
0x140163408 e863108600 call 0x1409c4470 ; country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass
0x14016340d c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x140163414 c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x14016341b 488b8da0170000 mov rcx, qword ptr [rbp + 0x17a0]
0x140163422 e8d9c77d00 call 0x14093fc00 ; country_core::resources::gui::EscMenuState::cast_ui_shadows
0x140163427 c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x14016342e c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x140163435 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x14016343c 4c8d85501a0000 lea r8, [rbp + 0x1a50]
0x140163443 4c89fa mov rdx, r15
0x140163446 4189c1 mov r9d, eax
0x140163449 e8b2328600 call 0x1409c6700 ; country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass
0x14016344e c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x140163455 c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x14016345c 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x140163463 4c8d85501a0000 lea r8, [rbp + 0x1a50]
0x14016346a 4c89fa mov rdx, r15
0x14016346d e89ee28600 call 0x1409d1710 ; country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass
0x140163472 c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x140163479 c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x140163480 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x140163487 4c8d8d501a0000 lea r9, [rbp + 0x1a50]
0x14016348e 4c89fa mov rdx, r15
0x140163491 41b004 mov r8b, 4
0x140163494 e8a79c1b00 call 0x14031d140 ; tiny_glade::screenshot::handle_screenshot_opportunity
0x140163499 c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x1401634a0 c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x1401634a7 4889c1 mov rcx, rax
0x1401634aa e8018b3800 call 0x1404ebfb0 ; tiny_glade::render_loop::handle_screenshot_error
0x1401634af c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x1401634b6 c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x1401634bd 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x1401634c4 4c8d85501a0000 lea r8, [rbp + 0x1a50]
0x1401634cb 4c89fa mov rdx, r15
0x1401634ce 4c8b8d001a0000 mov r9, qword ptr [rbp + 0x1a00]
0x1401634d5 e8164b6d00 call 0x140837ff0 ; country_core::systems::render_loop::draw_passes::ui_pass::loading_screen_pass
0x1401634da 4883bdb819000000 cmp qword ptr [rbp + 0x19b8], 0
0x1401634e2 742b je 0x14016350f
0x1401634e4 c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x1401634eb c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x1401634f2 488d95e0150000 lea rdx, [rbp + 0x15e0]
0x1401634f9 4c8d8d501a0000 lea r9, [rbp + 0x1a50]
0x140163500 488b8db8190000 mov rcx, qword ptr [rbp + 0x19b8]
0x140163507 4d89f8 mov r8, r15
0x14016350a e871a7f7ff call 0x1400ddc80 ; tiny_glade::egui_backend::EguiBackend::draw
0x14016350f c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x140163516 c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x14016351d 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x140163524 4c8d8d501a0000 lea r9, [rbp + 0x1a50]
0x14016352b 4c89fa mov rdx, r15
0x14016352e 41b006 mov r8b, 6
0x140163531 e80a9c1b00 call 0x14031d140 ; tiny_glade::screenshot::handle_screenshot_opportunity
0x140163536 c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x14016353d c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x140163544 4889c1 mov rcx, rax
0x140163547 e8648a3800 call 0x1404ebfb0 ; tiny_glade::render_loop::handle_screenshot_error
0x14016354c c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x140163553 c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x14016355a 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x140163561 4c8d85501a0000 lea r8, [rbp + 0x1a50]
0x140163568 4c89fa mov rdx, r15
0x14016356b e820674200 call 0x140589c90 ; tiny_glade::panic_reporter::panic_screenshot::capture_image
0x140163570 0f1085501a0000 movups xmm0, xmmword ptr [rbp + 0x1a50]
0x140163577 0f108d601a0000 movups xmm1, xmmword ptr [rbp + 0x1a60]
0x14016357e 0f298510180000 movaps xmmword ptr [rbp + 0x1810], xmm0
0x140163585 8b85701a0000 mov eax, dword ptr [rbp + 0x1a70]
0x14016358b 898530180000 mov dword ptr [rbp + 0x1830], eax
0x140163591 0f298d20180000 movaps xmmword ptr [rbp + 0x1820], xmm1
0x140163598 448bb510180000 mov r14d, dword ptr [rbp + 0x1810]
0x14016359f c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x1401635a6 c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x1401635ad 488d8d10180000 lea rcx, [rbp + 0x1810]
0x1401635b4 488d95e0150000 lea rdx, [rbp + 0x15e0]
0x1401635bb 41b022 mov r8b, 0x22
0x1401635be e8bd2da700 call 0x140bd6380 ; rhapsody::rg::import_export::impl$0::export
0x1401635c3 488db510180000 lea rsi, [rbp + 0x1810]
0x1401635ca 488d95e0150000 lea rdx, [rbp + 0x15e0]
0x1401635d1 41b898010000 mov r8d, 0x198
0x1401635d7 4889f1 mov rcx, rsi
0x1401635da e821067c02 call 0x142923c00 ; memcpy
0x1401635df c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x1401635e6 c685951a000000 mov byte ptr [rbp + 0x1a95], 0
0x1401635ed 488d8d50140000 lea rcx, [rbp + 0x1450]
0x1401635f4 4889f2 mov rdx, rsi
0x1401635f7 e854a48700 call 0x1409dda50 ; country_core::systems::render_loop::RenderContext::finish
0x1401635fc 48baaa4e7896b36185a6 movabs rdx, 0xa68561b396784eaa
0x140163606 49b814edf4fae674fdf5 movabs r8, 0xf5fd74e6faf4ed14
0x140163610 4c89e1 mov rcx, r12
0x140163613 e8d8a0f7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x140163618 488bbdb0190000 mov rdi, qword ptr [rbp + 0x19b0]
0x14016361f 4c8bad88170000 mov r13, qword ptr [rbp + 0x1788]
0x140163626 4885c0 test rax, rax
0x140163629 0f84e9090000 je 0x140164018
0x14016362f 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x140163636 0f86dc090000 jbe 0x140164018
0x14016363c 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x140163643 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140163647 4885c9 test rcx, rcx
0x14016364a 0f84c8090000 je 0x140164018
0x140163650 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x140163657 48f7d1 not rcx
0x14016365a 488d0c49 lea rcx, [rcx + rcx*2]
0x14016365e 48c1e105 shl rcx, 5
0x140163662 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x140163668 0f84aa090000 je 0x140164018
0x14016366e 4801c8 add rax, rcx
0x140163671 488b4810 mov rcx, qword ptr [rax + 0x10]
0x140163675 418b97b0050000 mov edx, dword ptr [r15 + 0x5b0]
0x14016367c 89505c mov dword ptr [rax + 0x5c], edx
0x14016367f 488d9550140000 lea rdx, [rbp + 0x1450]
0x140163686 e8a5977d00 call 0x14093ce30 ; country_core::asset_libraries::texture_library::AssetTextureLibrary::export_preset_textures_from_rg
0x14016368b 48ba5593676e78cc2682 movabs rdx, 0x8226cc786e679355
0x140163695 49b8ac41a8556adacf65 movabs r8, 0x65cfda6a55a841ac
0x14016369f 4c89e1 mov rcx, r12
0x1401636a2 e849a0f7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1401636a7 4885c0 test rax, rax
0x1401636aa 0f8479090000 je 0x140164029
0x1401636b0 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x1401636b7 0f866c090000 jbe 0x140164029
0x1401636bd 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x1401636c4 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1401636c8 4885c9 test rcx, rcx
0x1401636cb 0f8458090000 je 0x140164029
0x1401636d1 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x1401636d8 48f7d1 not rcx
0x1401636db 488d0c49 lea rcx, [rcx + rcx*2]
0x1401636df 48c1e105 shl rcx, 5
0x1401636e3 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1401636e9 0f843a090000 je 0x140164029
0x1401636ef 4801c8 add rax, rcx
0x1401636f2 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1401636f6 418b97b0050000 mov edx, dword ptr [r15 + 0x5b0]
0x1401636fd 89505c mov dword ptr [rax + 0x5c], edx
0x140163700 488d9550140000 lea rdx, [rbp + 0x1450]
0x140163707 e8a4b47b00 call 0x14091ebb0 ; country_core::asset_libraries::ssbo_library::AssetSsboLibrary::export_from_rg
0x14016370c 48baefa5998645782966 movabs rdx, 0x662978458699a5ef
0x140163716 49b8469b4162c157c03e movabs r8, 0x3ec057c162419b46
0x140163720 4c89e1 mov rcx, r12
0x140163723 e8c89ff7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x140163728 4885c0 test rax, rax
0x14016372b 0f8409090000 je 0x14016403a
0x140163731 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x140163738 0f86fc080000 jbe 0x14016403a
0x14016373e 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x140163745 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140163749 4885c9 test rcx, rcx
0x14016374c 0f84e8080000 je 0x14016403a
0x140163752 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x140163759 48f7d1 not rcx
0x14016375c 488d0c49 lea rcx, [rcx + rcx*2]
0x140163760 48c1e105 shl rcx, 5
0x140163764 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x14016376a 0f84ca080000 je 0x14016403a
0x140163770 4801c8 add rax, rcx
0x140163773 488b4810 mov rcx, qword ptr [rax + 0x10]
0x140163777 418b97b0050000 mov edx, dword ptr [r15 + 0x5b0]
0x14016377e 89505c mov dword ptr [rax + 0x5c], edx
0x140163781 488d9550140000 lea rdx, [rbp + 0x1450]
0x140163788 e893ea8200 call 0x140992220 ; country_core::render::nani::gpu_world::GpuWorld::export_from_rg
0x14016378d 488d8d10180000 lea rcx, [rbp + 0x1810]
0x140163794 488d9550140000 lea rdx, [rbp + 0x1450]
0x14016379b 41b810010000 mov r8d, 0x110
0x1401637a1 e85a047c02 call 0x142923c00 ; memcpy
0x1401637a6 488b95101a0000 mov rdx, qword ptr [rbp + 0x1a10]
0x1401637ad 4885d2 test rdx, rdx
0x1401637b0 7412 je 0x1401637c4
0x1401637b2 488b8d181a0000 mov rcx, qword ptr [rbp + 0x1a18]
0x1401637b9 41b801000000 mov r8d, 1
0x1401637bf e85c044b00 call 0x140613c20 ; __rust_dealloc
0x1401637c4 488d75d0 lea rsi, [rbp - 0x30]
0x1401637c8 488d9510180000 lea rdx, [rbp + 0x1810]
0x1401637cf 41b810010000 mov r8d, 0x110
0x1401637d5 4889f1 mov rcx, rsi
0x1401637d8 e823047c02 call 0x142923c00 ; memcpy
0x1401637dd 488d8de0000000 lea rcx, [rbp + 0xe0]
0x1401637e4 4889f2 mov rdx, rsi
0x1401637e7 e8b4443900 call 0x1404f7ca0 ; rhapsody::rg::temporal::TemporalRenderGraphRecorder::export_temporal<country_core::render::pass_builder::pass_resources::PassResources>
0x1401637ec 488d8d50140000 lea rcx, [rbp + 0x1450]
0x1401637f3 488d95e0000000 lea rdx, [rbp + 0xe0]
0x1401637fa 41b8a8000000 mov r8d, 0xa8
0x140163800 e8fb037c02 call 0x142923c00 ; memcpy
0x140163805 0f108588010000 movups xmm0, xmmword ptr [rbp + 0x188]
0x14016380c 0f108d98010000 movups xmm1, xmmword ptr [rbp + 0x198]
0x140163813 0f1095a8010000 movups xmm2, xmmword ptr [rbp + 0x1a8]
0x14016381a 0f109db8010000 movups xmm3, xmmword ptr [rbp + 0x1b8]
0x140163821 0f2985f0130000 movaps xmmword ptr [rbp + 0x13f0], xmm0
0x140163828 0f298d00140000 movaps xmmword ptr [rbp + 0x1400], xmm1
0x14016382f 0f299510140000 movaps xmmword ptr [rbp + 0x1410], xmm2
0x140163836 0f299d20140000 movaps xmmword ptr [rbp + 0x1420], xmm3
0x14016383d 0f1085c8010000 movups xmm0, xmmword ptr [rbp + 0x1c8]
0x140163844 0f298530140000 movaps xmmword ptr [rbp + 0x1430], xmm0
0x14016384b 0f1085d8010000 movups xmm0, xmmword ptr [rbp + 0x1d8]
0x140163852 0f298540140000 movaps xmmword ptr [rbp + 0x1440], xmm0
0x140163859 0f104350 movups xmm0, xmmword ptr [rbx + 0x50]
0x14016385d 0f298530010000 movaps xmmword ptr [rbp + 0x130], xmm0
0x140163864 0f104340 movups xmm0, xmmword ptr [rbx + 0x40]
0x140163868 0f298520010000 movaps xmmword ptr [rbp + 0x120], xmm0
0x14016386f 0f1003 movups xmm0, xmmword ptr [rbx]
0x140163872 0f104b10 movups xmm1, xmmword ptr [rbx + 0x10]
0x140163876 0f105320 movups xmm2, xmmword ptr [rbx + 0x20]
0x14016387a 0f105b30 movups xmm3, xmmword ptr [rbx + 0x30]
0x14016387e 0f299d10010000 movaps xmmword ptr [rbp + 0x110], xmm3
0x140163885 0f299500010000 movaps xmmword ptr [rbp + 0x100], xmm2
0x14016388c 0f298df0000000 movaps xmmword ptr [rbp + 0xf0], xmm1
0x140163893 0f2985e0000000 movaps xmmword ptr [rbp + 0xe0], xmm0
0x14016389a 0f2885f0130000 movaps xmm0, xmmword ptr [rbp + 0x13f0]
0x1401638a1 0f288d00140000 movaps xmm1, xmmword ptr [rbp + 0x1400]
0x1401638a8 0f289510140000 movaps xmm2, xmmword ptr [rbp + 0x1410]
0x1401638af 0f289d20140000 movaps xmm3, xmmword ptr [rbp + 0x1420]
0x1401638b6 0f1103 movups xmmword ptr [rbx], xmm0
0x1401638b9 0f114b10 movups xmmword ptr [rbx + 0x10], xmm1
0x1401638bd 0f115320 movups xmmword ptr [rbx + 0x20], xmm2
0x1401638c1 0f115b30 movups xmmword ptr [rbx + 0x30], xmm3
0x1401638c5 0f288530140000 movaps xmm0, xmmword ptr [rbp + 0x1430]
0x1401638cc 0f114340 movups xmmword ptr [rbx + 0x40], xmm0
0x1401638d0 0f288540140000 movaps xmm0, xmmword ptr [rbp + 0x1440]
0x1401638d7 0f114350 movups xmmword ptr [rbx + 0x50], xmm0
0x1401638db 4883bde000000000 cmp qword ptr [rbp + 0xe0], 0
0x1401638e3 488b9d90170000 mov rbx, qword ptr [rbp + 0x1790]
0x1401638ea 7457 je 0x140163943
0x1401638ec 488d9500010000 lea rdx, [rbp + 0x100]
0x1401638f3 488d8de0000000 lea rcx, [rbp + 0xe0]
0x1401638fa 41b868000000 mov r8d, 0x68
0x140163900 41b910000000 mov r9d, 0x10
0x140163906 e8a595eeff call 0x14004ceb0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>,alloc::alloc::Global>
0x14016390b 488b9518010000 mov rdx, qword ptr [rbp + 0x118]
0x140163912 4885d2 test rdx, rdx
0x140163915 742c je 0x140163943
0x140163917 4889d0 mov rax, rdx
0x14016391a 48c1e004 shl rax, 4
0x14016391e 4801c2 add rdx, rax
0x140163921 4883c221 add rdx, 0x21
0x140163925 741c je 0x140163943
0x140163927 488b8d10010000 mov rcx, qword ptr [rbp + 0x110]
0x14016392e 48f7d8 neg rax
0x140163931 4801c1 add rcx, rax
0x140163934 4883c1f0 add rcx, -0x10
0x140163938 41b810000000 mov r8d, 0x10
0x14016393e e8dd024b00 call 0x140613c20 ; __rust_dealloc
0x140163943 488d8d38130000 lea rcx, [rbp + 0x1338]
0x14016394a 488d9550140000 lea rdx, [rbp + 0x1450]
0x140163951 41b890000000 mov r8d, 0x90
0x140163957 e8a4027c02 call 0x142923c00 ; memcpy
0x14016395c 488b85f0140000 mov rax, qword ptr [rbp + 0x14f0]
0x140163963 488985601a0000 mov qword ptr [rbp + 0x1a60], rax
0x14016396a 0f1085e0140000 movups xmm0, xmmword ptr [rbp + 0x14e0]
0x140163971 0f2985501a0000 movaps xmmword ptr [rbp + 0x1a50], xmm0
0x140163978 48c785e000000000000000 mov qword ptr [rbp + 0xe0], 0
0x140163983 48c785e800000008000000 mov qword ptr [rbp + 0xe8], 8
0x14016398e 48c785f000000000000000 mov qword ptr [rbp + 0xf0], 0
0x140163999 31c9 xor ecx, ecx
0x14016399b e88061f7ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
0x1401639a0 4885c0 test rax, rax
0x1401639a3 0f84e3060000 je 0x14016408c
0x1401639a9 0f1000 movups xmm0, xmmword ptr [rax]
0x1401639ac 48ff00 inc qword ptr [rax]
0x1401639af 0f100d7ac68402 movups xmm1, xmmword ptr [rip + 0x284c67a]
0x1401639b6 0f118dc8160000 movups xmmword ptr [rbp + 0x16c8], xmm1
0x1401639bd 0f100d5cc68402 movups xmm1, xmmword ptr [rip + 0x284c65c]
0x1401639c4 0f118db8160000 movups xmmword ptr [rbp + 0x16b8], xmm1
0x1401639cb 0f1185d8160000 movups xmmword ptr [rbp + 0x16d8], xmm0
0x1401639d2 48c785e015000000000000 mov qword ptr [rbp + 0x15e0], 0
0x1401639dd 48c785e815000002000000 mov qword ptr [rbp + 0x15e8], 2
0x1401639e8 0f57c0 xorps xmm0, xmm0
0x1401639eb 0f1185f0150000 movups xmmword ptr [rbp + 0x15f0], xmm0
0x1401639f2 48c7850016000001000000 mov qword ptr [rbp + 0x1600], 1
0x1401639fd 48c7850816000000000000 mov qword ptr [rbp + 0x1608], 0
0x140163a08 0f108de0000000 movups xmm1, xmmword ptr [rbp + 0xe0]
0x140163a0f 0f118d10160000 movups xmmword ptr [rbp + 0x1610], xmm1
0x140163a16 488b85f0000000 mov rax, qword ptr [rbp + 0xf0]
0x140163a1d 48898520160000 mov qword ptr [rbp + 0x1620], rax
0x140163a24 488d8d28160000 lea rcx, [rbp + 0x1628]
0x140163a2b 48c7852816000000000000 mov qword ptr [rbp + 0x1628], 0
0x140163a36 48c7853016000004000000 mov qword ptr [rbp + 0x1630], 4
0x140163a41 0f118538160000 movups xmmword ptr [rbp + 0x1638], xmm0
0x140163a48 48c7854816000004000000 mov qword ptr [rbp + 0x1648], 4
0x140163a53 0f118550160000 movups xmmword ptr [rbp + 0x1650], xmm0
0x140163a5a 48c7856016000004000000 mov qword ptr [rbp + 0x1660], 4
0x140163a65 0f118568160000 movups xmmword ptr [rbp + 0x1668], xmm0
0x140163a6c 48c7857816000004000000 mov qword ptr [rbp + 0x1678], 4
0x140163a77 0f118580160000 movups xmmword ptr [rbp + 0x1680], xmm0
0x140163a7e 48c7859016000004000000 mov qword ptr [rbp + 0x1690], 4
0x140163a89 0f118598160000 movups xmmword ptr [rbp + 0x1698], xmm0
0x140163a90 48c785a816000004000000 mov qword ptr [rbp + 0x16a8], 4
0x140163a9b 48c785b016000000000000 mov qword ptr [rbp + 0x16b0], 0
0x140163aa6 4c8b85581a0000 mov r8, qword ptr [rbp + 0x1a58]
0x140163aad 4c8b8d601a0000 mov r9, qword ptr [rbp + 0x1a60]
0x140163ab4 c685901a000001 mov byte ptr [rbp + 0x1a90], 1
0x140163abb c6858f1a000001 mov byte ptr [rbp + 0x1a8f], 1
0x140163ac2 488d9538130000 lea rdx, [rbp + 0x1338]
0x140163ac9 e862c93c00 call 0x140530430 ; rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources>
0x140163ace c685901a000001 mov byte ptr [rbp + 0x1a90], 1
0x140163ad5 c6858f1a000001 mov byte ptr [rbp + 0x1a8f], 1
0x140163adc 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x140163ae3 488d9538130000 lea rdx, [rbp + 0x1338]
0x140163aea 4c8b8598170000 mov r8, qword ptr [rbp + 0x1798]
0x140163af1 4d89e9 mov r9, r13
0x140163af4 e8a740a800 call 0x140be7ba0 ; rhapsody::rg::resource_registry::ResourceRegistry::populate_resources
0x140163af9 84c0 test al, al
0x140163afb 0f84a8000000 je 0x140163ba9
0x140163b01 c685901a000001 mov byte ptr [rbp + 0x1a90], 1
0x140163b08 c6858f1a000001 mov byte ptr [rbp + 0x1a8f], 1
0x140163b0f 488d8d38130000 lea rcx, [rbp + 0x1338]
0x140163b16 e80559a800 call 0x140be9420 ; rhapsody::rg::graph::RenderGraph::on_retire
0x140163b1b 488d8de0000000 lea rcx, [rbp + 0xe0]
0x140163b22 41b890000000 mov r8d, 0x90
0x140163b28 4889fa mov rdx, rdi
0x140163b2b e8d0007c02 call 0x142923c00 ; memcpy
0x140163b30 488d9538130000 lea rdx, [rbp + 0x1338]
0x140163b37 41b890000000 mov r8d, 0x90
0x140163b3d 4889f9 mov rcx, rdi
0x140163b40 e8bb007c02 call 0x142923c00 ; memcpy
0x140163b45 31c0 xor eax, eax
0x140163b47 483b85e0000000 cmp rax, qword ptr [rbp + 0xe0]
0x140163b4e 701a jo 0x140163b6a
0x140163b50 c685901a000000 mov byte ptr [rbp + 0x1a90], 0
0x140163b57 c6858f1a000001 mov byte ptr [rbp + 0x1a8f], 1
0x140163b5e 488d8de0000000 lea rcx, [rbp + 0xe0]
0x140163b65 e8a6b64800 call 0x1405ef210 ; core::ptr::drop_in_place<rhapsody::rg::graph::RenderGraph>
0x140163b6a 48b80000000000000080 movabs rax, 0x8000000000000000
0x140163b74 488903 mov qword ptr [rbx], rax
0x140163b77 c685921a000000 mov byte ptr [rbp + 0x1a92], 0
0x140163b7e c685911a000001 mov byte ptr [rbp + 0x1a91], 1
0x140163b85 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x140163b8c e83f964900 call 0x1405fd1d0 ; core::ptr::drop_in_place<rhapsody::rg::resource_registry::ResourceRegistry>
0x140163b91 c6858b1a000000 mov byte ptr [rbp + 0x1a8b], 0
0x140163b98 488d8d501a0000 lea rcx, [rbp + 0x1a50]
0x140163b9f e87c6c4700 call 0x1405da820 ; core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources>,alloc::alloc::Global> >
0x140163ba4 e997000000 jmp 0x140163c40
0x140163ba9 488db528120000 lea rsi, [rbp + 0x1228]
0x140163bb0 488d9538130000 lea rdx, [rbp + 0x1338]
0x140163bb7 41b890000000 mov r8d, 0x90
0x140163bbd 4889f1 mov rcx, rsi
0x140163bc0 e83b007c02 call 0x142923c00 ; memcpy
0x140163bc5 488b85601a0000 mov rax, qword ptr [rbp + 0x1a60]
0x140163bcc 488985201a0000 mov qword ptr [rbp + 0x1a20], rax
0x140163bd3 0f2885501a0000 movaps xmm0, xmmword ptr [rbp + 0x1a50]
0x140163bda 0f2985101a0000 movaps xmmword ptr [rbp + 0x1a10], xmm0
0x140163be1 488dbd10180000 lea rdi, [rbp + 0x1810]
0x140163be8 488d95e0150000 lea rdx, [rbp + 0x15e0]
0x140163bef 41b808010000 mov r8d, 0x108
0x140163bf5 4889f9 mov rcx, rdi
0x140163bf8 e803007c02 call 0x142923c00 ; memcpy
0x140163bfd c685901a000000 mov byte ptr [rbp + 0x1a90], 0
0x140163c04 c6858f1a000000 mov byte ptr [rbp + 0x1a8f], 0
0x140163c0b 488d8de0000000 lea rcx, [rbp + 0xe0]
0x140163c12 4c8d85101a0000 lea r8, [rbp + 0x1a10]
0x140163c19 4889f2 mov rdx, rsi
0x140163c1c 4989f9 mov r9, rdi
0x140163c1f e85c061600 call 0x1402c4280 ; rhapsody::rg::compiled::CompiledRenderGraph::new<country_core::render::pass_builder::pass_resources::PassResources>
0x140163c24 488d95e0000000 lea rdx, [rbp + 0xe0]
0x140163c2b 41b818020000 mov r8d, 0x218
0x140163c31 4889d9 mov rcx, rbx
0x140163c34 e8c7ff7b02 call 0x142923c00 ; memcpy
0x140163c39 4489b318020000 mov dword ptr [rbx + 0x218], r14d
0x140163c40 4889d8 mov rax, rbx
0x140163c43 0f28b5a01a0000 movaps xmm6, xmmword ptr [rbp + 0x1aa0]
0x140163c4a 4881c4381b0000 add rsp, 0x1b38
0x140163c51 5b pop rbx
0x140163c52 5f pop rdi
0x140163c53 5e pop rsi
0x140163c54 415c pop r12
0x140163c56 415d pop r13
0x140163c58 415e pop r14
0x140163c5a 415f pop r15
0x140163c5c 5d pop rbp
0x140163c5d c3 ret 
0x140163c5e c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140163c65 4889f1 mov rcx, rsi
0x140163c68 e8f3bf7d00 call 0x14093fc60 ; country_core::resources::gui::EscMenuState::can_limit_frame_rate
0x140163c6d 488d0de89a7f02 lea rcx, [rip + 0x27f9ae8]
0x140163c74 48898d101a0000 mov qword ptr [rbp + 0x1a10], rcx
0x140163c7b 48c785181a00001f000000 mov qword ptr [rbp + 0x1a18], 0x1f
0x140163c86 84c0 test al, al
0x140163c88 0f8530ebffff jne 0x1401627be
0x140163c8e 488b0513874c03 mov rax, qword ptr [rip + 0x34c8713]
0x140163c95 488b00 mov rax, qword ptr [rax]
0x140163c98 4883f805 cmp rax, 5
0x140163c9c 0f829eebffff jb 0x140162840
0x140163ca2 4c89ad50140000 mov qword ptr [rbp + 0x1450], r13
0x140163ca9 488d05e061faff lea rax, [rip - 0x59e20]
0x140163cb0 48898558140000 mov qword ptr [rbp + 0x1458], rax
0x140163cb7 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140163cbe 488d0d937e8802 lea rcx, [rip + 0x2887e93]
0x140163cc5 e8b6a05100 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x140163cca 0f1000 movups xmm0, xmmword ptr [rax]
0x140163ccd 8b4010 mov eax, dword ptr [rax + 0x10]
0x140163cd0 48c7854018000005000000 mov qword ptr [rbp + 0x1840], 5
0x140163cdb 488d0d8e7e8802 lea rcx, [rip + 0x2887e8e]
0x140163ce2 48898d48180000 mov qword ptr [rbp + 0x1848], rcx
0x140163ce9 48c7855018000017000000 mov qword ptr [rbp + 0x1850], 0x17
0x140163cf4 488d15257e8802 lea rdx, [rip + 0x2887e25]
0x140163cfb 48899560180000 mov qword ptr [rbp + 0x1860], rdx
0x140163d02 48c7856818000001000000 mov qword ptr [rbp + 0x1868], 1
0x140163d0d 488d9550140000 lea rdx, [rbp + 0x1450]
0x140163d14 48899570180000 mov qword ptr [rbp + 0x1870], rdx
0x140163d1b 48c7857818000001000000 mov qword ptr [rbp + 0x1878], 1
0x140163d26 48c7858018000000000000 mov qword ptr [rbp + 0x1880], 0
0x140163d31 48c7851018000000000000 mov qword ptr [rbp + 0x1810], 0
0x140163d3c 48898d18180000 mov qword ptr [rbp + 0x1818], rcx
0x140163d43 48c7852018000017000000 mov qword ptr [rbp + 0x1820], 0x17
0x140163d4e 48c7852818000000000000 mov qword ptr [rbp + 0x1828], 0
0x140163d59 0f118530180000 movups xmmword ptr [rbp + 0x1830], xmm0
0x140163d60 c7855818000001000000 mov dword ptr [rbp + 0x1858], 1
0x140163d6a 89855c180000 mov dword ptr [rbp + 0x185c], eax
0x140163d70 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140163d77 488d8d501a0000 lea rcx, [rbp + 0x1a50]
0x140163d7e 488d9510180000 lea rdx, [rbp + 0x1810]
0x140163d85 e8a665be00 call 0x140d4a330 ; log::__private_api::impl$2::log
0x140163d8a e9b1eaffff jmp 0x140162840
0x140163d8f 488b0512864c03 mov rax, qword ptr [rip + 0x34c8612]
0x140163d96 488b00 mov rax, qword ptr [rax]
0x140163d99 4883f805 cmp rax, 5
0x140163d9d 0f829deaffff jb 0x140162840
0x140163da3 4c89ad50140000 mov qword ptr [rbp + 0x1450], r13
0x140163daa 488d05df60faff lea rax, [rip - 0x59f21]
0x140163db1 48898558140000 mov qword ptr [rbp + 0x1458], rax
0x140163db8 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140163dbf 488d0d927d8802 lea rcx, [rip + 0x2887d92]
0x140163dc6 e8b59f5100 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x140163dcb 0f1000 movups xmm0, xmmword ptr [rax]
0x140163dce 8b4010 mov eax, dword ptr [rax + 0x10]
0x140163dd1 48c7854018000005000000 mov qword ptr [rbp + 0x1840], 5
0x140163ddc 488d0d8d7d8802 lea rcx, [rip + 0x2887d8d]
0x140163de3 48898d48180000 mov qword ptr [rbp + 0x1848], rcx
0x140163dea 48c7855018000017000000 mov qword ptr [rbp + 0x1850], 0x17
0x140163df5 488d15247d8802 lea rdx, [rip + 0x2887d24]
0x140163dfc 48899560180000 mov qword ptr [rbp + 0x1860], rdx
0x140163e03 48c7856818000001000000 mov qword ptr [rbp + 0x1868], 1
0x140163e0e 488d9550140000 lea rdx, [rbp + 0x1450]
0x140163e15 48899570180000 mov qword ptr [rbp + 0x1870], rdx
0x140163e1c 48c7857818000001000000 mov qword ptr [rbp + 0x1878], 1
0x140163e27 48c7858018000000000000 mov qword ptr [rbp + 0x1880], 0
0x140163e32 48c7851018000000000000 mov qword ptr [rbp + 0x1810], 0
0x140163e3d 48898d18180000 mov qword ptr [rbp + 0x1818], rcx
0x140163e44 48c7852018000017000000 mov qword ptr [rbp + 0x1820], 0x17
0x140163e4f 48c7852818000000000000 mov qword ptr [rbp + 0x1828], 0
0x140163e5a 0f118530180000 movups xmmword ptr [rbp + 0x1830], xmm0
0x140163e61 c7855818000001000000 mov dword ptr [rbp + 0x1858], 1
0x140163e6b 89855c180000 mov dword ptr [rbp + 0x185c], eax
0x140163e71 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140163e78 488d8d501a0000 lea rcx, [rbp + 0x1a50]
0x140163e7f 488d9510180000 lea rdx, [rbp + 0x1810]
0x140163e86 e8a564be00 call 0x140d4a330 ; log::__private_api::impl$2::log
0x140163e8b e9b0e9ffff jmp 0x140162840
0x140163e90 c685941a000001 mov byte ptr [rbp + 0x1a94], 1
0x140163e97 4889f9 mov rcx, rdi
0x140163e9a 41b800ca9a3b mov r8d, 0x3b9aca00
0x140163ea0 e86b937602 call 0x1428cd210 ; parking_lot::raw_mutex::RawMutex::lock_slow
0x140163ea5 e92ae2ffff jmp 0x1401620d4
0x140163eaa 4c8d05ffa98702 lea r8, [rip + 0x287a9ff]
0x140163eb1 ba90100000 mov edx, 0x1090
0x140163eb6 e8255a7702 call 0x1428d98e0 ; core::slice::impl$0::copy_from_slice::len_mismatch_fail
0x140163ebb e902020000 jmp 0x1401640c2
0x140163ec0 c6858e1a000001 mov byte ptr [rbp + 0x1a8e], 1
0x140163ec7 4889f9 mov rcx, rdi
0x140163eca 31d2 xor edx, edx
0x140163ecc e8bf9c7602 call 0x1428cdb90 ; parking_lot::raw_mutex::RawMutex::unlock_slow
0x140163ed1 e9e6e2ffff jmp 0x1401621bc
0x140163ed6 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140163edd 488d0d6c714c03 lea rcx, [rip + 0x34c716c]
0x140163ee4 41b800ca9a3b mov r8d, 0x3b9aca00
0x140163eea e821937602 call 0x1428cd210 ; parking_lot::raw_mutex::RawMutex::lock_slow
0x140163eef e942edffff jmp 0x140162c36
0x140163ef4 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140163efb 488d0d4e714c03 lea rcx, [rip + 0x34c714e]
0x140163f02 31d2 xor edx, edx
0x140163f04 e8879c7602 call 0x1428cdb90 ; parking_lot::raw_mutex::RawMutex::unlock_slow
0x140163f09 e9d2edffff jmp 0x140162ce0
0x140163f0e c685931a000001 mov byte ptr [rbp + 0x1a93], 1
0x140163f15 488d0d14977f02 lea rcx, [rip + 0x27f9714]
0x140163f1c e8ef537702 call 0x1428d9310 ; core::option::unwrap_failed
0x140163f21 e99c010000 jmp 0x1401640c2
0x140163f26 c685941a000001 mov byte ptr [rbp + 0x1a94], 1
0x140163f2d 488d0d14977f02 lea rcx, [rip + 0x27f9714]
0x140163f34 e8d7537702 call 0x1428d9310 ; core::option::unwrap_failed
0x140163f39 e984010000 jmp 0x1401640c2
0x140163f3e c685941a000001 mov byte ptr [rbp + 0x1a94], 1
0x140163f45 488d0d14977f02 lea rcx, [rip + 0x27f9714]
0x140163f4c e8bf537702 call 0x1428d9310 ; core::option::unwrap_failed
0x140163f51 e96c010000 jmp 0x1401640c2
0x140163f56 c685941a000001 mov byte ptr [rbp + 0x1a94], 1
0x140163f5d 488d0d14977f02 lea rcx, [rip + 0x27f9714]
0x140163f64 e8a7537702 call 0x1428d9310 ; core::option::unwrap_failed
0x140163f69 e954010000 jmp 0x1401640c2
0x140163f6e c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140163f75 488d0d14977f02 lea rcx, [rip + 0x27f9714]
0x140163f7c e88f537702 call 0x1428d9310 ; core::option::unwrap_failed
0x140163f81 e93c010000 jmp 0x1401640c2
0x140163f86 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140163f8d 488d0d24977f02 lea rcx, [rip + 0x27f9724]
0x140163f94 e877537702 call 0x1428d9310 ; core::option::unwrap_failed
0x140163f99 e924010000 jmp 0x1401640c2
0x140163f9e c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140163fa5 488d0d24977f02 lea rcx, [rip + 0x27f9724]
0x140163fac e85f537702 call 0x1428d9310 ; core::option::unwrap_failed
0x140163fb1 e90c010000 jmp 0x1401640c2
0x140163fb6 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x140163fbd 488d0dde977f02 lea rcx, [rip + 0x27f97de]
0x140163fc4 4c8d05ed977f02 lea r8, [rip + 0x27f97ed]
0x140163fcb ba15000000 mov edx, 0x15
0x140163fd0 e85b537702 call 0x1428d9330 ; core::option::expect_failed
0x140163fd5 e9e8000000 jmp 0x1401640c2
0x140163fda c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x140163fe1 c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x140163fe8 488d0d41987f02 lea rcx, [rip + 0x27f9841]
0x140163fef e81c537702 call 0x1428d9310 ; core::option::unwrap_failed
0x140163ff4 e9c9000000 jmp 0x1401640c2
0x140163ff9 c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x140164000 c685951a000001 mov byte ptr [rbp + 0x1a95], 1
0x140164007 488d0d3a987f02 lea rcx, [rip + 0x27f983a]
0x14016400e e8fd527702 call 0x1428d9310 ; core::option::unwrap_failed
0x140164013 e9aa000000 jmp 0x1401640c2
0x140164018 488d0d41987f02 lea rcx, [rip + 0x27f9841]
0x14016401f e8ec527702 call 0x1428d9310 ; core::option::unwrap_failed
0x140164024 e999000000 jmp 0x1401640c2
0x140164029 488d0d48987f02 lea rcx, [rip + 0x27f9848]
0x140164030 e8db527702 call 0x1428d9310 ; core::option::unwrap_failed
0x140164035 e988000000 jmp 0x1401640c2
0x14016403a 488d0d4f987f02 lea rcx, [rip + 0x27f984f]
0x140164041 e8ca527702 call 0x1428d9310 ; core::option::unwrap_failed
0x140164046 eb7a jmp 0x1401640c2
0x140164048 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x14016404f 488d0d2a977f02 lea rcx, [rip + 0x27f972a]
0x140164056 e8b5527702 call 0x1428d9310 ; core::option::unwrap_failed
0x14016405b eb65 jmp 0x1401640c2
0x14016405d 488d0d74957f02 lea rcx, [rip + 0x27f9574]
0x140164064 e8a7527702 call 0x1428d9310 ; core::option::unwrap_failed
0x140164069 488d0d80957f02 lea rcx, [rip + 0x27f9580]
0x140164070 e89b527702 call 0x1428d9310 ; core::option::unwrap_failed
0x140164075 eb4b jmp 0x1401640c2
0x140164077 c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x14016407e 488d0d63967f02 lea rcx, [rip + 0x27f9663]
0x140164085 e886527702 call 0x1428d9310 ; core::option::unwrap_failed
0x14016408a eb36 jmp 0x1401640c2
0x14016408c 488d0d7de97c02 lea rcx, [rip + 0x27ce97d]
0x140164093 e838407702 call 0x1428d80d0 ; std::thread::local::panic_access_error
0x140164098 eb28 jmp 0x1401640c2
0x14016409a c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x1401640a1 488d0d88967f02 lea rcx, [rip + 0x27f9688]
0x1401640a8 e863527702 call 0x1428d9310 ; core::option::unwrap_failed
0x1401640ad eb13 jmp 0x1401640c2
0x1401640af c685971a000001 mov byte ptr [rbp + 0x1a97], 1
0x1401640b6 488d0d43967f02 lea rcx, [rip + 0x27f9643]
0x1401640bd e84e527702 call 0x1428d9310 ; core::option::unwrap_failed
0x1401640c2 0f0b ud2 
0x1401640c4 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1401640d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1401640d5 55 push rbp
0x1401640d6 4157 push r15
0x1401640d8 4156 push r14
0x1401640da 4155 push r13
0x1401640dc 4154 push r12
0x1401640de 56 push rsi
0x1401640df 57 push rdi
0x1401640e0 53 push rbx
0x1401640e1 4883ec68 sub rsp, 0x68
0x1401640e5 488daa80000000 lea rbp, [rdx + 0x80]
0x1401640ec 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x1401640f1 0f108530180000 movups xmm0, xmmword ptr [rbp + 0x1830]
0x1401640f8 0f108d40180000 movups xmm1, xmmword ptr [rbp + 0x1840]
0x1401640ff 0f298df0150000 movaps xmmword ptr [rbp + 0x15f0], xmm1
0x140164106 0f2985e0150000 movaps xmmword ptr [rbp + 0x15e0], xmm0
0x14016410d 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x140164114 e897e3a700 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
0x140164119 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016411e 4883c468 add rsp, 0x68
0x140164122 5b pop rbx
0x140164123 5f pop rdi
0x140164124 5e pop rsi
0x140164125 415c pop r12
0x140164127 415d pop r13
0x140164129 415e pop r14
0x14016412b 415f pop r15
0x14016412d 5d pop rbp
0x14016412e c3 ret 
0x14016412f 90 nop 
0x140164130 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140164135 55 push rbp
0x140164136 4157 push r15
0x140164138 4156 push r14
0x14016413a 4155 push r13
0x14016413c 4154 push r12
0x14016413e 56 push rsi
0x14016413f 57 push rdi
0x140164140 53 push rbx
0x140164141 4883ec68 sub rsp, 0x68
0x140164145 488daa80000000 lea rbp, [rdx + 0x80]
0x14016414c 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140164151 0fb69d8d1a0000 movzx ebx, byte ptr [rbp + 0x1a8d]
0x140164158 488b95101a0000 mov rdx, qword ptr [rbp + 0x1a10]
0x14016415f 4885d2 test rdx, rdx
0x140164162 7412 je 0x140164176
0x140164164 488b8d181a0000 mov rcx, qword ptr [rbp + 0x1a18]
0x14016416b 41b801000000 mov r8d, 1
0x140164171 e8aafa4a00 call 0x140613c20 ; __rust_dealloc
0x140164176 80e301 and bl, 1
0x140164179 889d971a0000 mov byte ptr [rbp + 0x1a97], bl
0x14016417f 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140164184 4883c468 add rsp, 0x68
0x140164188 5b pop rbx
0x140164189 5f pop rdi
0x14016418a 5e pop rsi
0x14016418b 415c pop r12
0x14016418d 415d pop r13
0x14016418f 415e pop r14
0x140164191 415f pop r15
0x140164193 5d pop rbp
0x140164194 c3 ret 
0x140164195 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1401641a0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1401641a5 55 push rbp
0x1401641a6 4157 push r15
0x1401641a8 4156 push r14
0x1401641aa 4155 push r13
0x1401641ac 4154 push r12
0x1401641ae 56 push rsi
0x1401641af 57 push rdi
0x1401641b0 53 push rbx
0x1401641b1 4883ec68 sub rsp, 0x68
0x1401641b5 488daa80000000 lea rbp, [rdx + 0x80]
0x1401641bc 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x1401641c1 0fb69d951a0000 movzx ebx, byte ptr [rbp + 0x1a95]
0x1401641c8 80bd961a000000 cmp byte ptr [rbp + 0x1a96], 0
0x1401641cf 744c je 0x14016421d
0x1401641d1 488b85081a0000 mov rax, qword ptr [rbp + 0x1a08]
0x1401641d8 488d044500000000 lea rax, [rax*2]
0x1401641e0 4885c0 test rax, rax
0x1401641e3 7419 je 0x1401641fe
0x1401641e5 41b801000000 mov r8d, 1
0x1401641eb 488b8df8190000 mov rcx, qword ptr [rbp + 0x19f8]
0x1401641f2 488b95081a0000 mov rdx, qword ptr [rbp + 0x1a08]
0x1401641f9 e822fa4a00 call 0x140613c20 ; __rust_dealloc
0x1401641fe 80e301 and bl, 1
0x140164201 889d8d1a0000 mov byte ptr [rbp + 0x1a8d], bl
0x140164207 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016420c 4883c468 add rsp, 0x68
0x140164210 5b pop rbx
0x140164211 5f pop rdi
0x140164212 5e pop rsi
0x140164213 415c pop r12
0x140164215 415d pop r13
0x140164217 415e pop r14
0x140164219 415f pop r15
0x14016421b 5d pop rbp
0x14016421c c3 ret 
0x14016421d 80e301 and bl, 1
0x140164220 889d8d1a0000 mov byte ptr [rbp + 0x1a8d], bl
0x140164226 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016422b 4883c468 add rsp, 0x68
0x14016422f 5b pop rbx
0x140164230 5f pop rdi
0x140164231 5e pop rsi
0x140164232 415c pop r12
0x140164234 415d pop r13
0x140164236 415e pop r14
0x140164238 415f pop r15
0x14016423a 5d pop rbp
0x14016423b c3 ret 
0x14016423c 0f1f4000 nop dword ptr [rax]
0x140164240 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140164245 55 push rbp
0x140164246 4157 push r15
0x140164248 4156 push r14
0x14016424a 4155 push r13
0x14016424c 4154 push r12
0x14016424e 56 push rsi
0x14016424f 57 push rdi
0x140164250 53 push rbx
0x140164251 4883ec68 sub rsp, 0x68
0x140164255 488daa80000000 lea rbp, [rdx + 0x80]
0x14016425c 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140164261 80bd971a000000 cmp byte ptr [rbp + 0x1a97], 0
0x140164268 7429 je 0x140164293
0x14016426a 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x140164271 e86a894900 call 0x1405fcbe0 ; core::ptr::drop_in_place<country_core::systems::render_loop::RenderContext>
0x140164276 c6858e1a000000 mov byte ptr [rbp + 0x1a8e], 0
0x14016427d 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140164282 4883c468 add rsp, 0x68
0x140164286 5b pop rbx
0x140164287 5f pop rdi
0x140164288 5e pop rsi
0x140164289 415c pop r12
0x14016428b 415d pop r13
0x14016428d 415e pop r14
0x14016428f 415f pop r15
0x140164291 5d pop rbp
0x140164292 c3 ret 
0x140164293 c6858e1a000000 mov byte ptr [rbp + 0x1a8e], 0
0x14016429a 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016429f 4883c468 add rsp, 0x68
0x1401642a3 5b pop rbx
0x1401642a4 5f pop rdi
0x1401642a5 5e pop rsi
0x1401642a6 415c pop r12
0x1401642a8 415d pop r13
0x1401642aa 415e pop r14
0x1401642ac 415f pop r15
0x1401642ae 5d pop rbp
0x1401642af c3 ret 
0x1401642b0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1401642b5 55 push rbp
0x1401642b6 4157 push r15
0x1401642b8 4156 push r14
0x1401642ba 4155 push r13
0x1401642bc 4154 push r12
0x1401642be 56 push rsi
0x1401642bf 57 push rdi
0x1401642c0 53 push rbx
0x1401642c1 4883ec68 sub rsp, 0x68
0x1401642c5 488daa80000000 lea rbp, [rdx + 0x80]
0x1401642cc 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x1401642d1 0fb69d8e1a0000 movzx ebx, byte ptr [rbp + 0x1a8e]
0x1401642d8 f6c301 test bl, 1
0x1401642db 7428 je 0x140164305
0x1401642dd 488d8d38130000 lea rcx, [rbp + 0x1338]
0x1401642e4 e867d8ffff call 0x140161b50 ; core::ptr::drop_in_place<rhapsody::raw::managed::handle::GpuOwned<array$<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,2>,rhapsody::raw::managed::handle::RawManagedHandle> >
0x1401642e9 889d941a0000 mov byte ptr [rbp + 0x1a94], bl
0x1401642ef 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401642f4 4883c468 add rsp, 0x68
0x1401642f8 5b pop rbx
0x1401642f9 5f pop rdi
0x1401642fa 5e pop rsi
0x1401642fb 415c pop r12
0x1401642fd 415d pop r13
0x1401642ff 415e pop r14
0x140164301 415f pop r15
0x140164303 5d pop rbp
0x140164304 c3 ret 
0x140164305 889d941a0000 mov byte ptr [rbp + 0x1a94], bl
0x14016430b 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140164310 4883c468 add rsp, 0x68
0x140164314 5b pop rbx
0x140164315 5f pop rdi
0x140164316 5e pop rsi
0x140164317 415c pop r12
0x140164319 415d pop r13
0x14016431b 415e pop r14
0x14016431d 415f pop r15
0x14016431f 5d pop rbp
0x140164320 c3 ret 
0x140164321 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140164330 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140164335 55 push rbp
0x140164336 4157 push r15
0x140164338 4156 push r14
0x14016433a 4155 push r13
0x14016433c 4154 push r12
0x14016433e 56 push rsi
0x14016433f 57 push rdi
0x140164340 53 push rbx
0x140164341 4883ec68 sub rsp, 0x68
0x140164345 488daa80000000 lea rbp, [rdx + 0x80]
0x14016434c 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140164351 0fb69d941a0000 movzx ebx, byte ptr [rbp + 0x1a94]
0x140164358 f6c301 test bl, 1
0x14016435b 7428 je 0x140164385
0x14016435d 488d8d68150000 lea rcx, [rbp + 0x1568]
0x140164364 e847e1a700 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
0x140164369 889d931a0000 mov byte ptr [rbp + 0x1a93], bl
0x14016436f 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140164374 4883c468 add rsp, 0x68
0x140164378 5b pop rbx
0x140164379 5f pop rdi
0x14016437a 5e pop rsi
0x14016437b 415c pop r12
0x14016437d 415d pop r13
0x14016437f 415e pop r14
0x140164381 415f pop r15
0x140164383 5d pop rbp
0x140164384 c3 ret 
0x140164385 889d931a0000 mov byte ptr [rbp + 0x1a93], bl
0x14016438b 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140164390 4883c468 add rsp, 0x68
0x140164394 5b pop rbx
0x140164395 5f pop rdi
0x140164396 5e pop rsi
0x140164397 415c pop r12
0x140164399 415d pop r13
0x14016439b 415e pop r14
0x14016439d 415f pop r15
0x14016439f 5d pop rbp
0x1401643a0 c3 ret 
0x1401643a1 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1401643b0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1401643b5 55 push rbp
0x1401643b6 4157 push r15
0x1401643b8 4156 push r14
0x1401643ba 4155 push r13
0x1401643bc 4154 push r12
0x1401643be 56 push rsi
0x1401643bf 57 push rdi
0x1401643c0 53 push rbx
0x1401643c1 4883ec68 sub rsp, 0x68
0x1401643c5 488daa80000000 lea rbp, [rdx + 0x80]
0x1401643cc 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x1401643d1 f685931a000001 test byte ptr [rbp + 0x1a93], 1
0x1401643d8 740c je 0x1401643e6
0x1401643da 488d8d28120000 lea rcx, [rbp + 0x1228]
0x1401643e1 e8da894700 call 0x1405dcdc0 ; core::ptr::drop_in_place<rhapsody::rg::temporal::TemporalRenderGraphRecorder<country_core::render::pass_builder::pass_resources::PassResources> >
0x1401643e6 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401643eb 4883c468 add rsp, 0x68
0x1401643ef 5b pop rbx
0x1401643f0 5f pop rdi
0x1401643f1 5e pop rsi
0x1401643f2 415c pop r12
0x1401643f4 415d pop r13
0x1401643f6 415e pop r14
0x1401643f8 415f pop r15
0x1401643fa 5d pop rbp
0x1401643fb c3 ret 
0x1401643fc 0f1f4000 nop dword ptr [rax]
0x140164400 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140164405 55 push rbp
0x140164406 4157 push r15
0x140164408 4156 push r14
0x14016440a 4155 push r13
0x14016440c 4154 push r12
0x14016440e 56 push rsi
0x14016440f 57 push rdi
0x140164410 53 push rbx
0x140164411 4883ec68 sub rsp, 0x68
0x140164415 488daa80000000 lea rbp, [rdx + 0x80]
0x14016441c 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140164421 488d8d10010000 lea rcx, [rbp + 0x110]
0x140164428 488d9530010000 lea rdx, [rbp + 0x130]
0x14016442f 41b810000000 mov r8d, 0x10
0x140164435 41b910000000 mov r9d, 0x10
0x14016443b e8c07feeff call 0x14004c400 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<core::ptr::non_null::NonNull<hashlink::linked_hash_map::Node<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> > >,tuple$<> >,allocator_api2::stable::alloc::global::Global>
0x140164440 c6858c1a000001 mov byte ptr [rbp + 0x1a8c], 1
0x140164447 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016444c 4883c468 add rsp, 0x68
0x140164450 5b pop rbx
0x140164451 5f pop rdi
0x140164452 5e pop rsi
0x140164453 415c pop r12
0x140164455 415d pop r13
0x140164457 415e pop r14
0x140164459 415f pop r15
0x14016445b 5d pop rbp
0x14016445c c3 ret 
0x14016445d 0f1f00 nop dword ptr [rax]
0x140164460 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140164465 55 push rbp
0x140164466 4157 push r15
0x140164468 4156 push r14
0x14016446a 4155 push r13
0x14016446c 4154 push r12
0x14016446e 56 push rsi
0x14016446f 57 push rdi
0x140164470 53 push rbx
0x140164471 4883ec68 sub rsp, 0x68
0x140164475 488daa80000000 lea rbp, [rdx + 0x80]
0x14016447c 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140164481 c685921a000001 mov byte ptr [rbp + 0x1a92], 1
0x140164488 c685911a000001 mov byte ptr [rbp + 0x1a91], 1
0x14016448f 488d8de0000000 lea rcx, [rbp + 0xe0]
0x140164496 e815714a00 call 0x14060b5b0 ; core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::rg::resource_registry::RegistryResource,alloc::alloc::Global> >
0x14016449b c685921a000001 mov byte ptr [rbp + 0x1a92], 1
0x1401644a2 c685911a000001 mov byte ptr [rbp + 0x1a91], 1
0x1401644a9 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401644ae 4883c468 add rsp, 0x68
0x1401644b2 5b pop rbx
0x1401644b3 5f pop rdi
0x1401644b4 5e pop rsi
0x1401644b5 415c pop r12
0x1401644b7 415d pop r13
0x1401644b9 415e pop r14
0x1401644bb 415f pop r15
0x1401644bd 5d pop rbp
0x1401644be c3 ret 
0x1401644bf 90 nop 
0x1401644c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1401644c5 55 push rbp
0x1401644c6 4157 push r15
0x1401644c8 4156 push r14
0x1401644ca 4155 push r13
0x1401644cc 4154 push r12
0x1401644ce 56 push rsi
0x1401644cf 57 push rdi
0x1401644d0 53 push rbx
0x1401644d1 4883ec68 sub rsp, 0x68
0x1401644d5 488daa80000000 lea rbp, [rdx + 0x80]
0x1401644dc 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x1401644e1 0fb69d8f1a0000 movzx ebx, byte ptr [rbp + 0x1a8f]
0x1401644e8 0fb6b5901a0000 movzx esi, byte ptr [rbp + 0x1a90]
0x1401644ef f6c301 test bl, 1
0x1401644f2 7433 je 0x140164527
0x1401644f4 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x1401644fb e8d08c4900 call 0x1405fd1d0 ; core::ptr::drop_in_place<rhapsody::rg::resource_registry::ResourceRegistry>
0x140164500 4080e601 and sil, 1
0x140164504 4088b5921a0000 mov byte ptr [rbp + 0x1a92], sil
0x14016450b 889d911a0000 mov byte ptr [rbp + 0x1a91], bl
0x140164511 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140164516 4883c468 add rsp, 0x68
0x14016451a 5b pop rbx
0x14016451b 5f pop rdi
0x14016451c 5e pop rsi
0x14016451d 415c pop r12
0x14016451f 415d pop r13
0x140164521 415e pop r14
0x140164523 415f pop r15
0x140164525 5d pop rbp
0x140164526 c3 ret 
0x140164527 4080e601 and sil, 1
0x14016452b 4088b5921a0000 mov byte ptr [rbp + 0x1a92], sil
0x140164532 889d911a0000 mov byte ptr [rbp + 0x1a91], bl
0x140164538 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016453d 4883c468 add rsp, 0x68
0x140164541 5b pop rbx
0x140164542 5f pop rdi
0x140164543 5e pop rsi
0x140164544 415c pop r12
0x140164546 415d pop r13
0x140164548 415e pop r14
0x14016454a 415f pop r15
0x14016454c 5d pop rbp
0x14016454d c3 ret 
0x14016454e 6690 nop 
0x140164550 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140164555 55 push rbp
0x140164556 4157 push r15
0x140164558 4156 push r14
0x14016455a 4155 push r13
0x14016455c 4154 push r12
0x14016455e 56 push rsi
0x14016455f 57 push rdi
0x140164560 53 push rbx
0x140164561 4883ec68 sub rsp, 0x68
0x140164565 488daa80000000 lea rbp, [rdx + 0x80]
0x14016456c 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140164571 0fb69d921a0000 movzx ebx, byte ptr [rbp + 0x1a92]
0x140164578 f685911a000001 test byte ptr [rbp + 0x1a91], 1
0x14016457f 742b je 0x1401645ac
0x140164581 488d8d501a0000 lea rcx, [rbp + 0x1a50]
0x140164588 e893624700 call 0x1405da820 ; core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources>,alloc::alloc::Global> >
0x14016458d 80e301 and bl, 1
0x140164590 889d8b1a0000 mov byte ptr [rbp + 0x1a8b], bl
0x140164596 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016459b 4883c468 add rsp, 0x68
0x14016459f 5b pop rbx
0x1401645a0 5f pop rdi
0x1401645a1 5e pop rsi
0x1401645a2 415c pop r12
0x1401645a4 415d pop r13
0x1401645a6 415e pop r14
0x1401645a8 415f pop r15
0x1401645aa 5d pop rbp
0x1401645ab c3 ret 
0x1401645ac 80e301 and bl, 1
0x1401645af 889d8b1a0000 mov byte ptr [rbp + 0x1a8b], bl
0x1401645b5 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401645ba 4883c468 add rsp, 0x68
0x1401645be 5b pop rbx
0x1401645bf 5f pop rdi
0x1401645c0 5e pop rsi
0x1401645c1 415c pop r12
0x1401645c3 415d pop r13
0x1401645c5 415e pop r14
0x1401645c7 415f pop r15
0x1401645c9 5d pop rbp
0x1401645ca c3 ret 
0x1401645cb 0f1f440000 nop dword ptr [rax + rax]
0x1401645d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1401645d5 55 push rbp
0x1401645d6 4157 push r15
0x1401645d8 4156 push r14
0x1401645da 4155 push r13
0x1401645dc 4154 push r12
0x1401645de 56 push rsi
0x1401645df 57 push rdi
0x1401645e0 53 push rbx
0x1401645e1 4883ec68 sub rsp, 0x68
0x1401645e5 488daa80000000 lea rbp, [rdx + 0x80]
0x1401645ec 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x1401645f1 80bd8b1a000000 cmp byte ptr [rbp + 0x1a8b], 0
0x1401645f8 7429 je 0x140164623
0x1401645fa 488d8d38130000 lea rcx, [rbp + 0x1338]
0x140164601 e80aac4800 call 0x1405ef210 ; core::ptr::drop_in_place<rhapsody::rg::graph::RenderGraph>
0x140164606 c6858c1a000000 mov byte ptr [rbp + 0x1a8c], 0
0x14016460d 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140164612 4883c468 add rsp, 0x68
0x140164616 5b pop rbx
0x140164617 5f pop rdi
0x140164618 5e pop rsi
0x140164619 415c pop r12
0x14016461b 415d pop r13
0x14016461d 415e pop r14
0x14016461f 415f pop r15
0x140164621 5d pop rbp
0x140164622 c3 ret 
0x140164623 c6858c1a000000 mov byte ptr [rbp + 0x1a8c], 0
0x14016462a 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016462f 4883c468 add rsp, 0x68
0x140164633 5b pop rbx
0x140164634 5f pop rdi
0x140164635 5e pop rsi
0x140164636 415c pop r12
0x140164638 415d pop r13
0x14016463a 415e pop r14
0x14016463c 415f pop r15
0x14016463e 5d pop rbp
0x14016463f c3 ret 
0x140164640 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140164645 55 push rbp
0x140164646 4157 push r15
0x140164648 4156 push r14
0x14016464a 4155 push r13
0x14016464c 4154 push r12
0x14016464e 56 push rsi
0x14016464f 57 push rdi
0x140164650 53 push rbx
0x140164651 4883ec68 sub rsp, 0x68
0x140164655 488daa80000000 lea rbp, [rdx + 0x80]
0x14016465c 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140164661 80bd8c1a000000 cmp byte ptr [rbp + 0x1a8c], 0
0x140164668 7422 je 0x14016468c
0x14016466a 488d8d50140000 lea rcx, [rbp + 0x1450]
0x140164671 e88a684700 call 0x1405daf00 ; core::ptr::drop_in_place<rhapsody::rg::recorder::RenderGraphRecorder<country_core::render::pass_builder::pass_resources::PassResources> >
0x140164676 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016467b 4883c468 add rsp, 0x68
0x14016467f 5b pop rbx
0x140164680 5f pop rdi
0x140164681 5e pop rsi
0x140164682 415c pop r12
0x140164684 415d pop r13
0x140164686 415e pop r14
0x140164688 415f pop r15
0x14016468a 5d pop rbp
0x14016468b c3 ret 
0x14016468c 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140164691 4883c468 add rsp, 0x68
0x140164695 5b pop rbx
0x140164696 5f pop rdi
0x140164697 5e pop rsi
0x140164698 415c pop r12
0x14016469a 415d pop r13
0x14016469c 415e pop r14
0x14016469e 415f pop r15
0x1401646a0 5d pop rbp
0x1401646a1 c3 ret 
0x1401646a2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1401646b0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1401646b5 55 push rbp
0x1401646b6 4157 push r15
0x1401646b8 4156 push r14
0x1401646ba 4155 push r13
0x1401646bc 4154 push r12
0x1401646be 56 push rsi
0x1401646bf 57 push rdi
0x1401646c0 53 push rbx
0x1401646c1 4883ec68 sub rsp, 0x68
0x1401646c5 488daa80000000 lea rbp, [rdx + 0x80]
0x1401646cc 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x1401646d1 4883bda817000000 cmp qword ptr [rbp + 0x17a8], 0
0x1401646d9 7416 je 0x1401646f1
0x1401646db 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401646e0 4883c468 add rsp, 0x68
0x1401646e4 5b pop rbx
0x1401646e5 5f pop rdi
0x1401646e6 5e pop rsi
0x1401646e7 415c pop r12
0x1401646e9 415d pop r13
0x1401646eb 415e pop r14
0x1401646ed 415f pop r15
0x1401646ef 5d pop rbp
0x1401646f0 c3 ret 
0x1401646f1 488d8d98110000 lea rcx, [rbp + 0x1198]
0x1401646f8 e813ab4800 call 0x1405ef210 ; core::ptr::drop_in_place<rhapsody::rg::graph::RenderGraph>
0x1401646fd ebdc jmp 0x1401646db
0x1401646ff 90 nop 
0x140164700 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140164705 55 push rbp
0x140164706 4157 push r15
0x140164708 4156 push r14
0x14016470a 4155 push r13
0x14016470c 4154 push r12
0x14016470e 56 push rsi
0x14016470f 57 push rdi
0x140164710 53 push rbx
0x140164711 4883ec68 sub rsp, 0x68
0x140164715 488daa80000000 lea rbp, [rdx + 0x80]
0x14016471c 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140164721 488b8d801a0000 mov rcx, qword ptr [rbp + 0x1a80]
0x140164728 e803d5ffff call 0x140161c30 ; core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> > >
0x14016472d c685941a000001 mov byte ptr [rbp + 0x1a94], 1
0x140164734 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140164739 4883c468 add rsp, 0x68
0x14016473d 5b pop rbx
0x14016473e 5f pop rdi
0x14016473f 5e pop rsi
0x140164740 415c pop r12
0x140164742 415d pop r13
0x140164744 415e pop r14
0x140164746 415f pop r15
0x140164748 5d pop rbp
0x140164749 c3 ret 
0x14016474a 660f1f440000 nop word ptr [rax + rax]
0x140164750 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140164755 55 push rbp
0x140164756 4157 push r15
0x140164758 4156 push r14
0x14016475a 4155 push r13
0x14016475c 4154 push r12
0x14016475e 56 push rsi
0x14016475f 57 push rdi
0x140164760 53 push rbx
0x140164761 4883ec68 sub rsp, 0x68
0x140164765 488daa80000000 lea rbp, [rdx + 0x80]
0x14016476c 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140164771 488d8d50140000 lea rcx, [rbp + 0x1450]
0x140164778 e843864700 call 0x1405dcdc0 ; core::ptr::drop_in_place<rhapsody::rg::temporal::TemporalRenderGraphRecorder<country_core::render::pass_builder::pass_resources::PassResources> >
0x14016477d c685961a000000 mov byte ptr [rbp + 0x1a96], 0
0x140164784 c685951a000000 mov byte ptr [rbp + 0x1a95], 0
0x14016478b 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140164790 4883c468 add rsp, 0x68
0x140164794 5b pop rbx
0x140164795 5f pop rdi
0x140164796 5e pop rsi
0x140164797 415c pop r12
0x140164799 415d pop r13
0x14016479b 415e pop r14
0x14016479d 415f pop r15
0x14016479f 5d pop rbp
0x1401647a0 c3 ret 