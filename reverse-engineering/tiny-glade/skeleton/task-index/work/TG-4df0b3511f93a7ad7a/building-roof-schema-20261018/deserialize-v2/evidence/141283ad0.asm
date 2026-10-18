0x141283ad0 4157 push r15
0x141283ad2 4156 push r14
0x141283ad4 4155 push r13
0x141283ad6 4154 push r12
0x141283ad8 56 push rsi
0x141283ad9 57 push rdi
0x141283ada 55 push rbp
0x141283adb 53 push rbx
0x141283adc 4881ec88000000 sub rsp, 0x88
0x141283ae3 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
0x141283ae9 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
0x141283aee 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
0x141283af3 4889ce mov rsi, rcx
0x141283af6 0fb602 movzx eax, byte ptr [rdx]
0x141283af9 83f814 cmp eax, 0x14
0x141283afc 7453 je 0x141283b51
0x141283afe 83f815 cmp eax, 0x15
0x141283b01 0f856e030000 jne 0x141283e75
0x141283b07 4c8b7218 mov r14, qword ptr [rdx + 0x18]
0x141283b0b 4d85f6 test r14, r14
0x141283b0e 0f8455030000 je 0x141283e69
0x141283b14 488b7a10 mov rdi, qword ptr [rdx + 0x10]
0x141283b18 4883c720 add rdi, 0x20
0x141283b1c 49c1e606 shl r14, 6
0x141283b20 31c0 xor eax, eax
0x141283b22 4c8d3d83409a01 lea r15, [rip + 0x19a4083]
0x141283b29 488d5c2430 lea rbx, [rsp + 0x30]
0x141283b2e 4c8d25b3409a01 lea r12, [rip + 0x19a40b3]
0x141283b35 4c8d2d34ae6a01 lea r13, [rip + 0x16aae34]
0x141283b3c 0f2835ad1d6a01 movaps xmm6, xmmword ptr [rip + 0x16a1dad]
0x141283b43 0f283d36ae6a01 movaps xmm7, xmmword ptr [rip + 0x16aae36]
0x141283b4a 31ed xor ebp, ebp
0x141283b4c e9ad000000 jmp 0x141283bfe
0x141283b51 488b5a18 mov rbx, qword ptr [rdx + 0x18]
0x141283b55 4885db test rbx, rbx
0x141283b58 0f8433030000 je 0x141283e91
0x141283b5e 488b7a10 mov rdi, qword ptr [rdx + 0x10]
0x141283b62 488d4c2430 lea rcx, [rsp + 0x30]
0x141283b67 4889fa mov rdx, rdi
0x141283b6a e8f10d0000 call 0x141284960
0x141283b6f 837c243001 cmp dword ptr [rsp + 0x30], 1
0x141283b74 7514 jne 0x141283b8a
0x141283b76 488b442438 mov rax, qword ptr [rsp + 0x38]
0x141283b7b 48894608 mov qword ptr [rsi + 8], rax
0x141283b7f c70601000000 mov dword ptr [rsi], 1
0x141283b85 e9d6030000 jmp 0x141283f60
0x141283b8a f20f10442434 movsd xmm0, qword ptr [rsp + 0x34]
0x141283b90 f20f11442440 movsd qword ptr [rsp + 0x40], xmm0
0x141283b96 4883fb01 cmp rbx, 1
0x141283b9a 0f840e040000 je 0x141283fae
0x141283ba0 0fb65720 movzx edx, byte ptr [rdi + 0x20]
0x141283ba4 ffca dec edx
0x141283ba6 83fa09 cmp edx, 9
0x141283ba9 0f8746040000 ja 0x141283ff5
0x141283baf 488b442440 mov rax, qword ptr [rsp + 0x40]
0x141283bb4 4889c1 mov rcx, rax
0x141283bb7 48c1e920 shr rcx, 0x20
0x141283bbb 4c8d054e409a01 lea r8, [rip + 0x19a404e]
0x141283bc2 49631490 movsxd rdx, dword ptr [r8 + rdx*4]
0x141283bc6 4c01c2 add rdx, r8
0x141283bc9 ffe2 jmp rdx
0x141283bcb 0fb65721 movzx edx, byte ptr [rdi + 0x21]
0x141283bcf e949030000 jmp 0x141283f1d
0x141283bd4 0fbe4f01 movsx ecx, byte ptr [rdi + 1]
0x141283bd8 450f57c0 xorps xmm8, xmm8
0x141283bdc f3440f2ac1 cvtsi2ss xmm8, ecx
0x141283be1 bd01000000 mov ebp, 1
0x141283be6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x141283bf0 4883c740 add rdi, 0x40
0x141283bf4 4983c6c0 add r14, -0x40
0x141283bf8 0f8437020000 je 0x141283e35
0x141283bfe 0fb64fe0 movzx ecx, byte ptr [rdi - 0x20]
0x141283c02 ffc9 dec ecx
0x141283c04 83f90e cmp ecx, 0xe
0x141283c07 0f87db030000 ja 0x141283fe8
0x141283c0d 49630c8f movsxd rcx, dword ptr [r15 + rcx*4]
0x141283c11 4c01f9 add rcx, r15
0x141283c14 ffe1 jmp rcx
0x141283c16 0fb64fe1 movzx ecx, byte ptr [rdi - 0x1f]
0x141283c1a 80f901 cmp cl, 1
0x141283c1d 0f94c2 sete dl
0x141283c20 84c9 test cl, cl
0x141283c22 0f85f5000000 jne 0x141283d1d
0x141283c28 e925010000 jmp 0x141283d52
0x141283c2d 48837ff806 cmp qword ptr [rdi - 8], 6
0x141283c32 75bc jne 0x141283bf0
0x141283c34 488b4ff0 mov rcx, qword ptr [rdi - 0x10]
0x141283c38 0fb611 movzx edx, byte ptr [rcx]
0x141283c3b 83fa72 cmp edx, 0x72
0x141283c3e 743b je 0x141283c7b
0x141283c40 83fa6f cmp edx, 0x6f
0x141283c43 75ab jne 0x141283bf0
0x141283c45 80790172 cmp byte ptr [rcx + 1], 0x72
0x141283c49 75a5 jne 0x141283bf0
0x141283c4b 80790269 cmp byte ptr [rcx + 2], 0x69
0x141283c4f 759f jne 0x141283bf0
0x141283c51 80790367 cmp byte ptr [rcx + 3], 0x67
0x141283c55 7599 jne 0x141283bf0
0x141283c57 80790469 cmp byte ptr [rcx + 4], 0x69
0x141283c5b 7593 jne 0x141283bf0
0x141283c5d 8079056e cmp byte ptr [rcx + 5], 0x6e
0x141283c61 758d jne 0x141283bf0
0x141283c63 e9ea000000 jmp 0x141283d52
0x141283c68 48837ff006 cmp qword ptr [rdi - 0x10], 6
0x141283c6d 7581 jne 0x141283bf0
0x141283c6f 488b4fe8 mov rcx, qword ptr [rdi - 0x18]
0x141283c73 0fb611 movzx edx, byte ptr [rcx]
0x141283c76 83fa72 cmp edx, 0x72
0x141283c79 75c5 jne 0x141283c40
0x141283c7b 80790161 cmp byte ptr [rcx + 1], 0x61
0x141283c7f 0f856bffffff jne 0x141283bf0
0x141283c85 80790264 cmp byte ptr [rcx + 2], 0x64
0x141283c89 0f8561ffffff jne 0x141283bf0
0x141283c8f 80790369 cmp byte ptr [rcx + 3], 0x69
0x141283c93 0f8557ffffff jne 0x141283bf0
0x141283c99 80790475 cmp byte ptr [rcx + 4], 0x75
0x141283c9d 0f854dffffff jne 0x141283bf0
0x141283ca3 80790573 cmp byte ptr [rcx + 5], 0x73
0x141283ca7 0f8543ffffff jne 0x141283bf0
0x141283cad eb7b jmp 0x141283d2a
0x141283caf 48837ff006 cmp qword ptr [rdi - 0x10], 6
0x141283cb4 0f8536ffffff jne 0x141283bf0
0x141283cba 488b4fe8 mov rcx, qword ptr [rdi - 0x18]
0x141283cbe eb0f jmp 0x141283ccf
0x141283cc0 48837ff806 cmp qword ptr [rdi - 8], 6
0x141283cc5 0f8525ffffff jne 0x141283bf0
0x141283ccb 488b4ff0 mov rcx, qword ptr [rdi - 0x10]
0x141283ccf 8b11 mov edx, dword ptr [rcx]
0x141283cd1 3315febfa001 xor edx, dword ptr [rip + 0x1a0bffe]
0x141283cd7 440fb74104 movzx r8d, word ptr [rcx + 4]
0x141283cdc 66443305f5bfa001 xor r8w, word ptr [rip + 0x1a0bff5]
0x141283ce4 450fb7c0 movzx r8d, r8w
0x141283ce8 4109d0 or r8d, edx
0x141283ceb 7465 je 0x141283d52
0x141283ced 8b11 mov edx, dword ptr [rcx]
0x141283cef 33154bb5a001 xor edx, dword ptr [rip + 0x1a0b54b]
0x141283cf5 0fb74904 movzx ecx, word ptr [rcx + 4]
0x141283cf9 66330d44b5a001 xor cx, word ptr [rip + 0x1a0b544]
0x141283d00 0fb7c9 movzx ecx, cx
0x141283d03 09d1 or ecx, edx
0x141283d05 0f85e5feffff jne 0x141283bf0
0x141283d0b eb1d jmp 0x141283d2a
0x141283d0d 488b4fe8 mov rcx, qword ptr [rdi - 0x18]
0x141283d11 4883f901 cmp rcx, 1
0x141283d15 0f94c2 sete dl
0x141283d18 4885c9 test rcx, rcx
0x141283d1b 7435 je 0x141283d52
0x141283d1d b102 mov cl, 2
0x141283d1f 28d1 sub cl, dl
0x141283d21 80f902 cmp cl, 2
0x141283d24 0f84c6feffff je 0x141283bf0
0x141283d2a 85ed test ebp, ebp
0x141283d2c 0f85e9020000 jne 0x14128401b
0x141283d32 0fb60f movzx ecx, byte ptr [rdi]
0x141283d35 ffc9 dec ecx
0x141283d37 83f909 cmp ecx, 9
0x141283d3a 0f87b9020000 ja 0x141283ff9
0x141283d40 49630c8c movsxd rcx, dword ptr [r12 + rcx*4]
0x141283d44 4c01e1 add rcx, r12
0x141283d47 ffe1 jmp rcx
0x141283d49 0fb64f01 movzx ecx, byte ptr [rdi + 1]
0x141283d4d e986feffff jmp 0x141283bd8
0x141283d52 85c0 test eax, eax
0x141283d54 0f85b8020000 jne 0x141284012
0x141283d5a 4889d9 mov rcx, rbx
0x141283d5d 4889fa mov rdx, rdi
0x141283d60 e8fb0b0000 call 0x141284960
0x141283d65 f644243001 test byte ptr [rsp + 0x30], 1
0x141283d6a 0f8506feffff jne 0x141283b76
0x141283d70 f30f10442434 movss xmm0, dword ptr [rsp + 0x34]
0x141283d76 f30f104c2438 movss xmm1, dword ptr [rsp + 0x38]
0x141283d7c b801000000 mov eax, 1
0x141283d81 e96afeffff jmp 0x141283bf0
0x141283d86 f30f7e5708 movq xmm2, qword ptr [rdi + 8]
0x141283d8b 66480f7ed1 movq rcx, xmm2
0x141283d90 31d2 xor edx, edx
0x141283d92 4885c9 test rcx, rcx
0x141283d95 0f99c2 setns dl
0x141283d98 f20f5ad2 cvtsd2ss xmm2, xmm2
0x141283d9c 0f54d7 andps xmm2, xmm7
0x141283d9f f3450f10449500 movss xmm8, dword ptr [r13 + rdx*4]
0x141283da6 440f54c6 andps xmm8, xmm6
0x141283daa 440f56c2 orps xmm8, xmm2
0x141283dae e92efeffff jmp 0x141283be1
0x141283db3 8b4f04 mov ecx, dword ptr [rdi + 4]
0x141283db6 450f57c0 xorps xmm8, xmm8
0x141283dba f34c0f2ac1 cvtsi2ss xmm8, rcx
0x141283dbf e91dfeffff jmp 0x141283be1
0x141283dc4 488b4f08 mov rcx, qword ptr [rdi + 8]
0x141283dc8 4885c9 test rcx, rcx
0x141283dcb 7849 js 0x141283e16
0x141283dcd 450f57c0 xorps xmm8, xmm8
0x141283dd1 f34c0f2ac1 cvtsi2ss xmm8, rcx
0x141283dd6 e906feffff jmp 0x141283be1
0x141283ddb 450f57c0 xorps xmm8, xmm8
0x141283ddf f34c0f2a4708 cvtsi2ss xmm8, qword ptr [rdi + 8]
0x141283de5 e9f7fdffff jmp 0x141283be1
0x141283dea 0fb74f02 movzx ecx, word ptr [rdi + 2]
0x141283dee e9e5fdffff jmp 0x141283bd8
0x141283df3 0fbf4f02 movsx ecx, word ptr [rdi + 2]
0x141283df7 e9dcfdffff jmp 0x141283bd8
0x141283dfc 450f57c0 xorps xmm8, xmm8
0x141283e00 f3440f2a4704 cvtsi2ss xmm8, dword ptr [rdi + 4]
0x141283e06 e9d6fdffff jmp 0x141283be1
0x141283e0b f3440f104704 movss xmm8, dword ptr [rdi + 4]
0x141283e11 e9cbfdffff jmp 0x141283be1
0x141283e16 4889ca mov rdx, rcx
0x141283e19 48d1ea shr rdx, 1
0x141283e1c 83e101 and ecx, 1
0x141283e1f 4809d1 or rcx, rdx
0x141283e22 450f57c0 xorps xmm8, xmm8
0x141283e26 f34c0f2ac1 cvtsi2ss xmm8, rcx
0x141283e2b f3450f58c0 addss xmm8, xmm8
0x141283e30 e9acfdffff jmp 0x141283be1
0x141283e35 a801 test al, 1
0x141283e37 7430 je 0x141283e69
0x141283e39 40f6c501 test bpl, 1
0x141283e3d 0f8455010000 je 0x141283f98
0x141283e43 f30f114c2440 movss dword ptr [rsp + 0x40], xmm1
0x141283e49 f3440f11442444 movss dword ptr [rsp + 0x44], xmm8
0x141283e50 488b442440 mov rax, qword ptr [rsp + 0x40]
0x141283e55 f30f114604 movss dword ptr [rsi + 4], xmm0
0x141283e5a 48894608 mov qword ptr [rsi + 8], rax
0x141283e5e c70600000000 mov dword ptr [rsi], 0
0x141283e64 e9f7000000 jmp 0x141283f60
0x141283e69 488d0de03a9a01 lea rcx, [rip + 0x19a3ae0]
0x141283e70 e92a010000 jmp 0x141283f9f
0x141283e75 4c8d0524369a01 lea r8, [rip + 0x19a3624]
0x141283e7c 488d442430 lea rax, [rsp + 0x30]
0x141283e81 4889d1 mov rcx, rdx
0x141283e84 4889c2 mov rdx, rax
0x141283e87 e864416701 call 0x1428f7ff0
0x141283e8c e9eafcffff jmp 0x141283b7b
0x141283e91 488d15a83a9a01 lea rdx, [rip + 0x19a3aa8]
0x141283e98 4c8d0519359a01 lea r8, [rip + 0x19a3519]
0x141283e9f 31c9 xor ecx, ecx
0x141283ea1 e91b010000 jmp 0x141283fc1
0x141283ea6 0fbe5721 movsx edx, byte ptr [rdi + 0x21]
0x141283eaa eb71 jmp 0x141283f1d
0x141283eac f30f7e4728 movq xmm0, qword ptr [rdi + 0x28]
0x141283eb1 66480f7ec2 movq rdx, xmm0
0x141283eb6 4531c0 xor r8d, r8d
0x141283eb9 4885d2 test rdx, rdx
0x141283ebc 410f99c0 setns r8b
0x141283ec0 f20f5ac0 cvtsd2ss xmm0, xmm0
0x141283ec4 488d15a5aa6a01 lea rdx, [rip + 0x16aaaa5]
0x141283ecb f3420f100c82 movss xmm1, dword ptr [rdx + r8*4]
0x141283ed1 0f540d181a6a01 andps xmm1, xmmword ptr [rip + 0x16a1a18]
0x141283ed8 0f5405a1aa6a01 andps xmm0, xmmword ptr [rip + 0x16aaaa1]
0x141283edf 0f56c1 orps xmm0, xmm1
0x141283ee2 eb40 jmp 0x141283f24
0x141283ee4 8b5724 mov edx, dword ptr [rdi + 0x24]
0x141283ee7 0f57c0 xorps xmm0, xmm0
0x141283eea f3480f2ac2 cvtsi2ss xmm0, rdx
0x141283eef eb33 jmp 0x141283f24
0x141283ef1 488b5728 mov rdx, qword ptr [rdi + 0x28]
0x141283ef5 4885d2 test rdx, rdx
0x141283ef8 0f88cd000000 js 0x141283fcb
0x141283efe 0f57c0 xorps xmm0, xmm0
0x141283f01 f3480f2ac2 cvtsi2ss xmm0, rdx
0x141283f06 eb1c jmp 0x141283f24
0x141283f08 0f57c0 xorps xmm0, xmm0
0x141283f0b f3480f2a4728 cvtsi2ss xmm0, qword ptr [rdi + 0x28]
0x141283f11 eb11 jmp 0x141283f24
0x141283f13 0fb75722 movzx edx, word ptr [rdi + 0x22]
0x141283f17 eb04 jmp 0x141283f1d
0x141283f19 0fbf5722 movsx edx, word ptr [rdi + 0x22]
0x141283f1d 0f57c0 xorps xmm0, xmm0
0x141283f20 f30f2ac2 cvtsi2ss xmm0, edx
0x141283f24 894c2448 mov dword ptr [rsp + 0x48], ecx
0x141283f28 f30f1144244c movss dword ptr [rsp + 0x4c], xmm0
0x141283f2e 48b9feffffffffffff07 movabs rcx, 0x7fffffffffffffe
0x141283f38 4801cb add rbx, rcx
0x141283f3b 48b9ffffffffffffff07 movabs rcx, 0x7ffffffffffffff
0x141283f45 4821d9 and rcx, rbx
0x141283f48 0f85e3000000 jne 0x141284031
0x141283f4e 488b4c2448 mov rcx, qword ptr [rsp + 0x48]
0x141283f53 894604 mov dword ptr [rsi + 4], eax
0x141283f56 48894e08 mov qword ptr [rsi + 8], rcx
0x141283f5a c70600000000 mov dword ptr [rsi], 0
0x141283f60 4889f0 mov rax, rsi
0x141283f63 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
0x141283f68 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
0x141283f6d 440f28442470 movaps xmm8, xmmword ptr [rsp + 0x70]
0x141283f73 4881c488000000 add rsp, 0x88
0x141283f7a 5b pop rbx
0x141283f7b 5d pop rbp
0x141283f7c 5f pop rdi
0x141283f7d 5e pop rsi
0x141283f7e 415c pop r12
0x141283f80 415d pop r13
0x141283f82 415e pop r14
0x141283f84 415f pop r15
0x141283f86 c3 ret 
0x141283f87 0f57c0 xorps xmm0, xmm0
0x141283f8a f30f2a4724 cvtsi2ss xmm0, dword ptr [rdi + 0x24]
0x141283f8f eb93 jmp 0x141283f24
0x141283f91 f30f104724 movss xmm0, dword ptr [rdi + 0x24]
0x141283f96 eb8c jmp 0x141283f24
0x141283f98 488d0d3f379a01 lea rcx, [rip + 0x19a373f]
0x141283f9f ba06000000 mov edx, 6
0x141283fa4 e8175c6701 call 0x1428f9bc0
0x141283fa9 e9cdfbffff jmp 0x141283b7b
0x141283fae 488d158b399a01 lea rdx, [rip + 0x19a398b]
0x141283fb5 4c8d05fc339a01 lea r8, [rip + 0x19a33fc]
0x141283fbc b901000000 mov ecx, 1
0x141283fc1 e85a5d6701 call 0x1428f9d20
0x141283fc6 e9b0fbffff jmp 0x141283b7b
0x141283fcb 4989d0 mov r8, rdx
0x141283fce 49d1e8 shr r8, 1
0x141283fd1 83e201 and edx, 1
0x141283fd4 4c09c2 or rdx, r8
0x141283fd7 0f57c0 xorps xmm0, xmm0
0x141283fda f3480f2ac2 cvtsi2ss xmm0, rdx
0x141283fdf f30f58c0 addss xmm0, xmm0
0x141283fe3 e93cffffff jmp 0x141283f24
0x141283fe8 4883c7e0 add rdi, -0x20
0x141283fec 4c8d050d359a01 lea r8, [rip + 0x19a350d]
0x141283ff3 eb0b jmp 0x141284000
0x141283ff5 4883c720 add rdi, 0x20
0x141283ff9 4c8d0588379a01 lea r8, [rip + 0x19a3788]
0x141284000 488d542430 lea rdx, [rsp + 0x30]
0x141284005 4889f9 mov rcx, rdi
0x141284008 e8e33f6701 call 0x1428f7ff0
0x14128400d e969fbffff jmp 0x141283b7b
0x141284012 488d0d37399a01 lea rcx, [rip + 0x19a3937]
0x141284019 eb07 jmp 0x141284022
0x14128401b 488d0dbc369a01 lea rcx, [rip + 0x19a36bc]
0x141284022 ba06000000 mov edx, 6
0x141284027 e8845d6701 call 0x1428f9db0
0x14128402c e94afbffff jmp 0x141283b7b
0x141284031 4883c102 add rcx, 2
0x141284035 48c744243002000000 mov qword ptr [rsp + 0x30], 2
0x14128403e 4c8d0523379a01 lea r8, [rip + 0x19a3723]
0x141284045 488d542430 lea rdx, [rsp + 0x30]
0x14128404a e972ffffff jmp 0x141283fc1
