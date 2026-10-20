country_core::systems::audio::water_audio::water_audio_init_system @ 0x140873140
140873141: push rsi
140873142: push rdi
140873143: sub rsp, 0x210
14087314a: lea rbp, [rsp + 0x80]
140873152: mov qword ptr [rbp + 0x188], 0xfffffffffffffffe
14087315d: cmp byte ptr [rip + 0x2dc1c2c], 0
140873164: jne 0x1408733e8
14087316a: mov rsi, rdx
14087316d: mov rdi, qword ptr [rcx]
140873170: mov qword ptr [rbp + 0x160], 3
14087317b: mov dword ptr [rbp + 0x168], 0x32
140873185: lea rax, [rip + 0x2224f5c]
14087318c: mov qword ptr [rbp + 0x150], rax
140873193: mov qword ptr [rbp + 0x158], 0x1a
14087319e: lea rcx, [rbp + 0x70]
1408731a2: lea r8, [rbp + 0x150]
1408731a9: mov rdx, rdi
1408731ac: call 0x140a59220
1408731b1: xor eax, eax
1408731b3: cmp rax, qword ptr [rbp + 0x70]
1408731b7: jo 0x1408733f4
1408731bd: mov rax, qword ptr [rbp + 0xa0]
1408731c4: mov qword ptr [rbp + 0x60], rax
1408731c8: movups xmm0, xmmword ptr [rbp + 0x70]
1408731cc: movups xmm1, xmmword ptr [rbp + 0x80]
1408731d3: movups xmm2, xmmword ptr [rbp + 0x90]
1408731da: movaps xmmword ptr [rbp + 0x50], xmm2
1408731de: movaps xmmword ptr [rbp + 0x40], xmm1
1408731e2: movaps xmmword ptr [rbp + 0x30], xmm0
1408731b7: jo 0x1408733f4
1408731bd: mov rax, qword ptr [rbp + 0xa0]
1408731c4: mov qword ptr [rbp + 0x60], rax
1408731c8: movups xmm0, xmmword ptr [rbp + 0x70]
1408731cc: movups xmm1, xmmword ptr [rbp + 0x80]
1408731d3: movups xmm2, xmmword ptr [rbp + 0x90]
1408731da: movaps xmmword ptr [rbp + 0x50], xmm2
1408731de: movaps xmmword ptr [rbp + 0x40], xmm1
1408731e2: movaps xmmword ptr [rbp + 0x30], xmm0
1408731e6: mov qword ptr [rbp + 0x160], 3
1408731f1: mov dword ptr [rbp + 0x168], 0x32
1408731fb: lea rax, [rip + 0x2224f4e]
140873202: mov qword ptr [rbp + 0x150], rax
140873209: mov qword ptr [rbp + 0x158], 0x1b
140873214: lea rcx, [rbp + 0x70]
140873218: lea r8, [rbp + 0x150]
14087321f: mov rdx, rdi
140873222: call 0x140a59220
140873227: xor eax, eax
140873229: cmp rax, qword ptr [rbp + 0x70]
14087322d: jo 0x14087342f
140873233: mov rax, qword ptr [rbp + 0xa0]
14087323a: mov qword ptr [rbp + 0x20], rax
14087323e: movups xmm0, xmmword ptr [rbp + 0x70]
140873242: movups xmm1, xmmword ptr [rbp + 0x80]
140873249: movups xmm2, xmmword ptr [rbp + 0x90]
140873250: movaps xmmword ptr [rbp + 0x10], xmm2
140873254: movaps xmmword ptr [rbp], xmm1
140873258: movaps xmmword ptr [rbp - 0x10], xmm0
14087322d: jo 0x14087342f
140873233: mov rax, qword ptr [rbp + 0xa0]
14087323a: mov qword ptr [rbp + 0x20], rax
14087323e: movups xmm0, xmmword ptr [rbp + 0x70]
140873242: movups xmm1, xmmword ptr [rbp + 0x80]
140873249: movups xmm2, xmmword ptr [rbp + 0x90]
140873250: movaps xmmword ptr [rbp + 0x10], xmm2
140873254: movaps xmmword ptr [rbp], xmm1
140873258: movaps xmmword ptr [rbp - 0x10], xmm0
14087325c: mov qword ptr [rbp + 0x160], 3
140873267: mov dword ptr [rbp + 0x168], 0x32
140873271: lea rax, [rip + 0x2224f10]
140873278: mov qword ptr [rbp + 0x150], rax
14087327f: mov qword ptr [rbp + 0x158], 0x18
14087328a: lea rcx, [rbp + 0x70]
14087328e: lea r8, [rbp + 0x150]
140873295: mov rdx, rdi
140873298: call 0x140a59220
14087329d: xor eax, eax
14087329f: cmp rax, qword ptr [rbp + 0x70]
1408732a3: jo 0x140873467
1408732a9: mov rax, qword ptr [rbp + 0xa0]
1408732b0: mov qword ptr [rbp - 0x20], rax
1408732b4: movups xmm0, xmmword ptr [rbp + 0x70]
1408732b8: movups xmm1, xmmword ptr [rbp + 0x80]
1408732bf: movups xmm2, xmmword ptr [rbp + 0x90]
1408732c6: movaps xmmword ptr [rbp - 0x30], xmm2
1408732ca: movaps xmmword ptr [rbp - 0x40], xmm1
1408732ce: movaps xmmword ptr [rbp - 0x50], xmm0
1408732a3: jo 0x140873467
1408732a9: mov rax, qword ptr [rbp + 0xa0]
1408732b0: mov qword ptr [rbp - 0x20], rax
1408732b4: movups xmm0, xmmword ptr [rbp + 0x70]
1408732b8: movups xmm1, xmmword ptr [rbp + 0x80]
1408732bf: movups xmm2, xmmword ptr [rbp + 0x90]
1408732c6: movaps xmmword ptr [rbp - 0x30], xmm2
1408732ca: movaps xmmword ptr [rbp - 0x40], xmm1
1408732ce: movaps xmmword ptr [rbp - 0x50], xmm0
1408732d2: mov qword ptr [rbp + 0x80], 3
1408732dd: mov dword ptr [rbp + 0x88], 0x32
1408732e7: lea rax, [rip + 0x2224eca]
1408732ee: mov qword ptr [rbp + 0x70], rax
1408732f2: mov qword ptr [rbp + 0x78], 0x19
1408732fa: lea rcx, [rbp + 0x150]
140873301: lea r8, [rbp + 0x70]
140873305: mov rdx, rdi
140873308: call 0x140a59220
14087330d: xor eax, eax
14087330f: cmp rax, qword ptr [rbp + 0x150]
140873316: jo 0x14087349f
14087331c: mov rax, qword ptr [rbp + 0x180]
140873323: mov qword ptr [rbp + 0x148], rax
14087332a: movups xmm0, xmmword ptr [rbp + 0x150]
140873331: movups xmm1, xmmword ptr [rbp + 0x160]
140873338: movups xmm2, xmmword ptr [rbp + 0x170]
14087333f: movups xmmword ptr [rbp + 0x138], xmm2
140873346: movups xmmword ptr [rbp + 0x128], xmm1
14087334d: movups xmmword ptr [rbp + 0x118], xmm0

