1402b1cb0: push rbp
1402b1cb1: push r15
1402b1cb3: push r14
1402b1cb5: push r13
1402b1cb7: push r12
1402b1cb9: push rsi
1402b1cba: push rdi
1402b1cbb: push rbx
1402b1cbc: sub rsp, 0x1b8
1402b1cc3: lea rbp, [rsp + 0x80]
1402b1ccb: movaps xmmword ptr [rbp + 0x120], xmm7
1402b1cd2: movaps xmmword ptr [rbp + 0x110], xmm6
1402b1cd9: mov qword ptr [rbp + 0x108], 0xfffffffffffffffe
1402b1ce4: mov rbx, r9
1402b1ce7: mov r14, r8
1402b1cea: mov rdi, rdx
1402b1ced: mov rsi, rcx
1402b1cf0: call 0x140d4d740
1402b1cf5: mov r15, rax
1402b1cf8: test byte ptr [rax], 1
1402b1cfb: je 0x1402b1d0e
1402b1cfd: mov rcx, r15
1402b1d00: add rcx, 8
1402b1d04: mov rax, qword ptr [r15 + 8]
1402b1d08: mov rdx, qword ptr [r15 + 0x10]
1402b1d0c: jmp 0x1402b1d29
1402b1d0e: call 0x140d64e00
1402b1d13: mov rcx, r15
1402b1d16: add rcx, 8
1402b1d1a: mov qword ptr [r15], 1
1402b1d21: mov qword ptr [r15 + 8], rax
1402b1d25: mov qword ptr [r15 + 0x10], rdx
1402b1d29: lea r8, [rax + 1]
1402b1d2d: mov qword ptr [rcx], r8
1402b1d30: movups xmm6, xmmword ptr [rip + 0x26fbd69]
1402b1d37: movaps xmmword ptr [rbp], xmm6
1402b1d3b: movups xmm7, xmmword ptr [rip + 0x26fbd4e]
1402b1d42: movaps xmmword ptr [rbp - 0x10], xmm7
1402b1d46: mov qword ptr [rbp + 0x10], rax
1402b1d4a: mov qword ptr [rbp + 0x18], rdx
1402b1d4e: call 0x140d4d740
1402b1d53: mov r15, rax
1402b1d56: test byte ptr [rax], 1
1402b1d59: je 0x1402b1d6c
1402b1d5b: mov rcx, r15
1402b1d5e: add rcx, 8
1402b1d62: mov rax, qword ptr [r15 + 8]
1402b1d66: mov rdx, qword ptr [r15 + 0x10]
1402b1d6a: jmp 0x1402b1d87
1402b1d6c: call 0x140d64e00
1402b1d71: mov rcx, r15
1402b1d74: add rcx, 8
1402b1d78: mov qword ptr [r15], 1
1402b1d7f: mov qword ptr [r15 + 8], rax
1402b1d83: mov qword ptr [r15 + 0x10], rdx
1402b1d87: lea r8, [rax + 1]
1402b1d8b: mov qword ptr [rcx], r8
1402b1d8e: movaps xmmword ptr [rbp + 0x30], xmm6
1402b1d92: movaps xmmword ptr [rbp + 0x20], xmm7
1402b1d96: mov qword ptr [rbp + 0x40], rax
1402b1d9a: mov qword ptr [rbp + 0x48], rdx
1402b1d9e: call 0x140d4d740
1402b1da3: mov r15, rax
1402b1da6: test byte ptr [rax], 1
1402b1da9: je 0x1402b1dc3
1402b1dab: mov r8, r15
1402b1dae: add r8, 8
1402b1db2: mov rax, qword ptr [r15 + 8]
1402b1db6: mov rcx, qword ptr [r15 + 0x10]
1402b1dba: mov qword ptr [rbp + 0x100], rcx
1402b1dc1: jmp 0x1402b1de5
1402b1dc3: call 0x140d64e00
1402b1dc8: mov r8, r15
1402b1dcb: add r8, 8
1402b1dcf: mov qword ptr [r15], 1
1402b1dd6: mov qword ptr [r15 + 8], rax
1402b1dda: mov qword ptr [rbp + 0x100], rdx
1402b1de1: mov qword ptr [r15 + 0x10], rdx
1402b1de5: mov qword ptr [rbp + 0xf0], rax
1402b1dec: lea rcx, [rax + 1]
1402b1df0: mov qword ptr [r8], rcx
1402b1df3: movaps xmm0, xmmword ptr [rbp - 0x10]
1402b1df7: movaps xmm1, xmmword ptr [rbp]
1402b1dfb: movaps xmm2, xmmword ptr [rbp + 0x10]
1402b1dff: movaps xmmword ptr [rbp + 0x90], xmm2
1402b1e06: movaps xmmword ptr [rbp + 0x80], xmm1
1402b1e0d: movaps xmmword ptr [rbp + 0x70], xmm0
1402b1e11: movaps xmm0, xmmword ptr [rbp + 0x20]
1402b1e15: movaps xmm1, xmmword ptr [rbp + 0x30]
1402b1e19: movaps xmm2, xmmword ptr [rbp + 0x40]
1402b1e1d: movaps xmmword ptr [rbp + 0xa0], xmm0
1402b1e24: movaps xmmword ptr [rbp + 0xb0], xmm1
1402b1e2b: movaps xmmword ptr [rbp + 0xc0], xmm2
1402b1e32: movaps xmmword ptr [rbp + 0xd0], xmm7
1402b1e39: movaps xmmword ptr [rbp + 0xe0], xmm6
1402b1e40: mov eax, dword ptr [r14 + 0x1c]
1402b1e44: mov rcx, qword ptr [r14 + 0x10]
1402b1e48: mov dword ptr [rcx], eax
1402b1e4a: mov rcx, qword ptr [r14]
1402b1e4d: mov qword ptr [rbp + 0xf8], rcx
1402b1e54: call 0x140602200
1402b1e59: mov r13, qword ptr [rbp + 0x1a0]
1402b1e60: movaps xmm0, xmmword ptr [rbp + 0xe0]
1402b1e67: mov rcx, qword ptr [rbp + 0xf8]
1402b1e6e: movups xmmword ptr [rcx + 0x70], xmm0
1402b1e72: movaps xmm0, xmmword ptr [rbp + 0xd0]
1402b1e79: movups xmmword ptr [rcx + 0x60], xmm0
1402b1e7d: movaps xmm0, xmmword ptr [rbp + 0xc0]
1402b1e84: movups xmmword ptr [rcx + 0x50], xmm0
1402b1e88: movaps xmm0, xmmword ptr [rbp + 0xb0]
1402b1e8f: movups xmmword ptr [rcx + 0x40], xmm0
1402b1e93: movdqa xmm0, xmmword ptr [rbp + 0x70]
1402b1e98: movaps xmm1, xmmword ptr [rbp + 0x80]
1402b1e9f: movaps xmm2, xmmword ptr [rbp + 0x90]
1402b1ea6: movaps xmm3, xmmword ptr [rbp + 0xa0]
1402b1ead: movups xmmword ptr [rcx + 0x30], xmm3
1402b1eb1: movups xmmword ptr [rcx + 0x20], xmm2
1402b1eb5: movups xmmword ptr [rcx + 0x10], xmm1
1402b1eb9: movdqu xmmword ptr [rcx], xmm0
1402b1ebd: mov rax, qword ptr [rbp + 0xf0]
1402b1ec4: mov qword ptr [rcx + 0x80], rax
1402b1ecb: mov rax, qword ptr [rbp + 0x100]
1402b1ed2: mov qword ptr [rcx + 0x88], rax
1402b1ed9: call 0x140d4d740
1402b1ede: mov r12, rax
1402b1ee1: test byte ptr [rax], 1
1402b1ee4: je 0x1402b1ef9
1402b1ee6: mov rax, r12
1402b1ee9: add rax, 8
1402b1eed: mov r14, qword ptr [r12 + 8]
1402b1ef2: mov r15, qword ptr [r12 + 0x10]
1402b1ef7: jmp 0x1402b1f1d
1402b1ef9: call 0x140d64e00
1402b1efe: mov r14, rax
1402b1f01: mov r15, rdx
1402b1f04: mov rax, r12
1402b1f07: add rax, 8
1402b1f0b: mov qword ptr [r12], 1
1402b1f13: mov qword ptr [r12 + 8], r14
1402b1f18: mov qword ptr [r12 + 0x10], rdx
1402b1f1d: lea rcx, [r14 + 1]
1402b1f21: mov qword ptr [rax], rcx
1402b1f24: mov eax, dword ptr [r13 + 0x1c]
1402b1f28: mov rcx, qword ptr [r13 + 0x10]
1402b1f2c: mov dword ptr [rcx], eax
1402b1f2e: mov r12, qword ptr [r13]
1402b1f32: mov rdx, qword ptr [r12 + 8]
1402b1f37: test rdx, rdx
1402b1f3a: je 0x1402b1f65
1402b1f3c: mov rax, rdx
1402b1f3f: shl rax, 4
1402b1f43: add rdx, rax
1402b1f46: add rdx, 0x21
1402b1f4a: je 0x1402b1f65
1402b1f4c: mov rcx, qword ptr [r12]
1402b1f50: neg rax
1402b1f53: add rcx, rax
1402b1f56: add rcx, -0x10
1402b1f5a: mov r8d, 0x10
1402b1f60: call 0x140613c20
1402b1f65: movups xmmword ptr [r12 + 0x10], xmm6
1402b1f6b: movups xmmword ptr [r12], xmm7
1402b1f70: mov qword ptr [r12 + 0x20], r14
1402b1f75: mov qword ptr [r12 + 0x28], r15
1402b1f7a: call 0x140d4d740
1402b1f7f: mov r14, rax
1402b1f82: test byte ptr [rax], 1
1402b1f85: je 0x1402b1f9f
1402b1f87: mov r8, r14
1402b1f8a: add r8, 8
1402b1f8e: mov rax, qword ptr [r14 + 8]
1402b1f92: mov rcx, qword ptr [r14 + 0x10]
1402b1f96: mov qword ptr [rbp + 0x100], rcx
1402b1f9d: jmp 0x1402b1fc1
1402b1f9f: call 0x140d64e00
1402b1fa4: mov r8, r14
1402b1fa7: add r8, 8
1402b1fab: mov qword ptr [r14], 1
1402b1fb2: mov qword ptr [r14 + 8], rax
1402b1fb6: mov qword ptr [rbp + 0x100], rdx
1402b1fbd: mov qword ptr [r14 + 0x10], rdx
1402b1fc1: mov qword ptr [rbp + 0xf0], rax
1402b1fc8: lea rcx, [rax + 1]
1402b1fcc: mov qword ptr [r8], rcx
1402b1fcf: mov eax, dword ptr [rbx + 0x1c]
1402b1fd2: mov rcx, qword ptr [rbx + 0x10]
1402b1fd6: mov dword ptr [rcx], eax
1402b1fd8: mov rcx, qword ptr [rbx]
1402b1fdb: lea rdx, [rcx + 0x20]
1402b1fdf: mov r8d, 0x20
1402b1fe5: mov r9d, 0x10
1402b1feb: mov qword ptr [rbp + 0xf8], rcx
1402b1ff2: call 0x140051fa0
1402b1ff7: mov rax, qword ptr [rbp + 0x1d0]
1402b1ffe: movups xmm6, xmmword ptr [rip + 0x26fba9b]
1402b2005: mov rdx, qword ptr [rbp + 0xf8]
1402b200c: movups xmmword ptr [rdx + 0x10], xmm6
1402b2010: movups xmm7, xmmword ptr [rip + 0x26fba79]
1402b2017: movups xmmword ptr [rdx], xmm7
1402b201a: mov rcx, qword ptr [rbp + 0xf0]
1402b2021: mov qword ptr [rdx + 0x20], rcx
1402b2025: mov rcx, qword ptr [rbp + 0x100]
1402b202c: mov qword ptr [rdx + 0x28], rcx
1402b2030: mov ecx, dword ptr [rax + 0x1c]
1402b2033: mov rdx, qword ptr [rax + 0x10]
1402b2037: mov dword ptr [rdx], ecx
1402b2039: mov rax, qword ptr [rax]
1402b203c: mov byte ptr [rax + 0x10], 0
1402b2040: call 0x140d4d740
1402b2045: mov rbx, rax
1402b2048: test byte ptr [rax], 1
1402b204b: je 0x1402b205e
1402b204d: mov rcx, rbx
1402b2050: add rcx, 8
1402b2054: mov rax, qword ptr [rbx + 8]
1402b2058: mov rdx, qword ptr [rbx + 0x10]
1402b205c: jmp 0x1402b2079
1402b205e: call 0x140d64e00
1402b2063: mov rcx, rbx
1402b2066: add rcx, 8
1402b206a: mov qword ptr [rbx], 1
1402b2071: mov qword ptr [rbx + 8], rax
1402b2075: mov qword ptr [rbx + 0x10], rdx
1402b2079: lea r8, [rax + 1]
1402b207d: mov qword ptr [rcx], r8
1402b2080: movaps xmmword ptr [rbp + 0x30], xmm6
1402b2084: movaps xmmword ptr [rbp + 0x20], xmm7
1402b2088: mov qword ptr [rbp + 0x40], rax
1402b208c: mov qword ptr [rbp + 0x48], rdx
1402b2090: call 0x140d4d740
1402b2095: mov rbx, rax
1402b2098: test byte ptr [rax], 1
1402b209b: je 0x1402b20ae
1402b209d: mov rcx, rbx
1402b20a0: add rcx, 8
1402b20a4: mov rax, qword ptr [rbx + 8]
1402b20a8: mov rdx, qword ptr [rbx + 0x10]
1402b20ac: jmp 0x1402b20c9
1402b20ae: call 0x140d64e00
1402b20b3: mov rcx, rbx
1402b20b6: add rcx, 8
1402b20ba: mov qword ptr [rbx], 1
1402b20c1: mov qword ptr [rbx + 8], rax
1402b20c5: mov qword ptr [rbx + 0x10], rdx
1402b20c9: lea r8, [rax + 1]
1402b20cd: mov qword ptr [rcx], r8
1402b20d0: movaps xmmword ptr [rbp + 0x80], xmm6
1402b20d7: movaps xmmword ptr [rbp + 0x70], xmm7
1402b20db: mov qword ptr [rbp + 0x90], rax
1402b20e2: mov qword ptr [rbp + 0x98], rdx
1402b20e9: xor ecx, ecx
1402b20eb: call 0x1400d9b20
1402b20f0: test rax, rax
1402b20f3: je 0x1402b2916
1402b20f9: mov rcx, qword ptr [rbp + 0x1d8]
1402b2100: mov r8, qword ptr [rax]
1402b2103: mov rdx, qword ptr [rax + 8]
1402b2107: mov qword ptr [rbp + 0x100], rdx
1402b210e: mov qword ptr [rbp + 0xf0], r8
1402b2115: lea rdx, [r8 + 1]
1402b2119: mov qword ptr [rax], rdx
1402b211c: movaps xmm0, xmmword ptr [rbp + 0x20]
1402b2120: movaps xmm1, xmmword ptr [rbp + 0x30]
1402b2124: movaps xmm2, xmmword ptr [rbp + 0x40]
1402b2128: movaps xmmword ptr [rbp + 0x10], xmm2
1402b212c: movaps xmmword ptr [rbp], xmm1
1402b2130: movaps xmmword ptr [rbp - 0x10], xmm0
1402b2134: movaps xmm0, xmmword ptr [rbp + 0x70]
1402b2138: movaps xmm1, xmmword ptr [rbp + 0x80]
1402b213f: movaps xmm2, xmmword ptr [rbp + 0x90]
1402b2146: movaps xmmword ptr [rbp - 0x40], xmm0
1402b214a: movaps xmmword ptr [rbp - 0x30], xmm1
1402b214e: movaps xmmword ptr [rbp - 0x20], xmm2
1402b2152: mov eax, dword ptr [rcx + 0x1c]
1402b2155: mov rdx, qword ptr [rcx + 0x10]
1402b2159: mov dword ptr [rdx], eax
1402b215b: mov rcx, qword ptr [rcx]
1402b215e: mov qword ptr [rbp + 0xf8], rcx
1402b2165: call 0x1405faf50
1402b216a: mov r15, qword ptr [rbp + 0x1e0]
1402b2171: mov r14, qword ptr [rbp + 0x1b0]
1402b2178: mov rax, qword ptr [rbp + 0x1a8]
1402b217f: mov rdx, qword ptr [rbp + 0xf8]
1402b2186: mov qword ptr [rdx], 0
1402b218d: mov qword ptr [rdx + 8], 8
1402b2195: mov qword ptr [rdx + 0x10], 0
1402b219d: movaps xmm0, xmmword ptr [rbp - 0x10]
1402b21a1: movaps xmm1, xmmword ptr [rbp]
1402b21a5: movaps xmm2, xmmword ptr [rbp + 0x10]
1402b21a9: movups xmmword ptr [rdx + 0x18], xmm0
1402b21ad: movups xmmword ptr [rdx + 0x28], xmm1
1402b21b1: movups xmmword ptr [rdx + 0x38], xmm2
1402b21b5: movaps xmm0, xmmword ptr [rbp - 0x40]
1402b21b9: movaps xmm1, xmmword ptr [rbp - 0x30]
1402b21bd: movaps xmm2, xmmword ptr [rbp - 0x20]
1402b21c1: movups xmmword ptr [rdx + 0x48], xmm0
1402b21c5: movups xmmword ptr [rdx + 0x58], xmm1
1402b21c9: movups xmmword ptr [rdx + 0x68], xmm2
1402b21cd: movups xmm0, xmmword ptr [rip + 0x26f16cc]
1402b21d4: movups xmmword ptr [rdx + 0x78], xmm0
1402b21d8: movups xmm0, xmmword ptr [rip + 0x26f16d1]
1402b21df: movups xmmword ptr [rdx + 0x88], xmm0
1402b21e6: mov rcx, qword ptr [rbp + 0xf0]
1402b21ed: mov qword ptr [rdx + 0x98], rcx
1402b21f4: mov rcx, qword ptr [rbp + 0x100]
1402b21fb: mov qword ptr [rdx + 0xa0], rcx
1402b2202: mov r12d, dword ptr [rax + 0x1c]
1402b2206: mov r13, qword ptr [rax + 0x10]
1402b220a: mov dword ptr [r13], r12d
1402b220e: mov rbx, qword ptr [rax]
1402b2211: mov rcx, rbx
1402b2214: call 0x14085a730
1402b2219: mov dword ptr [r13], r12d
1402b221d: mov rcx, rbx
1402b2220: call 0x14085a550
1402b2225: mov eax, dword ptr [r15 + 0x1c]
1402b2229: mov rcx, qword ptr [r15 + 0x10]
1402b222d: mov dword ptr [rcx], eax
1402b222f: mov rcx, qword ptr [r15]
1402b2232: call 0x140aaae80
1402b2237: lea rcx, [rbp + 0x70]
1402b223b: call 0x140aa5150
1402b2240: mov eax, dword ptr [r14 + 0x1c]
1402b2244: mov rcx, qword ptr [r14 + 0x10]
1402b2248: mov dword ptr [rcx], eax
1402b224a: mov rcx, qword ptr [r14]
1402b224d: mov qword ptr [rbp + 0x100], rcx
1402b2254: call 0x14060cfb0
1402b2259: mov rbx, qword ptr [rbp + 0x1b8]
1402b2260: movups xmm0, xmmword ptr [rbp + 0xc0]
1402b2267: mov rax, qword ptr [rbp + 0x100]
1402b226e: movups xmmword ptr [rax + 0x50], xmm0
1402b2272: movups xmm0, xmmword ptr [rbp + 0xb0]
1402b2279: movups xmmword ptr [rax + 0x40], xmm0
1402b227d: movups xmm0, xmmword ptr [rbp + 0x70]
1402b2281: movups xmm1, xmmword ptr [rbp + 0x80]
1402b2288: movups xmm2, xmmword ptr [rbp + 0x90]
1402b228f: movups xmm3, xmmword ptr [rbp + 0xa0]
1402b2296: movups xmmword ptr [rax + 0x30], xmm3
1402b229a: movups xmmword ptr [rax + 0x20], xmm2
1402b229e: movups xmmword ptr [rax + 0x10], xmm1
1402b22a2: movups xmmword ptr [rax], xmm0
1402b22a5: lea rcx, [rbp + 0x70]
1402b22a9: call 0x140b19890
1402b22ae: mov eax, dword ptr [rbx + 0x1c]
1402b22b1: mov rcx, qword ptr [rbx + 0x10]
1402b22b5: mov dword ptr [rcx], eax
1402b22b7: mov rbx, qword ptr [rbx]
1402b22ba: mov rdx, qword ptr [rbx]
1402b22bd: test rdx, rdx
1402b22c0: je 0x1402b22d5
1402b22c2: mov rcx, qword ptr [rbx + 8]
1402b22c6: shl rdx, 2
1402b22ca: mov r8d, 4
1402b22d0: call 0x140613c20
1402b22d5: mov rax, qword ptr [rbp + 0x80]
1402b22dc: mov qword ptr [rbx + 0x10], rax
1402b22e0: movups xmm0, xmmword ptr [rbp + 0x70]
1402b22e4: movups xmmword ptr [rbx], xmm0
1402b22e7: mov rdi, qword ptr [rdi]
1402b22ea: lea rdx, [rip + 0x26fbed7]
1402b22f1: mov rcx, rdi
1402b22f4: call 0x1405ca870
1402b22f9: test rax, rax
1402b22fc: je 0x1402b28f2
1402b2302: mov rbx, rax
1402b2305: lea rax, [rip + 0x274a9f0]
1402b230c: mov qword ptr [rbp + 0x20], rax
1402b2310: mov qword ptr [rbp + 0x28], 0x32
1402b2318: cmp qword ptr [rbx + 0x18], 0x32
1402b231d: jne 0x1402b289e
1402b2323: mov rdx, qword ptr [rbx + 0x10]
1402b2327: lea rcx, [rip + 0x274a9ce]
1402b232e: mov r8d, 0x32
1402b2334: call 0x142923bf0
1402b2339: test eax, eax
1402b233b: jne 0x1402b289e
1402b2341: mov rcx, qword ptr [rbx]
1402b2344: mov rax, qword ptr [rbx + 8]
1402b2348: call qword ptr [rax + 0x18]
1402b234b: test rax, rax
1402b234e: je 0x1402b28f2
1402b2354: mov rbx, rax
1402b2357: mov rsi, qword ptr [rsi]
1402b235a: mov rcx, rsi
1402b235d: call 0x140987ca0
1402b2362: mov r14, rax
1402b2365: call 0x14099ca50
1402b236a: mov r15, rax
1402b236d: test r15d, r15d
1402b2370: je 0x1402b2927
1402b2376: movzx eax, byte ptr [rip + 0x338288c]
1402b237d: mov ecx, 0x14000
1402b2382: mov edx, 4
1402b2387: call 0x140613c10
1402b238c: test rax, rax
1402b238f: je 0x1402b2933
1402b2395: mov r13, qword ptr [rbp + 0x1c8]
1402b239c: mov r12, qword ptr [rbp + 0x1c0]
1402b23a3: shr r15, 0x20
1402b23a7: movd xmm0, r15d
1402b23ac: pshufd xmm0, xmm0, 0
1402b23b1: mov ecx, 0x14
1402b23b6: nop word ptr cs:[rax + rax]
1402b23c0: movdqu xmmword ptr [rax + rcx*4 - 0x50], xmm0
1402b23c6: movdqu xmmword ptr [rax + rcx*4 - 0x40], xmm0
1402b23cc: movdqu xmmword ptr [rax + rcx*4 - 0x30], xmm0
1402b23d2: movdqu xmmword ptr [rax + rcx*4 - 0x20], xmm0
1402b23d8: movdqu xmmword ptr [rax + rcx*4 - 0x10], xmm0
1402b23de: movdqu xmmword ptr [rax + rcx*4], xmm0
1402b23e3: add rcx, 0x18
1402b23e7: cmp rcx, 0x500c
1402b23ee: jne 0x1402b23c0
1402b23f0: mov dword ptr [rax + 0x13fe0], r15d
1402b23f7: mov dword ptr [rax + 0x13fe4], r15d
1402b23fe: mov dword ptr [rax + 0x13fe8], r15d
1402b2405: mov dword ptr [rax + 0x13fec], r15d
1402b240c: mov dword ptr [rax + 0x13ff0], r15d
1402b2413: mov dword ptr [rax + 0x13ff4], r15d
1402b241a: mov dword ptr [rax + 0x13ff8], r15d
1402b2421: mov dword ptr [rax + 0x13ffc], r15d
1402b2428: cmp qword ptr [rbx + 0x18], 0x14000
1402b2430: mov qword ptr [rbp + 0x100], rax
1402b2437: jb 0x1402b2949
1402b243d: mov qword ptr [rbp - 0x10], 0
1402b2445: mov qword ptr [rbp - 8], 0x14000
1402b244d: lea r15, [rbp - 0x10]
1402b2451: mov qword ptr [rsp + 0x28], r15
1402b2456: mov qword ptr [rsp + 0x38], 0
1402b245f: mov qword ptr [rsp + 0x30], 1
1402b2468: mov qword ptr [rsp + 0x20], 0x14000
1402b2471: lea rcx, [rbp + 0x70]
1402b2475: mov rdx, r14
1402b2478: mov r8, rbx
1402b247b: mov r9, rax
1402b247e: call 0x140beefa0
1402b2483: mov rax, qword ptr [rbp + 0x70]
1402b2487: cmp rax, 0xb
1402b248b: jne 0x1402b2950
1402b2491: mov edx, 0x14000
1402b2496: mov r8d, 4
1402b249c: mov rcx, qword ptr [rbp + 0x100]
1402b24a3: call 0x140613c20
1402b24a8: lea rcx, [rbp + 0x70]
1402b24ac: call 0x140b19900
1402b24b1: mov eax, dword ptr [r12 + 0x1c]
1402b24b6: mov rcx, qword ptr [r12 + 0x10]
1402b24bb: mov dword ptr [rcx], eax
1402b24bd: mov rbx, qword ptr [r12]
1402b24c1: mov rdx, qword ptr [rbx]
1402b24c4: test rdx, rdx
1402b24c7: je 0x1402b24dc
1402b24c9: mov rcx, qword ptr [rbx + 8]
1402b24cd: shl rdx, 2
1402b24d1: mov r8d, 4
1402b24d7: call 0x140613c20
1402b24dc: mov rax, qword ptr [rbp + 0x80]
1402b24e3: mov qword ptr [rbx + 0x10], rax
1402b24e7: movups xmm0, xmmword ptr [rbp + 0x70]
1402b24eb: movups xmmword ptr [rbx], xmm0
1402b24ee: lea rdx, [rip + 0x26fbcbb]
1402b24f5: mov rcx, rdi
1402b24f8: call 0x1405ca870
1402b24fd: test rax, rax
1402b2500: je 0x1402b28fe
1402b2506: mov rbx, rax
1402b2509: lea rax, [rip + 0x274a6bd]
1402b2510: mov qword ptr [rbp + 0x20], rax
1402b2514: mov qword ptr [rbp + 0x28], 0x32
1402b251c: cmp qword ptr [rbx + 0x18], 0x32
1402b2521: jne 0x1402b289e
1402b2527: mov rdx, qword ptr [rbx + 0x10]
1402b252b: lea rcx, [rip + 0x274a69b]
1402b2532: mov r8d, 0x32
1402b2538: call 0x142923bf0
1402b253d: test eax, eax
1402b253f: jne 0x1402b289e
1402b2545: mov rcx, qword ptr [rbx]
1402b2548: mov rax, qword ptr [rbx + 8]
1402b254c: call qword ptr [rax + 0x18]
1402b254f: test rax, rax
1402b2552: je 0x1402b28fe
1402b2558: mov rbx, rax
1402b255b: mov r12, r15
1402b255e: mov rcx, rsi
1402b2561: call 0x140987ca0
1402b2566: mov r14, rax
1402b2569: call 0x14099ca50
1402b256e: mov r15, rax
1402b2571: test r15d, r15d
1402b2574: je 0x1402b29ed
1402b257a: movzx eax, byte ptr [rip + 0x3382688]
1402b2581: mov ecx, 0x14000
1402b2586: mov edx, 4
1402b258b: call 0x140613c10
1402b2590: test rax, rax
1402b2593: je 0x1402b29f9
1402b2599: shr r15, 0x20
1402b259d: movd xmm0, r15d
1402b25a2: pshufd xmm0, xmm0, 0
1402b25a7: mov ecx, 0x14
1402b25ac: nop dword ptr [rax]
1402b25b0: movdqu xmmword ptr [rax + rcx*4 - 0x50], xmm0
1402b25b6: movdqu xmmword ptr [rax + rcx*4 - 0x40], xmm0
1402b25bc: movdqu xmmword ptr [rax + rcx*4 - 0x30], xmm0
1402b25c2: movdqu xmmword ptr [rax + rcx*4 - 0x20], xmm0
1402b25c8: movdqu xmmword ptr [rax + rcx*4 - 0x10], xmm0
1402b25ce: movdqu xmmword ptr [rax + rcx*4], xmm0
1402b25d3: add rcx, 0x18
1402b25d7: cmp rcx, 0x500c
1402b25de: jne 0x1402b25b0
1402b25e0: mov dword ptr [rax + 0x13fe0], r15d
1402b25e7: mov dword ptr [rax + 0x13fe4], r15d
1402b25ee: mov dword ptr [rax + 0x13fe8], r15d
1402b25f5: mov dword ptr [rax + 0x13fec], r15d
1402b25fc: mov dword ptr [rax + 0x13ff0], r15d
1402b2603: mov dword ptr [rax + 0x13ff4], r15d
1402b260a: mov dword ptr [rax + 0x13ff8], r15d
1402b2611: mov dword ptr [rax + 0x13ffc], r15d
1402b2618: cmp qword ptr [rbx + 0x18], 0x14000
1402b2620: mov qword ptr [rbp + 0x100], rax
1402b2627: jb 0x1402b2a0f
1402b262d: mov qword ptr [rbp - 0x10], 0
1402b2635: mov qword ptr [rbp - 8], 0x14000
1402b263d: mov r15, r12
1402b2640: mov qword ptr [rsp + 0x28], r12
1402b2645: mov qword ptr [rsp + 0x38], 0
1402b264e: mov qword ptr [rsp + 0x30], 1
1402b2657: mov qword ptr [rsp + 0x20], 0x14000
1402b2660: lea rcx, [rbp + 0x70]
1402b2664: mov rdx, r14
1402b2667: mov r8, rbx
1402b266a: mov r9, rax
1402b266d: call 0x140beefa0
1402b2672: mov rax, qword ptr [rbp + 0x70]
1402b2676: cmp rax, 0xb
1402b267a: jne 0x1402b2a16
1402b2680: mov edx, 0x14000
1402b2685: mov r8d, 4
1402b268b: mov rcx, qword ptr [rbp + 0x100]
1402b2692: call 0x140613c20
1402b2697: lea rcx, [rbp + 0x70]
1402b269b: call 0x140b19970
1402b26a0: mov eax, dword ptr [r13 + 0x1c]
1402b26a4: mov rcx, qword ptr [r13 + 0x10]
1402b26a8: mov dword ptr [rcx], eax
1402b26aa: mov rbx, qword ptr [r13]
1402b26ae: mov rdx, qword ptr [rbx]
1402b26b1: test rdx, rdx
1402b26b4: je 0x1402b26c9
1402b26b6: mov rcx, qword ptr [rbx + 8]
1402b26ba: shl rdx, 2
1402b26be: mov r8d, 4
1402b26c4: call 0x140613c20
1402b26c9: mov rax, qword ptr [rbp + 0x80]
1402b26d0: mov qword ptr [rbx + 0x10], rax
1402b26d4: movups xmm0, xmmword ptr [rbp + 0x70]
1402b26d8: movups xmmword ptr [rbx], xmm0
1402b26db: lea rdx, [rip + 0x26fbaee]
1402b26e2: mov rcx, rdi
1402b26e5: call 0x1405ca870
1402b26ea: test rax, rax
1402b26ed: je 0x1402b290a
1402b26f3: mov rdi, rax
1402b26f6: lea rax, [rip + 0x274a3f3]
1402b26fd: mov qword ptr [rbp + 0x20], rax
1402b2701: mov qword ptr [rbp + 0x28], 0x38
1402b2709: cmp qword ptr [rdi + 0x18], 0x38
1402b270e: jne 0x1402b28c8
1402b2714: mov rdx, qword ptr [rdi + 0x10]
1402b2718: lea rcx, [rip + 0x274a3d1]
1402b271f: mov r8d, 0x38
1402b2725: call 0x142923bf0
1402b272a: test eax, eax
1402b272c: jne 0x1402b28c8
1402b2732: mov rcx, qword ptr [rdi]
1402b2735: mov rax, qword ptr [rdi + 8]
1402b2739: call qword ptr [rax + 0x18]
1402b273c: test rax, rax
1402b273f: je 0x1402b290a
1402b2745: mov rdi, rax
1402b2748: mov rcx, rsi
1402b274b: call 0x140987ca0
1402b2750: mov rsi, rax
1402b2753: call 0x14099ca50
1402b2758: mov rbx, rax
1402b275b: test ebx, ebx
1402b275d: je 0x1402b2ab3
1402b2763: movzx eax, byte ptr [rip + 0x338249f]
1402b276a: mov ecx, 0x14000
1402b276f: mov edx, 4
1402b2774: call 0x140613c10
1402b2779: test rax, rax
1402b277c: je 0x1402b2abf
1402b2782: shr rbx, 0x20
1402b2786: movd xmm0, ebx
1402b278a: pshufd xmm0, xmm0, 0
1402b278f: mov ecx, 0x14
1402b2794: nop word ptr cs:[rax + rax]
1402b27a0: movdqu xmmword ptr [rax + rcx*4 - 0x50], xmm0
1402b27a6: movdqu xmmword ptr [rax + rcx*4 - 0x40], xmm0
1402b27ac: movdqu xmmword ptr [rax + rcx*4 - 0x30], xmm0
1402b27b2: movdqu xmmword ptr [rax + rcx*4 - 0x20], xmm0
1402b27b8: movdqu xmmword ptr [rax + rcx*4 - 0x10], xmm0
1402b27be: movdqu xmmword ptr [rax + rcx*4], xmm0
1402b27c3: add rcx, 0x18
1402b27c7: cmp rcx, 0x500c
1402b27ce: jne 0x1402b27a0
1402b27d0: mov dword ptr [rax + 0x13fe0], ebx
1402b27d6: mov dword ptr [rax + 0x13fe4], ebx
1402b27dc: mov dword ptr [rax + 0x13fe8], ebx
1402b27e2: mov dword ptr [rax + 0x13fec], ebx
1402b27e8: mov dword ptr [rax + 0x13ff0], ebx
1402b27ee: mov dword ptr [rax + 0x13ff4], ebx
1402b27f4: mov dword ptr [rax + 0x13ff8], ebx
1402b27fa: mov dword ptr [rax + 0x13ffc], ebx
1402b2800: cmp qword ptr [rdi + 0x18], 0x14000
1402b2808: mov qword ptr [rbp + 0x100], rax
1402b280f: jb 0x1402b2ad5
1402b2815: mov qword ptr [rbp - 0x10], 0
1402b281d: mov qword ptr [rbp - 8], 0x14000
1402b2825: mov qword ptr [rsp + 0x28], r15
1402b282a: mov qword ptr [rsp + 0x38], 0
1402b2833: mov qword ptr [rsp + 0x30], 1
1402b283c: mov qword ptr [rsp + 0x20], 0x14000
1402b2845: lea rcx, [rbp + 0x70]
1402b2849: mov rdx, rsi
1402b284c: mov r8, rdi
1402b284f: mov r9, rax
1402b2852: call 0x140beefa0
1402b2857: mov rax, qword ptr [rbp + 0x70]
1402b285b: cmp rax, 0xb
1402b285f: jne 0x1402b2adc
1402b2865: mov edx, 0x14000
1402b286a: mov r8d, 4
1402b2870: mov rcx, qword ptr [rbp + 0x100]
1402b2877: call 0x140613c20
1402b287c: movaps xmm6, xmmword ptr [rbp + 0x110]
1402b2883: movaps xmm7, xmmword ptr [rbp + 0x120]
1402b288a: add rsp, 0x1b8
1402b2891: pop rbx
1402b2892: pop rdi
1402b2893: pop rsi
1402b2894: pop r12
1402b2896: pop r13
1402b2898: pop r14
1402b289a: pop r15
1402b289c: pop rbp
1402b289d: ret 
1402b289e: mov qword ptr [rbp + 0x70], 0
1402b28a6: add rbx, 0x10
1402b28aa: lea rax, [rip + 0x26fb8e7]
1402b28b1: mov qword ptr [rsp + 0x20], rax
1402b28b6: lea rdx, [rbp + 0x20]
1402b28ba: lea r9, [rbp + 0x70]
1402b28be: xor ecx, ecx
1402b28c0: mov r8, rbx
1402b28c3: call 0x1428cab41
1402b28c8: mov qword ptr [rbp + 0x70], 0
1402b28d0: add rdi, 0x10
1402b28d4: lea rax, [rip + 0x26fb8bd]
1402b28db: mov qword ptr [rsp + 0x20], rax
1402b28e0: lea rdx, [rbp + 0x20]
1402b28e4: lea r9, [rbp + 0x70]
1402b28e8: xor ecx, ecx
1402b28ea: mov r8, rdi
1402b28ed: call 0x1428cab41
1402b28f2: lea rcx, [rip + 0x26fb487]
1402b28f9: call 0x1428d9310
1402b28fe: lea rcx, [rip + 0x26fb4db]
1402b2905: call 0x1428d9310
1402b290a: lea rcx, [rip + 0x26fb52f]
1402b2911: call 0x1428d9310
1402b2916: lea rcx, [rip + 0x26800f3]
1402b291d: call 0x1428d80d0
1402b2922: jmp 0x1402b2b74
1402b2927: lea rcx, [rip + 0x26fb46a]
1402b292e: call 0x1428d9310
1402b2933: lea r8, [rip + 0x26fb476]
1402b293a: mov ecx, 4
1402b293f: mov edx, 0x14000
1402b2944: call 0x1428d8fe3
1402b2949: mov eax, 0xb
1402b294e: jmp 0x1402b2986
1402b2950: mov rcx, qword ptr [rbp + 0xb8]
1402b2957: mov qword ptr [rbp + 0x60], rcx
1402b295b: movdqu xmm0, xmmword ptr [rbp + 0x78]
1402b2960: movups xmm1, xmmword ptr [rbp + 0x88]
1402b2967: movups xmm2, xmmword ptr [rbp + 0x98]
1402b296e: movups xmm3, xmmword ptr [rbp + 0xa8]
1402b2975: movaps xmmword ptr [rbp + 0x50], xmm3
1402b2979: movaps xmmword ptr [rbp + 0x40], xmm2
1402b297d: movaps xmmword ptr [rbp + 0x30], xmm1
1402b2981: movdqa xmmword ptr [rbp + 0x20], xmm0
1402b2986: mov qword ptr [rbp + 0x70], rax
1402b298a: movdqa xmm0, xmmword ptr [rbp + 0x20]
1402b298f: movaps xmm1, xmmword ptr [rbp + 0x30]
1402b2993: movaps xmm2, xmmword ptr [rbp + 0x40]
1402b2997: movaps xmm3, xmmword ptr [rbp + 0x50]
1402b299b: movdqu xmmword ptr [rbp + 0x78], xmm0
1402b29a0: movups xmmword ptr [rbp + 0x88], xmm1
1402b29a7: movups xmmword ptr [rbp + 0x98], xmm2
1402b29ae: movups xmmword ptr [rbp + 0xa8], xmm3
1402b29b5: mov rax, qword ptr [rbp + 0x60]
1402b29b9: mov qword ptr [rbp + 0xb8], rax
1402b29c0: lea rax, [rip + 0x26fb401]
1402b29c7: mov qword ptr [rsp + 0x20], rax
1402b29cc: lea rcx, [rip + 0x26fac5d]
1402b29d3: lea r9, [rip + 0x26fac86]
1402b29da: lea r8, [rbp + 0x70]
1402b29de: mov edx, 0x2b
1402b29e3: call 0x1428d9760
1402b29e8: jmp 0x1402b2b74
1402b29ed: lea rcx, [rip + 0x26fb404]
1402b29f4: call 0x1428d9310
1402b29f9: lea r8, [rip + 0x26fb410]
1402b2a00: mov ecx, 4
1402b2a05: mov edx, 0x14000
1402b2a0a: call 0x1428d8fe3
1402b2a0f: mov eax, 0xb
1402b2a14: jmp 0x1402b2a4c
1402b2a16: mov rcx, qword ptr [rbp + 0xb8]
1402b2a1d: mov qword ptr [rbp + 0x60], rcx
1402b2a21: movdqu xmm0, xmmword ptr [rbp + 0x78]
1402b2a26: movups xmm1, xmmword ptr [rbp + 0x88]
1402b2a2d: movups xmm2, xmmword ptr [rbp + 0x98]
1402b2a34: movups xmm3, xmmword ptr [rbp + 0xa8]
1402b2a3b: movaps xmmword ptr [rbp + 0x50], xmm3
1402b2a3f: movaps xmmword ptr [rbp + 0x40], xmm2
1402b2a43: movaps xmmword ptr [rbp + 0x30], xmm1
1402b2a47: movdqa xmmword ptr [rbp + 0x20], xmm0
1402b2a4c: mov qword ptr [rbp + 0x70], rax
1402b2a50: movdqa xmm0, xmmword ptr [rbp + 0x20]
1402b2a55: movaps xmm1, xmmword ptr [rbp + 0x30]
1402b2a59: movaps xmm2, xmmword ptr [rbp + 0x40]
1402b2a5d: movaps xmm3, xmmword ptr [rbp + 0x50]
1402b2a61: movdqu xmmword ptr [rbp + 0x78], xmm0
1402b2a66: movups xmmword ptr [rbp + 0x88], xmm1
1402b2a6d: movups xmmword ptr [rbp + 0x98], xmm2
1402b2a74: movups xmmword ptr [rbp + 0xa8], xmm3
1402b2a7b: mov rax, qword ptr [rbp + 0x60]
1402b2a7f: mov qword ptr [rbp + 0xb8], rax
1402b2a86: lea rax, [rip + 0x26fb39b]
1402b2a8d: mov qword ptr [rsp + 0x20], rax
1402b2a92: lea rcx, [rip + 0x26fab97]
1402b2a99: lea r9, [rip + 0x26fabc0]
1402b2aa0: lea r8, [rbp + 0x70]
1402b2aa4: mov edx, 0x2b
1402b2aa9: call 0x1428d9760
1402b2aae: jmp 0x1402b2b74
1402b2ab3: lea rcx, [rip + 0x26fb39e]
1402b2aba: call 0x1428d9310
1402b2abf: lea r8, [rip + 0x26fb3aa]
1402b2ac6: mov ecx, 4
1402b2acb: mov edx, 0x14000
1402b2ad0: call 0x1428d8fe3
1402b2ad5: mov eax, 0xb
1402b2ada: jmp 0x1402b2b12
1402b2adc: mov rcx, qword ptr [rbp + 0xb8]
1402b2ae3: mov qword ptr [rbp + 0x60], rcx
1402b2ae7: movdqu xmm0, xmmword ptr [rbp + 0x78]
1402b2aec: movups xmm1, xmmword ptr [rbp + 0x88]
1402b2af3: movups xmm2, xmmword ptr [rbp + 0x98]
1402b2afa: movups xmm3, xmmword ptr [rbp + 0xa8]
1402b2b01: movaps xmmword ptr [rbp + 0x50], xmm3
1402b2b05: movaps xmmword ptr [rbp + 0x40], xmm2
1402b2b09: movaps xmmword ptr [rbp + 0x30], xmm1
1402b2b0d: movdqa xmmword ptr [rbp + 0x20], xmm0
1402b2b12: mov qword ptr [rbp + 0x70], rax
1402b2b16: movdqa xmm0, xmmword ptr [rbp + 0x20]
1402b2b1b: movaps xmm1, xmmword ptr [rbp + 0x30]
1402b2b1f: movaps xmm2, xmmword ptr [rbp + 0x40]
1402b2b23: movaps xmm3, xmmword ptr [rbp + 0x50]
1402b2b27: movdqu xmmword ptr [rbp + 0x78], xmm0
1402b2b2c: movups xmmword ptr [rbp + 0x88], xmm1
1402b2b33: movups xmmword ptr [rbp + 0x98], xmm2
1402b2b3a: movups xmmword ptr [rbp + 0xa8], xmm3
1402b2b41: mov rax, qword ptr [rbp + 0x60]
1402b2b45: mov qword ptr [rbp + 0xb8], rax
1402b2b4c: lea rax, [rip + 0x26fb335]
1402b2b53: mov qword ptr [rsp + 0x20], rax
1402b2b58: lea rcx, [rip + 0x26faad1]
1402b2b5f: lea r9, [rip + 0x26faafa]
1402b2b66: lea r8, [rbp + 0x70]
1402b2b6a: mov edx, 0x2b
1402b2b6f: call 0x1428d9760
1402b2b74: ud2 
1402b2b76: nop word ptr cs:[rax + rax]
1402b2b80: mov qword ptr [rsp + 0x10], rdx
1402b2b85: push rbp
1402b2b86: push r15
1402b2b88: push r14
1402b2b8a: push r13
1402b2b8c: push r12
1402b2b8e: push rsi
1402b2b8f: push rdi
1402b2b90: push rbx
1402b2b91: sub rsp, 0x68
1402b2b95: lea rbp, [rdx + 0x80]
1402b2b9c: movaps xmmword ptr [rsp + 0x40], xmm7
1402b2ba1: movaps xmmword ptr [rsp + 0x50], xmm6
1402b2ba6: lea rcx, [rbp - 0x10]
1402b2baa: mov r8d, 0x2a8
1402b2bb0: mov r9d, 0x10
1402b2bb6: lea rdx, [rbp + 0x10]
1402b2bba: call 0x1400523e0
1402b2bbf: movaps xmm6, xmmword ptr [rsp + 0x50]
1402b2bc4: movaps xmm7, xmmword ptr [rsp + 0x40]
1402b2bc9: add rsp, 0x68
1402b2bcd: pop rbx
1402b2bce: pop rdi
1402b2bcf: pop rsi
1402b2bd0: pop r12
1402b2bd2: pop r13
1402b2bd4: pop r14
1402b2bd6: pop r15
1402b2bd8: pop rbp
1402b2bd9: ret 
1402b2bda: nop word ptr [rax + rax]
1402b2be0: mov qword ptr [rsp + 0x10], rdx
1402b2be5: push rbp
1402b2be6: push r15
1402b2be8: push r14
1402b2bea: push r13
1402b2bec: push r12
1402b2bee: push rsi
1402b2bef: push rdi
1402b2bf0: push rbx
1402b2bf1: sub rsp, 0x68
1402b2bf5: lea rbp, [rdx + 0x80]
1402b2bfc: movaps xmmword ptr [rsp + 0x40], xmm7
1402b2c01: movaps xmmword ptr [rsp + 0x50], xmm6
1402b2c06: lea rcx, [rbp + 0x20]
1402b2c0a: call 0x140149c70
1402b2c0f: movaps xmm6, xmmword ptr [rsp + 0x50]
1402b2c14: movaps xmm7, xmmword ptr [rsp + 0x40]
1402b2c19: add rsp, 0x68
1402b2c1d: pop rbx
1402b2c1e: pop rdi
1402b2c1f: pop rsi
1402b2c20: pop r12
1402b2c22: pop r13
1402b2c24: pop r14
1402b2c26: pop r15
1402b2c28: pop rbp
1402b2c29: ret 
1402b2c2a: nop word ptr [rax + rax]
1402b2c30: mov qword ptr [rsp + 0x10], rdx
1402b2c35: push rbp
1402b2c36: push r15
1402b2c38: push r14
1402b2c3a: push r13
1402b2c3c: push r12
1402b2c3e: push rsi
1402b2c3f: push rdi
1402b2c40: push rbx
1402b2c41: sub rsp, 0x68
1402b2c45: lea rbp, [rdx + 0x80]
1402b2c4c: movaps xmmword ptr [rsp + 0x40], xmm7
1402b2c51: movaps xmmword ptr [rsp + 0x50], xmm6
1402b2c56: lea rcx, [rbp + 0x20]
1402b2c5a: mov r8d, 0x20
1402b2c60: mov r9d, 0x10
1402b2c66: lea rdx, [rbp + 0x40]
1402b2c6a: call 0x1400520b0
1402b2c6f: movaps xmm6, xmmword ptr [rsp + 0x50]
1402b2c74: movaps xmm7, xmmword ptr [rsp + 0x40]
1402b2c79: add rsp, 0x68
1402b2c7d: pop rbx
1402b2c7e: pop rdi
1402b2c7f: pop rsi
1402b2c80: pop r12
1402b2c82: pop r13
1402b2c84: pop r14
1402b2c86: pop r15
1402b2c88: pop rbp
1402b2c89: ret 
1402b2c8a: nop word ptr [rax + rax]
1402b2c90: mov qword ptr [rsp + 0x10], rdx
1402b2c95: push rbp
1402b2c96: push r15
1402b2c98: push r14
1402b2c9a: push r13
1402b2c9c: push r12
1402b2c9e: push rsi
1402b2c9f: push rdi
1402b2ca0: push rbx
1402b2ca1: sub rsp, 0x68
1402b2ca5: lea rbp, [rdx + 0x80]
1402b2cac: movaps xmmword ptr [rsp + 0x40], xmm7
1402b2cb1: movaps xmmword ptr [rsp + 0x50], xmm6
1402b2cb6: lea rcx, [rbp + 0x70]
1402b2cba: mov r8d, 0x20
1402b2cc0: mov r9d, 0x10
1402b2cc6: lea rdx, [rbp + 0x90]
1402b2ccd: call 0x14004ec10
1402b2cd2: movaps xmm6, xmmword ptr [rsp + 0x50]
1402b2cd7: movaps xmm7, xmmword ptr [rsp + 0x40]
1402b2cdc: add rsp, 0x68
1402b2ce0: pop rbx
1402b2ce1: pop rdi
1402b2ce2: pop rsi
1402b2ce3: pop r12
1402b2ce5: pop r13
1402b2ce7: pop r14
1402b2ce9: pop r15
1402b2ceb: pop rbp
1402b2cec: ret 
1402b2ced: nop dword ptr [rax]
1402b2cf0: mov qword ptr [rsp + 0x10], rdx
1402b2cf5: push rbp
1402b2cf6: push r15
1402b2cf8: push r14
1402b2cfa: push r13
1402b2cfc: push r12
1402b2cfe: push rsi
1402b2cff: push rdi
1402b2d00: push rbx
1402b2d01: sub rsp, 0x68
1402b2d05: lea rbp, [rdx + 0x80]
1402b2d0c: movaps xmmword ptr [rsp + 0x40], xmm7
1402b2d11: movaps xmmword ptr [rsp + 0x50], xmm6
1402b2d16: mov rcx, qword ptr [rbp + 0xf8]
1402b2d1d: mov qword ptr [rcx], 0
1402b2d24: mov qword ptr [rcx + 8], 8
1402b2d2c: mov qword ptr [rcx + 0x10], 0
1402b2d34: movaps xmm0, xmmword ptr [rbp - 0x10]
1402b2d38: movaps xmm1, xmmword ptr [rbp]
1402b2d3c: movaps xmm2, xmmword ptr [rbp + 0x10]
1402b2d40: movups xmmword ptr [rcx + 0x18], xmm0
1402b2d44: movups xmmword ptr [rcx + 0x28], xmm1
1402b2d48: movups xmmword ptr [rcx + 0x38], xmm2
1402b2d4c: movaps xmm0, xmmword ptr [rbp - 0x40]
1402b2d50: movaps xmm1, xmmword ptr [rbp - 0x30]
1402b2d54: movaps xmm2, xmmword ptr [rbp - 0x20]
1402b2d58: movups xmmword ptr [rcx + 0x48], xmm0
1402b2d5c: movups xmmword ptr [rcx + 0x58], xmm1
1402b2d60: movups xmmword ptr [rcx + 0x68], xmm2
1402b2d64: movups xmm0, xmmword ptr [rip + 0x26f0b35]
1402b2d6b: movups xmmword ptr [rcx + 0x78], xmm0
1402b2d6f: movups xmm0, xmmword ptr [rip + 0x26f0b3a]
1402b2d76: movups xmmword ptr [rcx + 0x88], xmm0
1402b2d7d: mov rax, qword ptr [rbp + 0xf0]
1402b2d84: mov qword ptr [rcx + 0x98], rax
1402b2d8b: mov rax, qword ptr [rbp + 0x100]
1402b2d92: mov qword ptr [rcx + 0xa0], rax
1402b2d99: movaps xmm6, xmmword ptr [rsp + 0x50]
1402b2d9e: movaps xmm7, xmmword ptr [rsp + 0x40]
1402b2da3: add rsp, 0x68
1402b2da7: pop rbx
1402b2da8: pop rdi
1402b2da9: pop rsi
1402b2daa: pop r12
1402b2dac: pop r13
1402b2dae: pop r14
1402b2db0: pop r15
1402b2db2: pop rbp
1402b2db3: ret 
1402b2db4: nop word ptr cs:[rax + rax]
1402b2dc0: mov qword ptr [rsp + 0x10], rdx
1402b2dc5: push rbp
1402b2dc6: push r15
1402b2dc8: push r14
1402b2dca: push r13
1402b2dcc: push r12
1402b2dce: push rsi
1402b2dcf: push rdi
1402b2dd0: push rbx
1402b2dd1: sub rsp, 0x68
1402b2dd5: lea rbp, [rdx + 0x80]
1402b2ddc: movaps xmmword ptr [rsp + 0x40], xmm7
1402b2de1: movaps xmmword ptr [rsp + 0x50], xmm6
1402b2de6: mov edx, 0x14000
1402b2deb: mov r8d, 4
1402b2df1: mov rcx, qword ptr [rbp + 0x100]
1402b2df8: call 0x140613c20
1402b2dfd: movaps xmm6, xmmword ptr [rsp + 0x50]
1402b2e02: movaps xmm7, xmmword ptr [rsp + 0x40]
1402b2e07: add rsp, 0x68
1402b2e0b: pop rbx
1402b2e0c: pop rdi
1402b2e0d: pop rsi
1402b2e0e: pop r12
1402b2e10: pop r13
1402b2e12: pop r14
1402b2e14: pop r15
1402b2e16: pop rbp
1402b2e17: ret 
1402b2e18: nop dword ptr [rax + rax]
1402b2e20: mov qword ptr [rsp + 0x10], rdx
1402b2e25: push rbp
1402b2e26: push r15
1402b2e28: push r14
1402b2e2a: push r13
1402b2e2c: push r12
1402b2e2e: push rsi
1402b2e2f: push rdi
1402b2e30: push rbx
1402b2e31: sub rsp, 0x68
1402b2e35: lea rbp, [rdx + 0x80]
1402b2e3c: movaps xmmword ptr [rsp + 0x40], xmm7
1402b2e41: movaps xmmword ptr [rsp + 0x50], xmm6
1402b2e46: mov edx, 0x14000
1402b2e4b: mov r8d, 4
1402b2e51: mov rcx, qword ptr [rbp + 0x100]
1402b2e58: call 0x140613c20
1402b2e5d: movaps xmm6, xmmword ptr [rsp + 0x50]
1402b2e62: movaps xmm7, xmmword ptr [rsp + 0x40]
1402b2e67: add rsp, 0x68
1402b2e6b: pop rbx
1402b2e6c: pop rdi
1402b2e6d: pop rsi
1402b2e6e: pop r12
1402b2e70: pop r13
1402b2e72: pop r14
1402b2e74: pop r15
1402b2e76: pop rbp
1402b2e77: ret 
1402b2e78: nop dword ptr [rax + rax]
1402b2e80: mov qword ptr [rsp + 0x10], rdx
1402b2e85: push rbp
1402b2e86: push r15
1402b2e88: push r14
1402b2e8a: push r13
1402b2e8c: push r12
1402b2e8e: push rsi
1402b2e8f: push rdi
1402b2e90: push rbx
1402b2e91: sub rsp, 0x68
1402b2e95: lea rbp, [rdx + 0x80]
1402b2e9c: movaps xmmword ptr [rsp + 0x40], xmm7
1402b2ea1: movaps xmmword ptr [rsp + 0x50], xmm6
1402b2ea6: mov edx, 0x14000
1402b2eab: mov r8d, 4
1402b2eb1: mov rcx, qword ptr [rbp + 0x100]
1402b2eb8: call 0x140613c20
1402b2ebd: movaps xmm6, xmmword ptr [rsp + 0x50]
1402b2ec2: movaps xmm7, xmmword ptr [rsp + 0x40]
1402b2ec7: add rsp, 0x68
1402b2ecb: pop rbx
1402b2ecc: pop rdi
1402b2ecd: pop rsi
1402b2ece: pop r12
1402b2ed0: pop r13
1402b2ed2: pop r14
1402b2ed4: pop r15
1402b2ed6: pop rbp
1402b2ed7: ret 
1402b2ed8: nop dword ptr [rax + rax]
1402b2ee0: mov qword ptr [rsp + 0x10], rdx
1402b2ee5: push rbp
1402b2ee6: push r15
1402b2ee8: push r14
1402b2eea: push r13
1402b2eec: push r12
1402b2eee: push rsi
1402b2eef: push rdi
1402b2ef0: push rbx
1402b2ef1: sub rsp, 0x68
1402b2ef5: lea rbp, [rdx + 0x80]
1402b2efc: movaps xmmword ptr [rsp + 0x40], xmm7
1402b2f01: movaps xmmword ptr [rsp + 0x50], xmm6
1402b2f06: movaps xmm0, xmmword ptr [rbp + 0xe0]
1402b2f0d: mov rcx, qword ptr [rbp + 0xf8]
1402b2f14: movups xmmword ptr [rcx + 0x70], xmm0
1402b2f18: movaps xmm0, xmmword ptr [rbp + 0xd0]
1402b2f1f: movups xmmword ptr [rcx + 0x60], xmm0
1402b2f23: movaps xmm0, xmmword ptr [rbp + 0xc0]
1402b2f2a: movups xmmword ptr [rcx + 0x50], xmm0
1402b2f2e: movaps xmm0, xmmword ptr [rbp + 0xb0]
1402b2f35: movups xmmword ptr [rcx + 0x40], xmm0
1402b2f39: movaps xmm0, xmmword ptr [rbp + 0x70]
1402b2f3d: movaps xmm1, xmmword ptr [rbp + 0x80]
1402b2f44: movaps xmm2, xmmword ptr [rbp + 0x90]
1402b2f4b: movaps xmm3, xmmword ptr [rbp + 0xa0]
1402b2f52: movups xmmword ptr [rcx + 0x30], xmm3
1402b2f56: movups xmmword ptr [rcx + 0x20], xmm2
1402b2f5a: movups xmmword ptr [rcx + 0x10], xmm1
1402b2f5e: movups xmmword ptr [rcx], xmm0
1402b2f61: mov rax, qword ptr [rbp + 0xf0]
1402b2f68: mov qword ptr [rcx + 0x80], rax
1402b2f6f: mov rax, qword ptr [rbp + 0x100]
1402b2f76: mov qword ptr [rcx + 0x88], rax
1402b2f7d: movaps xmm6, xmmword ptr [rsp + 0x50]
1402b2f82: movaps xmm7, xmmword ptr [rsp + 0x40]
1402b2f87: add rsp, 0x68
1402b2f8b: pop rbx
1402b2f8c: pop rdi
1402b2f8d: pop rsi
1402b2f8e: pop r12
1402b2f90: pop r13
1402b2f92: pop r14
1402b2f94: pop r15
1402b2f96: pop rbp
1402b2f97: ret 
1402b2f98: nop dword ptr [rax + rax]
1402b2fa0: mov qword ptr [rsp + 0x10], rdx
1402b2fa5: push rbp
1402b2fa6: push r15
1402b2fa8: push r14
1402b2faa: push r13
1402b2fac: push r12
1402b2fae: push rsi
1402b2faf: push rdi
1402b2fb0: push rbx
1402b2fb1: sub rsp, 0x68
1402b2fb5: lea rbp, [rdx + 0x80]
1402b2fbc: movaps xmmword ptr [rsp + 0x40], xmm7
1402b2fc1: movaps xmmword ptr [rsp + 0x50], xmm6
1402b2fc6: movups xmm0, xmmword ptr [rip + 0x26faad3]
1402b2fcd: mov rcx, qword ptr [rbp + 0xf8]
1402b2fd4: movups xmmword ptr [rcx + 0x10], xmm0
1402b2fd8: movups xmm0, xmmword ptr [rip + 0x26faab1]
1402b2fdf: movups xmmword ptr [rcx], xmm0
1402b2fe2: mov rax, qword ptr [rbp + 0xf0]
1402b2fe9: mov qword ptr [rcx + 0x20], rax
1402b2fed: mov rax, qword ptr [rbp + 0x100]
1402b2ff4: mov qword ptr [rcx + 0x28], rax
1402b2ff8: movaps xmm6, xmmword ptr [rsp + 0x50]
1402b2ffd: movaps xmm7, xmmword ptr [rsp + 0x40]
1402b3002: add rsp, 0x68
1402b3006: pop rbx
1402b3007: pop rdi
1402b3008: pop rsi
1402b3009: pop r12
1402b300b: pop r13
1402b300d: pop r14
1402b300f: pop r15
1402b3011: pop rbp
1402b3012: ret 
1402b3013: nop word ptr cs:[rax + rax]
1402b3020: mov qword ptr [rsp + 0x10], rdx
1402b3025: push rbp
1402b3026: push r15
1402b3028: push r14
1402b302a: push r13
1402b302c: push r12
1402b302e: push rsi
1402b302f: push rdi
1402b3030: push rbx
1402b3031: sub rsp, 0x68
1402b3035: lea rbp, [rdx + 0x80]
1402b303c: movaps xmmword ptr [rsp + 0x40], xmm7
1402b3041: movaps xmmword ptr [rsp + 0x50], xmm6
1402b3046: movups xmm0, xmmword ptr [rbp + 0xc0]
1402b304d: mov rax, qword ptr [rbp + 0x100]
1402b3054: movups xmmword ptr [rax + 0x50], xmm0
1402b3058: movups xmm0, xmmword ptr [rbp + 0xb0]
1402b305f: movups xmmword ptr [rax + 0x40], xmm0
1402b3063: movups xmm0, xmmword ptr [rbp + 0x70]
1402b3067: movups xmm1, xmmword ptr [rbp + 0x80]
1402b306e: movups xmm2, xmmword ptr [rbp + 0x90]
1402b3075: movups xmm3, xmmword ptr [rbp + 0xa0]
1402b307c: movups xmmword ptr [rax + 0x30], xmm3
1402b3080: movups xmmword ptr [rax + 0x20], xmm2
1402b3084: movups xmmword ptr [rax + 0x10], xmm1
1402b3088: movups xmmword ptr [rax], xmm0
1402b308b: movaps xmm6, xmmword ptr [rsp + 0x50]
1402b3090: movaps xmm7, xmmword ptr [rsp + 0x40]
1402b3095: add rsp, 0x68
1402b3099: pop rbx
1402b309a: pop rdi
1402b309b: pop rsi
1402b309c: pop r12
1402b309e: pop r13
1402b30a0: pop r14
1402b30a2: pop r15
1402b30a4: pop rbp
1402b30a5: ret 
1402b30a6: nop word ptr cs:[rax + rax]
1402b30b0: mov qword ptr [rsp + 0x10], rdx
1402b30b5: push rbp
1402b30b6: push r15
1402b30b8: push r14
1402b30ba: push r13
1402b30bc: push r12
1402b30be: push rsi
1402b30bf: push rdi
1402b30c0: push rbx
1402b30c1: sub rsp, 0x68
1402b30c5: lea rbp, [rdx + 0x80]
1402b30cc: movaps xmmword ptr [rsp + 0x40], xmm7
1402b30d1: movaps xmmword ptr [rsp + 0x50], xmm6
1402b30d6: lea rcx, [rbp + 0x70]
1402b30da: call 0x1402b1200
1402b30df: movaps xmm6, xmmword ptr [rsp + 0x50]
1402b30e4: movaps xmm7, xmmword ptr [rsp + 0x40]
1402b30e9: add rsp, 0x68
1402b30ed: pop rbx
1402b30ee: pop rdi
1402b30ef: pop rsi
1402b30f0: pop r12
1402b30f2: pop r13
1402b30f4: pop r14
1402b30f6: pop r15
1402b30f8: pop rbp
1402b30f9: ret 
1402b30fa: nop word ptr [rax + rax]
1402b3100: mov qword ptr [rsp + 0x10], rdx
1402b3105: push rbp
1402b3106: push r15
1402b3108: push r14
1402b310a: push r13
1402b310c: push r12
1402b310e: push rsi
1402b310f: push rdi
1402b3110: push rbx
1402b3111: sub rsp, 0x68
1402b3115: lea rbp, [rdx + 0x80]
1402b311c: movaps xmmword ptr [rsp + 0x40], xmm7
1402b3121: movaps xmmword ptr [rsp + 0x50], xmm6
1402b3126: lea rcx, [rbp + 0x70]
1402b312a: call 0x1402b1200
1402b312f: movaps xmm6, xmmword ptr [rsp + 0x50]
1402b3134: movaps xmm7, xmmword ptr [rsp + 0x40]
1402b3139: add rsp, 0x68
1402b313d: pop rbx
1402b313e: pop rdi
1402b313f: pop rsi
1402b3140: pop r12
1402b3142: pop r13
1402b3144: pop r14
1402b3146: pop r15
1402b3148: pop rbp
1402b3149: ret 
1402b314a: nop word ptr [rax + rax]
1402b3150: mov qword ptr [rsp + 0x10], rdx
1402b3155: push rbp
1402b3156: push r15
1402b3158: push r14
1402b315a: push r13
1402b315c: push r12
1402b315e: push rsi
1402b315f: push rdi
1402b3160: push rbx
1402b3161: sub rsp, 0x68
1402b3165: lea rbp, [rdx + 0x80]
1402b316c: movaps xmmword ptr [rsp + 0x40], xmm7
1402b3171: movaps xmmword ptr [rsp + 0x50], xmm6
1402b3176: lea rcx, [rbp + 0x70]
1402b317a: call 0x1402b1200
1402b317f: movaps xmm6, xmmword ptr [rsp + 0x50]
1402b3184: movaps xmm7, xmmword ptr [rsp + 0x40]
1402b3189: add rsp, 0x68
1402b318d: pop rbx
1402b318e: pop rdi
1402b318f: pop rsi
1402b3190: pop r12
1402b3192: pop r13
1402b3194: pop r14
1402b3196: pop r15
1402b3198: pop rbp
1402b3199: ret 
