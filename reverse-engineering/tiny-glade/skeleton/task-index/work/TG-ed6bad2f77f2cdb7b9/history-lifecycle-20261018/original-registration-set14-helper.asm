141738630: push rbp
141738631: push rsi
141738632: push rdi
141738633: push rbx
141738634: sub rsp, 0x208
14173863b: lea rbp, [rsp + 0x80]
141738643: mov qword ptr [rbp + 0x180], 0xfffffffffffffffe
14173864e: mov ebx, edx
141738650: mov rsi, rcx
141738653: xorps xmm0, xmm0
141738656: movups xmmword ptr [rbp + 0x60], xmm0
14173865a: movups xmmword ptr [rbp + 0x78], xmm0
14173865e: movups xmmword ptr [rbp + 0x90], xmm0
141738665: movups xmmword ptr [rbp + 0xa8], xmm0
14173866c: movups xmmword ptr [rbp + 0xc0], xmm0
141738673: mov dword ptr [rbp + 0xd0], 0
14173867d: movups xmmword ptr [rbp + 0xf8], xmm0
141738684: movups xmmword ptr [rbp + 0x110], xmm0
14173868b: movups xmmword ptr [rbp + 0x128], xmm0
141738692: movups xmmword ptr [rbp + 0x140], xmm0
141738699: movups xmmword ptr [rbp + 0x158], xmm0
1417386a0: mov dword ptr [rbp + 0x168], 0
1417386aa: mov qword ptr [rbp + 0x18], 0
1417386b2: mov qword ptr [rbp + 0x40], 0
1417386ba: mov qword ptr [rbp + 0x48], 8
1417386c2: mov qword ptr [rbp + 0x50], 0
1417386ca: mov qword ptr [rbp + 0x58], 0x10
1417386d2: mov qword ptr [rbp + 0x70], 0x10
1417386da: mov qword ptr [rbp + 0x88], 0x10
1417386e5: mov qword ptr [rbp + 0xa0], 0x10
1417386f0: mov qword ptr [rbp + 0xb8], 0x10
1417386fb: movabs rax, 0x8000000000000000
141738705: mov qword ptr [rbp + 0xd8], rax
14173870c: lea rax, [rip + 0x1594dfe]
141738713: mov qword ptr [rbp + 0xe0], rax
14173871a: mov qword ptr [rbp + 0xe8], 0x4a
141738725: mov qword ptr [rbp + 0xf0], 0x10
141738730: mov qword ptr [rbp + 0x108], 0x10
14173873b: mov qword ptr [rbp + 0x120], 0x10
141738746: mov qword ptr [rbp + 0x138], 0x10
141738751: mov qword ptr [rbp + 0x150], 0x10
14173875c: mov dword ptr [rbp + 0x170], 0
141738766: mov word ptr [rbp + 0x174], 1
14173876f: mov dword ptr [rbp + 0x178], 0
141738779: movzx eax, byte ptr [rip + 0x1efc489]
141738780: mov ecx, 0x168
141738785: mov edx, 8
14173878a: call 0x140613c10
14173878f: test rax, rax
141738792: je 0x14173882f
141738798: mov rdi, rax
14173879b: lea rdx, [rbp + 0x18]
14173879f: mov r8d, 0x168
1417387a5: mov rcx, rax
1417387a8: call 0x142923c00
1417387ad: lea r8, [rip + 0x1594b8c]
1417387b4: lea rcx, [rbp - 0x58]
1417387b8: mov rdx, rdi
1417387bb: call 0x140ce4b10
1417387c0: mov byte ptr [rbp + 0x18], bl
1417387c3: mov rcx, qword ptr [rip + 0x1ef3ba6]
1417387ca: lea r8, [rip + 0x152758f]
1417387d1: lea rdx, [rbp + 0x18]
1417387d5: call 0x140b218a0
1417387da: mov r8, rdx
1417387dd: lea rcx, [rbp - 0x58]
1417387e1: mov rdx, rax
1417387e4: call 0x141453000
1417387e9: movups xmm0, xmmword ptr [rbp + 8]
1417387ed: movups xmmword ptr [rsi + 0x60], xmm0
1417387f1: movups xmm0, xmmword ptr [rbp - 8]
1417387f5: movups xmmword ptr [rsi + 0x50], xmm0
1417387f9: movups xmm0, xmmword ptr [rbp - 0x18]
1417387fd: movups xmmword ptr [rsi + 0x40], xmm0
141738801: movups xmm0, xmmword ptr [rbp - 0x58]
141738805: movups xmm1, xmmword ptr [rbp - 0x48]
141738809: movups xmm2, xmmword ptr [rbp - 0x38]
14173880d: movups xmm3, xmmword ptr [rbp - 0x28]
141738811: movups xmmword ptr [rsi + 0x30], xmm3
141738815: movups xmmword ptr [rsi + 0x20], xmm2
141738819: movups xmmword ptr [rsi + 0x10], xmm1
14173881d: movups xmmword ptr [rsi], xmm0
141738820: mov rax, rsi
141738823: add rsp, 0x208
14173882a: pop rbx
14173882b: pop rdi
14173882c: pop rsi
14173882d: pop rbp
14173882e: ret 
14173882f: mov ecx, 8
141738834: mov edx, 0x168
141738839: call 0x1428d9000
14173883e: ud2 
141738840: mov qword ptr [rsp + 0x10], rdx
141738845: push rbp
141738846: push rsi
141738847: push rdi
141738848: push rbx
141738849: sub rsp, 0x28
14173884d: lea rbp, [rdx + 0x80]
141738854: lea rcx, [rbp + 0x18]
141738858: call 0x1405e6f10
14173885d: nop 
14173885e: add rsp, 0x28
141738862: pop rbx
141738863: pop rdi
141738864: pop rsi
141738865: pop rbp
141738866: ret 
141738867: nop word ptr [rax + rax]
141738870: mov qword ptr [rsp + 0x10], rdx
141738875: push rbp
141738876: push rsi
141738877: push rdi
141738878: push rbx
141738879: sub rsp, 0x28
14173887d: lea rbp, [rdx + 0x80]
141738884: lea rcx, [rbp - 0x58]
141738888: call 0x140029880
14173888d: nop 
