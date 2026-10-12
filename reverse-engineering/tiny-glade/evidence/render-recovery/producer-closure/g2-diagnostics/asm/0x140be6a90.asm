==== 0x140be6a90  file_off=0xbe5e90
  140be6a90  55                       push rbp
  140be6a91  4883ec50                 sub rsp, 0x50
  140be6a95  488d6c2450               lea rbp, [rsp + 0x50]
  140be6a9a  48c745f8feffffff         mov qword ptr [rbp - 8], 0xfffffffffffffffe
  140be6aa2  488d4dd0                 lea rcx, [rbp - 0x30]
  140be6aa6  ba00000100               mov edx, 0x10000
  140be6aab  e860a8cd00               call 0x1418c1310
  140be6ab0  488b0d89e3a402           mov rcx, qword ptr [rip + 0x2a4e389] -> 0x143634e40
  140be6ab7  488b158ae3a402           mov rdx, qword ptr [rip + 0x2a4e38a] -> 0x143634e48
  140be6abe  488b45e0                 mov rax, qword ptr [rbp - 0x20]
  140be6ac2  48890587e3a402           mov qword ptr [rip + 0x2a4e387], rax -> 0x143634e50
  140be6ac9  0f1045d0                 movups xmm0, xmmword ptr [rbp - 0x30]
  140be6acd  0f11056ce3a402           movups xmmword ptr [rip + 0x2a4e36c], xmm0 -> 0x143634e40
  140be6ad4  4885c9                   test rcx, rcx
  140be6ad7  7506                     jne 0x140be6adf
  140be6ad9  4883c450                 add rsp, 0x50
  140be6add  5d                       pop rbp
  140be6ade  c3                       ret 
  140be6adf  488955e8                 mov qword ptr [rbp - 0x18], rdx
  140be6ae3  48894df0                 mov qword ptr [rbp - 0x10], rcx
  140be6ae7  488d0d62c4f201           lea rcx, [rip + 0x1f2c462] -> 0x142b12f50
  140be6aee  4c8d057bc4f201           lea r8, [rip + 0x1f2c47b] -> 0x142b12f70
  140be6af5  ba20000000               mov edx, 0x20
  140be6afa  e83129cf01               call 0x1428d9430
  140be6aff  0f0b                     ud2 
  140be6b01  6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
  140be6b10  4889542410               mov qword ptr [rsp + 0x10], rdx
  140be6b15  55                       push rbp
  140be6b16  4883ec20                 sub rsp, 0x20
  140be6b1a  488d6a50                 lea rbp, [rdx + 0x50]
  140be6b1e  488b55e8                 mov rdx, qword ptr [rbp - 0x18]
  140be6b22  4885d2                   test rdx, rdx
  140be6b25  740f                     je 0x140be6b36
  140be6b27  41b801000000             mov r8d, 1
  140be6b2d  488b4df0                 mov rcx, qword ptr [rbp - 0x10]
  140be6b31  e8ead0a2ff               call 0x140613c20
  140be6b36  90                       nop 
  140be6b37  4883c420                 add rsp, 0x20
  140be6b3b  5d                       pop rbp
  140be6b3c  c3                       ret 
  140be6b3d  cc                       int3 
  140be6b3e  cc                       int3 
  140be6b3f  cc                       int3 
