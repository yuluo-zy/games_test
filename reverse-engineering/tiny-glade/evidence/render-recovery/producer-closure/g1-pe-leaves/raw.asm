=== 0x140082a00 size=477  rhapsody::raw::workarounds::_::_$LT$impl$u20$serde..ser..Serialize$u20$for$u20$rhapsody..raw..workarounds..GraphicsWorkarounds$GT$::serialize::h7a08a7a5b4a75fbb
    ; GraphicsWorkarounds serialize (serde): serializes workaround flags. Body decoded.  [medium]
    0x140082a00    55                       push             rbp
    0x140082a01    56                       push             rsi
    0x140082a02    57                       push             rdi
    0x140082a03    4881ecd0000000           sub              rsp, 0xd0
    0x140082a0a    488dac2480000000         lea              rbp, [rsp + 0x80]
    0x140082a12    48c74548feffffff         mov              qword ptr [rbp + 0x48], 0xfffffffffffffffe
    0x140082a1a    4c89c0                   mov              rax, r8
    0x140082a1d    4889d7                   mov              rdi, rdx
    0x140082a20    4889ce                   mov              rsi, rcx
    0x140082a23    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x140082a2c    4c8d059de18a02           lea              r8, [rip + 0x28ae19d] -> 0x142930bd0
    0x140082a33    488d4df0                 lea              rcx, [rbp - 0x10]
    0x140082a37    41b913000000             mov              r9d, 0x13
    0x140082a3d    4889c2                   mov              rdx, rax
    0x140082a40    e80bfa2c00               call             0x140352450 -> _$LT$$RF$mut$u20$ron..ser..Serializer$LT$W$GT$$u20$as$u20$serde..ser..Serializer$GT$::serialize_struct::hdcf5544ce6145dd5
    0x140082a45    0fb645f0                 movzx            eax, byte ptr [rbp - 0x10]
    0x140082a49    3c32                     cmp              al, 0x32
    0x140082a4b    0f85f9000000             jne              0x140082b4a
    0x140082a51    488b4508                 mov              rax, qword ptr [rbp + 8]
    0x140082a55    0f1045f8                 movups           xmm0, xmmword ptr [rbp - 8]
    0x140082a59    0f2945d0                 movaps           xmmword ptr [rbp - 0x30], xmm0
    0x140082a5d    488945e0                 mov              qword ptr [rbp - 0x20], rax
    0x140082a61    c6454701                 mov              byte ptr [rbp + 0x47], 1
    0x140082a65    48897c2420               mov              qword ptr [rsp + 0x20], rdi
    0x140082a6a    4c8d0562dd8a02           lea              r8, [rip + 0x28add62] -> 0x1429307d3
    0x140082a71    488d4df0                 lea              rcx, [rbp - 0x10]
    0x140082a75    488d55d0                 lea              rdx, [rbp - 0x30]
    0x140082a79    41b913000000             mov              r9d, 0x13
    0x140082a7f    e87c9f2c00               call             0x14034ca00 -> _$LT$ron..ser..Compound$LT$W$GT$$u20$as$u20$serde..ser..SerializeStruct$GT$::serialize_field::h3f638ae5314b3b46
    0x140082a84    0fb645f0                 movzx            eax, byte ptr [rbp - 0x10]
    0x140082a88    3c32                     cmp              al, 0x32
    0x140082a8a    0f85f6000000             jne              0x140082b86
    0x140082a90    488d4701                 lea              rax, [rdi + 1]
    0x140082a94    c6454701                 mov              byte ptr [rbp + 0x47], 1
    0x140082a98    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x140082a9d    4c8d0542dd8a02           lea              r8, [rip + 0x28add42] -> 0x1429307e6
    0x140082aa4    488d4df0                 lea              rcx, [rbp - 0x10]
    0x140082aa8    488d55d0                 lea              rdx, [rbp - 0x30]
    0x140082aac    41b910000000             mov              r9d, 0x10
    0x140082ab2    e8499f2c00               call             0x14034ca00 -> _$LT$ron..ser..Compound$LT$W$GT$$u20$as$u20$serde..ser..SerializeStruct$GT$::serialize_field::h3f638ae5314b3b46
    0x140082ab7    0fb645f0                 movzx            eax, byte ptr [rbp - 0x10]
    0x140082abb    3c32                     cmp              al, 0x32
    0x140082abd    0f85c3000000             jne              0x140082b86
    0x140082ac3    488d4702                 lea              rax, [rdi + 2]
    0x140082ac7    c6454701                 mov              byte ptr [rbp + 0x47], 1
    0x140082acb    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x140082ad0    4c8d051fdd8a02           lea              r8, [rip + 0x28add1f] -> 0x1429307f6
    0x140082ad7    488d4df0                 lea              rcx, [rbp - 0x10]
    0x140082adb    488d55d0                 lea              rdx, [rbp - 0x30]
    0x140082adf    41b914000000             mov              r9d, 0x14
    0x140082ae5    e8169f2c00               call             0x14034ca00 -> _$LT$ron..ser..Compound$LT$W$GT$$u20$as$u20$serde..ser..SerializeStruct$GT$::serialize_field::h3f638ae5314b3b46
    0x140082aea    0fb645f0                 movzx            eax, byte ptr [rbp - 0x10]
    0x140082aee    3c32                     cmp              al, 0x32
    0x140082af0    0f8590000000             jne              0x140082b86
    0x140082af6    4883c703                 add              rdi, 3
    0x140082afa    c6454701                 mov              byte ptr [rbp + 0x47], 1
    0x140082afe    48897c2420               mov              qword ptr [rsp + 0x20], rdi
    0x140082b03    4c8d0500dd8a02           lea              r8, [rip + 0x28add00] -> 0x14293080a
    0x140082b0a    488d4df0                 lea              rcx, [rbp - 0x10]
    0x140082b0e    488d55d0                 lea              rdx, [rbp - 0x30]
    0x140082b12    41b913000000             mov              r9d, 0x13
    0x140082b18    e8e39e2c00               call             0x14034ca00 -> _$LT$ron..ser..Compound$LT$W$GT$$u20$as$u20$serde..ser..SerializeStruct$GT$::serialize_field::h3f638ae5314b3b46
    0x140082b1d    0fb645f0                 movzx            eax, byte ptr [rbp - 0x10]
    0x140082b21    3c32                     cmp              al, 0x32
    0x140082b23    7561                     jne              0x140082b86
    0x140082b25    488b45e0                 mov              rax, qword ptr [rbp - 0x20]
    0x140082b29    48894500                 mov              qword ptr [rbp], rax
    0x140082b2d    0f2845d0                 movaps           xmm0, xmmword ptr [rbp - 0x30]
    0x140082b31    0f2945f0                 movaps           xmmword ptr [rbp - 0x10], xmm0
    0x140082b35    c6454700                 mov              byte ptr [rbp + 0x47], 0
    0x140082b39    488d55f0                 lea              rdx, [rbp - 0x10]
    0x140082b3d    4889f1                   mov              rcx, rsi
    0x140082b40    e8dbee2c00               call             0x140351a20 -> _$LT$ron..ser..Compound$LT$W$GT$$u20$as$u20$serde..ser..SerializeStruct$GT$::end::hcd533543cf4ddee0
    0x140082b45    e985000000               jmp              0x140082bcf
    0x140082b4a    0f1045f1                 movups           xmm0, xmmword ptr [rbp - 0xf]
    0x140082b4e    0f104d00                 movups           xmm1, xmmword ptr [rbp]
    0x140082b52    0f105510                 movups           xmm2, xmmword ptr [rbp + 0x10]
    0x140082b56    0f105d20                 movups           xmm3, xmmword ptr [rbp + 0x20]
    0x140082b5a    0f114dbf                 movups           xmmword ptr [rbp - 0x41], xmm1
    0x140082b5e    0f2945b0                 movaps           xmmword ptr [rbp - 0x50], xmm0
    0x140082b62    488b4d30                 mov              rcx, qword ptr [rbp + 0x30]
    0x140082b66    48894e40                 mov              qword ptr [rsi + 0x40], rcx
    0x140082b6a    0f115e30                 movups           xmmword ptr [rsi + 0x30], xmm3
    0x140082b6e    0f115620                 movups           xmmword ptr [rsi + 0x20], xmm2
    0x140082b72    0f1045bf                 movups           xmm0, xmmword ptr [rbp - 0x41]
    0x140082b76    0f114610                 movups           xmmword ptr [rsi + 0x10], xmm0
    0x140082b7a    0f2845b0                 movaps           xmm0, xmmword ptr [rbp - 0x50]
    0x140082b7e    0f114601                 movups           xmmword ptr [rsi + 1], xmm0
    0x140082b82    8806                     mov              byte ptr [rsi], al
    0x140082b84    eb49                     jmp              0x140082bcf
    0x140082b86    488b4d30                 mov              rcx, qword ptr [rbp + 0x30]
    0x140082b8a    48894e40                 mov              qword ptr [rsi + 0x40], rcx
    0x140082b8e    0f1045f1                 movups           xmm0, xmmword ptr [rbp - 0xf]
    0x140082b92    0f104d01                 movups           xmm1, xmmword ptr [rbp + 1]
    0x140082b96    0f105511                 movups           xmm2, xmmword ptr [rbp + 0x11]
    0x140082b9a    0f105d21                 movups           xmm3, xmmword ptr [rbp + 0x21]
    0x140082b9e    0f115e31                 movups           xmmword ptr [rsi + 0x31], xmm3
    0x140082ba2    0f115621                 movups           xmmword ptr [rsi + 0x21], xmm2
    0x140082ba6    0f114e11                 movups           xmmword ptr [rsi + 0x11], xmm1
    0x140082baa    0f114601                 movups           xmmword ptr [rsi + 1], xmm0
    0x140082bae    8806                     mov              byte ptr [rsi], al
    0x140082bb0    488b45d8                 mov              rax, qword ptr [rbp - 0x28]
    0x140082bb4    833801                   cmp              dword ptr [rax], 1
    0x140082bb7    7516                     jne              0x140082bcf
    0x140082bb9    488b4808                 mov              rcx, qword ptr [rax + 8]
    0x140082bbd    48ffc1                   inc              rcx
    0x140082bc0    48c7c2ffffffff           mov              rdx, 0xffffffffffffffff
    0x140082bc7    480f45d1                 cmovne           rdx, rcx
    0x140082bcb    48895008                 mov              qword ptr [rax + 8], rdx
    0x140082bcf    4889f0                   mov              rax, rsi
    0x140082bd2    4881c4d0000000           add              rsp, 0xd0
    0x140082bd9    5f                       pop              rdi
    0x140082bda    5e                       pop              rsi
    0x140082bdb    5d                       pop              rbp
    0x140082bdc    c3                       ret              

