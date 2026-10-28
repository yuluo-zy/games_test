0x1409bf800: push rbp
0x1409bf801: push r15
0x1409bf803: push r14
0x1409bf805: push r13
0x1409bf807: push r12
0x1409bf809: push rsi
0x1409bf80a: push rdi
0x1409bf80b: push rbx
0x1409bf80c: sub rsp, 0xc28
0x1409bf813: lea rbp, [rsp + 0x80]
0x1409bf81b: mov qword ptr [rbp + 0xba0], 0xfffffffffffffffe
0x1409bf826: mov rsi, r9
0x1409bf829: mov rdi, r8
0x1409bf82c: mov rbx, rdx
0x1409bf82f: mov r12, rcx
0x1409bf832: mov r14, qword ptr [rbp + 0xc10]
0x1409bf839: lea rcx, [rbp + 0x9a0]
0x1409bf840: mov edx, 0x10
0x1409bf845: mov r8d, 0x20
0x1409bf84b: call 0x140bd4e20
0x1409bf850: movups xmm0, xmmword ptr [rbp + 0x9a0]
0x1409bf857: movaps xmmword ptr [rbp + 0x1c0], xmm0
0x1409bf85e: lea rcx, [rbp + 0x5c0]
0x1409bf865: lea rdx, [rbp + 0x1c0]
0x1409bf86c: call 0x140bd6d00
0x1409bf871: lea rax, [rip + 0x2100379]
0x1409bf878: mov qword ptr [rbp + 0x5b0], rax
0x1409bf87f: mov qword ptr [rbp + 0x5b8], 0x15
0x1409bf88a: lea rdx, [rbp + 0x5b0]
0x1409bf891: mov rcx, r12
0x1409bf894: call 0x140be94d0
0x1409bf899: movups xmm0, xmmword ptr [rbp + 0x9a0]
0x1409bf8a0: movaps xmmword ptr [rbp + 0xb60], xmm0
0x1409bf8a7: mov dword ptr [rbp + 0xb70], eax
0x1409bf8ad: lea r8, [rbp + 0xb60]
0x1409bf8b4: mov rcx, r12
0x1409bf8b7: mov rdx, rbx
0x1409bf8ba: call 0x1409ddbb0
0x1409bf8bf: lea rcx, [r14 + 4]
0x1409bf8c3: call 0x140bd5790
0x1409bf8c8: mov rcx, rax
0x1409bf8cb: shr rcx, 0x20
0x1409bf8cf: imul eax, ecx
0x1409bf8d2: mov edx, eax
0x1409bf8d4: shr edx, 4
0x1409bf8d7: add edx, eax
0x1409bf8d9: shl rdx, 4
0x1409bf8dd: lea r15, [rbp + 0x9a0]
0x1409bf8e4: mov rcx, r15
0x1409bf8e7: mov r8d, 0x20
0x1409bf8ed: call 0x140bd4e20
0x1409bf8f2: movups xmm0, xmmword ptr [rbp + 0x9a0]
0x1409bf8f9: movaps xmmword ptr [rbp + 0x1c0], xmm0
0x1409bf900: lea rcx, [rbp + 0x5c0]
0x1409bf907: lea rdx, [rbp + 0x1c0]
0x1409bf90e: call 0x140bd6d00
0x1409bf913: lea rax, [rip + 0x21002ec]
0x1409bf91a: mov qword ptr [rbp + 0x5b0], rax
0x1409bf921: mov qword ptr [rbp + 0x5b8], 0x13
0x1409bf92c: lea rdx, [rbp + 0x5b0]
0x1409bf933: mov rcx, r12
0x1409bf936: call 0x140be94d0
0x1409bf93b: movups xmm0, xmmword ptr [rbp + 0x9a0]
0x1409bf942: movaps xmmword ptr [rbp + 0xb30], xmm0
0x1409bf949: mov dword ptr [rbp + 0xb40], eax
0x1409bf94f: lea rcx, [rsi + 4]
0x1409bf953: call 0x140bd5790
0x1409bf958: mov dword ptr [rbp + 0x5b0], 0
0x1409bf962: mov qword ptr [rbp + 0xb58], rdi
0x1409bf969: mov qword ptr [rbp + 0x5b8], rdi
0x1409bf970: mov qword ptr [rbp + 0x9a0], 0
0x1409bf97b: mov qword ptr [rbp + 0x9a8], rsi
0x1409bf982: mov qword ptr [rbp + 0x9b0], 0
0x1409bf98d: mov qword ptr [rbp + 0xb50], r12
0x1409bf994: mov qword ptr [rsp + 0x40], r12
0x1409bf999: mov qword ptr [rsp + 0x38], rbx
0x1409bf99e: mov qword ptr [rsp + 0x30], rax
0x1409bf9a3: mov qword ptr [rsp + 0x28], r15
0x1409bf9a8: mov qword ptr [rsp + 0x20], 1
0x1409bf9b1: lea rdx, [rip + 0x2100261]
0x1409bf9b8: lea rcx, [rbp + 0x1c0]
0x1409bf9bf: lea r12, [rbp + 0x5b0]
0x1409bf9c6: mov r8d, 0xb
0x1409bf9cc: mov r9, r12
0x1409bf9cf: call 0x140867970
0x1409bf9d4: mov qword ptr [rbp + 0xb90], rbx
0x1409bf9db: lea rcx, [rbx + 0x38]
0x1409bf9df: mov byte ptr [rbp + 0xb9f], 1
0x1409bf9e6: movabs rdx, 0x4bc362195b80f729
0x1409bf9f0: movabs r8, 0x982798ce80cbcefb
0x1409bf9fa: call 0x1400dd6f0
0x1409bf9ff: test rax, rax
0x1409bfa02: mov rcx, qword ptr [rbp + 0xb90]
0x1409bfa09: je 0x1409c040a
0x1409bfa0f: cmp qword ptr [rcx + 0x218], rdx
0x1409bfa16: jbe 0x1409c040a
0x1409bfa1c: mov rax, qword ptr [rcx + 0x210]
0x1409bfa23: mov rax, qword ptr [rax + rdx*8]
0x1409bfa27: test rax, rax
0x1409bfa2a: je 0x1409c040a
0x1409bfa30: mov rdx, qword ptr [rcx + 0x1e0]
0x1409bfa37: not rax
0x1409bfa3a: lea r8, [rax + rax*2]
0x1409bfa3e: shl r8, 5
0x1409bfa42: cmp qword ptr [rdx + r8 + 0x20], 0
0x1409bfa48: je 0x1409c040a
0x1409bfa4e: mov rcx, qword ptr [rbp + 0xc38]
0x1409bfa55: movzx eax, byte ptr [rbp + 0xc20]
0x1409bfa5c: add rdx, r8
0x1409bfa5f: mov rdx, qword ptr [rdx + 0x10]
0x1409bfa63: mov r8, qword ptr [rdx]
0x1409bfa66: test r8, r8
0x1409bfa69: js 0x1409bfa75
0x1409bfa6b: xorps xmm0, xmm0
0x1409bfa6e: cvtsi2ss xmm0, r8
0x1409bfa73: jmp 0x1409bfa8e
0x1409bfa75: mov r9, r8
0x1409bfa78: shr r9, 1
0x1409bfa7b: and r8d, 1
0x1409bfa7f: or r8, r9
0x1409bfa82: xorps xmm0, xmm0
0x1409bfa85: cvtsi2ss xmm0, r8
0x1409bfa8a: addss xmm0, xmm0
0x1409bfa8e: cvtsi2ss xmm1, dword ptr [rdx + 8]
0x1409bfa93: divss xmm1, dword ptr [rip + 0x1fe7ba5]
0x1409bfa9b: addss xmm1, xmm0
0x1409bfa9f: movss dword ptr [rbp + 0xb80], xmm1
0x1409bfaa7: mov byte ptr [rbp + 0xb84], 0
0x1409bfaae: mov r13, qword ptr [rcx + 0x170]
0x1409bfab5: mov rcx, qword ptr [rcx + 0x178]
0x1409bfabc: shl rcx, 7
0x1409bfac0: add rcx, r13
0x1409bfac3: mov qword ptr [rbp + 0xb48], rcx
0x1409bfaca: movzx eax, al
0x1409bfacd: mov dword ptr [rbp + 0xb98], eax
0x1409bfad3: lea rsi, [rbp + 0xb88]
0x1409bfada: cmp r13, qword ptr [rbp + 0xb48]
0x1409bfae1: je 0x1409bfe54
0x1409bfae7: nop word ptr [rax + rax]
0x1409bfaf0: mov rbx, qword ptr [r13 + 0x50]
0x1409bfaf4: mov rdi, qword ptr [r13 + 0x58]
0x1409bfaf8: mov byte ptr [rbp + 0xb9f], 1
0x1409bfaff: call 0x140be6750
0x1409bfb04: mov r14d, eax
0x1409bfb07: mov byte ptr [rbp + 0xb9f], 1
0x1409bfb0e: xor ecx, ecx
0x1409bfb10: call 0x1400d9b20
0x1409bfb15: test rax, rax
0x1409bfb18: je 0x1409c03f5
0x1409bfb1e: mov dword ptr [rbp + 0xa48], r14d
0x1409bfb25: mov word ptr [rbp + 0xa50], 0
0x1409bfb2e: mov dword ptr [rbp + 0xa4c], 0x3f800000
0x1409bfb38: mov qword ptr [rbp + 0x9a0], 1
0x1409bfb43: mov qword ptr [rbp + 0x9a8], rbx
0x1409bfb4a: mov qword ptr [rbp + 0xa40], rdi
0x1409bfb51: mov qword ptr [rbp + 0x9b0], 0
0x1409bfb5c: mov qword ptr [rbp + 0x9b8], 4
0x1409bfb67: mov qword ptr [rbp + 0x9c0], 0
0x1409bfb72: movups xmm0, xmmword ptr [rax]
0x1409bfb75: inc qword ptr [rax]
0x1409bfb78: movups xmm1, xmmword ptr [rip + 0x20d6bb1]
0x1409bfb7f: lea rax, [rbp + 0xa10]
0x1409bfb86: movups xmmword ptr [rax + 0x10], xmm1
0x1409bfb8a: movups xmm1, xmmword ptr [rip + 0x20d6b8f]
0x1409bfb91: movups xmmword ptr [rax], xmm1
0x1409bfb94: movups xmmword ptr [rbp + 0xa30], xmm0
0x1409bfb9b: movabs rax, 0x100000003
0x1409bfba5: mov qword ptr [rbp + 0x9e0], rax
0x1409bfbac: mov qword ptr [rbp + 0x9c8], 0
0x1409bfbb7: mov qword ptr [rbp + 0x9d0], 1
0x1409bfbc2: mov qword ptr [rbp + 0x9d8], 0
0x1409bfbcd: mov byte ptr [rbp + 0xb9f], 1
0x1409bfbd4: mov rcx, r12
0x1409bfbd7: mov rdx, r15
0x1409bfbda: lea r8, [rbp + 0x1c0]
0x1409bfbe1: xor r9d, r9d
0x1409bfbe4: call 0x140905be0
0x1409bfbe9: mov byte ptr [rbp + 0xb9f], 1
0x1409bfbf0: mov rcx, r15
0x1409bfbf3: mov rdx, r12
0x1409bfbf6: lea r8, [rbp + 0xb80]
0x1409bfbfd: mov r9, r13
0x1409bfc00: call 0x14090b460
0x1409bfc05: mov rax, qword ptr [rbp + 0xc10]
0x1409bfc0c: mov qword ptr [rbp + 0xb88], rax
0x1409bfc13: mov dword ptr [rbp + 0x5b0], 2
0x1409bfc1d: mov qword ptr [rsp + 0x20], r12
0x1409bfc22: mov rdi, rsi
0x1409bfc25: mov rcx, rsi
0x1409bfc28: mov rdx, r15
0x1409bfc2b: movabs r8, 0x2a333c3a28028230
0x1409bfc35: mov r9b, 0x1d
0x1409bfc38: call 0x1408ee800
0x1409bfc3d: mov r8d, 0xf8
0x1409bfc43: mov rcx, r12
0x1409bfc46: mov rdx, r15
0x1409bfc49: call 0x142923c00
0x1409bfc4e: mov rax, qword ptr [rbp + 0xc18]
0x1409bfc55: mov qword ptr [rbp + 0xb88], rax
0x1409bfc5c: mov dword ptr [rbp + 0x9a0], 2
0x1409bfc66: mov qword ptr [rsp + 0x20], r15
0x1409bfc6b: mov rcx, rdi
0x1409bfc6e: mov rdx, r12
0x1409bfc71: movabs r8, 0x4175185826c09d0
0x1409bfc7b: mov r9b, 0x1d
0x1409bfc7e: call 0x1408ee800
0x1409bfc83: mov r8d, 0xf8
0x1409bfc89: lea rcx, [rbp + 0xc8]
0x1409bfc90: mov rdx, r12
0x1409bfc93: call 0x142923c00
0x1409bfc98: cmp byte ptr [rbp + 0x1a6], 8
0x1409bfc9f: je 0x1409bfccd
0x1409bfca1: mov rdx, qword ptr [rbp + 0x1b0]
0x1409bfca8: mov eax, 0xe8
0x1409bfcad: add rdx, rax
0x1409bfcb0: lea rcx, [rbp + 0x180]
0x1409bfcb7: movabs r8, 0xa9a109b760eed902
0x1409bfcc1: mov r9d, dword ptr [rbp + 0xb98]
0x1409bfcc8: call 0x14096b570
0x1409bfccd: mov r8d, 0xf8
0x1409bfcd3: mov rcx, r12
0x1409bfcd6: lea rdi, [rbp + 0xc8]
0x1409bfcdd: mov rdx, rdi
0x1409bfce0: call 0x142923c00
0x1409bfce5: mov rax, qword ptr [rbp + 0xc30]
0x1409bfcec: mov qword ptr [rbp + 0xc8], rax
0x1409bfcf3: mov dword ptr [rbp + 0x9a0], 2
0x1409bfcfd: mov qword ptr [rsp + 0x20], r15
0x1409bfd02: mov rcx, rdi
0x1409bfd05: mov rdx, r12
0x1409bfd08: movabs r8, 0xb8a74ccd835a6be3
0x1409bfd12: mov r9b, 0x1d
0x1409bfd15: call 0x1408ee800
0x1409bfd1a: mov r8d, 0xf8
0x1409bfd20: mov rcx, r15
0x1409bfd23: mov rdx, r12
0x1409bfd26: call 0x142923c00
0x1409bfd2b: mov byte ptr [rbp + 0xb9f], 1
0x1409bfd32: mov rcx, r12
0x1409bfd35: mov rdx, r15
0x1409bfd38: movabs r8, 0x1863d10d6efe0b4e
0x1409bfd42: lea r9, [rbp + 0xb60]
0x1409bfd49: call 0x1409067e0
0x1409bfd4e: mov byte ptr [rbp + 0xb9f], 1
0x1409bfd55: lea rcx, [rbp - 0x30]
0x1409bfd59: mov rdx, r12
0x1409bfd5c: movabs r8, 0x8f7b94e37966c87b
0x1409bfd66: lea r9, [rbp + 0xb30]
0x1409bfd6d: call 0x1409067e0
0x1409bfd72: mov rax, qword ptr [rbp + 0xc28]
0x1409bfd79: mov qword ptr [rbp + 0x9a0], rax
0x1409bfd80: mov dword ptr [rbp + 0x5b0], 2
0x1409bfd8a: mov qword ptr [rsp + 0x20], r12
0x1409bfd8f: mov rcx, r15
0x1409bfd92: lea rdx, [rbp - 0x30]
0x1409bfd96: movabs r8, 0xe9208e8a28884e3c
0x1409bfda0: mov r9b, 0x1d
0x1409bfda3: call 0x1408ee800
0x1409bfda8: mov rdi, qword ptr [rbp - 0x30]
0x1409bfdac: mov r8d, 0xa3
0x1409bfdb2: lea r14, [rbp + 0x5b8]
0x1409bfdb9: mov rcx, r14
0x1409bfdbc: lea rbx, [rbp - 0x28]
0x1409bfdc0: mov rdx, rbx
0x1409bfdc3: call 0x142923c00
0x1409bfdc8: movzx eax, byte ptr [rbp + 0x7b]
0x1409bfdcc: movups xmm0, xmmword ptr [rbx + 0xa4]
0x1409bfdd3: movups xmm1, xmmword ptr [rbx + 0xb4]
0x1409bfdda: movups xmm2, xmmword ptr [rbx + 0xc4]
0x1409bfde1: movups xmm3, xmmword ptr [rbx + 0xd4]
0x1409bfde8: movups xmmword ptr [r14 + 0xa4], xmm0
0x1409bfdf0: movups xmmword ptr [r14 + 0xb4], xmm1
0x1409bfdf8: movups xmmword ptr [r14 + 0xc4], xmm2
0x1409bfe00: movups xmmword ptr [r14 + 0xd4], xmm3
0x1409bfe08: movups xmm0, xmmword ptr [rbx + 0xe0]
0x1409bfe0f: movups xmmword ptr [r14 + 0xe0], xmm0
0x1409bfe17: and al, 0x9f
0x1409bfe19: or al, 0x40
0x1409bfe1b: mov r9, qword ptr [r13 + 0x70]
0x1409bfe1f: mov qword ptr [rbp + 0x5b0], rdi
0x1409bfe26: mov byte ptr [rbp + 0x65b], al
0x1409bfe2c: mov byte ptr [rbp + 0xb9f], 1
0x1409bfe33: mov rcx, r12
0x1409bfe36: mov rdx, r13
0x1409bfe39: mov r8, qword ptr [rbp + 0xb90]
0x1409bfe40: call r9
0x1409bfe43: sub r13, -0x80
0x1409bfe47: cmp r13, qword ptr [rbp + 0xb48]
0x1409bfe4e: jne 0x1409bfaf0
0x1409bfe54: lea rsi, [rbp + 0x5b0]
0x1409bfe5b: lea rdx, [rbp + 0x1c0]
0x1409bfe62: mov r8d, 0x3e8
0x1409bfe68: mov rcx, rsi
0x1409bfe6b: call 0x142923c00
0x1409bfe70: mov byte ptr [rbp + 0xb9f], 0
0x1409bfe77: mov rcx, rsi
0x1409bfe7a: call 0x140868370
0x1409bfe7f: mov rdi, qword ptr [rbp + 0xb50]
0x1409bfe86: mov qword ptr [rsp + 0x30], rdi
0x1409bfe8b: mov r14, qword ptr [rbp + 0xb90]
0x1409bfe92: mov qword ptr [rsp + 0x28], r14
0x1409bfe97: mov qword ptr [rsp + 0x20], 0x24
0x1409bfea0: lea rdx, [rip + 0x20ffd7d]
0x1409bfea7: lea r9, [rip + 0x20ffd8c]
0x1409bfeae: mov r8d, 0x16
0x1409bfeb4: mov rcx, rsi
0x1409bfeb7: call 0x1416032a0
0x1409bfebc: lea rcx, [rbp + 0x1c0]
0x1409bfec3: movabs r8, 0x1863d10d6efe0b4e
0x1409bfecd: lea r9, [rbp + 0xb60]
0x1409bfed4: mov rdx, rsi
0x1409bfed7: call 0x14096dd60
0x1409bfedc: movups xmm0, xmmword ptr [rbp + 0x260]
0x1409bfee3: movaps xmmword ptr [rbp + 0x9e0], xmm0
0x1409bfeea: movups xmm0, xmmword ptr [rbp + 0x250]
0x1409bfef1: movaps xmmword ptr [rbp + 0x9d0], xmm0
0x1409bfef8: movups xmm0, xmmword ptr [rbp + 0x240]
0x1409bfeff: movaps xmmword ptr [rbp + 0x9c0], xmm0
0x1409bff06: movups xmm0, xmmword ptr [rbp + 0x230]
0x1409bff0d: movaps xmmword ptr [rbp + 0x9b0], xmm0
0x1409bff14: movups xmm0, xmmword ptr [rbp + 0x220]
0x1409bff1b: movaps xmmword ptr [rbp + 0x9a0], xmm0
0x1409bff22: mov rax, qword ptr [rbp + 0x2f8]
0x1409bff29: mov rcx, qword ptr [rbp + 0x340]
0x1409bff30: mov rdx, qword ptr [rbp + 0x1d0]
0x1409bff37: mov qword ptr [rbp + 0x5c0], rdx
0x1409bff3e: movups xmm0, xmmword ptr [rbp + 0x1c0]
0x1409bff45: movaps xmmword ptr [rbp + 0x5b0], xmm0
0x1409bff4c: movups xmm0, xmmword ptr [rbp + 0x1d8]
0x1409bff53: movaps xmmword ptr [rbp + 0x5e0], xmm0
0x1409bff5a: mov rdx, qword ptr [rbp + 0x1e8]
0x1409bff61: mov qword ptr [rbp + 0x5f0], rdx
0x1409bff68: movups xmm0, xmmword ptr [rbp + 0x1f0]
0x1409bff6f: movups xmmword ptr [rbp + 0x5c8], xmm0
0x1409bff76: mov rdx, qword ptr [rbp + 0x200]
0x1409bff7d: mov qword ptr [rbp + 0x5d8], rdx
0x1409bff84: movups xmm0, xmmword ptr [rbp + 0x270]
0x1409bff8b: movups xmm1, xmmword ptr [rbp + 0x280]
0x1409bff92: movups xmm2, xmmword ptr [rbp + 0x290]
0x1409bff99: movups xmm3, xmmword ptr [rbp + 0x2a0]
0x1409bffa0: movups xmmword ptr [rbp + 0x628], xmm0
0x1409bffa7: movups xmmword ptr [rbp + 0x638], xmm1
0x1409bffae: movups xmmword ptr [rbp + 0x648], xmm2
0x1409bffb5: movups xmmword ptr [rbp + 0x658], xmm3
0x1409bffbc: movups xmm0, xmmword ptr [rbp + 0x2b0]
0x1409bffc3: movups xmmword ptr [rbp + 0x668], xmm0
0x1409bffca: movups xmm0, xmmword ptr [rbp + 0x2c0]
0x1409bffd1: movups xmmword ptr [rbp + 0x678], xmm0
0x1409bffd8: mov rdx, qword ptr [rbp + 0x218]
0x1409bffdf: mov qword ptr [rbp + 0x608], rdx
0x1409bffe6: movups xmm0, xmmword ptr [rbp + 0x208]
0x1409bffed: movups xmmword ptr [rbp + 0x5f8], xmm0
0x1409bfff4: mov qword ptr [rbp + 0x690], rcx
0x1409bfffb: mov qword ptr [rbp + 0x688], rax
0x1409c0002: movaps xmm0, xmmword ptr [rip + 0x1f98fd7]
0x1409c0009: movaps xmmword ptr [rbp + 0x610], xmm0
0x1409c0010: lea rcx, [rbp + 0x9a0]
0x1409c0017: lea rdx, [rbp + 0x5b0]
0x1409c001e: call 0x1409dd4d0
0x1409c0023: cmp byte ptr [rbp + 0x2f6], 8
0x1409c002a: je 0x1409c0038
0x1409c002c: lea rcx, [rbp + 0x2e0]
0x1409c0033: call 0x140be24b0
0x1409c0038: mov qword ptr [rsp + 0x30], rdi
0x1409c003d: mov qword ptr [rsp + 0x28], r14
0x1409c0042: mov qword ptr [rsp + 0x20], 0x1a
0x1409c004b: lea rdx, [rip + 0x20ffc0c]
0x1409c0052: lea r9, [rip + 0x20ffc16]
0x1409c0059: mov r8d, 0x11
0x1409c005f: mov rcx, rsi
0x1409c0062: call 0x1416032a0
0x1409c0067: lea rdi, [rbp + 0x1c0]
0x1409c006e: movabs r8, 0x1863d10d6efe0b4e
0x1409c0078: lea r9, [rbp + 0xb60]
0x1409c007f: mov rcx, rdi
0x1409c0082: mov rdx, rsi
0x1409c0085: call 0x14096dac0
0x1409c008a: movabs r8, 0x8f7b94e37966c87b
0x1409c0094: lea r9, [rbp + 0xb30]
0x1409c009b: mov rcx, rsi
0x1409c009e: mov rdx, rdi
0x1409c00a1: call 0x14096dac0
0x1409c00a6: mov rax, qword ptr [rbp + 0xc10]
0x1409c00ad: mov qword ptr [rbp + 0x9a0], rax
0x1409c00b4: mov dword ptr [rbp + 0x1c0], 2
0x1409c00be: mov qword ptr [rsp + 0x20], rdi
0x1409c00c3: lea rcx, [rbp + 0x9a0]
0x1409c00ca: movabs r8, 0x2a333c3a28028230
0x1409c00d4: mov rdx, rsi
0x1409c00d7: mov r9b, 0x19
0x1409c00da: call 0x1408ee240
0x1409c00df: lea rsi, [rbp + 0x1c0]
0x1409c00e6: lea rdi, [rbp + 0x5b0]
0x1409c00ed: mov r8d, 0x190
0x1409c00f3: mov rcx, rsi
0x1409c00f6: mov rdx, rdi
0x1409c00f9: call 0x142923c00
0x1409c00fe: mov rax, qword ptr [rbp + 0xc18]
0x1409c0105: mov qword ptr [rbp + 0x9a0], rax
0x1409c010c: mov dword ptr [rbp + 0x5b0], 2
0x1409c0116: mov qword ptr [rsp + 0x20], rdi
0x1409c011b: lea rcx, [rbp + 0x9a0]
0x1409c0122: movabs r8, 0x4175185826c09d0
0x1409c012c: mov rdx, rsi
0x1409c012f: mov r9b, 0x19
0x1409c0132: call 0x1408ee240
0x1409c0137: lea rcx, [rbp + 0x5b0]
0x1409c013e: lea rdx, [rbp + 0x1c0]
0x1409c0145: mov r8d, 0x190
0x1409c014b: call 0x142923c00
0x1409c0150: cmp byte ptr [rbp + 0x6e6], 8
0x1409c0157: je 0x1409c0182
0x1409c0159: lea rcx, [rbp + 0x6c0]
0x1409c0160: mov edx, 0xe8
0x1409c0165: add rdx, qword ptr [rbp + 0x6e8]
0x1409c016c: movabs r8, 0xa9a109b760eed902
0x1409c0176: mov r9d, dword ptr [rbp + 0xb98]
0x1409c017d: call 0x14096b570
0x1409c0182: mov r8d, 0x190
0x1409c0188: mov rcx, rsi
0x1409c018b: mov rdx, rdi
0x1409c018e: call 0x142923c00
0x1409c0193: mov rax, qword ptr [rbp + 0xc30]
0x1409c019a: mov qword ptr [rbp + 0x9a0], rax
0x1409c01a1: mov dword ptr [rbp + 0x5b0], 2
0x1409c01ab: mov qword ptr [rsp + 0x20], rdi
0x1409c01b0: lea rcx, [rbp + 0x9a0]
0x1409c01b7: movabs r8, 0xb8a74ccd835a6be3
0x1409c01c1: mov rdx, rsi
0x1409c01c4: mov r9b, 0x19
0x1409c01c7: call 0x1408ee240
0x1409c01cc: lea rsi, [rbp + 0x5b0]
0x1409c01d3: lea rdi, [rbp + 0x1c0]
0x1409c01da: mov r8d, 0x190
0x1409c01e0: mov rcx, rsi
0x1409c01e3: mov rdx, rdi
0x1409c01e6: call 0x142923c00
0x1409c01eb: movabs rbx, 0x579f39ab91a01865
0x1409c01f5: mov qword ptr [rbp + 0x9a0], rbx
0x1409c01fc: mov dword ptr [rbp + 0x1c0], 2
0x1409c0206: mov qword ptr [rsp + 0x20], rdi
0x1409c020b: lea rcx, [rbp + 0x9a0]
0x1409c0212: movabs r8, 0x815bf5036966978c
0x1409c021c: mov rdx, rsi
0x1409c021f: mov r9b, 0x19
0x1409c0222: call 0x1409056d0
0x1409c0227: mov r8d, 0x190
0x1409c022d: mov rcx, rdi
0x1409c0230: mov rdx, rsi
0x1409c0233: call 0x142923c00
0x1409c0238: mov byte ptr [rbp + 0x9a0], 9
0x1409c023f: mov dword ptr [rbp + 0x5b0], 2
0x1409c0249: mov qword ptr [rsp + 0x20], rsi
0x1409c024e: lea rsi, [rbp + 0x9a0]
0x1409c0255: movabs r8, 0x61b08b5e0470c5f2
0x1409c025f: mov rcx, rsi
0x1409c0262: mov rdx, rdi
0x1409c0265: mov r9b, 0x19
0x1409c0268: call 0x140905260
0x1409c026d: lea rdi, [rbp + 0x5b0]
0x1409c0274: lea rdx, [rbp + 0x1c0]
0x1409c027b: mov r8d, 0x190
0x1409c0281: mov rcx, rdi
0x1409c0284: call 0x142923c00
0x1409c0289: mov byte ptr [rbp + 0xc8], 0xa
0x1409c0290: mov dword ptr [rbp + 0x9a0], 2
0x1409c029a: mov qword ptr [rsp + 0x20], rsi
0x1409c029f: lea rcx, [rbp + 0xc8]
0x1409c02a6: movabs r8, 0x297d1e4cdae8f657
0x1409c02b0: mov rdx, rdi
0x1409c02b3: mov r9b, 0x19
0x1409c02b6: call 0x140905260
0x1409c02bb: lea rdi, [rbp + 0x9a0]
0x1409c02c2: lea rsi, [rbp + 0x5b0]
0x1409c02c9: mov r8d, 0x190
0x1409c02cf: mov rcx, rdi
0x1409c02d2: mov rdx, rsi
0x1409c02d5: call 0x142923c00
0x1409c02da: mov byte ptr [rbp + 0x1c0], 8
0x1409c02e1: mov dword ptr [rbp + 0x5b0], 2
0x1409c02eb: mov qword ptr [rsp + 0x20], rsi
0x1409c02f0: lea rsi, [rbp + 0x1c0]
0x1409c02f7: movabs r8, 0x77160113ac8dca3f
0x1409c0301: mov rcx, rsi
0x1409c0304: mov rdx, rdi
0x1409c0307: mov r9b, 0x19
0x1409c030a: call 0x140905260
0x1409c030f: lea rdi, [rbp + 0x5b0]
0x1409c0316: lea rdx, [rbp + 0x9a0]
0x1409c031d: mov r8d, 0x190
0x1409c0323: mov rcx, rdi
0x1409c0326: call 0x142923c00
0x1409c032b: mov qword ptr [rbp + 0xc8], rbx
0x1409c0332: mov dword ptr [rbp + 0x1c0], 2
0x1409c033c: mov qword ptr [rsp + 0x20], rsi
0x1409c0341: lea rcx, [rbp + 0xc8]
0x1409c0348: movabs r8, 0x815bf5036966978c
0x1409c0352: mov rdx, rdi
0x1409c0355: mov r9b, 0x19
0x1409c0358: call 0x1409056d0
0x1409c035d: lea rdx, [rbp + 0x5b0]
0x1409c0364: mov r8d, 0x190
0x1409c036a: mov rcx, rsi
0x1409c036d: call 0x142923c00
0x1409c0372: mov rcx, rdi
0x1409c0375: mov rdx, rsi
0x1409c0378: mov r8, r14
0x1409c037b: call 0x1415fc230
0x1409c0380: mov rax, qword ptr [rbp + 0xb58]
0x1409c0387: mov qword ptr [rbp + 0x9a0], rax
0x1409c038e: mov dword ptr [rbp + 0x1c0], 2
0x1409c0398: mov qword ptr [rsp + 0x20], rsi
0x1409c039d: lea rcx, [rbp + 0x9a0]
0x1409c03a4: movabs r8, 0xd9978e541ce9a95e
0x1409c03ae: mov rdx, rdi
0x1409c03b1: mov r9b, 0x32
0x1409c03b4: call 0x1408edd00
0x1409c03b9: lea rdx, [rbp + 0x5b0]
0x1409c03c0: mov r8d, 0x190
0x1409c03c6: mov rcx, rsi
0x1409c03c9: call 0x142923c00
0x1409c03ce: lea rdx, [rbp + 0xb60]
0x1409c03d5: mov rcx, rsi
0x1409c03d8: xor r8d, r8d
0x1409c03db: call 0x14096e880
0x1409c03e0: nop 
0x1409c03e1: add rsp, 0xc28
0x1409c03e8: pop rbx
0x1409c03e9: pop rdi
0x1409c03ea: pop rsi
0x1409c03eb: pop r12
0x1409c03ed: pop r13
0x1409c03ef: pop r14
0x1409c03f1: pop r15
0x1409c03f3: pop rbp
0x1409c03f4: ret 
0x1409c03f5: mov byte ptr [rbp + 0xb9f], 1
0x1409c03fc: lea rcx, [rip + 0x227a69d]
0x1409c0403: call 0x1428d80d0
0x1409c0408: jmp 0x1409c041d
0x1409c040a: mov byte ptr [rbp + 0xb9f], 1
0x1409c0411: lea rcx, [rip + 0x20e57c0]
0x1409c0418: call 0x1428d9310
0x1409c041d: ud2 
0x1409c041f: nop 
0x1409c0420: mov qword ptr [rsp + 0x10], rdx
0x1409c0425: push rbp
0x1409c0426: push r15
0x1409c0428: push r14
0x1409c042a: push r13
0x1409c042c: push r12
0x1409c042e: push rsi
0x1409c042f: push rdi
0x1409c0430: push rbx
0x1409c0431: sub rsp, 0x48
0x1409c0435: lea rbp, [rdx + 0x80]
0x1409c043c: cmp byte ptr [rbp + 0x2f6], 8
0x1409c0443: je 0x1409c0451
0x1409c0445: lea rcx, [rbp + 0x2e0]
0x1409c044c: call 0x140be24b0
0x1409c0451: nop 
0x1409c0452: add rsp, 0x48
0x1409c0456: pop rbx
0x1409c0457: pop rdi
0x1409c0458: pop rsi
0x1409c0459: pop r12
0x1409c045b: pop r13
0x1409c045d: pop r14
0x1409c045f: pop r15
0x1409c0461: pop rbp
0x1409c0462: ret 
0x1409c0463: nop word ptr cs:[rax + rax]
0x1409c0470: mov qword ptr [rsp + 0x10], rdx
0x1409c0475: push rbp
0x1409c0476: push r15
0x1409c0478: push r14
0x1409c047a: push r13
0x1409c047c: push r12
0x1409c047e: push rsi
0x1409c047f: push rdi
0x1409c0480: push rbx
0x1409c0481: sub rsp, 0x48
0x1409c0485: lea rbp, [rdx + 0x80]
0x1409c048c: lea rcx, [rbp + 0x5b0]
0x1409c0493: call 0x140896fb0
0x1409c0498: nop 
0x1409c0499: add rsp, 0x48
0x1409c049d: pop rbx
0x1409c049e: pop rdi
0x1409c049f: pop rsi
0x1409c04a0: pop r12
0x1409c04a2: pop r13
0x1409c04a4: pop r14
0x1409c04a6: pop r15
0x1409c04a8: pop rbp
0x1409c04a9: ret 
0x1409c04aa: nop word ptr [rax + rax]
0x1409c04b0: mov qword ptr [rsp + 0x10], rdx
0x1409c04b5: push rbp
0x1409c04b6: push r15
0x1409c04b8: push r14
0x1409c04ba: push r13
0x1409c04bc: push r12
0x1409c04be: push rsi
0x1409c04bf: push rdi
0x1409c04c0: push rbx
0x1409c04c1: sub rsp, 0x48
0x1409c04c5: lea rbp, [rdx + 0x80]
0x1409c04cc: lea rcx, [rbp + 0x1c0]
0x1409c04d3: call 0x140896fb0
0x1409c04d8: nop 
0x1409c04d9: add rsp, 0x48
0x1409c04dd: pop rbx
0x1409c04de: pop rdi
0x1409c04df: pop rsi
0x1409c04e0: pop r12
0x1409c04e2: pop r13
0x1409c04e4: pop r14
0x1409c04e6: pop r15
0x1409c04e8: pop rbp
0x1409c04e9: ret 
0x1409c04ea: nop word ptr [rax + rax]
0x1409c04f0: mov qword ptr [rsp + 0x10], rdx
0x1409c04f5: push rbp
0x1409c04f6: push r15
0x1409c04f8: push r14
0x1409c04fa: push r13
0x1409c04fc: push r12
0x1409c04fe: push rsi
0x1409c04ff: push rdi
0x1409c0500: push rbx
0x1409c0501: sub rsp, 0x48
0x1409c0505: lea rbp, [rdx + 0x80]
0x1409c050c: lea rcx, [rbp + 0x5b0]
0x1409c0513: call 0x140896fb0
0x1409c0518: nop 
0x1409c0519: add rsp, 0x48
0x1409c051d: pop rbx
0x1409c051e: pop rdi
0x1409c051f: pop rsi
0x1409c0520: pop r12
0x1409c0522: pop r13
0x1409c0524: pop r14
0x1409c0526: pop r15
0x1409c0528: pop rbp
0x1409c0529: ret 
0x1409c052a: nop word ptr [rax + rax]
0x1409c0530: mov qword ptr [rsp + 0x10], rdx
0x1409c0535: push rbp
0x1409c0536: push r15
0x1409c0538: push r14
0x1409c053a: push r13
0x1409c053c: push r12
0x1409c053e: push rsi
0x1409c053f: push rdi
0x1409c0540: push rbx
0x1409c0541: sub rsp, 0x48
0x1409c0545: lea rbp, [rdx + 0x80]
0x1409c054c: lea rcx, [rbp + 0x1c0]
0x1409c0553: call 0x140896fb0
0x1409c0558: nop 
0x1409c0559: add rsp, 0x48
0x1409c055d: pop rbx
0x1409c055e: pop rdi
0x1409c055f: pop rsi
0x1409c0560: pop r12
0x1409c0562: pop r13
0x1409c0564: pop r14
0x1409c0566: pop r15
0x1409c0568: pop rbp
0x1409c0569: ret 
0x1409c056a: nop word ptr [rax + rax]
0x1409c0570: mov qword ptr [rsp + 0x10], rdx
0x1409c0575: push rbp
0x1409c0576: push r15
0x1409c0578: push r14
0x1409c057a: push r13
0x1409c057c: push r12
0x1409c057e: push rsi
0x1409c057f: push rdi
0x1409c0580: push rbx
0x1409c0581: sub rsp, 0x48
0x1409c0585: lea rbp, [rdx + 0x80]
0x1409c058c: lea rcx, [rbp + 0x5b0]
0x1409c0593: call 0x140896fb0
0x1409c0598: nop 
0x1409c0599: add rsp, 0x48
0x1409c059d: pop rbx
0x1409c059e: pop rdi
0x1409c059f: pop rsi
0x1409c05a0: pop r12
0x1409c05a2: pop r13
0x1409c05a4: pop r14
0x1409c05a6: pop r15
0x1409c05a8: pop rbp
0x1409c05a9: ret 
0x1409c05aa: nop word ptr [rax + rax]
0x1409c05b0: mov qword ptr [rsp + 0x10], rdx
0x1409c05b5: push rbp
0x1409c05b6: push r15
0x1409c05b8: push r14
0x1409c05ba: push r13
0x1409c05bc: push r12
0x1409c05be: push rsi
0x1409c05bf: push rdi
0x1409c05c0: push rbx
0x1409c05c1: sub rsp, 0x48
0x1409c05c5: lea rbp, [rdx + 0x80]
0x1409c05cc: lea rcx, [rbp + 0x1c0]
0x1409c05d3: call 0x140896fb0
0x1409c05d8: nop 
0x1409c05d9: add rsp, 0x48
0x1409c05dd: pop rbx
0x1409c05de: pop rdi
0x1409c05df: pop rsi
0x1409c05e0: pop r12
0x1409c05e2: pop r13
0x1409c05e4: pop r14
0x1409c05e6: pop r15
0x1409c05e8: pop rbp
0x1409c05e9: ret 
0x1409c05ea: nop word ptr [rax + rax]
0x1409c05f0: mov qword ptr [rsp + 0x10], rdx
0x1409c05f5: push rbp
0x1409c05f6: push r15
0x1409c05f8: push r14
0x1409c05fa: push r13
0x1409c05fc: push r12
0x1409c05fe: push rsi
0x1409c05ff: push rdi
0x1409c0600: push rbx
0x1409c0601: sub rsp, 0x48
0x1409c0605: lea rbp, [rdx + 0x80]
0x1409c060c: lea rcx, [rbp + 0x5b0]
0x1409c0613: call 0x140896fb0
0x1409c0618: nop 
0x1409c0619: add rsp, 0x48
0x1409c061d: pop rbx
0x1409c061e: pop rdi
0x1409c061f: pop rsi
0x1409c0620: pop r12
0x1409c0622: pop r13
0x1409c0624: pop r14
0x1409c0626: pop r15
0x1409c0628: pop rbp
0x1409c0629: ret 
0x1409c062a: nop word ptr [rax + rax]
0x1409c0630: mov qword ptr [rsp + 0x10], rdx
0x1409c0635: push rbp
0x1409c0636: push r15
0x1409c0638: push r14
0x1409c063a: push r13
0x1409c063c: push r12
0x1409c063e: push rsi
0x1409c063f: push rdi
0x1409c0640: push rbx
0x1409c0641: sub rsp, 0x48
0x1409c0645: lea rbp, [rdx + 0x80]
0x1409c064c: lea rcx, [rbp + 0x9a0]
0x1409c0653: call 0x140896fb0
0x1409c0658: nop 
0x1409c0659: add rsp, 0x48
0x1409c065d: pop rbx
0x1409c065e: pop rdi
0x1409c065f: pop rsi
0x1409c0660: pop r12
0x1409c0662: pop r13
0x1409c0664: pop r14
0x1409c0666: pop r15
0x1409c0668: pop rbp
0x1409c0669: ret 
0x1409c066a: nop word ptr [rax + rax]
0x1409c0670: mov qword ptr [rsp + 0x10], rdx
0x1409c0675: push rbp
0x1409c0676: push r15
0x1409c0678: push r14
0x1409c067a: push r13
0x1409c067c: push r12
0x1409c067e: push rsi
0x1409c067f: push rdi
0x1409c0680: push rbx
0x1409c0681: sub rsp, 0x48
0x1409c0685: lea rbp, [rdx + 0x80]
0x1409c068c: lea rcx, [rbp + 0x5b0]
0x1409c0693: call 0x140896fb0
0x1409c0698: nop 
0x1409c0699: add rsp, 0x48
0x1409c069d: pop rbx
0x1409c069e: pop rdi
0x1409c069f: pop rsi
0x1409c06a0: pop r12
0x1409c06a2: pop r13
0x1409c06a4: pop r14
0x1409c06a6: pop r15
0x1409c06a8: pop rbp
0x1409c06a9: ret 
0x1409c06aa: nop word ptr [rax + rax]
0x1409c06b0: mov qword ptr [rsp + 0x10], rdx
0x1409c06b5: push rbp
0x1409c06b6: push r15
0x1409c06b8: push r14
0x1409c06ba: push r13
0x1409c06bc: push r12
0x1409c06be: push rsi
0x1409c06bf: push rdi
0x1409c06c0: push rbx
0x1409c06c1: sub rsp, 0x48
0x1409c06c5: lea rbp, [rdx + 0x80]
0x1409c06cc: lea rcx, [rbp + 0x5b0]
0x1409c06d3: call 0x140896fb0
0x1409c06d8: nop 
0x1409c06d9: add rsp, 0x48
0x1409c06dd: pop rbx
0x1409c06de: pop rdi
0x1409c06df: pop rsi
0x1409c06e0: pop r12
0x1409c06e2: pop r13
0x1409c06e4: pop r14
0x1409c06e6: pop r15
0x1409c06e8: pop rbp
0x1409c06e9: ret 
0x1409c06ea: nop word ptr [rax + rax]
0x1409c06f0: mov qword ptr [rsp + 0x10], rdx
0x1409c06f5: push rbp
0x1409c06f6: push r15
0x1409c06f8: push r14
0x1409c06fa: push r13
0x1409c06fc: push r12
0x1409c06fe: push rsi
0x1409c06ff: push rdi
0x1409c0700: push rbx
0x1409c0701: sub rsp, 0x48
0x1409c0705: lea rbp, [rdx + 0x80]
0x1409c070c: cmp byte ptr [rbp + 0xb9f], 0
0x1409c0713: jne 0x1409c0726
0x1409c0715: add rsp, 0x48
0x1409c0719: pop rbx
0x1409c071a: pop rdi
0x1409c071b: pop rsi
0x1409c071c: pop r12
0x1409c071e: pop r13
0x1409c0720: pop r14
0x1409c0722: pop r15
0x1409c0724: pop rbp
0x1409c0725: ret 
0x1409c0726: lea rcx, [rbp + 0x1c0]
0x1409c072d: call 0x1408923c0
0x1409c0732: jmp 0x1409c0715
0x1409c0734: nop word ptr cs:[rax + rax]
0x1409c0740: mov qword ptr [rsp + 0x10], rdx
0x1409c0745: push rbp
0x1409c0746: push r15
0x1409c0748: push r14
0x1409c074a: push r13
0x1409c074c: push r12
0x1409c074e: push rsi
0x1409c074f: push rdi
0x1409c0750: push rbx
0x1409c0751: sub rsp, 0x48
0x1409c0755: lea rbp, [rdx + 0x80]
0x1409c075c: mov byte ptr [rbp + 0xb9f], 1
0x1409c0763: lea rcx, [rbp + 0x9a0]
0x1409c076a: call 0x1408935c0
0x1409c076f: mov byte ptr [rbp + 0xb9f], 1
0x1409c0776: add rsp, 0x48
0x1409c077a: pop rbx
0x1409c077b: pop rdi
0x1409c077c: pop rsi
0x1409c077d: pop r12
0x1409c077f: pop r13
0x1409c0781: pop r14
0x1409c0783: pop r15
0x1409c0785: pop rbp
0x1409c0786: ret 
0x1409c0787: nop word ptr [rax + rax]
0x1409c0790: mov qword ptr [rsp + 0x10], rdx
0x1409c0795: push rbp
0x1409c0796: push r15
0x1409c0798: push r14
0x1409c079a: push r13
0x1409c079c: push r12
0x1409c079e: push rsi
0x1409c079f: push rdi
0x1409c07a0: push rbx
0x1409c07a1: sub rsp, 0x48
0x1409c07a5: lea rbp, [rdx + 0x80]
0x1409c07ac: mov byte ptr [rbp + 0xb9f], 1
0x1409c07b3: lea rcx, [rbp + 0x5b0]
0x1409c07ba: call 0x1408935c0
0x1409c07bf: mov byte ptr [rbp + 0xb9f], 1
0x1409c07c6: add rsp, 0x48
0x1409c07ca: pop rbx
0x1409c07cb: pop rdi
0x1409c07cc: pop rsi
0x1409c07cd: pop r12
0x1409c07cf: pop r13
0x1409c07d1: pop r14
0x1409c07d3: pop r15
0x1409c07d5: pop rbp
0x1409c07d6: ret 
0x1409c07d7: nop word ptr [rax + rax]
0x1409c07e0: mov qword ptr [rsp + 0x10], rdx
0x1409c07e5: push rbp
0x1409c07e6: push r15
0x1409c07e8: push r14
0x1409c07ea: push r13
0x1409c07ec: push r12
0x1409c07ee: push rsi
0x1409c07ef: push rdi
0x1409c07f0: push rbx
0x1409c07f1: sub rsp, 0x48
0x1409c07f5: lea rbp, [rdx + 0x80]
0x1409c07fc: mov byte ptr [rbp + 0xb9f], 1
0x1409c0803: lea rcx, [rbp + 0xc8]
0x1409c080a: call 0x1408935c0
0x1409c080f: mov byte ptr [rbp + 0xb9f], 1
0x1409c0816: add rsp, 0x48
0x1409c081a: pop rbx
0x1409c081b: pop rdi
0x1409c081c: pop rsi
0x1409c081d: pop r12
0x1409c081f: pop r13
0x1409c0821: pop r14
0x1409c0823: pop r15
0x1409c0825: pop rbp
0x1409c0826: ret 
0x1409c0827: nop word ptr [rax + rax]
0x1409c0830: mov qword ptr [rsp + 0x10], rdx
0x1409c0835: push rbp
0x1409c0836: push r15
0x1409c0838: push r14
0x1409c083a: push r13
0x1409c083c: push r12
0x1409c083e: push rsi
0x1409c083f: push rdi
0x1409c0840: push rbx
0x1409c0841: sub rsp, 0x48
0x1409c0845: lea rbp, [rdx + 0x80]
0x1409c084c: mov byte ptr [rbp + 0xb9f], 1
0x1409c0853: lea rcx, [rbp + 0x5b0]
0x1409c085a: call 0x1408935c0
0x1409c085f: mov byte ptr [rbp + 0xb9f], 1
0x1409c0866: add rsp, 0x48
0x1409c086a: pop rbx
0x1409c086b: pop rdi
0x1409c086c: pop rsi
0x1409c086d: pop r12
0x1409c086f: pop r13
0x1409c0871: pop r14
0x1409c0873: pop r15
0x1409c0875: pop rbp
0x1409c0876: ret 
0x1409c0877: nop word ptr [rax + rax]
0x1409c0880: mov qword ptr [rsp + 0x10], rdx
0x1409c0885: push rbp
0x1409c0886: push r15
0x1409c0888: push r14
0x1409c088a: push r13
0x1409c088c: push r12
0x1409c088e: push rsi
0x1409c088f: push rdi
0x1409c0890: push rbx
0x1409c0891: sub rsp, 0x48
0x1409c0895: lea rbp, [rdx + 0x80]
0x1409c089c: mov byte ptr [rbp + 0xb9f], 1
0x1409c08a3: lea rcx, [rbp - 0x30]
0x1409c08a7: call 0x1408935c0
0x1409c08ac: mov byte ptr [rbp + 0xb9f], 1
0x1409c08b3: add rsp, 0x48
0x1409c08b7: pop rbx
0x1409c08b8: pop rdi
0x1409c08b9: pop rsi
0x1409c08ba: pop r12
0x1409c08bc: pop r13
0x1409c08be: pop r14
0x1409c08c0: pop r15
0x1409c08c2: pop rbp
0x1409c08c3: ret 
