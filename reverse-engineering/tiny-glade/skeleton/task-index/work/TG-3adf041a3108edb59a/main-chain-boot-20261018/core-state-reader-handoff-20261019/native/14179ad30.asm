14179ad30: 55  push rbp
14179ad31: 4157  push r15
14179ad33: 4156  push r14
14179ad35: 4154  push r12
14179ad37: 56  push rsi
14179ad38: 57  push rdi
14179ad39: 53  push rbx
14179ad3a: 4881ec40010000  sub rsp, 0x140
14179ad41: 488dac2480000000  lea rbp, [rsp + 0x80]
14179ad49: 48c785b8000000feffffff  mov qword ptr [rbp + 0xb8], 0xfffffffffffffffe
14179ad54: 4889ce  mov rsi, rcx
14179ad57: 0fb602  movzx eax, byte ptr [rdx]
14179ad5a: 83f803  cmp eax, 3
14179ad5d: 8985a0000000  mov dword ptr [rbp + 0xa0], eax
14179ad63: 48895578  mov qword ptr [rbp + 0x78], rdx
14179ad67: 0f84ed000000  je 0x14179ae5a
14179ad6d: 83f805  cmp eax, 5
14179ad70: 0f8548010000  jne 0x14179aebe
14179ad76: 4c8b7a08  mov r15, qword ptr [rdx + 8]
14179ad7a: 4c8b6210  mov r12, qword ptr [rdx + 0x10]
14179ad7e: 488b7a18  mov rdi, qword ptr [rdx + 0x18]
14179ad82: 31c0  xor eax, eax
14179ad84: 4d85ff  test r15, r15
14179ad87: 0f95c0  setne al
14179ad8a: 4889f9  mov rcx, rdi
14179ad8d: 490f44cf  cmove rcx, r15
14179ad91: 48894520  mov qword ptr [rbp + 0x20], rax
14179ad95: 48c7452800000000  mov qword ptr [rbp + 0x28], 0
14179ad9d: 4c897d30  mov qword ptr [rbp + 0x30], r15
14179ada1: 4c896538  mov qword ptr [rbp + 0x38], r12
14179ada5: 48894540  mov qword ptr [rbp + 0x40], rax
14179ada9: 48c7454800000000  mov qword ptr [rbp + 0x48], 0
14179adb1: 4c897d50  mov qword ptr [rbp + 0x50], r15
14179adb5: 4c896558  mov qword ptr [rbp + 0x58], r12
14179adb9: 48894d60  mov qword ptr [rbp + 0x60], rcx
14179adbd: 488d8d80000000  lea rcx, [rbp + 0x80]
14179adc4: 488d5520  lea rdx, [rbp + 0x20]
14179adc8: e88316caff  call 0x14143c450
14179adcd: 488b8580000000  mov rax, qword ptr [rbp + 0x80]
14179add4: 4885c0  test rax, rax
14179add7: 7444  je 0x14179ae1d
14179add9: 488b8d90000000  mov rcx, qword ptr [rbp + 0x90]
14179ade0: 488d1449  lea rdx, [rcx + rcx*2]
14179ade4: 4c8b84d068010000  mov r8, qword ptr [rax + rdx*8 + 0x168]
14179adec: 4c8b8cd070010000  mov r9, qword ptr [rax + rdx*8 + 0x170]
14179adf4: 488b9cd078010000  mov rbx, qword ptr [rax + rdx*8 + 0x178]
14179adfc: 48c1e105  shl rcx, 5
14179ae00: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
14179ae04: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
14179ae09: 0f2945e0  movaps xmmword ptr [rbp - 0x20], xmm0
14179ae0d: 0f294df0  movaps xmmword ptr [rbp - 0x10], xmm1
14179ae11: 4c89c0  mov rax, r8
14179ae14: 48f7d8  neg rax
14179ae17: 0f81bf010000  jno 0x14179afdc
14179ae1d: c685800000000b  mov byte ptr [rbp + 0x80], 0xb
14179ae24: 488d155df65301  lea rdx, [rip + 0x153f65d]
14179ae2b: 4c8d0566f65301  lea r8, [rip + 0x153f666]
14179ae32: 488d8d80000000  lea rcx, [rbp + 0x80]
14179ae39: e802ed1501  call 0x1428f9b40
14179ae3e: 48894608  mov qword ptr [rsi + 8], rax
14179ae42: c60601  mov byte ptr [rsi], 1
14179ae45: c685b700000001  mov byte ptr [rbp + 0xb7], 1
14179ae4c: 488d4d20  lea rcx, [rbp + 0x20]
14179ae50: e89bcf0eff  call 0x140887df0
14179ae55: e97d020000  jmp 0x14179b0d7
14179ae5a: 4c8b7a08  mov r15, qword ptr [rdx + 8]
14179ae5e: 4c8b6210  mov r12, qword ptr [rdx + 0x10]
14179ae62: 488b7a18  mov rdi, qword ptr [rdx + 0x18]
14179ae66: 41b606  mov r14b, 6
14179ae69: 4c89f8  mov rax, r15
14179ae6c: 4c89e1  mov rcx, r12
14179ae6f: 4889fb  mov rbx, rdi
14179ae72: 48894520  mov qword ptr [rbp + 0x20], rax
14179ae76: 48894d28  mov qword ptr [rbp + 0x28], rcx
14179ae7a: 48895d30  mov qword ptr [rbp + 0x30], rbx
14179ae7e: 0f2845c0  movaps xmm0, xmmword ptr [rbp - 0x40]
14179ae82: 0f114539  movups xmmword ptr [rbp + 0x39], xmm0
14179ae86: 0f1045cf  movups xmm0, xmmword ptr [rbp - 0x31]
14179ae8a: 0f114548  movups xmmword ptr [rbp + 0x48], xmm0
14179ae8e: 44887538  mov byte ptr [rbp + 0x38], r14b
14179ae92: c685b700000001  mov byte ptr [rbp + 0xb7], 1
14179ae99: 488d4de0  lea rcx, [rbp - 0x20]
14179ae9d: 488d5520  lea rdx, [rbp + 0x20]
14179aea1: e88a82e3ff  call 0x1415d3130
14179aea6: 0fb645e8  movzx eax, byte ptr [rbp - 0x18]
14179aeaa: 488b5de0  mov rbx, qword ptr [rbp - 0x20]
14179aeae: 3c07  cmp al, 7
14179aeb0: 0f8595000000  jne 0x14179af4b
14179aeb6: 4889d8  mov rax, rbx
14179aeb9: e9c5000000  jmp 0x14179af83
14179aebe: 0f1002  movups xmm0, xmmword ptr [rdx]
14179aec1: 0f104a10  movups xmm1, xmmword ptr [rdx + 0x10]
14179aec5: 0f294d30  movaps xmmword ptr [rbp + 0x30], xmm1
14179aec9: 0f294520  movaps xmmword ptr [rbp + 0x20], xmm0
14179aecd: 488d4de0  lea rcx, [rbp - 0x20]
14179aed1: 488d5520  lea rdx, [rbp + 0x20]
14179aed5: e836ed1201  call 0x1428c9c10
14179aeda: 488d15e7f55301  lea rdx, [rip + 0x153f5e7]
14179aee1: 4c8d05b0f55301  lea r8, [rip + 0x153f5b0]
14179aee8: 488d4de0  lea rcx, [rbp - 0x20]
14179aeec: e80feb1201  call 0x1428c9a00
14179aef1: 48894608  mov qword ptr [rsi + 8], rax
14179aef5: c60601  mov byte ptr [rsi], 1
14179aef8: 0fb64520  movzx eax, byte ptr [rbp + 0x20]
14179aefc: 83f803  cmp eax, 3
14179aeff: 0f82d2010000  jb 0x14179b0d7
14179af05: 0f84a2010000  je 0x14179b0ad
14179af0b: 83f804  cmp eax, 4
14179af0e: 0f85b3010000  jne 0x14179b0c7
14179af14: 488b4d30  mov rcx, qword ptr [rbp + 0x30]
14179af18: 488b5538  mov rdx, qword ptr [rbp + 0x38]
14179af1c: 48898da8000000  mov qword ptr [rbp + 0xa8], rcx
14179af23: e848de0eff  call 0x140888d70
14179af28: 488b5528  mov rdx, qword ptr [rbp + 0x28]
14179af2c: 4885d2  test rdx, rdx
14179af2f: 0f84a2010000  je 0x14179b0d7
14179af35: 48c1e205  shl rdx, 5
14179af39: 41b808000000  mov r8d, 8
14179af3f: 488b8da8000000  mov rcx, qword ptr [rbp + 0xa8]
14179af46: e975010000  jmp 0x14179b0c0
14179af4b: 0f1045e9  movups xmm0, xmmword ptr [rbp - 0x17]
14179af4f: 0f104df8  movups xmm1, xmmword ptr [rbp - 8]
14179af53: 0f118d90000000  movups xmmword ptr [rbp + 0x90], xmm1
14179af5a: 0f118581000000  movups xmmword ptr [rbp + 0x81], xmm0
14179af61: 888580000000  mov byte ptr [rbp + 0x80], al
14179af67: c685b700000001  mov byte ptr [rbp + 0xb7], 1
14179af6e: 488d8d80000000  lea rcx, [rbp + 0x80]
14179af75: e886d03eff  call 0x140b88000
14179af7a: 4885c0  test rax, rax
14179af7d: 0f8412010000  je 0x14179b095
14179af83: 48894608  mov qword ptr [rsi + 8], rax
14179af87: b001  mov al, 1
14179af89: 8806  mov byte ptr [rsi], al
14179af8b: 8b85a0000000  mov eax, dword ptr [rbp + 0xa0]
14179af91: 83f803  cmp eax, 3
14179af94: 0f843d010000  je 0x14179b0d7
14179af9a: 83f805  cmp eax, 5
14179af9d: 0f8434010000  je 0x14179b0d7
14179afa3: 4c89bda0000000  mov qword ptr [rbp + 0xa0], r15
14179afaa: 4c896578  mov qword ptr [rbp + 0x78], r12
14179afae: 4c89e1  mov rcx, r12
14179afb1: 4889fa  mov rdx, rdi
14179afb4: e8b7dd0eff  call 0x140888d70
14179afb9: 488b95a0000000  mov rdx, qword ptr [rbp + 0xa0]
14179afc0: 4885d2  test rdx, rdx
14179afc3: 488b4d78  mov rcx, qword ptr [rbp + 0x78]
14179afc7: 0f840a010000  je 0x14179b0d7
14179afcd: 48c1e205  shl rdx, 5
14179afd1: 41b808000000  mov r8d, 8
14179afd7: e9e4000000  jmp 0x14179b0c0
14179afdc: 4c894d70  mov qword ptr [rbp + 0x70], r9
14179afe0: 4c8985a8000000  mov qword ptr [rbp + 0xa8], r8
14179afe7: 0f2845e0  movaps xmm0, xmmword ptr [rbp - 0x20]
14179afeb: 0f284df0  movaps xmm1, xmmword ptr [rbp - 0x10]
14179afef: 0f298d90000000  movaps xmmword ptr [rbp + 0x90], xmm1
14179aff6: 0f298580000000  movaps xmmword ptr [rbp + 0x80], xmm0
14179affd: 488d4da8  lea rcx, [rbp - 0x58]
14179b001: 488d5520  lea rdx, [rbp + 0x20]
14179b005: e84614caff  call 0x14143c450
14179b00a: 488b45a8  mov rax, qword ptr [rbp - 0x58]
14179b00e: 4885c0  test rax, rax
14179b011: 7445  je 0x14179b058
14179b013: 488b4db8  mov rcx, qword ptr [rbp - 0x48]
14179b017: 488d1449  lea rdx, [rcx + rcx*2]
14179b01b: 4c8b84d078010000  mov r8, qword ptr [rax + rdx*8 + 0x178]
14179b023: 4c8945f0  mov qword ptr [rbp - 0x10], r8
14179b027: 0f1084d068010000  movups xmm0, xmmword ptr [rax + rdx*8 + 0x168]
14179b02f: 0f2945e0  movaps xmmword ptr [rbp - 0x20], xmm0
14179b033: 48c1e105  shl rcx, 5
14179b037: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
14179b03b: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
14179b040: 0f1145f8  movups xmmword ptr [rbp - 8], xmm0
14179b044: 0f114d08  movups xmmword ptr [rbp + 8], xmm1
14179b048: 488b55e0  mov rdx, qword ptr [rbp - 0x20]
14179b04c: 4889d0  mov rax, rdx
14179b04f: 48f7d8  neg rax
14179b052: 0f8194000000  jno 0x14179b0ec
14179b058: 448ab580000000  mov r14b, byte ptr [rbp + 0x80]
14179b05f: 0f108581000000  movups xmm0, xmmword ptr [rbp + 0x81]
14179b066: 0f2945c0  movaps xmmword ptr [rbp - 0x40], xmm0
14179b06a: 0f108590000000  movups xmm0, xmmword ptr [rbp + 0x90]
14179b071: 0f1145cf  movups xmmword ptr [rbp - 0x31], xmm0
14179b075: c685b700000001  mov byte ptr [rbp + 0xb7], 1
14179b07c: 488d4d20  lea rcx, [rbp + 0x20]
14179b080: e86bcd0eff  call 0x140887df0
14179b085: 488b85a8000000  mov rax, qword ptr [rbp + 0xa8]
14179b08c: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
14179b090: e9ddfdffff  jmp 0x14179ae72
14179b095: 885e01  mov byte ptr [rsi + 1], bl
14179b098: 31c0  xor eax, eax
14179b09a: 8806  mov byte ptr [rsi], al
14179b09c: 8b85a0000000  mov eax, dword ptr [rbp + 0xa0]
14179b0a2: 83f803  cmp eax, 3
14179b0a5: 0f85effeffff  jne 0x14179af9a
14179b0ab: eb2a  jmp 0x14179b0d7
14179b0ad: 488b5528  mov rdx, qword ptr [rbp + 0x28]
14179b0b1: 4885d2  test rdx, rdx
14179b0b4: 7421  je 0x14179b0d7
14179b0b6: 488b4d30  mov rcx, qword ptr [rbp + 0x30]
14179b0ba: 41b801000000  mov r8d, 1
14179b0c0: e85b8be7fe  call 0x140613c20
14179b0c5: eb10  jmp 0x14179b0d7
14179b0c7: 488d4d28  lea rcx, [rbp + 0x28]
14179b0cb: c685b700000000  mov byte ptr [rbp + 0xb7], 0
14179b0d2: e869de0fff  call 0x140898f40
14179b0d7: 4889f0  mov rax, rsi
14179b0da: 4881c440010000  add rsp, 0x140
14179b0e1: 5b  pop rbx
14179b0e2: 5f  pop rdi
14179b0e3: 5e  pop rsi
14179b0e4: 415c  pop r12
14179b0e6: 415e  pop r14
14179b0e8: 415f  pop r15
14179b0ea: 5d  pop rbp
14179b0eb: c3  ret 
14179b0ec: 730f  jae 0x14179b0fd
14179b0ee: 488b4de8  mov rcx, qword ptr [rbp - 0x18]
14179b0f2: 41b801000000  mov r8d, 1
14179b0f8: e8238be7fe  call 0x140613c20
14179b0fd: 0fb645f8  movzx eax, byte ptr [rbp - 8]
14179b101: 83f803  cmp eax, 3
14179b104: 7231  jb 0x14179b137
14179b106: 7410  je 0x14179b118
14179b108: 488d4d00  lea rcx, [rbp]
14179b10c: 83f804  cmp eax, 4
14179b10f: 7521  jne 0x14179b132
14179b111: e87ae20eff  call 0x140889390
14179b116: eb1f  jmp 0x14179b137
14179b118: 488b5500  mov rdx, qword ptr [rbp]
14179b11c: 4885d2  test rdx, rdx
14179b11f: 7416  je 0x14179b137
14179b121: 488b4d08  mov rcx, qword ptr [rbp + 8]
14179b125: 41b801000000  mov r8d, 1
14179b12b: e8f08ae7fe  call 0x140613c20
14179b130: eb05  jmp 0x14179b137
14179b132: e809de0fff  call 0x140898f40
14179b137: c645e00b  mov byte ptr [rbp - 0x20], 0xb
14179b13b: 488d1546f35301  lea rdx, [rip + 0x153f346]
14179b142: 4c8d054ff35301  lea r8, [rip + 0x153f34f]
14179b149: 488d4de0  lea rcx, [rbp - 0x20]
14179b14d: e8eee91501  call 0x1428f9b40
14179b152: 48894608  mov qword ptr [rsi + 8], rax
14179b156: c60601  mov byte ptr [rsi], 1
14179b159: 0fb68580000000  movzx eax, byte ptr [rbp + 0x80]
14179b160: 83f803  cmp eax, 3
14179b163: 723a  jb 0x14179b19f
14179b165: 7413  je 0x14179b17a
14179b167: 488d8d88000000  lea rcx, [rbp + 0x88]
14179b16e: 83f804  cmp eax, 4
14179b171: 7527  jne 0x14179b19a
14179b173: e818e20eff  call 0x140889390
14179b178: eb25  jmp 0x14179b19f
14179b17a: 488b9588000000  mov rdx, qword ptr [rbp + 0x88]
14179b181: 4885d2  test rdx, rdx
14179b184: 7419  je 0x14179b19f
14179b186: 488b8d90000000  mov rcx, qword ptr [rbp + 0x90]
14179b18d: 41b801000000  mov r8d, 1
14179b193: e8888ae7fe  call 0x140613c20
14179b198: eb05  jmp 0x14179b19f
14179b19a: e8a1dd0fff  call 0x140898f40
14179b19f: 488b95a8000000  mov rdx, qword ptr [rbp + 0xa8]
14179b1a6: 4885d2  test rdx, rdx
14179b1a9: 0f8496fcffff  je 0x14179ae45
14179b1af: 41b801000000  mov r8d, 1
14179b1b5: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
14179b1b9: e8628ae7fe  call 0x140613c20
14179b1be: e982fcffff  jmp 0x14179ae45
14179b1c3: 666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
14179b1d0: 4889542410  mov qword ptr [rsp + 0x10], rdx
14179b1d5: 55  push rbp
14179b1d6: 4157  push r15
14179b1d8: 4156  push r14
14179b1da: 4154  push r12
14179b1dc: 56  push rsi
14179b1dd: 57  push rdi
14179b1de: 53  push rbx
14179b1df: 4883ec20  sub rsp, 0x20
14179b1e3: 488daa80000000  lea rbp, [rdx + 0x80]
14179b1ea: 488d4d20  lea rcx, [rbp + 0x20]
14179b1ee: e8fdcb0eff  call 0x140887df0
14179b1f3: c685b700000001  mov byte ptr [rbp + 0xb7], 1
14179b1fa: 4883c420  add rsp, 0x20
14179b1fe: 5b  pop rbx
14179b1ff: 5f  pop rdi
14179b200: 5e  pop rsi
14179b201: 415c  pop r12
14179b203: 415e  pop r14
14179b205: 415f  pop r15
14179b207: 5d  pop rbp
14179b208: c3  ret 
14179b209: 0f1f8000000000  nop dword ptr [rax]
14179b210: 4889542410  mov qword ptr [rsp + 0x10], rdx
14179b215: 55  push rbp
14179b216: 4157  push r15
14179b218: 4156  push r14
14179b21a: 4154  push r12
14179b21c: 56  push rsi
14179b21d: 57  push rdi
14179b21e: 53  push rbx
14179b21f: 4883ec20  sub rsp, 0x20
14179b223: 488daa80000000  lea rbp, [rdx + 0x80]
14179b22a: 4883bda800000000  cmp qword ptr [rbp + 0xa8], 0
14179b232: 7416  je 0x14179b24a
14179b234: 41b801000000  mov r8d, 1
14179b23a: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
14179b23e: 488b95a8000000  mov rdx, qword ptr [rbp + 0xa8]
14179b245: e8d689e7fe  call 0x140613c20
14179b24a: 90  nop 
14179b24b: 4883c420  add rsp, 0x20
14179b24f: 5b  pop rbx
14179b250: 5f  pop rdi
14179b251: 5e  pop rsi
14179b252: 415c  pop r12
14179b254: 415e  pop r14
14179b256: 415f  pop r15
14179b258: 5d  pop rbp
14179b259: c3  ret 
14179b25a: 660f1f440000  nop word ptr [rax + rax]
14179b260: 4889542410  mov qword ptr [rsp + 0x10], rdx
14179b265: 55  push rbp
14179b266: 4157  push r15
14179b268: 4156  push r14
14179b26a: 4154  push r12
14179b26c: 56  push rsi
14179b26d: 57  push rdi
14179b26e: 53  push rbx
14179b26f: 4883ec20  sub rsp, 0x20
14179b273: 488daa80000000  lea rbp, [rdx + 0x80]
14179b27a: 4883bda000000000  cmp qword ptr [rbp + 0xa0], 0
14179b282: 741a  je 0x14179b29e
14179b284: 488b95a0000000  mov rdx, qword ptr [rbp + 0xa0]
14179b28b: 48c1e205  shl rdx, 5
14179b28f: 41b808000000  mov r8d, 8
14179b295: 488b4d78  mov rcx, qword ptr [rbp + 0x78]
14179b299: e88289e7fe  call 0x140613c20
14179b29e: 90  nop 
14179b29f: 4883c420  add rsp, 0x20
14179b2a3: 5b  pop rbx
14179b2a4: 5f  pop rdi
14179b2a5: 5e  pop rsi
14179b2a6: 415c  pop r12
14179b2a8: 415e  pop r14
14179b2aa: 415f  pop r15
14179b2ac: 5d  pop rbp
14179b2ad: c3  ret 
14179b2ae: 6690  nop 
14179b2b0: 4889542410  mov qword ptr [rsp + 0x10], rdx
14179b2b5: 55  push rbp
14179b2b6: 4157  push r15
14179b2b8: 4156  push r14
14179b2ba: 4154  push r12
14179b2bc: 56  push rsi
14179b2bd: 57  push rdi
14179b2be: 53  push rbx
14179b2bf: 4883ec20  sub rsp, 0x20
14179b2c3: 488daa80000000  lea rbp, [rdx + 0x80]
14179b2ca: 488d4d20  lea rcx, [rbp + 0x20]
14179b2ce: e8bd270cff  call 0x14085da90
14179b2d3: c685b700000000  mov byte ptr [rbp + 0xb7], 0
14179b2da: 4883c420  add rsp, 0x20
14179b2de: 5b  pop rbx
14179b2df: 5f  pop rdi
14179b2e0: 5e  pop rsi
14179b2e1: 415c  pop r12
14179b2e3: 415e  pop r14
14179b2e5: 415f  pop r15
14179b2e7: 5d  pop rbp
14179b2e8: c3  ret 
14179b2e9: 0f1f8000000000  nop dword ptr [rax]
14179b2f0: 4889542410  mov qword ptr [rsp + 0x10], rdx
14179b2f5: 55  push rbp
14179b2f6: 4157  push r15
14179b2f8: 4156  push r14
14179b2fa: 4154  push r12
14179b2fc: 56  push rsi
14179b2fd: 57  push rdi
14179b2fe: 53  push rbx
14179b2ff: 4883ec20  sub rsp, 0x20
14179b303: 488daa80000000  lea rbp, [rdx + 0x80]
14179b30a: 488b5528  mov rdx, qword ptr [rbp + 0x28]
14179b30e: 4885d2  test rdx, rdx
14179b311: 7416  je 0x14179b329
14179b313: 48c1e205  shl rdx, 5
14179b317: 41b808000000  mov r8d, 8
14179b31d: 488b8da8000000  mov rcx, qword ptr [rbp + 0xa8]
14179b324: e8f788e7fe  call 0x140613c20
14179b329: c685b700000000  mov byte ptr [rbp + 0xb7], 0
14179b330: 4883c420  add rsp, 0x20
14179b334: 5b  pop rbx
14179b335: 5f  pop rdi
14179b336: 5e  pop rsi
14179b337: 415c  pop r12
14179b339: 415e  pop r14
14179b33b: 415f  pop r15
14179b33d: 5d  pop rbp
14179b33e: c3  ret 
14179b33f: 90  nop 
14179b340: 4889542410  mov qword ptr [rsp + 0x10], rdx
14179b345: 55  push rbp
14179b346: 4157  push r15
14179b348: 4156  push r14
14179b34a: 4154  push r12
14179b34c: 56  push rsi
14179b34d: 57  push rdi
14179b34e: 53  push rbx
14179b34f: 4883ec20  sub rsp, 0x20
14179b353: 488daa80000000  lea rbp, [rdx + 0x80]
14179b35a: 80bdb700000000  cmp byte ptr [rbp + 0xb7], 0
14179b361: 7415  je 0x14179b378
14179b363: 80bda000000003  cmp byte ptr [rbp + 0xa0], 3
14179b36a: 740c  je 0x14179b378
14179b36c: 0fb685a0000000  movzx eax, byte ptr [rbp + 0xa0]
14179b373: 83f805  cmp eax, 5
14179b376: 750f  jne 0x14179b387
14179b378: 4883c420  add rsp, 0x20
14179b37c: 5b  pop rbx
14179b37d: 5f  pop rdi
14179b37e: 5e  pop rsi
14179b37f: 415c  pop r12
14179b381: 415e  pop r14
14179b383: 415f  pop r15
14179b385: 5d  pop rbp
14179b386: c3  ret 
14179b387: 488b4d78  mov rcx, qword ptr [rbp + 0x78]
14179b38b: e800270cff  call 0x14085da90
14179b390: ebe6  jmp 0x14179b378
14179b392: 66666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
14179b3a0: 4889542410  mov qword ptr [rsp + 0x10], rdx
14179b3a5: 55  push rbp
14179b3a6: 4157  push r15
14179b3a8: 4156  push r14
14179b3aa: 4154  push r12
14179b3ac: 56  push rsi
14179b3ad: 57  push rdi
14179b3ae: 53  push rbx
14179b3af: 4883ec20  sub rsp, 0x20
14179b3b3: 488daa80000000  lea rbp, [rdx + 0x80]
14179b3ba: 488d8d80000000  lea rcx, [rbp + 0x80]
14179b3c1: e8ca260cff  call 0x14085da90
14179b3c6: 90  nop 
14179b3c7: 4883c420  add rsp, 0x20
14179b3cb: 5b  pop rbx
14179b3cc: 5f  pop rdi
14179b3cd: 5e  pop rsi
14179b3ce: 415c  pop r12
14179b3d0: 415e  pop r14
14179b3d2: 415f  pop r15
14179b3d4: 5d  pop rbp
14179b3d5: c3  ret 
