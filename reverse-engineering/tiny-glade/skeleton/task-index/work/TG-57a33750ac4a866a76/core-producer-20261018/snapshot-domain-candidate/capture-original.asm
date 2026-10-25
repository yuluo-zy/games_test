1417a1d00: 55  push rbp
1417a1d01: 4157  push r15
1417a1d03: 4156  push r14
1417a1d05: 4155  push r13
1417a1d07: 4154  push r12
1417a1d09: 56  push rsi
1417a1d0a: 57  push rdi
1417a1d0b: 53  push rbx
1417a1d0c: 4881ecc8030000  sub rsp, 0x3c8
1417a1d13: 488dac2480000000  lea rbp, [rsp + 0x80]
1417a1d1b: 48c78540030000feffffff  mov qword ptr [rbp + 0x340], 0xfffffffffffffffe
1417a1d26: 4889cf  mov rdi, rcx
1417a1d29: 488b4238  mov rax, qword ptr [rdx + 0x38]
1417a1d2d: 48898560010000  mov qword ptr [rbp + 0x160], rax
1417a1d34: 0f104228  movups xmm0, xmmword ptr [rdx + 0x28]
1417a1d38: 0f298550010000  movaps xmmword ptr [rbp + 0x150], xmm0
1417a1d3f: 488b4250  mov rax, qword ptr [rdx + 0x50]
1417a1d43: 488985c0010000  mov qword ptr [rbp + 0x1c0], rax
1417a1d4a: 0f104240  movups xmm0, xmmword ptr [rdx + 0x40]
1417a1d4e: 0f2985b0010000  movaps xmmword ptr [rbp + 0x1b0], xmm0
1417a1d55: 488b4268  mov rax, qword ptr [rdx + 0x68]
1417a1d59: 48898540010000  mov qword ptr [rbp + 0x140], rax
1417a1d60: 0f104258  movups xmm0, xmmword ptr [rdx + 0x58]
1417a1d64: 0f298530010000  movaps xmmword ptr [rbp + 0x130], xmm0
1417a1d6b: 488b8280000000  mov rax, qword ptr [rdx + 0x80]
1417a1d72: 488985a0010000  mov qword ptr [rbp + 0x1a0], rax
1417a1d79: 0f104270  movups xmm0, xmmword ptr [rdx + 0x70]
1417a1d7d: 0f298590010000  movaps xmmword ptr [rbp + 0x190], xmm0
1417a1d84: 488b8298000000  mov rax, qword ptr [rdx + 0x98]
1417a1d8b: 488985f0000000  mov qword ptr [rbp + 0xf0], rax
1417a1d92: 0f108288000000  movups xmm0, xmmword ptr [rdx + 0x88]
1417a1d99: 0f2985e0000000  movaps xmmword ptr [rbp + 0xe0], xmm0
1417a1da0: 488b82b0000000  mov rax, qword ptr [rdx + 0xb0]
1417a1da7: 488985d0000000  mov qword ptr [rbp + 0xd0], rax
1417a1dae: 0f1082a0000000  movups xmm0, xmmword ptr [rdx + 0xa0]
1417a1db5: 0f2985c0000000  movaps xmmword ptr [rbp + 0xc0], xmm0
1417a1dbc: 488b82c8000000  mov rax, qword ptr [rdx + 0xc8]
1417a1dc3: 488985b0000000  mov qword ptr [rbp + 0xb0], rax
1417a1dca: 0f1082b8000000  movups xmm0, xmmword ptr [rdx + 0xb8]
1417a1dd1: 0f2985a0000000  movaps xmmword ptr [rbp + 0xa0], xmm0
1417a1dd8: 488b8258010000  mov rax, qword ptr [rdx + 0x158]
1417a1ddf: 48894520  mov qword ptr [rbp + 0x20], rax
1417a1de3: 0f108218010000  movups xmm0, xmmword ptr [rdx + 0x118]
1417a1dea: 0f108a28010000  movups xmm1, xmmword ptr [rdx + 0x128]
1417a1df1: 0f109238010000  movups xmm2, xmmword ptr [rdx + 0x138]
1417a1df8: 0f109a48010000  movups xmm3, xmmword ptr [rdx + 0x148]
1417a1dff: 0f295d10  movaps xmmword ptr [rbp + 0x10], xmm3
1417a1e03: 0f295500  movaps xmmword ptr [rbp], xmm2
1417a1e07: 0f294df0  movaps xmmword ptr [rbp - 0x10], xmm1
1417a1e0b: 0f2945e0  movaps xmmword ptr [rbp - 0x20], xmm0
1417a1e0f: 488b82d0000000  mov rax, qword ptr [rdx + 0xd0]
1417a1e16: 488985d0010000  mov qword ptr [rbp + 0x1d0], rax
1417a1e1d: 488b82d8000000  mov rax, qword ptr [rdx + 0xd8]
1417a1e24: 488985f8010000  mov qword ptr [rbp + 0x1f8], rax
1417a1e2b: 4c8bb2e0000000  mov r14, qword ptr [rdx + 0xe0]
1417a1e32: f20f108290010000  movsd xmm0, qword ptr [rdx + 0x190]
1417a1e3a: f20f114530  movsd qword ptr [rbp + 0x30], xmm0
1417a1e3f: 488b82f8000000  mov rax, qword ptr [rdx + 0xf8]
1417a1e46: 48898580010000  mov qword ptr [rbp + 0x180], rax
1417a1e4d: 0f1082e8000000  movups xmm0, xmmword ptr [rdx + 0xe8]
1417a1e54: 0f298570010000  movaps xmmword ptr [rbp + 0x170], xmm0
1417a1e5b: 488b8210010000  mov rax, qword ptr [rdx + 0x110]
1417a1e62: 48898520010000  mov qword ptr [rbp + 0x120], rax
1417a1e69: 0f108200010000  movups xmm0, xmmword ptr [rdx + 0x100]
1417a1e70: 0f298510010000  movaps xmmword ptr [rbp + 0x110], xmm0
1417a1e77: 488b4220  mov rax, qword ptr [rdx + 0x20]
1417a1e7b: 48894560  mov qword ptr [rbp + 0x60], rax
1417a1e7f: 0f1002  movups xmm0, xmmword ptr [rdx]
1417a1e82: 0f104a10  movups xmm1, xmmword ptr [rdx + 0x10]
1417a1e86: 0f294d50  movaps xmmword ptr [rbp + 0x50], xmm1
1417a1e8a: 0f294540  movaps xmmword ptr [rbp + 0x40], xmm0
1417a1e8e: f30f6f8260010000  movdqu xmm0, xmmword ptr [rdx + 0x160]
1417a1e96: 0f108a70010000  movups xmm1, xmmword ptr [rdx + 0x170]
1417a1e9d: 48899588000000  mov qword ptr [rbp + 0x88], rdx
1417a1ea4: 0f109280010000  movups xmm2, xmmword ptr [rdx + 0x180]
1417a1eab: 0f2955d0  movaps xmmword ptr [rbp - 0x30], xmm2
1417a1eaf: 0f294dc0  movaps xmmword ptr [rbp - 0x40], xmm1
1417a1eb3: 660f7f45b0  movdqa xmmword ptr [rbp - 0x50], xmm0
1417a1eb8: 0fb6054a2de901  movzx eax, byte ptr [rip + 0x1e92d4a]
1417a1ebf: b900000800  mov ecx, 0x80000
1417a1ec4: ba01000000  mov edx, 1
1417a1ec9: e8421de7fe  call 0x140613c10
1417a1ece: 4885c0  test rax, rax
1417a1ed1: 0f84232f0000  je 0x1417a4dfa
1417a1ed7: 48c785f800000000000800  mov qword ptr [rbp + 0xf8], 0x80000
1417a1ee2: 48898500010000  mov qword ptr [rbp + 0x100], rax
1417a1ee9: 48c7850801000000000000  mov qword ptr [rbp + 0x108], 0
1417a1ef4: 488d85f8000000  lea rax, [rbp + 0xf8]
1417a1efb: 48894570  mov qword ptr [rbp + 0x70], rax
1417a1eff: 660fefc0  pxor xmm0, xmm0
1417a1f03: f30f7f4578  movdqu xmmword ptr [rbp + 0x78], xmm0
1417a1f08: 488b8d58010000  mov rcx, qword ptr [rbp + 0x158]
1417a1f0f: 488b9560010000  mov rdx, qword ptr [rbp + 0x160]
1417a1f16: 488db590020000  lea rsi, [rbp + 0x290]
1417a1f1d: 4889b5d0020000  mov qword ptr [rbp + 0x2d0], rsi
1417a1f24: 4883fa02  cmp rdx, 2
1417a1f28: 0f833f2c0000  jae 0x1417a4b6d
1417a1f2e: 488b8db8010000  mov rcx, qword ptr [rbp + 0x1b8]
1417a1f35: 488b95c0010000  mov rdx, qword ptr [rbp + 0x1c0]
1417a1f3c: 4889b5d0020000  mov qword ptr [rbp + 0x2d0], rsi
1417a1f43: 4883fa02  cmp rdx, 2
1417a1f47: 0f83562c0000  jae 0x1417a4ba3
1417a1f4d: 488b8d38010000  mov rcx, qword ptr [rbp + 0x138]
1417a1f54: 488b9540010000  mov rdx, qword ptr [rbp + 0x140]
1417a1f5b: 4889b5d0020000  mov qword ptr [rbp + 0x2d0], rsi
1417a1f62: 4883fa02  cmp rdx, 2
1417a1f66: 0f836d2c0000  jae 0x1417a4bd9
1417a1f6c: 488b8d18010000  mov rcx, qword ptr [rbp + 0x118]
1417a1f73: 488b9520010000  mov rdx, qword ptr [rbp + 0x120]
1417a1f7a: 4889b5d0020000  mov qword ptr [rbp + 0x2d0], rsi
1417a1f81: 4883fa02  cmp rdx, 2
1417a1f85: 0f83842c0000  jae 0x1417a4c0f
1417a1f8b: 488b9580010000  mov rdx, qword ptr [rbp + 0x180]
1417a1f92: 4883fa02  cmp rdx, 2
1417a1f96: 0f83a92c0000  jae 0x1417a4c45
1417a1f9c: 4889bd48020000  mov qword ptr [rbp + 0x248], rdi
1417a1fa3: 48c7852802000000000000  mov qword ptr [rbp + 0x228], 0
1417a1fae: 48c7853802000000000000  mov qword ptr [rbp + 0x238], 0
1417a1fb9: 0fb605492ce901  movzx eax, byte ptr [rip + 0x1e92c49]
1417a1fc0: b907000000  mov ecx, 7
1417a1fc5: ba01000000  mov edx, 1
1417a1fca: e8411ce7fe  call 0x140613c10
1417a1fcf: 4885c0  test rax, rax
1417a1fd2: 0f84442e0000  je 0x1417a4e1c
1417a1fd8: c7400373696f6e  mov dword ptr [rax + 3], 0x6e6f6973
1417a1fdf: c70056657273  mov dword ptr [rax], 0x73726556
1417a1fe5: 48c7855002000007000000  mov qword ptr [rbp + 0x250], 7
1417a1ff0: 48898558020000  mov qword ptr [rbp + 0x258], rax
1417a1ff7: 48c7856002000007000000  mov qword ptr [rbp + 0x260], 7
1417a2002: c685d002000002  mov byte ptr [rbp + 0x2d0], 2
1417a2009: 48c785d802000000000000  mov qword ptr [rbp + 0x2d8], 0
1417a2014: 48c785e002000002000000  mov qword ptr [rbp + 0x2e0], 2
1417a201f: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a2026: 488d9528020000  lea rdx, [rbp + 0x228]
1417a202d: 4c8d8550020000  lea r8, [rbp + 0x250]
1417a2034: 4c8d8dd0020000  lea r9, [rbp + 0x2d0]
1417a203b: e8f099c9ff  call 0x14143ba30
1417a2040: 0fb68590020000  movzx eax, byte ptr [rbp + 0x290]
1417a2047: 4883f806  cmp rax, 6
1417a204b: 7769  ja 0x1417a20b6
1417a204d: 488d0d8ca85301  lea rcx, [rip + 0x153a88c]
1417a2054: 48630481  movsxd rax, dword ptr [rcx + rax*4]
1417a2058: 4801c8  add rax, rcx
1417a205b: ffe0  jmp rax
1417a205d: 488b9598020000  mov rdx, qword ptr [rbp + 0x298]
1417a2064: 4885d2  test rdx, rdx
1417a2067: 7459  je 0x1417a20c2
1417a2069: 488b8da0020000  mov rcx, qword ptr [rbp + 0x2a0]
1417a2070: 41b801000000  mov r8d, 1
1417a2076: eb37  jmp 0x1417a20af
1417a2078: 488b8da0020000  mov rcx, qword ptr [rbp + 0x2a0]
1417a207f: 488b95a8020000  mov rdx, qword ptr [rbp + 0x2a8]
1417a2086: 48898d30030000  mov qword ptr [rbp + 0x330], rcx
1417a208d: e8de6c0eff  call 0x140888d70
1417a2092: 488b9598020000  mov rdx, qword ptr [rbp + 0x298]
1417a2099: 4885d2  test rdx, rdx
1417a209c: 7424  je 0x1417a20c2
1417a209e: 48c1e205  shl rdx, 5
1417a20a2: 41b808000000  mov r8d, 8
1417a20a8: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a20af: e86c1be7fe  call 0x140613c20
1417a20b4: eb0c  jmp 0x1417a20c2
1417a20b6: 488d8d98020000  lea rcx, [rbp + 0x298]
1417a20bd: e87e6e0fff  call 0x140898f40
1417a20c2: 0fb605402be901  movzx eax, byte ptr [rip + 0x1e92b40]
1417a20c9: b905000000  mov ecx, 5
1417a20ce: ba01000000  mov edx, 1
1417a20d3: e8381be7fe  call 0x140613c10
1417a20d8: 4885c0  test rax, rax
1417a20db: 0f84562d0000  je 0x1417a4e37
1417a20e1: c6400473  mov byte ptr [rax + 4], 0x73
1417a20e5: c70057616c6c  mov dword ptr [rax], 0x6c6c6157
1417a20eb: 48c7855002000005000000  mov qword ptr [rbp + 0x250], 5
1417a20f6: 48898558020000  mov qword ptr [rbp + 0x258], rax
1417a20fd: 48c7856002000005000000  mov qword ptr [rbp + 0x260], 5
1417a2108: c6852d03000001  mov byte ptr [rbp + 0x32d], 1
1417a210f: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a2116: 488d9550010000  lea rdx, [rbp + 0x150]
1417a211d: e80ee9deff  call 0x141590a30
1417a2122: 80bdd002000006  cmp byte ptr [rbp + 0x2d0], 6
1417a2129: 0f84232d0000  je 0x1417a4e52
1417a212f: f30f6f85d0020000  movdqu xmm0, xmmword ptr [rbp + 0x2d0]
1417a2137: 0f108de0020000  movups xmm1, xmmword ptr [rbp + 0x2e0]
1417a213e: 0f298da0020000  movaps xmmword ptr [rbp + 0x2a0], xmm1
1417a2145: 660f7f8590020000  movdqa xmmword ptr [rbp + 0x290], xmm0
1417a214d: c6852d03000000  mov byte ptr [rbp + 0x32d], 0
1417a2154: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a215b: 488d9528020000  lea rdx, [rbp + 0x228]
1417a2162: 4c8d8550020000  lea r8, [rbp + 0x250]
1417a2169: 4c8d8d90020000  lea r9, [rbp + 0x290]
1417a2170: e8bb98c9ff  call 0x14143ba30
1417a2175: 0fb685d0020000  movzx eax, byte ptr [rbp + 0x2d0]
1417a217c: 4883f806  cmp rax, 6
1417a2180: 7769  ja 0x1417a21eb
1417a2182: 488d0d73a75301  lea rcx, [rip + 0x153a773]
1417a2189: 48630481  movsxd rax, dword ptr [rcx + rax*4]
1417a218d: 4801c8  add rax, rcx
1417a2190: ffe0  jmp rax
1417a2192: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a2199: 4885d2  test rdx, rdx
1417a219c: 7459  je 0x1417a21f7
1417a219e: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a21a5: 41b801000000  mov r8d, 1
1417a21ab: eb37  jmp 0x1417a21e4
1417a21ad: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a21b4: 488b95e8020000  mov rdx, qword ptr [rbp + 0x2e8]
1417a21bb: 48898d30030000  mov qword ptr [rbp + 0x330], rcx
1417a21c2: e8a96b0eff  call 0x140888d70
1417a21c7: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a21ce: 4885d2  test rdx, rdx
1417a21d1: 7424  je 0x1417a21f7
1417a21d3: 48c1e205  shl rdx, 5
1417a21d7: 41b808000000  mov r8d, 8
1417a21dd: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a21e4: e8371ae7fe  call 0x140613c20
1417a21e9: eb0c  jmp 0x1417a21f7
1417a21eb: 488d8dd8020000  lea rcx, [rbp + 0x2d8]
1417a21f2: e8496d0fff  call 0x140898f40
1417a21f7: 0fb6050b2ae901  movzx eax, byte ptr [rip + 0x1e92a0b]
1417a21fe: b905000000  mov ecx, 5
1417a2203: ba01000000  mov edx, 1
1417a2208: e8031ae7fe  call 0x140613c10
1417a220d: 4885c0  test rax, rax
1417a2210: 0f847a2c0000  je 0x1417a4e90
1417a2216: c6400473  mov byte ptr [rax + 4], 0x73
1417a221a: c700526f6f66  mov dword ptr [rax], 0x666f6f52
1417a2220: 48c7855002000005000000  mov qword ptr [rbp + 0x250], 5
1417a222b: 48898558020000  mov qword ptr [rbp + 0x258], rax
1417a2232: 48c7856002000005000000  mov qword ptr [rbp + 0x260], 5
1417a223d: c6852c03000001  mov byte ptr [rbp + 0x32c], 1
1417a2244: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a224b: 488d95b0010000  lea rdx, [rbp + 0x1b0]
1417a2252: e899d8deff  call 0x14158faf0
1417a2257: 80bdd002000006  cmp byte ptr [rbp + 0x2d0], 6
1417a225e: 0f84472c0000  je 0x1417a4eab
1417a2264: f30f6f85d0020000  movdqu xmm0, xmmword ptr [rbp + 0x2d0]
1417a226c: 0f108de0020000  movups xmm1, xmmword ptr [rbp + 0x2e0]
1417a2273: 0f298da0020000  movaps xmmword ptr [rbp + 0x2a0], xmm1
1417a227a: 660f7f8590020000  movdqa xmmword ptr [rbp + 0x290], xmm0
1417a2282: c6852c03000000  mov byte ptr [rbp + 0x32c], 0
1417a2289: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a2290: 488d9528020000  lea rdx, [rbp + 0x228]
1417a2297: 4c8d8550020000  lea r8, [rbp + 0x250]
1417a229e: 4c8d8d90020000  lea r9, [rbp + 0x290]
1417a22a5: e88697c9ff  call 0x14143ba30
1417a22aa: 0fb685d0020000  movzx eax, byte ptr [rbp + 0x2d0]
1417a22b1: 4883f806  cmp rax, 6
1417a22b5: 7769  ja 0x1417a2320
1417a22b7: 488d0d5aa65301  lea rcx, [rip + 0x153a65a]
1417a22be: 48630481  movsxd rax, dword ptr [rcx + rax*4]
1417a22c2: 4801c8  add rax, rcx
1417a22c5: ffe0  jmp rax
1417a22c7: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a22ce: 4885d2  test rdx, rdx
1417a22d1: 7459  je 0x1417a232c
1417a22d3: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a22da: 41b801000000  mov r8d, 1
1417a22e0: eb37  jmp 0x1417a2319
1417a22e2: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a22e9: 488b95e8020000  mov rdx, qword ptr [rbp + 0x2e8]
1417a22f0: 48898d30030000  mov qword ptr [rbp + 0x330], rcx
1417a22f7: e8746a0eff  call 0x140888d70
1417a22fc: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a2303: 4885d2  test rdx, rdx
1417a2306: 7424  je 0x1417a232c
1417a2308: 48c1e205  shl rdx, 5
1417a230c: 41b808000000  mov r8d, 8
1417a2312: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a2319: e80219e7fe  call 0x140613c20
1417a231e: eb0c  jmp 0x1417a232c
1417a2320: 488d8dd8020000  lea rcx, [rbp + 0x2d8]
1417a2327: e8146c0fff  call 0x140898f40
1417a232c: 0fb605d628e901  movzx eax, byte ptr [rip + 0x1e928d6]
1417a2333: b90a000000  mov ecx, 0xa
1417a2338: ba01000000  mov edx, 1
1417a233d: e8ce18e7fe  call 0x140613c10
1417a2342: 4885c0  test rax, rax
1417a2345: 0f849e2b0000  je 0x1417a4ee9
1417a234b: 48b94465636f7261746f  movabs rcx, 0x6f7461726f636544
1417a2355: 488908  mov qword ptr [rax], rcx
1417a2358: 66c740087273  mov word ptr [rax + 8], 0x7372
1417a235e: 48c785500200000a000000  mov qword ptr [rbp + 0x250], 0xa
1417a2369: 48898558020000  mov qword ptr [rbp + 0x258], rax
1417a2370: 48c785600200000a000000  mov qword ptr [rbp + 0x260], 0xa
1417a237b: c6852b03000001  mov byte ptr [rbp + 0x32b], 1
1417a2382: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a2389: 488d9530010000  lea rdx, [rbp + 0x130]
1417a2390: e8fbeadbff  call 0x141560e90
1417a2395: 80bdd002000006  cmp byte ptr [rbp + 0x2d0], 6
1417a239c: 0f84622b0000  je 0x1417a4f04
1417a23a2: f30f6f85d0020000  movdqu xmm0, xmmword ptr [rbp + 0x2d0]
1417a23aa: 0f108de0020000  movups xmm1, xmmword ptr [rbp + 0x2e0]
1417a23b1: 0f298da0020000  movaps xmmword ptr [rbp + 0x2a0], xmm1
1417a23b8: 660f7f8590020000  movdqa xmmword ptr [rbp + 0x290], xmm0
1417a23c0: c6852b03000000  mov byte ptr [rbp + 0x32b], 0
1417a23c7: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a23ce: 488d9528020000  lea rdx, [rbp + 0x228]
1417a23d5: 4c8d8550020000  lea r8, [rbp + 0x250]
1417a23dc: 4c8d8d90020000  lea r9, [rbp + 0x290]
1417a23e3: e84896c9ff  call 0x14143ba30
1417a23e8: 0fb685d0020000  movzx eax, byte ptr [rbp + 0x2d0]
1417a23ef: 4883f806  cmp rax, 6
1417a23f3: 7769  ja 0x1417a245e
1417a23f5: 488d0d38a55301  lea rcx, [rip + 0x153a538]
1417a23fc: 48630481  movsxd rax, dword ptr [rcx + rax*4]
1417a2400: 4801c8  add rax, rcx
1417a2403: ffe0  jmp rax
1417a2405: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a240c: 4885d2  test rdx, rdx
1417a240f: 7459  je 0x1417a246a
1417a2411: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a2418: 41b801000000  mov r8d, 1
1417a241e: eb37  jmp 0x1417a2457
1417a2420: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a2427: 488b95e8020000  mov rdx, qword ptr [rbp + 0x2e8]
1417a242e: 48898d30030000  mov qword ptr [rbp + 0x330], rcx
1417a2435: e836690eff  call 0x140888d70
1417a243a: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a2441: 4885d2  test rdx, rdx
1417a2444: 7424  je 0x1417a246a
1417a2446: 48c1e205  shl rdx, 5
1417a244a: 41b808000000  mov r8d, 8
1417a2450: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a2457: e8c417e7fe  call 0x140613c20
1417a245c: eb0c  jmp 0x1417a246a
1417a245e: 488d8dd8020000  lea rcx, [rbp + 0x2d8]
1417a2465: e8d66a0fff  call 0x140898f40
1417a246a: 0fb6059827e901  movzx eax, byte ptr [rip + 0x1e92798]
1417a2471: b906000000  mov ecx, 6
1417a2476: ba01000000  mov edx, 1
1417a247b: e89017e7fe  call 0x140613c10
1417a2480: 4885c0  test rax, rax
1417a2483: 0f84b92a0000  je 0x1417a4f42
1417a2489: 66c740046572  mov word ptr [rax + 4], 0x7265
1417a248f: c70052617374  mov dword ptr [rax], 0x74736152
1417a2495: 48c785d801000006000000  mov qword ptr [rbp + 0x1d8], 6
1417a24a0: 488985e0010000  mov qword ptr [rbp + 0x1e0], rax
1417a24a7: 48c785e801000006000000  mov qword ptr [rbp + 0x1e8], 6
1417a24b2: 488bbda0010000  mov rdi, qword ptr [rbp + 0x1a0]
1417a24b9: 4885ff  test rdi, rdi
1417a24bc: 7438  je 0x1417a24f6
1417a24be: 4989c7  mov r15, rax
1417a24c1: 488b9d98010000  mov rbx, qword ptr [rbp + 0x198]
1417a24c8: 4889fe  mov rsi, rdi
1417a24cb: 48c1e604  shl rsi, 4
1417a24cf: 0fb6053327e901  movzx eax, byte ptr [rip + 0x1e92733]
1417a24d6: ba08000000  mov edx, 8
1417a24db: 4889f1  mov rcx, rsi
1417a24de: e82d17e7fe  call 0x140613c10
1417a24e3: 4885c0  test rax, rax
1417a24e6: 0f84e12d0000  je 0x1417a52cd
1417a24ec: 4883ff01  cmp rdi, 1
1417a24f0: 750b  jne 0x1417a24fd
1417a24f2: 31c9  xor ecx, ecx
1417a24f4: eb41  jmp 0x1417a2537
1417a24f6: b808000000  mov eax, 8
1417a24fb: eb52  jmp 0x1417a254f
1417a24fd: 4889fa  mov rdx, rdi
1417a2500: 4883e2fe  and rdx, 0xfffffffffffffffe
1417a2504: 4c8d4330  lea r8, [rbx + 0x30]
1417a2508: 4c8d4818  lea r9, [rax + 0x18]
1417a250c: 31c9  xor ecx, ecx
1417a250e: 6690  nop
1417a2510: 410f1040d0  movups xmm0, xmmword ptr [r8 - 0x30]
1417a2515: 410f1141e8  movups xmmword ptr [r9 - 0x18], xmm0
1417a251a: f3410f6f40f8  movdqu xmm0, xmmword ptr [r8 - 8]
1417a2520: f3410f7f41f8  movdqu xmmword ptr [r9 - 8], xmm0
1417a2526: 4883c102  add rcx, 2
1417a252a: 4983c050  add r8, 0x50
1417a252e: 4983c120  add r9, 0x20
1417a2532: 4839ca  cmp rdx, rcx
1417a2535: 75d9  jne 0x1417a2510
1417a2537: 40f6c701  test dil, 1
1417a253b: 7412  je 0x1417a254f
1417a253d: 488d1489  lea rdx, [rcx + rcx*4]
1417a2541: f30f6f04d3  movdqu xmm0, xmmword ptr [rbx + rdx*8]
1417a2546: 48c1e104  shl rcx, 4
1417a254a: f30f7f0408  movdqu xmmword ptr [rax + rcx], xmm0
1417a254f: 4889bd50020000  mov qword ptr [rbp + 0x250], rdi
1417a2556: 48898558020000  mov qword ptr [rbp + 0x258], rax
1417a255d: 4889bd60020000  mov qword ptr [rbp + 0x260], rdi
1417a2564: c6852a03000001  mov byte ptr [rbp + 0x32a], 1
1417a256b: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a2572: 488d9550020000  lea rdx, [rbp + 0x250]
1417a2579: e862e0deff  call 0x1415905e0
1417a257e: 80bdd002000006  cmp byte ptr [rbp + 0x2d0], 6
1417a2585: 0f84d2290000  je 0x1417a4f5d
1417a258b: f30f6f85d0020000  movdqu xmm0, xmmword ptr [rbp + 0x2d0]
1417a2593: 0f108de0020000  movups xmm1, xmmword ptr [rbp + 0x2e0]
1417a259a: 0f298da0020000  movaps xmmword ptr [rbp + 0x2a0], xmm1
1417a25a1: 660f7f8590020000  movdqa xmmword ptr [rbp + 0x290], xmm0
1417a25a9: c6852a03000000  mov byte ptr [rbp + 0x32a], 0
1417a25b0: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a25b7: 488d9528020000  lea rdx, [rbp + 0x228]
1417a25be: 4c8d85d8010000  lea r8, [rbp + 0x1d8]
1417a25c5: 4c8d8d90020000  lea r9, [rbp + 0x290]
1417a25cc: e85f94c9ff  call 0x14143ba30
1417a25d1: 0fb685d0020000  movzx eax, byte ptr [rbp + 0x2d0]
1417a25d8: 4883f806  cmp rax, 6
1417a25dc: 0f879d010000  ja 0x1417a277f
1417a25e2: 488d0d67a35301  lea rcx, [rip + 0x153a367]
1417a25e9: 48630481  movsxd rax, dword ptr [rcx + rax*4]
1417a25ed: 4801c8  add rax, rcx
1417a25f0: ffe0  jmp rax
1417a25f2: 488b9550020000  mov rdx, qword ptr [rbp + 0x250]
1417a25f9: 4885d2  test rdx, rdx
1417a25fc: 7416  je 0x1417a2614
1417a25fe: 488b8d58020000  mov rcx, qword ptr [rbp + 0x258]
1417a2605: 48c1e204  shl rdx, 4
1417a2609: 41b808000000  mov r8d, 8
1417a260f: e80c16e7fe  call 0x140613c20
1417a2614: 0fb605ee25e901  movzx eax, byte ptr [rip + 0x1e925ee]
1417a261b: b90a000000  mov ecx, 0xa
1417a2620: ba01000000  mov edx, 1
1417a2625: e8e615e7fe  call 0x140613c10
1417a262a: 4885c0  test rax, rax
1417a262d: 0f8468290000  je 0x1417a4f9b
1417a2633: 48b957616c6c436f6c6f  movabs rcx, 0x6f6c6f436c6c6157
1417a263d: 488908  mov qword ptr [rax], rcx
1417a2640: 66c740087273  mov word ptr [rax + 8], 0x7372
1417a2646: 48c785500200000a000000  mov qword ptr [rbp + 0x250], 0xa
1417a2651: 48898558020000  mov qword ptr [rbp + 0x258], rax
1417a2658: 48c785600200000a000000  mov qword ptr [rbp + 0x260], 0xa
1417a2663: c6852903000001  mov byte ptr [rbp + 0x329], 1
1417a266a: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a2671: 488d95e0000000  lea rdx, [rbp + 0xe0]
1417a2678: e833d2deff  call 0x14158f8b0
1417a267d: 80bdd002000006  cmp byte ptr [rbp + 0x2d0], 6
1417a2684: 0f842c290000  je 0x1417a4fb6
1417a268a: f30f6f85d0020000  movdqu xmm0, xmmword ptr [rbp + 0x2d0]
1417a2692: 0f108de0020000  movups xmm1, xmmword ptr [rbp + 0x2e0]
1417a2699: 0f298da0020000  movaps xmmword ptr [rbp + 0x2a0], xmm1
1417a26a0: 660f7f8590020000  movdqa xmmword ptr [rbp + 0x290], xmm0
1417a26a8: c6852903000000  mov byte ptr [rbp + 0x329], 0
1417a26af: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a26b6: 488d9528020000  lea rdx, [rbp + 0x228]
1417a26bd: 4c8d8550020000  lea r8, [rbp + 0x250]
1417a26c4: 4c8d8d90020000  lea r9, [rbp + 0x290]
1417a26cb: e86093c9ff  call 0x14143ba30
1417a26d0: 0fb685d0020000  movzx eax, byte ptr [rbp + 0x2d0]
1417a26d7: 4883f806  cmp rax, 6
1417a26db: 0f87ed000000  ja 0x1417a27ce
1417a26e1: 488d0d84a25301  lea rcx, [rip + 0x153a284]
1417a26e8: 48630481  movsxd rax, dword ptr [rcx + rax*4]
1417a26ec: 4801c8  add rax, rcx
1417a26ef: ffe0  jmp rax
1417a26f1: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a26f8: 4885d2  test rdx, rdx
1417a26fb: 0f84d9000000  je 0x1417a27da
1417a2701: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a2708: 41b801000000  mov r8d, 1
1417a270e: e9b4000000  jmp 0x1417a27c7
1417a2713: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a271a: 4885d2  test rdx, rdx
1417a271d: 0f84cffeffff  je 0x1417a25f2
1417a2723: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a272a: 41b801000000  mov r8d, 1
1417a2730: e8eb14e7fe  call 0x140613c20
1417a2735: e9b8feffff  jmp 0x1417a25f2
1417a273a: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a2741: 488b95e8020000  mov rdx, qword ptr [rbp + 0x2e8]
1417a2748: 48898d30030000  mov qword ptr [rbp + 0x330], rcx
1417a274f: e81c660eff  call 0x140888d70
1417a2754: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a275b: 4885d2  test rdx, rdx
1417a275e: 0f848efeffff  je 0x1417a25f2
1417a2764: 48c1e205  shl rdx, 5
1417a2768: 41b808000000  mov r8d, 8
1417a276e: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a2775: e8a614e7fe  call 0x140613c20
1417a277a: e973feffff  jmp 0x1417a25f2
1417a277f: 488d8dd8020000  lea rcx, [rbp + 0x2d8]
1417a2786: e8b5670fff  call 0x140898f40
1417a278b: e962feffff  jmp 0x1417a25f2
1417a2790: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a2797: 488b95e8020000  mov rdx, qword ptr [rbp + 0x2e8]
1417a279e: 48898d30030000  mov qword ptr [rbp + 0x330], rcx
1417a27a5: e8c6650eff  call 0x140888d70
1417a27aa: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a27b1: 4885d2  test rdx, rdx
1417a27b4: 7424  je 0x1417a27da
1417a27b6: 48c1e205  shl rdx, 5
1417a27ba: 41b808000000  mov r8d, 8
1417a27c0: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a27c7: e85414e7fe  call 0x140613c20
1417a27cc: eb0c  jmp 0x1417a27da
1417a27ce: 488d8dd8020000  lea rcx, [rbp + 0x2d8]
1417a27d5: e866670fff  call 0x140898f40
1417a27da: 0fb6052824e901  movzx eax, byte ptr [rip + 0x1e92428]
1417a27e1: b90a000000  mov ecx, 0xa
1417a27e6: ba01000000  mov edx, 1
1417a27eb: e82014e7fe  call 0x140613c10
1417a27f0: 4885c0  test rax, rax
1417a27f3: 0f84fb270000  je 0x1417a4ff4
1417a27f9: 48b947617465436f6c6f  movabs rcx, 0x6f6c6f4365746147
1417a2803: 488908  mov qword ptr [rax], rcx
1417a2806: 66c740087273  mov word ptr [rax + 8], 0x7372
1417a280c: 48c785500200000a000000  mov qword ptr [rbp + 0x250], 0xa
1417a2817: 48898558020000  mov qword ptr [rbp + 0x258], rax
1417a281e: 48c785600200000a000000  mov qword ptr [rbp + 0x260], 0xa
1417a2829: c6852803000001  mov byte ptr [rbp + 0x328], 1
1417a2830: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a2837: 488d95c0000000  lea rdx, [rbp + 0xc0]
1417a283e: e86dd0deff  call 0x14158f8b0
1417a2843: 80bdd002000006  cmp byte ptr [rbp + 0x2d0], 6
1417a284a: 0f84bf270000  je 0x1417a500f
1417a2850: f30f6f85d0020000  movdqu xmm0, xmmword ptr [rbp + 0x2d0]
1417a2858: 0f108de0020000  movups xmm1, xmmword ptr [rbp + 0x2e0]
1417a285f: 0f298da0020000  movaps xmmword ptr [rbp + 0x2a0], xmm1
1417a2866: 660f7f8590020000  movdqa xmmword ptr [rbp + 0x290], xmm0
1417a286e: c6852803000000  mov byte ptr [rbp + 0x328], 0
1417a2875: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a287c: 488d9528020000  lea rdx, [rbp + 0x228]
1417a2883: 4c8d8550020000  lea r8, [rbp + 0x250]
1417a288a: 4c8d8d90020000  lea r9, [rbp + 0x290]
1417a2891: e89a91c9ff  call 0x14143ba30
1417a2896: 0fb685d0020000  movzx eax, byte ptr [rbp + 0x2d0]
1417a289d: 4883f806  cmp rax, 6
1417a28a1: 7769  ja 0x1417a290c
1417a28a3: 488d0ddea05301  lea rcx, [rip + 0x153a0de]
1417a28aa: 48630481  movsxd rax, dword ptr [rcx + rax*4]
1417a28ae: 4801c8  add rax, rcx
1417a28b1: ffe0  jmp rax
1417a28b3: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a28ba: 4885d2  test rdx, rdx
1417a28bd: 7459  je 0x1417a2918
1417a28bf: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a28c6: 41b801000000  mov r8d, 1
1417a28cc: eb37  jmp 0x1417a2905
1417a28ce: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a28d5: 488b95e8020000  mov rdx, qword ptr [rbp + 0x2e8]
1417a28dc: 48898d30030000  mov qword ptr [rbp + 0x330], rcx
1417a28e3: e888640eff  call 0x140888d70
1417a28e8: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a28ef: 4885d2  test rdx, rdx
1417a28f2: 7424  je 0x1417a2918
1417a28f4: 48c1e205  shl rdx, 5
1417a28f8: 41b808000000  mov r8d, 8
1417a28fe: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a2905: e81613e7fe  call 0x140613c20
1417a290a: eb0c  jmp 0x1417a2918
1417a290c: 488d8dd8020000  lea rcx, [rbp + 0x2d8]
1417a2913: e828660fff  call 0x140898f40
1417a2918: 0fb605ea22e901  movzx eax, byte ptr [rip + 0x1e922ea]
1417a291f: b910000000  mov ecx, 0x10
1417a2924: ba01000000  mov edx, 1
1417a2929: e8e212e7fe  call 0x140613c10
1417a292e: 4885c0  test rax, rax
1417a2931: 0f8416270000  je 0x1417a504d
1417a2937: f30f6f056b9c5301  movdqu xmm0, xmmword ptr [rip + 0x1539c6b]
1417a293f: f30f7f00  movdqu xmmword ptr [rax], xmm0
1417a2943: 48c7855002000010000000  mov qword ptr [rbp + 0x250], 0x10
1417a294e: 48898558020000  mov qword ptr [rbp + 0x258], rax
1417a2955: 48c7856002000010000000  mov qword ptr [rbp + 0x260], 0x10
1417a2960: c6852703000001  mov byte ptr [rbp + 0x327], 1
1417a2967: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a296e: 488d95a0000000  lea rdx, [rbp + 0xa0]
1417a2975: e836cfdeff  call 0x14158f8b0
1417a297a: 80bdd002000006  cmp byte ptr [rbp + 0x2d0], 6
1417a2981: 0f84e1260000  je 0x1417a5068
1417a2987: f30f6f85d0020000  movdqu xmm0, xmmword ptr [rbp + 0x2d0]
1417a298f: 0f108de0020000  movups xmm1, xmmword ptr [rbp + 0x2e0]
1417a2996: 0f298da0020000  movaps xmmword ptr [rbp + 0x2a0], xmm1
1417a299d: 660f7f8590020000  movdqa xmmword ptr [rbp + 0x290], xmm0
1417a29a5: c6852703000000  mov byte ptr [rbp + 0x327], 0
1417a29ac: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a29b3: 488d9528020000  lea rdx, [rbp + 0x228]
1417a29ba: 4c8d8550020000  lea r8, [rbp + 0x250]
1417a29c1: 4c8d8d90020000  lea r9, [rbp + 0x290]
1417a29c8: e86390c9ff  call 0x14143ba30
1417a29cd: 0fb685d0020000  movzx eax, byte ptr [rbp + 0x2d0]
1417a29d4: 4883f806  cmp rax, 6
1417a29d8: 7769  ja 0x1417a2a43
1417a29da: 488d0dc39f5301  lea rcx, [rip + 0x1539fc3]
1417a29e1: 48630481  movsxd rax, dword ptr [rcx + rax*4]
1417a29e5: 4801c8  add rax, rcx
1417a29e8: ffe0  jmp rax
1417a29ea: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a29f1: 4885d2  test rdx, rdx
1417a29f4: 7459  je 0x1417a2a4f
1417a29f6: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a29fd: 41b801000000  mov r8d, 1
1417a2a03: eb37  jmp 0x1417a2a3c
1417a2a05: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a2a0c: 488b95e8020000  mov rdx, qword ptr [rbp + 0x2e8]
1417a2a13: 48898d30030000  mov qword ptr [rbp + 0x330], rcx
1417a2a1a: e851630eff  call 0x140888d70
1417a2a1f: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a2a26: 4885d2  test rdx, rdx
1417a2a29: 7424  je 0x1417a2a4f
1417a2a2b: 48c1e205  shl rdx, 5
1417a2a2f: 41b808000000  mov r8d, 8
1417a2a35: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a2a3c: e8df11e7fe  call 0x140613c20
1417a2a41: eb0c  jmp 0x1417a2a4f
1417a2a43: 488d8dd8020000  lea rcx, [rbp + 0x2d8]
1417a2a4a: e8f1640fff  call 0x140898f40
1417a2a4f: 0fb605b321e901  movzx eax, byte ptr [rip + 0x1e921b3]
1417a2a56: b907000000  mov ecx, 7
1417a2a5b: ba01000000  mov edx, 1
1417a2a60: e8ab11e7fe  call 0x140613c10
1417a2a65: 4885c0  test rax, rax
1417a2a68: 0f8438260000  je 0x1417a50a6
1417a2a6e: c740037261696e  mov dword ptr [rax + 3], 0x6e696172
1417a2a75: c70054657272  mov dword ptr [rax], 0x72726554
1417a2a7b: 48c7855002000007000000  mov qword ptr [rbp + 0x250], 7
1417a2a86: 48898558020000  mov qword ptr [rbp + 0x258], rax
1417a2a8d: 48c7856002000007000000  mov qword ptr [rbp + 0x260], 7
1417a2a98: c6852603000001  mov byte ptr [rbp + 0x326], 1
1417a2a9f: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a2aa6: 488d55e0  lea rdx, [rbp - 0x20]
1417a2aaa: e8c14014ff  call 0x1408e6b70
1417a2aaf: 80bdd002000006  cmp byte ptr [rbp + 0x2d0], 6
1417a2ab6: 0f8405260000  je 0x1417a50c1
1417a2abc: f30f6f85d0020000  movdqu xmm0, xmmword ptr [rbp + 0x2d0]
1417a2ac4: 0f108de0020000  movups xmm1, xmmword ptr [rbp + 0x2e0]
1417a2acb: 0f298da0020000  movaps xmmword ptr [rbp + 0x2a0], xmm1
1417a2ad2: 660f7f8590020000  movdqa xmmword ptr [rbp + 0x290], xmm0
1417a2ada: c6852603000000  mov byte ptr [rbp + 0x326], 0
1417a2ae1: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a2ae8: 488d9528020000  lea rdx, [rbp + 0x228]
1417a2aef: 4c8d8550020000  lea r8, [rbp + 0x250]
1417a2af6: 4c8d8d90020000  lea r9, [rbp + 0x290]
1417a2afd: e82e8fc9ff  call 0x14143ba30
1417a2b02: 0fb685d0020000  movzx eax, byte ptr [rbp + 0x2d0]
1417a2b09: 4883f806  cmp rax, 6
1417a2b0d: 7769  ja 0x1417a2b78
1417a2b0f: 488d0daa9e5301  lea rcx, [rip + 0x1539eaa]
1417a2b16: 48630481  movsxd rax, dword ptr [rcx + rax*4]
1417a2b1a: 4801c8  add rax, rcx
1417a2b1d: ffe0  jmp rax
1417a2b1f: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a2b26: 4885d2  test rdx, rdx
1417a2b29: 7459  je 0x1417a2b84
1417a2b2b: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a2b32: 41b801000000  mov r8d, 1
1417a2b38: eb37  jmp 0x1417a2b71
1417a2b3a: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a2b41: 488b95e8020000  mov rdx, qword ptr [rbp + 0x2e8]
1417a2b48: 48898d30030000  mov qword ptr [rbp + 0x330], rcx
1417a2b4f: e81c620eff  call 0x140888d70
1417a2b54: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a2b5b: 4885d2  test rdx, rdx
1417a2b5e: 7424  je 0x1417a2b84
1417a2b60: 48c1e205  shl rdx, 5
1417a2b64: 41b808000000  mov r8d, 8
1417a2b6a: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a2b71: e8aa10e7fe  call 0x140613c20
1417a2b76: eb0c  jmp 0x1417a2b84
1417a2b78: 488d8dd8020000  lea rcx, [rbp + 0x2d8]
1417a2b7f: e8bc630fff  call 0x140898f40
1417a2b84: 0fb6057e20e901  movzx eax, byte ptr [rip + 0x1e9207e]
1417a2b8b: b912000000  mov ecx, 0x12
1417a2b90: ba01000000  mov edx, 1
1417a2b95: e87610e7fe  call 0x140613c10
1417a2b9a: 4885c0  test rax, rax
1417a2b9d: 0f845c250000  je 0x1417a50ff
1417a2ba3: f30f6f05169a5301  movdqu xmm0, xmmword ptr [rip + 0x1539a16]
1417a2bab: f30f7f00  movdqu xmmword ptr [rax], xmm0
1417a2baf: 66c740106765  mov word ptr [rax + 0x10], 0x6567
1417a2bb5: 48c7855002000012000000  mov qword ptr [rbp + 0x250], 0x12
1417a2bc0: 48898558020000  mov qword ptr [rbp + 0x258], rax
1417a2bc7: 48c7856002000012000000  mov qword ptr [rbp + 0x260], 0x12
1417a2bd2: c6852503000001  mov byte ptr [rbp + 0x325], 1
1417a2bd9: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a2be0: 488d5530  lea rdx, [rbp + 0x30]
1417a2be4: e847e338ff  call 0x140b30f30
1417a2be9: 80bdd002000006  cmp byte ptr [rbp + 0x2d0], 6
1417a2bf0: 0f8424250000  je 0x1417a511a
1417a2bf6: f30f6f85d0020000  movdqu xmm0, xmmword ptr [rbp + 0x2d0]
1417a2bfe: 0f108de0020000  movups xmm1, xmmword ptr [rbp + 0x2e0]
1417a2c05: 0f298da0020000  movaps xmmword ptr [rbp + 0x2a0], xmm1
1417a2c0c: 660f7f8590020000  movdqa xmmword ptr [rbp + 0x290], xmm0
1417a2c14: c6852503000000  mov byte ptr [rbp + 0x325], 0
1417a2c1b: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a2c22: 488d9528020000  lea rdx, [rbp + 0x228]
1417a2c29: 4c8d8550020000  lea r8, [rbp + 0x250]
1417a2c30: 4c8d8d90020000  lea r9, [rbp + 0x290]
1417a2c37: e8f48dc9ff  call 0x14143ba30
1417a2c3c: 0fb685d0020000  movzx eax, byte ptr [rbp + 0x2d0]
1417a2c43: 4883f806  cmp rax, 6
1417a2c47: 7769  ja 0x1417a2cb2
1417a2c49: 488d0d8c9d5301  lea rcx, [rip + 0x1539d8c]
1417a2c50: 48630481  movsxd rax, dword ptr [rcx + rax*4]
1417a2c54: 4801c8  add rax, rcx
1417a2c57: ffe0  jmp rax
1417a2c59: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a2c60: 4885d2  test rdx, rdx
1417a2c63: 7459  je 0x1417a2cbe
1417a2c65: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a2c6c: 41b801000000  mov r8d, 1
1417a2c72: eb37  jmp 0x1417a2cab
1417a2c74: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a2c7b: 488b95e8020000  mov rdx, qword ptr [rbp + 0x2e8]
1417a2c82: 48898d30030000  mov qword ptr [rbp + 0x330], rcx
1417a2c89: e8e2600eff  call 0x140888d70
1417a2c8e: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a2c95: 4885d2  test rdx, rdx
1417a2c98: 7424  je 0x1417a2cbe
1417a2c9a: 48c1e205  shl rdx, 5
1417a2c9e: 41b808000000  mov r8d, 8
1417a2ca4: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a2cab: e8700fe7fe  call 0x140613c20
1417a2cb0: eb0c  jmp 0x1417a2cbe
1417a2cb2: 488d8dd8020000  lea rcx, [rbp + 0x2d8]
1417a2cb9: e882620fff  call 0x140898f40
1417a2cbe: 0fb605441fe901  movzx eax, byte ptr [rip + 0x1e91f44]
1417a2cc5: b905000000  mov ecx, 5
1417a2cca: ba01000000  mov edx, 1
1417a2ccf: e83c0fe7fe  call 0x140613c10
1417a2cd4: 4885c0  test rax, rax
1417a2cd7: 0f847b240000  je 0x1417a5158
1417a2cdd: c6400473  mov byte ptr [rax + 4], 0x73
1417a2ce1: c70054726565  mov dword ptr [rax], 0x65657254
1417a2ce7: 48c7855002000005000000  mov qword ptr [rbp + 0x250], 5
1417a2cf2: 48898558020000  mov qword ptr [rbp + 0x258], rax
1417a2cf9: 48c7856002000005000000  mov qword ptr [rbp + 0x260], 5
1417a2d04: c6852403000001  mov byte ptr [rbp + 0x324], 1
1417a2d0b: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a2d12: 488d9510010000  lea rdx, [rbp + 0x110]
1417a2d19: e842d4deff  call 0x141590160
1417a2d1e: 80bdd002000006  cmp byte ptr [rbp + 0x2d0], 6
1417a2d25: 0f8448240000  je 0x1417a5173
1417a2d2b: f30f6f85d0020000  movdqu xmm0, xmmword ptr [rbp + 0x2d0]
1417a2d33: 0f108de0020000  movups xmm1, xmmword ptr [rbp + 0x2e0]
1417a2d3a: 0f298da0020000  movaps xmmword ptr [rbp + 0x2a0], xmm1
1417a2d41: 660f7f8590020000  movdqa xmmword ptr [rbp + 0x290], xmm0
1417a2d49: c6852403000000  mov byte ptr [rbp + 0x324], 0
1417a2d50: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a2d57: 488d9528020000  lea rdx, [rbp + 0x228]
1417a2d5e: 4c8d8550020000  lea r8, [rbp + 0x250]
1417a2d65: 4c8d8d90020000  lea r9, [rbp + 0x290]
1417a2d6c: e8bf8cc9ff  call 0x14143ba30
1417a2d71: 0fb685d0020000  movzx eax, byte ptr [rbp + 0x2d0]
1417a2d78: 4883f806  cmp rax, 6
1417a2d7c: 7769  ja 0x1417a2de7
1417a2d7e: 488d0d739c5301  lea rcx, [rip + 0x1539c73]
1417a2d85: 48630481  movsxd rax, dword ptr [rcx + rax*4]
1417a2d89: 4801c8  add rax, rcx
1417a2d8c: ffe0  jmp rax
1417a2d8e: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a2d95: 4885d2  test rdx, rdx
1417a2d98: 7459  je 0x1417a2df3
1417a2d9a: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a2da1: 41b801000000  mov r8d, 1
1417a2da7: eb37  jmp 0x1417a2de0
1417a2da9: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a2db0: 488b95e8020000  mov rdx, qword ptr [rbp + 0x2e8]
1417a2db7: 48898d30030000  mov qword ptr [rbp + 0x330], rcx
1417a2dbe: e8ad5f0eff  call 0x140888d70
1417a2dc3: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a2dca: 4885d2  test rdx, rdx
1417a2dcd: 7424  je 0x1417a2df3
1417a2dcf: 48c1e205  shl rdx, 5
1417a2dd3: 41b808000000  mov r8d, 8
1417a2dd9: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a2de0: e83b0ee7fe  call 0x140613c20
1417a2de5: eb0c  jmp 0x1417a2df3
1417a2de7: 488d8dd8020000  lea rcx, [rbp + 0x2d8]
1417a2dee: e84d610fff  call 0x140898f40
1417a2df3: 0fb6050f1ee901  movzx eax, byte ptr [rip + 0x1e91e0f]
1417a2dfa: b90b000000  mov ecx, 0xb
1417a2dff: ba01000000  mov edx, 1
1417a2e04: e8070ee7fe  call 0x140613c10
1417a2e09: 4885c0  test rax, rax
1417a2e0c: 0f849f230000  je 0x1417a51b1
1417a2e12: 48b94175746f436c7574  movabs rcx, 0x74756c436f747541
1417a2e1c: 488908  mov qword ptr [rax], rcx
1417a2e1f: c7400774746572  mov dword ptr [rax + 7], 0x72657474
1417a2e26: 48c785500200000b000000  mov qword ptr [rbp + 0x250], 0xb
1417a2e31: 48898558020000  mov qword ptr [rbp + 0x258], rax
1417a2e38: 48c785600200000b000000  mov qword ptr [rbp + 0x260], 0xb
1417a2e43: c6852303000001  mov byte ptr [rbp + 0x323], 1
1417a2e4a: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a2e51: 488d9570010000  lea rdx, [rbp + 0x170]
1417a2e58: e8f3c1deff  call 0x14158f050
1417a2e5d: 80bdd002000006  cmp byte ptr [rbp + 0x2d0], 6
1417a2e64: 0f8462230000  je 0x1417a51cc
1417a2e6a: f30f6f85d0020000  movdqu xmm0, xmmword ptr [rbp + 0x2d0]
1417a2e72: 0f108de0020000  movups xmm1, xmmword ptr [rbp + 0x2e0]
1417a2e79: 0f298da0020000  movaps xmmword ptr [rbp + 0x2a0], xmm1
1417a2e80: 660f7f8590020000  movdqa xmmword ptr [rbp + 0x290], xmm0
1417a2e88: c6852303000000  mov byte ptr [rbp + 0x323], 0
1417a2e8f: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a2e96: 488d9528020000  lea rdx, [rbp + 0x228]
1417a2e9d: 4c8d8550020000  lea r8, [rbp + 0x250]
1417a2ea4: 4c8d8d90020000  lea r9, [rbp + 0x290]
1417a2eab: e8808bc9ff  call 0x14143ba30
1417a2eb0: 0fb685d0020000  movzx eax, byte ptr [rbp + 0x2d0]
1417a2eb7: 4883f806  cmp rax, 6
1417a2ebb: 7769  ja 0x1417a2f26
1417a2ebd: 488d0d509b5301  lea rcx, [rip + 0x1539b50]
1417a2ec4: 48630481  movsxd rax, dword ptr [rcx + rax*4]
1417a2ec8: 4801c8  add rax, rcx
1417a2ecb: ffe0  jmp rax
1417a2ecd: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a2ed4: 4885d2  test rdx, rdx
1417a2ed7: 7459  je 0x1417a2f32
1417a2ed9: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a2ee0: 41b801000000  mov r8d, 1
1417a2ee6: eb37  jmp 0x1417a2f1f
1417a2ee8: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a2eef: 488b95e8020000  mov rdx, qword ptr [rbp + 0x2e8]
1417a2ef6: 48898d30030000  mov qword ptr [rbp + 0x330], rcx
1417a2efd: e86e5e0eff  call 0x140888d70
1417a2f02: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a2f09: 4885d2  test rdx, rdx
1417a2f0c: 7424  je 0x1417a2f32
1417a2f0e: 48c1e205  shl rdx, 5
1417a2f12: 41b808000000  mov r8d, 8
1417a2f18: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a2f1f: e8fc0ce7fe  call 0x140613c20
1417a2f24: eb0c  jmp 0x1417a2f32
1417a2f26: 488d8dd8020000  lea rcx, [rbp + 0x2d8]
1417a2f2d: e80e600fff  call 0x140898f40
1417a2f32: 0fb605d01ce901  movzx eax, byte ptr [rip + 0x1e91cd0]
1417a2f39: b906000000  mov ecx, 6
1417a2f3e: ba01000000  mov edx, 1
1417a2f43: e8c80ce7fe  call 0x140613c10
1417a2f48: 4885c0  test rax, rax
1417a2f4b: 0f84b9220000  je 0x1417a520a
1417a2f51: 66c740047273  mov word ptr [rax + 4], 0x7372
1417a2f57: c70053746169  mov dword ptr [rax], 0x69617453
1417a2f5d: 48c7855002000006000000  mov qword ptr [rbp + 0x250], 6
1417a2f68: 48898558020000  mov qword ptr [rbp + 0x258], rax
1417a2f6f: 48c7856002000006000000  mov qword ptr [rbp + 0x260], 6
1417a2f7a: c6852203000001  mov byte ptr [rbp + 0x322], 1
1417a2f81: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a2f88: 488d5540  lea rdx, [rbp + 0x40]
1417a2f8c: e87fa8ddff  call 0x14157d810
1417a2f91: 80bdd002000006  cmp byte ptr [rbp + 0x2d0], 6
1417a2f98: 0f8487220000  je 0x1417a5225
1417a2f9e: f30f6f85d0020000  movdqu xmm0, xmmword ptr [rbp + 0x2d0]
1417a2fa6: 0f108de0020000  movups xmm1, xmmword ptr [rbp + 0x2e0]
1417a2fad: 0f298da0020000  movaps xmmword ptr [rbp + 0x2a0], xmm1
1417a2fb4: 660f7f8590020000  movdqa xmmword ptr [rbp + 0x290], xmm0
1417a2fbc: c6852203000000  mov byte ptr [rbp + 0x322], 0
1417a2fc3: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a2fca: 488d9528020000  lea rdx, [rbp + 0x228]
1417a2fd1: 4c8d8550020000  lea r8, [rbp + 0x250]
1417a2fd8: 4c8d8d90020000  lea r9, [rbp + 0x290]
1417a2fdf: e84c8ac9ff  call 0x14143ba30
1417a2fe4: 0fb685d0020000  movzx eax, byte ptr [rbp + 0x2d0]
1417a2feb: 4883f806  cmp rax, 6
1417a2fef: 7769  ja 0x1417a305a
1417a2ff1: 488d0d389a5301  lea rcx, [rip + 0x1539a38]
1417a2ff8: 48630481  movsxd rax, dword ptr [rcx + rax*4]
1417a2ffc: 4801c8  add rax, rcx
1417a2fff: ffe0  jmp rax
1417a3001: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a3008: 4885d2  test rdx, rdx
1417a300b: 7459  je 0x1417a3066
1417a300d: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a3014: 41b801000000  mov r8d, 1
1417a301a: eb37  jmp 0x1417a3053
1417a301c: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a3023: 488b95e8020000  mov rdx, qword ptr [rbp + 0x2e8]
1417a302a: 48898d30030000  mov qword ptr [rbp + 0x330], rcx
1417a3031: e83a5d0eff  call 0x140888d70
1417a3036: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a303d: 4885d2  test rdx, rdx
1417a3040: 7424  je 0x1417a3066
1417a3042: 48c1e205  shl rdx, 5
1417a3046: 41b808000000  mov r8d, 8
1417a304c: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a3053: e8c80be7fe  call 0x140613c20
1417a3058: eb0c  jmp 0x1417a3066
1417a305a: 488d8dd8020000  lea rcx, [rbp + 0x2d8]
1417a3061: e8da5e0fff  call 0x140898f40
1417a3066: 0fb6059c1be901  movzx eax, byte ptr [rip + 0x1e91b9c]
1417a306d: b915000000  mov ecx, 0x15
1417a3072: ba01000000  mov edx, 1
1417a3077: e8940be7fe  call 0x140613c10
1417a307c: 4885c0  test rax, rax
1417a307f: 0f84de210000  je 0x1417a5263
1417a3085: f30f6f055c955301  movdqu xmm0, xmmword ptr [rip + 0x153955c]
1417a308d: f30f7f00  movdqu xmmword ptr [rax], xmm0
1417a3091: 48b9537570706f727473  movabs rcx, 0x7374726f70707553
1417a309b: 4889480d  mov qword ptr [rax + 0xd], rcx
1417a309f: 48c7855002000015000000  mov qword ptr [rbp + 0x250], 0x15
1417a30aa: 48898558020000  mov qword ptr [rbp + 0x258], rax
1417a30b1: 48c7856002000015000000  mov qword ptr [rbp + 0x260], 0x15
1417a30bc: c6852103000001  mov byte ptr [rbp + 0x321], 1
1417a30c3: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a30ca: 488d55b0  lea rdx, [rbp - 0x50]
1417a30ce: e8fd8939ff  call 0x140b3bad0
1417a30d3: 80bdd002000006  cmp byte ptr [rbp + 0x2d0], 6
1417a30da: 0f849e210000  je 0x1417a527e
1417a30e0: f30f6f85d0020000  movdqu xmm0, xmmword ptr [rbp + 0x2d0]
1417a30e8: 0f108de0020000  movups xmm1, xmmword ptr [rbp + 0x2e0]
1417a30ef: 0f298da0020000  movaps xmmword ptr [rbp + 0x2a0], xmm1
1417a30f6: 660f7f8590020000  movdqa xmmword ptr [rbp + 0x290], xmm0
1417a30fe: c6852103000000  mov byte ptr [rbp + 0x321], 0
1417a3105: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a310c: 488d9528020000  lea rdx, [rbp + 0x228]
1417a3113: 4c8d8550020000  lea r8, [rbp + 0x250]
1417a311a: 488dbd90020000  lea rdi, [rbp + 0x290]
1417a3121: 4989f9  mov r9, rdi
1417a3124: e80789c9ff  call 0x14143ba30
1417a3129: 0fb685d0020000  movzx eax, byte ptr [rbp + 0x2d0]
1417a3130: 4883f806  cmp rax, 6
1417a3134: 7769  ja 0x1417a319f
1417a3136: 488d0d0f995301  lea rcx, [rip + 0x153990f]
1417a313d: 48630481  movsxd rax, dword ptr [rcx + rax*4]
1417a3141: 4801c8  add rax, rcx
1417a3144: ffe0  jmp rax
1417a3146: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a314d: 4885d2  test rdx, rdx
1417a3150: 7459  je 0x1417a31ab
1417a3152: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a3159: 41b801000000  mov r8d, 1
1417a315f: eb37  jmp 0x1417a3198
1417a3161: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a3168: 488b95e8020000  mov rdx, qword ptr [rbp + 0x2e8]
1417a316f: 48898d30030000  mov qword ptr [rbp + 0x330], rcx
1417a3176: e8f55b0eff  call 0x140888d70
1417a317b: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a3182: 4885d2  test rdx, rdx
1417a3185: 7424  je 0x1417a31ab
1417a3187: 48c1e205  shl rdx, 5
1417a318b: 41b808000000  mov r8d, 8
1417a3191: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a3198: e8830ae7fe  call 0x140613c20
1417a319d: eb0c  jmp 0x1417a31ab
1417a319f: 488d8dd8020000  lea rcx, [rbp + 0x2d8]
1417a31a6: e8955d0fff  call 0x140898f40
1417a31ab: 4c897538  mov qword ptr [rbp + 0x38], r14
1417a31af: f30f6f8528020000  movdqu xmm0, xmmword ptr [rbp + 0x228]
1417a31b7: f30f7f8508020000  movdqu xmmword ptr [rbp + 0x208], xmm0
1417a31bf: 488b8538020000  mov rax, qword ptr [rbp + 0x238]
1417a31c6: 48898518020000  mov qword ptr [rbp + 0x218], rax
1417a31cd: c6850002000005  mov byte ptr [rbp + 0x200], 5
1417a31d4: c6853f03000001  mov byte ptr [rbp + 0x33f], 1
1417a31db: c6853e03000001  mov byte ptr [rbp + 0x33e], 1
1417a31e2: 4c8d0515945301  lea r8, [rip + 0x1539415]
1417a31e9: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a31f0: 488d5570  lea rdx, [rbp + 0x70]
1417a31f4: 41b908000000  mov r9d, 8
1417a31fa: e891f4daff  call 0x141552690
1417a31ff: 4c8bbdd0020000  mov r15, qword ptr [rbp + 0x2d0]
1417a3206: 488b8dd8020000  mov rcx, qword ptr [rbp + 0x2d8]
1417a320d: 4d85ff  test r15, r15
1417a3210: 0f84a0000000  je 0x1417a32b6
1417a3216: 488b85e0020000  mov rax, qword ptr [rbp + 0x2e0]
1417a321d: 4c89bd90020000  mov qword ptr [rbp + 0x290], r15
1417a3224: 48898d98020000  mov qword ptr [rbp + 0x298], rcx
1417a322b: 488985a0020000  mov qword ptr [rbp + 0x2a0], rax
1417a3232: 0fb68500020000  movzx eax, byte ptr [rbp + 0x200]
1417a3239: 488d0d28985301  lea rcx, [rip + 0x1539828]
1417a3240: 48630481  movsxd rax, dword ptr [rcx + rax*4]
1417a3244: 4801c8  add rax, rcx
1417a3247: ffe0  jmp rax
1417a3249: 498b37  mov rsi, qword ptr [r15]
1417a324c: 498b5f08  mov rbx, qword ptr [r15 + 8]
1417a3250: 4889df  mov rdi, rbx
1417a3253: 4883c704  add rdi, 4
1417a3257: 49c7c0ffffffff  mov r8, 0xffffffffffffffff
1417a325e: 4c0f43c7  cmovae r8, rdi
1417a3262: 488b06  mov rax, qword ptr [rsi]
1417a3265: 4939c0  cmp r8, rax
1417a3268: 7613  jbe 0x1417a327d
1417a326a: 488b5610  mov rdx, qword ptr [rsi + 0x10]
1417a326e: 4929d0  sub r8, rdx
1417a3271: 4829d0  sub rax, rdx
1417a3274: 4c39c0  cmp rax, r8
1417a3277: 0f82a81a0000  jb 0x1417a4d25
1417a327d: 488b4e10  mov rcx, qword ptr [rsi + 0x10]
1417a3281: 4989d8  mov r8, rbx
1417a3284: 4929c8  sub r8, rcx
1417a3287: 760f  jbe 0x1417a3298
1417a3289: 48034e08  add rcx, qword ptr [rsi + 8]
1417a328d: 31d2  xor edx, edx
1417a328f: e88c091801  call 0x142923c20
1417a3294: 48895e10  mov qword ptr [rsi + 0x10], rbx
1417a3298: 488b4608  mov rax, qword ptr [rsi + 8]
1417a329c: c704186e756c6c  mov dword ptr [rax + rbx], 0x6c6c756e
1417a32a3: 483b7e10  cmp rdi, qword ptr [rsi + 0x10]
1417a32a7: 7604  jbe 0x1417a32ad
1417a32a9: 48897e10  mov qword ptr [rsi + 0x10], rdi
1417a32ad: 49897f08  mov qword ptr [r15 + 8], rdi
1417a32b1: e9bb010000  jmp 0x1417a3471
1417a32b6: c6853f03000001  mov byte ptr [rbp + 0x33f], 1
1417a32bd: c6853e03000001  mov byte ptr [rbp + 0x33e], 1
1417a32c4: e8c7621001  call 0x1428a9590
1417a32c9: e923120000  jmp 0x1417a44f1
1417a32ce: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a32d5: 488d9508020000  lea rdx, [rbp + 0x208]
1417a32dc: e83fecddff  call 0x141581f20
1417a32e1: 4889c7  mov rdi, rax
1417a32e4: 4885ff  test rdi, rdi
1417a32e7: 0f8537110000  jne 0x1417a4424
1417a32ed: e97f010000  jmp 0x1417a3471
1417a32f2: 488b8508020000  mov rax, qword ptr [rbp + 0x208]
1417a32f9: 4883f802  cmp rax, 2
1417a32fd: 0f84d80c0000  je 0x1417a3fdb
1417a3303: 83f801  cmp eax, 1
1417a3306: 0f852e0d0000  jne 0x1417a403a
1417a330c: 4c8b8510020000  mov r8, qword ptr [rbp + 0x210]
1417a3313: 4c89c1  mov rcx, r8
1417a3316: 48f7d9  neg rcx
1417a3319: 490f48c8  cmovs rcx, r8
1417a331d: 41be14000000  mov r14d, 0x14
1417a3323: 4881f910270000  cmp rcx, 0x2710
1417a332a: 0f829e0d0000  jb 0x1417a40ce
1417a3330: 41bb14000000  mov r11d, 0x14
1417a3336: 49b94b598638d6c56d34  movabs r9, 0x346dc5d63886594b
1417a3340: 4c8d15c48e5301  lea r10, [rip + 0x1538ec4]
1417a3347: 660f1f840000000000  nop word ptr [rax + rax]
1417a3350: 4889c8  mov rax, rcx
1417a3353: 49f7e1  mul r9
1417a3356: 48c1ea0b  shr rdx, 0xb
1417a335a: 69c210270000  imul eax, edx, 0x2710
1417a3360: 89ce  mov esi, ecx
1417a3362: 29c6  sub esi, eax
1417a3364: 69c67b140000  imul eax, esi, 0x147b
1417a336a: c1e813  shr eax, 0x13
1417a336d: 6bf864  imul edi, eax, 0x64
1417a3370: 29fe  sub esi, edi
1417a3372: 0fb7f6  movzx esi, si
1417a3375: 4d8d73fc  lea r14, [r11 - 4]
1417a3379: 410fb70442  movzx eax, word ptr [r10 + rax*2]
1417a337e: 664289841dcc020000  mov word ptr [rbp + r11 + 0x2cc], ax
1417a3387: 410fb70472  movzx eax, word ptr [r10 + rsi*2]
1417a338c: 664289841dce020000  mov word ptr [rbp + r11 + 0x2ce], ax
1417a3395: 4d89f3  mov r11, r14
1417a3398: 4881f9ffe0f505  cmp rcx, 0x5f5e0ff
1417a339f: 4889d1  mov rcx, rdx
1417a33a2: 77ac  ja 0x1417a3350
1417a33a4: e9280d0000  jmp 0x1417a40d1
1417a33a9: 4c8b8510020000  mov r8, qword ptr [rbp + 0x210]
1417a33b0: 4c8b8d18020000  mov r9, qword ptr [rbp + 0x218]
1417a33b7: 488d95a8020000  lea rdx, [rbp + 0x2a8]
1417a33be: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a33c5: e816cfddff  call 0x1415802e0
1417a33ca: 4885c0  test rax, rax
1417a33cd: 0f849e000000  je 0x1417a3471
1417a33d3: 4889c1  mov rcx, rax
1417a33d6: e8c5641201  call 0x1428c98a0
1417a33db: e9520c0000  jmp 0x1417a4032
1417a33e0: 0fb68501020000  movzx eax, byte ptr [rbp + 0x201]
1417a33e7: 4889c6  mov rsi, rax
1417a33ea: 4883f605  xor rsi, 5
1417a33ee: 488d0da18c5301  lea rcx, [rip + 0x1538ca1]
1417a33f5: 488d1d958c5301  lea rbx, [rip + 0x1538c95]
1417a33fc: 4885c0  test rax, rax
1417a33ff: 480f45d9  cmovne rbx, rcx
1417a3403: 498b3f  mov rdi, qword ptr [r15]
1417a3406: 4d8b7708  mov r14, qword ptr [r15 + 8]
1417a340a: 4d89f4  mov r12, r14
1417a340d: 4901f4  add r12, rsi
1417a3410: 49c7c0ffffffff  mov r8, 0xffffffffffffffff
1417a3417: 4d0f43c4  cmovae r8, r12
1417a341b: 488b07  mov rax, qword ptr [rdi]
1417a341e: 4939c0  cmp r8, rax
1417a3421: 7613  jbe 0x1417a3436
1417a3423: 488b5710  mov rdx, qword ptr [rdi + 0x10]
1417a3427: 4929d0  sub r8, rdx
1417a342a: 4829d0  sub rax, rdx
1417a342d: 4c39c0  cmp rax, r8
1417a3430: 0f820b190000  jb 0x1417a4d41
1417a3436: 488b4f10  mov rcx, qword ptr [rdi + 0x10]
1417a343a: 4d89f0  mov r8, r14
1417a343d: 4929c8  sub r8, rcx
1417a3440: 760f  jbe 0x1417a3451
1417a3442: 48034f08  add rcx, qword ptr [rdi + 8]
1417a3446: 31d2  xor edx, edx
1417a3448: e8d3071801  call 0x142923c20
1417a344d: 4c897710  mov qword ptr [rdi + 0x10], r14
1417a3451: 4c037708  add r14, qword ptr [rdi + 8]
1417a3455: 4c89f1  mov rcx, r14
1417a3458: 4889da  mov rdx, rbx
1417a345b: 4989f0  mov r8, rsi
1417a345e: e89d071801  call 0x142923c00
1417a3463: 4c3b6710  cmp r12, qword ptr [rdi + 0x10]
1417a3467: 7604  jbe 0x1417a346d
1417a3469: 4c896710  mov qword ptr [rdi + 0x10], r12
1417a346d: 4d896708  mov qword ptr [r15 + 8], r12
1417a3471: 488bbd90020000  mov rdi, qword ptr [rbp + 0x290]
1417a3478: 4c8bb5a0020000  mov r14, qword ptr [rbp + 0x2a0]
1417a347f: 488b37  mov rsi, qword ptr [rdi]
1417a3482: 488b5f08  mov rbx, qword ptr [rdi + 8]
1417a3486: 4d8d7e08  lea r15, [r14 + 8]
1417a348a: 4c89f8  mov rax, r15
1417a348d: 4883c008  add rax, 8
1417a3491: 49c7c0ffffffff  mov r8, 0xffffffffffffffff
1417a3498: 4c0f43c0  cmovae r8, rax
1417a349c: 4c897f08  mov qword ptr [rdi + 8], r15
1417a34a0: 488b06  mov rax, qword ptr [rsi]
1417a34a3: 488b5610  mov rdx, qword ptr [rsi + 0x10]
1417a34a7: 4939c0  cmp r8, rax
1417a34aa: 760f  jbe 0x1417a34bb
1417a34ac: 4929d0  sub r8, rdx
1417a34af: 4829d0  sub rax, rdx
1417a34b2: 4c39c0  cmp rax, r8
1417a34b5: 0f82ee170000  jb 0x1417a4ca9
1417a34bb: 4989dc  mov r12, rbx
1417a34be: 4d29f4  sub r12, r14
1417a34c1: 4983c4f0  add r12, -0x10
1417a34c5: 4d89f8  mov r8, r15
1417a34c8: 4929d0  sub r8, rdx
1417a34cb: 7612  jbe 0x1417a34df
1417a34cd: 48035608  add rdx, qword ptr [rsi + 8]
1417a34d1: 4889d1  mov rcx, rdx
1417a34d4: 31d2  xor edx, edx
1417a34d6: e845071801  call 0x142923c20
1417a34db: 4c897e10  mov qword ptr [rsi + 0x10], r15
1417a34df: 488b4608  mov rax, qword ptr [rsi + 8]
1417a34e3: 4e89643008  mov qword ptr [rax + r14 + 8], r12
1417a34e8: 4983c610  add r14, 0x10
1417a34ec: 4c3b7610  cmp r14, qword ptr [rsi + 0x10]
1417a34f0: 7604  jbe 0x1417a34f6
1417a34f2: 4c897610  mov qword ptr [rsi + 0x10], r14
1417a34f6: 48895f08  mov qword ptr [rdi + 8], rbx
1417a34fa: 488b8598020000  mov rax, qword ptr [rbp + 0x298]
1417a3501: 48898530030000  mov qword ptr [rbp + 0x330], rax
1417a3508: 48833800  cmp qword ptr [rax], 0
1417a350c: 740c  je 0x1417a351a
1417a350e: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a3515: e8e644e4fe  call 0x1405e7a00
1417a351a: 488b8530030000  mov rax, qword ptr [rbp + 0x330]
1417a3521: 48c70000000000  mov qword ptr [rax], 0
1417a3528: 488b9590010000  mov rdx, qword ptr [rbp + 0x190]
1417a352f: 488b8d98010000  mov rcx, qword ptr [rbp + 0x198]
1417a3536: 488b85a0010000  mov rax, qword ptr [rbp + 0x1a0]
1417a353d: 48c1e003  shl rax, 3
1417a3541: 4c8d0480  lea r8, [rax + rax*4]
1417a3545: 4901c8  add r8, rcx
1417a3548: 48898dd8010000  mov qword ptr [rbp + 0x1d8], rcx
1417a354f: 48899590000000  mov qword ptr [rbp + 0x90], rdx
1417a3556: 488995e8010000  mov qword ptr [rbp + 0x1e8], rdx
1417a355d: 4c8985c8010000  mov qword ptr [rbp + 0x1c8], r8
1417a3564: 4c8985f0010000  mov qword ptr [rbp + 0x1f0], r8
1417a356b: 4c8d6140  lea r12, [rcx + 0x40]
1417a356f: 488d3480  lea rsi, [rax + rax*4]
1417a3573: 4883c6d8  add rsi, -0x28
1417a3577: 48898d98000000  mov qword ptr [rbp + 0x98], rcx
1417a357e: 4889c8  mov rax, rcx
1417a3581: eb18  jmp 0x1417a359b
1417a3583: 666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a3590: 4983c428  add r12, 0x28
1417a3594: 4883c6d8  add rsi, -0x28
1417a3598: 4889f8  mov rax, rdi
1417a359b: 483b85c8010000  cmp rax, qword ptr [rbp + 0x1c8]
1417a35a2: 0f845c040000  je 0x1417a3a04
1417a35a8: 488d7828  lea rdi, [rax + 0x28]
1417a35ac: 4889bde0010000  mov qword ptr [rbp + 0x1e0], rdi
1417a35b3: 488b5010  mov rdx, qword ptr [rax + 0x10]
1417a35b7: 4889d1  mov rcx, rdx
1417a35ba: 48f7d9  neg rcx
1417a35bd: 0f80f4030000  jo 0x1417a39b7
1417a35c3: 4889b520020000  mov qword ptr [rbp + 0x220], rsi
1417a35ca: 48899540020000  mov qword ptr [rbp + 0x240], rdx
1417a35d1: 488b08  mov rcx, qword ptr [rax]
1417a35d4: 488b5008  mov rdx, qword ptr [rax + 8]
1417a35d8: 4c8b4018  mov r8, qword ptr [rax + 0x18]
1417a35dc: 4c898530030000  mov qword ptr [rbp + 0x330], r8
1417a35e3: 488b7020  mov rsi, qword ptr [rax + 0x20]
1417a35e7: e814d78d00  call 0x142080d00
1417a35ec: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a35f3: 488d5570  lea rdx, [rbp + 0x70]
1417a35f7: 4989c0  mov r8, rax
1417a35fa: e821f2daff  call 0x141552820
1417a35ff: 488b9dd0020000  mov rbx, qword ptr [rbp + 0x2d0]
1417a3606: 488b8dd8020000  mov rcx, qword ptr [rbp + 0x2d8]
1417a360d: 4885db  test rbx, rbx
1417a3610: 0f84c1050000  je 0x1417a3bd7
1417a3616: 488b85e0020000  mov rax, qword ptr [rbp + 0x2e0]
1417a361d: 48899d28020000  mov qword ptr [rbp + 0x228], rbx
1417a3624: 48898d30020000  mov qword ptr [rbp + 0x230], rcx
1417a362b: 48898538020000  mov qword ptr [rbp + 0x238], rax
1417a3632: 4c8b3b  mov r15, qword ptr [rbx]
1417a3635: 4c8b6b08  mov r13, qword ptr [rbx + 8]
1417a3639: 4d89ee  mov r14, r13
1417a363c: 4983c608  add r14, 8
1417a3640: 4d89f0  mov r8, r14
1417a3643: 48c7c0ffffffff  mov rax, 0xffffffffffffffff
1417a364a: 4c0f42c0  cmovb r8, rax
1417a364e: 498b07  mov rax, qword ptr [r15]
1417a3651: 4939c0  cmp r8, rax
1417a3654: 7613  jbe 0x1417a3669
1417a3656: 498b5710  mov rdx, qword ptr [r15 + 0x10]
1417a365a: 4929d0  sub r8, rdx
1417a365d: 4829d0  sub rax, rdx
1417a3660: 4c39c0  cmp rax, r8
1417a3663: 0f8240020000  jb 0x1417a38a9
1417a3669: 498b4f10  mov rcx, qword ptr [r15 + 0x10]
1417a366d: 4d89e8  mov r8, r13
1417a3670: 4929c8  sub r8, rcx
1417a3673: 760f  jbe 0x1417a3684
1417a3675: 49034f08  add rcx, qword ptr [r15 + 8]
1417a3679: 31d2  xor edx, edx
1417a367b: e8a0051801  call 0x142923c20
1417a3680: 4d896f10  mov qword ptr [r15 + 0x10], r13
1417a3684: 498b4708  mov rax, qword ptr [r15 + 8]
1417a3688: 4a893428  mov qword ptr [rax + r13], rsi
1417a368c: 4d3b7710  cmp r14, qword ptr [r15 + 0x10]
1417a3690: 7604  jbe 0x1417a3696
1417a3692: 4d897710  mov qword ptr [r15 + 0x10], r14
1417a3696: 4c897308  mov qword ptr [rbx + 8], r14
1417a369a: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a36a1: ba06000000  mov edx, 6
1417a36a6: 4531c0  xor r8d, r8d
1417a36a9: e892c441ff  call 0x140bbfb40
1417a36ae: 0fb6055415e901  movzx eax, byte ptr [rip + 0x1e91554]
1417a36b5: b900800000  mov ecx, 0x8000
1417a36ba: ba01000000  mov edx, 1
1417a36bf: e84c05e7fe  call 0x140613c10
1417a36c4: 4885c0  test rax, rax
1417a36c7: 0f8412170000  je 0x1417a4ddf
1417a36cd: 488d8d28020000  lea rcx, [rbp + 0x228]
1417a36d4: 48898dc0020000  mov qword ptr [rbp + 0x2c0], rcx
1417a36db: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a36e2: 488d95a8020000  lea rdx, [rbp + 0x2a8]
1417a36e9: 48894a10  mov qword ptr [rdx + 0x10], rcx
1417a36ed: f30f6f85d0020000  movdqu xmm0, xmmword ptr [rbp + 0x2d0]
1417a36f5: f30f7f02  movdqu xmmword ptr [rdx], xmm0
1417a36f9: 48c7859002000000800000  mov qword ptr [rbp + 0x290], 0x8000
1417a3704: 48898598020000  mov qword ptr [rbp + 0x298], rax
1417a370b: 48c785a002000000000000  mov qword ptr [rbp + 0x2a0], 0
1417a3716: c6852f03000001  mov byte ptr [rbp + 0x32f], 1
1417a371d: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a3724: 488b9530030000  mov rdx, qword ptr [rbp + 0x330]
1417a372b: 4989f0  mov r8, rsi
1417a372e: e8bd0f27ff  call 0x140a146f0
1417a3733: 4885c0  test rax, rax
1417a3736: 0f85d4040000  jne 0x1417a3c10
1417a373c: 488b85c0020000  mov rax, qword ptr [rbp + 0x2c0]
1417a3743: 48898500030000  mov qword ptr [rbp + 0x300], rax
1417a374a: f30f6f8590020000  movdqu xmm0, xmmword ptr [rbp + 0x290]
1417a3752: 0f108da0020000  movups xmm1, xmmword ptr [rbp + 0x2a0]
1417a3759: 0f1095b0020000  movups xmm2, xmmword ptr [rbp + 0x2b0]
1417a3760: 0f2995f0020000  movaps xmmword ptr [rbp + 0x2f0], xmm2
1417a3767: 0f298de0020000  movaps xmmword ptr [rbp + 0x2e0], xmm1
1417a376e: 660f7f85d0020000  movdqa xmmword ptr [rbp + 0x2d0], xmm0
1417a3776: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a377d: e81e1f35ff  call 0x140af56a0
1417a3782: 4889c6  mov rsi, rax
1417a3785: 4989c7  mov r15, rax
1417a3788: 4885c0  test rax, rax
1417a378b: 751b  jne 0x1417a37a8
1417a378d: 4c8bbd00030000  mov r15, qword ptr [rbp + 0x300]
1417a3794: 48c7850003000000000000  mov qword ptr [rbp + 0x300], 0
1417a379f: 4d85ff  test r15, r15
1417a37a2: 0f84141b0000  je 0x1417a52bc
1417a37a8: c6852f03000000  mov byte ptr [rbp + 0x32f], 0
1417a37af: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a37b6: e8d5e00dff  call 0x140881890
1417a37bb: 4885f6  test rsi, rsi
1417a37be: 0f85f2040000  jne 0x1417a3cb6
1417a37c4: 4c8bbd28020000  mov r15, qword ptr [rbp + 0x228]
1417a37cb: 4c8bad38020000  mov r13, qword ptr [rbp + 0x238]
1417a37d2: 498b37  mov rsi, qword ptr [r15]
1417a37d5: 498b5f08  mov rbx, qword ptr [r15 + 8]
1417a37d9: 4d8d7508  lea r14, [r13 + 8]
1417a37dd: 4d89f0  mov r8, r14
1417a37e0: 4983c008  add r8, 8
1417a37e4: 48c7c0ffffffff  mov rax, 0xffffffffffffffff
1417a37eb: 4c0f42c0  cmovb r8, rax
1417a37ef: 4d897708  mov qword ptr [r15 + 8], r14
1417a37f3: 488b06  mov rax, qword ptr [rsi]
1417a37f6: 488b5610  mov rdx, qword ptr [rsi + 0x10]
1417a37fa: 4939c0  cmp r8, rax
1417a37fd: 760f  jbe 0x1417a380e
1417a37ff: 4929d0  sub r8, rdx
1417a3802: 4829d0  sub rax, rdx
1417a3805: 4c39c0  cmp rax, r8
1417a3808: 0f82b7000000  jb 0x1417a38c5
1417a380e: 4d89f0  mov r8, r14
1417a3811: 4929d0  sub r8, rdx
1417a3814: 7612  jbe 0x1417a3828
1417a3816: 48035608  add rdx, qword ptr [rsi + 8]
1417a381a: 4889d1  mov rcx, rdx
1417a381d: 31d2  xor edx, edx
1417a381f: e8fc031801  call 0x142923c20
1417a3824: 4c897610  mov qword ptr [rsi + 0x10], r14
1417a3828: 4889d8  mov rax, rbx
1417a382b: 4c29e8  sub rax, r13
1417a382e: 4883c0f0  add rax, -0x10
1417a3832: 488b4e08  mov rcx, qword ptr [rsi + 8]
1417a3836: 4a89442908  mov qword ptr [rcx + r13 + 8], rax
1417a383b: 4983c510  add r13, 0x10
1417a383f: 4c3b6e10  cmp r13, qword ptr [rsi + 0x10]
1417a3843: 7604  jbe 0x1417a3849
1417a3845: 4c896e10  mov qword ptr [rsi + 0x10], r13
1417a3849: 49895f08  mov qword ptr [r15 + 8], rbx
1417a384d: 488b8530020000  mov rax, qword ptr [rbp + 0x230]
1417a3854: 48898588020000  mov qword ptr [rbp + 0x288], rax
1417a385b: 48833800  cmp qword ptr [rax], 0
1417a385f: 740c  je 0x1417a386d
1417a3861: 488b8d88020000  mov rcx, qword ptr [rbp + 0x288]
1417a3868: e89341e4fe  call 0x1405e7a00
1417a386d: 488b8588020000  mov rax, qword ptr [rbp + 0x288]
1417a3874: 48c70000000000  mov qword ptr [rax], 0
1417a387b: 488b9540020000  mov rdx, qword ptr [rbp + 0x240]
1417a3882: 4885d2  test rdx, rdx
1417a3885: 488bb520020000  mov rsi, qword ptr [rbp + 0x220]
1417a388c: 0f84fefcffff  je 0x1417a3590
1417a3892: 41b801000000  mov r8d, 1
1417a3898: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a389f: e87c03e7fe  call 0x140613c20
1417a38a4: e9e7fcffff  jmp 0x1417a3590
1417a38a9: 48c744242001000000  mov qword ptr [rsp + 0x20], 1
1417a38b2: 41b901000000  mov r9d, 1
1417a38b8: 4c89f9  mov rcx, r15
1417a38bb: e8a0611001  call 0x1428a9a60
1417a38c0: e9a4fdffff  jmp 0x1417a3669
1417a38c5: 48c744242001000000  mov qword ptr [rsp + 0x20], 1
1417a38ce: 41b901000000  mov r9d, 1
1417a38d4: 4889f1  mov rcx, rsi
1417a38d7: e884611001  call 0x1428a9a60
1417a38dc: 488b5610  mov rdx, qword ptr [rsi + 0x10]
1417a38e0: 4d89f0  mov r8, r14
1417a38e3: 4929d0  sub r8, rdx
1417a38e6: 0f872affffff  ja 0x1417a3816
1417a38ec: e937ffffff  jmp 0x1417a3828
1417a38f1: 488b9d18020000  mov rbx, qword ptr [rbp + 0x218]
1417a38f8: 498b37  mov rsi, qword ptr [r15]
1417a38fb: 4d8b6708  mov r12, qword ptr [r15 + 8]
1417a38ff: 4d89e6  mov r14, r12
1417a3902: 49ffc6  inc r14
1417a3905: 49c7c0ffffffff  mov r8, 0xffffffffffffffff
1417a390c: 4d0f45c6  cmovne r8, r14
1417a3910: 488b06  mov rax, qword ptr [rsi]
1417a3913: 4939c0  cmp r8, rax
1417a3916: 7613  jbe 0x1417a392b
1417a3918: 488b5610  mov rdx, qword ptr [rsi + 0x10]
1417a391c: 4929d0  sub r8, rdx
1417a391f: 4829d0  sub rax, rdx
1417a3922: 4c39c0  cmp rax, r8
1417a3925: 0f8232140000  jb 0x1417a4d5d
1417a392b: 488b4e10  mov rcx, qword ptr [rsi + 0x10]
1417a392f: 4d89e0  mov r8, r12
1417a3932: 4929c8  sub r8, rcx
1417a3935: 762f  jbe 0x1417a3966
1417a3937: 48034e08  add rcx, qword ptr [rsi + 8]
1417a393b: 31d2  xor edx, edx
1417a393d: e8de021801  call 0x142923c20
1417a3942: 4c896610  mov qword ptr [rsi + 0x10], r12
1417a3946: 488b4608  mov rax, qword ptr [rsi + 8]
1417a394a: 42c604207b  mov byte ptr [rax + r12], 0x7b
1417a394f: 4c3b7610  cmp r14, qword ptr [rsi + 0x10]
1417a3953: 7720  ja 0x1417a3975
1417a3955: 4d897708  mov qword ptr [r15 + 8], r14
1417a3959: b001  mov al, 1
1417a395b: 4885db  test rbx, rbx
1417a395e: 0f85d1090000  jne 0x1417a4335
1417a3964: eb22  jmp 0x1417a3988
1417a3966: 488b4608  mov rax, qword ptr [rsi + 8]
1417a396a: 42c604207b  mov byte ptr [rax + r12], 0x7b
1417a396f: 4c3b7610  cmp r14, qword ptr [rsi + 0x10]
1417a3973: 76e0  jbe 0x1417a3955
1417a3975: 4c897610  mov qword ptr [rsi + 0x10], r14
1417a3979: 4d897708  mov qword ptr [r15 + 8], r14
1417a397d: b001  mov al, 1
1417a397f: 4885db  test rbx, rbx
1417a3982: 0f85ad090000  jne 0x1417a4335
1417a3988: 488d1501875301  lea rdx, [rip + 0x1538701]
1417a398f: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a3996: 41b801000000  mov r8d, 1
1417a399c: e84fc3daff  call 0x14154fcf0
1417a39a1: 4885c0  test rax, rax
1417a39a4: 0f8489090000  je 0x1417a4333
1417a39aa: 4889c1  mov rcx, rax
1417a39ad: e8ee5e1201  call 0x1428c98a0
1417a39b2: e97b060000  jmp 0x1417a4032
1417a39b7: 4839bdc8010000  cmp qword ptr [rbp + 0x1c8], rdi
1417a39be: 7444  je 0x1417a3a04
1417a39c0: 48b9cdcccccccccccccc  movabs rcx, 0xcccccccccccccccd
1417a39ca: 4889f0  mov rax, rsi
1417a39cd: 48f7e1  mul rcx
1417a39d0: 4889d6  mov rsi, rdx
1417a39d3: 48c1ee05  shr rsi, 5
1417a39d7: eb10  jmp 0x1417a39e9
1417a39d9: 0f1f8000000000  nop dword ptr [rax]
1417a39e0: 4983c428  add r12, 0x28
1417a39e4: 48ffce  dec rsi
1417a39e7: 741b  je 0x1417a3a04
1417a39e9: 498b5424f8  mov rdx, qword ptr [r12 - 8]
1417a39ee: 4885d2  test rdx, rdx
1417a39f1: 74ed  je 0x1417a39e0
1417a39f3: 498b0c24  mov rcx, qword ptr [r12]
1417a39f7: 41b801000000  mov r8d, 1
1417a39fd: e81e02e7fe  call 0x140613c20
1417a3a02: ebdc  jmp 0x1417a39e0
1417a3a04: 488b8590000000  mov rax, qword ptr [rbp + 0x90]
1417a3a0b: 4885c0  test rax, rax
1417a3a0e: 741a  je 0x1417a3a2a
1417a3a10: 48c1e003  shl rax, 3
1417a3a14: 488d1480  lea rdx, [rax + rax*4]
1417a3a18: 41b808000000  mov r8d, 8
1417a3a1e: 488b8d98000000  mov rcx, qword ptr [rbp + 0x98]
1417a3a25: e8f601e7fe  call 0x140613c20
1417a3a2a: c6853f03000000  mov byte ptr [rbp + 0x33f], 0
1417a3a31: c6853e03000001  mov byte ptr [rbp + 0x33e], 1
1417a3a38: 4c8d057b8b5301  lea r8, [rip + 0x1538b7b]
1417a3a3f: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a3a46: 488d5570  lea rdx, [rbp + 0x70]
1417a3a4a: 41b907000000  mov r9d, 7
1417a3a50: e83becdaff  call 0x141552690
1417a3a55: 488b85d0020000  mov rax, qword ptr [rbp + 0x2d0]
1417a3a5c: 488b8dd8020000  mov rcx, qword ptr [rbp + 0x2d8]
1417a3a63: 4885c0  test rax, rax
1417a3a66: 0f84f0030000  je 0x1417a3e5c
1417a3a6c: 488b95e0020000  mov rdx, qword ptr [rbp + 0x2e0]
1417a3a73: 488985d8010000  mov qword ptr [rbp + 0x1d8], rax
1417a3a7a: 48898de0010000  mov qword ptr [rbp + 0x1e0], rcx
1417a3a81: 488995e8010000  mov qword ptr [rbp + 0x1e8], rdx
1417a3a88: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a3a8f: ba06000000  mov edx, 6
1417a3a94: 4531c0  xor r8d, r8d
1417a3a97: e8a4c041ff  call 0x140bbfb40
1417a3a9c: 488bbd48020000  mov rdi, qword ptr [rbp + 0x248]
1417a3aa3: 0fb6055f11e901  movzx eax, byte ptr [rip + 0x1e9115f]
1417a3aaa: b900800000  mov ecx, 0x8000
1417a3aaf: ba01000000  mov edx, 1
1417a3ab4: e85701e7fe  call 0x140613c10
1417a3ab9: 4885c0  test rax, rax
1417a3abc: 4c8b4538  mov r8, qword ptr [rbp + 0x38]
1417a3ac0: 0f84e5180000  je 0x1417a53ab
1417a3ac6: 488d8dd8010000  lea rcx, [rbp + 0x1d8]
1417a3acd: 48898dc0020000  mov qword ptr [rbp + 0x2c0], rcx
1417a3ad4: f30f6f85d0020000  movdqu xmm0, xmmword ptr [rbp + 0x2d0]
1417a3adc: f30f7f85a8020000  movdqu xmmword ptr [rbp + 0x2a8], xmm0
1417a3ae4: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a3aeb: 48898db8020000  mov qword ptr [rbp + 0x2b8], rcx
1417a3af2: 48c7859002000000800000  mov qword ptr [rbp + 0x290], 0x8000
1417a3afd: 48898598020000  mov qword ptr [rbp + 0x298], rax
1417a3b04: 48c785a002000000000000  mov qword ptr [rbp + 0x2a0], 0
1417a3b0f: c6852e03000001  mov byte ptr [rbp + 0x32e], 1
1417a3b16: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a3b1d: 488b95f8010000  mov rdx, qword ptr [rbp + 0x1f8]
1417a3b24: e8c70b27ff  call 0x140a146f0
1417a3b29: 4889c6  mov rsi, rax
1417a3b2c: 4885c0  test rax, rax
1417a3b2f: 0f8457030000  je 0x1417a3e8c
1417a3b35: 4889b528020000  mov qword ptr [rbp + 0x228], rsi
1417a3b3c: c685cf02000001  mov byte ptr [rbp + 0x2cf], 1
1417a3b43: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a3b4a: e8b12b5bff  call 0x140d56700
1417a3b4f: f30f6f85d0020000  movdqu xmm0, xmmword ptr [rbp + 0x2d0]
1417a3b57: 0f108de0020000  movups xmm1, xmmword ptr [rbp + 0x2e0]
1417a3b5e: 0f1095f0020000  movups xmm2, xmmword ptr [rbp + 0x2f0]
1417a3b65: 0f299570020000  movaps xmmword ptr [rbp + 0x270], xmm2
1417a3b6c: 0f298d60020000  movaps xmmword ptr [rbp + 0x260], xmm1
1417a3b73: 660f7f8550020000  movdqa xmmword ptr [rbp + 0x250], xmm0
1417a3b7b: 488d05848a5301  lea rax, [rip + 0x1538a84]
1417a3b82: 488985d0020000  mov qword ptr [rbp + 0x2d0], rax
1417a3b89: 48c785d802000018000000  mov qword ptr [rbp + 0x2d8], 0x18
1417a3b94: 4889b5e0020000  mov qword ptr [rbp + 0x2e0], rsi
1417a3b9b: c685cf02000000  mov byte ptr [rbp + 0x2cf], 0
1417a3ba2: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a3ba9: 488d9550020000  lea rdx, [rbp + 0x250]
1417a3bb0: e8cb521001  call 0x1428a8e80
1417a3bb5: 48894708  mov qword ptr [rdi + 8], rax
1417a3bb9: 48b80000000000000080  movabs rax, 0x8000000000000000
1417a3bc3: 488907  mov qword ptr [rdi], rax
1417a3bc6: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a3bcd: e8bedc0dff  call 0x140881890
1417a3bd2: e92b030000  jmp 0x1417a3f02
1417a3bd7: e8b4591001  call 0x1428a9590
1417a3bdc: 488b8d48020000  mov rcx, qword ptr [rbp + 0x248]
1417a3be3: 48894108  mov qword ptr [rcx + 8], rax
1417a3be7: 48b80000000000000080  movabs rax, 0x8000000000000000
1417a3bf1: 488901  mov qword ptr [rcx], rax
1417a3bf4: 488bb520020000  mov rsi, qword ptr [rbp + 0x220]
1417a3bfb: 488b9540020000  mov rdx, qword ptr [rbp + 0x240]
1417a3c02: 4885d2  test rdx, rdx
1417a3c05: 0f85a7010000  jne 0x1417a3db2
1417a3c0b: e9b4010000  jmp 0x1417a3dc4
1417a3c10: 4889c6  mov rsi, rax
1417a3c13: 48894528  mov qword ptr [rbp + 0x28], rax
1417a3c17: c685cc02000001  mov byte ptr [rbp + 0x2cc], 1
1417a3c1e: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a3c25: e8d62a5bff  call 0x140d56700
1417a3c2a: f30f6f85d0020000  movdqu xmm0, xmmword ptr [rbp + 0x2d0]
1417a3c32: 0f108de0020000  movups xmm1, xmmword ptr [rbp + 0x2e0]
1417a3c39: 0f1095f0020000  movups xmm2, xmmword ptr [rbp + 0x2f0]
1417a3c40: 0f299570020000  movaps xmmword ptr [rbp + 0x270], xmm2
1417a3c47: 0f298d60020000  movaps xmmword ptr [rbp + 0x260], xmm1
1417a3c4e: 660f7f8550020000  movdqa xmmword ptr [rbp + 0x250], xmm0
1417a3c56: 488d05c1895301  lea rax, [rip + 0x15389c1]
1417a3c5d: 488985d0020000  mov qword ptr [rbp + 0x2d0], rax
1417a3c64: 48c785d802000017000000  mov qword ptr [rbp + 0x2d8], 0x17
1417a3c6f: 4889b5e0020000  mov qword ptr [rbp + 0x2e0], rsi
1417a3c76: c685cc02000000  mov byte ptr [rbp + 0x2cc], 0
1417a3c7d: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a3c84: 488d9550020000  lea rdx, [rbp + 0x250]
1417a3c8b: e8f0511001  call 0x1428a8e80
1417a3c90: 488b8d48020000  mov rcx, qword ptr [rbp + 0x248]
1417a3c97: 48894108  mov qword ptr [rcx + 8], rax
1417a3c9b: 48b80000000000000080  movabs rax, 0x8000000000000000
1417a3ca5: 488901  mov qword ptr [rcx], rax
1417a3ca8: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a3caf: e8dcdb0dff  call 0x140881890
1417a3cb4: eb27  jmp 0x1417a3cdd
1417a3cb6: c6852f03000000  mov byte ptr [rbp + 0x32f], 0
1417a3cbd: 4c89f9  mov rcx, r15
1417a3cc0: e8cb581001  call 0x1428a9590
1417a3cc5: 488b8d48020000  mov rcx, qword ptr [rbp + 0x248]
1417a3ccc: 48894108  mov qword ptr [rcx + 8], rax
1417a3cd0: 48b80000000000000080  movabs rax, 0x8000000000000000
1417a3cda: 488901  mov qword ptr [rcx], rax
1417a3cdd: 4c8bb528020000  mov r14, qword ptr [rbp + 0x228]
1417a3ce4: 4c8bbd38020000  mov r15, qword ptr [rbp + 0x238]
1417a3ceb: 498b36  mov rsi, qword ptr [r14]
1417a3cee: 4d8b6e08  mov r13, qword ptr [r14 + 8]
1417a3cf2: 498d5f08  lea rbx, [r15 + 8]
1417a3cf6: 4889d8  mov rax, rbx
1417a3cf9: 4883c008  add rax, 8
1417a3cfd: 49c7c0ffffffff  mov r8, 0xffffffffffffffff
1417a3d04: 4c0f43c0  cmovae r8, rax
1417a3d08: 49895e08  mov qword ptr [r14 + 8], rbx
1417a3d0c: 488b06  mov rax, qword ptr [rsi]
1417a3d0f: 488b5610  mov rdx, qword ptr [rsi + 0x10]
1417a3d13: 4939c0  cmp r8, rax
1417a3d16: 760f  jbe 0x1417a3d27
1417a3d18: 4929d0  sub r8, rdx
1417a3d1b: 4829d0  sub rax, rdx
1417a3d1e: 4c39c0  cmp rax, r8
1417a3d21: 0f82de0f0000  jb 0x1417a4d05
1417a3d27: 4c89ad88020000  mov qword ptr [rbp + 0x288], r13
1417a3d2e: 4d29fd  sub r13, r15
1417a3d31: 4983c5f0  add r13, -0x10
1417a3d35: 4989d8  mov r8, rbx
1417a3d38: 4929d0  sub r8, rdx
1417a3d3b: 7612  jbe 0x1417a3d4f
1417a3d3d: 48035608  add rdx, qword ptr [rsi + 8]
1417a3d41: 4889d1  mov rcx, rdx
1417a3d44: 31d2  xor edx, edx
1417a3d46: e8d5fe1701  call 0x142923c20
1417a3d4b: 48895e10  mov qword ptr [rsi + 0x10], rbx
1417a3d4f: 488b4608  mov rax, qword ptr [rsi + 8]
1417a3d53: 4e896c3808  mov qword ptr [rax + r15 + 8], r13
1417a3d58: 4983c710  add r15, 0x10
1417a3d5c: 4c3b7e10  cmp r15, qword ptr [rsi + 0x10]
1417a3d60: 7604  jbe 0x1417a3d66
1417a3d62: 4c897e10  mov qword ptr [rsi + 0x10], r15
1417a3d66: 488b8588020000  mov rax, qword ptr [rbp + 0x288]
1417a3d6d: 49894608  mov qword ptr [r14 + 8], rax
1417a3d71: 488b8530020000  mov rax, qword ptr [rbp + 0x230]
1417a3d78: 48898588020000  mov qword ptr [rbp + 0x288], rax
1417a3d7f: 48833800  cmp qword ptr [rax], 0
1417a3d83: 740c  je 0x1417a3d91
1417a3d85: 488b8d88020000  mov rcx, qword ptr [rbp + 0x288]
1417a3d8c: e86f3ce4fe  call 0x1405e7a00
1417a3d91: 488b8588020000  mov rax, qword ptr [rbp + 0x288]
1417a3d98: 48c70000000000  mov qword ptr [rax], 0
1417a3d9f: 488bb520020000  mov rsi, qword ptr [rbp + 0x220]
1417a3da6: 488b9540020000  mov rdx, qword ptr [rbp + 0x240]
1417a3dad: 4885d2  test rdx, rdx
1417a3db0: 7412  je 0x1417a3dc4
1417a3db2: 41b801000000  mov r8d, 1
1417a3db8: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a3dbf: e85cfee6fe  call 0x140613c20
1417a3dc4: 4839bdc8010000  cmp qword ptr [rbp + 0x1c8], rdi
1417a3dcb: 7447  je 0x1417a3e14
1417a3dcd: 48b9cdcccccccccccccc  movabs rcx, 0xcccccccccccccccd
1417a3dd7: 4889f0  mov rax, rsi
1417a3dda: 48f7e1  mul rcx
1417a3ddd: 4889d6  mov rsi, rdx
1417a3de0: 48c1ee05  shr rsi, 5
1417a3de4: eb13  jmp 0x1417a3df9
1417a3de6: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a3df0: 4983c428  add r12, 0x28
1417a3df4: 48ffce  dec rsi
1417a3df7: 741b  je 0x1417a3e14
1417a3df9: 498b5424f8  mov rdx, qword ptr [r12 - 8]
1417a3dfe: 4885d2  test rdx, rdx
1417a3e01: 74ed  je 0x1417a3df0
1417a3e03: 498b0c24  mov rcx, qword ptr [r12]
1417a3e07: 41b801000000  mov r8d, 1
1417a3e0d: e80efee6fe  call 0x140613c20
1417a3e12: ebdc  jmp 0x1417a3df0
1417a3e14: 488b8590000000  mov rax, qword ptr [rbp + 0x90]
1417a3e1b: 4885c0  test rax, rax
1417a3e1e: 741a  je 0x1417a3e3a
1417a3e20: 48c1e003  shl rax, 3
1417a3e24: 488d1480  lea rdx, [rax + rax*4]
1417a3e28: 41b808000000  mov r8d, 8
1417a3e2e: 488b8d98000000  mov rcx, qword ptr [rbp + 0x98]
1417a3e35: e8e6fde6fe  call 0x140613c20
1417a3e3a: 40b601  mov sil, 1
1417a3e3d: 4531ff  xor r15d, r15d
1417a3e40: 4c8bb548020000  mov r14, qword ptr [rbp + 0x248]
1417a3e47: 0fb68500020000  movzx eax, byte ptr [rbp + 0x200]
1417a3e4e: 83f803  cmp eax, 3
1417a3e51: 0f83c8060000  jae 0x1417a451f
1417a3e57: e95e070000  jmp 0x1417a45ba
1417a3e5c: c6853f03000000  mov byte ptr [rbp + 0x33f], 0
1417a3e63: c6853e03000001  mov byte ptr [rbp + 0x33e], 1
1417a3e6a: e821571001  call 0x1428a9590
1417a3e6f: 4c8bb548020000  mov r14, qword ptr [rbp + 0x248]
1417a3e76: 49894608  mov qword ptr [r14 + 8], rax
1417a3e7a: 48b80000000000000080  movabs rax, 0x8000000000000000
1417a3e84: 498906  mov qword ptr [r14], rax
1417a3e87: e934010000  jmp 0x1417a3fc0
1417a3e8c: 488b85c0020000  mov rax, qword ptr [rbp + 0x2c0]
1417a3e93: 48898500030000  mov qword ptr [rbp + 0x300], rax
1417a3e9a: f30f6f8590020000  movdqu xmm0, xmmword ptr [rbp + 0x290]
1417a3ea2: 0f108da0020000  movups xmm1, xmmword ptr [rbp + 0x2a0]
1417a3ea9: 0f1095b0020000  movups xmm2, xmmword ptr [rbp + 0x2b0]
1417a3eb0: 0f2995f0020000  movaps xmmword ptr [rbp + 0x2f0], xmm2
1417a3eb7: 0f298de0020000  movaps xmmword ptr [rbp + 0x2e0], xmm1
1417a3ebe: 660f7f85d0020000  movdqa xmmword ptr [rbp + 0x2d0], xmm0
1417a3ec6: c6852e03000000  mov byte ptr [rbp + 0x32e], 0
1417a3ecd: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a3ed4: e8471327ff  call 0x140a15220
1417a3ed9: 4885c0  test rax, rax
1417a3edc: 0f849c0a0000  je 0x1417a497e
1417a3ee2: c6852e03000000  mov byte ptr [rbp + 0x32e], 0
1417a3ee9: 4889d1  mov rcx, rdx
1417a3eec: e89f561001  call 0x1428a9590
1417a3ef1: 48894708  mov qword ptr [rdi + 8], rax
1417a3ef5: 48b80000000000000080  movabs rax, 0x8000000000000000
1417a3eff: 488907  mov qword ptr [rdi], rax
1417a3f02: 488bbdd8010000  mov rdi, qword ptr [rbp + 0x1d8]
1417a3f09: 4c8bb5e8010000  mov r14, qword ptr [rbp + 0x1e8]
1417a3f10: 488b37  mov rsi, qword ptr [rdi]
1417a3f13: 488b5f08  mov rbx, qword ptr [rdi + 8]
1417a3f17: 4d8d7e08  lea r15, [r14 + 8]
1417a3f1b: 4c89f8  mov rax, r15
1417a3f1e: 4883c008  add rax, 8
1417a3f22: 49c7c0ffffffff  mov r8, 0xffffffffffffffff
1417a3f29: 4c0f43c0  cmovae r8, rax
1417a3f2d: 4c897f08  mov qword ptr [rdi + 8], r15
1417a3f31: 488b06  mov rax, qword ptr [rsi]
1417a3f34: 488b5610  mov rdx, qword ptr [rsi + 0x10]
1417a3f38: 4939c0  cmp r8, rax
1417a3f3b: 760f  jbe 0x1417a3f4c
1417a3f3d: 4929d0  sub r8, rdx
1417a3f40: 4829d0  sub rax, rdx
1417a3f43: 4c39c0  cmp rax, r8
1417a3f46: 0f822d0e0000  jb 0x1417a4d79
1417a3f4c: 4989dc  mov r12, rbx
1417a3f4f: 4d29f4  sub r12, r14
1417a3f52: 4983c4f0  add r12, -0x10
1417a3f56: 4d89f8  mov r8, r15
1417a3f59: 4929d0  sub r8, rdx
1417a3f5c: 7612  jbe 0x1417a3f70
1417a3f5e: 48035608  add rdx, qword ptr [rsi + 8]
1417a3f62: 4889d1  mov rcx, rdx
1417a3f65: 31d2  xor edx, edx
1417a3f67: e8b4fc1701  call 0x142923c20
1417a3f6c: 4c897e10  mov qword ptr [rsi + 0x10], r15
1417a3f70: 488b4608  mov rax, qword ptr [rsi + 8]
1417a3f74: 4e89643008  mov qword ptr [rax + r14 + 8], r12
1417a3f79: 4983c610  add r14, 0x10
1417a3f7d: 4c3b7610  cmp r14, qword ptr [rsi + 0x10]
1417a3f81: 7604  jbe 0x1417a3f87
1417a3f83: 4c897610  mov qword ptr [rsi + 0x10], r14
1417a3f87: 48895f08  mov qword ptr [rdi + 8], rbx
1417a3f8b: 488b85e0010000  mov rax, qword ptr [rbp + 0x1e0]
1417a3f92: 48898530030000  mov qword ptr [rbp + 0x330], rax
1417a3f99: 48833800  cmp qword ptr [rax], 0
1417a3f9d: 4c8bb548020000  mov r14, qword ptr [rbp + 0x248]
1417a3fa4: 740c  je 0x1417a3fb2
1417a3fa6: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a3fad: e84e3ae4fe  call 0x1405e7a00
1417a3fb2: 488b8530030000  mov rax, qword ptr [rbp + 0x330]
1417a3fb9: 48c70000000000  mov qword ptr [rax], 0
1417a3fc0: 40b601  mov sil, 1
1417a3fc3: 4531ff  xor r15d, r15d
1417a3fc6: 0fb68500020000  movzx eax, byte ptr [rbp + 0x200]
1417a3fcd: 83f803  cmp eax, 3
1417a3fd0: 0f8349050000  jae 0x1417a451f
1417a3fd6: e9df050000  jmp 0x1417a45ba
1417a3fdb: f30f7e8510020000  movq xmm0, qword ptr [rbp + 0x210]
1417a3fe3: 66480f7ec0  movq rax, xmm0
1417a3fe8: 48b9ffffffffffffff7f  movabs rcx, 0x7fffffffffffffff
1417a3ff2: 4821c1  and rcx, rax
1417a3ff5: 48b8000000000000f07f  movabs rax, 0x7ff0000000000000
1417a3fff: 4839c1  cmp rcx, rax
1417a4002: 0f8ca3010000  jl 0x1417a41ab
1417a4008: 488d158b805301  lea rdx, [rip + 0x153808b]
1417a400f: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a4016: 41b804000000  mov r8d, 4
1417a401c: e8cfbcdaff  call 0x14154fcf0
1417a4021: 4885c0  test rax, rax
1417a4024: 0f8447f4ffff  je 0x1417a3471
1417a402a: 4889c1  mov rcx, rax
1417a402d: e86e581201  call 0x1428c98a0
1417a4032: 4889c7  mov rdi, rax
1417a4035: e9ea030000  jmp 0x1417a4424
1417a403a: 488b8d10020000  mov rcx, qword ptr [rbp + 0x210]
1417a4041: 41be14000000  mov r14d, 0x14
1417a4047: 4881f910270000  cmp rcx, 0x2710
1417a404e: 0f82ed010000  jb 0x1417a4241
1417a4054: 41ba14000000  mov r10d, 0x14
1417a405a: 49b84b598638d6c56d34  movabs r8, 0x346dc5d63886594b
1417a4064: 4c8d0da0815301  lea r9, [rip + 0x15381a0]
1417a406b: 0f1f440000  nop dword ptr [rax + rax]
1417a4070: 4889c8  mov rax, rcx
1417a4073: 49f7e0  mul r8
1417a4076: 48c1ea0b  shr rdx, 0xb
1417a407a: 69c210270000  imul eax, edx, 0x2710
1417a4080: 4189cb  mov r11d, ecx
1417a4083: 4129c3  sub r11d, eax
1417a4086: 4169c37b140000  imul eax, r11d, 0x147b
1417a408d: c1e813  shr eax, 0x13
1417a4090: 6bf064  imul esi, eax, 0x64
1417a4093: 4129f3  sub r11d, esi
1417a4096: 450fb7db  movzx r11d, r11w
1417a409a: 4d8d72fc  lea r14, [r10 - 4]
1417a409e: 410fb70441  movzx eax, word ptr [r9 + rax*2]
1417a40a3: 6642898415cc020000  mov word ptr [rbp + r10 + 0x2cc], ax
1417a40ac: 430fb70459  movzx eax, word ptr [r9 + r11*2]
1417a40b1: 6642898415ce020000  mov word ptr [rbp + r10 + 0x2ce], ax
1417a40ba: 4d89f2  mov r10, r14
1417a40bd: 4881f9ffe0f505  cmp rcx, 0x5f5e0ff
1417a40c4: 4889d1  mov rcx, rdx
1417a40c7: 77a7  ja 0x1417a4070
1417a40c9: e976010000  jmp 0x1417a4244
1417a40ce: 4889ca  mov rdx, rcx
1417a40d1: 4883fa63  cmp rdx, 0x63
1417a40d5: 7632  jbe 0x1417a4109
1417a40d7: 0fb7c2  movzx eax, dx
1417a40da: c1e802  shr eax, 2
1417a40dd: 69c07b140000  imul eax, eax, 0x147b
1417a40e3: c1e811  shr eax, 0x11
1417a40e6: 6bc864  imul ecx, eax, 0x64
1417a40e9: 29ca  sub edx, ecx
1417a40eb: 0fb7ca  movzx ecx, dx
1417a40ee: 488d1516815301  lea rdx, [rip + 0x1538116]
1417a40f5: 0fb70c4a  movzx ecx, word ptr [rdx + rcx*2]
1417a40f9: 6642898c35ce020000  mov word ptr [rbp + r14 + 0x2ce], cx
1417a4102: 4983c6fe  add r14, -2
1417a4106: 4889c2  mov rdx, rax
1417a4109: 4883fa0a  cmp rdx, 0xa
1417a410d: 7310  jae 0x1417a411f
1417a410f: 80ca30  or dl, 0x30
1417a4112: 42889435cf020000  mov byte ptr [rbp + r14 + 0x2cf], dl
1417a411a: 49ffce  dec r14
1417a411d: eb18  jmp 0x1417a4137
1417a411f: 488d05e5805301  lea rax, [rip + 0x15380e5]
1417a4126: 0fb70450  movzx eax, word ptr [rax + rdx*2]
1417a412a: 6642898435ce020000  mov word ptr [rbp + r14 + 0x2ce], ax
1417a4133: 4983c6fe  add r14, -2
1417a4137: 4d85c0  test r8, r8
1417a413a: 790c  jns 0x1417a4148
1417a413c: 42c68435cf0200002d  mov byte ptr [rbp + r14 + 0x2cf], 0x2d
1417a4145: 49ffce  dec r14
1417a4148: 4c89f6  mov rsi, r14
1417a414b: 4883ee14  sub rsi, 0x14
1417a414f: 0f841cf3ffff  je 0x1417a3471
1417a4155: 48f7de  neg rsi
1417a4158: 498b3f  mov rdi, qword ptr [r15]
1417a415b: 498b5f08  mov rbx, qword ptr [r15 + 8]
1417a415f: 4989dc  mov r12, rbx
1417a4162: 4901f4  add r12, rsi
1417a4165: 49c7c0ffffffff  mov r8, 0xffffffffffffffff
1417a416c: 4d0f43c4  cmovae r8, r12
1417a4170: 488b07  mov rax, qword ptr [rdi]
1417a4173: 4939c0  cmp r8, rax
1417a4176: 0f8671010000  jbe 0x1417a42ed
1417a417c: 488b5710  mov rdx, qword ptr [rdi + 0x10]
1417a4180: 4929d0  sub r8, rdx
1417a4183: 4829d0  sub rax, rdx
1417a4186: 4c39c0  cmp rax, r8
1417a4189: 0f835e010000  jae 0x1417a42ed
1417a418f: 48c744242001000000  mov qword ptr [rsp + 0x20], 1
1417a4198: 41b901000000  mov r9d, 1
1417a419e: 4889f9  mov rcx, rdi
1417a41a1: e8ba581001  call 0x1428a9a60
1417a41a6: e942010000  jmp 0x1417a42ed
1417a41ab: 488d95d0020000  lea rdx, [rbp + 0x2d0]
1417a41b2: e8f9893eff  call 0x140b8cbb0
1417a41b7: 4889c6  mov rsi, rax
1417a41ba: 4885c0  test rax, rax
1417a41bd: 0f84aef2ffff  je 0x1417a3471
1417a41c3: 4c8bb590020000  mov r14, qword ptr [rbp + 0x290]
1417a41ca: 498b3e  mov rdi, qword ptr [r14]
1417a41cd: 498b5e08  mov rbx, qword ptr [r14 + 8]
1417a41d1: 4989df  mov r15, rbx
1417a41d4: 4901f7  add r15, rsi
1417a41d7: 49c7c0ffffffff  mov r8, 0xffffffffffffffff
1417a41de: 4d0f43c7  cmovae r8, r15
1417a41e2: 488b07  mov rax, qword ptr [rdi]
1417a41e5: 4939c0  cmp r8, rax
1417a41e8: 7613  jbe 0x1417a41fd
1417a41ea: 488b5710  mov rdx, qword ptr [rdi + 0x10]
1417a41ee: 4929d0  sub r8, rdx
1417a41f1: 4829d0  sub rax, rdx
1417a41f4: 4c39c0  cmp rax, r8
1417a41f7: 0f82c60b0000  jb 0x1417a4dc3
1417a41fd: 488b4f10  mov rcx, qword ptr [rdi + 0x10]
1417a4201: 4989d8  mov r8, rbx
1417a4204: 4929c8  sub r8, rcx
1417a4207: 760f  jbe 0x1417a4218
1417a4209: 48034f08  add rcx, qword ptr [rdi + 8]
1417a420d: 31d2  xor edx, edx
1417a420f: e80cfa1701  call 0x142923c20
1417a4214: 48895f10  mov qword ptr [rdi + 0x10], rbx
1417a4218: 48035f08  add rbx, qword ptr [rdi + 8]
1417a421c: 488d95d0020000  lea rdx, [rbp + 0x2d0]
1417a4223: 4889d9  mov rcx, rbx
1417a4226: 4989f0  mov r8, rsi
1417a4229: e8d2f91701  call 0x142923c00
1417a422e: 4c3b7f10  cmp r15, qword ptr [rdi + 0x10]
1417a4232: 7604  jbe 0x1417a4238
1417a4234: 4c897f10  mov qword ptr [rdi + 0x10], r15
1417a4238: 4d897e08  mov qword ptr [r14 + 8], r15
1417a423c: e930f2ffff  jmp 0x1417a3471
1417a4241: 4889ca  mov rdx, rcx
1417a4244: 4883fa63  cmp rdx, 0x63
1417a4248: 7632  jbe 0x1417a427c
1417a424a: 0fb7c2  movzx eax, dx
1417a424d: c1e802  shr eax, 2
1417a4250: 69c07b140000  imul eax, eax, 0x147b
1417a4256: c1e811  shr eax, 0x11
1417a4259: 6bc864  imul ecx, eax, 0x64
1417a425c: 29ca  sub edx, ecx
1417a425e: 0fb7ca  movzx ecx, dx
1417a4261: 488d15a37f5301  lea rdx, [rip + 0x1537fa3]
1417a4268: 0fb70c4a  movzx ecx, word ptr [rdx + rcx*2]
1417a426c: 6642898c35ce020000  mov word ptr [rbp + r14 + 0x2ce], cx
1417a4275: 4983c6fe  add r14, -2
1417a4279: 4889c2  mov rdx, rax
1417a427c: 4883fa0a  cmp rdx, 0xa
1417a4280: 7310  jae 0x1417a4292
1417a4282: 80ca30  or dl, 0x30
1417a4285: 42889435cf020000  mov byte ptr [rbp + r14 + 0x2cf], dl
1417a428d: 49ffce  dec r14
1417a4290: eb18  jmp 0x1417a42aa
1417a4292: 488d05727f5301  lea rax, [rip + 0x1537f72]
1417a4299: 0fb70450  movzx eax, word ptr [rax + rdx*2]
1417a429d: 6642898435ce020000  mov word ptr [rbp + r14 + 0x2ce], ax
1417a42a6: 4983c6fe  add r14, -2
1417a42aa: 4c89f6  mov rsi, r14
1417a42ad: 4883ee14  sub rsi, 0x14
1417a42b1: 0f84baf1ffff  je 0x1417a3471
1417a42b7: 48f7de  neg rsi
1417a42ba: 498b3f  mov rdi, qword ptr [r15]
1417a42bd: 498b5f08  mov rbx, qword ptr [r15 + 8]
1417a42c1: 4989dc  mov r12, rbx
1417a42c4: 4901f4  add r12, rsi
1417a42c7: 49c7c0ffffffff  mov r8, 0xffffffffffffffff
1417a42ce: 4d0f43c4  cmovae r8, r12
1417a42d2: 488b07  mov rax, qword ptr [rdi]
1417a42d5: 4939c0  cmp r8, rax
1417a42d8: 7613  jbe 0x1417a42ed
1417a42da: 488b5710  mov rdx, qword ptr [rdi + 0x10]
1417a42de: 4929d0  sub r8, rdx
1417a42e1: 4829d0  sub rax, rdx
1417a42e4: 4c39c0  cmp rax, r8
1417a42e7: 0f82ba0a0000  jb 0x1417a4da7
1417a42ed: 4901ee  add r14, rbp
1417a42f0: 4981c6d0020000  add r14, 0x2d0
1417a42f7: 488b4f10  mov rcx, qword ptr [rdi + 0x10]
1417a42fb: 4989d8  mov r8, rbx
1417a42fe: 4929c8  sub r8, rcx
1417a4301: 760f  jbe 0x1417a4312
1417a4303: 48034f08  add rcx, qword ptr [rdi + 8]
1417a4307: 31d2  xor edx, edx
1417a4309: e812f91701  call 0x142923c20
1417a430e: 48895f10  mov qword ptr [rdi + 0x10], rbx
1417a4312: 48035f08  add rbx, qword ptr [rdi + 8]
1417a4316: 4889d9  mov rcx, rbx
1417a4319: 4c89f2  mov rdx, r14
1417a431c: 4989f0  mov r8, rsi
1417a431f: e8dcf81701  call 0x142923c00
1417a4324: 4c3b6710  cmp r12, qword ptr [rdi + 0x10]
1417a4328: 0f873bf1ffff  ja 0x1417a3469
1417a432e: e93af1ffff  jmp 0x1417a346d
1417a4333: 31c0  xor eax, eax
1417a4335: 4889bd50020000  mov qword ptr [rbp + 0x250], rdi
1417a433c: 888558020000  mov byte ptr [rbp + 0x258], al
1417a4342: 488b8508020000  mov rax, qword ptr [rbp + 0x208]
1417a4349: 488b8d10020000  mov rcx, qword ptr [rbp + 0x210]
1417a4350: 31d2  xor edx, edx
1417a4352: 4885c0  test rax, rax
1417a4355: 480f44d8  cmove rbx, rax
1417a4359: 0f95c2  setne dl
1417a435c: 488995d0020000  mov qword ptr [rbp + 0x2d0], rdx
1417a4363: 48c785d802000000000000  mov qword ptr [rbp + 0x2d8], 0
1417a436e: 488985e0020000  mov qword ptr [rbp + 0x2e0], rax
1417a4375: 48898de8020000  mov qword ptr [rbp + 0x2e8], rcx
1417a437c: 488995f0020000  mov qword ptr [rbp + 0x2f0], rdx
1417a4383: 48c785f802000000000000  mov qword ptr [rbp + 0x2f8], 0
1417a438e: 48898500030000  mov qword ptr [rbp + 0x300], rax
1417a4395: 48898d08030000  mov qword ptr [rbp + 0x308], rcx
1417a439c: 48899d10030000  mov qword ptr [rbp + 0x310], rbx
1417a43a3: 488db5d0020000  lea rsi, [rbp + 0x2d0]
1417a43aa: 488d9d50020000  lea rbx, [rbp + 0x250]
1417a43b1: 6666666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a43c0: 4889f1  mov rcx, rsi
1417a43c3: e80854c9ff  call 0x1414397d0
1417a43c8: 4885c0  test rax, rax
1417a43cb: 7418  je 0x1417a43e5
1417a43cd: 4989d0  mov r8, rdx
1417a43d0: 4889d9  mov rcx, rbx
1417a43d3: 4889c2  mov rdx, rax
1417a43d6: e8e56ddeff  call 0x14158b1c0
1417a43db: 4889c7  mov rdi, rax
1417a43de: 4885c0  test rax, rax
1417a43e1: 74dd  je 0x1417a43c0
1417a43e3: eb3f  jmp 0x1417a4424
1417a43e5: 80bd5802000000  cmp byte ptr [rbp + 0x258], 0
1417a43ec: 742b  je 0x1417a4419
1417a43ee: 488b8d50020000  mov rcx, qword ptr [rbp + 0x250]
1417a43f5: 488d15947c5301  lea rdx, [rip + 0x1537c94]
1417a43fc: 41b801000000  mov r8d, 1
1417a4402: e8e9b8daff  call 0x14154fcf0
1417a4407: 4885c0  test rax, rax
1417a440a: 740d  je 0x1417a4419
1417a440c: 4889c1  mov rcx, rax
1417a440f: e88c541201  call 0x1428c98a0
1417a4414: e9c8eeffff  jmp 0x1417a32e1
1417a4419: 31ff  xor edi, edi
1417a441b: 4885ff  test rdi, rdi
1417a441e: 0f844df0ffff  je 0x1417a3471
1417a4424: 488b9d90020000  mov rbx, qword ptr [rbp + 0x290]
1417a442b: 4c8bbda0020000  mov r15, qword ptr [rbp + 0x2a0]
1417a4432: 488b33  mov rsi, qword ptr [rbx]
1417a4435: 4c8b7308  mov r14, qword ptr [rbx + 8]
1417a4439: 4d8d6708  lea r12, [r15 + 8]
1417a443d: 4c89e0  mov rax, r12
1417a4440: 4883c008  add rax, 8
1417a4444: 49c7c0ffffffff  mov r8, 0xffffffffffffffff
1417a444b: 4c0f43c0  cmovae r8, rax
1417a444f: 4c896308  mov qword ptr [rbx + 8], r12
1417a4453: 488b06  mov rax, qword ptr [rsi]
1417a4456: 488b5610  mov rdx, qword ptr [rsi + 0x10]
1417a445a: 4939c0  cmp r8, rax
1417a445d: 760f  jbe 0x1417a446e
1417a445f: 4929d0  sub r8, rdx
1417a4462: 4829d0  sub rax, rdx
1417a4465: 4c39c0  cmp rax, r8
1417a4468: 0f8269080000  jb 0x1417a4cd7
1417a446e: 4d89f5  mov r13, r14
1417a4471: 4d29fd  sub r13, r15
1417a4474: 4983c5f0  add r13, -0x10
1417a4478: 4d89e0  mov r8, r12
1417a447b: 4929d0  sub r8, rdx
1417a447e: 7612  jbe 0x1417a4492
1417a4480: 48035608  add rdx, qword ptr [rsi + 8]
1417a4484: 4889d1  mov rcx, rdx
1417a4487: 31d2  xor edx, edx
1417a4489: e892f71701  call 0x142923c20
1417a448e: 4c896610  mov qword ptr [rsi + 0x10], r12
1417a4492: 488b4608  mov rax, qword ptr [rsi + 8]
1417a4496: 4e896c3808  mov qword ptr [rax + r15 + 8], r13
1417a449b: 4983c710  add r15, 0x10
1417a449f: 4c3b7e10  cmp r15, qword ptr [rsi + 0x10]
1417a44a3: 7604  jbe 0x1417a44a9
1417a44a5: 4c897e10  mov qword ptr [rsi + 0x10], r15
1417a44a9: 4c897308  mov qword ptr [rbx + 8], r14
1417a44ad: 488b8598020000  mov rax, qword ptr [rbp + 0x298]
1417a44b4: 48898530030000  mov qword ptr [rbp + 0x330], rax
1417a44bb: 48833800  cmp qword ptr [rax], 0
1417a44bf: 740c  je 0x1417a44cd
1417a44c1: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a44c8: e83335e4fe  call 0x1405e7a00
1417a44cd: 488b8530030000  mov rax, qword ptr [rbp + 0x330]
1417a44d4: 48c70000000000  mov qword ptr [rax], 0
1417a44db: c6853f03000001  mov byte ptr [rbp + 0x33f], 1
1417a44e2: c6853e03000001  mov byte ptr [rbp + 0x33e], 1
1417a44e9: 4889f9  mov rcx, rdi
1417a44ec: e85f531001  call 0x1428a9850
1417a44f1: 4c8bb548020000  mov r14, qword ptr [rbp + 0x248]
1417a44f8: 49894608  mov qword ptr [r14 + 8], rax
1417a44fc: 48b80000000000000080  movabs rax, 0x8000000000000000
1417a4506: 498906  mov qword ptr [r14], rax
1417a4509: 41b701  mov r15b, 1
1417a450c: 40b601  mov sil, 1
1417a450f: 0fb68500020000  movzx eax, byte ptr [rbp + 0x200]
1417a4516: 83f803  cmp eax, 3
1417a4519: 0f829b000000  jb 0x1417a45ba
1417a451f: 7458  je 0x1417a4579
1417a4521: 83f804  cmp eax, 4
1417a4524: 7573  jne 0x1417a4599
1417a4526: 89b588020000  mov dword ptr [rbp + 0x288], esi
1417a452c: 4489bd30030000  mov dword ptr [rbp + 0x330], r15d
1417a4533: 488b8d10020000  mov rcx, qword ptr [rbp + 0x210]
1417a453a: 488b9518020000  mov rdx, qword ptr [rbp + 0x218]
1417a4541: 48898d40020000  mov qword ptr [rbp + 0x240], rcx
1417a4548: e823480eff  call 0x140888d70
1417a454d: 488b9508020000  mov rdx, qword ptr [rbp + 0x208]
1417a4554: 4885d2  test rdx, rdx
1417a4557: 448bbd30030000  mov r15d, dword ptr [rbp + 0x330]
1417a455e: 8bb588020000  mov esi, dword ptr [rbp + 0x288]
1417a4564: 7454  je 0x1417a45ba
1417a4566: 48c1e205  shl rdx, 5
1417a456a: 41b808000000  mov r8d, 8
1417a4570: 488b8d40020000  mov rcx, qword ptr [rbp + 0x240]
1417a4577: eb19  jmp 0x1417a4592
1417a4579: 488b9508020000  mov rdx, qword ptr [rbp + 0x208]
1417a4580: 4885d2  test rdx, rdx
1417a4583: 7435  je 0x1417a45ba
1417a4585: 488b8d10020000  mov rcx, qword ptr [rbp + 0x210]
1417a458c: 41b801000000  mov r8d, 1
1417a4592: e889f6e6fe  call 0x140613c20
1417a4597: eb21  jmp 0x1417a45ba
1417a4599: 4488bd3d030000  mov byte ptr [rbp + 0x33d], r15b
1417a45a0: 4088b53c030000  mov byte ptr [rbp + 0x33c], sil
1417a45a7: c6853b03000001  mov byte ptr [rbp + 0x33b], 1
1417a45ae: 488d8d08020000  lea rcx, [rbp + 0x208]
1417a45b5: e886490fff  call 0x140898f40
1417a45ba: 4084f6  test sil, sil
1417a45bd: 0f8488000000  je 0x1417a464b
1417a45c3: 488b8580000000  mov rax, qword ptr [rbp + 0x80]
1417a45ca: 89c1  mov ecx, eax
1417a45cc: 83e103  and ecx, 3
1417a45cf: 83f901  cmp ecx, 1
1417a45d2: 7577  jne 0x1417a464b
1417a45d4: 4489bd30030000  mov dword ptr [rbp + 0x330], r15d
1417a45db: 488d48ff  lea rcx, [rax - 1]
1417a45df: 48898d20020000  mov qword ptr [rbp + 0x220], rcx
1417a45e6: 488b48ff  mov rcx, qword ptr [rax - 1]
1417a45ea: 48898d88020000  mov qword ptr [rbp + 0x288], rcx
1417a45f1: 488b4007  mov rax, qword ptr [rax + 7]
1417a45f5: 48898540020000  mov qword ptr [rbp + 0x240], rax
1417a45fc: 488b00  mov rax, qword ptr [rax]
1417a45ff: 4885c0  test rax, rax
1417a4602: 7409  je 0x1417a460d
1417a4604: 488b8d88020000  mov rcx, qword ptr [rbp + 0x288]
1417a460b: ffd0  call rax
1417a460d: 488b8d88020000  mov rcx, qword ptr [rbp + 0x288]
1417a4614: 488b8540020000  mov rax, qword ptr [rbp + 0x240]
1417a461b: 488b5008  mov rdx, qword ptr [rax + 8]
1417a461f: 4885d2  test rdx, rdx
1417a4622: 7409  je 0x1417a462d
1417a4624: 4c8b4010  mov r8, qword ptr [rax + 0x10]
1417a4628: e8f3f5e6fe  call 0x140613c20
1417a462d: ba18000000  mov edx, 0x18
1417a4632: 41b808000000  mov r8d, 8
1417a4638: 488b8d20020000  mov rcx, qword ptr [rbp + 0x220]
1417a463f: e8dcf5e6fe  call 0x140613c20
1417a4644: 448bbd30030000  mov r15d, dword ptr [rbp + 0x330]
1417a464b: 488b95f8000000  mov rdx, qword ptr [rbp + 0xf8]
1417a4652: 4885d2  test rdx, rdx
1417a4655: 7412  je 0x1417a4669
1417a4657: 488b8d00010000  mov rcx, qword ptr [rbp + 0x100]
1417a465e: 41b801000000  mov r8d, 1
1417a4664: e8b7f5e6fe  call 0x140613c20
1417a4669: 488b55b8  mov rdx, qword ptr [rbp - 0x48]
1417a466d: 4885d2  test rdx, rdx
1417a4670: 7427  je 0x1417a4699
1417a4672: 488d04d517000000  lea rax, [rdx*8 + 0x17]
1417a467a: 4883e0f0  and rax, 0xfffffffffffffff0
1417a467e: 4801c2  add rdx, rax
1417a4681: 4883c211  add rdx, 0x11
1417a4685: 7412  je 0x1417a4699
1417a4687: 488b4db0  mov rcx, qword ptr [rbp - 0x50]
1417a468b: 4829c1  sub rcx, rax
1417a468e: 41b810000000  mov r8d, 0x10
1417a4694: e887f5e6fe  call 0x140613c20
1417a4699: 488b5550  mov rdx, qword ptr [rbp + 0x50]
1417a469d: 4885d2  test rdx, rdx
1417a46a0: 7413  je 0x1417a46b5
1417a46a2: 488b4d58  mov rcx, qword ptr [rbp + 0x58]
1417a46a6: 48c1e203  shl rdx, 3
1417a46aa: 41b804000000  mov r8d, 4
1417a46b0: e86bf5e6fe  call 0x140613c20
1417a46b5: 488bb518010000  mov rsi, qword ptr [rbp + 0x118]
1417a46bc: 488bbd20010000  mov rdi, qword ptr [rbp + 0x120]
1417a46c3: 4885ff  test rdi, rdi
1417a46c6: 742a  je 0x1417a46f2
1417a46c8: 488d5e10  lea rbx, [rsi + 0x10]
1417a46cc: eb0b  jmp 0x1417a46d9
1417a46ce: 6690  nop
1417a46d0: 4883c340  add rbx, 0x40
1417a46d4: 48ffcf  dec rdi
1417a46d7: 7419  je 0x1417a46f2
1417a46d9: 488b53f8  mov rdx, qword ptr [rbx - 8]
1417a46dd: 4885d2  test rdx, rdx
1417a46e0: 74ee  je 0x1417a46d0
1417a46e2: 488b0b  mov rcx, qword ptr [rbx]
1417a46e5: 41b801000000  mov r8d, 1
1417a46eb: e830f5e6fe  call 0x140613c20
1417a46f0: ebde  jmp 0x1417a46d0
1417a46f2: 488b9510010000  mov rdx, qword ptr [rbp + 0x110]
1417a46f9: 4885d2  test rdx, rdx
1417a46fc: 7412  je 0x1417a4710
1417a46fe: 48c1e206  shl rdx, 6
1417a4702: 41b808000000  mov r8d, 8
1417a4708: 4889f1  mov rcx, rsi
1417a470b: e810f5e6fe  call 0x140613c20
1417a4710: 488b9570010000  mov rdx, qword ptr [rbp + 0x170]
1417a4717: 4885d2  test rdx, rdx
1417a471a: 7416  je 0x1417a4732
1417a471c: 488b8d78010000  mov rcx, qword ptr [rbp + 0x178]
1417a4723: 48c1e203  shl rdx, 3
1417a4727: 41b808000000  mov r8d, 8
1417a472d: e8eef4e6fe  call 0x140613c20
1417a4732: 488b95d0010000  mov rdx, qword ptr [rbp + 0x1d0]
1417a4739: 4885d2  test rdx, rdx
1417a473c: 7412  je 0x1417a4750
1417a473e: 41b801000000  mov r8d, 1
1417a4744: 488b8df8010000  mov rcx, qword ptr [rbp + 0x1f8]
1417a474b: e8d0f4e6fe  call 0x140613c20
1417a4750: 488d4de8  lea rcx, [rbp - 0x18]
1417a4754: 488d5508  lea rdx, [rbp + 8]
1417a4758: 4488bd1d030000  mov byte ptr [rbp + 0x31d], r15b
1417a475f: 41b830000000  mov r8d, 0x30
1417a4765: 41b910000000  mov r9d, 0x10
1417a476b: e800ae8afe  call 0x14004f570
1417a4770: 488b95a0000000  mov rdx, qword ptr [rbp + 0xa0]
1417a4777: 4885d2  test rdx, rdx
1417a477a: 7416  je 0x1417a4792
1417a477c: 488b8da8000000  mov rcx, qword ptr [rbp + 0xa8]
1417a4783: 48c1e204  shl rdx, 4
1417a4787: 41b808000000  mov r8d, 8
1417a478d: e88ef4e6fe  call 0x140613c20
1417a4792: 488b95c0000000  mov rdx, qword ptr [rbp + 0xc0]
1417a4799: 4885d2  test rdx, rdx
1417a479c: 7416  je 0x1417a47b4
1417a479e: 488b8dc8000000  mov rcx, qword ptr [rbp + 0xc8]
1417a47a5: 48c1e204  shl rdx, 4
1417a47a9: 41b808000000  mov r8d, 8
1417a47af: e86cf4e6fe  call 0x140613c20
1417a47b4: 488b95e0000000  mov rdx, qword ptr [rbp + 0xe0]
1417a47bb: 4885d2  test rdx, rdx
1417a47be: 7416  je 0x1417a47d6
1417a47c0: 488b8de8000000  mov rcx, qword ptr [rbp + 0xe8]
1417a47c7: 48c1e204  shl rdx, 4
1417a47cb: 41b808000000  mov r8d, 8
1417a47d1: e84af4e6fe  call 0x140613c20
1417a47d6: 4584ff  test r15b, r15b
1417a47d9: 7469  je 0x1417a4844
1417a47db: 488bb598010000  mov rsi, qword ptr [rbp + 0x198]
1417a47e2: 488bbda0010000  mov rdi, qword ptr [rbp + 0x1a0]
1417a47e9: 4885ff  test rdi, rdi
1417a47ec: 7434  je 0x1417a4822
1417a47ee: 488d5e18  lea rbx, [rsi + 0x18]
1417a47f2: eb15  jmp 0x1417a4809
1417a47f4: 6666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a4800: 4883c328  add rbx, 0x28
1417a4804: 48ffcf  dec rdi
1417a4807: 7419  je 0x1417a4822
1417a4809: 488b53f8  mov rdx, qword ptr [rbx - 8]
1417a480d: 4885d2  test rdx, rdx
1417a4810: 74ee  je 0x1417a4800
1417a4812: 488b0b  mov rcx, qword ptr [rbx]
1417a4815: 41b801000000  mov r8d, 1
1417a481b: e800f4e6fe  call 0x140613c20
1417a4820: ebde  jmp 0x1417a4800
1417a4822: 488b8590010000  mov rax, qword ptr [rbp + 0x190]
1417a4829: 4885c0  test rax, rax
1417a482c: 7416  je 0x1417a4844
1417a482e: 48c1e003  shl rax, 3
1417a4832: 488d1480  lea rdx, [rax + rax*4]
1417a4836: 41b808000000  mov r8d, 8
1417a483c: 4889f1  mov rcx, rsi
1417a483f: e8dcf3e6fe  call 0x140613c20
1417a4844: 488bb538010000  mov rsi, qword ptr [rbp + 0x138]
1417a484b: 488bbd40010000  mov rdi, qword ptr [rbp + 0x140]
1417a4852: 4885ff  test rdi, rdi
1417a4855: 742f  je 0x1417a4886
1417a4857: 488d5e18  lea rbx, [rsi + 0x18]
1417a485b: eb0c  jmp 0x1417a4869
1417a485d: 0f1f00  nop dword ptr [rax]
1417a4860: 4883c328  add rbx, 0x28
1417a4864: 48ffcf  dec rdi
1417a4867: 741d  je 0x1417a4886
1417a4869: 488b43f8  mov rax, qword ptr [rbx - 8]
1417a486d: 4885c0  test rax, rax
1417a4870: 74ee  je 0x1417a4860
1417a4872: 488b0b  mov rcx, qword ptr [rbx]
1417a4875: 486bd058  imul rdx, rax, 0x58
1417a4879: 41b808000000  mov r8d, 8
1417a487f: e89cf3e6fe  call 0x140613c20
1417a4884: ebda  jmp 0x1417a4860
1417a4886: 488b8530010000  mov rax, qword ptr [rbp + 0x130]
1417a488d: 4885c0  test rax, rax
1417a4890: 7416  je 0x1417a48a8
1417a4892: 48c1e003  shl rax, 3
1417a4896: 488d1480  lea rdx, [rax + rax*4]
1417a489a: 41b808000000  mov r8d, 8
1417a48a0: 4889f1  mov rcx, rsi
1417a48a3: e878f3e6fe  call 0x140613c20
1417a48a8: 488b85b0010000  mov rax, qword ptr [rbp + 0x1b0]
1417a48af: 4885c0  test rax, rax
1417a48b2: 7416  je 0x1417a48ca
1417a48b4: 488b8db8010000  mov rcx, qword ptr [rbp + 0x1b8]
1417a48bb: 486bd058  imul rdx, rax, 0x58
1417a48bf: 41b808000000  mov r8d, 8
1417a48c5: e856f3e6fe  call 0x140613c20
1417a48ca: 488bb558010000  mov rsi, qword ptr [rbp + 0x158]
1417a48d1: 488bbd60010000  mov rdi, qword ptr [rbp + 0x160]
1417a48d8: 4885ff  test rdi, rdi
1417a48db: 7455  je 0x1417a4932
1417a48dd: 488d5e28  lea rbx, [rsi + 0x28]
1417a48e1: eb16  jmp 0x1417a48f9
1417a48e3: 666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a48f0: 4883c378  add rbx, 0x78
1417a48f4: 48ffcf  dec rdi
1417a48f7: 7439  je 0x1417a4932
1417a48f9: 488b53e0  mov rdx, qword ptr [rbx - 0x20]
1417a48fd: 4885d2  test rdx, rdx
1417a4900: 7413  je 0x1417a4915
1417a4902: 488b4be8  mov rcx, qword ptr [rbx - 0x18]
1417a4906: 48c1e203  shl rdx, 3
1417a490a: 41b804000000  mov r8d, 4
1417a4910: e80bf3e6fe  call 0x140613c20
1417a4915: 488b53f8  mov rdx, qword ptr [rbx - 8]
1417a4919: 4885d2  test rdx, rdx
1417a491c: 74d2  je 0x1417a48f0
1417a491e: 488b0b  mov rcx, qword ptr [rbx]
1417a4921: 48c1e202  shl rdx, 2
1417a4925: 41b804000000  mov r8d, 4
1417a492b: e8f0f2e6fe  call 0x140613c20
1417a4930: ebbe  jmp 0x1417a48f0
1417a4932: 488b8550010000  mov rax, qword ptr [rbp + 0x150]
1417a4939: 4885c0  test rax, rax
1417a493c: 7412  je 0x1417a4950
1417a493e: 486bd078  imul rdx, rax, 0x78
1417a4942: 41b808000000  mov r8d, 8
1417a4948: 4889f1  mov rcx, rsi
1417a494b: e8d0f2e6fe  call 0x140613c20
1417a4950: ba98010000  mov edx, 0x198
1417a4955: 41b808000000  mov r8d, 8
1417a495b: 488b8d88000000  mov rcx, qword ptr [rbp + 0x88]
1417a4962: e8b9f2e6fe  call 0x140613c20
1417a4967: 4c89f0  mov rax, r14
1417a496a: 4881c4c8030000  add rsp, 0x3c8
1417a4971: 5b  pop rbx
1417a4972: 5f  pop rdi
1417a4973: 5e  pop rsi
1417a4974: 415c  pop r12
1417a4976: 415d  pop r13
1417a4978: 415e  pop r14
1417a497a: 415f  pop r15
1417a497c: 5d  pop rbp
1417a497d: c3  ret
1417a497e: c6853f03000000  mov byte ptr [rbp + 0x33f], 0
1417a4985: c6853e03000001  mov byte ptr [rbp + 0x33e], 1
1417a498c: 488d8dd8010000  lea rcx, [rbp + 0x1d8]
1417a4993: e858620dff  call 0x14087abf0
1417a4998: 488b8d80000000  mov rcx, qword ptr [rbp + 0x80]
1417a499f: 4885c9  test rcx, rcx
1417a49a2: 7445  je 0x1417a49e9
1417a49a4: c6853f03000000  mov byte ptr [rbp + 0x33f], 0
1417a49ab: c6853e03000000  mov byte ptr [rbp + 0x33e], 0
1417a49b2: e8d94b1001  call 0x1428a9590
1417a49b7: 4c8bb548020000  mov r14, qword ptr [rbp + 0x248]
1417a49be: 49894608  mov qword ptr [r14 + 8], rax
1417a49c2: 48b80000000000000080  movabs rax, 0x8000000000000000
1417a49cc: 498906  mov qword ptr [r14], rax
1417a49cf: 4531ff  xor r15d, r15d
1417a49d2: 31f6  xor esi, esi
1417a49d4: 0fb68500020000  movzx eax, byte ptr [rbp + 0x200]
1417a49db: 83f803  cmp eax, 3
1417a49de: 0f833bfbffff  jae 0x1417a451f
1417a49e4: e9d1fbffff  jmp 0x1417a45ba
1417a49e9: 488b8508010000  mov rax, qword ptr [rbp + 0x108]
1417a49f0: 4c8bb548020000  mov r14, qword ptr [rbp + 0x248]
1417a49f7: 49894610  mov qword ptr [r14 + 0x10], rax
1417a49fb: f30f6f85f8000000  movdqu xmm0, xmmword ptr [rbp + 0xf8]
1417a4a03: f3410f7f06  movdqu xmmword ptr [r14], xmm0
1417a4a08: c6853d03000000  mov byte ptr [rbp + 0x33d], 0
1417a4a0f: c6853c03000000  mov byte ptr [rbp + 0x33c], 0
1417a4a16: c6853b03000000  mov byte ptr [rbp + 0x33b], 0
1417a4a1d: 488d8d00020000  lea rcx, [rbp + 0x200]
1417a4a24: e867900bff  call 0x14085da90
1417a4a29: c685ce02000000  mov byte ptr [rbp + 0x2ce], 0
1417a4a30: 488d4db0  lea rcx, [rbp - 0x50]
1417a4a34: e817559afe  call 0x140149f50
1417a4a39: 488b5550  mov rdx, qword ptr [rbp + 0x50]
1417a4a3d: 4885d2  test rdx, rdx
1417a4a40: 7413  je 0x1417a4a55
1417a4a42: 488b4d58  mov rcx, qword ptr [rbp + 0x58]
1417a4a46: 48c1e203  shl rdx, 3
1417a4a4a: 41b804000000  mov r8d, 4
1417a4a50: e8cbf1e6fe  call 0x140613c20
1417a4a55: c685cd02000000  mov byte ptr [rbp + 0x2cd], 0
1417a4a5c: 488d8d10010000  lea rcx, [rbp + 0x110]
1417a4a63: e8b8a80dff  call 0x14087f320
1417a4a68: 488b9570010000  mov rdx, qword ptr [rbp + 0x170]
1417a4a6f: 4885d2  test rdx, rdx
1417a4a72: 7416  je 0x1417a4a8a
1417a4a74: 488b8d78010000  mov rcx, qword ptr [rbp + 0x178]
1417a4a7b: 48c1e203  shl rdx, 3
1417a4a7f: 41b808000000  mov r8d, 8
1417a4a85: e896f1e6fe  call 0x140613c20
1417a4a8a: 488b95d0010000  mov rdx, qword ptr [rbp + 0x1d0]
1417a4a91: 4885d2  test rdx, rdx
1417a4a94: 7412  je 0x1417a4aa8
1417a4a96: 41b801000000  mov r8d, 1
1417a4a9c: 488b8df8010000  mov rcx, qword ptr [rbp + 0x1f8]
1417a4aa3: e878f1e6fe  call 0x140613c20
1417a4aa8: 488d4de8  lea rcx, [rbp - 0x18]
1417a4aac: 488d5508  lea rdx, [rbp + 8]
1417a4ab0: c6851d03000000  mov byte ptr [rbp + 0x31d], 0
1417a4ab7: 41b830000000  mov r8d, 0x30
1417a4abd: 41b910000000  mov r9d, 0x10
1417a4ac3: e8a8aa8afe  call 0x14004f570
1417a4ac8: 488b95a0000000  mov rdx, qword ptr [rbp + 0xa0]
1417a4acf: 4885d2  test rdx, rdx
1417a4ad2: 7416  je 0x1417a4aea
1417a4ad4: 488b8da8000000  mov rcx, qword ptr [rbp + 0xa8]
1417a4adb: 48c1e204  shl rdx, 4
1417a4adf: 41b808000000  mov r8d, 8
1417a4ae5: e836f1e6fe  call 0x140613c20
1417a4aea: 488b95c0000000  mov rdx, qword ptr [rbp + 0xc0]
1417a4af1: 4885d2  test rdx, rdx
1417a4af4: 7416  je 0x1417a4b0c
1417a4af6: 488b8dc8000000  mov rcx, qword ptr [rbp + 0xc8]
1417a4afd: 48c1e204  shl rdx, 4
1417a4b01: 41b808000000  mov r8d, 8
1417a4b07: e814f1e6fe  call 0x140613c20
1417a4b0c: 488b95e0000000  mov rdx, qword ptr [rbp + 0xe0]
1417a4b13: 4885d2  test rdx, rdx
1417a4b16: 7416  je 0x1417a4b2e
1417a4b18: 488b8de8000000  mov rcx, qword ptr [rbp + 0xe8]
1417a4b1f: 48c1e204  shl rdx, 4
1417a4b23: 41b808000000  mov r8d, 8
1417a4b29: e8f2f0e6fe  call 0x140613c20
1417a4b2e: 488d8d30010000  lea rcx, [rbp + 0x130]
1417a4b35: e866da0dff  call 0x1408825a0
1417a4b3a: 488b85b0010000  mov rax, qword ptr [rbp + 0x1b0]
1417a4b41: 4885c0  test rax, rax
1417a4b44: 7416  je 0x1417a4b5c
1417a4b46: 488b8db8010000  mov rcx, qword ptr [rbp + 0x1b8]
1417a4b4d: 486bd058  imul rdx, rax, 0x58
1417a4b51: 41b808000000  mov r8d, 8
1417a4b57: e8c4f0e6fe  call 0x140613c20
1417a4b5c: 488d8d50010000  lea rcx, [rbp + 0x150]
1417a4b63: e8e8ab0dff  call 0x14087f750
1417a4b68: e9e3fdffff  jmp 0x1417a4950
1417a4b6d: 4883fa15  cmp rdx, 0x15
1417a4b71: 0f8376070000  jae 0x1417a52ed
1417a4b77: c6853d03000001  mov byte ptr [rbp + 0x33d], 1
1417a4b7e: c6853c03000001  mov byte ptr [rbp + 0x33c], 1
1417a4b85: c6853b03000001  mov byte ptr [rbp + 0x33b], 1
1417a4b8c: 4c8d8dd0020000  lea r9, [rbp + 0x2d0]
1417a4b93: 41b801000000  mov r8d, 1
1417a4b99: e89245b9ff  call 0x141339130
1417a4b9e: e98bd3ffff  jmp 0x1417a1f2e
1417a4ba3: 4883fa15  cmp rdx, 0x15
1417a4ba7: 0f8366070000  jae 0x1417a5313
1417a4bad: c6853d03000001  mov byte ptr [rbp + 0x33d], 1
1417a4bb4: c6853c03000001  mov byte ptr [rbp + 0x33c], 1
1417a4bbb: c6853b03000001  mov byte ptr [rbp + 0x33b], 1
1417a4bc2: 4c8d8dd0020000  lea r9, [rbp + 0x2d0]
1417a4bc9: 41b801000000  mov r8d, 1
1417a4bcf: e8ac41b9ff  call 0x141338d80
1417a4bd4: e974d3ffff  jmp 0x1417a1f4d
1417a4bd9: 4883fa15  cmp rdx, 0x15
1417a4bdd: 0f8356070000  jae 0x1417a5339
1417a4be3: c6853d03000001  mov byte ptr [rbp + 0x33d], 1
1417a4bea: c6853c03000001  mov byte ptr [rbp + 0x33c], 1
1417a4bf1: c6853b03000001  mov byte ptr [rbp + 0x33b], 1
1417a4bf8: 4c8d8dd0020000  lea r9, [rbp + 0x2d0]
1417a4bff: 41b801000000  mov r8d, 1
1417a4c05: e89646b9ff  call 0x1413392a0
1417a4c0a: e95dd3ffff  jmp 0x1417a1f6c
1417a4c0f: 4883fa15  cmp rdx, 0x15
1417a4c13: 0f8346070000  jae 0x1417a535f
1417a4c19: c6853d03000001  mov byte ptr [rbp + 0x33d], 1
1417a4c20: c6853c03000001  mov byte ptr [rbp + 0x33c], 1
1417a4c27: c6853b03000001  mov byte ptr [rbp + 0x33b], 1
1417a4c2e: 4c8d8dd0020000  lea r9, [rbp + 0x2d0]
1417a4c35: 41b801000000  mov r8d, 1
1417a4c3b: e8503db9ff  call 0x141338990
1417a4c40: e946d3ffff  jmp 0x1417a1f8b
1417a4c45: 488b8d78010000  mov rcx, qword ptr [rbp + 0x178]
1417a4c4c: 4883fa15  cmp rdx, 0x15
1417a4c50: 0f832f070000  jae 0x1417a5385
1417a4c56: 488d04d1  lea rax, [rcx + rdx*8]
1417a4c5a: 488d5108  lea rdx, [rcx + 8]
1417a4c5e: 41b808000000  mov r8d, 8
1417a4c64: eb17  jmp 0x1417a4c7d
1417a4c66: 4989ca  mov r10, rcx
1417a4c69: 4d890a  mov qword ptr [r10], r9
1417a4c6c: 4883c208  add rdx, 8
1417a4c70: 4983c008  add r8, 8
1417a4c74: 4839c2  cmp rdx, rax
1417a4c77: 0f841fd3ffff  je 0x1417a1f9c
1417a4c7d: 4c8b5af8  mov r11, qword ptr [rdx - 8]
1417a4c81: 4c8b0a  mov r9, qword ptr [rdx]
1417a4c84: 4d39d9  cmp r9, r11
1417a4c87: 73e3  jae 0x1417a4c6c
1417a4c89: 4d89c2  mov r10, r8
1417a4c8c: 4e891c11  mov qword ptr [rcx + r10], r11
1417a4c90: 4983fa08  cmp r10, 8
1417a4c94: 74d0  je 0x1417a4c66
1417a4c96: 4e8b5c11f0  mov r11, qword ptr [rcx + r10 - 0x10]
1417a4c9b: 4983c2f8  add r10, -8
1417a4c9f: 4d39d9  cmp r9, r11
1417a4ca2: 72e8  jb 0x1417a4c8c
1417a4ca4: 4901ca  add r10, rcx
1417a4ca7: ebc0  jmp 0x1417a4c69
1417a4ca9: c6853f03000001  mov byte ptr [rbp + 0x33f], 1
1417a4cb0: c6853e03000001  mov byte ptr [rbp + 0x33e], 1
1417a4cb7: 48c744242001000000  mov qword ptr [rsp + 0x20], 1
1417a4cc0: 41b901000000  mov r9d, 1
1417a4cc6: 4889f1  mov rcx, rsi
1417a4cc9: e8924d1001  call 0x1428a9a60
1417a4cce: 488b5610  mov rdx, qword ptr [rsi + 0x10]
1417a4cd2: e9e4e7ffff  jmp 0x1417a34bb
1417a4cd7: c6853f03000001  mov byte ptr [rbp + 0x33f], 1
1417a4cde: c6853e03000001  mov byte ptr [rbp + 0x33e], 1
1417a4ce5: 48c744242001000000  mov qword ptr [rsp + 0x20], 1
1417a4cee: 41b901000000  mov r9d, 1
1417a4cf4: 4889f1  mov rcx, rsi
1417a4cf7: e8644d1001  call 0x1428a9a60
1417a4cfc: 488b5610  mov rdx, qword ptr [rsi + 0x10]
1417a4d00: e969f7ffff  jmp 0x1417a446e
1417a4d05: 48c744242001000000  mov qword ptr [rsp + 0x20], 1
1417a4d0e: 41b901000000  mov r9d, 1
1417a4d14: 4889f1  mov rcx, rsi
1417a4d17: e8444d1001  call 0x1428a9a60
1417a4d1c: 488b5610  mov rdx, qword ptr [rsi + 0x10]
1417a4d20: e902f0ffff  jmp 0x1417a3d27
1417a4d25: 48c744242001000000  mov qword ptr [rsp + 0x20], 1
1417a4d2e: 41b901000000  mov r9d, 1
1417a4d34: 4889f1  mov rcx, rsi
1417a4d37: e8244d1001  call 0x1428a9a60
1417a4d3c: e93ce5ffff  jmp 0x1417a327d
1417a4d41: 48c744242001000000  mov qword ptr [rsp + 0x20], 1
1417a4d4a: 41b901000000  mov r9d, 1
1417a4d50: 4889f9  mov rcx, rdi
1417a4d53: e8084d1001  call 0x1428a9a60
1417a4d58: e9d9e6ffff  jmp 0x1417a3436
1417a4d5d: 48c744242001000000  mov qword ptr [rsp + 0x20], 1
1417a4d66: 41b901000000  mov r9d, 1
1417a4d6c: 4889f1  mov rcx, rsi
1417a4d6f: e8ec4c1001  call 0x1428a9a60
1417a4d74: e9b2ebffff  jmp 0x1417a392b
1417a4d79: c6853f03000000  mov byte ptr [rbp + 0x33f], 0
1417a4d80: c6853e03000001  mov byte ptr [rbp + 0x33e], 1
1417a4d87: 48c744242001000000  mov qword ptr [rsp + 0x20], 1
1417a4d90: 41b901000000  mov r9d, 1
1417a4d96: 4889f1  mov rcx, rsi
1417a4d99: e8c24c1001  call 0x1428a9a60
1417a4d9e: 488b5610  mov rdx, qword ptr [rsi + 0x10]
1417a4da2: e9a5f1ffff  jmp 0x1417a3f4c
1417a4da7: 48c744242001000000  mov qword ptr [rsp + 0x20], 1
1417a4db0: 41b901000000  mov r9d, 1
1417a4db6: 4889f9  mov rcx, rdi
1417a4db9: e8a24c1001  call 0x1428a9a60
1417a4dbe: e92af5ffff  jmp 0x1417a42ed
1417a4dc3: 48c744242001000000  mov qword ptr [rsp + 0x20], 1
1417a4dcc: 41b901000000  mov r9d, 1
1417a4dd2: 4889f9  mov rcx, rdi
1417a4dd5: e8864c1001  call 0x1428a9a60
1417a4dda: e91ef4ffff  jmp 0x1417a41fd
1417a4ddf: 4c8d05a2033401  lea r8, [rip + 0x13403a2]
1417a4de6: b901000000  mov ecx, 1
1417a4deb: ba00800000  mov edx, 0x8000
1417a4df0: e8ee411301  call 0x1428d8fe3
1417a4df5: e9c7050000  jmp 0x1417a53c1
1417a4dfa: c6852003000001  mov byte ptr [rbp + 0x320], 1
1417a4e01: 4c8d0538775301  lea r8, [rip + 0x1537738]
1417a4e08: b901000000  mov ecx, 1
1417a4e0d: ba00000800  mov edx, 0x80000
1417a4e12: e8cc411301  call 0x1428d8fe3
1417a4e17: e9a5050000  jmp 0x1417a53c1
1417a4e1c: 4c8d05fd745301  lea r8, [rip + 0x15374fd]
1417a4e23: b901000000  mov ecx, 1
1417a4e28: ba07000000  mov edx, 7
1417a4e2d: e8b1411301  call 0x1428d8fe3
1417a4e32: e98a050000  jmp 0x1417a53c1
1417a4e37: 4c8d05e2745301  lea r8, [rip + 0x15374e2]
1417a4e3e: b901000000  mov ecx, 1
1417a4e43: ba05000000  mov edx, 5
1417a4e48: e896411301  call 0x1428d8fe3
1417a4e4d: e96f050000  jmp 0x1417a53c1
1417a4e52: 488b85d8020000  mov rax, qword ptr [rbp + 0x2d8]
1417a4e59: 48898590020000  mov qword ptr [rbp + 0x290], rax
1417a4e60: 488d05f9765301  lea rax, [rip + 0x15376f9]
1417a4e67: 4889442420  mov qword ptr [rsp + 0x20], rax
1417a4e6c: 488d0d6d735301  lea rcx, [rip + 0x153736d]
1417a4e73: 4c8d0d46735301  lea r9, [rip + 0x1537346]
1417a4e7a: 4c8d8590020000  lea r8, [rbp + 0x290]
1417a4e81: ba2b000000  mov edx, 0x2b
1417a4e86: e8d5481301  call 0x1428d9760
1417a4e8b: e931050000  jmp 0x1417a53c1
1417a4e90: 4c8d0589745301  lea r8, [rip + 0x1537489]
1417a4e97: b901000000  mov ecx, 1
1417a4e9c: ba05000000  mov edx, 5
1417a4ea1: e83d411301  call 0x1428d8fe3
1417a4ea6: e916050000  jmp 0x1417a53c1
1417a4eab: 488b85d8020000  mov rax, qword ptr [rbp + 0x2d8]
1417a4eb2: 48898590020000  mov qword ptr [rbp + 0x290], rax
1417a4eb9: 488d05a0765301  lea rax, [rip + 0x15376a0]
1417a4ec0: 4889442420  mov qword ptr [rsp + 0x20], rax
1417a4ec5: 488d0d14735301  lea rcx, [rip + 0x1537314]
1417a4ecc: 4c8d0ded725301  lea r9, [rip + 0x15372ed]
1417a4ed3: 4c8d8590020000  lea r8, [rbp + 0x290]
1417a4eda: ba2b000000  mov edx, 0x2b
1417a4edf: e87c481301  call 0x1428d9760
1417a4ee4: e9d8040000  jmp 0x1417a53c1
1417a4ee9: 4c8d0530745301  lea r8, [rip + 0x1537430]
1417a4ef0: b901000000  mov ecx, 1
1417a4ef5: ba0a000000  mov edx, 0xa
1417a4efa: e8e4401301  call 0x1428d8fe3
1417a4eff: e9bd040000  jmp 0x1417a53c1
1417a4f04: 488b85d8020000  mov rax, qword ptr [rbp + 0x2d8]
1417a4f0b: 48898590020000  mov qword ptr [rbp + 0x290], rax
1417a4f12: 488d0547765301  lea rax, [rip + 0x1537647]
1417a4f19: 4889442420  mov qword ptr [rsp + 0x20], rax
1417a4f1e: 488d0dbb725301  lea rcx, [rip + 0x15372bb]
1417a4f25: 4c8d0d94725301  lea r9, [rip + 0x1537294]
1417a4f2c: 4c8d8590020000  lea r8, [rbp + 0x290]
1417a4f33: ba2b000000  mov edx, 0x2b
1417a4f38: e823481301  call 0x1428d9760
1417a4f3d: e97f040000  jmp 0x1417a53c1
1417a4f42: 4c8d05d7735301  lea r8, [rip + 0x15373d7]
1417a4f49: b901000000  mov ecx, 1
1417a4f4e: ba06000000  mov edx, 6
1417a4f53: e88b401301  call 0x1428d8fe3
1417a4f58: e964040000  jmp 0x1417a53c1
1417a4f5d: 488b85d8020000  mov rax, qword ptr [rbp + 0x2d8]
1417a4f64: 48898590020000  mov qword ptr [rbp + 0x290], rax
1417a4f6b: 488d05ee755301  lea rax, [rip + 0x15375ee]
1417a4f72: 4889442420  mov qword ptr [rsp + 0x20], rax
1417a4f77: 488d0d62725301  lea rcx, [rip + 0x1537262]
1417a4f7e: 4c8d0d3b725301  lea r9, [rip + 0x153723b]
1417a4f85: 4c8d8590020000  lea r8, [rbp + 0x290]
1417a4f8c: ba2b000000  mov edx, 0x2b
1417a4f91: e8ca471301  call 0x1428d9760
1417a4f96: e926040000  jmp 0x1417a53c1
1417a4f9b: 4c8d057e735301  lea r8, [rip + 0x153737e]
1417a4fa2: b901000000  mov ecx, 1
1417a4fa7: ba0a000000  mov edx, 0xa
1417a4fac: e832401301  call 0x1428d8fe3
1417a4fb1: e90b040000  jmp 0x1417a53c1
1417a4fb6: 488b85d8020000  mov rax, qword ptr [rbp + 0x2d8]
1417a4fbd: 48898590020000  mov qword ptr [rbp + 0x290], rax
1417a4fc4: 488d0595755301  lea rax, [rip + 0x1537595]
1417a4fcb: 4889442420  mov qword ptr [rsp + 0x20], rax
1417a4fd0: 488d0d09725301  lea rcx, [rip + 0x1537209]
1417a4fd7: 4c8d0de2715301  lea r9, [rip + 0x15371e2]
1417a4fde: 4c8d8590020000  lea r8, [rbp + 0x290]
1417a4fe5: ba2b000000  mov edx, 0x2b
1417a4fea: e871471301  call 0x1428d9760
1417a4fef: e9cd030000  jmp 0x1417a53c1
1417a4ff4: 4c8d0525735301  lea r8, [rip + 0x1537325]
1417a4ffb: b901000000  mov ecx, 1
1417a5000: ba0a000000  mov edx, 0xa
1417a5005: e8d93f1301  call 0x1428d8fe3
1417a500a: e9b2030000  jmp 0x1417a53c1
1417a500f: 488b85d8020000  mov rax, qword ptr [rbp + 0x2d8]
1417a5016: 48898590020000  mov qword ptr [rbp + 0x290], rax
1417a501d: 488d053c755301  lea rax, [rip + 0x153753c]
1417a5024: 4889442420  mov qword ptr [rsp + 0x20], rax
1417a5029: 488d0db0715301  lea rcx, [rip + 0x15371b0]
1417a5030: 4c8d0d89715301  lea r9, [rip + 0x1537189]
1417a5037: 4c8d8590020000  lea r8, [rbp + 0x290]
1417a503e: ba2b000000  mov edx, 0x2b
1417a5043: e818471301  call 0x1428d9760
1417a5048: e974030000  jmp 0x1417a53c1
1417a504d: 4c8d05cc725301  lea r8, [rip + 0x15372cc]
1417a5054: b901000000  mov ecx, 1
1417a5059: ba10000000  mov edx, 0x10
1417a505e: e8803f1301  call 0x1428d8fe3
1417a5063: e959030000  jmp 0x1417a53c1
1417a5068: 488b85d8020000  mov rax, qword ptr [rbp + 0x2d8]
1417a506f: 48898590020000  mov qword ptr [rbp + 0x290], rax
1417a5076: 488d05e3745301  lea rax, [rip + 0x15374e3]
1417a507d: 4889442420  mov qword ptr [rsp + 0x20], rax
1417a5082: 488d0d57715301  lea rcx, [rip + 0x1537157]
1417a5089: 4c8d0d30715301  lea r9, [rip + 0x1537130]
1417a5090: 4c8d8590020000  lea r8, [rbp + 0x290]
1417a5097: ba2b000000  mov edx, 0x2b
1417a509c: e8bf461301  call 0x1428d9760
1417a50a1: e91b030000  jmp 0x1417a53c1
1417a50a6: 4c8d0573725301  lea r8, [rip + 0x1537273]
1417a50ad: b901000000  mov ecx, 1
1417a50b2: ba07000000  mov edx, 7
1417a50b7: e8273f1301  call 0x1428d8fe3
1417a50bc: e900030000  jmp 0x1417a53c1
1417a50c1: 488b85d8020000  mov rax, qword ptr [rbp + 0x2d8]
1417a50c8: 48898590020000  mov qword ptr [rbp + 0x290], rax
1417a50cf: 488d058a745301  lea rax, [rip + 0x153748a]
1417a50d6: 4889442420  mov qword ptr [rsp + 0x20], rax
1417a50db: 488d0dfe705301  lea rcx, [rip + 0x15370fe]
1417a50e2: 4c8d0dd7705301  lea r9, [rip + 0x15370d7]
1417a50e9: 4c8d8590020000  lea r8, [rbp + 0x290]
1417a50f0: ba2b000000  mov edx, 0x2b
1417a50f5: e866461301  call 0x1428d9760
1417a50fa: e9c2020000  jmp 0x1417a53c1
1417a50ff: 4c8d051a725301  lea r8, [rip + 0x153721a]
1417a5106: b901000000  mov ecx, 1
1417a510b: ba12000000  mov edx, 0x12
1417a5110: e8ce3e1301  call 0x1428d8fe3
1417a5115: e9a7020000  jmp 0x1417a53c1
1417a511a: 488b85d8020000  mov rax, qword ptr [rbp + 0x2d8]
1417a5121: 48898590020000  mov qword ptr [rbp + 0x290], rax
1417a5128: 488d0531745301  lea rax, [rip + 0x1537431]
1417a512f: 4889442420  mov qword ptr [rsp + 0x20], rax
1417a5134: 488d0da5705301  lea rcx, [rip + 0x15370a5]
1417a513b: 4c8d0d7e705301  lea r9, [rip + 0x153707e]
1417a5142: 4c8d8590020000  lea r8, [rbp + 0x290]
1417a5149: ba2b000000  mov edx, 0x2b
1417a514e: e80d461301  call 0x1428d9760
1417a5153: e969020000  jmp 0x1417a53c1
1417a5158: 4c8d05c1715301  lea r8, [rip + 0x15371c1]
1417a515f: b901000000  mov ecx, 1
1417a5164: ba05000000  mov edx, 5
1417a5169: e8753e1301  call 0x1428d8fe3
1417a516e: e94e020000  jmp 0x1417a53c1
1417a5173: 488b85d8020000  mov rax, qword ptr [rbp + 0x2d8]
1417a517a: 48898590020000  mov qword ptr [rbp + 0x290], rax
1417a5181: 488d05d8735301  lea rax, [rip + 0x15373d8]
1417a5188: 4889442420  mov qword ptr [rsp + 0x20], rax
1417a518d: 488d0d4c705301  lea rcx, [rip + 0x153704c]
1417a5194: 4c8d0d25705301  lea r9, [rip + 0x1537025]
1417a519b: 4c8d8590020000  lea r8, [rbp + 0x290]
1417a51a2: ba2b000000  mov edx, 0x2b
1417a51a7: e8b4451301  call 0x1428d9760
1417a51ac: e910020000  jmp 0x1417a53c1
1417a51b1: 4c8d0568715301  lea r8, [rip + 0x1537168]
1417a51b8: b901000000  mov ecx, 1
1417a51bd: ba0b000000  mov edx, 0xb
1417a51c2: e81c3e1301  call 0x1428d8fe3
1417a51c7: e9f5010000  jmp 0x1417a53c1
1417a51cc: 488b85d8020000  mov rax, qword ptr [rbp + 0x2d8]
1417a51d3: 48898590020000  mov qword ptr [rbp + 0x290], rax
1417a51da: 488d057f735301  lea rax, [rip + 0x153737f]
1417a51e1: 4889442420  mov qword ptr [rsp + 0x20], rax
1417a51e6: 488d0df36f5301  lea rcx, [rip + 0x1536ff3]
1417a51ed: 4c8d0dcc6f5301  lea r9, [rip + 0x1536fcc]
1417a51f4: 4c8d8590020000  lea r8, [rbp + 0x290]
1417a51fb: ba2b000000  mov edx, 0x2b
1417a5200: e85b451301  call 0x1428d9760
1417a5205: e9b7010000  jmp 0x1417a53c1
1417a520a: 4c8d050f715301  lea r8, [rip + 0x153710f]
1417a5211: b901000000  mov ecx, 1
1417a5216: ba06000000  mov edx, 6
1417a521b: e8c33d1301  call 0x1428d8fe3
1417a5220: e99c010000  jmp 0x1417a53c1
1417a5225: 488b85d8020000  mov rax, qword ptr [rbp + 0x2d8]
1417a522c: 48898590020000  mov qword ptr [rbp + 0x290], rax
1417a5233: 488d0526735301  lea rax, [rip + 0x1537326]
1417a523a: 4889442420  mov qword ptr [rsp + 0x20], rax
1417a523f: 488d0d9a6f5301  lea rcx, [rip + 0x1536f9a]
1417a5246: 4c8d0d736f5301  lea r9, [rip + 0x1536f73]
1417a524d: 4c8d8590020000  lea r8, [rbp + 0x290]
1417a5254: ba2b000000  mov edx, 0x2b
1417a5259: e802451301  call 0x1428d9760
1417a525e: e95e010000  jmp 0x1417a53c1
1417a5263: 4c8d05b6705301  lea r8, [rip + 0x15370b6]
1417a526a: b901000000  mov ecx, 1
1417a526f: ba15000000  mov edx, 0x15
1417a5274: e86a3d1301  call 0x1428d8fe3
1417a5279: e943010000  jmp 0x1417a53c1
1417a527e: 488b85d8020000  mov rax, qword ptr [rbp + 0x2d8]
1417a5285: 48898590020000  mov qword ptr [rbp + 0x290], rax
1417a528c: 488d05cd725301  lea rax, [rip + 0x15372cd]
1417a5293: 4889442420  mov qword ptr [rsp + 0x20], rax
1417a5298: 488d0d416f5301  lea rcx, [rip + 0x1536f41]
1417a529f: 4c8d0d1a6f5301  lea r9, [rip + 0x1536f1a]
1417a52a6: 4c8d8590020000  lea r8, [rbp + 0x290]
1417a52ad: ba2b000000  mov edx, 0x2b
1417a52b2: e8a9441301  call 0x1428d9760
1417a52b7: e905010000  jmp 0x1417a53c1
1417a52bc: 488d0d352e3201  lea rcx, [rip + 0x1322e35]
1417a52c3: e848401301  call 0x1428d9310
1417a52c8: e9f4000000  jmp 0x1417a53c1
1417a52cd: 4c89bd30030000  mov qword ptr [rbp + 0x330], r15
1417a52d4: 4c8d05cd6e5301  lea r8, [rip + 0x1536ecd]
1417a52db: b908000000  mov ecx, 8
1417a52e0: 4889f2  mov rdx, rsi
1417a52e3: e8fb3c1301  call 0x1428d8fe3
1417a52e8: e9d4000000  jmp 0x1417a53c1
1417a52ed: c6853d03000001  mov byte ptr [rbp + 0x33d], 1
1417a52f4: c6853c03000001  mov byte ptr [rbp + 0x33c], 1
1417a52fb: c6853b03000001  mov byte ptr [rbp + 0x33b], 1
1417a5302: 4c8d85d0020000  lea r8, [rbp + 0x2d0]
1417a5309: e8d2381bff  call 0x140958be0
1417a530e: e91bccffff  jmp 0x1417a1f2e
1417a5313: c6853d03000001  mov byte ptr [rbp + 0x33d], 1
1417a531a: c6853c03000001  mov byte ptr [rbp + 0x33c], 1
1417a5321: c6853b03000001  mov byte ptr [rbp + 0x33b], 1
1417a5328: 4c8d85d0020000  lea r8, [rbp + 0x2d0]
1417a532f: e8dc2d1bff  call 0x140958110
1417a5334: e914ccffff  jmp 0x1417a1f4d
1417a5339: c6853d03000001  mov byte ptr [rbp + 0x33d], 1
1417a5340: c6853c03000001  mov byte ptr [rbp + 0x33c], 1
1417a5347: c6853b03000001  mov byte ptr [rbp + 0x33b], 1
1417a534e: 4c8d85d0020000  lea r8, [rbp + 0x2d0]
1417a5355: e8f62e1bff  call 0x140958250
1417a535a: e90dccffff  jmp 0x1417a1f6c
1417a535f: c6853d03000001  mov byte ptr [rbp + 0x33d], 1
1417a5366: c6853c03000001  mov byte ptr [rbp + 0x33c], 1
1417a536d: c6853b03000001  mov byte ptr [rbp + 0x33b], 1
1417a5374: 4c8d85d0020000  lea r8, [rbp + 0x2d0]
1417a537b: e8c0311bff  call 0x140958540
1417a5380: e906ccffff  jmp 0x1417a1f8b
1417a5385: c6853d03000001  mov byte ptr [rbp + 0x33d], 1
1417a538c: c6853c03000001  mov byte ptr [rbp + 0x33c], 1
1417a5393: c6853b03000001  mov byte ptr [rbp + 0x33b], 1
1417a539a: 4c8d8590020000  lea r8, [rbp + 0x290]
1417a53a1: e8ba391bff  call 0x140958d60
1417a53a6: e9f1cbffff  jmp 0x1417a1f9c
1417a53ab: 4c8d05d6fd3301  lea r8, [rip + 0x133fdd6]
1417a53b2: b901000000  mov ecx, 1
1417a53b7: ba00800000  mov edx, 0x8000
1417a53bc: e8223c1301  call 0x1428d8fe3
1417a53c1: 0f0b  ud2
1417a53c3: 666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a53d0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a53d5: 55  push rbp
1417a53d6: 4157  push r15
1417a53d8: 4156  push r14
1417a53da: 4155  push r13
1417a53dc: 4154  push r12
1417a53de: 56  push rsi
1417a53df: 57  push rdi
1417a53e0: 53  push rbx
1417a53e1: 4883ec28  sub rsp, 0x28
1417a53e5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a53ec: 488b8530030000  mov rax, qword ptr [rbp + 0x330]
1417a53f3: 48c70000000000  mov qword ptr [rax], 0
1417a53fa: c6853f03000000  mov byte ptr [rbp + 0x33f], 0
1417a5401: c6853e03000001  mov byte ptr [rbp + 0x33e], 1
1417a5408: 4883c428  add rsp, 0x28
1417a540c: 5b  pop rbx
1417a540d: 5f  pop rdi
1417a540e: 5e  pop rsi
1417a540f: 415c  pop r12
1417a5411: 415d  pop r13
1417a5413: 415e  pop r14
1417a5415: 415f  pop r15
1417a5417: 5d  pop rbp
1417a5418: c3  ret
1417a5419: 0f1f8000000000  nop dword ptr [rax]
1417a5420: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5425: 55  push rbp
1417a5426: 4157  push r15
1417a5428: 4156  push r14
1417a542a: 4155  push r13
1417a542c: 4154  push r12
1417a542e: 56  push rsi
1417a542f: 57  push rdi
1417a5430: 53  push rbx
1417a5431: 4883ec28  sub rsp, 0x28
1417a5435: 488daa80000000  lea rbp, [rdx + 0x80]
1417a543c: 488b8588020000  mov rax, qword ptr [rbp + 0x288]
1417a5443: 48c70000000000  mov qword ptr [rax], 0
1417a544a: 4883c428  add rsp, 0x28
1417a544e: 5b  pop rbx
1417a544f: 5f  pop rdi
1417a5450: 5e  pop rsi
1417a5451: 415c  pop r12
1417a5453: 415d  pop r13
1417a5455: 415e  pop r14
1417a5457: 415f  pop r15
1417a5459: 5d  pop rbp
1417a545a: c3  ret
1417a545b: 0f1f440000  nop dword ptr [rax + rax]
1417a5460: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5465: 55  push rbp
1417a5466: 4157  push r15
1417a5468: 4156  push r14
1417a546a: 4155  push r13
1417a546c: 4154  push r12
1417a546e: 56  push rsi
1417a546f: 57  push rdi
1417a5470: 53  push rbx
1417a5471: 4883ec28  sub rsp, 0x28
1417a5475: 488daa80000000  lea rbp, [rdx + 0x80]
1417a547c: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a5483: e80824e4fe  call 0x1405e7890
1417a5488: 90  nop
1417a5489: 4883c428  add rsp, 0x28
1417a548d: 5b  pop rbx
1417a548e: 5f  pop rdi
1417a548f: 5e  pop rsi
1417a5490: 415c  pop r12
1417a5492: 415d  pop r13
1417a5494: 415e  pop r14
1417a5496: 415f  pop r15
1417a5498: 5d  pop rbp
1417a5499: c3  ret
1417a549a: 660f1f440000  nop word ptr [rax + rax]
1417a54a0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a54a5: 55  push rbp
1417a54a6: 4157  push r15
1417a54a8: 4156  push r14
1417a54aa: 4155  push r13
1417a54ac: 4154  push r12
1417a54ae: 56  push rsi
1417a54af: 57  push rdi
1417a54b0: 53  push rbx
1417a54b1: 4883ec28  sub rsp, 0x28
1417a54b5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a54bc: 80bdcf02000000  cmp byte ptr [rbp + 0x2cf], 0
1417a54c3: 7413  je 0x1417a54d8
1417a54c5: c6852e03000001  mov byte ptr [rbp + 0x32e], 1
1417a54cc: 488d8d28020000  lea rcx, [rbp + 0x228]
1417a54d3: e82825e4fe  call 0x1405e7a00
1417a54d8: c6852e03000001  mov byte ptr [rbp + 0x32e], 1
1417a54df: 4883c428  add rsp, 0x28
1417a54e3: 5b  pop rbx
1417a54e4: 5f  pop rdi
1417a54e5: 5e  pop rsi
1417a54e6: 415c  pop r12
1417a54e8: 415d  pop r13
1417a54ea: 415e  pop r14
1417a54ec: 415f  pop r15
1417a54ee: 5d  pop rbp
1417a54ef: c3  ret
1417a54f0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a54f5: 55  push rbp
1417a54f6: 4157  push r15
1417a54f8: 4156  push r14
1417a54fa: 4155  push r13
1417a54fc: 4154  push r12
1417a54fe: 56  push rsi
1417a54ff: 57  push rdi
1417a5500: 53  push rbx
1417a5501: 4883ec28  sub rsp, 0x28
1417a5505: 488daa80000000  lea rbp, [rdx + 0x80]
1417a550c: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a5513: 4885d2  test rdx, rdx
1417a5516: 7416  je 0x1417a552e
1417a5518: 48c1e205  shl rdx, 5
1417a551c: 41b808000000  mov r8d, 8
1417a5522: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a5529: e8f2e6e6fe  call 0x140613c20
1417a552e: 90  nop
1417a552f: 4883c428  add rsp, 0x28
1417a5533: 5b  pop rbx
1417a5534: 5f  pop rdi
1417a5535: 5e  pop rsi
1417a5536: 415c  pop r12
1417a5538: 415d  pop r13
1417a553a: 415e  pop r14
1417a553c: 415f  pop r15
1417a553e: 5d  pop rbp
1417a553f: c3  ret
1417a5540: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5545: 55  push rbp
1417a5546: 4157  push r15
1417a5548: 4156  push r14
1417a554a: 4155  push r13
1417a554c: 4154  push r12
1417a554e: 56  push rsi
1417a554f: 57  push rdi
1417a5550: 53  push rbx
1417a5551: 4883ec28  sub rsp, 0x28
1417a5555: 488daa80000000  lea rbp, [rdx + 0x80]
1417a555c: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a5563: 4885d2  test rdx, rdx
1417a5566: 7416  je 0x1417a557e
1417a5568: 48c1e205  shl rdx, 5
1417a556c: 41b808000000  mov r8d, 8
1417a5572: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a5579: e8a2e6e6fe  call 0x140613c20
1417a557e: 90  nop
1417a557f: 4883c428  add rsp, 0x28
1417a5583: 5b  pop rbx
1417a5584: 5f  pop rdi
1417a5585: 5e  pop rsi
1417a5586: 415c  pop r12
1417a5588: 415d  pop r13
1417a558a: 415e  pop r14
1417a558c: 415f  pop r15
1417a558e: 5d  pop rbp
1417a558f: c3  ret
1417a5590: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5595: 55  push rbp
1417a5596: 4157  push r15
1417a5598: 4156  push r14
1417a559a: 4155  push r13
1417a559c: 4154  push r12
1417a559e: 56  push rsi
1417a559f: 57  push rdi
1417a55a0: 53  push rbx
1417a55a1: 4883ec28  sub rsp, 0x28
1417a55a5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a55ac: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a55b3: 4885d2  test rdx, rdx
1417a55b6: 7416  je 0x1417a55ce
1417a55b8: 48c1e205  shl rdx, 5
1417a55bc: 41b808000000  mov r8d, 8
1417a55c2: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a55c9: e852e6e6fe  call 0x140613c20
1417a55ce: 90  nop
1417a55cf: 4883c428  add rsp, 0x28
1417a55d3: 5b  pop rbx
1417a55d4: 5f  pop rdi
1417a55d5: 5e  pop rsi
1417a55d6: 415c  pop r12
1417a55d8: 415d  pop r13
1417a55da: 415e  pop r14
1417a55dc: 415f  pop r15
1417a55de: 5d  pop rbp
1417a55df: c3  ret
1417a55e0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a55e5: 55  push rbp
1417a55e6: 4157  push r15
1417a55e8: 4156  push r14
1417a55ea: 4155  push r13
1417a55ec: 4154  push r12
1417a55ee: 56  push rsi
1417a55ef: 57  push rdi
1417a55f0: 53  push rbx
1417a55f1: 4883ec28  sub rsp, 0x28
1417a55f5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a55fc: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a5603: 4885d2  test rdx, rdx
1417a5606: 7416  je 0x1417a561e
1417a5608: 48c1e205  shl rdx, 5
1417a560c: 41b808000000  mov r8d, 8
1417a5612: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a5619: e802e6e6fe  call 0x140613c20
1417a561e: 90  nop
1417a561f: 4883c428  add rsp, 0x28
1417a5623: 5b  pop rbx
1417a5624: 5f  pop rdi
1417a5625: 5e  pop rsi
1417a5626: 415c  pop r12
1417a5628: 415d  pop r13
1417a562a: 415e  pop r14
1417a562c: 415f  pop r15
1417a562e: 5d  pop rbp
1417a562f: c3  ret
1417a5630: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5635: 55  push rbp
1417a5636: 4157  push r15
1417a5638: 4156  push r14
1417a563a: 4155  push r13
1417a563c: 4154  push r12
1417a563e: 56  push rsi
1417a563f: 57  push rdi
1417a5640: 53  push rbx
1417a5641: 4883ec28  sub rsp, 0x28
1417a5645: 488daa80000000  lea rbp, [rdx + 0x80]
1417a564c: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a5653: 4885d2  test rdx, rdx
1417a5656: 7416  je 0x1417a566e
1417a5658: 48c1e205  shl rdx, 5
1417a565c: 41b808000000  mov r8d, 8
1417a5662: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a5669: e8b2e5e6fe  call 0x140613c20
1417a566e: 90  nop
1417a566f: 4883c428  add rsp, 0x28
1417a5673: 5b  pop rbx
1417a5674: 5f  pop rdi
1417a5675: 5e  pop rsi
1417a5676: 415c  pop r12
1417a5678: 415d  pop r13
1417a567a: 415e  pop r14
1417a567c: 415f  pop r15
1417a567e: 5d  pop rbp
1417a567f: c3  ret
1417a5680: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5685: 55  push rbp
1417a5686: 4157  push r15
1417a5688: 4156  push r14
1417a568a: 4155  push r13
1417a568c: 4154  push r12
1417a568e: 56  push rsi
1417a568f: 57  push rdi
1417a5690: 53  push rbx
1417a5691: 4883ec28  sub rsp, 0x28
1417a5695: 488daa80000000  lea rbp, [rdx + 0x80]
1417a569c: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a56a3: 4885d2  test rdx, rdx
1417a56a6: 7416  je 0x1417a56be
1417a56a8: 48c1e205  shl rdx, 5
1417a56ac: 41b808000000  mov r8d, 8
1417a56b2: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a56b9: e862e5e6fe  call 0x140613c20
1417a56be: 90  nop
1417a56bf: 4883c428  add rsp, 0x28
1417a56c3: 5b  pop rbx
1417a56c4: 5f  pop rdi
1417a56c5: 5e  pop rsi
1417a56c6: 415c  pop r12
1417a56c8: 415d  pop r13
1417a56ca: 415e  pop r14
1417a56cc: 415f  pop r15
1417a56ce: 5d  pop rbp
1417a56cf: c3  ret
1417a56d0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a56d5: 55  push rbp
1417a56d6: 4157  push r15
1417a56d8: 4156  push r14
1417a56da: 4155  push r13
1417a56dc: 4154  push r12
1417a56de: 56  push rsi
1417a56df: 57  push rdi
1417a56e0: 53  push rbx
1417a56e1: 4883ec28  sub rsp, 0x28
1417a56e5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a56ec: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a56f3: 4885d2  test rdx, rdx
1417a56f6: 7416  je 0x1417a570e
1417a56f8: 48c1e205  shl rdx, 5
1417a56fc: 41b808000000  mov r8d, 8
1417a5702: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a5709: e812e5e6fe  call 0x140613c20
1417a570e: 90  nop
1417a570f: 4883c428  add rsp, 0x28
1417a5713: 5b  pop rbx
1417a5714: 5f  pop rdi
1417a5715: 5e  pop rsi
1417a5716: 415c  pop r12
1417a5718: 415d  pop r13
1417a571a: 415e  pop r14
1417a571c: 415f  pop r15
1417a571e: 5d  pop rbp
1417a571f: c3  ret
1417a5720: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5725: 55  push rbp
1417a5726: 4157  push r15
1417a5728: 4156  push r14
1417a572a: 4155  push r13
1417a572c: 4154  push r12
1417a572e: 56  push rsi
1417a572f: 57  push rdi
1417a5730: 53  push rbx
1417a5731: 4883ec28  sub rsp, 0x28
1417a5735: 488daa80000000  lea rbp, [rdx + 0x80]
1417a573c: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a5743: 4885d2  test rdx, rdx
1417a5746: 7416  je 0x1417a575e
1417a5748: 48c1e205  shl rdx, 5
1417a574c: 41b808000000  mov r8d, 8
1417a5752: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a5759: e8c2e4e6fe  call 0x140613c20
1417a575e: 90  nop
1417a575f: 4883c428  add rsp, 0x28
1417a5763: 5b  pop rbx
1417a5764: 5f  pop rdi
1417a5765: 5e  pop rsi
1417a5766: 415c  pop r12
1417a5768: 415d  pop r13
1417a576a: 415e  pop r14
1417a576c: 415f  pop r15
1417a576e: 5d  pop rbp
1417a576f: c3  ret
1417a5770: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5775: 55  push rbp
1417a5776: 4157  push r15
1417a5778: 4156  push r14
1417a577a: 4155  push r13
1417a577c: 4154  push r12
1417a577e: 56  push rsi
1417a577f: 57  push rdi
1417a5780: 53  push rbx
1417a5781: 4883ec28  sub rsp, 0x28
1417a5785: 488daa80000000  lea rbp, [rdx + 0x80]
1417a578c: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a5793: 4885d2  test rdx, rdx
1417a5796: 7416  je 0x1417a57ae
1417a5798: 48c1e205  shl rdx, 5
1417a579c: 41b808000000  mov r8d, 8
1417a57a2: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a57a9: e872e4e6fe  call 0x140613c20
1417a57ae: 90  nop
1417a57af: 4883c428  add rsp, 0x28
1417a57b3: 5b  pop rbx
1417a57b4: 5f  pop rdi
1417a57b5: 5e  pop rsi
1417a57b6: 415c  pop r12
1417a57b8: 415d  pop r13
1417a57ba: 415e  pop r14
1417a57bc: 415f  pop r15
1417a57be: 5d  pop rbp
1417a57bf: c3  ret
1417a57c0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a57c5: 55  push rbp
1417a57c6: 4157  push r15
1417a57c8: 4156  push r14
1417a57ca: 4155  push r13
1417a57cc: 4154  push r12
1417a57ce: 56  push rsi
1417a57cf: 57  push rdi
1417a57d0: 53  push rbx
1417a57d1: 4883ec28  sub rsp, 0x28
1417a57d5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a57dc: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a57e3: 4885d2  test rdx, rdx
1417a57e6: 7416  je 0x1417a57fe
1417a57e8: 48c1e205  shl rdx, 5
1417a57ec: 41b808000000  mov r8d, 8
1417a57f2: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a57f9: e822e4e6fe  call 0x140613c20
1417a57fe: 90  nop
1417a57ff: 4883c428  add rsp, 0x28
1417a5803: 5b  pop rbx
1417a5804: 5f  pop rdi
1417a5805: 5e  pop rsi
1417a5806: 415c  pop r12
1417a5808: 415d  pop r13
1417a580a: 415e  pop r14
1417a580c: 415f  pop r15
1417a580e: 5d  pop rbp
1417a580f: c3  ret
1417a5810: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5815: 55  push rbp
1417a5816: 4157  push r15
1417a5818: 4156  push r14
1417a581a: 4155  push r13
1417a581c: 4154  push r12
1417a581e: 56  push rsi
1417a581f: 57  push rdi
1417a5820: 53  push rbx
1417a5821: 4883ec28  sub rsp, 0x28
1417a5825: 488daa80000000  lea rbp, [rdx + 0x80]
1417a582c: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a5833: 4885d2  test rdx, rdx
1417a5836: 7416  je 0x1417a584e
1417a5838: 48c1e205  shl rdx, 5
1417a583c: 41b808000000  mov r8d, 8
1417a5842: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a5849: e8d2e3e6fe  call 0x140613c20
1417a584e: 90  nop
1417a584f: 4883c428  add rsp, 0x28
1417a5853: 5b  pop rbx
1417a5854: 5f  pop rdi
1417a5855: 5e  pop rsi
1417a5856: 415c  pop r12
1417a5858: 415d  pop r13
1417a585a: 415e  pop r14
1417a585c: 415f  pop r15
1417a585e: 5d  pop rbp
1417a585f: c3  ret
1417a5860: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5865: 55  push rbp
1417a5866: 4157  push r15
1417a5868: 4156  push r14
1417a586a: 4155  push r13
1417a586c: 4154  push r12
1417a586e: 56  push rsi
1417a586f: 57  push rdi
1417a5870: 53  push rbx
1417a5871: 4883ec28  sub rsp, 0x28
1417a5875: 488daa80000000  lea rbp, [rdx + 0x80]
1417a587c: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a5883: 4885d2  test rdx, rdx
1417a5886: 7416  je 0x1417a589e
1417a5888: 48c1e205  shl rdx, 5
1417a588c: 41b808000000  mov r8d, 8
1417a5892: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a5899: e882e3e6fe  call 0x140613c20
1417a589e: 90  nop
1417a589f: 4883c428  add rsp, 0x28
1417a58a3: 5b  pop rbx
1417a58a4: 5f  pop rdi
1417a58a5: 5e  pop rsi
1417a58a6: 415c  pop r12
1417a58a8: 415d  pop r13
1417a58aa: 415e  pop r14
1417a58ac: 415f  pop r15
1417a58ae: 5d  pop rbp
1417a58af: c3  ret
1417a58b0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a58b5: 55  push rbp
1417a58b6: 4157  push r15
1417a58b8: 4156  push r14
1417a58ba: 4155  push r13
1417a58bc: 4154  push r12
1417a58be: 56  push rsi
1417a58bf: 57  push rdi
1417a58c0: 53  push rbx
1417a58c1: 4883ec28  sub rsp, 0x28
1417a58c5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a58cc: 488b95d8020000  mov rdx, qword ptr [rbp + 0x2d8]
1417a58d3: 4885d2  test rdx, rdx
1417a58d6: 7416  je 0x1417a58ee
1417a58d8: 48c1e205  shl rdx, 5
1417a58dc: 41b808000000  mov r8d, 8
1417a58e2: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a58e9: e832e3e6fe  call 0x140613c20
1417a58ee: 90  nop
1417a58ef: 4883c428  add rsp, 0x28
1417a58f3: 5b  pop rbx
1417a58f4: 5f  pop rdi
1417a58f5: 5e  pop rsi
1417a58f6: 415c  pop r12
1417a58f8: 415d  pop r13
1417a58fa: 415e  pop r14
1417a58fc: 415f  pop r15
1417a58fe: 5d  pop rbp
1417a58ff: c3  ret
1417a5900: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5905: 55  push rbp
1417a5906: 4157  push r15
1417a5908: 4156  push r14
1417a590a: 4155  push r13
1417a590c: 4154  push r12
1417a590e: 56  push rsi
1417a590f: 57  push rdi
1417a5910: 53  push rbx
1417a5911: 4883ec28  sub rsp, 0x28
1417a5915: 488daa80000000  lea rbp, [rdx + 0x80]
1417a591c: 488b9598020000  mov rdx, qword ptr [rbp + 0x298]
1417a5923: 4885d2  test rdx, rdx
1417a5926: 7416  je 0x1417a593e
1417a5928: 48c1e205  shl rdx, 5
1417a592c: 41b808000000  mov r8d, 8
1417a5932: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a5939: e8e2e2e6fe  call 0x140613c20
1417a593e: 90  nop
1417a593f: 4883c428  add rsp, 0x28
1417a5943: 5b  pop rbx
1417a5944: 5f  pop rdi
1417a5945: 5e  pop rsi
1417a5946: 415c  pop r12
1417a5948: 415d  pop r13
1417a594a: 415e  pop r14
1417a594c: 415f  pop r15
1417a594e: 5d  pop rbp
1417a594f: c3  ret
1417a5950: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5955: 55  push rbp
1417a5956: 4157  push r15
1417a5958: 4156  push r14
1417a595a: 4155  push r13
1417a595c: 4154  push r12
1417a595e: 56  push rsi
1417a595f: 57  push rdi
1417a5960: 53  push rbx
1417a5961: 4883ec28  sub rsp, 0x28
1417a5965: 488daa80000000  lea rbp, [rdx + 0x80]
1417a596c: 488b8530030000  mov rax, qword ptr [rbp + 0x330]
1417a5973: 48c70000000000  mov qword ptr [rax], 0
1417a597a: c6853f03000001  mov byte ptr [rbp + 0x33f], 1
1417a5981: c6853e03000001  mov byte ptr [rbp + 0x33e], 1
1417a5988: 4883c428  add rsp, 0x28
1417a598c: 5b  pop rbx
1417a598d: 5f  pop rdi
1417a598e: 5e  pop rsi
1417a598f: 415c  pop r12
1417a5991: 415d  pop r13
1417a5993: 415e  pop r14
1417a5995: 415f  pop r15
1417a5997: 5d  pop rbp
1417a5998: c3  ret
1417a5999: 0f1f8000000000  nop dword ptr [rax]
1417a59a0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a59a5: 55  push rbp
1417a59a6: 4157  push r15
1417a59a8: 4156  push r14
1417a59aa: 4155  push r13
1417a59ac: 4154  push r12
1417a59ae: 56  push rsi
1417a59af: 57  push rdi
1417a59b0: 53  push rbx
1417a59b1: 4883ec28  sub rsp, 0x28
1417a59b5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a59bc: 80bdcc02000000  cmp byte ptr [rbp + 0x2cc], 0
1417a59c3: 7410  je 0x1417a59d5
1417a59c5: c6852f03000001  mov byte ptr [rbp + 0x32f], 1
1417a59cc: 488d4d28  lea rcx, [rbp + 0x28]
1417a59d0: e82b20e4fe  call 0x1405e7a00
1417a59d5: c6852f03000001  mov byte ptr [rbp + 0x32f], 1
1417a59dc: 4883c428  add rsp, 0x28
1417a59e0: 5b  pop rbx
1417a59e1: 5f  pop rdi
1417a59e2: 5e  pop rsi
1417a59e3: 415c  pop r12
1417a59e5: 415d  pop r13
1417a59e7: 415e  pop r14
1417a59e9: 415f  pop r15
1417a59eb: 5d  pop rbp
1417a59ec: c3  ret
1417a59ed: 0f1f00  nop dword ptr [rax]
1417a59f0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a59f5: 55  push rbp
1417a59f6: 4157  push r15
1417a59f8: 4156  push r14
1417a59fa: 4155  push r13
1417a59fc: 4154  push r12
1417a59fe: 56  push rsi
1417a59ff: 57  push rdi
1417a5a00: 53  push rbx
1417a5a01: 4883ec28  sub rsp, 0x28
1417a5a05: 488daa80000000  lea rbp, [rdx + 0x80]
1417a5a0c: 488b8d88020000  mov rcx, qword ptr [rbp + 0x288]
1417a5a13: 488b8540020000  mov rax, qword ptr [rbp + 0x240]
1417a5a1a: 488b5008  mov rdx, qword ptr [rax + 8]
1417a5a1e: 4885d2  test rdx, rdx
1417a5a21: 7410  je 0x1417a5a33
1417a5a23: 488b8540020000  mov rax, qword ptr [rbp + 0x240]
1417a5a2a: 4c8b4010  mov r8, qword ptr [rax + 0x10]
1417a5a2e: e8ede1e6fe  call 0x140613c20
1417a5a33: ba18000000  mov edx, 0x18
1417a5a38: 41b808000000  mov r8d, 8
1417a5a3e: 488b8d20020000  mov rcx, qword ptr [rbp + 0x220]
1417a5a45: e8d6e1e6fe  call 0x140613c20
1417a5a4a: 8b8530030000  mov eax, dword ptr [rbp + 0x330]
1417a5a50: 88851f030000  mov byte ptr [rbp + 0x31f], al
1417a5a56: c6851e03000001  mov byte ptr [rbp + 0x31e], 1
1417a5a5d: 4883c428  add rsp, 0x28
1417a5a61: 5b  pop rbx
1417a5a62: 5f  pop rdi
1417a5a63: 5e  pop rsi
1417a5a64: 415c  pop r12
1417a5a66: 415d  pop r13
1417a5a68: 415e  pop r14
1417a5a6a: 415f  pop r15
1417a5a6c: 5d  pop rbp
1417a5a6d: c3  ret
1417a5a6e: 6690  nop
1417a5a70: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5a75: 55  push rbp
1417a5a76: 4157  push r15
1417a5a78: 4156  push r14
1417a5a7a: 4155  push r13
1417a5a7c: 4154  push r12
1417a5a7e: 56  push rsi
1417a5a7f: 57  push rdi
1417a5a80: 53  push rbx
1417a5a81: 4883ec28  sub rsp, 0x28
1417a5a85: 488daa80000000  lea rbp, [rdx + 0x80]
1417a5a8c: 488b9508020000  mov rdx, qword ptr [rbp + 0x208]
1417a5a93: 4885d2  test rdx, rdx
1417a5a96: 7416  je 0x1417a5aae
1417a5a98: 48c1e205  shl rdx, 5
1417a5a9c: 41b808000000  mov r8d, 8
1417a5aa2: 488b8d40020000  mov rcx, qword ptr [rbp + 0x240]
1417a5aa9: e872e1e6fe  call 0x140613c20
1417a5aae: 8b8530030000  mov eax, dword ptr [rbp + 0x330]
1417a5ab4: 88853d030000  mov byte ptr [rbp + 0x33d], al
1417a5aba: 8b8588020000  mov eax, dword ptr [rbp + 0x288]
1417a5ac0: 88853c030000  mov byte ptr [rbp + 0x33c], al
1417a5ac6: c6853b03000001  mov byte ptr [rbp + 0x33b], 1
1417a5acd: 4883c428  add rsp, 0x28
1417a5ad1: 5b  pop rbx
1417a5ad2: 5f  pop rdi
1417a5ad3: 5e  pop rsi
1417a5ad4: 415c  pop r12
1417a5ad6: 415d  pop r13
1417a5ad8: 415e  pop r14
1417a5ada: 415f  pop r15
1417a5adc: 5d  pop rbp
1417a5add: c3  ret
1417a5ade: 6690  nop
1417a5ae0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5ae5: 55  push rbp
1417a5ae6: 4157  push r15
1417a5ae8: 4156  push r14
1417a5aea: 4155  push r13
1417a5aec: 4154  push r12
1417a5aee: 56  push rsi
1417a5aef: 57  push rdi
1417a5af0: 53  push rbx
1417a5af1: 4883ec28  sub rsp, 0x28
1417a5af5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a5afc: 488b8530030000  mov rax, qword ptr [rbp + 0x330]
1417a5b03: 48c70000000000  mov qword ptr [rax], 0
1417a5b0a: c6853f03000001  mov byte ptr [rbp + 0x33f], 1
1417a5b11: c6853e03000001  mov byte ptr [rbp + 0x33e], 1
1417a5b18: 4883c428  add rsp, 0x28
1417a5b1c: 5b  pop rbx
1417a5b1d: 5f  pop rdi
1417a5b1e: 5e  pop rsi
1417a5b1f: 415c  pop r12
1417a5b21: 415d  pop r13
1417a5b23: 415e  pop r14
1417a5b25: 415f  pop r15
1417a5b27: 5d  pop rbp
1417a5b28: c3  ret
1417a5b29: 0f1f8000000000  nop dword ptr [rax]
1417a5b30: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5b35: 55  push rbp
1417a5b36: 4157  push r15
1417a5b38: 4156  push r14
1417a5b3a: 4155  push r13
1417a5b3c: 4154  push r12
1417a5b3e: 56  push rsi
1417a5b3f: 57  push rdi
1417a5b40: 53  push rbx
1417a5b41: 4883ec28  sub rsp, 0x28
1417a5b45: 488daa80000000  lea rbp, [rdx + 0x80]
1417a5b4c: 80bd2e03000000  cmp byte ptr [rbp + 0x32e], 0
1417a5b53: 741e  je 0x1417a5b73
1417a5b55: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a5b5c: e82fbd0dff  call 0x140881890
1417a5b61: 90  nop
1417a5b62: 4883c428  add rsp, 0x28
1417a5b66: 5b  pop rbx
1417a5b67: 5f  pop rdi
1417a5b68: 5e  pop rsi
1417a5b69: 415c  pop r12
1417a5b6b: 415d  pop r13
1417a5b6d: 415e  pop r14
1417a5b6f: 415f  pop r15
1417a5b71: 5d  pop rbp
1417a5b72: c3  ret
1417a5b73: 4883c428  add rsp, 0x28
1417a5b77: 5b  pop rbx
1417a5b78: 5f  pop rdi
1417a5b79: 5e  pop rsi
1417a5b7a: 415c  pop r12
1417a5b7c: 415d  pop r13
1417a5b7e: 415e  pop r14
1417a5b80: 415f  pop r15
1417a5b82: 5d  pop rbp
1417a5b83: c3  ret
1417a5b84: 6666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a5b90: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5b95: 55  push rbp
1417a5b96: 4157  push r15
1417a5b98: 4156  push r14
1417a5b9a: 4155  push r13
1417a5b9c: 4154  push r12
1417a5b9e: 56  push rsi
1417a5b9f: 57  push rdi
1417a5ba0: 53  push rbx
1417a5ba1: 4883ec28  sub rsp, 0x28
1417a5ba5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a5bac: 488d8dd8010000  lea rcx, [rbp + 0x1d8]
1417a5bb3: e838500dff  call 0x14087abf0
1417a5bb8: c6853f03000000  mov byte ptr [rbp + 0x33f], 0
1417a5bbf: c6853e03000001  mov byte ptr [rbp + 0x33e], 1
1417a5bc6: 4883c428  add rsp, 0x28
1417a5bca: 5b  pop rbx
1417a5bcb: 5f  pop rdi
1417a5bcc: 5e  pop rsi
1417a5bcd: 415c  pop r12
1417a5bcf: 415d  pop r13
1417a5bd1: 415e  pop r14
1417a5bd3: 415f  pop r15
1417a5bd5: 5d  pop rbp
1417a5bd6: c3  ret
1417a5bd7: 660f1f840000000000  nop word ptr [rax + rax]
1417a5be0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5be5: 55  push rbp
1417a5be6: 4157  push r15
1417a5be8: 4156  push r14
1417a5bea: 4155  push r13
1417a5bec: 4154  push r12
1417a5bee: 56  push rsi
1417a5bef: 57  push rdi
1417a5bf0: 53  push rbx
1417a5bf1: 4883ec28  sub rsp, 0x28
1417a5bf5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a5bfc: ba06000000  mov edx, 6
1417a5c01: 41b801000000  mov r8d, 1
1417a5c07: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a5c0e: e80de0e6fe  call 0x140613c20
1417a5c13: 90  nop
1417a5c14: 4883c428  add rsp, 0x28
1417a5c18: 5b  pop rbx
1417a5c19: 5f  pop rdi
1417a5c1a: 5e  pop rsi
1417a5c1b: 415c  pop r12
1417a5c1d: 415d  pop r13
1417a5c1f: 415e  pop r14
1417a5c21: 415f  pop r15
1417a5c23: 5d  pop rbp
1417a5c24: c3  ret
1417a5c25: 66662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a5c30: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5c35: 55  push rbp
1417a5c36: 4157  push r15
1417a5c38: 4156  push r14
1417a5c3a: 4155  push r13
1417a5c3c: 4154  push r12
1417a5c3e: 56  push rsi
1417a5c3f: 57  push rdi
1417a5c40: 53  push rbx
1417a5c41: 4883ec28  sub rsp, 0x28
1417a5c45: 488daa80000000  lea rbp, [rdx + 0x80]
1417a5c4c: c6852103000001  mov byte ptr [rbp + 0x321], 1
1417a5c53: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a5c5a: e8412ce4fe  call 0x1405e88a0
1417a5c5f: c6852103000001  mov byte ptr [rbp + 0x321], 1
1417a5c66: 4883c428  add rsp, 0x28
1417a5c6a: 5b  pop rbx
1417a5c6b: 5f  pop rdi
1417a5c6c: 5e  pop rsi
1417a5c6d: 415c  pop r12
1417a5c6f: 415d  pop r13
1417a5c71: 415e  pop r14
1417a5c73: 415f  pop r15
1417a5c75: 5d  pop rbp
1417a5c76: c3  ret
1417a5c77: 660f1f840000000000  nop word ptr [rax + rax]
1417a5c80: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5c85: 55  push rbp
1417a5c86: 4157  push r15
1417a5c88: 4156  push r14
1417a5c8a: 4155  push r13
1417a5c8c: 4154  push r12
1417a5c8e: 56  push rsi
1417a5c8f: 57  push rdi
1417a5c90: 53  push rbx
1417a5c91: 4883ec28  sub rsp, 0x28
1417a5c95: 488daa80000000  lea rbp, [rdx + 0x80]
1417a5c9c: c6852203000001  mov byte ptr [rbp + 0x322], 1
1417a5ca3: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a5caa: e8f12be4fe  call 0x1405e88a0
1417a5caf: c6852203000001  mov byte ptr [rbp + 0x322], 1
1417a5cb6: 4883c428  add rsp, 0x28
1417a5cba: 5b  pop rbx
1417a5cbb: 5f  pop rdi
1417a5cbc: 5e  pop rsi
1417a5cbd: 415c  pop r12
1417a5cbf: 415d  pop r13
1417a5cc1: 415e  pop r14
1417a5cc3: 415f  pop r15
1417a5cc5: 5d  pop rbp
1417a5cc6: c3  ret
1417a5cc7: 660f1f840000000000  nop word ptr [rax + rax]
1417a5cd0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5cd5: 55  push rbp
1417a5cd6: 4157  push r15
1417a5cd8: 4156  push r14
1417a5cda: 4155  push r13
1417a5cdc: 4154  push r12
1417a5cde: 56  push rsi
1417a5cdf: 57  push rdi
1417a5ce0: 53  push rbx
1417a5ce1: 4883ec28  sub rsp, 0x28
1417a5ce5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a5cec: c6852303000001  mov byte ptr [rbp + 0x323], 1
1417a5cf3: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a5cfa: e8a12be4fe  call 0x1405e88a0
1417a5cff: c6852303000001  mov byte ptr [rbp + 0x323], 1
1417a5d06: 4883c428  add rsp, 0x28
1417a5d0a: 5b  pop rbx
1417a5d0b: 5f  pop rdi
1417a5d0c: 5e  pop rsi
1417a5d0d: 415c  pop r12
1417a5d0f: 415d  pop r13
1417a5d11: 415e  pop r14
1417a5d13: 415f  pop r15
1417a5d15: 5d  pop rbp
1417a5d16: c3  ret
1417a5d17: 660f1f840000000000  nop word ptr [rax + rax]
1417a5d20: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5d25: 55  push rbp
1417a5d26: 4157  push r15
1417a5d28: 4156  push r14
1417a5d2a: 4155  push r13
1417a5d2c: 4154  push r12
1417a5d2e: 56  push rsi
1417a5d2f: 57  push rdi
1417a5d30: 53  push rbx
1417a5d31: 4883ec28  sub rsp, 0x28
1417a5d35: 488daa80000000  lea rbp, [rdx + 0x80]
1417a5d3c: c6852403000001  mov byte ptr [rbp + 0x324], 1
1417a5d43: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a5d4a: e8512be4fe  call 0x1405e88a0
1417a5d4f: c6852403000001  mov byte ptr [rbp + 0x324], 1
1417a5d56: 4883c428  add rsp, 0x28
1417a5d5a: 5b  pop rbx
1417a5d5b: 5f  pop rdi
1417a5d5c: 5e  pop rsi
1417a5d5d: 415c  pop r12
1417a5d5f: 415d  pop r13
1417a5d61: 415e  pop r14
1417a5d63: 415f  pop r15
1417a5d65: 5d  pop rbp
1417a5d66: c3  ret
1417a5d67: 660f1f840000000000  nop word ptr [rax + rax]
1417a5d70: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5d75: 55  push rbp
1417a5d76: 4157  push r15
1417a5d78: 4156  push r14
1417a5d7a: 4155  push r13
1417a5d7c: 4154  push r12
1417a5d7e: 56  push rsi
1417a5d7f: 57  push rdi
1417a5d80: 53  push rbx
1417a5d81: 4883ec28  sub rsp, 0x28
1417a5d85: 488daa80000000  lea rbp, [rdx + 0x80]
1417a5d8c: c6852503000001  mov byte ptr [rbp + 0x325], 1
1417a5d93: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a5d9a: e8012be4fe  call 0x1405e88a0
1417a5d9f: c6852503000001  mov byte ptr [rbp + 0x325], 1
1417a5da6: 4883c428  add rsp, 0x28
1417a5daa: 5b  pop rbx
1417a5dab: 5f  pop rdi
1417a5dac: 5e  pop rsi
1417a5dad: 415c  pop r12
1417a5daf: 415d  pop r13
1417a5db1: 415e  pop r14
1417a5db3: 415f  pop r15
1417a5db5: 5d  pop rbp
1417a5db6: c3  ret
1417a5db7: 660f1f840000000000  nop word ptr [rax + rax]
1417a5dc0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5dc5: 55  push rbp
1417a5dc6: 4157  push r15
1417a5dc8: 4156  push r14
1417a5dca: 4155  push r13
1417a5dcc: 4154  push r12
1417a5dce: 56  push rsi
1417a5dcf: 57  push rdi
1417a5dd0: 53  push rbx
1417a5dd1: 4883ec28  sub rsp, 0x28
1417a5dd5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a5ddc: c6852603000001  mov byte ptr [rbp + 0x326], 1
1417a5de3: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a5dea: e8b12ae4fe  call 0x1405e88a0
1417a5def: c6852603000001  mov byte ptr [rbp + 0x326], 1
1417a5df6: 4883c428  add rsp, 0x28
1417a5dfa: 5b  pop rbx
1417a5dfb: 5f  pop rdi
1417a5dfc: 5e  pop rsi
1417a5dfd: 415c  pop r12
1417a5dff: 415d  pop r13
1417a5e01: 415e  pop r14
1417a5e03: 415f  pop r15
1417a5e05: 5d  pop rbp
1417a5e06: c3  ret
1417a5e07: 660f1f840000000000  nop word ptr [rax + rax]
1417a5e10: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5e15: 55  push rbp
1417a5e16: 4157  push r15
1417a5e18: 4156  push r14
1417a5e1a: 4155  push r13
1417a5e1c: 4154  push r12
1417a5e1e: 56  push rsi
1417a5e1f: 57  push rdi
1417a5e20: 53  push rbx
1417a5e21: 4883ec28  sub rsp, 0x28
1417a5e25: 488daa80000000  lea rbp, [rdx + 0x80]
1417a5e2c: c6852703000001  mov byte ptr [rbp + 0x327], 1
1417a5e33: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a5e3a: e8612ae4fe  call 0x1405e88a0
1417a5e3f: c6852703000001  mov byte ptr [rbp + 0x327], 1
1417a5e46: 4883c428  add rsp, 0x28
1417a5e4a: 5b  pop rbx
1417a5e4b: 5f  pop rdi
1417a5e4c: 5e  pop rsi
1417a5e4d: 415c  pop r12
1417a5e4f: 415d  pop r13
1417a5e51: 415e  pop r14
1417a5e53: 415f  pop r15
1417a5e55: 5d  pop rbp
1417a5e56: c3  ret
1417a5e57: 660f1f840000000000  nop word ptr [rax + rax]
1417a5e60: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5e65: 55  push rbp
1417a5e66: 4157  push r15
1417a5e68: 4156  push r14
1417a5e6a: 4155  push r13
1417a5e6c: 4154  push r12
1417a5e6e: 56  push rsi
1417a5e6f: 57  push rdi
1417a5e70: 53  push rbx
1417a5e71: 4883ec28  sub rsp, 0x28
1417a5e75: 488daa80000000  lea rbp, [rdx + 0x80]
1417a5e7c: c6852803000001  mov byte ptr [rbp + 0x328], 1
1417a5e83: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a5e8a: e8112ae4fe  call 0x1405e88a0
1417a5e8f: c6852803000001  mov byte ptr [rbp + 0x328], 1
1417a5e96: 4883c428  add rsp, 0x28
1417a5e9a: 5b  pop rbx
1417a5e9b: 5f  pop rdi
1417a5e9c: 5e  pop rsi
1417a5e9d: 415c  pop r12
1417a5e9f: 415d  pop r13
1417a5ea1: 415e  pop r14
1417a5ea3: 415f  pop r15
1417a5ea5: 5d  pop rbp
1417a5ea6: c3  ret
1417a5ea7: 660f1f840000000000  nop word ptr [rax + rax]
1417a5eb0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5eb5: 55  push rbp
1417a5eb6: 4157  push r15
1417a5eb8: 4156  push r14
1417a5eba: 4155  push r13
1417a5ebc: 4154  push r12
1417a5ebe: 56  push rsi
1417a5ebf: 57  push rdi
1417a5ec0: 53  push rbx
1417a5ec1: 4883ec28  sub rsp, 0x28
1417a5ec5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a5ecc: c6852903000001  mov byte ptr [rbp + 0x329], 1
1417a5ed3: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a5eda: e8c129e4fe  call 0x1405e88a0
1417a5edf: c6852903000001  mov byte ptr [rbp + 0x329], 1
1417a5ee6: 4883c428  add rsp, 0x28
1417a5eea: 5b  pop rbx
1417a5eeb: 5f  pop rdi
1417a5eec: 5e  pop rsi
1417a5eed: 415c  pop r12
1417a5eef: 415d  pop r13
1417a5ef1: 415e  pop r14
1417a5ef3: 415f  pop r15
1417a5ef5: 5d  pop rbp
1417a5ef6: c3  ret
1417a5ef7: 660f1f840000000000  nop word ptr [rax + rax]
1417a5f00: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5f05: 55  push rbp
1417a5f06: 4157  push r15
1417a5f08: 4156  push r14
1417a5f0a: 4155  push r13
1417a5f0c: 4154  push r12
1417a5f0e: 56  push rsi
1417a5f0f: 57  push rdi
1417a5f10: 53  push rbx
1417a5f11: 4883ec28  sub rsp, 0x28
1417a5f15: 488daa80000000  lea rbp, [rdx + 0x80]
1417a5f1c: c6852a03000001  mov byte ptr [rbp + 0x32a], 1
1417a5f23: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a5f2a: e87129e4fe  call 0x1405e88a0
1417a5f2f: c6852a03000001  mov byte ptr [rbp + 0x32a], 1
1417a5f36: 4883c428  add rsp, 0x28
1417a5f3a: 5b  pop rbx
1417a5f3b: 5f  pop rdi
1417a5f3c: 5e  pop rsi
1417a5f3d: 415c  pop r12
1417a5f3f: 415d  pop r13
1417a5f41: 415e  pop r14
1417a5f43: 415f  pop r15
1417a5f45: 5d  pop rbp
1417a5f46: c3  ret
1417a5f47: 660f1f840000000000  nop word ptr [rax + rax]
1417a5f50: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5f55: 55  push rbp
1417a5f56: 4157  push r15
1417a5f58: 4156  push r14
1417a5f5a: 4155  push r13
1417a5f5c: 4154  push r12
1417a5f5e: 56  push rsi
1417a5f5f: 57  push rdi
1417a5f60: 53  push rbx
1417a5f61: 4883ec28  sub rsp, 0x28
1417a5f65: 488daa80000000  lea rbp, [rdx + 0x80]
1417a5f6c: c6852b03000001  mov byte ptr [rbp + 0x32b], 1
1417a5f73: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a5f7a: e82129e4fe  call 0x1405e88a0
1417a5f7f: c6852b03000001  mov byte ptr [rbp + 0x32b], 1
1417a5f86: 4883c428  add rsp, 0x28
1417a5f8a: 5b  pop rbx
1417a5f8b: 5f  pop rdi
1417a5f8c: 5e  pop rsi
1417a5f8d: 415c  pop r12
1417a5f8f: 415d  pop r13
1417a5f91: 415e  pop r14
1417a5f93: 415f  pop r15
1417a5f95: 5d  pop rbp
1417a5f96: c3  ret
1417a5f97: 660f1f840000000000  nop word ptr [rax + rax]
1417a5fa0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5fa5: 55  push rbp
1417a5fa6: 4157  push r15
1417a5fa8: 4156  push r14
1417a5faa: 4155  push r13
1417a5fac: 4154  push r12
1417a5fae: 56  push rsi
1417a5faf: 57  push rdi
1417a5fb0: 53  push rbx
1417a5fb1: 4883ec28  sub rsp, 0x28
1417a5fb5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a5fbc: c6852c03000001  mov byte ptr [rbp + 0x32c], 1
1417a5fc3: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a5fca: e8d128e4fe  call 0x1405e88a0
1417a5fcf: c6852c03000001  mov byte ptr [rbp + 0x32c], 1
1417a5fd6: 4883c428  add rsp, 0x28
1417a5fda: 5b  pop rbx
1417a5fdb: 5f  pop rdi
1417a5fdc: 5e  pop rsi
1417a5fdd: 415c  pop r12
1417a5fdf: 415d  pop r13
1417a5fe1: 415e  pop r14
1417a5fe3: 415f  pop r15
1417a5fe5: 5d  pop rbp
1417a5fe6: c3  ret
1417a5fe7: 660f1f840000000000  nop word ptr [rax + rax]
1417a5ff0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a5ff5: 55  push rbp
1417a5ff6: 4157  push r15
1417a5ff8: 4156  push r14
1417a5ffa: 4155  push r13
1417a5ffc: 4154  push r12
1417a5ffe: 56  push rsi
1417a5fff: 57  push rdi
1417a6000: 53  push rbx
1417a6001: 4883ec28  sub rsp, 0x28
1417a6005: 488daa80000000  lea rbp, [rdx + 0x80]
1417a600c: c6852d03000001  mov byte ptr [rbp + 0x32d], 1
1417a6013: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a601a: e88128e4fe  call 0x1405e88a0
1417a601f: c6852d03000001  mov byte ptr [rbp + 0x32d], 1
1417a6026: 4883c428  add rsp, 0x28
1417a602a: 5b  pop rbx
1417a602b: 5f  pop rdi
1417a602c: 5e  pop rsi
1417a602d: 415c  pop r12
1417a602f: 415d  pop r13
1417a6031: 415e  pop r14
1417a6033: 415f  pop r15
1417a6035: 5d  pop rbp
1417a6036: c3  ret
1417a6037: 660f1f840000000000  nop word ptr [rax + rax]
1417a6040: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6045: 55  push rbp
1417a6046: 4157  push r15
1417a6048: 4156  push r14
1417a604a: 4155  push r13
1417a604c: 4154  push r12
1417a604e: 56  push rsi
1417a604f: 57  push rdi
1417a6050: 53  push rbx
1417a6051: 4883ec28  sub rsp, 0x28
1417a6055: 488daa80000000  lea rbp, [rdx + 0x80]
1417a605c: 488b8588020000  mov rax, qword ptr [rbp + 0x288]
1417a6063: 48c70000000000  mov qword ptr [rax], 0
1417a606a: 4883c428  add rsp, 0x28
1417a606e: 5b  pop rbx
1417a606f: 5f  pop rdi
1417a6070: 5e  pop rsi
1417a6071: 415c  pop r12
1417a6073: 415d  pop r13
1417a6075: 415e  pop r14
1417a6077: 415f  pop r15
1417a6079: 5d  pop rbp
1417a607a: c3  ret
1417a607b: 0f1f440000  nop dword ptr [rax + rax]
1417a6080: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6085: 55  push rbp
1417a6086: 4157  push r15
1417a6088: 4156  push r14
1417a608a: 4155  push r13
1417a608c: 4154  push r12
1417a608e: 56  push rsi
1417a608f: 57  push rdi
1417a6090: 53  push rbx
1417a6091: 4883ec28  sub rsp, 0x28
1417a6095: 488daa80000000  lea rbp, [rdx + 0x80]
1417a609c: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a60a3: e8e817e4fe  call 0x1405e7890
1417a60a8: 90  nop
1417a60a9: 4883c428  add rsp, 0x28
1417a60ad: 5b  pop rbx
1417a60ae: 5f  pop rdi
1417a60af: 5e  pop rsi
1417a60b0: 415c  pop r12
1417a60b2: 415d  pop r13
1417a60b4: 415e  pop r14
1417a60b6: 415f  pop r15
1417a60b8: 5d  pop rbp
1417a60b9: c3  ret
1417a60ba: 660f1f440000  nop word ptr [rax + rax]
1417a60c0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a60c5: 55  push rbp
1417a60c6: 4157  push r15
1417a60c8: 4156  push r14
1417a60ca: 4155  push r13
1417a60cc: 4154  push r12
1417a60ce: 56  push rsi
1417a60cf: 57  push rdi
1417a60d0: 53  push rbx
1417a60d1: 4883ec28  sub rsp, 0x28
1417a60d5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a60dc: c6853f03000001  mov byte ptr [rbp + 0x33f], 1
1417a60e3: c6853e03000001  mov byte ptr [rbp + 0x33e], 1
1417a60ea: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a60f1: e8fa4a0dff  call 0x14087abf0
1417a60f6: c6853f03000001  mov byte ptr [rbp + 0x33f], 1
1417a60fd: c6853e03000001  mov byte ptr [rbp + 0x33e], 1
1417a6104: 4883c428  add rsp, 0x28
1417a6108: 5b  pop rbx
1417a6109: 5f  pop rdi
1417a610a: 5e  pop rsi
1417a610b: 415c  pop r12
1417a610d: 415d  pop r13
1417a610f: 415e  pop r14
1417a6111: 415f  pop r15
1417a6113: 5d  pop rbp
1417a6114: c3  ret
1417a6115: 66662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a6120: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6125: 55  push rbp
1417a6126: 4157  push r15
1417a6128: 4156  push r14
1417a612a: 4155  push r13
1417a612c: 4154  push r12
1417a612e: 56  push rsi
1417a612f: 57  push rdi
1417a6130: 53  push rbx
1417a6131: 4883ec28  sub rsp, 0x28
1417a6135: 488daa80000000  lea rbp, [rdx + 0x80]
1417a613c: 80bd2103000000  cmp byte ptr [rbp + 0x321], 0
1417a6143: 7430  je 0x1417a6175
1417a6145: 488b9550020000  mov rdx, qword ptr [rbp + 0x250]
1417a614c: 4885d2  test rdx, rdx
1417a614f: 7412  je 0x1417a6163
1417a6151: 488b8d58020000  mov rcx, qword ptr [rbp + 0x258]
1417a6158: 41b801000000  mov r8d, 1
1417a615e: e8bddae6fe  call 0x140613c20
1417a6163: 90  nop
1417a6164: 4883c428  add rsp, 0x28
1417a6168: 5b  pop rbx
1417a6169: 5f  pop rdi
1417a616a: 5e  pop rsi
1417a616b: 415c  pop r12
1417a616d: 415d  pop r13
1417a616f: 415e  pop r14
1417a6171: 415f  pop r15
1417a6173: 5d  pop rbp
1417a6174: c3  ret
1417a6175: 4883c428  add rsp, 0x28
1417a6179: 5b  pop rbx
1417a617a: 5f  pop rdi
1417a617b: 5e  pop rsi
1417a617c: 415c  pop r12
1417a617e: 415d  pop r13
1417a6180: 415e  pop r14
1417a6182: 415f  pop r15
1417a6184: 5d  pop rbp
1417a6185: c3  ret
1417a6186: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a6190: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6195: 55  push rbp
1417a6196: 4157  push r15
1417a6198: 4156  push r14
1417a619a: 4155  push r13
1417a619c: 4154  push r12
1417a619e: 56  push rsi
1417a619f: 57  push rdi
1417a61a0: 53  push rbx
1417a61a1: 4883ec28  sub rsp, 0x28
1417a61a5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a61ac: 80bd2203000000  cmp byte ptr [rbp + 0x322], 0
1417a61b3: 7430  je 0x1417a61e5
1417a61b5: 488b9550020000  mov rdx, qword ptr [rbp + 0x250]
1417a61bc: 4885d2  test rdx, rdx
1417a61bf: 7412  je 0x1417a61d3
1417a61c1: 488b8d58020000  mov rcx, qword ptr [rbp + 0x258]
1417a61c8: 41b801000000  mov r8d, 1
1417a61ce: e84ddae6fe  call 0x140613c20
1417a61d3: 90  nop
1417a61d4: 4883c428  add rsp, 0x28
1417a61d8: 5b  pop rbx
1417a61d9: 5f  pop rdi
1417a61da: 5e  pop rsi
1417a61db: 415c  pop r12
1417a61dd: 415d  pop r13
1417a61df: 415e  pop r14
1417a61e1: 415f  pop r15
1417a61e3: 5d  pop rbp
1417a61e4: c3  ret
1417a61e5: 4883c428  add rsp, 0x28
1417a61e9: 5b  pop rbx
1417a61ea: 5f  pop rdi
1417a61eb: 5e  pop rsi
1417a61ec: 415c  pop r12
1417a61ee: 415d  pop r13
1417a61f0: 415e  pop r14
1417a61f2: 415f  pop r15
1417a61f4: 5d  pop rbp
1417a61f5: c3  ret
1417a61f6: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a6200: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6205: 55  push rbp
1417a6206: 4157  push r15
1417a6208: 4156  push r14
1417a620a: 4155  push r13
1417a620c: 4154  push r12
1417a620e: 56  push rsi
1417a620f: 57  push rdi
1417a6210: 53  push rbx
1417a6211: 4883ec28  sub rsp, 0x28
1417a6215: 488daa80000000  lea rbp, [rdx + 0x80]
1417a621c: 80bd2303000000  cmp byte ptr [rbp + 0x323], 0
1417a6223: 7430  je 0x1417a6255
1417a6225: 488b9550020000  mov rdx, qword ptr [rbp + 0x250]
1417a622c: 4885d2  test rdx, rdx
1417a622f: 7412  je 0x1417a6243
1417a6231: 488b8d58020000  mov rcx, qword ptr [rbp + 0x258]
1417a6238: 41b801000000  mov r8d, 1
1417a623e: e8ddd9e6fe  call 0x140613c20
1417a6243: 90  nop
1417a6244: 4883c428  add rsp, 0x28
1417a6248: 5b  pop rbx
1417a6249: 5f  pop rdi
1417a624a: 5e  pop rsi
1417a624b: 415c  pop r12
1417a624d: 415d  pop r13
1417a624f: 415e  pop r14
1417a6251: 415f  pop r15
1417a6253: 5d  pop rbp
1417a6254: c3  ret
1417a6255: 4883c428  add rsp, 0x28
1417a6259: 5b  pop rbx
1417a625a: 5f  pop rdi
1417a625b: 5e  pop rsi
1417a625c: 415c  pop r12
1417a625e: 415d  pop r13
1417a6260: 415e  pop r14
1417a6262: 415f  pop r15
1417a6264: 5d  pop rbp
1417a6265: c3  ret
1417a6266: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a6270: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6275: 55  push rbp
1417a6276: 4157  push r15
1417a6278: 4156  push r14
1417a627a: 4155  push r13
1417a627c: 4154  push r12
1417a627e: 56  push rsi
1417a627f: 57  push rdi
1417a6280: 53  push rbx
1417a6281: 4883ec28  sub rsp, 0x28
1417a6285: 488daa80000000  lea rbp, [rdx + 0x80]
1417a628c: 80bd2403000000  cmp byte ptr [rbp + 0x324], 0
1417a6293: 7430  je 0x1417a62c5
1417a6295: 488b9550020000  mov rdx, qword ptr [rbp + 0x250]
1417a629c: 4885d2  test rdx, rdx
1417a629f: 7412  je 0x1417a62b3
1417a62a1: 488b8d58020000  mov rcx, qword ptr [rbp + 0x258]
1417a62a8: 41b801000000  mov r8d, 1
1417a62ae: e86dd9e6fe  call 0x140613c20
1417a62b3: 90  nop
1417a62b4: 4883c428  add rsp, 0x28
1417a62b8: 5b  pop rbx
1417a62b9: 5f  pop rdi
1417a62ba: 5e  pop rsi
1417a62bb: 415c  pop r12
1417a62bd: 415d  pop r13
1417a62bf: 415e  pop r14
1417a62c1: 415f  pop r15
1417a62c3: 5d  pop rbp
1417a62c4: c3  ret
1417a62c5: 4883c428  add rsp, 0x28
1417a62c9: 5b  pop rbx
1417a62ca: 5f  pop rdi
1417a62cb: 5e  pop rsi
1417a62cc: 415c  pop r12
1417a62ce: 415d  pop r13
1417a62d0: 415e  pop r14
1417a62d2: 415f  pop r15
1417a62d4: 5d  pop rbp
1417a62d5: c3  ret
1417a62d6: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a62e0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a62e5: 55  push rbp
1417a62e6: 4157  push r15
1417a62e8: 4156  push r14
1417a62ea: 4155  push r13
1417a62ec: 4154  push r12
1417a62ee: 56  push rsi
1417a62ef: 57  push rdi
1417a62f0: 53  push rbx
1417a62f1: 4883ec28  sub rsp, 0x28
1417a62f5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a62fc: 80bd2503000000  cmp byte ptr [rbp + 0x325], 0
1417a6303: 7430  je 0x1417a6335
1417a6305: 488b9550020000  mov rdx, qword ptr [rbp + 0x250]
1417a630c: 4885d2  test rdx, rdx
1417a630f: 7412  je 0x1417a6323
1417a6311: 488b8d58020000  mov rcx, qword ptr [rbp + 0x258]
1417a6318: 41b801000000  mov r8d, 1
1417a631e: e8fdd8e6fe  call 0x140613c20
1417a6323: 90  nop
1417a6324: 4883c428  add rsp, 0x28
1417a6328: 5b  pop rbx
1417a6329: 5f  pop rdi
1417a632a: 5e  pop rsi
1417a632b: 415c  pop r12
1417a632d: 415d  pop r13
1417a632f: 415e  pop r14
1417a6331: 415f  pop r15
1417a6333: 5d  pop rbp
1417a6334: c3  ret
1417a6335: 4883c428  add rsp, 0x28
1417a6339: 5b  pop rbx
1417a633a: 5f  pop rdi
1417a633b: 5e  pop rsi
1417a633c: 415c  pop r12
1417a633e: 415d  pop r13
1417a6340: 415e  pop r14
1417a6342: 415f  pop r15
1417a6344: 5d  pop rbp
1417a6345: c3  ret
1417a6346: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a6350: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6355: 55  push rbp
1417a6356: 4157  push r15
1417a6358: 4156  push r14
1417a635a: 4155  push r13
1417a635c: 4154  push r12
1417a635e: 56  push rsi
1417a635f: 57  push rdi
1417a6360: 53  push rbx
1417a6361: 4883ec28  sub rsp, 0x28
1417a6365: 488daa80000000  lea rbp, [rdx + 0x80]
1417a636c: 80bd2603000000  cmp byte ptr [rbp + 0x326], 0
1417a6373: 7430  je 0x1417a63a5
1417a6375: 488b9550020000  mov rdx, qword ptr [rbp + 0x250]
1417a637c: 4885d2  test rdx, rdx
1417a637f: 7412  je 0x1417a6393
1417a6381: 488b8d58020000  mov rcx, qword ptr [rbp + 0x258]
1417a6388: 41b801000000  mov r8d, 1
1417a638e: e88dd8e6fe  call 0x140613c20
1417a6393: 90  nop
1417a6394: 4883c428  add rsp, 0x28
1417a6398: 5b  pop rbx
1417a6399: 5f  pop rdi
1417a639a: 5e  pop rsi
1417a639b: 415c  pop r12
1417a639d: 415d  pop r13
1417a639f: 415e  pop r14
1417a63a1: 415f  pop r15
1417a63a3: 5d  pop rbp
1417a63a4: c3  ret
1417a63a5: 4883c428  add rsp, 0x28
1417a63a9: 5b  pop rbx
1417a63aa: 5f  pop rdi
1417a63ab: 5e  pop rsi
1417a63ac: 415c  pop r12
1417a63ae: 415d  pop r13
1417a63b0: 415e  pop r14
1417a63b2: 415f  pop r15
1417a63b4: 5d  pop rbp
1417a63b5: c3  ret
1417a63b6: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a63c0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a63c5: 55  push rbp
1417a63c6: 4157  push r15
1417a63c8: 4156  push r14
1417a63ca: 4155  push r13
1417a63cc: 4154  push r12
1417a63ce: 56  push rsi
1417a63cf: 57  push rdi
1417a63d0: 53  push rbx
1417a63d1: 4883ec28  sub rsp, 0x28
1417a63d5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a63dc: 80bd2703000000  cmp byte ptr [rbp + 0x327], 0
1417a63e3: 7430  je 0x1417a6415
1417a63e5: 488b9550020000  mov rdx, qword ptr [rbp + 0x250]
1417a63ec: 4885d2  test rdx, rdx
1417a63ef: 7412  je 0x1417a6403
1417a63f1: 488b8d58020000  mov rcx, qword ptr [rbp + 0x258]
1417a63f8: 41b801000000  mov r8d, 1
1417a63fe: e81dd8e6fe  call 0x140613c20
1417a6403: 90  nop
1417a6404: 4883c428  add rsp, 0x28
1417a6408: 5b  pop rbx
1417a6409: 5f  pop rdi
1417a640a: 5e  pop rsi
1417a640b: 415c  pop r12
1417a640d: 415d  pop r13
1417a640f: 415e  pop r14
1417a6411: 415f  pop r15
1417a6413: 5d  pop rbp
1417a6414: c3  ret
1417a6415: 4883c428  add rsp, 0x28
1417a6419: 5b  pop rbx
1417a641a: 5f  pop rdi
1417a641b: 5e  pop rsi
1417a641c: 415c  pop r12
1417a641e: 415d  pop r13
1417a6420: 415e  pop r14
1417a6422: 415f  pop r15
1417a6424: 5d  pop rbp
1417a6425: c3  ret
1417a6426: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a6430: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6435: 55  push rbp
1417a6436: 4157  push r15
1417a6438: 4156  push r14
1417a643a: 4155  push r13
1417a643c: 4154  push r12
1417a643e: 56  push rsi
1417a643f: 57  push rdi
1417a6440: 53  push rbx
1417a6441: 4883ec28  sub rsp, 0x28
1417a6445: 488daa80000000  lea rbp, [rdx + 0x80]
1417a644c: 80bd2803000000  cmp byte ptr [rbp + 0x328], 0
1417a6453: 7430  je 0x1417a6485
1417a6455: 488b9550020000  mov rdx, qword ptr [rbp + 0x250]
1417a645c: 4885d2  test rdx, rdx
1417a645f: 7412  je 0x1417a6473
1417a6461: 488b8d58020000  mov rcx, qword ptr [rbp + 0x258]
1417a6468: 41b801000000  mov r8d, 1
1417a646e: e8add7e6fe  call 0x140613c20
1417a6473: 90  nop
1417a6474: 4883c428  add rsp, 0x28
1417a6478: 5b  pop rbx
1417a6479: 5f  pop rdi
1417a647a: 5e  pop rsi
1417a647b: 415c  pop r12
1417a647d: 415d  pop r13
1417a647f: 415e  pop r14
1417a6481: 415f  pop r15
1417a6483: 5d  pop rbp
1417a6484: c3  ret
1417a6485: 4883c428  add rsp, 0x28
1417a6489: 5b  pop rbx
1417a648a: 5f  pop rdi
1417a648b: 5e  pop rsi
1417a648c: 415c  pop r12
1417a648e: 415d  pop r13
1417a6490: 415e  pop r14
1417a6492: 415f  pop r15
1417a6494: 5d  pop rbp
1417a6495: c3  ret
1417a6496: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a64a0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a64a5: 55  push rbp
1417a64a6: 4157  push r15
1417a64a8: 4156  push r14
1417a64aa: 4155  push r13
1417a64ac: 4154  push r12
1417a64ae: 56  push rsi
1417a64af: 57  push rdi
1417a64b0: 53  push rbx
1417a64b1: 4883ec28  sub rsp, 0x28
1417a64b5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a64bc: 80bd2903000000  cmp byte ptr [rbp + 0x329], 0
1417a64c3: 7430  je 0x1417a64f5
1417a64c5: 488b9550020000  mov rdx, qword ptr [rbp + 0x250]
1417a64cc: 4885d2  test rdx, rdx
1417a64cf: 7412  je 0x1417a64e3
1417a64d1: 488b8d58020000  mov rcx, qword ptr [rbp + 0x258]
1417a64d8: 41b801000000  mov r8d, 1
1417a64de: e83dd7e6fe  call 0x140613c20
1417a64e3: 90  nop
1417a64e4: 4883c428  add rsp, 0x28
1417a64e8: 5b  pop rbx
1417a64e9: 5f  pop rdi
1417a64ea: 5e  pop rsi
1417a64eb: 415c  pop r12
1417a64ed: 415d  pop r13
1417a64ef: 415e  pop r14
1417a64f1: 415f  pop r15
1417a64f3: 5d  pop rbp
1417a64f4: c3  ret
1417a64f5: 4883c428  add rsp, 0x28
1417a64f9: 5b  pop rbx
1417a64fa: 5f  pop rdi
1417a64fb: 5e  pop rsi
1417a64fc: 415c  pop r12
1417a64fe: 415d  pop r13
1417a6500: 415e  pop r14
1417a6502: 415f  pop r15
1417a6504: 5d  pop rbp
1417a6505: c3  ret
1417a6506: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a6510: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6515: 55  push rbp
1417a6516: 4157  push r15
1417a6518: 4156  push r14
1417a651a: 4155  push r13
1417a651c: 4154  push r12
1417a651e: 56  push rsi
1417a651f: 57  push rdi
1417a6520: 53  push rbx
1417a6521: 4883ec28  sub rsp, 0x28
1417a6525: 488daa80000000  lea rbp, [rdx + 0x80]
1417a652c: 80bd2a03000000  cmp byte ptr [rbp + 0x32a], 0
1417a6533: 7430  je 0x1417a6565
1417a6535: 488b95d8010000  mov rdx, qword ptr [rbp + 0x1d8]
1417a653c: 4885d2  test rdx, rdx
1417a653f: 7412  je 0x1417a6553
1417a6541: 488b8de0010000  mov rcx, qword ptr [rbp + 0x1e0]
1417a6548: 41b801000000  mov r8d, 1
1417a654e: e8cdd6e6fe  call 0x140613c20
1417a6553: 90  nop
1417a6554: 4883c428  add rsp, 0x28
1417a6558: 5b  pop rbx
1417a6559: 5f  pop rdi
1417a655a: 5e  pop rsi
1417a655b: 415c  pop r12
1417a655d: 415d  pop r13
1417a655f: 415e  pop r14
1417a6561: 415f  pop r15
1417a6563: 5d  pop rbp
1417a6564: c3  ret
1417a6565: 4883c428  add rsp, 0x28
1417a6569: 5b  pop rbx
1417a656a: 5f  pop rdi
1417a656b: 5e  pop rsi
1417a656c: 415c  pop r12
1417a656e: 415d  pop r13
1417a6570: 415e  pop r14
1417a6572: 415f  pop r15
1417a6574: 5d  pop rbp
1417a6575: c3  ret
1417a6576: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a6580: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6585: 55  push rbp
1417a6586: 4157  push r15
1417a6588: 4156  push r14
1417a658a: 4155  push r13
1417a658c: 4154  push r12
1417a658e: 56  push rsi
1417a658f: 57  push rdi
1417a6590: 53  push rbx
1417a6591: 4883ec28  sub rsp, 0x28
1417a6595: 488daa80000000  lea rbp, [rdx + 0x80]
1417a659c: 80bd2b03000000  cmp byte ptr [rbp + 0x32b], 0
1417a65a3: 7430  je 0x1417a65d5
1417a65a5: 488b9550020000  mov rdx, qword ptr [rbp + 0x250]
1417a65ac: 4885d2  test rdx, rdx
1417a65af: 7412  je 0x1417a65c3
1417a65b1: 488b8d58020000  mov rcx, qword ptr [rbp + 0x258]
1417a65b8: 41b801000000  mov r8d, 1
1417a65be: e85dd6e6fe  call 0x140613c20
1417a65c3: 90  nop
1417a65c4: 4883c428  add rsp, 0x28
1417a65c8: 5b  pop rbx
1417a65c9: 5f  pop rdi
1417a65ca: 5e  pop rsi
1417a65cb: 415c  pop r12
1417a65cd: 415d  pop r13
1417a65cf: 415e  pop r14
1417a65d1: 415f  pop r15
1417a65d3: 5d  pop rbp
1417a65d4: c3  ret
1417a65d5: 4883c428  add rsp, 0x28
1417a65d9: 5b  pop rbx
1417a65da: 5f  pop rdi
1417a65db: 5e  pop rsi
1417a65dc: 415c  pop r12
1417a65de: 415d  pop r13
1417a65e0: 415e  pop r14
1417a65e2: 415f  pop r15
1417a65e4: 5d  pop rbp
1417a65e5: c3  ret
1417a65e6: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a65f0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a65f5: 55  push rbp
1417a65f6: 4157  push r15
1417a65f8: 4156  push r14
1417a65fa: 4155  push r13
1417a65fc: 4154  push r12
1417a65fe: 56  push rsi
1417a65ff: 57  push rdi
1417a6600: 53  push rbx
1417a6601: 4883ec28  sub rsp, 0x28
1417a6605: 488daa80000000  lea rbp, [rdx + 0x80]
1417a660c: 80bd2c03000000  cmp byte ptr [rbp + 0x32c], 0
1417a6613: 7430  je 0x1417a6645
1417a6615: 488b9550020000  mov rdx, qword ptr [rbp + 0x250]
1417a661c: 4885d2  test rdx, rdx
1417a661f: 7412  je 0x1417a6633
1417a6621: 488b8d58020000  mov rcx, qword ptr [rbp + 0x258]
1417a6628: 41b801000000  mov r8d, 1
1417a662e: e8edd5e6fe  call 0x140613c20
1417a6633: 90  nop
1417a6634: 4883c428  add rsp, 0x28
1417a6638: 5b  pop rbx
1417a6639: 5f  pop rdi
1417a663a: 5e  pop rsi
1417a663b: 415c  pop r12
1417a663d: 415d  pop r13
1417a663f: 415e  pop r14
1417a6641: 415f  pop r15
1417a6643: 5d  pop rbp
1417a6644: c3  ret
1417a6645: 4883c428  add rsp, 0x28
1417a6649: 5b  pop rbx
1417a664a: 5f  pop rdi
1417a664b: 5e  pop rsi
1417a664c: 415c  pop r12
1417a664e: 415d  pop r13
1417a6650: 415e  pop r14
1417a6652: 415f  pop r15
1417a6654: 5d  pop rbp
1417a6655: c3  ret
1417a6656: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a6660: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6665: 55  push rbp
1417a6666: 4157  push r15
1417a6668: 4156  push r14
1417a666a: 4155  push r13
1417a666c: 4154  push r12
1417a666e: 56  push rsi
1417a666f: 57  push rdi
1417a6670: 53  push rbx
1417a6671: 4883ec28  sub rsp, 0x28
1417a6675: 488daa80000000  lea rbp, [rdx + 0x80]
1417a667c: 80bd2d03000000  cmp byte ptr [rbp + 0x32d], 0
1417a6683: 7430  je 0x1417a66b5
1417a6685: 488b9550020000  mov rdx, qword ptr [rbp + 0x250]
1417a668c: 4885d2  test rdx, rdx
1417a668f: 7412  je 0x1417a66a3
1417a6691: 488b8d58020000  mov rcx, qword ptr [rbp + 0x258]
1417a6698: 41b801000000  mov r8d, 1
1417a669e: e87dd5e6fe  call 0x140613c20
1417a66a3: 90  nop
1417a66a4: 4883c428  add rsp, 0x28
1417a66a8: 5b  pop rbx
1417a66a9: 5f  pop rdi
1417a66aa: 5e  pop rsi
1417a66ab: 415c  pop r12
1417a66ad: 415d  pop r13
1417a66af: 415e  pop r14
1417a66b1: 415f  pop r15
1417a66b3: 5d  pop rbp
1417a66b4: c3  ret
1417a66b5: 4883c428  add rsp, 0x28
1417a66b9: 5b  pop rbx
1417a66ba: 5f  pop rdi
1417a66bb: 5e  pop rsi
1417a66bc: 415c  pop r12
1417a66be: 415d  pop r13
1417a66c0: 415e  pop r14
1417a66c2: 415f  pop r15
1417a66c4: 5d  pop rbp
1417a66c5: c3  ret
1417a66c6: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a66d0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a66d5: 55  push rbp
1417a66d6: 4157  push r15
1417a66d8: 4156  push r14
1417a66da: 4155  push r13
1417a66dc: 4154  push r12
1417a66de: 56  push rsi
1417a66df: 57  push rdi
1417a66e0: 53  push rbx
1417a66e1: 4883ec28  sub rsp, 0x28
1417a66e5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a66ec: c6852f03000000  mov byte ptr [rbp + 0x32f], 0
1417a66f3: 488d8dd0020000  lea rcx, [rbp + 0x2d0]
1417a66fa: e891b10dff  call 0x140881890
1417a66ff: c6852f03000000  mov byte ptr [rbp + 0x32f], 0
1417a6706: 4883c428  add rsp, 0x28
1417a670a: 5b  pop rbx
1417a670b: 5f  pop rdi
1417a670c: 5e  pop rsi
1417a670d: 415c  pop r12
1417a670f: 415d  pop r13
1417a6711: 415e  pop r14
1417a6713: 415f  pop r15
1417a6715: 5d  pop rbp
1417a6716: c3  ret
1417a6717: 660f1f840000000000  nop word ptr [rax + rax]
1417a6720: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6725: 55  push rbp
1417a6726: 4157  push r15
1417a6728: 4156  push r14
1417a672a: 4155  push r13
1417a672c: 4154  push r12
1417a672e: 56  push rsi
1417a672f: 57  push rdi
1417a6730: 53  push rbx
1417a6731: 4883ec28  sub rsp, 0x28
1417a6735: 488daa80000000  lea rbp, [rdx + 0x80]
1417a673c: 488b9550020000  mov rdx, qword ptr [rbp + 0x250]
1417a6743: 4885d2  test rdx, rdx
1417a6746: 7416  je 0x1417a675e
1417a6748: 488b8d58020000  mov rcx, qword ptr [rbp + 0x258]
1417a674f: 48c1e204  shl rdx, 4
1417a6753: 41b808000000  mov r8d, 8
1417a6759: e8c2d4e6fe  call 0x140613c20
1417a675e: 90  nop
1417a675f: 4883c428  add rsp, 0x28
1417a6763: 5b  pop rbx
1417a6764: 5f  pop rdi
1417a6765: 5e  pop rsi
1417a6766: 415c  pop r12
1417a6768: 415d  pop r13
1417a676a: 415e  pop r14
1417a676c: 415f  pop r15
1417a676e: 5d  pop rbp
1417a676f: c3  ret
1417a6770: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6775: 55  push rbp
1417a6776: 4157  push r15
1417a6778: 4156  push r14
1417a677a: 4155  push r13
1417a677c: 4154  push r12
1417a677e: 56  push rsi
1417a677f: 57  push rdi
1417a6780: 53  push rbx
1417a6781: 4883ec28  sub rsp, 0x28
1417a6785: 488daa80000000  lea rbp, [rdx + 0x80]
1417a678c: 80bd2f03000000  cmp byte ptr [rbp + 0x32f], 0
1417a6793: 741e  je 0x1417a67b3
1417a6795: 488d8d90020000  lea rcx, [rbp + 0x290]
1417a679c: e8efb00dff  call 0x140881890
1417a67a1: 90  nop
1417a67a2: 4883c428  add rsp, 0x28
1417a67a6: 5b  pop rbx
1417a67a7: 5f  pop rdi
1417a67a8: 5e  pop rsi
1417a67a9: 415c  pop r12
1417a67ab: 415d  pop r13
1417a67ad: 415e  pop r14
1417a67af: 415f  pop r15
1417a67b1: 5d  pop rbp
1417a67b2: c3  ret
1417a67b3: 4883c428  add rsp, 0x28
1417a67b7: 5b  pop rbx
1417a67b8: 5f  pop rdi
1417a67b9: 5e  pop rsi
1417a67ba: 415c  pop r12
1417a67bc: 415d  pop r13
1417a67be: 415e  pop r14
1417a67c0: 415f  pop r15
1417a67c2: 5d  pop rbp
1417a67c3: c3  ret
1417a67c4: 6666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a67d0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a67d5: 55  push rbp
1417a67d6: 4157  push r15
1417a67d8: 4156  push r14
1417a67da: 4155  push r13
1417a67dc: 4154  push r12
1417a67de: 56  push rsi
1417a67df: 57  push rdi
1417a67e0: 53  push rbx
1417a67e1: 4883ec28  sub rsp, 0x28
1417a67e5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a67ec: 488d8d28020000  lea rcx, [rbp + 0x228]
1417a67f3: e8f8430dff  call 0x14087abf0
1417a67f8: 90  nop
1417a67f9: 4883c428  add rsp, 0x28
1417a67fd: 5b  pop rbx
1417a67fe: 5f  pop rdi
1417a67ff: 5e  pop rsi
1417a6800: 415c  pop r12
1417a6802: 415d  pop r13
1417a6804: 415e  pop r14
1417a6806: 415f  pop r15
1417a6808: 5d  pop rbp
1417a6809: c3  ret
1417a680a: 660f1f440000  nop word ptr [rax + rax]
1417a6810: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6815: 55  push rbp
1417a6816: 4157  push r15
1417a6818: 4156  push r14
1417a681a: 4155  push r13
1417a681c: 4154  push r12
1417a681e: 56  push rsi
1417a681f: 57  push rdi
1417a6820: 53  push rbx
1417a6821: 4883ec28  sub rsp, 0x28
1417a6825: 488daa80000000  lea rbp, [rdx + 0x80]
1417a682c: 4883bd4002000000  cmp qword ptr [rbp + 0x240], 0
1417a6834: 7419  je 0x1417a684f
1417a6836: 41b801000000  mov r8d, 1
1417a683c: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a6843: 488b9540020000  mov rdx, qword ptr [rbp + 0x240]
1417a684a: e8d1d3e6fe  call 0x140613c20
1417a684f: 488d8dd8010000  lea rcx, [rbp + 0x1d8]
1417a6856: e8a5030dff  call 0x140876c00
1417a685b: c6853f03000000  mov byte ptr [rbp + 0x33f], 0
1417a6862: c6853e03000001  mov byte ptr [rbp + 0x33e], 1
1417a6869: 4883c428  add rsp, 0x28
1417a686d: 5b  pop rbx
1417a686e: 5f  pop rdi
1417a686f: 5e  pop rsi
1417a6870: 415c  pop r12
1417a6872: 415d  pop r13
1417a6874: 415e  pop r14
1417a6876: 415f  pop r15
1417a6878: 5d  pop rbp
1417a6879: c3  ret
1417a687a: 660f1f440000  nop word ptr [rax + rax]
1417a6880: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6885: 55  push rbp
1417a6886: 4157  push r15
1417a6888: 4156  push r14
1417a688a: 4155  push r13
1417a688c: 4154  push r12
1417a688e: 56  push rsi
1417a688f: 57  push rdi
1417a6890: 53  push rbx
1417a6891: 4883ec28  sub rsp, 0x28
1417a6895: 488daa80000000  lea rbp, [rdx + 0x80]
1417a689c: 0fb69d3e030000  movzx ebx, byte ptr [rbp + 0x33e]
1417a68a3: 0fb6b53f030000  movzx esi, byte ptr [rbp + 0x33f]
1417a68aa: 488d8d00020000  lea rcx, [rbp + 0x200]
1417a68b1: e8da710bff  call 0x14085da90
1417a68b6: 4088b53d030000  mov byte ptr [rbp + 0x33d], sil
1417a68bd: 889d3c030000  mov byte ptr [rbp + 0x33c], bl
1417a68c3: c6853b03000001  mov byte ptr [rbp + 0x33b], 1
1417a68ca: 4883c428  add rsp, 0x28
1417a68ce: 5b  pop rbx
1417a68cf: 5f  pop rdi
1417a68d0: 5e  pop rsi
1417a68d1: 415c  pop r12
1417a68d3: 415d  pop r13
1417a68d5: 415e  pop r14
1417a68d7: 415f  pop r15
1417a68d9: 5d  pop rbp
1417a68da: c3  ret
1417a68db: 0f1f440000  nop dword ptr [rax + rax]
1417a68e0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a68e5: 55  push rbp
1417a68e6: 4157  push r15
1417a68e8: 4156  push r14
1417a68ea: 4155  push r13
1417a68ec: 4154  push r12
1417a68ee: 56  push rsi
1417a68ef: 57  push rdi
1417a68f0: 53  push rbx
1417a68f1: 4883ec28  sub rsp, 0x28
1417a68f5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a68fc: 488d8d28020000  lea rcx, [rbp + 0x228]
1417a6903: e838260fff  call 0x140898f40
1417a6908: c6853d03000001  mov byte ptr [rbp + 0x33d], 1
1417a690f: c6853c03000001  mov byte ptr [rbp + 0x33c], 1
1417a6916: c6853b03000001  mov byte ptr [rbp + 0x33b], 1
1417a691d: 4883c428  add rsp, 0x28
1417a6921: 5b  pop rbx
1417a6922: 5f  pop rdi
1417a6923: 5e  pop rsi
1417a6924: 415c  pop r12
1417a6926: 415d  pop r13
1417a6928: 415e  pop r14
1417a692a: 415f  pop r15
1417a692c: 5d  pop rbp
1417a692d: c3  ret
1417a692e: 6690  nop
1417a6930: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6935: 55  push rbp
1417a6936: 4157  push r15
1417a6938: 4156  push r14
1417a693a: 4155  push r13
1417a693c: 4154  push r12
1417a693e: 56  push rsi
1417a693f: 57  push rdi
1417a6940: 53  push rbx
1417a6941: 4883ec28  sub rsp, 0x28
1417a6945: 488daa80000000  lea rbp, [rdx + 0x80]
1417a694c: 0fb69d3b030000  movzx ebx, byte ptr [rbp + 0x33b]
1417a6953: 0fb6b53d030000  movzx esi, byte ptr [rbp + 0x33d]
1417a695a: f6853c03000001  test byte ptr [rbp + 0x33c], 1
1417a6961: 7437  je 0x1417a699a
1417a6963: 4883bd8000000000  cmp qword ptr [rbp + 0x80], 0
1417a696b: 740c  je 0x1417a6979
1417a696d: 488d8d80000000  lea rcx, [rbp + 0x80]
1417a6974: e88710e4fe  call 0x1405e7a00
1417a6979: 4088b51f030000  mov byte ptr [rbp + 0x31f], sil
1417a6980: 80e301  and bl, 1
1417a6983: 889d1e030000  mov byte ptr [rbp + 0x31e], bl
1417a6989: 4883c428  add rsp, 0x28
1417a698d: 5b  pop rbx
1417a698e: 5f  pop rdi
1417a698f: 5e  pop rsi
1417a6990: 415c  pop r12
1417a6992: 415d  pop r13
1417a6994: 415e  pop r14
1417a6996: 415f  pop r15
1417a6998: 5d  pop rbp
1417a6999: c3  ret
1417a699a: 4088b51f030000  mov byte ptr [rbp + 0x31f], sil
1417a69a1: 80e301  and bl, 1
1417a69a4: 889d1e030000  mov byte ptr [rbp + 0x31e], bl
1417a69aa: 4883c428  add rsp, 0x28
1417a69ae: 5b  pop rbx
1417a69af: 5f  pop rdi
1417a69b0: 5e  pop rsi
1417a69b1: 415c  pop r12
1417a69b3: 415d  pop r13
1417a69b5: 415e  pop r14
1417a69b7: 415f  pop r15
1417a69b9: 5d  pop rbp
1417a69ba: c3  ret
1417a69bb: 0f1f440000  nop dword ptr [rax + rax]
1417a69c0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a69c5: 55  push rbp
1417a69c6: 4157  push r15
1417a69c8: 4156  push r14
1417a69ca: 4155  push r13
1417a69cc: 4154  push r12
1417a69ce: 56  push rsi
1417a69cf: 57  push rdi
1417a69d0: 53  push rbx
1417a69d1: 4883ec28  sub rsp, 0x28
1417a69d5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a69dc: 0fb69d1f030000  movzx ebx, byte ptr [rbp + 0x31f]
1417a69e3: 80bd1e03000000  cmp byte ptr [rbp + 0x31e], 0
1417a69ea: 7435  je 0x1417a6a21
1417a69ec: 488b95f8000000  mov rdx, qword ptr [rbp + 0xf8]
1417a69f3: 4885d2  test rdx, rdx
1417a69f6: 7412  je 0x1417a6a0a
1417a69f8: 488b8d00010000  mov rcx, qword ptr [rbp + 0x100]
1417a69ff: 41b801000000  mov r8d, 1
1417a6a05: e816d2e6fe  call 0x140613c20
1417a6a0a: 889d20030000  mov byte ptr [rbp + 0x320], bl
1417a6a10: 4883c428  add rsp, 0x28
1417a6a14: 5b  pop rbx
1417a6a15: 5f  pop rdi
1417a6a16: 5e  pop rsi
1417a6a17: 415c  pop r12
1417a6a19: 415d  pop r13
1417a6a1b: 415e  pop r14
1417a6a1d: 415f  pop r15
1417a6a1f: 5d  pop rbp
1417a6a20: c3  ret
1417a6a21: 889d20030000  mov byte ptr [rbp + 0x320], bl
1417a6a27: 4883c428  add rsp, 0x28
1417a6a2b: 5b  pop rbx
1417a6a2c: 5f  pop rdi
1417a6a2d: 5e  pop rsi
1417a6a2e: 415c  pop r12
1417a6a30: 415d  pop r13
1417a6a32: 415e  pop r14
1417a6a34: 415f  pop r15
1417a6a36: 5d  pop rbp
1417a6a37: c3  ret
1417a6a38: 0f1f840000000000  nop dword ptr [rax + rax]
1417a6a40: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6a45: 55  push rbp
1417a6a46: 4157  push r15
1417a6a48: 4156  push r14
1417a6a4a: 4155  push r13
1417a6a4c: 4154  push r12
1417a6a4e: 56  push rsi
1417a6a4f: 57  push rdi
1417a6a50: 53  push rbx
1417a6a51: 4883ec28  sub rsp, 0x28
1417a6a55: 488daa80000000  lea rbp, [rdx + 0x80]
1417a6a5c: 0fb69d20030000  movzx ebx, byte ptr [rbp + 0x320]
1417a6a63: 488d4db0  lea rcx, [rbp - 0x50]
1417a6a67: e8e4349afe  call 0x140149f50
1417a6a6c: 889dce020000  mov byte ptr [rbp + 0x2ce], bl
1417a6a72: 4883c428  add rsp, 0x28
1417a6a76: 5b  pop rbx
1417a6a77: 5f  pop rdi
1417a6a78: 5e  pop rsi
1417a6a79: 415c  pop r12
1417a6a7b: 415d  pop r13
1417a6a7d: 415e  pop r14
1417a6a7f: 415f  pop r15
1417a6a81: 5d  pop rbp
1417a6a82: c3  ret
1417a6a83: 666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a6a90: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6a95: 55  push rbp
1417a6a96: 4157  push r15
1417a6a98: 4156  push r14
1417a6a9a: 4155  push r13
1417a6a9c: 4154  push r12
1417a6a9e: 56  push rsi
1417a6a9f: 57  push rdi
1417a6aa0: 53  push rbx
1417a6aa1: 4883ec28  sub rsp, 0x28
1417a6aa5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a6aac: 0fb69dce020000  movzx ebx, byte ptr [rbp + 0x2ce]
1417a6ab3: 488b5550  mov rdx, qword ptr [rbp + 0x50]
1417a6ab7: 4885d2  test rdx, rdx
1417a6aba: 7413  je 0x1417a6acf
1417a6abc: 488b4d58  mov rcx, qword ptr [rbp + 0x58]
1417a6ac0: 48c1e203  shl rdx, 3
1417a6ac4: 41b804000000  mov r8d, 4
1417a6aca: e851d1e6fe  call 0x140613c20
1417a6acf: 488d8d10010000  lea rcx, [rbp + 0x110]
1417a6ad6: e845880dff  call 0x14087f320
1417a6adb: 889dcd020000  mov byte ptr [rbp + 0x2cd], bl
1417a6ae1: 4883c428  add rsp, 0x28
1417a6ae5: 5b  pop rbx
1417a6ae6: 5f  pop rdi
1417a6ae7: 5e  pop rsi
1417a6ae8: 415c  pop r12
1417a6aea: 415d  pop r13
1417a6aec: 415e  pop r14
1417a6aee: 415f  pop r15
1417a6af0: 5d  pop rbp
1417a6af1: c3  ret
1417a6af2: 66666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a6b00: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6b05: 55  push rbp
1417a6b06: 4157  push r15
1417a6b08: 4156  push r14
1417a6b0a: 4155  push r13
1417a6b0c: 4154  push r12
1417a6b0e: 56  push rsi
1417a6b0f: 57  push rdi
1417a6b10: 53  push rbx
1417a6b11: 4883ec28  sub rsp, 0x28
1417a6b15: 488daa80000000  lea rbp, [rdx + 0x80]
1417a6b1c: 0fb69dcd020000  movzx ebx, byte ptr [rbp + 0x2cd]
1417a6b23: 488b9570010000  mov rdx, qword ptr [rbp + 0x170]
1417a6b2a: 4885d2  test rdx, rdx
1417a6b2d: 7416  je 0x1417a6b45
1417a6b2f: 488b8d78010000  mov rcx, qword ptr [rbp + 0x178]
1417a6b36: 48c1e203  shl rdx, 3
1417a6b3a: 41b808000000  mov r8d, 8
1417a6b40: e8dbd0e6fe  call 0x140613c20
1417a6b45: 488b95d0010000  mov rdx, qword ptr [rbp + 0x1d0]
1417a6b4c: 4885d2  test rdx, rdx
1417a6b4f: 7412  je 0x1417a6b63
1417a6b51: 41b801000000  mov r8d, 1
1417a6b57: 488b8df8010000  mov rcx, qword ptr [rbp + 0x1f8]
1417a6b5e: e8bdd0e6fe  call 0x140613c20
1417a6b63: 488d4de8  lea rcx, [rbp - 0x18]
1417a6b67: 488d5508  lea rdx, [rbp + 8]
1417a6b6b: 41b830000000  mov r8d, 0x30
1417a6b71: 41b910000000  mov r9d, 0x10
1417a6b77: e8f4898afe  call 0x14004f570
1417a6b7c: 889d1d030000  mov byte ptr [rbp + 0x31d], bl
1417a6b82: 4883c428  add rsp, 0x28
1417a6b86: 5b  pop rbx
1417a6b87: 5f  pop rdi
1417a6b88: 5e  pop rsi
1417a6b89: 415c  pop r12
1417a6b8b: 415d  pop r13
1417a6b8d: 415e  pop r14
1417a6b8f: 415f  pop r15
1417a6b91: 5d  pop rbp
1417a6b92: c3  ret
1417a6b93: 666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a6ba0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6ba5: 55  push rbp
1417a6ba6: 4157  push r15
1417a6ba8: 4156  push r14
1417a6baa: 4155  push r13
1417a6bac: 4154  push r12
1417a6bae: 56  push rsi
1417a6baf: 57  push rdi
1417a6bb0: 53  push rbx
1417a6bb1: 4883ec28  sub rsp, 0x28
1417a6bb5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a6bbc: 0fb69d1d030000  movzx ebx, byte ptr [rbp + 0x31d]
1417a6bc3: 488b95a0000000  mov rdx, qword ptr [rbp + 0xa0]
1417a6bca: 4885d2  test rdx, rdx
1417a6bcd: 753b  jne 0x1417a6c0a
1417a6bcf: 488b95c0000000  mov rdx, qword ptr [rbp + 0xc0]
1417a6bd6: 4885d2  test rdx, rdx
1417a6bd9: 7551  jne 0x1417a6c2c
1417a6bdb: 488b95e0000000  mov rdx, qword ptr [rbp + 0xe0]
1417a6be2: 4885d2  test rdx, rdx
1417a6be5: 7567  jne 0x1417a6c4e
1417a6be7: f6c301  test bl, 1
1417a6bea: 747d  je 0x1417a6c69
1417a6bec: 488d8d90010000  lea rcx, [rbp + 0x190]
1417a6bf3: e8c85ee4fe  call 0x1405ecac0
1417a6bf8: 90  nop
1417a6bf9: 4883c428  add rsp, 0x28
1417a6bfd: 5b  pop rbx
1417a6bfe: 5f  pop rdi
1417a6bff: 5e  pop rsi
1417a6c00: 415c  pop r12
1417a6c02: 415d  pop r13
1417a6c04: 415e  pop r14
1417a6c06: 415f  pop r15
1417a6c08: 5d  pop rbp
1417a6c09: c3  ret
1417a6c0a: 488b8da8000000  mov rcx, qword ptr [rbp + 0xa8]
1417a6c11: 48c1e204  shl rdx, 4
1417a6c15: 41b808000000  mov r8d, 8
1417a6c1b: e800d0e6fe  call 0x140613c20
1417a6c20: 488b95c0000000  mov rdx, qword ptr [rbp + 0xc0]
1417a6c27: 4885d2  test rdx, rdx
1417a6c2a: 74af  je 0x1417a6bdb
1417a6c2c: 488b8dc8000000  mov rcx, qword ptr [rbp + 0xc8]
1417a6c33: 48c1e204  shl rdx, 4
1417a6c37: 41b808000000  mov r8d, 8
1417a6c3d: e8decfe6fe  call 0x140613c20
1417a6c42: 488b95e0000000  mov rdx, qword ptr [rbp + 0xe0]
1417a6c49: 4885d2  test rdx, rdx
1417a6c4c: 7499  je 0x1417a6be7
1417a6c4e: 488b8de8000000  mov rcx, qword ptr [rbp + 0xe8]
1417a6c55: 48c1e204  shl rdx, 4
1417a6c59: 41b808000000  mov r8d, 8
1417a6c5f: e8bccfe6fe  call 0x140613c20
1417a6c64: f6c301  test bl, 1
1417a6c67: 7583  jne 0x1417a6bec
1417a6c69: 4883c428  add rsp, 0x28
1417a6c6d: 5b  pop rbx
1417a6c6e: 5f  pop rdi
1417a6c6f: 5e  pop rsi
1417a6c70: 415c  pop r12
1417a6c72: 415d  pop r13
1417a6c74: 415e  pop r14
1417a6c76: 415f  pop r15
1417a6c78: 5d  pop rbp
1417a6c79: c3  ret
1417a6c7a: 660f1f440000  nop word ptr [rax + rax]
1417a6c80: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6c85: 55  push rbp
1417a6c86: 4157  push r15
1417a6c88: 4156  push r14
1417a6c8a: 4155  push r13
1417a6c8c: 4154  push r12
1417a6c8e: 56  push rsi
1417a6c8f: 57  push rdi
1417a6c90: 53  push rbx
1417a6c91: 4883ec28  sub rsp, 0x28
1417a6c95: 488daa80000000  lea rbp, [rdx + 0x80]
1417a6c9c: 488d8d30010000  lea rcx, [rbp + 0x130]
1417a6ca3: e8f8b80dff  call 0x1408825a0
1417a6ca8: 90  nop
1417a6ca9: 4883c428  add rsp, 0x28
1417a6cad: 5b  pop rbx
1417a6cae: 5f  pop rdi
1417a6caf: 5e  pop rsi
1417a6cb0: 415c  pop r12
1417a6cb2: 415d  pop r13
1417a6cb4: 415e  pop r14
1417a6cb6: 415f  pop r15
1417a6cb8: 5d  pop rbp
1417a6cb9: c3  ret
1417a6cba: 660f1f440000  nop word ptr [rax + rax]
1417a6cc0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6cc5: 55  push rbp
1417a6cc6: 4157  push r15
1417a6cc8: 4156  push r14
1417a6cca: 4155  push r13
1417a6ccc: 4154  push r12
1417a6cce: 56  push rsi
1417a6ccf: 57  push rdi
1417a6cd0: 53  push rbx
1417a6cd1: 4883ec28  sub rsp, 0x28
1417a6cd5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a6cdc: 488b85b0010000  mov rax, qword ptr [rbp + 0x1b0]
1417a6ce3: 4885c0  test rax, rax
1417a6ce6: 7416  je 0x1417a6cfe
1417a6ce8: 488b8db8010000  mov rcx, qword ptr [rbp + 0x1b8]
1417a6cef: 486bd058  imul rdx, rax, 0x58
1417a6cf3: 41b808000000  mov r8d, 8
1417a6cf9: e822cfe6fe  call 0x140613c20
1417a6cfe: 488d8d50010000  lea rcx, [rbp + 0x150]
1417a6d05: e8468a0dff  call 0x14087f750
1417a6d0a: 90  nop
1417a6d0b: 4883c428  add rsp, 0x28
1417a6d0f: 5b  pop rbx
1417a6d10: 5f  pop rdi
1417a6d11: 5e  pop rsi
1417a6d12: 415c  pop r12
1417a6d14: 415d  pop r13
1417a6d16: 415e  pop r14
1417a6d18: 415f  pop r15
1417a6d1a: 5d  pop rbp
1417a6d1b: c3  ret
1417a6d1c: 0f1f4000  nop dword ptr [rax]
1417a6d20: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a6d25: 55  push rbp
1417a6d26: 4157  push r15
1417a6d28: 4156  push r14
1417a6d2a: 4155  push r13
1417a6d2c: 4154  push r12
1417a6d2e: 56  push rsi
1417a6d2f: 57  push rdi
1417a6d30: 53  push rbx
1417a6d31: 4883ec28  sub rsp, 0x28
1417a6d35: 488daa80000000  lea rbp, [rdx + 0x80]
1417a6d3c: ba98010000  mov edx, 0x198
1417a6d41: 41b808000000  mov r8d, 8
1417a6d47: 488b8d88000000  mov rcx, qword ptr [rbp + 0x88]
1417a6d4e: e8cdcee6fe  call 0x140613c20
1417a6d53: 90  nop
1417a6d54: 4883c428  add rsp, 0x28
1417a6d58: 5b  pop rbx
1417a6d59: 5f  pop rdi
1417a6d5a: 5e  pop rsi
1417a6d5b: 415c  pop r12
1417a6d5d: 415d  pop r13
1417a6d5f: 415e  pop r14
1417a6d61: 415f  pop r15
1417a6d63: 5d  pop rbp
1417a6d64: c3  ret
