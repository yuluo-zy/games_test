1415ad9a0: 55  push rbp
1415ad9a1: 4157  push r15
1415ad9a3: 4156  push r14
1415ad9a5: 4154  push r12
1415ad9a7: 56  push rsi
1415ad9a8: 57  push rdi
1415ad9a9: 53  push rbx
1415ad9aa: 4881ec90000000  sub rsp, 0x90
1415ad9b1: 488dac2480000000  lea rbp, [rsp + 0x80]
1415ad9b9: 48c74508feffffff  mov qword ptr [rbp + 8], 0xfffffffffffffffe
1415ad9c1: 4889ce  mov rsi, rcx
1415ad9c4: 4c8b7210  mov r14, qword ptr [rdx + 0x10]
1415ad9c8: 488d4dd8  lea rcx, [rbp - 0x28]
1415ad9cc: e8afa65dff  call 0x140b88080
1415ad9d1: 4c8b65e0  mov r12, qword ptr [rbp - 0x20]
1415ad9d5: 488b7df0  mov rdi, qword ptr [rbp - 0x10]
1415ad9d9: 4939fc  cmp r12, rdi
1415ad9dc: 7458  je 0x1415ada36
1415ad9de: 498d5c2420  lea rbx, [r12 + 0x20]
1415ad9e3: 48895de0  mov qword ptr [rbp - 0x20], rbx
1415ad9e7: 410fb60424  movzx eax, byte ptr [r12]
1415ad9ec: 3c06  cmp al, 6
1415ad9ee: 7449  je 0x1415ada39
1415ad9f0: 8845a8  mov byte ptr [rbp - 0x58], al
1415ad9f3: 410f10442401  movups xmm0, xmmword ptr [r12 + 1]
1415ad9f9: 410f104c2410  movups xmm1, xmmword ptr [r12 + 0x10]
1415ad9ff: 0f1145a9  movups xmmword ptr [rbp - 0x57], xmm0
1415ada03: 0f114db8  movups xmmword ptr [rbp - 0x48], xmm1
1415ada07: 48c744242002000000  mov qword ptr [rsp + 0x20], 2
1415ada10: 4c8d05319a6c01  lea r8, [rip + 0x16c9a31]
1415ada17: 488d4df8  lea rcx, [rbp - 8]
1415ada1b: 488d55a8  lea rdx, [rbp - 0x58]
1415ada1f: 41b904000000  mov r9d, 4
1415ada25: e8e65fffff  call 0x1415a3a10
1415ada2a: 837df801  cmp dword ptr [rbp - 8], 1
1415ada2e: 756b  jne 0x1415ada9b
1415ada30: 488b4500  mov rax, qword ptr [rbp]
1415ada34: eb18  jmp 0x1415ada4e
1415ada36: 4c89e3  mov rbx, r12
1415ada39: 488d15b0d56c01  lea rdx, [rip + 0x16cd5b0]
1415ada40: 4c8d0519c66c01  lea r8, [rip + 0x16cc619]
1415ada47: 31c9  xor ecx, ecx
1415ada49: e8d2c23401  call 0x1428f9d20
1415ada4e: 48894608  mov qword ptr [rsi + 8], rax
1415ada52: c70601000000  mov dword ptr [rsi], 1
1415ada58: 4829df  sub rdi, rbx
1415ada5b: 48c1ef05  shr rdi, 5
1415ada5f: 4889d9  mov rcx, rbx
1415ada62: 4889fa  mov rdx, rdi
1415ada65: e806b32dff  call 0x140888d70
1415ada6a: 488b55e8  mov rdx, qword ptr [rbp - 0x18]
1415ada6e: 4885d2  test rdx, rdx
1415ada71: 7413  je 0x1415ada86
1415ada73: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
1415ada77: 48c1e205  shl rdx, 5
1415ada7b: 41b808000000  mov r8d, 8
1415ada81: e89a6106ff  call 0x140613c20
1415ada86: 4889f0  mov rax, rsi
1415ada89: 4881c490000000  add rsp, 0x90
1415ada90: 5b  pop rbx
1415ada91: 5f  pop rdi
1415ada92: 5e  pop rsi
1415ada93: 415c  pop r12
1415ada95: 415e  pop r14
1415ada97: 415f  pop r15
1415ada99: 5d  pop rbp
1415ada9a: c3  ret 
1415ada9b: f20f1045fc  movsd xmm0, qword ptr [rbp - 4]
1415adaa0: f20f1145c8  movsd qword ptr [rbp - 0x38], xmm0
1415adaa5: 4c8b7dc8  mov r15, qword ptr [rbp - 0x38]
1415adaa9: 4839fb  cmp rbx, rdi
1415adaac: 7413  je 0x1415adac1
1415adaae: 498d5c2440  lea rbx, [r12 + 0x40]
1415adab3: 48895de0  mov qword ptr [rbp - 0x20], rbx
1415adab7: 410fb6442420  movzx eax, byte ptr [r12 + 0x20]
1415adabd: 3c06  cmp al, 6
1415adabf: 753b  jne 0x1415adafc
1415adac1: 0f57c0  xorps xmm0, xmm0
1415adac4: 4c89f8  mov rax, r15
1415adac7: 48c1e820  shr rax, 0x20
1415adacb: 8945d0  mov dword ptr [rbp - 0x30], eax
1415adace: f30f1145d4  movss dword ptr [rbp - 0x2c], xmm0
1415adad3: 4829df  sub rdi, rbx
1415adad6: 7559  jne 0x1415adb31
1415adad8: 488b45d0  mov rax, qword ptr [rbp - 0x30]
1415adadc: 44897e04  mov dword ptr [rsi + 4], r15d
1415adae0: 31c9  xor ecx, ecx
1415adae2: 48894608  mov qword ptr [rsi + 8], rax
1415adae6: 890e  mov dword ptr [rsi], ecx
1415adae8: 48c1ef05  shr rdi, 5
1415adaec: 4889d9  mov rcx, rbx
1415adaef: 4889fa  mov rdx, rdi
1415adaf2: e879b22dff  call 0x140888d70
1415adaf7: e96effffff  jmp 0x1415ada6a
1415adafc: 8845a8  mov byte ptr [rbp - 0x58], al
1415adaff: 410f10442421  movups xmm0, xmmword ptr [r12 + 0x21]
1415adb05: 410f104c2430  movups xmm1, xmmword ptr [r12 + 0x30]
1415adb0b: 0f1145a9  movups xmmword ptr [rbp - 0x57], xmm0
1415adb0f: 0f114db8  movups xmmword ptr [rbp - 0x48], xmm1
1415adb13: 488d4df8  lea rcx, [rbp - 8]
1415adb17: 488d55a8  lea rdx, [rbp - 0x58]
1415adb1b: e850c8feff  call 0x14159a370
1415adb20: 837df801  cmp dword ptr [rbp - 8], 1
1415adb24: 0f8406ffffff  je 0x1415ada30
1415adb2a: f30f1045fc  movss xmm0, dword ptr [rbp - 4]
1415adb2f: eb93  jmp 0x1415adac4
1415adb31: 488d1518c56c01  lea rdx, [rip + 0x16cc518]
1415adb38: 4c8d0521c56c01  lea r8, [rip + 0x16cc521]
1415adb3f: 4c89f1  mov rcx, r14
1415adb42: e8d9c13401  call 0x1428f9d20
1415adb47: b901000000  mov ecx, 1
1415adb4c: eb94  jmp 0x1415adae2
1415adb4e: 6690  nop 
1415adb50: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415adb55: 55  push rbp
1415adb56: 4157  push r15
1415adb58: 4156  push r14
1415adb5a: 4154  push r12
1415adb5c: 56  push rsi
1415adb5d: 57  push rdi
1415adb5e: 53  push rbx
1415adb5f: 4883ec30  sub rsp, 0x30
1415adb63: 488daa80000000  lea rbp, [rdx + 0x80]
1415adb6a: 488d4dd8  lea rcx, [rbp - 0x28]
1415adb6e: e8edc92dff  call 0x14088a560
1415adb73: 90  nop 
1415adb74: 4883c430  add rsp, 0x30
1415adb78: 5b  pop rbx
1415adb79: 5f  pop rdi
1415adb7a: 5e  pop rsi
1415adb7b: 415c  pop r12
1415adb7d: 415e  pop r14
1415adb7f: 415f  pop r15
1415adb81: 5d  pop rbp
1415adb82: c3  ret 
1415adb83: 666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415adb90: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415adb95: 55  push rbp
1415adb96: 4157  push r15
1415adb98: 4156  push r14
1415adb9a: 4154  push r12
1415adb9c: 56  push rsi
1415adb9d: 57  push rdi
1415adb9e: 53  push rbx
1415adb9f: 4883ec30  sub rsp, 0x30
1415adba3: 488daa80000000  lea rbp, [rdx + 0x80]
1415adbaa: 488b55e8  mov rdx, qword ptr [rbp - 0x18]
1415adbae: 4885d2  test rdx, rdx
1415adbb1: 7413  je 0x1415adbc6
1415adbb3: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
1415adbb7: 48c1e205  shl rdx, 5
1415adbbb: 41b808000000  mov r8d, 8
1415adbc1: e85a6006ff  call 0x140613c20
1415adbc6: 90  nop 
1415adbc7: 4883c430  add rsp, 0x30
1415adbcb: 5b  pop rbx
1415adbcc: 5f  pop rdi
1415adbcd: 5e  pop rsi
1415adbce: 415c  pop r12
1415adbd0: 415e  pop r14
1415adbd2: 415f  pop r15
1415adbd4: 5d  pop rbp
1415adbd5: c3  ret 
1415adbd6: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415adbe0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415adbe5: 55  push rbp
1415adbe6: 4157  push r15
1415adbe8: 4156  push r14
1415adbea: 4154  push r12
1415adbec: 56  push rsi
1415adbed: 57  push rdi
1415adbee: 53  push rbx
1415adbef: 4883ec30  sub rsp, 0x30
1415adbf3: 488daa80000000  lea rbp, [rdx + 0x80]
1415adbfa: 488b55e8  mov rdx, qword ptr [rbp - 0x18]
1415adbfe: 4885d2  test rdx, rdx
1415adc01: 7413  je 0x1415adc16
1415adc03: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
1415adc07: 48c1e205  shl rdx, 5
1415adc0b: 41b808000000  mov r8d, 8
1415adc11: e80a6006ff  call 0x140613c20
1415adc16: 90  nop 
1415adc17: 4883c430  add rsp, 0x30
1415adc1b: 5b  pop rbx
1415adc1c: 5f  pop rdi
1415adc1d: 5e  pop rsi
1415adc1e: 415c  pop r12
1415adc20: 415e  pop r14
1415adc22: 415f  pop r15
1415adc24: 5d  pop rbp
1415adc25: c3  ret 
