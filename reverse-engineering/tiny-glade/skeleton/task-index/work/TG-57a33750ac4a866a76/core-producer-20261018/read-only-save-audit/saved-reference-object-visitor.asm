1415ce1c0: 55  push rbp
1415ce1c1: 4157  push r15
1415ce1c3: 4156  push r14
1415ce1c5: 4155  push r13
1415ce1c7: 4154  push r12
1415ce1c9: 56  push rsi
1415ce1ca: 57  push rdi
1415ce1cb: 53  push rbx
1415ce1cc: 4881ec08010000  sub rsp, 0x108
1415ce1d3: 488dac2480000000  lea rbp, [rsp + 0x80]
1415ce1db: 48c78580000000feffffff  mov qword ptr [rbp + 0x80], 0xfffffffffffffffe
1415ce1e6: 4989cd  mov r13, rcx
1415ce1e9: 488b4210  mov rax, qword ptr [rdx + 0x10]
1415ce1ed: 48894520  mov qword ptr [rbp + 0x20], rax
1415ce1f1: 4c8d75a0  lea r14, [rbp - 0x60]
1415ce1f5: 4c89f1  mov rcx, r14
1415ce1f8: e8d39e5bff  call 0x140b880d0
1415ce1fd: 488d5d41  lea rbx, [rbp + 0x41]
1415ce201: c7457400000000  mov dword ptr [rbp + 0x74], 0
1415ce208: b001  mov al, 1
1415ce20a: 894578  mov dword ptr [rbp + 0x78], eax
1415ce20d: 4c8d6508  lea r12, [rbp + 8]
1415ce211: 4c8d3de4d66a01  lea r15, [rip + 0x16ad6e4]
1415ce218: c7457c00000000  mov dword ptr [rbp + 0x7c], 0
1415ce21f: 90  nop
1415ce220: 4c89e1  mov rcx, r12
1415ce223: 4c89f2  mov rdx, r14
1415ce226: e825e2e6ff  call 0x14143c450
1415ce22b: 488b4508  mov rax, qword ptr [rbp + 8]
1415ce22f: 4885c0  test rax, rax
1415ce232: 0f8436020000  je 0x1415ce46e
1415ce238: 488b4d18  mov rcx, qword ptr [rbp + 0x18]
1415ce23c: 488d1449  lea rdx, [rcx + rcx*2]
1415ce240: 4c8b84d068010000  mov r8, qword ptr [rax + rdx*8 + 0x168]
1415ce248: 4c8b8cd070010000  mov r9, qword ptr [rax + rdx*8 + 0x170]
1415ce250: 4c894d28  mov qword ptr [rbp + 0x28], r9
1415ce254: 488bbcd078010000  mov rdi, qword ptr [rax + rdx*8 + 0x178]
1415ce25c: 48c1e105  shl rcx, 5
1415ce260: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
1415ce264: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
1415ce269: 0f294540  movaps xmmword ptr [rbp + 0x40], xmm0
1415ce26d: 0f294d50  movaps xmmword ptr [rbp + 0x50], xmm1
1415ce271: 4c89c0  mov rax, r8
1415ce274: 48f7d8  neg rax
1415ce277: 0f80f1010000  jo 0x1415ce46e
1415ce27d: 4c894568  mov qword ptr [rbp + 0x68], r8
1415ce281: 0fb645e8  movzx eax, byte ptr [rbp - 0x18]
1415ce285: 4883f806  cmp rax, 6
1415ce289: 0f871d010000  ja 0x1415ce3ac
1415ce28f: 49630487  movsxd rax, dword ptr [r15 + rax*4]
1415ce293: 4c01f8  add rax, r15
1415ce296: ffe0  jmp rax
1415ce298: 0f284540  movaps xmm0, xmmword ptr [rbp + 0x40]
1415ce29c: 0f284d50  movaps xmm1, xmmword ptr [rbp + 0x50]
1415ce2a0: 488d45e8  lea rax, [rbp - 0x18]
1415ce2a4: 0f114810  movups xmmword ptr [rax + 0x10], xmm1
1415ce2a8: 0f1100  movups xmmword ptr [rax], xmm0
1415ce2ab: 40b602  mov sil, 2
1415ce2ae: 4883ff0b  cmp rdi, 0xb
1415ce2b2: 488b5568  mov rdx, qword ptr [rbp + 0x68]
1415ce2b6: 488b4d28  mov rcx, qword ptr [rbp + 0x28]
1415ce2ba: 753b  jne 0x1415ce2f7
1415ce2bc: 488b01  mov rax, qword ptr [rcx]
1415ce2bf: 48330559fd5101  xor rax, qword ptr [rip + 0x151fd59]
1415ce2c6: 4c8b4103  mov r8, qword ptr [rcx + 3]
1415ce2ca: 4c330551fd5101  xor r8, qword ptr [rip + 0x151fd51]
1415ce2d1: 4909c0  or r8, rax
1415ce2d4: 7476  je 0x1415ce34c
1415ce2d6: 488b01  mov rax, qword ptr [rcx]
1415ce2d9: 4c8b4103  mov r8, qword ptr [rcx + 3]
1415ce2dd: 483305c8fd5101  xor rax, qword ptr [rip + 0x151fdc8]
1415ce2e4: 4c3305c4fd5101  xor r8, qword ptr [rip + 0x151fdc4]
1415ce2eb: 4909c0  or r8, rax
1415ce2ee: 0f94c0  sete al
1415ce2f1: 40b602  mov sil, 2
1415ce2f4: 4028c6  sub sil, al
1415ce2f7: 4885d2  test rdx, rdx
1415ce2fa: 740b  je 0x1415ce307
1415ce2fc: 41b801000000  mov r8d, 1
1415ce302: e8195904ff  call 0x140613c20
1415ce307: 4080fe02  cmp sil, 2
1415ce30b: 0f85a9000000  jne 0x1415ce3ba
1415ce311: 0fb645e8  movzx eax, byte ptr [rbp - 0x18]
1415ce315: c645e806  mov byte ptr [rbp - 0x18], 6
1415ce319: 3c06  cmp al, 6
1415ce31b: 0f8497010000  je 0x1415ce4b8
1415ce321: 884540  mov byte ptr [rbp + 0x40], al
1415ce324: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415ce329: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415ce32e: 0f114b0f  movups xmmword ptr [rbx + 0xf], xmm1
1415ce332: 0f1103  movups xmmword ptr [rbx], xmm0
1415ce335: 488d4d40  lea rcx, [rbp + 0x40]
1415ce339: e80256fdff  call 0x1415a3940
1415ce33e: 4885c0  test rax, rax
1415ce341: 0f84d9feffff  je 0x1415ce220
1415ce347: e990010000  jmp 0x1415ce4dc
1415ce34c: 31f6  xor esi, esi
1415ce34e: 4885d2  test rdx, rdx
1415ce351: 75a9  jne 0x1415ce2fc
1415ce353: ebb2  jmp 0x1415ce307
1415ce355: 488b55f0  mov rdx, qword ptr [rbp - 0x10]
1415ce359: 4885d2  test rdx, rdx
1415ce35c: 0f8436ffffff  je 0x1415ce298
1415ce362: 488b4df8  mov rcx, qword ptr [rbp - 8]
1415ce366: 41b801000000  mov r8d, 1
1415ce36c: e8af5804ff  call 0x140613c20
1415ce371: e922ffffff  jmp 0x1415ce298
1415ce376: 488b4df8  mov rcx, qword ptr [rbp - 8]
1415ce37a: 488b5500  mov rdx, qword ptr [rbp]
1415ce37e: 48894d30  mov qword ptr [rbp + 0x30], rcx
1415ce382: e8e9a92bff  call 0x140888d70
1415ce387: 488b55f0  mov rdx, qword ptr [rbp - 0x10]
1415ce38b: 4885d2  test rdx, rdx
1415ce38e: 0f8404ffffff  je 0x1415ce298
1415ce394: 48c1e205  shl rdx, 5
1415ce398: 41b808000000  mov r8d, 8
1415ce39e: 488b4d30  mov rcx, qword ptr [rbp + 0x30]
1415ce3a2: e8795804ff  call 0x140613c20
1415ce3a7: e9ecfeffff  jmp 0x1415ce298
1415ce3ac: 488d4df0  lea rcx, [rbp - 0x10]
1415ce3b0: e88bab2cff  call 0x140898f40
1415ce3b5: e9defeffff  jmp 0x1415ce298
1415ce3ba: 400fb6c6  movzx eax, sil
1415ce3be: 83f801  cmp eax, 1
1415ce3c1: 7555  jne 0x1415ce418
1415ce3c3: f6457801  test byte ptr [rbp + 0x78], 1
1415ce3c7: 0f8476010000  je 0x1415ce543
1415ce3cd: 0fb645e8  movzx eax, byte ptr [rbp - 0x18]
1415ce3d1: c645e806  mov byte ptr [rbp - 0x18], 6
1415ce3d5: 3c06  cmp al, 6
1415ce3d7: 0f8447010000  je 0x1415ce524
1415ce3dd: 884540  mov byte ptr [rbp + 0x40], al
1415ce3e0: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415ce3e5: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415ce3ea: 0f114b0f  movups xmmword ptr [rbx + 0xf], xmm1
1415ce3ee: 0f1103  movups xmmword ptr [rbx], xmm0
1415ce3f1: 488d4d40  lea rcx, [rbp + 0x40]
1415ce3f5: e846effcff  call 0x14159d340
1415ce3fa: 48895538  mov qword ptr [rbp + 0x38], rdx
1415ce3fe: 4885c0  test rax, rax
1415ce401: 0f8530010000  jne 0x1415ce537
1415ce407: b001  mov al, 1
1415ce409: 894574  mov dword ptr [rbp + 0x74], eax
1415ce40c: c7457800000000  mov dword ptr [rbp + 0x78], 0
1415ce413: e908feffff  jmp 0x1415ce220
1415ce418: 837d7c00  cmp dword ptr [rbp + 0x7c], 0
1415ce41c: 0f8534010000  jne 0x1415ce556
1415ce422: 0fb645e8  movzx eax, byte ptr [rbp - 0x18]
1415ce426: c645e806  mov byte ptr [rbp - 0x18], 6
1415ce42a: 3c06  cmp al, 6
1415ce42c: 0f84df000000  je 0x1415ce511
1415ce432: 884540  mov byte ptr [rbp + 0x40], al
1415ce435: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415ce43a: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415ce43f: 0f114b0f  movups xmmword ptr [rbx + 0xf], xmm1
1415ce443: 0f1103  movups xmmword ptr [rbx], xmm0
1415ce446: 4c89e1  mov rcx, r12
1415ce449: 488d5540  lea rdx, [rbp + 0x40]
1415ce44d: e89ec4fcff  call 0x14159a8f0
1415ce452: f6450801  test byte ptr [rbp + 8], 1
1415ce456: 0f85e1000000  jne 0x1415ce53d
1415ce45c: 8b450c  mov eax, dword ptr [rbp + 0xc]
1415ce45f: 894564  mov dword ptr [rbp + 0x64], eax
1415ce462: c7457c01000000  mov dword ptr [rbp + 0x7c], 1
1415ce469: e9b2fdffff  jmp 0x1415ce220
1415ce46e: f6457c01  test byte ptr [rbp + 0x7c], 1
1415ce472: 7431  je 0x1415ce4a5
1415ce474: f6457401  test byte ptr [rbp + 0x74], 1
1415ce478: 7451  je 0x1415ce4cb
1415ce47a: 48837de000  cmp qword ptr [rbp - 0x20], 0
1415ce47f: 0f85e7000000  jne 0x1415ce56c
1415ce485: 488b4538  mov rax, qword ptr [rbp + 0x38]
1415ce489: 49894508  mov qword ptr [r13 + 8], rax
1415ce48d: 8b4564  mov eax, dword ptr [rbp + 0x64]
1415ce490: 41894510  mov dword ptr [r13 + 0x10], eax
1415ce494: 31c0  xor eax, eax
1415ce496: 49894500  mov qword ptr [r13], rax
1415ce49a: 488d4da0  lea rcx, [rbp - 0x60]
1415ce49e: e84d992bff  call 0x140887df0
1415ce4a3: eb4c  jmp 0x1415ce4f1
1415ce4a5: 488d0d9cc56a01  lea rcx, [rip + 0x16ac59c]
1415ce4ac: ba0b000000  mov edx, 0xb
1415ce4b1: e80ab73201  call 0x1428f9bc0
1415ce4b6: eb24  jmp 0x1415ce4dc
1415ce4b8: 488d0da9bd6a01  lea rcx, [rip + 0x16abda9]
1415ce4bf: ba10000000  mov edx, 0x10
1415ce4c4: e837bb3201  call 0x1428fa000
1415ce4c9: eb11  jmp 0x1415ce4dc
1415ce4cb: 488d0de6c56a01  lea rcx, [rip + 0x16ac5e6]
1415ce4d2: ba0b000000  mov edx, 0xb
1415ce4d7: e8e4b63201  call 0x1428f9bc0
1415ce4dc: 49894508  mov qword ptr [r13 + 8], rax
1415ce4e0: 49c7450001000000  mov qword ptr [r13], 1
1415ce4e8: 488d4da0  lea rcx, [rbp - 0x60]
1415ce4ec: e8ff982bff  call 0x140887df0
1415ce4f1: 488d4de8  lea rcx, [rbp - 0x18]
1415ce4f5: e836ff2bff  call 0x14088e430
1415ce4fa: 4c89e8  mov rax, r13
1415ce4fd: 4881c408010000  add rsp, 0x108
1415ce504: 5b  pop rbx
1415ce505: 5f  pop rdi
1415ce506: 5e  pop rsi
1415ce507: 415c  pop r12
1415ce509: 415d  pop r13
1415ce50b: 415e  pop r14
1415ce50d: 415f  pop r15
1415ce50f: 5d  pop rbp
1415ce510: c3  ret
1415ce511: 488d0d50bd6a01  lea rcx, [rip + 0x16abd50]
1415ce518: ba10000000  mov edx, 0x10
1415ce51d: e8deba3201  call 0x1428fa000
1415ce522: ebb8  jmp 0x1415ce4dc
1415ce524: 488d0d3dbd6a01  lea rcx, [rip + 0x16abd3d]
1415ce52b: ba10000000  mov edx, 0x10
1415ce530: e8cbba3201  call 0x1428fa000
1415ce535: eba5  jmp 0x1415ce4dc
1415ce537: 488b4538  mov rax, qword ptr [rbp + 0x38]
1415ce53b: eb9f  jmp 0x1415ce4dc
1415ce53d: 488b4510  mov rax, qword ptr [rbp + 0x10]
1415ce541: eb99  jmp 0x1415ce4dc
1415ce543: 488d0d6ec56a01  lea rcx, [rip + 0x16ac56e]
1415ce54a: ba0b000000  mov edx, 0xb
1415ce54f: e85cb83201  call 0x1428f9db0
1415ce554: eb86  jmp 0x1415ce4dc
1415ce556: 488d0debc46a01  lea rcx, [rip + 0x16ac4eb]
1415ce55d: ba0b000000  mov edx, 0xb
1415ce562: e849b83201  call 0x1428f9db0
1415ce567: e970ffffff  jmp 0x1415ce4dc
1415ce56c: 488d1525bb6a01  lea rdx, [rip + 0x16abb25]
1415ce573: 4c8d05e6ba6a01  lea r8, [rip + 0x16abae6]
1415ce57a: 488b4d20  mov rcx, qword ptr [rbp + 0x20]
1415ce57e: e89db73201  call 0x1428f9d20
1415ce583: 49894508  mov qword ptr [r13 + 8], rax
1415ce587: b801000000  mov eax, 1
1415ce58c: e905ffffff  jmp 0x1415ce496
1415ce591: 6666666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415ce5a0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415ce5a5: 55  push rbp
1415ce5a6: 4157  push r15
1415ce5a8: 4156  push r14
1415ce5aa: 4155  push r13
1415ce5ac: 4154  push r12
1415ce5ae: 56  push rsi
1415ce5af: 57  push rdi
1415ce5b0: 53  push rbx
1415ce5b1: 4883ec28  sub rsp, 0x28
1415ce5b5: 488daa80000000  lea rbp, [rdx + 0x80]
1415ce5bc: 488b55f0  mov rdx, qword ptr [rbp - 0x10]
1415ce5c0: 4885d2  test rdx, rdx
1415ce5c3: 7413  je 0x1415ce5d8
1415ce5c5: 48c1e205  shl rdx, 5
1415ce5c9: 41b808000000  mov r8d, 8
1415ce5cf: 488b4d30  mov rcx, qword ptr [rbp + 0x30]
1415ce5d3: e8485604ff  call 0x140613c20
1415ce5d8: 90  nop
1415ce5d9: 4883c428  add rsp, 0x28
1415ce5dd: 5b  pop rbx
1415ce5de: 5f  pop rdi
1415ce5df: 5e  pop rsi
1415ce5e0: 415c  pop r12
1415ce5e2: 415d  pop r13
1415ce5e4: 415e  pop r14
1415ce5e6: 415f  pop r15
1415ce5e8: 5d  pop rbp
1415ce5e9: c3  ret
1415ce5ea: 660f1f440000  nop word ptr [rax + rax]
1415ce5f0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415ce5f5: 55  push rbp
1415ce5f6: 4157  push r15
1415ce5f8: 4156  push r14
1415ce5fa: 4155  push r13
1415ce5fc: 4154  push r12
1415ce5fe: 56  push rsi
1415ce5ff: 57  push rdi
1415ce600: 53  push rbx
1415ce601: 4883ec28  sub rsp, 0x28
1415ce605: 488daa80000000  lea rbp, [rdx + 0x80]
1415ce60c: 0f284540  movaps xmm0, xmmword ptr [rbp + 0x40]
1415ce610: 0f284d50  movaps xmm1, xmmword ptr [rbp + 0x50]
1415ce614: 488d45e8  lea rax, [rbp - 0x18]
1415ce618: 0f114810  movups xmmword ptr [rax + 0x10], xmm1
1415ce61c: 0f1100  movups xmmword ptr [rax], xmm0
1415ce61f: 48837d6800  cmp qword ptr [rbp + 0x68], 0
1415ce624: 7413  je 0x1415ce639
1415ce626: 41b801000000  mov r8d, 1
1415ce62c: 488b4d28  mov rcx, qword ptr [rbp + 0x28]
1415ce630: 488b5568  mov rdx, qword ptr [rbp + 0x68]
1415ce634: e8e75504ff  call 0x140613c20
1415ce639: 90  nop
1415ce63a: 4883c428  add rsp, 0x28
1415ce63e: 5b  pop rbx
1415ce63f: 5f  pop rdi
1415ce640: 5e  pop rsi
1415ce641: 415c  pop r12
1415ce643: 415d  pop r13
1415ce645: 415e  pop r14
1415ce647: 415f  pop r15
1415ce649: 5d  pop rbp
1415ce64a: c3  ret
1415ce64b: 0f1f440000  nop dword ptr [rax + rax]
1415ce650: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415ce655: 55  push rbp
1415ce656: 4157  push r15
1415ce658: 4156  push r14
1415ce65a: 4155  push r13
1415ce65c: 4154  push r12
1415ce65e: 56  push rsi
1415ce65f: 57  push rdi
1415ce660: 53  push rbx
1415ce661: 4883ec28  sub rsp, 0x28
1415ce665: 488daa80000000  lea rbp, [rdx + 0x80]
1415ce66c: 488d4da0  lea rcx, [rbp - 0x60]
1415ce670: e88bbe2bff  call 0x14088a500
1415ce675: 90  nop
1415ce676: 4883c428  add rsp, 0x28
1415ce67a: 5b  pop rbx
1415ce67b: 5f  pop rdi
1415ce67c: 5e  pop rsi
1415ce67d: 415c  pop r12
1415ce67f: 415d  pop r13
1415ce681: 415e  pop r14
1415ce683: 415f  pop r15
1415ce685: 5d  pop rbp
1415ce686: c3  ret
1415ce687: 660f1f840000000000  nop word ptr [rax + rax]
1415ce690: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415ce695: 55  push rbp
1415ce696: 4157  push r15
1415ce698: 4156  push r14
1415ce69a: 4155  push r13
1415ce69c: 4154  push r12
1415ce69e: 56  push rsi
1415ce69f: 57  push rdi
1415ce6a0: 53  push rbx
1415ce6a1: 4883ec28  sub rsp, 0x28
1415ce6a5: 488daa80000000  lea rbp, [rdx + 0x80]
1415ce6ac: 488d4de8  lea rcx, [rbp - 0x18]
1415ce6b0: e87bfd2bff  call 0x14088e430
1415ce6b5: 90  nop
1415ce6b6: 4883c428  add rsp, 0x28
1415ce6ba: 5b  pop rbx
1415ce6bb: 5f  pop rdi
1415ce6bc: 5e  pop rsi
1415ce6bd: 415c  pop r12
1415ce6bf: 415d  pop r13
1415ce6c1: 415e  pop r14
1415ce6c3: 415f  pop r15
1415ce6c5: 5d  pop rbp
1415ce6c6: c3  ret
1415ce6c7: 660f1f840000000000  nop word ptr [rax + rax]
1415ce6d0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415ce6d5: 55  push rbp
1415ce6d6: 4157  push r15
1415ce6d8: 4156  push r14
1415ce6da: 4155  push r13
1415ce6dc: 4154  push r12
1415ce6de: 56  push rsi
1415ce6df: 57  push rdi
1415ce6e0: 53  push rbx
1415ce6e1: 4883ec28  sub rsp, 0x28
1415ce6e5: 488daa80000000  lea rbp, [rdx + 0x80]
1415ce6ec: 488d4de8  lea rcx, [rbp - 0x18]
1415ce6f0: e83bfd2bff  call 0x14088e430
1415ce6f5: 90  nop
1415ce6f6: 4883c428  add rsp, 0x28
1415ce6fa: 5b  pop rbx
1415ce6fb: 5f  pop rdi
1415ce6fc: 5e  pop rsi
1415ce6fd: 415c  pop r12
1415ce6ff: 415d  pop r13
1415ce701: 415e  pop r14
1415ce703: 415f  pop r15
1415ce705: 5d  pop rbp
1415ce706: c3  ret
