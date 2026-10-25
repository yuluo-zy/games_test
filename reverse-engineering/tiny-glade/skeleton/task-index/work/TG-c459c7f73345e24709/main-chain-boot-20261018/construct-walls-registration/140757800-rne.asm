0x140757800 55 push rbp
0x140757801 4157 push r15
0x140757803 4156 push r14
0x140757805 4155 push r13
0x140757807 4154 push r12
0x140757809 56 push rsi
0x14075780a 57 push rdi
0x14075780b 53 push rbx
0x14075780c 4881ec68050000 sub rsp, 0x568
0x140757813 488dac2480000000 lea rbp, [rsp + 0x80]
0x14075781b 440f29bdd0040000 movaps xmmword ptr [rbp + 0x4d0], xmm15
0x140757823 440f29b5c0040000 movaps xmmword ptr [rbp + 0x4c0], xmm14
0x14075782b 66440f7fadb0040000 movdqa xmmword ptr [rbp + 0x4b0], xmm13
0x140757834 66440f7fa5a0040000 movdqa xmmword ptr [rbp + 0x4a0], xmm12
0x14075783d 66440f7f9d90040000 movdqa xmmword ptr [rbp + 0x490], xmm11
0x140757846 440f299580040000 movaps xmmword ptr [rbp + 0x480], xmm10
0x14075784e 440f298d70040000 movaps xmmword ptr [rbp + 0x470], xmm9
0x140757856 440f298560040000 movaps xmmword ptr [rbp + 0x460], xmm8
0x14075785e 0f29bd50040000 movaps xmmword ptr [rbp + 0x450], xmm7
0x140757865 660f7fb540040000 movdqa xmmword ptr [rbp + 0x440], xmm6
0x14075786d 48c78538040000feffffff mov qword ptr [rbp + 0x438], 0xfffffffffffffffe
0x140757878 4d89ce mov r14, r9
0x14075787b 4c898518040000 mov qword ptr [rbp + 0x418], r8
0x140757882 488995c8020000 mov qword ptr [rbp + 0x2c8], rdx
0x140757889 488b8590050000 mov rax, qword ptr [rbp + 0x590]
0x140757890 488b11 mov rdx, qword ptr [rcx]
0x140757893 488995f8030000 mov qword ptr [rbp + 0x3f8], rdx
0x14075789a 488b5120 mov rdx, qword ptr [rcx + 0x20]
0x14075789e 488995a8020000 mov qword ptr [rbp + 0x2a8], rdx
0x1407578a5 488b7940 mov rdi, qword ptr [rcx + 0x40]
0x1407578a9 488b5948 mov rbx, qword ptr [rcx + 0x48]
0x1407578ad 488b4958 mov rcx, qword ptr [rcx + 0x58]
0x1407578b1 48898d20020000 mov qword ptr [rbp + 0x220], rcx
0x1407578b8 488b08 mov rcx, qword ptr [rax]
0x1407578bb 48898df0000000 mov qword ptr [rbp + 0xf0], rcx
0x1407578c2 f30f6f4020 movdqu xmm0, xmmword ptr [rax + 0x20]
0x1407578c7 f30f6f4830 movdqu xmm1, xmmword ptr [rax + 0x30]
0x1407578cc f30f6f5040 movdqu xmm2, xmmword ptr [rax + 0x40]
0x1407578d1 f30f6f5850 movdqu xmm3, xmmword ptr [rax + 0x50]
0x1407578d6 660f7f5d60 movdqa xmmword ptr [rbp + 0x60], xmm3
0x1407578db 660f7f5550 movdqa xmmword ptr [rbp + 0x50], xmm2
0x1407578e0 660f7f4d40 movdqa xmmword ptr [rbp + 0x40], xmm1
0x1407578e5 660f7f4530 movdqa xmmword ptr [rbp + 0x30], xmm0
0x1407578ea e811f14b00 call 0x140c16a00
0x1407578ef 888531040000 mov byte ptr [rbp + 0x431], al
0x1407578f5 84c0 test al, al
0x1407578f7 0f840f020000 je 0x140757b0c
0x1407578fd 41bf39000000 mov r15d, 0x39
0x140757903 b939000000 mov ecx, 0x39
0x140757908 e803faffff call 0x140757310
0x14075790d 4c8d25e7503202 lea r12, [rip + 0x23250e7]
0x140757914 4883f801 cmp rax, 1
0x140757918 0f85a1000000 jne 0x1407579bf
0x14075791e 4989d1 mov r9, rdx
0x140757921 4885d2 test rdx, rdx
0x140757924 7436 je 0x14075795c
0x140757926 4983f939 cmp r9, 0x39
0x14075792a 732e jae 0x14075795a
0x14075792c 488d05c8503202 lea rax, [rip + 0x23250c8]
0x140757933 41803c01bf cmp byte ptr [r9 + rax], 0xbf
0x140757938 7f22 jg 0x14075795c
0x14075793a 488d056f513202 lea rax, [rip + 0x232516f]
0x140757941 4889442420 mov qword ptr [rsp + 0x20], rax
0x140757946 488d0dae503202 lea rcx, [rip + 0x23250ae]
0x14075794d ba39000000 mov edx, 0x39
0x140757952 4531c0 xor r8d, r8d
0x140757955 e8e61f1802 call 0x1428d9940
0x14075795a 75de jne 0x14075793a
0x14075795c 4c89c9 mov rcx, r9
0x14075795f e8acf9ffff call 0x140757310
0x140757964 4883f801 cmp rax, 1
0x140757968 7555 jne 0x1407579bf
0x14075796a 4989d0 mov r8, rdx
0x14075796d 4983c002 add r8, 2
0x140757971 7439 je 0x1407579ac
0x140757973 4983f839 cmp r8, 0x39
0x140757977 7331 jae 0x1407579aa
0x140757979 488d057b503202 lea rax, [rip + 0x232507b]
0x140757980 41803c00bf cmp byte ptr [r8 + rax], 0xbf
0x140757985 7f25 jg 0x1407579ac
0x140757987 488d053a513202 lea rax, [rip + 0x232513a]
0x14075798e 4889442420 mov qword ptr [rsp + 0x20], rax
0x140757993 488d0d61503202 lea rcx, [rip + 0x2325061]
0x14075799a ba39000000 mov edx, 0x39
0x14075799f 41b939000000 mov r9d, 0x39
0x1407579a5 e8961f1802 call 0x1428d9940
0x1407579aa 75db jne 0x140757987
0x1407579ac 41bf37000000 mov r15d, 0x37
0x1407579b2 4929d7 sub r15, rdx
0x1407579b5 4c8d253f503202 lea r12, [rip + 0x232503f]
0x1407579bc 4d01c4 add r12, r8
0x1407579bf 4c8d0530523202 lea r8, [rip + 0x2325230]
0x1407579c6 4c8d2df3513202 lea r13, [rip + 0x23251f3]
0x1407579cd 0f1f00 nop dword ptr [rax]
0x1407579d0 410fbe40ff movsx eax, byte ptr [r8 - 1]
0x1407579d5 85c0 test eax, eax
0x1407579d7 7817 js 0x1407579f0
0x1407579d9 49ffc8 dec r8
0x1407579dc 83f85c cmp eax, 0x5c
0x1407579df 7563 jne 0x140757a44
0x1407579e1 eb72 jmp 0x140757a55
0x1407579e3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1407579f0 410fb648fe movzx ecx, byte ptr [r8 - 2]
0x1407579f5 80f9c0 cmp cl, 0xc0
0x1407579f8 7d23 jge 0x140757a1d
0x1407579fa 410fb650fd movzx edx, byte ptr [r8 - 3]
0x1407579ff 80fac0 cmp dl, 0xc0
0x140757a02 7d22 jge 0x140757a26
0x140757a04 450fb648fc movzx r9d, byte ptr [r8 - 4]
0x140757a09 4983c0fc add r8, -4
0x140757a0d 4183e107 and r9d, 7
0x140757a11 41c1e106 shl r9d, 6
0x140757a15 83e23f and edx, 0x3f
0x140757a18 4409ca or edx, r9d
0x140757a1b eb10 jmp 0x140757a2d
0x140757a1d 4983c0fe add r8, -2
0x140757a21 83e11f and ecx, 0x1f
0x140757a24 eb0f jmp 0x140757a35
0x140757a26 4983c0fd add r8, -3
0x140757a2a 83e20f and edx, 0xf
0x140757a2d c1e206 shl edx, 6
0x140757a30 83e13f and ecx, 0x3f
0x140757a33 09d1 or ecx, edx
0x140757a35 c1e106 shl ecx, 6
0x140757a38 243f and al, 0x3f
0x140757a3a 0fb6c0 movzx eax, al
0x140757a3d 09c8 or eax, ecx
0x140757a3f 83f85c cmp eax, 0x5c
0x140757a42 7411 je 0x140757a55
0x140757a44 83f82f cmp eax, 0x2f
0x140757a47 740c je 0x140757a55
0x140757a49 4d39e8 cmp r8, r13
0x140757a4c 7582 jne 0x1407579d0
0x140757a4e be36000000 mov esi, 0x36
0x140757a53 eb4c jmp 0x140757aa1
0x140757a55 4c8d2d64513202 lea r13, [rip + 0x2325164]
0x140757a5c 4d29e8 sub r8, r13
0x140757a5f 49ffc0 inc r8
0x140757a62 7432 je 0x140757a96
0x140757a64 4983f836 cmp r8, 0x36
0x140757a68 732a jae 0x140757a94
0x140757a6a 43803c28bf cmp byte ptr [r8 + r13], 0xbf
0x140757a6f 7f25 jg 0x140757a96
0x140757a71 488d0518503202 lea rax, [rip + 0x2325018]
0x140757a78 4889442420 mov qword ptr [rsp + 0x20], rax
0x140757a7d 488d0d3c513202 lea rcx, [rip + 0x232513c]
0x140757a84 ba36000000 mov edx, 0x36
0x140757a89 41b936000000 mov r9d, 0x36
0x140757a8f e8ac1e1802 call 0x1428d9940
0x140757a94 75db jne 0x140757a71
0x140757a96 be36000000 mov esi, 0x36
0x140757a9b 4c29c6 sub rsi, r8
0x140757a9e 4d01c5 add r13, r8
0x140757aa1 e88a064c00 call 0x140c18130
0x140757aa6 4889c1 mov rcx, rax
0x140757aa9 488b00 mov rax, qword ptr [rax]
0x140757aac 4883f801 cmp rax, 1
0x140757ab0 0f8504220000 jne 0x140759cba
0x140757ab6 4883c108 add rcx, 8
0x140757aba 48833900 cmp qword ptr [rcx], 0
0x140757abe 0f85f8220000 jne 0x140759dbc
0x140757ac4 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x140757acb 48898d20040000 mov qword ptr [rbp + 0x420], rcx
0x140757ad2 4883c108 add rcx, 8
0x140757ad6 4889742420 mov qword ptr [rsp + 0x20], rsi
0x140757adb 48c744243000000000 mov qword ptr [rsp + 0x30], 0
0x140757ae4 48c744242801000000 mov qword ptr [rsp + 0x28], 1
0x140757aed 4c89e2 mov rdx, r12
0x140757af0 4d89f8 mov r8, r15
0x140757af3 4d89e9 mov r9, r13
0x140757af6 e845f24b00 call 0x140c16d40
0x140757afb 488985e8020000 mov qword ptr [rbp + 0x2e8], rax
0x140757b02 488b8520040000 mov rax, qword ptr [rbp + 0x420]
0x140757b09 48ff00 inc qword ptr [rax]
0x140757b0c 4c89b5e8030000 mov qword ptr [rbp + 0x3e8], r14
0x140757b13 48899df8000000 mov qword ptr [rbp + 0xf8], rbx
0x140757b1a 4889bd00010000 mov qword ptr [rbp + 0x100], rdi
0x140757b21 488b8d58050000 mov rcx, qword ptr [rbp + 0x558]
0x140757b28 488b01 mov rax, qword ptr [rcx]
0x140757b2b 488b5108 mov rdx, qword ptr [rcx + 8]
0x140757b2f 4c8b10 mov r10, qword ptr [rax]
0x140757b32 4531c9 xor r9d, r9d
0x140757b35 4d89d3 mov r11, r10
0x140757b38 4c2b5a18 sub r11, qword ptr [rdx + 0x18]
0x140757b3c 4d0f42d9 cmovb r11, r9
0x140757b40 4c2b5238 sub r10, qword ptr [rdx + 0x38]
0x140757b44 4d0f42d1 cmovb r10, r9
0x140757b48 4c89d9 mov rcx, r11
0x140757b4b 48c1e104 shl rcx, 4
0x140757b4f 48034a08 add rcx, qword ptr [rdx + 8]
0x140757b53 4c8b4210 mov r8, qword ptr [rdx + 0x10]
0x140757b57 4d29d8 sub r8, r11
0x140757b5a 4d0f42c1 cmovb r8, r9
0x140757b5e bf08000000 mov edi, 8
0x140757b63 480f42cf cmovb rcx, rdi
0x140757b67 4d89d3 mov r11, r10
0x140757b6a 49c1e304 shl r11, 4
0x140757b6e 4c035a28 add r11, qword ptr [rdx + 0x28]
0x140757b72 488b7230 mov rsi, qword ptr [rdx + 0x30]
0x140757b76 4c29d6 sub rsi, r10
0x140757b79 490f42f1 cmovb rsi, r9
0x140757b7d 4c0f42df cmovb r11, rdi
0x140757b81 4e8d0c06 lea r9, [rsi + r8]
0x140757b85 488b5240 mov rdx, qword ptr [rdx + 0x40]
0x140757b89 4c29ca sub rdx, r9
0x140757b8c 488910 mov qword ptr [rax], rdx
0x140757b8f 49c1e004 shl r8, 4
0x140757b93 4901c8 add r8, rcx
0x140757b96 48c1e604 shl rsi, 4
0x140757b9a 4c01de add rsi, r11
0x140757b9d 488985f0020000 mov qword ptr [rbp + 0x2f0], rax
0x140757ba4 48898df8020000 mov qword ptr [rbp + 0x2f8], rcx
0x140757bab 4c898500030000 mov qword ptr [rbp + 0x300], r8
0x140757bb2 4c899d08030000 mov qword ptr [rbp + 0x308], r11
0x140757bb9 4889b510030000 mov qword ptr [rbp + 0x310], rsi
0x140757bc0 4c898d18030000 mov qword ptr [rbp + 0x318], r9
0x140757bc7 4c8d05ca4d3202 lea r8, [rip + 0x2324dca]
0x140757bce 488d8db0030000 lea rcx, [rbp + 0x3b0]
0x140757bd5 488d95f0020000 lea rdx, [rbp + 0x2f0]
0x140757bdc e8efbaaa00 call 0x1412036d0
0x140757be1 488b8560050000 mov rax, qword ptr [rbp + 0x560]
0x140757be8 488b18 mov rbx, qword ptr [rax]
0x140757beb 488b4008 mov rax, qword ptr [rax + 8]
0x140757bef 488b13 mov rdx, qword ptr [rbx]
0x140757bf2 31c9 xor ecx, ecx
0x140757bf4 4989d0 mov r8, rdx
0x140757bf7 4c2b4018 sub r8, qword ptr [rax + 0x18]
0x140757bfb 4c0f42c1 cmovb r8, rcx
0x140757bff 482b5038 sub rdx, qword ptr [rax + 0x38]
0x140757c03 480f42d1 cmovb rdx, rcx
0x140757c07 4d89c7 mov r15, r8
0x140757c0a 49c1e704 shl r15, 4
0x140757c0e 4c037808 add r15, qword ptr [rax + 8]
0x140757c12 4c8b4810 mov r9, qword ptr [rax + 0x10]
0x140757c16 4d29c1 sub r9, r8
0x140757c19 4c0f42c9 cmovb r9, rcx
0x140757c1d 4c0f42ff cmovb r15, rdi
0x140757c21 4989d6 mov r14, rdx
0x140757c24 49c1e604 shl r14, 4
0x140757c28 4c037028 add r14, qword ptr [rax + 0x28]
0x140757c2c 4c8b6030 mov r12, qword ptr [rax + 0x30]
0x140757c30 4929d4 sub r12, rdx
0x140757c33 4c0f42e1 cmovb r12, rcx
0x140757c37 4c0f42f7 cmovb r14, rdi
0x140757c3b 488b4040 mov rax, qword ptr [rax + 0x40]
0x140757c3f 4c29e0 sub rax, r12
0x140757c42 4c29c8 sub rax, r9
0x140757c45 488903 mov qword ptr [rbx], rax
0x140757c48 49c1e104 shl r9, 4
0x140757c4c 4d01f9 add r9, r15
0x140757c4f 4c898d20040000 mov qword ptr [rbp + 0x420], r9
0x140757c56 49c1e404 shl r12, 4
0x140757c5a 4d01f4 add r12, r14
0x140757c5d eb16 jmp 0x140757c75
0x140757c5f 90 nop 
0x140757c60 488b85b8030000 mov rax, qword ptr [rbp + 0x3b8]
0x140757c67 4c892cf8 mov qword ptr [rax + rdi*8], r13
0x140757c6b 48ffc7 inc rdi
0x140757c6e 4889bdc0030000 mov qword ptr [rbp + 0x3c0], rdi
0x140757c75 4c89f1 mov rcx, r14
0x140757c78 4c89fa mov rdx, r15
0x140757c7b 4885d2 test rdx, rdx
0x140757c7e 7520 jne 0x140757ca0
0x140757c80 4885c9 test rcx, rcx
0x140757c83 0f84ab000000 je 0x140757d34
0x140757c89 4c39e1 cmp rcx, r12
0x140757c8c 0f84a2000000 je 0x140757d34
0x140757c92 4c8d7110 lea r14, [rcx + 0x10]
0x140757c96 4c89f7 mov rdi, r14
0x140757c99 31f6 xor esi, esi
0x140757c9b 4889ca mov rdx, rcx
0x140757c9e eb13 jmp 0x140757cb3
0x140757ca0 483b9520040000 cmp rdx, qword ptr [rbp + 0x420]
0x140757ca7 743b je 0x140757ce4
0x140757ca9 4c8d7a10 lea r15, [rdx + 0x10]
0x140757cad 4889cf mov rdi, rcx
0x140757cb0 4c89fe mov rsi, r15
0x140757cb3 48ff03 inc qword ptr [rbx]
0x140757cb6 4c8b6a08 mov r13, qword ptr [rdx + 8]
0x140757cba c6853304000001 mov byte ptr [rbp + 0x433], 1
0x140757cc1 488b8df8030000 mov rcx, qword ptr [rbp + 0x3f8]
0x140757cc8 4c89ea mov rdx, r13
0x140757ccb 4c8d0576474c02 lea r8, [rip + 0x24c4776]
0x140757cd2 e819a23300 call 0x140a91ef0
0x140757cd7 4889f9 mov rcx, rdi
0x140757cda 4889f2 mov rdx, rsi
0x140757cdd 4885c0 test rax, rax
0x140757ce0 750e jne 0x140757cf0
0x140757ce2 eb97 jmp 0x140757c7b
0x140757ce4 4531ff xor r15d, r15d
0x140757ce7 4885c9 test rcx, rcx
0x140757cea 759d jne 0x140757c89
0x140757cec eb46 jmp 0x140757d34
0x140757cee 6690 nop 
0x140757cf0 488bbdc0030000 mov rdi, qword ptr [rbp + 0x3c0]
0x140757cf7 483bbdb0030000 cmp rdi, qword ptr [rbp + 0x3b0]
0x140757cfe 0f855cffffff jne 0x140757c60
0x140757d04 c6853304000001 mov byte ptr [rbp + 0x433], 1
0x140757d0b 48c744242008000000 mov qword ptr [rsp + 0x20], 8
0x140757d14 41b801000000 mov r8d, 1
0x140757d1a 41b908000000 mov r9d, 8
0x140757d20 488d8db0030000 lea rcx, [rbp + 0x3b0]
0x140757d27 4889fa mov rdx, rdi
0x140757d2a e8b1001502 call 0x1428a7de0
0x140757d2f e92cffffff jmp 0x140757c60
0x140757d34 488b9518040000 mov rdx, qword ptr [rbp + 0x418]
0x140757d3b 8b421c mov eax, dword ptr [rdx + 0x1c]
0x140757d3e 488b4a10 mov rcx, qword ptr [rdx + 0x10]
0x140757d42 898594030000 mov dword ptr [rbp + 0x394], eax
0x140757d48 48898d10020000 mov qword ptr [rbp + 0x210], rcx
0x140757d4f 8901 mov dword ptr [rcx], eax
0x140757d51 488b02 mov rax, qword ptr [rdx]
0x140757d54 488b38 mov rdi, qword ptr [rax]
0x140757d57 48898510040000 mov qword ptr [rbp + 0x410], rax
0x140757d5e 488b5818 mov rbx, qword ptr [rax + 0x18]
0x140757d62 660f6f07 movdqa xmm0, xmmword ptr [rdi]
0x140757d66 660fd7c8 pmovmskb ecx, xmm0
0x140757d6a f7d1 not ecx
0x140757d6c 4c8d7f10 lea r15, [rdi + 0x10]
0x140757d70 4c8d35e14e3202 lea r14, [rip + 0x2324ee1]
0x140757d77 eb1e jmp 0x140757d97
0x140757d79 0f1f8000000000 nop dword ptr [rax]
0x140757d80 488b85b8030000 mov rax, qword ptr [rbp + 0x3b8]
0x140757d87 4e892ce0 mov qword ptr [rax + r12*8], r13
0x140757d8b 49ffc4 inc r12
0x140757d8e 4c89a5c0030000 mov qword ptr [rbp + 0x3c0], r12
0x140757d95 89f1 mov ecx, esi
0x140757d97 4885db test rbx, rbx
0x140757d9a 0f84d0000000 je 0x140757e70
0x140757da0 6685c9 test cx, cx
0x140757da3 740b je 0x140757db0
0x140757da5 8d71ff lea esi, [rcx - 1]
0x140757da8 21ce and esi, ecx
0x140757daa eb2d jmp 0x140757dd9
0x140757dac 0f1f4000 nop dword ptr [rax]
0x140757db0 66410f6f07 movdqa xmm0, xmmword ptr [r15]
0x140757db5 660fd7c8 pmovmskb ecx, xmm0
0x140757db9 4881c780f9ffff add rdi, -0x680
0x140757dc0 4983c710 add r15, 0x10
0x140757dc4 81f9ffff0000 cmp ecx, 0xffff
0x140757dca 74e4 je 0x140757db0
0x140757dcc b8feffffff mov eax, 0xfffffffe
0x140757dd1 29c8 sub eax, ecx
0x140757dd3 f7d1 not ecx
0x140757dd5 89ce mov esi, ecx
0x140757dd7 21c6 and esi, eax
0x140757dd9 f30fbcc1 tzcnt eax, ecx
0x140757ddd 48f7d8 neg rax
0x140757de0 4c6be068 imul r12, rax, 0x68
0x140757de4 4e8b6c2798 mov r13, qword ptr [rdi + r12 - 0x68]
0x140757de9 c6853304000001 mov byte ptr [rbp + 0x433], 1
0x140757df0 488b8df8030000 mov rcx, qword ptr [rbp + 0x3f8]
0x140757df7 4c89ea mov rdx, r13
0x140757dfa 4d89f0 mov r8, r14
0x140757dfd e8eea03300 call 0x140a91ef0
0x140757e02 48ffcb dec rbx
0x140757e05 89f1 mov ecx, esi
0x140757e07 4885c0 test rax, rax
0x140757e0a 748b je 0x140757d97
0x140757e0c 4a8d0c27 lea rcx, [rdi + r12]
0x140757e10 4883c1a0 add rcx, -0x60
0x140757e14 660f6e888c020000 movd xmm1, dword ptr [rax + 0x28c]
0x140757e1c 83b85802000002 cmp dword ptr [rax + 0x258], 2
0x140757e23 410f95c0 setne r8b
0x140757e27 c6853304000001 mov byte ptr [rbp + 0x433], 1
0x140757e2e e8cdf48200 call 0x140f87300
0x140757e33 89f1 mov ecx, esi
0x140757e35 84c0 test al, al
0x140757e37 0f845affffff je 0x140757d97
0x140757e3d 4c8ba5c0030000 mov r12, qword ptr [rbp + 0x3c0]
0x140757e44 4c3ba5b0030000 cmp r12, qword ptr [rbp + 0x3b0]
0x140757e4b 0f852fffffff jne 0x140757d80
0x140757e51 c6853304000001 mov byte ptr [rbp + 0x433], 1
0x140757e58 488d8db0030000 lea rcx, [rbp + 0x3b0]
0x140757e5f 488d150a4e3202 lea rdx, [rip + 0x2324e0a]
0x140757e66 e805aae0ff call 0x140562870
0x140757e6b e910ffffff jmp 0x140757d80
0x140757e70 488b85b8030000 mov rax, qword ptr [rbp + 0x3b8]
0x140757e77 48898508040000 mov qword ptr [rbp + 0x408], rax
0x140757e7e 488b95c0030000 mov rdx, qword ptr [rbp + 0x3c0]
0x140757e85 4883fa01 cmp rdx, 1
0x140757e89 4c8ba5f8030000 mov r12, qword ptr [rbp + 0x3f8]
0x140757e90 0f87881e0000 ja 0x140759d1e
0x140757e96 4c8b95e8030000 mov r10, qword ptr [rbp + 0x3e8]
0x140757e9d 488b8d88050000 mov rcx, qword ptr [rbp + 0x588]
0x140757ea4 48c785b001000000000000 mov qword ptr [rbp + 0x1b0], 0
0x140757eaf 48c785b801000004000000 mov qword ptr [rbp + 0x1b8], 4
0x140757eba 48c785c001000000000000 mov qword ptr [rbp + 0x1c0], 0
0x140757ec5 488b8508040000 mov rax, qword ptr [rbp + 0x408]
0x140757ecc 488d1cd0 lea rbx, [rax + rdx*8]
0x140757ed0 488b09 mov rcx, qword ptr [rcx]
0x140757ed3 48898dd0020000 mov qword ptr [rbp + 0x2d0], rcx
0x140757eda 488b95c8020000 mov rdx, qword ptr [rbp + 0x2c8]
0x140757ee1 488b0a mov rcx, qword ptr [rdx]
0x140757ee4 48898dc8010000 mov qword ptr [rbp + 0x1c8], rcx
0x140757eeb 488b4a10 mov rcx, qword ptr [rdx + 0x10]
0x140757eef 48898de0000000 mov qword ptr [rbp + 0xe0], rcx
0x140757ef6 4c8b8550050000 mov r8, qword ptr [rbp + 0x550]
0x140757efd 498b08 mov rcx, qword ptr [r8]
0x140757f00 48898dc8000000 mov qword ptr [rbp + 0xc8], rcx
0x140757f07 498b4808 mov rcx, qword ptr [r8 + 8]
0x140757f0b 48898dd0000000 mov qword ptr [rbp + 0xd0], rcx
0x140757f12 418b4814 mov ecx, dword ptr [r8 + 0x14]
0x140757f16 898d8c030000 mov dword ptr [rbp + 0x38c], ecx
0x140757f1c 8b4a1c mov ecx, dword ptr [rdx + 0x1c]
0x140757f1f 898da4020000 mov dword ptr [rbp + 0x2a4], ecx
0x140757f25 4c8b8578050000 mov r8, qword ptr [rbp + 0x578]
0x140757f2c 418b481c mov ecx, dword ptr [r8 + 0x1c]
0x140757f30 898d9c020000 mov dword ptr [rbp + 0x29c], ecx
0x140757f36 498b10 mov rdx, qword ptr [r8]
0x140757f39 498b4810 mov rcx, qword ptr [r8 + 0x10]
0x140757f3d 48898db8000000 mov qword ptr [rbp + 0xb8], rcx
0x140757f44 4c8b8d80050000 mov r9, qword ptr [rbp + 0x580]
0x140757f4b 418b491c mov ecx, dword ptr [r9 + 0x1c]
0x140757f4f 898da0020000 mov dword ptr [rbp + 0x2a0], ecx
0x140757f55 4d8b01 mov r8, qword ptr [r9]
0x140757f58 498b4910 mov rcx, qword ptr [r9 + 0x10]
0x140757f5c 48898dd8000000 mov qword ptr [rbp + 0xd8], rcx
0x140757f63 418b4a1c mov ecx, dword ptr [r10 + 0x1c]
0x140757f67 898d88030000 mov dword ptr [rbp + 0x388], ecx
0x140757f6d 498b0a mov rcx, qword ptr [r10]
0x140757f70 48898dd8010000 mov qword ptr [rbp + 0x1d8], rcx
0x140757f77 498b4a10 mov rcx, qword ptr [r10 + 0x10]
0x140757f7b 48898de0010000 mov qword ptr [rbp + 0x1e0], rcx
0x140757f82 4c8b9568050000 mov r10, qword ptr [rbp + 0x568]
0x140757f89 418b4a1c mov ecx, dword ptr [r10 + 0x1c]
0x140757f8d 898d90030000 mov dword ptr [rbp + 0x390], ecx
0x140757f93 4d8b0a mov r9, qword ptr [r10]
0x140757f96 498b4a10 mov rcx, qword ptr [r10 + 0x10]
0x140757f9a 48898df8010000 mov qword ptr [rbp + 0x1f8], rcx
0x140757fa1 660f76f6 pcmpeqd xmm6, xmm6
0x140757fa5 0f57ff xorps xmm7, xmm7
0x140757fa8 f3440f103dbf483202 movss xmm15, dword ptr [rip + 0x23248bf]
0x140757fb1 f3440f1005b6d81c02 movss xmm8, dword ptr [rip + 0x21cd8b6]
0x140757fba 440f28359e483202 movaps xmm14, xmmword ptr [rip + 0x232489e]
0x140757fc2 440f2815962c1e02 movaps xmm10, xmmword ptr [rip + 0x21e2c96]
0x140757fca 488b8db0030000 mov rcx, qword ptr [rbp + 0x3b0]
0x140757fd1 48898d08020000 mov qword ptr [rbp + 0x208], rcx
0x140757fd8 4d8d742420 lea r14, [r12 + 0x20]
0x140757fdd 488995c8020000 mov qword ptr [rbp + 0x2c8], rdx
0x140757fe4 488d4a20 lea rcx, [rdx + 0x20]
0x140757fe8 48898dc0000000 mov qword ptr [rbp + 0xc0], rcx
0x140757fef 488b8d10040000 mov rcx, qword ptr [rbp + 0x410]
0x140757ff6 488d4920 lea rcx, [rcx + 0x20]
0x140757ffa 48898dd8020000 mov qword ptr [rbp + 0x2d8], rcx
0x140758001 4c8985f0010000 mov qword ptr [rbp + 0x1f0], r8
0x140758008 498d4820 lea rcx, [r8 + 0x20]
0x14075800c 48898d08010000 mov qword ptr [rbp + 0x108], rcx
0x140758013 4c898de8030000 mov qword ptr [rbp + 0x3e8], r9
0x14075801a 498d4920 lea rcx, [r9 + 0x20]
0x14075801e 48898d18020000 mov qword ptr [rbp + 0x218], rcx
0x140758025 4989c7 mov r15, rax
0x140758028 488985a8030000 mov qword ptr [rbp + 0x3a8], rax
0x14075802f 48899d00020000 mov qword ptr [rbp + 0x200], rbx
0x140758036 4c89b5e8010000 mov qword ptr [rbp + 0x1e8], r14
0x14075803d 4939df cmp r15, rbx
0x140758040 0f847a1b0000 je 0x140759bc0
0x140758046 498b37 mov rsi, qword ptr [r15]
0x140758049 4889b528040000 mov qword ptr [rbp + 0x428], rsi
0x140758050 49837c241800 cmp qword ptr [r12 + 0x18], 0
0x140758056 0f84531b0000 je 0x140759baf
0x14075805c 4983c708 add r15, 8
0x140758060 4c89f1 mov rcx, r14
0x140758063 488d9528040000 lea rdx, [rbp + 0x428]
0x14075806a e8018de5ff call 0x1405b0d70
0x14075806f 498b3c24 mov rdi, qword ptr [r12]
0x140758073 498b4c2408 mov rcx, qword ptr [r12 + 8]
0x140758078 4889ca mov rdx, rcx
0x14075807b 4821c2 and rdx, rax
0x14075807e 48c1e839 shr rax, 0x39
0x140758082 660f6ec0 movd xmm0, eax
0x140758086 660f60c0 punpcklbw xmm0, xmm0
0x14075808a f20f70c000 pshuflw xmm0, xmm0, 0
0x14075808f 660f70c000 pshufd xmm0, xmm0, 0
0x140758094 488d8758fdffff lea rax, [rdi - 0x2a8]
0x14075809b 4531c0 xor r8d, r8d
0x14075809e f30f6f0c17 movdqu xmm1, xmmword ptr [rdi + rdx]
0x1407580a3 660f6fd1 movdqa xmm2, xmm1
0x1407580a7 660f74d0 pcmpeqb xmm2, xmm0
0x1407580ab 66440fd7ca pmovmskb r9d, xmm2
0x1407580b0 4585c9 test r9d, r9d
0x1407580b3 742b je 0x1407580e0
0x1407580b5 f3450fbcd1 tzcnt r10d, r9d
0x1407580ba 4901d2 add r10, rdx
0x1407580bd 4921ca and r10, rcx
0x1407580c0 49f7da neg r10
0x1407580c3 4969daa8020000 imul rbx, r10, 0x2a8
0x1407580ca 483b3418 cmp rsi, qword ptr [rax + rbx]
0x1407580ce 7440 je 0x140758110
0x1407580d0 458d51ff lea r10d, [r9 - 1]
0x1407580d4 664521ca and r10w, r9w
0x1407580d8 4589d1 mov r9d, r10d
0x1407580db 75d8 jne 0x1407580b5
0x1407580dd 0f1f00 nop dword ptr [rax]
0x1407580e0 660f74ce pcmpeqb xmm1, xmm6
0x1407580e4 66440fd7c9 pmovmskb r9d, xmm1
0x1407580e9 4585c9 test r9d, r9d
0x1407580ec 0f85bd1a0000 jne 0x140759baf
0x1407580f2 4c01c2 add rdx, r8
0x1407580f5 4883c210 add rdx, 0x10
0x1407580f9 4983c010 add r8, 0x10
0x1407580fd 4821ca and rdx, rcx
0x140758100 eb9c jmp 0x14075809e
0x140758102 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140758110 4c89bdd0010000 mov qword ptr [rbp + 0x1d0], r15
0x140758117 48c785b002000000000000 mov qword ptr [rbp + 0x2b0], 0
0x140758122 48c785b802000004000000 mov qword ptr [rbp + 0x2b8], 4
0x14075812d 48c785c002000000000000 mov qword ptr [rbp + 0x2c0], 0
0x140758138 8b859c020000 mov eax, dword ptr [rbp + 0x29c]
0x14075813e 488b8db8000000 mov rcx, qword ptr [rbp + 0xb8]
0x140758145 8901 mov dword ptr [rcx], eax
0x140758147 488b85c8020000 mov rax, qword ptr [rbp + 0x2c8]
0x14075814e 4c8b7030 mov r14, qword ptr [rax + 0x30]
0x140758152 4c8b7840 mov r15, qword ptr [rax + 0x40]
0x140758156 4c3b7020 cmp r14, qword ptr [rax + 0x20]
0x14075815a 751a jne 0x140758176
0x14075815c c6853704000001 mov byte ptr [rbp + 0x437], 1
0x140758163 488b8dc0000000 mov rcx, qword ptr [rbp + 0xc0]
0x14075816a 488d1557434c02 lea rdx, [rip + 0x24c4357]
0x140758171 e8baa4e0ff call 0x140562630
0x140758176 488b95c8020000 mov rdx, qword ptr [rbp + 0x2c8]
0x14075817d 488b4228 mov rax, qword ptr [rdx + 0x28]
0x140758181 4c89f1 mov rcx, r14
0x140758184 48c1e104 shl rcx, 4
0x140758188 4c893c08 mov qword ptr [rax + rcx], r15
0x14075818c 4889740808 mov qword ptr [rax + rcx + 8], rsi
0x140758191 49ffc6 inc r14
0x140758194 4c897230 mov qword ptr [rdx + 0x30], r14
0x140758198 48ff4240 inc qword ptr [rdx + 0x40]
0x14075819c 488d341f lea rsi, [rdi + rbx]
0x1407581a0 488d041f lea rax, [rdi + rbx]
0x1407581a4 480560fdffff add rax, -0x2a0
0x1407581aa 48898520040000 mov qword ptr [rbp + 0x420], rax
0x1407581b1 807ef600 cmp byte ptr [rsi - 0xa], 0
0x1407581b5 0f85b3110000 jne 0x14075936e
0x1407581bb 488b9528040000 mov rdx, qword ptr [rbp + 0x428]
0x1407581c2 c6853704000001 mov byte ptr [rbp + 0x437], 1
0x1407581c9 4c89e1 mov rcx, r12
0x1407581cc e8dfa03300 call 0x140a922b0
0x1407581d1 4889c2 mov rdx, rax
0x1407581d4 48c1ea20 shr rdx, 0x20
0x1407581d8 4889b518040000 mov qword ptr [rbp + 0x418], rsi
0x1407581df 7534 jne 0x140758215
0x1407581e1 488d4ef5 lea rcx, [rsi - 0xb]
0x1407581e5 c6853704000001 mov byte ptr [rbp + 0x437], 1
0x1407581ec e83fb00d00 call 0x140833230
0x1407581f1 488985f0030000 mov qword ptr [rbp + 0x3f0], rax
0x1407581f8 488b85f0030000 mov rax, qword ptr [rbp + 0x3f0]
0x1407581ff 00c0 add al, al
0x140758201 488985f0030000 mov qword ptr [rbp + 0x3f0], rax
0x140758208 4531ed xor r13d, r13d
0x14075820b 4531e4 xor r12d, r12d
0x14075820e 31ff xor edi, edi
0x140758210 e965010000 jmp 0x14075837a
0x140758215 4189c1 mov r9d, eax
0x140758218 488bb5f8000000 mov rsi, qword ptr [rbp + 0xf8]
0x14075821f 4c3b4e10 cmp r9, qword ptr [rsi + 0x10]
0x140758223 0f83c61b0000 jae 0x140759def
0x140758229 488b4e08 mov rcx, qword ptr [rsi + 8]
0x14075822d 4889c2 mov rdx, rax
0x140758230 48c1ea20 shr rdx, 0x20
0x140758234 4f8d0489 lea r8, [r9 + r9*4]
0x140758238 42391481 cmp dword ptr [rcx + r8*4], edx
0x14075823c 4c8b9d00010000 mov r11, qword ptr [rbp + 0x100]
0x140758243 0f85a61b0000 jne 0x140759def
0x140758249 4a8d0c81 lea rcx, [rcx + r8*4]
0x14075824d 448b4104 mov r8d, dword ptr [rcx + 4]
0x140758251 41baffffffff mov r10d, 0xffffffff
0x140758257 4d39d0 cmp r8, r10
0x14075825a 0f848f1b0000 je 0x140759def
0x140758260 4531c9 xor r9d, r9d
0x140758263 4d394338 cmp qword ptr [r11 + 0x38], r8
0x140758267 0f86921b0000 jbe 0x140759dff
0x14075826d 498b5328 mov rdx, qword ptr [r11 + 0x28]
0x140758271 4589c2 mov r10d, r8d
0x140758274 41c1ea06 shr r10d, 6
0x140758278 4a8b14d2 mov rdx, qword ptr [rdx + r10*8]
0x14075827c 4c0fa3c2 bt rdx, r8
0x140758280 0f83791b0000 jae 0x140759dff
0x140758286 8b410c mov eax, dword ptr [rcx + 0xc]
0x140758289 488b96a8010000 mov rdx, qword ptr [rsi + 0x1a8]
0x140758290 4c8d04c0 lea r8, [rax + rax*8]
0x140758294 498b8310010000 mov rax, qword ptr [r11 + 0x110]
0x14075829b 4a3944c240 cmp qword ptr [rdx + r8*8 + 0x40], rax
0x1407582a0 762e jbe 0x1407582d0
0x1407582a2 4a8d14c2 lea rdx, [rdx + r8*8]
0x1407582a6 4c8b4238 mov r8, qword ptr [rdx + 0x38]
0x1407582aa 498b04c0 mov rax, qword ptr [r8 + rax*8]
0x1407582ae 4885c0 test rax, rax
0x1407582b1 488bb518040000 mov rsi, qword ptr [rbp + 0x418]
0x1407582b8 7421 je 0x1407582db
0x1407582ba 488b5218 mov rdx, qword ptr [rdx + 0x18]
0x1407582be 48f7d0 not rax
0x1407582c1 488d0440 lea rax, [rax + rax*2]
0x1407582c5 48c1e004 shl rax, 4
0x1407582c9 488b440210 mov rax, qword ptr [rdx + rax + 0x10]
0x1407582ce eb0d jmp 0x1407582dd
0x1407582d0 31c0 xor eax, eax
0x1407582d2 488bb518040000 mov rsi, qword ptr [rbp + 0x418]
0x1407582d9 eb02 jmp 0x1407582dd
0x1407582db 31c0 xor eax, eax
0x1407582dd 8b4910 mov ecx, dword ptr [rcx + 0x10]
0x1407582e0 486bf958 imul rdi, rcx, 0x58
0x1407582e4 4801c7 add rdi, rax
0x1407582e7 c6853704000001 mov byte ptr [rbp + 0x437], 1
0x1407582ee 4889f9 mov rcx, rdi
0x1407582f1 e8bab01800 call 0x1408e33b0
0x1407582f6 4189c5 mov r13d, eax
0x1407582f9 c6853704000001 mov byte ptr [rbp + 0x437], 1
0x140758300 4889f9 mov rcx, rdi
0x140758303 e848af1800 call 0x1408e3250
0x140758308 84c0 test al, al
0x14075830a 7531 jne 0x14075833d
0x14075830c 488d4f3d lea rcx, [rdi + 0x3d]
0x140758310 c6853704000001 mov byte ptr [rbp + 0x437], 1
0x140758317 e814af0d00 call 0x140833230
0x14075831c 84c0 test al, al
0x14075831e 741d je 0x14075833d
0x140758320 488d4ef9 lea rcx, [rsi - 7]
0x140758324 c6853704000001 mov byte ptr [rbp + 0x437], 1
0x14075832b e840d31600 call 0x1408c5670
0x140758330 00c0 add al, al
0x140758332 fec0 inc al
0x140758334 488985f0030000 mov qword ptr [rbp + 0x3f0], rax
0x14075833b eb0b jmp 0x140758348
0x14075833d 48c785f003000000000000 mov qword ptr [rbp + 0x3f0], 0
0x140758348 c6853704000001 mov byte ptr [rbp + 0x437], 1
0x14075834f 488d8df0020000 lea rcx, [rbp + 0x2f0]
0x140758356 4889fa mov rdx, rdi
0x140758359 e882b01800 call 0x1408e33e0
0x14075835e 4c8ba5f0020000 mov r12, qword ptr [rbp + 0x2f0]
0x140758365 4c8bb5f8020000 mov r14, qword ptr [rbp + 0x2f8]
0x14075836c 488b8500030000 mov rax, qword ptr [rbp + 0x300]
0x140758373 488985d0030000 mov qword ptr [rbp + 0x3d0], rax
0x14075837a c6853704000001 mov byte ptr [rbp + 0x437], 1
0x140758381 488d8d28040000 lea rcx, [rbp + 0x428]
0x140758388 e8430293ff call 0x1400885d0
0x14075838d 4869c02180b204 imul rax, rax, 0x4b28021
0x140758394 480386a8fdffff add rax, qword ptr [rsi - 0x258]
0x14075839b 488985e8000000 mov qword ptr [rbp + 0xe8], rax
0x1407583a2 4c8bbef8fdffff mov r15, qword ptr [rsi - 0x208]
0x1407583a9 4d85ff test r15, r15
0x1407583ac 4c89a5c8030000 mov qword ptr [rbp + 0x3c8], r12
0x1407583b3 0f84f3000000 je 0x1407584ac
0x1407583b9 488bb6f0fdffff mov rsi, qword ptr [rsi - 0x210]
0x1407583c0 4a8d1cbd00000000 lea rbx, [r15*4]
0x1407583c8 0fb6053ac8ed02 movzx eax, byte ptr [rip + 0x2edc83a]
0x1407583cf ba04000000 mov edx, 4
0x1407583d4 4889d9 mov rcx, rbx
0x1407583d7 e834b8ebff call 0x140613c10
0x1407583dc 4885c0 test rax, rax
0x1407583df 0f84781b0000 je 0x140759f5d
0x1407583e5 4983ff07 cmp r15, 7
0x1407583e9 48898500040000 mov qword ptr [rbp + 0x400], rax
0x1407583f0 7628 jbe 0x14075841a
0x1407583f2 4e8d04b8 lea r8, [rax + r15*4]
0x1407583f6 488d4e04 lea rcx, [rsi + 4]
0x1407583fa 4b8d147f lea rdx, [r15 + r15*2]
0x1407583fe 488d1496 lea rdx, [rsi + rdx*4]
0x140758402 4883c2fc add rdx, -4
0x140758406 4839d0 cmp rax, rdx
0x140758409 0f92c2 setb dl
0x14075840c 4c39c1 cmp rcx, r8
0x14075840f 0f92c1 setb cl
0x140758412 84ca test dl, cl
0x140758414 0f84a0000000 je 0x1407584ba
0x14075841a 4531d2 xor r10d, r10d
0x14075841d 4d89f8 mov r8, r15
0x140758420 4c89d1 mov rcx, r10
0x140758423 4983e003 and r8, 3
0x140758427 742e je 0x140758457
0x140758429 4b8d0c52 lea rcx, [r10 + r10*2]
0x14075842d 488d148e lea rdx, [rsi + rcx*4]
0x140758431 4883c204 add rdx, 4
0x140758435 4f8d0440 lea r8, [r8 + r8*2]
0x140758439 4531c9 xor r9d, r9d
0x14075843c 4c89d1 mov rcx, r10
0x14075843f 90 nop 
0x140758440 66420f6e048a movd xmm0, dword ptr [rdx + r9*4]
0x140758446 660f7e0488 movd dword ptr [rax + rcx*4], xmm0
0x14075844b 48ffc1 inc rcx
0x14075844e 4983c103 add r9, 3
0x140758452 4d39c8 cmp r8, r9
0x140758455 75e9 jne 0x140758440
0x140758457 4d29fa sub r10, r15
0x14075845a 4983fafc cmp r10, -4
0x14075845e 0f87cc000000 ja 0x140758530
0x140758464 488d1449 lea rdx, [rcx + rcx*2]
0x140758468 488d1496 lea rdx, [rsi + rdx*4]
0x14075846c 4883c228 add rdx, 0x28
0x140758470 f30f1042dc movss xmm0, dword ptr [rdx - 0x24]
0x140758475 f30f110488 movss dword ptr [rax + rcx*4], xmm0
0x14075847a f30f1042e8 movss xmm0, dword ptr [rdx - 0x18]
0x14075847f f30f11448804 movss dword ptr [rax + rcx*4 + 4], xmm0
0x140758485 f30f1042f4 movss xmm0, dword ptr [rdx - 0xc]
0x14075848a f30f11448808 movss dword ptr [rax + rcx*4 + 8], xmm0
0x140758490 660f6e02 movd xmm0, dword ptr [rdx]
0x140758494 660f7e44880c movd dword ptr [rax + rcx*4 + 0xc], xmm0
0x14075849a 4883c104 add rcx, 4
0x14075849e 4883c230 add rdx, 0x30
0x1407584a2 4939cf cmp r15, rcx
0x1407584a5 75c9 jne 0x140758470
0x1407584a7 e984000000 jmp 0x140758530
0x1407584ac b804000000 mov eax, 4
0x1407584b1 48898500040000 mov qword ptr [rbp + 0x400], rax
0x1407584b8 eb7d jmp 0x140758537
0x1407584ba 4d89fa mov r10, r15
0x1407584bd 4983e2f8 and r10, 0xfffffffffffffff8
0x1407584c1 488d4e58 lea rcx, [rsi + 0x58]
0x1407584c5 31d2 xor edx, edx
0x1407584c7 660f1f840000000000 nop word ptr [rax + rax]
0x1407584d0 660f6e41ac movd xmm0, dword ptr [rcx - 0x54]
0x1407584d5 660f6e49b8 movd xmm1, dword ptr [rcx - 0x48]
0x1407584da 660f62c1 punpckldq xmm0, xmm1
0x1407584de 660f6e49c4 movd xmm1, dword ptr [rcx - 0x3c]
0x1407584e3 660f6e51d0 movd xmm2, dword ptr [rcx - 0x30]
0x1407584e8 660f62ca punpckldq xmm1, xmm2
0x1407584ec 660f6cc1 punpcklqdq xmm0, xmm1
0x1407584f0 660f6e09 movd xmm1, dword ptr [rcx]
0x1407584f4 660f6e51f4 movd xmm2, dword ptr [rcx - 0xc]
0x1407584f9 660f62d1 punpckldq xmm2, xmm1
0x1407584fd 660f6e49e8 movd xmm1, dword ptr [rcx - 0x18]
0x140758502 660f6e59dc movd xmm3, dword ptr [rcx - 0x24]
0x140758507 660f62d9 punpckldq xmm3, xmm1
0x14075850b 660f6cda punpcklqdq xmm3, xmm2
0x14075850f f30f7f0490 movdqu xmmword ptr [rax + rdx*4], xmm0
0x140758514 f30f7f5c9010 movdqu xmmword ptr [rax + rdx*4 + 0x10], xmm3
0x14075851a 4883c208 add rdx, 8
0x14075851e 4883c160 add rcx, 0x60
0x140758522 4939d2 cmp r10, rdx
0x140758525 75a9 jne 0x1407584d0
0x140758527 4d39d7 cmp r15, r10
0x14075852a 0f85edfeffff jne 0x14075841d
0x140758530 488bb518040000 mov rsi, qword ptr [rbp + 0x418]
0x140758537 4c89bdd8030000 mov qword ptr [rbp + 0x3d8], r15
0x14075853e 488b8648feffff mov rax, qword ptr [rsi - 0x1b8]
0x140758545 4c8d248500000000 lea r12, [rax*4]
0x14075854d 488985a0030000 mov qword ptr [rbp + 0x3a0], rax
0x140758554 48c1e83e shr rax, 0x3e
0x140758558 0f852a1a0000 jne 0x140759f88
0x14075855e 41bf00000000 mov r15d, 0
0x140758564 48b8fcffffffffffff7f movabs rax, 0x7ffffffffffffffc
0x14075856e 4939c4 cmp r12, rax
0x140758571 0f87141a0000 ja 0x140759f8b
0x140758577 488b9e40feffff mov rbx, qword ptr [rsi - 0x1c0]
0x14075857e 4d85e4 test r12, r12
0x140758581 7433 je 0x1407585b6
0x140758583 0fb6057fc6ed02 movzx eax, byte ptr [rip + 0x2edc67f]
0x14075858a 41bf04000000 mov r15d, 4
0x140758590 ba04000000 mov edx, 4
0x140758595 4c89e1 mov rcx, r12
0x140758598 e873b6ebff call 0x140613c10
0x14075859d 4885c0 test rax, rax
0x1407585a0 0f84e5190000 je 0x140759f8b
0x1407585a6 488b8da0030000 mov rcx, qword ptr [rbp + 0x3a0]
0x1407585ad 48898de0020000 mov qword ptr [rbp + 0x2e0], rcx
0x1407585b4 eb10 jmp 0x1407585c6
0x1407585b6 b804000000 mov eax, 4
0x1407585bb 48c785e002000000000000 mov qword ptr [rbp + 0x2e0], 0
0x1407585c6 488985e0030000 mov qword ptr [rbp + 0x3e0], rax
0x1407585cd 4889c1 mov rcx, rax
0x1407585d0 4889da mov rdx, rbx
0x1407585d3 4d89e0 mov r8, r12
0x1407585d6 e825b61c02 call 0x142923c00
0x1407585db 4885ff test rdi, rdi
0x1407585de 0f84cc000000 je 0x1407586b0
0x1407585e4 4889f9 mov rcx, rdi
0x1407585e7 e894ad1800 call 0x1408e3380
0x1407585ec 84c0 test al, al
0x1407585ee 0f84bc000000 je 0x1407586b0
0x1407585f4 488d4f3d lea rcx, [rdi + 0x3d]
0x1407585f8 e833ac0d00 call 0x140833230
0x1407585fd 84c0 test al, al
0x1407585ff 0f85ab000000 jne 0x1407586b0
0x140758605 4883bda003000000 cmp qword ptr [rbp + 0x3a0], 0
0x14075860d 0f849d000000 je 0x1407586b0
0x140758613 4983c4fc add r12, -4
0x140758617 488b85e0030000 mov rax, qword ptr [rbp + 0x3e0]
0x14075861e 4983fc1b cmp r12, 0x1b
0x140758622 7650 jbe 0x140758674
0x140758624 49c1ec02 shr r12, 2
0x140758628 49ffc4 inc r12
0x14075862b 4c89e1 mov rcx, r12
0x14075862e 4883e1f8 and rcx, 0xfffffffffffffff8
0x140758632 4c8b85e0030000 mov r8, qword ptr [rbp + 0x3e0]
0x140758639 498d0488 lea rax, [r8 + rcx*4]
0x14075863d 31d2 xor edx, edx
0x14075863f 90 nop 
0x140758640 410f100490 movups xmm0, xmmword ptr [r8 + rdx*4]
0x140758645 410f104c9010 movups xmm1, xmmword ptr [r8 + rdx*4 + 0x10]
0x14075864b 410f58c6 addps xmm0, xmm14
0x14075864f 410f58ce addps xmm1, xmm14
0x140758653 410f5fc2 maxps xmm0, xmm10
0x140758657 410f5fca maxps xmm1, xmm10
0x14075865b 410f110490 movups xmmword ptr [r8 + rdx*4], xmm0
0x140758660 410f114c9010 movups xmmword ptr [r8 + rdx*4 + 0x10], xmm1
0x140758666 4883c208 add rdx, 8
0x14075866a 4839d1 cmp rcx, rdx
0x14075866d 75d1 jne 0x140758640
0x14075866f 4939cc cmp r12, rcx
0x140758672 743c je 0x1407586b0
0x140758674 488b8de0030000 mov rcx, qword ptr [rbp + 0x3e0]
0x14075867b 488b95a0030000 mov rdx, qword ptr [rbp + 0x3a0]
0x140758682 488d0c91 lea rcx, [rcx + rdx*4]
0x140758686 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140758690 f30f1000 movss xmm0, dword ptr [rax]
0x140758694 f3410f58c7 addss xmm0, xmm15
0x140758699 f3410f5fc0 maxss xmm0, xmm8
0x14075869e f30f1100 movss dword ptr [rax], xmm0
0x1407586a2 488d5004 lea rdx, [rax + 4]
0x1407586a6 4889d0 mov rax, rdx
0x1407586a9 4839ca cmp rdx, rcx
0x1407586ac 75e2 jne 0x140758690
0x1407586ae 6690 nop 
0x1407586b0 488b96f0fdffff mov rdx, qword ptr [rsi - 0x210]
0x1407586b7 488b86f8fdffff mov rax, qword ptr [rsi - 0x208]
0x1407586be 488d0440 lea rax, [rax + rax*2]
0x1407586c2 4c8d0482 lea r8, [rdx + rax*4]
0x1407586c6 488d8d30020000 lea rcx, [rbp + 0x230]
0x1407586cd 4c8d0dbc044c02 lea r9, [rip + 0x24c04bc]
0x1407586d4 e80723b6ff call 0x1402ba9e0
0x1407586d9 488d8df0020000 lea rcx, [rbp + 0x2f0]
0x1407586e0 488d9530020000 lea rdx, [rbp + 0x230]
0x1407586e7 4531c0 xor r8d, r8d
0x1407586ea e83112aa00 call 0x1411f9920
0x1407586ef 31c0 xor eax, eax
0x1407586f1 483b85f0020000 cmp rax, qword ptr [rbp + 0x2f0]
0x1407586f8 0f8022180000 jo 0x140759f20
0x1407586fe 488b8520030000 mov rax, qword ptr [rbp + 0x320]
0x140758705 48898570010000 mov qword ptr [rbp + 0x170], rax
0x14075870c f30f6f85f0020000 movdqu xmm0, xmmword ptr [rbp + 0x2f0]
0x140758714 f30f6f8d00030000 movdqu xmm1, xmmword ptr [rbp + 0x300]
0x14075871c f30f6f9510030000 movdqu xmm2, xmmword ptr [rbp + 0x310]
0x140758724 660f7f9560010000 movdqa xmmword ptr [rbp + 0x160], xmm2
0x14075872c 660f7f8d50010000 movdqa xmmword ptr [rbp + 0x150], xmm1
0x140758734 660f7f8540010000 movdqa xmmword ptr [rbp + 0x140], xmm0
0x14075873c 41b402 mov r12b, 2
0x14075873f 4584ed test r13b, r13b
0x140758742 0f8495000000 je 0x1407587dd
0x140758748 488b8d20040000 mov rcx, qword ptr [rbp + 0x420]
0x14075874f e8cc1e3d00 call 0x140b2a620
0x140758754 660f7e8598030000 movd dword ptr [rbp + 0x398], xmm0
0x14075875c 488d4ef9 lea rcx, [rsi - 7]
0x140758760 e80bcf1600 call 0x1408c5670
0x140758765 4188c4 mov r12b, al
0x140758768 488b8d20040000 mov rcx, qword ptr [rbp + 0x420]
0x14075876f e85c1e3d00 call 0x140b2a5d0
0x140758774 4889c1 mov rcx, rax
0x140758777 e8c46faa00 call 0x1411ff740
0x14075877c f685c803000001 test byte ptr [rbp + 0x3c8], 1
0x140758783 0f84f1170000 je 0x140759f7a
0x140758789 4989c5 mov r13, rax
0x14075878c 4889c1 mov rcx, rax
0x14075878f 4c89f2 mov rdx, r14
0x140758792 e8390f5400 call 0x140c996d0
0x140758797 66440f6fd8 movdqa xmm11, xmm0
0x14075879c 49ffc6 inc r14
0x14075879f 4c89e9 mov rcx, r13
0x1407587a2 4c89f2 mov rdx, r14
0x1407587a5 e8260f5400 call 0x140c996d0
0x1407587aa 66440f6fe0 movdqa xmm12, xmm0
0x1407587af 4c89e9 mov rcx, r13
0x1407587b2 488b95d0030000 mov rdx, qword ptr [rbp + 0x3d0]
0x1407587b9 e8120f5400 call 0x140c996d0
0x1407587be 66440f6fe8 movdqa xmm13, xmm0
0x1407587c3 488b95d0030000 mov rdx, qword ptr [rbp + 0x3d0]
0x1407587ca 48ffc2 inc rdx
0x1407587cd 4c89e9 mov rcx, r13
0x1407587d0 e8fb0e5400 call 0x140c996d0
0x1407587d5 660f7e859c030000 movd dword ptr [rbp + 0x39c], xmm0
0x1407587dd 4c8d7ef9 lea r15, [rsi - 7]
0x1407587e1 c6853404000000 mov byte ptr [rbp + 0x434], 0
0x1407587e8 4c89f9 mov rcx, r15
0x1407587eb e880ce1600 call 0x1408c5670
0x1407587f0 84c0 test al, al
0x1407587f2 0f84c7000000 je 0x1407588bf
0x1407587f8 8b8594030000 mov eax, dword ptr [rbp + 0x394]
0x1407587fe 488b8d10020000 mov rcx, qword ptr [rbp + 0x210]
0x140758805 8901 mov dword ptr [rcx], eax
0x140758807 488bb528040000 mov rsi, qword ptr [rbp + 0x428]
0x14075880e 4889b5f0020000 mov qword ptr [rbp + 0x2f0], rsi
0x140758815 488b8dd8020000 mov rcx, qword ptr [rbp + 0x2d8]
0x14075881c 488d95f0020000 lea rdx, [rbp + 0x2f0]
0x140758823 e84885e5ff call 0x1405b0d70
0x140758828 4889c3 mov rbx, rax
0x14075882b 48c1eb39 shr rbx, 0x39
0x14075882f 488b9510040000 mov rdx, qword ptr [rbp + 0x410]
0x140758836 488b0a mov rcx, qword ptr [rdx]
0x140758839 488b5208 mov rdx, qword ptr [rdx + 8]
0x14075883d 4989d0 mov r8, rdx
0x140758840 4921c0 and r8, rax
0x140758843 660f6ec3 movd xmm0, ebx
0x140758847 660f60c0 punpcklbw xmm0, xmm0
0x14075884b f20f70c000 pshuflw xmm0, xmm0, 0
0x140758850 660f70c000 pshufd xmm0, xmm0, 0
0x140758855 4c8d4998 lea r9, [rcx - 0x68]
0x140758859 4531d2 xor r10d, r10d
0x14075885c f3420f6f0c01 movdqu xmm1, xmmword ptr [rcx + r8]
0x140758862 660f6fd1 movdqa xmm2, xmm1
0x140758866 660f74d0 pcmpeqb xmm2, xmm0
0x14075886a 66440fd7da pmovmskb r11d, xmm2
0x14075886f 4585db test r11d, r11d
0x140758872 7429 je 0x14075889d
0x140758874 f3450fbcf3 tzcnt r14d, r11d
0x140758879 4d01c6 add r14, r8
0x14075887c 4921d6 and r14, rdx
0x14075887f 49f7de neg r14
0x140758882 4d6bf668 imul r14, r14, 0x68
0x140758886 4b393431 cmp qword ptr [r9 + r14], rsi
0x14075888a 0f84f2000000 je 0x140758982
0x140758890 458d73ff lea r14d, [r11 - 1]
0x140758894 664521de and r14w, r11w
0x140758898 4589f3 mov r11d, r14d
0x14075889b 75d7 jne 0x140758874
0x14075889d 660f74ce pcmpeqb xmm1, xmm6
0x1407588a1 66440fd7d9 pmovmskb r11d, xmm1
0x1407588a6 4585db test r11d, r11d
0x1407588a9 0f85e3000000 jne 0x140758992
0x1407588af 4d01d0 add r8, r10
0x1407588b2 4983c010 add r8, 0x10
0x1407588b6 4983c210 add r10, 0x10
0x1407588ba 4921d0 and r8, rdx
0x1407588bd eb9d jmp 0x14075885c
0x1407588bf 8b8594030000 mov eax, dword ptr [rbp + 0x394]
0x1407588c5 488b8d10020000 mov rcx, qword ptr [rbp + 0x210]
0x1407588cc 8901 mov dword ptr [rcx], eax
0x1407588ce 488bb528040000 mov rsi, qword ptr [rbp + 0x428]
0x1407588d5 4889b5f0020000 mov qword ptr [rbp + 0x2f0], rsi
0x1407588dc 488b8dd8020000 mov rcx, qword ptr [rbp + 0x2d8]
0x1407588e3 488d95f0020000 lea rdx, [rbp + 0x2f0]
0x1407588ea e88184e5ff call 0x1405b0d70
0x1407588ef 4889c3 mov rbx, rax
0x1407588f2 48c1eb39 shr rbx, 0x39
0x1407588f6 488b8d10040000 mov rcx, qword ptr [rbp + 0x410]
0x1407588fd 488b11 mov rdx, qword ptr [rcx]
0x140758900 488b4908 mov rcx, qword ptr [rcx + 8]
0x140758904 4989c8 mov r8, rcx
0x140758907 4921c0 and r8, rax
0x14075890a 660f6ec3 movd xmm0, ebx
0x14075890e 660f60c0 punpcklbw xmm0, xmm0
0x140758912 f20f70c000 pshuflw xmm0, xmm0, 0
0x140758917 660f70c000 pshufd xmm0, xmm0, 0
0x14075891c 4c8d4a98 lea r9, [rdx - 0x68]
0x140758920 4531d2 xor r10d, r10d
0x140758923 f3420f6f0c02 movdqu xmm1, xmmword ptr [rdx + r8]
0x140758929 660f6fd1 movdqa xmm2, xmm1
0x14075892d 660f74d0 pcmpeqb xmm2, xmm0
0x140758931 66440fd7da pmovmskb r11d, xmm2
0x140758936 4585db test r11d, r11d
0x140758939 7425 je 0x140758960
0x14075893b f3450fbcf3 tzcnt r14d, r11d
0x140758940 4d01c6 add r14, r8
0x140758943 4921ce and r14, rcx
0x140758946 49f7de neg r14
0x140758949 4d6bf668 imul r14, r14, 0x68
0x14075894d 4b393431 cmp qword ptr [r9 + r14], rsi
0x140758951 7437 je 0x14075898a
0x140758953 458d73ff lea r14d, [r11 - 1]
0x140758957 664521de and r14w, r11w
0x14075895b 4589f3 mov r11d, r14d
0x14075895e 75db jne 0x14075893b
0x140758960 660f74ce pcmpeqb xmm1, xmm6
0x140758964 66440fd7d9 pmovmskb r11d, xmm1
0x140758969 4585db test r11d, r11d
0x14075896c 0f854e010000 jne 0x140758ac0
0x140758972 4d01d0 add r8, r10
0x140758975 4983c010 add r8, 0x10
0x140758979 4983c210 add r10, 0x10
0x14075897d 4921c8 and r8, rcx
0x140758980 eba1 jmp 0x140758923
0x140758982 4c01f1 add rcx, r14
0x140758985 e908010000 jmp 0x140758a92
0x14075898a 4c01f2 add rdx, r14
0x14075898d e92a020000 jmp 0x140758bbc
0x140758992 4989c6 mov r14, rax
0x140758995 488b8510040000 mov rax, qword ptr [rbp + 0x410]
0x14075899c 4883781000 cmp qword ptr [rax + 0x10], 0
0x1407589a1 0f848c110000 je 0x140759b33
0x1407589a7 c6853404000000 mov byte ptr [rbp + 0x434], 0
0x1407589ae 488d8df8020000 lea rcx, [rbp + 0x2f8]
0x1407589b5 e896e48200 call 0x140f86e50
0x1407589ba 4889b5f0020000 mov qword ptr [rbp + 0x2f0], rsi
0x1407589c1 488b8d10040000 mov rcx, qword ptr [rbp + 0x410]
0x1407589c8 488b01 mov rax, qword ptr [rcx]
0x1407589cb 488b4908 mov rcx, qword ptr [rcx + 8]
0x1407589cf 4c89f2 mov rdx, r14
0x1407589d2 4821ca and rdx, rcx
0x1407589d5 f30f6f0410 movdqu xmm0, xmmword ptr [rax + rdx]
0x1407589da 66440fd7c0 pmovmskb r8d, xmm0
0x1407589df 4585c0 test r8d, r8d
0x1407589e2 0f8401110000 je 0x140759ae9
0x1407589e8 f3450fbcc0 tzcnt r8d, r8d
0x1407589ed 4901d0 add r8, rdx
0x1407589f0 4921c8 and r8, rcx
0x1407589f3 420fb61400 movzx edx, byte ptr [rax + r8]
0x1407589f8 84d2 test dl, dl
0x1407589fa 0f8981110000 jns 0x140759b81
0x140758a00 4d8d48f0 lea r9, [r8 - 0x10]
0x140758a04 4921c9 and r9, rcx
0x140758a07 42881c00 mov byte ptr [rax + r8], bl
0x140758a0b 42885c0810 mov byte ptr [rax + r9 + 0x10], bl
0x140758a10 49f7d8 neg r8
0x140758a13 4d6bc068 imul r8, r8, 0x68
0x140758a17 4a8d0c00 lea rcx, [rax + r8]
0x140758a1b 80e201 and dl, 1
0x140758a1e 0fb6d2 movzx edx, dl
0x140758a21 4c8b8d10040000 mov r9, qword ptr [rbp + 0x410]
0x140758a28 49295110 sub qword ptr [r9 + 0x10], rdx
0x140758a2c 488b9550030000 mov rdx, qword ptr [rbp + 0x350]
0x140758a33 4a895400f8 mov qword ptr [rax + r8 - 8], rdx
0x140758a38 0f108540030000 movups xmm0, xmmword ptr [rbp + 0x340]
0x140758a3f 420f114400e8 movups xmmword ptr [rax + r8 - 0x18], xmm0
0x140758a45 0f108530030000 movups xmm0, xmmword ptr [rbp + 0x330]
0x140758a4c 420f114400d8 movups xmmword ptr [rax + r8 - 0x28], xmm0
0x140758a52 f30f6f85f0020000 movdqu xmm0, xmmword ptr [rbp + 0x2f0]
0x140758a5a f30f6f8d00030000 movdqu xmm1, xmmword ptr [rbp + 0x300]
0x140758a62 f30f6f9510030000 movdqu xmm2, xmmword ptr [rbp + 0x310]
0x140758a6a f30f6f9d20030000 movdqu xmm3, xmmword ptr [rbp + 0x320]
0x140758a72 f3420f7f5c00c8 movdqu xmmword ptr [rax + r8 - 0x38], xmm3
0x140758a79 f3420f7f5400b8 movdqu xmmword ptr [rax + r8 - 0x48], xmm2
0x140758a80 f3420f7f4c00a8 movdqu xmmword ptr [rax + r8 - 0x58], xmm1
0x140758a87 f3420f7f440098 movdqu xmmword ptr [rax + r8 - 0x68], xmm0
0x140758a8e 49ff4118 inc qword ptr [r9 + 0x18]
0x140758a92 4883c1a0 add rcx, -0x60
0x140758a96 c6853404000000 mov byte ptr [rbp + 0x434], 0
0x140758a9d e80ee48200 call 0x140f86eb0
0x140758aa2 4989c6 mov r14, rax
0x140758aa5 450f28ce movaps xmm9, xmm14
0x140758aa9 450f28f0 movaps xmm14, xmm8
0x140758aad 450f28c7 movaps xmm8, xmm15
0x140758ab1 c785c803000000000000 mov dword ptr [rbp + 0x3c8], 0
0x140758abb e989020000 jmp 0x140758d49
0x140758ac0 4989c6 mov r14, rax
0x140758ac3 488b8510040000 mov rax, qword ptr [rbp + 0x410]
0x140758aca 4883781000 cmp qword ptr [rax + 0x10], 0
0x140758acf 0f8485100000 je 0x140759b5a
0x140758ad5 c6853404000000 mov byte ptr [rbp + 0x434], 0
0x140758adc 488d8df8020000 lea rcx, [rbp + 0x2f8]
0x140758ae3 e868e38200 call 0x140f86e50
0x140758ae8 4889b5f0020000 mov qword ptr [rbp + 0x2f0], rsi
0x140758aef 488b8d10040000 mov rcx, qword ptr [rbp + 0x410]
0x140758af6 488b01 mov rax, qword ptr [rcx]
0x140758af9 488b4908 mov rcx, qword ptr [rcx + 8]
0x140758afd 4c89f2 mov rdx, r14
0x140758b00 4821ca and rdx, rcx
0x140758b03 f30f6f0410 movdqu xmm0, xmmword ptr [rax + rdx]
0x140758b08 66440fd7c0 pmovmskb r8d, xmm0
0x140758b0d 4585c0 test r8d, r8d
0x140758b10 0f84f80f0000 je 0x140759b0e
0x140758b16 f3450fbcc0 tzcnt r8d, r8d
0x140758b1b 4901d0 add r8, rdx
0x140758b1e 4921c8 and r8, rcx
0x140758b21 460fb60c00 movzx r9d, byte ptr [rax + r8]
0x140758b26 4584c9 test r9b, r9b
0x140758b29 0f8969100000 jns 0x140759b98
0x140758b2f 498d50f0 lea rdx, [r8 - 0x10]
0x140758b33 4821ca and rdx, rcx
0x140758b36 42881c00 mov byte ptr [rax + r8], bl
0x140758b3a 885c1010 mov byte ptr [rax + rdx + 0x10], bl
0x140758b3e 49f7d8 neg r8
0x140758b41 496bc868 imul rcx, r8, 0x68
0x140758b45 488d1408 lea rdx, [rax + rcx]
0x140758b49 4180e101 and r9b, 1
0x140758b4d 450fb6c1 movzx r8d, r9b
0x140758b51 4c8b8d10040000 mov r9, qword ptr [rbp + 0x410]
0x140758b58 4d294110 sub qword ptr [r9 + 0x10], r8
0x140758b5c 4c8b8550030000 mov r8, qword ptr [rbp + 0x350]
0x140758b63 4c894408f8 mov qword ptr [rax + rcx - 8], r8
0x140758b68 0f108540030000 movups xmm0, xmmword ptr [rbp + 0x340]
0x140758b6f 0f114408e8 movups xmmword ptr [rax + rcx - 0x18], xmm0
0x140758b74 0f108530030000 movups xmm0, xmmword ptr [rbp + 0x330]
0x140758b7b 0f114408d8 movups xmmword ptr [rax + rcx - 0x28], xmm0
0x140758b80 f30f6f85f0020000 movdqu xmm0, xmmword ptr [rbp + 0x2f0]
0x140758b88 f30f6f8d00030000 movdqu xmm1, xmmword ptr [rbp + 0x300]
0x140758b90 f30f6f9510030000 movdqu xmm2, xmmword ptr [rbp + 0x310]
0x140758b98 f30f6f9d20030000 movdqu xmm3, xmmword ptr [rbp + 0x320]
0x140758ba0 f30f7f5c08c8 movdqu xmmword ptr [rax + rcx - 0x38], xmm3
0x140758ba6 f30f7f5408b8 movdqu xmmword ptr [rax + rcx - 0x48], xmm2
0x140758bac f30f7f4c08a8 movdqu xmmword ptr [rax + rcx - 0x58], xmm1
0x140758bb2 f30f7f440898 movdqu xmmword ptr [rax + rcx - 0x68], xmm0
0x140758bb8 49ff4118 inc qword ptr [r9 + 0x18]
0x140758bbc 4883c2a0 add rdx, -0x60
0x140758bc0 c6853404000000 mov byte ptr [rbp + 0x434], 0
0x140758bc7 488d8df0020000 lea rcx, [rbp + 0x2f0]
0x140758bce e87de38200 call 0x140f86f50
0x140758bd3 0f108540030000 movups xmm0, xmmword ptr [rbp + 0x340]
0x140758bda 0f298580020000 movaps xmmword ptr [rbp + 0x280], xmm0
0x140758be1 0f108530030000 movups xmm0, xmmword ptr [rbp + 0x330]
0x140758be8 0f298570020000 movaps xmmword ptr [rbp + 0x270], xmm0
0x140758bef f30f6f85f0020000 movdqu xmm0, xmmword ptr [rbp + 0x2f0]
0x140758bf7 f30f6f8d00030000 movdqu xmm1, xmmword ptr [rbp + 0x300]
0x140758bff f30f6f9510030000 movdqu xmm2, xmmword ptr [rbp + 0x310]
0x140758c07 f30f6f9d20030000 movdqu xmm3, xmmword ptr [rbp + 0x320]
0x140758c0f 660f7f9d60020000 movdqa xmmword ptr [rbp + 0x260], xmm3
0x140758c17 660f7f9550020000 movdqa xmmword ptr [rbp + 0x250], xmm2
0x140758c1f 660f7f8d40020000 movdqa xmmword ptr [rbp + 0x240], xmm1
0x140758c27 660f7f8530020000 movdqa xmmword ptr [rbp + 0x230], xmm0
0x140758c2f c6853404000001 mov byte ptr [rbp + 0x434], 1
0x140758c36 488d8d30020000 lea rcx, [rbp + 0x230]
0x140758c3d e8bee28200 call 0x140f86f00
0x140758c42 f30f108598030000 movss xmm0, dword ptr [rbp + 0x398]
0x140758c4a f30f1185f0020000 movss dword ptr [rbp + 0x2f0], xmm0
0x140758c52 66440f7e9df4020000 movd dword ptr [rbp + 0x2f4], xmm11
0x140758c5b 66440f7ea5f8020000 movd dword ptr [rbp + 0x2f8], xmm12
0x140758c64 66440f7eadfc020000 movd dword ptr [rbp + 0x2fc], xmm13
0x140758c6d 660f6e859c030000 movd xmm0, dword ptr [rbp + 0x39c]
0x140758c75 660f7e8500030000 movd dword ptr [rbp + 0x300], xmm0
0x140758c7d 4488a504030000 mov byte ptr [rbp + 0x304], r12b
0x140758c84 488b9528040000 mov rdx, qword ptr [rbp + 0x428]
0x140758c8b c6853404000001 mov byte ptr [rbp + 0x434], 1
0x140758c92 488b8dd0020000 mov rcx, qword ptr [rbp + 0x2d0]
0x140758c99 48894c2450 mov qword ptr [rsp + 0x50], rcx
0x140758c9e 488d8df0020000 lea rcx, [rbp + 0x2f0]
0x140758ca5 48894c2448 mov qword ptr [rsp + 0x48], rcx
0x140758caa 488b8df0030000 mov rcx, qword ptr [rbp + 0x3f0]
0x140758cb1 884c2440 mov byte ptr [rsp + 0x40], cl
0x140758cb5 4889442438 mov qword ptr [rsp + 0x38], rax
0x140758cba 488d85b0020000 lea rax, [rbp + 0x2b0]
0x140758cc1 4889442430 mov qword ptr [rsp + 0x30], rax
0x140758cc6 488b85a0030000 mov rax, qword ptr [rbp + 0x3a0]
0x140758ccd 4889442428 mov qword ptr [rsp + 0x28], rax
0x140758cd2 488b85e0030000 mov rax, qword ptr [rbp + 0x3e0]
0x140758cd9 4889442420 mov qword ptr [rsp + 0x20], rax
0x140758cde 488b8d20040000 mov rcx, qword ptr [rbp + 0x420]
0x140758ce5 4c8b8500040000 mov r8, qword ptr [rbp + 0x400]
0x140758cec 4c8b8dd8030000 mov r9, qword ptr [rbp + 0x3d8]
0x140758cf3 e848c8ab00 call 0x141215540
0x140758cf8 488b8518040000 mov rax, qword ptr [rbp + 0x418]
0x140758cff 660f6e48ec movd xmm1, dword ptr [rax - 0x14]
0x140758d04 8378b802 cmp dword ptr [rax - 0x48], 2
0x140758d08 410f95c0 setne r8b
0x140758d0c c6853404000001 mov byte ptr [rbp + 0x434], 1
0x140758d13 488d8d30020000 lea rcx, [rbp + 0x230]
0x140758d1a e8e1e58200 call 0x140f87300
0x140758d1f c6853404000001 mov byte ptr [rbp + 0x434], 1
0x140758d26 488d8d30020000 lea rcx, [rbp + 0x230]
0x140758d2d e87ee18200 call 0x140f86eb0
0x140758d32 4989c6 mov r14, rax
0x140758d35 450f28ce movaps xmm9, xmm14
0x140758d39 450f28f0 movaps xmm14, xmm8
0x140758d3d 450f28c7 movaps xmm8, xmm15
0x140758d41 b001 mov al, 1
0x140758d43 8985c8030000 mov dword ptr [rbp + 0x3c8], eax
0x140758d49 48c7851001000000000000 mov qword ptr [rbp + 0x110], 0
0x140758d54 48c7851801000004000000 mov qword ptr [rbp + 0x118], 4
0x140758d5f 48c7852001000000000000 mov qword ptr [rbp + 0x120], 0
0x140758d6a c6853604000001 mov byte ptr [rbp + 0x436], 1
0x140758d71 4c89f9 mov rcx, r15
0x140758d74 e8f7c81600 call 0x1408c5670
0x140758d79 4885ff test rdi, rdi
0x140758d7c 0f95c1 setne cl
0x140758d7f 84c1 test cl, al
0x140758d81 0f84ed000000 je 0x140758e74
0x140758d87 c6853604000001 mov byte ptr [rbp + 0x436], 1
0x140758d8e 488d4d30 lea rcx, [rbp + 0x30]
0x140758d92 e8d9dc3c00 call 0x140b26a70
0x140758d97 488985f0020000 mov qword ptr [rbp + 0x2f0], rax
0x140758d9e 488995f8020000 mov qword ptr [rbp + 0x2f8], rdx
0x140758da5 488b8518040000 mov rax, qword ptr [rbp + 0x418]
0x140758dac f30f6f8058feffff movdqu xmm0, xmmword ptr [rax - 0x1a8]
0x140758db4 488b9528040000 mov rdx, qword ptr [rbp + 0x428]
0x140758dbb c6853604000001 mov byte ptr [rbp + 0x436], 1
0x140758dc2 488d8510010000 lea rax, [rbp + 0x110]
0x140758dc9 4889442468 mov qword ptr [rsp + 0x68], rax
0x140758dce 4c89742460 mov qword ptr [rsp + 0x60], r14
0x140758dd3 48897c2458 mov qword ptr [rsp + 0x58], rdi
0x140758dd8 488b85d0020000 mov rax, qword ptr [rbp + 0x2d0]
0x140758ddf 4889442450 mov qword ptr [rsp + 0x50], rax
0x140758de4 488b85d8030000 mov rax, qword ptr [rbp + 0x3d8]
0x140758deb 4889442448 mov qword ptr [rsp + 0x48], rax
0x140758df0 488b8500040000 mov rax, qword ptr [rbp + 0x400]
0x140758df7 4889442440 mov qword ptr [rsp + 0x40], rax
0x140758dfc f30f7f442430 movdqu xmmword ptr [rsp + 0x30], xmm0
0x140758e02 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
0x140758e09 4889442428 mov qword ptr [rsp + 0x28], rax
0x140758e0e 488b8520020000 mov rax, qword ptr [rbp + 0x220]
0x140758e15 4889442420 mov qword ptr [rsp + 0x20], rax
0x140758e1a 488b8d20040000 mov rcx, qword ptr [rbp + 0x420]
0x140758e21 4c8b85f0000000 mov r8, qword ptr [rbp + 0xf0]
0x140758e28 4c8d8df0020000 lea r9, [rbp + 0x2f0]
0x140758e2f e8fcffab00 call 0x141218e30
0x140758e34 c6853604000001 mov byte ptr [rbp + 0x436], 1
0x140758e3b 4889f9 mov rcx, rdi
0x140758e3e e86da51800 call 0x1408e33b0
0x140758e43 84c0 test al, al
0x140758e45 742d je 0x140758e74
0x140758e47 488b9528040000 mov rdx, qword ptr [rbp + 0x428]
0x140758e4e c6853604000001 mov byte ptr [rbp + 0x436], 1
0x140758e55 48897c2420 mov qword ptr [rsp + 0x20], rdi
0x140758e5a 488b8d20040000 mov rcx, qword ptr [rbp + 0x420]
0x140758e61 4c8b85a8020000 mov r8, qword ptr [rbp + 0x2a8]
0x140758e68 4c8b8dd0020000 mov r9, qword ptr [rbp + 0x2d0]
0x140758e6f e86cedab00 call 0x141217be0
0x140758e74 488b8510010000 mov rax, qword ptr [rbp + 0x110]
0x140758e7b 488985d0030000 mov qword ptr [rbp + 0x3d0], rax
0x140758e82 488b8518010000 mov rax, qword ptr [rbp + 0x118]
0x140758e89 48898528020000 mov qword ptr [rbp + 0x228], rax
0x140758e90 488bbd20010000 mov rdi, qword ptr [rbp + 0x120]
0x140758e97 488b9d28040000 mov rbx, qword ptr [rbp + 0x428]
0x140758e9e 8b85a0020000 mov eax, dword ptr [rbp + 0x2a0]
0x140758ea4 488b8dd8000000 mov rcx, qword ptr [rbp + 0xd8]
0x140758eab 8901 mov dword ptr [rcx], eax
0x140758ead 488b85f0010000 mov rax, qword ptr [rbp + 0x1f0]
0x140758eb4 488b7030 mov rsi, qword ptr [rax + 0x30]
0x140758eb8 4c8b6840 mov r13, qword ptr [rax + 0x40]
0x140758ebc 483b7020 cmp rsi, qword ptr [rax + 0x20]
0x140758ec0 7513 jne 0x140758ed5
0x140758ec2 488b8d08010000 mov rcx, qword ptr [rbp + 0x108]
0x140758ec9 488d15f8354c02 lea rdx, [rip + 0x24c35f8]
0x140758ed0 e8db9be0ff call 0x140562ab0
0x140758ed5 488b95f0010000 mov rdx, qword ptr [rbp + 0x1f0]
0x140758edc 488b4228 mov rax, qword ptr [rdx + 0x28]
0x140758ee0 488d0cb6 lea rcx, [rsi + rsi*4]
0x140758ee4 4c8b85d0030000 mov r8, qword ptr [rbp + 0x3d0]
0x140758eeb 4c8904c8 mov qword ptr [rax + rcx*8], r8
0x140758eef 4c8b8528020000 mov r8, qword ptr [rbp + 0x228]
0x140758ef6 4c8944c808 mov qword ptr [rax + rcx*8 + 8], r8
0x140758efb 48897cc810 mov qword ptr [rax + rcx*8 + 0x10], rdi
0x140758f00 48895cc818 mov qword ptr [rax + rcx*8 + 0x18], rbx
0x140758f05 4c896cc820 mov qword ptr [rax + rcx*8 + 0x20], r13
0x140758f0a 48ffc6 inc rsi
0x140758f0d 48897230 mov qword ptr [rdx + 0x30], rsi
0x140758f11 48ff4240 inc qword ptr [rdx + 0x40]
0x140758f15 41803f01 cmp byte ptr [r15], 1
0x140758f19 755a jne 0x140758f75
0x140758f1b 488b8518040000 mov rax, qword ptr [rbp + 0x418]
0x140758f22 f30f6f8058feffff movdqu xmm0, xmmword ptr [rax - 0x1a8]
0x140758f2a 488b9528040000 mov rdx, qword ptr [rbp + 0x428]
0x140758f31 c6853604000000 mov byte ptr [rbp + 0x436], 0
0x140758f38 4c89742440 mov qword ptr [rsp + 0x40], r14
0x140758f3d f30f7f442430 movdqu xmmword ptr [rsp + 0x30], xmm0
0x140758f43 488b85d8030000 mov rax, qword ptr [rbp + 0x3d8]
0x140758f4a 4889442428 mov qword ptr [rsp + 0x28], rax
0x140758f4f 488b8500040000 mov rax, qword ptr [rbp + 0x400]
0x140758f56 4889442420 mov qword ptr [rsp + 0x20], rax
0x140758f5b 488b8d20040000 mov rcx, qword ptr [rbp + 0x420]
0x140758f62 4c8d85b0020000 lea r8, [rbp + 0x2b0]
0x140758f69 4c8d8d40010000 lea r9, [rbp + 0x140]
0x140758f70 e87b92ac00 call 0x1412221f0
0x140758f75 488b85f0030000 mov rax, qword ptr [rbp + 0x3f0]
0x140758f7c ffc8 dec eax
0x140758f7e a8fd test al, 0xfd
0x140758f80 7539 jne 0x140758fbb
0x140758f82 488b9528040000 mov rdx, qword ptr [rbp + 0x428]
0x140758f89 c6853604000000 mov byte ptr [rbp + 0x436], 0
0x140758f90 4c89742428 mov qword ptr [rsp + 0x28], r14
0x140758f95 488b85d0020000 mov rax, qword ptr [rbp + 0x2d0]
0x140758f9c 4889442420 mov qword ptr [rsp + 0x20], rax
0x140758fa1 488b8d20040000 mov rcx, qword ptr [rbp + 0x420]
0x140758fa8 4c8d85b0020000 lea r8, [rbp + 0x2b0]
0x140758faf 4c8b8d20020000 mov r9, qword ptr [rbp + 0x220]
0x140758fb6 e855adac00 call 0x141223d10
0x140758fbb 488b8d28040000 mov rcx, qword ptr [rbp + 0x428]
0x140758fc2 c6853604000000 mov byte ptr [rbp + 0x436], 0
0x140758fc9 e8c2f70f00 call 0x140858790
0x140758fce 89c7 mov edi, eax
0x140758fd0 488b8518040000 mov rax, qword ptr [rbp + 0x418]
0x140758fd7 488b9858feffff mov rbx, qword ptr [rax - 0x1a8]
0x140758fde 4c8bb860feffff mov r15, qword ptr [rax - 0x1a0]
0x140758fe5 660feff6 pxor xmm6, xmm6
0x140758fe9 4180fc02 cmp r12b, 2
0x140758fed 7567 jne 0x140759056
0x140758fef 488d48f5 lea rcx, [rax - 0xb]
0x140758ff3 c6853604000000 mov byte ptr [rbp + 0x436], 0
0x140758ffa e831a20d00 call 0x140833230
0x140758fff 660f6e356d383202 movd xmm6, dword ptr [rip + 0x232386d]
0x140759007 84c0 test al, al
0x140759009 488b8518040000 mov rax, qword ptr [rbp + 0x418]
0x140759010 7444 je 0x140759056
0x140759012 f30f1040ec movss xmm0, dword ptr [rax - 0x14]
0x140759017 f30f580559383202 addss xmm0, dword ptr [rip + 0x2323859]
0x14075901f f30f5e0555383202 divss xmm0, dword ptr [rip + 0x2323855]
0x140759027 660fefc9 pxor xmm1, xmm1
0x14075902b f30f5fc8 maxss xmm1, xmm0
0x14075902f f30f1005cdc81c02 movss xmm0, dword ptr [rip + 0x21cc8cd]
0x140759037 f30f5dc1 minss xmm0, xmm1
0x14075903b 0f28c8 movaps xmm1, xmm0
0x14075903e f30f59c8 mulss xmm1, xmm0
0x140759042 f30f58c0 addss xmm0, xmm0
0x140759046 f30f10352a402702 movss xmm6, dword ptr [rip + 0x227402a]
0x14075904e f30f5cf0 subss xmm6, xmm0
0x140759052 f30f59f1 mulss xmm6, xmm1
0x140759056 f30f1078d4 movss xmm7, dword ptr [rax - 0x2c]
0x14075905b f30f108598030000 movss xmm0, dword ptr [rbp + 0x398]
0x140759063 f30f1185f0020000 movss dword ptr [rbp + 0x2f0], xmm0
0x14075906b 66440f7e9df4020000 movd dword ptr [rbp + 0x2f4], xmm11
0x140759074 66440f7ea5f8020000 movd dword ptr [rbp + 0x2f8], xmm12
0x14075907d 66440f7eadfc020000 movd dword ptr [rbp + 0x2fc], xmm13
0x140759086 f30f10859c030000 movss xmm0, dword ptr [rbp + 0x39c]
0x14075908e f30f118500030000 movss dword ptr [rbp + 0x300], xmm0
0x140759096 4488a504030000 mov byte ptr [rbp + 0x304], r12b
0x14075909d 0fb6457a movzx eax, byte ptr [rbp + 0x7a]
0x1407590a1 488d8df8020000 lea rcx, [rbp + 0x2f8]
0x1407590a8 88410f mov byte ptr [rcx + 0xf], al
0x1407590ab 0fb74578 movzx eax, word ptr [rbp + 0x78]
0x1407590af 6689410d mov word ptr [rcx + 0xd], ax
0x1407590b3 c6853604000000 mov byte ptr [rbp + 0x436], 0
0x1407590ba 488b8d20040000 mov rcx, qword ptr [rbp + 0x420]
0x1407590c1 e83a153d00 call 0x140b2a600
0x1407590c6 31f6 xor esi, esi
0x1407590c8 4885c0 test rax, rax
0x1407590cb 7453 je 0x140759120
0x1407590cd f60001 test byte ptr [rax], 1
0x1407590d0 754e jne 0x140759120
0x1407590d2 660f6e500c movd xmm2, dword ptr [rax + 0xc]
0x1407590d7 c6853604000000 mov byte ptr [rbp + 0x436], 0
0x1407590de f30f10059a373202 movss xmm0, dword ptr [rip + 0x232379a]
0x1407590e6 f30f100d96373202 movss xmm1, dword ptr [rip + 0x2323796]
0x1407590ee e8ad735300 call 0x140c904a0
0x1407590f3 0f57c9 xorps xmm1, xmm1
0x1407590f6 f30f5fc8 maxss xmm1, xmm0
0x1407590fa f3440f103d01c81c02 movss xmm15, dword ptr [rip + 0x21cc801]
0x140759103 410f28c7 movaps xmm0, xmm15
0x140759107 f30f5dc1 minss xmm0, xmm1
0x14075910b f3440f5cf8 subss xmm15, xmm0
0x140759110 f3440f593d6f373202 mulss xmm15, dword ptr [rip + 0x232376f]
0x140759119 be01000000 mov esi, 1
0x14075911e eb00 jmp 0x140759120
0x140759120 c6853604000000 mov byte ptr [rbp + 0x436], 0
0x140759127 488b8d20040000 mov rcx, qword ptr [rbp + 0x420]
0x14075912e e8cd143d00 call 0x140b2a600
0x140759133 31c9 xor ecx, ecx
0x140759135 4885c0 test rax, rax
0x140759138 745a je 0x140759194
0x14075913a f60001 test byte ptr [rax], 1
0x14075913d 488b9500040000 mov rdx, qword ptr [rbp + 0x400]
0x140759144 7555 jne 0x14075919b
0x140759146 660f6e500c movd xmm2, dword ptr [rax + 0xc]
0x14075914b c6853604000000 mov byte ptr [rbp + 0x436], 0
0x140759152 f30f100526373202 movss xmm0, dword ptr [rip + 0x2323726]
0x14075915a f30f100d163f2702 movss xmm1, dword ptr [rip + 0x2273f16]
0x140759162 e839735300 call 0x140c904a0
0x140759167 0f57c9 xorps xmm1, xmm1
0x14075916a f30f5fc8 maxss xmm1, xmm0
0x14075916e f30f10058ec71c02 movss xmm0, dword ptr [rip + 0x21cc78e]
0x140759176 0f28d0 movaps xmm2, xmm0
0x140759179 f30f5dd1 minss xmm2, xmm1
0x14075917d f30f5cc2 subss xmm0, xmm2
0x140759181 f30f590503373202 mulss xmm0, dword ptr [rip + 0x2323703]
0x140759189 f30f58c2 addss xmm0, xmm2
0x14075918d b901000000 mov ecx, 1
0x140759192 eb00 jmp 0x140759194
0x140759194 488b9500040000 mov rdx, qword ptr [rbp + 0x400]
0x14075919b 488b8518040000 mov rax, qword ptr [rbp + 0x418]
0x1407591a2 660f6e48e4 movd xmm1, dword ptr [rax - 0x1c]
0x1407591a7 c6853604000000 mov byte ptr [rbp + 0x436], 0
0x1407591ae 898c2490000000 mov dword ptr [rsp + 0x90], ecx
0x1407591b5 89b42480000000 mov dword ptr [rsp + 0x80], esi
0x1407591bc 4c89742458 mov qword ptr [rsp + 0x58], r14
0x1407591c1 488b85e8000000 mov rax, qword ptr [rbp + 0xe8]
0x1407591c8 4889442450 mov qword ptr [rsp + 0x50], rax
0x1407591cd 488b85f0030000 mov rax, qword ptr [rbp + 0x3f0]
0x1407591d4 88442448 mov byte ptr [rsp + 0x48], al
0x1407591d8 4c897c2440 mov qword ptr [rsp + 0x40], r15
0x1407591dd 660f7e8c24a0000000 movd dword ptr [rsp + 0xa0], xmm1
0x1407591e6 f30f11842498000000 movss dword ptr [rsp + 0x98], xmm0
0x1407591ef f3440f11bc2488000000 movss dword ptr [rsp + 0x88], xmm15
0x1407591f9 488d85f0020000 lea rax, [rbp + 0x2f0]
0x140759200 4889442478 mov qword ptr [rsp + 0x78], rax
0x140759205 f30f117c2470 movss dword ptr [rsp + 0x70], xmm7
0x14075920b 488d85b0020000 lea rax, [rbp + 0x2b0]
0x140759212 4889442468 mov qword ptr [rsp + 0x68], rax
0x140759217 660f7e742460 movd dword ptr [rsp + 0x60], xmm6
0x14075921d 48895c2438 mov qword ptr [rsp + 0x38], rbx
0x140759222 488b85d8030000 mov rax, qword ptr [rbp + 0x3d8]
0x140759229 4889442430 mov qword ptr [rsp + 0x30], rax
0x14075922e 4889542428 mov qword ptr [rsp + 0x28], rdx
0x140759233 488b85a0030000 mov rax, qword ptr [rbp + 0x3a0]
0x14075923a 4889442420 mov qword ptr [rsp + 0x20], rax
0x14075923f c68424a800000000 mov byte ptr [rsp + 0xa8], 0
0x140759247 488d8db0010000 lea rcx, [rbp + 0x1b0]
0x14075924e 488d9540010000 lea rdx, [rbp + 0x140]
0x140759255 4189f8 mov r8d, edi
0x140759258 4c8b8de0030000 mov r9, qword ptr [rbp + 0x3e0]
0x14075925f e8fc040100 call 0x140769760
0x140759264 80bdc803000000 cmp byte ptr [rbp + 0x3c8], 0
0x14075926b 4c8ba5f8030000 mov r12, qword ptr [rbp + 0x3f8]
0x140759272 0f57ff xorps xmm7, xmm7
0x140759275 488bb5d8030000 mov rsi, qword ptr [rbp + 0x3d8]
0x14075927c 745c je 0x1407592da
0x14075927e f6853002000001 test byte ptr [rbp + 0x230], 1
0x140759285 0f841c080000 je 0x140759aa7
0x14075928b 488b8538020000 mov rax, qword ptr [rbp + 0x238]
0x140759292 4885c0 test rax, rax
0x140759295 741a je 0x1407592b1
0x140759297 488b8d40020000 mov rcx, qword ptr [rbp + 0x240]
0x14075929e 48c1e002 shl rax, 2
0x1407592a2 488d1480 lea rdx, [rax + rax*4]
0x1407592a6 41b804000000 mov r8d, 4
0x1407592ac e86fa9ebff call 0x140613c20
0x1407592b1 488b8558020000 mov rax, qword ptr [rbp + 0x258]
0x1407592b8 488d8d60020000 lea rcx, [rbp + 0x260]
0x1407592bf 4885c0 test rax, rax
0x1407592c2 7416 je 0x1407592da
0x1407592c4 488b09 mov rcx, qword ptr [rcx]
0x1407592c7 48c1e002 shl rax, 2
0x1407592cb 488d1480 lea rdx, [rax + rax*4]
0x1407592cf 41b804000000 mov r8d, 4
0x1407592d5 e846a9ebff call 0x140613c20
0x1407592da 488b9540010000 mov rdx, qword ptr [rbp + 0x140]
0x1407592e1 4885d2 test rdx, rdx
0x1407592e4 450f28f8 movaps xmm15, xmm8
0x1407592e8 7416 je 0x140759300
0x1407592ea 488b8d48010000 mov rcx, qword ptr [rbp + 0x148]
0x1407592f1 48c1e203 shl rdx, 3
0x1407592f5 41b804000000 mov r8d, 4
0x1407592fb e820a9ebff call 0x140613c20
0x140759300 488b9558010000 mov rdx, qword ptr [rbp + 0x158]
0x140759307 4885d2 test rdx, rdx
0x14075930a 450f28c6 movaps xmm8, xmm14
0x14075930e 7416 je 0x140759326
0x140759310 488b8d60010000 mov rcx, qword ptr [rbp + 0x160]
0x140759317 48c1e202 shl rdx, 2
0x14075931b 41b804000000 mov r8d, 4
0x140759321 e8faa8ebff call 0x140613c20
0x140759326 488b95e0020000 mov rdx, qword ptr [rbp + 0x2e0]
0x14075932d 4885d2 test rdx, rdx
0x140759330 660f76f6 pcmpeqd xmm6, xmm6
0x140759334 450f28f1 movaps xmm14, xmm9
0x140759338 7416 je 0x140759350
0x14075933a 48c1e202 shl rdx, 2
0x14075933e 41b804000000 mov r8d, 4
0x140759344 488b8de0030000 mov rcx, qword ptr [rbp + 0x3e0]
0x14075934b e8d0a8ebff call 0x140613c20
0x140759350 4885f6 test rsi, rsi
0x140759353 7419 je 0x14075936e
0x140759355 48c1e602 shl rsi, 2
0x140759359 41b804000000 mov r8d, 4
0x14075935f 488b8d00040000 mov rcx, qword ptr [rbp + 0x400]
0x140759366 4889f2 mov rdx, rsi
0x140759369 e8b2a8ebff call 0x140613c20
0x14075936e 488b9528040000 mov rdx, qword ptr [rbp + 0x428]
0x140759375 c6853704000001 mov byte ptr [rbp + 0x437], 1
0x14075937c 488b8dc8010000 mov rcx, qword ptr [rbp + 0x1c8]
0x140759383 e8788a3300 call 0x140a91e00
0x140759388 4889c2 mov rdx, rax
0x14075938b 48c1ea20 shr rdx, 0x20
0x14075938f 0f850b010000 jne 0x1407594a0
0x140759395 48c7854001000000000000 mov qword ptr [rbp + 0x140], 0
0x1407593a0 48c7854801000004000000 mov qword ptr [rbp + 0x148], 4
0x1407593ab 488d8550010000 lea rax, [rbp + 0x150]
0x1407593b2 0f1138 movups xmmword ptr [rax], xmm7
0x1407593b5 48c7856001000004000000 mov qword ptr [rbp + 0x160], 4
0x1407593c0 0f117818 movups xmmword ptr [rax + 0x18], xmm7
0x1407593c4 48c7857801000004000000 mov qword ptr [rbp + 0x178], 4
0x1407593cf 0f117830 movups xmmword ptr [rax + 0x30], xmm7
0x1407593d3 48c7859001000004000000 mov qword ptr [rbp + 0x190], 4
0x1407593de 48c7859801000000000000 mov qword ptr [rbp + 0x198], 0
0x1407593e9 c685a001000000 mov byte ptr [rbp + 0x1a0], 0
0x1407593f0 c6853504000001 mov byte ptr [rbp + 0x435], 1
0x1407593f7 488d8d40010000 lea rcx, [rbp + 0x140]
0x1407593fe 488b9520040000 mov rdx, qword ptr [rbp + 0x420]
0x140759405 e8c60a1900 call 0x1408e9ed0
0x14075940a 4c8bb5e8010000 mov r14, qword ptr [rbp + 0x1e8]
0x140759411 4c8d3d8d154c02 lea r15, [rip + 0x24c158d]
0x140759418 8b8588030000 mov eax, dword ptr [rbp + 0x388]
0x14075941e 488b8de0010000 mov rcx, qword ptr [rbp + 0x1e0]
0x140759425 8901 mov dword ptr [rcx], eax
0x140759427 488b9528040000 mov rdx, qword ptr [rbp + 0x428]
0x14075942e c6853504000001 mov byte ptr [rbp + 0x435], 1
0x140759435 488b8dd8010000 mov rcx, qword ptr [rbp + 0x1d8]
0x14075943c e87fc5ac00 call 0x1412259c0
0x140759441 4889c7 mov rdi, rax
0x140759444 488b9528040000 mov rdx, qword ptr [rbp + 0x428]
0x14075944b c6853504000001 mov byte ptr [rbp + 0x435], 1
0x140759452 488d8df0020000 lea rcx, [rbp + 0x2f0]
0x140759459 e8b2301600 call 0x1408bc510
0x14075945e f685f002000001 test byte ptr [rbp + 0x2f0], 1
0x140759465 0f84f8010000 je 0x140759663
0x14075946b 488b85f8020000 mov rax, qword ptr [rbp + 0x2f8]
0x140759472 48898530020000 mov qword ptr [rbp + 0x230], rax
0x140759479 c6853504000001 mov byte ptr [rbp + 0x435], 1
0x140759480 488d8d30020000 lea rcx, [rbp + 0x230]
0x140759487 e844f192ff call 0x1400885d0
0x14075948c 0d00000001 or eax, 0x1000000
0x140759491 e9d3010000 jmp 0x140759669
0x140759496 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1407594a0 89c1 mov ecx, eax
0x1407594a2 488bbdd0000000 mov rdi, qword ptr [rbp + 0xd0]
0x1407594a9 483b4f10 cmp rcx, qword ptr [rdi + 0x10]
0x1407594ad 4c8b9dc8000000 mov r11, qword ptr [rbp + 0xc8]
0x1407594b4 0f8391090000 jae 0x140759e4b
0x1407594ba 488b5708 mov rdx, qword ptr [rdi + 8]
0x1407594be 48c1e820 shr rax, 0x20
0x1407594c2 4c8d0c89 lea r9, [rcx + rcx*4]
0x1407594c6 41b801000000 mov r8d, 1
0x1407594cc 4239048a cmp dword ptr [rdx + r9*4], eax
0x1407594d0 0f8514090000 jne 0x140759dea
0x1407594d6 4e8d0c8a lea r9, [rdx + r9*4]
0x1407594da 418b5104 mov edx, dword ptr [r9 + 4]
0x1407594de 41baffffffff mov r10d, 0xffffffff
0x1407594e4 4c39d2 cmp rdx, r10
0x1407594e7 0f84fd080000 je 0x140759dea
0x1407594ed 49395338 cmp qword ptr [r11 + 0x38], rdx
0x1407594f1 0f86e9080000 jbe 0x140759de0
0x1407594f7 4d8b4328 mov r8, qword ptr [r11 + 0x28]
0x1407594fb 4189d2 mov r10d, edx
0x1407594fe 41c1ea06 shr r10d, 6
0x140759502 4f8b04d0 mov r8, qword ptr [r8 + r10*8]
0x140759506 490fa3d0 bt r8, rdx
0x14075950a 0f83d0080000 jae 0x140759de0
0x140759510 418b410c mov eax, dword ptr [r9 + 0xc]
0x140759514 418b7110 mov esi, dword ptr [r9 + 0x10]
0x140759518 488b8fa8010000 mov rcx, qword ptr [rdi + 0x1a8]
0x14075951f 488d04c0 lea rax, [rax + rax*8]
0x140759523 498b9310010000 mov rdx, qword ptr [r11 + 0x110]
0x14075952a 4d8b8318010000 mov r8, qword ptr [r11 + 0x118]
0x140759531 4c8b4cc118 mov r9, qword ptr [rcx + rax*8 + 0x18]
0x140759536 488b44c138 mov rax, qword ptr [rcx + rax*8 + 0x38]
0x14075953b 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x14075953f 48f7d1 not rcx
0x140759542 488d1449 lea rdx, [rcx + rcx*2]
0x140759546 48c1e204 shl rdx, 4
0x14075954a 4d8b541128 mov r10, qword ptr [r9 + rdx + 0x28]
0x14075954f 4a8b04c0 mov rax, qword ptr [rax + r8*8]
0x140759553 48f7d0 not rax
0x140759556 488d0440 lea rax, [rax + rax*2]
0x14075955a 48c1e004 shl rax, 4
0x14075955e 498b5c0128 mov rbx, qword ptr [r9 + rax + 0x28]
0x140759563 488d0c76 lea rcx, [rsi + rsi*2]
0x140759567 48c1e104 shl rcx, 4
0x14075956b 49034c1110 add rcx, qword ptr [r9 + rdx + 0x10]
0x140759570 4c8d04b500000000 lea r8, [rsi*4]
0x140759578 4d03441120 add r8, qword ptr [r9 + rdx + 0x20]
0x14075957d 4c8985a8030000 mov qword ptr [rbp + 0x3a8], r8
0x140759584 486bfe68 imul rdi, rsi, 0x68
0x140759588 49037c0110 add rdi, qword ptr [r9 + rax + 0x10]
0x14075958d 8b858c030000 mov eax, dword ptr [rbp + 0x38c]
0x140759593 418904b2 mov dword ptr [r10 + rsi*4], eax
0x140759597 488b85c0020000 mov rax, qword ptr [rbp + 0x2c0]
0x14075959e 48898500030000 mov qword ptr [rbp + 0x300], rax
0x1407595a5 0f1085b0020000 movups xmm0, xmmword ptr [rbp + 0x2b0]
0x1407595ac 0f2985f0020000 movaps xmmword ptr [rbp + 0x2f0], xmm0
0x1407595b3 c6853704000000 mov byte ptr [rbp + 0x437], 0
0x1407595ba 488d95f0020000 lea rdx, [rbp + 0x2f0]
0x1407595c1 e86af20f00 call 0x140858830
0x1407595c6 8b858c030000 mov eax, dword ptr [rbp + 0x38c]
0x1407595cc 8904b3 mov dword ptr [rbx + rsi*4], eax
0x1407595cf c6853704000000 mov byte ptr [rbp + 0x437], 0
0x1407595d6 4889f9 mov rcx, rdi
0x1407595d9 488b9520040000 mov rdx, qword ptr [rbp + 0x420]
0x1407595e0 e8eb081900 call 0x1408e9ed0
0x1407595e5 8b8588030000 mov eax, dword ptr [rbp + 0x388]
0x1407595eb 488b8de0010000 mov rcx, qword ptr [rbp + 0x1e0]
0x1407595f2 8901 mov dword ptr [rcx], eax
0x1407595f4 488b9528040000 mov rdx, qword ptr [rbp + 0x428]
0x1407595fb c6853704000000 mov byte ptr [rbp + 0x437], 0
0x140759602 488b8dd8010000 mov rcx, qword ptr [rbp + 0x1d8]
0x140759609 e8b2c3ac00 call 0x1412259c0
0x14075960e 4889c3 mov rbx, rax
0x140759611 488b9528040000 mov rdx, qword ptr [rbp + 0x428]
0x140759618 c6853704000000 mov byte ptr [rbp + 0x437], 0
0x14075961f 488d8df0020000 lea rcx, [rbp + 0x2f0]
0x140759626 e8e52e1600 call 0x1408bc510
0x14075962b f685f002000001 test byte ptr [rbp + 0x2f0], 1
0x140759632 0f8488030000 je 0x1407599c0
0x140759638 488b85f8020000 mov rax, qword ptr [rbp + 0x2f8]
0x14075963f 48898530020000 mov qword ptr [rbp + 0x230], rax
0x140759646 c6853704000000 mov byte ptr [rbp + 0x437], 0
0x14075964d 488d8d30020000 lea rcx, [rbp + 0x230]
0x140759654 e877ef92ff call 0x1400885d0
0x140759659 0d00000001 or eax, 0x1000000
0x14075965e e963030000 jmp 0x1407599c6
0x140759663 8b85f4020000 mov eax, dword ptr [rbp + 0x2f4]
0x140759669 894758 mov dword ptr [rdi + 0x58], eax
0x14075966c 48c7475000000000 mov qword ptr [rdi + 0x50], 0
0x140759674 c6853504000001 mov byte ptr [rbp + 0x435], 1
0x14075967b 488d8d40010000 lea rcx, [rbp + 0x140]
0x140759682 e859ea1800 call 0x1408e80e0
0x140759687 4883c740 add rdi, 0x40
0x14075968b 488d0cd2 lea rcx, [rdx + rdx*8]
0x14075968f 488d0c88 lea rcx, [rax + rcx*4]
0x140759693 48c785f002000000000000 mov qword ptr [rbp + 0x2f0], 0
0x14075969e 48c7853003000000000000 mov qword ptr [rbp + 0x330], 0
0x1407596a9 48898570030000 mov qword ptr [rbp + 0x370], rax
0x1407596b0 48898d78030000 mov qword ptr [rbp + 0x378], rcx
0x1407596b7 c6853504000001 mov byte ptr [rbp + 0x435], 1
0x1407596be 4889f9 mov rcx, rdi
0x1407596c1 488d95f0020000 lea rdx, [rbp + 0x2f0]
0x1407596c8 4c8d05b9353202 lea r8, [rip + 0x23235b9]
0x1407596cf e89c87aa00 call 0x141201e70
0x1407596d4 8b8590030000 mov eax, dword ptr [rbp + 0x390]
0x1407596da 488b8df8010000 mov rcx, qword ptr [rbp + 0x1f8]
0x1407596e1 8901 mov dword ptr [rcx], eax
0x1407596e3 488b85e8030000 mov rax, qword ptr [rbp + 0x3e8]
0x1407596ea 488b7030 mov rsi, qword ptr [rax + 0x30]
0x1407596ee 488b7840 mov rdi, qword ptr [rax + 0x40]
0x1407596f2 483b7020 cmp rsi, qword ptr [rax + 0x20]
0x1407596f6 751a jne 0x140759712
0x1407596f8 c6853504000001 mov byte ptr [rbp + 0x435], 1
0x1407596ff 488b8d18020000 mov rcx, qword ptr [rbp + 0x218]
0x140759706 488d151b3b4c02 lea rdx, [rip + 0x24c3b1b]
0x14075970d e85e91e0ff call 0x140562870
0x140759712 488b8de8030000 mov rcx, qword ptr [rbp + 0x3e8]
0x140759719 488b4128 mov rax, qword ptr [rcx + 0x28]
0x14075971d 48893cf0 mov qword ptr [rax + rsi*8], rdi
0x140759721 48ffc6 inc rsi
0x140759724 48897130 mov qword ptr [rcx + 0x30], rsi
0x140759728 48ff4140 inc qword ptr [rcx + 0x40]
0x14075972c c6853504000001 mov byte ptr [rbp + 0x435], 1
0x140759733 488b8da8020000 mov rcx, qword ptr [rbp + 0x2a8]
0x14075973a 488d1537114c02 lea rdx, [rip + 0x24c1137]
0x140759741 e8fa04ab00 call 0x141209c40
0x140759746 4889c3 mov rbx, rax
0x140759749 4885c0 test rax, rax
0x14075974c 0f8476060000 je 0x140759dc8
0x140759752 4c89bd30020000 mov qword ptr [rbp + 0x230], r15
0x140759759 48c785380200003a000000 mov qword ptr [rbp + 0x238], 0x3a
0x140759764 4889df mov rdi, rbx
0x140759767 4883c710 add rdi, 0x10
0x14075976b 48837b183a cmp qword ptr [rbx + 0x18], 0x3a
0x140759770 0f850b060000 jne 0x140759d81
0x140759776 488b17 mov rdx, qword ptr [rdi]
0x140759779 41b83a000000 mov r8d, 0x3a
0x14075977f 4c89f9 mov rcx, r15
0x140759782 e869a41c02 call 0x142923bf0
0x140759787 85c0 test eax, eax
0x140759789 0f85f2050000 jne 0x140759d81
0x14075978f 488b0b mov rcx, qword ptr [rbx]
0x140759792 488b4308 mov rax, qword ptr [rbx + 8]
0x140759796 488b4018 mov rax, qword ptr [rax + 0x18]
0x14075979a c6853504000001 mov byte ptr [rbp + 0x435], 1
0x1407597a1 ffd0 call rax
0x1407597a3 4889c7 mov rdi, rax
0x1407597a6 4885c0 test rax, rax
0x1407597a9 0f8419060000 je 0x140759dc8
0x1407597af 488b85a0010000 mov rax, qword ptr [rbp + 0x1a0]
0x1407597b6 48898590020000 mov qword ptr [rbp + 0x290], rax
0x1407597bd 0f108590010000 movups xmm0, xmmword ptr [rbp + 0x190]
0x1407597c4 0f298580020000 movaps xmmword ptr [rbp + 0x280], xmm0
0x1407597cb 0f108580010000 movups xmm0, xmmword ptr [rbp + 0x180]
0x1407597d2 0f298570020000 movaps xmmword ptr [rbp + 0x270], xmm0
0x1407597d9 0f108540010000 movups xmm0, xmmword ptr [rbp + 0x140]
0x1407597e0 f30f6f8d50010000 movdqu xmm1, xmmword ptr [rbp + 0x150]
0x1407597e8 f30f6f9560010000 movdqu xmm2, xmmword ptr [rbp + 0x160]
0x1407597f0 f30f6f9d70010000 movdqu xmm3, xmmword ptr [rbp + 0x170]
0x1407597f8 660f7f9d60020000 movdqa xmmword ptr [rbp + 0x260], xmm3
0x140759800 660f7f9550020000 movdqa xmmword ptr [rbp + 0x250], xmm2
0x140759808 660f7f8d40020000 movdqa xmmword ptr [rbp + 0x240], xmm1
0x140759810 0f298530020000 movaps xmmword ptr [rbp + 0x230], xmm0
0x140759817 488b85c0020000 mov rax, qword ptr [rbp + 0x2c0]
0x14075981e 488985b0000000 mov qword ptr [rbp + 0xb0], rax
0x140759825 0f1085b0020000 movups xmm0, xmmword ptr [rbp + 0x2b0]
0x14075982c 0f2985a0000000 movaps xmmword ptr [rbp + 0xa0], xmm0
0x140759833 0fb605cfb3ed02 movzx eax, byte ptr [rip + 0x2edb3cf]
0x14075983a b920000000 mov ecx, 0x20
0x14075983f ba08000000 mov edx, 8
0x140759844 e8c7a3ebff call 0x140613c10
0x140759849 4885c0 test rax, rax
0x14075984c 0f84b6060000 je 0x140759f08
0x140759852 4889c1 mov rcx, rax
0x140759855 48c70000000000 mov qword ptr [rax], 0
0x14075985c 48c7400804000000 mov qword ptr [rax + 8], 4
0x140759864 48c7401000000000 mov qword ptr [rax + 0x10], 0
0x14075986c c6401800 mov byte ptr [rax + 0x18], 0
0x140759870 4c8b4f60 mov r9, qword ptr [rdi + 0x60]
0x140759874 eb0c jmp 0x140759882
0x140759876 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140759880 f390 pause 
0x140759882 498b4108 mov rax, qword ptr [r9 + 8]
0x140759886 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140759890 4883f8ff cmp rax, -1
0x140759894 74ea je 0x140759880
0x140759896 4885c0 test rax, rax
0x140759899 0f8835040000 js 0x140759cd4
0x14075989f 488d5001 lea rdx, [rax + 1]
0x1407598a3 f0490fb15108 lock cmpxchg qword ptr [r9 + 8], rdx
0x1407598a9 75e5 jne 0x140759890
0x1407598ab 488b9528040000 mov rdx, qword ptr [rbp + 0x428]
0x1407598b2 c6853004000001 mov byte ptr [rbp + 0x430], 1
0x1407598b9 48894c2420 mov qword ptr [rsp + 0x20], rcx
0x1407598be 488d8d10010000 lea rcx, [rbp + 0x110]
0x1407598c5 4c8d85a0000000 lea r8, [rbp + 0xa0]
0x1407598cc e82fef0f00 call 0x140858800
0x1407598d1 488b8590020000 mov rax, qword ptr [rbp + 0x290]
0x1407598d8 488d8df8020000 lea rcx, [rbp + 0x2f8]
0x1407598df 48898188000000 mov qword ptr [rcx + 0x88], rax
0x1407598e6 0f288580020000 movaps xmm0, xmmword ptr [rbp + 0x280]
0x1407598ed 0f114178 movups xmmword ptr [rcx + 0x78], xmm0
0x1407598f1 0f288570020000 movaps xmm0, xmmword ptr [rbp + 0x270]
0x1407598f8 0f114168 movups xmmword ptr [rcx + 0x68], xmm0
0x1407598fc 0f288530020000 movaps xmm0, xmmword ptr [rbp + 0x230]
0x140759903 0f288d40020000 movaps xmm1, xmmword ptr [rbp + 0x240]
0x14075990a 0f289550020000 movaps xmm2, xmmword ptr [rbp + 0x250]
0x140759911 660f6f9d60020000 movdqa xmm3, xmmword ptr [rbp + 0x260]
0x140759919 f30f7f5958 movdqu xmmword ptr [rcx + 0x58], xmm3
0x14075991e 0f115148 movups xmmword ptr [rcx + 0x48], xmm2
0x140759922 0f114938 movups xmmword ptr [rcx + 0x38], xmm1
0x140759926 0f114128 movups xmmword ptr [rcx + 0x28], xmm0
0x14075992a 0f108510010000 movups xmm0, xmmword ptr [rbp + 0x110]
0x140759931 f30f6f8d20010000 movdqu xmm1, xmmword ptr [rbp + 0x120]
0x140759939 f30f6f9530010000 movdqu xmm2, xmmword ptr [rbp + 0x130]
0x140759941 0f2985f0020000 movaps xmmword ptr [rbp + 0x2f0], xmm0
0x140759948 660f7f8d00030000 movdqa xmmword ptr [rbp + 0x300], xmm1
0x140759950 660f7f9510030000 movdqa xmmword ptr [rbp + 0x310], xmm2
0x140759958 c6853004000000 mov byte ptr [rbp + 0x430], 0
0x14075995f 488d4d78 lea rcx, [rbp + 0x78]
0x140759963 488b9570050000 mov rdx, qword ptr [rbp + 0x570]
0x14075996a 4c8d85f0020000 lea r8, [rbp + 0x2f0]
0x140759971 4c8d0db8333202 lea r9, [rip + 0x23233b8]
0x140759978 e8d35bab00 call 0x14120f550
0x14075997d 4c8b8598000000 mov r8, qword ptr [rbp + 0x98]
0x140759984 488b85e0000000 mov rax, qword ptr [rbp + 0xe0]
0x14075998b 8b8da4020000 mov ecx, dword ptr [rbp + 0x2a4]
0x140759991 8908 mov dword ptr [rax], ecx
0x140759993 488b9528040000 mov rdx, qword ptr [rbp + 0x428]
0x14075999a c6853504000000 mov byte ptr [rbp + 0x435], 0
0x1407599a1 488b8dc8010000 mov rcx, qword ptr [rbp + 0x1c8]
0x1407599a8 e8c32f7900 call 0x140eec970
0x1407599ad 488b9d00020000 mov rbx, qword ptr [rbp + 0x200]
0x1407599b4 4c8bbdd0010000 mov r15, qword ptr [rbp + 0x1d0]
0x1407599bb e97de6ffff jmp 0x14075803d
0x1407599c0 8b85f4020000 mov eax, dword ptr [rbp + 0x2f4]
0x1407599c6 894358 mov dword ptr [rbx + 0x58], eax
0x1407599c9 48c7435000000000 mov qword ptr [rbx + 0x50], 0
0x1407599d1 c6853704000000 mov byte ptr [rbp + 0x437], 0
0x1407599d8 4889f9 mov rcx, rdi
0x1407599db e800e71800 call 0x1408e80e0
0x1407599e0 4883c340 add rbx, 0x40
0x1407599e4 488d0cd2 lea rcx, [rdx + rdx*8]
0x1407599e8 488d0c88 lea rcx, [rax + rcx*4]
0x1407599ec 48c785f002000000000000 mov qword ptr [rbp + 0x2f0], 0
0x1407599f7 48c7853003000000000000 mov qword ptr [rbp + 0x330], 0
0x140759a02 48898570030000 mov qword ptr [rbp + 0x370], rax
0x140759a09 48898d78030000 mov qword ptr [rbp + 0x378], rcx
0x140759a10 c6853704000000 mov byte ptr [rbp + 0x437], 0
0x140759a17 4889d9 mov rcx, rbx
0x140759a1a 488d95f0020000 lea rdx, [rbp + 0x2f0]
0x140759a21 4c8d0560323202 lea r8, [rip + 0x2323260]
0x140759a28 e84384aa00 call 0x141201e70
0x140759a2d 8b8590030000 mov eax, dword ptr [rbp + 0x390]
0x140759a33 488b8df8010000 mov rcx, qword ptr [rbp + 0x1f8]
0x140759a3a 8901 mov dword ptr [rcx], eax
0x140759a3c 488b85e8030000 mov rax, qword ptr [rbp + 0x3e8]
0x140759a43 488b7030 mov rsi, qword ptr [rax + 0x30]
0x140759a47 488b7840 mov rdi, qword ptr [rax + 0x40]
0x140759a4b 483b7020 cmp rsi, qword ptr [rax + 0x20]
0x140759a4f 751a jne 0x140759a6b
0x140759a51 c6853704000000 mov byte ptr [rbp + 0x437], 0
0x140759a58 488b8d18020000 mov rcx, qword ptr [rbp + 0x218]
0x140759a5f 488d15c2374c02 lea rdx, [rip + 0x24c37c2]
0x140759a66 e8058ee0ff call 0x140562870
0x140759a6b 48c1ada803000020 shr qword ptr [rbp + 0x3a8], 0x20
0x140759a73 488b8de8030000 mov rcx, qword ptr [rbp + 0x3e8]
0x140759a7a 488b4128 mov rax, qword ptr [rcx + 0x28]
0x140759a7e 48893cf0 mov qword ptr [rax + rsi*8], rdi
0x140759a82 48ffc6 inc rsi
0x140759a85 48897130 mov qword ptr [rcx + 0x30], rsi
0x140759a89 48ff4140 inc qword ptr [rcx + 0x40]
0x140759a8d 488b9d00020000 mov rbx, qword ptr [rbp + 0x200]
0x140759a94 4c8bb5e8010000 mov r14, qword ptr [rbp + 0x1e8]
0x140759a9b 4c8bbdd0010000 mov r15, qword ptr [rbp + 0x1d0]
0x140759aa2 e996e5ffff jmp 0x14075803d
0x140759aa7 488b8538020000 mov rax, qword ptr [rbp + 0x238]
0x140759aae 4885c0 test rax, rax
0x140759ab1 741a je 0x140759acd
0x140759ab3 488b8d40020000 mov rcx, qword ptr [rbp + 0x240]
0x140759aba 48c1e002 shl rax, 2
0x140759abe 488d1480 lea rdx, [rax + rax*4]
0x140759ac2 41b804000000 mov r8d, 4
0x140759ac8 e853a1ebff call 0x140613c20
0x140759acd 488b8550020000 mov rax, qword ptr [rbp + 0x250]
0x140759ad4 488d8d58020000 lea rcx, [rbp + 0x258]
0x140759adb 4885c0 test rax, rax
0x140759ade 0f85e0f7ffff jne 0x1407592c4
0x140759ae4 e9f1f7ffff jmp 0x1407592da
0x140759ae9 41b910000000 mov r9d, 0x10
0x140759aef 4c01ca add rdx, r9
0x140759af2 4821ca and rdx, rcx
0x140759af5 f30f6f0410 movdqu xmm0, xmmword ptr [rax + rdx]
0x140759afa 66440fd7c0 pmovmskb r8d, xmm0
0x140759aff 4983c110 add r9, 0x10
0x140759b03 4585c0 test r8d, r8d
0x140759b06 0f85dceeffff jne 0x1407589e8
0x140759b0c ebe1 jmp 0x140759aef
0x140759b0e 41b910000000 mov r9d, 0x10
0x140759b14 4c01ca add rdx, r9
0x140759b17 4821ca and rdx, rcx
0x140759b1a f30f6f0410 movdqu xmm0, xmmword ptr [rax + rdx]
0x140759b1f 66440fd7c0 pmovmskb r8d, xmm0
0x140759b24 4983c110 add r9, 0x10
0x140759b28 4585c0 test r8d, r8d
0x140759b2b 0f85e5efffff jne 0x140758b16
0x140759b31 ebe1 jmp 0x140759b14
0x140759b33 c6853404000000 mov byte ptr [rbp + 0x434], 0
0x140759b3a ba01000000 mov edx, 1
0x140759b3f 488b8d10040000 mov rcx, qword ptr [rbp + 0x410]
0x140759b46 4c8b85d8020000 mov r8, qword ptr [rbp + 0x2d8]
0x140759b4d 41b101 mov r9b, 1
0x140759b50 e86be51502 call 0x1428b80c0
0x140759b55 e94deeffff jmp 0x1407589a7
0x140759b5a c6853404000000 mov byte ptr [rbp + 0x434], 0
0x140759b61 ba01000000 mov edx, 1
0x140759b66 488b8d10040000 mov rcx, qword ptr [rbp + 0x410]
0x140759b6d 4c8b85d8020000 mov r8, qword ptr [rbp + 0x2d8]
0x140759b74 41b101 mov r9b, 1
0x140759b77 e844e51502 call 0x1428b80c0
0x140759b7c e954efffff jmp 0x140758ad5
0x140759b81 660f6f00 movdqa xmm0, xmmword ptr [rax]
0x140759b85 660fd7d0 pmovmskb edx, xmm0
0x140759b89 f3440fbcc2 tzcnt r8d, edx
0x140759b8e 420fb61400 movzx edx, byte ptr [rax + r8]
0x140759b93 e968eeffff jmp 0x140758a00
0x140759b98 660f6f00 movdqa xmm0, xmmword ptr [rax]
0x140759b9c 660fd7d0 pmovmskb edx, xmm0
0x140759ba0 f3440fbcc2 tzcnt r8d, edx
0x140759ba5 460fb60c00 movzx r9d, byte ptr [rax + r8]
0x140759baa e980efffff jmp 0x140758b2f
0x140759baf 488d0d42303202 lea rcx, [rip + 0x2323042]
0x140759bb6 e855f71702 call 0x1428d9310
0x140759bbb e9dd030000 jmp 0x140759f9d
0x140759bc0 488b9508020000 mov rdx, qword ptr [rbp + 0x208]
0x140759bc7 4885d2 test rdx, rdx
0x140759bca 7416 je 0x140759be2
0x140759bcc 48c1e203 shl rdx, 3
0x140759bd0 41b808000000 mov r8d, 8
0x140759bd6 488b8d08040000 mov rcx, qword ptr [rbp + 0x408]
0x140759bdd e83ea0ebff call 0x140613c20
0x140759be2 488b95b0010000 mov rdx, qword ptr [rbp + 0x1b0]
0x140759be9 4885d2 test rdx, rdx
0x140759bec 7416 je 0x140759c04
0x140759bee 488b8db8010000 mov rcx, qword ptr [rbp + 0x1b8]
0x140759bf5 48c1e204 shl rdx, 4
0x140759bf9 41b804000000 mov r8d, 4
0x140759bff e81ca0ebff call 0x140613c20
0x140759c04 80bd3104000000 cmp byte ptr [rbp + 0x431], 0
0x140759c0b 744b je 0x140759c58
0x140759c0d e81ee54b00 call 0x140c18130
0x140759c12 4889c1 mov rcx, rax
0x140759c15 488b00 mov rax, qword ptr [rax]
0x140759c18 4883f801 cmp rax, 1
0x140759c1c 0f85d1000000 jne 0x140759cf3
0x140759c22 4883c108 add rcx, 8
0x140759c26 488b95e8020000 mov rdx, qword ptr [rbp + 0x2e8]
0x140759c2d 48833900 cmp qword ptr [rcx], 0
0x140759c31 0f8585010000 jne 0x140759dbc
0x140759c37 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x140759c3e 48898df8030000 mov qword ptr [rbp + 0x3f8], rcx
0x140759c45 4883c108 add rcx, 8
0x140759c49 e8a2d44b00 call 0x140c170f0
0x140759c4e 488b85f8030000 mov rax, qword ptr [rbp + 0x3f8]
0x140759c55 48ff00 inc qword ptr [rax]
0x140759c58 0f28b540040000 movaps xmm6, xmmword ptr [rbp + 0x440]
0x140759c5f 0f28bd50040000 movaps xmm7, xmmword ptr [rbp + 0x450]
0x140759c66 440f288560040000 movaps xmm8, xmmword ptr [rbp + 0x460]
0x140759c6e 440f288d70040000 movaps xmm9, xmmword ptr [rbp + 0x470]
0x140759c76 440f289580040000 movaps xmm10, xmmword ptr [rbp + 0x480]
0x140759c7e 440f289d90040000 movaps xmm11, xmmword ptr [rbp + 0x490]
0x140759c86 440f28a5a0040000 movaps xmm12, xmmword ptr [rbp + 0x4a0]
0x140759c8e 440f28adb0040000 movaps xmm13, xmmword ptr [rbp + 0x4b0]
0x140759c96 440f28b5c0040000 movaps xmm14, xmmword ptr [rbp + 0x4c0]
0x140759c9e 440f28bdd0040000 movaps xmm15, xmmword ptr [rbp + 0x4d0]
0x140759ca6 4881c468050000 add rsp, 0x568
0x140759cad 5b pop rbx
0x140759cae 5f pop rdi
0x140759caf 5e pop rsi
0x140759cb0 415c pop r12
0x140759cb2 415d pop r13
0x140759cb4 415e pop r14
0x140759cb6 415f pop r15
0x140759cb8 5d pop rbp
0x140759cb9 c3 ret 
0x140759cba 4885c0 test rax, rax
0x140759cbd 7553 jne 0x140759d12
0x140759cbf 31d2 xor edx, edx
0x140759cc1 e8fa781902 call 0x1428f15c0
0x140759cc6 4889c1 mov rcx, rax
0x140759cc9 4885c0 test rax, rax
0x140759ccc 0f85e8ddffff jne 0x140757aba
0x140759cd2 eb3e jmp 0x140759d12
0x140759cd4 c6853204000001 mov byte ptr [rbp + 0x432], 1
0x140759cdb 488d0d76354c02 lea rcx, [rip + 0x24c3576]
0x140759ce2 488d15cf354c02 lea rdx, [rip + 0x24c35cf]
0x140759ce9 e812fd1402 call 0x1428a9a00
0x140759cee e9aa020000 jmp 0x140759f9d
0x140759cf3 4885c0 test rax, rax
0x140759cf6 751a jne 0x140759d12
0x140759cf8 31d2 xor edx, edx
0x140759cfa e8c1781902 call 0x1428f15c0
0x140759cff 488b95e8020000 mov rdx, qword ptr [rbp + 0x2e8]
0x140759d06 4889c1 mov rcx, rax
0x140759d09 4885c0 test rax, rax
0x140759d0c 0f851bffffff jne 0x140759c2d
0x140759d12 488d0d8ff74b02 lea rcx, [rip + 0x24bf78f]
0x140759d19 e8b2e31702 call 0x1428d80d0
0x140759d1e 4883fa15 cmp rdx, 0x15
0x140759d22 0f8377020000 jae 0x140759f9f
0x140759d28 4c8b9d08040000 mov r11, qword ptr [rbp + 0x408]
0x140759d2f 498d04d3 lea rax, [r11 + rdx*8]
0x140759d33 498d4b08 lea rcx, [r11 + 8]
0x140759d37 ba08000000 mov edx, 8
0x140759d3c eb17 jmp 0x140759d55
0x140759d3e 4d89d9 mov r9, r11
0x140759d41 4d8901 mov qword ptr [r9], r8
0x140759d44 4883c108 add rcx, 8
0x140759d48 4883c208 add rdx, 8
0x140759d4c 4839c1 cmp rcx, rax
0x140759d4f 0f8464020000 je 0x140759fb9
0x140759d55 4c8b51f8 mov r10, qword ptr [rcx - 8]
0x140759d59 4c8b01 mov r8, qword ptr [rcx]
0x140759d5c 4d39d0 cmp r8, r10
0x140759d5f 73e3 jae 0x140759d44
0x140759d61 4989d1 mov r9, rdx
0x140759d64 4f89140b mov qword ptr [r11 + r9], r10
0x140759d68 4983f908 cmp r9, 8
0x140759d6c 74d0 je 0x140759d3e
0x140759d6e 4f8b540bf0 mov r10, qword ptr [r11 + r9 - 0x10]
0x140759d73 4983c1f8 add r9, -8
0x140759d77 4d39d0 cmp r8, r10
0x140759d7a 72e8 jb 0x140759d64
0x140759d7c 4d01d9 add r9, r11
0x140759d7f ebc0 jmp 0x140759d41
0x140759d81 48c785f002000000000000 mov qword ptr [rbp + 0x2f0], 0
0x140759d8c c6853504000001 mov byte ptr [rbp + 0x435], 1
0x140759d93 488d05c60a4c02 lea rax, [rip + 0x24c0ac6]
0x140759d9a 4889442420 mov qword ptr [rsp + 0x20], rax
0x140759d9f 488d9530020000 lea rdx, [rbp + 0x230]
0x140759da6 4c8d8df0020000 lea r9, [rbp + 0x2f0]
0x140759dad 31c9 xor ecx, ecx
0x140759daf 4989f8 mov r8, rdi
0x140759db2 e88a0d1702 call 0x1428cab41
0x140759db7 e9e1010000 jmp 0x140759f9d
0x140759dbc 488d0d55f74b02 lea rcx, [rip + 0x24bf755]
0x140759dc3 e888f41702 call 0x1428d9250
0x140759dc8 c6853504000001 mov byte ptr [rbp + 0x435], 1
0x140759dcf 488d0d6a2e3202 lea rcx, [rip + 0x2322e6a]
0x140759dd6 e835f51702 call 0x1428d9310
0x140759ddb e9bd010000 jmp 0x140759f9d
0x140759de0 488995a8030000 mov qword ptr [rbp + 0x3a8], rdx
0x140759de7 4531c0 xor r8d, r8d
0x140759dea 4889c2 mov rdx, rax
0x140759ded eb62 jmp 0x140759e51
0x140759def 48c1e220 shl rdx, 0x20
0x140759df3 4c09ca or rdx, r9
0x140759df6 41b901000000 mov r9d, 1
0x140759dfc 4889d0 mov rax, rdx
0x140759dff 44898df0020000 mov dword ptr [rbp + 0x2f0], r9d
0x140759e06 448985f4020000 mov dword ptr [rbp + 0x2f4], r8d
0x140759e0d 488985f8020000 mov qword ptr [rbp + 0x2f8], rax
0x140759e14 c6853704000001 mov byte ptr [rbp + 0x437], 1
0x140759e1b 488d05ee2d3202 lea rax, [rip + 0x2322dee]
0x140759e22 4889442420 mov qword ptr [rsp + 0x20], rax
0x140759e27 488d0da22b3202 lea rcx, [rip + 0x2322ba2]
0x140759e2e 4c8d0d7b2b3202 lea r9, [rip + 0x2322b7b]
0x140759e35 4c8d85f0020000 lea r8, [rbp + 0x2f0]
0x140759e3c ba2b000000 mov edx, 0x2b
0x140759e41 e81af91702 call 0x1428d9760
0x140759e46 e952010000 jmp 0x140759f9d
0x140759e4b 41b801000000 mov r8d, 1
0x140759e51 488b85a8030000 mov rax, qword ptr [rbp + 0x3a8]
0x140759e58 48c1e020 shl rax, 0x20
0x140759e5c 4c09c0 or rax, r8
0x140759e5f 48898540010000 mov qword ptr [rbp + 0x140], rax
0x140759e66 48c1e220 shl rdx, 0x20
0x140759e6a 4809ca or rdx, rcx
0x140759e6d 48899548010000 mov qword ptr [rbp + 0x148], rdx
0x140759e74 488d8528040000 lea rax, [rbp + 0x428]
0x140759e7b 48898530020000 mov qword ptr [rbp + 0x230], rax
0x140759e82 488d0537b03d00 lea rax, [rip + 0x3db037]
0x140759e89 48898538020000 mov qword ptr [rbp + 0x238], rax
0x140759e90 488d8540010000 lea rax, [rbp + 0x140]
0x140759e97 48898540020000 mov qword ptr [rbp + 0x240], rax
0x140759e9e 488d059bd8ffff lea rax, [rip - 0x2765]
0x140759ea5 48898548020000 mov qword ptr [rbp + 0x248], rax
0x140759eac 488d05452e3202 lea rax, [rip + 0x2322e45]
0x140759eb3 488985f0020000 mov qword ptr [rbp + 0x2f0], rax
0x140759eba 48c785f802000002000000 mov qword ptr [rbp + 0x2f8], 2
0x140759ec5 48c7851003000000000000 mov qword ptr [rbp + 0x310], 0
0x140759ed0 488d8530020000 lea rax, [rbp + 0x230]
0x140759ed7 48898500030000 mov qword ptr [rbp + 0x300], rax
0x140759ede 48c7850803000002000000 mov qword ptr [rbp + 0x308], 2
0x140759ee9 c6853704000001 mov byte ptr [rbp + 0x437], 1
0x140759ef0 488d15212e3202 lea rdx, [rip + 0x2322e21]
0x140759ef7 488d8df0020000 lea rcx, [rbp + 0x2f0]
0x140759efe e88df41702 call 0x1428d9390
0x140759f03 e995000000 jmp 0x140759f9d
0x140759f08 c6853204000001 mov byte ptr [rbp + 0x432], 1
0x140759f0f b908000000 mov ecx, 8
0x140759f14 ba20000000 mov edx, 0x20
0x140759f19 e8e2f01702 call 0x1428d9000
0x140759f1e eb7d jmp 0x140759f9d
0x140759f20 f30f6f85f8020000 movdqu xmm0, xmmword ptr [rbp + 0x2f8]
0x140759f28 660f7f8530020000 movdqa xmmword ptr [rbp + 0x230], xmm0
0x140759f30 488d0541f44b02 lea rax, [rip + 0x24bf441]
0x140759f37 4889442420 mov qword ptr [rsp + 0x20], rax
0x140759f3c 488d0d85ec4b02 lea rcx, [rip + 0x24bec85]
0x140759f43 4c8d0d5eec4b02 lea r9, [rip + 0x24bec5e]
0x140759f4a 4c8d8530020000 lea r8, [rbp + 0x230]
0x140759f51 ba2b000000 mov edx, 0x2b
0x140759f56 e805f81702 call 0x1428d9760
0x140759f5b eb40 jmp 0x140759f9d
0x140759f5d c6853704000001 mov byte ptr [rbp + 0x437], 1
0x140759f64 4c8d052d2a3202 lea r8, [rip + 0x2322a2d]
0x140759f6b b904000000 mov ecx, 4
0x140759f70 4889da mov rdx, rbx
0x140759f73 e86bf01702 call 0x1428d8fe3
0x140759f78 eb23 jmp 0x140759f9d
0x140759f7a 488d0da72c3202 lea rcx, [rip + 0x2322ca7]
0x140759f81 e88af31702 call 0x1428d9310
0x140759f86 eb15 jmp 0x140759f9d
0x140759f88 4531ff xor r15d, r15d
0x140759f8b 4c8d05deff4b02 lea r8, [rip + 0x24bffde]
0x140759f92 4c89f9 mov rcx, r15
0x140759f95 4c89e2 mov rdx, r12
0x140759f98 e846f01702 call 0x1428d8fe3
0x140759f9d 0f0b ud2 
0x140759f9f c6853304000001 mov byte ptr [rbp + 0x433], 1
0x140759fa6 4c8d85f0020000 lea r8, [rbp + 0x2f0]
0x140759fad 488b8d08040000 mov rcx, qword ptr [rbp + 0x408]
0x140759fb4 e8a7ed1f00 call 0x140958d60
0x140759fb9 488b85b8030000 mov rax, qword ptr [rbp + 0x3b8]
0x140759fc0 48898508040000 mov qword ptr [rbp + 0x408], rax
0x140759fc7 488b95c0030000 mov rdx, qword ptr [rbp + 0x3c0]
0x140759fce 4883fa02 cmp rdx, 2
0x140759fd2 730c jae 0x140759fe0
0x140759fd4 4c8ba5f8030000 mov r12, qword ptr [rbp + 0x3f8]
0x140759fdb e9b6deffff jmp 0x140757e96
0x140759fe0 b801000000 mov eax, 1
0x140759fe5 4c8ba5f8030000 mov r12, qword ptr [rbp + 0x3f8]
0x140759fec 4c8b95e8030000 mov r10, qword ptr [rbp + 0x3e8]
0x140759ff3 4c8b8508040000 mov r8, qword ptr [rbp + 0x408]
0x140759ffa 498b0cc0 mov rcx, qword ptr [r8 + rax*8]
0x140759ffe 493b4cc0f8 cmp rcx, qword ptr [r8 + rax*8 - 8]
0x14075a003 740d je 0x14075a012
0x14075a005 48ffc0 inc rax
0x14075a008 4839c2 cmp rdx, rax
0x14075a00b 75ed jne 0x140759ffa
0x14075a00d e98bdeffff jmp 0x140757e9d
0x14075a012 488d4801 lea rcx, [rax + 1]
0x14075a016 4839d1 cmp rcx, rdx
0x14075a019 7333 jae 0x14075a04e
0x14075a01b 4c8b8d08040000 mov r9, qword ptr [rbp + 0x408]
0x14075a022 eb08 jmp 0x14075a02c
0x14075a024 48ffc1 inc rcx
0x14075a027 4839d1 cmp rcx, rdx
0x14075a02a 7314 jae 0x14075a040
0x14075a02c 4d8b04c9 mov r8, qword ptr [r9 + rcx*8]
0x14075a030 4d3b44c1f8 cmp r8, qword ptr [r9 + rax*8 - 8]
0x14075a035 74ed je 0x14075a024
0x14075a037 4d8904c1 mov qword ptr [r9 + rax*8], r8
0x14075a03b 48ffc0 inc rax
0x14075a03e ebe4 jmp 0x14075a024
0x14075a040 488b8db8030000 mov rcx, qword ptr [rbp + 0x3b8]
0x14075a047 48898d08040000 mov qword ptr [rbp + 0x408], rcx
0x14075a04e 488985c0030000 mov qword ptr [rbp + 0x3c0], rax
0x14075a055 4889c2 mov rdx, rax
0x14075a058 e940deffff jmp 0x140757e9d
0x14075a05d 0f1f00 nop dword ptr [rax]
0x14075a060 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14075a065 55 push rbp
0x14075a066 4157 push r15
0x14075a068 4156 push r14
0x14075a06a 4155 push r13
0x14075a06c 4154 push r12
0x14075a06e 56 push rsi
0x14075a06f 57 push rdi
0x14075a070 53 push rbx
0x14075a071 4881ec58010000 sub rsp, 0x158
0x14075a078 488daa80000000 lea rbp, [rdx + 0x80]
0x14075a07f 440f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm15
0x14075a088 440f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm14
0x14075a091 66440f7fac24d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm13
0x14075a09b 66440f7fa424e0000000 movdqa xmmword ptr [rsp + 0xe0], xmm12
0x14075a0a5 66440f7f9c24f0000000 movdqa xmmword ptr [rsp + 0xf0], xmm11
0x14075a0af 440f29942400010000 movaps xmmword ptr [rsp + 0x100], xmm10
0x14075a0b8 440f298c2410010000 movaps xmmword ptr [rsp + 0x110], xmm9
0x14075a0c1 440f29842420010000 movaps xmmword ptr [rsp + 0x120], xmm8
0x14075a0ca 0f29bc2430010000 movaps xmmword ptr [rsp + 0x130], xmm7
0x14075a0d2 660f7fb42440010000 movdqa xmmword ptr [rsp + 0x140], xmm6
0x14075a0db 488b85f8030000 mov rax, qword ptr [rbp + 0x3f8]
0x14075a0e2 48ff00 inc qword ptr [rax]
0x14075a0e5 0f28b42440010000 movaps xmm6, xmmword ptr [rsp + 0x140]
0x14075a0ed 0f28bc2430010000 movaps xmm7, xmmword ptr [rsp + 0x130]
0x14075a0f5 440f28842420010000 movaps xmm8, xmmword ptr [rsp + 0x120]
0x14075a0fe 440f288c2410010000 movaps xmm9, xmmword ptr [rsp + 0x110]
0x14075a107 440f28942400010000 movaps xmm10, xmmword ptr [rsp + 0x100]
0x14075a110 440f289c24f0000000 movaps xmm11, xmmword ptr [rsp + 0xf0]
0x14075a119 440f28a424e0000000 movaps xmm12, xmmword ptr [rsp + 0xe0]
0x14075a122 440f28ac24d0000000 movaps xmm13, xmmword ptr [rsp + 0xd0]
0x14075a12b 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
0x14075a134 440f28bc24b0000000 movaps xmm15, xmmword ptr [rsp + 0xb0]
0x14075a13d 4881c458010000 add rsp, 0x158
0x14075a144 5b pop rbx
0x14075a145 5f pop rdi
0x14075a146 5e pop rsi
0x14075a147 415c pop r12
0x14075a149 415d pop r13
0x14075a14b 415e pop r14
0x14075a14d 415f pop r15
0x14075a14f 5d pop rbp
0x14075a150 c3 ret 
0x14075a151 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14075a160 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14075a165 55 push rbp
0x14075a166 4157 push r15
0x14075a168 4156 push r14
0x14075a16a 4155 push r13
0x14075a16c 4154 push r12
0x14075a16e 56 push rsi
0x14075a16f 57 push rdi
0x14075a170 53 push rbx
0x14075a171 4881ec58010000 sub rsp, 0x158
0x14075a178 488daa80000000 lea rbp, [rdx + 0x80]
0x14075a17f 440f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm15
0x14075a188 440f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm14
0x14075a191 66440f7fac24d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm13
0x14075a19b 66440f7fa424e0000000 movdqa xmmword ptr [rsp + 0xe0], xmm12
0x14075a1a5 66440f7f9c24f0000000 movdqa xmmword ptr [rsp + 0xf0], xmm11
0x14075a1af 440f29942400010000 movaps xmmword ptr [rsp + 0x100], xmm10
0x14075a1b8 440f298c2410010000 movaps xmmword ptr [rsp + 0x110], xmm9
0x14075a1c1 440f29842420010000 movaps xmmword ptr [rsp + 0x120], xmm8
0x14075a1ca 0f29bc2430010000 movaps xmmword ptr [rsp + 0x130], xmm7
0x14075a1d2 660f7fb42440010000 movdqa xmmword ptr [rsp + 0x140], xmm6
0x14075a1db 4883bdd003000000 cmp qword ptr [rbp + 0x3d0], 0
0x14075a1e3 741d je 0x14075a202
0x14075a1e5 488b95d0030000 mov rdx, qword ptr [rbp + 0x3d0]
0x14075a1ec 48c1e206 shl rdx, 6
0x14075a1f0 41b804000000 mov r8d, 4
0x14075a1f6 488b8d28020000 mov rcx, qword ptr [rbp + 0x228]
0x14075a1fd e81e9aebff call 0x140613c20
0x14075a202 c6853604000000 mov byte ptr [rbp + 0x436], 0
0x14075a209 660f6fb42440010000 movdqa xmm6, xmmword ptr [rsp + 0x140]
0x14075a212 0f28bc2430010000 movaps xmm7, xmmword ptr [rsp + 0x130]
0x14075a21a 440f28842420010000 movaps xmm8, xmmword ptr [rsp + 0x120]
0x14075a223 440f288c2410010000 movaps xmm9, xmmword ptr [rsp + 0x110]
0x14075a22c 440f28942400010000 movaps xmm10, xmmword ptr [rsp + 0x100]
0x14075a235 66440f6f9c24f0000000 movdqa xmm11, xmmword ptr [rsp + 0xf0]
0x14075a23f 66440f6fa424e0000000 movdqa xmm12, xmmword ptr [rsp + 0xe0]
0x14075a249 66440f6fac24d0000000 movdqa xmm13, xmmword ptr [rsp + 0xd0]
0x14075a253 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
0x14075a25c 440f28bc24b0000000 movaps xmm15, xmmword ptr [rsp + 0xb0]
0x14075a265 4881c458010000 add rsp, 0x158
0x14075a26c 5b pop rbx
0x14075a26d 5f pop rdi
0x14075a26e 5e pop rsi
0x14075a26f 415c pop r12
0x14075a271 415d pop r13
0x14075a273 415e pop r14
0x14075a275 415f pop r15
0x14075a277 5d pop rbp
0x14075a278 c3 ret 
0x14075a279 0f1f8000000000 nop dword ptr [rax]
0x14075a280 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14075a285 55 push rbp
0x14075a286 4157 push r15
0x14075a288 4156 push r14
0x14075a28a 4155 push r13
0x14075a28c 4154 push r12
0x14075a28e 56 push rsi
0x14075a28f 57 push rdi
0x14075a290 53 push rbx
0x14075a291 4881ec58010000 sub rsp, 0x158
0x14075a298 488daa80000000 lea rbp, [rdx + 0x80]
0x14075a29f 440f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm15
0x14075a2a8 440f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm14
0x14075a2b1 66440f7fac24d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm13
0x14075a2bb 66440f7fa424e0000000 movdqa xmmword ptr [rsp + 0xe0], xmm12
0x14075a2c5 66440f7f9c24f0000000 movdqa xmmword ptr [rsp + 0xf0], xmm11
0x14075a2cf 440f29942400010000 movaps xmmword ptr [rsp + 0x100], xmm10
0x14075a2d8 440f298c2410010000 movaps xmmword ptr [rsp + 0x110], xmm9
0x14075a2e1 440f29842420010000 movaps xmmword ptr [rsp + 0x120], xmm8
0x14075a2ea 0f29bc2430010000 movaps xmmword ptr [rsp + 0x130], xmm7
0x14075a2f2 660f7fb42440010000 movdqa xmmword ptr [rsp + 0x140], xmm6
0x14075a2fb 488b8520040000 mov rax, qword ptr [rbp + 0x420]
0x14075a302 48ff00 inc qword ptr [rax]
0x14075a305 0f28b42440010000 movaps xmm6, xmmword ptr [rsp + 0x140]
0x14075a30d 0f28bc2430010000 movaps xmm7, xmmword ptr [rsp + 0x130]
0x14075a315 440f28842420010000 movaps xmm8, xmmword ptr [rsp + 0x120]
0x14075a31e 440f288c2410010000 movaps xmm9, xmmword ptr [rsp + 0x110]
0x14075a327 440f28942400010000 movaps xmm10, xmmword ptr [rsp + 0x100]
0x14075a330 440f289c24f0000000 movaps xmm11, xmmword ptr [rsp + 0xf0]
0x14075a339 440f28a424e0000000 movaps xmm12, xmmword ptr [rsp + 0xe0]
0x14075a342 440f28ac24d0000000 movaps xmm13, xmmword ptr [rsp + 0xd0]
0x14075a34b 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
0x14075a354 440f28bc24b0000000 movaps xmm15, xmmword ptr [rsp + 0xb0]
0x14075a35d 4881c458010000 add rsp, 0x158
0x14075a364 5b pop rbx
0x14075a365 5f pop rdi
0x14075a366 5e pop rsi
0x14075a367 415c pop r12
0x14075a369 415d pop r13
0x14075a36b 415e pop r14
0x14075a36d 415f pop r15
0x14075a36f 5d pop rbp
0x14075a370 c3 ret 
0x14075a371 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14075a380 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14075a385 55 push rbp
0x14075a386 4157 push r15
0x14075a388 4156 push r14
0x14075a38a 4155 push r13
0x14075a38c 4154 push r12
0x14075a38e 56 push rsi
0x14075a38f 57 push rdi
0x14075a390 53 push rbx
0x14075a391 4881ec58010000 sub rsp, 0x158
0x14075a398 488daa80000000 lea rbp, [rdx + 0x80]
0x14075a39f 440f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm15
0x14075a3a8 440f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm14
0x14075a3b1 66440f7fac24d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm13
0x14075a3bb 66440f7fa424e0000000 movdqa xmmword ptr [rsp + 0xe0], xmm12
0x14075a3c5 66440f7f9c24f0000000 movdqa xmmword ptr [rsp + 0xf0], xmm11
0x14075a3cf 440f29942400010000 movaps xmmword ptr [rsp + 0x100], xmm10
0x14075a3d8 440f298c2410010000 movaps xmmword ptr [rsp + 0x110], xmm9
0x14075a3e1 440f29842420010000 movaps xmmword ptr [rsp + 0x120], xmm8
0x14075a3ea 0f29bc2430010000 movaps xmmword ptr [rsp + 0x130], xmm7
0x14075a3f2 660f7fb42440010000 movdqa xmmword ptr [rsp + 0x140], xmm6
0x14075a3fb 80bd3004000000 cmp byte ptr [rbp + 0x430], 0
0x14075a402 7413 je 0x14075a417
0x14075a404 c6853204000000 mov byte ptr [rbp + 0x432], 0
0x14075a40b 488d8d30020000 lea rcx, [rbp + 0x230]
0x14075a412 e8090f9fff call 0x14014b320
0x14075a417 c6853204000000 mov byte ptr [rbp + 0x432], 0
0x14075a41e 660f6fb42440010000 movdqa xmm6, xmmword ptr [rsp + 0x140]
0x14075a427 0f28bc2430010000 movaps xmm7, xmmword ptr [rsp + 0x130]
0x14075a42f 440f28842420010000 movaps xmm8, xmmword ptr [rsp + 0x120]
0x14075a438 440f288c2410010000 movaps xmm9, xmmword ptr [rsp + 0x110]
0x14075a441 440f28942400010000 movaps xmm10, xmmword ptr [rsp + 0x100]
0x14075a44a 66440f6f9c24f0000000 movdqa xmm11, xmmword ptr [rsp + 0xf0]
0x14075a454 66440f6fa424e0000000 movdqa xmm12, xmmword ptr [rsp + 0xe0]
0x14075a45e 66440f6fac24d0000000 movdqa xmm13, xmmword ptr [rsp + 0xd0]
0x14075a468 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
0x14075a471 440f28bc24b0000000 movaps xmm15, xmmword ptr [rsp + 0xb0]
0x14075a47a 4881c458010000 add rsp, 0x158
0x14075a481 5b pop rbx
0x14075a482 5f pop rdi
0x14075a483 5e pop rsi
0x14075a484 415c pop r12
0x14075a486 415d pop r13
0x14075a488 415e pop r14
0x14075a48a 415f pop r15
0x14075a48c 5d pop rbp
0x14075a48d c3 ret 
0x14075a48e 6690 nop 
0x14075a490 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14075a495 55 push rbp
0x14075a496 4157 push r15
0x14075a498 4156 push r14
0x14075a49a 4155 push r13
0x14075a49c 4154 push r12
0x14075a49e 56 push rsi
0x14075a49f 57 push rdi
0x14075a4a0 53 push rbx
0x14075a4a1 4881ec58010000 sub rsp, 0x158
0x14075a4a8 488daa80000000 lea rbp, [rdx + 0x80]
0x14075a4af 440f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm15
0x14075a4b8 440f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm14
0x14075a4c1 66440f7fac24d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm13
0x14075a4cb 66440f7fa424e0000000 movdqa xmmword ptr [rsp + 0xe0], xmm12
0x14075a4d5 66440f7f9c24f0000000 movdqa xmmword ptr [rsp + 0xf0], xmm11
0x14075a4df 440f29942400010000 movaps xmmword ptr [rsp + 0x100], xmm10
0x14075a4e8 440f298c2410010000 movaps xmmword ptr [rsp + 0x110], xmm9
0x14075a4f1 440f29842420010000 movaps xmmword ptr [rsp + 0x120], xmm8
0x14075a4fa 0f29bc2430010000 movaps xmmword ptr [rsp + 0x130], xmm7
0x14075a502 660f7fb42440010000 movdqa xmmword ptr [rsp + 0x140], xmm6
0x14075a50b 80bd3604000000 cmp byte ptr [rbp + 0x436], 0
0x14075a512 0f849e000000 je 0x14075a5b6
0x14075a518 488b9510010000 mov rdx, qword ptr [rbp + 0x110]
0x14075a51f 4885d2 test rdx, rdx
0x14075a522 7416 je 0x14075a53a
0x14075a524 488b8d18010000 mov rcx, qword ptr [rbp + 0x118]
0x14075a52b 48c1e206 shl rdx, 6
0x14075a52f 41b804000000 mov r8d, 4
0x14075a535 e8e696ebff call 0x140613c20
0x14075a53a 8b85c8030000 mov eax, dword ptr [rbp + 0x3c8]
0x14075a540 888534040000 mov byte ptr [rbp + 0x434], al
0x14075a546 660f6fb42440010000 movdqa xmm6, xmmword ptr [rsp + 0x140]
0x14075a54f 0f28bc2430010000 movaps xmm7, xmmword ptr [rsp + 0x130]
0x14075a557 440f28842420010000 movaps xmm8, xmmword ptr [rsp + 0x120]
0x14075a560 440f288c2410010000 movaps xmm9, xmmword ptr [rsp + 0x110]
0x14075a569 440f28942400010000 movaps xmm10, xmmword ptr [rsp + 0x100]
0x14075a572 66440f6f9c24f0000000 movdqa xmm11, xmmword ptr [rsp + 0xf0]
0x14075a57c 66440f6fa424e0000000 movdqa xmm12, xmmword ptr [rsp + 0xe0]
0x14075a586 66440f6fac24d0000000 movdqa xmm13, xmmword ptr [rsp + 0xd0]
0x14075a590 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
0x14075a599 440f28bc24b0000000 movaps xmm15, xmmword ptr [rsp + 0xb0]
0x14075a5a2 4881c458010000 add rsp, 0x158
0x14075a5a9 5b pop rbx
0x14075a5aa 5f pop rdi
0x14075a5ab 5e pop rsi
0x14075a5ac 415c pop r12
0x14075a5ae 415d pop r13
0x14075a5b0 415e pop r14
0x14075a5b2 415f pop r15
0x14075a5b4 5d pop rbp
0x14075a5b5 c3 ret 
0x14075a5b6 8b85c8030000 mov eax, dword ptr [rbp + 0x3c8]
0x14075a5bc 888534040000 mov byte ptr [rbp + 0x434], al
0x14075a5c2 660f6fb42440010000 movdqa xmm6, xmmword ptr [rsp + 0x140]
0x14075a5cb 0f28bc2430010000 movaps xmm7, xmmword ptr [rsp + 0x130]
0x14075a5d3 440f28842420010000 movaps xmm8, xmmword ptr [rsp + 0x120]
0x14075a5dc 440f288c2410010000 movaps xmm9, xmmword ptr [rsp + 0x110]
0x14075a5e5 440f28942400010000 movaps xmm10, xmmword ptr [rsp + 0x100]
0x14075a5ee 66440f6f9c24f0000000 movdqa xmm11, xmmword ptr [rsp + 0xf0]
0x14075a5f8 66440f6fa424e0000000 movdqa xmm12, xmmword ptr [rsp + 0xe0]
0x14075a602 66440f6fac24d0000000 movdqa xmm13, xmmword ptr [rsp + 0xd0]
0x14075a60c 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
0x14075a615 440f28bc24b0000000 movaps xmm15, xmmword ptr [rsp + 0xb0]
0x14075a61e 4881c458010000 add rsp, 0x158
0x14075a625 5b pop rbx
0x14075a626 5f pop rdi
0x14075a627 5e pop rsi
0x14075a628 415c pop r12
0x14075a62a 415d pop r13
0x14075a62c 415e pop r14
0x14075a62e 415f pop r15
0x14075a630 5d pop rbp
0x14075a631 c3 ret 
0x14075a632 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14075a640 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14075a645 55 push rbp
0x14075a646 4157 push r15
0x14075a648 4156 push r14
0x14075a64a 4155 push r13
0x14075a64c 4154 push r12
0x14075a64e 56 push rsi
0x14075a64f 57 push rdi
0x14075a650 53 push rbx
0x14075a651 4881ec58010000 sub rsp, 0x158
0x14075a658 488daa80000000 lea rbp, [rdx + 0x80]
0x14075a65f 440f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm15
0x14075a668 440f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm14
0x14075a671 66440f7fac24d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm13
0x14075a67b 66440f7fa424e0000000 movdqa xmmword ptr [rsp + 0xe0], xmm12
0x14075a685 66440f7f9c24f0000000 movdqa xmmword ptr [rsp + 0xf0], xmm11
0x14075a68f 440f29942400010000 movaps xmmword ptr [rsp + 0x100], xmm10
0x14075a698 440f298c2410010000 movaps xmmword ptr [rsp + 0x110], xmm9
0x14075a6a1 440f29842420010000 movaps xmmword ptr [rsp + 0x120], xmm8
0x14075a6aa 0f29bc2430010000 movaps xmmword ptr [rsp + 0x130], xmm7
0x14075a6b2 660f7fb42440010000 movdqa xmmword ptr [rsp + 0x140], xmm6
0x14075a6bb f6853404000001 test byte ptr [rbp + 0x434], 1
0x14075a6c2 747c je 0x14075a740
0x14075a6c4 488d8d30020000 lea rcx, [rbp + 0x230]
0x14075a6cb e8b0cbffff call 0x140757280
0x14075a6d0 660f6fb42440010000 movdqa xmm6, xmmword ptr [rsp + 0x140]
0x14075a6d9 0f28bc2430010000 movaps xmm7, xmmword ptr [rsp + 0x130]
0x14075a6e1 440f28842420010000 movaps xmm8, xmmword ptr [rsp + 0x120]
0x14075a6ea 440f288c2410010000 movaps xmm9, xmmword ptr [rsp + 0x110]
0x14075a6f3 440f28942400010000 movaps xmm10, xmmword ptr [rsp + 0x100]
0x14075a6fc 66440f6f9c24f0000000 movdqa xmm11, xmmword ptr [rsp + 0xf0]
0x14075a706 66440f6fa424e0000000 movdqa xmm12, xmmword ptr [rsp + 0xe0]
0x14075a710 66440f6fac24d0000000 movdqa xmm13, xmmword ptr [rsp + 0xd0]
0x14075a71a 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
0x14075a723 440f28bc24b0000000 movaps xmm15, xmmword ptr [rsp + 0xb0]
0x14075a72c 4881c458010000 add rsp, 0x158
0x14075a733 5b pop rbx
0x14075a734 5f pop rdi
0x14075a735 5e pop rsi
0x14075a736 415c pop r12
0x14075a738 415d pop r13
0x14075a73a 415e pop r14
0x14075a73c 415f pop r15
0x14075a73e 5d pop rbp
0x14075a73f c3 ret 
0x14075a740 660f6fb42440010000 movdqa xmm6, xmmword ptr [rsp + 0x140]
0x14075a749 0f28bc2430010000 movaps xmm7, xmmword ptr [rsp + 0x130]
0x14075a751 440f28842420010000 movaps xmm8, xmmword ptr [rsp + 0x120]
0x14075a75a 440f288c2410010000 movaps xmm9, xmmword ptr [rsp + 0x110]
0x14075a763 440f28942400010000 movaps xmm10, xmmword ptr [rsp + 0x100]
0x14075a76c 66440f6f9c24f0000000 movdqa xmm11, xmmword ptr [rsp + 0xf0]
0x14075a776 66440f6fa424e0000000 movdqa xmm12, xmmword ptr [rsp + 0xe0]
0x14075a780 66440f6fac24d0000000 movdqa xmm13, xmmword ptr [rsp + 0xd0]
0x14075a78a 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
0x14075a793 440f28bc24b0000000 movaps xmm15, xmmword ptr [rsp + 0xb0]
0x14075a79c 4881c458010000 add rsp, 0x158
0x14075a7a3 5b pop rbx
0x14075a7a4 5f pop rdi
0x14075a7a5 5e pop rsi
0x14075a7a6 415c pop r12
0x14075a7a8 415d pop r13
0x14075a7aa 415e pop r14
0x14075a7ac 415f pop r15
0x14075a7ae 5d pop rbp
0x14075a7af c3 ret 
0x14075a7b0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14075a7b5 55 push rbp
0x14075a7b6 4157 push r15
0x14075a7b8 4156 push r14
0x14075a7ba 4155 push r13
0x14075a7bc 4154 push r12
0x14075a7be 56 push rsi
0x14075a7bf 57 push rdi
0x14075a7c0 53 push rbx
0x14075a7c1 4881ec58010000 sub rsp, 0x158
0x14075a7c8 488daa80000000 lea rbp, [rdx + 0x80]
0x14075a7cf 440f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm15
0x14075a7d8 440f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm14
0x14075a7e1 66440f7fac24d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm13
0x14075a7eb 66440f7fa424e0000000 movdqa xmmword ptr [rsp + 0xe0], xmm12
0x14075a7f5 66440f7f9c24f0000000 movdqa xmmword ptr [rsp + 0xf0], xmm11
0x14075a7ff 440f29942400010000 movaps xmmword ptr [rsp + 0x100], xmm10
0x14075a808 440f298c2410010000 movaps xmmword ptr [rsp + 0x110], xmm9
0x14075a811 440f29842420010000 movaps xmmword ptr [rsp + 0x120], xmm8
0x14075a81a 0f29bc2430010000 movaps xmmword ptr [rsp + 0x130], xmm7
0x14075a822 660f7fb42440010000 movdqa xmmword ptr [rsp + 0x140], xmm6
0x14075a82b 488d8d40010000 lea rcx, [rbp + 0x140]
0x14075a832 e8b97ce7ff call 0x1405d24f0
0x14075a837 660f6fb42440010000 movdqa xmm6, xmmword ptr [rsp + 0x140]
0x14075a840 0f28bc2430010000 movaps xmm7, xmmword ptr [rsp + 0x130]
0x14075a848 440f28842420010000 movaps xmm8, xmmword ptr [rsp + 0x120]
0x14075a851 440f288c2410010000 movaps xmm9, xmmword ptr [rsp + 0x110]
0x14075a85a 440f28942400010000 movaps xmm10, xmmword ptr [rsp + 0x100]
0x14075a863 440f289c24f0000000 movaps xmm11, xmmword ptr [rsp + 0xf0]
0x14075a86c 440f28a424e0000000 movaps xmm12, xmmword ptr [rsp + 0xe0]
0x14075a875 440f28ac24d0000000 movaps xmm13, xmmword ptr [rsp + 0xd0]
0x14075a87e 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
0x14075a887 440f28bc24b0000000 movaps xmm15, xmmword ptr [rsp + 0xb0]
0x14075a890 4881c458010000 add rsp, 0x158
0x14075a897 5b pop rbx
0x14075a898 5f pop rdi
0x14075a899 5e pop rsi
0x14075a89a 415c pop r12
0x14075a89c 415d pop r13
0x14075a89e 415e pop r14
0x14075a8a0 415f pop r15
0x14075a8a2 5d pop rbp
0x14075a8a3 c3 ret 
0x14075a8a4 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14075a8b0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14075a8b5 55 push rbp
0x14075a8b6 4157 push r15
0x14075a8b8 4156 push r14
0x14075a8ba 4155 push r13
0x14075a8bc 4154 push r12
0x14075a8be 56 push rsi
0x14075a8bf 57 push rdi
0x14075a8c0 53 push rbx
0x14075a8c1 4881ec58010000 sub rsp, 0x158
0x14075a8c8 488daa80000000 lea rbp, [rdx + 0x80]
0x14075a8cf 440f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm15
0x14075a8d8 440f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm14
0x14075a8e1 440f29ac24d0000000 movaps xmmword ptr [rsp + 0xd0], xmm13
0x14075a8ea 440f29a424e0000000 movaps xmmword ptr [rsp + 0xe0], xmm12
0x14075a8f3 440f299c24f0000000 movaps xmmword ptr [rsp + 0xf0], xmm11
0x14075a8fc 440f29942400010000 movaps xmmword ptr [rsp + 0x100], xmm10
0x14075a905 440f298c2410010000 movaps xmmword ptr [rsp + 0x110], xmm9
0x14075a90e 440f29842420010000 movaps xmmword ptr [rsp + 0x120], xmm8
0x14075a917 0f29bc2430010000 movaps xmmword ptr [rsp + 0x130], xmm7
0x14075a91f 660f7fb42440010000 movdqa xmmword ptr [rsp + 0x140], xmm6
0x14075a928 4883bde002000000 cmp qword ptr [rbp + 0x2e0], 0
0x14075a930 741d je 0x14075a94f
0x14075a932 488b95e0020000 mov rdx, qword ptr [rbp + 0x2e0]
0x14075a939 48c1e202 shl rdx, 2
0x14075a93d 41b804000000 mov r8d, 4
0x14075a943 488b8de0030000 mov rcx, qword ptr [rbp + 0x3e0]
0x14075a94a e8d192ebff call 0x140613c20
0x14075a94f 660f6fb42440010000 movdqa xmm6, xmmword ptr [rsp + 0x140]
0x14075a958 0f28bc2430010000 movaps xmm7, xmmword ptr [rsp + 0x130]
0x14075a960 440f28842420010000 movaps xmm8, xmmword ptr [rsp + 0x120]
0x14075a969 440f288c2410010000 movaps xmm9, xmmword ptr [rsp + 0x110]
0x14075a972 440f28942400010000 movaps xmm10, xmmword ptr [rsp + 0x100]
0x14075a97b 440f289c24f0000000 movaps xmm11, xmmword ptr [rsp + 0xf0]
0x14075a984 440f28a424e0000000 movaps xmm12, xmmword ptr [rsp + 0xe0]
0x14075a98d 440f28ac24d0000000 movaps xmm13, xmmword ptr [rsp + 0xd0]
0x14075a996 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
0x14075a99f 440f28bc24b0000000 movaps xmm15, xmmword ptr [rsp + 0xb0]
0x14075a9a8 4881c458010000 add rsp, 0x158
0x14075a9af 5b pop rbx
0x14075a9b0 5f pop rdi
0x14075a9b1 5e pop rsi
0x14075a9b2 415c pop r12
0x14075a9b4 415d pop r13
0x14075a9b6 415e pop r14
0x14075a9b8 415f pop r15
0x14075a9ba 5d pop rbp
0x14075a9bb c3 ret 
0x14075a9bc 0f1f4000 nop dword ptr [rax]
0x14075a9c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14075a9c5 55 push rbp
0x14075a9c6 4157 push r15
0x14075a9c8 4156 push r14
0x14075a9ca 4155 push r13
0x14075a9cc 4154 push r12
0x14075a9ce 56 push rsi
0x14075a9cf 57 push rdi
0x14075a9d0 53 push rbx
0x14075a9d1 4881ec58010000 sub rsp, 0x158
0x14075a9d8 488daa80000000 lea rbp, [rdx + 0x80]
0x14075a9df 440f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm15
0x14075a9e8 440f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm14
0x14075a9f1 440f29ac24d0000000 movaps xmmword ptr [rsp + 0xd0], xmm13
0x14075a9fa 440f29a424e0000000 movaps xmmword ptr [rsp + 0xe0], xmm12
0x14075aa03 440f299c24f0000000 movaps xmmword ptr [rsp + 0xf0], xmm11
0x14075aa0c 440f29942400010000 movaps xmmword ptr [rsp + 0x100], xmm10
0x14075aa15 440f298c2410010000 movaps xmmword ptr [rsp + 0x110], xmm9
0x14075aa1e 440f29842420010000 movaps xmmword ptr [rsp + 0x120], xmm8
0x14075aa27 0f29bc2430010000 movaps xmmword ptr [rsp + 0x130], xmm7
0x14075aa2f 660f7fb42440010000 movdqa xmmword ptr [rsp + 0x140], xmm6
0x14075aa38 4883bdd803000000 cmp qword ptr [rbp + 0x3d8], 0
0x14075aa40 741d je 0x14075aa5f
0x14075aa42 488b95d8030000 mov rdx, qword ptr [rbp + 0x3d8]
0x14075aa49 48c1e202 shl rdx, 2
0x14075aa4d 41b804000000 mov r8d, 4
0x14075aa53 488b8d00040000 mov rcx, qword ptr [rbp + 0x400]
0x14075aa5a e8c191ebff call 0x140613c20
0x14075aa5f c6853704000001 mov byte ptr [rbp + 0x437], 1
0x14075aa66 660f6fb42440010000 movdqa xmm6, xmmword ptr [rsp + 0x140]
0x14075aa6f 0f28bc2430010000 movaps xmm7, xmmword ptr [rsp + 0x130]
0x14075aa77 440f28842420010000 movaps xmm8, xmmword ptr [rsp + 0x120]
0x14075aa80 440f288c2410010000 movaps xmm9, xmmword ptr [rsp + 0x110]
0x14075aa89 440f28942400010000 movaps xmm10, xmmword ptr [rsp + 0x100]
0x14075aa92 66440f6f9c24f0000000 movdqa xmm11, xmmword ptr [rsp + 0xf0]
0x14075aa9c 66440f6fa424e0000000 movdqa xmm12, xmmword ptr [rsp + 0xe0]
0x14075aaa6 66440f6fac24d0000000 movdqa xmm13, xmmword ptr [rsp + 0xd0]
0x14075aab0 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
0x14075aab9 440f28bc24b0000000 movaps xmm15, xmmword ptr [rsp + 0xb0]
0x14075aac2 4881c458010000 add rsp, 0x158
0x14075aac9 5b pop rbx
0x14075aaca 5f pop rdi
0x14075aacb 5e pop rsi
0x14075aacc 415c pop r12
0x14075aace 415d pop r13
0x14075aad0 415e pop r14
0x14075aad2 415f pop r15
0x14075aad4 5d pop rbp
0x14075aad5 c3 ret 
0x14075aad6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14075aae0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14075aae5 55 push rbp
0x14075aae6 4157 push r15
0x14075aae8 4156 push r14
0x14075aaea 4155 push r13
0x14075aaec 4154 push r12
0x14075aaee 56 push rsi
0x14075aaef 57 push rdi
0x14075aaf0 53 push rbx
0x14075aaf1 4881ec58010000 sub rsp, 0x158
0x14075aaf8 488daa80000000 lea rbp, [rdx + 0x80]
0x14075aaff 440f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm15
0x14075ab08 440f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm14
0x14075ab11 66440f7fac24d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm13
0x14075ab1b 66440f7fa424e0000000 movdqa xmmword ptr [rsp + 0xe0], xmm12
0x14075ab25 66440f7f9c24f0000000 movdqa xmmword ptr [rsp + 0xf0], xmm11
0x14075ab2f 440f29942400010000 movaps xmmword ptr [rsp + 0x100], xmm10
0x14075ab38 440f298c2410010000 movaps xmmword ptr [rsp + 0x110], xmm9
0x14075ab41 440f29842420010000 movaps xmmword ptr [rsp + 0x120], xmm8
0x14075ab4a 0f29bc2430010000 movaps xmmword ptr [rsp + 0x130], xmm7
0x14075ab52 660f7fb42440010000 movdqa xmmword ptr [rsp + 0x140], xmm6
0x14075ab5b 80bd3204000000 cmp byte ptr [rbp + 0x432], 0
0x14075ab62 0f84a9000000 je 0x14075ac11
0x14075ab68 488b85a0000000 mov rax, qword ptr [rbp + 0xa0]
0x14075ab6f 4885c0 test rax, rax
0x14075ab72 741a je 0x14075ab8e
0x14075ab74 488b8da8000000 mov rcx, qword ptr [rbp + 0xa8]
0x14075ab7b 48c1e005 shl rax, 5
0x14075ab7f 488d1440 lea rdx, [rax + rax*2]
0x14075ab83 41b804000000 mov r8d, 4
0x14075ab89 e89290ebff call 0x140613c20
0x14075ab8e 488d8d30020000 lea rcx, [rbp + 0x230]
0x14075ab95 e886079fff call 0x14014b320
0x14075ab9a c6853504000000 mov byte ptr [rbp + 0x435], 0
0x14075aba1 660f6fb42440010000 movdqa xmm6, xmmword ptr [rsp + 0x140]
0x14075abaa 0f28bc2430010000 movaps xmm7, xmmword ptr [rsp + 0x130]
0x14075abb2 440f28842420010000 movaps xmm8, xmmword ptr [rsp + 0x120]
0x14075abbb 440f288c2410010000 movaps xmm9, xmmword ptr [rsp + 0x110]
0x14075abc4 440f28942400010000 movaps xmm10, xmmword ptr [rsp + 0x100]
0x14075abcd 66440f6f9c24f0000000 movdqa xmm11, xmmword ptr [rsp + 0xf0]
0x14075abd7 66440f6fa424e0000000 movdqa xmm12, xmmword ptr [rsp + 0xe0]
0x14075abe1 66440f6fac24d0000000 movdqa xmm13, xmmword ptr [rsp + 0xd0]
0x14075abeb 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
0x14075abf4 440f28bc24b0000000 movaps xmm15, xmmword ptr [rsp + 0xb0]
0x14075abfd 4881c458010000 add rsp, 0x158
0x14075ac04 5b pop rbx
0x14075ac05 5f pop rdi
0x14075ac06 5e pop rsi
0x14075ac07 415c pop r12
0x14075ac09 415d pop r13
0x14075ac0b 415e pop r14
0x14075ac0d 415f pop r15
0x14075ac0f 5d pop rbp
0x14075ac10 c3 ret 
0x14075ac11 c6853504000000 mov byte ptr [rbp + 0x435], 0
0x14075ac18 660f6fb42440010000 movdqa xmm6, xmmword ptr [rsp + 0x140]
0x14075ac21 0f28bc2430010000 movaps xmm7, xmmword ptr [rsp + 0x130]
0x14075ac29 440f28842420010000 movaps xmm8, xmmword ptr [rsp + 0x120]
0x14075ac32 440f288c2410010000 movaps xmm9, xmmword ptr [rsp + 0x110]
0x14075ac3b 440f28942400010000 movaps xmm10, xmmword ptr [rsp + 0x100]
0x14075ac44 66440f6f9c24f0000000 movdqa xmm11, xmmword ptr [rsp + 0xf0]
0x14075ac4e 66440f6fa424e0000000 movdqa xmm12, xmmword ptr [rsp + 0xe0]
0x14075ac58 66440f6fac24d0000000 movdqa xmm13, xmmword ptr [rsp + 0xd0]
0x14075ac62 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
0x14075ac6b 440f28bc24b0000000 movaps xmm15, xmmword ptr [rsp + 0xb0]
0x14075ac74 4881c458010000 add rsp, 0x158
0x14075ac7b 5b pop rbx
0x14075ac7c 5f pop rdi
0x14075ac7d 5e pop rsi
0x14075ac7e 415c pop r12
0x14075ac80 415d pop r13
0x14075ac82 415e pop r14
0x14075ac84 415f pop r15
0x14075ac86 5d pop rbp
0x14075ac87 c3 ret 
0x14075ac88 0f1f840000000000 nop dword ptr [rax + rax]
0x14075ac90 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14075ac95 55 push rbp
0x14075ac96 4157 push r15
0x14075ac98 4156 push r14
0x14075ac9a 4155 push r13
0x14075ac9c 4154 push r12
0x14075ac9e 56 push rsi
0x14075ac9f 57 push rdi
0x14075aca0 53 push rbx
0x14075aca1 4881ec58010000 sub rsp, 0x158
0x14075aca8 488daa80000000 lea rbp, [rdx + 0x80]
0x14075acaf 440f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm15
0x14075acb8 440f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm14
0x14075acc1 66440f7fac24d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm13
0x14075accb 66440f7fa424e0000000 movdqa xmmword ptr [rsp + 0xe0], xmm12
0x14075acd5 66440f7f9c24f0000000 movdqa xmmword ptr [rsp + 0xf0], xmm11
0x14075acdf 440f29942400010000 movaps xmmword ptr [rsp + 0x100], xmm10
0x14075ace8 440f298c2410010000 movaps xmmword ptr [rsp + 0x110], xmm9
0x14075acf1 440f29842420010000 movaps xmmword ptr [rsp + 0x120], xmm8
0x14075acfa 0f29bc2430010000 movaps xmmword ptr [rsp + 0x130], xmm7
0x14075ad02 660f7fb42440010000 movdqa xmmword ptr [rsp + 0x140], xmm6
0x14075ad0b 0fb69d35040000 movzx ebx, byte ptr [rbp + 0x435]
0x14075ad12 f6c301 test bl, 1
0x14075ad15 0f8482000000 je 0x14075ad9d
0x14075ad1b 488d8d40010000 lea rcx, [rbp + 0x140]
0x14075ad22 e8f9059fff call 0x14014b320
0x14075ad27 889d37040000 mov byte ptr [rbp + 0x437], bl
0x14075ad2d 660f6fb42440010000 movdqa xmm6, xmmword ptr [rsp + 0x140]
0x14075ad36 0f28bc2430010000 movaps xmm7, xmmword ptr [rsp + 0x130]
0x14075ad3e 440f28842420010000 movaps xmm8, xmmword ptr [rsp + 0x120]
0x14075ad47 440f288c2410010000 movaps xmm9, xmmword ptr [rsp + 0x110]
0x14075ad50 440f28942400010000 movaps xmm10, xmmword ptr [rsp + 0x100]
0x14075ad59 66440f6f9c24f0000000 movdqa xmm11, xmmword ptr [rsp + 0xf0]
0x14075ad63 66440f6fa424e0000000 movdqa xmm12, xmmword ptr [rsp + 0xe0]
0x14075ad6d 66440f6fac24d0000000 movdqa xmm13, xmmword ptr [rsp + 0xd0]
0x14075ad77 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
0x14075ad80 440f28bc24b0000000 movaps xmm15, xmmword ptr [rsp + 0xb0]
0x14075ad89 4881c458010000 add rsp, 0x158
0x14075ad90 5b pop rbx
0x14075ad91 5f pop rdi
0x14075ad92 5e pop rsi
0x14075ad93 415c pop r12
0x14075ad95 415d pop r13
0x14075ad97 415e pop r14
0x14075ad99 415f pop r15
0x14075ad9b 5d pop rbp
0x14075ad9c c3 ret 
0x14075ad9d 889d37040000 mov byte ptr [rbp + 0x437], bl
0x14075ada3 660f6fb42440010000 movdqa xmm6, xmmword ptr [rsp + 0x140]
0x14075adac 0f28bc2430010000 movaps xmm7, xmmword ptr [rsp + 0x130]
0x14075adb4 440f28842420010000 movaps xmm8, xmmword ptr [rsp + 0x120]
0x14075adbd 440f288c2410010000 movaps xmm9, xmmword ptr [rsp + 0x110]
0x14075adc6 440f28942400010000 movaps xmm10, xmmword ptr [rsp + 0x100]
0x14075adcf 66440f6f9c24f0000000 movdqa xmm11, xmmword ptr [rsp + 0xf0]
0x14075add9 66440f6fa424e0000000 movdqa xmm12, xmmword ptr [rsp + 0xe0]
0x14075ade3 66440f6fac24d0000000 movdqa xmm13, xmmword ptr [rsp + 0xd0]
0x14075aded 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
0x14075adf6 440f28bc24b0000000 movaps xmm15, xmmword ptr [rsp + 0xb0]
0x14075adff 4881c458010000 add rsp, 0x158
0x14075ae06 5b pop rbx
0x14075ae07 5f pop rdi
0x14075ae08 5e pop rsi
0x14075ae09 415c pop r12
0x14075ae0b 415d pop r13
0x14075ae0d 415e pop r14
0x14075ae0f 415f pop r15
0x14075ae11 5d pop rbp
0x14075ae12 c3 ret 
0x14075ae13 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14075ae20 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14075ae25 55 push rbp
0x14075ae26 4157 push r15
0x14075ae28 4156 push r14
0x14075ae2a 4155 push r13
0x14075ae2c 4154 push r12
0x14075ae2e 56 push rsi
0x14075ae2f 57 push rdi
0x14075ae30 53 push rbx
0x14075ae31 4881ec58010000 sub rsp, 0x158
0x14075ae38 488daa80000000 lea rbp, [rdx + 0x80]
0x14075ae3f 440f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm15
0x14075ae48 440f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm14
0x14075ae51 66440f7fac24d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm13
0x14075ae5b 66440f7fa424e0000000 movdqa xmmword ptr [rsp + 0xe0], xmm12
0x14075ae65 66440f7f9c24f0000000 movdqa xmmword ptr [rsp + 0xf0], xmm11
0x14075ae6f 440f29942400010000 movaps xmmword ptr [rsp + 0x100], xmm10
0x14075ae78 440f298c2410010000 movaps xmmword ptr [rsp + 0x110], xmm9
0x14075ae81 440f29842420010000 movaps xmmword ptr [rsp + 0x120], xmm8
0x14075ae8a 0f29bc2430010000 movaps xmmword ptr [rsp + 0x130], xmm7
0x14075ae92 660f7fb42440010000 movdqa xmmword ptr [rsp + 0x140], xmm6
0x14075ae9b f6853704000001 test byte ptr [rbp + 0x437], 1
0x14075aea2 0f8492000000 je 0x14075af3a
0x14075aea8 488b85b0020000 mov rax, qword ptr [rbp + 0x2b0]
0x14075aeaf 4885c0 test rax, rax
0x14075aeb2 741a je 0x14075aece
0x14075aeb4 488b8db8020000 mov rcx, qword ptr [rbp + 0x2b8]
0x14075aebb 48c1e005 shl rax, 5
0x14075aebf 488d1440 lea rdx, [rax + rax*2]
0x14075aec3 41b804000000 mov r8d, 4
0x14075aec9 e8528debff call 0x140613c20
0x14075aece 0f28b42440010000 movaps xmm6, xmmword ptr [rsp + 0x140]
0x14075aed6 0f28bc2430010000 movaps xmm7, xmmword ptr [rsp + 0x130]
0x14075aede 440f28842420010000 movaps xmm8, xmmword ptr [rsp + 0x120]
0x14075aee7 440f288c2410010000 movaps xmm9, xmmword ptr [rsp + 0x110]
0x14075aef0 440f28942400010000 movaps xmm10, xmmword ptr [rsp + 0x100]
0x14075aef9 440f289c24f0000000 movaps xmm11, xmmword ptr [rsp + 0xf0]
0x14075af02 440f28a424e0000000 movaps xmm12, xmmword ptr [rsp + 0xe0]
0x14075af0b 440f28ac24d0000000 movaps xmm13, xmmword ptr [rsp + 0xd0]
0x14075af14 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
0x14075af1d 440f28bc24b0000000 movaps xmm15, xmmword ptr [rsp + 0xb0]
0x14075af26 4881c458010000 add rsp, 0x158
0x14075af2d 5b pop rbx
0x14075af2e 5f pop rdi
0x14075af2f 5e pop rsi
0x14075af30 415c pop r12
0x14075af32 415d pop r13
0x14075af34 415e pop r14
0x14075af36 415f pop r15
0x14075af38 5d pop rbp
0x14075af39 c3 ret 
0x14075af3a 0f28b42440010000 movaps xmm6, xmmword ptr [rsp + 0x140]
0x14075af42 0f28bc2430010000 movaps xmm7, xmmword ptr [rsp + 0x130]
0x14075af4a 440f28842420010000 movaps xmm8, xmmword ptr [rsp + 0x120]
0x14075af53 440f288c2410010000 movaps xmm9, xmmword ptr [rsp + 0x110]
0x14075af5c 440f28942400010000 movaps xmm10, xmmword ptr [rsp + 0x100]
0x14075af65 440f289c24f0000000 movaps xmm11, xmmword ptr [rsp + 0xf0]
0x14075af6e 440f28a424e0000000 movaps xmm12, xmmword ptr [rsp + 0xe0]
0x14075af77 440f28ac24d0000000 movaps xmm13, xmmword ptr [rsp + 0xd0]
0x14075af80 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
0x14075af89 440f28bc24b0000000 movaps xmm15, xmmword ptr [rsp + 0xb0]
0x14075af92 4881c458010000 add rsp, 0x158
0x14075af99 5b pop rbx
0x14075af9a 5f pop rdi
0x14075af9b 5e pop rsi
0x14075af9c 415c pop r12
0x14075af9e 415d pop r13
0x14075afa0 415e pop r14
0x14075afa2 415f pop r15
0x14075afa4 5d pop rbp
0x14075afa5 c3 ret 
0x14075afa6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14075afb0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14075afb5 55 push rbp
0x14075afb6 4157 push r15
0x14075afb8 4156 push r14
0x14075afba 4155 push r13
0x14075afbc 4154 push r12
0x14075afbe 56 push rsi
0x14075afbf 57 push rdi
0x14075afc0 53 push rbx
0x14075afc1 4881ec58010000 sub rsp, 0x158
0x14075afc8 488daa80000000 lea rbp, [rdx + 0x80]
0x14075afcf 440f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm15
0x14075afd8 440f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm14
0x14075afe1 66440f7fac24d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm13
0x14075afeb 66440f7fa424e0000000 movdqa xmmword ptr [rsp + 0xe0], xmm12
0x14075aff5 66440f7f9c24f0000000 movdqa xmmword ptr [rsp + 0xf0], xmm11
0x14075afff 440f29942400010000 movaps xmmword ptr [rsp + 0x100], xmm10
0x14075b008 440f298c2410010000 movaps xmmword ptr [rsp + 0x110], xmm9
0x14075b011 440f29842420010000 movaps xmmword ptr [rsp + 0x120], xmm8
0x14075b01a 0f29bc2430010000 movaps xmmword ptr [rsp + 0x130], xmm7
0x14075b022 660f7fb42440010000 movdqa xmmword ptr [rsp + 0x140], xmm6
0x14075b02b 488b9508020000 mov rdx, qword ptr [rbp + 0x208]
0x14075b032 4885d2 test rdx, rdx
0x14075b035 7416 je 0x14075b04d
0x14075b037 48c1e203 shl rdx, 3
0x14075b03b 41b808000000 mov r8d, 8
0x14075b041 488b8d08040000 mov rcx, qword ptr [rbp + 0x408]
0x14075b048 e8d38bebff call 0x140613c20
0x14075b04d 488b95b0010000 mov rdx, qword ptr [rbp + 0x1b0]
0x14075b054 4885d2 test rdx, rdx
0x14075b057 7416 je 0x14075b06f
0x14075b059 488b8db8010000 mov rcx, qword ptr [rbp + 0x1b8]
0x14075b060 48c1e204 shl rdx, 4
0x14075b064 41b804000000 mov r8d, 4
0x14075b06a e8b18bebff call 0x140613c20
0x14075b06f c6853304000000 mov byte ptr [rbp + 0x433], 0
0x14075b076 660f6fb42440010000 movdqa xmm6, xmmword ptr [rsp + 0x140]
0x14075b07f 0f28bc2430010000 movaps xmm7, xmmword ptr [rsp + 0x130]
0x14075b087 440f28842420010000 movaps xmm8, xmmword ptr [rsp + 0x120]
0x14075b090 440f288c2410010000 movaps xmm9, xmmword ptr [rsp + 0x110]
0x14075b099 440f28942400010000 movaps xmm10, xmmword ptr [rsp + 0x100]
0x14075b0a2 66440f6f9c24f0000000 movdqa xmm11, xmmword ptr [rsp + 0xf0]
0x14075b0ac 66440f6fa424e0000000 movdqa xmm12, xmmword ptr [rsp + 0xe0]
0x14075b0b6 66440f6fac24d0000000 movdqa xmm13, xmmword ptr [rsp + 0xd0]
0x14075b0c0 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
0x14075b0c9 440f28bc24b0000000 movaps xmm15, xmmword ptr [rsp + 0xb0]
0x14075b0d2 4881c458010000 add rsp, 0x158
0x14075b0d9 5b pop rbx
0x14075b0da 5f pop rdi
0x14075b0db 5e pop rsi
0x14075b0dc 415c pop r12
0x14075b0de 415d pop r13
0x14075b0e0 415e pop r14
0x14075b0e2 415f pop r15
0x14075b0e4 5d pop rbp
0x14075b0e5 c3 ret 
0x14075b0e6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14075b0f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14075b0f5 55 push rbp
0x14075b0f6 4157 push r15
0x14075b0f8 4156 push r14
0x14075b0fa 4155 push r13
0x14075b0fc 4154 push r12
0x14075b0fe 56 push rsi
0x14075b0ff 57 push rdi
0x14075b100 53 push rbx
0x14075b101 4881ec58010000 sub rsp, 0x158
0x14075b108 488daa80000000 lea rbp, [rdx + 0x80]
0x14075b10f 440f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm15
0x14075b118 440f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm14
0x14075b121 66440f7fac24d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm13
0x14075b12b 66440f7fa424e0000000 movdqa xmmword ptr [rsp + 0xe0], xmm12
0x14075b135 66440f7f9c24f0000000 movdqa xmmword ptr [rsp + 0xf0], xmm11
0x14075b13f 440f29942400010000 movaps xmmword ptr [rsp + 0x100], xmm10
0x14075b148 440f298c2410010000 movaps xmmword ptr [rsp + 0x110], xmm9
0x14075b151 440f29842420010000 movaps xmmword ptr [rsp + 0x120], xmm8
0x14075b15a 0f29bc2430010000 movaps xmmword ptr [rsp + 0x130], xmm7
0x14075b162 660f7fb42440010000 movdqa xmmword ptr [rsp + 0x140], xmm6
0x14075b16b 80bd3304000000 cmp byte ptr [rbp + 0x433], 0
0x14075b172 0f8492000000 je 0x14075b20a
0x14075b178 488b95b0030000 mov rdx, qword ptr [rbp + 0x3b0]
0x14075b17f 4885d2 test rdx, rdx
0x14075b182 7416 je 0x14075b19a
0x14075b184 488b8db8030000 mov rcx, qword ptr [rbp + 0x3b8]
0x14075b18b 48c1e203 shl rdx, 3
0x14075b18f 41b808000000 mov r8d, 8
0x14075b195 e8868aebff call 0x140613c20
0x14075b19a 660f6fb42440010000 movdqa xmm6, xmmword ptr [rsp + 0x140]
0x14075b1a3 0f28bc2430010000 movaps xmm7, xmmword ptr [rsp + 0x130]
0x14075b1ab 440f28842420010000 movaps xmm8, xmmword ptr [rsp + 0x120]
0x14075b1b4 440f288c2410010000 movaps xmm9, xmmword ptr [rsp + 0x110]
0x14075b1bd 440f28942400010000 movaps xmm10, xmmword ptr [rsp + 0x100]
0x14075b1c6 66440f6f9c24f0000000 movdqa xmm11, xmmword ptr [rsp + 0xf0]
0x14075b1d0 66440f6fa424e0000000 movdqa xmm12, xmmword ptr [rsp + 0xe0]
0x14075b1da 66440f6fac24d0000000 movdqa xmm13, xmmword ptr [rsp + 0xd0]
0x14075b1e4 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
0x14075b1ed 440f28bc24b0000000 movaps xmm15, xmmword ptr [rsp + 0xb0]
0x14075b1f6 4881c458010000 add rsp, 0x158
0x14075b1fd 5b pop rbx
0x14075b1fe 5f pop rdi
0x14075b1ff 5e pop rsi
0x14075b200 415c pop r12
0x14075b202 415d pop r13
0x14075b204 415e pop r14
0x14075b206 415f pop r15
0x14075b208 5d pop rbp
0x14075b209 c3 ret 
0x14075b20a 660f6fb42440010000 movdqa xmm6, xmmword ptr [rsp + 0x140]
0x14075b213 0f28bc2430010000 movaps xmm7, xmmword ptr [rsp + 0x130]
0x14075b21b 440f28842420010000 movaps xmm8, xmmword ptr [rsp + 0x120]
0x14075b224 440f288c2410010000 movaps xmm9, xmmword ptr [rsp + 0x110]
0x14075b22d 440f28942400010000 movaps xmm10, xmmword ptr [rsp + 0x100]
0x14075b236 66440f6f9c24f0000000 movdqa xmm11, xmmword ptr [rsp + 0xf0]
0x14075b240 66440f6fa424e0000000 movdqa xmm12, xmmword ptr [rsp + 0xe0]
0x14075b24a 66440f6fac24d0000000 movdqa xmm13, xmmword ptr [rsp + 0xd0]
0x14075b254 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
0x14075b25d 440f28bc24b0000000 movaps xmm15, xmmword ptr [rsp + 0xb0]
0x14075b266 4881c458010000 add rsp, 0x158
0x14075b26d 5b pop rbx
0x14075b26e 5f pop rdi
0x14075b26f 5e pop rsi
0x14075b270 415c pop r12
0x14075b272 415d pop r13
0x14075b274 415e pop r14
0x14075b276 415f pop r15
0x14075b278 5d pop rbp
0x14075b279 c3 ret 
0x14075b27a 660f1f440000 nop word ptr [rax + rax]
0x14075b280 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14075b285 55 push rbp
0x14075b286 4157 push r15
0x14075b288 4156 push r14
0x14075b28a 4155 push r13
0x14075b28c 4154 push r12
0x14075b28e 56 push rsi
0x14075b28f 57 push rdi
0x14075b290 53 push rbx
0x14075b291 4881ec58010000 sub rsp, 0x158
0x14075b298 488daa80000000 lea rbp, [rdx + 0x80]
0x14075b29f 440f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm15
0x14075b2a8 440f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm14
0x14075b2b1 66440f7fac24d0000000 movdqa xmmword ptr [rsp + 0xd0], xmm13
0x14075b2bb 66440f7fa424e0000000 movdqa xmmword ptr [rsp + 0xe0], xmm12
0x14075b2c5 66440f7f9c24f0000000 movdqa xmmword ptr [rsp + 0xf0], xmm11
0x14075b2cf 440f29942400010000 movaps xmmword ptr [rsp + 0x100], xmm10
0x14075b2d8 440f298c2410010000 movaps xmmword ptr [rsp + 0x110], xmm9
0x14075b2e1 440f29842420010000 movaps xmmword ptr [rsp + 0x120], xmm8
0x14075b2ea 0f29bc2430010000 movaps xmmword ptr [rsp + 0x130], xmm7
0x14075b2f2 660f7fb42440010000 movdqa xmmword ptr [rsp + 0x140], xmm6
0x14075b2fb 80bd3104000000 cmp byte ptr [rbp + 0x431], 0
0x14075b302 743d je 0x14075b341
0x14075b304 e827ce4b00 call 0x140c18130
0x14075b309 4889c6 mov rsi, rax
0x14075b30c 488b00 mov rax, qword ptr [rax]
0x14075b30f 4883f801 cmp rax, 1
0x14075b313 0f8594000000 jne 0x14075b3ad
0x14075b319 4883c608 add rsi, 8
0x14075b31d 488b95e8020000 mov rdx, qword ptr [rbp + 0x2e8]
0x14075b324 48833e00 cmp qword ptr [rsi], 0
0x14075b328 0f85ad000000 jne 0x14075b3db
0x14075b32e 48c706ffffffff mov qword ptr [rsi], 0xffffffffffffffff
0x14075b335 488d4e08 lea rcx, [rsi + 8]
0x14075b339 e8b2bd4b00 call 0x140c170f0
0x14075b33e 48ff06 inc qword ptr [rsi]
0x14075b341 0f28b42440010000 movaps xmm6, xmmword ptr [rsp + 0x140]
0x14075b349 0f28bc2430010000 movaps xmm7, xmmword ptr [rsp + 0x130]
0x14075b351 440f28842420010000 movaps xmm8, xmmword ptr [rsp + 0x120]
0x14075b35a 440f288c2410010000 movaps xmm9, xmmword ptr [rsp + 0x110]
0x14075b363 440f28942400010000 movaps xmm10, xmmword ptr [rsp + 0x100]
0x14075b36c 440f289c24f0000000 movaps xmm11, xmmword ptr [rsp + 0xf0]
0x14075b375 440f28a424e0000000 movaps xmm12, xmmword ptr [rsp + 0xe0]
0x14075b37e 440f28ac24d0000000 movaps xmm13, xmmword ptr [rsp + 0xd0]
0x14075b387 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
0x14075b390 440f28bc24b0000000 movaps xmm15, xmmword ptr [rsp + 0xb0]
0x14075b399 4881c458010000 add rsp, 0x158
0x14075b3a0 5b pop rbx
0x14075b3a1 5f pop rdi
0x14075b3a2 5e pop rsi
0x14075b3a3 415c pop r12
0x14075b3a5 415d pop r13
0x14075b3a7 415e pop r14
0x14075b3a9 415f pop r15
0x14075b3ab 5d pop rbp
0x14075b3ac c3 ret 
0x14075b3ad 4885c0 test rax, rax
0x14075b3b0 751d jne 0x14075b3cf
0x14075b3b2 4889f1 mov rcx, rsi
0x14075b3b5 31d2 xor edx, edx
0x14075b3b7 e804621902 call 0x1428f15c0
0x14075b3bc 488b95e8020000 mov rdx, qword ptr [rbp + 0x2e8]
0x14075b3c3 4889c6 mov rsi, rax
0x14075b3c6 4885c0 test rax, rax
0x14075b3c9 0f8555ffffff jne 0x14075b324
0x14075b3cf 488d0dd2e04b02 lea rcx, [rip + 0x24be0d2]
0x14075b3d6 e8f5cc1702 call 0x1428d80d0
0x14075b3db 488d0d36e14b02 lea rcx, [rip + 0x24be136]
0x14075b3e2 e869de1702 call 0x1428d9250
0x14075b3e7 cc int3 
