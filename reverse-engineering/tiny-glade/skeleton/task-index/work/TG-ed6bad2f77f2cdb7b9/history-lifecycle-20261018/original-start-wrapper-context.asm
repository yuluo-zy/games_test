141734400: mov dword ptr [rsp + 0x28], eax
141734404: mov qword ptr [rsp + 0x30], 0x42
14173440d: mov qword ptr [rsp + 0x38], rsi
141734412: lea rax, [rip - 0x16f8949]
141734419: mov qword ptr [rsp + 0x40], rax
14173441e: lea rax, [rsp + 0x28]
141734423: mov qword ptr [rsp + 0x48], rax
141734428: lea rax, [rip - 0x162a59f]
14173442f: mov qword ptr [rsp + 0x50], rax
141734434: lea rax, [rip + 0x159864d]
14173443b: mov qword ptr [rsp + 0x58], rax
141734440: mov qword ptr [rsp + 0x60], 2
141734449: mov qword ptr [rsp + 0x78], 0
141734452: lea rax, [rsp + 0x38]
141734457: mov qword ptr [rsp + 0x68], rax
14173445c: mov qword ptr [rsp + 0x70], 2
141734465: lea rdx, [rip + 0x1598904]
14173446c: lea rcx, [rsp + 0x58]
141734471: call 0x1428d9390
141734476: lea rcx, [rip + 0x1598c93]
14173447d: lea r8, [rip + 0x1598cec]
141734484: mov edx, 0x59
141734489: call 0x1428d9330
14173448e: int3 
14173448f: int3 
141734490: push r15
141734492: push r14
141734494: push rsi
141734495: push rdi
141734496: push rbx
141734497: sub rsp, 0xb0
14173449e: mov esi, 1
1417344a3: lock xadd dword ptr [rdx + 0x5b0], esi
1417344ab: test byte ptr [rcx], 1
1417344ae: je 0x14173477e
1417344b4: mov rax, qword ptr [rcx + 8]
1417344b8: cmp qword ptr [rdx + 0x218], rax
1417344bf: jbe 0x141734656
1417344c5: mov r8, qword ptr [rdx + 0x210]
1417344cc: mov rax, qword ptr [r8 + rax*8]
1417344d0: test rax, rax
1417344d3: je 0x141734656
1417344d9: mov r8, qword ptr [rdx + 0x1e0]
1417344e0: not rax
1417344e3: lea rax, [rax + rax*2]
1417344e7: shl rax, 5
1417344eb: cmp qword ptr [r8 + rax + 0x20], 0
1417344f1: je 0x141734656
1417344f7: add r8, rax
1417344fa: mov rax, qword ptr [r8 + 0x10]
1417344fe: mov r9d, dword ptr [rdx + 0x5b0]
141734505: mov r10, qword ptr [rcx + 0x10]
141734509: mov r9, qword ptr [rdx + 0x218]
141734510: cmp r9, r10
141734513: jbe 0x1417346db
141734519: mov r11, qword ptr [rdx + 0x210]
141734520: mov rdi, qword ptr [r11 + r10*8]
141734524: test rdi, rdi
141734527: je 0x1417346db
14173452d: mov r10, qword ptr [rdx + 0x1e0]
141734534: not rdi
141734537: lea rdx, [rdi + rdi*2]
14173453b: shl rdx, 5
14173453f: cmp qword ptr [r10 + rdx + 0x20], 0
141734545: je 0x1417346db
14173454b: mov rdi, qword ptr [rcx + 0x18]
14173454f: cmp r9, rdi
141734552: jbe 0x1417346f9
141734558: mov r9, qword ptr [r11 + rdi*8]
14173455c: test r9, r9
14173455f: je 0x1417346f9
141734565: not r9
141734568: lea r11, [r9 + r9*2]
14173456c: shl r11, 5
141734570: cmp qword ptr [r10 + r11 + 0x20], 0
141734576: je 0x1417346f9
14173457c: mov edi, dword ptr [rcx + 0x158]
141734582: add rdx, r10
141734585: mov r9, qword ptr [rdx + 0x10]
141734589: add r11, r10
14173458c: lea rbx, [rdx + 0x5c]
141734590: add rdx, 0x58
141734594: lea r10, [r8 + 0x5c]
141734598: add r8, 0x58
14173459c: mov r14, qword ptr [r11 + 0x10]
1417345a0: lea r15, [r11 + 0x58]
1417345a4: add r11, 0x5c
1417345a8: mov qword ptr [rsp + 0x50], rax
1417345ad: mov qword ptr [rsp + 0x58], r8
1417345b2: mov qword ptr [rsp + 0x60], r10
1417345b7: mov dword ptr [rsp + 0x68], edi
1417345bb: mov dword ptr [rsp + 0x6c], esi
1417345bf: mov qword ptr [rsp + 0x70], r9
1417345c4: mov qword ptr [rsp + 0x78], rdx
1417345c9: mov qword ptr [rsp + 0x80], rbx
1417345d1: mov dword ptr [rsp + 0x88], edi
1417345d8: mov dword ptr [rsp + 0x8c], esi
1417345df: mov qword ptr [rsp + 0x90], r14
1417345e7: mov qword ptr [rsp + 0x98], r15
1417345ef: mov qword ptr [rsp + 0xa0], r11
1417345f7: mov dword ptr [rsp + 0xa8], edi
1417345fe: mov dword ptr [rsp + 0xac], esi
141734605: cmp dword ptr [r14], 0xd
141734609: je 0x14173463f
14173460b: cmp byte ptr [r9 + 0x18], 2
141734610: jb 0x14173463f
141734612: mov rdx, qword ptr [r9 + 0x20]
141734616: test byte ptr [rdx + 0xb9], 1
14173461d: je 0x14173463f
14173461f: cmp byte ptr [r9 + 0x19], 3
141734624: jne 0x14173463f
141734626: lea rdx, [rsp + 0x90]
14173462e: mov dword ptr [r10], esi
141734631: mov rdi, rcx
141734634: mov rcx, rax
141734637: call 0x140a86270
14173463c: mov rcx, rdi
14173463f: mov dword ptr [rcx + 0x158], esi
141734645: xor eax, eax
141734647: add rsp, 0xb0
14173464e: pop rbx
14173464f: pop rdi
141734650: pop rsi
141734651: pop r14
141734653: pop r15
141734655: ret 
141734656: add rcx, 0xc0
14173465d: lea rax, [rip + 0x159889c]
141734664: mov qword ptr [rsp + 0x20], rax
141734669: mov qword ptr [rsp + 0x28], 0x29
141734672: mov qword ptr [rsp + 0x30], rcx
141734677: lea rax, [rip - 0x16f8bae]
14173467e: mov qword ptr [rsp + 0x38], rax
141734683: lea rax, [rsp + 0x20]
141734688: mov qword ptr [rsp + 0x40], rax
14173468d: lea rax, [rip - 0x162a804]
141734694: mov qword ptr [rsp + 0x48], rax
141734699: lea rax, [rip + 0x15983e8]
1417346a0: mov qword ptr [rsp + 0x50], rax
1417346a5: mov qword ptr [rsp + 0x58], 2
1417346ae: mov qword ptr [rsp + 0x70], 0
1417346b7: lea rax, [rsp + 0x30]
1417346bc: mov qword ptr [rsp + 0x60], rax
1417346c1: mov qword ptr [rsp + 0x68], 2
1417346ca: lea rdx, [rip + 0x159869f]
1417346d1: lea rcx, [rsp + 0x50]
1417346d6: call 0x1428d9390
