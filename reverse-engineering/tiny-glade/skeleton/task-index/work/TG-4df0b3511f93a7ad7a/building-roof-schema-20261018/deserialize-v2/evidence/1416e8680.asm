0x1416e8680 55 push rbp
0x1416e8681 4157 push r15
0x1416e8683 4156 push r14
0x1416e8685 4154 push r12
0x1416e8687 56 push rsi
0x1416e8688 57 push rdi
0x1416e8689 53 push rbx
0x1416e868a 4881ec40010000 sub rsp, 0x140
0x1416e8691 488dac2480000000 lea rbp, [rsp + 0x80]
0x1416e8699 48c785b8000000feffffff mov qword ptr [rbp + 0xb8], 0xfffffffffffffffe
0x1416e86a4 4889ce mov rsi, rcx
0x1416e86a7 0fb602 movzx eax, byte ptr [rdx]
0x1416e86aa 83f803 cmp eax, 3
0x1416e86ad 8985a0000000 mov dword ptr [rbp + 0xa0], eax
0x1416e86b3 48895578 mov qword ptr [rbp + 0x78], rdx
0x1416e86b7 0f84ed000000 je 0x1416e87aa
0x1416e86bd 83f805 cmp eax, 5
0x1416e86c0 0f85cf010000 jne 0x1416e8895
0x1416e86c6 4c8b7a08 mov r15, qword ptr [rdx + 8]
0x1416e86ca 4c8b6210 mov r12, qword ptr [rdx + 0x10]
0x1416e86ce 488b7a18 mov rdi, qword ptr [rdx + 0x18]
0x1416e86d2 31c0 xor eax, eax
0x1416e86d4 4d85ff test r15, r15
0x1416e86d7 0f95c0 setne al
0x1416e86da 4889f9 mov rcx, rdi
0x1416e86dd 490f44cf cmove rcx, r15
0x1416e86e1 48894520 mov qword ptr [rbp + 0x20], rax
0x1416e86e5 48c7452800000000 mov qword ptr [rbp + 0x28], 0
0x1416e86ed 4c897d30 mov qword ptr [rbp + 0x30], r15
0x1416e86f1 4c896538 mov qword ptr [rbp + 0x38], r12
0x1416e86f5 48894540 mov qword ptr [rbp + 0x40], rax
0x1416e86f9 48c7454800000000 mov qword ptr [rbp + 0x48], 0
0x1416e8701 4c897d50 mov qword ptr [rbp + 0x50], r15
0x1416e8705 4c896558 mov qword ptr [rbp + 0x58], r12
0x1416e8709 48894d60 mov qword ptr [rbp + 0x60], rcx
0x1416e870d 488d8d80000000 lea rcx, [rbp + 0x80]
0x1416e8714 488d5520 lea rdx, [rbp + 0x20]
0x1416e8718 e8333dd5ff call 0x14143c450
0x1416e871d 488b8580000000 mov rax, qword ptr [rbp + 0x80]
0x1416e8724 4885c0 test rax, rax
0x1416e8727 7444 je 0x1416e876d
0x1416e8729 488b8d90000000 mov rcx, qword ptr [rbp + 0x90]
0x1416e8730 488d1449 lea rdx, [rcx + rcx*2]
0x1416e8734 4c8b84d068010000 mov r8, qword ptr [rax + rdx*8 + 0x168]
0x1416e873c 4c8b8cd070010000 mov r9, qword ptr [rax + rdx*8 + 0x170]
0x1416e8744 488b9cd078010000 mov rbx, qword ptr [rax + rdx*8 + 0x178]
0x1416e874c 48c1e105 shl rcx, 5
0x1416e8750 0f100408 movups xmm0, xmmword ptr [rax + rcx]
0x1416e8754 0f104c0810 movups xmm1, xmmword ptr [rax + rcx + 0x10]
0x1416e8759 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
0x1416e875d 0f294df0 movaps xmmword ptr [rbp - 0x10], xmm1
0x1416e8761 4c89c0 mov rax, r8
0x1416e8764 48f7d8 neg rax
0x1416e8767 0f81b5010000 jno 0x1416e8922
0x1416e876d c685800000000b mov byte ptr [rbp + 0x80], 0xb
0x1416e8774 488d156d6a5a01 lea rdx, [rip + 0x15a6a6d]
0x1416e877b 4c8d05766a5a01 lea r8, [rip + 0x15a6a76]
0x1416e8782 488d8d80000000 lea rcx, [rbp + 0x80]
0x1416e8789 e8b2132101 call 0x1428f9b40
0x1416e878e 48894608 mov qword ptr [rsi + 8], rax
0x1416e8792 c60601 mov byte ptr [rsi], 1
0x1416e8795 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416e879c 488d4d20 lea rcx, [rbp + 0x20]
0x1416e87a0 e84bf619ff call 0x140887df0
0x1416e87a5 e973020000 jmp 0x1416e8a1d
0x1416e87aa 4c8b7a08 mov r15, qword ptr [rdx + 8]
0x1416e87ae 4c8b6210 mov r12, qword ptr [rdx + 0x10]
0x1416e87b2 488b7a18 mov rdi, qword ptr [rdx + 0x18]
0x1416e87b6 41b606 mov r14b, 6
0x1416e87b9 4c89f8 mov rax, r15
0x1416e87bc 4c89e1 mov rcx, r12
0x1416e87bf 4889fb mov rbx, rdi
0x1416e87c2 48894520 mov qword ptr [rbp + 0x20], rax
0x1416e87c6 48894d28 mov qword ptr [rbp + 0x28], rcx
0x1416e87ca 48895d30 mov qword ptr [rbp + 0x30], rbx
0x1416e87ce 0f2845c0 movaps xmm0, xmmword ptr [rbp - 0x40]
0x1416e87d2 0f114539 movups xmmword ptr [rbp + 0x39], xmm0
0x1416e87d6 0f1045cf movups xmm0, xmmword ptr [rbp - 0x31]
0x1416e87da 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
0x1416e87de 44887538 mov byte ptr [rbp + 0x38], r14b
0x1416e87e2 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416e87e9 488d4de0 lea rcx, [rbp - 0x20]
0x1416e87ed 488d5520 lea rdx, [rbp + 0x20]
0x1416e87f1 e88ab7eeff call 0x1415d3f80
0x1416e87f6 0fb65de0 movzx ebx, byte ptr [rbp - 0x20]
0x1416e87fa 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x1416e87fe 80fb02 cmp bl, 2
0x1416e8801 7439 je 0x1416e883c
0x1416e8803 488b4d00 mov rcx, qword ptr [rbp]
0x1416e8807 48898d98000000 mov qword ptr [rbp + 0x98], rcx
0x1416e880e 0f1045f0 movups xmm0, xmmword ptr [rbp - 0x10]
0x1416e8812 0f118588000000 movups xmmword ptr [rbp + 0x88], xmm0
0x1416e8819 48898580000000 mov qword ptr [rbp + 0x80], rax
0x1416e8820 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416e8827 488d8d80000000 lea rcx, [rbp + 0x80]
0x1416e882e e8cdf749ff call 0x140b88000
0x1416e8833 4885c0 test rax, rax
0x1416e8836 0f849f010000 je 0x1416e89db
0x1416e883c 48894608 mov qword ptr [rsi + 8], rax
0x1416e8840 b001 mov al, 1
0x1416e8842 8806 mov byte ptr [rsi], al
0x1416e8844 8b85a0000000 mov eax, dword ptr [rbp + 0xa0]
0x1416e884a 83f803 cmp eax, 3
0x1416e884d 0f84ca010000 je 0x1416e8a1d
0x1416e8853 83f805 cmp eax, 5
0x1416e8856 0f84c1010000 je 0x1416e8a1d
0x1416e885c 4c89bda0000000 mov qword ptr [rbp + 0xa0], r15
0x1416e8863 4c896578 mov qword ptr [rbp + 0x78], r12
0x1416e8867 4c89e1 mov rcx, r12
0x1416e886a 4889fa mov rdx, rdi
0x1416e886d e8fe041aff call 0x140888d70
0x1416e8872 488b95a0000000 mov rdx, qword ptr [rbp + 0xa0]
0x1416e8879 4885d2 test rdx, rdx
0x1416e887c 488b4d78 mov rcx, qword ptr [rbp + 0x78]
0x1416e8880 0f8497010000 je 0x1416e8a1d
0x1416e8886 48c1e205 shl rdx, 5
0x1416e888a 41b808000000 mov r8d, 8
0x1416e8890 e971010000 jmp 0x1416e8a06
0x1416e8895 0f1002 movups xmm0, xmmword ptr [rdx]
0x1416e8898 0f104a10 movups xmm1, xmmword ptr [rdx + 0x10]
0x1416e889c 0f294d30 movaps xmmword ptr [rbp + 0x30], xmm1
0x1416e88a0 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
0x1416e88a4 488d4de0 lea rcx, [rbp - 0x20]
0x1416e88a8 488d5520 lea rdx, [rbp + 0x20]
0x1416e88ac e85f131e01 call 0x1428c9c10
0x1416e88b1 488d1570695a01 lea rdx, [rip + 0x15a6970]
0x1416e88b8 4c8d0539695a01 lea r8, [rip + 0x15a6939]
0x1416e88bf 488d4de0 lea rcx, [rbp - 0x20]
0x1416e88c3 e838111e01 call 0x1428c9a00
0x1416e88c8 48894608 mov qword ptr [rsi + 8], rax
0x1416e88cc c60601 mov byte ptr [rsi], 1
0x1416e88cf 0fb64520 movzx eax, byte ptr [rbp + 0x20]
0x1416e88d3 83f803 cmp eax, 3
0x1416e88d6 0f8241010000 jb 0x1416e8a1d
0x1416e88dc 0f8411010000 je 0x1416e89f3
0x1416e88e2 83f804 cmp eax, 4
0x1416e88e5 0f8522010000 jne 0x1416e8a0d
0x1416e88eb 488b4d30 mov rcx, qword ptr [rbp + 0x30]
0x1416e88ef 488b5538 mov rdx, qword ptr [rbp + 0x38]
0x1416e88f3 48898da8000000 mov qword ptr [rbp + 0xa8], rcx
0x1416e88fa e871041aff call 0x140888d70
0x1416e88ff 488b5528 mov rdx, qword ptr [rbp + 0x28]
0x1416e8903 4885d2 test rdx, rdx
0x1416e8906 0f8411010000 je 0x1416e8a1d
0x1416e890c 48c1e205 shl rdx, 5
0x1416e8910 41b808000000 mov r8d, 8
0x1416e8916 488b8da8000000 mov rcx, qword ptr [rbp + 0xa8]
0x1416e891d e9e4000000 jmp 0x1416e8a06
0x1416e8922 4c894d70 mov qword ptr [rbp + 0x70], r9
0x1416e8926 4c8985a8000000 mov qword ptr [rbp + 0xa8], r8
0x1416e892d 0f2845e0 movaps xmm0, xmmword ptr [rbp - 0x20]
0x1416e8931 0f284df0 movaps xmm1, xmmword ptr [rbp - 0x10]
0x1416e8935 0f298d90000000 movaps xmmword ptr [rbp + 0x90], xmm1
0x1416e893c 0f298580000000 movaps xmmword ptr [rbp + 0x80], xmm0
0x1416e8943 488d4da8 lea rcx, [rbp - 0x58]
0x1416e8947 488d5520 lea rdx, [rbp + 0x20]
0x1416e894b e8003bd5ff call 0x14143c450
0x1416e8950 488b45a8 mov rax, qword ptr [rbp - 0x58]
0x1416e8954 4885c0 test rax, rax
0x1416e8957 7445 je 0x1416e899e
0x1416e8959 488b4db8 mov rcx, qword ptr [rbp - 0x48]
0x1416e895d 488d1449 lea rdx, [rcx + rcx*2]
0x1416e8961 4c8b84d078010000 mov r8, qword ptr [rax + rdx*8 + 0x178]
0x1416e8969 4c8945f0 mov qword ptr [rbp - 0x10], r8
0x1416e896d 0f1084d068010000 movups xmm0, xmmword ptr [rax + rdx*8 + 0x168]
0x1416e8975 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
0x1416e8979 48c1e105 shl rcx, 5
0x1416e897d 0f100408 movups xmm0, xmmword ptr [rax + rcx]
0x1416e8981 0f104c0810 movups xmm1, xmmword ptr [rax + rcx + 0x10]
0x1416e8986 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x1416e898a 0f114d08 movups xmmword ptr [rbp + 8], xmm1
0x1416e898e 488b55e0 mov rdx, qword ptr [rbp - 0x20]
0x1416e8992 4889d0 mov rax, rdx
0x1416e8995 48f7d8 neg rax
0x1416e8998 0f8194000000 jno 0x1416e8a32
0x1416e899e 448ab580000000 mov r14b, byte ptr [rbp + 0x80]
0x1416e89a5 0f108581000000 movups xmm0, xmmword ptr [rbp + 0x81]
0x1416e89ac 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
0x1416e89b0 0f108590000000 movups xmm0, xmmword ptr [rbp + 0x90]
0x1416e89b7 0f1145cf movups xmmword ptr [rbp - 0x31], xmm0
0x1416e89bb c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416e89c2 488d4d20 lea rcx, [rbp + 0x20]
0x1416e89c6 e825f419ff call 0x140887df0
0x1416e89cb 488b85a8000000 mov rax, qword ptr [rbp + 0xa8]
0x1416e89d2 488b4d70 mov rcx, qword ptr [rbp + 0x70]
0x1416e89d6 e9e7fdffff jmp 0x1416e87c2
0x1416e89db 885e01 mov byte ptr [rsi + 1], bl
0x1416e89de 31c0 xor eax, eax
0x1416e89e0 8806 mov byte ptr [rsi], al
0x1416e89e2 8b85a0000000 mov eax, dword ptr [rbp + 0xa0]
0x1416e89e8 83f803 cmp eax, 3
0x1416e89eb 0f8562feffff jne 0x1416e8853
0x1416e89f1 eb2a jmp 0x1416e8a1d
0x1416e89f3 488b5528 mov rdx, qword ptr [rbp + 0x28]
0x1416e89f7 4885d2 test rdx, rdx
0x1416e89fa 7421 je 0x1416e8a1d
0x1416e89fc 488b4d30 mov rcx, qword ptr [rbp + 0x30]
0x1416e8a00 41b801000000 mov r8d, 1
0x1416e8a06 e815b2f2fe call 0x140613c20
0x1416e8a0b eb10 jmp 0x1416e8a1d
0x1416e8a0d 488d4d28 lea rcx, [rbp + 0x28]
0x1416e8a11 c685b700000000 mov byte ptr [rbp + 0xb7], 0
0x1416e8a18 e823051bff call 0x140898f40
0x1416e8a1d 4889f0 mov rax, rsi
0x1416e8a20 4881c440010000 add rsp, 0x140
0x1416e8a27 5b pop rbx
0x1416e8a28 5f pop rdi
0x1416e8a29 5e pop rsi
0x1416e8a2a 415c pop r12
0x1416e8a2c 415e pop r14
0x1416e8a2e 415f pop r15
0x1416e8a30 5d pop rbp
0x1416e8a31 c3 ret 
0x1416e8a32 730f jae 0x1416e8a43
0x1416e8a34 488b4de8 mov rcx, qword ptr [rbp - 0x18]
0x1416e8a38 41b801000000 mov r8d, 1
0x1416e8a3e e8ddb1f2fe call 0x140613c20
0x1416e8a43 0fb645f8 movzx eax, byte ptr [rbp - 8]
0x1416e8a47 83f803 cmp eax, 3
0x1416e8a4a 7231 jb 0x1416e8a7d
0x1416e8a4c 7410 je 0x1416e8a5e
0x1416e8a4e 488d4d00 lea rcx, [rbp]
0x1416e8a52 83f804 cmp eax, 4
0x1416e8a55 7521 jne 0x1416e8a78
0x1416e8a57 e834091aff call 0x140889390
0x1416e8a5c eb1f jmp 0x1416e8a7d
0x1416e8a5e 488b5500 mov rdx, qword ptr [rbp]
0x1416e8a62 4885d2 test rdx, rdx
0x1416e8a65 7416 je 0x1416e8a7d
0x1416e8a67 488b4d08 mov rcx, qword ptr [rbp + 8]
0x1416e8a6b 41b801000000 mov r8d, 1
0x1416e8a71 e8aab1f2fe call 0x140613c20
0x1416e8a76 eb05 jmp 0x1416e8a7d
0x1416e8a78 e8c3041bff call 0x140898f40
0x1416e8a7d c645e00b mov byte ptr [rbp - 0x20], 0xb
0x1416e8a81 488d1560675a01 lea rdx, [rip + 0x15a6760]
0x1416e8a88 4c8d0569675a01 lea r8, [rip + 0x15a6769]
0x1416e8a8f 488d4de0 lea rcx, [rbp - 0x20]
0x1416e8a93 e8a8102101 call 0x1428f9b40
0x1416e8a98 48894608 mov qword ptr [rsi + 8], rax
0x1416e8a9c c60601 mov byte ptr [rsi], 1
0x1416e8a9f 0fb68580000000 movzx eax, byte ptr [rbp + 0x80]
0x1416e8aa6 83f803 cmp eax, 3
0x1416e8aa9 723a jb 0x1416e8ae5
0x1416e8aab 7413 je 0x1416e8ac0
0x1416e8aad 488d8d88000000 lea rcx, [rbp + 0x88]
0x1416e8ab4 83f804 cmp eax, 4
0x1416e8ab7 7527 jne 0x1416e8ae0
0x1416e8ab9 e8d2081aff call 0x140889390
0x1416e8abe eb25 jmp 0x1416e8ae5
0x1416e8ac0 488b9588000000 mov rdx, qword ptr [rbp + 0x88]
0x1416e8ac7 4885d2 test rdx, rdx
0x1416e8aca 7419 je 0x1416e8ae5
0x1416e8acc 488b8d90000000 mov rcx, qword ptr [rbp + 0x90]
0x1416e8ad3 41b801000000 mov r8d, 1
0x1416e8ad9 e842b1f2fe call 0x140613c20
0x1416e8ade eb05 jmp 0x1416e8ae5
0x1416e8ae0 e85b041bff call 0x140898f40
0x1416e8ae5 488b95a8000000 mov rdx, qword ptr [rbp + 0xa8]
0x1416e8aec 4885d2 test rdx, rdx
0x1416e8aef 0f84a0fcffff je 0x1416e8795
0x1416e8af5 41b801000000 mov r8d, 1
0x1416e8afb 488b4d70 mov rcx, qword ptr [rbp + 0x70]
0x1416e8aff e81cb1f2fe call 0x140613c20
0x1416e8b04 e98cfcffff jmp 0x1416e8795
0x1416e8b09 0f1f8000000000 nop dword ptr [rax]
0x1416e8b10 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e8b15 55 push rbp
0x1416e8b16 4157 push r15
0x1416e8b18 4156 push r14
0x1416e8b1a 4154 push r12
0x1416e8b1c 56 push rsi
0x1416e8b1d 57 push rdi
0x1416e8b1e 53 push rbx
0x1416e8b1f 4883ec20 sub rsp, 0x20
0x1416e8b23 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e8b2a 488d4d20 lea rcx, [rbp + 0x20]
0x1416e8b2e e8bdf219ff call 0x140887df0
0x1416e8b33 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416e8b3a 4883c420 add rsp, 0x20
0x1416e8b3e 5b pop rbx
0x1416e8b3f 5f pop rdi
0x1416e8b40 5e pop rsi
0x1416e8b41 415c pop r12
0x1416e8b43 415e pop r14
0x1416e8b45 415f pop r15
0x1416e8b47 5d pop rbp
0x1416e8b48 c3 ret 
0x1416e8b49 0f1f8000000000 nop dword ptr [rax]
0x1416e8b50 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e8b55 55 push rbp
0x1416e8b56 4157 push r15
0x1416e8b58 4156 push r14
0x1416e8b5a 4154 push r12
0x1416e8b5c 56 push rsi
0x1416e8b5d 57 push rdi
0x1416e8b5e 53 push rbx
0x1416e8b5f 4883ec20 sub rsp, 0x20
0x1416e8b63 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e8b6a 4883bda800000000 cmp qword ptr [rbp + 0xa8], 0
0x1416e8b72 7416 je 0x1416e8b8a
0x1416e8b74 41b801000000 mov r8d, 1
0x1416e8b7a 488b4d70 mov rcx, qword ptr [rbp + 0x70]
0x1416e8b7e 488b95a8000000 mov rdx, qword ptr [rbp + 0xa8]
0x1416e8b85 e896b0f2fe call 0x140613c20
0x1416e8b8a 90 nop 
0x1416e8b8b 4883c420 add rsp, 0x20
0x1416e8b8f 5b pop rbx
0x1416e8b90 5f pop rdi
0x1416e8b91 5e pop rsi
0x1416e8b92 415c pop r12
0x1416e8b94 415e pop r14
0x1416e8b96 415f pop r15
0x1416e8b98 5d pop rbp
0x1416e8b99 c3 ret 
0x1416e8b9a 660f1f440000 nop word ptr [rax + rax]
0x1416e8ba0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e8ba5 55 push rbp
0x1416e8ba6 4157 push r15
0x1416e8ba8 4156 push r14
0x1416e8baa 4154 push r12
0x1416e8bac 56 push rsi
0x1416e8bad 57 push rdi
0x1416e8bae 53 push rbx
0x1416e8baf 4883ec20 sub rsp, 0x20
0x1416e8bb3 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e8bba 4883bda000000000 cmp qword ptr [rbp + 0xa0], 0
0x1416e8bc2 741a je 0x1416e8bde
0x1416e8bc4 488b95a0000000 mov rdx, qword ptr [rbp + 0xa0]
0x1416e8bcb 48c1e205 shl rdx, 5
0x1416e8bcf 41b808000000 mov r8d, 8
0x1416e8bd5 488b4d78 mov rcx, qword ptr [rbp + 0x78]
0x1416e8bd9 e842b0f2fe call 0x140613c20
0x1416e8bde 90 nop 
0x1416e8bdf 4883c420 add rsp, 0x20
0x1416e8be3 5b pop rbx
0x1416e8be4 5f pop rdi
0x1416e8be5 5e pop rsi
0x1416e8be6 415c pop r12
0x1416e8be8 415e pop r14
0x1416e8bea 415f pop r15
0x1416e8bec 5d pop rbp
0x1416e8bed c3 ret 
0x1416e8bee 6690 nop 
0x1416e8bf0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e8bf5 55 push rbp
0x1416e8bf6 4157 push r15
0x1416e8bf8 4156 push r14
0x1416e8bfa 4154 push r12
0x1416e8bfc 56 push rsi
0x1416e8bfd 57 push rdi
0x1416e8bfe 53 push rbx
0x1416e8bff 4883ec20 sub rsp, 0x20
0x1416e8c03 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e8c0a 488d4d20 lea rcx, [rbp + 0x20]
0x1416e8c0e e87d4e17ff call 0x14085da90
0x1416e8c13 c685b700000000 mov byte ptr [rbp + 0xb7], 0
0x1416e8c1a 4883c420 add rsp, 0x20
0x1416e8c1e 5b pop rbx
0x1416e8c1f 5f pop rdi
0x1416e8c20 5e pop rsi
0x1416e8c21 415c pop r12
0x1416e8c23 415e pop r14
0x1416e8c25 415f pop r15
0x1416e8c27 5d pop rbp
0x1416e8c28 c3 ret 
0x1416e8c29 0f1f8000000000 nop dword ptr [rax]
0x1416e8c30 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e8c35 55 push rbp
0x1416e8c36 4157 push r15
0x1416e8c38 4156 push r14
0x1416e8c3a 4154 push r12
0x1416e8c3c 56 push rsi
0x1416e8c3d 57 push rdi
0x1416e8c3e 53 push rbx
0x1416e8c3f 4883ec20 sub rsp, 0x20
0x1416e8c43 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e8c4a 488b5528 mov rdx, qword ptr [rbp + 0x28]
0x1416e8c4e 4885d2 test rdx, rdx
0x1416e8c51 7416 je 0x1416e8c69
0x1416e8c53 48c1e205 shl rdx, 5
0x1416e8c57 41b808000000 mov r8d, 8
0x1416e8c5d 488b8da8000000 mov rcx, qword ptr [rbp + 0xa8]
0x1416e8c64 e8b7aff2fe call 0x140613c20
0x1416e8c69 c685b700000000 mov byte ptr [rbp + 0xb7], 0
0x1416e8c70 4883c420 add rsp, 0x20
0x1416e8c74 5b pop rbx
0x1416e8c75 5f pop rdi
0x1416e8c76 5e pop rsi
0x1416e8c77 415c pop r12
0x1416e8c79 415e pop r14
0x1416e8c7b 415f pop r15
0x1416e8c7d 5d pop rbp
0x1416e8c7e c3 ret 
0x1416e8c7f 90 nop 
0x1416e8c80 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e8c85 55 push rbp
0x1416e8c86 4157 push r15
0x1416e8c88 4156 push r14
0x1416e8c8a 4154 push r12
0x1416e8c8c 56 push rsi
0x1416e8c8d 57 push rdi
0x1416e8c8e 53 push rbx
0x1416e8c8f 4883ec20 sub rsp, 0x20
0x1416e8c93 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e8c9a 80bdb700000000 cmp byte ptr [rbp + 0xb7], 0
0x1416e8ca1 7415 je 0x1416e8cb8
0x1416e8ca3 80bda000000003 cmp byte ptr [rbp + 0xa0], 3
0x1416e8caa 740c je 0x1416e8cb8
0x1416e8cac 0fb685a0000000 movzx eax, byte ptr [rbp + 0xa0]
0x1416e8cb3 83f805 cmp eax, 5
0x1416e8cb6 750f jne 0x1416e8cc7
0x1416e8cb8 4883c420 add rsp, 0x20
0x1416e8cbc 5b pop rbx
0x1416e8cbd 5f pop rdi
0x1416e8cbe 5e pop rsi
0x1416e8cbf 415c pop r12
0x1416e8cc1 415e pop r14
0x1416e8cc3 415f pop r15
0x1416e8cc5 5d pop rbp
0x1416e8cc6 c3 ret 
0x1416e8cc7 488b4d78 mov rcx, qword ptr [rbp + 0x78]
0x1416e8ccb e8c04d17ff call 0x14085da90
0x1416e8cd0 ebe6 jmp 0x1416e8cb8
0x1416e8cd2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1416e8ce0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e8ce5 55 push rbp
0x1416e8ce6 4157 push r15
0x1416e8ce8 4156 push r14
0x1416e8cea 4154 push r12
0x1416e8cec 56 push rsi
0x1416e8ced 57 push rdi
0x1416e8cee 53 push rbx
0x1416e8cef 4883ec20 sub rsp, 0x20
0x1416e8cf3 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e8cfa 488d8d80000000 lea rcx, [rbp + 0x80]
0x1416e8d01 e88a4d17ff call 0x14085da90
0x1416e8d06 90 nop 
0x1416e8d07 4883c420 add rsp, 0x20
0x1416e8d0b 5b pop rbx
0x1416e8d0c 5f pop rdi
0x1416e8d0d 5e pop rsi
0x1416e8d0e 415c pop r12
0x1416e8d10 415e pop r14
0x1416e8d12 415f pop r15
0x1416e8d14 5d pop rbp
0x1416e8d15 c3 ret 
