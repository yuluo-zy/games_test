1412c2150: push rbp
1412c2151: push rsi
1412c2152: sub rsp, 0xd8
1412c2159: lea rbp, [rsp + 0x80]
1412c2161: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c2169: mov rsi, rcx
1412c216c: lea rcx, [rbp - 0x58]
1412c2170: xor r8d, r8d
1412c2173: call 0x141613db0
1412c2178: movzx eax, byte ptr [rip + 0x2372a8a]
1412c217f: mov ecx, 0x28
1412c2184: mov edx, 8
1412c2189: call 0x140613c10
1412c218e: test rax, rax
1412c2191: je 0x1412c2247
1412c2197: mov rcx, qword ptr [rbp - 0x38]
1412c219b: mov qword ptr [rax + 0x20], rcx
1412c219f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c21a3: movups xmm1, xmmword ptr [rbp - 0x48]
1412c21a7: movups xmmword ptr [rax + 0x10], xmm1
1412c21ab: movups xmmword ptr [rax], xmm0
1412c21ae: lea rcx, [rip + 0x17dbb4a]
1412c21b5: mov qword ptr [rbp], rcx
1412c21b9: mov qword ptr [rbp + 8], 0x4b
1412c21c1: mov qword ptr [rbp + 0x10], rax
1412c21c5: lea rax, [rip + 0x196ac04]
1412c21cc: mov qword ptr [rbp + 0x18], rax
1412c21d0: mov qword ptr [rbp + 0x30], 0
1412c21d8: mov byte ptr [rbp + 0x46], 5
1412c21dc: mov qword ptr [rbp + 0x20], rcx
1412c21e0: mov qword ptr [rbp + 0x28], 0x4b
1412c21e8: movzx eax, byte ptr [rip + 0x2372a1a]
1412c21ef: mov ecx, 0x40
1412c21f4: mov edx, 8
1412c21f9: call 0x140613c10
1412c21fe: test rax, rax
1412c2201: je 0x1412c2258
1412c2203: movups xmm0, xmmword ptr [rbp + 0x10]
1412c2207: movups xmm1, xmmword ptr [rbp + 0x20]
1412c220b: movups xmm2, xmmword ptr [rbp + 0x30]
1412c220f: movups xmm3, xmmword ptr [rbp + 0x40]
1412c2213: movups xmmword ptr [rax + 0x30], xmm3
1412c2217: movups xmmword ptr [rax + 0x20], xmm2
1412c221b: movups xmmword ptr [rax + 0x10], xmm1
1412c221f: movups xmmword ptr [rax], xmm0
1412c2222: movabs rdx, 0xfb06797fe67b2c94
1412c222c: mov rcx, rsi
1412c222f: mov r8, rax
1412c2232: call 0x140969bf0
1412c2237: cmp rax, 1
1412c223b: je 0x1412c2269
1412c223d: add rsp, 0xd8
1412c2244: pop rsi
1412c2245: pop rbp
1412c2246: ret 
1412c2247: mov ecx, 8
1412c224c: mov edx, 0x28
1412c2251: call 0x1428d9000
1412c2256: jmp 0x1412c2267
1412c2258: mov ecx, 8
1412c225d: mov edx, 0x40
1412c2262: call 0x1428d9000
1412c2267: ud2 
1412c2269: mov rax, rdx
1412c226c: add rdx, 0x10
1412c2270: mov rsi, rdx
1412c2273: cmp qword ptr [rax + 0x18], 0x4b
1412c2278: jne 0x1412c22f6
1412c227a: mov rdx, qword ptr [rsi]
1412c227d: lea rcx, [rip + 0x17dba7b]
1412c2284: mov r8d, 0x4b
1412c228a: call 0x142923bf0
1412c228f: test eax, eax
1412c2291: jne 0x1412c22f6
1412c2293: lea rax, [rip + 0x196b75e]
1412c229a: mov qword ptr [rbp - 0x30], rax
1412c229e: lea rax, [rip - 0xd8ea05]
1412c22a5: mov qword ptr [rbp - 0x28], rax
1412c22a9: mov rax, rbp
1412c22ac: mov qword ptr [rbp - 0x20], rax
1412c22b0: lea rax, [rip - 0x11b8427]
1412c22b7: mov qword ptr [rbp - 0x18], rax
1412c22bb: lea rax, [rip + 0x196b366]
1412c22c2: mov qword ptr [rbp + 0x10], rax
1412c22c6: mov qword ptr [rbp + 0x18], 3
1412c22ce: mov qword ptr [rbp + 0x30], 0
1412c22d6: lea rax, [rbp - 0x30]
1412c22da: mov qword ptr [rbp + 0x20], rax
1412c22de: mov qword ptr [rbp + 0x28], 2
1412c22e6: lea rdx, [rip + 0x196b36b]
1412c22ed: lea rcx, [rbp + 0x10]
1412c22f1: call 0x1428d9390
1412c22f6: lea rax, [rip + 0x196b6fb]
1412c22fd: mov qword ptr [rbp + 0x10], rax
1412c2301: lea rax, [rip - 0xd8ea68]
1412c2308: mov qword ptr [rbp + 0x18], rax
1412c230c: mov qword ptr [rbp + 0x20], rsi
1412c2310: lea rax, [rip - 0x11b8487]
1412c2317: mov qword ptr [rbp + 0x28], rax
1412c231b: mov rcx, rbp
1412c231e: mov qword ptr [rbp + 0x30], rcx
1412c2322: mov qword ptr [rbp + 0x38], rax
1412c2326: lea rax, [rip + 0x196b23b]
1412c232d: mov qword ptr [rbp - 0x30], rax
1412c2331: mov qword ptr [rbp - 0x28], 3
1412c2339: mov qword ptr [rbp - 0x10], 0
1412c2341: lea rax, [rbp + 0x10]
1412c2345: mov qword ptr [rbp - 0x20], rax
1412c2349: mov qword ptr [rbp - 0x18], 3
1412c2351: lea rdx, [rip + 0x196b278]
1412c2358: lea rcx, [rbp - 0x30]
1412c235c: call 0x1428d9390
1412c2361: int3 
1412c2362: nop word ptr cs:[rax + rax]
1412c2370: mov qword ptr [rsp + 0x10], rdx
1412c2375: push rbp
1412c2376: push rsi
1412c2377: sub rsp, 0x28
1412c237b: lea rbp, [rdx + 0x80]
1412c2382: lea rcx, [rbp - 0x58]
1412c2386: call 0x140be24b0
1412c238b: nop 
1412c238c: add rsp, 0x28
1412c2390: pop rsi
1412c2391: pop rbp
1412c2392: ret 
1412c2393: nop word ptr cs:[rax + rax]
1412c23a0: mov qword ptr [rsp + 0x10], rdx
1412c23a5: push rbp
1412c23a6: push rsi
1412c23a7: sub rsp, 0x28
1412c23ab: lea rbp, [rdx + 0x80]
1412c23b2: lea rcx, [rbp + 0x10]
1412c23b6: call 0x1400c04b0
1412c23bb: nop 
1412c23bc: add rsp, 0x28
1412c23c0: pop rsi
1412c23c1: pop rbp
1412c23c2: ret 
