140a4a8a0: 55  push rbp
140a4a8a1: 4157  push r15
140a4a8a3: 4156  push r14
140a4a8a5: 4154  push r12
140a4a8a7: 56  push rsi
140a4a8a8: 57  push rdi
140a4a8a9: 53  push rbx
140a4a8aa: 4881ec40010000  sub rsp, 0x140
140a4a8b1: 488dac2480000000  lea rbp, [rsp + 0x80]
140a4a8b9: 48c785b8000000feffffff  mov qword ptr [rbp + 0xb8], 0xfffffffffffffffe
140a4a8c4: 4889ce  mov rsi, rcx
140a4a8c7: 0fb602  movzx eax, byte ptr [rdx]
140a4a8ca: 83f803  cmp eax, 3
140a4a8cd: 8985a8000000  mov dword ptr [rbp + 0xa8], eax
140a4a8d3: 48895518  mov qword ptr [rbp + 0x18], rdx
140a4a8d7: 0f84ed000000  je 0x140a4a9ca
140a4a8dd: 83f805  cmp eax, 5
140a4a8e0: 0f8553010000  jne 0x140a4aa39
140a4a8e6: 4c8b7a08  mov r15, qword ptr [rdx + 8]
140a4a8ea: 4c8b6210  mov r12, qword ptr [rdx + 0x10]
140a4a8ee: 488b7a18  mov rdi, qword ptr [rdx + 0x18]
140a4a8f2: 31c0  xor eax, eax
140a4a8f4: 4d85ff  test r15, r15
140a4a8f7: 0f95c0  setne al
140a4a8fa: 4889f9  mov rcx, rdi
140a4a8fd: 490f44cf  cmove rcx, r15
140a4a901: 488945b0  mov qword ptr [rbp - 0x50], rax
140a4a905: 48c745b800000000  mov qword ptr [rbp - 0x48], 0
140a4a90d: 4c897dc0  mov qword ptr [rbp - 0x40], r15
140a4a911: 4c8965c8  mov qword ptr [rbp - 0x38], r12
140a4a915: 488945d0  mov qword ptr [rbp - 0x30], rax
140a4a919: 48c745d800000000  mov qword ptr [rbp - 0x28], 0
140a4a921: 4c897de0  mov qword ptr [rbp - 0x20], r15
140a4a925: 4c8965e8  mov qword ptr [rbp - 0x18], r12
140a4a929: 48894df0  mov qword ptr [rbp - 0x10], rcx
140a4a92d: 488d8d80000000  lea rcx, [rbp + 0x80]
140a4a934: 488d55b0  lea rdx, [rbp - 0x50]
140a4a938: e8131b9f00  call 0x14143c450
140a4a93d: 488b8580000000  mov rax, qword ptr [rbp + 0x80]
140a4a944: 4885c0  test rax, rax
140a4a947: 7444  je 0x140a4a98d
140a4a949: 488b8d90000000  mov rcx, qword ptr [rbp + 0x90]
140a4a950: 488d1449  lea rdx, [rcx + rcx*2]
140a4a954: 4c8b84d068010000  mov r8, qword ptr [rax + rdx*8 + 0x168]
140a4a95c: 4c8b8cd070010000  mov r9, qword ptr [rax + rdx*8 + 0x170]
140a4a964: 488b9cd078010000  mov rbx, qword ptr [rax + rdx*8 + 0x178]
140a4a96c: 48c1e105  shl rcx, 5
140a4a970: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
140a4a974: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
140a4a979: 0f294520  movaps xmmword ptr [rbp + 0x20], xmm0
140a4a97d: 0f294d30  movaps xmmword ptr [rbp + 0x30], xmm1
140a4a981: 4c89c0  mov rax, r8
140a4a984: 48f7d8  neg rax
140a4a987: 0f8198010000  jno 0x140a4ab25
140a4a98d: c685800000000b  mov byte ptr [rbp + 0x80], 0xb
140a4a994: 488d15ad3a0802  lea rdx, [rip + 0x2083aad]
140a4a99b: 4c8d05b63a0802  lea r8, [rip + 0x2083ab6]
140a4a9a2: 488d8d80000000  lea rcx, [rbp + 0x80]
140a4a9a9: e892f1ea01  call 0x1428f9b40
140a4a9ae: 488906  mov qword ptr [rsi], rax
140a4a9b1: c6462205  mov byte ptr [rsi + 0x22], 5
140a4a9b5: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a4a9bc: 488d4db0  lea rcx, [rbp - 0x50]
140a4a9c0: e82bd4e3ff  call 0x140887df0
140a4a9c5: e949050000  jmp 0x140a4af13
140a4a9ca: 4c8b7a08  mov r15, qword ptr [rdx + 8]
140a4a9ce: 4c8b6210  mov r12, qword ptr [rdx + 0x10]
140a4a9d2: 488b7a18  mov rdi, qword ptr [rdx + 0x18]
140a4a9d6: 41b606  mov r14b, 6
140a4a9d9: 4c89f8  mov rax, r15
140a4a9dc: 4c89e1  mov rcx, r12
140a4a9df: 4889fb  mov rbx, rdi
140a4a9e2: 488945b0  mov qword ptr [rbp - 0x50], rax
140a4a9e6: 48894db8  mov qword ptr [rbp - 0x48], rcx
140a4a9ea: 48895dc0  mov qword ptr [rbp - 0x40], rbx
140a4a9ee: 0f284560  movaps xmm0, xmmword ptr [rbp + 0x60]
140a4a9f2: 0f1145c9  movups xmmword ptr [rbp - 0x37], xmm0
140a4a9f6: 0f10456f  movups xmm0, xmmword ptr [rbp + 0x6f]
140a4a9fa: 0f1145d8  movups xmmword ptr [rbp - 0x28], xmm0
140a4a9fe: 448875c8  mov byte ptr [rbp - 0x38], r14b
140a4aa02: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a4aa09: 488d4d20  lea rcx, [rbp + 0x20]
140a4aa0d: 488d55b0  lea rdx, [rbp - 0x50]
140a4aa11: e82a94b800  call 0x1415d3e40
140a4aa16: 0fb65520  movzx edx, byte ptr [rbp + 0x20]
140a4aa1a: 488b4528  mov rax, qword ptr [rbp + 0x28]
140a4aa1e: 83fa04  cmp edx, 4
140a4aa21: 0f859f000000  jne 0x140a4aac6
140a4aa27: 488906  mov qword ptr [rsi], rax
140a4aa2a: c6462205  mov byte ptr [rsi + 0x22], 5
140a4aa2e: 8b8da8000000  mov ecx, dword ptr [rbp + 0xa8]
140a4aa34: e99b040000  jmp 0x140a4aed4
140a4aa39: 0f1002  movups xmm0, xmmword ptr [rdx]
140a4aa3c: 0f104a10  movups xmm1, xmmword ptr [rdx + 0x10]
140a4aa40: 0f294dc0  movaps xmmword ptr [rbp - 0x40], xmm1
140a4aa44: 0f2945b0  movaps xmmword ptr [rbp - 0x50], xmm0
140a4aa48: 488d4d20  lea rcx, [rbp + 0x20]
140a4aa4c: 488d55b0  lea rdx, [rbp - 0x50]
140a4aa50: e8bbf1e701  call 0x1428c9c10
140a4aa55: 488d152c3a0802  lea rdx, [rip + 0x2083a2c]
140a4aa5c: 4c8d05f5390802  lea r8, [rip + 0x20839f5]
140a4aa63: 488d4d20  lea rcx, [rbp + 0x20]
140a4aa67: e894efe701  call 0x1428c9a00
140a4aa6c: 488906  mov qword ptr [rsi], rax
140a4aa6f: c6462205  mov byte ptr [rsi + 0x22], 5
140a4aa73: 0fb645b0  movzx eax, byte ptr [rbp - 0x50]
140a4aa77: 83f803  cmp eax, 3
140a4aa7a: 0f8293040000  jb 0x140a4af13
140a4aa80: 0f8457010000  je 0x140a4abdd
140a4aa86: 83f804  cmp eax, 4
140a4aa89: 0f856a010000  jne 0x140a4abf9
140a4aa8f: 488b4dc0  mov rcx, qword ptr [rbp - 0x40]
140a4aa93: 488b55c8  mov rdx, qword ptr [rbp - 0x38]
140a4aa97: 48898da0000000  mov qword ptr [rbp + 0xa0], rcx
140a4aa9e: e8cde2e3ff  call 0x140888d70
140a4aaa3: 488b55b8  mov rdx, qword ptr [rbp - 0x48]
140a4aaa7: 4885d2  test rdx, rdx
140a4aaaa: 0f8463040000  je 0x140a4af13
140a4aab0: 48c1e205  shl rdx, 5
140a4aab4: 41b808000000  mov r8d, 8
140a4aaba: 488b8da0000000  mov rcx, qword ptr [rbp + 0xa0]
140a4aac1: e948040000  jmp 0x140a4af0e
140a4aac6: 488b4d40  mov rcx, qword ptr [rbp + 0x40]
140a4aaca: 48894d10  mov qword ptr [rbp + 0x10], rcx
140a4aace: 0f104530  movups xmm0, xmmword ptr [rbp + 0x30]
140a4aad2: 0f294500  movaps xmmword ptr [rbp], xmm0
140a4aad6: 0fb6c8  movzx ecx, al
140a4aad9: 89d2  mov edx, edx
140a4aadb: 4c8d056a4d0802  lea r8, [rip + 0x2084d6a]
140a4aae2: 49631490  movsxd rdx, dword ptr [r8 + rdx*4]
140a4aae6: 4c01c2  add rdx, r8
140a4aae9: ffe2  jmp rdx
140a4aaeb: 83f906  cmp ecx, 6
140a4aaee: 0f85e9020000  jne 0x140a4addd
140a4aaf4: c645600d  mov byte ptr [rbp + 0x60], 0xd
140a4aaf8: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a4aaff: 488d15a2f72202  lea rdx, [rip + 0x222f7a2]
140a4ab06: 4c8d0553f52202  lea r8, [rip + 0x222f553]
140a4ab0d: 488d4d60  lea rcx, [rbp + 0x60]
140a4ab11: e8eaeee701  call 0x1428c9a00
140a4ab16: 48894520  mov qword ptr [rbp + 0x20], rax
140a4ab1a: 8b8da8000000  mov ecx, dword ptr [rbp + 0xa8]
140a4ab20: e91a030000  jmp 0x140a4ae3f
140a4ab25: 4c894df8  mov qword ptr [rbp - 8], r9
140a4ab29: 4c8985a0000000  mov qword ptr [rbp + 0xa0], r8
140a4ab30: 0f284520  movaps xmm0, xmmword ptr [rbp + 0x20]
140a4ab34: 0f284d30  movaps xmm1, xmmword ptr [rbp + 0x30]
140a4ab38: 0f298d90000000  movaps xmmword ptr [rbp + 0x90], xmm1
140a4ab3f: 0f298580000000  movaps xmmword ptr [rbp + 0x80], xmm0
140a4ab46: 4889e9  mov rcx, rbp
140a4ab49: 488d55b0  lea rdx, [rbp - 0x50]
140a4ab4d: e8fe189f00  call 0x14143c450
140a4ab52: 488b4500  mov rax, qword ptr [rbp]
140a4ab56: 4885c0  test rax, rax
140a4ab59: 7445  je 0x140a4aba0
140a4ab5b: 488b4d10  mov rcx, qword ptr [rbp + 0x10]
140a4ab5f: 488d1449  lea rdx, [rcx + rcx*2]
140a4ab63: 4c8b84d078010000  mov r8, qword ptr [rax + rdx*8 + 0x178]
140a4ab6b: 4c894530  mov qword ptr [rbp + 0x30], r8
140a4ab6f: 0f1084d068010000  movups xmm0, xmmword ptr [rax + rdx*8 + 0x168]
140a4ab77: 0f294520  movaps xmmword ptr [rbp + 0x20], xmm0
140a4ab7b: 48c1e105  shl rcx, 5
140a4ab7f: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
140a4ab83: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
140a4ab88: 0f114538  movups xmmword ptr [rbp + 0x38], xmm0
140a4ab8c: 0f114d48  movups xmmword ptr [rbp + 0x48], xmm1
140a4ab90: 488b5520  mov rdx, qword ptr [rbp + 0x20]
140a4ab94: 4889d0  mov rax, rdx
140a4ab97: 48f7d8  neg rax
140a4ab9a: 0f8120010000  jno 0x140a4acc0
140a4aba0: 448ab580000000  mov r14b, byte ptr [rbp + 0x80]
140a4aba7: 0f108581000000  movups xmm0, xmmword ptr [rbp + 0x81]
140a4abae: 0f294560  movaps xmmword ptr [rbp + 0x60], xmm0
140a4abb2: 0f108590000000  movups xmm0, xmmword ptr [rbp + 0x90]
140a4abb9: 0f11456f  movups xmmword ptr [rbp + 0x6f], xmm0
140a4abbd: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a4abc4: 488d4db0  lea rcx, [rbp - 0x50]
140a4abc8: e823d2e3ff  call 0x140887df0
140a4abcd: 488b85a0000000  mov rax, qword ptr [rbp + 0xa0]
140a4abd4: 488b4df8  mov rcx, qword ptr [rbp - 8]
140a4abd8: e905feffff  jmp 0x140a4a9e2
140a4abdd: 488b55b8  mov rdx, qword ptr [rbp - 0x48]
140a4abe1: 4885d2  test rdx, rdx
140a4abe4: 0f8429030000  je 0x140a4af13
140a4abea: 488b4dc0  mov rcx, qword ptr [rbp - 0x40]
140a4abee: 41b801000000  mov r8d, 1
140a4abf4: e915030000  jmp 0x140a4af0e
140a4abf9: 488d4db8  lea rcx, [rbp - 0x48]
140a4abfd: c685b700000000  mov byte ptr [rbp + 0xb7], 0
140a4ac04: e837e3e4ff  call 0x140898f40
140a4ac09: e905030000  jmp 0x140a4af13
140a4ac0e: 83f906  cmp ecx, 6
140a4ac11: 0f85e4000000  jne 0x140a4acfb
140a4ac17: c685800000000d  mov byte ptr [rbp + 0x80], 0xd
140a4ac1e: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a4ac25: 488d157cf62202  lea rdx, [rip + 0x222f67c]
140a4ac2c: 4c8d052df42202  lea r8, [rip + 0x222f42d]
140a4ac33: 488d8d80000000  lea rcx, [rbp + 0x80]
140a4ac3a: e8c1ede701  call 0x1428c9a00
140a4ac3f: eb64  jmp 0x140a4aca5
140a4ac41: 83f906  cmp ecx, 6
140a4ac44: 0f8522010000  jne 0x140a4ad6c
140a4ac4a: c685800000000d  mov byte ptr [rbp + 0x80], 0xd
140a4ac51: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a4ac58: 488d1549f62202  lea rdx, [rip + 0x222f649]
140a4ac5f: 4c8d05faf32202  lea r8, [rip + 0x222f3fa]
140a4ac66: 488d8d80000000  lea rcx, [rbp + 0x80]
140a4ac6d: e88eede701  call 0x1428c9a00
140a4ac72: eb31  jmp 0x140a4aca5
140a4ac74: 83f906  cmp ecx, 6
140a4ac77: 0f85d2010000  jne 0x140a4ae4f
140a4ac7d: c685800000000d  mov byte ptr [rbp + 0x80], 0xd
140a4ac84: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a4ac8b: 488d1516f62202  lea rdx, [rip + 0x222f616]
140a4ac92: 4c8d05c7f32202  lea r8, [rip + 0x222f3c7]
140a4ac99: 488d8d80000000  lea rcx, [rbp + 0x80]
140a4aca0: e85bede701  call 0x1428c9a00
140a4aca5: 48894568  mov qword ptr [rbp + 0x68], rax
140a4aca9: 8b8da8000000  mov ecx, dword ptr [rbp + 0xa8]
140a4acaf: 488b4568  mov rax, qword ptr [rbp + 0x68]
140a4acb3: 488906  mov qword ptr [rsi], rax
140a4acb6: b005  mov al, 5
140a4acb8: 884622  mov byte ptr [rsi + 0x22], al
140a4acbb: e914020000  jmp 0x140a4aed4
140a4acc0: 730f  jae 0x140a4acd1
140a4acc2: 488b4d28  mov rcx, qword ptr [rbp + 0x28]
140a4acc6: 41b801000000  mov r8d, 1
140a4accc: e84f8fbcff  call 0x140613c20
140a4acd1: 0fb64538  movzx eax, byte ptr [rbp + 0x38]
140a4acd5: 83f803  cmp eax, 3
140a4acd8: 0f8269020000  jb 0x140a4af47
140a4acde: 0f8444020000  je 0x140a4af28
140a4ace4: 488d4d40  lea rcx, [rbp + 0x40]
140a4ace8: 83f804  cmp eax, 4
140a4aceb: 0f8551020000  jne 0x140a4af42
140a4acf1: e89ae6e3ff  call 0x140889390
140a4acf6: e94c020000  jmp 0x140a4af47
140a4acfb: 48894520  mov qword ptr [rbp + 0x20], rax
140a4acff: 0f284500  movaps xmm0, xmmword ptr [rbp]
140a4ad03: 0f114528  movups xmmword ptr [rbp + 0x28], xmm0
140a4ad07: 488b4510  mov rax, qword ptr [rbp + 0x10]
140a4ad0b: 48894538  mov qword ptr [rbp + 0x38], rax
140a4ad0f: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a4ad16: 488d0503480802  lea rax, [rip + 0x2084803]
140a4ad1d: 4889442420  mov qword ptr [rsp + 0x20], rax
140a4ad22: 48c744242802000000  mov qword ptr [rsp + 0x28], 2
140a4ad2b: 4c8d05c6470802  lea r8, [rip + 0x20847c6]
140a4ad32: 488d4d60  lea rcx, [rbp + 0x60]
140a4ad36: 488d5520  lea rdx, [rbp + 0x20]
140a4ad3a: 41b911000000  mov r9d, 0x11
140a4ad40: e8eb4db500  call 0x14159fb30
140a4ad45: f6456001  test byte ptr [rbp + 0x60], 1
140a4ad49: 8b8da8000000  mov ecx, dword ptr [rbp + 0xa8]
140a4ad4f: 0f855affffff  jne 0x140a4acaf
140a4ad55: 8b456c  mov eax, dword ptr [rbp + 0x6c]
140a4ad58: 894608  mov dword ptr [rsi + 8], eax
140a4ad5b: 488b4564  mov rax, qword ptr [rbp + 0x64]
140a4ad5f: 488906  mov qword ptr [rsi], rax
140a4ad62: b003  mov al, 3
140a4ad64: 884622  mov byte ptr [rsi + 0x22], al
140a4ad67: e968010000  jmp 0x140a4aed4
140a4ad6c: 48894520  mov qword ptr [rbp + 0x20], rax
140a4ad70: 0f284500  movaps xmm0, xmmword ptr [rbp]
140a4ad74: 0f114528  movups xmmword ptr [rbp + 0x28], xmm0
140a4ad78: 488b4510  mov rax, qword ptr [rbp + 0x10]
140a4ad7c: 48894538  mov qword ptr [rbp + 0x38], rax
140a4ad80: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a4ad87: 488d05f2470802  lea rax, [rip + 0x20847f2]
140a4ad8e: 4889442420  mov qword ptr [rsp + 0x20], rax
140a4ad93: 48c744242803000000  mov qword ptr [rsp + 0x28], 3
140a4ad9c: 4c8d05bd470802  lea r8, [rip + 0x20847bd]
140a4ada3: 488d4d60  lea rcx, [rbp + 0x60]
140a4ada7: 488d5520  lea rdx, [rbp + 0x20]
140a4adab: 41b90f000000  mov r9d, 0xf
140a4adb1: e88a89b500  call 0x1415a3740
140a4adb6: f6456001  test byte ptr [rbp + 0x60], 1
140a4adba: 8b8da8000000  mov ecx, dword ptr [rbp + 0xa8]
140a4adc0: 0f85e9feffff  jne 0x140a4acaf
140a4adc6: 8b4574  mov eax, dword ptr [rbp + 0x74]
140a4adc9: 894610  mov dword ptr [rsi + 0x10], eax
140a4adcc: 0f104564  movups xmm0, xmmword ptr [rbp + 0x64]
140a4add0: 0f1106  movups xmmword ptr [rsi], xmm0
140a4add3: b004  mov al, 4
140a4add5: 884622  mov byte ptr [rsi + 0x22], al
140a4add8: e9f7000000  jmp 0x140a4aed4
140a4addd: 48898580000000  mov qword ptr [rbp + 0x80], rax
140a4ade4: 0f284500  movaps xmm0, xmmword ptr [rbp]
140a4ade8: 0f118588000000  movups xmmword ptr [rbp + 0x88], xmm0
140a4adef: 488b4510  mov rax, qword ptr [rbp + 0x10]
140a4adf3: 48898598000000  mov qword ptr [rbp + 0x98], rax
140a4adfa: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a4ae01: 488d0560460802  lea rax, [rip + 0x2084660]
140a4ae08: 4889442420  mov qword ptr [rsp + 0x20], rax
140a4ae0d: 48c744242802000000  mov qword ptr [rsp + 0x28], 2
140a4ae16: 4c8d052b460802  lea r8, [rip + 0x208462b]
140a4ae1d: 488d4d20  lea rcx, [rbp + 0x20]
140a4ae21: 488d9580000000  lea rdx, [rbp + 0x80]
140a4ae28: 41b90e000000  mov r9d, 0xe
140a4ae2e: e8fd58b500  call 0x1415a0730
140a4ae33: 807d4202  cmp byte ptr [rbp + 0x42], 2
140a4ae37: 8b8da8000000  mov ecx, dword ptr [rbp + 0xa8]
140a4ae3d: 757e  jne 0x140a4aebd
140a4ae3f: 488b4520  mov rax, qword ptr [rbp + 0x20]
140a4ae43: 488906  mov qword ptr [rsi], rax
140a4ae46: c6462205  mov byte ptr [rsi + 0x22], 5
140a4ae4a: e985000000  jmp 0x140a4aed4
140a4ae4f: 48894520  mov qword ptr [rbp + 0x20], rax
140a4ae53: 0f284500  movaps xmm0, xmmword ptr [rbp]
140a4ae57: 0f114528  movups xmmword ptr [rbp + 0x28], xmm0
140a4ae5b: 488b4510  mov rax, qword ptr [rbp + 0x10]
140a4ae5f: 48894538  mov qword ptr [rbp + 0x38], rax
140a4ae63: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a4ae6a: 488d0567460802  lea rax, [rip + 0x2084667]
140a4ae71: 4889442420  mov qword ptr [rsp + 0x20], rax
140a4ae76: 48c744242802000000  mov qword ptr [rsp + 0x28], 2
140a4ae7f: 4c8d0522460802  lea r8, [rip + 0x2084622]
140a4ae86: 488d4d60  lea rcx, [rbp + 0x60]
140a4ae8a: 488d5520  lea rdx, [rbp + 0x20]
140a4ae8e: 41b90e000000  mov r9d, 0xe
140a4ae94: e8a77ab500  call 0x1415a2940
140a4ae99: f6456001  test byte ptr [rbp + 0x60], 1
140a4ae9d: 8b8da8000000  mov ecx, dword ptr [rbp + 0xa8]
140a4aea3: 0f8506feffff  jne 0x140a4acaf
140a4aea9: 8b456c  mov eax, dword ptr [rbp + 0x6c]
140a4aeac: 894608  mov dword ptr [rsi + 8], eax
140a4aeaf: 488b4564  mov rax, qword ptr [rbp + 0x64]
140a4aeb3: 488906  mov qword ptr [rsi], rax
140a4aeb6: b002  mov al, 2
140a4aeb8: 884622  mov byte ptr [rsi + 0x22], al
140a4aebb: eb17  jmp 0x140a4aed4
140a4aebd: 488b4540  mov rax, qword ptr [rbp + 0x40]
140a4aec1: 48894620  mov qword ptr [rsi + 0x20], rax
140a4aec5: 0f104520  movups xmm0, xmmword ptr [rbp + 0x20]
140a4aec9: 0f104d30  movups xmm1, xmmword ptr [rbp + 0x30]
140a4aecd: 0f114e10  movups xmmword ptr [rsi + 0x10], xmm1
140a4aed1: 0f1106  movups xmmword ptr [rsi], xmm0
140a4aed4: 83f903  cmp ecx, 3
140a4aed7: 743a  je 0x140a4af13
140a4aed9: 83f905  cmp ecx, 5
140a4aedc: 7435  je 0x140a4af13
140a4aede: 4c89bda8000000  mov qword ptr [rbp + 0xa8], r15
140a4aee5: 4c896518  mov qword ptr [rbp + 0x18], r12
140a4aee9: 4c89e1  mov rcx, r12
140a4aeec: 4889fa  mov rdx, rdi
140a4aeef: e87cdee3ff  call 0x140888d70
140a4aef4: 488b95a8000000  mov rdx, qword ptr [rbp + 0xa8]
140a4aefb: 4885d2  test rdx, rdx
140a4aefe: 488b4d18  mov rcx, qword ptr [rbp + 0x18]
140a4af02: 740f  je 0x140a4af13
140a4af04: 48c1e205  shl rdx, 5
140a4af08: 41b808000000  mov r8d, 8
140a4af0e: e80d8dbcff  call 0x140613c20
140a4af13: 4889f0  mov rax, rsi
140a4af16: 4881c440010000  add rsp, 0x140
140a4af1d: 5b  pop rbx
140a4af1e: 5f  pop rdi
140a4af1f: 5e  pop rsi
140a4af20: 415c  pop r12
140a4af22: 415e  pop r14
140a4af24: 415f  pop r15
140a4af26: 5d  pop rbp
140a4af27: c3  ret 
140a4af28: 488b5540  mov rdx, qword ptr [rbp + 0x40]
140a4af2c: 4885d2  test rdx, rdx
140a4af2f: 7416  je 0x140a4af47
140a4af31: 488b4d48  mov rcx, qword ptr [rbp + 0x48]
140a4af35: 41b801000000  mov r8d, 1
140a4af3b: e8e08cbcff  call 0x140613c20
140a4af40: eb05  jmp 0x140a4af47
140a4af42: e8f9dfe4ff  call 0x140898f40
140a4af47: c645200b  mov byte ptr [rbp + 0x20], 0xb
140a4af4b: 488d15f6340802  lea rdx, [rip + 0x20834f6]
140a4af52: 4c8d05ff340802  lea r8, [rip + 0x20834ff]
140a4af59: 488d4d20  lea rcx, [rbp + 0x20]
140a4af5d: e8deebea01  call 0x1428f9b40
140a4af62: 488906  mov qword ptr [rsi], rax
140a4af65: c6462205  mov byte ptr [rsi + 0x22], 5
140a4af69: 0fb68580000000  movzx eax, byte ptr [rbp + 0x80]
140a4af70: 83f803  cmp eax, 3
140a4af73: 723a  jb 0x140a4afaf
140a4af75: 7413  je 0x140a4af8a
140a4af77: 488d8d88000000  lea rcx, [rbp + 0x88]
140a4af7e: 83f804  cmp eax, 4
140a4af81: 7527  jne 0x140a4afaa
140a4af83: e808e4e3ff  call 0x140889390
140a4af88: eb25  jmp 0x140a4afaf
140a4af8a: 488b9588000000  mov rdx, qword ptr [rbp + 0x88]
140a4af91: 4885d2  test rdx, rdx
140a4af94: 7419  je 0x140a4afaf
140a4af96: 488b8d90000000  mov rcx, qword ptr [rbp + 0x90]
140a4af9d: 41b801000000  mov r8d, 1
140a4afa3: e8788cbcff  call 0x140613c20
140a4afa8: eb05  jmp 0x140a4afaf
140a4afaa: e891dfe4ff  call 0x140898f40
140a4afaf: 488b95a0000000  mov rdx, qword ptr [rbp + 0xa0]
140a4afb6: 4885d2  test rdx, rdx
140a4afb9: 0f84f6f9ffff  je 0x140a4a9b5
140a4afbf: 41b801000000  mov r8d, 1
140a4afc5: 488b4df8  mov rcx, qword ptr [rbp - 8]
140a4afc9: e8528cbcff  call 0x140613c20
140a4afce: e9e2f9ffff  jmp 0x140a4a9b5
140a4afd3: 666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
140a4afe0: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a4afe5: 55  push rbp
140a4afe6: 4157  push r15
140a4afe8: 4156  push r14
140a4afea: 4154  push r12
140a4afec: 56  push rsi
140a4afed: 57  push rdi
140a4afee: 53  push rbx
140a4afef: 4883ec30  sub rsp, 0x30
140a4aff3: 488daa80000000  lea rbp, [rdx + 0x80]
140a4affa: 488d4db0  lea rcx, [rbp - 0x50]
140a4affe: e8edcde3ff  call 0x140887df0
140a4b003: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a4b00a: 4883c430  add rsp, 0x30
140a4b00e: 5b  pop rbx
140a4b00f: 5f  pop rdi
140a4b010: 5e  pop rsi
140a4b011: 415c  pop r12
140a4b013: 415e  pop r14
140a4b015: 415f  pop r15
140a4b017: 5d  pop rbp
140a4b018: c3  ret 
140a4b019: 0f1f8000000000  nop dword ptr [rax]
140a4b020: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a4b025: 55  push rbp
140a4b026: 4157  push r15
140a4b028: 4156  push r14
140a4b02a: 4154  push r12
140a4b02c: 56  push rsi
140a4b02d: 57  push rdi
140a4b02e: 53  push rbx
140a4b02f: 4883ec30  sub rsp, 0x30
140a4b033: 488daa80000000  lea rbp, [rdx + 0x80]
140a4b03a: 4883bda000000000  cmp qword ptr [rbp + 0xa0], 0
140a4b042: 7416  je 0x140a4b05a
140a4b044: 41b801000000  mov r8d, 1
140a4b04a: 488b4df8  mov rcx, qword ptr [rbp - 8]
140a4b04e: 488b95a0000000  mov rdx, qword ptr [rbp + 0xa0]
140a4b055: e8c68bbcff  call 0x140613c20
140a4b05a: 90  nop 
140a4b05b: 4883c430  add rsp, 0x30
140a4b05f: 5b  pop rbx
140a4b060: 5f  pop rdi
140a4b061: 5e  pop rsi
140a4b062: 415c  pop r12
140a4b064: 415e  pop r14
140a4b066: 415f  pop r15
140a4b068: 5d  pop rbp
140a4b069: c3  ret 
140a4b06a: 660f1f440000  nop word ptr [rax + rax]
140a4b070: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a4b075: 55  push rbp
140a4b076: 4157  push r15
140a4b078: 4156  push r14
140a4b07a: 4154  push r12
140a4b07c: 56  push rsi
140a4b07d: 57  push rdi
140a4b07e: 53  push rbx
140a4b07f: 4883ec30  sub rsp, 0x30
140a4b083: 488daa80000000  lea rbp, [rdx + 0x80]
140a4b08a: 4883bda800000000  cmp qword ptr [rbp + 0xa8], 0
140a4b092: 741a  je 0x140a4b0ae
140a4b094: 488b95a8000000  mov rdx, qword ptr [rbp + 0xa8]
140a4b09b: 48c1e205  shl rdx, 5
140a4b09f: 41b808000000  mov r8d, 8
140a4b0a5: 488b4d18  mov rcx, qword ptr [rbp + 0x18]
140a4b0a9: e8728bbcff  call 0x140613c20
140a4b0ae: 90  nop 
140a4b0af: 4883c430  add rsp, 0x30
140a4b0b3: 5b  pop rbx
140a4b0b4: 5f  pop rdi
140a4b0b5: 5e  pop rsi
140a4b0b6: 415c  pop r12
140a4b0b8: 415e  pop r14
140a4b0ba: 415f  pop r15
140a4b0bc: 5d  pop rbp
140a4b0bd: c3  ret 
140a4b0be: 6690  nop 
140a4b0c0: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a4b0c5: 55  push rbp
140a4b0c6: 4157  push r15
140a4b0c8: 4156  push r14
140a4b0ca: 4154  push r12
140a4b0cc: 56  push rsi
140a4b0cd: 57  push rdi
140a4b0ce: 53  push rbx
140a4b0cf: 4883ec30  sub rsp, 0x30
140a4b0d3: 488daa80000000  lea rbp, [rdx + 0x80]
140a4b0da: 488d4db0  lea rcx, [rbp - 0x50]
140a4b0de: e8ad29e1ff  call 0x14085da90
140a4b0e3: c685b700000000  mov byte ptr [rbp + 0xb7], 0
140a4b0ea: 4883c430  add rsp, 0x30
140a4b0ee: 5b  pop rbx
140a4b0ef: 5f  pop rdi
140a4b0f0: 5e  pop rsi
140a4b0f1: 415c  pop r12
140a4b0f3: 415e  pop r14
140a4b0f5: 415f  pop r15
140a4b0f7: 5d  pop rbp
140a4b0f8: c3  ret 
140a4b0f9: 0f1f8000000000  nop dword ptr [rax]
140a4b100: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a4b105: 55  push rbp
140a4b106: 4157  push r15
140a4b108: 4156  push r14
140a4b10a: 4154  push r12
140a4b10c: 56  push rsi
140a4b10d: 57  push rdi
140a4b10e: 53  push rbx
140a4b10f: 4883ec30  sub rsp, 0x30
140a4b113: 488daa80000000  lea rbp, [rdx + 0x80]
140a4b11a: 488b55b8  mov rdx, qword ptr [rbp - 0x48]
140a4b11e: 4885d2  test rdx, rdx
140a4b121: 7416  je 0x140a4b139
140a4b123: 48c1e205  shl rdx, 5
140a4b127: 41b808000000  mov r8d, 8
140a4b12d: 488b8da0000000  mov rcx, qword ptr [rbp + 0xa0]
140a4b134: e8e78abcff  call 0x140613c20
140a4b139: c685b700000000  mov byte ptr [rbp + 0xb7], 0
140a4b140: 4883c430  add rsp, 0x30
140a4b144: 5b  pop rbx
140a4b145: 5f  pop rdi
140a4b146: 5e  pop rsi
140a4b147: 415c  pop r12
140a4b149: 415e  pop r14
140a4b14b: 415f  pop r15
140a4b14d: 5d  pop rbp
140a4b14e: c3  ret 
140a4b14f: 90  nop 
140a4b150: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a4b155: 55  push rbp
140a4b156: 4157  push r15
140a4b158: 4156  push r14
140a4b15a: 4154  push r12
140a4b15c: 56  push rsi
140a4b15d: 57  push rdi
140a4b15e: 53  push rbx
140a4b15f: 4883ec30  sub rsp, 0x30
140a4b163: 488daa80000000  lea rbp, [rdx + 0x80]
140a4b16a: 80bdb700000000  cmp byte ptr [rbp + 0xb7], 0
140a4b171: 7415  je 0x140a4b188
140a4b173: 80bda800000003  cmp byte ptr [rbp + 0xa8], 3
140a4b17a: 740c  je 0x140a4b188
140a4b17c: 0fb685a8000000  movzx eax, byte ptr [rbp + 0xa8]
140a4b183: 83f805  cmp eax, 5
140a4b186: 750f  jne 0x140a4b197
140a4b188: 4883c430  add rsp, 0x30
140a4b18c: 5b  pop rbx
140a4b18d: 5f  pop rdi
140a4b18e: 5e  pop rsi
140a4b18f: 415c  pop r12
140a4b191: 415e  pop r14
140a4b193: 415f  pop r15
140a4b195: 5d  pop rbp
140a4b196: c3  ret 
140a4b197: 488b4d18  mov rcx, qword ptr [rbp + 0x18]
140a4b19b: e8f028e1ff  call 0x14085da90
140a4b1a0: ebe6  jmp 0x140a4b188
140a4b1a2: 66666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
140a4b1b0: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a4b1b5: 55  push rbp
140a4b1b6: 4157  push r15
140a4b1b8: 4156  push r14
140a4b1ba: 4154  push r12
140a4b1bc: 56  push rsi
140a4b1bd: 57  push rdi
140a4b1be: 53  push rbx
140a4b1bf: 4883ec30  sub rsp, 0x30
140a4b1c3: 488daa80000000  lea rbp, [rdx + 0x80]
140a4b1ca: 488d8d80000000  lea rcx, [rbp + 0x80]
140a4b1d1: e8ba28e1ff  call 0x14085da90
140a4b1d6: 90  nop 
140a4b1d7: 4883c430  add rsp, 0x30
140a4b1db: 5b  pop rbx
140a4b1dc: 5f  pop rdi
140a4b1dd: 5e  pop rsi
140a4b1de: 415c  pop r12
140a4b1e0: 415e  pop r14
140a4b1e2: 415f  pop r15
140a4b1e4: 5d  pop rbp
140a4b1e5: c3  ret 
