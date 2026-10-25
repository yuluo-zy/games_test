1415b7ff0: 55  push rbp
1415b7ff1: 4157  push r15
1415b7ff3: 4156  push r14
1415b7ff5: 4155  push r13
1415b7ff7: 4154  push r12
1415b7ff9: 56  push rsi
1415b7ffa: 57  push rdi
1415b7ffb: 53  push rbx
1415b7ffc: 4881ec58010000  sub rsp, 0x158
1415b8003: 488dac2480000000  lea rbp, [rsp + 0x80]
1415b800b: 0f29b5c0000000  movaps xmmword ptr [rbp + 0xc0], xmm6
1415b8012: 48c785b8000000feffffff  mov qword ptr [rbp + 0xb8], 0xfffffffffffffffe
1415b801d: 4889ce  mov rsi, rcx
1415b8020: 488b4210  mov rax, qword ptr [rdx + 0x10]
1415b8024: 488945b0  mov qword ptr [rbp - 0x50], rax
1415b8028: 4c8d75c0  lea r14, [rbp - 0x40]
1415b802c: 4c89f1  mov rcx, r14
1415b802f: e89c005dff  call 0x140b880d0
1415b8034: 488d7d41  lea rdi, [rbp + 0x41]
1415b8038: 48c7457800000000  mov qword ptr [rbp + 0x78], 0
1415b8040: b001  mov al, 1
1415b8042: 89858c000000  mov dword ptr [rbp + 0x8c], eax
1415b8048: c685b600000002  mov byte ptr [rbp + 0xb6], 2
1415b804f: 4c8da598000000  lea r12, [rbp + 0x98]
1415b8056: 4c8d6d40  lea r13, [rbp + 0x40]
1415b805a: 4c8d3ddf326c01  lea r15, [rip + 0x16c32df]
1415b8061: c685b700000002  mov byte ptr [rbp + 0xb7], 2
1415b8068: 48c7457000000000  mov qword ptr [rbp + 0x70], 0
1415b8070: 48c7452800000000  mov qword ptr [rbp + 0x28], 0
1415b8078: c785b000000000000000  mov dword ptr [rbp + 0xb0], 0
1415b8082: 66666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415b8090: 4c89e1  mov rcx, r12
1415b8093: 4c89f2  mov rdx, r14
1415b8096: e8b543e8ff  call 0x14143c450
1415b809b: 488b8598000000  mov rax, qword ptr [rbp + 0x98]
1415b80a2: 4885c0  test rax, rax
1415b80a5: 0f84c4030000  je 0x1415b846f
1415b80ab: 488b8da8000000  mov rcx, qword ptr [rbp + 0xa8]
1415b80b2: 488d1449  lea rdx, [rcx + rcx*2]
1415b80b6: 4c8b84d068010000  mov r8, qword ptr [rax + rdx*8 + 0x168]
1415b80be: 4c8b8cd070010000  mov r9, qword ptr [rax + rdx*8 + 0x170]
1415b80c6: 4c898d80000000  mov qword ptr [rbp + 0x80], r9
1415b80cd: 488b9cd078010000  mov rbx, qword ptr [rax + rdx*8 + 0x178]
1415b80d5: 48c1e105  shl rcx, 5
1415b80d9: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
1415b80dd: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
1415b80e2: 0f294540  movaps xmmword ptr [rbp + 0x40], xmm0
1415b80e6: 0f294d50  movaps xmmword ptr [rbp + 0x50], xmm1
1415b80ea: 4c898590000000  mov qword ptr [rbp + 0x90], r8
1415b80f1: 4c89c0  mov rax, r8
1415b80f4: 48f7d8  neg rax
1415b80f7: 0f8072030000  jo 0x1415b846f
1415b80fd: 0fb64508  movzx eax, byte ptr [rbp + 8]
1415b8101: 4883f806  cmp rax, 6
1415b8105: 0f8768010000  ja 0x1415b8273
1415b810b: 49630487  movsxd rax, dword ptr [r15 + rax*4]
1415b810f: 4c01f8  add rax, r15
1415b8112: ffe0  jmp rax
1415b8114: 0f284540  movaps xmm0, xmmword ptr [rbp + 0x40]
1415b8118: 0f284d50  movaps xmm1, xmmword ptr [rbp + 0x50]
1415b811c: 488d4508  lea rax, [rbp + 8]
1415b8120: 0f114810  movups xmmword ptr [rax + 0x10], xmm1
1415b8124: 0f1100  movups xmmword ptr [rax], xmm0
1415b8127: 4c89e1  mov rcx, r12
1415b812a: 488b9580000000  mov rdx, qword ptr [rbp + 0x80]
1415b8131: 4989d8  mov r8, rbx
1415b8134: e897271400  call 0x1416fa8d0
1415b8139: 488b9590000000  mov rdx, qword ptr [rbp + 0x90]
1415b8140: 4885d2  test rdx, rdx
1415b8143: 7412  je 0x1415b8157
1415b8145: 41b801000000  mov r8d, 1
1415b814b: 488b8d80000000  mov rcx, qword ptr [rbp + 0x80]
1415b8152: e8c9ba05ff  call 0x140613c20
1415b8157: 80bd9800000000  cmp byte ptr [rbp + 0x98], 0
1415b815e: 0f859d030000  jne 0x1415b8501
1415b8164: 0fb68599000000  movzx eax, byte ptr [rbp + 0x99]
1415b816b: 83f806  cmp eax, 6
1415b816e: 753a  jne 0x1415b81aa
1415b8170: 0fb64508  movzx eax, byte ptr [rbp + 8]
1415b8174: c6450806  mov byte ptr [rbp + 8], 6
1415b8178: 3c06  cmp al, 6
1415b817a: 0f849d030000  je 0x1415b851d
1415b8180: 884540  mov byte ptr [rbp + 0x40], al
1415b8183: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415b8188: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415b818d: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415b8191: 0f1107  movups xmmword ptr [rdi], xmm0
1415b8194: 4c89e9  mov rcx, r13
1415b8197: e8a4b7feff  call 0x1415a3940
1415b819c: 4885c0  test rax, rax
1415b819f: 0f84ebfeffff  je 0x1415b8090
1415b81a5: e984030000  jmp 0x1415b852e
1415b81aa: 89c0  mov eax, eax
1415b81ac: 488d0da9316c01  lea rcx, [rip + 0x16c31a9]
1415b81b3: 48630481  movsxd rax, dword ptr [rcx + rax*4]
1415b81b7: 4801c8  add rax, rcx
1415b81ba: ffe0  jmp rax
1415b81bc: 83bdb000000000  cmp dword ptr [rbp + 0xb0], 0
1415b81c3: 0f8557040000  jne 0x1415b8620
1415b81c9: 0fb64508  movzx eax, byte ptr [rbp + 8]
1415b81cd: c6450806  mov byte ptr [rbp + 8], 6
1415b81d1: 3c06  cmp al, 6
1415b81d3: 0f84b2030000  je 0x1415b858b
1415b81d9: 884540  mov byte ptr [rbp + 0x40], al
1415b81dc: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415b81e1: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415b81e6: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415b81ea: 0f1107  movups xmmword ptr [rdi], xmm0
1415b81ed: 4c89e1  mov rcx, r12
1415b81f0: 4c89ea  mov rdx, r13
1415b81f3: e87821feff  call 0x14159a370
1415b81f8: f6859800000001  test byte ptr [rbp + 0x98], 1
1415b81ff: 0f85fc020000  jne 0x1415b8501
1415b8205: f30f10b59c000000  movss xmm6, dword ptr [rbp + 0x9c]
1415b820d: c785b000000001000000  mov dword ptr [rbp + 0xb0], 1
1415b8217: e974feffff  jmp 0x1415b8090
1415b821c: 488b5510  mov rdx, qword ptr [rbp + 0x10]
1415b8220: 4885d2  test rdx, rdx
1415b8223: 0f84ebfeffff  je 0x1415b8114
1415b8229: 488b4d18  mov rcx, qword ptr [rbp + 0x18]
1415b822d: 41b801000000  mov r8d, 1
1415b8233: e8e8b905ff  call 0x140613c20
1415b8238: e9d7feffff  jmp 0x1415b8114
1415b823d: 488b4d18  mov rcx, qword ptr [rbp + 0x18]
1415b8241: 488b5520  mov rdx, qword ptr [rbp + 0x20]
1415b8245: 48894d30  mov qword ptr [rbp + 0x30], rcx
1415b8249: e8220b2dff  call 0x140888d70
1415b824e: 488b5510  mov rdx, qword ptr [rbp + 0x10]
1415b8252: 4885d2  test rdx, rdx
1415b8255: 0f84b9feffff  je 0x1415b8114
1415b825b: 48c1e205  shl rdx, 5
1415b825f: 41b808000000  mov r8d, 8
1415b8265: 488b4d30  mov rcx, qword ptr [rbp + 0x30]
1415b8269: e8b2b905ff  call 0x140613c20
1415b826e: e9a1feffff  jmp 0x1415b8114
1415b8273: 488d4d10  lea rcx, [rbp + 0x10]
1415b8277: e8c40c2eff  call 0x140898f40
1415b827c: e993feffff  jmp 0x1415b8114
1415b8281: 80bdb700000002  cmp byte ptr [rbp + 0xb7], 2
1415b8288: 0f857c030000  jne 0x1415b860a
1415b828e: 0fb64508  movzx eax, byte ptr [rbp + 8]
1415b8292: c6450806  mov byte ptr [rbp + 8], 6
1415b8296: 3c06  cmp al, 6
1415b8298: 0f84da020000  je 0x1415b8578
1415b829e: 884540  mov byte ptr [rbp + 0x40], al
1415b82a1: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415b82a6: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415b82ab: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415b82af: 0f1107  movups xmmword ptr [rdi], xmm0
1415b82b2: 4c89e1  mov rcx, r12
1415b82b5: 4c89ea  mov rdx, r13
1415b82b8: e83352feff  call 0x14159d4f0
1415b82bd: f6859800000001  test byte ptr [rbp + 0x98], 1
1415b82c4: 0f8537020000  jne 0x1415b8501
1415b82ca: 0fb68599000000  movzx eax, byte ptr [rbp + 0x99]
1415b82d1: 8885b7000000  mov byte ptr [rbp + 0xb7], al
1415b82d7: e9b4fdffff  jmp 0x1415b8090
1415b82dc: f6457001  test byte ptr [rbp + 0x70], 1
1415b82e0: 0f8550030000  jne 0x1415b8636
1415b82e6: 0fb64508  movzx eax, byte ptr [rbp + 8]
1415b82ea: c6450806  mov byte ptr [rbp + 8], 6
1415b82ee: 3c06  cmp al, 6
1415b82f0: 0f84d4020000  je 0x1415b85ca
1415b82f6: 884540  mov byte ptr [rbp + 0x40], al
1415b82f9: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415b82fe: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415b8303: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415b8307: 0f1107  movups xmmword ptr [rdi], xmm0
1415b830a: 4c89e1  mov rcx, r12
1415b830d: 4c89ea  mov rdx, r13
1415b8310: e86b1efeff  call 0x14159a180
1415b8315: 80bd9800000000  cmp byte ptr [rbp + 0x98], 0
1415b831c: 0f85df010000  jne 0x1415b8501
1415b8322: 0fb68599000000  movzx eax, byte ptr [rbp + 0x99]
1415b8329: 8885b4000000  mov byte ptr [rbp + 0xb4], al
1415b832f: b001  mov al, 1
1415b8331: 48894570  mov qword ptr [rbp + 0x70], rax
1415b8335: e956fdffff  jmp 0x1415b8090
1415b833a: 48837d7800  cmp qword ptr [rbp + 0x78], 0
1415b833f: 0f851d030000  jne 0x1415b8662
1415b8345: 0fb64508  movzx eax, byte ptr [rbp + 8]
1415b8349: c6450806  mov byte ptr [rbp + 8], 6
1415b834d: 3c06  cmp al, 6
1415b834f: 0f8410020000  je 0x1415b8565
1415b8355: 884540  mov byte ptr [rbp + 0x40], al
1415b8358: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415b835d: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415b8362: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415b8366: 0f1107  movups xmmword ptr [rdi], xmm0
1415b8369: 4c89e9  mov rcx, r13
1415b836c: e81f4efeff  call 0x14159d190
1415b8371: 48895538  mov qword ptr [rbp + 0x38], rdx
1415b8375: 4885c0  test rax, rax
1415b8378: 0f8562020000  jne 0x1415b85e0
1415b837e: b801000000  mov eax, 1
1415b8383: 48894578  mov qword ptr [rbp + 0x78], rax
1415b8387: e904fdffff  jmp 0x1415b8090
1415b838c: f6858c00000001  test byte ptr [rbp + 0x8c], 1
1415b8393: 0f84df020000  je 0x1415b8678
1415b8399: 0fb64508  movzx eax, byte ptr [rbp + 8]
1415b839d: c6450806  mov byte ptr [rbp + 8], 6
1415b83a1: 3c06  cmp al, 6
1415b83a3: 0f84f5010000  je 0x1415b859e
1415b83a9: 884540  mov byte ptr [rbp + 0x40], al
1415b83ac: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415b83b1: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415b83b6: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415b83ba: 0f1107  movups xmmword ptr [rdi], xmm0
1415b83bd: 48c744242003000000  mov qword ptr [rsp + 0x20], 3
1415b83c6: 41b904000000  mov r9d, 4
1415b83cc: 4c89e1  mov rcx, r12
1415b83cf: 4c89ea  mov rdx, r13
1415b83d2: 4c8d05efd14d01  lea r8, [rip + 0x14dd1ef]
1415b83d9: e812b8feff  call 0x1415a3bf0
1415b83de: f6859800000001  test byte ptr [rbp + 0x98], 1
1415b83e5: 0f8516010000  jne 0x1415b8501
1415b83eb: 8b859c000000  mov eax, dword ptr [rbp + 0x9c]
1415b83f1: 89456c  mov dword ptr [rbp + 0x6c], eax
1415b83f4: 488b85a0000000  mov rax, qword ptr [rbp + 0xa0]
1415b83fb: 488945b8  mov qword ptr [rbp - 0x48], rax
1415b83ff: b001  mov al, 1
1415b8401: 48894528  mov qword ptr [rbp + 0x28], rax
1415b8405: c7858c00000000000000  mov dword ptr [rbp + 0x8c], 0
1415b840f: e97cfcffff  jmp 0x1415b8090
1415b8414: 80bdb600000002  cmp byte ptr [rbp + 0xb6], 2
1415b841b: 0f852b020000  jne 0x1415b864c
1415b8421: 0fb64508  movzx eax, byte ptr [rbp + 8]
1415b8425: c6450806  mov byte ptr [rbp + 8], 6
1415b8429: 3c06  cmp al, 6
1415b842b: 0f8483010000  je 0x1415b85b4
1415b8431: 884540  mov byte ptr [rbp + 0x40], al
1415b8434: 410f104649  movups xmm0, xmmword ptr [r14 + 0x49]
1415b8439: 410f104e58  movups xmm1, xmmword ptr [r14 + 0x58]
1415b843e: 0f114f0f  movups xmmword ptr [rdi + 0xf], xmm1
1415b8442: 0f1107  movups xmmword ptr [rdi], xmm0
1415b8445: 4c89e1  mov rcx, r12
1415b8448: 4c89ea  mov rdx, r13
1415b844b: e890251400  call 0x1416fa9e0
1415b8450: f6859800000001  test byte ptr [rbp + 0x98], 1
1415b8457: 0f85a4000000  jne 0x1415b8501
1415b845d: 0fb68599000000  movzx eax, byte ptr [rbp + 0x99]
1415b8464: 8885b6000000  mov byte ptr [rbp + 0xb6], al
1415b846a: e921fcffff  jmp 0x1415b8090
1415b846f: f685b000000001  test byte ptr [rbp + 0xb0], 1
1415b8476: 7503  jne 0x1415b847b
1415b8478: 0f57f6  xorps xmm6, xmm6
1415b847b: 31c0  xor eax, eax
1415b847d: f6452801  test byte ptr [rbp + 0x28], 1
1415b8481: 448b4d6c  mov r9d, dword ptr [rbp + 0x6c]
1415b8485: 440f44c8  cmove r9d, eax
1415b8489: 480f4545b8  cmovne rax, qword ptr [rbp - 0x48]
1415b848e: f6457001  test byte ptr [rbp + 0x70], 1
1415b8492: 7476  je 0x1415b850a
1415b8494: 48837d0000  cmp qword ptr [rbp], 0
1415b8499: 0f854a010000  jne 0x1415b85e9
1415b849f: 31d2  xor edx, edx
1415b84a1: f6457801  test byte ptr [rbp + 0x78], 1
1415b84a5: 480f455538  cmovne rdx, qword ptr [rbp + 0x38]
1415b84aa: 440fb685b6000000  movzx r8d, byte ptr [rbp + 0xb6]
1415b84b2: 4489c1  mov ecx, r8d
1415b84b5: 80e101  and cl, 1
1415b84b8: 4180f802  cmp r8b, 2
1415b84bc: 440fb6c1  movzx r8d, cl
1415b84c0: b901000000  mov ecx, 1
1415b84c5: 410f45c8  cmovne ecx, r8d
1415b84c9: 440fb685b7000000  movzx r8d, byte ptr [rbp + 0xb7]
1415b84d1: 4180e001  and r8b, 1
1415b84d5: 488916  mov qword ptr [rsi], rdx
1415b84d8: f30f117608  movss dword ptr [rsi + 8], xmm6
1415b84dd: 44894e0c  mov dword ptr [rsi + 0xc], r9d
1415b84e1: 48894610  mov qword ptr [rsi + 0x10], rax
1415b84e5: 0fb685b4000000  movzx eax, byte ptr [rbp + 0xb4]
1415b84ec: 884618  mov byte ptr [rsi + 0x18], al
1415b84ef: 44884619  mov byte ptr [rsi + 0x19], r8b
1415b84f3: 884e1a  mov byte ptr [rsi + 0x1a], cl
1415b84f6: 488d4dc0  lea rcx, [rbp - 0x40]
1415b84fa: e8f1f82cff  call 0x140887df0
1415b84ff: eb3d  jmp 0x1415b853e
1415b8501: 488b85a0000000  mov rax, qword ptr [rbp + 0xa0]
1415b8508: eb24  jmp 0x1415b852e
1415b850a: 488d0dc62c6c01  lea rcx, [rip + 0x16c2cc6]
1415b8511: ba06000000  mov edx, 6
1415b8516: e8a5163401  call 0x1428f9bc0
1415b851b: eb11  jmp 0x1415b852e
1415b851d: 488d0d441d6c01  lea rcx, [rip + 0x16c1d44]
1415b8524: ba10000000  mov edx, 0x10
1415b8529: e8d21a3401  call 0x1428fa000
1415b852e: 488906  mov qword ptr [rsi], rax
1415b8531: c6461a02  mov byte ptr [rsi + 0x1a], 2
1415b8535: 488d4dc0  lea rcx, [rbp - 0x40]
1415b8539: e8b2f82cff  call 0x140887df0
1415b853e: 488d4d08  lea rcx, [rbp + 8]
1415b8542: e8e95e2dff  call 0x14088e430
1415b8547: 4889f0  mov rax, rsi
1415b854a: 0f28b5c0000000  movaps xmm6, xmmword ptr [rbp + 0xc0]
1415b8551: 4881c458010000  add rsp, 0x158
1415b8558: 5b  pop rbx
1415b8559: 5f  pop rdi
1415b855a: 5e  pop rsi
1415b855b: 415c  pop r12
1415b855d: 415d  pop r13
1415b855f: 415e  pop r14
1415b8561: 415f  pop r15
1415b8563: 5d  pop rbp
1415b8564: c3  ret 
1415b8565: 488d0dfc1c6c01  lea rcx, [rip + 0x16c1cfc]
1415b856c: ba10000000  mov edx, 0x10
1415b8571: e88a1a3401  call 0x1428fa000
1415b8576: ebb6  jmp 0x1415b852e
1415b8578: 488d0de91c6c01  lea rcx, [rip + 0x16c1ce9]
1415b857f: ba10000000  mov edx, 0x10
1415b8584: e8771a3401  call 0x1428fa000
1415b8589: eba3  jmp 0x1415b852e
1415b858b: 488d0dd61c6c01  lea rcx, [rip + 0x16c1cd6]
1415b8592: ba10000000  mov edx, 0x10
1415b8597: e8641a3401  call 0x1428fa000
1415b859c: eb90  jmp 0x1415b852e
1415b859e: 488d0dc31c6c01  lea rcx, [rip + 0x16c1cc3]
1415b85a5: ba10000000  mov edx, 0x10
1415b85aa: e8511a3401  call 0x1428fa000
1415b85af: e97affffff  jmp 0x1415b852e
1415b85b4: 488d0dad1c6c01  lea rcx, [rip + 0x16c1cad]
1415b85bb: ba10000000  mov edx, 0x10
1415b85c0: e83b1a3401  call 0x1428fa000
1415b85c5: e964ffffff  jmp 0x1415b852e
1415b85ca: 488d0d971c6c01  lea rcx, [rip + 0x16c1c97]
1415b85d1: ba10000000  mov edx, 0x10
1415b85d6: e8251a3401  call 0x1428fa000
1415b85db: e94effffff  jmp 0x1415b852e
1415b85e0: 488b4538  mov rax, qword ptr [rbp + 0x38]
1415b85e4: e945ffffff  jmp 0x1415b852e
1415b85e9: 488d15a81a6c01  lea rdx, [rip + 0x16c1aa8]
1415b85f0: 4c8d05691a6c01  lea r8, [rip + 0x16c1a69]
1415b85f7: 488b4db0  mov rcx, qword ptr [rbp - 0x50]
1415b85fb: e820173401  call 0x1428f9d20
1415b8600: 488906  mov qword ptr [rsi], rax
1415b8603: b102  mov cl, 2
1415b8605: e9e9feffff  jmp 0x1415b84f3
1415b860a: 488d0dd42b6c01  lea rcx, [rip + 0x16c2bd4]
1415b8611: ba0e000000  mov edx, 0xe
1415b8616: e895173401  call 0x1428f9db0
1415b861b: e90effffff  jmp 0x1415b852e
1415b8620: 488d0d912b6c01  lea rcx, [rip + 0x16c2b91]
1415b8627: ba16000000  mov edx, 0x16
1415b862c: e87f173401  call 0x1428f9db0
1415b8631: e9f8feffff  jmp 0x1415b852e
1415b8636: 488d0d9a2b6c01  lea rcx, [rip + 0x16c2b9a]
1415b863d: ba06000000  mov edx, 6
1415b8642: e869173401  call 0x1428f9db0
1415b8647: e9e2feffff  jmp 0x1415b852e
1415b864c: 488d0da02b6c01  lea rcx, [rip + 0x16c2ba0]
1415b8653: ba04000000  mov edx, 4
1415b8658: e853173401  call 0x1428f9db0
1415b865d: e9ccfeffff  jmp 0x1415b852e
1415b8662: 488d0d742b6c01  lea rcx, [rip + 0x16c2b74]
1415b8669: ba08000000  mov edx, 8
1415b866e: e83d173401  call 0x1428f9db0
1415b8673: e9b6feffff  jmp 0x1415b852e
1415b8678: 488d0d4f2b6c01  lea rcx, [rip + 0x16c2b4f]
1415b867f: ba09000000  mov edx, 9
1415b8684: e827173401  call 0x1428f9db0
1415b8689: e9a0feffff  jmp 0x1415b852e
1415b868e: 6690  nop 
1415b8690: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b8695: 55  push rbp
1415b8696: 4157  push r15
1415b8698: 4156  push r14
1415b869a: 4155  push r13
1415b869c: 4154  push r12
1415b869e: 56  push rsi
1415b869f: 57  push rdi
1415b86a0: 53  push rbx
1415b86a1: 4883ec38  sub rsp, 0x38
1415b86a5: 488daa80000000  lea rbp, [rdx + 0x80]
1415b86ac: 0f29742420  movaps xmmword ptr [rsp + 0x20], xmm6
1415b86b1: 488b5510  mov rdx, qword ptr [rbp + 0x10]
1415b86b5: 4885d2  test rdx, rdx
1415b86b8: 7413  je 0x1415b86cd
1415b86ba: 48c1e205  shl rdx, 5
1415b86be: 41b808000000  mov r8d, 8
1415b86c4: 488b4d30  mov rcx, qword ptr [rbp + 0x30]
1415b86c8: e853b505ff  call 0x140613c20
1415b86cd: 0f28742420  movaps xmm6, xmmword ptr [rsp + 0x20]
1415b86d2: 4883c438  add rsp, 0x38
1415b86d6: 5b  pop rbx
1415b86d7: 5f  pop rdi
1415b86d8: 5e  pop rsi
1415b86d9: 415c  pop r12
1415b86db: 415d  pop r13
1415b86dd: 415e  pop r14
1415b86df: 415f  pop r15
1415b86e1: 5d  pop rbp
1415b86e2: c3  ret 
1415b86e3: 666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415b86f0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b86f5: 55  push rbp
1415b86f6: 4157  push r15
1415b86f8: 4156  push r14
1415b86fa: 4155  push r13
1415b86fc: 4154  push r12
1415b86fe: 56  push rsi
1415b86ff: 57  push rdi
1415b8700: 53  push rbx
1415b8701: 4883ec38  sub rsp, 0x38
1415b8705: 488daa80000000  lea rbp, [rdx + 0x80]
1415b870c: 0f29742420  movaps xmmword ptr [rsp + 0x20], xmm6
1415b8711: 0f284540  movaps xmm0, xmmword ptr [rbp + 0x40]
1415b8715: 0f284d50  movaps xmm1, xmmword ptr [rbp + 0x50]
1415b8719: 488d4508  lea rax, [rbp + 8]
1415b871d: 0f114810  movups xmmword ptr [rax + 0x10], xmm1
1415b8721: 0f1100  movups xmmword ptr [rax], xmm0
1415b8724: c685b500000001  mov byte ptr [rbp + 0xb5], 1
1415b872b: 0f28742420  movaps xmm6, xmmword ptr [rsp + 0x20]
1415b8730: 4883c438  add rsp, 0x38
1415b8734: 5b  pop rbx
1415b8735: 5f  pop rdi
1415b8736: 5e  pop rsi
1415b8737: 415c  pop r12
1415b8739: 415d  pop r13
1415b873b: 415e  pop r14
1415b873d: 415f  pop r15
1415b873f: 5d  pop rbp
1415b8740: c3  ret 
1415b8741: 6666666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415b8750: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b8755: 55  push rbp
1415b8756: 4157  push r15
1415b8758: 4156  push r14
1415b875a: 4155  push r13
1415b875c: 4154  push r12
1415b875e: 56  push rsi
1415b875f: 57  push rdi
1415b8760: 53  push rbx
1415b8761: 4883ec38  sub rsp, 0x38
1415b8765: 488daa80000000  lea rbp, [rdx + 0x80]
1415b876c: 0f29742420  movaps xmmword ptr [rsp + 0x20], xmm6
1415b8771: 4883bd9000000000  cmp qword ptr [rbp + 0x90], 0
1415b8779: 7419  je 0x1415b8794
1415b877b: 41b801000000  mov r8d, 1
1415b8781: 488b8d80000000  mov rcx, qword ptr [rbp + 0x80]
1415b8788: 488b9590000000  mov rdx, qword ptr [rbp + 0x90]
1415b878f: e88cb405ff  call 0x140613c20
1415b8794: c685b500000000  mov byte ptr [rbp + 0xb5], 0
1415b879b: 0f28742420  movaps xmm6, xmmword ptr [rsp + 0x20]
1415b87a0: 4883c438  add rsp, 0x38
1415b87a4: 5b  pop rbx
1415b87a5: 5f  pop rdi
1415b87a6: 5e  pop rsi
1415b87a7: 415c  pop r12
1415b87a9: 415d  pop r13
1415b87ab: 415e  pop r14
1415b87ad: 415f  pop r15
1415b87af: 5d  pop rbp
1415b87b0: c3  ret 
1415b87b1: 6666666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415b87c0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b87c5: 55  push rbp
1415b87c6: 4157  push r15
1415b87c8: 4156  push r14
1415b87ca: 4155  push r13
1415b87cc: 4154  push r12
1415b87ce: 56  push rsi
1415b87cf: 57  push rdi
1415b87d0: 53  push rbx
1415b87d1: 4883ec38  sub rsp, 0x38
1415b87d5: 488daa80000000  lea rbp, [rdx + 0x80]
1415b87dc: 0f29742420  movaps xmmword ptr [rsp + 0x20], xmm6
1415b87e1: 80bdb500000000  cmp byte ptr [rbp + 0xb5], 0
1415b87e8: 7439  je 0x1415b8823
1415b87ea: 4883bd9000000000  cmp qword ptr [rbp + 0x90], 0
1415b87f2: 7419  je 0x1415b880d
1415b87f4: 41b801000000  mov r8d, 1
1415b87fa: 488b8d80000000  mov rcx, qword ptr [rbp + 0x80]
1415b8801: 488b9590000000  mov rdx, qword ptr [rbp + 0x90]
1415b8808: e813b405ff  call 0x140613c20
1415b880d: 0f28742420  movaps xmm6, xmmword ptr [rsp + 0x20]
1415b8812: 4883c438  add rsp, 0x38
1415b8816: 5b  pop rbx
1415b8817: 5f  pop rdi
1415b8818: 5e  pop rsi
1415b8819: 415c  pop r12
1415b881b: 415d  pop r13
1415b881d: 415e  pop r14
1415b881f: 415f  pop r15
1415b8821: 5d  pop rbp
1415b8822: c3  ret 
1415b8823: 0f28742420  movaps xmm6, xmmword ptr [rsp + 0x20]
1415b8828: 4883c438  add rsp, 0x38
1415b882c: 5b  pop rbx
1415b882d: 5f  pop rdi
1415b882e: 5e  pop rsi
1415b882f: 415c  pop r12
1415b8831: 415d  pop r13
1415b8833: 415e  pop r14
1415b8835: 415f  pop r15
1415b8837: 5d  pop rbp
1415b8838: c3  ret 
1415b8839: 0f1f8000000000  nop dword ptr [rax]
1415b8840: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b8845: 55  push rbp
1415b8846: 4157  push r15
1415b8848: 4156  push r14
1415b884a: 4155  push r13
1415b884c: 4154  push r12
1415b884e: 56  push rsi
1415b884f: 57  push rdi
1415b8850: 53  push rbx
1415b8851: 4883ec38  sub rsp, 0x38
1415b8855: 488daa80000000  lea rbp, [rdx + 0x80]
1415b885c: 0f29742420  movaps xmmword ptr [rsp + 0x20], xmm6
1415b8861: 488d4dc0  lea rcx, [rbp - 0x40]
1415b8865: e8961c2dff  call 0x14088a500
1415b886a: 0f28742420  movaps xmm6, xmmword ptr [rsp + 0x20]
1415b886f: 4883c438  add rsp, 0x38
1415b8873: 5b  pop rbx
1415b8874: 5f  pop rdi
1415b8875: 5e  pop rsi
1415b8876: 415c  pop r12
1415b8878: 415d  pop r13
1415b887a: 415e  pop r14
1415b887c: 415f  pop r15
1415b887e: 5d  pop rbp
1415b887f: c3  ret 
1415b8880: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b8885: 55  push rbp
1415b8886: 4157  push r15
1415b8888: 4156  push r14
1415b888a: 4155  push r13
1415b888c: 4154  push r12
1415b888e: 56  push rsi
1415b888f: 57  push rdi
1415b8890: 53  push rbx
1415b8891: 4883ec38  sub rsp, 0x38
1415b8895: 488daa80000000  lea rbp, [rdx + 0x80]
1415b889c: 0f29742420  movaps xmmword ptr [rsp + 0x20], xmm6
1415b88a1: 488d4d08  lea rcx, [rbp + 8]
1415b88a5: e8865b2dff  call 0x14088e430
1415b88aa: 0f28742420  movaps xmm6, xmmword ptr [rsp + 0x20]
1415b88af: 4883c438  add rsp, 0x38
1415b88b3: 5b  pop rbx
1415b88b4: 5f  pop rdi
1415b88b5: 5e  pop rsi
1415b88b6: 415c  pop r12
1415b88b8: 415d  pop r13
1415b88ba: 415e  pop r14
1415b88bc: 415f  pop r15
1415b88be: 5d  pop rbp
1415b88bf: c3  ret 
1415b88c0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b88c5: 55  push rbp
1415b88c6: 4157  push r15
1415b88c8: 4156  push r14
1415b88ca: 4155  push r13
1415b88cc: 4154  push r12
1415b88ce: 56  push rsi
1415b88cf: 57  push rdi
1415b88d0: 53  push rbx
1415b88d1: 4883ec38  sub rsp, 0x38
1415b88d5: 488daa80000000  lea rbp, [rdx + 0x80]
1415b88dc: 0f29742420  movaps xmmword ptr [rsp + 0x20], xmm6
1415b88e1: 488d4d08  lea rcx, [rbp + 8]
1415b88e5: e8465b2dff  call 0x14088e430
1415b88ea: 0f28742420  movaps xmm6, xmmword ptr [rsp + 0x20]
1415b88ef: 4883c438  add rsp, 0x38
1415b88f3: 5b  pop rbx
1415b88f4: 5f  pop rdi
1415b88f5: 5e  pop rsi
1415b88f6: 415c  pop r12
1415b88f8: 415d  pop r13
1415b88fa: 415e  pop r14
1415b88fc: 415f  pop r15
1415b88fe: 5d  pop rbp
1415b88ff: c3  ret 
