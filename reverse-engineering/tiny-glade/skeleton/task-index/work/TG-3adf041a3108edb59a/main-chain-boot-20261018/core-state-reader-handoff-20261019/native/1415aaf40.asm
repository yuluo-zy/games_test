1415aaf40: 55  push rbp
1415aaf41: 4157  push r15
1415aaf43: 4156  push r14
1415aaf45: 4155  push r13
1415aaf47: 4154  push r12
1415aaf49: 56  push rsi
1415aaf4a: 57  push rdi
1415aaf4b: 53  push rbx
1415aaf4c: 4881ec48010000  sub rsp, 0x148
1415aaf53: 488dac2480000000  lea rbp, [rsp + 0x80]
1415aaf5b: 440f2985b0000000  movaps xmmword ptr [rbp + 0xb0], xmm8
1415aaf63: 0f29bda0000000  movaps xmmword ptr [rbp + 0xa0], xmm7
1415aaf6a: 0f29b590000000  movaps xmmword ptr [rbp + 0x90], xmm6
1415aaf71: 48c78588000000feffffff  mov qword ptr [rbp + 0x88], 0xfffffffffffffffe
1415aaf7c: 4889ce  mov rsi, rcx
1415aaf7f: 488b5a10  mov rbx, qword ptr [rdx + 0x10]
1415aaf83: 488d4d38  lea rcx, [rbp + 0x38]
1415aaf87: e8f4d05dff  call 0x140b88080
1415aaf8c: 488b4540  mov rax, qword ptr [rbp + 0x40]
1415aaf90: 483b4550  cmp rax, qword ptr [rbp + 0x50]
1415aaf94: 7410  je 0x1415aafa6
1415aaf96: 488d4820  lea rcx, [rax + 0x20]
1415aaf9a: 48894d40  mov qword ptr [rbp + 0x40], rcx
1415aaf9e: 0fb608  movzx ecx, byte ptr [rax]
1415aafa1: 80f906  cmp cl, 6
1415aafa4: 757c  jne 0x1415ab022
1415aafa6: 488d1563f66c01  lea rdx, [rip + 0x16cf663]
1415aafad: 4c8d05acf06c01  lea r8, [rip + 0x16cf0ac]
1415aafb4: 31c9  xor ecx, ecx
1415aafb6: e865ed3401  call 0x1428f9d20
1415aafbb: 4889c7  mov rdi, rax
1415aafbe: 48893e  mov qword ptr [rsi], rdi
1415aafc1: c6464e02  mov byte ptr [rsi + 0x4e], 2
1415aafc5: 488b4d40  mov rcx, qword ptr [rbp + 0x40]
1415aafc9: 488b5550  mov rdx, qword ptr [rbp + 0x50]
1415aafcd: 4829ca  sub rdx, rcx
1415aafd0: 48c1ea05  shr rdx, 5
1415aafd4: e897dd2dff  call 0x140888d70
1415aafd9: 488b5548  mov rdx, qword ptr [rbp + 0x48]
1415aafdd: 4885d2  test rdx, rdx
1415aafe0: 7413  je 0x1415aaff5
1415aafe2: 488b4d38  mov rcx, qword ptr [rbp + 0x38]
1415aafe6: 48c1e205  shl rdx, 5
1415aafea: 41b808000000  mov r8d, 8
1415aaff0: e82b8c06ff  call 0x140613c20
1415aaff5: 4889f0  mov rax, rsi
1415aaff8: 0f28b590000000  movaps xmm6, xmmword ptr [rbp + 0x90]
1415aafff: 0f28bda0000000  movaps xmm7, xmmword ptr [rbp + 0xa0]
1415ab006: 440f2885b0000000  movaps xmm8, xmmword ptr [rbp + 0xb0]
1415ab00e: 4881c448010000  add rsp, 0x148
1415ab015: 5b  pop rbx
1415ab016: 5f  pop rdi
1415ab017: 5e  pop rsi
1415ab018: 415c  pop r12
1415ab01a: 415d  pop r13
1415ab01c: 415e  pop r14
1415ab01e: 415f  pop r15
1415ab020: 5d  pop rbp
1415ab021: c3  ret 
1415ab022: 884d18  mov byte ptr [rbp + 0x18], cl
1415ab025: 0f104001  movups xmm0, xmmword ptr [rax + 1]
1415ab029: 0f104810  movups xmm1, xmmword ptr [rax + 0x10]
1415ab02d: 0f114519  movups xmmword ptr [rbp + 0x19], xmm0
1415ab031: 0f114d28  movups xmmword ptr [rbp + 0x28], xmm1
1415ab035: 488d4d58  lea rcx, [rbp + 0x58]
1415ab039: 488d5518  lea rdx, [rbp + 0x18]
1415ab03d: e85ef849ff  call 0x140a4a8a0
1415ab042: 440fb6757a  movzx r14d, byte ptr [rbp + 0x7a]
1415ab047: 488b7d58  mov rdi, qword ptr [rbp + 0x58]
1415ab04b: 4180fe05  cmp r14b, 5
1415ab04f: 0f8469ffffff  je 0x1415aafbe
1415ab055: 0f104560  movups xmm0, xmmword ptr [rbp + 0x60]
1415ab059: 0f104d6a  movups xmm1, xmmword ptr [rbp + 0x6a]
1415ab05d: 0f114dba  movups xmmword ptr [rbp - 0x46], xmm1
1415ab061: 0f2945b0  movaps xmmword ptr [rbp - 0x50], xmm0
1415ab065: 8b457b  mov eax, dword ptr [rbp + 0x7b]
1415ab068: 894508  mov dword ptr [rbp + 8], eax
1415ab06b: 0fb6457f  movzx eax, byte ptr [rbp + 0x7f]
1415ab06f: 88450c  mov byte ptr [rbp + 0xc], al
1415ab072: 488b4540  mov rax, qword ptr [rbp + 0x40]
1415ab076: 483b4550  cmp rax, qword ptr [rbp + 0x50]
1415ab07a: 7410  je 0x1415ab08c
1415ab07c: 488d4820  lea rcx, [rax + 0x20]
1415ab080: 48894d40  mov qword ptr [rbp + 0x40], rcx
1415ab084: 0fb608  movzx ecx, byte ptr [rax]
1415ab087: 80f906  cmp cl, 6
1415ab08a: 751d  jne 0x1415ab0a9
1415ab08c: 488d157df56c01  lea rdx, [rip + 0x16cf57d]
1415ab093: 4c8d05c6ef6c01  lea r8, [rip + 0x16cefc6]
1415ab09a: b901000000  mov ecx, 1
1415ab09f: e87cec3401  call 0x1428f9d20
1415ab0a4: e912ffffff  jmp 0x1415aafbb
1415ab0a9: 884d58  mov byte ptr [rbp + 0x58], cl
1415ab0ac: 0f104001  movups xmm0, xmmword ptr [rax + 1]
1415ab0b0: 0f104810  movups xmm1, xmmword ptr [rax + 0x10]
1415ab0b4: 0f114559  movups xmmword ptr [rbp + 0x59], xmm0
1415ab0b8: 0f114d68  movups xmmword ptr [rbp + 0x68], xmm1
1415ab0bc: 488d4d18  lea rcx, [rbp + 0x18]
1415ab0c0: 488d5558  lea rdx, [rbp + 0x58]
1415ab0c4: e807d049ff  call 0x140a480d0
1415ab0c9: 807d1801  cmp byte ptr [rbp + 0x18], 1
1415ab0cd: 7509  jne 0x1415ab0d8
1415ab0cf: 488b7d20  mov rdi, qword ptr [rbp + 0x20]
1415ab0d3: e9e6feffff  jmp 0x1415aafbe
1415ab0d8: 488b4540  mov rax, qword ptr [rbp + 0x40]
1415ab0dc: 483b4550  cmp rax, qword ptr [rbp + 0x50]
1415ab0e0: 7415  je 0x1415ab0f7
1415ab0e2: 440fb67d19  movzx r15d, byte ptr [rbp + 0x19]
1415ab0e7: 488d4820  lea rcx, [rax + 0x20]
1415ab0eb: 48894d40  mov qword ptr [rbp + 0x40], rcx
1415ab0ef: 0fb608  movzx ecx, byte ptr [rax]
1415ab0f2: 80f906  cmp cl, 6
1415ab0f5: 751d  jne 0x1415ab114
1415ab0f7: 488d1512f56c01  lea rdx, [rip + 0x16cf512]
1415ab0fe: 4c8d055bef6c01  lea r8, [rip + 0x16cef5b]
1415ab105: b902000000  mov ecx, 2
1415ab10a: e811ec3401  call 0x1428f9d20
1415ab10f: e9a7feffff  jmp 0x1415aafbb
1415ab114: 884d58  mov byte ptr [rbp + 0x58], cl
1415ab117: 0f104001  movups xmm0, xmmword ptr [rax + 1]
1415ab11b: 0f104810  movups xmm1, xmmword ptr [rax + 0x10]
1415ab11f: 0f114559  movups xmmword ptr [rbp + 0x59], xmm0
1415ab123: 0f114d68  movups xmmword ptr [rbp + 0x68], xmm1
1415ab127: 488d05523d5201  lea rax, [rip + 0x1523d52]
1415ab12e: 4889442420  mov qword ptr [rsp + 0x20], rax
1415ab133: 48c744242801000000  mov qword ptr [rsp + 0x28], 1
1415ab13c: 4c8d052b3d5201  lea r8, [rip + 0x1523d2b]
1415ab143: 488d4d18  lea rcx, [rbp + 0x18]
1415ab147: 488d5558  lea rdx, [rbp + 0x58]
1415ab14b: 41b90d000000  mov r9d, 0xd
1415ab151: e8ea7bffff  call 0x1415a2d40
1415ab156: 807d1801  cmp byte ptr [rbp + 0x18], 1
1415ab15a: 0f846fffffff  je 0x1415ab0cf
1415ab160: 440fb66519  movzx r12d, byte ptr [rbp + 0x19]
1415ab165: 488b5540  mov rdx, qword ptr [rbp + 0x40]
1415ab169: 488b4d50  mov rcx, qword ptr [rbp + 0x50]
1415ab16d: 4839ca  cmp rdx, rcx
1415ab170: 744b  je 0x1415ab1bd
1415ab172: 488d4220  lea rax, [rdx + 0x20]
1415ab176: 48894540  mov qword ptr [rbp + 0x40], rax
1415ab17a: 440fb602  movzx r8d, byte ptr [rdx]
1415ab17e: 4180f806  cmp r8b, 6
1415ab182: 743c  je 0x1415ab1c0
1415ab184: 44884558  mov byte ptr [rbp + 0x58], r8b
1415ab188: 0f104201  movups xmm0, xmmword ptr [rdx + 1]
1415ab18c: 0f104a10  movups xmm1, xmmword ptr [rdx + 0x10]
1415ab190: 0f114559  movups xmmword ptr [rbp + 0x59], xmm0
1415ab194: 0f114d68  movups xmmword ptr [rbp + 0x68], xmm1
1415ab198: 488d4d18  lea rcx, [rbp + 0x18]
1415ab19c: 488d5558  lea rdx, [rbp + 0x58]
1415ab1a0: e8fb1dffff  call 0x14159cfa0
1415ab1a5: 837d1801  cmp dword ptr [rbp + 0x18], 1
1415ab1a9: 0f8420ffffff  je 0x1415ab0cf
1415ab1af: 448b6d1c  mov r13d, dword ptr [rbp + 0x1c]
1415ab1b3: 488b4540  mov rax, qword ptr [rbp + 0x40]
1415ab1b7: 488b4d50  mov rcx, qword ptr [rbp + 0x50]
1415ab1bb: eb06  jmp 0x1415ab1c3
1415ab1bd: 4889d0  mov rax, rdx
1415ab1c0: 4531ed  xor r13d, r13d
1415ab1c3: 4839c8  cmp rax, rcx
1415ab1c6: 7410  je 0x1415ab1d8
1415ab1c8: 488d4820  lea rcx, [rax + 0x20]
1415ab1cc: 48894d40  mov qword ptr [rbp + 0x40], rcx
1415ab1d0: 0fb608  movzx ecx, byte ptr [rax]
1415ab1d3: 80f906  cmp cl, 6
1415ab1d6: 751d  jne 0x1415ab1f5
1415ab1d8: 488d1531f46c01  lea rdx, [rip + 0x16cf431]
1415ab1df: 4c8d057aee6c01  lea r8, [rip + 0x16cee7a]
1415ab1e6: b904000000  mov ecx, 4
1415ab1eb: e830eb3401  call 0x1428f9d20
1415ab1f0: e9c6fdffff  jmp 0x1415aafbb
1415ab1f5: 884d58  mov byte ptr [rbp + 0x58], cl
1415ab1f8: 0f104001  movups xmm0, xmmword ptr [rax + 1]
1415ab1fc: 0f104810  movups xmm1, xmmword ptr [rax + 0x10]
1415ab200: 0f114559  movups xmmword ptr [rbp + 0x59], xmm0
1415ab204: 0f114d68  movups xmmword ptr [rbp + 0x68], xmm1
1415ab208: 488d4d18  lea rcx, [rbp + 0x18]
1415ab20c: 488d5558  lea rdx, [rbp + 0x58]
1415ab210: e88b1dffff  call 0x14159cfa0
1415ab215: 837d1801  cmp dword ptr [rbp + 0x18], 1
1415ab219: 0f84b0feffff  je 0x1415ab0cf
1415ab21f: 8b451c  mov eax, dword ptr [rbp + 0x1c]
1415ab222: 8945f0  mov dword ptr [rbp - 0x10], eax
1415ab225: 488b5540  mov rdx, qword ptr [rbp + 0x40]
1415ab229: 488b4550  mov rax, qword ptr [rbp + 0x50]
1415ab22d: 4839c2  cmp rdx, rax
1415ab230: 744e  je 0x1415ab280
1415ab232: 488d4a20  lea rcx, [rdx + 0x20]
1415ab236: 48894d40  mov qword ptr [rbp + 0x40], rcx
1415ab23a: 440fb602  movzx r8d, byte ptr [rdx]
1415ab23e: 4180f806  cmp r8b, 6
1415ab242: 743f  je 0x1415ab283
1415ab244: 44884558  mov byte ptr [rbp + 0x58], r8b
1415ab248: 0f104201  movups xmm0, xmmword ptr [rdx + 1]
1415ab24c: 0f104a10  movups xmm1, xmmword ptr [rdx + 0x10]
1415ab250: 0f114559  movups xmmword ptr [rbp + 0x59], xmm0
1415ab254: 0f114d68  movups xmmword ptr [rbp + 0x68], xmm1
1415ab258: 488d4d18  lea rcx, [rbp + 0x18]
1415ab25c: 488d5558  lea rdx, [rbp + 0x58]
1415ab260: e81beffeff  call 0x14159a180
1415ab265: 807d1801  cmp byte ptr [rbp + 0x18], 1
1415ab269: 0f8460feffff  je 0x1415ab0cf
1415ab26f: 0fb64519  movzx eax, byte ptr [rbp + 0x19]
1415ab273: 8945fc  mov dword ptr [rbp - 4], eax
1415ab276: 488b4d40  mov rcx, qword ptr [rbp + 0x40]
1415ab27a: 488b4550  mov rax, qword ptr [rbp + 0x50]
1415ab27e: eb0a  jmp 0x1415ab28a
1415ab280: 4889d1  mov rcx, rdx
1415ab283: c745fc00000000  mov dword ptr [rbp - 4], 0
1415ab28a: 4839c1  cmp rcx, rax
1415ab28d: 0f8497000000  je 0x1415ab32a
1415ab293: 488d5120  lea rdx, [rcx + 0x20]
1415ab297: 48895540  mov qword ptr [rbp + 0x40], rdx
1415ab29b: 440fb601  movzx r8d, byte ptr [rcx]
1415ab29f: 0f57f6  xorps xmm6, xmm6
1415ab2a2: 4180f806  cmp r8b, 6
1415ab2a6: 7438  je 0x1415ab2e0
1415ab2a8: 44884558  mov byte ptr [rbp + 0x58], r8b
1415ab2ac: 0f104101  movups xmm0, xmmword ptr [rcx + 1]
1415ab2b0: 0f104910  movups xmm1, xmmword ptr [rcx + 0x10]
1415ab2b4: 0f114559  movups xmmword ptr [rbp + 0x59], xmm0
1415ab2b8: 0f114d68  movups xmmword ptr [rbp + 0x68], xmm1
1415ab2bc: 488d4d18  lea rcx, [rbp + 0x18]
1415ab2c0: 488d5558  lea rdx, [rbp + 0x58]
1415ab2c4: e8a7f0feff  call 0x14159a370
1415ab2c9: f6451801  test byte ptr [rbp + 0x18], 1
1415ab2cd: 0f85fcfdffff  jne 0x1415ab0cf
1415ab2d3: f30f10751c  movss xmm6, dword ptr [rbp + 0x1c]
1415ab2d8: 488b5540  mov rdx, qword ptr [rbp + 0x40]
1415ab2dc: 488b4550  mov rax, qword ptr [rbp + 0x50]
1415ab2e0: 4839c2  cmp rdx, rax
1415ab2e3: 7448  je 0x1415ab32d
1415ab2e5: 488d4220  lea rax, [rdx + 0x20]
1415ab2e9: 48894540  mov qword ptr [rbp + 0x40], rax
1415ab2ed: 0fb602  movzx eax, byte ptr [rdx]
1415ab2f0: 3c06  cmp al, 6
1415ab2f2: 7439  je 0x1415ab32d
1415ab2f4: 884558  mov byte ptr [rbp + 0x58], al
1415ab2f7: 0f104201  movups xmm0, xmmword ptr [rdx + 1]
1415ab2fb: 0f104a10  movups xmm1, xmmword ptr [rdx + 0x10]
1415ab2ff: 0f114559  movups xmmword ptr [rbp + 0x59], xmm0
1415ab303: 0f114d68  movups xmmword ptr [rbp + 0x68], xmm1
1415ab307: 488d4d18  lea rcx, [rbp + 0x18]
1415ab30b: 488d5558  lea rdx, [rbp + 0x58]
1415ab30f: e8dc21ffff  call 0x14159d4f0
1415ab314: 807d1800  cmp byte ptr [rbp + 0x18], 0
1415ab318: 0f85b1fdffff  jne 0x1415ab0cf
1415ab31e: 0fb64519  movzx eax, byte ptr [rbp + 0x19]
1415ab322: 888587000000  mov byte ptr [rbp + 0x87], al
1415ab328: eb0a  jmp 0x1415ab334
1415ab32a: 0f57f6  xorps xmm6, xmm6
1415ab32d: c6858700000001  mov byte ptr [rbp + 0x87], 1
1415ab334: 488d4d58  lea rcx, [rbp + 0x58]
1415ab338: 488d5538  lea rdx, [rbp + 0x38]
1415ab33c: e85f5a0200  call 0x1415d0da0
1415ab341: f6455801  test byte ptr [rbp + 0x58], 1
1415ab345: 0f850c010000  jne 0x1415ab457
1415ab34b: c7450000000000  mov dword ptr [rbp], 0
1415ab352: 807d5c00  cmp byte ptr [rbp + 0x5c], 0
1415ab356: 7406  je 0x1415ab35e
1415ab358: 8b4560  mov eax, dword ptr [rbp + 0x60]
1415ab35b: 894500  mov dword ptr [rbp], eax
1415ab35e: 488d4d58  lea rcx, [rbp + 0x58]
1415ab362: 488d5538  lea rdx, [rbp + 0x38]
1415ab366: e8c55a0200  call 0x1415d0e30
1415ab36b: f6455801  test byte ptr [rbp + 0x58], 1
1415ab36f: 0f85e2000000  jne 0x1415ab457
1415ab375: 807d5c00  cmp byte ptr [rbp + 0x5c], 0
1415ab379: 750a  jne 0x1415ab385
1415ab37b: f30f103d81a53701  movss xmm7, dword ptr [rip + 0x137a581]
1415ab383: eb05  jmp 0x1415ab38a
1415ab385: f30f107d60  movss xmm7, dword ptr [rbp + 0x60]
1415ab38a: 488d4d58  lea rcx, [rbp + 0x58]
1415ab38e: 488d5538  lea rdx, [rbp + 0x38]
1415ab392: e889580200  call 0x1415d0c20
1415ab397: 807d5800  cmp byte ptr [rbp + 0x58], 0
1415ab39b: 0f85b6000000  jne 0x1415ab457
1415ab3a1: 0fb64559  movzx eax, byte ptr [rbp + 0x59]
1415ab3a5: 3c02  cmp al, 2
1415ab3a7: b901000000  mov ecx, 1
1415ab3ac: 0f45c8  cmovne ecx, eax
1415ab3af: 894df4  mov dword ptr [rbp - 0xc], ecx
1415ab3b2: 488d4d58  lea rcx, [rbp + 0x58]
1415ab3b6: 488d5538  lea rdx, [rbp + 0x38]
1415ab3ba: e8715a0200  call 0x1415d0e30
1415ab3bf: f6455801  test byte ptr [rbp + 0x58], 1
1415ab3c3: 0f858e000000  jne 0x1415ab457
1415ab3c9: 807d5c00  cmp byte ptr [rbp + 0x5c], 0
1415ab3cd: 750b  jne 0x1415ab3da
1415ab3cf: f3440f10052ca53701  movss xmm8, dword ptr [rip + 0x137a52c]
1415ab3d8: eb06  jmp 0x1415ab3e0
1415ab3da: f3440f104560  movss xmm8, dword ptr [rbp + 0x60]
1415ab3e0: 488d4d58  lea rcx, [rbp + 0x58]
1415ab3e4: 488d5538  lea rdx, [rbp + 0x38]
1415ab3e8: e8b3590200  call 0x1415d0da0
1415ab3ed: f6455801  test byte ptr [rbp + 0x58], 1
1415ab3f1: 7564  jne 0x1415ab457
1415ab3f3: c7450400000000  mov dword ptr [rbp + 4], 0
1415ab3fa: 807d5c00  cmp byte ptr [rbp + 0x5c], 0
1415ab3fe: 7406  je 0x1415ab406
1415ab400: 8b4560  mov eax, dword ptr [rbp + 0x60]
1415ab403: 894504  mov dword ptr [rbp + 4], eax
1415ab406: 488d4d58  lea rcx, [rbp + 0x58]
1415ab40a: 488d5538  lea rdx, [rbp + 0x38]
1415ab40e: e8ad5a0200  call 0x1415d0ec0
1415ab413: 807d5800  cmp byte ptr [rbp + 0x58], 0
1415ab417: 753e  jne 0x1415ab457
1415ab419: 0fb64559  movzx eax, byte ptr [rbp + 0x59]
1415ab41d: 31c9  xor ecx, ecx
1415ab41f: 3c04  cmp al, 4
1415ab421: 0f45c8  cmovne ecx, eax
1415ab424: 894df8  mov dword ptr [rbp - 8], ecx
1415ab427: 488d4d58  lea rcx, [rbp + 0x58]
1415ab42b: 488d5538  lea rdx, [rbp + 0x38]
1415ab42f: e88c5b0200  call 0x1415d0fc0
1415ab434: 807d5800  cmp byte ptr [rbp + 0x58], 0
1415ab438: 751d  jne 0x1415ab457
1415ab43a: 0fb64559  movzx eax, byte ptr [rbp + 0x59]
1415ab43e: 888586000000  mov byte ptr [rbp + 0x86], al
1415ab444: 488d4d58  lea rcx, [rbp + 0x58]
1415ab448: 488d5538  lea rdx, [rbp + 0x38]
1415ab44c: e84f590200  call 0x1415d0da0
1415ab451: f6455801  test byte ptr [rbp + 0x58], 1
1415ab455: 7409  je 0x1415ab460
1415ab457: 488b7d60  mov rdi, qword ptr [rbp + 0x60]
1415ab45b: e95efbffff  jmp 0x1415aafbe
1415ab460: 8b555c  mov edx, dword ptr [rbp + 0x5c]
1415ab463: 8b4560  mov eax, dword ptr [rbp + 0x60]
1415ab466: 0f2845b0  movaps xmm0, xmmword ptr [rbp - 0x50]
1415ab46a: 0f2945d0  movaps xmmword ptr [rbp - 0x30], xmm0
1415ab46e: 0f1045ba  movups xmm0, xmmword ptr [rbp - 0x46]
1415ab472: 0f1145da  movups xmmword ptr [rbp - 0x26], xmm0
1415ab476: 8b4d08  mov ecx, dword ptr [rbp + 8]
1415ab479: 894d10  mov dword ptr [rbp + 0x10], ecx
1415ab47c: 0fb64d0c  movzx ecx, byte ptr [rbp + 0xc]
1415ab480: 884d14  mov byte ptr [rbp + 0x14], cl
1415ab483: 488b4d50  mov rcx, qword ptr [rbp + 0x50]
1415ab487: 483b4d40  cmp rcx, qword ptr [rbp + 0x40]
1415ab48b: 0f859b000000  jne 0x1415ab52c
1415ab491: 4531c0  xor r8d, r8d
1415ab494: 0fb68d86000000  movzx ecx, byte ptr [rbp + 0x86]
1415ab49b: 80f902  cmp cl, 2
1415ab49e: 0fb6c9  movzx ecx, cl
1415ab4a1: 410f44c8  cmove ecx, r8d
1415ab4a5: 85d2  test edx, edx
1415ab4a7: 0f44c2  cmove eax, edx
1415ab4aa: 0f1045da  movups xmm0, xmmword ptr [rbp - 0x26]
1415ab4ae: 0f114612  movups xmmword ptr [rsi + 0x12], xmm0
1415ab4b2: 0f2845d0  movaps xmm0, xmmword ptr [rbp - 0x30]
1415ab4b6: 0f114608  movups xmmword ptr [rsi + 8], xmm0
1415ab4ba: 8b5510  mov edx, dword ptr [rbp + 0x10]
1415ab4bd: 895623  mov dword ptr [rsi + 0x23], edx
1415ab4c0: 0fb65514  movzx edx, byte ptr [rbp + 0x14]
1415ab4c4: 885627  mov byte ptr [rsi + 0x27], dl
1415ab4c7: 48893e  mov qword ptr [rsi], rdi
1415ab4ca: 44887622  mov byte ptr [rsi + 0x22], r14b
1415ab4ce: 44896e28  mov dword ptr [rsi + 0x28], r13d
1415ab4d2: 8b55f0  mov edx, dword ptr [rbp - 0x10]
1415ab4d5: 89562c  mov dword ptr [rsi + 0x2c], edx
1415ab4d8: f30f117630  movss dword ptr [rsi + 0x30], xmm6
1415ab4dd: 8b5500  mov edx, dword ptr [rbp]
1415ab4e0: 895634  mov dword ptr [rsi + 0x34], edx
1415ab4e3: f30f117e38  movss dword ptr [rsi + 0x38], xmm7
1415ab4e8: f3440f11463c  movss dword ptr [rsi + 0x3c], xmm8
1415ab4ee: 8b5504  mov edx, dword ptr [rbp + 4]
1415ab4f1: 895640  mov dword ptr [rsi + 0x40], edx
1415ab4f4: 894644  mov dword ptr [rsi + 0x44], eax
1415ab4f7: 44886648  mov byte ptr [rsi + 0x48], r12b
1415ab4fb: 8b45fc  mov eax, dword ptr [rbp - 4]
1415ab4fe: 884649  mov byte ptr [rsi + 0x49], al
1415ab501: 44887e4a  mov byte ptr [rsi + 0x4a], r15b
1415ab505: 8b45f8  mov eax, dword ptr [rbp - 8]
1415ab508: 88464b  mov byte ptr [rsi + 0x4b], al
1415ab50b: 0fb68587000000  movzx eax, byte ptr [rbp + 0x87]
1415ab512: 88464c  mov byte ptr [rsi + 0x4c], al
1415ab515: 8b45f4  mov eax, dword ptr [rbp - 0xc]
1415ab518: 88464d  mov byte ptr [rsi + 0x4d], al
1415ab51b: 884e4e  mov byte ptr [rsi + 0x4e], cl
1415ab51e: 488d4d38  lea rcx, [rbp + 0x38]
1415ab522: e839f02dff  call 0x14088a560
1415ab527: e9c9faffff  jmp 0x1415aaff5
1415ab52c: 488d151deb6c01  lea rdx, [rip + 0x16ceb1d]
1415ab533: 4c8d0526eb6c01  lea r8, [rip + 0x16ceb26]
1415ab53a: 4889d9  mov rcx, rbx
1415ab53d: e8dee73401  call 0x1428f9d20
1415ab542: 488906  mov qword ptr [rsi], rax
1415ab545: b102  mov cl, 2
1415ab547: ebd2  jmp 0x1415ab51b
1415ab549: 0f1f8000000000  nop dword ptr [rax]
1415ab550: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415ab555: 55  push rbp
1415ab556: 4157  push r15
1415ab558: 4156  push r14
1415ab55a: 4155  push r13
1415ab55c: 4154  push r12
1415ab55e: 56  push rsi
1415ab55f: 57  push rdi
1415ab560: 53  push rbx
1415ab561: 4883ec68  sub rsp, 0x68
1415ab565: 488daa80000000  lea rbp, [rdx + 0x80]
1415ab56c: 440f29442430  movaps xmmword ptr [rsp + 0x30], xmm8
1415ab572: 0f297c2440  movaps xmmword ptr [rsp + 0x40], xmm7
1415ab577: 0f29742450  movaps xmmword ptr [rsp + 0x50], xmm6
1415ab57c: 488b5548  mov rdx, qword ptr [rbp + 0x48]
1415ab580: 4885d2  test rdx, rdx
1415ab583: 7413  je 0x1415ab598
1415ab585: 488b4d38  mov rcx, qword ptr [rbp + 0x38]
1415ab589: 48c1e205  shl rdx, 5
1415ab58d: 41b808000000  mov r8d, 8
1415ab593: e8888606ff  call 0x140613c20
1415ab598: 0f28742450  movaps xmm6, xmmword ptr [rsp + 0x50]
1415ab59d: 0f287c2440  movaps xmm7, xmmword ptr [rsp + 0x40]
1415ab5a2: 440f28442430  movaps xmm8, xmmword ptr [rsp + 0x30]
1415ab5a8: 4883c468  add rsp, 0x68
1415ab5ac: 5b  pop rbx
1415ab5ad: 5f  pop rdi
1415ab5ae: 5e  pop rsi
1415ab5af: 415c  pop r12
1415ab5b1: 415d  pop r13
1415ab5b3: 415e  pop r14
1415ab5b5: 415f  pop r15
1415ab5b7: 5d  pop rbp
1415ab5b8: c3  ret 
1415ab5b9: 0f1f8000000000  nop dword ptr [rax]
1415ab5c0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415ab5c5: 55  push rbp
1415ab5c6: 4157  push r15
1415ab5c8: 4156  push r14
1415ab5ca: 4155  push r13
1415ab5cc: 4154  push r12
1415ab5ce: 56  push rsi
1415ab5cf: 57  push rdi
1415ab5d0: 53  push rbx
1415ab5d1: 4883ec68  sub rsp, 0x68
1415ab5d5: 488daa80000000  lea rbp, [rdx + 0x80]
1415ab5dc: 440f29442430  movaps xmmword ptr [rsp + 0x30], xmm8
1415ab5e2: 0f297c2440  movaps xmmword ptr [rsp + 0x40], xmm7
1415ab5e7: 0f29742450  movaps xmmword ptr [rsp + 0x50], xmm6
1415ab5ec: 488d4d38  lea rcx, [rbp + 0x38]
1415ab5f0: e86bef2dff  call 0x14088a560
1415ab5f5: 0f28742450  movaps xmm6, xmmword ptr [rsp + 0x50]
1415ab5fa: 0f287c2440  movaps xmm7, xmmword ptr [rsp + 0x40]
1415ab5ff: 440f28442430  movaps xmm8, xmmword ptr [rsp + 0x30]
1415ab605: 4883c468  add rsp, 0x68
1415ab609: 5b  pop rbx
1415ab60a: 5f  pop rdi
1415ab60b: 5e  pop rsi
1415ab60c: 415c  pop r12
1415ab60e: 415d  pop r13
1415ab610: 415e  pop r14
1415ab612: 415f  pop r15
1415ab614: 5d  pop rbp
1415ab615: c3  ret 
