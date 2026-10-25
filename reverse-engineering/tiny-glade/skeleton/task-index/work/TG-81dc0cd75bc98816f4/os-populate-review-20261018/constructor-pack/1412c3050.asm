1412c3050: push rbp
1412c3051: push rsi
1412c3052: sub rsp, 0xd8
1412c3059: lea rbp, [rsp + 0x80]
1412c3061: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c3069: mov rsi, rcx
1412c306c: lea rcx, [rbp - 0x58]
1412c3070: xor r8d, r8d
1412c3073: call 0x141612a90
1412c3078: movzx eax, byte ptr [rip + 0x2371b8a]
1412c307f: mov ecx, 0x28
1412c3084: mov edx, 8
1412c3089: call 0x140613c10
1412c308e: test rax, rax
1412c3091: je 0x1412c3147
1412c3097: mov rcx, qword ptr [rbp - 0x38]
1412c309b: mov qword ptr [rax + 0x20], rcx
1412c309f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c30a3: movups xmm1, xmmword ptr [rbp - 0x48]
1412c30a7: movups xmmword ptr [rax + 0x10], xmm1
1412c30ab: movups xmmword ptr [rax], xmm0
1412c30ae: lea rcx, [rip + 0x17db007]
1412c30b5: mov qword ptr [rbp], rcx
1412c30b9: mov qword ptr [rbp + 8], 0x39
1412c30c1: mov qword ptr [rbp + 0x10], rax
1412c30c5: lea rax, [rip + 0x1969e54]
1412c30cc: mov qword ptr [rbp + 0x18], rax
1412c30d0: mov qword ptr [rbp + 0x30], 0
1412c30d8: mov byte ptr [rbp + 0x46], 5
1412c30dc: mov qword ptr [rbp + 0x20], rcx
1412c30e0: mov qword ptr [rbp + 0x28], 0x39
1412c30e8: movzx eax, byte ptr [rip + 0x2371b1a]
1412c30ef: mov ecx, 0x40
1412c30f4: mov edx, 8
1412c30f9: call 0x140613c10
1412c30fe: test rax, rax
1412c3101: je 0x1412c3158
1412c3103: movups xmm0, xmmword ptr [rbp + 0x10]
1412c3107: movups xmm1, xmmword ptr [rbp + 0x20]
1412c310b: movups xmm2, xmmword ptr [rbp + 0x30]
1412c310f: movups xmm3, xmmword ptr [rbp + 0x40]
1412c3113: movups xmmword ptr [rax + 0x30], xmm3
1412c3117: movups xmmword ptr [rax + 0x20], xmm2
1412c311b: movups xmmword ptr [rax + 0x10], xmm1
1412c311f: movups xmmword ptr [rax], xmm0
1412c3122: movabs rdx, 0x72c19119a2cdd0a1
1412c312c: mov rcx, rsi
1412c312f: mov r8, rax
1412c3132: call 0x140969bf0
1412c3137: cmp rax, 1
1412c313b: je 0x1412c3169
1412c313d: add rsp, 0xd8
1412c3144: pop rsi
1412c3145: pop rbp
1412c3146: ret 
1412c3147: mov ecx, 8
1412c314c: mov edx, 0x28
1412c3151: call 0x1428d9000
1412c3156: jmp 0x1412c3167
1412c3158: mov ecx, 8
1412c315d: mov edx, 0x40
1412c3162: call 0x1428d9000
1412c3167: ud2 
1412c3169: mov rax, rdx
1412c316c: add rdx, 0x10
1412c3170: mov rsi, rdx
1412c3173: cmp qword ptr [rax + 0x18], 0x39
1412c3178: jne 0x1412c31f6
1412c317a: mov rdx, qword ptr [rsi]
1412c317d: lea rcx, [rip + 0x17daf38]
1412c3184: mov r8d, 0x39
1412c318a: call 0x142923bf0
1412c318f: test eax, eax
1412c3191: jne 0x1412c31f6
1412c3193: lea rax, [rip + 0x196a5ce]
1412c319a: mov qword ptr [rbp - 0x30], rax
1412c319e: lea rax, [rip - 0xd8f905]
1412c31a5: mov qword ptr [rbp - 0x28], rax
1412c31a9: mov rax, rbp
1412c31ac: mov qword ptr [rbp - 0x20], rax
1412c31b0: lea rax, [rip - 0x11b9327]
1412c31b7: mov qword ptr [rbp - 0x18], rax
1412c31bb: lea rax, [rip + 0x196a466]
1412c31c2: mov qword ptr [rbp + 0x10], rax
1412c31c6: mov qword ptr [rbp + 0x18], 3
1412c31ce: mov qword ptr [rbp + 0x30], 0
1412c31d6: lea rax, [rbp - 0x30]
1412c31da: mov qword ptr [rbp + 0x20], rax
1412c31de: mov qword ptr [rbp + 0x28], 2
1412c31e6: lea rdx, [rip + 0x196a46b]
1412c31ed: lea rcx, [rbp + 0x10]
1412c31f1: call 0x1428d9390
1412c31f6: lea rax, [rip + 0x196a56b]
1412c31fd: mov qword ptr [rbp + 0x10], rax
1412c3201: lea rax, [rip - 0xd8f968]
1412c3208: mov qword ptr [rbp + 0x18], rax
1412c320c: mov qword ptr [rbp + 0x20], rsi
1412c3210: lea rax, [rip - 0x11b9387]
1412c3217: mov qword ptr [rbp + 0x28], rax
1412c321b: mov rcx, rbp
1412c321e: mov qword ptr [rbp + 0x30], rcx
1412c3222: mov qword ptr [rbp + 0x38], rax
1412c3226: lea rax, [rip + 0x196a33b]
1412c322d: mov qword ptr [rbp - 0x30], rax
1412c3231: mov qword ptr [rbp - 0x28], 3
1412c3239: mov qword ptr [rbp - 0x10], 0
1412c3241: lea rax, [rbp + 0x10]
1412c3245: mov qword ptr [rbp - 0x20], rax
1412c3249: mov qword ptr [rbp - 0x18], 3
1412c3251: lea rdx, [rip + 0x196a378]
1412c3258: lea rcx, [rbp - 0x30]
1412c325c: call 0x1428d9390
1412c3261: int3 
1412c3262: nop word ptr cs:[rax + rax]
1412c3270: mov qword ptr [rsp + 0x10], rdx
1412c3275: push rbp
1412c3276: push rsi
1412c3277: sub rsp, 0x28
1412c327b: lea rbp, [rdx + 0x80]
1412c3282: lea rcx, [rbp - 0x58]
1412c3286: call 0x140be24b0
1412c328b: nop 
1412c328c: add rsp, 0x28
1412c3290: pop rsi
1412c3291: pop rbp
1412c3292: ret 
1412c3293: nop word ptr cs:[rax + rax]
1412c32a0: mov qword ptr [rsp + 0x10], rdx
1412c32a5: push rbp
1412c32a6: push rsi
1412c32a7: sub rsp, 0x28
1412c32ab: lea rbp, [rdx + 0x80]
1412c32b2: lea rcx, [rbp + 0x10]
1412c32b6: call 0x1400c04b0
1412c32bb: nop 
1412c32bc: add rsp, 0x28
1412c32c0: pop rsi
1412c32c1: pop rbp
1412c32c2: ret 
