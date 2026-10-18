140a51440 55 push rbp
140a51441 4157 push r15
140a51443 4156 push r14
140a51445 56 push rsi
140a51446 57 push rdi
140a51447 53 push rbx
140a51448 4881ec28050000 sub rsp, 0x528
140a5144f 488dac2480000000 lea rbp, [rsp + 0x80]
140a51457 48c785a0040000feffffff mov qword ptr [rbp + 0x4a0], 0xfffffffffffffffe
140a51462 4889d6 mov rsi, rdx
140a51465 4889cf mov rdi, rcx
140a51468 4883ba8800000000 cmp qword ptr [rdx + 0x88], 0
140a51470 48bb14edf4fae674fdf5 movabs rbx, 0xf5fd74e6faf4ed14
140a5147a 49beaa4e7896b36185a6 movabs r14, 0xa68561b396784eaa
140a51484 0f849d000000 je 0x140a51527
140a5148a 488b4670 mov rax, qword ptr [rsi + 0x70]
140a5148e 488b4e78 mov rcx, qword ptr [rsi + 0x78]
140a51492 4889ca mov rdx, rcx
140a51495 4821da and rdx, rbx
140a51498 4531c0 xor r8d, r8d
140a5149b 660f6f057df0ef01 movdqa xmm0, xmmword ptr [rip + 0x1eff07d]
140a514a3 660f76c9 pcmpeqd xmm1, xmm1
140a514a7 f30f6f1410 movdqu xmm2, xmmword ptr [rax + rdx]
140a514ac 660f6fda movdqa xmm3, xmm2
140a514b0 660f74d8 pcmpeqb xmm3, xmm0
140a514b4 66440fd7cb pmovmskb r9d, xmm3
140a514b9 4585c9 test r9d, r9d
140a514bc 752f jne 0x140a514ed
140a514be 660f74d1 pcmpeqb xmm2, xmm1
140a514c2 66440fd7ca pmovmskb r9d, xmm2
140a514c7 4585c9 test r9d, r9d
140a514ca 755b jne 0x140a51527
140a514cc 4c01c2 add rdx, r8
140a514cf 4883c210 add rdx, 0x10
140a514d3 4983c010 add r8, 0x10
140a514d7 4821ca and rdx, rcx
140a514da ebcb jmp 0x140a514a7
140a514dc 0f1f4000 nop dword ptr [rax]
140a514e0 458d51ff lea r10d, [r9 - 1]
140a514e4 664521ca and r10w, r9w
140a514e8 4589d1 mov r9d, r10d
140a514eb 74d1 je 0x140a514be
140a514ed f3450fbcd1 tzcnt r10d, r9d
140a514f2 4901d2 add r10, rdx
140a514f5 4921ca and r10, rcx
140a514f8 49f7da neg r10
140a514fb 4f8d1452 lea r10, [r10 + r10*2]
140a514ff 4e3974d0e8 cmp qword ptr [rax + r10*8 - 0x18], r14
140a51504 75da jne 0x140a514e0
140a51506 4a395cd0f0 cmp qword ptr [rax + r10*8 - 0x10], rbx
140a5150b 75d3 jne 0x140a514e0
140a5150d 4a8d04d0 lea rax, [rax + r10*8]
140a51511 488b58f8 mov rbx, qword ptr [rax - 8]
140a51515 48399e18020000 cmp qword ptr [rsi + 0x218], rbx
140a5151c 0f870a010000 ja 0x140a5162c
140a51522 e9c8050000 jmp 0x140a51aef
140a51527 8b8690000000 mov eax, dword ptr [rsi + 0x90]
140a5152d 3dfdffff3f cmp eax, 0x3ffffffd
140a51532 0f877c050000 ja 0x140a51ab4
140a51538 8d4801 lea ecx, [rax + 1]
140a5153b f00fb18e90000000 lock cmpxchg dword ptr [rsi + 0x90], ecx
140a51543 0f856b050000 jne 0x140a51ab4
140a51549 0fb68698000000 movzx eax, byte ptr [rsi + 0x98]
140a51550 4883bef000000000 cmp qword ptr [rsi + 0xf0], 0
140a51558 0f8477050000 je 0x140a51ad5
140a5155e 488b86d8000000 mov rax, qword ptr [rsi + 0xd8]
140a51565 488b8ee0000000 mov rcx, qword ptr [rsi + 0xe0]
140a5156c 4889ca mov rdx, rcx
140a5156f 4821da and rdx, rbx
140a51572 4531c0 xor r8d, r8d
140a51575 660f6f05a3efef01 movdqa xmm0, xmmword ptr [rip + 0x1efefa3]
140a5157d 660f76c9 pcmpeqd xmm1, xmm1
140a51581 f30f6f1410 movdqu xmm2, xmmword ptr [rax + rdx]
140a51586 660f6fda movdqa xmm3, xmm2
140a5158a 660f74d8 pcmpeqb xmm3, xmm0
140a5158e 66440fd7cb pmovmskb r9d, xmm3
140a51593 4585c9 test r9d, r9d
140a51596 7535 jne 0x140a515cd
140a51598 660f74d1 pcmpeqb xmm2, xmm1
140a5159c 66440fd7ca pmovmskb r9d, xmm2
140a515a1 4585c9 test r9d, r9d
140a515a4 0f852b050000 jne 0x140a51ad5
140a515aa 4c01c2 add rdx, r8
140a515ad 4883c210 add rdx, 0x10
140a515b1 4983c010 add r8, 0x10
140a515b5 4821ca and rdx, rcx
140a515b8 ebc7 jmp 0x140a51581
140a515ba 660f1f440000 nop word ptr [rax + rax]
140a515c0 458d51ff lea r10d, [r9 - 1]
140a515c4 664521ca and r10w, r9w
140a515c8 4589d1 mov r9d, r10d
140a515cb 74cb je 0x140a51598
140a515cd f3450fbcd9 tzcnt r11d, r9d
140a515d2 4901d3 add r11, rdx
140a515d5 4921cb and r11, rcx
140a515d8 4d89da mov r10, r11
140a515db 49f7da neg r10
140a515de 49c1e303 shl r11, 3
140a515e2 49c1e207 shl r10, 7
140a515e6 4d29da sub r10, r11
140a515e9 4e39b41078ffffff cmp qword ptr [rax + r10 - 0x88], r14
140a515f1 75cd jne 0x140a515c0
140a515f3 4a395c1080 cmp qword ptr [rax + r10 - 0x80], rbx
140a515f8 75c6 jne 0x140a515c0
140a515fa 4c01d0 add rax, r10
140a515fd 488b58f8 mov rbx, qword ptr [rax - 8]
140a51601 baffffffff mov edx, 0xffffffff
140a51606 f00fc19690000000 lock xadd dword ptr [rsi + 0x90], edx
140a5160e ffca dec edx
140a51610 89d0 mov eax, edx
140a51612 25ffffffbf and eax, 0xbfffffff
140a51617 f7d8 neg eax
140a51619 0f80dc040000 jo 0x140a51afb
140a5161f 48399e18020000 cmp qword ptr [rsi + 0x218], rbx
140a51626 0f86c3040000 jbe 0x140a51aef
140a5162c 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140a51633 488b0cd8 mov rcx, qword ptr [rax + rbx*8]
140a51637 4885c9 test rcx, rcx
140a5163a 0f84af040000 je 0x140a51aef
140a51640 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
140a51647 48f7d1 not rcx
140a5164a 488d0c49 lea rcx, [rcx + rcx*2]
140a5164e 48c1e105 shl rcx, 5
140a51652 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
140a51658 0f8491040000 je 0x140a51aef
140a5165e 4801c8 add rax, rcx
140a51661 488b5010 mov rdx, qword ptr [rax + 0x10]
140a51665 488d8de0020000 lea rcx, [rbp + 0x2e0]
140a5166c 41b008 mov r8b, 8
140a5166f e8bcb8eeff call 0x14093cf30 ; country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_texture
140a51674 0f1085e4020000 movups xmm0, xmmword ptr [rbp + 0x2e4]
140a5167b 0f108df4020000 movups xmm1, xmmword ptr [rbp + 0x2f4]
140a51682 0f298d80040000 movaps xmmword ptr [rbp + 0x480], xmm1
140a51689 0f298570040000 movaps xmmword ptr [rbp + 0x470], xmm0
140a51690 48897c2430 mov qword ptr [rsp + 0x30], rdi
140a51695 4889742428 mov qword ptr [rsp + 0x28], rsi
140a5169a 48c744242021000000 mov qword ptr [rsp + 0x20], 0x21
140a516a3 488d1516ed0702 lea rdx, [rip + 0x207ed16]
140a516aa 4c8d0d31ed0702 lea r9, [rip + 0x207ed31]
140a516b1 488d9de0020000 lea rbx, [rbp + 0x2e0]
140a516b8 41b822000000 mov r8d, 0x22
140a516be 4889d9 mov rcx, rbx
140a516c1 e8da1bbb00 call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a516c6 c6855001000008 mov byte ptr [rbp + 0x150], 8
140a516cd c745c002000000 mov dword ptr [rbp - 0x40], 2
140a516d4 488d45c0 lea rax, [rbp - 0x40]
140a516d8 4889442420 mov qword ptr [rsp + 0x20], rax
140a516dd 488d8d50010000 lea rcx, [rbp + 0x150]
140a516e4 49b8c88b4102027981b9 movabs r8, 0xb981790202418bc8
140a516ee 4889da mov rdx, rbx
140a516f1 41b132 mov r9b, 0x32
140a516f4 e8673bebff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a516f9 488d4dc0 lea rcx, [rbp - 0x40]
140a516fd 41b890010000 mov r8d, 0x190
140a51703 4889da mov rdx, rbx
140a51706 e8f524ed01 call 0x142923c00 ; 
140a5170b 488d4e38 lea rcx, [rsi + 0x38]
140a5170f c6859f04000001 mov byte ptr [rbp + 0x49f], 1
140a51716 48baffa3086aee51a4bb movabs rdx, 0xbba451ee6a08a3ff
140a51720 49b89df2bcfe158d008d movabs r8, 0x8d008d15febcf29d
140a5172a e8c1bf68ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
140a5172f 4885c0 test rax, rax
140a51732 0f84f9030000 je 0x140a51b31
140a51738 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140a5173f 0f86ec030000 jbe 0x140a51b31
140a51745 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140a5174c 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
140a51750 4885c9 test rcx, rcx
140a51753 0f84d8030000 je 0x140a51b31
140a51759 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
140a51760 48f7d1 not rcx
140a51763 488d0c49 lea rcx, [rcx + rcx*2]
140a51767 48c1e105 shl rcx, 5
140a5176b 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
140a51771 0f84ba030000 je 0x140a51b31
140a51777 4801c8 add rax, rcx
140a5177a 488b5010 mov rdx, qword ptr [rax + 0x10]
140a5177e c6859f04000001 mov byte ptr [rbp + 0x49f], 1
140a51785 488d8d90040000 lea rcx, [rbp + 0x490]
140a5178c e8af552400 call 0x140c96d40 ; utils::camera::CameraMatrices::eye_position
140a51791 80bdf600000008 cmp byte ptr [rbp + 0xf6], 8
140a51798 7443 je 0x140a517dd
140a5179a 488d8dd0000000 lea rcx, [rbp + 0xd0]
140a517a1 bae8000000 mov edx, 0xe8
140a517a6 480395f8000000 add rdx, qword ptr [rbp + 0xf8]
140a517ad 8b8598040000 mov eax, dword ptr [rbp + 0x498]
140a517b3 8985e8020000 mov dword ptr [rbp + 0x2e8], eax
140a517b9 488b8590040000 mov rax, qword ptr [rbp + 0x490]
140a517c0 488985e0020000 mov qword ptr [rbp + 0x2e0], rax
140a517c7 49b890393d870bb3bd4b movabs r8, 0x4bbdb30b873d3990
140a517d1 4c8d8de0020000 lea r9, [rbp + 0x2e0]
140a517d8 e8839ef1ff call 0x14096b660 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<glam::f32::vec3::Vec3>,glam::f32::vec3::Vec3>
140a517dd 4c8dbd50010000 lea r15, [rbp + 0x150]
140a517e4 488d55c0 lea rdx, [rbp - 0x40]
140a517e8 41b890010000 mov r8d, 0x190
140a517ee 4c89f9 mov rcx, r15
140a517f1 e80a24ed01 call 0x142923c00 ; 
140a517f6 c645c009 mov byte ptr [rbp - 0x40], 9
140a517fa c785e002000002000000 mov dword ptr [rbp + 0x2e0], 2
140a51804 48895c2420 mov qword ptr [rsp + 0x20], rbx
140a51809 4c8d75c0 lea r14, [rbp - 0x40]
140a5180d 49b8f2c570045e8bb061 movabs r8, 0x61b08b5e0470c5f2
140a51817 4c89f1 mov rcx, r14
140a5181a 4c89fa mov rdx, r15
140a5181d 41b119 mov r9b, 0x19
140a51820 e83b3aebff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a51825 4c8dbde0020000 lea r15, [rbp + 0x2e0]
140a5182c 488d9d50010000 lea rbx, [rbp + 0x150]
140a51833 41b890010000 mov r8d, 0x190
140a51839 4c89f9 mov rcx, r15
140a5183c 4889da mov rdx, rbx
140a5183f e8bc23ed01 call 0x142923c00 ; 
140a51844 c685900400000a mov byte ptr [rbp + 0x490], 0xa
140a5184b c745c002000000 mov dword ptr [rbp - 0x40], 2
140a51852 4c89742420 mov qword ptr [rsp + 0x20], r14
140a51857 488d8d90040000 lea rcx, [rbp + 0x490]
140a5185e 49b857f6e8da4c1e7d29 movabs r8, 0x297d1e4cdae8f657
140a51868 4c89fa mov rdx, r15
140a5186b 41b119 mov r9b, 0x19
140a5186e e8ed39ebff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a51873 488d4dc0 lea rcx, [rbp - 0x40]
140a51877 488d95e0020000 lea rdx, [rbp + 0x2e0]
140a5187e 41b890010000 mov r8d, 0x190
140a51884 e87723ed01 call 0x142923c00 ; 
140a51889 80bdf600000008 cmp byte ptr [rbp + 0xf6], 8
140a51890 7425 je 0x140a518b7
140a51892 488d8dd0000000 lea rcx, [rbp + 0xd0]
140a51899 bae8000000 mov edx, 0xe8
140a5189e 480395f8000000 add rdx, qword ptr [rbp + 0xf8]
140a518a5 49b84ca5591f44988a41 movabs r8, 0x418a98441f59a54c
140a518af 41b101 mov r9b, 1
140a518b2 e8999ff1ff call 0x14096b850 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<bool>,bool>
140a518b7 4c8db5e0020000 lea r14, [rbp + 0x2e0]
140a518be 488d55c0 lea rdx, [rbp - 0x40]
140a518c2 41b890010000 mov r8d, 0x190
140a518c8 4c89f1 mov rcx, r14
140a518cb e83023ed01 call 0x142923c00 ; 
140a518d0 c6859004000011 mov byte ptr [rbp + 0x490], 0x11
140a518d7 c7855001000002000000 mov dword ptr [rbp + 0x150], 2
140a518e1 48895c2420 mov qword ptr [rsp + 0x20], rbx
140a518e6 488d8d90040000 lea rcx, [rbp + 0x490]
140a518ed 49b8a41a84c94d1f6511 movabs r8, 0x11651f4dc9841aa4
140a518f7 4c89f2 mov rdx, r14
140a518fa 41b119 mov r9b, 0x19
140a518fd e85e39ebff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a51902 488d8d50010000 lea rcx, [rbp + 0x150]
140a51909 488d95e0020000 lea rdx, [rbp + 0x2e0]
140a51910 41b890010000 mov r8d, 0x190
140a51916 e8e522ed01 call 0x142923c00 ; 
140a5191b 0f1085f0010000 movups xmm0, xmmword ptr [rbp + 0x1f0]
140a51922 0f294500 movaps xmmword ptr [rbp], xmm0
140a51926 0f1085e0010000 movups xmm0, xmmword ptr [rbp + 0x1e0]
140a5192d 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
140a51931 0f1085d0010000 movups xmm0, xmmword ptr [rbp + 0x1d0]
140a51938 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
140a5193c 0f1085c0010000 movups xmm0, xmmword ptr [rbp + 0x1c0]
140a51943 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
140a51947 0f1085b0010000 movups xmm0, xmmword ptr [rbp + 0x1b0]
140a5194e 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
140a51952 488b8588020000 mov rax, qword ptr [rbp + 0x288]
140a51959 488b8dd0020000 mov rcx, qword ptr [rbp + 0x2d0]
140a51960 488b9560010000 mov rdx, qword ptr [rbp + 0x160]
140a51967 488995f0020000 mov qword ptr [rbp + 0x2f0], rdx
140a5196e 0f108550010000 movups xmm0, xmmword ptr [rbp + 0x150]
140a51975 0f2985e0020000 movaps xmmword ptr [rbp + 0x2e0], xmm0
140a5197c 0f108568010000 movups xmm0, xmmword ptr [rbp + 0x168]
140a51983 0f298510030000 movaps xmmword ptr [rbp + 0x310], xmm0
140a5198a 488b9578010000 mov rdx, qword ptr [rbp + 0x178]
140a51991 48899520030000 mov qword ptr [rbp + 0x320], rdx
140a51998 0f108580010000 movups xmm0, xmmword ptr [rbp + 0x180]
140a5199f 0f1185f8020000 movups xmmword ptr [rbp + 0x2f8], xmm0
140a519a6 488b9590010000 mov rdx, qword ptr [rbp + 0x190]
140a519ad 48899508030000 mov qword ptr [rbp + 0x308], rdx
140a519b4 0f108500020000 movups xmm0, xmmword ptr [rbp + 0x200]
140a519bb 0f108d10020000 movups xmm1, xmmword ptr [rbp + 0x210]
140a519c2 0f109520020000 movups xmm2, xmmword ptr [rbp + 0x220]
140a519c9 0f109d30020000 movups xmm3, xmmword ptr [rbp + 0x230]
140a519d0 0f118558030000 movups xmmword ptr [rbp + 0x358], xmm0
140a519d7 0f118d68030000 movups xmmword ptr [rbp + 0x368], xmm1
140a519de 0f119578030000 movups xmmword ptr [rbp + 0x378], xmm2
140a519e5 0f119d88030000 movups xmmword ptr [rbp + 0x388], xmm3
140a519ec 0f108540020000 movups xmm0, xmmword ptr [rbp + 0x240]
140a519f3 0f118598030000 movups xmmword ptr [rbp + 0x398], xmm0
140a519fa 0f108550020000 movups xmm0, xmmword ptr [rbp + 0x250]
140a51a01 0f1185a8030000 movups xmmword ptr [rbp + 0x3a8], xmm0
140a51a08 488b95a8010000 mov rdx, qword ptr [rbp + 0x1a8]
140a51a0f 48899538030000 mov qword ptr [rbp + 0x338], rdx
140a51a16 0f108598010000 movups xmm0, xmmword ptr [rbp + 0x198]
140a51a1d 0f118528030000 movups xmmword ptr [rbp + 0x328], xmm0
140a51a24 48898dc0030000 mov qword ptr [rbp + 0x3c0], rcx
140a51a2b 488985b8030000 mov qword ptr [rbp + 0x3b8], rax
140a51a32 c7854003000001000000 mov dword ptr [rbp + 0x340], 1
140a51a3c 488b857c040000 mov rax, qword ptr [rbp + 0x47c]
140a51a43 48898544030000 mov qword ptr [rbp + 0x344], rax
140a51a4a c7854c03000001000000 mov dword ptr [rbp + 0x34c], 1
140a51a54 488d4dc0 lea rcx, [rbp - 0x40]
140a51a58 488d95e0020000 lea rdx, [rbp + 0x2e0]
140a51a5f e86cbaf8ff call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a51a64 80bd8602000008 cmp byte ptr [rbp + 0x286], 8
140a51a6b 740c je 0x140a51a79
140a51a6d 488d8d70020000 lea rcx, [rbp + 0x270]
140a51a74 e8370a1900 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a51a79 488d05a0e90702 lea rax, [rip + 0x207e9a0]
140a51a80 4889442420 mov qword ptr [rsp + 0x20], rax
140a51a85 48c744242807000000 mov qword ptr [rsp + 0x28], 7
140a51a8e 4c8d8570040000 lea r8, [rbp + 0x470]
140a51a95 4889f9 mov rcx, rdi
140a51a98 4889f2 mov rdx, rsi
140a51a9b 41b108 mov r9b, 8
140a51a9e e84d8efeff call 0x140a3a8f0 ; country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::resources::texture_presets::TexturePreset>
140a51aa3 90 nop 
140a51aa4 4881c428050000 add rsp, 0x528
140a51aab 5b pop rbx
140a51aac 5f pop rdi
140a51aad 5e pop rsi
140a51aae 415e pop r14
140a51ab0 415f pop r15
140a51ab2 5d pop rbp
140a51ab3 c3 ret 
140a51ab4 488d8e90000000 lea rcx, [rsi + 0x90]
140a51abb e8706fe801 call 0x1428d8a30 ; std::sys::sync::rwlock::futex::RwLock::read_contended
140a51ac0 0fb68698000000 movzx eax, byte ptr [rsi + 0x98]
140a51ac7 4883bef000000000 cmp qword ptr [rsi + 0xf0], 0
140a51acf 0f8589faffff jne 0x140a5155e
140a51ad5 baffffffff mov edx, 0xffffffff
140a51ada f00fc19690000000 lock xadd dword ptr [rsi + 0x90], edx
140a51ae2 ffca dec edx
140a51ae4 89d0 mov eax, edx
140a51ae6 25ffffffbf and eax, 0xbfffffff
140a51aeb f7d8 neg eax
140a51aed 7027 jo 0x140a51b16
140a51aef 488d0db2e80702 lea rcx, [rip + 0x207e8b2]
140a51af6 e81578e801 call 0x1428d9310 ; core::option::unwrap_failed
140a51afb 488d8e90000000 lea rcx, [rsi + 0x90]
140a51b02 e83972e801 call 0x1428d8d40 ; std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
140a51b07 48399e18020000 cmp qword ptr [rsi + 0x218], rbx
140a51b0e 0f8718fbffff ja 0x140a5162c
140a51b14 ebd9 jmp 0x140a51aef
140a51b16 4881c690000000 add rsi, 0x90
140a51b1d 4889f1 mov rcx, rsi
140a51b20 e81b72e801 call 0x1428d8d40 ; std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
140a51b25 488d0d7ce80702 lea rcx, [rip + 0x207e87c]
140a51b2c e8df77e801 call 0x1428d9310 ; core::option::unwrap_failed
140a51b31 c6859f04000001 mov byte ptr [rbp + 0x49f], 1
140a51b38 488d0dc9e80702 lea rcx, [rip + 0x207e8c9]
140a51b3f e8cc77e801 call 0x1428d9310 ; core::option::unwrap_failed
140a51b44 0f0b ud2 
140a51b46 662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a51b50 4889542410 mov qword ptr [rsp + 0x10], rdx
140a51b55 55 push rbp
140a51b56 4157 push r15
140a51b58 4156 push r14
140a51b5a 56 push rsi
140a51b5b 57 push rdi
140a51b5c 53 push rbx
140a51b5d 4883ec38 sub rsp, 0x38
140a51b61 488daa80000000 lea rbp, [rdx + 0x80]
140a51b68 80bd8602000008 cmp byte ptr [rbp + 0x286], 8
140a51b6f 740c je 0x140a51b7d
140a51b71 488d8d70020000 lea rcx, [rbp + 0x270]
140a51b78 e833091900 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a51b7d 90 nop 
140a51b7e 4883c438 add rsp, 0x38
140a51b82 5b pop rbx
140a51b83 5f pop rdi
140a51b84 5e pop rsi
140a51b85 415e pop r14
140a51b87 415f pop r15
140a51b89 5d pop rbp
140a51b8a c3 ret 
140a51b8b 0f1f440000 nop dword ptr [rax + rax]
140a51b90 4889542410 mov qword ptr [rsp + 0x10], rdx
140a51b95 55 push rbp
140a51b96 4157 push r15
140a51b98 4156 push r14
140a51b9a 56 push rsi
140a51b9b 57 push rdi
140a51b9c 53 push rbx
140a51b9d 4883ec38 sub rsp, 0x38
140a51ba1 488daa80000000 lea rbp, [rdx + 0x80]
140a51ba8 80bd9f04000000 cmp byte ptr [rbp + 0x49f], 0
140a51baf 750d jne 0x140a51bbe
140a51bb1 4883c438 add rsp, 0x38
140a51bb5 5b pop rbx
140a51bb6 5f pop rdi
140a51bb7 5e pop rsi
140a51bb8 415e pop r14
140a51bba 415f pop r15
140a51bbc 5d pop rbp
140a51bbd c3 ret 
140a51bbe 488d4dc0 lea rcx, [rbp - 0x40]
140a51bc2 e8e953e4ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a51bc7 ebe8 jmp 0x140a51bb1
140a51bc9 0f1f8000000000 nop dword ptr [rax]
140a51bd0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a51bd5 55 push rbp
140a51bd6 4157 push r15
140a51bd8 4156 push r14
140a51bda 56 push rsi
140a51bdb 57 push rdi
140a51bdc 53 push rbx
140a51bdd 4883ec38 sub rsp, 0x38
140a51be1 488daa80000000 lea rbp, [rdx + 0x80]
140a51be8 488d8de0020000 lea rcx, [rbp + 0x2e0]
140a51bef e8bc53e4ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a51bf4 90 nop 
140a51bf5 4883c438 add rsp, 0x38
140a51bf9 5b pop rbx
140a51bfa 5f pop rdi
140a51bfb 5e pop rsi
140a51bfc 415e pop r14
140a51bfe 415f pop r15
140a51c00 5d pop rbp
140a51c01 c3 ret 
140a51c02 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a51c10 4889542410 mov qword ptr [rsp + 0x10], rdx
140a51c15 55 push rbp
140a51c16 4157 push r15
140a51c18 4156 push r14
140a51c1a 56 push rsi
140a51c1b 57 push rdi
140a51c1c 53 push rbx
140a51c1d 4883ec38 sub rsp, 0x38
140a51c21 488daa80000000 lea rbp, [rdx + 0x80]
140a51c28 c6859f04000000 mov byte ptr [rbp + 0x49f], 0
140a51c2f 488d4dc0 lea rcx, [rbp - 0x40]
140a51c33 e87853e4ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a51c38 c6859f04000000 mov byte ptr [rbp + 0x49f], 0
140a51c3f 4883c438 add rsp, 0x38
140a51c43 5b pop rbx
140a51c44 5f pop rdi
140a51c45 5e pop rsi
140a51c46 415e pop r14
140a51c48 415f pop r15
140a51c4a 5d pop rbp
140a51c4b c3 ret 
140a51c4c 0f1f4000 nop dword ptr [rax]
140a51c50 4889542410 mov qword ptr [rsp + 0x10], rdx
140a51c55 55 push rbp
140a51c56 4157 push r15
140a51c58 4156 push r14
140a51c5a 56 push rsi
140a51c5b 57 push rdi
140a51c5c 53 push rbx
140a51c5d 4883ec38 sub rsp, 0x38
140a51c61 488daa80000000 lea rbp, [rdx + 0x80]
140a51c68 488d8d50010000 lea rcx, [rbp + 0x150]
140a51c6f e83c53e4ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a51c74 90 nop 
140a51c75 4883c438 add rsp, 0x38
140a51c79 5b pop rbx
140a51c7a 5f pop rdi
140a51c7b 5e pop rsi
140a51c7c 415e pop r14
140a51c7e 415f pop r15
140a51c80 5d pop rbp
140a51c81 c3 ret 
140a51c82 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a51c90 4889542410 mov qword ptr [rsp + 0x10], rdx
140a51c95 55 push rbp
140a51c96 4157 push r15
140a51c98 4156 push r14
140a51c9a 56 push rsi
140a51c9b 57 push rdi
140a51c9c 53 push rbx
140a51c9d 4883ec38 sub rsp, 0x38
140a51ca1 488daa80000000 lea rbp, [rdx + 0x80]
140a51ca8 488d8de0020000 lea rcx, [rbp + 0x2e0]
140a51caf e8fc52e4ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a51cb4 90 nop 
140a51cb5 4883c438 add rsp, 0x38
140a51cb9 5b pop rbx
140a51cba 5f pop rdi
140a51cbb 5e pop rsi
140a51cbc 415e pop r14
140a51cbe 415f pop r15
140a51cc0 5d pop rbp
140a51cc1 c3 ret 
140a51cc2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a51cd0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a51cd5 55 push rbp
140a51cd6 4157 push r15
140a51cd8 4156 push r14
140a51cda 56 push rsi
140a51cdb 57 push rdi
140a51cdc 53 push rbx
140a51cdd 4883ec38 sub rsp, 0x38
140a51ce1 488daa80000000 lea rbp, [rdx + 0x80]
140a51ce8 488d4dc0 lea rcx, [rbp - 0x40]
140a51cec e8bf52e4ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a51cf1 90 nop 
140a51cf2 4883c438 add rsp, 0x38
140a51cf6 5b pop rbx
140a51cf7 5f pop rdi
140a51cf8 5e pop rsi
140a51cf9 415e pop r14
140a51cfb 415f pop r15
140a51cfd 5d pop rbp
140a51cfe c3 ret 
140a51cff 90 nop 
140a51d00 4889542410 mov qword ptr [rsp + 0x10], rdx
140a51d05 55 push rbp
140a51d06 4157 push r15
140a51d08 4156 push r14
140a51d0a 56 push rsi
140a51d0b 57 push rdi
140a51d0c 53 push rbx
140a51d0d 4883ec38 sub rsp, 0x38
140a51d11 488daa80000000 lea rbp, [rdx + 0x80]
140a51d18 488d8de0020000 lea rcx, [rbp + 0x2e0]
140a51d1f e88c52e4ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a51d24 90 nop 
140a51d25 4883c438 add rsp, 0x38
140a51d29 5b pop rbx
140a51d2a 5f pop rdi
140a51d2b 5e pop rsi
140a51d2c 415e pop r14
140a51d2e 415f pop r15
140a51d30 5d pop rbp
140a51d31 c3 ret 
