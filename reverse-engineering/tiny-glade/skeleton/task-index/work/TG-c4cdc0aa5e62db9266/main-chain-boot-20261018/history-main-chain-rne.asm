0x141737fb0: push rbp
0x141737fb1: push rsi
0x141737fb2: push rdi
0x141737fb3: push rbx
0x141737fb4: sub rsp, 0x428
0x141737fbb: lea rbp, [rsp + 0x80]
0x141737fc3: mov qword ptr [rbp + 0x3a0], 0xfffffffffffffffe
0x141737fce: mov rsi, rcx
0x141737fd1: movzx eax, byte ptr [rip + 0x1efcc31]
0x141737fd8: mov ecx, 0x2a0
0x141737fdd: mov edx, 8
0x141737fe2: call 0x140613c10
0x141737fe7: test rax, rax
0x141737fea: je 0x141738473
0x141737ff0: mov qword ptr [rbp + 0x398], rax
0x141737ff7: xorps xmm0, xmm0
0x141737ffa: movups xmmword ptr [rbp + 0x278], xmm0
0x141738001: movups xmmword ptr [rbp + 0x290], xmm0
0x141738008: movups xmmword ptr [rbp + 0x2a8], xmm0
0x14173800f: movups xmmword ptr [rbp + 0x2c0], xmm0
0x141738016: movups xmmword ptr [rbp + 0x2d8], xmm0
0x14173801d: mov dword ptr [rbp + 0x2e8], 0
0x141738027: movups xmmword ptr [rbp + 0x310], xmm0
0x14173802e: movups xmmword ptr [rbp + 0x328], xmm0
0x141738035: movups xmmword ptr [rbp + 0x340], xmm0
0x14173803c: movups xmmword ptr [rbp + 0x358], xmm0
0x141738043: movups xmmword ptr [rbp + 0x370], xmm0
0x14173804a: mov dword ptr [rbp + 0x380], 0
0x141738054: mov qword ptr [rbp + 0x1d8], 0
0x14173805f: mov qword ptr [rbp + 0x258], 0
0x14173806a: mov qword ptr [rbp + 0x260], 8
0x141738075: mov qword ptr [rbp + 0x268], 0
0x141738080: mov qword ptr [rbp + 0x270], 0x10
0x14173808b: mov qword ptr [rbp + 0x288], 0x10
0x141738096: mov qword ptr [rbp + 0x2a0], 0x10
0x1417380a1: mov qword ptr [rbp + 0x2b8], 0x10
0x1417380ac: mov qword ptr [rbp + 0x2d0], 0x10
0x1417380b7: movabs rbx, 0x8000000000000000
0x1417380c1: mov qword ptr [rbp + 0x2f0], rbx
0x1417380c8: lea rax, [rip + 0x15954c5]
0x1417380cf: mov qword ptr [rbp + 0x2f8], rax
0x1417380d6: mov qword ptr [rbp + 0x300], 0x4b
0x1417380e1: mov qword ptr [rbp + 0x308], 0x10
0x1417380ec: mov qword ptr [rbp + 0x320], 0x10
0x1417380f7: mov qword ptr [rbp + 0x338], 0x10
0x141738102: mov qword ptr [rbp + 0x350], 0x10
0x14173810d: mov qword ptr [rbp + 0x368], 0x10
0x141738118: mov dword ptr [rbp + 0x388], 0
0x141738122: mov word ptr [rbp + 0x38c], 1
0x14173812b: mov dword ptr [rbp + 0x390], 0
0x141738135: movzx eax, byte ptr [rip + 0x1efcacd]
0x14173813c: mov ecx, 0x1c0
0x141738141: mov edx, 8
0x141738146: call 0x140613c10
0x14173814b: test rax, rax
0x14173814e: je 0x141738482
0x141738154: mov rdi, rax
0x141738157: lea rdx, [rbp + 0x1d8]
0x14173815e: mov r8d, 0x1c0
0x141738164: mov rcx, rax
0x141738167: call 0x142923c00
0x14173816c: lea r8, [rip + 0x159505d]
0x141738173: lea rcx, [rbp + 0x168]
0x14173817a: mov rdx, rdi
0x14173817d: call 0x140ce4b10
0x141738182: lea rcx, [rbp + 0xf8]
0x141738189: call 0x14143b610
0x14173818e: lea rcx, [rbp + 0x88]
0x141738195: call 0x14143b290
0x14173819a: lea rcx, [rbp + 0x18]
0x14173819e: call 0x140a9f240
0x1417381a3: lea rcx, [rbp - 0x58]
0x1417381a7: call 0x141740460
0x1417381ac: lea rcx, [rbp + 0x1d8]
0x1417381b3: call 0x140a26240
0x1417381b8: movups xmm0, xmmword ptr [rbp + 0x1c8]
0x1417381bf: mov rax, qword ptr [rbp + 0x398]
0x1417381c6: movups xmmword ptr [rax + 0x60], xmm0
0x1417381ca: movups xmm0, xmmword ptr [rbp + 0x1b8]
0x1417381d1: movups xmmword ptr [rax + 0x50], xmm0
0x1417381d5: movups xmm0, xmmword ptr [rbp + 0x1a8]
0x1417381dc: movups xmmword ptr [rax + 0x40], xmm0
0x1417381e0: movups xmm0, xmmword ptr [rbp + 0x168]
0x1417381e7: movups xmm1, xmmword ptr [rbp + 0x178]
0x1417381ee: movups xmm2, xmmword ptr [rbp + 0x188]
0x1417381f5: movups xmm3, xmmword ptr [rbp + 0x198]
0x1417381fc: movups xmmword ptr [rax + 0x30], xmm3
0x141738200: movups xmmword ptr [rax + 0x20], xmm2
0x141738204: movups xmmword ptr [rax + 0x10], xmm1
0x141738208: movups xmmword ptr [rax], xmm0
0x14173820b: movups xmm0, xmmword ptr [rbp + 0xf8]
0x141738212: movups xmm1, xmmword ptr [rbp + 0x108]
0x141738219: movups xmm2, xmmword ptr [rbp + 0x118]
0x141738220: movups xmm3, xmmword ptr [rbp + 0x128]
0x141738227: movups xmmword ptr [rax + 0x70], xmm0
0x14173822b: movups xmmword ptr [rax + 0x80], xmm1
0x141738232: movups xmmword ptr [rax + 0x90], xmm2
0x141738239: movups xmmword ptr [rax + 0xa0], xmm3
0x141738240: movups xmm0, xmmword ptr [rbp + 0x138]
0x141738247: movups xmmword ptr [rax + 0xb0], xmm0
0x14173824e: movups xmm0, xmmword ptr [rbp + 0x148]
0x141738255: movups xmmword ptr [rax + 0xc0], xmm0
0x14173825c: movups xmm0, xmmword ptr [rbp + 0x158]
0x141738263: movups xmmword ptr [rax + 0xd0], xmm0
0x14173826a: movups xmm0, xmmword ptr [rbp + 0xe8]
0x141738271: movups xmmword ptr [rax + 0x140], xmm0
0x141738278: movups xmm0, xmmword ptr [rbp + 0xd8]
0x14173827f: movups xmmword ptr [rax + 0x130], xmm0
0x141738286: movups xmm0, xmmword ptr [rbp + 0xc8]
0x14173828d: movups xmmword ptr [rax + 0x120], xmm0
0x141738294: movups xmm0, xmmword ptr [rbp + 0x88]
0x14173829b: movups xmm1, xmmword ptr [rbp + 0x98]
0x1417382a2: movups xmm2, xmmword ptr [rbp + 0xa8]
0x1417382a9: movups xmm3, xmmword ptr [rbp + 0xb8]
0x1417382b0: movups xmmword ptr [rax + 0x110], xmm3
0x1417382b7: movups xmmword ptr [rax + 0x100], xmm2
0x1417382be: movups xmmword ptr [rax + 0xf0], xmm1
0x1417382c5: movups xmmword ptr [rax + 0xe0], xmm0
0x1417382cc: movups xmm0, xmmword ptr [rbp + 0x18]
0x1417382d0: movups xmm1, xmmword ptr [rbp + 0x28]
0x1417382d4: movups xmm2, xmmword ptr [rbp + 0x38]
0x1417382d8: movups xmm3, xmmword ptr [rbp + 0x48]
0x1417382dc: movups xmmword ptr [rax + 0x150], xmm0
0x1417382e3: movups xmmword ptr [rax + 0x160], xmm1
0x1417382ea: movups xmmword ptr [rax + 0x170], xmm2
0x1417382f1: movups xmmword ptr [rax + 0x180], xmm3
0x1417382f8: movups xmm0, xmmword ptr [rbp + 0x58]
0x1417382fc: movups xmmword ptr [rax + 0x190], xmm0
0x141738303: movups xmm0, xmmword ptr [rbp + 0x68]
0x141738307: movups xmmword ptr [rax + 0x1a0], xmm0
0x14173830e: movups xmm0, xmmword ptr [rbp + 0x78]
0x141738312: movups xmmword ptr [rax + 0x1b0], xmm0
0x141738319: movups xmm0, xmmword ptr [rbp - 0x58]
0x14173831d: movups xmm1, xmmword ptr [rbp - 0x48]
0x141738321: movups xmm2, xmmword ptr [rbp - 0x38]
0x141738325: movups xmm3, xmmword ptr [rbp - 0x28]
0x141738329: movups xmmword ptr [rax + 0x1c0], xmm0
0x141738330: movups xmmword ptr [rax + 0x1d0], xmm1
0x141738337: movups xmmword ptr [rax + 0x1e0], xmm2
0x14173833e: movups xmmword ptr [rax + 0x1f0], xmm3
0x141738345: movups xmm0, xmmword ptr [rbp - 0x18]
0x141738349: movups xmmword ptr [rax + 0x200], xmm0
0x141738350: movups xmm0, xmmword ptr [rbp + 8]
0x141738354: movups xmmword ptr [rax + 0x220], xmm0
0x14173835b: movups xmm0, xmmword ptr [rbp - 8]
0x14173835f: movups xmmword ptr [rax + 0x210], xmm0
0x141738366: movups xmm0, xmmword ptr [rbp + 0x238]
0x14173836d: movups xmmword ptr [rax + 0x290], xmm0
0x141738374: movups xmm0, xmmword ptr [rbp + 0x228]
0x14173837b: movups xmmword ptr [rax + 0x280], xmm0
0x141738382: movups xmm0, xmmword ptr [rbp + 0x218]
0x141738389: movups xmmword ptr [rax + 0x270], xmm0
0x141738390: movups xmm0, xmmword ptr [rbp + 0x1d8]
0x141738397: movups xmm1, xmmword ptr [rbp + 0x1e8]
0x14173839e: movups xmm2, xmmword ptr [rbp + 0x1f8]
0x1417383a5: movups xmm3, xmmword ptr [rbp + 0x208]
0x1417383ac: movups xmmword ptr [rax + 0x260], xmm3
0x1417383b3: movups xmmword ptr [rax + 0x250], xmm2
0x1417383ba: movups xmmword ptr [rax + 0x230], xmm0
0x1417383c1: movups xmmword ptr [rax + 0x240], xmm1
0x1417383c8: mov qword ptr [rbp + 0x1e0], 6
0x1417383d3: mov qword ptr [rbp + 0x1e8], rax
0x1417383da: mov qword ptr [rbp + 0x1f0], 6
0x1417383e5: mov qword ptr [rbp + 0x1f8], 0
0x1417383f0: mov qword ptr [rbp + 0x200], 8
0x1417383fb: xorps xmm0, xmm0
0x1417383fe: movups xmmword ptr [rbp + 0x208], xmm0
0x141738405: mov qword ptr [rbp + 0x1d8], rbx
0x14173840c: lea rcx, [rbp + 0x210]
0x141738413: call 0x1416a00a0
0x141738418: movups xmm0, xmmword ptr [rbp + 0x238]
0x14173841f: movups xmmword ptr [rsi + 0x60], xmm0
0x141738423: movups xmm0, xmmword ptr [rbp + 0x228]
0x14173842a: movups xmmword ptr [rsi + 0x50], xmm0
0x14173842e: movups xmm0, xmmword ptr [rbp + 0x218]
0x141738435: movups xmmword ptr [rsi + 0x40], xmm0
0x141738439: movups xmm0, xmmword ptr [rbp + 0x1d8]
0x141738440: movups xmm1, xmmword ptr [rbp + 0x1e8]
0x141738447: movups xmm2, xmmword ptr [rbp + 0x1f8]
0x14173844e: movups xmm3, xmmword ptr [rbp + 0x208]
0x141738455: movups xmmword ptr [rsi + 0x30], xmm3
0x141738459: movups xmmword ptr [rsi + 0x20], xmm2
