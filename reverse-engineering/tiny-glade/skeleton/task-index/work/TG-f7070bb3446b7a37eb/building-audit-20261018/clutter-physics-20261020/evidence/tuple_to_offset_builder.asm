0x140b26a80 4156 push r14
0x140b26a82 56 push rsi
0x140b26a83 57 push rdi
0x140b26a84 53 push rbx
0x140b26a85 4881ece8000000 sub rsp, 0xe8
0x140b26a8c 4c89c7 mov rdi, r8
0x140b26a8f 4889ce mov rsi, rcx
0x140b26a92 8b4208 mov eax, dword ptr [rdx + 8]
0x140b26a95 89442428 mov dword ptr [rsp + 0x28], eax
0x140b26a99 488b02 mov rax, qword ptr [rdx]
0x140b26a9c 4889442420 mov qword ptr [rsp + 0x20], rax
0x140b26aa1 0f284210 movaps xmm0, xmmword ptr [rdx + 0x10]
0x140b26aa5 498d5850 lea rbx, [r8 + 0x50]
0x140b26aa9 0f29442450 movaps xmmword ptr [rsp + 0x50], xmm0
0x140b26aae 4c8d742460 lea r14, [rsp + 0x60]
0x140b26ab3 488d542420 lea rdx, [rsp + 0x20]
0x140b26ab8 4c8d442450 lea r8, [rsp + 0x50]
0x140b26abd 4c89f1 mov rcx, r14
0x140b26ac0 e89bb01600 call 0x140c91b60
0x140b26ac5 488d4c2430 lea rcx, [rsp + 0x30]
0x140b26aca 4889da mov rdx, rbx
0x140b26acd 4d89f0 mov r8, r14
0x140b26ad0 e8abb01600 call 0x140c91b80
0x140b26ad5 488b5720 mov rdx, qword ptr [rdi + 0x20]
0x140b26ad9 4c8b4728 mov r8, qword ptr [rdi + 0x28]
0x140b26add f048ff02 lock inc qword ptr [rdx]
0x140b26ae1 0f8eda000000 jle 0x140b26bc1
0x140b26ae7 488d4c2460 lea rcx, [rsp + 0x60]
0x140b26aec e8afc40600 call 0x140b92fa0
0x140b26af1 c68424d600000001 mov byte ptr [rsp + 0xd6], 1
0x140b26af9 488b8424a0000000 mov rax, qword ptr [rsp + 0xa0]
0x140b26b01 48894660 mov qword ptr [rsi + 0x60], rax
0x140b26b05 0f28442460 movaps xmm0, xmmword ptr [rsp + 0x60]
0x140b26b0a 0f284c2470 movaps xmm1, xmmword ptr [rsp + 0x70]
0x140b26b0f 0f28942480000000 movaps xmm2, xmmword ptr [rsp + 0x80]
0x140b26b17 0f289c2490000000 movaps xmm3, xmmword ptr [rsp + 0x90]
0x140b26b1f 0f295e50 movaps xmmword ptr [rsi + 0x50], xmm3
0x140b26b23 0f295640 movaps xmmword ptr [rsi + 0x40], xmm2
0x140b26b27 0f294e30 movaps xmmword ptr [rsi + 0x30], xmm1
0x140b26b2b 0f294620 movaps xmmword ptr [rsi + 0x20], xmm0
0x140b26b2f 8b8424d0000000 mov eax, dword ptr [rsp + 0xd0]
0x140b26b36 898690000000 mov dword ptr [rsi + 0x90], eax
0x140b26b3c 0fb78424d4000000 movzx eax, word ptr [rsp + 0xd4]
0x140b26b44 66898694000000 mov word ptr [rsi + 0x94], ax
0x140b26b4b 0fb68424d6000000 movzx eax, byte ptr [rsp + 0xd6]
0x140b26b53 888696000000 mov byte ptr [rsi + 0x96], al
0x140b26b59 488b8424d7000000 mov rax, qword ptr [rsp + 0xd7]
0x140b26b61 48898697000000 mov qword ptr [rsi + 0x97], rax
0x140b26b68 0fb68424df000000 movzx eax, byte ptr [rsp + 0xdf]
0x140b26b70 88869f000000 mov byte ptr [rsi + 0x9f], al
0x140b26b76 0f108424c4000000 movups xmm0, xmmword ptr [rsp + 0xc4]
0x140b26b7e 0f118684000000 movups xmmword ptr [rsi + 0x84], xmm0
0x140b26b85 0f28442430 movaps xmm0, xmmword ptr [rsp + 0x30]
0x140b26b8a 0f284c2440 movaps xmm1, xmmword ptr [rsp + 0x40]
0x140b26b8f f30f10542438 movss xmm2, dword ptr [rsp + 0x38]
0x140b26b95 0f134610 movlps qword ptr [rsi + 0x10], xmm0
0x140b26b99 f30f115618 movss dword ptr [rsi + 0x18], xmm2
0x140b26b9e 0f114e68 movups xmmword ptr [rsi + 0x68], xmm1
0x140b26ba2 f30f119680000000 movss dword ptr [rsi + 0x80], xmm2
0x140b26baa 0f134678 movlps qword ptr [rsi + 0x78], xmm0
0x140b26bae 0f290e movaps xmmword ptr [rsi], xmm1
0x140b26bb1 4889f0 mov rax, rsi
0x140b26bb4 4881c4e8000000 add rsp, 0xe8
0x140b26bbb 5b pop rbx
0x140b26bbc 5f pop rdi
0x140b26bbd 5e pop rsi
0x140b26bbe 415e pop r14
0x140b26bc0 c3 ret
0x140b26bc1 0f0b ud2
