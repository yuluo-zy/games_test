140b760d0: push r14
140b760d2: push rsi
140b760d3: push rdi
140b760d4: push rbx
140b760d5: sub rsp, 0x38
140b760d9: mov rsi, rdx
140b760dc: mov rdi, rcx
140b760df: mov rbx, qword ptr [r8]
140b760e2: lea r14, [rsp + 0x28]
140b760e7: jmp 0x140b760f3
140b760e9: nop dword ptr [rax]
140b760f0: dec rsi
140b760f3: movzx ecx, word ptr [rdi + 0x18]
140b760f7: test cx, cx
140b760fa: je 0x140b76110
140b760fc: mov rax, qword ptr [rdi]
140b760ff: jmp 0x140b76143
140b76101: nop word ptr cs:[rax + rax]
140b76110: test rsi, rsi
140b76113: je 0x140b761a7
140b76119: mov rax, qword ptr [rdi]
140b7611c: mov rdx, qword ptr [rdi + 8]
140b76120: movdqa xmm0, xmmword ptr [rdx]
140b76124: pmovmskb ecx, xmm0
140b76128: add rax, -0x200
140b7612e: add rdx, 0x10
140b76132: cmp ecx, 0xffff
140b76138: je 0x140b76120
140b7613a: not ecx
140b7613c: mov qword ptr [rdi], rax
140b7613f: mov qword ptr [rdi + 8], rdx
140b76143: lea edx, [rcx - 1]
140b76146: tzcnt r8d, ecx
140b7614b: and edx, ecx
140b7614d: mov word ptr [rdi + 0x18], dx
140b76151: shl r8d, 5
140b76155: sub rax, r8
140b76158: mov rdx, qword ptr [rax - 0x20]
140b7615c: mov r8, qword ptr [rax - 0x18]
140b76160: movzx ecx, byte ptr [rax - 8]
140b76164: mov byte ptr [rax - 8], 0
140b76168: mov r9, qword ptr [rax - 0x10]
140b7616c: lock inc qword ptr [r9]
140b76170: jle 0x140b761b1
140b76172: mov byte ptr [rsp + 0x20], cl
140b76176: mov rcx, rbx
140b76179: call 0x14152b100
140b7617e: mov qword ptr [rsp + 0x28], rax
140b76183: mov byte ptr [rsp + 0x30], dl
140b76187: cmp dl, 2
140b7618a: je 0x140b760f0
140b76190: lock dec qword ptr [rax]
140b76194: jne 0x140b760f0
140b7619a: mov rcx, r14
140b7619d: call 0x140b5cad0
140b761a2: jmp 0x140b760f0
140b761a7: add rsp, 0x38
140b761ab: pop rbx
140b761ac: pop rdi
140b761ad: pop rsi
140b761ae: pop r14
140b761b0: ret 
140b761b1: ud2 
