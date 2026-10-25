1415b6310: 55  push rbp
1415b6311: 4157  push r15
1415b6313: 4156  push r14
1415b6315: 4155  push r13
1415b6317: 4154  push r12
1415b6319: 56  push rsi
1415b631a: 57  push rdi
1415b631b: 53  push rbx
1415b631c: 4881ec88010000  sub rsp, 0x188
1415b6323: 488dac2480000000  lea rbp, [rsp + 0x80]
1415b632b: 0f29b5f0000000  movaps xmmword ptr [rbp + 0xf0], xmm6
1415b6332: 48c785e8000000feffffff  mov qword ptr [rbp + 0xe8], 0xfffffffffffffffe
1415b633d: 48898dd8000000  mov qword ptr [rbp + 0xd8], rcx
1415b6344: 488b4210  mov rax, qword ptr [rdx + 0x10]
1415b6348: 48894558  mov qword ptr [rbp + 0x58], rax
1415b634c: 4c8d75e8  lea r14, [rbp - 0x18]
1415b6350: 4c89f1  mov rcx, r14
1415b6353: e8781d5dff  call 0x140b880d0
1415b6358: 4c8dbd81000000  lea r15, [rbp + 0x81]
1415b635f: c785cc00000000000000  mov dword ptr [rbp + 0xcc], 0
1415b6369: c685e700000002  mov byte ptr [rbp + 0xe7], 2
1415b6370: 4c8d6560  lea r12, [rbp + 0x60]
1415b6374: 4c8dad80000000  lea r13, [rbp + 0x80]
1415b637b: 488d3d264f6c01  lea rdi, [rip + 0x16c4f26]
1415b6382: 66666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415b6390: 4c89e1  mov rcx, r12
1415b6393: 4c89f2  mov rdx, r14
1415b6396: e8b560e8ff  call 0x14143c450
1415b639b: 488b4560  mov rax, qword ptr [rbp + 0x60]
1415b639f: 4885c0  test rax, rax
1415b63a2: 0f840a030000  je 0x1415b66b2
1415b63a8: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
1415b63ac: 488d1449  lea rdx, [rcx + rcx*2]
1415b63b0: 4c8b84d068010000  mov r8, qword ptr [rax + rdx*8 + 0x168]
1415b63b8: 4c8b8cd070010000  mov r9, qword ptr [rax + rdx*8 + 0x170]
1415b63c0: 4c898da8000000  mov qword ptr [rbp + 0xa8], r9
1415b63c7: 488b9cd078010000  mov rbx, qword ptr [rax + rdx*8 + 0x178]
1415b63cf: 48c1e105  shl rcx, 5
1415b63d3: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
1415b63d7: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
1415b63dc: 0f298580000000  movaps xmmword ptr [rbp + 0x80], xmm0
1415b63e3: 0f298d90000000  movaps xmmword ptr [rbp + 0x90], xmm1
1415b63ea: 4c89c0  mov rax, r8
1415b63ed: 48f7d8  neg rax
1415b63f0: 0f80bc020000  jo 0x1415b66b2
1415b63f6: 4c8985b0000000  mov qword ptr [rbp + 0xb0], r8
1415b63fd: 0fb64530  movzx eax, byte ptr [rbp + 0x30]
1415b6401: 4883f806  cmp rax, 6
1415b6405: 0f875f010000  ja 0x1415b656a
1415b640b: 48630487  movsxd rax, dword ptr [rdi + rax*4]
1415b640f: 4801f8  add rax, rdi
1415b6412: ffe0  jmp rax
1415b6414: 0f288580000000  movaps xmm0, xmmword ptr [rbp + 0x80]
1415b641b: 0f288d90000000  movaps xmm1, xmmword ptr [rbp + 0x90]
1415b6422: 488d4530  lea rax, [rbp + 0x30]
1415b6426: 0f114810  movups xmmword ptr [rax + 0x10], xmm1
1415b642a: 0f1100  movups xmmword ptr [rax], xmm0
1415b642d: 4883fb06  cmp rbx, 6
1415b6431: 0f8499000000  je 0x1415b64d0
1415b6437: 40b602  mov sil, 2
1415b643a: 4883fb05  cmp rbx, 5
1415b643e: 488b95b0000000  mov rdx, qword ptr [rbp + 0xb0]
1415b6445: 488b8da8000000  mov rcx, qword ptr [rbp + 0xa8]
1415b644c: 7522  jne 0x1415b6470
1415b644e: 8b01  mov eax, dword ptr [rcx]
1415b6450: 330500905101  xor eax, dword ptr [rip + 0x1519000]
1415b6456: 440fb64104  movzx r8d, byte ptr [rcx + 4]
1415b645b: 443205f88f5101  xor r8b, byte ptr [rip + 0x1518ff8]
1415b6462: 450fb6c0  movzx r8d, r8b
1415b6466: 4109c0  or r8d, eax
1415b6469: 400f95c6  setne sil
1415b646d: 4000f6  add sil, sil
1415b6470: 4885d2  test rdx, rdx
1415b6473: 740b  je 0x1415b6480
1415b6475: 41b801000000  mov r8d, 1
1415b647b: e8a0d705ff  call 0x140613c20
1415b6480: 4080fe02  cmp sil, 2
1415b6484: 0f85ee000000  jne 0x1415b6578
1415b648a: 0fb64530  movzx eax, byte ptr [rbp + 0x30]
1415b648e: c6453006  mov byte ptr [rbp + 0x30], 6
1415b6492: 3c06  cmp al, 6
1415b6494: 0f8469020000  je 0x1415b6703
1415b649a: 888580000000  mov byte ptr [rbp + 0x80], al
1415b64a0: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415b64a5: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415b64aa: 410f114f0f  movups xmmword ptr [r15 + 0xf], xmm1
1415b64af: 410f1107  movups xmmword ptr [r15], xmm0
1415b64b3: 4c89e9  mov rcx, r13
1415b64b6: e885d4feff  call 0x1415a3940
1415b64bb: 4885c0  test rax, rax
1415b64be: 0f84ccfeffff  je 0x1415b6390
1415b64c4: e915030000  jmp 0x1415b67de
1415b64c9: 0f1f8000000000  nop dword ptr [rax]
1415b64d0: 488b8da8000000  mov rcx, qword ptr [rbp + 0xa8]
1415b64d7: 8b01  mov eax, dword ptr [rcx]
1415b64d9: 33057c8f5101  xor eax, dword ptr [rip + 0x1518f7c]
1415b64df: 0fb75104  movzx edx, word ptr [rcx + 4]
1415b64e3: 663315758f5101  xor dx, word ptr [rip + 0x1518f75]
1415b64ea: 0fb7d2  movzx edx, dx
1415b64ed: 09c2  or edx, eax
1415b64ef: 0f94c0  sete al
1415b64f2: 40b602  mov sil, 2
1415b64f5: 4028c6  sub sil, al
1415b64f8: 488b95b0000000  mov rdx, qword ptr [rbp + 0xb0]
1415b64ff: 4885d2  test rdx, rdx
1415b6502: 0f856dffffff  jne 0x1415b6475
1415b6508: e973ffffff  jmp 0x1415b6480
1415b650d: 488b5538  mov rdx, qword ptr [rbp + 0x38]
1415b6511: 4885d2  test rdx, rdx
1415b6514: 0f84fafeffff  je 0x1415b6414
1415b651a: 488b4d40  mov rcx, qword ptr [rbp + 0x40]
1415b651e: 41b801000000  mov r8d, 1
1415b6524: e8f7d605ff  call 0x140613c20
1415b6529: e9e6feffff  jmp 0x1415b6414
1415b652e: 488b4d40  mov rcx, qword ptr [rbp + 0x40]
1415b6532: 488b5548  mov rdx, qword ptr [rbp + 0x48]
1415b6536: 48898da0000000  mov qword ptr [rbp + 0xa0], rcx
1415b653d: e82e282dff  call 0x140888d70
1415b6542: 488b5538  mov rdx, qword ptr [rbp + 0x38]
1415b6546: 4885d2  test rdx, rdx
1415b6549: 0f84c5feffff  je 0x1415b6414
1415b654f: 48c1e205  shl rdx, 5
1415b6553: 41b808000000  mov r8d, 8
1415b6559: 488b8da0000000  mov rcx, qword ptr [rbp + 0xa0]
1415b6560: e8bbd605ff  call 0x140613c20
1415b6565: e9aafeffff  jmp 0x1415b6414
1415b656a: 488d4d38  lea rcx, [rbp + 0x38]
1415b656e: e8cd292eff  call 0x140898f40
1415b6573: e99cfeffff  jmp 0x1415b6414
1415b6578: 400fb6c6  movzx eax, sil
1415b657c: 83f801  cmp eax, 1
1415b657f: 0f85ac000000  jne 0x1415b6631
1415b6585: 80bde700000002  cmp byte ptr [rbp + 0xe7], 2
1415b658c: 0f859c020000  jne 0x1415b682e
1415b6592: 0fb64530  movzx eax, byte ptr [rbp + 0x30]
1415b6596: c6453006  mov byte ptr [rbp + 0x30], 6
1415b659a: 3c06  cmp al, 6
1415b659c: 0f841e020000  je 0x1415b67c0
1415b65a2: 888580000000  mov byte ptr [rbp + 0x80], al
1415b65a8: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415b65ad: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415b65b2: 410f114f0f  movups xmmword ptr [r15 + 0xf], xmm1
1415b65b7: 410f1107  movups xmmword ptr [r15], xmm0
1415b65bb: 488d0506bb6d01  lea rax, [rip + 0x16dbb06]
1415b65c2: 4889442420  mov qword ptr [rsp + 0x20], rax
1415b65c7: 48c744242806000000  mov qword ptr [rsp + 0x28], 6
1415b65d0: 41b914000000  mov r9d, 0x14
1415b65d6: 4c89e1  mov rcx, r12
1415b65d9: 4c89ea  mov rdx, r13
1415b65dc: 4c8d0545bb6d01  lea r8, [rip + 0x16dbb45]
1415b65e3: e848a5feff  call 0x1415a0b30
1415b65e8: 0fb64d7a  movzx ecx, byte ptr [rbp + 0x7a]
1415b65ec: 488b4560  mov rax, qword ptr [rbp + 0x60]
1415b65f0: 488985d0000000  mov qword ptr [rbp + 0xd0], rax
1415b65f7: 888de7000000  mov byte ptr [rbp + 0xe7], cl
1415b65fd: 80f902  cmp cl, 2
1415b6600: 0f84df010000  je 0x1415b67e5
1415b6606: 488d4d68  lea rcx, [rbp + 0x68]
1415b660a: 0fb74110  movzx eax, word ptr [rcx + 0x10]
1415b660e: 668945e0  mov word ptr [rbp - 0x20], ax
1415b6612: 0f1001  movups xmm0, xmmword ptr [rcx]
1415b6615: 0f2945d0  movaps xmmword ptr [rbp - 0x30], xmm0
1415b6619: 8b4113  mov eax, dword ptr [rcx + 0x13]
1415b661c: 8985c4000000  mov dword ptr [rbp + 0xc4], eax
1415b6622: 0fb64117  movzx eax, byte ptr [rcx + 0x17]
1415b6626: 8885c8000000  mov byte ptr [rbp + 0xc8], al
1415b662c: e95ffdffff  jmp 0x1415b6390
1415b6631: 83bdcc00000000  cmp dword ptr [rbp + 0xcc], 0
1415b6638: 0f850a020000  jne 0x1415b6848
1415b663e: 0fb64530  movzx eax, byte ptr [rbp + 0x30]
1415b6642: c6453006  mov byte ptr [rbp + 0x30], 6
1415b6646: 3c06  cmp al, 6
1415b6648: 0f8458010000  je 0x1415b67a6
1415b664e: 888580000000  mov byte ptr [rbp + 0x80], al
1415b6654: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415b6659: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415b665e: 410f114f0f  movups xmmword ptr [r15 + 0xf], xmm1
1415b6663: 410f1107  movups xmmword ptr [r15], xmm0
1415b6667: 488d057a8d6d01  lea rax, [rip + 0x16d8d7a]
1415b666e: 4889442420  mov qword ptr [rsp + 0x20], rax
1415b6673: 48c744242802000000  mov qword ptr [rsp + 0x28], 2
1415b667c: 41b90e000000  mov r9d, 0xe
1415b6682: 4c89e1  mov rcx, r12
1415b6685: 4c89ea  mov rdx, r13
1415b6688: 4c8d05498d6d01  lea r8, [rip + 0x16d8d49]
1415b668f: e8acb8feff  call 0x1415a1f40
1415b6694: f6456001  test byte ptr [rbp + 0x60], 1
1415b6698: 0f853c010000  jne 0x1415b67da
1415b669e: f20f107564  movsd xmm6, qword ptr [rbp + 0x64]
1415b66a3: c785cc00000001000000  mov dword ptr [rbp + 0xcc], 1
1415b66ad: e9defcffff  jmp 0x1415b6390
1415b66b2: f685cc00000001  test byte ptr [rbp + 0xcc], 1
1415b66b9: 742b  je 0x1415b66e6
1415b66bb: 440fb685e7000000  movzx r8d, byte ptr [rbp + 0xe7]
1415b66c3: 4180f802  cmp r8b, 2
1415b66c7: 7557  jne 0x1415b6720
1415b66c9: 488d0d9d486c01  lea rcx, [rip + 0x16c489d]
1415b66d0: ba06000000  mov edx, 6
1415b66d5: e8e6343401  call 0x1428f9bc0
1415b66da: 488985d0000000  mov qword ptr [rbp + 0xd0], rax
1415b66e1: e9ff000000  jmp 0x1415b67e5
1415b66e6: 488d0d7b486c01  lea rcx, [rip + 0x16c487b]
1415b66ed: ba05000000  mov edx, 5
1415b66f2: e8c9343401  call 0x1428f9bc0
1415b66f7: 488985d0000000  mov qword ptr [rbp + 0xd0], rax
1415b66fe: e9e2000000  jmp 0x1415b67e5
1415b6703: 488d0d5e3b6c01  lea rcx, [rip + 0x16c3b5e]
1415b670a: ba10000000  mov edx, 0x10
1415b670f: e8ec383401  call 0x1428fa000
1415b6714: 488985d0000000  mov qword ptr [rbp + 0xd0], rax
1415b671b: e9c5000000  jmp 0x1415b67e5
1415b6720: 0fb745e0  movzx eax, word ptr [rbp - 0x20]
1415b6724: 668945c0  mov word ptr [rbp - 0x40], ax
1415b6728: 0f2845d0  movaps xmm0, xmmword ptr [rbp - 0x30]
1415b672c: 0f2945b0  movaps xmmword ptr [rbp - 0x50], xmm0
1415b6730: 8b85c4000000  mov eax, dword ptr [rbp + 0xc4]
1415b6736: 8985bc000000  mov dword ptr [rbp + 0xbc], eax
1415b673c: 0fb685c8000000  movzx eax, byte ptr [rbp + 0xc8]
1415b6743: 8885c0000000  mov byte ptr [rbp + 0xc0], al
1415b6749: 0f137550  movlps qword ptr [rbp + 0x50], xmm6
1415b674d: 48837d2800  cmp qword ptr [rbp + 0x28], 0
1415b6752: 0f850a010000  jne 0x1415b6862
1415b6758: 488b4550  mov rax, qword ptr [rbp + 0x50]
1415b675c: 0fb74dc0  movzx ecx, word ptr [rbp - 0x40]
1415b6760: 488b95d8000000  mov rdx, qword ptr [rbp + 0xd8]
1415b6767: 66894a20  mov word ptr [rdx + 0x20], cx
1415b676b: 0f2845b0  movaps xmm0, xmmword ptr [rbp - 0x50]
1415b676f: 0f114210  movups xmmword ptr [rdx + 0x10], xmm0
1415b6773: 8b8dbc000000  mov ecx, dword ptr [rbp + 0xbc]
1415b6779: 894a23  mov dword ptr [rdx + 0x23], ecx
1415b677c: 0fb68dc0000000  movzx ecx, byte ptr [rbp + 0xc0]
1415b6783: 884a27  mov byte ptr [rdx + 0x27], cl
1415b6786: 488902  mov qword ptr [rdx], rax
1415b6789: 4889d0  mov rax, rdx
1415b678c: 488b8dd0000000  mov rcx, qword ptr [rbp + 0xd0]
1415b6793: 48894a08  mov qword ptr [rdx + 8], rcx
1415b6797: 44884022  mov byte ptr [rax + 0x22], r8b
1415b679b: 488d4de8  lea rcx, [rbp - 0x18]
1415b679f: e84c162dff  call 0x140887df0
1415b67a4: eb5d  jmp 0x1415b6803
1415b67a6: 488d0dbb3a6c01  lea rcx, [rip + 0x16c3abb]
1415b67ad: ba10000000  mov edx, 0x10
1415b67b2: e849383401  call 0x1428fa000
1415b67b7: 488985d0000000  mov qword ptr [rbp + 0xd0], rax
1415b67be: eb25  jmp 0x1415b67e5
1415b67c0: 488d0da13a6c01  lea rcx, [rip + 0x16c3aa1]
1415b67c7: ba10000000  mov edx, 0x10
1415b67cc: e82f383401  call 0x1428fa000
1415b67d1: 488985d0000000  mov qword ptr [rbp + 0xd0], rax
1415b67d8: eb0b  jmp 0x1415b67e5
1415b67da: 488b4568  mov rax, qword ptr [rbp + 0x68]
1415b67de: 488985d0000000  mov qword ptr [rbp + 0xd0], rax
1415b67e5: 488b85d8000000  mov rax, qword ptr [rbp + 0xd8]
1415b67ec: 488b8dd0000000  mov rcx, qword ptr [rbp + 0xd0]
1415b67f3: 488908  mov qword ptr [rax], rcx
1415b67f6: c6402202  mov byte ptr [rax + 0x22], 2
1415b67fa: 488d4de8  lea rcx, [rbp - 0x18]
1415b67fe: e8ed152dff  call 0x140887df0
1415b6803: 488d4d30  lea rcx, [rbp + 0x30]
1415b6807: e8247c2dff  call 0x14088e430
1415b680c: 488b85d8000000  mov rax, qword ptr [rbp + 0xd8]
1415b6813: 0f28b5f0000000  movaps xmm6, xmmword ptr [rbp + 0xf0]
1415b681a: 4881c488010000  add rsp, 0x188
1415b6821: 5b  pop rbx
1415b6822: 5f  pop rdi
1415b6823: 5e  pop rsi
1415b6824: 415c  pop r12
1415b6826: 415d  pop r13
1415b6828: 415e  pop r14
1415b682a: 415f  pop r15
1415b682c: 5d  pop rbp
1415b682d: c3  ret 
1415b682e: 488d0d38476c01  lea rcx, [rip + 0x16c4738]
1415b6835: ba06000000  mov edx, 6
1415b683a: e871353401  call 0x1428f9db0
1415b683f: 488985d0000000  mov qword ptr [rbp + 0xd0], rax
1415b6846: eb9d  jmp 0x1415b67e5
1415b6848: 488d0d19476c01  lea rcx, [rip + 0x16c4719]
1415b684f: ba05000000  mov edx, 5
1415b6854: e857353401  call 0x1428f9db0
1415b6859: 488985d0000000  mov qword ptr [rbp + 0xd0], rax
1415b6860: eb83  jmp 0x1415b67e5
1415b6862: 488d152f386c01  lea rdx, [rip + 0x16c382f]
1415b6869: 4c8d05f0376c01  lea r8, [rip + 0x16c37f0]
1415b6870: 488b4d58  mov rcx, qword ptr [rbp + 0x58]
1415b6874: e8a7343401  call 0x1428f9d20
1415b6879: 488b8dd8000000  mov rcx, qword ptr [rbp + 0xd8]
1415b6880: 488901  mov qword ptr [rcx], rax
1415b6883: 4889c8  mov rax, rcx
1415b6886: 41b002  mov r8b, 2
1415b6889: e909ffffff  jmp 0x1415b6797
1415b688e: 6690  nop 
1415b6890: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b6895: 55  push rbp
1415b6896: 4157  push r15
1415b6898: 4156  push r14
1415b689a: 4155  push r13
1415b689c: 4154  push r12
1415b689e: 56  push rsi
1415b689f: 57  push rdi
1415b68a0: 53  push rbx
1415b68a1: 4883ec48  sub rsp, 0x48
1415b68a5: 488daa80000000  lea rbp, [rdx + 0x80]
1415b68ac: 0f29742430  movaps xmmword ptr [rsp + 0x30], xmm6
1415b68b1: 488d4d30  lea rcx, [rbp + 0x30]
1415b68b5: e8767b2dff  call 0x14088e430
1415b68ba: 0f28742430  movaps xmm6, xmmword ptr [rsp + 0x30]
1415b68bf: 4883c448  add rsp, 0x48
1415b68c3: 5b  pop rbx
1415b68c4: 5f  pop rdi
1415b68c5: 5e  pop rsi
1415b68c6: 415c  pop r12
1415b68c8: 415d  pop r13
1415b68ca: 415e  pop r14
1415b68cc: 415f  pop r15
1415b68ce: 5d  pop rbp
1415b68cf: c3  ret 
1415b68d0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b68d5: 55  push rbp
1415b68d6: 4157  push r15
1415b68d8: 4156  push r14
1415b68da: 4155  push r13
1415b68dc: 4154  push r12
1415b68de: 56  push rsi
1415b68df: 57  push rdi
1415b68e0: 53  push rbx
1415b68e1: 4883ec48  sub rsp, 0x48
1415b68e5: 488daa80000000  lea rbp, [rdx + 0x80]
1415b68ec: 0f29742430  movaps xmmword ptr [rsp + 0x30], xmm6
1415b68f1: 488d4d30  lea rcx, [rbp + 0x30]
1415b68f5: e8367b2dff  call 0x14088e430
1415b68fa: 0f28742430  movaps xmm6, xmmword ptr [rsp + 0x30]
1415b68ff: 4883c448  add rsp, 0x48
1415b6903: 5b  pop rbx
1415b6904: 5f  pop rdi
1415b6905: 5e  pop rsi
1415b6906: 415c  pop r12
1415b6908: 415d  pop r13
1415b690a: 415e  pop r14
1415b690c: 415f  pop r15
1415b690e: 5d  pop rbp
1415b690f: c3  ret 
1415b6910: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b6915: 55  push rbp
1415b6916: 4157  push r15
1415b6918: 4156  push r14
1415b691a: 4155  push r13
1415b691c: 4154  push r12
1415b691e: 56  push rsi
1415b691f: 57  push rdi
1415b6920: 53  push rbx
1415b6921: 4883ec48  sub rsp, 0x48
1415b6925: 488daa80000000  lea rbp, [rdx + 0x80]
1415b692c: 0f29742430  movaps xmmword ptr [rsp + 0x30], xmm6
1415b6931: 488b5538  mov rdx, qword ptr [rbp + 0x38]
1415b6935: 4885d2  test rdx, rdx
1415b6938: 7416  je 0x1415b6950
1415b693a: 48c1e205  shl rdx, 5
1415b693e: 41b808000000  mov r8d, 8
1415b6944: 488b8da0000000  mov rcx, qword ptr [rbp + 0xa0]
1415b694b: e8d0d205ff  call 0x140613c20
1415b6950: 0f28742430  movaps xmm6, xmmword ptr [rsp + 0x30]
1415b6955: 4883c448  add rsp, 0x48
1415b6959: 5b  pop rbx
1415b695a: 5f  pop rdi
1415b695b: 5e  pop rsi
1415b695c: 415c  pop r12
1415b695e: 415d  pop r13
1415b6960: 415e  pop r14
1415b6962: 415f  pop r15
1415b6964: 5d  pop rbp
1415b6965: c3  ret 
1415b6966: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415b6970: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b6975: 55  push rbp
1415b6976: 4157  push r15
1415b6978: 4156  push r14
1415b697a: 4155  push r13
1415b697c: 4154  push r12
1415b697e: 56  push rsi
1415b697f: 57  push rdi
1415b6980: 53  push rbx
1415b6981: 4883ec48  sub rsp, 0x48
1415b6985: 488daa80000000  lea rbp, [rdx + 0x80]
1415b698c: 0f29742430  movaps xmmword ptr [rsp + 0x30], xmm6
1415b6991: 0f288580000000  movaps xmm0, xmmword ptr [rbp + 0x80]
1415b6998: 0f288d90000000  movaps xmm1, xmmword ptr [rbp + 0x90]
1415b699f: 488d4530  lea rax, [rbp + 0x30]
1415b69a3: 0f114810  movups xmmword ptr [rax + 0x10], xmm1
1415b69a7: 0f1100  movups xmmword ptr [rax], xmm0
1415b69aa: 4883bdb000000000  cmp qword ptr [rbp + 0xb0], 0
1415b69b2: 7419  je 0x1415b69cd
1415b69b4: 41b801000000  mov r8d, 1
1415b69ba: 488b8da8000000  mov rcx, qword ptr [rbp + 0xa8]
1415b69c1: 488b95b0000000  mov rdx, qword ptr [rbp + 0xb0]
1415b69c8: e853d205ff  call 0x140613c20
1415b69cd: 0f28742430  movaps xmm6, xmmword ptr [rsp + 0x30]
1415b69d2: 4883c448  add rsp, 0x48
1415b69d6: 5b  pop rbx
1415b69d7: 5f  pop rdi
1415b69d8: 5e  pop rsi
1415b69d9: 415c  pop r12
1415b69db: 415d  pop r13
1415b69dd: 415e  pop r14
1415b69df: 415f  pop r15
1415b69e1: 5d  pop rbp
1415b69e2: c3  ret 
1415b69e3: 666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415b69f0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b69f5: 55  push rbp
1415b69f6: 4157  push r15
1415b69f8: 4156  push r14
1415b69fa: 4155  push r13
1415b69fc: 4154  push r12
1415b69fe: 56  push rsi
1415b69ff: 57  push rdi
1415b6a00: 53  push rbx
1415b6a01: 4883ec48  sub rsp, 0x48
1415b6a05: 488daa80000000  lea rbp, [rdx + 0x80]
1415b6a0c: 0f29742430  movaps xmmword ptr [rsp + 0x30], xmm6
1415b6a11: 488d4de8  lea rcx, [rbp - 0x18]
1415b6a15: e8e63a2dff  call 0x14088a500
1415b6a1a: 0f28742430  movaps xmm6, xmmword ptr [rsp + 0x30]
1415b6a1f: 4883c448  add rsp, 0x48
1415b6a23: 5b  pop rbx
1415b6a24: 5f  pop rdi
1415b6a25: 5e  pop rsi
1415b6a26: 415c  pop r12
1415b6a28: 415d  pop r13
1415b6a2a: 415e  pop r14
1415b6a2c: 415f  pop r15
1415b6a2e: 5d  pop rbp
1415b6a2f: c3  ret 
