0x140a57670 push     rbp
0x140a57671 push     r15
0x140a57673 push     r14
0x140a57675 push     rsi
0x140a57676 push     rdi
0x140a57677 push     rbx
0x140a57678 sub      rsp, 0x1f8
0x140a5767f lea      rbp, [rsp + 0x80]
0x140a57687 mov      qword ptr [rbp + 0x170], 0xfffffffffffffffe
0x140a57692 mov      r9, r8
0x140a57695 mov      qword ptr [rbp + 0x40], rdx
0x140a57699 mov      qword ptr [rbp + 0x48], r8
0x140a5769d cmp      dword ptr [rcx], 1
0x140a576a0 jne      0x140a57b0a
0x140a576a6 mov      r8, rdx
0x140a576a9 mov      rsi, rcx
0x140a576ac lea      rdx, [rcx + 8]
0x140a576b0 movss    dword ptr [rsp + 0x20], xmm3
0x140a576b6 mov      byte ptr [rsp + 0x28], 0
0x140a576bb lea      rcx, [rbp - 0x50]
0x140a576bf call     0x141437ce0 ; libfmod::EventInstance::set_parameter_by_name::hd5af5d129dad2630
0x140a576c4 movabs   rax, 0x8000000000000004
0x140a576ce cmp      qword ptr [rbp - 0x50], rax
0x140a576d2 je       0x140a57b0a
0x140a576d8 mov      rax, qword ptr [rbp - 0x20]
0x140a576dc mov      qword ptr [rbp + 0x30], rax
0x140a576e0 movups   xmm0, xmmword ptr [rbp - 0x50]
0x140a576e4 movups   xmm1, xmmword ptr [rbp - 0x40]
0x140a576e8 movups   xmm2, xmmword ptr [rbp - 0x30]
0x140a576ec movaps   xmmword ptr [rbp + 0x20], xmm2
0x140a576f0 movaps   xmmword ptr [rbp + 0x10], xmm1
0x140a576f4 movaps   xmmword ptr [rbp], xmm0
0x140a576f8 add      rsi, 0x10
0x140a576fc lea      rax, [rbp + 0x40]
0x140a57700 mov      qword ptr [rbp + 0x50], rax
0x140a57704 lea      rax, [rip - 0x523e6b]
0x140a5770b mov      qword ptr [rbp + 0x58], rax
0x140a5770f mov      qword ptr [rbp + 0x60], rsi
0x140a57713 lea      rax, [rip - 0x97a32a]
0x140a5771a mov      qword ptr [rbp + 0x68], rax
0x140a5771e mov      rax, rbp
0x140a57721 mov      qword ptr [rbp + 0x70], rax
0x140a57725 lea      rax, [rip - 0x4c3c]
0x140a5772c mov      qword ptr [rbp + 0x78], rax
0x140a57730 mov      qword ptr [rbp + 0x80], 2
0x140a5773b mov      qword ptr [rbp + 0x88], 2
0x140a57746 mov      qword ptr [rbp + 0x90], 2
0x140a57751 mov      qword ptr [rbp + 0xa0], 0
0x140a5775c mov      qword ptr [rbp + 0xa8], 0x20
0x140a57767 mov      byte ptr [rbp + 0xb0], 3
0x140a5776e mov      qword ptr [rbp + 0xb8], 2
0x140a57779 mov      qword ptr [rbp + 0xc0], 2
0x140a57784 mov      qword ptr [rbp + 0xc8], 2
0x140a5778f mov      qword ptr [rbp + 0xd8], 1
0x140a5779a mov      qword ptr [rbp + 0xe0], 0x20
0x140a577a5 mov      byte ptr [rbp + 0xe8], 3
0x140a577ac mov      qword ptr [rbp + 0xf0], 2
0x140a577b7 mov      qword ptr [rbp + 0xf8], 2
0x140a577c2 mov      qword ptr [rbp + 0x100], 2
0x140a577cd mov      qword ptr [rbp + 0x110], 2
0x140a577d8 movabs   rax, 0x400000020
0x140a577e2 mov      qword ptr [rbp + 0x118], rax
0x140a577e9 mov      byte ptr [rbp + 0x120], 3
0x140a577f0 lea      rax, [rip + 0x207b4d1]
0x140a577f7 mov      qword ptr [rbp + 0x130], rax
0x140a577fe mov      qword ptr [rbp + 0x138], 3
0x140a57809 lea      r15, [rbp + 0x50]
0x140a5780d mov      qword ptr [rbp + 0x140], r15
0x140a57814 mov      qword ptr [rbp + 0x148], 3
0x140a5781f lea      rax, [rbp + 0x80]
0x140a57826 mov      qword ptr [rbp + 0x150], rax
0x140a5782d mov      qword ptr [rbp + 0x158], 3
0x140a57838 lea      rcx, [rbp - 0x18]
0x140a5783c lea      rsi, [rbp + 0x130]
0x140a57843 mov      rdx, rsi
0x140a57846 call     0x140d79f90 ; alloc::fmt::format::format_inner::hdcc110f646744b43
0x140a5784b movups   xmm0, xmmword ptr [rbp - 0x18]
0x140a5784f movaps   xmmword ptr [rbp + 0x50], xmm0
0x140a57853 mov      rax, qword ptr [rbp - 8]
0x140a57857 mov      qword ptr [rbp + 0x60], rax
0x140a5785b mov      eax, dword ptr [rip + 0x2bdd50f]
0x140a57861 cmp      eax, 3
0x140a57864 jne      0x140a57b1b
0x140a5786a mov      rdx, qword ptr [rip + 0x2bdd4f7]
0x140a57871 lea      rcx, [rbp + 0x80]
0x140a57878 call     0x140542890 ; log_once::__MessagesSet::lock::hc1f0c63a0f714735 / std::sync::poison::mutex::Mutex$LT$T$GT$::lock::h89619656ab9d0de5
0x140a5787d cmp      dword ptr [rbp + 0x80], 1
0x140a57884 je       0x140a57ba6
0x140a5788a mov      rax, qword ptr [rbp + 0x88]
0x140a57891 mov      qword ptr [rbp + 0x128], rax
0x140a57898 movzx    eax, byte ptr [rbp + 0x90]
0x140a5789f mov      byte ptr [rbp + 0x16f], al
0x140a578a5 movzx    eax, byte ptr [rip + 0x2bdd35d]
0x140a578ac mov      ecx, 0xf
0x140a578b1 mov      edx, 1
0x140a578b6 call     0x140613c10 ; __rust_alloc
0x140a578bb test     rax, rax
0x140a578be je       0x140a57bee
0x140a578c4 movabs   rcx, 0x2928202168746170
0x140a578ce mov      qword ptr [rax + 7], rcx
0x140a578d2 movabs   rcx, 0x705f656c75646f6d
0x140a578dc mov      qword ptr [rax], rcx
0x140a578df mov      qword ptr [rbp + 0x80], 0xf
0x140a578ea mov      qword ptr [rbp + 0x88], rax
0x140a578f1 mov      qword ptr [rbp + 0x90], 0xf
0x140a578fc mov      qword ptr [rsp + 0x20], 1
0x140a57905 lea      rcx, [rbp + 0x80]
0x140a5790c mov      edx, 0xf
0x140a57911 mov      r8d, 0x17
0x140a57917 mov      r9d, 1
0x140a5791d call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140a57922 mov      rax, qword ptr [rbp + 0x88]
0x140a57929 mov      rdi, qword ptr [rbp + 0x90]
0x140a57930 movups   xmm0, xmmword ptr [rip + 0x207b124] ; @0x142ad2a5b '$crate :: Level :: Warn'
0x140a57937 movups   xmmword ptr [rax + rdi], xmm0
0x140a5793b movabs   rcx, 0x6e726157203a3a20
0x140a57945 mov      qword ptr [rax + rdi + 0xf], rcx
0x140a5794a add      rdi, 0x17
0x140a5794e mov      qword ptr [rbp + 0x140], rdi
0x140a57955 movups   xmm0, xmmword ptr [rbp + 0x80]
0x140a5795c movaps   xmmword ptr [rbp + 0x130], xmm0
0x140a57963 mov      r14, qword ptr [rbp + 0x58]
0x140a57967 mov      rbx, qword ptr [rbp + 0x60]
0x140a5796b mov      rax, qword ptr [rbp + 0x130]
0x140a57972 sub      rax, rdi
0x140a57975 cmp      rax, rbx
0x140a57978 jb       0x140a57b56
0x140a5797e mov      rcx, qword ptr [rbp + 0x138]
0x140a57985 add      rcx, rdi
0x140a57988 mov      rdx, r14
0x140a5798b mov      r8, rbx
0x140a5798e call     0x142923c00 ; memcpy
0x140a57993 add      rdi, rbx
0x140a57996 mov      qword ptr [rbp + 0x140], rdi
0x140a5799d mov      qword ptr [rbp + 0x90], rdi
0x140a579a4 movaps   xmm0, xmmword ptr [rbp + 0x130]
0x140a579ab movaps   xmmword ptr [rbp + 0x80], xmm0
0x140a579b2 mov      rax, qword ptr [rbp + 0x128]
0x140a579b9 lea      rcx, [rax + 8]
0x140a579bd lea      rdx, [rbp + 0x80]
0x140a579c4 call     0x14143b8b0 ; alloc::collections::btree::map::BTreeMap$LT$K$C$V$C$A$GT$::insert::h3b2d2ad0172fcd34
0x140a579c9 test     al, al
0x140a579cb jne      0x140a57ab8
0x140a579d1 mov      rax, qword ptr [rip + 0x2bd49d0]
0x140a579d8 mov      rax, qword ptr [rax]
0x140a579db cmp      rax, 2
0x140a579df jb       0x140a57ab8
0x140a579e5 mov      qword ptr [rbp + 0x130], r15
0x140a579ec lea      rax, [rip - 0xa1bf23]
0x140a579f3 mov      qword ptr [rbp + 0x138], rax
0x140a579fa lea      rcx, [rip + 0x207b2f7]
0x140a57a01 call     0x14067dd80 ; _$LT$country_core..render..ssbo..TypedStorageBuffer$LT$T$GT$$u20$as$u20$country_core..asset_libraries..ssbo_library..StorageBufferDyn$GT$::buffer::h55d27067a9534614 / _$LT$country_core..render..ssbo..TypedStorageBuffer$LT$T$GT$$u20$as$u20$country_core..asset_libraries..ssbo_library..StorageBufferDyn$GT$::buffer::h781648298f14b5a6
0x140a57a06 movups   xmm0, xmmword ptr [rax]
0x140a57a09 mov      eax, dword ptr [rax + 0x10]
0x140a57a0c mov      qword ptr [rbp + 0xb0], 2
0x140a57a17 lea      rcx, [rip + 0x207ab32] ; @0x142ad2550 'country_core::resources::audio_resources'
0x140a57a1e mov      qword ptr [rbp + 0xb8], rcx
0x140a57a25 mov      qword ptr [rbp + 0xc0], 0x28
0x140a57a30 lea      rdx, [rip + 0x207b041]
0x140a57a37 mov      qword ptr [rbp + 0xd0], rdx
0x140a57a3e mov      qword ptr [rbp + 0xd8], 1
0x140a57a49 mov      qword ptr [rbp + 0xe0], rsi
0x140a57a50 mov      qword ptr [rbp + 0xe8], 1
0x140a57a5b mov      qword ptr [rbp + 0xf0], 0
0x140a57a66 mov      qword ptr [rbp + 0x80], 0
0x140a57a71 mov      qword ptr [rbp + 0x88], rcx
0x140a57a78 mov      qword ptr [rbp + 0x90], 0x28
0x140a57a83 mov      qword ptr [rbp + 0x98], 0
0x140a57a8e movups   xmmword ptr [rbp + 0xa0], xmm0
0x140a57a95 mov      dword ptr [rbp + 0xc8], 1
0x140a57a9f mov      dword ptr [rbp + 0xcc], eax
0x140a57aa5 lea      rcx, [rbp + 0x16e]
0x140a57aac lea      rdx, [rbp + 0x80]
0x140a57ab3 call     0x140d4a330 ; _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log::h234cc9ac9918612c
0x140a57ab8 cmp      byte ptr [rbp + 0x16f], 0
0x140a57abf mov      rsi, qword ptr [rbp + 0x128]
0x140a57ac6 jne      0x140a57ade
0x140a57ac8 mov      rax, qword ptr [rip + 0x2bd4951]
0x140a57acf mov      rax, qword ptr [rax]
0x140a57ad2 shl      rax, 1
0x140a57ad5 test     rax, rax
0x140a57ad8 jne      0x140a57b90
0x140a57ade xor      eax, eax
0x140a57ae0 xchg     byte ptr [rsi], al
0x140a57ae2 cmp      al, 2
0x140a57ae4 je       0x140a57b83
0x140a57aea mov      rdx, qword ptr [rbp + 0x50]
0x140a57aee test     rdx, rdx
0x140a57af1 je       0x140a57b02
0x140a57af3 mov      rcx, qword ptr [rbp + 0x58]
0x140a57af7 mov      r8d, 1
0x140a57afd call     0x140613c20 ; __rust_dealloc
0x140a57b02 mov      rcx, rbp
0x140a57b05 call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x140a57b0a nop      
0x140a57b0b add      rsp, 0x1f8
0x140a57b12 pop      rbx
0x140a57b13 pop      rdi
0x140a57b14 pop      rsi
0x140a57b15 pop      r14
0x140a57b17 pop      r15
0x140a57b19 pop      rbp
0x140a57b1a ret      
0x140a57b1b mov      byte ptr [rbp + 0x130], 1
0x140a57b22 mov      qword ptr [rbp + 0x80], rsi
0x140a57b29 lea      rax, [rip + 0x207b1c8]
0x140a57b30 mov      qword ptr [rsp + 0x20], rax
0x140a57b35 lea      rcx, [rip + 0x2bdd234]
0x140a57b3c lea      r9, [rip + 0x207899d]
0x140a57b43 lea      r8, [rbp + 0x80]
0x140a57b4a xor      edx, edx
0x140a57b4c call     0x1428d8820 ; std::sys::sync::once::futex::Once::call::h1c537541047aece6
0x140a57b51 jmp      0x140a5786a ; 
0x140a57b56 mov      qword ptr [rsp + 0x20], 1
0x140a57b5f lea      rcx, [rbp + 0x130]
0x140a57b66 mov      r9d, 1
0x140a57b6c mov      rdx, rdi
0x140a57b6f mov      r8, rbx
0x140a57b72 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140a57b77 mov      rdi, qword ptr [rbp + 0x140]
0x140a57b7e jmp      0x140a5797e ; 
0x140a57b83 mov      rcx, rsi
0x140a57b86 call     0x1428d8810 ; std::sys::sync::mutex::futex::Mutex::wake::h6deb79e4d2d97c42
0x140a57b8b jmp      0x140a57aea ; 
0x140a57b90 call     0x1428d86b0 ; std::panicking::panic_count::is_zero_slow_path::hc6e505bc36aedeae
0x140a57b95 test     al, al
0x140a57b97 jne      0x140a57ade
0x140a57b9d mov      byte ptr [rsi + 1], 1
0x140a57ba1 jmp      0x140a57ade ; 
0x140a57ba6 mov      rax, qword ptr [rbp + 0x88]
0x140a57bad movzx    ecx, byte ptr [rbp + 0x90]
0x140a57bb4 mov      qword ptr [rbp + 0x130], rax
0x140a57bbb mov      byte ptr [rbp + 0x138], cl
0x140a57bc1 lea      rax, [rip + 0x207b130]
0x140a57bc8 mov      qword ptr [rsp + 0x20], rax
0x140a57bcd lea      rcx, [rip + 0x207ae74] ; @0x142ad2a48 'Mutex was poisonned$crate :: Level :: Warn'
0x140a57bd4 lea      r9, [rip + 0x20789c5] ; @0x142ad05a0 'p(]@'
0x140a57bdb lea      r8, [rbp + 0x130]
0x140a57be2 mov      edx, 0x13
0x140a57be7 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x140a57bec jmp      0x140a57c04 ; 
0x140a57bee lea      r8, [rip + 0x2078bbb]
0x140a57bf5 mov      ecx, 1
0x140a57bfa mov      edx, 0xf
0x140a57bff call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x140a57c04 ud2      
0x140a57c06 nop      word ptr cs:[rax + rax]
0x140a57c10 mov      qword ptr [rsp + 0x10], rdx
0x140a57c15 push     rbp
0x140a57c16 push     r15
0x140a57c18 push     r14
0x140a57c1a push     rsi
0x140a57c1b push     rdi
0x140a57c1c push     rbx
0x140a57c1d sub      rsp, 0x38
0x140a57c21 lea      rbp, [rdx + 0x80]
0x140a57c28 mov      rdx, qword ptr [rbp + 0x50]
0x140a57c2c test     rdx, rdx
0x140a57c2f je       0x140a57c40
0x140a57c31 mov      rcx, qword ptr [rbp + 0x58]
0x140a57c35 mov      r8d, 1
0x140a57c3b call     0x140613c20 ; __rust_dealloc
0x140a57c40 nop      
0x140a57c41 add      rsp, 0x38
0x140a57c45 pop      rbx
0x140a57c46 pop      rdi
0x140a57c47 pop      rsi
0x140a57c48 pop      r14
0x140a57c4a pop      r15
0x140a57c4c pop      rbp
0x140a57c4d ret      
0x140a57c4e nop      
0x140a57c50 mov      qword ptr [rsp + 0x10], rdx
0x140a57c55 push     rbp
0x140a57c56 push     r15
0x140a57c58 push     r14
0x140a57c5a push     rsi
0x140a57c5b push     rdi
0x140a57c5c push     rbx
0x140a57c5d sub      rsp, 0x38
0x140a57c61 lea      rbp, [rdx + 0x80]
0x140a57c68 lea      rcx, [rbp + 0x130]
0x140a57c6f call     0x1405d2870 ; core::ptr::drop_in_place$LT$std..sync..poison..PoisonError$LT$std..sync..poison..mutex..MutexGuard$LT$std..sync..mpmc..zero..Inner$GT$$GT$$GT$::h9ded1bd3ccc2294c (.llvm.4380053332811305371) / core::ptr::drop_in_place$LT$std..sync..poison..mutex..MutexGuard$LT$alloc..vec..Vec$LT$alloc..boxed..Box$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$core..cell..RefCell$LT$regex..exec..ProgramCacheInner$GT$$GT$$GT$$GT$$GT$$GT$::h1c92b00504c24a5f (.llvm.8877835209266785524)
0x140a57c74 nop      
0x140a57c75 add      rsp, 0x38
0x140a57c79 pop      rbx
0x140a57c7a pop      rdi
0x140a57c7b pop      rsi
0x140a57c7c pop      r14
0x140a57c7e pop      r15
0x140a57c80 pop      rbp
0x140a57c81 ret      
0x140a57c82 nop      word ptr cs:[rax + rax]
0x140a57c90 mov      qword ptr [rsp + 0x10], rdx
0x140a57c95 push     rbp
0x140a57c96 push     r15
0x140a57c98 push     r14
0x140a57c9a push     rsi
0x140a57c9b push     rdi
0x140a57c9c push     rbx
0x140a57c9d sub      rsp, 0x38
0x140a57ca1 lea      rbp, [rdx + 0x80]
0x140a57ca8 mov      rcx, qword ptr [rbp + 0x128]
0x140a57caf movzx    edx, byte ptr [rbp + 0x16f]
0x140a57cb6 call     0x140043000 ; core::ptr::drop_in_place$LT$std..sync..poison..mutex..MutexGuard$LT$std..sync..mpmc..zero..Inner$GT$$GT$::h91cb3a65d1ae3bd7 (.llvm.9642053767327545659) / core::ptr::drop_in_place$LT$std..sync..poison..mutex..MutexGuard$LT$std..sync..mpmc..zero..Inner$GT$$GT$::h95ee279bebe9c0d3 (.llvm.3835711356598153994)
0x140a57cbb nop      
0x140a57cbc add      rsp, 0x38
0x140a57cc0 pop      rbx
0x140a57cc1 pop      rdi
0x140a57cc2 pop      rsi
0x140a57cc3 pop      r14
0x140a57cc5 pop      r15
0x140a57cc7 pop      rbp
0x140a57cc8 ret      
0x140a57cc9 nop      dword ptr [rax]
0x140a57cd0 mov      qword ptr [rsp + 0x10], rdx
0x140a57cd5 push     rbp
0x140a57cd6 push     r15
0x140a57cd8 push     r14
0x140a57cda push     rsi
0x140a57cdb push     rdi
0x140a57cdc push     rbx
0x140a57cdd sub      rsp, 0x38
0x140a57ce1 lea      rbp, [rdx + 0x80]
0x140a57ce8 mov      rdx, qword ptr [rbp + 0x80]
0x140a57cef test     rdx, rdx
0x140a57cf2 je       0x140a57d06
0x140a57cf4 mov      rcx, qword ptr [rbp + 0x88]
0x140a57cfb mov      r8d, 1
0x140a57d01 call     0x140613c20 ; __rust_dealloc
0x140a57d06 nop      
0x140a57d07 add      rsp, 0x38
0x140a57d0b pop      rbx
0x140a57d0c pop      rdi
0x140a57d0d pop      rsi
0x140a57d0e pop      r14
0x140a57d10 pop      r15
0x140a57d12 pop      rbp
0x140a57d13 ret      
0x140a57d14 nop      word ptr cs:[rax + rax]
0x140a57d20 mov      qword ptr [rsp + 0x10], rdx
0x140a57d25 push     rbp
0x140a57d26 push     r15
0x140a57d28 push     r14
0x140a57d2a push     rsi
0x140a57d2b push     rdi
0x140a57d2c push     rbx
0x140a57d2d sub      rsp, 0x38
0x140a57d31 lea      rbp, [rdx + 0x80]
0x140a57d38 mov      rdx, qword ptr [rbp + 0x130]
0x140a57d3f test     rdx, rdx
0x140a57d42 je       0x140a57d56
0x140a57d44 mov      rcx, qword ptr [rbp + 0x138]
0x140a57d4b mov      r8d, 1
0x140a57d51 call     0x140613c20 ; __rust_dealloc
0x140a57d56 nop      
0x140a57d57 add      rsp, 0x38
0x140a57d5b pop      rbx
0x140a57d5c pop      rdi
0x140a57d5d pop      rsi
0x140a57d5e pop      r14
0x140a57d60 pop      r15
0x140a57d62 pop      rbp
0x140a57d63 ret      
0x140a57d64 nop      word ptr cs:[rax + rax]
0x140a57d70 mov      qword ptr [rsp + 0x10], rdx
0x140a57d75 push     rbp
0x140a57d76 push     r15
0x140a57d78 push     r14
0x140a57d7a push     rsi
0x140a57d7b push     rdi
0x140a57d7c push     rbx
0x140a57d7d sub      rsp, 0x38
0x140a57d81 lea      rbp, [rdx + 0x80]
0x140a57d88 mov      rcx, rbp
0x140a57d8b call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x140a57d90 nop      
0x140a57d91 add      rsp, 0x38
0x140a57d95 pop      rbx
0x140a57d96 pop      rdi
0x140a57d97 pop      rsi
0x140a57d98 pop      r14
0x140a57d9a pop      r15
0x140a57d9c pop      rbp
0x140a57d9d ret      