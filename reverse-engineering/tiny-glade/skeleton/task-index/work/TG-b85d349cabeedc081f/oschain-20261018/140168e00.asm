140168e00: push rsi
140168e01: push rdi
140168e02: push rbx
140168e03: sub rsp, 0x30
140168e07: lea rax, [r8 - 6]
140168e0b: cmp rax, 0x13
140168e0f: ja 0x14016916d
140168e15: lea r9, [rip + 0x27f58fc]
140168e1c: movsxd rax, dword ptr [r9 + rax*4]
140168e20: add rax, r9
140168e23: jmp rax
140168e25: mov eax, 0x65657246
140168e2a: xor eax, dword ptr [rdx]
140168e2c: movzx r9d, word ptr [rdx + 4]
140168e31: xor r9d, 0x657a
140168e38: or r9d, eax
140168e3b: jne 0x14016916d
140168e41: mov word ptr [rcx], 0xa32
140168e46: jmp 0x1401691b9
140168e4b: movdqu xmm0, xmmword ptr [rdx]
140168e4f: movzx eax, word ptr [rdx + 0x10]
140168e53: movd xmm1, eax
140168e57: pcmpeqb xmm0, xmmword ptr [rip + 0x27f4c81]
140168e5f: pcmpeqb xmm1, xmmword ptr [rip + 0x27f4c89]
140168e67: pand xmm1, xmm0
140168e6b: pmovmskb eax, xmm1
140168e6f: cmp eax, 0xffff
140168e74: jne 0x14016916d
140168e7a: mov word ptr [rcx], 0x332
140168e7f: jmp 0x1401691b9
140168e84: movabs rax, 0x6e79734174696157
140168e8e: xor rax, qword ptr [rdx]
140168e91: movzx r9d, byte ptr [rdx + 8]
140168e96: xor r9, 0x63
140168e9a: or r9, rax
140168e9d: je 0x14016918a
140168ea3: movabs rax, 0x6d69547465736552
140168ead: xor rax, qword ptr [rdx]
140168eb0: movzx r9d, byte ptr [rdx + 8]
140168eb5: xor r9, 0x65
140168eb9: or r9, rax
140168ebc: jne 0x14016916d
140168ec2: mov word ptr [rcx], 0x432
140168ec7: jmp 0x1401691b9
140168ecc: movabs rax, 0x657a656572666e55
140168ed6: cmp qword ptr [rdx], rax
140168ed9: jne 0x14016916d
140168edf: mov word ptr [rcx], 0xb32
140168ee4: jmp 0x1401691b9
140168ee9: movdqu xmm0, xmmword ptr [rdx]
140168eed: movdqu xmm1, xmmword ptr [rdx + 9]
140168ef2: pcmpeqb xmm1, xmmword ptr [rip + 0x27f4ba6]
140168efa: pcmpeqb xmm0, xmmword ptr [rip + 0x27f4bae]
140168f02: pand xmm0, xmm1
140168f06: pmovmskb eax, xmm0
140168f0a: cmp eax, 0xffff
140168f0f: jne 0x14016916d
140168f15: mov word ptr [rcx], 0x1032
140168f1a: jmp 0x1401691b9
140168f1f: movabs rax, 0x7373655364616f4c
140168f29: xor rax, qword ptr [rdx]
140168f2c: movabs r9, 0x6e6f697373655364
140168f36: xor r9, qword ptr [rdx + 3]
140168f3a: or r9, rax
140168f3d: je 0x140169191
140168f43: movabs rax, 0x73656d6954786946
140168f4d: xor rax, qword ptr [rdx]
140168f50: movabs r9, 0x70657473656d6954
140168f5a: xor r9, qword ptr [rdx + 3]
140168f5e: or r9, rax
140168f61: je 0x1401691ad
140168f67: movabs rax, 0x51646e4165766153
140168f71: xor rax, qword ptr [rdx]
140168f74: movabs r9, 0x74697551646e4165
140168f7e: xor r9, qword ptr [rdx + 3]
140168f82: or r9, rax
140168f85: je 0x1401691b4
140168f8b: movabs rax, 0x636f725074697551
140168f95: xor rax, qword ptr [rdx]
140168f98: movabs r9, 0x737365636f725074
140168fa2: xor r9, qword ptr [rdx + 3]
140168fa6: or r9, rax
140168fa9: jne 0x14016916d
140168faf: mov word ptr [rcx], 0x1332
140168fb4: jmp 0x1401691b9
140168fb9: movabs rax, 0x72506873696e6946
140168fc3: xor rax, qword ptr [rdx]
140168fc6: movabs r9, 0x676e696c69666f72
140168fd0: xor r9, qword ptr [rdx + 7]
140168fd4: or r9, rax
140168fd7: je 0x140169198
140168fdd: movabs rax, 0x654d74726f706552
140168fe7: xor rax, qword ptr [rdx]
140168fea: movabs r9, 0x65735579726f6d65
140168ff4: xor r9, qword ptr [rdx + 7]
140168ff8: or r9, rax
140168ffb: jne 0x14016916d
140169001: mov word ptr [rcx], 0xf32
140169006: jmp 0x1401691b9
14016900b: movabs rax, 0x6d61724674696157
140169015: xor rax, qword ptr [rdx]
140169018: movzx r9d, word ptr [rdx + 8]
14016901d: xor r9, 0x7365
140169024: or r9, rax
140169027: je 0x14016919f
14016902d: lea rax, [rip + 0x27f5168]
140169034: mov rdi, rcx
140169037: mov rcx, rdx
14016903a: mov rbx, rdx
14016903d: mov rdx, rax
140169040: mov rsi, r8
140169043: call 0x142923bf0
140169048: mov rdx, rbx
14016904b: mov r8, rsi
14016904e: mov rcx, rdi
140169051: test eax, eax
140169053: jne 0x14016916d
140169059: mov word ptr [rcx], 0x932
14016905e: jmp 0x1401691b9
140169063: movdqu xmm0, xmmword ptr [rdx]
140169067: movd xmm1, dword ptr [rdx + 0x10]
14016906c: pcmpeqb xmm1, xmmword ptr [rip + 0x27d493c]
140169074: pcmpeqb xmm0, xmmword ptr [rip + 0x27f4a44]
14016907c: pand xmm0, xmm1
140169080: pmovmskb eax, xmm0
140169084: cmp eax, 0xffff
140169089: jne 0x14016916d
14016908f: mov word ptr [rcx], 0xe32
140169094: jmp 0x1401691b9
140169099: movabs rax, 0x6f72507472617453
1401690a3: xor rax, qword ptr [rdx]
1401690a6: movabs r9, 0x676e696c69666f72
1401690b0: xor r9, qword ptr [rdx + 6]
1401690b4: or r9, rax
1401690b7: jne 0x14016916d
1401690bd: mov word ptr [rcx], 0xc32
1401690c2: jmp 0x1401691b9
1401690c7: movabs rax, 0x49656c6261736944
1401690d1: xor rax, qword ptr [rdx]
1401690d4: mov r9d, dword ptr [rdx + 8]
1401690d8: xor r9, 0x7475706e
1401690df: or r9, rax
1401690e2: jne 0x14016916d
1401690e8: mov word ptr [rcx], 0x732
1401690ed: jmp 0x1401691b9
1401690f2: movdqu xmm0, xmmword ptr [rdx]
1401690f6: pcmpeqb xmm0, xmmword ptr [rip + 0x27f49d2]
1401690fe: pmovmskb eax, xmm0
140169102: cmp eax, 0xffff
140169107: jne 0x14016916d
140169109: mov word ptr [rcx], 0x832
14016910e: jmp 0x1401691b9
140169113: movdqu xmm0, xmmword ptr [rdx]
140169117: movzx eax, byte ptr [rdx + 0x10]
14016911b: movd xmm1, eax
14016911f: pcmpeqb xmm0, xmmword ptr [rip + 0x27f49d9]
140169127: pcmpeqb xmm1, xmmword ptr [rip + 0x27f49e1]
14016912f: pand xmm1, xmm0
140169133: pmovmskb eax, xmm1
140169137: cmp eax, 0xffff
14016913c: je 0x1401691a6
14016913e: lea rax, [rip + 0x27f50d3]
140169145: mov rdi, rcx
140169148: mov rcx, rdx
14016914b: mov rbx, rdx
14016914e: mov rdx, rax
140169151: mov rsi, r8
140169154: call 0x142923bf0
140169159: mov rdx, rbx
14016915c: mov r8, rsi
14016915f: mov rcx, rdi
140169162: test eax, eax
140169164: jne 0x14016916d
140169166: mov word ptr [rcx], 0x1232
14016916b: jmp 0x1401691b9
14016916d: mov qword ptr [rsp + 0x20], 0x14
140169176: lea r9, [rip + 0x27f50bb]
14016917d: mov rsi, rcx
140169180: call 0x1428ca640
140169185: mov rax, rsi
140169188: jmp 0x1401691bc
14016918a: mov word ptr [rcx], 0x232
14016918f: jmp 0x1401691b9
140169191: mov word ptr [rcx], 0x32
140169196: jmp 0x1401691b9
140169198: mov word ptr [rcx], 0xd32
14016919d: jmp 0x1401691b9
14016919f: mov word ptr [rcx], 0x532
1401691a4: jmp 0x1401691b9
1401691a6: mov word ptr [rcx], 0x132
1401691ab: jmp 0x1401691b9
1401691ad: mov word ptr [rcx], 0x632
1401691b2: jmp 0x1401691b9
1401691b4: mov word ptr [rcx], 0x1132
1401691b9: mov rax, rcx
1401691bc: add rsp, 0x30
1401691c0: pop rbx
1401691c1: pop rdi
1401691c2: pop rsi
1401691c3: ret 
