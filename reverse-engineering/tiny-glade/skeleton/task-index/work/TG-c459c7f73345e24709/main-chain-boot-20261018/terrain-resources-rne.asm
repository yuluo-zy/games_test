1409457c0: push rbp
1409457c1: push rsi
1409457c2: sub rsp, 0xb8
1409457c9: lea rbp, [rsp + 0x80]
1409457d1: movaps xmmword ptr [rbp + 0x20], xmm6
1409457d5: mov qword ptr [rbp + 0x18], 0xfffffffffffffffe
1409457dd: movsd xmm0, qword ptr [rip + 0x216cbdb] ; 0x142ab23c0 0000000000007a440000000000000000
1409457e5: movsd qword ptr [rbp - 0x60], xmm0
1409457ea: mov byte ptr [rbp - 0x58], 0
1409457ee: lea rdx, [rbp - 0x60]
1409457f2: call 0x14153dcb0
1409457f7: xorps xmm0, xmm0
1409457fa: movaps xmmword ptr [rbp - 0x60], xmm0
1409457fe: mov dword ptr [rbp - 0x50], 0
140945805: lea rdx, [rbp - 0x60]
140945809: mov rcx, rax
14094580c: call 0x141546370
140945811: mov rsi, rax
140945814: movzx eax, byte ptr [rip + 0x2cef3ee] ; 0x143634c09 
14094581b: mov ecx, 0x80000
140945820: mov edx, 4
140945825: call 0x140613c10
14094582a: test rax, rax
14094582d: je 0x14094596b
140945833: mov qword ptr [rbp], 0x20000
14094583b: mov qword ptr [rbp + 8], rax
14094583f: mov qword ptr [rbp + 0x10], 0
140945847: lea r9, [rip + 0x2159cba] ; 0x142a9f508 d0f4a942010000003300000000000000
14094584e: mov rcx, rbp
140945851: mov edx, 0x20000
140945856: xor r8d, r8d
140945859: call 0x1412f3e20
14094585e: mov rax, qword ptr [rbp + 0x10]
140945862: mov qword ptr [rbp - 0x50], rax
140945866: movups xmm0, xmmword ptr [rbp]
14094586a: movaps xmmword ptr [rbp - 0x60], xmm0
14094586e: xorps xmm0, xmm0
140945871: movups xmmword ptr [rbp - 0x38], xmm0
140945875: movaps xmmword ptr [rbp - 0x20], xmm0
140945879: movsd xmm6, qword ptr [rip + 0x2159b6f] ; 0x142a9f3f0 000020c00000c0410000000000000000
140945881: movsd qword ptr [rbp - 0x10], xmm6
140945886: mov qword ptr [rbp - 0x48], 0
14094588e: mov qword ptr [rbp - 0x40], 8
140945896: mov qword ptr [rbp - 0x28], 8
14094589e: lea rdx, [rbp - 0x60]
1409458a2: mov rcx, rsi
1409458a5: call 0x141548b60
1409458aa: mov rsi, rax
1409458ad: movzx eax, byte ptr [rip + 0x2cef355] ; 0x143634c09 
1409458b4: mov ecx, 0x80000
1409458b9: mov edx, 4
1409458be: call 0x140613c10
1409458c3: test rax, rax
1409458c6: je 0x140945981
1409458cc: mov qword ptr [rbp], 0x20000
1409458d4: mov qword ptr [rbp + 8], rax
1409458d8: mov qword ptr [rbp + 0x10], 0
1409458e0: lea r9, [rip + 0x2159c39] ; 0x142a9f520 d0f4a942010000003300000000000000
1409458e7: mov rcx, rbp
1409458ea: mov edx, 0x20000
1409458ef: xor r8d, r8d
1409458f2: call 0x1412f3e20
1409458f7: mov rax, qword ptr [rbp + 0x10]
1409458fb: mov qword ptr [rbp - 0x50], rax
1409458ff: movups xmm0, xmmword ptr [rbp]
140945903: movaps xmmword ptr [rbp - 0x60], xmm0
140945907: xorps xmm0, xmm0
14094590a: movups xmmword ptr [rbp - 0x38], xmm0
14094590e: movaps xmmword ptr [rbp - 0x20], xmm0
140945912: movsd qword ptr [rbp - 0x10], xmm6
140945917: mov qword ptr [rbp - 0x48], 0
14094591f: mov qword ptr [rbp - 0x40], 8
140945927: mov qword ptr [rbp - 0x28], 8
14094592f: lea rdx, [rbp - 0x60]
140945933: mov rcx, rsi
140945936: call 0x141537ba0
14094593b: mov rcx, rax
14094593e: xor edx, edx
140945940: call 0x141541f80
140945945: mov qword ptr [rbp - 0x60], 0
14094594d: mov byte ptr [rbp - 0x58], 0
140945951: lea rdx, [rbp - 0x60]
140945955: mov rcx, rax
140945958: call 0x141535af0
14094595d: movaps xmm6, xmmword ptr [rbp + 0x20]
140945961: add rsp, 0xb8
140945968: pop rsi
140945969: pop rbp
14094596a: ret 
14094596b: lea r8, [rip + 0x2159b96] ; 0x142a9f508 d0f4a942010000003300000000000000
140945972: mov ecx, 4
140945977: mov edx, 0x80000
14094597c: call 0x1428d8fe3
140945981: lea r8, [rip + 0x2159b98] ; 0x142a9f520 d0f4a942010000003300000000000000
140945988: mov ecx, 4
14094598d: mov edx, 0x80000
140945992: call 0x1428d8fe3
140945997: int3 
140945998: nop dword ptr [rax + rax]
1409459a0: mov qword ptr [rsp + 0x10], rdx
1409459a5: push rbp
1409459a6: push rsi
1409459a7: sub rsp, 0x38
1409459ab: lea rbp, [rdx + 0x80]
1409459b2: movaps xmmword ptr [rsp + 0x20], xmm6
1409459b7: mov rdx, qword ptr [rbp]
1409459bb: test rdx, rdx
1409459be: je 0x1409459d3
1409459c0: mov rcx, qword ptr [rbp + 8]
1409459c4: shl rdx, 2
1409459c8: mov r8d, 4
1409459ce: call 0x140613c20
1409459d3: movaps xmm6, xmmword ptr [rsp + 0x20]
1409459d8: add rsp, 0x38
1409459dc: pop rsi
1409459dd: pop rbp
1409459de: ret 
1409459df: nop 
1409459e0: mov qword ptr [rsp + 0x10], rdx
1409459e5: push rbp
1409459e6: push rsi
1409459e7: sub rsp, 0x38
1409459eb: lea rbp, [rdx + 0x80]
1409459f2: movaps xmmword ptr [rsp + 0x20], xmm6
1409459f7: mov rdx, qword ptr [rbp]
1409459fb: test rdx, rdx
1409459fe: je 0x140945a13
140945a00: mov rcx, qword ptr [rbp + 8]
140945a04: shl rdx, 2
140945a08: mov r8d, 4
140945a0e: call 0x140613c20
140945a13: movaps xmm6, xmmword ptr [rsp + 0x20]
140945a18: add rsp, 0x38
140945a1c: pop rsi
140945a1d: pop rbp
140945a1e: ret 
140945a1f: int3 
140945a20: push rsi
140945a21: push rdi
140945a22: sub rsp, 0x28
140945a26: mov rsi, rcx
140945a29: lea rdi, [rcx + 0x38]
140945a2d: movabs rdx, 0x4cf482123ffcebbb
140945a37: movabs r8, 0xe009867fb3b8f480
140945a41: mov rcx, rdi
140945a44: call 0x1400dd6f0
140945a49: test rax, rax
140945a4c: je 0x140945a81
140945a4e: cmp qword ptr [rsi + 0x218], rdx
140945a55: jbe 0x140945a81
140945a57: mov rax, qword ptr [rsi + 0x210]
140945a5e: mov rax, qword ptr [rax + rdx*8]
140945a62: test rax, rax
140945a65: je 0x140945a81
140945a67: mov rcx, qword ptr [rsi + 0x1e0]
140945a6e: not rax
140945a71: lea rax, [rax + rax*2]
140945a75: shl rax, 5
140945a79: cmp qword ptr [rcx + rax + 0x20], 0
140945a7f: jne 0x140945a89
140945a81: mov rcx, rsi
140945a84: call 0x141514610
140945a89: movabs rdx, 0xc1e128444a73c6fc
140945a93: movabs r8, 0x3ed53fad9ca67ec3
140945a9d: mov rcx, rdi
140945aa0: call 0x1400dd6f0
140945aa5: test rax, rax
140945aa8: je 0x140945add
140945aaa: cmp qword ptr [rsi + 0x218], rdx
140945ab1: jbe 0x140945add
140945ab3: mov rax, qword ptr [rsi + 0x210]
140945aba: mov rax, qword ptr [rax + rdx*8]
140945abe: test rax, rax
140945ac1: je 0x140945add
140945ac3: mov rcx, qword ptr [rsi + 0x1e0]
140945aca: not rax
140945acd: lea rax, [rax + rax*2]
140945ad1: shl rax, 5
140945ad5: cmp qword ptr [rcx + rax + 0x20], 0
140945adb: jne 0x140945ae5
140945add: mov rcx, rsi
140945ae0: call 0x14150ab60
140945ae5: movabs rdx, 0xc26ebbeefbfc3261
140945aef: movabs r8, 0x7a483ee28328d5ec
140945af9: mov rcx, rdi
140945afc: call 0x1400dd6f0
140945b01: test rax, rax
140945b04: je 0x140945b39
140945b06: cmp qword ptr [rsi + 0x218], rdx
140945b0d: jbe 0x140945b39
140945b0f: mov rax, qword ptr [rsi + 0x210]
140945b16: mov rax, qword ptr [rax + rdx*8]
140945b1a: test rax, rax
140945b1d: je 0x140945b39
140945b1f: mov rcx, qword ptr [rsi + 0x1e0]
140945b26: not rax
140945b29: lea rax, [rax + rax*2]
140945b2d: shl rax, 5
140945b31: cmp qword ptr [rcx + rax + 0x20], 0
140945b37: jne 0x140945b41
140945b39: mov rcx, rsi
140945b3c: call 0x141513fe0
140945b41: movabs rdx, 0xcda6af2f6afb130e
140945b4b: movabs r8, 0x36e88a927bd70aa
140945b55: mov rcx, rdi
140945b58: call 0x1400dd6f0
140945b5d: test rax, rax
140945b60: je 0x140945b95
140945b62: cmp qword ptr [rsi + 0x218], rdx
140945b69: jbe 0x140945b95
140945b6b: mov rax, qword ptr [rsi + 0x210]
140945b72: mov rax, qword ptr [rax + rdx*8]
140945b76: test rax, rax
140945b79: je 0x140945b95
140945b7b: mov rcx, qword ptr [rsi + 0x1e0]
140945b82: not rax
140945b85: lea rax, [rax + rax*2]
140945b89: shl rax, 5
140945b8d: cmp qword ptr [rcx + rax + 0x20], 0
140945b93: jne 0x140945b9d
140945b95: mov rcx, rsi
140945b98: call 0x1415158a0
140945b9d: movabs rdx, 0xbd2e81353f745816
140945ba7: movabs r8, 0xedb45c680aa4ecd6
140945bb1: mov rcx, rdi
140945bb4: call 0x1400dd6f0
140945bb9: test rax, rax
140945bbc: je 0x140945bf1
140945bbe: cmp qword ptr [rsi + 0x218], rdx
140945bc5: jbe 0x140945bf1
140945bc7: mov rax, qword ptr [rsi + 0x210]
140945bce: mov rax, qword ptr [rax + rdx*8]
140945bd2: test rax, rax
140945bd5: je 0x140945bf1
140945bd7: mov rcx, qword ptr [rsi + 0x1e0]
140945bde: not rax
140945be1: lea rax, [rax + rax*2]
140945be5: shl rax, 5
140945be9: cmp qword ptr [rcx + rax + 0x20], 0
140945bef: jne 0x140945bf9
140945bf1: mov rcx, rsi
140945bf4: call 0x141510200
140945bf9: movabs rdx, 0x2965d932542eefc5
140945c03: movabs r8, 0x44620fefc84124d8
140945c0d: mov rcx, rdi
140945c10: call 0x1400dd6f0
140945c15: test rax, rax
140945c18: je 0x140945c4d
140945c1a: cmp qword ptr [rsi + 0x218], rdx
140945c21: jbe 0x140945c4d
140945c23: mov rax, qword ptr [rsi + 0x210]
140945c2a: mov rax, qword ptr [rax + rdx*8]
140945c2e: test rax, rax
140945c31: je 0x140945c4d
140945c33: mov rcx, qword ptr [rsi + 0x1e0]
140945c3a: not rax
140945c3d: lea rax, [rax + rax*2]
140945c41: shl rax, 5
140945c45: cmp qword ptr [rcx + rax + 0x20], 0
140945c4b: jne 0x140945c55
140945c4d: mov rcx, rsi
140945c50: call 0x14150bdf0
140945c55: mov rax, rsi
140945c58: add rsp, 0x28
140945c5c: pop rdi
140945c5d: pop rsi
140945c5e: ret 
140945c5f: int3 
140945c60: push rbp
140945c61: push rsi
140945c62: push rdi
140945c63: sub rsp, 0x210
140945c6a: lea rbp, [rsp + 0x80]
140945c72: mov qword ptr [rbp + 0x188], 0xfffffffffffffffe
140945c7d: mov rsi, rcx
140945c80: lea rcx, [rbp - 0x60]
140945c84: call 0x14175a380
140945c89: mov byte ptr [rbp + 0x187], 6
140945c90: mov rcx, qword ptr [rip + 0x2ce66d9] ; 0x14362c370 10c362430100000040c3624301000000
140945c97: lea r8, [rip + 0x231a0c2] ; 0x142c5fd60 00000000000000000100000000000000
140945c9e: lea rdx, [rbp + 0x187]
140945ca5: call 0x140b218a0
140945caa: mov r8, rdx
140945cad: lea rcx, [rbp - 0x60]
140945cb1: mov rdx, rax
140945cb4: call 0x141453000
140945cb9: movups xmm0, xmmword ptr [rbp]
140945cbd: movaps xmmword ptr [rbp + 0x170], xmm0
140945cc4: movups xmm0, xmmword ptr [rbp - 0x10]
140945cc8: movaps xmmword ptr [rbp + 0x160], xmm0
140945ccf: movups xmm0, xmmword ptr [rbp - 0x20]
140945cd3: movaps xmmword ptr [rbp + 0x150], xmm0
140945cda: movups xmm0, xmmword ptr [rbp - 0x60]
140945cde: movups xmm1, xmmword ptr [rbp - 0x50]
140945ce2: movups xmm2, xmmword ptr [rbp - 0x40]
140945ce6: movups xmm3, xmmword ptr [rbp - 0x30]
140945cea: movaps xmmword ptr [rbp + 0x140], xmm3
140945cf1: movaps xmmword ptr [rbp + 0x130], xmm2
140945cf8: movaps xmmword ptr [rbp + 0x120], xmm1
140945cff: movaps xmmword ptr [rbp + 0x110], xmm0
140945d06: lea rdx, [rbp + 0x110]
140945d0d: mov rcx, rsi
140945d10: call 0x140b67280
140945d15: xor ecx, ecx
140945d17: call 0x1400d9b20
140945d1c: test rax, rax
140945d1f: je 0x140945e89
140945d25: mov qword ptr [rbp], 0
140945d2d: movups xmm0, xmmword ptr [rax]
140945d30: inc qword ptr [rax]
140945d33: movups xmm1, xmmword ptr [rip + 0x2169d36] ; 0x142aafa70 00000000000000000000000000000000
140945d3a: movups xmmword ptr [rbp + 0x18], xmm1
140945d3e: movups xmm1, xmmword ptr [rip + 0x2169d1b] ; 0x142aafa60 50faaa42010000000000000000000000
140945d45: movups xmmword ptr [rbp + 8], xmm1
140945d49: movups xmmword ptr [rbp + 0x28], xmm0
140945d4d: mov qword ptr [rbp - 0x60], 0
140945d55: mov qword ptr [rbp - 0x58], 8
140945d5d: mov qword ptr [rbp - 0x50], 0
140945d65: mov byte ptr [rbp - 0x48], 0
140945d69: lea rdx, [rbp - 0x60]
140945d6d: mov rcx, rsi
140945d70: call 0x1415398d0
140945d75: mov qword ptr [rbp - 0x60], 0
140945d7d: mov qword ptr [rbp - 0x50], 0
140945d85: mov dword ptr [rbp - 0x38], 0x3b9aca00
140945d8c: mov byte ptr [rbp - 0x30], 0
140945d90: lea rdx, [rbp - 0x60]
140945d94: mov rcx, rax
140945d97: call 0x141540380
140945d9c: mov rsi, rax
140945d9f: mov rdi, rax
140945da2: add rdi, 0x38
140945da6: movabs rdx, 0x3917598efcc5579e
140945db0: movabs r8, 0xaba8f5962f5afb37
140945dba: mov rcx, rdi
140945dbd: call 0x1400dd6f0
140945dc2: test rax, rax
140945dc5: je 0x140945dfa
140945dc7: cmp qword ptr [rsi + 0x218], rdx
140945dce: jbe 0x140945dfa
140945dd0: mov rax, qword ptr [rsi + 0x210]
140945dd7: mov rax, qword ptr [rax + rdx*8]
140945ddb: test rax, rax
140945dde: je 0x140945dfa
140945de0: mov rcx, qword ptr [rsi + 0x1e0]
140945de7: not rax
140945dea: lea rax, [rax + rax*2]
140945dee: shl rax, 5
140945df2: cmp qword ptr [rcx + rax + 0x20], 0
140945df8: jne 0x140945e02
140945dfa: mov rcx, rsi
140945dfd: call 0x14150fbd0
140945e02: movabs rdx, 0x2a88611d41c83a99
