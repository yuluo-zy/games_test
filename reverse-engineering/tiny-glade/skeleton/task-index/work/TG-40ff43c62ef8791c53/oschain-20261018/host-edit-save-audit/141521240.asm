141521240: push rbp
141521241: push r15
141521243: push r14
141521245: push rsi
141521246: push rdi
141521247: push rbx
141521248: sub rsp, 0x248
14152124f: lea rbp, [rsp + 0x80]
141521257: mov qword ptr [rbp + 0x1c0], 0xfffffffffffffffe
141521262: mov rsi, rcx
141521265: lea rdi, [rcx + 0x38]
141521269: lea rax, [rcx + 0x5a8]
141521270: mov qword ptr [rbp + 0x1a0], rdi
141521277: mov qword ptr [rbp + 0x1a8], rax
14152127e: cmp qword ptr [rcx + 0x88], 0
141521286: movabs rax, 0xf7c86db5568a7d07
141521290: movabs rcx, 0x4ab328be485143c6
14152129a: je 0x141521347
1415212a0: mov rdx, qword ptr [rsi + 0x70]
1415212a4: mov r8, qword ptr [rsi + 0x78]
1415212a8: mov r9, r8
1415212ab: and r9, rax
1415212ae: xor r10d, r10d
1415212b1: movdqa xmm0, xmmword ptr [rip + 0x142f177]
1415212b9: pcmpeqd xmm1, xmm1
1415212bd: movdqu xmm2, xmmword ptr [rdx + r9]
1415212c3: movdqa xmm3, xmm2
1415212c7: pcmpeqb xmm3, xmm0
1415212cb: pmovmskb r11d, xmm3
1415212d0: test r11d, r11d
1415212d3: jne 0x14152130d
1415212d5: pcmpeqb xmm2, xmm1
1415212d9: pmovmskb r11d, xmm2
1415212de: test r11d, r11d
1415212e1: jne 0x141521347
1415212e3: add r9, r10
1415212e6: add r9, 0x10
1415212ea: add r10, 0x10
1415212ee: and r9, r8
1415212f1: jmp 0x1415212bd
1415212f3: nop word ptr cs:[rax + rax]
141521300: lea ebx, [r11 - 1]
141521304: and bx, r11w
141521308: mov r11d, ebx
14152130b: je 0x1415212d5
14152130d: tzcnt ebx, r11d
141521312: add rbx, r9
141521315: and rbx, r8
141521318: neg rbx
14152131b: lea rbx, [rbx + rbx*2]
14152131f: cmp qword ptr [rdx + rbx*8 - 0x18], rcx
141521324: jne 0x141521300
141521326: cmp qword ptr [rdx + rbx*8 - 0x10], rax
14152132b: jne 0x141521300
14152132d: lea rax, [rdx + rbx*8]
141521331: mov rbx, qword ptr [rax - 8]
141521335: cmp qword ptr [rsi + 0x218], rbx
14152133c: ja 0x141521646
141521342: jmp 0x141521674
141521347: movzx edx, byte ptr [rsi + 0x98]
14152134e: mov rdx, qword ptr [rsi + 0xd8]
141521355: mov r9, qword ptr [rsi + 0xe0]
14152135c: mov r10, r9
14152135f: and r10, rax
141521362: xor r11d, r11d
141521365: movdqa xmm0, xmmword ptr [rip + 0x142f0c3]
14152136d: pcmpeqd xmm1, xmm1
141521371: movdqu xmm2, xmmword ptr [rdx + r10]
141521377: movdqa xmm3, xmm2
14152137b: pcmpeqb xmm3, xmm0
14152137f: pmovmskb ebx, xmm3
141521383: test ebx, ebx
141521385: jne 0x1415213bd
141521387: pcmpeqb xmm2, xmm1
14152138b: pmovmskb r8d, xmm2
141521390: test r8d, r8d
141521393: jne 0x141521487
141521399: add r10, r11
14152139c: add r10, 0x10
1415213a0: add r11, 0x10
1415213a4: and r10, r9
1415213a7: jmp 0x141521371
1415213a9: nop dword ptr [rax]
1415213b0: lea r8d, [rbx - 1]
1415213b4: and r8w, bx
1415213b8: mov ebx, r8d
1415213bb: je 0x141521387
1415213bd: tzcnt r14d, ebx
1415213c2: add r14, r10
1415213c5: and r14, r9
1415213c8: mov r8, r14
1415213cb: neg r8
1415213ce: lea r15, [r14*8]
1415213d6: shl r8, 7
1415213da: sub r8, r15
1415213dd: cmp qword ptr [rdx + r8 - 0x88], rcx
1415213e5: jne 0x1415213b0
1415213e7: cmp qword ptr [rdx + r8 - 0x80], rax
1415213ec: jne 0x1415213b0
1415213ee: mov r10, r14
1415213f1: shl r10, 7
1415213f5: lea r11, [r10 + r14*8]
1415213f9: sar r11, 3
1415213fd: movabs r10, 0xf0f0f0f0f0f0f0f1
141521407: imul r10, r11
14152140b: lea r11, [r10 - 0x10]
14152140f: and r11, r9
141521412: movdqu xmm0, xmmword ptr [rdx + r11]
141521418: pcmpeqd xmm1, xmm1
14152141c: pcmpeqb xmm0, xmm1
141521420: pmovmskb ebx, xmm0
141521424: movdqu xmm0, xmmword ptr [rdx + r10]
14152142a: pcmpeqb xmm0, xmm1
14152142e: pmovmskb r9d, xmm0
141521433: test ebx, ebx
141521435: je 0x141521440
141521437: bsr bx, bx
14152143b: xor ebx, 0xf
14152143e: jmp 0x141521444
141521440: mov bx, 0x10
141521444: add r8, rdx
141521447: or r9d, 0x10000
14152144e: tzcnt r9d, r9d
141521453: add ebx, r9d
141521456: mov r9b, 0x80
141521459: cmp bx, 0xf
14152145d: ja 0x141521469
14152145f: inc qword ptr [rsi + 0xe8]
141521466: mov r9b, 0xff
141521469: mov byte ptr [rdx + r10], r9b
14152146d: mov byte ptr [rdx + r11 + 0x10], r9b
141521472: dec qword ptr [rsi + 0xf0]
141521479: mov rdx, qword ptr [r8 - 0x78]
14152147d: cmp rdx, 2
141521481: jne 0x1415215cd
141521487: mov rbx, qword ptr [rsi + 0x5a8]
14152148e: lea r14, [rbx + 1]
141521492: mov qword ptr [rsi + 0x5a8], r14
141521499: movabs rdx, 0x8000000000000000
1415214a3: mov qword ptr [rbp + 0x178], rdx
1415214aa: lea rdx, [rip + 0x1712db4]
1415214b1: mov qword ptr [rbp + 0x180], rdx
1415214b8: mov qword ptr [rbp + 0x188], 0x59
1415214c3: mov word ptr [rbp + 0x198], 0x100
1415214cc: mov qword ptr [rbp + 0x140], 1
1415214d7: mov qword ptr [rbp + 0x148], rcx
1415214de: mov qword ptr [rbp + 0x150], rax
1415214e5: mov qword ptr [rbp + 0x168], 8
1415214f0: mov qword ptr [rbp + 0x170], 0x48
1415214fb: lea rax, [rip - 0x13d7b22]
141521502: mov qword ptr [rbp + 0x190], rax
141521509: mov byte ptr [rbp + 0x19a], 1
141521510: mov qword ptr [rbp + 0x158], 0
14152151b: lea rcx, [rbp - 0x60]
14152151f: lea r8, [rbp + 0x140]
141521526: mov rdx, rbx
141521529: call 0x140d3f870
14152152e: cmp qword ptr [rsi + 0x48], r14
141521532: jae 0x14152154d
141521534: mov byte ptr [rbp + 0x1bf], 1
14152153b: lea r8, [rip + 0x174a8ae]
141521542: mov rcx, rdi
141521545: mov rdx, r14
141521548: call 0x1412f4340
14152154d: mov r14, qword ptr [rsi + 0x40]
141521551: imul r15, rbx, 0xd0
141521558: lea rax, [r14 + r15]
14152155c: mov qword ptr [rbp + 0x1b0], rax
141521563: lea rcx, [rbp + 0x70]
141521567: lea rdx, [rbp - 0x60]
14152156b: mov r8d, 0xd0
141521571: call 0x142923c00
141521576: cmp dword ptr [r14 + r15], 2
14152157b: je 0x141521589
14152157d: mov rcx, qword ptr [rbp + 0x1b0]
141521584: call 0x1405f0b50
141521589: lea rdx, [rbp + 0x70]
14152158d: mov r8d, 0xd0
141521593: mov rcx, qword ptr [rbp + 0x1b0]
14152159a: call 0x142923c00
14152159f: lea rcx, [rsi + 0x70]
1415215a3: movabs rdx, 0x4ab328be485143c6
1415215ad: movabs r8, 0xf7c86db5568a7d07
1415215b7: mov r9, rbx
1415215ba: call 0x140082c20
1415215bf: cmp qword ptr [rsi + 0x218], rbx
1415215c6: ja 0x141521646
1415215c8: jmp 0x141521674
1415215cd: movups xmm0, xmmword ptr [r8 - 0x10]
1415215d2: movups xmmword ptr [rbp + 0xd8], xmm0
1415215d9: movups xmm0, xmmword ptr [r8 - 0x20]
1415215de: movups xmmword ptr [rbp + 0xc8], xmm0
1415215e5: movups xmm0, xmmword ptr [r8 - 0x30]
1415215ea: movups xmmword ptr [rbp + 0xb8], xmm0
1415215f1: movdqu xmm0, xmmword ptr [r8 - 0x70]
1415215f7: movdqu xmm1, xmmword ptr [r8 - 0x60]
1415215fd: movdqu xmm2, xmmword ptr [r8 - 0x50]
141521603: movdqu xmm3, xmmword ptr [r8 - 0x40]
141521609: movdqu xmmword ptr [rbp + 0xa8], xmm3
141521611: movdqu xmmword ptr [rbp + 0x98], xmm2
141521619: movdqu xmmword ptr [rbp + 0x88], xmm1
141521621: movdqu xmmword ptr [rbp + 0x78], xmm0
141521626: mov qword ptr [rbp + 0x70], rdx
14152162a: lea rcx, [rbp + 0x70]
14152162e: lea rdx, [rbp + 0x1a0]
141521635: call 0x140d3fa40
14152163a: mov rbx, rax
14152163d: cmp qword ptr [rsi + 0x218], rbx
141521644: jbe 0x141521674
141521646: mov rax, qword ptr [rsi + 0x210]
14152164d: mov rax, qword ptr [rax + rbx*8]
141521651: test rax, rax
141521654: je 0x141521674
141521656: mov rcx, qword ptr [rsi + 0x1e0]
14152165d: not rax
141521660: lea rax, [rax + rax*2]
141521664: shl rax, 5
141521668: cmp qword ptr [rcx + rax + 0x20], 0
14152166e: jne 0x14152174b
141521674: lea r14, [rsi + 0x1d8]
14152167b: mov qword ptr [rbp + 0x70], 0
141521683: mov qword ptr [rbp + 0x78], 8
14152168b: pxor xmm0, xmm0
14152168f: movdqu xmmword ptr [rbp + 0x80], xmm0
141521697: mov qword ptr [rbp + 0x90], 0
1415216a2: mov qword ptr [rbp + 0x98], 8
1415216ad: movdqu xmmword ptr [rbp + 0xa0], xmm0
1415216b5: mov qword ptr [rbp + 0xb0], 0
1415216c0: mov r15d, dword ptr [rsi + 0x5b0]
1415216c7: mov rcx, rsi
1415216ca: call 0x140cf8250
1415216cf: lea rax, [rsi + 0xf8]
1415216d6: mov qword ptr [rbp + 0x140], rbx
1415216dd: mov qword ptr [rbp - 0x60], rdi
1415216e1: lea rcx, [rbp + 0x140]
1415216e8: mov qword ptr [rbp - 0x58], rcx
1415216ec: mov qword ptr [rbp - 0x50], rax
1415216f0: lea r8, [rbp - 0x60]
1415216f4: mov rcx, r14
1415216f7: mov rdx, rbx
1415216fa: call 0x14082b900
1415216ff: mov rdi, rax
141521702: cmp qword ptr [rax + 0x20], 0
141521707: je 0x141521719
141521709: lea r8, [rbp + 0x70]
14152170d: mov rcx, rdi
141521710: xor edx, edx
141521712: call 0x140cff670
141521717: jmp 0x141521747
141521719: cmp qword ptr [rdi + 0x18], 0
14152171e: je 0x1415217bd
141521724: xor ecx, ecx
141521726: lea rax, [rcx + 1]
14152172a: mov qword ptr [rdi + 0x20], rax
14152172e: mov r8, qword ptr [rdi + 8]
141521732: imul rcx, r8
141521736: add rcx, qword ptr [rdi + 0x10]
14152173a: lea rdx, [rbp + 0x70]
14152173e: call 0x142923c00
141521743: mov dword ptr [rdi + 0x58], r15d
141521747: mov dword ptr [rdi + 0x5c], r15d
14152174b: lea r8, [rip + 0x174a60e]
141521752: lea rcx, [rbp + 0x70]
141521756: mov rdx, rsi
141521759: call 0x140a500f0
14152175e: mov eax, dword ptr [rbp + 0x8c]
141521764: mov rcx, qword ptr [rbp + 0x80]
14152176b: mov dword ptr [rcx], eax
14152176d: mov rsi, qword ptr [rbp + 0x70]
141521771: mov rdi, qword ptr [rsi + 0x10]
141521775: cmp rdi, qword ptr [rsi]
141521778: jne 0x141521789
14152177a: lea rdx, [rip + 0x174a5f7]
141521781: mov rcx, rsi
141521784: call 0x140672ed0
141521789: mov rax, qword ptr [rsi + 8]
14152178d: lea rcx, [rdi + rdi*2]
141521791: lea rdx, [rip - 0xfbbe68]
141521798: mov qword ptr [rax + rcx*8], rdx
14152179c: mov qword ptr [rax + rcx*8 + 8], rbx
1415217a1: mov byte ptr [rax + rcx*8 + 0x10], 0
1415217a6: inc rdi
1415217a9: mov qword ptr [rsi + 0x10], rdi
1415217ad: add rsp, 0x248
1415217b4: pop rbx
1415217b5: pop rdi
1415217b6: pop rsi
1415217b7: pop r14
1415217b9: pop r15
1415217bb: pop rbp
1415217bc: ret 
1415217bd: mov edx, 1
1415217c2: mov rcx, rdi
1415217c5: call 0x1428cec40
1415217ca: mov rcx, qword ptr [rdi + 0x20]
1415217ce: jmp 0x141521726
1415217d3: nop word ptr cs:[rax + rax]
1415217e0: mov qword ptr [rsp + 0x10], rdx
1415217e5: push rbp
1415217e6: push r15
1415217e8: push r14
1415217ea: push rsi
1415217eb: push rdi
1415217ec: push rbx
1415217ed: sub rsp, 0x28
1415217f1: lea rbp, [rdx + 0x80]
1415217f8: lea rdx, [rbp + 0x70]
1415217fc: mov r8d, 0xd0
141521802: mov rcx, qword ptr [rbp + 0x1b0]
141521809: call 0x142923c00
14152180e: mov byte ptr [rbp + 0x1bf], 0
141521815: add rsp, 0x28
141521819: pop rbx
14152181a: pop rdi
14152181b: pop rsi
14152181c: pop r14
14152181e: pop r15
141521820: pop rbp
141521821: ret 
141521822: nop word ptr cs:[rax + rax]
141521830: mov qword ptr [rsp + 0x10], rdx
141521835: push rbp
141521836: push r15
141521838: push r14
14152183a: push rsi
14152183b: push rdi
14152183c: push rbx
14152183d: sub rsp, 0x28
141521841: lea rbp, [rdx + 0x80]
141521848: cmp byte ptr [rbp + 0x1bf], 0
14152184f: jne 0x14152185e
141521851: add rsp, 0x28
141521855: pop rbx
141521856: pop rdi
141521857: pop rsi
141521858: pop r14
14152185a: pop r15
14152185c: pop rbp
14152185d: ret 
14152185e: lea rcx, [rbp - 0x60]
141521862: call 0x1405f0b50
141521867: jmp 0x141521851
