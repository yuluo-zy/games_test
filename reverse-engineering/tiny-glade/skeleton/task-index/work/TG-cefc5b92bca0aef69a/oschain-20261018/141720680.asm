141720680: push rbp
141720681: push r15
141720683: push r14
141720685: push r13
141720687: push r12
141720689: push rsi
14172068a: push rdi
14172068b: push rbx
14172068c: sub rsp, 0x728
141720693: lea rbp, [rsp + 0x80]
14172069b: movaps xmmword ptr [rbp + 0x690], xmm6
1417206a2: mov qword ptr [rbp + 0x688], 0xfffffffffffffffe
1417206ad: mov qword ptr [rbp + 0x610], rcx
1417206b4: mov qword ptr [rbp + 0x618], rdx
1417206bb: mov qword ptr [rbp + 0x540], r8
1417206c2: mov qword ptr [rbp + 0x548], r9
1417206c9: mov qword ptr [rbp - 0x58], 3
1417206d1: xor ecx, ecx
1417206d3: call 0x1400d9b20
1417206d8: test rax, rax
1417206db: je 0x1417214e5
1417206e1: movups xmm0, xmmword ptr [rax]
1417206e4: inc qword ptr [rax]
1417206e7: movaps xmmword ptr [rbp + 0x520], xmm0
1417206ee: movups xmm0, xmmword ptr [rip + 0x154b80b]
1417206f5: movaps xmmword ptr [rbp + 0x500], xmm0
1417206fc: movups xmm0, xmmword ptr [rip + 0x154b80d]
141720703: movaps xmmword ptr [rbp + 0x510], xmm0
14172070a: call 0x140d635e0
14172070f: mov qword ptr [rbp + 0x620], rax
141720716: mov dword ptr [rbp + 0x654], edx
14172071c: mov qword ptr [rbp + 0x630], 0
141720727: mov qword ptr [rbp + 0x638], 8
141720732: mov qword ptr [rbp + 0x640], 0
14172073d: lea rdi, [rbp - 0x48]
141720741: mov r15d, 0xff
141720747: lea r13, [rbp + 0x660]
14172074e: xorps xmm6, xmm6
141720751: lea r14, [rip + 0x15aac7c]
141720758: jmp 0x1417207c1
14172075a: nop word ptr [rax + rax]
141720760: movaps xmm0, xmmword ptr [rbp + 0x590]
141720767: movaps xmm1, xmmword ptr [rbp + 0x5a0]
14172076e: movaps xmm2, xmmword ptr [rbp + 0x5b0]
141720775: movaps xmm3, xmmword ptr [rbp + 0x5c0]
14172077c: movaps xmmword ptr [rbp + 0x280], xmm3
141720783: movaps xmmword ptr [rbp + 0x270], xmm2
14172078a: movaps xmmword ptr [rbp + 0x260], xmm1
141720791: movaps xmmword ptr [rbp + 0x250], xmm0
141720798: mov r9, qword ptr [rbp + 0x710]
14172079f: add r9, 0x10
1417207a3: mov byte ptr [rbp + 0x686], 0
1417207aa: lea rcx, [rbp + 0x250]
1417207b1: lea rdx, [rbp - 0x58]
1417207b5: lea r8, [rbp + 0x500]
1417207bc: call 0x141721aa0
1417207c1: call 0x140d635e0
1417207c6: mov r12, rax
1417207c9: mov esi, edx
1417207cb: xor eax, eax
1417207cd: cmp r12, qword ptr [rbp + 0x620]
1417207d4: setne al
1417207d7: cmovb eax, r15d
1417207db: xor ecx, ecx
1417207dd: cmp edx, dword ptr [rbp + 0x654]
1417207e3: setne cl
1417207e6: cmovb ecx, r15d
1417207ea: test al, al
1417207ec: cmovne ecx, eax
1417207ef: cmp cl, 1
1417207f2: jne 0x141720e30
1417207f8: mov qword ptr [rbp + 0x660], 0
141720803: mov qword ptr [rbp + 0x668], 8
14172080e: mov qword ptr [rbp + 0x670], 0
141720819: cmp dword ptr [rbp - 0x58], 3
14172081d: je 0x14172091e
141720823: cmp qword ptr [rbp + 0x30], 0
141720828: je 0x14172091e
14172082e: mov byte ptr [rbp + 0x687], 1
141720835: mov rcx, rdi
141720838: call 0x14091a230
14172083d: cmp qword ptr [rbp + 0x30], 0
141720842: je 0x14172091e
141720848: mov byte ptr [rbp + 0x687], 1
14172084f: mov rcx, rdi
141720852: call 0x140d76ed0
141720857: mov r8, rdx
14172085a: mov byte ptr [rbp + 0x687], 1
141720861: lea rcx, [rbp + 0x550]
141720868: mov rdx, rax
14172086b: call 0x140d61b00
141720870: lea rcx, [rbp + 0x590]
141720877: lea rdx, [rbp + 0x30]
14172087b: call 0x141720490
141720880: movups xmm0, xmmword ptr [rbp + 0x550]
141720887: movups xmm1, xmmword ptr [rbp + 0x560]
14172088e: movaps xmmword ptr [rbp + 0x260], xmm1
141720895: movaps xmmword ptr [rbp + 0x250], xmm0
14172089c: movups xmm0, xmmword ptr [rbp + 0x590]
1417208a3: movups xmm1, xmmword ptr [rbp + 0x5a0]
1417208aa: lea rax, [rbp + 0x270]
1417208b1: movups xmmword ptr [rax + 0x10], xmm1
1417208b5: movups xmmword ptr [rax], xmm0
1417208b8: mov rbx, qword ptr [rbp + 0x670]
1417208bf: cmp rbx, qword ptr [rbp + 0x660]
1417208c6: jne 0x1417208d7
1417208c8: mov rcx, r13
1417208cb: lea rdx, [rip + 0x15a9fee]
1417208d2: call 0x1405627b0
1417208d7: mov rax, qword ptr [rbp + 0x668]
1417208de: mov rcx, rbx
1417208e1: shl rcx, 6
1417208e5: movaps xmm0, xmmword ptr [rbp + 0x250]
1417208ec: movaps xmm1, xmmword ptr [rbp + 0x260]
1417208f3: movaps xmm2, xmmword ptr [rbp + 0x270]
1417208fa: movaps xmm3, xmmword ptr [rbp + 0x280]
141720901: movups xmmword ptr [rax + rcx + 0x30], xmm3
141720906: movups xmmword ptr [rax + rcx + 0x20], xmm2
14172090b: movups xmmword ptr [rax + rcx + 0x10], xmm1
141720910: movups xmmword ptr [rax + rcx], xmm0
141720914: inc rbx
141720917: mov qword ptr [rbp + 0x670], rbx
14172091e: mov rbx, qword ptr [rbp + 0x710]
141720925: lea rdx, [rbx + 0x10]
141720929: xor eax, eax
14172092b: mov cl, 1
14172092d: lock cmpxchg byte ptr [rbx + 0x10], cl
141720932: mov qword ptr [rbp + 0x628], rdx
141720939: jne 0x1417211f9
14172093f: mov byte ptr [rbp + 0x687], 1
141720946: mov rcx, rdx
141720949: call 0x140c81c70
14172094e: add rbx, 0x18
141720952: mov rcx, rbx
141720955: mov rdx, r13
141720958: call 0x1412f3c60
14172095d: mov r8d, 1
141720963: mov rcx, r12
141720966: mov edx, esi
141720968: xor r9d, r9d
14172096b: call 0x140d63a20
141720970: mov qword ptr [rbp + 0x620], rax
141720977: mov dword ptr [rbp + 0x654], edx
14172097d: mov rcx, qword ptr [rbp + 0x670]
141720984: mov rax, qword ptr [rbp + 0x638]
14172098b: mov qword ptr [rbp + 0x658], rax
141720992: mov r12, qword ptr [rbp + 0x640]
141720999: cmp rcx, r12
14172099c: jne 0x141720a22
1417209a2: mov qword ptr [rbp + 0x648], rcx
1417209a9: mov rax, qword ptr [rbp + 0x668]
1417209b0: mov qword ptr [rbp + 0x538], rax
1417209b7: xor r13d, r13d
1417209ba: nop word ptr [rax + rax]
1417209c0: cmp r13, qword ptr [rbp + 0x648]
1417209c7: je 0x141720d7a
1417209cd: mov rsi, r13
1417209d0: shl rsi, 6
1417209d4: mov rax, qword ptr [rbp + 0x538]
1417209db: lea rbx, [rax + rsi]
1417209df: add rsi, qword ptr [rbp + 0x658]
1417209e6: mov rcx, rbx
1417209e9: mov rdx, rsi
1417209ec: call 0x140b0d840
1417209f1: test al, al
1417209f3: je 0x141720a1b
1417209f5: mov eax, dword ptr [rbx + 0x20]
1417209f8: cmp eax, dword ptr [rsi + 0x20]
1417209fb: jne 0x141720a1b
1417209fd: mov r8, qword ptr [rbx + 0x38]
141720a01: cmp r8, qword ptr [rsi + 0x38]
141720a05: jne 0x141720a1b
141720a07: inc r13
141720a0a: mov rdx, qword ptr [rsi + 0x30]
141720a0e: mov rcx, qword ptr [rbx + 0x30]
141720a12: call 0x142923bf0
141720a17: test eax, eax
141720a19: je 0x1417209c0
141720a1b: mov rcx, qword ptr [rbp + 0x670]
141720a22: mov rax, qword ptr [rip + 0x1f0b97f]
141720a29: mov rax, qword ptr [rax]
141720a2c: test rcx, rcx
141720a2f: lea rdx, [rbp + 0x660]
141720a36: je 0x141720b32
141720a3c: test rax, rax
141720a3f: je 0x141720bfa
141720a45: mov qword ptr [rbp + 0x550], rcx
141720a4c: lea rax, [rbp + 0x550]
141720a53: mov qword ptr [rbp + 0x590], rax
141720a5a: lea rax, [rip - 0x9933a1]
141720a61: mov qword ptr [rbp + 0x598], rax
141720a68: lea rcx, [rip + 0x15a9ea9]
141720a6f: call 0x14067dd80
141720a74: movups xmm0, xmmword ptr [rax]
141720a77: mov eax, dword ptr [rax + 0x10]
141720a7a: mov qword ptr [rbp + 0x280], 1
141720a85: lea rcx, [rip + 0x15a9ad4]
141720a8c: mov qword ptr [rbp + 0x288], rcx
141720a93: mov qword ptr [rbp + 0x290], 0x28
141720a9e: lea rdx, [rip + 0x15a9e63]
141720aa5: mov qword ptr [rbp + 0x2a0], rdx
141720aac: mov qword ptr [rbp + 0x2a8], 1
141720ab7: lea rdx, [rbp + 0x590]
141720abe: mov qword ptr [rbp + 0x2b0], rdx
141720ac5: mov qword ptr [rbp + 0x2b8], 1
141720ad0: mov qword ptr [rbp + 0x2c0], 0
141720adb: mov qword ptr [rbp + 0x250], 0
141720ae6: mov qword ptr [rbp + 0x258], rcx
141720aed: mov qword ptr [rbp + 0x260], 0x28
141720af8: mov qword ptr [rbp + 0x268], 0
141720b03: movups xmmword ptr [rbp + 0x270], xmm0
141720b0a: mov dword ptr [rbp + 0x298], 1
141720b14: mov dword ptr [rbp + 0x29c], eax
141720b1a: lea rcx, [rbp + 0x685]
141720b21: lea rdx, [rbp + 0x250]
141720b28: call 0x140d4a330
141720b2d: jmp 0x141720bf3
141720b32: cmp rax, 3
141720b36: jb 0x141720bfa
141720b3c: lea rcx, [rip + 0x15a9e1d]
141720b43: call 0x14067dd80
141720b48: movups xmm0, xmmword ptr [rax]
141720b4b: mov eax, dword ptr [rax + 0x10]
141720b4e: mov qword ptr [rbp + 0x280], 3
141720b59: lea rcx, [rip + 0x15a9a00]
141720b60: mov qword ptr [rbp + 0x288], rcx
141720b67: mov qword ptr [rbp + 0x290], 0x28
141720b72: lea rdx, [rip + 0x15a9dd7]
141720b79: mov qword ptr [rbp + 0x2a0], rdx
141720b80: mov qword ptr [rbp + 0x2a8], 1
141720b8b: mov qword ptr [rbp + 0x2b0], 8
141720b96: mov qword ptr [rbp + 0x250], 0
141720ba1: lea rdx, [rbp + 0x270]
141720ba8: movups xmmword ptr [rdx + 0x48], xmm6
141720bac: mov qword ptr [rbp + 0x258], rcx
141720bb3: mov qword ptr [rbp + 0x260], 0x28
141720bbe: mov qword ptr [rbp + 0x268], 0
141720bc9: movups xmmword ptr [rbp + 0x270], xmm0
141720bd0: mov dword ptr [rbp + 0x298], 1
141720bda: mov dword ptr [rbp + 0x29c], eax
141720be0: lea rcx, [rbp + 0x685]
141720be7: lea rdx, [rbp + 0x250]
141720bee: call 0x140d4a330
141720bf3: lea rdx, [rbp + 0x660]
141720bfa: lea rcx, [rbp + 0x250]
141720c01: lea r8, [rip + 0x15aa6e0]
141720c08: call 0x1412f86a0
141720c0d: lea rcx, [rbp + 0x590]
141720c14: lea rdx, [rbp + 0x540]
141720c1b: lea r8, [rbp + 0x250]
141720c22: call 0x1408da430
141720c27: mov rsi, qword ptr [rbp + 0x590]
141720c2e: mov rax, rsi
141720c31: neg rax
141720c34: jno 0x141720cb0
141720c36: mov rax, qword ptr [rbp + 0x670]
141720c3d: mov qword ptr [rbp + 0x260], rax
141720c44: movups xmm0, xmmword ptr [rbp + 0x660]
141720c4b: movaps xmmword ptr [rbp + 0x250], xmm0
141720c52: test r12, r12
141720c55: je 0x141720d10
141720c5b: mov rax, qword ptr [rbp + 0x658]
141720c62: lea rsi, [rax + 0x30]
141720c66: jmp 0x141720c7d
141720c68: nop dword ptr [rax + rax]
141720c70: add rsi, 0x40
141720c74: dec r12
141720c77: je 0x141720d10
141720c7d: mov rdx, qword ptr [rsi - 0x30]
141720c81: test rdx, rdx
141720c84: je 0x141720c95
141720c86: mov rcx, qword ptr [rsi - 0x28]
141720c8a: mov r8d, 1
141720c90: call 0x140613c20
141720c95: mov rdx, qword ptr [rsi - 8]
141720c99: test rdx, rdx
141720c9c: je 0x141720c70
141720c9e: mov rcx, qword ptr [rsi]
141720ca1: mov r8d, 1
141720ca7: call 0x140613c20
141720cac: jmp 0x141720c70
141720cae: nop 
141720cb0: mov rbx, qword ptr [rbp + 0x598]
141720cb7: mov r12, qword ptr [rbp + 0x5a0]
141720cbe: test r12, r12
141720cc1: je 0x141720d60
141720cc7: lea r13, [rbx + 0x30]
141720ccb: jmp 0x141720cdd
141720ccd: nop dword ptr [rax]
141720cd0: add r13, 0x40
141720cd4: dec r12
141720cd7: je 0x141720d60
141720cdd: mov rdx, qword ptr [r13 - 0x30]
141720ce1: test rdx, rdx
141720ce4: je 0x141720cf5
141720ce6: mov rcx, qword ptr [r13 - 0x28]
141720cea: mov r8d, 1
141720cf0: call 0x140613c20
141720cf5: mov rdx, qword ptr [r13 - 8]
141720cf9: test rdx, rdx
141720cfc: je 0x141720cd0
141720cfe: mov rcx, qword ptr [r13]
141720d02: mov r8d, 1
141720d08: call 0x140613c20
141720d0d: jmp 0x141720cd0
141720d0f: nop 
141720d10: mov rdx, qword ptr [rbp + 0x630]
141720d17: test rdx, rdx
141720d1a: je 0x141720d32
141720d1c: shl rdx, 6
141720d20: mov r8d, 8
141720d26: mov rcx, qword ptr [rbp + 0x658]
141720d2d: call 0x140613c20
141720d32: mov rax, qword ptr [rbp + 0x260]
141720d39: mov qword ptr [rbp + 0x640], rax
141720d40: movaps xmm0, xmmword ptr [rbp + 0x250]
141720d47: movaps xmmword ptr [rbp + 0x630], xmm0
141720d4e: xor esi, esi
141720d50: jmp 0x141720d7d
141720d52: nop word ptr cs:[rax + rax]
141720d60: test rsi, rsi
141720d63: je 0x141720d7a
141720d65: shl rsi, 6
141720d69: mov r8d, 8
141720d6f: mov rcx, rbx
141720d72: mov rdx, rsi
141720d75: call 0x140613c20
141720d7a: mov sil, 1
141720d7d: lea r13, [rbp + 0x660]
141720d84: mov byte ptr [rbp + 0x687], sil
141720d8b: mov rbx, qword ptr [rbp + 0x628]
141720d92: mov rcx, rbx
141720d95: call 0x140c81e10
141720d9a: mov al, 1
141720d9c: xor ecx, ecx
141720d9e: lock cmpxchg byte ptr [rbx], cl
141720da2: jne 0x14172121a
141720da8: test sil, sil
141720dab: je 0x141720e30
141720db1: mov rsi, qword ptr [rbp + 0x668]
141720db8: mov rbx, qword ptr [rbp + 0x670]
141720dbf: test rbx, rbx
141720dc2: je 0x141720e10
141720dc4: lea r12, [rsi + 0x30]
141720dc8: jmp 0x141720dd9
141720dca: nop word ptr [rax + rax]
141720dd0: add r12, 0x40
141720dd4: dec rbx
141720dd7: je 0x141720e10
141720dd9: mov rdx, qword ptr [r12 - 0x30]
141720dde: test rdx, rdx
141720de1: je 0x141720df3
141720de3: mov rcx, qword ptr [r12 - 0x28]
141720de8: mov r8d, 1
141720dee: call 0x140613c20
141720df3: mov rdx, qword ptr [r12 - 8]
141720df8: test rdx, rdx
141720dfb: je 0x141720dd0
141720dfd: mov rcx, qword ptr [r12]
141720e01: mov r8d, 1
141720e07: call 0x140613c20
141720e0c: jmp 0x141720dd0
141720e0e: nop 
141720e10: mov rdx, qword ptr [rbp + 0x660]
141720e17: test rdx, rdx
141720e1a: je 0x141720e30
141720e1c: shl rdx, 6
141720e20: mov r8d, 8
141720e26: mov rcx, rsi
141720e29: call 0x140613c20
141720e2e: nop 
141720e30: mov rax, qword ptr [rbp - 0x58]
141720e34: cmp rax, 3
141720e38: je 0x141720f20
141720e3e: mov rdx, qword ptr [rbp - 0x50]
141720e42: test rax, rax
141720e45: je 0x141720e5a
141720e47: cmp eax, 1
141720e4a: jne 0x141720e68
141720e4c: lea rcx, [rbp + 0x550]
141720e53: call 0x1416777a0
141720e58: jmp 0x141720e74
141720e5a: lea rcx, [rbp + 0x550]
141720e61: call 0x14167c600
141720e66: jmp 0x141720e74
141720e68: lea rcx, [rbp + 0x550]
141720e6f: call 0x1416d6a60
141720e74: xor eax, eax
141720e76: cmp rax, qword ptr [rbp + 0x550]
141720e7d: jo 0x141720f20
141720e83: movups xmm0, xmmword ptr [rbp + 0x550]
141720e8a: movups xmm1, xmmword ptr [rbp + 0x560]
141720e91: movaps xmmword ptr [rbp + 0x5a0], xmm1
141720e98: movaps xmmword ptr [rbp + 0x590], xmm0
141720e9f: lea rcx, [rbp + 0x230]
141720ea6: lea rdx, [rbp + 0x590]
141720ead: call 0x1412e84a0
141720eb2: lea rcx, [rbp + 0x250]
141720eb9: lea rdx, [rbp + 0x210]
141720ec0: mov r8, rax
141720ec3: lea r9, [rbp + 0x590]
141720eca: call 0x140b78be0
141720ecf: mov rdx, qword ptr [rbp + 0x250]
141720ed6: mov rax, rdx
141720ed9: neg rax
141720edc: jo 0x141721442
141720ee2: jae 0x141720ef6
141720ee4: mov rcx, qword ptr [rbp + 0x258]
141720eeb: mov r8d, 1
141720ef1: call 0x140613c20
141720ef6: mov rdx, qword ptr [rbp + 0x590]
141720efd: test rdx, rdx
141720f00: je 0x141720f20
141720f02: mov rcx, qword ptr [rbp + 0x598]
141720f09: mov r8d, 1
141720f0f: call 0x140613c20
141720f14: nop word ptr cs:[rax + rax]
141720f20: lea rax, [rbp - 0x58]
141720f24: mov qword ptr [rbp + 0x250], rax
141720f2b: lea rcx, [rbp + 0x500]
141720f32: lea rdx, [rbp + 0x250]
141720f39: call 0x14152d950
141720f3e: mov rax, qword ptr [rbp + 0x610]
141720f45: mov rdx, qword ptr [rbp + 0x618]
141720f4c: test rax, rax
141720f4f: je 0x141720f70
141720f51: cmp eax, 1
141720f54: jne 0x141720f80
141720f56: lea rcx, [rbp + 0x550]
141720f5d: call 0x141677330
141720f62: jmp 0x141720f8c
141720f64: nop word ptr cs:[rax + rax]
141720f70: lea rcx, [rbp + 0x550]
141720f77: call 0x14167c8a0
141720f7c: jmp 0x141720f8c
141720f7e: nop 
141720f80: lea rcx, [rbp + 0x550]
141720f87: call 0x1416d64e0
141720f8c: cmp dword ptr [rbp + 0x550], 0xb
141720f93: jne 0x141720fe0
141720f95: cmp byte ptr [rbp + 0x558], 0
141720f9c: jne 0x141721230
141720fa2: mov rax, qword ptr [rbp - 0x58]
141720fa6: cmp rax, 3
141720faa: lea rsi, [rbp - 0x58]
141720fae: mov ecx, 0
141720fb3: cmove rsi, rcx
141720fb7: cmp eax, 3
141720fba: jne 0x1417210e0
141720fc0: xor ecx, ecx
141720fc2: mov edx, 0x989680
141720fc7: call 0x140d55980
141720fcc: lea r13, [rbp + 0x660]
141720fd3: jmp 0x1417207c1
141720fd8: nop dword ptr [rax + rax]
141720fe0: movups xmm0, xmmword ptr [rbp + 0x550]
141720fe7: movups xmm1, xmmword ptr [rbp + 0x560]
141720fee: movups xmm2, xmmword ptr [rbp + 0x570]
141720ff5: movups xmm3, xmmword ptr [rbp + 0x580]
141720ffc: movaps xmmword ptr [rbp + 0x590], xmm0
141721003: movaps xmmword ptr [rbp + 0x5c0], xmm3
14172100a: movaps xmmword ptr [rbp + 0x5b0], xmm2
141721011: movaps xmmword ptr [rbp + 0x5a0], xmm1
141721018: cmp dword ptr [rbp + 0x590], 3
14172101f: jne 0x141720760
141721025: mov rsi, qword ptr [rbp + 0x710]
14172102c: mov byte ptr [rbp + 0x686], 1
141721033: lea rcx, [rbp + 0x250]
14172103a: lea rdx, [rbp + 0x5c0]
141721041: call 0x140b46000
141721046: lea rcx, [rbp + 0x250]
14172104d: call 0x140d76ed0
141721052: mov r8, rdx
141721055: mov qword ptr [rsp + 0x20], 0xc
14172105e: mov rcx, r13
141721061: mov rdx, rax
141721064: lea r9, [rip + 0x15a963d]
14172106b: call 0x140d62810
141721070: mov rcx, r13
141721073: call 0x140d76ed0
141721078: mov r8, rdx
14172107b: add rsi, 0x10
14172107f: mov rcx, rsi
141721082: mov rdx, rax
141721085: call 0x1417201a0
14172108a: test al, al
14172108c: jne 0x14172145f
141721092: mov rdx, qword ptr [rbp + 0x660]
141721099: test rdx, rdx
14172109c: je 0x1417210b0
14172109e: mov rcx, qword ptr [rbp + 0x668]
1417210a5: mov r8d, 1
1417210ab: call 0x140613c20
1417210b0: mov rdx, qword ptr [rbp + 0x250]
1417210b7: test rdx, rdx
1417210ba: je 0x141720760
1417210c0: mov rcx, qword ptr [rbp + 0x258]
1417210c7: mov r8d, 1
1417210cd: call 0x140613c20
1417210d2: jmp 0x141720760
1417210d7: nop word ptr [rax + rax]
1417210e0: call 0x140d635e0
1417210e5: mov r12, rax
1417210e8: mov ebx, edx
1417210ea: xor r13d, r13d
1417210ed: nop dword ptr [rax]
1417210f0: movzx eax, r13b
1417210f4: movsxd rcx, dword ptr [r14 + rax*4]
1417210f8: add rcx, r14
1417210fb: mov eax, r13d
1417210fe: mov edx, r13d
141721101: mov r13b, 2
141721104: jmp rcx
141721106: mov r13b, 1
141721109: nop dword ptr [rax]
141721110: mov edx, eax
141721112: movzx eax, dl
141721115: lea rcx, [rax + rax*4]
141721119: lea rax, [rdi + rcx*8]
14172111d: add rax, 0xc8
141721123: mov r9, qword ptr [rdi + rcx*8 + 0xc8]
14172112b: mov rcx, r9
14172112e: neg rcx
141721131: jo 0x1417210f0
141721133: xor ecx, ecx
141721135: cmp qword ptr [rax + 0x18], r12
141721139: setne cl
14172113c: cmovb ecx, r15d
141721140: xor r8d, r8d
141721143: cmp dword ptr [rax + 0x20], ebx
141721146: setne r8b
14172114a: cmovb r8d, r15d
14172114e: test cl, cl
141721150: cmovne r8d, ecx
141721154: cmp r8b, 1
141721158: je 0x1417210f0
14172115a: mov qword ptr [rbp + 0x658], r9
141721161: mov r8, qword ptr [rax + 8]
141721165: mov r9, qword ptr [rax + 0x10]
141721169: movabs rcx, 0x8000000000000000
141721173: mov qword ptr [rax], rcx
141721176: mov rcx, rsi
141721179: mov qword ptr [rbp + 0x648], r8
141721180: call 0x14171ece0
141721185: mov rdx, qword ptr [rbp + 0x658]
14172118c: test rdx, rdx
14172118f: je 0x1417210f0
141721195: mov r8d, 1
14172119b: mov rcx, qword ptr [rbp + 0x648]
1417211a2: call 0x140613c20
1417211a7: movzx eax, r13b
1417211ab: movsxd rcx, dword ptr [r14 + rax*4]
1417211af: add rcx, r14
1417211b2: mov edx, eax
1417211b4: mov r13b, 2
1417211b7: jmp rcx
1417211b9: mov r13b, 3
1417211bc: jmp 0x141721110
1417211c1: mov r13b, 7
1417211c4: jmp 0x141721110
1417211c9: mov r13b, 5
1417211cc: jmp 0x141721110
1417211d1: mov r13b, 6
1417211d4: jmp 0x141721110
1417211d9: mov r13b, 0xa
1417211dc: jmp 0x141721110
1417211e1: mov r13b, 4
1417211e4: jmp 0x141721110
1417211e9: mov r13b, 8
1417211ec: jmp 0x141721110
1417211f1: mov r13b, 9
1417211f4: jmp 0x141721110
1417211f9: mov byte ptr [rbp + 0x687], 1
141721200: mov rcx, rdx
141721203: mov r8d, 0x3b9aca00
141721209: call 0x1428cd210
14172120e: mov rdx, qword ptr [rbp + 0x628]
141721215: jmp 0x14172093f
14172121a: mov byte ptr [rbp + 0x687], sil
141721221: mov rcx, rbx
141721224: xor edx, edx
141721226: call 0x1428cdb90
14172122b: jmp 0x141720da8
141721230: mov rax, qword ptr [rbp - 0x58]
141721234: mov qword ptr [rbp - 0x58], 3
14172123c: cmp rax, 3
141721240: je 0x14172134c
141721246: mov qword ptr [rbp + 0x250], rax
14172124d: lea rcx, [rbp + 0x258]
141721254: mov r8d, 0x2a0
14172125a: lea rdx, [rbp - 0x50]
14172125e: call 0x142923c00
141721263: lea rcx, [rbp + 0x250]
14172126a: call 0x14171f7f0
14172126f: test al, al
141721271: jne 0x141721340
141721277: mov rax, qword ptr [rip + 0x1f0b12a]
14172127e: mov rax, qword ptr [rax]
141721281: test rax, rax
141721284: je 0x141721340
14172128a: lea rcx, [rip + 0x15a97df]
141721291: call 0x14067dd80
141721296: movups xmm0, xmmword ptr [rax]
141721299: mov eax, dword ptr [rax + 0x10]
14172129c: mov qword ptr [rbp + 0x5c0], 1
1417212a7: lea rdx, [rip + 0x15a92b2]
1417212ae: mov qword ptr [rbp + 0x5c8], rdx
1417212b5: mov qword ptr [rbp + 0x5d0], 0x28
1417212c0: lea rcx, [rip + 0x15a9799]
1417212c7: mov qword ptr [rbp + 0x5e0], rcx
1417212ce: mov qword ptr [rbp + 0x5e8], 1
1417212d9: mov qword ptr [rbp + 0x5f0], 8
1417212e4: mov qword ptr [rbp + 0x590], 0
1417212ef: xorps xmm1, xmm1
1417212f2: movups xmmword ptr [rbp + 0x5f8], xmm1
1417212f9: mov qword ptr [rbp + 0x598], rdx
141721300: mov qword ptr [rbp + 0x5a0], 0x28
14172130b: mov qword ptr [rbp + 0x5a8], 0
141721316: movups xmmword ptr [rbp + 0x5b0], xmm0
14172131d: mov dword ptr [rbp + 0x5d8], 1
141721327: mov dword ptr [rbp + 0x5dc], eax
14172132d: lea rcx, [rbp + 0x685]
141721334: lea rdx, [rbp + 0x590]
14172133b: call 0x140d4a330
141721340: lea rcx, [rbp + 0x250]
141721347: call 0x140891060
14172134c: mov rsi, qword ptr [rbp + 0x638]
141721353: mov rdi, qword ptr [rbp + 0x640]
14172135a: test rdi, rdi
14172135d: je 0x1417213aa
14172135f: lea rbx, [rsi + 0x30]
141721363: jmp 0x141721379
141721365: nop word ptr cs:[rax + rax]
141721370: add rbx, 0x40
141721374: dec rdi
141721377: je 0x1417213aa
141721379: mov rdx, qword ptr [rbx - 0x30]
14172137d: test rdx, rdx
141721380: je 0x141721391
141721382: mov rcx, qword ptr [rbx - 0x28]
141721386: mov r8d, 1
14172138c: call 0x140613c20
141721391: mov rdx, qword ptr [rbx - 8]
141721395: test rdx, rdx
141721398: je 0x141721370
14172139a: mov rcx, qword ptr [rbx]
14172139d: mov r8d, 1
1417213a3: call 0x140613c20
1417213a8: jmp 0x141721370
1417213aa: mov rdx, qword ptr [rbp + 0x630]
1417213b1: test rdx, rdx
1417213b4: je 0x1417213c8
1417213b6: shl rdx, 6
1417213ba: mov r8d, 8
1417213c0: mov rcx, rsi
1417213c3: call 0x140613c20
1417213c8: lea rcx, [rbp + 0x500]
1417213cf: mov r8d, 0x20
1417213d5: mov r9d, 0x10
1417213db: lea rdx, [rbp + 0x520]
1417213e2: call 0x140052a30
1417213e7: cmp dword ptr [rbp - 0x58], 3
1417213eb: je 0x1417213f6
1417213ed: lea rcx, [rbp - 0x58]
1417213f1: call 0x140891060
1417213f6: mov rax, qword ptr [rbp + 0x710]
1417213fd: lock dec qword ptr [rax]
141721401: jne 0x14172140f
141721403: lea rcx, [rbp + 0x710]
14172140a: call 0x140b5ce40
14172140f: lea rcx, [rbp + 0x540]
141721416: call 0x140878f10
14172141b: lea rcx, [rbp + 0x610]
141721422: call 0x140877550
141721427: movaps xmm6, xmmword ptr [rbp + 0x690]
14172142e: add rsp, 0x728
141721435: pop rbx
141721436: pop rdi
141721437: pop rsi
141721438: pop r12
14172143a: pop r13
14172143c: pop r14
14172143e: pop r15
141721440: pop rbp
141721441: ret 
141721442: lea rcx, [rip + 0x15a952f]
141721449: lea r8, [rip + 0x15a9570]
141721450: mov edx, 0x44
141721455: call 0x1428d9430
14172145a: jmp 0x1417214f1
14172145f: mov rdx, qword ptr [rbp + 0x660]
141721466: test rdx, rdx
141721469: je 0x14172147d
14172146b: mov rcx, qword ptr [rbp + 0x668]
141721472: mov r8d, 1
141721478: call 0x140613c20
14172147d: mov rdx, qword ptr [rbp + 0x250]
141721484: test rdx, rdx
141721487: je 0x14172149b
141721489: mov rcx, qword ptr [rbp + 0x258]
141721490: mov r8d, 1
141721496: call 0x140613c20
14172149b: lea rax, [rip + 0x15a955e]
1417214a2: mov qword ptr [rbp + 0x250], rax
1417214a9: mov qword ptr [rbp + 0x258], 1
1417214b4: mov qword ptr [rbp + 0x260], 8
1417214bf: xorps xmm0, xmm0
1417214c2: movups xmmword ptr [rbp + 0x268], xmm0
1417214c9: mov byte ptr [rbp + 0x686], 1
1417214d0: lea rdx, [rip + 0x15a9539]
1417214d7: lea rcx, [rbp + 0x250]
1417214de: call 0x1428d9390
1417214e3: jmp 0x1417214f1
1417214e5: lea rcx, [rip + 0x15195b4]
1417214ec: call 0x1428d80d0
1417214f1: ud2 
1417214f3: nop word ptr cs:[rax + rax]
141721500: mov qword ptr [rsp + 0x10], rdx
141721505: push rbp
141721506: push r15
141721508: push r14
14172150a: push r13
14172150c: push r12
14172150e: push rsi
14172150f: push rdi
141721510: push rbx
141721511: sub rsp, 0x38
141721515: lea rbp, [rdx + 0x80]
14172151c: movaps xmmword ptr [rsp + 0x20], xmm6
141721521: cmp dword ptr [rbp - 0x58], 3
141721525: je 0x141721530
141721527: lea rcx, [rbp - 0x58]
14172152b: call 0x140891060
141721530: movaps xmm6, xmmword ptr [rsp + 0x20]
141721535: add rsp, 0x38
141721539: pop rbx
14172153a: pop rdi
14172153b: pop rsi
14172153c: pop r12
14172153e: pop r13
141721540: pop r14
141721542: pop r15
141721544: pop rbp
141721545: ret 
141721546: nop word ptr cs:[rax + rax]
141721550: mov qword ptr [rsp + 0x10], rdx
141721555: push rbp
141721556: push r15
141721558: push r14
14172155a: push r13
14172155c: push r12
14172155e: push rsi
14172155f: push rdi
141721560: push rbx
141721561: sub rsp, 0x38
141721565: lea rbp, [rdx + 0x80]
14172156c: movaps xmmword ptr [rsp + 0x20], xmm6
141721571: lea rcx, [rbp + 0x500]
141721578: mov r8d, 0x20
14172157e: mov r9d, 0x10
141721584: lea rdx, [rbp + 0x520]
14172158b: call 0x140052a30
141721590: movaps xmm6, xmmword ptr [rsp + 0x20]
141721595: add rsp, 0x38
141721599: pop rbx
14172159a: pop rdi
14172159b: pop rsi
14172159c: pop r12
14172159e: pop r13
1417215a0: pop r14
1417215a2: pop r15
1417215a4: pop rbp
1417215a5: ret 
1417215a6: nop word ptr cs:[rax + rax]
1417215b0: mov qword ptr [rsp + 0x10], rdx
1417215b5: push rbp
1417215b6: push r15
1417215b8: push r14
1417215ba: push r13
1417215bc: push r12
1417215be: push rsi
1417215bf: push rdi
1417215c0: push rbx
1417215c1: sub rsp, 0x38
1417215c5: lea rbp, [rdx + 0x80]
1417215cc: movaps xmmword ptr [rsp + 0x20], xmm6
1417215d1: mov rdx, qword ptr [rbp + 0x550]
1417215d8: test rdx, rdx
1417215db: je 0x1417215ef
1417215dd: mov rcx, qword ptr [rbp + 0x558]
1417215e4: mov r8d, 1
1417215ea: call 0x140613c20
1417215ef: mov byte ptr [rbp + 0x687], 1
1417215f6: movaps xmm6, xmmword ptr [rsp + 0x20]
1417215fb: add rsp, 0x38
1417215ff: pop rbx
141721600: pop rdi
141721601: pop rsi
141721602: pop r12
141721604: pop r13
141721606: pop r14
141721608: pop r15
14172160a: pop rbp
14172160b: ret 
14172160c: nop dword ptr [rax]
141721610: mov qword ptr [rsp + 0x10], rdx
141721615: push rbp
141721616: push r15
141721618: push r14
14172161a: push r13
14172161c: push r12
14172161e: push rsi
14172161f: push rdi
141721620: push rbx
141721621: sub rsp, 0x38
141721625: lea rbp, [rdx + 0x80]
14172162c: movaps xmmword ptr [rsp + 0x20], xmm6
141721631: lea rcx, [rbp + 0x250]
141721638: call 0x140b70a70
14172163d: mov byte ptr [rbp + 0x687], 1
141721644: movaps xmm6, xmmword ptr [rsp + 0x20]
141721649: add rsp, 0x38
14172164d: pop rbx
14172164e: pop rdi
14172164f: pop rsi
141721650: pop r12
141721652: pop r13
141721654: pop r14
141721656: pop r15
141721658: pop rbp
141721659: ret 
14172165a: nop word ptr [rax + rax]
141721660: mov qword ptr [rsp + 0x10], rdx
141721665: push rbp
141721666: push r15
141721668: push r14
14172166a: push r13
14172166c: push r12
14172166e: push rsi
14172166f: push rdi
141721670: push rbx
141721671: sub rsp, 0x38
141721675: lea rbp, [rdx + 0x80]
14172167c: movaps xmmword ptr [rsp + 0x20], xmm6
141721681: mov rcx, qword ptr [rbp + 0x628]
141721688: call 0x140161c30
14172168d: mov byte ptr [rbp + 0x687], 1
141721694: movaps xmm6, xmmword ptr [rsp + 0x20]
141721699: add rsp, 0x38
14172169d: pop rbx
14172169e: pop rdi
14172169f: pop rsi
1417216a0: pop r12
1417216a2: pop r13
1417216a4: pop r14
1417216a6: pop r15
1417216a8: pop rbp
1417216a9: ret 
1417216aa: nop word ptr [rax + rax]
1417216b0: mov qword ptr [rsp + 0x10], rdx
1417216b5: push rbp
1417216b6: push r15
1417216b8: push r14
1417216ba: push r13
1417216bc: push r12
1417216be: push rsi
1417216bf: push rdi
1417216c0: push rbx
1417216c1: sub rsp, 0x38
1417216c5: lea rbp, [rdx + 0x80]
1417216cc: movaps xmmword ptr [rsp + 0x20], xmm6
1417216d1: mov rdx, qword ptr [rbp + 0x590]
1417216d8: test rdx, rdx
1417216db: je 0x1417216ef
1417216dd: mov rcx, qword ptr [rbp + 0x598]
1417216e4: mov r8d, 1
1417216ea: call 0x140613c20
1417216ef: movaps xmm6, xmmword ptr [rsp + 0x20]
1417216f4: add rsp, 0x38
1417216f8: pop rbx
1417216f9: pop rdi
1417216fa: pop rsi
1417216fb: pop r12
1417216fd: pop r13
1417216ff: pop r14
141721701: pop r15
141721703: pop rbp
141721704: ret 
141721705: nop word ptr cs:[rax + rax]
141721710: mov qword ptr [rsp + 0x10], rdx
141721715: push rbp
141721716: push r15
141721718: push r14
14172171a: push r13
14172171c: push r12
14172171e: push rsi
14172171f: push rdi
141721720: push rbx
141721721: sub rsp, 0x38
141721725: lea rbp, [rdx + 0x80]
14172172c: movaps xmmword ptr [rsp + 0x20], xmm6
141721731: mov rdx, qword ptr [rbp + 0x250]
141721738: test rdx, rdx
14172173b: je 0x14172174f
14172173d: mov rcx, qword ptr [rbp + 0x258]
141721744: mov r8d, 1
14172174a: call 0x140613c20
14172174f: mov byte ptr [rbp + 0x686], 1
141721756: movaps xmm6, xmmword ptr [rsp + 0x20]
14172175b: add rsp, 0x38
14172175f: pop rbx
141721760: pop rdi
141721761: pop rsi
141721762: pop r12
141721764: pop r13
141721766: pop r14
141721768: pop r15
14172176a: pop rbp
14172176b: ret 
14172176c: nop dword ptr [rax]
141721770: mov qword ptr [rsp + 0x10], rdx
141721775: push rbp
141721776: push r15
141721778: push r14
14172177a: push r13
14172177c: push r12
14172177e: push rsi
14172177f: push rdi
141721780: push rbx
141721781: sub rsp, 0x38
141721785: lea rbp, [rdx + 0x80]
14172178c: movaps xmmword ptr [rsp + 0x20], xmm6
141721791: mov rdx, qword ptr [rbp + 0x660]
141721798: test rdx, rdx
14172179b: je 0x1417217af
14172179d: mov rcx, qword ptr [rbp + 0x668]
1417217a4: mov r8d, 1
1417217aa: call 0x140613c20
1417217af: movaps xmm6, xmmword ptr [rsp + 0x20]
1417217b4: add rsp, 0x38
1417217b8: pop rbx
1417217b9: pop rdi
1417217ba: pop rsi
1417217bb: pop r12
1417217bd: pop r13
1417217bf: pop r14
1417217c1: pop r15
1417217c3: pop rbp
1417217c4: ret 
1417217c5: nop word ptr cs:[rax + rax]
1417217d0: mov qword ptr [rsp + 0x10], rdx
1417217d5: push rbp
1417217d6: push r15
1417217d8: push r14
1417217da: push r13
1417217dc: push r12
1417217de: push rsi
1417217df: push rdi
1417217e0: push rbx
1417217e1: sub rsp, 0x38
1417217e5: lea rbp, [rdx + 0x80]
1417217ec: movaps xmmword ptr [rsp + 0x20], xmm6
1417217f1: cmp byte ptr [rbp + 0x686], 0
1417217f8: je 0x14172181c
1417217fa: lea rcx, [rbp + 0x590]
141721801: call 0x14171e330
141721806: movaps xmm6, xmmword ptr [rsp + 0x20]
14172180b: add rsp, 0x38
14172180f: pop rbx
141721810: pop rdi
141721811: pop rsi
141721812: pop r12
141721814: pop r13
141721816: pop r14
141721818: pop r15
14172181a: pop rbp
14172181b: ret 
14172181c: movaps xmm6, xmmword ptr [rsp + 0x20]
141721821: add rsp, 0x38
141721825: pop rbx
141721826: pop rdi
141721827: pop rsi
141721828: pop r12
14172182a: pop r13
14172182c: pop r14
14172182e: pop r15
141721830: pop rbp
141721831: ret 
141721832: nop word ptr cs:[rax + rax]
141721840: mov qword ptr [rsp + 0x10], rdx
141721845: push rbp
141721846: push r15
141721848: push r14
14172184a: push r13
14172184c: push r12
14172184e: push rsi
14172184f: push rdi
141721850: push rbx
141721851: sub rsp, 0x38
141721855: lea rbp, [rdx + 0x80]
14172185c: movaps xmmword ptr [rsp + 0x20], xmm6
141721861: mov rdx, qword ptr [rbp + 0x658]
141721868: test rdx, rdx
14172186b: je 0x14172187f
14172186d: mov r8d, 1
141721873: mov rcx, qword ptr [rbp + 0x648]
14172187a: call 0x140613c20
14172187f: movaps xmm6, xmmword ptr [rsp + 0x20]
141721884: add rsp, 0x38
141721888: pop rbx
141721889: pop rdi
14172188a: pop rsi
14172188b: pop r12
14172188d: pop r13
14172188f: pop r14
141721891: pop r15
141721893: pop rbp
141721894: ret 
141721895: nop word ptr cs:[rax + rax]
1417218a0: mov qword ptr [rsp + 0x10], rdx
1417218a5: push rbp
1417218a6: push r15
1417218a8: push r14
1417218aa: push r13
1417218ac: push r12
1417218ae: push rsi
1417218af: push rdi
1417218b0: push rbx
1417218b1: sub rsp, 0x38
1417218b5: lea rbp, [rdx + 0x80]
1417218bc: movaps xmmword ptr [rsp + 0x20], xmm6
1417218c1: mov rax, qword ptr [rbp + 0x710]
1417218c8: lock dec qword ptr [rax]
1417218cc: jne 0x1417218da
1417218ce: lea rcx, [rbp + 0x710]
1417218d5: call 0x140b5ce40
1417218da: movaps xmm6, xmmword ptr [rsp + 0x20]
1417218df: add rsp, 0x38
1417218e3: pop rbx
1417218e4: pop rdi
1417218e5: pop rsi
1417218e6: pop r12
1417218e8: pop r13
1417218ea: pop r14
1417218ec: pop r15
1417218ee: pop rbp
1417218ef: ret 
1417218f0: mov qword ptr [rsp + 0x10], rdx
1417218f5: push rbp
1417218f6: push r15
1417218f8: push r14
1417218fa: push r13
1417218fc: push r12
1417218fe: push rsi
1417218ff: push rdi
141721900: push rbx
141721901: sub rsp, 0x38
141721905: lea rbp, [rdx + 0x80]
14172190c: movaps xmmword ptr [rsp + 0x20], xmm6
141721911: lea rcx, [rbp + 0x540]
141721918: call 0x140878f10
14172191d: movaps xmm6, xmmword ptr [rsp + 0x20]
141721922: add rsp, 0x38
141721926: pop rbx
141721927: pop rdi
141721928: pop rsi
141721929: pop r12
14172192b: pop r13
14172192d: pop r14
14172192f: pop r15
141721931: pop rbp
141721932: ret 
141721933: nop word ptr cs:[rax + rax]
141721940: mov qword ptr [rsp + 0x10], rdx
141721945: push rbp
141721946: push r15
141721948: push r14
14172194a: push r13
14172194c: push r12
14172194e: push rsi
14172194f: push rdi
141721950: push rbx
141721951: sub rsp, 0x38
141721955: lea rbp, [rdx + 0x80]
14172195c: movaps xmmword ptr [rsp + 0x20], xmm6
141721961: lea rcx, [rbp + 0x610]
141721968: call 0x140877550
14172196d: movaps xmm6, xmmword ptr [rsp + 0x20]
141721972: add rsp, 0x38
141721976: pop rbx
141721977: pop rdi
141721978: pop rsi
141721979: pop r12
14172197b: pop r13
14172197d: pop r14
14172197f: pop r15
141721981: pop rbp
141721982: ret 
141721983: nop word ptr cs:[rax + rax]
141721990: mov qword ptr [rsp + 0x10], rdx
141721995: push rbp
141721996: push r15
141721998: push r14
14172199a: push r13
14172199c: push r12
14172199e: push rsi
14172199f: push rdi
1417219a0: push rbx
1417219a1: sub rsp, 0x38
1417219a5: lea rbp, [rdx + 0x80]
1417219ac: movaps xmmword ptr [rsp + 0x20], xmm6
1417219b1: test byte ptr [rbp + 0x687], 1
1417219b8: je 0x1417219dc
1417219ba: lea rcx, [rbp + 0x660]
1417219c1: call 0x140603bb0
1417219c6: movaps xmm6, xmmword ptr [rsp + 0x20]
1417219cb: add rsp, 0x38
1417219cf: pop rbx
1417219d0: pop rdi
1417219d1: pop rsi
1417219d2: pop r12
1417219d4: pop r13
1417219d6: pop r14
1417219d8: pop r15
1417219da: pop rbp
1417219db: ret 
1417219dc: movaps xmm6, xmmword ptr [rsp + 0x20]
1417219e1: add rsp, 0x38
1417219e5: pop rbx
1417219e6: pop rdi
1417219e7: pop rsi
1417219e8: pop r12
1417219ea: pop r13
1417219ec: pop r14
1417219ee: pop r15
1417219f0: pop rbp
1417219f1: ret 
1417219f2: nop word ptr cs:[rax + rax]
141721a00: mov qword ptr [rsp + 0x10], rdx
141721a05: push rbp
141721a06: push r15
141721a08: push r14
141721a0a: push r13
141721a0c: push r12
141721a0e: push rsi
141721a0f: push rdi
141721a10: push rbx
141721a11: sub rsp, 0x38
141721a15: lea rbp, [rdx + 0x80]
141721a1c: movaps xmmword ptr [rsp + 0x20], xmm6
141721a21: lea rcx, [rbp + 0x630]
141721a28: call 0x140603bb0
141721a2d: movaps xmm6, xmmword ptr [rsp + 0x20]
141721a32: add rsp, 0x38
141721a36: pop rbx
141721a37: pop rdi
141721a38: pop rsi
141721a39: pop r12
141721a3b: pop r13
141721a3d: pop r14
141721a3f: pop r15
141721a41: pop rbp
141721a42: ret 
141721a43: nop word ptr cs:[rax + rax]
141721a50: mov qword ptr [rsp + 0x10], rdx
141721a55: push rbp
141721a56: push r15
141721a58: push r14
141721a5a: push r13
141721a5c: push r12
141721a5e: push rsi
141721a5f: push rdi
141721a60: push rbx
141721a61: sub rsp, 0x38
141721a65: lea rbp, [rdx + 0x80]
141721a6c: movaps xmmword ptr [rsp + 0x20], xmm6
141721a71: lea rcx, [rbp + 0x250]
141721a78: call 0x140891060
141721a7d: movaps xmm6, xmmword ptr [rsp + 0x20]
141721a82: add rsp, 0x38
141721a86: pop rbx
141721a87: pop rdi
141721a88: pop rsi
141721a89: pop r12
141721a8b: pop r13
141721a8d: pop r14
141721a8f: pop r15
141721a91: pop rbp
141721a92: ret 