country_core::systems::audio::wall_audio_fmod::wall_audio_init_system @ 0x14094fb80
14094fb80: push rbp
14094fb81: push rsi
14094fb82: push rdi
14094fb83: sub rsp, 0x540
14094fb8a: lea rbp, [rsp + 0x80]
14094fb92: mov qword ptr [rbp + 0x4b8], 0xfffffffffffffffe
14094fb9d: mov rsi, rdx
14094fba0: mov rdi, qword ptr [rcx]
14094fba3: mov qword ptr [rbp + 0x480], 3
14094fbae: mov dword ptr [rbp + 0x488], 0x32
14094fbb8: lea rax, [rip + 0x21643b1]
14094fbbf: mov qword ptr [rbp + 0x470], rax
14094fbc6: mov qword ptr [rbp + 0x478], 0x14
14094fbd1: lea rcx, [rbp + 0x1e0]
14094fbd8: lea r8, [rbp + 0x470]
14094fbdf: mov rdx, rdi
14094fbe2: call 0x140a59220
14094fbe7: xor eax, eax
14094fbe9: cmp rax, qword ptr [rbp + 0x1e0]
14094fbf0: jo 0x140950327
14094fbf6: mov rax, qword ptr [rbp + 0x210]
14094fbfd: mov qword ptr [rbp + 0x1d0], rax
14094fc04: movups xmm0, xmmword ptr [rbp + 0x1e0]
14094fc0b: movups xmm1, xmmword ptr [rbp + 0x1f0]
14094fc12: movups xmm2, xmmword ptr [rbp + 0x200]
14094fc19: movaps xmmword ptr [rbp + 0x1c0], xmm2
14094fc20: movaps xmmword ptr [rbp + 0x1b0], xmm1
14094fc27: movaps xmmword ptr [rbp + 0x1a0], xmm0
14094fbf6: mov rax, qword ptr [rbp + 0x210]
14094fbfd: mov qword ptr [rbp + 0x1d0], rax
14094fc04: movups xmm0, xmmword ptr [rbp + 0x1e0]
14094fc0b: movups xmm1, xmmword ptr [rbp + 0x1f0]
14094fc12: movups xmm2, xmmword ptr [rbp + 0x200]
14094fc19: movaps xmmword ptr [rbp + 0x1c0], xmm2
14094fc20: movaps xmmword ptr [rbp + 0x1b0], xmm1
14094fc27: movaps xmmword ptr [rbp + 0x1a0], xmm0
14094fc2e: mov byte ptr [rbp + 0x1d8], 0
14094fc35: mov qword ptr [rbp + 0x480], 3
14094fc40: mov dword ptr [rbp + 0x488], 0x32
14094fc4a: lea rax, [rip + 0x2164387]
14094fc51: mov qword ptr [rbp + 0x470], rax
14094fc58: mov qword ptr [rbp + 0x478], 0x1d
14094fc63: lea rcx, [rbp + 0x1e0]
14094fc6a: lea r8, [rbp + 0x470]
14094fc71: mov rdx, rdi
14094fc74: call 0x140a59220
14094fc79: xor eax, eax
14094fc7b: cmp rax, qword ptr [rbp + 0x1e0]
14094fc82: jo 0x140950365
14094fc88: mov rax, qword ptr [rbp + 0x210]
14094fc8f: mov qword ptr [rbp + 0x190], rax
14094fc96: movups xmm0, xmmword ptr [rbp + 0x1e0]
14094fc9d: movups xmm1, xmmword ptr [rbp + 0x1f0]
14094fca4: movups xmm2, xmmword ptr [rbp + 0x200]
14094fcab: movaps xmmword ptr [rbp + 0x180], xmm2
14094fcb2: movaps xmmword ptr [rbp + 0x170], xmm1
14094fcb9: movaps xmmword ptr [rbp + 0x160], xmm0
14094fc88: mov rax, qword ptr [rbp + 0x210]
14094fc8f: mov qword ptr [rbp + 0x190], rax
14094fc96: movups xmm0, xmmword ptr [rbp + 0x1e0]
14094fc9d: movups xmm1, xmmword ptr [rbp + 0x1f0]
14094fca4: movups xmm2, xmmword ptr [rbp + 0x200]
14094fcab: movaps xmmword ptr [rbp + 0x180], xmm2
14094fcb2: movaps xmmword ptr [rbp + 0x170], xmm1
14094fcb9: movaps xmmword ptr [rbp + 0x160], xmm0
14094fcc0: mov byte ptr [rbp + 0x198], 0
14094fcc7: mov qword ptr [rbp + 0x480], 3
14094fcd2: mov dword ptr [rbp + 0x488], 0x32
14094fcdc: lea rax, [rip + 0x216432d]
14094fce3: mov qword ptr [rbp + 0x470], rax
14094fcea: mov qword ptr [rbp + 0x478], 0x16
14094fcf5: lea rcx, [rbp + 0x1e0]
14094fcfc: lea r8, [rbp + 0x470]
14094fd03: mov rdx, rdi
14094fd06: call 0x140a59220
14094fd0b: xor eax, eax
14094fd0d: cmp rax, qword ptr [rbp + 0x1e0]
14094fd14: jo 0x1409503a3
14094fd1a: mov rax, qword ptr [rbp + 0x210]
14094fd21: mov qword ptr [rbp + 0x150], rax
14094fd28: movups xmm0, xmmword ptr [rbp + 0x1e0]
14094fd2f: movups xmm1, xmmword ptr [rbp + 0x1f0]
14094fd36: movups xmm2, xmmword ptr [rbp + 0x200]
14094fd3d: movaps xmmword ptr [rbp + 0x140], xmm2
14094fd44: movaps xmmword ptr [rbp + 0x130], xmm1
14094fd4b: movaps xmmword ptr [rbp + 0x120], xmm0
14094fd1a: mov rax, qword ptr [rbp + 0x210]
14094fd21: mov qword ptr [rbp + 0x150], rax
14094fd28: movups xmm0, xmmword ptr [rbp + 0x1e0]
14094fd2f: movups xmm1, xmmword ptr [rbp + 0x1f0]
14094fd36: movups xmm2, xmmword ptr [rbp + 0x200]
14094fd3d: movaps xmmword ptr [rbp + 0x140], xmm2
14094fd44: movaps xmmword ptr [rbp + 0x130], xmm1
14094fd4b: movaps xmmword ptr [rbp + 0x120], xmm0
14094fd52: mov byte ptr [rbp + 0x158], 0
14094fd59: mov qword ptr [rbp + 0x480], 3
14094fd64: mov dword ptr [rbp + 0x488], 0x32
14094fd6e: lea rax, [rip + 0x21642cb]
14094fd75: mov qword ptr [rbp + 0x470], rax
14094fd7c: mov qword ptr [rbp + 0x478], 0x17
14094fd87: lea rcx, [rbp + 0x1e0]
14094fd8e: lea r8, [rbp + 0x470]
14094fd95: mov rdx, rdi
14094fd98: call 0x140a59220
14094fd9d: xor eax, eax
14094fd9f: cmp rax, qword ptr [rbp + 0x1e0]
14094fda6: jo 0x1409503e1
14094fdac: mov rax, qword ptr [rbp + 0x210]
14094fdb3: mov qword ptr [rbp + 0x110], rax
14094fdba: movups xmm0, xmmword ptr [rbp + 0x1e0]
14094fdc1: movups xmm1, xmmword ptr [rbp + 0x1f0]
14094fdc8: movups xmm2, xmmword ptr [rbp + 0x200]
14094fdcf: movaps xmmword ptr [rbp + 0x100], xmm2
14094fdd6: movaps xmmword ptr [rbp + 0xf0], xmm1
14094fddd: movaps xmmword ptr [rbp + 0xe0], xmm0
14094fdac: mov rax, qword ptr [rbp + 0x210]
14094fdb3: mov qword ptr [rbp + 0x110], rax
14094fdba: movups xmm0, xmmword ptr [rbp + 0x1e0]
14094fdc1: movups xmm1, xmmword ptr [rbp + 0x1f0]
14094fdc8: movups xmm2, xmmword ptr [rbp + 0x200]
14094fdcf: movaps xmmword ptr [rbp + 0x100], xmm2
14094fdd6: movaps xmmword ptr [rbp + 0xf0], xmm1
14094fddd: movaps xmmword ptr [rbp + 0xe0], xmm0
14094fde4: mov byte ptr [rbp + 0x118], 0
14094fdeb: mov qword ptr [rbp + 0x480], 3
14094fdf6: mov dword ptr [rbp + 0x488], 0x32
14094fe00: lea rax, [rip + 0x2164269]
14094fe07: mov qword ptr [rbp + 0x470], rax
14094fe0e: mov qword ptr [rbp + 0x478], 0x1a
14094fe19: lea rcx, [rbp + 0x1e0]
14094fe20: lea r8, [rbp + 0x470]
14094fe27: mov rdx, rdi
14094fe2a: call 0x140a59220
14094fe2f: xor eax, eax
14094fe31: cmp rax, qword ptr [rbp + 0x1e0]
14094fe38: jo 0x14095041f
14094fe3e: mov rax, qword ptr [rbp + 0x210]
14094fe45: mov qword ptr [rbp + 0xd0], rax
14094fe4c: movups xmm0, xmmword ptr [rbp + 0x1e0]
14094fe53: movups xmm1, xmmword ptr [rbp + 0x1f0]
14094fe5a: movups xmm2, xmmword ptr [rbp + 0x200]
14094fe61: movaps xmmword ptr [rbp + 0xc0], xmm2
14094fe68: movaps xmmword ptr [rbp + 0xb0], xmm1
14094fe6f: movaps xmmword ptr [rbp + 0xa0], xmm0
14094fe3e: mov rax, qword ptr [rbp + 0x210]
14094fe45: mov qword ptr [rbp + 0xd0], rax
14094fe4c: movups xmm0, xmmword ptr [rbp + 0x1e0]
14094fe53: movups xmm1, xmmword ptr [rbp + 0x1f0]
14094fe5a: movups xmm2, xmmword ptr [rbp + 0x200]
14094fe61: movaps xmmword ptr [rbp + 0xc0], xmm2
14094fe68: movaps xmmword ptr [rbp + 0xb0], xmm1
14094fe6f: movaps xmmword ptr [rbp + 0xa0], xmm0
14094fe76: mov byte ptr [rbp + 0xd8], 0
14094fe7d: mov qword ptr [rbp + 0x480], 3
14094fe88: mov dword ptr [rbp + 0x488], 0x32
14094fe92: lea rax, [rip + 0x216420f]
14094fe99: mov qword ptr [rbp + 0x470], rax
14094fea0: mov qword ptr [rbp + 0x478], 0x17
14094feab: lea rcx, [rbp + 0x1e0]
14094feb2: lea r8, [rbp + 0x470]
14094feb9: mov rdx, rdi
14094febc: call 0x140a59220
14094fec1: xor eax, eax
14094fec3: cmp rax, qword ptr [rbp + 0x1e0]
14094feca: jo 0x14095045d
14094fed0: mov rax, qword ptr [rbp + 0x210]
14094fed7: mov qword ptr [rbp + 0x90], rax
14094fede: movups xmm0, xmmword ptr [rbp + 0x1e0]
14094fee5: movups xmm1, xmmword ptr [rbp + 0x1f0]
14094feec: movups xmm2, xmmword ptr [rbp + 0x200]
14094fef3: movaps xmmword ptr [rbp + 0x80], xmm2
14094fefa: movaps xmmword ptr [rbp + 0x70], xmm1
14094fefe: movaps xmmword ptr [rbp + 0x60], xmm0
14094fed0: mov rax, qword ptr [rbp + 0x210]
14094fed7: mov qword ptr [rbp + 0x90], rax
14094fede: movups xmm0, xmmword ptr [rbp + 0x1e0]
14094fee5: movups xmm1, xmmword ptr [rbp + 0x1f0]
14094feec: movups xmm2, xmmword ptr [rbp + 0x200]
14094fef3: movaps xmmword ptr [rbp + 0x80], xmm2
14094fefa: movaps xmmword ptr [rbp + 0x70], xmm1
14094fefe: movaps xmmword ptr [rbp + 0x60], xmm0
14094ff02: mov byte ptr [rbp + 0x98], 0
14094ff09: mov qword ptr [rbp + 0x480], 3
14094ff14: mov dword ptr [rbp + 0x488], 0x32
14094ff1e: lea rax, [rip + 0x21641b3]
14094ff25: mov qword ptr [rbp + 0x470], rax
14094ff2c: mov qword ptr [rbp + 0x478], 0x1f
14094ff37: lea rcx, [rbp + 0x1e0]
14094ff3e: lea r8, [rbp + 0x470]
14094ff45: mov rdx, rdi
14094ff48: call 0x140a59220
14094ff4d: xor eax, eax
14094ff4f: cmp rax, qword ptr [rbp + 0x1e0]
14094ff56: jo 0x14095049b
14094ff5c: mov rax, qword ptr [rbp + 0x210]
14094ff63: mov qword ptr [rbp + 0x50], rax
14094ff67: movups xmm0, xmmword ptr [rbp + 0x1e0]
14094ff6e: movups xmm1, xmmword ptr [rbp + 0x1f0]
14094ff75: movups xmm2, xmmword ptr [rbp + 0x200]
14094ff7c: movaps xmmword ptr [rbp + 0x40], xmm2
14094ff80: movaps xmmword ptr [rbp + 0x30], xmm1
14094ff84: movaps xmmword ptr [rbp + 0x20], xmm0
14094ff5c: mov rax, qword ptr [rbp + 0x210]
14094ff63: mov qword ptr [rbp + 0x50], rax
14094ff67: movups xmm0, xmmword ptr [rbp + 0x1e0]
14094ff6e: movups xmm1, xmmword ptr [rbp + 0x1f0]
14094ff75: movups xmm2, xmmword ptr [rbp + 0x200]
14094ff7c: movaps xmmword ptr [rbp + 0x40], xmm2
14094ff80: movaps xmmword ptr [rbp + 0x30], xmm1
14094ff84: movaps xmmword ptr [rbp + 0x20], xmm0
14094ff88: mov byte ptr [rbp + 0x58], 0
14094ff8c: mov qword ptr [rbp + 0x480], 3
14094ff97: mov dword ptr [rbp + 0x488], 0x32
14094ffa1: lea rax, [rip + 0x2164168]
14094ffa8: mov qword ptr [rbp + 0x470], rax
14094ffaf: mov qword ptr [rbp + 0x478], 0x19
14094ffba: lea rcx, [rbp + 0x1e0]
14094ffc1: lea r8, [rbp + 0x470]
14094ffc8: mov rdx, rdi
14094ffcb: call 0x140a59220
14094ffd0: xor eax, eax
14094ffd2: cmp rax, qword ptr [rbp + 0x1e0]
14094ffd9: jo 0x1409504d9
14094ffdf: mov rax, qword ptr [rbp + 0x210]
14094ffe6: mov qword ptr [rbp + 0x10], rax
14094ffea: movups xmm0, xmmword ptr [rbp + 0x1e0]
14094fff1: movups xmm1, xmmword ptr [rbp + 0x1f0]
14094fff8: movups xmm2, xmmword ptr [rbp + 0x200]
14094ffff: movaps xmmword ptr [rbp], xmm2
140950003: movaps xmmword ptr [rbp - 0x10], xmm1
140950007: movaps xmmword ptr [rbp - 0x20], xmm0
14094ffdf: mov rax, qword ptr [rbp + 0x210]
14094ffe6: mov qword ptr [rbp + 0x10], rax
14094ffea: movups xmm0, xmmword ptr [rbp + 0x1e0]
14094fff1: movups xmm1, xmmword ptr [rbp + 0x1f0]
14094fff8: movups xmm2, xmmword ptr [rbp + 0x200]
14094ffff: movaps xmmword ptr [rbp], xmm2
140950003: movaps xmmword ptr [rbp - 0x10], xmm1
140950007: movaps xmmword ptr [rbp - 0x20], xmm0
14095000b: mov byte ptr [rbp + 0x18], 0
14095000f: mov qword ptr [rbp + 0x480], 3
14095001a: mov dword ptr [rbp + 0x488], 0x32
140950024: lea rax, [rip + 0x216411d]
14095002b: mov qword ptr [rbp + 0x470], rax
140950032: mov qword ptr [rbp + 0x478], 0x16
14095003d: lea rcx, [rbp + 0x1e0]
140950044: lea r8, [rbp + 0x470]
14095004b: mov rdx, rdi
14095004e: call 0x140a59220
140950053: xor eax, eax
140950055: cmp rax, qword ptr [rbp + 0x1e0]
14095005c: jo 0x140950514
140950062: mov rax, qword ptr [rbp + 0x210]
140950069: mov qword ptr [rbp + 0x4a0], rax
140950070: movups xmm0, xmmword ptr [rbp + 0x1e0]
140950077: movups xmm1, xmmword ptr [rbp + 0x1f0]
14095007e: movups xmm2, xmmword ptr [rbp + 0x200]
140950085: movaps xmmword ptr [rbp + 0x490], xmm2
14095008c: movaps xmmword ptr [rbp + 0x480], xmm1
140950093: movaps xmmword ptr [rbp + 0x470], xmm0
140950062: mov rax, qword ptr [rbp + 0x210]
140950069: mov qword ptr [rbp + 0x4a0], rax
140950070: movups xmm0, xmmword ptr [rbp + 0x1e0]
140950077: movups xmm1, xmmword ptr [rbp + 0x1f0]
14095007e: movups xmm2, xmmword ptr [rbp + 0x200]
140950085: movaps xmmword ptr [rbp + 0x490], xmm2
14095008c: movaps xmmword ptr [rbp + 0x480], xmm1
140950093: movaps xmmword ptr [rbp + 0x470], xmm0
14095009a: mov byte ptr [rbp + 0x4a8], 0
1409500a1: mov qword ptr [rbp + 0x1f0], 3
1409500ac: mov dword ptr [rbp + 0x1f8], 0x32
1409500b6: lea rax, [rip + 0x21640bb]
1409500bd: mov qword ptr [rbp + 0x1e0], rax
1409500c4: mov qword ptr [rbp + 0x1e8], 0x14
1409500cf: lea rcx, [rbp - 0x58]
1409500d3: lea r8, [rbp + 0x1e0]
1409500da: mov rdx, rdi
1409500dd: call 0x140a59220
1409500e2: xor eax, eax
1409500e4: cmp rax, qword ptr [rbp - 0x58]
1409500e8: jo 0x14095054f
1409500ee: mov rax, qword ptr [rbp - 0x28]
1409500f2: mov qword ptr [rbp + 0x2d0], rax
1409500f9: movups xmm0, xmmword ptr [rbp - 0x58]
1409500fd: movups xmm1, xmmword ptr [rbp - 0x48]
140950101: movups xmm2, xmmword ptr [rbp - 0x38]
140950105: movaps xmmword ptr [rbp + 0x2c0], xmm2
14095010c: movaps xmmword ptr [rbp + 0x2b0], xmm1
140950113: movaps xmmword ptr [rbp + 0x2a0], xmm0

country_core::systems::audio::path_audio::path_audio_init_system @ 0x1409d08d0
1409d08d0: push rbp
1409d08d1: push rsi
1409d08d2: push rdi
1409d08d3: sub rsp, 0x250
1409d08da: lea rbp, [rsp + 0x80]
1409d08e2: mov qword ptr [rbp + 0x1c8], 0xfffffffffffffffe
1409d08ed: mov rsi, rdx
1409d08f0: mov rdi, qword ptr [rcx]
1409d08f3: mov qword ptr [rbp + 0x1a0], 3
1409d08fe: mov dword ptr [rbp + 0x1a8], 0x32
1409d0908: lea rax, [rip + 0x20f0ceb]
1409d090f: mov qword ptr [rbp + 0x190], rax
1409d0916: mov qword ptr [rbp + 0x198], 0x1a
1409d0921: lea rcx, [rbp + 0x70]
1409d0925: lea r8, [rbp + 0x190]
1409d092c: mov rdx, rdi
1409d092f: call 0x140a59220
1409d0934: mov rcx, qword ptr [rbp + 0x70]
1409d0938: mov rax, qword ptr [rbp + 0x78]
1409d093c: mov rdx, rcx
1409d093f: neg rdx
1409d0942: jo 0x1409d0ce4
1409d0948: mov edx, dword ptr [rbp + 0x90]
1409d094e: mov r8, qword ptr [rbp + 0x98]
1409d0955: mov r9d, dword ptr [rbp + 0xa0]
1409d095c: movzx r10d, word ptr [rbp + 0xa4]
1409d0964: movups xmm0, xmmword ptr [rbp + 0x80]
1409d096b: mov qword ptr [rbp + 0x38], rcx
1409d0964: movups xmm0, xmmword ptr [rbp + 0x80]
1409d096b: mov qword ptr [rbp + 0x38], rcx
1409d096f: mov qword ptr [rbp + 0x40], rax
1409d0973: movups xmmword ptr [rbp + 0x48], xmm0
1409d0977: mov dword ptr [rbp + 0x58], edx
1409d097a: mov qword ptr [rbp + 0x60], r8
1409d097e: mov dword ptr [rbp + 0x68], r9d
1409d0982: mov word ptr [rbp + 0x6c], r10w
1409d0987: mov byte ptr [rbp + 0x6e], 1
1409d098b: mov qword ptr [rbp + 0x1a0], 3
1409d0996: mov dword ptr [rbp + 0x1a8], 0x32
1409d09a0: lea rax, [rip + 0x20f0cb9]
1409d09a7: mov qword ptr [rbp + 0x190], rax
1409d09ae: mov qword ptr [rbp + 0x198], 0x14
1409d09b9: lea rcx, [rbp + 0x70]
1409d09bd: lea r8, [rbp + 0x190]
1409d09c4: mov rdx, rdi
1409d09c7: call 0x140a59220
1409d09cc: mov rcx, qword ptr [rbp + 0x70]
1409d09d0: mov rax, qword ptr [rbp + 0x78]
1409d09d4: mov rdx, rcx
1409d09d7: neg rdx
1409d09da: jo 0x1409d0d15
1409d09e0: mov edx, dword ptr [rbp + 0x90]
1409d09e6: mov r8, qword ptr [rbp + 0x98]
1409d09ed: mov r9d, dword ptr [rbp + 0xa0]
1409d09f4: movzx r10d, word ptr [rbp + 0xa4]
1409d09fc: movups xmm0, xmmword ptr [rbp + 0x80]
1409d0a03: mov qword ptr [rbp], rcx
1409d09fc: movups xmm0, xmmword ptr [rbp + 0x80]
1409d0a03: mov qword ptr [rbp], rcx
1409d0a07: mov qword ptr [rbp + 8], rax
1409d0a0b: movups xmmword ptr [rbp + 0x10], xmm0
1409d0a0f: mov dword ptr [rbp + 0x20], edx
1409d0a12: mov qword ptr [rbp + 0x28], r8
1409d0a16: mov dword ptr [rbp + 0x30], r9d
1409d0a1a: mov word ptr [rbp + 0x34], r10w
1409d0a1f: mov byte ptr [rbp + 0x36], 1
1409d0a23: mov qword ptr [rbp + 0x1a0], 3
1409d0a2e: mov dword ptr [rbp + 0x1a8], 0x32
1409d0a38: lea rax, [rip + 0x20f0c51]
1409d0a3f: mov qword ptr [rbp + 0x190], rax
1409d0a46: mov qword ptr [rbp + 0x198], 0x15
1409d0a51: lea rcx, [rbp + 0x70]
1409d0a55: lea r8, [rbp + 0x190]
1409d0a5c: mov rdx, rdi
1409d0a5f: call 0x140a59220
1409d0a64: mov rcx, qword ptr [rbp + 0x70]
1409d0a68: mov rax, qword ptr [rbp + 0x78]
1409d0a6c: mov rdx, rcx
1409d0a6f: neg rdx
1409d0a72: jo 0x1409d0d46
1409d0a78: mov edx, dword ptr [rbp + 0x90]
1409d0a7e: mov r8, qword ptr [rbp + 0x98]
1409d0a85: mov r9d, dword ptr [rbp + 0xa0]
1409d0a8c: movzx r10d, word ptr [rbp + 0xa4]
1409d0a94: movups xmm0, xmmword ptr [rbp + 0x80]
1409d0a9b: mov qword ptr [rbp - 0x38], rcx
1409d0a94: movups xmm0, xmmword ptr [rbp + 0x80]
1409d0a9b: mov qword ptr [rbp - 0x38], rcx
1409d0a9f: mov qword ptr [rbp - 0x30], rax
1409d0aa3: movups xmmword ptr [rbp - 0x28], xmm0
1409d0aa7: mov dword ptr [rbp - 0x18], edx
1409d0aaa: mov qword ptr [rbp - 0x10], r8
1409d0aae: mov dword ptr [rbp - 8], r9d
1409d0ab2: mov word ptr [rbp - 4], r10w
1409d0ab7: mov byte ptr [rbp - 2], 1
1409d0abb: mov qword ptr [rbp + 0x1a0], 3
1409d0ac6: mov dword ptr [rbp + 0x1a8], 0x32
1409d0ad0: lea rax, [rip + 0x20f0be9]
1409d0ad7: mov qword ptr [rbp + 0x190], rax
1409d0ade: mov qword ptr [rbp + 0x198], 0x1e
1409d0ae9: lea rcx, [rbp + 0x70]
1409d0aed: lea r8, [rbp + 0x190]
1409d0af4: mov rdx, rdi
1409d0af7: call 0x140a59220
1409d0afc: mov rcx, qword ptr [rbp + 0x70]
1409d0b00: mov rax, qword ptr [rbp + 0x78]
1409d0b04: mov rdx, rcx
1409d0b07: neg rdx
1409d0b0a: jo 0x1409d0d74
1409d0b10: mov edx, dword ptr [rbp + 0x90]
1409d0b16: mov r8, qword ptr [rbp + 0x98]
1409d0b1d: mov r9d, dword ptr [rbp + 0xa0]
1409d0b24: movzx r10d, word ptr [rbp + 0xa4]
1409d0b2c: movups xmm0, xmmword ptr [rbp + 0x80]
1409d0b33: mov qword ptr [rbp + 0x190], rcx
1409d0b2c: movups xmm0, xmmword ptr [rbp + 0x80]
1409d0b33: mov qword ptr [rbp + 0x190], rcx
1409d0b3a: mov qword ptr [rbp + 0x198], rax
1409d0b41: movups xmmword ptr [rbp + 0x1a0], xmm0
1409d0b48: mov dword ptr [rbp + 0x1b0], edx
1409d0b4e: mov qword ptr [rbp + 0x1b8], r8
1409d0b55: mov dword ptr [rbp + 0x1c0], r9d
1409d0b5c: mov word ptr [rbp + 0x1c4], r10w
1409d0b64: mov byte ptr [rbp + 0x1c6], 1
1409d0b6b: mov qword ptr [rbp - 0x48], 3
1409d0b73: mov dword ptr [rbp - 0x40], 0x32
1409d0b7a: lea rax, [rip + 0x20f0b77]
1409d0b81: mov qword ptr [rbp - 0x58], rax
1409d0b85: mov qword ptr [rbp - 0x50], 0x1f
1409d0b8d: lea rcx, [rbp + 0x70]
1409d0b91: lea r8, [rbp - 0x58]
1409d0b95: mov rdx, rdi
1409d0b98: call 0x140a59220
1409d0b9d: mov rcx, qword ptr [rbp + 0x70]
1409d0ba1: mov rax, qword ptr [rbp + 0x78]
1409d0ba5: mov rdx, rcx
1409d0ba8: neg rdx
1409d0bab: jo 0x1409d0da2
1409d0bb1: mov edx, dword ptr [rbp + 0x90]
1409d0bb7: mov r8, qword ptr [rbp + 0x98]
1409d0bbe: mov r9d, dword ptr [rbp + 0xa0]
1409d0bc5: movzx r10d, word ptr [rbp + 0xa4]
1409d0bcd: movups xmm0, xmmword ptr [rbp + 0x80]
1409d0bd4: mov qword ptr [rbp + 0x150], rcx

