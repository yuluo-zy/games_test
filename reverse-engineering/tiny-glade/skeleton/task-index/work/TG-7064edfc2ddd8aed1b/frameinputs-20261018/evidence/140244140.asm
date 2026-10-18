140244140: push rbp
140244141: push rsi
140244142: push rdi
140244143: sub rsp, 0x90
14024414a: lea rbp, [rsp + 0x80]
140244152: movaps xmmword ptr [rbp], xmm7
140244156: movaps xmmword ptr [rbp - 0x10], xmm6
14024415a: mov qword ptr [rbp - 0x18], 0xfffffffffffffffe
140244162: mov rsi, rcx
140244165: call 0x140d4d740
14024416a: mov rdi, rax
14024416d: test byte ptr [rax], 1
140244170: je 0x140244183
140244172: mov rcx, rdi
140244175: add rcx, 8
140244179: mov rax, qword ptr [rdi + 8]
14024417d: mov rdx, qword ptr [rdi + 0x10]
140244181: jmp 0x14024419e
140244183: call 0x140d64e00
140244188: mov rcx, rdi
14024418b: add rcx, 8
14024418f: mov qword ptr [rdi], 1
140244196: mov qword ptr [rdi + 8], rax
14024419a: mov qword ptr [rdi + 0x10], rdx
14024419e: lea r8, [rax + 1]
1402441a2: mov qword ptr [rcx], r8
1402441a5: mov qword ptr [rbp - 0x60], 0
1402441ad: mov qword ptr [rbp - 0x58], 8
1402441b5: mov qword ptr [rbp - 0x50], 0
1402441bd: movups xmm6, xmmword ptr [rip + 0x275c0cc]
1402441c4: movups xmmword ptr [rbp - 0x48], xmm6
1402441c8: movups xmm7, xmmword ptr [rip + 0x275c0d1]
1402441cf: movups xmmword ptr [rbp - 0x38], xmm7
1402441d3: mov qword ptr [rbp - 0x28], rax
1402441d7: mov qword ptr [rbp - 0x20], rdx
1402441db: call 0x140d4d740
1402441e0: mov rdi, rax
1402441e3: test byte ptr [rax], 1
1402441e6: je 0x1402441f9
1402441e8: mov rcx, rdi
1402441eb: add rcx, 8
1402441ef: mov rax, qword ptr [rdi + 8]
1402441f3: mov rdx, qword ptr [rdi + 0x10]
1402441f7: jmp 0x140244214
1402441f9: call 0x140d64e00
1402441fe: mov rcx, rdi
140244201: add rcx, 8
140244205: mov qword ptr [rdi], 1
14024420c: mov qword ptr [rdi + 8], rax
140244210: mov qword ptr [rdi + 0x10], rdx
140244214: lea r8, [rax + 1]
140244218: mov qword ptr [rcx], r8
14024421b: mov rcx, qword ptr [rbp - 0x20]
14024421f: mov qword ptr [rsi + 0x40], rcx
140244223: movups xmm0, xmmword ptr [rbp - 0x60]
140244227: movups xmm1, xmmword ptr [rbp - 0x50]
14024422b: movups xmm2, xmmword ptr [rbp - 0x40]
14024422f: movups xmm3, xmmword ptr [rbp - 0x30]
140244233: movups xmmword ptr [rsi + 0x30], xmm3
140244237: movups xmmword ptr [rsi + 0x20], xmm2
14024423b: movups xmmword ptr [rsi + 0x10], xmm1
14024423f: movups xmmword ptr [rsi], xmm0
140244242: mov qword ptr [rsi + 0x48], 0
14024424a: mov qword ptr [rsi + 0x50], 8
140244252: mov qword ptr [rsi + 0x58], 0
14024425a: movups xmmword ptr [rsi + 0x60], xmm6
14024425e: movups xmmword ptr [rsi + 0x70], xmm7
140244262: mov qword ptr [rsi + 0x80], rax
140244269: mov qword ptr [rsi + 0x88], rdx
140244270: mov rax, rsi
140244273: movaps xmm6, xmmword ptr [rbp - 0x10]
140244277: movaps xmm7, xmmword ptr [rbp]
14024427b: add rsp, 0x90
140244282: pop rdi
140244283: pop rsi
140244284: pop rbp
140244285: ret 
140244286: nop word ptr cs:[rax + rax]
140244290: mov qword ptr [rsp + 0x10], rdx
140244295: push rbp
140244296: push rsi
140244297: push rdi
140244298: sub rsp, 0x40
14024429c: lea rbp, [rdx + 0x80]
1402442a3: movaps xmmword ptr [rsp + 0x20], xmm7
1402442a8: movaps xmmword ptr [rsp + 0x30], xmm6
1402442ad: lea rcx, [rbp - 0x60]
1402442b1: call 0x1405e3f30
1402442b6: movaps xmm6, xmmword ptr [rsp + 0x30]
1402442bb: movaps xmm7, xmmword ptr [rsp + 0x20]
1402442c0: add rsp, 0x40
1402442c4: pop rdi
1402442c5: pop rsi
1402442c6: pop rbp
1402442c7: ret 
