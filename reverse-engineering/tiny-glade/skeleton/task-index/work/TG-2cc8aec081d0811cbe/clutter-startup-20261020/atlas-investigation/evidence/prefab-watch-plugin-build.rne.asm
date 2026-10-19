1408e4b00: 55 push rbp
1408e4b01: 56 push rsi
1408e4b02: 57 push rdi
1408e4b03: 53 push rbx
1408e4b04: 4881ec18010000 sub rsp, 0x118
1408e4b0b: 488dac2480000000 lea rbp, [rsp + 0x80]
1408e4b13: 48c78590000000feffffff mov qword ptr [rbp + 0x90], 0xfffffffffffffffe
1408e4b1e: 4889d6 mov rsi, rdx
1408e4b21: 4889cf mov rdi, rcx
1408e4b24: 488d5a38 lea rbx, [rdx + 0x38]
1408e4b28: 48ba3fa8503e174f712e movabs rdx, 0x2e714f173e50a83f
1408e4b32: 49b8b5e0ccbb65fe5920 movabs r8, 0x2059fe65bbcce0b5
1408e4b3c: 4889d9 mov rcx, rbx
1408e4b3f: e8ac8b7fff call 0x1400dd6f0
1408e4b44: 4885c0 test rax, rax
1408e4b47: 7433 je 0x1408e4b7c
1408e4b49: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
1408e4b50: 762a jbe 0x1408e4b7c
1408e4b52: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
1408e4b59: 488b04d0 mov rax, qword ptr [rax + rdx*8]
1408e4b5d: 4885c0 test rax, rax
1408e4b60: 741a je 0x1408e4b7c
1408e4b62: 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
1408e4b69: 48f7d0 not rax
1408e4b6c: 488d0440 lea rax, [rax + rax*2]
1408e4b70: 48c1e005 shl rax, 5
1408e4b74: 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
1408e4b7a: 7508 jne 0x1408e4b84
1408e4b7c: 4889f1 mov rcx, rsi
1408e4b7f: e89c78c200 call 0x14150c420
1408e4b84: 48baaedf9feee4326832 movabs rdx, 0x326832e4ee9fdfae
1408e4b8e: 49b855933da31c1fd4c9 movabs r8, 0xc9d41f1ca33d9355
1408e4b98: 4889d9 mov rcx, rbx
1408e4b9b: e8508b7fff call 0x1400dd6f0
1408e4ba0: 4885c0 test rax, rax
1408e4ba3: 7433 je 0x1408e4bd8
1408e4ba5: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
1408e4bac: 762a jbe 0x1408e4bd8
1408e4bae: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
1408e4bb5: 488b04d0 mov rax, qword ptr [rax + rdx*8]
1408e4bb9: 4885c0 test rax, rax
1408e4bbc: 741a je 0x1408e4bd8
1408e4bbe: 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
1408e4bc5: 48f7d0 not rax
1408e4bc8: 488d0440 lea rax, [rax + rax*2]
1408e4bcc: 48c1e005 shl rax, 5
1408e4bd0: 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
1408e4bd6: 7508 jne 0x1408e4be0
1408e4bd8: 4889f1 mov rcx, rsi
1408e4bdb: e8a04ac300 call 0x141519680
1408e4be0: 0fb61f movzx ebx, byte ptr [rdi]
1408e4be3: 488d7d18 lea rdi, [rbp + 0x18]
1408e4be7: 4889f9 mov rcx, rdi
1408e4bea: 89da mov edx, ebx
1408e4bec: e80f429c00 call 0x1412a8e00
1408e4bf1: 4889f1 mov rcx, rsi
1408e4bf4: 4889fa mov rdx, rdi
1408e4bf7: e86433c600 call 0x141547f60
1408e4bfc: 84db test bl, bl
1408e4bfe: 747d je 0x1408e4c7d
1408e4c00: 488d4d18 lea rcx, [rbp + 0x18]
1408e4c04: e8e7031200 call 0x140a04ff0
1408e4c09: c6858f00000000 mov byte ptr [rbp + 0x8f], 0
1408e4c10: 488b0d5977d402 mov rcx, qword ptr [rip + 0x2d47759]
1408e4c17: 4c8d0542b13702 lea r8, [rip + 0x237b142]
1408e4c1e: 488d958f000000 lea rdx, [rbp + 0x8f]
1408e4c25: e876cc2300 call 0x140b218a0
1408e4c2a: 4989d0 mov r8, rdx
1408e4c2d: 488d4d18 lea rcx, [rbp + 0x18]
1408e4c31: 4889c2 mov rdx, rax
1408e4c34: e8c7e3b600 call 0x141453000
1408e4c39: 0f104578 movups xmm0, xmmword ptr [rbp + 0x78]
1408e4c3d: 0f294500 movaps xmmword ptr [rbp], xmm0
1408e4c41: 0f104568 movups xmm0, xmmword ptr [rbp + 0x68]
1408e4c45: 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
1408e4c49: 0f104558 movups xmm0, xmmword ptr [rbp + 0x58]
1408e4c4d: 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
1408e4c51: 0f104518 movups xmm0, xmmword ptr [rbp + 0x18]
1408e4c55: 0f104d28 movups xmm1, xmmword ptr [rbp + 0x28]
1408e4c59: 0f105538 movups xmm2, xmmword ptr [rbp + 0x38]
1408e4c5d: 0f105d48 movups xmm3, xmmword ptr [rbp + 0x48]
1408e4c61: 0f295dd0 movaps xmmword ptr [rbp - 0x30], xmm3
1408e4c65: 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
1408e4c69: 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
1408e4c6d: 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
1408e4c71: 488d55a0 lea rdx, [rbp - 0x60]
1408e4c75: 4889f1 mov rcx, rsi
1408e4c78: e803262800 call 0x140b67280
1408e4c7d: 90 nop 
1408e4c7e: 4881c418010000 add rsp, 0x118
1408e4c85: 5b pop rbx
1408e4c86: 5f pop rdi
1408e4c87: 5e pop rsi
1408e4c88: 5d pop rbp
1408e4c89: c3 ret 
1408e4c8a: 660f1f440000 nop word ptr [rax + rax]
1408e4c90: 4889542410 mov qword ptr [rsp + 0x10], rdx
1408e4c95: 55 push rbp
1408e4c96: 56 push rsi
1408e4c97: 57 push rdi
1408e4c98: 53 push rbx
1408e4c99: 4883ec28 sub rsp, 0x28
1408e4c9d: 488daa80000000 lea rbp, [rdx + 0x80]
1408e4ca4: 488d4d18 lea rcx, [rbp + 0x18]
1408e4ca8: e8d34b74ff call 0x140029880
1408e4cad: 90 nop 
1408e4cae: 4883c428 add rsp, 0x28
1408e4cb2: 5b pop rbx
1408e4cb3: 5f pop rdi
1408e4cb4: 5e pop rsi
1408e4cb5: 5d pop rbp
1408e4cb6: c3 ret 
