=== 0x140833250 size=3  rhapsody::rg::resource::ResourceHandle::raw
    ; rg::resource::ResourceHandle::raw: `mov eax,ecx; ret` — identity returning the u32 handle.  [high]
    0x140833250    89c8                     mov              eax, ecx
    0x140833252    c3                       ret              

=== 0x140b25c90 size=5  rhapsody::rg::recorder::PassResourceAccessType::new
    ; PassResourceAccessType::new: `mov eax,edx; mov edx,ecx; ret` — swaps the two args (ecx<->edx). Root cause of the earlier mis-modeled field binding.  [high]
    0x140b25c90    89d0                     mov              eax, edx
    0x140b25c92    89ca                     mov              edx, ecx
    0x140b25c94    c3                       ret              

=== 0x140bd6c00 size=47  rhapsody::rg::pass_api::_$LT$impl$u20$rhapsody..rg..resource..Ref$LT$rhapsody..raw..texture..Texture$C$rhapsody..rg..resource..GpuSrv$GT$$GT$::bind_view::h773410cf4eefe834
    ; Ref<Texture,GpuSrv>::bind_view: copies 0x40B TextureViewDesc from r8, sets u32@0x3c:=[rdx], byte@0x40:=0.  [high]
    0x140bd6c00    4889c8                   mov              rax, rcx
    0x140bd6c03    8b0a                     mov              ecx, dword ptr [rdx]
    0x140bd6c05    410f1000                 movups           xmm0, xmmword ptr [r8]
    0x140bd6c09    410f104810               movups           xmm1, xmmword ptr [r8 + 0x10]
    0x140bd6c0e    410f105020               movups           xmm2, xmmword ptr [r8 + 0x20]
    0x140bd6c13    410f10582c               movups           xmm3, xmmword ptr [r8 + 0x2c]
    0x140bd6c18    0f11582c                 movups           xmmword ptr [rax + 0x2c], xmm3
    0x140bd6c1c    0f115020                 movups           xmmword ptr [rax + 0x20], xmm2
    0x140bd6c20    0f114810                 movups           xmmword ptr [rax + 0x10], xmm1
    0x140bd6c24    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140bd6c27    89483c                   mov              dword ptr [rax + 0x3c], ecx
    0x140bd6c2a    c6404000                 mov              byte ptr [rax + 0x40], 0
    0x140bd6c2e    c3                       ret              

=== 0x140bd6c30 size=47  rhapsody::rg::pass_api::_$LT$impl$u20$rhapsody..rg..resource..Ref$LT$rhapsody..raw..texture..Texture$C$rhapsody..rg..resource..GpuUav$GT$$GT$::bind_view::h511f5976314d1918
    ; Ref<Texture,GpuUav>::bind_view: same but byte@0x40:=1 (Srv=0, Uav=1 discriminator).  [high]
    0x140bd6c30    4889c8                   mov              rax, rcx
    0x140bd6c33    8b0a                     mov              ecx, dword ptr [rdx]
    0x140bd6c35    410f1000                 movups           xmm0, xmmword ptr [r8]
    0x140bd6c39    410f104810               movups           xmm1, xmmword ptr [r8 + 0x10]
    0x140bd6c3e    410f105020               movups           xmm2, xmmword ptr [r8 + 0x20]
    0x140bd6c43    410f10582c               movups           xmm3, xmmword ptr [r8 + 0x2c]
    0x140bd6c48    0f11582c                 movups           xmmword ptr [rax + 0x2c], xmm3
    0x140bd6c4c    0f115020                 movups           xmmword ptr [rax + 0x20], xmm2
    0x140bd6c50    0f114810                 movups           xmmword ptr [rax + 0x10], xmm1
    0x140bd6c54    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140bd6c57    89483c                   mov              dword ptr [rax + 0x3c], ecx
    0x140bd6c5a    c6404001                 mov              byte ptr [rax + 0x40], 1
    0x140bd6c5e    c3                       ret              

