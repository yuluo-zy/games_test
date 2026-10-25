1417a0430: 55  push rbp
1417a0431: 4157  push r15
1417a0433: 4156  push r14
1417a0435: 4155  push r13
1417a0437: 4154  push r12
1417a0439: 56  push rsi
1417a043a: 57  push rdi
1417a043b: 53  push rbx
1417a043c: 4881ec28040000  sub rsp, 0x428
1417a0443: 488dac2480000000  lea rbp, [rsp + 0x80]
1417a044b: 0f29bd90030000  movaps xmmword ptr [rbp + 0x390], xmm7
1417a0452: 0f29b580030000  movaps xmmword ptr [rbp + 0x380], xmm6
1417a0459: 48c78578030000feffffff  mov qword ptr [rbp + 0x378], 0xfffffffffffffffe
1417a0464: 48898d40030000  mov qword ptr [rbp + 0x340], rcx
1417a046b: c6857703000001  mov byte ptr [rbp + 0x377], 1
1417a0472: e8896547ff  call 0x140c16a00
1417a0477: 4189c7  mov r15d, eax
1417a047a: 84c0  test al, al
1417a047c: 746f  je 0x1417a04ed
1417a047e: c6857703000001  mov byte ptr [rbp + 0x377], 1
1417a0485: be46000000  mov esi, 0x46
1417a048a: b946000000  mov ecx, 0x46
1417a048f: e8acf8ffff  call 0x14179fd40
1417a0494: 488d3d15bf5301  lea rdi, [rip + 0x153bf15]
1417a049b: 4883f801  cmp rax, 1
1417a049f: 0f85c6000000  jne 0x1417a056b
1417a04a5: 4989d1  mov r9, rdx
1417a04a8: 4885d2  test rdx, rdx
1417a04ab: 7449  je 0x1417a04f6
1417a04ad: 4983f946  cmp r9, 0x46
1417a04b1: 7341  jae 0x1417a04f4
1417a04b3: 488d05f6be5301  lea rax, [rip + 0x153bef6]
1417a04ba: 41803c01bf  cmp byte ptr [r9 + rax], 0xbf
1417a04bf: 7f35  jg 0x1417a04f6
1417a04c1: c6857703000001  mov byte ptr [rbp + 0x377], 1
1417a04c8: 488d05b1bf5301  lea rax, [rip + 0x153bfb1]
1417a04cf: 4889442420  mov qword ptr [rsp + 0x20], rax
1417a04d4: 488d0dd5be5301  lea rcx, [rip + 0x153bed5]
1417a04db: ba46000000  mov edx, 0x46
1417a04e0: 4531c0  xor r8d, r8d
1417a04e3: e858941301  call 0x1428d9940
1417a04e8: e9980e0000  jmp 0x1417a1385
1417a04ed: 31c0  xor eax, eax
1417a04ef: e9ff010000  jmp 0x1417a06f3
1417a04f4: 75cb  jne 0x1417a04c1
1417a04f6: c6857703000001  mov byte ptr [rbp + 0x377], 1
1417a04fd: 4c89c9  mov rcx, r9
1417a0500: e83bf8ffff  call 0x14179fd40
1417a0505: 4883f801  cmp rax, 1
1417a0509: 7560  jne 0x1417a056b
1417a050b: 4989d0  mov r8, rdx
1417a050e: 4983c002  add r8, 2
1417a0512: 7445  je 0x1417a0559
1417a0514: 4983f846  cmp r8, 0x46
1417a0518: 733d  jae 0x1417a0557
1417a051a: 488d058fbe5301  lea rax, [rip + 0x153be8f]
1417a0521: 41803c00bf  cmp byte ptr [r8 + rax], 0xbf
1417a0526: 7f31  jg 0x1417a0559
1417a0528: c6857703000001  mov byte ptr [rbp + 0x377], 1
1417a052f: 488d0562bf5301  lea rax, [rip + 0x153bf62]
1417a0536: 4889442420  mov qword ptr [rsp + 0x20], rax
1417a053b: 488d0d6ebe5301  lea rcx, [rip + 0x153be6e]
1417a0542: ba46000000  mov edx, 0x46
1417a0547: 41b946000000  mov r9d, 0x46
1417a054d: e8ee931301  call 0x1428d9940
1417a0552: e92e0e0000  jmp 0x1417a1385
1417a0557: 75cf  jne 0x1417a0528
1417a0559: be44000000  mov esi, 0x44
1417a055e: 4829d6  sub rsi, rdx
1417a0561: 488d3d48be5301  lea rdi, [rip + 0x153be48]
1417a0568: 4c01c7  add rdi, r8
1417a056b: 4c8d0586bf5301  lea r8, [rip + 0x153bf86]
1417a0572: 488d1d4fbf5301  lea rbx, [rip + 0x153bf4f]
1417a0579: 0f1f8000000000  nop dword ptr [rax]
1417a0580: 410fbe40ff  movsx eax, byte ptr [r8 - 1]
1417a0585: 85c0  test eax, eax
1417a0587: 7817  js 0x1417a05a0
1417a0589: 49ffc8  dec r8
1417a058c: 83f85c  cmp eax, 0x5c
1417a058f: 7563  jne 0x1417a05f4
1417a0591: eb73  jmp 0x1417a0606
1417a0593: 666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a05a0: 410fb648fe  movzx ecx, byte ptr [r8 - 2]
1417a05a5: 80f9c0  cmp cl, 0xc0
1417a05a8: 7d23  jge 0x1417a05cd
1417a05aa: 410fb650fd  movzx edx, byte ptr [r8 - 3]
1417a05af: 80fac0  cmp dl, 0xc0
1417a05b2: 7d22  jge 0x1417a05d6
1417a05b4: 450fb648fc  movzx r9d, byte ptr [r8 - 4]
1417a05b9: 4983c0fc  add r8, -4
1417a05bd: 4183e107  and r9d, 7
1417a05c1: 41c1e106  shl r9d, 6
1417a05c5: 83e23f  and edx, 0x3f
1417a05c8: 4409ca  or edx, r9d
1417a05cb: eb10  jmp 0x1417a05dd
1417a05cd: 4983c0fe  add r8, -2
1417a05d1: 83e11f  and ecx, 0x1f
1417a05d4: eb0f  jmp 0x1417a05e5
1417a05d6: 4983c0fd  add r8, -3
1417a05da: 83e20f  and edx, 0xf
1417a05dd: c1e206  shl edx, 6
1417a05e0: 83e13f  and ecx, 0x3f
1417a05e3: 09d1  or ecx, edx
1417a05e5: c1e106  shl ecx, 6
1417a05e8: 243f  and al, 0x3f
1417a05ea: 0fb6c0  movzx eax, al
1417a05ed: 09c8  or eax, ecx
1417a05ef: 83f85c  cmp eax, 0x5c
1417a05f2: 7412  je 0x1417a0606
1417a05f4: 83f82f  cmp eax, 0x2f
1417a05f7: 740d  je 0x1417a0606
1417a05f9: 4939d8  cmp r8, rbx
1417a05fc: 7582  jne 0x1417a0580
1417a05fe: 41be30000000  mov r14d, 0x30
1417a0604: eb59  jmp 0x1417a065f
1417a0606: 488d1dbbbe5301  lea rbx, [rip + 0x153bebb]
1417a060d: 4929d8  sub r8, rbx
1417a0610: 49ffc0  inc r8
1417a0613: 743e  je 0x1417a0653
1417a0615: 4983f830  cmp r8, 0x30
1417a0619: 7336  jae 0x1417a0651
1417a061b: 41803c18bf  cmp byte ptr [r8 + rbx], 0xbf
1417a0620: 7f31  jg 0x1417a0653
1417a0622: c6857703000001  mov byte ptr [rbp + 0x377], 1
1417a0629: 488d0530be5301  lea rax, [rip + 0x153be30]
1417a0630: 4889442420  mov qword ptr [rsp + 0x20], rax
1417a0635: 488d0d8cbe5301  lea rcx, [rip + 0x153be8c]
1417a063c: ba30000000  mov edx, 0x30
1417a0641: 41b930000000  mov r9d, 0x30
1417a0647: e8f4921301  call 0x1428d9940
1417a064c: e9340d0000  jmp 0x1417a1385
1417a0651: 75cf  jne 0x1417a0622
1417a0653: 41be30000000  mov r14d, 0x30
1417a0659: 4d29c6  sub r14, r8
1417a065c: 4c01c3  add rbx, r8
1417a065f: e8cc7a47ff  call 0x140c18130
1417a0664: 4889c1  mov rcx, rax
1417a0667: 488b00  mov rax, qword ptr [rax]
1417a066a: 4885c0  test rax, rax
1417a066d: 740f  je 0x1417a067e
1417a066f: 83f801  cmp eax, 1
1417a0672: 0f85e50c0000  jne 0x1417a135d
1417a0678: 4883c108  add rcx, 8
1417a067c: eb1a  jmp 0x1417a0698
1417a067e: c6857703000001  mov byte ptr [rbp + 0x377], 1
1417a0685: 31d2  xor edx, edx
1417a0687: e844811501  call 0x1428f87d0
1417a068c: 4889c1  mov rcx, rax
1417a068f: 4885c0  test rax, rax
1417a0692: 0f84c50c0000  je 0x1417a135d
1417a0698: 48833900  cmp qword ptr [rcx], 0
1417a069c: 0f85410c0000  jne 0x1417a12e3
1417a06a2: 48c701ffffffff  mov qword ptr [rcx], 0xffffffffffffffff
1417a06a9: 48898d58030000  mov qword ptr [rbp + 0x358], rcx
1417a06b0: 4883c108  add rcx, 8
1417a06b4: 4c89742420  mov qword ptr [rsp + 0x20], r14
1417a06b9: 48c744243000000000  mov qword ptr [rsp + 0x30], 0
1417a06c2: 48c744242801000000  mov qword ptr [rsp + 0x28], 1
1417a06cb: 4889fa  mov rdx, rdi
1417a06ce: 4989f0  mov r8, rsi
1417a06d1: 4989d9  mov r9, rbx
1417a06d4: e8676647ff  call 0x140c16d40
1417a06d9: 488b8d58030000  mov rcx, qword ptr [rbp + 0x358]
1417a06e0: 48ff01  inc qword ptr [rcx]
1417a06e3: 488985e8020000  mov qword ptr [rbp + 0x2e8], rax
1417a06ea: 48894520  mov qword ptr [rbp + 0x20], rax
1417a06ee: b801000000  mov eax, 1
1417a06f3: 4488bd58030000  mov byte ptr [rbp + 0x358], r15b
1417a06fa: 48894518  mov qword ptr [rbp + 0x18], rax
1417a06fe: 803d9346e90100  cmp byte ptr [rip + 0x1e94693], 0
1417a0705: 4c8b8d40030000  mov r9, qword ptr [rbp + 0x340]
1417a070c: 7426  je 0x1417a0734
1417a070e: 48c7857802000000000000  mov qword ptr [rbp + 0x278], 0
1417a0719: 48c7858002000008000000  mov qword ptr [rbp + 0x280], 8
1417a0724: 48c7858802000000000000  mov qword ptr [rbp + 0x288], 0
1417a072f: e9cc000000  jmp 0x1417a0800
1417a0734: 498b8110010000  mov rax, qword ptr [r9 + 0x110]
1417a073b: 498b8918010000  mov rcx, qword ptr [r9 + 0x118]
1417a0742: 488b90f8000000  mov rdx, qword ptr [rax + 0xf8]
1417a0749: 4c8b8000010000  mov r8, qword ptr [rax + 0x100]
1417a0750: 4e8d0482  lea r8, [rdx + r8*4]
1417a0754: 440fb68808010000  movzx r9d, byte ptr [rax + 0x108]
1417a075c: 660fefc0  pxor xmm0, xmm0
1417a0760: f30f7f8548010000  movdqu xmmword ptr [rbp + 0x148], xmm0
1417a0768: f30f7f8538010000  movdqu xmmword ptr [rbp + 0x138], xmm0
1417a0770: f30f7f8528010000  movdqu xmmword ptr [rbp + 0x128], xmm0
1417a0778: 4c8d91f8000000  lea r10, [rcx + 0xf8]
1417a077f: 48898de0000000  mov qword ptr [rbp + 0xe0], rcx
1417a0786: 4881c1a0010000  add rcx, 0x1a0
1417a078d: 48898de8000000  mov qword ptr [rbp + 0xe8], rcx
1417a0794: 4c8995f0000000  mov qword ptr [rbp + 0xf0], r10
1417a079b: 488985f8000000  mov qword ptr [rbp + 0xf8], rax
1417a07a2: 48899500010000  mov qword ptr [rbp + 0x100], rdx
1417a07a9: 4c898508010000  mov qword ptr [rbp + 0x108], r8
1417a07b0: 48c7851001000008000000  mov qword ptr [rbp + 0x110], 8
1417a07bb: 48c7851801000000000000  mov qword ptr [rbp + 0x118], 0
1417a07c6: 48c7852001000008000000  mov qword ptr [rbp + 0x120], 8
1417a07d1: 44888d58010000  mov byte ptr [rbp + 0x158], r9b
1417a07d8: c6857303000001  mov byte ptr [rbp + 0x373], 1
1417a07df: 4c8d05c2b95301  lea r8, [rip + 0x153b9c2]
1417a07e6: 488d8d78020000  lea rcx, [rbp + 0x278]
1417a07ed: 488d95e0000000  lea rdx, [rbp + 0xe0]
1417a07f4: e87749b6ff  call 0x141305170
1417a07f9: 4c8b8d40030000  mov r9, qword ptr [rbp + 0x340]
1417a0800: 498b4118  mov rax, qword ptr [r9 + 0x18]
1417a0804: 488b08  mov rcx, qword ptr [rax]
1417a0807: 488b5008  mov rdx, qword ptr [rax + 8]
1417a080b: 4801ca  add rdx, rcx
1417a080e: 48ffc2  inc rdx
1417a0811: 660f6f01  movdqa xmm0, xmmword ptr [rcx]
1417a0815: 66440fd7c0  pmovmskb r8d, xmm0
1417a081a: 41f7d0  not r8d
1417a081d: 488b4018  mov rax, qword ptr [rax + 0x18]
1417a0821: 48898de0000000  mov qword ptr [rbp + 0xe0], rcx
1417a0828: 4883c110  add rcx, 0x10
1417a082c: 48898de8000000  mov qword ptr [rbp + 0xe8], rcx
1417a0833: 488995f0000000  mov qword ptr [rbp + 0xf0], rdx
1417a083a: 66448985f8000000  mov word ptr [rbp + 0xf8], r8w
1417a0842: 48898500010000  mov qword ptr [rbp + 0x100], rax
1417a0849: c6857203000001  mov byte ptr [rbp + 0x372], 1
1417a0850: c6857103000001  mov byte ptr [rbp + 0x371], 1
1417a0857: 4c8d054ab95301  lea r8, [rip + 0x153b94a]
1417a085e: 488d4dd8  lea rcx, [rbp - 0x28]
1417a0862: 488d95e0000000  lea rdx, [rbp + 0xe0]
1417a0869: 4c89ce  mov rsi, r9
1417a086c: e8cf82b6ff  call 0x141308b40
1417a0871: 488b8628010000  mov rax, qword ptr [rsi + 0x128]
1417a0878: 488b08  mov rcx, qword ptr [rax]
1417a087b: 488b5008  mov rdx, qword ptr [rax + 8]
1417a087f: 4801ca  add rdx, rcx
1417a0882: 48ffc2  inc rdx
1417a0885: 660f6f01  movdqa xmm0, xmmword ptr [rcx]
1417a0889: 66440fd7c0  pmovmskb r8d, xmm0
1417a088e: 41f7d0  not r8d
1417a0891: 488b4018  mov rax, qword ptr [rax + 0x18]
1417a0895: 48898de0000000  mov qword ptr [rbp + 0xe0], rcx
1417a089c: 4883c110  add rcx, 0x10
1417a08a0: 48898de8000000  mov qword ptr [rbp + 0xe8], rcx
1417a08a7: 488995f0000000  mov qword ptr [rbp + 0xf0], rdx
1417a08ae: 66448985f8000000  mov word ptr [rbp + 0xf8], r8w
1417a08b6: 48898500010000  mov qword ptr [rbp + 0x100], rax
1417a08bd: c6857003000001  mov byte ptr [rbp + 0x370], 1
1417a08c4: c6856f03000001  mov byte ptr [rbp + 0x36f], 1
1417a08cb: 4c8d05d6b85301  lea r8, [rip + 0x153b8d6]
1417a08d2: 488d8dc0000000  lea rcx, [rbp + 0xc0]
1417a08d9: 488d95e0000000  lea rdx, [rbp + 0xe0]
1417a08e0: e85b8fb6ff  call 0x141309840
1417a08e5: 488b8e30010000  mov rcx, qword ptr [rsi + 0x130]
1417a08ec: 80b97801000000  cmp byte ptr [rcx + 0x178], 0
1417a08f3: 0f84bf090000  je 0x1417a12b8
1417a08f9: 4883c128  add rcx, 0x28
1417a08fd: 48c7859002000000000000  mov qword ptr [rbp + 0x290], 0
1417a0908: 48c7859802000004000000  mov qword ptr [rbp + 0x298], 4
1417a0913: 48c785a002000000000000  mov qword ptr [rbp + 0x2a0], 0
1417a091e: c6856d03000001  mov byte ptr [rbp + 0x36d], 1
1417a0925: 488d9590020000  lea rdx, [rbp + 0x290]
1417a092c: e8cfc8ddff  call 0x14157d200
1417a0931: 488b85a0020000  mov rax, qword ptr [rbp + 0x2a0]
1417a0938: 488985f0000000  mov qword ptr [rbp + 0xf0], rax
1417a093f: f30f6f8590020000  movdqu xmm0, xmmword ptr [rbp + 0x290]
1417a0947: 660f7f85e0000000  movdqa xmmword ptr [rbp + 0xe0], xmm0
1417a094f: c6856d03000000  mov byte ptr [rbp + 0x36d], 0
1417a0956: 488d4df0  lea rcx, [rbp - 0x10]
1417a095a: 488d95e0000000  lea rdx, [rbp + 0xe0]
1417a0961: e8ba0716ff  call 0x140901120
1417a0966: 488b45e8  mov rax, qword ptr [rbp - 0x18]
1417a096a: 488985a0000000  mov qword ptr [rbp + 0xa0], rax
1417a0971: 0f1045d8  movups xmm0, xmmword ptr [rbp - 0x28]
1417a0975: 0f298590000000  movaps xmmword ptr [rbp + 0x90], xmm0
1417a097c: 4c8bad40030000  mov r13, qword ptr [rbp + 0x340]
1417a0983: 498b4500  mov rax, qword ptr [r13]
1417a0987: 488985f0020000  mov qword ptr [rbp + 0x2f0], rax
1417a098e: 498b4508  mov rax, qword ptr [r13 + 8]
1417a0992: 488985d8020000  mov qword ptr [rbp + 0x2d8], rax
1417a0999: 498b4510  mov rax, qword ptr [r13 + 0x10]
1417a099d: 488985d8000000  mov qword ptr [rbp + 0xd8], rax
1417a09a4: 498b4520  mov rax, qword ptr [r13 + 0x20]
1417a09a8: 488b08  mov rcx, qword ptr [rax]
1417a09ab: 488b5008  mov rdx, qword ptr [rax + 8]
1417a09af: 488d541101  lea rdx, [rcx + rdx + 1]
1417a09b4: 660f6f01  movdqa xmm0, xmmword ptr [rcx]
1417a09b8: 66440fd7c0  pmovmskb r8d, xmm0
1417a09bd: 41f7d0  not r8d
1417a09c0: 6644898518010000  mov word ptr [rbp + 0x118], r8w
1417a09c8: 48898d00010000  mov qword ptr [rbp + 0x100], rcx
1417a09cf: 4883c110  add rcx, 0x10
1417a09d3: 48898d08010000  mov qword ptr [rbp + 0x108], rcx
1417a09da: 48899510010000  mov qword ptr [rbp + 0x110], rdx
1417a09e1: 488b4818  mov rcx, qword ptr [rax + 0x18]
1417a09e5: 48898d20010000  mov qword ptr [rbp + 0x120], rcx
1417a09ec: 488d8890000000  lea rcx, [rax + 0x90]
1417a09f3: 488b9090000000  mov rdx, qword ptr [rax + 0x90]
1417a09fa: 4c8b8020010000  mov r8, qword ptr [rax + 0x120]
1417a0a01: 4885d2  test rdx, rdx
1417a0a04: 480f44ca  cmove rcx, rdx
1417a0a08: 480520010000  add rax, 0x120
1417a0a0e: 4d85c0  test r8, r8
1417a0a11: 490f44c0  cmove rax, r8
1417a0a15: 48c785f000000001000000  mov qword ptr [rbp + 0xf0], 1
1417a0a20: 48898df8000000  mov qword ptr [rbp + 0xf8], rcx
1417a0a27: 48c785e000000001000000  mov qword ptr [rbp + 0xe0], 1
1417a0a32: 488985e8000000  mov qword ptr [rbp + 0xe8], rax
1417a0a39: c6856c03000001  mov byte ptr [rbp + 0x36c], 1
1417a0a40: 4c8d0541803201  lea r8, [rip + 0x1328041]
1417a0a47: 488d4dc0  lea rcx, [rbp - 0x40]
1417a0a4b: 488d95e0000000  lea rdx, [rbp + 0xe0]
1417a0a52: e879b2b5ff  call 0x1412fbcd0
1417a0a57: 498b5528  mov rdx, qword ptr [r13 + 0x28]
1417a0a5b: 4d8b4d48  mov r9, qword ptr [r13 + 0x48]
1417a0a5f: 488d4208  lea rax, [rdx + 8]
1417a0a63: 4c8d4220  lea r8, [rdx + 0x20]
1417a0a67: 4c8d5210  lea r10, [rdx + 0x10]
1417a0a6b: 488d4a28  lea rcx, [rdx + 0x28]
1417a0a6f: 80ba9802000000  cmp byte ptr [rdx + 0x298], 0
1417a0a76: 490f45c0  cmovne rax, r8
1417a0a7a: 490f44ca  cmove rcx, r10
1417a0a7e: 498d5108  lea rdx, [r9 + 8]
1417a0a82: 4d8d5120  lea r10, [r9 + 0x20]
1417a0a86: 4d8d5910  lea r11, [r9 + 0x10]
1417a0a8a: 4d8d4128  lea r8, [r9 + 0x28]
1417a0a8e: 4180b99802000000  cmp byte ptr [r9 + 0x298], 0
1417a0a96: 490f45d2  cmovne rdx, r10
1417a0a9a: 4d0f44c3  cmove r8, r11
1417a0a9e: 4d8b5d68  mov r11, qword ptr [r13 + 0x68]
1417a0aa2: 4d8d4b08  lea r9, [r11 + 8]
1417a0aa6: 498d7320  lea rsi, [r11 + 0x20]
1417a0aaa: 498d7b10  lea rdi, [r11 + 0x10]
1417a0aae: 4d8d5328  lea r10, [r11 + 0x28]
1417a0ab2: 4180bb9802000000  cmp byte ptr [r11 + 0x298], 0
1417a0aba: 4c0f45ce  cmovne r9, rsi
1417a0abe: 4c0f44d7  cmove r10, rdi
1417a0ac2: 498bbd88000000  mov rdi, qword ptr [r13 + 0x88]
1417a0ac9: 4c8d5f08  lea r11, [rdi + 8]
1417a0acd: 488d5f20  lea rbx, [rdi + 0x20]
1417a0ad1: 4c8d7710  lea r14, [rdi + 0x10]
1417a0ad5: 488d7728  lea rsi, [rdi + 0x28]
1417a0ad9: 80bf9802000000  cmp byte ptr [rdi + 0x298], 0
1417a0ae0: 4c0f45db  cmovne r11, rbx
1417a0ae4: 490f44f6  cmove rsi, r14
1417a0ae8: 498b9da8000000  mov rbx, qword ptr [r13 + 0xa8]
1417a0aef: 4c8d7b08  lea r15, [rbx + 8]
1417a0af3: 4c8d7320  lea r14, [rbx + 0x20]
1417a0af7: 4c8d6310  lea r12, [rbx + 0x10]
1417a0afb: 488d7b28  lea rdi, [rbx + 0x28]
1417a0aff: 80bb9802000000  cmp byte ptr [rbx + 0x298], 0
1417a0b06: 4d0f45fe  cmovne r15, r14
1417a0b0a: 490f44fc  cmove rdi, r12
1417a0b0e: 498b9dc8000000  mov rbx, qword ptr [r13 + 0xc8]
1417a0b15: 4c8d7308  lea r14, [rbx + 8]
1417a0b19: 80bb9802000000  cmp byte ptr [rbx + 0x298], 0
1417a0b20: 4c8d6320  lea r12, [rbx + 0x20]
1417a0b24: 4d0f45f4  cmovne r14, r12
1417a0b28: 4c8d6310  lea r12, [rbx + 0x10]
1417a0b2c: 488d5b28  lea rbx, [rbx + 0x28]
1417a0b30: 490f44dc  cmove rbx, r12
1417a0b34: 4c8b21  mov r12, qword ptr [rcx]
1417a0b37: 4c8b28  mov r13, qword ptr [rax]
1417a0b3a: 498b00  mov rax, qword ptr [r8]
1417a0b3d: 488985f8020000  mov qword ptr [rbp + 0x2f8], rax
1417a0b44: 488b02  mov rax, qword ptr [rdx]
1417a0b47: 48898500030000  mov qword ptr [rbp + 0x300], rax
1417a0b4e: 498b02  mov rax, qword ptr [r10]
1417a0b51: 48898530030000  mov qword ptr [rbp + 0x330], rax
1417a0b58: 498b01  mov rax, qword ptr [r9]
1417a0b5b: 48898538030000  mov qword ptr [rbp + 0x338], rax
1417a0b62: 488b06  mov rax, qword ptr [rsi]
1417a0b65: 48898550030000  mov qword ptr [rbp + 0x350], rax
1417a0b6c: 498b33  mov rsi, qword ptr [r11]
1417a0b6f: 488b07  mov rax, qword ptr [rdi]
1417a0b72: 48898548030000  mov qword ptr [rbp + 0x348], rax
1417a0b79: 4d8b3f  mov r15, qword ptr [r15]
1417a0b7c: 488b3b  mov rdi, qword ptr [rbx]
1417a0b7f: 498b1e  mov rbx, qword ptr [r14]
1417a0b82: 0fb6058040e901  movzx eax, byte ptr [rip + 0x1e94080]
1417a0b89: b9f0000000  mov ecx, 0xf0
1417a0b8e: ba08000000  mov edx, 8
1417a0b93: e87830e7fe  call 0x140613c10
1417a0b98: 4885c0  test rax, rax
1417a0b9b: 0f846f070000  je 0x1417a1310
1417a0ba1: 488d0d56302f01  lea rcx, [rip + 0x12f3056]
1417a0ba8: 48898de0000000  mov qword ptr [rbp + 0xe0], rcx
1417a0baf: 48c785e80000000a000000  mov qword ptr [rbp + 0xe8], 0xa
1417a0bba: 4c89adf0000000  mov qword ptr [rbp + 0xf0], r13
1417a0bc1: 4c89a5f8000000  mov qword ptr [rbp + 0xf8], r12
1417a0bc8: 488d0d39302f01  lea rcx, [rip + 0x12f3039]
1417a0bcf: 48898d00010000  mov qword ptr [rbp + 0x100], rcx
1417a0bd6: 48c7850801000012000000  mov qword ptr [rbp + 0x108], 0x12
1417a0be1: 488b8d00030000  mov rcx, qword ptr [rbp + 0x300]
1417a0be8: 48898d10010000  mov qword ptr [rbp + 0x110], rcx
1417a0bef: 488b8df8020000  mov rcx, qword ptr [rbp + 0x2f8]
1417a0bf6: 48898d18010000  mov qword ptr [rbp + 0x118], rcx
1417a0bfd: 488d0d16302f01  lea rcx, [rip + 0x12f3016]
1417a0c04: 48898d20010000  mov qword ptr [rbp + 0x120], rcx
1417a0c0b: 48c7852801000012000000  mov qword ptr [rbp + 0x128], 0x12
1417a0c16: 488b8d38030000  mov rcx, qword ptr [rbp + 0x338]
1417a0c1d: 48898d30010000  mov qword ptr [rbp + 0x130], rcx
1417a0c24: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a0c2b: 48898d38010000  mov qword ptr [rbp + 0x138], rcx
1417a0c32: 488d0df32f2f01  lea rcx, [rip + 0x12f2ff3]
1417a0c39: 48898d40010000  mov qword ptr [rbp + 0x140], rcx
1417a0c40: 48c7854801000012000000  mov qword ptr [rbp + 0x148], 0x12
1417a0c4b: 4889b550010000  mov qword ptr [rbp + 0x150], rsi
1417a0c52: 488b8d50030000  mov rcx, qword ptr [rbp + 0x350]
1417a0c59: 48898d58010000  mov qword ptr [rbp + 0x158], rcx
1417a0c60: 488d0dd72f2f01  lea rcx, [rip + 0x12f2fd7]
1417a0c67: 48898d60010000  mov qword ptr [rbp + 0x160], rcx
1417a0c6e: 48c785680100000b000000  mov qword ptr [rbp + 0x168], 0xb
1417a0c79: 4c89bd70010000  mov qword ptr [rbp + 0x170], r15
1417a0c80: 488b8d48030000  mov rcx, qword ptr [rbp + 0x348]
1417a0c87: 48898d78010000  mov qword ptr [rbp + 0x178], rcx
1417a0c8e: 488d0db42f2f01  lea rcx, [rip + 0x12f2fb4]
1417a0c95: 48898d80010000  mov qword ptr [rbp + 0x180], rcx
1417a0c9c: 48c785880100000c000000  mov qword ptr [rbp + 0x188], 0xc
1417a0ca7: 48899d90010000  mov qword ptr [rbp + 0x190], rbx
1417a0cae: 4889bd98010000  mov qword ptr [rbp + 0x198], rdi
1417a0cb5: 488d8d20030000  lea rcx, [rbp + 0x320]
1417a0cbc: 48898d90020000  mov qword ptr [rbp + 0x290], rcx
1417a0cc3: 48c7851003000006000000  mov qword ptr [rbp + 0x310], 6
1417a0cce: 48898518030000  mov qword ptr [rbp + 0x318], rax
1417a0cd5: 48c7859802000000000000  mov qword ptr [rbp + 0x298], 0
1417a0ce0: 488985a0020000  mov qword ptr [rbp + 0x2a0], rax
1417a0ce7: 48c7852003000000000000  mov qword ptr [rbp + 0x320], 0
1417a0cf2: 48c785a001000000000000  mov qword ptr [rbp + 0x1a0], 0
1417a0cfd: 48c785a801000006000000  mov qword ptr [rbp + 0x1a8], 6
1417a0d08: 488d8de0000000  lea rcx, [rbp + 0xe0]
1417a0d0f: 488d9590020000  lea rdx, [rbp + 0x290]
1417a0d16: e8156fcdff  call 0x141477c30
1417a0d1b: f30f6f8510030000  movdqu xmm0, xmmword ptr [rbp + 0x310]
1417a0d23: 660f7f4570  movdqa xmmword ptr [rbp + 0x70], xmm0
1417a0d28: 488b8520030000  mov rax, qword ptr [rbp + 0x320]
1417a0d2f: 48898580000000  mov qword ptr [rbp + 0x80], rax
1417a0d36: 4c8bb540030000  mov r14, qword ptr [rbp + 0x340]
1417a0d3d: 498b86e8000000  mov rax, qword ptr [r14 + 0xe8]
1417a0d44: 488b4808  mov rcx, qword ptr [rax + 8]
1417a0d48: 488b4010  mov rax, qword ptr [rax + 0x10]
1417a0d4c: 488d0481  lea rax, [rcx + rax*4]
1417a0d50: 48898de0000000  mov qword ptr [rbp + 0xe0], rcx
1417a0d57: 488985e8000000  mov qword ptr [rbp + 0xe8], rax
1417a0d5e: 48c785f000000000000000  mov qword ptr [rbp + 0xf0], 0
1417a0d69: c6856b03000001  mov byte ptr [rbp + 0x36b], 1
1417a0d70: 4c8d0531b45301  lea r8, [rip + 0x153b431]
1417a0d77: 488d4d58  lea rcx, [rbp + 0x58]
1417a0d7b: 488d95e0000000  lea rdx, [rbp + 0xe0]
1417a0d82: e8e957b6ff  call 0x141306570
1417a0d87: 498b86f0000000  mov rax, qword ptr [r14 + 0xf0]
1417a0d8e: 488b4808  mov rcx, qword ptr [rax + 8]
1417a0d92: 488b4010  mov rax, qword ptr [rax + 0x10]
1417a0d96: 488d0481  lea rax, [rcx + rax*4]
1417a0d9a: 48898de0000000  mov qword ptr [rbp + 0xe0], rcx
1417a0da1: 488985e8000000  mov qword ptr [rbp + 0xe8], rax
1417a0da8: 48c785f000000000000000  mov qword ptr [rbp + 0xf0], 0
1417a0db3: c6856a03000001  mov byte ptr [rbp + 0x36a], 1
1417a0dba: 4c8d05e7b35301  lea r8, [rip + 0x153b3e7]
1417a0dc1: 488d4d40  lea rcx, [rbp + 0x40]
1417a0dc5: 488d95e0000000  lea rdx, [rbp + 0xe0]
1417a0dcc: e89f57b6ff  call 0x141306570
1417a0dd1: 498b86f8000000  mov rax, qword ptr [r14 + 0xf8]
1417a0dd8: 488b4808  mov rcx, qword ptr [rax + 8]
1417a0ddc: 488b4010  mov rax, qword ptr [rax + 0x10]
1417a0de0: 488d0481  lea rax, [rcx + rax*4]
1417a0de4: 48898de0000000  mov qword ptr [rbp + 0xe0], rcx
1417a0deb: 488985e8000000  mov qword ptr [rbp + 0xe8], rax
1417a0df2: 48c785f000000000000000  mov qword ptr [rbp + 0xf0], 0
1417a0dfd: c6856903000001  mov byte ptr [rbp + 0x369], 1
1417a0e04: 4c8d059db35301  lea r8, [rip + 0x153b39d]
1417a0e0b: 488d4d28  lea rcx, [rbp + 0x28]
1417a0e0f: 488d95e0000000  lea rdx, [rbp + 0xe0]
1417a0e16: e85557b6ff  call 0x141306570
1417a0e1b: 498bb600010000  mov rsi, qword ptr [r14 + 0x100]
1417a0e22: 488d5608  lea rdx, [rsi + 8]
1417a0e26: 488d8d98020000  lea rcx, [rbp + 0x298]
1417a0e2d: 0f107628  movups xmm6, xmmword ptr [rsi + 0x28]
1417a0e31: c6856803000001  mov byte ptr [rbp + 0x368], 1
1417a0e38: e863263dff  call 0x140b734a0
1417a0e3d: 0f11b5b8020000  movups xmmword ptr [rbp + 0x2b8], xmm6
1417a0e44: 488b4638  mov rax, qword ptr [rsi + 0x38]
1417a0e48: 8b0e  mov ecx, dword ptr [rsi]
1417a0e4a: 660f6e4604  movd xmm0, dword ptr [rsi + 4]
1417a0e4f: 0fb65640  movzx edx, byte ptr [rsi + 0x40]
1417a0e53: 488985c8020000  mov qword ptr [rbp + 0x2c8], rax
1417a0e5a: 898d90020000  mov dword ptr [rbp + 0x290], ecx
1417a0e60: 660f7e8594020000  movd dword ptr [rbp + 0x294], xmm0
1417a0e68: 8895d0020000  mov byte ptr [rbp + 0x2d0], dl
1417a0e6e: 498bb608010000  mov rsi, qword ptr [r14 + 0x108]
1417a0e75: 488b5e08  mov rbx, qword ptr [rsi + 8]
1417a0e79: 488b4610  mov rax, qword ptr [rsi + 0x10]
1417a0e7d: 488d3c8500000000  lea rdi, [rax*4]
1417a0e85: 4885c0  test rax, rax
1417a0e88: 7425  je 0x1417a0eaf
1417a0e8a: 0fb605783de901  movzx eax, byte ptr [rip + 0x1e93d78]
1417a0e91: ba01000000  mov edx, 1
1417a0e96: 4889f9  mov rcx, rdi
1417a0e99: e8722de7fe  call 0x140613c10
1417a0e9e: 4885c0  test rax, rax
1417a0ea1: 0f8499040000  je 0x1417a1340
1417a0ea7: 4989c7  mov r15, rax
1417a0eaa: 4989fc  mov r12, rdi
1417a0ead: eb09  jmp 0x1417a0eb8
1417a0eaf: 41bf01000000  mov r15d, 1
1417a0eb5: 4531e4  xor r12d, r12d
1417a0eb8: 4c89f9  mov rcx, r15
1417a0ebb: 4889da  mov rdx, rbx
1417a0ebe: 4989f8  mov r8, rdi
1417a0ec1: e83a2d1801  call 0x142923c00
1417a0ec6: f20f107e50  movsd xmm7, qword ptr [rsi + 0x50]
1417a0ecb: 488b85c0000000  mov rax, qword ptr [rbp + 0xc0]
1417a0ed2: 48898548030000  mov qword ptr [rbp + 0x348], rax
1417a0ed9: 488b85c8000000  mov rax, qword ptr [rbp + 0xc8]
1417a0ee0: 48898530030000  mov qword ptr [rbp + 0x330], rax
1417a0ee7: 488b85d0000000  mov rax, qword ptr [rbp + 0xd0]
1417a0eee: 48898500030000  mov qword ptr [rbp + 0x300], rax
1417a0ef5: 488b8588020000  mov rax, qword ptr [rbp + 0x288]
1417a0efc: 48898520030000  mov qword ptr [rbp + 0x320], rax
1417a0f03: f30f6f8578020000  movdqu xmm0, xmmword ptr [rbp + 0x278]
1417a0f0b: 660f7f8510030000  movdqa xmmword ptr [rbp + 0x310], xmm0
1417a0f13: 488b4500  mov rax, qword ptr [rbp]
1417a0f17: 48898550030000  mov qword ptr [rbp + 0x350], rax
1417a0f1e: 488b4508  mov rax, qword ptr [rbp + 8]
1417a0f22: 48898538030000  mov qword ptr [rbp + 0x338], rax
1417a0f29: 488b4510  mov rax, qword ptr [rbp + 0x10]
1417a0f2d: 488985f8020000  mov qword ptr [rbp + 0x2f8], rax
1417a0f34: 498bb638010000  mov rsi, qword ptr [r14 + 0x138]
1417a0f3b: 0f107620  movups xmm6, xmmword ptr [rsi + 0x20]
1417a0f3f: 4c8b6e08  mov r13, qword ptr [rsi + 8]
1417a0f43: 4d85ed  test r13, r13
1417a0f46: 7452  je 0x1417a0f9a
1417a0f48: 4c89a508030000  mov qword ptr [rbp + 0x308], r12
1417a0f4f: 4c89bde0020000  mov qword ptr [rbp + 0x2e0], r15
1417a0f56: 498d4501  lea rax, [r13 + 1]
1417a0f5a: 48c1e83d  shr rax, 0x3d
1417a0f5e: 7531  jne 0x1417a0f91
1417a0f60: 4e8d24ed08000000  lea r12, [r13*8 + 8]
1417a0f68: 4983fcf0  cmp r12, -0x10
1417a0f6c: 7723  ja 0x1417a0f91
1417a0f6e: 4983c40f  add r12, 0xf
1417a0f72: 4983e4f0  and r12, 0xfffffffffffffff0
1417a0f76: 4d8d7d11  lea r15, [r13 + 0x11]
1417a0f7a: 4c89e3  mov rbx, r12
1417a0f7d: 4c01fb  add rbx, r15
1417a0f80: 720f  jb 0x1417a0f91
1417a0f82: 48b8f1ffffffffffff7f  movabs rax, 0x7ffffffffffffff1
1417a0f8c: 4839c3  cmp rbx, rax
1417a0f8f: 721b  jb 0x1417a0fac
1417a0f91: b101  mov cl, 1
1417a0f93: e8e8835dff  call 0x140d79380
1417a0f98: eb45  jmp 0x1417a0fdf
1417a0f9a: 660fefc0  pxor xmm0, xmm0
1417a0f9e: 4c8d35fb3b3501  lea r14, [rip + 0x1353bfb]
1417a0fa5: 31db  xor ebx, ebx
1417a0fa7: e982000000  jmp 0x1417a102e
1417a0fac: 0fb605563ce901  movzx eax, byte ptr [rip + 0x1e93c56]
1417a0fb3: ba10000000  mov edx, 0x10
1417a0fb8: 4889d9  mov rcx, rbx
1417a0fbb: e8502ce7fe  call 0x140613c10
1417a0fc0: 4885c0  test rax, rax
1417a0fc3: 740b  je 0x1417a0fd0
1417a0fc5: 4989c6  mov r14, rax
1417a0fc8: 4d01e6  add r14, r12
1417a0fcb: 4c89eb  mov rbx, r13
1417a0fce: eb1c  jmp 0x1417a0fec
1417a0fd0: ba10000000  mov edx, 0x10
1417a0fd5: b101  mov cl, 1
1417a0fd7: 4989d8  mov r8, rbx
1417a0fda: e8f1835dff  call 0x140d793d0
1417a0fdf: 4889c3  mov rbx, rax
1417a0fe2: 4989c7  mov r15, rax
1417a0fe5: 4983c711  add r15, 0x11
1417a0fe9: 4531f6  xor r14d, r14d
1417a0fec: 4c8b26  mov r12, qword ptr [rsi]
1417a0fef: 4c89f1  mov rcx, r14
1417a0ff2: 4c89e2  mov rdx, r12
1417a0ff5: 4d89f8  mov r8, r15
1417a0ff8: e8032c1801  call 0x142923c00
1417a0ffd: 49f7d5  not r13
1417a1000: 4b8d14ec  lea rdx, [r12 + r13*8]
1417a1004: 4889d8  mov rax, rbx
1417a1007: 48f7d0  not rax
1417a100a: 498d0cc6  lea rcx, [r14 + rax*8]
1417a100e: 4c8d04dd08000000  lea r8, [rbx*8 + 8]
1417a1016: e8e52b1801  call 0x142923c00
1417a101b: f30f6f4610  movdqu xmm0, xmmword ptr [rsi + 0x10]
1417a1020: 4c8bbde0020000  mov r15, qword ptr [rbp + 0x2e0]
1417a1027: 4c8ba508030000  mov r12, qword ptr [rbp + 0x308]
1417a102e: 488b85a0000000  mov rax, qword ptr [rbp + 0xa0]
1417a1035: 48898518010000  mov qword ptr [rbp + 0x118], rax
1417a103c: 0f288d90000000  movaps xmm1, xmmword ptr [rbp + 0x90]
1417a1043: 0f118d08010000  movups xmmword ptr [rbp + 0x108], xmm1
1417a104a: 488b85f0020000  mov rax, qword ptr [rbp + 0x2f0]
1417a1051: 48898520010000  mov qword ptr [rbp + 0x120], rax
1417a1058: 488b85d8020000  mov rax, qword ptr [rbp + 0x2d8]
1417a105f: 48898528010000  mov qword ptr [rbp + 0x128], rax
1417a1066: 488b85d8000000  mov rax, qword ptr [rbp + 0xd8]
1417a106d: 48898530010000  mov qword ptr [rbp + 0x130], rax
1417a1074: 0f104dc0  movups xmm1, xmmword ptr [rbp - 0x40]
1417a1078: 0f118d38010000  movups xmmword ptr [rbp + 0x138], xmm1
1417a107f: 488b45d0  mov rax, qword ptr [rbp - 0x30]
1417a1083: 48898548010000  mov qword ptr [rbp + 0x148], rax
1417a108a: 0f284d70  movaps xmm1, xmmword ptr [rbp + 0x70]
1417a108e: 0f298d50010000  movaps xmmword ptr [rbp + 0x150], xmm1
1417a1095: 488b8580000000  mov rax, qword ptr [rbp + 0x80]
1417a109c: 48898560010000  mov qword ptr [rbp + 0x160], rax
1417a10a3: 0f104d58  movups xmm1, xmmword ptr [rbp + 0x58]
1417a10a7: 0f118d68010000  movups xmmword ptr [rbp + 0x168], xmm1
1417a10ae: 488b4568  mov rax, qword ptr [rbp + 0x68]
1417a10b2: 48898578010000  mov qword ptr [rbp + 0x178], rax
1417a10b9: 488b4550  mov rax, qword ptr [rbp + 0x50]
1417a10bd: 48898590010000  mov qword ptr [rbp + 0x190], rax
1417a10c4: 0f104d40  movups xmm1, xmmword ptr [rbp + 0x40]
1417a10c8: 0f298d80010000  movaps xmmword ptr [rbp + 0x180], xmm1
1417a10cf: 0f104d28  movups xmm1, xmmword ptr [rbp + 0x28]
1417a10d3: 0f118d98010000  movups xmmword ptr [rbp + 0x198], xmm1
1417a10da: 488b4538  mov rax, qword ptr [rbp + 0x38]
1417a10de: 488985a8010000  mov qword ptr [rbp + 0x1a8], rax
1417a10e5: 488b85d0020000  mov rax, qword ptr [rbp + 0x2d0]
1417a10ec: 48898538020000  mov qword ptr [rbp + 0x238], rax
1417a10f3: 0f108d90020000  movups xmm1, xmmword ptr [rbp + 0x290]
1417a10fa: 0f1095a0020000  movups xmm2, xmmword ptr [rbp + 0x2a0]
1417a1101: 0f109db0020000  movups xmm3, xmmword ptr [rbp + 0x2b0]
1417a1108: 0f10a5c0020000  movups xmm4, xmmword ptr [rbp + 0x2c0]
1417a110f: 0f118df8010000  movups xmmword ptr [rbp + 0x1f8], xmm1
1417a1116: 0f119508020000  movups xmmword ptr [rbp + 0x208], xmm2
1417a111d: 0f119d18020000  movups xmmword ptr [rbp + 0x218], xmm3
1417a1124: 0f11a528020000  movups xmmword ptr [rbp + 0x228], xmm4
1417a112b: 4c89a5b0010000  mov qword ptr [rbp + 0x1b0], r12
1417a1132: 4c89bdb8010000  mov qword ptr [rbp + 0x1b8], r15
1417a1139: 4889bdc0010000  mov qword ptr [rbp + 0x1c0], rdi
1417a1140: 0f13bd70020000  movlps qword ptr [rbp + 0x270], xmm7
1417a1147: 488b8548030000  mov rax, qword ptr [rbp + 0x348]
1417a114e: 488985c8010000  mov qword ptr [rbp + 0x1c8], rax
1417a1155: 488b8530030000  mov rax, qword ptr [rbp + 0x330]
1417a115c: 488985d0010000  mov qword ptr [rbp + 0x1d0], rax
1417a1163: 488b8500030000  mov rax, qword ptr [rbp + 0x300]
1417a116a: 488985d8010000  mov qword ptr [rbp + 0x1d8], rax
1417a1171: 488b8520030000  mov rax, qword ptr [rbp + 0x320]
1417a1178: 488985f0010000  mov qword ptr [rbp + 0x1f0], rax
1417a117f: 0f288d10030000  movaps xmm1, xmmword ptr [rbp + 0x310]
1417a1186: 0f298de0010000  movaps xmmword ptr [rbp + 0x1e0], xmm1
1417a118d: 0f104df0  movups xmm1, xmmword ptr [rbp - 0x10]
1417a1191: 0f298de0000000  movaps xmmword ptr [rbp + 0xe0], xmm1
1417a1198: 488b8550030000  mov rax, qword ptr [rbp + 0x350]
1417a119f: 488985f0000000  mov qword ptr [rbp + 0xf0], rax
1417a11a6: 488b8538030000  mov rax, qword ptr [rbp + 0x338]
1417a11ad: 488985f8000000  mov qword ptr [rbp + 0xf8], rax
1417a11b4: 488b85f8020000  mov rax, qword ptr [rbp + 0x2f8]
1417a11bb: 48898500010000  mov qword ptr [rbp + 0x100], rax
1417a11c2: 4c89b540020000  mov qword ptr [rbp + 0x240], r14
1417a11c9: 48899d48020000  mov qword ptr [rbp + 0x248], rbx
1417a11d0: 660f7f8550020000  movdqa xmmword ptr [rbp + 0x250], xmm0
1417a11d8: 0f29b560020000  movaps xmmword ptr [rbp + 0x260], xmm6
1417a11df: 0fb605233ae901  movzx eax, byte ptr [rip + 0x1e93a23]
1417a11e6: b998010000  mov ecx, 0x198
1417a11eb: ba08000000  mov edx, 8
1417a11f0: e81b2ae7fe  call 0x140613c10
1417a11f5: 4885c0  test rax, rax
1417a11f8: 0fb69d58030000  movzx ebx, byte ptr [rbp + 0x358]
1417a11ff: 0f842a010000  je 0x1417a132f
1417a1205: 4889c7  mov rdi, rax
1417a1208: 488d95e0000000  lea rdx, [rbp + 0xe0]
1417a120f: 41b898010000  mov r8d, 0x198
1417a1215: 4889c1  mov rcx, rax
1417a1218: e8e3291801  call 0x142923c00
1417a121d: 84db  test bl, bl
1417a121f: 7472  je 0x1417a1293
1417a1221: e80a6f47ff  call 0x140c18130
1417a1226: 4889c1  mov rcx, rax
1417a1229: 488b00  mov rax, qword ptr [rax]
1417a122c: 4885c0  test rax, rax
1417a122f: 7416  je 0x1417a1247
1417a1231: 83f801  cmp eax, 1
1417a1234: 488b95e8020000  mov rdx, qword ptr [rbp + 0x2e8]
1417a123b: 0f8531010000  jne 0x1417a1372
1417a1241: 4883c108  add rcx, 8
1417a1245: eb21  jmp 0x1417a1268
1417a1247: c6857703000000  mov byte ptr [rbp + 0x377], 0
1417a124e: 31d2  xor edx, edx
1417a1250: e87b751501  call 0x1428f87d0
1417a1255: 4889c1  mov rcx, rax
1417a1258: 4885c0  test rax, rax
1417a125b: 488b95e8020000  mov rdx, qword ptr [rbp + 0x2e8]
1417a1262: 0f840a010000  je 0x1417a1372
1417a1268: 48833900  cmp qword ptr [rcx], 0
1417a126c: 0f8589000000  jne 0x1417a12fb
1417a1272: 48c701ffffffff  mov qword ptr [rcx], 0xffffffffffffffff
1417a1279: 48898d58030000  mov qword ptr [rbp + 0x358], rcx
1417a1280: 4883c108  add rcx, 8
1417a1284: e8675e47ff  call 0x140c170f0
1417a1289: 488b8558030000  mov rax, qword ptr [rbp + 0x358]
1417a1290: 48ff00  inc qword ptr [rax]
1417a1293: 4889f8  mov rax, rdi
1417a1296: 0f28b580030000  movaps xmm6, xmmword ptr [rbp + 0x380]
1417a129d: 0f28bd90030000  movaps xmm7, xmmword ptr [rbp + 0x390]
1417a12a4: 4881c428040000  add rsp, 0x428
1417a12ab: 5b  pop rbx
1417a12ac: 5f  pop rdi
1417a12ad: 5e  pop rsi
1417a12ae: 415c  pop r12
1417a12b0: 415d  pop r13
1417a12b2: 415e  pop r14
1417a12b4: 415f  pop r15
1417a12b6: 5d  pop rbp
1417a12b7: c3  ret
1417a12b8: c6857603000001  mov byte ptr [rbp + 0x376], 1
1417a12bf: c6857503000001  mov byte ptr [rbp + 0x375], 1
1417a12c6: 488d0d2bb25301  lea rcx, [rip + 0x153b22b]
1417a12cd: 4c8d0554b25301  lea r8, [rip + 0x153b254]
1417a12d4: ba2e000000  mov edx, 0x2e
1417a12d9: e852811301  call 0x1428d9430
1417a12de: e9a2000000  jmp 0x1417a1385
1417a12e3: c6857703000001  mov byte ptr [rbp + 0x377], 1
1417a12ea: 488d0d7f984901  lea rcx, [rip + 0x149987f]
1417a12f1: e85a7f1301  call 0x1428d9250
1417a12f6: e98a000000  jmp 0x1417a1385
1417a12fb: c6857703000000  mov byte ptr [rbp + 0x377], 0
1417a1302: 488d0d67984901  lea rcx, [rip + 0x1499867]
1417a1309: e8427f1301  call 0x1428d9250
1417a130e: eb75  jmp 0x1417a1385
1417a1310: c6857403000001  mov byte ptr [rbp + 0x374], 1
1417a1317: 4c8d058aae5301  lea r8, [rip + 0x153ae8a]
1417a131e: b908000000  mov ecx, 8
1417a1323: baf0000000  mov edx, 0xf0
1417a1328: e8b67c1301  call 0x1428d8fe3
1417a132d: eb56  jmp 0x1417a1385
1417a132f: b908000000  mov ecx, 8
1417a1334: ba98010000  mov edx, 0x198
1417a1339: e8c27c1301  call 0x1428d9000
1417a133e: eb45  jmp 0x1417a1385
1417a1340: c6856703000001  mov byte ptr [rbp + 0x367], 1
1417a1347: 4c8d05d2af5301  lea r8, [rip + 0x153afd2]
1417a134e: b901000000  mov ecx, 1
1417a1353: 4889fa  mov rdx, rdi
1417a1356: e8887c1301  call 0x1428d8fe3
1417a135b: eb28  jmp 0x1417a1385
1417a135d: c6857703000001  mov byte ptr [rbp + 0x377], 1
1417a1364: 488d0d35974901  lea rcx, [rip + 0x1499735]
1417a136b: e8606d1301  call 0x1428d80d0
1417a1370: eb13  jmp 0x1417a1385
1417a1372: c6857703000000  mov byte ptr [rbp + 0x377], 0
1417a1379: 488d0d20974901  lea rcx, [rip + 0x1499720]
1417a1380: e84b6d1301  call 0x1428d80d0
1417a1385: 0f0b  ud2
1417a1387: 660f1f840000000000  nop word ptr [rax + rax]
1417a1390: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a1395: 55  push rbp
1417a1396: 4157  push r15
1417a1398: 4156  push r14
1417a139a: 4155  push r13
1417a139c: 4154  push r12
1417a139e: 56  push rsi
1417a139f: 57  push rdi
1417a13a0: 53  push rbx
1417a13a1: 4883ec58  sub rsp, 0x58
1417a13a5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a13ac: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1417a13b1: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1417a13b6: 488b8558030000  mov rax, qword ptr [rbp + 0x358]
1417a13bd: 48ff00  inc qword ptr [rax]
1417a13c0: c6857703000001  mov byte ptr [rbp + 0x377], 1
1417a13c7: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a13cc: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a13d1: 4883c458  add rsp, 0x58
1417a13d5: 5b  pop rbx
1417a13d6: 5f  pop rdi
1417a13d7: 5e  pop rsi
1417a13d8: 415c  pop r12
1417a13da: 415d  pop r13
1417a13dc: 415e  pop r14
1417a13de: 415f  pop r15
1417a13e0: 5d  pop rbp
1417a13e1: c3  ret
1417a13e2: 66666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a13f0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a13f5: 55  push rbp
1417a13f6: 4157  push r15
1417a13f8: 4156  push r14
1417a13fa: 4155  push r13
1417a13fc: 4154  push r12
1417a13fe: 56  push rsi
1417a13ff: 57  push rdi
1417a1400: 53  push rbx
1417a1401: 4883ec58  sub rsp, 0x58
1417a1405: 488daa80000000  lea rbp, [rdx + 0x80]
1417a140c: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1417a1411: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1417a1416: 0fb69d73030000  movzx ebx, byte ptr [rbp + 0x373]
1417a141d: 80bd5803000000  cmp byte ptr [rbp + 0x358], 0
1417a1424: 742c  je 0x1417a1452
1417a1426: 488d4520  lea rax, [rbp + 0x20]
1417a142a: 488985b8000000  mov qword ptr [rbp + 0xb8], rax
1417a1431: 488d85b8000000  lea rax, [rbp + 0xb8]
1417a1438: 488985b0000000  mov qword ptr [rbp + 0xb0], rax
1417a143f: 488d0dbaaf5301  lea rcx, [rip + 0x153afba]
1417a1446: 488d95b0000000  lea rdx, [rbp + 0xb0]
1417a144d: e87edeb8ff  call 0x14132f2d0
1417a1452: 889d77030000  mov byte ptr [rbp + 0x377], bl
1417a1458: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a145d: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a1462: 4883c458  add rsp, 0x58
1417a1466: 5b  pop rbx
1417a1467: 5f  pop rdi
1417a1468: 5e  pop rsi
1417a1469: 415c  pop r12
1417a146b: 415d  pop r13
1417a146d: 415e  pop r14
1417a146f: 415f  pop r15
1417a1471: 5d  pop rbp
1417a1472: c3  ret
1417a1473: 666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a1480: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a1485: 55  push rbp
1417a1486: 4157  push r15
1417a1488: 4156  push r14
1417a148a: 4155  push r13
1417a148c: 4154  push r12
1417a148e: 56  push rsi
1417a148f: 57  push rdi
1417a1490: 53  push rbx
1417a1491: 4883ec58  sub rsp, 0x58
1417a1495: 488daa80000000  lea rbp, [rdx + 0x80]
1417a149c: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1417a14a1: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1417a14a6: 80bd6d03000000  cmp byte ptr [rbp + 0x36d], 0
1417a14ad: 7422  je 0x1417a14d1
1417a14af: 488b9590020000  mov rdx, qword ptr [rbp + 0x290]
1417a14b6: 4885d2  test rdx, rdx
1417a14b9: 7416  je 0x1417a14d1
1417a14bb: 488b8d98020000  mov rcx, qword ptr [rbp + 0x298]
1417a14c2: 48c1e203  shl rdx, 3
1417a14c6: 41b804000000  mov r8d, 4
1417a14cc: e84f27e7fe  call 0x140613c20
1417a14d1: c6857603000001  mov byte ptr [rbp + 0x376], 1
1417a14d8: c6857503000001  mov byte ptr [rbp + 0x375], 1
1417a14df: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a14e4: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a14e9: 4883c458  add rsp, 0x58
1417a14ed: 5b  pop rbx
1417a14ee: 5f  pop rdi
1417a14ef: 5e  pop rsi
1417a14f0: 415c  pop r12
1417a14f2: 415d  pop r13
1417a14f4: 415e  pop r14
1417a14f6: 415f  pop r15
1417a14f8: 5d  pop rbp
1417a14f9: c3  ret
1417a14fa: 660f1f440000  nop word ptr [rax + rax]
1417a1500: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a1505: 55  push rbp
1417a1506: 4157  push r15
1417a1508: 4156  push r14
1417a150a: 4155  push r13
1417a150c: 4154  push r12
1417a150e: 56  push rsi
1417a150f: 57  push rdi
1417a1510: 53  push rbx
1417a1511: 4883ec58  sub rsp, 0x58
1417a1515: 488daa80000000  lea rbp, [rdx + 0x80]
1417a151c: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1417a1521: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1417a1526: 0fb69d6c030000  movzx ebx, byte ptr [rbp + 0x36c]
1417a152d: 4883bdf002000000  cmp qword ptr [rbp + 0x2f0], 0
1417a1535: 741a  je 0x1417a1551
1417a1537: 486b95f002000058  imul rdx, qword ptr [rbp + 0x2f0], 0x58
1417a153f: 41b808000000  mov r8d, 8
1417a1545: 488b8dd8020000  mov rcx, qword ptr [rbp + 0x2d8]
1417a154c: e8cf26e7fe  call 0x140613c20
1417a1551: 488d8d90000000  lea rcx, [rbp + 0x90]
1417a1558: e8f3e10dff  call 0x14087f750
1417a155d: 889d6e030000  mov byte ptr [rbp + 0x36e], bl
1417a1563: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a1568: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a156d: 4883c458  add rsp, 0x58
1417a1571: 5b  pop rbx
1417a1572: 5f  pop rdi
1417a1573: 5e  pop rsi
1417a1574: 415c  pop r12
1417a1576: 415d  pop r13
1417a1578: 415e  pop r14
1417a157a: 415f  pop r15
1417a157c: 5d  pop rbp
1417a157d: c3  ret
1417a157e: 6690  nop
1417a1580: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a1585: 55  push rbp
1417a1586: 4157  push r15
1417a1588: 4156  push r14
1417a158a: 4155  push r13
1417a158c: 4154  push r12
1417a158e: 56  push rsi
1417a158f: 57  push rdi
1417a1590: 53  push rbx
1417a1591: 4883ec58  sub rsp, 0x58
1417a1595: 488daa80000000  lea rbp, [rdx + 0x80]
1417a159c: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1417a15a1: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1417a15a6: 0fb69d74030000  movzx ebx, byte ptr [rbp + 0x374]
1417a15ad: 488d4dc0  lea rcx, [rbp - 0x40]
1417a15b1: e8ea0f0eff  call 0x1408825a0
1417a15b6: 889d6c030000  mov byte ptr [rbp + 0x36c], bl
1417a15bc: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a15c1: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a15c6: 4883c458  add rsp, 0x58
1417a15ca: 5b  pop rbx
1417a15cb: 5f  pop rdi
1417a15cc: 5e  pop rsi
1417a15cd: 415c  pop r12
1417a15cf: 415d  pop r13
1417a15d1: 415e  pop r14
1417a15d3: 415f  pop r15
1417a15d5: 5d  pop rbp
1417a15d6: c3  ret
1417a15d7: 660f1f840000000000  nop word ptr [rax + rax]
1417a15e0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a15e5: 55  push rbp
1417a15e6: 4157  push r15
1417a15e8: 4156  push r14
1417a15ea: 4155  push r13
1417a15ec: 4154  push r12
1417a15ee: 56  push rsi
1417a15ef: 57  push rdi
1417a15f0: 53  push rbx
1417a15f1: 4883ec58  sub rsp, 0x58
1417a15f5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a15fc: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1417a1601: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1417a1606: c6857403000001  mov byte ptr [rbp + 0x374], 1
1417a160d: 488d8d10030000  lea rcx, [rbp + 0x310]
1417a1614: e8a7b4e4fe  call 0x1405ecac0
1417a1619: c6857403000001  mov byte ptr [rbp + 0x374], 1
1417a1620: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a1625: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a162a: 4883c458  add rsp, 0x58
1417a162e: 5b  pop rbx
1417a162f: 5f  pop rdi
1417a1630: 5e  pop rsi
1417a1631: 415c  pop r12
1417a1633: 415d  pop r13
1417a1635: 415e  pop r14
1417a1637: 415f  pop r15
1417a1639: 5d  pop rbp
1417a163a: c3  ret
1417a163b: 0f1f440000  nop dword ptr [rax + rax]
1417a1640: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a1645: 55  push rbp
1417a1646: 4157  push r15
1417a1648: 4156  push r14
1417a164a: 4155  push r13
1417a164c: 4154  push r12
1417a164e: 56  push rsi
1417a164f: 57  push rdi
1417a1650: 53  push rbx
1417a1651: 4883ec58  sub rsp, 0x58
1417a1655: 488daa80000000  lea rbp, [rdx + 0x80]
1417a165c: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1417a1661: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1417a1666: 0fb69d6a030000  movzx ebx, byte ptr [rbp + 0x36a]
1417a166d: 488b5558  mov rdx, qword ptr [rbp + 0x58]
1417a1671: 4885d2  test rdx, rdx
1417a1674: 7413  je 0x1417a1689
1417a1676: 488b4d60  mov rcx, qword ptr [rbp + 0x60]
1417a167a: 48c1e204  shl rdx, 4
1417a167e: 41b808000000  mov r8d, 8
1417a1684: e89725e7fe  call 0x140613c20
1417a1689: 889d6b030000  mov byte ptr [rbp + 0x36b], bl
1417a168f: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a1694: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a1699: 4883c458  add rsp, 0x58
1417a169d: 5b  pop rbx
1417a169e: 5f  pop rdi
1417a169f: 5e  pop rsi
1417a16a0: 415c  pop r12
1417a16a2: 415d  pop r13
1417a16a4: 415e  pop r14
1417a16a6: 415f  pop r15
1417a16a8: 5d  pop rbp
1417a16a9: c3  ret
1417a16aa: 660f1f440000  nop word ptr [rax + rax]
1417a16b0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a16b5: 55  push rbp
1417a16b6: 4157  push r15
1417a16b8: 4156  push r14
1417a16ba: 4155  push r13
1417a16bc: 4154  push r12
1417a16be: 56  push rsi
1417a16bf: 57  push rdi
1417a16c0: 53  push rbx
1417a16c1: 4883ec58  sub rsp, 0x58
1417a16c5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a16cc: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1417a16d1: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1417a16d6: 0fb69d69030000  movzx ebx, byte ptr [rbp + 0x369]
1417a16dd: 488b5540  mov rdx, qword ptr [rbp + 0x40]
1417a16e1: 4885d2  test rdx, rdx
1417a16e4: 7413  je 0x1417a16f9
1417a16e6: 488b4d48  mov rcx, qword ptr [rbp + 0x48]
1417a16ea: 48c1e204  shl rdx, 4
1417a16ee: 41b808000000  mov r8d, 8
1417a16f4: e82725e7fe  call 0x140613c20
1417a16f9: 889d6a030000  mov byte ptr [rbp + 0x36a], bl
1417a16ff: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a1704: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a1709: 4883c458  add rsp, 0x58
1417a170d: 5b  pop rbx
1417a170e: 5f  pop rdi
1417a170f: 5e  pop rsi
1417a1710: 415c  pop r12
1417a1712: 415d  pop r13
1417a1714: 415e  pop r14
1417a1716: 415f  pop r15
1417a1718: 5d  pop rbp
1417a1719: c3  ret
1417a171a: 660f1f440000  nop word ptr [rax + rax]
1417a1720: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a1725: 55  push rbp
1417a1726: 4157  push r15
1417a1728: 4156  push r14
1417a172a: 4155  push r13
1417a172c: 4154  push r12
1417a172e: 56  push rsi
1417a172f: 57  push rdi
1417a1730: 53  push rbx
1417a1731: 4883ec58  sub rsp, 0x58
1417a1735: 488daa80000000  lea rbp, [rdx + 0x80]
1417a173c: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1417a1741: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1417a1746: 0fb69d68030000  movzx ebx, byte ptr [rbp + 0x368]
1417a174d: 488b5528  mov rdx, qword ptr [rbp + 0x28]
1417a1751: 4885d2  test rdx, rdx
1417a1754: 7413  je 0x1417a1769
1417a1756: 488b4d30  mov rcx, qword ptr [rbp + 0x30]
1417a175a: 48c1e204  shl rdx, 4
1417a175e: 41b808000000  mov r8d, 8
1417a1764: e8b724e7fe  call 0x140613c20
1417a1769: 889d69030000  mov byte ptr [rbp + 0x369], bl
1417a176f: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a1774: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a1779: 4883c458  add rsp, 0x58
1417a177d: 5b  pop rbx
1417a177e: 5f  pop rdi
1417a177f: 5e  pop rsi
1417a1780: 415c  pop r12
1417a1782: 415d  pop r13
1417a1784: 415e  pop r14
1417a1786: 415f  pop r15
1417a1788: 5d  pop rbp
1417a1789: c3  ret
1417a178a: 660f1f440000  nop word ptr [rax + rax]
1417a1790: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a1795: 55  push rbp
1417a1796: 4157  push r15
1417a1798: 4156  push r14
1417a179a: 4155  push r13
1417a179c: 4154  push r12
1417a179e: 56  push rsi
1417a179f: 57  push rdi
1417a17a0: 53  push rbx
1417a17a1: 4883ec58  sub rsp, 0x58
1417a17a5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a17ac: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1417a17b1: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1417a17b6: 0fb69d67030000  movzx ebx, byte ptr [rbp + 0x367]
1417a17bd: 41b830000000  mov r8d, 0x30
1417a17c3: 41b910000000  mov r9d, 0x10
1417a17c9: 488d8d98020000  lea rcx, [rbp + 0x298]
1417a17d0: 488d95b8020000  lea rdx, [rbp + 0x2b8]
1417a17d7: e894dd8afe  call 0x14004f570
1417a17dc: 889d68030000  mov byte ptr [rbp + 0x368], bl
1417a17e2: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a17e7: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a17ec: 4883c458  add rsp, 0x58
1417a17f0: 5b  pop rbx
1417a17f1: 5f  pop rdi
1417a17f2: 5e  pop rsi
1417a17f3: 415c  pop r12
1417a17f5: 415d  pop r13
1417a17f7: 415e  pop r14
1417a17f9: 415f  pop r15
1417a17fb: 5d  pop rbp
1417a17fc: c3  ret
1417a17fd: 0f1f00  nop dword ptr [rax]
1417a1800: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a1805: 55  push rbp
1417a1806: 4157  push r15
1417a1808: 4156  push r14
1417a180a: 4155  push r13
1417a180c: 4154  push r12
1417a180e: 56  push rsi
1417a180f: 57  push rdi
1417a1810: 53  push rbx
1417a1811: 4883ec58  sub rsp, 0x58
1417a1815: 488daa80000000  lea rbp, [rdx + 0x80]
1417a181c: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1417a1821: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1417a1826: 4883bd5003000000  cmp qword ptr [rbp + 0x350], 0
1417a182e: 755b  jne 0x1417a188b
1417a1830: 488d8d10030000  lea rcx, [rbp + 0x310]
1417a1837: e8e4da0dff  call 0x14087f320
1417a183c: 4883bd4803000000  cmp qword ptr [rbp + 0x348], 0
1417a1844: 7578  jne 0x1417a18be
1417a1846: 4883bd0803000000  cmp qword ptr [rbp + 0x308], 0
1417a184e: 7419  je 0x1417a1869
1417a1850: 41b801000000  mov r8d, 1
1417a1856: 488b8de0020000  mov rcx, qword ptr [rbp + 0x2e0]
1417a185d: 488b9508030000  mov rdx, qword ptr [rbp + 0x308]
1417a1864: e8b723e7fe  call 0x140613c20
1417a1869: c6856703000000  mov byte ptr [rbp + 0x367], 0
1417a1870: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a1875: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a187a: 4883c458  add rsp, 0x58
1417a187e: 5b  pop rbx
1417a187f: 5f  pop rdi
1417a1880: 5e  pop rsi
1417a1881: 415c  pop r12
1417a1883: 415d  pop r13
1417a1885: 415e  pop r14
1417a1887: 415f  pop r15
1417a1889: 5d  pop rbp
1417a188a: c3  ret
1417a188b: 488b9550030000  mov rdx, qword ptr [rbp + 0x350]
1417a1892: 48c1e203  shl rdx, 3
1417a1896: 41b804000000  mov r8d, 4
1417a189c: 488b8d38030000  mov rcx, qword ptr [rbp + 0x338]
1417a18a3: e87823e7fe  call 0x140613c20
1417a18a8: 488d8d10030000  lea rcx, [rbp + 0x310]
1417a18af: e86cda0dff  call 0x14087f320
1417a18b4: 4883bd4803000000  cmp qword ptr [rbp + 0x348], 0
1417a18bc: 7488  je 0x1417a1846
1417a18be: 488b9548030000  mov rdx, qword ptr [rbp + 0x348]
1417a18c5: 48c1e203  shl rdx, 3
1417a18c9: 41b808000000  mov r8d, 8
1417a18cf: 488b8d30030000  mov rcx, qword ptr [rbp + 0x330]
1417a18d6: e84523e7fe  call 0x140613c20
1417a18db: 4883bd0803000000  cmp qword ptr [rbp + 0x308], 0
1417a18e3: 0f8567ffffff  jne 0x1417a1850
1417a18e9: e97bffffff  jmp 0x1417a1869
1417a18ee: 6690  nop
1417a18f0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a18f5: 55  push rbp
1417a18f6: 4157  push r15
1417a18f8: 4156  push r14
1417a18fa: 4155  push r13
1417a18fc: 4154  push r12
1417a18fe: 56  push rsi
1417a18ff: 57  push rdi
1417a1900: 53  push rbx
1417a1901: 4883ec58  sub rsp, 0x58
1417a1905: 488daa80000000  lea rbp, [rdx + 0x80]
1417a190c: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1417a1911: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1417a1916: c6856e03000000  mov byte ptr [rbp + 0x36e], 0
1417a191d: 488d8de0000000  lea rcx, [rbp + 0xe0]
1417a1924: e817e90eff  call 0x140890240
1417a1929: c6856e03000000  mov byte ptr [rbp + 0x36e], 0
1417a1930: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a1935: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a193a: 4883c458  add rsp, 0x58
1417a193e: 5b  pop rbx
1417a193f: 5f  pop rdi
1417a1940: 5e  pop rsi
1417a1941: 415c  pop r12
1417a1943: 415d  pop r13
1417a1945: 415e  pop r14
1417a1947: 415f  pop r15
1417a1949: 5d  pop rbp
1417a194a: c3  ret
1417a194b: 0f1f440000  nop dword ptr [rax + rax]
1417a1950: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a1955: 55  push rbp
1417a1956: 4157  push r15
1417a1958: 4156  push r14
1417a195a: 4155  push r13
1417a195c: 4154  push r12
1417a195e: 56  push rsi
1417a195f: 57  push rdi
1417a1960: 53  push rbx
1417a1961: 4883ec58  sub rsp, 0x58
1417a1965: 488daa80000000  lea rbp, [rdx + 0x80]
1417a196c: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1417a1971: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1417a1976: 488b8558030000  mov rax, qword ptr [rbp + 0x358]
1417a197d: 48ff00  inc qword ptr [rax]
1417a1980: c6857703000000  mov byte ptr [rbp + 0x377], 0
1417a1987: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a198c: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a1991: 4883c458  add rsp, 0x58
1417a1995: 5b  pop rbx
1417a1996: 5f  pop rdi
1417a1997: 5e  pop rsi
1417a1998: 415c  pop r12
1417a199a: 415d  pop r13
1417a199c: 415e  pop r14
1417a199e: 415f  pop r15
1417a19a0: 5d  pop rbp
1417a19a1: c3  ret
1417a19a2: 66666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a19b0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a19b5: 55  push rbp
1417a19b6: 4157  push r15
1417a19b8: 4156  push r14
1417a19ba: 4155  push r13
1417a19bc: 4154  push r12
1417a19be: 56  push rsi
1417a19bf: 57  push rdi
1417a19c0: 53  push rbx
1417a19c1: 4883ec58  sub rsp, 0x58
1417a19c5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a19cc: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1417a19d1: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1417a19d6: 0fb69d6e030000  movzx ebx, byte ptr [rbp + 0x36e]
1417a19dd: f6c301  test bl, 1
1417a19e0: 7444  je 0x1417a1a26
1417a19e2: 488b5500  mov rdx, qword ptr [rbp]
1417a19e6: 4885d2  test rdx, rdx
1417a19e9: 7413  je 0x1417a19fe
1417a19eb: 488b4d08  mov rcx, qword ptr [rbp + 8]
1417a19ef: 48c1e203  shl rdx, 3
1417a19f3: 41b804000000  mov r8d, 4
1417a19f9: e82222e7fe  call 0x140613c20
1417a19fe: 889d76030000  mov byte ptr [rbp + 0x376], bl
1417a1a04: c6857503000000  mov byte ptr [rbp + 0x375], 0
1417a1a0b: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a1a10: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a1a15: 4883c458  add rsp, 0x58
1417a1a19: 5b  pop rbx
1417a1a1a: 5f  pop rdi
1417a1a1b: 5e  pop rsi
1417a1a1c: 415c  pop r12
1417a1a1e: 415d  pop r13
1417a1a20: 415e  pop r14
1417a1a22: 415f  pop r15
1417a1a24: 5d  pop rbp
1417a1a25: c3  ret
1417a1a26: 889d76030000  mov byte ptr [rbp + 0x376], bl
1417a1a2c: c6857503000000  mov byte ptr [rbp + 0x375], 0
1417a1a33: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a1a38: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a1a3d: 4883c458  add rsp, 0x58
1417a1a41: 5b  pop rbx
1417a1a42: 5f  pop rdi
1417a1a43: 5e  pop rsi
1417a1a44: 415c  pop r12
1417a1a46: 415d  pop r13
1417a1a48: 415e  pop r14
1417a1a4a: 415f  pop r15
1417a1a4c: 5d  pop rbp
1417a1a4d: c3  ret
1417a1a4e: 6690  nop
1417a1a50: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a1a55: 55  push rbp
1417a1a56: 4157  push r15
1417a1a58: 4156  push r14
1417a1a5a: 4155  push r13
1417a1a5c: 4154  push r12
1417a1a5e: 56  push rsi
1417a1a5f: 57  push rdi
1417a1a60: 53  push rbx
1417a1a61: 4883ec58  sub rsp, 0x58
1417a1a65: 488daa80000000  lea rbp, [rdx + 0x80]
1417a1a6c: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1417a1a71: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1417a1a76: 0fb69d75030000  movzx ebx, byte ptr [rbp + 0x375]
1417a1a7d: 0fb6b576030000  movzx esi, byte ptr [rbp + 0x376]
1417a1a84: 40f6c601  test sil, 1
1417a1a88: 744a  je 0x1417a1ad4
1417a1a8a: 488b95c0000000  mov rdx, qword ptr [rbp + 0xc0]
1417a1a91: 4885d2  test rdx, rdx
1417a1a94: 7416  je 0x1417a1aac
1417a1a96: 488b8dc8000000  mov rcx, qword ptr [rbp + 0xc8]
1417a1a9d: 48c1e203  shl rdx, 3
1417a1aa1: 41b808000000  mov r8d, 8
1417a1aa7: e87421e7fe  call 0x140613c20
1417a1aac: 4088b570030000  mov byte ptr [rbp + 0x370], sil
1417a1ab3: 889d6f030000  mov byte ptr [rbp + 0x36f], bl
1417a1ab9: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a1abe: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a1ac3: 4883c458  add rsp, 0x58
1417a1ac7: 5b  pop rbx
1417a1ac8: 5f  pop rdi
1417a1ac9: 5e  pop rsi
1417a1aca: 415c  pop r12
1417a1acc: 415d  pop r13
1417a1ace: 415e  pop r14
1417a1ad0: 415f  pop r15
1417a1ad2: 5d  pop rbp
1417a1ad3: c3  ret
1417a1ad4: 4088b570030000  mov byte ptr [rbp + 0x370], sil
1417a1adb: 889d6f030000  mov byte ptr [rbp + 0x36f], bl
1417a1ae1: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a1ae6: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a1aeb: 4883c458  add rsp, 0x58
1417a1aef: 5b  pop rbx
1417a1af0: 5f  pop rdi
1417a1af1: 5e  pop rsi
1417a1af2: 415c  pop r12
1417a1af4: 415d  pop r13
1417a1af6: 415e  pop r14
1417a1af8: 415f  pop r15
1417a1afa: 5d  pop rbp
1417a1afb: c3  ret
1417a1afc: 0f1f4000  nop dword ptr [rax]
1417a1b00: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a1b05: 55  push rbp
1417a1b06: 4157  push r15
1417a1b08: 4156  push r14
1417a1b0a: 4155  push r13
1417a1b0c: 4154  push r12
1417a1b0e: 56  push rsi
1417a1b0f: 57  push rdi
1417a1b10: 53  push rbx
1417a1b11: 4883ec58  sub rsp, 0x58
1417a1b15: 488daa80000000  lea rbp, [rdx + 0x80]
1417a1b1c: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1417a1b21: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1417a1b26: 0fb69d6f030000  movzx ebx, byte ptr [rbp + 0x36f]
1417a1b2d: 0fb6b570030000  movzx esi, byte ptr [rbp + 0x370]
1417a1b34: f6c301  test bl, 1
1417a1b37: 7431  je 0x1417a1b6a
1417a1b39: 488d4dd8  lea rcx, [rbp - 0x28]
1417a1b3d: e80edc0dff  call 0x14087f750
1417a1b42: 4088b572030000  mov byte ptr [rbp + 0x372], sil
1417a1b49: 889d71030000  mov byte ptr [rbp + 0x371], bl
1417a1b4f: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a1b54: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a1b59: 4883c458  add rsp, 0x58
1417a1b5d: 5b  pop rbx
1417a1b5e: 5f  pop rdi
1417a1b5f: 5e  pop rsi
1417a1b60: 415c  pop r12
1417a1b62: 415d  pop r13
1417a1b64: 415e  pop r14
1417a1b66: 415f  pop r15
1417a1b68: 5d  pop rbp
1417a1b69: c3  ret
1417a1b6a: 4088b572030000  mov byte ptr [rbp + 0x372], sil
1417a1b71: 889d71030000  mov byte ptr [rbp + 0x371], bl
1417a1b77: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a1b7c: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a1b81: 4883c458  add rsp, 0x58
1417a1b85: 5b  pop rbx
1417a1b86: 5f  pop rdi
1417a1b87: 5e  pop rsi
1417a1b88: 415c  pop r12
1417a1b8a: 415d  pop r13
1417a1b8c: 415e  pop r14
1417a1b8e: 415f  pop r15
1417a1b90: 5d  pop rbp
1417a1b91: c3  ret
1417a1b92: 66666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a1ba0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a1ba5: 55  push rbp
1417a1ba6: 4157  push r15
1417a1ba8: 4156  push r14
1417a1baa: 4155  push r13
1417a1bac: 4154  push r12
1417a1bae: 56  push rsi
1417a1baf: 57  push rdi
1417a1bb0: 53  push rbx
1417a1bb1: 4883ec58  sub rsp, 0x58
1417a1bb5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a1bbc: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1417a1bc1: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1417a1bc6: 0fb69d71030000  movzx ebx, byte ptr [rbp + 0x371]
1417a1bcd: f6857203000001  test byte ptr [rbp + 0x372], 1
1417a1bd4: 742d  je 0x1417a1c03
1417a1bd6: 488d8d78020000  lea rcx, [rbp + 0x278]
1417a1bdd: e83ed70dff  call 0x14087f320
1417a1be2: 889d73030000  mov byte ptr [rbp + 0x373], bl
1417a1be8: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a1bed: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a1bf2: 4883c458  add rsp, 0x58
1417a1bf6: 5b  pop rbx
1417a1bf7: 5f  pop rdi
1417a1bf8: 5e  pop rsi
1417a1bf9: 415c  pop r12
1417a1bfb: 415d  pop r13
1417a1bfd: 415e  pop r14
1417a1bff: 415f  pop r15
1417a1c01: 5d  pop rbp
1417a1c02: c3  ret
1417a1c03: 889d73030000  mov byte ptr [rbp + 0x373], bl
1417a1c09: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a1c0e: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a1c13: 4883c458  add rsp, 0x58
1417a1c17: 5b  pop rbx
1417a1c18: 5f  pop rdi
1417a1c19: 5e  pop rsi
1417a1c1a: 415c  pop r12
1417a1c1c: 415d  pop r13
1417a1c1e: 415e  pop r14
1417a1c20: 415f  pop r15
1417a1c22: 5d  pop rbp
1417a1c23: c3  ret
1417a1c24: 6666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1417a1c30: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a1c35: 55  push rbp
1417a1c36: 4157  push r15
1417a1c38: 4156  push r14
1417a1c3a: 4155  push r13
1417a1c3c: 4154  push r12
1417a1c3e: 56  push rsi
1417a1c3f: 57  push rdi
1417a1c40: 53  push rbx
1417a1c41: 4883ec58  sub rsp, 0x58
1417a1c45: 488daa80000000  lea rbp, [rdx + 0x80]
1417a1c4c: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1417a1c51: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1417a1c56: f6857703000001  test byte ptr [rbp + 0x377], 1
1417a1c5d: 488b8d40030000  mov rcx, qword ptr [rbp + 0x340]
1417a1c64: 741b  je 0x1417a1c81
1417a1c66: 488b01  mov rax, qword ptr [rcx]
1417a1c69: 4885c0  test rax, rax
1417a1c6c: 7413  je 0x1417a1c81
1417a1c6e: 488b4908  mov rcx, qword ptr [rcx + 8]
1417a1c72: 486bd058  imul rdx, rax, 0x58
1417a1c76: 41b808000000  mov r8d, 8
1417a1c7c: e89f1fe7fe  call 0x140613c20
1417a1c81: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a1c86: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a1c8b: 4883c458  add rsp, 0x58
1417a1c8f: 5b  pop rbx
1417a1c90: 5f  pop rdi
1417a1c91: 5e  pop rsi
1417a1c92: 415c  pop r12
1417a1c94: 415d  pop r13
1417a1c96: 415e  pop r14
1417a1c98: 415f  pop r15
1417a1c9a: 5d  pop rbp
1417a1c9b: c3  ret
1417a1c9c: 0f1f4000  nop dword ptr [rax]
1417a1ca0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417a1ca5: 55  push rbp
1417a1ca6: 4157  push r15
1417a1ca8: 4156  push r14
1417a1caa: 4155  push r13
1417a1cac: 4154  push r12
1417a1cae: 56  push rsi
1417a1caf: 57  push rdi
1417a1cb0: 53  push rbx
1417a1cb1: 4883ec58  sub rsp, 0x58
1417a1cb5: 488daa80000000  lea rbp, [rdx + 0x80]
1417a1cbc: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1417a1cc1: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1417a1cc6: 0fb69d6b030000  movzx ebx, byte ptr [rbp + 0x36b]
1417a1ccd: 488d4d70  lea rcx, [rbp + 0x70]
1417a1cd1: e8eaade4fe  call 0x1405ecac0
1417a1cd6: 889d74030000  mov byte ptr [rbp + 0x374], bl
1417a1cdc: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1417a1ce1: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1417a1ce6: 4883c458  add rsp, 0x58
1417a1cea: 5b  pop rbx
1417a1ceb: 5f  pop rdi
1417a1cec: 5e  pop rsi
1417a1ced: 415c  pop r12
1417a1cef: 415d  pop r13
1417a1cf1: 415e  pop r14
1417a1cf3: 415f  pop r15
1417a1cf5: 5d  pop rbp
1417a1cf6: c3  ret
