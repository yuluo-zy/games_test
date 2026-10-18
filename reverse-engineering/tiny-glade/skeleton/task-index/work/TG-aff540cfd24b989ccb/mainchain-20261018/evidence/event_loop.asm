0x14026e4a0 55 push rbp
0x14026e4a1 4157 push r15
0x14026e4a3 4156 push r14
0x14026e4a5 4155 push r13
0x14026e4a7 4154 push r12
0x14026e4a9 56 push rsi
0x14026e4aa 57 push rdi
0x14026e4ab 53 push rbx
0x14026e4ac 4881ecb8030000 sub rsp, 0x3b8
0x14026e4b3 488dac2480000000 lea rbp, [rsp + 0x80]
0x14026e4bb 440f298d20030000 movaps xmmword ptr [rbp + 0x320], xmm9
0x14026e4c3 440f298510030000 movaps xmmword ptr [rbp + 0x310], xmm8
0x14026e4cb 0f29bd00030000 movaps xmmword ptr [rbp + 0x300], xmm7
0x14026e4d2 0f29b5f0020000 movaps xmmword ptr [rbp + 0x2f0], xmm6
0x14026e4d9 48c785e8020000feffffff mov qword ptr [rbp + 0x2e8], 0xfffffffffffffffe
0x14026e4e4 4c8985e0010000 mov qword ptr [rbp + 0x1e0], r8
0x14026e4eb 48898dd8010000 mov qword ptr [rbp + 0x1d8], rcx
0x14026e4f2 660f1085a8030000 movupd xmm0, xmmword ptr [rbp + 0x3a8]
0x14026e4fa 488b85a0030000 mov rax, qword ptr [rbp + 0x3a0]
0x14026e501 488995c0020000 mov qword ptr [rbp + 0x2c0], rdx
0x14026e508 4c898dc8010000 mov qword ptr [rbp + 0x1c8], r9
0x14026e50f 488985d0010000 mov qword ptr [rbp + 0x1d0], rax
0x14026e516 660f118588000000 movupd xmmword ptr [rbp + 0x88], xmm0
0x14026e51e c785c401000000000000 mov dword ptr [rbp + 0x1c4], 0
0x14026e528 488b4a18 mov rcx, qword ptr [rdx + 0x18]
0x14026e52c e8cf954200 call 0x140697b00 ; winit::platform_impl::platform::monitor::current_monitor
0x14026e531 48c7457801000000 mov qword ptr [rbp + 0x78], 1
0x14026e539 48898580000000 mov qword ptr [rbp + 0x80], rax
0x14026e540 c685e302000001 mov byte ptr [rbp + 0x2e3], 1
0x14026e547 c7455402000000 mov dword ptr [rbp + 0x54], 2
0x14026e54e 48c7850002000000000000 mov qword ptr [rbp + 0x200], 0
0x14026e559 0f57f6 xorps xmm6, xmm6
0x14026e55c 0f11b510020000 movups xmmword ptr [rbp + 0x210], xmm6
0x14026e563 48c7850802000008000000 mov qword ptr [rbp + 0x208], 8
0x14026e56e c785ac02000000000000 mov dword ptr [rbp + 0x2ac], 0
0x14026e578 48c785e801000000000000 mov qword ptr [rbp + 0x1e8], 0
0x14026e583 c785f801000000000000 mov dword ptr [rbp + 0x1f8], 0
0x14026e58d e84e50af00 call 0x140d635e0 ; std::time::Instant::now
0x14026e592 48898508010000 mov qword ptr [rbp + 0x108], rax
0x14026e599 8995fc010000 mov dword ptr [rbp + 0x1fc], edx
0x14026e59f c685cf02000001 mov byte ptr [rbp + 0x2cf], 1
0x14026e5a6 488b85e0010000 mov rax, qword ptr [rbp + 0x1e0]
0x14026e5ad 4883c008 add rax, 8
0x14026e5b1 48898500010000 mov qword ptr [rbp + 0x100], rax
0x14026e5b8 4c8da520020000 lea r12, [rbp + 0x220]
0x14026e5bf 4c8d35a2977302 lea r14, [rip + 0x27397a2]
0x14026e5c6 488985f0010000 mov qword ptr [rbp + 0x1f0], rax
0x14026e5cd c785bc02000000000000 mov dword ptr [rbp + 0x2bc], 0
0x14026e5d7 80bdcf02000000 cmp byte ptr [rbp + 0x2cf], 0
0x14026e5de 0f843c140000 je 0x14026fa20
0x14026e5e4 488b8dd8010000 mov rcx, qword ptr [rbp + 0x1d8]
0x14026e5eb 488b4110 mov rax, qword ptr [rcx + 0x10]
0x14026e5ef c7401800ca9a3b mov dword ptr [rax + 0x18], 0x3b9aca00
0x14026e5f6 488b8500010000 mov rax, qword ptr [rbp + 0x100]
0x14026e5fd 48898520020000 mov qword ptr [rbp + 0x220], rax
0x14026e604 488d85c0020000 lea rax, [rbp + 0x2c0]
0x14026e60b 48898528020000 mov qword ptr [rbp + 0x228], rax
0x14026e612 488d85c8010000 lea rax, [rbp + 0x1c8]
0x14026e619 48898530020000 mov qword ptr [rbp + 0x230], rax
0x14026e620 488d85cf020000 lea rax, [rbp + 0x2cf]
0x14026e627 48898538020000 mov qword ptr [rbp + 0x238], rax
0x14026e62e 488d85c4010000 lea rax, [rbp + 0x1c4]
0x14026e635 48898540020000 mov qword ptr [rbp + 0x240], rax
0x14026e63c 488d4578 lea rax, [rbp + 0x78]
0x14026e640 48898548020000 mov qword ptr [rbp + 0x248], rax
0x14026e647 488d85e3020000 lea rax, [rbp + 0x2e3]
0x14026e64e 48898550020000 mov qword ptr [rbp + 0x250], rax
0x14026e655 488d4554 lea rax, [rbp + 0x54]
0x14026e659 48898558020000 mov qword ptr [rbp + 0x258], rax
0x14026e660 488d85e8010000 lea rax, [rbp + 0x1e8]
0x14026e667 48898560020000 mov qword ptr [rbp + 0x260], rax
0x14026e66e 488d85ac020000 lea rax, [rbp + 0x2ac]
0x14026e675 48898568020000 mov qword ptr [rbp + 0x268], rax
0x14026e67c 488d85f8010000 lea rax, [rbp + 0x1f8]
0x14026e683 48898570020000 mov qword ptr [rbp + 0x270], rax
0x14026e68a 31d2 xor edx, edx
0x14026e68c 41b8404b4c00 mov r8d, 0x4c4b40
0x14026e692 4d89e1 mov r9, r12
0x14026e695 e866dd2c00 call 0x14053c400 ; winit::platform_impl::platform::event_loop::EventLoop::pump_events<tuple$<>,tiny_glade::event_loop::run_event_loop::closure_env$0>
0x14026e69a 85c0 test eax, eax
0x14026e69c 0f857e130000 jne 0x14026fa20
0x14026e6a2 41bf04000000 mov r15d, 4
0x14026e6a8 31db xor ebx, ebx
0x14026e6aa eb14 jmp 0x14026e6c0
0x14026e6ac 4889b5e8010000 mov qword ptr [rbp + 0x1e8], rsi
0x14026e6b3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14026e6c0 488b8588000000 mov rax, qword ptr [rbp + 0x88]
0x14026e6c7 488b9590000000 mov rdx, qword ptr [rbp + 0x90]
0x14026e6ce 4885c0 test rax, rax
0x14026e6d1 4d89fd mov r13, r15
0x14026e6d4 741a je 0x14026e6f0
0x14026e6d6 83f801 cmp eax, 1
0x14026e6d9 7525 jne 0x14026e700
0x14026e6db 488d4dc8 lea rcx, [rbp - 0x38]
0x14026e6df e85c86ffff call 0x140266d40 ; std::sync::mpmc::list::Channel::try_recv<enum2$<tiny_glade::event_loop::GameEvent> >
0x14026e6e4 eb23 jmp 0x14026e709
0x14026e6e6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14026e6f0 488d4dc8 lea rcx, [rbp - 0x38]
0x14026e6f4 e8173f2800 call 0x1404f2610 ; std::sync::mpmc::array::Channel::try_recv<enum2$<tiny_glade::event_loop::GameEvent> >
0x14026e6f9 eb0e jmp 0x14026e709
0x14026e6fb 0f1f440000 nop dword ptr [rax + rax]
0x14026e700 488d4dc8 lea rcx, [rbp - 0x38]
0x14026e704 e867182b00 call 0x14051ff70 ; std::sync::mpmc::zero::Channel::try_recv<enum2$<tiny_glade::event_loop::GameEvent> >
0x14026e709 488b4dc8 mov rcx, qword ptr [rbp - 0x38]
0x14026e70d 48b80700000000000080 movabs rax, 0x8000000000000007
0x14026e717 4839c1 cmp rcx, rax
0x14026e71a 0f84100d0000 je 0x14026f430
0x14026e720 488b75d0 mov rsi, qword ptr [rbp - 0x30]
0x14026e724 8b45d8 mov eax, dword ptr [rbp - 0x28]
0x14026e727 f2440f104ddc movsd xmm9, qword ptr [rbp - 0x24]
0x14026e72d 4889ca mov rdx, rcx
0x14026e730 49b80000000000000080 movabs r8, 0x8000000000000000
0x14026e73a 4c31c2 xor rdx, r8
0x14026e73d 4883fa07 cmp rdx, 7
0x14026e741 41b803000000 mov r8d, 3
0x14026e747 490f43d0 cmovae rdx, r8
0x14026e74b 49631496 movsxd rdx, dword ptr [r14 + rdx*4]
0x14026e74f 4c01f2 add rdx, r14
0x14026e752 4d89ef mov r15, r13
0x14026e755 ffe2 jmp rdx
0x14026e757 4889f0 mov rax, rsi
0x14026e75a 48c1e820 shr rax, 0x20
0x14026e75e 488985f0010000 mov qword ptr [rbp + 0x1f0], rax
0x14026e765 4189f7 mov r15d, esi
0x14026e768 e953ffffff jmp 0x14026e6c0
0x14026e76d 48898d10010000 mov qword ptr [rbp + 0x110], rcx
0x14026e774 4889b518010000 mov qword ptr [rbp + 0x118], rsi
0x14026e77b 898520010000 mov dword ptr [rbp + 0x120], eax
0x14026e781 440f138d24010000 movlps qword ptr [rbp + 0x124], xmm9
0x14026e789 488d4de4 lea rcx, [rbp - 0x1c]
0x14026e78d 0f10414c movups xmm0, xmmword ptr [rcx + 0x4c]
0x14026e791 488d852c010000 lea rax, [rbp + 0x12c]
0x14026e798 0f11404c movups xmmword ptr [rax + 0x4c], xmm0
0x14026e79c 0f104140 movups xmm0, xmmword ptr [rcx + 0x40]
0x14026e7a0 0f114040 movups xmmword ptr [rax + 0x40], xmm0
0x14026e7a4 660f1001 movupd xmm0, xmmword ptr [rcx]
0x14026e7a8 0f104910 movups xmm1, xmmword ptr [rcx + 0x10]
0x14026e7ac 0f105120 movups xmm2, xmmword ptr [rcx + 0x20]
0x14026e7b0 0f105930 movups xmm3, xmmword ptr [rcx + 0x30]
0x14026e7b4 0f115830 movups xmmword ptr [rax + 0x30], xmm3
0x14026e7b8 0f115020 movups xmmword ptr [rax + 0x20], xmm2
0x14026e7bc 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x14026e7c0 660f1100 movupd xmmword ptr [rax], xmm0
0x14026e7c4 488b8500010000 mov rax, qword ptr [rbp + 0x100]
0x14026e7cb 488b30 mov rsi, qword ptr [rax]
0x14026e7ce 488d7e10 lea rdi, [rsi + 0x10]
0x14026e7d2 31c0 xor eax, eax
0x14026e7d4 b101 mov cl, 1
0x14026e7d6 f00fb04e10 lock cmpxchg byte ptr [rsi + 0x10], cl
0x14026e7db 0f85f00b0000 jne 0x14026f3d1
0x14026e7e1 488b0538dc3b03 mov rax, qword ptr [rip + 0x33bdc38]
0x14026e7e8 488b00 mov rax, qword ptr [rax]
0x14026e7eb 48d1e0 shl rax, 1
0x14026e7ee 4885c0 test rax, rax
0x14026e7f1 0f85ee0b0000 jne 0x14026f3e5
0x14026e7f7 c785d002000000000000 mov dword ptr [rbp + 0x2d0], 0
0x14026e801 4889bdd8020000 mov qword ptr [rbp + 0x2d8], rdi
0x14026e808 0fb64611 movzx eax, byte ptr [rsi + 0x11]
0x14026e80c 84c0 test al, al
0x14026e80e 0f85c1130000 jne 0x14026fbd5
0x14026e814 488d4e18 lea rcx, [rsi + 0x18]
0x14026e818 488b95c0020000 mov rdx, qword ptr [rbp + 0x2c0]
0x14026e81f 4883c210 add rdx, 0x10
0x14026e823 488b8580010000 mov rax, qword ptr [rbp + 0x180]
0x14026e82a 48898590020000 mov qword ptr [rbp + 0x290], rax
0x14026e831 0f108570010000 movups xmm0, xmmword ptr [rbp + 0x170]
0x14026e838 0f298580020000 movaps xmmword ptr [rbp + 0x280], xmm0
0x14026e83f 0f108560010000 movups xmm0, xmmword ptr [rbp + 0x160]
0x14026e846 0f298570020000 movaps xmmword ptr [rbp + 0x270], xmm0
0x14026e84d 0f108550010000 movups xmm0, xmmword ptr [rbp + 0x150]
0x14026e854 0f298560020000 movaps xmmword ptr [rbp + 0x260], xmm0
0x14026e85b 660f108510010000 movupd xmm0, xmmword ptr [rbp + 0x110]
0x14026e863 0f108d20010000 movups xmm1, xmmword ptr [rbp + 0x120]
0x14026e86a 0f109530010000 movups xmm2, xmmword ptr [rbp + 0x130]
0x14026e871 0f109d40010000 movups xmm3, xmmword ptr [rbp + 0x140]
0x14026e878 0f299d50020000 movaps xmmword ptr [rbp + 0x250], xmm3
0x14026e87f 0f299540020000 movaps xmmword ptr [rbp + 0x240], xmm2
0x14026e886 0f298d30020000 movaps xmmword ptr [rbp + 0x230], xmm1
0x14026e88d 660f298520020000 movapd xmmword ptr [rbp + 0x220], xmm0
0x14026e895 4d89e0 mov r8, r12
0x14026e898 e8a3964000 call 0x140677f40 ; egui_winit::State::handle_platform_output
0x14026e89d 80bdd002000000 cmp byte ptr [rbp + 0x2d0], 0
0x14026e8a4 488bbdd8020000 mov rdi, qword ptr [rbp + 0x2d8]
0x14026e8ab 7516 jne 0x14026e8c3
0x14026e8ad 488b056cdb3b03 mov rax, qword ptr [rip + 0x33bdb6c]
0x14026e8b4 488b00 mov rax, qword ptr [rax]
0x14026e8b7 48d1e0 shl rax, 1
0x14026e8ba 4885c0 test rax, rax
0x14026e8bd 0f854e0b0000 jne 0x14026f411
0x14026e8c3 31c0 xor eax, eax
0x14026e8c5 8607 xchg byte ptr [rdi], al
0x14026e8c7 3c02 cmp al, 2
0x14026e8c9 0f85f1fdffff jne 0x14026e6c0
0x14026e8cf c685e502000000 mov byte ptr [rbp + 0x2e5], 0
0x14026e8d6 4889f9 mov rcx, rdi
0x14026e8d9 e8329f6602 call 0x1428d8810 ; std::sys::sync::mutex::futex::Mutex::wake
0x14026e8de e9ddfdffff jmp 0x14026e6c0
0x14026e8e3 85c0 test eax, eax
0x14026e8e5 0f84b2090000 je 0x14026f29d
0x14026e8eb 83f801 cmp eax, 1
0x14026e8ee 0f85660a0000 jne 0x14026f35a
0x14026e8f4 488b95c0020000 mov rdx, qword ptr [rbp + 0x2c0]
0x14026e8fb 4883c210 add rdx, 0x10
0x14026e8ff 4c89e1 mov rcx, r12
0x14026e902 4531c0 xor r8d, r8d
0x14026e905 e896efffff call 0x14026d8a0 ; winit::window::Window::set_cursor_grab
0x14026e90a 83bd2002000002 cmp dword ptr [rbp + 0x220], 2
0x14026e911 4d89ef mov r15, r13
0x14026e914 757a jne 0x14026e990
0x14026e916 488b8538020000 mov rax, qword ptr [rbp + 0x238]
0x14026e91d 89c1 mov ecx, eax
0x14026e91f 83e103 and ecx, 3
0x14026e922 83f901 cmp ecx, 1
0x14026e925 7569 jne 0x14026e990
0x14026e927 488d48ff lea rcx, [rax - 1]
0x14026e92b 48898db0020000 mov qword ptr [rbp + 0x2b0], rcx
0x14026e932 488b48ff mov rcx, qword ptr [rax - 1]
0x14026e936 48898dd8020000 mov qword ptr [rbp + 0x2d8], rcx
0x14026e93d 488b4007 mov rax, qword ptr [rax + 7]
0x14026e941 488985d0020000 mov qword ptr [rbp + 0x2d0], rax
0x14026e948 488b00 mov rax, qword ptr [rax]
0x14026e94b 4885c0 test rax, rax
0x14026e94e 7409 je 0x14026e959
0x14026e950 488b8dd8020000 mov rcx, qword ptr [rbp + 0x2d8]
0x14026e957 ffd0 call rax
0x14026e959 488b8dd8020000 mov rcx, qword ptr [rbp + 0x2d8]
0x14026e960 488b85d0020000 mov rax, qword ptr [rbp + 0x2d0]
0x14026e967 488b5008 mov rdx, qword ptr [rax + 8]
0x14026e96b 4885d2 test rdx, rdx
0x14026e96e 7409 je 0x14026e979
0x14026e970 4c8b4010 mov r8, qword ptr [rax + 0x10]
0x14026e974 e8a7523a00 call 0x140613c20 ; __rust_dealloc
0x14026e979 ba18000000 mov edx, 0x18
0x14026e97e 41b808000000 mov r8d, 8
0x14026e984 488b8db0020000 mov rcx, qword ptr [rbp + 0x2b0]
0x14026e98b e890523a00 call 0x140613c20 ; __rust_dealloc
0x14026e990 c7455402000000 mov dword ptr [rbp + 0x54], 2
0x14026e997 31db xor ebx, ebx
0x14026e999 488b8d00020000 mov rcx, qword ptr [rbp + 0x200]
0x14026e9a0 488b8518020000 mov rax, qword ptr [rbp + 0x218]
0x14026e9a7 4839c8 cmp rax, rcx
0x14026e9aa 0f84c8090000 je 0x14026f378
0x14026e9b0 e9e7090000 jmp 0x14026f39c
0x14026e9b5 488bb5c0020000 mov rsi, qword ptr [rbp + 0x2c0]
0x14026e9bc 488b7e18 mov rdi, qword ptr [rsi + 0x18]
0x14026e9c0 488b4610 mov rax, qword ptr [rsi + 0x10]
0x14026e9c4 f048ff00 lock inc qword ptr [rax]
0x14026e9c8 0f8e4c120000 jle 0x14026fc1a
0x14026e9ce 48898510010000 mov qword ptr [rbp + 0x110], rax
0x14026e9d5 c685ce02000001 mov byte ptr [rbp + 0x2ce], 1
0x14026e9dc 4889f9 mov rcx, rdi
0x14026e9df 488985d8020000 mov qword ptr [rbp + 0x2d8], rax
0x14026e9e6 e835734200 call 0x140695d20 ; winit::platform_impl::platform::util::is_minimized
0x14026e9eb 4883c620 add rsi, 0x20
0x14026e9ef 4889bd20020000 mov qword ptr [rbp + 0x220], rdi
0x14026e9f6 488b8dd8020000 mov rcx, qword ptr [rbp + 0x2d8]
0x14026e9fd 48898d28020000 mov qword ptr [rbp + 0x228], rcx
0x14026ea04 888530020000 mov byte ptr [rbp + 0x230], al
0x14026ea0a c6853102000001 mov byte ptr [rbp + 0x231], 1
0x14026ea11 c685ce02000000 mov byte ptr [rbp + 0x2ce], 0
0x14026ea18 4889f1 mov rcx, rsi
0x14026ea1b 4c89e2 mov rdx, r12
0x14026ea1e e89d082d00 call 0x14053f2c0 ; winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread<winit::platform_impl::platform::window::impl$0::set_minimized::closure_env$0>
0x14026ea23 e998fcffff jmp 0x14026e6c0
0x14026ea28 4889b598000000 mov qword ptr [rbp + 0x98], rsi
0x14026ea2f 8985a0000000 mov dword ptr [rbp + 0xa0], eax
0x14026ea35 440f138da4000000 movlps qword ptr [rbp + 0xa4], xmm9
0x14026ea3d 488d55e4 lea rdx, [rbp - 0x1c]
0x14026ea41 8b4230 mov eax, dword ptr [rdx + 0x30]
0x14026ea44 488d8dac000000 lea rcx, [rbp + 0xac]
0x14026ea4b 894130 mov dword ptr [rcx + 0x30], eax
0x14026ea4e 660f1002 movupd xmm0, xmmword ptr [rdx]
0x14026ea52 0f104a10 movups xmm1, xmmword ptr [rdx + 0x10]
0x14026ea56 0f105220 movups xmm2, xmmword ptr [rdx + 0x20]
0x14026ea5a 0f115120 movups xmmword ptr [rcx + 0x20], xmm2
0x14026ea5e 0f114910 movups xmmword ptr [rcx + 0x10], xmm1
0x14026ea62 660f1101 movupd xmmword ptr [rcx], xmm0
0x14026ea66 488b95c0020000 mov rdx, qword ptr [rbp + 0x2c0]
0x14026ea6d 4883c210 add rdx, 0x10
0x14026ea71 c685e702000001 mov byte ptr [rbp + 0x2e7], 1
0x14026ea78 4c89e1 mov rcx, r12
0x14026ea7b e810904200 call 0x140697a90 ; winit::platform_impl::platform::window::Window::available_monitors
0x14026ea80 4c89e1 mov rcx, r12
0x14026ea83 488d9598000000 lea rdx, [rbp + 0x98]
0x14026ea8a 4c8d8540020000 lea r8, [rbp + 0x240]
0x14026ea91 e8da7f2b00 call 0x140526a70 ; alloc::collections::vec_deque::into_iter::impl$2::try_fold<winit::platform_impl::platform::monitor::MonitorHandle,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<winit::platform_impl::platform::monitor::MonitorHandle,winit::monitor::MonitorHandle,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > >,winit::window::impl$14::available_monitors::closure$0::closure_env$0,core::iter::traits::iterator::Iterator::find::check::closure_env$0<winit::monitor::MonitorHandle,tiny_glade::event_loop::run_event_loop::closure_env$2> >,enum2$<core::ops::control_flow::ControlFlow<winit::monitor::MonitorHandle,tuple$<> > > >
0x14026ea96 a801 test al, 1
0x14026ea98 0f8498050000 je 0x14026f036
0x14026ea9e 4889d7 mov rdi, rdx
0x14026eaa1 488b9520020000 mov rdx, qword ptr [rbp + 0x220]
0x14026eaa8 4885d2 test rdx, rdx
0x14026eaab 7416 je 0x14026eac3
0x14026eaad 488b8d28020000 mov rcx, qword ptr [rbp + 0x228]
0x14026eab4 48c1e203 shl rdx, 3
0x14026eab8 41b808000000 mov r8d, 8
0x14026eabe e85d513a00 call 0x140613c20 ; __rust_dealloc
0x14026eac3 80bddc00000000 cmp byte ptr [rbp + 0xdc], 0
0x14026eaca 0f848e050000 je 0x14026f05e
0x14026ead0 f685dd00000001 test byte ptr [rbp + 0xdd], 1
0x14026ead7 0f849b030000 je 0x14026ee78
0x14026eadd c685e702000001 mov byte ptr [rbp + 0x2e7], 1
0x14026eae4 488d4da8 lea rcx, [rbp - 0x58]
0x14026eae8 4889fa mov rdx, rdi
0x14026eaeb 4c8d8598000000 lea r8, [rbp + 0x98]
0x14026eaf2 e8c968ddff call 0x1400453c0 ; tiny_glade::setup::find_best_fullscreen_video_mode
0x14026eaf7 48837da800 cmp qword ptr [rbp - 0x58], 0
0x14026eafc 0f8476030000 je 0x14026ee78
0x14026eb02 4189dd mov r13d, ebx
0x14026eb05 660f1045a8 movupd xmm0, xmmword ptr [rbp - 0x58]
0x14026eb0a 0f104db8 movups xmm1, xmmword ptr [rbp - 0x48]
0x14026eb0e 0f298db0010000 movaps xmmword ptr [rbp + 0x1b0], xmm1
0x14026eb15 660f2985a0010000 movapd xmmword ptr [rbp + 0x1a0], xmm0
0x14026eb1d c685e602000001 mov byte ptr [rbp + 0x2e6], 1
0x14026eb24 488d8da0010000 lea rcx, [rbp + 0x1a0]
0x14026eb2b e8808e4200 call 0x1406979b0 ; winit::platform_impl::platform::monitor::VideoMode::size
0x14026eb30 89d3 mov ebx, edx
0x14026eb32 89c0 mov eax, eax
0x14026eb34 0f57c0 xorps xmm0, xmm0
0x14026eb37 f2480f2ac0 cvtsi2sd xmm0, rax
0x14026eb3c c685e602000001 mov byte ptr [rbp + 0x2e6], 1
0x14026eb43 e8a88c4200 call 0x1406977f0 ; winit::dpi::impl$2::from_f64
0x14026eb48 89c6 mov esi, eax
0x14026eb4a 89d8 mov eax, ebx
0x14026eb4c 0f57c0 xorps xmm0, xmm0
0x14026eb4f f2480f2ac0 cvtsi2sd xmm0, rax
0x14026eb54 c685e602000001 mov byte ptr [rbp + 0x2e6], 1
0x14026eb5b e8908c4200 call 0x1406977f0 ; winit::dpi::impl$2::from_f64
0x14026eb60 48c1e020 shl rax, 0x20
0x14026eb64 89f1 mov ecx, esi
0x14026eb66 4809c1 or rcx, rax
0x14026eb69 0fb785bc010000 movzx eax, word ptr [rbp + 0x1bc]
0x14026eb70 8b95b8010000 mov edx, dword ptr [rbp + 0x1b8]
0x14026eb76 48898dcc000000 mov qword ptr [rbp + 0xcc], rcx
0x14026eb7d 8995d4000000 mov dword ptr [rbp + 0xd4], edx
0x14026eb83 668985d8000000 mov word ptr [rbp + 0xd8], ax
0x14026eb8a c785c800000001000000 mov dword ptr [rbp + 0xc8], 1
0x14026eb94 488b050dd83b03 mov rax, qword ptr [rip + 0x33bd80d]
0x14026eb9b 488b00 mov rax, qword ptr [rax]
0x14026eb9e 4883f803 cmp rax, 3
0x14026eba2 0f82ed000000 jb 0x14026ec95
0x14026eba8 488d85a0010000 lea rax, [rbp + 0x1a0]
0x14026ebaf 48898510010000 mov qword ptr [rbp + 0x110], rax
0x14026ebb6 488d05837c4300 lea rax, [rip + 0x437c83]
0x14026ebbd 48898518010000 mov qword ptr [rbp + 0x118], rax
0x14026ebc4 c685e602000001 mov byte ptr [rbp + 0x2e6], 1
0x14026ebcb 488d0dfe8f7302 lea rcx, [rip + 0x2738ffe]
0x14026ebd2 e8a9f14000 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x14026ebd7 660f1000 movupd xmm0, xmmword ptr [rax]
0x14026ebdb 8b4010 mov eax, dword ptr [rax + 0x10]
0x14026ebde 48c7855002000003000000 mov qword ptr [rbp + 0x250], 3
0x14026ebe9 488d0d908f7302 lea rcx, [rip + 0x2738f90]
0x14026ebf0 48898d58020000 mov qword ptr [rbp + 0x258], rcx
0x14026ebf7 48c7856002000016000000 mov qword ptr [rbp + 0x260], 0x16
0x14026ec02 488d15b78f7302 lea rdx, [rip + 0x2738fb7]
0x14026ec09 48899570020000 mov qword ptr [rbp + 0x270], rdx
0x14026ec10 48c7857802000001000000 mov qword ptr [rbp + 0x278], 1
0x14026ec1b 488d9510010000 lea rdx, [rbp + 0x110]
0x14026ec22 48899580020000 mov qword ptr [rbp + 0x280], rdx
0x14026ec29 48c7858802000001000000 mov qword ptr [rbp + 0x288], 1
0x14026ec34 48c7859002000000000000 mov qword ptr [rbp + 0x290], 0
0x14026ec3f 48c7852002000000000000 mov qword ptr [rbp + 0x220], 0
0x14026ec4a 48898d28020000 mov qword ptr [rbp + 0x228], rcx
0x14026ec51 48c7853002000016000000 mov qword ptr [rbp + 0x230], 0x16
0x14026ec5c 48c7853802000000000000 mov qword ptr [rbp + 0x238], 0
0x14026ec67 660f118540020000 movupd xmmword ptr [rbp + 0x240], xmm0
0x14026ec6f c7856802000001000000 mov dword ptr [rbp + 0x268], 1
0x14026ec79 89856c020000 mov dword ptr [rbp + 0x26c], eax
0x14026ec7f c685e602000001 mov byte ptr [rbp + 0x2e6], 1
0x14026ec86 488d8de4020000 lea rcx, [rbp + 0x2e4]
0x14026ec8d 4c89e2 mov rdx, r12
0x14026ec90 e89bb6ad00 call 0x140d4a330 ; log::__private_api::impl$2::log
0x14026ec95 488b95c0020000 mov rdx, qword ptr [rbp + 0x2c0]
0x14026ec9c 4883c210 add rdx, 0x10
0x14026eca0 660f2885a0010000 movapd xmm0, xmmword ptr [rbp + 0x1a0]
0x14026eca8 0f288db0010000 movaps xmm1, xmmword ptr [rbp + 0x1b0]
0x14026ecaf 488d8528020000 lea rax, [rbp + 0x228]
0x14026ecb6 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x14026ecba 660f1100 movupd xmmword ptr [rax], xmm0
0x14026ecbe 48c7852002000001000000 mov qword ptr [rbp + 0x220], 1
0x14026ecc9 c685e602000000 mov byte ptr [rbp + 0x2e6], 0
0x14026ecd0 488d8de0000000 lea rcx, [rbp + 0xe0]
0x14026ecd7 4d89e0 mov r8, r12
0x14026ecda e8a1954200 call 0x140698280 ; winit::window::Window::try_set_fullscreen
0x14026ecdf 4883bde000000000 cmp qword ptr [rbp + 0xe0], 0
0x14026ece7 0f84dc060000 je 0x14026f3c9
0x14026eced 660f1085e0000000 movupd xmm0, xmmword ptr [rbp + 0xe0]
0x14026ecf5 0f108df0000000 movups xmm1, xmmword ptr [rbp + 0xf0]
0x14026ecfc 0f298d20010000 movaps xmmword ptr [rbp + 0x120], xmm1
0x14026ed03 660f298510010000 movapd xmmword ptr [rbp + 0x110], xmm0
0x14026ed0b c685dd00000000 mov byte ptr [rbp + 0xdd], 0
0x14026ed12 488b058fd63b03 mov rax, qword ptr [rip + 0x33bd68f]
0x14026ed19 488b00 mov rax, qword ptr [rax]
0x14026ed1c 4885c0 test rax, rax
0x14026ed1f 4489eb mov ebx, r13d
0x14026ed22 0f84d6000000 je 0x14026edfe
0x14026ed28 488d8510010000 lea rax, [rbp + 0x110]
0x14026ed2f 48894540 mov qword ptr [rbp + 0x40], rax
0x14026ed33 488d05b6eaffff lea rax, [rip - 0x154a]
0x14026ed3a 48894548 mov qword ptr [rbp + 0x48], rax
0x14026ed3e 488d0ddb8e7302 lea rcx, [rip + 0x2738edb]
0x14026ed45 e836f04000 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x14026ed4a 660f1000 movupd xmm0, xmmword ptr [rax]
0x14026ed4e 8b4010 mov eax, dword ptr [rax + 0x10]
0x14026ed51 48c7855002000001000000 mov qword ptr [rbp + 0x250], 1
0x14026ed5c 488d0d1d8e7302 lea rcx, [rip + 0x2738e1d]
0x14026ed63 48898d58020000 mov qword ptr [rbp + 0x258], rcx
0x14026ed6a 48c7856002000016000000 mov qword ptr [rbp + 0x260], 0x16
0x14026ed75 488d15948e7302 lea rdx, [rip + 0x2738e94]
0x14026ed7c 48899570020000 mov qword ptr [rbp + 0x270], rdx
0x14026ed83 48c7857802000001000000 mov qword ptr [rbp + 0x278], 1
0x14026ed8e 488d5540 lea rdx, [rbp + 0x40]
0x14026ed92 48899580020000 mov qword ptr [rbp + 0x280], rdx
0x14026ed99 48c7858802000001000000 mov qword ptr [rbp + 0x288], 1
0x14026eda4 48c7859002000000000000 mov qword ptr [rbp + 0x290], 0
0x14026edaf 48c7852002000000000000 mov qword ptr [rbp + 0x220], 0
0x14026edba 48898d28020000 mov qword ptr [rbp + 0x228], rcx
0x14026edc1 48c7853002000016000000 mov qword ptr [rbp + 0x230], 0x16
0x14026edcc 48c7853802000000000000 mov qword ptr [rbp + 0x238], 0
0x14026edd7 660f118540020000 movupd xmmword ptr [rbp + 0x240], xmm0
0x14026eddf c7856802000001000000 mov dword ptr [rbp + 0x268], 1
0x14026ede9 89856c020000 mov dword ptr [rbp + 0x26c], eax
0x14026edef 488d8de4020000 lea rcx, [rbp + 0x2e4]
0x14026edf6 4c89e2 mov rdx, r12
0x14026edf9 e832b5ad00 call 0x140d4a330 ; log::__private_api::impl$2::log
0x14026edfe 488b8520010000 mov rax, qword ptr [rbp + 0x120]
0x14026ee05 89c1 mov ecx, eax
0x14026ee07 83e103 and ecx, 3
0x14026ee0a 83f901 cmp ecx, 1
0x14026ee0d 7569 jne 0x14026ee78
0x14026ee0f 488d48ff lea rcx, [rax - 1]
0x14026ee13 48898db0020000 mov qword ptr [rbp + 0x2b0], rcx
0x14026ee1a 488b48ff mov rcx, qword ptr [rax - 1]
0x14026ee1e 48898dd8020000 mov qword ptr [rbp + 0x2d8], rcx
0x14026ee25 488b4007 mov rax, qword ptr [rax + 7]
0x14026ee29 488985d0020000 mov qword ptr [rbp + 0x2d0], rax
0x14026ee30 488b00 mov rax, qword ptr [rax]
0x14026ee33 4885c0 test rax, rax
0x14026ee36 7409 je 0x14026ee41
0x14026ee38 488b8dd8020000 mov rcx, qword ptr [rbp + 0x2d8]
0x14026ee3f ffd0 call rax
0x14026ee41 488b8dd8020000 mov rcx, qword ptr [rbp + 0x2d8]
0x14026ee48 488b85d0020000 mov rax, qword ptr [rbp + 0x2d0]
0x14026ee4f 488b5008 mov rdx, qword ptr [rax + 8]
0x14026ee53 4885d2 test rdx, rdx
0x14026ee56 7409 je 0x14026ee61
0x14026ee58 4c8b4010 mov r8, qword ptr [rax + 0x10]
0x14026ee5c e8bf4d3a00 call 0x140613c20 ; __rust_dealloc
0x14026ee61 ba18000000 mov edx, 0x18
0x14026ee66 41b808000000 mov r8d, 8
0x14026ee6c 488b8db0020000 mov rcx, qword ptr [rbp + 0x2b0]
0x14026ee73 e8a84d3a00 call 0x140613c20 ; __rust_dealloc
0x14026ee78 488b95c0020000 mov rdx, qword ptr [rbp + 0x2c0]
0x14026ee7f 4883c210 add rdx, 0x10
0x14026ee83 48c7852802000000000000 mov qword ptr [rbp + 0x228], 0
0x14026ee8e 48c7853002000001000000 mov qword ptr [rbp + 0x230], 1
0x14026ee99 4889bd38020000 mov qword ptr [rbp + 0x238], rdi
0x14026eea0 48c7852002000001000000 mov qword ptr [rbp + 0x220], 1
0x14026eeab c685e702000001 mov byte ptr [rbp + 0x2e7], 1
0x14026eeb2 488d8de0000000 lea rcx, [rbp + 0xe0]
0x14026eeb9 4d89e0 mov r8, r12
0x14026eebc e8bf934200 call 0x140698280 ; winit::window::Window::try_set_fullscreen
0x14026eec1 4883bde000000000 cmp qword ptr [rbp + 0xe0], 0
0x14026eec9 0f8453030000 je 0x14026f222
0x14026eecf 660f1085e0000000 movupd xmm0, xmmword ptr [rbp + 0xe0]
0x14026eed7 0f108df0000000 movups xmm1, xmmword ptr [rbp + 0xf0]
0x14026eede 0f298d20010000 movaps xmmword ptr [rbp + 0x120], xmm1
0x14026eee5 660f298510010000 movapd xmmword ptr [rbp + 0x110], xmm0
0x14026eeed c685dc00000000 mov byte ptr [rbp + 0xdc], 0
0x14026eef4 488b05add43b03 mov rax, qword ptr [rip + 0x33bd4ad]
0x14026eefb 488b00 mov rax, qword ptr [rax]
0x14026eefe 4885c0 test rax, rax
0x14026ef01 0f84df000000 je 0x14026efe6
0x14026ef07 488d8510010000 lea rax, [rbp + 0x110]
0x14026ef0e 488985a0010000 mov qword ptr [rbp + 0x1a0], rax
0x14026ef15 488d05d4e8ffff lea rax, [rip - 0x172c]
0x14026ef1c 488985a8010000 mov qword ptr [rbp + 0x1a8], rax
0x14026ef23 488d0d468d7302 lea rcx, [rip + 0x2738d46]
0x14026ef2a e851ee4000 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x14026ef2f 660f1000 movupd xmm0, xmmword ptr [rax]
0x14026ef33 8b4010 mov eax, dword ptr [rax + 0x10]
0x14026ef36 48c7855002000001000000 mov qword ptr [rbp + 0x250], 1
0x14026ef41 488d0d388c7302 lea rcx, [rip + 0x2738c38]
0x14026ef48 48898d58020000 mov qword ptr [rbp + 0x258], rcx
0x14026ef4f 48c7856002000016000000 mov qword ptr [rbp + 0x260], 0x16
0x14026ef5a 488d15ff8c7302 lea rdx, [rip + 0x2738cff]
0x14026ef61 48899570020000 mov qword ptr [rbp + 0x270], rdx
0x14026ef68 48c7857802000001000000 mov qword ptr [rbp + 0x278], 1
0x14026ef73 488d95a0010000 lea rdx, [rbp + 0x1a0]
0x14026ef7a 48899580020000 mov qword ptr [rbp + 0x280], rdx
0x14026ef81 48c7858802000001000000 mov qword ptr [rbp + 0x288], 1
0x14026ef8c 48c7859002000000000000 mov qword ptr [rbp + 0x290], 0
0x14026ef97 48c7852002000000000000 mov qword ptr [rbp + 0x220], 0
0x14026efa2 48898d28020000 mov qword ptr [rbp + 0x228], rcx
0x14026efa9 48c7853002000016000000 mov qword ptr [rbp + 0x230], 0x16
0x14026efb4 48c7853802000000000000 mov qword ptr [rbp + 0x238], 0
0x14026efbf 660f118540020000 movupd xmmword ptr [rbp + 0x240], xmm0
0x14026efc7 c7856802000001000000 mov dword ptr [rbp + 0x268], 1
0x14026efd1 89856c020000 mov dword ptr [rbp + 0x26c], eax
0x14026efd7 488d8de4020000 lea rcx, [rbp + 0x2e4]
0x14026efde 4c89e2 mov rdx, r12
0x14026efe1 e84ab3ad00 call 0x140d4a330 ; log::__private_api::impl$2::log
0x14026efe6 488b8520010000 mov rax, qword ptr [rbp + 0x120]
0x14026efed 89c1 mov ecx, eax
0x14026efef 83e103 and ecx, 3
0x14026eff2 83f901 cmp ecx, 1
0x14026eff5 0f8527020000 jne 0x14026f222
0x14026effb 488d48ff lea rcx, [rax - 1]
0x14026efff 48898db0020000 mov qword ptr [rbp + 0x2b0], rcx
0x14026f006 488b48ff mov rcx, qword ptr [rax - 1]
0x14026f00a 48898dd8020000 mov qword ptr [rbp + 0x2d8], rcx
0x14026f011 488b4007 mov rax, qword ptr [rax + 7]
0x14026f015 488985d0020000 mov qword ptr [rbp + 0x2d0], rax
0x14026f01c 488b00 mov rax, qword ptr [rax]
0x14026f01f 4885c0 test rax, rax
0x14026f022 0f84c3010000 je 0x14026f1eb
0x14026f028 488b8dd8020000 mov rcx, qword ptr [rbp + 0x2d8]
0x14026f02f ffd0 call rax
0x14026f031 e9b5010000 jmp 0x14026f1eb
0x14026f036 488b85c0020000 mov rax, qword ptr [rbp + 0x2c0]
0x14026f03d 488b4818 mov rcx, qword ptr [rax + 0x18]
0x14026f041 e8ba8a4200 call 0x140697b00 ; winit::platform_impl::platform::monitor::current_monitor
0x14026f046 4889c7 mov rdi, rax
0x14026f049 488b9520020000 mov rdx, qword ptr [rbp + 0x220]
0x14026f050 4885d2 test rdx, rdx
0x14026f053 0f8554faffff jne 0x14026eaad
0x14026f059 e965faffff jmp 0x14026eac3
0x14026f05e 488b95c0020000 mov rdx, qword ptr [rbp + 0x2c0]
0x14026f065 4883c210 add rdx, 0x10
0x14026f069 48c7852002000000000000 mov qword ptr [rbp + 0x220], 0
0x14026f074 c685e702000001 mov byte ptr [rbp + 0x2e7], 1
0x14026f07b 488d8de0000000 lea rcx, [rbp + 0xe0]
0x14026f082 4d89e0 mov r8, r12
0x14026f085 e8f6914200 call 0x140698280 ; winit::window::Window::try_set_fullscreen
0x14026f08a 4883bde000000000 cmp qword ptr [rbp + 0xe0], 0
0x14026f092 0f848a010000 je 0x14026f222
0x14026f098 660f1085e0000000 movupd xmm0, xmmword ptr [rbp + 0xe0]
0x14026f0a0 0f108df0000000 movups xmm1, xmmword ptr [rbp + 0xf0]
0x14026f0a7 0f298d20010000 movaps xmmword ptr [rbp + 0x120], xmm1
0x14026f0ae 660f298510010000 movapd xmmword ptr [rbp + 0x110], xmm0
0x14026f0b6 488b05ebd23b03 mov rax, qword ptr [rip + 0x33bd2eb]
0x14026f0bd 488b00 mov rax, qword ptr [rax]
0x14026f0c0 4885c0 test rax, rax
0x14026f0c3 0f84df000000 je 0x14026f1a8
0x14026f0c9 488d8510010000 lea rax, [rbp + 0x110]
0x14026f0d0 488985a0010000 mov qword ptr [rbp + 0x1a0], rax
0x14026f0d7 488d0512e7ffff lea rax, [rip - 0x18ee]
0x14026f0de 488985a8010000 mov qword ptr [rbp + 0x1a8], rax
0x14026f0e5 488d0d7c8a7302 lea rcx, [rip + 0x2738a7c]
0x14026f0ec e88fec4000 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x14026f0f1 660f1000 movupd xmm0, xmmword ptr [rax]
0x14026f0f5 8b4010 mov eax, dword ptr [rax + 0x10]
0x14026f0f8 48c7855002000001000000 mov qword ptr [rbp + 0x250], 1
0x14026f103 488d0d768a7302 lea rcx, [rip + 0x2738a76]
0x14026f10a 48898d58020000 mov qword ptr [rbp + 0x258], rcx
0x14026f111 48c7856002000016000000 mov qword ptr [rbp + 0x260], 0x16
0x14026f11c 488d15358a7302 lea rdx, [rip + 0x2738a35]
0x14026f123 48899570020000 mov qword ptr [rbp + 0x270], rdx
0x14026f12a 48c7857802000001000000 mov qword ptr [rbp + 0x278], 1
0x14026f135 488d95a0010000 lea rdx, [rbp + 0x1a0]
0x14026f13c 48899580020000 mov qword ptr [rbp + 0x280], rdx
0x14026f143 48c7858802000001000000 mov qword ptr [rbp + 0x288], 1
0x14026f14e 48c7859002000000000000 mov qword ptr [rbp + 0x290], 0
0x14026f159 48c7852002000000000000 mov qword ptr [rbp + 0x220], 0
0x14026f164 48898d28020000 mov qword ptr [rbp + 0x228], rcx
0x14026f16b 48c7853002000016000000 mov qword ptr [rbp + 0x230], 0x16
0x14026f176 48c7853802000000000000 mov qword ptr [rbp + 0x238], 0
0x14026f181 660f118540020000 movupd xmmword ptr [rbp + 0x240], xmm0
0x14026f189 c7856802000001000000 mov dword ptr [rbp + 0x268], 1
0x14026f193 89856c020000 mov dword ptr [rbp + 0x26c], eax
0x14026f199 488d8de4020000 lea rcx, [rbp + 0x2e4]
0x14026f1a0 4c89e2 mov rdx, r12
0x14026f1a3 e888b1ad00 call 0x140d4a330 ; log::__private_api::impl$2::log
0x14026f1a8 488b8520010000 mov rax, qword ptr [rbp + 0x120]
0x14026f1af 89c1 mov ecx, eax
0x14026f1b1 83e103 and ecx, 3
0x14026f1b4 83f901 cmp ecx, 1
0x14026f1b7 7569 jne 0x14026f222
0x14026f1b9 488d48ff lea rcx, [rax - 1]
0x14026f1bd 48898db0020000 mov qword ptr [rbp + 0x2b0], rcx
0x14026f1c4 488b48ff mov rcx, qword ptr [rax - 1]
0x14026f1c8 48898dd8020000 mov qword ptr [rbp + 0x2d8], rcx
0x14026f1cf 488b4007 mov rax, qword ptr [rax + 7]
0x14026f1d3 488985d0020000 mov qword ptr [rbp + 0x2d0], rax
0x14026f1da 488b00 mov rax, qword ptr [rax]
0x14026f1dd 4885c0 test rax, rax
0x14026f1e0 7409 je 0x14026f1eb
0x14026f1e2 488b8dd8020000 mov rcx, qword ptr [rbp + 0x2d8]
0x14026f1e9 ffd0 call rax
0x14026f1eb 488b8dd8020000 mov rcx, qword ptr [rbp + 0x2d8]
0x14026f1f2 488b85d0020000 mov rax, qword ptr [rbp + 0x2d0]
0x14026f1f9 488b5008 mov rdx, qword ptr [rax + 8]
0x14026f1fd 4885d2 test rdx, rdx
0x14026f200 7409 je 0x14026f20b
0x14026f202 4c8b4010 mov r8, qword ptr [rax + 0x10]
0x14026f206 e8154a3a00 call 0x140613c20 ; __rust_dealloc
0x14026f20b ba18000000 mov edx, 0x18
0x14026f210 41b808000000 mov r8d, 8
0x14026f216 488b8db0020000 mov rcx, qword ptr [rbp + 0x2b0]
0x14026f21d e8fe493a00 call 0x140613c20 ; __rust_dealloc
0x14026f222 488b85d8000000 mov rax, qword ptr [rbp + 0xd8]
0x14026f229 488d8d28020000 lea rcx, [rbp + 0x228]
0x14026f230 48894140 mov qword ptr [rcx + 0x40], rax
0x14026f234 660f108598000000 movupd xmm0, xmmword ptr [rbp + 0x98]
0x14026f23c 0f108da8000000 movups xmm1, xmmword ptr [rbp + 0xa8]
0x14026f243 0f1095b8000000 movups xmm2, xmmword ptr [rbp + 0xb8]
0x14026f24a 0f109dc8000000 movups xmm3, xmmword ptr [rbp + 0xc8]
0x14026f251 0f115930 movups xmmword ptr [rcx + 0x30], xmm3
0x14026f255 0f115120 movups xmmword ptr [rcx + 0x20], xmm2
0x14026f259 0f114910 movups xmmword ptr [rcx + 0x10], xmm1
0x14026f25d 660f1101 movupd xmmword ptr [rcx], xmm0
0x14026f261 66c785200200000900 mov word ptr [rbp + 0x220], 9
0x14026f26a c685e702000000 mov byte ptr [rbp + 0x2e7], 0
0x14026f271 488d8d10010000 lea rcx, [rbp + 0x110]
0x14026f278 488d95c8010000 lea rdx, [rbp + 0x1c8]
0x14026f27f 4d89e0 mov r8, r12
0x14026f282 e829661500 call 0x1403c58b0 ; std::sync::mpmc::Sender::send<enum2$<tiny_glade::event_loop::OsWindowEvent> >
0x14026f287 0fb78510010000 movzx eax, word ptr [rbp + 0x110]
0x14026f28e 6683f812 cmp ax, 0x12
0x14026f292 0f8428f4ffff je 0x14026e6c0
0x14026f298 e956070000 jmp 0x14026f9f3
0x14026f29d 488b95c0020000 mov rdx, qword ptr [rbp + 0x2c0]
0x14026f2a4 4883c210 add rdx, 0x10
0x14026f2a8 4c89e1 mov rcx, r12
0x14026f2ab 41b001 mov r8b, 1
0x14026f2ae e8ede5ffff call 0x14026d8a0 ; winit::window::Window::set_cursor_grab
0x14026f2b3 83bd2002000002 cmp dword ptr [rbp + 0x220], 2
0x14026f2ba 4d89ef mov r15, r13
0x14026f2bd 757a jne 0x14026f339
0x14026f2bf 488b8538020000 mov rax, qword ptr [rbp + 0x238]
0x14026f2c6 89c1 mov ecx, eax
0x14026f2c8 83e103 and ecx, 3
0x14026f2cb 83f901 cmp ecx, 1
0x14026f2ce 7569 jne 0x14026f339
0x14026f2d0 488d48ff lea rcx, [rax - 1]
0x14026f2d4 48898db0020000 mov qword ptr [rbp + 0x2b0], rcx
0x14026f2db 488b48ff mov rcx, qword ptr [rax - 1]
0x14026f2df 48898dd8020000 mov qword ptr [rbp + 0x2d8], rcx
0x14026f2e6 488b4007 mov rax, qword ptr [rax + 7]
0x14026f2ea 488985d0020000 mov qword ptr [rbp + 0x2d0], rax
0x14026f2f1 488b00 mov rax, qword ptr [rax]
0x14026f2f4 4885c0 test rax, rax
0x14026f2f7 7409 je 0x14026f302
0x14026f2f9 488b8dd8020000 mov rcx, qword ptr [rbp + 0x2d8]
0x14026f300 ffd0 call rax
0x14026f302 488b8dd8020000 mov rcx, qword ptr [rbp + 0x2d8]
0x14026f309 488b85d0020000 mov rax, qword ptr [rbp + 0x2d0]
0x14026f310 488b5008 mov rdx, qword ptr [rax + 8]
0x14026f314 4885d2 test rdx, rdx
0x14026f317 7409 je 0x14026f322
0x14026f319 4c8b4010 mov r8, qword ptr [rax + 0x10]
0x14026f31d e8fe483a00 call 0x140613c20 ; __rust_dealloc
0x14026f322 ba18000000 mov edx, 0x18
0x14026f327 41b808000000 mov r8d, 8
0x14026f32d 488b8db0020000 mov rcx, qword ptr [rbp + 0x2b0]
0x14026f334 e8e7483a00 call 0x140613c20 ; __rust_dealloc
0x14026f339 c7455400000000 mov dword ptr [rbp + 0x54], 0
0x14026f340 440f134d60 movlps qword ptr [rbp + 0x60], xmm9
0x14026f345 488b8d00020000 mov rcx, qword ptr [rbp + 0x200]
0x14026f34c 488b8518020000 mov rax, qword ptr [rbp + 0x218]
0x14026f353 4839c8 cmp rax, rcx
0x14026f356 7420 je 0x14026f378
0x14026f358 eb42 jmp 0x14026f39c
0x14026f35a f3410f16f9 movshdup xmm7, xmm9
0x14026f35f b301 mov bl, 1
0x14026f361 450f28c1 movaps xmm8, xmm9
0x14026f365 488b8d00020000 mov rcx, qword ptr [rbp + 0x200]
0x14026f36c 488b8518020000 mov rax, qword ptr [rbp + 0x218]
0x14026f373 4839c8 cmp rax, rcx
0x14026f376 7524 jne 0x14026f39c
0x14026f378 488d8d00020000 lea rcx, [rbp + 0x200]
0x14026f37f 488d1582877302 lea rdx, [rip + 0x2738782]
0x14026f386 e8f51f2a00 call 0x140511380 ; alloc::collections::vec_deque::VecDeque::grow<rhapsody::raw::disposal::SequentialDisposalItem,alloc::alloc::Global>
0x14026f38b 488b8d00020000 mov rcx, qword ptr [rbp + 0x200]
0x14026f392 488b8518020000 mov rax, qword ptr [rbp + 0x218]
0x14026f399 4d89ef mov r15, r13
0x14026f39c 48038510020000 add rax, qword ptr [rbp + 0x210]
0x14026f3a3 4839c8 cmp rax, rcx
0x14026f3a6 ba00000000 mov edx, 0
0x14026f3ab 480f43d1 cmovae rdx, rcx
0x14026f3af 4829d0 sub rax, rdx
0x14026f3b2 488b8d08020000 mov rcx, qword ptr [rbp + 0x208]
0x14026f3b9 488934c1 mov qword ptr [rcx + rax*8], rsi
0x14026f3bd 48ff8518020000 inc qword ptr [rbp + 0x218]
0x14026f3c4 e9f7f2ffff jmp 0x14026e6c0
0x14026f3c9 4489eb mov ebx, r13d
0x14026f3cc e951feffff jmp 0x14026f222
0x14026f3d1 c685e502000001 mov byte ptr [rbp + 0x2e5], 1
0x14026f3d8 4889f9 mov rcx, rdi
0x14026f3db e870936602 call 0x1428d8750 ; std::sys::sync::mutex::futex::Mutex::lock_contended
0x14026f3e0 e9fcf3ffff jmp 0x14026e7e1
0x14026f3e5 c685e502000001 mov byte ptr [rbp + 0x2e5], 1
0x14026f3ec e8bf926602 call 0x1428d86b0 ; std::panicking::panic_count::is_zero_slow_path
0x14026f3f1 3401 xor al, 1
0x14026f3f3 8985d0020000 mov dword ptr [rbp + 0x2d0], eax
0x14026f3f9 4889bdd8020000 mov qword ptr [rbp + 0x2d8], rdi
0x14026f400 0fb64611 movzx eax, byte ptr [rsi + 0x11]
0x14026f404 84c0 test al, al
0x14026f406 0f8408f4ffff je 0x14026e814
0x14026f40c e9c4070000 jmp 0x14026fbd5
0x14026f411 c685e502000000 mov byte ptr [rbp + 0x2e5], 0
0x14026f418 e893926602 call 0x1428d86b0 ; std::panicking::panic_count::is_zero_slow_path
0x14026f41d 84c0 test al, al
0x14026f41f 0f859ef4ffff jne 0x14026e8c3
0x14026f425 c6461101 mov byte ptr [rsi + 0x11], 1
0x14026f429 e995f4ffff jmp 0x14026e8c3
0x14026f42e 6690 nop 
0x14026f430 807dd000 cmp byte ptr [rbp - 0x30], 0
0x14026f434 0f85e6050000 jne 0x14026fa20
0x14026f43a 4c89ea mov rdx, r13
0x14026f43d f6c301 test bl, 1
0x14026f440 0f84c8000000 je 0x14026f50e
0x14026f446 0f57c0 xorps xmm0, xmm0
0x14026f449 f3410f5ac0 cvtss2sd xmm0, xmm8
0x14026f44e 4c8bbdc0020000 mov r15, qword ptr [rbp + 0x2c0]
0x14026f455 e8c6834200 call 0x140697820 ; winit::dpi::impl$5::from_f64
0x14026f45a 89c7 mov edi, eax
0x14026f45c 0f57c0 xorps xmm0, xmm0
0x14026f45f f30f5ac7 cvtss2sd xmm0, xmm7
0x14026f463 e8b8834200 call 0x140697820 ; winit::dpi::impl$5::from_f64
0x14026f468 89c6 mov esi, eax
0x14026f46a 498d4f10 lea rcx, [r15 + 0x10]
0x14026f46e e80de24200 call 0x14069d680 ; winit::platform_impl::platform::window::Window::window_state_lock
0x14026f473 4889c3 mov rbx, rax
0x14026f476 f6c201 test dl, 1
0x14026f479 7516 jne 0x14026f491
0x14026f47b 488b059ecf3b03 mov rax, qword ptr [rip + 0x33bcf9e]
0x14026f482 488b00 mov rax, qword ptr [rax]
0x14026f485 48d1e0 shl rax, 1
0x14026f488 4885c0 test rax, rax
0x14026f48b 0f85fe040000 jne 0x14026f98f
0x14026f491 31c0 xor eax, eax
0x14026f493 8603 xchg byte ptr [rbx], al
0x14026f495 3c02 cmp al, 2
0x14026f497 0f84cb040000 je 0x14026f968
0x14026f49d 0f57c0 xorps xmm0, xmm0
0x14026f4a0 f20f2ac7 cvtsi2sd xmm0, edi
0x14026f4a4 e877834200 call 0x140697820 ; winit::dpi::impl$5::from_f64
0x14026f4a9 89c7 mov edi, eax
0x14026f4ab 0f57c0 xorps xmm0, xmm0
0x14026f4ae f20f2ac6 cvtsi2sd xmm0, esi
0x14026f4b2 e869834200 call 0x140697820 ; winit::dpi::impl$5::from_f64
0x14026f4b7 89c3 mov ebx, eax
0x14026f4b9 0f57c0 xorps xmm0, xmm0
0x14026f4bc f20f2ac7 cvtsi2sd xmm0, edi
0x14026f4c0 e85b834200 call 0x140697820 ; winit::dpi::impl$5::from_f64
0x14026f4c5 89c6 mov esi, eax
0x14026f4c7 0f57c0 xorps xmm0, xmm0
0x14026f4ca f20f2ac3 cvtsi2sd xmm0, ebx
0x14026f4ce e84d834200 call 0x140697820 ; winit::dpi::impl$5::from_f64
0x14026f4d3 89b520020000 mov dword ptr [rbp + 0x220], esi
0x14026f4d9 898524020000 mov dword ptr [rbp + 0x224], eax
0x14026f4df 498b4f18 mov rcx, qword ptr [r15 + 0x18]
0x14026f4e3 4c89e2 mov rdx, r12
0x14026f4e6 ff150c33f202 call qword ptr [rip + 0x2f2330c]
0x14026f4ec 85c0 test eax, eax
0x14026f4ee 7416 je 0x14026f506
0x14026f4f0 8b8d20020000 mov ecx, dword ptr [rbp + 0x220]
0x14026f4f6 8b9524020000 mov edx, dword ptr [rbp + 0x224]
0x14026f4fc ff15fe34f202 call qword ptr [rip + 0x2f234fe]
0x14026f502 85c0 test eax, eax
0x14026f504 7505 jne 0x14026f50b
0x14026f506 e865c6af00 call 0x140d6bb70 ; std::sys::pal::windows::os::errno
0x14026f50b 4c89ea mov rdx, r13
0x14026f50e 83fa04 cmp edx, 4
0x14026f511 0f8529020000 jne 0x14026f740
0x14026f517 488bbd18020000 mov rdi, qword ptr [rbp + 0x218]
0x14026f51e 48c7851802000000000000 mov qword ptr [rbp + 0x218], 0
0x14026f529 488d8500020000 lea rax, [rbp + 0x200]
0x14026f530 488945c8 mov qword ptr [rbp - 0x38], rax
0x14026f534 48897dd0 mov qword ptr [rbp - 0x30], rdi
0x14026f538 488d45e4 lea rax, [rbp - 0x1c]
0x14026f53c 0f1170f4 movups xmmword ptr [rax - 0xc], xmm6
0x14026f540 48897de8 mov qword ptr [rbp - 0x18], rdi
0x14026f544 31f6 xor esi, esi
0x14026f546 4889fb mov rbx, rdi
0x14026f549 0f1f8000000000 nop dword ptr [rax]
0x14026f550 4883eb01 sub rbx, 1
0x14026f554 727a jb 0x14026f5d0
0x14026f556 488b8510020000 mov rax, qword ptr [rbp + 0x210]
0x14026f55d 4801f0 add rax, rsi
0x14026f560 488b8d00020000 mov rcx, qword ptr [rbp + 0x200]
0x14026f567 4839c8 cmp rax, rcx
0x14026f56a ba00000000 mov edx, 0
0x14026f56f 480f43d1 cmovae rdx, rcx
0x14026f573 488b8d08020000 mov rcx, qword ptr [rbp + 0x208]
0x14026f57a 4829d0 sub rax, rdx
0x14026f57d 48ffc6 inc rsi
0x14026f580 488975d8 mov qword ptr [rbp - 0x28], rsi
0x14026f584 48895de8 mov qword ptr [rbp - 0x18], rbx
0x14026f588 488b04c1 mov rax, qword ptr [rcx + rax*8]
0x14026f58c 48898528020000 mov qword ptr [rbp + 0x228], rax
0x14026f593 66c785200200000f00 mov word ptr [rbp + 0x220], 0xf
0x14026f59c 488d8d10010000 lea rcx, [rbp + 0x110]
0x14026f5a3 488d95c8010000 lea rdx, [rbp + 0x1c8]
0x14026f5aa 4d89e0 mov r8, r12
0x14026f5ad e8fe621500 call 0x1403c58b0 ; std::sync::mpmc::Sender::send<enum2$<tiny_glade::event_loop::OsWindowEvent> >
0x14026f5b2 0fb78510010000 movzx eax, word ptr [rbp + 0x110]
0x14026f5b9 6683f812 cmp ax, 0x12
0x14026f5bd 7491 je 0x14026f550
0x14026f5bf e9e1030000 jmp 0x14026f9a5
0x14026f5c4 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14026f5d0 4c8b8518020000 mov r8, qword ptr [rbp + 0x218]
0x14026f5d7 4d85c0 test r8, r8
0x14026f5da 0f8595030000 jne 0x14026f975
0x14026f5e0 488d8510020000 lea rax, [rbp + 0x210]
0x14026f5e7 0f1130 movups xmmword ptr [rax], xmm6
0x14026f5ea 80bde302000000 cmp byte ptr [rbp + 0x2e3], 0
0x14026f5f1 0f8479020000 je 0x14026f870
0x14026f5f7 c685e302000000 mov byte ptr [rbp + 0x2e3], 0
0x14026f5fe 837d7801 cmp dword ptr [rbp + 0x78], 1
0x14026f602 0f8568020000 jne 0x14026f870
0x14026f608 488d4dc8 lea rcx, [rbp - 0x38]
0x14026f60c 488d9580000000 lea rdx, [rbp + 0x80]
0x14026f613 e8e8cd1300 call 0x1403ac400 ; tiny_glade::icc::get_monitor_icc_profile
0x14026f618 31c0 xor eax, eax
0x14026f61a 483b45c8 cmp rax, qword ptr [rbp - 0x38]
0x14026f61e 0f81fc010000 jno 0x14026f820
0x14026f624 488b45d0 mov rax, qword ptr [rbp - 0x30]
0x14026f628 48898598000000 mov qword ptr [rbp + 0x98], rax
0x14026f62f 488b0572cd3b03 mov rax, qword ptr [rip + 0x33bcd72]
0x14026f636 488b00 mov rax, qword ptr [rax]
0x14026f639 4883f802 cmp rax, 2
0x14026f63d 0f82df000000 jb 0x14026f722
0x14026f643 488d8598000000 lea rax, [rbp + 0x98]
0x14026f64a 48898510010000 mov qword ptr [rbp + 0x110], rax
0x14026f651 488d05c8d5ad00 lea rax, [rip + 0xadd5c8]
0x14026f658 48898518010000 mov qword ptr [rbp + 0x118], rax
0x14026f65f 488d0dba867302 lea rcx, [rip + 0x27386ba]
0x14026f666 e815e74000 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x14026f66b 660f1000 movupd xmm0, xmmword ptr [rax]
0x14026f66f 8b4010 mov eax, dword ptr [rax + 0x10]
0x14026f672 48c7855002000002000000 mov qword ptr [rbp + 0x250], 2
0x14026f67d 488d0dfc847302 lea rcx, [rip + 0x27384fc]
0x14026f684 48898d58020000 mov qword ptr [rbp + 0x258], rcx
0x14026f68b 48c7856002000016000000 mov qword ptr [rbp + 0x260], 0x16
0x14026f696 488d1573867302 lea rdx, [rip + 0x2738673]
0x14026f69d 48899570020000 mov qword ptr [rbp + 0x270], rdx
0x14026f6a4 48c7857802000001000000 mov qword ptr [rbp + 0x278], 1
0x14026f6af 488d9510010000 lea rdx, [rbp + 0x110]
0x14026f6b6 48899580020000 mov qword ptr [rbp + 0x280], rdx
0x14026f6bd 48c7858802000001000000 mov qword ptr [rbp + 0x288], 1
0x14026f6c8 48c7859002000000000000 mov qword ptr [rbp + 0x290], 0
0x14026f6d3 48c7852002000000000000 mov qword ptr [rbp + 0x220], 0
0x14026f6de 48898d28020000 mov qword ptr [rbp + 0x228], rcx
0x14026f6e5 48c7853002000016000000 mov qword ptr [rbp + 0x230], 0x16
0x14026f6f0 48c7853802000000000000 mov qword ptr [rbp + 0x238], 0
0x14026f6fb 660f118540020000 movupd xmmword ptr [rbp + 0x240], xmm0
0x14026f703 c7856802000001000000 mov dword ptr [rbp + 0x268], 1
0x14026f70d 89856c020000 mov dword ptr [rbp + 0x26c], eax
0x14026f713 488d8de4020000 lea rcx, [rbp + 0x2e4]
0x14026f71a 4c89e2 mov rdx, r12
0x14026f71d e80eacad00 call 0x140d4a330 ; log::__private_api::impl$2::log
0x14026f722 488d8d98000000 lea rcx, [rbp + 0x98]
0x14026f729 e802d5ad00 call 0x140d4cc30 ; anyhow::error::impl$6::drop
0x14026f72e e93d010000 jmp 0x14026f870
0x14026f733 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14026f740 8d42fe lea eax, [rdx - 2]
0x14026f743 83f802 cmp eax, 2
0x14026f746 b902000000 mov ecx, 2
0x14026f74b 0f43c1 cmovae eax, ecx
0x14026f74e 85c0 test eax, eax
0x14026f750 745e je 0x14026f7b0
0x14026f752 83f801 cmp eax, 1
0x14026f755 0f859c000000 jne 0x14026f7f7
0x14026f75b 488b85f0010000 mov rax, qword ptr [rbp + 0x1f0]
0x14026f762 85c0 test eax, eax
0x14026f764 0f8437040000 je 0x14026fba1
0x14026f76a 8b8dac020000 mov ecx, dword ptr [rbp + 0x2ac]
0x14026f770 f7d0 not eax
0x14026f772 21c8 and eax, ecx
0x14026f774 8985ac020000 mov dword ptr [rbp + 0x2ac], eax
0x14026f77a 85c9 test ecx, ecx
0x14026f77c 0f8495fdffff je 0x14026f517
0x14026f782 b201 mov dl, 1
0x14026f784 85c0 test eax, eax
0x14026f786 0f858bfdffff jne 0x14026f517
0x14026f78c 83bdbc02000000 cmp dword ptr [rbp + 0x2bc], 0
0x14026f793 744d je 0x14026f7e2
0x14026f795 488b85e8010000 mov rax, qword ptr [rbp + 0x1e8]
0x14026f79c 4885c0 test rax, rax
0x14026f79f 0f8472fdffff je 0x14026f517
0x14026f7a5 0fb6ca movzx ecx, dl
0x14026f7a8 894808 mov dword ptr [rax + 8], ecx
0x14026f7ab e967fdffff jmp 0x14026f517
0x14026f7b0 488b95f0010000 mov rdx, qword ptr [rbp + 0x1f0]
0x14026f7b7 85d2 test edx, edx
0x14026f7b9 0f84fc030000 je 0x14026fbbb
0x14026f7bf 8b85ac020000 mov eax, dword ptr [rbp + 0x2ac]
0x14026f7c5 89c1 mov ecx, eax
0x14026f7c7 09d1 or ecx, edx
0x14026f7c9 898dac020000 mov dword ptr [rbp + 0x2ac], ecx
0x14026f7cf 85c0 test eax, eax
0x14026f7d1 0f8540fdffff jne 0x14026f517
0x14026f7d7 31d2 xor edx, edx
0x14026f7d9 83bdbc02000000 cmp dword ptr [rbp + 0x2bc], 0
0x14026f7e0 75b3 jne 0x14026f795
0x14026f7e2 488b8dc0020000 mov rcx, qword ptr [rbp + 0x2c0]
0x14026f7e9 4883c110 add rcx, 0x10
0x14026f7ed e87ee3ffff call 0x14026db70 ; winit::window::Window::set_cursor_visible
0x14026f7f2 e962010000 jmp 0x14026f959
0x14026f7f7 83bdbc02000000 cmp dword ptr [rbp + 0x2bc], 0
0x14026f7fe 0f84eb000000 je 0x14026f8ef
0x14026f804 85d2 test edx, edx
0x14026f806 0f8433010000 je 0x14026f93f
0x14026f80c 8995bc020000 mov dword ptr [rbp + 0x2bc], edx
0x14026f812 83fa01 cmp edx, 1
0x14026f815 0f84f5000000 je 0x14026f910
0x14026f81b e9f7fcffff jmp 0x14026f517
0x14026f820 660f1045c8 movupd xmm0, xmmword ptr [rbp - 0x38]
0x14026f825 0f104dd8 movups xmm1, xmmword ptr [rbp - 0x28]
0x14026f829 488d8528020000 lea rax, [rbp + 0x228]
0x14026f830 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x14026f834 660f1100 movupd xmmword ptr [rax], xmm0
0x14026f838 66c785200200001100 mov word ptr [rbp + 0x220], 0x11
0x14026f841 488d8d10010000 lea rcx, [rbp + 0x110]
0x14026f848 488d95c8010000 lea rdx, [rbp + 0x1c8]
0x14026f84f 4d89e0 mov r8, r12
0x14026f852 e859601500 call 0x1403c58b0 ; std::sync::mpmc::Sender::send<enum2$<tiny_glade::event_loop::OsWindowEvent> >
0x14026f857 6683bd1001000012 cmp word ptr [rbp + 0x110], 0x12
0x14026f85f 0f8511030000 jne 0x14026fb76
0x14026f865 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14026f870 e86b3daf00 call 0x140d635e0 ; std::time::Instant::now
0x14026f875 48894568 mov qword ptr [rbp + 0x68], rax
0x14026f879 895570 mov dword ptr [rbp + 0x70], edx
0x14026f87c 488d4d68 lea rcx, [rbp + 0x68]
0x14026f880 488b9508010000 mov rdx, qword ptr [rbp + 0x108]
0x14026f887 448b85fc010000 mov r8d, dword ptr [rbp + 0x1fc]
0x14026f88e e8ed3daf00 call 0x140d63680 ; std::time::Instant::duration_since
0x14026f893 31c9 xor ecx, ecx
0x14026f895 85d2 test edx, edx
0x14026f897 0f95c1 setne cl
0x14026f89a 31d2 xor edx, edx
0x14026f89c 4883f802 cmp rax, 2
0x14026f8a0 0f93c2 setae dl
0x14026f8a3 4883f801 cmp rax, 1
0x14026f8a7 0f45ca cmovne ecx, edx
0x14026f8aa 80f901 cmp cl, 1
0x14026f8ad 752f jne 0x14026f8de
0x14026f8af 83bdbc02000001 cmp dword ptr [rbp + 0x2bc], 1
0x14026f8b6 7512 jne 0x14026f8ca
0x14026f8b8 488b8dc0020000 mov rcx, qword ptr [rbp + 0x2c0]
0x14026f8bf 4883c110 add rcx, 0x10
0x14026f8c3 31d2 xor edx, edx
0x14026f8c5 e8a6e2ffff call 0x14026db70 ; winit::window::Window::set_cursor_visible
0x14026f8ca 488b4568 mov rax, qword ptr [rbp + 0x68]
0x14026f8ce 48898508010000 mov qword ptr [rbp + 0x108], rax
0x14026f8d5 8b4570 mov eax, dword ptr [rbp + 0x70]
0x14026f8d8 8985fc010000 mov dword ptr [rbp + 0x1fc], eax
0x14026f8de 31c9 xor ecx, ecx
0x14026f8e0 ba40420f00 mov edx, 0xf4240
0x14026f8e5 e89660ae00 call 0x140d55980 ; std::thread::sleep
0x14026f8ea e9e8ecffff jmp 0x14026e5d7
0x14026f8ef 8995bc020000 mov dword ptr [rbp + 0x2bc], edx
0x14026f8f5 83fa01 cmp edx, 1
0x14026f8f8 0f8519fcffff jne 0x14026f517
0x14026f8fe 488b8dc0020000 mov rcx, qword ptr [rbp + 0x2c0]
0x14026f905 4883c110 add rcx, 0x10
0x14026f909 31d2 xor edx, edx
0x14026f90b e860e2ffff call 0x14026db70 ; winit::window::Window::set_cursor_visible
0x14026f910 488b8df0010000 mov rcx, qword ptr [rbp + 0x1f0]
0x14026f917 898df8010000 mov dword ptr [rbp + 0x1f8], ecx
0x14026f91d 488b85e8010000 mov rax, qword ptr [rbp + 0x1e8]
0x14026f924 c785bc02000001000000 mov dword ptr [rbp + 0x2bc], 1
0x14026f92e 4885c0 test rax, rax
0x14026f931 0f84e0fbffff je 0x14026f517
0x14026f937 89480c mov dword ptr [rax + 0xc], ecx
0x14026f93a e9d8fbffff jmp 0x14026f517
0x14026f93f 488b8dc0020000 mov rcx, qword ptr [rbp + 0x2c0]
0x14026f946 4883c110 add rcx, 0x10
0x14026f94a 83bdac02000000 cmp dword ptr [rbp + 0x2ac], 0
0x14026f951 0f94c2 sete dl
0x14026f954 e817e2ffff call 0x14026db70 ; winit::window::Window::set_cursor_visible
0x14026f959 c785bc02000000000000 mov dword ptr [rbp + 0x2bc], 0
0x14026f963 e9affbffff jmp 0x14026f517
0x14026f968 4889d9 mov rcx, rbx
0x14026f96b e8a08e6602 call 0x1428d8810 ; std::sys::sync::mutex::futex::Mutex::wake
0x14026f970 e928fbffff jmp 0x14026f49d
0x14026f975 4d89c1 mov r9, r8
0x14026f978 49f7d9 neg r9
0x14026f97b 488d8d00020000 lea rcx, [rbp + 0x200]
0x14026f982 4889fa mov rdx, rdi
0x14026f985 e836266302 call 0x1428a1fc0 ; alloc::collections::vec_deque::drain::impl$4::drop::impl$0::drop::join_head_and_tail_wrapping<tiny_glade::event_loop::CursorRequestId,alloc::alloc::Global>
0x14026f98a e951fcffff jmp 0x14026f5e0
0x14026f98f e81c8d6602 call 0x1428d86b0 ; std::panicking::panic_count::is_zero_slow_path
0x14026f994 84c0 test al, al
0x14026f996 0f85f5faffff jne 0x14026f491
0x14026f99c c6430101 mov byte ptr [rbx + 1], 1
0x14026f9a0 e9ecfaffff jmp 0x14026f491
0x14026f9a5 0fb7c8 movzx ecx, ax
0x14026f9a8 83c1fc add ecx, -4
0x14026f9ab 6683f90e cmp cx, 0xe
0x14026f9af b807000000 mov eax, 7
0x14026f9b4 0f42c1 cmovb eax, ecx
0x14026f9b7 6683f80c cmp ax, 0xc
0x14026f9bb 0f872e010000 ja 0x14026faef
0x14026f9c1 0fb7c0 movzx eax, ax
0x14026f9c4 b95f1f0000 mov ecx, 0x1f5f
0x14026f9c9 0fa3c1 bt ecx, eax
0x14026f9cc 0f83eb000000 jae 0x14026fabd
0x14026f9d2 4c8b8518020000 mov r8, qword ptr [rbp + 0x218]
0x14026f9d9 4d85c0 test r8, r8
0x14026f9dc 0f85a5010000 jne 0x14026fb87
0x14026f9e2 660f57c0 xorpd xmm0, xmm0
0x14026f9e6 488d8510020000 lea rax, [rbp + 0x210]
0x14026f9ed 660f1100 movupd xmmword ptr [rax], xmm0
0x14026f9f1 eb2d jmp 0x14026fa20
0x14026f9f3 0fb7c8 movzx ecx, ax
0x14026f9f6 83c1fc add ecx, -4
0x14026f9f9 6683f90e cmp cx, 0xe
0x14026f9fd b807000000 mov eax, 7
0x14026fa02 0f42c1 cmovb eax, ecx
0x14026fa05 6683f80c cmp ax, 0xc
0x14026fa09 0f8740010000 ja 0x14026fb4f
0x14026fa0f 0fb7c0 movzx eax, ax
0x14026fa12 b95f1f0000 mov ecx, 0x1f5f
0x14026fa17 0fa3c1 bt ecx, eax
0x14026fa1a 0f83f6000000 jae 0x14026fb16
0x14026fa20 488b9500020000 mov rdx, qword ptr [rbp + 0x200]
0x14026fa27 4885d2 test rdx, rdx
0x14026fa2a 7416 je 0x14026fa42
0x14026fa2c 488b8d08020000 mov rcx, qword ptr [rbp + 0x208]
0x14026fa33 48c1e203 shl rdx, 3
0x14026fa37 41b808000000 mov r8d, 8
0x14026fa3d e8de413a00 call 0x140613c20 ; __rust_dealloc
0x14026fa42 488d8d88000000 lea rcx, [rbp + 0x88]
0x14026fa49 e872853900 call 0x140607fc0 ; core::ptr::drop_in_place<std::sync::mpsc::Receiver<enum2$<tiny_glade::event_loop::GameEvent> > >
0x14026fa4e 488d8dc8010000 lea rcx, [rbp + 0x1c8]
0x14026fa55 e896933900 call 0x140608df0 ; core::ptr::drop_in_place<std::sync::mpsc::Sender<enum2$<tiny_glade::event_loop::OsWindowEvent> > >
0x14026fa5a 488b8de0010000 mov rcx, qword ptr [rbp + 0x1e0]
0x14026fa61 e85a1e3800 call 0x1405f18c0 ; core::ptr::drop_in_place<tiny_glade::event_loop::EguiEventIo>
0x14026fa66 488b85c0020000 mov rax, qword ptr [rbp + 0x2c0]
0x14026fa6d f048ff08 lock dec qword ptr [rax]
0x14026fa71 750c jne 0x14026fa7f
0x14026fa73 488d8dc0020000 lea rcx, [rbp + 0x2c0]
0x14026fa7a e8c11d3c00 call 0x140631840 ; alloc::sync::Arc::drop_slow<winit::window::Window,alloc::alloc::Global>
0x14026fa7f 488b8dd8010000 mov rcx, qword ptr [rbp + 0x1d8]
0x14026fa86 e8459d3800 call 0x1405f97d0 ; core::ptr::drop_in_place<winit::event_loop::EventLoop<tuple$<> > >
0x14026fa8b 0f28b5f0020000 movaps xmm6, xmmword ptr [rbp + 0x2f0]
0x14026fa92 0f28bd00030000 movaps xmm7, xmmword ptr [rbp + 0x300]
0x14026fa99 440f288510030000 movaps xmm8, xmmword ptr [rbp + 0x310]
0x14026faa1 440f288d20030000 movaps xmm9, xmmword ptr [rbp + 0x320]
0x14026faa9 4881c4b8030000 add rsp, 0x3b8
0x14026fab0 5b pop rbx
0x14026fab1 5f pop rdi
0x14026fab2 5e pop rsi
0x14026fab3 415c pop r12
0x14026fab5 415d pop r13
0x14026fab7 415e pop r14
0x14026fab9 415f pop r15
0x14026fabb 5d pop rbp
0x14026fabc c3 ret 
0x14026fabd 83f805 cmp eax, 5
0x14026fac0 751c jne 0x14026fade
0x14026fac2 488b9518010000 mov rdx, qword ptr [rbp + 0x118]
0x14026fac9 48b80200000000000080 movabs rax, 0x8000000000000002
0x14026fad3 4839c2 cmp rdx, rax
0x14026fad6 0f8cf6feffff jl 0x14026f9d2
0x14026fadc eb18 jmp 0x14026faf6
0x14026fade 488d8d10010000 lea rcx, [rbp + 0x110]
0x14026fae5 e816853700 call 0x1405e8000 ; core::ptr::drop_in_place<winit::event::KeyEvent>
0x14026faea e9e3feffff jmp 0x14026f9d2
0x14026faef 488b9518010000 mov rdx, qword ptr [rbp + 0x118]
0x14026faf6 4885d2 test rdx, rdx
0x14026faf9 0f84d3feffff je 0x14026f9d2
0x14026faff 488b8d20010000 mov rcx, qword ptr [rbp + 0x120]
0x14026fb06 41b801000000 mov r8d, 1
0x14026fb0c e80f413a00 call 0x140613c20 ; __rust_dealloc
0x14026fb11 e9bcfeffff jmp 0x14026f9d2
0x14026fb16 83f805 cmp eax, 5
0x14026fb19 751c jne 0x14026fb37
0x14026fb1b 488b9518010000 mov rdx, qword ptr [rbp + 0x118]
0x14026fb22 48b80200000000000080 movabs rax, 0x8000000000000002
0x14026fb2c 4839c2 cmp rdx, rax
0x14026fb2f 0f8cebfeffff jl 0x14026fa20
0x14026fb35 eb1f jmp 0x14026fb56
0x14026fb37 c685e702000000 mov byte ptr [rbp + 0x2e7], 0
0x14026fb3e 488d8d10010000 lea rcx, [rbp + 0x110]
0x14026fb45 e8b6843700 call 0x1405e8000 ; core::ptr::drop_in_place<winit::event::KeyEvent>
0x14026fb4a e9d1feffff jmp 0x14026fa20
0x14026fb4f 488b9518010000 mov rdx, qword ptr [rbp + 0x118]
0x14026fb56 4885d2 test rdx, rdx
0x14026fb59 0f84c1feffff je 0x14026fa20
0x14026fb5f 488b8d20010000 mov rcx, qword ptr [rbp + 0x120]
0x14026fb66 41b801000000 mov r8d, 1
0x14026fb6c e8af403a00 call 0x140613c20 ; __rust_dealloc
0x14026fb71 e9aafeffff jmp 0x14026fa20
0x14026fb76 488d8d10010000 lea rcx, [rbp + 0x110]
0x14026fb7d e81e91ffff call 0x140268ca0 ; core::ptr::drop_in_place<std::sync::mpsc::SendError<enum2$<tiny_glade::event_loop::OsWindowEvent> > >
0x14026fb82 e999feffff jmp 0x14026fa20
0x14026fb87 4d89c1 mov r9, r8
0x14026fb8a 49f7d9 neg r9
0x14026fb8d 488d8d00020000 lea rcx, [rbp + 0x200]
0x14026fb94 4889fa mov rdx, rdi
0x14026fb97 e824246302 call 0x1428a1fc0 ; alloc::collections::vec_deque::drain::impl$4::drop::impl$0::drop::join_head_and_tail_wrapping<tiny_glade::event_loop::CursorRequestId,alloc::alloc::Global>
0x14026fb9c e941feffff jmp 0x14026f9e2
0x14026fba1 488d0de0807302 lea rcx, [rip + 0x27380e0]
0x14026fba8 4c8d0519817302 lea r8, [rip + 0x2738119]
0x14026fbaf ba24000000 mov edx, 0x24
0x14026fbb4 e877986602 call 0x1428d9430 ; core::panicking::panic
0x14026fbb9 eb5f jmp 0x14026fc1a
0x14026fbbb 488d0dc6807302 lea rcx, [rip + 0x27380c6]
0x14026fbc2 4c8d05e7807302 lea r8, [rip + 0x27380e7]
0x14026fbc9 ba24000000 mov edx, 0x24
0x14026fbce e85d986602 call 0x1428d9430 ; core::panicking::panic