=== 0x140bd6c60 size=15  rhapsody::rg::pass_api::impl$5::bind
    ; rg::pass_api::impl$5::bind: {u32@0x04:=[rdx]; u32@0x00:=4} — variant tag 4.  [high]
    0x140bd6c60    4889c8                   mov              rax, rcx
    0x140bd6c63    8b0a                     mov              ecx, dword ptr [rdx]
    0x140bd6c65    894804                   mov              dword ptr [rax + 4], ecx
    0x140bd6c68    c70004000000             mov              dword ptr [rax], 4
    0x140bd6c6e    c3                       ret              

=== 0x140bd6c70 size=15  rhapsody::rg::pass_api::_$LT$impl$u20$rhapsody..rg..resource..Ref$LT$rhapsody..raw..buffer..Buffer$C$rhapsody..rg..resource..GpuSrv$GT$$GT$::bind_uniform_buffer::hd85c0dde08ae74d6
    ; Ref<Buffer,GpuSrv>::bind_uniform_buffer: {u32@0x04:=[rdx]; u32@0x00:=3} — variant tag 3.  [high]
    0x140bd6c70    4889c8                   mov              rax, rcx
    0x140bd6c73    8b0a                     mov              ecx, dword ptr [rdx]
    0x140bd6c75    894804                   mov              dword ptr [rax + 4], ecx
    0x140bd6c78    c70003000000             mov              dword ptr [rax], 3
    0x140bd6c7e    c3                       ret              

=== 0x140bd6ce0 size=18  rhapsody::rg::resource::impl$4::from
    ; rg::resource::impl$4::from: copy 0x20B.  [high]
    0x140bd6ce0    4889c8                   mov              rax, rcx
    0x140bd6ce3    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140bd6ce6    0f104a10                 movups           xmm1, xmmword ptr [rdx + 0x10]
    0x140bd6cea    0f114910                 movups           xmmword ptr [rcx + 0x10], xmm1
    0x140bd6cee    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140bd6cf1    c3                       ret              

=== 0x140bd6d00 size=14  rhapsody::rg::resource::impl$5::from
    ; rg::resource::impl$5::from: copy 0x10B; byte@0x1e:=7 (tag 7).  [high]
    0x140bd6d00    4889c8                   mov              rax, rcx
    0x140bd6d03    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140bd6d06    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140bd6d09    c6411e07                 mov              byte ptr [rcx + 0x1e], 7
    0x140bd6d0d    c3                       ret              

=== 0x140bdbcc0 size=42  rhapsody::raw::init::impl$14::from
    ; rg::init::impl$14::from: copy 0x40B; qword@0:=0x14 (tag 20).  [high]
    0x140bdbcc0    4889c8                   mov              rax, rcx
    0x140bdbcc3    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140bdbcc6    0f104a10                 movups           xmm1, xmmword ptr [rdx + 0x10]
    0x140bdbcca    0f105220                 movups           xmm2, xmmword ptr [rdx + 0x20]
    0x140bdbcce    0f105a30                 movups           xmm3, xmmword ptr [rdx + 0x30]
    0x140bdbcd2    0f115938                 movups           xmmword ptr [rcx + 0x38], xmm3
    0x140bdbcd6    0f115128                 movups           xmmword ptr [rcx + 0x28], xmm2
    0x140bdbcda    0f114918                 movups           xmmword ptr [rcx + 0x18], xmm1
    0x140bdbcde    0f114108                 movups           xmmword ptr [rcx + 8], xmm0
    0x140bdbce2    48c70114000000           mov              qword ptr [rcx], 0x14
    0x140bdbce9    c3                       ret              

