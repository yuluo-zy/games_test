0x1409c6700: push rbp
0x1409c6701: push r15
0x1409c6703: push r14
0x1409c6705: push r13
0x1409c6707: push r12
0x1409c6709: push rsi
0x1409c670a: push rdi
0x1409c670b: push rbx
0x1409c670c: sub rsp, 0x9c8
0x1409c6713: lea rbp, [rsp + 0x80]
0x1409c671b: mov qword ptr [rbp + 0x940], 0xfffffffffffffffe
0x1409c6726: mov r12d, r9d
0x1409c6729: mov qword ptr [rbp + 0x930], r8
0x1409c6730: mov rsi, rdx
0x1409c6733: mov qword ptr [rbp + 0x920], rcx
0x1409c673a: call 0x140c16a00
0x1409c673f: test al, al
0x1409c6741: mov byte ptr [rbp + 0x93e], al
0x1409c6747: je 0x1409c67a8
0x1409c6749: lea r14, [rip + 0x20f92ab]
0x1409c6750: mov edi, 0x45
0x1409c6755: mov edx, 0x45
0x1409c675a: mov rcx, r14
0x1409c675d: call 0x1409be680
0x1409c6762: cmp rax, 1
0x1409c6766: jne 0x1409c681d
0x1409c676c: mov r9, rdx
0x1409c676f: test rdx, rdx
0x1409c6772: je 0x1409c67b1
0x1409c6774: cmp r9, 0x45
0x1409c6778: jae 0x1409c67af
0x1409c677a: lea rax, [rip + 0x20f927a]
0x1409c6781: cmp byte ptr [r9 + rax], 0xbf
0x1409c6786: jg 0x1409c67b1
0x1409c6788: lea rax, [rip + 0x20f9381]
0x1409c678f: mov qword ptr [rsp + 0x20], rax
0x1409c6794: lea rcx, [rip + 0x20f9260]
0x1409c679b: mov edx, 0x45
0x1409c67a0: xor r8d, r8d
0x1409c67a3: call 0x1428d9940
0x1409c67a8: xor eax, eax
0x1409c67aa: jmp 0x1409c6978
0x1409c67af: jne 0x1409c6788
0x1409c67b1: lea r14, [rip + 0x20f9243]
0x1409c67b8: mov rcx, r14
0x1409c67bb: mov rdx, r9
0x1409c67be: call 0x1409be680
0x1409c67c3: cmp rax, 1
0x1409c67c7: jne 0x1409c681d
0x1409c67c9: mov r8, rdx
0x1409c67cc: add r8, 2
0x1409c67d0: je 0x1409c680b
0x1409c67d2: cmp r8, 0x45
0x1409c67d6: jae 0x1409c6809
0x1409c67d8: lea rax, [rip + 0x20f921c]
0x1409c67df: cmp byte ptr [r8 + rax], 0xbf
0x1409c67e4: jg 0x1409c680b
0x1409c67e6: lea rax, [rip + 0x20f933b]
0x1409c67ed: mov qword ptr [rsp + 0x20], rax
0x1409c67f2: lea rcx, [rip + 0x20f9202]
0x1409c67f9: mov edx, 0x45
0x1409c67fe: mov r9d, 0x45
0x1409c6804: call 0x1428d9940
0x1409c6809: jne 0x1409c67e6
0x1409c680b: mov edi, 0x43
0x1409c6810: sub rdi, rdx
0x1409c6813: lea r14, [rip + 0x20f91e1]
0x1409c681a: add r14, r8
0x1409c681d: lea r8, [rip + 0x20f9710]
0x1409c6824: lea r15, [rip + 0x20f96c4]
0x1409c682b: nop dword ptr [rax + rax]
0x1409c6830: movsx eax, byte ptr [r8 - 1]
0x1409c6835: test eax, eax
0x1409c6837: js 0x1409c6850
0x1409c6839: dec r8
0x1409c683c: cmp eax, 0x5c
0x1409c683f: jne 0x1409c68a4
0x1409c6841: jmp 0x1409c68b5
0x1409c6843: nop word ptr cs:[rax + rax]
0x1409c6850: movzx ecx, byte ptr [r8 - 2]
0x1409c6855: cmp cl, 0xc0
0x1409c6858: jge 0x1409c687d
0x1409c685a: movzx edx, byte ptr [r8 - 3]
0x1409c685f: cmp dl, 0xc0
0x1409c6862: jge 0x1409c6886
0x1409c6864: movzx r9d, byte ptr [r8 - 4]
0x1409c6869: add r8, -4
0x1409c686d: and r9d, 7
0x1409c6871: shl r9d, 6
0x1409c6875: and edx, 0x3f
0x1409c6878: or edx, r9d
0x1409c687b: jmp 0x1409c688d
0x1409c687d: add r8, -2
0x1409c6881: and ecx, 0x1f
0x1409c6884: jmp 0x1409c6895
0x1409c6886: add r8, -3
0x1409c688a: and edx, 0xf
0x1409c688d: shl edx, 6
0x1409c6890: and ecx, 0x3f
0x1409c6893: or ecx, edx
0x1409c6895: shl ecx, 6
0x1409c6898: and al, 0x3f
0x1409c689a: movzx eax, al
0x1409c689d: or eax, ecx
0x1409c689f: cmp eax, 0x5c
0x1409c68a2: je 0x1409c68b5
0x1409c68a4: cmp eax, 0x2f
0x1409c68a7: je 0x1409c68b5
0x1409c68a9: cmp r8, r15
0x1409c68ac: jne 0x1409c6830
0x1409c68ae: mov ebx, 0x45
0x1409c68b3: jmp 0x1409c6901
0x1409c68b5: lea r15, [rip + 0x20f9633]
0x1409c68bc: sub r8, r15
0x1409c68bf: inc r8
0x1409c68c2: je 0x1409c68f6
0x1409c68c4: cmp r8, 0x45
0x1409c68c8: jae 0x1409c68f4
0x1409c68ca: cmp byte ptr [r8 + r15], 0xbf
0x1409c68cf: jg 0x1409c68f6
0x1409c68d1: lea rax, [rip + 0x20f9218]
0x1409c68d8: mov qword ptr [rsp + 0x20], rax
0x1409c68dd: lea rcx, [rip + 0x20f960b]
0x1409c68e4: mov edx, 0x45
0x1409c68e9: mov r9d, 0x45
0x1409c68ef: call 0x1428d9940
0x1409c68f4: jne 0x1409c68d1
0x1409c68f6: mov ebx, 0x45
0x1409c68fb: sub rbx, r8
0x1409c68fe: add r15, r8
0x1409c6901: call 0x140c18130
0x1409c6906: mov rcx, rax
0x1409c6909: mov rax, qword ptr [rax]
0x1409c690c: cmp rax, 1
0x1409c6910: jne 0x1409c70b6
0x1409c6916: add rcx, 8
0x1409c691a: cmp qword ptr [rcx], 0
0x1409c691e: jne 0x1409c70f0
0x1409c6924: mov qword ptr [rcx], 0xffffffffffffffff
0x1409c692b: mov qword ptr [rbp + 0x928], rcx
0x1409c6932: add rcx, 8
0x1409c6936: mov qword ptr [rsp + 0x20], rbx
0x1409c693b: mov qword ptr [rsp + 0x30], 0
0x1409c6944: mov qword ptr [rsp + 0x28], 1
0x1409c694d: mov rdx, r14
0x1409c6950: mov r8, rdi
0x1409c6953: mov r9, r15
0x1409c6956: call 0x140c16d40
0x1409c695b: mov rcx, qword ptr [rbp + 0x928]
0x1409c6962: inc qword ptr [rcx]
0x1409c6965: mov qword ptr [rbp + 0x8f0], rax
0x1409c696c: mov qword ptr [rbp + 0x8d0], rax
0x1409c6973: mov eax, 1
0x1409c6978: mov qword ptr [rbp + 0x8c8], rax
0x1409c697f: lea r15, [rsi + 0x38]
0x1409c6983: movabs rdx, 0x8597ee0727b461b3
0x1409c698d: movabs r8, 0x8b297ec5ae15f15c
0x1409c6997: mov rcx, r15
0x1409c699a: call 0x1400dd6f0
0x1409c699f: test rax, rax
0x1409c69a2: je 0x1409c70fc
0x1409c69a8: cmp qword ptr [rsi + 0x218], rdx
0x1409c69af: jbe 0x1409c70fc
0x1409c69b5: mov rax, qword ptr [rsi + 0x210]
0x1409c69bc: mov rcx, qword ptr [rax + rdx*8]
0x1409c69c0: test rcx, rcx
0x1409c69c3: je 0x1409c70fc
0x1409c69c9: mov rax, qword ptr [rsi + 0x1e0]
0x1409c69d0: not rcx
0x1409c69d3: lea rcx, [rcx + rcx*2]
0x1409c69d7: shl rcx, 5
0x1409c69db: cmp qword ptr [rax + rcx + 0x20], 0
0x1409c69e1: je 0x1409c70fc
0x1409c69e7: add rax, rcx
0x1409c69ea: mov rcx, qword ptr [rax + 0x10]
0x1409c69ee: lea rdx, [rip + 0x20f9633]
0x1409c69f5: mov r8d, 4
0x1409c69fb: call 0x140937aa0
0x1409c6a00: test al, 1
0x1409c6a02: je 0x1409c713b
0x1409c6a08: mov r14, rdx
0x1409c6a0b: movabs rdx, 0x864ddca98c2312a6
0x1409c6a15: movabs r8, 0x27b0e9369e9a484e
0x1409c6a1f: mov rcx, r15
0x1409c6a22: call 0x1400dd6f0
0x1409c6a27: test rax, rax
0x1409c6a2a: je 0x1409c710a
0x1409c6a30: cmp qword ptr [rsi + 0x218], rdx
0x1409c6a37: jbe 0x1409c710a
0x1409c6a3d: mov rax, qword ptr [rsi + 0x210]
0x1409c6a44: mov rcx, qword ptr [rax + rdx*8]
0x1409c6a48: test rcx, rcx
0x1409c6a4b: je 0x1409c710a
0x1409c6a51: mov rax, qword ptr [rsi + 0x1e0]
0x1409c6a58: not rcx
0x1409c6a5b: lea rcx, [rcx + rcx*2]
0x1409c6a5f: shl rcx, 5
0x1409c6a63: cmp qword ptr [rax + rcx + 0x20], 0
0x1409c6a69: je 0x1409c710a
0x1409c6a6f: add rax, rcx
0x1409c6a72: mov rax, qword ptr [rax + 0x10]
0x1409c6a76: mov ecx, dword ptr [rax]
0x1409c6a78: mov qword ptr [rbp + 0x8e8], rcx
0x1409c6a7f: mov eax, dword ptr [rax + 4]
0x1409c6a82: mov qword ptr [rbp + 0x928], rax
0x1409c6a89: movabs rdx, 0x958c258b851a69dc
0x1409c6a93: movabs r8, 0x47bebccb61712be8
0x1409c6a9d: mov rcx, r15
0x1409c6aa0: call 0x1400dd6f0
0x1409c6aa5: test rax, rax
0x1409c6aa8: je 0x1409c7118
0x1409c6aae: cmp qword ptr [rsi + 0x218], rdx
0x1409c6ab5: jbe 0x1409c7118
0x1409c6abb: mov rax, qword ptr [rsi + 0x210]
0x1409c6ac2: mov rcx, qword ptr [rax + rdx*8]
0x1409c6ac6: test rcx, rcx
0x1409c6ac9: je 0x1409c7118
0x1409c6acf: mov rax, qword ptr [rsi + 0x1e0]
0x1409c6ad6: not rcx
0x1409c6ad9: lea rcx, [rcx + rcx*2]
0x1409c6add: shl rcx, 5
0x1409c6ae1: cmp qword ptr [rax + rcx + 0x20], 0
0x1409c6ae7: je 0x1409c7118
0x1409c6aed: add rax, rcx
0x1409c6af0: mov rcx, qword ptr [rax + 0x10]
0x1409c6af4: call 0x140b8f670
0x1409c6af9: mov rdi, rax
0x1409c6afc: mov r13, rdx
0x1409c6aff: test r12b, r12b
0x1409c6b02: je 0x1409c6bc8
0x1409c6b08: mov rax, qword ptr [rbp + 0x930]
0x1409c6b0f: lea rcx, [rax + 4]
0x1409c6b13: call 0x140bd5790
0x1409c6b18: lea rcx, [rbp + 0x7b0]
0x1409c6b1f: mov rdx, rax
0x1409c6b22: mov r8d, 0x2b
0x1409c6b28: call 0x140bd52a0
0x1409c6b2d: movups xmm0, xmmword ptr [rbp + 0x7b0]
0x1409c6b34: movups xmm1, xmmword ptr [rbp + 0x7c0]
0x1409c6b3b: movaps xmmword ptr [rbp - 0x20], xmm1
0x1409c6b3f: movaps xmmword ptr [rbp - 0x30], xmm0
0x1409c6b43: lea rcx, [rbp + 0x3d0]
0x1409c6b4a: lea rdx, [rbp - 0x30]
0x1409c6b4e: call 0x140bd6ce0
0x1409c6b53: lea rax, [rip + 0x20f951e]
0x1409c6b5a: mov qword ptr [rbp + 0x3c0], rax
0x1409c6b61: mov qword ptr [rbp + 0x3c8], 8
0x1409c6b6c: lea rdx, [rbp + 0x3c0]
0x1409c6b73: mov rcx, qword ptr [rbp + 0x920]
0x1409c6b7a: call 0x140be94d0
0x1409c6b7f: movups xmm0, xmmword ptr [rbp + 0x7b0]
0x1409c6b86: movups xmm1, xmmword ptr [rbp + 0x7c0]
0x1409c6b8d: movups xmmword ptr [rbp + 0x910], xmm1
0x1409c6b94: movups xmmword ptr [rbp + 0x900], xmm0
0x1409c6b9b: mov dword ptr [rbp + 0x8fc], eax
0x1409c6ba1: xor r12d, r12d
0x1409c6ba4: xor ebx, ebx
0x1409c6ba6: cmp byte ptr [rbp + 0x91e], 7
0x1409c6bad: mov rax, qword ptr [rbp + 0x930]
0x1409c6bb4: cmove r12, rax
0x1409c6bb8: setne bl
0x1409c6bbb: lea rcx, [rbp + 0x8fc]
0x1409c6bc2: cmove rcx, rax
0x1409c6bc6: jmp 0x1409c6bdb
0x1409c6bc8: mov byte ptr [rbp + 0x91e], 7
0x1409c6bcf: xor ebx, ebx
0x1409c6bd1: mov rcx, qword ptr [rbp + 0x930]
0x1409c6bd8: mov r12, rcx
0x1409c6bdb: add rcx, 4
0x1409c6bdf: call 0x140bd5790
0x1409c6be4: mov dword ptr [rbp + 0x3c0], ebx
0x1409c6bea: mov dword ptr [rbp + 0x3c4], 0
0x1409c6bf4: mov qword ptr [rbp + 0x3c8], r12
0x1409c6bfb: mov qword ptr [rbp + 0x3d0], 0
0x1409c6c06: lea rcx, [rbp + 0x8fc]
0x1409c6c0d: mov qword ptr [rbp + 0x3d8], rcx
0x1409c6c14: mov dword ptr [rbp + 0x7b0], 2
0x1409c6c1e: mov rcx, qword ptr [rbp + 0x920]
0x1409c6c25: mov qword ptr [rsp + 0x40], rcx
0x1409c6c2a: mov qword ptr [rsp + 0x38], rsi
0x1409c6c2f: mov qword ptr [rsp + 0x30], rax
0x1409c6c34: lea rax, [rbp + 0x7b0]
0x1409c6c3b: mov qword ptr [rsp + 0x28], rax
0x1409c6c40: mov qword ptr [rsp + 0x20], 1
0x1409c6c49: lea rdx, [rip + 0x20f9430]
0x1409c6c50: lea rcx, [rbp - 0x30]
0x1409c6c54: lea r9, [rbp + 0x3c0]
0x1409c6c5b: mov r8d, 6
0x1409c6c61: call 0x140867970
0x1409c6c66: mov byte ptr [rbp + 0x93f], 1
0x1409c6c6d: call 0x140be6750
0x1409c6c72: mov r12d, eax
0x1409c6c75: mov byte ptr [rbp + 0x93f], 1
0x1409c6c7c: xor ecx, ecx
0x1409c6c7e: call 0x1400d9b20
0x1409c6c83: test rax, rax
0x1409c6c86: je 0x1409c7149
0x1409c6c8c: mov dword ptr [rbp + 0x858], r12d
0x1409c6c93: mov word ptr [rbp + 0x860], 0
0x1409c6c9c: mov dword ptr [rbp + 0x85c], 0x3f800000
0x1409c6ca6: mov qword ptr [rbp + 0x7b0], 0
0x1409c6cb1: mov qword ptr [rbp + 0x850], r14
0x1409c6cb8: mov qword ptr [rbp + 0x7c0], 0
0x1409c6cc3: mov qword ptr [rbp + 0x7c8], 4
0x1409c6cce: mov qword ptr [rbp + 0x7d0], 0
0x1409c6cd9: movups xmm0, xmmword ptr [rax]
0x1409c6cdc: inc qword ptr [rax]
0x1409c6cdf: movups xmm1, xmmword ptr [rip + 0x20cfa4a]
0x1409c6ce6: movups xmmword ptr [rbp + 0x830], xmm1
0x1409c6ced: movups xmm1, xmmword ptr [rip + 0x20cfa2c]
0x1409c6cf4: movups xmmword ptr [rbp + 0x820], xmm1
0x1409c6cfb: movups xmmword ptr [rbp + 0x840], xmm0
0x1409c6d02: movabs rax, 0x100000003
0x1409c6d0c: mov qword ptr [rbp + 0x7f0], rax
0x1409c6d13: mov qword ptr [rbp + 0x7d8], 0
0x1409c6d1e: mov qword ptr [rbp + 0x7e0], 1
0x1409c6d29: mov qword ptr [rbp + 0x7e8], 0
0x1409c6d34: mov byte ptr [rbp + 0x93f], 1
0x1409c6d3b: lea rcx, [rbp + 0x3c0]
0x1409c6d42: lea rdx, [rbp + 0x7b0]
0x1409c6d49: lea r8, [rbp - 0x30]
0x1409c6d4d: xor r9d, r9d
0x1409c6d50: call 0x140905be0
0x1409c6d55: cmp byte ptr [rbp + 0x49e], 8
0x1409c6d5c: je 0x1409c6d9b
0x1409c6d5e: cvtsi2ss xmm3, qword ptr [rbp + 0x8e8]
0x1409c6d67: xorps xmm0, xmm0
0x1409c6d6a: cvtsi2ss xmm0, qword ptr [rbp + 0x928]
0x1409c6d73: lea rcx, [rbp + 0x478]
0x1409c6d7a: mov edx, 0xe8
0x1409c6d7f: add rdx, qword ptr [rbp + 0x4a8]
0x1409c6d86: movss dword ptr [rsp + 0x20], xmm0
0x1409c6d8c: movabs r8, 0xa5a0625b7860559
0x1409c6d96: call 0x14096b940
0x1409c6d9b: lea r14, [rbp + 0x7b0]
0x1409c6da2: lea r12, [rbp + 0x3c0]
0x1409c6da9: mov r8d, 0xf8
0x1409c6daf: mov rcx, r14
0x1409c6db2: mov rdx, r12
0x1409c6db5: call 0x142923c00
0x1409c6dba: movabs rax, 0x579f39ab91a01865
0x1409c6dc4: mov qword ptr [rbp + 0x8a8], rax
0x1409c6dcb: mov dword ptr [rbp + 0x3c0], 2
0x1409c6dd5: mov qword ptr [rsp + 0x20], r12
0x1409c6dda: lea rcx, [rbp + 0x8a8]
0x1409c6de1: movabs r8, 0x815bf5036966978c
0x1409c6deb: mov rdx, r14
0x1409c6dee: mov r9b, 0x1d
0x1409c6df1: call 0x140908610
0x1409c6df6: lea r14, [rbp + 0x3c0]
0x1409c6dfd: lea r12, [rbp + 0x7b0]
0x1409c6e04: mov r8d, 0xf8
0x1409c6e0a: mov rcx, r14
0x1409c6e0d: mov rdx, r12
0x1409c6e10: call 0x142923c00
0x1409c6e15: mov qword ptr [rbp + 0x8a8], rdi
0x1409c6e1c: mov dword ptr [rbp + 0x7b0], 2
0x1409c6e26: mov qword ptr [rsp + 0x20], r12
0x1409c6e2b: lea rdi, [rbp + 0x8a8]
0x1409c6e32: movabs r8, 0x753991f424563352
0x1409c6e3c: mov rcx, rdi
0x1409c6e3f: mov rdx, r14
0x1409c6e42: mov r9b, 0x1d
0x1409c6e45: call 0x1408fd6c0
0x1409c6e4a: lea r14, [rbp + 0x7b0]
0x1409c6e51: lea rdx, [rbp + 0x3c0]
0x1409c6e58: mov r8d, 0xf8
0x1409c6e5e: mov rcx, r14
0x1409c6e61: call 0x142923c00
0x1409c6e66: mov byte ptr [rbp + 0x93f], 1
0x1409c6e6d: lea rcx, [rbp + 0x3c0]
0x1409c6e74: movabs r8, 0xd7bebe7cbc8b02bf
0x1409c6e7e: mov rdx, r14
0x1409c6e81: call 0x14162c590
0x1409c6e86: mov byte ptr [rbp + 0x93f], 1
0x1409c6e8d: lea rcx, [rbp + 0x7b0]
0x1409c6e94: lea rdx, [rbp + 0x3c0]
0x1409c6e9b: movabs r8, 0x912f62afc23333b3
0x1409c6ea5: call 0x141629e90
0x1409c6eaa: and byte ptr [rbp + 0x858], 0x7f
0x1409c6eb1: movzx eax, byte ptr [rbp + 0x85b]
0x1409c6eb8: and al, 0x9f
0x1409c6eba: or al, 0x40
0x1409c6ebc: mov byte ptr [rbp + 0x85b], al
0x1409c6ec2: lea r14, [rbp + 0x3c0]
0x1409c6ec9: lea rdx, [rbp + 0x7b0]
0x1409c6ed0: mov r8d, 0xf8
0x1409c6ed6: mov rcx, r14
0x1409c6ed9: call 0x142923c00
0x1409c6ede: add r13, r13
0x1409c6ee1: lea rdx, [r13*2]
0x1409c6ee9: add rdx, r13
0x1409c6eec: mov byte ptr [rbp + 0x93f], 1
0x1409c6ef3: mov rcx, r14
0x1409c6ef6: call 0x140906a90
0x1409c6efb: lea rdx, [rbp - 0x30]
0x1409c6eff: mov r8d, 0x3e8
0x1409c6f05: mov rcx, r14
0x1409c6f08: call 0x142923c00
0x1409c6f0d: mov byte ptr [rbp + 0x93f], 0
0x1409c6f14: mov rcx, r14
0x1409c6f17: call 0x140868370
0x1409c6f1c: cmp byte ptr [rbp + 0x91e], 7
0x1409c6f23: je 0x1409c7056
0x1409c6f29: mov eax, dword ptr [rbp + 0x91c]
0x1409c6f2f: mov dword ptr [rbp + 0x7d0], eax
0x1409c6f35: movups xmm0, xmmword ptr [rbp + 0x8fc]
0x1409c6f3c: movups xmm1, xmmword ptr [rbp + 0x90c]
0x1409c6f43: movaps xmmword ptr [rbp + 0x7c0], xmm1
0x1409c6f4a: movaps xmmword ptr [rbp + 0x7b0], xmm0
0x1409c6f51: mov byte ptr [rbp + 0x93f], 0
0x1409c6f58: movabs rdx, 0x8597ee0727b461b3
0x1409c6f62: movabs r8, 0x8b297ec5ae15f15c
0x1409c6f6c: mov rcx, r15
0x1409c6f6f: call 0x1400dd6f0
0x1409c6f74: test rax, rax
0x1409c6f77: je 0x1409c7126
0x1409c6f7d: cmp qword ptr [rsi + 0x218], rdx
0x1409c6f84: jbe 0x1409c7126
0x1409c6f8a: mov rax, qword ptr [rsi + 0x210]
0x1409c6f91: mov rcx, qword ptr [rax + rdx*8]
0x1409c6f95: test rcx, rcx
0x1409c6f98: je 0x1409c7126
0x1409c6f9e: mov rax, qword ptr [rsi + 0x1e0]
0x1409c6fa5: not rcx
0x1409c6fa8: lea rcx, [rcx + rcx*2]
0x1409c6fac: shl rcx, 5
0x1409c6fb0: cmp qword ptr [rax + rcx + 0x20], 0
0x1409c6fb6: je 0x1409c7126
0x1409c6fbc: add rax, rcx
0x1409c6fbf: mov rcx, qword ptr [rax + 0x10]
0x1409c6fc3: mov byte ptr [rbp + 0x93f], 0
0x1409c6fca: lea rdx, [rip + 0x20f90cf]
0x1409c6fd1: mov r8d, 0xa
0x1409c6fd7: call 0x140937aa0
0x1409c6fdc: test al, 1
0x1409c6fde: je 0x1409c715e
0x1409c6fe4: mov dword ptr [rbp + 0x8a8], 0
0x1409c6fee: mov rax, qword ptr [rbp + 0x930]
0x1409c6ff5: mov qword ptr [rbp + 0x8b0], rax
0x1409c6ffc: mov eax, dword ptr [rbp + 0x7d0]
0x1409c7002: mov dword ptr [rbp + 0x3e0], eax
0x1409c7008: movaps xmm0, xmmword ptr [rbp + 0x7b0]
0x1409c700f: movaps xmm1, xmmword ptr [rbp + 0x7c0]
0x1409c7016: movaps xmmword ptr [rbp + 0x3d0], xmm1
0x1409c701d: movaps xmmword ptr [rbp + 0x3c0], xmm0
0x1409c7024: mov byte ptr [rbp + 0x93f], 0
0x1409c702b: mov qword ptr [rsp + 0x30], r14
0x1409c7030: mov qword ptr [rsp + 0x28], rdi
0x1409c7035: mov qword ptr [rsp + 0x20], rdx
0x1409c703a: lea r8, [rip + 0x20f9087]
0x1409c7041: mov r9d, 0x11
0x1409c7047: mov rcx, qword ptr [rbp + 0x920]
0x1409c704e: mov rdx, rsi
0x1409c7051: call 0x1409cdec0
0x1409c7056: cmp byte ptr [rbp + 0x93e], 0
0x1409c705d: je 0x1409c70a2
0x1409c705f: call 0x140c18130
0x1409c7064: mov rcx, rax
0x1409c7067: mov rax, qword ptr [rax]
0x1409c706a: cmp rax, 1
0x1409c706e: jne 0x1409c70d0
0x1409c7070: add rcx, 8
0x1409c7074: cmp qword ptr [rcx], 0
0x1409c7078: jne 0x1409c70f0
0x1409c707a: mov qword ptr [rcx], 0xffffffffffffffff
0x1409c7081: mov qword ptr [rbp + 0x930], rcx
0x1409c7088: add rcx, 8
0x1409c708c: mov rdx, qword ptr [rbp + 0x8f0]
0x1409c7093: call 0x140c170f0
0x1409c7098: mov rax, qword ptr [rbp + 0x930]
0x1409c709f: inc qword ptr [rax]
0x1409c70a2: add rsp, 0x9c8
0x1409c70a9: pop rbx
0x1409c70aa: pop rdi
0x1409c70ab: pop rsi
0x1409c70ac: pop r12
0x1409c70ae: pop r13
0x1409c70b0: pop r14
0x1409c70b2: pop r15
0x1409c70b4: pop rbp
0x1409c70b5: ret 
0x1409c70b6: test rax, rax
0x1409c70b9: jne 0x1409c70e4
0x1409c70bb: xor edx, edx
0x1409c70bd: call 0x1428f87d0
0x1409c70c2: mov rcx, rax
0x1409c70c5: test rax, rax
0x1409c70c8: jne 0x1409c691a
0x1409c70ce: jmp 0x1409c70e4
0x1409c70d0: test rax, rax
0x1409c70d3: jne 0x1409c70e4
0x1409c70d5: xor edx, edx
0x1409c70d7: call 0x1428f87d0
0x1409c70dc: mov rcx, rax
0x1409c70df: test rax, rax
0x1409c70e2: jne 0x1409c7074
0x1409c70e4: lea rcx, [rip + 0x22739b5]
0x1409c70eb: call 0x1428d80d0
0x1409c70f0: lea rcx, [rip + 0x2273a79]
0x1409c70f7: call 0x1428d9250
0x1409c70fc: lea rcx, [rip + 0x20f8f0d]
0x1409c7103: call 0x1428d9310
0x1409c7108: jmp 0x1409c7171
0x1409c710a: lea rcx, [rip + 0x20f8f37]
0x1409c7111: call 0x1428d9310
0x1409c7116: jmp 0x1409c7171
0x1409c7118: lea rcx, [rip + 0x20f8f41]
0x1409c711f: call 0x1428d9310
0x1409c7124: jmp 0x1409c7171
0x1409c7126: mov byte ptr [rbp + 0x93f], 0
0x1409c712d: lea rcx, [rip + 0x20f8f54]
0x1409c7134: call 0x1428d9310
0x1409c7139: jmp 0x1409c7171
0x1409c713b: lea rcx, [rip + 0x20f8eee]
0x1409c7142: call 0x1428d9310
0x1409c7147: jmp 0x1409c7171
0x1409c7149: mov byte ptr [rbp + 0x93f], 1
0x1409c7150: lea rcx, [rip + 0x2273949]
0x1409c7157: call 0x1428d80d0
0x1409c715c: jmp 0x1409c7171
0x1409c715e: mov byte ptr [rbp + 0x93f], 0
0x1409c7165: lea rcx, [rip + 0x20f8f44]
0x1409c716c: call 0x1428d9310
0x1409c7171: ud2 
0x1409c7173: nop word ptr cs:[rax + rax]
0x1409c7180: mov qword ptr [rsp + 0x10], rdx
0x1409c7185: push rbp
0x1409c7186: push r15
0x1409c7188: push r14
0x1409c718a: push r13
0x1409c718c: push r12
0x1409c718e: push rsi
0x1409c718f: push rdi
0x1409c7190: push rbx
0x1409c7191: sub rsp, 0x48
0x1409c7195: lea rbp, [rdx + 0x80]
0x1409c719c: mov rax, qword ptr [rbp + 0x928]
0x1409c71a3: inc qword ptr [rax]
0x1409c71a6: add rsp, 0x48
0x1409c71aa: pop rbx
0x1409c71ab: pop rdi
0x1409c71ac: pop rsi
0x1409c71ad: pop r12
0x1409c71af: pop r13
0x1409c71b1: pop r14
0x1409c71b3: pop r15
0x1409c71b5: pop rbp
0x1409c71b6: ret 
0x1409c71b7: nop word ptr [rax + rax]
0x1409c71c0: mov qword ptr [rsp + 0x10], rdx
0x1409c71c5: push rbp
0x1409c71c6: push r15
0x1409c71c8: push r14
0x1409c71ca: push r13
0x1409c71cc: push r12
0x1409c71ce: push rsi
0x1409c71cf: push rdi
0x1409c71d0: push rbx
0x1409c71d1: sub rsp, 0x48
0x1409c71d5: lea rbp, [rdx + 0x80]
0x1409c71dc: cmp byte ptr [rbp + 0x93e], 0
0x1409c71e3: je 0x1409c7214
0x1409c71e5: lea rax, [rbp + 0x8d0]
0x1409c71ec: mov qword ptr [rbp + 0x8e0], rax
0x1409c71f3: lea rax, [rbp + 0x8e0]
0x1409c71fa: mov qword ptr [rbp + 0x8d8], rax
0x1409c7201: lea rcx, [rip + 0x20f8888]
0x1409c7208: lea rdx, [rbp + 0x8d8]
0x1409c720f: call 0x14132f2d0
0x1409c7214: nop 
0x1409c7215: add rsp, 0x48
0x1409c7219: pop rbx
0x1409c721a: pop rdi
0x1409c721b: pop rsi
0x1409c721c: pop r12
0x1409c721e: pop r13
0x1409c7220: pop r14
0x1409c7222: pop r15
0x1409c7224: pop rbp
0x1409c7225: ret 
0x1409c7226: nop word ptr cs:[rax + rax]
0x1409c7230: mov qword ptr [rsp + 0x10], rdx
0x1409c7235: push rbp
0x1409c7236: push r15
0x1409c7238: push r14
0x1409c723a: push r13
0x1409c723c: push r12
0x1409c723e: push rsi
0x1409c723f: push rdi
0x1409c7240: push rbx
0x1409c7241: sub rsp, 0x48
0x1409c7245: lea rbp, [rdx + 0x80]
0x1409c724c: mov rax, qword ptr [rbp + 0x930]
0x1409c7253: inc qword ptr [rax]
0x1409c7256: add rsp, 0x48
0x1409c725a: pop rbx
0x1409c725b: pop rdi
0x1409c725c: pop rsi
0x1409c725d: pop r12
0x1409c725f: pop r13
0x1409c7261: pop r14
0x1409c7263: pop r15
0x1409c7265: pop rbp
0x1409c7266: ret 
0x1409c7267: nop word ptr [rax + rax]
0x1409c7270: mov qword ptr [rsp + 0x10], rdx
0x1409c7275: push rbp
0x1409c7276: push r15
0x1409c7278: push r14
0x1409c727a: push r13
0x1409c727c: push r12
0x1409c727e: push rsi
0x1409c727f: push rdi
0x1409c7280: push rbx
0x1409c7281: sub rsp, 0x48
0x1409c7285: lea rbp, [rdx + 0x80]
0x1409c728c: cmp byte ptr [rbp + 0x93f], 0
0x1409c7293: je 0x1409c72b0
0x1409c7295: lea rcx, [rbp - 0x30]
0x1409c7299: call 0x1408923c0
0x1409c729e: nop 
0x1409c729f: add rsp, 0x48
0x1409c72a3: pop rbx
0x1409c72a4: pop rdi
0x1409c72a5: pop rsi
0x1409c72a6: pop r12
0x1409c72a8: pop r13
0x1409c72aa: pop r14
0x1409c72ac: pop r15
0x1409c72ae: pop rbp
0x1409c72af: ret 
0x1409c72b0: add rsp, 0x48
0x1409c72b4: pop rbx
0x1409c72b5: pop rdi
0x1409c72b6: pop rsi
0x1409c72b7: pop r12
0x1409c72b9: pop r13
0x1409c72bb: pop r14
0x1409c72bd: pop r15
0x1409c72bf: pop rbp
0x1409c72c0: ret 
0x1409c72c1: nop word ptr cs:[rax + rax]
0x1409c72d0: mov qword ptr [rsp + 0x10], rdx
0x1409c72d5: push rbp
0x1409c72d6: push r15
0x1409c72d8: push r14
0x1409c72da: push r13
0x1409c72dc: push r12
0x1409c72de: push rsi
0x1409c72df: push rdi
0x1409c72e0: push rbx
0x1409c72e1: sub rsp, 0x48
0x1409c72e5: lea rbp, [rdx + 0x80]
0x1409c72ec: mov byte ptr [rbp + 0x93f], 1
0x1409c72f3: lea rcx, [rbp + 0x3c0]
0x1409c72fa: call 0x1408935c0
0x1409c72ff: mov byte ptr [rbp + 0x93f], 1
0x1409c7306: add rsp, 0x48
0x1409c730a: pop rbx
0x1409c730b: pop rdi
0x1409c730c: pop rsi
0x1409c730d: pop r12
0x1409c730f: pop r13
0x1409c7311: pop r14
0x1409c7313: pop r15
0x1409c7315: pop rbp
0x1409c7316: ret 
0x1409c7317: nop word ptr [rax + rax]
0x1409c7320: mov qword ptr [rsp + 0x10], rdx
0x1409c7325: push rbp
0x1409c7326: push r15
0x1409c7328: push r14
0x1409c732a: push r13
0x1409c732c: push r12
0x1409c732e: push rsi
0x1409c732f: push rdi
0x1409c7330: push rbx
0x1409c7331: sub rsp, 0x48
0x1409c7335: lea rbp, [rdx + 0x80]
0x1409c733c: mov byte ptr [rbp + 0x93f], 1
0x1409c7343: lea rcx, [rbp + 0x7b0]
0x1409c734a: call 0x1408935c0
0x1409c734f: mov byte ptr [rbp + 0x93f], 1
0x1409c7356: add rsp, 0x48
0x1409c735a: pop rbx
0x1409c735b: pop rdi
0x1409c735c: pop rsi
0x1409c735d: pop r12
0x1409c735f: pop r13
0x1409c7361: pop r14
0x1409c7363: pop r15
0x1409c7365: pop rbp
0x1409c7366: ret 
0x1409c7367: nop word ptr [rax + rax]
0x1409c7370: mov qword ptr [rsp + 0x10], rdx
0x1409c7375: push rbp
0x1409c7376: push r15
0x1409c7378: push r14
0x1409c737a: push r13
0x1409c737c: push r12
0x1409c737e: push rsi
0x1409c737f: push rdi
0x1409c7380: push rbx
0x1409c7381: sub rsp, 0x48
0x1409c7385: lea rbp, [rdx + 0x80]
0x1409c738c: mov byte ptr [rbp + 0x93f], 1
0x1409c7393: lea rcx, [rbp + 0x3c0]
0x1409c739a: call 0x1408935c0
0x1409c739f: mov byte ptr [rbp + 0x93f], 1
0x1409c73a6: add rsp, 0x48
0x1409c73aa: pop rbx
0x1409c73ab: pop rdi
0x1409c73ac: pop rsi
0x1409c73ad: pop r12
0x1409c73af: pop r13
0x1409c73b1: pop r14
0x1409c73b3: pop r15
0x1409c73b5: pop rbp
0x1409c73b6: ret 
