141611730: push rbp
141611731: push r15
141611733: push r14
141611735: push r12
141611737: push rsi
141611738: push rdi
141611739: push rbx
14161173a: sub rsp, 0x100
141611741: lea rbp, [rsp + 0x80]
141611749: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141611751: mov rbx, r8
141611754: mov rdi, rdx
141611757: mov rsi, rcx
14161175a: test r8, r8
14161175d: je 0x1416117bb
14161175f: movabs rcx, 0xaaaaaaaaaaaaaaab
141611769: mov rax, r9
14161176c: mul rcx
14161176f: shr rdx, 1
141611772: and rdx, 0xfffffffffffffff0
141611776: lea rax, [rdx + rdx*2]
14161177a: mov rcx, r9
14161177d: sub rcx, rax
141611780: mov qword ptr [rbp - 0x40], rcx
141611784: jne 0x14161184c
14161178a: mov rax, rbx
14161178d: and rax, 3
141611791: mov qword ptr [rbp - 0x40], rax
141611795: jne 0x14161185d
14161179b: cmp r9, 0x600030
1416117a2: jae 0x1416118d9
1416117a8: mov eax, r9d
1416117ab: mov r12d, 0xaaaaaaab
1416117b1: imul r12, rax
1416117b5: shr r12, 0x25
1416117b9: jmp 0x1416117bb
1416117bb: lea r14, [rbp + 0x10]
1416117bf: mov edx, 0x600000
1416117c4: mov rcx, r14
1416117c7: mov r8d, 0x22
1416117cd: mov r9b, 1
1416117d0: call 0x140bd4de0
1416117d5: lea r15, [rbp + 0x68]
1416117d9: mov r8d, 4
1416117df: mov rcx, r15
1416117e2: mov rdx, r14
1416117e5: call 0x140bd4f20
1416117ea: shl r12, 4
1416117ee: lea rax, [r12 + r12*2]
1416117f2: mov qword ptr [rsp + 0x30], rax
1416117f7: mov qword ptr [rsp + 0x28], rbx
1416117fc: mov qword ptr [rsp + 0x20], r15
141611801: lea r8, [rip + 0x166e1a0]
141611808: lea rcx, [rbp - 0x40]
14161180c: mov r9d, 8
141611812: mov rdx, rdi
141611815: call 0x1416a2dc0
14161181a: cmp dword ptr [rbp - 0x40], 0xb
14161181e: jne 0x141611887
141611820: mov rax, qword ptr [rbp - 0x18]
141611824: mov qword ptr [rsi + 0x20], rax
141611828: movups xmm0, xmmword ptr [rbp - 0x38]
14161182c: movups xmm1, xmmword ptr [rbp - 0x28]
141611830: movups xmmword ptr [rsi + 0x10], xmm1
141611834: movups xmmword ptr [rsi], xmm0
141611837: mov rax, rsi
14161183a: add rsp, 0x100
141611841: pop rbx
141611842: pop rdi
141611843: pop rsi
141611844: pop r12
141611846: pop r14
141611848: pop r15
14161184a: pop rbp
14161184b: ret 
14161184c: mov qword ptr [rbp + 0x10], 0
141611854: lea rax, [rip + 0x16b86ed]
14161185b: jmp 0x14161186c
14161185d: mov qword ptr [rbp + 0x10], 0
141611865: lea rax, [rip + 0x16b86f4]
14161186c: mov qword ptr [rsp + 0x20], rax
141611871: lea r8, [rip + 0x16b86c8]
141611878: lea rdx, [rbp - 0x40]
14161187c: lea r9, [rbp + 0x10]
141611880: xor ecx, ecx
141611882: call 0x1428d9596
141611887: movups xmm0, xmmword ptr [rbp]
14161188b: movaps xmmword ptr [rbp + 0x50], xmm0
14161188f: movups xmm0, xmmword ptr [rbp - 0x40]
141611893: movups xmm1, xmmword ptr [rbp - 0x30]
141611897: movups xmm2, xmmword ptr [rbp - 0x20]
14161189b: movups xmm3, xmmword ptr [rbp - 0x10]
14161189f: movaps xmmword ptr [rbp + 0x40], xmm3
1416118a3: movaps xmmword ptr [rbp + 0x30], xmm2
1416118a7: movaps xmmword ptr [rbp + 0x20], xmm1
1416118ab: movaps xmmword ptr [rbp + 0x10], xmm0
1416118af: lea rax, [rip + 0x166e8f2]
1416118b6: mov qword ptr [rsp + 0x20], rax
1416118bb: lea rcx, [rip + 0x166d226]
1416118c2: lea r9, [rip + 0x166d1ff]
1416118c9: lea r8, [rbp + 0x10]
1416118cd: mov edx, 0x2b
1416118d2: call 0x1428d9760
1416118d7: ud2 
1416118d9: lea rax, [rip + 0x148c1e3]
1416118e0: mov qword ptr [rbp - 0x40], rax
1416118e4: mov qword ptr [rbp - 0x38], 0x49
1416118ec: lea rax, [rbp - 0x40]
1416118f0: mov qword ptr [rbp + 0x68], rax
1416118f4: lea rax, [rip - 0x1507a6b]
1416118fb: mov qword ptr [rbp + 0x70], rax
1416118ff: lea rax, [rip + 0x166dec2]
141611906: mov qword ptr [rbp + 0x10], rax
14161190a: mov qword ptr [rbp + 0x18], 1
141611912: mov qword ptr [rbp + 0x30], 0
14161191a: lea rax, [rbp + 0x68]
14161191e: mov qword ptr [rbp + 0x20], rax
141611922: mov qword ptr [rbp + 0x28], 1
14161192a: lea rdx, [rip + 0x166dea7]
141611931: lea rcx, [rbp + 0x10]
141611935: call 0x1428d9390
14161193a: int3 
14161193b: nop dword ptr [rax + rax]
141611940: mov qword ptr [rsp + 0x10], rdx
141611945: push rbp
141611946: push r15
141611948: push r14
14161194a: push r12
14161194c: push rsi
14161194d: push rdi
14161194e: push rbx
14161194f: sub rsp, 0x40
141611953: lea rbp, [rdx + 0x80]
14161195a: lea rcx, [rbp + 0x10]
14161195e: call 0x1407cea70
141611963: nop 
141611964: add rsp, 0x40
141611968: pop rbx
141611969: pop rdi
14161196a: pop rsi
14161196b: pop r12
14161196d: pop r14
14161196f: pop r15
141611971: pop rbp
141611972: ret 
