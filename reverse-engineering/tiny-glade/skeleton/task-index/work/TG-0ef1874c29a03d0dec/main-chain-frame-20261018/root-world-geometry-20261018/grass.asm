140a60280 55 push rbp
140a60281 4157 push r15
140a60283 4156 push r14
140a60285 4154 push r12
140a60287 56 push rsi
140a60288 57 push rdi
140a60289 53 push rbx
140a6028a 4881ec80060000 sub rsp, 0x680
140a60291 488dac2480000000 lea rbp, [rsp + 0x80]
140a60299 0f29bdf0050000 movaps xmmword ptr [rbp + 0x5f0], xmm7
140a602a0 0f29b5e0050000 movaps xmmword ptr [rbp + 0x5e0], xmm6
140a602a7 48c785d8050000feffffff mov qword ptr [rbp + 0x5d8], 0xfffffffffffffffe
140a602b2 4c89c3 mov rbx, r8
140a602b5 4889d6 mov rsi, rdx
140a602b8 4889cf mov rdi, rcx
140a602bb 4c8db540040000 lea r14, [rbp + 0x440]
140a602c2 ba00000100 mov edx, 0x10000
140a602c7 4c89f1 mov rcx, r14
140a602ca 41b820000000 mov r8d, 0x20
140a602d0 e84b4b1700 call 0x140bd4e20 ; rhapsody::raw::buffer::BufferDesc::new_gpu_only
140a602d5 4c89742420 mov qword ptr [rsp + 0x20], r14
140a602da 4c8d05173c0702 lea r8, [rip + 0x2073c17]
140a602e1 488d8db0020000 lea rcx, [rbp + 0x2b0]
140a602e8 41b911000000 mov r9d, 0x11
140a602ee 4889fa mov rdx, rdi
140a602f1 e81ac0f0ff call 0x14096c310 ; rhapsody::rg::temporal::impl$12::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >
140a602f6 80bdc802000002 cmp byte ptr [rbp + 0x2c8], 2
140a602fd 0f84ad170000 je 0x140a61ab0
140a60303 0f1085b0020000 movups xmm0, xmmword ptr [rbp + 0x2b0]
140a6030a 0f108dc0020000 movups xmm1, xmmword ptr [rbp + 0x2c0]
140a60311 0f298d50040000 movaps xmmword ptr [rbp + 0x450], xmm1
140a60318 0f298570020000 movaps xmmword ptr [rbp + 0x270], xmm0
140a6031f 488b8550040000 mov rax, qword ptr [rbp + 0x450]
140a60326 48898580020000 mov qword ptr [rbp + 0x280], rax
140a6032d 488d8da0000000 lea rcx, [rbp + 0xa0]
140a60334 ba00000100 mov edx, 0x10000
140a60339 4531c0 xor r8d, r8d
140a6033c e8df4a1700 call 0x140bd4e20 ; rhapsody::raw::buffer::BufferDesc::new_gpu_only
140a60341 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
140a60348 0f298540040000 movaps xmmword ptr [rbp + 0x440], xmm0
140a6034f 488d8dc0020000 lea rcx, [rbp + 0x2c0]
140a60356 488d9540040000 lea rdx, [rbp + 0x440]
140a6035d e89e691700 call 0x140bd6d00 ; rhapsody::rg::resource::impl$5::from
140a60362 488d05bf3b0702 lea rax, [rip + 0x2073bbf]
140a60369 488985b0020000 mov qword ptr [rbp + 0x2b0], rax
140a60370 48c785b802000013000000 mov qword ptr [rbp + 0x2b8], 0x13
140a6037b 488d95b0020000 lea rdx, [rbp + 0x2b0]
140a60382 4889f9 mov rcx, rdi
140a60385 e846911800 call 0x140be94d0 ; rhapsody::rg::graph::RenderGraph::create_raw_resource
140a6038a 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
140a60391 0f298580000000 movaps xmmword ptr [rbp + 0x80], xmm0
140a60398 898590000000 mov dword ptr [rbp + 0x90], eax
140a6039e 488d8da0000000 lea rcx, [rbp + 0xa0]
140a603a5 ba04040000 mov edx, 0x404
140a603aa 4531c0 xor r8d, r8d
140a603ad e86e4a1700 call 0x140bd4e20 ; rhapsody::raw::buffer::BufferDesc::new_gpu_only
140a603b2 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
140a603b9 0f298540040000 movaps xmmword ptr [rbp + 0x440], xmm0
140a603c0 488d8dc0020000 lea rcx, [rbp + 0x2c0]
140a603c7 488d9540040000 lea rdx, [rbp + 0x440]
140a603ce e82d691700 call 0x140bd6d00 ; rhapsody::rg::resource::impl$5::from
140a603d3 488d05613b0702 lea rax, [rip + 0x2073b61]
140a603da 488985b0020000 mov qword ptr [rbp + 0x2b0], rax
140a603e1 48c785b80200001c000000 mov qword ptr [rbp + 0x2b8], 0x1c
140a603ec 4c8db5b0020000 lea r14, [rbp + 0x2b0]
140a603f3 4889f9 mov rcx, rdi
140a603f6 4c89f2 mov rdx, r14
140a603f9 e8d2901800 call 0x140be94d0 ; rhapsody::rg::graph::RenderGraph::create_raw_resource
140a603fe 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
140a60405 0f298550020000 movaps xmmword ptr [rbp + 0x250], xmm0
140a6040c 898560020000 mov dword ptr [rbp + 0x260], eax
140a60412 48897c2430 mov qword ptr [rsp + 0x30], rdi
140a60417 4889742428 mov qword ptr [rsp + 0x28], rsi
140a6041c 48c744242019000000 mov qword ptr [rsp + 0x20], 0x19
140a60425 488d152b3b0702 lea rdx, [rip + 0x2073b2b]
140a6042c 4c8d0d343b0702 lea r9, [rip + 0x2073b34]
140a60433 4c8dbd40040000 lea r15, [rbp + 0x440]
140a6043a 41b810000000 mov r8d, 0x10
140a60440 4c89f9 mov rcx, r15
140a60443 e8582eba00 call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a60448 c685a000000009 mov byte ptr [rbp + 0xa0], 9
140a6044f c785b002000002000000 mov dword ptr [rbp + 0x2b0], 2
140a60459 4c89742420 mov qword ptr [rsp + 0x20], r14
140a6045e 4c8db5a0000000 lea r14, [rbp + 0xa0]
140a60465 49b8f2c570045e8bb061 movabs r8, 0x61b08b5e0470c5f2
140a6046f 4c89f1 mov rcx, r14
140a60472 4c89fa mov rdx, r15
140a60475 41b119 mov r9b, 0x19
140a60478 e8e34deaff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a6047d 4c8dbdb0020000 lea r15, [rbp + 0x2b0]
140a60484 488d9540040000 lea rdx, [rbp + 0x440]
140a6048b 41b890010000 mov r8d, 0x190
140a60491 4c89f9 mov rcx, r15
140a60494 e86737ec01 call 0x142923c00 ; 
140a60499 c645e00a mov byte ptr [rbp - 0x20], 0xa
140a6049d c785a000000002000000 mov dword ptr [rbp + 0xa0], 2
140a604a7 4c89742420 mov qword ptr [rsp + 0x20], r14
140a604ac 488d4de0 lea rcx, [rbp - 0x20]
140a604b0 49b857f6e8da4c1e7d29 movabs r8, 0x297d1e4cdae8f657
140a604ba 4c89fa mov rdx, r15
140a604bd 41b119 mov r9b, 0x19
140a604c0 e89b4deaff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a604c5 41b890010000 mov r8d, 0x190
140a604cb 4c89f1 mov rcx, r14
140a604ce 4c89fa mov rdx, r15
140a604d1 e82a37ec01 call 0x142923c00 ; 
140a604d6 488d4308 lea rax, [rbx + 8]
140a604da 48898540040000 mov qword ptr [rbp + 0x440], rax
140a604e1 c785b002000002000000 mov dword ptr [rbp + 0x2b0], 2
140a604eb 4c897c2420 mov qword ptr [rsp + 0x20], r15
140a604f0 488d8d40040000 lea rcx, [rbp + 0x440]
140a604f7 49b869a5a8892f81b662 movabs r8, 0x62b6812f89a8a569
140a60501 4c89f2 mov rdx, r14
140a60504 41b119 mov r9b, 0x19
140a60507 e834dde8ff call 0x1408ee240 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >
140a6050c 488d8db0020000 lea rcx, [rbp + 0x2b0]
140a60513 488d95a0000000 lea rdx, [rbp + 0xa0]
140a6051a 41b890010000 mov r8d, 0x190
140a60520 e8db36ec01 call 0x142923c00 ; 
140a60525 80bde603000008 cmp byte ptr [rbp + 0x3e6], 8
140a6052c 7425 je 0x140a60553
140a6052e 4c8b0b mov r9, qword ptr [rbx]
140a60531 488d8dc0030000 lea rcx, [rbp + 0x3c0]
140a60538 bae8000000 mov edx, 0xe8
140a6053d 480395e8030000 add rdx, qword ptr [rbp + 0x3e8]
140a60544 49b8d34103961336d2e8 movabs r8, 0xe8d23613960341d3
140a6054e e80db2f0ff call 0x14096b760 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<array$<u32,2> >,array$<u32,2> >
140a60553 488d8d40040000 lea rcx, [rbp + 0x440]
140a6055a 488d95b0020000 lea rdx, [rbp + 0x2b0]
140a60561 41b890010000 mov r8d, 0x190
140a60567 e89436ec01 call 0x142923c00 ; 
140a6056c 80bd7605000008 cmp byte ptr [rbp + 0x576], 8
140a60573 742a je 0x140a6059f
140a60575 440fb60d2348bd02 movzx r9d, byte ptr [rip + 0x2bd4823]
140a6057d 488d8d50050000 lea rcx, [rbp + 0x550]
140a60584 bae8000000 mov edx, 0xe8
140a60589 48039578050000 add rdx, qword ptr [rbp + 0x578]
140a60590 49b82bdac01e5607776d movabs r8, 0x6d7707561ec0da2b
140a6059a e8b1b2f0ff call 0x14096b850 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<bool>,bool>
140a6059f 488d9db0020000 lea rbx, [rbp + 0x2b0]
140a605a6 488d9540040000 lea rdx, [rbp + 0x440]
140a605ad 41b890010000 mov r8d, 0x190
140a605b3 4889d9 mov rcx, rbx
140a605b6 e84536ec01 call 0x142923c00 ; 
140a605bb 4c8db540040000 lea r14, [rbp + 0x440]
140a605c2 49b86cdda7d4e435b967 movabs r8, 0x67b935e4d4a7dd6c
140a605cc 4c8d8d70020000 lea r9, [rbp + 0x270]
140a605d3 4c89f1 mov rcx, r14
140a605d6 4889da mov rdx, rbx
140a605d9 e882d7f0ff call 0x14096dd60 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
140a605de 49b839ed461c8779608c movabs r8, 0x8c6079871c46ed39
140a605e8 4c8d8d80000000 lea r9, [rbp + 0x80]
140a605ef 4889d9 mov rcx, rbx
140a605f2 4c89f2 mov rdx, r14
140a605f5 e866d7f0ff call 0x14096dd60 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
140a605fa 488d8d40040000 lea rcx, [rbp + 0x440]
140a60601 49b8ba77a97a37e73e3f movabs r8, 0x3f3ee7377aa977ba
140a6060b 4c8d8d50020000 lea r9, [rbp + 0x250]
140a60612 4889da mov rdx, rbx
140a60615 e846d7f0ff call 0x14096dd60 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
140a6061a 0f1085e0040000 movups xmm0, xmmword ptr [rbp + 0x4e0]
140a60621 0f2985e0000000 movaps xmmword ptr [rbp + 0xe0], xmm0
140a60628 0f1085d0040000 movups xmm0, xmmword ptr [rbp + 0x4d0]
140a6062f 0f2985d0000000 movaps xmmword ptr [rbp + 0xd0], xmm0
140a60636 0f1085c0040000 movups xmm0, xmmword ptr [rbp + 0x4c0]
140a6063d 0f2985c0000000 movaps xmmword ptr [rbp + 0xc0], xmm0
140a60644 0f1085b0040000 movups xmm0, xmmword ptr [rbp + 0x4b0]
140a6064b 0f2985b0000000 movaps xmmword ptr [rbp + 0xb0], xmm0
140a60652 0f1085a0040000 movups xmm0, xmmword ptr [rbp + 0x4a0]
140a60659 0f2985a0000000 movaps xmmword ptr [rbp + 0xa0], xmm0
140a60660 488b8578050000 mov rax, qword ptr [rbp + 0x578]
140a60667 488b8dc0050000 mov rcx, qword ptr [rbp + 0x5c0]
140a6066e 488b9550040000 mov rdx, qword ptr [rbp + 0x450]
140a60675 488995c0020000 mov qword ptr [rbp + 0x2c0], rdx
140a6067c 0f108540040000 movups xmm0, xmmword ptr [rbp + 0x440]
140a60683 0f2985b0020000 movaps xmmword ptr [rbp + 0x2b0], xmm0
140a6068a 0f108558040000 movups xmm0, xmmword ptr [rbp + 0x458]
140a60691 0f2985e0020000 movaps xmmword ptr [rbp + 0x2e0], xmm0
140a60698 488b9568040000 mov rdx, qword ptr [rbp + 0x468]
140a6069f 488995f0020000 mov qword ptr [rbp + 0x2f0], rdx
140a606a6 0f108570040000 movups xmm0, xmmword ptr [rbp + 0x470]
140a606ad 0f1185c8020000 movups xmmword ptr [rbp + 0x2c8], xmm0
140a606b4 488b9580040000 mov rdx, qword ptr [rbp + 0x480]
140a606bb 488995d8020000 mov qword ptr [rbp + 0x2d8], rdx
140a606c2 0f1085f0040000 movups xmm0, xmmword ptr [rbp + 0x4f0]
140a606c9 0f108d00050000 movups xmm1, xmmword ptr [rbp + 0x500]
140a606d0 0f109510050000 movups xmm2, xmmword ptr [rbp + 0x510]
140a606d7 0f109d20050000 movups xmm3, xmmword ptr [rbp + 0x520]
140a606de 0f118528030000 movups xmmword ptr [rbp + 0x328], xmm0
140a606e5 0f118d38030000 movups xmmword ptr [rbp + 0x338], xmm1
140a606ec 0f119548030000 movups xmmword ptr [rbp + 0x348], xmm2
140a606f3 0f119d58030000 movups xmmword ptr [rbp + 0x358], xmm3
140a606fa 0f108530050000 movups xmm0, xmmword ptr [rbp + 0x530]
140a60701 0f118568030000 movups xmmword ptr [rbp + 0x368], xmm0
140a60708 0f108540050000 movups xmm0, xmmword ptr [rbp + 0x540]
140a6070f 0f118578030000 movups xmmword ptr [rbp + 0x378], xmm0
140a60716 488b9598040000 mov rdx, qword ptr [rbp + 0x498]
140a6071d 48899508030000 mov qword ptr [rbp + 0x308], rdx
140a60724 0f108588040000 movups xmm0, xmmword ptr [rbp + 0x488]
140a6072b 0f1185f8020000 movups xmmword ptr [rbp + 0x2f8], xmm0
140a60732 48898d90030000 mov qword ptr [rbp + 0x390], rcx
140a60739 48898588030000 mov qword ptr [rbp + 0x388], rax
140a60740 0f2835293e0602 movaps xmm6, xmmword ptr [rip + 0x2063e29]
140a60747 0f29b510030000 movaps xmmword ptr [rbp + 0x310], xmm6
140a6074e 488d8da0000000 lea rcx, [rbp + 0xa0]
140a60755 488d95b0020000 lea rdx, [rbp + 0x2b0]
140a6075c e86fcdf7ff call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a60761 80bd7605000008 cmp byte ptr [rbp + 0x576], 8
140a60768 740c je 0x140a60776
140a6076a 488d8d60050000 lea rcx, [rbp + 0x560]
140a60771 e83a1d1800 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a60776 48897c2430 mov qword ptr [rsp + 0x30], rdi
140a6077b 4889742428 mov qword ptr [rsp + 0x28], rsi
140a60780 48c74424201b000000 mov qword ptr [rsp + 0x20], 0x1b
140a60789 488d15f0370702 lea rdx, [rip + 0x20737f0]
140a60790 4c8d0dfb370702 lea r9, [rip + 0x20737fb]
140a60797 41b812000000 mov r8d, 0x12
140a6079d 4889d9 mov rcx, rbx
140a607a0 e8fb2aba00 call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a607a5 488d8d40040000 lea rcx, [rbp + 0x440]
140a607ac 49b8ba77a97a37e73e3f movabs r8, 0x3f3ee7377aa977ba
140a607b6 4c8d8d50020000 lea r9, [rbp + 0x250]
140a607bd 4889da mov rdx, rbx
140a607c0 e89bd5f0ff call 0x14096dd60 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
140a607c5 0f1085e0040000 movups xmm0, xmmword ptr [rbp + 0x4e0]
140a607cc 0f2985e0000000 movaps xmmword ptr [rbp + 0xe0], xmm0
140a607d3 0f1085d0040000 movups xmm0, xmmword ptr [rbp + 0x4d0]
140a607da 0f2985d0000000 movaps xmmword ptr [rbp + 0xd0], xmm0
140a607e1 0f1085c0040000 movups xmm0, xmmword ptr [rbp + 0x4c0]
140a607e8 0f2985c0000000 movaps xmmword ptr [rbp + 0xc0], xmm0
140a607ef 0f1085b0040000 movups xmm0, xmmword ptr [rbp + 0x4b0]
140a607f6 0f2985b0000000 movaps xmmword ptr [rbp + 0xb0], xmm0
140a607fd 0f1085a0040000 movups xmm0, xmmword ptr [rbp + 0x4a0]
140a60804 0f2985a0000000 movaps xmmword ptr [rbp + 0xa0], xmm0
140a6080b 488b8578050000 mov rax, qword ptr [rbp + 0x578]
140a60812 488b8dc0050000 mov rcx, qword ptr [rbp + 0x5c0]
140a60819 488b9550040000 mov rdx, qword ptr [rbp + 0x450]
140a60820 488995c0020000 mov qword ptr [rbp + 0x2c0], rdx
140a60827 0f108540040000 movups xmm0, xmmword ptr [rbp + 0x440]
140a6082e 0f2985b0020000 movaps xmmword ptr [rbp + 0x2b0], xmm0
140a60835 0f108558040000 movups xmm0, xmmword ptr [rbp + 0x458]
140a6083c 0f2985e0020000 movaps xmmword ptr [rbp + 0x2e0], xmm0
140a60843 488b9568040000 mov rdx, qword ptr [rbp + 0x468]
140a6084a 488995f0020000 mov qword ptr [rbp + 0x2f0], rdx
140a60851 0f108570040000 movups xmm0, xmmword ptr [rbp + 0x470]
140a60858 0f1185c8020000 movups xmmword ptr [rbp + 0x2c8], xmm0
140a6085f 488b9580040000 mov rdx, qword ptr [rbp + 0x480]
140a60866 488995d8020000 mov qword ptr [rbp + 0x2d8], rdx
140a6086d 0f1085f0040000 movups xmm0, xmmword ptr [rbp + 0x4f0]
140a60874 0f108d00050000 movups xmm1, xmmword ptr [rbp + 0x500]
140a6087b 0f109510050000 movups xmm2, xmmword ptr [rbp + 0x510]
140a60882 0f109d20050000 movups xmm3, xmmword ptr [rbp + 0x520]
140a60889 0f118528030000 movups xmmword ptr [rbp + 0x328], xmm0
140a60890 0f118d38030000 movups xmmword ptr [rbp + 0x338], xmm1
140a60897 0f119548030000 movups xmmword ptr [rbp + 0x348], xmm2
140a6089e 0f119d58030000 movups xmmword ptr [rbp + 0x358], xmm3
140a608a5 0f108530050000 movups xmm0, xmmword ptr [rbp + 0x530]
140a608ac 0f118568030000 movups xmmword ptr [rbp + 0x368], xmm0
140a608b3 0f108540050000 movups xmm0, xmmword ptr [rbp + 0x540]
140a608ba 0f118578030000 movups xmmword ptr [rbp + 0x378], xmm0
140a608c1 488b9598040000 mov rdx, qword ptr [rbp + 0x498]
140a608c8 48899508030000 mov qword ptr [rbp + 0x308], rdx
140a608cf 0f108588040000 movups xmm0, xmmword ptr [rbp + 0x488]
140a608d6 0f1185f8020000 movups xmmword ptr [rbp + 0x2f8], xmm0
140a608dd 48898d90030000 mov qword ptr [rbp + 0x390], rcx
140a608e4 48898588030000 mov qword ptr [rbp + 0x388], rax
140a608eb 0f283d4e2d0702 movaps xmm7, xmmword ptr [rip + 0x2072d4e]
140a608f2 0f29bd10030000 movaps xmmword ptr [rbp + 0x310], xmm7
140a608f9 488d8da0000000 lea rcx, [rbp + 0xa0]
140a60900 488d95b0020000 lea rdx, [rbp + 0x2b0]
140a60907 e8c4cbf7ff call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a6090c 80bd7605000008 cmp byte ptr [rbp + 0x576], 8
140a60913 740c je 0x140a60921
140a60915 488d8d60050000 lea rcx, [rbp + 0x560]
140a6091c e88f1b1800 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a60921 488d8da0000000 lea rcx, [rbp + 0xa0]
140a60928 ba00000400 mov edx, 0x40000
140a6092d 4531c0 xor r8d, r8d
140a60930 e8eb441700 call 0x140bd4e20 ; rhapsody::raw::buffer::BufferDesc::new_gpu_only
140a60935 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
140a6093c 0f298540040000 movaps xmmword ptr [rbp + 0x440], xmm0
140a60943 488d8dc0020000 lea rcx, [rbp + 0x2c0]
140a6094a 488d9540040000 lea rdx, [rbp + 0x440]
140a60951 e8aa631700 call 0x140bd6d00 ; rhapsody::rg::resource::impl$5::from
140a60956 488d0550360702 lea rax, [rip + 0x2073650]
140a6095d 488985b0020000 mov qword ptr [rbp + 0x2b0], rax
140a60964 48c785b802000019000000 mov qword ptr [rbp + 0x2b8], 0x19
140a6096f 488d95b0020000 lea rdx, [rbp + 0x2b0]
140a60976 4889f9 mov rcx, rdi
140a60979 e8528b1800 call 0x140be94d0 ; rhapsody::rg::graph::RenderGraph::create_raw_resource
140a6097e 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
140a60985 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
140a60989 8945d0 mov dword ptr [rbp - 0x30], eax
140a6098c 488d8da0000000 lea rcx, [rbp + 0xa0]
140a60993 ba04040000 mov edx, 0x404
140a60998 4531c0 xor r8d, r8d
140a6099b e880441700 call 0x140bd4e20 ; rhapsody::raw::buffer::BufferDesc::new_gpu_only
140a609a0 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
140a609a7 0f298540040000 movaps xmmword ptr [rbp + 0x440], xmm0
140a609ae 488d8dc0020000 lea rcx, [rbp + 0x2c0]
140a609b5 488d9540040000 lea rdx, [rbp + 0x440]
140a609bc e83f631700 call 0x140bd6d00 ; rhapsody::rg::resource::impl$5::from
140a609c1 488d05fe350702 lea rax, [rip + 0x20735fe]
140a609c8 488985b0020000 mov qword ptr [rbp + 0x2b0], rax
140a609cf 48c785b80200001b000000 mov qword ptr [rbp + 0x2b8], 0x1b
140a609da 488d95b0020000 lea rdx, [rbp + 0x2b0]
140a609e1 4889f9 mov rcx, rdi
140a609e4 e8e78a1800 call 0x140be94d0 ; rhapsody::rg::graph::RenderGraph::create_raw_resource
140a609e9 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
140a609f0 0f298530020000 movaps xmmword ptr [rbp + 0x230], xmm0
140a609f7 898540020000 mov dword ptr [rbp + 0x240], eax
140a609fd 48897c2430 mov qword ptr [rsp + 0x30], rdi
140a60a02 4889742428 mov qword ptr [rsp + 0x28], rsi
140a60a07 48c744242020000000 mov qword ptr [rsp + 0x20], 0x20
140a60a10 488d15ca350702 lea rdx, [rip + 0x20735ca]
140a60a17 4c8d0dda350702 lea r9, [rip + 0x20735da]
140a60a1e 41b817000000 mov r8d, 0x17
140a60a24 4889d9 mov rcx, rbx
140a60a27 e87428ba00 call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a60a2c 49b87eb7c0cce8c9c23f movabs r8, 0x3fc2c9e8ccc0b77e
140a60a36 4c8d4dc0 lea r9, [rbp - 0x40]
140a60a3a 4c89f1 mov rcx, r14
140a60a3d 4889da mov rdx, rbx
140a60a40 e81bd3f0ff call 0x14096dd60 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
140a60a45 49b8def8f6116688d373 movabs r8, 0x73d3886611f6f8de
140a60a4f 4c8d8d30020000 lea r9, [rbp + 0x230]
140a60a56 4889d9 mov rcx, rbx
140a60a59 4c89f2 mov rdx, r14
140a60a5c e8ffd2f0ff call 0x14096dd60 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
140a60a61 49b86cdda7d4e435b967 movabs r8, 0x67b935e4d4a7dd6c
140a60a6b 4c8d8d70020000 lea r9, [rbp + 0x270]
140a60a72 4c89f1 mov rcx, r14
140a60a75 4889da mov rdx, rbx
140a60a78 e843d0f0ff call 0x14096dac0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
140a60a7d 49b839ed461c8779608c movabs r8, 0x8c6079871c46ed39
140a60a87 4c8d8d80000000 lea r9, [rbp + 0x80]
140a60a8e 4889d9 mov rcx, rbx
140a60a91 4c89f2 mov rdx, r14
140a60a94 e827d0f0ff call 0x14096dac0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
140a60a99 488d8d40040000 lea rcx, [rbp + 0x440]
140a60aa0 49b8ba77a97a37e73e3f movabs r8, 0x3f3ee7377aa977ba
140a60aaa 4c8d8d50020000 lea r9, [rbp + 0x250]
140a60ab1 4889da mov rdx, rbx
140a60ab4 e807d0f0ff call 0x14096dac0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
140a60ab9 0f1085e0040000 movups xmm0, xmmword ptr [rbp + 0x4e0]
140a60ac0 0f2985e0000000 movaps xmmword ptr [rbp + 0xe0], xmm0
140a60ac7 0f1085d0040000 movups xmm0, xmmword ptr [rbp + 0x4d0]
140a60ace 0f2985d0000000 movaps xmmword ptr [rbp + 0xd0], xmm0
140a60ad5 0f1085c0040000 movups xmm0, xmmword ptr [rbp + 0x4c0]
140a60adc 0f2985c0000000 movaps xmmword ptr [rbp + 0xc0], xmm0
140a60ae3 0f1085b0040000 movups xmm0, xmmword ptr [rbp + 0x4b0]
140a60aea 0f2985b0000000 movaps xmmword ptr [rbp + 0xb0], xmm0
140a60af1 0f1085a0040000 movups xmm0, xmmword ptr [rbp + 0x4a0]
140a60af8 0f2985a0000000 movaps xmmword ptr [rbp + 0xa0], xmm0
140a60aff 488b8578050000 mov rax, qword ptr [rbp + 0x578]
140a60b06 488b8dc0050000 mov rcx, qword ptr [rbp + 0x5c0]
140a60b0d 488b9550040000 mov rdx, qword ptr [rbp + 0x450]
140a60b14 488995c0020000 mov qword ptr [rbp + 0x2c0], rdx
140a60b1b 0f108540040000 movups xmm0, xmmword ptr [rbp + 0x440]
140a60b22 0f2985b0020000 movaps xmmword ptr [rbp + 0x2b0], xmm0
140a60b29 0f108558040000 movups xmm0, xmmword ptr [rbp + 0x458]
140a60b30 0f2985e0020000 movaps xmmword ptr [rbp + 0x2e0], xmm0
140a60b37 488b9568040000 mov rdx, qword ptr [rbp + 0x468]
140a60b3e 488995f0020000 mov qword ptr [rbp + 0x2f0], rdx
140a60b45 0f108570040000 movups xmm0, xmmword ptr [rbp + 0x470]
140a60b4c 0f1185c8020000 movups xmmword ptr [rbp + 0x2c8], xmm0
140a60b53 488b9580040000 mov rdx, qword ptr [rbp + 0x480]
140a60b5a 488995d8020000 mov qword ptr [rbp + 0x2d8], rdx
140a60b61 0f1085f0040000 movups xmm0, xmmword ptr [rbp + 0x4f0]
140a60b68 0f108d00050000 movups xmm1, xmmword ptr [rbp + 0x500]
140a60b6f 0f109510050000 movups xmm2, xmmword ptr [rbp + 0x510]
140a60b76 0f109d20050000 movups xmm3, xmmword ptr [rbp + 0x520]
140a60b7d 0f118528030000 movups xmmword ptr [rbp + 0x328], xmm0
140a60b84 0f118d38030000 movups xmmword ptr [rbp + 0x338], xmm1
140a60b8b 0f119548030000 movups xmmword ptr [rbp + 0x348], xmm2
140a60b92 0f119d58030000 movups xmmword ptr [rbp + 0x358], xmm3
140a60b99 0f108530050000 movups xmm0, xmmword ptr [rbp + 0x530]
140a60ba0 0f118568030000 movups xmmword ptr [rbp + 0x368], xmm0
140a60ba7 0f108540050000 movups xmm0, xmmword ptr [rbp + 0x540]
140a60bae 0f118578030000 movups xmmword ptr [rbp + 0x378], xmm0
140a60bb5 488b9598040000 mov rdx, qword ptr [rbp + 0x498]
140a60bbc 48899508030000 mov qword ptr [rbp + 0x308], rdx
140a60bc3 0f108588040000 movups xmm0, xmmword ptr [rbp + 0x488]
140a60bca 0f1185f8020000 movups xmmword ptr [rbp + 0x2f8], xmm0
140a60bd1 48898d90030000 mov qword ptr [rbp + 0x390], rcx
140a60bd8 48898588030000 mov qword ptr [rbp + 0x388], rax
140a60bdf 0f29b510030000 movaps xmmword ptr [rbp + 0x310], xmm6
140a60be6 488d8da0000000 lea rcx, [rbp + 0xa0]
140a60bed 488d95b0020000 lea rdx, [rbp + 0x2b0]
140a60bf4 e8d7c8f7ff call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a60bf9 80bd7605000008 cmp byte ptr [rbp + 0x576], 8
140a60c00 740c je 0x140a60c0e
140a60c02 488d8d60050000 lea rcx, [rbp + 0x560]
140a60c09 e8a2181800 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a60c0e 48897c2430 mov qword ptr [rsp + 0x30], rdi
140a60c13 4889742428 mov qword ptr [rsp + 0x28], rsi
140a60c18 48c74424201d000000 mov qword ptr [rsp + 0x20], 0x1d
140a60c21 488d15f0330702 lea rdx, [rip + 0x20733f0]
140a60c28 4c8d0dfd330702 lea r9, [rip + 0x20733fd]
140a60c2f 41b814000000 mov r8d, 0x14
140a60c35 4889d9 mov rcx, rbx
140a60c38 e86326ba00 call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a60c3d 488d8d40040000 lea rcx, [rbp + 0x440]
140a60c44 49b8def8f6116688d373 movabs r8, 0x73d3886611f6f8de
140a60c4e 4c8d8d30020000 lea r9, [rbp + 0x230]
140a60c55 4889da mov rdx, rbx
140a60c58 e803d1f0ff call 0x14096dd60 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
140a60c5d 0f1085e0040000 movups xmm0, xmmword ptr [rbp + 0x4e0]
140a60c64 0f2985e0000000 movaps xmmword ptr [rbp + 0xe0], xmm0
140a60c6b 0f1085d0040000 movups xmm0, xmmword ptr [rbp + 0x4d0]
140a60c72 0f2985d0000000 movaps xmmword ptr [rbp + 0xd0], xmm0
140a60c79 0f1085c0040000 movups xmm0, xmmword ptr [rbp + 0x4c0]
140a60c80 0f2985c0000000 movaps xmmword ptr [rbp + 0xc0], xmm0
140a60c87 0f1085b0040000 movups xmm0, xmmword ptr [rbp + 0x4b0]
140a60c8e 0f2985b0000000 movaps xmmword ptr [rbp + 0xb0], xmm0
140a60c95 0f1085a0040000 movups xmm0, xmmword ptr [rbp + 0x4a0]
140a60c9c 0f2985a0000000 movaps xmmword ptr [rbp + 0xa0], xmm0
140a60ca3 488b8578050000 mov rax, qword ptr [rbp + 0x578]
140a60caa 488b8dc0050000 mov rcx, qword ptr [rbp + 0x5c0]
140a60cb1 488b9550040000 mov rdx, qword ptr [rbp + 0x450]
140a60cb8 488995c0020000 mov qword ptr [rbp + 0x2c0], rdx
140a60cbf 0f108540040000 movups xmm0, xmmword ptr [rbp + 0x440]
140a60cc6 0f2985b0020000 movaps xmmword ptr [rbp + 0x2b0], xmm0
140a60ccd 0f108558040000 movups xmm0, xmmword ptr [rbp + 0x458]
140a60cd4 0f2985e0020000 movaps xmmword ptr [rbp + 0x2e0], xmm0
140a60cdb 488b9568040000 mov rdx, qword ptr [rbp + 0x468]
140a60ce2 488995f0020000 mov qword ptr [rbp + 0x2f0], rdx
140a60ce9 0f108570040000 movups xmm0, xmmword ptr [rbp + 0x470]
140a60cf0 0f1185c8020000 movups xmmword ptr [rbp + 0x2c8], xmm0
140a60cf7 488b9580040000 mov rdx, qword ptr [rbp + 0x480]
140a60cfe 488995d8020000 mov qword ptr [rbp + 0x2d8], rdx
140a60d05 0f1085f0040000 movups xmm0, xmmword ptr [rbp + 0x4f0]
140a60d0c 0f108d00050000 movups xmm1, xmmword ptr [rbp + 0x500]
140a60d13 0f109510050000 movups xmm2, xmmword ptr [rbp + 0x510]
140a60d1a 0f109d20050000 movups xmm3, xmmword ptr [rbp + 0x520]
140a60d21 0f118528030000 movups xmmword ptr [rbp + 0x328], xmm0
140a60d28 0f118d38030000 movups xmmword ptr [rbp + 0x338], xmm1
140a60d2f 0f119548030000 movups xmmword ptr [rbp + 0x348], xmm2
140a60d36 0f119d58030000 movups xmmword ptr [rbp + 0x358], xmm3
140a60d3d 0f108530050000 movups xmm0, xmmword ptr [rbp + 0x530]
140a60d44 0f118568030000 movups xmmword ptr [rbp + 0x368], xmm0
140a60d4b 0f108540050000 movups xmm0, xmmword ptr [rbp + 0x540]
140a60d52 0f118578030000 movups xmmword ptr [rbp + 0x378], xmm0
140a60d59 488b9598040000 mov rdx, qword ptr [rbp + 0x498]
140a60d60 48899508030000 mov qword ptr [rbp + 0x308], rdx
140a60d67 0f108588040000 movups xmm0, xmmword ptr [rbp + 0x488]
140a60d6e 0f1185f8020000 movups xmmword ptr [rbp + 0x2f8], xmm0
140a60d75 48898d90030000 mov qword ptr [rbp + 0x390], rcx
140a60d7c 48898588030000 mov qword ptr [rbp + 0x388], rax
140a60d83 0f29bd10030000 movaps xmmword ptr [rbp + 0x310], xmm7
140a60d8a 488d8da0000000 lea rcx, [rbp + 0xa0]
140a60d91 488d95b0020000 lea rdx, [rbp + 0x2b0]
140a60d98 e833c7f7ff call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a60d9d 80bd7605000008 cmp byte ptr [rbp + 0x576], 8
140a60da4 740c je 0x140a60db2
140a60da6 488d8d60050000 lea rcx, [rbp + 0x560]
140a60dad e8fe161800 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a60db2 488d8da0000000 lea rcx, [rbp + 0xa0]
140a60db9 ba00000800 mov edx, 0x80000
140a60dbe 4531c0 xor r8d, r8d
140a60dc1 e85a401700 call 0x140bd4e20 ; rhapsody::raw::buffer::BufferDesc::new_gpu_only
140a60dc6 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
140a60dcd 0f298540040000 movaps xmmword ptr [rbp + 0x440], xmm0
140a60dd4 488d8dc0020000 lea rcx, [rbp + 0x2c0]
140a60ddb 488d9540040000 lea rdx, [rbp + 0x440]
140a60de2 e8195f1700 call 0x140bd6d00 ; rhapsody::rg::resource::impl$5::from
140a60de7 488d055b320702 lea rax, [rip + 0x207325b]
140a60dee 488985b0020000 mov qword ptr [rbp + 0x2b0], rax
140a60df5 48c785b80200000d000000 mov qword ptr [rbp + 0x2b8], 0xd
140a60e00 488d95b0020000 lea rdx, [rbp + 0x2b0]
140a60e07 4889f9 mov rcx, rdi
140a60e0a e8c1861800 call 0x140be94d0 ; rhapsody::rg::graph::RenderGraph::create_raw_resource
140a60e0f 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
140a60e16 0f294560 movaps xmmword ptr [rbp + 0x60], xmm0
140a60e1a 894570 mov dword ptr [rbp + 0x70], eax
140a60e1d 488d8da0000000 lea rcx, [rbp + 0xa0]
140a60e24 ba00000800 mov edx, 0x80000
140a60e29 4531c0 xor r8d, r8d
140a60e2c e8ef3f1700 call 0x140bd4e20 ; rhapsody::raw::buffer::BufferDesc::new_gpu_only
140a60e31 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
140a60e38 0f298540040000 movaps xmmword ptr [rbp + 0x440], xmm0
140a60e3f 488d8dc0020000 lea rcx, [rbp + 0x2c0]
140a60e46 488d9540040000 lea rdx, [rbp + 0x440]
140a60e4d e8ae5e1700 call 0x140bd6d00 ; rhapsody::rg::resource::impl$5::from
140a60e52 488d05fd310702 lea rax, [rip + 0x20731fd]
140a60e59 488985b0020000 mov qword ptr [rbp + 0x2b0], rax
140a60e60 48c785b80200001c000000 mov qword ptr [rbp + 0x2b8], 0x1c
140a60e6b 488d95b0020000 lea rdx, [rbp + 0x2b0]
140a60e72 4889f9 mov rcx, rdi
140a60e75 e856861800 call 0x140be94d0 ; rhapsody::rg::graph::RenderGraph::create_raw_resource
140a60e7a 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
140a60e81 0f294540 movaps xmmword ptr [rbp + 0x40], xmm0
140a60e85 894550 mov dword ptr [rbp + 0x50], eax
140a60e88 48897c2430 mov qword ptr [rsp + 0x30], rdi
140a60e8d 4889742428 mov qword ptr [rsp + 0x28], rsi
140a60e92 48c74424201e000000 mov qword ptr [rsp + 0x20], 0x1e
140a60e9b 488d15d0310702 lea rdx, [rip + 0x20731d0]
140a60ea2 4c8d0dde310702 lea r9, [rip + 0x20731de]
140a60ea9 41b815000000 mov r8d, 0x15
140a60eaf 4889d9 mov rcx, rbx
140a60eb2 e8e923ba00 call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a60eb7 49b8387df3bf0b7d4449 movabs r8, 0x49447d0bbff37d38
140a60ec1 4c8d4d60 lea r9, [rbp + 0x60]
140a60ec5 4c89f1 mov rcx, r14
140a60ec8 4889da mov rdx, rbx
140a60ecb e890cef0ff call 0x14096dd60 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
140a60ed0 49b8d9166bdc67d5b314 movabs r8, 0x14b3d567dc6b16d9
140a60eda 4c8d4d40 lea r9, [rbp + 0x40]
140a60ede 4889d9 mov rcx, rbx
140a60ee1 4c89f2 mov rdx, r14
140a60ee4 e877cef0ff call 0x14096dd60 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
140a60ee9 49b87eb7c0cce8c9c23f movabs r8, 0x3fc2c9e8ccc0b77e
140a60ef3 4c8d4dc0 lea r9, [rbp - 0x40]
140a60ef7 4c89f1 mov rcx, r14
140a60efa 4889da mov rdx, rbx
140a60efd e8becbf0ff call 0x14096dac0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
140a60f02 49b8def8f6116688d373 movabs r8, 0x73d3886611f6f8de
140a60f0c 4c8d8d30020000 lea r9, [rbp + 0x230]
140a60f13 4889d9 mov rcx, rbx
140a60f16 4c89f2 mov rdx, r14
140a60f19 e8a2cbf0ff call 0x14096dac0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
140a60f1e 49b86cdda7d4e435b967 movabs r8, 0x67b935e4d4a7dd6c
140a60f28 4c8d8d70020000 lea r9, [rbp + 0x270]
140a60f2f 4c89f1 mov rcx, r14
140a60f32 4889da mov rdx, rbx
140a60f35 e886cbf0ff call 0x14096dac0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
140a60f3a 49b839ed461c8779608c movabs r8, 0x8c6079871c46ed39
140a60f44 4c8d8d80000000 lea r9, [rbp + 0x80]
140a60f4b 4889d9 mov rcx, rbx
140a60f4e 4c89f2 mov rdx, r14
140a60f51 e86acbf0ff call 0x14096dac0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
140a60f56 488d8d40040000 lea rcx, [rbp + 0x440]
140a60f5d 49b8ba77a97a37e73e3f movabs r8, 0x3f3ee7377aa977ba
140a60f67 4c8d8d50020000 lea r9, [rbp + 0x250]
140a60f6e 4889da mov rdx, rbx
140a60f71 e84acbf0ff call 0x14096dac0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
140a60f76 0f1085e0040000 movups xmm0, xmmword ptr [rbp + 0x4e0]
140a60f7d 0f2985e0000000 movaps xmmword ptr [rbp + 0xe0], xmm0
140a60f84 0f1085d0040000 movups xmm0, xmmword ptr [rbp + 0x4d0]
140a60f8b 0f2985d0000000 movaps xmmword ptr [rbp + 0xd0], xmm0
140a60f92 0f1085c0040000 movups xmm0, xmmword ptr [rbp + 0x4c0]
140a60f99 0f2985c0000000 movaps xmmword ptr [rbp + 0xc0], xmm0
140a60fa0 0f1085b0040000 movups xmm0, xmmword ptr [rbp + 0x4b0]
140a60fa7 0f2985b0000000 movaps xmmword ptr [rbp + 0xb0], xmm0
140a60fae 0f1085a0040000 movups xmm0, xmmword ptr [rbp + 0x4a0]
140a60fb5 0f2985a0000000 movaps xmmword ptr [rbp + 0xa0], xmm0
140a60fbc 488b8578050000 mov rax, qword ptr [rbp + 0x578]
140a60fc3 488b8dc0050000 mov rcx, qword ptr [rbp + 0x5c0]
140a60fca 488b9550040000 mov rdx, qword ptr [rbp + 0x450]
140a60fd1 488995c0020000 mov qword ptr [rbp + 0x2c0], rdx
140a60fd8 0f108540040000 movups xmm0, xmmword ptr [rbp + 0x440]
140a60fdf 0f2985b0020000 movaps xmmword ptr [rbp + 0x2b0], xmm0
140a60fe6 0f108558040000 movups xmm0, xmmword ptr [rbp + 0x458]
140a60fed 0f2985e0020000 movaps xmmword ptr [rbp + 0x2e0], xmm0
140a60ff4 488b9568040000 mov rdx, qword ptr [rbp + 0x468]
140a60ffb 488995f0020000 mov qword ptr [rbp + 0x2f0], rdx
140a61002 0f108570040000 movups xmm0, xmmword ptr [rbp + 0x470]
140a61009 0f1185c8020000 movups xmmword ptr [rbp + 0x2c8], xmm0
140a61010 488b9580040000 mov rdx, qword ptr [rbp + 0x480]
140a61017 488995d8020000 mov qword ptr [rbp + 0x2d8], rdx
140a6101e 0f1085f0040000 movups xmm0, xmmword ptr [rbp + 0x4f0]
140a61025 0f108d00050000 movups xmm1, xmmword ptr [rbp + 0x500]
140a6102c 0f109510050000 movups xmm2, xmmword ptr [rbp + 0x510]
140a61033 0f109d20050000 movups xmm3, xmmword ptr [rbp + 0x520]
140a6103a 0f118528030000 movups xmmword ptr [rbp + 0x328], xmm0
140a61041 0f118d38030000 movups xmmword ptr [rbp + 0x338], xmm1
140a61048 0f119548030000 movups xmmword ptr [rbp + 0x348], xmm2
140a6104f 0f119d58030000 movups xmmword ptr [rbp + 0x358], xmm3
140a61056 0f108530050000 movups xmm0, xmmword ptr [rbp + 0x530]
140a6105d 0f118568030000 movups xmmword ptr [rbp + 0x368], xmm0
140a61064 0f108540050000 movups xmm0, xmmword ptr [rbp + 0x540]
140a6106b 0f118578030000 movups xmmword ptr [rbp + 0x378], xmm0
140a61072 488b9598040000 mov rdx, qword ptr [rbp + 0x498]
140a61079 48899508030000 mov qword ptr [rbp + 0x308], rdx
140a61080 0f108588040000 movups xmm0, xmmword ptr [rbp + 0x488]
140a61087 0f1185f8020000 movups xmmword ptr [rbp + 0x2f8], xmm0
140a6108e 48898d90030000 mov qword ptr [rbp + 0x390], rcx
140a61095 48898588030000 mov qword ptr [rbp + 0x388], rax
140a6109c 0f29b510030000 movaps xmmword ptr [rbp + 0x310], xmm6
140a610a3 488d8da0000000 lea rcx, [rbp + 0xa0]
140a610aa 488d95b0020000 lea rdx, [rbp + 0x2b0]
140a610b1 e81ac4f7ff call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a610b6 80bd7605000008 cmp byte ptr [rbp + 0x576], 8
140a610bd 740c je 0x140a610cb
140a610bf 488d8d60050000 lea rcx, [rbp + 0x560]
140a610c6 e8e5131800 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a610cb 488d5d40 lea rbx, [rbp + 0x40]
140a610cf 4889f9 mov rcx, rdi
140a610d2 4889f2 mov rdx, rsi
140a610d5 4989d8 mov r8, rbx
140a610d8 e883aef2ff call 0x14098bf60 ; country_core::systems::render_loop::compute_passes::prefix_scan::inclusive_prefix_scan_u32_1m
140a610dd 488d8da0000000 lea rcx, [rbp + 0xa0]
140a610e4 ba20000000 mov edx, 0x20
140a610e9 4531c0 xor r8d, r8d
140a610ec e82f3d1700 call 0x140bd4e20 ; rhapsody::raw::buffer::BufferDesc::new_gpu_only
140a610f1 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
140a610f8 0f298540040000 movaps xmmword ptr [rbp + 0x440], xmm0
140a610ff 488d8dc0020000 lea rcx, [rbp + 0x2c0]
140a61106 488d9540040000 lea rdx, [rbp + 0x440]
140a6110d e8ee5b1700 call 0x140bd6d00 ; rhapsody::rg::resource::impl$5::from
140a61112 488d058c2f0702 lea rax, [rip + 0x2072f8c]
140a61119 488985b0020000 mov qword ptr [rbp + 0x2b0], rax
140a61120 48c785b80200001a000000 mov qword ptr [rbp + 0x2b8], 0x1a
140a6112b 488d95b0020000 lea rdx, [rbp + 0x2b0]
140a61132 4889f9 mov rcx, rdi
140a61135 e896831800 call 0x140be94d0 ; rhapsody::rg::graph::RenderGraph::create_raw_resource
140a6113a 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
140a61141 0f298590020000 movaps xmmword ptr [rbp + 0x290], xmm0
140a61148 8985a0020000 mov dword ptr [rbp + 0x2a0], eax
140a6114e 48897c2430 mov qword ptr [rsp + 0x30], rdi
140a61153 4889742428 mov qword ptr [rsp + 0x28], rsi
140a61158 48c744242021000000 mov qword ptr [rsp + 0x20], 0x21
140a61161 488d15572f0702 lea rdx, [rip + 0x2072f57]
140a61168 4c8d0d632f0702 lea r9, [rip + 0x2072f63]
140a6116f 4c8db5b0020000 lea r14, [rbp + 0x2b0]
140a61176 41b813000000 mov r8d, 0x13
140a6117c 4c89f1 mov rcx, r14
140a6117f e81c21ba00 call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a61184 4c8dbd40040000 lea r15, [rbp + 0x440]
140a6118b 49b8870cb32dabf5bada movabs r8, 0xdabaf5ab2db30c87
140a61195 4c8d8d90020000 lea r9, [rbp + 0x290]
140a6119c 4c89f9 mov rcx, r15
140a6119f 4c89f2 mov rdx, r14
140a611a2 e8b9cbf0ff call 0x14096dd60 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
140a611a7 49b8d9166bdc67d5b314 movabs r8, 0x14b3d567dc6b16d9
140a611b1 4c89f1 mov rcx, r14
140a611b4 4c89fa mov rdx, r15
140a611b7 4989d9 mov r9, rbx
140a611ba e801c9f0ff call 0x14096dac0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
140a611bf 488d8d40040000 lea rcx, [rbp + 0x440]
140a611c6 49b8def8f6116688d373 movabs r8, 0x73d3886611f6f8de
140a611d0 4c8d8d30020000 lea r9, [rbp + 0x230]
140a611d7 4c89f2 mov rdx, r14
140a611da e8e1c8f0ff call 0x14096dac0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
140a611df 0f1085e0040000 movups xmm0, xmmword ptr [rbp + 0x4e0]
140a611e6 0f2985e0000000 movaps xmmword ptr [rbp + 0xe0], xmm0
140a611ed 0f1085d0040000 movups xmm0, xmmword ptr [rbp + 0x4d0]
140a611f4 0f2985d0000000 movaps xmmword ptr [rbp + 0xd0], xmm0
140a611fb 0f1085c0040000 movups xmm0, xmmword ptr [rbp + 0x4c0]
140a61202 0f2985c0000000 movaps xmmword ptr [rbp + 0xc0], xmm0
140a61209 0f1085b0040000 movups xmm0, xmmword ptr [rbp + 0x4b0]
140a61210 0f2985b0000000 movaps xmmword ptr [rbp + 0xb0], xmm0
140a61217 0f1085a0040000 movups xmm0, xmmword ptr [rbp + 0x4a0]
140a6121e 0f2985a0000000 movaps xmmword ptr [rbp + 0xa0], xmm0
140a61225 488b8578050000 mov rax, qword ptr [rbp + 0x578]
140a6122c 488b8dc0050000 mov rcx, qword ptr [rbp + 0x5c0]
140a61233 488b9550040000 mov rdx, qword ptr [rbp + 0x450]
140a6123a 488995c0020000 mov qword ptr [rbp + 0x2c0], rdx
140a61241 0f108540040000 movups xmm0, xmmword ptr [rbp + 0x440]
140a61248 0f2985b0020000 movaps xmmword ptr [rbp + 0x2b0], xmm0
140a6124f 0f108558040000 movups xmm0, xmmword ptr [rbp + 0x458]
140a61256 0f2985e0020000 movaps xmmword ptr [rbp + 0x2e0], xmm0
140a6125d 488b9568040000 mov rdx, qword ptr [rbp + 0x468]
140a61264 488995f0020000 mov qword ptr [rbp + 0x2f0], rdx
140a6126b 0f108570040000 movups xmm0, xmmword ptr [rbp + 0x470]
140a61272 0f1185c8020000 movups xmmword ptr [rbp + 0x2c8], xmm0
140a61279 488b9580040000 mov rdx, qword ptr [rbp + 0x480]
140a61280 488995d8020000 mov qword ptr [rbp + 0x2d8], rdx
140a61287 0f1085f0040000 movups xmm0, xmmword ptr [rbp + 0x4f0]
140a6128e 0f108d00050000 movups xmm1, xmmword ptr [rbp + 0x500]
140a61295 0f109510050000 movups xmm2, xmmword ptr [rbp + 0x510]
140a6129c 0f109d20050000 movups xmm3, xmmword ptr [rbp + 0x520]
140a612a3 0f118528030000 movups xmmword ptr [rbp + 0x328], xmm0
140a612aa 0f118d38030000 movups xmmword ptr [rbp + 0x338], xmm1
140a612b1 0f119548030000 movups xmmword ptr [rbp + 0x348], xmm2
140a612b8 0f119d58030000 movups xmmword ptr [rbp + 0x358], xmm3
140a612bf 0f108530050000 movups xmm0, xmmword ptr [rbp + 0x530]
140a612c6 0f118568030000 movups xmmword ptr [rbp + 0x368], xmm0
140a612cd 0f108540050000 movups xmm0, xmmword ptr [rbp + 0x540]
140a612d4 0f118578030000 movups xmmword ptr [rbp + 0x378], xmm0
140a612db 488b9598040000 mov rdx, qword ptr [rbp + 0x498]
140a612e2 48899508030000 mov qword ptr [rbp + 0x308], rdx
140a612e9 0f108588040000 movups xmm0, xmmword ptr [rbp + 0x488]
140a612f0 0f1185f8020000 movups xmmword ptr [rbp + 0x2f8], xmm0
140a612f7 48898d90030000 mov qword ptr [rbp + 0x390], rcx
140a612fe 48898588030000 mov qword ptr [rbp + 0x388], rax
140a61305 0f2805d47cef01 movaps xmm0, xmmword ptr [rip + 0x1ef7cd4]
140a6130c 0f298510030000 movaps xmmword ptr [rbp + 0x310], xmm0
140a61313 488d8da0000000 lea rcx, [rbp + 0xa0]
140a6131a 488d95b0020000 lea rdx, [rbp + 0x2b0]
140a61321 e8aac1f7ff call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a61326 80bd7605000008 cmp byte ptr [rbp + 0x576], 8
140a6132d 740c je 0x140a6133b
140a6132f 488d8d60050000 lea rcx, [rbp + 0x560]
140a61336 e875111800 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a6133b 488d4de0 lea rcx, [rbp - 0x20]
140a6133f ba00000400 mov edx, 0x40000
140a61344 4531c0 xor r8d, r8d
140a61347 e8d43a1700 call 0x140bd4e20 ; rhapsody::raw::buffer::BufferDesc::new_gpu_only
140a6134c 0f1045e0 movups xmm0, xmmword ptr [rbp - 0x20]
140a61350 0f298540040000 movaps xmmword ptr [rbp + 0x440], xmm0
140a61357 488d8dc0020000 lea rcx, [rbp + 0x2c0]
140a6135e 488d9540040000 lea rdx, [rbp + 0x440]
140a61365 e896591700 call 0x140bd6d00 ; rhapsody::rg::resource::impl$5::from
140a6136a 488d05822d0702 lea rax, [rip + 0x2072d82]
140a61371 488985b0020000 mov qword ptr [rbp + 0x2b0], rax
140a61378 48c785b802000016000000 mov qword ptr [rbp + 0x2b8], 0x16
140a61383 488d95b0020000 lea rdx, [rbp + 0x2b0]
140a6138a 4889f9 mov rcx, rdi
140a6138d e83e811800 call 0x140be94d0 ; rhapsody::rg::graph::RenderGraph::create_raw_resource
140a61392 0f1045e0 movups xmm0, xmmword ptr [rbp - 0x20]
140a61396 0f2985a0000000 movaps xmmword ptr [rbp + 0xa0], xmm0
140a6139d 8985b0000000 mov dword ptr [rbp + 0xb0], eax
140a613a3 4c8db5a0000000 lea r14, [rbp + 0xa0]
140a613aa 4889f9 mov rcx, rdi
140a613ad 4889f2 mov rdx, rsi
140a613b0 4d89f0 mov r8, r14
140a613b3 e8f8c7f7ff call 0x1409ddbb0 ; country_core::systems::render_loop::clear_rg_buffer
140a613b8 48897c2430 mov qword ptr [rsp + 0x30], rdi
140a613bd 4889742428 mov qword ptr [rsp + 0x28], rsi
140a613c2 48c74424202a000000 mov qword ptr [rsp + 0x20], 0x2a
140a613cb 488d15372d0702 lea rdx, [rip + 0x2072d37]
140a613d2 4c8d0d432d0702 lea r9, [rip + 0x2072d43]
140a613d9 4c8dbdb0020000 lea r15, [rbp + 0x2b0]
140a613e0 41b813000000 mov r8d, 0x13
140a613e6 4c89f9 mov rcx, r15
140a613e9 e8b21eba00 call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a613ee 4c8da540040000 lea r12, [rbp + 0x440]
140a613f5 48bb52532876d5a90d1a movabs rbx, 0x1a0da9d576285352
140a613ff 4c89e1 mov rcx, r12
140a61402 4c89fa mov rdx, r15
140a61405 4989d8 mov r8, rbx
140a61408 4d89f1 mov r9, r14
140a6140b e850c9f0ff call 0x14096dd60 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_rg_storage_buffer
140a61410 4c8db5b0020000 lea r14, [rbp + 0x2b0]
140a61417 49b8870cb32dabf5bada movabs r8, 0xdabaf5ab2db30c87
140a61421 4c8dbd90020000 lea r15, [rbp + 0x290]
140a61428 4c89f1 mov rcx, r14
140a6142b 4c89e2 mov rdx, r12
140a6142e 4d89f9 mov r9, r15
140a61431 e88ac6f0ff call 0x14096dac0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
140a61436 49b8d9166bdc67d5b314 movabs r8, 0x14b3d567dc6b16d9
140a61440 4c8d4d40 lea r9, [rbp + 0x40]
140a61444 4c89e1 mov rcx, r12
140a61447 4c89f2 mov rdx, r14
140a6144a e871c6f0ff call 0x14096dac0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
140a6144f 41b810000000 mov r8d, 0x10
140a61455 4c89e1 mov rcx, r12
140a61458 4c89fa mov rdx, r15
140a6145b e820d4f0ff call 0x14096e880 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_indirect_raw
140a61460 0f288570020000 movaps xmm0, xmmword ptr [rbp + 0x270]
140a61467 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
140a6146b 488b8580020000 mov rax, qword ptr [rbp + 0x280]
140a61472 488945f0 mov qword ptr [rbp - 0x10], rax
140a61476 4c8d75f8 lea r14, [rbp - 8]
140a6147a 0f2885a0000000 movaps xmm0, xmmword ptr [rbp + 0xa0]
140a61481 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
140a61485 488b85b0000000 mov rax, qword ptr [rbp + 0xb0]
140a6148c 48894508 mov qword ptr [rbp + 8], rax
140a61490 0f288590020000 movaps xmm0, xmmword ptr [rbp + 0x290]
140a61497 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
140a6149b 488b85a0020000 mov rax, qword ptr [rbp + 0x2a0]
140a614a2 48894520 mov qword ptr [rbp + 0x20], rax
140a614a6 4c8d7d28 lea r15, [rbp + 0x28]
140a614aa 0f284560 movaps xmm0, xmmword ptr [rbp + 0x60]
140a614ae 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
140a614b2 488b4570 mov rax, qword ptr [rbp + 0x70]
140a614b6 48894538 mov qword ptr [rbp + 0x38], rax
140a614ba 48897c2430 mov qword ptr [rsp + 0x30], rdi
140a614bf 4889742428 mov qword ptr [rsp + 0x28], rsi
140a614c4 48c744242017000000 mov qword ptr [rsp + 0x20], 0x17
140a614cd 488d15ac290702 lea rdx, [rip + 0x20729ac]
140a614d4 4c8d0daa290702 lea r9, [rip + 0x20729aa]
140a614db 488dbdb0020000 lea rdi, [rbp + 0x2b0]
140a614e2 41b805000000 mov r8d, 5
140a614e8 4889f9 mov rcx, rdi
140a614eb e8b01dba00 call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a614f0 49b86cdda7d4e435b967 movabs r8, 0x67b935e4d4a7dd6c
140a614fa 4c8d4de0 lea r9, [rbp - 0x20]
140a614fe 4c89e1 mov rcx, r12
140a61501 4889fa mov rdx, rdi
140a61504 e8b7c5f0ff call 0x14096dac0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
140a61509 49b8387df3bf0b7d4449 movabs r8, 0x49447d0bbff37d38
140a61513 4889f9 mov rcx, rdi
140a61516 4c89e2 mov rdx, r12
140a61519 4d89f9 mov r9, r15
140a6151c e89fc5f0ff call 0x14096dac0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
140a61521 4c8dbd40040000 lea r15, [rbp + 0x440]
140a61528 4c89f9 mov rcx, r15
140a6152b 4889fa mov rdx, rdi
140a6152e 4989d8 mov r8, rbx
140a61531 4d89f1 mov r9, r14
140a61534 e887c5f0ff call 0x14096dac0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::read_rg_storage_buffer
140a61539 488dbda0000000 lea rdi, [rbp + 0xa0]
140a61540 49b8f340c85b0968019d movabs r8, 0x9d0168095bc840f3
140a6154a 4889f9 mov rcx, rdi
140a6154d 4c89fa mov rdx, r15
140a61550 e82b45ba00 call 0x141605a80 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::grass::GrassBladeDataSsbo>
140a61555 488d8db0020000 lea rcx, [rbp + 0x2b0]
140a6155c 49b85a7612802b7ac0f1 movabs r8, 0xf1c07a2b8012765a
140a61566 4889fa mov rdx, rdi
140a61569 41b901000000 mov r9d, 1
140a6156f e89c76ba00 call 0x141608c10 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>
140a61574 80bde603000008 cmp byte ptr [rbp + 0x3e6], 8
140a6157b 7425 je 0x140a615a2
140a6157d 488d8dc0030000 lea rcx, [rbp + 0x3c0]
140a61584 bae8000000 mov edx, 0xe8
140a61589 480395e8030000 add rdx, qword ptr [rbp + 0x3e8]
140a61590 49b88957b716a611950e movabs r8, 0xe9511a616b75789
140a6159a 4531c9 xor r9d, r9d
140a6159d e8ce9ff0ff call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
140a615a2 488dbd40040000 lea rdi, [rbp + 0x440]
140a615a9 488d95b0020000 lea rdx, [rbp + 0x2b0]
140a615b0 41b890010000 mov r8d, 0x190
140a615b6 4889f9 mov rcx, rdi
140a615b9 e84226ec01 call 0x142923c00 ; 
140a615be 488d8db0020000 lea rcx, [rbp + 0x2b0]
140a615c5 49b89282cd9c47ed3269 movabs r8, 0x6932ed479ccd8292
140a615cf 4889fa mov rdx, rdi
140a615d2 e88932ba00 call 0x141604860 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::indirect_params::DrawIndirectBuffer>
140a615d7 80bde603000008 cmp byte ptr [rbp + 0x3e6], 8
140a615de 7425 je 0x140a61605
140a615e0 488d8dc0030000 lea rcx, [rbp + 0x3c0]
140a615e7 bae8000000 mov edx, 0xe8
140a615ec 480395e8030000 add rdx, qword ptr [rbp + 0x3e8]
140a615f3 49b84cadab6382085739 movabs r8, 0x3957088263abad4c
140a615fd 4531c9 xor r9d, r9d
140a61600 e86b9ff0ff call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
140a61605 488d95b0020000 lea rdx, [rbp + 0x2b0]
140a6160c 41b890010000 mov r8d, 0x190
140a61612 4889f9 mov rcx, rdi
140a61615 e8e625ec01 call 0x142923c00 ; 
140a6161a 488d8db0020000 lea rcx, [rbp + 0x2b0]
140a61621 49b8092945ea31c7cd83 movabs r8, 0x83cdc731ea452909
140a6162b 4889fa mov rdx, rdi
140a6162e e8ad40ba00 call 0x1416056e0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::indirect_params::AtomicCounterBuffer>
140a61633 80bde603000008 cmp byte ptr [rbp + 0x3e6], 8
140a6163a 7428 je 0x140a61664
140a6163c 488d8dc0030000 lea rcx, [rbp + 0x3c0]
140a61643 bae8000000 mov edx, 0xe8
140a61648 480395e8030000 add rdx, qword ptr [rbp + 0x3e8]
140a6164f 49b894169ffdc74b6eb0 movabs r8, 0xb06e4bc7fd9f1694
140a61659 41b900000900 mov r9d, 0x90000
140a6165f e80c9ff0ff call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
140a61664 488d8d40040000 lea rcx, [rbp + 0x440]
140a6166b 488d95b0020000 lea rdx, [rbp + 0x2b0]
140a61672 41b890010000 mov r8d, 0x190
140a61678 e88325ec01 call 0x142923c00 ; 
140a6167d 488d4e38 lea rcx, [rsi + 0x38]
140a61681 c685d705000001 mov byte ptr [rbp + 0x5d7], 1
140a61688 48ba28641ac941e9b8c2 movabs rdx, 0xc2b8e941c91a6428
140a61692 49b8a043dad3d1c6f359 movabs r8, 0x59f3c6d1d3da43a0
140a6169c e84fc067ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
140a616a1 4885c0 test rax, rax
140a616a4 0f84f1030000 je 0x140a61a9b
140a616aa 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140a616b1 0f86e4030000 jbe 0x140a61a9b
140a616b7 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140a616be 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
140a616c2 4885c9 test rcx, rcx
140a616c5 0f84d0030000 je 0x140a61a9b
140a616cb 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
140a616d2 48f7d1 not rcx
140a616d5 488d0c49 lea rcx, [rcx + rcx*2]
140a616d9 48c1e105 shl rcx, 5
140a616dd 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
140a616e3 0f84b2030000 je 0x140a61a9b
140a616e9 4801c8 add rax, rcx
140a616ec 488b4010 mov rax, qword ptr [rax + 0x10]
140a616f0 8b08 mov ecx, dword ptr [rax]
140a616f2 8d41fa lea eax, [rcx - 6]
140a616f5 4883c1f9 add rcx, -7
140a616f9 83f80f cmp eax, 0xf
140a616fc 48c7c0feffffff mov rax, 0xfffffffffffffffe
140a61703 480f42c1 cmovb rax, rcx
140a61707 4531c9 xor r9d, r9d
140a6170a 4883f802 cmp rax, 2
140a6170e 770b ja 0x140a6171b
140a61710 488d0dc1310702 lea rcx, [rip + 0x20731c1]
140a61717 448b0c81 mov r9d, dword ptr [rcx + rax*4]
140a6171b 80bd7605000008 cmp byte ptr [rbp + 0x576], 8
140a61722 7422 je 0x140a61746
140a61724 488d8d50050000 lea rcx, [rbp + 0x550]
140a6172b bae8000000 mov edx, 0xe8
140a61730 48039578050000 add rdx, qword ptr [rbp + 0x578]
140a61737 49b8405a9da3f5c8dada movabs r8, 0xdadac8f5a39d5a40
140a61741 e82a9ef0ff call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
140a61746 488db5b0020000 lea rsi, [rbp + 0x2b0]
140a6174d 41b890010000 mov r8d, 0x190
140a61753 4889f1 mov rcx, rsi
140a61756 4889fa mov rdx, rdi
140a61759 e8a224ec01 call 0x142923c00 ; 
140a6175e c685a00000000e mov byte ptr [rbp + 0xa0], 0xe
140a61765 c7854004000002000000 mov dword ptr [rbp + 0x440], 2
140a6176f 48897c2420 mov qword ptr [rsp + 0x20], rdi
140a61774 488d8da0000000 lea rcx, [rbp + 0xa0]
140a6177b 49b8b9d21286b398e04e movabs r8, 0x4ee098b38612d2b9
140a61785 4889f2 mov rdx, rsi
140a61788 41b119 mov r9b, 0x19
140a6178b e8d03aeaff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a61790 488db540040000 lea rsi, [rbp + 0x440]
140a61797 488dbdb0020000 lea rdi, [rbp + 0x2b0]
140a6179e 41b890010000 mov r8d, 0x190
140a617a4 4889f1 mov rcx, rsi
140a617a7 4889fa mov rdx, rdi
140a617aa e85124ec01 call 0x142923c00 ; 
140a617af c685a000000013 mov byte ptr [rbp + 0xa0], 0x13
140a617b6 c785b002000002000000 mov dword ptr [rbp + 0x2b0], 2
140a617c0 48897c2420 mov qword ptr [rsp + 0x20], rdi
140a617c5 488d8da0000000 lea rcx, [rbp + 0xa0]
140a617cc 49b841f0c818cd3cd66a movabs r8, 0x6ad63ccd18c8f041
140a617d6 4889f2 mov rdx, rsi
140a617d9 41b119 mov r9b, 0x19
140a617dc e87f3aeaff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a617e1 488db5b0020000 lea rsi, [rbp + 0x2b0]
140a617e8 488dbd40040000 lea rdi, [rbp + 0x440]
140a617ef 41b890010000 mov r8d, 0x190
140a617f5 4889f1 mov rcx, rsi
140a617f8 4889fa mov rdx, rdi
140a617fb e80024ec01 call 0x142923c00 ; 
140a61800 c685a000000015 mov byte ptr [rbp + 0xa0], 0x15
140a61807 c7854004000002000000 mov dword ptr [rbp + 0x440], 2
140a61811 48897c2420 mov qword ptr [rsp + 0x20], rdi
140a61816 488d8da0000000 lea rcx, [rbp + 0xa0]
140a6181d 49b813f0f0efb3ec9a4a movabs r8, 0x4a9aecb3eff0f013
140a61827 4889f2 mov rdx, rsi
140a6182a 41b119 mov r9b, 0x19
140a6182d e82e3aeaff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a61832 41b890010000 mov r8d, 0x190
140a61838 4889f9 mov rcx, rdi
140a6183b 4889f2 mov rdx, rsi
140a6183e e8bd23ec01 call 0x142923c00 ; 
140a61843 c6859002000010 mov byte ptr [rbp + 0x290], 0x10
140a6184a c785b002000002000000 mov dword ptr [rbp + 0x2b0], 2
140a61854 4889742420 mov qword ptr [rsp + 0x20], rsi
140a61859 488d8d90020000 lea rcx, [rbp + 0x290]
140a61860 49b84ce305ca55ab7e0d movabs r8, 0xd7eab55ca05e34c
140a6186a 4889fa mov rdx, rdi
140a6186d 41b119 mov r9b, 0x19
140a61870 e8eb39eaff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a61875 488dbda0000000 lea rdi, [rbp + 0xa0]
140a6187c 488d9540040000 lea rdx, [rbp + 0x440]
140a61883 41b890010000 mov r8d, 0x190
140a61889 4889f9 mov rcx, rdi
140a6188c e86f23ec01 call 0x142923c00 ; 
140a61891 c6854004000009 mov byte ptr [rbp + 0x440], 9
140a61898 c785b002000002000000 mov dword ptr [rbp + 0x2b0], 2
140a618a2 4889742420 mov qword ptr [rsp + 0x20], rsi
140a618a7 488db540040000 lea rsi, [rbp + 0x440]
140a618ae 49b8f2c570045e8bb061 movabs r8, 0x61b08b5e0470c5f2
140a618b8 4889f1 mov rcx, rsi
140a618bb 4889fa mov rdx, rdi
140a618be 41b119 mov r9b, 0x19
140a618c1 e89a39eaff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a618c6 488dbdb0020000 lea rdi, [rbp + 0x2b0]
140a618cd 488d95a0000000 lea rdx, [rbp + 0xa0]
140a618d4 41b890010000 mov r8d, 0x190
140a618da 4889f9 mov rcx, rdi
140a618dd e81e23ec01 call 0x142923c00 ; 
140a618e2 c685900200000a mov byte ptr [rbp + 0x290], 0xa
140a618e9 c7854004000002000000 mov dword ptr [rbp + 0x440], 2
140a618f3 4889742420 mov qword ptr [rsp + 0x20], rsi
140a618f8 488d8d90020000 lea rcx, [rbp + 0x290]
140a618ff 49b857f6e8da4c1e7d29 movabs r8, 0x297d1e4cdae8f657
140a61909 4889fa mov rdx, rdi
140a6190c 41b119 mov r9b, 0x19
140a6190f e84c39eaff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a61914 41b890010000 mov r8d, 0x190
140a6191a 4889f1 mov rcx, rsi
140a6191d 4889fa mov rdx, rdi
140a61920 e8db22ec01 call 0x142923c00 ; 
140a61925 c685a000000005 mov byte ptr [rbp + 0xa0], 5
140a6192c c785b002000002000000 mov dword ptr [rbp + 0x2b0], 2
140a61936 48897c2420 mov qword ptr [rsp + 0x20], rdi
140a6193b 488d8da0000000 lea rcx, [rbp + 0xa0]
140a61942 49b81f02c46dff2f50ed movabs r8, 0xed502fff6dc4021f
140a6194c 4889f2 mov rdx, rsi
140a6194f 41b119 mov r9b, 0x19
140a61952 e80939eaff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a61957 488db5b0020000 lea rsi, [rbp + 0x2b0]
140a6195e 488dbd40040000 lea rdi, [rbp + 0x440]
140a61965 41b890010000 mov r8d, 0x190
140a6196b 4889f1 mov rcx, rsi
140a6196e 4889fa mov rdx, rdi
140a61971 e88a22ec01 call 0x142923c00 ; 
140a61976 c685a000000014 mov byte ptr [rbp + 0xa0], 0x14
140a6197d c7854004000002000000 mov dword ptr [rbp + 0x440], 2
140a61987 48897c2420 mov qword ptr [rsp + 0x20], rdi
140a6198c 488d8da0000000 lea rcx, [rbp + 0xa0]
140a61993 49b88d738378483af449 movabs r8, 0x49f43a487883738d
140a6199d 4889f2 mov rdx, rsi
140a619a0 41b119 mov r9b, 0x19
140a619a3 e8b838eaff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a619a8 488db540040000 lea rsi, [rbp + 0x440]
140a619af 488dbdb0020000 lea rdi, [rbp + 0x2b0]
140a619b6 41b890010000 mov r8d, 0x190
140a619bc 4889f1 mov rcx, rsi
140a619bf 4889fa mov rdx, rdi
140a619c2 e83922ec01 call 0x142923c00 ; 
140a619c7 c685a00000000f mov byte ptr [rbp + 0xa0], 0xf
140a619ce c785b002000002000000 mov dword ptr [rbp + 0x2b0], 2
140a619d8 48897c2420 mov qword ptr [rsp + 0x20], rdi
140a619dd 488d8da0000000 lea rcx, [rbp + 0xa0]
140a619e4 49b8e613ab6db9805203 movabs r8, 0x35280b96dab13e6
140a619ee 4889f2 mov rdx, rsi
140a619f1 41b119 mov r9b, 0x19
140a619f4 e86738eaff call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a619f9 488db5b0020000 lea rsi, [rbp + 0x2b0]
140a61a00 488dbd40040000 lea rdi, [rbp + 0x440]
140a61a07 41b890010000 mov r8d, 0x190
140a61a0d 4889f1 mov rcx, rsi
140a61a10 4889fa mov rdx, rdi
140a61a13 e8e821ec01 call 0x142923c00 ; 
140a61a18 48b8c58a049ab3bd9189 movabs rax, 0x8991bdb39a048ac5
140a61a22 488985a0000000 mov qword ptr [rbp + 0xa0], rax
140a61a29 c7854004000002000000 mov dword ptr [rbp + 0x440], 2
140a61a33 48897c2420 mov qword ptr [rsp + 0x20], rdi
140a61a38 488d8da0000000 lea rcx, [rbp + 0xa0]
140a61a3f 49b8503587d03550725b movabs r8, 0x5b725035d0873550
140a61a49 4889f2 mov rdx, rsi
140a61a4c 41b119 mov r9b, 0x19
140a61a4f e87c3ceaff call 0x1409056d0 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TextureFromDisk>
140a61a54 488d7510 lea rsi, [rbp + 0x10]
140a61a58 488d95b0020000 lea rdx, [rbp + 0x2b0]
140a61a5f 41b890010000 mov r8d, 0x190
140a61a65 4889f9 mov rcx, rdi
140a61a68 e89321ec01 call 0x142923c00 ; 
140a61a6d 4889f9 mov rcx, rdi
140a61a70 4889f2 mov rdx, rsi
140a61a73 4531c0 xor r8d, r8d
140a61a76 e805cef0ff call 0x14096e880 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_indirect_raw
140a61a7b 0f28b5e0050000 movaps xmm6, xmmword ptr [rbp + 0x5e0]
140a61a82 0f28bdf0050000 movaps xmm7, xmmword ptr [rbp + 0x5f0]
140a61a89 4881c480060000 add rsp, 0x680
140a61a90 5b pop rbx
140a61a91 5f pop rdi
140a61a92 5e pop rsi
140a61a93 415c pop r12
140a61a95 415e pop r14
140a61a97 415f pop r15
140a61a99 5d pop rbp
140a61a9a c3 ret 
140a61a9b c685d705000001 mov byte ptr [rbp + 0x5d7], 1
140a61aa2 488d0d37240702 lea rcx, [rip + 0x2072437]
140a61aa9 e86278e701 call 0x1428d9310 ; core::option::unwrap_failed
140a61aae eb39 jmp 0x140a61ae9
140a61ab0 488b85b0020000 mov rax, qword ptr [rbp + 0x2b0]
140a61ab7 48898540040000 mov qword ptr [rbp + 0x440], rax
140a61abe 488d054b240702 lea rax, [rip + 0x207244b]
140a61ac5 4889442420 mov qword ptr [rsp + 0x20], rax
140a61aca 488d0d2f200702 lea rcx, [rip + 0x207202f]
140a61ad1 4c8d0d08200702 lea r9, [rip + 0x2072008]
140a61ad8 4c8d8540040000 lea r8, [rbp + 0x440]
140a61adf ba2b000000 mov edx, 0x2b
140a61ae4 e8777ce701 call 0x1428d9760 ; core::result::unwrap_failed
140a61ae9 0f0b ud2 
140a61aeb 0f1f440000 nop dword ptr [rax + rax]
140a61af0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a61af5 55 push rbp
140a61af6 4157 push r15
140a61af8 4156 push r14
140a61afa 4154 push r12
140a61afc 56 push rsi
140a61afd 57 push rdi
140a61afe 53 push rbx
140a61aff 4883ec60 sub rsp, 0x60
140a61b03 488daa80000000 lea rbp, [rdx + 0x80]
140a61b0a 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a61b0f 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a61b14 488d8d40040000 lea rcx, [rbp + 0x440]
140a61b1b e89054e3ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a61b20 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a61b25 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a61b2a 4883c460 add rsp, 0x60
140a61b2e 5b pop rbx
140a61b2f 5f pop rdi
140a61b30 5e pop rsi
140a61b31 415c pop r12
140a61b33 415e pop r14
140a61b35 415f pop r15
140a61b37 5d pop rbp
140a61b38 c3 ret 
140a61b39 0f1f8000000000 nop dword ptr [rax]
140a61b40 4889542410 mov qword ptr [rsp + 0x10], rdx
140a61b45 55 push rbp
140a61b46 4157 push r15
140a61b48 4156 push r14
140a61b4a 4154 push r12
140a61b4c 56 push rsi
140a61b4d 57 push rdi
140a61b4e 53 push rbx
140a61b4f 4883ec60 sub rsp, 0x60
140a61b53 488daa80000000 lea rbp, [rdx + 0x80]
140a61b5a 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a61b5f 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a61b64 80bd7605000008 cmp byte ptr [rbp + 0x576], 8
140a61b6b 740c je 0x140a61b79
140a61b6d 488d8d60050000 lea rcx, [rbp + 0x560]
140a61b74 e837091800 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a61b79 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a61b7e 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a61b83 4883c460 add rsp, 0x60
140a61b87 5b pop rbx
140a61b88 5f pop rdi
140a61b89 5e pop rsi
140a61b8a 415c pop r12
140a61b8c 415e pop r14
140a61b8e 415f pop r15
140a61b90 5d pop rbp
140a61b91 c3 ret 
140a61b92 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a61ba0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a61ba5 55 push rbp
140a61ba6 4157 push r15
140a61ba8 4156 push r14
140a61baa 4154 push r12
140a61bac 56 push rsi
140a61bad 57 push rdi
140a61bae 53 push rbx
140a61baf 4883ec60 sub rsp, 0x60
140a61bb3 488daa80000000 lea rbp, [rdx + 0x80]
140a61bba 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a61bbf 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a61bc4 80bd7605000008 cmp byte ptr [rbp + 0x576], 8
140a61bcb 740c je 0x140a61bd9
140a61bcd 488d8d60050000 lea rcx, [rbp + 0x560]
140a61bd4 e8d7081800 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a61bd9 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a61bde 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a61be3 4883c460 add rsp, 0x60
140a61be7 5b pop rbx
140a61be8 5f pop rdi
140a61be9 5e pop rsi
140a61bea 415c pop r12
140a61bec 415e pop r14
140a61bee 415f pop r15
140a61bf0 5d pop rbp
140a61bf1 c3 ret 
140a61bf2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a61c00 4889542410 mov qword ptr [rsp + 0x10], rdx
140a61c05 55 push rbp
140a61c06 4157 push r15
140a61c08 4156 push r14
140a61c0a 4154 push r12
140a61c0c 56 push rsi
140a61c0d 57 push rdi
140a61c0e 53 push rbx
140a61c0f 4883ec60 sub rsp, 0x60
140a61c13 488daa80000000 lea rbp, [rdx + 0x80]
140a61c1a 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a61c1f 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a61c24 80bd7605000008 cmp byte ptr [rbp + 0x576], 8
140a61c2b 740c je 0x140a61c39
140a61c2d 488d8d60050000 lea rcx, [rbp + 0x560]
140a61c34 e877081800 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a61c39 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a61c3e 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a61c43 4883c460 add rsp, 0x60
140a61c47 5b pop rbx
140a61c48 5f pop rdi
140a61c49 5e pop rsi
140a61c4a 415c pop r12
140a61c4c 415e pop r14
140a61c4e 415f pop r15
140a61c50 5d pop rbp
140a61c51 c3 ret 
140a61c52 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a61c60 4889542410 mov qword ptr [rsp + 0x10], rdx
140a61c65 55 push rbp
140a61c66 4157 push r15
140a61c68 4156 push r14
140a61c6a 4154 push r12
140a61c6c 56 push rsi
140a61c6d 57 push rdi
140a61c6e 53 push rbx
140a61c6f 4883ec60 sub rsp, 0x60
140a61c73 488daa80000000 lea rbp, [rdx + 0x80]
140a61c7a 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a61c7f 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a61c84 80bd7605000008 cmp byte ptr [rbp + 0x576], 8
140a61c8b 740c je 0x140a61c99
140a61c8d 488d8d60050000 lea rcx, [rbp + 0x560]
140a61c94 e817081800 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a61c99 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a61c9e 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a61ca3 4883c460 add rsp, 0x60
140a61ca7 5b pop rbx
140a61ca8 5f pop rdi
140a61ca9 5e pop rsi
140a61caa 415c pop r12
140a61cac 415e pop r14
140a61cae 415f pop r15
140a61cb0 5d pop rbp
140a61cb1 c3 ret 
140a61cb2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a61cc0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a61cc5 55 push rbp
140a61cc6 4157 push r15
140a61cc8 4156 push r14
140a61cca 4154 push r12
140a61ccc 56 push rsi
140a61ccd 57 push rdi
140a61cce 53 push rbx
140a61ccf 4883ec60 sub rsp, 0x60
140a61cd3 488daa80000000 lea rbp, [rdx + 0x80]
140a61cda 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a61cdf 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a61ce4 80bd7605000008 cmp byte ptr [rbp + 0x576], 8
140a61ceb 740c je 0x140a61cf9
140a61ced 488d8d60050000 lea rcx, [rbp + 0x560]
140a61cf4 e8b7071800 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a61cf9 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a61cfe 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a61d03 4883c460 add rsp, 0x60
140a61d07 5b pop rbx
140a61d08 5f pop rdi
140a61d09 5e pop rsi
140a61d0a 415c pop r12
140a61d0c 415e pop r14
140a61d0e 415f pop r15
140a61d10 5d pop rbp
140a61d11 c3 ret 
140a61d12 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a61d20 4889542410 mov qword ptr [rsp + 0x10], rdx
140a61d25 55 push rbp
140a61d26 4157 push r15
140a61d28 4156 push r14
140a61d2a 4154 push r12
140a61d2c 56 push rsi
140a61d2d 57 push rdi
140a61d2e 53 push rbx
140a61d2f 4883ec60 sub rsp, 0x60
140a61d33 488daa80000000 lea rbp, [rdx + 0x80]
140a61d3a 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a61d3f 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a61d44 80bd7605000008 cmp byte ptr [rbp + 0x576], 8
140a61d4b 740c je 0x140a61d59
140a61d4d 488d8d60050000 lea rcx, [rbp + 0x560]
140a61d54 e857071800 call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a61d59 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a61d5e 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a61d63 4883c460 add rsp, 0x60
140a61d67 5b pop rbx
140a61d68 5f pop rdi
140a61d69 5e pop rsi
140a61d6a 415c pop r12
140a61d6c 415e pop r14
140a61d6e 415f pop r15
140a61d70 5d pop rbp
140a61d71 c3 ret 
140a61d72 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a61d80 4889542410 mov qword ptr [rsp + 0x10], rdx
140a61d85 55 push rbp
140a61d86 4157 push r15
140a61d88 4156 push r14
140a61d8a 4154 push r12
140a61d8c 56 push rsi
140a61d8d 57 push rdi
140a61d8e 53 push rbx
140a61d8f 4883ec60 sub rsp, 0x60
140a61d93 488daa80000000 lea rbp, [rdx + 0x80]
140a61d9a 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a61d9f 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a61da4 488d8db0020000 lea rcx, [rbp + 0x2b0]
140a61dab e80052e3ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a61db0 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a61db5 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a61dba 4883c460 add rsp, 0x60
140a61dbe 5b pop rbx
140a61dbf 5f pop rdi
140a61dc0 5e pop rsi
140a61dc1 415c pop r12
140a61dc3 415e pop r14
140a61dc5 415f pop r15
140a61dc7 5d pop rbp
140a61dc8 c3 ret 
140a61dc9 0f1f8000000000 nop dword ptr [rax]
140a61dd0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a61dd5 55 push rbp
140a61dd6 4157 push r15
140a61dd8 4156 push r14
140a61dda 4154 push r12
140a61ddc 56 push rsi
140a61ddd 57 push rdi
140a61dde 53 push rbx
140a61ddf 4883ec60 sub rsp, 0x60
140a61de3 488daa80000000 lea rbp, [rdx + 0x80]
140a61dea 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a61def 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a61df4 488d8db0020000 lea rcx, [rbp + 0x2b0]
140a61dfb e8b051e3ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a61e00 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a61e05 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a61e0a 4883c460 add rsp, 0x60
140a61e0e 5b pop rbx
140a61e0f 5f pop rdi
140a61e10 5e pop rsi
140a61e11 415c pop r12
140a61e13 415e pop r14
140a61e15 415f pop r15
140a61e17 5d pop rbp
140a61e18 c3 ret 
140a61e19 0f1f8000000000 nop dword ptr [rax]
140a61e20 4889542410 mov qword ptr [rsp + 0x10], rdx
140a61e25 55 push rbp
140a61e26 4157 push r15
140a61e28 4156 push r14
140a61e2a 4154 push r12
140a61e2c 56 push rsi
140a61e2d 57 push rdi
140a61e2e 53 push rbx
140a61e2f 4883ec60 sub rsp, 0x60
140a61e33 488daa80000000 lea rbp, [rdx + 0x80]
140a61e3a 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a61e3f 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a61e44 488d8db0020000 lea rcx, [rbp + 0x2b0]
140a61e4b e86051e3ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a61e50 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a61e55 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a61e5a 4883c460 add rsp, 0x60
140a61e5e 5b pop rbx
140a61e5f 5f pop rdi
140a61e60 5e pop rsi
140a61e61 415c pop r12
140a61e63 415e pop r14
140a61e65 415f pop r15
140a61e67 5d pop rbp
140a61e68 c3 ret 
140a61e69 0f1f8000000000 nop dword ptr [rax]
140a61e70 4889542410 mov qword ptr [rsp + 0x10], rdx
140a61e75 55 push rbp
140a61e76 4157 push r15
140a61e78 4156 push r14
140a61e7a 4154 push r12
140a61e7c 56 push rsi
140a61e7d 57 push rdi
140a61e7e 53 push rbx
140a61e7f 4883ec60 sub rsp, 0x60
140a61e83 488daa80000000 lea rbp, [rdx + 0x80]
140a61e8a 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a61e8f 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a61e94 80bdd705000000 cmp byte ptr [rbp + 0x5d7], 0
140a61e9b 7519 jne 0x140a61eb6
140a61e9d 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a61ea2 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a61ea7 4883c460 add rsp, 0x60
140a61eab 5b pop rbx
140a61eac 5f pop rdi
140a61ead 5e pop rsi
140a61eae 415c pop r12
140a61eb0 415e pop r14
140a61eb2 415f pop r15
140a61eb4 5d pop rbp
140a61eb5 c3 ret 
140a61eb6 488d8d40040000 lea rcx, [rbp + 0x440]
140a61ebd e8ee50e3ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a61ec2 ebd9 jmp 0x140a61e9d
140a61ec4 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a61ed0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a61ed5 55 push rbp
140a61ed6 4157 push r15
140a61ed8 4156 push r14
140a61eda 4154 push r12
140a61edc 56 push rsi
140a61edd 57 push rdi
140a61ede 53 push rbx
140a61edf 4883ec60 sub rsp, 0x60
140a61ee3 488daa80000000 lea rbp, [rdx + 0x80]
140a61eea 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a61eef 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a61ef4 488d8d40040000 lea rcx, [rbp + 0x440]
140a61efb e830ad2e00 call 0x140d4cc30 ; anyhow::error::impl$6::drop
140a61f00 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a61f05 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a61f0a 4883c460 add rsp, 0x60
140a61f0e 5b pop rbx
140a61f0f 5f pop rdi
140a61f10 5e pop rsi
140a61f11 415c pop r12
140a61f13 415e pop r14
140a61f15 415f pop r15
140a61f17 5d pop rbp
140a61f18 c3 ret 
140a61f19 0f1f8000000000 nop dword ptr [rax]
140a61f20 4889542410 mov qword ptr [rsp + 0x10], rdx
140a61f25 55 push rbp
140a61f26 4157 push r15
140a61f28 4156 push r14
140a61f2a 4154 push r12
140a61f2c 56 push rsi
140a61f2d 57 push rdi
140a61f2e 53 push rbx
140a61f2f 4883ec60 sub rsp, 0x60
140a61f33 488daa80000000 lea rbp, [rdx + 0x80]
140a61f3a 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a61f3f 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a61f44 488d8db0020000 lea rcx, [rbp + 0x2b0]
140a61f4b e86050e3ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a61f50 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a61f55 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a61f5a 4883c460 add rsp, 0x60
140a61f5e 5b pop rbx
140a61f5f 5f pop rdi
140a61f60 5e pop rsi
140a61f61 415c pop r12
140a61f63 415e pop r14
140a61f65 415f pop r15
140a61f67 5d pop rbp
140a61f68 c3 ret 
140a61f69 0f1f8000000000 nop dword ptr [rax]
140a61f70 4889542410 mov qword ptr [rsp + 0x10], rdx
140a61f75 55 push rbp
140a61f76 4157 push r15
140a61f78 4156 push r14
140a61f7a 4154 push r12
140a61f7c 56 push rsi
140a61f7d 57 push rdi
140a61f7e 53 push rbx
140a61f7f 4883ec60 sub rsp, 0x60
140a61f83 488daa80000000 lea rbp, [rdx + 0x80]
140a61f8a 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a61f8f 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a61f94 488d8da0000000 lea rcx, [rbp + 0xa0]
140a61f9b e81050e3ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a61fa0 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a61fa5 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a61faa 4883c460 add rsp, 0x60
140a61fae 5b pop rbx
140a61faf 5f pop rdi
140a61fb0 5e pop rsi
140a61fb1 415c pop r12
140a61fb3 415e pop r14
140a61fb5 415f pop r15
140a61fb7 5d pop rbp
140a61fb8 c3 ret 
140a61fb9 0f1f8000000000 nop dword ptr [rax]
140a61fc0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a61fc5 55 push rbp
140a61fc6 4157 push r15
140a61fc8 4156 push r14
140a61fca 4154 push r12
140a61fcc 56 push rsi
140a61fcd 57 push rdi
140a61fce 53 push rbx
140a61fcf 4883ec60 sub rsp, 0x60
140a61fd3 488daa80000000 lea rbp, [rdx + 0x80]
140a61fda 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a61fdf 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a61fe4 488d8db0020000 lea rcx, [rbp + 0x2b0]
140a61feb e8c04fe3ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a61ff0 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a61ff5 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a61ffa 4883c460 add rsp, 0x60
140a61ffe 5b pop rbx
140a61fff 5f pop rdi
140a62000 5e pop rsi
140a62001 415c pop r12
140a62003 415e pop r14
140a62005 415f pop r15
140a62007 5d pop rbp
140a62008 c3 ret 
140a62009 0f1f8000000000 nop dword ptr [rax]
140a62010 4889542410 mov qword ptr [rsp + 0x10], rdx
140a62015 55 push rbp
140a62016 4157 push r15
140a62018 4156 push r14
140a6201a 4154 push r12
140a6201c 56 push rsi
140a6201d 57 push rdi
140a6201e 53 push rbx
140a6201f 4883ec60 sub rsp, 0x60
140a62023 488daa80000000 lea rbp, [rdx + 0x80]
140a6202a 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a6202f 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a62034 488d8d40040000 lea rcx, [rbp + 0x440]
140a6203b e8704fe3ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a62040 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a62045 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a6204a 4883c460 add rsp, 0x60
140a6204e 5b pop rbx
140a6204f 5f pop rdi
140a62050 5e pop rsi
140a62051 415c pop r12
140a62053 415e pop r14
140a62055 415f pop r15
140a62057 5d pop rbp
140a62058 c3 ret 
140a62059 0f1f8000000000 nop dword ptr [rax]
140a62060 4889542410 mov qword ptr [rsp + 0x10], rdx
140a62065 55 push rbp
140a62066 4157 push r15
140a62068 4156 push r14
140a6206a 4154 push r12
140a6206c 56 push rsi
140a6206d 57 push rdi
140a6206e 53 push rbx
140a6206f 4883ec60 sub rsp, 0x60
140a62073 488daa80000000 lea rbp, [rdx + 0x80]
140a6207a 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a6207f 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a62084 c685d705000000 mov byte ptr [rbp + 0x5d7], 0
140a6208b 488d8d40040000 lea rcx, [rbp + 0x440]
140a62092 e8194fe3ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a62097 c685d705000000 mov byte ptr [rbp + 0x5d7], 0
140a6209e 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a620a3 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a620a8 4883c460 add rsp, 0x60
140a620ac 5b pop rbx
140a620ad 5f pop rdi
140a620ae 5e pop rsi
140a620af 415c pop r12
140a620b1 415e pop r14
140a620b3 415f pop r15
140a620b5 5d pop rbp
140a620b6 c3 ret 
140a620b7 660f1f840000000000 nop word ptr [rax + rax]
140a620c0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a620c5 55 push rbp
140a620c6 4157 push r15
140a620c8 4156 push r14
140a620ca 4154 push r12
140a620cc 56 push rsi
140a620cd 57 push rdi
140a620ce 53 push rbx
140a620cf 4883ec60 sub rsp, 0x60
140a620d3 488daa80000000 lea rbp, [rdx + 0x80]
140a620da 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a620df 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a620e4 488d8db0020000 lea rcx, [rbp + 0x2b0]
140a620eb e8c04ee3ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a620f0 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a620f5 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a620fa 4883c460 add rsp, 0x60
140a620fe 5b pop rbx
140a620ff 5f pop rdi
140a62100 5e pop rsi
140a62101 415c pop r12
140a62103 415e pop r14
140a62105 415f pop r15
140a62107 5d pop rbp
140a62108 c3 ret 
140a62109 0f1f8000000000 nop dword ptr [rax]
140a62110 4889542410 mov qword ptr [rsp + 0x10], rdx
140a62115 55 push rbp
140a62116 4157 push r15
140a62118 4156 push r14
140a6211a 4154 push r12
140a6211c 56 push rsi
140a6211d 57 push rdi
140a6211e 53 push rbx
140a6211f 4883ec60 sub rsp, 0x60
140a62123 488daa80000000 lea rbp, [rdx + 0x80]
140a6212a 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a6212f 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a62134 488d8d40040000 lea rcx, [rbp + 0x440]
140a6213b e8704ee3ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a62140 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a62145 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a6214a 4883c460 add rsp, 0x60
140a6214e 5b pop rbx
140a6214f 5f pop rdi
140a62150 5e pop rsi
140a62151 415c pop r12
140a62153 415e pop r14
140a62155 415f pop r15
140a62157 5d pop rbp
140a62158 c3 ret 
140a62159 0f1f8000000000 nop dword ptr [rax]
140a62160 4889542410 mov qword ptr [rsp + 0x10], rdx
140a62165 55 push rbp
140a62166 4157 push r15
140a62168 4156 push r14
140a6216a 4154 push r12
140a6216c 56 push rsi
140a6216d 57 push rdi
140a6216e 53 push rbx
140a6216f 4883ec60 sub rsp, 0x60
140a62173 488daa80000000 lea rbp, [rdx + 0x80]
140a6217a 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a6217f 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a62184 488d8db0020000 lea rcx, [rbp + 0x2b0]
140a6218b e8204ee3ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a62190 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a62195 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a6219a 4883c460 add rsp, 0x60
140a6219e 5b pop rbx
140a6219f 5f pop rdi
140a621a0 5e pop rsi
140a621a1 415c pop r12
140a621a3 415e pop r14
140a621a5 415f pop r15
140a621a7 5d pop rbp
140a621a8 c3 ret 
140a621a9 0f1f8000000000 nop dword ptr [rax]
140a621b0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a621b5 55 push rbp
140a621b6 4157 push r15
140a621b8 4156 push r14
140a621ba 4154 push r12
140a621bc 56 push rsi
140a621bd 57 push rdi
140a621be 53 push rbx
140a621bf 4883ec60 sub rsp, 0x60
140a621c3 488daa80000000 lea rbp, [rdx + 0x80]
140a621ca 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a621cf 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a621d4 488d8d40040000 lea rcx, [rbp + 0x440]
140a621db e8d04de3ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a621e0 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a621e5 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a621ea 4883c460 add rsp, 0x60
140a621ee 5b pop rbx
140a621ef 5f pop rdi
140a621f0 5e pop rsi
140a621f1 415c pop r12
140a621f3 415e pop r14
140a621f5 415f pop r15
140a621f7 5d pop rbp
140a621f8 c3 ret 
140a621f9 0f1f8000000000 nop dword ptr [rax]
140a62200 4889542410 mov qword ptr [rsp + 0x10], rdx
140a62205 55 push rbp
140a62206 4157 push r15
140a62208 4156 push r14
140a6220a 4154 push r12
140a6220c 56 push rsi
140a6220d 57 push rdi
140a6220e 53 push rbx
140a6220f 4883ec60 sub rsp, 0x60
140a62213 488daa80000000 lea rbp, [rdx + 0x80]
140a6221a 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a6221f 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a62224 488d8da0000000 lea rcx, [rbp + 0xa0]
140a6222b e8804de3ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a62230 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a62235 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a6223a 4883c460 add rsp, 0x60
140a6223e 5b pop rbx
140a6223f 5f pop rdi
140a62240 5e pop rsi
140a62241 415c pop r12
140a62243 415e pop r14
140a62245 415f pop r15
140a62247 5d pop rbp
140a62248 c3 ret 
140a62249 0f1f8000000000 nop dword ptr [rax]
140a62250 4889542410 mov qword ptr [rsp + 0x10], rdx
140a62255 55 push rbp
140a62256 4157 push r15
140a62258 4156 push r14
140a6225a 4154 push r12
140a6225c 56 push rsi
140a6225d 57 push rdi
140a6225e 53 push rbx
140a6225f 4883ec60 sub rsp, 0x60
140a62263 488daa80000000 lea rbp, [rdx + 0x80]
140a6226a 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a6226f 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a62274 488d8db0020000 lea rcx, [rbp + 0x2b0]
140a6227b e8304de3ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a62280 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a62285 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a6228a 4883c460 add rsp, 0x60
140a6228e 5b pop rbx
140a6228f 5f pop rdi
140a62290 5e pop rsi
140a62291 415c pop r12
140a62293 415e pop r14
140a62295 415f pop r15
140a62297 5d pop rbp
140a62298 c3 ret 
140a62299 0f1f8000000000 nop dword ptr [rax]
140a622a0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a622a5 55 push rbp
140a622a6 4157 push r15
140a622a8 4156 push r14
140a622aa 4154 push r12
140a622ac 56 push rsi
140a622ad 57 push rdi
140a622ae 53 push rbx
140a622af 4883ec60 sub rsp, 0x60
140a622b3 488daa80000000 lea rbp, [rdx + 0x80]
140a622ba 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a622bf 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a622c4 488d8d40040000 lea rcx, [rbp + 0x440]
140a622cb e8e04ce3ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a622d0 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a622d5 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a622da 4883c460 add rsp, 0x60
140a622de 5b pop rbx
140a622df 5f pop rdi
140a622e0 5e pop rsi
140a622e1 415c pop r12
140a622e3 415e pop r14
140a622e5 415f pop r15
140a622e7 5d pop rbp
140a622e8 c3 ret 
140a622e9 0f1f8000000000 nop dword ptr [rax]
140a622f0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a622f5 55 push rbp
140a622f6 4157 push r15
140a622f8 4156 push r14
140a622fa 4154 push r12
140a622fc 56 push rsi
140a622fd 57 push rdi
140a622fe 53 push rbx
140a622ff 4883ec60 sub rsp, 0x60
140a62303 488daa80000000 lea rbp, [rdx + 0x80]
140a6230a 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a6230f 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a62314 488d8db0020000 lea rcx, [rbp + 0x2b0]
140a6231b e8904ce3ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a62320 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a62325 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a6232a 4883c460 add rsp, 0x60
140a6232e 5b pop rbx
140a6232f 5f pop rdi
140a62330 5e pop rsi
140a62331 415c pop r12
140a62333 415e pop r14
140a62335 415f pop r15
140a62337 5d pop rbp
140a62338 c3 ret 
140a62339 0f1f8000000000 nop dword ptr [rax]
140a62340 4889542410 mov qword ptr [rsp + 0x10], rdx
140a62345 55 push rbp
140a62346 4157 push r15
140a62348 4156 push r14
140a6234a 4154 push r12
140a6234c 56 push rsi
140a6234d 57 push rdi
140a6234e 53 push rbx
140a6234f 4883ec60 sub rsp, 0x60
140a62353 488daa80000000 lea rbp, [rdx + 0x80]
140a6235a 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a6235f 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a62364 488d8d40040000 lea rcx, [rbp + 0x440]
140a6236b e8404ce3ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a62370 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a62375 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a6237a 4883c460 add rsp, 0x60
140a6237e 5b pop rbx
140a6237f 5f pop rdi
140a62380 5e pop rsi
140a62381 415c pop r12
140a62383 415e pop r14
140a62385 415f pop r15
140a62387 5d pop rbp
140a62388 c3 ret 
140a62389 0f1f8000000000 nop dword ptr [rax]
140a62390 4889542410 mov qword ptr [rsp + 0x10], rdx
140a62395 55 push rbp
140a62396 4157 push r15
140a62398 4156 push r14
140a6239a 4154 push r12
140a6239c 56 push rsi
140a6239d 57 push rdi
140a6239e 53 push rbx
140a6239f 4883ec60 sub rsp, 0x60
140a623a3 488daa80000000 lea rbp, [rdx + 0x80]
140a623aa 0f297c2430 movaps xmmword ptr [rsp + 0x30], xmm7
140a623af 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140a623b4 488d8db0020000 lea rcx, [rbp + 0x2b0]
140a623bb e8f04be3ff call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a623c0 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140a623c5 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
140a623ca 4883c460 add rsp, 0x60
140a623ce 5b pop rbx
140a623cf 5f pop rdi
140a623d0 5e pop rsi
140a623d1 415c pop r12
140a623d3 415e pop r14
140a623d5 415f pop r15
140a623d7 5d pop rbp
140a623d8 c3 ret 