country_core::systems::audio::signifier_audio::signifier_audio_init_system @ 0x1409d6320
1409d67e1: mov dword ptr [rbp - 0x27], r9d
1409d67e5: movzx r9d, word ptr [rbp + 0xdd]
1409d67ed: mov word ptr [rbp - 0x23], r9w
1409d67f2: movzx r9d, byte ptr [rbp + 0xdf]
1409d67fa: mov byte ptr [rbp - 0x21], r9b
1409d67fe: mov qword ptr [rbp - 0x40], rdx
1409d6802: mov qword ptr [rbp - 0x38], r8
1409d6806: mov qword ptr [rbp - 0x50], rax
1409d680a: mov qword ptr [rbp - 0x48], rcx
1409d680e: mov qword ptr [rbp + 0xc0], 3
1409d6819: mov dword ptr [rbp + 0xc8], 0x32
1409d6823: lea rax, [rip + 0x20ebd36]
1409d682a: mov qword ptr [rbp + 0xb0], rax
1409d6831: mov qword ptr [rbp + 0xb8], 0x21
1409d683c: lea rcx, [rbp + 0x78]
1409d6840: lea r8, [rbp + 0xb0]
1409d6847: mov rdx, rdi
1409d684a: call 0x140a59220
1409d684f: xor eax, eax
1409d6851: cmp rax, qword ptr [rbp + 0x78]
1409d6855: jo 0x1409d6941
1409d685b: mov rax, qword ptr [rbp + 0xa8]
1409d6862: mov qword ptr [rbp + 0x1b0], rax
1409d6869: movups xmm0, xmmword ptr [rbp + 0x78]
1409d686d: movups xmm1, xmmword ptr [rbp + 0x88]
1409d6874: movups xmm2, xmmword ptr [rbp + 0x98]
1409d687b: movaps xmmword ptr [rbp + 0x1a0], xmm2
1409d6882: movaps xmmword ptr [rbp + 0x190], xmm1
1409d6889: movaps xmmword ptr [rbp + 0x180], xmm0

country_core::systems::audio::timberframe_audio::timberframe_audio_init_system @ 0x140a67810
140a67810: push rbp
140a67811: push rsi
140a67812: push rdi
140a67813: sub rsp, 0x1a0
140a6781a: lea rbp, [rsp + 0x80]
140a67822: mov qword ptr [rbp + 0x118], 0xfffffffffffffffe
140a6782d: mov rsi, rdx
140a67830: mov rdi, qword ptr [rcx]
140a67833: mov qword ptr [rbp + 0xf0], 3
140a6783e: mov dword ptr [rbp + 0xf8], 0x32
140a67848: lea rax, [rip + 0x206d65b]
140a6784f: mov qword ptr [rbp + 0xe0], rax
140a67856: mov qword ptr [rbp + 0xe8], 0x1a
140a67861: lea rcx, [rbp + 0x30]
140a67865: lea r8, [rbp + 0xe0]
140a6786c: mov rdx, rdi
140a6786f: call 0x140a59220
140a67874: xor eax, eax
140a67876: cmp rax, qword ptr [rbp + 0x30]
140a6787a: jo 0x140a679ee
140a67880: mov rax, qword ptr [rbp + 0x60]
140a67884: mov qword ptr [rbp + 0x20], rax
140a67888: movups xmm0, xmmword ptr [rbp + 0x30]
140a6788c: movups xmm1, xmmword ptr [rbp + 0x40]
140a67890: movups xmm2, xmmword ptr [rbp + 0x50]
140a67894: movaps xmmword ptr [rbp + 0x10], xmm2
140a67898: movaps xmmword ptr [rbp], xmm1
140a6789c: movaps xmmword ptr [rbp - 0x10], xmm0
140a6787a: jo 0x140a679ee
140a67880: mov rax, qword ptr [rbp + 0x60]
140a67884: mov qword ptr [rbp + 0x20], rax
140a67888: movups xmm0, xmmword ptr [rbp + 0x30]
140a6788c: movups xmm1, xmmword ptr [rbp + 0x40]
140a67890: movups xmm2, xmmword ptr [rbp + 0x50]
140a67894: movaps xmmword ptr [rbp + 0x10], xmm2
140a67898: movaps xmmword ptr [rbp], xmm1
140a6789c: movaps xmmword ptr [rbp - 0x10], xmm0
140a678a0: mov qword ptr [rbp + 0xf0], 3
140a678ab: mov dword ptr [rbp + 0xf8], 0x32
140a678b5: lea rax, [rip + 0x206d65c]
140a678bc: mov qword ptr [rbp + 0xe0], rax
140a678c3: mov qword ptr [rbp + 0xe8], 0x1c
140a678ce: lea rcx, [rbp + 0x30]
140a678d2: lea r8, [rbp + 0xe0]
140a678d9: mov rdx, rdi
140a678dc: call 0x140a59220
140a678e1: xor eax, eax
140a678e3: cmp rax, qword ptr [rbp + 0x30]
140a678e7: jo 0x140a67a26
140a678ed: mov rax, qword ptr [rbp + 0x60]
140a678f1: mov qword ptr [rbp - 0x20], rax
140a678f5: movups xmm0, xmmword ptr [rbp + 0x30]
140a678f9: movups xmm1, xmmword ptr [rbp + 0x40]
140a678fd: movups xmm2, xmmword ptr [rbp + 0x50]
140a67901: movaps xmmword ptr [rbp - 0x30], xmm2
140a67905: movaps xmmword ptr [rbp - 0x40], xmm1
140a67909: movaps xmmword ptr [rbp - 0x50], xmm0
140a678e7: jo 0x140a67a26
140a678ed: mov rax, qword ptr [rbp + 0x60]
140a678f1: mov qword ptr [rbp - 0x20], rax
140a678f5: movups xmm0, xmmword ptr [rbp + 0x30]
140a678f9: movups xmm1, xmmword ptr [rbp + 0x40]
140a678fd: movups xmm2, xmmword ptr [rbp + 0x50]
140a67901: movaps xmmword ptr [rbp - 0x30], xmm2
140a67905: movaps xmmword ptr [rbp - 0x40], xmm1
140a67909: movaps xmmword ptr [rbp - 0x50], xmm0
140a6790d: mov qword ptr [rbp + 0x40], 3
140a67915: mov dword ptr [rbp + 0x48], 0x32
140a6791c: lea rax, [rip + 0x206d62d]
140a67923: mov qword ptr [rbp + 0x30], rax
140a67927: mov qword ptr [rbp + 0x38], 0x1b
140a6792f: lea rcx, [rbp + 0xe0]
140a67936: lea r8, [rbp + 0x30]
140a6793a: mov rdx, rdi
140a6793d: call 0x140a59220
140a67942: xor eax, eax
140a67944: cmp rax, qword ptr [rbp + 0xe0]
140a6794b: jo 0x140a67a5e
140a67951: mov rax, qword ptr [rbp + 0x110]
140a67958: mov qword ptr [rbp + 0xd0], rax
140a6795f: movups xmm0, xmmword ptr [rbp + 0xe0]
140a67966: movups xmm1, xmmword ptr [rbp + 0xf0]
140a6796d: movups xmm2, xmmword ptr [rbp + 0x100]
140a67974: movaps xmmword ptr [rbp + 0xc0], xmm2
140a6797b: movaps xmmword ptr [rbp + 0xb0], xmm1
140a67982: movaps xmmword ptr [rbp + 0xa0], xmm0

country_core::systems::audio::garden_audio::garden_audio_init_system @ 0x140a88fa0
140a88fa0: push rbp
140a88fa1: push rsi
140a88fa2: push rdi
140a88fa3: sub rsp, 0x210
140a88faa: lea rbp, [rsp + 0x80]
140a88fb2: mov qword ptr [rbp + 0x188], 0xfffffffffffffffe
140a88fbd: mov rsi, rdx
140a88fc0: mov rdi, qword ptr [rcx]
140a88fc3: mov qword ptr [rbp + 0x160], 3
140a88fce: mov dword ptr [rbp + 0x168], 0x32
140a88fd8: lea rax, [rip + 0x204ff03]
140a88fdf: mov qword ptr [rbp + 0x150], rax
140a88fe6: mov qword ptr [rbp + 0x158], 0x1c
140a88ff1: lea rcx, [rbp + 0x70]
140a88ff5: lea r8, [rbp + 0x150]
140a88ffc: mov rdx, rdi
140a88fff: call 0x140a59220
140a89004: xor eax, eax
140a89006: cmp rax, qword ptr [rbp + 0x70]
140a8900a: jo 0x140a89247
140a89010: mov rax, qword ptr [rbp + 0xa0]
140a89017: mov qword ptr [rbp + 0x60], rax
140a8901b: movups xmm0, xmmword ptr [rbp + 0x70]
140a8901f: movups xmm1, xmmword ptr [rbp + 0x80]
140a89026: movups xmm2, xmmword ptr [rbp + 0x90]
140a8902d: movaps xmmword ptr [rbp + 0x50], xmm2
140a89031: movaps xmmword ptr [rbp + 0x40], xmm1
140a89035: movaps xmmword ptr [rbp + 0x30], xmm0
140a8900a: jo 0x140a89247
140a89010: mov rax, qword ptr [rbp + 0xa0]
140a89017: mov qword ptr [rbp + 0x60], rax
140a8901b: movups xmm0, xmmword ptr [rbp + 0x70]
140a8901f: movups xmm1, xmmword ptr [rbp + 0x80]
140a89026: movups xmm2, xmmword ptr [rbp + 0x90]
140a8902d: movaps xmmword ptr [rbp + 0x50], xmm2
140a89031: movaps xmmword ptr [rbp + 0x40], xmm1
140a89035: movaps xmmword ptr [rbp + 0x30], xmm0
140a89039: mov qword ptr [rbp + 0x160], 3
140a89044: mov dword ptr [rbp + 0x168], 0x32
140a8904e: lea rax, [rip + 0x204fefb]
140a89055: mov qword ptr [rbp + 0x150], rax
140a8905c: mov qword ptr [rbp + 0x158], 0x21
140a89067: lea rcx, [rbp + 0x70]
140a8906b: lea r8, [rbp + 0x150]
140a89072: mov rdx, rdi
140a89075: call 0x140a59220
140a8907a: xor eax, eax
140a8907c: cmp rax, qword ptr [rbp + 0x70]
140a89080: jo 0x140a89282
140a89086: mov rax, qword ptr [rbp + 0xa0]
140a8908d: mov qword ptr [rbp + 0x20], rax
140a89091: movups xmm0, xmmword ptr [rbp + 0x70]
140a89095: movups xmm1, xmmword ptr [rbp + 0x80]
140a8909c: movups xmm2, xmmword ptr [rbp + 0x90]
140a890a3: movaps xmmword ptr [rbp + 0x10], xmm2
140a890a7: movaps xmmword ptr [rbp], xmm1
140a890ab: movaps xmmword ptr [rbp - 0x10], xmm0
140a89080: jo 0x140a89282
140a89086: mov rax, qword ptr [rbp + 0xa0]
140a8908d: mov qword ptr [rbp + 0x20], rax
140a89091: movups xmm0, xmmword ptr [rbp + 0x70]
140a89095: movups xmm1, xmmword ptr [rbp + 0x80]
140a8909c: movups xmm2, xmmword ptr [rbp + 0x90]
140a890a3: movaps xmmword ptr [rbp + 0x10], xmm2
140a890a7: movaps xmmword ptr [rbp], xmm1
140a890ab: movaps xmmword ptr [rbp - 0x10], xmm0
140a890af: mov qword ptr [rbp + 0x160], 3
140a890ba: mov dword ptr [rbp + 0x168], 0x32
140a890c4: lea rax, [rip + 0x204fec5]
140a890cb: mov qword ptr [rbp + 0x150], rax
140a890d2: mov qword ptr [rbp + 0x158], 0x1d
140a890dd: lea rcx, [rbp + 0x70]
140a890e1: lea r8, [rbp + 0x150]
140a890e8: mov rdx, rdi
140a890eb: call 0x140a59220
140a890f0: xor eax, eax
140a890f2: cmp rax, qword ptr [rbp + 0x70]
140a890f6: jo 0x140a892ba
140a890fc: mov rax, qword ptr [rbp + 0xa0]
140a89103: mov qword ptr [rbp - 0x20], rax
140a89107: movups xmm0, xmmword ptr [rbp + 0x70]
140a8910b: movups xmm1, xmmword ptr [rbp + 0x80]
140a89112: movups xmm2, xmmword ptr [rbp + 0x90]
140a89119: movaps xmmword ptr [rbp - 0x30], xmm2
140a8911d: movaps xmmword ptr [rbp - 0x40], xmm1
140a89121: movaps xmmword ptr [rbp - 0x50], xmm0
140a890f6: jo 0x140a892ba
140a890fc: mov rax, qword ptr [rbp + 0xa0]
140a89103: mov qword ptr [rbp - 0x20], rax
140a89107: movups xmm0, xmmword ptr [rbp + 0x70]
140a8910b: movups xmm1, xmmword ptr [rbp + 0x80]
140a89112: movups xmm2, xmmword ptr [rbp + 0x90]
140a89119: movaps xmmword ptr [rbp - 0x30], xmm2
140a8911d: movaps xmmword ptr [rbp - 0x40], xmm1
140a89121: movaps xmmword ptr [rbp - 0x50], xmm0
140a89125: mov qword ptr [rbp + 0x80], 3
140a89130: mov dword ptr [rbp + 0x88], 0x32
140a8913a: lea rax, [rip + 0x204fe87]
140a89141: mov qword ptr [rbp + 0x70], rax
140a89145: mov qword ptr [rbp + 0x78], 0x22
140a8914d: lea rcx, [rbp + 0x150]
140a89154: lea r8, [rbp + 0x70]
140a89158: mov rdx, rdi
140a8915b: call 0x140a59220
140a89160: xor eax, eax
140a89162: cmp rax, qword ptr [rbp + 0x150]
140a89169: jo 0x140a892f2
140a8916f: mov rax, qword ptr [rbp + 0x180]
140a89176: mov qword ptr [rbp + 0x148], rax
140a8917d: movups xmm0, xmmword ptr [rbp + 0x150]
140a89184: movups xmm1, xmmword ptr [rbp + 0x160]
140a8918b: movups xmm2, xmmword ptr [rbp + 0x170]
140a89192: movups xmmword ptr [rbp + 0x138], xmm2
140a89199: movups xmmword ptr [rbp + 0x128], xmm1
140a891a0: movups xmmword ptr [rbp + 0x118], xmm0

