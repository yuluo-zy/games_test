0x140166510 55 push rbp
0x140166511 4157 push r15
0x140166513 4156 push r14
0x140166515 4155 push r13
0x140166517 4154 push r12
0x140166519 56 push rsi
0x14016651a 57 push rdi
0x14016651b 53 push rbx
0x14016651c b898190000 mov eax, 0x1998
0x140166521 e81a457302 call 0x14289aa40 ; __chkstk
0x140166526 4829c4 sub rsp, rax
0x140166529 488dac2480000000 lea rbp, [rsp + 0x80]
0x140166531 48c78510190000feffffff mov qword ptr [rbp + 0x1910], 0xfffffffffffffffe
0x14016653c 4889ce mov rsi, rcx
0x14016653f 4c8bbd80190000 mov r15, qword ptr [rbp + 0x1980]
0x140166546 488b02 mov rax, qword ptr [rdx]
0x140166549 48b90000000000000080 movabs rcx, 0x8000000000000000
0x140166553 48890a mov qword ptr [rdx], rcx
0x140166556 4889c1 mov rcx, rax
0x140166559 48f7d9 neg rcx
0x14016655c 0f80870c0000 jo 0x1401671e9
0x140166562 4d89ce mov r14, r9
0x140166565 4c89c3 mov rbx, r8
0x140166568 4889d7 mov rdi, rdx
0x14016656b 4883c208 add rdx, 8
0x14016656f 48898550110000 mov qword ptr [rbp + 0x1150], rax
0x140166576 488d8d58110000 lea rcx, [rbp + 0x1158]
0x14016657d 41b888000000 mov r8d, 0x88
0x140166583 e878d67b02 call 0x142923c00 ; memcpy
0x140166588 4c8ba790000000 mov r12, qword ptr [rdi + 0x90]
0x14016658f 48c7879000000000000000 mov qword ptr [rdi + 0x90], 0
0x14016659a 4d85e4 test r12, r12
0x14016659d 4c89a5c8180000 mov qword ptr [rbp + 0x18c8], r12
0x1401665a4 0f84540c0000 je 0x1401671fe
0x1401665aa 488b87e8000000 mov rax, qword ptr [rdi + 0xe8]
0x1401665b1 488985b8150000 mov qword ptr [rbp + 0x15b8], rax
0x1401665b8 0f1087d8000000 movups xmm0, xmmword ptr [rdi + 0xd8]
0x1401665bf 0f1185a8150000 movups xmmword ptr [rbp + 0x15a8], xmm0
0x1401665c6 0f108798000000 movups xmm0, xmmword ptr [rdi + 0x98]
0x1401665cd 0f108fa8000000 movups xmm1, xmmword ptr [rdi + 0xa8]
0x1401665d4 0f1097b8000000 movups xmm2, xmmword ptr [rdi + 0xb8]
0x1401665db 0f109fc8000000 movups xmm3, xmmword ptr [rdi + 0xc8]
0x1401665e2 0f119d98150000 movups xmmword ptr [rbp + 0x1598], xmm3
0x1401665e9 0f119588150000 movups xmmword ptr [rbp + 0x1588], xmm2
0x1401665f0 0f118d78150000 movups xmmword ptr [rbp + 0x1578], xmm1
0x1401665f7 0f118568150000 movups xmmword ptr [rbp + 0x1568], xmm0
0x1401665fe 488d8db0140000 lea rcx, [rbp + 0x14b0]
0x140166605 488d9550110000 lea rdx, [rbp + 0x1150]
0x14016660c 41b890000000 mov r8d, 0x90
0x140166612 e8e9d57b02 call 0x142923c00 ; memcpy
0x140166617 410f104750 movups xmm0, xmmword ptr [r15 + 0x50]
0x14016661c 0f2985b0180000 movaps xmmword ptr [rbp + 0x18b0], xmm0
0x140166623 410f104740 movups xmm0, xmmword ptr [r15 + 0x40]
0x140166628 0f2985a0180000 movaps xmmword ptr [rbp + 0x18a0], xmm0
0x14016662f 410f1007 movups xmm0, xmmword ptr [r15]
0x140166633 410f104f10 movups xmm1, xmmword ptr [r15 + 0x10]
0x140166638 410f105720 movups xmm2, xmmword ptr [r15 + 0x20]
0x14016663d 410f105f30 movups xmm3, xmmword ptr [r15 + 0x30]
0x140166642 0f299d90180000 movaps xmmword ptr [rbp + 0x1890], xmm3
0x140166649 0f299580180000 movaps xmmword ptr [rbp + 0x1880], xmm2
0x140166650 0f298d70180000 movaps xmmword ptr [rbp + 0x1870], xmm1
0x140166657 0f298560180000 movaps xmmword ptr [rbp + 0x1860], xmm0
0x14016665e 48c7854015000000000000 mov qword ptr [rbp + 0x1540], 0
0x140166669 48c7854815000008000000 mov qword ptr [rbp + 0x1548], 8
0x140166674 48c7855015000000000000 mov qword ptr [rbp + 0x1550], 0
0x14016667f 48899d58150000 mov qword ptr [rbp + 0x1558], rbx
0x140166686 4c89a560150000 mov qword ptr [rbp + 0x1560], r12
0x14016668d 4c8ba560180000 mov r12, qword ptr [rbp + 0x1860]
0x140166694 498d4c2438 lea rcx, [r12 + 0x38]
0x140166699 c6850d19000001 mov byte ptr [rbp + 0x190d], 1
0x1401666a0 c6850c19000001 mov byte ptr [rbp + 0x190c], 1
0x1401666a7 48baaa4e7896b36185a6 movabs rdx, 0xa68561b396784eaa
0x1401666b1 49b814edf4fae674fdf5 movabs r8, 0xf5fd74e6faf4ed14
0x1401666bb e83070f7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1401666c0 4885c0 test rax, rax
0x1401666c3 0f84ad0a0000 je 0x140167176
0x1401666c9 4939942418020000 cmp qword ptr [r12 + 0x218], rdx
0x1401666d1 0f869f0a0000 jbe 0x140167176
0x1401666d7 498b842410020000 mov rax, qword ptr [r12 + 0x210]
0x1401666df 488b04d0 mov rax, qword ptr [rax + rdx*8]
0x1401666e3 4885c0 test rax, rax
0x1401666e6 0f848a0a0000 je 0x140167176
0x1401666ec 4d8bac24e0010000 mov r13, qword ptr [r12 + 0x1e0]
0x1401666f4 48f7d0 not rax
0x1401666f7 488d0440 lea rax, [rax + rax*2]
0x1401666fb 48c1e005 shl rax, 5
0x1401666ff 49837c052000 cmp qword ptr [r13 + rax + 0x20], 0
0x140166705 0f846b0a0000 je 0x140167176
0x14016670b 4901c5 add r13, rax
0x14016670e 4d8b7d10 mov r15, qword ptr [r13 + 0x10]
0x140166712 458ba424b0050000 mov r12d, dword ptr [r12 + 0x5b0]
0x14016671a 4589655c mov dword ptr [r13 + 0x5c], r12d
0x14016671e c6850d19000001 mov byte ptr [rbp + 0x190d], 1
0x140166725 c6850c19000001 mov byte ptr [rbp + 0x190c], 1
0x14016672c 488d95b0140000 lea rdx, [rbp + 0x14b0]
0x140166733 4c89f9 mov rcx, r15
0x140166736 e8f5657d00 call 0x14093cd30 ; country_core::asset_libraries::texture_library::AssetTextureLibrary::import_texture_presets_to_rg
0x14016673b 4589655c mov dword ptr [r13 + 0x5c], r12d
0x14016673f 488b9568180000 mov rdx, qword ptr [rbp + 0x1868]
0x140166746 c6850d19000001 mov byte ptr [rbp + 0x190d], 1
0x14016674d c6850c19000001 mov byte ptr [rbp + 0x190c], 1
0x140166754 4c89f9 mov rcx, r15
0x140166757 e8e4687d00 call 0x14093d040 ; country_core::asset_libraries::texture_library::AssetTextureLibrary::bindless_textures_dset
0x14016675c c6850d19000001 mov byte ptr [rbp + 0x190d], 1
0x140166763 c6850c19000001 mov byte ptr [rbp + 0x190c], 1
0x14016676a 488d8d48180000 lea rcx, [rbp + 0x1848]
0x140166771 4889c2 mov rdx, rax
0x140166774 e88703a700 call 0x140bd6b00 ; rhapsody::raw::managed::handle::impl$5::clone
0x140166779 4c8ba560180000 mov r12, qword ptr [rbp + 0x1860]
0x140166780 4d8d7c2438 lea r15, [r12 + 0x38]
0x140166785 c6850f19000001 mov byte ptr [rbp + 0x190f], 1
0x14016678c c6850e19000001 mov byte ptr [rbp + 0x190e], 1
0x140166793 48baca3b97c719dcd98f movabs rdx, 0x8fd9dc19c7973bca
0x14016679d 49b8fd459bebc5f41f36 movabs r8, 0x361ff4c5eb9b45fd
0x1401667a7 4c89f9 mov rcx, r15
0x1401667aa e8416ff7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1401667af 4885c0 test rax, rax
0x1401667b2 0f84dd090000 je 0x140167195
0x1401667b8 4939942418020000 cmp qword ptr [r12 + 0x218], rdx
0x1401667c0 0f86cf090000 jbe 0x140167195
0x1401667c6 498b842410020000 mov rax, qword ptr [r12 + 0x210]
0x1401667ce 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1401667d2 4885c9 test rcx, rcx
0x1401667d5 0f84ba090000 je 0x140167195
0x1401667db 498b8424e0010000 mov rax, qword ptr [r12 + 0x1e0]
0x1401667e3 48f7d1 not rcx
0x1401667e6 488d0c49 lea rcx, [rcx + rcx*2]
0x1401667ea 48c1e105 shl rcx, 5
0x1401667ee 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1401667f4 0f849b090000 je 0x140167195
0x1401667fa 4801c8 add rax, rcx
0x1401667fd 488b5010 mov rdx, qword ptr [rax + 0x10]
0x140166801 488d8dc0000000 lea rcx, [rbp + 0xc0]
0x140166808 41b890100000 mov r8d, 0x1090
0x14016680e e8edd37b02 call 0x142923c00 ; memcpy
0x140166813 c6850f19000001 mov byte ptr [rbp + 0x190f], 1
0x14016681a c6850e19000001 mov byte ptr [rbp + 0x190e], 1
0x140166821 48ba2ea78382b74cc947 movabs rdx, 0x47c94cb78283a72e
0x14016682b 49b89d3e3297ed3d178c movabs r8, 0x8c173ded97323e9d
0x140166835 4c89f9 mov rcx, r15
0x140166838 e8b36ef7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x14016683d 4885c0 test rax, rax
0x140166840 0f846b090000 je 0x1401671b1
0x140166846 4939942418020000 cmp qword ptr [r12 + 0x218], rdx
0x14016684e 0f865d090000 jbe 0x1401671b1
0x140166854 498b842410020000 mov rax, qword ptr [r12 + 0x210]
0x14016685c 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140166860 4885c9 test rcx, rcx
0x140166863 0f8448090000 je 0x1401671b1
0x140166869 498b8424e0010000 mov rax, qword ptr [r12 + 0x1e0]
0x140166871 48f7d1 not rcx
0x140166874 488d0c49 lea rcx, [rcx + rcx*2]
0x140166878 48c1e105 shl rcx, 5
0x14016687c 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x140166882 0f8429090000 je 0x1401671b1
0x140166888 4801c8 add rax, rcx
0x14016688b 488b4810 mov rcx, qword ptr [rax + 0x10]
0x14016688f c6850f19000001 mov byte ptr [rbp + 0x190f], 1
0x140166896 c6850e19000001 mov byte ptr [rbp + 0x190e], 1
0x14016689d e8de745100 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x1401668a2 4989c7 mov r15, rax
0x1401668a5 b101 mov cl, 1
0x1401668a7 31c0 xor eax, eax
0x1401668a9 f0410fb00f lock cmpxchg byte ptr [r15], cl
0x1401668ae 0f856e080000 jne 0x140167122
0x1401668b4 c6850f19000001 mov byte ptr [rbp + 0x190f], 1
0x1401668bb c6850e19000001 mov byte ptr [rbp + 0x190e], 1
0x1401668c2 4c89f9 mov rcx, r15
0x1401668c5 e8a6b3b100 call 0x140c81c70 ; parking_lot_core::parking_lot::deadlock_impl::acquire_resource
0x1401668ca 4c89bdf8180000 mov qword ptr [rbp + 0x18f8], r15
0x1401668d1 4c89fa mov rdx, r15
0x1401668d4 4883c208 add rdx, 8
0x1401668d8 4c8bbd68180000 mov r15, qword ptr [rbp + 0x1868]
0x1401668df 48c744242000000000 mov qword ptr [rsp + 0x20], 0
0x1401668e8 488d8df0120000 lea rcx, [rbp + 0x12f0]
0x1401668ef 41b990100000 mov r9d, 0x1090
0x1401668f5 4d89f8 mov r8, r15
0x1401668f8 e843bea900 call 0x140c02740 ; rhapsody::raw::dynamic_constants::DynamicConstants::allocate
0x1401668fd 488b8df8120000 mov rcx, qword ptr [rbp + 0x12f8]
0x140166904 4881f990100000 cmp rcx, 0x1090
0x14016690b 0f8532080000 jne 0x140167143
0x140166911 488b8df0120000 mov rcx, qword ptr [rbp + 0x12f0]
0x140166918 488d95c0000000 lea rdx, [rbp + 0xc0]
0x14016691f 41b890100000 mov r8d, 0x1090
0x140166925 e8d6d27b02 call 0x142923c00 ; memcpy
0x14016692a 0f1085f0120000 movups xmm0, xmmword ptr [rbp + 0x12f0]
0x140166931 0f108d00130000 movups xmm1, xmmword ptr [rbp + 0x1300]
0x140166938 0f109510130000 movups xmm2, xmmword ptr [rbp + 0x1310]
0x14016693f 0f109d20130000 movups xmm3, xmmword ptr [rbp + 0x1320]
0x140166946 0f2985b0160000 movaps xmmword ptr [rbp + 0x16b0], xmm0
0x14016694d 0f298dc0160000 movaps xmmword ptr [rbp + 0x16c0], xmm1
0x140166954 0f2995d0160000 movaps xmmword ptr [rbp + 0x16d0], xmm2
0x14016695b 0f299de0160000 movaps xmmword ptr [rbp + 0x16e0], xmm3
0x140166962 488b8530130000 mov rax, qword ptr [rbp + 0x1330]
0x140166969 488985f0160000 mov qword ptr [rbp + 0x16f0], rax
0x140166970 488d8dc0150000 lea rcx, [rbp + 0x15c0]
0x140166977 488d95b0160000 lea rdx, [rbp + 0x16b0]
0x14016697e 4d89f8 mov r8, r15
0x140166981 e8eab9a900 call 0x140c02370 ; rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_and_storage_buffer_binding
0x140166986 c6850619000001 mov byte ptr [rbp + 0x1906], 1
0x14016698d c6850519000001 mov byte ptr [rbp + 0x1905], 1
0x140166994 4c8bbdf8180000 mov r15, qword ptr [rbp + 0x18f8]
0x14016699b 4c89f9 mov rcx, r15
0x14016699e e86db4b100 call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
0x1401669a3 31c9 xor ecx, ecx
0x1401669a5 b001 mov al, 1
0x1401669a7 f0410fb00f lock cmpxchg byte ptr [r15], cl
0x1401669ac 0f85a7070000 jne 0x140167159
0x1401669b2 4c8dbdb0160000 lea r15, [rbp + 0x16b0]
0x1401669b9 488d95b0140000 lea rdx, [rbp + 0x14b0]
0x1401669c0 41b810010000 mov r8d, 0x110
0x1401669c6 4c89f9 mov rcx, r15
0x1401669c9 e832d27b02 call 0x142923c00 ; memcpy
0x1401669ce 0f1085c0150000 movups xmm0, xmmword ptr [rbp + 0x15c0]
0x1401669d5 0f108dd0150000 movups xmm1, xmmword ptr [rbp + 0x15d0]
0x1401669dc 0f1095e0150000 movups xmm2, xmmword ptr [rbp + 0x15e0]
0x1401669e3 0f109df0150000 movups xmm3, xmmword ptr [rbp + 0x15f0]
0x1401669ea 0f1185c0170000 movups xmmword ptr [rbp + 0x17c0], xmm0
0x1401669f1 0f118dd0170000 movups xmmword ptr [rbp + 0x17d0], xmm1
0x1401669f8 0f1195e0170000 movups xmmword ptr [rbp + 0x17e0], xmm2
0x1401669ff 0f119df0170000 movups xmmword ptr [rbp + 0x17f0], xmm3
0x140166a06 0f108500160000 movups xmm0, xmmword ptr [rbp + 0x1600]
0x140166a0d 0f118500180000 movups xmmword ptr [rbp + 0x1800], xmm0
0x140166a14 0f108510160000 movups xmm0, xmmword ptr [rbp + 0x1610]
0x140166a1b 0f118510180000 movups xmmword ptr [rbp + 0x1810], xmm0
0x140166a22 0f108548180000 movups xmm0, xmmword ptr [rbp + 0x1848]
0x140166a29 0f118520180000 movups xmmword ptr [rbp + 0x1820], xmm0
0x140166a30 488b8558180000 mov rax, qword ptr [rbp + 0x1858]
0x140166a37 48898530180000 mov qword ptr [rbp + 0x1830], rax
0x140166a3e f30f6f8568180000 movdqu xmm0, xmmword ptr [rbp + 0x1868]
0x140166a46 660f70c04e pshufd xmm0, xmm0, 0x4e
0x140166a4b f30f7f8538180000 movdqu xmmword ptr [rbp + 0x1838], xmm0
0x140166a53 488b85b8180000 mov rax, qword ptr [rbp + 0x18b8]
0x140166a5a 48898520130000 mov qword ptr [rbp + 0x1320], rax
0x140166a61 0f108588180000 movups xmm0, xmmword ptr [rbp + 0x1888]
0x140166a68 0f108d98180000 movups xmm1, xmmword ptr [rbp + 0x1898]
0x140166a6f 0f1095a8180000 movups xmm2, xmmword ptr [rbp + 0x18a8]
0x140166a76 0f299510130000 movaps xmmword ptr [rbp + 0x1310], xmm2
0x140166a7d 0f298d00130000 movaps xmmword ptr [rbp + 0x1300], xmm1
0x140166a84 0f2985f0120000 movaps xmmword ptr [rbp + 0x12f0], xmm0
0x140166a8b c6850b19000001 mov byte ptr [rbp + 0x190b], 1
0x140166a92 488d8d8c140000 lea rcx, [rbp + 0x148c]
0x140166a99 488d95f0120000 lea rdx, [rbp + 0x12f0]
0x140166aa0 4d89f8 mov r8, r15
0x140166aa3 41b122 mov r9b, 0x22
0x140166aa6 e875f7a600 call 0x140bd6220 ; rhapsody::rg::import_export::impl$0::import_owned
0x140166aab 488b9578180000 mov rdx, qword ptr [rbp + 0x1878]
0x140166ab2 c6850b19000001 mov byte ptr [rbp + 0x190b], 1
0x140166ab9 488d8dd0180000 lea rcx, [rbp + 0x18d0]
0x140166ac0 4c8d85b0160000 lea r8, [rbp + 0x16b0]
0x140166ac7 4531c9 xor r9d, r9d
0x140166aca e8d1f5a600 call 0x140bd60a0 ; rhapsody::rg::import_export::impl$0::import_unowned
0x140166acf 4c8bbd60180000 mov r15, qword ptr [rbp + 0x1860]
0x140166ad6 488b8580180000 mov rax, qword ptr [rbp + 0x1880]
0x140166add 0fb680c2000000 movzx eax, byte ptr [rax + 0xc2]
0x140166ae4 c6850b19000001 mov byte ptr [rbp + 0x190b], 1
0x140166aeb 88442420 mov byte ptr [rsp + 0x20], al
0x140166aef 488d8db0160000 lea rcx, [rbp + 0x16b0]
0x140166af6 4c8d858c140000 lea r8, [rbp + 0x148c]
0x140166afd 4c8d8dd0180000 lea r9, [rbp + 0x18d0]
0x140166b04 4c89fa mov rdx, r15
0x140166b07 e8446d7a00 call 0x14090d850 ; country_core::systems::render_loop::draw_passes::final_post::resolve_to_swapchain
0x140166b0c 8b85f0180000 mov eax, dword ptr [rbp + 0x18f0]
0x140166b12 898510130000 mov dword ptr [rbp + 0x1310], eax
0x140166b18 0f1085d0180000 movups xmm0, xmmword ptr [rbp + 0x18d0]
0x140166b1f 0f108de0180000 movups xmm1, xmmword ptr [rbp + 0x18e0]
0x140166b26 0f298d00130000 movaps xmmword ptr [rbp + 0x1300], xmm1
0x140166b2d 0f2985f0120000 movaps xmmword ptr [rbp + 0x12f0], xmm0
0x140166b34 c6850b19000001 mov byte ptr [rbp + 0x190b], 1
0x140166b3b 488d8df0120000 lea rcx, [rbp + 0x12f0]
0x140166b42 488d95b0160000 lea rdx, [rbp + 0x16b0]
0x140166b49 41b027 mov r8b, 0x27
0x140166b4c e82ff8a600 call 0x140bd6380 ; rhapsody::rg::import_export::impl$0::export
0x140166b51 4c8da5f0120000 lea r12, [rbp + 0x12f0]
0x140166b58 488d95b0160000 lea rdx, [rbp + 0x16b0]
0x140166b5f 41b898010000 mov r8d, 0x198
0x140166b65 4c89e1 mov rcx, r12
0x140166b68 e893d07b02 call 0x142923c00 ; memcpy
0x140166b6d c6850b19000000 mov byte ptr [rbp + 0x190b], 0
0x140166b74 488d8de0110000 lea rcx, [rbp + 0x11e0]
0x140166b7b 4c89e2 mov rdx, r12
0x140166b7e e8cd6e8700 call 0x1409dda50 ; country_core::systems::render_loop::RenderContext::finish
0x140166b83 498d4f38 lea rcx, [r15 + 0x38]
0x140166b87 48baaa4e7896b36185a6 movabs rdx, 0xa68561b396784eaa
0x140166b91 49b814edf4fae674fdf5 movabs r8, 0xf5fd74e6faf4ed14
0x140166b9b e8506bf7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x140166ba0 4885c0 test rax, rax
0x140166ba3 0f8424060000 je 0x1401671cd
0x140166ba9 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x140166bb0 0f8617060000 jbe 0x1401671cd
0x140166bb6 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x140166bbd 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140166bc1 4885c9 test rcx, rcx
0x140166bc4 0f8403060000 je 0x1401671cd
0x140166bca 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x140166bd1 48f7d1 not rcx
0x140166bd4 488d0c49 lea rcx, [rcx + rcx*2]
0x140166bd8 48c1e105 shl rcx, 5
0x140166bdc 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x140166be2 0f84e5050000 je 0x1401671cd
0x140166be8 4801c8 add rax, rcx
0x140166beb 488b4810 mov rcx, qword ptr [rax + 0x10]
0x140166bef 418b97b0050000 mov edx, dword ptr [r15 + 0x5b0]
0x140166bf6 89505c mov dword ptr [rax + 0x5c], edx
0x140166bf9 488d95e0110000 lea rdx, [rbp + 0x11e0]
0x140166c00 e82b627d00 call 0x14093ce30 ; country_core::asset_libraries::texture_library::AssetTextureLibrary::export_preset_textures_from_rg
0x140166c05 4c8bbd60180000 mov r15, qword ptr [rbp + 0x1860]
0x140166c0c 498d4f38 lea rcx, [r15 + 0x38]
0x140166c10 48ba5593676e78cc2682 movabs rdx, 0x8226cc786e679355
0x140166c1a 49b8ac41a8556adacf65 movabs r8, 0x65cfda6a55a841ac
0x140166c24 e8c76af7ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x140166c29 4885c0 test rax, rax
0x140166c2c 0f84a9050000 je 0x1401671db
0x140166c32 49399718020000 cmp qword ptr [r15 + 0x218], rdx
0x140166c39 0f869c050000 jbe 0x1401671db
0x140166c3f 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x140166c46 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140166c4a 4885c9 test rcx, rcx
0x140166c4d 0f8488050000 je 0x1401671db
0x140166c53 498b87e0010000 mov rax, qword ptr [r15 + 0x1e0]
0x140166c5a 48f7d1 not rcx
0x140166c5d 488d0c49 lea rcx, [rcx + rcx*2]
0x140166c61 48c1e105 shl rcx, 5
0x140166c65 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x140166c6b 0f846a050000 je 0x1401671db
0x140166c71 4801c8 add rax, rcx
0x140166c74 488b4810 mov rcx, qword ptr [rax + 0x10]
0x140166c78 418b97b0050000 mov edx, dword ptr [r15 + 0x5b0]
0x140166c7f 89505c mov dword ptr [rax + 0x5c], edx
0x140166c82 488d95e0110000 lea rdx, [rbp + 0x11e0]
0x140166c89 e8227f7b00 call 0x14091ebb0 ; country_core::asset_libraries::ssbo_library::AssetSsboLibrary::export_from_rg
0x140166c8e 4c8d7db0 lea r15, [rbp - 0x50]
0x140166c92 488d95e0110000 lea rdx, [rbp + 0x11e0]
0x140166c99 41b810010000 mov r8d, 0x110
0x140166c9f 4c89f9 mov rcx, r15
0x140166ca2 e859cf7b02 call 0x142923c00 ; memcpy
0x140166ca7 488d8dc0000000 lea rcx, [rbp + 0xc0]
0x140166cae 4c89fa mov rdx, r15
0x140166cb1 e8ea0f3900 call 0x1404f7ca0 ; rhapsody::rg::temporal::TemporalRenderGraphRecorder::export_temporal<country_core::render::pass_builder::pass_resources::PassResources>
0x140166cb6 4c8dbf90000000 lea r15, [rdi + 0x90]
0x140166cbd 488d8de0110000 lea rcx, [rbp + 0x11e0]
0x140166cc4 488d95c0000000 lea rdx, [rbp + 0xc0]
0x140166ccb 41b8a8000000 mov r8d, 0xa8
0x140166cd1 e82acf7b02 call 0x142923c00 ; memcpy
0x140166cd6 0f108568010000 movups xmm0, xmmword ptr [rbp + 0x168]
0x140166cdd 0f108d78010000 movups xmm1, xmmword ptr [rbp + 0x178]
0x140166ce4 0f109588010000 movups xmm2, xmmword ptr [rbp + 0x188]
0x140166ceb 0f109d98010000 movups xmm3, xmmword ptr [rbp + 0x198]
0x140166cf2 0f298550160000 movaps xmmword ptr [rbp + 0x1650], xmm0
0x140166cf9 0f298d60160000 movaps xmmword ptr [rbp + 0x1660], xmm1
0x140166d00 0f299570160000 movaps xmmword ptr [rbp + 0x1670], xmm2
0x140166d07 0f299d80160000 movaps xmmword ptr [rbp + 0x1680], xmm3
0x140166d0e 0f1085a8010000 movups xmm0, xmmword ptr [rbp + 0x1a8]
0x140166d15 0f298590160000 movaps xmmword ptr [rbp + 0x1690], xmm0
0x140166d1c 0f1085b8010000 movups xmm0, xmmword ptr [rbp + 0x1b8]
0x140166d23 0f2985a0160000 movaps xmmword ptr [rbp + 0x16a0], xmm0
0x140166d2a 410f104750 movups xmm0, xmmword ptr [r15 + 0x50]
0x140166d2f 0f298510010000 movaps xmmword ptr [rbp + 0x110], xmm0
0x140166d36 410f104740 movups xmm0, xmmword ptr [r15 + 0x40]
0x140166d3b 0f298500010000 movaps xmmword ptr [rbp + 0x100], xmm0
0x140166d42 410f1007 movups xmm0, xmmword ptr [r15]
0x140166d46 410f104f10 movups xmm1, xmmword ptr [r15 + 0x10]
0x140166d4b 410f105720 movups xmm2, xmmword ptr [r15 + 0x20]
0x140166d50 410f105f30 movups xmm3, xmmword ptr [r15 + 0x30]
0x140166d55 0f299df0000000 movaps xmmword ptr [rbp + 0xf0], xmm3
0x140166d5c 0f2995e0000000 movaps xmmword ptr [rbp + 0xe0], xmm2
0x140166d63 0f298dd0000000 movaps xmmword ptr [rbp + 0xd0], xmm1
0x140166d6a 0f2985c0000000 movaps xmmword ptr [rbp + 0xc0], xmm0
0x140166d71 0f288550160000 movaps xmm0, xmmword ptr [rbp + 0x1650]
0x140166d78 0f288d60160000 movaps xmm1, xmmword ptr [rbp + 0x1660]
0x140166d7f 0f289570160000 movaps xmm2, xmmword ptr [rbp + 0x1670]
0x140166d86 0f289d80160000 movaps xmm3, xmmword ptr [rbp + 0x1680]
0x140166d8d 410f1107 movups xmmword ptr [r15], xmm0
0x140166d91 410f114f10 movups xmmword ptr [r15 + 0x10], xmm1
0x140166d96 410f115720 movups xmmword ptr [r15 + 0x20], xmm2
0x140166d9b 410f115f30 movups xmmword ptr [r15 + 0x30], xmm3
0x140166da0 0f288590160000 movaps xmm0, xmmword ptr [rbp + 0x1690]
0x140166da7 410f114740 movups xmmword ptr [r15 + 0x40], xmm0
0x140166dac 0f2885a0160000 movaps xmm0, xmmword ptr [rbp + 0x16a0]
0x140166db3 410f114750 movups xmmword ptr [r15 + 0x50], xmm0
0x140166db8 4883bdc000000000 cmp qword ptr [rbp + 0xc0], 0
0x140166dc0 7457 je 0x140166e19
0x140166dc2 488d95e0000000 lea rdx, [rbp + 0xe0]
0x140166dc9 488d8dc0000000 lea rcx, [rbp + 0xc0]
0x140166dd0 41b868000000 mov r8d, 0x68
0x140166dd6 41b910000000 mov r9d, 0x10
0x140166ddc e8cf60eeff call 0x14004ceb0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>,alloc::alloc::Global>
0x140166de1 488b95f8000000 mov rdx, qword ptr [rbp + 0xf8]
0x140166de8 4885d2 test rdx, rdx
0x140166deb 742c je 0x140166e19
0x140166ded 4889d0 mov rax, rdx
0x140166df0 48c1e004 shl rax, 4
0x140166df4 4801c2 add rdx, rax
0x140166df7 4883c221 add rdx, 0x21
0x140166dfb 741c je 0x140166e19
0x140166dfd 488b8df0000000 mov rcx, qword ptr [rbp + 0xf0]
0x140166e04 48f7d8 neg rax
0x140166e07 4801c1 add rcx, rax
0x140166e0a 4883c1f0 add rcx, -0x10
0x140166e0e 41b810000000 mov r8d, 0x10
0x140166e14 e807ce4a00 call 0x140613c20 ; __rust_dealloc
0x140166e19 488d8dc0150000 lea rcx, [rbp + 0x15c0]
0x140166e20 488d95e0110000 lea rdx, [rbp + 0x11e0]
0x140166e27 41b890000000 mov r8d, 0x90
0x140166e2d e8cecd7b02 call 0x142923c00 ; memcpy
0x140166e32 488b8580120000 mov rax, qword ptr [rbp + 0x1280]
0x140166e39 488985e0180000 mov qword ptr [rbp + 0x18e0], rax
0x140166e40 0f108570120000 movups xmm0, xmmword ptr [rbp + 0x1270]
0x140166e47 0f2985d0180000 movaps xmmword ptr [rbp + 0x18d0], xmm0
0x140166e4e 48c785c000000000000000 mov qword ptr [rbp + 0xc0], 0
0x140166e59 48c785c800000008000000 mov qword ptr [rbp + 0xc8], 8
0x140166e64 48c785d000000000000000 mov qword ptr [rbp + 0xd0], 0
0x140166e6f 31c9 xor ecx, ecx
0x140166e71 e8aa2cf7ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
0x140166e76 4885c0 test rax, rax
0x140166e79 0f848d030000 je 0x14016720c
0x140166e7f 0f1000 movups xmm0, xmmword ptr [rax]
0x140166e82 48ff00 inc qword ptr [rax]
0x140166e85 0f100da4918402 movups xmm1, xmmword ptr [rip + 0x28491a4]
0x140166e8c 0f118d98170000 movups xmmword ptr [rbp + 0x1798], xmm1
0x140166e93 0f100d86918402 movups xmm1, xmmword ptr [rip + 0x2849186]
0x140166e9a 0f118d88170000 movups xmmword ptr [rbp + 0x1788], xmm1
0x140166ea1 0f1185a8170000 movups xmmword ptr [rbp + 0x17a8], xmm0
0x140166ea8 48c785b016000000000000 mov qword ptr [rbp + 0x16b0], 0
0x140166eb3 48c785b816000002000000 mov qword ptr [rbp + 0x16b8], 2
0x140166ebe 0f57c0 xorps xmm0, xmm0
0x140166ec1 0f1185c0160000 movups xmmword ptr [rbp + 0x16c0], xmm0
0x140166ec8 48c785d016000001000000 mov qword ptr [rbp + 0x16d0], 1
0x140166ed3 48c785d816000000000000 mov qword ptr [rbp + 0x16d8], 0
0x140166ede 0f108dc0000000 movups xmm1, xmmword ptr [rbp + 0xc0]
0x140166ee5 0f118de0160000 movups xmmword ptr [rbp + 0x16e0], xmm1
0x140166eec 488b85d0000000 mov rax, qword ptr [rbp + 0xd0]
0x140166ef3 488985f0160000 mov qword ptr [rbp + 0x16f0], rax
0x140166efa 488d8df8160000 lea rcx, [rbp + 0x16f8]
0x140166f01 48c785f816000000000000 mov qword ptr [rbp + 0x16f8], 0
0x140166f0c 48c7850017000004000000 mov qword ptr [rbp + 0x1700], 4
0x140166f17 0f118508170000 movups xmmword ptr [rbp + 0x1708], xmm0
0x140166f1e 48c7851817000004000000 mov qword ptr [rbp + 0x1718], 4
0x140166f29 0f118520170000 movups xmmword ptr [rbp + 0x1720], xmm0
0x140166f30 48c7853017000004000000 mov qword ptr [rbp + 0x1730], 4
0x140166f3b 0f118538170000 movups xmmword ptr [rbp + 0x1738], xmm0
0x140166f42 48c7854817000004000000 mov qword ptr [rbp + 0x1748], 4
0x140166f4d 0f118550170000 movups xmmword ptr [rbp + 0x1750], xmm0
0x140166f54 48c7856017000004000000 mov qword ptr [rbp + 0x1760], 4
0x140166f5f 0f118568170000 movups xmmword ptr [rbp + 0x1768], xmm0
0x140166f66 48c7857817000004000000 mov qword ptr [rbp + 0x1778], 4
0x140166f71 48c7858017000000000000 mov qword ptr [rbp + 0x1780], 0
0x140166f7c 4c8b85d8180000 mov r8, qword ptr [rbp + 0x18d8]
0x140166f83 4c8b8de0180000 mov r9, qword ptr [rbp + 0x18e0]
0x140166f8a c6850819000001 mov byte ptr [rbp + 0x1908], 1
0x140166f91 c6850719000001 mov byte ptr [rbp + 0x1907], 1
0x140166f98 488d95c0150000 lea rdx, [rbp + 0x15c0]
0x140166f9f e88c943c00 call 0x140530430 ; rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources>
0x140166fa4 c6850819000001 mov byte ptr [rbp + 0x1908], 1
0x140166fab c6850719000001 mov byte ptr [rbp + 0x1907], 1
0x140166fb2 488d8db0160000 lea rcx, [rbp + 0x16b0]
0x140166fb9 488d95c0150000 lea rdx, [rbp + 0x15c0]
0x140166fc0 4d89f0 mov r8, r14
0x140166fc3 4989d9 mov r9, rbx
0x140166fc6 e8d50ba800 call 0x140be7ba0 ; rhapsody::rg::resource_registry::ResourceRegistry::populate_resources
0x140166fcb 84c0 test al, al
0x140166fcd 0f84a8000000 je 0x14016707b
0x140166fd3 c6850819000001 mov byte ptr [rbp + 0x1908], 1
0x140166fda c6850719000001 mov byte ptr [rbp + 0x1907], 1
0x140166fe1 488d8dc0150000 lea rcx, [rbp + 0x15c0]
0x140166fe8 e83324a800 call 0x140be9420 ; rhapsody::rg::graph::RenderGraph::on_retire
0x140166fed 488d8dc0000000 lea rcx, [rbp + 0xc0]
0x140166ff4 41b890000000 mov r8d, 0x90
0x140166ffa 4889fa mov rdx, rdi
0x140166ffd e8fecb7b02 call 0x142923c00 ; memcpy
0x140167002 488d95c0150000 lea rdx, [rbp + 0x15c0]
0x140167009 41b890000000 mov r8d, 0x90
0x14016700f 4889f9 mov rcx, rdi
0x140167012 e8e9cb7b02 call 0x142923c00 ; memcpy
0x140167017 31c0 xor eax, eax
0x140167019 483b85c0000000 cmp rax, qword ptr [rbp + 0xc0]
0x140167020 701a jo 0x14016703c
0x140167022 c6850819000000 mov byte ptr [rbp + 0x1908], 0
0x140167029 c6850719000001 mov byte ptr [rbp + 0x1907], 1
0x140167030 488d8dc0000000 lea rcx, [rbp + 0xc0]
0x140167037 e8d4814800 call 0x1405ef210 ; core::ptr::drop_in_place<rhapsody::rg::graph::RenderGraph>
0x14016703c 48b80000000000000080 movabs rax, 0x8000000000000000
0x140167046 488906 mov qword ptr [rsi], rax
0x140167049 c6850a19000000 mov byte ptr [rbp + 0x190a], 0
0x140167050 c6850919000001 mov byte ptr [rbp + 0x1909], 1
0x140167057 488d8db0160000 lea rcx, [rbp + 0x16b0]
0x14016705e e86d614900 call 0x1405fd1d0 ; core::ptr::drop_in_place<rhapsody::rg::resource_registry::ResourceRegistry>
0x140167063 c6850219000000 mov byte ptr [rbp + 0x1902], 0
0x14016706a 488d8dd0180000 lea rcx, [rbp + 0x18d0]
0x140167071 e8aa374700 call 0x1405da820 ; core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources>,alloc::alloc::Global> >
0x140167076 e990000000 jmp 0x14016710b
0x14016707b 488dbdb0140000 lea rdi, [rbp + 0x14b0]
0x140167082 488d95c0150000 lea rdx, [rbp + 0x15c0]
0x140167089 41b890000000 mov r8d, 0x90
0x14016708f 4889f9 mov rcx, rdi
0x140167092 e869cb7b02 call 0x142923c00 ; memcpy
0x140167097 488b85e0180000 mov rax, qword ptr [rbp + 0x18e0]
0x14016709e 48898570180000 mov qword ptr [rbp + 0x1870], rax
0x1401670a5 0f2885d0180000 movaps xmm0, xmmword ptr [rbp + 0x18d0]
0x1401670ac 0f298560180000 movaps xmmword ptr [rbp + 0x1860], xmm0
0x1401670b3 488d9df0120000 lea rbx, [rbp + 0x12f0]
0x1401670ba 488d95b0160000 lea rdx, [rbp + 0x16b0]
0x1401670c1 41b808010000 mov r8d, 0x108
0x1401670c7 4889d9 mov rcx, rbx
0x1401670ca e831cb7b02 call 0x142923c00 ; memcpy
0x1401670cf c6850819000000 mov byte ptr [rbp + 0x1908], 0
0x1401670d6 c6850719000000 mov byte ptr [rbp + 0x1907], 0
0x1401670dd 488d8dc0000000 lea rcx, [rbp + 0xc0]
0x1401670e4 4c8d8560180000 lea r8, [rbp + 0x1860]
0x1401670eb 4889fa mov rdx, rdi
0x1401670ee 4989d9 mov r9, rbx
0x1401670f1 e88ad11500 call 0x1402c4280 ; rhapsody::rg::compiled::CompiledRenderGraph::new<country_core::render::pass_builder::pass_resources::PassResources>
0x1401670f6 488d95c0000000 lea rdx, [rbp + 0xc0]
0x1401670fd 41b818020000 mov r8d, 0x218
0x140167103 4889f1 mov rcx, rsi
0x140167106 e8f5ca7b02 call 0x142923c00 ; memcpy
0x14016710b 4889f0 mov rax, rsi
0x14016710e 4881c498190000 add rsp, 0x1998
0x140167115 5b pop rbx
0x140167116 5f pop rdi
0x140167117 5e pop rsi
0x140167118 415c pop r12
0x14016711a 415d pop r13
0x14016711c 415e pop r14
0x14016711e 415f pop r15
0x140167120 5d pop rbp
0x140167121 c3 ret 
0x140167122 c6850f19000001 mov byte ptr [rbp + 0x190f], 1
0x140167129 c6850e19000001 mov byte ptr [rbp + 0x190e], 1
0x140167130 4c89f9 mov rcx, r15
0x140167133 41b800ca9a3b mov r8d, 0x3b9aca00
0x140167139 e8d2607602 call 0x1428cd210 ; parking_lot::raw_mutex::RawMutex::lock_slow
0x14016713e e971f7ffff jmp 0x1401668b4
0x140167143 4c8d0566778702 lea r8, [rip + 0x2877766]
0x14016714a ba90100000 mov edx, 0x1090
0x14016714f e88c277702 call 0x1428d98e0 ; core::slice::impl$0::copy_from_slice::len_mismatch_fail
0x140167154 e9bf000000 jmp 0x140167218
0x140167159 c6850619000001 mov byte ptr [rbp + 0x1906], 1
0x140167160 c6850519000001 mov byte ptr [rbp + 0x1905], 1
0x140167167 4c89f9 mov rcx, r15
0x14016716a 31d2 xor edx, edx
0x14016716c e81f6a7602 call 0x1428cdb90 ; parking_lot::raw_mutex::RawMutex::unlock_slow
0x140167171 e93cf8ffff jmp 0x1401669b2
0x140167176 c6850d19000001 mov byte ptr [rbp + 0x190d], 1
0x14016717d c6850c19000001 mov byte ptr [rbp + 0x190c], 1
0x140167184 488d0d25677f02 lea rcx, [rip + 0x27f6725]
0x14016718b e880217702 call 0x1428d9310 ; core::option::unwrap_failed
0x140167190 e983000000 jmp 0x140167218
0x140167195 c6850f19000001 mov byte ptr [rbp + 0x190f], 1
0x14016719c c6850e19000001 mov byte ptr [rbp + 0x190e], 1
0x1401671a3 488d0d1e677f02 lea rcx, [rip + 0x27f671e]
0x1401671aa e861217702 call 0x1428d9310 ; core::option::unwrap_failed
0x1401671af eb67 jmp 0x140167218
0x1401671b1 c6850f19000001 mov byte ptr [rbp + 0x190f], 1
0x1401671b8 c6850e19000001 mov byte ptr [rbp + 0x190e], 1
0x1401671bf 488d0d1a677f02 lea rcx, [rip + 0x27f671a]
0x1401671c6 e845217702 call 0x1428d9310 ; core::option::unwrap_failed
0x1401671cb eb4b jmp 0x140167218
0x1401671cd 488d0d24677f02 lea rcx, [rip + 0x27f6724]
0x1401671d4 e837217702 call 0x1428d9310 ; core::option::unwrap_failed
0x1401671d9 eb3d jmp 0x140167218
0x1401671db 488d0d2e677f02 lea rcx, [rip + 0x27f672e]
0x1401671e2 e829217702 call 0x1428d9310 ; core::option::unwrap_failed
0x1401671e7 eb2f jmp 0x140167218
0x1401671e9 c6850419000001 mov byte ptr [rbp + 0x1904], 1
0x1401671f0 488d0de1637f02 lea rcx, [rip + 0x27f63e1]
0x1401671f7 e814217702 call 0x1428d9310 ; core::option::unwrap_failed
0x1401671fc eb1a jmp 0x140167218
0x1401671fe 488d0deb637f02 lea rcx, [rip + 0x27f63eb]
0x140167205 e806217702 call 0x1428d9310 ; core::option::unwrap_failed
0x14016720a eb0c jmp 0x140167218
0x14016720c 488d0dfdb77c02 lea rcx, [rip + 0x27cb7fd]
0x140167213 e8b80e7702 call 0x1428d80d0 ; std::thread::local::panic_access_error
0x140167218 0f0b ud2 
0x14016721a 660f1f440000 nop word ptr [rax + rax]
0x140167220 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140167225 55 push rbp
0x140167226 4157 push r15
0x140167228 4156 push r14
0x14016722a 4155 push r13
0x14016722c 4154 push r12
0x14016722e 56 push rsi
0x14016722f 57 push rdi
0x140167230 53 push rbx
0x140167231 4883ec38 sub rsp, 0x38
0x140167235 488daa80000000 lea rbp, [rdx + 0x80]
0x14016723c 0f108510130000 movups xmm0, xmmword ptr [rbp + 0x1310]
0x140167243 0f108d20130000 movups xmm1, xmmword ptr [rbp + 0x1320]
0x14016724a 0f298dc0160000 movaps xmmword ptr [rbp + 0x16c0], xmm1
0x140167251 0f2985b0160000 movaps xmmword ptr [rbp + 0x16b0], xmm0
0x140167258 488d8db0160000 lea rcx, [rbp + 0x16b0]
0x14016725f e84cb2a700 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
0x140167264 90 nop 
0x140167265 4883c438 add rsp, 0x38
0x140167269 5b pop rbx
0x14016726a 5f pop rdi
0x14016726b 5e pop rsi
0x14016726c 415c pop r12
0x14016726e 415d pop r13
0x140167270 415e pop r14
0x140167272 415f pop r15
0x140167274 5d pop rbp
0x140167275 c3 ret 
0x140167276 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140167280 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140167285 55 push rbp
0x140167286 4157 push r15
0x140167288 4156 push r14
0x14016728a 4155 push r13
0x14016728c 4154 push r12
0x14016728e 56 push rsi
0x14016728f 57 push rdi
0x140167290 53 push rbx
0x140167291 4883ec38 sub rsp, 0x38
0x140167295 488daa80000000 lea rbp, [rdx + 0x80]
0x14016729c 80bd0b19000000 cmp byte ptr [rbp + 0x190b], 0
0x1401672a3 742b je 0x1401672d0
0x1401672a5 488d8db0160000 lea rcx, [rbp + 0x16b0]
0x1401672ac e82f594900 call 0x1405fcbe0 ; core::ptr::drop_in_place<country_core::systems::render_loop::RenderContext>
0x1401672b1 c6850619000000 mov byte ptr [rbp + 0x1906], 0
0x1401672b8 c6850519000000 mov byte ptr [rbp + 0x1905], 0
0x1401672bf 4883c438 add rsp, 0x38
0x1401672c3 5b pop rbx
0x1401672c4 5f pop rdi
0x1401672c5 5e pop rsi
0x1401672c6 415c pop r12
0x1401672c8 415d pop r13
0x1401672ca 415e pop r14
0x1401672cc 415f pop r15
0x1401672ce 5d pop rbp
0x1401672cf c3 ret 
0x1401672d0 c6850619000000 mov byte ptr [rbp + 0x1906], 0
0x1401672d7 c6850519000000 mov byte ptr [rbp + 0x1905], 0
0x1401672de 4883c438 add rsp, 0x38
0x1401672e2 5b pop rbx
0x1401672e3 5f pop rdi
0x1401672e4 5e pop rsi
0x1401672e5 415c pop r12
0x1401672e7 415d pop r13
0x1401672e9 415e pop r14
0x1401672eb 415f pop r15
0x1401672ed 5d pop rbp
0x1401672ee c3 ret 
0x1401672ef 90 nop 
0x1401672f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1401672f5 55 push rbp
0x1401672f6 4157 push r15
0x1401672f8 4156 push r14
0x1401672fa 4155 push r13
0x1401672fc 4154 push r12
0x1401672fe 56 push rsi
0x1401672ff 57 push rdi
0x140167300 53 push rbx
0x140167301 4883ec38 sub rsp, 0x38
0x140167305 488daa80000000 lea rbp, [rdx + 0x80]
0x14016730c 0fb69d05190000 movzx ebx, byte ptr [rbp + 0x1905]
0x140167313 0fb6b506190000 movzx esi, byte ptr [rbp + 0x1906]
0x14016731a f6c301 test bl, 1
0x14016731d 742e je 0x14016734d
0x14016731f 488d8dc0150000 lea rcx, [rbp + 0x15c0]
0x140167326 e825a8ffff call 0x140161b50 ; core::ptr::drop_in_place<rhapsody::raw::managed::handle::GpuOwned<array$<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,2>,rhapsody::raw::managed::handle::RawManagedHandle> >
0x14016732b 4080e601 and sil, 1
0x14016732f 4088b50f190000 mov byte ptr [rbp + 0x190f], sil
0x140167336 889d0e190000 mov byte ptr [rbp + 0x190e], bl
0x14016733c 4883c438 add rsp, 0x38
0x140167340 5b pop rbx
0x140167341 5f pop rdi
0x140167342 5e pop rsi
0x140167343 415c pop r12
0x140167345 415d pop r13
0x140167347 415e pop r14
0x140167349 415f pop r15
0x14016734b 5d pop rbp
0x14016734c c3 ret 
0x14016734d 4080e601 and sil, 1
0x140167351 4088b50f190000 mov byte ptr [rbp + 0x190f], sil
0x140167358 889d0e190000 mov byte ptr [rbp + 0x190e], bl
0x14016735e 4883c438 add rsp, 0x38
0x140167362 5b pop rbx
0x140167363 5f pop rdi
0x140167364 5e pop rsi
0x140167365 415c pop r12
0x140167367 415d pop r13
0x140167369 415e pop r14
0x14016736b 415f pop r15
0x14016736d 5d pop rbp
0x14016736e c3 ret 
0x14016736f 90 nop 
0x140167370 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140167375 55 push rbp
0x140167376 4157 push r15
0x140167378 4156 push r14
0x14016737a 4155 push r13
0x14016737c 4154 push r12
0x14016737e 56 push rsi
0x14016737f 57 push rdi
0x140167380 53 push rbx
0x140167381 4883ec38 sub rsp, 0x38
0x140167385 488daa80000000 lea rbp, [rdx + 0x80]
0x14016738c 0fb69d0e190000 movzx ebx, byte ptr [rbp + 0x190e]
0x140167393 0fb6b50f190000 movzx esi, byte ptr [rbp + 0x190f]
0x14016739a f6c301 test bl, 1
0x14016739d 742e je 0x1401673cd
0x14016739f 488d8d48180000 lea rcx, [rbp + 0x1848]
0x1401673a6 e805b1a700 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
0x1401673ab 4080e601 and sil, 1
0x1401673af 4088b50d190000 mov byte ptr [rbp + 0x190d], sil
0x1401673b6 889d0c190000 mov byte ptr [rbp + 0x190c], bl
0x1401673bc 4883c438 add rsp, 0x38
0x1401673c0 5b pop rbx
0x1401673c1 5f pop rdi
0x1401673c2 5e pop rsi
0x1401673c3 415c pop r12
0x1401673c5 415d pop r13
0x1401673c7 415e pop r14
0x1401673c9 415f pop r15
0x1401673cb 5d pop rbp
0x1401673cc c3 ret 
0x1401673cd 4080e601 and sil, 1
0x1401673d1 4088b50d190000 mov byte ptr [rbp + 0x190d], sil
0x1401673d8 889d0c190000 mov byte ptr [rbp + 0x190c], bl
0x1401673de 4883c438 add rsp, 0x38
0x1401673e2 5b pop rbx
0x1401673e3 5f pop rdi
0x1401673e4 5e pop rsi
0x1401673e5 415c pop r12
0x1401673e7 415d pop r13
0x1401673e9 415e pop r14
0x1401673eb 415f pop r15
0x1401673ed 5d pop rbp
0x1401673ee c3 ret 
0x1401673ef 90 nop 
0x1401673f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1401673f5 55 push rbp
0x1401673f6 4157 push r15
0x1401673f8 4156 push r14
0x1401673fa 4155 push r13
0x1401673fc 4154 push r12
0x1401673fe 56 push rsi
0x1401673ff 57 push rdi
0x140167400 53 push rbx
0x140167401 4883ec38 sub rsp, 0x38
0x140167405 488daa80000000 lea rbp, [rdx + 0x80]
0x14016740c 0fb69d0d190000 movzx ebx, byte ptr [rbp + 0x190d]
0x140167413 f6850c19000001 test byte ptr [rbp + 0x190c], 1
0x14016741a 740c je 0x140167428
0x14016741c 488d8db0140000 lea rcx, [rbp + 0x14b0]
0x140167423 e898594700 call 0x1405dcdc0 ; core::ptr::drop_in_place<rhapsody::rg::temporal::TemporalRenderGraphRecorder<country_core::render::pass_builder::pass_resources::PassResources> >
0x140167428 f6c301 test bl, 1
0x14016742b 740c je 0x140167439
0x14016742d 488d8d88180000 lea rcx, [rbp + 0x1888]
0x140167434 e8a7714800 call 0x1405ee5e0 ; core::ptr::drop_in_place<rhapsody::raw::texture::Texture>
0x140167439 90 nop 
0x14016743a 4883c438 add rsp, 0x38
0x14016743e 5b pop rbx
0x14016743f 5f pop rdi
0x140167440 5e pop rsi
0x140167441 415c pop r12
0x140167443 415d pop r13
0x140167445 415e pop r14
0x140167447 415f pop r15
0x140167449 5d pop rbp
0x14016744a c3 ret 
0x14016744b 0f1f440000 nop dword ptr [rax + rax]
0x140167450 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140167455 55 push rbp
0x140167456 4157 push r15
0x140167458 4156 push r14
0x14016745a 4155 push r13
0x14016745c 4154 push r12
0x14016745e 56 push rsi
0x14016745f 57 push rdi
0x140167460 53 push rbx
0x140167461 4883ec38 sub rsp, 0x38
0x140167465 488daa80000000 lea rbp, [rdx + 0x80]
0x14016746c c6850a19000001 mov byte ptr [rbp + 0x190a], 1
0x140167473 c6850919000001 mov byte ptr [rbp + 0x1909], 1
0x14016747a 488d8dc0000000 lea rcx, [rbp + 0xc0]
0x140167481 e82a414a00 call 0x14060b5b0 ; core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::rg::resource_registry::RegistryResource,alloc::alloc::Global> >
0x140167486 c6850a19000001 mov byte ptr [rbp + 0x190a], 1
0x14016748d c6850919000001 mov byte ptr [rbp + 0x1909], 1
0x140167494 4883c438 add rsp, 0x38
0x140167498 5b pop rbx
0x140167499 5f pop rdi
0x14016749a 5e pop rsi
0x14016749b 415c pop r12
0x14016749d 415d pop r13
0x14016749f 415e pop r14
0x1401674a1 415f pop r15
0x1401674a3 5d pop rbp
0x1401674a4 c3 ret 
0x1401674a5 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1401674b0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1401674b5 55 push rbp
0x1401674b6 4157 push r15
0x1401674b8 4156 push r14
0x1401674ba 4155 push r13
0x1401674bc 4154 push r12
0x1401674be 56 push rsi
0x1401674bf 57 push rdi
0x1401674c0 53 push rbx
0x1401674c1 4883ec38 sub rsp, 0x38
0x1401674c5 488daa80000000 lea rbp, [rdx + 0x80]
0x1401674cc 0fb69d07190000 movzx ebx, byte ptr [rbp + 0x1907]
0x1401674d3 0fb6b508190000 movzx esi, byte ptr [rbp + 0x1908]
0x1401674da f6c301 test bl, 1
0x1401674dd 742e je 0x14016750d
0x1401674df 488d8db0160000 lea rcx, [rbp + 0x16b0]
0x1401674e6 e8e55c4900 call 0x1405fd1d0 ; core::ptr::drop_in_place<rhapsody::rg::resource_registry::ResourceRegistry>
0x1401674eb 4080e601 and sil, 1
0x1401674ef 4088b50a190000 mov byte ptr [rbp + 0x190a], sil
0x1401674f6 889d09190000 mov byte ptr [rbp + 0x1909], bl
0x1401674fc 4883c438 add rsp, 0x38
0x140167500 5b pop rbx
0x140167501 5f pop rdi
0x140167502 5e pop rsi
0x140167503 415c pop r12
0x140167505 415d pop r13
0x140167507 415e pop r14
0x140167509 415f pop r15
0x14016750b 5d pop rbp
0x14016750c c3 ret 
0x14016750d 4080e601 and sil, 1
0x140167511 4088b50a190000 mov byte ptr [rbp + 0x190a], sil
0x140167518 889d09190000 mov byte ptr [rbp + 0x1909], bl
0x14016751e 4883c438 add rsp, 0x38
0x140167522 5b pop rbx
0x140167523 5f pop rdi
0x140167524 5e pop rsi
0x140167525 415c pop r12
0x140167527 415d pop r13
0x140167529 415e pop r14
0x14016752b 415f pop r15
0x14016752d 5d pop rbp
0x14016752e c3 ret 
0x14016752f 90 nop 
0x140167530 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140167535 55 push rbp
0x140167536 4157 push r15
0x140167538 4156 push r14
0x14016753a 4155 push r13
0x14016753c 4154 push r12
0x14016753e 56 push rsi
0x14016753f 57 push rdi
0x140167540 53 push rbx
0x140167541 4883ec38 sub rsp, 0x38
0x140167545 488daa80000000 lea rbp, [rdx + 0x80]
0x14016754c 0fb69d0a190000 movzx ebx, byte ptr [rbp + 0x190a]
0x140167553 f6850919000001 test byte ptr [rbp + 0x1909], 1
0x14016755a 7426 je 0x140167582
0x14016755c 488d8dd0180000 lea rcx, [rbp + 0x18d0]
0x140167563 e8b8324700 call 0x1405da820 ; core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources>,alloc::alloc::Global> >
0x140167568 80e301 and bl, 1
0x14016756b 889d02190000 mov byte ptr [rbp + 0x1902], bl
0x140167571 4883c438 add rsp, 0x38
0x140167575 5b pop rbx
0x140167576 5f pop rdi
0x140167577 5e pop rsi
0x140167578 415c pop r12
0x14016757a 415d pop r13
0x14016757c 415e pop r14
0x14016757e 415f pop r15
0x140167580 5d pop rbp
0x140167581 c3 ret 
0x140167582 80e301 and bl, 1
0x140167585 889d02190000 mov byte ptr [rbp + 0x1902], bl
0x14016758b 4883c438 add rsp, 0x38
0x14016758f 5b pop rbx
0x140167590 5f pop rdi
0x140167591 5e pop rsi
0x140167592 415c pop r12
0x140167594 415d pop r13
0x140167596 415e pop r14
0x140167598 415f pop r15
0x14016759a 5d pop rbp
0x14016759b c3 ret 
0x14016759c 0f1f4000 nop dword ptr [rax]
0x1401675a0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1401675a5 55 push rbp
0x1401675a6 4157 push r15
0x1401675a8 4156 push r14
0x1401675aa 4155 push r13
0x1401675ac 4154 push r12
0x1401675ae 56 push rsi
0x1401675af 57 push rdi
0x1401675b0 53 push rbx
0x1401675b1 4883ec38 sub rsp, 0x38
0x1401675b5 488daa80000000 lea rbp, [rdx + 0x80]
0x1401675bc 80bd0219000000 cmp byte ptr [rbp + 0x1902], 0
0x1401675c3 7424 je 0x1401675e9
0x1401675c5 488d8dc0150000 lea rcx, [rbp + 0x15c0]
0x1401675cc e83f7c4800 call 0x1405ef210 ; core::ptr::drop_in_place<rhapsody::rg::graph::RenderGraph>
0x1401675d1 c6850319000000 mov byte ptr [rbp + 0x1903], 0
0x1401675d8 4883c438 add rsp, 0x38
0x1401675dc 5b pop rbx
0x1401675dd 5f pop rdi
0x1401675de 5e pop rsi
0x1401675df 415c pop r12
0x1401675e1 415d pop r13
0x1401675e3 415e pop r14
0x1401675e5 415f pop r15
0x1401675e7 5d pop rbp
0x1401675e8 c3 ret 
0x1401675e9 c6850319000000 mov byte ptr [rbp + 0x1903], 0
0x1401675f0 4883c438 add rsp, 0x38
0x1401675f4 5b pop rbx
0x1401675f5 5f pop rdi
0x1401675f6 5e pop rsi
0x1401675f7 415c pop r12
0x1401675f9 415d pop r13
0x1401675fb 415e pop r14
0x1401675fd 415f pop r15
0x1401675ff 5d pop rbp
0x140167600 c3 ret 
0x140167601 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140167610 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140167615 55 push rbp
0x140167616 4157 push r15
0x140167618 4156 push r14
0x14016761a 4155 push r13
0x14016761c 4154 push r12
0x14016761e 56 push rsi
0x14016761f 57 push rdi
0x140167620 53 push rbx
0x140167621 4883ec38 sub rsp, 0x38
0x140167625 488daa80000000 lea rbp, [rdx + 0x80]
0x14016762c 80bd0319000000 cmp byte ptr [rbp + 0x1903], 0
0x140167633 741e je 0x140167653
0x140167635 488d8de0110000 lea rcx, [rbp + 0x11e0]
0x14016763c e8bf384700 call 0x1405daf00 ; core::ptr::drop_in_place<rhapsody::rg::recorder::RenderGraphRecorder<country_core::render::pass_builder::pass_resources::PassResources> >
0x140167641 90 nop 
0x140167642 4883c438 add rsp, 0x38
0x140167646 5b pop rbx
0x140167647 5f pop rdi
0x140167648 5e pop rsi
0x140167649 415c pop r12
0x14016764b 415d pop r13
0x14016764d 415e pop r14
0x14016764f 415f pop r15
0x140167651 5d pop rbp
0x140167652 c3 ret 
0x140167653 4883c438 add rsp, 0x38
0x140167657 5b pop rbx
0x140167658 5f pop rdi
0x140167659 5e pop rsi
0x14016765a 415c pop r12
0x14016765c 415d pop r13
0x14016765e 415e pop r14
0x140167660 415f pop r15
0x140167662 5d pop rbp
0x140167663 c3 ret 
0x140167664 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140167670 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140167675 55 push rbp
0x140167676 4157 push r15
0x140167678 4156 push r14
0x14016767a 4155 push r13
0x14016767c 4154 push r12
0x14016767e 56 push rsi
0x14016767f 57 push rdi
0x140167680 53 push rbx
0x140167681 4883ec38 sub rsp, 0x38
0x140167685 488daa80000000 lea rbp, [rdx + 0x80]
0x14016768c 4883bdc818000000 cmp qword ptr [rbp + 0x18c8], 0
0x140167694 7524 jne 0x1401676ba
0x140167696 488d8d50110000 lea rcx, [rbp + 0x1150]
0x14016769d e86e7b4800 call 0x1405ef210 ; core::ptr::drop_in_place<rhapsody::rg::graph::RenderGraph>
0x1401676a2 c6850419000001 mov byte ptr [rbp + 0x1904], 1
0x1401676a9 4883c438 add rsp, 0x38
0x1401676ad 5b pop rbx
0x1401676ae 5f pop rdi
0x1401676af 5e pop rsi
0x1401676b0 415c pop r12
0x1401676b2 415d pop r13
0x1401676b4 415e pop r14
0x1401676b6 415f pop r15
0x1401676b8 5d pop rbp
0x1401676b9 c3 ret 
0x1401676ba c6850419000000 mov byte ptr [rbp + 0x1904], 0
0x1401676c1 4883c438 add rsp, 0x38
0x1401676c5 5b pop rbx
0x1401676c6 5f pop rdi
0x1401676c7 5e pop rsi
0x1401676c8 415c pop r12
0x1401676ca 415d pop r13
0x1401676cc 415e pop r14
0x1401676ce 415f pop r15
0x1401676d0 5d pop rbp
0x1401676d1 c3 ret 
0x1401676d2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1401676e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1401676e5 55 push rbp
0x1401676e6 4157 push r15
0x1401676e8 4156 push r14
0x1401676ea 4155 push r13
0x1401676ec 4154 push r12
0x1401676ee 56 push rsi
0x1401676ef 57 push rdi
0x1401676f0 53 push rbx
0x1401676f1 4883ec38 sub rsp, 0x38
0x1401676f5 488daa80000000 lea rbp, [rdx + 0x80]
0x1401676fc 80bd0419000000 cmp byte ptr [rbp + 0x1904], 0
0x140167703 7410 je 0x140167715
0x140167705 488b8d80190000 mov rcx, qword ptr [rbp + 0x1980]
0x14016770c 4883c128 add rcx, 0x28
0x140167710 e8cb6e4800 call 0x1405ee5e0 ; core::ptr::drop_in_place<rhapsody::raw::texture::Texture>
0x140167715 90 nop 
0x140167716 4883c438 add rsp, 0x38
0x14016771a 5b pop rbx
0x14016771b 5f pop rdi
0x14016771c 5e pop rsi
0x14016771d 415c pop r12
0x14016771f 415d pop r13
0x140167721 415e pop r14
0x140167723 415f pop r15
0x140167725 5d pop rbp
0x140167726 c3 ret 
0x140167727 660f1f840000000000 nop word ptr [rax + rax]
0x140167730 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140167735 55 push rbp
0x140167736 4157 push r15
0x140167738 4156 push r14
0x14016773a 4155 push r13
0x14016773c 4154 push r12
0x14016773e 56 push rsi
0x14016773f 57 push rdi
0x140167740 53 push rbx
0x140167741 4883ec38 sub rsp, 0x38
0x140167745 488daa80000000 lea rbp, [rdx + 0x80]
0x14016774c 488b8df8180000 mov rcx, qword ptr [rbp + 0x18f8]
0x140167753 e8d8a4ffff call 0x140161c30 ; core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> > >
0x140167758 c6850f19000001 mov byte ptr [rbp + 0x190f], 1
0x14016775f c6850e19000001 mov byte ptr [rbp + 0x190e], 1
0x140167766 4883c438 add rsp, 0x38
0x14016776a 5b pop rbx
0x14016776b 5f pop rdi
0x14016776c 5e pop rsi
0x14016776d 415c pop r12
0x14016776f 415d pop r13
0x140167771 415e pop r14
0x140167773 415f pop r15
0x140167775 5d pop rbp
0x140167776 c3 ret 
0x140167777 660f1f840000000000 nop word ptr [rax + rax]
0x140167780 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140167785 55 push rbp
0x140167786 4157 push r15
0x140167788 4156 push r14
0x14016778a 4155 push r13
0x14016778c 4154 push r12
0x14016778e 56 push rsi
0x14016778f 57 push rdi
0x140167790 53 push rbx
0x140167791 4883ec38 sub rsp, 0x38
0x140167795 488daa80000000 lea rbp, [rdx + 0x80]
0x14016779c 488d8de0110000 lea rcx, [rbp + 0x11e0]
0x1401677a3 e818564700 call 0x1405dcdc0 ; core::ptr::drop_in_place<rhapsody::rg::temporal::TemporalRenderGraphRecorder<country_core::render::pass_builder::pass_resources::PassResources> >
0x1401677a8 c6850b19000000 mov byte ptr [rbp + 0x190b], 0
0x1401677af 4883c438 add rsp, 0x38
0x1401677b3 5b pop rbx
0x1401677b4 5f pop rdi
0x1401677b5 5e pop rsi
0x1401677b6 415c pop r12
0x1401677b8 415d pop r13
0x1401677ba 415e pop r14
0x1401677bc 415f pop r15
0x1401677be 5d pop rbp
0x1401677bf c3 ret 
0x1401677c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1401677c5 55 push rbp
0x1401677c6 4157 push r15
0x1401677c8 4156 push r14
0x1401677ca 4155 push r13
0x1401677cc 4154 push r12
0x1401677ce 56 push rsi
0x1401677cf 57 push rdi
0x1401677d0 53 push rbx
0x1401677d1 4883ec38 sub rsp, 0x38
0x1401677d5 488daa80000000 lea rbp, [rdx + 0x80]
0x1401677dc 488d8df0000000 lea rcx, [rbp + 0xf0]
0x1401677e3 488d9510010000 lea rdx, [rbp + 0x110]
0x1401677ea 41b810000000 mov r8d, 0x10
0x1401677f0 41b910000000 mov r9d, 0x10
0x1401677f6 e8054ceeff call 0x14004c400 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<core::ptr::non_null::NonNull<hashlink::linked_hash_map::Node<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> > >,tuple$<> >,allocator_api2::stable::alloc::global::Global>
0x1401677fb c6850319000001 mov byte ptr [rbp + 0x1903], 1
0x140167802 4883c438 add rsp, 0x38
0x140167806 5b pop rbx
0x140167807 5f pop rdi
0x140167808 5e pop rsi
0x140167809 415c pop r12
0x14016780b 415d pop r13
0x14016780d 415e pop r14
0x14016780f 415f pop r15
0x140167811 5d pop rbp
0x140167812 c3 ret 