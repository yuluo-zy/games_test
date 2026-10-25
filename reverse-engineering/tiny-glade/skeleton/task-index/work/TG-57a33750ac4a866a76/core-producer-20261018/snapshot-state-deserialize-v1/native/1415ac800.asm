1415ac800: 55  push rbp
1415ac801: 4157  push r15
1415ac803: 4156  push r14
1415ac805: 4155  push r13
1415ac807: 4154  push r12
1415ac809: 56  push rsi
1415ac80a: 57  push rdi
1415ac80b: 53  push rbx
1415ac80c: 4881eca8000000  sub rsp, 0xa8
1415ac813: 488dac2480000000  lea rbp, [rsp + 0x80]
1415ac81b: 0f297510  movaps xmmword ptr [rbp + 0x10], xmm6
1415ac81f: 48c74508feffffff  mov qword ptr [rbp + 8], 0xfffffffffffffffe
1415ac827: 4889ce  mov rsi, rcx
1415ac82a: 488b5a10  mov rbx, qword ptr [rdx + 0x10]
1415ac82e: 488d4db0  lea rcx, [rbp - 0x50]
1415ac832: e849b85dff  call 0x140b88080
1415ac837: 488b45b8  mov rax, qword ptr [rbp - 0x48]
1415ac83b: 488b7dc8  mov rdi, qword ptr [rbp - 0x38]
1415ac83f: 4839f8  cmp rax, rdi
1415ac842: 7445  je 0x1415ac889
1415ac844: 4c8d6820  lea r13, [rax + 0x20]
1415ac848: 4c896db8  mov qword ptr [rbp - 0x48], r13
1415ac84c: 0fb608  movzx ecx, byte ptr [rax]
1415ac84f: 80f906  cmp cl, 6
1415ac852: 7438  je 0x1415ac88c
1415ac854: 884dd0  mov byte ptr [rbp - 0x30], cl
1415ac857: 0f104001  movups xmm0, xmmword ptr [rax + 1]
1415ac85b: 0f104810  movups xmm1, xmmword ptr [rax + 0x10]
1415ac85f: 0f1145d1  movups xmmword ptr [rbp - 0x2f], xmm0
1415ac863: 0f114de0  movups xmmword ptr [rbp - 0x20], xmm1
1415ac867: 488d4df0  lea rcx, [rbp - 0x10]
1415ac86b: 488d55d0  lea rdx, [rbp - 0x30]
1415ac86f: e8fcdafeff  call 0x14159a370
1415ac874: 837df001  cmp dword ptr [rbp - 0x10], 1
1415ac878: 747a  je 0x1415ac8f4
1415ac87a: f30f1075f4  movss xmm6, dword ptr [rbp - 0xc]
1415ac87f: 4939fd  cmp r13, rdi
1415ac882: 7510  jne 0x1415ac894
1415ac884: 4d89ee  mov r14, r13
1415ac887: eb1c  jmp 0x1415ac8a5
1415ac889: 4989c5  mov r13, rax
1415ac88c: 0f57f6  xorps xmm6, xmm6
1415ac88f: 4939fd  cmp r13, rdi
1415ac892: 74f0  je 0x1415ac884
1415ac894: 4d8d7520  lea r14, [r13 + 0x20]
1415ac898: 4c8975b8  mov qword ptr [rbp - 0x48], r14
1415ac89c: 410fb64500  movzx eax, byte ptr [r13]
1415ac8a1: 3c06  cmp al, 6
1415ac8a3: 7555  jne 0x1415ac8fa
1415ac8a5: 4531ff  xor r15d, r15d
1415ac8a8: 4531e4  xor r12d, r12d
1415ac8ab: 4939fe  cmp r14, rdi
1415ac8ae: 0f849e000000  je 0x1415ac952
1415ac8b4: 4d8d6e20  lea r13, [r14 + 0x20]
1415ac8b8: 4c896db8  mov qword ptr [rbp - 0x48], r13
1415ac8bc: 410fb606  movzx eax, byte ptr [r14]
1415ac8c0: 3c06  cmp al, 6
1415ac8c2: 0f848d000000  je 0x1415ac955
1415ac8c8: 8845d0  mov byte ptr [rbp - 0x30], al
1415ac8cb: 410f104601  movups xmm0, xmmword ptr [r14 + 1]
1415ac8d0: 410f104e10  movups xmm1, xmmword ptr [r14 + 0x10]
1415ac8d5: 0f1145d1  movups xmmword ptr [rbp - 0x2f], xmm0
1415ac8d9: 0f114de0  movups xmmword ptr [rbp - 0x20], xmm1
1415ac8dd: 488d4df0  lea rcx, [rbp - 0x10]
1415ac8e1: 488d55d0  lea rdx, [rbp - 0x30]
1415ac8e5: e896d8feff  call 0x14159a180
1415ac8ea: 807df001  cmp byte ptr [rbp - 0x10], 1
1415ac8ee: 0f85c9000000  jne 0x1415ac9bd
1415ac8f4: 488b45f8  mov rax, qword ptr [rbp - 8]
1415ac8f8: eb73  jmp 0x1415ac96d
1415ac8fa: 8845d0  mov byte ptr [rbp - 0x30], al
1415ac8fd: 410f104501  movups xmm0, xmmword ptr [r13 + 1]
1415ac902: 410f104d10  movups xmm1, xmmword ptr [r13 + 0x10]
1415ac907: 0f1145d1  movups xmmword ptr [rbp - 0x2f], xmm0
1415ac90b: 0f114de0  movups xmmword ptr [rbp - 0x20], xmm1
1415ac90f: 48c744242003000000  mov qword ptr [rsp + 0x20], 3
1415ac918: 4c8d05a98c4e01  lea r8, [rip + 0x14e8ca9]
1415ac91f: 488d4df0  lea rcx, [rbp - 0x10]
1415ac923: 488d55d0  lea rdx, [rbp - 0x30]
1415ac927: 41b904000000  mov r9d, 4
1415ac92d: e8be72ffff  call 0x1415a3bf0
1415ac932: 837df001  cmp dword ptr [rbp - 0x10], 1
1415ac936: 7509  jne 0x1415ac941
1415ac938: 488b45f8  mov rax, qword ptr [rbp - 8]
1415ac93c: 4d89f5  mov r13, r14
1415ac93f: eb2c  jmp 0x1415ac96d
1415ac941: 448b65f4  mov r12d, dword ptr [rbp - 0xc]
1415ac945: 4c8b7df8  mov r15, qword ptr [rbp - 8]
1415ac949: 4939fe  cmp r14, rdi
1415ac94c: 0f8562ffffff  jne 0x1415ac8b4
1415ac952: 4d89f5  mov r13, r14
1415ac955: 488d154ce86c01  lea rdx, [rip + 0x16ce84c]
1415ac95c: 4c8d05fdd66c01  lea r8, [rip + 0x16cd6fd]
1415ac963: b902000000  mov ecx, 2
1415ac968: e8b3d33401  call 0x1428f9d20
1415ac96d: 488906  mov qword ptr [rsi], rax
1415ac970: c6461a02  mov byte ptr [rsi + 0x1a], 2
1415ac974: 4c29ef  sub rdi, r13
1415ac977: 48c1ef05  shr rdi, 5
1415ac97b: 4c89e9  mov rcx, r13
1415ac97e: 4889fa  mov rdx, rdi
1415ac981: e8eac32dff  call 0x140888d70
1415ac986: 488b55c0  mov rdx, qword ptr [rbp - 0x40]
1415ac98a: 4885d2  test rdx, rdx
1415ac98d: 7413  je 0x1415ac9a2
1415ac98f: 488b4db0  mov rcx, qword ptr [rbp - 0x50]
1415ac993: 48c1e205  shl rdx, 5
1415ac997: 41b808000000  mov r8d, 8
1415ac99d: e87e7206ff  call 0x140613c20
1415ac9a2: 4889f0  mov rax, rsi
1415ac9a5: 0f287510  movaps xmm6, xmmword ptr [rbp + 0x10]
1415ac9a9: 4881c4a8000000  add rsp, 0xa8
1415ac9b0: 5b  pop rbx
1415ac9b1: 5f  pop rdi
1415ac9b2: 5e  pop rsi
1415ac9b3: 415c  pop r12
1415ac9b5: 415d  pop r13
1415ac9b7: 415e  pop r14
1415ac9b9: 415f  pop r15
1415ac9bb: 5d  pop rbp
1415ac9bc: c3  ret 
1415ac9bd: 0fb655f1  movzx edx, byte ptr [rbp - 0xf]
1415ac9c1: 4939fd  cmp r13, rdi
1415ac9c4: 7411  je 0x1415ac9d7
1415ac9c6: 4d8d6e40  lea r13, [r14 + 0x40]
1415ac9ca: 4c896db8  mov qword ptr [rbp - 0x48], r13
1415ac9ce: 410fb64620  movzx eax, byte ptr [r14 + 0x20]
1415ac9d3: 3c06  cmp al, 6
1415ac9d5: 7561  jne 0x1415aca38
1415ac9d7: 31c0  xor eax, eax
1415ac9d9: 4939fd  cmp r13, rdi
1415ac9dc: 0f8493000000  je 0x1415aca75
1415ac9e2: 4d8d7520  lea r14, [r13 + 0x20]
1415ac9e6: 4c8975b8  mov qword ptr [rbp - 0x48], r14
1415ac9ea: 410fb64d00  movzx ecx, byte ptr [r13]
1415ac9ef: 80f906  cmp cl, 6
1415ac9f2: 0f8480000000  je 0x1415aca78
1415ac9f8: 885507  mov byte ptr [rbp + 7], dl
1415ac9fb: 884dd0  mov byte ptr [rbp - 0x30], cl
1415ac9fe: 410f104501  movups xmm0, xmmword ptr [r13 + 1]
1415aca03: 410f104d10  movups xmm1, xmmword ptr [r13 + 0x10]
1415aca08: 0f1145d1  movups xmmword ptr [rbp - 0x2f], xmm0
1415aca0c: 0f114de0  movups xmmword ptr [rbp - 0x20], xmm1
1415aca10: 488d4df0  lea rcx, [rbp - 0x10]
1415aca14: 488d55d0  lea rdx, [rbp - 0x30]
1415aca18: 4989c5  mov r13, rax
1415aca1b: e8d00affff  call 0x14159d4f0
1415aca20: 807df001  cmp byte ptr [rbp - 0x10], 1
1415aca24: 0f840effffff  je 0x1415ac938
1415aca2a: 4c89e8  mov rax, r13
1415aca2d: 440fb645f1  movzx r8d, byte ptr [rbp - 0xf]
1415aca32: 0fb65507  movzx edx, byte ptr [rbp + 7]
1415aca36: eb43  jmp 0x1415aca7b
1415aca38: 885507  mov byte ptr [rbp + 7], dl
1415aca3b: 8845d0  mov byte ptr [rbp - 0x30], al
1415aca3e: 410f104621  movups xmm0, xmmword ptr [r14 + 0x21]
1415aca43: 410f104e30  movups xmm1, xmmword ptr [r14 + 0x30]
1415aca48: 0f1145d1  movups xmmword ptr [rbp - 0x2f], xmm0
1415aca4c: 0f114de0  movups xmmword ptr [rbp - 0x20], xmm1
1415aca50: 488d4dd0  lea rcx, [rbp - 0x30]
1415aca54: e83707ffff  call 0x14159d190
1415aca59: 4889c1  mov rcx, rax
1415aca5c: 4889d0  mov rax, rdx
1415aca5f: 4885c9  test rcx, rcx
1415aca62: 0f8505ffffff  jne 0x1415ac96d
1415aca68: 0fb65507  movzx edx, byte ptr [rbp + 7]
1415aca6c: 4939fd  cmp r13, rdi
1415aca6f: 0f856dffffff  jne 0x1415ac9e2
1415aca75: 4d89ee  mov r14, r13
1415aca78: 4531c0  xor r8d, r8d
1415aca7b: 4939fe  cmp r14, rdi
1415aca7e: 7458  je 0x1415acad8
1415aca80: 4d8d6e20  lea r13, [r14 + 0x20]
1415aca84: 4c896db8  mov qword ptr [rbp - 0x48], r13
1415aca88: 410fb60e  movzx ecx, byte ptr [r14]
1415aca8c: 80f906  cmp cl, 6
1415aca8f: 744a  je 0x1415acadb
1415aca91: 448945ac  mov dword ptr [rbp - 0x54], r8d
1415aca95: 885507  mov byte ptr [rbp + 7], dl
1415aca98: 884dd0  mov byte ptr [rbp - 0x30], cl
1415aca9b: 410f104601  movups xmm0, xmmword ptr [r14 + 1]
1415acaa0: 410f104e10  movups xmm1, xmmword ptr [r14 + 0x10]
1415acaa5: 0f1145d1  movups xmmword ptr [rbp - 0x2f], xmm0
1415acaa9: 0f114de0  movups xmmword ptr [rbp - 0x20], xmm1
1415acaad: 488d4df0  lea rcx, [rbp - 0x10]
1415acab1: 488d55d0  lea rdx, [rbp - 0x30]
1415acab5: 4989c6  mov r14, rax
1415acab8: e823df1400  call 0x1416fa9e0
1415acabd: 807df001  cmp byte ptr [rbp - 0x10], 1
1415acac1: 448b45ac  mov r8d, dword ptr [rbp - 0x54]
1415acac5: 0f8429feffff  je 0x1415ac8f4
1415acacb: 4c89f0  mov rax, r14
1415acace: 0fb64df1  movzx ecx, byte ptr [rbp - 0xf]
1415acad2: 0fb65507  movzx edx, byte ptr [rbp + 7]
1415acad6: eb05  jmp 0x1415acadd
1415acad8: 4d89f5  mov r13, r14
1415acadb: b101  mov cl, 1
1415acadd: 4c29ef  sub rdi, r13
1415acae0: 752e  jne 0x1415acb10
1415acae2: 488906  mov qword ptr [rsi], rax
1415acae5: f30f117608  movss dword ptr [rsi + 8], xmm6
1415acaea: 4489660c  mov dword ptr [rsi + 0xc], r12d
1415acaee: 4c897e10  mov qword ptr [rsi + 0x10], r15
1415acaf2: 885618  mov byte ptr [rsi + 0x18], dl
1415acaf5: 44884619  mov byte ptr [rsi + 0x19], r8b
1415acaf9: 884e1a  mov byte ptr [rsi + 0x1a], cl
1415acafc: 48c1ef05  shr rdi, 5
1415acb00: 4c89e9  mov rcx, r13
1415acb03: 4889fa  mov rdx, rdi
1415acb06: e865c22dff  call 0x140888d70
1415acb0b: e976feffff  jmp 0x1415ac986
1415acb10: 488d1539d56c01  lea rdx, [rip + 0x16cd539]
1415acb17: 4c8d0542d56c01  lea r8, [rip + 0x16cd542]
1415acb1e: 4889d9  mov rcx, rbx
1415acb21: e8fad13401  call 0x1428f9d20
1415acb26: 488906  mov qword ptr [rsi], rax
1415acb29: b102  mov cl, 2
1415acb2b: ebcc  jmp 0x1415acaf9
1415acb2d: 0f1f00  nop dword ptr [rax]
1415acb30: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415acb35: 55  push rbp
1415acb36: 4157  push r15
1415acb38: 4156  push r14
1415acb3a: 4155  push r13
1415acb3c: 4154  push r12
1415acb3e: 56  push rsi
1415acb3f: 57  push rdi
1415acb40: 53  push rbx
1415acb41: 4883ec38  sub rsp, 0x38
1415acb45: 488daa80000000  lea rbp, [rdx + 0x80]
1415acb4c: 0f29742420  movaps xmmword ptr [rsp + 0x20], xmm6
1415acb51: 488b55c0  mov rdx, qword ptr [rbp - 0x40]
1415acb55: 4885d2  test rdx, rdx
1415acb58: 7413  je 0x1415acb6d
1415acb5a: 488b4db0  mov rcx, qword ptr [rbp - 0x50]
1415acb5e: 48c1e205  shl rdx, 5
1415acb62: 41b808000000  mov r8d, 8
1415acb68: e8b37006ff  call 0x140613c20
1415acb6d: 0f28742420  movaps xmm6, xmmword ptr [rsp + 0x20]
1415acb72: 4883c438  add rsp, 0x38
1415acb76: 5b  pop rbx
1415acb77: 5f  pop rdi
1415acb78: 5e  pop rsi
1415acb79: 415c  pop r12
1415acb7b: 415d  pop r13
1415acb7d: 415e  pop r14
1415acb7f: 415f  pop r15
1415acb81: 5d  pop rbp
1415acb82: c3  ret 
1415acb83: 666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415acb90: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415acb95: 55  push rbp
1415acb96: 4157  push r15
1415acb98: 4156  push r14
1415acb9a: 4155  push r13
1415acb9c: 4154  push r12
1415acb9e: 56  push rsi
1415acb9f: 57  push rdi
1415acba0: 53  push rbx
1415acba1: 4883ec38  sub rsp, 0x38
1415acba5: 488daa80000000  lea rbp, [rdx + 0x80]
1415acbac: 0f29742420  movaps xmmword ptr [rsp + 0x20], xmm6
1415acbb1: 488b55c0  mov rdx, qword ptr [rbp - 0x40]
1415acbb5: 4885d2  test rdx, rdx
1415acbb8: 7413  je 0x1415acbcd
1415acbba: 488b4db0  mov rcx, qword ptr [rbp - 0x50]
1415acbbe: 48c1e205  shl rdx, 5
1415acbc2: 41b808000000  mov r8d, 8
1415acbc8: e8537006ff  call 0x140613c20
1415acbcd: 0f28742420  movaps xmm6, xmmword ptr [rsp + 0x20]
1415acbd2: 4883c438  add rsp, 0x38
1415acbd6: 5b  pop rbx
1415acbd7: 5f  pop rdi
1415acbd8: 5e  pop rsi
1415acbd9: 415c  pop r12
1415acbdb: 415d  pop r13
1415acbdd: 415e  pop r14
1415acbdf: 415f  pop r15
1415acbe1: 5d  pop rbp
1415acbe2: c3  ret 
1415acbe3: 666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415acbf0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415acbf5: 55  push rbp
1415acbf6: 4157  push r15
1415acbf8: 4156  push r14
1415acbfa: 4155  push r13
1415acbfc: 4154  push r12
1415acbfe: 56  push rsi
1415acbff: 57  push rdi
1415acc00: 53  push rbx
1415acc01: 4883ec38  sub rsp, 0x38
1415acc05: 488daa80000000  lea rbp, [rdx + 0x80]
1415acc0c: 0f29742420  movaps xmmword ptr [rsp + 0x20], xmm6
1415acc11: 488d4db0  lea rcx, [rbp - 0x50]
1415acc15: e846d92dff  call 0x14088a560
1415acc1a: 0f28742420  movaps xmm6, xmmword ptr [rsp + 0x20]
1415acc1f: 4883c438  add rsp, 0x38
1415acc23: 5b  pop rbx
1415acc24: 5f  pop rdi
1415acc25: 5e  pop rsi
1415acc26: 415c  pop r12
1415acc28: 415d  pop r13
1415acc2a: 415e  pop r14
1415acc2c: 415f  pop r15
1415acc2e: 5d  pop rbp
1415acc2f: c3  ret 
