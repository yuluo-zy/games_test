140866540: push rbp
140866541: push rsi
140866542: push rdi
140866543: sub rsp, 0x500
14086654a: lea rbp, [rsp + 0x80]
140866552: mov qword ptr [rbp + 0x478], 0xfffffffffffffffe
14086655d: mov qword ptr [rsp + 0x30], rcx
140866562: mov qword ptr [rsp + 0x28], rdx
140866567: mov qword ptr [rsp + 0x20], 0x2b
140866570: lea rdx, [rip + 0x22300a1]
140866577: lea r9, [rip + 0x22300b4]
14086657e: lea rsi, [rbp + 0x150]
140866585: mov r8d, 0x1a
14086658b: mov rcx, rsi
14086658e: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140866593: lea rdi, [rbp + 0x2e8]
14086659a: movabs r8, 0xfc7c5f7286f434fe
1408665a4: mov rcx, rdi
1408665a7: mov rdx, rsi
1408665aa: call 0x1416095a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_to_cpu_mapped_storage_buffer<country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo>
1408665af: lea rsi, [rbp - 0x40]
1408665b3: movabs r8, 0x5baefac4a34b3dee
1408665bd: mov rcx, rsi
1408665c0: mov rdx, rdi
1408665c3: call 0x14160da20 ; country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>
1408665c8: lea rcx, [rbp + 0x150]
1408665cf: movabs r8, 0xf1c07a2b8012765a
1408665d9: mov rdx, rsi
1408665dc: call 0x14160d4b0 ; country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>
1408665e1: cmp byte ptr [rbp + 0x286], 8
1408665e8: je 0x140866612
1408665ea: lea rcx, [rbp + 0x260]
1408665f1: mov edx, 0xe8
1408665f6: add rdx, qword ptr [rbp + 0x288]
1408665fd: movabs r8, 0xe9511a616b75789
140866607: mov r9d, 0xa
14086660d: call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
140866612: lea rcx, [rbp + 0x2e8]
140866619: lea rdx, [rbp + 0x150]
140866620: mov r8d, 0x190
140866626: call 0x142923c00
14086662b: movups xmm0, xmmword ptr [rbp + 0x388]
140866632: movaps xmmword ptr [rbp], xmm0
140866636: movups xmm0, xmmword ptr [rbp + 0x378]
14086663d: movaps xmmword ptr [rbp - 0x10], xmm0
140866641: movups xmm0, xmmword ptr [rbp + 0x368]
140866648: movaps xmmword ptr [rbp - 0x20], xmm0
14086664c: movups xmm0, xmmword ptr [rbp + 0x358]
140866653: movaps xmmword ptr [rbp - 0x30], xmm0
140866657: movups xmm0, xmmword ptr [rbp + 0x348]
14086665e: movaps xmmword ptr [rbp - 0x40], xmm0
140866662: mov rax, qword ptr [rbp + 0x420]
140866669: mov rcx, qword ptr [rbp + 0x468]
140866670: mov rdx, qword ptr [rbp + 0x2f8]
140866677: mov qword ptr [rbp + 0x160], rdx
14086667e: movups xmm0, xmmword ptr [rbp + 0x2e8]
140866685: movaps xmmword ptr [rbp + 0x150], xmm0
14086668c: movups xmm0, xmmword ptr [rbp + 0x300]
140866693: movaps xmmword ptr [rbp + 0x180], xmm0
14086669a: mov rdx, qword ptr [rbp + 0x310]
1408666a1: mov qword ptr [rbp + 0x190], rdx
1408666a8: movups xmm0, xmmword ptr [rbp + 0x318]
1408666af: movups xmmword ptr [rbp + 0x168], xmm0
1408666b6: mov rdx, qword ptr [rbp + 0x328]
1408666bd: mov qword ptr [rbp + 0x178], rdx
1408666c4: movups xmm0, xmmword ptr [rbp + 0x398]
1408666cb: movups xmm1, xmmword ptr [rbp + 0x3a8]
1408666d2: movups xmm2, xmmword ptr [rbp + 0x3b8]
1408666d9: movups xmm3, xmmword ptr [rbp + 0x3c8]
1408666e0: movups xmmword ptr [rbp + 0x1c8], xmm0
1408666e7: movups xmmword ptr [rbp + 0x1d8], xmm1
1408666ee: movups xmmword ptr [rbp + 0x1e8], xmm2
1408666f5: movups xmmword ptr [rbp + 0x1f8], xmm3
1408666fc: movups xmm0, xmmword ptr [rbp + 0x3d8]
140866703: movups xmmword ptr [rbp + 0x208], xmm0
14086670a: movups xmm0, xmmword ptr [rbp + 0x3e8]
140866711: movups xmmword ptr [rbp + 0x218], xmm0
140866718: mov rdx, qword ptr [rbp + 0x340]
14086671f: mov qword ptr [rbp + 0x1a8], rdx
140866726: movups xmm0, xmmword ptr [rbp + 0x330]
14086672d: movups xmmword ptr [rbp + 0x198], xmm0
140866734: mov qword ptr [rbp + 0x230], rcx
14086673b: mov qword ptr [rbp + 0x228], rax
140866742: movaps xmm0, xmmword ptr [rip + 0x222f697]
140866749: movaps xmmword ptr [rbp + 0x1b0], xmm0
140866750: lea rcx, [rbp - 0x40]
140866754: lea rdx, [rbp + 0x150]
14086675b: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140866760: cmp byte ptr [rbp + 0x41e], 8
140866767: je 0x140866775
140866769: lea rcx, [rbp + 0x408]
140866770: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140866775: nop 
140866776: add rsp, 0x500
14086677d: pop rdi
14086677e: pop rsi
14086677f: pop rbp
140866780: ret 
140866781: nop word ptr cs:[rax + rax]
140866790: mov qword ptr [rsp + 0x10], rdx
140866795: push rbp
140866796: push rsi
140866797: push rdi
140866798: sub rsp, 0x40
14086679c: lea rbp, [rdx + 0x80]
1408667a3: cmp byte ptr [rbp + 0x41e], 8
1408667aa: je 0x1408667b8
1408667ac: lea rcx, [rbp + 0x408]
1408667b3: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1408667b8: nop 
1408667b9: add rsp, 0x40
1408667bd: pop rdi
1408667be: pop rsi
1408667bf: pop rbp
1408667c0: ret 
1408667c1: nop word ptr cs:[rax + rax]
1408667d0: mov qword ptr [rsp + 0x10], rdx
1408667d5: push rbp
1408667d6: push rsi
1408667d7: push rdi
1408667d8: sub rsp, 0x40
1408667dc: lea rbp, [rdx + 0x80]
1408667e3: lea rcx, [rbp + 0x150]
1408667ea: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408667ef: nop 
1408667f0: add rsp, 0x40
1408667f4: pop rdi
1408667f5: pop rsi
1408667f6: pop rbp
1408667f7: ret 
