140b66830: push rbp
140b66831: push rsi
140b66832: push rbx
140b66833: sub rsp, 0xd0
140b6683a: lea rbp, [rsp + 0x80]
140b66842: mov qword ptr [rbp + 0x48], 0xfffffffffffffffe
140b6684a: mov qword ptr [rbp + 0x38], r8
140b6684e: mov bl, dl
140b66850: mov rsi, rcx
140b66853: add rcx, 0x38
140b66857: mov byte ptr [rbp + 0x47], 1
140b6685b: movabs rdx, 0x8e1bd251962ba784
140b66865: movabs r8, 0x6a782374c0e6529b
140b6686f: call 0x1400dd6f0
140b66874: test rax, rax
140b66877: je 0x140b66920
140b6687d: cmp qword ptr [rsi + 0x218], rdx
140b66884: jbe 0x140b66920
140b6688a: mov rax, qword ptr [rsi + 0x210]
140b66891: mov rcx, qword ptr [rax + rdx*8]
140b66895: test rcx, rcx
140b66898: je 0x140b66920
140b6689e: mov rax, qword ptr [rsi + 0x1e0]
140b668a5: not rcx
140b668a8: lea rcx, [rcx + rcx*2]
140b668ac: shl rcx, 5
140b668b0: cmp qword ptr [rax + rcx + 0x20], 0
140b668b6: je 0x140b66920
140b668b8: add rax, rcx
140b668bb: mov rcx, qword ptr [rax + 0x10]
140b668bf: mov edx, dword ptr [rsi + 0x5b0]
140b668c5: mov dword ptr [rax + 0x5c], edx
140b668c8: mov rax, qword ptr [rbp + 0x38]
140b668cc: movups xmm0, xmmword ptr [rax + 0x60]
140b668d0: movaps xmmword ptr [rbp], xmm0
140b668d4: movups xmm0, xmmword ptr [rax + 0x50]
140b668d8: movaps xmmword ptr [rbp - 0x10], xmm0
140b668dc: movups xmm0, xmmword ptr [rax + 0x40]
140b668e0: movaps xmmword ptr [rbp - 0x20], xmm0
140b668e4: movups xmm0, xmmword ptr [rax]
140b668e7: movups xmm1, xmmword ptr [rax + 0x10]
140b668eb: movups xmm2, xmmword ptr [rax + 0x20]
140b668ef: movups xmm3, xmmword ptr [rax + 0x30]
140b668f3: movaps xmmword ptr [rbp - 0x30], xmm3
140b668f7: movaps xmmword ptr [rbp - 0x40], xmm2
140b668fb: movaps xmmword ptr [rbp - 0x50], xmm1
140b668ff: movaps xmmword ptr [rbp - 0x60], xmm0
140b66903: mov byte ptr [rbp + 0x47], 0
140b66907: lea r8, [rbp - 0x60]
140b6690b: mov edx, ebx
140b6690d: call 0x1416a02f0
140b66912: mov rax, rsi
140b66915: add rsp, 0xd0
140b6691c: pop rbx
140b6691d: pop rsi
140b6691e: pop rbp
140b6691f: ret 
140b66920: lea rax, [rip + 0x1f8c399]
140b66927: mov qword ptr [rbp + 0x18], rax
140b6692b: mov qword ptr [rbp + 0x20], 0x27
140b66933: lea rax, [rbp + 0x18]
140b66937: mov qword ptr [rbp + 0x28], rax
140b6693b: lea rax, [rip - 0xa5cab2]
140b66942: mov qword ptr [rbp + 0x30], rax
140b66946: lea rax, [rip + 0x1f8c4a3]
140b6694d: mov qword ptr [rbp - 0x60], rax
140b66951: mov qword ptr [rbp - 0x58], 2
140b66959: mov qword ptr [rbp - 0x40], 0
140b66961: lea rax, [rbp + 0x28]
140b66965: mov qword ptr [rbp - 0x50], rax
140b66969: mov qword ptr [rbp - 0x48], 1
140b66971: mov byte ptr [rbp + 0x47], 1
140b66975: lea rdx, [rip + 0x1f8c32c]
140b6697c: lea rcx, [rbp - 0x60]
140b66980: call 0x1428d9390
140b66985: ud2 
140b66987: nop word ptr [rax + rax]
140b66990: mov qword ptr [rsp + 0x10], rdx
140b66995: push rbp
140b66996: push rsi
140b66997: push rbx
140b66998: sub rsp, 0x20
140b6699c: lea rbp, [rdx + 0x80]
140b669a3: cmp byte ptr [rbp + 0x47], 0
140b669a7: jne 0x140b669b1
140b669a9: add rsp, 0x20
140b669ad: pop rbx
140b669ae: pop rsi
140b669af: pop rbp
140b669b0: ret 
140b669b1: mov rcx, qword ptr [rbp + 0x38]
140b669b5: call 0x140029880
140b669ba: jmp 0x140b669a9
