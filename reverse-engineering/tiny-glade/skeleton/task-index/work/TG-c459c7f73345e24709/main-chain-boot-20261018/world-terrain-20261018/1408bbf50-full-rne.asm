1408bbf50: push rbp
1408bbf51: push r15
1408bbf53: push r14
1408bbf55: push r13
1408bbf57: push r12
1408bbf59: push rsi
1408bbf5a: push rdi
1408bbf5b: push rbx
1408bbf5c: sub rsp, 0xc8
1408bbf63: lea rbp, [rsp + 0x80]
1408bbf6b: mov qword ptr [rbp + 0x40], 0xfffffffffffffffe
1408bbf73: mov r14, r9
1408bbf76: mov r13, r8
1408bbf79: mov r8, rdx
1408bbf7c: mov rdx, rcx
1408bbf7f: lea rcx, [rbp - 0x58]
1408bbf83: call 0x140d59140
1408bbf88: mov rcx, qword ptr [rbp - 0x58]
1408bbf8c: mov rdx, qword ptr [rbp - 0x50]
1408bbf90: mov rax, rcx
1408bbf93: neg rax
1408bbf96: jo 0x1408bc2e9
1408bbf9c: mov qword ptr [rbp + 0x20], rcx
1408bbfa0: mov r8, qword ptr [rbp - 0x48]
1408bbfa4: lea rcx, [rbp - 0x58]
1408bbfa8: mov qword ptr [rbp + 0x18], rdx
1408bbfac: call 0x141405b00
1408bbfb1: mov rbx, qword ptr [rbp - 0x58]
1408bbfb5: mov rdi, qword ptr [rbp - 0x50]
1408bbfb9: mov rax, rbx
1408bbfbc: neg rax
1408bbfbf: jno 0x1408bbff2
1408bbfc1: mov rcx, rdi
1408bbfc4: call 0x1428a9850
1408bbfc9: mov rsi, rax
1408bbfcc: mov rdx, qword ptr [rbp + 0x20]
1408bbfd0: test rdx, rdx
1408bbfd3: je 0x1408bc156
1408bbfd9: mov r8d, 1
1408bbfdf: mov rcx, qword ptr [rbp + 0x18]
1408bbfe3: call 0x140613c20
1408bbfe8: mov eax, 1
1408bbfed: jmp 0x1408bc2d2
1408bbff2: movups xmm0, xmmword ptr [rbp - 0x48]
1408bbff6: movups xmm1, xmmword ptr [rbp - 0x38]
1408bbffa: movups xmmword ptr [rbp - 8], xmm1
1408bbffe: movups xmmword ptr [rbp - 0x18], xmm0
1408bc002: mov qword ptr [rbp - 0x28], rbx
1408bc006: mov qword ptr [rbp - 0x20], rdi
1408bc00a: mov rsi, qword ptr [rbp - 0x18]
1408bc00e: xor r12d, r12d
1408bc011: mov ecx, 0x30
1408bc016: mov rax, rsi
1408bc019: mul rcx
1408bc01c: mov r15, rax
1408bc01f: jo 0x1408bc2fb
1408bc025: movabs rax, 0x7ffffffffffffffc
1408bc02f: cmp r15, rax
1408bc032: ja 0x1408bc2fb
1408bc038: test r15, r15
1408bc03b: je 0x1408bc06e
1408bc03d: movzx eax, byte ptr [rip + 0x2d78bc5]
1408bc044: mov r12d, 4
1408bc04a: mov edx, 4
1408bc04f: mov rcx, r15
1408bc052: call 0x140613c10
1408bc057: test rax, rax
1408bc05a: je 0x1408bc2fb
1408bc060: mov r8, rax
1408bc063: mov qword ptr [rbp + 0x28], rsi
1408bc067: test rsi, rsi
1408bc06a: jne 0x1408bc081
1408bc06c: jmp 0x1408bc0e4
1408bc06e: mov r8d, 4
1408bc074: mov qword ptr [rbp + 0x28], 0
1408bc07c: test rsi, rsi
1408bc07f: je 0x1408bc0e4
1408bc081: lea rax, [r8 + 0x2c]
1408bc085: mov rcx, rdi
1408bc088: mov rdx, rsi
1408bc08b: nop dword ptr [rax + rax]
1408bc090: movss xmm0, dword ptr [rcx + 0xc]
1408bc095: mov r11d, dword ptr [rcx + 0x10]
1408bc099: mov r9d, dword ptr [rcx + 0x20]
1408bc09d: mov r10d, dword ptr [rcx + 8]
1408bc0a1: mov dword ptr [rax - 0x24], r10d
1408bc0a5: mov r10, qword ptr [rcx]
1408bc0a8: mov qword ptr [rax - 0x2c], r10
1408bc0ac: movss dword ptr [rax - 0x20], xmm0
1408bc0b1: mov r10, qword ptr [rcx]
1408bc0b4: mov qword ptr [rax - 0x1c], r10
1408bc0b8: mov r10d, dword ptr [rcx + 8]
1408bc0bc: mov dword ptr [rax - 0x14], r10d
1408bc0c0: mov dword ptr [rax - 0x10], r11d
1408bc0c4: mov r10, qword ptr [rcx + 0x14]
1408bc0c8: mov qword ptr [rax - 0xc], r10
1408bc0cc: mov r10d, dword ptr [rcx + 0x1c]
1408bc0d0: mov dword ptr [rax - 4], r10d
1408bc0d4: mov dword ptr [rax], r9d
1408bc0d7: add rax, 0x30
1408bc0db: add rcx, 0x24
1408bc0df: dec rdx
1408bc0e2: jne 0x1408bc090
1408bc0e4: mov r9, rsi
1408bc0e7: mov qword ptr [rbp + 8], rdi
1408bc0eb: mov rcx, r14
1408bc0ee: mov r15, r13
1408bc0f1: mov rdx, r13
1408bc0f4: mov qword ptr [rbp + 0x10], r8
1408bc0f8: call 0x1412bc1f0
1408bc0fd: mov r12, qword ptr [rbp]
1408bc101: mov r13, r12
1408bc104: shl r13, 5
1408bc108: movabs rax, 0x7ffffffffffffffc
1408bc112: cmp r13, rax
1408bc115: ja 0x1408bc30f
1408bc11b: mov rdi, r14
1408bc11e: mov r14, qword ptr [rbp - 8]
1408bc122: test r13, r13
1408bc125: je 0x1408bc160
1408bc127: movzx eax, byte ptr [rip + 0x2d78adb]
1408bc12e: mov eax, 4
1408bc133: mov qword ptr [rbp + 0x38], rax
1408bc137: mov edx, 4
1408bc13c: mov rcx, r13
1408bc13f: call 0x140613c10
1408bc144: test rax, rax
1408bc147: je 0x1408bc317
1408bc14d: mov r8, rax
1408bc150: mov qword ptr [rbp + 0x30], r12
1408bc154: jmp 0x1408bc16e
1408bc156: mov eax, 1
1408bc15b: jmp 0x1408bc2d2
1408bc160: mov r8d, 4
1408bc166: mov qword ptr [rbp + 0x30], 0
1408bc16e: test r12, r12
1408bc171: mov rdx, r15
1408bc174: mov r13, r14
1408bc177: mov rcx, rdi
1408bc17a: mov rdi, qword ptr [rbp + 8]
1408bc17e: mov qword ptr [rbp + 0x38], r8
1408bc182: je 0x1408bc23e
1408bc188: cmp r12, 1
1408bc18c: jne 0x1408bc192
1408bc18e: xor eax, eax
1408bc190: jmp 0x1408bc203
1408bc192: mov r14, r12
1408bc195: and r14, 0xfffffffffffffffe
1408bc199: lea r15, [r13 + 0x1c]
1408bc19d: add r8, 0x30
1408bc1a1: xor r9d, r9d
1408bc1a4: xorps xmm0, xmm0
1408bc1a7: xor eax, eax
1408bc1a9: nop dword ptr [rax]
1408bc1b0: mov r10d, dword ptr [r15 + r9 - 0x10]
1408bc1b5: mov r11d, dword ptr [r15 + r9 - 0x14]
1408bc1ba: mov dword ptr [r8 + r9*2 - 0x28], r11d
1408bc1bf: mov r11, qword ptr [r15 + r9 - 0x1c]
1408bc1c4: mov qword ptr [r8 + r9*2 - 0x30], r11
1408bc1c9: mov dword ptr [r8 + r9*2 - 0x24], r10d
1408bc1ce: movups xmmword ptr [r8 + r9*2 - 0x20], xmm0
1408bc1d4: mov r10d, dword ptr [r15 + r9]
1408bc1d8: mov r11d, dword ptr [r15 + r9 - 4]
1408bc1dd: mov dword ptr [r8 + r9*2 - 8], r11d
1408bc1e2: mov r11, qword ptr [r15 + r9 - 0xc]
1408bc1e7: mov qword ptr [r8 + r9*2 - 0x10], r11
1408bc1ec: mov dword ptr [r8 + r9*2 - 4], r10d
1408bc1f1: add rax, 2
1408bc1f5: movups xmmword ptr [r8 + r9*2], xmm0
1408bc1fa: add r9, 0x20
1408bc1fe: cmp r14, rax
1408bc201: jne 0x1408bc1b0
1408bc203: test r12b, 1
1408bc207: mov r8, qword ptr [rbp + 0x38]
1408bc20b: je 0x1408bc23e
1408bc20d: mov r10, rax
1408bc210: shl r10, 4
1408bc214: mov r9d, dword ptr [r13 + r10 + 0xc]
1408bc219: shl rax, 5
1408bc21d: mov r11d, dword ptr [r13 + r10 + 8]
1408bc222: mov dword ptr [r8 + rax + 8], r11d
1408bc227: mov r10, qword ptr [r13 + r10]
1408bc22c: mov qword ptr [r8 + rax], r10
1408bc230: mov dword ptr [r8 + rax + 0xc], r9d
1408bc235: xorps xmm0, xmm0
1408bc238: movups xmmword ptr [r8 + rax + 0x10], xmm0
1408bc23e: mov r9, r12
1408bc241: call 0x1412bba70
1408bc246: mov rdx, qword ptr [rbp + 0x30]
1408bc24a: test rdx, rdx
1408bc24d: je 0x1408bc262
1408bc24f: shl rdx, 5
1408bc253: mov r8d, 4
1408bc259: mov rcx, qword ptr [rbp + 0x38]
1408bc25d: call 0x140613c20
1408bc262: mov rax, qword ptr [rbp + 0x28]
1408bc266: test rax, rax
1408bc269: je 0x1408bc282
1408bc26b: shl rax, 4
1408bc26f: lea rdx, [rax + rax*2]
1408bc273: mov r8d, 4
1408bc279: mov rcx, qword ptr [rbp + 0x10]
1408bc27d: call 0x140613c20
1408bc282: test rbx, rbx
1408bc285: je 0x1408bc29d
1408bc287: shl rbx, 2
1408bc28b: lea rdx, [rbx + rbx*8]
1408bc28f: mov r8d, 4
1408bc295: mov rcx, rdi
1408bc298: call 0x140613c20
1408bc29d: mov rdx, qword ptr [rbp - 0x10]
1408bc2a1: test rdx, rdx
1408bc2a4: je 0x1408bc2b8
1408bc2a6: shl rdx, 4
1408bc2aa: mov r8d, 4
1408bc2b0: mov rcx, r13
1408bc2b3: call 0x140613c20
1408bc2b8: mov rdx, qword ptr [rbp + 0x20]
1408bc2bc: test rdx, rdx
1408bc2bf: je 0x1408bc2d0
1408bc2c1: mov r8d, 1
1408bc2c7: mov rcx, qword ptr [rbp + 0x18]
1408bc2cb: call 0x140613c20
1408bc2d0: xor eax, eax
1408bc2d2: mov rdx, rsi
1408bc2d5: add rsp, 0xc8
1408bc2dc: pop rbx
1408bc2dd: pop rdi
1408bc2de: pop rsi
1408bc2df: pop r12
1408bc2e1: pop r13
1408bc2e3: pop r14
1408bc2e5: pop r15
1408bc2e7: pop rbp
1408bc2e8: ret 
1408bc2e9: mov rcx, rdx
1408bc2ec: call 0x1428a9590
1408bc2f1: mov rsi, rax
1408bc2f4: mov eax, 1
1408bc2f9: jmp 0x1408bc2d2
1408bc2fb: lea r8, [rip + 0x21e403e]
1408bc302: mov rcx, r12
1408bc305: mov rdx, r15
1408bc308: call 0x1428d8fe3
1408bc30d: jmp 0x1408bc32a
1408bc30f: mov qword ptr [rbp + 0x38], 0
1408bc317: lea r8, [rip + 0x21e4022]
1408bc31e: mov rcx, qword ptr [rbp + 0x38]
1408bc322: mov rdx, r13
1408bc325: call 0x1428d8fe3
1408bc32a: ud2 
1408bc32c: nop dword ptr [rax]
1408bc330: mov qword ptr [rsp + 0x10], rdx
1408bc335: push rbp
1408bc336: push r15
1408bc338: push r14
1408bc33a: push r13
1408bc33c: push r12
1408bc33e: push rsi
1408bc33f: push rdi
1408bc340: push rbx
1408bc341: sub rsp, 0x28
1408bc345: lea rbp, [rdx + 0x80]
1408bc34c: cmp qword ptr [rbp + 0x20], 0
1408bc351: je 0x1408bc366
1408bc353: mov r8d, 1
1408bc359: mov rcx, qword ptr [rbp + 0x18]
1408bc35d: mov rdx, qword ptr [rbp + 0x20]
1408bc361: call 0x140613c20
1408bc366: nop 
1408bc367: add rsp, 0x28
1408bc36b: pop rbx
1408bc36c: pop rdi
1408bc36d: pop rsi
1408bc36e: pop r12
1408bc370: pop r13
1408bc372: pop r14
1408bc374: pop r15
1408bc376: pop rbp
1408bc377: ret 
1408bc378: nop dword ptr [rax + rax]
1408bc380: mov qword ptr [rsp + 0x10], rdx
1408bc385: push rbp
1408bc386: push r15
1408bc388: push r14
1408bc38a: push r13
1408bc38c: push r12
1408bc38e: push rsi
1408bc38f: push rdi
1408bc390: push rbx
1408bc391: sub rsp, 0x28
1408bc395: lea rbp, [rdx + 0x80]
1408bc39c: lea rcx, [rbp - 0x28]
1408bc3a0: call 0x140890ae0
1408bc3a5: nop 
1408bc3a6: add rsp, 0x28
1408bc3aa: pop rbx
1408bc3ab: pop rdi
1408bc3ac: pop rsi
1408bc3ad: pop r12
1408bc3af: pop r13
1408bc3b1: pop r14
1408bc3b3: pop r15
1408bc3b5: pop rbp
1408bc3b6: ret 
1408bc3b7: nop word ptr [rax + rax]
1408bc3c0: mov qword ptr [rsp + 0x10], rdx
1408bc3c5: push rbp
1408bc3c6: push r15
1408bc3c8: push r14
1408bc3ca: push r13
1408bc3cc: push r12
1408bc3ce: push rsi
1408bc3cf: push rdi
1408bc3d0: push rbx
1408bc3d1: sub rsp, 0x28
1408bc3d5: lea rbp, [rdx + 0x80]
1408bc3dc: cmp qword ptr [rbp + 0x28], 0
1408bc3e1: je 0x1408bc3fe
1408bc3e3: mov rax, qword ptr [rbp + 0x28]
1408bc3e7: shl rax, 4
1408bc3eb: lea rdx, [rax + rax*2]
1408bc3ef: mov r8d, 4
1408bc3f5: mov rcx, qword ptr [rbp + 0x10]
1408bc3f9: call 0x140613c20
1408bc3fe: nop 
1408bc3ff: add rsp, 0x28
1408bc403: pop rbx
1408bc404: pop rdi
1408bc405: pop rsi
1408bc406: pop r12
1408bc408: pop r13
1408bc40a: pop r14
1408bc40c: pop r15
1408bc40e: pop rbp
1408bc40f: ret 
1408bc410: mov qword ptr [rsp + 0x10], rdx
1408bc415: push rbp
1408bc416: push r15
1408bc418: push r14
1408bc41a: push r13
1408bc41c: push r12
1408bc41e: push rsi
1408bc41f: push rdi
1408bc420: push rbx
1408bc421: sub rsp, 0x28
1408bc425: lea rbp, [rdx + 0x80]
1408bc42c: cmp qword ptr [rbp + 0x30], 0
1408bc431: je 0x1408bc44a
1408bc433: mov rdx, qword ptr [rbp + 0x30]
1408bc437: shl rdx, 5
1408bc43b: mov r8d, 4
1408bc441: mov rcx, qword ptr [rbp + 0x38]
1408bc445: call 0x140613c20
1408bc44a: nop 
1408bc44b: add rsp, 0x28
1408bc44f: pop rbx
1408bc450: pop rdi
1408bc451: pop rsi
1408bc452: pop r12
1408bc454: pop r13
1408bc456: pop r14
1408bc458: pop r15
1408bc45a: pop rbp
1408bc45b: ret 