=== 0x14067dd80 size=4  rhapsody::raw::allocator::GpuAllocator::inner
    ; GpuAllocator::inner: `mov rax,rcx; ret` — identity (returns &self).  [high]
    0x14067dd80    4889c8                   mov              rax, rcx
    0x14067dd83    c3                       ret              

=== 0x14090dda0 size=401  rhapsody::raw::temp_list::TempList$LT$T$GT$::add::h98e0b1a7504fb6a4
    ; TempList<T>::add: list-append. Body decoded.  [medium]
    0x14090dda0    55                       push             rbp
    0x14090dda1    56                       push             rsi
    0x14090dda2    57                       push             rdi
    0x14090dda3    4881ecd0000000           sub              rsp, 0xd0
    0x14090ddaa    488dac2480000000         lea              rbp, [rsp + 0x80]
    0x14090ddb2    48c74548feffffff         mov              qword ptr [rbp + 0x48], 0xfffffffffffffffe
    0x14090ddba    4889cf                   mov              rdi, rcx
    0x14090ddbd    488b01                   mov              rax, qword ptr [rcx]
    0x14090ddc0    8b8890000000             mov              ecx, dword ptr [rax + 0x90]
    0x14090ddc6    4883f908                 cmp              rcx, 8
    0x14090ddca    7337                     jae              0x14090de03
    0x14090ddcc    4989c9                   mov              r9, rcx
    0x14090ddcf    49c1e104                 shl              r9, 4
    0x14090ddd3    4a89540810               mov              qword ptr [rax + r9 + 0x10], rdx
    0x14090ddd8    4e89440818               mov              qword ptr [rax + r9 + 0x18], r8
    0x14090dddd    ffc1                     inc              ecx
    0x14090dddf    898890000000             mov              dword ptr [rax + 0x90], ecx
    0x14090dde5    488b07                   mov              rax, qword ptr [rdi]
    0x14090dde8    8b9090000000             mov              edx, dword ptr [rax + 0x90]
    0x14090ddee    4885d2                   test             rdx, rdx
    0x14090ddf1    0f84a4000000             je               0x14090de9b
    0x14090ddf7    48c1e204                 shl              rdx, 4
    0x14090ddfb    4801d0                   add              rax, rdx
    0x14090ddfe    e98d000000               jmp              0x14090de90
    0x14090de03    488955b0                 mov              qword ptr [rbp - 0x50], rdx
    0x14090de07    4c8945b8                 mov              qword ptr [rbp - 0x48], r8
    0x14090de0b    c7453001000000           mov              dword ptr [rbp + 0x30], 1
    0x14090de12    48c745a000000000         mov              qword ptr [rbp - 0x60], 0
    0x14090de1a    0fb605e86dd202           movzx            eax, byte ptr [rip + 0x2d26de8] -> 0x143634c09
    0x14090de21    b998000000               mov              ecx, 0x98
    0x14090de26    ba08000000               mov              edx, 8
    0x14090de2b    e8e05dd0ff               call             0x140613c10 -> __rust_alloc
    0x14090de30    4885c0                   test             rax, rax
    0x14090de33    7479                     je               0x14090deae
    0x14090de35    4889c6                   mov              rsi, rax
    0x14090de38    488d55a0                 lea              rdx, [rbp - 0x60]
    0x14090de3c    41b898000000             mov              r8d, 0x98
    0x14090de42    4889c1                   mov              rcx, rax
    0x14090de45    e8b65d0102               call             0x142923c00 -> memcpy
    0x14090de4a    488b07                   mov              rax, qword ptr [rdi]
    0x14090de4d    488937                   mov              qword ptr [rdi], rsi
    0x14090de50    48833e00                 cmp              qword ptr [rsi], 0
    0x14090de54    741f                     je               0x14090de75
    0x14090de56    48894540                 mov              qword ptr [rbp + 0x40], rax
    0x14090de5a    48897d38                 mov              qword ptr [rbp + 0x38], rdi
    0x14090de5e    4883c608                 add              rsi, 8
    0x14090de62    4889f1                   mov              rcx, rsi
    0x14090de65    e866cff6ff               call             0x14087add0 -> core::ptr::drop_in_place$LT$alloc..boxed..Box$LT$rhapsody..raw..temp_list..TempListInner$LT$alloc..ffi..c_str..CString$GT$$GT$$GT$::hcb1f6614e60567ef (.llvm.12095432060108768980)
    0x14090de6a    488b7d38                 mov              rdi, qword ptr [rbp + 0x38]
    0x14090de6e    488b37                   mov              rsi, qword ptr [rdi]
    0x14090de71    488b4540                 mov              rax, qword ptr [rbp + 0x40]
    0x14090de75    48c70601000000           mov              qword ptr [rsi], 1
    0x14090de7c    48894608                 mov              qword ptr [rsi + 8], rax
    0x14090de80    488b07                   mov              rax, qword ptr [rdi]
    0x14090de83    83b89000000000           cmp              dword ptr [rax + 0x90], 0
    0x14090de8a    7433                     je               0x14090debf
    0x14090de8c    4883c010                 add              rax, 0x10
    0x14090de90    4881c4d0000000           add              rsp, 0xd0
    0x14090de97    5f                       pop              rdi
    0x14090de98    5e                       pop              rsi
    0x14090de99    5d                       pop              rbp
    0x14090de9a    c3                       ret              
    0x14090de9b    4c8d05bed71902           lea              r8, [rip + 0x219d7be] -> 0x142aab660
    0x14090dea2    48c7c1ffffffff           mov              rcx, 0xffffffffffffffff
    0x14090dea9    e86ab6fc01               call             0x1428d9518 -> core::panicking::panic_bounds_check::h213dd6812833a70b
    0x14090deae    b908000000               mov              ecx, 8
    0x14090deb3    ba98000000               mov              edx, 0x98
    0x14090deb8    e843b1fc01               call             0x1428d9000 -> alloc::alloc::handle_alloc_error::h3e7daf9bcd04547a
    0x14090debd    0f0b                     ud2              
    0x14090debf    4c8d0582d71902           lea              r8, [rip + 0x219d782] -> 0x142aab648
    0x14090dec6    31c9                     xor              ecx, ecx
    0x14090dec8    31d2                     xor              edx, edx
    0x14090deca    e849b6fc01               call             0x1428d9518 -> core::panicking::panic_bounds_check::h213dd6812833a70b
    0x14090decf    cc                       int3             
    0x14090ded0    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x14090ded5    55                       push             rbp
    0x14090ded6    56                       push             rsi
    0x14090ded7    57                       push             rdi
    0x14090ded8    4883ec20                 sub              rsp, 0x20
    0x14090dedc    488daa80000000           lea              rbp, [rdx + 0x80]
    0x14090dee3    488d4da0                 lea              rcx, [rbp - 0x60]
    0x14090dee7    e804a7f8ff               call             0x1408985f0 -> core::ptr::drop_in_place$LT$rhapsody..raw..temp_list..TempListInner$LT$alloc..ffi..c_str..CString$GT$$GT$::h70f8ccaba243ecdb
    0x14090deec    90                       nop              
    0x14090deed    4883c420                 add              rsp, 0x20
    0x14090def1    5f                       pop              rdi
    0x14090def2    5e                       pop              rsi
    0x14090def3    5d                       pop              rbp
    0x14090def4    c3                       ret              
    0x14090def5    66662e0f1f840000000000   nop              word ptr cs:[rax + rax]
    0x14090df00    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x14090df05    55                       push             rbp
    0x14090df06    56                       push             rsi
    0x14090df07    57                       push             rdi
    0x14090df08    4883ec20                 sub              rsp, 0x20
    0x14090df0c    488daa80000000           lea              rbp, [rdx + 0x80]
    0x14090df13    488b4538                 mov              rax, qword ptr [rbp + 0x38]
    0x14090df17    488b00                   mov              rax, qword ptr [rax]
    0x14090df1a    48c70001000000           mov              qword ptr [rax], 1
    0x14090df21    488b4d40                 mov              rcx, qword ptr [rbp + 0x40]
    0x14090df25    48894808                 mov              qword ptr [rax + 8], rcx
    0x14090df29    4883c420                 add              rsp, 0x20
    0x14090df2d    5f                       pop              rdi
    0x14090df2e    5e                       pop              rsi
    0x14090df2f    5d                       pop              rbp
    0x14090df30    c3                       ret              