=== 0x140be57d0 size=622  rhapsody::rg::state::impl$0::default
    ; rg::state::impl$0::default: constructs a default RenderGraphState incl. a std RandomState HashMap (calls RandomState::new KEYS shim). Body fully decoded.  [medium]
    0x140be57d0    55                       push             rbp
    0x140be57d1    4156                     push             r14
    0x140be57d3    56                       push             rsi
    0x140be57d4    57                       push             rdi
    0x140be57d5    53                       push             rbx
    0x140be57d6    4881ec60010000           sub              rsp, 0x160
    0x140be57dd    488dac2480000000         lea              rbp, [rsp + 0x80]
    0x140be57e5    0f29bdd0000000           movaps           xmmword ptr [rbp + 0xd0], xmm7
    0x140be57ec    0f29b5c0000000           movaps           xmmword ptr [rbp + 0xc0], xmm6
    0x140be57f3    48c785b8000000feffffff   mov              qword ptr [rbp + 0xb8], 0xfffffffffffffffe
    0x140be57fe    4889ce                   mov              rsi, rcx
    0x140be5801    0f57c0                   xorps            xmm0, xmm0
    0x140be5804    0f1145b0                 movups           xmmword ptr [rbp - 0x50], xmm0
    0x140be5808    0f1145c8                 movups           xmmword ptr [rbp - 0x38], xmm0
    0x140be580c    48c745a000000000         mov              qword ptr [rbp - 0x60], 0
    0x140be5814    48c745a808000000         mov              qword ptr [rbp - 0x58], 8
    0x140be581c    48c745c008000000         mov              qword ptr [rbp - 0x40], 8
    0x140be5824    48c745d808000000         mov              qword ptr [rbp - 0x28], 8
    0x140be582c    48c745e000000000         mov              qword ptr [rbp - 0x20], 0
    0x140be5834    48b80000000000000080     movabs           rax, 0x8000000000000000
    0x140be583e    488945e8                 mov              qword ptr [rbp - 0x18], rax
    0x140be5842    c6452a07                 mov              byte ptr [rbp + 0x2a], 7
    0x140be5846    e8f57e1600               call             0x140d4d740 -> std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$::he31e4d4088ff0746
    0x140be584b    4889c7                   mov              rdi, rax
    0x140be584e    f60001                   test             byte ptr [rax], 1
    0x140be5851    7411                     je               0x140be5864
    0x140be5853    4889f9                   mov              rcx, rdi
    0x140be5856    4883c108                 add              rcx, 8
    0x140be585a    488b4708                 mov              rax, qword ptr [rdi + 8]
    0x140be585e    488b5710                 mov              rdx, qword ptr [rdi + 0x10]
    0x140be5862    eb1b                     jmp              0x140be587f
    0x140be5864    e897f51700               call             0x140d64e00 -> std::sys::random::hashmap_random_keys::h6ad24d26f7fc1598
    0x140be5869    4889f9                   mov              rcx, rdi
    0x140be586c    4883c108                 add              rcx, 8
    0x140be5870    48c70701000000           mov              qword ptr [rdi], 1
    0x140be5877    48894708                 mov              qword ptr [rdi + 8], rax
    0x140be587b    48895710                 mov              qword ptr [rdi + 0x10], rdx
    0x140be587f    4c8d4001                 lea              r8, [rax + 1]
    0x140be5883    4c8901                   mov              qword ptr [rcx], r8
    0x140be5886    0f103513ddf201           movups           xmm6, xmmword ptr [rip + 0x1f2dd13] -> 0x142b135a0
    0x140be588d    0f29b590000000           movaps           xmmword ptr [rbp + 0x90], xmm6
    0x140be5894    0f103df5dcf201           movups           xmm7, xmmword ptr [rip + 0x1f2dcf5] -> 0x142b13590
    0x140be589b    0f29bd80000000           movaps           xmmword ptr [rbp + 0x80], xmm7
    0x140be58a2    488985a0000000           mov              qword ptr [rbp + 0xa0], rax
    0x140be58a9    488995a8000000           mov              qword ptr [rbp + 0xa8], rdx
    0x140be58b0    e88b7e1600               call             0x140d4d740 -> std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$::he31e4d4088ff0746
    0x140be58b5    4989c6                   mov              r14, rax
    0x140be58b8    f60001                   test             byte ptr [rax], 1
    0x140be58bb    7411                     je               0x140be58ce
    0x140be58bd    4c89f0                   mov              rax, r14
    0x140be58c0    4883c008                 add              rax, 8
    0x140be58c4    498b7e08                 mov              rdi, qword ptr [r14 + 8]
    0x140be58c8    498b5e10                 mov              rbx, qword ptr [r14 + 0x10]
    0x140be58cc    eb21                     jmp              0x140be58ef
    0x140be58ce    e82df51700               call             0x140d64e00 -> std::sys::random::hashmap_random_keys::h6ad24d26f7fc1598
    0x140be58d3    4889c7                   mov              rdi, rax
    0x140be58d6    4889d3                   mov              rbx, rdx
    0x140be58d9    4c89f0                   mov              rax, r14
    0x140be58dc    4883c008                 add              rax, 8
    0x140be58e0    49c70601000000           mov              qword ptr [r14], 1
    0x140be58e7    49897e08                 mov              qword ptr [r14 + 8], rdi
    0x140be58eb    49895610                 mov              qword ptr [r14 + 0x10], rdx
    0x140be58ef    488d4f01                 lea              rcx, [rdi + 1]
    0x140be58f3    488908                   mov              qword ptr [rax], rcx
    0x140be58f6    0f288580000000           movaps           xmm0, xmmword ptr [rbp + 0x80]
    0x140be58fd    0f288d90000000           movaps           xmm1, xmmword ptr [rbp + 0x90]
    0x140be5904    0f2895a0000000           movaps           xmm2, xmmword ptr [rbp + 0xa0]
    0x140be590b    0f295550                 movaps           xmmword ptr [rbp + 0x50], xmm2
    0x140be590f    0f294d40                 movaps           xmmword ptr [rbp + 0x40], xmm1
    0x140be5913    0f294530                 movaps           xmmword ptr [rbp + 0x30], xmm0
    0x140be5917    0f297d60                 movaps           xmmword ptr [rbp + 0x60], xmm7
    0x140be591b    0f297570                 movaps           xmmword ptr [rbp + 0x70], xmm6
    0x140be591f    488d55a0                 lea              rdx, [rbp - 0x60]
    0x140be5923    41b890000000             mov              r8d, 0x90
    0x140be5929    4889f1                   mov              rcx, rsi
    0x140be592c    e8cfe2d301               call             0x142923c00 -> memcpy
    0x140be5931    0f284530                 movaps           xmm0, xmmword ptr [rbp + 0x30]
    0x140be5935    0f284d40                 movaps           xmm1, xmmword ptr [rbp + 0x40]
    0x140be5939    0f285550                 movaps           xmm2, xmmword ptr [rbp + 0x50]
    0x140be593d    0f285d60                 movaps           xmm3, xmmword ptr [rbp + 0x60]
    0x140be5941    0f118690000000           movups           xmmword ptr [rsi + 0x90], xmm0
    0x140be5948    0f118ea0000000           movups           xmmword ptr [rsi + 0xa0], xmm1
    0x140be594f    0f1196b0000000           movups           xmmword ptr [rsi + 0xb0], xmm2
    0x140be5956    0f119ec0000000           movups           xmmword ptr [rsi + 0xc0], xmm3
    0x140be595d    0f284570                 movaps           xmm0, xmmword ptr [rbp + 0x70]
    0x140be5961    0f1186d0000000           movups           xmmword ptr [rsi + 0xd0], xmm0
    0x140be5968    4889bee0000000           mov              qword ptr [rsi + 0xe0], rdi
    0x140be596f    48899ee8000000           mov              qword ptr [rsi + 0xe8], rbx
    0x140be5976    4889f0                   mov              rax, rsi
    0x140be5979    0f28b5c0000000           movaps           xmm6, xmmword ptr [rbp + 0xc0]
    0x140be5980    0f28bdd0000000           movaps           xmm7, xmmword ptr [rbp + 0xd0]
    0x140be5987    4881c460010000           add              rsp, 0x160
    0x140be598e    5b                       pop              rbx
    0x140be598f    5f                       pop              rdi
    0x140be5990    5e                       pop              rsi
    0x140be5991    415e                     pop              r14
    0x140be5993    5d                       pop              rbp
    0x140be5994    c3                       ret              
    0x140be5995    66662e0f1f840000000000   nop              word ptr cs:[rax + rax]
    0x140be59a0    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x140be59a5    55                       push             rbp
    0x140be59a6    4156                     push             r14
    0x140be59a8    56                       push             rsi
    0x140be59a9    57                       push             rdi
    0x140be59aa    53                       push             rbx
    0x140be59ab    4883ec40                 sub              rsp, 0x40
    0x140be59af    488daa80000000           lea              rbp, [rdx + 0x80]
    0x140be59b6    0f297c2420               movaps           xmmword ptr [rsp + 0x20], xmm7
    0x140be59bb    0f29742430               movaps           xmmword ptr [rsp + 0x30], xmm6
    0x140be59c0    488d8d80000000           lea              rcx, [rbp + 0x80]
    0x140be59c7    41b868000000             mov              r8d, 0x68
    0x140be59cd    41b910000000             mov              r9d, 0x10
    0x140be59d3    488d95a0000000           lea              rdx, [rbp + 0xa0]
    0x140be59da    e8c11cc000               call             0x1417e76a0 -> hashbrown::raw::RawTableInner::drop_inner_table::h9126dbabe7fce4ab
    0x140be59df    0f28742430               movaps           xmm6, xmmword ptr [rsp + 0x30]
    0x140be59e4    0f287c2420               movaps           xmm7, xmmword ptr [rsp + 0x20]
    0x140be59e9    4883c440                 add              rsp, 0x40
    0x140be59ed    5b                       pop              rbx
    0x140be59ee    5f                       pop              rdi
    0x140be59ef    5e                       pop              rsi
    0x140be59f0    415e                     pop              r14
    0x140be59f2    5d                       pop              rbp
    0x140be59f3    c3                       ret              
    0x140be59f4    6666662e0f1f840000000000 nop              word ptr cs:[rax + rax]
    0x140be5a00    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x140be5a05    55                       push             rbp
    0x140be5a06    4156                     push             r14
    0x140be5a08    56                       push             rsi
    0x140be5a09    57                       push             rdi
    0x140be5a0a    53                       push             rbx
    0x140be5a0b    4883ec40                 sub              rsp, 0x40
    0x140be5a0f    488daa80000000           lea              rbp, [rdx + 0x80]
    0x140be5a16    0f297c2420               movaps           xmmword ptr [rsp + 0x20], xmm7
    0x140be5a1b    0f29742430               movaps           xmmword ptr [rsp + 0x30], xmm6
    0x140be5a20    488d4da0                 lea              rcx, [rbp - 0x60]
    0x140be5a24    e8e799ffff               call             0x140bdf410 -> core::ptr::drop_in_place$LT$rhapsody..rg..graph..RenderGraph$GT$::hdab19b2337789a18
    0x140be5a29    0f28742430               movaps           xmm6, xmmword ptr [rsp + 0x30]
    0x140be5a2e    0f287c2420               movaps           xmm7, xmmword ptr [rsp + 0x20]
    0x140be5a33    4883c440                 add              rsp, 0x40
    0x140be5a37    5b                       pop              rbx
    0x140be5a38    5f                       pop              rdi
    0x140be5a39    5e                       pop              rsi
    0x140be5a3a    415e                     pop              r14
    0x140be5a3c    5d                       pop              rbp
    0x140be5a3d    c3                       ret              