country_core::systems::audio::forest_audio::forest_audio_init_system @ 0x140aa18f0
140aa19a5: mov qword ptr [rax + rcx + 0x30], rdx
140aa19aa: movups xmmword ptr [rax + rcx + 0x20], xmm2
140aa19af: movups xmmword ptr [rax + rcx + 0x10], xmm1
140aa19b4: movups xmmword ptr [rax + rcx], xmm0
140aa19b8: inc r15
140aa19bb: mov qword ptr [rbp + 0x98], r15
140aa19c2: mov r15, r8
140aa19c5: cmp r15, 0x32
140aa19c9: mov qword ptr [rbp + 0x48], r15
140aa19cd: jae 0x140aa1a4d
140aa19cf: mov qword ptr [rbp + 0x60], 3
140aa19d7: mov dword ptr [rbp + 0x68], 0x32
140aa19de: mov qword ptr [rbp + 0x50], rsi
140aa19e2: mov qword ptr [rbp + 0x58], 0x15
140aa19ea: mov rcx, rbx
140aa19ed: mov rdx, rdi
140aa19f0: mov r8, r14
140aa19f3: call 0x140a59220
140aa19f8: cmp r13, qword ptr [rbp - 0x50]
140aa19fc: jo 0x140aa1d42
140aa1a02: mov rax, qword ptr [rbp - 0x20]
140aa1a06: mov qword ptr [rbp + 0x80], rax
140aa1a0d: movups xmm0, xmmword ptr [rbp - 0x50]
140aa1a11: movups xmm1, xmmword ptr [rbp - 0x40]
140aa1a15: movups xmm2, xmmword ptr [rbp - 0x30]
140aa1a19: movaps xmmword ptr [rbp + 0x70], xmm2
140aa1a1d: movaps xmmword ptr [rbp + 0x60], xmm1
140aa1a21: movaps xmmword ptr [rbp + 0x50], xmm0
140aa1a25: mov r15, qword ptr [rbp + 0x98]

country_core::systems::audio::roof_audio::roof_audio_init_system @ 0x140aa2fd0
140aa2fd0: push rbp
140aa2fd1: push rsi
140aa2fd2: push rdi
140aa2fd3: sub rsp, 0x1a0
140aa2fda: lea rbp, [rsp + 0x80]
140aa2fe2: mov qword ptr [rbp + 0x118], 0xfffffffffffffffe
140aa2fed: mov rsi, rdx
140aa2ff0: mov rdi, qword ptr [rcx]
140aa2ff3: mov qword ptr [rbp + 0xf0], 3
140aa2ffe: mov dword ptr [rbp + 0xf8], 0x32
140aa3008: lea rax, [rip + 0x2039ac9]
140aa300f: mov qword ptr [rbp + 0xe0], rax
140aa3016: mov qword ptr [rbp + 0xe8], 0x14
140aa3021: lea rcx, [rbp + 0x30]
140aa3025: lea r8, [rbp + 0xe0]
140aa302c: mov rdx, rdi
140aa302f: call 0x140a59220
140aa3034: xor eax, eax
140aa3036: cmp rax, qword ptr [rbp + 0x30]
140aa303a: jo 0x140aa31be
140aa3040: mov rax, qword ptr [rbp + 0x60]
140aa3044: mov qword ptr [rbp + 0x20], rax
140aa3048: movups xmm0, xmmword ptr [rbp + 0x30]
140aa304c: movups xmm1, xmmword ptr [rbp + 0x40]
140aa3050: movups xmm2, xmmword ptr [rbp + 0x50]
140aa3054: movaps xmmword ptr [rbp + 0x10], xmm2
140aa3058: movaps xmmword ptr [rbp], xmm1
140aa305c: movaps xmmword ptr [rbp - 0x10], xmm0
140aa303a: jo 0x140aa31be
140aa3040: mov rax, qword ptr [rbp + 0x60]
140aa3044: mov qword ptr [rbp + 0x20], rax
140aa3048: movups xmm0, xmmword ptr [rbp + 0x30]
140aa304c: movups xmm1, xmmword ptr [rbp + 0x40]
140aa3050: movups xmm2, xmmword ptr [rbp + 0x50]
140aa3054: movaps xmmword ptr [rbp + 0x10], xmm2
140aa3058: movaps xmmword ptr [rbp], xmm1
140aa305c: movaps xmmword ptr [rbp - 0x10], xmm0
140aa3060: mov qword ptr [rbp + 0xf0], 3
140aa306b: mov dword ptr [rbp + 0xf8], 0x32
140aa3075: lea rax, [rip + 0x2039abc]
140aa307c: mov qword ptr [rbp + 0xe0], rax
140aa3083: mov qword ptr [rbp + 0xe8], 0x16
140aa308e: lea rcx, [rbp + 0x30]
140aa3092: lea r8, [rbp + 0xe0]
140aa3099: mov rdx, rdi
140aa309c: call 0x140a59220
140aa30a1: xor eax, eax
140aa30a3: cmp rax, qword ptr [rbp + 0x30]
140aa30a7: jo 0x140aa31f6
140aa30ad: mov rax, qword ptr [rbp + 0x60]
140aa30b1: mov qword ptr [rbp - 0x20], rax
140aa30b5: movups xmm0, xmmword ptr [rbp + 0x30]
140aa30b9: movups xmm1, xmmword ptr [rbp + 0x40]
140aa30bd: movups xmm2, xmmword ptr [rbp + 0x50]
140aa30c1: movaps xmmword ptr [rbp - 0x30], xmm2
140aa30c5: movaps xmmword ptr [rbp - 0x40], xmm1
140aa30c9: movaps xmmword ptr [rbp - 0x50], xmm0
140aa30a7: jo 0x140aa31f6
140aa30ad: mov rax, qword ptr [rbp + 0x60]
140aa30b1: mov qword ptr [rbp - 0x20], rax
140aa30b5: movups xmm0, xmmword ptr [rbp + 0x30]
140aa30b9: movups xmm1, xmmword ptr [rbp + 0x40]
140aa30bd: movups xmm2, xmmword ptr [rbp + 0x50]
140aa30c1: movaps xmmword ptr [rbp - 0x30], xmm2
140aa30c5: movaps xmmword ptr [rbp - 0x40], xmm1
140aa30c9: movaps xmmword ptr [rbp - 0x50], xmm0
140aa30cd: mov qword ptr [rbp + 0x40], 3
140aa30d5: mov dword ptr [rbp + 0x48], 0x32
140aa30dc: lea rax, [rip + 0x2039a85]
140aa30e3: mov qword ptr [rbp + 0x30], rax
140aa30e7: mov qword ptr [rbp + 0x38], 0x24
140aa30ef: lea rcx, [rbp + 0xe0]
140aa30f6: lea r8, [rbp + 0x30]
140aa30fa: mov rdx, rdi
140aa30fd: call 0x140a59220
140aa3102: xor eax, eax
140aa3104: cmp rax, qword ptr [rbp + 0xe0]
140aa310b: jo 0x140aa322e
140aa3111: mov rax, qword ptr [rbp + 0x110]
140aa3118: mov qword ptr [rbp + 0xd0], rax
140aa311f: movups xmm0, xmmword ptr [rbp + 0xe0]
140aa3126: movups xmm1, xmmword ptr [rbp + 0xf0]
140aa312d: movups xmm2, xmmword ptr [rbp + 0x100]
140aa3134: movaps xmmword ptr [rbp + 0xc0], xmm2
140aa313b: movaps xmmword ptr [rbp + 0xb0], xmm1
140aa3142: movaps xmmword ptr [rbp + 0xa0], xmm0

country_core::systems::audio::playermade_stairs_audio::stairs_audio_init_system @ 0x140f878d0
140f878d0: push rbp
140f878d1: push rsi
140f878d2: push rdi
140f878d3: sub rsp, 0x300
140f878da: lea rbp, [rsp + 0x80]
140f878e2: mov qword ptr [rbp + 0x278], 0xfffffffffffffffe
140f878ed: mov rsi, rdx
140f878f0: mov rdi, qword ptr [rcx]
140f878f3: mov qword ptr [rbp + 0x250], 3
140f878fe: mov dword ptr [rbp + 0x258], 0x32
140f87908: lea rax, [rip + 0x1c25bf1]
140f8790f: mov qword ptr [rbp + 0x240], rax
140f87916: mov qword ptr [rbp + 0x248], 0x1b
140f87921: lea rcx, [rbp + 0xf0]
140f87928: lea r8, [rbp + 0x240]
140f8792f: mov rdx, rdi
140f87932: call 0x140a59220
140f87937: xor eax, eax
140f87939: cmp rax, qword ptr [rbp + 0xf0]
140f87940: jo 0x140f87d21
140f87946: mov rax, qword ptr [rbp + 0x120]
140f8794d: mov qword ptr [rbp + 0xe0], rax
140f87954: movups xmm0, xmmword ptr [rbp + 0xf0]
140f8795b: movups xmm1, xmmword ptr [rbp + 0x100]
140f87962: movups xmm2, xmmword ptr [rbp + 0x110]
140f87969: movaps xmmword ptr [rbp + 0xd0], xmm2
140f87970: movaps xmmword ptr [rbp + 0xc0], xmm1
140f87977: movaps xmmword ptr [rbp + 0xb0], xmm0
140f87940: jo 0x140f87d21
140f87946: mov rax, qword ptr [rbp + 0x120]
140f8794d: mov qword ptr [rbp + 0xe0], rax
140f87954: movups xmm0, xmmword ptr [rbp + 0xf0]
140f8795b: movups xmm1, xmmword ptr [rbp + 0x100]
140f87962: movups xmm2, xmmword ptr [rbp + 0x110]
140f87969: movaps xmmword ptr [rbp + 0xd0], xmm2
140f87970: movaps xmmword ptr [rbp + 0xc0], xmm1
140f87977: movaps xmmword ptr [rbp + 0xb0], xmm0
140f8797e: mov qword ptr [rbp + 0x250], 3
140f87989: mov dword ptr [rbp + 0x258], 0x32
140f87993: lea rax, [rip + 0x1c25bde]
140f8799a: mov qword ptr [rbp + 0x240], rax
140f879a1: mov qword ptr [rbp + 0x248], 0x23
140f879ac: lea rcx, [rbp + 0xf0]
140f879b3: lea r8, [rbp + 0x240]
140f879ba: mov rdx, rdi
140f879bd: call 0x140a59220
140f879c2: xor eax, eax
140f879c4: cmp rax, qword ptr [rbp + 0xf0]
140f879cb: jo 0x140f87d5f
140f879d1: mov rax, qword ptr [rbp + 0x120]
140f879d8: mov qword ptr [rbp + 0xa0], rax
140f879df: movups xmm0, xmmword ptr [rbp + 0xf0]
140f879e6: movups xmm1, xmmword ptr [rbp + 0x100]
140f879ed: movups xmm2, xmmword ptr [rbp + 0x110]
140f879f4: movaps xmmword ptr [rbp + 0x90], xmm2
140f879fb: movaps xmmword ptr [rbp + 0x80], xmm1
140f87a02: movaps xmmword ptr [rbp + 0x70], xmm0
140f879cb: jo 0x140f87d5f
140f879d1: mov rax, qword ptr [rbp + 0x120]
140f879d8: mov qword ptr [rbp + 0xa0], rax
140f879df: movups xmm0, xmmword ptr [rbp + 0xf0]
140f879e6: movups xmm1, xmmword ptr [rbp + 0x100]
140f879ed: movups xmm2, xmmword ptr [rbp + 0x110]
140f879f4: movaps xmmword ptr [rbp + 0x90], xmm2
140f879fb: movaps xmmword ptr [rbp + 0x80], xmm1
140f87a02: movaps xmmword ptr [rbp + 0x70], xmm0
140f87a06: mov qword ptr [rbp + 0x250], 3
140f87a11: mov dword ptr [rbp + 0x258], 0x32
140f87a1b: lea rax, [rip + 0x1c25b96]
140f87a22: mov qword ptr [rbp + 0x240], rax
140f87a29: mov qword ptr [rbp + 0x248], 0x22
140f87a34: lea rcx, [rbp + 0xf0]
140f87a3b: lea r8, [rbp + 0x240]
140f87a42: mov rdx, rdi
140f87a45: call 0x140a59220
140f87a4a: xor eax, eax
140f87a4c: cmp rax, qword ptr [rbp + 0xf0]
140f87a53: jo 0x140f87d9d
140f87a59: mov rax, qword ptr [rbp + 0x120]
140f87a60: mov qword ptr [rbp + 0x60], rax
140f87a64: movups xmm0, xmmword ptr [rbp + 0xf0]
140f87a6b: movups xmm1, xmmword ptr [rbp + 0x100]
140f87a72: movups xmm2, xmmword ptr [rbp + 0x110]
140f87a79: movaps xmmword ptr [rbp + 0x50], xmm2
140f87a7d: movaps xmmword ptr [rbp + 0x40], xmm1
140f87a81: movaps xmmword ptr [rbp + 0x30], xmm0
140f87a53: jo 0x140f87d9d
140f87a59: mov rax, qword ptr [rbp + 0x120]
140f87a60: mov qword ptr [rbp + 0x60], rax
140f87a64: movups xmm0, xmmword ptr [rbp + 0xf0]
140f87a6b: movups xmm1, xmmword ptr [rbp + 0x100]
140f87a72: movups xmm2, xmmword ptr [rbp + 0x110]
140f87a79: movaps xmmword ptr [rbp + 0x50], xmm2
140f87a7d: movaps xmmword ptr [rbp + 0x40], xmm1
140f87a81: movaps xmmword ptr [rbp + 0x30], xmm0
140f87a85: mov qword ptr [rbp + 0x250], 3
140f87a90: mov dword ptr [rbp + 0x258], 0x32
140f87a9a: lea rax, [rip + 0x1c25b57]
140f87aa1: mov qword ptr [rbp + 0x240], rax
140f87aa8: mov qword ptr [rbp + 0x248], 0x1c
140f87ab3: lea rcx, [rbp + 0xf0]
140f87aba: lea r8, [rbp + 0x240]
140f87ac1: mov rdx, rdi
140f87ac4: call 0x140a59220
140f87ac9: xor eax, eax
140f87acb: cmp rax, qword ptr [rbp + 0xf0]
140f87ad2: jo 0x140f87ddb
140f87ad8: mov rax, qword ptr [rbp + 0x120]
140f87adf: mov qword ptr [rbp + 0x20], rax
140f87ae3: movups xmm0, xmmword ptr [rbp + 0xf0]
140f87aea: movups xmm1, xmmword ptr [rbp + 0x100]
140f87af1: movups xmm2, xmmword ptr [rbp + 0x110]
140f87af8: movaps xmmword ptr [rbp + 0x10], xmm2
140f87afc: movaps xmmword ptr [rbp], xmm1
140f87b00: movaps xmmword ptr [rbp - 0x10], xmm0
140f87ad2: jo 0x140f87ddb
140f87ad8: mov rax, qword ptr [rbp + 0x120]
140f87adf: mov qword ptr [rbp + 0x20], rax
140f87ae3: movups xmm0, xmmword ptr [rbp + 0xf0]
140f87aea: movups xmm1, xmmword ptr [rbp + 0x100]
140f87af1: movups xmm2, xmmword ptr [rbp + 0x110]
140f87af8: movaps xmmword ptr [rbp + 0x10], xmm2
140f87afc: movaps xmmword ptr [rbp], xmm1
140f87b00: movaps xmmword ptr [rbp - 0x10], xmm0
140f87b04: mov qword ptr [rbp + 0x250], 3
140f87b0f: mov dword ptr [rbp + 0x258], 0x32
140f87b19: lea rax, [rip + 0x1c25b10]
140f87b20: mov qword ptr [rbp + 0x240], rax
140f87b27: mov qword ptr [rbp + 0x248], 0x24
140f87b32: lea rcx, [rbp + 0xf0]
140f87b39: lea r8, [rbp + 0x240]
140f87b40: mov rdx, rdi
140f87b43: call 0x140a59220
140f87b48: xor eax, eax
140f87b4a: cmp rax, qword ptr [rbp + 0xf0]
140f87b51: jo 0x140f87e16
140f87b57: mov rax, qword ptr [rbp + 0x120]
140f87b5e: mov qword ptr [rbp - 0x20], rax
140f87b62: movups xmm0, xmmword ptr [rbp + 0xf0]
140f87b69: movups xmm1, xmmword ptr [rbp + 0x100]
140f87b70: movups xmm2, xmmword ptr [rbp + 0x110]
140f87b77: movaps xmmword ptr [rbp - 0x30], xmm2
140f87b7b: movaps xmmword ptr [rbp - 0x40], xmm1
140f87b7f: movaps xmmword ptr [rbp - 0x50], xmm0
140f87b51: jo 0x140f87e16
140f87b57: mov rax, qword ptr [rbp + 0x120]
140f87b5e: mov qword ptr [rbp - 0x20], rax
140f87b62: movups xmm0, xmmword ptr [rbp + 0xf0]
140f87b69: movups xmm1, xmmword ptr [rbp + 0x100]
140f87b70: movups xmm2, xmmword ptr [rbp + 0x110]
140f87b77: movaps xmmword ptr [rbp - 0x30], xmm2
140f87b7b: movaps xmmword ptr [rbp - 0x40], xmm1
140f87b7f: movaps xmmword ptr [rbp - 0x50], xmm0
140f87b83: mov qword ptr [rbp + 0x100], 3
140f87b8e: mov dword ptr [rbp + 0x108], 0x32
140f87b98: lea rax, [rip + 0x1c25ad1]
140f87b9f: mov qword ptr [rbp + 0xf0], rax
140f87ba6: mov qword ptr [rbp + 0xf8], 0x23
140f87bb1: lea rcx, [rbp + 0x240]
140f87bb8: lea r8, [rbp + 0xf0]
140f87bbf: mov rdx, rdi
140f87bc2: call 0x140a59220
140f87bc7: xor eax, eax
140f87bc9: cmp rax, qword ptr [rbp + 0x240]
140f87bd0: jo 0x140f87e51
140f87bd6: mov rax, qword ptr [rbp + 0x270]
140f87bdd: mov qword ptr [rbp + 0x238], rax
140f87be4: movups xmm0, xmmword ptr [rbp + 0x240]
140f87beb: movups xmm1, xmmword ptr [rbp + 0x250]
140f87bf2: movups xmm2, xmmword ptr [rbp + 0x260]
140f87bf9: movups xmmword ptr [rbp + 0x228], xmm2
140f87c00: movups xmmword ptr [rbp + 0x218], xmm1
140f87c07: movups xmmword ptr [rbp + 0x208], xmm0

