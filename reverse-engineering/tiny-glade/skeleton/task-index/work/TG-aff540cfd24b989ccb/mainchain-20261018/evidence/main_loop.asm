0x1404d9580 55 push rbp
0x1404d9581 4157 push r15
0x1404d9583 4156 push r14
0x1404d9585 4155 push r13
0x1404d9587 4154 push r12
0x1404d9589 56 push rsi
0x1404d958a 57 push rdi
0x1404d958b 53 push rbx
0x1404d958c b8d8150000 mov eax, 0x15d8
0x1404d9591 e8aa143c02 call 0x14289aa40 ; __chkstk
0x1404d9596 4829c4 sub rsp, rax
0x1404d9599 488dac2480000000 lea rbp, [rsp + 0x80]
0x1404d95a1 66440f7fbd40150000 movdqa xmmword ptr [rbp + 0x1540], xmm15
0x1404d95aa 440f29b530150000 movaps xmmword ptr [rbp + 0x1530], xmm14
0x1404d95b2 66440f7fad20150000 movdqa xmmword ptr [rbp + 0x1520], xmm13
0x1404d95bb 440f29a510150000 movaps xmmword ptr [rbp + 0x1510], xmm12
0x1404d95c3 440f299d00150000 movaps xmmword ptr [rbp + 0x1500], xmm11
0x1404d95cb 66440f2995f0140000 movapd xmmword ptr [rbp + 0x14f0], xmm10
0x1404d95d4 66440f7f8de0140000 movdqa xmmword ptr [rbp + 0x14e0], xmm9
0x1404d95dd 440f2985d0140000 movaps xmmword ptr [rbp + 0x14d0], xmm8
0x1404d95e5 660f7fbdc0140000 movdqa xmmword ptr [rbp + 0x14c0], xmm7
0x1404d95ed 660f29b5b0140000 movapd xmmword ptr [rbp + 0x14b0], xmm6
0x1404d95f5 48c785a8140000feffffff mov qword ptr [rbp + 0x14a8], 0xfffffffffffffffe
0x1404d9600 48c785b013000000000000 mov qword ptr [rbp + 0x13b0], 0
0x1404d960b 48c785b813000008000000 mov qword ptr [rbp + 0x13b8], 8
0x1404d9616 48c785c013000000000000 mov qword ptr [rbp + 0x13c0], 0
0x1404d9621 48898d78140000 mov qword ptr [rbp + 0x1478], rcx
0x1404d9628 e8438e0000 call 0x1404e2470 ; tiny_glade::game_loop::GameLoop::extract_available_monitors_info
0x1404d962d 488b8578140000 mov rax, qword ptr [rbp + 0x1478]
0x1404d9634 488d4830 lea rcx, [rax + 0x30]
0x1404d9638 48898d50140000 mov qword ptr [rbp + 0x1450], rcx
0x1404d963f 488d4868 lea rcx, [rax + 0x68]
0x1404d9643 48898d88140000 mov qword ptr [rbp + 0x1488], rcx
0x1404d964a 488d88880a0000 lea rcx, [rax + 0xa88]
0x1404d9651 48898d10140000 mov qword ptr [rbp + 0x1410], rcx
0x1404d9658 488d4820 lea rcx, [rax + 0x20]
0x1404d965c 48898d18140000 mov qword ptr [rbp + 0x1418], rcx
0x1404d9663 488d8815110000 lea rcx, [rax + 0x1115]
0x1404d966a 48898da8130000 mov qword ptr [rbp + 0x13a8], rcx
0x1404d9671 66440f281536ff5002 movapd xmm10, xmmword ptr [rip + 0x250ff36]
0x1404d967a 450f57db xorps xmm11, xmm11
0x1404d967e f3440f102549714c02 movss xmm12, dword ptr [rip + 0x24c7149]
0x1404d9687 66440f6f2d60c24402 movdqa xmm13, xmmword ptr [rip + 0x244c260]
0x1404d9690 66440f6f0dd77e4802 movdqa xmm9, xmmword ptr [rip + 0x2487ed7]
0x1404d9699 488d88c8100000 lea rcx, [rax + 0x10c8]
0x1404d96a0 48898d20140000 mov qword ptr [rbp + 0x1420], rcx
0x1404d96a7 488d88c0000000 lea rcx, [rax + 0xc0]
0x1404d96ae 48898dd8130000 mov qword ptr [rbp + 0x13d8], rcx
0x1404d96b5 488d88a0060000 lea rcx, [rax + 0x6a0]
0x1404d96bc 48898d50130000 mov qword ptr [rbp + 0x1350], rcx
0x1404d96c3 488d8890080000 lea rcx, [rax + 0x890]
0x1404d96ca 48898dc8130000 mov qword ptr [rbp + 0x13c8], rcx
0x1404d96d1 488d88e0080000 lea rcx, [rax + 0x8e0]
0x1404d96d8 48898d70130000 mov qword ptr [rbp + 0x1370], rcx
0x1404d96df 488d8880090000 lea rcx, [rax + 0x980]
0x1404d96e6 48898dd0130000 mov qword ptr [rbp + 0x13d0], rcx
0x1404d96ed 488d88d0090000 lea rcx, [rax + 0x9d0]
0x1404d96f4 48898d78130000 mov qword ptr [rbp + 0x1378], rcx
0x1404d96fb 488d88e0090000 lea rcx, [rax + 0x9e0]
0x1404d9702 48898d90130000 mov qword ptr [rbp + 0x1390], rcx
0x1404d9709 488d88f0100000 lea rcx, [rax + 0x10f0]
0x1404d9710 48898d88130000 mov qword ptr [rbp + 0x1388], rcx
0x1404d9717 488d8830070000 lea rcx, [rax + 0x730]
0x1404d971e 48898d30130000 mov qword ptr [rbp + 0x1330], rcx
0x1404d9725 488d8888060000 lea rcx, [rax + 0x688]
0x1404d972c 48898d38130000 mov qword ptr [rbp + 0x1338], rcx
0x1404d9733 488d88b0100000 lea rcx, [rax + 0x10b0]
0x1404d973a 48898d40130000 mov qword ptr [rbp + 0x1340], rcx
0x1404d9741 488d88900a0000 lea rcx, [rax + 0xa90]
0x1404d9748 48898d48130000 mov qword ptr [rbp + 0x1348], rcx
0x1404d974f 488d88b0080000 lea rcx, [rax + 0x8b0]
0x1404d9756 48898d58130000 mov qword ptr [rbp + 0x1358], rcx
0x1404d975d 488d88c0080000 lea rcx, [rax + 0x8c0]
0x1404d9764 48898d98130000 mov qword ptr [rbp + 0x1398], rcx
0x1404d976b 488d88a0090000 lea rcx, [rax + 0x9a0]
0x1404d9772 48898d60130000 mov qword ptr [rbp + 0x1360], rcx
0x1404d9779 488d88b0090000 lea rcx, [rax + 0x9b0]
0x1404d9780 48898da0130000 mov qword ptr [rbp + 0x13a0], rcx
0x1404d9787 4805400a0000 add rax, 0xa40
0x1404d978d 48898568130000 mov qword ptr [rbp + 0x1368], rax
0x1404d9794 48c7858013000000000000 mov qword ptr [rbp + 0x1380], 0
0x1404d979f 660f76ff pcmpeqd xmm7, xmm7
0x1404d97a3 eb27 jmp 0x1404d97cc
0x1404d97a5 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404d97b0 41b8c8000000 mov r8d, 0xc8
0x1404d97b6 488b8d98140000 mov rcx, qword ptr [rbp + 0x1498]
0x1404d97bd 4889f2 mov rdx, rsi
0x1404d97c0 e83ba44402 call 0x142923c00 ; memcpy
0x1404d97c5 48ff8580130000 inc qword ptr [rbp + 0x1380]
0x1404d97cc 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404d97d3 48babaa57efadc08df98 movabs rdx, 0x98df08dcfa7ea5ba
0x1404d97dd 49b8c32b630ea617b167 movabs r8, 0x67b117a60e632bc3
0x1404d97e7 e8043fc0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404d97ec 4885c0 test rax, rax
0x1404d97ef 4c8b8578140000 mov r8, qword ptr [rbp + 0x1478]
0x1404d97f6 7458 je 0x1404d9850
0x1404d97f8 49399048020000 cmp qword ptr [r8 + 0x248], rdx
0x1404d97ff 764f jbe 0x1404d9850
0x1404d9801 498b8040020000 mov rax, qword ptr [r8 + 0x240]
0x1404d9808 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404d980c 4885c9 test rcx, rcx
0x1404d980f 743f je 0x1404d9850
0x1404d9811 498b8010020000 mov rax, qword ptr [r8 + 0x210]
0x1404d9818 48f7d1 not rcx
0x1404d981b 488d0c49 lea rcx, [rcx + rcx*2]
0x1404d981f 48c1e105 shl rcx, 5
0x1404d9823 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404d9829 7425 je 0x1404d9850
0x1404d982b 4801c8 add rax, rcx
0x1404d982e 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1404d9832 e8d9046400 call 0x140b19d10 ; country_core::resources::logo_animation::LogoAnimation::is_waiting
0x1404d9837 84c0 test al, al
0x1404d9839 4c8b8578140000 mov r8, qword ptr [rbp + 0x1478]
0x1404d9840 0f841d020000 je 0x1404d9a63
0x1404d9846 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404d9850 498b00 mov rax, qword ptr [r8]
0x1404d9853 498b5008 mov rdx, qword ptr [r8 + 8]
0x1404d9857 4885c0 test rax, rax
0x1404d985a 7414 je 0x1404d9870
0x1404d985c 83f801 cmp eax, 1
0x1404d985f 751f jne 0x1404d9880
0x1404d9861 488d4dc0 lea rcx, [rbp - 0x40]
0x1404d9865 e8f6ddd8ff call 0x140267660 ; std::sync::mpmc::list::Channel::try_recv<bevy_app::sub_app::SubApp>
0x1404d986a eb1d jmp 0x1404d9889
0x1404d986c 0f1f4000 nop dword ptr [rax]
0x1404d9870 488d4dc0 lea rcx, [rbp - 0x40]
0x1404d9874 e817880100 call 0x1404f2090 ; std::sync::mpmc::array::Channel::try_recv<bevy_app::sub_app::SubApp>
0x1404d9879 eb0e jmp 0x1404d9889
0x1404d987b 0f1f440000 nop dword ptr [rax + rax]
0x1404d9880 488d4dc0 lea rcx, [rbp - 0x40]
0x1404d9884 e8276d0400 call 0x1405205b0 ; std::sync::mpmc::zero::Channel::try_recv<bevy_app::sub_app::SubApp>
0x1404d9889 31c0 xor eax, eax
0x1404d988b 483b45c0 cmp rax, qword ptr [rbp - 0x40]
0x1404d988f 0f80ce010000 jo 0x1404d9a63
0x1404d9895 41b828060000 mov r8d, 0x628
0x1404d989b 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404d98a2 488d55c0 lea rdx, [rbp - 0x40]
0x1404d98a6 e855a34402 call 0x142923c00 ; memcpy
0x1404d98ab 488b05f62a1503 mov rax, qword ptr [rip + 0x3152af6]
0x1404d98b2 488b00 mov rax, qword ptr [rax]
0x1404d98b5 4883f803 cmp rax, 3
0x1404d98b9 488bb578140000 mov rsi, qword ptr [rbp + 0x1478]
0x1404d98c0 488bbd10140000 mov rdi, qword ptr [rbp + 0x1410]
0x1404d98c7 488d9d300c0000 lea rbx, [rbp + 0xc30]
0x1404d98ce 0f82c7000000 jb 0x1404d999b
0x1404d98d4 c6859714000001 mov byte ptr [rbp + 0x1497], 1
0x1404d98db 488d0d5e045102 lea rcx, [rip + 0x251045e]
0x1404d98e2 e899441a00 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x1404d98e7 660f1000 movupd xmm0, xmmword ptr [rax]
0x1404d98eb 8b4010 mov eax, dword ptr [rax + 0x10]
0x1404d98ee 48c785600c000003000000 mov qword ptr [rbp + 0xc60], 3
0x1404d98f9 488d15e2035102 lea rdx, [rip + 0x25103e2]
0x1404d9900 488995680c0000 mov qword ptr [rbp + 0xc68], rdx
0x1404d9907 48c785700c000015000000 mov qword ptr [rbp + 0xc70], 0x15
0x1404d9912 488d0d17045102 lea rcx, [rip + 0x2510417]
0x1404d9919 48898d800c0000 mov qword ptr [rbp + 0xc80], rcx
0x1404d9920 48c785880c000001000000 mov qword ptr [rbp + 0xc88], 1
0x1404d992b 48c785900c000008000000 mov qword ptr [rbp + 0xc90], 8
0x1404d9936 48c785300c000000000000 mov qword ptr [rbp + 0xc30], 0
0x1404d9941 488d8d980c0000 lea rcx, [rbp + 0xc98]
0x1404d9948 660fefc9 pxor xmm1, xmm1
0x1404d994c f30f7f09 movdqu xmmword ptr [rcx], xmm1
0x1404d9950 488995380c0000 mov qword ptr [rbp + 0xc38], rdx
0x1404d9957 48c785400c000015000000 mov qword ptr [rbp + 0xc40], 0x15
0x1404d9962 48c785480c000000000000 mov qword ptr [rbp + 0xc48], 0
0x1404d996d 660f1185500c0000 movupd xmmword ptr [rbp + 0xc50], xmm0
0x1404d9975 c785780c000001000000 mov dword ptr [rbp + 0xc78], 1
0x1404d997f 89857c0c0000 mov dword ptr [rbp + 0xc7c], eax
0x1404d9985 c6859714000001 mov byte ptr [rbp + 0x1497], 1
0x1404d998c 488d8d77140000 lea rcx, [rbp + 0x1477]
0x1404d9993 4889da mov rdx, rbx
0x1404d9996 e895098700 call 0x140d4a330 ; log::__private_api::impl$2::log
0x1404d999b 41b828060000 mov r8d, 0x628
0x1404d99a1 4889d9 mov rcx, rbx
0x1404d99a4 488d9500060000 lea rdx, [rbp + 0x600]
0x1404d99ab e850a24402 call 0x142923c00 ; memcpy
0x1404d99b0 31c0 xor eax, eax
0x1404d99b2 483b07 cmp rax, qword ptr [rdi]
0x1404d99b5 700c jo 0x1404d99c3
0x1404d99b7 488b8d10140000 mov rcx, qword ptr [rbp + 0x1410]
0x1404d99be e89df11000 call 0x1405e8b60 ; core::ptr::drop_in_place<bevy_app::sub_app::SubApp>
0x1404d99c3 41b828060000 mov r8d, 0x628
0x1404d99c9 488b8d10140000 mov rcx, qword ptr [rbp + 0x1410]
0x1404d99d0 4889da mov rdx, rbx
0x1404d99d3 e828a24402 call 0x142923c00 ; memcpy
0x1404d99d8 c6859714000000 mov byte ptr [rbp + 0x1497], 0
0x1404d99df 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404d99e6 48babaa57efadc08df98 movabs rdx, 0x98df08dcfa7ea5ba
0x1404d99f0 49b8c32b630ea617b167 movabs r8, 0x67b117a60e632bc3
0x1404d99fa e8f13cc0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404d99ff 4885c0 test rax, rax
0x1404d9a02 0f84b8590000 je 0x1404df3c0
0x1404d9a08 48399648020000 cmp qword ptr [rsi + 0x248], rdx
0x1404d9a0f 0f86ab590000 jbe 0x1404df3c0
0x1404d9a15 488b8640020000 mov rax, qword ptr [rsi + 0x240]
0x1404d9a1c 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404d9a20 4885c9 test rcx, rcx
0x1404d9a23 0f8497590000 je 0x1404df3c0
0x1404d9a29 488b8610020000 mov rax, qword ptr [rsi + 0x210]
0x1404d9a30 48f7d1 not rcx
0x1404d9a33 488d0c49 lea rcx, [rcx + rcx*2]
0x1404d9a37 48c1e105 shl rcx, 5
0x1404d9a3b 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404d9a41 0f8479590000 je 0x1404df3c0
0x1404d9a47 4801c8 add rax, rcx
0x1404d9a4a 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1404d9a4e 8b96e0050000 mov edx, dword ptr [rsi + 0x5e0]
0x1404d9a54 89505c mov dword ptr [rax + 0x5c], edx
0x1404d9a57 c6859714000000 mov byte ptr [rbp + 0x1497], 0
0x1404d9a5e e8cd026400 call 0x140b19d30 ; country_core::resources::logo_animation::LogoAnimation::finish_wait
0x1404d9a63 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404d9a6a 48babaa57efadc08df98 movabs rdx, 0x98df08dcfa7ea5ba
0x1404d9a74 49b8c32b630ea617b167 movabs r8, 0x67b117a60e632bc3
0x1404d9a7e e86d3cc0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404d9a83 4885c0 test rax, rax
0x1404d9a86 4c8bb578140000 mov r14, qword ptr [rbp + 0x1478]
0x1404d9a8d 4c8b8510140000 mov r8, qword ptr [rbp + 0x1410]
0x1404d9a94 4c8da5300c0000 lea r12, [rbp + 0xc30]
0x1404d9a9b 7463 je 0x1404d9b00
0x1404d9a9d 49399648020000 cmp qword ptr [r14 + 0x248], rdx
0x1404d9aa4 765a jbe 0x1404d9b00
0x1404d9aa6 498b8640020000 mov rax, qword ptr [r14 + 0x240]
0x1404d9aad 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404d9ab1 4885c9 test rcx, rcx
0x1404d9ab4 744a je 0x1404d9b00
0x1404d9ab6 498b8610020000 mov rax, qword ptr [r14 + 0x210]
0x1404d9abd 48f7d1 not rcx
0x1404d9ac0 488d0c49 lea rcx, [rcx + rcx*2]
0x1404d9ac4 48c1e105 shl rcx, 5
0x1404d9ac8 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404d9ace 7430 je 0x1404d9b00
0x1404d9ad0 4801c8 add rax, rcx
0x1404d9ad3 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1404d9ad7 e844026400 call 0x140b19d20 ; country_core::resources::logo_animation::LogoAnimation::is_finished
0x1404d9adc 84c0 test al, al
0x1404d9ade 4c8bb578140000 mov r14, qword ptr [rbp + 0x1478]
0x1404d9ae5 4c8b8510140000 mov r8, qword ptr [rbp + 0x1410]
0x1404d9aec 0f848f080000 je 0x1404da381
0x1404d9af2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404d9b00 498b00 mov rax, qword ptr [r8]
0x1404d9b03 48b90000000000000080 movabs rcx, 0x8000000000000000
0x1404d9b0d 498908 mov qword ptr [r8], rcx
0x1404d9b10 4889c1 mov rcx, rax
0x1404d9b13 48f7d9 neg rcx
0x1404d9b16 0f8065080000 jo 0x1404da381
0x1404d9b1c 48898500060000 mov qword ptr [rbp + 0x600], rax
0x1404d9b23 41b820060000 mov r8d, 0x620
0x1404d9b29 488d8d08060000 lea rcx, [rbp + 0x608]
0x1404d9b30 488b9548130000 mov rdx, qword ptr [rbp + 0x1348]
0x1404d9b37 e8c4a04402 call 0x142923c00 ; memcpy
0x1404d9b3c 488b0565281503 mov rax, qword ptr [rip + 0x3152865]
0x1404d9b43 488b00 mov rax, qword ptr [rax]
0x1404d9b46 4883f803 cmp rax, 3
0x1404d9b4a 4c8bbd50140000 mov r15, qword ptr [rbp + 0x1450]
0x1404d9b51 0f82c7000000 jb 0x1404d9c1e
0x1404d9b57 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404d9b5e 488d0d4b025102 lea rcx, [rip + 0x251024b]
0x1404d9b65 e816421a00 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x1404d9b6a 660f1000 movupd xmm0, xmmword ptr [rax]
0x1404d9b6e 8b4010 mov eax, dword ptr [rax + 0x10]
0x1404d9b71 48c785600c000003000000 mov qword ptr [rbp + 0xc60], 3
0x1404d9b7c 488d155f015102 lea rdx, [rip + 0x251015f]
0x1404d9b83 488995680c0000 mov qword ptr [rbp + 0xc68], rdx
0x1404d9b8a 48c785700c000015000000 mov qword ptr [rbp + 0xc70], 0x15
0x1404d9b95 488d0d04025102 lea rcx, [rip + 0x2510204]
0x1404d9b9c 48898d800c0000 mov qword ptr [rbp + 0xc80], rcx
0x1404d9ba3 48c785880c000001000000 mov qword ptr [rbp + 0xc88], 1
0x1404d9bae 48c785900c000008000000 mov qword ptr [rbp + 0xc90], 8
0x1404d9bb9 48c785300c000000000000 mov qword ptr [rbp + 0xc30], 0
0x1404d9bc4 488d8d980c0000 lea rcx, [rbp + 0xc98]
0x1404d9bcb 660fefc9 pxor xmm1, xmm1
0x1404d9bcf f30f7f09 movdqu xmmword ptr [rcx], xmm1
0x1404d9bd3 488995380c0000 mov qword ptr [rbp + 0xc38], rdx
0x1404d9bda 48c785400c000015000000 mov qword ptr [rbp + 0xc40], 0x15
0x1404d9be5 48c785480c000000000000 mov qword ptr [rbp + 0xc48], 0
0x1404d9bf0 660f1185500c0000 movupd xmmword ptr [rbp + 0xc50], xmm0
0x1404d9bf8 c785780c000001000000 mov dword ptr [rbp + 0xc78], 1
0x1404d9c02 89857c0c0000 mov dword ptr [rbp + 0xc7c], eax
0x1404d9c08 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404d9c0f 488d8d77140000 lea rcx, [rbp + 0x1477]
0x1404d9c16 4c89e2 mov rdx, r12
0x1404d9c19 e812078700 call 0x140d4a330 ; log::__private_api::impl$2::log
0x1404d9c1e c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404d9c25 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404d9c2c 48ba407cd487e7567b3d movabs rdx, 0x3d7b56e787d47c40
0x1404d9c36 49b8c7bf438d0ab1b067 movabs r8, 0x67b0b10a8d43bfc7
0x1404d9c40 e8ab3ac0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404d9c45 4885c0 test rax, rax
0x1404d9c48 0f84b8550000 je 0x1404df206
0x1404d9c4e 49399648020000 cmp qword ptr [r14 + 0x248], rdx
0x1404d9c55 0f86ab550000 jbe 0x1404df206
0x1404d9c5b 498b8640020000 mov rax, qword ptr [r14 + 0x240]
0x1404d9c62 488b04d0 mov rax, qword ptr [rax + rdx*8]
0x1404d9c66 4885c0 test rax, rax
0x1404d9c69 0f8497550000 je 0x1404df206
0x1404d9c6f 498b8e10020000 mov rcx, qword ptr [r14 + 0x210]
0x1404d9c76 48f7d0 not rax
0x1404d9c79 488d0440 lea rax, [rax + rax*2]
0x1404d9c7d 48c1e005 shl rax, 5
0x1404d9c81 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
0x1404d9c87 0f8479550000 je 0x1404df206
0x1404d9c8d 4801c1 add rcx, rax
0x1404d9c90 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404d9c97 31d2 xor edx, edx
0x1404d9c99 e8725a8200 call 0x140cff710 ; bevy_ecs::storage::blob_vec::BlobVec::swap_remove_and_forget_unchecked
0x1404d9c9e 488b08 mov rcx, qword ptr [rax]
0x1404d9ca1 4889ca mov rdx, rcx
0x1404d9ca4 48f7da neg rdx
0x1404d9ca7 0f8059550000 jo 0x1404df206
0x1404d9cad 4883c008 add rax, 8
0x1404d9cb1 48898d300c0000 mov qword ptr [rbp + 0xc30], rcx
0x1404d9cb8 41b820010000 mov r8d, 0x120
0x1404d9cbe 488d8d380c0000 lea rcx, [rbp + 0xc38]
0x1404d9cc5 4889c2 mov rdx, rax
0x1404d9cc8 e8339f4402 call 0x142923c00 ; memcpy
0x1404d9ccd c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404d9cd4 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404d9cdb 4c89e2 mov rdx, r12
0x1404d9cde e89df0e4ff call 0x140328d80 ; bevy_app::sub_app::SubApp::insert_resource<country_core::game_config::GameConfig>
0x1404d9ce3 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404d9cea 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404d9cf1 48ba8abfe67d0e98392b movabs rdx, 0x2b39980e7de6bf8a
0x1404d9cfb 49b8c5560dd2bfaad952 movabs r8, 0x52d9aabfd20d56c5
0x1404d9d05 e8e639c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404d9d0a 4885c0 test rax, rax
0x1404d9d0d 0f84db540000 je 0x1404df1ee
0x1404d9d13 49399648020000 cmp qword ptr [r14 + 0x248], rdx
0x1404d9d1a 0f86ce540000 jbe 0x1404df1ee
0x1404d9d20 498b8640020000 mov rax, qword ptr [r14 + 0x240]
0x1404d9d27 488b04d0 mov rax, qword ptr [rax + rdx*8]
0x1404d9d2b 4885c0 test rax, rax
0x1404d9d2e 0f84ba540000 je 0x1404df1ee
0x1404d9d34 498b8e10020000 mov rcx, qword ptr [r14 + 0x210]
0x1404d9d3b 48f7d0 not rax
0x1404d9d3e 488d0440 lea rax, [rax + rax*2]
0x1404d9d42 48c1e005 shl rax, 5
0x1404d9d46 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
0x1404d9d4c 0f849c540000 je 0x1404df1ee
0x1404d9d52 4801c1 add rcx, rax
0x1404d9d55 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404d9d5c 31d2 xor edx, edx
0x1404d9d5e e8ad598200 call 0x140cff710 ; bevy_ecs::storage::blob_vec::BlobVec::swap_remove_and_forget_unchecked
0x1404d9d63 8b4818 mov ecx, dword ptr [rax + 0x18]
0x1404d9d66 81f900ca9a3b cmp ecx, 0x3b9aca00
0x1404d9d6c 0f847c540000 je 0x1404df1ee
0x1404d9d72 8b501c mov edx, dword ptr [rax + 0x1c]
0x1404d9d75 4c8b4010 mov r8, qword ptr [rax + 0x10]
0x1404d9d79 4c8985400c0000 mov qword ptr [rbp + 0xc40], r8
0x1404d9d80 660f1000 movupd xmm0, xmmword ptr [rax]
0x1404d9d84 660f2985300c0000 movapd xmmword ptr [rbp + 0xc30], xmm0
0x1404d9d8c 898d480c0000 mov dword ptr [rbp + 0xc48], ecx
0x1404d9d92 89954c0c0000 mov dword ptr [rbp + 0xc4c], edx
0x1404d9d98 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404d9d9f 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404d9da6 4c89e2 mov rdx, r12
0x1404d9da9 e8a2f5e4ff call 0x140329350 ; bevy_app::sub_app::SubApp::insert_resource<country_core::input::ActiveInputModeTracker>
0x1404d9dae c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404d9db5 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404d9dbc 48baa612238ca9dc4d86 movabs rdx, 0x864ddca98c2312a6
0x1404d9dc6 49b84e489a9e36e9b027 movabs r8, 0x27b0e9369e9a484e
0x1404d9dd0 e81b39c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404d9dd5 4885c0 test rax, rax
0x1404d9dd8 0f8473560000 je 0x1404df451
0x1404d9dde 49399648020000 cmp qword ptr [r14 + 0x248], rdx
0x1404d9de5 0f8666560000 jbe 0x1404df451
0x1404d9deb 498b8640020000 mov rax, qword ptr [r14 + 0x240]
0x1404d9df2 488b04d0 mov rax, qword ptr [rax + rdx*8]
0x1404d9df6 4885c0 test rax, rax
0x1404d9df9 0f8452560000 je 0x1404df451
0x1404d9dff 498b8e10020000 mov rcx, qword ptr [r14 + 0x210]
0x1404d9e06 48f7d0 not rax
0x1404d9e09 488d0440 lea rax, [rax + rax*2]
0x1404d9e0d 48c1e005 shl rax, 5
0x1404d9e11 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
0x1404d9e17 0f8434560000 je 0x1404df451
0x1404d9e1d 4801c1 add rcx, rax
0x1404d9e20 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404d9e27 31d2 xor edx, edx
0x1404d9e29 e8e2588200 call 0x140cff710 ; bevy_ecs::storage::blob_vec::BlobVec::swap_remove_and_forget_unchecked
0x1404d9e2e 8b10 mov edx, dword ptr [rax]
0x1404d9e30 448b4004 mov r8d, dword ptr [rax + 4]
0x1404d9e34 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404d9e3b 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404d9e42 e82900e5ff call 0x140329e70 ; bevy_app::sub_app::SubApp::insert_resource<country_core::resources::window::WindowSize>
0x1404d9e47 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404d9e4e 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404d9e55 48bad7986d9dcb47e0e5 movabs rdx, 0xe5e047cb9d6d98d7
0x1404d9e5f 49b85a8a780a5cd35f9c movabs r8, 0x9c5fd35c0a788a5a
0x1404d9e69 e88238c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404d9e6e 4885c0 test rax, rax
0x1404d9e71 0f84a7530000 je 0x1404df21e
0x1404d9e77 49399648020000 cmp qword ptr [r14 + 0x248], rdx
0x1404d9e7e 0f869a530000 jbe 0x1404df21e
0x1404d9e84 498b8640020000 mov rax, qword ptr [r14 + 0x240]
0x1404d9e8b 488b04d0 mov rax, qword ptr [rax + rdx*8]
0x1404d9e8f 4885c0 test rax, rax
0x1404d9e92 0f8486530000 je 0x1404df21e
0x1404d9e98 498b8e10020000 mov rcx, qword ptr [r14 + 0x210]
0x1404d9e9f 48f7d0 not rax
0x1404d9ea2 488d0440 lea rax, [rax + rax*2]
0x1404d9ea6 48c1e005 shl rax, 5
0x1404d9eaa 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
0x1404d9eb0 0f8468530000 je 0x1404df21e
0x1404d9eb6 4801c1 add rcx, rax
0x1404d9eb9 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404d9ec0 31d2 xor edx, edx
0x1404d9ec2 e849588200 call 0x140cff710 ; bevy_ecs::storage::blob_vec::BlobVec::swap_remove_and_forget_unchecked
0x1404d9ec7 488b08 mov rcx, qword ptr [rax]
0x1404d9eca 4889ca mov rdx, rcx
0x1404d9ecd 48f7da neg rdx
0x1404d9ed0 0f8048530000 jo 0x1404df21e
0x1404d9ed6 48898d300c0000 mov qword ptr [rbp + 0xc30], rcx
0x1404d9edd 660f104008 movupd xmm0, xmmword ptr [rax + 8]
0x1404d9ee2 488d85380c0000 lea rax, [rbp + 0xc38]
0x1404d9ee9 660f1100 movupd xmmword ptr [rax], xmm0
0x1404d9eed c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404d9ef4 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404d9efb 4c89e2 mov rdx, r12
0x1404d9efe e8ed0ae5ff call 0x14032a9f0 ; bevy_app::sub_app::SubApp::insert_resource<country_core::resources::window::AvailableMonitors>
0x1404d9f03 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404d9f0a 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404d9f11 48ba8b047be66126c0a4 movabs rdx, 0xa4c02661e67b048b
0x1404d9f1b 49b8ec672aaae661e7ea movabs r8, 0xeae761e6aa2a67ec
0x1404d9f25 e8c637c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404d9f2a 4885c0 test rax, rax
0x1404d9f2d 0f8403530000 je 0x1404df236
0x1404d9f33 49399648020000 cmp qword ptr [r14 + 0x248], rdx
0x1404d9f3a 0f86f6520000 jbe 0x1404df236
0x1404d9f40 498b8640020000 mov rax, qword ptr [r14 + 0x240]
0x1404d9f47 488b04d0 mov rax, qword ptr [rax + rdx*8]
0x1404d9f4b 4885c0 test rax, rax
0x1404d9f4e 0f84e2520000 je 0x1404df236
0x1404d9f54 498b8e10020000 mov rcx, qword ptr [r14 + 0x210]
0x1404d9f5b 48f7d0 not rax
0x1404d9f5e 488d0440 lea rax, [rax + rax*2]
0x1404d9f62 48c1e005 shl rax, 5
0x1404d9f66 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
0x1404d9f6c 0f84c4520000 je 0x1404df236
0x1404d9f72 4801c1 add rcx, rax
0x1404d9f75 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404d9f7c 31d2 xor edx, edx
0x1404d9f7e e88d578200 call 0x140cff710 ; bevy_ecs::storage::blob_vec::BlobVec::swap_remove_and_forget_unchecked
0x1404d9f83 0fb64820 movzx ecx, byte ptr [rax + 0x20]
0x1404d9f87 80f902 cmp cl, 2
0x1404d9f8a 0f84a6520000 je 0x1404df236
0x1404d9f90 660f1000 movupd xmm0, xmmword ptr [rax]
0x1404d9f94 f30f6f4810 movdqu xmm1, xmmword ptr [rax + 0x10]
0x1404d9f99 660f7f8d400c0000 movdqa xmmword ptr [rbp + 0xc40], xmm1
0x1404d9fa1 660f2985300c0000 movapd xmmword ptr [rbp + 0xc30], xmm0
0x1404d9fa9 888d500c0000 mov byte ptr [rbp + 0xc50], cl
0x1404d9faf 0fb64823 movzx ecx, byte ptr [rax + 0x23]
0x1404d9fb3 488d95380c0000 lea rdx, [rbp + 0xc38]
0x1404d9fba 884a1b mov byte ptr [rdx + 0x1b], cl
0x1404d9fbd 0fb74021 movzx eax, word ptr [rax + 0x21]
0x1404d9fc1 66894219 mov word ptr [rdx + 0x19], ax
0x1404d9fc5 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404d9fcc 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404d9fd3 4c89e2 mov rdx, r12
0x1404d9fd6 e865e8e4ff call 0x140328840 ; bevy_app::sub_app::SubApp::insert_resource<country_core::render::render_config::FrameRenderConfig>
0x1404d9fdb c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404d9fe2 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404d9fe9 48ba80eebc5f6a383533 movabs rdx, 0x3335386a5fbcee80
0x1404d9ff3 49b860ec651af3592a8f movabs r8, 0x8f2a59f31a65ec60
0x1404d9ffd e8ee36c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404da002 4885c0 test rax, rax
0x1404da005 0f8443520000 je 0x1404df24e
0x1404da00b 49399648020000 cmp qword ptr [r14 + 0x248], rdx
0x1404da012 0f8636520000 jbe 0x1404df24e
0x1404da018 498b8640020000 mov rax, qword ptr [r14 + 0x240]
0x1404da01f 488b04d0 mov rax, qword ptr [rax + rdx*8]
0x1404da023 4885c0 test rax, rax
0x1404da026 0f8422520000 je 0x1404df24e
0x1404da02c 498b8e10020000 mov rcx, qword ptr [r14 + 0x210]
0x1404da033 48f7d0 not rax
0x1404da036 488d0440 lea rax, [rax + rax*2]
0x1404da03a 48c1e005 shl rax, 5
0x1404da03e 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
0x1404da044 0f8404520000 je 0x1404df24e
0x1404da04a 4801c1 add rcx, rax
0x1404da04d c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404da054 31d2 xor edx, edx
0x1404da056 e8b5568200 call 0x140cff710 ; bevy_ecs::storage::blob_vec::BlobVec::swap_remove_and_forget_unchecked
0x1404da05b 488b08 mov rcx, qword ptr [rax]
0x1404da05e 4883f904 cmp rcx, 4
0x1404da062 0f84e6510000 je 0x1404df24e
0x1404da068 48898d300c0000 mov qword ptr [rbp + 0xc30], rcx
0x1404da06f 488b4828 mov rcx, qword ptr [rax + 0x28]
0x1404da073 488d95380c0000 lea rdx, [rbp + 0xc38]
0x1404da07a 48894a20 mov qword ptr [rdx + 0x20], rcx
0x1404da07e 660f104008 movupd xmm0, xmmword ptr [rax + 8]
0x1404da083 f30f6f4818 movdqu xmm1, xmmword ptr [rax + 0x18]
0x1404da088 f30f7f4a10 movdqu xmmword ptr [rdx + 0x10], xmm1
0x1404da08d 660f1102 movupd xmmword ptr [rdx], xmm0
0x1404da091 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404da098 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404da09f 4c89e2 mov rdx, r12
0x1404da0a2 e849e2e4ff call 0x1403282f0 ; bevy_app::sub_app::SubApp::insert_resource<country_core::render::adaptive_render_state::AdaptiveRenderState>
0x1404da0a7 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404da0ae 488d8d38060000 lea rcx, [rbp + 0x638]
0x1404da0b5 48ba97e75eca10f7ee76 movabs rdx, 0x76eef710ca5ee797
0x1404da0bf 49b81396f3b069514a80 movabs r8, 0x804a5169b0f39613
0x1404da0c9 e82236c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404da0ce 4885c0 test rax, rax
0x1404da0d1 0f8492530000 je 0x1404df469
0x1404da0d7 48399518080000 cmp qword ptr [rbp + 0x818], rdx
0x1404da0de 0f8685530000 jbe 0x1404df469
0x1404da0e4 488b8510080000 mov rax, qword ptr [rbp + 0x810]
0x1404da0eb 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404da0ef 4885c9 test rcx, rcx
0x1404da0f2 0f8471530000 je 0x1404df469
0x1404da0f8 488b85e0070000 mov rax, qword ptr [rbp + 0x7e0]
0x1404da0ff 48f7d1 not rcx
0x1404da102 488d0c49 lea rcx, [rcx + rcx*2]
0x1404da106 48c1e105 shl rcx, 5
0x1404da10a 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404da110 0f8453530000 je 0x1404df469
0x1404da116 4801c8 add rax, rcx
0x1404da119 488b7010 mov rsi, qword ptr [rax + 0x10]
0x1404da11d 8b8db00b0000 mov ecx, dword ptr [rbp + 0xbb0]
0x1404da123 89485c mov dword ptr [rax + 0x5c], ecx
0x1404da126 488b7e30 mov rdi, qword ptr [rsi + 0x30]
0x1404da12a 488b5e40 mov rbx, qword ptr [rsi + 0x40]
0x1404da12e 483b7e20 cmp rdi, qword ptr [rsi + 0x20]
0x1404da132 7517 jne 0x1404da14b
0x1404da134 488d4e20 lea rcx, [rsi + 0x20]
0x1404da138 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404da13f 488d15e28f5102 lea rdx, [rip + 0x2518fe2]
0x1404da146 e825870800 call 0x140562870 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::glade_border::ShowDemoBorderOverlay>,alloc::alloc::Global>
0x1404da14b 488b4628 mov rax, qword ptr [rsi + 0x28]
0x1404da14f 48891cf8 mov qword ptr [rax + rdi*8], rbx
0x1404da153 48ffc7 inc rdi
0x1404da156 48897e30 mov qword ptr [rsi + 0x30], rdi
0x1404da15a 48ff4640 inc qword ptr [rsi + 0x40]
0x1404da15e c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404da165 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404da16c 48ba3febbf685e1584a2 movabs rdx, 0xa284155e68bfeb3f
0x1404da176 49b83a4ac61b44b3a284 movabs r8, 0x84a2b3441bc64a3a
0x1404da180 e86b35c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404da185 4885c0 test rax, rax
0x1404da188 0f84d8500000 je 0x1404df266
0x1404da18e 49399648020000 cmp qword ptr [r14 + 0x248], rdx
0x1404da195 0f86cb500000 jbe 0x1404df266
0x1404da19b 498b8640020000 mov rax, qword ptr [r14 + 0x240]
0x1404da1a2 488b04d0 mov rax, qword ptr [rax + rdx*8]
0x1404da1a6 4885c0 test rax, rax
0x1404da1a9 0f84b7500000 je 0x1404df266
0x1404da1af 498b8e10020000 mov rcx, qword ptr [r14 + 0x210]
0x1404da1b6 48f7d0 not rax
0x1404da1b9 488d0440 lea rax, [rax + rax*2]
0x1404da1bd 48c1e005 shl rax, 5
0x1404da1c1 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
0x1404da1c7 0f8499500000 je 0x1404df266
0x1404da1cd 4801c1 add rcx, rax
0x1404da1d0 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404da1d7 31d2 xor edx, edx
0x1404da1d9 e832558200 call 0x140cff710 ; bevy_ecs::storage::blob_vec::BlobVec::swap_remove_and_forget_unchecked
0x1404da1de 8b4848 mov ecx, dword ptr [rax + 0x48]
0x1404da1e1 83f903 cmp ecx, 3
0x1404da1e4 0f847c500000 je 0x1404df266
0x1404da1ea 488b5040 mov rdx, qword ptr [rax + 0x40]
0x1404da1ee 488995700c0000 mov qword ptr [rbp + 0xc70], rdx
0x1404da1f5 660f1000 movupd xmm0, xmmword ptr [rax]
0x1404da1f9 f30f6f4810 movdqu xmm1, xmmword ptr [rax + 0x10]
0x1404da1fe f30f6f5020 movdqu xmm2, xmmword ptr [rax + 0x20]
0x1404da203 f30f6f5830 movdqu xmm3, xmmword ptr [rax + 0x30]
0x1404da208 660f7f9d600c0000 movdqa xmmword ptr [rbp + 0xc60], xmm3
0x1404da210 660f7f95500c0000 movdqa xmmword ptr [rbp + 0xc50], xmm2
0x1404da218 660f7f8d400c0000 movdqa xmmword ptr [rbp + 0xc40], xmm1
0x1404da220 660f2985300c0000 movapd xmmword ptr [rbp + 0xc30], xmm0
0x1404da228 898d780c0000 mov dword ptr [rbp + 0xc78], ecx
0x1404da22e 8b4854 mov ecx, dword ptr [rax + 0x54]
0x1404da231 488d95380c0000 lea rdx, [rbp + 0xc38]
0x1404da238 894a4c mov dword ptr [rdx + 0x4c], ecx
0x1404da23b 488b404c mov rax, qword ptr [rax + 0x4c]
0x1404da23f 48894244 mov qword ptr [rdx + 0x44], rax
0x1404da243 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404da24a 488d4dc0 lea rcx, [rbp - 0x40]
0x1404da24e 4c89e2 mov rdx, r12
0x1404da251 e88adf3c00 call 0x1408a81e0 ; country_core::systems::render_loop::vermeer::VermeerState::into_disable_hold
0x1404da256 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404da25d 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404da264 488d55c0 lea rdx, [rbp - 0x40]
0x1404da268 e85301e5ff call 0x14032a3c0 ; bevy_app::sub_app::SubApp::insert_resource<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >
0x1404da26d c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404da274 488d8d38060000 lea rcx, [rbp + 0x638]
0x1404da27b 48baa38bf67e77b5d7ae movabs rdx, 0xaed7b5777ef68ba3
0x1404da285 49b8722b69b099a13458 movabs r8, 0x5834a199b0692b72
0x1404da28f e85c34c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404da294 4885c0 test rax, rax
0x1404da297 0f84e4510000 je 0x1404df481
0x1404da29d 48399518080000 cmp qword ptr [rbp + 0x818], rdx
0x1404da2a4 0f86d7510000 jbe 0x1404df481
0x1404da2aa 488b8510080000 mov rax, qword ptr [rbp + 0x810]
0x1404da2b1 488b04d0 mov rax, qword ptr [rax + rdx*8]
0x1404da2b5 4885c0 test rax, rax
0x1404da2b8 0f84c3510000 je 0x1404df481
0x1404da2be 488b8de0070000 mov rcx, qword ptr [rbp + 0x7e0]
0x1404da2c5 48f7d0 not rax
0x1404da2c8 488d0440 lea rax, [rax + rax*2]
0x1404da2cc 48c1e005 shl rax, 5
0x1404da2d0 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
0x1404da2d6 0f84a5510000 je 0x1404df481
0x1404da2dc 4801c1 add rcx, rax
0x1404da2df c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404da2e6 31d2 xor edx, edx
0x1404da2e8 e823548200 call 0x140cff710 ; bevy_ecs::storage::blob_vec::BlobVec::swap_remove_and_forget_unchecked
0x1404da2ed 488b08 mov rcx, qword ptr [rax]
0x1404da2f0 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404da2f7 e8843a1a00 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x1404da2fc c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404da303 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404da30a 4889c2 mov rdx, rax
0x1404da30d e87ef5e4ff call 0x140329890 ; bevy_app::sub_app::SubApp::insert_resource<country_core::resources::gui::EguiContextResource>
0x1404da312 41b828060000 mov r8d, 0x628
0x1404da318 4c89e1 mov rcx, r12
0x1404da31b 488d9500060000 lea rdx, [rbp + 0x600]
0x1404da322 e8d9984402 call 0x142923c00 ; memcpy
0x1404da327 4c89f9 mov rcx, r15
0x1404da32a e831e81000 call 0x1405e8b60 ; core::ptr::drop_in_place<bevy_app::sub_app::SubApp>
0x1404da32f 41b828060000 mov r8d, 0x628
0x1404da335 488b8d50140000 mov rcx, qword ptr [rbp + 0x1450]
0x1404da33c 4c89e2 mov rdx, r12
0x1404da33f e8bc984402 call 0x142923c00 ; memcpy
0x1404da344 eb3b jmp 0x1404da381
0x1404da346 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404da350 4889f0 mov rax, rsi
0x1404da353 48c1e007 shl rax, 7
0x1404da357 488d0cf0 lea rcx, [rax + rsi*8]
0x1404da35b 48038db8130000 add rcx, qword ptr [rbp + 0x13b8]
0x1404da362 41b888000000 mov r8d, 0x88
0x1404da368 4889fa mov rdx, rdi
0x1404da36b e8a0984402 call 0x142923c10 ; memmove
0x1404da370 48ffc6 inc rsi
0x1404da373 4889b5c0130000 mov qword ptr [rbp + 0x13c0], rsi
0x1404da37a 4c8bb578140000 mov r14, qword ptr [rbp + 0x1478]
0x1404da381 498b4610 mov rax, qword ptr [r14 + 0x10]
0x1404da385 498b5618 mov rdx, qword ptr [r14 + 0x18]
0x1404da389 4883f802 cmp rax, 2
0x1404da38d 7421 je 0x1404da3b0
0x1404da38f 83f801 cmp eax, 1
0x1404da392 752c jne 0x1404da3c0
0x1404da394 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404da39b e820d7d8ff call 0x140267ac0 ; std::sync::mpmc::list::Channel::try_recv<enum2$<tiny_glade::event_loop::OsWindowEvent> >
0x1404da3a0 eb2a jmp 0x1404da3cc
0x1404da3a2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404da3b0 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404da3b7 e8c4500400 call 0x14051f480 ; std::sync::mpmc::zero::Channel::try_recv<enum2$<tiny_glade::event_loop::OsWindowEvent> >
0x1404da3bc eb0e jmp 0x1404da3cc
0x1404da3be 6690 nop 
0x1404da3c0 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404da3c7 e8d4850100 call 0x1404f29a0 ; std::sync::mpmc::array::Channel::try_recv<enum2$<tiny_glade::event_loop::OsWindowEvent> >
0x1404da3cc 6683bd0006000012 cmp word ptr [rbp + 0x600], 0x12
0x1404da3d4 744a je 0x1404da420
0x1404da3d6 41b888000000 mov r8d, 0x88
0x1404da3dc 488dbd300c0000 lea rdi, [rbp + 0xc30]
0x1404da3e3 4889f9 mov rcx, rdi
0x1404da3e6 488d9500060000 lea rdx, [rbp + 0x600]
0x1404da3ed e80e984402 call 0x142923c00 ; memcpy
0x1404da3f2 488bb5c0130000 mov rsi, qword ptr [rbp + 0x13c0]
0x1404da3f9 483bb5b0130000 cmp rsi, qword ptr [rbp + 0x13b0]
0x1404da400 0f854affffff jne 0x1404da350
0x1404da406 488d8db0130000 lea rcx, [rbp + 0x13b0]
0x1404da40d 488d15e4f85002 lea rdx, [rip + 0x250f8e4]
0x1404da414 e8578d0800 call 0x140563170 ; alloc::raw_vec::RawVec::grow_one<country_core::resources::walls::decorator::DecoratorOpEvent,alloc::alloc::Global>
0x1404da419 e932ffffff jmp 0x1404da350
0x1404da41e 6690 nop 
0x1404da420 80bd0206000000 cmp byte ptr [rbp + 0x602], 0
0x1404da427 0f8581490000 jne 0x1404dedae
0x1404da42d 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404da434 48ba513b7bbfc46838fb movabs rdx, 0xfb3868c4bf7b3b51
0x1404da43e 49b839f6dc90203d9ee4 movabs r8, 0xe49e3d2090dcf639
0x1404da448 e8a332c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404da44d 4885c0 test rax, rax
0x1404da450 488b8d78140000 mov rcx, qword ptr [rbp + 0x1478]
0x1404da457 7437 je 0x1404da490
0x1404da459 48399148020000 cmp qword ptr [rcx + 0x248], rdx
0x1404da460 762e jbe 0x1404da490
0x1404da462 488b8140020000 mov rax, qword ptr [rcx + 0x240]
0x1404da469 488b04d0 mov rax, qword ptr [rax + rdx*8]
0x1404da46d 4885c0 test rax, rax
0x1404da470 741e je 0x1404da490
0x1404da472 488b8910020000 mov rcx, qword ptr [rcx + 0x210]
0x1404da479 48f7d0 not rax
0x1404da47c 488d0440 lea rax, [rax + rax*2]
0x1404da480 48c1e005 shl rax, 5
0x1404da484 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
0x1404da48a 0f8594490000 jne 0x1404dee24
0x1404da490 488d4dc0 lea rcx, [rbp - 0x40]
0x1404da494 488b9550140000 mov rdx, qword ptr [rbp + 0x1450]
0x1404da49b e800a6ddff call 0x1402b4aa0 ; country_core::utils::get_last_event<enum2$<country_core::resources::mouse_capture::OsCursorCaptureCmd> >
0x1404da4a0 8b75c0 mov esi, dword ptr [rbp - 0x40]
0x1404da4a3 83fe03 cmp esi, 3
0x1404da4a6 0f84a4010000 je 0x1404da650
0x1404da4ac f20f1075c4 movsd xmm6, qword ptr [rbp - 0x3c]
0x1404da4b1 83fe02 cmp esi, 2
0x1404da4b4 0f82c3000000 jb 0x1404da57d
0x1404da4ba 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404da4c1 48ba3626d531c31a3f92 movabs rdx, 0x923f1ac331d52636
0x1404da4cb 49b8f62c93ffeac91441 movabs r8, 0x4114c9eaff932cf6
0x1404da4d5 e81632c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404da4da 4885c0 test rax, rax
0x1404da4dd 4c8b8578140000 mov r8, qword ptr [rbp + 0x1478]
0x1404da4e4 0f8415500000 je 0x1404df4ff
0x1404da4ea 49399048020000 cmp qword ptr [r8 + 0x248], rdx
0x1404da4f1 0f8608500000 jbe 0x1404df4ff
0x1404da4f7 498b8040020000 mov rax, qword ptr [r8 + 0x240]
0x1404da4fe 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404da502 4885c9 test rcx, rcx
0x1404da505 0f84f44f0000 je 0x1404df4ff
0x1404da50b 498b8010020000 mov rax, qword ptr [r8 + 0x210]
0x1404da512 48f7d1 not rcx
0x1404da515 488d0c49 lea rcx, [rcx + rcx*2]
0x1404da519 48c1e105 shl rcx, 5
0x1404da51d 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404da523 0f84d64f0000 je 0x1404df4ff
0x1404da529 4801c8 add rax, rcx
0x1404da52c 488b7810 mov rdi, qword ptr [rax + 0x10]
0x1404da530 418b88e0050000 mov ecx, dword ptr [r8 + 0x5e0]
0x1404da537 89485c mov dword ptr [rax + 0x5c], ecx
0x1404da53a 488b5f30 mov rbx, qword ptr [rdi + 0x30]
0x1404da53e 4c8b7740 mov r14, qword ptr [rdi + 0x40]
0x1404da542 483b5f20 cmp rbx, qword ptr [rdi + 0x20]
0x1404da546 7510 jne 0x1404da558
0x1404da548 488d4f20 lea rcx, [rdi + 0x20]
0x1404da54c 488d15d58b5102 lea rdx, [rip + 0x2518bd5]
0x1404da553 e858880800 call 0x140562db0 ; alloc::raw_vec::RawVec::grow_one<relative_path::RelativePathBuf,alloc::alloc::Global>
0x1404da558 488b4728 mov rax, qword ptr [rdi + 0x28]
0x1404da55c 488d0c5b lea rcx, [rbx + rbx*2]
0x1404da560 4c8934c8 mov qword ptr [rax + rcx*8], r14
0x1404da564 c744c80801000000 mov dword ptr [rax + rcx*8 + 8], 1
0x1404da56c 660f1374c80c movlpd qword ptr [rax + rcx*8 + 0xc], xmm6
0x1404da572 48ffc3 inc rbx
0x1404da575 48895f30 mov qword ptr [rdi + 0x30], rbx
0x1404da579 48ff4740 inc qword ptr [rdi + 0x40]
0x1404da57d 488b8d78140000 mov rcx, qword ptr [rbp + 0x1478]
0x1404da584 488b81f8100000 mov rax, qword ptr [rcx + 0x10f8]
0x1404da58b 48ffc0 inc rax
0x1404da58e 488981f8100000 mov qword ptr [rcx + 0x10f8], rax
0x1404da595 89b5400c0000 mov dword ptr [rbp + 0xc40], esi
0x1404da59b 660f13b5440c0000 movlpd qword ptr [rbp + 0xc44], xmm6
0x1404da5a3 488985380c0000 mov qword ptr [rbp + 0xc38], rax
0x1404da5aa 48b80100000000000080 movabs rax, 0x8000000000000001
0x1404da5b4 488985300c0000 mov qword ptr [rbp + 0xc30], rax
0x1404da5bb 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404da5c2 488b9518140000 mov rdx, qword ptr [rbp + 0x1418]
0x1404da5c9 4c8d85300c0000 lea r8, [rbp + 0xc30]
0x1404da5d0 e83bb4eeff call 0x1403c5a10 ; std::sync::mpmc::Sender::send<enum2$<tiny_glade::event_loop::GameEvent> >
0x1404da5d5 488b8500060000 mov rax, qword ptr [rbp + 0x600]
0x1404da5dc 48b90700000000000080 movabs rcx, 0x8000000000000007
0x1404da5e6 4839c8 cmp rax, rcx
0x1404da5e9 7465 je 0x1404da650
0x1404da5eb 48b90000000000000080 movabs rcx, 0x8000000000000000
0x1404da5f5 4831c8 xor rax, rcx
0x1404da5f8 4883f807 cmp rax, 7
0x1404da5fc b903000000 mov ecx, 3
0x1404da601 480f43c1 cmovae rax, rcx
0x1404da605 4883f804 cmp rax, 4
0x1404da609 7414 je 0x1404da61f
0x1404da60b 4883f803 cmp rax, 3
0x1404da60f 753f jne 0x1404da650
0x1404da611 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404da618 e8f3661100 call 0x1405f0d10 ; core::ptr::drop_in_place<egui::data::output::PlatformOutput>
0x1404da61d eb31 jmp 0x1404da650
0x1404da61f 488b9508060000 mov rdx, qword ptr [rbp + 0x608]
0x1404da626 48b80200000000000080 movabs rax, 0x8000000000000002
0x1404da630 4839c2 cmp rdx, rax
0x1404da633 7c1b jl 0x1404da650
0x1404da635 4885d2 test rdx, rdx
0x1404da638 7416 je 0x1404da650
0x1404da63a 488b8d10060000 mov rcx, qword ptr [rbp + 0x610]
0x1404da641 41b801000000 mov r8d, 1
0x1404da647 e8d4951300 call 0x140613c20 ; __rust_dealloc
0x1404da64c 0f1f4000 nop dword ptr [rax]
0x1404da650 488d8d300c0000 lea rcx, [rbp + 0xc30]
0x1404da657 488b9550140000 mov rdx, qword ptr [rbp + 0x1450]
0x1404da65e e88da1ddff call 0x1402b47f0 ; country_core::utils::drain_events<enum2$<country_core::resources::window::OsWindowCmd> >
0x1404da663 4c8bad400c0000 mov r13, qword ptr [rbp + 0xc40]
0x1404da66a 4d85ed test r13, r13
0x1404da66d 0f84ed020000 je 0x1404da960
0x1404da673 488b9d380c0000 mov rbx, qword ptr [rbp + 0xc38]
0x1404da67a 4901dd add r13, rbx
0x1404da67d eb1d jmp 0x1404da69c
0x1404da67f e89c951300 call 0x140613c20 ; __rust_dealloc
0x1404da684 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404da690 49ffcd dec r13
0x1404da693 4c39eb cmp rbx, r13
0x1404da696 0f84c4020000 je 0x1404da960
0x1404da69c 488b8d78140000 mov rcx, qword ptr [rbp + 0x1478]
0x1404da6a3 488b4120 mov rax, qword ptr [rcx + 0x20]
0x1404da6a7 488b5128 mov rdx, qword ptr [rcx + 0x28]
0x1404da6ab 4883f802 cmp rax, 2
0x1404da6af 743f je 0x1404da6f0
0x1404da6b1 83f801 cmp eax, 1
0x1404da6b4 756a jne 0x1404da720
0x1404da6b6 48b80200000000000080 movabs rax, 0x8000000000000002
0x1404da6c0 488985300c0000 mov qword ptr [rbp + 0xc30], rax
0x1404da6c7 c744242000ca9a3b mov dword ptr [rsp + 0x20], 0x3b9aca00
0x1404da6cf 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404da6d6 4c8d85300c0000 lea r8, [rbp + 0xc30]
0x1404da6dd e8eebbd8ff call 0x1402662d0 ; std::sync::mpmc::list::Channel::send<enum2$<tiny_glade::event_loop::GameEvent> >
0x1404da6e2 eb68 jmp 0x1404da74c
0x1404da6e4 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404da6f0 48b80200000000000080 movabs rax, 0x8000000000000002
0x1404da6fa 488985300c0000 mov qword ptr [rbp + 0xc30], rax
0x1404da701 c744242000ca9a3b mov dword ptr [rsp + 0x20], 0x3b9aca00
0x1404da709 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404da710 4c8d85300c0000 lea r8, [rbp + 0xc30]
0x1404da717 e8a4d50300 call 0x140517cc0 ; std::sync::mpmc::zero::Channel::send<enum2$<tiny_glade::event_loop::GameEvent> >
0x1404da71c eb2e jmp 0x1404da74c
0x1404da71e 6690 nop 
0x1404da720 48b80200000000000080 movabs rax, 0x8000000000000002
0x1404da72a 488985300c0000 mov qword ptr [rbp + 0xc30], rax
0x1404da731 c744242000ca9a3b mov dword ptr [rsp + 0x20], 0x3b9aca00
0x1404da739 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404da740 4c8d85300c0000 lea r8, [rbp + 0xc30]
0x1404da747 e8c45e0100 call 0x1404f0610 ; std::sync::mpmc::array::Channel::send<enum2$<tiny_glade::event_loop::GameEvent> >
0x1404da74c 83bd0006000002 cmp dword ptr [rbp + 0x600], 2
0x1404da753 0f8437ffffff je 0x1404da690
0x1404da759 0f108500060000 movups xmm0, xmmword ptr [rbp + 0x600]
0x1404da760 f30f6f8d10060000 movdqu xmm1, xmmword ptr [rbp + 0x610]
0x1404da768 f30f6f9520060000 movdqu xmm2, xmmword ptr [rbp + 0x620]
0x1404da770 f30f6f9d30060000 movdqu xmm3, xmmword ptr [rbp + 0x630]
0x1404da778 0f2985300c0000 movaps xmmword ptr [rbp + 0xc30], xmm0
0x1404da77f 0f108570060000 movups xmm0, xmmword ptr [rbp + 0x670]
0x1404da786 0f2985a00c0000 movaps xmmword ptr [rbp + 0xca0], xmm0
0x1404da78d 0f108560060000 movups xmm0, xmmword ptr [rbp + 0x660]
0x1404da794 0f2985900c0000 movaps xmmword ptr [rbp + 0xc90], xmm0
0x1404da79b 0f108550060000 movups xmm0, xmmword ptr [rbp + 0x650]
0x1404da7a2 0f2985800c0000 movaps xmmword ptr [rbp + 0xc80], xmm0
0x1404da7a9 660f108540060000 movupd xmm0, xmmword ptr [rbp + 0x640]
0x1404da7b1 660f2985700c0000 movapd xmmword ptr [rbp + 0xc70], xmm0
0x1404da7b9 660f7f9d600c0000 movdqa xmmword ptr [rbp + 0xc60], xmm3
0x1404da7c1 660f7f95500c0000 movdqa xmmword ptr [rbp + 0xc50], xmm2
0x1404da7c9 660f7f8d400c0000 movdqa xmmword ptr [rbp + 0xc40], xmm1
0x1404da7d1 488b85300c0000 mov rax, qword ptr [rbp + 0xc30]
0x1404da7d8 4885c0 test rax, rax
0x1404da7db 0f847d470000 je 0x1404def5e
0x1404da7e1 4c8ba508060000 mov r12, qword ptr [rbp + 0x608]
0x1404da7e8 48b80700000000000080 movabs rax, 0x8000000000000007
0x1404da7f2 4939c4 cmp r12, rax
0x1404da7f5 0f8495feffff je 0x1404da690
0x1404da7fb 4c8bb510060000 mov r14, qword ptr [rbp + 0x610]
0x1404da802 4c89e0 mov rax, r12
0x1404da805 48b90000000000000080 movabs rcx, 0x8000000000000000
0x1404da80f 4831c8 xor rax, rcx
0x1404da812 4883f807 cmp rax, 7
0x1404da816 b903000000 mov ecx, 3
0x1404da81b 480f43c1 cmovae rax, rcx
0x1404da81f 4883f804 cmp rax, 4
0x1404da823 0f84e6000000 je 0x1404da90f
0x1404da829 4883f803 cmp rax, 3
0x1404da82d 0f855dfeffff jne 0x1404da690
0x1404da833 488bb520060000 mov rsi, qword ptr [rbp + 0x620]
0x1404da83a 488bbd28060000 mov rdi, qword ptr [rbp + 0x628]
0x1404da841 4c8bbd30060000 mov r15, qword ptr [rbp + 0x630]
0x1404da848 488b9538060000 mov rdx, qword ptr [rbp + 0x638]
0x1404da84f 488d045500000000 lea rax, [rdx*2]
0x1404da857 4885c0 test rax, rax
0x1404da85a 0f85e0000000 jne 0x1404da940
0x1404da860 4d85e4 test r12, r12
0x1404da863 7411 je 0x1404da876
0x1404da865 41b801000000 mov r8d, 1
0x1404da86b 4c89f1 mov rcx, r14
0x1404da86e 4c89e2 mov rdx, r12
0x1404da871 e8aa931300 call 0x140613c20 ; __rust_dealloc
0x1404da876 4d85ff test r15, r15
0x1404da879 7476 je 0x1404da8f1
0x1404da87b 4c8d7750 lea r14, [rdi + 0x50]
0x1404da87f eb18 jmp 0x1404da899
0x1404da881 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404da890 4983ee80 sub r14, -0x80
0x1404da894 49ffcf dec r15
0x1404da897 7458 je 0x1404da8f1
0x1404da899 498b56c8 mov rdx, qword ptr [r14 - 0x38]
0x1404da89d 4889d0 mov rax, rdx
0x1404da8a0 48f7d8 neg rax
0x1404da8a3 7011 jo 0x1404da8b6
0x1404da8a5 730f jae 0x1404da8b6
0x1404da8a7 498b4ed0 mov rcx, qword ptr [r14 - 0x30]
0x1404da8ab 41b801000000 mov r8d, 1
0x1404da8b1 e86a931300 call 0x140613c20 ; __rust_dealloc
0x1404da8b6 498b56e0 mov rdx, qword ptr [r14 - 0x20]
0x1404da8ba 4889d0 mov rax, rdx
0x1404da8bd 48f7d8 neg rax
0x1404da8c0 7011 jo 0x1404da8d3
0x1404da8c2 730f jae 0x1404da8d3
0x1404da8c4 498b4ee8 mov rcx, qword ptr [r14 - 0x18]
0x1404da8c8 41b801000000 mov r8d, 1
0x1404da8ce e84d931300 call 0x140613c20 ; __rust_dealloc
0x1404da8d3 498b56f8 mov rdx, qword ptr [r14 - 8]
0x1404da8d7 4889d0 mov rax, rdx
0x1404da8da 48f7d8 neg rax
0x1404da8dd 70b1 jo 0x1404da890
0x1404da8df 73af jae 0x1404da890
0x1404da8e1 498b0e mov rcx, qword ptr [r14]
0x1404da8e4 41b801000000 mov r8d, 1
0x1404da8ea e831931300 call 0x140613c20 ; __rust_dealloc
0x1404da8ef eb9f jmp 0x1404da890
0x1404da8f1 4885f6 test rsi, rsi
0x1404da8f4 0f8496fdffff je 0x1404da690
0x1404da8fa 48c1e607 shl rsi, 7
0x1404da8fe 41b808000000 mov r8d, 8
0x1404da904 4889f9 mov rcx, rdi
0x1404da907 4889f2 mov rdx, rsi
0x1404da90a e970fdffff jmp 0x1404da67f
0x1404da90f 48b80200000000000080 movabs rax, 0x8000000000000002
0x1404da919 4939c6 cmp r14, rax
0x1404da91c 0f8c6efdffff jl 0x1404da690
0x1404da922 4d85f6 test r14, r14
0x1404da925 0f8465fdffff je 0x1404da690
0x1404da92b 488b8d18060000 mov rcx, qword ptr [rbp + 0x618]
0x1404da932 41b801000000 mov r8d, 1
0x1404da938 4c89f2 mov rdx, r14
0x1404da93b e93ffdffff jmp 0x1404da67f
0x1404da940 488b8d40060000 mov rcx, qword ptr [rbp + 0x640]
0x1404da947 41b801000000 mov r8d, 1
0x1404da94d e8ce921300 call 0x140613c20 ; __rust_dealloc
0x1404da952 4d85e4 test r12, r12
0x1404da955 0f850affffff jne 0x1404da865
0x1404da95b e916ffffff jmp 0x1404da876
0x1404da960 488b8d50140000 mov rcx, qword ptr [rbp + 0x1450]
0x1404da967 e8d4a2ddff call 0x1402b4c40 ; country_core::utils::get_last_event<enum2$<country_core::resources::mouse_capture::OsCursorVisibilityCmd> >
0x1404da96c 83f804 cmp eax, 4
0x1404da96f 0f84bb000000 je 0x1404daa30
0x1404da975 4889c1 mov rcx, rax
0x1404da978 48c1e920 shr rcx, 0x20
0x1404da97c 8985380c0000 mov dword ptr [rbp + 0xc38], eax
0x1404da982 898d3c0c0000 mov dword ptr [rbp + 0xc3c], ecx
0x1404da988 48b80000000000000080 movabs rax, 0x8000000000000000
0x1404da992 488985300c0000 mov qword ptr [rbp + 0xc30], rax
0x1404da999 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404da9a0 488b9518140000 mov rdx, qword ptr [rbp + 0x1418]
0x1404da9a7 4c8d85300c0000 lea r8, [rbp + 0xc30]
0x1404da9ae e85db0eeff call 0x1403c5a10 ; std::sync::mpmc::Sender::send<enum2$<tiny_glade::event_loop::GameEvent> >
0x1404da9b3 488b8500060000 mov rax, qword ptr [rbp + 0x600]
0x1404da9ba 48b90700000000000080 movabs rcx, 0x8000000000000007
0x1404da9c4 4839c8 cmp rax, rcx
0x1404da9c7 7467 je 0x1404daa30
0x1404da9c9 48b90000000000000080 movabs rcx, 0x8000000000000000
0x1404da9d3 4831c8 xor rax, rcx
0x1404da9d6 4883f807 cmp rax, 7
0x1404da9da b903000000 mov ecx, 3
0x1404da9df 480f43c1 cmovae rax, rcx
0x1404da9e3 4883f804 cmp rax, 4
0x1404da9e7 7414 je 0x1404da9fd
0x1404da9e9 4883f803 cmp rax, 3
0x1404da9ed 7541 jne 0x1404daa30
0x1404da9ef 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404da9f6 e815631100 call 0x1405f0d10 ; core::ptr::drop_in_place<egui::data::output::PlatformOutput>
0x1404da9fb eb33 jmp 0x1404daa30
0x1404da9fd 488b9508060000 mov rdx, qword ptr [rbp + 0x608]
0x1404daa04 48b80200000000000080 movabs rax, 0x8000000000000002
0x1404daa0e 4839c2 cmp rdx, rax
0x1404daa11 7c1d jl 0x1404daa30
0x1404daa13 4885d2 test rdx, rdx
0x1404daa16 7418 je 0x1404daa30
0x1404daa18 488b8d10060000 mov rcx, qword ptr [rbp + 0x610]
0x1404daa1f 41b801000000 mov r8d, 1
0x1404daa25 e8f6911300 call 0x140613c20 ; __rust_dealloc
0x1404daa2a 660f1f440000 nop word ptr [rax + rax]
0x1404daa30 488b8d50140000 mov rcx, qword ptr [rbp + 0x1450]
0x1404daa37 e8d49eddff call 0x1402b4910 ; country_core::utils::get_last_event<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>
0x1404daa3c 4885c0 test rax, rax
0x1404daa3f 0f84ab000000 je 0x1404daaf0
0x1404daa45 488985380c0000 mov qword ptr [rbp + 0xc38], rax
0x1404daa4c 48b80600000000000080 movabs rax, 0x8000000000000006
0x1404daa56 488985300c0000 mov qword ptr [rbp + 0xc30], rax
0x1404daa5d 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404daa64 488b9518140000 mov rdx, qword ptr [rbp + 0x1418]
0x1404daa6b 4c8d85300c0000 lea r8, [rbp + 0xc30]
0x1404daa72 e899afeeff call 0x1403c5a10 ; std::sync::mpmc::Sender::send<enum2$<tiny_glade::event_loop::GameEvent> >
0x1404daa77 488b8500060000 mov rax, qword ptr [rbp + 0x600]
0x1404daa7e 48b90700000000000080 movabs rcx, 0x8000000000000007
0x1404daa88 4839c8 cmp rax, rcx
0x1404daa8b 7463 je 0x1404daaf0
0x1404daa8d 48b90000000000000080 movabs rcx, 0x8000000000000000
0x1404daa97 4831c8 xor rax, rcx
0x1404daa9a 4883f807 cmp rax, 7
0x1404daa9e b903000000 mov ecx, 3
0x1404daaa3 480f43c1 cmovae rax, rcx
0x1404daaa7 4883f804 cmp rax, 4
0x1404daaab 7414 je 0x1404daac1
0x1404daaad 4883f803 cmp rax, 3
0x1404daab1 753d jne 0x1404daaf0
0x1404daab3 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404daaba e851621100 call 0x1405f0d10 ; core::ptr::drop_in_place<egui::data::output::PlatformOutput>
0x1404daabf eb2f jmp 0x1404daaf0
0x1404daac1 488b9508060000 mov rdx, qword ptr [rbp + 0x608]
0x1404daac8 48b80200000000000080 movabs rax, 0x8000000000000002
0x1404daad2 4839c2 cmp rdx, rax
0x1404daad5 7c19 jl 0x1404daaf0
0x1404daad7 4885d2 test rdx, rdx
0x1404daada 7414 je 0x1404daaf0
0x1404daadc 488b8d10060000 mov rcx, qword ptr [rbp + 0x610]
0x1404daae3 41b801000000 mov r8d, 1
0x1404daae9 e832911300 call 0x140613c20 ; __rust_dealloc
0x1404daaee 6690 nop 
0x1404daaf0 488b95b8130000 mov rdx, qword ptr [rbp + 0x13b8]
0x1404daaf7 488b8dc0130000 mov rcx, qword ptr [rbp + 0x13c0]
0x1404daafe 48c785c013000000000000 mov qword ptr [rbp + 0x13c0], 0
0x1404dab09 4889c8 mov rax, rcx
0x1404dab0c 48c1e007 shl rax, 7
0x1404dab10 488d04c8 lea rax, [rax + rcx*8]
0x1404dab14 4801d0 add rax, rdx
0x1404dab17 488995e0130000 mov qword ptr [rbp + 0x13e0], rdx
0x1404dab1e 488985e8130000 mov qword ptr [rbp + 0x13e8], rax
0x1404dab25 4c8d85b0130000 lea r8, [rbp + 0x13b0]
0x1404dab2c 4c8985f0130000 mov qword ptr [rbp + 0x13f0], r8
0x1404dab33 48898df8130000 mov qword ptr [rbp + 0x13f8], rcx
0x1404dab3a 48c7850014000000000000 mov qword ptr [rbp + 0x1400], 0
0x1404dab45 4c8ba578140000 mov r12, qword ptr [rbp + 0x1478]
0x1404dab4c 4c8dada2120000 lea r13, [rbp + 0x12a2]
0x1404dab53 eb44 jmp 0x1404dab99
0x1404dab55 488b4628 mov rax, qword ptr [rsi + 0x28]
0x1404dab59 4889f9 mov rcx, rdi
0x1404dab5c 48c1e105 shl rcx, 5
0x1404dab60 48891c08 mov qword ptr [rax + rcx], rbx
0x1404dab64 488b95400c0000 mov rdx, qword ptr [rbp + 0xc40]
0x1404dab6b 4889540818 mov qword ptr [rax + rcx + 0x18], rdx
0x1404dab70 660f2885300c0000 movapd xmm0, xmmword ptr [rbp + 0xc30]
0x1404dab78 660f11440808 movupd xmmword ptr [rax + rcx + 8], xmm0
0x1404dab7e 6690 nop 
0x1404dab80 48ffc7 inc rdi
0x1404dab83 48897e30 mov qword ptr [rsi + 0x30], rdi
0x1404dab87 48ff4640 inc qword ptr [rsi + 0x40]
0x1404dab8b 488b95e0130000 mov rdx, qword ptr [rbp + 0x13e0]
0x1404dab92 488b85e8130000 mov rax, qword ptr [rbp + 0x13e8]
0x1404dab99 4839c2 cmp rdx, rax
0x1404dab9c 0f84ae1c0000 je 0x1404dc850
0x1404daba2 488d8288000000 lea rax, [rdx + 0x88]
0x1404daba9 488985e0130000 mov qword ptr [rbp + 0x13e0], rax
0x1404dabb0 0fb732 movzx esi, word ptr [rdx]
0x1404dabb3 6683fe12 cmp si, 0x12
0x1404dabb7 0f84931c0000 je 0x1404dc850
0x1404dabbd 4883c202 add rdx, 2
0x1404dabc1 6689b5a0120000 mov word ptr [rbp + 0x12a0], si
0x1404dabc8 41b886000000 mov r8d, 0x86
0x1404dabce 4c89e9 mov rcx, r13
0x1404dabd1 e83a904402 call 0x142923c10 ; memmove
0x1404dabd6 83c6fc add esi, -4
0x1404dabd9 6683fe0e cmp si, 0xe
0x1404dabdd b807000000 mov eax, 7
0x1404dabe2 0f43f0 cmovae esi, eax
0x1404dabe5 0fb7c6 movzx eax, si
0x1404dabe8 488d0d61f85002 lea rcx, [rip + 0x250f861]
0x1404dabef 48630481 movsxd rax, dword ptr [rcx + rax*4]
0x1404dabf3 4801c8 add rax, rcx
0x1404dabf6 ffe0 jmp rax
0x1404dabf8 8bbda4120000 mov edi, dword ptr [rbp + 0x12a4]
0x1404dabfe 81ff204e0000 cmp edi, 0x4e20
0x1404dac04 7385 jae 0x1404dab8b
0x1404dac06 8bb5a8120000 mov esi, dword ptr [rbp + 0x12a8]
0x1404dac0c c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404dac13 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404dac1a c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dac21 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dac28 48baa612238ca9dc4d86 movabs rdx, 0x864ddca98c2312a6
0x1404dac32 49b84e489a9e36e9b027 movabs r8, 0x27b0e9369e9a484e
0x1404dac3c e8af2ac0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404dac41 4885c0 test rax, rax
0x1404dac44 0f8434460000 je 0x1404df27e
0x1404dac4a 4939942448020000 cmp qword ptr [r12 + 0x248], rdx
0x1404dac52 0f8626460000 jbe 0x1404df27e
0x1404dac58 498b842440020000 mov rax, qword ptr [r12 + 0x240]
0x1404dac60 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404dac64 4885c9 test rcx, rcx
0x1404dac67 0f8411460000 je 0x1404df27e
0x1404dac6d 498b842410020000 mov rax, qword ptr [r12 + 0x210]
0x1404dac75 48f7d1 not rcx
0x1404dac78 488d0c49 lea rcx, [rcx + rcx*2]
0x1404dac7c 48c1e105 shl rcx, 5
0x1404dac80 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dac86 0f84f2450000 je 0x1404df27e
0x1404dac8c 4801c8 add rax, rcx
0x1404dac8f 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1404dac93 418b9424e0050000 mov edx, dword ptr [r12 + 0x5e0]
0x1404dac9b 89505c mov dword ptr [rax + 0x5c], edx
0x1404dac9e c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404daca5 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404dacac c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dacb3 89fa mov edx, edi
0x1404dacb5 4189f0 mov r8d, esi
0x1404dacb8 e843ef4600 call 0x140949c00 ; country_core::resources::window::WindowSize::set
0x1404dacbd c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404dacc4 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404daccb c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dacd2 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dacd9 48ba8b047be66126c0a4 movabs rdx, 0xa4c02661e67b048b
0x1404dace3 49b8ec672aaae661e7ea movabs r8, 0xeae761e6aa2a67ec
0x1404daced e8fe29c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404dacf2 4885c0 test rax, rax
0x1404dacf5 0f84a9450000 je 0x1404df2a4
0x1404dacfb 4939942448020000 cmp qword ptr [r12 + 0x248], rdx
0x1404dad03 0f869b450000 jbe 0x1404df2a4
0x1404dad09 498b842440020000 mov rax, qword ptr [r12 + 0x240]
0x1404dad11 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404dad15 4885c9 test rcx, rcx
0x1404dad18 0f8486450000 je 0x1404df2a4
0x1404dad1e 498b842410020000 mov rax, qword ptr [r12 + 0x210]
0x1404dad26 48f7d1 not rcx
0x1404dad29 488d0c49 lea rcx, [rcx + rcx*2]
0x1404dad2d 48c1e105 shl rcx, 5
0x1404dad31 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dad37 0f8467450000 je 0x1404df2a4
0x1404dad3d 4801c8 add rax, rcx
0x1404dad40 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1404dad44 418b9424e0050000 mov edx, dword ptr [r12 + 0x5e0]
0x1404dad4c 89505c mov dword ptr [rax + 0x5c], edx
0x1404dad4f 8939 mov dword ptr [rcx], edi
0x1404dad51 897104 mov dword ptr [rcx + 4], esi
0x1404dad54 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404dad5b c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404dad62 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dad69 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dad70 48ba407cd487e7567b3d movabs rdx, 0x3d7b56e787d47c40
0x1404dad7a 49b8c7bf438d0ab1b067 movabs r8, 0x67b0b10a8d43bfc7
0x1404dad84 e86729c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404dad89 4885c0 test rax, rax
0x1404dad8c 0f845e450000 je 0x1404df2f0
0x1404dad92 4939942448020000 cmp qword ptr [r12 + 0x248], rdx
0x1404dad9a 0f8650450000 jbe 0x1404df2f0
0x1404dada0 498b842440020000 mov rax, qword ptr [r12 + 0x240]
0x1404dada8 488b04d0 mov rax, qword ptr [rax + rdx*8]
0x1404dadac 4885c0 test rax, rax
0x1404dadaf 0f843b450000 je 0x1404df2f0
0x1404dadb5 498b9c2410020000 mov rbx, qword ptr [r12 + 0x210]
0x1404dadbd 48f7d0 not rax
0x1404dadc0 488d0440 lea rax, [rax + rax*2]
0x1404dadc4 48c1e005 shl rax, 5
0x1404dadc8 48837c032000 cmp qword ptr [rbx + rax + 0x20], 0
0x1404dadce 0f841c450000 je 0x1404df2f0
0x1404dadd4 4801c3 add rbx, rax
0x1404dadd7 4c8b7310 mov r14, qword ptr [rbx + 0x10]
0x1404daddb f20f2ac7 cvtsi2sd xmm0, edi
0x1404daddf 458bbc24e0050000 mov r15d, dword ptr [r12 + 0x5e0]
0x1404dade7 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404dadee c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404dadf5 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dadfc e8efc91b00 call 0x1406977f0 ; winit::dpi::impl$2::from_f64
0x1404dae01 89c7 mov edi, eax
0x1404dae03 89f0 mov eax, esi
0x1404dae05 0f57c0 xorps xmm0, xmm0
0x1404dae08 f2480f2ac0 cvtsi2sd xmm0, rax
0x1404dae0d c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404dae14 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404dae1b c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dae22 e8c9c91b00 call 0x1406977f0 ; winit::dpi::impl$2::from_f64
0x1404dae27 48c1e020 shl rax, 0x20
0x1404dae2b 89f9 mov ecx, edi
0x1404dae2d 4809c1 or rcx, rax
0x1404dae30 44897b5c mov dword ptr [rbx + 0x5c], r15d
0x1404dae34 49894e60 mov qword ptr [r14 + 0x60], rcx
0x1404dae38 40b601 mov sil, 1
0x1404dae3b 4983bc24b800000000 cmp qword ptr [r12 + 0xb8], 0
0x1404dae44 0f853d0f0000 jne 0x1404dbd87
0x1404dae4a e9df0f0000 jmp 0x1404dbe2e
0x1404dae4f 488b85a8120000 mov rax, qword ptr [rbp + 0x12a8]
0x1404dae56 498b8c2400110000 mov rcx, qword ptr [r12 + 0x1100]
0x1404dae5e 4839c8 cmp rax, rcx
0x1404dae61 480f47c8 cmova rcx, rax
0x1404dae65 49898c2400110000 mov qword ptr [r12 + 0x1100], rcx
0x1404dae6d e919fdffff jmp 0x1404dab8b
0x1404dae72 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404dae79 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404dae80 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dae87 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dae8e 48ba407cd487e7567b3d movabs rdx, 0x3d7b56e787d47c40
0x1404dae98 49b8c7bf438d0ab1b067 movabs r8, 0x67b0b10a8d43bfc7
0x1404daea2 e84928c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404daea7 4885c0 test rax, rax
0x1404daeaa 0f8429420000 je 0x1404df0d9
0x1404daeb0 4939942448020000 cmp qword ptr [r12 + 0x248], rdx
0x1404daeb8 0f861b420000 jbe 0x1404df0d9
0x1404daebe 498b842440020000 mov rax, qword ptr [r12 + 0x240]
0x1404daec6 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404daeca 4885c9 test rcx, rcx
0x1404daecd 0f8406420000 je 0x1404df0d9
0x1404daed3 498b842410020000 mov rax, qword ptr [r12 + 0x210]
0x1404daedb 48f7d1 not rcx
0x1404daede 488d0c49 lea rcx, [rcx + rcx*2]
0x1404daee2 48c1e105 shl rcx, 5
0x1404daee6 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404daeec 0f84e7410000 je 0x1404df0d9
0x1404daef2 4801c8 add rax, rcx
0x1404daef5 488b5810 mov rbx, qword ptr [rax + 0x10]
0x1404daef9 418b8c24e0050000 mov ecx, dword ptr [r12 + 0x5e0]
0x1404daf01 89485c mov dword ptr [rax + 0x5c], ecx
0x1404daf04 498b8c24e8100000 mov rcx, qword ptr [r12 + 0x10e8]
0x1404daf0c 4883c110 add rcx, 0x10
0x1404daf10 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404daf17 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404daf1e c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404daf25 e856271c00 call 0x14069d680 ; winit::platform_impl::platform::window::Window::window_state_lock
0x1404daf2a 4989c7 mov r15, rax
0x1404daf2d 4c8b7028 mov r14, qword ptr [rax + 0x28]
0x1404daf31 4983fe01 cmp r14, 1
0x1404daf35 754d jne 0x1404daf84
0x1404daf37 498b7f30 mov rdi, qword ptr [r15 + 0x30]
0x1404daf3b 4885ff test rdi, rdi
0x1404daf3e 0f8452110000 je 0x1404dc096
0x1404daf44 889598140000 mov byte ptr [rbp + 0x1498], dl
0x1404daf4a 0fb605b89c1503 movzx eax, byte ptr [rip + 0x3159cb8]
0x1404daf51 b9dc000000 mov ecx, 0xdc
0x1404daf56 ba04000000 mov edx, 4
0x1404daf5b e8b08c1300 call 0x140613c10 ; __rust_alloc
0x1404daf60 4885c0 test rax, rax
0x1404daf63 0f84eb450000 je 0x1404df554
0x1404daf69 4889c6 mov rsi, rax
0x1404daf6c 41b8dc000000 mov r8d, 0xdc
0x1404daf72 4889c1 mov rcx, rax
0x1404daf75 4889fa mov rdx, rdi
0x1404daf78 e8838c4402 call 0x142923c00 ; memcpy
0x1404daf7d 0fb69598140000 movzx edx, byte ptr [rbp + 0x1498]
0x1404daf84 f6c201 test dl, 1
0x1404daf87 7516 jne 0x1404daf9f
0x1404daf89 488b0590141503 mov rax, qword ptr [rip + 0x3151490]
0x1404daf90 488b00 mov rax, qword ptr [rax]
0x1404daf93 48d1e0 shl rax, 1
0x1404daf96 4885c0 test rax, rax
0x1404daf99 0f850c180000 jne 0x1404dc7ab
0x1404daf9f 31c0 xor eax, eax
0x1404dafa1 418607 xchg byte ptr [r15], al
0x1404dafa4 3c02 cmp al, 2
0x1404dafa6 0f84ff160000 je 0x1404dc6ab
0x1404dafac 4584f6 test r14b, r14b
0x1404dafaf 0f84c40c0000 je 0x1404dbc79
0x1404dafb5 c6437c00 mov byte ptr [rbx + 0x7c], 0
0x1404dafb9 4885f6 test rsi, rsi
0x1404dafbc 0f84bb0c0000 je 0x1404dbc7d
0x1404dafc2 badc000000 mov edx, 0xdc
0x1404dafc7 41b804000000 mov r8d, 4
0x1404dafcd 4889f1 mov rcx, rsi
0x1404dafd0 e84b8c1300 call 0x140613c20 ; __rust_dealloc
0x1404dafd5 e9a30c0000 jmp 0x1404dbc7d
0x1404dafda 498b4516 mov rax, qword ptr [r13 + 0x16]
0x1404dafde 488985400c0000 mov qword ptr [rbp + 0xc40], rax
0x1404dafe5 66410f104506 movupd xmm0, xmmword ptr [r13 + 6]
0x1404dafeb 660f2985300c0000 movapd xmmword ptr [rbp + 0xc30], xmm0
0x1404daff3 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404daffa c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404db001 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404db008 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404db00f 48ba3ca3c89470608822 movabs rdx, 0x2288607094c8a33c
0x1404db019 49b8bcb2595a8f16b3b2 movabs r8, 0xb2b3168f5a59b2bc
0x1404db023 e8c826c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404db028 4885c0 test rax, rax
0x1404db02b 0f8444400000 je 0x1404df075
0x1404db031 4939942448020000 cmp qword ptr [r12 + 0x248], rdx
0x1404db039 0f8636400000 jbe 0x1404df075
0x1404db03f 498b842440020000 mov rax, qword ptr [r12 + 0x240]
0x1404db047 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404db04b 4885c9 test rcx, rcx
0x1404db04e 0f8421400000 je 0x1404df075
0x1404db054 498b842410020000 mov rax, qword ptr [r12 + 0x210]
0x1404db05c 48f7d1 not rcx
0x1404db05f 488d0c49 lea rcx, [rcx + rcx*2]
0x1404db063 48c1e105 shl rcx, 5
0x1404db067 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404db06d 0f8402400000 je 0x1404df075
0x1404db073 4801c8 add rax, rcx
0x1404db076 488b7010 mov rsi, qword ptr [rax + 0x10]
0x1404db07a 418b8c24e0050000 mov ecx, dword ptr [r12 + 0x5e0]
0x1404db082 89485c mov dword ptr [rax + 0x5c], ecx
0x1404db085 488b7e30 mov rdi, qword ptr [rsi + 0x30]
0x1404db089 488b5e40 mov rbx, qword ptr [rsi + 0x40]
0x1404db08d 483b7e20 cmp rdi, qword ptr [rsi + 0x20]
0x1404db091 0f85befaffff jne 0x1404dab55
0x1404db097 488d4e20 lea rcx, [rsi + 0x20]
0x1404db09b c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404db0a2 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404db0a9 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404db0b0 488d1571805102 lea rdx, [rip + 0x2518071]
0x1404db0b7 e8747b0800 call 0x140562c30 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::water::WaterEraseEvent>,alloc::alloc::Global>
0x1404db0bc e994faffff jmp 0x1404dab55
0x1404db0c1 f20f10b5a4120000 movsd xmm6, qword ptr [rbp + 0x12a4]
0x1404db0c9 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404db0d0 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404db0d7 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404db0de 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404db0e5 48ba3626d531c31a3f92 movabs rdx, 0x923f1ac331d52636
0x1404db0ef 49b8f62c93ffeac91441 movabs r8, 0x4114c9eaff932cf6
0x1404db0f9 e8f225c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404db0fe 4885c0 test rax, rax
0x1404db101 0f84be3e0000 je 0x1404defc5
0x1404db107 4939942448020000 cmp qword ptr [r12 + 0x248], rdx
0x1404db10f 0f86b03e0000 jbe 0x1404defc5
0x1404db115 498b842440020000 mov rax, qword ptr [r12 + 0x240]
0x1404db11d 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404db121 4885c9 test rcx, rcx
0x1404db124 0f849b3e0000 je 0x1404defc5
0x1404db12a 498b842410020000 mov rax, qword ptr [r12 + 0x210]
0x1404db132 48f7d1 not rcx
0x1404db135 488d0c49 lea rcx, [rcx + rcx*2]
0x1404db139 48c1e105 shl rcx, 5
0x1404db13d 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404db143 0f847c3e0000 je 0x1404defc5
0x1404db149 4801c8 add rax, rcx
0x1404db14c 488b7010 mov rsi, qword ptr [rax + 0x10]
0x1404db150 418b8c24e0050000 mov ecx, dword ptr [r12 + 0x5e0]
0x1404db158 89485c mov dword ptr [rax + 0x5c], ecx
0x1404db15b 488b7e30 mov rdi, qword ptr [rsi + 0x30]
0x1404db15f 488b5e40 mov rbx, qword ptr [rsi + 0x40]
0x1404db163 483b7e20 cmp rdi, qword ptr [rsi + 0x20]
0x1404db167 7525 jne 0x1404db18e
0x1404db169 488d4e20 lea rcx, [rsi + 0x20]
0x1404db16d c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404db174 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404db17b c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404db182 488d159f7f5102 lea rdx, [rip + 0x2517f9f]
0x1404db189 e8227c0800 call 0x140562db0 ; alloc::raw_vec::RawVec::grow_one<relative_path::RelativePathBuf,alloc::alloc::Global>
0x1404db18e 488b4628 mov rax, qword ptr [rsi + 0x28]
0x1404db192 488d0c7f lea rcx, [rdi + rdi*2]
0x1404db196 48891cc8 mov qword ptr [rax + rcx*8], rbx
0x1404db19a c744c80802000000 mov dword ptr [rax + rcx*8 + 8], 2
0x1404db1a2 660f1374c80c movlpd qword ptr [rax + rcx*8 + 0xc], xmm6
0x1404db1a8 e9d3f9ffff jmp 0x1404dab80
0x1404db1ad c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404db1b4 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404db1bb c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404db1c2 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404db1c9 48bacfde921b5889d12e movabs rdx, 0x2ed189581b92decf
0x1404db1d3 49b8385f069693d9d315 movabs r8, 0x15d3d99396065f38
0x1404db1dd e80e25c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404db1e2 4885c0 test rax, rax
0x1404db1e5 0f84003e0000 je 0x1404defeb
0x1404db1eb 4939942448020000 cmp qword ptr [r12 + 0x248], rdx
0x1404db1f3 0f86f23d0000 jbe 0x1404defeb
0x1404db1f9 498b842440020000 mov rax, qword ptr [r12 + 0x240]
0x1404db201 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404db205 4885c9 test rcx, rcx
0x1404db208 0f84dd3d0000 je 0x1404defeb
0x1404db20e 498b842410020000 mov rax, qword ptr [r12 + 0x210]
0x1404db216 48f7d1 not rcx
0x1404db219 488d0c49 lea rcx, [rcx + rcx*2]
0x1404db21d 48c1e105 shl rcx, 5
0x1404db221 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404db227 0f84be3d0000 je 0x1404defeb
0x1404db22d 4801c8 add rax, rcx
0x1404db230 488b4010 mov rax, qword ptr [rax + 0x10]
0x1404db234 488b10 mov rdx, qword ptr [rax]
0x1404db237 4883c210 add rdx, 0x10
0x1404db23b c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404db242 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404db249 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404db250 488b8d20140000 mov rcx, qword ptr [rbp + 0x1420]
0x1404db257 e8f4367300 call 0x140c0e950 ; rhapsody::raw::swapchain::Swapchain::mark_stale
0x1404db25c e92af9ffff jmp 0x1404dab8b
0x1404db261 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404db268 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404db26f c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404db276 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404db27d 48baa1bf7693bb3e382e movabs rdx, 0x2e383ebb9376bfa1
0x1404db287 49b83c837cc1e247b896 movabs r8, 0x96b847e2c17c833c
0x1404db291 e85a24c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404db296 4885c0 test rax, rax
0x1404db299 0f84b03d0000 je 0x1404df04f
0x1404db29f 4939942448020000 cmp qword ptr [r12 + 0x248], rdx
0x1404db2a7 0f86a23d0000 jbe 0x1404df04f
0x1404db2ad 498b842440020000 mov rax, qword ptr [r12 + 0x240]
0x1404db2b5 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404db2b9 4885c9 test rcx, rcx
0x1404db2bc 0f848d3d0000 je 0x1404df04f
0x1404db2c2 498b842410020000 mov rax, qword ptr [r12 + 0x210]
0x1404db2ca 48f7d1 not rcx
0x1404db2cd 488d0c49 lea rcx, [rcx + rcx*2]
0x1404db2d1 48c1e105 shl rcx, 5
0x1404db2d5 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404db2db 0f846e3d0000 je 0x1404df04f
0x1404db2e1 4801c8 add rax, rcx
0x1404db2e4 488b7010 mov rsi, qword ptr [rax + 0x10]
0x1404db2e8 418b8c24e0050000 mov ecx, dword ptr [r12 + 0x5e0]
0x1404db2f0 89485c mov dword ptr [rax + 0x5c], ecx
0x1404db2f3 488b7e30 mov rdi, qword ptr [rsi + 0x30]
0x1404db2f7 488b5e40 mov rbx, qword ptr [rsi + 0x40]
0x1404db2fb 483b7e20 cmp rdi, qword ptr [rsi + 0x20]
0x1404db2ff 7525 jne 0x1404db326
0x1404db301 488d4e20 lea rcx, [rsi + 0x20]
0x1404db305 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404db30c c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404db313 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404db31a 488d15077e5102 lea rdx, [rip + 0x2517e07]
0x1404db321 e84a750800 call 0x140562870 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::glade_border::ShowDemoBorderOverlay>,alloc::alloc::Global>
0x1404db326 488b4628 mov rax, qword ptr [rsi + 0x28]
0x1404db32a 48891cf8 mov qword ptr [rax + rdi*8], rbx
0x1404db32e e94df8ffff jmp 0x1404dab80
0x1404db333 0f108510130000 movups xmm0, xmmword ptr [rbp + 0x1310]
0x1404db33a 0f2985a00c0000 movaps xmmword ptr [rbp + 0xca0], xmm0
0x1404db341 0f108500130000 movups xmm0, xmmword ptr [rbp + 0x1300]
0x1404db348 0f2985900c0000 movaps xmmword ptr [rbp + 0xc90], xmm0
0x1404db34f 0f1085f0120000 movups xmm0, xmmword ptr [rbp + 0x12f0]
0x1404db356 0f2985800c0000 movaps xmmword ptr [rbp + 0xc80], xmm0
0x1404db35d 0f1085e0120000 movups xmm0, xmmword ptr [rbp + 0x12e0]
0x1404db364 0f2985700c0000 movaps xmmword ptr [rbp + 0xc70], xmm0
0x1404db36b 660f1085a0120000 movupd xmm0, xmmword ptr [rbp + 0x12a0]
0x1404db373 f30f6f8db0120000 movdqu xmm1, xmmword ptr [rbp + 0x12b0]
0x1404db37b f30f6f95c0120000 movdqu xmm2, xmmword ptr [rbp + 0x12c0]
0x1404db383 f30f6f9dd0120000 movdqu xmm3, xmmword ptr [rbp + 0x12d0]
0x1404db38b 660f7f9d600c0000 movdqa xmmword ptr [rbp + 0xc60], xmm3
0x1404db393 660f7f95500c0000 movdqa xmmword ptr [rbp + 0xc50], xmm2
0x1404db39b 660f7f8d400c0000 movdqa xmmword ptr [rbp + 0xc40], xmm1
0x1404db3a3 660f2985300c0000 movapd xmmword ptr [rbp + 0xc30], xmm0
0x1404db3ab 8bb520130000 mov esi, dword ptr [rbp + 0x1320]
0x1404db3b1 40f6c620 test sil, 0x20
0x1404db3b5 7430 je 0x1404db3e7
0x1404db3b7 c685a214000001 mov byte ptr [rbp + 0x14a2], 1
0x1404db3be 488d8d300c0000 lea rcx, [rbp + 0xc30]
0x1404db3c5 e826b21c00 call 0x1406a65f0 ; winit::keyboard::Key::to_text
0x1404db3ca 4885c0 test rax, rax
0x1404db3cd 0f95c1 setne cl
0x1404db3d0 4883fa01 cmp rdx, 1
0x1404db3d4 0f94c2 sete dl
0x1404db3d7 20ca and dl, cl
0x1404db3d9 80fa01 cmp dl, 1
0x1404db3dc 7509 jne 0x1404db3e7
0x1404db3de 803876 cmp byte ptr [rax], 0x76
0x1404db3e1 0f84610b0000 je 0x1404dbf48
0x1404db3e7 89f3 mov ebx, esi
0x1404db3e9 83e304 and ebx, 4
0x1404db3ec 0f853a0b0000 jne 0x1404dbf2c
0x1404db3f2 c685a214000001 mov byte ptr [rbp + 0x14a2], 1
0x1404db3f9 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404db400 488d95300c0000 lea rdx, [rbp + 0xc30]
0x1404db407 e84404c2ff call 0x1400fb850 ; tiny_glade::winit_bevy_glue::winit_to_bevy_keycode
0x1404db40c 440fb685900c0000 movzx r8d, byte ptr [rbp + 0xc90]
0x1404db414 0fb68da80c0000 movzx ecx, byte ptr [rbp + 0xca8]
0x1404db41b 0fb685a90c0000 movzx eax, byte ptr [rbp + 0xca9]
0x1404db422 41b21b mov r10b, 0x1b
0x1404db425 4180f81b cmp r8b, 0x1b
0x1404db429 0f84bd0c0000 je 0x1404dc0ec
0x1404db42f 418d50e8 lea edx, [r8 - 0x18]
0x1404db433 80fa03 cmp dl, 3
0x1404db436 0fb6d2 movzx edx, dl
0x1404db439 41b901000000 mov r9d, 1
0x1404db43f 410f43d1 cmovae edx, r9d
0x1404db443 84d2 test dl, dl
0x1404db445 0f845b0c0000 je 0x1404dc0a6
0x1404db44b 0fb6d2 movzx edx, dl
0x1404db44e 83fa01 cmp edx, 1
0x1404db451 0f85650c0000 jne 0x1404dc0bc
0x1404db457 4c8d8d380c0000 lea r9, [rbp + 0xc38]
0x1404db45e 418b5159 mov edx, dword ptr [r9 + 0x59]
0x1404db462 458b495c mov r9d, dword ptr [r9 + 0x5c]
0x1404db466 44898d33140000 mov dword ptr [rbp + 0x1433], r9d
0x1404db46d 899530140000 mov dword ptr [rbp + 0x1430], edx
0x1404db473 e9470c0000 jmp 0x1404dc0bf
0x1404db478 498b8424e8100000 mov rax, qword ptr [r12 + 0x10e8]
0x1404db480 488b4818 mov rcx, qword ptr [rax + 0x18]
0x1404db484 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404db48b c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404db492 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404db499 e862c61b00 call 0x140697b00 ; winit::platform_impl::platform::monitor::current_monitor
0x1404db49e 48c7850006000001000000 mov qword ptr [rbp + 0x600], 1
0x1404db4a9 48898508060000 mov qword ptr [rbp + 0x608], rax
0x1404db4b0 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404db4b7 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404db4be c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404db4c5 4c89e1 mov rcx, r12
0x1404db4c8 e8a36f0000 call 0x1404e2470 ; tiny_glade::game_loop::GameLoop::extract_available_monitors_info
0x1404db4cd c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404db4d4 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404db4db c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404db4e2 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404db4e9 48ba407cd487e7567b3d movabs rdx, 0x3d7b56e787d47c40
0x1404db4f3 49b8c7bf438d0ab1b067 movabs r8, 0x67b0b10a8d43bfc7
0x1404db4fd e8ee21c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404db502 4885c0 test rax, rax
0x1404db505 0f84063b0000 je 0x1404df011
0x1404db50b 4939942448020000 cmp qword ptr [r12 + 0x248], rdx
0x1404db513 0f86f83a0000 jbe 0x1404df011
0x1404db519 498b842440020000 mov rax, qword ptr [r12 + 0x240]
0x1404db521 488b04d0 mov rax, qword ptr [rax + rdx*8]
0x1404db525 4885c0 test rax, rax
0x1404db528 0f84e33a0000 je 0x1404df011
0x1404db52e 498bbc2410020000 mov rdi, qword ptr [r12 + 0x210]
0x1404db536 48f7d0 not rax
0x1404db539 488d0440 lea rax, [rax + rax*2]
0x1404db53d 48c1e005 shl rax, 5
0x1404db541 48837c072000 cmp qword ptr [rdi + rax + 0x20], 0
0x1404db547 0f84c43a0000 je 0x1404df011
0x1404db54d 4801c7 add rdi, rax
0x1404db550 488b7710 mov rsi, qword ptr [rdi + 0x10]
0x1404db554 418b9c24e0050000 mov ebx, dword ptr [r12 + 0x5e0]
0x1404db55c f6850006000001 test byte ptr [rbp + 0x600], 1
0x1404db563 0f84a4070000 je 0x1404dbd0d
0x1404db569 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404db570 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404db577 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404db57e 488d8d300c0000 lea rcx, [rbp + 0xc30]
0x1404db585 488d9508060000 lea rdx, [rbp + 0x608]
0x1404db58c e8bf9ec9ff call 0x140175450 ; tiny_glade::setup::impl$0::from_winit
0x1404db591 e988070000 jmp 0x1404dbd1e
0x1404db596 488b85a8120000 mov rax, qword ptr [rbp + 0x12a8]
0x1404db59d 48898568140000 mov qword ptr [rbp + 0x1468], rax
0x1404db5a4 488b85b0120000 mov rax, qword ptr [rbp + 0x12b0]
0x1404db5ab 48898560140000 mov qword ptr [rbp + 0x1460], rax
0x1404db5b2 66410f104516 movupd xmm0, xmmword ptr [r13 + 0x16]
0x1404db5b8 f3410f6f4d26 movdqu xmm1, xmmword ptr [r13 + 0x26]
0x1404db5be f3410f6f5536 movdqu xmm2, xmmword ptr [r13 + 0x36]
0x1404db5c4 660f298560120000 movapd xmmword ptr [rbp + 0x1260], xmm0
0x1404db5cc 660f7f8d70120000 movdqa xmmword ptr [rbp + 0x1270], xmm1
0x1404db5d4 660f7f9580120000 movdqa xmmword ptr [rbp + 0x1280], xmm2
0x1404db5dc 498b4546 mov rax, qword ptr [r13 + 0x46]
0x1404db5e0 48898590120000 mov qword ptr [rbp + 0x1290], rax
0x1404db5e7 c6858714000001 mov byte ptr [rbp + 0x1487], 1
0x1404db5ee 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404db5f5 48ba407cd487e7567b3d movabs rdx, 0x3d7b56e787d47c40
0x1404db5ff 49b8c7bf438d0ab1b067 movabs r8, 0x67b0b10a8d43bfc7
0x1404db609 e8e220c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404db60e 4885c0 test rax, rax
0x1404db611 0f84aa3a0000 je 0x1404df0c1
0x1404db617 4939942448020000 cmp qword ptr [r12 + 0x248], rdx
0x1404db61f 0f869c3a0000 jbe 0x1404df0c1
0x1404db625 498b842440020000 mov rax, qword ptr [r12 + 0x240]
0x1404db62d 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404db631 4885c9 test rcx, rcx
0x1404db634 0f84873a0000 je 0x1404df0c1
0x1404db63a 498b842410020000 mov rax, qword ptr [r12 + 0x210]
0x1404db642 48f7d1 not rcx
0x1404db645 488d0c49 lea rcx, [rcx + rcx*2]
0x1404db649 48c1e105 shl rcx, 5
0x1404db64d 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404db653 0f84683a0000 je 0x1404df0c1
0x1404db659 4801c8 add rax, rcx
0x1404db65c 488b5010 mov rdx, qword ptr [rax + 0x10]
0x1404db660 418b8c24e0050000 mov ecx, dword ptr [r12 + 0x5e0]
0x1404db668 89485c mov dword ptr [rax + 0x5c], ecx
0x1404db66b 488b4238 mov rax, qword ptr [rdx + 0x38]
0x1404db66f 48b90200000000000080 movabs rcx, 0x8000000000000002
0x1404db679 4839c8 cmp rax, rcx
0x1404db67c 7c1d jl 0x1404db69b
0x1404db67e 4885c0 test rax, rax
0x1404db681 7418 je 0x1404db69b
0x1404db683 488b4a40 mov rcx, qword ptr [rdx + 0x40]
0x1404db687 41b801000000 mov r8d, 1
0x1404db68d 4889d6 mov rsi, rdx
0x1404db690 4889c2 mov rdx, rax
0x1404db693 e888851300 call 0x140613c20 ; __rust_dealloc
0x1404db698 4889f2 mov rdx, rsi
0x1404db69b 488b8568140000 mov rax, qword ptr [rbp + 0x1468]
0x1404db6a2 48894238 mov qword ptr [rdx + 0x38], rax
0x1404db6a6 488b8560140000 mov rax, qword ptr [rbp + 0x1460]
0x1404db6ad 48894240 mov qword ptr [rdx + 0x40], rax
0x1404db6b1 488b8590120000 mov rax, qword ptr [rbp + 0x1290]
0x1404db6b8 48894278 mov qword ptr [rdx + 0x78], rax
0x1404db6bc 660f288560120000 movapd xmm0, xmmword ptr [rbp + 0x1260]
0x1404db6c4 660f6f8d70120000 movdqa xmm1, xmmword ptr [rbp + 0x1270]
0x1404db6cc 660f6f9580120000 movdqa xmm2, xmmword ptr [rbp + 0x1280]
0x1404db6d4 f30f7f5268 movdqu xmmword ptr [rdx + 0x68], xmm2
0x1404db6d9 f30f7f4a58 movdqu xmmword ptr [rdx + 0x58], xmm1
0x1404db6de 660f114248 movupd xmmword ptr [rdx + 0x48], xmm0
0x1404db6e3 c6858714000000 mov byte ptr [rbp + 0x1487], 0
0x1404db6ea 488d8d300c0000 lea rcx, [rbp + 0xc30]
0x1404db6f1 e85ac6ffff call 0x1404d7d50 ; country_core::render::render_config::impl$38::clone
0x1404db6f6 41b8c8000000 mov r8d, 0xc8
0x1404db6fc 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404db703 488d95300c0000 lea rdx, [rbp + 0xc30]
0x1404db70a e8f1844402 call 0x142923c00 ; memcpy
0x1404db70f c6857614000001 mov byte ptr [rbp + 0x1476], 1
0x1404db716 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404db71d 48ba036149a6340c83fe movabs rdx, 0xfe830c34a6496103
0x1404db727 49b88d08295fccba6ed6 movabs r8, 0xd66ebacc5f29088d
0x1404db731 e8ba1fc0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404db736 4885c0 test rax, rax
0x1404db739 0f84c0390000 je 0x1404df0ff
0x1404db73f 4939942448020000 cmp qword ptr [r12 + 0x248], rdx
0x1404db747 0f86b2390000 jbe 0x1404df0ff
0x1404db74d 498b842440020000 mov rax, qword ptr [r12 + 0x240]
0x1404db755 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404db759 4885c9 test rcx, rcx
0x1404db75c 0f849d390000 je 0x1404df0ff
0x1404db762 498b842410020000 mov rax, qword ptr [r12 + 0x210]
0x1404db76a 48f7d1 not rcx
0x1404db76d 488d0c49 lea rcx, [rcx + rcx*2]
0x1404db771 48c1e105 shl rcx, 5
0x1404db775 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404db77b 0f847e390000 je 0x1404df0ff
0x1404db781 4801c8 add rax, rcx
0x1404db784 488b5010 mov rdx, qword ptr [rax + 0x10]
0x1404db788 418b8c24e0050000 mov ecx, dword ptr [r12 + 0x5e0]
0x1404db790 89485c mov dword ptr [rax + 0x5c], ecx
0x1404db793 31c0 xor eax, eax
0x1404db795 48899598140000 mov qword ptr [rbp + 0x1498], rdx
0x1404db79c 483b02 cmp rax, qword ptr [rdx]
0x1404db79f 700c jo 0x1404db7ad
0x1404db7a1 488b8d98140000 mov rcx, qword ptr [rbp + 0x1498]
0x1404db7a8 e8d3231200 call 0x1405fdb80 ; core::ptr::drop_in_place<country_core::render::render_config::VideoSettings>
0x1404db7ad 41b8c8000000 mov r8d, 0xc8
0x1404db7b3 488b8d98140000 mov rcx, qword ptr [rbp + 0x1498]
0x1404db7ba 488d9500060000 lea rdx, [rbp + 0x600]
0x1404db7c1 e83a844402 call 0x142923c00 ; memcpy
0x1404db7c6 31f6 xor esi, esi
0x1404db7c8 4983bc24b800000000 cmp qword ptr [r12 + 0xb8], 0
0x1404db7d1 0f85b0050000 jne 0x1404dbd87
0x1404db7d7 e952060000 jmp 0x1404dbe2e
0x1404db7dc 488b85a8120000 mov rax, qword ptr [rbp + 0x12a8]
0x1404db7e3 48898598140000 mov qword ptr [rbp + 0x1498], rax
0x1404db7ea 488b85b0120000 mov rax, qword ptr [rbp + 0x12b0]
0x1404db7f1 48898568140000 mov qword ptr [rbp + 0x1468], rax
0x1404db7f8 66410f104516 movupd xmm0, xmmword ptr [r13 + 0x16]
0x1404db7fe 660f2985f0050000 movapd xmmword ptr [rbp + 0x5f0], xmm0
0x1404db806 c6857514000001 mov byte ptr [rbp + 0x1475], 1
0x1404db80d 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404db814 48baea1a85f9b727ec09 movabs rdx, 0x9ec27b7f9851aea
0x1404db81e 49b83e88e6b989f0731e movabs r8, 0x1e73f089b9e6883e
0x1404db828 e8c31ec0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404db82d 4885c0 test rax, rax
0x1404db830 0f8401380000 je 0x1404df037
0x1404db836 4939942448020000 cmp qword ptr [r12 + 0x248], rdx
0x1404db83e 0f86f3370000 jbe 0x1404df037
0x1404db844 498b842440020000 mov rax, qword ptr [r12 + 0x240]
0x1404db84c 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404db850 4885c9 test rcx, rcx
0x1404db853 0f84de370000 je 0x1404df037
0x1404db859 498b842410020000 mov rax, qword ptr [r12 + 0x210]
0x1404db861 48f7d1 not rcx
0x1404db864 488d0c49 lea rcx, [rcx + rcx*2]
0x1404db868 48c1e105 shl rcx, 5
0x1404db86c 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404db872 0f84bf370000 je 0x1404df037
0x1404db878 4801c8 add rax, rcx
0x1404db87b 488b7010 mov rsi, qword ptr [rax + 0x10]
0x1404db87f 418b8c24e0050000 mov ecx, dword ptr [r12 + 0x5e0]
0x1404db887 89485c mov dword ptr [rax + 0x5c], ecx
0x1404db88a 488b7e30 mov rdi, qword ptr [rsi + 0x30]
0x1404db88e 488b5e40 mov rbx, qword ptr [rsi + 0x40]
0x1404db892 483b7e20 cmp rdi, qword ptr [rsi + 0x20]
0x1404db896 7510 jne 0x1404db8a8
0x1404db898 488d4e20 lea rcx, [rsi + 0x20]
0x1404db89c 488d1585785102 lea rdx, [rip + 0x2517885]
0x1404db8a3 e808720800 call 0x140562ab0 ; alloc::raw_vec::RawVec::grow_one<dxgi_enumerate_gpus::DxgiAdapter,alloc::alloc::Global>
0x1404db8a8 488b4628 mov rax, qword ptr [rsi + 0x28]
0x1404db8ac 488d0cbf lea rcx, [rdi + rdi*4]
0x1404db8b0 488b9598140000 mov rdx, qword ptr [rbp + 0x1498]
0x1404db8b7 488914c8 mov qword ptr [rax + rcx*8], rdx
0x1404db8bb 488b9568140000 mov rdx, qword ptr [rbp + 0x1468]
0x1404db8c2 488954c808 mov qword ptr [rax + rcx*8 + 8], rdx
0x1404db8c7 660f2885f0050000 movapd xmm0, xmmword ptr [rbp + 0x5f0]
0x1404db8cf 660f1144c810 movupd xmmword ptr [rax + rcx*8 + 0x10], xmm0
0x1404db8d5 48895cc820 mov qword ptr [rax + rcx*8 + 0x20], rbx
0x1404db8da e9a1f2ffff jmp 0x1404dab80
0x1404db8df 408ab5a2120000 mov sil, byte ptr [rbp + 0x12a2]
0x1404db8e6 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404db8ed c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404db8f4 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404db8fb 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404db902 48bacfde921b5889d12e movabs rdx, 0x2ed189581b92decf
0x1404db90c 49b8385f069693d9d315 movabs r8, 0x15d3d99396065f38
0x1404db916 e8d51dc0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404db91b 4885c0 test rax, rax
0x1404db91e 0f8419380000 je 0x1404df13d
0x1404db924 4939942448020000 cmp qword ptr [r12 + 0x248], rdx
0x1404db92c 0f860b380000 jbe 0x1404df13d
0x1404db932 498b842440020000 mov rax, qword ptr [r12 + 0x240]
0x1404db93a 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404db93e 4885c9 test rcx, rcx
0x1404db941 0f84f6370000 je 0x1404df13d
0x1404db947 498b842410020000 mov rax, qword ptr [r12 + 0x210]
0x1404db94f 48f7d1 not rcx
0x1404db952 488d0c49 lea rcx, [rcx + rcx*2]
0x1404db956 48c1e105 shl rcx, 5
0x1404db95a 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404db960 0f84d7370000 je 0x1404df13d
0x1404db966 4801c8 add rax, rcx
0x1404db969 488b4010 mov rax, qword ptr [rax + 0x10]
0x1404db96d 488b10 mov rdx, qword ptr [rax]
0x1404db970 4883c210 add rdx, 0x10
0x1404db974 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404db97b c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404db982 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404db989 488b8d20140000 mov rcx, qword ptr [rbp + 0x1420]
0x1404db990 4189f0 mov r8d, esi
0x1404db993 e8182d7300 call 0x140c0e6b0 ; rhapsody::raw::swapchain::Swapchain::on_focus
0x1404db998 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404db99f c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404db9a6 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404db9ad 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404db9b4 48bacb64e7ae73167cbb movabs rdx, 0xbb7c1673aee764cb
0x1404db9be 49b8d6e2453938ee0a25 movabs r8, 0x250aee383945e2d6
0x1404db9c8 e8231dc0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404db9cd 4885c0 test rax, rax
0x1404db9d0 0f8441370000 je 0x1404df117
0x1404db9d6 4939942448020000 cmp qword ptr [r12 + 0x248], rdx
0x1404db9de 0f8633370000 jbe 0x1404df117
0x1404db9e4 498b842440020000 mov rax, qword ptr [r12 + 0x240]
0x1404db9ec 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404db9f0 4885c9 test rcx, rcx
0x1404db9f3 0f841e370000 je 0x1404df117
0x1404db9f9 498b842410020000 mov rax, qword ptr [r12 + 0x210]
0x1404dba01 48f7d1 not rcx
0x1404dba04 488d0c49 lea rcx, [rcx + rcx*2]
0x1404dba08 48c1e105 shl rcx, 5
0x1404dba0c 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dba12 0f84ff360000 je 0x1404df117
0x1404dba18 4801c8 add rax, rcx
0x1404dba1b 488b7810 mov rdi, qword ptr [rax + 0x10]
0x1404dba1f 418b8c24e0050000 mov ecx, dword ptr [r12 + 0x5e0]
0x1404dba27 89485c mov dword ptr [rax + 0x5c], ecx
0x1404dba2a 488b5f30 mov rbx, qword ptr [rdi + 0x30]
0x1404dba2e 4c8b7740 mov r14, qword ptr [rdi + 0x40]
0x1404dba32 483b5f20 cmp rbx, qword ptr [rdi + 0x20]
0x1404dba36 7525 jne 0x1404dba5d
0x1404dba38 488d4f20 lea rcx, [rdi + 0x20]
0x1404dba3c c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404dba43 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404dba4a c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dba51 488d15d0765102 lea rdx, [rip + 0x25176d0]
0x1404dba58 e8d36b0800 call 0x140562630 ; alloc::raw_vec::RawVec::grow_one<tuple$<enum2$<core::option::Option<bevy_ecs::entity::Entity> >,bool>,alloc::alloc::Global>
0x1404dba5d 488b4728 mov rax, qword ptr [rdi + 0x28]
0x1404dba61 4889d9 mov rcx, rbx
0x1404dba64 48c1e104 shl rcx, 4
0x1404dba68 4c893408 mov qword ptr [rax + rcx], r14
0x1404dba6c 4088740808 mov byte ptr [rax + rcx + 8], sil
0x1404dba71 e910060000 jmp 0x1404dc086
0x1404dba76 8a9da6120000 mov bl, byte ptr [rbp + 0x12a6]
0x1404dba7c f30f10b5a2120000 movss xmm6, dword ptr [rbp + 0x12a2]
0x1404dba84 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404dba8b c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404dba92 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dba99 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dbaa0 48ba7aee7f5788aa1ee7 movabs rdx, 0xe71eaa88577fee7a
0x1404dbaaa 49b899882c7cb0d4a591 movabs r8, 0x91a5d4b07c2c8899
0x1404dbab4 e8371cc0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404dbab9 4885c0 test rax, rax
0x1404dbabc 0f84d9350000 je 0x1404df09b
0x1404dbac2 4939942448020000 cmp qword ptr [r12 + 0x248], rdx
0x1404dbaca 0f86cb350000 jbe 0x1404df09b
0x1404dbad0 498b842440020000 mov rax, qword ptr [r12 + 0x240]
0x1404dbad8 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404dbadc 4885c9 test rcx, rcx
0x1404dbadf 0f84b6350000 je 0x1404df09b
0x1404dbae5 498b842410020000 mov rax, qword ptr [r12 + 0x210]
0x1404dbaed 48f7d1 not rcx
0x1404dbaf0 488d0c49 lea rcx, [rcx + rcx*2]
0x1404dbaf4 48c1e105 shl rcx, 5
0x1404dbaf8 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dbafe 0f8497350000 je 0x1404df09b
0x1404dbb04 4801c8 add rax, rcx
0x1404dbb07 488b7010 mov rsi, qword ptr [rax + 0x10]
0x1404dbb0b 418b8c24e0050000 mov ecx, dword ptr [r12 + 0x5e0]
0x1404dbb13 89485c mov dword ptr [rax + 0x5c], ecx
0x1404dbb16 488b7e30 mov rdi, qword ptr [rsi + 0x30]
0x1404dbb1a 4c8b7640 mov r14, qword ptr [rsi + 0x40]
0x1404dbb1e 483b7e20 cmp rdi, qword ptr [rsi + 0x20]
0x1404dbb22 7525 jne 0x1404dbb49
0x1404dbb24 488d4e20 lea rcx, [rsi + 0x20]
0x1404dbb28 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404dbb2f c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404dbb36 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dbb3d 488d15e4755102 lea rdx, [rip + 0x25175e4]
0x1404dbb44 e867720800 call 0x140562db0 ; alloc::raw_vec::RawVec::grow_one<relative_path::RelativePathBuf,alloc::alloc::Global>
0x1404dbb49 488b4628 mov rax, qword ptr [rsi + 0x28]
0x1404dbb4d 488d0c7f lea rcx, [rdi + rdi*2]
0x1404dbb51 4c8934c8 mov qword ptr [rax + rcx*8], r14
0x1404dbb55 48baffffffff01000000 movabs rdx, 0x1ffffffff
0x1404dbb5f 488954c808 mov qword ptr [rax + rcx*8 + 8], rdx
0x1404dbb64 f30f1174c810 movss dword ptr [rax + rcx*8 + 0x10], xmm6
0x1404dbb6a 885cc814 mov byte ptr [rax + rcx*8 + 0x14], bl
0x1404dbb6e e90df0ffff jmp 0x1404dab80
0x1404dbb73 498b842400110000 mov rax, qword ptr [r12 + 0x1100]
0x1404dbb7b 493b8424f8100000 cmp rax, qword ptr [r12 + 0x10f8]
0x1404dbb83 0f8202f0ffff jb 0x1404dab8b
0x1404dbb89 660f10b5a8120000 movupd xmm6, xmmword ptr [rbp + 0x12a8]
0x1404dbb91 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404dbb98 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404dbb9f c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dbba6 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dbbad 48ba3626d531c31a3f92 movabs rdx, 0x923f1ac331d52636
0x1404dbbb7 49b8f62c93ffeac91441 movabs r8, 0x4114c9eaff932cf6
0x1404dbbc1 e82a1bc0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404dbbc6 4885c0 test rax, rax
0x1404dbbc9 0f84fb360000 je 0x1404df2ca
0x1404dbbcf 4939942448020000 cmp qword ptr [r12 + 0x248], rdx
0x1404dbbd7 0f86ed360000 jbe 0x1404df2ca
0x1404dbbdd 498b842440020000 mov rax, qword ptr [r12 + 0x240]
0x1404dbbe5 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404dbbe9 4885c9 test rcx, rcx
0x1404dbbec 0f84d8360000 je 0x1404df2ca
0x1404dbbf2 498b842410020000 mov rax, qword ptr [r12 + 0x210]
0x1404dbbfa 48f7d1 not rcx
0x1404dbbfd 488d0c49 lea rcx, [rcx + rcx*2]
0x1404dbc01 48c1e105 shl rcx, 5
0x1404dbc05 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dbc0b 0f84b9360000 je 0x1404df2ca
0x1404dbc11 4801c8 add rax, rcx
0x1404dbc14 488b7010 mov rsi, qword ptr [rax + 0x10]
0x1404dbc18 418b8c24e0050000 mov ecx, dword ptr [r12 + 0x5e0]
0x1404dbc20 89485c mov dword ptr [rax + 0x5c], ecx
0x1404dbc23 488b7e30 mov rdi, qword ptr [rsi + 0x30]
0x1404dbc27 488b5e40 mov rbx, qword ptr [rsi + 0x40]
0x1404dbc2b 483b7e20 cmp rdi, qword ptr [rsi + 0x20]
0x1404dbc2f 7525 jne 0x1404dbc56
0x1404dbc31 488d4e20 lea rcx, [rsi + 0x20]
0x1404dbc35 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404dbc3c c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404dbc43 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dbc4a 488d15d7745102 lea rdx, [rip + 0x25174d7]
0x1404dbc51 e85a710800 call 0x140562db0 ; alloc::raw_vec::RawVec::grow_one<relative_path::RelativePathBuf,alloc::alloc::Global>
0x1404dbc56 660f5ac6 cvtpd2ps xmm0, xmm6
0x1404dbc5a 488b4628 mov rax, qword ptr [rsi + 0x28]
0x1404dbc5e 488d0c7f lea rcx, [rdi + rdi*2]
0x1404dbc62 48891cc8 mov qword ptr [rax + rcx*8], rbx
0x1404dbc66 c744c80801000000 mov dword ptr [rax + rcx*8 + 8], 1
0x1404dbc6e 660f1344c80c movlpd qword ptr [rax + rcx*8 + 0xc], xmm0
0x1404dbc74 e907efffff jmp 0x1404dab80
0x1404dbc79 c6437c01 mov byte ptr [rbx + 0x7c], 1
0x1404dbc7d 31c0 xor eax, eax
0x1404dbc7f b101 mov cl, 1
0x1404dbc81 f00fb00d088f1503 lock cmpxchg byte ptr [rip + 0x3158f08], cl
0x1404dbc89 0f853e0a0000 jne 0x1404dc6cd
0x1404dbc8f 488b058a071503 mov rax, qword ptr [rip + 0x315078a]
0x1404dbc96 488b00 mov rax, qword ptr [rax]
0x1404dbc99 48d1e0 shl rax, 1
0x1404dbc9c 4885c0 test rax, rax
0x1404dbc9f 0f854e0a0000 jne 0x1404dc6f3
0x1404dbca5 31c0 xor eax, eax
0x1404dbca7 0fb60de48e1503 movzx ecx, byte ptr [rip + 0x3158ee4]
0x1404dbcae 84c9 test cl, cl
0x1404dbcb0 0f8505380000 jne 0x1404df4bb
0x1404dbcb6 800dd68e150301 or byte ptr [rip + 0x3158ed6], 1
0x1404dbcbd 84c0 test al, al
0x1404dbcbf 7516 jne 0x1404dbcd7
0x1404dbcc1 488b0558071503 mov rax, qword ptr [rip + 0x3150758]
0x1404dbcc8 488b00 mov rax, qword ptr [rax]
0x1404dbccb 48d1e0 shl rax, 1
0x1404dbcce 4885c0 test rax, rax
0x1404dbcd1 0f85000b0000 jne 0x1404dc7d7
0x1404dbcd7 31c0 xor eax, eax
0x1404dbcd9 8605b28e1503 xchg byte ptr [rip + 0x3158eb2], al
0x1404dbcdf 3c02 cmp al, 2
0x1404dbce1 0f85a4eeffff jne 0x1404dab8b
0x1404dbce7 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404dbcee c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404dbcf5 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dbcfc 488d0d8e8e1503 lea rcx, [rip + 0x3158e8e]
0x1404dbd03 e808cb3f02 call 0x1428d8810 ; std::sys::sync::mutex::futex::Mutex::wake
0x1404dbd08 e97eeeffff jmp 0x1404dab8b
0x1404dbd0d 48b80100000000000080 movabs rax, 0x8000000000000001
0x1404dbd17 488985300c0000 mov qword ptr [rbp + 0xc30], rax
0x1404dbd1e 895f5c mov dword ptr [rdi + 0x5c], ebx
0x1404dbd21 488b5638 mov rdx, qword ptr [rsi + 0x38]
0x1404dbd25 48b80100000000000080 movabs rax, 0x8000000000000001
0x1404dbd2f 4839c2 cmp rdx, rax
0x1404dbd32 7419 je 0x1404dbd4d
0x1404dbd34 4889d0 mov rax, rdx
0x1404dbd37 48f7d8 neg rax
0x1404dbd3a 7011 jo 0x1404dbd4d
0x1404dbd3c 730f jae 0x1404dbd4d
0x1404dbd3e 488b4e40 mov rcx, qword ptr [rsi + 0x40]
0x1404dbd42 41b801000000 mov r8d, 1
0x1404dbd48 e8d37e1300 call 0x140613c20 ; __rust_dealloc
0x1404dbd4d 4883c638 add rsi, 0x38
0x1404dbd51 488b85500c0000 mov rax, qword ptr [rbp + 0xc50]
0x1404dbd58 48894620 mov qword ptr [rsi + 0x20], rax
0x1404dbd5c 660f1085300c0000 movupd xmm0, xmmword ptr [rbp + 0xc30]
0x1404dbd64 f30f6f8d400c0000 movdqu xmm1, xmmword ptr [rbp + 0xc40]
0x1404dbd6c f30f7f4e10 movdqu xmmword ptr [rsi + 0x10], xmm1
0x1404dbd71 660f1106 movupd xmmword ptr [rsi], xmm0
0x1404dbd75 40b601 mov sil, 1
0x1404dbd78 4983bc24b800000000 cmp qword ptr [r12 + 0xb8], 0
0x1404dbd81 0f84a7000000 je 0x1404dbe2e
0x1404dbd87 498b8424a0000000 mov rax, qword ptr [r12 + 0xa0]
0x1404dbd8f 498b8c24a8000000 mov rcx, qword ptr [r12 + 0xa8]
0x1404dbd97 4889ca mov rdx, rcx
0x1404dbd9a 49b89d8ebb86a993b929 movabs r8, 0x29b993a986bb8e9d
0x1404dbda4 4c21c2 and rdx, r8
0x1404dbda7 4531c0 xor r8d, r8d
0x1404dbdaa f30f6f0410 movdqu xmm0, xmmword ptr [rax + rdx]
0x1404dbdaf 660f6fc8 movdqa xmm1, xmm0
0x1404dbdb3 66410f74c9 pcmpeqb xmm1, xmm9
0x1404dbdb8 66440fd7c9 pmovmskb r9d, xmm1
0x1404dbdbd 4585c9 test r9d, r9d
0x1404dbdc0 752b jne 0x1404dbded
0x1404dbdc2 660f74c7 pcmpeqb xmm0, xmm7
0x1404dbdc6 66440fd7c8 pmovmskb r9d, xmm0
0x1404dbdcb 4585c9 test r9d, r9d
0x1404dbdce 755e jne 0x1404dbe2e
0x1404dbdd0 4c01c2 add rdx, r8
0x1404dbdd3 4883c210 add rdx, 0x10
0x1404dbdd7 4983c010 add r8, 0x10
0x1404dbddb 4821ca and rdx, rcx
0x1404dbdde ebca jmp 0x1404dbdaa
0x1404dbde0 458d51ff lea r10d, [r9 - 1]
0x1404dbde4 664521ca and r10w, r9w
0x1404dbde8 4589d1 mov r9d, r10d
0x1404dbdeb 74d5 je 0x1404dbdc2
0x1404dbded f3450fbcd1 tzcnt r10d, r9d
0x1404dbdf2 4901d2 add r10, rdx
0x1404dbdf5 4921ca and r10, rcx
0x1404dbdf8 49f7da neg r10
0x1404dbdfb 4f8d1452 lea r10, [r10 + r10*2]
0x1404dbdff 49bba01cbf3bbb4f550a movabs r11, 0xa554fbb3bbf1ca0
0x1404dbe09 4e395cd0e8 cmp qword ptr [rax + r10*8 - 0x18], r11
0x1404dbe0e 75d0 jne 0x1404dbde0
0x1404dbe10 49bb9d8ebb86a993b929 movabs r11, 0x29b993a986bb8e9d
0x1404dbe1a 4e395cd0f0 cmp qword ptr [rax + r10*8 - 0x10], r11
0x1404dbe1f 75bf jne 0x1404dbde0
0x1404dbe21 4a8d04d0 lea rax, [rax + r10*8]
0x1404dbe25 488b58f8 mov rbx, qword ptr [rax - 8]
0x1404dbe29 e9c9010000 jmp 0x1404dbff7
0x1404dbe2e 488b85d8130000 mov rax, qword ptr [rbp + 0x13d8]
0x1404dbe35 8b00 mov eax, dword ptr [rax]
0x1404dbe37 3dfdffff3f cmp eax, 0x3ffffffd
0x1404dbe3c 0f8716080000 ja 0x1404dc658
0x1404dbe42 8d4801 lea ecx, [rax + 1]
0x1404dbe45 488b95d8130000 mov rdx, qword ptr [rbp + 0x13d8]
0x1404dbe4c f00fb10a lock cmpxchg dword ptr [rdx], ecx
0x1404dbe50 0f8502080000 jne 0x1404dc658
0x1404dbe56 410fb68424c8000000 movzx eax, byte ptr [r12 + 0xc8]
0x1404dbe5f 40b701 mov dil, 1
0x1404dbe62 4983bc242001000000 cmp qword ptr [r12 + 0x120], 0
0x1404dbe6b 0f845c010000 je 0x1404dbfcd
0x1404dbe71 498b842408010000 mov rax, qword ptr [r12 + 0x108]
0x1404dbe79 498b8c2410010000 mov rcx, qword ptr [r12 + 0x110]
0x1404dbe81 4889ca mov rdx, rcx
0x1404dbe84 49b89d8ebb86a993b929 movabs r8, 0x29b993a986bb8e9d
0x1404dbe8e 4c21c2 and rdx, r8
0x1404dbe91 4531c0 xor r8d, r8d
0x1404dbe94 f30f6f0410 movdqu xmm0, xmmword ptr [rax + rdx]
0x1404dbe99 660f6fc8 movdqa xmm1, xmm0
0x1404dbe9d 66410f74c9 pcmpeqb xmm1, xmm9
0x1404dbea2 66440fd7c9 pmovmskb r9d, xmm1
0x1404dbea7 4585c9 test r9d, r9d
0x1404dbeaa 7531 jne 0x1404dbedd
0x1404dbeac 660f74c7 pcmpeqb xmm0, xmm7
0x1404dbeb0 66440fd7c8 pmovmskb r9d, xmm0
0x1404dbeb5 4585c9 test r9d, r9d
0x1404dbeb8 0f850f010000 jne 0x1404dbfcd
0x1404dbebe 4c01c2 add rdx, r8
0x1404dbec1 4883c210 add rdx, 0x10
0x1404dbec5 4983c010 add r8, 0x10
0x1404dbec9 4821ca and rdx, rcx
0x1404dbecc ebc6 jmp 0x1404dbe94
0x1404dbece 6690 nop 
0x1404dbed0 458d51ff lea r10d, [r9 - 1]
0x1404dbed4 664521ca and r10w, r9w
0x1404dbed8 4589d1 mov r9d, r10d
0x1404dbedb 74cf je 0x1404dbeac
0x1404dbedd f3450fbcd9 tzcnt r11d, r9d
0x1404dbee2 4901d3 add r11, rdx
0x1404dbee5 4921cb and r11, rcx
0x1404dbee8 4d89da mov r10, r11
0x1404dbeeb 49f7da neg r10
0x1404dbeee 49c1e303 shl r11, 3
0x1404dbef2 49c1e207 shl r10, 7
0x1404dbef6 4d29da sub r10, r11
0x1404dbef9 49bba01cbf3bbb4f550a movabs r11, 0xa554fbb3bbf1ca0
0x1404dbf03 4e399c1078ffffff cmp qword ptr [rax + r10 - 0x88], r11
0x1404dbf0b 75c3 jne 0x1404dbed0
0x1404dbf0d 49bb9d8ebb86a993b929 movabs r11, 0x29b993a986bb8e9d
0x1404dbf17 4e395c1080 cmp qword ptr [rax + r10 - 0x80], r11
0x1404dbf1c 75b2 jne 0x1404dbed0
0x1404dbf1e 4c01d0 add rax, r10
0x1404dbf21 488b58f8 mov rbx, qword ptr [rax - 8]
0x1404dbf25 31ff xor edi, edi
0x1404dbf27 e9a1000000 jmp 0x1404dbfcd
0x1404dbf2c 6683bd300c000000 cmp word ptr [rbp + 0xc30], 0
0x1404dbf34 0f85b8f4ffff jne 0x1404db3f2
0x1404dbf3a 6683bd320c000021 cmp word ptr [rbp + 0xc32], 0x21
0x1404dbf42 0f85aaf4ffff jne 0x1404db3f2
0x1404dbf48 80bda80c000000 cmp byte ptr [rbp + 0xca8], 0
0x1404dbf4f 0f8505030000 jne 0x1404dc25a
0x1404dbf55 4180bc241411000001 cmp byte ptr [r12 + 0x1114], 1
0x1404dbf5e 0f85f6020000 jne 0x1404dc25a
0x1404dbf64 c685a214000001 mov byte ptr [rbp + 0x14a2], 1
0x1404dbf6b 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404dbf72 488b95a8130000 mov rdx, qword ptr [rbp + 0x13a8]
0x1404dbf79 e8b22b1500 call 0x14062eb30 ; arboard::Clipboard::get_text
0x1404dbf7e 488b9508060000 mov rdx, qword ptr [rbp + 0x608]
0x1404dbf85 83bd0006000001 cmp dword ptr [rbp + 0x600], 1
0x1404dbf8c 0f85c0020000 jne 0x1404dc252
0x1404dbf92 b001 mov al, 1
0x1404dbf94 898598140000 mov dword ptr [rbp + 0x1498], eax
0x1404dbf9a 48b80400000000000080 movabs rax, 0x8000000000000004
0x1404dbfa4 4839c2 cmp rdx, rax
0x1404dbfa7 0f8c0e060000 jl 0x1404dc5bb
0x1404dbfad 4885d2 test rdx, rdx
0x1404dbfb0 0f8405060000 je 0x1404dc5bb
0x1404dbfb6 488b8d10060000 mov rcx, qword ptr [rbp + 0x610]
0x1404dbfbd 41b801000000 mov r8d, 1
0x1404dbfc3 e8587c1300 call 0x140613c20 ; __rust_dealloc
0x1404dbfc8 e9ee050000 jmp 0x1404dc5bb
0x1404dbfcd baffffffff mov edx, 0xffffffff
0x1404dbfd2 488b85d8130000 mov rax, qword ptr [rbp + 0x13d8]
0x1404dbfd9 f00fc110 lock xadd dword ptr [rax], edx
0x1404dbfdd ffca dec edx
0x1404dbfdf 89d0 mov eax, edx
0x1404dbfe1 25ffffffbf and eax, 0xbfffffff
0x1404dbfe6 f7d8 neg eax
0x1404dbfe8 0f8097070000 jo 0x1404dc785
0x1404dbfee 4084ff test dil, dil
0x1404dbff1 0f858b2f0000 jne 0x1404def82
0x1404dbff7 49399c2448020000 cmp qword ptr [r12 + 0x248], rbx
0x1404dbfff 0f867d2f0000 jbe 0x1404def82
0x1404dc005 498b842440020000 mov rax, qword ptr [r12 + 0x240]
0x1404dc00d 488b0cd8 mov rcx, qword ptr [rax + rbx*8]
0x1404dc011 4885c9 test rcx, rcx
0x1404dc014 0f84682f0000 je 0x1404def82
0x1404dc01a 498b842410020000 mov rax, qword ptr [r12 + 0x210]
0x1404dc022 48f7d1 not rcx
0x1404dc025 488d0c49 lea rcx, [rcx + rcx*2]
0x1404dc029 48c1e105 shl rcx, 5
0x1404dc02d 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dc033 0f84492f0000 je 0x1404def82
0x1404dc039 4801c8 add rax, rcx
0x1404dc03c 488b7810 mov rdi, qword ptr [rax + 0x10]
0x1404dc040 418b8c24e0050000 mov ecx, dword ptr [r12 + 0x5e0]
0x1404dc048 89485c mov dword ptr [rax + 0x5c], ecx
0x1404dc04b 488b5f30 mov rbx, qword ptr [rdi + 0x30]
0x1404dc04f 4c8b7740 mov r14, qword ptr [rdi + 0x40]
0x1404dc053 483b5f20 cmp rbx, qword ptr [rdi + 0x20]
0x1404dc057 7525 jne 0x1404dc07e
0x1404dc059 488d4f20 lea rcx, [rdi + 0x20]
0x1404dc05d 4088b5a7140000 mov byte ptr [rbp + 0x14a7], sil
0x1404dc064 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404dc06b c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dc072 488d15af705102 lea rdx, [rip + 0x25170af]
0x1404dc079 e8f2670800 call 0x140562870 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::glade_border::ShowDemoBorderOverlay>,alloc::alloc::Global>
0x1404dc07e 488b4728 mov rax, qword ptr [rdi + 0x28]
0x1404dc082 4c8934d8 mov qword ptr [rax + rbx*8], r14
0x1404dc086 48ffc3 inc rbx
0x1404dc089 48895f30 mov qword ptr [rdi + 0x30], rbx
0x1404dc08d 48ff4740 inc qword ptr [rdi + 0x40]
0x1404dc091 e9f5eaffff jmp 0x1404dab8b
0x1404dc096 31f6 xor esi, esi
0x1404dc098 f6c201 test dl, 1
0x1404dc09b 0f84e8eeffff je 0x1404daf89
0x1404dc0a1 e9f9eeffff jmp 0x1404daf9f
0x1404dc0a6 488b95980c0000 mov rdx, qword ptr [rbp + 0xc98]
0x1404dc0ad f048ff02 lock inc qword ptr [rdx]
0x1404dc0b1 0f8ea3350000 jle 0x1404df65a
0x1404dc0b7 41b018 mov r8b, 0x18
0x1404dc0ba eb0a jmp 0x1404dc0c6
0x1404dc0bc 41b01a mov r8b, 0x1a
0x1404dc0bf 488b95980c0000 mov rdx, qword ptr [rbp + 0xc98]
0x1404dc0c6 4c8b8da00c0000 mov r9, qword ptr [rbp + 0xca0]
0x1404dc0cd 448b9530140000 mov r10d, dword ptr [rbp + 0x1430]
0x1404dc0d4 448b9d33140000 mov r11d, dword ptr [rbp + 0x1433]
0x1404dc0db 44899558140000 mov dword ptr [rbp + 0x1458], r10d
0x1404dc0e2 44899d5b140000 mov dword ptr [rbp + 0x145b], r11d
0x1404dc0e9 4589c2 mov r10d, r8d
0x1404dc0ec c1eb02 shr ebx, 2
0x1404dc0ef 448d4304 lea r8d, [rbx + 4]
0x1404dc0f3 40f6c620 test sil, 0x20
0x1404dc0f7 448b9d08060000 mov r11d, dword ptr [rbp + 0x608]
0x1404dc0fe 488d7d00 lea rdi, [rbp]
0x1404dc102 44895f08 mov dword ptr [rdi + 8], r11d
0x1404dc106 4c8b9d00060000 mov r11, qword ptr [rbp + 0x600]
0x1404dc10d 4c891f mov qword ptr [rdi], r11
0x1404dc110 c745c0b3000000 mov dword ptr [rbp - 0x40], 0xb3
0x1404dc117 884d0c mov byte ptr [rbp + 0xc], cl
0x1404dc11a 448855e0 mov byte ptr [rbp - 0x20], r10b
0x1404dc11e 8b8d58140000 mov ecx, dword ptr [rbp + 0x1458]
0x1404dc124 448b955b140000 mov r10d, dword ptr [rbp + 0x145b]
0x1404dc12b 448957e4 mov dword ptr [rdi - 0x1c], r10d
0x1404dc12f 894fe1 mov dword ptr [rdi - 0x1f], ecx
0x1404dc132 488955e8 mov qword ptr [rbp - 0x18], rdx
0x1404dc136 4c894df0 mov qword ptr [rbp - 0x10], r9
0x1404dc13a 88450d mov byte ptr [rbp + 0xd], al
0x1404dc13d 48b8ffffffff01000000 movabs rax, 0x1ffffffff
0x1404dc147 488945f8 mov qword ptr [rbp - 8], rax
0x1404dc14b 410fb6f8 movzx edi, r8b
0x1404dc14f 0f44fb cmove edi, ebx
0x1404dc152 c6859614000001 mov byte ptr [rbp + 0x1496], 1
0x1404dc159 c6859514000001 mov byte ptr [rbp + 0x1495], 1
0x1404dc160 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dc167 48ba79adb1ac921a18a6 movabs rdx, 0xa6181a92acb1ad79
0x1404dc171 49b8e97c40a3c9b6586d movabs r8, 0x6d58b6c9a3407ce9
0x1404dc17b e87015c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404dc180 b301 mov bl, 1
0x1404dc182 4885c0 test rax, rax
0x1404dc185 0f84e1020000 je 0x1404dc46c
0x1404dc18b 4939942448020000 cmp qword ptr [r12 + 0x248], rdx
0x1404dc193 0f86d3020000 jbe 0x1404dc46c
0x1404dc199 498b842440020000 mov rax, qword ptr [r12 + 0x240]
0x1404dc1a1 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404dc1a5 4885c9 test rcx, rcx
0x1404dc1a8 0f84be020000 je 0x1404dc46c
0x1404dc1ae 498b842410020000 mov rax, qword ptr [r12 + 0x210]
0x1404dc1b6 48f7d1 not rcx
0x1404dc1b9 488d0c49 lea rcx, [rcx + rcx*2]
0x1404dc1bd 48c1e105 shl rcx, 5
0x1404dc1c1 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dc1c7 0f849f020000 je 0x1404dc46c
0x1404dc1cd 4801c8 add rax, rcx
0x1404dc1d0 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1404dc1d4 418b9424e0050000 mov edx, dword ptr [r12 + 0x5e0]
0x1404dc1dc 89505c mov dword ptr [rax + 0x5c], edx
0x1404dc1df 8b5dc0 mov ebx, dword ptr [rbp - 0x40]
0x1404dc1e2 8d53fd lea edx, [rbx - 3]
0x1404dc1e5 81fa32010000 cmp edx, 0x132
0x1404dc1eb 0f838d040000 jae 0x1404dc67e
0x1404dc1f1 440fb64de0 movzx r9d, byte ptr [rbp - 0x20]
0x1404dc1f6 0fb6550c movzx edx, byte ptr [rbp + 0xc]
0x1404dc1fa 41b31b mov r11b, 0x1b
0x1404dc1fd 4180f91b cmp r9b, 0x1b
0x1404dc201 0f847f010000 je 0x1404dc386
0x1404dc207 458d41e8 lea r8d, [r9 - 0x18]
0x1404dc20b 4180f803 cmp r8b, 3
0x1404dc20f 450fb6c0 movzx r8d, r8b
0x1404dc213 b801000000 mov eax, 1
0x1404dc218 440f43c0 cmovae r8d, eax
0x1404dc21c 4584c0 test r8b, r8b
0x1404dc21f 0f8440010000 je 0x1404dc365
0x1404dc225 450fb6c0 movzx r8d, r8b
0x1404dc229 4183f801 cmp r8d, 1
0x1404dc22d 0f8545010000 jne 0x1404dc378
0x1404dc233 4c8d5500 lea r10, [rbp]
0x1404dc237 458b42e1 mov r8d, dword ptr [r10 - 0x1f]
0x1404dc23b 458b52e4 mov r10d, dword ptr [r10 - 0x1c]
0x1404dc23f 4489955b140000 mov dword ptr [rbp + 0x145b], r10d
0x1404dc246 44898558140000 mov dword ptr [rbp + 0x1458], r8d
0x1404dc24d e929010000 jmp 0x1404dc37b
0x1404dc252 4889d0 mov rax, rdx
0x1404dc255 48f7d8 neg rax
0x1404dc258 710d jno 0x1404dc267
0x1404dc25a b001 mov al, 1
0x1404dc25c 898598140000 mov dword ptr [rbp + 0x1498], eax
0x1404dc262 e954030000 jmp 0x1404dc5bb
0x1404dc267 48899568140000 mov qword ptr [rbp + 0x1468], rdx
0x1404dc26e 488b8510060000 mov rax, qword ptr [rbp + 0x610]
0x1404dc275 48898560140000 mov qword ptr [rbp + 0x1460], rax
0x1404dc27c 488bb518060000 mov rsi, qword ptr [rbp + 0x618]
0x1404dc283 c6857314000001 mov byte ptr [rbp + 0x1473], 1
0x1404dc28a 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dc291 48ba79adb1ac921a18a6 movabs rdx, 0xa6181a92acb1ad79
0x1404dc29b 49b8e97c40a3c9b6586d movabs r8, 0x6d58b6c9a3407ce9
0x1404dc2a5 e84614c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404dc2aa 4885c0 test rax, rax
0x1404dc2ad 0f8483000000 je 0x1404dc336
0x1404dc2b3 4939942448020000 cmp qword ptr [r12 + 0x248], rdx
0x1404dc2bb 7679 jbe 0x1404dc336
0x1404dc2bd 498b842440020000 mov rax, qword ptr [r12 + 0x240]
0x1404dc2c5 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404dc2c9 4885c9 test rcx, rcx
0x1404dc2cc 7468 je 0x1404dc336
0x1404dc2ce 498b842410020000 mov rax, qword ptr [r12 + 0x210]
0x1404dc2d6 48f7d1 not rcx
0x1404dc2d9 488d0c49 lea rcx, [rcx + rcx*2]
0x1404dc2dd 48c1e105 shl rcx, 5
0x1404dc2e1 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dc2e7 744d je 0x1404dc336
0x1404dc2e9 4801c8 add rax, rcx
0x1404dc2ec 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1404dc2f0 418b9424e0050000 mov edx, dword ptr [r12 + 0x5e0]
0x1404dc2f8 89505c mov dword ptr [rax + 0x5c], edx
0x1404dc2fb 488b8568140000 mov rax, qword ptr [rbp + 0x1468]
0x1404dc302 48898500060000 mov qword ptr [rbp + 0x600], rax
0x1404dc309 488b8560140000 mov rax, qword ptr [rbp + 0x1460]
0x1404dc310 48898508060000 mov qword ptr [rbp + 0x608], rax
0x1404dc317 4889b510060000 mov qword ptr [rbp + 0x610], rsi
0x1404dc31e c6857314000000 mov byte ptr [rbp + 0x1473], 0
0x1404dc325 488d9500060000 lea rdx, [rbp + 0x600]
0x1404dc32c e89f9d1a00 call 0x1406860d0 ; toolbar::mei::core::MeiContext::on_clipboard_paste
0x1404dc331 e924ffffff jmp 0x1404dc25a
0x1404dc336 b001 mov al, 1
0x1404dc338 898598140000 mov dword ptr [rbp + 0x1498], eax
0x1404dc33e 488b9568140000 mov rdx, qword ptr [rbp + 0x1468]
0x1404dc345 4885d2 test rdx, rdx
0x1404dc348 0f846d020000 je 0x1404dc5bb
0x1404dc34e 41b801000000 mov r8d, 1
0x1404dc354 488b8d60140000 mov rcx, qword ptr [rbp + 0x1460]
0x1404dc35b e8c0781300 call 0x140613c20 ; __rust_dealloc
0x1404dc360 e956020000 jmp 0x1404dc5bb
0x1404dc365 4c8b45e8 mov r8, qword ptr [rbp - 0x18]
0x1404dc369 f049ff00 lock inc qword ptr [r8]
0x1404dc36d 0f8ee7320000 jle 0x1404df65a
0x1404dc373 41b118 mov r9b, 0x18
0x1404dc376 eb07 jmp 0x1404dc37f
0x1404dc378 41b11a mov r9b, 0x1a
0x1404dc37b 4c8b45e8 mov r8, qword ptr [rbp - 0x18]
0x1404dc37f 4c8b55f0 mov r10, qword ptr [rbp - 0x10]
0x1404dc383 4589cb mov r11d, r9d
0x1404dc386 4189f1 mov r9d, esi
0x1404dc389 41c1e905 shr r9d, 5
0x1404dc38d 4180e108 and r9b, 8
0x1404dc391 c1ee07 shr esi, 7
0x1404dc394 4080e610 and sil, 0x10
0x1404dc398 4408ce or sil, r9b
0x1404dc39b 4008fe or sil, dil
0x1404dc39e 440fb64d0d movzx r9d, byte ptr [rbp + 0xd]
0x1404dc3a3 488b7df8 mov rdi, qword ptr [rbp - 8]
0x1404dc3a7 488d4500 lea rax, [rbp]
0x1404dc3ab 448b7008 mov r14d, dword ptr [rax + 8]
0x1404dc3af 4c8dbd38060000 lea r15, [rbp + 0x638]
0x1404dc3b6 45897710 mov dword ptr [r15 + 0x10], r14d
0x1404dc3ba 4c8b30 mov r14, qword ptr [rax]
0x1404dc3bd 4d897708 mov qword ptr [r15 + 8], r14
0x1404dc3c1 899d00060000 mov dword ptr [rbp + 0x600], ebx
0x1404dc3c7 8b8598140000 mov eax, dword ptr [rbp + 0x1498]
0x1404dc3cd 898504060000 mov dword ptr [rbp + 0x604], eax
0x1404dc3d3 488b8540140000 mov rax, qword ptr [rbp + 0x1440]
0x1404dc3da 498947e0 mov qword ptr [r15 - 0x20], rax
0x1404dc3de 0f108530140000 movups xmm0, xmmword ptr [rbp + 0x1430]
0x1404dc3e5 410f1147d0 movups xmmword ptr [r15 - 0x30], xmm0
0x1404dc3ea 88954c060000 mov byte ptr [rbp + 0x64c], dl
0x1404dc3f0 44889d20060000 mov byte ptr [rbp + 0x620], r11b
0x1404dc3f7 8b8558140000 mov eax, dword ptr [rbp + 0x1458]
0x1404dc3fd 8b955b140000 mov edx, dword ptr [rbp + 0x145b]
0x1404dc403 418957ec mov dword ptr [r15 - 0x14], edx
0x1404dc407 418947e9 mov dword ptr [r15 - 0x17], eax
0x1404dc40b 4c898528060000 mov qword ptr [rbp + 0x628], r8
0x1404dc412 4c899530060000 mov qword ptr [rbp + 0x630], r10
0x1404dc419 44888d4d060000 mov byte ptr [rbp + 0x64d], r9b
0x1404dc420 4889bd38060000 mov qword ptr [rbp + 0x638], rdi
0x1404dc427 488d95380c0000 lea rdx, [rbp + 0xc38]
0x1404dc42e 488b4268 mov rax, qword ptr [rdx + 0x68]
0x1404dc432 48898540140000 mov qword ptr [rbp + 0x1440], rax
0x1404dc439 660f104258 movupd xmm0, xmmword ptr [rdx + 0x58]
0x1404dc43e 660f298530140000 movapd xmmword ptr [rbp + 0x1430], xmm0
0x1404dc446 c6859614000001 mov byte ptr [rbp + 0x1496], 1
0x1404dc44d c6859514000000 mov byte ptr [rbp + 0x1495], 0
0x1404dc454 488d9500060000 lea rdx, [rbp + 0x600]
0x1404dc45b 4189f0 mov r8d, esi
0x1404dc45e 4c8d8d30140000 lea r9, [rbp + 0x1430]
0x1404dc465 e8569b1a00 call 0x140685fc0 ; toolbar::mei::core::MeiContext::on_key_event
0x1404dc46a 31db xor ebx, ebx
0x1404dc46c c6859614000001 mov byte ptr [rbp + 0x1496], 1
0x1404dc473 889d95140000 mov byte ptr [rbp + 0x1495], bl
0x1404dc479 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dc480 48babaf7bff599300f50 movabs rdx, 0x500f3099f5bff7ba
0x1404dc48a 49b82817c1147099f385 movabs r8, 0x85f3997014c11728
0x1404dc494 e85712c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404dc499 4885c0 test rax, rax
0x1404dc49c 0f84e32c0000 je 0x1404df185
0x1404dc4a2 4939942448020000 cmp qword ptr [r12 + 0x248], rdx
0x1404dc4aa 0f86d52c0000 jbe 0x1404df185
0x1404dc4b0 498b842440020000 mov rax, qword ptr [r12 + 0x240]
0x1404dc4b8 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404dc4bc 4885c9 test rcx, rcx
0x1404dc4bf 0f84c02c0000 je 0x1404df185
0x1404dc4c5 498b842410020000 mov rax, qword ptr [r12 + 0x210]
0x1404dc4cd 48f7d1 not rcx
0x1404dc4d0 488d0c49 lea rcx, [rcx + rcx*2]
0x1404dc4d4 48c1e105 shl rcx, 5
0x1404dc4d8 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dc4de 0f84a12c0000 je 0x1404df185
0x1404dc4e4 899d98140000 mov dword ptr [rbp + 0x1498], ebx
0x1404dc4ea 4801c8 add rax, rcx
0x1404dc4ed 488b7010 mov rsi, qword ptr [rax + 0x10]
0x1404dc4f1 418b8c24e0050000 mov ecx, dword ptr [r12 + 0x5e0]
0x1404dc4f9 89485c mov dword ptr [rax + 0x5c], ecx
0x1404dc4fc 0f104500 movups xmm0, xmmword ptr [rbp]
0x1404dc500 488d8508060000 lea rax, [rbp + 0x608]
0x1404dc507 0f114040 movups xmmword ptr [rax + 0x40], xmm0
0x1404dc50b 660f1045c0 movupd xmm0, xmmword ptr [rbp - 0x40]
0x1404dc510 f30f6f4dd0 movdqu xmm1, xmmword ptr [rbp - 0x30]
0x1404dc515 f30f6f55e0 movdqu xmm2, xmmword ptr [rbp - 0x20]
0x1404dc51a f30f6f5df0 movdqu xmm3, xmmword ptr [rbp - 0x10]
0x1404dc51f f30f7f5830 movdqu xmmword ptr [rax + 0x30], xmm3
0x1404dc524 f30f7f5020 movdqu xmmword ptr [rax + 0x20], xmm2
0x1404dc529 f30f7f4810 movdqu xmmword ptr [rax + 0x10], xmm1
0x1404dc52e 660f1100 movupd xmmword ptr [rax], xmm0
0x1404dc532 488b4640 mov rax, qword ptr [rsi + 0x40]
0x1404dc536 48898500060000 mov qword ptr [rbp + 0x600], rax
0x1404dc53d 488b7e30 mov rdi, qword ptr [rsi + 0x30]
0x1404dc541 483b7e20 cmp rdi, qword ptr [rsi + 0x20]
0x1404dc545 7510 jne 0x1404dc557
0x1404dc547 488d4e20 lea rcx, [rsi + 0x20]
0x1404dc54b 488d15d66b5102 lea rdx, [rip + 0x2516bd6]
0x1404dc552 e8d9690800 call 0x140562f30 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<system_onboarding::hint_orchestrator::DisplayHintRequest>,alloc::alloc::Global>
0x1404dc557 488b4628 mov rax, qword ptr [rsi + 0x28]
0x1404dc55b 486bcf58 imul rcx, rdi, 0x58
0x1404dc55f 660f108500060000 movupd xmm0, xmmword ptr [rbp + 0x600]
0x1404dc567 f30f6f8d10060000 movdqu xmm1, xmmword ptr [rbp + 0x610]
0x1404dc56f f30f6f9520060000 movdqu xmm2, xmmword ptr [rbp + 0x620]
0x1404dc577 f30f6f9d30060000 movdqu xmm3, xmmword ptr [rbp + 0x630]
0x1404dc57f f30f6fa540060000 movdqu xmm4, xmmword ptr [rbp + 0x640]
0x1404dc587 488b9550060000 mov rdx, qword ptr [rbp + 0x650]
0x1404dc58e 4889540850 mov qword ptr [rax + rcx + 0x50], rdx
0x1404dc593 f30f7f640840 movdqu xmmword ptr [rax + rcx + 0x40], xmm4
0x1404dc599 f30f7f5c0830 movdqu xmmword ptr [rax + rcx + 0x30], xmm3
0x1404dc59f f30f7f540820 movdqu xmmword ptr [rax + rcx + 0x20], xmm2
0x1404dc5a5 f30f7f4c0810 movdqu xmmword ptr [rax + rcx + 0x10], xmm1
0x1404dc5ab 660f110408 movupd xmmword ptr [rax + rcx], xmm0
0x1404dc5b0 48ffc7 inc rdi
0x1404dc5b3 48897e30 mov qword ptr [rsi + 0x30], rdi
0x1404dc5b7 48ff4640 inc qword ptr [rsi + 0x40]
0x1404dc5bb 0fb785300c0000 movzx eax, word ptr [rbp + 0xc30]
0x1404dc5c2 83f802 cmp eax, 2
0x1404dc5c5 740e je 0x1404dc5d5
0x1404dc5c7 83f801 cmp eax, 1
0x1404dc5ca 7535 jne 0x1404dc601
0x1404dc5cc 0fb685380c0000 movzx eax, byte ptr [rbp + 0xc38]
0x1404dc5d3 eb0f jmp 0x1404dc5e4
0x1404dc5d5 0fb685380c0000 movzx eax, byte ptr [rbp + 0xc38]
0x1404dc5dc 8d48e5 lea ecx, [rax - 0x1b]
0x1404dc5df 80f905 cmp cl, 5
0x1404dc5e2 721d jb 0x1404dc601
0x1404dc5e4 3c18 cmp al, 0x18
0x1404dc5e6 7519 jne 0x1404dc601
0x1404dc5e8 488b85400c0000 mov rax, qword ptr [rbp + 0xc40]
0x1404dc5ef f048ff08 lock dec qword ptr [rax]
0x1404dc5f3 750c jne 0x1404dc601
0x1404dc5f5 488d8d400c0000 lea rcx, [rbp + 0xc40]
0x1404dc5fc e82f251700 call 0x14064eb30 ; alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global>
0x1404dc601 0fb685900c0000 movzx eax, byte ptr [rbp + 0xc90]
0x1404dc608 3c1b cmp al, 0x1b
0x1404dc60a 0f95c1 setne cl
0x1404dc60d 848d98140000 test byte ptr [rbp + 0x1498], cl
0x1404dc613 741d je 0x1404dc632
0x1404dc615 3c18 cmp al, 0x18
0x1404dc617 7519 jne 0x1404dc632
0x1404dc619 488b85980c0000 mov rax, qword ptr [rbp + 0xc98]
0x1404dc620 f048ff08 lock dec qword ptr [rax]
0x1404dc624 750c jne 0x1404dc632
0x1404dc626 488d8d980c0000 lea rcx, [rbp + 0xc98]
0x1404dc62d e8fe241700 call 0x14064eb30 ; alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global>
0x1404dc632 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404dc639 c685a614000000 mov byte ptr [rbp + 0x14a6], 0
0x1404dc640 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dc647 488d8d500c0000 lea rcx, [rbp + 0xc50]
0x1404dc64e e8bdda1100 call 0x1405fa110 ; core::ptr::drop_in_place<winit::platform_impl::platform::KeyEventExtra>
0x1404dc653 e933e5ffff jmp 0x1404dab8b
0x1404dc658 4088b5a7140000 mov byte ptr [rbp + 0x14a7], sil
0x1404dc65f c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404dc666 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dc66d 488b8dd8130000 mov rcx, qword ptr [rbp + 0x13d8]
0x1404dc674 e8b7c33f02 call 0x1428d8a30 ; std::sys::sync::rwlock::futex::RwLock::read_contended
0x1404dc679 e9d8f7ffff jmp 0x1404dbe56
0x1404dc67e 83fb02 cmp ebx, 2
0x1404dc681 0f849c000000 je 0x1404dc723
0x1404dc687 83fb01 cmp ebx, 1
0x1404dc68a 0f85a1000000 jne 0x1404dc731
0x1404dc690 4989ce mov r14, rcx
0x1404dc693 488d8d30140000 lea rcx, [rbp + 0x1430]
0x1404dc69a 488d55c8 lea rdx, [rbp - 0x38]
0x1404dc69e e8cdb5ffff call 0x1404d7c70 ; bevy_input::keyboard::impl$32::clone
0x1404dc6a3 4c89f1 mov rcx, r14
0x1404dc6a6 e946fbffff jmp 0x1404dc1f1
0x1404dc6ab c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404dc6b2 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404dc6b9 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dc6c0 4c89f9 mov rcx, r15
0x1404dc6c3 e848c13f02 call 0x1428d8810 ; std::sys::sync::mutex::futex::Mutex::wake
0x1404dc6c8 e9dfe8ffff jmp 0x1404dafac
0x1404dc6cd c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404dc6d4 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404dc6db c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dc6e2 488d0da8841503 lea rcx, [rip + 0x31584a8]
0x1404dc6e9 e862c03f02 call 0x1428d8750 ; std::sys::sync::mutex::futex::Mutex::lock_contended
0x1404dc6ee e99cf5ffff jmp 0x1404dbc8f
0x1404dc6f3 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404dc6fa c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404dc701 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dc708 e8a3bf3f02 call 0x1428d86b0 ; std::panicking::panic_count::is_zero_slow_path
0x1404dc70d 3401 xor al, 1
0x1404dc70f 0fb60d7c841503 movzx ecx, byte ptr [rip + 0x315847c]
0x1404dc716 84c9 test cl, cl
0x1404dc718 0f8498f5ffff je 0x1404dbcb6
0x1404dc71e e9982d0000 jmp 0x1404df4bb
0x1404dc723 8b45c4 mov eax, dword ptr [rbp - 0x3c]
0x1404dc726 898598140000 mov dword ptr [rbp + 0x1498], eax
0x1404dc72c e9c0faffff jmp 0x1404dc1f1
0x1404dc731 0fb645c8 movzx eax, byte ptr [rbp - 0x38]
0x1404dc735 8d50e8 lea edx, [rax - 0x18]
0x1404dc738 80fa03 cmp dl, 3
0x1404dc73b 0fb6d2 movzx edx, dl
0x1404dc73e 41b801000000 mov r8d, 1
0x1404dc744 410f43d0 cmovae edx, r8d
0x1404dc748 84d2 test dl, dl
0x1404dc74a 0f84b5000000 je 0x1404dc805
0x1404dc750 0fb6d2 movzx edx, dl
0x1404dc753 83fa01 cmp edx, 1
0x1404dc756 0f85d5000000 jne 0x1404dc831
0x1404dc75c 4c8d4500 lea r8, [rbp]
0x1404dc760 498b50d8 mov rdx, qword ptr [r8 - 0x28]
0x1404dc764 4c8d8d31140000 lea r9, [rbp + 0x1431]
0x1404dc76b 4989510f mov qword ptr [r9 + 0xf], rdx
0x1404dc76f 66410f1040c9 movupd xmm0, xmmword ptr [r8 - 0x37]
0x1404dc775 66410f1101 movupd xmmword ptr [r9], xmm0
0x1404dc77a 888530140000 mov byte ptr [rbp + 0x1430], al
0x1404dc780 e96cfaffff jmp 0x1404dc1f1
0x1404dc785 4088b5a7140000 mov byte ptr [rbp + 0x14a7], sil
0x1404dc78c c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404dc793 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dc79a 488b8dd8130000 mov rcx, qword ptr [rbp + 0x13d8]
0x1404dc7a1 e89ac53f02 call 0x1428d8d40 ; std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
0x1404dc7a6 e943f8ffff jmp 0x1404dbfee
0x1404dc7ab c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404dc7b2 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404dc7b9 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dc7c0 e8ebbe3f02 call 0x1428d86b0 ; std::panicking::panic_count::is_zero_slow_path
0x1404dc7c5 84c0 test al, al
0x1404dc7c7 0f85d2e7ffff jne 0x1404daf9f
0x1404dc7cd 41c6470101 mov byte ptr [r15 + 1], 1
0x1404dc7d2 e9c8e7ffff jmp 0x1404daf9f
0x1404dc7d7 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404dc7de c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404dc7e5 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404dc7ec e8bfbe3f02 call 0x1428d86b0 ; std::panicking::panic_count::is_zero_slow_path
0x1404dc7f1 84c0 test al, al
0x1404dc7f3 0f85def4ffff jne 0x1404dbcd7
0x1404dc7f9 c6059283150301 mov byte ptr [rip + 0x3158392], 1
0x1404dc800 e9d2f4ffff jmp 0x1404dbcd7
0x1404dc805 488b45d0 mov rax, qword ptr [rbp - 0x30]
0x1404dc809 f048ff00 lock inc qword ptr [rax]
0x1404dc80d 0f8e472e0000 jle 0x1404df65a
0x1404dc813 488b55d8 mov rdx, qword ptr [rbp - 0x28]
0x1404dc817 48898538140000 mov qword ptr [rbp + 0x1438], rax
0x1404dc81e 48899540140000 mov qword ptr [rbp + 0x1440], rdx
0x1404dc825 c6853014000018 mov byte ptr [rbp + 0x1430], 0x18
0x1404dc82c e9c0f9ffff jmp 0x1404dc1f1
0x1404dc831 660f1045d0 movupd xmm0, xmmword ptr [rbp - 0x30]
0x1404dc836 660f118538140000 movupd xmmword ptr [rbp + 0x1438], xmm0
0x1404dc83e c685301400001a mov byte ptr [rbp + 0x1430], 0x1a
0x1404dc845 e9a7f9ffff jmp 0x1404dc1f1
0x1404dc84a 660f1f440000 nop word ptr [rax + rax]
0x1404dc850 488d8de0130000 lea rcx, [rbp + 0x13e0]
0x1404dc857 e8b4c91200 call 0x140609210 ; core::ptr::drop_in_place<alloc::vec::drain::Drain<enum2$<tiny_glade::event_loop::OsWindowEvent>,alloc::alloc::Global> >
0x1404dc85c 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dc863 48ba8b047be66126c0a4 movabs rdx, 0xa4c02661e67b048b
0x1404dc86d 49b8ec672aaae661e7ea movabs r8, 0xeae761e6aa2a67ec
0x1404dc877 e8740ec0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404dc87c 4885c0 test rax, rax
0x1404dc87f 4c8b8578140000 mov r8, qword ptr [rbp + 0x1478]
0x1404dc886 0f84e8280000 je 0x1404df174
0x1404dc88c 49399048020000 cmp qword ptr [r8 + 0x248], rdx
0x1404dc893 0f86db280000 jbe 0x1404df174
0x1404dc899 498b8040020000 mov rax, qword ptr [r8 + 0x240]
0x1404dc8a0 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404dc8a4 4885c9 test rcx, rcx
0x1404dc8a7 0f84c7280000 je 0x1404df174
0x1404dc8ad 498b8010020000 mov rax, qword ptr [r8 + 0x210]
0x1404dc8b4 48f7d1 not rcx
0x1404dc8b7 488d0c49 lea rcx, [rcx + rcx*2]
0x1404dc8bb 48c1e105 shl rcx, 5
0x1404dc8bf 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dc8c5 0f84a9280000 je 0x1404df174
0x1404dc8cb 4801c8 add rax, rcx
0x1404dc8ce 488b4010 mov rax, qword ptr [rax + 0x10]
0x1404dc8d2 488b00 mov rax, qword ptr [rax]
0x1404dc8d5 48898598140000 mov qword ptr [rbp + 0x1498], rax
0x1404dc8dc 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dc8e3 48ba80eebc5f6a383533 movabs rdx, 0x3335386a5fbcee80
0x1404dc8ed 49b860ec651af3592a8f movabs r8, 0x8f2a59f31a65ec60
0x1404dc8f7 e8f40dc0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404dc8fc 4885c0 test rax, rax
0x1404dc8ff 4c8b8578140000 mov r8, qword ptr [rbp + 0x1478]
0x1404dc906 0f8457280000 je 0x1404df163
0x1404dc90c 49399048020000 cmp qword ptr [r8 + 0x248], rdx
0x1404dc913 0f864a280000 jbe 0x1404df163
0x1404dc919 498b8040020000 mov rax, qword ptr [r8 + 0x240]
0x1404dc920 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404dc924 4885c9 test rcx, rcx
0x1404dc927 0f8436280000 je 0x1404df163
0x1404dc92d 498b8010020000 mov rax, qword ptr [r8 + 0x210]
0x1404dc934 48f7d1 not rcx
0x1404dc937 488d0c49 lea rcx, [rcx + rcx*2]
0x1404dc93b 48c1e105 shl rcx, 5
0x1404dc93f 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dc945 0f8418280000 je 0x1404df163
0x1404dc94b 4c8b8598140000 mov r8, qword ptr [rbp + 0x1498]
0x1404dc952 4585c0 test r8d, r8d
0x1404dc955 0f95c2 setne dl
0x1404dc958 49c1e820 shr r8, 0x20
0x1404dc95c 410f95c0 setne r8b
0x1404dc960 4184d0 test r8b, dl
0x1404dc963 751b jne 0x1404dc980
0x1404dc965 31c9 xor ecx, ecx
0x1404dc967 ba40420f00 mov edx, 0xf4240
0x1404dc96c e80f908700 call 0x140d55980 ; std::thread::sleep
0x1404dc971 e9d71e0000 jmp 0x1404de84d
0x1404dc976 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404dc980 4801c8 add rax, rcx
0x1404dc983 488b4010 mov rax, qword ptr [rax + 0x10]
0x1404dc987 8a5828 mov bl, byte ptr [rax + 0x28]
0x1404dc98a 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dc991 48bacfde921b5889d12e movabs rdx, 0x2ed189581b92decf
0x1404dc99b 49b8385f069693d9d315 movabs r8, 0x15d3d99396065f38
0x1404dc9a5 e8460dc0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404dc9aa 4885c0 test rax, rax
0x1404dc9ad 4c8b8578140000 mov r8, qword ptr [rbp + 0x1478]
0x1404dc9b4 0f845c290000 je 0x1404df316
0x1404dc9ba 49399048020000 cmp qword ptr [r8 + 0x248], rdx
0x1404dc9c1 0f864f290000 jbe 0x1404df316
0x1404dc9c7 498b8040020000 mov rax, qword ptr [r8 + 0x240]
0x1404dc9ce 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404dc9d2 4885c9 test rcx, rcx
0x1404dc9d5 0f843b290000 je 0x1404df316
0x1404dc9db 498b8010020000 mov rax, qword ptr [r8 + 0x210]
0x1404dc9e2 48f7d1 not rcx
0x1404dc9e5 488d0c49 lea rcx, [rcx + rcx*2]
0x1404dc9e9 48c1e105 shl rcx, 5
0x1404dc9ed 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dc9f3 0f841d290000 je 0x1404df316
0x1404dc9f9 4801c8 add rax, rcx
0x1404dc9fc 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1404dca00 e89bb24a00 call 0x140987ca0 ; country_core::render::impl$0::deref
0x1404dca05 4889c1 mov rcx, rax
0x1404dca08 e873c57100 call 0x140bf8f80 ; rhapsody::raw::device::GraphicsDevice::refresh_memory_budget
0x1404dca0d 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dca14 48bacfde921b5889d12e movabs rdx, 0x2ed189581b92decf
0x1404dca1e 49b8385f069693d9d315 movabs r8, 0x15d3d99396065f38
0x1404dca28 e8c30cc0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404dca2d 4885c0 test rax, rax
0x1404dca30 4c8b8578140000 mov r8, qword ptr [rbp + 0x1478]
0x1404dca37 0f84ea280000 je 0x1404df327
0x1404dca3d 49399048020000 cmp qword ptr [r8 + 0x248], rdx
0x1404dca44 0f86dd280000 jbe 0x1404df327
0x1404dca4a 498b8040020000 mov rax, qword ptr [r8 + 0x240]
0x1404dca51 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404dca55 4885c9 test rcx, rcx
0x1404dca58 0f84c9280000 je 0x1404df327
0x1404dca5e 498b8010020000 mov rax, qword ptr [r8 + 0x210]
0x1404dca65 48f7d1 not rcx
0x1404dca68 488d0c49 lea rcx, [rcx + rcx*2]
0x1404dca6c 48c1e105 shl rcx, 5
0x1404dca70 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dca76 0f84ab280000 je 0x1404df327
0x1404dca7c 4801c8 add rax, rcx
0x1404dca7f 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1404dca83 e818b24a00 call 0x140987ca0 ; country_core::render::impl$0::deref
0x1404dca88 4889c7 mov rdi, rax
0x1404dca8b f30f1035ed8e4402 movss xmm6, dword ptr [rip + 0x2448eed]
0x1404dca93 84db test bl, bl
0x1404dca95 7413 je 0x1404dcaaa
0x1404dca97 4889f9 mov rcx, rdi
0x1404dca9a 488b9598140000 mov rdx, qword ptr [rbp + 0x1498]
0x1404dcaa1 e87ab40100 call 0x1404f7f20 ; tiny_glade::vram::calculate_max_resolution_scale
0x1404dcaa6 660f28f0 movapd xmm6, xmm0
0x1404dcaaa 488b8d20140000 mov rcx, qword ptr [rbp + 0x1420]
0x1404dcab1 4889fa mov rdx, rdi
0x1404dcab4 e8d71f7300 call 0x140c0ea90 ; rhapsody::raw::swapchain::Swapchain::extent
0x1404dcab9 48898568140000 mov qword ptr [rbp + 0x1468], rax
0x1404dcac0 488b8568140000 mov rax, qword ptr [rbp + 0x1468]
0x1404dcac7 483b8598140000 cmp rax, qword ptr [rbp + 0x1498]
0x1404dcace 7416 je 0x1404dcae6
0x1404dcad0 488b8d20140000 mov rcx, qword ptr [rbp + 0x1420]
0x1404dcad7 4889fa mov rdx, rdi
0x1404dcada 4c8b8598140000 mov r8, qword ptr [rbp + 0x1498]
0x1404dcae1 e80a1d7300 call 0x140c0e7f0 ; rhapsody::raw::swapchain::Swapchain::resize
0x1404dcae6 f30f11b558140000 movss dword ptr [rbp + 0x1458], xmm6
0x1404dcaee 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dcaf5 48ba80eebc5f6a383533 movabs rdx, 0x3335386a5fbcee80
0x1404dcaff 49b860ec651af3592a8f movabs r8, 0x8f2a59f31a65ec60
0x1404dcb09 e8e20bc0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404dcb0e 4885c0 test rax, rax
0x1404dcb11 4c8b8578140000 mov r8, qword ptr [rbp + 0x1478]
0x1404dcb18 0f841a280000 je 0x1404df338
0x1404dcb1e 49399048020000 cmp qword ptr [r8 + 0x248], rdx
0x1404dcb25 0f860d280000 jbe 0x1404df338
0x1404dcb2b 498b8040020000 mov rax, qword ptr [r8 + 0x240]
0x1404dcb32 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404dcb36 4885c9 test rcx, rcx
0x1404dcb39 0f84f9270000 je 0x1404df338
0x1404dcb3f 498b8010020000 mov rax, qword ptr [r8 + 0x210]
0x1404dcb46 48f7d1 not rcx
0x1404dcb49 488d0c49 lea rcx, [rcx + rcx*2]
0x1404dcb4d 48c1e105 shl rcx, 5
0x1404dcb51 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dcb57 0f84db270000 je 0x1404df338
0x1404dcb5d 4801c8 add rax, rcx
0x1404dcb60 488b4010 mov rax, qword ptr [rax + 0x10]
0x1404dcb64 4c8b30 mov r14, qword ptr [rax]
0x1404dcb67 488b4808 mov rcx, qword ptr [rax + 8]
0x1404dcb6b 48898d28140000 mov qword ptr [rbp + 0x1428], rcx
0x1404dcb72 448b7810 mov r15d, dword ptr [rax + 0x10]
0x1404dcb76 4c8b6818 mov r13, qword ptr [rax + 0x18]
0x1404dcb7a 8b7820 mov edi, dword ptr [rax + 0x20]
0x1404dcb7d 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dcb84 48ba407cd487e7567b3d movabs rdx, 0x3d7b56e787d47c40
0x1404dcb8e 49b8c7bf438d0ab1b067 movabs r8, 0x67b0b10a8d43bfc7
0x1404dcb98 e8530bc0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404dcb9d 4885c0 test rax, rax
0x1404dcba0 4c8b8578140000 mov r8, qword ptr [rbp + 0x1478]
0x1404dcba7 0f849c270000 je 0x1404df349
0x1404dcbad 49399048020000 cmp qword ptr [r8 + 0x248], rdx
0x1404dcbb4 0f868f270000 jbe 0x1404df349
0x1404dcbba 498b8040020000 mov rax, qword ptr [r8 + 0x240]
0x1404dcbc1 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404dcbc5 4885c9 test rcx, rcx
0x1404dcbc8 0f847b270000 je 0x1404df349
0x1404dcbce 498b8010020000 mov rax, qword ptr [r8 + 0x210]
0x1404dcbd5 48f7d1 not rcx
0x1404dcbd8 488d0c49 lea rcx, [rcx + rcx*2]
0x1404dcbdc 48c1e105 shl rcx, 5
0x1404dcbe0 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dcbe6 0f845d270000 je 0x1404df349
0x1404dcbec 4801c8 add rax, rcx
0x1404dcbef 488b5010 mov rdx, qword ptr [rax + 0x10]
0x1404dcbf3 488d8d300c0000 lea rcx, [rbp + 0xc30]
0x1404dcbfa e851b1ffff call 0x1404d7d50 ; country_core::render::render_config::impl$38::clone
0x1404dcbff 80bdb80c000000 cmp byte ptr [rbp + 0xcb8], 0
0x1404dcc06 7518 jne 0x1404dcc20
0x1404dcc08 f30f1005f48c4402 movss xmm0, dword ptr [rip + 0x2448cf4]
0x1404dcc10 eb16 jmp 0x1404dcc28
0x1404dcc12 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404dcc20 f30f1085bc0c0000 movss xmm0, dword ptr [rbp + 0xcbc]
0x1404dcc28 f30f118530140000 movss dword ptr [rbp + 0x1430], xmm0
0x1404dcc30 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dcc37 48ba8b047be66126c0a4 movabs rdx, 0xa4c02661e67b048b
0x1404dcc41 49b8ec672aaae661e7ea movabs r8, 0xeae761e6aa2a67ec
0x1404dcc4b e8a00ac0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404dcc50 4885c0 test rax, rax
0x1404dcc53 488b8d78140000 mov rcx, qword ptr [rbp + 0x1478]
0x1404dcc5a 0f84fa260000 je 0x1404df35a
0x1404dcc60 48399148020000 cmp qword ptr [rcx + 0x248], rdx
0x1404dcc67 0f86ed260000 jbe 0x1404df35a
0x1404dcc6d 488b8140020000 mov rax, qword ptr [rcx + 0x240]
0x1404dcc74 488b04d0 mov rax, qword ptr [rax + rdx*8]
0x1404dcc78 4885c0 test rax, rax
0x1404dcc7b 0f84d9260000 je 0x1404df35a
0x1404dcc81 4c8ba110020000 mov r12, qword ptr [rcx + 0x210]
0x1404dcc88 48f7d0 not rax
0x1404dcc8b 488d0440 lea rax, [rax + rax*2]
0x1404dcc8f 48c1e005 shl rax, 5
0x1404dcc93 49837c042000 cmp qword ptr [r12 + rax + 0x20], 0
0x1404dcc99 0f84bb260000 je 0x1404df35a
0x1404dcc9f 4901c4 add r12, rax
0x1404dcca2 498b442410 mov rax, qword ptr [r12 + 0x10]
0x1404dcca7 8b89e0050000 mov ecx, dword ptr [rcx + 0x5e0]
0x1404dccad 898d48140000 mov dword ptr [rbp + 0x1448], ecx
0x1404dccb3 488d4818 lea rcx, [rax + 0x18]
0x1404dccb7 48898d60140000 mov qword ptr [rbp + 0x1460], rcx
0x1404dccbe 48898528130000 mov qword ptr [rbp + 0x1328], rax
0x1404dccc5 f30f104018 movss xmm0, dword ptr [rax + 0x18]
0x1404dccca f30f10b558140000 movss xmm6, dword ptr [rbp + 0x1458]
0x1404dccd2 0f2ec6 ucomiss xmm0, xmm6
0x1404dccd5 0f86e5010000 jbe 0x1404dcec0
0x1404dccdb 488b05c6f61403 mov rax, qword ptr [rip + 0x314f6c6]
0x1404dcce2 488b00 mov rax, qword ptr [rax]
0x1404dcce5 4883f803 cmp rax, 3
0x1404dcce9 0f8271010000 jb 0x1404dce60
0x1404dccef 488b8560140000 mov rax, qword ptr [rbp + 0x1460]
0x1404dccf6 488985e0130000 mov qword ptr [rbp + 0x13e0], rax
0x1404dccfd 488d0d0cfc8a00 lea rcx, [rip + 0x8afc0c]
0x1404dcd04 48898de8130000 mov qword ptr [rbp + 0x13e8], rcx
0x1404dcd0b 488d8558140000 lea rax, [rbp + 0x1458]
0x1404dcd12 488985f0130000 mov qword ptr [rbp + 0x13f0], rax
0x1404dcd19 48898df8130000 mov qword ptr [rbp + 0x13f8], rcx
0x1404dcd20 48c745c000000000 mov qword ptr [rbp - 0x40], 0
0x1404dcd28 48c745c801000000 mov qword ptr [rbp - 0x38], 1
0x1404dcd30 48c745d000000000 mov qword ptr [rbp - 0x30], 0
0x1404dcd38 48c745d801000000 mov qword ptr [rbp - 0x28], 1
0x1404dcd40 48c745e000000000 mov qword ptr [rbp - 0x20], 0
0x1404dcd48 48c745e820000000 mov qword ptr [rbp - 0x18], 0x20
0x1404dcd50 c645f003 mov byte ptr [rbp - 0x10], 3
0x1404dcd54 48c745f800000000 mov qword ptr [rbp - 8], 0
0x1404dcd5c 48c7450001000000 mov qword ptr [rbp], 1
0x1404dcd64 48c7450800000000 mov qword ptr [rbp + 8], 0
0x1404dcd6c 48c7451001000000 mov qword ptr [rbp + 0x10], 1
0x1404dcd74 48c7451801000000 mov qword ptr [rbp + 0x18], 1
0x1404dcd7c 48c7452020000000 mov qword ptr [rbp + 0x20], 0x20
0x1404dcd84 c6452803 mov byte ptr [rbp + 0x28], 3
0x1404dcd88 488d0df1d45002 lea rcx, [rip + 0x250d4f1]
0x1404dcd8f e8ec0f1a00 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x1404dcd94 0f1000 movups xmm0, xmmword ptr [rax]
0x1404dcd97 8b4010 mov eax, dword ptr [rax + 0x10]
0x1404dcd9a 48c7853006000003000000 mov qword ptr [rbp + 0x630], 3
0x1404dcda5 488d1536cf5002 lea rdx, [rip + 0x250cf36]
0x1404dcdac 48899538060000 mov qword ptr [rbp + 0x638], rdx
0x1404dcdb3 48c7854006000015000000 mov qword ptr [rbp + 0x640], 0x15
0x1404dcdbe 488d0d9bd45002 lea rcx, [rip + 0x250d49b]
0x1404dcdc5 48898d50060000 mov qword ptr [rbp + 0x650], rcx
0x1404dcdcc 48c7855806000002000000 mov qword ptr [rbp + 0x658], 2
0x1404dcdd7 488d8de0130000 lea rcx, [rbp + 0x13e0]
0x1404dcdde 48898d60060000 mov qword ptr [rbp + 0x660], rcx
0x1404dcde5 48c7856806000002000000 mov qword ptr [rbp + 0x668], 2
0x1404dcdf0 488d4dc0 lea rcx, [rbp - 0x40]
0x1404dcdf4 48898d70060000 mov qword ptr [rbp + 0x670], rcx
0x1404dcdfb 48c7857806000002000000 mov qword ptr [rbp + 0x678], 2
0x1404dce06 48c7850006000000000000 mov qword ptr [rbp + 0x600], 0
0x1404dce11 48899508060000 mov qword ptr [rbp + 0x608], rdx
0x1404dce18 48c7851006000015000000 mov qword ptr [rbp + 0x610], 0x15
0x1404dce23 48c7851806000000000000 mov qword ptr [rbp + 0x618], 0
0x1404dce2e 0f118520060000 movups xmmword ptr [rbp + 0x620], xmm0
0x1404dce35 c7854806000001000000 mov dword ptr [rbp + 0x648], 1
0x1404dce3f 89854c060000 mov dword ptr [rbp + 0x64c], eax
0x1404dce45 488d8d77140000 lea rcx, [rbp + 0x1477]
0x1404dce4c 488d9500060000 lea rdx, [rbp + 0x600]
0x1404dce53 e8d8d48600 call 0x140d4a330 ; log::__private_api::impl$2::log
0x1404dce58 f30f10b558140000 movss xmm6, dword ptr [rbp + 0x1458]
0x1404dce60 8b8548140000 mov eax, dword ptr [rbp + 0x1448]
0x1404dce66 418944245c mov dword ptr [r12 + 0x5c], eax
0x1404dce6b 488b8560140000 mov rax, qword ptr [rbp + 0x1460]
0x1404dce72 f30f1130 movss dword ptr [rax], xmm6
0x1404dce76 488d0d1bd65002 lea rcx, [rip + 0x250d61b]
0x1404dce7d 4a6304b1 movsxd rax, dword ptr [rcx + r14*4]
0x1404dce81 4801c8 add rax, rcx
0x1404dce84 ffe0 jmp rax
0x1404dce86 e855678800 call 0x140d635e0 ; std::time::Instant::now
0x1404dce8b 4889c1 mov rcx, rax
0x1404dce8e 4531c0 xor r8d, r8d
0x1404dce91 41b980b2e60e mov r9d, 0xee6b280
0x1404dce97 e8846b8800 call 0x140d63a20 ; std::time::impl$1::add
0x1404dce9c 4989c5 mov r13, rax
0x1404dce9f 89d7 mov edi, edx
0x1404dcea1 41be01000000 mov r14d, 1
0x1404dcea7 41bf80b2e60e mov r15d, 0xee6b280
0x1404dcead 31f6 xor esi, esi
0x1404dceaf e93c030000 jmp 0x1404dd1f0
0x1404dceb4 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404dcec0 e81b678800 call 0x140d635e0 ; std::time::Instant::now
0x1404dcec5 4889c3 mov rbx, rax
0x1404dcec8 488d0db9d55002 lea rcx, [rip + 0x250d5b9]
0x1404dcecf 4a6304b1 movsxd rax, dword ptr [rcx + r14*4]
0x1404dced3 4801c8 add rax, rcx
0x1404dced6 ffe0 jmp rax
0x1404dced8 31c0 xor eax, eax
0x1404dceda 4c39eb cmp rbx, r13
0x1404dcedd 0f95c0 setne al
0x1404dcee0 b9ff000000 mov ecx, 0xff
0x1404dcee5 0f42c1 cmovb eax, ecx
0x1404dcee8 39fa cmp edx, edi
0x1404dceea b900000000 mov ecx, 0
0x1404dceef 19c9 sbb ecx, ecx
0x1404dcef1 84c0 test al, al
0x1404dcef3 0f45c8 cmovne ecx, eax
0x1404dcef6 41be01000000 mov r14d, 1
0x1404dcefc 80f901 cmp cl, 1
0x1404dceff 7711 ja 0x1404dcf12
0x1404dcf01 f30f108530140000 movss xmm0, dword ptr [rbp + 0x1430]
0x1404dcf09 0f2ef0 ucomiss xmm6, xmm0
0x1404dcf0c 0f83d2000000 jae 0x1404dcfe4
0x1404dcf12 488bb528140000 mov rsi, qword ptr [rbp + 0x1428]
0x1404dcf19 e9d2020000 jmp 0x1404dd1f0
0x1404dcf1e e8bd668800 call 0x140d635e0 ; std::time::Instant::now
0x1404dcf23 4889c1 mov rcx, rax
0x1404dcf26 4c8b8528140000 mov r8, qword ptr [rbp + 0x1428]
0x1404dcf2d 4589f9 mov r9d, r15d
0x1404dcf30 e8eb6a8800 call 0x140d63a20 ; std::time::impl$1::add
0x1404dcf35 4989c5 mov r13, rax
0x1404dcf38 89d7 mov edi, edx
0x1404dcf3a 41be01000000 mov r14d, 1
0x1404dcf40 488bb528140000 mov rsi, qword ptr [rbp + 0x1428]
0x1404dcf47 e9a4020000 jmp 0x1404dd1f0
0x1404dcf4c 4883bd2814000000 cmp qword ptr [rbp + 0x1428], 0
0x1404dcf54 0f88b71f0000 js 0x1404def11
0x1404dcf5a 438d043f lea eax, [r15 + r15]
0x1404dcf5e 31f6 xor esi, esi
0x1404dcf60 4181ff0065cd1d cmp r15d, 0x1dcd6500
0x1404dcf67 478dbc3f003665c4 lea r15d, [r15 + r15 - 0x3b9aca00]
0x1404dcf6f 440f42f8 cmovb r15d, eax
0x1404dcf73 0f93c3 setae bl
0x1404dcf76 e865668800 call 0x140d635e0 ; std::time::Instant::now
0x1404dcf7b 4088de mov sil, bl
0x1404dcf7e 488b8d28140000 mov rcx, qword ptr [rbp + 0x1428]
0x1404dcf85 488d344e lea rsi, [rsi + rcx*2]
0x1404dcf89 4889c1 mov rcx, rax
0x1404dcf8c 4989f0 mov r8, rsi
0x1404dcf8f 4589f9 mov r9d, r15d
0x1404dcf92 e8896a8800 call 0x140d63a20 ; std::time::impl$1::add
0x1404dcf97 4989c5 mov r13, rax
0x1404dcf9a 89d7 mov edi, edx
0x1404dcf9c 41be01000000 mov r14d, 1
0x1404dcfa2 e949020000 jmp 0x1404dd1f0
0x1404dcfa7 31c0 xor eax, eax
0x1404dcfa9 4c39eb cmp rbx, r13
0x1404dcfac 0f95c0 setne al
0x1404dcfaf b9ff000000 mov ecx, 0xff
0x1404dcfb4 0f42c1 cmovb eax, ecx
0x1404dcfb7 39fa cmp edx, edi
0x1404dcfb9 b900000000 mov ecx, 0
0x1404dcfbe 19c9 sbb ecx, ecx
0x1404dcfc0 84c0 test al, al
0x1404dcfc2 0f45c8 cmovne ecx, eax
0x1404dcfc5 80f902 cmp cl, 2
0x1404dcfc8 7308 jae 0x1404dcfd2
0x1404dcfca 4531f6 xor r14d, r14d
0x1404dcfcd e91e020000 jmp 0x1404dd1f0
0x1404dcfd2 41be03000000 mov r14d, 3
0x1404dcfd8 488bb528140000 mov rsi, qword ptr [rbp + 0x1428]
0x1404dcfdf e90c020000 jmp 0x1404dd1f0
0x1404dcfe4 89d7 mov edi, edx
0x1404dcfe6 488b05bbf31403 mov rax, qword ptr [rip + 0x314f3bb]
0x1404dcfed 488b00 mov rax, qword ptr [rax]
0x1404dcff0 4883f803 cmp rax, 3
0x1404dcff4 0f8271010000 jb 0x1404dd16b
0x1404dcffa 488b8560140000 mov rax, qword ptr [rbp + 0x1460]
0x1404dd001 488985e0130000 mov qword ptr [rbp + 0x13e0], rax
0x1404dd008 488d0501f98a00 lea rax, [rip + 0x8af901]
0x1404dd00f 488985e8130000 mov qword ptr [rbp + 0x13e8], rax
0x1404dd016 488d8d30140000 lea rcx, [rbp + 0x1430]
0x1404dd01d 48898df0130000 mov qword ptr [rbp + 0x13f0], rcx
0x1404dd024 488985f8130000 mov qword ptr [rbp + 0x13f8], rax
0x1404dd02b 48c745c000000000 mov qword ptr [rbp - 0x40], 0
0x1404dd033 48c745c801000000 mov qword ptr [rbp - 0x38], 1
0x1404dd03b 48c745d000000000 mov qword ptr [rbp - 0x30], 0
0x1404dd043 48c745d801000000 mov qword ptr [rbp - 0x28], 1
0x1404dd04b 48c745e000000000 mov qword ptr [rbp - 0x20], 0
0x1404dd053 48c745e820000000 mov qword ptr [rbp - 0x18], 0x20
0x1404dd05b c645f003 mov byte ptr [rbp - 0x10], 3
0x1404dd05f 48c745f800000000 mov qword ptr [rbp - 8], 0
0x1404dd067 48c7450001000000 mov qword ptr [rbp], 1
0x1404dd06f 48c7450800000000 mov qword ptr [rbp + 8], 0
0x1404dd077 48c7451001000000 mov qword ptr [rbp + 0x10], 1
0x1404dd07f 48c7451801000000 mov qword ptr [rbp + 0x18], 1
0x1404dd087 48c7452020000000 mov qword ptr [rbp + 0x20], 0x20
0x1404dd08f c6452803 mov byte ptr [rbp + 0x28], 3
0x1404dd093 488d0d76d15002 lea rcx, [rip + 0x250d176]
0x1404dd09a e8e10c1a00 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x1404dd09f 0f1000 movups xmm0, xmmword ptr [rax]
0x1404dd0a2 8b4010 mov eax, dword ptr [rax + 0x10]
0x1404dd0a5 48c7853006000003000000 mov qword ptr [rbp + 0x630], 3
0x1404dd0b0 488d152bcc5002 lea rdx, [rip + 0x250cc2b]
0x1404dd0b7 48899538060000 mov qword ptr [rbp + 0x638], rdx
0x1404dd0be 48c7854006000015000000 mov qword ptr [rbp + 0x640], 0x15
0x1404dd0c9 488d0d20d15002 lea rcx, [rip + 0x250d120]
0x1404dd0d0 48898d50060000 mov qword ptr [rbp + 0x650], rcx
0x1404dd0d7 48c7855806000002000000 mov qword ptr [rbp + 0x658], 2
0x1404dd0e2 488d8de0130000 lea rcx, [rbp + 0x13e0]
0x1404dd0e9 48898d60060000 mov qword ptr [rbp + 0x660], rcx
0x1404dd0f0 48c7856806000002000000 mov qword ptr [rbp + 0x668], 2
0x1404dd0fb 488d4dc0 lea rcx, [rbp - 0x40]
0x1404dd0ff 48898d70060000 mov qword ptr [rbp + 0x670], rcx
0x1404dd106 48c7857806000002000000 mov qword ptr [rbp + 0x678], 2
0x1404dd111 48c7850006000000000000 mov qword ptr [rbp + 0x600], 0
0x1404dd11c 48899508060000 mov qword ptr [rbp + 0x608], rdx
0x1404dd123 48c7851006000015000000 mov qword ptr [rbp + 0x610], 0x15
0x1404dd12e 48c7851806000000000000 mov qword ptr [rbp + 0x618], 0
0x1404dd139 0f118520060000 movups xmmword ptr [rbp + 0x620], xmm0
0x1404dd140 c7854806000001000000 mov dword ptr [rbp + 0x648], 1
0x1404dd14a 89854c060000 mov dword ptr [rbp + 0x64c], eax
0x1404dd150 488d8d77140000 lea rcx, [rbp + 0x1477]
0x1404dd157 488d9500060000 lea rdx, [rbp + 0x600]
0x1404dd15e e8cdd18600 call 0x140d4a330 ; log::__private_api::impl$2::log
0x1404dd163 f30f108530140000 movss xmm0, dword ptr [rbp + 0x1430]
0x1404dd16b 8b8548140000 mov eax, dword ptr [rbp + 0x1448]
0x1404dd171 418944245c mov dword ptr [r12 + 0x5c], eax
0x1404dd176 488b8560140000 mov rax, qword ptr [rbp + 0x1460]
0x1404dd17d f30f1100 movss dword ptr [rax], xmm0
0x1404dd181 488b9528140000 mov rdx, qword ptr [rbp + 0x1428]
0x1404dd188 4885d2 test rdx, rdx
0x1404dd18b 0f88171e0000 js 0x1404defa8
0x1404dd191 438d043f lea eax, [r15 + r15]
0x1404dd195 31c9 xor ecx, ecx
0x1404dd197 4181ff0065cd1d cmp r15d, 0x1dcd6500
0x1404dd19e 478dbc3f003665c4 lea r15d, [r15 + r15 - 0x3b9aca00]
0x1404dd1a6 440f42f8 cmovb r15d, eax
0x1404dd1aa 0f93c1 setae cl
0x1404dd1ad 488d3451 lea rsi, [rcx + rdx*2]
0x1404dd1b1 4885d2 test rdx, rdx
0x1404dd1b4 4589f9 mov r9d, r15d
0x1404dd1b7 b800000000 mov eax, 0
0x1404dd1bc 440f44c8 cmove r9d, eax
0x1404dd1c0 4883fe03 cmp rsi, 3
0x1404dd1c4 41b802000000 mov r8d, 2
0x1404dd1ca 4c0f43c6 cmovae r8, rsi
0x1404dd1ce 4889d9 mov rcx, rbx
0x1404dd1d1 89fa mov edx, edi
0x1404dd1d3 e848688800 call 0x140d63a20 ; std::time::impl$1::add
0x1404dd1d8 4989c5 mov r13, rax
0x1404dd1db 89d7 mov edi, edx
0x1404dd1dd 41be03000000 mov r14d, 3
0x1404dd1e3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404dd1f0 488b8560140000 mov rax, qword ptr [rbp + 0x1460]
0x1404dd1f7 f30f1030 movss xmm6, dword ptr [rax]
0x1404dd1fb 80bddb0c000001 cmp byte ptr [rbp + 0xcdb], 1
0x1404dd202 f30f1005fa864402 movss xmm0, dword ptr [rip + 0x24486fa]
0x1404dd20a 750c jne 0x1404dd218
0x1404dd20c 488d8df30c0000 lea rcx, [rbp + 0xcf3]
0x1404dd213 e898b94800 call 0x140968bb0 ; country_core::render::render_config::UpscalingPreset::tgsr_resolution_scale
0x1404dd218 448b9d48140000 mov r11d, dword ptr [rbp + 0x1448]
0x1404dd21f 45895c245c mov dword ptr [r12 + 0x5c], r11d
0x1404dd224 4c8b9528130000 mov r10, qword ptr [rbp + 0x1328]
0x1404dd22b f3410f11421c movss dword ptr [r10 + 0x1c], xmm0
0x1404dd231 f2410f1002 movsd xmm0, qword ptr [r10]
0x1404dd236 0f140563874402 unpcklps xmm0, xmmword ptr [rip + 0x2448763]
0x1404dd23d 66410f56c2 orpd xmm0, xmm10
0x1404dd242 66410f5cc2 subpd xmm0, xmm10
0x1404dd247 660f5ac0 cvtpd2ps xmm0, xmm0
0x1404dd24b f30f12f6 movsldup xmm6, xmm6
0x1404dd24f 0f59c6 mulps xmm0, xmm6
0x1404dd252 0f28c8 movaps xmm1, xmm0
0x1404dd255 0fc6c8ff shufps xmm1, xmm0, 0xff
0x1404dd259 f3480f2cc1 cvttss2si rax, xmm1
0x1404dd25e 410f2ecb ucomiss xmm1, xmm11
0x1404dd262 41b800000000 mov r8d, 0
0x1404dd268 410f42c0 cmovb eax, r8d
0x1404dd26c 410f2ecc ucomiss xmm1, xmm12
0x1404dd270 41b9ffffffff mov r9d, 0xffffffff
0x1404dd276 410f47c1 cmova eax, r9d
0x1404dd27a 0f28c8 movaps xmm1, xmm0
0x1404dd27d 660f15c8 unpckhpd xmm1, xmm0
0x1404dd281 f3480f2cc9 cvttss2si rcx, xmm1
0x1404dd286 410f2ecb ucomiss xmm1, xmm11
0x1404dd28a 410f42c8 cmovb ecx, r8d
0x1404dd28e 410f2ecc ucomiss xmm1, xmm12
0x1404dd292 660f6ec8 movd xmm1, eax
0x1404dd296 410f47c9 cmova ecx, r9d
0x1404dd29a f3480f2cc0 cvttss2si rax, xmm0
0x1404dd29f 410f2ec3 ucomiss xmm0, xmm11
0x1404dd2a3 410f42c0 cmovb eax, r8d
0x1404dd2a7 410f2ec4 ucomiss xmm0, xmm12
0x1404dd2ab 410f47c1 cmova eax, r9d
0x1404dd2af f30f16c0 movshdup xmm0, xmm0
0x1404dd2b3 f3480f2cd0 cvttss2si rdx, xmm0
0x1404dd2b8 410f2ec3 ucomiss xmm0, xmm11
0x1404dd2bc 410f42d0 cmovb edx, r8d
0x1404dd2c0 410f2ec4 ucomiss xmm0, xmm12
0x1404dd2c4 660f6ec1 movd xmm0, ecx
0x1404dd2c8 410f47d1 cmova edx, r9d
0x1404dd2cc 660f6ed0 movd xmm2, eax
0x1404dd2d0 660f62c1 punpckldq xmm0, xmm1
0x1404dd2d4 660f6eca movd xmm1, edx
0x1404dd2d8 660f62d1 punpckldq xmm2, xmm1
0x1404dd2dc 660f6cd0 punpcklqdq xmm2, xmm0
0x1404dd2e0 660f6fc2 movdqa xmm0, xmm2
0x1404dd2e4 66410fefc5 pxor xmm0, xmm13
0x1404dd2e9 660f6605cfc25002 pcmpgtd xmm0, xmmword ptr [rip + 0x250c2cf]
0x1404dd2f1 660f6fc8 movdqa xmm1, xmm0
0x1404dd2f5 660fdfca pandn xmm1, xmm2
0x1404dd2f9 660fdb05cfc25002 pand xmm0, xmmword ptr [rip + 0x250c2cf]
0x1404dd301 660febc1 por xmm0, xmm1
0x1404dd305 660f6fc8 movdqa xmm1, xmm0
0x1404dd309 66410fefcd pxor xmm1, xmm13
0x1404dd30e 660f660dcac25002 pcmpgtd xmm1, xmmword ptr [rip + 0x250c2ca]
0x1404dd316 660fdbc1 pand xmm0, xmm1
0x1404dd31a 660fdf0dcec25002 pandn xmm1, xmmword ptr [rip + 0x250c2ce]
0x1404dd322 660febc8 por xmm1, xmm0
0x1404dd326 45895c245c mov dword ptr [r12 + 0x5c], r11d
0x1404dd32b 66410fd64a08 movq qword ptr [r10 + 8], xmm1
0x1404dd331 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dd338 48ba80eebc5f6a383533 movabs rdx, 0x3335386a5fbcee80
0x1404dd342 49b860ec651af3592a8f movabs r8, 0x8f2a59f31a65ec60
0x1404dd34c e89f03c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404dd351 4885c0 test rax, rax
0x1404dd354 4c8b8578140000 mov r8, qword ptr [rbp + 0x1478]
0x1404dd35b 0f840a200000 je 0x1404df36b
0x1404dd361 49399048020000 cmp qword ptr [r8 + 0x248], rdx
0x1404dd368 0f86fd1f0000 jbe 0x1404df36b
0x1404dd36e 498b8040020000 mov rax, qword ptr [r8 + 0x240]
0x1404dd375 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404dd379 4885c9 test rcx, rcx
0x1404dd37c 0f84e91f0000 je 0x1404df36b
0x1404dd382 498b8010020000 mov rax, qword ptr [r8 + 0x210]
0x1404dd389 48f7d1 not rcx
0x1404dd38c 488d0c49 lea rcx, [rcx + rcx*2]
0x1404dd390 48c1e105 shl rcx, 5
0x1404dd394 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dd39a 0f84cb1f0000 je 0x1404df36b
0x1404dd3a0 4801c8 add rax, rcx
0x1404dd3a3 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1404dd3a7 418b90e0050000 mov edx, dword ptr [r8 + 0x5e0]
0x1404dd3ae 89505c mov dword ptr [rax + 0x5c], edx
0x1404dd3b1 4c8931 mov qword ptr [rcx], r14
0x1404dd3b4 48897108 mov qword ptr [rcx + 8], rsi
0x1404dd3b8 44897910 mov dword ptr [rcx + 0x10], r15d
0x1404dd3bc 4c896918 mov qword ptr [rcx + 0x18], r13
0x1404dd3c0 897920 mov dword ptr [rcx + 0x20], edi
0x1404dd3c3 488d8d300c0000 lea rcx, [rbp + 0xc30]
0x1404dd3ca e8b1071200 call 0x1405fdb80 ; core::ptr::drop_in_place<country_core::render::render_config::VideoSettings>
0x1404dd3cf 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dd3d6 48ba8b047be66126c0a4 movabs rdx, 0xa4c02661e67b048b
0x1404dd3e0 49b8ec672aaae661e7ea movabs r8, 0xeae761e6aa2a67ec
0x1404dd3ea e80103c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404dd3ef 4885c0 test rax, rax
0x1404dd3f2 4c8b8578140000 mov r8, qword ptr [rbp + 0x1478]
0x1404dd3f9 0f848e1f0000 je 0x1404df38d
0x1404dd3ff 49399048020000 cmp qword ptr [r8 + 0x248], rdx
0x1404dd406 0f86811f0000 jbe 0x1404df38d
0x1404dd40c 498b8040020000 mov rax, qword ptr [r8 + 0x240]
0x1404dd413 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404dd417 4885c9 test rcx, rcx
0x1404dd41a 0f846d1f0000 je 0x1404df38d
0x1404dd420 498b8010020000 mov rax, qword ptr [r8 + 0x210]
0x1404dd427 48f7d1 not rcx
0x1404dd42a 488d0c49 lea rcx, [rcx + rcx*2]
0x1404dd42e 48c1e105 shl rcx, 5
0x1404dd432 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dd438 0f844f1f0000 je 0x1404df38d
0x1404dd43e f20f108598140000 movsd xmm0, qword ptr [rbp + 0x1498]
0x1404dd446 4801c8 add rax, rcx
0x1404dd449 0f140550854402 unpcklps xmm0, xmmword ptr [rip + 0x2448550]
0x1404dd450 66410f56c2 orpd xmm0, xmm10
0x1404dd455 66410f5cc2 subpd xmm0, xmm10
0x1404dd45a 660f5ac0 cvtpd2ps xmm0, xmm0
0x1404dd45e 488b4010 mov rax, qword ptr [rax + 0x10]
0x1404dd462 0f59f0 mulps xmm6, xmm0
0x1404dd465 f30f10401c movss xmm0, dword ptr [rax + 0x1c]
0x1404dd46a f30f12c0 movsldup xmm0, xmm0
0x1404dd46e 0f59c6 mulps xmm0, xmm6
0x1404dd471 0f580588c15002 addps xmm0, xmmword ptr [rip + 0x250c188]
0x1404dd478 0f28c8 movaps xmm1, xmm0
0x1404dd47b 0fc6c8ff shufps xmm1, xmm0, 0xff
0x1404dd47f f3480f2cd1 cvttss2si rdx, xmm1
0x1404dd484 410f2ecb ucomiss xmm1, xmm11
0x1404dd488 b800000000 mov eax, 0
0x1404dd48d 0f42d0 cmovb edx, eax
0x1404dd490 410f2ecc ucomiss xmm1, xmm12
0x1404dd494 b9ffffffff mov ecx, 0xffffffff
0x1404dd499 0f47d1 cmova edx, ecx
0x1404dd49c 48899560140000 mov qword ptr [rbp + 0x1460], rdx
0x1404dd4a3 0f28c8 movaps xmm1, xmm0
0x1404dd4a6 660f15c8 unpckhpd xmm1, xmm0
0x1404dd4aa f34c0f2cf1 cvttss2si r14, xmm1
0x1404dd4af 410f2ecb ucomiss xmm1, xmm11
0x1404dd4b3 440f42f0 cmovb r14d, eax
0x1404dd4b7 410f2ecc ucomiss xmm1, xmm12
0x1404dd4bb 440f47f1 cmova r14d, ecx
0x1404dd4bf f34c0f2cf8 cvttss2si r15, xmm0
0x1404dd4c4 410f2ec3 ucomiss xmm0, xmm11
0x1404dd4c8 440f42f8 cmovb r15d, eax
0x1404dd4cc 410f2ec4 ucomiss xmm0, xmm12
0x1404dd4d0 440f47f9 cmova r15d, ecx
0x1404dd4d4 f30f16c0 movshdup xmm0, xmm0
0x1404dd4d8 f3480f2cf0 cvttss2si rsi, xmm0
0x1404dd4dd 410f2ec3 ucomiss xmm0, xmm11
0x1404dd4e1 0f42f0 cmovb esi, eax
0x1404dd4e4 410f2ec4 ucomiss xmm0, xmm12
0x1404dd4e8 0f47f1 cmova esi, ecx
0x1404dd4eb 0f28c6 movaps xmm0, xmm6
0x1404dd4ee 0fc6c6ff shufps xmm0, xmm6, 0xff
0x1404dd4f2 f3480f2cd8 cvttss2si rbx, xmm0
0x1404dd4f7 410f2ec3 ucomiss xmm0, xmm11
0x1404dd4fb 0f42d8 cmovb ebx, eax
0x1404dd4fe 410f2ec4 ucomiss xmm0, xmm12
0x1404dd502 0f47d9 cmova ebx, ecx
0x1404dd505 0f28c6 movaps xmm0, xmm6
0x1404dd508 660f15c6 unpckhpd xmm0, xmm6
0x1404dd50c f34c0f2ce0 cvttss2si r12, xmm0
0x1404dd511 410f2ec3 ucomiss xmm0, xmm11
0x1404dd515 440f42e0 cmovb r12d, eax
0x1404dd519 410f2ec4 ucomiss xmm0, xmm12
0x1404dd51d 440f47e1 cmova r12d, ecx
0x1404dd521 f34c0f2cee cvttss2si r13, xmm6
0x1404dd526 410f2ef3 ucomiss xmm6, xmm11
0x1404dd52a 440f42e8 cmovb r13d, eax
0x1404dd52e 410f2ef4 ucomiss xmm6, xmm12
0x1404dd532 440f47e9 cmova r13d, ecx
0x1404dd536 f30f16c6 movshdup xmm0, xmm6
0x1404dd53a f3480f2cf8 cvttss2si rdi, xmm0
0x1404dd53f 410f2ec3 ucomiss xmm0, xmm11
0x1404dd543 0f42f8 cmovb edi, eax
0x1404dd546 410f2ec4 ucomiss xmm0, xmm12
0x1404dd54a 0f47f9 cmova edi, ecx
0x1404dd54d 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dd554 48ba8b047be66126c0a4 movabs rdx, 0xa4c02661e67b048b
0x1404dd55e 49b8ec672aaae661e7ea movabs r8, 0xeae761e6aa2a67ec
0x1404dd568 e88301c0ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404dd56d 4885c0 test rax, rax
0x1404dd570 4c8b8578140000 mov r8, qword ptr [rbp + 0x1478]
0x1404dd577 0f84ff1d0000 je 0x1404df37c
0x1404dd57d 49399048020000 cmp qword ptr [r8 + 0x248], rdx
0x1404dd584 0f86f21d0000 jbe 0x1404df37c
0x1404dd58a 498b8040020000 mov rax, qword ptr [r8 + 0x240]
0x1404dd591 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404dd595 4885c9 test rcx, rcx
0x1404dd598 0f84de1d0000 je 0x1404df37c
0x1404dd59e 498b8010020000 mov rax, qword ptr [r8 + 0x210]
0x1404dd5a5 48f7d1 not rcx
0x1404dd5a8 488d0c49 lea rcx, [rcx + rcx*2]
0x1404dd5ac 48c1e105 shl rcx, 5
0x1404dd5b0 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dd5b6 0f84c01d0000 je 0x1404df37c
0x1404dd5bc 660f6e8560140000 movd xmm0, dword ptr [rbp + 0x1460]
0x1404dd5c4 66410f6ece movd xmm1, r14d
0x1404dd5c9 660f62c8 punpckldq xmm1, xmm0
0x1404dd5cd 66410f6ec7 movd xmm0, r15d
0x1404dd5d2 660f6ed6 movd xmm2, esi
0x1404dd5d6 660f62c2 punpckldq xmm0, xmm2
0x1404dd5da 660f6cc1 punpcklqdq xmm0, xmm1
0x1404dd5de 660fdb052ac05002 pand xmm0, xmmword ptr [rip + 0x250c02a]
0x1404dd5e6 660f6fc8 movdqa xmm1, xmm0
0x1404dd5ea 66410fefcd pxor xmm1, xmm13
0x1404dd5ef 660f6f3529c05002 movdqa xmm6, xmmword ptr [rip + 0x250c029]
0x1404dd5f7 660f6fd6 movdqa xmm2, xmm6
0x1404dd5fb 660f66d1 pcmpgtd xmm2, xmm1
0x1404dd5ff 660fdbc2 pand xmm0, xmm2
0x1404dd603 660f6f1dc5bf5002 movdqa xmm3, xmmword ptr [rip + 0x250bfc5]
0x1404dd60b 660fdfd3 pandn xmm2, xmm3
0x1404dd60f 660febd0 por xmm2, xmm0
0x1404dd613 66440f6ffa movdqa xmm15, xmm2
0x1404dd618 66450feffd pxor xmm15, xmm13
0x1404dd61d 660f6f25bbbf5002 movdqa xmm4, xmmword ptr [rip + 0x250bfbb]
0x1404dd625 66440f66fc pcmpgtd xmm15, xmm4
0x1404dd62a 66410fdbd7 pand xmm2, xmm15
0x1404dd62f 660f6f2db9bf5002 movdqa xmm5, xmmword ptr [rip + 0x250bfb9]
0x1404dd637 66440fdffd pandn xmm15, xmm5
0x1404dd63c 66440febfa por xmm15, xmm2
0x1404dd641 660f6ec3 movd xmm0, ebx
0x1404dd645 66410f6ecc movd xmm1, r12d
0x1404dd64a 660f62c8 punpckldq xmm1, xmm0
0x1404dd64e 66410f6ec5 movd xmm0, r13d
0x1404dd653 660f6ed7 movd xmm2, edi
0x1404dd657 660f62c2 punpckldq xmm0, xmm2
0x1404dd65b 660f6cc1 punpcklqdq xmm0, xmm1
0x1404dd65f 660f6fc8 movdqa xmm1, xmm0
0x1404dd663 66410fefcd pxor xmm1, xmm13
0x1404dd668 660f6fd6 movdqa xmm2, xmm6
0x1404dd66c 660f66d1 pcmpgtd xmm2, xmm1
0x1404dd670 660fdbc2 pand xmm0, xmm2
0x1404dd674 660fdfd3 pandn xmm2, xmm3
0x1404dd678 660febd0 por xmm2, xmm0
0x1404dd67c 660f6fc2 movdqa xmm0, xmm2
0x1404dd680 66410fefc5 pxor xmm0, xmm13
0x1404dd685 660f66c4 pcmpgtd xmm0, xmm4
0x1404dd689 660fdbd0 pand xmm2, xmm0
0x1404dd68d 660fdfc5 pandn xmm0, xmm5
0x1404dd691 660febc2 por xmm0, xmm2
0x1404dd695 4801c8 add rax, rcx
0x1404dd698 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1404dd69c 418b90e0050000 mov edx, dword ptr [r8 + 0x5e0]
0x1404dd6a3 66440fd67910 movq qword ptr [rcx + 0x10], xmm15
0x1404dd6a9 89505c mov dword ptr [rax + 0x5c], edx
0x1404dd6ac 660fd64108 movq qword ptr [rcx + 8], xmm0
0x1404dd6b1 66440f7efb movd ebx, xmm15
0x1404dd6b6 66410f70f755 pshufd xmm6, xmm15, 0x55
0x1404dd6bc 488b8568140000 mov rax, qword ptr [rbp + 0x1468]
0x1404dd6c3 483b8598140000 cmp rax, qword ptr [rbp + 0x1498]
0x1404dd6ca 7524 jne 0x1404dd6f0
0x1404dd6cc 4183b80811000001 cmp dword ptr [r8 + 0x1108], 1
0x1404dd6d4 751a jne 0x1404dd6f0
0x1404dd6d6 660f7ef0 movd eax, xmm6
0x1404dd6da 48c1e020 shl rax, 0x20
0x1404dd6de 4809d8 or rax, rbx
0x1404dd6e1 4939800c110000 cmp qword ptr [r8 + 0x110c], rax
0x1404dd6e8 0f8449050000 je 0x1404ddc37
0x1404dd6ee 6690 nop 
0x1404dd6f0 488b8d50130000 mov rcx, qword ptr [rbp + 0x1350]
0x1404dd6f7 e8f49f7000 call 0x140be76f0 ; rhapsody::rg::resource_pool::ResourcePool::clear
0x1404dd6fc e83f008700 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
0x1404dd701 4889c6 mov rsi, rax
0x1404dd704 f60001 test byte ptr [rax], 1
0x1404dd707 7411 je 0x1404dd71a
0x1404dd709 4889f1 mov rcx, rsi
0x1404dd70c 4883c108 add rcx, 8
0x1404dd710 488b4608 mov rax, qword ptr [rsi + 8]
0x1404dd714 488b5610 mov rdx, qword ptr [rsi + 0x10]
0x1404dd718 eb1b jmp 0x1404dd735
0x1404dd71a e8e1768800 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
0x1404dd71f 4889f1 mov rcx, rsi
0x1404dd722 4883c108 add rcx, 8
0x1404dd726 48c70601000000 mov qword ptr [rsi], 1
0x1404dd72d 48894608 mov qword ptr [rsi + 8], rax
0x1404dd731 48895610 mov qword ptr [rsi + 0x10], rdx
0x1404dd735 4c8d4001 lea r8, [rax + 1]
0x1404dd739 4c8901 mov qword ptr [rcx], r8
0x1404dd73c 440f10356cc55002 movups xmm14, xmmword ptr [rip + 0x250c56c]
0x1404dd744 440f29b510060000 movaps xmmword ptr [rbp + 0x610], xmm14
0x1404dd74c 440f10054cc55002 movups xmm8, xmmword ptr [rip + 0x250c54c]
0x1404dd754 440f298500060000 movaps xmmword ptr [rbp + 0x600], xmm8
0x1404dd75c 48898520060000 mov qword ptr [rbp + 0x620], rax
0x1404dd763 48899528060000 mov qword ptr [rbp + 0x628], rdx
0x1404dd76a e8d1ff8600 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
0x1404dd76f 4889c6 mov rsi, rax
0x1404dd772 f60001 test byte ptr [rax], 1
0x1404dd775 7411 je 0x1404dd788
0x1404dd777 4889f0 mov rax, rsi
0x1404dd77a 4883c008 add rax, 8
0x1404dd77e 4c8b7e08 mov r15, qword ptr [rsi + 8]
0x1404dd782 4c8b7610 mov r14, qword ptr [rsi + 0x10]
0x1404dd786 eb21 jmp 0x1404dd7a9
0x1404dd788 e873768800 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
0x1404dd78d 4989c7 mov r15, rax
0x1404dd790 4989d6 mov r14, rdx
0x1404dd793 4889f0 mov rax, rsi
0x1404dd796 4883c008 add rax, 8
0x1404dd79a 48c70601000000 mov qword ptr [rsi], 1
0x1404dd7a1 4c897e08 mov qword ptr [rsi + 8], r15
0x1404dd7a5 48895610 mov qword ptr [rsi + 0x10], rdx
0x1404dd7a9 488bbd78140000 mov rdi, qword ptr [rbp + 0x1478]
0x1404dd7b0 498d4f01 lea rcx, [r15 + 1]
0x1404dd7b4 488908 mov qword ptr [rax], rcx
0x1404dd7b7 0f288500060000 movaps xmm0, xmmword ptr [rbp + 0x600]
0x1404dd7be 0f288d10060000 movaps xmm1, xmmword ptr [rbp + 0x610]
0x1404dd7c5 0f289520060000 movaps xmm2, xmmword ptr [rbp + 0x620]
0x1404dd7cc 0f2995500c0000 movaps xmmword ptr [rbp + 0xc50], xmm2
0x1404dd7d3 0f298d400c0000 movaps xmmword ptr [rbp + 0xc40], xmm1
0x1404dd7da 0f2985300c0000 movaps xmmword ptr [rbp + 0xc30], xmm0
0x1404dd7e1 488d85380c0000 lea rax, [rbp + 0xc38]
0x1404dd7e8 440f117038 movups xmmword ptr [rax + 0x38], xmm14
0x1404dd7ed 440f114028 movups xmmword ptr [rax + 0x28], xmm8
0x1404dd7f2 0f2885300c0000 movaps xmm0, xmmword ptr [rbp + 0xc30]
0x1404dd7f9 0f288d400c0000 movaps xmm1, xmmword ptr [rbp + 0xc40]
0x1404dd800 0f2895500c0000 movaps xmm2, xmmword ptr [rbp + 0xc50]
0x1404dd807 0f289d600c0000 movaps xmm3, xmmword ptr [rbp + 0xc60]
0x1404dd80e 0f298500060000 movaps xmmword ptr [rbp + 0x600], xmm0
0x1404dd815 0f298d10060000 movaps xmmword ptr [rbp + 0x610], xmm1
0x1404dd81c 0f299520060000 movaps xmmword ptr [rbp + 0x620], xmm2
0x1404dd823 0f299d30060000 movaps xmmword ptr [rbp + 0x630], xmm3
0x1404dd82a 0f2885700c0000 movaps xmm0, xmmword ptr [rbp + 0xc70]
0x1404dd831 0f298540060000 movaps xmmword ptr [rbp + 0x640], xmm0
0x1404dd838 488b85c8130000 mov rax, qword ptr [rbp + 0x13c8]
0x1404dd83f 48833800 cmp qword ptr [rax], 0
0x1404dd843 747d je 0x1404dd8c2
0x1404dd845 4c89bd68140000 mov qword ptr [rbp + 0x1468], r15
0x1404dd84c 4c89b598140000 mov qword ptr [rbp + 0x1498], r14
0x1404dd853 41b868000000 mov r8d, 0x68
0x1404dd859 41b910000000 mov r9d, 0x10
0x1404dd85f 488b8dc8130000 mov rcx, qword ptr [rbp + 0x13c8]
0x1404dd866 488b9558130000 mov rdx, qword ptr [rbp + 0x1358]
0x1404dd86d e83ef6b6ff call 0x14004ceb0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>,alloc::alloc::Global>
0x1404dd872 488bbd78140000 mov rdi, qword ptr [rbp + 0x1478]
0x1404dd879 488b97c8080000 mov rdx, qword ptr [rdi + 0x8c8]
0x1404dd880 4885d2 test rdx, rdx
0x1404dd883 4c8bb598140000 mov r14, qword ptr [rbp + 0x1498]
0x1404dd88a 4c8bbd68140000 mov r15, qword ptr [rbp + 0x1468]
0x1404dd891 742f je 0x1404dd8c2
0x1404dd893 4889d0 mov rax, rdx
0x1404dd896 48c1e004 shl rax, 4
0x1404dd89a 4801c2 add rdx, rax
0x1404dd89d 4883c221 add rdx, 0x21
0x1404dd8a1 741f je 0x1404dd8c2
0x1404dd8a3 488b8d98130000 mov rcx, qword ptr [rbp + 0x1398]
0x1404dd8aa 488b09 mov rcx, qword ptr [rcx]
0x1404dd8ad 48f7d8 neg rax
0x1404dd8b0 4801c1 add rcx, rax
0x1404dd8b3 4883c1f0 add rcx, -0x10
0x1404dd8b7 41b810000000 mov r8d, 0x10
0x1404dd8bd e85e631300 call 0x140613c20 ; __rust_dealloc
0x1404dd8c2 0f288540060000 movaps xmm0, xmmword ptr [rbp + 0x640]
0x1404dd8c9 488b85c8130000 mov rax, qword ptr [rbp + 0x13c8]
0x1404dd8d0 0f114040 movups xmmword ptr [rax + 0x40], xmm0
0x1404dd8d4 0f288500060000 movaps xmm0, xmmword ptr [rbp + 0x600]
0x1404dd8db 0f288d10060000 movaps xmm1, xmmword ptr [rbp + 0x610]
0x1404dd8e2 0f289520060000 movaps xmm2, xmmword ptr [rbp + 0x620]
0x1404dd8e9 0f289d30060000 movaps xmm3, xmmword ptr [rbp + 0x630]
0x1404dd8f0 0f115830 movups xmmword ptr [rax + 0x30], xmm3
0x1404dd8f4 0f115020 movups xmmword ptr [rax + 0x20], xmm2
0x1404dd8f8 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x1404dd8fc 0f1100 movups xmmword ptr [rax], xmm0
0x1404dd8ff 4c89bfe0080000 mov qword ptr [rdi + 0x8e0], r15
0x1404dd906 4c89b7e8080000 mov qword ptr [rdi + 0x8e8], r14
0x1404dd90d e82efe8600 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
0x1404dd912 4889c6 mov rsi, rax
0x1404dd915 f60001 test byte ptr [rax], 1
0x1404dd918 7411 je 0x1404dd92b
0x1404dd91a 4889f1 mov rcx, rsi
0x1404dd91d 4883c108 add rcx, 8
0x1404dd921 488b4608 mov rax, qword ptr [rsi + 8]
0x1404dd925 488b5610 mov rdx, qword ptr [rsi + 0x10]
0x1404dd929 eb22 jmp 0x1404dd94d
0x1404dd92b e8d0748800 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
0x1404dd930 4889f1 mov rcx, rsi
0x1404dd933 4883c108 add rcx, 8
0x1404dd937 48c70601000000 mov qword ptr [rsi], 1
0x1404dd93e 48894608 mov qword ptr [rsi + 8], rax
0x1404dd942 48895610 mov qword ptr [rsi + 0x10], rdx
0x1404dd946 488bbd78140000 mov rdi, qword ptr [rbp + 0x1478]
0x1404dd94d 4c8d4001 lea r8, [rax + 1]
0x1404dd951 4c8901 mov qword ptr [rcx], r8
0x1404dd954 440f29b510060000 movaps xmmword ptr [rbp + 0x610], xmm14
0x1404dd95c 440f298500060000 movaps xmmword ptr [rbp + 0x600], xmm8
0x1404dd964 48898520060000 mov qword ptr [rbp + 0x620], rax
0x1404dd96b 48899528060000 mov qword ptr [rbp + 0x628], rdx
0x1404dd972 e8c9fd8600 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
0x1404dd977 4889c6 mov rsi, rax
0x1404dd97a f60001 test byte ptr [rax], 1
0x1404dd97d 7411 je 0x1404dd990
0x1404dd97f 4889f0 mov rax, rsi
0x1404dd982 4883c008 add rax, 8
0x1404dd986 4c8b7e08 mov r15, qword ptr [rsi + 8]
0x1404dd98a 4c8b7610 mov r14, qword ptr [rsi + 0x10]
0x1404dd98e eb21 jmp 0x1404dd9b1
0x1404dd990 e86b748800 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
0x1404dd995 4989c7 mov r15, rax
0x1404dd998 4989d6 mov r14, rdx
0x1404dd99b 4889f0 mov rax, rsi
0x1404dd99e 4883c008 add rax, 8
0x1404dd9a2 48c70601000000 mov qword ptr [rsi], 1
0x1404dd9a9 4c897e08 mov qword ptr [rsi + 8], r15
0x1404dd9ad 48895610 mov qword ptr [rsi + 0x10], rdx
0x1404dd9b1 498d4f01 lea rcx, [r15 + 1]
0x1404dd9b5 488908 mov qword ptr [rax], rcx
0x1404dd9b8 0f288500060000 movaps xmm0, xmmword ptr [rbp + 0x600]
0x1404dd9bf 0f288d10060000 movaps xmm1, xmmword ptr [rbp + 0x610]
0x1404dd9c6 0f289520060000 movaps xmm2, xmmword ptr [rbp + 0x620]
0x1404dd9cd 0f2995500c0000 movaps xmmword ptr [rbp + 0xc50], xmm2
0x1404dd9d4 0f298d400c0000 movaps xmmword ptr [rbp + 0xc40], xmm1
0x1404dd9db 0f2985300c0000 movaps xmmword ptr [rbp + 0xc30], xmm0
0x1404dd9e2 488d85380c0000 lea rax, [rbp + 0xc38]
0x1404dd9e9 440f117038 movups xmmword ptr [rax + 0x38], xmm14
0x1404dd9ee 440f114028 movups xmmword ptr [rax + 0x28], xmm8
0x1404dd9f3 0f2885300c0000 movaps xmm0, xmmword ptr [rbp + 0xc30]
0x1404dd9fa 0f288d400c0000 movaps xmm1, xmmword ptr [rbp + 0xc40]
0x1404dda01 0f2895500c0000 movaps xmm2, xmmword ptr [rbp + 0xc50]
0x1404dda08 0f289d600c0000 movaps xmm3, xmmword ptr [rbp + 0xc60]
0x1404dda0f 0f298500060000 movaps xmmword ptr [rbp + 0x600], xmm0
0x1404dda16 0f298d10060000 movaps xmmword ptr [rbp + 0x610], xmm1
0x1404dda1d 0f299520060000 movaps xmmword ptr [rbp + 0x620], xmm2
0x1404dda24 0f299d30060000 movaps xmmword ptr [rbp + 0x630], xmm3
0x1404dda2b 0f2885700c0000 movaps xmm0, xmmword ptr [rbp + 0xc70]
0x1404dda32 0f298540060000 movaps xmmword ptr [rbp + 0x640], xmm0
0x1404dda39 488b85d0130000 mov rax, qword ptr [rbp + 0x13d0]
0x1404dda40 48833800 cmp qword ptr [rax], 0
0x1404dda44 747d je 0x1404ddac3
0x1404dda46 4c89bd68140000 mov qword ptr [rbp + 0x1468], r15
0x1404dda4d 4c89b598140000 mov qword ptr [rbp + 0x1498], r14
0x1404dda54 41b868000000 mov r8d, 0x68
0x1404dda5a 41b910000000 mov r9d, 0x10
0x1404dda60 488b8dd0130000 mov rcx, qword ptr [rbp + 0x13d0]
0x1404dda67 488b9560130000 mov rdx, qword ptr [rbp + 0x1360]
0x1404dda6e e83df4b6ff call 0x14004ceb0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>,alloc::alloc::Global>
0x1404dda73 488bbd78140000 mov rdi, qword ptr [rbp + 0x1478]
0x1404dda7a 488b97b8090000 mov rdx, qword ptr [rdi + 0x9b8]
0x1404dda81 4885d2 test rdx, rdx
0x1404dda84 4c8bb598140000 mov r14, qword ptr [rbp + 0x1498]
0x1404dda8b 4c8bbd68140000 mov r15, qword ptr [rbp + 0x1468]
0x1404dda92 742f je 0x1404ddac3
0x1404dda94 4889d0 mov rax, rdx
0x1404dda97 48c1e004 shl rax, 4
0x1404dda9b 4801c2 add rdx, rax
0x1404dda9e 4883c221 add rdx, 0x21
0x1404ddaa2 741f je 0x1404ddac3
0x1404ddaa4 488b8da0130000 mov rcx, qword ptr [rbp + 0x13a0]
0x1404ddaab 488b09 mov rcx, qword ptr [rcx]
0x1404ddaae 48f7d8 neg rax
0x1404ddab1 4801c1 add rcx, rax
0x1404ddab4 4883c1f0 add rcx, -0x10
0x1404ddab8 41b810000000 mov r8d, 0x10
0x1404ddabe e85d611300 call 0x140613c20 ; __rust_dealloc
0x1404ddac3 0f288540060000 movaps xmm0, xmmword ptr [rbp + 0x640]
0x1404ddaca 488b85d0130000 mov rax, qword ptr [rbp + 0x13d0]
0x1404ddad1 0f114040 movups xmmword ptr [rax + 0x40], xmm0
0x1404ddad5 660f6f8500060000 movdqa xmm0, xmmword ptr [rbp + 0x600]
0x1404ddadd 660f6f8d10060000 movdqa xmm1, xmmword ptr [rbp + 0x610]
0x1404ddae5 660f6f9520060000 movdqa xmm2, xmmword ptr [rbp + 0x620]
0x1404ddaed 660f6f9d30060000 movdqa xmm3, xmmword ptr [rbp + 0x630]
0x1404ddaf5 f30f7f5830 movdqu xmmword ptr [rax + 0x30], xmm3
0x1404ddafa f30f7f5020 movdqu xmmword ptr [rax + 0x20], xmm2
0x1404ddaff f30f7f4810 movdqu xmmword ptr [rax + 0x10], xmm1
0x1404ddb04 f30f7f00 movdqu xmmword ptr [rax], xmm0
0x1404ddb08 4c89bfd0090000 mov qword ptr [rdi + 0x9d0], r15
0x1404ddb0f 4c89b7d8090000 mov qword ptr [rdi + 0x9d8], r14
0x1404ddb16 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404ddb1d 48bacfde921b5889d12e movabs rdx, 0x2ed189581b92decf
0x1404ddb27 49b8385f069693d9d315 movabs r8, 0x15d3d99396065f38
0x1404ddb31 e8bafbbfff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404ddb36 4885c0 test rax, rax
0x1404ddb39 4c8b8578140000 mov r8, qword ptr [rbp + 0x1478]
0x1404ddb40 0f8453190000 je 0x1404df499
0x1404ddb46 49399048020000 cmp qword ptr [r8 + 0x248], rdx
0x1404ddb4d 0f8646190000 jbe 0x1404df499
0x1404ddb53 498b8040020000 mov rax, qword ptr [r8 + 0x240]
0x1404ddb5a 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404ddb5e 4885c9 test rcx, rcx
0x1404ddb61 0f8432190000 je 0x1404df499
0x1404ddb67 498b8010020000 mov rax, qword ptr [r8 + 0x210]
0x1404ddb6e 48f7d1 not rcx
0x1404ddb71 488d0c49 lea rcx, [rcx + rcx*2]
0x1404ddb75 48c1e105 shl rcx, 5
0x1404ddb79 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404ddb7f 0f8414190000 je 0x1404df499
0x1404ddb85 4801c8 add rax, rcx
0x1404ddb88 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1404ddb8c e80fa14a00 call 0x140987ca0 ; country_core::render::impl$0::deref
0x1404ddb91 4889c1 mov rcx, rax
0x1404ddb94 e8c7b27100 call 0x140bf8e60 ; rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources
0x1404ddb99 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404ddba0 48ba2a26366264338ef0 movabs rdx, 0xf08e33646236262a
0x1404ddbaa 49b827c1f9ddf74066d9 movabs r8, 0xd96640f7ddf9c127
0x1404ddbb4 e837fbbfff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404ddbb9 4885c0 test rax, rax
0x1404ddbbc 4c8b8578140000 mov r8, qword ptr [rbp + 0x1478]
0x1404ddbc3 0f84e1180000 je 0x1404df4aa
0x1404ddbc9 49399048020000 cmp qword ptr [r8 + 0x248], rdx
0x1404ddbd0 0f86d4180000 jbe 0x1404df4aa
0x1404ddbd6 498b8040020000 mov rax, qword ptr [r8 + 0x240]
0x1404ddbdd 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404ddbe1 4885c9 test rcx, rcx
0x1404ddbe4 0f84c0180000 je 0x1404df4aa
0x1404ddbea 498b8010020000 mov rax, qword ptr [r8 + 0x210]
0x1404ddbf1 48f7d1 not rcx
0x1404ddbf4 488d0c49 lea rcx, [rcx + rcx*2]
0x1404ddbf8 48c1e105 shl rcx, 5
0x1404ddbfc 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404ddc02 0f84a2180000 je 0x1404df4aa
0x1404ddc08 4801c8 add rax, rcx
0x1404ddc0b 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1404ddc0f 418b90e0050000 mov edx, dword ptr [r8 + 0x5e0]
0x1404ddc16 89505c mov dword ptr [rax + 0x5c], edx
0x1404ddc19 c7810801000000000000 mov dword ptr [rcx + 0x108], 0
0x1404ddc23 41c7800811000001000000 mov dword ptr [r8 + 0x1108], 1
0x1404ddc2e 66450fd6b80c110000 movq qword ptr [r8 + 0x110c], xmm15
0x1404ddc37 e8c48d7300 call 0x140c16a00 ; puffin::are_scopes_on
0x1404ddc3c 84c0 test al, al
0x1404ddc3e 48899d60140000 mov qword ptr [rbp + 0x1460], rbx
0x1404ddc45 0f84a5000000 je 0x1404ddcf0
0x1404ddc4b 488d3590c05002 lea rsi, [rip + 0x250c090]
0x1404ddc52 4c8d0d67c05002 lea r9, [rip + 0x250c067]
0x1404ddc59 0f1f8000000000 nop dword ptr [rax]
0x1404ddc60 0fbe46ff movsx eax, byte ptr [rsi - 1]
0x1404ddc64 85c0 test eax, eax
0x1404ddc66 7818 js 0x1404ddc80
0x1404ddc68 48ffce dec rsi
0x1404ddc6b 83f85c cmp eax, 0x5c
0x1404ddc6e 7562 jne 0x1404ddcd2
0x1404ddc70 e982000000 jmp 0x1404ddcf7
0x1404ddc75 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404ddc80 0fb64efe movzx ecx, byte ptr [rsi - 2]
0x1404ddc84 80f9c0 cmp cl, 0xc0
0x1404ddc87 7d22 jge 0x1404ddcab
0x1404ddc89 0fb656fd movzx edx, byte ptr [rsi - 3]
0x1404ddc8d 80fac0 cmp dl, 0xc0
0x1404ddc90 7d22 jge 0x1404ddcb4
0x1404ddc92 440fb646fc movzx r8d, byte ptr [rsi - 4]
0x1404ddc97 4883c6fc add rsi, -4
0x1404ddc9b 4183e007 and r8d, 7
0x1404ddc9f 41c1e006 shl r8d, 6
0x1404ddca3 83e23f and edx, 0x3f
0x1404ddca6 4409c2 or edx, r8d
0x1404ddca9 eb10 jmp 0x1404ddcbb
0x1404ddcab 4883c6fe add rsi, -2
0x1404ddcaf 83e11f and ecx, 0x1f
0x1404ddcb2 eb0f jmp 0x1404ddcc3
0x1404ddcb4 4883c6fd add rsi, -3
0x1404ddcb8 83e20f and edx, 0xf
0x1404ddcbb c1e206 shl edx, 6
0x1404ddcbe 83e13f and ecx, 0x3f
0x1404ddcc1 09d1 or ecx, edx
0x1404ddcc3 c1e106 shl ecx, 6
0x1404ddcc6 243f and al, 0x3f
0x1404ddcc8 0fb6c0 movzx eax, al
0x1404ddccb 09c8 or eax, ecx
0x1404ddccd 83f85c cmp eax, 0x5c
0x1404ddcd0 7425 je 0x1404ddcf7
0x1404ddcd2 83f82f cmp eax, 0x2f
0x1404ddcd5 7420 je 0x1404ddcf7
0x1404ddcd7 4c39ce cmp rsi, r9
0x1404ddcda 7584 jne 0x1404ddc60
0x1404ddcdc bf22000000 mov edi, 0x22
0x1404ddce1 4c89ce mov rsi, r9
0x1404ddce4 eb3c jmp 0x1404ddd22
0x1404ddce6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404ddcf0 31c0 xor eax, eax
0x1404ddcf2 e9bb000000 jmp 0x1404dddb2
0x1404ddcf7 4c29ce sub rsi, r9
0x1404ddcfa 48ffc6 inc rsi
0x1404ddcfd 7418 je 0x1404ddd17
0x1404ddcff 4883fe22 cmp rsi, 0x22
0x1404ddd03 7207 jb 0x1404ddd0c
0x1404ddd05 7410 je 0x1404ddd17
0x1404ddd07 e9a7180000 jmp 0x1404df5b3
0x1404ddd0c 42803c0ebf cmp byte ptr [rsi + r9], 0xbf
0x1404ddd11 0f8e9c180000 jle 0x1404df5b3
0x1404ddd17 bf22000000 mov edi, 0x22
0x1404ddd1c 4829f7 sub rdi, rsi
0x1404ddd1f 4c01ce add rsi, r9
0x1404ddd22 e809a47300 call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
0x1404ddd27 4889c1 mov rcx, rax
0x1404ddd2a 488b00 mov rax, qword ptr [rax]
0x1404ddd2d 4885c0 test rax, rax
0x1404ddd30 740f je 0x1404ddd41
0x1404ddd32 83f801 cmp eax, 1
0x1404ddd35 0f8545180000 jne 0x1404df580
0x1404ddd3b 4883c108 add rcx, 8
0x1404ddd3f eb13 jmp 0x1404ddd54
0x1404ddd41 31d2 xor edx, edx
0x1404ddd43 e8f8383c02 call 0x1428a1640 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
0x1404ddd48 4889c1 mov rcx, rax
0x1404ddd4b 4885c0 test rax, rax
0x1404ddd4e 0f842c180000 je 0x1404df580
0x1404ddd54 48833900 cmp qword ptr [rcx], 0
0x1404ddd58 0f8580110000 jne 0x1404deede
0x1404ddd5e 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x1404ddd65 48898d98140000 mov qword ptr [rbp + 0x1498], rcx
0x1404ddd6c 4883c108 add rcx, 8
0x1404ddd70 48897c2420 mov qword ptr [rsp + 0x20], rdi
0x1404ddd75 48c744243000000000 mov qword ptr [rsp + 0x30], 0
0x1404ddd7e 48c744242801000000 mov qword ptr [rsp + 0x28], 1
0x1404ddd87 41b809000000 mov r8d, 9
0x1404ddd8d 488d157cc35002 lea rdx, [rip + 0x250c37c]
0x1404ddd94 4989f1 mov r9, rsi
0x1404ddd97 e8a48f7300 call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
0x1404ddd9c 488b8d98140000 mov rcx, qword ptr [rbp + 0x1498]
0x1404ddda3 48ff01 inc qword ptr [rcx]
0x1404ddda6 488985e8130000 mov qword ptr [rbp + 0x13e8], rax
0x1404dddad b801000000 mov eax, 1
0x1404dddb2 4c8bbd78140000 mov r15, qword ptr [rbp + 0x1478]
0x1404dddb9 4c8bad50140000 mov r13, qword ptr [rbp + 0x1450]
0x1404dddc0 488985e0130000 mov qword ptr [rbp + 0x13e0], rax
0x1404dddc7 488b052ae51403 mov rax, qword ptr [rip + 0x314e52a]
0x1404dddce 488b00 mov rax, qword ptr [rax]
0x1404dddd1 4883f802 cmp rax, 2
0x1404dddd5 740f je 0x1404ddde6
0x1404dddd7 488b0d1ae51403 mov rcx, qword ptr [rip + 0x314e51a]
0x1404dddde 4889ca mov rdx, rcx
0x1404ddde1 e86a673c02 call 0x1428a4550 ; once_cell::imp::OnceCell::initialize<tuple$<i64,std::time::Instant>,once_cell::sync::impl$6::get_or_init::closure_env$0<tuple$<i64,std::time::Instant>,once_cell::sync::impl$11::force::closure_env$0<tuple$<i64,std::time::Instant>,tuple$<i64,std::time::Instant> (*)()> >,enum2$<once_cell::sync::impl$6::get_or_init::Void> >
0x1404ddde6 488b050be51403 mov rax, qword ptr [rip + 0x314e50b]
0x1404ddded 488b7008 mov rsi, qword ptr [rax + 8]
0x1404dddf1 488b00 mov rax, qword ptr [rax]
0x1404dddf4 4883f802 cmp rax, 2
0x1404dddf8 740f je 0x1404dde09
0x1404dddfa 488b0df7e41403 mov rcx, qword ptr [rip + 0x314e4f7]
0x1404dde01 4889ca mov rdx, rcx
0x1404dde04 e847673c02 call 0x1428a4550 ; once_cell::imp::OnceCell::initialize<tuple$<i64,std::time::Instant>,once_cell::sync::impl$6::get_or_init::closure_env$0<tuple$<i64,std::time::Instant>,once_cell::sync::impl$11::force::closure_env$0<tuple$<i64,std::time::Instant>,tuple$<i64,std::time::Instant> (*)()> >,enum2$<once_cell::sync::impl$6::get_or_init::Void> >
0x1404dde09 488b05e8e41403 mov rax, qword ptr [rip + 0x314e4e8]
0x1404dde10 488d4810 lea rcx, [rax + 0x10]
0x1404dde14 e8f75a8800 call 0x140d63910 ; std::time::Instant::elapsed
0x1404dde19 899568140000 mov dword ptr [rbp + 0x1468], edx
0x1404dde1f 4989c6 mov r14, rax
0x1404dde22 e8f9957300 call 0x140c17420 ; puffin::GlobalProfiler::lock
0x1404dde27 48898598140000 mov qword ptr [rbp + 0x1498], rax
0x1404dde2e 4889c1 mov rcx, rax
0x1404dde31 4883c108 add rcx, 8
0x1404dde35 e846967300 call 0x140c17480 ; puffin::GlobalProfiler::new_frame
0x1404dde3a 488bbd98140000 mov rdi, qword ptr [rbp + 0x1498]
0x1404dde41 4889f9 mov rcx, rdi
0x1404dde44 e8c73f7a00 call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
0x1404dde49 b001 mov al, 1
0x1404dde4b 31c9 xor ecx, ecx
0x1404dde4d f00fb00f lock cmpxchg byte ptr [rdi], cl
0x1404dde51 0f852f0f0000 jne 0x1404ded86
0x1404dde57 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dde5e 48bad115d45039305c90 movabs rdx, 0x905c303950d415d1
0x1404dde68 49b872c8357026de6746 movabs r8, 0x4667de267035c872
0x1404dde72 e879f8bfff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404dde77 41b401 mov r12b, 1
0x1404dde7a 4885c0 test rax, rax
0x1404dde7d 0f842d030000 je 0x1404de1b0
0x1404dde83 49399748020000 cmp qword ptr [r15 + 0x248], rdx
0x1404dde8a 0f8620030000 jbe 0x1404de1b0
0x1404dde90 498b8740020000 mov rax, qword ptr [r15 + 0x240]
0x1404dde97 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404dde9b 4885c9 test rcx, rcx
0x1404dde9e 0f840c030000 je 0x1404de1b0
0x1404ddea4 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x1404ddeab 48f7d1 not rcx
0x1404ddeae 488d0c49 lea rcx, [rcx + rcx*2]
0x1404ddeb2 48c1e105 shl rcx, 5
0x1404ddeb6 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404ddebc 0f84ee020000 je 0x1404de1b0
0x1404ddec2 4801c8 add rax, rcx
0x1404ddec5 488b7810 mov rdi, qword ptr [rax + 0x10]
0x1404ddec9 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dded0 48ba01c39d77ef21b348 movabs rdx, 0x48b321ef779dc301
0x1404ddeda 49b8c8ddd994717665bd movabs r8, 0xbd65767194d9ddc8
0x1404ddee4 e807f8bfff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404ddee9 4885c0 test rax, rax
0x1404ddeec 7454 je 0x1404ddf42
0x1404ddeee 49399748020000 cmp qword ptr [r15 + 0x248], rdx
0x1404ddef5 764b jbe 0x1404ddf42
0x1404ddef7 498b8740020000 mov rax, qword ptr [r15 + 0x240]
0x1404ddefe 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404ddf02 4885c9 test rcx, rcx
0x1404ddf05 743b je 0x1404ddf42
0x1404ddf07 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x1404ddf0e 48f7d1 not rcx
0x1404ddf11 488d0c49 lea rcx, [rcx + rcx*2]
0x1404ddf15 48c1e105 shl rcx, 5
0x1404ddf19 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404ddf1f 7421 je 0x1404ddf42
0x1404ddf21 4801c8 add rax, rcx
0x1404ddf24 488b4010 mov rax, qword ptr [rax + 0x10]
0x1404ddf28 80b8f000000000 cmp byte ptr [rax + 0xf0], 0
0x1404ddf2f 0f85fc0d0000 jne 0x1404ded31
0x1404ddf35 f6800101000001 test byte ptr [rax + 0x101], 1
0x1404ddf3c 0f85ef0d0000 jne 0x1404ded31
0x1404ddf42 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404ddf49 e822e87400 call 0x140c2c770 ; egui::data::input::impl$0::default
0x1404ddf4e c685a314000001 mov byte ptr [rbp + 0x14a3], 1
0x1404ddf55 e8a68a7300 call 0x140c16a00 ; puffin::are_scopes_on
0x1404ddf5a 4c8d0d5fbd5002 lea r9, [rip + 0x250bd5f]
0x1404ddf61 84c0 test al, al
0x1404ddf63 888598140000 mov byte ptr [rbp + 0x1498], al
0x1404ddf69 0f8499000000 je 0x1404de008
0x1404ddf6f 4c8d2d6cbd5002 lea r13, [rip + 0x250bd6c]
0x1404ddf76 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404ddf80 410fbe45ff movsx eax, byte ptr [r13 - 1]
0x1404ddf85 85c0 test eax, eax
0x1404ddf87 7817 js 0x1404ddfa0
0x1404ddf89 49ffcd dec r13
0x1404ddf8c 83f85c cmp eax, 0x5c
0x1404ddf8f 7563 jne 0x1404ddff4
0x1404ddf91 eb7c jmp 0x1404de00f
0x1404ddf93 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404ddfa0 410fb64dfe movzx ecx, byte ptr [r13 - 2]
0x1404ddfa5 80f9c0 cmp cl, 0xc0
0x1404ddfa8 7d23 jge 0x1404ddfcd
0x1404ddfaa 410fb655fd movzx edx, byte ptr [r13 - 3]
0x1404ddfaf 80fac0 cmp dl, 0xc0
0x1404ddfb2 7d22 jge 0x1404ddfd6
0x1404ddfb4 450fb645fc movzx r8d, byte ptr [r13 - 4]
0x1404ddfb9 4983c5fc add r13, -4
0x1404ddfbd 4183e007 and r8d, 7
0x1404ddfc1 41c1e006 shl r8d, 6
0x1404ddfc5 83e23f and edx, 0x3f
0x1404ddfc8 4409c2 or edx, r8d
0x1404ddfcb eb10 jmp 0x1404ddfdd
0x1404ddfcd 4983c5fe add r13, -2
0x1404ddfd1 83e11f and ecx, 0x1f
0x1404ddfd4 eb0f jmp 0x1404ddfe5
0x1404ddfd6 4983c5fd add r13, -3
0x1404ddfda 83e20f and edx, 0xf
0x1404ddfdd c1e206 shl edx, 6
0x1404ddfe0 83e13f and ecx, 0x3f
0x1404ddfe3 09d1 or ecx, edx
0x1404ddfe5 c1e106 shl ecx, 6
0x1404ddfe8 243f and al, 0x3f
0x1404ddfea 0fb6c0 movzx eax, al
0x1404ddfed 09c8 or eax, ecx
0x1404ddfef 83f85c cmp eax, 0x5c
0x1404ddff2 741b je 0x1404de00f
0x1404ddff4 83f82f cmp eax, 0x2f
0x1404ddff7 7416 je 0x1404de00f
0x1404ddff9 4d39cd cmp r13, r9
0x1404ddffc 7582 jne 0x1404ddf80
0x1404ddffe bb22000000 mov ebx, 0x22
0x1404de003 4d89cd mov r13, r9
0x1404de006 eb33 jmp 0x1404de03b
0x1404de008 31c0 xor eax, eax
0x1404de00a e9c0000000 jmp 0x1404de0cf
0x1404de00f 4d29cd sub r13, r9
0x1404de012 49ffc5 inc r13
0x1404de015 7419 je 0x1404de030
0x1404de017 4983fd22 cmp r13, 0x22
0x1404de01b 7207 jb 0x1404de024
0x1404de01d 7411 je 0x1404de030
0x1404de01f e909160000 jmp 0x1404df62d
0x1404de024 43807c0d00bf cmp byte ptr [r13 + r9], 0xbf
0x1404de02a 0f8efd150000 jle 0x1404df62d
0x1404de030 bb22000000 mov ebx, 0x22
0x1404de035 4c29eb sub rbx, r13
0x1404de038 4d01cd add r13, r9
0x1404de03b e8f0a07300 call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
0x1404de040 4889c1 mov rcx, rax
0x1404de043 488b00 mov rax, qword ptr [rax]
0x1404de046 4885c0 test rax, rax
0x1404de049 740f je 0x1404de05a
0x1404de04b 83f801 cmp eax, 1
0x1404de04e 0f85af150000 jne 0x1404df603
0x1404de054 4883c108 add rcx, 8
0x1404de058 eb1a jmp 0x1404de074
0x1404de05a c685a314000001 mov byte ptr [rbp + 0x14a3], 1
0x1404de061 31d2 xor edx, edx
0x1404de063 e8d8353c02 call 0x1428a1640 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
0x1404de068 4889c1 mov rcx, rax
0x1404de06b 4885c0 test rax, rax
0x1404de06e 0f848f150000 je 0x1404df603
0x1404de074 48833900 cmp qword ptr [rcx], 0
0x1404de078 0f85b00e0000 jne 0x1404def2e
0x1404de07e 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x1404de085 48898d48140000 mov qword ptr [rbp + 0x1448], rcx
0x1404de08c 4883c108 add rcx, 8
0x1404de090 48895c2420 mov qword ptr [rsp + 0x20], rbx
0x1404de095 48c744243000000000 mov qword ptr [rsp + 0x30], 0
0x1404de09e 48c744242801000000 mov qword ptr [rsp + 0x28], 1
0x1404de0a7 41b810000000 mov r8d, 0x10
0x1404de0ad 488d1565c05002 lea rdx, [rip + 0x250c065]
0x1404de0b4 4d89e9 mov r9, r13
0x1404de0b7 e8848c7300 call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
0x1404de0bc 488b8d48140000 mov rcx, qword ptr [rbp + 0x1448]
0x1404de0c3 48ff01 inc qword ptr [rcx]
0x1404de0c6 488945c8 mov qword ptr [rbp - 0x38], rax
0x1404de0ca b801000000 mov eax, 1
0x1404de0cf 4c8bbd78140000 mov r15, qword ptr [rbp + 0x1478]
0x1404de0d6 488d9d300c0000 lea rbx, [rbp + 0xc30]
0x1404de0dd 488945c0 mov qword ptr [rbp - 0x40], rax
0x1404de0e1 c6857214000001 mov byte ptr [rbp + 0x1472], 1
0x1404de0e8 4889f9 mov rcx, rdi
0x1404de0eb e890fc1900 call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
0x1404de0f0 4889c7 mov rdi, rax
0x1404de0f3 41b8b0000000 mov r8d, 0xb0
0x1404de0f9 4889d9 mov rcx, rbx
0x1404de0fc 488d9500060000 lea rdx, [rbp + 0x600]
0x1404de103 e8f85a4402 call 0x142923c00 ; memcpy
0x1404de108 c6857214000000 mov byte ptr [rbp + 0x1472], 0
0x1404de10f 4889f9 mov rcx, rdi
0x1404de112 4889da mov rdx, rbx
0x1404de115 e8e6317800 call 0x140c61300 ; egui::context::Context::begin_frame
0x1404de11a 80bd9814000000 cmp byte ptr [rbp + 0x1498], 0
0x1404de121 4c8bad50140000 mov r13, qword ptr [rbp + 0x1450]
0x1404de128 7476 je 0x1404de1a0
0x1404de12a e801a07300 call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
0x1404de12f 4889c1 mov rcx, rax
0x1404de132 488b00 mov rax, qword ptr [rax]
0x1404de135 4885c0 test rax, rax
0x1404de138 740f je 0x1404de149
0x1404de13a 83f801 cmp eax, 1
0x1404de13d 0f85d5140000 jne 0x1404df618
0x1404de143 4883c108 add rcx, 8
0x1404de147 eb1a jmp 0x1404de163
0x1404de149 c685a314000000 mov byte ptr [rbp + 0x14a3], 0
0x1404de150 31d2 xor edx, edx
0x1404de152 e8e9343c02 call 0x1428a1640 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
0x1404de157 4889c1 mov rcx, rax
0x1404de15a 4885c0 test rax, rax
0x1404de15d 0f84b5140000 je 0x1404df618
0x1404de163 48833900 cmp qword ptr [rcx], 0
0x1404de167 0f85d90d0000 jne 0x1404def46
0x1404de16d 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x1404de174 48898d98140000 mov qword ptr [rbp + 0x1498], rcx
0x1404de17b 4883c108 add rcx, 8
0x1404de17f 488b55c8 mov rdx, qword ptr [rbp - 0x38]
0x1404de183 e8688f7300 call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
0x1404de188 4c8bbd78140000 mov r15, qword ptr [rbp + 0x1478]
0x1404de18f 488b8598140000 mov rax, qword ptr [rbp + 0x1498]
0x1404de196 48ff00 inc qword ptr [rax]
0x1404de199 4c8bad50140000 mov r13, qword ptr [rbp + 0x1450]
0x1404de1a0 4531e4 xor r12d, r12d
0x1404de1a3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404de1b0 e84b887300 call 0x140c16a00 ; puffin::are_scopes_on
0x1404de1b5 888598140000 mov byte ptr [rbp + 0x1498], al
0x1404de1bb 84c0 test al, al
0x1404de1bd 0f84ad000000 je 0x1404de270
0x1404de1c3 488d3d18bb5002 lea rdi, [rip + 0x250bb18]
0x1404de1ca 660f1f440000 nop word ptr [rax + rax]
0x1404de1d0 0fbe47ff movsx eax, byte ptr [rdi - 1]
0x1404de1d4 85c0 test eax, eax
0x1404de1d6 7818 js 0x1404de1f0
0x1404de1d8 48ffcf dec rdi
0x1404de1db 83f85c cmp eax, 0x5c
0x1404de1de 7562 jne 0x1404de242
0x1404de1e0 e992000000 jmp 0x1404de277
0x1404de1e5 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404de1f0 0fb64ffe movzx ecx, byte ptr [rdi - 2]
0x1404de1f4 80f9c0 cmp cl, 0xc0
0x1404de1f7 7d22 jge 0x1404de21b
0x1404de1f9 0fb657fd movzx edx, byte ptr [rdi - 3]
0x1404de1fd 80fac0 cmp dl, 0xc0
0x1404de200 7d22 jge 0x1404de224
0x1404de202 440fb647fc movzx r8d, byte ptr [rdi - 4]
0x1404de207 4883c7fc add rdi, -4
0x1404de20b 4183e007 and r8d, 7
0x1404de20f 41c1e006 shl r8d, 6
0x1404de213 83e23f and edx, 0x3f
0x1404de216 4409c2 or edx, r8d
0x1404de219 eb10 jmp 0x1404de22b
0x1404de21b 4883c7fe add rdi, -2
0x1404de21f 83e11f and ecx, 0x1f
0x1404de222 eb0f jmp 0x1404de233
0x1404de224 4883c7fd add rdi, -3
0x1404de228 83e20f and edx, 0xf
0x1404de22b c1e206 shl edx, 6
0x1404de22e 83e13f and ecx, 0x3f
0x1404de231 09d1 or ecx, edx
0x1404de233 c1e106 shl ecx, 6
0x1404de236 243f and al, 0x3f
0x1404de238 0fb6c0 movzx eax, al
0x1404de23b 09c8 or eax, ecx
0x1404de23d 83f85c cmp eax, 0x5c
0x1404de240 7435 je 0x1404de277
0x1404de242 83f82f cmp eax, 0x2f
0x1404de245 7430 je 0x1404de277
0x1404de247 488d0572ba5002 lea rax, [rip + 0x250ba72]
0x1404de24e 4839c7 cmp rdi, rax
0x1404de251 0f8579ffffff jne 0x1404de1d0
0x1404de257 bb22000000 mov ebx, 0x22
0x1404de25c 488d3d5dba5002 lea rdi, [rip + 0x250ba5d]
0x1404de263 eb4a jmp 0x1404de2af
0x1404de265 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404de270 31c0 xor eax, eax
0x1404de272 e9c8000000 jmp 0x1404de33f
0x1404de277 488d0542ba5002 lea rax, [rip + 0x250ba42]
0x1404de27e 4829c7 sub rdi, rax
0x1404de281 48ffc7 inc rdi
0x1404de284 741e je 0x1404de2a4
0x1404de286 488d0533ba5002 lea rax, [rip + 0x250ba33]
0x1404de28d 4883ff22 cmp rdi, 0x22
0x1404de291 7207 jb 0x1404de29a
0x1404de293 740f je 0x1404de2a4
0x1404de295 e941130000 jmp 0x1404df5db
0x1404de29a 803c07bf cmp byte ptr [rdi + rax], 0xbf
0x1404de29e 0f8e37130000 jle 0x1404df5db
0x1404de2a4 bb22000000 mov ebx, 0x22
0x1404de2a9 4829fb sub rbx, rdi
0x1404de2ac 4801c7 add rdi, rax
0x1404de2af e87c9e7300 call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
0x1404de2b4 4889c1 mov rcx, rax
0x1404de2b7 488b00 mov rax, qword ptr [rax]
0x1404de2ba 4885c0 test rax, rax
0x1404de2bd 740f je 0x1404de2ce
0x1404de2bf 83f801 cmp eax, 1
0x1404de2c2 0f85c9120000 jne 0x1404df591
0x1404de2c8 4883c108 add rcx, 8
0x1404de2cc eb13 jmp 0x1404de2e1
0x1404de2ce 31d2 xor edx, edx
0x1404de2d0 e86b333c02 call 0x1428a1640 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
0x1404de2d5 4889c1 mov rcx, rax
0x1404de2d8 4885c0 test rax, rax
0x1404de2db 0f84b0120000 je 0x1404df591
0x1404de2e1 48833900 cmp qword ptr [rcx], 0
0x1404de2e5 0f85040c0000 jne 0x1404deeef
0x1404de2eb 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x1404de2f2 48898d48140000 mov qword ptr [rbp + 0x1448], rcx
0x1404de2f9 4883c108 add rcx, 8
0x1404de2fd 48895c2420 mov qword ptr [rsp + 0x20], rbx
0x1404de302 48c744243000000000 mov qword ptr [rsp + 0x30], 0
0x1404de30b 48c744242801000000 mov qword ptr [rsp + 0x28], 1
0x1404de314 41b80a000000 mov r8d, 0xa
0x1404de31a 488d1508be5002 lea rdx, [rip + 0x250be08]
0x1404de321 4989f9 mov r9, rdi
0x1404de324 e8178a7300 call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
0x1404de329 488b8d48140000 mov rcx, qword ptr [rbp + 0x1448]
0x1404de330 48ff01 inc qword ptr [rcx]
0x1404de333 488985380c0000 mov qword ptr [rbp + 0xc38], rax
0x1404de33a b801000000 mov eax, 1
0x1404de33f 488985300c0000 mov qword ptr [rbp + 0xc30], rax
0x1404de346 4c89e9 mov rcx, r13
0x1404de349 e852f57c00 call 0x140cad8a0 ; bevy_app::app::App::update
0x1404de34e 80bd9814000000 cmp byte ptr [rbp + 0x1498], 0
0x1404de355 7464 je 0x1404de3bb
0x1404de357 e8d49d7300 call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
0x1404de35c 4889c1 mov rcx, rax
0x1404de35f 488b00 mov rax, qword ptr [rax]
0x1404de362 4885c0 test rax, rax
0x1404de365 740f je 0x1404de376
0x1404de367 83f801 cmp eax, 1
0x1404de36a 0f8532120000 jne 0x1404df5a2
0x1404de370 4883c108 add rcx, 8
0x1404de374 eb13 jmp 0x1404de389
0x1404de376 31d2 xor edx, edx
0x1404de378 e8c3323c02 call 0x1428a1640 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
0x1404de37d 4889c1 mov rcx, rax
0x1404de380 4885c0 test rax, rax
0x1404de383 0f8419120000 je 0x1404df5a2
0x1404de389 48833900 cmp qword ptr [rcx], 0
0x1404de38d 0f856d0b0000 jne 0x1404def00
0x1404de393 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x1404de39a 48898d98140000 mov qword ptr [rbp + 0x1498], rcx
0x1404de3a1 4883c108 add rcx, 8
0x1404de3a5 488b95380c0000 mov rdx, qword ptr [rbp + 0xc38]
0x1404de3ac e83f8d7300 call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
0x1404de3b1 488b8598140000 mov rax, qword ptr [rbp + 0x1498]
0x1404de3b8 48ff00 inc qword ptr [rax]
0x1404de3bb 4584e4 test r12b, r12b
0x1404de3be 0f851c010000 jne 0x1404de4e0
0x1404de3c4 488d8d300c0000 lea rcx, [rbp + 0xc30]
0x1404de3cb 488b9590130000 mov rdx, qword ptr [rbp + 0x1390]
0x1404de3d2 e839f5bfff call 0x1400dd910 ; tiny_glade::egui_backend::EguiBackend::end_frame
0x1404de3d7 488b85a00c0000 mov rax, qword ptr [rbp + 0xca0]
0x1404de3de 48898570060000 mov qword ptr [rbp + 0x670], rax
0x1404de3e5 0f1085900c0000 movups xmm0, xmmword ptr [rbp + 0xc90]
0x1404de3ec 0f298560060000 movaps xmmword ptr [rbp + 0x660], xmm0
0x1404de3f3 0f1085800c0000 movups xmm0, xmmword ptr [rbp + 0xc80]
0x1404de3fa 0f298550060000 movaps xmmword ptr [rbp + 0x650], xmm0
0x1404de401 0f1085700c0000 movups xmm0, xmmword ptr [rbp + 0xc70]
0x1404de408 0f298540060000 movaps xmmword ptr [rbp + 0x640], xmm0
0x1404de40f f30f6f85300c0000 movdqu xmm0, xmmword ptr [rbp + 0xc30]
0x1404de417 f30f6f8d400c0000 movdqu xmm1, xmmword ptr [rbp + 0xc40]
0x1404de41f f30f6f95500c0000 movdqu xmm2, xmmword ptr [rbp + 0xc50]
0x1404de427 f30f6f9d600c0000 movdqu xmm3, xmmword ptr [rbp + 0xc60]
0x1404de42f 660f7f9d30060000 movdqa xmmword ptr [rbp + 0x630], xmm3
0x1404de437 660f7f9520060000 movdqa xmmword ptr [rbp + 0x620], xmm2
0x1404de43f 660f7f8d10060000 movdqa xmmword ptr [rbp + 0x610], xmm1
0x1404de447 660f7f8500060000 movdqa xmmword ptr [rbp + 0x600], xmm0
0x1404de44f 488d8d300c0000 lea rcx, [rbp + 0xc30]
0x1404de456 488b9518140000 mov rdx, qword ptr [rbp + 0x1418]
0x1404de45d 4c8d8500060000 lea r8, [rbp + 0x600]
0x1404de464 e8a775eeff call 0x1403c5a10 ; std::sync::mpmc::Sender::send<enum2$<tiny_glade::event_loop::GameEvent> >
0x1404de469 488b85300c0000 mov rax, qword ptr [rbp + 0xc30]
0x1404de470 48b90700000000000080 movabs rcx, 0x8000000000000007
0x1404de47a 4839c8 cmp rax, rcx
0x1404de47d 7461 je 0x1404de4e0
0x1404de47f 48b90000000000000080 movabs rcx, 0x8000000000000000
0x1404de489 4831c8 xor rax, rcx
0x1404de48c 4883f807 cmp rax, 7
0x1404de490 b903000000 mov ecx, 3
0x1404de495 480f43c1 cmovae rax, rcx
0x1404de499 4883f804 cmp rax, 4
0x1404de49d 7414 je 0x1404de4b3
0x1404de49f 4883f803 cmp rax, 3
0x1404de4a3 753b jne 0x1404de4e0
0x1404de4a5 488d8d300c0000 lea rcx, [rbp + 0xc30]
0x1404de4ac e85f281100 call 0x1405f0d10 ; core::ptr::drop_in_place<egui::data::output::PlatformOutput>
0x1404de4b1 eb2d jmp 0x1404de4e0
0x1404de4b3 488b95380c0000 mov rdx, qword ptr [rbp + 0xc38]
0x1404de4ba 48b80200000000000080 movabs rax, 0x8000000000000002
0x1404de4c4 4839c2 cmp rdx, rax
0x1404de4c7 7c17 jl 0x1404de4e0
0x1404de4c9 4885d2 test rdx, rdx
0x1404de4cc 7412 je 0x1404de4e0
0x1404de4ce 488b8d400c0000 mov rcx, qword ptr [rbp + 0xc40]
0x1404de4d5 41b801000000 mov r8d, 1
0x1404de4db e840571300 call 0x140613c20 ; __rust_dealloc
0x1404de4e0 488b8588130000 mov rax, qword ptr [rbp + 0x1388]
0x1404de4e7 48833800 cmp qword ptr [rax], 0
0x1404de4eb 740c je 0x1404de4f9
0x1404de4ed 488b8d88130000 mov rcx, qword ptr [rbp + 0x1388]
0x1404de4f4 e8978ac0ff call 0x1400e6f90 ; steamworks::Client::run_callbacks<steamworks::ClientManager>
0x1404de4f9 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404de500 48baffa3086aee51a4bb movabs rdx, 0xbba451ee6a08a3ff
0x1404de50a 49b89df2bcfe158d008d movabs r8, 0x8d008d15febcf29d
0x1404de514 e8d7f1bfff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404de519 4885c0 test rax, rax
0x1404de51c 7442 je 0x1404de560
0x1404de51e 49399748020000 cmp qword ptr [r15 + 0x248], rdx
0x1404de525 7639 jbe 0x1404de560
0x1404de527 498b8740020000 mov rax, qword ptr [r15 + 0x240]
0x1404de52e 488b04d0 mov rax, qword ptr [rax + rdx*8]
0x1404de532 4885c0 test rax, rax
0x1404de535 7429 je 0x1404de560
0x1404de537 498b8f10020000 mov rcx, qword ptr [r15 + 0x210]
0x1404de53e 48f7d0 not rax
0x1404de541 488d0440 lea rax, [rax + rax*2]
0x1404de545 48c1e005 shl rax, 5
0x1404de549 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
0x1404de54f 0f95c0 setne al
0x1404de552 00c0 add al, al
0x1404de554 eb0c jmp 0x1404de562
0x1404de556 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404de560 31c0 xor eax, eax
0x1404de562 4969ce00ca9a3b imul rcx, r14, 0x3b9aca00
0x1404de569 8b9568140000 mov edx, dword ptr [rbp + 0x1468]
0x1404de56f 4801d6 add rsi, rdx
0x1404de572 4801ce add rsi, rcx
0x1404de575 660f7ef2 movd edx, xmm6
0x1404de579 48c1e220 shl rdx, 0x20
0x1404de57d 488b8d60140000 mov rcx, qword ptr [rbp + 0x1460]
0x1404de584 4809d1 or rcx, rdx
0x1404de587 88442438 mov byte ptr [rsp + 0x38], al
0x1404de58b 488b8590130000 mov rax, qword ptr [rbp + 0x1390]
0x1404de592 4889442430 mov qword ptr [rsp + 0x30], rax
0x1404de597 488b8520140000 mov rax, qword ptr [rbp + 0x1420]
0x1404de59e 4889442428 mov qword ptr [rsp + 0x28], rax
0x1404de5a3 488b8538130000 mov rax, qword ptr [rbp + 0x1338]
0x1404de5aa 4889442420 mov qword ptr [rsp + 0x20], rax
0x1404de5af 4c89ea mov rdx, r13
0x1404de5b2 4989f0 mov r8, rsi
0x1404de5b5 4c8b8d30130000 mov r9, qword ptr [rbp + 0x1330]
0x1404de5bc e84f9a0000 call 0x1404e8010 ; tiny_glade::render_loop::render
0x1404de5c1 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404de5c8 48ba2a26366264338ef0 movabs rdx, 0xf08e33646236262a
0x1404de5d2 49b827c1f9ddf74066d9 movabs r8, 0xd96640f7ddf9c127
0x1404de5dc e80ff1bfff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404de5e1 4885c0 test rax, rax
0x1404de5e4 0f84b40d0000 je 0x1404df39e
0x1404de5ea 49399748020000 cmp qword ptr [r15 + 0x248], rdx
0x1404de5f1 0f86a70d0000 jbe 0x1404df39e
0x1404de5f7 498b8740020000 mov rax, qword ptr [r15 + 0x240]
0x1404de5fe 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404de602 4885c9 test rcx, rcx
0x1404de605 0f84930d0000 je 0x1404df39e
0x1404de60b 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x1404de612 48f7d1 not rcx
0x1404de615 488d0c49 lea rcx, [rcx + rcx*2]
0x1404de619 48c1e105 shl rcx, 5
0x1404de61d 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404de623 0f84750d0000 je 0x1404df39e
0x1404de629 4801c8 add rax, rcx
0x1404de62c 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1404de630 418b97e0050000 mov edx, dword ptr [r15 + 0x5e0]
0x1404de637 89505c mov dword ptr [rax + 0x5c], edx
0x1404de63a ff8108010000 inc dword ptr [rcx + 0x108]
0x1404de640 e8bbcbefff call 0x1403db200 ; tiny_glade::log_time_to_first_frame
0x1404de645 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404de64c 48ba407cd487e7567b3d movabs rdx, 0x3d7b56e787d47c40
0x1404de656 49b8c7bf438d0ab1b067 movabs r8, 0x67b0b10a8d43bfc7
0x1404de660 e88bf0bfff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404de665 4885c0 test rax, rax
0x1404de668 0f84410d0000 je 0x1404df3af
0x1404de66e 49399748020000 cmp qword ptr [r15 + 0x248], rdx
0x1404de675 0f86340d0000 jbe 0x1404df3af
0x1404de67b 498b8740020000 mov rax, qword ptr [r15 + 0x240]
0x1404de682 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404de686 4885c9 test rcx, rcx
0x1404de689 0f84200d0000 je 0x1404df3af
0x1404de68f 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x1404de696 48f7d1 not rcx
0x1404de699 488d0c49 lea rcx, [rcx + rcx*2]
0x1404de69d 48c1e105 shl rcx, 5
0x1404de6a1 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404de6a7 0f84020d0000 je 0x1404df3af
0x1404de6ad 4801c8 add rax, rcx
0x1404de6b0 488b4010 mov rax, qword ptr [rax + 0x10]
0x1404de6b4 f30f10b098000000 movss xmm6, dword ptr [rax + 0x98]
0x1404de6bc 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404de6c3 48ba7a9cb77f61ae8dd3 movabs rdx, 0xd38dae617fb79c7a
0x1404de6cd 49b8ae1ead5d5cdfaa22 movabs r8, 0x22aadf5c5dad1eae
0x1404de6d7 e814f0bfff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404de6dc 4885c0 test rax, rax
0x1404de6df 0f84eb000000 je 0x1404de7d0
0x1404de6e5 49399748020000 cmp qword ptr [r15 + 0x248], rdx
0x1404de6ec 0f86de000000 jbe 0x1404de7d0
0x1404de6f2 498b8740020000 mov rax, qword ptr [r15 + 0x240]
0x1404de6f9 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404de6fd 4885c9 test rcx, rcx
0x1404de700 0f84ca000000 je 0x1404de7d0
0x1404de706 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x1404de70d 48f7d1 not rcx
0x1404de710 488d0c49 lea rcx, [rcx + rcx*2]
0x1404de714 48c1e105 shl rcx, 5
0x1404de718 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404de71e 0f84ac000000 je 0x1404de7d0
0x1404de724 4801c8 add rax, rcx
0x1404de727 488b7010 mov rsi, qword ptr [rax + 0x10]
0x1404de72b 4889f1 mov rcx, rsi
0x1404de72e e88d144600 call 0x14093fbc0 ; country_core::resources::gui::EscMenuState::is_open
0x1404de733 84c0 test al, al
0x1404de735 0f8495000000 je 0x1404de7d0
0x1404de73b 4889f1 mov rcx, rsi
0x1404de73e e81d154600 call 0x14093fc60 ; country_core::resources::gui::EscMenuState::can_limit_frame_rate
0x1404de743 84c0 test al, al
0x1404de745 0f8485000000 je 0x1404de7d0
0x1404de74b 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404de752 48ba5951918cf9e83168 movabs rdx, 0x6831e8f98c915159
0x1404de75c 49b8250700e479c415d5 movabs r8, 0xd515c479e4000725
0x1404de766 e885efbfff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404de76b 4885c0 test rax, rax
0x1404de76e 7443 je 0x1404de7b3
0x1404de770 49399748020000 cmp qword ptr [r15 + 0x248], rdx
0x1404de777 763a jbe 0x1404de7b3
0x1404de779 498b8740020000 mov rax, qword ptr [r15 + 0x240]
0x1404de780 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404de784 4885c9 test rcx, rcx
0x1404de787 742a je 0x1404de7b3
0x1404de789 498b8710020000 mov rax, qword ptr [r15 + 0x210]
0x1404de790 48f7d1 not rcx
0x1404de793 488d0c49 lea rcx, [rcx + rcx*2]
0x1404de797 48c1e105 shl rcx, 5
0x1404de79b 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404de7a1 7410 je 0x1404de7b3
0x1404de7a3 4801c8 add rax, rcx
0x1404de7a6 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1404de7aa e841094400 call 0x14091f0f0 ; country_core::meta_commander::MetaCommander::is_profiling
0x1404de7af 84c0 test al, al
0x1404de7b1 751d jne 0x1404de7d0
0x1404de7b3 410f2ef3 ucomiss xmm6, xmm11
0x1404de7b7 660f6e0d0d314f02 movd xmm1, dword ptr [rip + 0x24f310d]
0x1404de7bf 7612 jbe 0x1404de7d3
0x1404de7c1 f30f5d3503314f02 minss xmm6, dword ptr [rip + 0x24f3103]
0x1404de7c9 0f1f8000000000 nop dword ptr [rax]
0x1404de7d0 0f28ce movaps xmm1, xmm6
0x1404de7d3 488b8d40130000 mov rcx, qword ptr [rbp + 0x1340]
0x1404de7da e87120d9ff call 0x140270850 ; tiny_glade::fps_limiter::FpsLimiter::frame
0x1404de7df 4883bde013000000 cmp qword ptr [rbp + 0x13e0], 0
0x1404de7e7 7464 je 0x1404de84d
0x1404de7e9 e842997300 call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
0x1404de7ee 4889c1 mov rcx, rax
0x1404de7f1 488b00 mov rax, qword ptr [rax]
0x1404de7f4 4885c0 test rax, rax
0x1404de7f7 740f je 0x1404de808
0x1404de7f9 83f801 cmp eax, 1
0x1404de7fc 0f856d0d0000 jne 0x1404df56f
0x1404de802 4883c108 add rcx, 8
0x1404de806 eb13 jmp 0x1404de81b
0x1404de808 31d2 xor edx, edx
0x1404de80a e8312e3c02 call 0x1428a1640 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
0x1404de80f 4889c1 mov rcx, rax
0x1404de812 4885c0 test rax, rax
0x1404de815 0f84540d0000 je 0x1404df56f
0x1404de81b 48833900 cmp qword ptr [rcx], 0
0x1404de81f 0f85a8060000 jne 0x1404deecd
0x1404de825 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x1404de82c 48898d98140000 mov qword ptr [rbp + 0x1498], rcx
0x1404de833 4883c108 add rcx, 8
0x1404de837 488b95e8130000 mov rdx, qword ptr [rbp + 0x13e8]
0x1404de83e e8ad887300 call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
0x1404de843 488b8598140000 mov rax, qword ptr [rbp + 0x1498]
0x1404de84a 48ff00 inc qword ptr [rax]
0x1404de84d 488b85a8130000 mov rax, qword ptr [rbp + 0x13a8]
0x1404de854 803800 cmp byte ptr [rax], 0
0x1404de857 756c jne 0x1404de8c5
0x1404de859 488b8d20140000 mov rcx, qword ptr [rbp + 0x1420]
0x1404de860 e80b077300 call 0x140c0ef70 ; rhapsody::raw::swapchain::Swapchain::any_present_successful
0x1404de865 4883bd8013000002 cmp qword ptr [rbp + 0x1380], 2
0x1404de86d 0f93c1 setae cl
0x1404de870 08c8 or al, cl
0x1404de872 3c01 cmp al, 1
0x1404de874 754f jne 0x1404de8c5
0x1404de876 488b8578140000 mov rax, qword ptr [rbp + 0x1478]
0x1404de87d 488b88e8100000 mov rcx, qword ptr [rax + 0x10e8]
0x1404de884 488b4110 mov rax, qword ptr [rcx + 0x10]
0x1404de888 488b5118 mov rdx, qword ptr [rcx + 0x18]
0x1404de88c f048ff00 lock inc qword ptr [rax]
0x1404de890 0f8ec40d0000 jle 0x1404df65a
0x1404de896 4883c120 add rcx, 0x20
0x1404de89a 488995380c0000 mov qword ptr [rbp + 0xc38], rdx
0x1404de8a1 488985300c0000 mov qword ptr [rbp + 0xc30], rax
0x1404de8a8 c685400c000001 mov byte ptr [rbp + 0xc40], 1
0x1404de8af 488d95300c0000 lea rdx, [rbp + 0xc30]
0x1404de8b6 e8f5f60500 call 0x14053dfb0 ; winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread<winit::platform_impl::platform::window::impl$0::set_visible::closure_env$0>
0x1404de8bb 488b85a8130000 mov rax, qword ptr [rbp + 0x13a8]
0x1404de8c2 c60001 mov byte ptr [rax], 1
0x1404de8c5 31c0 xor eax, eax
0x1404de8c7 b101 mov cl, 1
0x1404de8c9 f00fb00dc0621503 lock cmpxchg byte ptr [rip + 0x31562c0], cl
0x1404de8d1 0f8572040000 jne 0x1404ded49
0x1404de8d7 488b0542db1403 mov rax, qword ptr [rip + 0x314db42]
0x1404de8de 488b00 mov rax, qword ptr [rax]
0x1404de8e1 48d1e0 shl rax, 1
0x1404de8e4 4885c0 test rax, rax
0x1404de8e7 0f856d040000 jne 0x1404ded5a
0x1404de8ed 31c0 xor eax, eax
0x1404de8ef 0fb60d9c621503 movzx ecx, byte ptr [rip + 0x315629c]
0x1404de8f6 84c9 test cl, cl
0x1404de8f8 0f85120c0000 jne 0x1404df510
0x1404de8fe 0fb61d8e621503 movzx ebx, byte ptr [rip + 0x315628e]
0x1404de905 c6058762150300 mov byte ptr [rip + 0x3156287], 0
0x1404de90c 84c0 test al, al
0x1404de90e 7516 jne 0x1404de926
0x1404de910 488b0509db1403 mov rax, qword ptr [rip + 0x314db09]
0x1404de917 488b00 mov rax, qword ptr [rax]
0x1404de91a 48d1e0 shl rax, 1
0x1404de91d 4885c0 test rax, rax
0x1404de920 0f856f040000 jne 0x1404ded95
0x1404de926 31c0 xor eax, eax
0x1404de928 860563621503 xchg byte ptr [rip + 0x3156263], al
0x1404de92e 3c02 cmp al, 2
0x1404de930 0f843f040000 je 0x1404ded75
0x1404de936 84db test bl, bl
0x1404de938 0f8487aeffff je 0x1404d97c5
0x1404de93e 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404de945 48ba407cd487e7567b3d movabs rdx, 0x3d7b56e787d47c40
0x1404de94f 49b8c7bf438d0ab1b067 movabs r8, 0x67b0b10a8d43bfc7
0x1404de959 e892edbfff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404de95e 4885c0 test rax, rax
0x1404de961 4c8b8578140000 mov r8, qword ptr [rbp + 0x1478]
0x1404de968 0f8435080000 je 0x1404df1a3
0x1404de96e 49399048020000 cmp qword ptr [r8 + 0x248], rdx
0x1404de975 0f8628080000 jbe 0x1404df1a3
0x1404de97b 498b8040020000 mov rax, qword ptr [r8 + 0x240]
0x1404de982 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404de986 4885c9 test rcx, rcx
0x1404de989 0f8414080000 je 0x1404df1a3
0x1404de98f 498b8010020000 mov rax, qword ptr [r8 + 0x210]
0x1404de996 48f7d1 not rcx
0x1404de999 488d0c49 lea rcx, [rcx + rcx*2]
0x1404de99d 48c1e105 shl rcx, 5
0x1404de9a1 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404de9a7 0f84f6070000 je 0x1404df1a3
0x1404de9ad 4801c8 add rax, rcx
0x1404de9b0 488b7010 mov rsi, qword ptr [rax + 0x10]
0x1404de9b4 418b80e0050000 mov eax, dword ptr [r8 + 0x5e0]
0x1404de9bb 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404de9c2 48bacfde921b5889d12e movabs rdx, 0x2ed189581b92decf
0x1404de9cc 49b8385f069693d9d315 movabs r8, 0x15d3d99396065f38
0x1404de9d6 e815edbfff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404de9db 4885c0 test rax, rax
0x1404de9de 4c8b8578140000 mov r8, qword ptr [rbp + 0x1478]
0x1404de9e5 0f84c9070000 je 0x1404df1b4
0x1404de9eb 49399048020000 cmp qword ptr [r8 + 0x248], rdx
0x1404de9f2 0f86bc070000 jbe 0x1404df1b4
0x1404de9f8 498b8040020000 mov rax, qword ptr [r8 + 0x240]
0x1404de9ff 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404dea03 4885c9 test rcx, rcx
0x1404dea06 0f84a8070000 je 0x1404df1b4
0x1404dea0c 498b8010020000 mov rax, qword ptr [r8 + 0x210]
0x1404dea13 48f7d1 not rcx
0x1404dea16 488d0c49 lea rcx, [rcx + rcx*2]
0x1404dea1a 48c1e105 shl rcx, 5
0x1404dea1e 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dea24 0f848a070000 je 0x1404df1b4
0x1404dea2a f6c302 test bl, 2
0x1404dea2d 7423 je 0x1404dea52
0x1404dea2f 4801c8 add rax, rcx
0x1404dea32 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1404dea36 e865924a00 call 0x140987ca0 ; country_core::render::impl$0::deref
0x1404dea3b 440fb686a8000000 movzx r8d, byte ptr [rsi + 0xa8]
0x1404dea43 488b8d20140000 mov rcx, qword ptr [rbp + 0x1420]
0x1404dea4a 4889c2 mov rdx, rax
0x1404dea4d e84efe7200 call 0x140c0e8a0 ; rhapsody::raw::swapchain::Swapchain::set_vsync
0x1404dea52 f6c305 test bl, 5
0x1404dea55 0f8465010000 je 0x1404debc0
0x1404dea5b 31ff xor edi, edi
0x1404dea5d 837e6801 cmp dword ptr [rsi + 0x68], 1
0x1404dea61 7512 jne 0x1404dea75
0x1404dea63 660f10466c movupd xmm0, xmmword ptr [rsi + 0x6c]
0x1404dea68 660f2985e0130000 movapd xmmword ptr [rbp + 0x13e0], xmm0
0x1404dea70 bf01000000 mov edi, 1
0x1404dea75 0fb65e7c movzx ebx, byte ptr [rsi + 0x7c]
0x1404dea79 440fb6767d movzx r14d, byte ptr [rsi + 0x7d]
0x1404dea7e 4c8b7e60 mov r15, qword ptr [rsi + 0x60]
0x1404dea82 488b4638 mov rax, qword ptr [rsi + 0x38]
0x1404dea86 4889c1 mov rcx, rax
0x1404dea89 48f7d9 neg rcx
0x1404dea8c 4c8d85300c0000 lea r8, [rbp + 0xc30]
0x1404dea93 702d jo 0x1404deac2
0x1404dea95 48b90100000000000080 movabs rcx, 0x8000000000000001
0x1404dea9f 4839c8 cmp rax, rcx
0x1404deaa2 7432 je 0x1404dead6
0x1404deaa4 488d5638 lea rdx, [rsi + 0x38]
0x1404deaa8 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404deaaf e8dcd28900 call 0x140d7bd90 ; alloc::string::impl$6::clone
0x1404deab4 488b8500060000 mov rax, qword ptr [rbp + 0x600]
0x1404deabb 4c8d85300c0000 lea r8, [rbp + 0xc30]
0x1404deac2 0f104650 movups xmm0, xmmword ptr [rsi + 0x50]
0x1404deac6 488d8d08060000 lea rcx, [rbp + 0x608]
0x1404deacd f30f6f09 movdqu xmm1, xmmword ptr [rcx]
0x1404dead1 660f7f4dc0 movdqa xmmword ptr [rbp - 0x40], xmm1
0x1404dead6 488985380c0000 mov qword ptr [rbp + 0xc38], rax
0x1404deadd 660f6f4dc0 movdqa xmm1, xmmword ptr [rbp - 0x40]
0x1404deae2 488d85380c0000 lea rax, [rbp + 0xc38]
0x1404deae9 f30f7f4808 movdqu xmmword ptr [rax + 8], xmm1
0x1404deaee 0f1185500c0000 movups xmmword ptr [rbp + 0xc50], xmm0
0x1404deaf5 4c89bd600c0000 mov qword ptr [rbp + 0xc60], r15
0x1404deafc 89bd680c0000 mov dword ptr [rbp + 0xc68], edi
0x1404deb02 660f2885e0130000 movapd xmm0, xmmword ptr [rbp + 0x13e0]
0x1404deb0a 660f114034 movupd xmmword ptr [rax + 0x34], xmm0
0x1404deb0f 889d7c0c0000 mov byte ptr [rbp + 0xc7c], bl
0x1404deb15 4488b57d0c0000 mov byte ptr [rbp + 0xc7d], r14b
0x1404deb1c 48b80400000000000080 movabs rax, 0x8000000000000004
0x1404deb26 488985300c0000 mov qword ptr [rbp + 0xc30], rax
0x1404deb2d 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404deb34 488b9518140000 mov rdx, qword ptr [rbp + 0x1418]
0x1404deb3b e8d06eeeff call 0x1403c5a10 ; std::sync::mpmc::Sender::send<enum2$<tiny_glade::event_loop::GameEvent> >
0x1404deb40 488b8500060000 mov rax, qword ptr [rbp + 0x600]
0x1404deb47 48b90700000000000080 movabs rcx, 0x8000000000000007
0x1404deb51 4839c8 cmp rax, rcx
0x1404deb54 746a je 0x1404debc0
0x1404deb56 48b90000000000000080 movabs rcx, 0x8000000000000000
0x1404deb60 4831c8 xor rax, rcx
0x1404deb63 4883f807 cmp rax, 7
0x1404deb67 b903000000 mov ecx, 3
0x1404deb6c 480f43c1 cmovae rax, rcx
0x1404deb70 4883f804 cmp rax, 4
0x1404deb74 7414 je 0x1404deb8a
0x1404deb76 4883f803 cmp rax, 3
0x1404deb7a 7544 jne 0x1404debc0
0x1404deb7c 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404deb83 e888211100 call 0x1405f0d10 ; core::ptr::drop_in_place<egui::data::output::PlatformOutput>
0x1404deb88 eb36 jmp 0x1404debc0
0x1404deb8a 488b9508060000 mov rdx, qword ptr [rbp + 0x608]
0x1404deb91 48b80200000000000080 movabs rax, 0x8000000000000002
0x1404deb9b 4839c2 cmp rdx, rax
0x1404deb9e 7c20 jl 0x1404debc0
0x1404deba0 4885d2 test rdx, rdx
0x1404deba3 741b je 0x1404debc0
0x1404deba5 488b8d10060000 mov rcx, qword ptr [rbp + 0x610]
0x1404debac 41b801000000 mov r8d, 1
0x1404debb2 e869501300 call 0x140613c20 ; __rust_dealloc
0x1404debb7 660f1f840000000000 nop word ptr [rax + rax]
0x1404debc0 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404debc7 48baa01cbf3bbb4f550a movabs rdx, 0xa554fbb3bbf1ca0
0x1404debd1 49b89d8ebb86a993b929 movabs r8, 0x29b993a986bb8e9d
0x1404debdb e810ebbfff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404debe0 4885c0 test rax, rax
0x1404debe3 4c8b8578140000 mov r8, qword ptr [rbp + 0x1478]
0x1404debea 0f84ed050000 je 0x1404df1dd
0x1404debf0 49399048020000 cmp qword ptr [r8 + 0x248], rdx
0x1404debf7 0f86e0050000 jbe 0x1404df1dd
0x1404debfd 498b8040020000 mov rax, qword ptr [r8 + 0x240]
0x1404dec04 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404dec08 4885c9 test rcx, rcx
0x1404dec0b 0f84cc050000 je 0x1404df1dd
0x1404dec11 498b8010020000 mov rax, qword ptr [r8 + 0x210]
0x1404dec18 48f7d1 not rcx
0x1404dec1b 488d0c49 lea rcx, [rcx + rcx*2]
0x1404dec1f 48c1e105 shl rcx, 5
0x1404dec23 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404dec29 0f84ae050000 je 0x1404df1dd
0x1404dec2f 4801c8 add rax, rcx
0x1404dec32 488b7810 mov rdi, qword ptr [rax + 0x10]
0x1404dec36 418b88e0050000 mov ecx, dword ptr [r8 + 0x5e0]
0x1404dec3d 89485c mov dword ptr [rax + 0x5c], ecx
0x1404dec40 488b5f30 mov rbx, qword ptr [rdi + 0x30]
0x1404dec44 4c8b7740 mov r14, qword ptr [rdi + 0x40]
0x1404dec48 483b5f20 cmp rbx, qword ptr [rdi + 0x20]
0x1404dec4c 7510 jne 0x1404dec5e
0x1404dec4e 488d4f20 lea rcx, [rdi + 0x20]
0x1404dec52 488d15cf445102 lea rdx, [rip + 0x25144cf]
0x1404dec59 e8123c0800 call 0x140562870 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::glade_border::ShowDemoBorderOverlay>,alloc::alloc::Global>
0x1404dec5e 488b4728 mov rax, qword ptr [rdi + 0x28]
0x1404dec62 4c8934d8 mov qword ptr [rax + rbx*8], r14
0x1404dec66 48ffc3 inc rbx
0x1404dec69 48895f30 mov qword ptr [rdi + 0x30], rbx
0x1404dec6d 48ff4740 inc qword ptr [rdi + 0x40]
0x1404dec71 488d8d300c0000 lea rcx, [rbp + 0xc30]
0x1404dec78 4889f2 mov rdx, rsi
0x1404dec7b e8d090ffff call 0x1404d7d50 ; country_core::render::render_config::impl$38::clone
0x1404dec80 c6857414000001 mov byte ptr [rbp + 0x1474], 1
0x1404dec87 488b8d88140000 mov rcx, qword ptr [rbp + 0x1488]
0x1404dec8e 48ba036149a6340c83fe movabs rdx, 0xfe830c34a6496103
0x1404dec98 49b88d08295fccba6ed6 movabs r8, 0xd66ebacc5f29088d
0x1404deca2 e849eabfff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
0x1404deca7 4885c0 test rax, rax
0x1404decaa 4c8b8578140000 mov r8, qword ptr [rbp + 0x1478]
0x1404decb1 488db5300c0000 lea rsi, [rbp + 0xc30]
0x1404decb8 0f8407050000 je 0x1404df1c5
0x1404decbe 49399048020000 cmp qword ptr [r8 + 0x248], rdx
0x1404decc5 0f86fa040000 jbe 0x1404df1c5
0x1404deccb 498b8040020000 mov rax, qword ptr [r8 + 0x240]
0x1404decd2 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x1404decd6 4885c9 test rcx, rcx
0x1404decd9 0f84e6040000 je 0x1404df1c5
0x1404decdf 498b8010020000 mov rax, qword ptr [r8 + 0x210]
0x1404dece6 48f7d1 not rcx
0x1404dece9 488d0c49 lea rcx, [rcx + rcx*2]
0x1404deced 48c1e105 shl rcx, 5
0x1404decf1 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1404decf7 0f84c8040000 je 0x1404df1c5
0x1404decfd 4801c8 add rax, rcx
0x1404ded00 488b5010 mov rdx, qword ptr [rax + 0x10]
0x1404ded04 418b88e0050000 mov ecx, dword ptr [r8 + 0x5e0]
0x1404ded0b 89485c mov dword ptr [rax + 0x5c], ecx
0x1404ded0e 31c0 xor eax, eax
0x1404ded10 48899598140000 mov qword ptr [rbp + 0x1498], rdx
0x1404ded17 483b02 cmp rax, qword ptr [rdx]
0x1404ded1a 0f8090aaffff jo 0x1404d97b0
0x1404ded20 488b8d98140000 mov rcx, qword ptr [rbp + 0x1498]
0x1404ded27 e854ee1100 call 0x1405fdb80 ; core::ptr::drop_in_place<country_core::render::render_config::VideoSettings>
0x1404ded2c e97faaffff jmp 0x1404d97b0
0x1404ded31 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404ded38 488b9568130000 mov rdx, qword ptr [rbp + 0x1368]
0x1404ded3f e88cf2d8ff call 0x14026dfd0 ; tiny_glade::event_loop::EguiEventIo::take_egui_input
0x1404ded44 e905f2ffff jmp 0x1404ddf4e
0x1404ded49 488d0d415e1503 lea rcx, [rip + 0x3155e41]
0x1404ded50 e8fb993f02 call 0x1428d8750 ; std::sys::sync::mutex::futex::Mutex::lock_contended
0x1404ded55 e97dfbffff jmp 0x1404de8d7
0x1404ded5a e851993f02 call 0x1428d86b0 ; std::panicking::panic_count::is_zero_slow_path
0x1404ded5f 3401 xor al, 1
0x1404ded61 0fb60d2a5e1503 movzx ecx, byte ptr [rip + 0x3155e2a]
0x1404ded68 84c9 test cl, cl
0x1404ded6a 0f848efbffff je 0x1404de8fe
0x1404ded70 e99b070000 jmp 0x1404df510
0x1404ded75 488d0d155e1503 lea rcx, [rip + 0x3155e15]
0x1404ded7c e88f9a3f02 call 0x1428d8810 ; std::sys::sync::mutex::futex::Mutex::wake
0x1404ded81 e9b0fbffff jmp 0x1404de936
0x1404ded86 4889f9 mov rcx, rdi
0x1404ded89 31d2 xor edx, edx
0x1404ded8b e800ee3e02 call 0x1428cdb90 ; parking_lot::raw_mutex::RawMutex::unlock_slow
0x1404ded90 e9c2f0ffff jmp 0x1404dde57
0x1404ded95 e816993f02 call 0x1428d86b0 ; std::panicking::panic_count::is_zero_slow_path
0x1404ded9a 84c0 test al, al
0x1404ded9c 0f8584fbffff jne 0x1404de926
0x1404deda2 c605e95d150301 mov byte ptr [rip + 0x3155de9], 1
0x1404deda9 e978fbffff jmp 0x1404de926
0x1404dedae be01000000 mov esi, 1
0x1404dedb3 488d8db0130000 lea rcx, [rbp + 0x13b0]
0x1404dedba e831481200 call 0x1406035f0 ; core::ptr::drop_in_place<alloc::vec::Vec<enum2$<tiny_glade::event_loop::OsWindowEvent>,alloc::alloc::Global> >
0x1404dedbf 4889f0 mov rax, rsi
0x1404dedc2 0f28b5b0140000 movaps xmm6, xmmword ptr [rbp + 0x14b0]
0x1404dedc9 0f28bdc0140000 movaps xmm7, xmmword ptr [rbp + 0x14c0]
0x1404dedd0 440f2885d0140000 movaps xmm8, xmmword ptr [rbp + 0x14d0]
0x1404dedd8 440f288de0140000 movaps xmm9, xmmword ptr [rbp + 0x14e0]
0x1404dede0 440f2895f0140000 movaps xmm10, xmmword ptr [rbp + 0x14f0]
0x1404dede8 440f289d00150000 movaps xmm11, xmmword ptr [rbp + 0x1500]
0x1404dedf0 440f28a510150000 movaps xmm12, xmmword ptr [rbp + 0x1510]
0x1404dedf8 440f28ad20150000 movaps xmm13, xmmword ptr [rbp + 0x1520]
0x1404dee00 440f28b530150000 movaps xmm14, xmmword ptr [rbp + 0x1530]
0x1404dee08 440f28bd40150000 movaps xmm15, xmmword ptr [rbp + 0x1540]
0x1404dee10 4881c4d8150000 add rsp, 0x15d8
0x1404dee17 5b pop rbx
0x1404dee18 5f pop rdi
0x1404dee19 5e pop rsi
0x1404dee1a 415c pop r12
0x1404dee1c 415d pop r13
0x1404dee1e 415e pop r14
0x1404dee20 415f pop r15
0x1404dee22 5d pop rbp
0x1404dee23 c3 ret 
0x1404dee24 48b80500000000000080 movabs rax, 0x8000000000000005
0x1404dee2e 488985300c0000 mov qword ptr [rbp + 0xc30], rax
0x1404dee35 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404dee3c 4c8d85300c0000 lea r8, [rbp + 0xc30]
0x1404dee43 488b9518140000 mov rdx, qword ptr [rbp + 0x1418]
0x1404dee4a e8c16beeff call 0x1403c5a10 ; std::sync::mpmc::Sender::send<enum2$<tiny_glade::event_loop::GameEvent> >
0x1404dee4f 488b8500060000 mov rax, qword ptr [rbp + 0x600]
0x1404dee56 48b90700000000000080 movabs rcx, 0x8000000000000007
0x1404dee60 4839c8 cmp rax, rcx
0x1404dee63 7461 je 0x1404deec6
0x1404dee65 48b90000000000000080 movabs rcx, 0x8000000000000000
0x1404dee6f 4831c8 xor rax, rcx
0x1404dee72 4883f807 cmp rax, 7
0x1404dee76 b903000000 mov ecx, 3
0x1404dee7b 480f42c8 cmovb rcx, rax
0x1404dee7f 4883f904 cmp rcx, 4
0x1404dee83 7414 je 0x1404dee99
0x1404dee85 4883f903 cmp rcx, 3
0x1404dee89 753b jne 0x1404deec6
0x1404dee8b 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404dee92 e8791e1100 call 0x1405f0d10 ; core::ptr::drop_in_place<egui::data::output::PlatformOutput>
0x1404dee97 eb2d jmp 0x1404deec6
0x1404dee99 488b9508060000 mov rdx, qword ptr [rbp + 0x608]
0x1404deea0 48b80200000000000080 movabs rax, 0x8000000000000002
0x1404deeaa 4839c2 cmp rdx, rax
0x1404deead 7c17 jl 0x1404deec6
0x1404deeaf 4885d2 test rdx, rdx
0x1404deeb2 7412 je 0x1404deec6
0x1404deeb4 488b8d10060000 mov rcx, qword ptr [rbp + 0x610]
0x1404deebb 41b801000000 mov r8d, 1
0x1404deec1 e85a4d1300 call 0x140613c20 ; __rust_dealloc
0x1404deec6 31f6 xor esi, esi
0x1404deec8 e9e6feffff jmp 0x1404dedb3
0x1404deecd 488d0de43b4502 lea rcx, [rip + 0x2453be4]
0x1404deed4 e877a33f02 call 0x1428d9250 ; core::cell::panic_already_borrowed
0x1404deed9 e97c070000 jmp 0x1404df65a
0x1404deede 488d0dd33b4502 lea rcx, [rip + 0x2453bd3]
0x1404deee5 e866a33f02 call 0x1428d9250 ; core::cell::panic_already_borrowed
0x1404deeea e96b070000 jmp 0x1404df65a
0x1404deeef 488d0dc23b4502 lea rcx, [rip + 0x2453bc2]
0x1404deef6 e855a33f02 call 0x1428d9250 ; core::cell::panic_already_borrowed
0x1404deefb e95a070000 jmp 0x1404df65a
0x1404def00 488d0db13b4502 lea rcx, [rip + 0x2453bb1]
0x1404def07 e844a33f02 call 0x1428d9250 ; core::cell::panic_already_borrowed
0x1404def0c e949070000 jmp 0x1404df65a
0x1404def11 488d0df0aa5002 lea rcx, [rip + 0x250aaf0]
0x1404def18 4c8d0519ab5002 lea r8, [rip + 0x250ab19]
0x1404def1f ba2c000000 mov edx, 0x2c
0x1404def24 e807a43f02 call 0x1428d9330 ; core::option::expect_failed
0x1404def29 e92c070000 jmp 0x1404df65a
0x1404def2e c685a314000001 mov byte ptr [rbp + 0x14a3], 1
0x1404def35 488d0d7c3b4502 lea rcx, [rip + 0x2453b7c]
0x1404def3c e80fa33f02 call 0x1428d9250 ; core::cell::panic_already_borrowed
0x1404def41 e914070000 jmp 0x1404df65a
0x1404def46 c685a314000000 mov byte ptr [rbp + 0x14a3], 0
0x1404def4d 488d0d643b4502 lea rcx, [rip + 0x2453b64]
0x1404def54 e8f7a23f02 call 0x1428d9250 ; core::cell::panic_already_borrowed
0x1404def59 e9fc060000 jmp 0x1404df65a
0x1404def5e 48898578140000 mov qword ptr [rbp + 0x1478], rax
0x1404def65 488d0dcc044f02 lea rcx, [rip + 0x24f04cc]
0x1404def6c 4c8d053d054f02 lea r8, [rip + 0x24f053d]
0x1404def73 ba28000000 mov edx, 0x28
0x1404def78 e8b3a43f02 call 0x1428d9430 ; core::panicking::panic
0x1404def7d e9d8060000 jmp 0x1404df65a
0x1404def82 4088b5a7140000 mov byte ptr [rbp + 0x14a7], sil
0x1404def89 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404def90 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404def97 488d0d9ab05002 lea rcx, [rip + 0x250b09a]
0x1404def9e e86da33f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404defa3 e9b2060000 jmp 0x1404df65a
0x1404defa8 488d0d59aa5002 lea rcx, [rip + 0x250aa59]
0x1404defaf 4c8d0582aa5002 lea r8, [rip + 0x250aa82]
0x1404defb6 ba2c000000 mov edx, 0x2c
0x1404defbb e870a33f02 call 0x1428d9330 ; core::option::expect_failed
0x1404defc0 e995060000 jmp 0x1404df65a
0x1404defc5 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404defcc c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404defd3 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404defda 488d0df7af5002 lea rcx, [rip + 0x250aff7]
0x1404defe1 e82aa33f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404defe6 e96f060000 jmp 0x1404df65a
0x1404defeb c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404deff2 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404deff9 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404df000 488d0df9ae5002 lea rcx, [rip + 0x250aef9]
0x1404df007 e804a33f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df00c e949060000 jmp 0x1404df65a
0x1404df011 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404df018 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404df01f c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404df026 488d0dbbae5002 lea rcx, [rip + 0x250aebb]
0x1404df02d e8dea23f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df032 e923060000 jmp 0x1404df65a
0x1404df037 c6857514000001 mov byte ptr [rbp + 0x1475], 1
0x1404df03e 488d0ddbaf5002 lea rcx, [rip + 0x250afdb]
0x1404df045 e8c6a23f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df04a e90b060000 jmp 0x1404df65a
0x1404df04f c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404df056 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404df05d c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404df064 488d0dadae5002 lea rcx, [rip + 0x250aead]
0x1404df06b e8a0a23f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df070 e9e5050000 jmp 0x1404df65a
0x1404df075 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404df07c c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404df083 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404df08a 488d0d77af5002 lea rcx, [rip + 0x250af77]
0x1404df091 e87aa23f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df096 e9bf050000 jmp 0x1404df65a
0x1404df09b c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404df0a2 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404df0a9 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404df0b0 488d0d39af5002 lea rcx, [rip + 0x250af39]
0x1404df0b7 e854a23f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df0bc e999050000 jmp 0x1404df65a
0x1404df0c1 c6858714000001 mov byte ptr [rbp + 0x1487], 1
0x1404df0c8 488d0d91ae5002 lea rcx, [rip + 0x250ae91]
0x1404df0cf e83ca23f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df0d4 e981050000 jmp 0x1404df65a
0x1404df0d9 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404df0e0 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404df0e7 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404df0ee 488d0d3bae5002 lea rcx, [rip + 0x250ae3b]
0x1404df0f5 e816a23f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df0fa e95b050000 jmp 0x1404df65a
0x1404df0ff c6857614000001 mov byte ptr [rbp + 0x1476], 1
0x1404df106 488d0d6bae5002 lea rcx, [rip + 0x250ae6b]
0x1404df10d e8fea13f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df112 e943050000 jmp 0x1404df65a
0x1404df117 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404df11e c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404df125 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404df12c 488d0d75ae5002 lea rcx, [rip + 0x250ae75]
0x1404df133 e8d8a13f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df138 e91d050000 jmp 0x1404df65a
0x1404df13d c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404df144 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404df14b c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404df152 488d0d37ae5002 lea rcx, [rip + 0x250ae37]
0x1404df159 e8b2a13f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df15e e9f7040000 jmp 0x1404df65a
0x1404df163 488d0dfeae5002 lea rcx, [rip + 0x250aefe]
0x1404df16a e8a1a13f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df16f e9e6040000 jmp 0x1404df65a
0x1404df174 488d0dd5ae5002 lea rcx, [rip + 0x250aed5]
0x1404df17b e890a13f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df180 e9d5040000 jmp 0x1404df65a
0x1404df185 c6859614000001 mov byte ptr [rbp + 0x1496], 1
0x1404df18c 889d95140000 mov byte ptr [rbp + 0x1495], bl
0x1404df192 488d0d27ae5002 lea rcx, [rip + 0x250ae27]
0x1404df199 e872a13f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df19e e9b7040000 jmp 0x1404df65a
0x1404df1a3 488d0d1eb15002 lea rcx, [rip + 0x250b11e]
0x1404df1aa e861a13f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df1af e9a6040000 jmp 0x1404df65a
0x1404df1b4 488d0d25b15002 lea rcx, [rip + 0x250b125]
0x1404df1bb e850a13f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df1c0 e995040000 jmp 0x1404df65a
0x1404df1c5 c6857414000001 mov byte ptr [rbp + 0x1474], 1
0x1404df1cc 488d0d3db15002 lea rcx, [rip + 0x250b13d]
0x1404df1d3 e838a13f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df1d8 e97d040000 jmp 0x1404df65a
0x1404df1dd 488d0d14b15002 lea rcx, [rip + 0x250b114]
0x1404df1e4 e827a13f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df1e9 e96c040000 jmp 0x1404df65a
0x1404df1ee c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404df1f5 488d0de4ab5002 lea rcx, [rip + 0x250abe4]
0x1404df1fc e80fa13f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df201 e954040000 jmp 0x1404df65a
0x1404df206 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404df20d 488d0db4ab5002 lea rcx, [rip + 0x250abb4]
0x1404df214 e8f7a03f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df219 e93c040000 jmp 0x1404df65a
0x1404df21e c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404df225 488d0de4ab5002 lea rcx, [rip + 0x250abe4]
0x1404df22c e8dfa03f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df231 e924040000 jmp 0x1404df65a
0x1404df236 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404df23d 488d0de4ab5002 lea rcx, [rip + 0x250abe4]
0x1404df244 e8c7a03f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df249 e90c040000 jmp 0x1404df65a
0x1404df24e c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404df255 488d0de4ab5002 lea rcx, [rip + 0x250abe4]
0x1404df25c e8afa03f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df261 e9f4030000 jmp 0x1404df65a
0x1404df266 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404df26d 488d0dfcab5002 lea rcx, [rip + 0x250abfc]
0x1404df274 e897a03f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df279 e9dc030000 jmp 0x1404df65a
0x1404df27e c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404df285 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404df28c c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404df293 488d0d06ac5002 lea rcx, [rip + 0x250ac06]
0x1404df29a e871a03f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df29f e9b6030000 jmp 0x1404df65a
0x1404df2a4 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404df2ab c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404df2b2 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404df2b9 488d0df8ab5002 lea rcx, [rip + 0x250abf8]
0x1404df2c0 e84ba03f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df2c5 e990030000 jmp 0x1404df65a
0x1404df2ca c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404df2d1 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404df2d8 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404df2df 488d0d72b05002 lea rcx, [rip + 0x250b072]
0x1404df2e6 e825a03f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df2eb e96a030000 jmp 0x1404df65a
0x1404df2f0 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404df2f7 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404df2fe c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404df305 488d0dc4ab5002 lea rcx, [rip + 0x250abc4]
0x1404df30c e8ff9f3f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df311 e944030000 jmp 0x1404df65a
0x1404df316 488d0d63ad5002 lea rcx, [rip + 0x250ad63]
0x1404df31d e8ee9f3f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df322 e933030000 jmp 0x1404df65a
0x1404df327 488d0d6aad5002 lea rcx, [rip + 0x250ad6a]
0x1404df32e e8dd9f3f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df333 e922030000 jmp 0x1404df65a
0x1404df338 488d0d29ae5002 lea rcx, [rip + 0x250ae29]
0x1404df33f e8cc9f3f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df344 e911030000 jmp 0x1404df65a
0x1404df349 488d0d30ae5002 lea rcx, [rip + 0x250ae30]
0x1404df350 e8bb9f3f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df355 e900030000 jmp 0x1404df65a
0x1404df35a 488d0d37ae5002 lea rcx, [rip + 0x250ae37]
0x1404df361 e8aa9f3f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df366 e9ef020000 jmp 0x1404df65a
0x1404df36b 488d0d26af5002 lea rcx, [rip + 0x250af26]
0x1404df372 e8999f3f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df377 e9de020000 jmp 0x1404df65a
0x1404df37c 488d0d45ad5002 lea rcx, [rip + 0x250ad45]
0x1404df383 e8889f3f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df388 e9cd020000 jmp 0x1404df65a
0x1404df38d 488d0d1cad5002 lea rcx, [rip + 0x250ad1c]
0x1404df394 e8779f3f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df399 e9bc020000 jmp 0x1404df65a
0x1404df39e 488d0d93ad5002 lea rcx, [rip + 0x250ad93]
0x1404df3a5 e8669f3f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df3aa e9ab020000 jmp 0x1404df65a
0x1404df3af 488d0d9aad5002 lea rcx, [rip + 0x250ad9a]
0x1404df3b6 e8559f3f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df3bb e99a020000 jmp 0x1404df65a
0x1404df3c0 488d0501a75002 lea rax, [rip + 0x250a701]
0x1404df3c7 488985e0130000 mov qword ptr [rbp + 0x13e0], rax
0x1404df3ce 48c785e813000036000000 mov qword ptr [rbp + 0x13e8], 0x36
0x1404df3d9 488d85e0130000 lea rax, [rbp + 0x13e0]
0x1404df3e0 48898530140000 mov qword ptr [rbp + 0x1430], rax
0x1404df3e7 488d05a2aac2ff lea rax, [rip - 0x3d555e]
0x1404df3ee 48898538140000 mov qword ptr [rbp + 0x1438], rax
0x1404df3f5 488d050ca85002 lea rax, [rip + 0x250a80c]
0x1404df3fc 488985300c0000 mov qword ptr [rbp + 0xc30], rax
0x1404df403 48c785380c000002000000 mov qword ptr [rbp + 0xc38], 2
0x1404df40e 48c785500c000000000000 mov qword ptr [rbp + 0xc50], 0
0x1404df419 488d8530140000 lea rax, [rbp + 0x1430]
0x1404df420 488985400c0000 mov qword ptr [rbp + 0xc40], rax
0x1404df427 48c785480c000001000000 mov qword ptr [rbp + 0xc48], 1
0x1404df432 c6859714000000 mov byte ptr [rbp + 0x1497], 0
0x1404df439 488d1518a95002 lea rdx, [rip + 0x250a918]
0x1404df440 488d8d300c0000 lea rcx, [rbp + 0xc30]
0x1404df447 e8449f3f02 call 0x1428d9390 ; core::panicking::panic_fmt
0x1404df44c e909020000 jmp 0x1404df65a
0x1404df451 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404df458 488d0d99a95002 lea rcx, [rip + 0x250a999]
0x1404df45f e8ac9e3f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df464 e9f1010000 jmp 0x1404df65a
0x1404df469 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404df470 488d0de1a95002 lea rcx, [rip + 0x250a9e1]
0x1404df477 e8949e3f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df47c e9d9010000 jmp 0x1404df65a
0x1404df481 c685a414000001 mov byte ptr [rbp + 0x14a4], 1
0x1404df488 488d0df9a95002 lea rcx, [rip + 0x250a9f9]
0x1404df48f e87c9e3f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df494 e9c1010000 jmp 0x1404df65a
0x1404df499 488d0d40ac5002 lea rcx, [rip + 0x250ac40]
0x1404df4a0 e86b9e3f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df4a5 e9b0010000 jmp 0x1404df65a
0x1404df4aa 488d0d47ac5002 lea rcx, [rip + 0x250ac47]
0x1404df4b1 e85a9e3f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df4b6 e99f010000 jmp 0x1404df65a
0x1404df4bb 488d0dcf561503 lea rcx, [rip + 0x31556cf]
0x1404df4c2 48898d300c0000 mov qword ptr [rbp + 0xc30], rcx
0x1404df4c9 8885380c0000 mov byte ptr [rbp + 0xc38], al
0x1404df4cf 488d0572aa5002 lea rax, [rip + 0x250aa72]
0x1404df4d6 4889442420 mov qword ptr [rsp + 0x20], rax
0x1404df4db 488d0d9ea25002 lea rcx, [rip + 0x250a29e]
0x1404df4e2 4c8d0dc7a25002 lea r9, [rip + 0x250a2c7]
0x1404df4e9 4c8d85300c0000 lea r8, [rbp + 0xc30]
0x1404df4f0 ba2b000000 mov edx, 0x2b
0x1404df4f5 e866a23f02 call 0x1428d9760 ; core::result::unwrap_failed
0x1404df4fa e95b010000 jmp 0x1404df65a
0x1404df4ff 488d0d52ae5002 lea rcx, [rip + 0x250ae52]
0x1404df506 e8059e3f02 call 0x1428d9310 ; core::option::unwrap_failed
0x1404df50b e94a010000 jmp 0x1404df65a
0x1404df510 488d0d7a561503 lea rcx, [rip + 0x315567a]
0x1404df517 48898d300c0000 mov qword ptr [rbp + 0xc30], rcx
0x1404df51e 8885380c0000 mov byte ptr [rbp + 0xc38], al
0x1404df524 488d0585ad5002 lea rax, [rip + 0x250ad85]
0x1404df52b 4889442420 mov qword ptr [rsp + 0x20], rax
0x1404df530 488d0d49a25002 lea rcx, [rip + 0x250a249]
0x1404df537 4c8d0d72a25002 lea r9, [rip + 0x250a272]
0x1404df53e 4c8d85300c0000 lea r8, [rbp + 0xc30]
0x1404df545 ba2b000000 mov edx, 0x2b
0x1404df54a e811a23f02 call 0x1428d9760 ; core::result::unwrap_failed
0x1404df54f e906010000 jmp 0x1404df65a
0x1404df554 4c89bd78140000 mov qword ptr [rbp + 0x1478], r15
0x1404df55b b904000000 mov ecx, 4
0x1404df560 badc000000 mov edx, 0xdc
0x1404df565 e8969a3f02 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
0x1404df56a e9eb000000 jmp 0x1404df65a
0x1404df56f 488d0d9a344502 lea rcx, [rip + 0x245349a]
0x1404df576 e8558b3f02 call 0x1428d80d0 ; std::thread::local::panic_access_error
0x1404df57b e9da000000 jmp 0x1404df65a
0x1404df580 488d0d89344502 lea rcx, [rip + 0x2453489]
0x1404df587 e8448b3f02 call 0x1428d80d0 ; std::thread::local::panic_access_error
0x1404df58c e9c9000000 jmp 0x1404df65a
0x1404df591 488d0d78344502 lea rcx, [rip + 0x2453478]
0x1404df598 e8338b3f02 call 0x1428d80d0 ; std::thread::local::panic_access_error
0x1404df59d e9b8000000 jmp 0x1404df65a
0x1404df5a2 488d0d67344502 lea rcx, [rip + 0x2453467]
0x1404df5a9 e8228b3f02 call 0x1428d80d0 ; std::thread::local::panic_access_error
0x1404df5ae e9a7000000 jmp 0x1404df65a
0x1404df5b3 488d0536a45002 lea rax, [rip + 0x250a436]
0x1404df5ba 4889442420 mov qword ptr [rsp + 0x20], rax
0x1404df5bf 488d0dfaa65002 lea rcx, [rip + 0x250a6fa]
0x1404df5c6 ba22000000 mov edx, 0x22
0x1404df5cb 41b922000000 mov r9d, 0x22
0x1404df5d1 4989f0 mov r8, rsi
0x1404df5d4 e867a33f02 call 0x1428d9940 ; core::str::slice_error_fail
0x1404df5d9 eb7f jmp 0x1404df65a
0x1404df5db 488d050ea45002 lea rax, [rip + 0x250a40e]
0x1404df5e2 4889442420 mov qword ptr [rsp + 0x20], rax
0x1404df5e7 488d0dd2a65002 lea rcx, [rip + 0x250a6d2]
0x1404df5ee ba22000000 mov edx, 0x22
0x1404df5f3 41b922000000 mov r9d, 0x22
0x1404df5f9 4989f8 mov r8, rdi
0x1404df5fc e83fa33f02 call 0x1428d9940 ; core::str::slice_error_fail
0x1404df601 eb57 jmp 0x1404df65a
0x1404df603 c685a314000001 mov byte ptr [rbp + 0x14a3], 1
0x1404df60a 488d0dff334502 lea rcx, [rip + 0x24533ff]
0x1404df611 e8ba8a3f02 call 0x1428d80d0 ; std::thread::local::panic_access_error
0x1404df616 eb42 jmp 0x1404df65a
0x1404df618 c685a314000000 mov byte ptr [rbp + 0x14a3], 0
0x1404df61f 488d0dea334502 lea rcx, [rip + 0x24533ea]
0x1404df626 e8a58a3f02 call 0x1428d80d0 ; std::thread::local::panic_access_error
0x1404df62b eb2d jmp 0x1404df65a
0x1404df62d c685a314000001 mov byte ptr [rbp + 0x14a3], 1
0x1404df634 488d05b5a35002 lea rax, [rip + 0x250a3b5]
0x1404df63b 4889442420 mov qword ptr [rsp + 0x20], rax
0x1404df640 488d0d79a65002 lea rcx, [rip + 0x250a679]
0x1404df647 ba22000000 mov edx, 0x22
0x1404df64c 41b922000000 mov r9d, 0x22
0x1404df652 4d89e8 mov r8, r13
0x1404df655 e8e6a23f02 call 0x1428d9940 ; core::str::slice_error_fail
0x1404df65a 0f0b ud2 
0x1404df65c 0f1f4000 nop dword ptr [rax]
0x1404df660 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404df665 55 push rbp
0x1404df666 4157 push r15
0x1404df668 4156 push r14
0x1404df66a 4155 push r13
0x1404df66c 4154 push r12
0x1404df66e 56 push rsi
0x1404df66f 57 push rdi
0x1404df670 53 push rbx
0x1404df671 4881ece8000000 sub rsp, 0xe8
0x1404df678 488daa80000000 lea rbp, [rdx + 0x80]
0x1404df67f 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404df686 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404df68c 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404df693 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404df699 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404df6a2 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404df6ac 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404df6b6 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404df6bf 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404df6c8 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404df6d1 80bd7314000000 cmp byte ptr [rbp + 0x1473], 0
0x1404df6d8 0f8490000000 je 0x1404df76e
0x1404df6de 4883bd6814000000 cmp qword ptr [rbp + 0x1468], 0
0x1404df6e6 7419 je 0x1404df701
0x1404df6e8 41b801000000 mov r8d, 1
0x1404df6ee 488b8d60140000 mov rcx, qword ptr [rbp + 0x1460]
0x1404df6f5 488b9568140000 mov rdx, qword ptr [rbp + 0x1468]
0x1404df6fc e81f451300 call 0x140613c20 ; __rust_dealloc
0x1404df701 c685a214000001 mov byte ptr [rbp + 0x14a2], 1
0x1404df708 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404df711 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404df71a 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404df723 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404df72d 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404df737 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404df740 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404df746 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404df74d 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404df753 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404df75a 4881c4e8000000 add rsp, 0xe8
0x1404df761 5b pop rbx
0x1404df762 5f pop rdi
0x1404df763 5e pop rsi
0x1404df764 415c pop r12
0x1404df766 415d pop r13
0x1404df768 415e pop r14
0x1404df76a 415f pop r15
0x1404df76c 5d pop rbp
0x1404df76d c3 ret 
0x1404df76e c685a214000001 mov byte ptr [rbp + 0x14a2], 1
0x1404df775 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404df77e 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404df787 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404df790 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404df79a 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404df7a4 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404df7ad 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404df7b3 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404df7ba 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404df7c0 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404df7c7 4881c4e8000000 add rsp, 0xe8
0x1404df7ce 5b pop rbx
0x1404df7cf 5f pop rdi
0x1404df7d0 5e pop rsi
0x1404df7d1 415c pop r12
0x1404df7d3 415d pop r13
0x1404df7d5 415e pop r14
0x1404df7d7 415f pop r15
0x1404df7d9 5d pop rbp
0x1404df7da c3 ret 
0x1404df7db 0f1f440000 nop dword ptr [rax + rax]
0x1404df7e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404df7e5 55 push rbp
0x1404df7e6 4157 push r15
0x1404df7e8 4156 push r14
0x1404df7ea 4155 push r13
0x1404df7ec 4154 push r12
0x1404df7ee 56 push rsi
0x1404df7ef 57 push rdi
0x1404df7f0 53 push rbx
0x1404df7f1 4881ece8000000 sub rsp, 0xe8
0x1404df7f8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404df7ff 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404df806 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404df80c 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404df813 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404df819 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404df822 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404df82c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404df836 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404df83f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404df848 660f7fb424d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404df851 488b8598140000 mov rax, qword ptr [rbp + 0x1498]
0x1404df858 48ff00 inc qword ptr [rax]
0x1404df85b c685a314000000 mov byte ptr [rbp + 0x14a3], 0
0x1404df862 660f6fb424d0000000 movdqa xmm6, xmmword ptr [rsp + 0xd0]
0x1404df86b 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404df874 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404df87d 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404df887 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404df891 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404df89a 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404df8a0 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404df8a7 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404df8ad 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404df8b4 4881c4e8000000 add rsp, 0xe8
0x1404df8bb 5b pop rbx
0x1404df8bc 5f pop rdi
0x1404df8bd 5e pop rsi
0x1404df8be 415c pop r12
0x1404df8c0 415d pop r13
0x1404df8c2 415e pop r14
0x1404df8c4 415f pop r15
0x1404df8c6 5d pop rbp
0x1404df8c7 c3 ret 
0x1404df8c8 0f1f840000000000 nop dword ptr [rax + rax]
0x1404df8d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404df8d5 55 push rbp
0x1404df8d6 4157 push r15
0x1404df8d8 4156 push r14
0x1404df8da 4155 push r13
0x1404df8dc 4154 push r12
0x1404df8de 56 push rsi
0x1404df8df 57 push rdi
0x1404df8e0 53 push rbx
0x1404df8e1 4881ece8000000 sub rsp, 0xe8
0x1404df8e8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404df8ef 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404df8f6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404df8fc 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404df903 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404df909 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404df912 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404df91c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404df926 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404df92f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404df938 660f7fb424d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404df941 488b8548140000 mov rax, qword ptr [rbp + 0x1448]
0x1404df948 48ff00 inc qword ptr [rax]
0x1404df94b c685a314000001 mov byte ptr [rbp + 0x14a3], 1
0x1404df952 660f6fb424d0000000 movdqa xmm6, xmmword ptr [rsp + 0xd0]
0x1404df95b 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404df964 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404df96d 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404df977 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404df981 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404df98a 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404df990 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404df997 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404df99d 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404df9a4 4881c4e8000000 add rsp, 0xe8
0x1404df9ab 5b pop rbx
0x1404df9ac 5f pop rdi
0x1404df9ad 5e pop rsi
0x1404df9ae 415c pop r12
0x1404df9b0 415d pop r13
0x1404df9b2 415e pop r14
0x1404df9b4 415f pop r15
0x1404df9b6 5d pop rbp
0x1404df9b7 c3 ret 
0x1404df9b8 0f1f840000000000 nop dword ptr [rax + rax]
0x1404df9c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404df9c5 55 push rbp
0x1404df9c6 4157 push r15
0x1404df9c8 4156 push r14
0x1404df9ca 4155 push r13
0x1404df9cc 4154 push r12
0x1404df9ce 56 push rsi
0x1404df9cf 57 push rdi
0x1404df9d0 53 push rbx
0x1404df9d1 4881ece8000000 sub rsp, 0xe8
0x1404df9d8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404df9df 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404df9e6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404df9ec 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404df9f3 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404df9f9 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404dfa02 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404dfa0c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404dfa16 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404dfa1f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404dfa28 660f7fb424d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404dfa31 0fb69d72140000 movzx ebx, byte ptr [rbp + 0x1472]
0x1404dfa38 80bd9814000000 cmp byte ptr [rbp + 0x1498], 0
0x1404dfa3f 742c je 0x1404dfa6d
0x1404dfa41 488d45c8 lea rax, [rbp - 0x38]
0x1404dfa45 48898558140000 mov qword ptr [rbp + 0x1458], rax
0x1404dfa4c 488d8558140000 lea rax, [rbp + 0x1458]
0x1404dfa53 48898530140000 mov qword ptr [rbp + 0x1430], rax
0x1404dfa5a 488d0d2f9f5002 lea rcx, [rip + 0x2509f2f]
0x1404dfa61 488d9530140000 lea rdx, [rbp + 0x1430]
0x1404dfa68 e8132bbcff call 0x1400a2580 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
0x1404dfa6d 80e301 and bl, 1
0x1404dfa70 889da3140000 mov byte ptr [rbp + 0x14a3], bl
0x1404dfa76 660f6fb424d0000000 movdqa xmm6, xmmword ptr [rsp + 0xd0]
0x1404dfa7f 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404dfa88 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404dfa91 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404dfa9b 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404dfaa5 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404dfaae 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404dfab4 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404dfabb 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404dfac1 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404dfac8 4881c4e8000000 add rsp, 0xe8
0x1404dfacf 5b pop rbx
0x1404dfad0 5f pop rdi
0x1404dfad1 5e pop rsi
0x1404dfad2 415c pop r12
0x1404dfad4 415d pop r13
0x1404dfad6 415e pop r14
0x1404dfad8 415f pop r15
0x1404dfada 5d pop rbp
0x1404dfadb c3 ret 
0x1404dfadc 0f1f4000 nop dword ptr [rax]
0x1404dfae0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404dfae5 55 push rbp
0x1404dfae6 4157 push r15
0x1404dfae8 4156 push r14
0x1404dfaea 4155 push r13
0x1404dfaec 4154 push r12
0x1404dfaee 56 push rsi
0x1404dfaef 57 push rdi
0x1404dfaf0 53 push rbx
0x1404dfaf1 4881ece8000000 sub rsp, 0xe8
0x1404dfaf8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404dfaff 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404dfb06 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404dfb0c 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404dfb13 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404dfb19 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404dfb22 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404dfb2c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404dfb36 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404dfb3f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404dfb48 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404dfb51 80bd9814000000 cmp byte ptr [rbp + 0x1498], 0
0x1404dfb58 0f848e000000 je 0x1404dfbec
0x1404dfb5e 0fb685900c0000 movzx eax, byte ptr [rbp + 0xc90]
0x1404dfb65 3c1b cmp al, 0x1b
0x1404dfb67 741d je 0x1404dfb86
0x1404dfb69 3c18 cmp al, 0x18
0x1404dfb6b 7519 jne 0x1404dfb86
0x1404dfb6d 488b85980c0000 mov rax, qword ptr [rbp + 0xc98]
0x1404dfb74 f048ff08 lock dec qword ptr [rax]
0x1404dfb78 750c jne 0x1404dfb86
0x1404dfb7a 488d8d980c0000 lea rcx, [rbp + 0xc98]
0x1404dfb81 e8aaef1600 call 0x14064eb30 ; alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global>
0x1404dfb86 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404dfb8f 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404dfb98 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404dfba1 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404dfbab 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404dfbb5 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404dfbbe 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404dfbc4 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404dfbcb 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404dfbd1 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404dfbd8 4881c4e8000000 add rsp, 0xe8
0x1404dfbdf 5b pop rbx
0x1404dfbe0 5f pop rdi
0x1404dfbe1 5e pop rsi
0x1404dfbe2 415c pop r12
0x1404dfbe4 415d pop r13
0x1404dfbe6 415e pop r14
0x1404dfbe8 415f pop r15
0x1404dfbea 5d pop rbp
0x1404dfbeb c3 ret 
0x1404dfbec 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404dfbf5 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404dfbfe 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404dfc07 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404dfc11 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404dfc1b 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404dfc24 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404dfc2a 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404dfc31 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404dfc37 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404dfc3e 4881c4e8000000 add rsp, 0xe8
0x1404dfc45 5b pop rbx
0x1404dfc46 5f pop rdi
0x1404dfc47 5e pop rsi
0x1404dfc48 415c pop r12
0x1404dfc4a 415d pop r13
0x1404dfc4c 415e pop r14
0x1404dfc4e 415f pop r15
0x1404dfc50 5d pop rbp
0x1404dfc51 c3 ret 
0x1404dfc52 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404dfc60 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404dfc65 55 push rbp
0x1404dfc66 4157 push r15
0x1404dfc68 4156 push r14
0x1404dfc6a 4155 push r13
0x1404dfc6c 4154 push r12
0x1404dfc6e 56 push rsi
0x1404dfc6f 57 push rdi
0x1404dfc70 53 push rbx
0x1404dfc71 4881ece8000000 sub rsp, 0xe8
0x1404dfc78 488daa80000000 lea rbp, [rdx + 0x80]
0x1404dfc7f 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404dfc86 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404dfc8c 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404dfc93 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404dfc99 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404dfca2 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404dfcac 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404dfcb6 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404dfcbf 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404dfcc8 660f7fb424d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404dfcd1 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404dfcd8 41b868000000 mov r8d, 0x68
0x1404dfcde 41b910000000 mov r9d, 0x10
0x1404dfce4 488d9520060000 lea rdx, [rbp + 0x620]
0x1404dfceb e8c0d1b6ff call 0x14004ceb0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>,alloc::alloc::Global>
0x1404dfcf0 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404dfcf8 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404dfd00 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404dfd09 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404dfd12 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404dfd1b 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404dfd24 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404dfd2a 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404dfd30 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404dfd36 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404dfd3c 4881c4e8000000 add rsp, 0xe8
0x1404dfd43 5b pop rbx
0x1404dfd44 5f pop rdi
0x1404dfd45 5e pop rsi
0x1404dfd46 415c pop r12
0x1404dfd48 415d pop r13
0x1404dfd4a 415e pop r14
0x1404dfd4c 415f pop r15
0x1404dfd4e 5d pop rbp
0x1404dfd4f c3 ret 
0x1404dfd50 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404dfd55 55 push rbp
0x1404dfd56 4157 push r15
0x1404dfd58 4156 push r14
0x1404dfd5a 4155 push r13
0x1404dfd5c 4154 push r12
0x1404dfd5e 56 push rsi
0x1404dfd5f 57 push rdi
0x1404dfd60 53 push rbx
0x1404dfd61 4881ece8000000 sub rsp, 0xe8
0x1404dfd68 488daa80000000 lea rbp, [rdx + 0x80]
0x1404dfd6f 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404dfd76 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404dfd7c 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404dfd83 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404dfd89 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404dfd92 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404dfd9c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404dfda6 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404dfdaf 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404dfdb8 660f7fb424d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404dfdc1 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404dfdc8 41b868000000 mov r8d, 0x68
0x1404dfdce 41b910000000 mov r9d, 0x10
0x1404dfdd4 488d9520060000 lea rdx, [rbp + 0x620]
0x1404dfddb e8d0d0b6ff call 0x14004ceb0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>,alloc::alloc::Global>
0x1404dfde0 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404dfde8 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404dfdf0 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404dfdf9 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404dfe02 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404dfe0b 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404dfe14 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404dfe1a 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404dfe20 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404dfe26 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404dfe2c 4881c4e8000000 add rsp, 0xe8
0x1404dfe33 5b pop rbx
0x1404dfe34 5f pop rdi
0x1404dfe35 5e pop rsi
0x1404dfe36 415c pop r12
0x1404dfe38 415d pop r13
0x1404dfe3a 415e pop r14
0x1404dfe3c 415f pop r15
0x1404dfe3e 5d pop rbp
0x1404dfe3f c3 ret 
0x1404dfe40 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404dfe45 55 push rbp
0x1404dfe46 4157 push r15
0x1404dfe48 4156 push r14
0x1404dfe4a 4155 push r13
0x1404dfe4c 4154 push r12
0x1404dfe4e 56 push rsi
0x1404dfe4f 57 push rdi
0x1404dfe50 53 push rbx
0x1404dfe51 4881ece8000000 sub rsp, 0xe8
0x1404dfe58 488daa80000000 lea rbp, [rdx + 0x80]
0x1404dfe5f 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404dfe66 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404dfe6c 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404dfe73 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404dfe79 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404dfe82 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404dfe8c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404dfe96 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404dfe9f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404dfea8 660f7fb424d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404dfeb1 41b810000000 mov r8d, 0x10
0x1404dfeb7 41b910000000 mov r9d, 0x10
0x1404dfebd 488b8da0130000 mov rcx, qword ptr [rbp + 0x13a0]
0x1404dfec4 488b9578130000 mov rdx, qword ptr [rbp + 0x1378]
0x1404dfecb e830c5b6ff call 0x14004c400 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<core::ptr::non_null::NonNull<hashlink::linked_hash_map::Node<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> > >,tuple$<> >,allocator_api2::stable::alloc::global::Global>
0x1404dfed0 0f288540060000 movaps xmm0, xmmword ptr [rbp + 0x640]
0x1404dfed7 488b85d0130000 mov rax, qword ptr [rbp + 0x13d0]
0x1404dfede 0f114040 movups xmmword ptr [rax + 0x40], xmm0
0x1404dfee2 0f288500060000 movaps xmm0, xmmword ptr [rbp + 0x600]
0x1404dfee9 0f288d10060000 movaps xmm1, xmmword ptr [rbp + 0x610]
0x1404dfef0 0f289520060000 movaps xmm2, xmmword ptr [rbp + 0x620]
0x1404dfef7 0f289d30060000 movaps xmm3, xmmword ptr [rbp + 0x630]
0x1404dfefe 0f115830 movups xmmword ptr [rax + 0x30], xmm3
0x1404dff02 0f115020 movups xmmword ptr [rax + 0x20], xmm2
0x1404dff06 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x1404dff0a 0f1100 movups xmmword ptr [rax], xmm0
0x1404dff0d 488b8578140000 mov rax, qword ptr [rbp + 0x1478]
0x1404dff14 488b8d68140000 mov rcx, qword ptr [rbp + 0x1468]
0x1404dff1b 488988d0090000 mov qword ptr [rax + 0x9d0], rcx
0x1404dff22 488b8d98140000 mov rcx, qword ptr [rbp + 0x1498]
0x1404dff29 488988d8090000 mov qword ptr [rax + 0x9d8], rcx
0x1404dff30 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404dff38 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404dff40 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404dff49 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404dff52 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404dff5b 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404dff64 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404dff6a 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404dff70 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404dff76 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404dff7c 4881c4e8000000 add rsp, 0xe8
0x1404dff83 5b pop rbx
0x1404dff84 5f pop rdi
0x1404dff85 5e pop rsi
0x1404dff86 415c pop r12
0x1404dff88 415d pop r13
0x1404dff8a 415e pop r14
0x1404dff8c 415f pop r15
0x1404dff8e 5d pop rbp
0x1404dff8f c3 ret 
0x1404dff90 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404dff95 55 push rbp
0x1404dff96 4157 push r15
0x1404dff98 4156 push r14
0x1404dff9a 4155 push r13
0x1404dff9c 4154 push r12
0x1404dff9e 56 push rsi
0x1404dff9f 57 push rdi
0x1404dffa0 53 push rbx
0x1404dffa1 4881ece8000000 sub rsp, 0xe8
0x1404dffa8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404dffaf 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404dffb6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404dffbc 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404dffc3 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404dffc9 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404dffd2 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404dffdc 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404dffe6 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404dffef 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404dfff8 660f7fb424d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404e0001 41b810000000 mov r8d, 0x10
0x1404e0007 41b910000000 mov r9d, 0x10
0x1404e000d 488b8d98130000 mov rcx, qword ptr [rbp + 0x1398]
0x1404e0014 488b9570130000 mov rdx, qword ptr [rbp + 0x1370]
0x1404e001b e8e0c3b6ff call 0x14004c400 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<core::ptr::non_null::NonNull<hashlink::linked_hash_map::Node<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> > >,tuple$<> >,allocator_api2::stable::alloc::global::Global>
0x1404e0020 0f288540060000 movaps xmm0, xmmword ptr [rbp + 0x640]
0x1404e0027 488b85c8130000 mov rax, qword ptr [rbp + 0x13c8]
0x1404e002e 0f114040 movups xmmword ptr [rax + 0x40], xmm0
0x1404e0032 0f288500060000 movaps xmm0, xmmword ptr [rbp + 0x600]
0x1404e0039 0f288d10060000 movaps xmm1, xmmword ptr [rbp + 0x610]
0x1404e0040 0f289520060000 movaps xmm2, xmmword ptr [rbp + 0x620]
0x1404e0047 0f289d30060000 movaps xmm3, xmmword ptr [rbp + 0x630]
0x1404e004e 0f115830 movups xmmword ptr [rax + 0x30], xmm3
0x1404e0052 0f115020 movups xmmword ptr [rax + 0x20], xmm2
0x1404e0056 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x1404e005a 0f1100 movups xmmword ptr [rax], xmm0
0x1404e005d 488b8578140000 mov rax, qword ptr [rbp + 0x1478]
0x1404e0064 488b8d68140000 mov rcx, qword ptr [rbp + 0x1468]
0x1404e006b 488988e0080000 mov qword ptr [rax + 0x8e0], rcx
0x1404e0072 488b8d98140000 mov rcx, qword ptr [rbp + 0x1498]
0x1404e0079 488988e8080000 mov qword ptr [rax + 0x8e8], rcx
0x1404e0080 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0088 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0090 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0099 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e00a2 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e00ab 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e00b4 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e00ba 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e00c0 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e00c6 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e00cc 4881c4e8000000 add rsp, 0xe8
0x1404e00d3 5b pop rbx
0x1404e00d4 5f pop rdi
0x1404e00d5 5e pop rsi
0x1404e00d6 415c pop r12
0x1404e00d8 415d pop r13
0x1404e00da 415e pop r14
0x1404e00dc 415f pop r15
0x1404e00de 5d pop rbp
0x1404e00df c3 ret 
0x1404e00e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e00e5 55 push rbp
0x1404e00e6 4157 push r15
0x1404e00e8 4156 push r14
0x1404e00ea 4155 push r13
0x1404e00ec 4154 push r12
0x1404e00ee 56 push rsi
0x1404e00ef 57 push rdi
0x1404e00f0 53 push rbx
0x1404e00f1 4881ece8000000 sub rsp, 0xe8
0x1404e00f8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e00ff 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e0106 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e010c 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e0113 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e0119 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e0122 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e012c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0136 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e013f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0148 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e0151 488d95300c0000 lea rdx, [rbp + 0xc30]
0x1404e0158 41b828060000 mov r8d, 0x628
0x1404e015e 488b8d10140000 mov rcx, qword ptr [rbp + 0x1410]
0x1404e0165 e8963a4402 call 0x142923c00 ; memcpy
0x1404e016a c6859714000000 mov byte ptr [rbp + 0x1497], 0
0x1404e0171 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e017a 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0183 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e018c 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0196 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e01a0 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e01a9 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e01af 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e01b6 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e01bc 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e01c3 4881c4e8000000 add rsp, 0xe8
0x1404e01ca 5b pop rbx
0x1404e01cb 5f pop rdi
0x1404e01cc 5e pop rsi
0x1404e01cd 415c pop r12
0x1404e01cf 415d pop r13
0x1404e01d1 415e pop r14
0x1404e01d3 415f pop r15
0x1404e01d5 5d pop rbp
0x1404e01d6 c3 ret 
0x1404e01d7 660f1f840000000000 nop word ptr [rax + rax]
0x1404e01e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e01e5 55 push rbp
0x1404e01e6 4157 push r15
0x1404e01e8 4156 push r14
0x1404e01ea 4155 push r13
0x1404e01ec 4154 push r12
0x1404e01ee 56 push rsi
0x1404e01ef 57 push rdi
0x1404e01f0 53 push rbx
0x1404e01f1 4881ece8000000 sub rsp, 0xe8
0x1404e01f8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e01ff 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e0206 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e020c 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e0213 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e0219 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e0222 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e022c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0236 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e023f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0248 660f7fb424d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404e0251 488b8598140000 mov rax, qword ptr [rbp + 0x1498]
0x1404e0258 48ff00 inc qword ptr [rax]
0x1404e025b 660f6fb424d0000000 movdqa xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0264 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e026d 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0276 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0280 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e028a 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0293 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0299 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e02a0 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e02a6 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e02ad 4881c4e8000000 add rsp, 0xe8
0x1404e02b4 5b pop rbx
0x1404e02b5 5f pop rdi
0x1404e02b6 5e pop rsi
0x1404e02b7 415c pop r12
0x1404e02b9 415d pop r13
0x1404e02bb 415e pop r14
0x1404e02bd 415f pop r15
0x1404e02bf 5d pop rbp
0x1404e02c0 c3 ret 
0x1404e02c1 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404e02d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e02d5 55 push rbp
0x1404e02d6 4157 push r15
0x1404e02d8 4156 push r14
0x1404e02da 4155 push r13
0x1404e02dc 4154 push r12
0x1404e02de 56 push rsi
0x1404e02df 57 push rdi
0x1404e02e0 53 push rbx
0x1404e02e1 4881ece8000000 sub rsp, 0xe8
0x1404e02e8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e02ef 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e02f6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e02fc 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e0303 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e0309 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e0312 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e031c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0326 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e032f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0338 660f7fb424d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404e0341 488b8548140000 mov rax, qword ptr [rbp + 0x1448]
0x1404e0348 48ff00 inc qword ptr [rax]
0x1404e034b 660f6fb424d0000000 movdqa xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0354 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e035d 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0366 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0370 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e037a 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0383 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0389 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e0390 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e0396 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e039d 4881c4e8000000 add rsp, 0xe8
0x1404e03a4 5b pop rbx
0x1404e03a5 5f pop rdi
0x1404e03a6 5e pop rsi
0x1404e03a7 415c pop r12
0x1404e03a9 415d pop r13
0x1404e03ab 415e pop r14
0x1404e03ad 415f pop r15
0x1404e03af 5d pop rbp
0x1404e03b0 c3 ret 
0x1404e03b1 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404e03c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e03c5 55 push rbp
0x1404e03c6 4157 push r15
0x1404e03c8 4156 push r14
0x1404e03ca 4155 push r13
0x1404e03cc 4154 push r12
0x1404e03ce 56 push rsi
0x1404e03cf 57 push rdi
0x1404e03d0 53 push rbx
0x1404e03d1 4881ece8000000 sub rsp, 0xe8
0x1404e03d8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e03df 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e03e6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e03ec 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e03f3 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e03f9 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e0402 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e040c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0416 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e041f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0428 660f7fb424d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404e0431 488b8598140000 mov rax, qword ptr [rbp + 0x1498]
0x1404e0438 48ff00 inc qword ptr [rax]
0x1404e043b 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0443 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e044b 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0454 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e045d 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e0466 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e046f 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0475 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e047b 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e0481 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e0487 4881c4e8000000 add rsp, 0xe8
0x1404e048e 5b pop rbx
0x1404e048f 5f pop rdi
0x1404e0490 5e pop rsi
0x1404e0491 415c pop r12
0x1404e0493 415d pop r13
0x1404e0495 415e pop r14
0x1404e0497 415f pop r15
0x1404e0499 5d pop rbp
0x1404e049a c3 ret 
0x1404e049b 0f1f440000 nop dword ptr [rax + rax]
0x1404e04a0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e04a5 55 push rbp
0x1404e04a6 4157 push r15
0x1404e04a8 4156 push r14
0x1404e04aa 4155 push r13
0x1404e04ac 4154 push r12
0x1404e04ae 56 push rsi
0x1404e04af 57 push rdi
0x1404e04b0 53 push rbx
0x1404e04b1 4881ece8000000 sub rsp, 0xe8
0x1404e04b8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e04bf 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e04c6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e04cc 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e04d3 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e04d9 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e04e2 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e04ec 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e04f6 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e04ff 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0508 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e0511 488d8d500c0000 lea rcx, [rbp + 0xc50]
0x1404e0518 e8f39b1100 call 0x1405fa110 ; core::ptr::drop_in_place<winit::platform_impl::platform::KeyEventExtra>
0x1404e051d c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404e0524 c685a614000000 mov byte ptr [rbp + 0x14a6], 0
0x1404e052b c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404e0532 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e053b 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0544 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e054d 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0557 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e0561 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e056a 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0570 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e0577 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e057d 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e0584 4881c4e8000000 add rsp, 0xe8
0x1404e058b 5b pop rbx
0x1404e058c 5f pop rdi
0x1404e058d 5e pop rsi
0x1404e058e 415c pop r12
0x1404e0590 415d pop r13
0x1404e0592 415e pop r14
0x1404e0594 415f pop r15
0x1404e0596 5d pop rbp
0x1404e0597 c3 ret 
0x1404e0598 0f1f840000000000 nop dword ptr [rax + rax]
0x1404e05a0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e05a5 55 push rbp
0x1404e05a6 4157 push r15
0x1404e05a8 4156 push r14
0x1404e05aa 4155 push r13
0x1404e05ac 4154 push r12
0x1404e05ae 56 push rsi
0x1404e05af 57 push rdi
0x1404e05b0 53 push rbx
0x1404e05b1 4881ece8000000 sub rsp, 0xe8
0x1404e05b8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e05bf 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e05c6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e05cc 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e05d3 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e05d9 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e05e2 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e05ec 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e05f6 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e05ff 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0608 0f29b424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm6
0x1404e0610 488b8598140000 mov rax, qword ptr [rbp + 0x1498]
0x1404e0617 48ff00 inc qword ptr [rax]
0x1404e061a 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0622 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e062a 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0633 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e063c 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e0645 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e064e 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0654 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e065a 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e0660 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e0666 4881c4e8000000 add rsp, 0xe8
0x1404e066d 5b pop rbx
0x1404e066e 5f pop rdi
0x1404e066f 5e pop rsi
0x1404e0670 415c pop r12
0x1404e0672 415d pop r13
0x1404e0674 415e pop r14
0x1404e0676 415f pop r15
0x1404e0678 5d pop rbp
0x1404e0679 c3 ret 
0x1404e067a 660f1f440000 nop word ptr [rax + rax]
0x1404e0680 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e0685 55 push rbp
0x1404e0686 4157 push r15
0x1404e0688 4156 push r14
0x1404e068a 4155 push r13
0x1404e068c 4154 push r12
0x1404e068e 56 push rsi
0x1404e068f 57 push rdi
0x1404e0690 53 push rbx
0x1404e0691 4881ece8000000 sub rsp, 0xe8
0x1404e0698 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e069f 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e06a6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e06ac 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e06b3 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e06b9 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e06c2 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e06cc 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e06d6 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e06df 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e06e8 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e06f1 488d95300c0000 lea rdx, [rbp + 0xc30]
0x1404e06f8 41b8c8000000 mov r8d, 0xc8
0x1404e06fe 488b8d98140000 mov rcx, qword ptr [rbp + 0x1498]
0x1404e0705 e8f6344402 call 0x142923c00 ; memcpy
0x1404e070a c6857414000000 mov byte ptr [rbp + 0x1474], 0
0x1404e0711 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e071a 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0723 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e072c 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0736 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e0740 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0749 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e074f 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e0756 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e075c 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e0763 4881c4e8000000 add rsp, 0xe8
0x1404e076a 5b pop rbx
0x1404e076b 5f pop rdi
0x1404e076c 5e pop rsi
0x1404e076d 415c pop r12
0x1404e076f 415d pop r13
0x1404e0771 415e pop r14
0x1404e0773 415f pop r15
0x1404e0775 5d pop rbp
0x1404e0776 c3 ret 
0x1404e0777 660f1f840000000000 nop word ptr [rax + rax]
0x1404e0780 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e0785 55 push rbp
0x1404e0786 4157 push r15
0x1404e0788 4156 push r14
0x1404e078a 4155 push r13
0x1404e078c 4154 push r12
0x1404e078e 56 push rsi
0x1404e078f 57 push rdi
0x1404e0790 53 push rbx
0x1404e0791 4881ece8000000 sub rsp, 0xe8
0x1404e0798 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e079f 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e07a6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e07ac 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e07b3 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e07b9 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e07c2 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e07cc 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e07d6 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e07df 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e07e8 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e07f1 0fb69598140000 movzx edx, byte ptr [rbp + 0x1498]
0x1404e07f8 80e201 and dl, 1
0x1404e07fb c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404e0802 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404e0809 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404e0810 488b8d78140000 mov rcx, qword ptr [rbp + 0x1478]
0x1404e0817 e8e427b6ff call 0x140043000 ; core::ptr::drop_in_place<std::sync::poison::mutex::MutexGuard<std::sync::mpmc::zero::Inner> >
0x1404e081c c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404e0823 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404e082a c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404e0831 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e083a 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0843 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e084c 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0856 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e0860 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0869 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e086f 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e0876 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e087c 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e0883 4881c4e8000000 add rsp, 0xe8
0x1404e088a 5b pop rbx
0x1404e088b 5f pop rdi
0x1404e088c 5e pop rsi
0x1404e088d 415c pop r12
0x1404e088f 415d pop r13
0x1404e0891 415e pop r14
0x1404e0893 415f pop r15
0x1404e0895 5d pop rbp
0x1404e0896 c3 ret 
0x1404e0897 660f1f840000000000 nop word ptr [rax + rax]
0x1404e08a0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e08a5 55 push rbp
0x1404e08a6 4157 push r15
0x1404e08a8 4156 push r14
0x1404e08aa 4155 push r13
0x1404e08ac 4154 push r12
0x1404e08ae 56 push rsi
0x1404e08af 57 push rdi
0x1404e08b0 53 push rbx
0x1404e08b1 4881ece8000000 sub rsp, 0xe8
0x1404e08b8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e08bf 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e08c6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e08cc 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e08d3 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e08d9 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e08e2 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e08ec 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e08f6 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e08ff 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0908 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e0911 c6859614000000 mov byte ptr [rbp + 0x1496], 0
0x1404e0918 8bb598140000 mov esi, dword ptr [rbp + 0x1498]
0x1404e091e 4088b595140000 mov byte ptr [rbp + 0x1495], sil
0x1404e0925 488d8d08060000 lea rcx, [rbp + 0x608]
0x1404e092c e8ff0d1100 call 0x1405f1730 ; core::ptr::drop_in_place<bevy_input::keyboard::KeyboardInput>
0x1404e0931 c6859614000000 mov byte ptr [rbp + 0x1496], 0
0x1404e0938 4088b595140000 mov byte ptr [rbp + 0x1495], sil
0x1404e093f 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0948 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0951 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e095a 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0964 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e096e 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0977 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e097d 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e0984 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e098a 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e0991 4881c4e8000000 add rsp, 0xe8
0x1404e0998 5b pop rbx
0x1404e0999 5f pop rdi
0x1404e099a 5e pop rsi
0x1404e099b 415c pop r12
0x1404e099d 415d pop r13
0x1404e099f 415e pop r14
0x1404e09a1 415f pop r15
0x1404e09a3 5d pop rbp
0x1404e09a4 c3 ret 
0x1404e09a5 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404e09b0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e09b5 55 push rbp
0x1404e09b6 4157 push r15
0x1404e09b8 4156 push r14
0x1404e09ba 4155 push r13
0x1404e09bc 4154 push r12
0x1404e09be 56 push rsi
0x1404e09bf 57 push rdi
0x1404e09c0 53 push rbx
0x1404e09c1 4881ece8000000 sub rsp, 0xe8
0x1404e09c8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e09cf 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e09d6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e09dc 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e09e3 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e09e9 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e09f2 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e09fc 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0a06 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e0a0f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0a18 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e0a21 488d95300c0000 lea rdx, [rbp + 0xc30]
0x1404e0a28 41b828060000 mov r8d, 0x628
0x1404e0a2e 488b8d50140000 mov rcx, qword ptr [rbp + 0x1450]
0x1404e0a35 e8c6314402 call 0x142923c00 ; memcpy
0x1404e0a3a c685a414000000 mov byte ptr [rbp + 0x14a4], 0
0x1404e0a41 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0a4a 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0a53 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0a5c 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0a66 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e0a70 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0a79 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0a7f 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e0a86 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e0a8c 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e0a93 4881c4e8000000 add rsp, 0xe8
0x1404e0a9a 5b pop rbx
0x1404e0a9b 5f pop rdi
0x1404e0a9c 5e pop rsi
0x1404e0a9d 415c pop r12
0x1404e0a9f 415d pop r13
0x1404e0aa1 415e pop r14
0x1404e0aa3 415f pop r15
0x1404e0aa5 5d pop rbp
0x1404e0aa6 c3 ret 
0x1404e0aa7 660f1f840000000000 nop word ptr [rax + rax]
0x1404e0ab0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e0ab5 55 push rbp
0x1404e0ab6 4157 push r15
0x1404e0ab8 4156 push r14
0x1404e0aba 4155 push r13
0x1404e0abc 4154 push r12
0x1404e0abe 56 push rsi
0x1404e0abf 57 push rdi
0x1404e0ac0 53 push rbx
0x1404e0ac1 4881ece8000000 sub rsp, 0xe8
0x1404e0ac8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e0acf 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e0ad6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e0adc 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e0ae3 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e0ae9 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e0af2 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e0afc 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0b06 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e0b0f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0b18 660f7fb424d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404e0b21 80bd9814000000 cmp byte ptr [rbp + 0x1498], 0
0x1404e0b28 7429 je 0x1404e0b53
0x1404e0b2a 488d85380c0000 lea rax, [rbp + 0xc38]
0x1404e0b31 488945c0 mov qword ptr [rbp - 0x40], rax
0x1404e0b35 488d45c0 lea rax, [rbp - 0x40]
0x1404e0b39 48898500060000 mov qword ptr [rbp + 0x600], rax
0x1404e0b40 488d0d498e5002 lea rcx, [rip + 0x2508e49]
0x1404e0b47 488d9500060000 lea rdx, [rbp + 0x600]
0x1404e0b4e e82d1abcff call 0x1400a2580 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
0x1404e0b53 660f6fb424d0000000 movdqa xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0b5c 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0b65 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0b6e 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0b78 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e0b82 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0b8b 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0b91 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e0b98 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e0b9e 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e0ba5 4881c4e8000000 add rsp, 0xe8
0x1404e0bac 5b pop rbx
0x1404e0bad 5f pop rdi
0x1404e0bae 5e pop rsi
0x1404e0baf 415c pop r12
0x1404e0bb1 415d pop r13
0x1404e0bb3 415e pop r14
0x1404e0bb5 415f pop r15
0x1404e0bb7 5d pop rbp
0x1404e0bb8 c3 ret 
0x1404e0bb9 0f1f8000000000 nop dword ptr [rax]
0x1404e0bc0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e0bc5 55 push rbp
0x1404e0bc6 4157 push r15
0x1404e0bc8 4156 push r14
0x1404e0bca 4155 push r13
0x1404e0bcc 4154 push r12
0x1404e0bce 56 push rsi
0x1404e0bcf 57 push rdi
0x1404e0bd0 53 push rbx
0x1404e0bd1 4881ece8000000 sub rsp, 0xe8
0x1404e0bd8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e0bdf 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e0be6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e0bec 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e0bf3 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e0bf9 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e0c02 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e0c0c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0c16 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e0c1f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0c28 660f7fb424d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404e0c31 488b8d98140000 mov rcx, qword ptr [rbp + 0x1498]
0x1404e0c38 e8f30fc8ff call 0x140161c30 ; core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> > >
0x1404e0c3d 660f6fb424d0000000 movdqa xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0c46 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0c4f 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0c58 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0c62 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e0c6c 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0c75 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0c7b 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e0c82 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e0c88 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e0c8f 4881c4e8000000 add rsp, 0xe8
0x1404e0c96 5b pop rbx
0x1404e0c97 5f pop rdi
0x1404e0c98 5e pop rsi
0x1404e0c99 415c pop r12
0x1404e0c9b 415d pop r13
0x1404e0c9d 415e pop r14
0x1404e0c9f 415f pop r15
0x1404e0ca1 5d pop rbp
0x1404e0ca2 c3 ret 
0x1404e0ca3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404e0cb0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e0cb5 55 push rbp
0x1404e0cb6 4157 push r15
0x1404e0cb8 4156 push r14
0x1404e0cba 4155 push r13
0x1404e0cbc 4154 push r12
0x1404e0cbe 56 push rsi
0x1404e0cbf 57 push rdi
0x1404e0cc0 53 push rbx
0x1404e0cc1 4881ece8000000 sub rsp, 0xe8
0x1404e0cc8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e0ccf 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e0cd6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e0cdc 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e0ce3 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e0ce9 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e0cf2 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e0cfc 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0d06 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e0d0f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0d18 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e0d21 488d9500060000 lea rdx, [rbp + 0x600]
0x1404e0d28 41b8c8000000 mov r8d, 0xc8
0x1404e0d2e 488b8d98140000 mov rcx, qword ptr [rbp + 0x1498]
0x1404e0d35 e8c62e4402 call 0x142923c00 ; memcpy
0x1404e0d3a c6857614000000 mov byte ptr [rbp + 0x1476], 0
0x1404e0d41 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0d4a 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0d53 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0d5c 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0d66 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e0d70 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0d79 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0d7f 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e0d86 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e0d8c 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e0d93 4881c4e8000000 add rsp, 0xe8
0x1404e0d9a 5b pop rbx
0x1404e0d9b 5f pop rdi
0x1404e0d9c 5e pop rsi
0x1404e0d9d 415c pop r12
0x1404e0d9f 415d pop r13
0x1404e0da1 415e pop r14
0x1404e0da3 415f pop r15
0x1404e0da5 5d pop rbp
0x1404e0da6 c3 ret 
0x1404e0da7 660f1f840000000000 nop word ptr [rax + rax]
0x1404e0db0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e0db5 55 push rbp
0x1404e0db6 4157 push r15
0x1404e0db8 4156 push r14
0x1404e0dba 4155 push r13
0x1404e0dbc 4154 push r12
0x1404e0dbe 56 push rsi
0x1404e0dbf 57 push rdi
0x1404e0dc0 53 push rbx
0x1404e0dc1 4881ece8000000 sub rsp, 0xe8
0x1404e0dc8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e0dcf 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e0dd6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e0ddc 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e0de3 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e0de9 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e0df2 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e0dfc 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0e06 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e0e0f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0e18 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e0e21 4883bd9814000000 cmp qword ptr [rbp + 0x1498], 0
0x1404e0e29 7419 je 0x1404e0e44
0x1404e0e2b 41b801000000 mov r8d, 1
0x1404e0e31 488b8d68140000 mov rcx, qword ptr [rbp + 0x1468]
0x1404e0e38 488b9598140000 mov rdx, qword ptr [rbp + 0x1498]
0x1404e0e3f e8dc2d1300 call 0x140613c20 ; __rust_dealloc
0x1404e0e44 c6857514000000 mov byte ptr [rbp + 0x1475], 0
0x1404e0e4b 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0e54 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0e5d 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0e66 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0e70 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e0e7a 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0e83 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0e89 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e0e90 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e0e96 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e0e9d 4881c4e8000000 add rsp, 0xe8
0x1404e0ea4 5b pop rbx
0x1404e0ea5 5f pop rdi
0x1404e0ea6 5e pop rsi
0x1404e0ea7 415c pop r12
0x1404e0ea9 415d pop r13
0x1404e0eab 415e pop r14
0x1404e0ead 415f pop r15
0x1404e0eaf 5d pop rbp
0x1404e0eb0 c3 ret 
0x1404e0eb1 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404e0ec0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e0ec5 55 push rbp
0x1404e0ec6 4157 push r15
0x1404e0ec8 4156 push r14
0x1404e0eca 4155 push r13
0x1404e0ecc 4154 push r12
0x1404e0ece 56 push rsi
0x1404e0ecf 57 push rdi
0x1404e0ed0 53 push rbx
0x1404e0ed1 4881ece8000000 sub rsp, 0xe8
0x1404e0ed8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e0edf 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e0ee6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e0eec 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e0ef3 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e0ef9 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e0f02 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e0f0c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0f16 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e0f1f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e0f28 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e0f31 488d8d300c0000 lea rcx, [rbp + 0xc30]
0x1404e0f38 e833190f00 call 0x1405d2870 ; core::ptr::drop_in_place<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<alloc::collections::vec_deque::VecDeque<enum2$<tiny_http::util::messages_queue::Control<enum2$<tiny_http::Message> > >,alloc::alloc::Global> > > >
0x1404e0f3d 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e0f45 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e0f4d 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e0f56 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e0f5f 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e0f68 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e0f71 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e0f77 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e0f7d 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e0f83 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e0f89 4881c4e8000000 add rsp, 0xe8
0x1404e0f90 5b pop rbx
0x1404e0f91 5f pop rdi
0x1404e0f92 5e pop rsi
0x1404e0f93 415c pop r12
0x1404e0f95 415d pop r13
0x1404e0f97 415e pop r14
0x1404e0f99 415f pop r15
0x1404e0f9b 5d pop rbp
0x1404e0f9c c3 ret 
0x1404e0f9d 0f1f00 nop dword ptr [rax]
0x1404e0fa0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e0fa5 55 push rbp
0x1404e0fa6 4157 push r15
0x1404e0fa8 4156 push r14
0x1404e0faa 4155 push r13
0x1404e0fac 4154 push r12
0x1404e0fae 56 push rsi
0x1404e0faf 57 push rdi
0x1404e0fb0 53 push rbx
0x1404e0fb1 4881ece8000000 sub rsp, 0xe8
0x1404e0fb8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e0fbf 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e0fc6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e0fcc 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e0fd3 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e0fd9 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e0fe2 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e0fec 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e0ff6 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e0fff 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1008 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e1011 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404e1018 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404e101f c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404e1026 488d8d300c0000 lea rcx, [rbp + 0xc30]
0x1404e102d e83e180f00 call 0x1405d2870 ; core::ptr::drop_in_place<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<alloc::collections::vec_deque::VecDeque<enum2$<tiny_http::util::messages_queue::Control<enum2$<tiny_http::Message> > >,alloc::alloc::Global> > > >
0x1404e1032 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404e1039 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404e1040 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404e1047 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1050 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1059 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1062 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e106c 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e1076 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e107f 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1085 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e108c 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1092 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e1099 4881c4e8000000 add rsp, 0xe8
0x1404e10a0 5b pop rbx
0x1404e10a1 5f pop rdi
0x1404e10a2 5e pop rsi
0x1404e10a3 415c pop r12
0x1404e10a5 415d pop r13
0x1404e10a7 415e pop r14
0x1404e10a9 415f pop r15
0x1404e10ab 5d pop rbp
0x1404e10ac c3 ret 
0x1404e10ad 0f1f00 nop dword ptr [rax]
0x1404e10b0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e10b5 55 push rbp
0x1404e10b6 4157 push r15
0x1404e10b8 4156 push r14
0x1404e10ba 4155 push r13
0x1404e10bc 4154 push r12
0x1404e10be 56 push rsi
0x1404e10bf 57 push rdi
0x1404e10c0 53 push rbx
0x1404e10c1 4881ece8000000 sub rsp, 0xe8
0x1404e10c8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e10cf 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e10d6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e10dc 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e10e3 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e10e9 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e10f2 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e10fc 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1106 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e110f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1118 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e1121 80bd7414000000 cmp byte ptr [rbp + 0x1474], 0
0x1404e1128 7477 je 0x1404e11a1
0x1404e112a 31c0 xor eax, eax
0x1404e112c 483b85300c0000 cmp rax, qword ptr [rbp + 0xc30]
0x1404e1133 700c jo 0x1404e1141
0x1404e1135 488d8d300c0000 lea rcx, [rbp + 0xc30]
0x1404e113c e83fca1100 call 0x1405fdb80 ; core::ptr::drop_in_place<country_core::render::render_config::VideoSettings>
0x1404e1141 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1149 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1151 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e115a 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1163 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e116c 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1175 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e117b 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e1181 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1187 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e118d 4881c4e8000000 add rsp, 0xe8
0x1404e1194 5b pop rbx
0x1404e1195 5f pop rdi
0x1404e1196 5e pop rsi
0x1404e1197 415c pop r12
0x1404e1199 415d pop r13
0x1404e119b 415e pop r14
0x1404e119d 415f pop r15
0x1404e119f 5d pop rbp
0x1404e11a0 c3 ret 
0x1404e11a1 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e11a9 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e11b1 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e11ba 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e11c3 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e11cc 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e11d5 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e11db 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e11e1 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e11e7 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e11ed 4881c4e8000000 add rsp, 0xe8
0x1404e11f4 5b pop rbx
0x1404e11f5 5f pop rdi
0x1404e11f6 5e pop rsi
0x1404e11f7 415c pop r12
0x1404e11f9 415d pop r13
0x1404e11fb 415e pop r14
0x1404e11fd 415f pop r15
0x1404e11ff 5d pop rbp
0x1404e1200 c3 ret 
0x1404e1201 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404e1210 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e1215 55 push rbp
0x1404e1216 4157 push r15
0x1404e1218 4156 push r14
0x1404e121a 4155 push r13
0x1404e121c 4154 push r12
0x1404e121e 56 push rsi
0x1404e121f 57 push rdi
0x1404e1220 53 push rbx
0x1404e1221 4881ece8000000 sub rsp, 0xe8
0x1404e1228 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e122f 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e1236 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e123c 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1243 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1249 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e1252 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e125c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1266 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e126f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1278 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e1281 80bd9714000000 cmp byte ptr [rbp + 0x1497], 0
0x1404e1288 746c je 0x1404e12f6
0x1404e128a 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404e1291 e8ca781000 call 0x1405e8b60 ; core::ptr::drop_in_place<bevy_app::sub_app::SubApp>
0x1404e1296 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e129e 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e12a6 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e12af 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e12b8 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e12c1 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e12ca 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e12d0 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e12d6 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e12dc 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e12e2 4881c4e8000000 add rsp, 0xe8
0x1404e12e9 5b pop rbx
0x1404e12ea 5f pop rdi
0x1404e12eb 5e pop rsi
0x1404e12ec 415c pop r12
0x1404e12ee 415d pop r13
0x1404e12f0 415e pop r14
0x1404e12f2 415f pop r15
0x1404e12f4 5d pop rbp
0x1404e12f5 c3 ret 
0x1404e12f6 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e12fe 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1306 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e130f 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1318 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e1321 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e132a 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1330 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e1336 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e133c 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e1342 4881c4e8000000 add rsp, 0xe8
0x1404e1349 5b pop rbx
0x1404e134a 5f pop rdi
0x1404e134b 5e pop rsi
0x1404e134c 415c pop r12
0x1404e134e 415d pop r13
0x1404e1350 415e pop r14
0x1404e1352 415f pop r15
0x1404e1354 5d pop rbp
0x1404e1355 c3 ret 
0x1404e1356 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404e1360 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e1365 55 push rbp
0x1404e1366 4157 push r15
0x1404e1368 4156 push r14
0x1404e136a 4155 push r13
0x1404e136c 4154 push r12
0x1404e136e 56 push rsi
0x1404e136f 57 push rdi
0x1404e1370 53 push rbx
0x1404e1371 4881ece8000000 sub rsp, 0xe8
0x1404e1378 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e137f 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e1386 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e138c 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1393 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1399 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e13a2 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e13ac 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e13b6 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e13bf 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e13c8 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e13d1 0fb69d95140000 movzx ebx, byte ptr [rbp + 0x1495]
0x1404e13d8 80bd9614000000 cmp byte ptr [rbp + 0x1496], 0
0x1404e13df 7475 je 0x1404e1456
0x1404e13e1 488d4dc0 lea rcx, [rbp - 0x40]
0x1404e13e5 e846031100 call 0x1405f1730 ; core::ptr::drop_in_place<bevy_input::keyboard::KeyboardInput>
0x1404e13ea 889da2140000 mov byte ptr [rbp + 0x14a2], bl
0x1404e13f0 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e13f9 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1402 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e140b 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1415 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e141f 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1428 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e142e 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e1435 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e143b 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e1442 4881c4e8000000 add rsp, 0xe8
0x1404e1449 5b pop rbx
0x1404e144a 5f pop rdi
0x1404e144b 5e pop rsi
0x1404e144c 415c pop r12
0x1404e144e 415d pop r13
0x1404e1450 415e pop r14
0x1404e1452 415f pop r15
0x1404e1454 5d pop rbp
0x1404e1455 c3 ret 
0x1404e1456 889da2140000 mov byte ptr [rbp + 0x14a2], bl
0x1404e145c 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1465 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e146e 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1477 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1481 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e148b 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1494 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e149a 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e14a1 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e14a7 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e14ae 4881c4e8000000 add rsp, 0xe8
0x1404e14b5 5b pop rbx
0x1404e14b6 5f pop rdi
0x1404e14b7 5e pop rsi
0x1404e14b8 415c pop r12
0x1404e14ba 415d pop r13
0x1404e14bc 415e pop r14
0x1404e14be 415f pop r15
0x1404e14c0 5d pop rbp
0x1404e14c1 c3 ret 
0x1404e14c2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404e14d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e14d5 55 push rbp
0x1404e14d6 4157 push r15
0x1404e14d8 4156 push r14
0x1404e14da 4155 push r13
0x1404e14dc 4154 push r12
0x1404e14de 56 push rsi
0x1404e14df 57 push rdi
0x1404e14e0 53 push rbx
0x1404e14e1 4881ece8000000 sub rsp, 0xe8
0x1404e14e8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e14ef 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e14f6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e14fc 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1503 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1509 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e1512 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e151c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1526 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e152f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1538 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e1541 80bd7614000000 cmp byte ptr [rbp + 0x1476], 0
0x1404e1548 0f8484000000 je 0x1404e15d2
0x1404e154e 31c0 xor eax, eax
0x1404e1550 483b8500060000 cmp rax, qword ptr [rbp + 0x600]
0x1404e1557 700c jo 0x1404e1565
0x1404e1559 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404e1560 e81bc61100 call 0x1405fdb80 ; core::ptr::drop_in_place<country_core::render::render_config::VideoSettings>
0x1404e1565 c6858714000000 mov byte ptr [rbp + 0x1487], 0
0x1404e156c 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1575 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e157e 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1587 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1591 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e159b 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e15a4 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e15aa 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e15b1 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e15b7 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e15be 4881c4e8000000 add rsp, 0xe8
0x1404e15c5 5b pop rbx
0x1404e15c6 5f pop rdi
0x1404e15c7 5e pop rsi
0x1404e15c8 415c pop r12
0x1404e15ca 415d pop r13
0x1404e15cc 415e pop r14
0x1404e15ce 415f pop r15
0x1404e15d0 5d pop rbp
0x1404e15d1 c3 ret 
0x1404e15d2 c6858714000000 mov byte ptr [rbp + 0x1487], 0
0x1404e15d9 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e15e2 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e15eb 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e15f4 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e15fe 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e1608 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1611 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1617 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e161e 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1624 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e162b 4881c4e8000000 add rsp, 0xe8
0x1404e1632 5b pop rbx
0x1404e1633 5f pop rdi
0x1404e1634 5e pop rsi
0x1404e1635 415c pop r12
0x1404e1637 415d pop r13
0x1404e1639 415e pop r14
0x1404e163b 415f pop r15
0x1404e163d 5d pop rbp
0x1404e163e c3 ret 
0x1404e163f 90 nop 
0x1404e1640 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e1645 55 push rbp
0x1404e1646 4157 push r15
0x1404e1648 4156 push r14
0x1404e164a 4155 push r13
0x1404e164c 4154 push r12
0x1404e164e 56 push rsi
0x1404e164f 57 push rdi
0x1404e1650 53 push rbx
0x1404e1651 4881ece8000000 sub rsp, 0xe8
0x1404e1658 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e165f 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e1666 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e166c 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1673 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1679 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e1682 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e168c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1696 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e169f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e16a8 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e16b1 80bd7514000000 cmp byte ptr [rbp + 0x1475], 0
0x1404e16b8 0f849e000000 je 0x1404e175c
0x1404e16be 4883bd9814000000 cmp qword ptr [rbp + 0x1498], 0
0x1404e16c6 7419 je 0x1404e16e1
0x1404e16c8 41b801000000 mov r8d, 1
0x1404e16ce 488b8d68140000 mov rcx, qword ptr [rbp + 0x1468]
0x1404e16d5 488b9598140000 mov rdx, qword ptr [rbp + 0x1498]
0x1404e16dc e83f251300 call 0x140613c20 ; __rust_dealloc
0x1404e16e1 c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404e16e8 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404e16ef c685a514000000 mov byte ptr [rbp + 0x14a5], 0
0x1404e16f6 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e16ff 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1708 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1711 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e171b 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e1725 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e172e 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1734 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e173b 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1741 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e1748 4881c4e8000000 add rsp, 0xe8
0x1404e174f 5b pop rbx
0x1404e1750 5f pop rdi
0x1404e1751 5e pop rsi
0x1404e1752 415c pop r12
0x1404e1754 415d pop r13
0x1404e1756 415e pop r14
0x1404e1758 415f pop r15
0x1404e175a 5d pop rbp
0x1404e175b c3 ret 
0x1404e175c c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404e1763 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404e176a c685a514000000 mov byte ptr [rbp + 0x14a5], 0
0x1404e1771 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e177a 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1783 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e178c 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1796 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e17a0 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e17a9 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e17af 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e17b6 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e17bc 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e17c3 4881c4e8000000 add rsp, 0xe8
0x1404e17ca 5b pop rbx
0x1404e17cb 5f pop rdi
0x1404e17cc 5e pop rsi
0x1404e17cd 415c pop r12
0x1404e17cf 415d pop r13
0x1404e17d1 415e pop r14
0x1404e17d3 415f pop r15
0x1404e17d5 5d pop rbp
0x1404e17d6 c3 ret 
0x1404e17d7 660f1f840000000000 nop word ptr [rax + rax]
0x1404e17e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e17e5 55 push rbp
0x1404e17e6 4157 push r15
0x1404e17e8 4156 push r14
0x1404e17ea 4155 push r13
0x1404e17ec 4154 push r12
0x1404e17ee 56 push rsi
0x1404e17ef 57 push rdi
0x1404e17f0 53 push rbx
0x1404e17f1 4881ece8000000 sub rsp, 0xe8
0x1404e17f8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e17ff 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e1806 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e180c 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1813 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1819 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e1822 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e182c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1836 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e183f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1848 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e1851 0fb69da2140000 movzx ebx, byte ptr [rbp + 0x14a2]
0x1404e1858 488d8d300c0000 lea rcx, [rbp + 0xc30]
0x1404e185f e8cc63ffff call 0x1404d7c30 ; core::ptr::drop_in_place<enum2$<winit::keyboard::Key<smol_str::SmolStr> > >
0x1404e1864 0fb685900c0000 movzx eax, byte ptr [rbp + 0xc90]
0x1404e186b 3c1b cmp al, 0x1b
0x1404e186d 0f95c1 setne cl
0x1404e1870 84cb test bl, cl
0x1404e1872 741d je 0x1404e1891
0x1404e1874 3c18 cmp al, 0x18
0x1404e1876 7519 jne 0x1404e1891
0x1404e1878 488b85980c0000 mov rax, qword ptr [rbp + 0xc98]
0x1404e187f f048ff08 lock dec qword ptr [rax]
0x1404e1883 750c jne 0x1404e1891
0x1404e1885 488d8d980c0000 lea rcx, [rbp + 0xc98]
0x1404e188c e89fd21600 call 0x14064eb30 ; alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global>
0x1404e1891 488d8d500c0000 lea rcx, [rbp + 0xc50]
0x1404e1898 e873881100 call 0x1405fa110 ; core::ptr::drop_in_place<winit::platform_impl::platform::KeyEventExtra>
0x1404e189d c685a714000001 mov byte ptr [rbp + 0x14a7], 1
0x1404e18a4 c685a614000000 mov byte ptr [rbp + 0x14a6], 0
0x1404e18ab c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404e18b2 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e18bb 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e18c4 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e18cd 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e18d7 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e18e1 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e18ea 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e18f0 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e18f7 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e18fd 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e1904 4881c4e8000000 add rsp, 0xe8
0x1404e190b 5b pop rbx
0x1404e190c 5f pop rdi
0x1404e190d 5e pop rsi
0x1404e190e 415c pop r12
0x1404e1910 415d pop r13
0x1404e1912 415e pop r14
0x1404e1914 415f pop r15
0x1404e1916 5d pop rbp
0x1404e1917 c3 ret 
0x1404e1918 0f1f840000000000 nop dword ptr [rax + rax]
0x1404e1920 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e1925 55 push rbp
0x1404e1926 4157 push r15
0x1404e1928 4156 push r14
0x1404e192a 4155 push r13
0x1404e192c 4154 push r12
0x1404e192e 56 push rsi
0x1404e192f 57 push rdi
0x1404e1930 53 push rbx
0x1404e1931 4881ece8000000 sub rsp, 0xe8
0x1404e1938 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e193f 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e1946 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e194c 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1953 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1959 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e1962 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e196c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1976 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e197f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1988 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e1991 4883bd7814000000 cmp qword ptr [rbp + 0x1478], 0
0x1404e1999 750c jne 0x1404e19a7
0x1404e199b 488d8d300c0000 lea rcx, [rbp + 0xc30]
0x1404e19a2 e85947eeff call 0x1403c6100 ; core::ptr::drop_in_place<enum2$<std::sync::mpmc::error::SendTimeoutError<enum2$<tiny_glade::event_loop::GameEvent> > > >
0x1404e19a7 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e19af 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e19b7 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e19c0 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e19c9 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e19d2 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e19db 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e19e1 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e19e7 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e19ed 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e19f3 4881c4e8000000 add rsp, 0xe8
0x1404e19fa 5b pop rbx
0x1404e19fb 5f pop rdi
0x1404e19fc 5e pop rsi
0x1404e19fd 415c pop r12
0x1404e19ff 415d pop r13
0x1404e1a01 415e pop r14
0x1404e1a03 415f pop r15
0x1404e1a05 5d pop rbp
0x1404e1a06 c3 ret 
0x1404e1a07 660f1f840000000000 nop word ptr [rax + rax]
0x1404e1a10 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e1a15 55 push rbp
0x1404e1a16 4157 push r15
0x1404e1a18 4156 push r14
0x1404e1a1a 4155 push r13
0x1404e1a1c 4154 push r12
0x1404e1a1e 56 push rsi
0x1404e1a1f 57 push rdi
0x1404e1a20 53 push rbx
0x1404e1a21 4881ece8000000 sub rsp, 0xe8
0x1404e1a28 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e1a2f 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e1a36 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e1a3c 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1a43 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1a49 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e1a52 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e1a5c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1a66 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e1a6f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1a78 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e1a81 80bd8714000000 cmp byte ptr [rbp + 0x1487], 0
0x1404e1a88 0f84a8000000 je 0x1404e1b36
0x1404e1a8e 48b80200000000000080 movabs rax, 0x8000000000000002
0x1404e1a98 488b9568140000 mov rdx, qword ptr [rbp + 0x1468]
0x1404e1a9f 4839c2 cmp rdx, rax
0x1404e1aa2 7c17 jl 0x1404e1abb
0x1404e1aa4 4885d2 test rdx, rdx
0x1404e1aa7 7412 je 0x1404e1abb
0x1404e1aa9 41b801000000 mov r8d, 1
0x1404e1aaf 488b8d60140000 mov rcx, qword ptr [rbp + 0x1460]
0x1404e1ab6 e865211300 call 0x140613c20 ; __rust_dealloc
0x1404e1abb c685a714000000 mov byte ptr [rbp + 0x14a7], 0
0x1404e1ac2 c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404e1ac9 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404e1ad0 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1ad9 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1ae2 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1aeb 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1af5 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e1aff 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1b08 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1b0e 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e1b15 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1b1b 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e1b22 4881c4e8000000 add rsp, 0xe8
0x1404e1b29 5b pop rbx
0x1404e1b2a 5f pop rdi
0x1404e1b2b 5e pop rsi
0x1404e1b2c 415c pop r12
0x1404e1b2e 415d pop r13
0x1404e1b30 415e pop r14
0x1404e1b32 415f pop r15
0x1404e1b34 5d pop rbp
0x1404e1b35 c3 ret 
0x1404e1b36 c685a714000000 mov byte ptr [rbp + 0x14a7], 0
0x1404e1b3d c685a614000001 mov byte ptr [rbp + 0x14a6], 1
0x1404e1b44 c685a514000001 mov byte ptr [rbp + 0x14a5], 1
0x1404e1b4b 660f28b424d0000000 movapd xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1b54 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1b5d 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1b66 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1b70 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e1b7a 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1b83 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1b89 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e1b90 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1b96 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e1b9d 4881c4e8000000 add rsp, 0xe8
0x1404e1ba4 5b pop rbx
0x1404e1ba5 5f pop rdi
0x1404e1ba6 5e pop rsi
0x1404e1ba7 415c pop r12
0x1404e1ba9 415d pop r13
0x1404e1bab 415e pop r14
0x1404e1bad 415f pop r15
0x1404e1baf 5d pop rbp
0x1404e1bb0 c3 ret 
0x1404e1bb1 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404e1bc0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e1bc5 55 push rbp
0x1404e1bc6 4157 push r15
0x1404e1bc8 4156 push r14
0x1404e1bca 4155 push r13
0x1404e1bcc 4154 push r12
0x1404e1bce 56 push rsi
0x1404e1bcf 57 push rdi
0x1404e1bd0 53 push rbx
0x1404e1bd1 4881ece8000000 sub rsp, 0xe8
0x1404e1bd8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e1bdf 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e1be6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e1bec 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1bf3 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1bf9 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e1c02 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e1c0c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1c16 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e1c1f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1c28 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e1c31 488d8d300c0000 lea rcx, [rbp + 0xc30]
0x1404e1c38 e86370d8ff call 0x140268ca0 ; core::ptr::drop_in_place<std::sync::mpsc::SendError<enum2$<tiny_glade::event_loop::OsWindowEvent> > >
0x1404e1c3d 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1c45 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1c4d 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1c56 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1c5f 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e1c68 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1c71 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1c77 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e1c7d 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1c83 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e1c89 4881c4e8000000 add rsp, 0xe8
0x1404e1c90 5b pop rbx
0x1404e1c91 5f pop rdi
0x1404e1c92 5e pop rsi
0x1404e1c93 415c pop r12
0x1404e1c95 415d pop r13
0x1404e1c97 415e pop r14
0x1404e1c99 415f pop r15
0x1404e1c9b 5d pop rbp
0x1404e1c9c c3 ret 
0x1404e1c9d 0f1f00 nop dword ptr [rax]
0x1404e1ca0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e1ca5 55 push rbp
0x1404e1ca6 4157 push r15
0x1404e1ca8 4156 push r14
0x1404e1caa 4155 push r13
0x1404e1cac 4154 push r12
0x1404e1cae 56 push rsi
0x1404e1caf 57 push rdi
0x1404e1cb0 53 push rbx
0x1404e1cb1 4881ece8000000 sub rsp, 0xe8
0x1404e1cb8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e1cbf 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e1cc6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e1ccc 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1cd3 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1cd9 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e1ce2 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e1cec 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1cf6 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e1cff 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1d08 660f7fb424d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404e1d11 80bda314000000 cmp byte ptr [rbp + 0x14a3], 0
0x1404e1d18 7472 je 0x1404e1d8c
0x1404e1d1a 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404e1d21 e8ea801000 call 0x1405e9e10 ; core::ptr::drop_in_place<egui::data::input::RawInput>
0x1404e1d26 660f6fb424d0000000 movdqa xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1d2f 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1d38 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1d41 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1d4b 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e1d55 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1d5e 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1d64 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e1d6b 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1d71 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e1d78 4881c4e8000000 add rsp, 0xe8
0x1404e1d7f 5b pop rbx
0x1404e1d80 5f pop rdi
0x1404e1d81 5e pop rsi
0x1404e1d82 415c pop r12
0x1404e1d84 415d pop r13
0x1404e1d86 415e pop r14
0x1404e1d88 415f pop r15
0x1404e1d8a 5d pop rbp
0x1404e1d8b c3 ret 
0x1404e1d8c 660f6fb424d0000000 movdqa xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1d95 660f6fbc24c0000000 movdqa xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1d9e 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1da7 66440f6f8c24a0000000 movdqa xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1db1 66440f28942490000000 movapd xmm10, xmmword ptr [rsp + 0x90]
0x1404e1dbb 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1dc4 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1dca 66440f6f6c2460 movdqa xmm13, xmmword ptr [rsp + 0x60]
0x1404e1dd1 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1dd7 66440f6f7c2440 movdqa xmm15, xmmword ptr [rsp + 0x40]
0x1404e1dde 4881c4e8000000 add rsp, 0xe8
0x1404e1de5 5b pop rbx
0x1404e1de6 5f pop rdi
0x1404e1de7 5e pop rsi
0x1404e1de8 415c pop r12
0x1404e1dea 415d pop r13
0x1404e1dec 415e pop r14
0x1404e1dee 415f pop r15
0x1404e1df0 5d pop rbp
0x1404e1df1 c3 ret 
0x1404e1df2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404e1e00 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e1e05 55 push rbp
0x1404e1e06 4157 push r15
0x1404e1e08 4156 push r14
0x1404e1e0a 4155 push r13
0x1404e1e0c 4154 push r12
0x1404e1e0e 56 push rsi
0x1404e1e0f 57 push rdi
0x1404e1e10 53 push rbx
0x1404e1e11 4881ece8000000 sub rsp, 0xe8
0x1404e1e18 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e1e1f 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e1e26 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e1e2c 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1e33 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1e39 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e1e42 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e1e4c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1e56 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e1e5f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1e68 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e1e71 80bda414000000 cmp byte ptr [rbp + 0x14a4], 0
0x1404e1e78 746c je 0x1404e1ee6
0x1404e1e7a 488d8d00060000 lea rcx, [rbp + 0x600]
0x1404e1e81 e8da6c1000 call 0x1405e8b60 ; core::ptr::drop_in_place<bevy_app::sub_app::SubApp>
0x1404e1e86 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1e8e 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1e96 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1e9f 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1ea8 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e1eb1 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1eba 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1ec0 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e1ec6 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1ecc 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e1ed2 4881c4e8000000 add rsp, 0xe8
0x1404e1ed9 5b pop rbx
0x1404e1eda 5f pop rdi
0x1404e1edb 5e pop rsi
0x1404e1edc 415c pop r12
0x1404e1ede 415d pop r13
0x1404e1ee0 415e pop r14
0x1404e1ee2 415f pop r15
0x1404e1ee4 5d pop rbp
0x1404e1ee5 c3 ret 
0x1404e1ee6 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1eee 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1ef6 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1eff 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1f08 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e1f11 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e1f1a 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e1f20 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e1f26 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e1f2c 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e1f32 4881c4e8000000 add rsp, 0xe8
0x1404e1f39 5b pop rbx
0x1404e1f3a 5f pop rdi
0x1404e1f3b 5e pop rsi
0x1404e1f3c 415c pop r12
0x1404e1f3e 415d pop r13
0x1404e1f40 415e pop r14
0x1404e1f42 415f pop r15
0x1404e1f44 5d pop rbp
0x1404e1f45 c3 ret 
0x1404e1f46 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1404e1f50 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e1f55 55 push rbp
0x1404e1f56 4157 push r15
0x1404e1f58 4156 push r14
0x1404e1f5a 4155 push r13
0x1404e1f5c 4154 push r12
0x1404e1f5e 56 push rsi
0x1404e1f5f 57 push rdi
0x1404e1f60 53 push rbx
0x1404e1f61 4881ece8000000 sub rsp, 0xe8
0x1404e1f68 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e1f6f 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e1f76 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e1f7c 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e1f83 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e1f89 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e1f92 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e1f9c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e1fa6 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e1faf 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e1fb8 0f29b424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm6
0x1404e1fc0 488d8d300c0000 lea rcx, [rbp + 0xc30]
0x1404e1fc7 e8b4bb1100 call 0x1405fdb80 ; core::ptr::drop_in_place<country_core::render::render_config::VideoSettings>
0x1404e1fcc 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e1fd4 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e1fdc 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e1fe5 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e1fee 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e1ff7 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e2000 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e2006 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e200c 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e2012 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e2018 4881c4e8000000 add rsp, 0xe8
0x1404e201f 5b pop rbx
0x1404e2020 5f pop rdi
0x1404e2021 5e pop rsi
0x1404e2022 415c pop r12
0x1404e2024 415d pop r13
0x1404e2026 415e pop r14
0x1404e2028 415f pop r15
0x1404e202a 5d pop rbp
0x1404e202b c3 ret 
0x1404e202c 0f1f4000 nop dword ptr [rax]
0x1404e2030 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e2035 55 push rbp
0x1404e2036 4157 push r15
0x1404e2038 4156 push r14
0x1404e203a 4155 push r13
0x1404e203c 4154 push r12
0x1404e203e 56 push rsi
0x1404e203f 57 push rdi
0x1404e2040 53 push rbx
0x1404e2041 4881ece8000000 sub rsp, 0xe8
0x1404e2048 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e204f 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e2056 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e205c 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e2063 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e2069 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e2072 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e207c 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e2086 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e208f 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e2098 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e20a1 8b8da0120000 mov ecx, dword ptr [rbp + 0x12a0]
0x1404e20a7 83c1fc add ecx, -4
0x1404e20aa 6683f90e cmp cx, 0xe
0x1404e20ae b807000000 mov eax, 7
0x1404e20b3 0f42c1 cmovb eax, ecx
0x1404e20b6 6683f807 cmp ax, 7
0x1404e20ba 7575 jne 0x1404e2131
0x1404e20bc f685a614000001 test byte ptr [rbp + 0x14a6], 1
0x1404e20c3 740c je 0x1404e20d1
0x1404e20c5 488d8da0120000 lea rcx, [rbp + 0x12a0]
0x1404e20cc e82f5f1000 call 0x1405e8000 ; core::ptr::drop_in_place<winit::event::KeyEvent>
0x1404e20d1 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e20d9 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e20e1 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e20ea 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e20f3 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e20fc 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e2105 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e210b 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e2111 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e2117 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e211d 4881c4e8000000 add rsp, 0xe8
0x1404e2124 5b pop rbx
0x1404e2125 5f pop rdi
0x1404e2126 5e pop rsi
0x1404e2127 415c pop r12
0x1404e2129 415d pop r13
0x1404e212b 415e pop r14
0x1404e212d 415f pop r15
0x1404e212f 5d pop rbp
0x1404e2130 c3 ret 
0x1404e2131 0fb7c0 movzx eax, ax
0x1404e2134 83f805 cmp eax, 5
0x1404e2137 7417 je 0x1404e2150
0x1404e2139 83f80d cmp eax, 0xd
0x1404e213c 7593 jne 0x1404e20d1
0x1404e213e f685a514000001 test byte ptr [rbp + 0x14a5], 1
0x1404e2145 748a je 0x1404e20d1
0x1404e2147 488b95a8120000 mov rdx, qword ptr [rbp + 0x12a8]
0x1404e214e eb27 jmp 0x1404e2177
0x1404e2150 f685a714000001 test byte ptr [rbp + 0x14a7], 1
0x1404e2157 0f8474ffffff je 0x1404e20d1
0x1404e215d 488b95a8120000 mov rdx, qword ptr [rbp + 0x12a8]
0x1404e2164 48b80200000000000080 movabs rax, 0x8000000000000002
0x1404e216e 4839c2 cmp rdx, rax
0x1404e2171 0f8c5affffff jl 0x1404e20d1
0x1404e2177 4885d2 test rdx, rdx
0x1404e217a 0f8451ffffff je 0x1404e20d1
0x1404e2180 488b8db0120000 mov rcx, qword ptr [rbp + 0x12b0]
0x1404e2187 41b801000000 mov r8d, 1
0x1404e218d e88e1a1300 call 0x140613c20 ; __rust_dealloc
0x1404e2192 e93affffff jmp 0x1404e20d1
0x1404e2197 660f1f840000000000 nop word ptr [rax + rax]
0x1404e21a0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e21a5 55 push rbp
0x1404e21a6 4157 push r15
0x1404e21a8 4156 push r14
0x1404e21aa 4155 push r13
0x1404e21ac 4154 push r12
0x1404e21ae 56 push rsi
0x1404e21af 57 push rdi
0x1404e21b0 53 push rbx
0x1404e21b1 4881ece8000000 sub rsp, 0xe8
0x1404e21b8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e21bf 440f297c2440 movaps xmmword ptr [rsp + 0x40], xmm15
0x1404e21c5 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e21cb 440f296c2460 movaps xmmword ptr [rsp + 0x60], xmm13
0x1404e21d1 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e21d7 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e21e0 440f29942490000000 movaps xmmword ptr [rsp + 0x90], xmm10
0x1404e21e9 440f298c24a0000000 movaps xmmword ptr [rsp + 0xa0], xmm9
0x1404e21f2 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e21fb 0f29bc24c0000000 movaps xmmword ptr [rsp + 0xc0], xmm7
0x1404e2203 0f29b424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm6
0x1404e220b 488d8de0130000 lea rcx, [rbp + 0x13e0]
0x1404e2212 e8f96f1200 call 0x140609210 ; core::ptr::drop_in_place<alloc::vec::drain::Drain<enum2$<tiny_glade::event_loop::OsWindowEvent>,alloc::alloc::Global> >
0x1404e2217 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e221f 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e2227 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e2230 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e2239 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e2242 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e224b 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e2251 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e2257 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e225d 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e2263 4881c4e8000000 add rsp, 0xe8
0x1404e226a 5b pop rbx
0x1404e226b 5f pop rdi
0x1404e226c 5e pop rsi
0x1404e226d 415c pop r12
0x1404e226f 415d pop r13
0x1404e2271 415e pop r14
0x1404e2273 415f pop r15
0x1404e2275 5d pop rbp
0x1404e2276 c3 ret 
0x1404e2277 660f1f840000000000 nop word ptr [rax + rax]
0x1404e2280 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e2285 55 push rbp
0x1404e2286 4157 push r15
0x1404e2288 4156 push r14
0x1404e228a 4155 push r13
0x1404e228c 4154 push r12
0x1404e228e 56 push rsi
0x1404e228f 57 push rdi
0x1404e2290 53 push rbx
0x1404e2291 4881ece8000000 sub rsp, 0xe8
0x1404e2298 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e229f 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e22a6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e22ac 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e22b3 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e22b9 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e22c2 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e22cc 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e22d6 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e22df 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e22e8 660f7fb424d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm6
0x1404e22f1 4883bde013000000 cmp qword ptr [rbp + 0x13e0], 0
0x1404e22f9 742f je 0x1404e232a
0x1404e22fb 488d85e8130000 lea rax, [rbp + 0x13e8]
0x1404e2302 48898558140000 mov qword ptr [rbp + 0x1458], rax
0x1404e2309 488d8558140000 lea rax, [rbp + 0x1458]
0x1404e2310 48898530140000 mov qword ptr [rbp + 0x1430], rax
0x1404e2317 488d0d72765002 lea rcx, [rip + 0x2507672]
0x1404e231e 488d9530140000 lea rdx, [rbp + 0x1430]
0x1404e2325 e85602bcff call 0x1400a2580 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
0x1404e232a 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e2332 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e233a 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e2343 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e234c 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e2355 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e235e 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e2364 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e236a 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e2370 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e2376 4881c4e8000000 add rsp, 0xe8
0x1404e237d 5b pop rbx
0x1404e237e 5f pop rdi
0x1404e237f 5e pop rsi
0x1404e2380 415c pop r12
0x1404e2382 415d pop r13
0x1404e2384 415e pop r14
0x1404e2386 415f pop r15
0x1404e2388 5d pop rbp
0x1404e2389 c3 ret 
0x1404e238a 660f1f440000 nop word ptr [rax + rax]
0x1404e2390 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1404e2395 55 push rbp
0x1404e2396 4157 push r15
0x1404e2398 4156 push r14
0x1404e239a 4155 push r13
0x1404e239c 4154 push r12
0x1404e239e 56 push rsi
0x1404e239f 57 push rdi
0x1404e23a0 53 push rbx
0x1404e23a1 4881ece8000000 sub rsp, 0xe8
0x1404e23a8 488daa80000000 lea rbp, [rdx + 0x80]
0x1404e23af 66440f7f7c2440 movdqa xmmword ptr [rsp + 0x40], xmm15
0x1404e23b6 440f29742450 movaps xmmword ptr [rsp + 0x50], xmm14
0x1404e23bc 66440f7f6c2460 movdqa xmmword ptr [rsp + 0x60], xmm13
0x1404e23c3 440f29642470 movaps xmmword ptr [rsp + 0x70], xmm12
0x1404e23c9 440f299c2480000000 movaps xmmword ptr [rsp + 0x80], xmm11
0x1404e23d2 66440f29942490000000 movapd xmmword ptr [rsp + 0x90], xmm10
0x1404e23dc 66440f7f8c24a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm9
0x1404e23e6 440f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm8
0x1404e23ef 660f7fbc24c0000000 movdqa xmmword ptr [rsp + 0xc0], xmm7
0x1404e23f8 660f29b424d0000000 movapd xmmword ptr [rsp + 0xd0], xmm6
0x1404e2401 488d8db0130000 lea rcx, [rbp + 0x13b0]
0x1404e2408 e8e3111200 call 0x1406035f0 ; core::ptr::drop_in_place<alloc::vec::Vec<enum2$<tiny_glade::event_loop::OsWindowEvent>,alloc::alloc::Global> >
0x1404e240d 0f28b424d0000000 movaps xmm6, xmmword ptr [rsp + 0xd0]
0x1404e2415 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
0x1404e241d 440f288424b0000000 movaps xmm8, xmmword ptr [rsp + 0xb0]
0x1404e2426 440f288c24a0000000 movaps xmm9, xmmword ptr [rsp + 0xa0]
0x1404e242f 440f28942490000000 movaps xmm10, xmmword ptr [rsp + 0x90]
0x1404e2438 440f289c2480000000 movaps xmm11, xmmword ptr [rsp + 0x80]
0x1404e2441 440f28642470 movaps xmm12, xmmword ptr [rsp + 0x70]
0x1404e2447 440f286c2460 movaps xmm13, xmmword ptr [rsp + 0x60]
0x1404e244d 440f28742450 movaps xmm14, xmmword ptr [rsp + 0x50]
0x1404e2453 440f287c2440 movaps xmm15, xmmword ptr [rsp + 0x40]
0x1404e2459 4881c4e8000000 add rsp, 0xe8
0x1404e2460 5b pop rbx
0x1404e2461 5f pop rdi
0x1404e2462 5e pop rsi
0x1404e2463 415c pop r12
0x1404e2465 415d pop r13
0x1404e2467 415e pop r14
0x1404e2469 415f pop r15
0x1404e246b 5d pop rbp
0x1404e246c c3 ret 