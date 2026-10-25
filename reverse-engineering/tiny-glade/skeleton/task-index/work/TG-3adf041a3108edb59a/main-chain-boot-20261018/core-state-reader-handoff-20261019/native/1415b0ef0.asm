1415b0ef0: 55  push rbp
1415b0ef1: 4157  push r15
1415b0ef3: 4156  push r14
1415b0ef5: 4154  push r12
1415b0ef7: 56  push rsi
1415b0ef8: 57  push rdi
1415b0ef9: 53  push rbx
1415b0efa: 4881ec90000000  sub rsp, 0x90
1415b0f01: 488dac2480000000  lea rbp, [rsp + 0x80]
1415b0f09: 48c74508feffffff  mov qword ptr [rbp + 8], 0xfffffffffffffffe
1415b0f11: 4889ce  mov rsi, rcx
1415b0f14: 4c8b7210  mov r14, qword ptr [rdx + 0x10]
1415b0f18: 488d4dd8  lea rcx, [rbp - 0x28]
1415b0f1c: e85f715dff  call 0x140b88080
1415b0f21: 4c8b65e0  mov r12, qword ptr [rbp - 0x20]
1415b0f25: 488b7df0  mov rdi, qword ptr [rbp - 0x10]
1415b0f29: 4939fc  cmp r12, rdi
1415b0f2c: 7458  je 0x1415b0f86
1415b0f2e: 498d5c2420  lea rbx, [r12 + 0x20]
1415b0f33: 48895de0  mov qword ptr [rbp - 0x20], rbx
1415b0f37: 410fb60424  movzx eax, byte ptr [r12]
1415b0f3c: 3c06  cmp al, 6
1415b0f3e: 7449  je 0x1415b0f89
1415b0f40: 8845a8  mov byte ptr [rbp - 0x58], al
1415b0f43: 410f10442401  movups xmm0, xmmword ptr [r12 + 1]
1415b0f49: 410f104c2410  movups xmm1, xmmword ptr [r12 + 0x10]
1415b0f4f: 0f1145a9  movups xmmword ptr [rbp - 0x57], xmm0
1415b0f53: 0f114db8  movups xmmword ptr [rbp - 0x48], xmm1
1415b0f57: 48c744242002000000  mov qword ptr [rsp + 0x20], 2
1415b0f60: 4c8d05e1646c01  lea r8, [rip + 0x16c64e1]
1415b0f67: 488d4df8  lea rcx, [rbp - 8]
1415b0f6b: 488d55a8  lea rdx, [rbp - 0x58]
1415b0f6f: 41b904000000  mov r9d, 4
1415b0f75: e8962affff  call 0x1415a3a10
1415b0f7a: 837df801  cmp dword ptr [rbp - 8], 1
1415b0f7e: 756b  jne 0x1415b0feb
1415b0f80: 488b4500  mov rax, qword ptr [rbp]
1415b0f84: eb18  jmp 0x1415b0f9e
1415b0f86: 4c89e3  mov rbx, r12
1415b0f89: 488d1508a06c01  lea rdx, [rip + 0x16ca008]
1415b0f90: 4c8d05c9906c01  lea r8, [rip + 0x16c90c9]
1415b0f97: 31c9  xor ecx, ecx
1415b0f99: e8828d3401  call 0x1428f9d20
1415b0f9e: 48894608  mov qword ptr [rsi + 8], rax
1415b0fa2: c70601000000  mov dword ptr [rsi], 1
1415b0fa8: 4829df  sub rdi, rbx
1415b0fab: 48c1ef05  shr rdi, 5
1415b0faf: 4889d9  mov rcx, rbx
1415b0fb2: 4889fa  mov rdx, rdi
1415b0fb5: e8b67d2dff  call 0x140888d70
1415b0fba: 488b55e8  mov rdx, qword ptr [rbp - 0x18]
1415b0fbe: 4885d2  test rdx, rdx
1415b0fc1: 7413  je 0x1415b0fd6
1415b0fc3: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
1415b0fc7: 48c1e205  shl rdx, 5
1415b0fcb: 41b808000000  mov r8d, 8
1415b0fd1: e84a2c06ff  call 0x140613c20
1415b0fd6: 4889f0  mov rax, rsi
1415b0fd9: 4881c490000000  add rsp, 0x90
1415b0fe0: 5b  pop rbx
1415b0fe1: 5f  pop rdi
1415b0fe2: 5e  pop rsi
1415b0fe3: 415c  pop r12
1415b0fe5: 415e  pop r14
1415b0fe7: 415f  pop r15
1415b0fe9: 5d  pop rbp
1415b0fea: c3  ret 
1415b0feb: f20f1045fc  movsd xmm0, qword ptr [rbp - 4]
1415b0ff0: f20f1145c8  movsd qword ptr [rbp - 0x38], xmm0
1415b0ff5: 4c8b7dc8  mov r15, qword ptr [rbp - 0x38]
1415b0ff9: 4839fb  cmp rbx, rdi
1415b0ffc: 7413  je 0x1415b1011
1415b0ffe: 498d5c2440  lea rbx, [r12 + 0x40]
1415b1003: 48895de0  mov qword ptr [rbp - 0x20], rbx
1415b1007: 410fb6442420  movzx eax, byte ptr [r12 + 0x20]
1415b100d: 3c06  cmp al, 6
1415b100f: 753b  jne 0x1415b104c
1415b1011: 0f57c0  xorps xmm0, xmm0
1415b1014: 4c89f8  mov rax, r15
1415b1017: 48c1e820  shr rax, 0x20
1415b101b: 8945d0  mov dword ptr [rbp - 0x30], eax
1415b101e: f30f1145d4  movss dword ptr [rbp - 0x2c], xmm0
1415b1023: 4829df  sub rdi, rbx
1415b1026: 7559  jne 0x1415b1081
1415b1028: 488b45d0  mov rax, qword ptr [rbp - 0x30]
1415b102c: 44897e04  mov dword ptr [rsi + 4], r15d
1415b1030: 31c9  xor ecx, ecx
1415b1032: 48894608  mov qword ptr [rsi + 8], rax
1415b1036: 890e  mov dword ptr [rsi], ecx
1415b1038: 48c1ef05  shr rdi, 5
1415b103c: 4889d9  mov rcx, rbx
1415b103f: 4889fa  mov rdx, rdi
1415b1042: e8297d2dff  call 0x140888d70
1415b1047: e96effffff  jmp 0x1415b0fba
1415b104c: 8845a8  mov byte ptr [rbp - 0x58], al
1415b104f: 410f10442421  movups xmm0, xmmword ptr [r12 + 0x21]
1415b1055: 410f104c2430  movups xmm1, xmmword ptr [r12 + 0x30]
1415b105b: 0f1145a9  movups xmmword ptr [rbp - 0x57], xmm0
1415b105f: 0f114db8  movups xmmword ptr [rbp - 0x48], xmm1
1415b1063: 488d4df8  lea rcx, [rbp - 8]
1415b1067: 488d55a8  lea rdx, [rbp - 0x58]
1415b106b: e80093feff  call 0x14159a370
1415b1070: 837df801  cmp dword ptr [rbp - 8], 1
1415b1074: 0f8406ffffff  je 0x1415b0f80
1415b107a: f30f1045fc  movss xmm0, dword ptr [rbp - 4]
1415b107f: eb93  jmp 0x1415b1014
1415b1081: 488d15c88f6c01  lea rdx, [rip + 0x16c8fc8]
1415b1088: 4c8d05d18f6c01  lea r8, [rip + 0x16c8fd1]
1415b108f: 4c89f1  mov rcx, r14
1415b1092: e8898c3401  call 0x1428f9d20
1415b1097: b901000000  mov ecx, 1
1415b109c: eb94  jmp 0x1415b1032
1415b109e: 6690  nop 
1415b10a0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b10a5: 55  push rbp
1415b10a6: 4157  push r15
1415b10a8: 4156  push r14
1415b10aa: 4154  push r12
1415b10ac: 56  push rsi
1415b10ad: 57  push rdi
1415b10ae: 53  push rbx
1415b10af: 4883ec30  sub rsp, 0x30
1415b10b3: 488daa80000000  lea rbp, [rdx + 0x80]
1415b10ba: 488d4dd8  lea rcx, [rbp - 0x28]
1415b10be: e89d942dff  call 0x14088a560
1415b10c3: 90  nop 
1415b10c4: 4883c430  add rsp, 0x30
1415b10c8: 5b  pop rbx
1415b10c9: 5f  pop rdi
1415b10ca: 5e  pop rsi
1415b10cb: 415c  pop r12
1415b10cd: 415e  pop r14
1415b10cf: 415f  pop r15
1415b10d1: 5d  pop rbp
1415b10d2: c3  ret 
1415b10d3: 666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415b10e0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b10e5: 55  push rbp
1415b10e6: 4157  push r15
1415b10e8: 4156  push r14
1415b10ea: 4154  push r12
1415b10ec: 56  push rsi
1415b10ed: 57  push rdi
1415b10ee: 53  push rbx
1415b10ef: 4883ec30  sub rsp, 0x30
1415b10f3: 488daa80000000  lea rbp, [rdx + 0x80]
1415b10fa: 488b55e8  mov rdx, qword ptr [rbp - 0x18]
1415b10fe: 4885d2  test rdx, rdx
1415b1101: 7413  je 0x1415b1116
1415b1103: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
1415b1107: 48c1e205  shl rdx, 5
1415b110b: 41b808000000  mov r8d, 8
1415b1111: e80a2b06ff  call 0x140613c20
1415b1116: 90  nop 
1415b1117: 4883c430  add rsp, 0x30
1415b111b: 5b  pop rbx
1415b111c: 5f  pop rdi
1415b111d: 5e  pop rsi
1415b111e: 415c  pop r12
1415b1120: 415e  pop r14
1415b1122: 415f  pop r15
1415b1124: 5d  pop rbp
1415b1125: c3  ret 
1415b1126: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415b1130: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b1135: 55  push rbp
1415b1136: 4157  push r15
1415b1138: 4156  push r14
1415b113a: 4154  push r12
1415b113c: 56  push rsi
1415b113d: 57  push rdi
1415b113e: 53  push rbx
1415b113f: 4883ec30  sub rsp, 0x30
1415b1143: 488daa80000000  lea rbp, [rdx + 0x80]
1415b114a: 488b55e8  mov rdx, qword ptr [rbp - 0x18]
1415b114e: 4885d2  test rdx, rdx
1415b1151: 7413  je 0x1415b1166
1415b1153: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
1415b1157: 48c1e205  shl rdx, 5
1415b115b: 41b808000000  mov r8d, 8
1415b1161: e8ba2a06ff  call 0x140613c20
1415b1166: 90  nop 
1415b1167: 4883c430  add rsp, 0x30
1415b116b: 5b  pop rbx
1415b116c: 5f  pop rdi
1415b116d: 5e  pop rsi
1415b116e: 415c  pop r12
1415b1170: 415e  pop r14
1415b1172: 415f  pop r15
1415b1174: 5d  pop rbp
1415b1175: c3  ret 