country_core::systems::audio::decorator_audio::decorator_audio_init_system @ 0x14145c6b0
14145c6b0: push rbp
14145c6b1: push rsi
14145c6b2: push rdi
14145c6b3: push rbx
14145c6b4: sub rsp, 0x7a8
14145c6bb: lea rbp, [rsp + 0x80]
14145c6c3: mov qword ptr [rbp + 0x720], 0xfffffffffffffffe
14145c6ce: mov rsi, rdx
14145c6d1: mov rdi, qword ptr [rcx]
14145c6d4: mov qword ptr [rbp + 0x640], 3
14145c6df: mov dword ptr [rbp + 0x648], 0x32
14145c6e9: lea rax, [rip + 0x1805d98]
14145c6f0: mov qword ptr [rbp + 0x630], rax
14145c6f7: mov qword ptr [rbp + 0x638], 0x15
14145c702: lea rcx, [rbp + 0x270]
14145c709: lea r8, [rbp + 0x630]
14145c710: mov rdx, rdi
14145c713: call 0x140a59220
14145c718: xor eax, eax
14145c71a: cmp rax, qword ptr [rbp + 0x270]
14145c721: jo 0x14145d274
14145c727: mov rax, qword ptr [rbp + 0x2a0]
14145c72e: mov qword ptr [rbp + 0x260], rax
14145c735: movups xmm0, xmmword ptr [rbp + 0x270]
14145c73c: movups xmm1, xmmword ptr [rbp + 0x280]
14145c743: movups xmm2, xmmword ptr [rbp + 0x290]
14145c74a: movaps xmmword ptr [rbp + 0x250], xmm2
14145c751: movaps xmmword ptr [rbp + 0x240], xmm1
14145c758: movaps xmmword ptr [rbp + 0x230], xmm0
14145c721: jo 0x14145d274
14145c727: mov rax, qword ptr [rbp + 0x2a0]
14145c72e: mov qword ptr [rbp + 0x260], rax
14145c735: movups xmm0, xmmword ptr [rbp + 0x270]
14145c73c: movups xmm1, xmmword ptr [rbp + 0x280]
14145c743: movups xmm2, xmmword ptr [rbp + 0x290]
14145c74a: movaps xmmword ptr [rbp + 0x250], xmm2
14145c751: movaps xmmword ptr [rbp + 0x240], xmm1
14145c758: movaps xmmword ptr [rbp + 0x230], xmm0
14145c75f: mov qword ptr [rbp + 0x640], 3
14145c76a: mov dword ptr [rbp + 0x648], 0x32
14145c774: lea rax, [rip + 0x1805d75]
14145c77b: mov qword ptr [rbp + 0x630], rax
14145c782: mov qword ptr [rbp + 0x638], 0x27
14145c78d: lea rcx, [rbp + 0x270]
14145c794: lea r8, [rbp + 0x630]
14145c79b: mov rdx, rdi
14145c79e: call 0x140a59220
14145c7a3: xor eax, eax
14145c7a5: cmp rax, qword ptr [rbp + 0x270]
14145c7ac: jo 0x14145d2b2
14145c7b2: mov rax, qword ptr [rbp + 0x2a0]
14145c7b9: mov qword ptr [rbp + 0x220], rax
14145c7c0: movups xmm0, xmmword ptr [rbp + 0x270]
14145c7c7: movups xmm1, xmmword ptr [rbp + 0x280]
14145c7ce: movups xmm2, xmmword ptr [rbp + 0x290]
14145c7d5: movaps xmmword ptr [rbp + 0x210], xmm2
14145c7dc: movaps xmmword ptr [rbp + 0x200], xmm1
14145c7e3: movaps xmmword ptr [rbp + 0x1f0], xmm0
14145c7ac: jo 0x14145d2b2
14145c7b2: mov rax, qword ptr [rbp + 0x2a0]
14145c7b9: mov qword ptr [rbp + 0x220], rax
14145c7c0: movups xmm0, xmmword ptr [rbp + 0x270]
14145c7c7: movups xmm1, xmmword ptr [rbp + 0x280]
14145c7ce: movups xmm2, xmmword ptr [rbp + 0x290]
14145c7d5: movaps xmmword ptr [rbp + 0x210], xmm2
14145c7dc: movaps xmmword ptr [rbp + 0x200], xmm1
14145c7e3: movaps xmmword ptr [rbp + 0x1f0], xmm0
14145c7ea: mov qword ptr [rbp + 0x640], 3
14145c7f5: mov dword ptr [rbp + 0x648], 0x32
14145c7ff: lea rax, [rip + 0x1805d2a]
14145c806: mov qword ptr [rbp + 0x630], rax
14145c80d: mov qword ptr [rbp + 0x638], 0x1b
14145c818: lea rcx, [rbp + 0x270]
14145c81f: lea r8, [rbp + 0x630]
14145c826: mov rdx, rdi
14145c829: call 0x140a59220
14145c82e: xor eax, eax
14145c830: cmp rax, qword ptr [rbp + 0x270]
14145c837: jo 0x14145d2f0
14145c83d: mov rax, qword ptr [rbp + 0x2a0]
14145c844: mov qword ptr [rbp + 0x1e0], rax
14145c84b: movups xmm0, xmmword ptr [rbp + 0x270]
14145c852: movups xmm1, xmmword ptr [rbp + 0x280]
14145c859: movups xmm2, xmmword ptr [rbp + 0x290]
14145c860: movaps xmmword ptr [rbp + 0x1d0], xmm2
14145c867: movaps xmmword ptr [rbp + 0x1c0], xmm1
14145c86e: movaps xmmword ptr [rbp + 0x1b0], xmm0
14145c837: jo 0x14145d2f0
14145c83d: mov rax, qword ptr [rbp + 0x2a0]
14145c844: mov qword ptr [rbp + 0x1e0], rax
14145c84b: movups xmm0, xmmword ptr [rbp + 0x270]
14145c852: movups xmm1, xmmword ptr [rbp + 0x280]
14145c859: movups xmm2, xmmword ptr [rbp + 0x290]
14145c860: movaps xmmword ptr [rbp + 0x1d0], xmm2
14145c867: movaps xmmword ptr [rbp + 0x1c0], xmm1
14145c86e: movaps xmmword ptr [rbp + 0x1b0], xmm0
14145c875: mov qword ptr [rbp + 0x640], 3
14145c880: mov dword ptr [rbp + 0x648], 0x32
14145c88a: lea rax, [rip + 0x1805cd7]
14145c891: mov qword ptr [rbp + 0x630], rax
14145c898: mov qword ptr [rbp + 0x638], 0x1d
14145c8a3: lea rcx, [rbp + 0x270]
14145c8aa: lea r8, [rbp + 0x630]
14145c8b1: mov rdx, rdi
14145c8b4: call 0x140a59220
14145c8b9: xor eax, eax
14145c8bb: cmp rax, qword ptr [rbp + 0x270]
14145c8c2: jo 0x14145d32e
14145c8c8: mov rax, qword ptr [rbp + 0x2a0]
14145c8cf: mov qword ptr [rbp + 0x1a0], rax
14145c8d6: movups xmm0, xmmword ptr [rbp + 0x270]
14145c8dd: movups xmm1, xmmword ptr [rbp + 0x280]
14145c8e4: movups xmm2, xmmword ptr [rbp + 0x290]
14145c8eb: movaps xmmword ptr [rbp + 0x190], xmm2
14145c8f2: movaps xmmword ptr [rbp + 0x180], xmm1
14145c8f9: movaps xmmword ptr [rbp + 0x170], xmm0
14145c8c2: jo 0x14145d32e
14145c8c8: mov rax, qword ptr [rbp + 0x2a0]
14145c8cf: mov qword ptr [rbp + 0x1a0], rax
14145c8d6: movups xmm0, xmmword ptr [rbp + 0x270]
14145c8dd: movups xmm1, xmmword ptr [rbp + 0x280]
14145c8e4: movups xmm2, xmmword ptr [rbp + 0x290]
14145c8eb: movaps xmmword ptr [rbp + 0x190], xmm2
14145c8f2: movaps xmmword ptr [rbp + 0x180], xmm1
14145c8f9: movaps xmmword ptr [rbp + 0x170], xmm0
14145c900: mov qword ptr [rbp + 0x640], 3
14145c90b: mov dword ptr [rbp + 0x648], 0x32
14145c915: lea rax, [rip + 0x1805c84]
14145c91c: mov qword ptr [rbp + 0x630], rax
14145c923: mov qword ptr [rbp + 0x638], 0x19
14145c92e: lea rcx, [rbp + 0x270]
14145c935: lea r8, [rbp + 0x630]
14145c93c: mov rdx, rdi
14145c93f: call 0x140a59220
14145c944: xor eax, eax
14145c946: cmp rax, qword ptr [rbp + 0x270]
14145c94d: jo 0x14145d36c
14145c953: mov rax, qword ptr [rbp + 0x2a0]
14145c95a: mov qword ptr [rbp + 0x160], rax
14145c961: movups xmm0, xmmword ptr [rbp + 0x270]
14145c968: movups xmm1, xmmword ptr [rbp + 0x280]
14145c96f: movups xmm2, xmmword ptr [rbp + 0x290]
14145c976: movaps xmmword ptr [rbp + 0x150], xmm2
14145c97d: movaps xmmword ptr [rbp + 0x140], xmm1
14145c984: movaps xmmword ptr [rbp + 0x130], xmm0
14145c94d: jo 0x14145d36c
14145c953: mov rax, qword ptr [rbp + 0x2a0]
14145c95a: mov qword ptr [rbp + 0x160], rax
14145c961: movups xmm0, xmmword ptr [rbp + 0x270]
14145c968: movups xmm1, xmmword ptr [rbp + 0x280]
14145c96f: movups xmm2, xmmword ptr [rbp + 0x290]
14145c976: movaps xmmword ptr [rbp + 0x150], xmm2
14145c97d: movaps xmmword ptr [rbp + 0x140], xmm1
14145c984: movaps xmmword ptr [rbp + 0x130], xmm0
14145c98b: mov qword ptr [rbp + 0x640], 3
14145c996: mov dword ptr [rbp + 0x648], 0x32
14145c9a0: lea rbx, [rip + 0x1805c31]
14145c9a7: mov qword ptr [rbp + 0x630], rbx
14145c9ae: mov qword ptr [rbp + 0x638], 0x16
14145c9b9: lea rcx, [rbp + 0x270]
14145c9c0: lea r8, [rbp + 0x630]
14145c9c7: mov rdx, rdi
14145c9ca: call 0x140a59220
14145c9cf: xor eax, eax
14145c9d1: cmp rax, qword ptr [rbp + 0x270]
14145c9d8: jo 0x14145d3aa
14145c9de: mov rax, qword ptr [rbp + 0x2a0]
14145c9e5: mov qword ptr [rbp + 0x620], rax
14145c9ec: movups xmm0, xmmword ptr [rbp + 0x270]
14145c9f3: movups xmm1, xmmword ptr [rbp + 0x280]
14145c9fa: movups xmm2, xmmword ptr [rbp + 0x290]
14145ca01: movaps xmmword ptr [rbp + 0x610], xmm2
14145ca08: movaps xmmword ptr [rbp + 0x600], xmm1
14145ca0f: movaps xmmword ptr [rbp + 0x5f0], xmm0
14145c9d1: cmp rax, qword ptr [rbp + 0x270]
14145c9d8: jo 0x14145d3aa
14145c9de: mov rax, qword ptr [rbp + 0x2a0]
14145c9e5: mov qword ptr [rbp + 0x620], rax
14145c9ec: movups xmm0, xmmword ptr [rbp + 0x270]
14145c9f3: movups xmm1, xmmword ptr [rbp + 0x280]
14145c9fa: movups xmm2, xmmword ptr [rbp + 0x290]
14145ca01: movaps xmmword ptr [rbp + 0x610], xmm2
14145ca08: movaps xmmword ptr [rbp + 0x600], xmm1
14145ca0f: movaps xmmword ptr [rbp + 0x5f0], xmm0
14145ca16: mov qword ptr [rbp + 0x640], 3
14145ca21: mov dword ptr [rbp + 0x648], 0x32
14145ca2b: mov qword ptr [rbp + 0x630], rbx
14145ca32: mov qword ptr [rbp + 0x638], 0x16
14145ca3d: lea rcx, [rbp + 0x270]
14145ca44: lea r8, [rbp + 0x630]
14145ca4b: mov rdx, rdi
14145ca4e: call 0x140a59220
14145ca53: xor eax, eax
14145ca55: cmp rax, qword ptr [rbp + 0x270]
14145ca5c: jo 0x14145d3e8
14145ca62: mov rax, qword ptr [rbp + 0x2a0]
14145ca69: mov qword ptr [rbp + 0x710], rax
14145ca70: movups xmm0, xmmword ptr [rbp + 0x270]
14145ca77: movups xmm1, xmmword ptr [rbp + 0x280]
14145ca7e: movups xmm2, xmmword ptr [rbp + 0x290]
14145ca85: movaps xmmword ptr [rbp + 0x700], xmm2
14145ca8c: movaps xmmword ptr [rbp + 0x6f0], xmm1
14145ca93: movaps xmmword ptr [rbp + 0x6e0], xmm0
14145ca55: cmp rax, qword ptr [rbp + 0x270]
14145ca5c: jo 0x14145d3e8
14145ca62: mov rax, qword ptr [rbp + 0x2a0]
14145ca69: mov qword ptr [rbp + 0x710], rax
14145ca70: movups xmm0, xmmword ptr [rbp + 0x270]
14145ca77: movups xmm1, xmmword ptr [rbp + 0x280]
14145ca7e: movups xmm2, xmmword ptr [rbp + 0x290]
14145ca85: movaps xmmword ptr [rbp + 0x700], xmm2
14145ca8c: movaps xmmword ptr [rbp + 0x6f0], xmm1
14145ca93: movaps xmmword ptr [rbp + 0x6e0], xmm0
14145ca9a: mov qword ptr [rbp + 0x640], 3
14145caa5: mov dword ptr [rbp + 0x648], 0x32
14145caaf: mov qword ptr [rbp + 0x630], rbx
14145cab6: mov qword ptr [rbp + 0x638], 0x16
14145cac1: lea rcx, [rbp + 0x270]
14145cac8: lea r8, [rbp + 0x630]
14145cacf: mov rdx, rdi
14145cad2: call 0x140a59220
14145cad7: xor eax, eax
14145cad9: cmp rax, qword ptr [rbp + 0x270]
14145cae0: jo 0x14145d426
14145cae6: mov rax, qword ptr [rbp + 0x2a0]
14145caed: mov qword ptr [rbp + 0x6d0], rax
14145caf4: movups xmm0, xmmword ptr [rbp + 0x270]
14145cafb: movups xmm1, xmmword ptr [rbp + 0x280]
14145cb02: movups xmm2, xmmword ptr [rbp + 0x290]
14145cb09: movaps xmmword ptr [rbp + 0x6c0], xmm2
14145cb10: movaps xmmword ptr [rbp + 0x6b0], xmm1
14145cb17: movaps xmmword ptr [rbp + 0x6a0], xmm0
14145cb4f: mov qword ptr [rbp + 0x660], rax
14145cb56: mov rax, qword ptr [rbp + 0x710]
14145cb5d: mov qword ptr [rbp + 0x698], rax
14145cb64: movaps xmm0, xmmword ptr [rbp + 0x6e0]
14145cb6b: movaps xmm1, xmmword ptr [rbp + 0x6f0]
14145cb72: movaps xmm2, xmmword ptr [rbp + 0x700]
14145cb79: movups xmmword ptr [rbp + 0x688], xmm2
14145cb80: movups xmmword ptr [rbp + 0x678], xmm1
14145cb87: movups xmmword ptr [rbp + 0x668], xmm0
14145cb8e: mov qword ptr [rbp + 0x6f0], 3
14145cb99: mov dword ptr [rbp + 0x6f8], 0x32
14145cba3: lea rax, [rip + 0x1805a8e]
14145cbaa: mov qword ptr [rbp + 0x6e0], rax
14145cbb1: mov qword ptr [rbp + 0x6e8], 0x1e
14145cbbc: lea rcx, [rbp + 0x270]
14145cbc3: lea r8, [rbp + 0x6e0]
14145cbca: mov rdx, rdi
14145cbcd: call 0x140a59220
14145cbd2: xor eax, eax
14145cbd4: cmp rax, qword ptr [rbp + 0x270]
14145cbdb: jo 0x14145d464
14145cbe1: mov rax, qword ptr [rbp + 0x2a0]
14145cbe8: mov qword ptr [rbp + 0x120], rax
14145cbef: movups xmm0, xmmword ptr [rbp + 0x270]
14145cbf6: movups xmm1, xmmword ptr [rbp + 0x280]
14145cbfd: movups xmm2, xmmword ptr [rbp + 0x290]
14145cc04: movaps xmmword ptr [rbp + 0x110], xmm2
14145cc0b: movaps xmmword ptr [rbp + 0x100], xmm1
14145cc12: movaps xmmword ptr [rbp + 0xf0], xmm0
14145cbdb: jo 0x14145d464
14145cbe1: mov rax, qword ptr [rbp + 0x2a0]
14145cbe8: mov qword ptr [rbp + 0x120], rax
14145cbef: movups xmm0, xmmword ptr [rbp + 0x270]
14145cbf6: movups xmm1, xmmword ptr [rbp + 0x280]
14145cbfd: movups xmm2, xmmword ptr [rbp + 0x290]
14145cc04: movaps xmmword ptr [rbp + 0x110], xmm2
14145cc0b: movaps xmmword ptr [rbp + 0x100], xmm1
14145cc12: movaps xmmword ptr [rbp + 0xf0], xmm0
14145cc19: mov qword ptr [rbp + 0x6f0], 3
14145cc24: mov dword ptr [rbp + 0x6f8], 0x32
14145cc2e: lea rax, [rip + 0x1805a3b]
14145cc35: mov qword ptr [rbp + 0x6e0], rax
14145cc3c: mov qword ptr [rbp + 0x6e8], 0x16
14145cc47: lea rcx, [rbp + 0x270]
14145cc4e: lea r8, [rbp + 0x6e0]
14145cc55: mov rdx, rdi
14145cc58: call 0x140a59220
14145cc5d: xor eax, eax
14145cc5f: cmp rax, qword ptr [rbp + 0x270]
14145cc66: jo 0x14145d4a2
14145cc6c: mov rax, qword ptr [rbp + 0x2a0]
14145cc73: mov qword ptr [rbp + 0xe0], rax
14145cc7a: movups xmm0, xmmword ptr [rbp + 0x270]
14145cc81: movups xmm1, xmmword ptr [rbp + 0x280]
14145cc88: movups xmm2, xmmword ptr [rbp + 0x290]
14145cc8f: movaps xmmword ptr [rbp + 0xd0], xmm2
14145cc96: movaps xmmword ptr [rbp + 0xc0], xmm1
14145cc9d: movaps xmmword ptr [rbp + 0xb0], xmm0
14145cc66: jo 0x14145d4a2
14145cc6c: mov rax, qword ptr [rbp + 0x2a0]
14145cc73: mov qword ptr [rbp + 0xe0], rax
14145cc7a: movups xmm0, xmmword ptr [rbp + 0x270]
14145cc81: movups xmm1, xmmword ptr [rbp + 0x280]
14145cc88: movups xmm2, xmmword ptr [rbp + 0x290]
14145cc8f: movaps xmmword ptr [rbp + 0xd0], xmm2
14145cc96: movaps xmmword ptr [rbp + 0xc0], xmm1
14145cc9d: movaps xmmword ptr [rbp + 0xb0], xmm0
14145cca4: mov qword ptr [rbp + 0x6f0], 3
14145ccaf: mov dword ptr [rbp + 0x6f8], 0x32
14145ccb9: lea rax, [rip + 0x18059e0]
14145ccc0: mov qword ptr [rbp + 0x6e0], rax
14145ccc7: mov qword ptr [rbp + 0x6e8], 0x17
14145ccd2: lea rcx, [rbp + 0x270]
14145ccd9: lea r8, [rbp + 0x6e0]
14145cce0: mov rdx, rdi
14145cce3: call 0x140a59220
14145cce8: xor eax, eax
14145ccea: cmp rax, qword ptr [rbp + 0x270]
14145ccf1: jo 0x14145d4e0
14145ccf7: mov rax, qword ptr [rbp + 0x2a0]
14145ccfe: mov qword ptr [rbp + 0xa0], rax
14145cd05: movups xmm0, xmmword ptr [rbp + 0x270]
14145cd0c: movups xmm1, xmmword ptr [rbp + 0x280]
14145cd13: movups xmm2, xmmword ptr [rbp + 0x290]
14145cd1a: movaps xmmword ptr [rbp + 0x90], xmm2
14145cd21: movaps xmmword ptr [rbp + 0x80], xmm1
14145cd28: movaps xmmword ptr [rbp + 0x70], xmm0
14145ccf1: jo 0x14145d4e0
14145ccf7: mov rax, qword ptr [rbp + 0x2a0]
14145ccfe: mov qword ptr [rbp + 0xa0], rax
14145cd05: movups xmm0, xmmword ptr [rbp + 0x270]
14145cd0c: movups xmm1, xmmword ptr [rbp + 0x280]
14145cd13: movups xmm2, xmmword ptr [rbp + 0x290]
14145cd1a: movaps xmmword ptr [rbp + 0x90], xmm2
14145cd21: movaps xmmword ptr [rbp + 0x80], xmm1
14145cd28: movaps xmmword ptr [rbp + 0x70], xmm0
14145cd2c: mov qword ptr [rbp + 0x6f0], 3
14145cd37: mov dword ptr [rbp + 0x6f8], 0x32
14145cd41: lea rax, [rip + 0x1805988]
14145cd48: mov qword ptr [rbp + 0x6e0], rax
14145cd4f: mov qword ptr [rbp + 0x6e8], 0x15
14145cd5a: lea rcx, [rbp + 0x270]
14145cd61: lea r8, [rbp + 0x6e0]
14145cd68: mov rdx, rdi
14145cd6b: call 0x140a59220
14145cd70: xor eax, eax
14145cd72: cmp rax, qword ptr [rbp + 0x270]
14145cd79: jo 0x14145d51e
14145cd7f: mov rax, qword ptr [rbp + 0x2a0]
14145cd86: mov qword ptr [rbp + 0x60], rax
14145cd8a: movups xmm0, xmmword ptr [rbp + 0x270]
14145cd91: movups xmm1, xmmword ptr [rbp + 0x280]
14145cd98: movups xmm2, xmmword ptr [rbp + 0x290]
14145cd9f: movaps xmmword ptr [rbp + 0x50], xmm2
14145cda3: movaps xmmword ptr [rbp + 0x40], xmm1
14145cda7: movaps xmmword ptr [rbp + 0x30], xmm0
14145cd79: jo 0x14145d51e
14145cd7f: mov rax, qword ptr [rbp + 0x2a0]
14145cd86: mov qword ptr [rbp + 0x60], rax
14145cd8a: movups xmm0, xmmword ptr [rbp + 0x270]
14145cd91: movups xmm1, xmmword ptr [rbp + 0x280]
14145cd98: movups xmm2, xmmword ptr [rbp + 0x290]
14145cd9f: movaps xmmword ptr [rbp + 0x50], xmm2
14145cda3: movaps xmmword ptr [rbp + 0x40], xmm1
14145cda7: movaps xmmword ptr [rbp + 0x30], xmm0
14145cdab: mov qword ptr [rbp + 0x6f0], 3
14145cdb6: mov dword ptr [rbp + 0x6f8], 0x32
14145cdc0: lea rax, [rip + 0x1805939]
14145cdc7: mov qword ptr [rbp + 0x6e0], rax
14145cdce: mov qword ptr [rbp + 0x6e8], 0x1b
14145cdd9: lea rcx, [rbp + 0x270]
14145cde0: lea r8, [rbp + 0x6e0]
14145cde7: mov rdx, rdi
14145cdea: call 0x140a59220
14145cdef: xor eax, eax
14145cdf1: cmp rax, qword ptr [rbp + 0x270]
14145cdf8: jo 0x14145d55c
14145cdfe: mov rax, qword ptr [rbp + 0x2a0]
14145ce05: mov qword ptr [rbp + 0x20], rax
14145ce09: movups xmm0, xmmword ptr [rbp + 0x270]
14145ce10: movups xmm1, xmmword ptr [rbp + 0x280]
14145ce17: movups xmm2, xmmword ptr [rbp + 0x290]
14145ce1e: movaps xmmword ptr [rbp + 0x10], xmm2
14145ce22: movaps xmmword ptr [rbp], xmm1
14145ce26: movaps xmmword ptr [rbp - 0x10], xmm0
14145cdf8: jo 0x14145d55c
14145cdfe: mov rax, qword ptr [rbp + 0x2a0]
14145ce05: mov qword ptr [rbp + 0x20], rax
14145ce09: movups xmm0, xmmword ptr [rbp + 0x270]
14145ce10: movups xmm1, xmmword ptr [rbp + 0x280]
14145ce17: movups xmm2, xmmword ptr [rbp + 0x290]
14145ce1e: movaps xmmword ptr [rbp + 0x10], xmm2
14145ce22: movaps xmmword ptr [rbp], xmm1
14145ce26: movaps xmmword ptr [rbp - 0x10], xmm0
14145ce2a: mov qword ptr [rbp + 0x6f0], 3
14145ce35: mov dword ptr [rbp + 0x6f8], 0x32
14145ce3f: lea rax, [rip + 0x18058f2]
14145ce46: mov qword ptr [rbp + 0x6e0], rax
14145ce4d: mov qword ptr [rbp + 0x6e8], 0x23
14145ce58: lea rcx, [rbp + 0x270]
14145ce5f: lea r8, [rbp + 0x6e0]
14145ce66: mov rdx, rdi
14145ce69: call 0x140a59220
14145ce6e: xor eax, eax
14145ce70: cmp rax, qword ptr [rbp + 0x270]
14145ce77: jo 0x14145d59a
14145ce7d: mov rax, qword ptr [rbp + 0x2a0]
14145ce84: mov qword ptr [rbp - 0x20], rax
14145ce88: movups xmm0, xmmword ptr [rbp + 0x270]
14145ce8f: movups xmm1, xmmword ptr [rbp + 0x280]
14145ce96: movups xmm2, xmmword ptr [rbp + 0x290]
14145ce9d: movaps xmmword ptr [rbp - 0x30], xmm2
14145cea1: movaps xmmword ptr [rbp - 0x40], xmm1
14145cea5: movaps xmmword ptr [rbp - 0x50], xmm0
14145ce77: jo 0x14145d59a
14145ce7d: mov rax, qword ptr [rbp + 0x2a0]
14145ce84: mov qword ptr [rbp - 0x20], rax
14145ce88: movups xmm0, xmmword ptr [rbp + 0x270]
14145ce8f: movups xmm1, xmmword ptr [rbp + 0x280]
14145ce96: movups xmm2, xmmword ptr [rbp + 0x290]
14145ce9d: movaps xmmword ptr [rbp - 0x30], xmm2
14145cea1: movaps xmmword ptr [rbp - 0x40], xmm1
14145cea5: movaps xmmword ptr [rbp - 0x50], xmm0
14145cea9: mov qword ptr [rbp + 0x6f0], 3
14145ceb4: mov dword ptr [rbp + 0x6f8], 0x32
14145cebe: lea rax, [rip + 0x18058b3]
14145cec5: mov qword ptr [rbp + 0x6e0], rax
14145cecc: mov qword ptr [rbp + 0x6e8], 0x1f
14145ced7: lea rcx, [rbp + 0x270]
14145cede: lea r8, [rbp + 0x6e0]
14145cee5: mov rdx, rdi
14145cee8: call 0x140a59220
14145ceed: xor eax, eax
14145ceef: cmp rax, qword ptr [rbp + 0x270]
14145cef6: jo 0x14145d5d5
14145cefc: mov rax, qword ptr [rbp + 0x2a0]
14145cf03: mov qword ptr [rbp + 0x620], rax
14145cf0a: movups xmm0, xmmword ptr [rbp + 0x270]
14145cf11: movups xmm1, xmmword ptr [rbp + 0x280]
14145cf18: movups xmm2, xmmword ptr [rbp + 0x290]
14145cf1f: movaps xmmword ptr [rbp + 0x610], xmm2
14145cf26: movaps xmmword ptr [rbp + 0x600], xmm1
14145cf2d: movaps xmmword ptr [rbp + 0x5f0], xmm0
14145cef6: jo 0x14145d5d5
14145cefc: mov rax, qword ptr [rbp + 0x2a0]
14145cf03: mov qword ptr [rbp + 0x620], rax
14145cf0a: movups xmm0, xmmword ptr [rbp + 0x270]
14145cf11: movups xmm1, xmmword ptr [rbp + 0x280]
14145cf18: movups xmm2, xmmword ptr [rbp + 0x290]
14145cf1f: movaps xmmword ptr [rbp + 0x610], xmm2
14145cf26: movaps xmmword ptr [rbp + 0x600], xmm1
14145cf2d: movaps xmmword ptr [rbp + 0x5f0], xmm0
14145cf34: mov qword ptr [rbp + 0x280], 3
14145cf3f: mov dword ptr [rbp + 0x288], 0x32
14145cf49: lea rax, [rip + 0x1805860]
14145cf50: mov qword ptr [rbp + 0x270], rax
14145cf57: mov qword ptr [rbp + 0x278], 0x21
14145cf62: lea rcx, [rbp + 0x6e0]
14145cf69: lea r8, [rbp + 0x270]
14145cf70: mov rdx, rdi
14145cf73: call 0x140a59220
14145cf78: xor eax, eax
14145cf7a: cmp rax, qword ptr [rbp + 0x6e0]
14145cf81: jo 0x14145d610
14145cf87: mov rax, qword ptr [rbp + 0x710]
14145cf8e: mov qword ptr [rbp + 0x5b0], rax
14145cf95: movups xmm0, xmmword ptr [rbp + 0x6e0]
14145cf9c: movups xmm1, xmmword ptr [rbp + 0x6f0]
14145cfa3: movups xmm2, xmmword ptr [rbp + 0x700]
14145cfaa: movaps xmmword ptr [rbp + 0x5a0], xmm2
14145cfb1: movaps xmmword ptr [rbp + 0x590], xmm1
14145cfb8: movaps xmmword ptr [rbp + 0x580], xmm0