=== 0x140be95e0 size=30  rhapsody::rg::compiled::BatchCompiler::new
    ; rg::compiled::BatchCompiler::new: constructs {cap/ptr@0..0x08 (empty vec: [0]=0,[8]=8),[0x10..0x1f]=0,[0x19..0x28]=0}. The previously-'unavailable' 30-byte body, now solved.  [high]
    0x140be95e0    4889c8                   mov              rax, rcx
    0x140be95e3    48c70100000000           mov              qword ptr [rcx], 0
    0x140be95ea    48c7410808000000         mov              qword ptr [rcx + 8], 8
    0x140be95f2    0f57c0                   xorps            xmm0, xmm0
    0x140be95f5    0f114110                 movups           xmmword ptr [rcx + 0x10], xmm0
    0x140be95f9    0f114119                 movups           xmmword ptr [rcx + 0x19], xmm0
    0x140be95fd    c3                       ret              

=== 0x140be9690 size=5  rhapsody::rg::compiled::pass_access_range_indices
    ; rg::compiled::pass_access_range_indices: `mov eax,ecx; mov edx,edx; ret` — identity, returns the two u32 args unchanged as a pair. The previously-'unavailable' 5-byte body, now solved.  [high]
    0x140be9690    89c8                     mov              eax, ecx
    0x140be9692    89d2                     mov              edx, edx
    0x140be9694    c3                       ret              

