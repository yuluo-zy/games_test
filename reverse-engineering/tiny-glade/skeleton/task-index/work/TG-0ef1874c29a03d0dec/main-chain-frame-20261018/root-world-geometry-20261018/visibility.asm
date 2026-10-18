140a2f2e0 55 push rbp
140a2f2e1 56 push rsi
140a2f2e2 57 push rdi
140a2f2e3 4881ec00050000 sub rsp, 0x500
140a2f2ea 488dac2480000000 lea rbp, [rsp + 0x80]
140a2f2f2 48c78578040000feffffff mov qword ptr [rbp + 0x478], 0xfffffffffffffffe
140a2f2fd 4c89c6 mov rsi, r8
140a2f300 48894c2430 mov qword ptr [rsp + 0x30], rcx
140a2f305 4889542428 mov qword ptr [rsp + 0x28], rdx
140a2f30a 48c744242023000000 mov qword ptr [rsp + 0x20], 0x23
140a2f313 488d15e6c40902 lea rdx, [rip + 0x209c4e6]
140a2f31a 4c8d0df0c40902 lea r9, [rip + 0x209c4f0]
140a2f321 488dbd50010000 lea rdi, [rbp + 0x150]
140a2f328 41b811000000 mov r8d, 0x11
140a2f32e 4889f9 mov rcx, rdi
140a2f331 e86a3fbd00 call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a2f336 488975c0 mov qword ptr [rbp - 0x40], rsi
140a2f33a c785e002000002000000 mov dword ptr [rbp + 0x2e0], 2
140a2f344 488d85e0020000 lea rax, [rbp + 0x2e0]
140a2f34b 4889442420 mov qword ptr [rsp + 0x20], rax
140a2f350 488d4dc0 lea rcx, [rbp - 0x40]
140a2f354 49b8fcfb49a09d576148 movabs r8, 0x4861579da049fbfc
140a2f35e 4889fa mov rdx, rdi
140a2f361 41b119 mov r9b, 0x19
140a2f364 e8d7eeebff call 0x1408ee240 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >
140a2f369 488dbde0020000 lea rdi, [rbp + 0x2e0]
140a2f370 488db550010000 lea rsi, [rbp + 0x150]
140a2f377 41b890010000 mov r8d, 0x190
140a2f37d 4889f9 mov rcx, rdi
140a2f380 4889f2 mov rdx, rsi
140a2f383 e87848ef01 call 0x142923c00 ; 
140a2f388 c645c009 mov byte ptr [rbp - 0x40], 9
140a2f38c c7855001000002000000 mov dword ptr [rbp + 0x150], 2
140a2f396 4889742420 mov qword ptr [rsp + 0x20], rsi
140a2f39b 488d75c0 lea rsi, [rbp - 0x40]
140a2f39f 49b8f2c570045e8bb061 movabs r8, 0x61b08b5e0470c5f2
140a2f3a9 4889f1 mov rcx, rsi
140a2f3ac 4889fa mov rdx, rdi
140a2f3af 41b119 mov r9b, 0x19
140a2f3b2 e8a95eedff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a2f3b7 488dbd50010000 lea rdi, [rbp + 0x150]
140a2f3be 488d95e0020000 lea rdx, [rbp + 0x2e0]
140a2f3c5 41b890010000 mov r8d, 0x190
140a2f3cb 4889f9 mov rcx, rdi
140a2f3ce e82d48ef01 call 0x142923c00 ; 
140a2f3d3 c685770400000a mov byte ptr [rbp + 0x477], 0xa
140a2f3da c745c002000000 mov dword ptr [rbp - 0x40], 2
140a2f3e1 4889742420 mov qword ptr [rsp + 0x20], rsi
140a2f3e6 488d8d77040000 lea rcx, [rbp + 0x477]
140a2f3ed 49b857f6e8da4c1e7d29 movabs r8, 0x297d1e4cdae8f657
140a2f3f7 4889fa mov rdx, rdi
140a2f3fa 41b119 mov r9b, 0x19
140a2f3fd e85e5eedff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a2f402 488d9550010000 lea rdx, [rbp + 0x150]
140a2f409 41b890010000 mov r8d, 0x190
140a2f40f 4889f1 mov rcx, rsi
140a2f412 e8e947ef01 call 0x142923c00 ; 
140a2f417 49b88983cdbdf97f0b7d movabs r8, 0x7d0b7ff9bdcd8389
140a2f421 4889f9 mov rcx, rdi
140a2f424 4889f2 mov rdx, rsi
140a2f427 e8a4d5bd00 call 0x14160c9d0 ; country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::systems::render_loop::resources::critters::FireflySsbo>
140a2f42c 488d8de0020000 lea rcx, [rbp + 0x2e0]
140a2f433 49b8f2f640016b87db66 movabs r8, 0x66db876b0140f6f2
140a2f43d 4889fa mov rdx, rdi
140a2f440 e8fb7bbd00 call 0x141607040 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::resources::critters::FireflyVisibilitySsbo>
140a2f445 0f108580030000 movups xmm0, xmmword ptr [rbp + 0x380]
140a2f44c 0f294500 movaps xmmword ptr [rbp], xmm0
140a2f450 0f108570030000 movups xmm0, xmmword ptr [rbp + 0x370]
140a2f457 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
140a2f45b 0f108560030000 movups xmm0, xmmword ptr [rbp + 0x360]
140a2f462 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
140a2f466 0f108550030000 movups xmm0, xmmword ptr [rbp + 0x350]
140a2f46d 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
140a2f471 0f108540030000 movups xmm0, xmmword ptr [rbp + 0x340]
140a2f478 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
140a2f47c 488b8518040000 mov rax, qword ptr [rbp + 0x418]
140a2f483 488b8d60040000 mov rcx, qword ptr [rbp + 0x460]
140a2f48a 488b95f0020000 mov rdx, qword ptr [rbp + 0x2f0]
140a2f491 48899560010000 mov qword ptr [rbp + 0x160], rdx
140a2f498 0f1085e0020000 movups xmm0, xmmword ptr [rbp + 0x2e0]
140a2f49f 0f298550010000 movaps xmmword ptr [rbp + 0x150], xmm0
140a2f4a6 0f1085f8020000 movups xmm0, xmmword ptr [rbp + 0x2f8]
140a2f4ad 0f298580010000 movaps xmmword ptr [rbp + 0x180], xmm0
140a2f4b4 488b9508030000 mov rdx, qword ptr [rbp + 0x308]
140a2f4bb 48899590010000 mov qword ptr [rbp + 0x190], rdx
140a2f4c2 0f108510030000 movups xmm0, xmmword ptr [rbp + 0x310]
140a2f4c9 0f118568010000 movups xmmword ptr [rbp + 0x168], xmm0
140a2f4d0 488b9520030000 mov rdx, qword ptr [rbp + 0x320]
140a2f4d7 48899578010000 mov qword ptr [rbp + 0x178], rdx
140a2f4de 0f108590030000 movups xmm0, xmmword ptr [rbp + 0x390]
140a2f4e5 0f108da0030000 movups xmm1, xmmword ptr [rbp + 0x3a0]
140a2f4ec 0f1095b0030000 movups xmm2, xmmword ptr [rbp + 0x3b0]
140a2f4f3 0f109dc0030000 movups xmm3, xmmword ptr [rbp + 0x3c0]
140a2f4fa 0f1185c8010000 movups xmmword ptr [rbp + 0x1c8], xmm0
140a2f501 0f118dd8010000 movups xmmword ptr [rbp + 0x1d8], xmm1
140a2f508 0f1195e8010000 movups xmmword ptr [rbp + 0x1e8], xmm2
140a2f50f 0f119df8010000 movups xmmword ptr [rbp + 0x1f8], xmm3
140a2f516 0f1085d0030000 movups xmm0, xmmword ptr [rbp + 0x3d0]
140a2f51d 0f118508020000 movups xmmword ptr [rbp + 0x208], xmm0
140a2f524 0f1085e0030000 movups xmm0, xmmword ptr [rbp + 0x3e0]
140a2f52b 0f118518020000 movups xmmword ptr [rbp + 0x218], xmm0
140a2f532 488b9538030000 mov rdx, qword ptr [rbp + 0x338]
140a2f539 488995a8010000 mov qword ptr [rbp + 0x1a8], rdx
140a2f540 0f108528030000 movups xmm0, xmmword ptr [rbp + 0x328]
140a2f547 0f118598010000 movups xmmword ptr [rbp + 0x198], xmm0
140a2f54e 48898d30020000 mov qword ptr [rbp + 0x230], rcx
140a2f555 48898528020000 mov qword ptr [rbp + 0x228], rax
140a2f55c 0f28059db70902 movaps xmm0, xmmword ptr [rip + 0x209b79d]
140a2f563 0f2985b0010000 movaps xmmword ptr [rbp + 0x1b0], xmm0
140a2f56a 488d4dc0 lea rcx, [rbp - 0x40]
140a2f56e 488d9550010000 lea rdx, [rbp + 0x150]
140a2f575 e856dffaff call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a2f57a 80bd1604000008 cmp byte ptr [rbp + 0x416], 8
140a2f581 740c je 0x140a2f58f
140a2f583 488d8d00040000 lea rcx, [rbp + 0x400]
140a2f58a e8212f1b00 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a2f58f 90 nop 
140a2f590 4881c400050000 add rsp, 0x500
140a2f597 5f pop rdi
140a2f598 5e pop rsi
140a2f599 5d pop rbp
140a2f59a c3 ret 
140a2f59b 0f1f440000 nop dword ptr [rax + rax]
140a2f5a0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a2f5a5 55 push rbp
140a2f5a6 56 push rsi
140a2f5a7 57 push rdi
140a2f5a8 4883ec40 sub rsp, 0x40
140a2f5ac 488daa80000000 lea rbp, [rdx + 0x80]
140a2f5b3 80bd1604000008 cmp byte ptr [rbp + 0x416], 8
140a2f5ba 740c je 0x140a2f5c8
140a2f5bc 488d8d00040000 lea rcx, [rbp + 0x400]
140a2f5c3 e8e82e1b00 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a2f5c8 90 nop 
140a2f5c9 4883c440 add rsp, 0x40
140a2f5cd 5f pop rdi
140a2f5ce 5e pop rsi
140a2f5cf 5d pop rbp
140a2f5d0 c3 ret 
140a2f5d1 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a2f5e0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a2f5e5 55 push rbp
140a2f5e6 56 push rsi
140a2f5e7 57 push rdi
140a2f5e8 4883ec40 sub rsp, 0x40
140a2f5ec 488daa80000000 lea rbp, [rdx + 0x80]
140a2f5f3 488d8d50010000 lea rcx, [rbp + 0x150]
140a2f5fa e8b179e6ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a2f5ff 90 nop 
140a2f600 4883c440 add rsp, 0x40
140a2f604 5f pop rdi
140a2f605 5e pop rsi
140a2f606 5d pop rbp
140a2f607 c3 ret 
140a2f608 0f1f840000000000 nop dword ptr [rax + rax]
140a2f610 4889542410 mov qword ptr [rsp + 0x10], rdx
140a2f615 55 push rbp
140a2f616 56 push rsi
140a2f617 57 push rdi
140a2f618 4883ec40 sub rsp, 0x40
140a2f61c 488daa80000000 lea rbp, [rdx + 0x80]
140a2f623 488d8de0020000 lea rcx, [rbp + 0x2e0]
140a2f62a e88179e6ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a2f62f 90 nop 
140a2f630 4883c440 add rsp, 0x40
140a2f634 5f pop rdi
140a2f635 5e pop rsi
140a2f636 5d pop rbp
140a2f637 c3 ret 
140a2f638 0f1f840000000000 nop dword ptr [rax + rax]
140a2f640 4889542410 mov qword ptr [rsp + 0x10], rdx
140a2f645 55 push rbp
140a2f646 56 push rsi
140a2f647 57 push rdi
140a2f648 4883ec40 sub rsp, 0x40
140a2f64c 488daa80000000 lea rbp, [rdx + 0x80]
140a2f653 488d8d50010000 lea rcx, [rbp + 0x150]
140a2f65a e85179e6ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a2f65f 90 nop 
140a2f660 4883c440 add rsp, 0x40
140a2f664 5f pop rdi
140a2f665 5e pop rsi
140a2f666 5d pop rbp
140a2f667 c3 ret 
