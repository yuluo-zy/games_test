0x140a9f5c0 55 push rbp
0x140a9f5c1 56 push rsi
0x140a9f5c2 57 push rdi
0x140a9f5c3 4881ecb0010000 sub rsp, 0x1b0
0x140a9f5ca 488dac2480000000 lea rbp, [rsp + 0x80]
0x140a9f5d2 48c78528010000feffffff mov qword ptr [rbp + 0x128], 0xfffffffffffffffe
0x140a9f5dd 4889ce mov rsi, rcx
0x140a9f5e0 0f57c0 xorps xmm0, xmm0
0x140a9f5e3 0f1145c0 movups xmmword ptr [rbp - 0x40], xmm0
0x140a9f5e7 0f1145d8 movups xmmword ptr [rbp - 0x28], xmm0
0x140a9f5eb 0f1145f0 movups xmmword ptr [rbp - 0x10], xmm0
0x140a9f5ef 0f114508 movups xmmword ptr [rbp + 8], xmm0
0x140a9f5f3 0f114520 movups xmmword ptr [rbp + 0x20], xmm0
0x140a9f5f7 c7453000000000 mov dword ptr [rbp + 0x30], 0
0x140a9f5fe 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
0x140a9f602 0f114570 movups xmmword ptr [rbp + 0x70], xmm0
0x140a9f606 0f118588000000 movups xmmword ptr [rbp + 0x88], xmm0
0x140a9f60d 0f1185a0000000 movups xmmword ptr [rbp + 0xa0], xmm0
0x140a9f614 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
0x140a9f61b c785c800000000000000 mov dword ptr [rbp + 0xc8], 0
0x140a9f625 48c745a000000000 mov qword ptr [rbp - 0x60], 0
0x140a9f62d 48c745a808000000 mov qword ptr [rbp - 0x58], 8
0x140a9f635 48c745b000000000 mov qword ptr [rbp - 0x50], 0
0x140a9f63d 48c745b810000000 mov qword ptr [rbp - 0x48], 0x10
0x140a9f645 48c745d010000000 mov qword ptr [rbp - 0x30], 0x10
0x140a9f64d 48c745e810000000 mov qword ptr [rbp - 0x18], 0x10
0x140a9f655 48c7450010000000 mov qword ptr [rbp], 0x10
0x140a9f65d 48c7451810000000 mov qword ptr [rbp + 0x18], 0x10
0x140a9f665 48b80000000000000080 movabs rax, 0x8000000000000000
0x140a9f66f 48894538 mov qword ptr [rbp + 0x38], rax
0x140a9f673 488d0df3ca0302 lea rcx, [rip + 0x203caf3]
0x140a9f67a 48894d40 mov qword ptr [rbp + 0x40], rcx
0x140a9f67e 48c745482d000000 mov qword ptr [rbp + 0x48], 0x2d
0x140a9f686 48c7455010000000 mov qword ptr [rbp + 0x50], 0x10
0x140a9f68e 48c7456810000000 mov qword ptr [rbp + 0x68], 0x10
0x140a9f696 48c7858000000010000000 mov qword ptr [rbp + 0x80], 0x10
0x140a9f6a1 48c7859800000010000000 mov qword ptr [rbp + 0x98], 0x10
0x140a9f6ac 48c785b000000010000000 mov qword ptr [rbp + 0xb0], 0x10
0x140a9f6b7 c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
0x140a9f6c1 66c785d40000000100 mov word ptr [rbp + 0xd4], 1
0x140a9f6ca 488985e0000000 mov qword ptr [rbp + 0xe0], rax
0x140a9f6d1 c7852001000000000000 mov dword ptr [rbp + 0x120], 0
0x140a9f6db 0fb6052755b902 movzx eax, byte ptr [rip + 0x2b95527]
0x140a9f6e2 b988010000 mov ecx, 0x188
0x140a9f6e7 ba08000000 mov edx, 8
0x140a9f6ec e81f45b7ff call 0x140613c10
0x140a9f6f1 4885c0 test rax, rax
0x140a9f6f4 7435 je 0x140a9f72b
0x140a9f6f6 4889c7 mov rdi, rax
0x140a9f6f9 488d55a0 lea rdx, [rbp - 0x60]
0x140a9f6fd 41b888010000 mov r8d, 0x188
0x140a9f703 4889c1 mov rcx, rax
0x140a9f706 e8f544e801 call 0x142923c00
0x140a9f70b 4c8d0586c90302 lea r8, [rip + 0x203c986]
0x140a9f712 4889f1 mov rcx, rsi
0x140a9f715 4889fa mov rdx, rdi
0x140a9f718 e8f3532400 call 0x140ce4b10
0x140a9f71d 4889f0 mov rax, rsi
0x140a9f720 4881c4b0010000 add rsp, 0x1b0
0x140a9f727 5f pop rdi
0x140a9f728 5e pop rsi
0x140a9f729 5d pop rbp
0x140a9f72a c3 ret 
0x140a9f72b b908000000 mov ecx, 8
0x140a9f730 ba88010000 mov edx, 0x188
0x140a9f735 e8c698e301 call 0x1428d9000
0x140a9f73a 0f0b ud2 
0x140a9f73c 0f1f4000 nop dword ptr [rax]
0x140a9f740 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a9f745 55 push rbp
0x140a9f746 56 push rsi
0x140a9f747 57 push rdi
0x140a9f748 4883ec20 sub rsp, 0x20
0x140a9f74c 488daa80000000 lea rbp, [rdx + 0x80]
0x140a9f753 488d4da0 lea rcx, [rbp - 0x60]
0x140a9f757 e8e46fb4ff call 0x1405e6740
0x140a9f75c 90 nop 
0x140a9f75d 4883c420 add rsp, 0x20
0x140a9f761 5f pop rdi
0x140a9f762 5e pop rsi
0x140a9f763 5d pop rbp
0x140a9f764 c3 ret 
