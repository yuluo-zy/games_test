1403bf820: push r15
1403bf822: push r14
1403bf824: push r13
1403bf826: push r12
1403bf828: push rsi
1403bf829: push rdi
1403bf82a: push rbp
1403bf82b: push rbx
1403bf82c: sub rsp, 0x2c8
1403bf833: mov edi, 1
1403bf838: lock xadd dword ptr [rdx + 0x5b0], edi
1403bf840: test byte ptr [rcx], 1
1403bf843: je 0x1403c0316
1403bf849: mov r9, qword ptr [rcx + 8]
1403bf84d: mov rax, qword ptr [rdx + 0x218]
1403bf854: cmp rax, r9
1403bf857: jbe 0x1403c007a
1403bf85d: mov r8, qword ptr [rdx + 0x210]
1403bf864: mov r10, qword ptr [r8 + r9*8]
1403bf868: test r10, r10
1403bf86b: je 0x1403c007a
1403bf871: mov r9, qword ptr [rdx + 0x1e0]
1403bf878: not r10
1403bf87b: lea r12, [r10 + r10*2]
1403bf87f: shl r12, 5
1403bf883: cmp qword ptr [r9 + r12 + 0x20], 0
1403bf889: je 0x1403c007a
1403bf88f: mov r10, qword ptr [rcx + 0x10]
1403bf893: cmp rax, r10
1403bf896: jbe 0x1403c0120
1403bf89c: mov rax, qword ptr [r8 + r10*8]
1403bf8a0: test rax, rax
1403bf8a3: je 0x1403c0120
1403bf8a9: not rax
1403bf8ac: lea r8, [rax + rax*2]
1403bf8b0: shl r8, 5
1403bf8b4: cmp qword ptr [r9 + r8 + 0x20], 0
1403bf8ba: je 0x1403c0120
1403bf8c0: add r12, r9
1403bf8c3: mov rbx, qword ptr [r12 + 0x10]
1403bf8c8: mov eax, dword ptr [rcx + 0x1a8]
1403bf8ce: add r8, r9
1403bf8d1: mov r14, qword ptr [r8 + 0x10]
1403bf8d5: mov r9d, dword ptr [rdx + 0x5b0]
1403bf8dc: mov r9, qword ptr [rcx + 0x18]
1403bf8e0: cmp qword ptr [rdx + 0x218], r9
1403bf8e7: jbe 0x1403c0141
1403bf8ed: mov r10, qword ptr [rdx + 0x210]
1403bf8f4: mov r9, qword ptr [r10 + r9*8]
1403bf8f8: test r9, r9
1403bf8fb: je 0x1403c0141
1403bf901: mov rsi, qword ptr [rdx + 0x1e0]
1403bf908: not r9
1403bf90b: lea r9, [r9 + r9*2]
1403bf90f: shl r9, 5
1403bf913: cmp qword ptr [rsi + r9 + 0x20], 0
1403bf919: je 0x1403c0141
1403bf91f: add rsi, r9
1403bf922: mov r13, qword ptr [rsi + 0x10]
1403bf926: mov r9d, dword ptr [rdx + 0x5b0]
1403bf92d: mov r9, qword ptr [rcx + 0x20]
1403bf931: cmp qword ptr [rdx + 0x218], r9
1403bf938: jbe 0x1403c0162
1403bf93e: mov r10, qword ptr [rdx + 0x210]
1403bf945: mov r9, qword ptr [r10 + r9*8]
1403bf949: test r9, r9
1403bf94c: je 0x1403c0162
1403bf952: mov r11, qword ptr [rdx + 0x1e0]
1403bf959: not r9
1403bf95c: lea r9, [r9 + r9*2]
1403bf960: shl r9, 5
1403bf964: cmp qword ptr [r11 + r9 + 0x20], 0
1403bf96a: je 0x1403c0162
1403bf970: add r11, r9
1403bf973: mov r15, qword ptr [r11 + 0x10]
1403bf977: mov r9d, dword ptr [rdx + 0x5b0]
1403bf97e: mov r9, qword ptr [rcx + 0x28]
1403bf982: cmp qword ptr [rdx + 0x218], r9
1403bf989: jbe 0x1403c0172
1403bf98f: mov r10, qword ptr [rdx + 0x210]
1403bf996: mov r9, qword ptr [r10 + r9*8]
1403bf99a: test r9, r9
1403bf99d: je 0x1403c0172
1403bf9a3: mov r10, qword ptr [rdx + 0x1e0]
1403bf9aa: not r9
1403bf9ad: lea r9, [r9 + r9*2]
1403bf9b1: shl r9, 5
1403bf9b5: cmp qword ptr [r10 + r9 + 0x20], 0
1403bf9bb: je 0x1403c0172
1403bf9c1: add r10, r9
1403bf9c4: mov qword ptr [rsp + 0x2c0], r10
1403bf9cc: mov r9, qword ptr [r10 + 0x10]
1403bf9d0: mov qword ptr [rsp + 0x2b8], r9
1403bf9d8: mov r9d, dword ptr [rdx + 0x5b0]
1403bf9df: mov r9, qword ptr [rcx + 0x30]
1403bf9e3: cmp qword ptr [rdx + 0x218], r9
1403bf9ea: jbe 0x1403c0193
1403bf9f0: mov r10, qword ptr [rdx + 0x210]
1403bf9f7: mov r9, qword ptr [r10 + r9*8]
1403bf9fb: test r9, r9
1403bf9fe: je 0x1403c0193
1403bfa04: mov rbp, qword ptr [rdx + 0x1e0]
1403bfa0b: not r9
1403bfa0e: lea r9, [r9 + r9*2]
1403bfa12: shl r9, 5
1403bfa16: cmp qword ptr [rbp + r9 + 0x20], 0
1403bfa1c: je 0x1403c0193
1403bfa22: mov qword ptr [rsp + 0x2a8], r15
1403bfa2a: mov qword ptr [rsp + 0x2b0], r11
1403bfa32: add rbp, r9
1403bfa35: mov r9, qword ptr [rbp + 0x10]
1403bfa39: mov qword ptr [rsp + 0x2a0], r9
1403bfa41: mov r9d, dword ptr [rdx + 0x5b0]
1403bfa48: mov r9, qword ptr [rcx + 0x38]
1403bfa4c: cmp qword ptr [rdx + 0x218], r9
1403bfa53: jbe 0x1403c01b4
1403bfa59: mov r10, qword ptr [rdx + 0x210]
1403bfa60: mov r9, qword ptr [r10 + r9*8]
1403bfa64: test r9, r9
1403bfa67: je 0x1403c01b4
1403bfa6d: mov r11, qword ptr [rdx + 0x1e0]
1403bfa74: not r9
1403bfa77: lea r9, [r9 + r9*2]
1403bfa7b: shl r9, 5
1403bfa7f: cmp qword ptr [r11 + r9 + 0x20], 0
1403bfa85: je 0x1403c01b4
1403bfa8b: add r11, r9
1403bfa8e: mov r9, qword ptr [r11 + 0x10]
1403bfa92: mov qword ptr [rsp + 0x298], r9
1403bfa9a: mov r9d, dword ptr [rdx + 0x5b0]
1403bfaa1: mov r9, qword ptr [rcx + 0x40]
1403bfaa5: cmp qword ptr [rdx + 0x218], r9
1403bfaac: jbe 0x1403c01d5
1403bfab2: mov r10, qword ptr [rdx + 0x210]
1403bfab9: mov r9, qword ptr [r10 + r9*8]
1403bfabd: test r9, r9
1403bfac0: je 0x1403c01d5
1403bfac6: mov r15, qword ptr [rdx + 0x1e0]
1403bfacd: not r9
1403bfad0: lea r9, [r9 + r9*2]
1403bfad4: shl r9, 5
1403bfad8: cmp qword ptr [r15 + r9 + 0x20], 0
1403bfade: je 0x1403c01d5
1403bfae4: add r15, r9
1403bfae7: mov r9, qword ptr [r15 + 0x10]
1403bfaeb: mov qword ptr [rsp + 0x290], r9
1403bfaf3: mov r9d, dword ptr [rdx + 0x5b0]
1403bfafa: mov r9, qword ptr [rcx + 0x48]
1403bfafe: cmp qword ptr [rdx + 0x218], r9
1403bfb05: jbe 0x1403c01e5
1403bfb0b: mov r10, qword ptr [rdx + 0x210]
1403bfb12: mov r10, qword ptr [r10 + r9*8]
1403bfb16: test r10, r10
1403bfb19: je 0x1403c01e5
1403bfb1f: mov r9, qword ptr [rdx + 0x1e0]
1403bfb26: not r10
1403bfb29: lea r10, [r10 + r10*2]
1403bfb2d: shl r10, 5
1403bfb31: cmp qword ptr [r9 + r10 + 0x20], 0
1403bfb37: je 0x1403c01e5
1403bfb3d: mov qword ptr [rsp + 0x278], r13
1403bfb45: mov qword ptr [rsp + 0x280], rbx
1403bfb4d: mov qword ptr [rsp + 0x288], rsi
1403bfb55: add r9, r10
1403bfb58: mov r10, qword ptr [r9 + 0x10]
1403bfb5c: mov qword ptr [rsp + 0x270], r10
1403bfb64: mov r10d, dword ptr [rdx + 0x5b0]
1403bfb6b: mov r10, qword ptr [rcx + 0x50]
1403bfb6f: cmp qword ptr [rdx + 0x218], r10
1403bfb76: jbe 0x1403c0206
1403bfb7c: mov rsi, qword ptr [rdx + 0x210]
1403bfb83: mov r10, qword ptr [rsi + r10*8]
1403bfb87: test r10, r10
1403bfb8a: je 0x1403c0206
1403bfb90: mov rbx, qword ptr [rdx + 0x1e0]
1403bfb97: not r10
1403bfb9a: lea r10, [r10 + r10*2]
1403bfb9e: shl r10, 5
1403bfba2: cmp qword ptr [rbx + r10 + 0x20], 0
1403bfba8: je 0x1403c0206
1403bfbae: mov qword ptr [rsp + 0x268], r14
1403bfbb6: add rbx, r10
1403bfbb9: mov r10, qword ptr [rbx + 0x10]
1403bfbbd: mov qword ptr [rsp + 0x260], r10
1403bfbc5: mov r10d, dword ptr [rdx + 0x5b0]
1403bfbcc: mov r10, qword ptr [rcx + 0x58]
1403bfbd0: cmp qword ptr [rdx + 0x218], r10
1403bfbd7: jbe 0x1403c0224
1403bfbdd: mov rsi, qword ptr [rdx + 0x210]
1403bfbe4: mov r10, qword ptr [rsi + r10*8]
1403bfbe8: test r10, r10
1403bfbeb: je 0x1403c0224
1403bfbf1: mov r13, qword ptr [rdx + 0x1e0]
1403bfbf8: not r10
1403bfbfb: lea r10, [r10 + r10*2]
1403bfbff: shl r10, 5
1403bfc03: cmp qword ptr [r13 + r10 + 0x20], 0
1403bfc09: je 0x1403c0224
1403bfc0f: add r13, r10
1403bfc12: mov r10, qword ptr [r13 + 0x10]
1403bfc16: mov qword ptr [rsp + 0x258], r10
1403bfc1e: mov r10d, dword ptr [rdx + 0x5b0]
1403bfc25: mov qword ptr [rsp + 0xa0], rcx
1403bfc2d: mov r10, qword ptr [rcx + 0x60]
1403bfc31: cmp qword ptr [rdx + 0x218], r10
1403bfc38: jbe 0x1403c0242
1403bfc3e: mov rsi, qword ptr [rdx + 0x210]
1403bfc45: mov r14, qword ptr [rsi + r10*8]
1403bfc49: test r14, r14
1403bfc4c: je 0x1403c0242
1403bfc52: mov r10, qword ptr [rdx + 0x1e0]
1403bfc59: not r14
1403bfc5c: lea r14, [r14 + r14*2]
1403bfc60: shl r14, 5
1403bfc64: cmp qword ptr [r10 + r14 + 0x20], 0
1403bfc6a: je 0x1403c0242
1403bfc70: add r10, r14
1403bfc73: mov rcx, qword ptr [r10 + 0x10]
1403bfc77: mov qword ptr [rsp + 0x250], rcx
1403bfc7f: mov esi, dword ptr [rdx + 0x5b0]
1403bfc85: mov rcx, qword ptr [rsp + 0xa0]
1403bfc8d: mov rsi, qword ptr [rcx + 0x68]
1403bfc91: cmp qword ptr [rdx + 0x218], rsi
1403bfc98: jbe 0x1403c0268
1403bfc9e: mov r14, qword ptr [rdx + 0x210]
1403bfca5: mov rsi, qword ptr [r14 + rsi*8]
1403bfca9: test rsi, rsi
1403bfcac: je 0x1403c0268
1403bfcb2: mov r14, qword ptr [rdx + 0x1e0]
1403bfcb9: not rsi
1403bfcbc: lea rsi, [rsi + rsi*2]
1403bfcc0: shl rsi, 5
1403bfcc4: cmp qword ptr [r14 + rsi + 0x20], 0
1403bfcca: je 0x1403c0268
1403bfcd0: add r14, rsi
1403bfcd3: mov rcx, qword ptr [r14 + 0x10]
1403bfcd7: mov qword ptr [rsp + 0x248], rcx
1403bfcdf: mov edx, dword ptr [rdx + 0x5b0]
1403bfce5: mov rcx, qword ptr [rsp + 0x280]
1403bfced: mov qword ptr [rsp + 0xa8], rcx
1403bfcf5: lea rdx, [r12 + 0x5c]
1403bfcfa: add r12, 0x58
1403bfcfe: mov qword ptr [rsp + 0xb0], r12
1403bfd06: mov qword ptr [rsp + 0xb8], rdx
1403bfd0e: mov dword ptr [rsp + 0xc0], eax
1403bfd15: mov dword ptr [rsp + 0xc4], edi
1403bfd1c: mov rcx, qword ptr [rsp + 0x268]
1403bfd24: mov qword ptr [rsp + 0xc8], rcx
1403bfd2c: lea rcx, [r8 + 0x5c]
1403bfd30: add r8, 0x58
1403bfd34: mov qword ptr [rsp + 0xd0], r8
1403bfd3c: mov qword ptr [rsp + 0xd8], rcx
1403bfd44: mov dword ptr [rsp + 0xe0], eax
1403bfd4b: mov dword ptr [rsp + 0xe4], edi
1403bfd52: mov rcx, qword ptr [rsp + 0x278]
1403bfd5a: mov qword ptr [rsp + 0xe8], rcx
1403bfd62: mov rdx, qword ptr [rsp + 0x288]
1403bfd6a: lea rcx, [rdx + 0x5c]
1403bfd6e: add rdx, 0x58
1403bfd72: mov qword ptr [rsp + 0xf0], rdx
1403bfd7a: mov qword ptr [rsp + 0xf8], rcx
1403bfd82: mov dword ptr [rsp + 0x100], eax
1403bfd89: mov dword ptr [rsp + 0x104], edi
1403bfd90: mov rcx, qword ptr [rsp + 0x2a8]
1403bfd98: mov qword ptr [rsp + 0x108], rcx
1403bfda0: mov rdx, qword ptr [rsp + 0x2b0]
1403bfda8: lea rcx, [rdx + 0x5c]
1403bfdac: add rdx, 0x58
1403bfdb0: mov qword ptr [rsp + 0x110], rdx
1403bfdb8: mov qword ptr [rsp + 0x118], rcx
1403bfdc0: mov dword ptr [rsp + 0x120], eax
1403bfdc7: mov dword ptr [rsp + 0x124], edi
1403bfdce: mov rcx, qword ptr [rsp + 0x2b8]
1403bfdd6: mov qword ptr [rsp + 0x128], rcx
1403bfdde: mov rdx, qword ptr [rsp + 0x2c0]
1403bfde6: lea rcx, [rdx + 0x5c]
1403bfdea: add rdx, 0x58
1403bfdee: mov qword ptr [rsp + 0x130], rdx
1403bfdf6: mov qword ptr [rsp + 0x138], rcx
1403bfdfe: mov dword ptr [rsp + 0x140], eax
1403bfe05: mov dword ptr [rsp + 0x144], edi
1403bfe0c: mov rcx, qword ptr [rsp + 0x2a0]
1403bfe14: mov qword ptr [rsp + 0x148], rcx
1403bfe1c: lea rcx, [rbp + 0x5c]
1403bfe20: add rbp, 0x58
1403bfe24: mov qword ptr [rsp + 0x150], rbp
1403bfe2c: mov qword ptr [rsp + 0x158], rcx
1403bfe34: mov dword ptr [rsp + 0x160], eax
1403bfe3b: mov dword ptr [rsp + 0x164], edi
1403bfe42: mov rcx, qword ptr [rsp + 0x298]
1403bfe4a: mov qword ptr [rsp + 0x168], rcx
1403bfe52: lea rcx, [r11 + 0x5c]
1403bfe56: add r11, 0x58
1403bfe5a: mov qword ptr [rsp + 0x170], r11
1403bfe62: mov qword ptr [rsp + 0x178], rcx
1403bfe6a: mov dword ptr [rsp + 0x180], eax
1403bfe71: mov dword ptr [rsp + 0x184], edi
1403bfe78: mov rcx, qword ptr [rsp + 0x290]
1403bfe80: mov qword ptr [rsp + 0x188], rcx
1403bfe88: lea rcx, [r15 + 0x5c]
1403bfe8c: add r15, 0x58
1403bfe90: mov qword ptr [rsp + 0x190], r15
1403bfe98: mov qword ptr [rsp + 0x198], rcx
1403bfea0: lea rcx, [r10 + 0x5c]
1403bfea4: add r10, 0x58
1403bfea8: mov dword ptr [rsp + 0x1a0], eax
1403bfeaf: mov dword ptr [rsp + 0x1a4], edi
1403bfeb6: mov rdx, qword ptr [rsp + 0x270]
1403bfebe: mov qword ptr [rsp + 0x1a8], rdx
1403bfec6: lea rdx, [r9 + 0x5c]
1403bfeca: add r9, 0x58
1403bfece: mov qword ptr [rsp + 0x1b0], r9
1403bfed6: lea rsi, [r13 + 0x5c]
1403bfeda: add r13, 0x58
1403bfede: mov qword ptr [rsp + 0x1b8], rdx
1403bfee6: lea r8, [rbx + 0x5c]
1403bfeea: add rbx, 0x58
1403bfeee: mov dword ptr [rsp + 0x1c0], eax
1403bfef5: mov dword ptr [rsp + 0x1c4], edi
1403bfefc: mov rdx, qword ptr [rsp + 0x260]
1403bff04: mov qword ptr [rsp + 0x1c8], rdx
1403bff0c: lea r15, [r14 + 0x58]
1403bff10: add r14, 0x5c
1403bff14: mov qword ptr [rsp + 0x1d0], rbx
1403bff1c: lea rdx, [rsp + 0xc8]
1403bff24: mov qword ptr [rsp + 0x1d8], r8
1403bff2c: lea r8, [rsp + 0xe8]
1403bff34: mov dword ptr [rsp + 0x1e0], eax
1403bff3b: mov dword ptr [rsp + 0x1e4], edi
1403bff42: mov r9, qword ptr [rsp + 0x258]
1403bff4a: mov qword ptr [rsp + 0x1e8], r9
1403bff52: lea r9, [rsp + 0x108]
1403bff5a: mov qword ptr [rsp + 0x1f0], r13
1403bff62: lea r11, [rsp + 0x128]
1403bff6a: mov qword ptr [rsp + 0x1f8], rsi
1403bff72: lea rbx, [rsp + 0x148]
1403bff7a: mov dword ptr [rsp + 0x200], eax
1403bff81: mov dword ptr [rsp + 0x204], edi
1403bff88: mov rsi, qword ptr [rsp + 0x250]
1403bff90: mov qword ptr [rsp + 0x208], rsi
1403bff98: lea rsi, [rsp + 0x168]
1403bffa0: mov qword ptr [rsp + 0x210], r10
1403bffa8: lea r10, [rsp + 0x188]
1403bffb0: mov qword ptr [rsp + 0x218], rcx
1403bffb8: lea rcx, [rsp + 0x1a8]
1403bffc0: mov dword ptr [rsp + 0x220], eax
1403bffc7: mov dword ptr [rsp + 0x224], edi
1403bffce: mov r12, qword ptr [rsp + 0x248]
1403bffd6: mov qword ptr [rsp + 0x228], r12
1403bffde: lea r12, [rsp + 0x1c8]
1403bffe6: mov qword ptr [rsp + 0x230], r15
1403bffee: lea r15, [rsp + 0x1e8]
1403bfff6: mov qword ptr [rsp + 0x238], r14
1403bfffe: lea r14, [rsp + 0x208]
1403c0006: mov dword ptr [rsp + 0x240], eax
1403c000d: lea rax, [rsp + 0x228]
1403c0015: mov dword ptr [rsp + 0x244], edi
1403c001c: mov qword ptr [rsp + 0x60], rax
1403c0021: mov qword ptr [rsp + 0x58], r14
1403c0026: mov qword ptr [rsp + 0x50], r15
1403c002b: mov qword ptr [rsp + 0x48], r12
1403c0030: mov qword ptr [rsp + 0x40], rcx
1403c0035: mov qword ptr [rsp + 0x38], r10
1403c003a: mov qword ptr [rsp + 0x30], rsi
1403c003f: mov qword ptr [rsp + 0x28], rbx
1403c0044: mov qword ptr [rsp + 0x20], r11
1403c0049: lea rcx, [rsp + 0xa8]
1403c0051: call 0x1402b1cb0
1403c0056: mov rax, qword ptr [rsp + 0xa0]
1403c005e: mov dword ptr [rax + 0x1a8], edi
1403c0064: xor eax, eax
1403c0066: add rsp, 0x2c8
1403c006d: pop rbx
1403c006e: pop rbp
1403c006f: pop rdi
1403c0070: pop rsi
1403c0071: pop r12
1403c0073: pop r13
1403c0075: pop r14
1403c0077: pop r15
1403c0079: ret 
1403c007a: add rcx, 0x110
1403c0081: lea rax, [rip + 0x260dbb0]
1403c0088: mov qword ptr [rsp + 0x70], rax
1403c008d: mov qword ptr [rsp + 0x78], 0x2c
1403c0096: mov qword ptr [rsp + 0x80], rcx
1403c009e: lea rax, [rip - 0x3845d5]
1403c00a5: mov qword ptr [rsp + 0x88], rax
1403c00ad: lea rax, [rsp + 0x70]
1403c00b2: mov qword ptr [rsp + 0x90], rax
1403c00ba: lea rax, [rip - 0x2b6231]
1403c00c1: mov qword ptr [rsp + 0x98], rax
1403c00c9: lea rax, [rip + 0x260dac8]
1403c00d0: mov qword ptr [rsp + 0xa8], rax
1403c00d8: mov qword ptr [rsp + 0xb0], 2
1403c00e4: mov qword ptr [rsp + 0xc8], 0
1403c00f0: lea rax, [rsp + 0x80]
1403c00f8: mov qword ptr [rsp + 0xb8], rax
1403c0100: mov qword ptr [rsp + 0xc0], 2
1403c010c: lea rdx, [rip + 0x260db0d]
1403c0113: lea rcx, [rsp + 0xa8]
1403c011b: call 0x1428d9390
1403c0120: add rcx, 0x110
1403c0127: lea rax, [rip + 0x260dcb8]
1403c012e: mov qword ptr [rsp + 0x70], rax
1403c0133: mov qword ptr [rsp + 0x78], 0x3d
1403c013c: jmp 0x1403c028c
1403c0141: add rcx, 0x110
1403c0148: lea rax, [rip + 0x260dd4d]
1403c014f: mov qword ptr [rsp + 0x70], rax
1403c0154: mov qword ptr [rsp + 0x78], 0x39
1403c015d: jmp 0x1403c028c
1403c0162: add rcx, 0x110
1403c0169: lea rax, [rip + 0x260dbe0]
1403c0170: jmp 0x1403c0180
1403c0172: add rcx, 0x110
1403c0179: lea rax, [rip + 0x260db96]
1403c0180: mov qword ptr [rsp + 0x70], rax
1403c0185: mov qword ptr [rsp + 0x78], 0x3a
1403c018e: jmp 0x1403c028c
1403c0193: add rcx, 0x110
1403c019a: lea rax, [rip + 0x260db0f]
1403c01a1: mov qword ptr [rsp + 0x70], rax
1403c01a6: mov qword ptr [rsp + 0x78], 0x37
1403c01af: jmp 0x1403c028c
1403c01b4: add rcx, 0x110
1403c01bb: lea rax, [rip + 0x260dc8f]
1403c01c2: mov qword ptr [rsp + 0x70], rax
1403c01c7: mov qword ptr [rsp + 0x78], 0x4b
1403c01d0: jmp 0x1403c028c
1403c01d5: add rcx, 0x110
1403c01dc: lea rax, [rip + 0x260dba7]
1403c01e3: jmp 0x1403c01f3
1403c01e5: add rcx, 0x110
1403c01ec: lea rax, [rip + 0x260daf4]
1403c01f3: mov qword ptr [rsp + 0x70], rax
1403c01f8: mov qword ptr [rsp + 0x78], 0x2f
1403c0201: jmp 0x1403c028c
1403c0206: add rcx, 0x110
1403c020d: lea rax, [rip + 0x260dcc1]
1403c0214: mov qword ptr [rsp + 0x70], rax
1403c0219: mov qword ptr [rsp + 0x78], 0x35
1403c0222: jmp 0x1403c028c
1403c0224: add rcx, 0x110
1403c022b: lea rax, [rip + 0x260db87]
1403c0232: mov qword ptr [rsp + 0x70], rax
1403c0237: mov qword ptr [rsp + 0x78], 0x2d
1403c0240: jmp 0x1403c028c
1403c0242: mov rcx, qword ptr [rsp + 0xa0]
1403c024a: add rcx, 0x110
1403c0251: lea rax, [rip + 0x260dbcb]
1403c0258: mov qword ptr [rsp + 0x70], rax
1403c025d: mov qword ptr [rsp + 0x78], 0x2e
1403c0266: jmp 0x1403c028c
1403c0268: mov rcx, qword ptr [rsp + 0xa0]
1403c0270: add rcx, 0x110
1403c0277: lea rax, [rip + 0x260d9e6]
1403c027e: mov qword ptr [rsp + 0x70], rax
1403c0283: mov qword ptr [rsp + 0x78], 0x33
1403c028c: mov qword ptr [rsp + 0x80], rcx
1403c0294: lea rax, [rip - 0x3847cb]
1403c029b: mov qword ptr [rsp + 0x88], rax
1403c02a3: lea rax, [rsp + 0x70]
1403c02a8: mov qword ptr [rsp + 0x90], rax
1403c02b0: lea rax, [rip - 0x2b6427]
1403c02b7: mov qword ptr [rsp + 0x98], rax
1403c02bf: lea rax, [rip + 0x260d8d2]
1403c02c6: mov qword ptr [rsp + 0xa8], rax
1403c02ce: mov qword ptr [rsp + 0xb0], 2
1403c02da: mov qword ptr [rsp + 0xc8], 0
1403c02e6: lea rax, [rsp + 0x80]
1403c02ee: mov qword ptr [rsp + 0xb8], rax
1403c02f6: mov qword ptr [rsp + 0xc0], 2
1403c0302: lea rdx, [rip + 0x260d98f]
1403c0309: lea rcx, [rsp + 0xa8]
1403c0311: call 0x1428d9390
1403c0316: lea rcx, [rip + 0x260dd23]
1403c031d: lea r8, [rip + 0x260dd7c]
1403c0324: mov edx, 0x59
1403c0329: call 0x1428d9330
1403c032e: int3 
