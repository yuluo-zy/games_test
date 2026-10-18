140a01620 55 push rbp
140a01621 4157 push r15
140a01623 4156 push r14
140a01625 56 push rsi
140a01626 57 push rdi
140a01627 53 push rbx
140a01628 4881ecf8030000 sub rsp, 0x3f8
140a0162f 488dac2480000000 lea rbp, [rsp + 0x80]
140a01637 440f298560030000 movaps xmmword ptr [rbp + 0x360], xmm8
140a0163f 0f29bd50030000 movaps xmmword ptr [rbp + 0x350], xmm7
140a01646 0f29b540030000 movaps xmmword ptr [rbp + 0x340], xmm6
140a0164d 48c78538030000feffffff mov qword ptr [rbp + 0x338], 0xfffffffffffffffe
140a01658 4c89c3 mov rbx, r8
140a0165b 4889d6 mov rsi, rdx
140a0165e 4989ce mov r14, rcx
140a01661 488d7a38 lea rdi, [rdx + 0x38]
140a01665 48ba80db3f3b7610ab4c movabs rdx, 0x4cab10763b3fdb80
140a0166f 49b84db53475ef22a401 movabs r8, 0x1a422ef7534b54d
140a01679 4889f9 mov rcx, rdi
140a0167c e86fc06dff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
140a01681 4885c0 test rax, rax
140a01684 0f8416040000 je 0x140a01aa0
140a0168a 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140a01691 0f8609040000 jbe 0x140a01aa0
140a01697 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140a0169e 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
140a016a2 4885c9 test rcx, rcx
140a016a5 0f84f5030000 je 0x140a01aa0
140a016ab 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
140a016b2 48f7d1 not rcx
140a016b5 488d0c49 lea rcx, [rcx + rcx*2]
140a016b9 48c1e105 shl rcx, 5
140a016bd 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
140a016c3 0f84d7030000 je 0x140a01aa0
140a016c9 4801c8 add rax, rcx
140a016cc 488b4010 mov rax, qword ptr [rax + 0x10]
140a016d0 f30f1030 movss xmm6, dword ptr [rax]
140a016d4 f3440f104004 movss xmm8, dword ptr [rax + 4]
140a016da 48ba407cd487e7567b3d movabs rdx, 0x3d7b56e787d47c40
140a016e4 49b8c7bf438d0ab1b067 movabs r8, 0x67b0b10a8d43bfc7
140a016ee 4889f9 mov rcx, rdi
140a016f1 e8fabf6dff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
140a016f6 4885c0 test rax, rax
140a016f9 0f84ad030000 je 0x140a01aac
140a016ff 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140a01706 0f86a0030000 jbe 0x140a01aac
140a0170c 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140a01713 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
140a01717 4885c9 test rcx, rcx
140a0171a 0f848c030000 je 0x140a01aac
140a01720 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
140a01727 48f7d1 not rcx
140a0172a 488d0c49 lea rcx, [rcx + rcx*2]
140a0172e 48c1e105 shl rcx, 5
140a01732 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
140a01738 0f846e030000 je 0x140a01aac
140a0173e 4801c8 add rax, rcx
140a01741 488b4010 mov rax, qword ptr [rax + 0x10]
140a01745 f30f10b810010000 movss xmm7, dword ptr [rax + 0x110]
140a0174d 4c89742430 mov qword ptr [rsp + 0x30], r14
140a01752 4889742428 mov qword ptr [rsp + 0x28], rsi
140a01757 48c74424201f000000 mov qword ptr [rsp + 0x20], 0x1f
140a01760 488d1569430c02 lea rdx, [rip + 0x20c4369]
140a01767 4c8d0d7b430c02 lea r9, [rip + 0x20c437b]
140a0176e 4c8d75c0 lea r14, [rbp - 0x40]
140a01772 41b819000000 mov r8d, 0x19
140a01778 4c89f1 mov rcx, r14
140a0177b e8201bc000 call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a01780 4c8dbd50010000 lea r15, [rbp + 0x150]
140a01787 49b8ba36bd4496b668a8 movabs r8, 0xa868b69644bd36ba
140a01791 4c89f9 mov rcx, r15
140a01794 4c89f2 mov rdx, r14
140a01797 e8d47fc000 call 0x141609770 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_to_cpu_mapped_storage_buffer<country_core::resources::compute_mouse_depth_value::MouseDepthValueSsbo>
140a0179c 48899de0020000 mov qword ptr [rbp + 0x2e0], rbx
140a017a3 c745c002000000 mov dword ptr [rbp - 0x40], 2
140a017aa 4c89742420 mov qword ptr [rsp + 0x20], r14
140a017af 488d8de0020000 lea rcx, [rbp + 0x2e0]
140a017b6 49b8cfaa9d259d50ce6f movabs r8, 0x6fce509d259daacf
140a017c0 4c89fa mov rdx, r15
140a017c3 41b119 mov r9b, 0x19
140a017c6 e875caeeff call 0x1408ee240 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >
140a017cb 488d4dc0 lea rcx, [rbp - 0x40]
140a017cf 488d9550010000 lea rdx, [rbp + 0x150]
140a017d6 41b890010000 mov r8d, 0x190
140a017dc e81f24f201 call 0x142923c00 ; 
140a017e1 80bdf600000008 cmp byte ptr [rbp + 0xf6], 8
140a017e8 742c je 0x140a01816
140a017ea 488d8dd0000000 lea rcx, [rbp + 0xd0]
140a017f1 bae8000000 mov edx, 0xe8
140a017f6 480395f8000000 add rdx, qword ptr [rbp + 0xf8]
140a017fd f3440f11442420 movss dword ptr [rsp + 0x20], xmm8
140a01804 49b883ac296c71ce8f3b movabs r8, 0x3b8fce716c29ac83
140a0180e 0f28de movaps xmm3, xmm6
140a01811 e82aa1f6ff call 0x14096b940 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<glam::f32::vec2::Vec2>,glam::f32::vec2::Vec2>
140a01816 488d8d50010000 lea rcx, [rbp + 0x150]
140a0181d 488d55c0 lea rdx, [rbp - 0x40]
140a01821 41b890010000 mov r8d, 0x190
140a01827 e8d423f201 call 0x142923c00 ; 
140a0182c c6853703000001 mov byte ptr [rbp + 0x337], 1
140a01833 48baa612238ca9dc4d86 movabs rdx, 0x864ddca98c2312a6
140a0183d 49b84e489a9e36e9b027 movabs r8, 0x27b0e9369e9a484e
140a01847 4889f9 mov rcx, rdi
140a0184a e8a1be6dff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
140a0184f 4885c0 test rax, rax
140a01852 0f8460020000 je 0x140a01ab8
140a01858 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140a0185f 0f8653020000 jbe 0x140a01ab8
140a01865 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140a0186c 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
140a01870 4885c9 test rcx, rcx
140a01873 0f843f020000 je 0x140a01ab8
140a01879 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
140a01880 48f7d1 not rcx
140a01883 488d0c49 lea rcx, [rcx + rcx*2]
140a01887 48c1e105 shl rcx, 5
140a0188b 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
140a01891 0f8421020000 je 0x140a01ab8
140a01897 80bd8602000008 cmp byte ptr [rbp + 0x286], 8
140a0189e 7441 je 0x140a018e1
140a018a0 4801c8 add rax, rcx
140a018a3 488b4010 mov rax, qword ptr [rax + 0x10]
140a018a7 8b4804 mov ecx, dword ptr [rax + 4]
140a018aa f3480f2ac1 cvtsi2ss xmm0, rcx
140a018af 8b00 mov eax, dword ptr [rax]
140a018b1 0f57db xorps xmm3, xmm3
140a018b4 f3480f2ad8 cvtsi2ss xmm3, rax
140a018b9 488d8d60020000 lea rcx, [rbp + 0x260]
140a018c0 bae8000000 mov edx, 0xe8
140a018c5 48039588020000 add rdx, qword ptr [rbp + 0x288]
140a018cc f30f11442420 movss dword ptr [rsp + 0x20], xmm0
140a018d2 49b887471c999babfd69 movabs r8, 0x69fdab9b991c4787
140a018dc e85fa0f6ff call 0x14096b940 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<glam::f32::vec2::Vec2>,glam::f32::vec2::Vec2>
140a018e1 488d4dc0 lea rcx, [rbp - 0x40]
140a018e5 488d9550010000 lea rdx, [rbp + 0x150]
140a018ec 41b890010000 mov r8d, 0x190
140a018f2 e80923f201 call 0x142923c00 ; 
140a018f7 80bdf600000008 cmp byte ptr [rbp + 0xf6], 8
140a018fe 7435 je 0x140a01935
140a01900 0f57c0 xorps xmm0, xmm0
140a01903 f30f5fc7 maxss xmm0, xmm7
140a01907 f30f101df53ff201 movss xmm3, dword ptr [rip + 0x1f23ff5]
140a0190f f30f5dd8 minss xmm3, xmm0
140a01913 488d8dd0000000 lea rcx, [rbp + 0xd0]
140a0191a bae8000000 mov edx, 0xe8
140a0191f 480395f8000000 add rdx, qword ptr [rbp + 0xf8]
140a01926 49b846d01ee430fe0bc1 movabs r8, 0xc10bfe30e41ed046
140a01930 e8fba0f6ff call 0x14096ba30 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<f32>,f32>
140a01935 488d8d50010000 lea rcx, [rbp + 0x150]
140a0193c 488d55c0 lea rdx, [rbp - 0x40]
140a01940 41b890010000 mov r8d, 0x190
140a01946 e8b522f201 call 0x142923c00 ; 
140a0194b 0f1085f0010000 movups xmm0, xmmword ptr [rbp + 0x1f0]
140a01952 0f298520030000 movaps xmmword ptr [rbp + 0x320], xmm0
140a01959 0f1085e0010000 movups xmm0, xmmword ptr [rbp + 0x1e0]
140a01960 0f298510030000 movaps xmmword ptr [rbp + 0x310], xmm0
140a01967 0f1085d0010000 movups xmm0, xmmword ptr [rbp + 0x1d0]
140a0196e 0f298500030000 movaps xmmword ptr [rbp + 0x300], xmm0
140a01975 0f1085c0010000 movups xmm0, xmmword ptr [rbp + 0x1c0]
140a0197c 0f2985f0020000 movaps xmmword ptr [rbp + 0x2f0], xmm0
140a01983 0f1085b0010000 movups xmm0, xmmword ptr [rbp + 0x1b0]
140a0198a 0f2985e0020000 movaps xmmword ptr [rbp + 0x2e0], xmm0
140a01991 488b8588020000 mov rax, qword ptr [rbp + 0x288]
140a01998 488b8dd0020000 mov rcx, qword ptr [rbp + 0x2d0]
140a0199f 488b9560010000 mov rdx, qword ptr [rbp + 0x160]
140a019a6 488955d0 mov qword ptr [rbp - 0x30], rdx
140a019aa 0f108550010000 movups xmm0, xmmword ptr [rbp + 0x150]
140a019b1 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
140a019b5 0f108568010000 movups xmm0, xmmword ptr [rbp + 0x168]
140a019bc 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
140a019c0 488b9578010000 mov rdx, qword ptr [rbp + 0x178]
140a019c7 48895500 mov qword ptr [rbp], rdx
140a019cb 0f108580010000 movups xmm0, xmmword ptr [rbp + 0x180]
140a019d2 0f1145d8 movups xmmword ptr [rbp - 0x28], xmm0
140a019d6 488b9590010000 mov rdx, qword ptr [rbp + 0x190]
140a019dd 488955e8 mov qword ptr [rbp - 0x18], rdx
140a019e1 0f108500020000 movups xmm0, xmmword ptr [rbp + 0x200]
140a019e8 0f108d10020000 movups xmm1, xmmword ptr [rbp + 0x210]
140a019ef 0f109520020000 movups xmm2, xmmword ptr [rbp + 0x220]
140a019f6 0f109d30020000 movups xmm3, xmmword ptr [rbp + 0x230]
140a019fd 0f114538 movups xmmword ptr [rbp + 0x38], xmm0
140a01a01 0f114d48 movups xmmword ptr [rbp + 0x48], xmm1
140a01a05 0f115558 movups xmmword ptr [rbp + 0x58], xmm2
140a01a09 0f115d68 movups xmmword ptr [rbp + 0x68], xmm3
140a01a0d 0f108540020000 movups xmm0, xmmword ptr [rbp + 0x240]
140a01a14 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
140a01a18 0f108550020000 movups xmm0, xmmword ptr [rbp + 0x250]
140a01a1f 0f118588000000 movups xmmword ptr [rbp + 0x88], xmm0
140a01a26 488b95a8010000 mov rdx, qword ptr [rbp + 0x1a8]
140a01a2d 48895518 mov qword ptr [rbp + 0x18], rdx
140a01a31 0f108598010000 movups xmm0, xmmword ptr [rbp + 0x198]
140a01a38 0f114508 movups xmmword ptr [rbp + 8], xmm0
140a01a3c 48898da0000000 mov qword ptr [rbp + 0xa0], rcx
140a01a43 48898598000000 mov qword ptr [rbp + 0x98], rax
140a01a4a 0f28058f75f501 movaps xmm0, xmmword ptr [rip + 0x1f5758f]
140a01a51 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
140a01a55 488d8de0020000 lea rcx, [rbp + 0x2e0]
140a01a5c 488d55c0 lea rdx, [rbp - 0x40]
140a01a60 e86bbafdff call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a01a65 80bd8602000008 cmp byte ptr [rbp + 0x286], 8
140a01a6c 740c je 0x140a01a7a
140a01a6e 488d8d70020000 lea rcx, [rbp + 0x270]
140a01a75 e8360a1e00 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a01a7a 0f28b540030000 movaps xmm6, xmmword ptr [rbp + 0x340]
140a01a81 0f28bd50030000 movaps xmm7, xmmword ptr [rbp + 0x350]
140a01a88 440f288560030000 movaps xmm8, xmmword ptr [rbp + 0x360]
140a01a90 4881c4f8030000 add rsp, 0x3f8
140a01a97 5b pop rbx
140a01a98 5f pop rdi
140a01a99 5e pop rsi
140a01a9a 415e pop r14
140a01a9c 415f pop r15
140a01a9e 5d pop rbp
140a01a9f c3 ret 
140a01aa0 488d0df93f0c02 lea rcx, [rip + 0x20c3ff9]
140a01aa7 e86478ed01 call 0x1428d9310 ; core::option::unwrap_failed
140a01aac 488d0d05400c02 lea rcx, [rip + 0x20c4005]
140a01ab3 e85878ed01 call 0x1428d9310 ; core::option::unwrap_failed
140a01ab8 c6853703000001 mov byte ptr [rbp + 0x337], 1
140a01abf 488d0d42400c02 lea rcx, [rip + 0x20c4042]
140a01ac6 e84578ed01 call 0x1428d9310 ; core::option::unwrap_failed
140a01acb 0f0b ud2 
140a01acd 0f1f00 nop dword ptr [rax]
140a01ad0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a01ad5 55 push rbp
140a01ad6 4157 push r15
140a01ad8 4156 push r14
140a01ada 56 push rsi
140a01adb 57 push rdi
140a01adc 53 push rbx
140a01add 4883ec68 sub rsp, 0x68
140a01ae1 488daa80000000 lea rbp, [rdx + 0x80]
140a01ae8 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
140a01aee 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
140a01af3 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
140a01af8 c6853703000000 mov byte ptr [rbp + 0x337], 0
140a01aff 488d8d50010000 lea rcx, [rbp + 0x150]
140a01b06 e8a554e9ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a01b0b c6853703000000 mov byte ptr [rbp + 0x337], 0
140a01b12 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
140a01b17 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
140a01b1c 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
140a01b22 4883c468 add rsp, 0x68
140a01b26 5b pop rbx
140a01b27 5f pop rdi
140a01b28 5e pop rsi
140a01b29 415e pop r14
140a01b2b 415f pop r15
140a01b2d 5d pop rbp
140a01b2e c3 ret 
140a01b2f 90 nop 
140a01b30 4889542410 mov qword ptr [rsp + 0x10], rdx
140a01b35 55 push rbp
140a01b36 4157 push r15
140a01b38 4156 push r14
140a01b3a 56 push rsi
140a01b3b 57 push rdi
140a01b3c 53 push rbx
140a01b3d 4883ec68 sub rsp, 0x68
140a01b41 488daa80000000 lea rbp, [rdx + 0x80]
140a01b48 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
140a01b4e 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
140a01b53 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
140a01b58 80bd8602000008 cmp byte ptr [rbp + 0x286], 8
140a01b5f 740c je 0x140a01b6d
140a01b61 488d8d70020000 lea rcx, [rbp + 0x270]
140a01b68 e843091e00 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a01b6d 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
140a01b72 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
140a01b77 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
140a01b7d 4883c468 add rsp, 0x68
140a01b81 5b pop rbx
140a01b82 5f pop rdi
140a01b83 5e pop rsi
140a01b84 415e pop r14
140a01b86 415f pop r15
140a01b88 5d pop rbp
140a01b89 c3 ret 
140a01b8a 660f1f440000 nop word ptr [rax + rax]
140a01b90 4889542410 mov qword ptr [rsp + 0x10], rdx
140a01b95 55 push rbp
140a01b96 4157 push r15
140a01b98 4156 push r14
140a01b9a 56 push rsi
140a01b9b 57 push rdi
140a01b9c 53 push rbx
140a01b9d 4883ec68 sub rsp, 0x68
140a01ba1 488daa80000000 lea rbp, [rdx + 0x80]
140a01ba8 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
140a01bae 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
140a01bb3 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
140a01bb8 80bd3703000000 cmp byte ptr [rbp + 0x337], 0
140a01bbf 751d jne 0x140a01bde
140a01bc1 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
140a01bc6 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
140a01bcb 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
140a01bd1 4883c468 add rsp, 0x68
140a01bd5 5b pop rbx
140a01bd6 5f pop rdi
140a01bd7 5e pop rsi
140a01bd8 415e pop r14
140a01bda 415f pop r15
140a01bdc 5d pop rbp
140a01bdd c3 ret 
140a01bde 488d8d50010000 lea rcx, [rbp + 0x150]
140a01be5 e8c653e9ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a01bea ebd5 jmp 0x140a01bc1
140a01bec 0f1f4000 nop dword ptr [rax]
140a01bf0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a01bf5 55 push rbp
140a01bf6 4157 push r15
140a01bf8 4156 push r14
140a01bfa 56 push rsi
140a01bfb 57 push rdi
140a01bfc 53 push rbx
140a01bfd 4883ec68 sub rsp, 0x68
140a01c01 488daa80000000 lea rbp, [rdx + 0x80]
140a01c08 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
140a01c0e 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
140a01c13 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
140a01c18 488d8d50010000 lea rcx, [rbp + 0x150]
140a01c1f e88c53e9ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a01c24 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
140a01c29 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
140a01c2e 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
140a01c34 4883c468 add rsp, 0x68
140a01c38 5b pop rbx
140a01c39 5f pop rdi
140a01c3a 5e pop rsi
140a01c3b 415e pop r14
140a01c3d 415f pop r15
140a01c3f 5d pop rbp
140a01c40 c3 ret 
140a01c41 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a01c50 4889542410 mov qword ptr [rsp + 0x10], rdx
140a01c55 55 push rbp
140a01c56 4157 push r15
140a01c58 4156 push r14
140a01c5a 56 push rsi
140a01c5b 57 push rdi
140a01c5c 53 push rbx
140a01c5d 4883ec68 sub rsp, 0x68
140a01c61 488daa80000000 lea rbp, [rdx + 0x80]
140a01c68 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
140a01c6e 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
140a01c73 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
140a01c78 488d4dc0 lea rcx, [rbp - 0x40]
140a01c7c e82f53e9ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a01c81 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
140a01c86 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
140a01c8b 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
140a01c91 4883c468 add rsp, 0x68
140a01c95 5b pop rbx
140a01c96 5f pop rdi
140a01c97 5e pop rsi
140a01c98 415e pop r14
140a01c9a 415f pop r15
140a01c9c 5d pop rbp
140a01c9d c3 ret 
140a01c9e 6690 nop 
140a01ca0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a01ca5 55 push rbp
140a01ca6 4157 push r15
140a01ca8 4156 push r14
140a01caa 56 push rsi
140a01cab 57 push rdi
140a01cac 53 push rbx
140a01cad 4883ec68 sub rsp, 0x68
140a01cb1 488daa80000000 lea rbp, [rdx + 0x80]
140a01cb8 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
140a01cbe 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
140a01cc3 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
140a01cc8 488d4dc0 lea rcx, [rbp - 0x40]
140a01ccc e8df52e9ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a01cd1 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
140a01cd6 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
140a01cdb 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
140a01ce1 4883c468 add rsp, 0x68
140a01ce5 5b pop rbx
140a01ce6 5f pop rdi
140a01ce7 5e pop rsi
140a01ce8 415e pop r14
140a01cea 415f pop r15
140a01cec 5d pop rbp
140a01ced c3 ret 
