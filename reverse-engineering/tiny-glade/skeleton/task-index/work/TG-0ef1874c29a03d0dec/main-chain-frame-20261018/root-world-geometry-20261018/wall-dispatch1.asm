1415f9350 55 push rbp
1415f9351 4157 push r15
1415f9353 4156 push r14
1415f9355 4154 push r12
1415f9357 56 push rsi
1415f9358 57 push rdi
1415f9359 53 push rbx
1415f935a 4883ec70 sub rsp, 0x70
1415f935e 488d6c2470 lea rbp, [rsp + 0x70]
1415f9363 48c745f8feffffff mov qword ptr [rbp - 8], 0xfffffffffffffffe
1415f936b 488955b8 mov qword ptr [rbp - 0x48], rdx
1415f936f 4c8945c0 mov qword ptr [rbp - 0x40], r8
1415f9373 488b01 mov rax, qword ptr [rcx]
1415f9376 488945e8 mov qword ptr [rbp - 0x18], rax
1415f937a 488b4108 mov rax, qword ptr [rcx + 8]
1415f937e 488945e0 mov qword ptr [rbp - 0x20], rax
1415f9382 48894dc8 mov qword ptr [rbp - 0x38], rcx
1415f9386 488b4110 mov rax, qword ptr [rcx + 0x10]
1415f938a 4885c0 test rax, rax
1415f938d 7461 je 0x1415f93f0
1415f938f 4c8b7de0 mov r15, qword ptr [rbp - 0x20]
1415f9393 498d1cc7 lea rbx, [r15 + rax*8]
1415f9397 488d75d4 lea rsi, [rbp - 0x2c]
1415f939b 49be0100000001000000 movabs r14, 0x100000001
1415f93a5 488d7db8 lea rdi, [rbp - 0x48]
1415f93a9 0f1f8000000000 nop dword ptr [rax]
1415f93b0 418b07 mov eax, dword ptr [r15]
1415f93b3 458b6704 mov r12d, dword ptr [r15 + 4]
1415f93b7 8945d4 mov dword ptr [rbp - 0x2c], eax
1415f93ba 448965d8 mov dword ptr [rbp - 0x28], r12d
1415f93be 488b4db8 mov rcx, qword ptr [rbp - 0x48]
1415f93c2 488b55c0 mov rdx, qword ptr [rbp - 0x40]
1415f93c6 41b908000000 mov r9d, 8
1415f93cc 4989f0 mov r8, rsi
1415f93cf e87ce60f00 call 0x1416f7a50
1415f93d4 448965d4 mov dword ptr [rbp - 0x2c], r12d
1415f93d8 4c8975d8 mov qword ptr [rbp - 0x28], r14
1415f93dc 4889f9 mov rcx, rdi
1415f93df 4889f2 mov rdx, rsi
1415f93e2 e8b95b37ff call 0x14096efa0
1415f93e7 4983c708 add r15, 8
1415f93eb 4939df cmp r15, rbx
1415f93ee 75c0 jne 0x1415f93b0
1415f93f0 488b55e8 mov rdx, qword ptr [rbp - 0x18]
1415f93f4 4885d2 test rdx, rdx
1415f93f7 7413 je 0x1415f940c
1415f93f9 48c1e203 shl rdx, 3
1415f93fd 41b804000000 mov r8d, 4
1415f9403 488b4de0 mov rcx, qword ptr [rbp - 0x20]
1415f9407 e814a801ff call 0x140613c20
1415f940c 488b45c8 mov rax, qword ptr [rbp - 0x38]
1415f9410 488b4818 mov rcx, qword ptr [rax + 0x18]
1415f9414 48894de8 mov qword ptr [rbp - 0x18], rcx
1415f9418 488b4820 mov rcx, qword ptr [rax + 0x20]
1415f941c 48894de0 mov qword ptr [rbp - 0x20], rcx
1415f9420 488b4028 mov rax, qword ptr [rax + 0x28]
1415f9424 4885c0 test rax, rax
1415f9427 746f je 0x1415f9498
1415f9429 4c8b7de0 mov r15, qword ptr [rbp - 0x20]
1415f942d 498d1cc7 lea rbx, [r15 + rax*8]
1415f9431 488d75d4 lea rsi, [rbp - 0x2c]
1415f9435 49be0100000001000000 movabs r14, 0x100000001
1415f943f 488d7db8 lea rdi, [rbp - 0x48]
1415f9443 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1415f9450 418b07 mov eax, dword ptr [r15]
1415f9453 458b6704 mov r12d, dword ptr [r15 + 4]
1415f9457 4489e1 mov ecx, r12d
1415f945a 81c900000080 or ecx, 0x80000000
1415f9460 8945d4 mov dword ptr [rbp - 0x2c], eax
1415f9463 894dd8 mov dword ptr [rbp - 0x28], ecx
1415f9466 488b4db8 mov rcx, qword ptr [rbp - 0x48]
1415f946a 488b55c0 mov rdx, qword ptr [rbp - 0x40]
1415f946e 41b908000000 mov r9d, 8
1415f9474 4989f0 mov r8, rsi
1415f9477 e8d4e50f00 call 0x1416f7a50
1415f947c 448965d4 mov dword ptr [rbp - 0x2c], r12d
1415f9480 4c8975d8 mov qword ptr [rbp - 0x28], r14
1415f9484 4889f9 mov rcx, rdi
1415f9487 4889f2 mov rdx, rsi
1415f948a e8115b37ff call 0x14096efa0
1415f948f 4983c708 add r15, 8
1415f9493 4939df cmp r15, rbx
1415f9496 75b8 jne 0x1415f9450
1415f9498 488b55e8 mov rdx, qword ptr [rbp - 0x18]
1415f949c 4885d2 test rdx, rdx
1415f949f 7413 je 0x1415f94b4
1415f94a1 48c1e203 shl rdx, 3
1415f94a5 41b804000000 mov r8d, 4
1415f94ab 488b4de0 mov rcx, qword ptr [rbp - 0x20]
1415f94af e86ca701ff call 0x140613c20
1415f94b4 90 nop 
1415f94b5 4883c470 add rsp, 0x70
1415f94b9 5b pop rbx
1415f94ba 5f pop rdi
1415f94bb 5e pop rsi
1415f94bc 415c pop r12
1415f94be 415e pop r14
1415f94c0 415f pop r15
1415f94c2 5d pop rbp
1415f94c3 c3 ret 
1415f94c4 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1415f94d0 4889542410 mov qword ptr [rsp + 0x10], rdx
1415f94d5 55 push rbp
1415f94d6 4157 push r15
1415f94d8 4156 push r14
1415f94da 4154 push r12
1415f94dc 56 push rsi
1415f94dd 57 push rdi
1415f94de 53 push rbx
1415f94df 4883ec20 sub rsp, 0x20
1415f94e3 488d6a70 lea rbp, [rdx + 0x70]
1415f94e7 48837de800 cmp qword ptr [rbp - 0x18], 0
1415f94ec 7417 je 0x1415f9505
1415f94ee 488b55e8 mov rdx, qword ptr [rbp - 0x18]
1415f94f2 48c1e203 shl rdx, 3
1415f94f6 41b804000000 mov r8d, 4
1415f94fc 488b4de0 mov rcx, qword ptr [rbp - 0x20]
1415f9500 e81ba701ff call 0x140613c20
1415f9505 c645f700 mov byte ptr [rbp - 9], 0
1415f9509 4883c420 add rsp, 0x20
1415f950d 5b pop rbx
1415f950e 5f pop rdi
1415f950f 5e pop rsi
1415f9510 415c pop r12
1415f9512 415e pop r14
1415f9514 415f pop r15
1415f9516 5d pop rbp
1415f9517 c3 ret 
1415f9518 0f1f840000000000 nop dword ptr [rax + rax]
1415f9520 4889542410 mov qword ptr [rsp + 0x10], rdx
1415f9525 55 push rbp
1415f9526 4157 push r15
1415f9528 4156 push r14
1415f952a 4154 push r12
1415f952c 56 push rsi
1415f952d 57 push rdi
1415f952e 53 push rbx
1415f952f 4883ec20 sub rsp, 0x20
1415f9533 488d6a70 lea rbp, [rdx + 0x70]
1415f9537 48837de800 cmp qword ptr [rbp - 0x18], 0
1415f953c 7417 je 0x1415f9555
1415f953e 488b55e8 mov rdx, qword ptr [rbp - 0x18]
1415f9542 48c1e203 shl rdx, 3
1415f9546 41b804000000 mov r8d, 4
1415f954c 488b4de0 mov rcx, qword ptr [rbp - 0x20]
1415f9550 e8cba601ff call 0x140613c20
1415f9555 c645f701 mov byte ptr [rbp - 9], 1
1415f9559 4883c420 add rsp, 0x20
1415f955d 5b pop rbx
1415f955e 5f pop rdi
1415f955f 5e pop rsi
1415f9560 415c pop r12
1415f9562 415e pop r14
1415f9564 415f pop r15
1415f9566 5d pop rbp
1415f9567 c3 ret 
1415f9568 0f1f840000000000 nop dword ptr [rax + rax]
1415f9570 4889542410 mov qword ptr [rsp + 0x10], rdx
1415f9575 55 push rbp
1415f9576 4157 push r15
1415f9578 4156 push r14
1415f957a 4154 push r12
1415f957c 56 push rsi
1415f957d 57 push rdi
1415f957e 53 push rbx
1415f957f 4883ec20 sub rsp, 0x20
1415f9583 488d6a70 lea rbp, [rdx + 0x70]
1415f9587 807df700 cmp byte ptr [rbp - 9], 0
1415f958b 7524 jne 0x1415f95b1
1415f958d 488b45c8 mov rax, qword ptr [rbp - 0x38]
1415f9591 488b5018 mov rdx, qword ptr [rax + 0x18]
1415f9595 4885d2 test rdx, rdx
1415f9598 7417 je 0x1415f95b1
1415f959a 488b45c8 mov rax, qword ptr [rbp - 0x38]
1415f959e 488b4820 mov rcx, qword ptr [rax + 0x20]
1415f95a2 48c1e203 shl rdx, 3
1415f95a6 41b804000000 mov r8d, 4
1415f95ac e86fa601ff call 0x140613c20
1415f95b1 90 nop 
1415f95b2 4883c420 add rsp, 0x20
1415f95b6 5b pop rbx
1415f95b7 5f pop rdi
1415f95b8 5e pop rsi
1415f95b9 415c pop r12
1415f95bb 415e pop r14
1415f95bd 415f pop r15
1415f95bf 5d pop rbp
1415f95c0 c3 ret 
