140366a40: mov rax, rcx
140366a43: add r8, -4
140366a47: cmp r8, 0xf
140366a4b: ja 0x140366f2b
140366a51: lea rcx, [rip + 0x265f700]
140366a58: movsxd r8, dword ptr [rcx + r8*4]
140366a5c: add r8, rcx
140366a5f: jmp r8
140366a62: cmp dword ptr [rdx], 0x68746170
140366a68: je 0x140366f46
140366a6e: cmp dword ptr [rdx], 0x65657274
140366a74: je 0x140366f97
140366a7a: cmp dword ptr [rdx], 0x6f646e75
140366a80: je 0x140366fd6
140366a86: cmp dword ptr [rdx], 0x6f646572
140366a8c: jne 0x140366f2b
140366a92: mov cl, 0x16
140366a94: mov byte ptr [rax + 1], cl
140366a97: mov byte ptr [rax], 0x32
140366a9a: ret 
140366a9b: movdqu xmm0, xmmword ptr [rdx]
140366a9f: movzx ecx, byte ptr [rdx + 0x10]
140366aa3: movd xmm1, ecx
140366aa7: pcmpeqb xmm0, xmmword ptr [rip + 0x265ef81]
140366aaf: pcmpeqb xmm1, xmmword ptr [rip + 0x25d1429]
140366ab7: pand xmm1, xmm0
140366abb: pmovmskb ecx, xmm1
140366abf: cmp ecx, 0xffff
140366ac5: jne 0x140366f2b
140366acb: mov cl, 0x1b
140366acd: mov byte ptr [rax + 1], cl
140366ad0: mov byte ptr [rax], 0x32
140366ad3: ret 
140366ad4: mov ecx, 0x73617265
140366ad9: xor ecx, dword ptr [rdx]
140366adb: movzx r8d, word ptr [rdx + 4]
140366ae0: xor r8d, 0x7265
140366ae7: or r8d, ecx
140366aea: je 0x140366f34
140366af0: mov ecx, 0x64726167
140366af5: xor ecx, dword ptr [rdx]
140366af7: movzx edx, word ptr [rdx + 4]
140366afb: xor edx, 0x6e65
140366b01: or edx, ecx
140366b03: jne 0x140366f2b
140366b09: mov cl, 0x10
140366b0b: mov byte ptr [rax + 1], cl
140366b0e: mov byte ptr [rax], 0x32
140366b11: ret 
140366b12: movabs rcx, 0x6361625f65766f6d
140366b1c: xor rcx, qword ptr [rdx]
140366b1f: movzx r8d, byte ptr [rdx + 8]
140366b24: xor r8, 0x6b
140366b28: or r8, rcx
140366b2b: je 0x140366f3d
140366b31: movabs rcx, 0x66656c5f65766f6d
140366b3b: xor rcx, qword ptr [rdx]
140366b3e: movzx r8d, byte ptr [rdx + 8]
140366b43: xor r8, 0x74
140366b47: or r8, rcx
140366b4a: je 0x140366f8e
140366b50: movabs rcx, 0x776f645f65766f6d
140366b5a: xor rcx, qword ptr [rdx]
140366b5d: movzx r8d, byte ptr [rdx + 8]
140366b62: xor r8, 0x6e
140366b66: or r8, rcx
140366b69: je 0x140366fcd
140366b6f: movabs rcx, 0x7a696d6f74737563
140366b79: xor rcx, qword ptr [rdx]
140366b7c: movzx r8d, byte ptr [rdx + 8]
140366b81: xor r8, 0x65
140366b85: or r8, rcx
140366b88: je 0x14036700c
140366b8e: movabs rcx, 0x646f6d6f746f6870
140366b98: xor rcx, qword ptr [rdx]
140366b9b: movzx r8d, byte ptr [rdx + 8]
140366ba0: xor r8, 0x65
140366ba4: or r8, rcx
140366ba7: je 0x140367027
140366bad: movabs rcx, 0x65636e61635f6975
140366bb7: xor rcx, qword ptr [rdx]
140366bba: movzx r8d, byte ptr [rdx + 8]
140366bbf: xor r8, 0x6c
140366bc3: or r8, rcx
140366bc6: je 0x140367030
140366bcc: movabs rcx, 0x61705f6573756f6d
140366bd6: xor rcx, qword ptr [rdx]
140366bd9: movzx edx, byte ptr [rdx + 8]
140366bdd: xor rdx, 0x6e
140366be1: or rdx, rcx
140366be4: jne 0x140366f2b
140366bea: mov cl, 0x25
140366bec: mov byte ptr [rax + 1], cl
140366bef: mov byte ptr [rax], 0x32
140366bf2: ret 
140366bf3: movdqu xmm0, xmmword ptr [rdx]
140366bf7: movdqu xmm1, xmmword ptr [rdx + 3]
140366bfc: pcmpeqb xmm1, xmmword ptr [rip + 0x265ee0c]
140366c04: pcmpeqb xmm0, xmmword ptr [rip + 0x265ee14]
140366c0c: pand xmm0, xmm1
140366c10: pmovmskb ecx, xmm0
140366c14: cmp ecx, 0xffff
140366c1a: jne 0x140366f2b
140366c20: mov cl, 0x27
140366c22: mov byte ptr [rax + 1], cl
140366c25: mov byte ptr [rax], 0x32
140366c28: ret 
140366c29: mov ecx, 0x65746177
140366c2e: xor ecx, dword ptr [rdx]
140366c30: movzx r8d, byte ptr [rdx + 4]
140366c35: xor r8d, 0x72
140366c39: or r8d, ecx
140366c3c: je 0x140366f4f
140366c42: mov ecx, 0x755f6975
140366c47: xor ecx, dword ptr [rdx]
140366c49: movzx edx, byte ptr [rdx + 4]
140366c4d: xor edx, 0x70
140366c50: or edx, ecx
140366c52: jne 0x140366f2b
140366c58: mov cl, 0x21
140366c5a: mov byte ptr [rax + 1], cl
140366c5d: mov byte ptr [rax], 0x32
140366c60: ret 
140366c61: movabs rcx, 0x6769725f65766f6d
140366c6b: xor rcx, qword ptr [rdx]
140366c6e: movzx r8d, word ptr [rdx + 8]
140366c73: xor r8, 0x7468
140366c7a: or r8, rcx
140366c7d: je 0x140366f58
140366c83: movabs rcx, 0x6f7461726f636564
140366c8d: xor rcx, qword ptr [rdx]
140366c90: movzx r8d, word ptr [rdx + 8]
140366c95: xor r8, 0x7372
140366c9c: or r8, rcx
140366c9f: je 0x140366fa0
140366ca5: movabs rcx, 0x64655f7375636f66
140366caf: xor rcx, qword ptr [rdx]
140366cb2: movzx r8d, word ptr [rdx + 8]
140366cb7: xor r8, 0x7469
140366cbe: or r8, rcx
140366cc1: je 0x140366fdf
140366cc7: movabs rcx, 0x68736e6565726373
140366cd1: xor rcx, qword ptr [rdx]
140366cd4: movzx r8d, word ptr [rdx + 8]
140366cd9: xor r8, 0x746f
140366ce0: or r8, rcx
140366ce3: je 0x140367015
140366ce9: movabs rcx, 0x69666e6f635f6975
140366cf3: xor rcx, qword ptr [rdx]
140366cf6: movzx edx, word ptr [rdx + 8]
140366cfa: xor rdx, 0x6d72
140366d01: or rdx, rcx
140366d04: jne 0x140366f2b
140366d0a: mov cl, 0x1d
140366d0c: mov byte ptr [rax + 1], cl
140366d0f: mov byte ptr [rax], 0x32
140366d12: ret 
140366d13: movabs rcx, 0x6c5f657461746f72
140366d1d: xor rcx, qword ptr [rdx]
140366d20: movabs r8, 0x7466656c5f657461
140366d2a: xor r8, qword ptr [rdx + 3]
140366d2e: or r8, rcx
140366d31: je 0x140366f61
140366d37: movabs rcx, 0x5f666f5f656d6974
140366d41: xor rcx, qword ptr [rdx]
140366d44: movabs r8, 0x7961645f666f5f65
140366d4e: xor r8, qword ptr [rdx + 3]
140366d52: or r8, rcx
140366d55: je 0x140366fa9
140366d5b: movabs rcx, 0x705f6261745f6975
140366d65: xor rcx, qword ptr [rdx]
140366d68: movabs r8, 0x766572705f626174
140366d72: xor r8, qword ptr [rdx + 3]
140366d76: or r8, rcx
140366d79: je 0x140366fe8
140366d7f: movabs rcx, 0x6e5f6261745f6975
140366d89: xor rcx, qword ptr [rdx]
140366d8c: movabs r8, 0x7478656e5f626174
140366d96: xor r8, qword ptr [rdx + 3]
140366d9a: or r8, rcx
140366d9d: jne 0x140366f2b
140366da3: mov cl, 0x23
140366da5: mov byte ptr [rax + 1], cl
140366da8: mov byte ptr [rax], 0x32
140366dab: ret 
140366dac: mov ecx, 0x65766f6d
140366db1: xor ecx, dword ptr [rdx]
140366db3: mov r8d, 0x70755f65
140366db9: xor r8d, dword ptr [rdx + 3]
140366dbd: or r8d, ecx
140366dc0: je 0x140366f6a
140366dc6: mov ecx, 0x6d6f6f7a
140366dcb: xor ecx, dword ptr [rdx]
140366dcd: mov r8d, 0x6e695f6d
140366dd3: xor r8d, dword ptr [rdx + 3]
140366dd7: or r8d, ecx
140366dda: je 0x140366fb2
140366de0: mov ecx, 0x72726574
140366de5: xor ecx, dword ptr [rdx]
140366de7: mov r8d, 0x6e696172
140366ded: xor r8d, dword ptr [rdx + 3]
140366df1: or r8d, ecx
140366df4: je 0x140366ff1
140366dfa: mov ecx, 0x6c5f6975
140366dff: xor ecx, dword ptr [rdx]
140366e01: mov r8d, 0x7466656c
140366e07: xor r8d, dword ptr [rdx + 3]
140366e0b: or r8d, ecx
140366e0e: je 0x14036701e
140366e14: mov ecx, 0x645f6975
140366e19: xor ecx, dword ptr [rdx]
140366e1b: mov r8d, 0x6e776f64
140366e21: xor r8d, dword ptr [rdx + 3]
140366e25: or r8d, ecx
140366e28: jne 0x140366f2b
140366e2e: mov cl, 0x20
140366e30: mov byte ptr [rax + 1], cl
140366e33: mov byte ptr [rax], 0x32
140366e36: ret 
140366e37: movabs rcx, 0x74756f5f6d6f6f7a
140366e41: cmp qword ptr [rdx], rcx
140366e44: je 0x140366f73
140366e4a: movabs rcx, 0x676e69646c697562
140366e54: cmp qword ptr [rdx], rcx
140366e57: je 0x140366fbb
140366e5d: movabs rcx, 0x6b63616264656566
140366e67: cmp qword ptr [rdx], rcx
140366e6a: je 0x140366ffa
140366e70: movabs rcx, 0x74686769725f6975
140366e7a: cmp qword ptr [rdx], rcx
140366e7d: jne 0x140366f2b
140366e83: mov cl, 0x1f
140366e85: mov byte ptr [rax + 1], cl
140366e88: mov byte ptr [rax], 0x32
140366e8b: ret 
140366e8c: movdqu xmm0, xmmword ptr [rdx]
140366e90: pcmpeqb xmm0, xmmword ptr [rip + 0x265eba8]
140366e98: pmovmskb ecx, xmm0
140366e9c: cmp ecx, 0xffff
140366ea2: je 0x140366f7c
140366ea8: movdqu xmm0, xmmword ptr [rdx]
140366eac: pcmpeqb xmm0, xmmword ptr [rip + 0x265eb9c]
140366eb4: pmovmskb ecx, xmm0
140366eb8: cmp ecx, 0xffff
140366ebe: jne 0x140366f2b
140366ec0: mov cl, 0x26
140366ec2: mov byte ptr [rax + 1], cl
140366ec5: mov byte ptr [rax], 0x32
140366ec8: ret 
140366ec9: movabs rcx, 0x726f665f65766f6d
140366ed3: xor rcx, qword ptr [rdx]
140366ed6: mov r8d, dword ptr [rdx + 8]
140366eda: xor r8, 0x64726177
140366ee1: or r8, rcx
140366ee4: je 0x140366f85
140366eea: movabs rcx, 0x725f657461746f72
140366ef4: xor rcx, qword ptr [rdx]
140366ef7: mov r8d, dword ptr [rdx + 8]
140366efb: xor r8, 0x74686769
140366f02: or r8, rcx
140366f05: je 0x140366fc4
140366f0b: movabs rcx, 0x6f725f6573756f6d
140366f15: xor rcx, qword ptr [rdx]
140366f18: mov edx, dword ptr [rdx + 8]
140366f1b: xor rdx, 0x65746174
140366f22: or rdx, rcx
140366f25: je 0x140367003
140366f2b: mov cl, 0x28
140366f2d: mov byte ptr [rax + 1], cl
140366f30: mov byte ptr [rax], 0x32
140366f33: ret 
140366f34: mov cl, 0xe
140366f36: mov byte ptr [rax + 1], cl
140366f39: mov byte ptr [rax], 0x32
140366f3c: ret 
140366f3d: mov cl, 1
140366f3f: mov byte ptr [rax + 1], cl
140366f42: mov byte ptr [rax], 0x32
140366f45: ret 
140366f46: mov cl, 0xb
140366f48: mov byte ptr [rax + 1], cl
140366f4b: mov byte ptr [rax], 0x32
140366f4e: ret 
140366f4f: mov cl, 0x12
140366f51: mov byte ptr [rax + 1], cl
140366f54: mov byte ptr [rax], 0x32
140366f57: ret 
140366f58: mov cl, 3
140366f5a: mov byte ptr [rax + 1], cl
140366f5d: mov byte ptr [rax], 0x32
140366f60: ret 
140366f61: mov cl, 6
140366f63: mov byte ptr [rax + 1], cl
140366f66: mov byte ptr [rax], 0x32
140366f69: ret 
140366f6a: mov cl, 4
140366f6c: mov byte ptr [rax + 1], cl
140366f6f: mov byte ptr [rax], 0x32
140366f72: ret 
140366f73: mov cl, 9
140366f75: mov byte ptr [rax + 1], cl
140366f78: mov byte ptr [rax], 0x32
140366f7b: ret 
140366f7c: mov cl, 0x18
140366f7e: mov byte ptr [rax + 1], cl
140366f81: mov byte ptr [rax], 0x32
140366f84: ret 
140366f85: xor ecx, ecx
140366f87: mov byte ptr [rax + 1], cl
140366f8a: mov byte ptr [rax], 0x32
140366f8d: ret 
140366f8e: mov cl, 2
140366f90: mov byte ptr [rax + 1], cl
140366f93: mov byte ptr [rax], 0x32
140366f96: ret 
140366f97: mov cl, 0x11
140366f99: mov byte ptr [rax + 1], cl
140366f9c: mov byte ptr [rax], 0x32
140366f9f: ret 
140366fa0: mov cl, 0xc
140366fa2: mov byte ptr [rax + 1], cl
140366fa5: mov byte ptr [rax], 0x32
140366fa8: ret 
140366fa9: mov cl, 0x14
140366fab: mov byte ptr [rax + 1], cl
140366fae: mov byte ptr [rax], 0x32
140366fb1: ret 
140366fb2: mov cl, 8
140366fb4: mov byte ptr [rax + 1], cl
140366fb7: mov byte ptr [rax], 0x32
140366fba: ret 
140366fbb: mov cl, 0xa
140366fbd: mov byte ptr [rax + 1], cl
140366fc0: mov byte ptr [rax], 0x32
140366fc3: ret 
140366fc4: mov cl, 7
140366fc6: mov byte ptr [rax + 1], cl
140366fc9: mov byte ptr [rax], 0x32
140366fcc: ret 
140366fcd: mov cl, 5
140366fcf: mov byte ptr [rax + 1], cl
140366fd2: mov byte ptr [rax], 0x32
140366fd5: ret 
140366fd6: mov cl, 0x15
140366fd8: mov byte ptr [rax + 1], cl
140366fdb: mov byte ptr [rax], 0x32
140366fde: ret 
140366fdf: mov cl, 0x17
140366fe1: mov byte ptr [rax + 1], cl
140366fe4: mov byte ptr [rax], 0x32
140366fe7: ret 
140366fe8: mov cl, 0x22
140366fea: mov byte ptr [rax + 1], cl
140366fed: mov byte ptr [rax], 0x32
140366ff0: ret 
140366ff1: mov cl, 0xf
140366ff3: mov byte ptr [rax + 1], cl
140366ff6: mov byte ptr [rax], 0x32
140366ff9: ret 
140366ffa: mov cl, 0x1a
140366ffc: mov byte ptr [rax + 1], cl
140366fff: mov byte ptr [rax], 0x32
140367002: ret 
140367003: mov cl, 0x24
140367005: mov byte ptr [rax + 1], cl
140367008: mov byte ptr [rax], 0x32
14036700b: ret 
14036700c: mov cl, 0xd
14036700e: mov byte ptr [rax + 1], cl
140367011: mov byte ptr [rax], 0x32
140367014: ret 
140367015: mov cl, 0x19
140367017: mov byte ptr [rax + 1], cl
14036701a: mov byte ptr [rax], 0x32
14036701d: ret 
14036701e: mov cl, 0x1e
140367020: mov byte ptr [rax + 1], cl
140367023: mov byte ptr [rax], 0x32
140367026: ret 
140367027: mov cl, 0x13
140367029: mov byte ptr [rax + 1], cl
14036702c: mov byte ptr [rax], 0x32
14036702f: ret 
140367030: mov cl, 0x1c
140367032: mov byte ptr [rax + 1], cl
140367035: mov byte ptr [rax], 0x32
140367038: ret 