=== 0x140a9fa70 size=7  rhapsody::raw::init::AvailableGraphicsDeviceInitState::is_error
    ; AvailableGraphicsDeviceInitState::is_error: (byte[rcx]==1).  [high]
    0x140a9fa70    803901                   cmp              byte ptr [rcx], 1
    0x140a9fa73    0f94c0                   sete             al
    0x140a9fa76    c3                       ret              

=== 0x140bdd3f0 size=416  rhapsody::raw::error::impl$3::fmt
    ; raw::error::impl$3::fmt: Debug for a boxed error chain (discriminant-based variant selection). Diagnostic.  [medium]
    0x140bdd3f0    4881ec98000000           sub              rsp, 0x98
    0x140bdd3f7    4c8b01                   mov              r8, qword ptr [rcx]
    0x140bdd3fa    4d8d48f7                 lea              r9, [r8 - 9]
    0x140bdd3fe    4983c0f8                 add              r8, -8
    0x140bdd402    31c0                     xor              eax, eax
    0x140bdd404    4983f902                 cmp              r9, 2
    0x140bdd408    490f42c0                 cmovb            rax, r8
    0x140bdd40c    4885c0                   test             rax, rax
    0x140bdd40f    747d                     je               0x140bdd48e
    0x140bdd411    4883f801                 cmp              rax, 1
    0x140bdd415    0f850a010000             jne              0x140bdd525
    0x140bdd41b    488d4120                 lea              rax, [rcx + 0x20]
    0x140bdd41f    4889442428               mov              qword ptr [rsp + 0x28], rax
    0x140bdd424    4883c108                 add              rcx, 8
    0x140bdd428    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x140bdd42d    488d442428               lea              rax, [rsp + 0x28]
    0x140bdd432    4889442460               mov              qword ptr [rsp + 0x60], rax
    0x140bdd437    488d054209d3ff           lea              rax, [rip - 0x2cf6be] -> _$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$::fmt::h3a716608ac1a9da7
    0x140bdd43e    4889442468               mov              qword ptr [rsp + 0x68], rax
    0x140bdd443    488d442420               lea              rax, [rsp + 0x20]
    0x140bdd448    4889442470               mov              qword ptr [rsp + 0x70], rax
    0x140bdd44d    488d050c3570ff           lea              rax, [rip - 0x8fcaf4] -> _$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$::fmt::hc46075afe06100b7
    0x140bdd454    4889442478               mov              qword ptr [rsp + 0x78], rax
    0x140bdd459    488b4a30                 mov              rcx, qword ptr [rdx + 0x30]
    0x140bdd45d    488b5238                 mov              rdx, qword ptr [rdx + 0x38]
    0x140bdd461    488d057055f301           lea              rax, [rip + 0x1f35570] -> 0x142b129d8
    0x140bdd468    4889442430               mov              qword ptr [rsp + 0x30], rax
    0x140bdd46d    48c744243802000000       mov              qword ptr [rsp + 0x38], 2
    0x140bdd476    488d442460               lea              rax, [rsp + 0x60]
    0x140bdd47b    4889442440               mov              qword ptr [rsp + 0x40], rax
    0x140bdd480    48c744244802000000       mov              qword ptr [rsp + 0x48], 2
    0x140bdd489    e9e6000000               jmp              0x140bdd574
    0x140bdd48e    48898c2490000000         mov              qword ptr [rsp + 0x90], rcx
    0x140bdd496    488d4120                 lea              rax, [rcx + 0x20]
    0x140bdd49a    4889442428               mov              qword ptr [rsp + 0x28], rax
    0x140bdd49f    4883c138                 add              rcx, 0x38
    0x140bdd4a3    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x140bdd4a8    488d442428               lea              rax, [rsp + 0x28]
    0x140bdd4ad    4889442460               mov              qword ptr [rsp + 0x60], rax
    0x140bdd4b2    488d05276447ff           lea              rax, [rip - 0xb89bd9] -> _$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$::fmt::hd790704f60176ea8
    0x140bdd4b9    4889442468               mov              qword ptr [rsp + 0x68], rax
    0x140bdd4be    488d442420               lea              rax, [rsp + 0x20]
    0x140bdd4c3    4889442470               mov              qword ptr [rsp + 0x70], rax
    0x140bdd4c8    488d05b10ace00           lea              rax, [rip + 0xce0ab1] -> _$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$::fmt::h4abdced7edcbcfbe
    0x140bdd4cf    4889442478               mov              qword ptr [rsp + 0x78], rax
    0x140bdd4d4    488d842490000000         lea              rax, [rsp + 0x90]
    0x140bdd4dc    4889842480000000         mov              qword ptr [rsp + 0x80], rax
    0x140bdd4e4    488d05d50cce00           lea              rax, [rip + 0xce0cd5] -> _$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$::fmt::h9da929c99531807d
    0x140bdd4eb    4889842488000000         mov              qword ptr [rsp + 0x88], rax
    0x140bdd4f3    488b4a30                 mov              rcx, qword ptr [rdx + 0x30]
    0x140bdd4f7    488b5238                 mov              rdx, qword ptr [rdx + 0x38]
    0x140bdd4fb    488d059654f301           lea              rax, [rip + 0x1f35496] -> 0x142b12998
    0x140bdd502    4889442430               mov              qword ptr [rsp + 0x30], rax
    0x140bdd507    48c744243803000000       mov              qword ptr [rsp + 0x38], 3
    0x140bdd510    488d442460               lea              rax, [rsp + 0x60]
    0x140bdd515    4889442440               mov              qword ptr [rsp + 0x40], rax
    0x140bdd51a    48c744244803000000       mov              qword ptr [rsp + 0x48], 3
    0x140bdd523    eb4f                     jmp              0x140bdd574
    0x140bdd525    4883c108                 add              rcx, 8
    0x140bdd529    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x140bdd52e    488d442420               lea              rax, [rsp + 0x20]
    0x140bdd533    4889442460               mov              qword ptr [rsp + 0x60], rax
    0x140bdd538    488d05a16347ff           lea              rax, [rip - 0xb89c5f] -> _$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$::fmt::hd790704f60176ea8
    0x140bdd53f    4889442468               mov              qword ptr [rsp + 0x68], rax
    0x140bdd544    488b4a30                 mov              rcx, qword ptr [rdx + 0x30]
    0x140bdd548    488b5238                 mov              rdx, qword ptr [rdx + 0x38]
    0x140bdd54c    488d05c554f301           lea              rax, [rip + 0x1f354c5] -> 0x142b12a18
    0x140bdd553    4889442430               mov              qword ptr [rsp + 0x30], rax
    0x140bdd558    48c744243801000000       mov              qword ptr [rsp + 0x38], 1
    0x140bdd561    488d442460               lea              rax, [rsp + 0x60]
    0x140bdd566    4889442440               mov              qword ptr [rsp + 0x40], rax
    0x140bdd56b    48c744244801000000       mov              qword ptr [rsp + 0x48], 1
    0x140bdd574    48c744245000000000       mov              qword ptr [rsp + 0x50], 0
    0x140bdd57d    4c8d442430               lea              r8, [rsp + 0x30]
    0x140bdd582    e849851a00               call             0x140d85ad0 -> core::fmt::write::h75516f97e1c7e596
    0x140bdd587    90                       nop              
    0x140bdd588    4881c498000000           add              rsp, 0x98
    0x140bdd58f    c3                       ret              