country_core::systems::audio::terrain_audio::terrain_audio_init_system @ 0x1414613e0
1414613e0: push rbp
1414613e1: push rsi
1414613e2: push rdi
1414613e3: sub rsp, 0x230
1414613ea: lea rbp, [rsp + 0x80]
1414613f2: mov qword ptr [rbp + 0x1a8], 0xfffffffffffffffe
1414613fd: mov rsi, rdx
141461400: mov rdi, qword ptr [rcx]
141461403: mov qword ptr [rbp + 0x170], 3
14146140e: mov dword ptr [rbp + 0x178], 0x32
141461418: lea rax, [rip + 0x18017fb]
14146141f: mov qword ptr [rbp + 0x160], rax
141461426: mov qword ptr [rbp + 0x168], 0x1b
141461431: lea rcx, [rbp + 0x60]
141461435: lea r8, [rbp + 0x160]
14146143c: mov rdx, rdi
14146143f: call 0x140a59220
141461444: xor eax, eax
141461446: cmp rax, qword ptr [rbp + 0x60]
14146144a: jo 0x141461691
141461450: mov rax, qword ptr [rbp + 0x90]
141461457: mov qword ptr [rbp + 0x50], rax
14146145b: movups xmm0, xmmword ptr [rbp + 0x60]
14146145f: movups xmm1, xmmword ptr [rbp + 0x70]
141461463: movups xmm2, xmmword ptr [rbp + 0x80]
14146146a: movaps xmmword ptr [rbp + 0x40], xmm2
14146146e: movaps xmmword ptr [rbp + 0x30], xmm1
141461472: movaps xmmword ptr [rbp + 0x20], xmm0
141461450: mov rax, qword ptr [rbp + 0x90]
141461457: mov qword ptr [rbp + 0x50], rax
14146145b: movups xmm0, xmmword ptr [rbp + 0x60]
14146145f: movups xmm1, xmmword ptr [rbp + 0x70]
141461463: movups xmm2, xmmword ptr [rbp + 0x80]
14146146a: movaps xmmword ptr [rbp + 0x40], xmm2
14146146e: movaps xmmword ptr [rbp + 0x30], xmm1
141461472: movaps xmmword ptr [rbp + 0x20], xmm0
141461476: mov byte ptr [rbp + 0x58], 0
14146147a: mov qword ptr [rbp + 0x170], 3
141461485: mov dword ptr [rbp + 0x178], 0x32
14146148f: lea rax, [rip + 0x18017f2]
141461496: mov qword ptr [rbp + 0x160], rax
14146149d: mov qword ptr [rbp + 0x168], 0x1c
1414614a8: lea rcx, [rbp + 0x60]
1414614ac: lea r8, [rbp + 0x160]
1414614b3: mov rdx, rdi
1414614b6: call 0x140a59220
1414614bb: xor eax, eax
1414614bd: cmp rax, qword ptr [rbp + 0x60]
1414614c1: jo 0x1414616cc
1414614c7: mov rax, qword ptr [rbp + 0x90]
1414614ce: mov qword ptr [rbp + 0x10], rax
1414614d2: movups xmm0, xmmword ptr [rbp + 0x60]
1414614d6: movups xmm1, xmmword ptr [rbp + 0x70]
1414614da: movups xmm2, xmmword ptr [rbp + 0x80]
1414614e1: movaps xmmword ptr [rbp], xmm2
1414614e5: movaps xmmword ptr [rbp - 0x10], xmm1
1414614e9: movaps xmmword ptr [rbp - 0x20], xmm0
1414614c7: mov rax, qword ptr [rbp + 0x90]
1414614ce: mov qword ptr [rbp + 0x10], rax
1414614d2: movups xmm0, xmmword ptr [rbp + 0x60]
1414614d6: movups xmm1, xmmword ptr [rbp + 0x70]
1414614da: movups xmm2, xmmword ptr [rbp + 0x80]
1414614e1: movaps xmmword ptr [rbp], xmm2
1414614e5: movaps xmmword ptr [rbp - 0x10], xmm1
1414614e9: movaps xmmword ptr [rbp - 0x20], xmm0
1414614ed: mov byte ptr [rbp + 0x18], 0
1414614f1: mov qword ptr [rbp + 0x170], 3
1414614fc: mov dword ptr [rbp + 0x178], 0x32
141461506: lea rax, [rip + 0x18017b3]
14146150d: mov qword ptr [rbp + 0x160], rax
141461514: mov qword ptr [rbp + 0x168], 0x17
14146151f: lea rcx, [rbp + 0x60]
141461523: lea r8, [rbp + 0x160]
14146152a: mov rdx, rdi
14146152d: call 0x140a59220
141461532: xor eax, eax
141461534: cmp rax, qword ptr [rbp + 0x60]
141461538: jo 0x141461704
14146153e: mov rax, qword ptr [rbp + 0x90]
141461545: mov qword ptr [rbp + 0x190], rax
14146154c: movups xmm0, xmmword ptr [rbp + 0x60]
141461550: movups xmm1, xmmword ptr [rbp + 0x70]
141461554: movups xmm2, xmmword ptr [rbp + 0x80]
14146155b: movaps xmmword ptr [rbp + 0x180], xmm2
141461562: movaps xmmword ptr [rbp + 0x170], xmm1
141461569: movaps xmmword ptr [rbp + 0x160], xmm0
14146153e: mov rax, qword ptr [rbp + 0x90]
141461545: mov qword ptr [rbp + 0x190], rax
14146154c: movups xmm0, xmmword ptr [rbp + 0x60]
141461550: movups xmm1, xmmword ptr [rbp + 0x70]
141461554: movups xmm2, xmmword ptr [rbp + 0x80]
14146155b: movaps xmmword ptr [rbp + 0x180], xmm2
141461562: movaps xmmword ptr [rbp + 0x170], xmm1
141461569: movaps xmmword ptr [rbp + 0x160], xmm0
141461570: mov byte ptr [rbp + 0x198], 0
141461577: mov qword ptr [rbp + 0x70], 3
14146157f: mov dword ptr [rbp + 0x78], 0x32
141461586: lea rax, [rip + 0x1801763]
14146158d: mov qword ptr [rbp + 0x60], rax
141461591: mov qword ptr [rbp + 0x68], 0x16
141461599: lea rcx, [rbp - 0x58]
14146159d: lea r8, [rbp + 0x60]
1414615a1: mov rdx, rdi
1414615a4: call 0x140a59220
1414615a9: xor eax, eax
1414615ab: cmp rax, qword ptr [rbp - 0x58]
1414615af: jo 0x14146173c
1414615b5: mov rax, qword ptr [rbp - 0x28]
1414615b9: mov qword ptr [rbp + 0x150], rax
1414615c0: movups xmm0, xmmword ptr [rbp - 0x58]
1414615c4: movups xmm1, xmmword ptr [rbp - 0x48]
1414615c8: movups xmm2, xmmword ptr [rbp - 0x38]
1414615cc: movaps xmmword ptr [rbp + 0x140], xmm2
1414615d3: movaps xmmword ptr [rbp + 0x130], xmm1
1414615da: movaps xmmword ptr [rbp + 0x120], xmm0

