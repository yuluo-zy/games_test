140a68a50 55 push rbp
140a68a51 4156 push r14
140a68a53 56 push rsi
140a68a54 57 push rdi
140a68a55 53 push rbx
140a68a56 4881ec00050000 sub rsp, 0x500
140a68a5d 488dac2480000000 lea rbp, [rsp + 0x80]
140a68a65 48c78578040000feffffff mov qword ptr [rbp + 0x478], 0xfffffffffffffffe
140a68a70 4889d6 mov rsi, rdx
140a68a73 4889cf mov rdi, rcx
140a68a76 48894c2430 mov qword ptr [rsp + 0x30], rcx
140a68a7b 4889542428 mov qword ptr [rsp + 0x28], rdx
140a68a80 48c744242024000000 mov qword ptr [rsp + 0x20], 0x24
140a68a89 488d159fc60602 lea rdx, [rip + 0x206c69f]
140a68a90 4c8d0dafc60602 lea r9, [rip + 0x206c6af]
140a68a97 488d9de0020000 lea rbx, [rbp + 0x2e0]
140a68a9e 41b817000000 mov r8d, 0x17
140a68aa4 4889d9 mov rcx, rbx
140a68aa7 e8f4a7b900 call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a68aac 488d8d50010000 lea rcx, [rbp + 0x150]
140a68ab3 49b8ca2c81ce403b13d2 movabs r8, 0xd2133b40ce812cca
140a68abd 4889da mov rdx, rbx
140a68ac0 e8abc6b900 call 0x141605170 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo>
140a68ac5 80bd8602000008 cmp byte ptr [rbp + 0x286], 8
140a68acc 742c je 0x140a68afa
140a68ace 488d8d60020000 lea rcx, [rbp + 0x260]
140a68ad5 bae8000000 mov edx, 0xe8
140a68ada 48039588020000 add rdx, qword ptr [rbp + 0x288]
140a68ae1 49b871fc471af99620f3 movabs r8, 0xf32096f91a47fc71
140a68aeb 49b95000000050000000 movabs r9, 0x5000000050
140a68af5 e8662cf0ff call 0x14096b760 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<array$<u32,2> >,array$<u32,2> >
140a68afa 488d8de0020000 lea rcx, [rbp + 0x2e0]
140a68b01 488d9550010000 lea rdx, [rbp + 0x150]
140a68b08 41b890010000 mov r8d, 0x190
140a68b0e e8edb0eb01 call 0x142923c00 ; 
140a68b13 80bd1604000008 cmp byte ptr [rbp + 0x416], 8
140a68b1a 7428 je 0x140a68b44
140a68b1c 488d8df0030000 lea rcx, [rbp + 0x3f0]
140a68b23 bae8000000 mov edx, 0xe8
140a68b28 48039518040000 add rdx, qword ptr [rbp + 0x418]
140a68b2f 49b892d66d4f6f90870c movabs r8, 0xc87906f4f6dd692
140a68b39 41b9409c0000 mov r9d, 0x9c40
140a68b3f e82c2af0ff call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
140a68b44 488d8d50010000 lea rcx, [rbp + 0x150]
140a68b4b 41b890010000 mov r8d, 0x190
140a68b51 4889da mov rdx, rbx
140a68b54 e8a7b0eb01 call 0x142923c00 ; 
140a68b59 80bd8602000008 cmp byte ptr [rbp + 0x286], 8
140a68b60 7428 je 0x140a68b8a
140a68b62 488d8d60020000 lea rcx, [rbp + 0x260]
140a68b69 bae8000000 mov edx, 0xe8
140a68b6e 48039588020000 add rdx, qword ptr [rbp + 0x288]
140a68b75 49b8534548857c334b3d movabs r8, 0x3d4b337c85484553
140a68b7f 41b902000000 mov r9d, 2
140a68b85 e8e629f0ff call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
140a68b8a 4c8d75c0 lea r14, [rbp - 0x40]
140a68b8e 488d9550010000 lea rdx, [rbp + 0x150]
140a68b95 41b890010000 mov r8d, 0x190
140a68b9b 4c89f1 mov rcx, r14
140a68b9e e85db0eb01 call 0x142923c00 ; 
140a68ba3 c6855001000009 mov byte ptr [rbp + 0x150], 9
140a68baa c785e002000002000000 mov dword ptr [rbp + 0x2e0], 2
140a68bb4 48895c2420 mov qword ptr [rsp + 0x20], rbx
140a68bb9 488d9d50010000 lea rbx, [rbp + 0x150]
140a68bc0 49b8f2c570045e8bb061 movabs r8, 0x61b08b5e0470c5f2
140a68bca 4889d9 mov rcx, rbx
140a68bcd 4c89f2 mov rdx, r14
140a68bd0 41b119 mov r9b, 0x19
140a68bd3 e888c6e9ff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a68bd8 4c8db5e0020000 lea r14, [rbp + 0x2e0]
140a68bdf 488d55c0 lea rdx, [rbp - 0x40]
140a68be3 41b890010000 mov r8d, 0x190
140a68be9 4c89f1 mov rcx, r14
140a68bec e80fb0eb01 call 0x142923c00 ; 
140a68bf1 c685770400000a mov byte ptr [rbp + 0x477], 0xa
140a68bf8 c7855001000002000000 mov dword ptr [rbp + 0x150], 2
140a68c02 48895c2420 mov qword ptr [rsp + 0x20], rbx
140a68c07 488d8d77040000 lea rcx, [rbp + 0x477]
140a68c0e 49b857f6e8da4c1e7d29 movabs r8, 0x297d1e4cdae8f657
140a68c18 4c89f2 mov rdx, r14
140a68c1b 41b119 mov r9b, 0x19
140a68c1e e83dc6e9ff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a68c23 41b890010000 mov r8d, 0x190
140a68c29 4889d9 mov rcx, rbx
140a68c2c 4c89f2 mov rdx, r14
140a68c2f e8ccafeb01 call 0x142923c00 ; 
140a68c34 c645c00e mov byte ptr [rbp - 0x40], 0xe
140a68c38 c785e002000002000000 mov dword ptr [rbp + 0x2e0], 2
140a68c42 4c89742420 mov qword ptr [rsp + 0x20], r14
140a68c47 488d4dc0 lea rcx, [rbp - 0x40]
140a68c4b 49b8b9d21286b398e04e movabs r8, 0x4ee098b38612d2b9
140a68c55 4889da mov rdx, rbx
140a68c58 41b119 mov r9b, 0x19
140a68c5b e800c6e9ff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a68c60 488d9de0020000 lea rbx, [rbp + 0x2e0]
140a68c67 4c8db550010000 lea r14, [rbp + 0x150]
140a68c6e 41b890010000 mov r8d, 0x190
140a68c74 4889d9 mov rcx, rbx
140a68c77 4c89f2 mov rdx, r14
140a68c7a e881afeb01 call 0x142923c00 ; 
140a68c7f c645c005 mov byte ptr [rbp - 0x40], 5
140a68c83 c7855001000002000000 mov dword ptr [rbp + 0x150], 2
140a68c8d 4c89742420 mov qword ptr [rsp + 0x20], r14
140a68c92 488d4dc0 lea rcx, [rbp - 0x40]
140a68c96 49b81f02c46dff2f50ed movabs r8, 0xed502fff6dc4021f
140a68ca0 4889da mov rdx, rbx
140a68ca3 41b119 mov r9b, 0x19
140a68ca6 e8b5c5e9ff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a68cab 488d9d50010000 lea rbx, [rbp + 0x150]
140a68cb2 4c8db5e0020000 lea r14, [rbp + 0x2e0]
140a68cb9 41b890010000 mov r8d, 0x190
140a68cbf 4889d9 mov rcx, rbx
140a68cc2 4c89f2 mov rdx, r14
140a68cc5 e836afeb01 call 0x142923c00 ; 
140a68cca c6857704000013 mov byte ptr [rbp + 0x477], 0x13
140a68cd1 c785e002000002000000 mov dword ptr [rbp + 0x2e0], 2
140a68cdb 4c89742420 mov qword ptr [rsp + 0x20], r14
140a68ce0 488d8d77040000 lea rcx, [rbp + 0x477]
140a68ce7 49b841f0c818cd3cd66a movabs r8, 0x6ad63ccd18c8f041
140a68cf1 4889da mov rdx, rbx
140a68cf4 41b119 mov r9b, 0x19
140a68cf7 e864c5e9ff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a68cfc 488d5dc0 lea rbx, [rbp - 0x40]
140a68d00 488d9550010000 lea rdx, [rbp + 0x150]
140a68d07 41b890010000 mov r8d, 0x190
140a68d0d 4889d9 mov rcx, rbx
140a68d10 e8ebaeeb01 call 0x142923c00 ; 
140a68d15 488d8de0020000 lea rcx, [rbp + 0x2e0]
140a68d1c 49b85a7612802b7ac0f1 movabs r8, 0xf1c07a2b8012765a
140a68d26 4889da mov rdx, rbx
140a68d29 41b901000000 mov r9d, 1
140a68d2f e8dcfeb900 call 0x141608c10 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>
140a68d34 80bd1604000008 cmp byte ptr [rbp + 0x416], 8
140a68d3b 7428 je 0x140a68d65
140a68d3d 488d8df0030000 lea rcx, [rbp + 0x3f0]
140a68d44 bae8000000 mov edx, 0xe8
140a68d49 48039518040000 add rdx, qword ptr [rbp + 0x418]
140a68d50 49b88957b716a611950e movabs r8, 0xe9511a616b75789
140a68d5a 41b901000000 mov r9d, 1
140a68d60 e80b28f0ff call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
140a68d65 488d8d50010000 lea rcx, [rbp + 0x150]
140a68d6c 488d95e0020000 lea rdx, [rbp + 0x2e0]
140a68d73 41b890010000 mov r8d, 0x190
140a68d79 e882aeeb01 call 0x142923c00 ; 
140a68d7e 0f1085f0010000 movups xmm0, xmmword ptr [rbp + 0x1f0]
140a68d85 0f294500 movaps xmmword ptr [rbp], xmm0
140a68d89 0f1085e0010000 movups xmm0, xmmword ptr [rbp + 0x1e0]
140a68d90 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
140a68d94 0f1085d0010000 movups xmm0, xmmword ptr [rbp + 0x1d0]
140a68d9b 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
140a68d9f 0f1085c0010000 movups xmm0, xmmword ptr [rbp + 0x1c0]
140a68da6 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
140a68daa 0f1085b0010000 movups xmm0, xmmword ptr [rbp + 0x1b0]
140a68db1 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
140a68db5 488b8588020000 mov rax, qword ptr [rbp + 0x288]
140a68dbc 488b8dd0020000 mov rcx, qword ptr [rbp + 0x2d0]
140a68dc3 488b9560010000 mov rdx, qword ptr [rbp + 0x160]
140a68dca 488995f0020000 mov qword ptr [rbp + 0x2f0], rdx
140a68dd1 0f108550010000 movups xmm0, xmmword ptr [rbp + 0x150]
140a68dd8 0f2985e0020000 movaps xmmword ptr [rbp + 0x2e0], xmm0
140a68ddf 0f108568010000 movups xmm0, xmmword ptr [rbp + 0x168]
140a68de6 0f298510030000 movaps xmmword ptr [rbp + 0x310], xmm0
140a68ded 488b9578010000 mov rdx, qword ptr [rbp + 0x178]
140a68df4 48899520030000 mov qword ptr [rbp + 0x320], rdx
140a68dfb 0f108580010000 movups xmm0, xmmword ptr [rbp + 0x180]
140a68e02 0f1185f8020000 movups xmmword ptr [rbp + 0x2f8], xmm0
140a68e09 488b9590010000 mov rdx, qword ptr [rbp + 0x190]
140a68e10 48899508030000 mov qword ptr [rbp + 0x308], rdx
140a68e17 0f108500020000 movups xmm0, xmmword ptr [rbp + 0x200]
140a68e1e 0f108d10020000 movups xmm1, xmmword ptr [rbp + 0x210]
140a68e25 0f109520020000 movups xmm2, xmmword ptr [rbp + 0x220]
140a68e2c 0f109d30020000 movups xmm3, xmmword ptr [rbp + 0x230]
140a68e33 0f118558030000 movups xmmword ptr [rbp + 0x358], xmm0
140a68e3a 0f118d68030000 movups xmmword ptr [rbp + 0x368], xmm1
140a68e41 0f119578030000 movups xmmword ptr [rbp + 0x378], xmm2
140a68e48 0f119d88030000 movups xmmword ptr [rbp + 0x388], xmm3
140a68e4f 0f108540020000 movups xmm0, xmmword ptr [rbp + 0x240]
140a68e56 0f118598030000 movups xmmword ptr [rbp + 0x398], xmm0
140a68e5d 0f108550020000 movups xmm0, xmmword ptr [rbp + 0x250]
140a68e64 0f1185a8030000 movups xmmword ptr [rbp + 0x3a8], xmm0
140a68e6b 488b95a8010000 mov rdx, qword ptr [rbp + 0x1a8]
140a68e72 48899538030000 mov qword ptr [rbp + 0x338], rdx
140a68e79 0f108598010000 movups xmm0, xmmword ptr [rbp + 0x198]
140a68e80 0f118528030000 movups xmmword ptr [rbp + 0x328], xmm0
140a68e87 48898dc0030000 mov qword ptr [rbp + 0x3c0], rcx
140a68e8e 488985b8030000 mov qword ptr [rbp + 0x3b8], rax
140a68e95 0f2805a4ba0602 movaps xmm0, xmmword ptr [rip + 0x206baa4]
140a68e9c 0f298540030000 movaps xmmword ptr [rbp + 0x340], xmm0
140a68ea3 488d4dc0 lea rcx, [rbp - 0x40]
140a68ea7 488d95e0020000 lea rdx, [rbp + 0x2e0]
140a68eae e81d46f7ff call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a68eb3 80bd8602000008 cmp byte ptr [rbp + 0x286], 8
140a68eba 740c je 0x140a68ec8
140a68ebc 488d8d70020000 lea rcx, [rbp + 0x270]
140a68ec3 e8e8951700 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a68ec8 48897c2430 mov qword ptr [rsp + 0x30], rdi
140a68ecd 4889742428 mov qword ptr [rsp + 0x28], rsi
140a68ed2 48c74424202c000000 mov qword ptr [rsp + 0x20], 0x2c
140a68edb 488d1588c20602 lea rdx, [rip + 0x206c288]
140a68ee2 4c8d0d8fc20602 lea r9, [rip + 0x206c28f]
140a68ee9 488db550010000 lea rsi, [rbp + 0x150]
140a68ef0 41b80e000000 mov r8d, 0xe
140a68ef6 4889f1 mov rcx, rsi
140a68ef9 e8a2a3b900 call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a68efe 488d8de0020000 lea rcx, [rbp + 0x2e0]
140a68f05 49b85a7612802b7ac0f1 movabs r8, 0xf1c07a2b8012765a
140a68f0f 4889f2 mov rdx, rsi
140a68f12 e89945ba00 call 0x14160d4b0 ; country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>
140a68f17 80bd1604000008 cmp byte ptr [rbp + 0x416], 8
140a68f1e 7428 je 0x140a68f48
140a68f20 488d8df0030000 lea rcx, [rbp + 0x3f0]
140a68f27 bae8000000 mov edx, 0xe8
140a68f2c 48039518040000 add rdx, qword ptr [rbp + 0x418]
140a68f33 49b88957b716a611950e movabs r8, 0xe9511a616b75789
140a68f3d 41b901000000 mov r9d, 1
140a68f43 e82826f0ff call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
140a68f48 488d75c0 lea rsi, [rbp - 0x40]
140a68f4c 488d95e0020000 lea rdx, [rbp + 0x2e0]
140a68f53 41b890010000 mov r8d, 0x190
140a68f59 4889f1 mov rcx, rsi
140a68f5c e89faceb01 call 0x142923c00 ; 
140a68f61 488d8de0020000 lea rcx, [rbp + 0x2e0]
140a68f68 49b87407d19b4c73e4a6 movabs r8, 0xa6e4734c9bd10774
140a68f72 4889f2 mov rdx, rsi
140a68f75 e8a6ebb900 call 0x141607b20 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::audio::terrain_stairs_audio::StairsCounterSsbo>
140a68f7a 0f108580030000 movups xmm0, xmmword ptr [rbp + 0x380]
140a68f81 0f294500 movaps xmmword ptr [rbp], xmm0
140a68f85 0f108570030000 movups xmm0, xmmword ptr [rbp + 0x370]
140a68f8c 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
140a68f90 0f108560030000 movups xmm0, xmmword ptr [rbp + 0x360]
140a68f97 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
140a68f9b 0f108550030000 movups xmm0, xmmword ptr [rbp + 0x350]
140a68fa2 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
140a68fa6 0f108540030000 movups xmm0, xmmword ptr [rbp + 0x340]
140a68fad 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
140a68fb1 488b8518040000 mov rax, qword ptr [rbp + 0x418]
140a68fb8 488b8d60040000 mov rcx, qword ptr [rbp + 0x460]
140a68fbf 488b95f0020000 mov rdx, qword ptr [rbp + 0x2f0]
140a68fc6 48899560010000 mov qword ptr [rbp + 0x160], rdx
140a68fcd 0f1085e0020000 movups xmm0, xmmword ptr [rbp + 0x2e0]
140a68fd4 0f298550010000 movaps xmmword ptr [rbp + 0x150], xmm0
140a68fdb 0f1085f8020000 movups xmm0, xmmword ptr [rbp + 0x2f8]
140a68fe2 0f298580010000 movaps xmmword ptr [rbp + 0x180], xmm0
140a68fe9 488b9508030000 mov rdx, qword ptr [rbp + 0x308]
140a68ff0 48899590010000 mov qword ptr [rbp + 0x190], rdx
140a68ff7 0f108510030000 movups xmm0, xmmword ptr [rbp + 0x310]
140a68ffe 0f118568010000 movups xmmword ptr [rbp + 0x168], xmm0
140a69005 488b9520030000 mov rdx, qword ptr [rbp + 0x320]
140a6900c 48899578010000 mov qword ptr [rbp + 0x178], rdx
140a69013 0f108590030000 movups xmm0, xmmword ptr [rbp + 0x390]
140a6901a 0f108da0030000 movups xmm1, xmmword ptr [rbp + 0x3a0]
140a69021 0f1095b0030000 movups xmm2, xmmword ptr [rbp + 0x3b0]
140a69028 0f109dc0030000 movups xmm3, xmmword ptr [rbp + 0x3c0]
140a6902f 0f1185c8010000 movups xmmword ptr [rbp + 0x1c8], xmm0
140a69036 0f118dd8010000 movups xmmword ptr [rbp + 0x1d8], xmm1
140a6903d 0f1195e8010000 movups xmmword ptr [rbp + 0x1e8], xmm2
140a69044 0f119df8010000 movups xmmword ptr [rbp + 0x1f8], xmm3
140a6904b 0f1085d0030000 movups xmm0, xmmword ptr [rbp + 0x3d0]
140a69052 0f118508020000 movups xmmword ptr [rbp + 0x208], xmm0
140a69059 0f1085e0030000 movups xmm0, xmmword ptr [rbp + 0x3e0]
140a69060 0f118518020000 movups xmmword ptr [rbp + 0x218], xmm0
140a69067 488b9538030000 mov rdx, qword ptr [rbp + 0x338]
140a6906e 488995a8010000 mov qword ptr [rbp + 0x1a8], rdx
140a69075 0f108528030000 movups xmm0, xmmword ptr [rbp + 0x328]
140a6907c 0f118598010000 movups xmmword ptr [rbp + 0x198], xmm0
140a69083 48898d30020000 mov qword ptr [rbp + 0x230], rcx
140a6908a 48898528020000 mov qword ptr [rbp + 0x228], rax
140a69091 0f280548ffee01 movaps xmm0, xmmword ptr [rip + 0x1eeff48]
140a69098 0f2985b0010000 movaps xmmword ptr [rbp + 0x1b0], xmm0
140a6909f 488d4dc0 lea rcx, [rbp - 0x40]
140a690a3 488d9550010000 lea rdx, [rbp + 0x150]
140a690aa e82144f7ff call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a690af 80bd1604000008 cmp byte ptr [rbp + 0x416], 8
140a690b6 740c je 0x140a690c4
140a690b8 488d8d00040000 lea rcx, [rbp + 0x400]
140a690bf e8ec931700 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a690c4 90 nop 
140a690c5 4881c400050000 add rsp, 0x500
140a690cc 5b pop rbx
140a690cd 5f pop rdi
140a690ce 5e pop rsi
140a690cf 415e pop r14
140a690d1 5d pop rbp
140a690d2 c3 ret 
140a690d3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a690e0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a690e5 55 push rbp
140a690e6 4156 push r14
140a690e8 56 push rsi
140a690e9 57 push rdi
140a690ea 53 push rbx
140a690eb 4883ec40 sub rsp, 0x40
140a690ef 488daa80000000 lea rbp, [rdx + 0x80]
140a690f6 80bd8602000008 cmp byte ptr [rbp + 0x286], 8
140a690fd 740c je 0x140a6910b
140a690ff 488d8d70020000 lea rcx, [rbp + 0x270]
140a69106 e8a5931700 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a6910b 90 nop 
140a6910c 4883c440 add rsp, 0x40
140a69110 5b pop rbx
140a69111 5f pop rdi
140a69112 5e pop rsi
140a69113 415e pop r14
140a69115 5d pop rbp
140a69116 c3 ret 
140a69117 660f1f840000000000 nop word ptr [rax + rax]
140a69120 4889542410 mov qword ptr [rsp + 0x10], rdx
140a69125 55 push rbp
140a69126 4156 push r14
140a69128 56 push rsi
140a69129 57 push rdi
140a6912a 53 push rbx
140a6912b 4883ec40 sub rsp, 0x40
140a6912f 488daa80000000 lea rbp, [rdx + 0x80]
140a69136 488d8de0020000 lea rcx, [rbp + 0x2e0]
140a6913d e86edee2ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a69142 90 nop 
140a69143 4883c440 add rsp, 0x40
140a69147 5b pop rbx
140a69148 5f pop rdi
140a69149 5e pop rsi
140a6914a 415e pop r14
140a6914c 5d pop rbp
140a6914d c3 ret 
140a6914e 6690 nop 
140a69150 4889542410 mov qword ptr [rsp + 0x10], rdx
140a69155 55 push rbp
140a69156 4156 push r14
140a69158 56 push rsi
140a69159 57 push rdi
140a6915a 53 push rbx
140a6915b 4883ec40 sub rsp, 0x40
140a6915f 488daa80000000 lea rbp, [rdx + 0x80]
140a69166 80bd1604000008 cmp byte ptr [rbp + 0x416], 8
140a6916d 740c je 0x140a6917b
140a6916f 488d8d00040000 lea rcx, [rbp + 0x400]
140a69176 e835931700 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a6917b 90 nop 
140a6917c 4883c440 add rsp, 0x40
140a69180 5b pop rbx
140a69181 5f pop rdi
140a69182 5e pop rsi
140a69183 415e pop r14
140a69185 5d pop rbp
140a69186 c3 ret 
140a69187 660f1f840000000000 nop word ptr [rax + rax]
140a69190 4889542410 mov qword ptr [rsp + 0x10], rdx
140a69195 55 push rbp
140a69196 4156 push r14
140a69198 56 push rsi
140a69199 57 push rdi
140a6919a 53 push rbx
140a6919b 4883ec40 sub rsp, 0x40
140a6919f 488daa80000000 lea rbp, [rdx + 0x80]
140a691a6 488d8d50010000 lea rcx, [rbp + 0x150]
140a691ad e8fedde2ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a691b2 90 nop 
140a691b3 4883c440 add rsp, 0x40
140a691b7 5b pop rbx
140a691b8 5f pop rdi
140a691b9 5e pop rsi
140a691ba 415e pop r14
140a691bc 5d pop rbp
140a691bd c3 ret 
140a691be 6690 nop 
140a691c0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a691c5 55 push rbp
140a691c6 4156 push r14
140a691c8 56 push rsi
140a691c9 57 push rdi
140a691ca 53 push rbx
140a691cb 4883ec40 sub rsp, 0x40
140a691cf 488daa80000000 lea rbp, [rdx + 0x80]
140a691d6 488d8de0020000 lea rcx, [rbp + 0x2e0]
140a691dd e8cedde2ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a691e2 90 nop 
140a691e3 4883c440 add rsp, 0x40
140a691e7 5b pop rbx
140a691e8 5f pop rdi
140a691e9 5e pop rsi
140a691ea 415e pop r14
140a691ec 5d pop rbp
140a691ed c3 ret 
140a691ee 6690 nop 
140a691f0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a691f5 55 push rbp
140a691f6 4156 push r14
140a691f8 56 push rsi
140a691f9 57 push rdi
140a691fa 53 push rbx
140a691fb 4883ec40 sub rsp, 0x40
140a691ff 488daa80000000 lea rbp, [rdx + 0x80]
140a69206 488d8d50010000 lea rcx, [rbp + 0x150]
140a6920d e89edde2ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a69212 90 nop 
140a69213 4883c440 add rsp, 0x40
140a69217 5b pop rbx
140a69218 5f pop rdi
140a69219 5e pop rsi
140a6921a 415e pop r14
140a6921c 5d pop rbp
140a6921d c3 ret 
140a6921e 6690 nop 
140a69220 4889542410 mov qword ptr [rsp + 0x10], rdx
140a69225 55 push rbp
140a69226 4156 push r14
140a69228 56 push rsi
140a69229 57 push rdi
140a6922a 53 push rbx
140a6922b 4883ec40 sub rsp, 0x40
140a6922f 488daa80000000 lea rbp, [rdx + 0x80]
140a69236 488d4dc0 lea rcx, [rbp - 0x40]
140a6923a e871dde2ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a6923f 90 nop 
140a69240 4883c440 add rsp, 0x40
140a69244 5b pop rbx
140a69245 5f pop rdi
140a69246 5e pop rsi
140a69247 415e pop r14
140a69249 5d pop rbp
140a6924a c3 ret 
140a6924b 0f1f440000 nop dword ptr [rax + rax]
140a69250 4889542410 mov qword ptr [rsp + 0x10], rdx
140a69255 55 push rbp
140a69256 4156 push r14
140a69258 56 push rsi
140a69259 57 push rdi
140a6925a 53 push rbx
140a6925b 4883ec40 sub rsp, 0x40
140a6925f 488daa80000000 lea rbp, [rdx + 0x80]
140a69266 488d8de0020000 lea rcx, [rbp + 0x2e0]
140a6926d e83edde2ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a69272 90 nop 
140a69273 4883c440 add rsp, 0x40
140a69277 5b pop rbx
140a69278 5f pop rdi
140a69279 5e pop rsi
140a6927a 415e pop r14
140a6927c 5d pop rbp
140a6927d c3 ret 
140a6927e 6690 nop 
140a69280 4889542410 mov qword ptr [rsp + 0x10], rdx
140a69285 55 push rbp
140a69286 4156 push r14
140a69288 56 push rsi
140a69289 57 push rdi
140a6928a 53 push rbx
140a6928b 4883ec40 sub rsp, 0x40
140a6928f 488daa80000000 lea rbp, [rdx + 0x80]
140a69296 488d8d50010000 lea rcx, [rbp + 0x150]
140a6929d e80edde2ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a692a2 90 nop 
140a692a3 4883c440 add rsp, 0x40
140a692a7 5b pop rbx
140a692a8 5f pop rdi
140a692a9 5e pop rsi
140a692aa 415e pop r14
140a692ac 5d pop rbp
140a692ad c3 ret 
140a692ae 6690 nop 
140a692b0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a692b5 55 push rbp
140a692b6 4156 push r14
140a692b8 56 push rsi
140a692b9 57 push rdi
140a692ba 53 push rbx
140a692bb 4883ec40 sub rsp, 0x40
140a692bf 488daa80000000 lea rbp, [rdx + 0x80]
140a692c6 488d8de0020000 lea rcx, [rbp + 0x2e0]
140a692cd e8dedce2ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a692d2 90 nop 
140a692d3 4883c440 add rsp, 0x40
140a692d7 5b pop rbx
140a692d8 5f pop rdi
140a692d9 5e pop rsi
140a692da 415e pop r14
140a692dc 5d pop rbp
140a692dd c3 ret 
140a692de 6690 nop 
140a692e0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a692e5 55 push rbp
140a692e6 4156 push r14
140a692e8 56 push rsi
140a692e9 57 push rdi
140a692ea 53 push rbx
140a692eb 4883ec40 sub rsp, 0x40
140a692ef 488daa80000000 lea rbp, [rdx + 0x80]
140a692f6 488d8d50010000 lea rcx, [rbp + 0x150]
140a692fd e8aedce2ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a69302 90 nop 
140a69303 4883c440 add rsp, 0x40
140a69307 5b pop rbx
140a69308 5f pop rdi
140a69309 5e pop rsi
140a6930a 415e pop r14
140a6930c 5d pop rbp
140a6930d c3 ret 
140a6930e 6690 nop 
140a69310 4889542410 mov qword ptr [rsp + 0x10], rdx
140a69315 55 push rbp
140a69316 4156 push r14
140a69318 56 push rsi
140a69319 57 push rdi
140a6931a 53 push rbx
140a6931b 4883ec40 sub rsp, 0x40
140a6931f 488daa80000000 lea rbp, [rdx + 0x80]
140a69326 488d8de0020000 lea rcx, [rbp + 0x2e0]
140a6932d e87edce2ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a69332 90 nop 
140a69333 4883c440 add rsp, 0x40
140a69337 5b pop rbx
140a69338 5f pop rdi
140a69339 5e pop rsi
140a6933a 415e pop r14
140a6933c 5d pop rbp
140a6933d c3 ret 
