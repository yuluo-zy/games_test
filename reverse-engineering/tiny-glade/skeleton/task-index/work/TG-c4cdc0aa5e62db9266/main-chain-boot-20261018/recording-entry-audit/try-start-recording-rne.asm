140a86510: push rbp
140a86511: push rsi
140a86512: push rdi
140a86513: sub rsp, 0x70
140a86517: lea rbp, [rsp + 0x70]
140a8651c: mov qword ptr [rbp - 8], 0xfffffffffffffffe
140a86524: mov rsi, rcx
140a86527: mov rcx, qword ptr [rdx]
140a8652a: lea rdx, [rip + 0x2051eb7]
140a86531: call 0x140a825e0
140a86536: test al, al
140a86538: je 0x140a86658
140a8653e: xor eax, eax
140a86540: cmp rax, qword ptr [rsi + 0x30]
140a86544: jno 0x140a8665a
140a8654a: mov qword ptr [rbp - 0x28], 0
140a86552: mov qword ptr [rbp - 0x20], 8
140a8655a: mov qword ptr [rbp - 0x18], 0
140a86562: movzx eax, byte ptr [rip + 0x2bae6a0]
140a86569: mov ecx, 0x40
140a8656e: mov edx, 8
140a86573: call 0x140613c10
140a86578: mov qword ptr [rbp - 0x10], rax
140a8657c: test rax, rax
140a8657f: je 0x140a86662
140a86585: xor ecx, ecx
140a86587: call 0x1400d9b20
140a8658c: test rax, rax
140a8658f: je 0x140a86673
140a86595: movups xmm0, xmmword ptr [rax]
140a86598: inc qword ptr [rax]
140a8659b: mov rdi, qword ptr [rbp - 0x10]
140a8659f: mov qword ptr [rdi], 1
140a865a6: mov qword ptr [rdi + 8], 1
140a865ae: movups xmm1, xmmword ptr [rip + 0x206aebb]
140a865b5: movups xmmword ptr [rdi + 0x10], xmm1
140a865b9: movups xmm1, xmmword ptr [rip + 0x206aec0]
140a865c0: movups xmmword ptr [rdi + 0x20], xmm1
140a865c4: movups xmmword ptr [rdi + 0x30], xmm0
140a865c8: mov qword ptr [rbp - 0x30], rdi
140a865cc: movzx eax, byte ptr [rip + 0x2bae636]
140a865d3: mov ecx, 0x60
140a865d8: mov edx, 8
140a865dd: call 0x140613c10
140a865e2: test rax, rax
140a865e5: je 0x140a86681
140a865eb: lea rcx, [rsi + 0x30]
140a865ef: mov qword ptr [rax], 1
140a865f6: mov qword ptr [rax + 8], 1
140a865fe: mov qword ptr [rax + 0x10], 0
140a86606: mov qword ptr [rax + 0x20], 0
140a8660e: mov qword ptr [rax + 0x30], 0
140a86616: mov qword ptr [rax + 0x38], 4
140a8661e: xorps xmm0, xmm0
140a86621: movups xmmword ptr [rax + 0x40], xmm0
140a86625: mov qword ptr [rax + 0x50], 4
140a8662d: mov qword ptr [rax + 0x58], 0
140a86635: mov rdx, qword ptr [rbp - 0x18]
140a86639: mov qword ptr [rbp - 0x40], rdx
140a8663d: movups xmm0, xmmword ptr [rbp - 0x28]
140a86641: movaps xmmword ptr [rbp - 0x50], xmm0
140a86645: mov qword ptr [rcx + 0x10], rdx
140a86649: movups xmmword ptr [rcx], xmm0
140a8664c: mov qword ptr [rsi + 0x48], rdi
140a86650: mov qword ptr [rsi + 0x50], rax
140a86654: mov al, 1
140a86656: jmp 0x140a8665a
140a86658: xor eax, eax
140a8665a: add rsp, 0x70
140a8665e: pop rdi
140a8665f: pop rsi
140a86660: pop rbp
140a86661: ret 
140a86662: mov ecx, 8
140a86667: mov edx, 0x40
140a8666c: call 0x1428d9000
140a86671: jmp 0x140a86690
140a86673: lea rcx, [rip + 0x21b4426]
140a8667a: call 0x1428d80d0
140a8667f: jmp 0x140a86690
140a86681: mov ecx, 8
140a86686: mov edx, 0x60
140a8668b: call 0x1428d9000
140a86690: ud2 
140a86692: nop word ptr cs:[rax + rax]
140a866a0: mov qword ptr [rsp + 0x10], rdx
140a866a5: push rbp
140a866a6: push rsi
140a866a7: push rdi
140a866a8: sub rsp, 0x20
140a866ac: lea rbp, [rdx + 0x70]
140a866b0: mov edx, 0x40
140a866b5: mov r8d, 8
140a866bb: mov rcx, qword ptr [rbp - 0x10]
140a866bf: call 0x140613c20
140a866c4: nop 
140a866c5: add rsp, 0x20
140a866c9: pop rdi
140a866ca: pop rsi
140a866cb: pop rbp
140a866cc: ret 
140a866cd: nop dword ptr [rax]
140a866d0: mov qword ptr [rsp + 0x10], rdx
140a866d5: push rbp
140a866d6: push rsi
140a866d7: push rdi
140a866d8: sub rsp, 0x20
140a866dc: lea rbp, [rdx + 0x70]
140a866e0: lea rcx, [rbp - 0x28]
140a866e4: call 0x140894ab0
140a866e9: nop 
140a866ea: add rsp, 0x20
140a866ee: pop rdi
140a866ef: pop rsi
140a866f0: pop rbp
140a866f1: ret 
140a866f2: nop word ptr cs:[rax + rax]
140a86700: mov qword ptr [rsp + 0x10], rdx
140a86705: push rbp
140a86706: push rsi
140a86707: push rdi
140a86708: sub rsp, 0x20
140a8670c: lea rbp, [rdx + 0x70]
140a86710: mov rax, qword ptr [rbp - 0x10]
140a86714: lock dec qword ptr [rax]
140a86718: jne 0x140a86723
140a8671a: lea rcx, [rbp - 0x30]
140a8671e: call 0x140b5cb60
140a86723: nop 
140a86724: add rsp, 0x20
140a86728: pop rdi
140a86729: pop rsi
140a8672a: pop rbp
140a8672b: ret 