country_core::systems::audio::tree_audio::tree_audio_init_system @ 0x1414633e0
1414633e0: push rbp
1414633e1: push rsi
1414633e2: push rdi
1414633e3: push rbx
1414633e4: sub rsp, 0x288
1414633eb: lea rbp, [rsp + 0x80]
1414633f3: mov qword ptr [rbp + 0x200], 0xfffffffffffffffe
1414633fe: mov rsi, rdx
141463401: mov rdi, qword ptr [rcx]
141463404: mov qword ptr [rbp + 0x160], 3
14146340f: mov dword ptr [rbp + 0x168], 0x32
141463419: lea rax, [rip + 0x17ffaa0]
141463420: mov qword ptr [rbp + 0x150], rax
141463427: mov qword ptr [rbp + 0x158], 0x18
141463432: lea rcx, [rbp - 0x50]
141463436: lea r8, [rbp + 0x150]
14146343d: mov rdx, rdi
141463440: call 0x140a59220
141463445: xor eax, eax
141463447: cmp rax, qword ptr [rbp - 0x50]
14146344b: jo 0x1414636ac
141463451: mov rax, qword ptr [rbp - 0x20]
141463455: mov qword ptr [rbp + 0x140], rax
14146345c: movups xmm0, xmmword ptr [rbp - 0x50]
141463460: movups xmm1, xmmword ptr [rbp - 0x40]
141463464: movups xmm2, xmmword ptr [rbp - 0x30]
141463468: movaps xmmword ptr [rbp + 0x130], xmm2
14146346f: movaps xmmword ptr [rbp + 0x120], xmm1
141463476: movaps xmmword ptr [rbp + 0x110], xmm0
14146344b: jo 0x1414636ac
141463451: mov rax, qword ptr [rbp - 0x20]
141463455: mov qword ptr [rbp + 0x140], rax
14146345c: movups xmm0, xmmword ptr [rbp - 0x50]
141463460: movups xmm1, xmmword ptr [rbp - 0x40]
141463464: movups xmm2, xmmword ptr [rbp - 0x30]
141463468: movaps xmmword ptr [rbp + 0x130], xmm2
14146346f: movaps xmmword ptr [rbp + 0x120], xmm1
141463476: movaps xmmword ptr [rbp + 0x110], xmm0
14146347d: mov qword ptr [rbp + 0x160], 3
141463488: mov dword ptr [rbp + 0x168], 0x32
141463492: lea rbx, [rip + 0x17ffa8f]
141463499: mov qword ptr [rbp + 0x150], rbx
1414634a0: mov qword ptr [rbp + 0x158], 0x13
1414634ab: lea rcx, [rbp - 0x50]
1414634af: lea r8, [rbp + 0x150]
1414634b6: mov rdx, rdi
1414634b9: call 0x140a59220
1414634be: xor eax, eax
1414634c0: cmp rax, qword ptr [rbp - 0x50]
1414634c4: jo 0x1414636e7
1414634ca: mov rax, qword ptr [rbp - 0x20]
1414634ce: mov qword ptr [rbp + 0x100], rax
1414634d5: movups xmm0, xmmword ptr [rbp - 0x50]
1414634d9: movups xmm1, xmmword ptr [rbp - 0x40]
1414634dd: movups xmm2, xmmword ptr [rbp - 0x30]
1414634e1: movaps xmmword ptr [rbp + 0xf0], xmm2
1414634e8: movaps xmmword ptr [rbp + 0xe0], xmm1
1414634ef: movaps xmmword ptr [rbp + 0xd0], xmm0
1414634c0: cmp rax, qword ptr [rbp - 0x50]
1414634c4: jo 0x1414636e7
1414634ca: mov rax, qword ptr [rbp - 0x20]
1414634ce: mov qword ptr [rbp + 0x100], rax
1414634d5: movups xmm0, xmmword ptr [rbp - 0x50]
1414634d9: movups xmm1, xmmword ptr [rbp - 0x40]
1414634dd: movups xmm2, xmmword ptr [rbp - 0x30]
1414634e1: movaps xmmword ptr [rbp + 0xf0], xmm2
1414634e8: movaps xmmword ptr [rbp + 0xe0], xmm1
1414634ef: movaps xmmword ptr [rbp + 0xd0], xmm0
1414634f6: mov qword ptr [rbp + 0x160], 3
141463501: mov dword ptr [rbp + 0x168], 0x32
14146350b: mov qword ptr [rbp + 0x150], rbx
141463512: mov qword ptr [rbp + 0x158], 0x13
14146351d: lea rcx, [rbp - 0x50]
141463521: lea r8, [rbp + 0x150]
141463528: mov rdx, rdi
14146352b: call 0x140a59220
141463530: xor eax, eax
141463532: cmp rax, qword ptr [rbp - 0x50]
141463536: jo 0x14146371f
14146353c: mov rax, qword ptr [rbp - 0x20]
141463540: mov qword ptr [rbp + 0xc0], rax
141463547: movups xmm0, xmmword ptr [rbp - 0x50]
14146354b: movups xmm1, xmmword ptr [rbp - 0x40]
14146354f: movups xmm2, xmmword ptr [rbp - 0x30]
141463553: movaps xmmword ptr [rbp + 0xb0], xmm2
14146355a: movaps xmmword ptr [rbp + 0xa0], xmm1
141463561: movaps xmmword ptr [rbp + 0x90], xmm0
141463532: cmp rax, qword ptr [rbp - 0x50]
141463536: jo 0x14146371f
14146353c: mov rax, qword ptr [rbp - 0x20]
141463540: mov qword ptr [rbp + 0xc0], rax
141463547: movups xmm0, xmmword ptr [rbp - 0x50]
14146354b: movups xmm1, xmmword ptr [rbp - 0x40]
14146354f: movups xmm2, xmmword ptr [rbp - 0x30]
141463553: movaps xmmword ptr [rbp + 0xb0], xmm2
14146355a: movaps xmmword ptr [rbp + 0xa0], xmm1
141463561: movaps xmmword ptr [rbp + 0x90], xmm0
141463568: mov qword ptr [rbp + 0x160], 3
141463573: mov dword ptr [rbp + 0x168], 0x32
14146357d: mov qword ptr [rbp + 0x150], rbx
141463584: mov qword ptr [rbp + 0x158], 0x13
14146358f: lea rcx, [rbp - 0x50]
141463593: lea r8, [rbp + 0x150]
14146359a: mov rdx, rdi
14146359d: call 0x140a59220
1414635a2: xor eax, eax
1414635a4: cmp rax, qword ptr [rbp - 0x50]
1414635a8: jo 0x141463757
1414635ae: mov rax, qword ptr [rbp - 0x20]
1414635b2: mov qword ptr [rbp + 0x1f0], rax
1414635b9: movups xmm0, xmmword ptr [rbp - 0x50]
1414635bd: movups xmm1, xmmword ptr [rbp - 0x40]
1414635c1: movups xmm2, xmmword ptr [rbp - 0x30]
1414635c5: movaps xmmword ptr [rbp + 0x1e0], xmm2
1414635cc: movaps xmmword ptr [rbp + 0x1d0], xmm1
1414635d3: movaps xmmword ptr [rbp + 0x1c0], xmm0