=== 0x140be2370 size=47  rhapsody::raw::command_buffer::impl$4::drop
    ; command_buffer::impl$4::drop: read [rcx]; if ==0 or ==3 -> return (no destroy); else panic. Guard against dropping a live command buffer.  [high]
    0x140be2370    4883ec28                 sub              rsp, 0x28
    0x140be2374    488b01                   mov              rax, qword ptr [rcx]
    0x140be2377    4885c0                   test             rax, rax
    0x140be237a    7405                     je               0x140be2381
    0x140be237c    83f803                   cmp              eax, 3
    0x140be237f    7505                     jne              0x140be2386
    0x140be2381    4883c428                 add              rsp, 0x28
    0x140be2385    c3                       ret              
    0x140be2386    488d0dcb07f301           lea              rcx, [rip + 0x1f307cb] -> 0x142b12b58
    0x140be238d    4c8d055c08f301           lea              r8, [rip + 0x1f3085c] -> 0x142b12bf0
    0x140be2394    ba6e000000               mov              edx, 0x6e
    0x140be2399    e89270cf01               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x140be239e    cc                       int3             

=== 0x140be2710 size=367  rhapsody::leek::impl$0::fmt
    ; leek::impl$0::fmt: Debug impl (diagnostic).  [low]
    0x140be2710    55                       push             rbp
    0x140be2711    4157                     push             r15
    0x140be2713    4156                     push             r14
    0x140be2715    4154                     push             r12
    0x140be2717    56                       push             rsi
    0x140be2718    57                       push             rdi
    0x140be2719    53                       push             rbx
    0x140be271a    4883ec70                 sub              rsp, 0x70
    0x140be271e    488d6c2470               lea              rbp, [rsp + 0x70]
    0x140be2723    48c745f8feffffff         mov              qword ptr [rbp - 8], 0xfffffffffffffffe
    0x140be272b    4889d6                   mov              rsi, rdx
    0x140be272e    48c745e000000000         mov              qword ptr [rbp - 0x20], 0
    0x140be2736    48c745e801000000         mov              qword ptr [rbp - 0x18], 1
    0x140be273e    48c745f000000000         mov              qword ptr [rbp - 0x10], 0
    0x140be2746    0f1001                   movups           xmm0, xmmword ptr [rcx]
    0x140be2749    0f2945c0                 movaps           xmmword ptr [rbp - 0x40], xmm0
    0x140be274d    31ff                     xor              edi, edi
    0x140be274f    488d1d9207f301           lea              rbx, [rip + 0x1f30792] -> 0x142b12ee8
    0x140be2756    4c8d75e0                 lea              r14, [rbp - 0x20]
    0x140be275a    4531ff                   xor              r15d, r15d
    0x140be275d    4983ff10                 cmp              r15, 0x10
    0x140be2761    7552                     jne              0x140be27b5
    0x140be2763    488b4de8                 mov              rcx, qword ptr [rbp - 0x18]
    0x140be2767    4889fa                   mov              rdx, rdi
    0x140be276a    4989f0                   mov              r8, rsi
    0x140be276d    e8ee5b1a00               call             0x140d88360 -> _$LT$str$u20$as$u20$core..fmt..Debug$GT$::fmt::h74a3e2a88eca3b3a
    0x140be2772    488b55e0                 mov              rdx, qword ptr [rbp - 0x20]
    0x140be2776    4885d2                   test             rdx, rdx
    0x140be2779    7413                     je               0x140be278e
    0x140be277b    488b4de8                 mov              rcx, qword ptr [rbp - 0x18]
    0x140be277f    41b801000000             mov              r8d, 1
    0x140be2785    89c3                     mov              ebx, eax
    0x140be2787    e89414a3ff               call             0x140613c20 -> __rust_dealloc
    0x140be278c    89d8                     mov              eax, ebx
    0x140be278e    4883c470                 add              rsp, 0x70
    0x140be2792    5b                       pop              rbx
    0x140be2793    5f                       pop              rdi
    0x140be2794    5e                       pop              rsi
    0x140be2795    415c                     pop              r12
    0x140be2797    415e                     pop              r14
    0x140be2799    415f                     pop              r15
    0x140be279b    5d                       pop              rbp
    0x140be279c    c3                       ret              
    0x140be279d    0f1f00                   nop              dword ptr [rax]
    0x140be27a0    488b45e8                 mov              rax, qword ptr [rbp - 0x18]
    0x140be27a4    44882438                 mov              byte ptr [rax + rdi], r12b
    0x140be27a8    48ffc7                   inc              rdi
    0x140be27ab    48897df0                 mov              qword ptr [rbp - 0x10], rdi
    0x140be27af    4983ff10                 cmp              r15, 0x10
    0x140be27b3    74ae                     je               0x140be2763
    0x140be27b5    460fb6643dc0             movzx            r12d, byte ptr [rbp + r15 - 0x40]
    0x140be27bb    49ffc7                   inc              r15
    0x140be27be    4584e4                   test             r12b, r12b
    0x140be27c1    74a0                     je               0x140be2763
    0x140be27c3    781b                     js               0x140be27e0
    0x140be27c5    483b7de0                 cmp              rdi, qword ptr [rbp - 0x20]
    0x140be27c9    75d5                     jne              0x140be27a0
    0x140be27cb    4c89f1                   mov              rcx, r14
    0x140be27ce    4889da                   mov              rdx, rbx
    0x140be27d1    e89a6d1900               call             0x140d79570 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h526e3d19dbc6f270
    0x140be27d6    ebc8                     jmp              0x140be27a0
    0x140be27d8    0f1f840000000000         nop              dword ptr [rax + rax]
    0x140be27e0    488b45e0                 mov              rax, qword ptr [rbp - 0x20]
    0x140be27e4    4829f8                   sub              rax, rdi
    0x140be27e7    4883f802                 cmp              rax, 2
    0x140be27eb    7622                     jbe              0x140be280f
    0x140be27ed    488b45e8                 mov              rax, qword ptr [rbp - 0x18]
    0x140be27f1    c644380293               mov              byte ptr [rax + rdi + 2], 0x93
    0x140be27f6    66c70438e29d             mov              word ptr [rax + rdi], 0x9de2
    0x140be27fc    4883c703                 add              rdi, 3
    0x140be2800    48897df0                 mov              qword ptr [rbp - 0x10], rdi
    0x140be2804    4983ff10                 cmp              r15, 0x10
    0x140be2808    75ab                     jne              0x140be27b5
    0x140be280a    e954ffffff               jmp              0x140be2763
    0x140be280f    48c744242001000000       mov              qword ptr [rsp + 0x20], 1
    0x140be2818    41b803000000             mov              r8d, 3
    0x140be281e    41b901000000             mov              r9d, 1
    0x140be2824    4c89f1                   mov              rcx, r14
    0x140be2827    4889fa                   mov              rdx, rdi
    0x140be282a    e81181ce01               call             0x1428ca940 -> alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::h9449bed9caa63c91
    0x140be282f    488b7df0                 mov              rdi, qword ptr [rbp - 0x10]
    0x140be2833    ebb8                     jmp              0x140be27ed
    0x140be2835    66662e0f1f840000000000   nop              word ptr cs:[rax + rax]
    0x140be2840    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x140be2845    55                       push             rbp
    0x140be2846    4157                     push             r15
    0x140be2848    4156                     push             r14
    0x140be284a    4154                     push             r12
    0x140be284c    56                       push             rsi
    0x140be284d    57                       push             rdi
    0x140be284e    53                       push             rbx
    0x140be284f    4883ec30                 sub              rsp, 0x30
    0x140be2853    488d6a70                 lea              rbp, [rdx + 0x70]
    0x140be2857    488b55e0                 mov              rdx, qword ptr [rbp - 0x20]
    0x140be285b    4885d2                   test             rdx, rdx
    0x140be285e    740f                     je               0x140be286f
    0x140be2860    488b4de8                 mov              rcx, qword ptr [rbp - 0x18]
    0x140be2864    41b801000000             mov              r8d, 1
    0x140be286a    e8b113a3ff               call             0x140613c20 -> __rust_dealloc
    0x140be286f    90                       nop              
    0x140be2870    4883c430                 add              rsp, 0x30
    0x140be2874    5b                       pop              rbx
    0x140be2875    5f                       pop              rdi
    0x140be2876    5e                       pop              rsi
    0x140be2877    415c                     pop              r12
    0x140be2879    415e                     pop              r14
    0x140be287b    415f                     pop              r15
    0x140be287d    5d                       pop              rbp
    0x140be287e    c3                       ret              

