141449790: push rbp
141449791: push rsi
141449792: push rdi
141449793: push rbx
141449794: sub rsp, 0x88
14144979b: lea rbp, [rsp + 0x80]
1414497a3: mov qword ptr [rbp], 0xfffffffffffffffe
1414497ab: mov rdi, rdx
1414497ae: mov rsi, rcx
1414497b1: call 0x140d4d740
1414497b6: mov rbx, rax
1414497b9: test byte ptr [rax], 1
1414497bc: je 0x1414497cf
1414497be: mov rcx, rbx
1414497c1: add rcx, 8
1414497c5: mov rax, qword ptr [rbx + 8]
1414497c9: mov rdx, qword ptr [rbx + 0x10]
1414497cd: jmp 0x1414497ea
1414497cf: call 0x140d64e00
1414497d4: mov rcx, rbx
1414497d7: add rcx, 8
1414497db: mov qword ptr [rbx], 1
1414497e2: mov qword ptr [rbx + 8], rax
1414497e6: mov qword ptr [rbx + 0x10], rdx
1414497ea: lea r8, [rax + 1]
1414497ee: mov qword ptr [rcx], r8
1414497f1: movups xmm0, xmmword ptr [rip + 0x1816048]
1414497f8: movaps xmmword ptr [rbp - 0x20], xmm0
1414497fc: movups xmm0, xmmword ptr [rip + 0x181602d]
141449803: movaps xmmword ptr [rbp - 0x30], xmm0
141449807: mov qword ptr [rbp - 0x10], rax
14144980b: mov qword ptr [rbp - 8], rdx
14144980f: mov rdx, qword ptr [rdi + 0x20]
141449813: test rdx, rdx
141449816: jne 0x141449862
141449818: lea rax, [rbp - 0x30]
14144981c: mov qword ptr [rbp - 0x38], rax
141449820: movups xmm0, xmmword ptr [rdi]
141449823: movups xmm1, xmmword ptr [rdi + 0x10]
141449827: movaps xmmword ptr [rbp - 0x60], xmm0
14144982b: movaps xmmword ptr [rbp - 0x50], xmm1
14144982f: lea rcx, [rbp - 0x60]
141449833: lea r8, [rbp - 0x38]
141449837: call 0x140b760d0
14144983c: movaps xmm0, xmmword ptr [rbp - 0x30]
141449840: movaps xmm1, xmmword ptr [rbp - 0x20]
141449844: movaps xmm2, xmmword ptr [rbp - 0x10]
141449848: movups xmmword ptr [rsi + 0x20], xmm2
14144984c: movups xmmword ptr [rsi + 0x10], xmm1
141449850: movups xmmword ptr [rsi], xmm0
141449853: mov rax, rsi
141449856: add rsp, 0x88
14144985d: pop rbx
14144985e: pop rdi
14144985f: pop rsi
141449860: pop rbp
141449861: ret 
141449862: lea rcx, [rbp - 0x30]
141449866: mov rbx, rdx
141449869: lea r8, [rbp - 0x10]
14144986d: mov r9b, 1
141449870: call 0x1428c1430
141449875: mov rdx, rbx
141449878: jmp 0x141449818
14144987a: nop word ptr [rax + rax]
141449880: mov qword ptr [rsp + 0x10], rdx
141449885: push rbp
141449886: push rsi
141449887: push rdi
141449888: push rbx
141449889: sub rsp, 0x28
14144988d: lea rbp, [rdx + 0x80]
141449894: lea rcx, [rbp - 0x30]
141449898: mov r8d, 0x20
14144989e: mov r9d, 0x10
1414498a4: lea rdx, [rbp - 0x10]
1414498a8: call 0x1400524e0
1414498ad: nop 
1414498ae: add rsp, 0x28
1414498b2: pop rbx
1414498b3: pop rdi
1414498b4: pop rsi
1414498b5: pop rbp
1414498b6: ret 
