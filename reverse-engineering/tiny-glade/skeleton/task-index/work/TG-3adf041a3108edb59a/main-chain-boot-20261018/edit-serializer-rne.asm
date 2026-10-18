140b39cd0: push r14
140b39cd2: push rsi
140b39cd3: push rdi
140b39cd4: push rbx
140b39cd5: sub rsp, 0x38
140b39cd9: mov rsi, rdx
140b39cdc: mov rax, qword ptr [rcx]
140b39cdf: lea rdx, [rip + 0x1fb46ee] ; 0x142aee3d4 b'\x1b\xb9\x04\xfe\xe7\xbc\x04\xfe\xf4\xbb\x04\xfe-\xbc\x04\xfe\xb9\xba\x04\xfe \xbd\x04\xfe\xd7\xbd\x04\xfef\xbc\x04\xfe\xfb\xbe\x04\xfes\xbb\x04\xfez\xbe\x04\xfe\x1d\xba\x04\xfe\xf2\xba\x04\xfeY\xbd\x04\xfe\x9c\xb9\x04\xfe'
140b39ce6: movsxd rax, dword ptr [rdx + rax*4]
140b39cea: add rax, rdx
140b39ced: jmp rax
140b39cef: mov rdi, qword ptr [rcx + 8]
140b39cf3: lea rdx, [rsi + 8]
140b39cf7: mov rbx, qword ptr [rsi]
140b39cfa: mov rax, qword ptr [rbx + 0x10]
140b39cfe: cmp qword ptr [rbx], rax
140b39d01: je 0x140b3aa72
140b39d07: mov rcx, qword ptr [rbx + 8]
140b39d0b: mov byte ptr [rcx + rax], 0x7b
140b39d0f: inc rax
140b39d12: mov qword ptr [rbx + 0x10], rax
140b39d16: lea r8, [rip + 0x1fb43d0] ; 0x142aee0ed b'TerrainPathCobblestone1Cobblestone2Cobblestone3WaterGardenWa'
140b39d1d: mov r9d, 7
140b39d23: mov rcx, rsi
140b39d26: call 0x141580d20
140b39d2b: test rax, rax
140b39d2e: jne 0x140b3a36e
140b39d34: mov rbx, qword ptr [rsi]
140b39d37: mov rdx, qword ptr [rbx + 0x10]
140b39d3b: cmp qword ptr [rbx], rdx
140b39d3e: je 0x140b3acbc
140b39d44: mov rax, qword ptr [rbx + 8]
140b39d48: mov byte ptr [rax + rdx], 0x3a
140b39d4c: inc rdx
140b39d4f: mov qword ptr [rbx + 0x10], rdx
140b39d53: add rdi, 0x10
140b39d57: mov rcx, rdi
140b39d5a: mov rdx, rsi
140b39d5d: call 0x1417c1590
140b39d62: test rax, rax
140b39d65: jne 0x140b3a244
140b39d6b: jmp 0x140b3a223
140b39d70: mov rdi, qword ptr [rcx + 8]
140b39d74: lea rdx, [rsi + 8]
140b39d78: mov rbx, qword ptr [rsi]
140b39d7b: mov rax, qword ptr [rbx + 0x10]
140b39d7f: cmp qword ptr [rbx], rax
140b39d82: je 0x140b3a8f7
140b39d88: mov rcx, qword ptr [rbx + 8]
140b39d8c: mov byte ptr [rcx + rax], 0x7b
140b39d90: inc rax
140b39d93: mov qword ptr [rbx + 0x10], rax
140b39d97: lea r8, [rip + 0x1fb43bc] ; 0x142aee15a b'Stairs\xed\xe0\xaeB\x01\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00\xf4\xe0\xaeB\x01\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\xf8\xe0\xaeB\x01\x00\x00\x00\x0c\x00\x00\x00\x00\x00\x00\x00\x04\xe1\xaeB\x01\x00'
140b39d9e: mov r9d, 6
140b39da4: mov rcx, rsi
140b39da7: call 0x141580d20
140b39dac: test rax, rax
140b39daf: jne 0x140b3a36e
140b39db5: mov rbx, qword ptr [rsi]
140b39db8: mov rdx, qword ptr [rbx + 0x10]
140b39dbc: cmp qword ptr [rbx], rdx
140b39dbf: je 0x140b3abb2
140b39dc5: mov rax, qword ptr [rbx + 8]
140b39dc9: mov byte ptr [rax + rdx], 0x3a
140b39dcd: inc rdx
140b39dd0: mov qword ptr [rbx + 0x10], rdx
140b39dd4: add rdi, 0x10
140b39dd8: mov rcx, rdi
140b39ddb: mov rdx, rsi
140b39dde: call 0x140a18ba0
140b39de3: test rax, rax
140b39de6: jne 0x140b3a244
140b39dec: jmp 0x140b3a223
140b39df1: lea rdx, [rsi + 8]
140b39df5: mov rdi, qword ptr [rsi]
140b39df8: mov rax, qword ptr [rdi + 0x10]
140b39dfc: cmp qword ptr [rdi], rax
140b39dff: je 0x140b3a926
140b39e05: mov rcx, qword ptr [rdi + 8]
140b39e09: mov byte ptr [rcx + rax], 0x7b
140b39e0d: inc rax
140b39e10: mov qword ptr [rdi + 0x10], rax
140b39e14: lea r8, [rip + 0x1fb4324] ; 0x142aee13f b'DecoratorOldTreeAutoClutterStairs\xed\xe0\xaeB\x01\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00\xf4\xe0\xaeB\x01\x00\x00\x00\x04\x00\x00'
140b39e1b: mov r9d, 0xc
140b39e21: mov rcx, rsi
140b39e24: call 0x141580d20
140b39e29: test rax, rax
140b39e2c: jne 0x140b3a36e
140b39e32: mov rdi, qword ptr [rsi]
140b39e35: mov rdx, qword ptr [rdi + 0x10]
140b39e39: cmp qword ptr [rdi], rdx
140b39e3c: je 0x140b3abd8
140b39e42: mov rax, qword ptr [rdi + 8]
140b39e46: mov byte ptr [rax + rdx], 0x3a
140b39e4a: inc rdx
140b39e4d: mov qword ptr [rdi + 0x10], rdx
140b39e51: mov rdi, qword ptr [rsi]
140b39e54: mov rdx, qword ptr [rdi + 0x10]
140b39e58: cmp qword ptr [rdi], rdx
140b39e5b: je 0x140b3abfe
140b39e61: mov rax, qword ptr [rdi + 8]
140b39e65: mov byte ptr [rax + rdx], 0x7b
140b39e69: inc rdx
140b39e6c: mov qword ptr [rdi + 0x10], rdx
140b39e70: cmp qword ptr [rdi], rdx
140b39e73: je 0x140b3ac24
140b39e79: mov rax, qword ptr [rdi + 8]
140b39e7d: mov byte ptr [rax + rdx], 0x7d
140b39e81: inc rdx
140b39e84: mov qword ptr [rdi + 0x10], rdx
140b39e88: jmp 0x140b3a223
140b39e8d: mov rdi, qword ptr [rcx + 8]
140b39e91: lea rdx, [rsi + 8]
140b39e95: mov rbx, qword ptr [rsi]
140b39e98: mov rax, qword ptr [rbx + 0x10]
140b39e9c: cmp qword ptr [rbx], rax
140b39e9f: je 0x140b3a955
140b39ea5: mov rcx, qword ptr [rbx + 8]
140b39ea9: mov byte ptr [rcx + rax], 0x7b
140b39ead: inc rax
140b39eb0: mov qword ptr [rbx + 0x10], rax
140b39eb4: lea r8, [rip + 0x1fb4255] ; 0x142aee110 b'Cobblestone3WaterGardenWallRoofColorDecoratorV5DecoratorOldT'
140b39ebb: mov r9d, 0xc
140b39ec1: jmp 0x140b3a1df
140b39ec6: mov rdi, qword ptr [rcx + 8]
140b39eca: lea rdx, [rsi + 8]
140b39ece: mov rbx, qword ptr [rsi]
140b39ed1: mov rax, qword ptr [rbx + 0x10]
140b39ed5: cmp qword ptr [rbx], rax
140b39ed8: je 0x140b3a984
140b39ede: mov rcx, qword ptr [rbx + 8]
140b39ee2: mov byte ptr [rcx + rax], 0x7b
140b39ee6: inc rax
140b39ee9: mov qword ptr [rbx + 0x10], rax
140b39eed: lea r8, [rip + 0x1fb4257] ; 0x142aee14b b'TreeAutoClutterStairs\xed\xe0\xaeB\x01\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00\xf4\xe0\xaeB\x01\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\xf8\xe0\xaeB\x01\x00\x00'
140b39ef4: mov r9d, 4
140b39efa: mov rcx, rsi
140b39efd: call 0x141580d20
140b39f02: test rax, rax
140b39f05: jne 0x140b3a36e
140b39f0b: mov rbx, qword ptr [rsi]
140b39f0e: mov rdx, qword ptr [rbx + 0x10]
140b39f12: cmp qword ptr [rbx], rdx
140b39f15: je 0x140b3ac4a
140b39f1b: mov rax, qword ptr [rbx + 8]
140b39f1f: mov byte ptr [rax + rdx], 0x3a
140b39f23: inc rdx
140b39f26: mov qword ptr [rbx + 0x10], rdx
140b39f2a: add rdi, 0x10
140b39f2e: mov rcx, rdi
140b39f31: mov rdx, rsi
140b39f34: call 0x1417c2a50
140b39f39: test rax, rax
140b39f3c: jne 0x140b3a244
140b39f42: jmp 0x140b3a223
140b39f47: mov rdi, qword ptr [rcx + 8]
140b39f4b: lea rdx, [rsi + 8]
140b39f4f: mov rbx, qword ptr [rsi]
140b39f52: mov rax, qword ptr [rbx + 0x10]
140b39f56: cmp qword ptr [rbx], rax
140b39f59: je 0x140b3a9b3
140b39f5f: mov rcx, qword ptr [rbx + 8]
140b39f63: mov byte ptr [rcx + rax], 0x7b
140b39f67: inc rax
140b39f6a: mov qword ptr [rbx + 0x10], rax
140b39f6e: lea r8, [rip + 0x1fb41ba] ; 0x142aee12f b'ColorDecoratorV5DecoratorOldTreeAutoClutterStairs\xed\xe0\xaeB\x01\x00\x00\x00\x07\x00\x00'
140b39f75: mov r9d, 5
140b39f7b: mov rcx, rsi
140b39f7e: call 0x141580d20
140b39f83: test rax, rax
140b39f86: jne 0x140b3a36e
140b39f8c: mov rbx, qword ptr [rsi]
140b39f8f: mov rdx, qword ptr [rbx + 0x10]
140b39f93: cmp qword ptr [rbx], rdx
140b39f96: je 0x140b3ac70
140b39f9c: mov rax, qword ptr [rbx + 8]
140b39fa0: mov byte ptr [rax + rdx], 0x3a
140b39fa4: inc rdx
140b39fa7: mov qword ptr [rbx + 0x10], rdx
140b39fab: add rdi, 0x10
140b39faf: mov rcx, rdi
140b39fb2: mov rdx, rsi
140b39fb5: call 0x1417be2b0
140b39fba: test rax, rax
140b39fbd: jne 0x140b3a244
140b39fc3: jmp 0x140b3a223
140b39fc8: mov rdi, qword ptr [rcx + 8]
140b39fcc: lea rdx, [rsi + 8]
140b39fd0: mov rbx, qword ptr [rsi]
140b39fd3: mov rax, qword ptr [rbx + 0x10]
140b39fd7: cmp qword ptr [rbx], rax
140b39fda: je 0x140b3a9e2
140b39fe0: mov rcx, qword ptr [rbx + 8]
140b39fe4: mov byte ptr [rcx + rax], 0x7b
140b39fe8: inc rax
140b39feb: mov qword ptr [rbx + 0x10], rax
140b39fef: lea r8, [rip + 0x1fb4102] ; 0x142aee0f8 b'Cobblestone1Cobblestone2Cobblestone3WaterGardenWallRoofColor'
140b39ff6: mov r9d, 0xc
140b39ffc: jmp 0x140b3a1df
140b3a001: mov rdi, qword ptr [rcx + 8]
140b3a005: lea rdx, [rsi + 8]
140b3a009: mov rbx, qword ptr [rsi]
140b3a00c: mov rax, qword ptr [rbx + 0x10]
140b3a010: cmp qword ptr [rbx], rax
140b3a013: je 0x140b3aa11
140b3a019: mov rcx, qword ptr [rbx + 8]
140b3a01d: mov byte ptr [rcx + rax], 0x7b
140b3a021: inc rax
140b3a024: mov qword ptr [rbx + 0x10], rax
140b3a028: lea r8, [rip + 0x1fb40d5] ; 0x142aee104 b'Cobblestone2Cobblestone3WaterGardenWallRoofColorDecoratorV5D'
140b3a02f: mov r9d, 0xc
140b3a035: jmp 0x140b3a1df
140b3a03a: lea rdx, [rsi + 8]
140b3a03e: mov rdi, qword ptr [rsi]
140b3a041: mov rax, qword ptr [rdi + 0x10]
140b3a045: cmp qword ptr [rdi], rax
140b3a048: je 0x140b3aa40
140b3a04e: mov rbx, rcx
140b3a051: mov rcx, qword ptr [rdi + 8]
140b3a055: mov byte ptr [rcx + rax], 0x7b
140b3a059: inc rax
140b3a05c: mov qword ptr [rdi + 0x10], rax
140b3a060: lea r8, [rip + 0x1fb40c0] ; 0x142aee127 b'WallRoofColorDecoratorV5DecoratorOldTreeAutoClutterStairs\xed\xe0\xae'
140b3a067: mov r9d, 4
140b3a06d: mov rcx, rsi
140b3a070: call 0x141580d20
140b3a075: test rax, rax
140b3a078: jne 0x140b3a36e
140b3a07e: mov rdi, qword ptr [rsi]
140b3a081: mov rdx, qword ptr [rdi + 0x10]
140b3a085: cmp qword ptr [rdi], rdx
140b3a088: je 0x140b3ac96
140b3a08e: mov rax, qword ptr [rdi + 8]
140b3a092: mov byte ptr [rax + rdx], 0x3a
140b3a096: inc rdx
140b3a099: mov qword ptr [rdi + 0x10], rdx
140b3a09d: mov rdx, qword ptr [rbx + 8]
140b3a0a1: mov r8, qword ptr [rbx + 0x10]
140b3a0a5: mov rcx, rsi
140b3a0a8: call 0x141582520
140b3a0ad: test rax, rax
140b3a0b0: jne 0x140b3a244
140b3a0b6: jmp 0x140b3a223
140b3a0bb: mov rdi, qword ptr [rcx + 8]
140b3a0bf: lea rdx, [rsi + 8]
140b3a0c3: mov rbx, qword ptr [rsi]
140b3a0c6: mov rax, qword ptr [rbx + 0x10]
140b3a0ca: cmp qword ptr [rbx], rax
140b3a0cd: je 0x140b3aaa1
140b3a0d3: mov rcx, qword ptr [rbx + 8]
140b3a0d7: mov byte ptr [rcx + rax], 0x7b
140b3a0db: inc rax
140b3a0de: mov qword ptr [rbx + 0x10], rax
140b3a0e2: lea r8, [rip + 0x1fb400b] ; 0x142aee0f4 b'PathCobblestone1Cobblestone2Cobblestone3WaterGardenWallRoofC'
140b3a0e9: mov r9d, 4
140b3a0ef: jmp 0x140b3a1df
140b3a0f4: mov rdi, qword ptr [rcx + 8]
140b3a0f8: lea rdx, [rsi + 8]
140b3a0fc: mov rbx, qword ptr [rsi]
140b3a0ff: mov rax, qword ptr [rbx + 0x10]
140b3a103: cmp qword ptr [rbx], rax
140b3a106: je 0x140b3aad0
140b3a10c: mov rcx, qword ptr [rbx + 8]
140b3a110: mov byte ptr [rcx + rax], 0x7b
140b3a114: inc rax
140b3a117: mov qword ptr [rbx + 0x10], rax
140b3a11b: lea r8, [rip + 0x1fb3ffa] ; 0x142aee11c b'WaterGardenWallRoofColorDecoratorV5DecoratorOldTreeAutoClutt'
140b3a122: mov r9d, 5
140b3a128: jmp 0x140b3a1df
140b3a12d: mov rdi, qword ptr [rcx + 8]
140b3a131: lea rdx, [rsi + 8]
140b3a135: mov rbx, qword ptr [rsi]
140b3a138: mov rax, qword ptr [rbx + 0x10]
140b3a13c: cmp qword ptr [rbx], rax
140b3a13f: je 0x140b3aaff
140b3a145: mov rcx, qword ptr [rbx + 8]
140b3a149: mov byte ptr [rcx + rax], 0x7b
140b3a14d: inc rax
140b3a150: mov qword ptr [rbx + 0x10], rax
140b3a154: lea r8, [rip + 0x1fb3ff4] ; 0x142aee14f b'AutoClutterStairs\xed\xe0\xaeB\x01\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00\xf4\xe0\xaeB\x01\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\xf8\xe0\xaeB\x01\x00\x00\x00\x0c\x00\x00'
140b3a15b: mov r9d, 0xb
140b3a161: mov rcx, rsi
140b3a164: call 0x141580d20
140b3a169: test rax, rax
140b3a16c: jne 0x140b3a36e
140b3a172: mov rbx, qword ptr [rsi]
140b3a175: mov rdx, qword ptr [rbx + 0x10]
140b3a179: cmp qword ptr [rbx], rdx
140b3a17c: je 0x140b3ace2
140b3a182: mov rax, qword ptr [rbx + 8]
140b3a186: mov byte ptr [rax + rdx], 0x3a
140b3a18a: inc rdx
140b3a18d: mov qword ptr [rbx + 0x10], rdx
140b3a191: add rdi, 0x10
140b3a195: mov rcx, rdi
140b3a198: mov rdx, rsi
140b3a19b: call 0x140a18320
140b3a1a0: test rax, rax
140b3a1a3: jne 0x140b3a244
140b3a1a9: jmp 0x140b3a223
140b3a1ab: mov rdi, qword ptr [rcx + 8]
140b3a1af: lea rdx, [rsi + 8]
140b3a1b3: mov rbx, qword ptr [rsi]
140b3a1b6: mov rax, qword ptr [rbx + 0x10]
140b3a1ba: cmp qword ptr [rbx], rax
140b3a1bd: je 0x140b3ab2e
140b3a1c3: mov rcx, qword ptr [rbx + 8]
140b3a1c7: mov byte ptr [rcx + rax], 0x7b
140b3a1cb: inc rax
140b3a1ce: mov qword ptr [rbx + 0x10], rax
140b3a1d2: lea r8, [rip + 0x1fb3f48] ; 0x142aee121 b'GardenWallRoofColorDecoratorV5DecoratorOldTreeAutoClutterSta'
140b3a1d9: mov r9d, 6
140b3a1df: mov rcx, rsi
140b3a1e2: call 0x141580d20
140b3a1e7: test rax, rax
140b3a1ea: jne 0x140b3a36e
140b3a1f0: mov rbx, qword ptr [rsi]
140b3a1f3: mov rdx, qword ptr [rbx + 0x10]
140b3a1f7: cmp qword ptr [rbx], rdx
140b3a1fa: je 0x140b3a8ab
140b3a200: mov rax, qword ptr [rbx + 8]
140b3a204: mov byte ptr [rax + rdx], 0x3a
140b3a208: inc rdx
140b3a20b: mov qword ptr [rbx + 0x10], rdx
140b3a20f: add rdi, 0x10
140b3a213: mov rcx, rdi
140b3a216: mov rdx, rsi
140b3a219: call 0x1416e8340
140b3a21e: test rax, rax
140b3a221: jne 0x140b3a244
140b3a223: mov rsi, qword ptr [rsi]
140b3a226: mov rdx, qword ptr [rsi + 0x10]
140b3a22a: cmp qword ptr [rsi], rdx
140b3a22d: je 0x140b3a8d1
140b3a233: mov rax, qword ptr [rsi + 8]
140b3a237: mov byte ptr [rax + rdx], 0x7d
140b3a23b: inc rdx
140b3a23e: mov qword ptr [rsi + 0x10], rdx
140b3a242: xor eax, eax
140b3a244: add rsp, 0x38
140b3a248: pop rbx
140b3a249: pop rdi
140b3a24a: pop rsi
140b3a24b: pop r14
140b3a24d: ret 
140b3a24e: mov rdi, qword ptr [rcx + 8]
140b3a252: lea rdx, [rsi + 8]
140b3a256: mov rbx, qword ptr [rsi]
140b3a259: mov rax, qword ptr [rbx + 0x10]
140b3a25d: cmp qword ptr [rbx], rax
140b3a260: je 0x140b3ab5d
140b3a266: mov rcx, qword ptr [rbx + 8]
140b3a26a: mov byte ptr [rcx + rax], 0x7b
140b3a26e: inc rax
140b3a271: mov qword ptr [rbx + 0x10], rax
140b3a275: lea r8, [rip + 0x1fb3eb8] ; 0x142aee134 b'DecoratorV5DecoratorOldTreeAutoClutterStairs\xed\xe0\xaeB\x01\x00\x00\x00\x07\x00\x00\x00\x00\x00\x00\x00'
140b3a27c: mov r9d, 0xb
140b3a282: mov rcx, rsi
140b3a285: call 0x141580d20
140b3a28a: test rax, rax
140b3a28d: jne 0x140b3a36e
140b3a293: mov rbx, qword ptr [rsi]
140b3a296: mov rdx, qword ptr [rbx + 0x10]
140b3a29a: cmp qword ptr [rbx], rdx
140b3a29d: je 0x140b3ad08
140b3a2a3: mov rax, qword ptr [rbx + 8]
140b3a2a7: mov byte ptr [rax + rdx], 0x3a
140b3a2ab: inc rdx
140b3a2ae: mov qword ptr [rbx + 0x10], rdx
140b3a2b2: add rdi, 0x10
140b3a2b6: mov rcx, rdi
140b3a2b9: mov rdx, rsi
140b3a2bc: call 0x1417bfb10
140b3a2c1: test rax, rax
140b3a2c4: jne 0x140b3a244
140b3a2ca: jmp 0x140b3a223
140b3a2cf: mov rdi, qword ptr [rcx + 8]
140b3a2d3: lea rbx, [rsi + 8]
140b3a2d7: mov r14, qword ptr [rsi]
140b3a2da: mov rdx, qword ptr [r14 + 0x10]
140b3a2de: cmp qword ptr [r14], rdx
140b3a2e1: je 0x140b3ab8c
140b3a2e7: mov rax, qword ptr [r14 + 8]
140b3a2eb: mov byte ptr [rax + rdx], 0x7b
140b3a2ef: inc rdx
140b3a2f2: mov qword ptr [r14 + 0x10], rdx
140b3a2f6: lea r8, [rip + 0x1fb3e2e] ; 0x142aee12b b'RoofColorDecoratorV5DecoratorOldTreeAutoClutterStairs\xed\xe0\xaeB\x01\x00\x00'
140b3a2fd: mov r9d, 4
140b3a303: mov rcx, rsi
140b3a306: mov rdx, rbx
140b3a309: call 0x141580d20
140b3a30e: test rax, rax
140b3a311: jne 0x140b3a36e
140b3a313: mov r14, qword ptr [rsi]
140b3a316: mov rdx, qword ptr [r14 + 0x10]
140b3a31a: cmp qword ptr [r14], rdx
140b3a31d: je 0x140b3ad2e
140b3a323: mov rax, qword ptr [r14 + 8]
140b3a327: mov byte ptr [rax + rdx], 0x3a
140b3a32b: inc rdx
140b3a32e: mov qword ptr [r14 + 0x10], rdx
140b3a332: movzx eax, byte ptr [rdi + 0x10]
140b3a336: lea rcx, [rip + 0x1fb40d3] ; 0x142aee410 b'6\xbf\x04\xfeJ\xc3\x04\xfeX\xc0\x04\xfea\xc2\x04\xfeo\xbf\x04\xfe\x9a\xcb\x04\xfe\x06\xce\x04\xfe\x88\xcd\x04\xfe\xac\xcd\x04\xfe\x1d\xcd\x04\xfe*\xce\x04\xfen\xce\x04\xfe\xd0\xcd\x04\xfe\xbd\xce\x04\xfed\xcd\x04\xfe'
140b3a33d: movsxd rax, dword ptr [rcx + rax*4]
140b3a341: add rax, rcx
140b3a344: jmp rax
140b3a346: mov rcx, qword ptr [rsi]
140b3a349: lea rdx, [rip + 0x1fb3759] ; 0x142aedaa9 b'{:map with a single key\xab\xda\xaeB\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00'
140b3a350: lea r9, [rip + 0x1fb3839] ; 0x142aedb90 b'@\xdb\xaeB\x01\x00\x00\x00K\x00\x00\x00\x00\x00\x00\x00\xee\x01\x00\x00\x0e\x00\x00\x00/rustc/05f9846f893b09a1be1fc8560e33f'
140b3a357: mov r8d, 1
140b3a35d: call 0x1412f4520
140b3a362: lea r8, [rip + 0x1fb3997] ; 0x142aedd00 b'CreateRoofwall_indexparamscolor_idflat_roof_styleDeleteRoofV'
140b3a369: jmp 0x140b3a77d
140b3a36e: mov rcx, rax
140b3a371: add rsp, 0x38
140b3a375: pop rbx
140b3a376: pop rdi
140b3a377: pop rsi
140b3a378: pop r14
140b3a37a: jmp 0x1428c98a0
140b3a37f: mov rcx, qword ptr [rsi]
140b3a382: lea rdx, [rip + 0x1fb3720] ; 0x142aedaa9 b'{:map with a single key\xab\xda\xaeB\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00'
140b3a389: lea r9, [rip + 0x1fb3800] ; 0x142aedb90 b'@\xdb\xaeB\x01\x00\x00\x00K\x00\x00\x00\x00\x00\x00\x00\xee\x01\x00\x00\x0e\x00\x00\x00/rustc/05f9846f893b09a1be1fc8560e33f'
140b3a390: mov r8d, 1
140b3a396: call 0x1412f4520
140b3a39b: lea r8, [rip + 0x1fb3a28] ; 0x142aeddca b'ChangeFlatRoofRimStylebeforeafter\x00\x00\x00\x00\x00\x00\xdd\xaeB\x01\x00\x00\x00\n\x00\x00\x00\x00\x00\x00\x001\xdd\xaeB\x01\x00'
140b3a3a2: mov r9d, 0x16
140b3a3a8: mov rcx, rsi
140b3a3ab: mov rdx, rbx
140b3a3ae: call 0x141580d20
140b3a3b3: test rax, rax
140b3a3b6: jne 0x140b3a89e
140b3a3bc: mov rcx, qword ptr [rsi]
140b3a3bf: lea rdx, [rip + 0x1fb36e4] ; 0x142aedaaa b':map with a single key\xab\xda\xaeB\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00'
140b3a3c6: lea rbx, [rip + 0x1fb37c3] ; 0x142aedb90 b'@\xdb\xaeB\x01\x00\x00\x00K\x00\x00\x00\x00\x00\x00\x00\xee\x01\x00\x00\x0e\x00\x00\x00/rustc/05f9846f893b09a1be1fc8560e33f'
140b3a3cd: mov r8d, 1
140b3a3d3: mov r9, rbx
140b3a3d6: call 0x1412f4520
140b3a3db: mov rcx, qword ptr [rsi]
140b3a3de: lea rdx, [rip + 0x1fb36c4] ; 0x142aedaa9 b'{:map with a single key\xab\xda\xaeB\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00'
140b3a3e5: mov r8d, 1
140b3a3eb: mov r9, rbx
140b3a3ee: call 0x1412f4520
140b3a3f3: lea r9, [rdi + 0x18]
140b3a3f7: mov qword ptr [rsp + 0x28], rsi
140b3a3fc: mov byte ptr [rsp + 0x30], 1
140b3a401: lea rdx, [rip + 0x1fb3902] ; 0x142aedd0a b'wall_indexparamscolor_idflat_roof_styleDeleteRoofVisualEditp'
140b3a408: lea rcx, [rsp + 0x28]
140b3a40d: mov r8d, 0xa
140b3a413: call 0x141584ab0
140b3a418: test rax, rax
140b3a41b: jne 0x140b3a244
140b3a421: lea r9, [rdi + 0x11]
140b3a425: lea rdx, [rip + 0x1fb39b4] ; 0x142aedde0 b'beforeafter\x00\x00\x00\x00\x00\x00\xdd\xaeB\x01\x00\x00\x00\n\x00\x00\x00\x00\x00\x00\x001\xdd\xaeB\x01\x00\x00\x00\n\x00\x00\x00\x00\x00\x00\x00;\xdd\xaeB\x01\x00\x00\x00\n\x00\x00\x00'
140b3a42c: lea rcx, [rsp + 0x28]
140b3a431: mov r8d, 6
140b3a437: call 0x141584610
140b3a43c: test rax, rax
140b3a43f: jne 0x140b3a244
140b3a445: add rdi, 0x12
140b3a449: lea rdx, [rip + 0x1fb3996] ; 0x142aedde6 b'after\x00\x00\x00\x00\x00\x00\xdd\xaeB\x01\x00\x00\x00\n\x00\x00\x00\x00\x00\x00\x001\xdd\xaeB\x01\x00\x00\x00\n\x00\x00\x00\x00\x00\x00\x00;\xdd\xaeB\x01\x00\x00\x00\n\x00\x00\x00\x00\x00\x00\x00\xb2\xdd'
140b3a450: lea rcx, [rsp + 0x28]
140b3a455: mov r8d, 5
140b3a45b: mov r9, rdi
140b3a45e: call 0x141584610
140b3a463: jmp 0x140b3a862
140b3a468: mov rcx, qword ptr [rsi]
140b3a46b: lea rdx, [rip + 0x1fb3637] ; 0x142aedaa9 b'{:map with a single key\xab\xda\xaeB\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00'
140b3a472: lea r9, [rip + 0x1fb3717] ; 0x142aedb90 b'@\xdb\xaeB\x01\x00\x00\x00K\x00\x00\x00\x00\x00\x00\x00\xee\x01\x00\x00\x0e\x00\x00\x00/rustc/05f9846f893b09a1be1fc8560e33f'
140b3a479: mov r8d, 1
140b3a47f: call 0x1412f4520
140b3a484: lea r8, [rip + 0x1fb38b0] ; 0x142aedd3b b'VisualEditprev_heightprev_profileprev_tip_offsetprev_ridgepr'
140b3a48b: mov r9d, 0xa
140b3a491: mov rcx, rsi
140b3a494: mov rdx, rbx
140b3a497: call 0x141580d20
140b3a49c: test rax, rax
140b3a49f: jne 0x140b3a89e
140b3a4a5: mov rcx, qword ptr [rsi]
140b3a4a8: lea rdx, [rip + 0x1fb35fb] ; 0x142aedaaa b':map with a single key\xab\xda\xaeB\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00'
140b3a4af: lea rbx, [rip + 0x1fb36da] ; 0x142aedb90 b'@\xdb\xaeB\x01\x00\x00\x00K\x00\x00\x00\x00\x00\x00\x00\xee\x01\x00\x00\x0e\x00\x00\x00/rustc/05f9846f893b09a1be1fc8560e33f'
140b3a4b6: mov r8d, 1
140b3a4bc: mov r9, rbx
140b3a4bf: call 0x1412f4520
140b3a4c4: mov rcx, qword ptr [rsi]
140b3a4c7: lea rdx, [rip + 0x1fb35db] ; 0x142aedaa9 b'{:map with a single key\xab\xda\xaeB\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00'
140b3a4ce: mov r8d, 1
140b3a4d4: mov r9, rbx
140b3a4d7: call 0x1412f4520
140b3a4dc: lea r9, [rdi + 0x38]
140b3a4e0: mov qword ptr [rsp + 0x28], rsi
140b3a4e5: mov byte ptr [rsp + 0x30], 1
140b3a4ea: lea rdx, [rip + 0x1fb3819] ; 0x142aedd0a b'wall_indexparamscolor_idflat_roof_styleDeleteRoofVisualEditp'
140b3a4f1: lea rcx, [rsp + 0x28]
140b3a4f6: mov r8d, 0xa
140b3a4fc: call 0x141584ab0
140b3a501: test rax, rax
140b3a504: jne 0x140b3a244
140b3a50a: lea r9, [rdi + 0x14]
140b3a50e: lea rdx, [rip + 0x1fb3830] ; 0x142aedd45 b'prev_heightprev_profileprev_tip_offsetprev_ridgeprev_eavenew'
140b3a515: lea rcx, [rsp + 0x28]
140b3a51a: mov r8d, 0xb
140b3a520: call 0x141586090
140b3a525: test rax, rax
140b3a528: jne 0x140b3a244
140b3a52e: lea r9, [rdi + 0x18]
140b3a532: lea rdx, [rip + 0x1fb3817] ; 0x142aedd50 b'prev_profileprev_tip_offsetprev_ridgeprev_eavenew_heightnew_'
140b3a539: lea rcx, [rsp + 0x28]
140b3a53e: mov r8d, 0xc
140b3a544: call 0x141586090
140b3a549: test rax, rax
140b3a54c: jne 0x140b3a244
140b3a552: lea r9, [rdi + 0x40]
140b3a556: lea rdx, [rip + 0x1fb37ff] ; 0x142aedd5c b'prev_tip_offsetprev_ridgeprev_eavenew_heightnew_profilenew_t'
140b3a55d: lea rcx, [rsp + 0x28]
140b3a562: mov r8d, 0xf
140b3a568: call 0x141585d40
140b3a56d: test rax, rax
140b3a570: jne 0x140b3a244
140b3a576: lea r9, [rdi + 0x1c]
140b3a57a: lea rdx, [rip + 0x1fb37ea] ; 0x142aedd6b b'prev_ridgeprev_eavenew_heightnew_profilenew_tip_offsetnew_ri'
140b3a581: lea rcx, [rsp + 0x28]
140b3a586: mov r8d, 0xa
140b3a58c: call 0x141586090
140b3a591: test rax, rax
140b3a594: jne 0x140b3a244
140b3a59a: lea r9, [rdi + 0x20]
140b3a59e: lea rdx, [rip + 0x1fb37d0] ; 0x142aedd75 b'prev_eavenew_heightnew_profilenew_tip_offsetnew_ridgenew_eav'
140b3a5a5: lea rcx, [rsp + 0x28]
140b3a5aa: mov r8d, 9
140b3a5b0: call 0x141586090
140b3a5b5: test rax, rax
140b3a5b8: jne 0x140b3a244
140b3a5be: lea r9, [rdi + 0x24]
140b3a5c2: lea rdx, [rip + 0x1fb37b5] ; 0x142aedd7e b'new_heightnew_profilenew_tip_offsetnew_ridgenew_eaveSwitchRo'
140b3a5c9: lea rcx, [rsp + 0x28]
140b3a5ce: mov r8d, 0xa
140b3a5d4: call 0x141586090
140b3a5d9: test rax, rax
140b3a5dc: jne 0x140b3a244
140b3a5e2: lea r9, [rdi + 0x28]
140b3a5e6: lea rdx, [rip + 0x1fb379b] ; 0x142aedd88 b'new_profilenew_tip_offsetnew_ridgenew_eaveSwitchRoofRidgeDir'
140b3a5ed: lea rcx, [rsp + 0x28]
140b3a5f2: mov r8d, 0xb
140b3a5f8: call 0x141586090
140b3a5fd: test rax, rax
140b3a600: jne 0x140b3a244
140b3a606: lea r9, [rdi + 0x48]
140b3a60a: lea rdx, [rip + 0x1fb3782] ; 0x142aedd93 b'new_tip_offsetnew_ridgenew_eaveSwitchRoofRidgeDirfromtoChang'
140b3a611: lea rcx, [rsp + 0x28]
140b3a616: mov r8d, 0xe
140b3a61c: call 0x141585d40
140b3a621: test rax, rax
140b3a624: jne 0x140b3a244
140b3a62a: lea r9, [rdi + 0x2c]
140b3a62e: lea rdx, [rip + 0x1fb376c] ; 0x142aedda1 b'new_ridgenew_eaveSwitchRoofRidgeDirfromtoChangeFlatRoofRimSt'
140b3a635: lea rcx, [rsp + 0x28]
140b3a63a: mov r8d, 9
140b3a640: call 0x141586090
140b3a645: test rax, rax
140b3a648: jne 0x140b3a244
140b3a64e: add rdi, 0x30
140b3a652: lea rdx, [rip + 0x1fb3751] ; 0x142aeddaa b'new_eaveSwitchRoofRidgeDirfromtoChangeFlatRoofRimStylebefore'
140b3a659: lea rcx, [rsp + 0x28]
140b3a65e: mov r8d, 8
140b3a664: mov r9, rdi
140b3a667: call 0x141586090
140b3a66c: jmp 0x140b3a862
140b3a671: mov rcx, qword ptr [rsi]
140b3a674: lea rdx, [rip + 0x1fb342e] ; 0x142aedaa9 b'{:map with a single key\xab\xda\xaeB\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00'
140b3a67b: lea r9, [rip + 0x1fb350e] ; 0x142aedb90 b'@\xdb\xaeB\x01\x00\x00\x00K\x00\x00\x00\x00\x00\x00\x00\xee\x01\x00\x00\x0e\x00\x00\x00/rustc/05f9846f893b09a1be1fc8560e33f'
140b3a682: mov r8d, 1
140b3a688: call 0x1412f4520
140b3a68d: lea r8, [rip + 0x1fb371e] ; 0x142aeddb2 b'SwitchRoofRidgeDirfromtoChangeFlatRoofRimStylebeforeafter\x00\x00\x00'
140b3a694: mov r9d, 0x12
140b3a69a: mov rcx, rsi
140b3a69d: mov rdx, rbx
140b3a6a0: call 0x141580d20
140b3a6a5: test rax, rax
140b3a6a8: jne 0x140b3a89e
140b3a6ae: mov rcx, qword ptr [rsi]
140b3a6b1: lea rdx, [rip + 0x1fb33f2] ; 0x142aedaaa b':map with a single key\xab\xda\xaeB\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00'
140b3a6b8: lea rbx, [rip + 0x1fb34d1] ; 0x142aedb90 b'@\xdb\xaeB\x01\x00\x00\x00K\x00\x00\x00\x00\x00\x00\x00\xee\x01\x00\x00\x0e\x00\x00\x00/rustc/05f9846f893b09a1be1fc8560e33f'
140b3a6bf: mov r8d, 1
140b3a6c5: mov r9, rbx
140b3a6c8: call 0x1412f4520
140b3a6cd: mov rcx, qword ptr [rsi]
140b3a6d0: lea rdx, [rip + 0x1fb33d2] ; 0x142aedaa9 b'{:map with a single key\xab\xda\xaeB\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00'
140b3a6d7: mov r8d, 1
140b3a6dd: mov r9, rbx
140b3a6e0: call 0x1412f4520
140b3a6e5: lea r9, [rdi + 0x18]
140b3a6e9: mov qword ptr [rsp + 0x28], rsi
140b3a6ee: mov byte ptr [rsp + 0x30], 1
140b3a6f3: lea rdx, [rip + 0x1fb3610] ; 0x142aedd0a b'wall_indexparamscolor_idflat_roof_styleDeleteRoofVisualEditp'
140b3a6fa: lea rcx, [rsp + 0x28]
140b3a6ff: mov r8d, 0xa
140b3a705: call 0x141584ab0
140b3a70a: test rax, rax
140b3a70d: jne 0x140b3a244
140b3a713: lea r9, [rdi + 0x11]
140b3a717: lea rdx, [rip + 0x1fb36a6] ; 0x142aeddc4 b'fromtoChangeFlatRoofRimStylebeforeafter\x00\x00\x00\x00\x00\x00\xdd\xaeB\x01\x00\x00\x00\n\x00\x00\x00\x00\x00\x00\x00'
140b3a71e: lea rcx, [rsp + 0x28]
140b3a723: mov r8d, 4
140b3a729: call 0x141586250
140b3a72e: test rax, rax
140b3a731: jne 0x140b3a244
140b3a737: add rdi, 0x12
140b3a73b: lea rdx, [rip + 0x1fb3686] ; 0x142aeddc8 b'toChangeFlatRoofRimStylebeforeafter\x00\x00\x00\x00\x00\x00\xdd\xaeB\x01\x00\x00\x00\n\x00\x00\x00\x00\x00\x00\x001\xdd\xaeB'
140b3a742: lea rcx, [rsp + 0x28]
140b3a747: mov r8d, 2
140b3a74d: mov r9, rdi
140b3a750: call 0x141586250
140b3a755: jmp 0x140b3a862
140b3a75a: mov rcx, qword ptr [rsi]
140b3a75d: lea rdx, [rip + 0x1fb3345] ; 0x142aedaa9 b'{:map with a single key\xab\xda\xaeB\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00'
140b3a764: lea r9, [rip + 0x1fb3425] ; 0x142aedb90 b'@\xdb\xaeB\x01\x00\x00\x00K\x00\x00\x00\x00\x00\x00\x00\xee\x01\x00\x00\x0e\x00\x00\x00/rustc/05f9846f893b09a1be1fc8560e33f'
140b3a76b: mov r8d, 1
140b3a771: call 0x1412f4520
140b3a776: lea r8, [rip + 0x1fb35b4] ; 0x142aedd31 b'DeleteRoofVisualEditprev_heightprev_profileprev_tip_offsetpr'
140b3a77d: mov r9d, 0xa
140b3a783: mov rcx, rsi
140b3a786: mov rdx, rbx
140b3a789: call 0x141580d20
140b3a78e: test rax, rax
140b3a791: jne 0x140b3a89e
140b3a797: mov rcx, qword ptr [rsi]
140b3a79a: lea rdx, [rip + 0x1fb3309] ; 0x142aedaaa b':map with a single key\xab\xda\xaeB\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00'
140b3a7a1: lea rbx, [rip + 0x1fb33e8] ; 0x142aedb90 b'@\xdb\xaeB\x01\x00\x00\x00K\x00\x00\x00\x00\x00\x00\x00\xee\x01\x00\x00\x0e\x00\x00\x00/rustc/05f9846f893b09a1be1fc8560e33f'
140b3a7a8: mov r8d, 1
140b3a7ae: mov r9, rbx
140b3a7b1: call 0x1412f4520
140b3a7b6: mov rcx, qword ptr [rsi]
140b3a7b9: lea rdx, [rip + 0x1fb32e9] ; 0x142aedaa9 b'{:map with a single key\xab\xda\xaeB\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00'
140b3a7c0: mov r8d, 1
140b3a7c6: mov r9, rbx
140b3a7c9: call 0x1412f4520
140b3a7ce: lea r9, [rdi + 0x40]
140b3a7d2: mov qword ptr [rsp + 0x28], rsi
140b3a7d7: mov byte ptr [rsp + 0x30], 1
140b3a7dc: lea rdx, [rip + 0x1fb3527] ; 0x142aedd0a b'wall_indexparamscolor_idflat_roof_styleDeleteRoofVisualEditp'
140b3a7e3: lea rcx, [rsp + 0x28]
140b3a7e8: mov r8d, 0xa
140b3a7ee: call 0x141584ab0
140b3a7f3: test rax, rax
140b3a7f6: jne 0x140b3a244
140b3a7fc: lea r9, [rdi + 0x24]
140b3a800: lea rdx, [rip + 0x1fb350d] ; 0x142aedd14 b'paramscolor_idflat_roof_styleDeleteRoofVisualEditprev_height'
140b3a807: lea rcx, [rsp + 0x28]
140b3a80c: mov r8d, 6
140b3a812: call 0x141589d90
140b3a817: test rax, rax
140b3a81a: jne 0x140b3a244
140b3a820: lea r9, [rdi + 0x14]
140b3a824: lea rdx, [rip + 0x1fb34ef] ; 0x142aedd1a b'color_idflat_roof_styleDeleteRoofVisualEditprev_heightprev_p'
140b3a82b: lea rcx, [rsp + 0x28]
140b3a830: mov r8d, 8
140b3a836: call 0x141583a40
140b3a83b: test rax, rax
140b3a83e: jne 0x140b3a244
140b3a844: add rdi, 0x18
140b3a848: lea rdx, [rip + 0x1fb34d3] ; 0x142aedd22 b'flat_roof_styleDeleteRoofVisualEditprev_heightprev_profilepr'
140b3a84f: lea rcx, [rsp + 0x28]
140b3a854: mov r8d, 0xf
140b3a85a: mov r9, rdi
140b3a85d: call 0x14158dc20
140b3a862: test rax, rax
140b3a865: jne 0x140b3a244
140b3a86b: mov rax, qword ptr [rsp + 0x28]
140b3a870: movzx edx, byte ptr [rsp + 0x30]
140b3a875: mov rcx, qword ptr [rax]
140b3a878: call 0x140b36350
140b3a87d: mov rcx, qword ptr [rsi]
140b3a880: lea rdx, [rip + 0x1fb3221] ; 0x142aedaa8 b'}{:map with a single key\xab\xda\xaeB\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00'
140b3a887: lea r9, [rip + 0x1fb3302] ; 0x142aedb90 b'@\xdb\xaeB\x01\x00\x00\x00K\x00\x00\x00\x00\x00\x00\x00\xee\x01\x00\x00\x0e\x00\x00\x00/rustc/05f9846f893b09a1be1fc8560e33f'
140b3a88e: mov r8d, 1
140b3a894: call 0x1412f4520
140b3a899: jmp 0x140b3a242
140b3a89e: mov rcx, rax
140b3a8a1: call 0x1428c98a0
140b3a8a6: jmp 0x140b3a244
140b3a8ab: mov qword ptr [rsp + 0x20], 1
140b3a8b4: mov r8d, 1
140b3a8ba: mov r9d, 1
140b3a8c0: mov rcx, rbx
140b3a8c3: call 0x1428a9a60
140b3a8c8: mov rdx, qword ptr [rbx + 0x10]
140b3a8cc: jmp 0x140b3a200
140b3a8d1: mov qword ptr [rsp + 0x20], 1
140b3a8da: mov r8d, 1
140b3a8e0: mov r9d, 1
140b3a8e6: mov rcx, rsi
140b3a8e9: call 0x1428a9a60
140b3a8ee: mov rdx, qword ptr [rsi + 0x10]
140b3a8f2: jmp 0x140b3a233
140b3a8f7: mov qword ptr [rsp + 0x20], 1
140b3a900: mov r8d, 1
140b3a906: mov r9d, 1
140b3a90c: mov rcx, rbx
140b3a90f: mov r14, rdx
140b3a912: mov rdx, rax
140b3a915: call 0x1428a9a60
140b3a91a: mov rdx, r14
140b3a91d: mov rax, qword ptr [rbx + 0x10]
140b3a921: jmp 0x140b39d88
140b3a926: mov qword ptr [rsp + 0x20], 1
140b3a92f: mov r8d, 1
140b3a935: mov r9d, 1
140b3a93b: mov rcx, rdi
140b3a93e: mov rbx, rdx
140b3a941: mov rdx, rax
140b3a944: call 0x1428a9a60
140b3a949: mov rdx, rbx
140b3a94c: mov rax, qword ptr [rdi + 0x10]
140b3a950: jmp 0x140b39e05
140b3a955: mov qword ptr [rsp + 0x20], 1
140b3a95e: mov r8d, 1
140b3a964: mov r9d, 1
140b3a96a: mov rcx, rbx
140b3a96d: mov r14, rdx
140b3a970: mov rdx, rax
140b3a973: call 0x1428a9a60
140b3a978: mov rdx, r14
140b3a97b: mov rax, qword ptr [rbx + 0x10]
140b3a97f: jmp 0x140b39ea5
140b3a984: mov qword ptr [rsp + 0x20], 1
140b3a98d: mov r8d, 1
140b3a993: mov r9d, 1
140b3a999: mov rcx, rbx
140b3a99c: mov r14, rdx
140b3a99f: mov rdx, rax
140b3a9a2: call 0x1428a9a60
140b3a9a7: mov rdx, r14
140b3a9aa: mov rax, qword ptr [rbx + 0x10]
140b3a9ae: jmp 0x140b39ede
140b3a9b3: mov qword ptr [rsp + 0x20], 1
140b3a9bc: mov r8d, 1
140b3a9c2: mov r9d, 1
140b3a9c8: mov rcx, rbx
140b3a9cb: mov r14, rdx
140b3a9ce: mov rdx, rax
140b3a9d1: call 0x1428a9a60
140b3a9d6: mov rdx, r14
140b3a9d9: mov rax, qword ptr [rbx + 0x10]
140b3a9dd: jmp 0x140b39f5f
140b3a9e2: mov qword ptr [rsp + 0x20], 1
140b3a9eb: mov r8d, 1
140b3a9f1: mov r9d, 1
140b3a9f7: mov rcx, rbx
140b3a9fa: mov r14, rdx
140b3a9fd: mov rdx, rax
140b3aa00: call 0x1428a9a60
140b3aa05: mov rdx, r14
140b3aa08: mov rax, qword ptr [rbx + 0x10]
140b3aa0c: jmp 0x140b39fe0
140b3aa11: mov qword ptr [rsp + 0x20], 1
140b3aa1a: mov r8d, 1
140b3aa20: mov r9d, 1
140b3aa26: mov rcx, rbx
140b3aa29: mov r14, rdx
140b3aa2c: mov rdx, rax
140b3aa2f: call 0x1428a9a60
140b3aa34: mov rdx, r14
140b3aa37: mov rax, qword ptr [rbx + 0x10]
140b3aa3b: jmp 0x140b3a019
140b3aa40: mov rbx, rcx
140b3aa43: mov qword ptr [rsp + 0x20], 1
140b3aa4c: mov r8d, 1
140b3aa52: mov r9d, 1
140b3aa58: mov rcx, rdi
140b3aa5b: mov r14, rdx
140b3aa5e: mov rdx, rax
140b3aa61: call 0x1428a9a60
140b3aa66: mov rdx, r14
140b3aa69: mov rax, qword ptr [rdi + 0x10]
140b3aa6d: jmp 0x140b3a051
140b3aa72: mov qword ptr [rsp + 0x20], 1
140b3aa7b: mov r8d, 1
140b3aa81: mov r9d, 1
140b3aa87: mov rcx, rbx
140b3aa8a: mov r14, rdx
140b3aa8d: mov rdx, rax
140b3aa90: call 0x1428a9a60
140b3aa95: mov rdx, r14
140b3aa98: mov rax, qword ptr [rbx + 0x10]
140b3aa9c: jmp 0x140b39d07
140b3aaa1: mov qword ptr [rsp + 0x20], 1
140b3aaaa: mov r8d, 1
140b3aab0: mov r9d, 1
140b3aab6: mov rcx, rbx
140b3aab9: mov r14, rdx
140b3aabc: mov rdx, rax
140b3aabf: call 0x1428a9a60
140b3aac4: mov rdx, r14
140b3aac7: mov rax, qword ptr [rbx + 0x10]
140b3aacb: jmp 0x140b3a0d3
140b3aad0: mov qword ptr [rsp + 0x20], 1
140b3aad9: mov r8d, 1
140b3aadf: mov r9d, 1
140b3aae5: mov rcx, rbx
140b3aae8: mov r14, rdx
140b3aaeb: mov rdx, rax
140b3aaee: call 0x1428a9a60
140b3aaf3: mov rdx, r14
140b3aaf6: mov rax, qword ptr [rbx + 0x10]
140b3aafa: jmp 0x140b3a10c
140b3aaff: mov qword ptr [rsp + 0x20], 1
140b3ab08: mov r8d, 1
140b3ab0e: mov r9d, 1
140b3ab14: mov rcx, rbx
140b3ab17: mov r14, rdx
140b3ab1a: mov rdx, rax
140b3ab1d: call 0x1428a9a60
140b3ab22: mov rdx, r14
140b3ab25: mov rax, qword ptr [rbx + 0x10]
140b3ab29: jmp 0x140b3a145
140b3ab2e: mov qword ptr [rsp + 0x20], 1
140b3ab37: mov r8d, 1
140b3ab3d: mov r9d, 1
140b3ab43: mov rcx, rbx
140b3ab46: mov r14, rdx
140b3ab49: mov rdx, rax
140b3ab4c: call 0x1428a9a60
140b3ab51: mov rdx, r14
140b3ab54: mov rax, qword ptr [rbx + 0x10]
140b3ab58: jmp 0x140b3a1c3
140b3ab5d: mov qword ptr [rsp + 0x20], 1
140b3ab66: mov r8d, 1
140b3ab6c: mov r9d, 1
140b3ab72: mov rcx, rbx
140b3ab75: mov r14, rdx
140b3ab78: mov rdx, rax
140b3ab7b: call 0x1428a9a60
140b3ab80: mov rdx, r14
140b3ab83: mov rax, qword ptr [rbx + 0x10]
140b3ab87: jmp 0x140b3a266
140b3ab8c: mov qword ptr [rsp + 0x20], 1
140b3ab95: mov r8d, 1
140b3ab9b: mov r9d, 1
140b3aba1: mov rcx, r14
140b3aba4: call 0x1428a9a60
140b3aba9: mov rdx, qword ptr [r14 + 0x10]
140b3abad: jmp 0x140b3a2e7
140b3abb2: mov qword ptr [rsp + 0x20], 1
140b3abbb: mov r8d, 1
140b3abc1: mov r9d, 1
140b3abc7: mov rcx, rbx
140b3abca: call 0x1428a9a60
140b3abcf: mov rdx, qword ptr [rbx + 0x10]
140b3abd3: jmp 0x140b39dc5
140b3abd8: mov qword ptr [rsp + 0x20], 1
140b3abe1: mov r8d, 1
140b3abe7: mov r9d, 1
140b3abed: mov rcx, rdi
140b3abf0: call 0x1428a9a60
140b3abf5: mov rdx, qword ptr [rdi + 0x10]
140b3abf9: jmp 0x140b39e42
140b3abfe: mov qword ptr [rsp + 0x20], 1
140b3ac07: mov r8d, 1
140b3ac0d: mov r9d, 1
140b3ac13: mov rcx, rdi
140b3ac16: call 0x1428a9a60
140b3ac1b: mov rdx, qword ptr [rdi + 0x10]
140b3ac1f: jmp 0x140b39e61
140b3ac24: mov qword ptr [rsp + 0x20], 1
140b3ac2d: mov r8d, 1
140b3ac33: mov r9d, 1
140b3ac39: mov rcx, rdi
140b3ac3c: call 0x1428a9a60
140b3ac41: mov rdx, qword ptr [rdi + 0x10]
140b3ac45: jmp 0x140b39e79
140b3ac4a: mov qword ptr [rsp + 0x20], 1
140b3ac53: mov r8d, 1
140b3ac59: mov r9d, 1
140b3ac5f: mov rcx, rbx
140b3ac62: call 0x1428a9a60
140b3ac67: mov rdx, qword ptr [rbx + 0x10]
140b3ac6b: jmp 0x140b39f1b
140b3ac70: mov qword ptr [rsp + 0x20], 1
140b3ac79: mov r8d, 1
140b3ac7f: mov r9d, 1
140b3ac85: mov rcx, rbx
140b3ac88: call 0x1428a9a60
140b3ac8d: mov rdx, qword ptr [rbx + 0x10]
140b3ac91: jmp 0x140b39f9c
140b3ac96: mov qword ptr [rsp + 0x20], 1
140b3ac9f: mov r8d, 1
140b3aca5: mov r9d, 1
140b3acab: mov rcx, rdi
140b3acae: call 0x1428a9a60
140b3acb3: mov rdx, qword ptr [rdi + 0x10]
140b3acb7: jmp 0x140b3a08e
140b3acbc: mov qword ptr [rsp + 0x20], 1
140b3acc5: mov r8d, 1
140b3accb: mov r9d, 1
140b3acd1: mov rcx, rbx
140b3acd4: call 0x1428a9a60
140b3acd9: mov rdx, qword ptr [rbx + 0x10]
140b3acdd: jmp 0x140b39d44
140b3ace2: mov qword ptr [rsp + 0x20], 1
140b3aceb: mov r8d, 1
140b3acf1: mov r9d, 1
140b3acf7: mov rcx, rbx
140b3acfa: call 0x1428a9a60
140b3acff: mov rdx, qword ptr [rbx + 0x10]
140b3ad03: jmp 0x140b3a182
140b3ad08: mov qword ptr [rsp + 0x20], 1
140b3ad11: mov r8d, 1
140b3ad17: mov r9d, 1
140b3ad1d: mov rcx, rbx
140b3ad20: call 0x1428a9a60
140b3ad25: mov rdx, qword ptr [rbx + 0x10]
140b3ad29: jmp 0x140b3a2a3
140b3ad2e: mov qword ptr [rsp + 0x20], 1
140b3ad37: mov r8d, 1
140b3ad3d: mov r9d, 1
140b3ad43: mov rcx, r14
140b3ad46: call 0x1428a9a60
140b3ad4b: mov rdx, qword ptr [r14 + 0x10]
140b3ad4f: jmp 0x140b3a323