=== 0x140be2880 size=271  rhapsody::leek::impl$1::from
    ; leek::impl$1::from: UTF-8 decode of a byte slice into a 16-byte (char?) array with bounds check. Body fully decoded.  [medium]
    0x140be2880    56                       push             rsi
    0x140be2881    4883ec30                 sub              rsp, 0x30
    0x140be2885    0f57c0                   xorps            xmm0, xmm0
    0x140be2888    0f29442420               movaps           xmmword ptr [rsp + 0x20], xmm0
    0x140be288d    4d85c0                   test             r8, r8
    0x140be2890    0f84d3000000             je               0x140be2969
    0x140be2896    4901d0                   add              r8, rdx
    0x140be2899    31c0                     xor              eax, eax
    0x140be289b    eb0c                     jmp              0x140be28a9
    0x140be289d    0f1f00                   nop              dword ptr [rax]
    0x140be28a0    4c39c2                   cmp              rdx, r8
    0x140be28a3    0f84c0000000             je               0x140be2969
    0x140be28a9    440fb60a                 movzx            r9d, byte ptr [rdx]
    0x140be28ad    4584c9                   test             r9b, r9b
    0x140be28b0    780e                     js               0x140be28c0
    0x140be28b2    48ffc2                   inc              rdx
    0x140be28b5    e996000000               jmp              0x140be2950
    0x140be28ba    660f1f440000             nop              word ptr [rax + rax]
    0x140be28c0    4589ca                   mov              r10d, r9d
    0x140be28c3    4183e21f                 and              r10d, 0x1f
    0x140be28c7    0fb67201                 movzx            esi, byte ptr [rdx + 1]
    0x140be28cb    83e63f                   and              esi, 0x3f
    0x140be28ce    4180f9df                 cmp              r9b, 0xdf
    0x140be28d2    7645                     jbe              0x140be2919
    0x140be28d4    440fb65a02               movzx            r11d, byte ptr [rdx + 2]
    0x140be28d9    c1e606                   shl              esi, 6
    0x140be28dc    4183e33f                 and              r11d, 0x3f
    0x140be28e0    4109f3                   or               r11d, esi
    0x140be28e3    4180f9f0                 cmp              r9b, 0xf0
    0x140be28e7    724a                     jb               0x140be2933
    0x140be28e9    440fb64a03               movzx            r9d, byte ptr [rdx + 3]
    0x140be28ee    4183e207                 and              r10d, 7
    0x140be28f2    41c1e212                 shl              r10d, 0x12
    0x140be28f6    41c1e306                 shl              r11d, 6
    0x140be28fa    4183e13f                 and              r9d, 0x3f
    0x140be28fe    4509d9                   or               r9d, r11d
    0x140be2901    4509d1                   or               r9d, r10d
    0x140be2904    4181f900001100           cmp              r9d, 0x110000
    0x140be290b    745c                     je               0x140be2969
    0x140be290d    4883c204                 add              rdx, 4
    0x140be2911    4183f97f                 cmp              r9d, 0x7f
    0x140be2915    7789                     ja               0x140be28a0
    0x140be2917    eb37                     jmp              0x140be2950
    0x140be2919    4883c202                 add              rdx, 2
    0x140be291d    41c1e206                 shl              r10d, 6
    0x140be2921    4109f2                   or               r10d, esi
    0x140be2924    4589d1                   mov              r9d, r10d
    0x140be2927    4183f97f                 cmp              r9d, 0x7f
    0x140be292b    0f876fffffff             ja               0x140be28a0
    0x140be2931    eb1d                     jmp              0x140be2950
    0x140be2933    4883c203                 add              rdx, 3
    0x140be2937    41c1e20c                 shl              r10d, 0xc
    0x140be293b    4509d3                   or               r11d, r10d
    0x140be293e    4589d9                   mov              r9d, r11d
    0x140be2941    4183f97f                 cmp              r9d, 0x7f
    0x140be2945    0f8755ffffff             ja               0x140be28a0
    0x140be294b    0f1f440000               nop              dword ptr [rax + rax]
    0x140be2950    4883f810                 cmp              rax, 0x10
    0x140be2954    7324                     jae              0x140be297a
    0x140be2956    44884c0420               mov              byte ptr [rsp + rax + 0x20], r9b
    0x140be295b    4883f80f                 cmp              rax, 0xf
    0x140be295f    7408                     je               0x140be2969
    0x140be2961    48ffc0                   inc              rax
    0x140be2964    e937ffffff               jmp              0x140be28a0
    0x140be2969    0f28442420               movaps           xmm0, xmmword ptr [rsp + 0x20]
    0x140be296e    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140be2971    4889c8                   mov              rax, rcx
    0x140be2974    4883c430                 add              rsp, 0x30
    0x140be2978    5e                       pop              rsi
    0x140be2979    c3                       ret              
    0x140be297a    4c8d059f05f301           lea              r8, [rip + 0x1f3059f] -> 0x142b12f20
    0x140be2981    ba10000000               mov              edx, 0x10
    0x140be2986    4889c1                   mov              rcx, rax
    0x140be2989    e88a6bcf01               call             0x1428d9518 -> core::panicking::panic_bounds_check::h213dd6812833a70b
    0x140be298e    cc                       int3             