=== 0x140be9c70 size=7  rhapsody::rg::executor::PassPgoDuration::new
    ; rg::executor::PassPgoDuration::new: `mov rdx,r8; mov rax,rcx; ret` — returns (rcx, r8) as a 2-field struct (args passthrough).  [medium]
    0x140be9c70    4c89c2                   mov              rdx, r8
    0x140be9c73    4889c8                   mov              rax, rcx
    0x140be9c76    c3                       ret              

=== 0x140bece40 size=3  rhapsody::rg::recorder::PassResourceRef::resource_index
    ; rg::recorder::PassResourceRef::resource_index: returns u32 [rcx].  [high]
    0x140bece40    8b01                     mov              eax, dword ptr [rcx]
    0x140bece42    c3                       ret              

=== 0x140bece50 size=14  rhapsody::rg::recorder::ResourceSyncGroup::new::h9c88dac2e7b1f0a3
    ; ResourceSyncGroup::new: eax=1; `lock xadd dword [0x14362c230], eax` -> returns previous value of the global AtomicU32 (monotonic group-id allocator).  [high]
    0x140bece50    b801000000               mov              eax, 1
    0x140bece55    f00fc105d3f3a302         lock xadd        dword ptr [rip + 0x2a3f3d3], eax -> 0x14362c230
    0x140bece5d    c3                       ret              

