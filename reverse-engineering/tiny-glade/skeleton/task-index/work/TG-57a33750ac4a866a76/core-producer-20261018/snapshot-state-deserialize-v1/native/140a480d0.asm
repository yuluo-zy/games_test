140a480d0: 55  push rbp
140a480d1: 4157  push r15
140a480d3: 4156  push r14
140a480d5: 4154  push r12
140a480d7: 56  push rsi
140a480d8: 57  push rdi
140a480d9: 53  push rbx
140a480da: 4881ec40010000  sub rsp, 0x140
140a480e1: 488dac2480000000  lea rbp, [rsp + 0x80]
140a480e9: 48c785b8000000feffffff  mov qword ptr [rbp + 0xb8], 0xfffffffffffffffe
140a480f4: 4889ce  mov rsi, rcx
140a480f7: 0fb602  movzx eax, byte ptr [rdx]
140a480fa: 83f803  cmp eax, 3
140a480fd: 8985a0000000  mov dword ptr [rbp + 0xa0], eax
140a48103: 48895578  mov qword ptr [rbp + 0x78], rdx
140a48107: 0f84ed000000  je 0x140a481fa
140a4810d: 83f805  cmp eax, 5
140a48110: 0f859a010000  jne 0x140a482b0
140a48116: 4c8b7a08  mov r15, qword ptr [rdx + 8]
140a4811a: 4c8b6210  mov r12, qword ptr [rdx + 0x10]
140a4811e: 488b7a18  mov rdi, qword ptr [rdx + 0x18]
140a48122: 31c0  xor eax, eax
140a48124: 4d85ff  test r15, r15
140a48127: 0f95c0  setne al
140a4812a: 4889f9  mov rcx, rdi
140a4812d: 490f44cf  cmove rcx, r15
140a48131: 48894520  mov qword ptr [rbp + 0x20], rax
140a48135: 48c7452800000000  mov qword ptr [rbp + 0x28], 0
140a4813d: 4c897d30  mov qword ptr [rbp + 0x30], r15
140a48141: 4c896538  mov qword ptr [rbp + 0x38], r12
140a48145: 48894540  mov qword ptr [rbp + 0x40], rax
140a48149: 48c7454800000000  mov qword ptr [rbp + 0x48], 0
140a48151: 4c897d50  mov qword ptr [rbp + 0x50], r15
140a48155: 4c896558  mov qword ptr [rbp + 0x58], r12
140a48159: 48894d60  mov qword ptr [rbp + 0x60], rcx
140a4815d: 488d8d80000000  lea rcx, [rbp + 0x80]
140a48164: 488d5520  lea rdx, [rbp + 0x20]
140a48168: e8e3429f00  call 0x14143c450
140a4816d: 488b8580000000  mov rax, qword ptr [rbp + 0x80]
140a48174: 4885c0  test rax, rax
140a48177: 7444  je 0x140a481bd
140a48179: 488b8d90000000  mov rcx, qword ptr [rbp + 0x90]
140a48180: 488d1449  lea rdx, [rcx + rcx*2]
140a48184: 4c8b84d068010000  mov r8, qword ptr [rax + rdx*8 + 0x168]
140a4818c: 4c8b8cd070010000  mov r9, qword ptr [rax + rdx*8 + 0x170]
140a48194: 488b9cd078010000  mov rbx, qword ptr [rax + rdx*8 + 0x178]
140a4819c: 48c1e105  shl rcx, 5
140a481a0: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
140a481a4: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
140a481a9: 0f2945e0  movaps xmmword ptr [rbp - 0x20], xmm0
140a481ad: 0f294df0  movaps xmmword ptr [rbp - 0x10], xmm1
140a481b1: 4c89c0  mov rax, r8
140a481b4: 48f7d8  neg rax
140a481b7: 0f81d4010000  jno 0x140a48391
140a481bd: c685800000000b  mov byte ptr [rbp + 0x80], 0xb
140a481c4: 488d157d620802  lea rdx, [rip + 0x208627d]
140a481cb: 4c8d0586620802  lea r8, [rip + 0x2086286]
140a481d2: 488d8d80000000  lea rcx, [rbp + 0x80]
140a481d9: e86219eb01  call 0x1428f9b40
140a481de: 48894608  mov qword ptr [rsi + 8], rax
140a481e2: c60601  mov byte ptr [rsi], 1
140a481e5: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a481ec: 488d4d20  lea rcx, [rbp + 0x20]
140a481f0: e8fbfbe3ff  call 0x140887df0
140a481f5: e976020000  jmp 0x140a48470
140a481fa: 4c8b7a08  mov r15, qword ptr [rdx + 8]
140a481fe: 4c8b6210  mov r12, qword ptr [rdx + 0x10]
140a48202: 488b7a18  mov rdi, qword ptr [rdx + 0x18]
140a48206: 41b606  mov r14b, 6
140a48209: 4c89f8  mov rax, r15
140a4820c: 4c89e1  mov rcx, r12
140a4820f: 4889fb  mov rbx, rdi
140a48212: 48894520  mov qword ptr [rbp + 0x20], rax
140a48216: 48894d28  mov qword ptr [rbp + 0x28], rcx
140a4821a: 48895d30  mov qword ptr [rbp + 0x30], rbx
140a4821e: 0f2845c0  movaps xmm0, xmmword ptr [rbp - 0x40]
140a48222: 0f114539  movups xmmword ptr [rbp + 0x39], xmm0
140a48226: 0f1045cf  movups xmm0, xmmword ptr [rbp - 0x31]
140a4822a: 0f114548  movups xmmword ptr [rbp + 0x48], xmm0
140a4822e: 44887538  mov byte ptr [rbp + 0x38], r14b
140a48232: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a48239: 488d4de0  lea rcx, [rbp - 0x20]
140a4823d: 488d5520  lea rdx, [rbp + 0x20]
140a48241: e84aa3b800  call 0x1415d2590
140a48246: 0fb64de8  movzx ecx, byte ptr [rbp - 0x18]
140a4824a: 488b45e0  mov rax, qword ptr [rbp - 0x20]
140a4824e: 80f907  cmp cl, 7
140a48251: 0f85e6000000  jne 0x140a4833d
140a48257: 48894608  mov qword ptr [rsi + 8], rax
140a4825b: b001  mov al, 1
140a4825d: 8806  mov byte ptr [rsi], al
140a4825f: 8b85a0000000  mov eax, dword ptr [rbp + 0xa0]
140a48265: 83f803  cmp eax, 3
140a48268: 0f8402020000  je 0x140a48470
140a4826e: 83f805  cmp eax, 5
140a48271: 0f84f9010000  je 0x140a48470
140a48277: 4c89bda0000000  mov qword ptr [rbp + 0xa0], r15
140a4827e: 4c896578  mov qword ptr [rbp + 0x78], r12
140a48282: 4c89e1  mov rcx, r12
140a48285: 4889fa  mov rdx, rdi
140a48288: e8e30ae4ff  call 0x140888d70
140a4828d: 488b95a0000000  mov rdx, qword ptr [rbp + 0xa0]
140a48294: 4885d2  test rdx, rdx
140a48297: 488b4d78  mov rcx, qword ptr [rbp + 0x78]
140a4829b: 0f84cf010000  je 0x140a48470
140a482a1: 48c1e205  shl rdx, 5
140a482a5: 41b808000000  mov r8d, 8
140a482ab: e9a9010000  jmp 0x140a48459
140a482b0: 0f1002  movups xmm0, xmmword ptr [rdx]
140a482b3: 0f104a10  movups xmm1, xmmword ptr [rdx + 0x10]
140a482b7: 0f294d30  movaps xmmword ptr [rbp + 0x30], xmm1
140a482bb: 0f294520  movaps xmmword ptr [rbp + 0x20], xmm0
140a482bf: 488d4de0  lea rcx, [rbp - 0x20]
140a482c3: 488d5520  lea rdx, [rbp + 0x20]
140a482c7: e84419e801  call 0x1428c9c10
140a482cc: 488d15b5610802  lea rdx, [rip + 0x20861b5]
140a482d3: 4c8d057e610802  lea r8, [rip + 0x208617e]
140a482da: 488d4de0  lea rcx, [rbp - 0x20]
140a482de: e81d17e801  call 0x1428c9a00
140a482e3: 48894608  mov qword ptr [rsi + 8], rax
140a482e7: c60601  mov byte ptr [rsi], 1
140a482ea: 0fb64520  movzx eax, byte ptr [rbp + 0x20]
140a482ee: 83f803  cmp eax, 3
140a482f1: 0f8279010000  jb 0x140a48470
140a482f7: 0f8449010000  je 0x140a48446
140a482fd: 83f804  cmp eax, 4
140a48300: 0f855a010000  jne 0x140a48460
140a48306: 488b4d30  mov rcx, qword ptr [rbp + 0x30]
140a4830a: 488b5538  mov rdx, qword ptr [rbp + 0x38]
140a4830e: 48898da8000000  mov qword ptr [rbp + 0xa8], rcx
140a48315: e8560ae4ff  call 0x140888d70
140a4831a: 488b5528  mov rdx, qword ptr [rbp + 0x28]
140a4831e: 4885d2  test rdx, rdx
140a48321: 0f8449010000  je 0x140a48470
140a48327: 48c1e205  shl rdx, 5
140a4832b: 41b808000000  mov r8d, 8
140a48331: 488b8da8000000  mov rcx, qword ptr [rbp + 0xa8]
140a48338: e91c010000  jmp 0x140a48459
140a4833d: 0f1045e9  movups xmm0, xmmword ptr [rbp - 0x17]
140a48341: 0f104df8  movups xmm1, xmmword ptr [rbp - 8]
140a48345: 0f118d90000000  movups xmmword ptr [rbp + 0x90], xmm1
140a4834c: 0f118581000000  movups xmmword ptr [rbp + 0x81], xmm0
140a48353: 888d80000000  mov byte ptr [rbp + 0x80], cl
140a48359: 0fb6c0  movzx eax, al
140a4835c: 488d0d45740802  lea rcx, [rip + 0x2087445]
140a48363: 48630481  movsxd rax, dword ptr [rcx + rax*4]
140a48367: 4801c8  add rax, rcx
140a4836a: ffe0  jmp rax
140a4836c: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a48373: 488d8d80000000  lea rcx, [rbp + 0x80]
140a4837a: e881fc1300  call 0x140b88000
140a4837f: 4885c0  test rax, rax
140a48382: 0f85cffeffff  jne 0x140a48257
140a48388: c6460138  mov byte ptr [rsi + 1], 0x38
140a4838c: e9ff010000  jmp 0x140a48590
140a48391: 4c894d70  mov qword ptr [rbp + 0x70], r9
140a48395: 4c8985a8000000  mov qword ptr [rbp + 0xa8], r8
140a4839c: 0f2845e0  movaps xmm0, xmmword ptr [rbp - 0x20]
140a483a0: 0f284df0  movaps xmm1, xmmword ptr [rbp - 0x10]
140a483a4: 0f298d90000000  movaps xmmword ptr [rbp + 0x90], xmm1
140a483ab: 0f298580000000  movaps xmmword ptr [rbp + 0x80], xmm0
140a483b2: 488d4da8  lea rcx, [rbp - 0x58]
140a483b6: 488d5520  lea rdx, [rbp + 0x20]
140a483ba: e891409f00  call 0x14143c450
140a483bf: 488b45a8  mov rax, qword ptr [rbp - 0x58]
140a483c3: 4885c0  test rax, rax
140a483c6: 7441  je 0x140a48409
140a483c8: 488b4db8  mov rcx, qword ptr [rbp - 0x48]
140a483cc: 488d1449  lea rdx, [rcx + rcx*2]
140a483d0: 4c8b84d078010000  mov r8, qword ptr [rax + rdx*8 + 0x178]
140a483d8: 4c8945f0  mov qword ptr [rbp - 0x10], r8
140a483dc: 0f1084d068010000  movups xmm0, xmmword ptr [rax + rdx*8 + 0x168]
140a483e4: 0f2945e0  movaps xmmword ptr [rbp - 0x20], xmm0
140a483e8: 48c1e105  shl rcx, 5
140a483ec: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
140a483f0: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
140a483f5: 0f1145f8  movups xmmword ptr [rbp - 8], xmm0
140a483f9: 0f114d08  movups xmmword ptr [rbp + 8], xmm1
140a483fd: 488b55e0  mov rdx, qword ptr [rbp - 0x20]
140a48401: 4889d0  mov rax, rdx
140a48404: 48f7d8  neg rax
140a48407: 717c  jno 0x140a48485
140a48409: 448ab580000000  mov r14b, byte ptr [rbp + 0x80]
140a48410: 0f108581000000  movups xmm0, xmmword ptr [rbp + 0x81]
140a48417: 0f2945c0  movaps xmmword ptr [rbp - 0x40], xmm0
140a4841b: 0f108590000000  movups xmm0, xmmword ptr [rbp + 0x90]
140a48422: 0f1145cf  movups xmmword ptr [rbp - 0x31], xmm0
140a48426: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a4842d: 488d4d20  lea rcx, [rbp + 0x20]
140a48431: e8baf9e3ff  call 0x140887df0
140a48436: 488b85a8000000  mov rax, qword ptr [rbp + 0xa8]
140a4843d: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
140a48441: e9ccfdffff  jmp 0x140a48212
140a48446: 488b5528  mov rdx, qword ptr [rbp + 0x28]
140a4844a: 4885d2  test rdx, rdx
140a4844d: 7421  je 0x140a48470
140a4844f: 488b4d30  mov rcx, qword ptr [rbp + 0x30]
140a48453: 41b801000000  mov r8d, 1
140a48459: e8c2b7bcff  call 0x140613c20
140a4845e: eb10  jmp 0x140a48470
140a48460: 488d4d28  lea rcx, [rbp + 0x28]
140a48464: c685b700000000  mov byte ptr [rbp + 0xb7], 0
140a4846b: e8d00ae5ff  call 0x140898f40
140a48470: 4889f0  mov rax, rsi
140a48473: 4881c440010000  add rsp, 0x140
140a4847a: 5b  pop rbx
140a4847b: 5f  pop rdi
140a4847c: 5e  pop rsi
140a4847d: 415c  pop r12
140a4847f: 415e  pop r14
140a48481: 415f  pop r15
140a48483: 5d  pop rbp
140a48484: c3  ret 
140a48485: 730f  jae 0x140a48496
140a48487: 488b4de8  mov rcx, qword ptr [rbp - 0x18]
140a4848b: 41b801000000  mov r8d, 1
140a48491: e88ab7bcff  call 0x140613c20
140a48496: 0fb645f8  movzx eax, byte ptr [rbp - 8]
140a4849a: 83f803  cmp eax, 3
140a4849d: 0f823b010000  jb 0x140a485de
140a484a3: 0f8416010000  je 0x140a485bf
140a484a9: 488d4d00  lea rcx, [rbp]
140a484ad: 83f804  cmp eax, 4
140a484b0: 0f8523010000  jne 0x140a485d9
140a484b6: e8d50ee4ff  call 0x140889390
140a484bb: e91e010000  jmp 0x140a485de
140a484c0: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a484c7: 488d8d80000000  lea rcx, [rbp + 0x80]
140a484ce: e82dfb1300  call 0x140b88000
140a484d3: 4885c0  test rax, rax
140a484d6: 0f857bfdffff  jne 0x140a48257
140a484dc: c646013c  mov byte ptr [rsi + 1], 0x3c
140a484e0: e9ab000000  jmp 0x140a48590
140a484e5: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a484ec: 488d8d80000000  lea rcx, [rbp + 0x80]
140a484f3: e808fb1300  call 0x140b88000
140a484f8: 4885c0  test rax, rax
140a484fb: 0f8556fdffff  jne 0x140a48257
140a48501: c646013a  mov byte ptr [rsi + 1], 0x3a
140a48505: e986000000  jmp 0x140a48590
140a4850a: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a48511: 488d8d80000000  lea rcx, [rbp + 0x80]
140a48518: e8e3fa1300  call 0x140b88000
140a4851d: 4885c0  test rax, rax
140a48520: 0f8531fdffff  jne 0x140a48257
140a48526: c646013b  mov byte ptr [rsi + 1], 0x3b
140a4852a: eb64  jmp 0x140a48590
140a4852c: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a48533: 488d8d80000000  lea rcx, [rbp + 0x80]
140a4853a: e8c1fa1300  call 0x140b88000
140a4853f: 4885c0  test rax, rax
140a48542: 0f850ffdffff  jne 0x140a48257
140a48548: c646013f  mov byte ptr [rsi + 1], 0x3f
140a4854c: eb42  jmp 0x140a48590
140a4854e: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a48555: 488d8d80000000  lea rcx, [rbp + 0x80]
140a4855c: e89ffa1300  call 0x140b88000
140a48561: 4885c0  test rax, rax
140a48564: 0f85edfcffff  jne 0x140a48257
140a4856a: c6460139  mov byte ptr [rsi + 1], 0x39
140a4856e: eb20  jmp 0x140a48590
140a48570: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a48577: 488d8d80000000  lea rcx, [rbp + 0x80]
140a4857e: e87dfa1300  call 0x140b88000
140a48583: 4885c0  test rax, rax
140a48586: 0f85cbfcffff  jne 0x140a48257
140a4858c: c646013d  mov byte ptr [rsi + 1], 0x3d
140a48590: 31c0  xor eax, eax
140a48592: e9c6fcffff  jmp 0x140a4825d
140a48597: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a4859e: 4c8d057b680802  lea r8, [rip + 0x208687b]
140a485a5: 488d9580000000  lea rdx, [rbp + 0x80]
140a485ac: 41b901000000  mov r9d, 1
140a485b2: 4889f1  mov rcx, rsi
140a485b5: e8d663b900  call 0x1415de990
140a485ba: e9a0fcffff  jmp 0x140a4825f
140a485bf: 488b5500  mov rdx, qword ptr [rbp]
140a485c3: 4885d2  test rdx, rdx
140a485c6: 7416  je 0x140a485de
140a485c8: 488b4d08  mov rcx, qword ptr [rbp + 8]
140a485cc: 41b801000000  mov r8d, 1
140a485d2: e849b6bcff  call 0x140613c20
140a485d7: eb05  jmp 0x140a485de
140a485d9: e86209e5ff  call 0x140898f40
140a485de: c645e00b  mov byte ptr [rbp - 0x20], 0xb
140a485e2: 488d155f5e0802  lea rdx, [rip + 0x2085e5f]
140a485e9: 4c8d05685e0802  lea r8, [rip + 0x2085e68]
140a485f0: 488d4de0  lea rcx, [rbp - 0x20]
140a485f4: e84715eb01  call 0x1428f9b40
140a485f9: 48894608  mov qword ptr [rsi + 8], rax
140a485fd: c60601  mov byte ptr [rsi], 1
140a48600: 0fb68580000000  movzx eax, byte ptr [rbp + 0x80]
140a48607: 83f803  cmp eax, 3
140a4860a: 723a  jb 0x140a48646
140a4860c: 7413  je 0x140a48621
140a4860e: 488d8d88000000  lea rcx, [rbp + 0x88]
140a48615: 83f804  cmp eax, 4
140a48618: 7527  jne 0x140a48641
140a4861a: e8710de4ff  call 0x140889390
140a4861f: eb25  jmp 0x140a48646
140a48621: 488b9588000000  mov rdx, qword ptr [rbp + 0x88]
140a48628: 4885d2  test rdx, rdx
140a4862b: 7419  je 0x140a48646
140a4862d: 488b8d90000000  mov rcx, qword ptr [rbp + 0x90]
140a48634: 41b801000000  mov r8d, 1
140a4863a: e8e1b5bcff  call 0x140613c20
140a4863f: eb05  jmp 0x140a48646
140a48641: e8fa08e5ff  call 0x140898f40
140a48646: 488b95a8000000  mov rdx, qword ptr [rbp + 0xa8]
140a4864d: 4885d2  test rdx, rdx
140a48650: 0f848ffbffff  je 0x140a481e5
140a48656: 41b801000000  mov r8d, 1
140a4865c: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
140a48660: e8bbb5bcff  call 0x140613c20
140a48665: e97bfbffff  jmp 0x140a481e5
140a4866a: 660f1f440000  nop word ptr [rax + rax]
140a48670: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a48675: 55  push rbp
140a48676: 4157  push r15
140a48678: 4156  push r14
140a4867a: 4154  push r12
140a4867c: 56  push rsi
140a4867d: 57  push rdi
140a4867e: 53  push rbx
140a4867f: 4883ec20  sub rsp, 0x20
140a48683: 488daa80000000  lea rbp, [rdx + 0x80]
140a4868a: 488d4d20  lea rcx, [rbp + 0x20]
140a4868e: e85df7e3ff  call 0x140887df0
140a48693: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a4869a: 4883c420  add rsp, 0x20
140a4869e: 5b  pop rbx
140a4869f: 5f  pop rdi
140a486a0: 5e  pop rsi
140a486a1: 415c  pop r12
140a486a3: 415e  pop r14
140a486a5: 415f  pop r15
140a486a7: 5d  pop rbp
140a486a8: c3  ret 
140a486a9: 0f1f8000000000  nop dword ptr [rax]
140a486b0: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a486b5: 55  push rbp
140a486b6: 4157  push r15
140a486b8: 4156  push r14
140a486ba: 4154  push r12
140a486bc: 56  push rsi
140a486bd: 57  push rdi
140a486be: 53  push rbx
140a486bf: 4883ec20  sub rsp, 0x20
140a486c3: 488daa80000000  lea rbp, [rdx + 0x80]
140a486ca: 4883bda800000000  cmp qword ptr [rbp + 0xa8], 0
140a486d2: 7416  je 0x140a486ea
140a486d4: 41b801000000  mov r8d, 1
140a486da: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
140a486de: 488b95a8000000  mov rdx, qword ptr [rbp + 0xa8]
140a486e5: e836b5bcff  call 0x140613c20
140a486ea: 90  nop 
140a486eb: 4883c420  add rsp, 0x20
140a486ef: 5b  pop rbx
140a486f0: 5f  pop rdi
140a486f1: 5e  pop rsi
140a486f2: 415c  pop r12
140a486f4: 415e  pop r14
140a486f6: 415f  pop r15
140a486f8: 5d  pop rbp
140a486f9: c3  ret 
140a486fa: 660f1f440000  nop word ptr [rax + rax]
140a48700: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a48705: 55  push rbp
140a48706: 4157  push r15
140a48708: 4156  push r14
140a4870a: 4154  push r12
140a4870c: 56  push rsi
140a4870d: 57  push rdi
140a4870e: 53  push rbx
140a4870f: 4883ec20  sub rsp, 0x20
140a48713: 488daa80000000  lea rbp, [rdx + 0x80]
140a4871a: 4883bda000000000  cmp qword ptr [rbp + 0xa0], 0
140a48722: 741a  je 0x140a4873e
140a48724: 488b95a0000000  mov rdx, qword ptr [rbp + 0xa0]
140a4872b: 48c1e205  shl rdx, 5
140a4872f: 41b808000000  mov r8d, 8
140a48735: 488b4d78  mov rcx, qword ptr [rbp + 0x78]
140a48739: e8e2b4bcff  call 0x140613c20
140a4873e: 90  nop 
140a4873f: 4883c420  add rsp, 0x20
140a48743: 5b  pop rbx
140a48744: 5f  pop rdi
140a48745: 5e  pop rsi
140a48746: 415c  pop r12
140a48748: 415e  pop r14
140a4874a: 415f  pop r15
140a4874c: 5d  pop rbp
140a4874d: c3  ret 
140a4874e: 6690  nop 
140a48750: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a48755: 55  push rbp
140a48756: 4157  push r15
140a48758: 4156  push r14
140a4875a: 4154  push r12
140a4875c: 56  push rsi
140a4875d: 57  push rdi
140a4875e: 53  push rbx
140a4875f: 4883ec20  sub rsp, 0x20
140a48763: 488daa80000000  lea rbp, [rdx + 0x80]
140a4876a: 488d4d20  lea rcx, [rbp + 0x20]
140a4876e: e81d53e1ff  call 0x14085da90
140a48773: c685b700000000  mov byte ptr [rbp + 0xb7], 0
140a4877a: 4883c420  add rsp, 0x20
140a4877e: 5b  pop rbx
140a4877f: 5f  pop rdi
140a48780: 5e  pop rsi
140a48781: 415c  pop r12
140a48783: 415e  pop r14
140a48785: 415f  pop r15
140a48787: 5d  pop rbp
140a48788: c3  ret 
140a48789: 0f1f8000000000  nop dword ptr [rax]
140a48790: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a48795: 55  push rbp
140a48796: 4157  push r15
140a48798: 4156  push r14
140a4879a: 4154  push r12
140a4879c: 56  push rsi
140a4879d: 57  push rdi
140a4879e: 53  push rbx
140a4879f: 4883ec20  sub rsp, 0x20
140a487a3: 488daa80000000  lea rbp, [rdx + 0x80]
140a487aa: 488b5528  mov rdx, qword ptr [rbp + 0x28]
140a487ae: 4885d2  test rdx, rdx
140a487b1: 7416  je 0x140a487c9
140a487b3: 48c1e205  shl rdx, 5
140a487b7: 41b808000000  mov r8d, 8
140a487bd: 488b8da8000000  mov rcx, qword ptr [rbp + 0xa8]
140a487c4: e857b4bcff  call 0x140613c20
140a487c9: c685b700000000  mov byte ptr [rbp + 0xb7], 0
140a487d0: 4883c420  add rsp, 0x20
140a487d4: 5b  pop rbx
140a487d5: 5f  pop rdi
140a487d6: 5e  pop rsi
140a487d7: 415c  pop r12
140a487d9: 415e  pop r14
140a487db: 415f  pop r15
140a487dd: 5d  pop rbp
140a487de: c3  ret 
140a487df: 90  nop 
140a487e0: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a487e5: 55  push rbp
140a487e6: 4157  push r15
140a487e8: 4156  push r14
140a487ea: 4154  push r12
140a487ec: 56  push rsi
140a487ed: 57  push rdi
140a487ee: 53  push rbx
140a487ef: 4883ec20  sub rsp, 0x20
140a487f3: 488daa80000000  lea rbp, [rdx + 0x80]
140a487fa: 80bdb700000000  cmp byte ptr [rbp + 0xb7], 0
140a48801: 7415  je 0x140a48818
140a48803: 80bda000000003  cmp byte ptr [rbp + 0xa0], 3
140a4880a: 740c  je 0x140a48818
140a4880c: 0fb685a0000000  movzx eax, byte ptr [rbp + 0xa0]
140a48813: 83f805  cmp eax, 5
140a48816: 750f  jne 0x140a48827
140a48818: 4883c420  add rsp, 0x20
140a4881c: 5b  pop rbx
140a4881d: 5f  pop rdi
140a4881e: 5e  pop rsi
140a4881f: 415c  pop r12
140a48821: 415e  pop r14
140a48823: 415f  pop r15
140a48825: 5d  pop rbp
140a48826: c3  ret 
140a48827: 488b4d78  mov rcx, qword ptr [rbp + 0x78]
140a4882b: e86052e1ff  call 0x14085da90
140a48830: ebe6  jmp 0x140a48818
140a48832: 66666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
140a48840: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a48845: 55  push rbp
140a48846: 4157  push r15
140a48848: 4156  push r14
140a4884a: 4154  push r12
140a4884c: 56  push rsi
140a4884d: 57  push rdi
140a4884e: 53  push rbx
140a4884f: 4883ec20  sub rsp, 0x20
140a48853: 488daa80000000  lea rbp, [rdx + 0x80]
140a4885a: 488d8d80000000  lea rcx, [rbp + 0x80]
140a48861: e82a52e1ff  call 0x14085da90
140a48866: 90  nop 
140a48867: 4883c420  add rsp, 0x20
140a4886b: 5b  pop rbx
140a4886c: 5f  pop rdi
140a4886d: 5e  pop rsi
140a4886e: 415c  pop r12
140a48870: 415e  pop r14
140a48872: 415f  pop r15
140a48874: 5d  pop rbp
140a48875: c3  ret 
