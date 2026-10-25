0x14143fc1e  mov      rax, qword ptr [rcx + 8]
0x14143fc22  lock inc qword ptr [rax]
0x14143fc26  jle      0x141441b81
0x14143fc2c  xor      dil, 1
0x14143fc30  movzx    ecx, dil
0x14143fc34  mov      dword ptr [rbp + 0x30], ecx
0x14143fc37  mov      dword ptr [rbp + 0x34], 0x3dcccccd
0x14143fc3e  mov      qword ptr [rbp + 0x38], rax
0x14143fc42  mov      dword ptr [rbp + 0x40], 0
0x14143fc49  mov      byte ptr [rbp + 0x44], bl
0x14143fc4c  mov      byte ptr [rbp + 0x45], 0
0x14143fc50  mov      byte ptr [rbp + 0x28], 9
0x14143fc54  mov      rdi, qword ptr [rsi + 0x10]
0x14143fc58  cmp      rdi, qword ptr [rsi]
0x14143fc5b  jne      0x141440b61
0x14143fc61  lea      rdx, [rip + 0x181f468]
0x14143fc68  mov      rcx, rsi
0x14143fc6b  call     0x1405629f0
0x14143fc70  jmp      0x141440b61