=== 0x140be2990 size=175  rhapsody::leek::impl$2::fmt
    ; leek::impl$2::fmt: Debug impl (diagnostic).  [low]
    0x140be2990    4881ec98000000           sub              rsp, 0x98
    0x140be2997    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x140be299c    488d05ada21a00           lea              rax, [rip + 0x1aa2ad] -> core::fmt::num::_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$::fmt::h99c425e376f2e768
    0x140be29a3    4889442428               mov              qword ptr [rsp + 0x28], rax
    0x140be29a8    48c744246002000000       mov              qword ptr [rsp + 0x60], 2
    0x140be29b1    48c744247000000000       mov              qword ptr [rsp + 0x70], 0
    0x140be29ba    48c744247808000000       mov              qword ptr [rsp + 0x78], 8
    0x140be29c3    48c784248000000000000000 mov              qword ptr [rsp + 0x80], 0
    0x140be29cf    48b82000000008000000     movabs           rax, 0x800000020
    0x140be29d9    4889842488000000         mov              qword ptr [rsp + 0x88], rax
    0x140be29e1    c684249000000003         mov              byte ptr [rsp + 0x90], 3
    0x140be29e9    488b4a30                 mov              rcx, qword ptr [rdx + 0x30]
    0x140be29ed    488b5238                 mov              rdx, qword ptr [rdx + 0x38]
    0x140be29f1    488d054805f301           lea              rax, [rip + 0x1f30548] -> 0x142b12f40
    0x140be29f8    4889442430               mov              qword ptr [rsp + 0x30], rax
    0x140be29fd    48c744243801000000       mov              qword ptr [rsp + 0x38], 1
    0x140be2a06    488d442420               lea              rax, [rsp + 0x20]
    0x140be2a0b    4889442440               mov              qword ptr [rsp + 0x40], rax
    0x140be2a10    48c744244801000000       mov              qword ptr [rsp + 0x48], 1
    0x140be2a19    488d442460               lea              rax, [rsp + 0x60]
    0x140be2a1e    4889442450               mov              qword ptr [rsp + 0x50], rax
    0x140be2a23    48c744245801000000       mov              qword ptr [rsp + 0x58], 1
    0x140be2a2c    4c8d442430               lea              r8, [rsp + 0x30]
    0x140be2a31    e89a301a00               call             0x140d85ad0 -> core::fmt::write::h75516f97e1c7e596
    0x140be2a36    90                       nop              
    0x140be2a37    4881c498000000           add              rsp, 0x98
    0x140be2a3e    c3                       ret              

=== 0x140be2ad0 size=50  rhapsody::leek::begin_rg_pass
    ; leek::begin_rg_pass: copies 16B from rcx, appends tag byte 0x2a with payload; RingBuffer::append(global@0x143634e40, rec, 0x15). Trace marker (0x2a).  [high]
    0x140be2ad0    4883ec38                 sub              rsp, 0x38
    0x140be2ad4    0f1001                   movups           xmm0, xmmword ptr [rcx]
    0x140be2ad7    0f29442420               movaps           xmmword ptr [rsp + 0x20], xmm0
    0x140be2adc    89542430                 mov              dword ptr [rsp + 0x30], edx
    0x140be2ae0    c64424342a               mov              byte ptr [rsp + 0x34], 0x2a
    0x140be2ae5    488d0d5423a502           lea              rcx, [rip + 0x2a52354] -> 0x143634e40
    0x140be2aec    488d542420               lea              rdx, [rsp + 0x20]
    0x140be2af1    41b815000000             mov              r8d, 0x15
    0x140be2af7    e884e8cd00               call             0x1418c1380 -> leekspin::ringbuffer::RingBuffer::append::h526d1752a60c2257
    0x140be2afc    90                       nop              
    0x140be2afd    4883c438                 add              rsp, 0x38
    0x140be2b01    c3                       ret              

=== 0x140be2b10 size=38  rhapsody::leek::end_rg_pass
    ; leek::end_rg_pass: RingBuffer::append(global, {0x2b}, 1). Trace marker 0x2b.  [high]
    0x140be2b10    4883ec28                 sub              rsp, 0x28
    0x140be2b14    c64424272b               mov              byte ptr [rsp + 0x27], 0x2b
    0x140be2b19    488d0d2023a502           lea              rcx, [rip + 0x2a52320] -> 0x143634e40
    0x140be2b20    488d542427               lea              rdx, [rsp + 0x27]
    0x140be2b25    41b801000000             mov              r8d, 1
    0x140be2b2b    e850e8cd00               call             0x1418c1380 -> leekspin::ringbuffer::RingBuffer::append::h526d1752a60c2257
    0x140be2b30    90                       nop              
    0x140be2b31    4883c428                 add              rsp, 0x28
    0x140be2b35    c3                       ret              

