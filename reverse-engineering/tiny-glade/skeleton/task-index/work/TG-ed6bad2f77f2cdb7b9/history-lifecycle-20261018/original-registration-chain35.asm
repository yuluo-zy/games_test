141737fb0: push rbp
141737fb1: push rsi
141737fb2: push rdi
141737fb3: push rbx
141737fb4: sub rsp, 0x428
141737fbb: lea rbp, [rsp + 0x80]
141737fc3: mov qword ptr [rbp + 0x3a0], 0xfffffffffffffffe
141737fce: mov rsi, rcx
141737fd1: movzx eax, byte ptr [rip + 0x1efcc31]
141737fd8: mov ecx, 0x2a0
141737fdd: mov edx, 8
141737fe2: call 0x140613c10
141737fe7: test rax, rax
141737fea: je 0x141738473
141737ff0: mov qword ptr [rbp + 0x398], rax
141737ff7: xorps xmm0, xmm0
141737ffa: movups xmmword ptr [rbp + 0x278], xmm0
141738001: movups xmmword ptr [rbp + 0x290], xmm0
141738008: movups xmmword ptr [rbp + 0x2a8], xmm0
14173800f: movups xmmword ptr [rbp + 0x2c0], xmm0
141738016: movups xmmword ptr [rbp + 0x2d8], xmm0
14173801d: mov dword ptr [rbp + 0x2e8], 0
141738027: movups xmmword ptr [rbp + 0x310], xmm0
14173802e: movups xmmword ptr [rbp + 0x328], xmm0
141738035: movups xmmword ptr [rbp + 0x340], xmm0
14173803c: movups xmmword ptr [rbp + 0x358], xmm0
141738043: movups xmmword ptr [rbp + 0x370], xmm0
14173804a: mov dword ptr [rbp + 0x380], 0
141738054: mov qword ptr [rbp + 0x1d8], 0
14173805f: mov qword ptr [rbp + 0x258], 0
14173806a: mov qword ptr [rbp + 0x260], 8
141738075: mov qword ptr [rbp + 0x268], 0
141738080: mov qword ptr [rbp + 0x270], 0x10
14173808b: mov qword ptr [rbp + 0x288], 0x10
141738096: mov qword ptr [rbp + 0x2a0], 0x10
1417380a1: mov qword ptr [rbp + 0x2b8], 0x10
1417380ac: mov qword ptr [rbp + 0x2d0], 0x10
1417380b7: movabs rbx, 0x8000000000000000
1417380c1: mov qword ptr [rbp + 0x2f0], rbx
1417380c8: lea rax, [rip + 0x15954c5]
1417380cf: mov qword ptr [rbp + 0x2f8], rax
1417380d6: mov qword ptr [rbp + 0x300], 0x4b
1417380e1: mov qword ptr [rbp + 0x308], 0x10
1417380ec: mov qword ptr [rbp + 0x320], 0x10
1417380f7: mov qword ptr [rbp + 0x338], 0x10
141738102: mov qword ptr [rbp + 0x350], 0x10
14173810d: mov qword ptr [rbp + 0x368], 0x10
141738118: mov dword ptr [rbp + 0x388], 0
141738122: mov word ptr [rbp + 0x38c], 1
14173812b: mov dword ptr [rbp + 0x390], 0
141738135: movzx eax, byte ptr [rip + 0x1efcacd]
14173813c: mov ecx, 0x1c0
141738141: mov edx, 8
141738146: call 0x140613c10
14173814b: test rax, rax
14173814e: je 0x141738482
141738154: mov rdi, rax
141738157: lea rdx, [rbp + 0x1d8]
14173815e: mov r8d, 0x1c0
141738164: mov rcx, rax
141738167: call 0x142923c00
14173816c: lea r8, [rip + 0x159505d]
141738173: lea rcx, [rbp + 0x168]
14173817a: mov rdx, rdi
14173817d: call 0x140ce4b10
141738182: lea rcx, [rbp + 0xf8]
141738189: call 0x14143b610
14173818e: lea rcx, [rbp + 0x88]
141738195: call 0x14143b290
14173819a: lea rcx, [rbp + 0x18]
14173819e: call 0x140a9f240
1417381a3: lea rcx, [rbp - 0x58]
1417381a7: call 0x141740460
1417381ac: lea rcx, [rbp + 0x1d8]
1417381b3: call 0x140a26240
1417381b8: movups xmm0, xmmword ptr [rbp + 0x1c8]
1417381bf: mov rax, qword ptr [rbp + 0x398]
1417381c6: movups xmmword ptr [rax + 0x60], xmm0
1417381ca: movups xmm0, xmmword ptr [rbp + 0x1b8]
1417381d1: movups xmmword ptr [rax + 0x50], xmm0
1417381d5: movups xmm0, xmmword ptr [rbp + 0x1a8]
1417381dc: movups xmmword ptr [rax + 0x40], xmm0
1417381e0: movups xmm0, xmmword ptr [rbp + 0x168]
1417381e7: movups xmm1, xmmword ptr [rbp + 0x178]
1417381ee: movups xmm2, xmmword ptr [rbp + 0x188]
1417381f5: movups xmm3, xmmword ptr [rbp + 0x198]
1417381fc: movups xmmword ptr [rax + 0x30], xmm3
141738200: movups xmmword ptr [rax + 0x20], xmm2
141738204: movups xmmword ptr [rax + 0x10], xmm1
141738208: movups xmmword ptr [rax], xmm0
14173820b: movups xmm0, xmmword ptr [rbp + 0xf8]
141738212: movups xmm1, xmmword ptr [rbp + 0x108]
141738219: movups xmm2, xmmword ptr [rbp + 0x118]
141738220: movups xmm3, xmmword ptr [rbp + 0x128]
141738227: movups xmmword ptr [rax + 0x70], xmm0
14173822b: movups xmmword ptr [rax + 0x80], xmm1
141738232: movups xmmword ptr [rax + 0x90], xmm2
141738239: movups xmmword ptr [rax + 0xa0], xmm3
141738240: movups xmm0, xmmword ptr [rbp + 0x138]
141738247: movups xmmword ptr [rax + 0xb0], xmm0
14173824e: movups xmm0, xmmword ptr [rbp + 0x148]
141738255: movups xmmword ptr [rax + 0xc0], xmm0
14173825c: movups xmm0, xmmword ptr [rbp + 0x158]
141738263: movups xmmword ptr [rax + 0xd0], xmm0
14173826a: movups xmm0, xmmword ptr [rbp + 0xe8]
141738271: movups xmmword ptr [rax + 0x140], xmm0
141738278: movups xmm0, xmmword ptr [rbp + 0xd8]
14173827f: movups xmmword ptr [rax + 0x130], xmm0
141738286: movups xmm0, xmmword ptr [rbp + 0xc8]
14173828d: movups xmmword ptr [rax + 0x120], xmm0
141738294: movups xmm0, xmmword ptr [rbp + 0x88]
14173829b: movups xmm1, xmmword ptr [rbp + 0x98]
1417382a2: movups xmm2, xmmword ptr [rbp + 0xa8]
1417382a9: movups xmm3, xmmword ptr [rbp + 0xb8]
1417382b0: movups xmmword ptr [rax + 0x110], xmm3
1417382b7: movups xmmword ptr [rax + 0x100], xmm2
1417382be: movups xmmword ptr [rax + 0xf0], xmm1
1417382c5: movups xmmword ptr [rax + 0xe0], xmm0
1417382cc: movups xmm0, xmmword ptr [rbp + 0x18]
1417382d0: movups xmm1, xmmword ptr [rbp + 0x28]
1417382d4: movups xmm2, xmmword ptr [rbp + 0x38]
1417382d8: movups xmm3, xmmword ptr [rbp + 0x48]
1417382dc: movups xmmword ptr [rax + 0x150], xmm0
1417382e3: movups xmmword ptr [rax + 0x160], xmm1
1417382ea: movups xmmword ptr [rax + 0x170], xmm2
1417382f1: movups xmmword ptr [rax + 0x180], xmm3
1417382f8: movups xmm0, xmmword ptr [rbp + 0x58]
1417382fc: movups xmmword ptr [rax + 0x190], xmm0
141738303: movups xmm0, xmmword ptr [rbp + 0x68]
141738307: movups xmmword ptr [rax + 0x1a0], xmm0
14173830e: movups xmm0, xmmword ptr [rbp + 0x78]
141738312: movups xmmword ptr [rax + 0x1b0], xmm0
141738319: movups xmm0, xmmword ptr [rbp - 0x58]
14173831d: movups xmm1, xmmword ptr [rbp - 0x48]
141738321: movups xmm2, xmmword ptr [rbp - 0x38]
141738325: movups xmm3, xmmword ptr [rbp - 0x28]
141738329: movups xmmword ptr [rax + 0x1c0], xmm0
141738330: movups xmmword ptr [rax + 0x1d0], xmm1
141738337: movups xmmword ptr [rax + 0x1e0], xmm2
14173833e: movups xmmword ptr [rax + 0x1f0], xmm3
141738345: movups xmm0, xmmword ptr [rbp - 0x18]
141738349: movups xmmword ptr [rax + 0x200], xmm0
141738350: movups xmm0, xmmword ptr [rbp + 8]
141738354: movups xmmword ptr [rax + 0x220], xmm0
14173835b: movups xmm0, xmmword ptr [rbp - 8]
14173835f: movups xmmword ptr [rax + 0x210], xmm0
141738366: movups xmm0, xmmword ptr [rbp + 0x238]
14173836d: movups xmmword ptr [rax + 0x290], xmm0
141738374: movups xmm0, xmmword ptr [rbp + 0x228]
14173837b: movups xmmword ptr [rax + 0x280], xmm0
141738382: movups xmm0, xmmword ptr [rbp + 0x218]
141738389: movups xmmword ptr [rax + 0x270], xmm0
141738390: movups xmm0, xmmword ptr [rbp + 0x1d8]
141738397: movups xmm1, xmmword ptr [rbp + 0x1e8]
14173839e: movups xmm2, xmmword ptr [rbp + 0x1f8]
1417383a5: movups xmm3, xmmword ptr [rbp + 0x208]
1417383ac: movups xmmword ptr [rax + 0x260], xmm3
1417383b3: movups xmmword ptr [rax + 0x250], xmm2
1417383ba: movups xmmword ptr [rax + 0x230], xmm0
1417383c1: movups xmmword ptr [rax + 0x240], xmm1
1417383c8: mov qword ptr [rbp + 0x1e0], 6
1417383d3: mov qword ptr [rbp + 0x1e8], rax
1417383da: mov qword ptr [rbp + 0x1f0], 6
1417383e5: mov qword ptr [rbp + 0x1f8], 0
1417383f0: mov qword ptr [rbp + 0x200], 8
1417383fb: xorps xmm0, xmm0
1417383fe: movups xmmword ptr [rbp + 0x208], xmm0
141738405: mov qword ptr [rbp + 0x1d8], rbx
14173840c: lea rcx, [rbp + 0x210]
141738413: call 0x1416a00a0
141738418: movups xmm0, xmmword ptr [rbp + 0x238]
14173841f: movups xmmword ptr [rsi + 0x60], xmm0
141738423: movups xmm0, xmmword ptr [rbp + 0x228]
14173842a: movups xmmword ptr [rsi + 0x50], xmm0
14173842e: movups xmm0, xmmword ptr [rbp + 0x218]
141738435: movups xmmword ptr [rsi + 0x40], xmm0
141738439: movups xmm0, xmmword ptr [rbp + 0x1d8]
141738440: movups xmm1, xmmword ptr [rbp + 0x1e8]
141738447: movups xmm2, xmmword ptr [rbp + 0x1f8]
14173844e: movups xmm3, xmmword ptr [rbp + 0x208]
141738455: movups xmmword ptr [rsi + 0x30], xmm3
141738459: movups xmmword ptr [rsi + 0x20], xmm2
14173845d: movups xmmword ptr [rsi + 0x10], xmm1
141738461: movups xmmword ptr [rsi], xmm0
141738464: mov rax, rsi
141738467: add rsp, 0x428
14173846e: pop rbx
14173846f: pop rdi
141738470: pop rsi
141738471: pop rbp
141738472: ret 
141738473: mov ecx, 8
141738478: mov edx, 0x2a0
14173847d: call 0x1428d9000
141738482: mov ecx, 8
141738487: mov edx, 0x1c0
14173848c: call 0x1428d9000
141738491: ud2 
141738493: nop word ptr cs:[rax + rax]
1417384a0: mov qword ptr [rsp + 0x10], rdx
1417384a5: push rbp
1417384a6: push rsi
1417384a7: push rdi
1417384a8: push rbx
1417384a9: sub rsp, 0x28
1417384ad: lea rbp, [rdx + 0x80]
1417384b4: lea rcx, [rbp + 0x1d8]
1417384bb: call 0x1405dd9d0
1417384c0: nop 
1417384c1: add rsp, 0x28
1417384c5: pop rbx
1417384c6: pop rdi
1417384c7: pop rsi
1417384c8: pop rbp
1417384c9: ret 
1417384ca: nop word ptr [rax + rax]
1417384d0: mov qword ptr [rsp + 0x10], rdx
1417384d5: push rbp
1417384d6: push rsi
1417384d7: push rdi
1417384d8: push rbx
1417384d9: sub rsp, 0x28
1417384dd: lea rbp, [rdx + 0x80]
1417384e4: mov edx, 0x2a0
1417384e9: mov r8d, 8
1417384ef: mov rcx, qword ptr [rbp + 0x398]
1417384f6: call 0x140613c20
1417384fb: nop 
1417384fc: add rsp, 0x28
141738500: pop rbx
141738501: pop rdi
141738502: pop rsi
141738503: pop rbp
141738504: ret 
141738505: nop word ptr cs:[rax + rax]
141738510: mov qword ptr [rsp + 0x10], rdx
141738515: push rbp
141738516: push rsi
141738517: push rdi
141738518: push rbx
141738519: sub rsp, 0x28
14173851d: lea rbp, [rdx + 0x80]
141738524: lea rcx, [rbp + 0x168]
14173852b: call 0x140029880
141738530: nop 
141738531: add rsp, 0x28
141738535: pop rbx
141738536: pop rdi
141738537: pop rsi
141738538: pop rbp
141738539: ret 
14173853a: nop word ptr [rax + rax]
141738540: mov qword ptr [rsp + 0x10], rdx
141738545: push rbp
141738546: push rsi
141738547: push rdi
141738548: push rbx
141738549: sub rsp, 0x28
14173854d: lea rbp, [rdx + 0x80]
141738554: lea rcx, [rbp + 0xf8]
14173855b: call 0x140029880
141738560: nop 
141738561: add rsp, 0x28
141738565: pop rbx
141738566: pop rdi
141738567: pop rsi
141738568: pop rbp
141738569: ret 
14173856a: nop word ptr [rax + rax]
141738570: mov qword ptr [rsp + 0x10], rdx
141738575: push rbp
141738576: push rsi
141738577: push rdi
141738578: push rbx
141738579: sub rsp, 0x28
14173857d: lea rbp, [rdx + 0x80]
141738584: lea rcx, [rbp + 0x88]
14173858b: call 0x140029880
141738590: nop 
141738591: add rsp, 0x28
141738595: pop rbx
141738596: pop rdi
141738597: pop rsi
141738598: pop rbp
141738599: ret 
14173859a: nop word ptr [rax + rax]
1417385a0: mov qword ptr [rsp + 0x10], rdx
1417385a5: push rbp
1417385a6: push rsi
1417385a7: push rdi
1417385a8: push rbx
1417385a9: sub rsp, 0x28
1417385ad: lea rbp, [rdx + 0x80]
1417385b4: lea rcx, [rbp + 0x18]
1417385b8: call 0x140029880
1417385bd: nop 
1417385be: add rsp, 0x28
1417385c2: pop rbx
1417385c3: pop rdi
1417385c4: pop rsi
1417385c5: pop rbp
1417385c6: ret 
1417385c7: nop word ptr [rax + rax]
1417385d0: mov qword ptr [rsp + 0x10], rdx
1417385d5: push rbp
1417385d6: push rsi
1417385d7: push rdi
1417385d8: push rbx
1417385d9: sub rsp, 0x28
1417385dd: lea rbp, [rdx + 0x80]
1417385e4: lea rcx, [rbp - 0x58]
1417385e8: call 0x140029880
1417385ed: nop 
1417385ee: add rsp, 0x28
1417385f2: pop rbx
1417385f3: pop rdi
1417385f4: pop rsi
1417385f5: pop rbp
1417385f6: ret 
1417385f7: nop word ptr [rax + rax]
141738600: mov qword ptr [rsp + 0x10], rdx
141738605: push rbp
141738606: push rsi
141738607: push rdi
141738608: push rbx
141738609: sub rsp, 0x28
14173860d: lea rbp, [rdx + 0x80]
141738614: lea rcx, [rbp + 0x1d8]
14173861b: call 0x140029880
141738620: nop 
141738621: add rsp, 0x28
141738625: pop rbx
141738626: pop rdi
141738627: pop rsi
141738628: pop rbp
141738629: ret 
14173862a: int3 
14173862b: int3 
14173862c: int3 
14173862d: int3 
14173862e: int3 
14173862f: int3 
