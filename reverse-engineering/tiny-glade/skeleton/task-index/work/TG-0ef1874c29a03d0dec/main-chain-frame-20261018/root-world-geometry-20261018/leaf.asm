1408f4d00 55 push rbp
1408f4d01 4157 push r15
1408f4d03 4156 push r14
1408f4d05 56 push rsi
1408f4d06 57 push rdi
1408f4d07 53 push rbx
1408f4d08 4881ecc8030000 sub rsp, 0x3c8
1408f4d0f 488dac2480000000 lea rbp, [rsp + 0x80]
1408f4d17 48c78540030000feffffff mov qword ptr [rbp + 0x340], 0xfffffffffffffffe
1408f4d22 4889d7 mov rdi, rdx
1408f4d25 4989ce mov r14, rcx
1408f4d28 488d5a38 lea rbx, [rdx + 0x38]
1408f4d2c 48baff0db57f2336ce3d movabs rdx, 0x3dce36237fb50dff
1408f4d36 49b85c882d51f47ca293 movabs r8, 0x93a27cf4512d885c
1408f4d40 4889d9 mov rcx, rbx
1408f4d43 e8a8897eff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
1408f4d48 4885c0 test rax, rax
1408f4d4b 0f841b030000 je 0x1408f506c
1408f4d51 48399718020000 cmp qword ptr [rdi + 0x218], rdx
1408f4d58 0f860e030000 jbe 0x1408f506c
1408f4d5e 488b8710020000 mov rax, qword ptr [rdi + 0x210]
1408f4d65 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
1408f4d69 4885c9 test rcx, rcx
1408f4d6c 0f84fa020000 je 0x1408f506c
1408f4d72 488b87e0010000 mov rax, qword ptr [rdi + 0x1e0]
1408f4d79 48f7d1 not rcx
1408f4d7c 488d0c49 lea rcx, [rcx + rcx*2]
1408f4d80 48c1e105 shl rcx, 5
1408f4d84 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1408f4d8a 0f84dc020000 je 0x1408f506c
1408f4d90 4801c8 add rax, rcx
1408f4d93 488b4010 mov rax, qword ptr [rax + 0x10]
1408f4d97 488b00 mov rax, qword ptr [rax]
1408f4d9a 4885c0 test rax, rax
1408f4d9d 0f84b8020000 je 0x1408f505b
1408f4da3 483d40030000 cmp rax, 0x340
1408f4da9 be40030000 mov esi, 0x340
1408f4dae 480f42f0 cmovb rsi, rax
1408f4db2 4c89742430 mov qword ptr [rsp + 0x30], r14
1408f4db7 48897c2428 mov qword ptr [rsp + 0x28], rdi
1408f4dbc 48c744242017000000 mov qword ptr [rsp + 0x20], 0x17
1408f4dc5 488d152c351b02 lea rdx, [rip + 0x21b352c]
1408f4dcc 4c8d0d32351b02 lea r9, [rip + 0x21b3532]
1408f4dd3 4c8d75c0 lea r14, [rbp - 0x40]
1408f4dd7 41b80d000000 mov r8d, 0xd
1408f4ddd 4c89f1 mov rcx, r14
1408f4de0 e8bbe4d000 call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
1408f4de5 4c8dbda8010000 lea r15, [rbp + 0x1a8]
1408f4dec 49b813713ef988dd1603 movabs r8, 0x316dd88f93e7113
1408f4df6 4c89f9 mov rcx, r15
1408f4df9 4c89f2 mov rdx, r14
1408f4dfc e8df25d100 call 0x1416073e0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::leaf_particles::LeafParticleSsbo>
1408f4e01 49b839367182a8d8d31a movabs r8, 0x1ad3d8a882713639
1408f4e0b 4c89f1 mov rcx, r14
1408f4e0e 4c89fa mov rdx, r15
1408f4e11 e8da8dd100 call 0x14160dbf0 ; country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::systems::render_loop::compute_passes::leaf_particles::ParticleEmitterSsbo>
1408f4e16 488d8da8010000 lea rcx, [rbp + 0x1a8]
1408f4e1d 4c89f2 mov rdx, r14
1408f4e20 e80b95d100 call 0x14160e330 ; country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$2::bind_water_object_push<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408f4e25 80bdde02000008 cmp byte ptr [rbp + 0x2de], 8
1408f4e2c 7425 je 0x1408f4e53
1408f4e2e 488d8db8020000 lea rcx, [rbp + 0x2b8]
1408f4e35 bae8000000 mov edx, 0xe8
1408f4e3a 480395e0020000 add rdx, qword ptr [rbp + 0x2e0]
1408f4e41 49b8b48c515eb6af67ae movabs r8, 0xae67afb65e518cb4
1408f4e4b 4189f1 mov r9d, esi
1408f4e4e e81d670700 call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
1408f4e53 488d4dc0 lea rcx, [rbp - 0x40]
1408f4e57 488d95a8010000 lea rdx, [rbp + 0x1a8]
1408f4e5e 41b890010000 mov r8d, 0x190
1408f4e64 e897ed0202 call 0x142923c00 ; 
1408f4e69 c6853f03000001 mov byte ptr [rbp + 0x33f], 1
1408f4e70 48ba6c052a2050a35942 movabs rdx, 0x4259a350202a056c
1408f4e7a 49b865d43391553f9e1e movabs r8, 0x1e9e3f559133d465
1408f4e84 4889d9 mov rcx, rbx
1408f4e87 e864887eff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
1408f4e8c 4885c0 test rax, rax
1408f4e8f 0f84e3010000 je 0x1408f5078
1408f4e95 48399718020000 cmp qword ptr [rdi + 0x218], rdx
1408f4e9c 0f86d6010000 jbe 0x1408f5078
1408f4ea2 488b8710020000 mov rax, qword ptr [rdi + 0x210]
1408f4ea9 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
1408f4ead 4885c9 test rcx, rcx
1408f4eb0 0f84c2010000 je 0x1408f5078
1408f4eb6 488b87e0010000 mov rax, qword ptr [rdi + 0x1e0]
1408f4ebd 48f7d1 not rcx
1408f4ec0 488d0c49 lea rcx, [rcx + rcx*2]
1408f4ec4 48c1e105 shl rcx, 5
1408f4ec8 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1408f4ece 0f84a4010000 je 0x1408f5078
1408f4ed4 80bdf600000008 cmp byte ptr [rbp + 0xf6], 8
1408f4edb 742c je 0x1408f4f09
1408f4edd 4801c8 add rax, rcx
1408f4ee0 488b4010 mov rax, qword ptr [rax + 0x10]
1408f4ee4 4c8b08 mov r9, qword ptr [rax]
1408f4ee7 488d8dd0000000 lea rcx, [rbp + 0xd0]
1408f4eee bae8000000 mov edx, 0xe8
1408f4ef3 480395f8000000 add rdx, qword ptr [rbp + 0xf8]
1408f4efa 49b8429a7174ef60e86a movabs r8, 0x6ae860ef74719a42
1408f4f04 e867660700 call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
1408f4f09 488d8da8010000 lea rcx, [rbp + 0x1a8]
1408f4f10 488d55c0 lea rdx, [rbp - 0x40]
1408f4f14 41b890010000 mov r8d, 0x190
1408f4f1a e8e1ec0202 call 0x142923c00 ; 
1408f4f1f 0f108548020000 movups xmm0, xmmword ptr [rbp + 0x248]
1408f4f26 0f298590010000 movaps xmmword ptr [rbp + 0x190], xmm0
1408f4f2d 0f108538020000 movups xmm0, xmmword ptr [rbp + 0x238]
1408f4f34 0f298580010000 movaps xmmword ptr [rbp + 0x180], xmm0
1408f4f3b 0f108528020000 movups xmm0, xmmword ptr [rbp + 0x228]
1408f4f42 0f298570010000 movaps xmmword ptr [rbp + 0x170], xmm0
1408f4f49 0f108518020000 movups xmm0, xmmword ptr [rbp + 0x218]
1408f4f50 0f298560010000 movaps xmmword ptr [rbp + 0x160], xmm0
1408f4f57 0f108508020000 movups xmm0, xmmword ptr [rbp + 0x208]
1408f4f5e 0f298550010000 movaps xmmword ptr [rbp + 0x150], xmm0
1408f4f65 488b85e0020000 mov rax, qword ptr [rbp + 0x2e0]
1408f4f6c 488b8d28030000 mov rcx, qword ptr [rbp + 0x328]
1408f4f73 488b95b8010000 mov rdx, qword ptr [rbp + 0x1b8]
1408f4f7a 488955d0 mov qword ptr [rbp - 0x30], rdx
1408f4f7e 0f1085a8010000 movups xmm0, xmmword ptr [rbp + 0x1a8]
1408f4f85 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
1408f4f89 0f1085c0010000 movups xmm0, xmmword ptr [rbp + 0x1c0]
1408f4f90 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
1408f4f94 488b95d0010000 mov rdx, qword ptr [rbp + 0x1d0]
1408f4f9b 48895500 mov qword ptr [rbp], rdx
1408f4f9f 0f1085d8010000 movups xmm0, xmmword ptr [rbp + 0x1d8]
1408f4fa6 0f1145d8 movups xmmword ptr [rbp - 0x28], xmm0
1408f4faa 488b95e8010000 mov rdx, qword ptr [rbp + 0x1e8]
1408f4fb1 488955e8 mov qword ptr [rbp - 0x18], rdx
1408f4fb5 0f108558020000 movups xmm0, xmmword ptr [rbp + 0x258]
1408f4fbc 0f108d68020000 movups xmm1, xmmword ptr [rbp + 0x268]
1408f4fc3 0f109578020000 movups xmm2, xmmword ptr [rbp + 0x278]
1408f4fca 0f109d88020000 movups xmm3, xmmword ptr [rbp + 0x288]
1408f4fd1 0f114538 movups xmmword ptr [rbp + 0x38], xmm0
1408f4fd5 0f114d48 movups xmmword ptr [rbp + 0x48], xmm1
1408f4fd9 0f115558 movups xmmword ptr [rbp + 0x58], xmm2
1408f4fdd 0f115d68 movups xmmword ptr [rbp + 0x68], xmm3
1408f4fe1 0f108598020000 movups xmm0, xmmword ptr [rbp + 0x298]
1408f4fe8 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
1408f4fec 0f1085a8020000 movups xmm0, xmmword ptr [rbp + 0x2a8]
1408f4ff3 0f118588000000 movups xmmword ptr [rbp + 0x88], xmm0
1408f4ffa 488b9500020000 mov rdx, qword ptr [rbp + 0x200]
1408f5001 48895518 mov qword ptr [rbp + 0x18], rdx
1408f5005 0f1085f0010000 movups xmm0, xmmword ptr [rbp + 0x1f0]
1408f500c 0f114508 movups xmmword ptr [rbp + 8], xmm0
1408f5010 48898da0000000 mov qword ptr [rbp + 0xa0], rcx
1408f5017 48898598000000 mov qword ptr [rbp + 0x98], rax
1408f501e c7452001000000 mov dword ptr [rbp + 0x20], 1
1408f5025 897524 mov dword ptr [rbp + 0x24], esi
1408f5028 48b80100000001000000 movabs rax, 0x100000001
1408f5032 48894528 mov qword ptr [rbp + 0x28], rax
1408f5036 488d8d50010000 lea rcx, [rbp + 0x150]
1408f503d 488d55c0 lea rdx, [rbp - 0x40]
1408f5041 e88a840e00 call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
1408f5046 80bdde02000008 cmp byte ptr [rbp + 0x2de], 8
1408f504d 740c je 0x1408f505b
1408f504f 488d8dc8020000 lea rcx, [rbp + 0x2c8]
1408f5056 e855d42e00 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1408f505b 90 nop 
1408f505c 4881c4c8030000 add rsp, 0x3c8
1408f5063 5b pop rbx
1408f5064 5f pop rdi
1408f5065 5e pop rsi
1408f5066 415e pop r14
1408f5068 415f pop r15
1408f506a 5d pop rbp
1408f506b c3 ret 
1408f506c 488d0d6d321b02 lea rcx, [rip + 0x21b326d]
1408f5073 e89842fe01 call 0x1428d9310 ; core::option::unwrap_failed
1408f5078 c6853f03000001 mov byte ptr [rbp + 0x33f], 1
1408f507f 488d0d9a321b02 lea rcx, [rip + 0x21b329a]
1408f5086 e88542fe01 call 0x1428d9310 ; core::option::unwrap_failed
1408f508b 0f0b ud2 
1408f508d 0f1f00 nop dword ptr [rax]
1408f5090 4889542410 mov qword ptr [rsp + 0x10], rdx
1408f5095 55 push rbp
1408f5096 4157 push r15
1408f5098 4156 push r14
1408f509a 56 push rsi
1408f509b 57 push rdi
1408f509c 53 push rbx
1408f509d 4883ec38 sub rsp, 0x38
1408f50a1 488daa80000000 lea rbp, [rdx + 0x80]
1408f50a8 c6853f03000000 mov byte ptr [rbp + 0x33f], 0
1408f50af 488d4dc0 lea rcx, [rbp - 0x40]
1408f50b3 e8f81efaff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408f50b8 c6853f03000000 mov byte ptr [rbp + 0x33f], 0
1408f50bf 4883c438 add rsp, 0x38
1408f50c3 5b pop rbx
1408f50c4 5f pop rdi
1408f50c5 5e pop rsi
1408f50c6 415e pop r14
1408f50c8 415f pop r15
1408f50ca 5d pop rbp
1408f50cb c3 ret 
1408f50cc 0f1f4000 nop dword ptr [rax]
1408f50d0 4889542410 mov qword ptr [rsp + 0x10], rdx
1408f50d5 55 push rbp
1408f50d6 4157 push r15
1408f50d8 4156 push r14
1408f50da 56 push rsi
1408f50db 57 push rdi
1408f50dc 53 push rbx
1408f50dd 4883ec38 sub rsp, 0x38
1408f50e1 488daa80000000 lea rbp, [rdx + 0x80]
1408f50e8 80bdde02000008 cmp byte ptr [rbp + 0x2de], 8
1408f50ef 740c je 0x1408f50fd
1408f50f1 488d8dc8020000 lea rcx, [rbp + 0x2c8]
1408f50f8 e8b3d32e00 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1408f50fd 90 nop 
1408f50fe 4883c438 add rsp, 0x38
1408f5102 5b pop rbx
1408f5103 5f pop rdi
1408f5104 5e pop rsi
1408f5105 415e pop r14
1408f5107 415f pop r15
1408f5109 5d pop rbp
1408f510a c3 ret 
1408f510b 0f1f440000 nop dword ptr [rax + rax]
1408f5110 4889542410 mov qword ptr [rsp + 0x10], rdx
1408f5115 55 push rbp
1408f5116 4157 push r15
1408f5118 4156 push r14
1408f511a 56 push rsi
1408f511b 57 push rdi
1408f511c 53 push rbx
1408f511d 4883ec38 sub rsp, 0x38
1408f5121 488daa80000000 lea rbp, [rdx + 0x80]
1408f5128 80bd3f03000000 cmp byte ptr [rbp + 0x33f], 0
1408f512f 750d jne 0x1408f513e
1408f5131 4883c438 add rsp, 0x38
1408f5135 5b pop rbx
1408f5136 5f pop rdi
1408f5137 5e pop rsi
1408f5138 415e pop r14
1408f513a 415f pop r15
1408f513c 5d pop rbp
1408f513d c3 ret 
1408f513e 488d4dc0 lea rcx, [rbp - 0x40]
1408f5142 e8691efaff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408f5147 ebe8 jmp 0x1408f5131
1408f5149 0f1f8000000000 nop dword ptr [rax]
1408f5150 4889542410 mov qword ptr [rsp + 0x10], rdx
1408f5155 55 push rbp
1408f5156 4157 push r15
1408f5158 4156 push r14
1408f515a 56 push rsi
1408f515b 57 push rdi
1408f515c 53 push rbx
1408f515d 4883ec38 sub rsp, 0x38
1408f5161 488daa80000000 lea rbp, [rdx + 0x80]
1408f5168 488d8da8010000 lea rcx, [rbp + 0x1a8]
1408f516f e83c1efaff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408f5174 90 nop 
1408f5175 4883c438 add rsp, 0x38
1408f5179 5b pop rbx
1408f517a 5f pop rdi
1408f517b 5e pop rsi
1408f517c 415e pop r14
1408f517e 415f pop r15
1408f5180 5d pop rbp
1408f5181 c3 ret 