=== 0x140be6a90 size=173  rhapsody::init_logging
    ; init_logging: logging initializer (body decoded).  [low]
    0x140be6a90    55                       push             rbp
    0x140be6a91    4883ec50                 sub              rsp, 0x50
    0x140be6a95    488d6c2450               lea              rbp, [rsp + 0x50]
    0x140be6a9a    48c745f8feffffff         mov              qword ptr [rbp - 8], 0xfffffffffffffffe
    0x140be6aa2    488d4dd0                 lea              rcx, [rbp - 0x30]
    0x140be6aa6    ba00000100               mov              edx, 0x10000
    0x140be6aab    e860a8cd00               call             0x1418c1310 -> leekspin::ringbuffer::RingBuffer::new::hfbc3136f54dc9e32
    0x140be6ab0    488b0d89e3a402           mov              rcx, qword ptr [rip + 0x2a4e389] -> 0x143634e40
    0x140be6ab7    488b158ae3a402           mov              rdx, qword ptr [rip + 0x2a4e38a] -> 0x143634e48
    0x140be6abe    488b45e0                 mov              rax, qword ptr [rbp - 0x20]
    0x140be6ac2    48890587e3a402           mov              qword ptr [rip + 0x2a4e387], rax -> 0x143634e50
    0x140be6ac9    0f1045d0                 movups           xmm0, xmmword ptr [rbp - 0x30]
    0x140be6acd    0f11056ce3a402           movups           xmmword ptr [rip + 0x2a4e36c], xmm0 -> 0x143634e40
    0x140be6ad4    4885c9                   test             rcx, rcx
    0x140be6ad7    7506                     jne              0x140be6adf
    0x140be6ad9    4883c450                 add              rsp, 0x50
    0x140be6add    5d                       pop              rbp
    0x140be6ade    c3                       ret              
    0x140be6adf    488955e8                 mov              qword ptr [rbp - 0x18], rdx
    0x140be6ae3    48894df0                 mov              qword ptr [rbp - 0x10], rcx
    0x140be6ae7    488d0d62c4f201           lea              rcx, [rip + 0x1f2c462] -> 0x142b12f50
    0x140be6aee    4c8d057bc4f201           lea              r8, [rip + 0x1f2c47b] -> 0x142b12f70
    0x140be6af5    ba20000000               mov              edx, 0x20
    0x140be6afa    e83129cf01               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x140be6aff    0f0b                     ud2              
    0x140be6b01    6666666666662e0f1f840000000000 nop              word ptr cs:[rax + rax]
    0x140be6b10    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x140be6b15    55                       push             rbp
    0x140be6b16    4883ec20                 sub              rsp, 0x20
    0x140be6b1a    488d6a50                 lea              rbp, [rdx + 0x50]
    0x140be6b1e    488b55e8                 mov              rdx, qword ptr [rbp - 0x18]
    0x140be6b22    4885d2                   test             rdx, rdx
    0x140be6b25    740f                     je               0x140be6b36
    0x140be6b27    41b801000000             mov              r8d, 1
    0x140be6b2d    488b4df0                 mov              rcx, qword ptr [rbp - 0x10]
    0x140be6b31    e8ead0a2ff               call             0x140613c20 -> __rust_dealloc
    0x140be6b36    90                       nop              
    0x140be6b37    4883c420                 add              rsp, 0x20
    0x140be6b3b    5d                       pop              rbp
    0x140be6b3c    c3                       ret              

=== 0x140be6b40 size=5  rhapsody::print_log_dump
    ; print_log_dump: `jmp leekspin_print_dump`.  [high]
    0x140be6b40    e9fbbfffff               jmp              0x140be2b40 -> rhapsody::leek::leekspin_print_dump

=== 0x140be6b50 size=52  rhapsody::dump_log
    ; dump_log: if global [0x143634e40]!=0 -> RingBuffer::to_inner(&global) else unwrap_failed. Flushes the leekspin ring log.  [high]
    0x140be6b50    56                       push             rsi
    0x140be6b51    4883ec20                 sub              rsp, 0x20
    0x140be6b55    48833de3e2a40200         cmp              qword ptr [rip + 0x2a4e2e3], 0 -> 0x143634e40
    0x140be6b5d    7418                     je               0x140be6b77
    0x140be6b5f    4889ce                   mov              rsi, rcx
    0x140be6b62    488d15d7e2a402           lea              rdx, [rip + 0x2a4e2d7] -> 0x143634e40
    0x140be6b69    e8c2a8cd00               call             0x1418c1430 -> leekspin::ringbuffer::RingBuffer::to_inner::hc2f6f86ac706db4b
    0x140be6b6e    4889f0                   mov              rax, rsi
    0x140be6b71    4883c420                 add              rsp, 0x20
    0x140be6b75    5e                       pop              rsi
    0x140be6b76    c3                       ret              
    0x140be6b77    488d0df2c3f201           lea              rcx, [rip + 0x1f2c3f2] -> 0x142b12f70
    0x140be6b7e    e88d27cf01               call             0x1428d9310 -> core::option::unwrap_failed::h837cefa1ab459360
    0x140be6b83    cc                       int3             

=== 0x140bed620 size=365  rhapsody::raw::command_buffer::impl$11::fmt
    ; command_buffer::impl$11::fmt: Debug for the command-buffer state enum; names include 'Initial','Recording','Recorded','Submitted','Universal' and fields 'queue','seq'. Diagnostic.  [medium]
    0x140bed620    4883ec68                 sub              rsp, 0x68
    0x140bed624    4889d0                   mov              rax, rdx
    0x140bed627    488b11                   mov              rdx, qword ptr [rcx]
    0x140bed62a    4c8d053f78f201           lea              r8, [rip + 0x1f2783f] -> 0x142b14e70
    0x140bed631    49631490                 movsxd           rdx, dword ptr [r8 + rdx*4]
    0x140bed635    4c01c2                   add              rdx, r8
    0x140bed638    ffe2                     jmp              rdx
    0x140bed63a    488d158f77f201           lea              rdx, [rip + 0x1f2778f] -> 0x142b14dd0
    0x140bed641    41b807000000             mov              r8d, 7
    0x140bed647    4889c1                   mov              rcx, rax
    0x140bed64a    4883c468                 add              rsp, 0x68
    0x140bed64e    e93d951900               jmp              0x140d86b90 -> _$LT$core..fmt..Formatter$u20$as$u20$core..fmt..Write$GT$::write_str::h5c3f9ed06cfb23bd
    0x140bed653    488d5110                 lea              rdx, [rcx + 0x10]
    0x140bed657    4883c108                 add              rcx, 8
    0x140bed65b    48894c2460               mov              qword ptr [rsp + 0x60], rcx
    0x140bed660    488d0d9177f201           lea              rcx, [rip + 0x1f27791] -> 0x142b14df8
    0x140bed667    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x140bed66c    488d4c2460               lea              rcx, [rsp + 0x60]
    0x140bed671    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x140bed676    488d0da977f201           lea              rcx, [rip + 0x1f277a9] -> 0x142b14e26
    0x140bed67d    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x140bed682    488d0d4f77f201           lea              rcx, [rip + 0x1f2774f] -> 0x142b14dd8
    0x140bed689    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140bed68e    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x140bed693    48c744244003000000       mov              qword ptr [rsp + 0x40], 3
    0x140bed69c    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x140bed6a5    488d157d77f201           lea              rdx, [rip + 0x1f2777d] -> 0x142b14e29
    0x140bed6ac    4c8d0d6e77f201           lea              r9, [rip + 0x1f2776e] -> 0x142b14e21
    0x140bed6b3    41b808000000             mov              r8d, 8
    0x140bed6b9    e9c1000000               jmp              0x140bed77f
    0x140bed6be    488d5110                 lea              rdx, [rcx + 0x10]
    0x140bed6c2    4883c108                 add              rcx, 8
    0x140bed6c6    48894c2460               mov              qword ptr [rsp + 0x60], rcx
    0x140bed6cb    488d0d2677f201           lea              rcx, [rip + 0x1f27726] -> 0x142b14df8
    0x140bed6d2    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x140bed6d7    488d4c2460               lea              rcx, [rsp + 0x60]
    0x140bed6dc    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x140bed6e1    488d0d3e77f201           lea              rcx, [rip + 0x1f2773e] -> 0x142b14e26
    0x140bed6e8    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x140bed6ed    488d0de476f201           lea              rcx, [rip + 0x1f276e4] -> 0x142b14dd8
    0x140bed6f4    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140bed6f9    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x140bed6fe    48c744244003000000       mov              qword ptr [rsp + 0x40], 3
    0x140bed707    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x140bed710    488d151a77f201           lea              rdx, [rip + 0x1f2771a] -> 0x142b14e31
    0x140bed717    eb59                     jmp              0x140bed772
    0x140bed719    488d5110                 lea              rdx, [rcx + 0x10]
    0x140bed71d    4883c108                 add              rcx, 8
    0x140bed721    48894c2460               mov              qword ptr [rsp + 0x60], rcx
    0x140bed726    488d0dcb76f201           lea              rcx, [rip + 0x1f276cb] -> 0x142b14df8
    0x140bed72d    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x140bed732    488d4c2460               lea              rcx, [rsp + 0x60]
    0x140bed737    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x140bed73c    488d0de376f201           lea              rcx, [rip + 0x1f276e3] -> 0x142b14e26
    0x140bed743    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x140bed748    488d0d8976f201           lea              rcx, [rip + 0x1f27689] -> 0x142b14dd8
    0x140bed74f    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140bed754    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x140bed759    48c744244003000000       mov              qword ptr [rsp + 0x40], 3
    0x140bed762    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x140bed76b    488d15a676f201           lea              rdx, [rip + 0x1f276a6] -> 0x142b14e18
    0x140bed772    4c8d0da876f201           lea              r9, [rip + 0x1f276a8] -> 0x142b14e21
    0x140bed779    41b809000000             mov              r8d, 9
    0x140bed77f    4889c1                   mov              rcx, rax
    0x140bed782    e819951900               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x140bed787    90                       nop              
    0x140bed788    4883c468                 add              rsp, 0x68
    0x140bed78c    c3                       ret              

