0x14050f480 55 push rbp
0x14050f481 56 push rsi
0x14050f482 57 push rdi
0x14050f483 4881ecf0020000 sub rsp, 0x2f0
0x14050f48a 488dac2480000000 lea rbp, [rsp + 0x80]
0x14050f492 48c78568020000feffffff mov qword ptr [rbp + 0x268], 0xfffffffffffffffe
0x14050f49d 4889ce mov rsi, rcx
0x14050f4a0 0f57c0 xorps xmm0, xmm0
0x14050f4a3 0f1145c0 movups xmmword ptr [rbp - 0x40], xmm0
0x14050f4a7 0f1145d8 movups xmmword ptr [rbp - 0x28], xmm0
0x14050f4ab 0f1145f0 movups xmmword ptr [rbp - 0x10], xmm0
0x14050f4af 0f114508 movups xmmword ptr [rbp + 8], xmm0
0x14050f4b3 0f114520 movups xmmword ptr [rbp + 0x20], xmm0
0x14050f4b7 c7453000000000 mov dword ptr [rbp + 0x30], 0
0x14050f4be 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
0x14050f4c2 0f114570 movups xmmword ptr [rbp + 0x70], xmm0
0x14050f4c6 0f118588000000 movups xmmword ptr [rbp + 0x88], xmm0
0x14050f4cd 0f1185a0000000 movups xmmword ptr [rbp + 0xa0], xmm0
0x14050f4d4 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
0x14050f4db c785c800000000000000 mov dword ptr [rbp + 0xc8], 0
0x14050f4e5 48c745a000000000 mov qword ptr [rbp - 0x60], 0
0x14050f4ed 48c745a808000000 mov qword ptr [rbp - 0x58], 8
0x14050f4f5 48c745b000000000 mov qword ptr [rbp - 0x50], 0
0x14050f4fd 48c745b810000000 mov qword ptr [rbp - 0x48], 0x10
0x14050f505 48c745d010000000 mov qword ptr [rbp - 0x30], 0x10
0x14050f50d 48c745e810000000 mov qword ptr [rbp - 0x18], 0x10
0x14050f515 48c7450010000000 mov qword ptr [rbp], 0x10
0x14050f51d 48c7451810000000 mov qword ptr [rbp + 0x18], 0x10
0x14050f525 48b80000000000000080 movabs rax, 0x8000000000000000
0x14050f52f 48894538 mov qword ptr [rbp + 0x38], rax
0x14050f533 488d0d4b104e02 lea rcx, [rip + 0x24e104b]
0x14050f53a 48894d40 mov qword ptr [rbp + 0x40], rcx
0x14050f53e 48c7454841000000 mov qword ptr [rbp + 0x48], 0x41
0x14050f546 48c7455010000000 mov qword ptr [rbp + 0x50], 0x10
0x14050f54e 48c7456810000000 mov qword ptr [rbp + 0x68], 0x10
0x14050f556 48c7858000000010000000 mov qword ptr [rbp + 0x80], 0x10
0x14050f561 48c7859800000010000000 mov qword ptr [rbp + 0x98], 0x10
0x14050f56c 48c785b000000010000000 mov qword ptr [rbp + 0xb0], 0x10
0x14050f577 c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
0x14050f581 66c785d40000000100 mov word ptr [rbp + 0xd4], 1
0x14050f58a 488985d0010000 mov qword ptr [rbp + 0x1d0], rax
0x14050f591 c7856002000000000000 mov dword ptr [rbp + 0x260], 0
0x14050f59b 0fb60567561203 movzx eax, byte ptr [rip + 0x3125667]
0x14050f5a2 b9c8020000 mov ecx, 0x2c8
0x14050f5a7 ba08000000 mov edx, 8
0x14050f5ac e85f461000 call 0x140613c10
0x14050f5b1 4885c0 test rax, rax
0x14050f5b4 7435 je 0x14050f5eb
0x14050f5b6 4889c7 mov rdi, rax
0x14050f5b9 488d55a0 lea rdx, [rbp - 0x60]
0x14050f5bd 41b8c8020000 mov r8d, 0x2c8
0x14050f5c3 4889c1 mov rcx, rax
0x14050f5c6 e835464102 call 0x142923c00
0x14050f5cb 4c8d05ee0c4e02 lea r8, [rip + 0x24e0cee]
0x14050f5d2 4889f1 mov rcx, rsi
0x14050f5d5 4889fa mov rdx, rdi
0x14050f5d8 e833557d00 call 0x140ce4b10
0x14050f5dd 4889f0 mov rax, rsi
0x14050f5e0 4881c4f0020000 add rsp, 0x2f0
0x14050f5e7 5f pop rdi
0x14050f5e8 5e pop rsi
0x14050f5e9 5d pop rbp
0x14050f5ea c3 ret 
0x14050f5eb b908000000 mov ecx, 8
0x14050f5f0 bac8020000 mov edx, 0x2c8
0x14050f5f5 e8069a3c02 call 0x1428d9000
0x14050f5fa 0f0b ud2 
0x14050f5fc 0f1f4000 nop dword ptr [rax]
0x14050f600 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14050f605 55 push rbp
0x14050f606 56 push rsi
0x14050f607 57 push rdi
0x14050f608 4883ec20 sub rsp, 0x20
0x14050f60c 488daa80000000 lea rbp, [rdx + 0x80]
0x14050f613 488d4da0 lea rcx, [rbp - 0x60]
0x14050f617 e894fd0b00 call 0x1405cf3b0
0x14050f61c 90 nop 
0x14050f61d 4883c420 add rsp, 0x20
0x14050f621 5f pop rdi
0x14050f622 5e pop rsi
0x14050f623 5d pop rbp
0x14050f624 c3 ret 
