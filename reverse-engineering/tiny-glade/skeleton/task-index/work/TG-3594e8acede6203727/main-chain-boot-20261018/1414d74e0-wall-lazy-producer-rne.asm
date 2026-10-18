0x1414d74e0 55 push rbp
0x1414d74e1 4157 push r15
0x1414d74e3 4156 push r14
0x1414d74e5 56 push rsi
0x1414d74e6 57 push rdi
0x1414d74e7 53 push rbx
0x1414d74e8 4881ecc8000000 sub rsp, 0xc8
0x1414d74ef 488dac2480000000 lea rbp, [rsp + 0x80]
0x1414d74f7 48c74540feffffff mov qword ptr [rbp + 0x40], 0xfffffffffffffffe
0x1414d74ff 4889d7 mov rdi, rdx
0x1414d7502 4889ce mov rsi, rcx
0x1414d7505 e8e6b8f1ff call 0x1413f2df0
0x1414d750a 41be01000000 mov r14d, 1
0x1414d7510 f0440fc1b7b0050000 lock xadd dword ptr [rdi + 0x5b0], r14d
0x1414d7519 31c0 xor eax, eax
0x1414d751b 483b8648010000 cmp rax, qword ptr [rsi + 0x148]
0x1414d7522 0f80de010000 jo 0x1414d7706
0x1414d7528 488b8638010000 mov rax, qword ptr [rsi + 0x138]
0x1414d752f 48398718020000 cmp qword ptr [rdi + 0x218], rax
0x1414d7536 0f8637010000 jbe 0x1414d7673
0x1414d753c 488b8f10020000 mov rcx, qword ptr [rdi + 0x210]
0x1414d7543 488b04c1 mov rax, qword ptr [rcx + rax*8]
0x1414d7547 4885c0 test rax, rax
0x1414d754a 0f8423010000 je 0x1414d7673
0x1414d7550 4c8bbfe0010000 mov r15, qword ptr [rdi + 0x1e0]
0x1414d7557 48f7d0 not rax
0x1414d755a 488d0440 lea rax, [rax + rax*2]
0x1414d755e 48c1e005 shl rax, 5
0x1414d7562 49837c072000 cmp qword ptr [r15 + rax + 0x20], 0
0x1414d7568 0f8405010000 je 0x1414d7673
0x1414d756e 4901c7 add r15, rax
0x1414d7571 498b5f10 mov rbx, qword ptr [r15 + 0x10]
0x1414d7575 8b87b0050000 mov eax, dword ptr [rdi + 0x5b0]
0x1414d757b 488b8640010000 mov rax, qword ptr [rsi + 0x140]
0x1414d7582 48398718020000 cmp qword ptr [rdi + 0x218], rax
0x1414d7589 0f8600010000 jbe 0x1414d768f
0x1414d758f 488b8f10020000 mov rcx, qword ptr [rdi + 0x210]
0x1414d7596 488b0cc1 mov rcx, qword ptr [rcx + rax*8]
0x1414d759a 4885c9 test rcx, rcx
0x1414d759d 0f84ec000000 je 0x1414d768f
0x1414d75a3 488b87e0010000 mov rax, qword ptr [rdi + 0x1e0]
0x1414d75aa 48f7d1 not rcx
0x1414d75ad 488d0c49 lea rcx, [rcx + rcx*2]
0x1414d75b1 48c1e105 shl rcx, 5
0x1414d75b5 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x1414d75bb 0f84ce000000 je 0x1414d768f
0x1414d75c1 8b9630010000 mov edx, dword ptr [rsi + 0x130]
0x1414d75c7 4801c8 add rax, rcx
0x1414d75ca 4d8d4758 lea r8, [r15 + 0x58]
0x1414d75ce 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1414d75d2 448b8fb0050000 mov r9d, dword ptr [rdi + 0x5b0]
0x1414d75d9 48895da8 mov qword ptr [rbp - 0x58], rbx
0x1414d75dd 4c8945b0 mov qword ptr [rbp - 0x50], r8
0x1414d75e1 4d8d475c lea r8, [r15 + 0x5c]
0x1414d75e5 4c8945b8 mov qword ptr [rbp - 0x48], r8
0x1414d75e9 4c8d4058 lea r8, [rax + 0x58]
0x1414d75ed 8955c0 mov dword ptr [rbp - 0x40], edx
0x1414d75f0 448975c4 mov dword ptr [rbp - 0x3c], r14d
0x1414d75f4 48894dc8 mov qword ptr [rbp - 0x38], rcx
0x1414d75f8 4c8945d0 mov qword ptr [rbp - 0x30], r8
0x1414d75fc 4c8d405c lea r8, [rax + 0x5c]
0x1414d7600 4c8945d8 mov qword ptr [rbp - 0x28], r8
0x1414d7604 8955e0 mov dword ptr [rbp - 0x20], edx
0x1414d7607 448975e4 mov dword ptr [rbp - 0x1c], r14d
0x1414d760b 48897de8 mov qword ptr [rbp - 0x18], rdi
0x1414d760f 488d9648010000 lea rdx, [rsi + 0x148]
0x1414d7616 48c745f000000000 mov qword ptr [rbp - 0x10], 0
0x1414d761e 488955f8 mov qword ptr [rbp - 8], rdx
0x1414d7622 4489705c mov dword ptr [rax + 0x5c], r14d
0x1414d7626 e8b5e253ff call 0x140a158e0
0x1414d762b 48894538 mov qword ptr [rbp + 0x38], rax
0x1414d762f 48894518 mov qword ptr [rbp + 0x18], rax
0x1414d7633 4589775c mov dword ptr [r15 + 0x5c], r14d
0x1414d7637 4883c318 add rbx, 0x18
0x1414d763b 4889d9 mov rcx, rbx
0x1414d763e e84dccf7ff call 0x141454290
0x1414d7643 488d4de8 lea rcx, [rbp - 0x18]
0x1414d7647 4c8d0dfa2b6001 lea r9, [rip + 0x1602bfa]
0x1414d764e 488b5538 mov rdx, qword ptr [rbp + 0x38]
0x1414d7652 4989c0 mov r8, rax
0x1414d7655 e8a696e0ff call 0x1412e0d00
0x1414d765a 4489b630010000 mov dword ptr [rsi + 0x130], r14d
0x1414d7661 31c0 xor eax, eax
0x1414d7663 4881c4c8000000 add rsp, 0xc8
0x1414d766a 5b pop rbx
0x1414d766b 5f pop rdi
0x1414d766c 5e pop rsi
0x1414d766d 415e pop r14
0x1414d766f 415f pop r15
0x1414d7671 5d pop rbp
0x1414d7672 c3 ret 
0x1414d7673 4881c698000000 add rsi, 0x98
0x1414d767a 488d0575f47801 lea rax, [rip + 0x178f475]
0x1414d7681 48894508 mov qword ptr [rbp + 8], rax
0x1414d7685 48c7451077000000 mov qword ptr [rbp + 0x10], 0x77
0x1414d768d eb1a jmp 0x1414d76a9
0x1414d768f 4881c698000000 add rsi, 0x98
0x1414d7696 488d05ac1b7901 lea rax, [rip + 0x1791bac]
0x1414d769d 48894508 mov qword ptr [rbp + 8], rax
0x1414d76a1 48c7451036000000 mov qword ptr [rbp + 0x10], 0x36
0x1414d76a9 48897518 mov qword ptr [rbp + 0x18], rsi
0x1414d76ad 488d051c44b6fe lea rax, [rip - 0x149bbe4]
0x1414d76b4 48894520 mov qword ptr [rbp + 0x20], rax
0x1414d76b8 488d4508 lea rax, [rbp + 8]
0x1414d76bc 48894528 mov qword ptr [rbp + 0x28], rax
0x1414d76c0 488d05c927c3fe lea rax, [rip - 0x13cd837]
0x1414d76c7 48894530 mov qword ptr [rbp + 0x30], rax
0x1414d76cb 488d05aee77801 lea rax, [rip + 0x178e7ae]
0x1414d76d2 488945a8 mov qword ptr [rbp - 0x58], rax
0x1414d76d6 48c745b002000000 mov qword ptr [rbp - 0x50], 2
0x1414d76de 48c745c800000000 mov qword ptr [rbp - 0x38], 0
0x1414d76e6 488d4518 lea rax, [rbp + 0x18]
0x1414d76ea 488945b8 mov qword ptr [rbp - 0x48], rax
0x1414d76ee 48c745c002000000 mov qword ptr [rbp - 0x40], 2
0x1414d76f6 488d1543217901 lea rdx, [rip + 0x1792143]
0x1414d76fd 488d4da8 lea rcx, [rbp - 0x58]
0x1414d7701 e88a1c4001 call 0x1428d9390
0x1414d7706 488d0d34427901 lea rcx, [rip + 0x1794234]
0x1414d770d 4c8d05f4427901 lea r8, [rip + 0x17942f4]
0x1414d7714 ba59000000 mov edx, 0x59
0x1414d7719 e8121c4001 call 0x1428d9330
0x1414d771e cc int3 
0x1414d771f 90 nop 
0x1414d7720 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1414d7725 55 push rbp
0x1414d7726 4157 push r15
0x1414d7728 4156 push r14
0x1414d772a 56 push rsi
0x1414d772b 57 push rdi
0x1414d772c 53 push rbx
0x1414d772d 4883ec28 sub rsp, 0x28
0x1414d7731 488daa80000000 lea rbp, [rdx + 0x80]
0x1414d7738 488b4538 mov rax, qword ptr [rbp + 0x38]
0x1414d773c f048ff08 lock dec qword ptr [rax]
0x1414d7740 7509 jne 0x1414d774b
0x1414d7742 488d4d18 lea rcx, [rbp + 0x18]
0x1414d7746 e8f55868ff call 0x140b5d040
0x1414d774b 90 nop 
0x1414d774c 4883c428 add rsp, 0x28
0x1414d7750 5b pop rbx
0x1414d7751 5f pop rdi
0x1414d7752 5e pop rsi
0x1414d7753 415e pop r14
0x1414d7755 415f pop r15
0x1414d7757 5d pop rbp
0x1414d7758 c3 ret 
