1415b90d0: 55  push rbp
1415b90d1: 4157  push r15
1415b90d3: 4156  push r14
1415b90d5: 4155  push r13
1415b90d7: 4154  push r12
1415b90d9: 56  push rsi
1415b90da: 57  push rdi
1415b90db: 53  push rbx
1415b90dc: 4881ec28010000  sub rsp, 0x128
1415b90e3: 488dac2480000000  lea rbp, [rsp + 0x80]
1415b90eb: 0f29bd90000000  movaps xmmword ptr [rbp + 0x90], xmm7
1415b90f2: 0f29b580000000  movaps xmmword ptr [rbp + 0x80], xmm6
1415b90f9: 48c74578feffffff  mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
1415b9101: 4989cc  mov r12, rcx
1415b9104: 488b4210  mov rax, qword ptr [rdx + 0x10]
1415b9108: 48894510  mov qword ptr [rbp + 0x10], rax
1415b910c: 488d5da8  lea rbx, [rbp - 0x58]
1415b9110: 4889d9  mov rcx, rbx
1415b9113: e8b8ef5cff  call 0x140b880d0
1415b9118: 4c8d6d41  lea r13, [rbp + 0x41]
1415b911c: b001  mov al, 1
1415b911e: 894570  mov dword ptr [rbp + 0x70], eax
1415b9121: 4c8d7d18  lea r15, [rbp + 0x18]
1415b9125: 4c8d3580226c01  lea r14, [rip + 0x16c2280]
1415b912c: c7456c00000000  mov dword ptr [rbp + 0x6c], 0
1415b9133: c7457400000000  mov dword ptr [rbp + 0x74], 0
1415b913a: 660f1f440000  nop word ptr [rax + rax]
1415b9140: 4c89f9  mov rcx, r15
1415b9143: 4889da  mov rdx, rbx
1415b9146: e80533e8ff  call 0x14143c450
1415b914b: 488b4518  mov rax, qword ptr [rbp + 0x18]
1415b914f: 4885c0  test rax, rax
1415b9152: 0f8414020000  je 0x1415b936c
1415b9158: 488b4d28  mov rcx, qword ptr [rbp + 0x28]
1415b915c: 488d1449  lea rdx, [rcx + rcx*2]
1415b9160: 4c8b84d068010000  mov r8, qword ptr [rax + rdx*8 + 0x168]
1415b9168: 4c8b8cd070010000  mov r9, qword ptr [rax + rdx*8 + 0x170]
1415b9170: 4c894d30  mov qword ptr [rbp + 0x30], r9
1415b9174: 488bb4d078010000  mov rsi, qword ptr [rax + rdx*8 + 0x178]
1415b917c: 48c1e105  shl rcx, 5
1415b9180: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
1415b9184: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
1415b9189: 0f294540  movaps xmmword ptr [rbp + 0x40], xmm0
1415b918d: 0f294d50  movaps xmmword ptr [rbp + 0x50], xmm1
1415b9191: 4c89c0  mov rax, r8
1415b9194: 48f7d8  neg rax
1415b9197: 0f80cf010000  jo 0x1415b936c
1415b919d: 4c894560  mov qword ptr [rbp + 0x60], r8
1415b91a1: 0fb645f0  movzx eax, byte ptr [rbp - 0x10]
1415b91a5: 4883f806  cmp rax, 6
1415b91a9: 0f87f5000000  ja 0x1415b92a4
1415b91af: 49630486  movsxd rax, dword ptr [r14 + rax*4]
1415b91b3: 4c01f0  add rax, r14
1415b91b6: ffe0  jmp rax
1415b91b8: 0f284540  movaps xmm0, xmmword ptr [rbp + 0x40]
1415b91bc: 0f284d50  movaps xmm1, xmmword ptr [rbp + 0x50]
1415b91c0: 488d45f0  lea rax, [rbp - 0x10]
1415b91c4: 0f114810  movups xmmword ptr [rax + 0x10], xmm1
1415b91c8: 0f1100  movups xmmword ptr [rax], xmm0
1415b91cb: 40b702  mov dil, 2
1415b91ce: 4883fe01  cmp rsi, 1
1415b91d2: 488b4d30  mov rcx, qword ptr [rbp + 0x30]
1415b91d6: 751b  jne 0x1415b91f3
1415b91d8: 0fb601  movzx eax, byte ptr [rcx]
1415b91db: 3c79  cmp al, 0x79
1415b91dd: 410f94c0  sete r8b
1415b91e1: b202  mov dl, 2
1415b91e3: 4428c2  sub dl, r8b
1415b91e6: 3c78  cmp al, 0x78
1415b91e8: 0fb6fa  movzx edi, dl
1415b91eb: b800000000  mov eax, 0
1415b91f0: 0f44f8  cmove edi, eax
1415b91f3: 488b5560  mov rdx, qword ptr [rbp + 0x60]
1415b91f7: 4885d2  test rdx, rdx
1415b91fa: 740b  je 0x1415b9207
1415b91fc: 41b801000000  mov r8d, 1
1415b9202: e819aa05ff  call 0x140613c20
1415b9207: 4080ff02  cmp dil, 2
1415b920b: 0f85a1000000  jne 0x1415b92b2
1415b9211: 0fb645f0  movzx eax, byte ptr [rbp - 0x10]
1415b9215: c645f006  mov byte ptr [rbp - 0x10], 6
1415b9219: 3c06  cmp al, 6
1415b921b: 0f8494010000  je 0x1415b93b5
1415b9221: 884540  mov byte ptr [rbp + 0x40], al
1415b9224: 0f104349  movups xmm0, xmmword ptr [rbx + 0x49]
1415b9228: 0f104b58  movups xmm1, xmmword ptr [rbx + 0x58]
1415b922c: 410f114d0f  movups xmmword ptr [r13 + 0xf], xmm1
1415b9231: 410f114500  movups xmmword ptr [r13], xmm0
1415b9236: 488d4d40  lea rcx, [rbp + 0x40]
1415b923a: e801a7feff  call 0x1415a3940
1415b923f: 4885c0  test rax, rax
1415b9242: 0f84f8feffff  je 0x1415b9140
1415b9248: e98c010000  jmp 0x1415b93d9
1415b924d: 488b55f8  mov rdx, qword ptr [rbp - 8]
1415b9251: 4885d2  test rdx, rdx
1415b9254: 0f845effffff  je 0x1415b91b8
1415b925a: 488b4d00  mov rcx, qword ptr [rbp]
1415b925e: 41b801000000  mov r8d, 1
1415b9264: e8b7a905ff  call 0x140613c20
1415b9269: e94affffff  jmp 0x1415b91b8
1415b926e: 488b4d00  mov rcx, qword ptr [rbp]
1415b9272: 488b5508  mov rdx, qword ptr [rbp + 8]
1415b9276: 48894d38  mov qword ptr [rbp + 0x38], rcx
1415b927a: e8f1fa2cff  call 0x140888d70
1415b927f: 488b55f8  mov rdx, qword ptr [rbp - 8]
1415b9283: 4885d2  test rdx, rdx
1415b9286: 0f842cffffff  je 0x1415b91b8
1415b928c: 48c1e205  shl rdx, 5
1415b9290: 41b808000000  mov r8d, 8
1415b9296: 488b4d38  mov rcx, qword ptr [rbp + 0x38]
1415b929a: e881a905ff  call 0x140613c20
1415b929f: e914ffffff  jmp 0x1415b91b8
1415b92a4: 488d4df8  lea rcx, [rbp - 8]
1415b92a8: e893fc2dff  call 0x140898f40
1415b92ad: e906ffffff  jmp 0x1415b91b8
1415b92b2: 400fb6c7  movzx eax, dil
1415b92b6: 83f801  cmp eax, 1
1415b92b9: 755b  jne 0x1415b9316
1415b92bb: f6457001  test byte ptr [rbp + 0x70], 1
1415b92bf: 0f849a010000  je 0x1415b945f
1415b92c5: 0fb645f0  movzx eax, byte ptr [rbp - 0x10]
1415b92c9: c645f006  mov byte ptr [rbp - 0x10], 6
1415b92cd: 3c06  cmp al, 6
1415b92cf: 0f844e010000  je 0x1415b9423
1415b92d5: 884540  mov byte ptr [rbp + 0x40], al
1415b92d8: 0f104349  movups xmm0, xmmword ptr [rbx + 0x49]
1415b92dc: 0f104b58  movups xmm1, xmmword ptr [rbx + 0x58]
1415b92e0: 410f114d0f  movups xmmword ptr [r13 + 0xf], xmm1
1415b92e5: 410f114500  movups xmmword ptr [r13], xmm0
1415b92ea: 4c89f9  mov rcx, r15
1415b92ed: 488d5540  lea rdx, [rbp + 0x40]
1415b92f1: e87a10feff  call 0x14159a370
1415b92f6: f6451801  test byte ptr [rbp + 0x18], 1
1415b92fa: 0f851d010000  jne 0x1415b941d
1415b9300: f30f10751c  movss xmm6, dword ptr [rbp + 0x1c]
1415b9305: b001  mov al, 1
1415b9307: 89456c  mov dword ptr [rbp + 0x6c], eax
1415b930a: c7457000000000  mov dword ptr [rbp + 0x70], 0
1415b9311: e92afeffff  jmp 0x1415b9140
1415b9316: 837d7400  cmp dword ptr [rbp + 0x74], 0
1415b931a: 0f8529010000  jne 0x1415b9449
1415b9320: 0fb645f0  movzx eax, byte ptr [rbp - 0x10]
1415b9324: c645f006  mov byte ptr [rbp - 0x10], 6
1415b9328: 3c06  cmp al, 6
1415b932a: 0f8406010000  je 0x1415b9436
1415b9330: 884540  mov byte ptr [rbp + 0x40], al
1415b9333: 0f104349  movups xmm0, xmmword ptr [rbx + 0x49]
1415b9337: 0f104b58  movups xmm1, xmmword ptr [rbx + 0x58]
1415b933b: 410f114d0f  movups xmmword ptr [r13 + 0xf], xmm1
1415b9340: 410f114500  movups xmmword ptr [r13], xmm0
1415b9345: 4c89f9  mov rcx, r15
1415b9348: 488d5540  lea rdx, [rbp + 0x40]
1415b934c: e81f10feff  call 0x14159a370
1415b9351: f6451801  test byte ptr [rbp + 0x18], 1
1415b9355: 0f85c2000000  jne 0x1415b941d
1415b935b: f30f107d1c  movss xmm7, dword ptr [rbp + 0x1c]
1415b9360: c7457401000000  mov dword ptr [rbp + 0x74], 1
1415b9367: e9d4fdffff  jmp 0x1415b9140
1415b936c: f6457401  test byte ptr [rbp + 0x74], 1
1415b9370: 7430  je 0x1415b93a2
1415b9372: f6456c01  test byte ptr [rbp + 0x6c], 1
1415b9376: 7450  je 0x1415b93c8
1415b9378: 48837de800  cmp qword ptr [rbp - 0x18], 0
1415b937d: 0f85f2000000  jne 0x1415b9475
1415b9383: f3410f117c2404  movss dword ptr [r12 + 4], xmm7
1415b938a: f3410f11742408  movss dword ptr [r12 + 8], xmm6
1415b9391: 31c0  xor eax, eax
1415b9393: 41890424  mov dword ptr [r12], eax
1415b9397: 488d4da8  lea rcx, [rbp - 0x58]
1415b939b: e850ea2cff  call 0x140887df0
1415b93a0: eb4d  jmp 0x1415b93ef
1415b93a2: 488d0ddf0d6c01  lea rcx, [rip + 0x16c0ddf]
1415b93a9: ba01000000  mov edx, 1
1415b93ae: e80d083401  call 0x1428f9bc0
1415b93b3: eb24  jmp 0x1415b93d9
1415b93b5: 488d0dac0e6c01  lea rcx, [rip + 0x16c0eac]
1415b93bc: ba10000000  mov edx, 0x10
1415b93c1: e83a0c3401  call 0x1428fa000
1415b93c6: eb11  jmp 0x1415b93d9
1415b93c8: 488d0dba0d6c01  lea rcx, [rip + 0x16c0dba]
1415b93cf: ba01000000  mov edx, 1
1415b93d4: e8e7073401  call 0x1428f9bc0
1415b93d9: 4989442408  mov qword ptr [r12 + 8], rax
1415b93de: 41c7042401000000  mov dword ptr [r12], 1
1415b93e6: 488d4da8  lea rcx, [rbp - 0x58]
1415b93ea: e801ea2cff  call 0x140887df0
1415b93ef: 488d4df0  lea rcx, [rbp - 0x10]
1415b93f3: e838502dff  call 0x14088e430
1415b93f8: 4c89e0  mov rax, r12
1415b93fb: 0f28b580000000  movaps xmm6, xmmword ptr [rbp + 0x80]
1415b9402: 0f28bd90000000  movaps xmm7, xmmword ptr [rbp + 0x90]
1415b9409: 4881c428010000  add rsp, 0x128
1415b9410: 5b  pop rbx
1415b9411: 5f  pop rdi
1415b9412: 5e  pop rsi
1415b9413: 415c  pop r12
1415b9415: 415d  pop r13
1415b9417: 415e  pop r14
1415b9419: 415f  pop r15
1415b941b: 5d  pop rbp
1415b941c: c3  ret 
1415b941d: 488b4520  mov rax, qword ptr [rbp + 0x20]
1415b9421: ebb6  jmp 0x1415b93d9
1415b9423: 488d0d3e0e6c01  lea rcx, [rip + 0x16c0e3e]
1415b942a: ba10000000  mov edx, 0x10
1415b942f: e8cc0b3401  call 0x1428fa000
1415b9434: eba3  jmp 0x1415b93d9
1415b9436: 488d0d2b0e6c01  lea rcx, [rip + 0x16c0e2b]
1415b943d: ba10000000  mov edx, 0x10
1415b9442: e8b90b3401  call 0x1428fa000
1415b9447: eb90  jmp 0x1415b93d9
1415b9449: 488d0d380d6c01  lea rcx, [rip + 0x16c0d38]
1415b9450: ba01000000  mov edx, 1
1415b9455: e856093401  call 0x1428f9db0
1415b945a: e97affffff  jmp 0x1415b93d9
1415b945f: 488d0d230d6c01  lea rcx, [rip + 0x16c0d23]
1415b9466: ba01000000  mov edx, 1
1415b946b: e840093401  call 0x1428f9db0
1415b9470: e964ffffff  jmp 0x1415b93d9
1415b9475: 488d151c0c6c01  lea rdx, [rip + 0x16c0c1c]
1415b947c: 4c8d05dd0b6c01  lea r8, [rip + 0x16c0bdd]
1415b9483: 488b4d10  mov rcx, qword ptr [rbp + 0x10]
1415b9487: e894083401  call 0x1428f9d20
1415b948c: 4989442408  mov qword ptr [r12 + 8], rax
1415b9491: b801000000  mov eax, 1
1415b9496: e9f8feffff  jmp 0x1415b9393
1415b949b: 0f1f440000  nop dword ptr [rax + rax]
1415b94a0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b94a5: 55  push rbp
1415b94a6: 4157  push r15
1415b94a8: 4156  push r14
1415b94aa: 4155  push r13
1415b94ac: 4154  push r12
1415b94ae: 56  push rsi
1415b94af: 57  push rdi
1415b94b0: 53  push rbx
1415b94b1: 4883ec48  sub rsp, 0x48
1415b94b5: 488daa80000000  lea rbp, [rdx + 0x80]
1415b94bc: 0f297c2420  movaps xmmword ptr [rsp + 0x20], xmm7
1415b94c1: 0f29742430  movaps xmmword ptr [rsp + 0x30], xmm6
1415b94c6: 488b55f8  mov rdx, qword ptr [rbp - 8]
1415b94ca: 4885d2  test rdx, rdx
1415b94cd: 7413  je 0x1415b94e2
1415b94cf: 48c1e205  shl rdx, 5
1415b94d3: 41b808000000  mov r8d, 8
1415b94d9: 488b4d38  mov rcx, qword ptr [rbp + 0x38]
1415b94dd: e83ea705ff  call 0x140613c20
1415b94e2: 0f28742430  movaps xmm6, xmmword ptr [rsp + 0x30]
1415b94e7: 0f287c2420  movaps xmm7, xmmword ptr [rsp + 0x20]
1415b94ec: 4883c448  add rsp, 0x48
1415b94f0: 5b  pop rbx
1415b94f1: 5f  pop rdi
1415b94f2: 5e  pop rsi
1415b94f3: 415c  pop r12
1415b94f5: 415d  pop r13
1415b94f7: 415e  pop r14
1415b94f9: 415f  pop r15
1415b94fb: 5d  pop rbp
1415b94fc: c3  ret 
1415b94fd: 0f1f00  nop dword ptr [rax]
1415b9500: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b9505: 55  push rbp
1415b9506: 4157  push r15
1415b9508: 4156  push r14
1415b950a: 4155  push r13
1415b950c: 4154  push r12
1415b950e: 56  push rsi
1415b950f: 57  push rdi
1415b9510: 53  push rbx
1415b9511: 4883ec48  sub rsp, 0x48
1415b9515: 488daa80000000  lea rbp, [rdx + 0x80]
1415b951c: 0f297c2420  movaps xmmword ptr [rsp + 0x20], xmm7
1415b9521: 0f29742430  movaps xmmword ptr [rsp + 0x30], xmm6
1415b9526: 0f284540  movaps xmm0, xmmword ptr [rbp + 0x40]
1415b952a: 0f284d50  movaps xmm1, xmmword ptr [rbp + 0x50]
1415b952e: 488d45f0  lea rax, [rbp - 0x10]
1415b9532: 0f114810  movups xmmword ptr [rax + 0x10], xmm1
1415b9536: 0f1100  movups xmmword ptr [rax], xmm0
1415b9539: 48837d6000  cmp qword ptr [rbp + 0x60], 0
1415b953e: 7413  je 0x1415b9553
1415b9540: 41b801000000  mov r8d, 1
1415b9546: 488b4d30  mov rcx, qword ptr [rbp + 0x30]
1415b954a: 488b5560  mov rdx, qword ptr [rbp + 0x60]
1415b954e: e8cda605ff  call 0x140613c20
1415b9553: 0f28742430  movaps xmm6, xmmword ptr [rsp + 0x30]
1415b9558: 0f287c2420  movaps xmm7, xmmword ptr [rsp + 0x20]
1415b955d: 4883c448  add rsp, 0x48
1415b9561: 5b  pop rbx
1415b9562: 5f  pop rdi
1415b9563: 5e  pop rsi
1415b9564: 415c  pop r12
1415b9566: 415d  pop r13
1415b9568: 415e  pop r14
1415b956a: 415f  pop r15
1415b956c: 5d  pop rbp
1415b956d: c3  ret 
1415b956e: 6690  nop 
1415b9570: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b9575: 55  push rbp
1415b9576: 4157  push r15
1415b9578: 4156  push r14
1415b957a: 4155  push r13
1415b957c: 4154  push r12
1415b957e: 56  push rsi
1415b957f: 57  push rdi
1415b9580: 53  push rbx
1415b9581: 4883ec48  sub rsp, 0x48
1415b9585: 488daa80000000  lea rbp, [rdx + 0x80]
1415b958c: 0f297c2420  movaps xmmword ptr [rsp + 0x20], xmm7
1415b9591: 0f29742430  movaps xmmword ptr [rsp + 0x30], xmm6
1415b9596: 488d4da8  lea rcx, [rbp - 0x58]
1415b959a: e8610f2dff  call 0x14088a500
1415b959f: 0f28742430  movaps xmm6, xmmword ptr [rsp + 0x30]
1415b95a4: 0f287c2420  movaps xmm7, xmmword ptr [rsp + 0x20]
1415b95a9: 4883c448  add rsp, 0x48
1415b95ad: 5b  pop rbx
1415b95ae: 5f  pop rdi
1415b95af: 5e  pop rsi
1415b95b0: 415c  pop r12
1415b95b2: 415d  pop r13
1415b95b4: 415e  pop r14
1415b95b6: 415f  pop r15
1415b95b8: 5d  pop rbp
1415b95b9: c3  ret 
1415b95ba: 660f1f440000  nop word ptr [rax + rax]
1415b95c0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b95c5: 55  push rbp
1415b95c6: 4157  push r15
1415b95c8: 4156  push r14
1415b95ca: 4155  push r13
1415b95cc: 4154  push r12
1415b95ce: 56  push rsi
1415b95cf: 57  push rdi
1415b95d0: 53  push rbx
1415b95d1: 4883ec48  sub rsp, 0x48
1415b95d5: 488daa80000000  lea rbp, [rdx + 0x80]
1415b95dc: 0f297c2420  movaps xmmword ptr [rsp + 0x20], xmm7
1415b95e1: 0f29742430  movaps xmmword ptr [rsp + 0x30], xmm6
1415b95e6: 488d4df0  lea rcx, [rbp - 0x10]
1415b95ea: e8414e2dff  call 0x14088e430
1415b95ef: 0f28742430  movaps xmm6, xmmword ptr [rsp + 0x30]
1415b95f4: 0f287c2420  movaps xmm7, xmmword ptr [rsp + 0x20]
1415b95f9: 4883c448  add rsp, 0x48
1415b95fd: 5b  pop rbx
1415b95fe: 5f  pop rdi
1415b95ff: 5e  pop rsi
1415b9600: 415c  pop r12
1415b9602: 415d  pop r13
1415b9604: 415e  pop r14
1415b9606: 415f  pop r15
1415b9608: 5d  pop rbp
1415b9609: c3  ret 
1415b960a: 660f1f440000  nop word ptr [rax + rax]
1415b9610: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b9615: 55  push rbp
1415b9616: 4157  push r15
1415b9618: 4156  push r14
1415b961a: 4155  push r13
1415b961c: 4154  push r12
1415b961e: 56  push rsi
1415b961f: 57  push rdi
1415b9620: 53  push rbx
1415b9621: 4883ec48  sub rsp, 0x48
1415b9625: 488daa80000000  lea rbp, [rdx + 0x80]
1415b962c: 0f297c2420  movaps xmmword ptr [rsp + 0x20], xmm7
1415b9631: 0f29742430  movaps xmmword ptr [rsp + 0x30], xmm6
1415b9636: 488d4df0  lea rcx, [rbp - 0x10]
1415b963a: e8f14d2dff  call 0x14088e430
1415b963f: 0f28742430  movaps xmm6, xmmword ptr [rsp + 0x30]
1415b9644: 0f287c2420  movaps xmm7, xmmword ptr [rsp + 0x20]
1415b9649: 4883c448  add rsp, 0x48
1415b964d: 5b  pop rbx
1415b964e: 5f  pop rdi
1415b964f: 5e  pop rsi
1415b9650: 415c  pop r12
1415b9652: 415d  pop r13
1415b9654: 415e  pop r14
1415b9656: 415f  pop r15
1415b9658: 5d  pop rbp
1415b9659: c3  ret 
