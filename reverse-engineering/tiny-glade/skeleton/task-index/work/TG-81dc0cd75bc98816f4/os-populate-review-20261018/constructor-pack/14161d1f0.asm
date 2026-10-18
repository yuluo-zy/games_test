14161d1f0: push rbp
14161d1f1: push r15
14161d1f3: push r14
14161d1f5: push r12
14161d1f7: push rsi
14161d1f8: push rdi
14161d1f9: push rbx
14161d1fa: sub rsp, 0x110
14161d201: lea rbp, [rsp + 0x80]
14161d209: mov qword ptr [rbp + 0x88], 0xfffffffffffffffe
14161d214: mov rdi, rdx
14161d217: mov rsi, rcx
14161d21a: test r8, r8
14161d21d: je 0x14161d237
14161d21f: mov rbx, r9
14161d222: cmp r9, 0x40001
14161d229: jae 0x14161d3fb
14161d22f: mov r15, r8
14161d232: xor r12d, r12d
14161d235: jmp 0x14161d271
14161d237: movzx eax, byte ptr [rip + 0x20179cb]
14161d23e: mov ebx, 0x40000
14161d243: mov ecx, 0x40000
14161d248: mov edx, 1
14161d24d: call 0x140613c10
14161d252: test rax, rax
14161d255: je 0x14161d465
14161d25b: mov r15, rax
14161d25e: mov r8d, 0x40000
14161d264: mov rcx, rax
14161d267: xor edx, edx
14161d269: call 0x142923c20
14161d26e: mov r12b, 1
14161d271: mov qword ptr [rbp + 0x78], r15
14161d275: mov byte ptr [rbp + 0x87], r12b
14161d27c: lea rcx, [rbp + 0x20]
14161d280: mov edx, 0x40000
14161d285: mov r8d, 0x22
14161d28b: mov r9b, 2
14161d28e: call 0x140bd4de0
14161d293: mov qword ptr [rbp + 0x78], r15
14161d297: mov byte ptr [rbp + 0x87], r12b
14161d29e: lea r14, [rbp + 8]
14161d2a2: lea rdx, [rbp + 0x20]
14161d2a6: mov r8d, 1
14161d2ac: mov rcx, r14
14161d2af: call 0x140bd4f20
14161d2b4: mov qword ptr [rbp + 0x78], r15
14161d2b8: mov byte ptr [rbp + 0x87], r12b
14161d2bf: mov qword ptr [rsp + 0x30], rbx
14161d2c4: mov qword ptr [rsp + 0x28], r15
14161d2c9: mov qword ptr [rsp + 0x20], r14
14161d2ce: lea r8, [rip + 0x1662cad]
14161d2d5: lea rcx, [rbp - 0x48]
14161d2d9: mov r9d, 0x1d
14161d2df: mov rdx, rdi
14161d2e2: call 0x1416a2dc0
14161d2e7: cmp dword ptr [rbp - 0x48], 0xb
14161d2eb: jne 0x14161d38d
14161d2f1: mov rax, qword ptr [rbp - 0x20]
14161d2f5: mov qword ptr [rbp + 0x40], rax
14161d2f9: movups xmm0, xmmword ptr [rbp - 0x40]
14161d2fd: movups xmm1, xmmword ptr [rbp - 0x30]
14161d301: movaps xmmword ptr [rbp + 0x30], xmm1
14161d305: movaps xmmword ptr [rbp + 0x20], xmm0
14161d309: test r12b, r12b
14161d30c: je 0x14161d321
14161d30e: mov edx, 0x40000
14161d313: mov r8d, 1
14161d319: mov rcx, r15
14161d31c: call 0x140613c20
14161d321: lea rcx, [rbp + 0x20]
14161d325: mov rdx, rdi
14161d328: call 0x140bd4cd0
14161d32d: test rax, rax
14161d330: je 0x14161d3ea
14161d336: cmp rdx, qword ptr [rbp + 0x38]
14161d33a: jb 0x14161d370
14161d33c: mov rcx, qword ptr [rbp + 0x40]
14161d340: mov qword ptr [rsi + 0x30], rcx
14161d344: movaps xmm0, xmmword ptr [rbp + 0x20]
14161d348: movaps xmm1, xmmword ptr [rbp + 0x30]
14161d34c: movups xmmword ptr [rsi + 0x20], xmm1
14161d350: movups xmmword ptr [rsi + 0x10], xmm0
14161d354: mov qword ptr [rsi], rax
14161d357: mov qword ptr [rsi + 8], rdx
14161d35b: mov rax, rsi
14161d35e: add rsp, 0x110
14161d365: pop rbx
14161d366: pop rdi
14161d367: pop rsi
14161d368: pop r12
14161d36a: pop r14
14161d36c: pop r15
14161d36e: pop rbp
14161d36f: ret 
14161d370: lea rcx, [rip + 0x1662e79]
14161d377: lea r8, [rip + 0x1662eaa]
14161d37e: mov edx, 0x37
14161d383: call 0x1428d9430
14161d388: jmp 0x14161d482
14161d38d: mov dword ptr [rbp + 0x74], r12d
14161d391: mov qword ptr [rbp + 0x18], r15
14161d395: movups xmm0, xmmword ptr [rbp - 8]
14161d399: movaps xmmword ptr [rbp + 0x60], xmm0
14161d39d: movups xmm0, xmmword ptr [rbp - 0x48]
14161d3a1: movups xmm1, xmmword ptr [rbp - 0x38]
14161d3a5: movups xmm2, xmmword ptr [rbp - 0x28]
14161d3a9: movups xmm3, xmmword ptr [rbp - 0x18]
14161d3ad: movaps xmmword ptr [rbp + 0x50], xmm3
14161d3b1: movaps xmmword ptr [rbp + 0x40], xmm2
14161d3b5: movaps xmmword ptr [rbp + 0x30], xmm1
14161d3b9: movaps xmmword ptr [rbp + 0x20], xmm0
14161d3bd: lea rax, [rip + 0x1662de4]
14161d3c4: mov qword ptr [rsp + 0x20], rax
14161d3c9: lea rcx, [rip + 0x1661718]
14161d3d0: lea r9, [rip + 0x16616f1]
14161d3d7: lea r8, [rbp + 0x20]
14161d3db: mov edx, 0x2b
14161d3e0: call 0x1428d9760
14161d3e5: jmp 0x14161d482
14161d3ea: lea rcx, [rip + 0x1662de7]
14161d3f1: call 0x1428d9310
14161d3f6: jmp 0x14161d482
14161d3fb: lea rax, [rip + 0x14810aa]
14161d402: mov qword ptr [rbp - 0x48], rax
14161d406: mov qword ptr [rbp - 0x40], 0x49
14161d40e: lea rax, [rbp - 0x48]
14161d412: mov qword ptr [rbp + 8], rax
14161d416: lea rax, [rip - 0x151358d]
14161d41d: mov qword ptr [rbp + 0x10], rax
14161d421: lea rax, [rip + 0x16623a0]
14161d428: mov qword ptr [rbp + 0x20], rax
14161d42c: mov qword ptr [rbp + 0x28], 1
14161d434: mov qword ptr [rbp + 0x40], 0
14161d43c: lea rax, [rbp + 8]
14161d440: mov qword ptr [rbp + 0x30], rax
14161d444: mov qword ptr [rbp + 0x38], 1
14161d44c: mov byte ptr [rbp + 0x87], 0
14161d453: lea rdx, [rip + 0x166237e]
14161d45a: lea rcx, [rbp + 0x20]
14161d45e: call 0x1428d9390
14161d463: jmp 0x14161d482
14161d465: mov byte ptr [rbp + 0x87], 0
14161d46c: lea r8, [rip + 0x166163d]
14161d473: mov ecx, 1
14161d478: mov edx, 0x40000
14161d47d: call 0x1428d8fe3
14161d482: ud2 
14161d484: nop word ptr cs:[rax + rax]
14161d490: mov qword ptr [rsp + 0x10], rdx
14161d495: push rbp
14161d496: push r15
14161d498: push r14
14161d49a: push r12
14161d49c: push rsi
14161d49d: push rdi
14161d49e: push rbx
14161d49f: sub rsp, 0x40
14161d4a3: lea rbp, [rdx + 0x80]
14161d4aa: test byte ptr [rbp + 0x87], 1
14161d4b1: je 0x14161d4c7
14161d4b3: mov rcx, qword ptr [rbp + 0x78]
14161d4b7: mov edx, 0x40000
14161d4bc: mov r8d, 1
14161d4c2: call 0x140613c20
14161d4c7: nop 
14161d4c8: add rsp, 0x40
14161d4cc: pop rbx
14161d4cd: pop rdi
14161d4ce: pop rsi
14161d4cf: pop r12
14161d4d1: pop r14
14161d4d3: pop r15
14161d4d5: pop rbp
14161d4d6: ret 
14161d4d7: nop word ptr [rax + rax]
14161d4e0: mov qword ptr [rsp + 0x10], rdx
14161d4e5: push rbp
14161d4e6: push r15
14161d4e8: push r14
14161d4ea: push r12
14161d4ec: push rsi
14161d4ed: push rdi
14161d4ee: push rbx
14161d4ef: sub rsp, 0x40
14161d4f3: lea rbp, [rdx + 0x80]
14161d4fa: lea rcx, [rbp + 0x20]
14161d4fe: call 0x140be24b0
14161d503: nop 
14161d504: add rsp, 0x40
14161d508: pop rbx
14161d509: pop rdi
14161d50a: pop rsi
14161d50b: pop r12
14161d50d: pop r14
14161d50f: pop r15
14161d511: pop rbp
14161d512: ret 
14161d513: nop word ptr cs:[rax + rax]
14161d520: mov qword ptr [rsp + 0x10], rdx
14161d525: push rbp
14161d526: push r15
14161d528: push r14
14161d52a: push r12
14161d52c: push rsi
14161d52d: push rdi
14161d52e: push rbx
14161d52f: sub rsp, 0x40
14161d533: lea rbp, [rdx + 0x80]
14161d53a: mov rsi, qword ptr [rbp + 0x18]
14161d53e: mov qword ptr [rbp + 0x78], rsi
14161d542: mov edi, dword ptr [rbp + 0x74]
14161d545: mov byte ptr [rbp + 0x87], dil
14161d54c: lea rcx, [rbp + 0x20]
14161d550: call 0x1407cea70
14161d555: mov qword ptr [rbp + 0x78], rsi
14161d559: mov byte ptr [rbp + 0x87], dil
14161d560: add rsp, 0x40
14161d564: pop rbx
14161d565: pop rdi
14161d566: pop rsi
14161d567: pop r12
14161d569: pop r14
14161d56b: pop r15
14161d56d: pop rbp
14161d56e: ret 