=== 0x140bece60 size=8  rhapsody::rg::recorder::ResourceSyncGroup::is_equivalent
    ; ResourceSyncGroup::is_equivalent: (u32[a]==u32[b]) — equality of the group id.  [high]
    0x140bece60    8b01                     mov              eax, dword ptr [rcx]
    0x140bece62    3b02                     cmp              eax, dword ptr [rdx]
    0x140bece64    0f94c0                   sete             al
    0x140bece67    c3                       ret              

=== 0x1415fc050 size=22  rhapsody::rg::recorder::impl$1::render<country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0,country_core::render::pass_builder::pass_resources::PassResources>
    ; rg::recorder::impl$1::render<...dispatch closure...>: push rsi; call Box<FnOnce>::call_once; return rsi. Thin closure-invoker.  [medium]
    0x1415fc050    56                       push             rsi
    0x1415fc051    4883ec20                 sub              rsp, 0x20
    0x1415fc055    4889ce                   mov              rsi, rcx
    0x1415fc058    e813ef0900               call             0x14169af70 -> _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..FnOnce$LT$Args$GT$$GT$::call_once::hf749a58f06269f5d
    0x1415fc05d    4889f0                   mov              rax, rsi
    0x1415fc060    4883c420                 add              rsp, 0x20
    0x1415fc064    5e                       pop              rsi
    0x1415fc065    c3                       ret              

