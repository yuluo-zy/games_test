0x1401647b0: push rbp
0x1401647b1: push r15
0x1401647b3: push r14
0x1401647b5: push r13
0x1401647b7: push r12
0x1401647b9: push rsi
0x1401647ba: push rdi
0x1401647bb: push rbx
0x1401647bc: mov eax, 0x1a18
0x1401647c1: call 0x14289aa40
0x1401647c6: sub rsp, rax
0x1401647c9: lea rbp, [rsp + 0x80]
0x1401647d1: movaps xmmword ptr [rbp + 0x1980], xmm6
0x1401647d8: mov qword ptr [rbp + 0x1978], 0xfffffffffffffffe
0x1401647e3: mov qword ptr [rbp + 0x18e8], rcx
0x1401647ea: mov rax, qword ptr [rdx]
0x1401647ed: movabs rcx, 0x8000000000000000
0x1401647f7: mov qword ptr [rdx], rcx
0x1401647fa: mov rcx, rax
0x1401647fd: neg rcx
0x140164800: jo 0x140165be3
0x140164806: mov r12, r8
0x140164809: mov rdi, rdx
0x14016480c: mov qword ptr [rbp + 0x18d0], r9
0x140164813: add rdx, 8
0x140164817: mov qword ptr [rbp + 0x1170], rax
0x14016481e: lea rcx, [rbp + 0x1178]
0x140164825: mov r8d, 0x88
0x14016482b: call 0x142923c00
0x140164830: mov rbx, qword ptr [rdi + 0x90]
0x140164837: mov qword ptr [rdi + 0x90], 0
0x140164842: test rbx, rbx
0x140164845: mov qword ptr [rbp + 0x18e0], rbx
0x14016484c: je 0x140165bef
0x140164852: mov rsi, qword ptr [rbp + 0x1a00]
0x140164859: mov rax, qword ptr [rdi + 0xe8]
0x140164860: mov qword ptr [rbp + 0x1328], rax
0x140164867: movups xmm0, xmmword ptr [rdi + 0xd8]
0x14016486e: movups xmmword ptr [rbp + 0x1318], xmm0
0x140164875: movups xmm0, xmmword ptr [rdi + 0x98]
0x14016487c: movups xmm1, xmmword ptr [rdi + 0xa8]
0x140164883: movups xmm2, xmmword ptr [rdi + 0xb8]
0x14016488a: movups xmm3, xmmword ptr [rdi + 0xc8]
0x140164891: movups xmmword ptr [rbp + 0x1308], xmm3
0x140164898: movups xmmword ptr [rbp + 0x12f8], xmm2
0x14016489f: movups xmmword ptr [rbp + 0x12e8], xmm1
0x1401648a6: movups xmmword ptr [rbp + 0x12d8], xmm0
0x1401648ad: lea rcx, [rbp + 0x1220]
0x1401648b4: lea rdx, [rbp + 0x1170]
0x1401648bb: mov r8d, 0x90
0x1401648c1: call 0x142923c00
0x1401648c6: mov r15, qword ptr [rsi]
0x1401648c9: mov r14, qword ptr [rsi + 8]
0x1401648cd: mov rax, qword ptr [rsi + 0x10]
0x1401648d1: mov qword ptr [rbp + 0x1958], rax
0x1401648d8: mov rax, qword ptr [rsi + 0x18]
0x1401648dc: mov qword ptr [rbp + 0x18c0], rax
0x1401648e3: mov rax, qword ptr [rsi + 0x20]
0x1401648e7: mov qword ptr [rbp + 0x18c8], rax
0x1401648ee: mov qword ptr [rbp + 0x12b0], 0
0x1401648f9: mov qword ptr [rbp + 0x12b8], 8
0x140164904: mov qword ptr [rbp + 0x12c0], 0
0x14016490f: mov qword ptr [rbp + 0x18d8], r12
0x140164916: mov qword ptr [rbp + 0x12c8], r12
0x14016491d: mov qword ptr [rbp + 0x12d0], rbx
0x140164924: lea r12, [r15 + 0x38]
0x140164928: mov byte ptr [rbp + 0x1975], 1
0x14016492f: movabs rdx, 0xa68561b396784eaa
0x140164939: movabs r8, 0xf5fd74e6faf4ed14
0x140164943: mov rcx, r12
0x140164946: call 0x1400dd6f0
0x14016494b: test rax, rax
0x14016494e: je 0x140165b4f
0x140164954: cmp qword ptr [r15 + 0x218], rdx
0x14016495b: jbe 0x140165b4f
0x140164961: mov rax, qword ptr [r15 + 0x210]
0x140164968: mov rax, qword ptr [rax + rdx*8]
0x14016496c: test rax, rax
0x14016496f: je 0x140165b4f
0x140164975: mov rbx, qword ptr [r15 + 0x1e0]
0x14016497c: not rax
0x14016497f: lea rax, [rax + rax*2]
0x140164983: shl rax, 5
0x140164987: cmp qword ptr [rbx + rax + 0x20], 0
0x14016498d: je 0x140165b4f
0x140164993: add rbx, rax
0x140164996: mov r13, qword ptr [rbx + 0x10]
0x14016499a: mov esi, dword ptr [r15 + 0x5b0]
0x1401649a1: mov dword ptr [rbx + 0x5c], esi
0x1401649a4: mov byte ptr [rbp + 0x1975], 1
0x1401649ab: lea rdx, [rbp + 0x1220]
0x1401649b2: mov rcx, r13
0x1401649b5: call 0x14093cd30
0x1401649ba: mov dword ptr [rbx + 0x5c], esi
0x1401649bd: mov byte ptr [rbp + 0x1975], 1
0x1401649c4: mov rcx, r13
0x1401649c7: mov rdx, r14
0x1401649ca: call 0x14093d040
0x1401649cf: mov byte ptr [rbp + 0x1975], 1
0x1401649d6: lea rcx, [rbp + 0x1888]
0x1401649dd: mov rdx, rax
0x1401649e0: call 0x140bd6b00
0x1401649e5: mov byte ptr [rbp + 0x1976], 1
0x1401649ec: movabs rdx, 0x8fd9dc19c7973bca
0x1401649f6: movabs r8, 0x361ff4c5eb9b45fd
0x140164a00: mov rcx, r12
0x140164a03: call 0x1400dd6f0
0x140164a08: test rax, rax
0x140164a0b: je 0x140165b67
0x140164a11: cmp qword ptr [r15 + 0x218], rdx
0x140164a18: jbe 0x140165b67
0x140164a1e: mov rax, qword ptr [r15 + 0x210]
0x140164a25: mov rcx, qword ptr [rax + rdx*8]
0x140164a29: test rcx, rcx
0x140164a2c: je 0x140165b67
0x140164a32: mov rax, qword ptr [r15 + 0x1e0]
0x140164a39: not rcx
0x140164a3c: lea rcx, [rcx + rcx*2]
0x140164a40: shl rcx, 5
0x140164a44: cmp qword ptr [rax + rcx + 0x20], 0
0x140164a4a: je 0x140165b67
0x140164a50: add rax, rcx
0x140164a53: mov rdx, qword ptr [rax + 0x10]
0x140164a57: lea rcx, [rbp + 0xe0]
0x140164a5e: mov r8d, 0x1090
0x140164a64: call 0x142923c00
0x140164a69: mov byte ptr [rbp + 0x1976], 1
0x140164a70: movabs rdx, 0x47c94cb78283a72e
0x140164a7a: movabs r8, 0x8c173ded97323e9d
0x140164a84: mov rcx, r12
0x140164a87: call 0x1400dd6f0
0x140164a8c: test rax, rax
0x140164a8f: je 0x140165b7f
0x140164a95: cmp qword ptr [r15 + 0x218], rdx
0x140164a9c: jbe 0x140165b7f
0x140164aa2: mov rax, qword ptr [r15 + 0x210]
0x140164aa9: mov rcx, qword ptr [rax + rdx*8]
0x140164aad: test rcx, rcx
0x140164ab0: je 0x140165b7f
0x140164ab6: mov rax, qword ptr [r15 + 0x1e0]
0x140164abd: not rcx
0x140164ac0: lea rcx, [rcx + rcx*2]
0x140164ac4: shl rcx, 5
0x140164ac8: cmp qword ptr [rax + rcx + 0x20], 0
0x140164ace: je 0x140165b7f
0x140164ad4: add rax, rcx
0x140164ad7: mov rcx, qword ptr [rax + 0x10]
0x140164adb: mov byte ptr [rbp + 0x1976], 1
0x140164ae2: call 0x14067dd80
0x140164ae7: mov rsi, rax
0x140164aea: mov cl, 1
0x140164aec: xor eax, eax
0x140164aee: lock cmpxchg byte ptr [rsi], cl
0x140164af2: jne 0x140165b09
0x140164af8: mov byte ptr [rbp + 0x1976], 1
0x140164aff: mov rcx, rsi
0x140164b02: call 0x140c81c70
0x140164b07: mov qword ptr [rbp + 0x1960], rsi
0x140164b0e: add rsi, 8
0x140164b12: mov qword ptr [rsp + 0x20], 0
0x140164b1b: lea rcx, [rbp + 0x16f0]
0x140164b22: mov r9d, 0x1090
0x140164b28: mov rdx, rsi
0x140164b2b: mov r8, r14
0x140164b2e: call 0x140c02740
0x140164b33: mov rcx, qword ptr [rbp + 0x16f8]
0x140164b3a: cmp rcx, 0x1090
0x140164b41: jne 0x140165b23
0x140164b47: mov rcx, qword ptr [rbp + 0x16f0]
0x140164b4e: lea rdx, [rbp + 0xe0]
0x140164b55: mov r8d, 0x1090
0x140164b5b: call 0x142923c00
0x140164b60: movups xmm0, xmmword ptr [rbp + 0x16f0]
0x140164b67: movups xmm1, xmmword ptr [rbp + 0x1700]
0x140164b6e: movups xmm2, xmmword ptr [rbp + 0x1710]
0x140164b75: movups xmm3, xmmword ptr [rbp + 0x1720]
0x140164b7c: movaps xmmword ptr [rbp + 0x1440], xmm0
0x140164b83: movaps xmmword ptr [rbp + 0x1450], xmm1
0x140164b8a: movaps xmmword ptr [rbp + 0x1460], xmm2
0x140164b91: movaps xmmword ptr [rbp + 0x1470], xmm3
0x140164b98: mov rax, qword ptr [rbp + 0x1730]
0x140164b9f: mov qword ptr [rbp + 0x1480], rax
0x140164ba6: lea rcx, [rbp + 0x1350]
0x140164bad: lea rdx, [rbp + 0x1440]
0x140164bb4: mov r8, r14
0x140164bb7: call 0x140c02370
0x140164bbc: mov byte ptr [rbp + 0x1970], 1
0x140164bc3: mov rsi, qword ptr [rbp + 0x1960]
0x140164bca: mov rcx, rsi
0x140164bcd: call 0x140c81e10
0x140164bd2: xor ecx, ecx
0x140164bd4: mov al, 1
0x140164bd6: lock cmpxchg byte ptr [rsi], cl
0x140164bda: jne 0x140165b39
0x140164be0: lea rcx, [rbp + 0x1440]
0x140164be7: lea rdx, [rbp + 0x1220]
0x140164bee: mov r8d, 0x110
0x140164bf4: call 0x142923c00
0x140164bf9: movups xmm0, xmmword ptr [rbp + 0x1350]
0x140164c00: movups xmm1, xmmword ptr [rbp + 0x1360]
0x140164c07: movups xmm2, xmmword ptr [rbp + 0x1370]
0x140164c0e: movups xmm3, xmmword ptr [rbp + 0x1380]
0x140164c15: movups xmmword ptr [rbp + 0x1550], xmm0
0x140164c1c: movups xmmword ptr [rbp + 0x1560], xmm1
0x140164c23: movups xmmword ptr [rbp + 0x1570], xmm2
0x140164c2a: movups xmmword ptr [rbp + 0x1580], xmm3
0x140164c31: movups xmm0, xmmword ptr [rbp + 0x1390]
0x140164c38: movups xmmword ptr [rbp + 0x1590], xmm0
0x140164c3f: movups xmm0, xmmword ptr [rbp + 0x13a0]
0x140164c46: movups xmmword ptr [rbp + 0x15a0], xmm0
0x140164c4d: movups xmm0, xmmword ptr [rbp + 0x1888]
0x140164c54: movups xmmword ptr [rbp + 0x15b0], xmm0
0x140164c5b: mov rax, qword ptr [rbp + 0x1898]
0x140164c62: mov qword ptr [rbp + 0x15c0], rax
0x140164c69: mov rax, qword ptr [rbp + 0x1958]
0x140164c70: mov qword ptr [rbp + 0x15c8], rax
0x140164c77: mov qword ptr [rbp + 0x15d0], r14
0x140164c7e: cmp byte ptr [rip + 0x34c6461], 8
0x140164c85: jne 0x14016511c
0x140164c8b: mov byte ptr [rbp + 0x1977], 1
0x140164c92: movabs rdx, 0x4da5153112b09210
0x140164c9c: movabs r8, 0x5b807edb4e205af1
0x140164ca6: mov rcx, r12
0x140164ca9: call 0x1400dd6f0
0x140164cae: test rax, rax
0x140164cb1: je 0x14016509e
0x140164cb7: cmp qword ptr [r15 + 0x218], rdx
0x140164cbe: jbe 0x14016509e
0x140164cc4: mov rax, qword ptr [r15 + 0x210]
0x140164ccb: mov rcx, qword ptr [rax + rdx*8]
0x140164ccf: test rcx, rcx
0x140164cd2: je 0x14016509e
0x140164cd8: mov rax, qword ptr [r15 + 0x1e0]
0x140164cdf: not rcx
0x140164ce2: lea rcx, [rcx + rcx*2]
0x140164ce6: shl rcx, 5
0x140164cea: cmp qword ptr [rax + rcx + 0x20], 0
0x140164cf0: je 0x14016509e
0x140164cf6: mov byte ptr [rbp + 0x1960], 7
0x140164cfd: add rax, rcx
0x140164d00: mov rax, qword ptr [rax + 0x10]
0x140164d04: cmp qword ptr [rax + 0x10], 0
0x140164d09: je 0x1401650a5
0x140164d0f: mov rdx, qword ptr [rax + 8]
0x140164d13: mov byte ptr [rbp + 0x1977], 1
0x140164d1a: lea rcx, [rbp + 0x1330]
0x140164d21: call 0x140b46a50
0x140164d26: mov byte ptr [rbp + 0x1977], 1
0x140164d2d: lea rcx, [rbp + 0x16f0]
0x140164d34: lea rdx, [rbp + 0x1330]
0x140164d3b: call 0x1400c6970
0x140164d40: mov rax, qword ptr [rbp + 0x16f0]
0x140164d47: movabs rcx, 0x800000000000000a
0x140164d51: cmp rax, rcx
0x140164d54: jne 0x14016599f
0x140164d5a: mov rax, qword ptr [rbp + 0x16f8]
0x140164d61: movups xmm0, xmmword ptr [rbp + 0x1700]
0x140164d68: movaps xmmword ptr [rbp + 0x1920], xmm0
0x140164d6f: movups xmm0, xmmword ptr [rbp + 0x1710]
0x140164d76: movaps xmmword ptr [rbp + 0x1930], xmm0
0x140164d7d: cmp rax, 0xa
0x140164d81: je 0x1401650a5
0x140164d87: movaps xmm0, xmmword ptr [rbp + 0x1920]
0x140164d8e: movaps xmm1, xmmword ptr [rbp + 0x1930]
0x140164d95: movups xmmword ptr [rbp + 0x15f8], xmm1
0x140164d9c: movups xmmword ptr [rbp + 0x15e8], xmm0
0x140164da3: mov qword ptr [rbp + 0x15e0], rax
0x140164daa: lea rcx, [rbp + 0x18a0]
0x140164db1: lea rdx, [rbp + 0x15e0]
0x140164db8: call 0x140bad8d0
0x140164dbd: lea rcx, [rbp + 0x15e0]
0x140164dc4: call 0x140161c60
0x140164dc9: mov rdx, qword ptr [rbp + 0x18b8]
0x140164dd0: mov byte ptr [rbp + 0x196f], 1
0x140164dd7: lea rcx, [rbp + 0x18f0]
0x140164dde: mov r8d, 0x2b
0x140164de4: call 0x140bd52a0
0x140164de9: mov byte ptr [rbp + 0x196f], 1
0x140164df0: lea r13, [rbp + 0x1200]
0x140164df7: lea rdx, [rbp + 0x18f0]
0x140164dfe: mov rcx, r13
0x140164e01: call 0x140bd5370
0x140164e06: mov rax, qword ptr [rbp + 0x18a0]
0x140164e0d: mov qword ptr [rbp + 0x1958], rax
0x140164e14: mov rcx, qword ptr [rbp + 0x18a8]
0x140164e1b: mov rax, qword ptr [rbp + 0x18b0]
0x140164e22: mov qword ptr [rsp + 0x30], rax
0x140164e27: mov qword ptr [rbp + 0x1918], rcx
0x140164e2e: mov qword ptr [rsp + 0x28], rcx
0x140164e33: mov qword ptr [rsp + 0x20], r13
0x140164e38: lea r8, [rip + 0x27f8be1]
0x140164e3f: lea rcx, [rbp + 0x16f0]
0x140164e46: mov r9d, 0x10
0x140164e4c: mov rdx, r14
0x140164e4f: call 0x1405b9b00
0x140164e54: mov rax, qword ptr [rbp + 0x16f0]
0x140164e5b: cmp rax, 0xb
0x140164e5f: jne 0x1401659eb
0x140164e65: mov rax, qword ptr [rbp + 0x1726]
0x140164e6c: mov qword ptr [rbp + 0x160e], rax
0x140164e73: movups xmm0, xmmword ptr [rbp + 0x16f8]
0x140164e7a: movups xmm1, xmmword ptr [rbp + 0x1708]
0x140164e81: movups xmm2, xmmword ptr [rbp + 0x1718]
0x140164e88: movaps xmmword ptr [rbp + 0x1600], xmm2
0x140164e8f: movaps xmmword ptr [rbp + 0x15f0], xmm1
0x140164e96: movaps xmmword ptr [rbp + 0x15e0], xmm0
0x140164e9d: movzx eax, byte ptr [rbp + 0x172e]
0x140164ea4: cmp al, 7
0x140164ea6: je 0x140165ae2
0x140164eac: movzx ecx, byte ptr [rbp + 0x172f]
0x140164eb3: mov rdx, qword ptr [rbp + 0x160e]
0x140164eba: mov qword ptr [rbp + 0x194e], rdx
0x140164ec1: movaps xmm0, xmmword ptr [rbp + 0x15e0]
0x140164ec8: movaps xmm1, xmmword ptr [rbp + 0x15f0]
0x140164ecf: movaps xmm2, xmmword ptr [rbp + 0x1600]
0x140164ed6: movaps xmmword ptr [rbp + 0x1940], xmm2
0x140164edd: movaps xmmword ptr [rbp + 0x1930], xmm1
0x140164ee4: movaps xmmword ptr [rbp + 0x1920], xmm0
0x140164eeb: mov byte ptr [rbp + 0x1956], al
0x140164ef1: mov byte ptr [rbp + 0x1957], cl
0x140164ef7: mov rdx, qword ptr [rbp + 0x1958]
0x140164efe: test rdx, rdx
0x140164f01: je 0x140164f15
0x140164f03: mov r8d, 1
0x140164f09: mov rcx, qword ptr [rbp + 0x1918]
0x140164f10: call 0x140613c20
0x140164f15: movabs rdx, 0xa284155e68bfeb3f
0x140164f1f: movabs r8, 0x84a2b3441bc64a3a
0x140164f29: mov rcx, r12
0x140164f2c: call 0x1400dd6f0
0x140164f31: test rax, rax
0x140164f34: je 0x140165c2c
0x140164f3a: cmp qword ptr [r15 + 0x218], rdx
0x140164f41: jbe 0x140165c2c
0x140164f47: mov rax, qword ptr [r15 + 0x210]
0x140164f4e: mov rcx, qword ptr [rax + rdx*8]
0x140164f52: test rcx, rcx
0x140164f55: je 0x140165c2c
0x140164f5b: mov rax, qword ptr [r15 + 0x1e0]
0x140164f62: not rcx
0x140164f65: lea rcx, [rcx + rcx*2]
0x140164f69: shl rcx, 5
0x140164f6d: cmp qword ptr [rax + rcx + 0x20], 0
0x140164f73: je 0x140165c2c
0x140164f79: add rax, rcx
0x140164f7c: mov rsi, qword ptr [rax + 0x10]
0x140164f80: mov ecx, dword ptr [r15 + 0x5b0]
0x140164f87: mov dword ptr [rax + 0x5c], ecx
0x140164f8a: cmp dword ptr [rsi + 0x48], 2
0x140164f8e: je 0x140165c3a
0x140164f94: lea rcx, [rbp + 0x16f0]
0x140164f9b: lea rdx, [rbp + 0x1920]
0x140164fa2: call 0x140bd6b00
0x140164fa7: movups xmm0, xmmword ptr [rbp + 0x1938]
0x140164fae: movups xmm1, xmmword ptr [rbp + 0x1948]
0x140164fb5: movups xmmword ptr [rbp + 0x1708], xmm0
0x140164fbc: movups xmmword ptr [rbp + 0x1718], xmm1
0x140164fc3: movups xmm0, xmmword ptr [rbp + 0x16f0]
0x140164fca: movaps xmmword ptr [rbp + 0x15e0], xmm0
0x140164fd1: movups xmm0, xmmword ptr [rbp + 0x1700]
0x140164fd8: movaps xmmword ptr [rbp + 0x15f0], xmm0
0x140164fdf: movups xmm0, xmmword ptr [rbp + 0x1710]
0x140164fe6: movaps xmmword ptr [rbp + 0x1600], xmm0
0x140164fed: mov rax, qword ptr [rbp + 0x1720]
0x140164ff4: mov qword ptr [rbp + 0x1610], rax
0x140164ffb: lea rax, [rsi + 8]
0x140164fff: mov qword ptr [rbp + 0x1960], rax
0x140165006: cmp byte ptr [rsi + 0x3e], 7
0x14016500a: je 0x140165018
0x14016500c: mov rcx, qword ptr [rbp + 0x1960]
0x140165013: call 0x1405ee5e0
0x140165018: mov rax, qword ptr [rbp + 0x1610]
0x14016501f: mov rcx, qword ptr [rbp + 0x1960]
0x140165026: mov qword ptr [rcx + 0x30], rax
0x14016502a: movaps xmm0, xmmword ptr [rbp + 0x15e0]
0x140165031: movaps xmm1, xmmword ptr [rbp + 0x15f0]
0x140165038: movaps xmm2, xmmword ptr [rbp + 0x1600]
0x14016503f: movups xmmword ptr [rcx + 0x20], xmm2
0x140165043: movups xmmword ptr [rcx + 0x10], xmm1
0x140165047: movups xmmword ptr [rcx], xmm0
0x14016504a: movaps xmm0, xmmword ptr [rbp + 0x1920]
0x140165051: movaps xmm1, xmmword ptr [rbp + 0x1930]
0x140165058: movaps xmm2, xmmword ptr [rbp + 0x1940]
0x14016505f: movaps xmmword ptr [rbp + 0x16f0], xmm0
0x140165066: movaps xmmword ptr [rbp + 0x1700], xmm1
0x14016506d: movaps xmmword ptr [rbp + 0x1710], xmm2
0x140165074: mov rax, qword ptr [rbp + 0x194e]
0x14016507b: mov qword ptr [rbp + 0x171e], rax
0x140165082: movzx eax, byte ptr [rbp + 0x1956]
0x140165089: mov byte ptr [rbp + 0x1960], al
0x14016508f: movzx eax, byte ptr [rbp + 0x1957]
0x140165096: mov byte ptr [rbp + 0x1958], al
0x14016509c: jmp 0x1401650a5
0x14016509e: mov byte ptr [rbp + 0x1960], 7
0x1401650a5: movzx eax, byte ptr [rip + 0x34c603a]
0x1401650ac: add al, 0xf9
0x1401650ae: cmp al, 2
0x1401650b0: jb 0x1401650ca
0x1401650b2: lea rcx, [rip + 0x34c5ff7]
0x1401650b9: call 0x14003bb80
0x1401650be: lea rcx, [rip + 0x34c5feb]
0x1401650c5: call 0x140be24b0
0x1401650ca: mov rax, qword ptr [rbp + 0x171e]
0x1401650d1: mov qword ptr [rip + 0x34c6006], rax
0x1401650d8: movaps xmm0, xmmword ptr [rbp + 0x16f0]
0x1401650df: movaps xmm1, xmmword ptr [rbp + 0x1700]
0x1401650e6: movaps xmm2, xmmword ptr [rbp + 0x1710]
0x1401650ed: movups xmmword ptr [rip + 0x34c5fdc], xmm2
0x1401650f4: movups xmmword ptr [rip + 0x34c5fc5], xmm1
0x1401650fb: movups xmmword ptr [rip + 0x34c5fae], xmm0
0x140165102: movzx eax, byte ptr [rbp + 0x1960]
0x140165109: mov byte ptr [rip + 0x34c5fd7], al
0x14016510f: movzx eax, byte ptr [rbp + 0x1958]
0x140165116: mov byte ptr [rip + 0x34c5fcb], al
0x14016511c: mov rax, qword ptr [rbp + 0x18c0]
0x140165123: mov rdx, qword ptr [rax]
0x140165126: mov byte ptr [rbp + 0x1977], 1
0x14016512d: lea rcx, [rbp + 0x18f0]
0x140165134: mov r8d, 0x2b
0x14016513a: call 0x140bd52a0
0x14016513f: movups xmm0, xmmword ptr [rbp + 0x18f0]
0x140165146: movups xmm1, xmmword ptr [rbp + 0x1900]
0x14016514d: movaps xmmword ptr [rbp + 0x15f0], xmm1
0x140165154: movaps xmmword ptr [rbp + 0x15e0], xmm0
0x14016515b: lea rcx, [rbp + 0x1700]
0x140165162: mov byte ptr [rbp + 0x1977], 1
0x140165169: lea rdx, [rbp + 0x15e0]
0x140165170: call 0x140bd6ce0
0x140165175: lea rax, [rip + 0x27f87f4]
0x14016517c: mov qword ptr [rbp + 0x16f0], rax
0x140165183: mov qword ptr [rbp + 0x16f8], 0xe
0x14016518e: mov byte ptr [rbp + 0x1977], 1
0x140165195: lea rcx, [rbp + 0x1440]
0x14016519c: lea rdx, [rbp + 0x16f0]
0x1401651a3: call 0x140be94d0
0x1401651a8: movups xmm0, xmmword ptr [rbp + 0x18f0]
0x1401651af: movups xmm1, xmmword ptr [rbp + 0x1900]
0x1401651b6: movups xmmword ptr [rbp + 0x1924], xmm0
0x1401651bd: movups xmmword ptr [rbp + 0x1934], xmm1
0x1401651c4: mov dword ptr [rbp + 0x1920], eax
0x1401651ca: mov byte ptr [rbp + 0x1977], 1
0x1401651d1: movabs rdx, 0x109ca79d5cccc2d1
0x1401651db: movabs r8, 0x24eabc1830c9840c
0x1401651e5: mov rcx, r12
0x1401651e8: call 0x1400dd6f0
0x1401651ed: test rax, rax
0x1401651f0: je 0x140165b97
0x1401651f6: cmp qword ptr [r15 + 0x218], rdx
0x1401651fd: jbe 0x140165b97
0x140165203: mov rax, qword ptr [r15 + 0x210]
0x14016520a: mov rcx, qword ptr [rax + rdx*8]
0x14016520e: test rcx, rcx
0x140165211: je 0x140165b97
0x140165217: mov rax, qword ptr [r15 + 0x1e0]
0x14016521e: not rcx
0x140165221: lea rcx, [rcx + rcx*2]
0x140165225: shl rcx, 5
0x140165229: cmp qword ptr [rax + rcx + 0x20], 0
0x14016522f: je 0x140165b97
0x140165235: add rax, rcx
0x140165238: mov rax, qword ptr [rax + 0x10]
0x14016523c: movss xmm6, dword ptr [rax + 0x58]
0x140165241: mov byte ptr [rbp + 0x1977], 1
0x140165248: movabs rdx, 0xa284155e68bfeb3f
0x140165252: movabs r8, 0x84a2b3441bc64a3a
0x14016525c: mov rcx, r12
0x14016525f: call 0x1400dd6f0
0x140165264: test rax, rax
0x140165267: je 0x140165baf
0x14016526d: cmp qword ptr [r15 + 0x218], rdx
0x140165274: jbe 0x140165baf
0x14016527a: mov rax, qword ptr [r15 + 0x210]
0x140165281: mov rcx, qword ptr [rax + rdx*8]
0x140165285: test rcx, rcx
0x140165288: je 0x140165baf
0x14016528e: mov rax, qword ptr [r15 + 0x1e0]
0x140165295: not rcx
0x140165298: lea rcx, [rcx + rcx*2]
0x14016529c: shl rcx, 5
0x1401652a0: cmp qword ptr [rax + rcx + 0x20], 0
0x1401652a6: je 0x140165baf
0x1401652ac: add rax, rcx
0x1401652af: mov rcx, qword ptr [rax + 0x10]
0x1401652b3: mov edx, dword ptr [r15 + 0x5b0]
0x1401652ba: mov dword ptr [rax + 0x5c], edx
0x1401652bd: cmp dword ptr [rcx + 0x48], 2
0x1401652c1: je 0x140165bfd
0x1401652c7: minss xmm6, dword ptr [rip + 0x27f82b1]
0x1401652cf: addss xmm6, dword ptr [rcx + 0x44]
0x1401652d4: movss dword ptr [rcx + 0x44], xmm6
0x1401652d9: xor eax, eax
0x1401652db: cmp byte ptr [rip + 0x34c5e04], 7
0x1401652e2: lea r8, [rip + 0x34c5dc7]
0x1401652e9: cmove r8, rax
0x1401652ed: mov byte ptr [rbp + 0x1977], 1
0x1401652f4: movss dword ptr [rsp + 0x28], xmm6
0x1401652fa: lea rax, [rbp + 0x1920]
0x140165301: mov qword ptr [rsp + 0x20], rax
0x140165306: mov byte ptr [rsp + 0x48], 1
0x14016530b: mov byte ptr [rsp + 0x40], 0
0x140165310: mov byte ptr [rsp + 0x38], 0
0x140165315: mov dword ptr [rsp + 0x30], 0
0x14016531d: lea rcx, [rbp + 0x1440]
0x140165324: mov rdx, r15
0x140165327: xor r9d, r9d
0x14016532a: call 0x1408a8d70
0x14016532f: mov byte ptr [rbp + 0x1977], 1
0x140165336: lea rcx, [rbp + 0x1440]
0x14016533d: lea r8, [rbp + 0x1920]
0x140165344: mov rdx, r15
0x140165347: xor r9d, r9d
0x14016534a: call 0x1409c6700
0x14016534f: mov byte ptr [rbp + 0x1977], 1
0x140165356: lea rcx, [rbp + 0x1440]
0x14016535d: lea r8, [rbp + 0x1920]
0x140165364: mov rdx, r15
0x140165367: mov r9, qword ptr [rbp + 0x18c8]
0x14016536e: call 0x140837ff0
0x140165373: movups xmm0, xmmword ptr [rbp + 0x1920]
0x14016537a: movups xmm1, xmmword ptr [rbp + 0x1930]
0x140165381: movaps xmmword ptr [rbp + 0x16f0], xmm0
0x140165388: mov eax, dword ptr [rbp + 0x1940]
0x14016538e: mov dword ptr [rbp + 0x1710], eax
0x140165394: movaps xmmword ptr [rbp + 0x1700], xmm1
0x14016539b: mov r13d, dword ptr [rbp + 0x16f0]
0x1401653a2: mov byte ptr [rbp + 0x1977], 1
0x1401653a9: lea rcx, [rbp + 0x16f0]
0x1401653b0: lea rdx, [rbp + 0x1440]
0x1401653b7: mov r8b, 0x22
0x1401653ba: call 0x140bd6380
0x1401653bf: lea r14, [rbp + 0x16f0]
0x1401653c6: lea rdx, [rbp + 0x1440]
0x1401653cd: mov r8d, 0x198
0x1401653d3: mov rcx, r14
0x1401653d6: call 0x142923c00
0x1401653db: mov byte ptr [rbp + 0x1977], 0
0x1401653e2: lea rcx, [rbp + 0x15e0]
0x1401653e9: mov rdx, r14
0x1401653ec: call 0x1409dda50
0x1401653f1: movabs rdx, 0xa68561b396784eaa
0x1401653fb: movabs r8, 0xf5fd74e6faf4ed14
0x140165405: mov rcx, r12
0x140165408: call 0x1400dd6f0
0x14016540d: test rax, rax
0x140165410: je 0x140165bc7
0x140165416: cmp qword ptr [r15 + 0x218], rdx
0x14016541d: jbe 0x140165bc7
0x140165423: mov rax, qword ptr [r15 + 0x210]
0x14016542a: mov rcx, qword ptr [rax + rdx*8]
0x14016542e: test rcx, rcx
0x140165431: je 0x140165bc7
0x140165437: mov rax, qword ptr [r15 + 0x1e0]
0x14016543e: not rcx
0x140165441: lea rcx, [rcx + rcx*2]
0x140165445: shl rcx, 5
0x140165449: cmp qword ptr [rax + rcx + 0x20], 0
0x14016544f: je 0x140165bc7
0x140165455: add rax, rcx
0x140165458: mov rcx, qword ptr [rax + 0x10]
0x14016545c: mov edx, dword ptr [r15 + 0x5b0]
0x140165463: mov dword ptr [rax + 0x5c], edx
0x140165466: lea rdx, [rbp + 0x15e0]
0x14016546d: call 0x14093ce30
0x140165472: movabs rdx, 0x8226cc786e679355
0x14016547c: movabs r8, 0x65cfda6a55a841ac
0x140165486: mov rcx, r12
0x140165489: call 0x1400dd6f0
0x14016548e: test rax, rax
0x140165491: je 0x140165bd5
0x140165497: cmp qword ptr [r15 + 0x218], rdx
0x14016549e: jbe 0x140165bd5
0x1401654a4: mov rax, qword ptr [r15 + 0x210]
0x1401654ab: mov rcx, qword ptr [rax + rdx*8]
0x1401654af: test rcx, rcx
0x1401654b2: je 0x140165bd5
0x1401654b8: mov rax, qword ptr [r15 + 0x1e0]
0x1401654bf: not rcx
0x1401654c2: lea rcx, [rcx + rcx*2]
0x1401654c6: shl rcx, 5
0x1401654ca: cmp qword ptr [rax + rcx + 0x20], 0
0x1401654d0: je 0x140165bd5
0x1401654d6: add rax, rcx
0x1401654d9: mov rcx, qword ptr [rax + 0x10]
0x1401654dd: mov edx, dword ptr [r15 + 0x5b0]
0x1401654e4: mov dword ptr [rax + 0x5c], edx
0x1401654e7: lea rdx, [rbp + 0x15e0]
0x1401654ee: call 0x14091ebb0
0x1401654f3: lea r14, [rbp - 0x30]
0x1401654f7: lea rdx, [rbp + 0x15e0]
0x1401654fe: mov r8d, 0x110
0x140165504: mov rcx, r14
0x140165507: call 0x142923c00
0x14016550c: lea rcx, [rbp + 0xe0]
0x140165513: mov rdx, r14
0x140165516: call 0x1404f7ca0
0x14016551b: lea rsi, [rdi + 0x90]
0x140165522: lea rcx, [rbp + 0x15e0]
0x140165529: lea rdx, [rbp + 0xe0]
0x140165530: mov r8d, 0xa8
0x140165536: call 0x142923c00
0x14016553b: movups xmm0, xmmword ptr [rbp + 0x188]
0x140165542: movups xmm1, xmmword ptr [rbp + 0x198]
0x140165549: movups xmm2, xmmword ptr [rbp + 0x1a8]
0x140165550: movups xmm3, xmmword ptr [rbp + 0x1b8]
0x140165557: movaps xmmword ptr [rbp + 0x13e0], xmm0
0x14016555e: movaps xmmword ptr [rbp + 0x13f0], xmm1
0x140165565: movaps xmmword ptr [rbp + 0x1400], xmm2
0x14016556c: movaps xmmword ptr [rbp + 0x1410], xmm3
0x140165573: movups xmm0, xmmword ptr [rbp + 0x1c8]
0x14016557a: movaps xmmword ptr [rbp + 0x1420], xmm0
0x140165581: movups xmm0, xmmword ptr [rbp + 0x1d8]
0x140165588: movaps xmmword ptr [rbp + 0x1430], xmm0
0x14016558f: movups xmm0, xmmword ptr [rsi + 0x50]
0x140165593: movaps xmmword ptr [rbp + 0x130], xmm0
0x14016559a: movups xmm0, xmmword ptr [rsi + 0x40]
0x14016559e: movaps xmmword ptr [rbp + 0x120], xmm0
0x1401655a5: movups xmm0, xmmword ptr [rsi]
0x1401655a8: movups xmm1, xmmword ptr [rsi + 0x10]
0x1401655ac: movups xmm2, xmmword ptr [rsi + 0x20]
0x1401655b0: movups xmm3, xmmword ptr [rsi + 0x30]
0x1401655b4: movaps xmmword ptr [rbp + 0x110], xmm3
0x1401655bb: movaps xmmword ptr [rbp + 0x100], xmm2
0x1401655c2: movaps xmmword ptr [rbp + 0xf0], xmm1
0x1401655c9: movaps xmmword ptr [rbp + 0xe0], xmm0
0x1401655d0: movaps xmm0, xmmword ptr [rbp + 0x13e0]
0x1401655d7: movaps xmm1, xmmword ptr [rbp + 0x13f0]
0x1401655de: movaps xmm2, xmmword ptr [rbp + 0x1400]
0x1401655e5: movaps xmm3, xmmword ptr [rbp + 0x1410]
0x1401655ec: movups xmmword ptr [rsi], xmm0
0x1401655ef: movups xmmword ptr [rsi + 0x10], xmm1
0x1401655f3: movups xmmword ptr [rsi + 0x20], xmm2
0x1401655f7: movups xmmword ptr [rsi + 0x30], xmm3
0x1401655fb: movaps xmm0, xmmword ptr [rbp + 0x1420]
0x140165602: movups xmmword ptr [rsi + 0x40], xmm0
0x140165606: movaps xmm0, xmmword ptr [rbp + 0x1430]
0x14016560d: movups xmmword ptr [rsi + 0x50], xmm0
0x140165611: cmp qword ptr [rbp + 0xe0], 0
0x140165619: je 0x140165672
0x14016561b: lea rdx, [rbp + 0x100]
0x140165622: lea rcx, [rbp + 0xe0]
0x140165629: mov r8d, 0x68
0x14016562f: mov r9d, 0x10
0x140165635: call 0x14004ceb0
0x14016563a: mov rdx, qword ptr [rbp + 0x118]
0x140165641: test rdx, rdx
0x140165644: je 0x140165672
0x140165646: mov rax, rdx
0x140165649: shl rax, 4
0x14016564d: add rdx, rax
0x140165650: add rdx, 0x21
0x140165654: je 0x140165672
0x140165656: mov rcx, qword ptr [rbp + 0x110]
0x14016565d: neg rax
0x140165660: add rcx, rax
0x140165663: add rcx, -0x10
0x140165667: mov r8d, 0x10
0x14016566d: call 0x140613c20
0x140165672: lea rcx, [rbp + 0x1350]
0x140165679: lea rdx, [rbp + 0x15e0]
0x140165680: mov r8d, 0x90
0x140165686: call 0x142923c00
0x14016568b: mov rax, qword ptr [rbp + 0x1680]
0x140165692: mov qword ptr [rbp + 0x1900], rax
0x140165699: movups xmm0, xmmword ptr [rbp + 0x1670]
0x1401656a0: movaps xmmword ptr [rbp + 0x18f0], xmm0
0x1401656a7: mov qword ptr [rbp + 0xe0], 0
0x1401656b2: mov qword ptr [rbp + 0xe8], 8
0x1401656bd: mov qword ptr [rbp + 0xf0], 0
0x1401656c8: xor ecx, ecx
0x1401656ca: call 0x1400d9b20
0x1401656cf: test rax, rax
0x1401656d2: je 0x140165c1e
0x1401656d8: movups xmm0, xmmword ptr [rax]
0x1401656db: inc qword ptr [rax]
0x1401656de: movups xmm1, xmmword ptr [rip + 0x284a94b]
0x1401656e5: movups xmmword ptr [rbp + 0x1528], xmm1
0x1401656ec: movups xmm1, xmmword ptr [rip + 0x284a92d]
0x1401656f3: movups xmmword ptr [rbp + 0x1518], xmm1
0x1401656fa: movups xmmword ptr [rbp + 0x1538], xmm0
0x140165701: mov qword ptr [rbp + 0x1440], 0
0x14016570c: mov qword ptr [rbp + 0x1448], 2
0x140165717: xorps xmm0, xmm0
0x14016571a: movups xmmword ptr [rbp + 0x1450], xmm0
0x140165721: mov qword ptr [rbp + 0x1460], 1
0x14016572c: mov qword ptr [rbp + 0x1468], 0
0x140165737: movups xmm1, xmmword ptr [rbp + 0xe0]
0x14016573e: movups xmmword ptr [rbp + 0x1470], xmm1
0x140165745: mov rax, qword ptr [rbp + 0xf0]
0x14016574c: mov qword ptr [rbp + 0x1480], rax
0x140165753: lea rcx, [rbp + 0x1488]
0x14016575a: mov qword ptr [rbp + 0x1488], 0
0x140165765: mov qword ptr [rbp + 0x1490], 4
0x140165770: movups xmmword ptr [rbp + 0x1498], xmm0
0x140165777: mov qword ptr [rbp + 0x14a8], 4
0x140165782: movups xmmword ptr [rbp + 0x14b0], xmm0
0x140165789: mov qword ptr [rbp + 0x14c0], 4
0x140165794: movups xmmword ptr [rbp + 0x14c8], xmm0
0x14016579b: mov qword ptr [rbp + 0x14d8], 4
0x1401657a6: movups xmmword ptr [rbp + 0x14e0], xmm0
0x1401657ad: mov qword ptr [rbp + 0x14f0], 4
0x1401657b8: movups xmmword ptr [rbp + 0x14f8], xmm0
0x1401657bf: mov qword ptr [rbp + 0x1508], 4
0x1401657ca: mov qword ptr [rbp + 0x1510], 0
0x1401657d5: mov r8, qword ptr [rbp + 0x18f8]
0x1401657dc: mov r9, qword ptr [rbp + 0x1900]
0x1401657e3: mov byte ptr [rbp + 0x1972], 1
0x1401657ea: mov byte ptr [rbp + 0x1971], 1
0x1401657f1: lea rdx, [rbp + 0x1350]
0x1401657f8: call 0x140530430
0x1401657fd: mov byte ptr [rbp + 0x1972], 1
0x140165804: mov byte ptr [rbp + 0x1971], 1
0x14016580b: lea rcx, [rbp + 0x1440]
0x140165812: lea rdx, [rbp + 0x1350]
0x140165819: mov r8, qword ptr [rbp + 0x18d0]
0x140165820: mov r9, qword ptr [rbp + 0x18d8]
0x140165827: call 0x140be7ba0
0x14016582c: test al, al
0x14016582e: je 0x1401658e3
0x140165834: mov byte ptr [rbp + 0x1972], 1
0x14016583b: mov byte ptr [rbp + 0x1971], 1
0x140165842: lea rcx, [rbp + 0x1350]
0x140165849: call 0x140be9420
0x14016584e: lea rcx, [rbp + 0xe0]
0x140165855: mov r8d, 0x90
0x14016585b: mov rdx, rdi
0x14016585e: call 0x142923c00
0x140165863: lea rdx, [rbp + 0x1350]
0x14016586a: mov r8d, 0x90
0x140165870: mov rcx, rdi
0x140165873: call 0x142923c00
0x140165878: xor eax, eax
0x14016587a: cmp rax, qword ptr [rbp + 0xe0]
0x140165881: jo 0x14016589d
0x140165883: mov byte ptr [rbp + 0x1972], 0
0x14016588a: mov byte ptr [rbp + 0x1971], 1
0x140165891: lea rcx, [rbp + 0xe0]
0x140165898: call 0x1405ef210
0x14016589d: mov rsi, qword ptr [rbp + 0x18e8]
0x1401658a4: movabs rax, 0x8000000000000000
0x1401658ae: mov qword ptr [rsi], rax
0x1401658b1: mov byte ptr [rbp + 0x1974], 0
0x1401658b8: mov byte ptr [rbp + 0x1973], 1
0x1401658bf: lea rcx, [rbp + 0x1440]
0x1401658c6: call 0x1405fd1d0
0x1401658cb: mov byte ptr [rbp + 0x196d], 0
0x1401658d2: lea rcx, [rbp + 0x18f0]
0x1401658d9: call 0x1405da820
0x1401658de: jmp 0x140165981
0x1401658e3: lea rdi, [rbp + 0x1220]
0x1401658ea: lea rdx, [rbp + 0x1350]
0x1401658f1: mov r8d, 0x90
0x1401658f7: mov rcx, rdi
0x1401658fa: call 0x142923c00
0x1401658ff: mov rax, qword ptr [rbp + 0x1900]
0x140165906: mov qword ptr [rbp + 0x1930], rax
0x14016590d: movaps xmm0, xmmword ptr [rbp + 0x18f0]
0x140165914: movaps xmmword ptr [rbp + 0x1920], xmm0
0x14016591b: lea rbx, [rbp + 0x16f0]
0x140165922: lea rdx, [rbp + 0x1440]
0x140165929: mov r8d, 0x108
0x14016592f: mov rcx, rbx
0x140165932: call 0x142923c00
0x140165937: mov byte ptr [rbp + 0x1972], 0
0x14016593e: mov byte ptr [rbp + 0x1971], 0
0x140165945: lea rcx, [rbp + 0xe0]
0x14016594c: lea r8, [rbp + 0x1920]
0x140165953: mov rdx, rdi
0x140165956: mov r9, rbx
0x140165959: call 0x1402c4280
0x14016595e: lea rdx, [rbp + 0xe0]
0x140165965: mov r8d, 0x218
0x14016596b: mov rsi, qword ptr [rbp + 0x18e8]
0x140165972: mov rcx, rsi
0x140165975: call 0x142923c00
0x14016597a: mov dword ptr [rsi + 0x218], r13d
0x140165981: mov rax, rsi
0x140165984: movaps xmm6, xmmword ptr [rbp + 0x1980]
0x14016598b: add rsp, 0x1a18
0x140165992: pop rbx
0x140165993: pop rdi
0x140165994: pop rsi
0x140165995: pop r12
0x140165997: pop r13
0x140165999: pop r14
0x14016599b: pop r15
0x14016599d: pop rbp
0x14016599e: ret 
0x14016599f: movabs rcx, 0x7ffffffffffffffc
0x1401659a9: add rax, rcx
0x1401659ac: cmp rax, 6
0x1401659b0: mov ecx, 4
0x1401659b5: cmovb rcx, rax
0x1401659b9: cmp rcx, 4
0x1401659bd: ja 0x140165a65
0x1401659c3: lea rax, [rip + 0x27f80ba]
0x1401659ca: movsxd rcx, dword ptr [rax + rcx*4]
0x1401659ce: add rcx, rax
0x1401659d1: jmp rcx
0x1401659d3: lea rcx, [rbp + 0x16f8]
0x1401659da: mov byte ptr [rbp + 0x1977], 1
0x1401659e1: call 0x1405ea740
0x1401659e6: jmp 0x1401650a5
0x1401659eb: lea rdx, [rax - 9]
0x1401659ef: lea r8, [rax - 8]
0x1401659f3: xor ecx, ecx
0x1401659f5: cmp rdx, 2
0x1401659f9: cmovb rcx, r8
0x1401659fd: test rcx, rcx
0x140165a00: je 0x140165a7d
0x140165a02: cmp rcx, 1
0x140165a06: jne 0x140165ac1
0x140165a0c: lea rcx, [rbp + 0x16f8]
0x140165a13: call 0x1405ebcf0
0x140165a18: jmp 0x140165ae2
0x140165a1d: mov byte ptr [rbp + 0x1977], 1
0x140165a24: lea rcx, [rbp + 0x16f0]
0x140165a2b: call 0x1405ed2c0
0x140165a30: jmp 0x1401650a5
0x140165a35: lea rcx, [rbp + 0x16f8]
0x140165a3c: mov byte ptr [rbp + 0x1977], 1
0x140165a43: call 0x1405ea740
0x140165a48: jmp 0x1401650a5
0x140165a4d: lea rcx, [rbp + 0x16f8]
0x140165a54: mov byte ptr [rbp + 0x1977], 1
0x140165a5b: call 0x1405eb290
0x140165a60: jmp 0x1401650a5
0x140165a65: lea rcx, [rbp + 0x16f8]
0x140165a6c: mov byte ptr [rbp + 0x1977], 1
0x140165a73: call 0x1405e7a00
0x140165a78: jmp 0x1401650a5
0x140165a7d: cmp rax, 5
0x140165a81: ja 0x140165aac
0x140165a83: mov ecx, 0x32
0x140165a88: bt rcx, rax
0x140165a8c: jae 0x140165aac
0x140165a8e: mov rdx, qword ptr [rbp + 0x16f8]
0x140165a95: test rdx, rdx
0x140165a98: je 0x140165aac
0x140165a9a: mov rcx, qword ptr [rbp + 0x1700]
0x140165aa1: mov r8d, 1
0x140165aa7: call 0x140613c20
0x140165aac: mov rdx, qword ptr [rbp + 0x1710]
0x140165ab3: test rdx, rdx
0x140165ab6: je 0x140165ae2
0x140165ab8: lea rax, [rbp + 0x1718]
0x140165abf: jmp 0x140165ad4
0x140165ac1: mov rdx, qword ptr [rbp + 0x16f8]
0x140165ac8: test rdx, rdx
0x140165acb: je 0x140165ae2
0x140165acd: lea rax, [rbp + 0x1700]
0x140165ad4: mov rcx, qword ptr [rax]
0x140165ad7: mov r8d, 1
0x140165add: call 0x140613c20
0x140165ae2: mov rdx, qword ptr [rbp + 0x1958]
0x140165ae9: test rdx, rdx
0x140165aec: je 0x1401650a5
0x140165af2: mov r8d, 1
0x140165af8: mov rcx, qword ptr [rbp + 0x1918]
0x140165aff: call 0x140613c20
0x140165b04: jmp 0x1401650a5
0x140165b09: mov byte ptr [rbp + 0x1976], 1
0x140165b10: mov rcx, rsi
0x140165b13: mov r8d, 0x3b9aca00
0x140165b19: call 0x1428cd210
0x140165b1e: jmp 0x140164af8
0x140165b23: lea r8, [rip + 0x2878d86]
0x140165b2a: mov edx, 0x1090
0x140165b2f: call 0x1428d98e0
0x140165b34: jmp 0x140165c52
0x140165b39: mov byte ptr [rbp + 0x1970], 1
0x140165b40: mov rcx, rsi
0x140165b43: xor edx, edx
0x140165b45: call 0x1428cdb90
0x140165b4a: jmp 0x140164be0
0x140165b4f: mov byte ptr [rbp + 0x1975], 1
0x140165b56: lea rcx, [rip + 0x27f7dcb]
0x140165b5d: call 0x1428d9310
0x140165b62: jmp 0x140165c52
0x140165b67: mov byte ptr [rbp + 0x1976], 1
0x140165b6e: lea rcx, [rip + 0x27f7dcb]
0x140165b75: call 0x1428d9310
0x140165b7a: jmp 0x140165c52
0x140165b7f: mov byte ptr [rbp + 0x1976], 1
0x140165b86: lea rcx, [rip + 0x27f7dcb]
0x140165b8d: call 0x1428d9310
0x140165b92: jmp 0x140165c52
0x140165b97: mov byte ptr [rbp + 0x1977], 1
0x140165b9e: lea rcx, [rip + 0x27f7ddb]
0x140165ba5: call 0x1428d9310
0x140165baa: jmp 0x140165c52
0x140165baf: mov byte ptr [rbp + 0x1977], 1
0x140165bb6: lea rcx, [rip + 0x27f7ddb]
0x140165bbd: call 0x1428d9310
0x140165bc2: jmp 0x140165c52
0x140165bc7: lea rcx, [rip + 0x27f7de2]
0x140165bce: call 0x1428d9310
0x140165bd3: jmp 0x140165c52
0x140165bd5: lea rcx, [rip + 0x27f7dec]
0x140165bdc: call 0x1428d9310
0x140165be1: jmp 0x140165c52
0x140165be3: lea rcx, [rip + 0x27f79ee]
0x140165bea: call 0x1428d9310
0x140165bef: lea rcx, [rip + 0x27f79fa]
0x140165bf6: call 0x1428d9310
0x140165bfb: jmp 0x140165c52
0x140165bfd: mov byte ptr [rbp + 0x1977], 1
0x140165c04: lea rcx, [rip + 0x27f7dd5]
0x140165c0b: lea r8, [rip + 0x27f7df6]
0x140165c12: mov edx, 0x28
0x140165c17: call 0x1428d9430
0x140165c1c: jmp 0x140165c52
0x140165c1e: lea rcx, [rip + 0x27ccdeb]
0x140165c25: call 0x1428d80d0
0x140165c2a: jmp 0x140165c52
0x140165c2c: lea rcx, [rip + 0x27f7dfd]
0x140165c33: call 0x1428d9310
0x140165c38: jmp 0x140165c52
0x140165c3a: lea rcx, [rip + 0x27f7d9f]
0x140165c41: lea r8, [rip + 0x27f7e00]
0x140165c48: mov edx, 0x28
0x140165c4d: call 0x1428d9430
0x140165c52: ud2 
0x140165c54: nop word ptr cs:[rax + rax]
0x140165c60: mov qword ptr [rsp + 0x10], rdx
0x140165c65: push rbp
0x140165c66: push r15
0x140165c68: push r14
0x140165c6a: push r13
0x140165c6c: push r12
0x140165c6e: push rsi
0x140165c6f: push rdi
0x140165c70: push rbx
0x140165c71: sub rsp, 0x68
0x140165c75: lea rbp, [rdx + 0x80]
0x140165c7c: movaps xmmword ptr [rsp + 0x50], xmm6
0x140165c81: mov rax, qword ptr [rbp + 0x1610]
0x140165c88: mov rcx, qword ptr [rbp + 0x1960]
0x140165c8f: mov qword ptr [rcx + 0x30], rax
0x140165c93: movaps xmm0, xmmword ptr [rbp + 0x15e0]
0x140165c9a: movaps xmm1, xmmword ptr [rbp + 0x15f0]
0x140165ca1: movaps xmm2, xmmword ptr [rbp + 0x1600]
0x140165ca8: movups xmmword ptr [rcx + 0x20], xmm2
0x140165cac: movups xmmword ptr [rcx + 0x10], xmm1
0x140165cb0: movups xmmword ptr [rcx], xmm0
0x140165cb3: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140165cb8: add rsp, 0x68
0x140165cbc: pop rbx
0x140165cbd: pop rdi
0x140165cbe: pop rsi
0x140165cbf: pop r12
0x140165cc1: pop r13
0x140165cc3: pop r14
0x140165cc5: pop r15
0x140165cc7: pop rbp
0x140165cc8: ret 
0x140165cc9: nop dword ptr [rax]
0x140165cd0: mov qword ptr [rsp + 0x10], rdx
0x140165cd5: push rbp
0x140165cd6: push r15
0x140165cd8: push r14
0x140165cda: push r13
0x140165cdc: push r12
0x140165cde: push rsi
0x140165cdf: push rdi
0x140165ce0: push rbx
0x140165ce1: sub rsp, 0x68
0x140165ce5: lea rbp, [rdx + 0x80]
0x140165cec: movaps xmmword ptr [rsp + 0x50], xmm6
0x140165cf1: lea rcx, [rbp + 0x15e0]
0x140165cf8: call 0x140161c60
0x140165cfd: mov byte ptr [rbp + 0x1977], 1
0x140165d04: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140165d09: add rsp, 0x68
0x140165d0d: pop rbx
0x140165d0e: pop rdi
0x140165d0f: pop rsi
0x140165d10: pop r12
0x140165d12: pop r13
0x140165d14: pop r14
0x140165d16: pop r15
0x140165d18: pop rbp
0x140165d19: ret 
0x140165d1a: nop word ptr [rax + rax]
0x140165d20: mov qword ptr [rsp + 0x10], rdx
0x140165d25: push rbp
0x140165d26: push r15
0x140165d28: push r14
0x140165d2a: push r13
0x140165d2c: push r12
0x140165d2e: push rsi
0x140165d2f: push rdi
0x140165d30: push rbx
0x140165d31: sub rsp, 0x68
0x140165d35: lea rbp, [rdx + 0x80]
0x140165d3c: movaps xmmword ptr [rsp + 0x50], xmm6
0x140165d41: cmp qword ptr [rbp + 0x1958], 0
0x140165d49: je 0x140165d64
0x140165d4b: mov r8d, 1
0x140165d51: mov rcx, qword ptr [rbp + 0x1918]
0x140165d58: mov rdx, qword ptr [rbp + 0x1958]
0x140165d5f: call 0x140613c20
0x140165d64: mov byte ptr [rbp + 0x196f], 0
0x140165d6b: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140165d70: add rsp, 0x68
0x140165d74: pop rbx
0x140165d75: pop rdi
0x140165d76: pop rsi
0x140165d77: pop r12
0x140165d79: pop r13
0x140165d7b: pop r14
0x140165d7d: pop r15
0x140165d7f: pop rbp
0x140165d80: ret 
0x140165d81: nop word ptr cs:[rax + rax]
0x140165d90: mov qword ptr [rsp + 0x10], rdx
0x140165d95: push rbp
0x140165d96: push r15
0x140165d98: push r14
0x140165d9a: push r13
0x140165d9c: push r12
0x140165d9e: push rsi
0x140165d9f: push rdi
0x140165da0: push rbx
0x140165da1: sub rsp, 0x68
0x140165da5: lea rbp, [rdx + 0x80]
0x140165dac: movaps xmmword ptr [rsp + 0x50], xmm6
0x140165db1: lea rcx, [rbp + 0x1920]
0x140165db8: call 0x1405ee5e0
0x140165dbd: mov byte ptr [rbp + 0x196f], 0
0x140165dc4: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140165dc9: add rsp, 0x68
0x140165dcd: pop rbx
0x140165dce: pop rdi
0x140165dcf: pop rsi
0x140165dd0: pop r12
0x140165dd2: pop r13
0x140165dd4: pop r14
0x140165dd6: pop r15
0x140165dd8: pop rbp
0x140165dd9: ret 
0x140165dda: nop word ptr [rax + rax]
0x140165de0: mov qword ptr [rsp + 0x10], rdx
0x140165de5: push rbp
0x140165de6: push r15
0x140165de8: push r14
0x140165dea: push r13
0x140165dec: push r12
0x140165dee: push rsi
0x140165def: push rdi
0x140165df0: push rbx
0x140165df1: sub rsp, 0x68
0x140165df5: lea rbp, [rdx + 0x80]
0x140165dfc: movaps xmmword ptr [rsp + 0x50], xmm6
0x140165e01: cmp byte ptr [rbp + 0x196f], 0
0x140165e08: je 0x140165e45
0x140165e0a: mov rdx, qword ptr [rbp + 0x18a0]
0x140165e11: test rdx, rdx
0x140165e14: je 0x140165e28
0x140165e16: mov rcx, qword ptr [rbp + 0x18a8]
0x140165e1d: mov r8d, 1
0x140165e23: call 0x140613c20
0x140165e28: mov byte ptr [rbp + 0x1977], 1
0x140165e2f: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140165e34: add rsp, 0x68
0x140165e38: pop rbx
0x140165e39: pop rdi
0x140165e3a: pop rsi
0x140165e3b: pop r12
0x140165e3d: pop r13
0x140165e3f: pop r14
0x140165e41: pop r15
0x140165e43: pop rbp
0x140165e44: ret 
0x140165e45: mov byte ptr [rbp + 0x1977], 1
0x140165e4c: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140165e51: add rsp, 0x68
0x140165e55: pop rbx
0x140165e56: pop rdi
0x140165e57: pop rsi
0x140165e58: pop r12
0x140165e5a: pop r13
0x140165e5c: pop r14
0x140165e5e: pop r15
0x140165e60: pop rbp
0x140165e61: ret 
0x140165e62: nop word ptr cs:[rax + rax]
0x140165e70: mov qword ptr [rsp + 0x10], rdx
0x140165e75: push rbp
0x140165e76: push r15
0x140165e78: push r14
0x140165e7a: push r13
0x140165e7c: push r12
0x140165e7e: push rsi
0x140165e7f: push rdi
0x140165e80: push rbx
0x140165e81: sub rsp, 0x68
0x140165e85: lea rbp, [rdx + 0x80]
0x140165e8c: movaps xmmword ptr [rsp + 0x50], xmm6
0x140165e91: lea rcx, [rip + 0x34c5218]
0x140165e98: call 0x140be24b0
0x140165e9d: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140165ea2: add rsp, 0x68
0x140165ea6: pop rbx
0x140165ea7: pop rdi
0x140165ea8: pop rsi
0x140165ea9: pop r12
0x140165eab: pop r13
0x140165ead: pop r14
0x140165eaf: pop r15
0x140165eb1: pop rbp
0x140165eb2: ret 
0x140165eb3: nop word ptr cs:[rax + rax]
0x140165ec0: mov qword ptr [rsp + 0x10], rdx
0x140165ec5: push rbp
0x140165ec6: push r15
0x140165ec8: push r14
0x140165eca: push r13
0x140165ecc: push r12
0x140165ece: push rsi
0x140165ecf: push rdi
0x140165ed0: push rbx
0x140165ed1: sub rsp, 0x68
0x140165ed5: lea rbp, [rdx + 0x80]
0x140165edc: movaps xmmword ptr [rsp + 0x50], xmm6
0x140165ee1: mov rax, qword ptr [rbp + 0x171e]
0x140165ee8: mov qword ptr [rip + 0x34c51ef], rax
0x140165eef: movaps xmm0, xmmword ptr [rbp + 0x16f0]
0x140165ef6: movaps xmm1, xmmword ptr [rbp + 0x1700]
0x140165efd: movaps xmm2, xmmword ptr [rbp + 0x1710]
0x140165f04: movups xmmword ptr [rip + 0x34c51c5], xmm2
0x140165f0b: movups xmmword ptr [rip + 0x34c51ae], xmm1
0x140165f12: movups xmmword ptr [rip + 0x34c5197], xmm0
0x140165f19: movzx eax, byte ptr [rbp + 0x1960]
0x140165f20: mov byte ptr [rip + 0x34c51c0], al
0x140165f26: movzx eax, byte ptr [rbp + 0x1958]
0x140165f2d: mov byte ptr [rip + 0x34c51b4], al
0x140165f33: mov byte ptr [rbp + 0x1977], 1
0x140165f3a: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140165f3f: add rsp, 0x68
0x140165f43: pop rbx
0x140165f44: pop rdi
0x140165f45: pop rsi
0x140165f46: pop r12
0x140165f48: pop r13
0x140165f4a: pop r14
0x140165f4c: pop r15
0x140165f4e: pop rbp
0x140165f4f: ret 
0x140165f50: mov qword ptr [rsp + 0x10], rdx
0x140165f55: push rbp
0x140165f56: push r15
0x140165f58: push r14
0x140165f5a: push r13
0x140165f5c: push r12
0x140165f5e: push rsi
0x140165f5f: push rdi
0x140165f60: push rbx
0x140165f61: sub rsp, 0x68
0x140165f65: lea rbp, [rdx + 0x80]
0x140165f6c: movaps xmmword ptr [rsp + 0x50], xmm6
0x140165f71: lea rcx, [rbp + 0x110]
0x140165f78: lea rdx, [rbp + 0x130]
0x140165f7f: mov r8d, 0x10
0x140165f85: mov r9d, 0x10
0x140165f8b: call 0x14004c400
0x140165f90: mov byte ptr [rbp + 0x196e], 1
0x140165f97: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140165f9c: add rsp, 0x68
0x140165fa0: pop rbx
0x140165fa1: pop rdi
0x140165fa2: pop rsi
0x140165fa3: pop r12
0x140165fa5: pop r13
0x140165fa7: pop r14
0x140165fa9: pop r15
0x140165fab: pop rbp
0x140165fac: ret 
0x140165fad: nop dword ptr [rax]
0x140165fb0: mov qword ptr [rsp + 0x10], rdx
0x140165fb5: push rbp
0x140165fb6: push r15
0x140165fb8: push r14
0x140165fba: push r13
0x140165fbc: push r12
0x140165fbe: push rsi
0x140165fbf: push rdi
0x140165fc0: push rbx
0x140165fc1: sub rsp, 0x68
0x140165fc5: lea rbp, [rdx + 0x80]
0x140165fcc: movaps xmmword ptr [rsp + 0x50], xmm6
0x140165fd1: mov byte ptr [rbp + 0x1974], 1
0x140165fd8: mov byte ptr [rbp + 0x1973], 1
0x140165fdf: lea rcx, [rbp + 0xe0]
0x140165fe6: call 0x14060b5b0
0x140165feb: mov byte ptr [rbp + 0x1974], 1
0x140165ff2: mov byte ptr [rbp + 0x1973], 1
0x140165ff9: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140165ffe: add rsp, 0x68
0x140166002: pop rbx
0x140166003: pop rdi
0x140166004: pop rsi
0x140166005: pop r12
0x140166007: pop r13
0x140166009: pop r14
0x14016600b: pop r15
0x14016600d: pop rbp
0x14016600e: ret 
0x14016600f: nop 
0x140166010: mov qword ptr [rsp + 0x10], rdx
0x140166015: push rbp
0x140166016: push r15
0x140166018: push r14
0x14016601a: push r13
0x14016601c: push r12
0x14016601e: push rsi
0x14016601f: push rdi
0x140166020: push rbx
0x140166021: sub rsp, 0x68
0x140166025: lea rbp, [rdx + 0x80]
0x14016602c: movaps xmmword ptr [rsp + 0x50], xmm6
0x140166031: movzx ebx, byte ptr [rbp + 0x1971]
0x140166038: movzx esi, byte ptr [rbp + 0x1972]
0x14016603f: test bl, 1
0x140166042: je 0x140166077
0x140166044: lea rcx, [rbp + 0x1440]
0x14016604b: call 0x1405fd1d0
0x140166050: and sil, 1
0x140166054: mov byte ptr [rbp + 0x1974], sil
0x14016605b: mov byte ptr [rbp + 0x1973], bl
0x140166061: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140166066: add rsp, 0x68
0x14016606a: pop rbx
0x14016606b: pop rdi
0x14016606c: pop rsi
0x14016606d: pop r12
0x14016606f: pop r13
0x140166071: pop r14
0x140166073: pop r15
0x140166075: pop rbp
0x140166076: ret 
0x140166077: and sil, 1
0x14016607b: mov byte ptr [rbp + 0x1974], sil
0x140166082: mov byte ptr [rbp + 0x1973], bl
0x140166088: movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016608d: add rsp, 0x68
0x140166091: pop rbx
0x140166092: pop rdi
0x140166093: pop rsi
0x140166094: pop r12
0x140166096: pop r13
0x140166098: pop r14
0x14016609a: pop r15
0x14016609c: pop rbp
0x14016609d: ret 
0x14016609e: nop 
0x1401660a0: mov qword ptr [rsp + 0x10], rdx
0x1401660a5: push rbp
0x1401660a6: push r15
0x1401660a8: push r14
0x1401660aa: push r13
0x1401660ac: push r12
0x1401660ae: push rsi
0x1401660af: push rdi
0x1401660b0: push rbx
0x1401660b1: sub rsp, 0x68
0x1401660b5: lea rbp, [rdx + 0x80]
0x1401660bc: movaps xmmword ptr [rsp + 0x50], xmm6
0x1401660c1: movzx ebx, byte ptr [rbp + 0x1974]
0x1401660c8: test byte ptr [rbp + 0x1973], 1
0x1401660cf: je 0x1401660fc
0x1401660d1: lea rcx, [rbp + 0x18f0]
0x1401660d8: call 0x1405da820
0x1401660dd: and bl, 1
0x1401660e0: mov byte ptr [rbp + 0x196d], bl
0x1401660e6: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401660eb: add rsp, 0x68
0x1401660ef: pop rbx
0x1401660f0: pop rdi
0x1401660f1: pop rsi
0x1401660f2: pop r12
0x1401660f4: pop r13
0x1401660f6: pop r14
0x1401660f8: pop r15
0x1401660fa: pop rbp
0x1401660fb: ret 
0x1401660fc: and bl, 1
0x1401660ff: mov byte ptr [rbp + 0x196d], bl
0x140166105: movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016610a: add rsp, 0x68
0x14016610e: pop rbx
0x14016610f: pop rdi
0x140166110: pop rsi
0x140166111: pop r12
0x140166113: pop r13
0x140166115: pop r14
0x140166117: pop r15
0x140166119: pop rbp
0x14016611a: ret 
0x14016611b: nop dword ptr [rax + rax]
0x140166120: mov qword ptr [rsp + 0x10], rdx
0x140166125: push rbp
0x140166126: push r15
0x140166128: push r14
0x14016612a: push r13
0x14016612c: push r12
0x14016612e: push rsi
0x14016612f: push rdi
0x140166130: push rbx
0x140166131: sub rsp, 0x68
0x140166135: lea rbp, [rdx + 0x80]
0x14016613c: movaps xmmword ptr [rsp + 0x50], xmm6
0x140166141: cmp byte ptr [rbp + 0x196d], 0
0x140166148: je 0x140166173
0x14016614a: lea rcx, [rbp + 0x1350]
0x140166151: call 0x1405ef210
0x140166156: mov byte ptr [rbp + 0x196e], 0
0x14016615d: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140166162: add rsp, 0x68
0x140166166: pop rbx
0x140166167: pop rdi
0x140166168: pop rsi
0x140166169: pop r12
0x14016616b: pop r13
0x14016616d: pop r14
0x14016616f: pop r15
0x140166171: pop rbp
0x140166172: ret 
0x140166173: mov byte ptr [rbp + 0x196e], 0
0x14016617a: movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016617f: add rsp, 0x68
0x140166183: pop rbx
0x140166184: pop rdi
0x140166185: pop rsi
0x140166186: pop r12
0x140166188: pop r13
0x14016618a: pop r14
0x14016618c: pop r15
0x14016618e: pop rbp
0x14016618f: ret 
0x140166190: mov qword ptr [rsp + 0x10], rdx
0x140166195: push rbp
0x140166196: push r15
0x140166198: push r14
0x14016619a: push r13
0x14016619c: push r12
0x14016619e: push rsi
0x14016619f: push rdi
0x1401661a0: push rbx
0x1401661a1: sub rsp, 0x68
0x1401661a5: lea rbp, [rdx + 0x80]
0x1401661ac: movaps xmmword ptr [rsp + 0x50], xmm6
0x1401661b1: cmp byte ptr [rbp + 0x196e], 0
0x1401661b8: je 0x1401661dc
0x1401661ba: lea rcx, [rbp + 0x15e0]
0x1401661c1: call 0x1405daf00
0x1401661c6: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401661cb: add rsp, 0x68
0x1401661cf: pop rbx
0x1401661d0: pop rdi
0x1401661d1: pop rsi
0x1401661d2: pop r12
0x1401661d4: pop r13
0x1401661d6: pop r14
0x1401661d8: pop r15
0x1401661da: pop rbp
0x1401661db: ret 
0x1401661dc: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401661e1: add rsp, 0x68
0x1401661e5: pop rbx
0x1401661e6: pop rdi
0x1401661e7: pop rsi
0x1401661e8: pop r12
0x1401661ea: pop r13
0x1401661ec: pop r14
0x1401661ee: pop r15
0x1401661f0: pop rbp
0x1401661f1: ret 
0x1401661f2: nop word ptr cs:[rax + rax]
0x140166200: mov qword ptr [rsp + 0x10], rdx
0x140166205: push rbp
0x140166206: push r15
0x140166208: push r14
0x14016620a: push r13
0x14016620c: push r12
0x14016620e: push rsi
0x14016620f: push rdi
0x140166210: push rbx
0x140166211: sub rsp, 0x68
0x140166215: lea rbp, [rdx + 0x80]
0x14016621c: movaps xmmword ptr [rsp + 0x50], xmm6
0x140166221: lea rcx, [rbp + 0x15e0]
0x140166228: call 0x1405dcdc0
0x14016622d: mov byte ptr [rbp + 0x1977], 0
0x140166234: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140166239: add rsp, 0x68
0x14016623d: pop rbx
0x14016623e: pop rdi
0x14016623f: pop rsi
0x140166240: pop r12
0x140166242: pop r13
0x140166244: pop r14
0x140166246: pop r15
0x140166248: pop rbp
0x140166249: ret 
0x14016624a: nop word ptr [rax + rax]
0x140166250: mov qword ptr [rsp + 0x10], rdx
0x140166255: push rbp
0x140166256: push r15
0x140166258: push r14
0x14016625a: push r13
0x14016625c: push r12
0x14016625e: push rsi
0x14016625f: push rdi
0x140166260: push rbx
0x140166261: sub rsp, 0x68
0x140166265: lea rbp, [rdx + 0x80]
0x14016626c: movaps xmmword ptr [rsp + 0x50], xmm6
0x140166271: cmp byte ptr [rbp + 0x1977], 0
0x140166278: je 0x1401662a3
0x14016627a: lea rcx, [rbp + 0x1440]
0x140166281: call 0x1405fcbe0
0x140166286: mov byte ptr [rbp + 0x1970], 0
0x14016628d: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140166292: add rsp, 0x68
0x140166296: pop rbx
0x140166297: pop rdi
0x140166298: pop rsi
0x140166299: pop r12
0x14016629b: pop r13
0x14016629d: pop r14
0x14016629f: pop r15
0x1401662a1: pop rbp
0x1401662a2: ret 
0x1401662a3: mov byte ptr [rbp + 0x1970], 0
0x1401662aa: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401662af: add rsp, 0x68
0x1401662b3: pop rbx
0x1401662b4: pop rdi
0x1401662b5: pop rsi
0x1401662b6: pop r12
0x1401662b8: pop r13
0x1401662ba: pop r14
0x1401662bc: pop r15
0x1401662be: pop rbp
0x1401662bf: ret 
0x1401662c0: mov qword ptr [rsp + 0x10], rdx
0x1401662c5: push rbp
0x1401662c6: push r15
0x1401662c8: push r14
0x1401662ca: push r13
0x1401662cc: push r12
0x1401662ce: push rsi
0x1401662cf: push rdi
0x1401662d0: push rbx
0x1401662d1: sub rsp, 0x68
0x1401662d5: lea rbp, [rdx + 0x80]
0x1401662dc: movaps xmmword ptr [rsp + 0x50], xmm6
0x1401662e1: movzx ebx, byte ptr [rbp + 0x1970]
0x1401662e8: test bl, 1
0x1401662eb: je 0x140166315
0x1401662ed: lea rcx, [rbp + 0x1350]
0x1401662f4: call 0x140161b50
0x1401662f9: mov byte ptr [rbp + 0x1976], bl
0x1401662ff: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140166304: add rsp, 0x68
0x140166308: pop rbx
0x140166309: pop rdi
0x14016630a: pop rsi
0x14016630b: pop r12
0x14016630d: pop r13
0x14016630f: pop r14
0x140166311: pop r15
0x140166313: pop rbp
0x140166314: ret 
0x140166315: mov byte ptr [rbp + 0x1976], bl
0x14016631b: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140166320: add rsp, 0x68
0x140166324: pop rbx
0x140166325: pop rdi
0x140166326: pop rsi
0x140166327: pop r12
0x140166329: pop r13
0x14016632b: pop r14
0x14016632d: pop r15
0x14016632f: pop rbp
0x140166330: ret 
0x140166331: nop word ptr cs:[rax + rax]
0x140166340: mov qword ptr [rsp + 0x10], rdx
0x140166345: push rbp
0x140166346: push r15
0x140166348: push r14
0x14016634a: push r13
0x14016634c: push r12
0x14016634e: push rsi
0x14016634f: push rdi
0x140166350: push rbx
0x140166351: sub rsp, 0x68
0x140166355: lea rbp, [rdx + 0x80]
0x14016635c: movaps xmmword ptr [rsp + 0x50], xmm6
0x140166361: movups xmm0, xmmword ptr [rbp + 0x1710]
0x140166368: movups xmm1, xmmword ptr [rbp + 0x1720]
0x14016636f: movaps xmmword ptr [rbp + 0x1450], xmm1
0x140166376: movaps xmmword ptr [rbp + 0x1440], xmm0
0x14016637d: lea rcx, [rbp + 0x1440]
0x140166384: call 0x140be24b0
0x140166389: movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016638e: add rsp, 0x68
0x140166392: pop rbx
0x140166393: pop rdi
0x140166394: pop rsi
0x140166395: pop r12
0x140166397: pop r13
0x140166399: pop r14
0x14016639b: pop r15
0x14016639d: pop rbp
0x14016639e: ret 
0x14016639f: nop 
0x1401663a0: mov qword ptr [rsp + 0x10], rdx
0x1401663a5: push rbp
0x1401663a6: push r15
0x1401663a8: push r14
0x1401663aa: push r13
0x1401663ac: push r12
0x1401663ae: push rsi
0x1401663af: push rdi
0x1401663b0: push rbx
0x1401663b1: sub rsp, 0x68
0x1401663b5: lea rbp, [rdx + 0x80]
0x1401663bc: movaps xmmword ptr [rsp + 0x50], xmm6
0x1401663c1: mov rcx, qword ptr [rbp + 0x1960]
0x1401663c8: call 0x140161c30
0x1401663cd: mov byte ptr [rbp + 0x1976], 1
0x1401663d4: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401663d9: add rsp, 0x68
0x1401663dd: pop rbx
0x1401663de: pop rdi
0x1401663df: pop rsi
0x1401663e0: pop r12
0x1401663e2: pop r13
0x1401663e4: pop r14
0x1401663e6: pop r15
0x1401663e8: pop rbp
0x1401663e9: ret 
0x1401663ea: nop word ptr [rax + rax]
0x1401663f0: mov qword ptr [rsp + 0x10], rdx
0x1401663f5: push rbp
0x1401663f6: push r15
0x1401663f8: push r14
0x1401663fa: push r13
0x1401663fc: push r12
0x1401663fe: push rsi
0x1401663ff: push rdi
0x140166400: push rbx
0x140166401: sub rsp, 0x68
0x140166405: lea rbp, [rdx + 0x80]
0x14016640c: movaps xmmword ptr [rsp + 0x50], xmm6
0x140166411: movzx ebx, byte ptr [rbp + 0x1976]
0x140166418: test bl, 1
0x14016641b: je 0x140166445
0x14016641d: lea rcx, [rbp + 0x1888]
0x140166424: call 0x140be24b0
0x140166429: mov byte ptr [rbp + 0x1975], bl
0x14016642f: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140166434: add rsp, 0x68
0x140166438: pop rbx
0x140166439: pop rdi
0x14016643a: pop rsi
0x14016643b: pop r12
0x14016643d: pop r13
0x14016643f: pop r14
0x140166441: pop r15
0x140166443: pop rbp
0x140166444: ret 
0x140166445: mov byte ptr [rbp + 0x1975], bl
0x14016644b: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140166450: add rsp, 0x68
0x140166454: pop rbx
0x140166455: pop rdi
0x140166456: pop rsi
0x140166457: pop r12
0x140166459: pop r13
0x14016645b: pop r14
0x14016645d: pop r15
0x14016645f: pop rbp
0x140166460: ret 
0x140166461: nop word ptr cs:[rax + rax]
0x140166470: mov qword ptr [rsp + 0x10], rdx
0x140166475: push rbp
0x140166476: push r15
0x140166478: push r14
0x14016647a: push r13
0x14016647c: push r12
0x14016647e: push rsi
0x14016647f: push rdi
0x140166480: push rbx
0x140166481: sub rsp, 0x68
0x140166485: lea rbp, [rdx + 0x80]
0x14016648c: movaps xmmword ptr [rsp + 0x50], xmm6
0x140166491: test byte ptr [rbp + 0x1975], 1
0x140166498: je 0x1401664a6
0x14016649a: lea rcx, [rbp + 0x1220]
0x1401664a1: call 0x1405dcdc0
0x1401664a6: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401664ab: add rsp, 0x68
0x1401664af: pop rbx
0x1401664b0: pop rdi
0x1401664b1: pop rsi
0x1401664b2: pop r12
0x1401664b4: pop r13
0x1401664b6: pop r14
0x1401664b8: pop r15
0x1401664ba: pop rbp
0x1401664bb: ret 
0x1401664bc: nop dword ptr [rax]
0x1401664c0: mov qword ptr [rsp + 0x10], rdx
0x1401664c5: push rbp
0x1401664c6: push r15
0x1401664c8: push r14
0x1401664ca: push r13
0x1401664cc: push r12
0x1401664ce: push rsi
0x1401664cf: push rdi
0x1401664d0: push rbx
0x1401664d1: sub rsp, 0x68
0x1401664d5: lea rbp, [rdx + 0x80]
0x1401664dc: movaps xmmword ptr [rsp + 0x50], xmm6
0x1401664e1: cmp qword ptr [rbp + 0x18e0], 0
0x1401664e9: je 0x140166501
0x1401664eb: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401664f0: add rsp, 0x68
0x1401664f4: pop rbx
0x1401664f5: pop rdi
0x1401664f6: pop rsi
0x1401664f7: pop r12
0x1401664f9: pop r13
0x1401664fb: pop r14
0x1401664fd: pop r15
0x1401664ff: pop rbp
0x140166500: ret 
0x140166501: lea rcx, [rbp + 0x1170]
0x140166508: call 0x1405ef210
0x14016650d: jmp 0x1401664eb
