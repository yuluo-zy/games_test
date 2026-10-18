141737510: 55  push rbp
141737511: 4157  push r15
141737513: 4156  push r14
141737515: 4155  push r13
141737517: 4154  push r12
141737519: 56  push rsi
14173751a: 57  push rdi
14173751b: 53  push rbx
14173751c: 4883ec58  sub rsp, 0x58
141737520: 488d6c2450  lea rbp, [rsp + 0x50]
141737525: 48c74500feffffff  mov qword ptr [rbp], 0xfffffffffffffffe
14173752d: 4889d7  mov rdi, rdx
141737530: 48894de0  mov qword ptr [rbp - 0x20], rcx
141737534: 488b0a  mov rcx, qword ptr [rdx]
141737537: 4c8b7208  mov r14, qword ptr [rdx + 8]
14173753b: 488b4218  mov rax, qword ptr [rdx + 0x18]
14173753f: 4889c2  mov rdx, rax
141737542: 4c29f2  sub rdx, r14
141737545: 48c1ea04  shr rdx, 4
141737549: 488955e8  mov qword ptr [rbp - 0x18], rdx
14173754d: 488b5710  mov rdx, qword ptr [rdi + 0x10]
141737551: 488955f0  mov qword ptr [rbp - 0x10], rdx
141737555: 48894df8  mov qword ptr [rbp - 8], rcx
141737559: 48894dd0  mov qword ptr [rbp - 0x30], rcx
14173755d: 4c39f0  cmp rax, r14
141737560: 0f849c000000  je 0x141737602
141737566: 4d8b26  mov r12, qword ptr [r14]
141737569: 458b7e08  mov r15d, dword ptr [r14 + 8]
14173756d: 0fb60595d6ef01  movzx eax, byte ptr [rip + 0x1efd695]
141737574: b918000000  mov ecx, 0x18
141737579: ba08000000  mov edx, 8
14173757e: e88dc6edfe  call 0x140613c10
141737583: 4c8b6df8  mov r13, qword ptr [rbp - 8]
141737587: 4885c0  test rax, rax
14173758a: 7461  je 0x1417375ed
14173758c: 4983c618  add r14, 0x18
141737590: 4c8b6df8  mov r13, qword ptr [rbp - 8]
141737594: 488b5de8  mov rbx, qword ptr [rbp - 0x18]
141737598: 0f1f840000000000  nop dword ptr [rax + rax]
1417375a0: 48c70001000000  mov qword ptr [rax], 1
1417375a7: 48c7400801000000  mov qword ptr [rax + 8], 1
1417375af: 4c896010  mov qword ptr [rax + 0x10], r12
1417375b3: 49894500  mov qword ptr [r13], rax
1417375b7: 45897d08  mov dword ptr [r13 + 8], r15d
1417375bb: 48ffcb  dec rbx
1417375be: 7442  je 0x141737602
1417375c0: 498d7610  lea rsi, [r14 + 0x10]
1417375c4: 4983c510  add r13, 0x10
1417375c8: 4d8b66f8  mov r12, qword ptr [r14 - 8]
1417375cc: 458b3e  mov r15d, dword ptr [r14]
1417375cf: 0fb60533d6ef01  movzx eax, byte ptr [rip + 0x1efd633]
1417375d6: b918000000  mov ecx, 0x18
1417375db: ba08000000  mov edx, 8
1417375e0: e82bc6edfe  call 0x140613c10
1417375e5: 4989f6  mov r14, rsi
1417375e8: 4885c0  test rax, rax
1417375eb: 75b3  jne 0x1417375a0
1417375ed: 4c896dd8  mov qword ptr [rbp - 0x28], r13
1417375f1: b908000000  mov ecx, 8
1417375f6: ba18000000  mov edx, 0x18
1417375fb: e8001a1a01  call 0x1428d9000
141737600: 0f0b  ud2
141737602: 48c7471000000000  mov qword ptr [rdi + 0x10], 0
14173760a: 48c70708000000  mov qword ptr [rdi], 8
141737611: 48c7470808000000  mov qword ptr [rdi + 8], 8
141737619: 48c7471808000000  mov qword ptr [rdi + 0x18], 8
141737621: 488b45f0  mov rax, qword ptr [rbp - 0x10]
141737625: 488b4de0  mov rcx, qword ptr [rbp - 0x20]
141737629: 488901  mov qword ptr [rcx], rax
14173762c: 488b45f8  mov rax, qword ptr [rbp - 8]
141737630: 48894108  mov qword ptr [rcx + 8], rax
141737634: 488b45e8  mov rax, qword ptr [rbp - 0x18]
141737638: 48894110  mov qword ptr [rcx + 0x10], rax
14173763c: 4889c8  mov rax, rcx
14173763f: 4883c458  add rsp, 0x58
141737643: 5b  pop rbx
141737644: 5f  pop rdi
141737645: 5e  pop rsi
141737646: 415c  pop r12
141737648: 415d  pop r13
14173764a: 415e  pop r14
14173764c: 415f  pop r15
14173764e: 5d  pop rbp
14173764f: c3  ret
141737650: 4889542410  mov qword ptr [rsp + 0x10], rdx
141737655: 55  push rbp
141737656: 4157  push r15
141737658: 4156  push r14
14173765a: 4155  push r13
14173765c: 4154  push r12
14173765e: 56  push rsi
14173765f: 57  push rdi
141737660: 53  push rbx
141737661: 4883ec28  sub rsp, 0x28
141737665: 488d6a50  lea rbp, [rdx + 0x50]
141737669: 48837df000  cmp qword ptr [rbp - 0x10], 0
14173766e: 7417  je 0x141737687
141737670: 488b55f0  mov rdx, qword ptr [rbp - 0x10]
141737674: 48c1e204  shl rdx, 4
141737678: 41b808000000  mov r8d, 8
14173767e: 488b4df8  mov rcx, qword ptr [rbp - 8]
141737682: e899c5edfe  call 0x140613c20
141737687: 90  nop
141737688: 4883c428  add rsp, 0x28
14173768c: 5b  pop rbx
14173768d: 5f  pop rdi
14173768e: 5e  pop rsi
14173768f: 415c  pop r12
141737691: 415d  pop r13
141737693: 415e  pop r14
141737695: 415f  pop r15
141737697: 5d  pop rbp
141737698: c3  ret
141737699: 0f1f8000000000  nop dword ptr [rax]
1417376a0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1417376a5: 55  push rbp
1417376a6: 4157  push r15
1417376a8: 4156  push r14
1417376aa: 4155  push r13
1417376ac: 4154  push r12
1417376ae: 56  push rsi
1417376af: 57  push rdi
1417376b0: 53  push rbx
1417376b1: 4883ec28  sub rsp, 0x28
1417376b5: 488d6a50  lea rbp, [rdx + 0x50]
1417376b9: 488d4dd0  lea rcx, [rbp - 0x30]
1417376bd: e8aef9ffff  call 0x141737070
1417376c2: 90  nop
1417376c3: 4883c428  add rsp, 0x28
1417376c7: 5b  pop rbx
1417376c8: 5f  pop rdi
1417376c9: 5e  pop rsi
1417376ca: 415c  pop r12
1417376cc: 415d  pop r13
1417376ce: 415e  pop r14
1417376d0: 415f  pop r15
1417376d2: 5d  pop rbp
1417376d3: c3  ret
