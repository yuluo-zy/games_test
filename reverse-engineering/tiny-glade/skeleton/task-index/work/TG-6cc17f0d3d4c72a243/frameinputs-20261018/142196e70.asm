142196e70: push rbp
142196e71: push r15
142196e73: push r14
142196e75: push r13
142196e77: push r12
142196e79: push rsi
142196e7a: push rdi
142196e7b: push rbx
142196e7c: sub rsp, 0x168
142196e83: lea rbp, [rsp + 0x80]
142196e8b: mov qword ptr [rbp + 0xe0], 0xfffffffffffffffe
142196e96: mov rbx, r8
142196e99: mov rdi, rdx
142196e9c: mov rsi, rcx
142196e9f: call 0x140c16a00
142196ea4: test al, al
142196ea6: mov byte ptr [rbp + 0xd8], al
142196eac: je 0x142196f0b
142196eae: mov r14d, 0x34
142196eb4: mov ecx, 0x34
142196eb9: call 0x1421969e0
142196ebe: lea r15, [rip + 0xe5d1b6]
142196ec5: cmp rax, 1
142196ec9: jne 0x142196f78
142196ecf: mov r9, rdx
142196ed2: test rdx, rdx
142196ed5: je 0x142196f15
142196ed7: cmp r9, 0x34
142196edb: jae 0x142196f13
142196edd: lea rax, [rip + 0xe5d197]
142196ee4: cmp byte ptr [r9 + rax], 0xbf
142196ee9: jg 0x142196f15
142196eeb: lea rax, [rip + 0xe5d246]
142196ef2: mov qword ptr [rsp + 0x20], rax
142196ef7: lea rcx, [rip + 0xe5d17d]
142196efe: mov edx, 0x34
142196f03: xor r8d, r8d
142196f06: call 0x1428d9940
142196f0b: xor r15d, r15d
142196f0e: jmp 0x1421970d1
142196f13: jne 0x142196eeb
142196f15: mov rcx, r9
142196f18: call 0x1421969e0
142196f1d: cmp rax, 1
142196f21: jne 0x142196f78
142196f23: mov r8, rdx
142196f26: add r8, 2
142196f2a: je 0x142196f65
142196f2c: cmp r8, 0x34
142196f30: jae 0x142196f63
142196f32: lea rax, [rip + 0xe5d142]
142196f39: cmp byte ptr [r8 + rax], 0xbf
142196f3e: jg 0x142196f65
142196f40: lea rax, [rip + 0xe5d209]
142196f47: mov qword ptr [rsp + 0x20], rax
142196f4c: lea rcx, [rip + 0xe5d128]
142196f53: mov edx, 0x34
142196f58: mov r9d, 0x34
142196f5e: call 0x1428d9940
142196f63: jne 0x142196f40
142196f65: mov r14d, 0x32
142196f6b: sub r14, rdx
142196f6e: lea r15, [rip + 0xe5d106]
142196f75: add r15, r8
142196f78: lea r8, [rip + 0xe5d26f]
142196f7f: lea r12, [rip + 0xe5d23a]
142196f86: nop word ptr cs:[rax + rax]
142196f90: movsx eax, byte ptr [r8 - 1]
142196f95: test eax, eax
142196f97: js 0x142196fb0
142196f99: dec r8
142196f9c: cmp eax, 0x5c
142196f9f: jne 0x142197004
142196fa1: jmp 0x142197016
142196fa3: nop word ptr cs:[rax + rax]
142196fb0: movzx ecx, byte ptr [r8 - 2]
142196fb5: cmp cl, 0xc0
142196fb8: jge 0x142196fdd
142196fba: movzx edx, byte ptr [r8 - 3]
142196fbf: cmp dl, 0xc0
142196fc2: jge 0x142196fe6
142196fc4: movzx r9d, byte ptr [r8 - 4]
142196fc9: add r8, -4
142196fcd: and r9d, 7
142196fd1: shl r9d, 6
142196fd5: and edx, 0x3f
142196fd8: or edx, r9d
142196fdb: jmp 0x142196fed
142196fdd: add r8, -2
142196fe1: and ecx, 0x1f
142196fe4: jmp 0x142196ff5
142196fe6: add r8, -3
142196fea: and edx, 0xf
142196fed: shl edx, 6
142196ff0: and ecx, 0x3f
142196ff3: or ecx, edx
142196ff5: shl ecx, 6
142196ff8: and al, 0x3f
142196ffa: movzx eax, al
142196ffd: or eax, ecx
142196fff: cmp eax, 0x5c
142197002: je 0x142197016
142197004: cmp eax, 0x2f
142197007: je 0x142197016
142197009: cmp r8, r12
14219700c: jne 0x142196f90
14219700e: mov r13d, 0x2e
142197014: jmp 0x142197063
142197016: lea r12, [rip + 0xe5d1a3]
14219701d: sub r8, r12
142197020: inc r8
142197023: je 0x142197057
142197025: cmp r8, 0x2e
142197029: jae 0x142197055
14219702b: cmp byte ptr [r8 + r12], 0xbf
142197030: jg 0x142197057
142197032: lea rax, [rip + 0xe5d0df]
142197039: mov qword ptr [rsp + 0x20], rax
14219703e: lea rcx, [rip + 0xe5d17b]
142197045: mov edx, 0x2e
14219704a: mov r9d, 0x2e
142197050: call 0x1428d9940
142197055: jne 0x142197032
142197057: mov r13d, 0x2e
14219705d: sub r13, r8
142197060: add r12, r8
142197063: call 0x140c18130
142197068: mov rcx, rax
14219706b: mov rax, qword ptr [rax]
14219706e: cmp rax, 1
142197072: jne 0x1421973ac
142197078: add rcx, 8
14219707c: cmp qword ptr [rcx], 0
142197080: jne 0x1421973d0
142197086: mov qword ptr [rcx], 0xffffffffffffffff
14219708d: mov qword ptr [rbp + 0xb8], rcx
142197094: add rcx, 8
142197098: mov qword ptr [rsp + 0x20], r13
14219709d: mov qword ptr [rsp + 0x30], 0
1421970a6: mov qword ptr [rsp + 0x28], 1
1421970af: mov rdx, r15
1421970b2: mov r8, r14
1421970b5: mov r9, r12
1421970b8: call 0x140c16d40
1421970bd: mov rcx, qword ptr [rbp + 0xb8]
1421970c4: inc qword ptr [rcx]
1421970c7: mov qword ptr [rbp + 0x30], rax
1421970cb: mov r15d, 1
1421970d1: mov qword ptr [rbp + 0x28], r15
1421970d5: mov rcx, qword ptr [rbx]
1421970d8: lea rdx, [rip + 0xd24281]
1421970df: call 0x1421aeb40
1421970e4: mov rbx, rax
1421970e7: test rax, rax
1421970ea: je 0x14219728e
1421970f0: lea rax, [rip + 0xe60d66]
1421970f7: mov qword ptr [rbp + 0xc0], rax
1421970fe: mov qword ptr [rbp + 0xc8], 0x36
142197109: mov r14, rbx
14219710c: add r14, 0x10
142197110: cmp qword ptr [rbx + 0x18], 0x36
142197115: jne 0x1421973dc
14219711b: mov rdx, qword ptr [r14]
14219711e: lea rcx, [rip + 0xe60d38]
142197125: mov r8d, 0x36
14219712b: call 0x142923bf0
142197130: test eax, eax
142197132: jne 0x1421973dc
142197138: mov rcx, qword ptr [rbx]
14219713b: mov rax, qword ptr [rbx + 8]
14219713f: call qword ptr [rax + 0x18]
142197142: mov rbx, rax
142197145: test rax, rax
142197148: je 0x14219728e
14219714e: mov rax, qword ptr [rdi]
142197151: mov rcx, qword ptr [rdi + 8]
142197155: mov rdx, qword ptr [rax + 0xf8]
14219715c: mov r8, qword ptr [rax + 0x100]
142197163: lea r8, [rdx + r8*4]
142197167: movzx r9d, byte ptr [rax + 0x108]
14219716f: xorps xmm0, xmm0
142197172: movups xmmword ptr [rbp + 0x90], xmm0
142197179: movups xmmword ptr [rbp + 0x80], xmm0
142197180: mov qword ptr [rbp + 0xa0], 0
14219718b: lea r10, [rcx + 0xf8]
142197192: mov qword ptr [rbp + 0x38], rcx
142197196: add rcx, 0x1a0
14219719d: mov qword ptr [rbp + 0x40], rcx
1421971a1: mov qword ptr [rbp + 0x48], r10
1421971a5: mov qword ptr [rbp + 0x50], rax
1421971a9: mov qword ptr [rbp + 0x58], rdx
1421971ad: mov qword ptr [rbp + 0x60], r8
1421971b1: mov qword ptr [rbp + 0x68], 8
1421971b9: mov qword ptr [rbp + 0x70], 0
1421971c1: mov qword ptr [rbp + 0x78], 8
1421971c9: mov byte ptr [rbp + 0xa8], r9b
1421971d0: lea r8, [rip + 0xe5ce41]
1421971d7: lea rcx, [rbp + 0xc0]
1421971de: lea rdx, [rbp + 0x38]
1421971e2: call 0x140dce380
1421971e7: mov rcx, qword ptr [rsi]
1421971ea: call 0x140987ca0
1421971ef: mov rsi, qword ptr [rbp + 0xd0]
1421971f6: test rsi, rsi
1421971f9: je 0x142197260
1421971fb: mov rcx, rsi
1421971fe: shl rcx, 5
142197202: cmp rcx, qword ptr [rbx + 0x20]
142197206: ja 0x14219740a
14219720c: mov r9, qword ptr [rbp + 0xc8]
142197213: mov r8, rbx
142197216: add r8, 8
14219721a: mov qword ptr [rbp + 8], 0
142197222: mov qword ptr [rbp + 0x10], rcx
142197226: lea rdx, [rbp + 8]
14219722a: mov qword ptr [rsp + 0x28], rdx
14219722f: mov qword ptr [rsp + 0x20], rcx
142197234: mov qword ptr [rsp + 0x38], 0
14219723d: mov qword ptr [rsp + 0x30], 1
142197246: lea rcx, [rbp + 0x38]
14219724a: mov rdx, rax
14219724d: call 0x140beefa0
142197252: mov rax, qword ptr [rbp + 0x38]
142197256: cmp rax, 0xb
14219725a: jne 0x142197411
142197260: mov qword ptr [rbx], rsi
142197263: mov rdx, qword ptr [rbp + 0xc0]
14219726a: test rdx, rdx
14219726d: je 0x142197285
14219726f: mov rcx, qword ptr [rbp + 0xc8]
142197276: shl rdx, 5
14219727a: mov r8d, 0x10
142197280: call 0x140613c20
142197285: mov r15, qword ptr [rbp + 0x28]
142197289: jmp 0x14219733d
14219728e: mov rax, qword ptr [rip + 0x1495113]
142197295: mov rax, qword ptr [rax]
142197298: cmp rax, 2
14219729c: jb 0x14219733d
1421972a2: lea rcx, [rip + 0xe5cf9f]
1421972a9: call 0x14067dd80
1421972ae: movups xmm0, xmmword ptr [rax]
1421972b1: mov eax, dword ptr [rax + 0x10]
1421972b4: mov qword ptr [rbp + 0x68], 2
1421972bc: lea rcx, [rip + 0xe5cf9d]
1421972c3: mov qword ptr [rbp + 0x70], rcx
1421972c7: mov qword ptr [rbp + 0x78], 0x21
1421972cf: lea rdx, [rip + 0xe5cf62]
1421972d6: mov qword ptr [rbp + 0x88], rdx
1421972dd: mov qword ptr [rbp + 0x90], 1
1421972e8: mov qword ptr [rbp + 0x98], 8
1421972f3: mov qword ptr [rbp + 0x38], 0
1421972fb: xorps xmm1, xmm1
1421972fe: movups xmmword ptr [rbp + 0xa0], xmm1
142197305: mov qword ptr [rbp + 0x40], rcx
142197309: mov qword ptr [rbp + 0x48], 0x21
142197311: mov qword ptr [rbp + 0x50], 0
142197319: movups xmmword ptr [rbp + 0x58], xmm0
14219731d: mov dword ptr [rbp + 0x80], 1
142197327: mov dword ptr [rbp + 0x84], eax
14219732d: lea rcx, [rbp + 0xc0]
142197334: lea rdx, [rbp + 0x38]
142197338: call 0x140d4a330
14219733d: test r15, r15
142197340: je 0x142197382
142197342: call 0x140c18130
142197347: mov rcx, rax
14219734a: mov rax, qword ptr [rax]
14219734d: cmp rax, 1
142197351: jne 0x142197396
142197353: add rcx, 8
142197357: cmp qword ptr [rcx], 0
14219735b: jne 0x1421973d0
14219735d: mov qword ptr [rcx], 0xffffffffffffffff
142197364: mov qword ptr [rbp + 0xd8], rcx
14219736b: add rcx, 8
14219736f: mov rdx, qword ptr [rbp + 0x30]
142197373: call 0x140c170f0
142197378: mov rax, qword ptr [rbp + 0xd8]
14219737f: inc qword ptr [rax]
142197382: add rsp, 0x168
142197389: pop rbx
14219738a: pop rdi
14219738b: pop rsi
14219738c: pop r12
14219738e: pop r13
142197390: pop r14
142197392: pop r15
142197394: pop rbp
142197395: ret 
142197396: test rax, rax
142197399: jne 0x1421973c4
14219739b: xor edx, edx
14219739d: call 0x142918340
1421973a2: mov rcx, rax
1421973a5: test rax, rax
1421973a8: jne 0x142197357
1421973aa: jmp 0x1421973c4
1421973ac: test rax, rax
1421973af: jne 0x1421973c4
1421973b1: xor edx, edx
1421973b3: call 0x142918340
1421973b8: mov rcx, rax
1421973bb: test rax, rax
1421973be: jne 0x14219707c
1421973c4: lea rcx, [rip + 0xe5f11d]
1421973cb: call 0x1428d80d0
1421973d0: lea rcx, [rip + 0xe5f181]
1421973d7: call 0x1428d9250
1421973dc: mov qword ptr [rbp + 0x38], 0
1421973e4: lea rax, [rip + 0xd23f5d]
1421973eb: mov qword ptr [rsp + 0x20], rax
1421973f0: lea rdx, [rbp + 0xc0]
1421973f7: lea r9, [rbp + 0x38]
1421973fb: xor ecx, ecx
1421973fd: mov r8, r14
142197400: call 0x1428cab41
142197405: jmp 0x142197493
14219740a: mov eax, 0xb
14219740f: jmp 0x14219743c
142197411: mov rcx, qword ptr [rbp + 0x80]
142197418: mov qword ptr [rbp], rcx
14219741c: movups xmm0, xmmword ptr [rbp + 0x40]
142197420: movups xmm1, xmmword ptr [rbp + 0x50]
142197424: movups xmm2, xmmword ptr [rbp + 0x60]
142197428: movups xmm3, xmmword ptr [rbp + 0x70]
14219742c: movaps xmmword ptr [rbp - 0x10], xmm3
142197430: movaps xmmword ptr [rbp - 0x20], xmm2
142197434: movaps xmmword ptr [rbp - 0x30], xmm1
142197438: movaps xmmword ptr [rbp - 0x40], xmm0
14219743c: mov qword ptr [rbp + 0x38], rax
142197440: movaps xmm0, xmmword ptr [rbp - 0x40]
142197444: movaps xmm1, xmmword ptr [rbp - 0x30]
142197448: movaps xmm2, xmmword ptr [rbp - 0x20]
14219744c: movaps xmm3, xmmword ptr [rbp - 0x10]
142197450: movups xmmword ptr [rbp + 0x40], xmm0
142197454: movups xmmword ptr [rbp + 0x50], xmm1
142197458: movups xmmword ptr [rbp + 0x60], xmm2
14219745c: movups xmmword ptr [rbp + 0x70], xmm3
142197460: mov rax, qword ptr [rbp]
142197464: mov qword ptr [rbp + 0x80], rax
14219746b: lea rax, [rip + 0xe5cd7e]
142197472: mov qword ptr [rsp + 0x20], rax
142197477: lea rcx, [rip + 0xe5cbd2]
14219747e: lea r9, [rip + 0xe5cbab]
142197485: lea r8, [rbp + 0x38]
142197489: mov edx, 0x2b
14219748e: call 0x1428d9760
142197493: ud2 
142197495: nop word ptr cs:[rax + rax]
1421974a0: mov qword ptr [rsp + 0x10], rdx
1421974a5: push rbp
1421974a6: push r15
1421974a8: push r14
1421974aa: push r13
1421974ac: push r12
1421974ae: push rsi
1421974af: push rdi
1421974b0: push rbx
1421974b1: sub rsp, 0x48
1421974b5: lea rbp, [rdx + 0x80]
1421974bc: mov rax, qword ptr [rbp + 0xb8]
1421974c3: inc qword ptr [rax]
1421974c6: add rsp, 0x48
1421974ca: pop rbx
1421974cb: pop rdi
1421974cc: pop rsi
1421974cd: pop r12
1421974cf: pop r13
1421974d1: pop r14
1421974d3: pop r15
1421974d5: pop rbp
1421974d6: ret 
1421974d7: nop word ptr [rax + rax]
1421974e0: mov qword ptr [rsp + 0x10], rdx
1421974e5: push rbp
1421974e6: push r15
1421974e8: push r14
1421974ea: push r13
1421974ec: push r12
1421974ee: push rsi
1421974ef: push rdi
1421974f0: push rbx
1421974f1: sub rsp, 0x48
1421974f5: lea rbp, [rdx + 0x80]
1421974fc: cmp byte ptr [rbp + 0xd8], 0
142197503: je 0x142197525
142197505: lea rax, [rbp + 0x30]
142197509: mov qword ptr [rbp + 0x20], rax
14219750d: lea rax, [rbp + 0x20]
142197511: mov qword ptr [rbp + 0x18], rax
142197515: lea rcx, [rip + 0xe5cb9c]
14219751c: lea rdx, [rbp + 0x18]
142197520: call 0x1421a03e0
142197525: nop 
142197526: add rsp, 0x48
14219752a: pop rbx
14219752b: pop rdi
14219752c: pop rsi
14219752d: pop r12
14219752f: pop r13
142197531: pop r14
142197533: pop r15
142197535: pop rbp
142197536: ret 
142197537: nop word ptr [rax + rax]
142197540: mov qword ptr [rsp + 0x10], rdx
142197545: push rbp
142197546: push r15
142197548: push r14
14219754a: push r13
14219754c: push r12
14219754e: push rsi
14219754f: push rdi
142197550: push rbx
142197551: sub rsp, 0x48
142197555: lea rbp, [rdx + 0x80]
14219755c: mov rdx, qword ptr [rbp + 0xc0]
142197563: test rdx, rdx
142197566: je 0x14219757e
142197568: mov rcx, qword ptr [rbp + 0xc8]
14219756f: shl rdx, 5
142197573: mov r8d, 0x10
142197579: call 0x140613c20
14219757e: nop 
14219757f: add rsp, 0x48
142197583: pop rbx
142197584: pop rdi
142197585: pop rsi
142197586: pop r12
142197588: pop r13
14219758a: pop r14
14219758c: pop r15
14219758e: pop rbp
14219758f: ret 
142197590: mov qword ptr [rsp + 0x10], rdx
142197595: push rbp
142197596: push r15
142197598: push r14
14219759a: push r13
14219759c: push r12
14219759e: push rsi
14219759f: push rdi
1421975a0: push rbx
1421975a1: sub rsp, 0x48
1421975a5: lea rbp, [rdx + 0x80]
1421975ac: mov rax, qword ptr [rbp + 0xd8]
1421975b3: inc qword ptr [rax]
1421975b6: add rsp, 0x48
1421975ba: pop rbx
1421975bb: pop rdi
1421975bc: pop rsi
1421975bd: pop r12
1421975bf: pop r13
1421975c1: pop r14
1421975c3: pop r15
1421975c5: pop rbp
1421975c6: ret 
1421975c7: nop word ptr [rax + rax]
1421975d0: mov qword ptr [rsp + 0x10], rdx
1421975d5: push rbp
1421975d6: push r15
1421975d8: push r14
1421975da: push r13
1421975dc: push r12
1421975de: push rsi
1421975df: push rdi
1421975e0: push rbx
1421975e1: sub rsp, 0x48
1421975e5: lea rbp, [rdx + 0x80]
1421975ec: lea rcx, [rbp + 0x38]
1421975f0: call 0x1407b1810
1421975f5: nop 
1421975f6: add rsp, 0x48
1421975fa: pop rbx
1421975fb: pop rdi
1421975fc: pop rsi
1421975fd: pop r12
1421975ff: pop r13
142197601: pop r14
142197603: pop r15
142197605: pop rbp
142197606: ret 
