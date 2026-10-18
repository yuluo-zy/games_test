1416fa9e0: 55  push rbp
1416fa9e1: 4157  push r15
1416fa9e3: 4156  push r14
1416fa9e5: 4154  push r12
1416fa9e7: 56  push rsi
1416fa9e8: 57  push rdi
1416fa9e9: 53  push rbx
1416fa9ea: 4881ec40010000  sub rsp, 0x140
1416fa9f1: 488dac2480000000  lea rbp, [rsp + 0x80]
1416fa9f9: 48c785b8000000feffffff  mov qword ptr [rbp + 0xb8], 0xfffffffffffffffe
1416faa04: 4889ce  mov rsi, rcx
1416faa07: 0fb602  movzx eax, byte ptr [rdx]
1416faa0a: 83f803  cmp eax, 3
1416faa0d: 8985a0000000  mov dword ptr [rbp + 0xa0], eax
1416faa13: 48895578  mov qword ptr [rbp + 0x78], rdx
1416faa17: 0f84ed000000  je 0x1416fab0a
1416faa1d: 83f805  cmp eax, 5
1416faa20: 0f85cf010000  jne 0x1416fabf5
1416faa26: 4c8b7a08  mov r15, qword ptr [rdx + 8]
1416faa2a: 4c8b6210  mov r12, qword ptr [rdx + 0x10]
1416faa2e: 488b7a18  mov rdi, qword ptr [rdx + 0x18]
1416faa32: 31c0  xor eax, eax
1416faa34: 4d85ff  test r15, r15
1416faa37: 0f95c0  setne al
1416faa3a: 4889f9  mov rcx, rdi
1416faa3d: 490f44cf  cmove rcx, r15
1416faa41: 48894520  mov qword ptr [rbp + 0x20], rax
1416faa45: 48c7452800000000  mov qword ptr [rbp + 0x28], 0
1416faa4d: 4c897d30  mov qword ptr [rbp + 0x30], r15
1416faa51: 4c896538  mov qword ptr [rbp + 0x38], r12
1416faa55: 48894540  mov qword ptr [rbp + 0x40], rax
1416faa59: 48c7454800000000  mov qword ptr [rbp + 0x48], 0
1416faa61: 4c897d50  mov qword ptr [rbp + 0x50], r15
1416faa65: 4c896558  mov qword ptr [rbp + 0x58], r12
1416faa69: 48894d60  mov qword ptr [rbp + 0x60], rcx
1416faa6d: 488d8d80000000  lea rcx, [rbp + 0x80]
1416faa74: 488d5520  lea rdx, [rbp + 0x20]
1416faa78: e8d319d4ff  call 0x14143c450
1416faa7d: 488b8580000000  mov rax, qword ptr [rbp + 0x80]
1416faa84: 4885c0  test rax, rax
1416faa87: 7444  je 0x1416faacd
1416faa89: 488b8d90000000  mov rcx, qword ptr [rbp + 0x90]
1416faa90: 488d1449  lea rdx, [rcx + rcx*2]
1416faa94: 4c8b84d068010000  mov r8, qword ptr [rax + rdx*8 + 0x168]
1416faa9c: 4c8b8cd070010000  mov r9, qword ptr [rax + rdx*8 + 0x170]
1416faaa4: 488b9cd078010000  mov rbx, qword ptr [rax + rdx*8 + 0x178]
1416faaac: 48c1e105  shl rcx, 5
1416faab0: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
1416faab4: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
1416faab9: 0f2945e0  movaps xmmword ptr [rbp - 0x20], xmm0
1416faabd: 0f294df0  movaps xmmword ptr [rbp - 0x10], xmm1
1416faac1: 4c89c0  mov rax, r8
1416faac4: 48f7d8  neg rax
1416faac7: 0f81b5010000  jno 0x1416fac82
1416faacd: c685800000000b  mov byte ptr [rbp + 0x80], 0xb
1416faad4: 488d15256f5901  lea rdx, [rip + 0x1596f25]
1416faadb: 4c8d052e6f5901  lea r8, [rip + 0x1596f2e]
1416faae2: 488d8d80000000  lea rcx, [rbp + 0x80]
1416faae9: e852f01f01  call 0x1428f9b40
1416faaee: 48894608  mov qword ptr [rsi + 8], rax
1416faaf2: c60601  mov byte ptr [rsi], 1
1416faaf5: c685b700000001  mov byte ptr [rbp + 0xb7], 1
1416faafc: 488d4d20  lea rcx, [rbp + 0x20]
1416fab00: e8ebd218ff  call 0x140887df0
1416fab05: e973020000  jmp 0x1416fad7d
1416fab0a: 4c8b7a08  mov r15, qword ptr [rdx + 8]
1416fab0e: 4c8b6210  mov r12, qword ptr [rdx + 0x10]
1416fab12: 488b7a18  mov rdi, qword ptr [rdx + 0x18]
1416fab16: 41b606  mov r14b, 6
1416fab19: 4c89f8  mov rax, r15
1416fab1c: 4c89e1  mov rcx, r12
1416fab1f: 4889fb  mov rbx, rdi
1416fab22: 48894520  mov qword ptr [rbp + 0x20], rax
1416fab26: 48894d28  mov qword ptr [rbp + 0x28], rcx
1416fab2a: 48895d30  mov qword ptr [rbp + 0x30], rbx
1416fab2e: 0f2845c0  movaps xmm0, xmmword ptr [rbp - 0x40]
1416fab32: 0f114539  movups xmmword ptr [rbp + 0x39], xmm0
1416fab36: 0f1045cf  movups xmm0, xmmword ptr [rbp - 0x31]
1416fab3a: 0f114548  movups xmmword ptr [rbp + 0x48], xmm0
1416fab3e: 44887538  mov byte ptr [rbp + 0x38], r14b
1416fab42: c685b700000001  mov byte ptr [rbp + 0xb7], 1
1416fab49: 488d4de0  lea rcx, [rbp - 0x20]
1416fab4d: 488d5520  lea rdx, [rbp + 0x20]
1416fab51: e81a91edff  call 0x1415d3c70
1416fab56: 0fb65de0  movzx ebx, byte ptr [rbp - 0x20]
1416fab5a: 488b45e8  mov rax, qword ptr [rbp - 0x18]
1416fab5e: 80fb02  cmp bl, 2
1416fab61: 7439  je 0x1416fab9c
1416fab63: 488b4d00  mov rcx, qword ptr [rbp]
1416fab67: 48898d98000000  mov qword ptr [rbp + 0x98], rcx
1416fab6e: 0f1045f0  movups xmm0, xmmword ptr [rbp - 0x10]
1416fab72: 0f118588000000  movups xmmword ptr [rbp + 0x88], xmm0
1416fab79: 48898580000000  mov qword ptr [rbp + 0x80], rax
1416fab80: c685b700000001  mov byte ptr [rbp + 0xb7], 1
1416fab87: 488d8d80000000  lea rcx, [rbp + 0x80]
1416fab8e: e86dd448ff  call 0x140b88000
1416fab93: 4885c0  test rax, rax
1416fab96: 0f849f010000  je 0x1416fad3b
1416fab9c: 48894608  mov qword ptr [rsi + 8], rax
1416faba0: b001  mov al, 1
1416faba2: 8806  mov byte ptr [rsi], al
1416faba4: 8b85a0000000  mov eax, dword ptr [rbp + 0xa0]
1416fabaa: 83f803  cmp eax, 3
1416fabad: 0f84ca010000  je 0x1416fad7d
1416fabb3: 83f805  cmp eax, 5
1416fabb6: 0f84c1010000  je 0x1416fad7d
1416fabbc: 4c89bda0000000  mov qword ptr [rbp + 0xa0], r15
1416fabc3: 4c896578  mov qword ptr [rbp + 0x78], r12
1416fabc7: 4c89e1  mov rcx, r12
1416fabca: 4889fa  mov rdx, rdi
1416fabcd: e89ee118ff  call 0x140888d70
1416fabd2: 488b95a0000000  mov rdx, qword ptr [rbp + 0xa0]
1416fabd9: 4885d2  test rdx, rdx
1416fabdc: 488b4d78  mov rcx, qword ptr [rbp + 0x78]
1416fabe0: 0f8497010000  je 0x1416fad7d
1416fabe6: 48c1e205  shl rdx, 5
1416fabea: 41b808000000  mov r8d, 8
1416fabf0: e971010000  jmp 0x1416fad66
1416fabf5: 0f1002  movups xmm0, xmmword ptr [rdx]
1416fabf8: 0f104a10  movups xmm1, xmmword ptr [rdx + 0x10]
1416fabfc: 0f294d30  movaps xmmword ptr [rbp + 0x30], xmm1
1416fac00: 0f294520  movaps xmmword ptr [rbp + 0x20], xmm0
1416fac04: 488d4de0  lea rcx, [rbp - 0x20]
1416fac08: 488d5520  lea rdx, [rbp + 0x20]
1416fac0c: e8ffef1c01  call 0x1428c9c10
1416fac11: 488d15286e5901  lea rdx, [rip + 0x1596e28]
1416fac18: 4c8d05f16d5901  lea r8, [rip + 0x1596df1]
1416fac1f: 488d4de0  lea rcx, [rbp - 0x20]
1416fac23: e8d8ed1c01  call 0x1428c9a00
1416fac28: 48894608  mov qword ptr [rsi + 8], rax
1416fac2c: c60601  mov byte ptr [rsi], 1
1416fac2f: 0fb64520  movzx eax, byte ptr [rbp + 0x20]
1416fac33: 83f803  cmp eax, 3
1416fac36: 0f8241010000  jb 0x1416fad7d
1416fac3c: 0f8411010000  je 0x1416fad53
1416fac42: 83f804  cmp eax, 4
1416fac45: 0f8522010000  jne 0x1416fad6d
1416fac4b: 488b4d30  mov rcx, qword ptr [rbp + 0x30]
1416fac4f: 488b5538  mov rdx, qword ptr [rbp + 0x38]
1416fac53: 48898da8000000  mov qword ptr [rbp + 0xa8], rcx
1416fac5a: e811e118ff  call 0x140888d70
1416fac5f: 488b5528  mov rdx, qword ptr [rbp + 0x28]
1416fac63: 4885d2  test rdx, rdx
1416fac66: 0f8411010000  je 0x1416fad7d
1416fac6c: 48c1e205  shl rdx, 5
1416fac70: 41b808000000  mov r8d, 8
1416fac76: 488b8da8000000  mov rcx, qword ptr [rbp + 0xa8]
1416fac7d: e9e4000000  jmp 0x1416fad66
1416fac82: 4c894d70  mov qword ptr [rbp + 0x70], r9
1416fac86: 4c8985a8000000  mov qword ptr [rbp + 0xa8], r8
1416fac8d: 0f2845e0  movaps xmm0, xmmword ptr [rbp - 0x20]
1416fac91: 0f284df0  movaps xmm1, xmmword ptr [rbp - 0x10]
1416fac95: 0f298d90000000  movaps xmmword ptr [rbp + 0x90], xmm1
1416fac9c: 0f298580000000  movaps xmmword ptr [rbp + 0x80], xmm0
1416faca3: 488d4da8  lea rcx, [rbp - 0x58]
1416faca7: 488d5520  lea rdx, [rbp + 0x20]
1416facab: e8a017d4ff  call 0x14143c450
1416facb0: 488b45a8  mov rax, qword ptr [rbp - 0x58]
1416facb4: 4885c0  test rax, rax
1416facb7: 7445  je 0x1416facfe
1416facb9: 488b4db8  mov rcx, qword ptr [rbp - 0x48]
1416facbd: 488d1449  lea rdx, [rcx + rcx*2]
1416facc1: 4c8b84d078010000  mov r8, qword ptr [rax + rdx*8 + 0x178]
1416facc9: 4c8945f0  mov qword ptr [rbp - 0x10], r8
1416faccd: 0f1084d068010000  movups xmm0, xmmword ptr [rax + rdx*8 + 0x168]
1416facd5: 0f2945e0  movaps xmmword ptr [rbp - 0x20], xmm0
1416facd9: 48c1e105  shl rcx, 5
1416facdd: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
1416face1: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
1416face6: 0f1145f8  movups xmmword ptr [rbp - 8], xmm0
1416facea: 0f114d08  movups xmmword ptr [rbp + 8], xmm1
1416facee: 488b55e0  mov rdx, qword ptr [rbp - 0x20]
1416facf2: 4889d0  mov rax, rdx
1416facf5: 48f7d8  neg rax
1416facf8: 0f8194000000  jno 0x1416fad92
1416facfe: 448ab580000000  mov r14b, byte ptr [rbp + 0x80]
1416fad05: 0f108581000000  movups xmm0, xmmword ptr [rbp + 0x81]
1416fad0c: 0f2945c0  movaps xmmword ptr [rbp - 0x40], xmm0
1416fad10: 0f108590000000  movups xmm0, xmmword ptr [rbp + 0x90]
1416fad17: 0f1145cf  movups xmmword ptr [rbp - 0x31], xmm0
1416fad1b: c685b700000001  mov byte ptr [rbp + 0xb7], 1
1416fad22: 488d4d20  lea rcx, [rbp + 0x20]
1416fad26: e8c5d018ff  call 0x140887df0
1416fad2b: 488b85a8000000  mov rax, qword ptr [rbp + 0xa8]
1416fad32: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
1416fad36: e9e7fdffff  jmp 0x1416fab22
1416fad3b: 885e01  mov byte ptr [rsi + 1], bl
1416fad3e: 31c0  xor eax, eax
1416fad40: 8806  mov byte ptr [rsi], al
1416fad42: 8b85a0000000  mov eax, dword ptr [rbp + 0xa0]
1416fad48: 83f803  cmp eax, 3
1416fad4b: 0f8562feffff  jne 0x1416fabb3
1416fad51: eb2a  jmp 0x1416fad7d
1416fad53: 488b5528  mov rdx, qword ptr [rbp + 0x28]
1416fad57: 4885d2  test rdx, rdx
1416fad5a: 7421  je 0x1416fad7d
1416fad5c: 488b4d30  mov rcx, qword ptr [rbp + 0x30]
1416fad60: 41b801000000  mov r8d, 1
1416fad66: e8b58ef1fe  call 0x140613c20
1416fad6b: eb10  jmp 0x1416fad7d
1416fad6d: 488d4d28  lea rcx, [rbp + 0x28]
1416fad71: c685b700000000  mov byte ptr [rbp + 0xb7], 0
1416fad78: e8c3e119ff  call 0x140898f40
1416fad7d: 4889f0  mov rax, rsi
1416fad80: 4881c440010000  add rsp, 0x140
1416fad87: 5b  pop rbx
1416fad88: 5f  pop rdi
1416fad89: 5e  pop rsi
1416fad8a: 415c  pop r12
1416fad8c: 415e  pop r14
1416fad8e: 415f  pop r15
1416fad90: 5d  pop rbp
1416fad91: c3  ret 
1416fad92: 730f  jae 0x1416fada3
1416fad94: 488b4de8  mov rcx, qword ptr [rbp - 0x18]
1416fad98: 41b801000000  mov r8d, 1
1416fad9e: e87d8ef1fe  call 0x140613c20
1416fada3: 0fb645f8  movzx eax, byte ptr [rbp - 8]
1416fada7: 83f803  cmp eax, 3
1416fadaa: 7231  jb 0x1416faddd
1416fadac: 7410  je 0x1416fadbe
1416fadae: 488d4d00  lea rcx, [rbp]
1416fadb2: 83f804  cmp eax, 4
1416fadb5: 7521  jne 0x1416fadd8
1416fadb7: e8d4e518ff  call 0x140889390
1416fadbc: eb1f  jmp 0x1416faddd
1416fadbe: 488b5500  mov rdx, qword ptr [rbp]
1416fadc2: 4885d2  test rdx, rdx
1416fadc5: 7416  je 0x1416faddd
1416fadc7: 488b4d08  mov rcx, qword ptr [rbp + 8]
1416fadcb: 41b801000000  mov r8d, 1
1416fadd1: e84a8ef1fe  call 0x140613c20
1416fadd6: eb05  jmp 0x1416faddd
1416fadd8: e863e119ff  call 0x140898f40
1416faddd: c645e00b  mov byte ptr [rbp - 0x20], 0xb
1416fade1: 488d15186c5901  lea rdx, [rip + 0x1596c18]
1416fade8: 4c8d05216c5901  lea r8, [rip + 0x1596c21]
1416fadef: 488d4de0  lea rcx, [rbp - 0x20]
1416fadf3: e848ed1f01  call 0x1428f9b40
1416fadf8: 48894608  mov qword ptr [rsi + 8], rax
1416fadfc: c60601  mov byte ptr [rsi], 1
1416fadff: 0fb68580000000  movzx eax, byte ptr [rbp + 0x80]
1416fae06: 83f803  cmp eax, 3
1416fae09: 723a  jb 0x1416fae45
1416fae0b: 7413  je 0x1416fae20
1416fae0d: 488d8d88000000  lea rcx, [rbp + 0x88]
1416fae14: 83f804  cmp eax, 4
1416fae17: 7527  jne 0x1416fae40
1416fae19: e872e518ff  call 0x140889390
1416fae1e: eb25  jmp 0x1416fae45
1416fae20: 488b9588000000  mov rdx, qword ptr [rbp + 0x88]
1416fae27: 4885d2  test rdx, rdx
1416fae2a: 7419  je 0x1416fae45
1416fae2c: 488b8d90000000  mov rcx, qword ptr [rbp + 0x90]
1416fae33: 41b801000000  mov r8d, 1
1416fae39: e8e28df1fe  call 0x140613c20
1416fae3e: eb05  jmp 0x1416fae45
1416fae40: e8fbe019ff  call 0x140898f40
1416fae45: 488b95a8000000  mov rdx, qword ptr [rbp + 0xa8]
1416fae4c: 4885d2  test rdx, rdx
1416fae4f: 0f84a0fcffff  je 0x1416faaf5
1416fae55: 41b801000000  mov r8d, 1
1416fae5b: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
1416fae5f: e8bc8df1fe  call 0x140613c20
1416fae64: e98cfcffff  jmp 0x1416faaf5
1416fae69: 0f1f8000000000  nop dword ptr [rax]
1416fae70: 4889542410  mov qword ptr [rsp + 0x10], rdx
1416fae75: 55  push rbp
1416fae76: 4157  push r15
1416fae78: 4156  push r14
1416fae7a: 4154  push r12
1416fae7c: 56  push rsi
1416fae7d: 57  push rdi
1416fae7e: 53  push rbx
1416fae7f: 4883ec20  sub rsp, 0x20
1416fae83: 488daa80000000  lea rbp, [rdx + 0x80]
1416fae8a: 488d4d20  lea rcx, [rbp + 0x20]
1416fae8e: e85dcf18ff  call 0x140887df0
1416fae93: c685b700000001  mov byte ptr [rbp + 0xb7], 1
1416fae9a: 4883c420  add rsp, 0x20
1416fae9e: 5b  pop rbx
1416fae9f: 5f  pop rdi
1416faea0: 5e  pop rsi
1416faea1: 415c  pop r12
1416faea3: 415e  pop r14
1416faea5: 415f  pop r15
1416faea7: 5d  pop rbp
1416faea8: c3  ret 
1416faea9: 0f1f8000000000  nop dword ptr [rax]
1416faeb0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1416faeb5: 55  push rbp
1416faeb6: 4157  push r15
1416faeb8: 4156  push r14
1416faeba: 4154  push r12
1416faebc: 56  push rsi
1416faebd: 57  push rdi
1416faebe: 53  push rbx
1416faebf: 4883ec20  sub rsp, 0x20
1416faec3: 488daa80000000  lea rbp, [rdx + 0x80]
1416faeca: 4883bda800000000  cmp qword ptr [rbp + 0xa8], 0
1416faed2: 7416  je 0x1416faeea
1416faed4: 41b801000000  mov r8d, 1
1416faeda: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
1416faede: 488b95a8000000  mov rdx, qword ptr [rbp + 0xa8]
1416faee5: e8368df1fe  call 0x140613c20
1416faeea: 90  nop 
1416faeeb: 4883c420  add rsp, 0x20
1416faeef: 5b  pop rbx
1416faef0: 5f  pop rdi
1416faef1: 5e  pop rsi
1416faef2: 415c  pop r12
1416faef4: 415e  pop r14
1416faef6: 415f  pop r15
1416faef8: 5d  pop rbp
1416faef9: c3  ret 
1416faefa: 660f1f440000  nop word ptr [rax + rax]
1416faf00: 4889542410  mov qword ptr [rsp + 0x10], rdx
1416faf05: 55  push rbp
1416faf06: 4157  push r15
1416faf08: 4156  push r14
1416faf0a: 4154  push r12
1416faf0c: 56  push rsi
1416faf0d: 57  push rdi
1416faf0e: 53  push rbx
1416faf0f: 4883ec20  sub rsp, 0x20
1416faf13: 488daa80000000  lea rbp, [rdx + 0x80]
1416faf1a: 4883bda000000000  cmp qword ptr [rbp + 0xa0], 0
1416faf22: 741a  je 0x1416faf3e
1416faf24: 488b95a0000000  mov rdx, qword ptr [rbp + 0xa0]
1416faf2b: 48c1e205  shl rdx, 5
1416faf2f: 41b808000000  mov r8d, 8
1416faf35: 488b4d78  mov rcx, qword ptr [rbp + 0x78]
1416faf39: e8e28cf1fe  call 0x140613c20
1416faf3e: 90  nop 
1416faf3f: 4883c420  add rsp, 0x20
1416faf43: 5b  pop rbx
1416faf44: 5f  pop rdi
1416faf45: 5e  pop rsi
1416faf46: 415c  pop r12
1416faf48: 415e  pop r14
1416faf4a: 415f  pop r15
1416faf4c: 5d  pop rbp
1416faf4d: c3  ret 
1416faf4e: 6690  nop 
1416faf50: 4889542410  mov qword ptr [rsp + 0x10], rdx
1416faf55: 55  push rbp
1416faf56: 4157  push r15
1416faf58: 4156  push r14
1416faf5a: 4154  push r12
1416faf5c: 56  push rsi
1416faf5d: 57  push rdi
1416faf5e: 53  push rbx
1416faf5f: 4883ec20  sub rsp, 0x20
1416faf63: 488daa80000000  lea rbp, [rdx + 0x80]
1416faf6a: 488d4d20  lea rcx, [rbp + 0x20]
1416faf6e: e81d2b16ff  call 0x14085da90
1416faf73: c685b700000000  mov byte ptr [rbp + 0xb7], 0
1416faf7a: 4883c420  add rsp, 0x20
1416faf7e: 5b  pop rbx
1416faf7f: 5f  pop rdi
1416faf80: 5e  pop rsi
1416faf81: 415c  pop r12
1416faf83: 415e  pop r14
1416faf85: 415f  pop r15
1416faf87: 5d  pop rbp
1416faf88: c3  ret 
1416faf89: 0f1f8000000000  nop dword ptr [rax]
1416faf90: 4889542410  mov qword ptr [rsp + 0x10], rdx
1416faf95: 55  push rbp
1416faf96: 4157  push r15
1416faf98: 4156  push r14
1416faf9a: 4154  push r12
1416faf9c: 56  push rsi
1416faf9d: 57  push rdi
1416faf9e: 53  push rbx
1416faf9f: 4883ec20  sub rsp, 0x20
1416fafa3: 488daa80000000  lea rbp, [rdx + 0x80]
1416fafaa: 488b5528  mov rdx, qword ptr [rbp + 0x28]
1416fafae: 4885d2  test rdx, rdx
1416fafb1: 7416  je 0x1416fafc9
1416fafb3: 48c1e205  shl rdx, 5
1416fafb7: 41b808000000  mov r8d, 8
1416fafbd: 488b8da8000000  mov rcx, qword ptr [rbp + 0xa8]
1416fafc4: e8578cf1fe  call 0x140613c20
1416fafc9: c685b700000000  mov byte ptr [rbp + 0xb7], 0
1416fafd0: 4883c420  add rsp, 0x20
1416fafd4: 5b  pop rbx
1416fafd5: 5f  pop rdi
1416fafd6: 5e  pop rsi
1416fafd7: 415c  pop r12
1416fafd9: 415e  pop r14
1416fafdb: 415f  pop r15
1416fafdd: 5d  pop rbp
1416fafde: c3  ret 
1416fafdf: 90  nop 
1416fafe0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1416fafe5: 55  push rbp
1416fafe6: 4157  push r15
1416fafe8: 4156  push r14
1416fafea: 4154  push r12
1416fafec: 56  push rsi
1416fafed: 57  push rdi
1416fafee: 53  push rbx
1416fafef: 4883ec20  sub rsp, 0x20
1416faff3: 488daa80000000  lea rbp, [rdx + 0x80]
1416faffa: 80bdb700000000  cmp byte ptr [rbp + 0xb7], 0
1416fb001: 7415  je 0x1416fb018
1416fb003: 80bda000000003  cmp byte ptr [rbp + 0xa0], 3
1416fb00a: 740c  je 0x1416fb018
1416fb00c: 0fb685a0000000  movzx eax, byte ptr [rbp + 0xa0]
1416fb013: 83f805  cmp eax, 5
1416fb016: 750f  jne 0x1416fb027
1416fb018: 4883c420  add rsp, 0x20
1416fb01c: 5b  pop rbx
1416fb01d: 5f  pop rdi
1416fb01e: 5e  pop rsi
1416fb01f: 415c  pop r12
1416fb021: 415e  pop r14
1416fb023: 415f  pop r15
1416fb025: 5d  pop rbp
1416fb026: c3  ret 
1416fb027: 488b4d78  mov rcx, qword ptr [rbp + 0x78]
1416fb02b: e8602a16ff  call 0x14085da90
1416fb030: ebe6  jmp 0x1416fb018
1416fb032: 66666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1416fb040: 4889542410  mov qword ptr [rsp + 0x10], rdx
1416fb045: 55  push rbp
1416fb046: 4157  push r15
1416fb048: 4156  push r14
1416fb04a: 4154  push r12
1416fb04c: 56  push rsi
1416fb04d: 57  push rdi
1416fb04e: 53  push rbx
1416fb04f: 4883ec20  sub rsp, 0x20
1416fb053: 488daa80000000  lea rbp, [rdx + 0x80]
1416fb05a: 488d8d80000000  lea rcx, [rbp + 0x80]
1416fb061: e82a2a16ff  call 0x14085da90
1416fb066: 90  nop 
1416fb067: 4883c420  add rsp, 0x20
1416fb06b: 5b  pop rbx
1416fb06c: 5f  pop rdi
1416fb06d: 5e  pop rsi
1416fb06e: 415c  pop r12
1416fb070: 415e  pop r14
1416fb072: 415f  pop r15
1416fb074: 5d  pop rbp
1416fb075: c3  ret 