=== 0x1416f7850 size=18  rhapsody::rg::pass_api::RenderPassApi::end_render_pass
    ; RenderPassApi::end_render_pass: rcx=[self]; rdx=[self+8]; rcx=[rax+0x48]; jmp [rdx+0x428] (vtable dispatch).  [high]
    0x1416f7850    488b01                   mov              rax, qword ptr [rcx]
    0x1416f7853    488b5108                 mov              rdx, qword ptr [rcx + 8]
    0x1416f7857    488b4848                 mov              rcx, qword ptr [rax + 0x48]
    0x1416f785b    48ffa228040000           jmp              qword ptr [rdx + 0x428]

=== 0x1416f7960 size=31  rhapsody::rg::pass_api::RenderPassApi::set_line_width
    ; RenderPassApi::set_line_width: if byte[*(self+8)+0xad9]!=0 then jmp through [(*(self)+0x48)+0x2f0]. Gated by a feature flag at +0xad9.  [high]
    0x1416f7960    488b4108                 mov              rax, qword ptr [rcx + 8]
    0x1416f7964    80b8d90a000000           cmp              byte ptr [rax + 0xad9], 0
    0x1416f796b    7411                     je               0x1416f797e
    0x1416f796d    488b09                   mov              rcx, qword ptr [rcx]
    0x1416f7970    488b4948                 mov              rcx, qword ptr [rcx + 0x48]
    0x1416f7974    488b80f0020000           mov              rax, qword ptr [rax + 0x2f0]
    0x1416f797b    48ffe0                   jmp              rax
    0x1416f797e    c3                       ret              

=== 0x1428caa40 size=17  rhapsody::rg::resource_registry::ResourceRegistry::image_from_raw_handle::panic_cold_explicit::h05ee72f6a1c08fcd (.llvm.15200196457440221650)
    ; ResourceRegistry::image_from_raw_handle panic_cold_explicit: calls panic_explicit with msg@0x142b13cf0.  [medium]
    0x1428caa40    4883ec28                 sub              rsp, 0x28
    0x1428caa44    488d0da5922400           lea              rcx, [rip + 0x2492a5] -> 0x142b13cf0
    0x1428caa4b    e870ea0000               call             0x1428d94c0 -> core::panicking::panic_explicit::h3ab45e09908fb10a
    0x1428caa50    cc                       int3             

=== 0x1428caa60 size=17  rhapsody::rg::resource_registry::ResourceRegistry::buffer::panic_cold_explicit::hda41c95fa11d4db1 (.llvm.15200196457440221650)
    ; ResourceRegistry::buffer panic_cold: panic_explicit msg@0x142b13d50.  [medium]
    0x1428caa60    4883ec28                 sub              rsp, 0x28
    0x1428caa64    488d0de5922400           lea              rcx, [rip + 0x2492e5] -> 0x142b13d50
    0x1428caa6b    e850ea0000               call             0x1428d94c0 -> core::panicking::panic_explicit::h3ab45e09908fb10a
    0x1428caa70    cc                       int3             

=== 0x1428caa80 size=17  rhapsody::rg::resource_registry::ResourceRegistry::image_view::panic_cold_explicit::h557d252ef5ebcab8 (.llvm.15200196457440221650)
    ; ResourceRegistry::image_view panic_cold: panic_explicit msg@0x142b13dc8.  [medium]
    0x1428caa80    4883ec28                 sub              rsp, 0x28
    0x1428caa84    488d0d3d932400           lea              rcx, [rip + 0x24933d] -> 0x142b13dc8
    0x1428caa8b    e830ea0000               call             0x1428d94c0 -> core::panicking::panic_explicit::h3ab45e09908fb10a
    0x1428caa90    cc                       int3             

=== 0x1428ee9f0 size=10  rhapsody::rg::import_export::impl$1::from_owned_rg_resource::panic_cold_explicit
    ; rg::import_export::impl$1::from_owned_rg_resource panic_cold: panic_explicit.  [medium]
    0x1428ee9f0    4883ec28                 sub              rsp, 0x28
    0x1428ee9f4    e8c7aafeff               call             0x1428d94c0 -> core::panicking::panic_explicit::h3ab45e09908fb10a
    0x1428ee9f9    cc                       int3             