country_core::systems::audio::building_audio::building_audio_init_system @ 0x1417d21a0
1417d21a0: push rbp
1417d21a1: push rsi
1417d21a2: push rdi
1417d21a3: sub rsp, 0x380
1417d21aa: lea rbp, [rsp + 0x80]
1417d21b2: mov qword ptr [rbp + 0x2f8], 0xfffffffffffffffe
1417d21bd: mov rsi, rdx
1417d21c0: mov rdi, qword ptr [rcx]
1417d21c3: mov qword ptr [rbp + 0x2d0], 3
1417d21ce: mov dword ptr [rbp + 0x2d8], 0x32
1417d21d8: lea rax, [rip + 0x1512453]
1417d21df: mov qword ptr [rbp + 0x2c0], rax
1417d21e6: mov qword ptr [rbp + 0x2c8], 0x22
1417d21f1: lea rcx, [rbp + 0x130]
1417d21f8: lea r8, [rbp + 0x2c0]
1417d21ff: mov rdx, rdi
1417d2202: call 0x140a59220
1417d2207: xor eax, eax
1417d2209: cmp rax, qword ptr [rbp + 0x130]
1417d2210: jo 0x1417d26b4
1417d2216: mov rax, qword ptr [rbp + 0x160]
1417d221d: mov qword ptr [rbp + 0x120], rax
1417d2224: movups xmm0, xmmword ptr [rbp + 0x130]
1417d222b: movups xmm1, xmmword ptr [rbp + 0x140]
1417d2232: movups xmm2, xmmword ptr [rbp + 0x150]
1417d2239: movaps xmmword ptr [rbp + 0x110], xmm2
1417d2240: movaps xmmword ptr [rbp + 0x100], xmm1
1417d2247: movaps xmmword ptr [rbp + 0xf0], xmm0
1417d2210: jo 0x1417d26b4
1417d2216: mov rax, qword ptr [rbp + 0x160]
1417d221d: mov qword ptr [rbp + 0x120], rax
1417d2224: movups xmm0, xmmword ptr [rbp + 0x130]
1417d222b: movups xmm1, xmmword ptr [rbp + 0x140]
1417d2232: movups xmm2, xmmword ptr [rbp + 0x150]
1417d2239: movaps xmmword ptr [rbp + 0x110], xmm2
1417d2240: movaps xmmword ptr [rbp + 0x100], xmm1
1417d2247: movaps xmmword ptr [rbp + 0xf0], xmm0
1417d224e: mov qword ptr [rbp + 0x2d0], 3
1417d2259: mov dword ptr [rbp + 0x2d8], 0x32
1417d2263: lea rax, [rip + 0x151243e]
1417d226a: mov qword ptr [rbp + 0x2c0], rax
1417d2271: mov qword ptr [rbp + 0x2c8], 0x2b
1417d227c: lea rcx, [rbp + 0x130]
1417d2283: lea r8, [rbp + 0x2c0]
1417d228a: mov rdx, rdi
1417d228d: call 0x140a59220
1417d2292: xor eax, eax
1417d2294: cmp rax, qword ptr [rbp + 0x130]
1417d229b: jo 0x1417d26f2
1417d22a1: mov rax, qword ptr [rbp + 0x160]
1417d22a8: mov qword ptr [rbp + 0xe0], rax
1417d22af: movups xmm0, xmmword ptr [rbp + 0x130]
1417d22b6: movups xmm1, xmmword ptr [rbp + 0x140]
1417d22bd: movups xmm2, xmmword ptr [rbp + 0x150]
1417d22c4: movaps xmmword ptr [rbp + 0xd0], xmm2
1417d22cb: movaps xmmword ptr [rbp + 0xc0], xmm1
1417d22d2: movaps xmmword ptr [rbp + 0xb0], xmm0
1417d229b: jo 0x1417d26f2
1417d22a1: mov rax, qword ptr [rbp + 0x160]
1417d22a8: mov qword ptr [rbp + 0xe0], rax
1417d22af: movups xmm0, xmmword ptr [rbp + 0x130]
1417d22b6: movups xmm1, xmmword ptr [rbp + 0x140]
1417d22bd: movups xmm2, xmmword ptr [rbp + 0x150]
1417d22c4: movaps xmmword ptr [rbp + 0xd0], xmm2
1417d22cb: movaps xmmword ptr [rbp + 0xc0], xmm1
1417d22d2: movaps xmmword ptr [rbp + 0xb0], xmm0
1417d22d9: mov qword ptr [rbp + 0x2d0], 3
1417d22e4: mov dword ptr [rbp + 0x2d8], 0x32
1417d22ee: lea rax, [rip + 0x15123fb]
1417d22f5: mov qword ptr [rbp + 0x2c0], rax
1417d22fc: mov qword ptr [rbp + 0x2c8], 0x28
1417d2307: lea rcx, [rbp + 0x130]
1417d230e: lea r8, [rbp + 0x2c0]
1417d2315: mov rdx, rdi
1417d2318: call 0x140a59220
1417d231d: xor eax, eax
1417d231f: cmp rax, qword ptr [rbp + 0x130]
1417d2326: jo 0x1417d2730
1417d232c: mov rax, qword ptr [rbp + 0x160]
1417d2333: mov qword ptr [rbp + 0xa0], rax
1417d233a: movups xmm0, xmmword ptr [rbp + 0x130]
1417d2341: movups xmm1, xmmword ptr [rbp + 0x140]
1417d2348: movups xmm2, xmmword ptr [rbp + 0x150]
1417d234f: movaps xmmword ptr [rbp + 0x90], xmm2
1417d2356: movaps xmmword ptr [rbp + 0x80], xmm1
1417d235d: movaps xmmword ptr [rbp + 0x70], xmm0
1417d2326: jo 0x1417d2730
1417d232c: mov rax, qword ptr [rbp + 0x160]
1417d2333: mov qword ptr [rbp + 0xa0], rax
1417d233a: movups xmm0, xmmword ptr [rbp + 0x130]
1417d2341: movups xmm1, xmmword ptr [rbp + 0x140]
1417d2348: movups xmm2, xmmword ptr [rbp + 0x150]
1417d234f: movaps xmmword ptr [rbp + 0x90], xmm2
1417d2356: movaps xmmword ptr [rbp + 0x80], xmm1
1417d235d: movaps xmmword ptr [rbp + 0x70], xmm0
1417d2361: mov qword ptr [rbp + 0x2d0], 3
1417d236c: mov dword ptr [rbp + 0x2d8], 0x32
1417d2376: lea rax, [rip + 0x15123b3]
1417d237d: mov qword ptr [rbp + 0x2c0], rax
1417d2384: mov qword ptr [rbp + 0x2c8], 0x28
1417d238f: lea rcx, [rbp + 0x130]
1417d2396: lea r8, [rbp + 0x2c0]
1417d239d: mov rdx, rdi
1417d23a0: call 0x140a59220
1417d23a5: xor eax, eax
1417d23a7: cmp rax, qword ptr [rbp + 0x130]
1417d23ae: jo 0x1417d276e
1417d23b4: mov rax, qword ptr [rbp + 0x160]
1417d23bb: mov qword ptr [rbp + 0x60], rax
1417d23bf: movups xmm0, xmmword ptr [rbp + 0x130]
1417d23c6: movups xmm1, xmmword ptr [rbp + 0x140]
1417d23cd: movups xmm2, xmmword ptr [rbp + 0x150]
1417d23d4: movaps xmmword ptr [rbp + 0x50], xmm2
1417d23d8: movaps xmmword ptr [rbp + 0x40], xmm1
1417d23dc: movaps xmmword ptr [rbp + 0x30], xmm0
1417d23ae: jo 0x1417d276e
1417d23b4: mov rax, qword ptr [rbp + 0x160]
1417d23bb: mov qword ptr [rbp + 0x60], rax
1417d23bf: movups xmm0, xmmword ptr [rbp + 0x130]
1417d23c6: movups xmm1, xmmword ptr [rbp + 0x140]
1417d23cd: movups xmm2, xmmword ptr [rbp + 0x150]
1417d23d4: movaps xmmword ptr [rbp + 0x50], xmm2
1417d23d8: movaps xmmword ptr [rbp + 0x40], xmm1
1417d23dc: movaps xmmword ptr [rbp + 0x30], xmm0
1417d23e0: mov qword ptr [rbp + 0x2d0], 3
1417d23eb: mov dword ptr [rbp + 0x2d8], 0x32
1417d23f5: lea rax, [rip + 0x1512374]
1417d23fc: mov qword ptr [rbp + 0x2c0], rax
1417d2403: mov qword ptr [rbp + 0x2c8], 0x20
1417d240e: lea rcx, [rbp + 0x130]
1417d2415: lea r8, [rbp + 0x2c0]
1417d241c: mov rdx, rdi
1417d241f: call 0x140a59220
1417d2424: xor eax, eax
1417d2426: cmp rax, qword ptr [rbp + 0x130]
1417d242d: jo 0x1417d27ac
1417d2433: mov rax, qword ptr [rbp + 0x160]
1417d243a: mov qword ptr [rbp + 0x20], rax
1417d243e: movups xmm0, xmmword ptr [rbp + 0x130]
1417d2445: movups xmm1, xmmword ptr [rbp + 0x140]
1417d244c: movups xmm2, xmmword ptr [rbp + 0x150]
1417d2453: movaps xmmword ptr [rbp + 0x10], xmm2
1417d2457: movaps xmmword ptr [rbp], xmm1
1417d245b: movaps xmmword ptr [rbp - 0x10], xmm0
1417d242d: jo 0x1417d27ac
1417d2433: mov rax, qword ptr [rbp + 0x160]
1417d243a: mov qword ptr [rbp + 0x20], rax
1417d243e: movups xmm0, xmmword ptr [rbp + 0x130]
1417d2445: movups xmm1, xmmword ptr [rbp + 0x140]
1417d244c: movups xmm2, xmmword ptr [rbp + 0x150]
1417d2453: movaps xmmword ptr [rbp + 0x10], xmm2
1417d2457: movaps xmmword ptr [rbp], xmm1
1417d245b: movaps xmmword ptr [rbp - 0x10], xmm0
1417d245f: mov qword ptr [rbp + 0x2d0], 3
1417d246a: mov dword ptr [rbp + 0x2d8], 0x32
1417d2474: lea rax, [rip + 0x151232d]
1417d247b: mov qword ptr [rbp + 0x2c0], rax
1417d2482: mov qword ptr [rbp + 0x2c8], 0x1c
1417d248d: lea rcx, [rbp + 0x130]
1417d2494: lea r8, [rbp + 0x2c0]
1417d249b: mov rdx, rdi
1417d249e: call 0x140a59220
1417d24a3: xor eax, eax
1417d24a5: cmp rax, qword ptr [rbp + 0x130]
1417d24ac: jo 0x1417d27e7
1417d24b2: mov rax, qword ptr [rbp + 0x160]
1417d24b9: mov qword ptr [rbp - 0x20], rax
1417d24bd: movups xmm0, xmmword ptr [rbp + 0x130]
1417d24c4: movups xmm1, xmmword ptr [rbp + 0x140]
1417d24cb: movups xmm2, xmmword ptr [rbp + 0x150]
1417d24d2: movaps xmmword ptr [rbp - 0x30], xmm2
1417d24d6: movaps xmmword ptr [rbp - 0x40], xmm1
1417d24da: movaps xmmword ptr [rbp - 0x50], xmm0
1417d24ac: jo 0x1417d27e7
1417d24b2: mov rax, qword ptr [rbp + 0x160]
1417d24b9: mov qword ptr [rbp - 0x20], rax
1417d24bd: movups xmm0, xmmword ptr [rbp + 0x130]
1417d24c4: movups xmm1, xmmword ptr [rbp + 0x140]
1417d24cb: movups xmm2, xmmword ptr [rbp + 0x150]
1417d24d2: movaps xmmword ptr [rbp - 0x30], xmm2
1417d24d6: movaps xmmword ptr [rbp - 0x40], xmm1
1417d24da: movaps xmmword ptr [rbp - 0x50], xmm0
1417d24de: mov qword ptr [rbp + 0x140], 3
1417d24e9: mov dword ptr [rbp + 0x148], 0x32
1417d24f3: lea rax, [rip + 0x15122e6]
1417d24fa: mov qword ptr [rbp + 0x130], rax
1417d2501: mov qword ptr [rbp + 0x138], 0x27
1417d250c: lea rcx, [rbp + 0x2c0]
1417d2513: lea r8, [rbp + 0x130]
1417d251a: mov rdx, rdi
1417d251d: call 0x140a59220
1417d2522: xor eax, eax
1417d2524: cmp rax, qword ptr [rbp + 0x2c0]
1417d252b: jo 0x1417d2822
1417d2531: mov rax, qword ptr [rbp + 0x2f0]
1417d2538: mov qword ptr [rbp + 0x2b0], rax
1417d253f: movups xmm0, xmmword ptr [rbp + 0x2c0]
1417d2546: movups xmm1, xmmword ptr [rbp + 0x2d0]
1417d254d: movups xmm2, xmmword ptr [rbp + 0x2e0]
1417d2554: movaps xmmword ptr [rbp + 0x2a0], xmm2
1417d255b: movaps xmmword ptr [rbp + 0x290], xmm1
1417d2562: movaps xmmword ptr [rbp + 0x280], xmm0