=== 0x140c001c0 size=28  rhapsody::raw::physical_device::DeviceDriverVersion::vendor_string
    ; DeviceDriverVersion::vendor_string: table lookup by vendor dword -> &str. table@0x142b17d38/0x142b17d70 = {0:NVIDIA,1:Intel,2:Intel,3:AMD,4:Qualcomm,5:Apple,6:Unknown}.  [high]
    0x140c001c0    8b01                     mov              eax, dword ptr [rcx]
    0x140c001c2    488d0d6f7bf101           lea              rcx, [rip + 0x1f17b6f] -> 0x142b17d38
    0x140c001c9    488b14c1                 mov              rdx, qword ptr [rcx + rax*8]
    0x140c001cd    488d0d9c7bf101           lea              rcx, [rip + 0x1f17b9c] -> 0x142b17d70
    0x140c001d4    48630481                 movsxd           rax, dword ptr [rcx + rax*4]
    0x140c001d8    4801c8                   add              rax, rcx
    0x140c001db    c3                       ret              

=== 0x140c001e0 size=7  rhapsody::raw::physical_device::DeviceDriverVersion::is_amd
    ; DeviceDriverVersion::is_amd: (dword[rcx]==3).  [high]
    0x140c001e0    833903                   cmp              dword ptr [rcx], 3
    0x140c001e3    0f94c0                   sete             al
    0x140c001e6    c3                       ret              

=== 0x140c001f0 size=7  rhapsody::raw::physical_device::DeviceDriverVersion::is_apple
    ; DeviceDriverVersion::is_apple: (dword[rcx]==5).  [high]
    0x140c001f0    833905                   cmp              dword ptr [rcx], 5
    0x140c001f3    0f94c0                   sete             al
    0x140c001f6    c3                       ret              

=== 0x140c02280 size=5  rhapsody::raw::dynamic_constants::DynamicConstantsChunk::device_address
    ; DynamicConstantsChunk::device_address: returns qword [rcx+0x10].  [high]
    0x140c02280    488b4110                 mov              rax, qword ptr [rcx + 0x10]
    0x140c02284    c3                       ret              

=== 0x140c02290 size=19  rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_gpu_lifetime_tracker
    ; DynamicConstantsChunk::into_gpu_lifetime_tracker: copies [rdx+0x20],[rdx+0x30] -> out (0x20B).  [high]
    0x140c02290    4889c8                   mov              rax, rcx
    0x140c02293    0f104220                 movups           xmm0, xmmword ptr [rdx + 0x20]
    0x140c02297    0f104a30                 movups           xmm1, xmmword ptr [rdx + 0x30]
    0x140c0229b    0f114910                 movups           xmmword ptr [rcx + 0x10], xmm1
    0x140c0229f    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140c022a2    c3                       ret              

=== 0x140c02da0 size=34  rhapsody::raw::stats::RenderStats::snapshot_and_reset
    ; RenderStats::snapshot_and_reset: three 64-bit atomic xchg-0 on [rdx],[rdx+8],[rdx+0x10]; old values written to out[0..0x10]. 3 atomic u64 counters.  [high]
    0x140c02da0    4889c8                   mov              rax, rcx
    0x140c02da3    31c9                     xor              ecx, ecx
    0x140c02da5    48870a                   xchg             qword ptr [rdx], rcx
    0x140c02da8    4531c0                   xor              r8d, r8d
    0x140c02dab    4c874208                 xchg             qword ptr [rdx + 8], r8
    0x140c02daf    4531c9                   xor              r9d, r9d
    0x140c02db2    4c874a10                 xchg             qword ptr [rdx + 0x10], r9
    0x140c02db6    488908                   mov              qword ptr [rax], rcx
    0x140c02db9    4c894008                 mov              qword ptr [rax + 8], r8
    0x140c02dbd    4c894810                 mov              qword ptr [rax + 0x10], r9
    0x140c02dc1    c3                       ret              

=== 0x140c0e600 size=8  rhapsody::raw::allocator::GpuAllocator::peak_total_allocated_bytes
    ; GpuAllocator::peak_total_allocated_bytes: qword [rcx+0x680].  [high]
    0x140c0e600    488b8180060000           mov              rax, qword ptr [rcx + 0x680]
    0x140c0e607    c3                       ret              

=== 0x140c0e610 size=21  rhapsody::raw::allocator::GpuAllocator::peak_allocator_report
    ; GpuAllocator::peak_allocator_report: returns &(rcx+0x620) if its value !=0 else NULL (Option accessor).  [high]
    0x140c0e610    31d2                     xor              edx, edx
    0x140c0e612    483b9120060000           cmp              rdx, qword ptr [rcx + 0x620]
    0x140c0e619    488d8120060000           lea              rax, [rcx + 0x620]
    0x140c0e620    480f40c2                 cmovo            rax, rdx
    0x140c0e624    c3                       ret              

=== 0x140c0e630 size=8  rhapsody::raw::allocator::GpuAllocator::peak_device_local_bytes
    ; GpuAllocator::peak_device_local_bytes: qword [rcx+0x670].  [high]
    0x140c0e630    488b8170060000           mov              rax, qword ptr [rcx + 0x670]
    0x140c0e637    c3                       ret              

=== 0x140c0e640 size=8  rhapsody::raw::allocator::GpuAllocator::peak_system_bytes
    ; GpuAllocator::peak_system_bytes: qword [rcx+0x678].  [high]
    0x140c0e640    488b8178060000           mov              rax, qword ptr [rcx + 0x678]
    0x140c0e647    c3                       ret              
