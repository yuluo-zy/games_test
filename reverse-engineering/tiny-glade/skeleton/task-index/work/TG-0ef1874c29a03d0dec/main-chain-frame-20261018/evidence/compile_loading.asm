0x1401647b0 55 push rbp
0x1401647b1 4157 push r15
0x1401647b3 4156 push r14
0x1401647b5 4155 push r13
0x1401647b7 4154 push r12
0x1401647b9 56 push rsi
0x1401647ba 57 push rdi
0x1401647bb 53 push rbx
0x1401647bc b8181a0000 mov eax, 0x1a18
0x1401647c1 e87a627302 call 0x14289aa40 ; __chkstk
0x1401647c6 4829c4 sub rsp, rax
0x1401647c9 488dac2480000000 lea rbp, [rsp + 0x80]
0x1401647d1 0f29b580190000 movaps xmmword ptr [rbp + 0x1980], xmm6
0x1401647d8 48c78578190000feffffff mov qword ptr [rbp + 0x1978], 0xfffffffffffffffe
0x1401647e3 48898de8180000 mov qword ptr [rbp + 0x18e8], rcx
0x1401647ea 488b02 mov rax, qword ptr [rdx]
0x1401647ed 48b90000000000000080 movabs rcx, 0x8000000000000000
0x1401647f7 48890a mov qword ptr [rdx], rcx
0x1401647fa 4889c1 mov rcx, rax
0x1401647fd 48f7d9 neg rcx
0x140164800 0f80dd130000 jo 0x140165be3
0x140164806 4d89c4 mov r12, r8
0x140164809 4889d7 mov rdi, rdx
0x14016480c 4c898dd0180000 mov qword ptr [rbp + 0x18d0], r9
0x140164813 4883c208 add rdx, 8
0x140164817 48898570110000 mov qword ptr [rbp + 0x1170], rax
0x14016481e 488d8d78110000 lea rcx, [rbp + 0x1178]
0x140164825 41b888000000 mov r8d, 0x88
0x14016482b e8d0f37b02 call 0x142923c00 ; memcpy
0x140164830 488b9f90000000 mov rbx, qword ptr [rdi + 0x90]
0x140164837 48c7879000000000000000 mov qword ptr [rdi + 0x90], 0
0x140164842 4885db test rbx, rbx
0x140164845 48899de0180000 mov qword ptr [rbp + 0x18e0], rbx
0x14016484c 0f849d130000 je 0x140165bef
0x140164852 488bb5001a0000 mov rsi, qword ptr [rbp + 0x1a00]
0x140164859 488b87e8000000 mov rax, qword ptr [rdi + 0xe8]
0x140164860 48898528130000 mov qword ptr [rbp + 0x1328], rax
0x140164867 0f1087d8000000 movups xmm0, xmmword ptr [rdi + 0xd8]
0x14016486e 0f118518130000 movups xmmword ptr [rbp + 0x1318], xmm0
0x140164875 0f108798000000 movups xmm0, xmmword ptr [rdi + 0x98]
0x14016487c 0f108fa8000000 movups xmm1, xmmword ptr [rdi + 0xa8]
0x140164883 0f1097b8000000 movups xmm2, xmmword ptr [rdi + 0xb8]
0x14016488a 0f109fc8000000 movups xmm3, xmmword ptr [rdi + 0xc8]
0x140164891 0f119d08130000 movups xmmword ptr [rbp + 0x1308], xmm3
0x140164898 0f1195f8120000 movups xmmword ptr [rbp + 0x12f8], xmm2
0x14016489f 0f118de8120000 movups xmmword ptr [rbp + 0x12e8], xmm1
0x1401648a6 0f1185d8120000 movups xmmword ptr [rbp + 0x12d8], xmm0
0x1401648ad 488d8d20120000 lea rcx, [rbp + 0x1220]
0x1401648b4 488d9570110000 lea rdx, [rbp + 0x1170]
0x1401648bb 41b890000000 mov r8d, 0x90
0x1401648c1 e83af37b02 call 0x142923c00 ; memcpy
0x1401648c6 4c8b3e mov r15, qword ptr [rsi]
0x1401648c9 4c8b7608 mov r14, qword ptr [rsi + 8]
0x1401648cd 488b4610 mov rax, qword ptr [rsi + 0x10]
0x1401648d1 48898558190000 mov qword ptr [rbp + 0x1958], rax
0x1401648d8 488b4618 mov rax, qword ptr [rsi + 0x18]
0x1401648dc 488985c0180000 mov qword ptr [rbp + 0x18c0], rax
0x1401648e3 488b4620 mov rax, qword ptr [rsi + 0x20]
0x1401648e7 488985c8180000 mov qword ptr [rbp + 0x18c8], rax
0x1401648ee 48c785b012000000000000 mov qword ptr [rbp + 0x12b0], 0
0x1401648f9 48c785b812000008000000 mov qword ptr [rbp + 0x12b8], 8
0x140164904 48c785c012000000000000 mov qword ptr [rbp + 0x12c0], 0
0x14016490f 4c89a5d8180000 mov qword ptr [rbp + 0x18d8], r12
0x140164916 4c89a5c8120000 mov qword ptr [rbp + 0x12c8], r12
0x14016491d 48899dd0120000 mov qword ptr [rbp + 0x12d0], rbx
0x140164924 4d8d6738 lea r12, [r15 + 0x38]
0x140164928 c6857519000001 mov byte ptr [rbp + 0x1975], 1
0x14016492f 48baaa4e7896b36185a6 movabs rdx, 0xa68561b396784eaa
0x140164939 49b814edf4fae674fdf5 movabs r8, 0xf5fd74e6faf4ed14
0x140164943 4c89e1 mov rcx, r12
0x140164946 e8a58df7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x14016494b 4885c0 test rax, rax
0x14016494e 0f84fb110000 je 0x140165b4f
0x140164954 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x14016495b 0f86ee110000 jbe 0x140165b4f
0x140164961 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x140164968 488b04d0 mov rax, qword ptr [rax + rdx*8]
0x14016496c 4885c0 test rax, rax
0x14016496f 0f84da110000 je 0x140165b4f
0x140164975 498b9fe0010000 mov rbx, qword ptr [r15 + 0x1e0]
0x14016497c 48f7d0 not rax
0x14016497f 488d0440 lea rax, [rax + rax*2]
0x140164983 48c1e005 shl rax, 5
0x140164987 48837c032000 cmp qword ptr [rbx + rax + 0x20], 0
0x14016498d 0f84bc110000 je 0x140165b4f
0x140164993 4801c3 add rbx, rax
0x140164996 4c8b6b10 mov r13, qword ptr [rbx + 0x10]
0x14016499a 418bb7b0050000 mov esi, dword ptr [r15 + 0x5b0]
0x1401649a1 89735c mov dword ptr [rbx + 0x5c], esi
0x1401649a4 c6857519000001 mov byte ptr [rbp + 0x1975], 1
0x1401649ab 488d9520120000 lea rdx, [rbp + 0x1220]
0x1401649b2 4c89e9 mov rcx, r13
0x1401649b5 e876837d00 call 0x14093cd30 ; country_core::asset_libraries::texture_library::AssetTextureLibrary::import_texture_presets_to_rg
0x1401649ba 89735c mov dword ptr [rbx + 0x5c], esi
0x1401649bd c6857519000001 mov byte ptr [rbp + 0x1975], 1
0x1401649c4 4c89e9 mov rcx, r13
0x1401649c7 4c89f2 mov rdx, r14
0x1401649ca e871867d00 call 0x14093d040 ; country_core::asset_libraries::texture_library::AssetTextureLibrary::bindless_textures_dset
0x1401649cf c6857519000001 mov byte ptr [rbp + 0x1975], 1
0x1401649d6 488d8d88180000 lea rcx, [rbp + 0x1888]
0x1401649dd 4889c2 mov rdx, rax
0x1401649e0 e81b21a700 call 0x140bd6b00 ; rhapsody::raw::managed::handle::impl$5::clone
0x1401649e5 c6857619000001 mov byte ptr [rbp + 0x1976], 1
0x1401649ec 48baca3b97c719dcd98f movabs rdx, 0x8fd9dc19c7973bca
0x1401649f6 49b8fd459bebc5f41f36 movabs r8, 0x361ff4c5eb9b45fd
0x140164a00 4c89e1 mov rcx, r12
0x140164a03 e8e88cf7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x140164a08 4885c0 test rax, rax
0x140164a0b 0f8456110000 je 0x140165b67
0x140164a11 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x140164a18 0f8649110000 jbe 0x140165b67
0x140164a1e 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x140164a25 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140164a29 4885c9 test rcx, rcx
0x140164a2c 0f8435110000 je 0x140165b67
0x140164a32 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x140164a39 48f7d1 not rcx
0x140164a3c 488d0c49 lea rcx, [rcx + rcx*2]
0x140164a40 48c1e105 shl rcx, 5
0x140164a44 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x140164a4a 0f8417110000 je 0x140165b67
0x140164a50 4801c8 add rax, rcx
0x140164a53 488b5010 mov rdx, qword ptr [rax + 0x10]
0x140164a57 488d8de0000000 lea rcx, [rbp + 0xe0]
0x140164a5e 41b890100000 mov r8d, 0x1090
0x140164a64 e897f17b02 call 0x142923c00 ; memcpy
0x140164a69 c6857619000001 mov byte ptr [rbp + 0x1976], 1
0x140164a70 48ba2ea78382b74cc947 movabs rdx, 0x47c94cb78283a72e
0x140164a7a 49b89d3e3297ed3d178c movabs r8, 0x8c173ded97323e9d
0x140164a84 4c89e1 mov rcx, r12
0x140164a87 e8648cf7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x140164a8c 4885c0 test rax, rax
0x140164a8f 0f84ea100000 je 0x140165b7f
0x140164a95 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x140164a9c 0f86dd100000 jbe 0x140165b7f
0x140164aa2 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x140164aa9 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140164aad 4885c9 test rcx, rcx
0x140164ab0 0f84c9100000 je 0x140165b7f
0x140164ab6 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x140164abd 48f7d1 not rcx
0x140164ac0 488d0c49 lea rcx, [rcx + rcx*2]
0x140164ac4 48c1e105 shl rcx, 5
0x140164ac8 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x140164ace 0f84ab100000 je 0x140165b7f
0x140164ad4 4801c8 add rax, rcx
0x140164ad7 488b4810 mov rcx, qword ptr [rax + 0x10]
0x140164adb c6857619000001 mov byte ptr [rbp + 0x1976], 1
0x140164ae2 e899925100 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x140164ae7 4889c6 mov rsi, rax
0x140164aea b101 mov cl, 1
0x140164aec 31c0 xor eax, eax
0x140164aee f00fb00e lock cmpxchg byte ptr [rsi], cl
0x140164af2 0f8511100000 jne 0x140165b09
0x140164af8 c6857619000001 mov byte ptr [rbp + 0x1976], 1
0x140164aff 4889f1 mov rcx, rsi
0x140164b02 e869d1b100 call 0x140c81c70 ; parking_lot_core::parking_lot::deadlock_impl::acquire_resource
0x140164b07 4889b560190000 mov qword ptr [rbp + 0x1960], rsi
0x140164b0e 4883c608 add rsi, 8
0x140164b12 48c744242000000000 mov qword ptr [rsp + 0x20], 0
0x140164b1b 488d8df0160000 lea rcx, [rbp + 0x16f0]
0x140164b22 41b990100000 mov r9d, 0x1090
0x140164b28 4889f2 mov rdx, rsi
0x140164b2b 4d89f0 mov r8, r14
0x140164b2e e80ddca900 call 0x140c02740 ; rhapsody::raw::dynamic_constants::DynamicConstants::allocate
0x140164b33 488b8df8160000 mov rcx, qword ptr [rbp + 0x16f8]
0x140164b3a 4881f990100000 cmp rcx, 0x1090
0x140164b41 0f85dc0f0000 jne 0x140165b23
0x140164b47 488b8df0160000 mov rcx, qword ptr [rbp + 0x16f0]
0x140164b4e 488d95e0000000 lea rdx, [rbp + 0xe0]
0x140164b55 41b890100000 mov r8d, 0x1090
0x140164b5b e8a0f07b02 call 0x142923c00 ; memcpy
0x140164b60 0f1085f0160000 movups xmm0, xmmword ptr [rbp + 0x16f0]
0x140164b67 0f108d00170000 movups xmm1, xmmword ptr [rbp + 0x1700]
0x140164b6e 0f109510170000 movups xmm2, xmmword ptr [rbp + 0x1710]
0x140164b75 0f109d20170000 movups xmm3, xmmword ptr [rbp + 0x1720]
0x140164b7c 0f298540140000 movaps xmmword ptr [rbp + 0x1440], xmm0
0x140164b83 0f298d50140000 movaps xmmword ptr [rbp + 0x1450], xmm1
0x140164b8a 0f299560140000 movaps xmmword ptr [rbp + 0x1460], xmm2
0x140164b91 0f299d70140000 movaps xmmword ptr [rbp + 0x1470], xmm3
0x140164b98 488b8530170000 mov rax, qword ptr [rbp + 0x1730]
0x140164b9f 48898580140000 mov qword ptr [rbp + 0x1480], rax
0x140164ba6 488d8d50130000 lea rcx, [rbp + 0x1350]
0x140164bad 488d9540140000 lea rdx, [rbp + 0x1440]
0x140164bb4 4d89f0 mov r8, r14
0x140164bb7 e8b4d7a900 call 0x140c02370 ; rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_and_storage_buffer_binding
0x140164bbc c6857019000001 mov byte ptr [rbp + 0x1970], 1
0x140164bc3 488bb560190000 mov rsi, qword ptr [rbp + 0x1960]
0x140164bca 4889f1 mov rcx, rsi
0x140164bcd e83ed2b100 call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
0x140164bd2 31c9 xor ecx, ecx
0x140164bd4 b001 mov al, 1
0x140164bd6 f00fb00e lock cmpxchg byte ptr [rsi], cl
0x140164bda 0f85590f0000 jne 0x140165b39
0x140164be0 488d8d40140000 lea rcx, [rbp + 0x1440]
0x140164be7 488d9520120000 lea rdx, [rbp + 0x1220]
0x140164bee 41b810010000 mov r8d, 0x110
0x140164bf4 e807f07b02 call 0x142923c00 ; memcpy
0x140164bf9 0f108550130000 movups xmm0, xmmword ptr [rbp + 0x1350]
0x140164c00 0f108d60130000 movups xmm1, xmmword ptr [rbp + 0x1360]
0x140164c07 0f109570130000 movups xmm2, xmmword ptr [rbp + 0x1370]
0x140164c0e 0f109d80130000 movups xmm3, xmmword ptr [rbp + 0x1380]
0x140164c15 0f118550150000 movups xmmword ptr [rbp + 0x1550], xmm0
0x140164c1c 0f118d60150000 movups xmmword ptr [rbp + 0x1560], xmm1
0x140164c23 0f119570150000 movups xmmword ptr [rbp + 0x1570], xmm2
0x140164c2a 0f119d80150000 movups xmmword ptr [rbp + 0x1580], xmm3
0x140164c31 0f108590130000 movups xmm0, xmmword ptr [rbp + 0x1390]
0x140164c38 0f118590150000 movups xmmword ptr [rbp + 0x1590], xmm0
0x140164c3f 0f1085a0130000 movups xmm0, xmmword ptr [rbp + 0x13a0]
0x140164c46 0f1185a0150000 movups xmmword ptr [rbp + 0x15a0], xmm0
0x140164c4d 0f108588180000 movups xmm0, xmmword ptr [rbp + 0x1888]
0x140164c54 0f1185b0150000 movups xmmword ptr [rbp + 0x15b0], xmm0
0x140164c5b 488b8598180000 mov rax, qword ptr [rbp + 0x1898]
0x140164c62 488985c0150000 mov qword ptr [rbp + 0x15c0], rax
0x140164c69 488b8558190000 mov rax, qword ptr [rbp + 0x1958]
0x140164c70 488985c8150000 mov qword ptr [rbp + 0x15c8], rax
0x140164c77 4c89b5d0150000 mov qword ptr [rbp + 0x15d0], r14
0x140164c7e 803d61644c0308 cmp byte ptr [rip + 0x34c6461], 8
0x140164c85 0f8591040000 jne 0x14016511c
0x140164c8b c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x140164c92 48ba1092b0123115a54d movabs rdx, 0x4da5153112b09210
0x140164c9c 49b8f15a204edb7e805b movabs r8, 0x5b807edb4e205af1
0x140164ca6 4c89e1 mov rcx, r12
0x140164ca9 e8428af7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x140164cae 4885c0 test rax, rax
0x140164cb1 0f84e7030000 je 0x14016509e
0x140164cb7 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x140164cbe 0f86da030000 jbe 0x14016509e
0x140164cc4 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x140164ccb 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140164ccf 4885c9 test rcx, rcx
0x140164cd2 0f84c6030000 je 0x14016509e
0x140164cd8 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x140164cdf 48f7d1 not rcx
0x140164ce2 488d0c49 lea rcx, [rcx + rcx*2]
0x140164ce6 48c1e105 shl rcx, 5
0x140164cea 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x140164cf0 0f84a8030000 je 0x14016509e
0x140164cf6 c6856019000007 mov byte ptr [rbp + 0x1960], 7
0x140164cfd 4801c8 add rax, rcx
0x140164d00 488b4010 mov rax, qword ptr [rax + 0x10]
0x140164d04 4883781000 cmp qword ptr [rax + 0x10], 0
0x140164d09 0f8496030000 je 0x1401650a5
0x140164d0f 488b5008 mov rdx, qword ptr [rax + 8]
0x140164d13 c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x140164d1a 488d8d30130000 lea rcx, [rbp + 0x1330]
0x140164d21 e82a1d9e00 call 0x140b46a50 ; country_core::resources::session::SavedSession::loading_background_path
0x140164d26 c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x140164d2d 488d8df0160000 lea rcx, [rbp + 0x16f0]
0x140164d34 488d9530130000 lea rdx, [rbp + 0x1330]
0x140164d3b e8301cf6ff call 0x1400c6970 ; image::dynimage::open<std::path::PathBuf>
0x140164d40 488b85f0160000 mov rax, qword ptr [rbp + 0x16f0]
0x140164d47 48b90a00000000000080 movabs rcx, 0x800000000000000a
0x140164d51 4839c8 cmp rax, rcx
0x140164d54 0f85450c0000 jne 0x14016599f
0x140164d5a 488b85f8160000 mov rax, qword ptr [rbp + 0x16f8]
0x140164d61 0f108500170000 movups xmm0, xmmword ptr [rbp + 0x1700]
0x140164d68 0f298520190000 movaps xmmword ptr [rbp + 0x1920], xmm0
0x140164d6f 0f108510170000 movups xmm0, xmmword ptr [rbp + 0x1710]
0x140164d76 0f298530190000 movaps xmmword ptr [rbp + 0x1930], xmm0
0x140164d7d 4883f80a cmp rax, 0xa
0x140164d81 0f841e030000 je 0x1401650a5
0x140164d87 0f288520190000 movaps xmm0, xmmword ptr [rbp + 0x1920]
0x140164d8e 0f288d30190000 movaps xmm1, xmmword ptr [rbp + 0x1930]
0x140164d95 0f118df8150000 movups xmmword ptr [rbp + 0x15f8], xmm1
0x140164d9c 0f1185e8150000 movups xmmword ptr [rbp + 0x15e8], xmm0
0x140164da3 488985e0150000 mov qword ptr [rbp + 0x15e0], rax
0x140164daa 488d8da0180000 lea rcx, [rbp + 0x18a0]
0x140164db1 488d95e0150000 lea rdx, [rbp + 0x15e0]
0x140164db8 e8138ba400 call 0x140bad8d0 ; image::dynimage::DynamicImage::to_rgba8
0x140164dbd 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x140164dc4 e897ceffff call 0x140161c60 ; core::ptr::drop_in_place<enum2$<image::dynimage::DynamicImage> >
0x140164dc9 488b95b8180000 mov rdx, qword ptr [rbp + 0x18b8]
0x140164dd0 c6856f19000001 mov byte ptr [rbp + 0x196f], 1
0x140164dd7 488d8df0180000 lea rcx, [rbp + 0x18f0]
0x140164dde 41b82b000000 mov r8d, 0x2b
0x140164de4 e8b704a700 call 0x140bd52a0 ; rhapsody::raw::texture::TextureDesc::new_2d
0x140164de9 c6856f19000001 mov byte ptr [rbp + 0x196f], 1
0x140164df0 4c8dad00120000 lea r13, [rbp + 0x1200]
0x140164df7 488d95f0180000 lea rdx, [rbp + 0x18f0]
0x140164dfe 4c89e9 mov rcx, r13
0x140164e01 e86a05a700 call 0x140bd5370 ; rhapsody::raw::texture::TextureDesc::sampled
0x140164e06 488b85a0180000 mov rax, qword ptr [rbp + 0x18a0]
0x140164e0d 48898558190000 mov qword ptr [rbp + 0x1958], rax
0x140164e14 488b8da8180000 mov rcx, qword ptr [rbp + 0x18a8]
0x140164e1b 488b85b0180000 mov rax, qword ptr [rbp + 0x18b0]
0x140164e22 4889442430 mov qword ptr [rsp + 0x30], rax
0x140164e27 48898d18190000 mov qword ptr [rbp + 0x1918], rcx
0x140164e2e 48894c2428 mov qword ptr [rsp + 0x28], rcx
0x140164e33 4c896c2420 mov qword ptr [rsp + 0x20], r13
0x140164e38 4c8d05e18b7f02 lea r8, [rip + 0x27f8be1]
0x140164e3f 488d8df0160000 lea rcx, [rbp + 0x16f0]
0x140164e46 41b910000000 mov r9d, 0x10
0x140164e4c 4c89f2 mov rdx, r14
0x140164e4f e8ac4c4500 call 0x1405b9b00 ; rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > >
0x140164e54 488b85f0160000 mov rax, qword ptr [rbp + 0x16f0]
0x140164e5b 4883f80b cmp rax, 0xb
0x140164e5f 0f85860b0000 jne 0x1401659eb
0x140164e65 488b8526170000 mov rax, qword ptr [rbp + 0x1726]
0x140164e6c 4889850e160000 mov qword ptr [rbp + 0x160e], rax
0x140164e73 0f1085f8160000 movups xmm0, xmmword ptr [rbp + 0x16f8]
0x140164e7a 0f108d08170000 movups xmm1, xmmword ptr [rbp + 0x1708]
0x140164e81 0f109518170000 movups xmm2, xmmword ptr [rbp + 0x1718]
0x140164e88 0f299500160000 movaps xmmword ptr [rbp + 0x1600], xmm2
0x140164e8f 0f298df0150000 movaps xmmword ptr [rbp + 0x15f0], xmm1
0x140164e96 0f2985e0150000 movaps xmmword ptr [rbp + 0x15e0], xmm0
0x140164e9d 0fb6852e170000 movzx eax, byte ptr [rbp + 0x172e]
0x140164ea4 3c07 cmp al, 7
0x140164ea6 0f84360c0000 je 0x140165ae2
0x140164eac 0fb68d2f170000 movzx ecx, byte ptr [rbp + 0x172f]
0x140164eb3 488b950e160000 mov rdx, qword ptr [rbp + 0x160e]
0x140164eba 4889954e190000 mov qword ptr [rbp + 0x194e], rdx
0x140164ec1 0f2885e0150000 movaps xmm0, xmmword ptr [rbp + 0x15e0]
0x140164ec8 0f288df0150000 movaps xmm1, xmmword ptr [rbp + 0x15f0]
0x140164ecf 0f289500160000 movaps xmm2, xmmword ptr [rbp + 0x1600]
0x140164ed6 0f299540190000 movaps xmmword ptr [rbp + 0x1940], xmm2
0x140164edd 0f298d30190000 movaps xmmword ptr [rbp + 0x1930], xmm1
0x140164ee4 0f298520190000 movaps xmmword ptr [rbp + 0x1920], xmm0
0x140164eeb 888556190000 mov byte ptr [rbp + 0x1956], al
0x140164ef1 888d57190000 mov byte ptr [rbp + 0x1957], cl
0x140164ef7 488b9558190000 mov rdx, qword ptr [rbp + 0x1958]
0x140164efe 4885d2 test rdx, rdx
0x140164f01 7412 je 0x140164f15
0x140164f03 41b801000000 mov r8d, 1
0x140164f09 488b8d18190000 mov rcx, qword ptr [rbp + 0x1918]
0x140164f10 e80bed4a00 call 0x140613c20 ; __rust_dealloc
0x140164f15 48ba3febbf685e1584a2 movabs rdx, 0xa284155e68bfeb3f
0x140164f1f 49b83a4ac61b44b3a284 movabs r8, 0x84a2b3441bc64a3a
0x140164f29 4c89e1 mov rcx, r12
0x140164f2c e8bf87f7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x140164f31 4885c0 test rax, rax
0x140164f34 0f84f20c0000 je 0x140165c2c
0x140164f3a 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x140164f41 0f86e50c0000 jbe 0x140165c2c
0x140164f47 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x140164f4e 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140164f52 4885c9 test rcx, rcx
0x140164f55 0f84d10c0000 je 0x140165c2c
0x140164f5b 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x140164f62 48f7d1 not rcx
0x140164f65 488d0c49 lea rcx, [rcx + rcx*2]
0x140164f69 48c1e105 shl rcx, 5
0x140164f6d 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x140164f73 0f84b30c0000 je 0x140165c2c
0x140164f79 4801c8 add rax, rcx
0x140164f7c 488b7010 mov rsi, qword ptr [rax + 0x10]
0x140164f80 418b8fb0050000 mov ecx, dword ptr [r15 + 0x5b0]
0x140164f87 89485c mov dword ptr [rax + 0x5c], ecx
0x140164f8a 837e4802 cmp dword ptr [rsi + 0x48], 2
0x140164f8e 0f84a60c0000 je 0x140165c3a
0x140164f94 488d8df0160000 lea rcx, [rbp + 0x16f0]
0x140164f9b 488d9520190000 lea rdx, [rbp + 0x1920]
0x140164fa2 e8591ba700 call 0x140bd6b00 ; rhapsody::raw::managed::handle::impl$5::clone
0x140164fa7 0f108538190000 movups xmm0, xmmword ptr [rbp + 0x1938]
0x140164fae 0f108d48190000 movups xmm1, xmmword ptr [rbp + 0x1948]
0x140164fb5 0f118508170000 movups xmmword ptr [rbp + 0x1708], xmm0
0x140164fbc 0f118d18170000 movups xmmword ptr [rbp + 0x1718], xmm1
0x140164fc3 0f1085f0160000 movups xmm0, xmmword ptr [rbp + 0x16f0]
0x140164fca 0f2985e0150000 movaps xmmword ptr [rbp + 0x15e0], xmm0
0x140164fd1 0f108500170000 movups xmm0, xmmword ptr [rbp + 0x1700]
0x140164fd8 0f2985f0150000 movaps xmmword ptr [rbp + 0x15f0], xmm0
0x140164fdf 0f108510170000 movups xmm0, xmmword ptr [rbp + 0x1710]
0x140164fe6 0f298500160000 movaps xmmword ptr [rbp + 0x1600], xmm0
0x140164fed 488b8520170000 mov rax, qword ptr [rbp + 0x1720]
0x140164ff4 48898510160000 mov qword ptr [rbp + 0x1610], rax
0x140164ffb 488d4608 lea rax, [rsi + 8]
0x140164fff 48898560190000 mov qword ptr [rbp + 0x1960], rax
0x140165006 807e3e07 cmp byte ptr [rsi + 0x3e], 7
0x14016500a 740c je 0x140165018
0x14016500c 488b8d60190000 mov rcx, qword ptr [rbp + 0x1960]
0x140165013 e8c8954800 call 0x1405ee5e0 ; core::ptr::drop_in_place<rhapsody::raw::texture::Texture>
0x140165018 488b8510160000 mov rax, qword ptr [rbp + 0x1610]
0x14016501f 488b8d60190000 mov rcx, qword ptr [rbp + 0x1960]
0x140165026 48894130 mov qword ptr [rcx + 0x30], rax
0x14016502a 0f2885e0150000 movaps xmm0, xmmword ptr [rbp + 0x15e0]
0x140165031 0f288df0150000 movaps xmm1, xmmword ptr [rbp + 0x15f0]
0x140165038 0f289500160000 movaps xmm2, xmmword ptr [rbp + 0x1600]
0x14016503f 0f115120 movups xmmword ptr [rcx + 0x20], xmm2
0x140165043 0f114910 movups xmmword ptr [rcx + 0x10], xmm1
0x140165047 0f1101 movups xmmword ptr [rcx], xmm0
0x14016504a 0f288520190000 movaps xmm0, xmmword ptr [rbp + 0x1920]
0x140165051 0f288d30190000 movaps xmm1, xmmword ptr [rbp + 0x1930]
0x140165058 0f289540190000 movaps xmm2, xmmword ptr [rbp + 0x1940]
0x14016505f 0f2985f0160000 movaps xmmword ptr [rbp + 0x16f0], xmm0
0x140165066 0f298d00170000 movaps xmmword ptr [rbp + 0x1700], xmm1
0x14016506d 0f299510170000 movaps xmmword ptr [rbp + 0x1710], xmm2
0x140165074 488b854e190000 mov rax, qword ptr [rbp + 0x194e]
0x14016507b 4889851e170000 mov qword ptr [rbp + 0x171e], rax
0x140165082 0fb68556190000 movzx eax, byte ptr [rbp + 0x1956]
0x140165089 888560190000 mov byte ptr [rbp + 0x1960], al
0x14016508f 0fb68557190000 movzx eax, byte ptr [rbp + 0x1957]
0x140165096 888558190000 mov byte ptr [rbp + 0x1958], al
0x14016509c eb07 jmp 0x1401650a5
0x14016509e c6856019000007 mov byte ptr [rbp + 0x1960], 7
0x1401650a5 0fb6053a604c03 movzx eax, byte ptr [rip + 0x34c603a]
0x1401650ac 04f9 add al, 0xf9
0x1401650ae 3c02 cmp al, 2
0x1401650b0 7218 jb 0x1401650ca
0x1401650b2 488d0df75f4c03 lea rcx, [rip + 0x34c5ff7]
0x1401650b9 e8c26aedff call 0x14003bb80 ; core::error::Error::provide<enum2$<bevy_ecs::world::error::EntityMutableFetchError> >
0x1401650be 488d0deb5f4c03 lea rcx, [rip + 0x34c5feb]
0x1401650c5 e8e6d3a700 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
0x1401650ca 488b851e170000 mov rax, qword ptr [rbp + 0x171e]
0x1401650d1 48890506604c03 mov qword ptr [rip + 0x34c6006], rax
0x1401650d8 0f2885f0160000 movaps xmm0, xmmword ptr [rbp + 0x16f0]
0x1401650df 0f288d00170000 movaps xmm1, xmmword ptr [rbp + 0x1700]
0x1401650e6 0f289510170000 movaps xmm2, xmmword ptr [rbp + 0x1710]
0x1401650ed 0f1115dc5f4c03 movups xmmword ptr [rip + 0x34c5fdc], xmm2
0x1401650f4 0f110dc55f4c03 movups xmmword ptr [rip + 0x34c5fc5], xmm1
0x1401650fb 0f1105ae5f4c03 movups xmmword ptr [rip + 0x34c5fae], xmm0
0x140165102 0fb68560190000 movzx eax, byte ptr [rbp + 0x1960]
0x140165109 8805d75f4c03 mov byte ptr [rip + 0x34c5fd7], al
0x14016510f 0fb68558190000 movzx eax, byte ptr [rbp + 0x1958]
0x140165116 8805cb5f4c03 mov byte ptr [rip + 0x34c5fcb], al
0x14016511c 488b85c0180000 mov rax, qword ptr [rbp + 0x18c0]
0x140165123 488b10 mov rdx, qword ptr [rax]
0x140165126 c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x14016512d 488d8df0180000 lea rcx, [rbp + 0x18f0]
0x140165134 41b82b000000 mov r8d, 0x2b
0x14016513a e86101a700 call 0x140bd52a0 ; rhapsody::raw::texture::TextureDesc::new_2d
0x14016513f 0f1085f0180000 movups xmm0, xmmword ptr [rbp + 0x18f0]
0x140165146 0f108d00190000 movups xmm1, xmmword ptr [rbp + 0x1900]
0x14016514d 0f298df0150000 movaps xmmword ptr [rbp + 0x15f0], xmm1
0x140165154 0f2985e0150000 movaps xmmword ptr [rbp + 0x15e0], xmm0
0x14016515b 488d8d00170000 lea rcx, [rbp + 0x1700]
0x140165162 c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x140165169 488d95e0150000 lea rdx, [rbp + 0x15e0]
0x140165170 e86b1ba700 call 0x140bd6ce0 ; rhapsody::rg::resource::impl$4::from
0x140165175 488d05f4877f02 lea rax, [rip + 0x27f87f4]
0x14016517c 488985f0160000 mov qword ptr [rbp + 0x16f0], rax
0x140165183 48c785f81600000e000000 mov qword ptr [rbp + 0x16f8], 0xe
0x14016518e c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x140165195 488d8d40140000 lea rcx, [rbp + 0x1440]
0x14016519c 488d95f0160000 lea rdx, [rbp + 0x16f0]
0x1401651a3 e82843a800 call 0x140be94d0 ; rhapsody::rg::graph::RenderGraph::create_raw_resource
0x1401651a8 0f1085f0180000 movups xmm0, xmmword ptr [rbp + 0x18f0]
0x1401651af 0f108d00190000 movups xmm1, xmmword ptr [rbp + 0x1900]
0x1401651b6 0f118524190000 movups xmmword ptr [rbp + 0x1924], xmm0
0x1401651bd 0f118d34190000 movups xmmword ptr [rbp + 0x1934], xmm1
0x1401651c4 898520190000 mov dword ptr [rbp + 0x1920], eax
0x1401651ca c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x1401651d1 48bad1c2cc5c9da79c10 movabs rdx, 0x109ca79d5cccc2d1
0x1401651db 49b80c84c93018bcea24 movabs r8, 0x24eabc1830c9840c
0x1401651e5 4c89e1 mov rcx, r12
0x1401651e8 e80385f7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1401651ed 4885c0 test rax, rax
0x1401651f0 0f84a1090000 je 0x140165b97
0x1401651f6 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x1401651fd 0f8694090000 jbe 0x140165b97
0x140165203 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x14016520a 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x14016520e 4885c9 test rcx, rcx
0x140165211 0f8480090000 je 0x140165b97
0x140165217 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x14016521e 48f7d1 not rcx
0x140165221 488d0c49 lea rcx, [rcx + rcx*2]
0x140165225 48c1e105 shl rcx, 5
0x140165229 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x14016522f 0f8462090000 je 0x140165b97
0x140165235 4801c8 add rax, rcx
0x140165238 488b4010 mov rax, qword ptr [rax + 0x10]
0x14016523c f30f107058 movss xmm6, dword ptr [rax + 0x58]
0x140165241 c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x140165248 48ba3febbf685e1584a2 movabs rdx, 0xa284155e68bfeb3f
0x140165252 49b83a4ac61b44b3a284 movabs r8, 0x84a2b3441bc64a3a
0x14016525c 4c89e1 mov rcx, r12
0x14016525f e88c84f7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x140165264 4885c0 test rax, rax
0x140165267 0f8442090000 je 0x140165baf
0x14016526d 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x140165274 0f8635090000 jbe 0x140165baf
0x14016527a 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x140165281 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140165285 4885c9 test rcx, rcx
0x140165288 0f8421090000 je 0x140165baf
0x14016528e 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x140165295 48f7d1 not rcx
0x140165298 488d0c49 lea rcx, [rcx + rcx*2]
0x14016529c 48c1e105 shl rcx, 5
0x1401652a0 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1401652a6 0f8403090000 je 0x140165baf
0x1401652ac 4801c8 add rax, rcx
0x1401652af 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1401652b3 418b97b0050000 mov edx, dword ptr [r15 + 0x5b0]
0x1401652ba 89505c mov dword ptr [rax + 0x5c], edx
0x1401652bd 83794802 cmp dword ptr [rcx + 0x48], 2
0x1401652c1 0f8436090000 je 0x140165bfd
0x1401652c7 f30f5d35b1827f02 minss xmm6, dword ptr [rip + 0x27f82b1]
0x1401652cf f30f587144 addss xmm6, dword ptr [rcx + 0x44]
0x1401652d4 f30f117144 movss dword ptr [rcx + 0x44], xmm6
0x1401652d9 31c0 xor eax, eax
0x1401652db 803d045e4c0307 cmp byte ptr [rip + 0x34c5e04], 7
0x1401652e2 4c8d05c75d4c03 lea r8, [rip + 0x34c5dc7]
0x1401652e9 4c0f44c0 cmove r8, rax
0x1401652ed c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x1401652f4 f30f11742428 movss dword ptr [rsp + 0x28], xmm6
0x1401652fa 488d8520190000 lea rax, [rbp + 0x1920]
0x140165301 4889442420 mov qword ptr [rsp + 0x20], rax
0x140165306 c644244801 mov byte ptr [rsp + 0x48], 1
0x14016530b c644244000 mov byte ptr [rsp + 0x40], 0
0x140165310 c644243800 mov byte ptr [rsp + 0x38], 0
0x140165315 c744243000000000 mov dword ptr [rsp + 0x30], 0
0x14016531d 488d8d40140000 lea rcx, [rbp + 0x1440]
0x140165324 4c89fa mov rdx, r15
0x140165327 4531c9 xor r9d, r9d
0x14016532a e8413a7400 call 0x1408a8d70 ; country_core::systems::render_loop::vermeer::vermeer_render_from_target
0x14016532f c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x140165336 488d8d40140000 lea rcx, [rbp + 0x1440]
0x14016533d 4c8d8520190000 lea r8, [rbp + 0x1920]
0x140165344 4c89fa mov rdx, r15
0x140165347 4531c9 xor r9d, r9d
0x14016534a e8b1138600 call 0x1409c6700 ; country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass
0x14016534f c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x140165356 488d8d40140000 lea rcx, [rbp + 0x1440]
0x14016535d 4c8d8520190000 lea r8, [rbp + 0x1920]
0x140165364 4c89fa mov rdx, r15
0x140165367 4c8b8dc8180000 mov r9, qword ptr [rbp + 0x18c8]
0x14016536e e87d2c6d00 call 0x140837ff0 ; country_core::systems::render_loop::draw_passes::ui_pass::loading_screen_pass
0x140165373 0f108520190000 movups xmm0, xmmword ptr [rbp + 0x1920]
0x14016537a 0f108d30190000 movups xmm1, xmmword ptr [rbp + 0x1930]
0x140165381 0f2985f0160000 movaps xmmword ptr [rbp + 0x16f0], xmm0
0x140165388 8b8540190000 mov eax, dword ptr [rbp + 0x1940]
0x14016538e 898510170000 mov dword ptr [rbp + 0x1710], eax
0x140165394 0f298d00170000 movaps xmmword ptr [rbp + 0x1700], xmm1
0x14016539b 448badf0160000 mov r13d, dword ptr [rbp + 0x16f0]
0x1401653a2 c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x1401653a9 488d8df0160000 lea rcx, [rbp + 0x16f0]
0x1401653b0 488d9540140000 lea rdx, [rbp + 0x1440]
0x1401653b7 41b022 mov r8b, 0x22
0x1401653ba e8c10fa700 call 0x140bd6380 ; rhapsody::rg::import_export::impl$0::export
0x1401653bf 4c8db5f0160000 lea r14, [rbp + 0x16f0]
0x1401653c6 488d9540140000 lea rdx, [rbp + 0x1440]
0x1401653cd 41b898010000 mov r8d, 0x198
0x1401653d3 4c89f1 mov rcx, r14
0x1401653d6 e825e87b02 call 0x142923c00 ; memcpy
0x1401653db c6857719000000 mov byte ptr [rbp + 0x1977], 0
0x1401653e2 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x1401653e9 4c89f2 mov rdx, r14
0x1401653ec e85f868700 call 0x1409dda50 ; country_core::systems::render_loop::RenderContext::finish
0x1401653f1 48baaa4e7896b36185a6 movabs rdx, 0xa68561b396784eaa
0x1401653fb 49b814edf4fae674fdf5 movabs r8, 0xf5fd74e6faf4ed14
0x140165405 4c89e1 mov rcx, r12
0x140165408 e8e382f7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x14016540d 4885c0 test rax, rax
0x140165410 0f84b1070000 je 0x140165bc7
0x140165416 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x14016541d 0f86a4070000 jbe 0x140165bc7
0x140165423 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x14016542a 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x14016542e 4885c9 test rcx, rcx
0x140165431 0f8490070000 je 0x140165bc7
0x140165437 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x14016543e 48f7d1 not rcx
0x140165441 488d0c49 lea rcx, [rcx + rcx*2]
0x140165445 48c1e105 shl rcx, 5
0x140165449 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x14016544f 0f8472070000 je 0x140165bc7
0x140165455 4801c8 add rax, rcx
0x140165458 488b4810 mov rcx, qword ptr [rax + 0x10]
0x14016545c 418b97b0050000 mov edx, dword ptr [r15 + 0x5b0]
0x140165463 89505c mov dword ptr [rax + 0x5c], edx
0x140165466 488d95e0150000 lea rdx, [rbp + 0x15e0]
0x14016546d e8be797d00 call 0x14093ce30 ; country_core::asset_libraries::texture_library::AssetTextureLibrary::export_preset_textures_from_rg
0x140165472 48ba5593676e78cc2682 movabs rdx, 0x8226cc786e679355
0x14016547c 49b8ac41a8556adacf65 movabs r8, 0x65cfda6a55a841ac
0x140165486 4c89e1 mov rcx, r12
0x140165489 e86282f7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x14016548e 4885c0 test rax, rax
0x140165491 0f843e070000 je 0x140165bd5
0x140165497 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x14016549e 0f8631070000 jbe 0x140165bd5
0x1401654a4 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x1401654ab 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1401654af 4885c9 test rcx, rcx
0x1401654b2 0f841d070000 je 0x140165bd5
0x1401654b8 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x1401654bf 48f7d1 not rcx
0x1401654c2 488d0c49 lea rcx, [rcx + rcx*2]
0x1401654c6 48c1e105 shl rcx, 5
0x1401654ca 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1401654d0 0f84ff060000 je 0x140165bd5
0x1401654d6 4801c8 add rax, rcx
0x1401654d9 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1401654dd 418b97b0050000 mov edx, dword ptr [r15 + 0x5b0]
0x1401654e4 89505c mov dword ptr [rax + 0x5c], edx
0x1401654e7 488d95e0150000 lea rdx, [rbp + 0x15e0]
0x1401654ee e8bd967b00 call 0x14091ebb0 ; country_core::asset_libraries::ssbo_library::AssetSsboLibrary::export_from_rg
0x1401654f3 4c8d75d0 lea r14, [rbp - 0x30]
0x1401654f7 488d95e0150000 lea rdx, [rbp + 0x15e0]
0x1401654fe 41b810010000 mov r8d, 0x110
0x140165504 4c89f1 mov rcx, r14
0x140165507 e8f4e67b02 call 0x142923c00 ; memcpy
0x14016550c 488d8de0000000 lea rcx, [rbp + 0xe0]
0x140165513 4c89f2 mov rdx, r14
0x140165516 e885273900 call 0x1404f7ca0 ; rhapsody::rg::temporal::TemporalRenderGraphRecorder::export_temporal<country_core::render::pass_builder::pass_resources::PassResources>
0x14016551b 488db790000000 lea rsi, [rdi + 0x90]
0x140165522 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x140165529 488d95e0000000 lea rdx, [rbp + 0xe0]
0x140165530 41b8a8000000 mov r8d, 0xa8
0x140165536 e8c5e67b02 call 0x142923c00 ; memcpy
0x14016553b 0f108588010000 movups xmm0, xmmword ptr [rbp + 0x188]
0x140165542 0f108d98010000 movups xmm1, xmmword ptr [rbp + 0x198]
0x140165549 0f1095a8010000 movups xmm2, xmmword ptr [rbp + 0x1a8]
0x140165550 0f109db8010000 movups xmm3, xmmword ptr [rbp + 0x1b8]
0x140165557 0f2985e0130000 movaps xmmword ptr [rbp + 0x13e0], xmm0
0x14016555e 0f298df0130000 movaps xmmword ptr [rbp + 0x13f0], xmm1
0x140165565 0f299500140000 movaps xmmword ptr [rbp + 0x1400], xmm2
0x14016556c 0f299d10140000 movaps xmmword ptr [rbp + 0x1410], xmm3
0x140165573 0f1085c8010000 movups xmm0, xmmword ptr [rbp + 0x1c8]
0x14016557a 0f298520140000 movaps xmmword ptr [rbp + 0x1420], xmm0
0x140165581 0f1085d8010000 movups xmm0, xmmword ptr [rbp + 0x1d8]
0x140165588 0f298530140000 movaps xmmword ptr [rbp + 0x1430], xmm0
0x14016558f 0f104650 movups xmm0, xmmword ptr [rsi + 0x50]
0x140165593 0f298530010000 movaps xmmword ptr [rbp + 0x130], xmm0
0x14016559a 0f104640 movups xmm0, xmmword ptr [rsi + 0x40]
0x14016559e 0f298520010000 movaps xmmword ptr [rbp + 0x120], xmm0
0x1401655a5 0f1006 movups xmm0, xmmword ptr [rsi]
0x1401655a8 0f104e10 movups xmm1, xmmword ptr [rsi + 0x10]
0x1401655ac 0f105620 movups xmm2, xmmword ptr [rsi + 0x20]
0x1401655b0 0f105e30 movups xmm3, xmmword ptr [rsi + 0x30]
0x1401655b4 0f299d10010000 movaps xmmword ptr [rbp + 0x110], xmm3
0x1401655bb 0f299500010000 movaps xmmword ptr [rbp + 0x100], xmm2
0x1401655c2 0f298df0000000 movaps xmmword ptr [rbp + 0xf0], xmm1
0x1401655c9 0f2985e0000000 movaps xmmword ptr [rbp + 0xe0], xmm0
0x1401655d0 0f2885e0130000 movaps xmm0, xmmword ptr [rbp + 0x13e0]
0x1401655d7 0f288df0130000 movaps xmm1, xmmword ptr [rbp + 0x13f0]
0x1401655de 0f289500140000 movaps xmm2, xmmword ptr [rbp + 0x1400]
0x1401655e5 0f289d10140000 movaps xmm3, xmmword ptr [rbp + 0x1410]
0x1401655ec 0f1106 movups xmmword ptr [rsi], xmm0
0x1401655ef 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
0x1401655f3 0f115620 movups xmmword ptr [rsi + 0x20], xmm2
0x1401655f7 0f115e30 movups xmmword ptr [rsi + 0x30], xmm3
0x1401655fb 0f288520140000 movaps xmm0, xmmword ptr [rbp + 0x1420]
0x140165602 0f114640 movups xmmword ptr [rsi + 0x40], xmm0
0x140165606 0f288530140000 movaps xmm0, xmmword ptr [rbp + 0x1430]
0x14016560d 0f114650 movups xmmword ptr [rsi + 0x50], xmm0
0x140165611 4883bde000000000 cmp qword ptr [rbp + 0xe0], 0
0x140165619 7457 je 0x140165672
0x14016561b 488d9500010000 lea rdx, [rbp + 0x100]
0x140165622 488d8de0000000 lea rcx, [rbp + 0xe0]
0x140165629 41b868000000 mov r8d, 0x68
0x14016562f 41b910000000 mov r9d, 0x10
0x140165635 e87678eeff call 0x14004ceb0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>,alloc::alloc::Global>
0x14016563a 488b9518010000 mov rdx, qword ptr [rbp + 0x118]
0x140165641 4885d2 test rdx, rdx
0x140165644 742c je 0x140165672
0x140165646 4889d0 mov rax, rdx
0x140165649 48c1e004 shl rax, 4
0x14016564d 4801c2 add rdx, rax
0x140165650 4883c221 add rdx, 0x21
0x140165654 741c je 0x140165672
0x140165656 488b8d10010000 mov rcx, qword ptr [rbp + 0x110]
0x14016565d 48f7d8 neg rax
0x140165660 4801c1 add rcx, rax
0x140165663 4883c1f0 add rcx, -0x10
0x140165667 41b810000000 mov r8d, 0x10
0x14016566d e8aee54a00 call 0x140613c20 ; __rust_dealloc
0x140165672 488d8d50130000 lea rcx, [rbp + 0x1350]
0x140165679 488d95e0150000 lea rdx, [rbp + 0x15e0]
0x140165680 41b890000000 mov r8d, 0x90
0x140165686 e875e57b02 call 0x142923c00 ; memcpy
0x14016568b 488b8580160000 mov rax, qword ptr [rbp + 0x1680]
0x140165692 48898500190000 mov qword ptr [rbp + 0x1900], rax
0x140165699 0f108570160000 movups xmm0, xmmword ptr [rbp + 0x1670]
0x1401656a0 0f2985f0180000 movaps xmmword ptr [rbp + 0x18f0], xmm0
0x1401656a7 48c785e000000000000000 mov qword ptr [rbp + 0xe0], 0
0x1401656b2 48c785e800000008000000 mov qword ptr [rbp + 0xe8], 8
0x1401656bd 48c785f000000000000000 mov qword ptr [rbp + 0xf0], 0
0x1401656c8 31c9 xor ecx, ecx
0x1401656ca e85144f7ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
0x1401656cf 4885c0 test rax, rax
0x1401656d2 0f8446050000 je 0x140165c1e
0x1401656d8 0f1000 movups xmm0, xmmword ptr [rax]
0x1401656db 48ff00 inc qword ptr [rax]
0x1401656de 0f100d4ba98402 movups xmm1, xmmword ptr [rip + 0x284a94b]
0x1401656e5 0f118d28150000 movups xmmword ptr [rbp + 0x1528], xmm1
0x1401656ec 0f100d2da98402 movups xmm1, xmmword ptr [rip + 0x284a92d]
0x1401656f3 0f118d18150000 movups xmmword ptr [rbp + 0x1518], xmm1
0x1401656fa 0f118538150000 movups xmmword ptr [rbp + 0x1538], xmm0
0x140165701 48c7854014000000000000 mov qword ptr [rbp + 0x1440], 0
0x14016570c 48c7854814000002000000 mov qword ptr [rbp + 0x1448], 2
0x140165717 0f57c0 xorps xmm0, xmm0
0x14016571a 0f118550140000 movups xmmword ptr [rbp + 0x1450], xmm0
0x140165721 48c7856014000001000000 mov qword ptr [rbp + 0x1460], 1
0x14016572c 48c7856814000000000000 mov qword ptr [rbp + 0x1468], 0
0x140165737 0f108de0000000 movups xmm1, xmmword ptr [rbp + 0xe0]
0x14016573e 0f118d70140000 movups xmmword ptr [rbp + 0x1470], xmm1
0x140165745 488b85f0000000 mov rax, qword ptr [rbp + 0xf0]
0x14016574c 48898580140000 mov qword ptr [rbp + 0x1480], rax
0x140165753 488d8d88140000 lea rcx, [rbp + 0x1488]
0x14016575a 48c7858814000000000000 mov qword ptr [rbp + 0x1488], 0
0x140165765 48c7859014000004000000 mov qword ptr [rbp + 0x1490], 4
0x140165770 0f118598140000 movups xmmword ptr [rbp + 0x1498], xmm0
0x140165777 48c785a814000004000000 mov qword ptr [rbp + 0x14a8], 4
0x140165782 0f1185b0140000 movups xmmword ptr [rbp + 0x14b0], xmm0
0x140165789 48c785c014000004000000 mov qword ptr [rbp + 0x14c0], 4
0x140165794 0f1185c8140000 movups xmmword ptr [rbp + 0x14c8], xmm0
0x14016579b 48c785d814000004000000 mov qword ptr [rbp + 0x14d8], 4
0x1401657a6 0f1185e0140000 movups xmmword ptr [rbp + 0x14e0], xmm0
0x1401657ad 48c785f014000004000000 mov qword ptr [rbp + 0x14f0], 4
0x1401657b8 0f1185f8140000 movups xmmword ptr [rbp + 0x14f8], xmm0
0x1401657bf 48c7850815000004000000 mov qword ptr [rbp + 0x1508], 4
0x1401657ca 48c7851015000000000000 mov qword ptr [rbp + 0x1510], 0
0x1401657d5 4c8b85f8180000 mov r8, qword ptr [rbp + 0x18f8]
0x1401657dc 4c8b8d00190000 mov r9, qword ptr [rbp + 0x1900]
0x1401657e3 c6857219000001 mov byte ptr [rbp + 0x1972], 1
0x1401657ea c6857119000001 mov byte ptr [rbp + 0x1971], 1
0x1401657f1 488d9550130000 lea rdx, [rbp + 0x1350]
0x1401657f8 e833ac3c00 call 0x140530430 ; rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources>
0x1401657fd c6857219000001 mov byte ptr [rbp + 0x1972], 1
0x140165804 c6857119000001 mov byte ptr [rbp + 0x1971], 1
0x14016580b 488d8d40140000 lea rcx, [rbp + 0x1440]
0x140165812 488d9550130000 lea rdx, [rbp + 0x1350]
0x140165819 4c8b85d0180000 mov r8, qword ptr [rbp + 0x18d0]
0x140165820 4c8b8dd8180000 mov r9, qword ptr [rbp + 0x18d8]
0x140165827 e87423a800 call 0x140be7ba0 ; rhapsody::rg::resource_registry::ResourceRegistry::populate_resources
0x14016582c 84c0 test al, al
0x14016582e 0f84af000000 je 0x1401658e3
0x140165834 c6857219000001 mov byte ptr [rbp + 0x1972], 1
0x14016583b c6857119000001 mov byte ptr [rbp + 0x1971], 1
0x140165842 488d8d50130000 lea rcx, [rbp + 0x1350]
0x140165849 e8d23ba800 call 0x140be9420 ; rhapsody::rg::graph::RenderGraph::on_retire
0x14016584e 488d8de0000000 lea rcx, [rbp + 0xe0]
0x140165855 41b890000000 mov r8d, 0x90
0x14016585b 4889fa mov rdx, rdi
0x14016585e e89de37b02 call 0x142923c00 ; memcpy
0x140165863 488d9550130000 lea rdx, [rbp + 0x1350]
0x14016586a 41b890000000 mov r8d, 0x90
0x140165870 4889f9 mov rcx, rdi
0x140165873 e888e37b02 call 0x142923c00 ; memcpy
0x140165878 31c0 xor eax, eax
0x14016587a 483b85e0000000 cmp rax, qword ptr [rbp + 0xe0]
0x140165881 701a jo 0x14016589d
0x140165883 c6857219000000 mov byte ptr [rbp + 0x1972], 0
0x14016588a c6857119000001 mov byte ptr [rbp + 0x1971], 1
0x140165891 488d8de0000000 lea rcx, [rbp + 0xe0]
0x140165898 e873994800 call 0x1405ef210 ; core::ptr::drop_in_place<rhapsody::rg::graph::RenderGraph>
0x14016589d 488bb5e8180000 mov rsi, qword ptr [rbp + 0x18e8]
0x1401658a4 48b80000000000000080 movabs rax, 0x8000000000000000
0x1401658ae 488906 mov qword ptr [rsi], rax
0x1401658b1 c6857419000000 mov byte ptr [rbp + 0x1974], 0
0x1401658b8 c6857319000001 mov byte ptr [rbp + 0x1973], 1
0x1401658bf 488d8d40140000 lea rcx, [rbp + 0x1440]
0x1401658c6 e805794900 call 0x1405fd1d0 ; core::ptr::drop_in_place<rhapsody::rg::resource_registry::ResourceRegistry>
0x1401658cb c6856d19000000 mov byte ptr [rbp + 0x196d], 0
0x1401658d2 488d8df0180000 lea rcx, [rbp + 0x18f0]
0x1401658d9 e8424f4700 call 0x1405da820 ; core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources>,alloc::alloc::Global> >
0x1401658de e99e000000 jmp 0x140165981
0x1401658e3 488dbd20120000 lea rdi, [rbp + 0x1220]
0x1401658ea 488d9550130000 lea rdx, [rbp + 0x1350]
0x1401658f1 41b890000000 mov r8d, 0x90
0x1401658f7 4889f9 mov rcx, rdi
0x1401658fa e801e37b02 call 0x142923c00 ; memcpy
0x1401658ff 488b8500190000 mov rax, qword ptr [rbp + 0x1900]
0x140165906 48898530190000 mov qword ptr [rbp + 0x1930], rax
0x14016590d 0f2885f0180000 movaps xmm0, xmmword ptr [rbp + 0x18f0]
0x140165914 0f298520190000 movaps xmmword ptr [rbp + 0x1920], xmm0
0x14016591b 488d9df0160000 lea rbx, [rbp + 0x16f0]
0x140165922 488d9540140000 lea rdx, [rbp + 0x1440]
0x140165929 41b808010000 mov r8d, 0x108
0x14016592f 4889d9 mov rcx, rbx
0x140165932 e8c9e27b02 call 0x142923c00 ; memcpy
0x140165937 c6857219000000 mov byte ptr [rbp + 0x1972], 0
0x14016593e c6857119000000 mov byte ptr [rbp + 0x1971], 0
0x140165945 488d8de0000000 lea rcx, [rbp + 0xe0]
0x14016594c 4c8d8520190000 lea r8, [rbp + 0x1920]
0x140165953 4889fa mov rdx, rdi
0x140165956 4989d9 mov r9, rbx
0x140165959 e822e91500 call 0x1402c4280 ; rhapsody::rg::compiled::CompiledRenderGraph::new<country_core::render::pass_builder::pass_resources::PassResources>
0x14016595e 488d95e0000000 lea rdx, [rbp + 0xe0]
0x140165965 41b818020000 mov r8d, 0x218
0x14016596b 488bb5e8180000 mov rsi, qword ptr [rbp + 0x18e8]
0x140165972 4889f1 mov rcx, rsi
0x140165975 e886e27b02 call 0x142923c00 ; memcpy
0x14016597a 4489ae18020000 mov dword ptr [rsi + 0x218], r13d
0x140165981 4889f0 mov rax, rsi
0x140165984 0f28b580190000 movaps xmm6, xmmword ptr [rbp + 0x1980]
0x14016598b 4881c4181a0000 add rsp, 0x1a18
0x140165992 5b pop rbx
0x140165993 5f pop rdi
0x140165994 5e pop rsi
0x140165995 415c pop r12
0x140165997 415d pop r13
0x140165999 415e pop r14
0x14016599b 415f pop r15
0x14016599d 5d pop rbp
0x14016599e c3 ret 
0x14016599f 48b9fcffffffffffff7f movabs rcx, 0x7ffffffffffffffc
0x1401659a9 4801c8 add rax, rcx
0x1401659ac 4883f806 cmp rax, 6
0x1401659b0 b904000000 mov ecx, 4
0x1401659b5 480f42c8 cmovb rcx, rax
0x1401659b9 4883f904 cmp rcx, 4
0x1401659bd 0f87a2000000 ja 0x140165a65
0x1401659c3 488d05ba807f02 lea rax, [rip + 0x27f80ba]
0x1401659ca 48630c88 movsxd rcx, dword ptr [rax + rcx*4]
0x1401659ce 4801c1 add rcx, rax
0x1401659d1 ffe1 jmp rcx
0x1401659d3 488d8df8160000 lea rcx, [rbp + 0x16f8]
0x1401659da c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x1401659e1 e85a4d4800 call 0x1405ea740 ; core::ptr::drop_in_place<image::error::DecodingError>
0x1401659e6 e9baf6ffff jmp 0x1401650a5
0x1401659eb 488d50f7 lea rdx, [rax - 9]
0x1401659ef 4c8d40f8 lea r8, [rax - 8]
0x1401659f3 31c9 xor ecx, ecx
0x1401659f5 4883fa02 cmp rdx, 2
0x1401659f9 490f42c8 cmovb rcx, r8
0x1401659fd 4885c9 test rcx, rcx
0x140165a00 747b je 0x140165a7d
0x140165a02 4883f901 cmp rcx, 1
0x140165a06 0f85b5000000 jne 0x140165ac1
0x140165a0c 488d8df8160000 lea rcx, [rbp + 0x16f8]
0x140165a13 e8d8624800 call 0x1405ebcf0 ; core::ptr::drop_in_place<alloc::vec::Vec<backtrace::capture::BacktraceFrame,alloc::alloc::Global> >
0x140165a18 e9c5000000 jmp 0x140165ae2
0x140165a1d c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x140165a24 488d8df0160000 lea rcx, [rbp + 0x16f0]
0x140165a2b e890784800 call 0x1405ed2c0 ; core::ptr::drop_in_place<image::error::UnsupportedError>
0x140165a30 e970f6ffff jmp 0x1401650a5
0x140165a35 488d8df8160000 lea rcx, [rbp + 0x16f8]
0x140165a3c c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x140165a43 e8f84c4800 call 0x1405ea740 ; core::ptr::drop_in_place<image::error::DecodingError>
0x140165a48 e958f6ffff jmp 0x1401650a5
0x140165a4d 488d8df8160000 lea rcx, [rbp + 0x16f8]
0x140165a54 c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x140165a5b e830584800 call 0x1405eb290 ; core::ptr::drop_in_place<image::error::ParameterError>
0x140165a60 e940f6ffff jmp 0x1401650a5
0x140165a65 488d8df8160000 lea rcx, [rbp + 0x16f8]
0x140165a6c c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x140165a73 e8881f4800 call 0x1405e7a00 ; core::ptr::drop_in_place<std::io::error::Error>
0x140165a78 e928f6ffff jmp 0x1401650a5
0x140165a7d 4883f805 cmp rax, 5
0x140165a81 7729 ja 0x140165aac
0x140165a83 b932000000 mov ecx, 0x32
0x140165a88 480fa3c1 bt rcx, rax
0x140165a8c 731e jae 0x140165aac
0x140165a8e 488b95f8160000 mov rdx, qword ptr [rbp + 0x16f8]
0x140165a95 4885d2 test rdx, rdx
0x140165a98 7412 je 0x140165aac
0x140165a9a 488b8d00170000 mov rcx, qword ptr [rbp + 0x1700]
0x140165aa1 41b801000000 mov r8d, 1
0x140165aa7 e874e14a00 call 0x140613c20 ; __rust_dealloc
0x140165aac 488b9510170000 mov rdx, qword ptr [rbp + 0x1710]
0x140165ab3 4885d2 test rdx, rdx
0x140165ab6 742a je 0x140165ae2
0x140165ab8 488d8518170000 lea rax, [rbp + 0x1718]
0x140165abf eb13 jmp 0x140165ad4
0x140165ac1 488b95f8160000 mov rdx, qword ptr [rbp + 0x16f8]
0x140165ac8 4885d2 test rdx, rdx
0x140165acb 7415 je 0x140165ae2
0x140165acd 488d8500170000 lea rax, [rbp + 0x1700]
0x140165ad4 488b08 mov rcx, qword ptr [rax]
0x140165ad7 41b801000000 mov r8d, 1
0x140165add e83ee14a00 call 0x140613c20 ; __rust_dealloc
0x140165ae2 488b9558190000 mov rdx, qword ptr [rbp + 0x1958]
0x140165ae9 4885d2 test rdx, rdx
0x140165aec 0f84b3f5ffff je 0x1401650a5
0x140165af2 41b801000000 mov r8d, 1
0x140165af8 488b8d18190000 mov rcx, qword ptr [rbp + 0x1918]
0x140165aff e81ce14a00 call 0x140613c20 ; __rust_dealloc
0x140165b04 e99cf5ffff jmp 0x1401650a5
0x140165b09 c6857619000001 mov byte ptr [rbp + 0x1976], 1
0x140165b10 4889f1 mov rcx, rsi
0x140165b13 41b800ca9a3b mov r8d, 0x3b9aca00
0x140165b19 e8f2767602 call 0x1428cd210 ; parking_lot::raw_mutex::RawMutex::lock_slow
0x140165b1e e9d5efffff jmp 0x140164af8
0x140165b23 4c8d05868d8702 lea r8, [rip + 0x2878d86]
0x140165b2a ba90100000 mov edx, 0x1090
0x140165b2f e8ac3d7702 call 0x1428d98e0 ; core::slice::impl$0::copy_from_slice::len_mismatch_fail
0x140165b34 e919010000 jmp 0x140165c52
0x140165b39 c6857019000001 mov byte ptr [rbp + 0x1970], 1
0x140165b40 4889f1 mov rcx, rsi
0x140165b43 31d2 xor edx, edx
0x140165b45 e846807602 call 0x1428cdb90 ; parking_lot::raw_mutex::RawMutex::unlock_slow
0x140165b4a e991f0ffff jmp 0x140164be0
0x140165b4f c6857519000001 mov byte ptr [rbp + 0x1975], 1
0x140165b56 488d0dcb7d7f02 lea rcx, [rip + 0x27f7dcb]
0x140165b5d e8ae377702 call 0x1428d9310 ; core::option::unwrap_failed
0x140165b62 e9eb000000 jmp 0x140165c52
0x140165b67 c6857619000001 mov byte ptr [rbp + 0x1976], 1
0x140165b6e 488d0dcb7d7f02 lea rcx, [rip + 0x27f7dcb]
0x140165b75 e896377702 call 0x1428d9310 ; core::option::unwrap_failed
0x140165b7a e9d3000000 jmp 0x140165c52
0x140165b7f c6857619000001 mov byte ptr [rbp + 0x1976], 1
0x140165b86 488d0dcb7d7f02 lea rcx, [rip + 0x27f7dcb]
0x140165b8d e87e377702 call 0x1428d9310 ; core::option::unwrap_failed
0x140165b92 e9bb000000 jmp 0x140165c52
0x140165b97 c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x140165b9e 488d0ddb7d7f02 lea rcx, [rip + 0x27f7ddb]
0x140165ba5 e866377702 call 0x1428d9310 ; core::option::unwrap_failed
0x140165baa e9a3000000 jmp 0x140165c52
0x140165baf c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x140165bb6 488d0ddb7d7f02 lea rcx, [rip + 0x27f7ddb]
0x140165bbd e84e377702 call 0x1428d9310 ; core::option::unwrap_failed
0x140165bc2 e98b000000 jmp 0x140165c52
0x140165bc7 488d0de27d7f02 lea rcx, [rip + 0x27f7de2]
0x140165bce e83d377702 call 0x1428d9310 ; core::option::unwrap_failed
0x140165bd3 eb7d jmp 0x140165c52
0x140165bd5 488d0dec7d7f02 lea rcx, [rip + 0x27f7dec]
0x140165bdc e82f377702 call 0x1428d9310 ; core::option::unwrap_failed
0x140165be1 eb6f jmp 0x140165c52
0x140165be3 488d0dee797f02 lea rcx, [rip + 0x27f79ee]
0x140165bea e821377702 call 0x1428d9310 ; core::option::unwrap_failed
0x140165bef 488d0dfa797f02 lea rcx, [rip + 0x27f79fa]
0x140165bf6 e815377702 call 0x1428d9310 ; core::option::unwrap_failed
0x140165bfb eb55 jmp 0x140165c52
0x140165bfd c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x140165c04 488d0dd57d7f02 lea rcx, [rip + 0x27f7dd5]
0x140165c0b 4c8d05f67d7f02 lea r8, [rip + 0x27f7df6]
0x140165c12 ba28000000 mov edx, 0x28
0x140165c17 e814387702 call 0x1428d9430 ; core::panicking::panic
0x140165c1c eb34 jmp 0x140165c52
0x140165c1e 488d0debcd7c02 lea rcx, [rip + 0x27ccdeb]
0x140165c25 e8a6247702 call 0x1428d80d0 ; std::thread::local::panic_access_error
0x140165c2a eb26 jmp 0x140165c52
0x140165c2c 488d0dfd7d7f02 lea rcx, [rip + 0x27f7dfd]
0x140165c33 e8d8367702 call 0x1428d9310 ; core::option::unwrap_failed
0x140165c38 eb18 jmp 0x140165c52
0x140165c3a 488d0d9f7d7f02 lea rcx, [rip + 0x27f7d9f]
0x140165c41 4c8d05007e7f02 lea r8, [rip + 0x27f7e00]
0x140165c48 ba28000000 mov edx, 0x28
0x140165c4d e8de377702 call 0x1428d9430 ; core::panicking::panic
0x140165c52 0f0b ud2 
0x140165c54 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140165c60 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140165c65 55 push rbp
0x140165c66 4157 push r15
0x140165c68 4156 push r14
0x140165c6a 4155 push r13
0x140165c6c 4154 push r12
0x140165c6e 56 push rsi
0x140165c6f 57 push rdi
0x140165c70 53 push rbx
0x140165c71 4883ec68 sub rsp, 0x68
0x140165c75 488daa80000000 lea rbp, [rdx + 0x80]
0x140165c7c 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140165c81 488b8510160000 mov rax, qword ptr [rbp + 0x1610]
0x140165c88 488b8d60190000 mov rcx, qword ptr [rbp + 0x1960]
0x140165c8f 48894130 mov qword ptr [rcx + 0x30], rax
0x140165c93 0f2885e0150000 movaps xmm0, xmmword ptr [rbp + 0x15e0]
0x140165c9a 0f288df0150000 movaps xmm1, xmmword ptr [rbp + 0x15f0]
0x140165ca1 0f289500160000 movaps xmm2, xmmword ptr [rbp + 0x1600]
0x140165ca8 0f115120 movups xmmword ptr [rcx + 0x20], xmm2
0x140165cac 0f114910 movups xmmword ptr [rcx + 0x10], xmm1
0x140165cb0 0f1101 movups xmmword ptr [rcx], xmm0
0x140165cb3 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140165cb8 4883c468 add rsp, 0x68
0x140165cbc 5b pop rbx
0x140165cbd 5f pop rdi
0x140165cbe 5e pop rsi
0x140165cbf 415c pop r12
0x140165cc1 415d pop r13
0x140165cc3 415e pop r14
0x140165cc5 415f pop r15
0x140165cc7 5d pop rbp
0x140165cc8 c3 ret 
0x140165cc9 0f1f8000000000 nop dword ptr [rax]
0x140165cd0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140165cd5 55 push rbp
0x140165cd6 4157 push r15
0x140165cd8 4156 push r14
0x140165cda 4155 push r13
0x140165cdc 4154 push r12
0x140165cde 56 push rsi
0x140165cdf 57 push rdi
0x140165ce0 53 push rbx
0x140165ce1 4883ec68 sub rsp, 0x68
0x140165ce5 488daa80000000 lea rbp, [rdx + 0x80]
0x140165cec 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140165cf1 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x140165cf8 e863bfffff call 0x140161c60 ; core::ptr::drop_in_place<enum2$<image::dynimage::DynamicImage> >
0x140165cfd c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x140165d04 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140165d09 4883c468 add rsp, 0x68
0x140165d0d 5b pop rbx
0x140165d0e 5f pop rdi
0x140165d0f 5e pop rsi
0x140165d10 415c pop r12
0x140165d12 415d pop r13
0x140165d14 415e pop r14
0x140165d16 415f pop r15
0x140165d18 5d pop rbp
0x140165d19 c3 ret 
0x140165d1a 660f1f440000 nop word ptr [rax + rax]
0x140165d20 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140165d25 55 push rbp
0x140165d26 4157 push r15
0x140165d28 4156 push r14
0x140165d2a 4155 push r13
0x140165d2c 4154 push r12
0x140165d2e 56 push rsi
0x140165d2f 57 push rdi
0x140165d30 53 push rbx
0x140165d31 4883ec68 sub rsp, 0x68
0x140165d35 488daa80000000 lea rbp, [rdx + 0x80]
0x140165d3c 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140165d41 4883bd5819000000 cmp qword ptr [rbp + 0x1958], 0
0x140165d49 7419 je 0x140165d64
0x140165d4b 41b801000000 mov r8d, 1
0x140165d51 488b8d18190000 mov rcx, qword ptr [rbp + 0x1918]
0x140165d58 488b9558190000 mov rdx, qword ptr [rbp + 0x1958]
0x140165d5f e8bcde4a00 call 0x140613c20 ; __rust_dealloc
0x140165d64 c6856f19000000 mov byte ptr [rbp + 0x196f], 0
0x140165d6b 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140165d70 4883c468 add rsp, 0x68
0x140165d74 5b pop rbx
0x140165d75 5f pop rdi
0x140165d76 5e pop rsi
0x140165d77 415c pop r12
0x140165d79 415d pop r13
0x140165d7b 415e pop r14
0x140165d7d 415f pop r15
0x140165d7f 5d pop rbp
0x140165d80 c3 ret 
0x140165d81 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140165d90 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140165d95 55 push rbp
0x140165d96 4157 push r15
0x140165d98 4156 push r14
0x140165d9a 4155 push r13
0x140165d9c 4154 push r12
0x140165d9e 56 push rsi
0x140165d9f 57 push rdi
0x140165da0 53 push rbx
0x140165da1 4883ec68 sub rsp, 0x68
0x140165da5 488daa80000000 lea rbp, [rdx + 0x80]
0x140165dac 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140165db1 488d8d20190000 lea rcx, [rbp + 0x1920]
0x140165db8 e823884800 call 0x1405ee5e0 ; core::ptr::drop_in_place<rhapsody::raw::texture::Texture>
0x140165dbd c6856f19000000 mov byte ptr [rbp + 0x196f], 0
0x140165dc4 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140165dc9 4883c468 add rsp, 0x68
0x140165dcd 5b pop rbx
0x140165dce 5f pop rdi
0x140165dcf 5e pop rsi
0x140165dd0 415c pop r12
0x140165dd2 415d pop r13
0x140165dd4 415e pop r14
0x140165dd6 415f pop r15
0x140165dd8 5d pop rbp
0x140165dd9 c3 ret 
0x140165dda 660f1f440000 nop word ptr [rax + rax]
0x140165de0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140165de5 55 push rbp
0x140165de6 4157 push r15
0x140165de8 4156 push r14
0x140165dea 4155 push r13
0x140165dec 4154 push r12
0x140165dee 56 push rsi
0x140165def 57 push rdi
0x140165df0 53 push rbx
0x140165df1 4883ec68 sub rsp, 0x68
0x140165df5 488daa80000000 lea rbp, [rdx + 0x80]
0x140165dfc 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140165e01 80bd6f19000000 cmp byte ptr [rbp + 0x196f], 0
0x140165e08 743b je 0x140165e45
0x140165e0a 488b95a0180000 mov rdx, qword ptr [rbp + 0x18a0]
0x140165e11 4885d2 test rdx, rdx
0x140165e14 7412 je 0x140165e28
0x140165e16 488b8da8180000 mov rcx, qword ptr [rbp + 0x18a8]
0x140165e1d 41b801000000 mov r8d, 1
0x140165e23 e8f8dd4a00 call 0x140613c20 ; __rust_dealloc
0x140165e28 c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x140165e2f 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140165e34 4883c468 add rsp, 0x68
0x140165e38 5b pop rbx
0x140165e39 5f pop rdi
0x140165e3a 5e pop rsi
0x140165e3b 415c pop r12
0x140165e3d 415d pop r13
0x140165e3f 415e pop r14
0x140165e41 415f pop r15
0x140165e43 5d pop rbp
0x140165e44 c3 ret 
0x140165e45 c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x140165e4c 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140165e51 4883c468 add rsp, 0x68
0x140165e55 5b pop rbx
0x140165e56 5f pop rdi
0x140165e57 5e pop rsi
0x140165e58 415c pop r12
0x140165e5a 415d pop r13
0x140165e5c 415e pop r14
0x140165e5e 415f pop r15
0x140165e60 5d pop rbp
0x140165e61 c3 ret 
0x140165e62 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140165e70 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140165e75 55 push rbp
0x140165e76 4157 push r15
0x140165e78 4156 push r14
0x140165e7a 4155 push r13
0x140165e7c 4154 push r12
0x140165e7e 56 push rsi
0x140165e7f 57 push rdi
0x140165e80 53 push rbx
0x140165e81 4883ec68 sub rsp, 0x68
0x140165e85 488daa80000000 lea rbp, [rdx + 0x80]
0x140165e8c 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140165e91 488d0d18524c03 lea rcx, [rip + 0x34c5218]
0x140165e98 e813c6a700 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
0x140165e9d 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140165ea2 4883c468 add rsp, 0x68
0x140165ea6 5b pop rbx
0x140165ea7 5f pop rdi
0x140165ea8 5e pop rsi
0x140165ea9 415c pop r12
0x140165eab 415d pop r13
0x140165ead 415e pop r14
0x140165eaf 415f pop r15
0x140165eb1 5d pop rbp
0x140165eb2 c3 ret 
0x140165eb3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140165ec0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140165ec5 55 push rbp
0x140165ec6 4157 push r15
0x140165ec8 4156 push r14
0x140165eca 4155 push r13
0x140165ecc 4154 push r12
0x140165ece 56 push rsi
0x140165ecf 57 push rdi
0x140165ed0 53 push rbx
0x140165ed1 4883ec68 sub rsp, 0x68
0x140165ed5 488daa80000000 lea rbp, [rdx + 0x80]
0x140165edc 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140165ee1 488b851e170000 mov rax, qword ptr [rbp + 0x171e]
0x140165ee8 488905ef514c03 mov qword ptr [rip + 0x34c51ef], rax
0x140165eef 0f2885f0160000 movaps xmm0, xmmword ptr [rbp + 0x16f0]
0x140165ef6 0f288d00170000 movaps xmm1, xmmword ptr [rbp + 0x1700]
0x140165efd 0f289510170000 movaps xmm2, xmmword ptr [rbp + 0x1710]
0x140165f04 0f1115c5514c03 movups xmmword ptr [rip + 0x34c51c5], xmm2
0x140165f0b 0f110dae514c03 movups xmmword ptr [rip + 0x34c51ae], xmm1
0x140165f12 0f110597514c03 movups xmmword ptr [rip + 0x34c5197], xmm0
0x140165f19 0fb68560190000 movzx eax, byte ptr [rbp + 0x1960]
0x140165f20 8805c0514c03 mov byte ptr [rip + 0x34c51c0], al
0x140165f26 0fb68558190000 movzx eax, byte ptr [rbp + 0x1958]
0x140165f2d 8805b4514c03 mov byte ptr [rip + 0x34c51b4], al
0x140165f33 c6857719000001 mov byte ptr [rbp + 0x1977], 1
0x140165f3a 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140165f3f 4883c468 add rsp, 0x68
0x140165f43 5b pop rbx
0x140165f44 5f pop rdi
0x140165f45 5e pop rsi
0x140165f46 415c pop r12
0x140165f48 415d pop r13
0x140165f4a 415e pop r14
0x140165f4c 415f pop r15
0x140165f4e 5d pop rbp
0x140165f4f c3 ret 
0x140165f50 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140165f55 55 push rbp
0x140165f56 4157 push r15
0x140165f58 4156 push r14
0x140165f5a 4155 push r13
0x140165f5c 4154 push r12
0x140165f5e 56 push rsi
0x140165f5f 57 push rdi
0x140165f60 53 push rbx
0x140165f61 4883ec68 sub rsp, 0x68
0x140165f65 488daa80000000 lea rbp, [rdx + 0x80]
0x140165f6c 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140165f71 488d8d10010000 lea rcx, [rbp + 0x110]
0x140165f78 488d9530010000 lea rdx, [rbp + 0x130]
0x140165f7f 41b810000000 mov r8d, 0x10
0x140165f85 41b910000000 mov r9d, 0x10
0x140165f8b e87064eeff call 0x14004c400 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<core::ptr::non_null::NonNull<hashlink::linked_hash_map::Node<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> > >,tuple$<> >,allocator_api2::stable::alloc::global::Global>
0x140165f90 c6856e19000001 mov byte ptr [rbp + 0x196e], 1
0x140165f97 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140165f9c 4883c468 add rsp, 0x68
0x140165fa0 5b pop rbx
0x140165fa1 5f pop rdi
0x140165fa2 5e pop rsi
0x140165fa3 415c pop r12
0x140165fa5 415d pop r13
0x140165fa7 415e pop r14
0x140165fa9 415f pop r15
0x140165fab 5d pop rbp
0x140165fac c3 ret 
0x140165fad 0f1f00 nop dword ptr [rax]
0x140165fb0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140165fb5 55 push rbp
0x140165fb6 4157 push r15
0x140165fb8 4156 push r14
0x140165fba 4155 push r13
0x140165fbc 4154 push r12
0x140165fbe 56 push rsi
0x140165fbf 57 push rdi
0x140165fc0 53 push rbx
0x140165fc1 4883ec68 sub rsp, 0x68
0x140165fc5 488daa80000000 lea rbp, [rdx + 0x80]
0x140165fcc 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140165fd1 c6857419000001 mov byte ptr [rbp + 0x1974], 1
0x140165fd8 c6857319000001 mov byte ptr [rbp + 0x1973], 1
0x140165fdf 488d8de0000000 lea rcx, [rbp + 0xe0]
0x140165fe6 e8c5554a00 call 0x14060b5b0 ; core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::rg::resource_registry::RegistryResource,alloc::alloc::Global> >
0x140165feb c6857419000001 mov byte ptr [rbp + 0x1974], 1
0x140165ff2 c6857319000001 mov byte ptr [rbp + 0x1973], 1
0x140165ff9 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140165ffe 4883c468 add rsp, 0x68
0x140166002 5b pop rbx
0x140166003 5f pop rdi
0x140166004 5e pop rsi
0x140166005 415c pop r12
0x140166007 415d pop r13
0x140166009 415e pop r14
0x14016600b 415f pop r15
0x14016600d 5d pop rbp
0x14016600e c3 ret 
0x14016600f 90 nop 
0x140166010 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140166015 55 push rbp
0x140166016 4157 push r15
0x140166018 4156 push r14
0x14016601a 4155 push r13
0x14016601c 4154 push r12
0x14016601e 56 push rsi
0x14016601f 57 push rdi
0x140166020 53 push rbx
0x140166021 4883ec68 sub rsp, 0x68
0x140166025 488daa80000000 lea rbp, [rdx + 0x80]
0x14016602c 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140166031 0fb69d71190000 movzx ebx, byte ptr [rbp + 0x1971]
0x140166038 0fb6b572190000 movzx esi, byte ptr [rbp + 0x1972]
0x14016603f f6c301 test bl, 1
0x140166042 7433 je 0x140166077
0x140166044 488d8d40140000 lea rcx, [rbp + 0x1440]
0x14016604b e880714900 call 0x1405fd1d0 ; core::ptr::drop_in_place<rhapsody::rg::resource_registry::ResourceRegistry>
0x140166050 4080e601 and sil, 1
0x140166054 4088b574190000 mov byte ptr [rbp + 0x1974], sil
0x14016605b 889d73190000 mov byte ptr [rbp + 0x1973], bl
0x140166061 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140166066 4883c468 add rsp, 0x68
0x14016606a 5b pop rbx
0x14016606b 5f pop rdi
0x14016606c 5e pop rsi
0x14016606d 415c pop r12
0x14016606f 415d pop r13
0x140166071 415e pop r14
0x140166073 415f pop r15
0x140166075 5d pop rbp
0x140166076 c3 ret 
0x140166077 4080e601 and sil, 1
0x14016607b 4088b574190000 mov byte ptr [rbp + 0x1974], sil
0x140166082 889d73190000 mov byte ptr [rbp + 0x1973], bl
0x140166088 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016608d 4883c468 add rsp, 0x68
0x140166091 5b pop rbx
0x140166092 5f pop rdi
0x140166093 5e pop rsi
0x140166094 415c pop r12
0x140166096 415d pop r13
0x140166098 415e pop r14
0x14016609a 415f pop r15
0x14016609c 5d pop rbp
0x14016609d c3 ret 
0x14016609e 6690 nop 
0x1401660a0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1401660a5 55 push rbp
0x1401660a6 4157 push r15
0x1401660a8 4156 push r14
0x1401660aa 4155 push r13
0x1401660ac 4154 push r12
0x1401660ae 56 push rsi
0x1401660af 57 push rdi
0x1401660b0 53 push rbx
0x1401660b1 4883ec68 sub rsp, 0x68
0x1401660b5 488daa80000000 lea rbp, [rdx + 0x80]
0x1401660bc 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x1401660c1 0fb69d74190000 movzx ebx, byte ptr [rbp + 0x1974]
0x1401660c8 f6857319000001 test byte ptr [rbp + 0x1973], 1
0x1401660cf 742b je 0x1401660fc
0x1401660d1 488d8df0180000 lea rcx, [rbp + 0x18f0]
0x1401660d8 e843474700 call 0x1405da820 ; core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources>,alloc::alloc::Global> >
0x1401660dd 80e301 and bl, 1
0x1401660e0 889d6d190000 mov byte ptr [rbp + 0x196d], bl
0x1401660e6 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401660eb 4883c468 add rsp, 0x68
0x1401660ef 5b pop rbx
0x1401660f0 5f pop rdi
0x1401660f1 5e pop rsi
0x1401660f2 415c pop r12
0x1401660f4 415d pop r13
0x1401660f6 415e pop r14
0x1401660f8 415f pop r15
0x1401660fa 5d pop rbp
0x1401660fb c3 ret 
0x1401660fc 80e301 and bl, 1
0x1401660ff 889d6d190000 mov byte ptr [rbp + 0x196d], bl
0x140166105 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016610a 4883c468 add rsp, 0x68
0x14016610e 5b pop rbx
0x14016610f 5f pop rdi
0x140166110 5e pop rsi
0x140166111 415c pop r12
0x140166113 415d pop r13
0x140166115 415e pop r14
0x140166117 415f pop r15
0x140166119 5d pop rbp
0x14016611a c3 ret 
0x14016611b 0f1f440000 nop dword ptr [rax + rax]
0x140166120 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140166125 55 push rbp
0x140166126 4157 push r15
0x140166128 4156 push r14
0x14016612a 4155 push r13
0x14016612c 4154 push r12
0x14016612e 56 push rsi
0x14016612f 57 push rdi
0x140166130 53 push rbx
0x140166131 4883ec68 sub rsp, 0x68
0x140166135 488daa80000000 lea rbp, [rdx + 0x80]
0x14016613c 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140166141 80bd6d19000000 cmp byte ptr [rbp + 0x196d], 0
0x140166148 7429 je 0x140166173
0x14016614a 488d8d50130000 lea rcx, [rbp + 0x1350]
0x140166151 e8ba904800 call 0x1405ef210 ; core::ptr::drop_in_place<rhapsody::rg::graph::RenderGraph>
0x140166156 c6856e19000000 mov byte ptr [rbp + 0x196e], 0
0x14016615d 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140166162 4883c468 add rsp, 0x68
0x140166166 5b pop rbx
0x140166167 5f pop rdi
0x140166168 5e pop rsi
0x140166169 415c pop r12
0x14016616b 415d pop r13
0x14016616d 415e pop r14
0x14016616f 415f pop r15
0x140166171 5d pop rbp
0x140166172 c3 ret 
0x140166173 c6856e19000000 mov byte ptr [rbp + 0x196e], 0
0x14016617a 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016617f 4883c468 add rsp, 0x68
0x140166183 5b pop rbx
0x140166184 5f pop rdi
0x140166185 5e pop rsi
0x140166186 415c pop r12
0x140166188 415d pop r13
0x14016618a 415e pop r14
0x14016618c 415f pop r15
0x14016618e 5d pop rbp
0x14016618f c3 ret 
0x140166190 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140166195 55 push rbp
0x140166196 4157 push r15
0x140166198 4156 push r14
0x14016619a 4155 push r13
0x14016619c 4154 push r12
0x14016619e 56 push rsi
0x14016619f 57 push rdi
0x1401661a0 53 push rbx
0x1401661a1 4883ec68 sub rsp, 0x68
0x1401661a5 488daa80000000 lea rbp, [rdx + 0x80]
0x1401661ac 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x1401661b1 80bd6e19000000 cmp byte ptr [rbp + 0x196e], 0
0x1401661b8 7422 je 0x1401661dc
0x1401661ba 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x1401661c1 e83a4d4700 call 0x1405daf00 ; core::ptr::drop_in_place<rhapsody::rg::recorder::RenderGraphRecorder<country_core::render::pass_builder::pass_resources::PassResources> >
0x1401661c6 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401661cb 4883c468 add rsp, 0x68
0x1401661cf 5b pop rbx
0x1401661d0 5f pop rdi
0x1401661d1 5e pop rsi
0x1401661d2 415c pop r12
0x1401661d4 415d pop r13
0x1401661d6 415e pop r14
0x1401661d8 415f pop r15
0x1401661da 5d pop rbp
0x1401661db c3 ret 
0x1401661dc 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401661e1 4883c468 add rsp, 0x68
0x1401661e5 5b pop rbx
0x1401661e6 5f pop rdi
0x1401661e7 5e pop rsi
0x1401661e8 415c pop r12
0x1401661ea 415d pop r13
0x1401661ec 415e pop r14
0x1401661ee 415f pop r15
0x1401661f0 5d pop rbp
0x1401661f1 c3 ret 
0x1401661f2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140166200 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140166205 55 push rbp
0x140166206 4157 push r15
0x140166208 4156 push r14
0x14016620a 4155 push r13
0x14016620c 4154 push r12
0x14016620e 56 push rsi
0x14016620f 57 push rdi
0x140166210 53 push rbx
0x140166211 4883ec68 sub rsp, 0x68
0x140166215 488daa80000000 lea rbp, [rdx + 0x80]
0x14016621c 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140166221 488d8de0150000 lea rcx, [rbp + 0x15e0]
0x140166228 e8936b4700 call 0x1405dcdc0 ; core::ptr::drop_in_place<rhapsody::rg::temporal::TemporalRenderGraphRecorder<country_core::render::pass_builder::pass_resources::PassResources> >
0x14016622d c6857719000000 mov byte ptr [rbp + 0x1977], 0
0x140166234 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140166239 4883c468 add rsp, 0x68
0x14016623d 5b pop rbx
0x14016623e 5f pop rdi
0x14016623f 5e pop rsi
0x140166240 415c pop r12
0x140166242 415d pop r13
0x140166244 415e pop r14
0x140166246 415f pop r15
0x140166248 5d pop rbp
0x140166249 c3 ret 
0x14016624a 660f1f440000 nop word ptr [rax + rax]
0x140166250 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140166255 55 push rbp
0x140166256 4157 push r15
0x140166258 4156 push r14
0x14016625a 4155 push r13
0x14016625c 4154 push r12
0x14016625e 56 push rsi
0x14016625f 57 push rdi
0x140166260 53 push rbx
0x140166261 4883ec68 sub rsp, 0x68
0x140166265 488daa80000000 lea rbp, [rdx + 0x80]
0x14016626c 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140166271 80bd7719000000 cmp byte ptr [rbp + 0x1977], 0
0x140166278 7429 je 0x1401662a3
0x14016627a 488d8d40140000 lea rcx, [rbp + 0x1440]
0x140166281 e85a694900 call 0x1405fcbe0 ; core::ptr::drop_in_place<country_core::systems::render_loop::RenderContext>
0x140166286 c6857019000000 mov byte ptr [rbp + 0x1970], 0
0x14016628d 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140166292 4883c468 add rsp, 0x68
0x140166296 5b pop rbx
0x140166297 5f pop rdi
0x140166298 5e pop rsi
0x140166299 415c pop r12
0x14016629b 415d pop r13
0x14016629d 415e pop r14
0x14016629f 415f pop r15
0x1401662a1 5d pop rbp
0x1401662a2 c3 ret 
0x1401662a3 c6857019000000 mov byte ptr [rbp + 0x1970], 0
0x1401662aa 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401662af 4883c468 add rsp, 0x68
0x1401662b3 5b pop rbx
0x1401662b4 5f pop rdi
0x1401662b5 5e pop rsi
0x1401662b6 415c pop r12
0x1401662b8 415d pop r13
0x1401662ba 415e pop r14
0x1401662bc 415f pop r15
0x1401662be 5d pop rbp
0x1401662bf c3 ret 
0x1401662c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1401662c5 55 push rbp
0x1401662c6 4157 push r15
0x1401662c8 4156 push r14
0x1401662ca 4155 push r13
0x1401662cc 4154 push r12
0x1401662ce 56 push rsi
0x1401662cf 57 push rdi
0x1401662d0 53 push rbx
0x1401662d1 4883ec68 sub rsp, 0x68
0x1401662d5 488daa80000000 lea rbp, [rdx + 0x80]
0x1401662dc 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x1401662e1 0fb69d70190000 movzx ebx, byte ptr [rbp + 0x1970]
0x1401662e8 f6c301 test bl, 1
0x1401662eb 7428 je 0x140166315
0x1401662ed 488d8d50130000 lea rcx, [rbp + 0x1350]
0x1401662f4 e857b8ffff call 0x140161b50 ; core::ptr::drop_in_place<rhapsody::raw::managed::handle::GpuOwned<array$<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,2>,rhapsody::raw::managed::handle::RawManagedHandle> >
0x1401662f9 889d76190000 mov byte ptr [rbp + 0x1976], bl
0x1401662ff 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140166304 4883c468 add rsp, 0x68
0x140166308 5b pop rbx
0x140166309 5f pop rdi
0x14016630a 5e pop rsi
0x14016630b 415c pop r12
0x14016630d 415d pop r13
0x14016630f 415e pop r14
0x140166311 415f pop r15
0x140166313 5d pop rbp
0x140166314 c3 ret 
0x140166315 889d76190000 mov byte ptr [rbp + 0x1976], bl
0x14016631b 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140166320 4883c468 add rsp, 0x68
0x140166324 5b pop rbx
0x140166325 5f pop rdi
0x140166326 5e pop rsi
0x140166327 415c pop r12
0x140166329 415d pop r13
0x14016632b 415e pop r14
0x14016632d 415f pop r15
0x14016632f 5d pop rbp
0x140166330 c3 ret 
0x140166331 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140166340 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140166345 55 push rbp
0x140166346 4157 push r15
0x140166348 4156 push r14
0x14016634a 4155 push r13
0x14016634c 4154 push r12
0x14016634e 56 push rsi
0x14016634f 57 push rdi
0x140166350 53 push rbx
0x140166351 4883ec68 sub rsp, 0x68
0x140166355 488daa80000000 lea rbp, [rdx + 0x80]
0x14016635c 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140166361 0f108510170000 movups xmm0, xmmword ptr [rbp + 0x1710]
0x140166368 0f108d20170000 movups xmm1, xmmword ptr [rbp + 0x1720]
0x14016636f 0f298d50140000 movaps xmmword ptr [rbp + 0x1450], xmm1
0x140166376 0f298540140000 movaps xmmword ptr [rbp + 0x1440], xmm0
0x14016637d 488d8d40140000 lea rcx, [rbp + 0x1440]
0x140166384 e827c1a700 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
0x140166389 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016638e 4883c468 add rsp, 0x68
0x140166392 5b pop rbx
0x140166393 5f pop rdi
0x140166394 5e pop rsi
0x140166395 415c pop r12
0x140166397 415d pop r13
0x140166399 415e pop r14
0x14016639b 415f pop r15
0x14016639d 5d pop rbp
0x14016639e c3 ret 
0x14016639f 90 nop 
0x1401663a0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1401663a5 55 push rbp
0x1401663a6 4157 push r15
0x1401663a8 4156 push r14
0x1401663aa 4155 push r13
0x1401663ac 4154 push r12
0x1401663ae 56 push rsi
0x1401663af 57 push rdi
0x1401663b0 53 push rbx
0x1401663b1 4883ec68 sub rsp, 0x68
0x1401663b5 488daa80000000 lea rbp, [rdx + 0x80]
0x1401663bc 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x1401663c1 488b8d60190000 mov rcx, qword ptr [rbp + 0x1960]
0x1401663c8 e863b8ffff call 0x140161c30 ; core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> > >
0x1401663cd c6857619000001 mov byte ptr [rbp + 0x1976], 1
0x1401663d4 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401663d9 4883c468 add rsp, 0x68
0x1401663dd 5b pop rbx
0x1401663de 5f pop rdi
0x1401663df 5e pop rsi
0x1401663e0 415c pop r12
0x1401663e2 415d pop r13
0x1401663e4 415e pop r14
0x1401663e6 415f pop r15
0x1401663e8 5d pop rbp
0x1401663e9 c3 ret 
0x1401663ea 660f1f440000 nop word ptr [rax + rax]
0x1401663f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1401663f5 55 push rbp
0x1401663f6 4157 push r15
0x1401663f8 4156 push r14
0x1401663fa 4155 push r13
0x1401663fc 4154 push r12
0x1401663fe 56 push rsi
0x1401663ff 57 push rdi
0x140166400 53 push rbx
0x140166401 4883ec68 sub rsp, 0x68
0x140166405 488daa80000000 lea rbp, [rdx + 0x80]
0x14016640c 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140166411 0fb69d76190000 movzx ebx, byte ptr [rbp + 0x1976]
0x140166418 f6c301 test bl, 1
0x14016641b 7428 je 0x140166445
0x14016641d 488d8d88180000 lea rcx, [rbp + 0x1888]
0x140166424 e887c0a700 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
0x140166429 889d75190000 mov byte ptr [rbp + 0x1975], bl
0x14016642f 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140166434 4883c468 add rsp, 0x68
0x140166438 5b pop rbx
0x140166439 5f pop rdi
0x14016643a 5e pop rsi
0x14016643b 415c pop r12
0x14016643d 415d pop r13
0x14016643f 415e pop r14
0x140166441 415f pop r15
0x140166443 5d pop rbp
0x140166444 c3 ret 
0x140166445 889d75190000 mov byte ptr [rbp + 0x1975], bl
0x14016644b 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x140166450 4883c468 add rsp, 0x68
0x140166454 5b pop rbx
0x140166455 5f pop rdi
0x140166456 5e pop rsi
0x140166457 415c pop r12
0x140166459 415d pop r13
0x14016645b 415e pop r14
0x14016645d 415f pop r15
0x14016645f 5d pop rbp
0x140166460 c3 ret 
0x140166461 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140166470 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140166475 55 push rbp
0x140166476 4157 push r15
0x140166478 4156 push r14
0x14016647a 4155 push r13
0x14016647c 4154 push r12
0x14016647e 56 push rsi
0x14016647f 57 push rdi
0x140166480 53 push rbx
0x140166481 4883ec68 sub rsp, 0x68
0x140166485 488daa80000000 lea rbp, [rdx + 0x80]
0x14016648c 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x140166491 f6857519000001 test byte ptr [rbp + 0x1975], 1
0x140166498 740c je 0x1401664a6
0x14016649a 488d8d20120000 lea rcx, [rbp + 0x1220]
0x1401664a1 e81a694700 call 0x1405dcdc0 ; core::ptr::drop_in_place<rhapsody::rg::temporal::TemporalRenderGraphRecorder<country_core::render::pass_builder::pass_resources::PassResources> >
0x1401664a6 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401664ab 4883c468 add rsp, 0x68
0x1401664af 5b pop rbx
0x1401664b0 5f pop rdi
0x1401664b1 5e pop rsi
0x1401664b2 415c pop r12
0x1401664b4 415d pop r13
0x1401664b6 415e pop r14
0x1401664b8 415f pop r15
0x1401664ba 5d pop rbp
0x1401664bb c3 ret 
0x1401664bc 0f1f4000 nop dword ptr [rax]
0x1401664c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1401664c5 55 push rbp
0x1401664c6 4157 push r15
0x1401664c8 4156 push r14
0x1401664ca 4155 push r13
0x1401664cc 4154 push r12
0x1401664ce 56 push rsi
0x1401664cf 57 push rdi
0x1401664d0 53 push rbx
0x1401664d1 4883ec68 sub rsp, 0x68
0x1401664d5 488daa80000000 lea rbp, [rdx + 0x80]
0x1401664dc 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x1401664e1 4883bde018000000 cmp qword ptr [rbp + 0x18e0], 0
0x1401664e9 7416 je 0x140166501
0x1401664eb 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401664f0 4883c468 add rsp, 0x68
0x1401664f4 5b pop rbx
0x1401664f5 5f pop rdi
0x1401664f6 5e pop rsi
0x1401664f7 415c pop r12
0x1401664f9 415d pop r13
0x1401664fb 415e pop r14
0x1401664fd 415f pop r15
0x1401664ff 5d pop rbp
0x140166500 c3 ret 
0x140166501 488d8d70110000 lea rcx, [rbp + 0x1170]
0x140166508 e8038d4800 call 0x1405ef210 ; core::ptr::drop_in_place<rhapsody::rg::graph::RenderGraph>
0x14016650d ebdc jmp 0x1401664eb