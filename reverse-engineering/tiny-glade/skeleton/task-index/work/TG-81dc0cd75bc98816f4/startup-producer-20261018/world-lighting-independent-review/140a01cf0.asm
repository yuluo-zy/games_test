140a01cf0: 55 push rbp
140a01cf1: 4157 push r15
140a01cf3: 4156 push r14
140a01cf5: 4155 push r13
140a01cf7: 4154 push r12
140a01cf9: 56 push rsi
140a01cfa: 57 push rdi
140a01cfb: 53 push rbx
140a01cfc: 4881ec98060000 sub rsp, 0x698
140a01d03: 488dac2480000000 lea rbp, [rsp + 0x80]
140a01d0b: 48c78510060000feffffff mov qword ptr [rbp + 0x610], 0xfffffffffffffffe
140a01d16: 4c89c3 mov rbx, r8
140a01d19: 4989d7 mov r15, rdx
140a01d1c: 4889ce mov rsi, rcx
140a01d1f: 410f104128 movups xmm0, xmmword ptr [r9 + 0x28]
140a01d24: 4c898dc8020000 mov qword ptr [rbp + 0x2c8], r9
140a01d2b: 410f104938 movups xmm1, xmmword ptr [r9 + 0x38]
140a01d30: 0f298de0020000 movaps xmmword ptr [rbp + 0x2e0], xmm1
140a01d37: 0f2985d0020000 movaps xmmword ptr [rbp + 0x2d0], xmm0
140a01d3e: 48b80200000002000000 movabs rax, 0x200000002
140a01d48: 488985f0000000 mov qword ptr [rbp + 0xf0], rax
140a01d4f: c785f800000001000000 mov dword ptr [rbp + 0xf8], 1
140a01d59: 488dbd60040000 lea rdi, [rbp + 0x460]
140a01d60: 488d95d0020000 lea rdx, [rbp + 0x2d0]
140a01d67: 4c8d85f0000000 lea r8, [rbp + 0xf0]
140a01d6e: 4889f9 mov rcx, rdi
140a01d71: e86a381d00 call 0x140bd55e0
140a01d76: 4889f9 mov rcx, rdi
140a01d79: e8123a1d00 call 0x140bd5790
140a01d7e: 4989c6 mov r14, rax
140a01d81: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a01d88: 4889c2 mov rdx, rax
140a01d8b: 41b861000000 mov r8d, 0x61
140a01d91: e80a351d00 call 0x140bd52a0
140a01d96: 0f1085f0000000 movups xmm0, xmmword ptr [rbp + 0xf0]
140a01d9d: 0f108d00010000 movups xmm1, xmmword ptr [rbp + 0x100]
140a01da4: 0f298d70040000 movaps xmmword ptr [rbp + 0x470], xmm1
140a01dab: 0f298560040000 movaps xmmword ptr [rbp + 0x460], xmm0
140a01db2: 488d8de0020000 lea rcx, [rbp + 0x2e0]
140a01db9: 488d9560040000 lea rdx, [rbp + 0x460]
140a01dc0: e81b4f1d00 call 0x140bd6ce0
140a01dc5: 488d05543d0c02 lea rax, [rip + 0x20c3d54]
140a01dcc: 488985d0020000 mov qword ptr [rbp + 0x2d0], rax
140a01dd3: 48c785d802000015000000 mov qword ptr [rbp + 0x2d8], 0x15
140a01dde: 488d95d0020000 lea rdx, [rbp + 0x2d0]
140a01de5: 4c89f9 mov rcx, r15
140a01de8: e8e3761e00 call 0x140be94d0
140a01ded: 0f1085f0000000 movups xmm0, xmmword ptr [rbp + 0xf0]
140a01df4: 0f108d00010000 movups xmm1, xmmword ptr [rbp + 0x100]
140a01dfb: 0f114540 movups xmmword ptr [rbp + 0x40], xmm0
140a01dff: 0f114d50 movups xmmword ptr [rbp + 0x50], xmm1
140a01e03: 89453c mov dword ptr [rbp + 0x3c], eax
140a01e06: 4c8da5f0000000 lea r12, [rbp + 0xf0]
140a01e0d: 4c89e1 mov rcx, r12
140a01e10: 4c89f2 mov rdx, r14
140a01e13: 41b86d000000 mov r8d, 0x6d
140a01e19: e882341d00 call 0x140bd52a0
140a01e1e: 0f1085f0000000 movups xmm0, xmmword ptr [rbp + 0xf0]
140a01e25: 0f108d00010000 movups xmm1, xmmword ptr [rbp + 0x100]
140a01e2c: 0f298d70040000 movaps xmmword ptr [rbp + 0x470], xmm1
140a01e33: 0f298560040000 movaps xmmword ptr [rbp + 0x460], xmm0
140a01e3a: 488d8de0020000 lea rcx, [rbp + 0x2e0]
140a01e41: 488d9560040000 lea rdx, [rbp + 0x460]
140a01e48: e8934e1d00 call 0x140bd6ce0
140a01e4d: 488d05e13c0c02 lea rax, [rip + 0x20c3ce1]
140a01e54: 488985d0020000 mov qword ptr [rbp + 0x2d0], rax
140a01e5b: 48c785d802000010000000 mov qword ptr [rbp + 0x2d8], 0x10
140a01e66: 488d95d0020000 lea rdx, [rbp + 0x2d0]
140a01e6d: 4c89f9 mov rcx, r15
140a01e70: e85b761e00 call 0x140be94d0
140a01e75: 0f1085f0000000 movups xmm0, xmmword ptr [rbp + 0xf0]
140a01e7c: 0f108d00010000 movups xmm1, xmmword ptr [rbp + 0x100]
140a01e83: 0f11451c movups xmmword ptr [rbp + 0x1c], xmm0
140a01e87: 0f114d2c movups xmmword ptr [rbp + 0x2c], xmm1
140a01e8b: 894518 mov dword ptr [rbp + 0x18], eax
140a01e8e: 4c897c2430 mov qword ptr [rsp + 0x30], r15
140a01e93: 48899d00060000 mov qword ptr [rbp + 0x600], rbx
140a01e9a: 48895c2428 mov qword ptr [rsp + 0x28], rbx
140a01e9f: 48c74424201f000000 mov qword ptr [rsp + 0x20], 0x1f
140a01ea8: 488d15963c0c02 lea rdx, [rip + 0x20c3c96]
140a01eaf: 4c8d0da23c0c02 lea r9, [rip + 0x20c3ca2]
140a01eb6: 488d8d60040000 lea rcx, [rbp + 0x460]
140a01ebd: 41b813000000 mov r8d, 0x13
140a01ec3: e8d813c000 call 0x1416032a0
140a01ec8: 80bd9605000008 cmp byte ptr [rbp + 0x596], 8
140a01ecf: 7425 je 0x140a01ef6
140a01ed1: 488d8d70050000 lea rcx, [rbp + 0x570]
140a01ed8: bae8000000 mov edx, 0xe8
140a01edd: 48039598050000 add rdx, qword ptr [rbp + 0x598]
140a01ee4: 49b84ca5591f44988a41 movabs r8, 0x418a98441f59a54c
140a01eee: 41b101 mov r9b, 1
140a01ef1: e85a99f6ff call 0x14096b850
140a01ef6: 488d9dd0020000 lea rbx, [rbp + 0x2d0]
140a01efd: 4c8dad60040000 lea r13, [rbp + 0x460]
140a01f04: 41b890010000 mov r8d, 0x190
140a01f0a: 4889d9 mov rcx, rbx
140a01f0d: 4c89ea mov rdx, r13
140a01f10: e8eb1cf201 call 0x142923c00
140a01f15: c685a002000011 mov byte ptr [rbp + 0x2a0], 0x11
140a01f1c: c785f000000002000000 mov dword ptr [rbp + 0xf0], 2
140a01f26: 4c89642420 mov qword ptr [rsp + 0x20], r12
140a01f2b: 488d8da0020000 lea rcx, [rbp + 0x2a0]
140a01f32: 49b8a41a84c94d1f6511 movabs r8, 0x11651f4dc9841aa4
140a01f3c: 4889da mov rdx, rbx
140a01f3f: 41b119 mov r9b, 0x19
140a01f42: e81933f0ff call 0x140905260
140a01f47: 0fb69d98060000 movzx ebx, byte ptr [rbp + 0x698]
140a01f4e: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a01f55: 488d95d0020000 lea rdx, [rbp + 0x2d0]
140a01f5c: 41b890010000 mov r8d, 0x190
140a01f62: e8991cf201 call 0x142923c00
140a01f67: 0fb6fb movzx edi, bl
140a01f6a: 80bd2602000008 cmp byte ptr [rbp + 0x226], 8
140a01f71: 7425 je 0x140a01f98
140a01f73: 488d8d00020000 lea rcx, [rbp + 0x200]
140a01f7a: bae8000000 mov edx, 0xe8
140a01f7f: 48039528020000 add rdx, qword ptr [rbp + 0x228]
140a01f86: 49b802d9ee60b709a1a9 movabs r8, 0xa9a109b760eed902
140a01f90: 4189f9 mov r9d, edi
140a01f93: e8d895f6ff call 0x14096b570
140a01f98: 89bdfc050000 mov dword ptr [rbp + 0x5fc], edi
140a01f9e: 488b85c8020000 mov rax, qword ptr [rbp + 0x2c8]
140a01fa5: 488d7824 lea rdi, [rax + 0x24]
140a01fa9: 488d9dd0020000 lea rbx, [rbp + 0x2d0]
140a01fb0: 488d95f0000000 lea rdx, [rbp + 0xf0]
140a01fb7: 41b890010000 mov r8d, 0x190
140a01fbd: 4889d9 mov rcx, rbx
140a01fc0: e83b1cf201 call 0x142923c00
140a01fc5: 4889bd88020000 mov qword ptr [rbp + 0x288], rdi
140a01fcc: 4889bdf0000000 mov qword ptr [rbp + 0xf0], rdi
140a01fd3: c7856004000002000000 mov dword ptr [rbp + 0x460], 2
140a01fdd: 4c896c2420 mov qword ptr [rsp + 0x20], r13
140a01fe2: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a01fe9: 49b8fcfb49a09d576148 movabs r8, 0x4861579da049fbfc
140a01ff3: 4889da mov rdx, rbx
140a01ff6: 41b119 mov r9b, 0x19
140a01ff9: e842c2eeff call 0x1408ee240
140a01ffe: 488bbd90060000 mov rdi, qword ptr [rbp + 0x690]
140a02005: 488d9d60040000 lea rbx, [rbp + 0x460]
140a0200c: 4c8da5d0020000 lea r12, [rbp + 0x2d0]
140a02013: 41b890010000 mov r8d, 0x190
140a02019: 4889d9 mov rcx, rbx
140a0201c: 4c89e2 mov rdx, r12
140a0201f: e8dc1bf201 call 0x142923c00
140a02024: 4889bdf0000000 mov qword ptr [rbp + 0xf0], rdi
140a0202b: c785d002000002000000 mov dword ptr [rbp + 0x2d0], 2
140a02035: 4c89642420 mov qword ptr [rsp + 0x20], r12
140a0203a: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a02041: 49b8d0096c8285511704 movabs r8, 0x4175185826c09d0
140a0204b: 4889da mov rdx, rbx
140a0204e: 41b119 mov r9b, 0x19
140a02051: e8eac1eeff call 0x1408ee240
140a02056: 488bbda0060000 mov rdi, qword ptr [rbp + 0x6a0]
140a0205d: 488d9dd0020000 lea rbx, [rbp + 0x2d0]
140a02064: 4c8da560040000 lea r12, [rbp + 0x460]
140a0206b: 41b890010000 mov r8d, 0x190
140a02071: 4889d9 mov rcx, rbx
140a02074: 4c89e2 mov rdx, r12
140a02077: e8841bf201 call 0x142923c00
140a0207c: 4889bdf0000000 mov qword ptr [rbp + 0xf0], rdi
140a02083: c7856004000002000000 mov dword ptr [rbp + 0x460], 2
140a0208d: 4c89642420 mov qword ptr [rsp + 0x20], r12
140a02092: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a02099: 49b8e36b5a83cd4ca7b8 movabs r8, 0xb8a74ccd835a6be3
140a020a3: 4889da mov rdx, rbx
140a020a6: 41b119 mov r9b, 0x19
140a020a9: e892c1eeff call 0x1408ee240
140a020ae: 488bbda8060000 mov rdi, qword ptr [rbp + 0x6a8]
140a020b5: 488d9d60040000 lea rbx, [rbp + 0x460]
140a020bc: 4c8da5d0020000 lea r12, [rbp + 0x2d0]
140a020c3: 41b890010000 mov r8d, 0x190
140a020c9: 4889d9 mov rcx, rbx
140a020cc: 4c89e2 mov rdx, r12
140a020cf: e82c1bf201 call 0x142923c00
140a020d4: 4889bdf0000000 mov qword ptr [rbp + 0xf0], rdi
140a020db: c785d002000002000000 mov dword ptr [rbp + 0x2d0], 2
140a020e5: 4c89642420 mov qword ptr [rsp + 0x20], r12
140a020ea: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a020f1: 49b8f99a3964bbe203e0 movabs r8, 0xe003e2bb64399af9
140a020fb: 4889da mov rdx, rbx
140a020fe: 41b119 mov r9b, 0x19
140a02101: e83ac1eeff call 0x1408ee240
140a02106: 488bbd80060000 mov rdi, qword ptr [rbp + 0x680]
140a0210d: 488d9dd0020000 lea rbx, [rbp + 0x2d0]
140a02114: 4c8da560040000 lea r12, [rbp + 0x460]
140a0211b: 41b890010000 mov r8d, 0x190
140a02121: 4889d9 mov rcx, rbx
140a02124: 4c89e2 mov rdx, r12
140a02127: e8d41af201 call 0x142923c00
140a0212c: 4889bdf0000000 mov qword ptr [rbp + 0xf0], rdi
140a02133: c7856004000002000000 mov dword ptr [rbp + 0x460], 2
140a0213d: 4c89642420 mov qword ptr [rsp + 0x20], r12
140a02142: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a02149: 49b8308202283a3c332a movabs r8, 0x2a333c3a28028230
140a02153: 4889da mov rdx, rbx
140a02156: 41b119 mov r9b, 0x19
140a02159: e8e2c0eeff call 0x1408ee240
140a0215e: 488bbd88060000 mov rdi, qword ptr [rbp + 0x688]
140a02165: 488d9d60040000 lea rbx, [rbp + 0x460]
140a0216c: 4c8dadd0020000 lea r13, [rbp + 0x2d0]
140a02173: 41b890010000 mov r8d, 0x190
140a02179: 4889d9 mov rcx, rbx
140a0217c: 4c89ea mov rdx, r13
140a0217f: e87c1af201 call 0x142923c00
140a02184: 4889bda0020000 mov qword ptr [rbp + 0x2a0], rdi
140a0218b: c785d002000002000000 mov dword ptr [rbp + 0x2d0], 2
140a02195: 4c896c2420 mov qword ptr [rsp + 0x20], r13
140a0219a: 488d8da0020000 lea rcx, [rbp + 0x2a0]
140a021a1: 49b8101d0514c78d0f35 movabs r8, 0x350f8dc714051d10
140a021ab: 4889da mov rdx, rbx
140a021ae: 41b119 mov r9b, 0x19
140a021b1: e88ac0eeff call 0x1408ee240
140a021b6: 488d9df0000000 lea rbx, [rbp + 0xf0]
140a021bd: 488d9560040000 lea rdx, [rbp + 0x460]
140a021c4: 41b890010000 mov r8d, 0x190
140a021ca: 4889d9 mov rcx, rbx
140a021cd: e82e1af201 call 0x142923c00
140a021d2: c6856004000009 mov byte ptr [rbp + 0x460], 9
140a021d9: c785d002000002000000 mov dword ptr [rbp + 0x2d0], 2
140a021e3: 4c896c2420 mov qword ptr [rsp + 0x20], r13
140a021e8: 49b8f2c570045e8bb061 movabs r8, 0x61b08b5e0470c5f2
140a021f2: 4c89e1 mov rcx, r12
140a021f5: 4889da mov rdx, rbx
140a021f8: 41b119 mov r9b, 0x19
140a021fb: e86030f0ff call 0x140905260
140a02200: 488d9dd0020000 lea rbx, [rbp + 0x2d0]
140a02207: 488d95f0000000 lea rdx, [rbp + 0xf0]
140a0220e: 41b890010000 mov r8d, 0x190
140a02214: 4889d9 mov rcx, rbx
140a02217: e8e419f201 call 0x142923c00
140a0221c: c685a00200000a mov byte ptr [rbp + 0x2a0], 0xa
140a02223: c7856004000002000000 mov dword ptr [rbp + 0x460], 2
140a0222d: 4c89642420 mov qword ptr [rsp + 0x20], r12
140a02232: 488d8da0020000 lea rcx, [rbp + 0x2a0]
140a02239: 49b857f6e8da4c1e7d29 movabs r8, 0x297d1e4cdae8f657
140a02243: 4889da mov rdx, rbx
140a02246: 41b119 mov r9b, 0x19
140a02249: e81230f0ff call 0x140905260
140a0224e: 488d9d60040000 lea rbx, [rbp + 0x460]
140a02255: 4c8da5d0020000 lea r12, [rbp + 0x2d0]
140a0225c: 41b890010000 mov r8d, 0x190
140a02262: 4889d9 mov rcx, rbx
140a02265: 4c89e2 mov rdx, r12
140a02268: e89319f201 call 0x142923c00
140a0226d: c685f000000008 mov byte ptr [rbp + 0xf0], 8
140a02274: c785d002000002000000 mov dword ptr [rbp + 0x2d0], 2
140a0227e: 4c89642420 mov qword ptr [rsp + 0x20], r12
140a02283: 4c8da5f0000000 lea r12, [rbp + 0xf0]
140a0228a: 49b83fca8dac13011677 movabs r8, 0x77160113ac8dca3f
140a02294: 4c89e1 mov rcx, r12
140a02297: 4889da mov rdx, rbx
140a0229a: 41b119 mov r9b, 0x19
140a0229d: e8be2ff0ff call 0x140905260
140a022a2: 488d9dd0020000 lea rbx, [rbp + 0x2d0]
140a022a9: 4c8dad60040000 lea r13, [rbp + 0x460]
140a022b0: 41b890010000 mov r8d, 0x190
140a022b6: 4889d9 mov rcx, rbx
140a022b9: 4c89ea mov rdx, r13
140a022bc: e83f19f201 call 0x142923c00
140a022c1: 48b86518a091ab399f57 movabs rax, 0x579f39ab91a01865
140a022cb: 488985a0020000 mov qword ptr [rbp + 0x2a0], rax
140a022d2: c785f000000002000000 mov dword ptr [rbp + 0xf0], 2
140a022dc: 4c89642420 mov qword ptr [rsp + 0x20], r12
140a022e1: 488d8da0020000 lea rcx, [rbp + 0x2a0]
140a022e8: 49b88c97666903f55b81 movabs r8, 0x815bf5036966978c
140a022f2: 4889da mov rdx, rbx
140a022f5: 41b119 mov r9b, 0x19
140a022f8: e8d333f0ff call 0x1409056d0
140a022fd: 488d9df0000000 lea rbx, [rbp + 0xf0]
140a02304: 488d95d0020000 lea rdx, [rbp + 0x2d0]
140a0230b: 41b890010000 mov r8d, 0x190
140a02311: 4889d9 mov rcx, rbx
140a02314: e8e718f201 call 0x142923c00
140a02319: 4c8da5d0020000 lea r12, [rbp + 0x2d0]
140a02320: 4c89e1 mov rcx, r12
140a02323: 4889da mov rdx, rbx
140a02326: 4c8b8500060000 mov r8, qword ptr [rbp + 0x600]
140a0232d: e8fe9ebf00 call 0x1415fc230
140a02332: 488d453c lea rax, [rbp + 0x3c]
140a02336: 488985f0000000 mov qword ptr [rbp + 0xf0], rax
140a0233d: c7856004000002000000 mov dword ptr [rbp + 0x460], 2
140a02347: 4c896c2420 mov qword ptr [rsp + 0x20], r13
140a0234c: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a02353: 49b81cf30ccb4fc1ccb5 movabs r8, 0xb5ccc14fcb0cf31c
140a0235d: 4c89e2 mov rdx, r12
140a02360: 41b132 mov r9b, 0x32
140a02363: e898b9eeff call 0x1408edd00
140a02368: 488d9d60040000 lea rbx, [rbp + 0x460]
140a0236f: 41b890010000 mov r8d, 0x190
140a02375: 4889d9 mov rcx, rbx
140a02378: 4c89e2 mov rdx, r12
140a0237b: e88018f201 call 0x142923c00
140a02380: 488d4518 lea rax, [rbp + 0x18]
140a02384: 488985f0000000 mov qword ptr [rbp + 0xf0], rax
140a0238b: c785d002000002000000 mov dword ptr [rbp + 0x2d0], 2
140a02395: 4c89642420 mov qword ptr [rsp + 0x20], r12
140a0239a: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a023a1: 49b8e695eba72c5c1450 movabs r8, 0x50145c2ca7eb95e6
140a023ab: 4889da mov rdx, rbx
140a023ae: 41b132 mov r9b, 0x32
140a023b1: e84ab9eeff call 0x1408edd00
140a023b6: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
140a023bd: 488d9560040000 lea rdx, [rbp + 0x460]
140a023c4: 41b890010000 mov r8d, 0x190
140a023ca: e83118f201 call 0x142923c00
140a023cf: 4c89f7 mov rdi, r14
140a023d2: 48c1ef20 shr rdi, 0x20
140a023d6: 0f108570030000 movups xmm0, xmmword ptr [rbp + 0x370]
140a023dd: 0f298530010000 movaps xmmword ptr [rbp + 0x130], xmm0
140a023e4: 0f108560030000 movups xmm0, xmmword ptr [rbp + 0x360]
140a023eb: 0f298520010000 movaps xmmword ptr [rbp + 0x120], xmm0
140a023f2: 0f108550030000 movups xmm0, xmmword ptr [rbp + 0x350]
140a023f9: 0f298510010000 movaps xmmword ptr [rbp + 0x110], xmm0
140a02400: 0f108540030000 movups xmm0, xmmword ptr [rbp + 0x340]
140a02407: 0f298500010000 movaps xmmword ptr [rbp + 0x100], xmm0
140a0240e: 0f108530030000 movups xmm0, xmmword ptr [rbp + 0x330]
140a02415: 0f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm0
140a0241c: 488b8508040000 mov rax, qword ptr [rbp + 0x408]
140a02423: 488b8d50040000 mov rcx, qword ptr [rbp + 0x450]
140a0242a: 488b95e0020000 mov rdx, qword ptr [rbp + 0x2e0]
140a02431: 48899570040000 mov qword ptr [rbp + 0x470], rdx
140a02438: 0f1085d0020000 movups xmm0, xmmword ptr [rbp + 0x2d0]
140a0243f: 0f298560040000 movaps xmmword ptr [rbp + 0x460], xmm0
140a02446: 0f1085e8020000 movups xmm0, xmmword ptr [rbp + 0x2e8]
140a0244d: 0f298590040000 movaps xmmword ptr [rbp + 0x490], xmm0
140a02454: 488b95f8020000 mov rdx, qword ptr [rbp + 0x2f8]
140a0245b: 488995a0040000 mov qword ptr [rbp + 0x4a0], rdx
140a02462: 0f108500030000 movups xmm0, xmmword ptr [rbp + 0x300]
140a02469: 0f118578040000 movups xmmword ptr [rbp + 0x478], xmm0
140a02470: 488b9510030000 mov rdx, qword ptr [rbp + 0x310]
140a02477: 48899588040000 mov qword ptr [rbp + 0x488], rdx
140a0247e: 0f108580030000 movups xmm0, xmmword ptr [rbp + 0x380]
140a02485: 0f108d90030000 movups xmm1, xmmword ptr [rbp + 0x390]
140a0248c: 0f1095a0030000 movups xmm2, xmmword ptr [rbp + 0x3a0]
140a02493: 0f109db0030000 movups xmm3, xmmword ptr [rbp + 0x3b0]
140a0249a: 0f1185d8040000 movups xmmword ptr [rbp + 0x4d8], xmm0
140a024a1: 0f118de8040000 movups xmmword ptr [rbp + 0x4e8], xmm1
140a024a8: 0f1195f8040000 movups xmmword ptr [rbp + 0x4f8], xmm2
140a024af: 0f119d08050000 movups xmmword ptr [rbp + 0x508], xmm3
140a024b6: 0f1085c0030000 movups xmm0, xmmword ptr [rbp + 0x3c0]
140a024bd: 0f118518050000 movups xmmword ptr [rbp + 0x518], xmm0
140a024c4: 0f1085d0030000 movups xmm0, xmmword ptr [rbp + 0x3d0]
140a024cb: 0f118528050000 movups xmmword ptr [rbp + 0x528], xmm0
140a024d2: 488b9528030000 mov rdx, qword ptr [rbp + 0x328]
140a024d9: 488995b8040000 mov qword ptr [rbp + 0x4b8], rdx
140a024e0: 0f108518030000 movups xmm0, xmmword ptr [rbp + 0x318]
140a024e7: 0f1185a8040000 movups xmmword ptr [rbp + 0x4a8], xmm0
140a024ee: 48898d40050000 mov qword ptr [rbp + 0x540], rcx
140a024f5: 48898538050000 mov qword ptr [rbp + 0x538], rax
140a024fc: c785c004000001000000 mov dword ptr [rbp + 0x4c0], 1
140a02506: 4489b5c4040000 mov dword ptr [rbp + 0x4c4], r14d
140a0250d: 89bdc8040000 mov dword ptr [rbp + 0x4c8], edi
140a02513: c785cc04000001000000 mov dword ptr [rbp + 0x4cc], 1
140a0251d: 4c8dadf0000000 lea r13, [rbp + 0xf0]
140a02524: 488d9560040000 lea rdx, [rbp + 0x460]
140a0252b: 4c89e9 mov rcx, r13
140a0252e: e89daffdff call 0x1409dd4d0
140a02533: 4889bd90020000 mov qword ptr [rbp + 0x290], rdi
140a0253a: 80bd0604000008 cmp byte ptr [rbp + 0x406], 8
140a02541: 740c je 0x140a0254f
140a02543: 488d8df0030000 lea rcx, [rbp + 0x3f0]
140a0254a: e861ff1d00 call 0x140be24b0
140a0254f: 488d9dd0020000 lea rbx, [rbp + 0x2d0]
140a02556: 4889d9 mov rcx, rbx
140a02559: 4c89f2 mov rdx, r14
140a0255c: 41b861000000 mov r8d, 0x61
140a02562: e8392d1d00 call 0x140bd52a0
140a02567: 4c8da560040000 lea r12, [rbp + 0x460]
140a0256e: 4c89e1 mov rcx, r12
140a02571: 4889da mov rdx, rbx
140a02574: e8f72d1d00 call 0x140bd5370
140a02579: 4c89e9 mov rcx, r13
140a0257c: 4c89e2 mov rdx, r12
140a0257f: e80c2e1d00 call 0x140bd5390
140a02584: 4c896c2420 mov qword ptr [rsp + 0x20], r13
140a02589: 4c8d05e7350c02 lea r8, [rip + 0x20c35e7]
140a02590: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
140a02597: 41b90f000000 mov r9d, 0xf
140a0259d: 4c89fa mov rdx, r15
140a025a0: e8ebaef6ff call 0x14096d490
140a025a5: 83bdd002000001 cmp dword ptr [rbp + 0x2d0], 1
140a025ac: 0f84140b0000 je 0x140a030c6
140a025b2: 488b8514030000 mov rax, qword ptr [rbp + 0x314]
140a025b9: 488985a0040000 mov qword ptr [rbp + 0x4a0], rax
140a025c0: 0f1085d4020000 movups xmm0, xmmword ptr [rbp + 0x2d4]
140a025c7: 0f108de4020000 movups xmm1, xmmword ptr [rbp + 0x2e4]
140a025ce: 0f1095f4020000 movups xmm2, xmmword ptr [rbp + 0x2f4]
140a025d5: 0f109d04030000 movups xmm3, xmmword ptr [rbp + 0x304]
140a025dc: 0f299d90040000 movaps xmmword ptr [rbp + 0x490], xmm3
140a025e3: 0f299580040000 movaps xmmword ptr [rbp + 0x480], xmm2
140a025ea: 0f298d70040000 movaps xmmword ptr [rbp + 0x470], xmm1
140a025f1: 0f298560040000 movaps xmmword ptr [rbp + 0x460], xmm0
140a025f8: 0fb6bd1c030000 movzx edi, byte ptr [rbp + 0x31c]
140a025ff: 8b8580040000 mov eax, dword ptr [rbp + 0x480]
140a02605: 8985e0000000 mov dword ptr [rbp + 0xe0], eax
140a0260b: 0f298dd0000000 movaps xmmword ptr [rbp + 0xd0], xmm1
140a02612: 0f2985c0000000 movaps xmmword ptr [rbp + 0xc0], xmm0
140a02619: 0f108594040000 movups xmm0, xmmword ptr [rbp + 0x494]
140a02620: 0f294500 movaps xmmword ptr [rbp], xmm0
140a02624: 0f108584040000 movups xmm0, xmmword ptr [rbp + 0x484]
140a0262b: 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
140a0262f: 8b85a4040000 mov eax, dword ptr [rbp + 0x4a4]
140a02635: 894510 mov dword ptr [rbp + 0x10], eax
140a02638: 4889d9 mov rcx, rbx
140a0263b: 4c89f2 mov rdx, r14
140a0263e: 41b861000000 mov r8d, 0x61
140a02644: e8572c1d00 call 0x140bd52a0
140a02649: 4c89e1 mov rcx, r12
140a0264c: 4889da mov rdx, rbx
140a0264f: e81c2d1d00 call 0x140bd5370
140a02654: 4c89e9 mov rcx, r13
140a02657: 4c89e2 mov rdx, r12
140a0265a: e8312d1d00 call 0x140bd5390
140a0265f: 4c896c2420 mov qword ptr [rsp + 0x20], r13
140a02664: 4c8d057d350c02 lea r8, [rip + 0x20c357d]
140a0266b: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
140a02672: 41b90e000000 mov r9d, 0xe
140a02678: 4c89fa mov rdx, r15
140a0267b: e810aef6ff call 0x14096d490
140a02680: 83bdd002000001 cmp dword ptr [rbp + 0x2d0], 1
140a02687: 0f84740a0000 je 0x140a03101
140a0268d: 4c89f2 mov rdx, r14
140a02690: 488b8514030000 mov rax, qword ptr [rbp + 0x314]
140a02697: 488985a0040000 mov qword ptr [rbp + 0x4a0], rax
140a0269e: 0f1085d4020000 movups xmm0, xmmword ptr [rbp + 0x2d4]
140a026a5: 0f108de4020000 movups xmm1, xmmword ptr [rbp + 0x2e4]
140a026ac: 0f1095f4020000 movups xmm2, xmmword ptr [rbp + 0x2f4]
140a026b3: 0f109d04030000 movups xmm3, xmmword ptr [rbp + 0x304]
140a026ba: 0f299d90040000 movaps xmmword ptr [rbp + 0x490], xmm3
140a026c1: 0f299580040000 movaps xmmword ptr [rbp + 0x480], xmm2
140a026c8: 0f298d70040000 movaps xmmword ptr [rbp + 0x470], xmm1
140a026cf: 0f298560040000 movaps xmmword ptr [rbp + 0x460], xmm0
140a026d6: 440fb6b51c030000 movzx r14d, byte ptr [rbp + 0x31c]
140a026de: 8b8580040000 mov eax, dword ptr [rbp + 0x480]
140a026e4: 8985b0000000 mov dword ptr [rbp + 0xb0], eax
140a026ea: 0f298da0000000 movaps xmmword ptr [rbp + 0xa0], xmm1
140a026f1: 0f298590000000 movaps xmmword ptr [rbp + 0x90], xmm0
140a026f8: 0f108594040000 movups xmm0, xmmword ptr [rbp + 0x494]
140a026ff: 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
140a02703: 0f108584040000 movups xmm0, xmmword ptr [rbp + 0x484]
140a0270a: 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
140a0270e: 8b85a4040000 mov eax, dword ptr [rbp + 0x4a4]
140a02714: 8945e0 mov dword ptr [rbp - 0x20], eax
140a02717: 4889d9 mov rcx, rbx
140a0271a: 48899598020000 mov qword ptr [rbp + 0x298], rdx
140a02721: 41b865000000 mov r8d, 0x65
140a02727: e8742b1d00 call 0x140bd52a0
140a0272c: 4c89e1 mov rcx, r12
140a0272f: 4889da mov rdx, rbx
140a02732: e8392c1d00 call 0x140bd5370
140a02737: 4c89e9 mov rcx, r13
140a0273a: 4c89e2 mov rdx, r12
140a0273d: e84e2c1d00 call 0x140bd5390
140a02742: 4c896c2420 mov qword ptr [rsp + 0x20], r13
140a02747: 4c8d05c2340c02 lea r8, [rip + 0x20c34c2]
140a0274e: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
140a02755: 41b910000000 mov r9d, 0x10
140a0275b: 4c89fa mov rdx, r15
140a0275e: e82dadf6ff call 0x14096d490
140a02763: 83bdd002000001 cmp dword ptr [rbp + 0x2d0], 1
140a0276a: 0f84cc090000 je 0x140a0313c
140a02770: 4120fe and r14b, dil
140a02773: 488b8514030000 mov rax, qword ptr [rbp + 0x314]
140a0277a: 488985a0040000 mov qword ptr [rbp + 0x4a0], rax
140a02781: 0f1085d4020000 movups xmm0, xmmword ptr [rbp + 0x2d4]
140a02788: 0f108de4020000 movups xmm1, xmmword ptr [rbp + 0x2e4]
140a0278f: 0f1095f4020000 movups xmm2, xmmword ptr [rbp + 0x2f4]
140a02796: 0f109d04030000 movups xmm3, xmmword ptr [rbp + 0x304]
140a0279d: 0f299d90040000 movaps xmmword ptr [rbp + 0x490], xmm3
140a027a4: 0f299580040000 movaps xmmword ptr [rbp + 0x480], xmm2
140a027ab: 0f298d70040000 movaps xmmword ptr [rbp + 0x470], xmm1
140a027b2: 0f298560040000 movaps xmmword ptr [rbp + 0x460], xmm0
140a027b9: 4422b51c030000 and r14b, byte ptr [rbp + 0x31c]
140a027c0: 8b8580040000 mov eax, dword ptr [rbp + 0x480]
140a027c6: 898580000000 mov dword ptr [rbp + 0x80], eax
140a027cc: 0f294d70 movaps xmmword ptr [rbp + 0x70], xmm1
140a027d0: 0f294560 movaps xmmword ptr [rbp + 0x60], xmm0
140a027d4: 0f108594040000 movups xmm0, xmmword ptr [rbp + 0x494]
140a027db: 0f2985b0020000 movaps xmmword ptr [rbp + 0x2b0], xmm0
140a027e2: 0f108584040000 movups xmm0, xmmword ptr [rbp + 0x484]
140a027e9: 0f2985a0020000 movaps xmmword ptr [rbp + 0x2a0], xmm0
140a027f0: 8b85a4040000 mov eax, dword ptr [rbp + 0x4a4]
140a027f6: 8985c0020000 mov dword ptr [rbp + 0x2c0], eax
140a027fc: 4c897c2430 mov qword ptr [rsp + 0x30], r15
140a02801: 488b8500060000 mov rax, qword ptr [rbp + 0x600]
140a02808: 4889442428 mov qword ptr [rsp + 0x28], rax
140a0280d: 48c74424201e000000 mov qword ptr [rsp + 0x20], 0x1e
140a02816: 488d151b340c02 lea rdx, [rip + 0x20c341b]
140a0281d: 4c8d0d26340c02 lea r9, [rip + 0x20c3426]
140a02824: 488d8d60040000 lea rcx, [rbp + 0x460]
140a0282b: 41b812000000 mov r8d, 0x12
140a02831: e86a0ac000 call 0x1416032a0
140a02836: 80bd9605000008 cmp byte ptr [rbp + 0x596], 8
140a0283d: 7425 je 0x140a02864
140a0283f: 488d8d70050000 lea rcx, [rbp + 0x570]
140a02846: bae8000000 mov edx, 0xe8
140a0284b: 48039598050000 add rdx, qword ptr [rbp + 0x598]
140a02852: 49b84ca5591f44988a41 movabs r8, 0x418a98441f59a54c
140a0285c: 41b101 mov r9b, 1
140a0285f: e8ec8ff6ff call 0x14096b850
140a02864: 4c8dbd60040000 lea r15, [rbp + 0x460]
140a0286b: 41b890010000 mov r8d, 0x190
140a02871: 4889d9 mov rcx, rbx
140a02874: 4c89fa mov rdx, r15
140a02877: e88413f201 call 0x142923c00
140a0287c: c6850806000011 mov byte ptr [rbp + 0x608], 0x11
140a02883: c785f000000002000000 mov dword ptr [rbp + 0xf0], 2
140a0288d: 4c896c2420 mov qword ptr [rsp + 0x20], r13
140a02892: 488d8d08060000 lea rcx, [rbp + 0x608]
140a02899: 49b8a41a84c94d1f6511 movabs r8, 0x11651f4dc9841aa4
140a028a3: 4889da mov rdx, rbx
140a028a6: 41b119 mov r9b, 0x19
140a028a9: e8b229f0ff call 0x140905260
140a028ae: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a028b5: 488d95d0020000 lea rdx, [rbp + 0x2d0]
140a028bc: 41b890010000 mov r8d, 0x190
140a028c2: e83913f201 call 0x142923c00
140a028c7: 80bd2602000008 cmp byte ptr [rbp + 0x226], 8
140a028ce: 488bbd90060000 mov rdi, qword ptr [rbp + 0x690]
140a028d5: 7431 je 0x140a02908
140a028d7: 4531c9 xor r9d, r9d
140a028da: 41f6c601 test r14b, 1
140a028de: 440f458dfc050000 cmovne r9d, dword ptr [rbp + 0x5fc]
140a028e6: 488d8d00020000 lea rcx, [rbp + 0x200]
140a028ed: bae8000000 mov edx, 0xe8
140a028f2: 48039528020000 add rdx, qword ptr [rbp + 0x228]
140a028f9: 49b802d9ee60b709a1a9 movabs r8, 0xa9a109b760eed902
140a02903: e8688cf6ff call 0x14096b570
140a02908: 488d95f0000000 lea rdx, [rbp + 0xf0]
140a0290f: 41b890010000 mov r8d, 0x190
140a02915: 4889d9 mov rcx, rbx
140a02918: e8e312f201 call 0x142923c00
140a0291d: 488b85c8020000 mov rax, qword ptr [rbp + 0x2c8]
140a02924: 488985f0000000 mov qword ptr [rbp + 0xf0], rax
140a0292b: c7856004000002000000 mov dword ptr [rbp + 0x460], 2
140a02935: 4c897c2420 mov qword ptr [rsp + 0x20], r15
140a0293a: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a02941: 49b8360bea2d0d4aaa41 movabs r8, 0x41aa4a0d2dea0b36
140a0294b: 4889da mov rdx, rbx
140a0294e: 41b119 mov r9b, 0x19
140a02951: e8eab8eeff call 0x1408ee240
140a02956: 488d9d60040000 lea rbx, [rbp + 0x460]
140a0295d: 4c8db5d0020000 lea r14, [rbp + 0x2d0]
140a02964: 41b890010000 mov r8d, 0x190
140a0296a: 4889d9 mov rcx, rbx
140a0296d: 4c89f2 mov rdx, r14
140a02970: e88b12f201 call 0x142923c00
140a02975: 488b8588020000 mov rax, qword ptr [rbp + 0x288]
140a0297c: 488985f0000000 mov qword ptr [rbp + 0xf0], rax
140a02983: c785d002000002000000 mov dword ptr [rbp + 0x2d0], 2
140a0298d: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a02992: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a02999: 49b8fcfb49a09d576148 movabs r8, 0x4861579da049fbfc
140a029a3: 4889da mov rdx, rbx
140a029a6: 41b119 mov r9b, 0x19
140a029a9: e892b8eeff call 0x1408ee240
140a029ae: 488d9dd0020000 lea rbx, [rbp + 0x2d0]
140a029b5: 4c8db560040000 lea r14, [rbp + 0x460]
140a029bc: 41b890010000 mov r8d, 0x190
140a029c2: 4889d9 mov rcx, rbx
140a029c5: 4c89f2 mov rdx, r14
140a029c8: e83312f201 call 0x142923c00
140a029cd: 4889bdf0000000 mov qword ptr [rbp + 0xf0], rdi
140a029d4: c7856004000002000000 mov dword ptr [rbp + 0x460], 2
140a029de: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a029e3: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a029ea: 49b8d0096c8285511704 movabs r8, 0x4175185826c09d0
140a029f4: 4889da mov rdx, rbx
140a029f7: 41b119 mov r9b, 0x19
140a029fa: e841b8eeff call 0x1408ee240
140a029ff: 488d9d60040000 lea rbx, [rbp + 0x460]
140a02a06: 4c8db5d0020000 lea r14, [rbp + 0x2d0]
140a02a0d: 41b890010000 mov r8d, 0x190
140a02a13: 4889d9 mov rcx, rbx
140a02a16: 4c89f2 mov rdx, r14
140a02a19: e8e211f201 call 0x142923c00
140a02a1e: 488d45f0 lea rax, [rbp - 0x10]
140a02a22: 488985f0000000 mov qword ptr [rbp + 0xf0], rax
140a02a29: c785d002000002000000 mov dword ptr [rbp + 0x2d0], 2
140a02a33: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a02a38: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a02a3f: 49b8aa9d74979fa58dfe movabs r8, 0xfe8da59f97749daa
140a02a49: 4889da mov rdx, rbx
140a02a4c: 41b119 mov r9b, 0x19
140a02a4f: e83cc6f6ff call 0x14096f090
140a02a54: 488dbda0020000 lea rdi, [rbp + 0x2a0]
140a02a5b: 488d9dd0020000 lea rbx, [rbp + 0x2d0]
140a02a62: 4c8db560040000 lea r14, [rbp + 0x460]
140a02a69: 41b890010000 mov r8d, 0x190
140a02a6f: 4889d9 mov rcx, rbx
140a02a72: 4c89f2 mov rdx, r14
140a02a75: e88611f201 call 0x142923c00
140a02a7a: 488d45c0 lea rax, [rbp - 0x40]
140a02a7e: 488985f0000000 mov qword ptr [rbp + 0xf0], rax
140a02a85: c7856004000002000000 mov dword ptr [rbp + 0x460], 2
140a02a8f: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a02a94: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a02a9b: 49b8f19da22b99dd360d movabs r8, 0xd36dd992ba29df1
140a02aa5: 4889da mov rdx, rbx
140a02aa8: 41b119 mov r9b, 0x19
140a02aab: e8e0c5f6ff call 0x14096f090
140a02ab0: 488d9d60040000 lea rbx, [rbp + 0x460]
140a02ab7: 4c8db5d0020000 lea r14, [rbp + 0x2d0]
140a02abe: 41b890010000 mov r8d, 0x190
140a02ac4: 4889d9 mov rcx, rbx
140a02ac7: 4c89f2 mov rdx, r14
140a02aca: e83111f201 call 0x142923c00
140a02acf: 4889bdf0000000 mov qword ptr [rbp + 0xf0], rdi
140a02ad6: c785d002000002000000 mov dword ptr [rbp + 0x2d0], 2
140a02ae0: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a02ae5: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a02aec: 49b8f04df136d28b1b4b movabs r8, 0x4b1b8bd236f14df0
140a02af6: 4889da mov rdx, rbx
140a02af9: 41b119 mov r9b, 0x19
140a02afc: e88fc5f6ff call 0x14096f090
140a02b01: 488d9dd0020000 lea rbx, [rbp + 0x2d0]
140a02b08: 4c8db560040000 lea r14, [rbp + 0x460]
140a02b0f: 41b890010000 mov r8d, 0x190
140a02b15: 4889d9 mov rcx, rbx
140a02b18: 4c89f2 mov rdx, r14
140a02b1b: e8e010f201 call 0x142923c00
140a02b20: 488b85a0060000 mov rax, qword ptr [rbp + 0x6a0]
140a02b27: 488985f0000000 mov qword ptr [rbp + 0xf0], rax
140a02b2e: c7856004000002000000 mov dword ptr [rbp + 0x460], 2
140a02b38: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a02b3d: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a02b44: 49b8e36b5a83cd4ca7b8 movabs r8, 0xb8a74ccd835a6be3
140a02b4e: 4889da mov rdx, rbx
140a02b51: 41b119 mov r9b, 0x19
140a02b54: e8e7b6eeff call 0x1408ee240
140a02b59: 488d9d60040000 lea rbx, [rbp + 0x460]
140a02b60: 4c8db5d0020000 lea r14, [rbp + 0x2d0]
140a02b67: 41b890010000 mov r8d, 0x190
140a02b6d: 4889d9 mov rcx, rbx
140a02b70: 4c89f2 mov rdx, r14
140a02b73: e88810f201 call 0x142923c00
140a02b78: 488b85a8060000 mov rax, qword ptr [rbp + 0x6a8]
140a02b7f: 488985f0000000 mov qword ptr [rbp + 0xf0], rax
140a02b86: c785d002000002000000 mov dword ptr [rbp + 0x2d0], 2
140a02b90: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a02b95: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a02b9c: 49b8f99a3964bbe203e0 movabs r8, 0xe003e2bb64399af9
140a02ba6: 4889da mov rdx, rbx
140a02ba9: 41b119 mov r9b, 0x19
140a02bac: e88fb6eeff call 0x1408ee240
140a02bb1: 488d9dd0020000 lea rbx, [rbp + 0x2d0]
140a02bb8: 4c8db560040000 lea r14, [rbp + 0x460]
140a02bbf: 41b890010000 mov r8d, 0x190
140a02bc5: 4889d9 mov rcx, rbx
140a02bc8: 4c89f2 mov rdx, r14
140a02bcb: e83010f201 call 0x142923c00
140a02bd0: 488d453c lea rax, [rbp + 0x3c]
140a02bd4: 488985f0000000 mov qword ptr [rbp + 0xf0], rax
140a02bdb: c7856004000002000000 mov dword ptr [rbp + 0x460], 2
140a02be5: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a02bea: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a02bf1: 49b81cf30ccb4fc1ccb5 movabs r8, 0xb5ccc14fcb0cf31c
140a02bfb: 4889da mov rdx, rbx
140a02bfe: 41b119 mov r9b, 0x19
140a02c01: e83ab6eeff call 0x1408ee240
140a02c06: 488d9d60040000 lea rbx, [rbp + 0x460]
140a02c0d: 4c8db5d0020000 lea r14, [rbp + 0x2d0]
140a02c14: 41b890010000 mov r8d, 0x190
140a02c1a: 4889d9 mov rcx, rbx
140a02c1d: 4c89f2 mov rdx, r14
140a02c20: e8db0ff201 call 0x142923c00
140a02c25: 488d4518 lea rax, [rbp + 0x18]
140a02c29: 48898508060000 mov qword ptr [rbp + 0x608], rax
140a02c30: c785d002000002000000 mov dword ptr [rbp + 0x2d0], 2
140a02c3a: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a02c3f: 488d8d08060000 lea rcx, [rbp + 0x608]
140a02c46: 49b8e695eba72c5c1450 movabs r8, 0x50145c2ca7eb95e6
140a02c50: 4889da mov rdx, rbx
140a02c53: 41b119 mov r9b, 0x19
140a02c56: e8e5b5eeff call 0x1408ee240
140a02c5b: 488d9df0000000 lea rbx, [rbp + 0xf0]
140a02c62: 488d9560040000 lea rdx, [rbp + 0x460]
140a02c69: 41b890010000 mov r8d, 0x190
140a02c6f: 4889d9 mov rcx, rbx
140a02c72: e8890ff201 call 0x142923c00
140a02c77: c6856004000009 mov byte ptr [rbp + 0x460], 9
140a02c7e: c785d002000002000000 mov dword ptr [rbp + 0x2d0], 2
140a02c88: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a02c8d: 4c8db560040000 lea r14, [rbp + 0x460]
140a02c94: 49b8f2c570045e8bb061 movabs r8, 0x61b08b5e0470c5f2
140a02c9e: 4c89f1 mov rcx, r14
140a02ca1: 4889da mov rdx, rbx
140a02ca4: 41b119 mov r9b, 0x19
140a02ca7: e8b425f0ff call 0x140905260
140a02cac: 488d9dd0020000 lea rbx, [rbp + 0x2d0]
140a02cb3: 488d95f0000000 lea rdx, [rbp + 0xf0]
140a02cba: 41b890010000 mov r8d, 0x190
140a02cc0: 4889d9 mov rcx, rbx
140a02cc3: e8380ff201 call 0x142923c00
140a02cc8: c685080600000a mov byte ptr [rbp + 0x608], 0xa
140a02ccf: c7856004000002000000 mov dword ptr [rbp + 0x460], 2
140a02cd9: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a02cde: 488d8d08060000 lea rcx, [rbp + 0x608]
140a02ce5: 49b857f6e8da4c1e7d29 movabs r8, 0x297d1e4cdae8f657
140a02cef: 4889da mov rdx, rbx
140a02cf2: 41b119 mov r9b, 0x19
140a02cf5: e86625f0ff call 0x140905260
140a02cfa: 488d9d60040000 lea rbx, [rbp + 0x460]
140a02d01: 4c8db5d0020000 lea r14, [rbp + 0x2d0]
140a02d08: 41b890010000 mov r8d, 0x190
140a02d0e: 4889d9 mov rcx, rbx
140a02d11: 4c89f2 mov rdx, r14
140a02d14: e8e70ef201 call 0x142923c00
140a02d19: c685f000000008 mov byte ptr [rbp + 0xf0], 8
140a02d20: c785d002000002000000 mov dword ptr [rbp + 0x2d0], 2
140a02d2a: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a02d2f: 4c8dbdf0000000 lea r15, [rbp + 0xf0]
140a02d36: 49b83fca8dac13011677 movabs r8, 0x77160113ac8dca3f
140a02d40: 4c89f9 mov rcx, r15
140a02d43: 4889da mov rdx, rbx
140a02d46: 41b119 mov r9b, 0x19
140a02d49: e81225f0ff call 0x140905260
140a02d4e: 488d9dd0020000 lea rbx, [rbp + 0x2d0]
140a02d55: 4c8db560040000 lea r14, [rbp + 0x460]
140a02d5c: 41b890010000 mov r8d, 0x190
140a02d62: 4889d9 mov rcx, rbx
140a02d65: 4c89f2 mov rdx, r14
140a02d68: e8930ef201 call 0x142923c00
140a02d6d: 48b86518a091ab399f57 movabs rax, 0x579f39ab91a01865
140a02d77: 48898508060000 mov qword ptr [rbp + 0x608], rax
140a02d7e: c785f000000002000000 mov dword ptr [rbp + 0xf0], 2
140a02d88: 4c897c2420 mov qword ptr [rsp + 0x20], r15
140a02d8d: 488d8d08060000 lea rcx, [rbp + 0x608]
140a02d94: 49b88c97666903f55b81 movabs r8, 0x815bf5036966978c
140a02d9e: 4889da mov rdx, rbx
140a02da1: 41b119 mov r9b, 0x19
140a02da4: e82729f0ff call 0x1409056d0
140a02da9: 488d9df0000000 lea rbx, [rbp + 0xf0]
140a02db0: 488d95d0020000 lea rdx, [rbp + 0x2d0]
140a02db7: 41b890010000 mov r8d, 0x190
140a02dbd: 4889d9 mov rcx, rbx
140a02dc0: e83b0ef201 call 0x142923c00
140a02dc5: 4c8dbdd0020000 lea r15, [rbp + 0x2d0]
140a02dcc: 4c89f9 mov rcx, r15
140a02dcf: 4889da mov rdx, rbx
140a02dd2: 4c8b8500060000 mov r8, qword ptr [rbp + 0x600]
140a02dd9: e85294bf00 call 0x1415fc230
140a02dde: 488d85c0000000 lea rax, [rbp + 0xc0]
140a02de5: 488985f0000000 mov qword ptr [rbp + 0xf0], rax
140a02dec: c7856004000002000000 mov dword ptr [rbp + 0x460], 2
140a02df6: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a02dfb: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a02e02: 49b8f51fb8de1f3a3ec4 movabs r8, 0xc43e3a1fdeb81ff5
140a02e0c: 4c89fa mov rdx, r15
140a02e0f: 41b132 mov r9b, 0x32
140a02e12: e8e9aeeeff call 0x1408edd00
140a02e17: 488d9d60040000 lea rbx, [rbp + 0x460]
140a02e1e: 4c8db5d0020000 lea r14, [rbp + 0x2d0]
140a02e25: 41b890010000 mov r8d, 0x190
140a02e2b: 4889d9 mov rcx, rbx
140a02e2e: 4c89f2 mov rdx, r14
140a02e31: e8ca0df201 call 0x142923c00
140a02e36: 488d8590000000 lea rax, [rbp + 0x90]
140a02e3d: 488985f0000000 mov qword ptr [rbp + 0xf0], rax
140a02e44: c785d002000002000000 mov dword ptr [rbp + 0x2d0], 2
140a02e4e: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a02e53: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a02e5a: 49b84af8ff4289728f7f movabs r8, 0x7f8f728942fff84a
140a02e64: 4889da mov rdx, rbx
140a02e67: 41b132 mov r9b, 0x32
140a02e6a: e891aeeeff call 0x1408edd00
140a02e6f: 488d9dd0020000 lea rbx, [rbp + 0x2d0]
140a02e76: 4c8db560040000 lea r14, [rbp + 0x460]
140a02e7d: 41b890010000 mov r8d, 0x190
140a02e83: 4889d9 mov rcx, rbx
140a02e86: 4c89f2 mov rdx, r14
140a02e89: e8720df201 call 0x142923c00
140a02e8e: 488d4560 lea rax, [rbp + 0x60]
140a02e92: 488985f0000000 mov qword ptr [rbp + 0xf0], rax
140a02e99: c7856004000002000000 mov dword ptr [rbp + 0x460], 2
140a02ea3: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a02ea8: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a02eaf: 49b8536f9d7308bb3ef0 movabs r8, 0xf03ebb08739d6f53
140a02eb9: 4889da mov rdx, rbx
140a02ebc: 41b132 mov r9b, 0x32
140a02ebf: e83caeeeff call 0x1408edd00
140a02ec4: 488d8d60040000 lea rcx, [rbp + 0x460]
140a02ecb: 488d95d0020000 lea rdx, [rbp + 0x2d0]
140a02ed2: 41b890010000 mov r8d, 0x190
140a02ed8: e8230df201 call 0x142923c00
140a02edd: 0f108500050000 movups xmm0, xmmword ptr [rbp + 0x500]
140a02ee4: 0f298530010000 movaps xmmword ptr [rbp + 0x130], xmm0
140a02eeb: 0f1085f0040000 movups xmm0, xmmword ptr [rbp + 0x4f0]
140a02ef2: 0f298520010000 movaps xmmword ptr [rbp + 0x120], xmm0
140a02ef9: 0f1085e0040000 movups xmm0, xmmword ptr [rbp + 0x4e0]
140a02f00: 0f298510010000 movaps xmmword ptr [rbp + 0x110], xmm0
140a02f07: 0f1085d0040000 movups xmm0, xmmword ptr [rbp + 0x4d0]
140a02f0e: 0f298500010000 movaps xmmword ptr [rbp + 0x100], xmm0
140a02f15: 0f1085c0040000 movups xmm0, xmmword ptr [rbp + 0x4c0]
140a02f1c: 0f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm0
140a02f23: 488b8598050000 mov rax, qword ptr [rbp + 0x598]
140a02f2a: 488b8de0050000 mov rcx, qword ptr [rbp + 0x5e0]
140a02f31: 488b9570040000 mov rdx, qword ptr [rbp + 0x470]
140a02f38: 488995e0020000 mov qword ptr [rbp + 0x2e0], rdx
140a02f3f: 0f108560040000 movups xmm0, xmmword ptr [rbp + 0x460]
140a02f46: 0f2985d0020000 movaps xmmword ptr [rbp + 0x2d0], xmm0
140a02f4d: 0f108578040000 movups xmm0, xmmword ptr [rbp + 0x478]
140a02f54: 0f298500030000 movaps xmmword ptr [rbp + 0x300], xmm0
140a02f5b: 488b9588040000 mov rdx, qword ptr [rbp + 0x488]
140a02f62: 48899510030000 mov qword ptr [rbp + 0x310], rdx
140a02f69: 0f108590040000 movups xmm0, xmmword ptr [rbp + 0x490]
140a02f70: 0f1185e8020000 movups xmmword ptr [rbp + 0x2e8], xmm0
140a02f77: 488b95a0040000 mov rdx, qword ptr [rbp + 0x4a0]
140a02f7e: 488995f8020000 mov qword ptr [rbp + 0x2f8], rdx
140a02f85: 0f108510050000 movups xmm0, xmmword ptr [rbp + 0x510]
140a02f8c: 0f108d20050000 movups xmm1, xmmword ptr [rbp + 0x520]
140a02f93: 0f109530050000 movups xmm2, xmmword ptr [rbp + 0x530]
140a02f9a: 0f109d40050000 movups xmm3, xmmword ptr [rbp + 0x540]
140a02fa1: 0f118548030000 movups xmmword ptr [rbp + 0x348], xmm0
140a02fa8: 0f118d58030000 movups xmmword ptr [rbp + 0x358], xmm1
140a02faf: 0f119568030000 movups xmmword ptr [rbp + 0x368], xmm2
140a02fb6: 0f119d78030000 movups xmmword ptr [rbp + 0x378], xmm3
140a02fbd: 0f108550050000 movups xmm0, xmmword ptr [rbp + 0x550]
140a02fc4: 0f118588030000 movups xmmword ptr [rbp + 0x388], xmm0
140a02fcb: 0f108560050000 movups xmm0, xmmword ptr [rbp + 0x560]
140a02fd2: 0f118598030000 movups xmmword ptr [rbp + 0x398], xmm0
140a02fd9: 488b95b8040000 mov rdx, qword ptr [rbp + 0x4b8]
140a02fe0: 48899528030000 mov qword ptr [rbp + 0x328], rdx
140a02fe7: 0f1085a8040000 movups xmm0, xmmword ptr [rbp + 0x4a8]
140a02fee: 0f118518030000 movups xmmword ptr [rbp + 0x318], xmm0
140a02ff5: 48898db0030000 mov qword ptr [rbp + 0x3b0], rcx
140a02ffc: 488985a8030000 mov qword ptr [rbp + 0x3a8], rax
140a03003: c7853003000001000000 mov dword ptr [rbp + 0x330], 1
140a0300d: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
140a03014: 898534030000 mov dword ptr [rbp + 0x334], eax
140a0301a: 488b8590020000 mov rax, qword ptr [rbp + 0x290]
140a03021: 898538030000 mov dword ptr [rbp + 0x338], eax
140a03027: c7853c03000001000000 mov dword ptr [rbp + 0x33c], 1
140a03031: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a03038: 488d95d0020000 lea rdx, [rbp + 0x2d0]
140a0303f: e88ca4fdff call 0x1409dd4d0
140a03044: 80bd9605000008 cmp byte ptr [rbp + 0x596], 8
140a0304b: 740c je 0x140a03059
140a0304d: 488d8d80050000 lea rcx, [rbp + 0x580]
140a03054: e857f41d00 call 0x140be24b0
140a03059: 8b85e0000000 mov eax, dword ptr [rbp + 0xe0]
140a0305f: 894620 mov dword ptr [rsi + 0x20], eax
140a03062: 0f2885c0000000 movaps xmm0, xmmword ptr [rbp + 0xc0]
140a03069: 0f288dd0000000 movaps xmm1, xmmword ptr [rbp + 0xd0]
140a03070: 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
140a03074: 0f1106 movups xmmword ptr [rsi], xmm0
140a03077: 0f288590000000 movaps xmm0, xmmword ptr [rbp + 0x90]
140a0307e: 0f288da0000000 movaps xmm1, xmmword ptr [rbp + 0xa0]
140a03085: 0f114e34 movups xmmword ptr [rsi + 0x34], xmm1
140a03089: 8b85b0000000 mov eax, dword ptr [rbp + 0xb0]
140a0308f: 894644 mov dword ptr [rsi + 0x44], eax
140a03092: 0f114624 movups xmmword ptr [rsi + 0x24], xmm0
140a03096: 8b8580000000 mov eax, dword ptr [rbp + 0x80]
140a0309c: 894668 mov dword ptr [rsi + 0x68], eax
140a0309f: 0f284560 movaps xmm0, xmmword ptr [rbp + 0x60]
140a030a3: 0f284d70 movaps xmm1, xmmword ptr [rbp + 0x70]
140a030a7: 0f114e58 movups xmmword ptr [rsi + 0x58], xmm1
140a030ab: 0f114648 movups xmmword ptr [rsi + 0x48], xmm0
140a030af: 4889f0 mov rax, rsi
140a030b2: 4881c498060000 add rsp, 0x698
140a030b9: 5b pop rbx
140a030ba: 5f pop rdi
140a030bb: 5e pop rsi
140a030bc: 415c pop r12
140a030be: 415d pop r13
140a030c0: 415e pop r14
140a030c2: 415f pop r15
140a030c4: 5d pop rbp
140a030c5: c3 ret 
140a030c6: 488b85d8020000 mov rax, qword ptr [rbp + 0x2d8]
140a030cd: 48898560040000 mov qword ptr [rbp + 0x460], rax
140a030d4: 488d05f52a0c02 lea rax, [rip + 0x20c2af5]
140a030db: 4889442420 mov qword ptr [rsp + 0x20], rax
140a030e0: 488d0d89260c02 lea rcx, [rip + 0x20c2689]
140a030e7: 4c8d0d62260c02 lea r9, [rip + 0x20c2662]
140a030ee: 4c8d8560040000 lea r8, [rbp + 0x460]
140a030f5: ba2b000000 mov edx, 0x2b
140a030fa: e86166ed01 call 0x1428d9760
140a030ff: eb74 jmp 0x140a03175
140a03101: 488b85d8020000 mov rax, qword ptr [rbp + 0x2d8]
140a03108: 48898560040000 mov qword ptr [rbp + 0x460], rax
140a0310f: 488d05e22a0c02 lea rax, [rip + 0x20c2ae2]
140a03116: 4889442420 mov qword ptr [rsp + 0x20], rax
140a0311b: 488d0d4e260c02 lea rcx, [rip + 0x20c264e]
140a03122: 4c8d0d27260c02 lea r9, [rip + 0x20c2627]
140a03129: 4c8d8560040000 lea r8, [rbp + 0x460]
140a03130: ba2b000000 mov edx, 0x2b
140a03135: e82666ed01 call 0x1428d9760
140a0313a: eb39 jmp 0x140a03175
140a0313c: 488b85d8020000 mov rax, qword ptr [rbp + 0x2d8]
140a03143: 48898560040000 mov qword ptr [rbp + 0x460], rax
140a0314a: 488d05cf2a0c02 lea rax, [rip + 0x20c2acf]
140a03151: 4889442420 mov qword ptr [rsp + 0x20], rax
140a03156: 488d0d13260c02 lea rcx, [rip + 0x20c2613]
140a0315d: 4c8d0dec250c02 lea r9, [rip + 0x20c25ec]
140a03164: 4c8d8560040000 lea r8, [rbp + 0x460]
140a0316b: ba2b000000 mov edx, 0x2b
140a03170: e8eb65ed01 call 0x1428d9760
140a03175: 0f0b ud2 
140a03177: 660f1f840000000000 nop word ptr [rax + rax]
140a03180: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03185: 55 push rbp
140a03186: 4157 push r15
140a03188: 4156 push r14
140a0318a: 4155 push r13
140a0318c: 4154 push r12
140a0318e: 56 push rsi
140a0318f: 57 push rdi
140a03190: 53 push rbx
140a03191: 4883ec38 sub rsp, 0x38
140a03195: 488daa80000000 lea rbp, [rdx + 0x80]
140a0319c: 80bd0604000008 cmp byte ptr [rbp + 0x406], 8
140a031a3: 740c je 0x140a031b1
140a031a5: 488d8df0030000 lea rcx, [rbp + 0x3f0]
140a031ac: e8fff21d00 call 0x140be24b0
140a031b1: 90 nop 
140a031b2: 4883c438 add rsp, 0x38
140a031b6: 5b pop rbx
140a031b7: 5f pop rdi
140a031b8: 5e pop rsi
140a031b9: 415c pop r12
140a031bb: 415d pop r13
140a031bd: 415e pop r14
140a031bf: 415f pop r15
140a031c1: 5d pop rbp
140a031c2: c3 ret 
140a031c3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a031d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a031d5: 55 push rbp
140a031d6: 4157 push r15
140a031d8: 4156 push r14
140a031da: 4155 push r13
140a031dc: 4154 push r12
140a031de: 56 push rsi
140a031df: 57 push rdi
140a031e0: 53 push rbx
140a031e1: 4883ec38 sub rsp, 0x38
140a031e5: 488daa80000000 lea rbp, [rdx + 0x80]
140a031ec: 488d8d60040000 lea rcx, [rbp + 0x460]
140a031f3: e8389a3400 call 0x140d4cc30
140a031f8: 90 nop 
140a031f9: 4883c438 add rsp, 0x38
140a031fd: 5b pop rbx
140a031fe: 5f pop rdi
140a031ff: 5e pop rsi
140a03200: 415c pop r12
140a03202: 415d pop r13
140a03204: 415e pop r14
140a03206: 415f pop r15
140a03208: 5d pop rbp
140a03209: c3 ret 
140a0320a: 660f1f440000 nop word ptr [rax + rax]
140a03210: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03215: 55 push rbp
140a03216: 4157 push r15
140a03218: 4156 push r14
140a0321a: 4155 push r13
140a0321c: 4154 push r12
140a0321e: 56 push rsi
140a0321f: 57 push rdi
140a03220: 53 push rbx
140a03221: 4883ec38 sub rsp, 0x38
140a03225: 488daa80000000 lea rbp, [rdx + 0x80]
140a0322c: 488d8d60040000 lea rcx, [rbp + 0x460]
140a03233: e8783de9ff call 0x140896fb0
140a03238: 90 nop 
140a03239: 4883c438 add rsp, 0x38
140a0323d: 5b pop rbx
140a0323e: 5f pop rdi
140a0323f: 5e pop rsi
140a03240: 415c pop r12
140a03242: 415d pop r13
140a03244: 415e pop r14
140a03246: 415f pop r15
140a03248: 5d pop rbp
140a03249: c3 ret 
140a0324a: 660f1f440000 nop word ptr [rax + rax]
140a03250: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03255: 55 push rbp
140a03256: 4157 push r15
140a03258: 4156 push r14
140a0325a: 4155 push r13
140a0325c: 4154 push r12
140a0325e: 56 push rsi
140a0325f: 57 push rdi
140a03260: 53 push rbx
140a03261: 4883ec38 sub rsp, 0x38
140a03265: 488daa80000000 lea rbp, [rdx + 0x80]
140a0326c: 80bd9605000008 cmp byte ptr [rbp + 0x596], 8
140a03273: 740c je 0x140a03281
140a03275: 488d8d80050000 lea rcx, [rbp + 0x580]
140a0327c: e82ff21d00 call 0x140be24b0
140a03281: 90 nop 
140a03282: 4883c438 add rsp, 0x38
140a03286: 5b pop rbx
140a03287: 5f pop rdi
140a03288: 5e pop rsi
140a03289: 415c pop r12
140a0328b: 415d pop r13
140a0328d: 415e pop r14
140a0328f: 415f pop r15
140a03291: 5d pop rbp
140a03292: c3 ret 
140a03293: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a032a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a032a5: 55 push rbp
140a032a6: 4157 push r15
140a032a8: 4156 push r14
140a032aa: 4155 push r13
140a032ac: 4154 push r12
140a032ae: 56 push rsi
140a032af: 57 push rdi
140a032b0: 53 push rbx
140a032b1: 4883ec38 sub rsp, 0x38
140a032b5: 488daa80000000 lea rbp, [rdx + 0x80]
140a032bc: 488d8d60040000 lea rcx, [rbp + 0x460]
140a032c3: e8e83ce9ff call 0x140896fb0
140a032c8: 90 nop 
140a032c9: 4883c438 add rsp, 0x38
140a032cd: 5b pop rbx
140a032ce: 5f pop rdi
140a032cf: 5e pop rsi
140a032d0: 415c pop r12
140a032d2: 415d pop r13
140a032d4: 415e pop r14
140a032d6: 415f pop r15
140a032d8: 5d pop rbp
140a032d9: c3 ret 
140a032da: 660f1f440000 nop word ptr [rax + rax]
140a032e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a032e5: 55 push rbp
140a032e6: 4157 push r15
140a032e8: 4156 push r14
140a032ea: 4155 push r13
140a032ec: 4154 push r12
140a032ee: 56 push rsi
140a032ef: 57 push rdi
140a032f0: 53 push rbx
140a032f1: 4883ec38 sub rsp, 0x38
140a032f5: 488daa80000000 lea rbp, [rdx + 0x80]
140a032fc: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
140a03303: e8a83ce9ff call 0x140896fb0
140a03308: 90 nop 
140a03309: 4883c438 add rsp, 0x38
140a0330d: 5b pop rbx
140a0330e: 5f pop rdi
140a0330f: 5e pop rsi
140a03310: 415c pop r12
140a03312: 415d pop r13
140a03314: 415e pop r14
140a03316: 415f pop r15
140a03318: 5d pop rbp
140a03319: c3 ret 
140a0331a: 660f1f440000 nop word ptr [rax + rax]
140a03320: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03325: 55 push rbp
140a03326: 4157 push r15
140a03328: 4156 push r14
140a0332a: 4155 push r13
140a0332c: 4154 push r12
140a0332e: 56 push rsi
140a0332f: 57 push rdi
140a03330: 53 push rbx
140a03331: 4883ec38 sub rsp, 0x38
140a03335: 488daa80000000 lea rbp, [rdx + 0x80]
140a0333c: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a03343: e8683ce9ff call 0x140896fb0
140a03348: 90 nop 
140a03349: 4883c438 add rsp, 0x38
140a0334d: 5b pop rbx
140a0334e: 5f pop rdi
140a0334f: 5e pop rsi
140a03350: 415c pop r12
140a03352: 415d pop r13
140a03354: 415e pop r14
140a03356: 415f pop r15
140a03358: 5d pop rbp
140a03359: c3 ret 
140a0335a: 660f1f440000 nop word ptr [rax + rax]
140a03360: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03365: 55 push rbp
140a03366: 4157 push r15
140a03368: 4156 push r14
140a0336a: 4155 push r13
140a0336c: 4154 push r12
140a0336e: 56 push rsi
140a0336f: 57 push rdi
140a03370: 53 push rbx
140a03371: 4883ec38 sub rsp, 0x38
140a03375: 488daa80000000 lea rbp, [rdx + 0x80]
140a0337c: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
140a03383: e8283ce9ff call 0x140896fb0
140a03388: 90 nop 
140a03389: 4883c438 add rsp, 0x38
140a0338d: 5b pop rbx
140a0338e: 5f pop rdi
140a0338f: 5e pop rsi
140a03390: 415c pop r12
140a03392: 415d pop r13
140a03394: 415e pop r14
140a03396: 415f pop r15
140a03398: 5d pop rbp
140a03399: c3 ret 
140a0339a: 660f1f440000 nop word ptr [rax + rax]
140a033a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a033a5: 55 push rbp
140a033a6: 4157 push r15
140a033a8: 4156 push r14
140a033aa: 4155 push r13
140a033ac: 4154 push r12
140a033ae: 56 push rsi
140a033af: 57 push rdi
140a033b0: 53 push rbx
140a033b1: 4883ec38 sub rsp, 0x38
140a033b5: 488daa80000000 lea rbp, [rdx + 0x80]
140a033bc: 488d8d60040000 lea rcx, [rbp + 0x460]
140a033c3: e8e83be9ff call 0x140896fb0
140a033c8: 90 nop 
140a033c9: 4883c438 add rsp, 0x38
140a033cd: 5b pop rbx
140a033ce: 5f pop rdi
140a033cf: 5e pop rsi
140a033d0: 415c pop r12
140a033d2: 415d pop r13
140a033d4: 415e pop r14
140a033d6: 415f pop r15
140a033d8: 5d pop rbp
140a033d9: c3 ret 
140a033da: 660f1f440000 nop word ptr [rax + rax]
140a033e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a033e5: 55 push rbp
140a033e6: 4157 push r15
140a033e8: 4156 push r14
140a033ea: 4155 push r13
140a033ec: 4154 push r12
140a033ee: 56 push rsi
140a033ef: 57 push rdi
140a033f0: 53 push rbx
140a033f1: 4883ec38 sub rsp, 0x38
140a033f5: 488daa80000000 lea rbp, [rdx + 0x80]
140a033fc: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
140a03403: e8a83be9ff call 0x140896fb0
140a03408: 90 nop 
140a03409: 4883c438 add rsp, 0x38
140a0340d: 5b pop rbx
140a0340e: 5f pop rdi
140a0340f: 5e pop rsi
140a03410: 415c pop r12
140a03412: 415d pop r13
140a03414: 415e pop r14
140a03416: 415f pop r15
140a03418: 5d pop rbp
140a03419: c3 ret 
140a0341a: 660f1f440000 nop word ptr [rax + rax]
140a03420: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03425: 55 push rbp
140a03426: 4157 push r15
140a03428: 4156 push r14
140a0342a: 4155 push r13
140a0342c: 4154 push r12
140a0342e: 56 push rsi
140a0342f: 57 push rdi
140a03430: 53 push rbx
140a03431: 4883ec38 sub rsp, 0x38
140a03435: 488daa80000000 lea rbp, [rdx + 0x80]
140a0343c: 488d8d60040000 lea rcx, [rbp + 0x460]
140a03443: e8683be9ff call 0x140896fb0
140a03448: 90 nop 
140a03449: 4883c438 add rsp, 0x38
140a0344d: 5b pop rbx
140a0344e: 5f pop rdi
140a0344f: 5e pop rsi
140a03450: 415c pop r12
140a03452: 415d pop r13
140a03454: 415e pop r14
140a03456: 415f pop r15
140a03458: 5d pop rbp
140a03459: c3 ret 
140a0345a: 660f1f440000 nop word ptr [rax + rax]
140a03460: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03465: 55 push rbp
140a03466: 4157 push r15
140a03468: 4156 push r14
140a0346a: 4155 push r13
140a0346c: 4154 push r12
140a0346e: 56 push rsi
140a0346f: 57 push rdi
140a03470: 53 push rbx
140a03471: 4883ec38 sub rsp, 0x38
140a03475: 488daa80000000 lea rbp, [rdx + 0x80]
140a0347c: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
140a03483: e8283be9ff call 0x140896fb0
140a03488: 90 nop 
140a03489: 4883c438 add rsp, 0x38
140a0348d: 5b pop rbx
140a0348e: 5f pop rdi
140a0348f: 5e pop rsi
140a03490: 415c pop r12
140a03492: 415d pop r13
140a03494: 415e pop r14
140a03496: 415f pop r15
140a03498: 5d pop rbp
140a03499: c3 ret 
140a0349a: 660f1f440000 nop word ptr [rax + rax]
140a034a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a034a5: 55 push rbp
140a034a6: 4157 push r15
140a034a8: 4156 push r14
140a034aa: 4155 push r13
140a034ac: 4154 push r12
140a034ae: 56 push rsi
140a034af: 57 push rdi
140a034b0: 53 push rbx
140a034b1: 4883ec38 sub rsp, 0x38
140a034b5: 488daa80000000 lea rbp, [rdx + 0x80]
140a034bc: 488d8d60040000 lea rcx, [rbp + 0x460]
140a034c3: e8e83ae9ff call 0x140896fb0
140a034c8: 90 nop 
140a034c9: 4883c438 add rsp, 0x38
140a034cd: 5b pop rbx
140a034ce: 5f pop rdi
140a034cf: 5e pop rsi
140a034d0: 415c pop r12
140a034d2: 415d pop r13
140a034d4: 415e pop r14
140a034d6: 415f pop r15
140a034d8: 5d pop rbp
140a034d9: c3 ret 
140a034da: 660f1f440000 nop word ptr [rax + rax]
140a034e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a034e5: 55 push rbp
140a034e6: 4157 push r15
140a034e8: 4156 push r14
140a034ea: 4155 push r13
140a034ec: 4154 push r12
140a034ee: 56 push rsi
140a034ef: 57 push rdi
140a034f0: 53 push rbx
140a034f1: 4883ec38 sub rsp, 0x38
140a034f5: 488daa80000000 lea rbp, [rdx + 0x80]
140a034fc: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a03503: e8a83ae9ff call 0x140896fb0
140a03508: 90 nop 
140a03509: 4883c438 add rsp, 0x38
140a0350d: 5b pop rbx
140a0350e: 5f pop rdi
140a0350f: 5e pop rsi
140a03510: 415c pop r12
140a03512: 415d pop r13
140a03514: 415e pop r14
140a03516: 415f pop r15
140a03518: 5d pop rbp
140a03519: c3 ret 
140a0351a: 660f1f440000 nop word ptr [rax + rax]
140a03520: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03525: 55 push rbp
140a03526: 4157 push r15
140a03528: 4156 push r14
140a0352a: 4155 push r13
140a0352c: 4154 push r12
140a0352e: 56 push rsi
140a0352f: 57 push rdi
140a03530: 53 push rbx
140a03531: 4883ec38 sub rsp, 0x38
140a03535: 488daa80000000 lea rbp, [rdx + 0x80]
140a0353c: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
140a03543: e8683ae9ff call 0x140896fb0
140a03548: 90 nop 
140a03549: 4883c438 add rsp, 0x38
140a0354d: 5b pop rbx
140a0354e: 5f pop rdi
140a0354f: 5e pop rsi
140a03550: 415c pop r12
140a03552: 415d pop r13
140a03554: 415e pop r14
140a03556: 415f pop r15
140a03558: 5d pop rbp
140a03559: c3 ret 
140a0355a: 660f1f440000 nop word ptr [rax + rax]
140a03560: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03565: 55 push rbp
140a03566: 4157 push r15
140a03568: 4156 push r14
140a0356a: 4155 push r13
140a0356c: 4154 push r12
140a0356e: 56 push rsi
140a0356f: 57 push rdi
140a03570: 53 push rbx
140a03571: 4883ec38 sub rsp, 0x38
140a03575: 488daa80000000 lea rbp, [rdx + 0x80]
140a0357c: 488d8d60040000 lea rcx, [rbp + 0x460]
140a03583: e8283ae9ff call 0x140896fb0
140a03588: 90 nop 
140a03589: 4883c438 add rsp, 0x38
140a0358d: 5b pop rbx
140a0358e: 5f pop rdi
140a0358f: 5e pop rsi
140a03590: 415c pop r12
140a03592: 415d pop r13
140a03594: 415e pop r14
140a03596: 415f pop r15
140a03598: 5d pop rbp
140a03599: c3 ret 
140a0359a: 660f1f440000 nop word ptr [rax + rax]
140a035a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a035a5: 55 push rbp
140a035a6: 4157 push r15
140a035a8: 4156 push r14
140a035aa: 4155 push r13
140a035ac: 4154 push r12
140a035ae: 56 push rsi
140a035af: 57 push rdi
140a035b0: 53 push rbx
140a035b1: 4883ec38 sub rsp, 0x38
140a035b5: 488daa80000000 lea rbp, [rdx + 0x80]
140a035bc: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
140a035c3: e8e839e9ff call 0x140896fb0
140a035c8: 90 nop 
140a035c9: 4883c438 add rsp, 0x38
140a035cd: 5b pop rbx
140a035ce: 5f pop rdi
140a035cf: 5e pop rsi
140a035d0: 415c pop r12
140a035d2: 415d pop r13
140a035d4: 415e pop r14
140a035d6: 415f pop r15
140a035d8: 5d pop rbp
140a035d9: c3 ret 
140a035da: 660f1f440000 nop word ptr [rax + rax]
140a035e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a035e5: 55 push rbp
140a035e6: 4157 push r15
140a035e8: 4156 push r14
140a035ea: 4155 push r13
140a035ec: 4154 push r12
140a035ee: 56 push rsi
140a035ef: 57 push rdi
140a035f0: 53 push rbx
140a035f1: 4883ec38 sub rsp, 0x38
140a035f5: 488daa80000000 lea rbp, [rdx + 0x80]
140a035fc: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
140a03603: e8a839e9ff call 0x140896fb0
140a03608: 90 nop 
140a03609: 4883c438 add rsp, 0x38
140a0360d: 5b pop rbx
140a0360e: 5f pop rdi
140a0360f: 5e pop rsi
140a03610: 415c pop r12
140a03612: 415d pop r13
140a03614: 415e pop r14
140a03616: 415f pop r15
140a03618: 5d pop rbp
140a03619: c3 ret 
140a0361a: 660f1f440000 nop word ptr [rax + rax]
140a03620: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03625: 55 push rbp
140a03626: 4157 push r15
140a03628: 4156 push r14
140a0362a: 4155 push r13
140a0362c: 4154 push r12
140a0362e: 56 push rsi
140a0362f: 57 push rdi
140a03630: 53 push rbx
140a03631: 4883ec38 sub rsp, 0x38
140a03635: 488daa80000000 lea rbp, [rdx + 0x80]
140a0363c: 488d8d60040000 lea rcx, [rbp + 0x460]
140a03643: e86839e9ff call 0x140896fb0
140a03648: 90 nop 
140a03649: 4883c438 add rsp, 0x38
140a0364d: 5b pop rbx
140a0364e: 5f pop rdi
140a0364f: 5e pop rsi
140a03650: 415c pop r12
140a03652: 415d pop r13
140a03654: 415e pop r14
140a03656: 415f pop r15
140a03658: 5d pop rbp
140a03659: c3 ret 
140a0365a: 660f1f440000 nop word ptr [rax + rax]
140a03660: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03665: 55 push rbp
140a03666: 4157 push r15
140a03668: 4156 push r14
140a0366a: 4155 push r13
140a0366c: 4154 push r12
140a0366e: 56 push rsi
140a0366f: 57 push rdi
140a03670: 53 push rbx
140a03671: 4883ec38 sub rsp, 0x38
140a03675: 488daa80000000 lea rbp, [rdx + 0x80]
140a0367c: 488d8d60040000 lea rcx, [rbp + 0x460]
140a03683: e8a8953400 call 0x140d4cc30
140a03688: 90 nop 
140a03689: 4883c438 add rsp, 0x38
140a0368d: 5b pop rbx
140a0368e: 5f pop rdi
140a0368f: 5e pop rsi
140a03690: 415c pop r12
140a03692: 415d pop r13
140a03694: 415e pop r14
140a03696: 415f pop r15
140a03698: 5d pop rbp
140a03699: c3 ret 
140a0369a: 660f1f440000 nop word ptr [rax + rax]
140a036a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a036a5: 55 push rbp
140a036a6: 4157 push r15
140a036a8: 4156 push r14
140a036aa: 4155 push r13
140a036ac: 4154 push r12
140a036ae: 56 push rsi
140a036af: 57 push rdi
140a036b0: 53 push rbx
140a036b1: 4883ec38 sub rsp, 0x38
140a036b5: 488daa80000000 lea rbp, [rdx + 0x80]
140a036bc: 488d8d60040000 lea rcx, [rbp + 0x460]
140a036c3: e868953400 call 0x140d4cc30
140a036c8: 90 nop 
140a036c9: 4883c438 add rsp, 0x38
140a036cd: 5b pop rbx
140a036ce: 5f pop rdi
140a036cf: 5e pop rsi
140a036d0: 415c pop r12
140a036d2: 415d pop r13
140a036d4: 415e pop r14
140a036d6: 415f pop r15
140a036d8: 5d pop rbp
140a036d9: c3 ret 
140a036da: 660f1f440000 nop word ptr [rax + rax]
140a036e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a036e5: 55 push rbp
140a036e6: 4157 push r15
140a036e8: 4156 push r14
140a036ea: 4155 push r13
140a036ec: 4154 push r12
140a036ee: 56 push rsi
140a036ef: 57 push rdi
140a036f0: 53 push rbx
140a036f1: 4883ec38 sub rsp, 0x38
140a036f5: 488daa80000000 lea rbp, [rdx + 0x80]
140a036fc: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
140a03703: e8a838e9ff call 0x140896fb0
140a03708: 90 nop 
140a03709: 4883c438 add rsp, 0x38
140a0370d: 5b pop rbx
140a0370e: 5f pop rdi
140a0370f: 5e pop rsi
140a03710: 415c pop r12
140a03712: 415d pop r13
140a03714: 415e pop r14
140a03716: 415f pop r15
140a03718: 5d pop rbp
140a03719: c3 ret 
140a0371a: 660f1f440000 nop word ptr [rax + rax]
140a03720: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03725: 55 push rbp
140a03726: 4157 push r15
140a03728: 4156 push r14
140a0372a: 4155 push r13
140a0372c: 4154 push r12
140a0372e: 56 push rsi
140a0372f: 57 push rdi
140a03730: 53 push rbx
140a03731: 4883ec38 sub rsp, 0x38
140a03735: 488daa80000000 lea rbp, [rdx + 0x80]
140a0373c: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a03743: e86838e9ff call 0x140896fb0
140a03748: 90 nop 
140a03749: 4883c438 add rsp, 0x38
140a0374d: 5b pop rbx
140a0374e: 5f pop rdi
140a0374f: 5e pop rsi
140a03750: 415c pop r12
140a03752: 415d pop r13
140a03754: 415e pop r14
140a03756: 415f pop r15
140a03758: 5d pop rbp
140a03759: c3 ret 
140a0375a: 660f1f440000 nop word ptr [rax + rax]
140a03760: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03765: 55 push rbp
140a03766: 4157 push r15
140a03768: 4156 push r14
140a0376a: 4155 push r13
140a0376c: 4154 push r12
140a0376e: 56 push rsi
140a0376f: 57 push rdi
140a03770: 53 push rbx
140a03771: 4883ec38 sub rsp, 0x38
140a03775: 488daa80000000 lea rbp, [rdx + 0x80]
140a0377c: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
140a03783: e82838e9ff call 0x140896fb0
140a03788: 90 nop 
140a03789: 4883c438 add rsp, 0x38
140a0378d: 5b pop rbx
140a0378e: 5f pop rdi
140a0378f: 5e pop rsi
140a03790: 415c pop r12
140a03792: 415d pop r13
140a03794: 415e pop r14
140a03796: 415f pop r15
140a03798: 5d pop rbp
140a03799: c3 ret 
140a0379a: 660f1f440000 nop word ptr [rax + rax]
140a037a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a037a5: 55 push rbp
140a037a6: 4157 push r15
140a037a8: 4156 push r14
140a037aa: 4155 push r13
140a037ac: 4154 push r12
140a037ae: 56 push rsi
140a037af: 57 push rdi
140a037b0: 53 push rbx
140a037b1: 4883ec38 sub rsp, 0x38
140a037b5: 488daa80000000 lea rbp, [rdx + 0x80]
140a037bc: 488d8d60040000 lea rcx, [rbp + 0x460]
140a037c3: e8e837e9ff call 0x140896fb0
140a037c8: 90 nop 
140a037c9: 4883c438 add rsp, 0x38
140a037cd: 5b pop rbx
140a037ce: 5f pop rdi
140a037cf: 5e pop rsi
140a037d0: 415c pop r12
140a037d2: 415d pop r13
140a037d4: 415e pop r14
140a037d6: 415f pop r15
140a037d8: 5d pop rbp
140a037d9: c3 ret 
140a037da: 660f1f440000 nop word ptr [rax + rax]
140a037e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a037e5: 55 push rbp
140a037e6: 4157 push r15
140a037e8: 4156 push r14
140a037ea: 4155 push r13
140a037ec: 4154 push r12
140a037ee: 56 push rsi
140a037ef: 57 push rdi
140a037f0: 53 push rbx
140a037f1: 4883ec38 sub rsp, 0x38
140a037f5: 488daa80000000 lea rbp, [rdx + 0x80]
140a037fc: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
140a03803: e8a837e9ff call 0x140896fb0
140a03808: 90 nop 
140a03809: 4883c438 add rsp, 0x38
140a0380d: 5b pop rbx
140a0380e: 5f pop rdi
140a0380f: 5e pop rsi
140a03810: 415c pop r12
140a03812: 415d pop r13
140a03814: 415e pop r14
140a03816: 415f pop r15
140a03818: 5d pop rbp
140a03819: c3 ret 
140a0381a: 660f1f440000 nop word ptr [rax + rax]
140a03820: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03825: 55 push rbp
140a03826: 4157 push r15
140a03828: 4156 push r14
140a0382a: 4155 push r13
140a0382c: 4154 push r12
140a0382e: 56 push rsi
140a0382f: 57 push rdi
140a03830: 53 push rbx
140a03831: 4883ec38 sub rsp, 0x38
140a03835: 488daa80000000 lea rbp, [rdx + 0x80]
140a0383c: 488d8d60040000 lea rcx, [rbp + 0x460]
140a03843: e86837e9ff call 0x140896fb0
140a03848: 90 nop 
140a03849: 4883c438 add rsp, 0x38
140a0384d: 5b pop rbx
140a0384e: 5f pop rdi
140a0384f: 5e pop rsi
140a03850: 415c pop r12
140a03852: 415d pop r13
140a03854: 415e pop r14
140a03856: 415f pop r15
140a03858: 5d pop rbp
140a03859: c3 ret 
140a0385a: 660f1f440000 nop word ptr [rax + rax]
140a03860: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03865: 55 push rbp
140a03866: 4157 push r15
140a03868: 4156 push r14
140a0386a: 4155 push r13
140a0386c: 4154 push r12
140a0386e: 56 push rsi
140a0386f: 57 push rdi
140a03870: 53 push rbx
140a03871: 4883ec38 sub rsp, 0x38
140a03875: 488daa80000000 lea rbp, [rdx + 0x80]
140a0387c: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
140a03883: e82837e9ff call 0x140896fb0
140a03888: 90 nop 
140a03889: 4883c438 add rsp, 0x38
140a0388d: 5b pop rbx
140a0388e: 5f pop rdi
140a0388f: 5e pop rsi
140a03890: 415c pop r12
140a03892: 415d pop r13
140a03894: 415e pop r14
140a03896: 415f pop r15
140a03898: 5d pop rbp
140a03899: c3 ret 
140a0389a: 660f1f440000 nop word ptr [rax + rax]
140a038a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a038a5: 55 push rbp
140a038a6: 4157 push r15
140a038a8: 4156 push r14
140a038aa: 4155 push r13
140a038ac: 4154 push r12
140a038ae: 56 push rsi
140a038af: 57 push rdi
140a038b0: 53 push rbx
140a038b1: 4883ec38 sub rsp, 0x38
140a038b5: 488daa80000000 lea rbp, [rdx + 0x80]
140a038bc: 488d8d60040000 lea rcx, [rbp + 0x460]
140a038c3: e8e836e9ff call 0x140896fb0
140a038c8: 90 nop 
140a038c9: 4883c438 add rsp, 0x38
140a038cd: 5b pop rbx
140a038ce: 5f pop rdi
140a038cf: 5e pop rsi
140a038d0: 415c pop r12
140a038d2: 415d pop r13
140a038d4: 415e pop r14
140a038d6: 415f pop r15
140a038d8: 5d pop rbp
140a038d9: c3 ret 
140a038da: 660f1f440000 nop word ptr [rax + rax]
140a038e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a038e5: 55 push rbp
140a038e6: 4157 push r15
140a038e8: 4156 push r14
140a038ea: 4155 push r13
140a038ec: 4154 push r12
140a038ee: 56 push rsi
140a038ef: 57 push rdi
140a038f0: 53 push rbx
140a038f1: 4883ec38 sub rsp, 0x38
140a038f5: 488daa80000000 lea rbp, [rdx + 0x80]
140a038fc: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
140a03903: e8a836e9ff call 0x140896fb0
140a03908: 90 nop 
140a03909: 4883c438 add rsp, 0x38
140a0390d: 5b pop rbx
140a0390e: 5f pop rdi
140a0390f: 5e pop rsi
140a03910: 415c pop r12
140a03912: 415d pop r13
140a03914: 415e pop r14
140a03916: 415f pop r15
140a03918: 5d pop rbp
140a03919: c3 ret 
140a0391a: 660f1f440000 nop word ptr [rax + rax]
140a03920: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03925: 55 push rbp
140a03926: 4157 push r15
140a03928: 4156 push r14
140a0392a: 4155 push r13
140a0392c: 4154 push r12
140a0392e: 56 push rsi
140a0392f: 57 push rdi
140a03930: 53 push rbx
140a03931: 4883ec38 sub rsp, 0x38
140a03935: 488daa80000000 lea rbp, [rdx + 0x80]
140a0393c: 488d8d60040000 lea rcx, [rbp + 0x460]
140a03943: e86836e9ff call 0x140896fb0
140a03948: 90 nop 
140a03949: 4883c438 add rsp, 0x38
140a0394d: 5b pop rbx
140a0394e: 5f pop rdi
140a0394f: 5e pop rsi
140a03950: 415c pop r12
140a03952: 415d pop r13
140a03954: 415e pop r14
140a03956: 415f pop r15
140a03958: 5d pop rbp
140a03959: c3 ret 
140a0395a: 660f1f440000 nop word ptr [rax + rax]
140a03960: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03965: 55 push rbp
140a03966: 4157 push r15
140a03968: 4156 push r14
140a0396a: 4155 push r13
140a0396c: 4154 push r12
140a0396e: 56 push rsi
140a0396f: 57 push rdi
140a03970: 53 push rbx
140a03971: 4883ec38 sub rsp, 0x38
140a03975: 488daa80000000 lea rbp, [rdx + 0x80]
140a0397c: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
140a03983: e82836e9ff call 0x140896fb0
140a03988: 90 nop 
140a03989: 4883c438 add rsp, 0x38
140a0398d: 5b pop rbx
140a0398e: 5f pop rdi
140a0398f: 5e pop rsi
140a03990: 415c pop r12
140a03992: 415d pop r13
140a03994: 415e pop r14
140a03996: 415f pop r15
140a03998: 5d pop rbp
140a03999: c3 ret 
140a0399a: 660f1f440000 nop word ptr [rax + rax]
140a039a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a039a5: 55 push rbp
140a039a6: 4157 push r15
140a039a8: 4156 push r14
140a039aa: 4155 push r13
140a039ac: 4154 push r12
140a039ae: 56 push rsi
140a039af: 57 push rdi
140a039b0: 53 push rbx
140a039b1: 4883ec38 sub rsp, 0x38
140a039b5: 488daa80000000 lea rbp, [rdx + 0x80]
140a039bc: 488d8d60040000 lea rcx, [rbp + 0x460]
140a039c3: e8e835e9ff call 0x140896fb0
140a039c8: 90 nop 
140a039c9: 4883c438 add rsp, 0x38
140a039cd: 5b pop rbx
140a039ce: 5f pop rdi
140a039cf: 5e pop rsi
140a039d0: 415c pop r12
140a039d2: 415d pop r13
140a039d4: 415e pop r14
140a039d6: 415f pop r15
140a039d8: 5d pop rbp
140a039d9: c3 ret 
140a039da: 660f1f440000 nop word ptr [rax + rax]
140a039e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a039e5: 55 push rbp
140a039e6: 4157 push r15
140a039e8: 4156 push r14
140a039ea: 4155 push r13
140a039ec: 4154 push r12
140a039ee: 56 push rsi
140a039ef: 57 push rdi
140a039f0: 53 push rbx
140a039f1: 4883ec38 sub rsp, 0x38
140a039f5: 488daa80000000 lea rbp, [rdx + 0x80]
140a039fc: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a03a03: e8a835e9ff call 0x140896fb0
140a03a08: 90 nop 
140a03a09: 4883c438 add rsp, 0x38
140a03a0d: 5b pop rbx
140a03a0e: 5f pop rdi
140a03a0f: 5e pop rsi
140a03a10: 415c pop r12
140a03a12: 415d pop r13
140a03a14: 415e pop r14
140a03a16: 415f pop r15
140a03a18: 5d pop rbp
140a03a19: c3 ret 
140a03a1a: 660f1f440000 nop word ptr [rax + rax]
140a03a20: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03a25: 55 push rbp
140a03a26: 4157 push r15
140a03a28: 4156 push r14
140a03a2a: 4155 push r13
140a03a2c: 4154 push r12
140a03a2e: 56 push rsi
140a03a2f: 57 push rdi
140a03a30: 53 push rbx
140a03a31: 4883ec38 sub rsp, 0x38
140a03a35: 488daa80000000 lea rbp, [rdx + 0x80]
140a03a3c: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
140a03a43: e86835e9ff call 0x140896fb0
140a03a48: 90 nop 
140a03a49: 4883c438 add rsp, 0x38
140a03a4d: 5b pop rbx
140a03a4e: 5f pop rdi
140a03a4f: 5e pop rsi
140a03a50: 415c pop r12
140a03a52: 415d pop r13
140a03a54: 415e pop r14
140a03a56: 415f pop r15
140a03a58: 5d pop rbp
140a03a59: c3 ret 
140a03a5a: 660f1f440000 nop word ptr [rax + rax]
140a03a60: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03a65: 55 push rbp
140a03a66: 4157 push r15
140a03a68: 4156 push r14
140a03a6a: 4155 push r13
140a03a6c: 4154 push r12
140a03a6e: 56 push rsi
140a03a6f: 57 push rdi
140a03a70: 53 push rbx
140a03a71: 4883ec38 sub rsp, 0x38
140a03a75: 488daa80000000 lea rbp, [rdx + 0x80]
140a03a7c: 488d8d60040000 lea rcx, [rbp + 0x460]
140a03a83: e82835e9ff call 0x140896fb0
140a03a88: 90 nop 
140a03a89: 4883c438 add rsp, 0x38
140a03a8d: 5b pop rbx
140a03a8e: 5f pop rdi
140a03a8f: 5e pop rsi
140a03a90: 415c pop r12
140a03a92: 415d pop r13
140a03a94: 415e pop r14
140a03a96: 415f pop r15
140a03a98: 5d pop rbp
140a03a99: c3 ret 
140a03a9a: 660f1f440000 nop word ptr [rax + rax]
140a03aa0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03aa5: 55 push rbp
140a03aa6: 4157 push r15
140a03aa8: 4156 push r14
140a03aaa: 4155 push r13
140a03aac: 4154 push r12
140a03aae: 56 push rsi
140a03aaf: 57 push rdi
140a03ab0: 53 push rbx
140a03ab1: 4883ec38 sub rsp, 0x38
140a03ab5: 488daa80000000 lea rbp, [rdx + 0x80]
140a03abc: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
140a03ac3: e8e834e9ff call 0x140896fb0
140a03ac8: 90 nop 
140a03ac9: 4883c438 add rsp, 0x38
140a03acd: 5b pop rbx
140a03ace: 5f pop rdi
140a03acf: 5e pop rsi
140a03ad0: 415c pop r12
140a03ad2: 415d pop r13
140a03ad4: 415e pop r14
140a03ad6: 415f pop r15
140a03ad8: 5d pop rbp
140a03ad9: c3 ret 
140a03ada: 660f1f440000 nop word ptr [rax + rax]
140a03ae0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03ae5: 55 push rbp
140a03ae6: 4157 push r15
140a03ae8: 4156 push r14
140a03aea: 4155 push r13
140a03aec: 4154 push r12
140a03aee: 56 push rsi
140a03aef: 57 push rdi
140a03af0: 53 push rbx
140a03af1: 4883ec38 sub rsp, 0x38
140a03af5: 488daa80000000 lea rbp, [rdx + 0x80]
140a03afc: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
140a03b03: e8a834e9ff call 0x140896fb0
140a03b08: 90 nop 
140a03b09: 4883c438 add rsp, 0x38
140a03b0d: 5b pop rbx
140a03b0e: 5f pop rdi
140a03b0f: 5e pop rsi
140a03b10: 415c pop r12
140a03b12: 415d pop r13
140a03b14: 415e pop r14
140a03b16: 415f pop r15
140a03b18: 5d pop rbp
140a03b19: c3 ret 
140a03b1a: 660f1f440000 nop word ptr [rax + rax]
140a03b20: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03b25: 55 push rbp
140a03b26: 4157 push r15
140a03b28: 4156 push r14
140a03b2a: 4155 push r13
140a03b2c: 4154 push r12
140a03b2e: 56 push rsi
140a03b2f: 57 push rdi
140a03b30: 53 push rbx
140a03b31: 4883ec38 sub rsp, 0x38
140a03b35: 488daa80000000 lea rbp, [rdx + 0x80]
140a03b3c: 488d8d60040000 lea rcx, [rbp + 0x460]
140a03b43: e86834e9ff call 0x140896fb0
140a03b48: 90 nop 
140a03b49: 4883c438 add rsp, 0x38
140a03b4d: 5b pop rbx
140a03b4e: 5f pop rdi
140a03b4f: 5e pop rsi
140a03b50: 415c pop r12
140a03b52: 415d pop r13
140a03b54: 415e pop r14
140a03b56: 415f pop r15
140a03b58: 5d pop rbp
140a03b59: c3 ret 
140a03b5a: 660f1f440000 nop word ptr [rax + rax]
140a03b60: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a03b65: 55 push rbp
140a03b66: 4157 push r15
140a03b68: 4156 push r14
140a03b6a: 4155 push r13
140a03b6c: 4154 push r12
140a03b6e: 56 push rsi
140a03b6f: 57 push rdi
140a03b70: 53 push rbx
140a03b71: 4883ec38 sub rsp, 0x38
140a03b75: 488daa80000000 lea rbp, [rdx + 0x80]
140a03b7c: 488d8dd0020000 lea rcx, [rbp + 0x2d0]
140a03b83: e82834e9ff call 0x140896fb0
140a03b88: 90 nop 
140a03b89: 4883c438 add rsp, 0x38
140a03b8d: 5b pop rbx
140a03b8e: 5f pop rdi
140a03b8f: 5e pop rsi
140a03b90: 415c pop r12
140a03b92: 415d pop r13
140a03b94: 415e pop r14
140a03b96: 415f pop r15
140a03b98: 5d pop rbp
140a03b99: c3 ret 
