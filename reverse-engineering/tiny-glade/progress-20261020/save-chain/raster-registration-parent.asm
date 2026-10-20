14059d450: push rbp
14059d451: push r15
14059d453: push r14
14059d455: push r13
14059d457: push r12
14059d459: push rsi
14059d45a: push rdi
14059d45b: push rbx
14059d45c: mov eax, 0x37b8
14059d461: call 0x14289aa40
14059d466: sub rsp, rax
14059d469: lea rbp, [rsp + 0x80]
14059d471: movaps xmmword ptr [rbp + 0x3720], xmm15
14059d479: movaps xmmword ptr [rbp + 0x3710], xmm14
14059d481: movaps xmmword ptr [rbp + 0x3700], xmm13
14059d489: movaps xmmword ptr [rbp + 0x36f0], xmm12
14059d491: movaps xmmword ptr [rbp + 0x36e0], xmm11
14059d499: movaps xmmword ptr [rbp + 0x36d0], xmm10
14059d4a1: movaps xmmword ptr [rbp + 0x36c0], xmm9
14059d4a9: movaps xmmword ptr [rbp + 0x36b0], xmm8
14059d4b1: movaps xmmword ptr [rbp + 0x36a0], xmm7
14059d4b8: movaps xmmword ptr [rbp + 0x3690], xmm6
14059d4bf: and rsp, 0xffffffffffffff80
14059d4c3: mov rbx, rsp
14059d4c6: mov qword ptr [rbx + 0x3700], rbp
14059d4cd: mov qword ptr [rbp + 0x3688], 0xfffffffffffffffe
14059d4d8: mov qword ptr [rbx + 0xc8], r9
14059d4df: mov rax, qword ptr [rbp + 0x37a0]
14059d4e6: movups xmm0, xmmword ptr [rbp + 0x37d8]
14059d4ed: movups xmm1, xmmword ptr [rbp + 0x37c8]
14059d4f4: mov qword ptr [rbx + 0x2c0], rcx
14059d4fb: mov qword ptr [rbx + 0x2c8], rdx
14059d502: mov qword ptr [rbx + 0x140], r8
14059d509: movups xmmword ptr [rbx + 0x328], xmm1
14059d510: movups xmmword ptr [rbx + 0x160], xmm0
14059d517: movupd xmm0, xmmword ptr [rax + 0xae0]
14059d51f: movups xmm1, xmmword ptr [rax + 0xaf0]
14059d526: movaps xmmword ptr [rbx + 0x360], xmm1
14059d52d: movapd xmmword ptr [rbx + 0x350], xmm0
14059d535: mov rcx, qword ptr [rax + 0xb00]
14059d53c: mov qword ptr [rbx + 0x2b8], rcx
14059d543: mov rcx, qword ptr [rax + 0xb08]
14059d54a: mov qword ptr [rbx + 0x2b0], rcx
14059d551: mov qword ptr [rbx + 0xb00], 1
14059d55c: mov qword ptr [rbx + 0xb08], 1
14059d567: lea rcx, [rbx + 0xb10]
14059d56e: mov r8d, 0xae0
14059d574: mov rdx, rax
14059d577: call 0x142923c00
14059d57c: movzx eax, byte ptr [rip + 0x3097686]
14059d583: mov ecx, 0xaf0
14059d588: mov edx, 8
14059d58d: call 0x140613c10
14059d592: test rax, rax
14059d595: je 0x1405a6a26
14059d59b: mov rsi, rax
14059d59e: lea rdx, [rbx + 0xb00]
14059d5a5: mov r8d, 0xaf0
14059d5ab: mov rcx, rax
14059d5ae: call 0x142923c00
14059d5b3: mov qword ptr [rbx + 0x138], rsi
14059d5ba: cmp byte ptr [rsi + 0xae8], 1
14059d5c1: jne 0x14059d63c
14059d5c3: add rsi, 0x10
14059d5c7: mov byte ptr [rbx + 0xbc], 1
14059d5ce: mov byte ptr [rbx + 0xbd], 1
14059d5d5: mov byte ptr [rbx + 0xbe], 1
14059d5dc: mov byte ptr [rbx + 0xbf], 1
14059d5e3: mov byte ptr [rbx + 0xc0], 1
14059d5ea: mov byte ptr [rbx + 0xc1], 1
14059d5f1: mov rcx, rsi
14059d5f4: call 0x140bf8950
14059d5f9: add rax, 0x68
14059d5fd: mov byte ptr [rbx + 0xbc], 1
14059d604: mov byte ptr [rbx + 0xbd], 1
14059d60b: mov byte ptr [rbx + 0xbe], 1
14059d612: mov byte ptr [rbx + 0xbf], 1
14059d619: mov byte ptr [rbx + 0xc0], 1
14059d620: mov byte ptr [rbx + 0xc1], 1
14059d627: mov rcx, rax
14059d62a: call 0x140c001e0
14059d62f: test al, al
14059d631: je 0x14059d646
14059d633: mov rax, qword ptr [rip + 0x308ea8e]
14059d63a: jmp 0x14059d643
14059d63c: mov rax, qword ptr [rip + 0x308ea8d]
14059d643: mov byte ptr [rax], 1
14059d646: mov rcx, qword ptr [rbx + 0x138]
14059d64d: add rcx, 0x10
14059d651: mov byte ptr [rbx + 0xbc], 1
14059d658: mov byte ptr [rbx + 0xbd], 1
14059d65f: mov byte ptr [rbx + 0xbe], 1
14059d666: mov byte ptr [rbx + 0xbf], 1
14059d66d: mov byte ptr [rbx + 0xc0], 1
14059d674: mov byte ptr [rbx + 0xc1], 1
14059d67b: call 0x1405898a0
14059d680: mov rax, qword ptr [rbx + 0x140]
14059d687: mov rcx, qword ptr [rax + 0x18]
14059d68b: xorps xmm6, xmm6
14059d68e: movaps xmmword ptr [rbx + 0x440], xmm6
14059d695: lea rdx, [rbx + 0x440]
14059d69c: call qword ptr [rip + 0x2bf51de]
14059d6a2: test eax, eax
14059d6a4: je 0x1405a6520
14059d6aa: mov edi, dword ptr [rbx + 0x444]
14059d6b0: mov eax, dword ptr [rbx + 0x448]
14059d6b6: sub eax, dword ptr [rbx + 0x440]
14059d6bc: mov r14d, dword ptr [rbx + 0x44c]
14059d6c3: xorps xmm0, xmm0
14059d6c6: cvtsi2sd xmm0, rax
14059d6cb: mov byte ptr [rbx + 0xbc], 1
14059d6d2: mov byte ptr [rbx + 0xbd], 1
14059d6d9: mov byte ptr [rbx + 0xbe], 1
14059d6e0: mov byte ptr [rbx + 0xbf], 1
14059d6e7: mov byte ptr [rbx + 0xc0], 1
14059d6ee: mov byte ptr [rbx + 0xc1], 1
14059d6f5: call 0x1406977f0
14059d6fa: mov esi, eax
14059d6fc: sub r14d, edi
14059d6ff: xorps xmm0, xmm0
14059d702: cvtsi2sd xmm0, r14
14059d707: mov byte ptr [rbx + 0xbc], 1
14059d70e: mov byte ptr [rbx + 0xbd], 1
14059d715: mov byte ptr [rbx + 0xbe], 1
14059d71c: mov byte ptr [rbx + 0xbf], 1
14059d723: mov byte ptr [rbx + 0xc0], 1
14059d72a: mov byte ptr [rbx + 0xc1], 1
14059d731: call 0x1406977f0
14059d736: shl rax, 0x20
14059d73a: mov ecx, esi
14059d73c: or rcx, rax
14059d73f: mov byte ptr [rbx + 0xbc], 1
14059d746: mov byte ptr [rbx + 0xbd], 1
14059d74d: mov byte ptr [rbx + 0xbe], 1
14059d754: mov byte ptr [rbx + 0xbf], 1
14059d75b: mov byte ptr [rbx + 0xc0], 1
14059d762: mov byte ptr [rbx + 0xc1], 1
14059d769: call 0x140949bf0
14059d76e: mov dword ptr [rbx + 0x148], eax
14059d774: mov dword ptr [rbx + 0x14c], edx
14059d77a: mov rcx, qword ptr [rbp + 0x37b0]
14059d781: movzx eax, byte ptr [rcx + 0xf4]
14059d788: movss xmm0, dword ptr [rcx + 0xec]
14059d790: mov dword ptr [rbx + 0xb08], eax
14059d796: movss dword ptr [rbx + 0xb0c], xmm0
14059d79e: movabs r13, 0x8000000000000000
14059d7a8: mov qword ptr [rbx + 0xb00], r13
14059d7af: mov byte ptr [rbx + 0xbc], 1
14059d7b6: mov byte ptr [rbx + 0xbd], 1
14059d7bd: mov byte ptr [rbx + 0xbe], 1
14059d7c4: mov byte ptr [rbx + 0xbf], 1
14059d7cb: mov byte ptr [rbx + 0xc0], 1
14059d7d2: mov byte ptr [rbx + 0xc1], 1
14059d7d9: lea rsi, [rbx + 0x440]
14059d7e0: lea rdx, [rbx + 0x160]
14059d7e7: lea r8, [rbx + 0xb00]
14059d7ee: mov rcx, rsi
14059d7f1: call 0x1403c5a10
14059d7f6: movabs rax, 0x8000000000000007
14059d800: cmp qword ptr [rbx + 0x440], rax
14059d807: jne 0x1405a6a3a
14059d80d: call 0x140d4d740
14059d812: mov rdi, rax
14059d815: test byte ptr [rax], 1
14059d818: je 0x14059d82b
14059d81a: mov rcx, rdi
14059d81d: add rcx, 8
14059d821: mov rax, qword ptr [rdi + 8]
14059d825: mov rdx, qword ptr [rdi + 0x10]
14059d829: jmp 0x14059d870
14059d82b: mov byte ptr [rbx + 0xbc], 1
14059d832: mov byte ptr [rbx + 0xbd], 1
14059d839: mov byte ptr [rbx + 0xbe], 1
14059d840: mov byte ptr [rbx + 0xbf], 1
14059d847: mov byte ptr [rbx + 0xc0], 1
14059d84e: mov byte ptr [rbx + 0xc1], 1
14059d855: call 0x140d64e00
14059d85a: mov rcx, rdi
14059d85d: add rcx, 8
14059d861: mov qword ptr [rdi], 1
14059d868: mov qword ptr [rdi + 8], rax
14059d86c: mov qword ptr [rdi + 0x10], rdx
14059d870: lea r8, [rax + 1]
14059d874: mov qword ptr [rcx], r8
14059d877: movups xmm6, xmmword ptr [rip + 0x2460e32]
14059d87e: movaps xmmword ptr [rbx + 0xb10], xmm6
14059d885: movups xmm7, xmmword ptr [rip + 0x2460e14]
14059d88c: movaps xmmword ptr [rbx + 0xb00], xmm7
14059d893: mov qword ptr [rbx + 0xb20], rax
14059d89a: mov qword ptr [rbx + 0xb28], rdx
14059d8a1: call 0x140d4d740
14059d8a6: mov rdi, rax
14059d8a9: test byte ptr [rax], 1
14059d8ac: je 0x14059d8bf
14059d8ae: mov rcx, rdi
14059d8b1: add rcx, 8
14059d8b5: mov rax, qword ptr [rdi + 8]
14059d8b9: mov rdx, qword ptr [rdi + 0x10]
14059d8bd: jmp 0x14059d8da
14059d8bf: call 0x140d64e00
14059d8c4: mov rcx, rdi
14059d8c7: add rcx, 8
14059d8cb: mov qword ptr [rdi], 1
14059d8d2: mov qword ptr [rdi + 8], rax
14059d8d6: mov qword ptr [rdi + 0x10], rdx
14059d8da: lea r8, [rax + 1]
14059d8de: mov qword ptr [rcx], r8
14059d8e1: movaps xmm0, xmmword ptr [rbx + 0xb00]
14059d8e8: movaps xmm1, xmmword ptr [rbx + 0xb10]
14059d8ef: movaps xmm2, xmmword ptr [rbx + 0xb20]
14059d8f6: movaps xmmword ptr [rbx + 0xac0], xmm2
14059d8fd: movaps xmmword ptr [rbx + 0xab0], xmm1
14059d904: movaps xmmword ptr [rbx + 0xaa0], xmm0
14059d90b: movaps xmmword ptr [rbx + 0xad0], xmm7
14059d912: movaps xmmword ptr [rbx + 0xae0], xmm6
14059d919: mov qword ptr [rbx + 0xaf0], rax
14059d920: mov qword ptr [rbx + 0xaf8], rdx
14059d927: mov rdx, qword ptr [rbp + 0x37b8]
14059d92e: mov rax, qword ptr [rdx]
14059d931: lock inc qword ptr [rax]
14059d935: jle 0x1405a7040
14059d93b: mov rcx, qword ptr [rdx + 8]
14059d93f: lock inc qword ptr [rcx]
14059d943: jle 0x1405a7040
14059d949: mov rdx, qword ptr [rdx + 0x10]
14059d94d: lock inc qword ptr [rdx]
14059d951: jle 0x1405a7040
14059d957: mov qword ptr [rbx + 0xb00], rax
14059d95e: mov qword ptr [rbx + 0xb08], rcx
14059d965: mov qword ptr [rbx + 0xb10], rdx
14059d96c: xor ecx, ecx
14059d96e: call 0x1400d9b20
14059d973: test rax, rax
14059d976: je 0x1405a6adc
14059d97c: xorps xmm0, xmm0
14059d97f: movups xmmword ptr [rbx + 0x2108], xmm0
14059d986: movups xmm1, xmmword ptr [rax]
14059d989: inc qword ptr [rax]
14059d98c: mov rax, qword ptr [rbx + 0xb10]
14059d993: mov qword ptr [rbx + 0x2150], rax
14059d99a: movups xmm2, xmmword ptr [rbx + 0xb00]
14059d9a1: movups xmmword ptr [rbx + 0x2140], xmm2
14059d9a8: movups xmm2, xmmword ptr [rip + 0x239d741]
14059d9af: movups xmmword ptr [rbx + 0x2158], xmm2
14059d9b6: movups xmm2, xmmword ptr [rip + 0x239d743]
14059d9bd: movups xmmword ptr [rbx + 0x2168], xmm2
14059d9c4: movups xmmword ptr [rbx + 0x2178], xmm1
14059d9cb: mov qword ptr [rbx + 0x20e0], 0
14059d9d6: mov qword ptr [rbx + 0x20e8], 8
14059d9e1: movups xmmword ptr [rbx + 0x20f0], xmm0
14059d9e8: mov qword ptr [rbx + 0x2100], 8
14059d9f3: mov qword ptr [rbx + 0x2118], 8
14059d9fe: movups xmmword ptr [rbx + 0x2120], xmm0
14059da05: mov qword ptr [rbx + 0x2130], 8
14059da10: mov qword ptr [rbx + 0x2138], 0
14059da1b: mov rax, qword ptr [rip + 0x308e986]
14059da22: mov rax, qword ptr [rax]
14059da25: cmp rax, 3
14059da29: jb 0x14059db6b
14059da2f: lea rax, [rip + 0x2461d52]
14059da36: mov qword ptr [rbx + 0x440], rax
14059da3d: lea rax, [rip - 0x493bb4]
14059da44: mov qword ptr [rbx + 0x448], rax
14059da4b: mov byte ptr [rbx + 0xdc], 1
14059da52: mov byte ptr [rbx + 0xdd], 1
14059da59: mov byte ptr [rbx + 0xde], 1
14059da60: mov byte ptr [rbx + 0xdf], 1
14059da67: mov byte ptr [rbx + 0xe0], 1
14059da6e: mov byte ptr [rbx + 0xe1], 1
14059da75: mov byte ptr [rbx + 0xe2], 1
14059da7c: lea rcx, [rip + 0x2461d35]
14059da83: call 0x14067dd80
14059da88: movups xmm0, xmmword ptr [rax]
14059da8b: mov eax, dword ptr [rax + 0x10]
14059da8e: mov qword ptr [rbx + 0xb30], 3
14059da99: lea rcx, [rip + 0x2461950]
14059daa0: mov qword ptr [rbx + 0xb38], rcx
14059daa7: mov qword ptr [rbx + 0xb40], 0x14
14059dab2: lea rdx, [rip + 0x2461cef]
14059dab9: mov qword ptr [rbx + 0xb50], rdx
14059dac0: mov qword ptr [rbx + 0xb58], 1
14059dacb: mov qword ptr [rbx + 0xb60], rsi
14059dad2: mov qword ptr [rbx + 0xb68], 1
14059dadd: mov qword ptr [rbx + 0xb70], 0
14059dae8: mov qword ptr [rbx + 0xb00], 0
14059daf3: mov qword ptr [rbx + 0xb08], rcx
14059dafa: mov qword ptr [rbx + 0xb10], 0x14
14059db05: mov qword ptr [rbx + 0xb18], 0
14059db10: movups xmmword ptr [rbx + 0xb20], xmm0
14059db17: mov dword ptr [rbx + 0xb48], 1
14059db21: mov dword ptr [rbx + 0xb4c], eax
14059db27: mov byte ptr [rbx + 0xdc], 1
14059db2e: mov byte ptr [rbx + 0xdd], 1
14059db35: mov byte ptr [rbx + 0xde], 1
14059db3c: mov byte ptr [rbx + 0xdf], 1
14059db43: mov byte ptr [rbx + 0xe0], 1
14059db4a: mov byte ptr [rbx + 0xe1], 1
14059db51: mov byte ptr [rbx + 0xe2], 1
14059db58: lea rcx, [rbx + 0x370]
14059db5f: lea rdx, [rbx + 0xb00]
14059db66: call 0x140d4a330
14059db6b: mov byte ptr [rbx + 0xdc], 1
14059db72: mov byte ptr [rbx + 0xdd], 1
14059db79: mov byte ptr [rbx + 0xde], 1
14059db80: mov byte ptr [rbx + 0xdf], 1
14059db87: mov byte ptr [rbx + 0xe0], 1
14059db8e: mov byte ptr [rbx + 0xe1], 1
14059db95: mov byte ptr [rbx + 0xe2], 1
14059db9c: lea rcx, [rbx + 0x148]
14059dba3: call 0x140949c20
14059dba8: mov byte ptr [rbx + 0xdc], 1
14059dbaf: mov byte ptr [rbx + 0xdd], 1
14059dbb6: mov byte ptr [rbx + 0xde], 1
14059dbbd: mov byte ptr [rbx + 0xdf], 1
14059dbc4: mov byte ptr [rbx + 0xe0], 1
14059dbcb: mov byte ptr [rbx + 0xe1], 1
14059dbd2: mov byte ptr [rbx + 0xe2], 1
14059dbd9: lea rcx, [rbx + 0x3220]
14059dbe0: movaps xmm1, xmm0
14059dbe3: call 0x14090ec70
14059dbe8: movzx eax, byte ptr [rip + 0x309701a]
14059dbef: mov ecx, 0xb0
14059dbf4: mov edx, 8
14059dbf9: call 0x140613c10
14059dbfe: test rax, rax
14059dc01: je 0x1405a6aed
14059dc07: mov qword ptr [rax], 1
14059dc0e: mov qword ptr [rax + 8], 1
14059dc16: mov qword ptr [rax + 0x10], 0
14059dc1e: mov byte ptr [rax + 0x18], 0
14059dc22: mov qword ptr [rax + 0x20], r13
14059dc26: mov byte ptr [rax + 0xa0], 2
14059dc2d: mov byte ptr [rax + 0xa8], 0
14059dc34: mov qword ptr [rbx + 0x158], rax
14059dc3b: mov rax, qword ptr [rbp + 0x37c0]
14059dc42: cmp byte ptr [rax + 0x147], 1
14059dc49: jne 0x14059dc96
14059dc4b: mov byte ptr [rbx + 0xaa], 1
14059dc52: mov byte ptr [rbx + 0xab], 1
14059dc59: mov byte ptr [rbx + 0xac], 1
14059dc60: mov byte ptr [rbx + 0xad], 1
14059dc67: mov byte ptr [rbx + 0xae], 1
14059dc6e: mov byte ptr [rbx + 0xaf], 1
14059dc75: mov byte ptr [rbx + 0xb0], 1
14059dc7c: mov byte ptr [rbx + 0xb1], 1
14059dc83: mov byte ptr [rbx + 0xb2], 1
14059dc8a: mov rcx, qword ptr [rbx + 0xc8]
14059dc91: call 0x140328130
14059dc96: mov qword ptr [rbx + 0x440], 1
14059dca1: mov qword ptr [rbx + 0x448], 0xffffffffffffffff
14059dcac: mov qword ptr [rbx + 0x450], 0
14059dcb7: mov qword ptr [rbx + 0x460], 0
14059dcc2: mov qword ptr [rbx + 0x470], 1
14059dccd: mov qword ptr [rbx + 0x478], 4
14059dcd8: mov dword ptr [rbx + 0x480], 0x3e800000
14059dce2: mov qword ptr [rbx + 0x488], 0
14059dced: mov qword ptr [rbx + 0x498], 0
14059dcf8: mov qword ptr [rbx + 0x4a8], 1
14059dd03: mov qword ptr [rbx + 0x4b0], 4
14059dd0e: mov dword ptr [rbx + 0x4b8], 0x3e800000
14059dd18: mov qword ptr [rbx + 0x4c0], 0
14059dd23: mov qword ptr [rbx + 0x4d0], 0
14059dd2e: mov qword ptr [rbx + 0x4e0], 1
14059dd39: mov qword ptr [rbx + 0x4e8], 0xffffffffffffffff
14059dd44: mov dword ptr [rbx + 0x4f0], 0x3f800000
14059dd4e: mov byte ptr [rbx + 0x110], 1
14059dd55: mov rcx, qword ptr [rbx + 0xc8]
14059dd5c: call 0x140113ff0
14059dd61: cmp al, 1
14059dd63: ja 0x1405a6b39
14059dd69: lea rsi, [rbx + 0xb00]
14059dd70: lea rdx, [rbx + 0x440]
14059dd77: mov r8d, 0xb8
14059dd7d: mov rcx, rsi
14059dd80: call 0x142923c00
14059dd85: mov byte ptr [rbx + 0x110], 0
14059dd8c: lea r8, [rip + 0x2461a3d]
14059dd93: mov rcx, rsi
14059dd96: mov rsi, qword ptr [rbx + 0xc8]
14059dd9d: mov rdx, rsi
14059dda0: call 0x1400ec810
14059dda5: mov byte ptr [rbx + 0xaa], 1
14059ddac: mov byte ptr [rbx + 0xab], 1
14059ddb3: mov byte ptr [rbx + 0xac], 1
14059ddba: mov byte ptr [rbx + 0xad], 1
14059ddc1: mov byte ptr [rbx + 0xae], 1
14059ddc8: mov byte ptr [rbx + 0xaf], 1
14059ddcf: mov byte ptr [rbx + 0xb0], 1
14059ddd6: mov byte ptr [rbx + 0xb1], 1
14059dddd: mov byte ptr [rbx + 0xb2], 1
14059dde4: mov rcx, rsi
14059dde7: call 0x140113ff0
14059ddec: cmp al, 1
14059ddee: ja 0x1405a6b86
14059ddf4: mov byte ptr [rbx + 0xaa], 1
14059ddfb: mov byte ptr [rbx + 0xab], 1
14059de02: mov byte ptr [rbx + 0xac], 1
14059de09: mov byte ptr [rbx + 0xad], 1
14059de10: mov byte ptr [rbx + 0xae], 1
14059de17: mov byte ptr [rbx + 0xaf], 1
14059de1e: mov byte ptr [rbx + 0xb0], 1
14059de25: mov byte ptr [rbx + 0xb1], 1
14059de2c: mov byte ptr [rbx + 0xb2], 1
14059de33: lea rdx, [rip + 0x24619ae]
14059de3a: mov rcx, qword ptr [rbx + 0xc8]
14059de41: call 0x14032b660
14059de46: mov byte ptr [rbx + 0xaa], 1
14059de4d: mov byte ptr [rbx + 0xab], 1
14059de54: mov byte ptr [rbx + 0xac], 1
14059de5b: mov byte ptr [rbx + 0xad], 1
14059de62: mov byte ptr [rbx + 0xae], 1
14059de69: mov byte ptr [rbx + 0xaf], 1
14059de70: mov byte ptr [rbx + 0xb0], 1
14059de77: mov byte ptr [rbx + 0xb1], 1
14059de7e: mov byte ptr [rbx + 0xb2], 1
14059de85: mov rcx, qword ptr [rbx + 0xc8]
14059de8c: call 0x140113ff0
14059de91: cmp al, 1
14059de93: ja 0x1405a6c0b
14059de99: mov byte ptr [rbx + 0xaa], 1
14059dea0: mov byte ptr [rbx + 0xab], 1
14059dea7: mov byte ptr [rbx + 0xac], 1
14059deae: mov byte ptr [rbx + 0xad], 1
14059deb5: mov byte ptr [rbx + 0xae], 1
14059debc: mov byte ptr [rbx + 0xaf], 1
14059dec3: mov byte ptr [rbx + 0xb0], 1
14059deca: mov byte ptr [rbx + 0xb1], 1
14059ded1: mov byte ptr [rbx + 0xb2], 1
14059ded8: lea r9, [rip + 0x2461921]
14059dedf: xor ecx, ecx
14059dee1: xor edx, edx
14059dee3: mov r8, qword ptr [rbx + 0xc8]
14059deea: call 0x1405b0140
14059deef: mov byte ptr [rbx + 0xaa], 1
14059def6: mov byte ptr [rbx + 0xab], 1
14059defd: mov byte ptr [rbx + 0xac], 1
14059df04: mov byte ptr [rbx + 0xad], 1
14059df0b: mov byte ptr [rbx + 0xae], 1
14059df12: mov byte ptr [rbx + 0xaf], 1
14059df19: mov byte ptr [rbx + 0xb0], 1
14059df20: mov byte ptr [rbx + 0xb1], 1
14059df27: mov byte ptr [rbx + 0xb2], 1
14059df2e: mov rcx, qword ptr [rbx + 0xc8]
14059df35: call 0x140113ff0
14059df3a: cmp al, 1
14059df3c: ja 0x1405a6c90
14059df42: mov byte ptr [rbx + 0xaa], 1
14059df49: mov byte ptr [rbx + 0xab], 1
14059df50: mov byte ptr [rbx + 0xac], 1
14059df57: mov byte ptr [rbx + 0xad], 1
14059df5e: mov byte ptr [rbx + 0xae], 1
14059df65: mov byte ptr [rbx + 0xaf], 1
14059df6c: mov byte ptr [rbx + 0xb0], 1
14059df73: mov byte ptr [rbx + 0xb1], 1
14059df7a: mov byte ptr [rbx + 0xb2], 1
14059df81: lea rdx, [rip + 0x2461890]
14059df88: mov rcx, qword ptr [rbx + 0xc8]
14059df8f: call 0x14030a670
14059df94: mov byte ptr [rbx + 0xaa], 1
14059df9b: mov byte ptr [rbx + 0xab], 1
14059dfa2: mov byte ptr [rbx + 0xac], 1
14059dfa9: mov byte ptr [rbx + 0xad], 1
14059dfb0: mov byte ptr [rbx + 0xae], 1
14059dfb7: mov byte ptr [rbx + 0xaf], 1
14059dfbe: mov byte ptr [rbx + 0xb0], 1
14059dfc5: mov byte ptr [rbx + 0xb1], 1
14059dfcc: mov byte ptr [rbx + 0xb2], 1
14059dfd3: mov rcx, qword ptr [rbx + 0xc8]
14059dfda: call 0x1403258c0
14059dfdf: mov rax, qword ptr [rbx + 0xc8]
14059dfe6: lea rcx, [rax + 0x38]
14059dfea: mov byte ptr [rbx + 0xaa], 1
14059dff1: mov byte ptr [rbx + 0xab], 1
14059dff8: mov byte ptr [rbx + 0xac], 1
14059dfff: mov byte ptr [rbx + 0xad], 1
14059e006: mov byte ptr [rbx + 0xae], 1
14059e00d: mov byte ptr [rbx + 0xaf], 1
14059e014: mov byte ptr [rbx + 0xb0], 1
14059e01b: mov byte ptr [rbx + 0xb1], 1
14059e022: mov byte ptr [rbx + 0xb2], 1
14059e029: movabs rdx, 0x2e383ebb9376bfa1
14059e033: movabs r8, 0x96b847e2c17c833c
14059e03d: mov qword ptr [rbx + 0xe8], rcx
14059e044: call 0x1400dd6f0
14059e049: test rax, rax
14059e04c: je 0x14059e096
14059e04e: mov rax, qword ptr [rbx + 0xc8]
14059e055: cmp qword ptr [rax + 0x218], rdx
14059e05c: jbe 0x14059e096
14059e05e: mov rax, qword ptr [rbx + 0xc8]
14059e065: mov rax, qword ptr [rax + 0x210]
14059e06c: mov rax, qword ptr [rax + rdx*8]
14059e070: test rax, rax
14059e073: je 0x14059e096
14059e075: mov rcx, qword ptr [rbx + 0xc8]
14059e07c: mov rcx, qword ptr [rcx + 0x1e0]
14059e083: not rax
14059e086: lea rax, [rax + rax*2]
14059e08a: shl rax, 5
14059e08e: cmp qword ptr [rcx + rax + 0x20], 0
14059e094: jne 0x14059e0e1
14059e096: mov byte ptr [rbx + 0xaa], 1
14059e09d: mov byte ptr [rbx + 0xab], 1
14059e0a4: mov byte ptr [rbx + 0xac], 1
14059e0ab: mov byte ptr [rbx + 0xad], 1
14059e0b2: mov byte ptr [rbx + 0xae], 1
14059e0b9: mov byte ptr [rbx + 0xaf], 1
14059e0c0: mov byte ptr [rbx + 0xb0], 1
14059e0c7: mov byte ptr [rbx + 0xb1], 1
14059e0ce: mov byte ptr [rbx + 0xb2], 1
14059e0d5: mov rcx, qword ptr [rbx + 0xc8]
14059e0dc: call 0x140566ee0
14059e0e1: mov byte ptr [rbx + 0xaa], 1
14059e0e8: mov byte ptr [rbx + 0xab], 1
14059e0ef: mov byte ptr [rbx + 0xac], 1
14059e0f6: mov byte ptr [rbx + 0xad], 1
14059e0fd: mov byte ptr [rbx + 0xae], 1
14059e104: mov byte ptr [rbx + 0xaf], 1
14059e10b: mov byte ptr [rbx + 0xb0], 1
14059e112: mov byte ptr [rbx + 0xb1], 1
14059e119: mov byte ptr [rbx + 0xb2], 1
14059e120: movabs rdx, 0x47f704a227dd9e87
14059e12a: movabs r8, 0xc1485275b097c794
14059e134: mov rcx, qword ptr [rbx + 0xe8]
14059e13b: call 0x1400dd6f0
14059e140: test rax, rax
14059e143: je 0x14059e18d
14059e145: mov rax, qword ptr [rbx + 0xc8]
14059e14c: cmp qword ptr [rax + 0x218], rdx
14059e153: jbe 0x14059e18d
14059e155: mov rax, qword ptr [rbx + 0xc8]
14059e15c: mov rax, qword ptr [rax + 0x210]
14059e163: mov rax, qword ptr [rax + rdx*8]
14059e167: test rax, rax
14059e16a: je 0x14059e18d
14059e16c: mov rcx, qword ptr [rbx + 0xc8]
14059e173: mov rcx, qword ptr [rcx + 0x1e0]
14059e17a: not rax
14059e17d: lea rax, [rax + rax*2]
14059e181: shl rax, 5
14059e185: cmp qword ptr [rcx + rax + 0x20], 0
14059e18b: jne 0x14059e1d8
14059e18d: mov byte ptr [rbx + 0xaa], 1
14059e194: mov byte ptr [rbx + 0xab], 1
14059e19b: mov byte ptr [rbx + 0xac], 1
14059e1a2: mov byte ptr [rbx + 0xad], 1
14059e1a9: mov byte ptr [rbx + 0xae], 1
14059e1b0: mov byte ptr [rbx + 0xaf], 1
14059e1b7: mov byte ptr [rbx + 0xb0], 1
14059e1be: mov byte ptr [rbx + 0xb1], 1
14059e1c5: mov byte ptr [rbx + 0xb2], 1
14059e1cc: mov rcx, qword ptr [rbx + 0xc8]
14059e1d3: call 0x14056b920
14059e1d8: mov byte ptr [rbx + 0xaa], 1
14059e1df: mov byte ptr [rbx + 0xab], 1
14059e1e6: mov byte ptr [rbx + 0xac], 1
14059e1ed: mov byte ptr [rbx + 0xad], 1
14059e1f4: mov byte ptr [rbx + 0xae], 1
14059e1fb: mov byte ptr [rbx + 0xaf], 1
14059e202: mov byte ptr [rbx + 0xb0], 1
14059e209: mov byte ptr [rbx + 0xb1], 1
14059e210: mov byte ptr [rbx + 0xb2], 1
14059e217: movabs rdx, 0xb5215f98f93b0bd0
14059e221: movabs r8, 0xa52829b46b4ee6bb
14059e22b: mov rcx, qword ptr [rbx + 0xe8]
14059e232: call 0x1400dd6f0
14059e237: test rax, rax
14059e23a: je 0x14059e284
14059e23c: mov rax, qword ptr [rbx + 0xc8]
14059e243: cmp qword ptr [rax + 0x218], rdx
14059e24a: jbe 0x14059e284
14059e24c: mov rax, qword ptr [rbx + 0xc8]
14059e253: mov rax, qword ptr [rax + 0x210]
14059e25a: mov rax, qword ptr [rax + rdx*8]
14059e25e: test rax, rax
14059e261: je 0x14059e284
14059e263: mov rcx, qword ptr [rbx + 0xc8]
14059e26a: mov rcx, qword ptr [rcx + 0x1e0]
14059e271: not rax
14059e274: lea rax, [rax + rax*2]
14059e278: shl rax, 5
14059e27c: cmp qword ptr [rcx + rax + 0x20], 0
14059e282: jne 0x14059e2cf
14059e284: mov byte ptr [rbx + 0xaa], 1
14059e28b: mov byte ptr [rbx + 0xab], 1
14059e292: mov byte ptr [rbx + 0xac], 1
14059e299: mov byte ptr [rbx + 0xad], 1
14059e2a0: mov byte ptr [rbx + 0xae], 1
14059e2a7: mov byte ptr [rbx + 0xaf], 1
14059e2ae: mov byte ptr [rbx + 0xb0], 1
14059e2b5: mov byte ptr [rbx + 0xb1], 1
14059e2bc: mov byte ptr [rbx + 0xb2], 1
14059e2c3: mov rcx, qword ptr [rbx + 0xc8]
14059e2ca: call 0x1405857b0
14059e2cf: mov byte ptr [rbx + 0xaa], 1
14059e2d6: mov byte ptr [rbx + 0xab], 1
14059e2dd: mov byte ptr [rbx + 0xac], 1
14059e2e4: mov byte ptr [rbx + 0xad], 1
14059e2eb: mov byte ptr [rbx + 0xae], 1
14059e2f2: mov byte ptr [rbx + 0xaf], 1
14059e2f9: mov byte ptr [rbx + 0xb0], 1
14059e300: mov byte ptr [rbx + 0xb1], 1
14059e307: mov byte ptr [rbx + 0xb2], 1
14059e30e: movabs rdx, 0xbb7c1673aee764cb
14059e318: movabs r8, 0x250aee383945e2d6
14059e322: mov rcx, qword ptr [rbx + 0xe8]
14059e329: call 0x1400dd6f0
14059e32e: test rax, rax
14059e331: je 0x14059e37b
14059e333: mov rax, qword ptr [rbx + 0xc8]
14059e33a: cmp qword ptr [rax + 0x218], rdx
14059e341: jbe 0x14059e37b
14059e343: mov rax, qword ptr [rbx + 0xc8]
14059e34a: mov rax, qword ptr [rax + 0x210]
14059e351: mov rax, qword ptr [rax + rdx*8]
14059e355: test rax, rax
14059e358: je 0x14059e37b
14059e35a: mov rcx, qword ptr [rbx + 0xc8]
14059e361: mov rcx, qword ptr [rcx + 0x1e0]
14059e368: not rax
14059e36b: lea rax, [rax + rax*2]
14059e36f: shl rax, 5
14059e373: cmp qword ptr [rcx + rax + 0x20], 0
14059e379: jne 0x14059e3c6
14059e37b: mov byte ptr [rbx + 0xaa], 1
14059e382: mov byte ptr [rbx + 0xab], 1
14059e389: mov byte ptr [rbx + 0xac], 1
14059e390: mov byte ptr [rbx + 0xad], 1
14059e397: mov byte ptr [rbx + 0xae], 1
14059e39e: mov byte ptr [rbx + 0xaf], 1
14059e3a5: mov byte ptr [rbx + 0xb0], 1
14059e3ac: mov byte ptr [rbx + 0xb1], 1
14059e3b3: mov byte ptr [rbx + 0xb2], 1
14059e3ba: mov rcx, qword ptr [rbx + 0xc8]
14059e3c1: call 0x1405687a0
14059e3c6: mov byte ptr [rbx + 0xaa], 1
14059e3cd: mov byte ptr [rbx + 0xab], 1
14059e3d4: mov byte ptr [rbx + 0xac], 1
14059e3db: mov byte ptr [rbx + 0xad], 1
14059e3e2: mov byte ptr [rbx + 0xae], 1
14059e3e9: mov byte ptr [rbx + 0xaf], 1
14059e3f0: mov byte ptr [rbx + 0xb0], 1
14059e3f7: mov byte ptr [rbx + 0xb1], 1
14059e3fe: mov byte ptr [rbx + 0xb2], 1
14059e405: movabs rdx, 0x998e2a38a408421c
14059e40f: movabs r8, 0xf8878e80950c1d68
14059e419: mov rcx, qword ptr [rbx + 0xe8]
14059e420: call 0x1400dd6f0
14059e425: test rax, rax
14059e428: je 0x14059e472
14059e42a: mov rax, qword ptr [rbx + 0xc8]
14059e431: cmp qword ptr [rax + 0x218], rdx
14059e438: jbe 0x14059e472
14059e43a: mov rax, qword ptr [rbx + 0xc8]
14059e441: mov rax, qword ptr [rax + 0x210]
14059e448: mov rax, qword ptr [rax + rdx*8]
14059e44c: test rax, rax
14059e44f: je 0x14059e472
14059e451: mov rcx, qword ptr [rbx + 0xc8]
14059e458: mov rcx, qword ptr [rcx + 0x1e0]
14059e45f: not rax
14059e462: lea rax, [rax + rax*2]
14059e466: shl rax, 5
14059e46a: cmp qword ptr [rcx + rax + 0x20], 0
14059e470: jne 0x14059e4bd
14059e472: mov byte ptr [rbx + 0xaa], 1
14059e479: mov byte ptr [rbx + 0xab], 1
14059e480: mov byte ptr [rbx + 0xac], 1
14059e487: mov byte ptr [rbx + 0xad], 1
14059e48e: mov byte ptr [rbx + 0xae], 1
14059e495: mov byte ptr [rbx + 0xaf], 1
14059e49c: mov byte ptr [rbx + 0xb0], 1
14059e4a3: mov byte ptr [rbx + 0xb1], 1
14059e4aa: mov byte ptr [rbx + 0xb2], 1
14059e4b1: mov rcx, qword ptr [rbx + 0xc8]
14059e4b8: call 0x14056d1e0
14059e4bd: mov byte ptr [rbx + 0xaa], 1
14059e4c4: mov byte ptr [rbx + 0xab], 1
14059e4cb: mov byte ptr [rbx + 0xac], 1
14059e4d2: mov byte ptr [rbx + 0xad], 1
14059e4d9: mov byte ptr [rbx + 0xae], 1
14059e4e0: mov byte ptr [rbx + 0xaf], 1
14059e4e7: mov byte ptr [rbx + 0xb0], 1
14059e4ee: mov byte ptr [rbx + 0xb1], 1
14059e4f5: mov byte ptr [rbx + 0xb2], 1
14059e4fc: movabs rdx, 0x5206f06e93414f24
14059e506: movabs r8, 0x39f4e4f76f731964
14059e510: mov rcx, qword ptr [rbx + 0xe8]
14059e517: call 0x1400dd6f0
14059e51c: test rax, rax
14059e51f: je 0x14059e569
14059e521: mov rax, qword ptr [rbx + 0xc8]
14059e528: cmp qword ptr [rax + 0x218], rdx
14059e52f: jbe 0x14059e569
14059e531: mov rax, qword ptr [rbx + 0xc8]
14059e538: mov rax, qword ptr [rax + 0x210]
14059e53f: mov rax, qword ptr [rax + rdx*8]
14059e543: test rax, rax
14059e546: je 0x14059e569
14059e548: mov rcx, qword ptr [rbx + 0xc8]
14059e54f: mov rcx, qword ptr [rcx + 0x1e0]
14059e556: not rax
14059e559: lea rax, [rax + rax*2]
14059e55d: shl rax, 5
14059e561: cmp qword ptr [rcx + rax + 0x20], 0
14059e567: jne 0x14059e5b4
14059e569: mov byte ptr [rbx + 0xaa], 1
14059e570: mov byte ptr [rbx + 0xab], 1
14059e577: mov byte ptr [rbx + 0xac], 1
14059e57e: mov byte ptr [rbx + 0xad], 1
14059e585: mov byte ptr [rbx + 0xae], 1
14059e58c: mov byte ptr [rbx + 0xaf], 1
14059e593: mov byte ptr [rbx + 0xb0], 1
14059e59a: mov byte ptr [rbx + 0xb1], 1
14059e5a1: mov byte ptr [rbx + 0xb2], 1
14059e5a8: mov rcx, qword ptr [rbx + 0xc8]
14059e5af: call 0x1405715f0
14059e5b4: mov byte ptr [rbx + 0xaa], 1
14059e5bb: mov byte ptr [rbx + 0xab], 1
14059e5c2: mov byte ptr [rbx + 0xac], 1
14059e5c9: mov byte ptr [rbx + 0xad], 1
14059e5d0: mov byte ptr [rbx + 0xae], 1
14059e5d7: mov byte ptr [rbx + 0xaf], 1
14059e5de: mov byte ptr [rbx + 0xb0], 1
14059e5e5: mov byte ptr [rbx + 0xb1], 1
14059e5ec: mov byte ptr [rbx + 0xb2], 1
14059e5f3: movabs rdx, 0x7e476b44c6fa1e68
14059e5fd: movabs r8, 0xada174e41ab1f3e
14059e607: mov rcx, qword ptr [rbx + 0xe8]
14059e60e: call 0x1400dd6f0
14059e613: test rax, rax
14059e616: je 0x14059e660
14059e618: mov rax, qword ptr [rbx + 0xc8]
14059e61f: cmp qword ptr [rax + 0x218], rdx
14059e626: jbe 0x14059e660
14059e628: mov rax, qword ptr [rbx + 0xc8]
14059e62f: mov rax, qword ptr [rax + 0x210]
14059e636: mov rax, qword ptr [rax + rdx*8]
14059e63a: test rax, rax
14059e63d: je 0x14059e660
14059e63f: mov rcx, qword ptr [rbx + 0xc8]
14059e646: mov rcx, qword ptr [rcx + 0x1e0]
14059e64d: not rax
14059e650: lea rax, [rax + rax*2]
14059e654: shl rax, 5
14059e658: cmp qword ptr [rcx + rax + 0x20], 0
14059e65e: jne 0x14059e6ab
14059e660: mov byte ptr [rbx + 0xaa], 1
14059e667: mov byte ptr [rbx + 0xab], 1
14059e66e: mov byte ptr [rbx + 0xac], 1
14059e675: mov byte ptr [rbx + 0xad], 1
14059e67c: mov byte ptr [rbx + 0xae], 1
14059e683: mov byte ptr [rbx + 0xaf], 1
14059e68a: mov byte ptr [rbx + 0xb0], 1
14059e691: mov byte ptr [rbx + 0xb1], 1
14059e698: mov byte ptr [rbx + 0xb2], 1
14059e69f: mov rcx, qword ptr [rbx + 0xc8]
14059e6a6: call 0x140572eb0
14059e6ab: mov byte ptr [rbx + 0xaa], 1
14059e6b2: mov byte ptr [rbx + 0xab], 1
14059e6b9: mov byte ptr [rbx + 0xac], 1
14059e6c0: mov byte ptr [rbx + 0xad], 1
14059e6c7: mov byte ptr [rbx + 0xae], 1
14059e6ce: mov byte ptr [rbx + 0xaf], 1
14059e6d5: mov byte ptr [rbx + 0xb0], 1
14059e6dc: mov byte ptr [rbx + 0xb1], 1
14059e6e3: mov byte ptr [rbx + 0xb2], 1
14059e6ea: movabs rdx, 0x50c7f48d615ae830
14059e6f4: movabs r8, 0x360552332bdc4363
14059e6fe: mov rcx, qword ptr [rbx + 0xe8]
14059e705: call 0x1400dd6f0
14059e70a: test rax, rax
14059e70d: je 0x14059e757
14059e70f: mov rax, qword ptr [rbx + 0xc8]
14059e716: cmp qword ptr [rax + 0x218], rdx
14059e71d: jbe 0x14059e757
14059e71f: mov rax, qword ptr [rbx + 0xc8]
14059e726: mov rax, qword ptr [rax + 0x210]
14059e72d: mov rax, qword ptr [rax + rdx*8]
14059e731: test rax, rax
14059e734: je 0x14059e757
14059e736: mov rcx, qword ptr [rbx + 0xc8]
14059e73d: mov rcx, qword ptr [rcx + 0x1e0]
14059e744: not rax
14059e747: lea rax, [rax + rax*2]
14059e74b: shl rax, 5
14059e74f: cmp qword ptr [rcx + rax + 0x20], 0
14059e755: jne 0x14059e7a2
14059e757: mov byte ptr [rbx + 0xaa], 1
14059e75e: mov byte ptr [rbx + 0xab], 1
14059e765: mov byte ptr [rbx + 0xac], 1
14059e76c: mov byte ptr [rbx + 0xad], 1
14059e773: mov byte ptr [rbx + 0xae], 1
14059e77a: mov byte ptr [rbx + 0xaf], 1
14059e781: mov byte ptr [rbx + 0xb0], 1
14059e788: mov byte ptr [rbx + 0xb1], 1
14059e78f: mov byte ptr [rbx + 0xb2], 1
14059e796: mov rcx, qword ptr [rbx + 0xc8]
14059e79d: call 0x140583290
14059e7a2: mov byte ptr [rbx + 0xaa], 1
14059e7a9: mov byte ptr [rbx + 0xab], 1
14059e7b0: mov byte ptr [rbx + 0xac], 1
14059e7b7: mov byte ptr [rbx + 0xad], 1
14059e7be: mov byte ptr [rbx + 0xae], 1
14059e7c5: mov byte ptr [rbx + 0xaf], 1
14059e7cc: mov byte ptr [rbx + 0xb0], 1
14059e7d3: mov byte ptr [rbx + 0xb1], 1
14059e7da: mov byte ptr [rbx + 0xb2], 1
14059e7e1: movabs rdx, 0x76eef710ca5ee797
14059e7eb: movabs r8, 0x804a5169b0f39613
14059e7f5: mov rcx, qword ptr [rbx + 0xe8]
14059e7fc: call 0x1400dd6f0
14059e801: test rax, rax
14059e804: je 0x14059e84e
14059e806: mov rax, qword ptr [rbx + 0xc8]
14059e80d: cmp qword ptr [rax + 0x218], rdx
14059e814: jbe 0x14059e84e
14059e816: mov rax, qword ptr [rbx + 0xc8]
14059e81d: mov rax, qword ptr [rax + 0x210]
14059e824: mov rax, qword ptr [rax + rdx*8]
14059e828: test rax, rax
14059e82b: je 0x14059e84e
14059e82d: mov rcx, qword ptr [rbx + 0xc8]
14059e834: mov rcx, qword ptr [rcx + 0x1e0]
14059e83b: not rax
14059e83e: lea rax, [rax + rax*2]
14059e842: shl rax, 5
14059e846: cmp qword ptr [rcx + rax + 0x20], 0
14059e84c: jne 0x14059e899
14059e84e: mov byte ptr [rbx + 0xaa], 1
14059e855: mov byte ptr [rbx + 0xab], 1
14059e85c: mov byte ptr [rbx + 0xac], 1
14059e863: mov byte ptr [rbx + 0xad], 1
14059e86a: mov byte ptr [rbx + 0xae], 1
14059e871: mov byte ptr [rbx + 0xaf], 1
14059e878: mov byte ptr [rbx + 0xb0], 1
14059e87f: mov byte ptr [rbx + 0xb1], 1
14059e886: mov byte ptr [rbx + 0xb2], 1
14059e88d: mov rcx, qword ptr [rbx + 0xc8]
14059e894: call 0x140578b80
14059e899: mov byte ptr [rbx + 0xaa], 1
14059e8a0: mov byte ptr [rbx + 0xab], 1
14059e8a7: mov byte ptr [rbx + 0xac], 1
14059e8ae: mov byte ptr [rbx + 0xad], 1
14059e8b5: mov byte ptr [rbx + 0xae], 1
14059e8bc: mov byte ptr [rbx + 0xaf], 1
14059e8c3: mov byte ptr [rbx + 0xb0], 1
14059e8ca: mov byte ptr [rbx + 0xb1], 1
14059e8d1: mov byte ptr [rbx + 0xb2], 1
14059e8d8: movabs rdx, 0xd8f93d064ea9d61c
14059e8e2: movabs r8, 0x6447617066d9832f
14059e8ec: mov rcx, qword ptr [rbx + 0xe8]
14059e8f3: call 0x1400dd6f0
14059e8f8: test rax, rax
14059e8fb: je 0x14059e945
14059e8fd: mov rax, qword ptr [rbx + 0xc8]
14059e904: cmp qword ptr [rax + 0x218], rdx
14059e90b: jbe 0x14059e945
14059e90d: mov rax, qword ptr [rbx + 0xc8]
14059e914: mov rax, qword ptr [rax + 0x210]
14059e91b: mov rax, qword ptr [rax + rdx*8]
14059e91f: test rax, rax
14059e922: je 0x14059e945
14059e924: mov rcx, qword ptr [rbx + 0xc8]
14059e92b: mov rcx, qword ptr [rcx + 0x1e0]
14059e932: not rax
14059e935: lea rax, [rax + rax*2]
14059e939: shl rax, 5
14059e93d: cmp qword ptr [rcx + rax + 0x20], 0
14059e943: jne 0x14059e990
14059e945: mov byte ptr [rbx + 0xaa], 1
14059e94c: mov byte ptr [rbx + 0xab], 1
14059e953: mov byte ptr [rbx + 0xac], 1
14059e95a: mov byte ptr [rbx + 0xad], 1
14059e961: mov byte ptr [rbx + 0xae], 1
14059e968: mov byte ptr [rbx + 0xaf], 1
14059e96f: mov byte ptr [rbx + 0xb0], 1
14059e976: mov byte ptr [rbx + 0xb1], 1
14059e97d: mov byte ptr [rbx + 0xb2], 1
14059e984: mov rcx, qword ptr [rbx + 0xc8]
14059e98b: call 0x1405734e0
14059e990: mov byte ptr [rbx + 0xaa], 1
14059e997: mov byte ptr [rbx + 0xab], 1
14059e99e: mov byte ptr [rbx + 0xac], 1
14059e9a5: mov byte ptr [rbx + 0xad], 1
14059e9ac: mov byte ptr [rbx + 0xae], 1
14059e9b3: mov byte ptr [rbx + 0xaf], 1
14059e9ba: mov byte ptr [rbx + 0xb0], 1
14059e9c1: mov byte ptr [rbx + 0xb1], 1
14059e9c8: mov byte ptr [rbx + 0xb2], 1
14059e9cf: movabs rdx, 0x53692c24f21fcb2f
14059e9d9: movabs r8, 0xbc74dd13ef48076b
14059e9e3: mov rcx, qword ptr [rbx + 0xe8]
14059e9ea: call 0x1400dd6f0
14059e9ef: test rax, rax
14059e9f2: je 0x14059ea3c
14059e9f4: mov rax, qword ptr [rbx + 0xc8]
14059e9fb: cmp qword ptr [rax + 0x218], rdx
14059ea02: jbe 0x14059ea3c
14059ea04: mov rax, qword ptr [rbx + 0xc8]
14059ea0b: mov rax, qword ptr [rax + 0x210]
14059ea12: mov rax, qword ptr [rax + rdx*8]
14059ea16: test rax, rax
14059ea19: je 0x14059ea3c
14059ea1b: mov rcx, qword ptr [rbx + 0xc8]
14059ea22: mov rcx, qword ptr [rcx + 0x1e0]
14059ea29: not rax
14059ea2c: lea rax, [rax + rax*2]
14059ea30: shl rax, 5
14059ea34: cmp qword ptr [rcx + rax + 0x20], 0
14059ea3a: jne 0x14059ea87
14059ea3c: mov byte ptr [rbx + 0xaa], 1
14059ea43: mov byte ptr [rbx + 0xab], 1
14059ea4a: mov byte ptr [rbx + 0xac], 1
14059ea51: mov byte ptr [rbx + 0xad], 1
14059ea58: mov byte ptr [rbx + 0xae], 1
14059ea5f: mov byte ptr [rbx + 0xaf], 1
14059ea66: mov byte ptr [rbx + 0xb0], 1
14059ea6d: mov byte ptr [rbx + 0xb1], 1
14059ea74: mov byte ptr [rbx + 0xb2], 1
14059ea7b: mov rcx, qword ptr [rbx + 0xc8]
14059ea82: call 0x140570fc0
14059ea87: mov rax, qword ptr [rbp + 0x37b0]
14059ea8e: lea r14, [rax + 0xe0]
14059ea95: mov byte ptr [rbx + 0xaa], 1
14059ea9c: mov byte ptr [rbx + 0xab], 1
14059eaa3: mov byte ptr [rbx + 0xac], 1
14059eaaa: mov byte ptr [rbx + 0xad], 1
14059eab1: mov byte ptr [rbx + 0xae], 1
14059eab8: mov byte ptr [rbx + 0xaf], 1
14059eabf: mov byte ptr [rbx + 0xb0], 1
14059eac6: mov byte ptr [rbx + 0xb1], 1
14059eacd: mov byte ptr [rbx + 0xb2], 1
14059ead4: mov rcx, r14
14059ead7: call 0x14090df40
14059eadc: mov byte ptr [rbx + 0xaa], 1
14059eae3: mov byte ptr [rbx + 0xab], 1
14059eaea: mov byte ptr [rbx + 0xac], 1
14059eaf1: mov byte ptr [rbx + 0xad], 1
14059eaf8: mov byte ptr [rbx + 0xae], 1
14059eaff: mov byte ptr [rbx + 0xaf], 1
14059eb06: mov byte ptr [rbx + 0xb0], 1
14059eb0d: mov byte ptr [rbx + 0xb1], 1
14059eb14: mov byte ptr [rbx + 0xb2], 1
14059eb1b: lea rcx, [rbx + 0xb00]
14059eb22: mov edx, eax
14059eb24: call 0x1408ba0f0
14059eb29: mov byte ptr [rbx + 0xaa], 1
14059eb30: mov byte ptr [rbx + 0xab], 1
14059eb37: mov byte ptr [rbx + 0xac], 1
14059eb3e: mov byte ptr [rbx + 0xad], 1
14059eb45: mov byte ptr [rbx + 0xae], 1
14059eb4c: mov byte ptr [rbx + 0xaf], 1
14059eb53: mov byte ptr [rbx + 0xb0], 1
14059eb5a: mov byte ptr [rbx + 0xb1], 1
14059eb61: mov byte ptr [rbx + 0xb2], 1
14059eb68: lea rdx, [rbx + 0xb00]
14059eb6f: mov rcx, qword ptr [rbx + 0xc8]
14059eb76: call 0x140131d60
14059eb7b: mov byte ptr [rbx + 0xaa], 1
14059eb82: mov byte ptr [rbx + 0xab], 1
14059eb89: mov byte ptr [rbx + 0xac], 1
14059eb90: mov byte ptr [rbx + 0xad], 1
14059eb97: mov byte ptr [rbx + 0xae], 1
14059eb9e: mov byte ptr [rbx + 0xaf], 1
14059eba5: mov byte ptr [rbx + 0xb0], 1
14059ebac: mov byte ptr [rbx + 0xb1], 1
14059ebb3: mov byte ptr [rbx + 0xb2], 1
14059ebba: mov rcx, rax
14059ebbd: call 0x1409457c0
14059ebc2: mov byte ptr [rbx + 0xaa], 1
14059ebc9: mov byte ptr [rbx + 0xab], 1
14059ebd0: mov byte ptr [rbx + 0xac], 1
14059ebd7: mov byte ptr [rbx + 0xad], 1
14059ebde: mov byte ptr [rbx + 0xae], 1
14059ebe5: mov byte ptr [rbx + 0xaf], 1
14059ebec: mov byte ptr [rbx + 0xb0], 1
14059ebf3: mov byte ptr [rbx + 0xb1], 1
14059ebfa: mov byte ptr [rbx + 0xb2], 1
14059ec01: mov rcx, rax
14059ec04: call 0x140943e10
14059ec09: mov rdx, qword ptr [rbx + 0x138]
14059ec10: add rdx, 0x10
14059ec14: mov byte ptr [rbx + 0xaa], 1
14059ec1b: mov byte ptr [rbx + 0xab], 1
14059ec22: mov byte ptr [rbx + 0xac], 1
14059ec29: mov byte ptr [rbx + 0xad], 1
14059ec30: mov byte ptr [rbx + 0xae], 1
14059ec37: mov byte ptr [rbx + 0xaf], 1
14059ec3e: mov byte ptr [rbx + 0xb0], 1
14059ec45: mov byte ptr [rbx + 0xb1], 1
14059ec4c: mov byte ptr [rbx + 0xb2], 1
14059ec53: mov rcx, rax
14059ec56: mov r8, qword ptr [rbp + 0x37b0]
14059ec5d: call 0x140940190
14059ec62: mov rsi, rax
14059ec65: mov rdx, qword ptr [rbx + 0x138]
14059ec6c: add rdx, 0x10
14059ec70: mov byte ptr [rbx + 0xaa], 1
14059ec77: mov byte ptr [rbx + 0xab], 1
14059ec7e: mov byte ptr [rbx + 0xac], 1
14059ec85: mov byte ptr [rbx + 0xad], 1
14059ec8c: mov byte ptr [rbx + 0xae], 1
14059ec93: mov byte ptr [rbx + 0xaf], 1
14059ec9a: mov byte ptr [rbx + 0xb0], 1
14059eca1: mov byte ptr [rbx + 0xb1], 1
14059eca8: mov byte ptr [rbx + 0xb2], 1
14059ecaf: lea rcx, [rbx + 0xb00]
14059ecb6: call 0x140c02570
14059ecbb: movups xmm0, xmmword ptr [rbx + 0xb40]
14059ecc2: movups xmmword ptr [rbx + 0x487], xmm0
14059ecc9: movups xmm0, xmmword ptr [rbx + 0xb00]
14059ecd0: movups xmm1, xmmword ptr [rbx + 0xb10]
14059ecd7: movups xmm2, xmmword ptr [rbx + 0xb20]
14059ecde: movups xmm3, xmmword ptr [rbx + 0xb30]
14059ece5: movups xmmword ptr [rbx + 0x477], xmm3
14059ecec: movups xmmword ptr [rbx + 0x467], xmm2
14059ecf3: movups xmmword ptr [rbx + 0x457], xmm1
14059ecfa: movups xmmword ptr [rbx + 0x447], xmm0
14059ed01: mov byte ptr [rbx + 0xb00], 0
14059ed08: movups xmm0, xmmword ptr [rbx + 0x440]
14059ed0f: movups xmmword ptr [rbx + 0xb01], xmm0
14059ed16: movups xmm0, xmmword ptr [rbx + 0x450]
14059ed1d: movups xmmword ptr [rbx + 0xb11], xmm0
14059ed24: movups xmm0, xmmword ptr [rbx + 0x460]
14059ed2b: movups xmmword ptr [rbx + 0xb21], xmm0
14059ed32: movups xmm0, xmmword ptr [rbx + 0x470]
14059ed39: movups xmmword ptr [rbx + 0xb31], xmm0
14059ed40: movups xmm0, xmmword ptr [rbx + 0x480]
14059ed47: movups xmmword ptr [rbx + 0xb41], xmm0
14059ed4e: mov rax, qword ptr [rbx + 0x48f]
14059ed55: mov qword ptr [rbx + 0xb50], rax
14059ed5c: mov byte ptr [rbx + 0xaa], 1
14059ed63: mov byte ptr [rbx + 0xab], 1
14059ed6a: mov byte ptr [rbx + 0xac], 1
14059ed71: mov byte ptr [rbx + 0xad], 1
14059ed78: mov byte ptr [rbx + 0xae], 1
14059ed7f: mov byte ptr [rbx + 0xaf], 1
14059ed86: mov byte ptr [rbx + 0xb0], 1
14059ed8d: mov byte ptr [rbx + 0xb1], 1
14059ed94: mov byte ptr [rbx + 0xb2], 1
14059ed9b: lea rdx, [rbx + 0xb00]
14059eda2: mov rcx, rsi
14059eda5: call 0x14011f840
14059edaa: mov rdx, qword ptr [rbx + 0x138]
14059edb1: lock inc qword ptr [rdx]
14059edb5: jle 0x1405a7040
14059edbb: mov byte ptr [rbx + 0xaa], 1
14059edc2: mov byte ptr [rbx + 0xab], 1
14059edc9: mov byte ptr [rbx + 0xac], 1
14059edd0: mov byte ptr [rbx + 0xad], 1
14059edd7: mov byte ptr [rbx + 0xae], 1
14059edde: mov byte ptr [rbx + 0xaf], 1
14059ede5: mov byte ptr [rbx + 0xb0], 1
14059edec: mov byte ptr [rbx + 0xb1], 1
14059edf3: mov byte ptr [rbx + 0xb2], 1
14059edfa: mov rcx, rax
14059edfd: call 0x14011cb90
14059ee02: mov rsi, rax
14059ee05: mov byte ptr [rbx + 0xaa], 1
14059ee0c: mov byte ptr [rbx + 0xab], 1
14059ee13: mov byte ptr [rbx + 0xac], 1
14059ee1a: mov byte ptr [rbx + 0xad], 1
14059ee21: mov byte ptr [rbx + 0xae], 1
14059ee28: mov byte ptr [rbx + 0xaf], 1
14059ee2f: mov byte ptr [rbx + 0xb0], 0
14059ee36: mov byte ptr [rbx + 0xb1], 1
14059ee3d: mov byte ptr [rbx + 0xb2], 1
14059ee44: mov rcx, qword ptr [rbx + 0x2b8]
14059ee4b: mov rdx, qword ptr [rbx + 0x2b0]
14059ee52: call 0x140987cb0
14059ee57: mov r8, rdx
14059ee5a: mov byte ptr [rbx + 0xaa], 1
14059ee61: mov byte ptr [rbx + 0xab], 1
14059ee68: mov byte ptr [rbx + 0xac], 1
14059ee6f: mov byte ptr [rbx + 0xad], 1
14059ee76: mov byte ptr [rbx + 0xae], 1
14059ee7d: mov byte ptr [rbx + 0xaf], 1
14059ee84: mov byte ptr [rbx + 0xb0], 0
14059ee8b: mov byte ptr [rbx + 0xb1], 1
14059ee92: mov byte ptr [rbx + 0xb2], 1
14059ee99: mov rcx, rsi
14059ee9c: mov rdx, rax
14059ee9f: call 0x140123e00
14059eea4: mov rsi, rax
14059eea7: movaps xmm8, xmmword ptr [rip + 0x245ff51]
14059eeaf: movaps xmmword ptr [rbx + 0x470], xmm8
14059eeb7: movaps xmm9, xmmword ptr [rip + 0x245ff31]
14059eebf: movaps xmmword ptr [rbx + 0x460], xmm9
14059eec7: movaps xmm10, xmmword ptr [rip + 0x245ff11]
14059eecf: movaps xmmword ptr [rbx + 0x450], xmm10
14059eed7: movaps xmm11, xmmword ptr [rip + 0x245fef1]
14059eedf: movaps xmmword ptr [rbx + 0x440], xmm11
14059eee7: movaps xmmword ptr [rbx + 0x480], xmm11
14059eeef: movaps xmmword ptr [rbx + 0x490], xmm10
14059eef7: movaps xmmword ptr [rbx + 0x4a0], xmm9
14059eeff: movaps xmmword ptr [rbx + 0x4b0], xmm8
14059ef07: movaps xmmword ptr [rbx + 0x4c0], xmm11
14059ef0f: movaps xmmword ptr [rbx + 0x4d0], xmm10
14059ef17: movaps xmmword ptr [rbx + 0x4e0], xmm9
14059ef1f: movaps xmmword ptr [rbx + 0x4f0], xmm8
14059ef27: movaps xmmword ptr [rbx + 0x530], xmm8
14059ef2f: movaps xmmword ptr [rbx + 0x520], xmm9
14059ef37: movaps xmmword ptr [rbx + 0x510], xmm10
14059ef3f: movaps xmmword ptr [rbx + 0x500], xmm11
14059ef47: mov dword ptr [rbx + 0xc08], 0
14059ef51: lea rdi, [rbx + 0xb00]
14059ef58: lea rdx, [rbx + 0x440]
14059ef5f: mov r8d, 0x100
14059ef65: mov rcx, rdi
14059ef68: call 0x142923c00
14059ef6d: mov qword ptr [rbx + 0xc00], 0
14059ef78: mov byte ptr [rbx + 0xaa], 1
14059ef7f: mov byte ptr [rbx + 0xab], 1
14059ef86: mov byte ptr [rbx + 0xac], 1
14059ef8d: mov byte ptr [rbx + 0xad], 1
14059ef94: mov byte ptr [rbx + 0xae], 1
14059ef9b: mov byte ptr [rbx + 0xaf], 1
14059efa2: mov byte ptr [rbx + 0xb0], 0
14059efa9: mov byte ptr [rbx + 0xb1], 1
14059efb0: mov byte ptr [rbx + 0xb2], 1
14059efb7: mov rcx, rsi
14059efba: mov rdx, rdi
14059efbd: call 0x140135cc0
14059efc2: mov byte ptr [rbx + 0xaa], 1
14059efc9: mov byte ptr [rbx + 0xab], 1
14059efd0: mov byte ptr [rbx + 0xac], 1
14059efd7: mov byte ptr [rbx + 0xad], 1
14059efde: mov byte ptr [rbx + 0xae], 1
14059efe5: mov byte ptr [rbx + 0xaf], 1
14059efec: mov byte ptr [rbx + 0xb0], 0
14059eff3: mov byte ptr [rbx + 0xb1], 1
14059effa: mov byte ptr [rbx + 0xb2], 1
14059f001: mov rcx, rax
14059f004: call 0x1409440c0
14059f009: mov rsi, rax
14059f00c: mov rax, qword ptr [rbp + 0x37b8]
14059f013: mov rcx, qword ptr [rax]
14059f016: lock inc qword ptr [rcx]
14059f01a: jle 0x1405a7040
14059f020: mov byte ptr [rbx + 0xaa], 1
14059f027: mov byte ptr [rbx + 0xab], 1
14059f02e: mov byte ptr [rbx + 0xac], 1
14059f035: mov byte ptr [rbx + 0xad], 1
14059f03c: mov byte ptr [rbx + 0xae], 1
14059f043: mov byte ptr [rbx + 0xaf], 1
14059f04a: mov byte ptr [rbx + 0xb0], 0
14059f051: mov byte ptr [rbx + 0xb1], 1
14059f058: mov byte ptr [rbx + 0xb2], 1
14059f05f: call 0x14067dd80
14059f064: mov byte ptr [rbx + 0xaa], 1
14059f06b: mov byte ptr [rbx + 0xab], 1
14059f072: mov byte ptr [rbx + 0xac], 1
14059f079: mov byte ptr [rbx + 0xad], 1
14059f080: mov byte ptr [rbx + 0xae], 1
14059f087: mov byte ptr [rbx + 0xaf], 1
14059f08e: mov byte ptr [rbx + 0xb0], 0
14059f095: mov byte ptr [rbx + 0xb1], 1
14059f09c: mov byte ptr [rbx + 0xb2], 1
14059f0a3: mov rcx, rsi
14059f0a6: mov rdx, rax
14059f0a9: call 0x140127e40
14059f0ae: mov rsi, rax
14059f0b1: mov byte ptr [rbx + 0xaa], 1
14059f0b8: mov byte ptr [rbx + 0xab], 1
14059f0bf: mov byte ptr [rbx + 0xac], 1
14059f0c6: mov byte ptr [rbx + 0xad], 1
14059f0cd: mov byte ptr [rbx + 0xae], 1
14059f0d4: mov byte ptr [rbx + 0xaf], 1
14059f0db: mov byte ptr [rbx + 0xb0], 0
14059f0e2: mov byte ptr [rbx + 0xb1], 1
14059f0e9: mov byte ptr [rbx + 0xb2], 1
14059f0f0: lea rcx, [rbx + 0xb00]
14059f0f7: call 0x14093fce0
14059f0fc: mov byte ptr [rbx + 0xaa], 1
14059f103: mov byte ptr [rbx + 0xab], 1
14059f10a: mov byte ptr [rbx + 0xac], 1
14059f111: mov byte ptr [rbx + 0xad], 1
14059f118: mov byte ptr [rbx + 0xae], 1
14059f11f: mov byte ptr [rbx + 0xaf], 1
14059f126: mov byte ptr [rbx + 0xb0], 0
14059f12d: mov byte ptr [rbx + 0xb1], 1
14059f134: mov byte ptr [rbx + 0xb2], 1
14059f13b: lea rdx, [rbx + 0xb00]
14059f142: mov rcx, rsi
14059f145: call 0x140128a10
14059f14a: mov rsi, rax
14059f14d: mov byte ptr [rbx + 0xaa], 1
14059f154: mov byte ptr [rbx + 0xab], 1
14059f15b: mov byte ptr [rbx + 0xac], 1
14059f162: mov byte ptr [rbx + 0xad], 1
14059f169: mov byte ptr [rbx + 0xae], 1
14059f170: mov byte ptr [rbx + 0xaf], 1
14059f177: mov byte ptr [rbx + 0xb0], 0
14059f17e: mov byte ptr [rbx + 0xb1], 1
14059f185: mov byte ptr [rbx + 0xb2], 1
14059f18c: lea rcx, [rbx + 0xb00]
14059f193: mov rdx, qword ptr [rbp + 0x37c0]
14059f19a: call 0x140353390
14059f19f: mov byte ptr [rbx + 0xaa], 1
14059f1a6: mov byte ptr [rbx + 0xab], 1
14059f1ad: mov byte ptr [rbx + 0xac], 1
14059f1b4: mov byte ptr [rbx + 0xad], 1
14059f1bb: mov byte ptr [rbx + 0xae], 1
14059f1c2: mov byte ptr [rbx + 0xaf], 1
14059f1c9: mov byte ptr [rbx + 0xb0], 0
14059f1d0: mov byte ptr [rbx + 0xb1], 1
14059f1d7: mov byte ptr [rbx + 0xb2], 1
14059f1de: lea rdx, [rbx + 0xb00]
14059f1e5: lea r8, [rbx + 0x3220]
14059f1ec: mov rcx, rsi
14059f1ef: call 0x140942f80
14059f1f4: mov rsi, rax
14059f1f7: lea rdi, [rbx + 0xb00]
14059f1fe: lea rdx, [rbx + 0x3220]
14059f205: mov r8d, 0x4e0
14059f20b: mov rcx, rdi
14059f20e: call 0x142923c00
14059f213: mov byte ptr [rbx + 0xaa], 1
14059f21a: mov byte ptr [rbx + 0xab], 1
14059f221: mov byte ptr [rbx + 0xac], 1
14059f228: mov byte ptr [rbx + 0xad], 1
14059f22f: mov byte ptr [rbx + 0xae], 0
14059f236: mov byte ptr [rbx + 0xaf], 1
14059f23d: mov byte ptr [rbx + 0xb0], 0
14059f244: mov byte ptr [rbx + 0xb1], 1
14059f24b: mov byte ptr [rbx + 0xb2], 1
14059f252: mov rcx, rsi
14059f255: mov rdx, rdi
14059f258: call 0x140133510
14059f25d: mov rsi, rax
14059f260: mov byte ptr [rbx + 0xaa], 1
14059f267: mov byte ptr [rbx + 0xab], 1
14059f26e: mov byte ptr [rbx + 0xac], 1
14059f275: mov byte ptr [rbx + 0xad], 1
14059f27c: mov byte ptr [rbx + 0xae], 0
14059f283: mov byte ptr [rbx + 0xaf], 1
14059f28a: mov byte ptr [rbx + 0xb0], 0
14059f291: mov byte ptr [rbx + 0xb1], 1
14059f298: mov byte ptr [rbx + 0xb2], 1
14059f29f: lea rcx, [rbx + 0xb00]
14059f2a6: call 0x1408ae7f0
14059f2ab: mov byte ptr [rbx + 0xaa], 1
14059f2b2: mov byte ptr [rbx + 0xab], 1
14059f2b9: mov byte ptr [rbx + 0xac], 1
14059f2c0: mov byte ptr [rbx + 0xad], 1
14059f2c7: mov byte ptr [rbx + 0xae], 0
14059f2ce: mov byte ptr [rbx + 0xaf], 1
14059f2d5: mov byte ptr [rbx + 0xb0], 0
14059f2dc: mov byte ptr [rbx + 0xb1], 1
14059f2e3: mov byte ptr [rbx + 0xb2], 1
14059f2ea: lea rdx, [rbx + 0xb00]
14059f2f1: mov rcx, rsi
14059f2f4: call 0x140122c30
14059f2f9: mov rsi, rax
14059f2fc: mov byte ptr [rbx + 0xaa], 1
14059f303: mov byte ptr [rbx + 0xab], 1
14059f30a: mov byte ptr [rbx + 0xac], 1
14059f311: mov byte ptr [rbx + 0xad], 1
14059f318: mov byte ptr [rbx + 0xae], 0
14059f31f: mov byte ptr [rbx + 0xaf], 1
14059f326: mov byte ptr [rbx + 0xb0], 0
14059f32d: mov byte ptr [rbx + 0xb1], 1
14059f334: mov byte ptr [rbx + 0xb2], 1
14059f33b: lea rcx, [rbx + 0xb00]
14059f342: call 0x1408705b0
14059f347: mov byte ptr [rbx + 0xaa], 1
14059f34e: mov byte ptr [rbx + 0xab], 1
14059f355: mov byte ptr [rbx + 0xac], 1
14059f35c: mov byte ptr [rbx + 0xad], 1
14059f363: mov byte ptr [rbx + 0xae], 0
14059f36a: mov byte ptr [rbx + 0xaf], 1
14059f371: mov byte ptr [rbx + 0xb0], 0
14059f378: mov byte ptr [rbx + 0xb1], 1
14059f37f: mov byte ptr [rbx + 0xb2], 1
14059f386: lea rdx, [rbx + 0xb00]
14059f38d: mov rcx, rsi
14059f390: call 0x14012fa90
14059f395: mov byte ptr [rbx + 0xaa], 1
14059f39c: mov byte ptr [rbx + 0xab], 1
14059f3a3: mov byte ptr [rbx + 0xac], 1
14059f3aa: mov byte ptr [rbx + 0xad], 1
14059f3b1: mov byte ptr [rbx + 0xae], 0
14059f3b8: mov byte ptr [rbx + 0xaf], 1
14059f3bf: mov byte ptr [rbx + 0xb0], 0
14059f3c6: mov byte ptr [rbx + 0xb1], 1
14059f3cd: mov byte ptr [rbx + 0xb2], 1
14059f3d4: mov rcx, rax
14059f3d7: xor edx, edx
14059f3d9: call 0x14011d170
14059f3de: mov byte ptr [rbx + 0xaa], 1
14059f3e5: mov byte ptr [rbx + 0xab], 1
14059f3ec: mov byte ptr [rbx + 0xac], 1
14059f3f3: mov byte ptr [rbx + 0xad], 1
14059f3fa: mov byte ptr [rbx + 0xae], 0
14059f401: mov byte ptr [rbx + 0xaf], 1
14059f408: mov byte ptr [rbx + 0xb0], 0
14059f40f: mov byte ptr [rbx + 0xb1], 1
14059f416: mov byte ptr [rbx + 0xb2], 1
14059f41d: xorps xmm1, xmm1
14059f420: xorps xmm2, xmm2
14059f423: mov rcx, rax
14059f426: call 0x140138520
14059f42b: mov rsi, rax
14059f42e: mov rdx, qword ptr [rbx + 0x138]
14059f435: add rdx, 0x10
14059f439: mov byte ptr [rbx + 0xaa], 1
14059f440: mov byte ptr [rbx + 0xab], 1
14059f447: mov byte ptr [rbx + 0xac], 1
14059f44e: mov byte ptr [rbx + 0xad], 1
14059f455: mov byte ptr [rbx + 0xae], 0
14059f45c: mov byte ptr [rbx + 0xaf], 1
14059f463: mov byte ptr [rbx + 0xb0], 0
14059f46a: mov byte ptr [rbx + 0xb1], 1
14059f471: mov byte ptr [rbx + 0xb2], 1
14059f478: lea rcx, [rbx + 0xb00]
14059f47f: call 0x1409917d0
14059f484: mov byte ptr [rbx + 0xaa], 1
14059f48b: mov byte ptr [rbx + 0xab], 1
14059f492: mov byte ptr [rbx + 0xac], 1
14059f499: mov byte ptr [rbx + 0xad], 1
14059f4a0: mov byte ptr [rbx + 0xae], 0
14059f4a7: mov byte ptr [rbx + 0xaf], 1
14059f4ae: mov byte ptr [rbx + 0xb0], 0
14059f4b5: mov byte ptr [rbx + 0xb1], 1
14059f4bc: mov byte ptr [rbx + 0xb2], 1
14059f4c3: lea rdx, [rbx + 0xb00]
14059f4ca: mov rcx, rsi
14059f4cd: call 0x1401159d0
14059f4d2: mov byte ptr [rbx + 0xaa], 1
14059f4d9: mov byte ptr [rbx + 0xab], 1
14059f4e0: mov byte ptr [rbx + 0xac], 1
14059f4e7: mov byte ptr [rbx + 0xad], 1
14059f4ee: mov byte ptr [rbx + 0xae], 0
14059f4f5: mov byte ptr [rbx + 0xaf], 1
14059f4fc: mov byte ptr [rbx + 0xb0], 0
14059f503: mov byte ptr [rbx + 0xb1], 1
14059f50a: mov byte ptr [rbx + 0xb2], 1
14059f511: mov rcx, rax
14059f514: call 0x1404936b0
14059f519: mov byte ptr [rbx + 0xaa], 1
14059f520: mov byte ptr [rbx + 0xab], 1
14059f527: mov byte ptr [rbx + 0xac], 1
14059f52e: mov byte ptr [rbx + 0xad], 1
14059f535: mov byte ptr [rbx + 0xae], 0
14059f53c: mov byte ptr [rbx + 0xaf], 1
14059f543: mov byte ptr [rbx + 0xb0], 0
14059f54a: mov byte ptr [rbx + 0xb1], 1
14059f551: mov byte ptr [rbx + 0xb2], 1
14059f558: mov rcx, rax
14059f55b: call 0x140493120
14059f560: mov byte ptr [rbx + 0xaa], 1
14059f567: mov byte ptr [rbx + 0xab], 1
14059f56e: mov byte ptr [rbx + 0xac], 1
14059f575: mov byte ptr [rbx + 0xad], 1
14059f57c: mov byte ptr [rbx + 0xae], 0
14059f583: mov byte ptr [rbx + 0xaf], 1
14059f58a: mov byte ptr [rbx + 0xb0], 0
14059f591: mov byte ptr [rbx + 0xb1], 1
14059f598: mov byte ptr [rbx + 0xb2], 1
14059f59f: mov rcx, rax
14059f5a2: call 0x140945f50
14059f5a7: mov byte ptr [rbx + 0xaa], 1
14059f5ae: mov byte ptr [rbx + 0xab], 1
14059f5b5: mov byte ptr [rbx + 0xac], 1
14059f5bc: mov byte ptr [rbx + 0xad], 1
14059f5c3: mov byte ptr [rbx + 0xae], 0
14059f5ca: mov byte ptr [rbx + 0xaf], 1
14059f5d1: mov byte ptr [rbx + 0xb0], 0
14059f5d8: mov byte ptr [rbx + 0xb1], 1
14059f5df: mov byte ptr [rbx + 0xb2], 1
14059f5e6: mov rcx, rax
14059f5e9: call 0x140945a20
14059f5ee: mov byte ptr [rbx + 0xaa], 1
14059f5f5: mov byte ptr [rbx + 0xab], 1
14059f5fc: mov byte ptr [rbx + 0xac], 1
14059f603: mov byte ptr [rbx + 0xad], 1
14059f60a: mov byte ptr [rbx + 0xae], 0
14059f611: mov byte ptr [rbx + 0xaf], 1
14059f618: mov byte ptr [rbx + 0xb0], 0
14059f61f: mov byte ptr [rbx + 0xb1], 1
14059f626: mov byte ptr [rbx + 0xb2], 1
14059f62d: mov rcx, rax
14059f630: call 0x1409455f0
14059f635: mov byte ptr [rbx + 0xaa], 1
14059f63c: mov byte ptr [rbx + 0xab], 1
14059f643: mov byte ptr [rbx + 0xac], 1
14059f64a: mov byte ptr [rbx + 0xad], 1
14059f651: mov byte ptr [rbx + 0xae], 0
14059f658: mov byte ptr [rbx + 0xaf], 1
14059f65f: mov byte ptr [rbx + 0xb0], 0
14059f666: mov byte ptr [rbx + 0xb1], 1
14059f66d: mov byte ptr [rbx + 0xb2], 1
14059f674: mov rcx, rax
14059f677: mov rdx, qword ptr [rbp + 0x37b0]
14059f67e: call 0x140942c80
14059f683: mov byte ptr [rbx + 0xaa], 1
14059f68a: mov byte ptr [rbx + 0xab], 1
14059f691: mov byte ptr [rbx + 0xac], 1
14059f698: mov byte ptr [rbx + 0xad], 1
14059f69f: mov byte ptr [rbx + 0xae], 0
14059f6a6: mov byte ptr [rbx + 0xaf], 1
14059f6ad: mov byte ptr [rbx + 0xb0], 0
14059f6b4: mov byte ptr [rbx + 0xb1], 1
14059f6bb: mov byte ptr [rbx + 0xb2], 1
14059f6c2: mov rcx, rax
14059f6c5: call 0x140943660
14059f6ca: mov byte ptr [rbx + 0xaa], 1
14059f6d1: mov byte ptr [rbx + 0xab], 1
14059f6d8: mov byte ptr [rbx + 0xac], 1
14059f6df: mov byte ptr [rbx + 0xad], 1
14059f6e6: mov byte ptr [rbx + 0xae], 0
14059f6ed: mov byte ptr [rbx + 0xaf], 1
14059f6f4: mov byte ptr [rbx + 0xb0], 0
14059f6fb: mov byte ptr [rbx + 0xb1], 1
14059f702: mov byte ptr [rbx + 0xb2], 1
14059f709: mov rcx, rax
14059f70c: call 0x1409438a0
14059f711: mov byte ptr [rbx + 0xaa], 1
14059f718: mov byte ptr [rbx + 0xab], 1
14059f71f: mov byte ptr [rbx + 0xac], 1
14059f726: mov byte ptr [rbx + 0xad], 1
14059f72d: mov byte ptr [rbx + 0xae], 0
14059f734: mov byte ptr [rbx + 0xaf], 1
14059f73b: mov byte ptr [rbx + 0xb0], 0
14059f742: mov byte ptr [rbx + 0xb1], 1
14059f749: mov byte ptr [rbx + 0xb2], 1
14059f750: mov rcx, rax
14059f753: call 0x140943f40
14059f758: mov byte ptr [rbx + 0xaa], 1
14059f75f: mov byte ptr [rbx + 0xab], 1
14059f766: mov byte ptr [rbx + 0xac], 1
14059f76d: mov byte ptr [rbx + 0xad], 1
14059f774: mov byte ptr [rbx + 0xae], 0
14059f77b: mov byte ptr [rbx + 0xaf], 1
14059f782: mov byte ptr [rbx + 0xb0], 0
14059f789: mov byte ptr [rbx + 0xb1], 1
14059f790: mov byte ptr [rbx + 0xb2], 1
14059f797: mov rcx, rax
14059f79a: call 0x140944070
14059f79f: mov byte ptr [rbx + 0xaa], 1
14059f7a6: mov byte ptr [rbx + 0xab], 1
14059f7ad: mov byte ptr [rbx + 0xac], 1
14059f7b4: mov byte ptr [rbx + 0xad], 1
14059f7bb: mov byte ptr [rbx + 0xae], 0
14059f7c2: mov byte ptr [rbx + 0xaf], 1
14059f7c9: mov byte ptr [rbx + 0xb0], 0
14059f7d0: mov byte ptr [rbx + 0xb1], 1
14059f7d7: mov byte ptr [rbx + 0xb2], 1
14059f7de: mov rcx, rax
14059f7e1: call 0x140944280
14059f7e6: mov byte ptr [rbx + 0xaa], 1
14059f7ed: mov byte ptr [rbx + 0xab], 1
14059f7f4: mov byte ptr [rbx + 0xac], 1
14059f7fb: mov byte ptr [rbx + 0xad], 1
14059f802: mov byte ptr [rbx + 0xae], 0
14059f809: mov byte ptr [rbx + 0xaf], 1
14059f810: mov byte ptr [rbx + 0xb0], 0
14059f817: mov byte ptr [rbx + 0xb1], 1
14059f81e: mov byte ptr [rbx + 0xb2], 1
14059f825: mov rcx, rax
14059f828: call 0x1409411a0
14059f82d: mov byte ptr [rbx + 0xaa], 1
14059f834: mov byte ptr [rbx + 0xab], 1
14059f83b: mov byte ptr [rbx + 0xac], 1
14059f842: mov byte ptr [rbx + 0xad], 1
14059f849: mov byte ptr [rbx + 0xae], 0
14059f850: mov byte ptr [rbx + 0xaf], 1
14059f857: mov byte ptr [rbx + 0xb0], 0
14059f85e: mov byte ptr [rbx + 0xb1], 1
14059f865: mov byte ptr [rbx + 0xb2], 1
14059f86c: mov rcx, rax
14059f86f: call 0x140941450
14059f874: mov byte ptr [rbx + 0xaa], 1
14059f87b: mov byte ptr [rbx + 0xab], 1
14059f882: mov byte ptr [rbx + 0xac], 1
14059f889: mov byte ptr [rbx + 0xad], 1
14059f890: mov byte ptr [rbx + 0xae], 0
14059f897: mov byte ptr [rbx + 0xaf], 1
14059f89e: mov byte ptr [rbx + 0xb0], 0
14059f8a5: mov byte ptr [rbx + 0xb1], 1
14059f8ac: mov byte ptr [rbx + 0xb2], 1
14059f8b3: mov rcx, rax
14059f8b6: call 0x1409443b0
14059f8bb: mov byte ptr [rbx + 0xaa], 1
14059f8c2: mov byte ptr [rbx + 0xab], 1
14059f8c9: mov byte ptr [rbx + 0xac], 1
14059f8d0: mov byte ptr [rbx + 0xad], 1
14059f8d7: mov byte ptr [rbx + 0xae], 0
14059f8de: mov byte ptr [rbx + 0xaf], 1
14059f8e5: mov byte ptr [rbx + 0xb0], 0
14059f8ec: mov byte ptr [rbx + 0xb1], 1
14059f8f3: mov byte ptr [rbx + 0xb2], 1
14059f8fa: mov rcx, rax
14059f8fd: call 0x1409447e0
14059f902: mov byte ptr [rbx + 0xaa], 1
14059f909: mov byte ptr [rbx + 0xab], 1
14059f910: mov byte ptr [rbx + 0xac], 1
14059f917: mov byte ptr [rbx + 0xad], 1
14059f91e: mov byte ptr [rbx + 0xae], 0
14059f925: mov byte ptr [rbx + 0xaf], 1
14059f92c: mov byte ptr [rbx + 0xb0], 0
14059f933: mov byte ptr [rbx + 0xb1], 1
14059f93a: mov byte ptr [rbx + 0xb2], 1
14059f941: mov rcx, rax
14059f944: call 0x14062fcb0
14059f949: mov byte ptr [rbx + 0xaa], 1
14059f950: mov byte ptr [rbx + 0xab], 1
14059f957: mov byte ptr [rbx + 0xac], 1
14059f95e: mov byte ptr [rbx + 0xad], 1
14059f965: mov byte ptr [rbx + 0xae], 0
14059f96c: mov byte ptr [rbx + 0xaf], 1
14059f973: mov byte ptr [rbx + 0xb0], 0
14059f97a: mov byte ptr [rbx + 0xb1], 1
14059f981: mov byte ptr [rbx + 0xb2], 1
14059f988: mov rcx, rax
14059f98b: call 0x140945c60
14059f990: mov rsi, rax
14059f993: mov byte ptr [rbx + 0xaa], 1
14059f99a: mov byte ptr [rbx + 0xab], 1
14059f9a1: mov byte ptr [rbx + 0xac], 1
14059f9a8: mov byte ptr [rbx + 0xad], 1
14059f9af: mov byte ptr [rbx + 0xae], 0
14059f9b6: mov byte ptr [rbx + 0xaf], 1
14059f9bd: mov byte ptr [rbx + 0xb0], 0
14059f9c4: mov byte ptr [rbx + 0xb1], 1
14059f9cb: mov byte ptr [rbx + 0xb2], 1
14059f9d2: lea rcx, [rbx + 0x2590]
14059f9d9: call 0x1403c9610
14059f9de: lea rcx, [rbx + 0x440]
14059f9e5: call 0x1400b18f0
14059f9ea: movups xmm0, xmmword ptr [rbx + 0x25f0]
14059f9f1: movaps xmmword ptr [rbx + 0xb60], xmm0
14059f9f8: movups xmm0, xmmword ptr [rbx + 0x25e0]
14059f9ff: movaps xmmword ptr [rbx + 0xb50], xmm0
14059fa06: movups xmm0, xmmword ptr [rbx + 0x25d0]
14059fa0d: movaps xmmword ptr [rbx + 0xb40], xmm0
14059fa14: movups xmm0, xmmword ptr [rbx + 0x2590]
14059fa1b: movups xmm1, xmmword ptr [rbx + 0x25a0]
14059fa22: movups xmm2, xmmword ptr [rbx + 0x25b0]
14059fa29: movups xmm3, xmmword ptr [rbx + 0x25c0]
14059fa30: movaps xmmword ptr [rbx + 0xb30], xmm3
14059fa37: movaps xmmword ptr [rbx + 0xb20], xmm2
14059fa3e: movaps xmmword ptr [rbx + 0xb10], xmm1
14059fa45: movaps xmmword ptr [rbx + 0xb00], xmm0
14059fa4c: movups xmm0, xmmword ptr [rbx + 0x440]
14059fa53: movups xmm1, xmmword ptr [rbx + 0x450]
14059fa5a: movups xmm2, xmmword ptr [rbx + 0x460]
14059fa61: movups xmm3, xmmword ptr [rbx + 0x470]
14059fa68: movaps xmmword ptr [rbx + 0xb70], xmm0
14059fa6f: movaps xmmword ptr [rbx + 0xb80], xmm1
14059fa76: movaps xmmword ptr [rbx + 0xb90], xmm2
14059fa7d: movaps xmmword ptr [rbx + 0xba0], xmm3
14059fa84: movups xmm0, xmmword ptr [rbx + 0x480]
14059fa8b: movaps xmmword ptr [rbx + 0xbb0], xmm0
14059fa92: movups xmm0, xmmword ptr [rbx + 0x490]
14059fa99: movaps xmmword ptr [rbx + 0xbc0], xmm0
14059faa0: movups xmm0, xmmword ptr [rbx + 0x4a0]
14059faa7: movaps xmmword ptr [rbx + 0xbd0], xmm0
14059faae: mov byte ptr [rbx + 0xaa], 1
14059fab5: mov byte ptr [rbx + 0xab], 1
14059fabc: mov byte ptr [rbx + 0xac], 1
14059fac3: mov byte ptr [rbx + 0xad], 1
14059faca: mov byte ptr [rbx + 0xae], 0
14059fad1: mov byte ptr [rbx + 0xaf], 1
14059fad8: mov byte ptr [rbx + 0xb0], 0
14059fadf: mov byte ptr [rbx + 0xb1], 1
14059fae6: mov byte ptr [rbx + 0xb2], 1
14059faed: lea r9, [rip + 0x245fd3c]
14059faf4: lea rcx, [rbx + 0x440]
14059fafb: lea rdx, [rbx + 0xb00]
14059fb02: mov r8b, 0xa
14059fb05: call 0x1400d6520
14059fb0a: mov byte ptr [rbx + 0xaa], 1
14059fb11: mov byte ptr [rbx + 0xab], 1
14059fb18: mov byte ptr [rbx + 0xac], 1
14059fb1f: mov byte ptr [rbx + 0xad], 1
14059fb26: mov byte ptr [rbx + 0xae], 0
14059fb2d: mov byte ptr [rbx + 0xaf], 1
14059fb34: mov byte ptr [rbx + 0xb0], 0
14059fb3b: mov byte ptr [rbx + 0xb1], 1
14059fb42: mov byte ptr [rbx + 0xb2], 1
14059fb49: lea rcx, [rbx + 0x24e8]
14059fb50: lea rdx, [rbx + 0x440]
14059fb57: call 0x1400d37d0
14059fb5c: lea rcx, [rbx + 0xb00]
14059fb63: call 0x1405389b0
14059fb68: mov byte ptr [rbx + 0x440], 0x16
14059fb6f: mov rcx, qword ptr [rip + 0x308c7fa]
14059fb76: lea r8, [rip + 0x238b41b]
14059fb7d: lea rdx, [rbx + 0x440]
14059fb84: call 0x140364d20
14059fb89: mov r8, rdx
14059fb8c: lea rcx, [rbx + 0xb00]
14059fb93: mov rdx, rax
14059fb96: call 0x1400d68c0
14059fb9b: movups xmm0, xmmword ptr [rbx + 0xb60]
14059fba2: movaps xmmword ptr [rbx + 0x3d0], xmm0
14059fba9: movups xmm0, xmmword ptr [rbx + 0xb50]
14059fbb0: movaps xmmword ptr [rbx + 0x3c0], xmm0
14059fbb7: movups xmm0, xmmword ptr [rbx + 0xb40]
14059fbbe: movaps xmmword ptr [rbx + 0x3b0], xmm0
14059fbc5: movups xmm0, xmmword ptr [rbx + 0xb00]
14059fbcc: movups xmm1, xmmword ptr [rbx + 0xb10]
14059fbd3: movups xmm2, xmmword ptr [rbx + 0xb20]
14059fbda: movups xmm3, xmmword ptr [rbx + 0xb30]
14059fbe1: movaps xmmword ptr [rbx + 0x3a0], xmm3
14059fbe8: movaps xmmword ptr [rbx + 0x390], xmm2
14059fbef: movaps xmmword ptr [rbx + 0x380], xmm1
14059fbf6: movaps xmmword ptr [rbx + 0x370], xmm0
14059fbfd: lea r8, [rip + 0x245fc44]
14059fc04: lea rcx, [rbx + 0xb00]
14059fc0b: mov dl, 0x24
14059fc0d: call 0x1402cc220
14059fc12: lea rcx, [rbx + 0x1ff0]
14059fc19: lea rdx, [rbx + 0xb00]
14059fc20: call 0x1400d37d0
14059fc25: lea r8, [rip + 0x245fc34]
14059fc2c: lea rcx, [rbx + 0xb00]
14059fc33: mov dl, 0x24
14059fc35: call 0x140524400
14059fc3a: lea rcx, [rbx + 0x2230]
14059fc41: lea rdx, [rbx + 0xb00]
14059fc48: call 0x1400d37d0
14059fc4d: lea r8, [rip + 0x245fc24]
14059fc54: lea rcx, [rbx + 0x2bf0]
14059fc5b: mov dl, 0x24
14059fc5d: call 0x1403429a0
14059fc62: lea r8, [rip + 0x245fc27]
14059fc69: lea rcx, [rbx + 0x2590]
14059fc70: mov dl, 0x2c
14059fc72: call 0x1403c7a30
14059fc77: lea r8, [rip + 0x245fc2a]
14059fc7e: lea rcx, [rbx + 0x440]
14059fc85: mov dl, 0xe
14059fc87: call 0x1405c0400
14059fc8c: movups xmm0, xmmword ptr [rbx + 0x2548]
14059fc93: movaps xmmword ptr [rbx + 0xb60], xmm0
14059fc9a: movups xmm0, xmmword ptr [rbx + 0x2538]
14059fca1: movaps xmmword ptr [rbx + 0xb50], xmm0
14059fca8: movups xmm0, xmmword ptr [rbx + 0x2528]
14059fcaf: movaps xmmword ptr [rbx + 0xb40], xmm0
14059fcb6: movups xmm0, xmmword ptr [rbx + 0x24e8]
14059fcbd: movups xmm1, xmmword ptr [rbx + 0x24f8]
14059fcc4: movups xmm2, xmmword ptr [rbx + 0x2508]
14059fccb: movups xmm3, xmmword ptr [rbx + 0x2518]
14059fcd2: movaps xmmword ptr [rbx + 0xb30], xmm3
14059fcd9: movaps xmmword ptr [rbx + 0xb20], xmm2
14059fce0: movaps xmmword ptr [rbx + 0xb10], xmm1
14059fce7: movaps xmmword ptr [rbx + 0xb00], xmm0
14059fcee: movaps xmm0, xmmword ptr [rbx + 0x370]
14059fcf5: movaps xmm1, xmmword ptr [rbx + 0x380]
14059fcfc: movaps xmm2, xmmword ptr [rbx + 0x390]
14059fd03: movaps xmm3, xmmword ptr [rbx + 0x3a0]
14059fd0a: movaps xmmword ptr [rbx + 0xb70], xmm0
14059fd11: movaps xmmword ptr [rbx + 0xb80], xmm1
14059fd18: movaps xmmword ptr [rbx + 0xb90], xmm2
14059fd1f: movaps xmmword ptr [rbx + 0xba0], xmm3
14059fd26: movaps xmm0, xmmword ptr [rbx + 0x3b0]
14059fd2d: movaps xmmword ptr [rbx + 0xbb0], xmm0
14059fd34: movaps xmm0, xmmword ptr [rbx + 0x3c0]
14059fd3b: movaps xmmword ptr [rbx + 0xbc0], xmm0
14059fd42: movaps xmm0, xmmword ptr [rbx + 0x3d0]
14059fd49: movaps xmmword ptr [rbx + 0xbd0], xmm0
14059fd50: movups xmm0, xmmword ptr [rbx + 0x2050]
14059fd57: movaps xmmword ptr [rbx + 0xc40], xmm0
14059fd5e: movups xmm0, xmmword ptr [rbx + 0x2040]
14059fd65: movaps xmmword ptr [rbx + 0xc30], xmm0
14059fd6c: movups xmm0, xmmword ptr [rbx + 0x2030]
14059fd73: movaps xmmword ptr [rbx + 0xc20], xmm0
14059fd7a: movups xmm0, xmmword ptr [rbx + 0x1ff0]
14059fd81: movups xmm1, xmmword ptr [rbx + 0x2000]
14059fd88: movups xmm2, xmmword ptr [rbx + 0x2010]
14059fd8f: movups xmm3, xmmword ptr [rbx + 0x2020]
14059fd96: movaps xmmword ptr [rbx + 0xc10], xmm3
14059fd9d: movaps xmmword ptr [rbx + 0xc00], xmm2
14059fda4: movaps xmmword ptr [rbx + 0xbf0], xmm1
14059fdab: movaps xmmword ptr [rbx + 0xbe0], xmm0
14059fdb2: movups xmm0, xmmword ptr [rbx + 0x2230]
14059fdb9: movups xmm1, xmmword ptr [rbx + 0x2240]
14059fdc0: movups xmm2, xmmword ptr [rbx + 0x2250]
14059fdc7: movups xmm3, xmmword ptr [rbx + 0x2260]
14059fdce: movaps xmmword ptr [rbx + 0xc50], xmm0
14059fdd5: movaps xmmword ptr [rbx + 0xc60], xmm1
14059fddc: movaps xmmword ptr [rbx + 0xc70], xmm2
14059fde3: movaps xmmword ptr [rbx + 0xc80], xmm3
14059fdea: movups xmm0, xmmword ptr [rbx + 0x2270]
14059fdf1: movaps xmmword ptr [rbx + 0xc90], xmm0
14059fdf8: movups xmm0, xmmword ptr [rbx + 0x2280]
14059fdff: movaps xmmword ptr [rbx + 0xca0], xmm0
14059fe06: movups xmm0, xmmword ptr [rbx + 0x2290]
14059fe0d: movaps xmmword ptr [rbx + 0xcb0], xmm0
14059fe14: movups xmm0, xmmword ptr [rbx + 0x2c50]
14059fe1b: movaps xmmword ptr [rbx + 0xd20], xmm0
14059fe22: movups xmm0, xmmword ptr [rbx + 0x2c40]
14059fe29: movaps xmmword ptr [rbx + 0xd10], xmm0
14059fe30: movups xmm0, xmmword ptr [rbx + 0x2c30]
14059fe37: movaps xmmword ptr [rbx + 0xd00], xmm0
14059fe3e: movups xmm0, xmmword ptr [rbx + 0x2bf0]
14059fe45: movups xmm1, xmmword ptr [rbx + 0x2c00]
14059fe4c: movups xmm2, xmmword ptr [rbx + 0x2c10]
14059fe53: movups xmm3, xmmword ptr [rbx + 0x2c20]
14059fe5a: movaps xmmword ptr [rbx + 0xcf0], xmm3
14059fe61: movaps xmmword ptr [rbx + 0xce0], xmm2
14059fe68: movaps xmmword ptr [rbx + 0xcd0], xmm1
14059fe6f: movaps xmmword ptr [rbx + 0xcc0], xmm0
14059fe76: movups xmm0, xmmword ptr [rbx + 0x25f0]
14059fe7d: movaps xmmword ptr [rbx + 0xd90], xmm0
14059fe84: movups xmm0, xmmword ptr [rbx + 0x25e0]
14059fe8b: movaps xmmword ptr [rbx + 0xd80], xmm0
14059fe92: movups xmm0, xmmword ptr [rbx + 0x25d0]
14059fe99: movaps xmmword ptr [rbx + 0xd70], xmm0
14059fea0: movups xmm0, xmmword ptr [rbx + 0x2590]
14059fea7: movups xmm1, xmmword ptr [rbx + 0x25a0]
14059feae: movups xmm2, xmmword ptr [rbx + 0x25b0]
14059feb5: movups xmm3, xmmword ptr [rbx + 0x25c0]
14059febc: movaps xmmword ptr [rbx + 0xd60], xmm3
14059fec3: movaps xmmword ptr [rbx + 0xd50], xmm2
14059feca: movaps xmmword ptr [rbx + 0xd40], xmm1
14059fed1: movaps xmmword ptr [rbx + 0xd30], xmm0
14059fed8: movups xmm0, xmmword ptr [rbx + 0x4a0]
14059fedf: movaps xmmword ptr [rbx + 0xe00], xmm0
14059fee6: movups xmm0, xmmword ptr [rbx + 0x490]
14059feed: movaps xmmword ptr [rbx + 0xdf0], xmm0
14059fef4: movups xmm0, xmmword ptr [rbx + 0x480]
14059fefb: movaps xmmword ptr [rbx + 0xde0], xmm0
14059ff02: movups xmm0, xmmword ptr [rbx + 0x440]
14059ff09: movups xmm1, xmmword ptr [rbx + 0x450]
14059ff10: movups xmm2, xmmword ptr [rbx + 0x460]
14059ff17: movups xmm3, xmmword ptr [rbx + 0x470]
14059ff1e: movaps xmmword ptr [rbx + 0xdd0], xmm3
14059ff25: movaps xmmword ptr [rbx + 0xdc0], xmm2
14059ff2c: movaps xmmword ptr [rbx + 0xdb0], xmm1
14059ff33: movaps xmmword ptr [rbx + 0xda0], xmm0
14059ff3a: mov byte ptr [rbx + 0xaa], 1
14059ff41: mov byte ptr [rbx + 0xab], 1
14059ff48: mov byte ptr [rbx + 0xac], 1
14059ff4f: mov byte ptr [rbx + 0xad], 1
14059ff56: mov byte ptr [rbx + 0xae], 0
14059ff5d: mov byte ptr [rbx + 0xaf], 1
14059ff64: mov byte ptr [rbx + 0xb0], 0
14059ff6b: mov byte ptr [rbx + 0xb1], 1
14059ff72: mov byte ptr [rbx + 0xb2], 1
14059ff79: lea rdx, [rbx + 0xb00]
14059ff80: mov rcx, rsi
14059ff83: call 0x140327490
14059ff88: mov rcx, rsi
14059ff8b: add rcx, 0x38
14059ff8f: mov byte ptr [rbx + 0xaa], 1
14059ff96: mov byte ptr [rbx + 0xab], 1
14059ff9d: mov byte ptr [rbx + 0xac], 1
14059ffa4: mov byte ptr [rbx + 0xad], 1
14059ffab: mov byte ptr [rbx + 0xae], 0
14059ffb2: mov byte ptr [rbx + 0xaf], 1
14059ffb9: mov byte ptr [rbx + 0xb0], 0
14059ffc0: mov byte ptr [rbx + 0xb1], 1
14059ffc7: mov byte ptr [rbx + 0xb2], 1
14059ffce: movabs rdx, 0x1379125545270eb2
14059ffd8: movabs r8, 0x4eae26c0c92a587b
14059ffe2: call 0x1400dd6f0
14059ffe7: test rax, rax
14059ffea: je 0x1405a001f
14059ffec: cmp qword ptr [rsi + 0x218], rdx
14059fff3: jbe 0x1405a001f
14059fff5: mov rax, qword ptr [rsi + 0x210]
14059fffc: mov rax, qword ptr [rax + rdx*8]
1405a0000: test rax, rax
1405a0003: je 0x1405a001f
1405a0005: mov rcx, qword ptr [rsi + 0x1e0]
1405a000c: not rax
1405a000f: lea rax, [rax + rax*2]
1405a0013: shl rax, 5
1405a0017: cmp qword ptr [rcx + rax + 0x20], 0
1405a001d: jne 0x1405a0066
1405a001f: mov byte ptr [rbx + 0xaa], 1
1405a0026: mov byte ptr [rbx + 0xab], 1
1405a002d: mov byte ptr [rbx + 0xac], 1
1405a0034: mov byte ptr [rbx + 0xad], 1
1405a003b: mov byte ptr [rbx + 0xae], 0
1405a0042: mov byte ptr [rbx + 0xaf], 1
1405a0049: mov byte ptr [rbx + 0xb0], 0
1405a0050: mov byte ptr [rbx + 0xb1], 1
1405a0057: mov byte ptr [rbx + 0xb2], 1
1405a005e: mov rcx, rsi
1405a0061: call 0x14056d810
1405a0066: mov rcx, qword ptr [rbx + 0x158]
1405a006d: lock inc qword ptr [rcx]
1405a0071: jle 0x1405a7040
1405a0077: mov byte ptr [rbx + 0xaa], 1
1405a007e: mov byte ptr [rbx + 0xab], 1
1405a0085: mov byte ptr [rbx + 0xac], 1
1405a008c: mov byte ptr [rbx + 0xad], 1
1405a0093: mov byte ptr [rbx + 0xae], 0
1405a009a: mov byte ptr [rbx + 0xaf], 1
1405a00a1: mov byte ptr [rbx + 0xb0], 0
1405a00a8: mov byte ptr [rbx + 0xb1], 1
1405a00af: mov byte ptr [rbx + 0xb2], 1
1405a00b6: call 0x14067dd80
1405a00bb: mov byte ptr [rbx + 0xaa], 1
1405a00c2: mov byte ptr [rbx + 0xab], 1
1405a00c9: mov byte ptr [rbx + 0xac], 1
1405a00d0: mov byte ptr [rbx + 0xad], 1
1405a00d7: mov byte ptr [rbx + 0xae], 0
1405a00de: mov byte ptr [rbx + 0xaf], 1
1405a00e5: mov byte ptr [rbx + 0xb0], 0
1405a00ec: mov byte ptr [rbx + 0xb1], 1
1405a00f3: mov byte ptr [rbx + 0xb2], 1
1405a00fa: mov rcx, rsi
1405a00fd: mov rdx, rax
1405a0100: call 0x140138aa0
1405a0105: mov byte ptr [rbx + 0xaa], 1
1405a010c: mov byte ptr [rbx + 0xab], 1
1405a0113: mov byte ptr [rbx + 0xac], 1
1405a011a: mov byte ptr [rbx + 0xad], 1
1405a0121: mov byte ptr [rbx + 0xae], 0
1405a0128: mov byte ptr [rbx + 0xaf], 1
1405a012f: mov byte ptr [rbx + 0xb0], 0
1405a0136: mov byte ptr [rbx + 0xb1], 1
1405a013d: mov byte ptr [rbx + 0xb2], 1
1405a0144: mov rcx, rax
1405a0147: call 0x1406305f0
1405a014c: mov r15, rax
1405a014f: mov byte ptr [rbx + 0xaa], 1
1405a0156: mov byte ptr [rbx + 0xab], 1
1405a015d: mov byte ptr [rbx + 0xac], 1
1405a0164: mov byte ptr [rbx + 0xad], 1
1405a016b: mov byte ptr [rbx + 0xae], 0
1405a0172: mov byte ptr [rbx + 0xaf], 1
1405a0179: mov byte ptr [rbx + 0xb0], 0
1405a0180: mov byte ptr [rbx + 0xb1], 1
1405a0187: mov byte ptr [rbx + 0xb2], 1
1405a018e: lea r8, [rip + 0x245f72b]
1405a0195: lea rcx, [rbx + 0xb00]
1405a019c: mov dl, 7
1405a019e: call 0x140363710
1405a01a3: mov byte ptr [rbx + 0xaa], 1
1405a01aa: mov byte ptr [rbx + 0xab], 1
1405a01b1: mov byte ptr [rbx + 0xac], 1
1405a01b8: mov byte ptr [rbx + 0xad], 1
1405a01bf: mov byte ptr [rbx + 0xae], 0
1405a01c6: mov byte ptr [rbx + 0xaf], 1
1405a01cd: mov byte ptr [rbx + 0xb0], 0
1405a01d4: mov byte ptr [rbx + 0xb1], 1
1405a01db: mov byte ptr [rbx + 0xb2], 1
1405a01e2: lea rdx, [rbx + 0xb00]
1405a01e9: mov rcx, r15
1405a01ec: call 0x1403276f0
1405a01f1: mov byte ptr [rbx + 0xaa], 1
1405a01f8: mov byte ptr [rbx + 0xab], 1
1405a01ff: mov byte ptr [rbx + 0xac], 1
1405a0206: mov byte ptr [rbx + 0xad], 1
1405a020d: mov byte ptr [rbx + 0xae], 0
1405a0214: mov byte ptr [rbx + 0xaf], 1
1405a021b: mov byte ptr [rbx + 0xb0], 0
1405a0222: mov byte ptr [rbx + 0xb1], 1
1405a0229: mov byte ptr [rbx + 0xb2], 1
1405a0230: lea rcx, [rbx + 0xb00]
1405a0237: call 0x1403770b0
1405a023c: mov byte ptr [rbx + 0x2590], 0x21
1405a0243: mov rcx, qword ptr [rip + 0x308c126]
1405a024a: lea r8, [rip + 0x238ad47]
1405a0251: lea rdx, [rbx + 0x2590]
1405a0258: call 0x140364d20
1405a025d: mov r8, rdx
1405a0260: lea rcx, [rbx + 0xb00]
1405a0267: mov rdx, rax
1405a026a: call 0x1400d68c0
1405a026f: movups xmm0, xmmword ptr [rbx + 0xb60]
1405a0276: movaps xmmword ptr [rbx + 0x4a0], xmm0
1405a027d: movups xmm0, xmmword ptr [rbx + 0xb50]
1405a0284: movaps xmmword ptr [rbx + 0x490], xmm0
1405a028b: movups xmm0, xmmword ptr [rbx + 0xb40]
1405a0292: movaps xmmword ptr [rbx + 0x480], xmm0
1405a0299: movups xmm0, xmmword ptr [rbx + 0xb00]
1405a02a0: movups xmm1, xmmword ptr [rbx + 0xb10]
1405a02a7: movups xmm2, xmmword ptr [rbx + 0xb20]
1405a02ae: movups xmm3, xmmword ptr [rbx + 0xb30]
1405a02b5: movaps xmmword ptr [rbx + 0x470], xmm3
1405a02bc: movaps xmmword ptr [rbx + 0x460], xmm2
1405a02c3: movaps xmmword ptr [rbx + 0x450], xmm1
1405a02ca: movaps xmmword ptr [rbx + 0x440], xmm0
1405a02d1: mov byte ptr [rbx + 0xaa], 1
1405a02d8: mov byte ptr [rbx + 0xab], 1
1405a02df: mov byte ptr [rbx + 0xac], 1
1405a02e6: mov byte ptr [rbx + 0xad], 1
1405a02ed: mov byte ptr [rbx + 0xae], 0
1405a02f4: mov byte ptr [rbx + 0xaf], 1
1405a02fb: mov byte ptr [rbx + 0xb0], 0
1405a0302: mov byte ptr [rbx + 0xb1], 1
1405a0309: mov byte ptr [rbx + 0xb2], 1
1405a0310: lea rdx, [rbx + 0x440]
1405a0317: mov rcx, r15
1405a031a: call 0x1403276f0
1405a031f: mov byte ptr [rbx + 0xaa], 1
1405a0326: mov byte ptr [rbx + 0xab], 1
1405a032d: mov byte ptr [rbx + 0xac], 1
1405a0334: mov byte ptr [rbx + 0xad], 1
1405a033b: mov byte ptr [rbx + 0xae], 0
1405a0342: mov byte ptr [rbx + 0xaf], 1
1405a0349: mov byte ptr [rbx + 0xb0], 0
1405a0350: mov byte ptr [rbx + 0xb1], 1
1405a0357: mov byte ptr [rbx + 0xb2], 1
1405a035e: lea rcx, [rbx + 0x2bf0]
1405a0365: call 0x14016bcd0
1405a036a: lea rcx, [rbx + 0x2590]
1405a0371: call 0x1405b6ee0
1405a0376: lea rcx, [rbx + 0xb00]
1405a037d: call 0x1402e8280
1405a0382: lea rcx, [rbx + 0x440]
1405a0389: lea rdx, [rbx + 0xb00]
1405a0390: call 0x1400d3350
1405a0395: movups xmm0, xmmword ptr [rbx + 0x2bf0]
1405a039c: movups xmm1, xmmword ptr [rbx + 0x2c00]
1405a03a3: movups xmm2, xmmword ptr [rbx + 0x2c10]
1405a03aa: movups xmm3, xmmword ptr [rbx + 0x2c20]
1405a03b1: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a03b8: movaps xmmword ptr [rbx + 0xb10], xmm1
1405a03bf: movaps xmmword ptr [rbx + 0xb20], xmm2
1405a03c6: movaps xmmword ptr [rbx + 0xb30], xmm3
1405a03cd: movups xmm0, xmmword ptr [rbx + 0x2c30]
1405a03d4: movaps xmmword ptr [rbx + 0xb40], xmm0
1405a03db: movups xmm0, xmmword ptr [rbx + 0x2c40]
1405a03e2: movaps xmmword ptr [rbx + 0xb50], xmm0
1405a03e9: movups xmm0, xmmword ptr [rbx + 0x2c50]
1405a03f0: movaps xmmword ptr [rbx + 0xb60], xmm0
1405a03f7: movups xmm0, xmmword ptr [rbx + 0x2590]
1405a03fe: movups xmm1, xmmword ptr [rbx + 0x25a0]
1405a0405: movups xmm2, xmmword ptr [rbx + 0x25b0]
1405a040c: movups xmm3, xmmword ptr [rbx + 0x25c0]
1405a0413: movaps xmmword ptr [rbx + 0xb70], xmm0
1405a041a: movaps xmmword ptr [rbx + 0xb80], xmm1
1405a0421: movaps xmmword ptr [rbx + 0xb90], xmm2
1405a0428: movaps xmmword ptr [rbx + 0xba0], xmm3
1405a042f: movups xmm0, xmmword ptr [rbx + 0x25d0]
1405a0436: movaps xmmword ptr [rbx + 0xbb0], xmm0
1405a043d: movups xmm0, xmmword ptr [rbx + 0x25e0]
1405a0444: movaps xmmword ptr [rbx + 0xbc0], xmm0
1405a044b: movups xmm0, xmmword ptr [rbx + 0x25f0]
1405a0452: movaps xmmword ptr [rbx + 0xbd0], xmm0
1405a0459: movups xmm0, xmmword ptr [rbx + 0x4a0]
1405a0460: movaps xmmword ptr [rbx + 0xc40], xmm0
1405a0467: movups xmm0, xmmword ptr [rbx + 0x490]
1405a046e: movaps xmmword ptr [rbx + 0xc30], xmm0
1405a0475: movups xmm0, xmmword ptr [rbx + 0x480]
1405a047c: movaps xmmword ptr [rbx + 0xc20], xmm0
1405a0483: movups xmm0, xmmword ptr [rbx + 0x440]
1405a048a: movups xmm1, xmmword ptr [rbx + 0x450]
1405a0491: movups xmm2, xmmword ptr [rbx + 0x460]
1405a0498: movups xmm3, xmmword ptr [rbx + 0x470]
1405a049f: movaps xmmword ptr [rbx + 0xc10], xmm3
1405a04a6: movaps xmmword ptr [rbx + 0xc00], xmm2
1405a04ad: movaps xmmword ptr [rbx + 0xbf0], xmm1
1405a04b4: movaps xmmword ptr [rbx + 0xbe0], xmm0
1405a04bb: mov byte ptr [rbx + 0xaa], 1
1405a04c2: mov byte ptr [rbx + 0xab], 1
1405a04c9: mov byte ptr [rbx + 0xac], 1
1405a04d0: mov byte ptr [rbx + 0xad], 1
1405a04d7: mov byte ptr [rbx + 0xae], 0
1405a04de: mov byte ptr [rbx + 0xaf], 1
1405a04e5: mov byte ptr [rbx + 0xb0], 0
1405a04ec: mov byte ptr [rbx + 0xb1], 1
1405a04f3: mov byte ptr [rbx + 0xb2], 1
1405a04fa: lea r9, [rip + 0x245f3d7]
1405a0501: lea rcx, [rbx + 0x440]
1405a0508: lea rdx, [rbx + 0xb00]
1405a050f: mov r8b, 0x27
1405a0512: call 0x140611040
1405a0517: mov byte ptr [rbx + 0xaa], 1
1405a051e: mov byte ptr [rbx + 0xab], 1
1405a0525: mov byte ptr [rbx + 0xac], 1
1405a052c: mov byte ptr [rbx + 0xad], 1
1405a0533: mov byte ptr [rbx + 0xae], 0
1405a053a: mov byte ptr [rbx + 0xaf], 1
1405a0541: mov byte ptr [rbx + 0xb0], 0
1405a0548: mov byte ptr [rbx + 0xb1], 1
1405a054f: mov byte ptr [rbx + 0xb2], 1
1405a0556: lea rdx, [rbx + 0x440]
1405a055d: mov rcx, r15
1405a0560: call 0x1403276f0
1405a0565: mov byte ptr [rbx + 0xaa], 1
1405a056c: mov byte ptr [rbx + 0xab], 1
1405a0573: mov byte ptr [rbx + 0xac], 1
1405a057a: mov byte ptr [rbx + 0xad], 1
1405a0581: mov byte ptr [rbx + 0xae], 0
1405a0588: mov byte ptr [rbx + 0xaf], 1
1405a058f: mov byte ptr [rbx + 0xb0], 0
1405a0596: mov byte ptr [rbx + 0xb1], 1
1405a059d: mov byte ptr [rbx + 0xb2], 1
1405a05a4: mov rcx, r15
1405a05a7: call 0x140327d70
1405a05ac: mov r12, r15
1405a05af: add r12, 0x38
1405a05b3: mov byte ptr [rbx + 0xaa], 1
1405a05ba: mov byte ptr [rbx + 0xab], 1
1405a05c1: mov byte ptr [rbx + 0xac], 1
1405a05c8: mov byte ptr [rbx + 0xad], 1
1405a05cf: mov byte ptr [rbx + 0xae], 0
1405a05d6: mov byte ptr [rbx + 0xaf], 1
1405a05dd: mov byte ptr [rbx + 0xb0], 0
1405a05e4: mov byte ptr [rbx + 0xb1], 1
1405a05eb: mov byte ptr [rbx + 0xb2], 1
1405a05f2: movabs rdx, 0xb8da66e7e904e3d
1405a05fc: movabs r8, 0x2fbe5d1891a41745
1405a0606: mov rcx, r12
1405a0609: call 0x1400dd6f0
1405a060e: test rax, rax
1405a0611: je 0x1405a0646
1405a0613: cmp qword ptr [r15 + 0x218], rdx
1405a061a: jbe 0x1405a0646
1405a061c: mov rax, qword ptr [r15 + 0x210]
1405a0623: mov rax, qword ptr [rax + rdx*8]
1405a0627: test rax, rax
1405a062a: je 0x1405a0646
1405a062c: mov rcx, qword ptr [r15 + 0x1e0]
1405a0633: not rax
1405a0636: lea rax, [rax + rax*2]
1405a063a: shl rax, 5
1405a063e: cmp qword ptr [rcx + rax + 0x20], 0
1405a0644: jne 0x1405a068d
1405a0646: mov byte ptr [rbx + 0xaa], 1
1405a064d: mov byte ptr [rbx + 0xab], 1
1405a0654: mov byte ptr [rbx + 0xac], 1
1405a065b: mov byte ptr [rbx + 0xad], 1
1405a0662: mov byte ptr [rbx + 0xae], 0
1405a0669: mov byte ptr [rbx + 0xaf], 1
1405a0670: mov byte ptr [rbx + 0xb0], 0
1405a0677: mov byte ptr [rbx + 0xb1], 1
1405a067e: mov byte ptr [rbx + 0xb2], 1
1405a0685: mov rcx, r15
1405a0688: call 0x140568170
1405a068d: mov byte ptr [rbx + 0xaa], 1
1405a0694: mov byte ptr [rbx + 0xab], 1
1405a069b: mov byte ptr [rbx + 0xac], 1
1405a06a2: mov byte ptr [rbx + 0xad], 1
1405a06a9: mov byte ptr [rbx + 0xae], 0
1405a06b0: mov byte ptr [rbx + 0xaf], 1
1405a06b7: mov byte ptr [rbx + 0xb0], 0
1405a06be: mov byte ptr [rbx + 0xb1], 1
1405a06c5: mov byte ptr [rbx + 0xb2], 1
1405a06cc: movabs rdx, 0x75bf40b77651060d
1405a06d6: movabs r8, 0xc86ab9d809a19210
1405a06e0: mov rcx, r12
1405a06e3: call 0x1400dd6f0
1405a06e8: test rax, rax
1405a06eb: je 0x1405a0720
1405a06ed: cmp qword ptr [r15 + 0x218], rdx
1405a06f4: jbe 0x1405a0720
1405a06f6: mov rax, qword ptr [r15 + 0x210]
1405a06fd: mov rax, qword ptr [rax + rdx*8]
1405a0701: test rax, rax
1405a0704: je 0x1405a0720
1405a0706: mov rcx, qword ptr [r15 + 0x1e0]
1405a070d: not rax
1405a0710: lea rax, [rax + rax*2]
1405a0714: shl rax, 5
1405a0718: cmp qword ptr [rcx + rax + 0x20], 0
1405a071e: jne 0x1405a0767
1405a0720: mov byte ptr [rbx + 0xaa], 1
1405a0727: mov byte ptr [rbx + 0xab], 1
1405a072e: mov byte ptr [rbx + 0xac], 1
1405a0735: mov byte ptr [rbx + 0xad], 1
1405a073c: mov byte ptr [rbx + 0xae], 0
1405a0743: mov byte ptr [rbx + 0xaf], 1
1405a074a: mov byte ptr [rbx + 0xb0], 0
1405a0751: mov byte ptr [rbx + 0xb1], 1
1405a0758: mov byte ptr [rbx + 0xb2], 1
1405a075f: mov rcx, r15
1405a0762: call 0x140569400
1405a0767: mov byte ptr [rbx + 0xaa], 1
1405a076e: mov byte ptr [rbx + 0xab], 1
1405a0775: mov byte ptr [rbx + 0xac], 1
1405a077c: mov byte ptr [rbx + 0xad], 1
1405a0783: mov byte ptr [rbx + 0xae], 0
1405a078a: mov byte ptr [rbx + 0xaf], 1
1405a0791: mov byte ptr [rbx + 0xb0], 0
1405a0798: mov byte ptr [rbx + 0xb1], 1
1405a079f: mov byte ptr [rbx + 0xb2], 1
1405a07a6: movabs rdx, 0xbbdd82cfacba045f
1405a07b0: movabs r8, 0xcaf27ba849498836
1405a07ba: mov rcx, r12
1405a07bd: call 0x1400dd6f0
1405a07c2: test rax, rax
1405a07c5: je 0x1405a07fa
1405a07c7: cmp qword ptr [r15 + 0x218], rdx
1405a07ce: jbe 0x1405a07fa
1405a07d0: mov rax, qword ptr [r15 + 0x210]
1405a07d7: mov rax, qword ptr [rax + rdx*8]
1405a07db: test rax, rax
1405a07de: je 0x1405a07fa
1405a07e0: mov rcx, qword ptr [r15 + 0x1e0]
1405a07e7: not rax
1405a07ea: lea rax, [rax + rax*2]
1405a07ee: shl rax, 5
1405a07f2: cmp qword ptr [rcx + rax + 0x20], 0
1405a07f8: jne 0x1405a0841
1405a07fa: mov byte ptr [rbx + 0xaa], 1
1405a0801: mov byte ptr [rbx + 0xab], 1
1405a0808: mov byte ptr [rbx + 0xac], 1
1405a080f: mov byte ptr [rbx + 0xad], 1
1405a0816: mov byte ptr [rbx + 0xae], 0
1405a081d: mov byte ptr [rbx + 0xaf], 1
1405a0824: mov byte ptr [rbx + 0xb0], 0
1405a082b: mov byte ptr [rbx + 0xb1], 1
1405a0832: mov byte ptr [rbx + 0xb2], 1
1405a0839: mov rcx, r15
1405a083c: call 0x140566280
1405a0841: mov byte ptr [rbx + 0xaa], 1
1405a0848: mov byte ptr [rbx + 0xab], 1
1405a084f: mov byte ptr [rbx + 0xac], 1
1405a0856: mov byte ptr [rbx + 0xad], 1
1405a085d: mov byte ptr [rbx + 0xae], 0
1405a0864: mov byte ptr [rbx + 0xaf], 1
1405a086b: mov byte ptr [rbx + 0xb0], 0
1405a0872: mov byte ptr [rbx + 0xb1], 1
1405a0879: mov byte ptr [rbx + 0xb2], 1
1405a0880: movabs rdx, 0x1c5d03917730cc4f
1405a088a: movabs r8, 0x3d29e4fb18bada6c
1405a0894: mov rcx, r12
1405a0897: call 0x1400dd6f0
1405a089c: test rax, rax
1405a089f: je 0x1405a08d4
1405a08a1: cmp qword ptr [r15 + 0x218], rdx
1405a08a8: jbe 0x1405a08d4
1405a08aa: mov rax, qword ptr [r15 + 0x210]
1405a08b1: mov rax, qword ptr [rax + rdx*8]
1405a08b5: test rax, rax
1405a08b8: je 0x1405a08d4
1405a08ba: mov rcx, qword ptr [r15 + 0x1e0]
1405a08c1: not rax
1405a08c4: lea rax, [rax + rax*2]
1405a08c8: shl rax, 5
1405a08cc: cmp qword ptr [rcx + rax + 0x20], 0
1405a08d2: jne 0x1405a091b
1405a08d4: mov byte ptr [rbx + 0xaa], 1
1405a08db: mov byte ptr [rbx + 0xab], 1
1405a08e2: mov byte ptr [rbx + 0xac], 1
1405a08e9: mov byte ptr [rbx + 0xad], 1
1405a08f0: mov byte ptr [rbx + 0xae], 0
1405a08f7: mov byte ptr [rbx + 0xaf], 1
1405a08fe: mov byte ptr [rbx + 0xb0], 0
1405a0905: mov byte ptr [rbx + 0xb1], 1
1405a090c: mov byte ptr [rbx + 0xb2], 1
1405a0913: mov rcx, r15
1405a0916: call 0x14057e850
1405a091b: mov byte ptr [rbx + 0xaa], 1
1405a0922: mov byte ptr [rbx + 0xab], 1
1405a0929: mov byte ptr [rbx + 0xac], 1
1405a0930: mov byte ptr [rbx + 0xad], 1
1405a0937: mov byte ptr [rbx + 0xae], 0
1405a093e: mov byte ptr [rbx + 0xaf], 1
1405a0945: mov byte ptr [rbx + 0xb0], 0
1405a094c: mov byte ptr [rbx + 0xb1], 1
1405a0953: mov byte ptr [rbx + 0xb2], 1
1405a095a: movabs rdx, 0x6ecb4684a4c9e1f9
1405a0964: movabs r8, 0xfae2e21abac81ddc
1405a096e: mov rcx, r12
1405a0971: call 0x1400dd6f0
1405a0976: test rax, rax
1405a0979: je 0x1405a09ae
1405a097b: cmp qword ptr [r15 + 0x218], rdx
1405a0982: jbe 0x1405a09ae
1405a0984: mov rax, qword ptr [r15 + 0x210]
1405a098b: mov rax, qword ptr [rax + rdx*8]
1405a098f: test rax, rax
1405a0992: je 0x1405a09ae
1405a0994: mov rcx, qword ptr [r15 + 0x1e0]
1405a099b: not rax
1405a099e: lea rax, [rax + rax*2]
1405a09a2: shl rax, 5
1405a09a6: cmp qword ptr [rcx + rax + 0x20], 0
1405a09ac: jne 0x1405a09f5
1405a09ae: mov byte ptr [rbx + 0xaa], 1
1405a09b5: mov byte ptr [rbx + 0xab], 1
1405a09bc: mov byte ptr [rbx + 0xac], 1
1405a09c3: mov byte ptr [rbx + 0xad], 1
1405a09ca: mov byte ptr [rbx + 0xae], 0
1405a09d1: mov byte ptr [rbx + 0xaf], 1
1405a09d8: mov byte ptr [rbx + 0xb0], 0
1405a09df: mov byte ptr [rbx + 0xb1], 1
1405a09e6: mov byte ptr [rbx + 0xb2], 1
1405a09ed: mov rcx, r15
1405a09f0: call 0x140575a00
1405a09f5: mov byte ptr [rbx + 0xaa], 1
1405a09fc: mov byte ptr [rbx + 0xab], 1
1405a0a03: mov byte ptr [rbx + 0xac], 1
1405a0a0a: mov byte ptr [rbx + 0xad], 1
1405a0a11: mov byte ptr [rbx + 0xae], 0
1405a0a18: mov byte ptr [rbx + 0xaf], 1
1405a0a1f: mov byte ptr [rbx + 0xb0], 0
1405a0a26: mov byte ptr [rbx + 0xb1], 1
1405a0a2d: mov byte ptr [rbx + 0xb2], 1
1405a0a34: movabs rdx, 0xdac2c590678dd6c0
1405a0a3e: movabs r8, 0x5d30eb6733ae4d59
1405a0a48: mov rcx, r12
1405a0a4b: call 0x1400dd6f0
1405a0a50: test rax, rax
1405a0a53: je 0x1405a0a88
1405a0a55: cmp qword ptr [r15 + 0x218], rdx
1405a0a5c: jbe 0x1405a0a88
1405a0a5e: mov rax, qword ptr [r15 + 0x210]
1405a0a65: mov rax, qword ptr [rax + rdx*8]
1405a0a69: test rax, rax
1405a0a6c: je 0x1405a0a88
1405a0a6e: mov rcx, qword ptr [r15 + 0x1e0]
1405a0a75: not rax
1405a0a78: lea rax, [rax + rax*2]
1405a0a7c: shl rax, 5
1405a0a80: cmp qword ptr [rcx + rax + 0x20], 0
1405a0a86: jne 0x1405a0acf
1405a0a88: mov byte ptr [rbx + 0xaa], 1
1405a0a8f: mov byte ptr [rbx + 0xab], 1
1405a0a96: mov byte ptr [rbx + 0xac], 1
1405a0a9d: mov byte ptr [rbx + 0xad], 1
1405a0aa4: mov byte ptr [rbx + 0xae], 0
1405a0aab: mov byte ptr [rbx + 0xaf], 1
1405a0ab2: mov byte ptr [rbx + 0xb0], 0
1405a0ab9: mov byte ptr [rbx + 0xb1], 1
1405a0ac0: mov byte ptr [rbx + 0xb2], 1
1405a0ac7: mov rcx, r15
1405a0aca: call 0x140584520
1405a0acf: mov byte ptr [rbx + 0xaa], 1
1405a0ad6: mov byte ptr [rbx + 0xab], 1
1405a0add: mov byte ptr [rbx + 0xac], 1
1405a0ae4: mov byte ptr [rbx + 0xad], 1
1405a0aeb: mov byte ptr [rbx + 0xae], 0
1405a0af2: mov byte ptr [rbx + 0xaf], 1
1405a0af9: mov byte ptr [rbx + 0xb0], 0
1405a0b00: mov byte ptr [rbx + 0xb1], 1
1405a0b07: mov byte ptr [rbx + 0xb2], 1
1405a0b0e: movabs rdx, 0x25b1b2cd3a0e7966
1405a0b18: movabs r8, 0x102186ba43f6414e
1405a0b22: mov rcx, r12
1405a0b25: call 0x1400dd6f0
1405a0b2a: test rax, rax
1405a0b2d: je 0x1405a0b62
1405a0b2f: cmp qword ptr [r15 + 0x218], rdx
1405a0b36: jbe 0x1405a0b62
1405a0b38: mov rax, qword ptr [r15 + 0x210]
1405a0b3f: mov rax, qword ptr [rax + rdx*8]
1405a0b43: test rax, rax
1405a0b46: je 0x1405a0b62
1405a0b48: mov rcx, qword ptr [r15 + 0x1e0]
1405a0b4f: not rax
1405a0b52: lea rax, [rax + rax*2]
1405a0b56: shl rax, 5
1405a0b5a: cmp qword ptr [rcx + rax + 0x20], 0
1405a0b60: jne 0x1405a0ba9
1405a0b62: mov byte ptr [rbx + 0xaa], 1
1405a0b69: mov byte ptr [rbx + 0xab], 1
1405a0b70: mov byte ptr [rbx + 0xac], 1
1405a0b77: mov byte ptr [rbx + 0xad], 1
1405a0b7e: mov byte ptr [rbx + 0xae], 0
1405a0b85: mov byte ptr [rbx + 0xaf], 1
1405a0b8c: mov byte ptr [rbx + 0xb0], 0
1405a0b93: mov byte ptr [rbx + 0xb1], 1
1405a0b9a: mov byte ptr [rbx + 0xb2], 1
1405a0ba1: mov rcx, r15
1405a0ba4: call 0x14057cf90
1405a0ba9: mov byte ptr [rbx + 0xaa], 1
1405a0bb0: mov byte ptr [rbx + 0xab], 1
1405a0bb7: mov byte ptr [rbx + 0xac], 1
1405a0bbe: mov byte ptr [rbx + 0xad], 1
1405a0bc5: mov byte ptr [rbx + 0xae], 0
1405a0bcc: mov byte ptr [rbx + 0xaf], 1
1405a0bd3: mov byte ptr [rbx + 0xb0], 0
1405a0bda: mov byte ptr [rbx + 0xb1], 1
1405a0be1: mov byte ptr [rbx + 0xb2], 1
1405a0be8: movabs rdx, 0x1be1b6f9ce2b6798
1405a0bf2: movabs r8, 0xd606952bfb407b93
1405a0bfc: mov rcx, r12
1405a0bff: call 0x1400dd6f0
1405a0c04: test rax, rax
1405a0c07: je 0x1405a0c3c
1405a0c09: cmp qword ptr [r15 + 0x218], rdx
1405a0c10: jbe 0x1405a0c3c
1405a0c12: mov rax, qword ptr [r15 + 0x210]
1405a0c19: mov rax, qword ptr [rax + rdx*8]
1405a0c1d: test rax, rax
1405a0c20: je 0x1405a0c3c
1405a0c22: mov rcx, qword ptr [r15 + 0x1e0]
1405a0c29: not rax
1405a0c2c: lea rax, [rax + rax*2]
1405a0c30: shl rax, 5
1405a0c34: cmp qword ptr [rcx + rax + 0x20], 0
1405a0c3a: jne 0x1405a0c83
1405a0c3c: mov byte ptr [rbx + 0xaa], 1
1405a0c43: mov byte ptr [rbx + 0xab], 1
1405a0c4a: mov byte ptr [rbx + 0xac], 1
1405a0c51: mov byte ptr [rbx + 0xad], 1
1405a0c58: mov byte ptr [rbx + 0xae], 0
1405a0c5f: mov byte ptr [rbx + 0xaf], 1
1405a0c66: mov byte ptr [rbx + 0xb0], 0
1405a0c6d: mov byte ptr [rbx + 0xb1], 1
1405a0c74: mov byte ptr [rbx + 0xb2], 1
1405a0c7b: mov rcx, r15
1405a0c7e: call 0x140571c20
1405a0c83: mov byte ptr [rbx + 0xaa], 1
1405a0c8a: mov byte ptr [rbx + 0xab], 1
1405a0c91: mov byte ptr [rbx + 0xac], 1
1405a0c98: mov byte ptr [rbx + 0xad], 1
1405a0c9f: mov byte ptr [rbx + 0xae], 0
1405a0ca6: mov byte ptr [rbx + 0xaf], 1
1405a0cad: mov byte ptr [rbx + 0xb0], 0
1405a0cb4: mov byte ptr [rbx + 0xb1], 1
1405a0cbb: mov byte ptr [rbx + 0xb2], 1
1405a0cc2: movabs rdx, 0xcf31a175ed46a479
1405a0ccc: movabs r8, 0x7ed3edadd1d139e1
1405a0cd6: mov rcx, r12
1405a0cd9: call 0x1400dd6f0
1405a0cde: test rax, rax
1405a0ce1: je 0x1405a0d16
1405a0ce3: cmp qword ptr [r15 + 0x218], rdx
1405a0cea: jbe 0x1405a0d16
1405a0cec: mov rax, qword ptr [r15 + 0x210]
1405a0cf3: mov rax, qword ptr [rax + rdx*8]
1405a0cf7: test rax, rax
1405a0cfa: je 0x1405a0d16
1405a0cfc: mov rcx, qword ptr [r15 + 0x1e0]
1405a0d03: not rax
1405a0d06: lea rax, [rax + rax*2]
1405a0d0a: shl rax, 5
1405a0d0e: cmp qword ptr [rcx + rax + 0x20], 0
1405a0d14: jne 0x1405a0d5d
1405a0d16: mov byte ptr [rbx + 0xaa], 1
1405a0d1d: mov byte ptr [rbx + 0xab], 1
1405a0d24: mov byte ptr [rbx + 0xac], 1
1405a0d2b: mov byte ptr [rbx + 0xad], 1
1405a0d32: mov byte ptr [rbx + 0xae], 0
1405a0d39: mov byte ptr [rbx + 0xaf], 1
1405a0d40: mov byte ptr [rbx + 0xb0], 0
1405a0d47: mov byte ptr [rbx + 0xb1], 1
1405a0d4e: mov byte ptr [rbx + 0xb2], 1
1405a0d55: mov rcx, r15
1405a0d58: call 0x140586410
1405a0d5d: mov byte ptr [rbx + 0xaa], 1
1405a0d64: mov byte ptr [rbx + 0xab], 1
1405a0d6b: mov byte ptr [rbx + 0xac], 1
1405a0d72: mov byte ptr [rbx + 0xad], 1
1405a0d79: mov byte ptr [rbx + 0xae], 0
1405a0d80: mov byte ptr [rbx + 0xaf], 1
1405a0d87: mov byte ptr [rbx + 0xb0], 0
1405a0d8e: mov byte ptr [rbx + 0xb1], 1
1405a0d95: mov byte ptr [rbx + 0xb2], 1
1405a0d9c: movabs rdx, 0x2394d67f95c7f498
1405a0da6: movabs r8, 0x800129258aa79cc6
1405a0db0: mov rcx, r12
1405a0db3: call 0x1400dd6f0
1405a0db8: test rax, rax
1405a0dbb: je 0x1405a0df0
1405a0dbd: cmp qword ptr [r15 + 0x218], rdx
1405a0dc4: jbe 0x1405a0df0
1405a0dc6: mov rax, qword ptr [r15 + 0x210]
1405a0dcd: mov rax, qword ptr [rax + rdx*8]
1405a0dd1: test rax, rax
1405a0dd4: je 0x1405a0df0
1405a0dd6: mov rcx, qword ptr [r15 + 0x1e0]
1405a0ddd: not rax
1405a0de0: lea rax, [rax + rax*2]
1405a0de4: shl rax, 5
1405a0de8: cmp qword ptr [rcx + rax + 0x20], 0
1405a0dee: jne 0x1405a0e37
1405a0df0: mov byte ptr [rbx + 0xaa], 1
1405a0df7: mov byte ptr [rbx + 0xab], 1
1405a0dfe: mov byte ptr [rbx + 0xac], 1
1405a0e05: mov byte ptr [rbx + 0xad], 1
1405a0e0c: mov byte ptr [rbx + 0xae], 0
1405a0e13: mov byte ptr [rbx + 0xaf], 1
1405a0e1a: mov byte ptr [rbx + 0xb0], 0
1405a0e21: mov byte ptr [rbx + 0xb1], 1
1405a0e28: mov byte ptr [rbx + 0xb2], 1
1405a0e2f: mov rcx, r15
1405a0e32: call 0x14056a060
1405a0e37: mov byte ptr [rbx + 0xaa], 1
1405a0e3e: mov byte ptr [rbx + 0xab], 1
1405a0e45: mov byte ptr [rbx + 0xac], 1
1405a0e4c: mov byte ptr [rbx + 0xad], 1
1405a0e53: mov byte ptr [rbx + 0xae], 0
1405a0e5a: mov byte ptr [rbx + 0xaf], 1
1405a0e61: mov byte ptr [rbx + 0xb0], 0
1405a0e68: mov byte ptr [rbx + 0xb1], 1
1405a0e6f: mov byte ptr [rbx + 0xb2], 1
1405a0e76: movabs rdx, 0xd1de6c9a7f2b6c4f
1405a0e80: movabs r8, 0xf073d18fea2ba51e
1405a0e8a: mov rcx, r12
1405a0e8d: call 0x1400dd6f0
1405a0e92: test rax, rax
1405a0e95: je 0x1405a0eca
1405a0e97: cmp qword ptr [r15 + 0x218], rdx
1405a0e9e: jbe 0x1405a0eca
1405a0ea0: mov rax, qword ptr [r15 + 0x210]
1405a0ea7: mov rax, qword ptr [rax + rdx*8]
1405a0eab: test rax, rax
1405a0eae: je 0x1405a0eca
1405a0eb0: mov rcx, qword ptr [r15 + 0x1e0]
1405a0eb7: not rax
1405a0eba: lea rax, [rax + rax*2]
1405a0ebe: shl rax, 5
1405a0ec2: cmp qword ptr [rcx + rax + 0x20], 0
1405a0ec8: jne 0x1405a0f11
1405a0eca: mov byte ptr [rbx + 0xaa], 1
1405a0ed1: mov byte ptr [rbx + 0xab], 1
1405a0ed8: mov byte ptr [rbx + 0xac], 1
1405a0edf: mov byte ptr [rbx + 0xad], 1
1405a0ee6: mov byte ptr [rbx + 0xae], 0
1405a0eed: mov byte ptr [rbx + 0xaf], 1
1405a0ef4: mov byte ptr [rbx + 0xb0], 0
1405a0efb: mov byte ptr [rbx + 0xb1], 1
1405a0f02: mov byte ptr [rbx + 0xb2], 1
1405a0f09: mov rcx, r15
1405a0f0c: call 0x140585de0
1405a0f11: mov byte ptr [rbx + 0xaa], 1
1405a0f18: mov byte ptr [rbx + 0xab], 1
1405a0f1f: mov byte ptr [rbx + 0xac], 1
1405a0f26: mov byte ptr [rbx + 0xad], 1
1405a0f2d: mov byte ptr [rbx + 0xae], 0
1405a0f34: mov byte ptr [rbx + 0xaf], 1
1405a0f3b: mov byte ptr [rbx + 0xb0], 0
1405a0f42: mov byte ptr [rbx + 0xb1], 1
1405a0f49: mov byte ptr [rbx + 0xb2], 1
1405a0f50: movabs rdx, 0xb4f0d2490f280d80
1405a0f5a: movabs r8, 0xf8449329f12602fe
1405a0f64: mov rcx, r12
1405a0f67: call 0x1400dd6f0
1405a0f6c: test rax, rax
1405a0f6f: je 0x1405a0fa4
1405a0f71: cmp qword ptr [r15 + 0x218], rdx
1405a0f78: jbe 0x1405a0fa4
1405a0f7a: mov rax, qword ptr [r15 + 0x210]
1405a0f81: mov rax, qword ptr [rax + rdx*8]
1405a0f85: test rax, rax
1405a0f88: je 0x1405a0fa4
1405a0f8a: mov rcx, qword ptr [r15 + 0x1e0]
1405a0f91: not rax
1405a0f94: lea rax, [rax + rax*2]
1405a0f98: shl rax, 5
1405a0f9c: cmp qword ptr [rcx + rax + 0x20], 0
1405a0fa2: jne 0x1405a0feb
1405a0fa4: mov byte ptr [rbx + 0xaa], 1
1405a0fab: mov byte ptr [rbx + 0xab], 1
1405a0fb2: mov byte ptr [rbx + 0xac], 1
1405a0fb9: mov byte ptr [rbx + 0xad], 1
1405a0fc0: mov byte ptr [rbx + 0xae], 0
1405a0fc7: mov byte ptr [rbx + 0xaf], 1
1405a0fce: mov byte ptr [rbx + 0xb0], 0
1405a0fd5: mov byte ptr [rbx + 0xb1], 1
1405a0fdc: mov byte ptr [rbx + 0xb2], 1
1405a0fe3: mov rcx, r15
1405a0fe6: call 0x1405668b0
1405a0feb: mov byte ptr [rbx + 0xaa], 1
1405a0ff2: mov byte ptr [rbx + 0xab], 1
1405a0ff9: mov byte ptr [rbx + 0xac], 1
1405a1000: mov byte ptr [rbx + 0xad], 1
1405a1007: mov byte ptr [rbx + 0xae], 0
1405a100e: mov byte ptr [rbx + 0xaf], 1
1405a1015: mov byte ptr [rbx + 0xb0], 0
1405a101c: mov byte ptr [rbx + 0xb1], 1
1405a1023: mov byte ptr [rbx + 0xb2], 1
1405a102a: movabs rdx, 0xa554fbb3bbf1ca0
1405a1034: movabs r8, 0x29b993a986bb8e9d
1405a103e: mov rcx, r12
1405a1041: call 0x1400dd6f0
1405a1046: test rax, rax
1405a1049: je 0x1405a107e
1405a104b: cmp qword ptr [r15 + 0x218], rdx
1405a1052: jbe 0x1405a107e
1405a1054: mov rax, qword ptr [r15 + 0x210]
1405a105b: mov rax, qword ptr [rax + rdx*8]
1405a105f: test rax, rax
1405a1062: je 0x1405a107e
1405a1064: mov rcx, qword ptr [r15 + 0x1e0]
1405a106b: not rax
1405a106e: lea rax, [rax + rax*2]
1405a1072: shl rax, 5
1405a1076: cmp qword ptr [rcx + rax + 0x20], 0
1405a107c: jne 0x1405a10c5
1405a107e: mov byte ptr [rbx + 0xaa], 1
1405a1085: mov byte ptr [rbx + 0xab], 1
1405a108c: mov byte ptr [rbx + 0xac], 1
1405a1093: mov byte ptr [rbx + 0xad], 1
1405a109a: mov byte ptr [rbx + 0xae], 0
1405a10a1: mov byte ptr [rbx + 0xaf], 1
1405a10a8: mov byte ptr [rbx + 0xb0], 0
1405a10af: mov byte ptr [rbx + 0xb1], 1
1405a10b6: mov byte ptr [rbx + 0xb2], 1
1405a10bd: mov rcx, r15
1405a10c0: call 0x1405797e0
1405a10c5: mov byte ptr [rbx + 0xaa], 1
1405a10cc: mov byte ptr [rbx + 0xab], 1
1405a10d3: mov byte ptr [rbx + 0xac], 1
1405a10da: mov byte ptr [rbx + 0xad], 1
1405a10e1: mov byte ptr [rbx + 0xae], 0
1405a10e8: mov byte ptr [rbx + 0xaf], 1
1405a10ef: mov byte ptr [rbx + 0xb0], 0
1405a10f6: mov byte ptr [rbx + 0xb1], 1
1405a10fd: mov byte ptr [rbx + 0xb2], 1
1405a1104: movabs rdx, 0xdb5e6652a838d059
1405a110e: movabs r8, 0x820aea602265b9f1
1405a1118: mov rcx, r12
1405a111b: call 0x1400dd6f0
1405a1120: test rax, rax
1405a1123: je 0x1405a1158
1405a1125: cmp qword ptr [r15 + 0x218], rdx
1405a112c: jbe 0x1405a1158
1405a112e: mov rax, qword ptr [r15 + 0x210]
1405a1135: mov rax, qword ptr [rax + rdx*8]
1405a1139: test rax, rax
1405a113c: je 0x1405a1158
1405a113e: mov rcx, qword ptr [r15 + 0x1e0]
1405a1145: not rax
1405a1148: lea rax, [rax + rax*2]
1405a114c: shl rax, 5
1405a1150: cmp qword ptr [rcx + rax + 0x20], 0
1405a1156: jne 0x1405a119f
1405a1158: mov byte ptr [rbx + 0xaa], 1
1405a115f: mov byte ptr [rbx + 0xab], 1
1405a1166: mov byte ptr [rbx + 0xac], 1
1405a116d: mov byte ptr [rbx + 0xad], 1
1405a1174: mov byte ptr [rbx + 0xae], 0
1405a117b: mov byte ptr [rbx + 0xaf], 1
1405a1182: mov byte ptr [rbx + 0xb0], 0
1405a1189: mov byte ptr [rbx + 0xb1], 1
1405a1190: mov byte ptr [rbx + 0xb2], 1
1405a1197: mov rcx, r15
1405a119a: call 0x14057dbf0
1405a119f: mov byte ptr [rbx + 0xaa], 1
1405a11a6: mov byte ptr [rbx + 0xab], 1
1405a11ad: mov byte ptr [rbx + 0xac], 1
1405a11b4: mov byte ptr [rbx + 0xad], 1
1405a11bb: mov byte ptr [rbx + 0xae], 0
1405a11c2: mov byte ptr [rbx + 0xaf], 1
1405a11c9: mov byte ptr [rbx + 0xb0], 0
1405a11d0: mov byte ptr [rbx + 0xb1], 1
1405a11d7: mov byte ptr [rbx + 0xb2], 1
1405a11de: movabs rdx, 0x3a04fec58ff7409b
1405a11e8: movabs r8, 0xccf689c1cb3e03a2
1405a11f2: mov rcx, r12
1405a11f5: call 0x1400dd6f0
1405a11fa: test rax, rax
1405a11fd: je 0x1405a1232
1405a11ff: cmp qword ptr [r15 + 0x218], rdx
1405a1206: jbe 0x1405a1232
1405a1208: mov rax, qword ptr [r15 + 0x210]
1405a120f: mov rax, qword ptr [rax + rdx*8]
1405a1213: test rax, rax
1405a1216: je 0x1405a1232
1405a1218: mov rcx, qword ptr [r15 + 0x1e0]
1405a121f: not rax
1405a1222: lea rax, [rax + rax*2]
1405a1226: shl rax, 5
1405a122a: cmp qword ptr [rcx + rax + 0x20], 0
1405a1230: jne 0x1405a1279
1405a1232: mov byte ptr [rbx + 0xaa], 1
1405a1239: mov byte ptr [rbx + 0xab], 1
1405a1240: mov byte ptr [rbx + 0xac], 1
1405a1247: mov byte ptr [rbx + 0xad], 1
1405a124e: mov byte ptr [rbx + 0xae], 0
1405a1255: mov byte ptr [rbx + 0xaf], 1
1405a125c: mov byte ptr [rbx + 0xb0], 0
1405a1263: mov byte ptr [rbx + 0xb1], 1
1405a126a: mov byte ptr [rbx + 0xb2], 1
1405a1271: mov rcx, r15
1405a1274: call 0x1405819d0
1405a1279: mov byte ptr [rbx + 0xaa], 1
1405a1280: mov byte ptr [rbx + 0xab], 1
1405a1287: mov byte ptr [rbx + 0xac], 1
1405a128e: mov byte ptr [rbx + 0xad], 1
1405a1295: mov byte ptr [rbx + 0xae], 0
1405a129c: mov byte ptr [rbx + 0xaf], 1
1405a12a3: mov byte ptr [rbx + 0xb0], 0
1405a12aa: mov byte ptr [rbx + 0xb1], 1
1405a12b1: mov byte ptr [rbx + 0xb2], 1
1405a12b8: movabs rdx, 0x2c026e94513be739
1405a12c2: movabs r8, 0xa05489b04f4c6987
1405a12cc: mov rcx, r12
1405a12cf: call 0x1400dd6f0
1405a12d4: test rax, rax
1405a12d7: je 0x1405a130c
1405a12d9: cmp qword ptr [r15 + 0x218], rdx
1405a12e0: jbe 0x1405a130c
1405a12e2: mov rax, qword ptr [r15 + 0x210]
1405a12e9: mov rax, qword ptr [rax + rdx*8]
1405a12ed: test rax, rax
1405a12f0: je 0x1405a130c
1405a12f2: mov rcx, qword ptr [r15 + 0x1e0]
1405a12f9: not rax
1405a12fc: lea rax, [rax + rax*2]
1405a1300: shl rax, 5
1405a1304: cmp qword ptr [rcx + rax + 0x20], 0
1405a130a: jne 0x1405a1353
1405a130c: mov byte ptr [rbx + 0xaa], 1
1405a1313: mov byte ptr [rbx + 0xab], 1
1405a131a: mov byte ptr [rbx + 0xac], 1
1405a1321: mov byte ptr [rbx + 0xad], 1
1405a1328: mov byte ptr [rbx + 0xae], 0
1405a132f: mov byte ptr [rbx + 0xaf], 1
1405a1336: mov byte ptr [rbx + 0xb0], 0
1405a133d: mov byte ptr [rbx + 0xb1], 1
1405a1344: mov byte ptr [rbx + 0xb2], 1
1405a134b: mov rcx, r15
1405a134e: call 0x14057d5c0
1405a1353: mov byte ptr [rbx + 0xaa], 1
1405a135a: mov byte ptr [rbx + 0xab], 1
1405a1361: mov byte ptr [rbx + 0xac], 1
1405a1368: mov byte ptr [rbx + 0xad], 1
1405a136f: mov byte ptr [rbx + 0xae], 0
1405a1376: mov byte ptr [rbx + 0xaf], 1
1405a137d: mov byte ptr [rbx + 0xb0], 0
1405a1384: mov byte ptr [rbx + 0xb1], 1
1405a138b: mov byte ptr [rbx + 0xb2], 1
1405a1392: movabs rdx, 0x3b770906f36e3dd8
1405a139c: movabs r8, 0x1bce870c4bce18ee
1405a13a6: mov rcx, r12
1405a13a9: call 0x1400dd6f0
1405a13ae: test rax, rax
1405a13b1: je 0x1405a13e6
1405a13b3: cmp qword ptr [r15 + 0x218], rdx
1405a13ba: jbe 0x1405a13e6
1405a13bc: mov rax, qword ptr [r15 + 0x210]
1405a13c3: mov rax, qword ptr [rax + rdx*8]
1405a13c7: test rax, rax
1405a13ca: je 0x1405a13e6
1405a13cc: mov rcx, qword ptr [r15 + 0x1e0]
1405a13d3: not rax
1405a13d6: lea rax, [rax + rax*2]
1405a13da: shl rax, 5
1405a13de: cmp qword ptr [rcx + rax + 0x20], 0
1405a13e4: jne 0x1405a142d
1405a13e6: mov byte ptr [rbx + 0xaa], 1
1405a13ed: mov byte ptr [rbx + 0xab], 1
1405a13f4: mov byte ptr [rbx + 0xac], 1
1405a13fb: mov byte ptr [rbx + 0xad], 1
1405a1402: mov byte ptr [rbx + 0xae], 0
1405a1409: mov byte ptr [rbx + 0xaf], 1
1405a1410: mov byte ptr [rbx + 0xb0], 0
1405a1417: mov byte ptr [rbx + 0xb1], 1
1405a141e: mov byte ptr [rbx + 0xb2], 1
1405a1425: mov rcx, r15
1405a1428: call 0x14057ee80
1405a142d: mov byte ptr [rbx + 0xaa], 1
1405a1434: mov byte ptr [rbx + 0xab], 1
1405a143b: mov byte ptr [rbx + 0xac], 1
1405a1442: mov byte ptr [rbx + 0xad], 1
1405a1449: mov byte ptr [rbx + 0xae], 0
1405a1450: mov byte ptr [rbx + 0xaf], 1
1405a1457: mov byte ptr [rbx + 0xb0], 0
1405a145e: mov byte ptr [rbx + 0xb1], 1
1405a1465: mov byte ptr [rbx + 0xb2], 1
1405a146c: mov rcx, r15
1405a146f: mov dl, 2
1405a1471: call 0x1401295c0
1405a1476: mov rsi, rax
1405a1479: mov qword ptr [rbx + 0x440], 0
1405a1484: xorps xmm0, xmm0
1405a1487: movups xmmword ptr [rbx + 0x450], xmm0
1405a148e: mov qword ptr [rbx + 0x448], 8
1405a1499: call 0x140d4d740
1405a149e: mov rdi, rax
1405a14a1: test byte ptr [rax], 1
1405a14a4: je 0x1405a14b7
1405a14a6: mov rcx, rdi
1405a14a9: add rcx, 8
1405a14ad: mov rax, qword ptr [rdi + 8]
1405a14b1: mov rdx, qword ptr [rdi + 0x10]
1405a14b5: jmp 0x1405a14d2
1405a14b7: call 0x140d64e00
1405a14bc: mov rcx, rdi
1405a14bf: add rcx, 8
1405a14c3: mov qword ptr [rdi], 1
1405a14ca: mov qword ptr [rdi + 8], rax
1405a14ce: mov qword ptr [rdi + 0x10], rdx
1405a14d2: lea r8, [rax + 1]
1405a14d6: mov qword ptr [rcx], r8
1405a14d9: movups xmm0, xmmword ptr [rbx + 0x440]
1405a14e0: movups xmm1, xmmword ptr [rbx + 0x450]
1405a14e7: movaps xmmword ptr [rbx + 0xb10], xmm1
1405a14ee: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a14f5: movaps xmmword ptr [rbx + 0xb20], xmm7
1405a14fc: movaps xmmword ptr [rbx + 0xb30], xmm6
1405a1503: mov qword ptr [rbx + 0xb40], rax
1405a150a: mov qword ptr [rbx + 0xb48], rdx
1405a1511: mov byte ptr [rbx + 0xaa], 1
1405a1518: mov byte ptr [rbx + 0xab], 1
1405a151f: mov byte ptr [rbx + 0xac], 1
1405a1526: mov byte ptr [rbx + 0xad], 1
1405a152d: mov byte ptr [rbx + 0xae], 0
1405a1534: mov byte ptr [rbx + 0xaf], 1
1405a153b: mov byte ptr [rbx + 0xb0], 0
1405a1542: mov byte ptr [rbx + 0xb1], 1
1405a1549: mov byte ptr [rbx + 0xb2], 1
1405a1550: lea rdx, [rbx + 0xb00]
1405a1557: mov rcx, rsi
1405a155a: call 0x140118630
1405a155f: mov rsi, rax
1405a1562: mov byte ptr [rbx + 0xaa], 1
1405a1569: mov byte ptr [rbx + 0xab], 1
1405a1570: mov byte ptr [rbx + 0xac], 1
1405a1577: mov byte ptr [rbx + 0xad], 1
1405a157e: mov byte ptr [rbx + 0xae], 0
1405a1585: mov byte ptr [rbx + 0xaf], 1
1405a158c: mov byte ptr [rbx + 0xb0], 0
1405a1593: mov byte ptr [rbx + 0xb1], 1
1405a159a: mov byte ptr [rbx + 0xb2], 1
1405a15a1: lea rcx, [rbx + 0xb00]
1405a15a8: call 0x140a86b30
1405a15ad: mov byte ptr [rbx + 0xaa], 1
1405a15b4: mov byte ptr [rbx + 0xab], 1
1405a15bb: mov byte ptr [rbx + 0xac], 1
1405a15c2: mov byte ptr [rbx + 0xad], 1
1405a15c9: mov byte ptr [rbx + 0xae], 0
1405a15d0: mov byte ptr [rbx + 0xaf], 1
1405a15d7: mov byte ptr [rbx + 0xb0], 0
1405a15de: mov byte ptr [rbx + 0xb1], 1
1405a15e5: mov byte ptr [rbx + 0xb2], 1
1405a15ec: lea rdx, [rbx + 0xb00]
1405a15f3: mov rcx, rsi
1405a15f6: call 0x1401180f0
1405a15fb: mov rsi, rax
1405a15fe: call 0x140d4d740
1405a1603: mov rdi, rax
1405a1606: test byte ptr [rax], 1
1405a1609: je 0x1405a161c
1405a160b: mov rcx, rdi
1405a160e: add rcx, 8
1405a1612: mov rax, qword ptr [rdi + 8]
1405a1616: mov rdx, qword ptr [rdi + 0x10]
1405a161a: jmp 0x1405a1676
1405a161c: mov byte ptr [rbx + 0xaa], 1
1405a1623: mov byte ptr [rbx + 0xab], 1
1405a162a: mov byte ptr [rbx + 0xac], 1
1405a1631: mov byte ptr [rbx + 0xad], 1
1405a1638: mov byte ptr [rbx + 0xae], 0
1405a163f: mov byte ptr [rbx + 0xaf], 1
1405a1646: mov byte ptr [rbx + 0xb0], 0
1405a164d: mov byte ptr [rbx + 0xb1], 1
1405a1654: mov byte ptr [rbx + 0xb2], 1
1405a165b: call 0x140d64e00
1405a1660: mov rcx, rdi
1405a1663: add rcx, 8
1405a1667: mov qword ptr [rdi], 1
1405a166e: mov qword ptr [rdi + 8], rax
1405a1672: mov qword ptr [rdi + 0x10], rdx
1405a1676: lea r8, [rax + 1]
1405a167a: mov qword ptr [rcx], r8
1405a167d: movaps xmmword ptr [rbx + 0xb10], xmm6
1405a1684: movaps xmmword ptr [rbx + 0xb00], xmm7
1405a168b: mov qword ptr [rbx + 0xb20], rax
1405a1692: mov qword ptr [rbx + 0xb28], rdx
1405a1699: mov byte ptr [rbx + 0xaa], 1
1405a16a0: mov byte ptr [rbx + 0xab], 1
1405a16a7: mov byte ptr [rbx + 0xac], 1
1405a16ae: mov byte ptr [rbx + 0xad], 1
1405a16b5: mov byte ptr [rbx + 0xae], 0
1405a16bc: mov byte ptr [rbx + 0xaf], 1
1405a16c3: mov byte ptr [rbx + 0xb0], 0
1405a16ca: mov byte ptr [rbx + 0xb1], 1
1405a16d1: mov byte ptr [rbx + 0xb2], 1
1405a16d8: lea rdx, [rbx + 0xb00]
1405a16df: mov rcx, rsi
1405a16e2: call 0x140131170
1405a16e7: mov rsi, rax
1405a16ea: call 0x140d4d740
1405a16ef: mov rdi, rax
1405a16f2: test byte ptr [rax], 1
1405a16f5: je 0x1405a1708
1405a16f7: mov rcx, rdi
1405a16fa: add rcx, 8
1405a16fe: mov rax, qword ptr [rdi + 8]
1405a1702: mov rdx, qword ptr [rdi + 0x10]
1405a1706: jmp 0x1405a1762
1405a1708: mov byte ptr [rbx + 0xaa], 1
1405a170f: mov byte ptr [rbx + 0xab], 1
1405a1716: mov byte ptr [rbx + 0xac], 1
1405a171d: mov byte ptr [rbx + 0xad], 1
1405a1724: mov byte ptr [rbx + 0xae], 0
1405a172b: mov byte ptr [rbx + 0xaf], 1
1405a1732: mov byte ptr [rbx + 0xb0], 0
1405a1739: mov byte ptr [rbx + 0xb1], 1
1405a1740: mov byte ptr [rbx + 0xb2], 1
1405a1747: call 0x140d64e00
1405a174c: mov rcx, rdi
1405a174f: add rcx, 8
1405a1753: mov qword ptr [rdi], 1
1405a175a: mov qword ptr [rdi + 8], rax
1405a175e: mov qword ptr [rdi + 0x10], rdx
1405a1762: lea r8, [rax + 1]
1405a1766: mov qword ptr [rcx], r8
1405a1769: movaps xmmword ptr [rbx + 0xb10], xmm6
1405a1770: movaps xmmword ptr [rbx + 0xb00], xmm7
1405a1777: mov qword ptr [rbx + 0xb20], rax
1405a177e: mov qword ptr [rbx + 0xb28], rdx
1405a1785: mov byte ptr [rbx + 0xaa], 1
1405a178c: mov byte ptr [rbx + 0xab], 1
1405a1793: mov byte ptr [rbx + 0xac], 1
1405a179a: mov byte ptr [rbx + 0xad], 1
1405a17a1: mov byte ptr [rbx + 0xae], 0
1405a17a8: mov byte ptr [rbx + 0xaf], 1
1405a17af: mov byte ptr [rbx + 0xb0], 0
1405a17b6: mov byte ptr [rbx + 0xb1], 1
1405a17bd: mov byte ptr [rbx + 0xb2], 1
1405a17c4: lea rdx, [rbx + 0xb00]
1405a17cb: mov rcx, rsi
1405a17ce: call 0x14011f240
1405a17d3: mov rsi, rax
1405a17d6: mov byte ptr [rbx + 0xaa], 1
1405a17dd: mov byte ptr [rbx + 0xab], 1
1405a17e4: mov byte ptr [rbx + 0xac], 1
1405a17eb: mov byte ptr [rbx + 0xad], 1
1405a17f2: mov byte ptr [rbx + 0xae], 0
1405a17f9: mov byte ptr [rbx + 0xaf], 1
1405a1800: mov byte ptr [rbx + 0xb0], 0
1405a1807: mov byte ptr [rbx + 0xb1], 1
1405a180e: mov byte ptr [rbx + 0xb2], 1
1405a1815: lea rcx, [rbx + 0xb00]
1405a181c: call 0x140af8c50
1405a1821: mov byte ptr [rbx + 0xaa], 1
1405a1828: mov byte ptr [rbx + 0xab], 1
1405a182f: mov byte ptr [rbx + 0xac], 1
1405a1836: mov byte ptr [rbx + 0xad], 1
1405a183d: mov byte ptr [rbx + 0xae], 0
1405a1844: mov byte ptr [rbx + 0xaf], 1
1405a184b: mov byte ptr [rbx + 0xb0], 0
1405a1852: mov byte ptr [rbx + 0xb1], 1
1405a1859: mov byte ptr [rbx + 0xb2], 1
1405a1860: lea rdx, [rbx + 0xb00]
1405a1867: mov rcx, rsi
1405a186a: call 0x14013f8f0
1405a186f: mov rsi, rax
1405a1872: mov byte ptr [rbx + 0xaa], 1
1405a1879: mov byte ptr [rbx + 0xab], 1
1405a1880: mov byte ptr [rbx + 0xac], 1
1405a1887: mov byte ptr [rbx + 0xad], 1
1405a188e: mov byte ptr [rbx + 0xae], 0
1405a1895: mov byte ptr [rbx + 0xaf], 1
1405a189c: mov byte ptr [rbx + 0xb0], 0
1405a18a3: mov byte ptr [rbx + 0xb1], 1
1405a18aa: mov byte ptr [rbx + 0xb2], 1
1405a18b1: lea rcx, [rbx + 0xb00]
1405a18b8: call 0x1408c2240
1405a18bd: mov byte ptr [rbx + 0xaa], 1
1405a18c4: mov byte ptr [rbx + 0xab], 1
1405a18cb: mov byte ptr [rbx + 0xac], 1
1405a18d2: mov byte ptr [rbx + 0xad], 1
1405a18d9: mov byte ptr [rbx + 0xae], 0
1405a18e0: mov byte ptr [rbx + 0xaf], 1
1405a18e7: mov byte ptr [rbx + 0xb0], 0
1405a18ee: mov byte ptr [rbx + 0xb1], 1
1405a18f5: mov byte ptr [rbx + 0xb2], 1
1405a18fc: lea rdx, [rbx + 0xb00]
1405a1903: mov rcx, rsi
1405a1906: call 0x14011d6b0
1405a190b: mov byte ptr [rbx + 0xaa], 1
1405a1912: mov byte ptr [rbx + 0xab], 1
1405a1919: mov byte ptr [rbx + 0xac], 1
1405a1920: mov byte ptr [rbx + 0xad], 1
1405a1927: mov byte ptr [rbx + 0xae], 0
1405a192e: mov byte ptr [rbx + 0xaf], 1
1405a1935: mov byte ptr [rbx + 0xb0], 0
1405a193c: mov byte ptr [rbx + 0xb1], 1
1405a1943: mov byte ptr [rbx + 0xb2], 1
1405a194a: mov rcx, rax
1405a194d: xor edx, edx
1405a194f: call 0x14012c910
1405a1954: mov byte ptr [rbx + 0xaa], 1
1405a195b: mov byte ptr [rbx + 0xab], 1
1405a1962: mov byte ptr [rbx + 0xac], 1
1405a1969: mov byte ptr [rbx + 0xad], 1
1405a1970: mov byte ptr [rbx + 0xae], 0
1405a1977: mov byte ptr [rbx + 0xaf], 1
1405a197e: mov byte ptr [rbx + 0xb0], 0
1405a1985: mov byte ptr [rbx + 0xb1], 1
1405a198c: mov byte ptr [rbx + 0xb2], 1
1405a1993: mov rcx, rax
1405a1996: xor edx, edx
1405a1998: call 0x14011b520
1405a199d: mov rsi, rax
1405a19a0: mov byte ptr [rbx + 0xaa], 1
1405a19a7: mov byte ptr [rbx + 0xab], 1
1405a19ae: mov byte ptr [rbx + 0xac], 1
1405a19b5: mov byte ptr [rbx + 0xad], 1
1405a19bc: mov byte ptr [rbx + 0xae], 0
1405a19c3: mov byte ptr [rbx + 0xaf], 1
1405a19ca: mov byte ptr [rbx + 0xb0], 0
1405a19d1: mov byte ptr [rbx + 0xb1], 1
1405a19d8: mov byte ptr [rbx + 0xb2], 1
1405a19df: call 0x1404b0ec0
1405a19e4: mov r8d, edx
1405a19e7: mov byte ptr [rbx + 0xaa], 1
1405a19ee: mov byte ptr [rbx + 0xab], 1
1405a19f5: mov byte ptr [rbx + 0xac], 1
1405a19fc: mov byte ptr [rbx + 0xad], 1
1405a1a03: mov byte ptr [rbx + 0xae], 0
1405a1a0a: mov byte ptr [rbx + 0xaf], 1
1405a1a11: mov byte ptr [rbx + 0xb0], 0
1405a1a18: mov byte ptr [rbx + 0xb1], 1
1405a1a1f: mov byte ptr [rbx + 0xb2], 1
1405a1a26: and al, 1
1405a1a28: and r8b, 1
1405a1a2c: mov rcx, rsi
1405a1a2f: mov edx, eax
1405a1a31: call 0x14011ecf0
1405a1a36: mov qword ptr [rbx + 0xb05], 0
1405a1a41: mov qword ptr [rbx + 0xb00], 0
1405a1a4c: mov byte ptr [rbx + 0xaa], 1
1405a1a53: mov byte ptr [rbx + 0xab], 1
1405a1a5a: mov byte ptr [rbx + 0xac], 1
1405a1a61: mov byte ptr [rbx + 0xad], 1
1405a1a68: mov byte ptr [rbx + 0xae], 0
1405a1a6f: mov byte ptr [rbx + 0xaf], 1
1405a1a76: mov byte ptr [rbx + 0xb0], 0
1405a1a7d: mov byte ptr [rbx + 0xb1], 1
1405a1a84: mov byte ptr [rbx + 0xb2], 1
1405a1a8b: lea rdx, [rbx + 0xb00]
1405a1a92: mov rcx, rax
1405a1a95: call 0x14012e960
1405a1a9a: mov rsi, rax
1405a1a9d: call 0x140d4d740
1405a1aa2: mov rdi, rax
1405a1aa5: test byte ptr [rax], 1
1405a1aa8: je 0x1405a1abb
1405a1aaa: mov rcx, rdi
1405a1aad: add rcx, 8
1405a1ab1: mov rax, qword ptr [rdi + 8]
1405a1ab5: mov rdx, qword ptr [rdi + 0x10]
1405a1ab9: jmp 0x1405a1b15
1405a1abb: mov byte ptr [rbx + 0xaa], 1
1405a1ac2: mov byte ptr [rbx + 0xab], 1
1405a1ac9: mov byte ptr [rbx + 0xac], 1
1405a1ad0: mov byte ptr [rbx + 0xad], 1
1405a1ad7: mov byte ptr [rbx + 0xae], 0
1405a1ade: mov byte ptr [rbx + 0xaf], 1
1405a1ae5: mov byte ptr [rbx + 0xb0], 0
1405a1aec: mov byte ptr [rbx + 0xb1], 1
1405a1af3: mov byte ptr [rbx + 0xb2], 1
1405a1afa: call 0x140d64e00
1405a1aff: mov rcx, rdi
1405a1b02: add rcx, 8
1405a1b06: mov qword ptr [rdi], 1
1405a1b0d: mov qword ptr [rdi + 8], rax
1405a1b11: mov qword ptr [rdi + 0x10], rdx
1405a1b15: lea r8, [rax + 1]
1405a1b19: mov qword ptr [rcx], r8
1405a1b1c: movaps xmmword ptr [rbx + 0xb10], xmm6
1405a1b23: movaps xmmword ptr [rbx + 0xb00], xmm7
1405a1b2a: mov qword ptr [rbx + 0xb20], rax
1405a1b31: mov qword ptr [rbx + 0xb28], rdx
1405a1b38: mov byte ptr [rbx + 0xaa], 1
1405a1b3f: mov byte ptr [rbx + 0xab], 1
1405a1b46: mov byte ptr [rbx + 0xac], 1
1405a1b4d: mov byte ptr [rbx + 0xad], 1
1405a1b54: mov byte ptr [rbx + 0xae], 0
1405a1b5b: mov byte ptr [rbx + 0xaf], 1
1405a1b62: mov byte ptr [rbx + 0xb0], 0
1405a1b69: mov byte ptr [rbx + 0xb1], 1
1405a1b70: mov byte ptr [rbx + 0xb2], 1
1405a1b77: lea rdx, [rbx + 0xb00]
1405a1b7e: mov rcx, rsi
1405a1b81: call 0x1401272f0
1405a1b86: mov rsi, rax
1405a1b89: call 0x140d4d740
1405a1b8e: mov rdi, rax
1405a1b91: test byte ptr [rax], 1
1405a1b94: je 0x1405a1ba7
1405a1b96: mov rcx, rdi
1405a1b99: add rcx, 8
1405a1b9d: mov rax, qword ptr [rdi + 8]
1405a1ba1: mov rdx, qword ptr [rdi + 0x10]
1405a1ba5: jmp 0x1405a1c01
1405a1ba7: mov byte ptr [rbx + 0xaa], 1
1405a1bae: mov byte ptr [rbx + 0xab], 1
1405a1bb5: mov byte ptr [rbx + 0xac], 1
1405a1bbc: mov byte ptr [rbx + 0xad], 1
1405a1bc3: mov byte ptr [rbx + 0xae], 0
1405a1bca: mov byte ptr [rbx + 0xaf], 1
1405a1bd1: mov byte ptr [rbx + 0xb0], 0
1405a1bd8: mov byte ptr [rbx + 0xb1], 1
1405a1bdf: mov byte ptr [rbx + 0xb2], 1
1405a1be6: call 0x140d64e00
1405a1beb: mov rcx, rdi
1405a1bee: add rcx, 8
1405a1bf2: mov qword ptr [rdi], 1
1405a1bf9: mov qword ptr [rdi + 8], rax
1405a1bfd: mov qword ptr [rdi + 0x10], rdx
1405a1c01: lea r8, [rax + 1]
1405a1c05: mov qword ptr [rcx], r8
1405a1c08: movaps xmmword ptr [rbx + 0xb10], xmm6
1405a1c0f: movaps xmmword ptr [rbx + 0xb00], xmm7
1405a1c16: mov qword ptr [rbx + 0xb20], rax
1405a1c1d: mov qword ptr [rbx + 0xb28], rdx
1405a1c24: mov byte ptr [rbx + 0xaa], 1
1405a1c2b: mov byte ptr [rbx + 0xab], 1
1405a1c32: mov byte ptr [rbx + 0xac], 1
1405a1c39: mov byte ptr [rbx + 0xad], 1
1405a1c40: mov byte ptr [rbx + 0xae], 0
1405a1c47: mov byte ptr [rbx + 0xaf], 1
1405a1c4e: mov byte ptr [rbx + 0xb0], 0
1405a1c55: mov byte ptr [rbx + 0xb1], 1
1405a1c5c: mov byte ptr [rbx + 0xb2], 1
1405a1c63: lea rdx, [rbx + 0xb00]
1405a1c6a: mov rcx, rsi
1405a1c6d: call 0x14012acb0
1405a1c72: mov rsi, rax
1405a1c75: call 0x140d4d740
1405a1c7a: mov rdi, rax
1405a1c7d: test byte ptr [rax], 1
1405a1c80: je 0x1405a1c93
1405a1c82: mov rcx, rdi
1405a1c85: add rcx, 8
1405a1c89: mov rax, qword ptr [rdi + 8]
1405a1c8d: mov rdx, qword ptr [rdi + 0x10]
1405a1c91: jmp 0x1405a1ced
1405a1c93: mov byte ptr [rbx + 0xaa], 1
1405a1c9a: mov byte ptr [rbx + 0xab], 1
1405a1ca1: mov byte ptr [rbx + 0xac], 1
1405a1ca8: mov byte ptr [rbx + 0xad], 1
1405a1caf: mov byte ptr [rbx + 0xae], 0
1405a1cb6: mov byte ptr [rbx + 0xaf], 1
1405a1cbd: mov byte ptr [rbx + 0xb0], 0
1405a1cc4: mov byte ptr [rbx + 0xb1], 1
1405a1ccb: mov byte ptr [rbx + 0xb2], 1
1405a1cd2: call 0x140d64e00
1405a1cd7: mov rcx, rdi
1405a1cda: add rcx, 8
1405a1cde: mov qword ptr [rdi], 1
1405a1ce5: mov qword ptr [rdi + 8], rax
1405a1ce9: mov qword ptr [rdi + 0x10], rdx
1405a1ced: lea r8, [rax + 1]
1405a1cf1: mov qword ptr [rcx], r8
1405a1cf4: movaps xmmword ptr [rbx + 0xb10], xmm6
1405a1cfb: movaps xmmword ptr [rbx + 0xb00], xmm7
1405a1d02: mov qword ptr [rbx + 0xb20], rax
1405a1d09: mov qword ptr [rbx + 0xb28], rdx
1405a1d10: mov byte ptr [rbx + 0xaa], 1
1405a1d17: mov byte ptr [rbx + 0xab], 1
1405a1d1e: mov byte ptr [rbx + 0xac], 1
1405a1d25: mov byte ptr [rbx + 0xad], 1
1405a1d2c: mov byte ptr [rbx + 0xae], 0
1405a1d33: mov byte ptr [rbx + 0xaf], 1
1405a1d3a: mov byte ptr [rbx + 0xb0], 0
1405a1d41: mov byte ptr [rbx + 0xb1], 1
1405a1d48: mov byte ptr [rbx + 0xb2], 1
1405a1d4f: lea rdx, [rbx + 0xb00]
1405a1d56: mov rcx, rsi
1405a1d59: call 0x14012f490
1405a1d5e: mov rsi, rax
1405a1d61: call 0x140d4d740
1405a1d66: mov rdi, rax
1405a1d69: test byte ptr [rax], 1
1405a1d6c: je 0x1405a1d7f
1405a1d6e: mov rcx, rdi
1405a1d71: add rcx, 8
1405a1d75: mov rax, qword ptr [rdi + 8]
1405a1d79: mov rdx, qword ptr [rdi + 0x10]
1405a1d7d: jmp 0x1405a1dd9
1405a1d7f: mov byte ptr [rbx + 0xaa], 1
1405a1d86: mov byte ptr [rbx + 0xab], 1
1405a1d8d: mov byte ptr [rbx + 0xac], 1
1405a1d94: mov byte ptr [rbx + 0xad], 1
1405a1d9b: mov byte ptr [rbx + 0xae], 0
1405a1da2: mov byte ptr [rbx + 0xaf], 1
1405a1da9: mov byte ptr [rbx + 0xb0], 0
1405a1db0: mov byte ptr [rbx + 0xb1], 1
1405a1db7: mov byte ptr [rbx + 0xb2], 1
1405a1dbe: call 0x140d64e00
1405a1dc3: mov rcx, rdi
1405a1dc6: add rcx, 8
1405a1dca: mov qword ptr [rdi], 1
1405a1dd1: mov qword ptr [rdi + 8], rax
1405a1dd5: mov qword ptr [rdi + 0x10], rdx
1405a1dd9: lea r8, [rax + 1]
1405a1ddd: mov qword ptr [rcx], r8
1405a1de0: movaps xmmword ptr [rbx + 0xb10], xmm6
1405a1de7: movaps xmmword ptr [rbx + 0xb00], xmm7
1405a1dee: mov qword ptr [rbx + 0xb20], rax
1405a1df5: mov qword ptr [rbx + 0xb28], rdx
1405a1dfc: mov byte ptr [rbx + 0xaa], 1
1405a1e03: mov byte ptr [rbx + 0xab], 1
1405a1e0a: mov byte ptr [rbx + 0xac], 1
1405a1e11: mov byte ptr [rbx + 0xad], 1
1405a1e18: mov byte ptr [rbx + 0xae], 0
1405a1e1f: mov byte ptr [rbx + 0xaf], 1
1405a1e26: mov byte ptr [rbx + 0xb0], 0
1405a1e2d: mov byte ptr [rbx + 0xb1], 1
1405a1e34: mov byte ptr [rbx + 0xb2], 1
1405a1e3b: lea rdx, [rbx + 0xb00]
1405a1e42: mov rcx, rsi
1405a1e45: call 0x14012bdc0
1405a1e4a: mov rsi, rax
1405a1e4d: call 0x140d4d740
1405a1e52: mov rdi, rax
1405a1e55: test byte ptr [rax], 1
1405a1e58: je 0x1405a1e6b
1405a1e5a: mov rcx, rdi
1405a1e5d: add rcx, 8
1405a1e61: mov rax, qword ptr [rdi + 8]
1405a1e65: mov rdx, qword ptr [rdi + 0x10]
1405a1e69: jmp 0x1405a1ec5
1405a1e6b: mov byte ptr [rbx + 0xaa], 1
1405a1e72: mov byte ptr [rbx + 0xab], 1
1405a1e79: mov byte ptr [rbx + 0xac], 1
1405a1e80: mov byte ptr [rbx + 0xad], 1
1405a1e87: mov byte ptr [rbx + 0xae], 0
1405a1e8e: mov byte ptr [rbx + 0xaf], 1
1405a1e95: mov byte ptr [rbx + 0xb0], 0
1405a1e9c: mov byte ptr [rbx + 0xb1], 1
1405a1ea3: mov byte ptr [rbx + 0xb2], 1
1405a1eaa: call 0x140d64e00
1405a1eaf: mov rcx, rdi
1405a1eb2: add rcx, 8
1405a1eb6: mov qword ptr [rdi], 1
1405a1ebd: mov qword ptr [rdi + 8], rax
1405a1ec1: mov qword ptr [rdi + 0x10], rdx
1405a1ec5: lea r8, [rax + 1]
1405a1ec9: mov qword ptr [rcx], r8
1405a1ecc: movaps xmmword ptr [rbx + 0xb10], xmm6
1405a1ed3: movaps xmmword ptr [rbx + 0xb00], xmm7
1405a1eda: mov qword ptr [rbx + 0xb20], rax
1405a1ee1: mov qword ptr [rbx + 0xb28], rdx
1405a1ee8: mov byte ptr [rbx + 0xaa], 1
1405a1eef: mov byte ptr [rbx + 0xab], 1
1405a1ef6: mov byte ptr [rbx + 0xac], 1
1405a1efd: mov byte ptr [rbx + 0xad], 1
1405a1f04: mov byte ptr [rbx + 0xae], 0
1405a1f0b: mov byte ptr [rbx + 0xaf], 1
1405a1f12: mov byte ptr [rbx + 0xb0], 0
1405a1f19: mov byte ptr [rbx + 0xb1], 1
1405a1f20: mov byte ptr [rbx + 0xb2], 1
1405a1f27: lea rdx, [rbx + 0xb00]
1405a1f2e: mov rcx, rsi
1405a1f31: call 0x140125010
1405a1f36: mov rsi, rax
1405a1f39: call 0x140d4d740
1405a1f3e: mov rdi, rax
1405a1f41: test byte ptr [rax], 1
1405a1f44: je 0x1405a1f57
1405a1f46: mov rcx, rdi
1405a1f49: add rcx, 8
1405a1f4d: mov rax, qword ptr [rdi + 8]
1405a1f51: mov rdx, qword ptr [rdi + 0x10]
1405a1f55: jmp 0x1405a1fb1
1405a1f57: mov byte ptr [rbx + 0xaa], 1
1405a1f5e: mov byte ptr [rbx + 0xab], 1
1405a1f65: mov byte ptr [rbx + 0xac], 1
1405a1f6c: mov byte ptr [rbx + 0xad], 1
1405a1f73: mov byte ptr [rbx + 0xae], 0
1405a1f7a: mov byte ptr [rbx + 0xaf], 1
1405a1f81: mov byte ptr [rbx + 0xb0], 0
1405a1f88: mov byte ptr [rbx + 0xb1], 1
1405a1f8f: mov byte ptr [rbx + 0xb2], 1
1405a1f96: call 0x140d64e00
1405a1f9b: mov rcx, rdi
1405a1f9e: add rcx, 8
1405a1fa2: mov qword ptr [rdi], 1
1405a1fa9: mov qword ptr [rdi + 8], rax
1405a1fad: mov qword ptr [rdi + 0x10], rdx
1405a1fb1: lea r8, [rax + 1]
1405a1fb5: mov qword ptr [rcx], r8
1405a1fb8: movaps xmmword ptr [rbx + 0x450], xmm6
1405a1fbf: movaps xmmword ptr [rbx + 0x440], xmm7
1405a1fc6: mov qword ptr [rbx + 0x460], rax
1405a1fcd: mov qword ptr [rbx + 0x468], rdx
1405a1fd4: call 0x140d4d740
1405a1fd9: mov rdi, rax
1405a1fdc: test byte ptr [rax], 1
1405a1fdf: je 0x1405a1ff2
1405a1fe1: mov rcx, rdi
1405a1fe4: add rcx, 8
1405a1fe8: mov rax, qword ptr [rdi + 8]
1405a1fec: mov rdx, qword ptr [rdi + 0x10]
1405a1ff0: jmp 0x1405a200d
1405a1ff2: call 0x140d64e00
1405a1ff7: mov rcx, rdi
1405a1ffa: add rcx, 8
1405a1ffe: mov qword ptr [rdi], 1
1405a2005: mov qword ptr [rdi + 8], rax
1405a2009: mov qword ptr [rdi + 0x10], rdx
1405a200d: lea r8, [rax + 1]
1405a2011: mov qword ptr [rcx], r8
1405a2014: movaps xmm0, xmmword ptr [rbx + 0x440]
1405a201b: movaps xmm1, xmmword ptr [rbx + 0x450]
1405a2022: movaps xmm2, xmmword ptr [rbx + 0x460]
1405a2029: movaps xmmword ptr [rbx + 0xb20], xmm2
1405a2030: movaps xmmword ptr [rbx + 0xb10], xmm1
1405a2037: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a203e: movaps xmmword ptr [rbx + 0xb30], xmm7
1405a2045: movaps xmmword ptr [rbx + 0xb40], xmm6
1405a204c: mov qword ptr [rbx + 0xb50], rax
1405a2053: mov qword ptr [rbx + 0xb58], rdx
1405a205a: mov byte ptr [rbx + 0xaa], 1
1405a2061: mov byte ptr [rbx + 0xab], 1
1405a2068: mov byte ptr [rbx + 0xac], 1
1405a206f: mov byte ptr [rbx + 0xad], 1
1405a2076: mov byte ptr [rbx + 0xae], 0
1405a207d: mov byte ptr [rbx + 0xaf], 1
1405a2084: mov byte ptr [rbx + 0xb0], 0
1405a208b: mov byte ptr [rbx + 0xb1], 1
1405a2092: mov byte ptr [rbx + 0xb2], 1
1405a2099: lea rdx, [rbx + 0xb00]
1405a20a0: mov rcx, rsi
1405a20a3: call 0x14011fe50
1405a20a8: mov byte ptr [rbx + 0xaa], 1
1405a20af: mov byte ptr [rbx + 0xab], 1
1405a20b6: mov byte ptr [rbx + 0xac], 1
1405a20bd: mov byte ptr [rbx + 0xad], 1
1405a20c4: mov byte ptr [rbx + 0xae], 0
1405a20cb: mov byte ptr [rbx + 0xaf], 1
1405a20d2: mov byte ptr [rbx + 0xb0], 0
1405a20d9: mov byte ptr [rbx + 0xb1], 1
1405a20e0: mov byte ptr [rbx + 0xb2], 1
1405a20e7: mov rcx, rax
1405a20ea: call 0x140651870
1405a20ef: mov byte ptr [rbx + 0xaa], 1
1405a20f6: mov byte ptr [rbx + 0xab], 1
1405a20fd: mov byte ptr [rbx + 0xac], 1
1405a2104: mov byte ptr [rbx + 0xad], 1
1405a210b: mov byte ptr [rbx + 0xae], 0
1405a2112: mov byte ptr [rbx + 0xaf], 1
1405a2119: mov byte ptr [rbx + 0xb0], 0
1405a2120: mov byte ptr [rbx + 0xb1], 1
1405a2127: mov byte ptr [rbx + 0xb2], 1
1405a212e: mov rcx, rax
1405a2131: call 0x140630a20
1405a2136: mov byte ptr [rbx + 0xaa], 1
1405a213d: mov byte ptr [rbx + 0xab], 1
1405a2144: mov byte ptr [rbx + 0xac], 1
1405a214b: mov byte ptr [rbx + 0xad], 1
1405a2152: mov byte ptr [rbx + 0xae], 0
1405a2159: mov byte ptr [rbx + 0xaf], 1
1405a2160: mov byte ptr [rbx + 0xb0], 0
1405a2167: mov byte ptr [rbx + 0xb1], 1
1405a216e: mov byte ptr [rbx + 0xb2], 1
1405a2175: mov rcx, rax
1405a2178: call 0x140650d50
1405a217d: mov edx, dword ptr [rbx + 0x148]
1405a2183: mov r8d, dword ptr [rbx + 0x14c]
1405a218a: mov byte ptr [rbx + 0xaa], 1
1405a2191: mov byte ptr [rbx + 0xab], 1
1405a2198: mov byte ptr [rbx + 0xac], 1
1405a219f: mov byte ptr [rbx + 0xad], 1
1405a21a6: mov byte ptr [rbx + 0xae], 0
1405a21ad: mov byte ptr [rbx + 0xaf], 1
1405a21b4: mov byte ptr [rbx + 0xb0], 0
1405a21bb: mov byte ptr [rbx + 0xb1], 1
1405a21c2: mov byte ptr [rbx + 0xb2], 1
1405a21c9: mov rcx, rax
1405a21cc: call 0x140940d80
1405a21d1: mov rsi, rax
1405a21d4: mov byte ptr [rbx + 0xaa], 1
1405a21db: mov byte ptr [rbx + 0xab], 1
1405a21e2: mov byte ptr [rbx + 0xac], 1
1405a21e9: mov byte ptr [rbx + 0xad], 1
1405a21f0: mov byte ptr [rbx + 0xae], 0
1405a21f7: mov byte ptr [rbx + 0xaf], 1
1405a21fe: mov byte ptr [rbx + 0xb0], 0
1405a2205: mov byte ptr [rbx + 0xb1], 1
1405a220c: mov byte ptr [rbx + 0xb2], 1
1405a2213: lea rcx, [rbx + 0xb00]
1405a221a: mov rdx, qword ptr [rbp + 0x37b0]
1405a2221: call 0x140593510
1405a2226: mov byte ptr [rbx + 0xaa], 1
1405a222d: mov byte ptr [rbx + 0xab], 1
1405a2234: mov byte ptr [rbx + 0xac], 1
1405a223b: mov byte ptr [rbx + 0xad], 1
1405a2242: mov byte ptr [rbx + 0xae], 0
1405a2249: mov byte ptr [rbx + 0xaf], 1
1405a2250: mov byte ptr [rbx + 0xb0], 0
1405a2257: mov byte ptr [rbx + 0xb1], 1
1405a225e: mov byte ptr [rbx + 0xb2], 1
1405a2265: lea rdx, [rbx + 0xb00]
1405a226c: mov rcx, rsi
1405a226f: call 0x140119810
1405a2274: mov byte ptr [rbx + 0xaa], 1
1405a227b: mov byte ptr [rbx + 0xab], 1
1405a2282: mov byte ptr [rbx + 0xac], 1
1405a2289: mov byte ptr [rbx + 0xad], 1
1405a2290: mov byte ptr [rbx + 0xae], 0
1405a2297: mov byte ptr [rbx + 0xaf], 1
1405a229e: mov byte ptr [rbx + 0xb0], 0
1405a22a5: mov byte ptr [rbx + 0xb1], 1
1405a22ac: mov byte ptr [rbx + 0xb2], 1
1405a22b3: movabs rdx, 0xe3601bf355496a1b
1405a22bd: movabs r8, 0x8acc546ccef23be3
1405a22c7: mov rcx, qword ptr [rbx + 0xe8]
1405a22ce: call 0x1400dd6f0
1405a22d3: test rax, rax
1405a22d6: mov r8, qword ptr [rbx + 0xc8]
1405a22dd: je 0x1405a249d
1405a22e3: cmp qword ptr [r8 + 0x218], rdx
1405a22ea: jbe 0x1405a249d
1405a22f0: mov rax, qword ptr [r8 + 0x210]
1405a22f7: mov rcx, qword ptr [rax + rdx*8]
1405a22fb: test rcx, rcx
1405a22fe: je 0x1405a249d
1405a2304: mov rax, qword ptr [r8 + 0x1e0]
1405a230b: not rcx
1405a230e: lea rcx, [rcx + rcx*2]
1405a2312: shl rcx, 5
1405a2316: cmp qword ptr [rax + rcx + 0x20], 0
1405a231c: je 0x1405a249d
1405a2322: add rax, rcx
1405a2325: mov rcx, qword ptr [rax + 0x10]
1405a2329: mov byte ptr [rbx + 0xaa], 1
1405a2330: mov byte ptr [rbx + 0xab], 1
1405a2337: mov byte ptr [rbx + 0xac], 1
1405a233e: mov byte ptr [rbx + 0xad], 1
1405a2345: mov byte ptr [rbx + 0xae], 0
1405a234c: mov byte ptr [rbx + 0xaf], 1
1405a2353: mov byte ptr [rbx + 0xb0], 0
1405a235a: mov byte ptr [rbx + 0xb1], 1
1405a2361: mov byte ptr [rbx + 0xb2], 1
1405a2368: call 0x14067dd80
1405a236d: mov rsi, rax
1405a2370: call 0x142923570
1405a2375: mov rcx, qword ptr [rsi]
1405a2378: lock inc qword ptr [rcx]
1405a237c: jle 0x1405a7040
1405a2382: mov qword ptr [rbx + 0xb00], rcx
1405a2389: mov qword ptr [rbx + 0xb08], rax
1405a2390: mov rcx, rax
1405a2393: call 0x1429234f0
1405a2398: mov esi, eax
1405a239a: mov rax, qword ptr [rbx + 0xb00]
1405a23a1: lock dec qword ptr [rax]
1405a23a5: jne 0x1405a23f2
1405a23a7: mov byte ptr [rbx + 0xaa], 1
1405a23ae: mov byte ptr [rbx + 0xab], 1
1405a23b5: mov byte ptr [rbx + 0xac], 1
1405a23bc: mov byte ptr [rbx + 0xad], 1
1405a23c3: mov byte ptr [rbx + 0xae], 0
1405a23ca: mov byte ptr [rbx + 0xaf], 1
1405a23d1: mov byte ptr [rbx + 0xb0], 0
1405a23d8: mov byte ptr [rbx + 0xb1], 1
1405a23df: mov byte ptr [rbx + 0xb2], 1
1405a23e6: lea rcx, [rbx + 0xb00]
1405a23ed: call 0x140b85350
1405a23f2: test sil, sil
1405a23f5: je 0x1405a249d
1405a23fb: mov byte ptr [rbx + 0xaa], 1
1405a2402: mov byte ptr [rbx + 0xab], 1
1405a2409: mov byte ptr [rbx + 0xac], 1
1405a2410: mov byte ptr [rbx + 0xad], 1
1405a2417: mov byte ptr [rbx + 0xae], 0
1405a241e: mov byte ptr [rbx + 0xaf], 1
1405a2425: mov byte ptr [rbx + 0xb0], 0
1405a242c: mov byte ptr [rbx + 0xb1], 1
1405a2433: mov byte ptr [rbx + 0xb2], 1
1405a243a: lea rcx, [rbx + 0xb00]
1405a2441: call 0x140958eb0
1405a2446: mov byte ptr [rbx + 0xaa], 1
1405a244d: mov byte ptr [rbx + 0xab], 1
1405a2454: mov byte ptr [rbx + 0xac], 1
1405a245b: mov byte ptr [rbx + 0xad], 1
1405a2462: mov byte ptr [rbx + 0xae], 0
1405a2469: mov byte ptr [rbx + 0xaf], 1
1405a2470: mov byte ptr [rbx + 0xb0], 0
1405a2477: mov byte ptr [rbx + 0xb1], 1
1405a247e: mov byte ptr [rbx + 0xb2], 1
1405a2485: lea rdx, [rbx + 0xb00]
1405a248c: mov rcx, qword ptr [rbx + 0xc8]
1405a2493: call 0x14012b880
1405a2498: jmp 0x1405a253a
1405a249d: mov byte ptr [rbx + 0xaa], 1
1405a24a4: mov byte ptr [rbx + 0xab], 1
1405a24ab: mov byte ptr [rbx + 0xac], 1
1405a24b2: mov byte ptr [rbx + 0xad], 1
1405a24b9: mov byte ptr [rbx + 0xae], 0
1405a24c0: mov byte ptr [rbx + 0xaf], 1
1405a24c7: mov byte ptr [rbx + 0xb0], 0
1405a24ce: mov byte ptr [rbx + 0xb1], 1
1405a24d5: mov byte ptr [rbx + 0xb2], 1
1405a24dc: lea rcx, [rbx + 0xb00]
1405a24e3: call 0x140958ef0
1405a24e8: mov byte ptr [rbx + 0xaa], 1
1405a24ef: mov byte ptr [rbx + 0xab], 1
1405a24f6: mov byte ptr [rbx + 0xac], 1
1405a24fd: mov byte ptr [rbx + 0xad], 1
1405a2504: mov byte ptr [rbx + 0xae], 0
1405a250b: mov byte ptr [rbx + 0xaf], 1
1405a2512: mov byte ptr [rbx + 0xb0], 0
1405a2519: mov byte ptr [rbx + 0xb1], 1
1405a2520: mov byte ptr [rbx + 0xb2], 1
1405a2527: lea rdx, [rbx + 0xb00]
1405a252e: mov rcx, qword ptr [rbx + 0xc8]
1405a2535: call 0x14012b880
1405a253a: mov byte ptr [rbx + 0xaa], 1
1405a2541: mov byte ptr [rbx + 0xab], 1
1405a2548: mov byte ptr [rbx + 0xac], 1
1405a254f: mov byte ptr [rbx + 0xad], 1
1405a2556: mov byte ptr [rbx + 0xae], 0
1405a255d: mov byte ptr [rbx + 0xaf], 1
1405a2564: mov byte ptr [rbx + 0xb0], 0
1405a256b: mov byte ptr [rbx + 0xb1], 1
1405a2572: mov byte ptr [rbx + 0xb2], 1
1405a2579: lea rcx, [rbx + 0x148]
1405a2580: call 0x1400885d0
1405a2585: mov rsi, rax
1405a2588: mov byte ptr [rbx + 0xaa], 1
1405a258f: mov byte ptr [rbx + 0xab], 1
1405a2596: mov byte ptr [rbx + 0xac], 1
1405a259d: mov byte ptr [rbx + 0xad], 1
1405a25a4: mov byte ptr [rbx + 0xae], 0
1405a25ab: mov byte ptr [rbx + 0xaf], 1
1405a25b2: mov byte ptr [rbx + 0xb0], 0
1405a25b9: mov byte ptr [rbx + 0xb1], 1
1405a25c0: mov byte ptr [rbx + 0xb2], 1
1405a25c7: lea rcx, [rbx + 0x148]
1405a25ce: call 0x1400885d0
1405a25d3: mov rdi, rax
1405a25d6: mov byte ptr [rbx + 0xaa], 1
1405a25dd: mov byte ptr [rbx + 0xab], 1
1405a25e4: mov byte ptr [rbx + 0xac], 1
1405a25eb: mov byte ptr [rbx + 0xad], 1
1405a25f2: mov byte ptr [rbx + 0xae], 0
1405a25f9: mov byte ptr [rbx + 0xaf], 1
1405a2600: mov byte ptr [rbx + 0xb0], 0
1405a2607: mov byte ptr [rbx + 0xb1], 1
1405a260e: mov byte ptr [rbx + 0xb2], 1
1405a2615: lea rcx, [rbx + 0x148]
1405a261c: call 0x1400885d0
1405a2621: mov r15, rax
1405a2624: mov rax, qword ptr [rbp + 0x37b0]
1405a262b: cmp byte ptr [rax + 0x88], 0
1405a2632: movss xmm0, dword ptr [rip + 0x23832ca]
1405a263a: jne 0x1405a264b
1405a263c: movaps xmm12, xmm0
1405a2640: cmp byte ptr [rax + 0xab], 1
1405a2647: je 0x1405a265d
1405a2649: jmp 0x1405a26a8
1405a264b: movss xmm12, dword ptr [rax + 0x8c]
1405a2654: cmp byte ptr [rax + 0xab], 1
1405a265b: jne 0x1405a26a8
1405a265d: lea rcx, [rax + 0xc3]
1405a2664: mov byte ptr [rbx + 0xaa], 1
1405a266b: mov byte ptr [rbx + 0xab], 1
1405a2672: mov byte ptr [rbx + 0xac], 1
1405a2679: mov byte ptr [rbx + 0xad], 1
1405a2680: mov byte ptr [rbx + 0xae], 0
1405a2687: mov byte ptr [rbx + 0xaf], 1
1405a268e: mov byte ptr [rbx + 0xb0], 0
1405a2695: mov byte ptr [rbx + 0xb1], 1
1405a269c: mov byte ptr [rbx + 0xb2], 1
1405a26a3: call 0x140968bb0
1405a26a8: mov qword ptr [rbx + 0xb00], rsi
1405a26af: mov qword ptr [rbx + 0xb08], rdi
1405a26b6: mov qword ptr [rbx + 0xb10], r15
1405a26bd: mov byte ptr [rbx + 0xb20], 0
1405a26c4: movss dword ptr [rbx + 0xb18], xmm12
1405a26cd: movss dword ptr [rbx + 0xb1c], xmm0
1405a26d5: mov byte ptr [rbx + 0xaa], 1
1405a26dc: mov byte ptr [rbx + 0xab], 1
1405a26e3: mov byte ptr [rbx + 0xac], 1
1405a26ea: mov byte ptr [rbx + 0xad], 1
1405a26f1: mov byte ptr [rbx + 0xae], 0
1405a26f8: mov byte ptr [rbx + 0xaf], 1
1405a26ff: mov byte ptr [rbx + 0xb0], 0
1405a2706: mov byte ptr [rbx + 0xb1], 1
1405a270d: mov byte ptr [rbx + 0xb2], 1
1405a2714: lea rdx, [rbx + 0xb00]
1405a271b: mov rcx, qword ptr [rbx + 0xc8]
1405a2722: call 0x14012e420
1405a2727: mov rax, qword ptr [rbp + 0x37c0]
1405a272e: movzx eax, byte ptr [rax + 0x141]
1405a2735: xor al, 1
1405a2737: mov byte ptr [rbx + 0xb28], al
1405a273d: mov qword ptr [rbx + 0xb00], 0
1405a2748: mov byte ptr [rbx + 0xaa], 1
1405a274f: mov byte ptr [rbx + 0xab], 1
1405a2756: mov byte ptr [rbx + 0xac], 1
1405a275d: mov byte ptr [rbx + 0xad], 1
1405a2764: mov byte ptr [rbx + 0xae], 0
1405a276b: mov byte ptr [rbx + 0xaf], 1
1405a2772: mov byte ptr [rbx + 0xb0], 0
1405a2779: mov byte ptr [rbx + 0xb1], 1
1405a2780: mov byte ptr [rbx + 0xb2], 1
1405a2787: lea rdx, [rbx + 0xb00]
1405a278e: mov rcx, qword ptr [rbx + 0xc8]
1405a2795: call 0x14013d5f0
1405a279a: mov rax, qword ptr [rbx + 0x2c0]
1405a27a1: mov rdx, qword ptr [rbx + 0x2c8]
1405a27a8: test rax, rax
1405a27ab: je 0x1405a280c
1405a27ad: cmp eax, 1
1405a27b0: jne 0x1405a285f
1405a27b6: mov byte ptr [rbx + 0xaa], 1
1405a27bd: mov byte ptr [rbx + 0xab], 1
1405a27c4: mov byte ptr [rbx + 0xac], 1
1405a27cb: mov byte ptr [rbx + 0xad], 1
1405a27d2: mov byte ptr [rbx + 0xae], 0
1405a27d9: mov byte ptr [rbx + 0xaf], 1
1405a27e0: mov byte ptr [rbx + 0xb0], 0
1405a27e7: mov byte ptr [rbx + 0xb1], 1
1405a27ee: mov byte ptr [rbx + 0xb2], 1
1405a27f5: lea rcx, [rbx + 0xb00]
1405a27fc: mov r9d, 0x3b9aca00
1405a2802: call 0x140264130
1405a2807: jmp 0x1405a28b0
1405a280c: mov byte ptr [rbx + 0xaa], 1
1405a2813: mov byte ptr [rbx + 0xab], 1
1405a281a: mov byte ptr [rbx + 0xac], 1
1405a2821: mov byte ptr [rbx + 0xad], 1
1405a2828: mov byte ptr [rbx + 0xae], 0
1405a282f: mov byte ptr [rbx + 0xaf], 1
1405a2836: mov byte ptr [rbx + 0xb0], 0
1405a283d: mov byte ptr [rbx + 0xb1], 1
1405a2844: mov byte ptr [rbx + 0xb2], 1
1405a284b: lea rcx, [rbx + 0xb00]
1405a2852: mov r9d, 0x3b9aca00
1405a2858: call 0x1404ee940
1405a285d: jmp 0x1405a28b0
1405a285f: mov byte ptr [rbx + 0xaa], 1
1405a2866: mov byte ptr [rbx + 0xab], 1
1405a286d: mov byte ptr [rbx + 0xac], 1
1405a2874: mov byte ptr [rbx + 0xad], 1
1405a287b: mov byte ptr [rbx + 0xae], 0
1405a2882: mov byte ptr [rbx + 0xaf], 1
1405a2889: mov byte ptr [rbx + 0xb0], 0
1405a2890: mov byte ptr [rbx + 0xb1], 1
1405a2897: mov byte ptr [rbx + 0xb2], 1
1405a289e: lea rcx, [rbx + 0xb00]
1405a28a5: mov r9d, 0x3b9aca00
1405a28ab: call 0x140513570
1405a28b0: mov rax, qword ptr [rbx + 0xb00]
1405a28b7: cmp rax, 2
1405a28bb: je 0x1405a6d15
1405a28c1: movups xmm0, xmmword ptr [rbx + 0xb08]
1405a28c8: movups xmm1, xmmword ptr [rbx + 0xb18]
1405a28cf: movups xmmword ptr [rbx + 0x318], xmm1
1405a28d6: movups xmmword ptr [rbx + 0x308], xmm0
1405a28dd: mov qword ptr [rbx + 0x300], rax
1405a28e4: mov rdx, qword ptr [rbx + 0x320]
1405a28eb: test rdx, rdx
1405a28ee: mov rcx, qword ptr [rbx + 0xc8]
1405a28f5: je 0x1405a2901
1405a28f7: lock inc qword ptr [rdx]
1405a28fb: jle 0x1405a7040
1405a2901: mov byte ptr [rbx + 0xb4], 1
1405a2908: mov byte ptr [rbx + 0xb5], 1
1405a290f: mov byte ptr [rbx + 0xb6], 1
1405a2916: mov byte ptr [rbx + 0xb7], 1
1405a291d: mov byte ptr [rbx + 0xb8], 1
1405a2924: mov byte ptr [rbx + 0xb9], 1
1405a292b: mov byte ptr [rbx + 0xba], 1
1405a2932: mov byte ptr [rbx + 0xbb], 1
1405a2939: call 0x140114e10
1405a293e: movups xmm0, xmmword ptr [rbx + 0x300]
1405a2945: movups xmm1, xmmword ptr [rbx + 0x310]
1405a294c: movaps xmmword ptr [rbx + 0xb10], xmm1
1405a2953: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a295a: mov byte ptr [rbx + 0xb4], 1
1405a2961: mov byte ptr [rbx + 0xb5], 1
1405a2968: mov byte ptr [rbx + 0xb6], 1
1405a296f: mov byte ptr [rbx + 0xb7], 0
1405a2976: mov byte ptr [rbx + 0xb8], 1
1405a297d: mov byte ptr [rbx + 0xb9], 1
1405a2984: mov byte ptr [rbx + 0xba], 1
1405a298b: mov byte ptr [rbx + 0xbb], 1
1405a2992: lea rdx, [rbx + 0xb00]
1405a2999: mov rcx, qword ptr [rbx + 0xc8]
1405a29a0: call 0x14012b2a0
1405a29a5: mov byte ptr [rbx + 0xb4], 1
1405a29ac: mov byte ptr [rbx + 0xb5], 1
1405a29b3: mov byte ptr [rbx + 0xb6], 1
1405a29ba: mov byte ptr [rbx + 0xb7], 0
1405a29c1: mov byte ptr [rbx + 0xb8], 1
1405a29c8: mov byte ptr [rbx + 0xb9], 1
1405a29cf: mov byte ptr [rbx + 0xba], 1
1405a29d6: mov byte ptr [rbx + 0xbb], 1
1405a29dd: mov rcx, qword ptr [rbx + 0xc8]
1405a29e4: call 0x140327eb0
1405a29e9: mov byte ptr [rbx + 0xb4], 1
1405a29f0: mov byte ptr [rbx + 0xb5], 1
1405a29f7: mov byte ptr [rbx + 0xb6], 1
1405a29fe: mov byte ptr [rbx + 0xb7], 0
1405a2a05: mov byte ptr [rbx + 0xb8], 1
1405a2a0c: mov byte ptr [rbx + 0xb9], 1
1405a2a13: mov byte ptr [rbx + 0xba], 1
1405a2a1a: mov byte ptr [rbx + 0xbb], 1
1405a2a21: lea rdx, [rip + 0x245cee0]
1405a2a28: mov rcx, qword ptr [rbx + 0xc8]
1405a2a2f: call 0x140113340
1405a2a34: mov byte ptr [rbx + 0xb4], 1
1405a2a3b: mov byte ptr [rbx + 0xb5], 1
1405a2a42: mov byte ptr [rbx + 0xb6], 1
1405a2a49: mov byte ptr [rbx + 0xb7], 0
1405a2a50: mov byte ptr [rbx + 0xb8], 1
1405a2a57: mov byte ptr [rbx + 0xb9], 1
1405a2a5e: mov byte ptr [rbx + 0xba], 1
1405a2a65: mov byte ptr [rbx + 0xbb], 1
1405a2a6c: lea rdx, [rip + 0x245cead]
1405a2a73: mov rcx, qword ptr [rbx + 0xc8]
1405a2a7a: call 0x1401130e0
1405a2a7f: mov byte ptr [rbx + 0xb4], 1
1405a2a86: mov byte ptr [rbx + 0xb5], 1
1405a2a8d: mov byte ptr [rbx + 0xb6], 1
1405a2a94: mov byte ptr [rbx + 0xb7], 0
1405a2a9b: mov byte ptr [rbx + 0xb8], 1
1405a2aa2: mov byte ptr [rbx + 0xb9], 1
1405a2aa9: mov byte ptr [rbx + 0xba], 1
1405a2ab0: mov byte ptr [rbx + 0xbb], 1
1405a2ab7: lea rdx, [rip + 0x245ce7a]
1405a2abe: mov rcx, qword ptr [rbx + 0xc8]
1405a2ac5: call 0x140113210
1405a2aca: mov byte ptr [rbx + 0xb4], 1
1405a2ad1: mov byte ptr [rbx + 0xb5], 1
1405a2ad8: mov byte ptr [rbx + 0xb6], 1
1405a2adf: mov byte ptr [rbx + 0xb7], 0
1405a2ae6: mov byte ptr [rbx + 0xb8], 1
1405a2aed: mov byte ptr [rbx + 0xb9], 1
1405a2af4: mov byte ptr [rbx + 0xba], 1
1405a2afb: mov byte ptr [rbx + 0xbb], 1
1405a2b02: lea rdx, [rip + 0x245ce47]
1405a2b09: mov rcx, qword ptr [rbx + 0xc8]
1405a2b10: call 0x140112fb0
1405a2b15: mov byte ptr [rbx + 0xb4], 1
1405a2b1c: mov byte ptr [rbx + 0xb5], 1
1405a2b23: mov byte ptr [rbx + 0xb6], 1
1405a2b2a: mov byte ptr [rbx + 0xb7], 0
1405a2b31: mov byte ptr [rbx + 0xb8], 1
1405a2b38: mov byte ptr [rbx + 0xb9], 1
1405a2b3f: mov byte ptr [rbx + 0xba], 1
1405a2b46: mov byte ptr [rbx + 0xbb], 1
1405a2b4d: mov rcx, r14
1405a2b50: call 0x14090df40
1405a2b55: mov sil, al
1405a2b58: mov byte ptr [rbx + 0xb4], 1
1405a2b5f: mov byte ptr [rbx + 0xb5], 1
1405a2b66: mov byte ptr [rbx + 0xb6], 1
1405a2b6d: mov byte ptr [rbx + 0xb7], 0
1405a2b74: mov byte ptr [rbx + 0xb8], 1
1405a2b7b: mov byte ptr [rbx + 0xb9], 1
1405a2b82: mov byte ptr [rbx + 0xba], 1
1405a2b89: mov byte ptr [rbx + 0xbb], 1
1405a2b90: mov rcx, qword ptr [rbx + 0xc8]
1405a2b97: call 0x140113ff0
1405a2b9c: cmp al, 1
1405a2b9e: ja 0x1405a6d84
1405a2ba4: mov byte ptr [rbx + 0xb4], 1
1405a2bab: mov byte ptr [rbx + 0xb5], 1
1405a2bb2: mov byte ptr [rbx + 0xb6], 1
1405a2bb9: mov byte ptr [rbx + 0xb7], 0
1405a2bc0: mov byte ptr [rbx + 0xb8], 1
1405a2bc7: mov byte ptr [rbx + 0xb9], 1
1405a2bce: mov byte ptr [rbx + 0xba], 1
1405a2bd5: mov byte ptr [rbx + 0xbb], 1
1405a2bdc: lea r8, [rip + 0x245cd85]
1405a2be3: mov ecx, esi
1405a2be5: mov rdx, qword ptr [rbx + 0xc8]
1405a2bec: call 0x14032afd0
1405a2bf1: mov rax, qword ptr [rbp + 0x37b0]
1405a2bf8: movzx eax, byte ptr [rax + 0xe4]
1405a2bff: mov byte ptr [rbx + 0x107], al
1405a2c05: xor eax, eax
1405a2c07: mov rcx, qword ptr [rbp + 0x37c0]
1405a2c0e: cmp rax, qword ptr [rcx + 0xc8]
1405a2c15: jo 0x1405a30e7
1405a2c1b: add rcx, 0xc8
1405a2c22: mov qword ptr [rbx + 0x338], rcx
1405a2c29: mov byte ptr [rbx + 0xb4], 1
1405a2c30: mov byte ptr [rbx + 0xb5], 1
1405a2c37: mov byte ptr [rbx + 0xb6], 1
1405a2c3e: mov byte ptr [rbx + 0xb7], 0
1405a2c45: mov byte ptr [rbx + 0xb8], 1
1405a2c4c: mov byte ptr [rbx + 0xb9], 1
1405a2c53: mov byte ptr [rbx + 0xba], 1
1405a2c5a: mov byte ptr [rbx + 0xbb], 1
1405a2c61: call 0x140d76ed0
1405a2c66: mov r8, rdx
1405a2c69: mov byte ptr [rbx + 0xb4], 1
1405a2c70: mov byte ptr [rbx + 0xb5], 1
1405a2c77: mov byte ptr [rbx + 0xb6], 1
1405a2c7e: mov byte ptr [rbx + 0xb7], 0
1405a2c85: mov byte ptr [rbx + 0xb8], 1
1405a2c8c: mov byte ptr [rbx + 0xb9], 1
1405a2c93: mov byte ptr [rbx + 0xba], 1
1405a2c9a: mov byte ptr [rbx + 0xbb], 1
1405a2ca1: lea rcx, [rbx + 0xb00]
1405a2ca8: mov rdx, rax
1405a2cab: call 0x140d59140
1405a2cb0: mov byte ptr [rbx + 0xb4], 1
1405a2cb7: mov byte ptr [rbx + 0xb5], 1
1405a2cbe: mov byte ptr [rbx + 0xb6], 1
1405a2cc5: mov byte ptr [rbx + 0xb7], 0
1405a2ccc: mov byte ptr [rbx + 0xb8], 1
1405a2cd3: mov byte ptr [rbx + 0xb9], 1
1405a2cda: mov byte ptr [rbx + 0xba], 1
1405a2ce1: mov byte ptr [rbx + 0xbb], 1
1405a2ce8: lea rcx, [rbx + 0x2590]
1405a2cef: lea rdx, [rbx + 0xb00]
1405a2cf6: lea r8, [rbx + 0x338]
1405a2cfd: call 0x140311ea0
1405a2d02: mov rax, qword ptr [rbx + 0x2590]
1405a2d09: mov r8, qword ptr [rbx + 0x2598]
1405a2d10: mov qword ptr [rbx + 0x130], rax
1405a2d17: neg rax
1405a2d1a: jo 0x1405a6e72
1405a2d20: mov r9, qword ptr [rbx + 0x25a0]
1405a2d27: mov qword ptr [rbx + 0xb10], 0
1405a2d32: mov qword ptr [rbx + 0xb00], 1
1405a2d3d: mov qword ptr [rbx + 0xb08], 0x80
1405a2d48: lea rcx, [rbx + 0x440]
1405a2d4f: lea rdx, [rbx + 0xb00]
1405a2d56: mov qword ptr [rbx + 0x128], r8
1405a2d5d: call 0x1405233f0
1405a2d62: movzx eax, byte ptr [rbx + 0x440]
1405a2d69: cmp al, 0x32
1405a2d6b: jne 0x1405a3153
1405a2d71: mov r13, qword ptr [rbx + 0x448]
1405a2d78: mov rsi, qword ptr [rbx + 0x450]
1405a2d7f: movups xmm0, xmmword ptr [rbx + 0x458]
1405a2d86: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a2d8d: mov rdx, qword ptr [rbx + 0x130]
1405a2d94: test rdx, rdx
1405a2d97: je 0x1405a2dab
1405a2d99: mov r8d, 1
1405a2d9f: mov rcx, qword ptr [rbx + 0x128]
1405a2da6: call 0x140613c20
1405a2dab: mov rax, r13
1405a2dae: neg rax
1405a2db1: jo 0x1405a6e6f
1405a2db7: mov qword ptr [rbx + 0x2230], r13
1405a2dbe: mov qword ptr [rbx + 0x2238], rsi
1405a2dc5: movaps xmm0, xmmword ptr [rbx + 0xb00]
1405a2dcc: movups xmmword ptr [rbx + 0x2240], xmm0
1405a2dd3: lea r15, [rbx + 0x2598]
1405a2dda: xor r12d, r12d
1405a2ddd: movabs r13, 0x8000000000000014
1405a2de7: movabs r14, 0x8000000000000000
1405a2df1: xorps xmm12, xmm12
1405a2df5: lea rsi, [rbx + 0xb00]
1405a2dfc: lea rdi, [rbx + 0x2590]
1405a2e03: xorps xmm13, xmm13
1405a2e07: nop word ptr [rax + rax]
1405a2e10: mov rax, qword ptr [rbx + 0x2248]
1405a2e17: test rax, rax
1405a2e1a: je 0x1405a309c
1405a2e20: mov r9, qword ptr [rbx + 0x2240]
1405a2e27: lea r8, [r9 + 1]
1405a2e2b: mov rdx, qword ptr [rbx + 0x2230]
1405a2e32: mov rcx, qword ptr [rbx + 0x2238]
1405a2e39: cmp r8, rdx
1405a2e3c: mov r8, rdx
1405a2e3f: cmovb r8, r12
1405a2e43: neg r8
1405a2e46: add r8, r9
1405a2e49: inc r8
1405a2e4c: mov qword ptr [rbx + 0x2240], r8
1405a2e53: lea r10, [rax - 1]
1405a2e57: mov qword ptr [rbx + 0x2248], r10
1405a2e5e: imul r10, r9, 0x38
1405a2e62: mov r9, qword ptr [rcx + r10]
1405a2e66: movups xmm0, xmmword ptr [rcx + r10 + 8]
1405a2e6c: movaps xmmword ptr [rbx + 0x1ff0], xmm0
1405a2e73: movups xmm0, xmmword ptr [rcx + r10 + 0x18]
1405a2e79: movups xmm1, xmmword ptr [rcx + r10 + 0x28]
1405a2e7f: movaps xmmword ptr [rbx + 0x2bf0], xmm0
1405a2e86: movaps xmmword ptr [rbx + 0x2c00], xmm1
1405a2e8d: cmp r9, r13
1405a2e90: je 0x1405a309c
1405a2e96: mov qword ptr [rbx + 0x2590], r9
1405a2e9d: movaps xmm0, xmmword ptr [rbx + 0x1ff0]
1405a2ea4: movups xmmword ptr [r15], xmm0
1405a2ea8: movaps xmm0, xmmword ptr [rbx + 0x2bf0]
1405a2eaf: movaps xmm1, xmmword ptr [rbx + 0x2c00]
1405a2eb6: movups xmmword ptr [r15 + 0x20], xmm1
1405a2ebb: movups xmmword ptr [r15 + 0x10], xmm0
1405a2ec0: mov r10, r9
1405a2ec3: xor r10, r14
1405a2ec6: cmp r10, 6
1405a2eca: je 0x1405a2f50
1405a2ed0: cmp r10, 7
1405a2ed4: jne 0x1405a3976
1405a2eda: lea rax, [rbx + 0xb10]
1405a2ee1: movups xmmword ptr [rax], xmm13
1405a2ee5: lea rax, [rbx + 0xb30]
1405a2eec: movups xmmword ptr [rax], xmm13
1405a2ef0: mov qword ptr [rbx + 0xb00], 0
1405a2efb: mov qword ptr [rbx + 0xb08], 8
1405a2f06: mov qword ptr [rbx + 0xb20], 0
1405a2f11: mov qword ptr [rbx + 0xb28], 8
1405a2f1c: movabs rax, 0x8000000000000003
1405a2f26: mov qword ptr [rbx + 0xb40], rax
1405a2f2d: mov byte ptr [rbx + 0xd9], 1
1405a2f34: mov byte ptr [rbx + 0xda], 1
1405a2f3b: mov rcx, qword ptr [rbx + 0xc8]
1405a2f42: mov rdx, rsi
1405a2f45: call 0x140137910
1405a2f4a: jmp 0x1405a3002
1405a2f4f: nop 
1405a2f50: movss xmm0, dword ptr [rbx + 0x2598]
1405a2f58: ucomiss xmm12, xmm0
1405a2f5c: ja 0x1405a65b1
1405a2f62: movd r9d, xmm0
1405a2f67: mov r8d, r9d
1405a2f6a: shr r8d, 0x17
1405a2f6e: movzx ecx, r8b
1405a2f72: xor eax, eax
1405a2f74: cmp cx, 0x60
1405a2f78: jb 0x1405a2fcb
1405a2f7a: mov edx, r9d
1405a2f7d: and edx, 0x7fffff
1405a2f83: or edx, 0x800000
1405a2f89: cmp cx, 0x7f
1405a2f8d: jae 0x1405a3023
1405a2f93: mov eax, edx
1405a2f95: lea ecx, [r8 + 0x2a]
1405a2f99: shl rax, cl
1405a2f9c: mov ecx, 0x3b9aca00
1405a2fa1: mul rcx
1405a2fa4: mov rcx, rax
1405a2fa7: neg rcx
1405a2faa: setno cl
1405a2fad: shr rax, 0x3f
1405a2fb1: mov r8d, edx
1405a2fb4: or r8b, cl
1405a2fb7: and r8b, al
1405a2fba: movzx eax, r8b
1405a2fbe: add eax, edx
1405a2fc0: cmp eax, 0x3b9aca00
1405a2fc5: je 0x1405a65b1
1405a2fcb: xor edx, edx
1405a2fcd: mov qword ptr [rbx + 0xb08], rdx
1405a2fd4: mov dword ptr [rbx + 0xb10], eax
1405a2fda: mov qword ptr [rbx + 0xb00], 2
1405a2fe5: mov byte ptr [rbx + 0xd9], 1
1405a2fec: mov byte ptr [rbx + 0xda], 1
1405a2ff3: mov rcx, qword ptr [rbx + 0xc8]
1405a2ffa: mov rdx, rsi
1405a2ffd: call 0x140135170
1405a3002: cmp r12, qword ptr [rbx + 0x2590]
1405a3009: jo 0x1405a2e10
1405a300f: mov byte ptr [rbx + 0xf4], 1
1405a3016: mov rcx, rdi
1405a3019: call 0x140592cc0
1405a301e: jmp 0x1405a2e10
1405a3023: movzx ecx, cx
1405a3026: cmp ecx, 0x96
1405a302c: jae 0x1405a3082
1405a302e: lea eax, [r8 + 1]
1405a3032: mov cl, 0x16
1405a3034: sub cl, r8b
1405a3037: shr edx, cl
1405a3039: mov ecx, eax
1405a303b: shl r9d, cl
1405a303e: and r9d, 0x7fffff
1405a3045: imul rax, r9, 0x3b9aca00
1405a304c: mov rcx, rax
1405a304f: shr rcx, 0x17
1405a3053: mov r8d, eax
1405a3056: and r8d, 0x7ffe00
1405a305d: cmp r8d, 0x400000
1405a3064: setne r8b
1405a3068: shr eax, 0x16
1405a306b: and eax, 1
1405a306e: mov r9d, ecx
1405a3071: or r9b, r8b
1405a3074: and r9b, al
1405a3077: movzx eax, r9b
1405a307b: add eax, ecx
1405a307d: jmp 0x1405a2fcd
1405a3082: cmp ecx, 0xbf
1405a3088: jae 0x1405a6ffd
1405a308e: mov edx, edx
1405a3090: lea ecx, [r8 + 0x2a]
1405a3094: shl rdx, cl
1405a3097: jmp 0x1405a2fcd
1405a309c: mov byte ptr [rbx + 0xb4], 1
1405a30a3: mov byte ptr [rbx + 0xb5], 1
1405a30aa: mov byte ptr [rbx + 0xb6], 1
1405a30b1: mov byte ptr [rbx + 0xb7], 0
1405a30b8: mov byte ptr [rbx + 0xb8], 1
1405a30bf: mov byte ptr [rbx + 0xb9], 1
1405a30c6: mov byte ptr [rbx + 0xba], 1
1405a30cd: mov byte ptr [rbx + 0xbb], 1
1405a30d4: lea rcx, [rbx + 0x2230]
1405a30db: call 0x1405d2cf0
1405a30e0: mov rcx, qword ptr [rbp + 0x37c0]
1405a30e7: xor eax, eax
1405a30e9: cmp rax, qword ptr [rcx + 0xa8]
1405a30f0: jno 0x1405a3140
1405a30f2: mov byte ptr [rbx + 0xb4], 1
1405a30f9: mov byte ptr [rbx + 0xb5], 1
1405a3100: mov byte ptr [rbx + 0xb6], 1
1405a3107: mov byte ptr [rbx + 0xb7], 0
1405a310e: mov byte ptr [rbx + 0xb8], 1
1405a3115: mov byte ptr [rbx + 0xb9], 1
1405a311c: mov byte ptr [rbx + 0xba], 1
1405a3123: mov byte ptr [rbx + 0xbb], 1
1405a312a: mov dil, 1
1405a312d: mov rcx, qword ptr [rbx + 0xc8]
1405a3134: xor edx, edx
1405a3136: call 0x1401209b0
1405a313b: jmp 0x1405a3e37
1405a3140: cmp byte ptr [rcx + 0x20], 0
1405a3144: je 0x1405a31e2
1405a314a: mov rdi, qword ptr [rcx + 0x28]
1405a314e: jmp 0x1405a31e4
1405a3153: mov ecx, dword ptr [rbx + 0x441]
1405a3159: mov edx, dword ptr [rbx + 0x444]
1405a315f: mov dword ptr [rbx + 0xb04], edx
1405a3165: mov dword ptr [rbx + 0xb01], ecx
1405a316b: movups xmm0, xmmword ptr [rbx + 0x448]
1405a3172: movups xmm1, xmmword ptr [rbx + 0x458]
1405a3179: movaps xmmword ptr [rbx + 0x2bf0], xmm1
1405a3180: movups xmm2, xmmword ptr [rbx + 0x468]
1405a3187: movups xmmword ptr [rbx + 0xb28], xmm2
1405a318e: movups xmm2, xmmword ptr [rbx + 0x478]
1405a3195: movups xmmword ptr [rbx + 0xb38], xmm2
1405a319c: movups xmm2, xmmword ptr [rbx + 0x488]
1405a31a3: movups xmmword ptr [rbx + 0xb48], xmm2
1405a31aa: movups xmmword ptr [rbx + 0xb08], xmm0
1405a31b1: movups xmmword ptr [rbx + 0xb18], xmm1
1405a31b8: mov byte ptr [rbx + 0xb00], al
1405a31be: lea rcx, [rbx + 0xb00]
1405a31c5: call 0x1428a3e00
1405a31ca: mov rsi, rax
1405a31cd: mov rdx, qword ptr [rbx + 0x130]
1405a31d4: test rdx, rdx
1405a31d7: jne 0x1405a2d99
1405a31dd: jmp 0x1405a2dab
1405a31e2: xor edi, edi
1405a31e4: mov byte ptr [rbx + 0xb4], 1
1405a31eb: mov byte ptr [rbx + 0xb5], 1
1405a31f2: mov byte ptr [rbx + 0xb6], 1
1405a31f9: mov byte ptr [rbx + 0xb7], 0
1405a3200: mov byte ptr [rbx + 0xb8], 1
1405a3207: mov byte ptr [rbx + 0xb9], 1
1405a320e: mov byte ptr [rbx + 0xba], 1
1405a3215: mov byte ptr [rbx + 0xbb], 1
1405a321c: lea rcx, [rbx + 0x2d0]
1405a3223: call 0x140916420
1405a3228: mov byte ptr [rbx + 0xb4], 1
1405a322f: mov byte ptr [rbx + 0xb5], 1
1405a3236: mov byte ptr [rbx + 0xb6], 1
1405a323d: mov byte ptr [rbx + 0xb7], 0
1405a3244: mov byte ptr [rbx + 0xb8], 1
1405a324b: mov byte ptr [rbx + 0xb9], 1
1405a3252: mov byte ptr [rbx + 0xba], 1
1405a3259: mov byte ptr [rbx + 0xbb], 1
1405a3260: lea rcx, [rbx + 0xb00]
1405a3267: lea rdx, [rbx + 0x2d0]
1405a326e: call 0x140b46000
1405a3273: mov byte ptr [rbx + 0x440], 1
1405a327a: lea rcx, [rbx + 0xb00]
1405a3281: call 0x140d76ed0
1405a3286: mov r8, rdx
1405a3289: lea rcx, [rbx + 0x440]
1405a3290: mov rdx, rax
1405a3293: call 0x140d5a150
1405a3298: mov rsi, rax
1405a329b: mov rdx, qword ptr [rbx + 0xb00]
1405a32a2: test rdx, rdx
1405a32a5: je 0x1405a32b9
1405a32a7: mov rcx, qword ptr [rbx + 0xb08]
1405a32ae: mov r8d, 1
1405a32b4: call 0x140613c20
1405a32b9: test rsi, rsi
1405a32bc: jne 0x1405a6ea9
1405a32c2: movups xmm0, xmmword ptr [rbx + 0x2d0]
1405a32c9: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a32d0: mov byte ptr [rbx + 0xb4], 1
1405a32d7: mov byte ptr [rbx + 0xb5], 1
1405a32de: mov byte ptr [rbx + 0xb6], 1
1405a32e5: mov byte ptr [rbx + 0xb7], 0
1405a32ec: mov byte ptr [rbx + 0xb8], 1
1405a32f3: mov byte ptr [rbx + 0xb9], 1
1405a32fa: mov byte ptr [rbx + 0xba], 1
1405a3301: mov byte ptr [rbx + 0xbb], 1
1405a3308: lea rcx, [rbx + 0x440]
1405a330f: lea rdx, [rbx + 0xb00]
1405a3316: call 0x140b46920
1405a331b: mov rax, qword ptr [rbp + 0x37c0]
1405a3322: lea rsi, [rax + 0xa8]
1405a3329: mov qword ptr [rbx + 0x2bf8], 0xa
1405a3334: movabs rax, 0x8000000000000005
1405a333e: mov qword ptr [rbx + 0x2bf0], rax
1405a3345: movabs rax, 0x8000000000000001
1405a334f: mov qword ptr [rbx + 0x2590], rax
1405a3356: mov rcx, rsi
1405a3359: call 0x140d76ed0
1405a335e: mov r8, rdx
1405a3361: mov qword ptr [rsp + 0x20], 0xb
1405a336a: lea r9, [rip + 0x245c67f]
1405a3371: lea rcx, [rbx + 0x2230]
1405a3378: mov rdx, rax
1405a337b: call 0x140d62810
1405a3380: movups xmm0, xmmword ptr [rbx + 0x2230]
1405a3387: movups xmm1, xmmword ptr [rbx + 0x2240]
1405a338e: movups xmmword ptr [rbx + 0xb88], xmm1
1405a3395: movups xmmword ptr [rbx + 0xb78], xmm0
1405a339c: movups xmm0, xmmword ptr [rbx + 0x2bf0]
1405a33a3: movups xmm1, xmmword ptr [rbx + 0x2c00]
1405a33aa: movups xmm2, xmmword ptr [rbx + 0x2c10]
1405a33b1: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a33b8: movaps xmmword ptr [rbx + 0xb10], xmm1
1405a33bf: movaps xmmword ptr [rbx + 0xb20], xmm2
1405a33c6: mov rax, qword ptr [rbx + 0x2c20]
1405a33cd: mov qword ptr [rbx + 0xb30], rax
1405a33d4: movups xmm0, xmmword ptr [rbx + 0x2590]
1405a33db: movups xmm1, xmmword ptr [rbx + 0x25a0]
1405a33e2: movups xmm2, xmmword ptr [rbx + 0x25b0]
1405a33e9: movups xmmword ptr [rbx + 0xb38], xmm0
1405a33f0: movups xmmword ptr [rbx + 0xb48], xmm1
1405a33f7: movups xmmword ptr [rbx + 0xb58], xmm2
1405a33fe: mov rax, qword ptr [rbx + 0x25c0]
1405a3405: mov qword ptr [rbx + 0xb68], rax
1405a340c: movabs rax, 0x8000000000000008
1405a3416: mov qword ptr [rbx + 0xb70], rax
1405a341d: mov qword ptr [rbx + 0xb98], rdi
1405a3424: mov byte ptr [rbx + 0x11e], 0
1405a342b: lea rcx, [rbx + 0x2590]
1405a3432: lea rdx, [rbx + 0x440]
1405a3439: lea r8, [rbx + 0xb00]
1405a3440: call 0x1403b5150
1405a3445: mov byte ptr [rbx + 0xb4], 1
1405a344c: mov byte ptr [rbx + 0xb5], 1
1405a3453: mov byte ptr [rbx + 0xb6], 1
1405a345a: mov byte ptr [rbx + 0xb7], 0
1405a3461: mov byte ptr [rbx + 0xb8], 1
1405a3468: mov byte ptr [rbx + 0xb9], 1
1405a346f: mov byte ptr [rbx + 0xba], 1
1405a3476: mov byte ptr [rbx + 0xbb], 1
1405a347d: lea rdx, [rbx + 0x2590]
1405a3484: mov rcx, qword ptr [rbx + 0xc8]
1405a348b: call 0x14011a3b0
1405a3490: mov byte ptr [rbx + 0xb4], 1
1405a3497: mov byte ptr [rbx + 0xb5], 1
1405a349e: mov byte ptr [rbx + 0xb6], 1
1405a34a5: mov byte ptr [rbx + 0xb7], 0
1405a34ac: mov byte ptr [rbx + 0xb8], 1
1405a34b3: mov byte ptr [rbx + 0xb9], 1
1405a34ba: mov byte ptr [rbx + 0xba], 1
1405a34c1: mov byte ptr [rbx + 0xbb], 1
1405a34c8: mov rcx, rsi
1405a34cb: call 0x140d76ed0
1405a34d0: mov r8, rdx
1405a34d3: mov byte ptr [rbx + 0xb4], 1
1405a34da: mov byte ptr [rbx + 0xb5], 1
1405a34e1: mov byte ptr [rbx + 0xb6], 1
1405a34e8: mov byte ptr [rbx + 0xb7], 0
1405a34ef: mov byte ptr [rbx + 0xb8], 1
1405a34f6: mov byte ptr [rbx + 0xb9], 1
1405a34fd: mov byte ptr [rbx + 0xba], 1
1405a3504: mov byte ptr [rbx + 0xbb], 1
1405a350b: mov qword ptr [rsp + 0x20], 9
1405a3514: lea r9, [rip + 0x245c4e0]
1405a351b: lea rcx, [rbx + 0x2590]
1405a3522: mov rdx, rax
1405a3525: call 0x140d62810
1405a352a: lea rcx, [rbx + 0xb00]
1405a3531: lea rdx, [rbx + 0x2d0]
1405a3538: call 0x140b46000
1405a353d: mov byte ptr [rbx + 0x111], 1
1405a3544: lea rcx, [rbx + 0xb00]
1405a354b: call 0x140d76ed0
1405a3550: mov r8, rdx
1405a3553: mov byte ptr [rbx + 0x111], 1
1405a355a: mov qword ptr [rsp + 0x20], 0xc
1405a3563: lea r9, [rip + 0x245c49a]
1405a356a: lea rcx, [rbx + 0x440]
1405a3571: mov rdx, rax
1405a3574: call 0x140d62810
1405a3579: mov byte ptr [rbx + 0x111], 0
1405a3580: lea rdi, [rbx + 0x2590]
1405a3587: lea rdx, [rbx + 0x440]
1405a358e: mov rcx, rdi
1405a3591: call 0x1403107e0
1405a3596: test al, 1
1405a3598: jne 0x1405a6ee0
1405a359e: mov rdx, qword ptr [rbx + 0xb00]
1405a35a5: test rdx, rdx
1405a35a8: je 0x1405a35bc
1405a35aa: mov rcx, qword ptr [rbx + 0xb08]
1405a35b1: mov r8d, 1
1405a35b7: call 0x140613c20
1405a35bc: mov byte ptr [rbx + 0xb4], 1
1405a35c3: mov byte ptr [rbx + 0xb5], 1
1405a35ca: mov byte ptr [rbx + 0xb6], 1
1405a35d1: mov byte ptr [rbx + 0xb7], 0
1405a35d8: mov byte ptr [rbx + 0xb8], 1
1405a35df: mov byte ptr [rbx + 0xb9], 1
1405a35e6: mov byte ptr [rbx + 0xba], 1
1405a35ed: mov byte ptr [rbx + 0xbb], 1
1405a35f4: mov rcx, rsi
1405a35f7: call 0x140d76ed0
1405a35fc: mov r8, rdx
1405a35ff: mov byte ptr [rbx + 0xb4], 1
1405a3606: mov byte ptr [rbx + 0xb5], 1
1405a360d: mov byte ptr [rbx + 0xb6], 1
1405a3614: mov byte ptr [rbx + 0xb7], 0
1405a361b: mov byte ptr [rbx + 0xb8], 1
1405a3622: mov byte ptr [rbx + 0xb9], 1
1405a3629: mov byte ptr [rbx + 0xba], 1
1405a3630: mov byte ptr [rbx + 0xbb], 1
1405a3637: mov qword ptr [rsp + 0x20], 9
1405a3640: lea r9, [rip + 0x245c3e1]
1405a3647: lea rcx, [rbx + 0x1ff0]
1405a364e: mov rdx, rax
1405a3651: call 0x140d62810
1405a3656: mov byte ptr [rbx + 0xd3], 1
1405a365d: lea rcx, [rbx + 0x1ff0]
1405a3664: call 0x140d76ed0
1405a3669: mov byte ptr [rbx + 0xd3], 1
1405a3670: mov rcx, rax
1405a3673: call 0x140d62e10
1405a3678: test al, al
1405a367a: je 0x1405a3702
1405a3680: mov byte ptr [rbx + 0xd3], 1
1405a3687: lea rcx, [rbx + 0xb00]
1405a368e: lea rdx, [rbx + 0x2d0]
1405a3695: call 0x140b46000
1405a369a: lea rcx, [rbx + 0xb00]
1405a36a1: call 0x140d76ed0
1405a36a6: mov r8, rdx
1405a36a9: mov qword ptr [rsp + 0x20], 9
1405a36b2: lea r9, [rip + 0x245c36f]
1405a36b9: lea rcx, [rbx + 0x440]
1405a36c0: mov rdx, rax
1405a36c3: call 0x140d62810
1405a36c8: lea rcx, [rbx + 0x1ff0]
1405a36cf: lea rdx, [rbx + 0x440]
1405a36d6: call 0x140355800
1405a36db: test rax, rax
1405a36de: jne 0x1405a700e
1405a36e4: mov rdx, qword ptr [rbx + 0xb00]
1405a36eb: test rdx, rdx
1405a36ee: je 0x1405a3702
1405a36f0: mov rcx, qword ptr [rbx + 0xb08]
1405a36f7: mov r8d, 1
1405a36fd: call 0x140613c20
1405a3702: mov byte ptr [rbx + 0xd3], 1
1405a3709: lea rcx, [rbx + 0xb00]
1405a3710: lea rdx, [rbx + 0x2d0]
1405a3717: call 0x140b46000
1405a371c: lea rcx, [rbx + 0xb00]
1405a3723: call 0x140d76ed0
1405a3728: mov r8, rdx
1405a372b: mov qword ptr [rsp + 0x20], 0xc
1405a3734: lea r9, [rip + 0x245c315]
1405a373b: lea rcx, [rbx + 0x440]
1405a3742: mov rdx, rax
1405a3745: call 0x140d62810
1405a374a: lea r14, [rbx + 0x440]
1405a3751: mov rcx, r14
1405a3754: call 0x140d76ed0
1405a3759: lea r8, [rip + 0x245c2fc]
1405a3760: mov r9d, 5
1405a3766: mov rcx, rax
1405a3769: call 0x140d59430
1405a376e: mov r15, rax
1405a3771: mov rdx, qword ptr [rbx + 0x440]
1405a3778: test rdx, rdx
1405a377b: je 0x1405a378f
1405a377d: mov rcx, qword ptr [rbx + 0x448]
1405a3784: mov r8d, 1
1405a378a: call 0x140613c20
1405a378f: test r15, r15
1405a3792: jne 0x1405a6f17
1405a3798: mov rdx, qword ptr [rbx + 0xb00]
1405a379f: test rdx, rdx
1405a37a2: je 0x1405a37b6
1405a37a4: mov rcx, qword ptr [rbx + 0xb08]
1405a37ab: mov r8d, 1
1405a37b1: call 0x140613c20
1405a37b6: mov byte ptr [rbx + 0xd3], 1
1405a37bd: mov rcx, rsi
1405a37c0: call 0x140d76ed0
1405a37c5: mov r8, rdx
1405a37c8: mov byte ptr [rbx + 0xd3], 1
1405a37cf: mov qword ptr [rsp + 0x20], 0xc
1405a37d8: lea r9, [rip + 0x245c2a1]
1405a37df: lea rcx, [rbx + 0x2230]
1405a37e6: mov rdx, rax
1405a37e9: call 0x140d62810
1405a37ee: mov byte ptr [rbx + 0xc7], 1
1405a37f5: lea rcx, [rbx + 0x2230]
1405a37fc: call 0x140d76ed0
1405a3801: mov r8, rdx
1405a3804: mov byte ptr [rbx + 0xc7], 1
1405a380b: lea rcx, [rbx + 0xb00]
1405a3812: mov rdx, rax
1405a3815: call 0x140d59140
1405a381a: mov byte ptr [rbx + 0xc7], 1
1405a3821: lea rcx, [rbx + 0x370]
1405a3828: lea rdx, [rbx + 0xb00]
1405a382f: lea r8, [rbx + 0x2230]
1405a3836: call 0x140311b40
1405a383b: mov rcx, qword ptr [rbx + 0x370]
1405a3842: mov r8, qword ptr [rbx + 0x378]
1405a3849: mov rax, rcx
1405a384c: neg rax
1405a384f: jo 0x1405a3b68
1405a3855: mov qword ptr [rbx + 0x130], rcx
1405a385c: mov r9, qword ptr [rbx + 0x380]
1405a3863: mov qword ptr [rbx + 0x25a0], 0
1405a386e: mov qword ptr [rbx + 0x2590], 1
1405a3879: mov qword ptr [rbx + 0x2598], 0x80
1405a3884: lea rcx, [rbx + 0xb00]
1405a388b: lea rdx, [rbx + 0x2590]
1405a3892: mov qword ptr [rbx + 0x128], r8
1405a3899: call 0x140522ad0
1405a389e: mov r15, qword ptr [rbx + 0xb00]
1405a38a5: mov rsi, qword ptr [rbx + 0xb08]
1405a38ac: mov rax, r15
1405a38af: neg rax
1405a38b2: movups xmm0, xmmword ptr [rbx + 0xb10]
1405a38b9: movaps xmmword ptr [rbx + 0x440], xmm0
1405a38c0: movups xmm0, xmmword ptr [rbx + 0xb20]
1405a38c7: movaps xmmword ptr [rbx + 0x450], xmm0
1405a38ce: movups xmm0, xmmword ptr [rbx + 0xb30]
1405a38d5: movaps xmmword ptr [rbx + 0x460], xmm0
1405a38dc: movups xmm0, xmmword ptr [rbx + 0xb40]
1405a38e3: movaps xmmword ptr [rbx + 0x470], xmm0
1405a38ea: movups xmm0, xmmword ptr [rbx + 0xb50]
1405a38f1: movaps xmmword ptr [rbx + 0x480], xmm0
1405a38f8: jno 0x1405a3ad5
1405a38fe: mov qword ptr [rbx + 0xb00], rsi
1405a3905: movaps xmm0, xmmword ptr [rbx + 0x440]
1405a390c: movaps xmm1, xmmword ptr [rbx + 0x450]
1405a3913: movaps xmm2, xmmword ptr [rbx + 0x460]
1405a391a: movaps xmm3, xmmword ptr [rbx + 0x470]
1405a3921: movups xmmword ptr [rbx + 0xb08], xmm0
1405a3928: movups xmmword ptr [rbx + 0xb18], xmm1
1405a392f: movups xmmword ptr [rbx + 0xb28], xmm2
1405a3936: movups xmmword ptr [rbx + 0xb38], xmm3
1405a393d: movaps xmm0, xmmword ptr [rbx + 0x480]
1405a3944: movups xmmword ptr [rbx + 0xb48], xmm0
1405a394b: lea rcx, [rbx + 0xb00]
1405a3952: call 0x1428a3e00
1405a3957: mov rsi, rax
1405a395a: mov rdx, qword ptr [rbx + 0x130]
1405a3961: test rdx, rdx
1405a3964: mov r12, qword ptr [rbp + 0x37b0]
1405a396b: jne 0x1405a3b47
1405a3971: jmp 0x1405a3b59
1405a3976: test r10, r10
1405a3979: jne 0x1405a6441
1405a397f: movaps xmm0, xmmword ptr [rbx + 0x1ff0]
1405a3986: movaps xmmword ptr [rbx + 0x1fe0], xmm0
1405a398d: movzx esi, byte ptr [rbx + 0x25b0]
1405a3994: mov rax, qword ptr [rbx + 0x25a8]
1405a399b: mov qword ptr [rbx + 0x370], rax
1405a39a2: test rax, rax
1405a39a5: jne 0x1405a658d
1405a39ab: mov byte ptr [rbx + 0xe3], 1
1405a39b2: lea rcx, [rbx + 0xb00]
1405a39b9: lea rdx, [rbx + 0x1fe0]
1405a39c0: call 0x140b46920
1405a39c5: mov byte ptr [rbx + 0xe3], 1
1405a39cc: lea rcx, [rbx + 0x440]
1405a39d3: lea rdx, [rbx + 0xb00]
1405a39da: mov r8d, esi
1405a39dd: call 0x140b46a20
1405a39e2: movups xmm0, xmmword ptr [rbx + 0x2230]
1405a39e9: movups xmm1, xmmword ptr [rbx + 0x2240]
1405a39f0: movaps xmmword ptr [rbx + 0xb10], xmm1
1405a39f7: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a39fe: mov rsi, qword ptr [rbx + 0x450]
1405a3a05: cmp rsi, qword ptr [rbx + 0x440]
1405a3a0c: jne 0x1405a3a21
1405a3a0e: lea rdx, [rip + 0x2429093]
1405a3a15: lea rcx, [rbx + 0x440]
1405a3a1c: call 0x140562c30
1405a3a21: mov rax, qword ptr [rbx + 0x448]
1405a3a28: mov rcx, rsi
1405a3a2b: shl rcx, 5
1405a3a2f: movaps xmm0, xmmword ptr [rbx + 0xb00]
1405a3a36: movaps xmm1, xmmword ptr [rbx + 0xb10]
1405a3a3d: movups xmmword ptr [rax + rcx + 0x10], xmm1
1405a3a42: movups xmmword ptr [rax + rcx], xmm0
1405a3a46: inc rsi
1405a3a49: mov qword ptr [rbx + 0x450], rsi
1405a3a50: lea rsi, [rbx + 0xb00]
1405a3a57: lea rdx, [rbx + 0x440]
1405a3a5e: mov r8d, 0xa0
1405a3a64: mov rcx, rsi
1405a3a67: call 0x142923c00
1405a3a6c: mov byte ptr [rbx + 0xe3], 0
1405a3a73: mov rcx, qword ptr [rbx + 0xc8]
1405a3a7a: mov rdx, rsi
1405a3a7d: call 0x14011a3b0
1405a3a82: mov rax, qword ptr [rbx + 0x370]
1405a3a89: test rax, rax
1405a3a8c: je 0x1405a3aae
1405a3a8e: lock dec qword ptr [rax]
1405a3a92: jne 0x1405a3aae
1405a3a94: mov byte ptr [rbx + 0xd9], 0
1405a3a9b: mov byte ptr [rbx + 0xda], 1
1405a3aa2: lea rcx, [rbx + 0x370]
1405a3aa9: call 0x1402e5d80
1405a3aae: xor eax, eax
1405a3ab0: cmp rax, qword ptr [rbx + 0x2590]
1405a3ab7: jo 0x1405a30e0
1405a3abd: mov byte ptr [rbx + 0xf4], 0
1405a3ac4: lea rcx, [rbx + 0x2590]
1405a3acb: call 0x140592cc0
1405a3ad0: jmp 0x1405a30e0
1405a3ad5: lea rdx, [rbx + 0xb60]
1405a3adc: lea rcx, [rbx + 0x2c40]
1405a3ae3: mov r8d, 0xc8
1405a3ae9: call 0x142923c00
1405a3aee: movaps xmm0, xmmword ptr [rbx + 0x440]
1405a3af5: movaps xmm1, xmmword ptr [rbx + 0x450]
1405a3afc: movaps xmm2, xmmword ptr [rbx + 0x460]
1405a3b03: movaps xmm3, xmmword ptr [rbx + 0x470]
1405a3b0a: movaps xmmword ptr [rbx + 0x2bf0], xmm0
1405a3b11: movaps xmmword ptr [rbx + 0x2c00], xmm1
1405a3b18: movaps xmmword ptr [rbx + 0x2c10], xmm2
1405a3b1f: movaps xmmword ptr [rbx + 0x2c20], xmm3
1405a3b26: movaps xmm0, xmmword ptr [rbx + 0x480]
1405a3b2d: movaps xmmword ptr [rbx + 0x2c30], xmm0
1405a3b34: mov rdx, qword ptr [rbx + 0x130]
1405a3b3b: test rdx, rdx
1405a3b3e: mov r12, qword ptr [rbp + 0x37b0]
1405a3b45: je 0x1405a3b59
1405a3b47: mov r8d, 1
1405a3b4d: mov rcx, qword ptr [rbx + 0x128]
1405a3b54: call 0x140613c20
1405a3b59: mov rax, r15
1405a3b5c: neg rax
1405a3b5f: jno 0x1405a3c71
1405a3b65: mov r8, rsi
1405a3b68: mov qword ptr [rbx + 0x2590], r8
1405a3b6f: mov rax, qword ptr [rip + 0x3088832]
1405a3b76: mov rax, qword ptr [rax]
1405a3b79: cmp rax, 2
1405a3b7d: jb 0x1405a3c56
1405a3b83: mov qword ptr [rbx + 0x440], rdi
1405a3b8a: lea rax, [rip + 0x7a908f]
1405a3b91: mov qword ptr [rbx + 0x448], rax
1405a3b98: lea rcx, [rip + 0x245bf21]
1405a3b9f: call 0x14067dd80
1405a3ba4: movups xmm0, xmmword ptr [rax]
1405a3ba7: mov eax, dword ptr [rax + 0x10]
1405a3baa: mov qword ptr [rbx + 0xb30], 2
1405a3bb5: lea rcx, [rip + 0x245b834]
1405a3bbc: mov qword ptr [rbx + 0xb38], rcx
1405a3bc3: mov qword ptr [rbx + 0xb40], 0x14
1405a3bce: lea rdx, [rip + 0x245bedb]
1405a3bd5: mov qword ptr [rbx + 0xb50], rdx
1405a3bdc: mov qword ptr [rbx + 0xb58], 1
1405a3be7: mov qword ptr [rbx + 0xb60], r14
1405a3bee: mov qword ptr [rbx + 0xb68], 1
1405a3bf9: mov qword ptr [rbx + 0xb70], 0
1405a3c04: mov qword ptr [rbx + 0xb00], 0
1405a3c0f: mov qword ptr [rbx + 0xb08], rcx
1405a3c16: mov qword ptr [rbx + 0xb10], 0x14
1405a3c21: mov qword ptr [rbx + 0xb18], 0
1405a3c2c: movups xmmword ptr [rbx + 0xb20], xmm0
1405a3c33: mov dword ptr [rbx + 0xb48], 1
1405a3c3d: mov dword ptr [rbx + 0xb4c], eax
1405a3c43: lea rcx, [rbx + 0x370]
1405a3c4a: lea rdx, [rbx + 0xb00]
1405a3c51: call 0x140d4a330
1405a3c56: mov byte ptr [rbx + 0xc7], 1
1405a3c5d: lea rcx, [rbx + 0x2590]
1405a3c64: call 0x140d4cc30
1405a3c69: mov sil, 1
1405a3c6c: jmp 0x1405a3d16
1405a3c71: mov qword ptr [rbx + 0x2590], r15
1405a3c78: mov qword ptr [rbx + 0x2598], rsi
1405a3c7f: lea rcx, [rbx + 0x25a0]
1405a3c86: lea rdx, [rbx + 0x2bf0]
1405a3c8d: mov r8d, 0x118
1405a3c93: call 0x142923c00
1405a3c98: lea rsi, [rbx + 0x440]
1405a3c9f: mov r8d, 0x128
1405a3ca5: mov rcx, rsi
1405a3ca8: mov rdx, r12
1405a3cab: call 0x142923c00
1405a3cb0: movups xmm0, xmmword ptr [rbx + 0x2cc8]
1405a3cb7: movups xmmword ptr [rbx + 0x528], xmm0
1405a3cbe: lea rdi, [rbx + 0xb00]
1405a3cc5: mov r8d, 0x128
1405a3ccb: mov rcx, rdi
1405a3cce: mov rdx, rsi
1405a3cd1: call 0x142923c00
1405a3cd6: mov rcx, qword ptr [rbx + 0xc8]
1405a3cdd: mov rdx, rdi
1405a3ce0: call 0x140119810
1405a3ce5: lea rcx, [rbx + 0x2590]
1405a3cec: call 0x1405fdb80
1405a3cf1: mov rdx, qword ptr [rbx + 0x2658]
1405a3cf8: mov rax, rdx
1405a3cfb: neg rax
1405a3cfe: jo 0x1405a3d14
1405a3d00: jae 0x1405a3d14
1405a3d02: mov rcx, qword ptr [rbx + 0x2660]
1405a3d09: mov r8d, 1
1405a3d0f: call 0x140613c20
1405a3d14: xor esi, esi
1405a3d16: mov rcx, qword ptr [rbx + 0xc8]
1405a3d1d: mov byte ptr [rbx + 0xc7], sil
1405a3d24: mov dl, 1
1405a3d26: call 0x1401209b0
1405a3d2b: mov byte ptr [rbx + 0xc7], sil
1405a3d32: lea r8, [rip + 0x245bd9f]
1405a3d39: lea rcx, [rbx + 0xb00]
1405a3d40: mov dl, 1
1405a3d42: call 0x1401525a0
1405a3d47: mov dword ptr [rbx + 0x130], esi
1405a3d4d: mov rcx, qword ptr [rip + 0x308861c]
1405a3d54: lea r8, [rip + 0x23c7b0d]
1405a3d5b: lea rdx, [rbx + 0x370]
1405a3d62: call 0x140364d20
1405a3d67: mov r8, rdx
1405a3d6a: lea rcx, [rbx + 0xb00]
1405a3d71: mov rdx, rax
1405a3d74: call 0x1400d67b0
1405a3d79: movups xmm0, xmmword ptr [rbx + 0xb60]
1405a3d80: movaps xmmword ptr [rbx + 0x4a0], xmm0
1405a3d87: movups xmm0, xmmword ptr [rbx + 0xb50]
1405a3d8e: movaps xmmword ptr [rbx + 0x490], xmm0
1405a3d95: movups xmm0, xmmword ptr [rbx + 0xb40]
1405a3d9c: movaps xmmword ptr [rbx + 0x480], xmm0
1405a3da3: movups xmm0, xmmword ptr [rbx + 0xb00]
1405a3daa: movups xmm1, xmmword ptr [rbx + 0xb10]
1405a3db1: movups xmm2, xmmword ptr [rbx + 0xb20]
1405a3db8: movups xmm3, xmmword ptr [rbx + 0xb30]
1405a3dbf: movaps xmmword ptr [rbx + 0x470], xmm3
1405a3dc6: movaps xmmword ptr [rbx + 0x460], xmm2
1405a3dcd: movaps xmmword ptr [rbx + 0x450], xmm1
1405a3dd4: movaps xmmword ptr [rbx + 0x440], xmm0
1405a3ddb: mov edi, dword ptr [rbx + 0x130]
1405a3de1: mov byte ptr [rbx + 0xc7], dil
1405a3de8: lea rdx, [rbx + 0x440]
1405a3def: mov rcx, qword ptr [rbx + 0xc8]
1405a3df6: call 0x1403276f0
1405a3dfb: mov rdx, qword ptr [rbx + 0x2230]
1405a3e02: test rdx, rdx
1405a3e05: je 0x1405a3e19
1405a3e07: mov rcx, qword ptr [rbx + 0x2238]
1405a3e0e: mov r8d, 1
1405a3e14: call 0x140613c20
1405a3e19: mov rdx, qword ptr [rbx + 0x1ff0]
1405a3e20: test rdx, rdx
1405a3e23: je 0x1405a3e37
1405a3e25: mov rcx, qword ptr [rbx + 0x1ff8]
1405a3e2c: mov r8d, 1
1405a3e32: call 0x140613c20
1405a3e37: movaps xmm0, xmmword ptr [rbx + 0xaf0]
1405a3e3e: movaps xmmword ptr [rbx + 0xb50], xmm0
1405a3e45: movaps xmm0, xmmword ptr [rbx + 0xae0]
1405a3e4c: movaps xmmword ptr [rbx + 0xb40], xmm0
1405a3e53: movaps xmm0, xmmword ptr [rbx + 0xaa0]
1405a3e5a: movaps xmm1, xmmword ptr [rbx + 0xab0]
1405a3e61: movaps xmm2, xmmword ptr [rbx + 0xac0]
1405a3e68: movaps xmm3, xmmword ptr [rbx + 0xad0]
1405a3e6f: movaps xmmword ptr [rbx + 0xb30], xmm3
1405a3e76: movaps xmmword ptr [rbx + 0xb20], xmm2
1405a3e7d: movaps xmmword ptr [rbx + 0xb10], xmm1
1405a3e84: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a3e8b: call 0x140d4d740
1405a3e90: mov rsi, rax
1405a3e93: test byte ptr [rax], 1
1405a3e96: mov dword ptr [rbx + 0x130], edi
1405a3e9c: je 0x1405a3eb7
1405a3e9e: mov rcx, rsi
1405a3ea1: add rcx, 8
1405a3ea5: mov rax, qword ptr [rsi + 8]
1405a3ea9: mov rdx, qword ptr [rsi + 0x10]
1405a3ead: movzx r13d, byte ptr [rbx + 0x107]
1405a3eb5: jmp 0x1405a3ed9
1405a3eb7: call 0x140d64e00
1405a3ebc: mov r13b, byte ptr [rbx + 0x107]
1405a3ec3: mov rcx, rsi
1405a3ec6: add rcx, 8
1405a3eca: mov qword ptr [rsi], 1
1405a3ed1: mov qword ptr [rsi + 8], rax
1405a3ed5: mov qword ptr [rsi + 0x10], rdx
1405a3ed9: lea r8, [rax + 1]
1405a3edd: mov qword ptr [rcx], r8
1405a3ee0: movaps xmm0, xmmword ptr [rbx + 0xb50]
1405a3ee7: movups xmmword ptr [rbx + 0x21f0], xmm0
1405a3eee: movaps xmm0, xmmword ptr [rbx + 0xb40]
1405a3ef5: movups xmmword ptr [rbx + 0x21e0], xmm0
1405a3efc: movaps xmm0, xmmword ptr [rbx + 0xb00]
1405a3f03: movaps xmm1, xmmword ptr [rbx + 0xb10]
1405a3f0a: movaps xmm2, xmmword ptr [rbx + 0xb20]
1405a3f11: movaps xmm3, xmmword ptr [rbx + 0xb30]
1405a3f18: movups xmmword ptr [rbx + 0x21d0], xmm3
1405a3f1f: movups xmmword ptr [rbx + 0x21c0], xmm2
1405a3f26: movups xmmword ptr [rbx + 0x21b0], xmm1
1405a3f2d: movups xmmword ptr [rbx + 0x21a0], xmm0
1405a3f34: movups xmmword ptr [rbx + 0x2200], xmm7
1405a3f3b: movups xmmword ptr [rbx + 0x2210], xmm6
1405a3f42: mov qword ptr [rbx + 0x2220], rax
1405a3f49: mov qword ptr [rbx + 0x2228], rdx
1405a3f50: mov qword ptr [rbx + 0x2188], 0
1405a3f5b: mov qword ptr [rbx + 0x2190], 8
1405a3f66: mov qword ptr [rbx + 0x2198], 0
1405a3f71: movabs rax, 0x8000000000000000
1405a3f7b: mov qword ptr [rbx + 0xb10], rax
1405a3f82: mov qword ptr [rbx + 0xb00], 0
1405a3f8d: mov byte ptr [rbx + 0xb28], 0
1405a3f94: movzx eax, byte ptr [rip + 0x3090c6e]
1405a3f9b: mov ecx, 0x12
1405a3fa0: mov edx, 1
1405a3fa5: call 0x140613c10
1405a3faa: test rax, rax
1405a3fad: mov qword ptr [rbx + 0x128], rax
1405a3fb4: je 0x1405a6e02
1405a3fba: movups xmm0, xmmword ptr [rip + 0x245bb2f]
1405a3fc1: movups xmmword ptr [rax], xmm0
1405a3fc4: mov word ptr [rax + 0x10], 0x7265
1405a3fca: mov qword ptr [rbx + 0x2590], 0x12
1405a3fd5: mov qword ptr [rbx + 0x2598], rax
1405a3fdc: mov qword ptr [rbx + 0x25a0], 0x12
1405a3fe7: lea rcx, [rbx + 0x440]
1405a3fee: lea rdx, [rbx + 0xb00]
1405a3ff5: lea r14, [rbx + 0x2590]
1405a3ffc: mov r8, r14
1405a3fff: call 0x140d558f0
1405a4004: mov r8, qword ptr [rbx + 0x158]
1405a400b: mov byte ptr [rbx + 0xd4], 1
1405a4012: mov byte ptr [rbx + 0xd5], 1
1405a4019: mov byte ptr [rbx + 0xd6], 1
1405a4020: mov byte ptr [rbx + 0xd7], 0
1405a4027: mov byte ptr [rbx + 0xd8], 1
1405a402e: lea rcx, [rbx + 0xb00]
1405a4035: lea rdx, [rbx + 0x440]
1405a403c: call 0x14028ae90
1405a4041: cmp qword ptr [rbx + 0xb00], 0
1405a4049: je 0x1405a6e1d
1405a404f: mov rax, qword ptr [rbx + 0xb10]
1405a4056: mov qword ptr [rbx + 0x2a0], rax
1405a405d: movups xmm0, xmmword ptr [rbx + 0xb00]
1405a4064: movaps xmmword ptr [rbx + 0x290], xmm0
1405a406b: mov rax, qword ptr [rbp + 0x37c0]
1405a4072: cmp byte ptr [rax + 0x14a], 0
1405a4079: je 0x1405a409c
1405a407b: mov byte ptr [rbx + 0xe4], 1
1405a4082: mov byte ptr [rbx + 0xe5], 1
1405a4089: mov byte ptr [rbx + 0xe6], 1
1405a4090: mov byte ptr [rbx + 0xe7], 1
1405a4097: call 0x140365be0
1405a409c: mov qword ptr [rbx + 0xc80], 1
1405a40a7: mov qword ptr [rbx + 0xc88], 1
1405a40b2: mov byte ptr [rbx + 0xc90], 0
1405a40b9: xorps xmm0, xmm0
1405a40bc: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a40c3: movaps xmmword ptr [rbx + 0xb80], xmm0
1405a40ca: mov word ptr [rbx + 0xc00], 0
1405a40d3: mov qword ptr [rbx + 0xc08], 0
1405a40de: mov qword ptr [rbx + 0xc10], 8
1405a40e9: movups xmmword ptr [rbx + 0xc18], xmm0
1405a40f0: mov qword ptr [rbx + 0xc28], 8
1405a40fb: mov qword ptr [rbx + 0xc30], 0
1405a4106: mov byte ptr [rbx + 0xc38], 1
1405a410d: movzx eax, byte ptr [rip + 0x3090af5]
1405a4114: mov ecx, 0x200
1405a4119: mov edx, 0x80
1405a411e: call 0x140613c10
1405a4123: test rax, rax
1405a4126: je 0x1405a6e5b
1405a412c: mov rsi, rax
1405a412f: lea rdx, [rbx + 0xb00]
1405a4136: mov r8d, 0x200
1405a413c: mov rcx, rax
1405a413f: call 0x142923c00
1405a4144: mov qword ptr [rbx + 0x2e0], 1
1405a414f: mov qword ptr [rbx + 0x2e8], rsi
1405a4156: mov qword ptr [rbx + 0x2f0], 1
1405a4161: mov qword ptr [rbx + 0x2f8], rsi
1405a4168: test r13b, r13b
1405a416b: je 0x1405a43f1
1405a4171: mov byte ptr [rbx + 0xc2], 1
1405a4178: mov byte ptr [rbx + 0xc3], 1
1405a417f: mov byte ptr [rbx + 0xc4], 1
1405a4186: mov byte ptr [rbx + 0xc5], 1
1405a418d: mov byte ptr [rbx + 0xc6], 1
1405a4194: movabs rdx, 0xaed7b5777ef68ba3
1405a419e: movabs r8, 0x5834a199b0692b72
1405a41a8: mov rcx, qword ptr [rbx + 0xe8]
1405a41af: call 0x1400dd6f0
1405a41b4: test rax, rax
1405a41b7: mov rcx, qword ptr [rbx + 0xc8]
1405a41be: je 0x1405a65f1
1405a41c4: cmp qword ptr [rcx + 0x218], rdx
1405a41cb: jbe 0x1405a65f1
1405a41d1: mov rax, qword ptr [rcx + 0x210]
1405a41d8: mov rax, qword ptr [rax + rdx*8]
1405a41dc: test rax, rax
1405a41df: je 0x1405a65f1
1405a41e5: mov rcx, qword ptr [rcx + 0x1e0]
1405a41ec: not rax
1405a41ef: lea rax, [rax + rax*2]
1405a41f3: shl rax, 5
1405a41f7: cmp qword ptr [rcx + rax + 0x20], 0
1405a41fd: je 0x1405a65f1
1405a4203: add rcx, rax
1405a4206: mov byte ptr [rbx + 0xc2], 1
1405a420d: mov byte ptr [rbx + 0xc3], 1
1405a4214: mov byte ptr [rbx + 0xc4], 1
1405a421b: mov byte ptr [rbx + 0xc5], 1
1405a4222: mov byte ptr [rbx + 0xc6], 1
1405a4229: xor edx, edx
1405a422b: call 0x140cff710
1405a4230: mov rcx, qword ptr [rax]
1405a4233: mov byte ptr [rbx + 0xc2], 1
1405a423a: mov byte ptr [rbx + 0xc3], 1
1405a4241: mov byte ptr [rbx + 0xc4], 1
1405a4248: mov byte ptr [rbx + 0xc5], 1
1405a424f: mov byte ptr [rbx + 0xc6], 1
1405a4256: call 0x14067dd80
1405a425b: mov byte ptr [rbx + 0xc2], 1
1405a4262: mov byte ptr [rbx + 0xc3], 1
1405a4269: mov byte ptr [rbx + 0xc4], 1
1405a4270: mov byte ptr [rbx + 0xc5], 1
1405a4277: mov byte ptr [rbx + 0xc6], 1
1405a427e: mov rcx, qword ptr [rbx + 0xc8]
1405a4285: mov rdx, rax
1405a4288: call 0x14013c9f0
1405a428d: mov byte ptr [rbx + 0xc2], 1
1405a4294: mov byte ptr [rbx + 0xc3], 1
1405a429b: mov byte ptr [rbx + 0xc4], 1
1405a42a2: mov byte ptr [rbx + 0xc5], 1
1405a42a9: mov byte ptr [rbx + 0xc6], 1
1405a42b0: movabs rdx, 0xd8f93d064ea9d61c
1405a42ba: movabs r8, 0x6447617066d9832f
1405a42c4: mov rcx, qword ptr [rbx + 0xe8]
1405a42cb: call 0x1400dd6f0
1405a42d0: test rax, rax
1405a42d3: mov r8, qword ptr [rbx + 0xc8]
1405a42da: je 0x1405a66af
1405a42e0: cmp qword ptr [r8 + 0x218], rdx
1405a42e7: jbe 0x1405a66af
1405a42ed: mov rax, qword ptr [r8 + 0x210]
1405a42f4: mov rcx, qword ptr [rax + rdx*8]
1405a42f8: test rcx, rcx
1405a42fb: je 0x1405a66af
1405a4301: mov rax, qword ptr [r8 + 0x1e0]
1405a4308: not rcx
1405a430b: lea rcx, [rcx + rcx*2]
1405a430f: shl rcx, 5
1405a4313: cmp qword ptr [rax + rcx + 0x20], 0
1405a4319: je 0x1405a66af
1405a431f: add rax, rcx
1405a4322: mov rsi, qword ptr [rax + 0x10]
1405a4326: mov ecx, dword ptr [r8 + 0x5b0]
1405a432d: mov dword ptr [rax + 0x5c], ecx
1405a4330: mov rdi, qword ptr [rsi + 0x30]
1405a4334: mov r15, qword ptr [rsi + 0x40]
1405a4338: cmp rdi, qword ptr [rsi + 0x20]
1405a433c: jne 0x1405a4371
1405a433e: lea rcx, [rsi + 0x20]
1405a4342: mov byte ptr [rbx + 0xc2], 1
1405a4349: mov byte ptr [rbx + 0xc3], 1
1405a4350: mov byte ptr [rbx + 0xc4], 1
1405a4357: mov byte ptr [rbx + 0xc5], 1
1405a435e: mov byte ptr [rbx + 0xc6], 1
1405a4365: lea rdx, [rip + 0x244edbc]
1405a436c: call 0x140562870
1405a4371: mov rax, qword ptr [rsi + 0x28]
1405a4375: mov qword ptr [rax + rdi*8], r15
1405a4379: inc rdi
1405a437c: mov qword ptr [rsi + 0x30], rdi
1405a4380: inc qword ptr [rsi + 0x40]
1405a4384: lea rcx, [rbx + 0x440]
1405a438b: mov r8d, 0x658
1405a4391: mov rdx, qword ptr [rbx + 0xc8]
1405a4398: call 0x142923c00
1405a439d: movups xmm0, xmmword ptr [rbx + 0x2f0]
1405a43a4: movups xmmword ptr [rbx + 0x280], xmm0
1405a43ab: movups xmm0, xmmword ptr [rbx + 0x328]
1405a43b2: movups xmmword ptr [rbx + 0x24e8], xmm0
1405a43b9: mov rax, qword ptr [rbx + 0x160]
1405a43c0: cmp rax, 2
1405a43c4: je 0x1405a5fc5
1405a43ca: cmp eax, 1
1405a43cd: jne 0x1405a5fdc
1405a43d3: mov rcx, qword ptr [rbx + 0x168]
1405a43da: lock inc qword ptr [rcx + 0x180]
1405a43e2: jg 0x1405a5ff1
1405a43e8: mov ecx, 7
1405a43ed: int 0x29
1405a43ef: ud2 
1405a43f1: mov r15, qword ptr [rbx + 0x138]
1405a43f8: lock inc qword ptr [r15]
1405a43fc: jle 0x1405a7040
1405a4402: mov qword ptr [rbx + 0x2230], r15
1405a4409: mov byte ptr [rbx + 0x11f], 1
1405a4410: lea rcx, [rbx + 0x440]
1405a4417: call 0x140cad810
1405a441c: mov byte ptr [rbx + 0xb3], 1
1405a4423: lea rcx, [rbx + 0x440]
1405a442a: call 0x140945ed0
1405a442f: mov byte ptr [rbx + 0xb3], 1
1405a4436: movabs rdx, 0x864ddca98c2312a6
1405a4440: movabs r8, 0x27b0e9369e9a484e
1405a444a: mov rcx, qword ptr [rbx + 0xe8]
1405a4451: call 0x1400dd6f0
1405a4456: test rax, rax
1405a4459: mov r8, qword ptr [rbx + 0xc8]
1405a4460: je 0x1405a6625
1405a4466: cmp qword ptr [r8 + 0x218], rdx
1405a446d: jbe 0x1405a6625
1405a4473: mov rax, qword ptr [r8 + 0x210]
1405a447a: mov rcx, qword ptr [rax + rdx*8]
1405a447e: test rcx, rcx
1405a4481: je 0x1405a6625
1405a4487: mov rax, qword ptr [r8 + 0x1e0]
1405a448e: not rcx
1405a4491: lea rcx, [rcx + rcx*2]
1405a4495: shl rcx, 5
1405a4499: cmp qword ptr [rax + rcx + 0x20], 0
1405a449f: je 0x1405a6625
1405a44a5: add rax, rcx
1405a44a8: mov rax, qword ptr [rax + 0x10]
1405a44ac: mov r12d, dword ptr [rax]
1405a44af: mov r13d, dword ptr [rax + 4]
1405a44b3: mov byte ptr [rbx + 0xb3], 1
1405a44ba: lea rcx, [rbx + 0x440]
1405a44c1: call 0x140113ff0
1405a44c6: cmp al, 1
1405a44c8: ja 0x1405a6f4e
1405a44ce: mov byte ptr [rbx + 0xb3], 1
1405a44d5: lea rdx, [rip + 0x245b6a4]
1405a44dc: lea rcx, [rbx + 0x440]
1405a44e3: call 0x14032b3f0
1405a44e8: mov byte ptr [rbx + 0xb3], 1
1405a44ef: lea rcx, [rbx + 0x440]
1405a44f6: call 0x140113ff0
1405a44fb: cmp al, 1
1405a44fd: ja 0x1405a6f9b
1405a4503: mov byte ptr [rbx + 0xb3], 1
1405a450a: lea r9, [rip + 0x245b687]
1405a4511: lea r8, [rbx + 0x440]
1405a4518: mov cl, 1
1405a451a: mov dl, 1
1405a451c: call 0x1400cd450
1405a4521: lea rdi, [rbx + 0x478]
1405a4528: mov byte ptr [rbx + 0xb3], 1
1405a452f: movabs rdx, 0xbbdd82cfacba045f
1405a4539: movabs r8, 0xcaf27ba849498836
1405a4543: mov rcx, rdi
1405a4546: call 0x1400dd6f0
1405a454b: test rax, rax
1405a454e: je 0x1405a4583
1405a4550: cmp qword ptr [rbx + 0x658], rdx
1405a4557: jbe 0x1405a4583
1405a4559: mov rax, qword ptr [rbx + 0x650]
1405a4560: mov rax, qword ptr [rax + rdx*8]
1405a4564: test rax, rax
1405a4567: je 0x1405a4583
1405a4569: mov rcx, qword ptr [rbx + 0x620]
1405a4570: not rax
1405a4573: lea rax, [rax + rax*2]
1405a4577: shl rax, 5
1405a457b: cmp qword ptr [rcx + rax + 0x20], 0
1405a4581: jne 0x1405a4596
1405a4583: mov byte ptr [rbx + 0xb3], 1
1405a458a: lea rcx, [rbx + 0x440]
1405a4591: call 0x140566280
1405a4596: mov byte ptr [rbx + 0xb3], 1
1405a459d: movabs rdx, 0x1c5d03917730cc4f
1405a45a7: movabs r8, 0x3d29e4fb18bada6c
1405a45b1: mov rcx, rdi
1405a45b4: call 0x1400dd6f0
1405a45b9: test rax, rax
1405a45bc: je 0x1405a45f1
1405a45be: cmp qword ptr [rbx + 0x658], rdx
1405a45c5: jbe 0x1405a45f1
1405a45c7: mov rax, qword ptr [rbx + 0x650]
1405a45ce: mov rax, qword ptr [rax + rdx*8]
1405a45d2: test rax, rax
1405a45d5: je 0x1405a45f1
1405a45d7: mov rcx, qword ptr [rbx + 0x620]
1405a45de: not rax
1405a45e1: lea rax, [rax + rax*2]
1405a45e5: shl rax, 5
1405a45e9: cmp qword ptr [rcx + rax + 0x20], 0
1405a45ef: jne 0x1405a4604
1405a45f1: mov byte ptr [rbx + 0xb3], 1
1405a45f8: lea rcx, [rbx + 0x440]
1405a45ff: call 0x14057e850
1405a4604: mov byte ptr [rbx + 0xb3], 1
1405a460b: movabs rdx, 0x6ecb4684a4c9e1f9
1405a4615: movabs r8, 0xfae2e21abac81ddc
1405a461f: mov rcx, rdi
1405a4622: call 0x1400dd6f0
1405a4627: test rax, rax
1405a462a: je 0x1405a465f
1405a462c: cmp qword ptr [rbx + 0x658], rdx
1405a4633: jbe 0x1405a465f
1405a4635: mov rax, qword ptr [rbx + 0x650]
1405a463c: mov rax, qword ptr [rax + rdx*8]
1405a4640: test rax, rax
1405a4643: je 0x1405a465f
1405a4645: mov rcx, qword ptr [rbx + 0x620]
1405a464c: not rax
1405a464f: lea rax, [rax + rax*2]
1405a4653: shl rax, 5
1405a4657: cmp qword ptr [rcx + rax + 0x20], 0
1405a465d: jne 0x1405a4672
1405a465f: mov byte ptr [rbx + 0xb3], 1
1405a4666: lea rcx, [rbx + 0x440]
1405a466d: call 0x140575a00
1405a4672: mov byte ptr [rbx + 0xb3], 1
1405a4679: movabs rdx, 0xdac2c590678dd6c0
1405a4683: movabs r8, 0x5d30eb6733ae4d59
1405a468d: mov rcx, rdi
1405a4690: call 0x1400dd6f0
1405a4695: test rax, rax
1405a4698: je 0x1405a46cd
1405a469a: cmp qword ptr [rbx + 0x658], rdx
1405a46a1: jbe 0x1405a46cd
1405a46a3: mov rax, qword ptr [rbx + 0x650]
1405a46aa: mov rax, qword ptr [rax + rdx*8]
1405a46ae: test rax, rax
1405a46b1: je 0x1405a46cd
1405a46b3: mov rcx, qword ptr [rbx + 0x620]
1405a46ba: not rax
1405a46bd: lea rax, [rax + rax*2]
1405a46c1: shl rax, 5
1405a46c5: cmp qword ptr [rcx + rax + 0x20], 0
1405a46cb: jne 0x1405a46e0
1405a46cd: mov byte ptr [rbx + 0xb3], 1
1405a46d4: lea rcx, [rbx + 0x440]
1405a46db: call 0x140584520
1405a46e0: mov byte ptr [rbx + 0xb3], 1
1405a46e7: movabs rdx, 0xa554fbb3bbf1ca0
1405a46f1: movabs r8, 0x29b993a986bb8e9d
1405a46fb: mov rcx, rdi
1405a46fe: call 0x1400dd6f0
1405a4703: test rax, rax
1405a4706: je 0x1405a473b
1405a4708: cmp qword ptr [rbx + 0x658], rdx
1405a470f: jbe 0x1405a473b
1405a4711: mov rax, qword ptr [rbx + 0x650]
1405a4718: mov rax, qword ptr [rax + rdx*8]
1405a471c: test rax, rax
1405a471f: je 0x1405a473b
1405a4721: mov rcx, qword ptr [rbx + 0x620]
1405a4728: not rax
1405a472b: lea rax, [rax + rax*2]
1405a472f: shl rax, 5
1405a4733: cmp qword ptr [rcx + rax + 0x20], 0
1405a4739: jne 0x1405a474e
1405a473b: mov byte ptr [rbx + 0xb3], 1
1405a4742: lea rcx, [rbx + 0x440]
1405a4749: call 0x1405797e0
1405a474e: mov byte ptr [rbx + 0xb3], 1
1405a4755: movabs rdx, 0xbb7c1673aee764cb
1405a475f: movabs r8, 0x250aee383945e2d6
1405a4769: mov rcx, rdi
1405a476c: call 0x1400dd6f0
1405a4771: test rax, rax
1405a4774: je 0x1405a47a9
1405a4776: cmp qword ptr [rbx + 0x658], rdx
1405a477d: jbe 0x1405a47a9
1405a477f: mov rax, qword ptr [rbx + 0x650]
1405a4786: mov rax, qword ptr [rax + rdx*8]
1405a478a: test rax, rax
1405a478d: je 0x1405a47a9
1405a478f: mov rcx, qword ptr [rbx + 0x620]
1405a4796: not rax
1405a4799: lea rax, [rax + rax*2]
1405a479d: shl rax, 5
1405a47a1: cmp qword ptr [rcx + rax + 0x20], 0
1405a47a7: jne 0x1405a47bc
1405a47a9: mov byte ptr [rbx + 0xb3], 1
1405a47b0: lea rcx, [rbx + 0x440]
1405a47b7: call 0x1405687a0
1405a47bc: mov byte ptr [rbx + 0xb3], 1
1405a47c3: movabs rdx, 0x825d6aa59c91551
1405a47cd: movabs r8, 0xe073afcfe6bed5b6
1405a47d7: mov rcx, rdi
1405a47da: call 0x1400dd6f0
1405a47df: test rax, rax
1405a47e2: je 0x1405a4817
1405a47e4: cmp qword ptr [rbx + 0x658], rdx
1405a47eb: jbe 0x1405a4817
1405a47ed: mov rax, qword ptr [rbx + 0x650]
1405a47f4: mov rax, qword ptr [rax + rdx*8]
1405a47f8: test rax, rax
1405a47fb: je 0x1405a4817
1405a47fd: mov rcx, qword ptr [rbx + 0x620]
1405a4804: not rax
1405a4807: lea rax, [rax + rax*2]
1405a480b: shl rax, 5
1405a480f: cmp qword ptr [rcx + rax + 0x20], 0
1405a4815: jne 0x1405a482a
1405a4817: mov byte ptr [rbx + 0xb3], 1
1405a481e: lea rcx, [rbx + 0x440]
1405a4825: call 0x1405778f0
1405a482a: mov byte ptr [rbx + 0xb3], 1
1405a4831: movabs rdx, 0x923f1ac331d52636
1405a483b: movabs r8, 0x4114c9eaff932cf6
1405a4845: mov rcx, rdi
1405a4848: call 0x1400dd6f0
1405a484d: test rax, rax
1405a4850: je 0x1405a4885
1405a4852: cmp qword ptr [rbx + 0x658], rdx
1405a4859: jbe 0x1405a4885
1405a485b: mov rax, qword ptr [rbx + 0x650]
1405a4862: mov rax, qword ptr [rax + rdx*8]
1405a4866: test rax, rax
1405a4869: je 0x1405a4885
1405a486b: mov rcx, qword ptr [rbx + 0x620]
1405a4872: not rax
1405a4875: lea rax, [rax + rax*2]
1405a4879: shl rax, 5
1405a487d: cmp qword ptr [rcx + rax + 0x20], 0
1405a4883: jne 0x1405a4898
1405a4885: mov byte ptr [rbx + 0xb3], 1
1405a488c: lea rcx, [rbx + 0x440]
1405a4893: call 0x140572250
1405a4898: mov byte ptr [rbx + 0xb3], 1
1405a489f: movabs rdx, 0x2e383ebb9376bfa1
1405a48a9: movabs r8, 0x96b847e2c17c833c
1405a48b3: mov rcx, rdi
1405a48b6: call 0x1400dd6f0
1405a48bb: test rax, rax
1405a48be: je 0x1405a48f3
1405a48c0: cmp qword ptr [rbx + 0x658], rdx
1405a48c7: jbe 0x1405a48f3
1405a48c9: mov rax, qword ptr [rbx + 0x650]
1405a48d0: mov rax, qword ptr [rax + rdx*8]
1405a48d4: test rax, rax
1405a48d7: je 0x1405a48f3
1405a48d9: mov rcx, qword ptr [rbx + 0x620]
1405a48e0: not rax
1405a48e3: lea rax, [rax + rax*2]
1405a48e7: shl rax, 5
1405a48eb: cmp qword ptr [rcx + rax + 0x20], 0
1405a48f1: jne 0x1405a4906
1405a48f3: mov byte ptr [rbx + 0xb3], 1
1405a48fa: lea rcx, [rbx + 0x440]
1405a4901: call 0x140566ee0
1405a4906: mov byte ptr [rbx + 0xb3], 1
1405a490d: movabs rdx, 0x9ec27b7f9851aea
1405a4917: movabs r8, 0x1e73f089b9e6883e
1405a4921: mov rcx, rdi
1405a4924: call 0x1400dd6f0
1405a4929: test rax, rax
1405a492c: je 0x1405a4961
1405a492e: cmp qword ptr [rbx + 0x658], rdx
1405a4935: jbe 0x1405a4961
1405a4937: mov rax, qword ptr [rbx + 0x650]
1405a493e: mov rax, qword ptr [rax + rdx*8]
1405a4942: test rax, rax
1405a4945: je 0x1405a4961
1405a4947: mov rcx, qword ptr [rbx + 0x620]
1405a494e: not rax
1405a4951: lea rax, [rax + rax*2]
1405a4955: shl rax, 5
1405a4959: cmp qword ptr [rcx + rax + 0x20], 0
1405a495f: jne 0x1405a4974
1405a4961: mov byte ptr [rbx + 0xb3], 1
1405a4968: lea rcx, [rbx + 0x440]
1405a496f: call 0x1405791b0
1405a4974: mov byte ptr [rbx + 0xb3], 1
1405a497b: movabs rdx, 0xfc533cfe7fb82312
1405a4985: movabs r8, 0x2e13a6a03dedb277
1405a498f: mov rcx, rdi
1405a4992: call 0x1400dd6f0
1405a4997: test rax, rax
1405a499a: je 0x1405a49cf
1405a499c: cmp qword ptr [rbx + 0x658], rdx
1405a49a3: jbe 0x1405a49cf
1405a49a5: mov rax, qword ptr [rbx + 0x650]
1405a49ac: mov rax, qword ptr [rax + rdx*8]
1405a49b0: test rax, rax
1405a49b3: je 0x1405a49cf
1405a49b5: mov rcx, qword ptr [rbx + 0x620]
1405a49bc: not rax
1405a49bf: lea rax, [rax + rax*2]
1405a49c3: shl rax, 5
1405a49c7: cmp qword ptr [rcx + rax + 0x20], 0
1405a49cd: jne 0x1405a49e2
1405a49cf: mov byte ptr [rbx + 0xb3], 1
1405a49d6: lea rcx, [rbx + 0x440]
1405a49dd: call 0x140570360
1405a49e2: mov byte ptr [rbx + 0xb3], 1
1405a49e9: movabs rdx, 0xa4c02661e67b048b
1405a49f3: movabs r8, 0xeae761e6aa2a67ec
1405a49fd: mov rcx, qword ptr [rbx + 0xe8]
1405a4a04: call 0x1400dd6f0
1405a4a09: test rax, rax
1405a4a0c: mov r8, qword ptr [rbx + 0xc8]
1405a4a13: je 0x1405a66e3
1405a4a19: cmp qword ptr [r8 + 0x218], rdx
1405a4a20: jbe 0x1405a66e3
1405a4a26: mov rax, qword ptr [r8 + 0x210]
1405a4a2d: mov rcx, qword ptr [rax + rdx*8]
1405a4a31: test rcx, rcx
1405a4a34: je 0x1405a66e3
1405a4a3a: mov rax, qword ptr [r8 + 0x1e0]
1405a4a41: not rcx
1405a4a44: lea rcx, [rcx + rcx*2]
1405a4a48: shl rcx, 5
1405a4a4c: cmp qword ptr [rax + rcx + 0x20], 0
1405a4a52: je 0x1405a66e3
1405a4a58: add rax, rcx
1405a4a5b: mov rax, qword ptr [rax + 0x10]
1405a4a5f: mov rcx, qword ptr [rax + 0x10]
1405a4a63: movzx edx, byte ptr [rax + 0x20]
1405a4a67: movups xmm0, xmmword ptr [rax]
1405a4a6a: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a4a71: mov qword ptr [rbx + 0xb10], rcx
1405a4a78: mov byte ptr [rbx + 0xb20], dl
1405a4a7e: movsd xmm0, qword ptr [rax + 0x18]
1405a4a83: movsd qword ptr [rbx + 0xb18], xmm0
1405a4a8b: mov byte ptr [rbx + 0xb3], 1
1405a4a92: lea rcx, [rbx + 0x440]
1405a4a99: lea rdx, [rbx + 0xb00]
1405a4aa0: call 0x14012e420
1405a4aa5: mov rsi, rax
1405a4aa8: mov byte ptr [rbx + 0xb3], 1
1405a4aaf: movabs rdx, 0x3335386a5fbcee80
1405a4ab9: movabs r8, 0x8f2a59f31a65ec60
1405a4ac3: mov rcx, qword ptr [rbx + 0xe8]
1405a4aca: call 0x1400dd6f0
1405a4acf: test rax, rax
1405a4ad2: mov r8, qword ptr [rbx + 0xc8]
1405a4ad9: je 0x1405a676d
1405a4adf: cmp qword ptr [r8 + 0x218], rdx
1405a4ae6: jbe 0x1405a676d
1405a4aec: mov rax, qword ptr [r8 + 0x210]
1405a4af3: mov rcx, qword ptr [rax + rdx*8]
1405a4af7: test rcx, rcx
1405a4afa: je 0x1405a676d
1405a4b00: mov rax, qword ptr [r8 + 0x1e0]
1405a4b07: not rcx
1405a4b0a: lea rcx, [rcx + rcx*2]
1405a4b0e: shl rcx, 5
1405a4b12: cmp qword ptr [rax + rcx + 0x20], 0
1405a4b18: je 0x1405a676d
1405a4b1e: add rax, rcx
1405a4b21: mov rax, qword ptr [rax + 0x10]
1405a4b25: movzx ecx, byte ptr [rax + 0x28]
1405a4b29: mov byte ptr [rbx + 0xb28], cl
1405a4b2f: movups xmm0, xmmword ptr [rax]
1405a4b32: movups xmm1, xmmword ptr [rax + 0x10]
1405a4b36: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a4b3d: movaps xmmword ptr [rbx + 0xb10], xmm1
1405a4b44: mov rax, qword ptr [rax + 0x20]
1405a4b48: mov qword ptr [rbx + 0xb20], rax
1405a4b4f: mov byte ptr [rbx + 0xb3], 1
1405a4b56: lea rdx, [rbx + 0xb00]
1405a4b5d: mov rcx, rsi
1405a4b60: call 0x14013d5f0
1405a4b65: mov byte ptr [rbx + 0xb3], 1
1405a4b6c: mov rcx, rax
1405a4b6f: mov edx, r12d
1405a4b72: mov r8d, r13d
1405a4b75: call 0x140120460
1405a4b7a: mov rsi, rax
1405a4b7d: mov byte ptr [rbx + 0xb3], 1
1405a4b84: lea rcx, [rbx + 0xb00]
1405a4b8b: call 0x140b64d00
1405a4b90: mov byte ptr [rbx + 0xb3], 1
1405a4b97: lea rdx, [rbx + 0xb00]
1405a4b9e: mov rcx, rsi
1405a4ba1: call 0x14012c3c0
1405a4ba6: mov rsi, rax
1405a4ba9: mov byte ptr [rbx + 0xb3], 1
1405a4bb0: movabs rdx, 0x3d7b56e787d47c40
1405a4bba: movabs r8, 0x67b0b10a8d43bfc7
1405a4bc4: mov rcx, qword ptr [rbx + 0xe8]
1405a4bcb: call 0x1400dd6f0
1405a4bd0: test rax, rax
1405a4bd3: mov r8, qword ptr [rbx + 0xc8]
1405a4bda: je 0x1405a67f7
1405a4be0: cmp qword ptr [r8 + 0x218], rdx
1405a4be7: jbe 0x1405a67f7
1405a4bed: mov rax, qword ptr [r8 + 0x210]
1405a4bf4: mov rcx, qword ptr [rax + rdx*8]
1405a4bf8: test rcx, rcx
1405a4bfb: je 0x1405a67f7
1405a4c01: mov rax, qword ptr [r8 + 0x1e0]
1405a4c08: not rcx
1405a4c0b: lea rcx, [rcx + rcx*2]
1405a4c0f: shl rcx, 5
1405a4c13: cmp qword ptr [rax + rcx + 0x20], 0
1405a4c19: je 0x1405a67f7
1405a4c1f: add rax, rcx
1405a4c22: mov rdx, qword ptr [rax + 0x10]
1405a4c26: mov byte ptr [rbx + 0xb3], 1
1405a4c2d: lea rcx, [rbx + 0xb00]
1405a4c34: call 0x140593510
1405a4c39: mov byte ptr [rbx + 0xb3], 1
1405a4c40: lea rdx, [rbx + 0xb00]
1405a4c47: mov rcx, rsi
1405a4c4a: call 0x140119810
1405a4c4f: mov rsi, rax
1405a4c52: mov byte ptr [rbx + 0xb3], 1
1405a4c59: movabs rdx, 0xfdf985a9f052d51c
1405a4c63: movabs r8, 0xd7c2d9b5621db8c6
1405a4c6d: mov rcx, qword ptr [rbx + 0xe8]
1405a4c74: call 0x1400dd6f0
1405a4c79: test rax, rax
1405a4c7c: mov r8, qword ptr [rbx + 0xc8]
1405a4c83: je 0x1405a6881
1405a4c89: cmp qword ptr [r8 + 0x218], rdx
1405a4c90: jbe 0x1405a6881
1405a4c96: mov rax, qword ptr [r8 + 0x210]
1405a4c9d: mov rcx, qword ptr [rax + rdx*8]
1405a4ca1: test rcx, rcx
1405a4ca4: je 0x1405a6881
1405a4caa: mov rax, qword ptr [r8 + 0x1e0]
1405a4cb1: not rcx
1405a4cb4: lea rcx, [rcx + rcx*2]
1405a4cb8: shl rcx, 5
1405a4cbc: cmp qword ptr [rax + rcx + 0x20], 0
1405a4cc2: je 0x1405a6881
1405a4cc8: add rax, rcx
1405a4ccb: mov rax, qword ptr [rax + 0x10]
1405a4ccf: mov rdx, qword ptr [rax]
1405a4cd2: test rdx, rdx
1405a4cd5: je 0x1405a4ce1
1405a4cd7: lock inc qword ptr [rdx]
1405a4cdb: jle 0x1405a7040
1405a4ce1: mov byte ptr [rbx + 0xb3], 1
1405a4ce8: mov rcx, rsi
1405a4ceb: call 0x140114e10
1405a4cf0: mov rsi, rax
1405a4cf3: mov byte ptr [rbx + 0xb3], 1
1405a4cfa: movabs rdx, 0x8b8bde009a4ee44
1405a4d04: movabs r8, 0x2ad7572b7bbdf9dd
1405a4d0e: mov rcx, qword ptr [rbx + 0xe8]
1405a4d15: call 0x1400dd6f0
1405a4d1a: test rax, rax
1405a4d1d: mov r8, qword ptr [rbx + 0xc8]
1405a4d24: je 0x1405a690b
1405a4d2a: cmp qword ptr [r8 + 0x218], rdx
1405a4d31: jbe 0x1405a690b
1405a4d37: mov rax, qword ptr [r8 + 0x210]
1405a4d3e: mov rcx, qword ptr [rax + rdx*8]
1405a4d42: test rcx, rcx
1405a4d45: je 0x1405a690b
1405a4d4b: mov rax, qword ptr [r8 + 0x1e0]
1405a4d52: not rcx
1405a4d55: lea rcx, [rcx + rcx*2]
1405a4d59: shl rcx, 5
1405a4d5d: cmp qword ptr [rax + rcx + 0x20], 0
1405a4d63: je 0x1405a690b
1405a4d69: add rax, rcx
1405a4d6c: mov rax, qword ptr [rax + 0x10]
1405a4d70: movzx edx, byte ptr [rax]
1405a4d73: mov byte ptr [rbx + 0xb3], 1
1405a4d7a: mov rcx, rsi
1405a4d7d: call 0x14013a1d0
1405a4d82: mov rsi, rax
1405a4d85: mov byte ptr [rbx + 0xb3], 1
1405a4d8c: lea rcx, [rbx + 0xb00]
1405a4d93: xor edx, edx
1405a4d95: call 0x1408ba0f0
1405a4d9a: mov byte ptr [rbx + 0xb3], 1
1405a4da1: lea rdx, [rbx + 0xb00]
1405a4da8: mov rcx, rsi
1405a4dab: call 0x140131d60
1405a4db0: mov rsi, rax
1405a4db3: movaps xmmword ptr [rbx + 0x25c0], xmm8
1405a4dbb: movaps xmmword ptr [rbx + 0x25b0], xmm9
1405a4dc3: movaps xmmword ptr [rbx + 0x25a0], xmm10
1405a4dcb: movaps xmmword ptr [rbx + 0x2590], xmm11
1405a4dd3: movaps xmmword ptr [rbx + 0x25d0], xmm11
1405a4ddb: movaps xmmword ptr [rbx + 0x25e0], xmm10
1405a4de3: movaps xmmword ptr [rbx + 0x25f0], xmm9
1405a4deb: movaps xmmword ptr [rbx + 0x2600], xmm8
1405a4df3: movaps xmmword ptr [rbx + 0x2610], xmm11
1405a4dfb: movaps xmmword ptr [rbx + 0x2620], xmm10
1405a4e03: movaps xmmword ptr [rbx + 0x2630], xmm9
1405a4e0b: movaps xmmword ptr [rbx + 0x2640], xmm8
1405a4e13: movaps xmmword ptr [rbx + 0x2650], xmm11
1405a4e1b: movaps xmmword ptr [rbx + 0x2660], xmm10
1405a4e23: movaps xmmword ptr [rbx + 0x2670], xmm9
1405a4e2b: movaps xmmword ptr [rbx + 0x2680], xmm8
1405a4e33: mov dword ptr [rbx + 0xc08], 0
1405a4e3d: lea rdi, [rbx + 0xb00]
1405a4e44: lea rdx, [rbx + 0x2590]
1405a4e4b: mov r8d, 0x100
1405a4e51: mov rcx, rdi
1405a4e54: call 0x142923c00
1405a4e59: mov qword ptr [rbx + 0xc00], 0
1405a4e64: mov byte ptr [rbx + 0xb3], 1
1405a4e6b: mov rcx, rsi
1405a4e6e: mov rdx, rdi
1405a4e71: call 0x140135cc0
1405a4e76: mov rsi, rax
1405a4e79: mov byte ptr [rbx + 0xb3], 1
1405a4e80: lea rcx, [rbx + 0xb00]
1405a4e87: call 0x140936c60
1405a4e8c: mov byte ptr [rbx + 0xb3], 1
1405a4e93: lea rdx, [rbx + 0xb00]
1405a4e9a: mov rcx, rsi
1405a4e9d: call 0x14012a6e0
1405a4ea2: mov rdi, rax
1405a4ea5: lea rsi, [r15 + 0x10]
1405a4ea9: mov byte ptr [rbx + 0xb3], 1
1405a4eb0: lea rcx, [rbx + 0xb00]
1405a4eb7: mov rdx, rsi
1405a4eba: call 0x14093bb10
1405a4ebf: mov byte ptr [rbx + 0xb3], 1
1405a4ec6: lea rdx, [rbx + 0xb00]
1405a4ecd: mov rcx, rdi
1405a4ed0: call 0x140126d20
1405a4ed5: mov rdi, rax
1405a4ed8: mov byte ptr [rbx + 0xb3], 1
1405a4edf: lea rcx, [rbx + 0xb00]
1405a4ee6: call 0x14092c920
1405a4eeb: mov byte ptr [rbx + 0xb3], 1
1405a4ef2: lea rdx, [rbx + 0xb00]
1405a4ef9: mov rcx, rdi
1405a4efc: call 0x140126750
1405a4f01: mov rdi, rax
1405a4f04: mov byte ptr [rbx + 0xb3], 1
1405a4f0b: lea rcx, [rbx + 0xb00]
1405a4f12: call 0x14091eb60
1405a4f17: mov byte ptr [rbx + 0xb3], 1
1405a4f1e: lea rdx, [rbx + 0xb00]
1405a4f25: mov rcx, rdi
1405a4f28: call 0x140119210
1405a4f2d: mov rdi, rax
1405a4f30: mov qword ptr [rbx + 0xba0], 0
1405a4f3b: mov dword ptr [rbx + 0xba8], 0
1405a4f45: mov qword ptr [rbx + 0xbb0], 0
1405a4f50: mov dword ptr [rbx + 0xbb8], 0
1405a4f5a: mov qword ptr [rbx + 0xbc0], 0
1405a4f65: mov dword ptr [rbx + 0xbc8], 0
1405a4f6f: xorps xmm6, xmm6
1405a4f72: movaps xmmword ptr [rbx + 0xbe0], xmm6
1405a4f79: lea rcx, [rbx + 0xbf0]
1405a4f80: mov byte ptr [rbx + 0xb3], 1
1405a4f87: call 0x1400ef490
1405a4f8c: mov dword ptr [rbx + 0xbac], 0
1405a4f96: mov dword ptr [rbx + 0xbbc], 0
1405a4fa0: movups xmmword ptr [rbx + 0xbcc], xmm6
1405a4fa7: mov dword ptr [rbx + 0xbdc], 0
1405a4fb1: movaps xmmword ptr [rbx + 0xb00], xmm6
1405a4fb8: movaps xmmword ptr [rbx + 0xb10], xmm6
1405a4fbf: movaps xmmword ptr [rbx + 0xb20], xmm6
1405a4fc6: movaps xmmword ptr [rbx + 0xb30], xmm6
1405a4fcd: movaps xmmword ptr [rbx + 0xb40], xmm6
1405a4fd4: movaps xmmword ptr [rbx + 0xb50], xmm6
1405a4fdb: movaps xmmword ptr [rbx + 0xb60], xmm6
1405a4fe2: movaps xmmword ptr [rbx + 0xb70], xmm6
1405a4fe9: movaps xmmword ptr [rbx + 0xb80], xmm6
1405a4ff0: movaps xmmword ptr [rbx + 0xb90], xmm6
1405a4ff7: mov byte ptr [rbx + 0xb3], 1
1405a4ffe: lea rdx, [rbx + 0xb00]
1405a5005: mov rcx, rdi
1405a5008: call 0x14012d450
1405a500d: mov rdi, rax
1405a5010: mov byte ptr [rbx + 0xb3], 1
1405a5017: lea rcx, [rbx + 0xb00]
1405a501e: xor edx, edx
1405a5020: call 0x1408ba490
1405a5025: mov byte ptr [rbx + 0xb3], 1
1405a502c: lea rdx, [rbx + 0xb00]
1405a5033: mov rcx, rdi
1405a5036: call 0x140128420
1405a503b: mov byte ptr [rbx + 0xb3], 1
1405a5042: mov rcx, rax
1405a5045: call 0x140115fa0
1405a504a: mov rdi, rax
1405a504d: mov byte ptr [rbx + 0xb3], 1
1405a5054: lea rcx, [rbx + 0xb00]
1405a505b: xor edx, edx
1405a505d: call 0x1408bab30
1405a5062: mov byte ptr [rbx + 0xb3], 1
1405a5069: lea rdx, [rbx + 0xb00]
1405a5070: mov rcx, rdi
1405a5073: call 0x140136de0
1405a5078: mov rdi, rax
1405a507b: mov byte ptr [rbx + 0xb3], 1
1405a5082: lea rcx, [rbx + 0xb00]
1405a5089: call 0x140924030
1405a508e: mov byte ptr [rbx + 0xb3], 1
1405a5095: lea rdx, [rbx + 0xb00]
1405a509c: mov rcx, rdi
1405a509f: call 0x1401214f0
1405a50a4: mov rdi, rax
1405a50a7: mov byte ptr [rbx + 0xb3], 1
1405a50ae: lea rcx, [rbx + 0xb00]
1405a50b5: mov rdx, rsi
1405a50b8: call 0x140c02570
1405a50bd: movups xmm0, xmmword ptr [rbx + 0xb40]
1405a50c4: movups xmmword ptr [rbx + 0x25d7], xmm0
1405a50cb: movups xmm0, xmmword ptr [rbx + 0xb00]
1405a50d2: movups xmm1, xmmword ptr [rbx + 0xb10]
1405a50d9: movups xmm2, xmmword ptr [rbx + 0xb20]
1405a50e0: movups xmm3, xmmword ptr [rbx + 0xb30]
1405a50e7: movups xmmword ptr [rbx + 0x25c7], xmm3
1405a50ee: movups xmmword ptr [rbx + 0x25b7], xmm2
1405a50f5: movups xmmword ptr [rbx + 0x25a7], xmm1
1405a50fc: movups xmmword ptr [rbx + 0x2597], xmm0
1405a5103: mov byte ptr [rbx + 0xb00], 0
1405a510a: movups xmm0, xmmword ptr [rbx + 0x2590]
1405a5111: movups xmmword ptr [rbx + 0xb01], xmm0
1405a5118: movups xmm0, xmmword ptr [rbx + 0x25a0]
1405a511f: movups xmmword ptr [rbx + 0xb11], xmm0
1405a5126: movups xmm0, xmmword ptr [rbx + 0x25b0]
1405a512d: movups xmmword ptr [rbx + 0xb21], xmm0
1405a5134: movups xmm0, xmmword ptr [rbx + 0x25c0]
1405a513b: movups xmmword ptr [rbx + 0xb31], xmm0
1405a5142: movups xmm0, xmmword ptr [rbx + 0x25d0]
1405a5149: movups xmmword ptr [rbx + 0xb41], xmm0
1405a5150: mov rax, qword ptr [rbx + 0x25df]
1405a5157: mov qword ptr [rbx + 0xb50], rax
1405a515e: mov byte ptr [rbx + 0xb3], 1
1405a5165: lea rdx, [rbx + 0xb00]
1405a516c: mov rcx, rdi
1405a516f: call 0x14011f840
1405a5174: mov byte ptr [rbx + 0xb3], 0
1405a517b: mov rcx, rax
1405a517e: mov rdx, r15
1405a5181: call 0x14011cb90
1405a5186: mov rsi, rax
1405a5189: mov byte ptr [rbx + 0xb3], 0
1405a5190: lea rcx, [rbx + 0xb00]
1405a5197: call 0x140924030
1405a519c: mov byte ptr [rbx + 0xb3], 0
1405a51a3: lea rdx, [rbx + 0xb00]
1405a51aa: mov rcx, rsi
1405a51ad: call 0x1401214f0
1405a51b2: mov rsi, rax
1405a51b5: mov byte ptr [rbx + 0xb3], 0
1405a51bc: lea rcx, [rbx + 0xb00]
1405a51c3: call 0x140867700
1405a51c8: mov byte ptr [rbx + 0xb3], 0
1405a51cf: lea rdx, [rbx + 0xb00]
1405a51d6: mov rcx, rsi
1405a51d9: call 0x1401237f0
1405a51de: mov byte ptr [rbx + 0xb3], 0
1405a51e5: mov rcx, rax
1405a51e8: call 0x140115fa0
1405a51ed: mov qword ptr [rbx + 0xb00], 0
1405a51f8: mov qword ptr [rbx + 0xb08], 4
1405a5203: mov qword ptr [rbx + 0xb10], 0
1405a520e: mov byte ptr [rbx + 0xb3], 0
1405a5215: lea rdx, [rbx + 0xb00]
1405a521c: mov rcx, rax
1405a521f: call 0x140137f20
1405a5224: mov rsi, rax
1405a5227: mov byte ptr [rbx + 0xb3], 0
1405a522e: lea rcx, [rbx + 0xb00]
1405a5235: call 0x1408ae7f0
1405a523a: mov byte ptr [rbx + 0xb3], 0
1405a5241: lea rdx, [rbx + 0xb00]
1405a5248: mov rcx, rsi
1405a524b: call 0x140122c30
1405a5250: mov qword ptr [rbx + 0xb00], 0
1405a525b: mov qword ptr [rbx + 0xb10], 0
1405a5266: mov dword ptr [rbx + 0xb28], 0x3b9aca00
1405a5270: mov byte ptr [rbx + 0xb30], 0
1405a5277: mov byte ptr [rbx + 0xb3], 0
1405a527e: lea rdx, [rbx + 0xb00]
1405a5285: mov rcx, rax
1405a5288: call 0x14011e1d0
1405a528d: mov qword ptr [rbx + 0x128], rax
1405a5294: mov byte ptr [rbx + 0xb3], 0
1405a529b: call 0x14093fa60
1405a52a0: mov dword ptr [rbx + 0x150], eax
1405a52a6: movss dword ptr [rbx + 0x26c], xmm0
1405a52ae: mov byte ptr [rbx + 0xb3], 0
1405a52b5: call 0x14093fa60
1405a52ba: mov dword ptr [rbx + 0x268], eax
1405a52c0: movss dword ptr [rbx + 0x264], xmm0
1405a52c8: mov byte ptr [rbx + 0xb3], 0
1405a52cf: call 0x14093fa60
1405a52d4: mov dword ptr [rbx + 0x260], eax
1405a52da: movss dword ptr [rbx + 0x25c], xmm0
1405a52e2: mov byte ptr [rbx + 0xb3], 0
1405a52e9: call 0x14093fa60
1405a52ee: mov dword ptr [rbx + 0x258], eax
1405a52f4: movss dword ptr [rbx + 0x254], xmm0
1405a52fc: mov byte ptr [rbx + 0xb3], 0
1405a5303: call 0x14093fa60
1405a5308: mov dword ptr [rbx + 0x250], eax
1405a530e: movss dword ptr [rbx + 0x24c], xmm0
1405a5316: mov byte ptr [rbx + 0xb3], 0
1405a531d: call 0x14093fa60
1405a5322: mov dword ptr [rbx + 0x248], eax
1405a5328: movss dword ptr [rbx + 0x244], xmm0
1405a5330: mov byte ptr [rbx + 0xb3], 0
1405a5337: call 0x14093fa60
1405a533c: mov dword ptr [rbx + 0x240], eax
1405a5342: movss dword ptr [rbx + 0x23c], xmm0
1405a534a: mov byte ptr [rbx + 0xb3], 0
1405a5351: call 0x14093fa60
1405a5356: mov dword ptr [rbx + 0x238], eax
1405a535c: movss dword ptr [rbx + 0x234], xmm0
1405a5364: mov byte ptr [rbx + 0xb3], 0
1405a536b: call 0x14093fa60
1405a5370: mov dword ptr [rbx + 0x230], eax
1405a5376: movss dword ptr [rbx + 0x22c], xmm0
1405a537e: mov byte ptr [rbx + 0xb3], 0
1405a5385: call 0x14093fa60
1405a538a: mov dword ptr [rbx + 0x228], eax
1405a5390: movss dword ptr [rbx + 0x224], xmm0
1405a5398: mov byte ptr [rbx + 0xb3], 0
1405a539f: call 0x14093fa60
1405a53a4: mov dword ptr [rbx + 0x220], eax
1405a53aa: movss dword ptr [rbx + 0x21c], xmm0
1405a53b2: mov byte ptr [rbx + 0xb3], 0
1405a53b9: call 0x14093fa60
1405a53be: mov dword ptr [rbx + 0x218], eax
1405a53c4: movss dword ptr [rbx + 0x214], xmm0
1405a53cc: mov byte ptr [rbx + 0xb3], 0
1405a53d3: call 0x14093fa60
1405a53d8: mov dword ptr [rbx + 0x210], eax
1405a53de: movss dword ptr [rbx + 0x20c], xmm0
1405a53e6: mov byte ptr [rbx + 0xb3], 0
1405a53ed: call 0x14093fa60
1405a53f2: mov dword ptr [rbx + 0x208], eax
1405a53f8: movss dword ptr [rbx + 0x204], xmm0
1405a5400: mov byte ptr [rbx + 0xb3], 0
1405a5407: call 0x14093fa60
1405a540c: mov dword ptr [rbx + 0x200], eax
1405a5412: movss dword ptr [rbx + 0x1fc], xmm0
1405a541a: mov byte ptr [rbx + 0xb3], 0
1405a5421: call 0x14093fa60
1405a5426: mov dword ptr [rbx + 0x1f8], eax
1405a542c: movss dword ptr [rbx + 0x1f4], xmm0
1405a5434: mov byte ptr [rbx + 0xb3], 0
1405a543b: call 0x14093fa60
1405a5440: mov dword ptr [rbx + 0x1f0], eax
1405a5446: movss dword ptr [rbx + 0x1ec], xmm0
1405a544e: mov byte ptr [rbx + 0xb3], 0
1405a5455: call 0x14093fa60
1405a545a: mov dword ptr [rbx + 0x1e8], eax
1405a5460: movss dword ptr [rbx + 0x1e4], xmm0
1405a5468: mov byte ptr [rbx + 0xb3], 0
1405a546f: call 0x14093fa60
1405a5474: mov dword ptr [rbx + 0x1e0], eax
1405a547a: movss dword ptr [rbx + 0x1dc], xmm0
1405a5482: mov byte ptr [rbx + 0xb3], 0
1405a5489: call 0x14093fa60
1405a548e: mov dword ptr [rbx + 0x1d8], eax
1405a5494: movss dword ptr [rbx + 0x1d4], xmm0
1405a549c: mov byte ptr [rbx + 0xb3], 0
1405a54a3: call 0x14093fa60
1405a54a8: mov dword ptr [rbx + 0x1d0], eax
1405a54ae: movss dword ptr [rbx + 0x1cc], xmm0
1405a54b6: mov byte ptr [rbx + 0xb3], 0
1405a54bd: call 0x14093fa60
1405a54c2: mov dword ptr [rbx + 0x1c8], eax
1405a54c8: movss dword ptr [rbx + 0x1c4], xmm0
1405a54d0: mov byte ptr [rbx + 0xb3], 0
1405a54d7: call 0x14093fa60
1405a54dc: mov dword ptr [rbx + 0x1c0], eax
1405a54e2: movss dword ptr [rbx + 0x1bc], xmm0
1405a54ea: mov byte ptr [rbx + 0xb3], 0
1405a54f1: call 0x14093fa60
1405a54f6: mov dword ptr [rbx + 0x1b8], eax
1405a54fc: movss dword ptr [rbx + 0x1b4], xmm0
1405a5504: mov byte ptr [rbx + 0xb3], 0
1405a550b: call 0x14093fa60
1405a5510: mov dword ptr [rbx + 0x1b0], eax
1405a5516: movss dword ptr [rbx + 0x1ac], xmm0
1405a551e: mov byte ptr [rbx + 0xb3], 0
1405a5525: call 0x14093fa60
1405a552a: mov dword ptr [rbx + 0x1a8], eax
1405a5530: movss dword ptr [rbx + 0x1a4], xmm0
1405a5538: mov byte ptr [rbx + 0xb3], 0
1405a553f: call 0x14093fa60
1405a5544: mov dword ptr [rbx + 0x1a0], eax
1405a554a: movss dword ptr [rbx + 0x19c], xmm0
1405a5552: mov byte ptr [rbx + 0xb3], 0
1405a5559: call 0x14093fa60
1405a555e: mov dword ptr [rbx + 0x198], eax
1405a5564: movss dword ptr [rbx + 0x194], xmm0
1405a556c: mov byte ptr [rbx + 0xb3], 0
1405a5573: call 0x14093fa60
1405a5578: mov dword ptr [rbx + 0x190], eax
1405a557e: movss dword ptr [rbx + 0x18c], xmm0
1405a5586: mov byte ptr [rbx + 0xb3], 0
1405a558d: call 0x14093fa60
1405a5592: mov dword ptr [rbx + 0x188], eax
1405a5598: movaps xmm15, xmm0
1405a559c: mov byte ptr [rbx + 0xb3], 0
1405a55a3: call 0x14093fa60
1405a55a8: mov dword ptr [rbx + 0x184], eax
1405a55ae: movaps xmm6, xmm0
1405a55b1: mov byte ptr [rbx + 0xb3], 0
1405a55b8: call 0x14093fa60
1405a55bd: mov dword ptr [rbx + 0x180], eax
1405a55c3: movaps xmm7, xmm0
1405a55c6: mov byte ptr [rbx + 0xb3], 0
1405a55cd: call 0x14093fa60
1405a55d2: mov dword ptr [rbx + 0x17c], eax
1405a55d8: movaps xmm8, xmm0
1405a55dc: mov byte ptr [rbx + 0xb3], 0
1405a55e3: call 0x14093fa60
1405a55e8: mov r15d, eax
1405a55eb: movaps xmm9, xmm0
1405a55ef: mov byte ptr [rbx + 0xb3], 0
1405a55f6: call 0x14093fa60
1405a55fb: mov r12d, eax
1405a55fe: movaps xmm10, xmm0
1405a5602: mov byte ptr [rbx + 0xb3], 0
1405a5609: call 0x14093fa60
1405a560e: mov esi, eax
1405a5610: movaps xmm11, xmm0
1405a5614: mov byte ptr [rbx + 0xb3], 0
1405a561b: call 0x14093fa60
1405a5620: mov r13d, eax
1405a5623: movaps xmm12, xmm0
1405a5627: mov byte ptr [rbx + 0xb3], 0
1405a562e: call 0x14093fa60
1405a5633: mov edi, eax
1405a5635: movaps xmm13, xmm0
1405a5639: mov byte ptr [rbx + 0xb3], 0
1405a5640: call 0x14093fa60
1405a5645: mov r14d, eax
1405a5648: movaps xmm14, xmm0
1405a564c: mov byte ptr [rbx + 0xb3], 0
1405a5653: call 0x14093fa60
1405a5658: xorps xmm1, xmm1
1405a565b: movups xmmword ptr [rbx + 0xc60], xmm1
1405a5662: movups xmmword ptr [rbx + 0xc50], xmm1
1405a5669: movups xmmword ptr [rbx + 0xc40], xmm1
1405a5670: mov ecx, dword ptr [rbx + 0x150]
1405a5676: mov dword ptr [rbx + 0xb00], ecx
1405a567c: movss xmm1, dword ptr [rbx + 0x26c]
1405a5684: movss dword ptr [rbx + 0xb04], xmm1
1405a568c: mov ecx, dword ptr [rbx + 0x268]
1405a5692: mov dword ptr [rbx + 0xb08], ecx
1405a5698: movss xmm1, dword ptr [rbx + 0x264]
1405a56a0: movss dword ptr [rbx + 0xb0c], xmm1
1405a56a8: mov ecx, dword ptr [rbx + 0x260]
1405a56ae: mov dword ptr [rbx + 0xb10], ecx
1405a56b4: movss xmm1, dword ptr [rbx + 0x25c]
1405a56bc: movss dword ptr [rbx + 0xb14], xmm1
1405a56c4: mov ecx, dword ptr [rbx + 0x258]
1405a56ca: mov dword ptr [rbx + 0xb18], ecx
1405a56d0: movss xmm1, dword ptr [rbx + 0x254]
1405a56d8: movss dword ptr [rbx + 0xb1c], xmm1
1405a56e0: mov ecx, dword ptr [rbx + 0x250]
1405a56e6: mov dword ptr [rbx + 0xb20], ecx
1405a56ec: movss xmm1, dword ptr [rbx + 0x24c]
1405a56f4: movss dword ptr [rbx + 0xb24], xmm1
1405a56fc: mov ecx, dword ptr [rbx + 0x248]
1405a5702: mov dword ptr [rbx + 0xb28], ecx
1405a5708: movss xmm1, dword ptr [rbx + 0x244]
1405a5710: movss dword ptr [rbx + 0xb2c], xmm1
1405a5718: mov ecx, dword ptr [rbx + 0x240]
1405a571e: mov dword ptr [rbx + 0xb30], ecx
1405a5724: movss xmm1, dword ptr [rbx + 0x23c]
1405a572c: movss dword ptr [rbx + 0xb34], xmm1
1405a5734: mov ecx, dword ptr [rbx + 0x238]
1405a573a: mov dword ptr [rbx + 0xb38], ecx
1405a5740: movss xmm1, dword ptr [rbx + 0x234]
1405a5748: movss dword ptr [rbx + 0xb3c], xmm1
1405a5750: mov ecx, dword ptr [rbx + 0x230]
1405a5756: mov dword ptr [rbx + 0xb40], ecx
1405a575c: movss xmm1, dword ptr [rbx + 0x22c]
1405a5764: movss dword ptr [rbx + 0xb44], xmm1
1405a576c: mov ecx, dword ptr [rbx + 0x228]
1405a5772: mov dword ptr [rbx + 0xb48], ecx
1405a5778: movss xmm1, dword ptr [rbx + 0x224]
1405a5780: movss dword ptr [rbx + 0xb4c], xmm1
1405a5788: mov ecx, dword ptr [rbx + 0x220]
1405a578e: mov dword ptr [rbx + 0xb50], ecx
1405a5794: movss xmm1, dword ptr [rbx + 0x21c]
1405a579c: movss dword ptr [rbx + 0xb54], xmm1
1405a57a4: mov ecx, dword ptr [rbx + 0x218]
1405a57aa: mov dword ptr [rbx + 0xb58], ecx
1405a57b0: movss xmm1, dword ptr [rbx + 0x214]
1405a57b8: movss dword ptr [rbx + 0xb5c], xmm1
1405a57c0: mov ecx, dword ptr [rbx + 0x210]
1405a57c6: mov dword ptr [rbx + 0xb60], ecx
1405a57cc: movss xmm1, dword ptr [rbx + 0x20c]
1405a57d4: movss dword ptr [rbx + 0xb64], xmm1
1405a57dc: mov ecx, dword ptr [rbx + 0x208]
1405a57e2: mov dword ptr [rbx + 0xb68], ecx
1405a57e8: movss xmm1, dword ptr [rbx + 0x204]
1405a57f0: movss dword ptr [rbx + 0xb6c], xmm1
1405a57f8: mov ecx, dword ptr [rbx + 0x200]
1405a57fe: mov dword ptr [rbx + 0xb70], ecx
1405a5804: movss xmm1, dword ptr [rbx + 0x1fc]
1405a580c: movss dword ptr [rbx + 0xb74], xmm1
1405a5814: mov ecx, dword ptr [rbx + 0x1f8]
1405a581a: mov dword ptr [rbx + 0xb78], ecx
1405a5820: movss xmm1, dword ptr [rbx + 0x1f4]
1405a5828: movss dword ptr [rbx + 0xb7c], xmm1
1405a5830: mov ecx, dword ptr [rbx + 0x1f0]
1405a5836: mov dword ptr [rbx + 0xb80], ecx
1405a583c: movss xmm1, dword ptr [rbx + 0x1ec]
1405a5844: movss dword ptr [rbx + 0xb84], xmm1
1405a584c: mov ecx, dword ptr [rbx + 0x1e8]
1405a5852: mov dword ptr [rbx + 0xb88], ecx
1405a5858: movss xmm1, dword ptr [rbx + 0x1e4]
1405a5860: movss dword ptr [rbx + 0xb8c], xmm1
1405a5868: mov ecx, dword ptr [rbx + 0x1e0]
1405a586e: mov dword ptr [rbx + 0xb90], ecx
1405a5874: movss xmm1, dword ptr [rbx + 0x1dc]
1405a587c: movss dword ptr [rbx + 0xb94], xmm1
1405a5884: mov ecx, dword ptr [rbx + 0x1d8]
1405a588a: mov dword ptr [rbx + 0xb98], ecx
1405a5890: movss xmm1, dword ptr [rbx + 0x1d4]
1405a5898: movss dword ptr [rbx + 0xb9c], xmm1
1405a58a0: mov ecx, dword ptr [rbx + 0x1d0]
1405a58a6: mov dword ptr [rbx + 0xba0], ecx
1405a58ac: movss xmm1, dword ptr [rbx + 0x1cc]
1405a58b4: movss dword ptr [rbx + 0xba4], xmm1
1405a58bc: mov ecx, dword ptr [rbx + 0x1c8]
1405a58c2: mov dword ptr [rbx + 0xba8], ecx
1405a58c8: movss xmm1, dword ptr [rbx + 0x1c4]
1405a58d0: movss dword ptr [rbx + 0xbac], xmm1
1405a58d8: mov ecx, dword ptr [rbx + 0x1c0]
1405a58de: mov dword ptr [rbx + 0xbb0], ecx
1405a58e4: movss xmm1, dword ptr [rbx + 0x1bc]
1405a58ec: movss dword ptr [rbx + 0xbb4], xmm1
1405a58f4: mov ecx, dword ptr [rbx + 0x1b8]
1405a58fa: mov dword ptr [rbx + 0xbb8], ecx
1405a5900: movss xmm1, dword ptr [rbx + 0x1b4]
1405a5908: movss dword ptr [rbx + 0xbbc], xmm1
1405a5910: mov ecx, dword ptr [rbx + 0x1b0]
1405a5916: mov dword ptr [rbx + 0xbc0], ecx
1405a591c: movss xmm1, dword ptr [rbx + 0x1ac]
1405a5924: movss dword ptr [rbx + 0xbc4], xmm1
1405a592c: mov ecx, dword ptr [rbx + 0x1a8]
1405a5932: mov dword ptr [rbx + 0xbc8], ecx
1405a5938: movss xmm1, dword ptr [rbx + 0x1a4]
1405a5940: movss dword ptr [rbx + 0xbcc], xmm1
1405a5948: mov ecx, dword ptr [rbx + 0x1a0]
1405a594e: mov dword ptr [rbx + 0xbd0], ecx
1405a5954: movss xmm1, dword ptr [rbx + 0x19c]
1405a595c: movss dword ptr [rbx + 0xbd4], xmm1
1405a5964: mov ecx, dword ptr [rbx + 0x198]
1405a596a: mov dword ptr [rbx + 0xbd8], ecx
1405a5970: movss xmm1, dword ptr [rbx + 0x194]
1405a5978: movss dword ptr [rbx + 0xbdc], xmm1
1405a5980: mov ecx, dword ptr [rbx + 0x190]
1405a5986: mov dword ptr [rbx + 0xbe0], ecx
1405a598c: movss xmm1, dword ptr [rbx + 0x18c]
1405a5994: movss dword ptr [rbx + 0xbe4], xmm1
1405a599c: mov ecx, dword ptr [rbx + 0x188]
1405a59a2: mov dword ptr [rbx + 0xbe8], ecx
1405a59a8: movss dword ptr [rbx + 0xbec], xmm15
1405a59b1: mov ecx, dword ptr [rbx + 0x184]
1405a59b7: mov dword ptr [rbx + 0xbf0], ecx
1405a59bd: movss dword ptr [rbx + 0xbf4], xmm6
1405a59c5: mov ecx, dword ptr [rbx + 0x180]
1405a59cb: mov dword ptr [rbx + 0xbf8], ecx
1405a59d1: movss dword ptr [rbx + 0xbfc], xmm7
1405a59d9: mov ecx, dword ptr [rbx + 0x17c]
1405a59df: mov dword ptr [rbx + 0xc00], ecx
1405a59e5: movss dword ptr [rbx + 0xc04], xmm8
1405a59ee: mov dword ptr [rbx + 0xc08], r15d
1405a59f5: movss dword ptr [rbx + 0xc0c], xmm9
1405a59fe: mov dword ptr [rbx + 0xc10], r12d
1405a5a05: movss dword ptr [rbx + 0xc14], xmm10
1405a5a0e: mov dword ptr [rbx + 0xc18], esi
1405a5a14: movss dword ptr [rbx + 0xc1c], xmm11
1405a5a1d: mov dword ptr [rbx + 0xc20], r13d
1405a5a24: movss dword ptr [rbx + 0xc24], xmm12
1405a5a2d: mov dword ptr [rbx + 0xc28], edi
1405a5a33: movss dword ptr [rbx + 0xc2c], xmm13
1405a5a3c: mov dword ptr [rbx + 0xc30], r14d
1405a5a43: movss dword ptr [rbx + 0xc34], xmm14
1405a5a4c: mov dword ptr [rbx + 0xc38], eax
1405a5a52: movss dword ptr [rbx + 0xc3c], xmm0
1405a5a5a: mov byte ptr [rbx + 0xb3], 0
1405a5a61: lea rdx, [rbx + 0xb00]
1405a5a68: mov rcx, qword ptr [rbx + 0x128]
1405a5a6f: call 0x1401278f0
1405a5a74: mov rsi, rax
1405a5a77: mov byte ptr [rbx + 0xb3], 0
1405a5a7e: movabs rdx, 0x2b39980e7de6bf8a
1405a5a88: movabs r8, 0x52d9aabfd20d56c5
1405a5a92: mov rcx, qword ptr [rbx + 0xe8]
1405a5a99: call 0x1400dd6f0
1405a5a9e: test rax, rax
1405a5aa1: mov r8, qword ptr [rbx + 0xc8]
1405a5aa8: je 0x1405a6995
1405a5aae: cmp qword ptr [r8 + 0x218], rdx
1405a5ab5: jbe 0x1405a6995
1405a5abb: mov rax, qword ptr [r8 + 0x210]
1405a5ac2: mov rcx, qword ptr [rax + rdx*8]
1405a5ac6: test rcx, rcx
1405a5ac9: je 0x1405a6995
1405a5acf: mov rax, qword ptr [r8 + 0x1e0]
1405a5ad6: not rcx
1405a5ad9: lea rcx, [rcx + rcx*2]
1405a5add: shl rcx, 5
1405a5ae1: cmp qword ptr [rax + rcx + 0x20], 0
1405a5ae7: je 0x1405a6995
1405a5aed: add rax, rcx
1405a5af0: mov rax, qword ptr [rax + 0x10]
1405a5af4: mov rcx, qword ptr [rax]
1405a5af7: mov edx, dword ptr [rax + 8]
1405a5afa: mov r8, qword ptr [rax + 0x10]
1405a5afe: mov eax, dword ptr [rax + 0x18]
1405a5b01: mov qword ptr [rbx + 0xb00], rcx
1405a5b08: mov dword ptr [rbx + 0xb08], edx
1405a5b0e: mov qword ptr [rbx + 0xb10], r8
1405a5b15: mov dword ptr [rbx + 0xb18], eax
1405a5b1b: mov byte ptr [rbx + 0xb3], 0
1405a5b22: lea rdx, [rbx + 0xb00]
1405a5b29: mov rcx, rsi
1405a5b2c: call 0x14012b880
1405a5b31: mov rsi, rax
1405a5b34: mov byte ptr [rbx + 0xb3], 0
1405a5b3b: lea rcx, [rbx + 0xb00]
1405a5b42: call 0x1405b7290
1405a5b47: movups xmm0, xmmword ptr [rbx + 0xb60]
1405a5b4e: movaps xmmword ptr [rbx + 0x25f0], xmm0
1405a5b55: movups xmm0, xmmword ptr [rbx + 0xb50]
1405a5b5c: movaps xmmword ptr [rbx + 0x25e0], xmm0
1405a5b63: movups xmm0, xmmword ptr [rbx + 0xb40]
1405a5b6a: movaps xmmword ptr [rbx + 0x25d0], xmm0
1405a5b71: movups xmm0, xmmword ptr [rbx + 0xb00]
1405a5b78: movups xmm1, xmmword ptr [rbx + 0xb10]
1405a5b7f: movups xmm2, xmmword ptr [rbx + 0xb20]
1405a5b86: movups xmm3, xmmword ptr [rbx + 0xb30]
1405a5b8d: movaps xmmword ptr [rbx + 0x25c0], xmm3
1405a5b94: movaps xmmword ptr [rbx + 0x25b0], xmm2
1405a5b9b: movaps xmmword ptr [rbx + 0x25a0], xmm1
1405a5ba2: movaps xmmword ptr [rbx + 0x2590], xmm0
1405a5ba9: mov byte ptr [rbx + 0xb3], 0
1405a5bb0: lea rdx, [rbx + 0x2590]
1405a5bb7: mov rcx, rsi
1405a5bba: call 0x140326890
1405a5bbf: lea rcx, [rbx + 0x2590]
1405a5bc6: lea rdx, [rbx + 0x440]
1405a5bcd: mov r8d, 0x658
1405a5bd3: call 0x142923c00
1405a5bd8: mov byte ptr [rbx + 0x112], 1
1405a5bdf: mov byte ptr [rbx + 0x113], 1
1405a5be6: mov byte ptr [rbx + 0x114], 1
1405a5bed: lea rcx, [rbx + 0xb00]
1405a5bf4: call 0x140ca5fd0
1405a5bf9: lea rcx, [rbx + 0x2bf0]
1405a5c00: mov r8d, 0x628
1405a5c06: mov rsi, qword ptr [rbx + 0xc8]
1405a5c0d: mov rdx, rsi
1405a5c10: call 0x142923c00
1405a5c15: lea rdx, [rbx + 0xb00]
1405a5c1c: mov r8d, 0x628
1405a5c22: mov rcx, rsi
1405a5c25: call 0x142923c00
1405a5c2a: mov rax, qword ptr [rbx + 0x138]
1405a5c31: lock inc qword ptr [rax]
1405a5c35: movabs rcx, 0x8000000000000000
1405a5c3f: jle 0x1405a7040
1405a5c45: mov qword ptr [rbx + 0x1ff0], rax
1405a5c4c: mov qword ptr [rbx + 0xb10], rcx
1405a5c53: mov qword ptr [rbx + 0xb00], 0
1405a5c5e: mov byte ptr [rbx + 0xb28], 0
1405a5c65: movzx eax, byte ptr [rip + 0x308ef9d]
1405a5c6c: mov ecx, 0xa
1405a5c71: mov edx, 1
1405a5c76: call 0x140613c10
1405a5c7b: test rax, rax
1405a5c7e: mov qword ptr [rbx + 0xe8], rax
1405a5c85: je 0x1405a6fe5
1405a5c8b: movabs rcx, 0x6f6c20636e797361
1405a5c95: mov qword ptr [rax], rcx
1405a5c98: mov word ptr [rax + 8], 0x6461
1405a5c9e: mov qword ptr [rbx + 0x2230], 0xa
1405a5ca9: mov qword ptr [rbx + 0x2238], rax
1405a5cb0: mov qword ptr [rbx + 0x2240], 0xa
1405a5cbb: lea rcx, [rbx + 0x440]
1405a5cc2: lea rdx, [rbx + 0xb00]
1405a5cc9: lea r8, [rbx + 0x2230]
1405a5cd0: call 0x140d558f0
1405a5cd5: mov r13b, byte ptr [rbx + 0x107]
1405a5cdc: mov rax, qword ptr [rbx + 0x1ff0]
1405a5ce3: mov qword ptr [rbx + 0x1138], rax
1405a5cea: lea rcx, [rbx + 0xb10]
1405a5cf1: lea rdx, [rbx + 0x2bf0]
1405a5cf8: mov r8d, 0x628
1405a5cfe: call 0x142923c00
1405a5d03: movups xmm0, xmmword ptr [rbx + 0x2e0]
1405a5d0a: movups xmmword ptr [rbx + 0xb00], xmm0
1405a5d11: mov byte ptr [rbx + 0xdb], 0
1405a5d18: lea rsi, [rbx + 0x2230]
1405a5d1f: lea rdx, [rbx + 0x440]
1405a5d26: lea r8, [rbx + 0xb00]
1405a5d2d: mov rcx, rsi
1405a5d30: call 0x14028a680
1405a5d35: mov rdi, qword ptr [rbx + 0x2230]
1405a5d3c: test rdi, rdi
1405a5d3f: je 0x1405a5d86
1405a5d41: mov rcx, qword ptr [rbx + 0x2240]
1405a5d48: call qword ptr [rip + 0x2bec6a2]
1405a5d4e: lock dec qword ptr [rdi]
1405a5d52: jne 0x1405a5d60
1405a5d54: lea rcx, [rbx + 0x2230]
1405a5d5b: call 0x140d53b60
1405a5d60: mov rax, qword ptr [rbx + 0x2238]
1405a5d67: lock dec qword ptr [rax]
1405a5d6b: jne 0x1405a5e00
1405a5d71: lea rcx, [rbx + 0x2238]
1405a5d78: mov byte ptr [rbx + 0xdb], 0
1405a5d7f: call 0x140d53db0
1405a5d84: jmp 0x1405a5e00
1405a5d86: mov rax, qword ptr [rbx + 0x2238]
1405a5d8d: mov ecx, eax
1405a5d8f: and ecx, 3
1405a5d92: cmp ecx, 1
1405a5d95: jne 0x1405a5e00
1405a5d97: lea rcx, [rax - 1]
1405a5d9b: mov qword ptr [rbx + 0x150], rcx
1405a5da2: mov rcx, qword ptr [rax - 1]
1405a5da6: mov qword ptr [rbx + 0xe8], rcx
1405a5dad: mov rax, qword ptr [rax + 7]
1405a5db1: mov qword ptr [rbx + 0x128], rax
1405a5db8: mov rax, qword ptr [rax]
1405a5dbb: test rax, rax
1405a5dbe: je 0x1405a5dc9
1405a5dc0: mov rcx, qword ptr [rbx + 0xe8]
1405a5dc7: call rax
1405a5dc9: mov rcx, qword ptr [rbx + 0xe8]
1405a5dd0: mov rax, qword ptr [rbx + 0x128]
1405a5dd7: mov rdx, qword ptr [rax + 8]
1405a5ddb: test rdx, rdx
1405a5dde: je 0x1405a5de9
1405a5de0: mov r8, qword ptr [rax + 0x10]
1405a5de4: call 0x140613c20
1405a5de9: mov edx, 0x18
1405a5dee: mov r8d, 8
1405a5df4: mov rcx, qword ptr [rbx + 0x150]
1405a5dfb: call 0x140613c20
1405a5e00: lea rcx, [rbx + 0x440]
1405a5e07: lea rdx, [rbx + 0x2590]
1405a5e0e: mov r8d, 0x658
1405a5e14: call 0x142923c00
1405a5e19: movups xmm0, xmmword ptr [rbx + 0x2f0]
1405a5e20: movups xmmword ptr [rbx + 0x340], xmm0
1405a5e27: movups xmm0, xmmword ptr [rbx + 0x328]
1405a5e2e: movups xmmword ptr [rbx + 0x270], xmm0
1405a5e35: movups xmm0, xmmword ptr [rbx + 0x160]
1405a5e3c: movups xmmword ptr [rbx + 0x280], xmm0
1405a5e43: mov rax, qword ptr [rbx + 0x140]
1405a5e4a: mov qword ptr [rbx + 0x170], rax
1405a5e51: lea rdi, [rbx + 0x24e8]
1405a5e58: lea rdx, [rbx + 0x2188]
1405a5e5f: mov r8d, 0xa8
1405a5e65: mov rcx, rdi
1405a5e68: call 0x142923c00
1405a5e6d: mov rdx, qword ptr [rbx + 0x138]
1405a5e74: add rdx, 0x10
1405a5e78: mov byte ptr [rbx + 0x105], 1
1405a5e7f: lea rcx, [rbx + 0x370]
1405a5e86: call 0x140bea2d0
1405a5e8b: lea rcx, [rbx + 0x1ff0]
1405a5e92: call 0x140be57d0
1405a5e97: lea rcx, [rbx + 0xb00]
1405a5e9e: call 0x140be57d0
1405a5ea3: lea r14, [rbx + 0x370]
1405a5eaa: mov r8d, 0xd0
1405a5eb0: mov rcx, rsi
1405a5eb3: mov rdx, r14
1405a5eb6: call 0x142923c00
1405a5ebb: lea rcx, [rbx + 0x2300]
1405a5ec2: lea r15, [rbx + 0x1ff0]
1405a5ec9: mov r8d, 0xf0
1405a5ecf: mov rdx, r15
1405a5ed2: call 0x142923c00
1405a5ed7: lea rcx, [rbx + 0x23f0]
1405a5ede: lea rdx, [rbx + 0xb00]
1405a5ee5: mov r8d, 0xf0
1405a5eeb: call 0x142923c00
1405a5ef0: movaps xmm0, xmmword ptr [rbx + 0x350]
1405a5ef7: movaps xmm1, xmmword ptr [rbx + 0x360]
1405a5efe: movaps xmmword ptr [rbx + 0x380], xmm1
1405a5f05: movaps xmmword ptr [rbx + 0x370], xmm0
1405a5f0c: mov r12, qword ptr [rbp + 0x37a8]
1405a5f13: lea rdx, [rbx + 0x20e0]
1405a5f1a: mov r8d, 0xa8
1405a5f20: mov rcx, r15
1405a5f23: call 0x142923c00
1405a5f28: mov r8, qword ptr [rbx + 0x340]
1405a5f2f: mov r9, qword ptr [rbx + 0x348]
1405a5f36: movups xmm0, xmmword ptr [rbx + 0x270]
1405a5f3d: movups xmm1, xmmword ptr [rbx + 0x280]
1405a5f44: mov rax, qword ptr [rbx + 0x170]
1405a5f4b: mov byte ptr [rbx + 0x105], 0
1405a5f52: mov qword ptr [rsp + 0x68], r15
1405a5f57: mov qword ptr [rsp + 0x60], r12
1405a5f5c: mov qword ptr [rsp + 0x58], r14
1405a5f61: mov qword ptr [rsp + 0x50], rsi
1405a5f66: mov qword ptr [rsp + 0x48], rdi
1405a5f6b: mov qword ptr [rsp + 0x40], rax
1405a5f70: movups xmmword ptr [rsp + 0x30], xmm1
1405a5f75: movups xmmword ptr [rsp + 0x20], xmm0
1405a5f7a: lea rcx, [rbx + 0xb00]
1405a5f81: lea rdx, [rbx + 0x440]
1405a5f88: call 0x1404d8100
1405a5f8d: mov byte ptr [rbx + 0xf5], 0
1405a5f94: mov byte ptr [rbx + 0xf6], 0
1405a5f9b: mov byte ptr [rbx + 0xf7], 0
1405a5fa2: lea rcx, [rbx + 0xb00]
1405a5fa9: call 0x1404d8250
1405a5fae: mov al, 1
1405a5fb0: mov dword ptr [rbx + 0x128], eax
1405a5fb6: mov dword ptr [rbx + 0xe8], 0
1405a5fc0: jmp 0x1405a61bc
1405a5fc5: mov rcx, qword ptr [rbx + 0x168]
1405a5fcc: lock inc qword ptr [rcx + 0x70]
1405a5fd1: jg 0x1405a5ff1
1405a5fd3: mov ecx, 7
1405a5fd8: int 0x29
1405a5fda: ud2 
1405a5fdc: mov rcx, qword ptr [rbx + 0x168]
1405a5fe3: lock inc qword ptr [rcx + 0x200]
1405a5feb: jle 0x1405a7042
1405a5ff1: mov qword ptr [rbx + 0x370], rax
1405a5ff8: mov qword ptr [rbx + 0x378], rcx
1405a5fff: mov rax, qword ptr [rbx + 0x140]
1405a6006: lock inc qword ptr [rax]
1405a600a: jle 0x1405a7040
1405a6010: mov qword ptr [rbx + 0x270], rax
1405a6017: lea rsi, [rbx + 0x1ff0]
1405a601e: lea rdx, [rbx + 0x2188]
1405a6025: mov r8d, 0xa8
1405a602b: mov rcx, rsi
1405a602e: call 0x142923c00
1405a6033: mov rdx, qword ptr [rbx + 0x138]
1405a603a: add rdx, 0x10
1405a603e: mov byte ptr [rbx + 0x106], 1
1405a6045: lea rcx, [rbx + 0x2230]
1405a604c: call 0x140bea2d0
1405a6051: lea rcx, [rbx + 0x2bf0]
1405a6058: call 0x140be57d0
1405a605d: lea rcx, [rbx + 0xb00]
1405a6064: call 0x140be57d0
1405a6069: lea rdi, [rbx + 0x2230]
1405a6070: mov r8d, 0xd0
1405a6076: mov rcx, r14
1405a6079: mov rdx, rdi
1405a607c: call 0x142923c00
1405a6081: lea rcx, [rbx + 0x2660]
1405a6088: lea r15, [rbx + 0x2bf0]
1405a608f: mov r8d, 0xf0
1405a6095: mov rdx, r15
1405a6098: call 0x142923c00
1405a609d: lea rcx, [rbx + 0x2750]
1405a60a4: lea rdx, [rbx + 0xb00]
1405a60ab: mov r8d, 0xf0
1405a60b1: call 0x142923c00
1405a60b6: movaps xmm0, xmmword ptr [rbx + 0x350]
1405a60bd: movaps xmm1, xmmword ptr [rbx + 0x360]
1405a60c4: movaps xmmword ptr [rbx + 0x2240], xmm1
1405a60cb: movaps xmmword ptr [rbx + 0x2230], xmm0
1405a60d2: mov r12, qword ptr [rbp + 0x37a8]
1405a60d9: lea rdx, [rbx + 0x20e0]
1405a60e0: mov r8d, 0xa8
1405a60e6: mov rcx, r15
1405a60e9: call 0x142923c00
1405a60ee: mov r8, qword ptr [rbx + 0x280]
1405a60f5: mov r9, qword ptr [rbx + 0x288]
1405a60fc: movups xmm0, xmmword ptr [rbx + 0x24e8]
1405a6103: movups xmm1, xmmword ptr [rbx + 0x370]
1405a610a: mov rax, qword ptr [rbx + 0x270]
1405a6111: mov byte ptr [rbx + 0x106], 0
1405a6118: mov qword ptr [rsp + 0x68], r15
1405a611d: mov qword ptr [rsp + 0x60], r12
1405a6122: mov qword ptr [rsp + 0x58], rdi
1405a6127: mov qword ptr [rsp + 0x50], r14
1405a612c: mov qword ptr [rsp + 0x48], rsi
1405a6131: mov qword ptr [rsp + 0x40], rax
1405a6136: movups xmmword ptr [rsp + 0x30], xmm1
1405a613b: movups xmmword ptr [rsp + 0x20], xmm0
1405a6140: lea rcx, [rbx + 0xb00]
1405a6147: lea rdx, [rbx + 0x440]
1405a614e: call 0x1404d8100
1405a6153: mov byte ptr [rbx + 0xc2], 0
1405a615a: mov byte ptr [rbx + 0xc3], 0
1405a6161: mov byte ptr [rbx + 0xc4], 1
1405a6168: mov byte ptr [rbx + 0xc5], 1
1405a616f: mov byte ptr [rbx + 0xc6], 0
1405a6176: lea rcx, [rbx + 0xb00]
1405a617d: call 0x1404d8250
1405a6182: mov byte ptr [rbx + 0xe4], 0
1405a6189: mov byte ptr [rbx + 0xe5], 0
1405a6190: mov byte ptr [rbx + 0xe6], 1
1405a6197: mov byte ptr [rbx + 0xe7], 0
1405a619e: lea rcx, [rbx + 0x2e0]
1405a61a5: call 0x140601d60
1405a61aa: mov al, 1
1405a61ac: mov dword ptr [rbx + 0xe8], eax
1405a61b2: mov dword ptr [rbx + 0x128], 0
1405a61bc: mov rcx, qword ptr [rbx + 0x2a0]
1405a61c3: call qword ptr [rip + 0x2bec227]
1405a61c9: mov rax, qword ptr [rbx + 0x290]
1405a61d0: lock dec qword ptr [rax]
1405a61d4: jne 0x1405a61e2
1405a61d6: lea rcx, [rbx + 0x290]
1405a61dd: call 0x140d53b60
1405a61e2: mov rax, qword ptr [rbx + 0x298]
1405a61e9: lock dec qword ptr [rax]
1405a61ed: mov edi, dword ptr [rbx + 0xe8]
1405a61f3: mov r14d, dword ptr [rbx + 0x128]
1405a61fa: jne 0x1405a622b
1405a61fc: lea rcx, [rbx + 0x298]
1405a6203: mov byte ptr [rbx + 0xd4], 0
1405a620a: mov byte ptr [rbx + 0xd5], r14b
1405a6211: mov byte ptr [rbx + 0xd6], dil
1405a6218: mov byte ptr [rbx + 0xd7], 0
1405a621f: mov byte ptr [rbx + 0xd8], 0
1405a6226: call 0x140d53db0
1405a622b: mov rax, qword ptr [rbx + 0x320]
1405a6232: test rax, rax
1405a6235: mov esi, dword ptr [rbx + 0x130]
1405a623b: je 0x1405a628e
1405a623d: lock dec qword ptr [rax]
1405a6241: jne 0x1405a628e
1405a6243: mov byte ptr [rbx + 0xaa], r14b
1405a624a: mov byte ptr [rbx + 0xab], sil
1405a6251: mov byte ptr [rbx + 0xac], dil
1405a6258: mov byte ptr [rbx + 0xad], 0
1405a625f: mov byte ptr [rbx + 0xae], 0
1405a6266: mov byte ptr [rbx + 0xaf], 0
1405a626d: mov byte ptr [rbx + 0xb0], 0
1405a6274: mov byte ptr [rbx + 0xb1], 0
1405a627b: mov byte ptr [rbx + 0xb2], 0
1405a6282: lea rcx, [rbx + 0x320]
1405a6289: call 0x1406978e0
1405a628e: mov rax, qword ptr [rbx + 0x138]
1405a6295: lock dec qword ptr [rax]
1405a6299: jne 0x1405a62d1
1405a629b: mov byte ptr [rbx + 0xf8], r14b
1405a62a2: mov byte ptr [rbx + 0xf9], sil
1405a62a9: mov byte ptr [rbx + 0xfa], dil
1405a62b0: mov byte ptr [rbx + 0xfb], 0
1405a62b7: mov byte ptr [rbx + 0xfc], 0
1405a62be: mov byte ptr [rbx + 0xfd], 0
1405a62c5: lea rcx, [rbx + 0x138]
1405a62cc: call 0x140b5be90
1405a62d1: test r13b, r13b
1405a62d4: je 0x1405a6305
1405a62d6: mov byte ptr [rbx + 0x115], dil
1405a62dd: mov byte ptr [rbx + 0x116], r14b
1405a62e4: mov byte ptr [rbx + 0x117], sil
1405a62eb: mov byte ptr [rbx + 0x118], 0
1405a62f2: mov byte ptr [rbx + 0x119], 0
1405a62f9: lea rcx, [rbx + 0x160]
1405a6300: call 0x140606870
1405a6305: mov byte ptr [rbx + 0x120], dil
1405a630c: mov byte ptr [rbx + 0x121], r14b
1405a6313: mov byte ptr [rbx + 0x122], sil
1405a631a: mov byte ptr [rbx + 0x123], 0
1405a6321: mov rcx, qword ptr [rbp + 0x37c0]
1405a6328: call 0x1405ee980
1405a632d: mov byte ptr [rbx + 0x124], dil
1405a6334: mov byte ptr [rbx + 0x125], r14b
1405a633b: mov byte ptr [rbx + 0x126], sil
1405a6342: mov byte ptr [rbx + 0x127], 0
1405a6349: mov rcx, qword ptr [rbp + 0x37b8]
1405a6350: call 0x1405f18c0
1405a6355: test sil, sil
1405a6358: mov rcx, qword ptr [rbp + 0x37b0]
1405a635f: je 0x1405a639d
1405a6361: call 0x1405fdb80
1405a6366: mov rcx, qword ptr [rbp + 0x37b0]
1405a636d: mov rdx, qword ptr [rcx + 0xc8]
1405a6374: mov rax, rdx
1405a6377: neg rax
1405a637a: mov edi, dword ptr [rbx + 0xe8]
1405a6380: mov r14d, dword ptr [rbx + 0x128]
1405a6387: jo 0x1405a639d
1405a6389: jae 0x1405a639d
1405a638b: mov rcx, qword ptr [rcx + 0xd0]
1405a6392: mov r8d, 1
1405a6398: call 0x140613c20
1405a639d: test r14b, r14b
1405a63a0: je 0x1405a63b5
1405a63a2: mov byte ptr [rbx + 0x11d], dil
1405a63a9: mov rcx, qword ptr [rbx + 0xc8]
1405a63b0: call 0x1405e6cd0
1405a63b5: test r13b, r13b
1405a63b8: je 0x1405a63d3
1405a63ba: mov rax, qword ptr [rbx + 0x140]
1405a63c1: lock dec qword ptr [rax]
1405a63c5: jne 0x1405a63d3
1405a63c7: lea rcx, [rbx + 0x140]
1405a63ce: call 0x140631840
1405a63d3: lea rcx, [rbx + 0x2c0]
1405a63da: call 0x1405d3280
1405a63df: movaps xmm6, xmmword ptr [rbp + 0x3690]
1405a63e6: movaps xmm7, xmmword ptr [rbp + 0x36a0]
1405a63ed: movaps xmm8, xmmword ptr [rbp + 0x36b0]
1405a63f5: movaps xmm9, xmmword ptr [rbp + 0x36c0]
1405a63fd: movaps xmm10, xmmword ptr [rbp + 0x36d0]
1405a6405: movaps xmm11, xmmword ptr [rbp + 0x36e0]
1405a640d: movaps xmm12, xmmword ptr [rbp + 0x36f0]
1405a6415: movaps xmm13, xmmword ptr [rbp + 0x3700]
1405a641d: movaps xmm14, xmmword ptr [rbp + 0x3710]
1405a6425: movaps xmm15, xmmword ptr [rbp + 0x3720]
1405a642d: lea rsp, [rbp + 0x3738]
1405a6434: pop rbx
1405a6435: pop rdi
1405a6436: pop rsi
1405a6437: pop r12
1405a6439: pop r13
1405a643b: pop r14
1405a643d: pop r15
1405a643f: pop rbp
1405a6440: ret 
1405a6441: movaps xmm0, xmmword ptr [rbx + 0x1ff0]
1405a6448: movaps xmmword ptr [rbx + 0x440], xmm0
1405a644f: movaps xmm0, xmmword ptr [rbx + 0x2bf0]
1405a6456: movaps xmm1, xmmword ptr [rbx + 0x2c00]
1405a645d: movaps xmmword ptr [rbx + 0xb00], xmm0
1405a6464: movaps xmmword ptr [rbx + 0xb10], xmm1
1405a646b: dec r8
1405a646e: add rdx, r8
1405a6471: cmovae rdx, r8
1405a6475: mov qword ptr [rbx + 0x2240], rdx
1405a647c: mov qword ptr [rbx + 0x2248], rax
1405a6483: imul rax, rdx, 0x38
1405a6487: mov qword ptr [rcx + rax], r9
1405a648b: movaps xmm0, xmmword ptr [rbx + 0x440]
1405a6492: movups xmmword ptr [rcx + rax + 8], xmm0
1405a6497: movaps xmm0, xmmword ptr [rbx + 0xb00]
1405a649e: movaps xmm1, xmmword ptr [rbx + 0xb10]
1405a64a5: movups xmmword ptr [rcx + rax + 0x28], xmm1
1405a64aa: movups xmmword ptr [rcx + rax + 0x18], xmm0
1405a64af: movups xmm0, xmmword ptr [rbx + 0x2230]
1405a64b6: mov rax, qword ptr [rbx + 0x2240]
1405a64bd: mov qword ptr [rbx + 0x450], rax
1405a64c4: mov rax, qword ptr [rbx + 0x2248]
1405a64cb: mov qword ptr [rbx + 0x458], rax
1405a64d2: movaps xmmword ptr [rbx + 0x440], xmm0
1405a64d9: mov byte ptr [rbx + 0xd9], 0
1405a64e0: mov byte ptr [rbx + 0xda], 0
1405a64e7: lea rcx, [rbx + 0xb00]
1405a64ee: lea rdx, [rbx + 0x440]
1405a64f5: call 0x14091eff0
1405a64fa: mov byte ptr [rbx + 0xd9], 0
1405a6501: mov byte ptr [rbx + 0xda], 0
1405a6508: lea rdx, [rbx + 0xb00]
1405a650f: mov rcx, qword ptr [rbx + 0xc8]
1405a6516: call 0x140118630
1405a651b: jmp 0x1405a30e0
1405a6520: lea rax, [rip + 0x2458681]
1405a6527: mov qword ptr [rbx + 0xb00], rax
1405a652e: mov qword ptr [rbx + 0xb08], 1
1405a6539: mov qword ptr [rbx + 0xb10], 8
1405a6544: movups xmmword ptr [rbx + 0xb18], xmm6
1405a654b: mov byte ptr [rbx + 0xbc], 1
1405a6552: mov byte ptr [rbx + 0xbd], 1
1405a6559: mov byte ptr [rbx + 0xbe], 1
1405a6560: mov byte ptr [rbx + 0xbf], 1
1405a6567: mov byte ptr [rbx + 0xc0], 1
1405a656e: mov byte ptr [rbx + 0xc1], 1
1405a6575: lea rdx, [rip + 0x245869c]
1405a657c: lea rcx, [rbx + 0xb00]
1405a6583: call 0x1428d9390
1405a6588: jmp 0x1405a7040
1405a658d: mov byte ptr [rbx + 0xe3], 1
1405a6594: lea rcx, [rip + 0x24593fd]
1405a659b: lea r8, [rip + 0x245941e]
1405a65a2: mov edx, 0x21
1405a65a7: call 0x1428d9430
1405a65ac: jmp 0x1405a7040
1405a65b1: mov eax, 0x3b
1405a65b6: lea rcx, [rip + 0x2458243]
1405a65bd: mov qword ptr [rbx + 0xb00], rcx
1405a65c4: mov qword ptr [rbx + 0xb08], rax
1405a65cb: mov byte ptr [rbx + 0xd9], 1
1405a65d2: mov byte ptr [rbx + 0xda], 1
1405a65d9: lea rdx, [rip + 0x24582f0]
1405a65e0: lea rcx, [rbx + 0xb00]
1405a65e7: call 0x1428ce090
1405a65ec: jmp 0x1405a7040
1405a65f1: mov byte ptr [rbx + 0xc2], 1
1405a65f8: mov byte ptr [rbx + 0xc3], 1
1405a65ff: mov byte ptr [rbx + 0xc4], 1
1405a6606: mov byte ptr [rbx + 0xc5], 1
1405a660d: mov byte ptr [rbx + 0xc6], 1
1405a6614: lea rcx, [rip + 0x245951d]
1405a661b: call 0x1428d9310
1405a6620: jmp 0x1405a7040
1405a6625: lea rax, [rip + 0x2458b74]
1405a662c: mov qword ptr [rbx + 0x2590], rax
1405a6633: mov qword ptr [rbx + 0x2598], 0x2b
1405a663e: mov qword ptr [rbx + 0x2bf0], r14
1405a6645: lea rax, [rip - 0x49c7bc]
1405a664c: mov qword ptr [rbx + 0x2bf8], rax
1405a6653: lea rax, [rip + 0x2458b26]
1405a665a: mov qword ptr [rbx + 0xb00], rax
1405a6661: mov qword ptr [rbx + 0xb08], 2
1405a666c: mov qword ptr [rbx + 0xb20], 0
1405a6677: lea rax, [rbx + 0x2bf0]
1405a667e: mov qword ptr [rbx + 0xb10], rax
1405a6685: mov qword ptr [rbx + 0xb18], 1
1405a6690: mov byte ptr [rbx + 0xb3], 1
1405a6697: lea rdx, [rip + 0x24594ca]
1405a669e: lea rcx, [rbx + 0xb00]
1405a66a5: call 0x1428d9390
1405a66aa: jmp 0x1405a7040
1405a66af: mov byte ptr [rbx + 0xc2], 1
1405a66b6: mov byte ptr [rbx + 0xc3], 1
1405a66bd: mov byte ptr [rbx + 0xc4], 1
1405a66c4: mov byte ptr [rbx + 0xc5], 1
1405a66cb: mov byte ptr [rbx + 0xc6], 1
1405a66d2: lea rcx, [rip + 0x2459477]
1405a66d9: call 0x1428d9310
1405a66de: jmp 0x1405a7040
1405a66e3: lea rax, [rip + 0x2458956]
1405a66ea: mov qword ptr [rbx + 0x2590], rax
1405a66f1: mov qword ptr [rbx + 0x2598], 0x36
1405a66fc: mov qword ptr [rbx + 0x2bf0], r14
1405a6703: lea rax, [rip - 0x49c87a]
1405a670a: mov qword ptr [rbx + 0x2bf8], rax
1405a6711: lea rax, [rip + 0x2458a68]
1405a6718: mov qword ptr [rbx + 0xb00], rax
1405a671f: mov qword ptr [rbx + 0xb08], 2
1405a672a: mov qword ptr [rbx + 0xb20], 0
1405a6735: lea rax, [rbx + 0x2bf0]
1405a673c: mov qword ptr [rbx + 0xb10], rax
1405a6743: mov qword ptr [rbx + 0xb18], 1
1405a674e: mov byte ptr [rbx + 0xb3], 1
1405a6755: lea rdx, [rip + 0x2459454]
1405a675c: lea rcx, [rbx + 0xb00]
1405a6763: call 0x1428d9390
1405a6768: jmp 0x1405a7040
1405a676d: lea rax, [rip + 0x2458abc]
1405a6774: mov qword ptr [rbx + 0x2590], rax
1405a677b: mov qword ptr [rbx + 0x2598], 0x40
1405a6786: mov qword ptr [rbx + 0x2bf0], r14
1405a678d: lea rax, [rip - 0x49c904]
1405a6794: mov qword ptr [rbx + 0x2bf8], rax
1405a679b: lea rax, [rip + 0x24589de]
1405a67a2: mov qword ptr [rbx + 0xb00], rax
1405a67a9: mov qword ptr [rbx + 0xb08], 2
1405a67b4: mov qword ptr [rbx + 0xb20], 0
1405a67bf: lea rax, [rbx + 0x2bf0]
1405a67c6: mov qword ptr [rbx + 0xb10], rax
1405a67cd: mov qword ptr [rbx + 0xb18], 1
1405a67d8: mov byte ptr [rbx + 0xb3], 1
1405a67df: lea rdx, [rip + 0x24593e2]
1405a67e6: lea rcx, [rbx + 0xb00]
1405a67ed: call 0x1428d9390
1405a67f2: jmp 0x1405a7040
1405a67f7: lea rax, [rip + 0x2458a9d]
1405a67fe: mov qword ptr [rbx + 0x2590], rax
1405a6805: mov qword ptr [rbx + 0x2598], 0x25
1405a6810: mov qword ptr [rbx + 0x2bf0], r14
1405a6817: lea rax, [rip - 0x49c98e]
1405a681e: mov qword ptr [rbx + 0x2bf8], rax
1405a6825: lea rax, [rip + 0x2458954]
1405a682c: mov qword ptr [rbx + 0xb00], rax
1405a6833: mov qword ptr [rbx + 0xb08], 2
1405a683e: mov qword ptr [rbx + 0xb20], 0
1405a6849: lea rax, [rbx + 0x2bf0]
1405a6850: mov qword ptr [rbx + 0xb10], rax
1405a6857: mov qword ptr [rbx + 0xb18], 1
1405a6862: mov byte ptr [rbx + 0xb3], 1
1405a6869: lea rdx, [rip + 0x2459370]
1405a6870: lea rcx, [rbx + 0xb00]
1405a6877: call 0x1428d9390
1405a687c: jmp 0x1405a7040
1405a6881: lea rax, [rip + 0x2458943]
1405a6888: mov qword ptr [rbx + 0x2590], rax
1405a688f: mov qword ptr [rbx + 0x2598], 0x33
1405a689a: mov qword ptr [rbx + 0x2bf0], r14
1405a68a1: lea rax, [rip - 0x49ca18]
1405a68a8: mov qword ptr [rbx + 0x2bf8], rax
1405a68af: lea rax, [rip + 0x24588ca]
1405a68b6: mov qword ptr [rbx + 0xb00], rax
1405a68bd: mov qword ptr [rbx + 0xb08], 2
1405a68c8: mov qword ptr [rbx + 0xb20], 0
1405a68d3: lea rax, [rbx + 0x2bf0]
1405a68da: mov qword ptr [rbx + 0xb10], rax
1405a68e1: mov qword ptr [rbx + 0xb18], 1
1405a68ec: mov byte ptr [rbx + 0xb3], 1
1405a68f3: lea rdx, [rip + 0x24592fe]
1405a68fa: lea rcx, [rbx + 0xb00]
1405a6901: call 0x1428d9390
1405a6906: jmp 0x1405a7040
1405a690b: lea rax, [rip + 0x24588ec]
1405a6912: mov qword ptr [rbx + 0x2590], rax
1405a6919: mov qword ptr [rbx + 0x2598], 0x32
1405a6924: mov qword ptr [rbx + 0x2bf0], r14
1405a692b: lea rax, [rip - 0x49caa2]
1405a6932: mov qword ptr [rbx + 0x2bf8], rax
1405a6939: lea rax, [rip + 0x2458840]
1405a6940: mov qword ptr [rbx + 0xb00], rax
1405a6947: mov qword ptr [rbx + 0xb08], 2
1405a6952: mov qword ptr [rbx + 0xb20], 0
1405a695d: lea rax, [rbx + 0x2bf0]
1405a6964: mov qword ptr [rbx + 0xb10], rax
1405a696b: mov qword ptr [rbx + 0xb18], 1
1405a6976: mov byte ptr [rbx + 0xb3], 1
1405a697d: lea rdx, [rip + 0x245928c]
1405a6984: lea rcx, [rbx + 0xb00]
1405a698b: call 0x1428d9390
1405a6990: jmp 0x1405a7040
1405a6995: lea rax, [rip + 0x24588d4]
1405a699c: mov qword ptr [rbx + 0x2590], rax
1405a69a3: mov qword ptr [rbx + 0x2598], 0x2b
1405a69ae: lea rax, [rbx + 0x2590]
1405a69b5: mov qword ptr [rbx + 0x2bf0], rax
1405a69bc: lea rax, [rip - 0x49cb33]
1405a69c3: mov qword ptr [rbx + 0x2bf8], rax
1405a69ca: lea rax, [rip + 0x24587af]
1405a69d1: mov qword ptr [rbx + 0xb00], rax
1405a69d8: mov qword ptr [rbx + 0xb08], 2
1405a69e3: mov qword ptr [rbx + 0xb20], 0
1405a69ee: lea rax, [rbx + 0x2bf0]
1405a69f5: mov qword ptr [rbx + 0xb10], rax
1405a69fc: mov qword ptr [rbx + 0xb18], 1
1405a6a07: mov byte ptr [rbx + 0xb3], 0
1405a6a0e: lea rdx, [rip + 0x2459213]
1405a6a15: lea rcx, [rbx + 0xb00]
1405a6a1c: call 0x1428d9390
1405a6a21: jmp 0x1405a7040
1405a6a26: mov ecx, 8
1405a6a2b: mov edx, 0xaf0
1405a6a30: call 0x1428d9000
1405a6a35: jmp 0x1405a7040
1405a6a3a: mov rax, qword ptr [rbx + 0x4b0]
1405a6a41: mov qword ptr [rbx + 0xb70], rax
1405a6a48: movups xmm0, xmmword ptr [rbx + 0x4a0]
1405a6a4f: movaps xmmword ptr [rbx + 0xb60], xmm0
1405a6a56: movups xmm0, xmmword ptr [rbx + 0x490]
1405a6a5d: movaps xmmword ptr [rbx + 0xb50], xmm0
1405a6a64: movups xmm0, xmmword ptr [rbx + 0x480]
1405a6a6b: movaps xmmword ptr [rbx + 0xb40], xmm0
1405a6a72: movupd xmm0, xmmword ptr [rbx + 0x440]
1405a6a7a: movups xmm1, xmmword ptr [rbx + 0x450]
1405a6a81: movups xmm2, xmmword ptr [rbx + 0x460]
1405a6a88: movups xmm3, xmmword ptr [rbx + 0x470]
1405a6a8f: movaps xmmword ptr [rbx + 0xb30], xmm3
1405a6a96: movaps xmmword ptr [rbx + 0xb20], xmm2
1405a6a9d: movaps xmmword ptr [rbx + 0xb10], xmm1
1405a6aa4: movapd xmmword ptr [rbx + 0xb00], xmm0
1405a6aac: lea rax, [rip + 0x2458ca5]
1405a6ab3: mov qword ptr [rsp + 0x20], rax
1405a6ab8: lea rcx, [rip + 0x2457ef1]
1405a6abf: lea r9, [rip + 0x2457f5a]
1405a6ac6: lea r8, [rbx + 0xb00]
1405a6acd: mov edx, 0x2b
1405a6ad2: call 0x1428d9760
1405a6ad7: jmp 0x1405a7040
1405a6adc: lea rcx, [rip + 0x238bf2d]
1405a6ae3: call 0x1428d80d0
1405a6ae8: jmp 0x1405a7040
1405a6aed: mov byte ptr [rbx + 0x108], 1
1405a6af4: mov byte ptr [rbx + 0x109], 1
1405a6afb: mov byte ptr [rbx + 0x10a], 1
1405a6b02: mov byte ptr [rbx + 0x10b], 1
1405a6b09: mov byte ptr [rbx + 0x10c], 1
1405a6b10: mov byte ptr [rbx + 0x10d], 1
1405a6b17: mov byte ptr [rbx + 0x10e], 1
1405a6b1e: mov byte ptr [rbx + 0x10f], 1
1405a6b25: mov ecx, 8
1405a6b2a: mov edx, 0xb0
1405a6b2f: call 0x1428d9000
1405a6b34: jmp 0x1405a7040
1405a6b39: lea rax, [rip + 0x23a9cd0]
1405a6b40: mov qword ptr [rbx + 0xb00], rax
1405a6b47: mov qword ptr [rbx + 0xb08], 1
1405a6b52: mov qword ptr [rbx + 0xb10], 8
1405a6b5d: xorps xmm0, xmm0
1405a6b60: movups xmmword ptr [rbx + 0xb18], xmm0
1405a6b67: mov byte ptr [rbx + 0x110], 1
1405a6b6e: lea rdx, [rip + 0x2458c5b]
1405a6b75: lea rcx, [rbx + 0xb00]
1405a6b7c: call 0x1428d9390
1405a6b81: jmp 0x1405a7040
1405a6b86: lea rax, [rip + 0x23a9c83]
1405a6b8d: mov qword ptr [rbx + 0xb00], rax
1405a6b94: mov qword ptr [rbx + 0xb08], 1
1405a6b9f: mov qword ptr [rbx + 0xb10], 8
1405a6baa: xorps xmm0, xmm0
1405a6bad: movups xmmword ptr [rbx + 0xb18], xmm0
1405a6bb4: mov byte ptr [rbx + 0xaa], 1
1405a6bbb: mov byte ptr [rbx + 0xab], 1
1405a6bc2: mov byte ptr [rbx + 0xac], 1
1405a6bc9: mov byte ptr [rbx + 0xad], 1
1405a6bd0: mov byte ptr [rbx + 0xae], 1
1405a6bd7: mov byte ptr [rbx + 0xaf], 1
1405a6bde: mov byte ptr [rbx + 0xb0], 1
1405a6be5: mov byte ptr [rbx + 0xb1], 1
1405a6bec: mov byte ptr [rbx + 0xb2], 1
1405a6bf3: lea rdx, [rip + 0x2458bee]
1405a6bfa: lea rcx, [rbx + 0xb00]
1405a6c01: call 0x1428d9390
1405a6c06: jmp 0x1405a7040
1405a6c0b: lea rax, [rip + 0x23a9bfe]
1405a6c12: mov qword ptr [rbx + 0xb00], rax
1405a6c19: mov qword ptr [rbx + 0xb08], 1
1405a6c24: mov qword ptr [rbx + 0xb10], 8
1405a6c2f: xorps xmm0, xmm0
1405a6c32: movups xmmword ptr [rbx + 0xb18], xmm0
1405a6c39: mov byte ptr [rbx + 0xaa], 1
1405a6c40: mov byte ptr [rbx + 0xab], 1
1405a6c47: mov byte ptr [rbx + 0xac], 1
1405a6c4e: mov byte ptr [rbx + 0xad], 1
1405a6c55: mov byte ptr [rbx + 0xae], 1
1405a6c5c: mov byte ptr [rbx + 0xaf], 1
1405a6c63: mov byte ptr [rbx + 0xb0], 1
1405a6c6a: mov byte ptr [rbx + 0xb1], 1
1405a6c71: mov byte ptr [rbx + 0xb2], 1
1405a6c78: lea rdx, [rip + 0x2458b81]
1405a6c7f: lea rcx, [rbx + 0xb00]
1405a6c86: call 0x1428d9390
1405a6c8b: jmp 0x1405a7040
1405a6c90: lea rax, [rip + 0x23a9b79]
1405a6c97: mov qword ptr [rbx + 0xb00], rax
1405a6c9e: mov qword ptr [rbx + 0xb08], 1
1405a6ca9: mov qword ptr [rbx + 0xb10], 8
1405a6cb4: xorps xmm0, xmm0
1405a6cb7: movups xmmword ptr [rbx + 0xb18], xmm0
1405a6cbe: mov byte ptr [rbx + 0xaa], 1
1405a6cc5: mov byte ptr [rbx + 0xab], 1
1405a6ccc: mov byte ptr [rbx + 0xac], 1
1405a6cd3: mov byte ptr [rbx + 0xad], 1
1405a6cda: mov byte ptr [rbx + 0xae], 1
1405a6ce1: mov byte ptr [rbx + 0xaf], 1
1405a6ce8: mov byte ptr [rbx + 0xb0], 1
1405a6cef: mov byte ptr [rbx + 0xb1], 1
1405a6cf6: mov byte ptr [rbx + 0xb2], 1
1405a6cfd: lea rdx, [rip + 0x2458b14]
1405a6d04: lea rcx, [rbx + 0xb00]
1405a6d0b: call 0x1428d9390
1405a6d10: jmp 0x1405a7040
1405a6d15: mov byte ptr [rbx + 0xaa], 1
1405a6d1c: mov byte ptr [rbx + 0xab], 1
1405a6d23: mov byte ptr [rbx + 0xac], 1
1405a6d2a: mov byte ptr [rbx + 0xad], 1
1405a6d31: mov byte ptr [rbx + 0xae], 0
1405a6d38: mov byte ptr [rbx + 0xaf], 1
1405a6d3f: mov byte ptr [rbx + 0xb0], 0
1405a6d46: mov byte ptr [rbx + 0xb1], 1
1405a6d4d: mov byte ptr [rbx + 0xb2], 1
1405a6d54: lea rax, [rip + 0x2458b95]
1405a6d5b: mov qword ptr [rsp + 0x20], rax
1405a6d60: lea rcx, [rip + 0x2457c49]
1405a6d67: lea r9, [rip + 0x2457c02]
1405a6d6e: lea r8, [rbx + 0x370]
1405a6d75: mov edx, 0x2b
1405a6d7a: call 0x1428d9760
1405a6d7f: jmp 0x1405a7040
1405a6d84: lea rax, [rip + 0x23a9a85]
1405a6d8b: mov qword ptr [rbx + 0xb00], rax
1405a6d92: mov qword ptr [rbx + 0xb08], 1
1405a6d9d: mov qword ptr [rbx + 0xb10], 8
1405a6da8: xorps xmm0, xmm0
1405a6dab: movups xmmword ptr [rbx + 0xb18], xmm0
1405a6db2: mov byte ptr [rbx + 0xb4], 1
1405a6db9: mov byte ptr [rbx + 0xb5], 1
1405a6dc0: mov byte ptr [rbx + 0xb6], 1
1405a6dc7: mov byte ptr [rbx + 0xb7], 0
1405a6dce: mov byte ptr [rbx + 0xb8], 1
1405a6dd5: mov byte ptr [rbx + 0xb9], 1
1405a6ddc: mov byte ptr [rbx + 0xba], 1
1405a6de3: mov byte ptr [rbx + 0xbb], 1
1405a6dea: lea rdx, [rip + 0x2458b77]
1405a6df1: lea rcx, [rbx + 0xb00]
1405a6df8: call 0x1428d9390
1405a6dfd: jmp 0x1405a7040
1405a6e02: lea r8, [rip + 0x2457c87]
1405a6e09: mov ecx, 1
1405a6e0e: mov edx, 0x12
1405a6e13: call 0x1428d8fe3
1405a6e18: jmp 0x1405a7040
1405a6e1d: mov rax, qword ptr [rbx + 0xb08]
1405a6e24: mov qword ptr [rbx + 0x440], rax
1405a6e2b: lea rax, [rip + 0x2458cee]
1405a6e32: mov qword ptr [rsp + 0x20], rax
1405a6e37: lea rcx, [rip + 0x2458cc4]
1405a6e3e: lea r9, [rip + 0x2457acb]
1405a6e45: lea r8, [rbx + 0x440]
1405a6e4c: mov edx, 0x1a
1405a6e51: call 0x1428d9760
1405a6e56: jmp 0x1405a7040
1405a6e5b: mov ecx, 0x80
1405a6e60: mov edx, 0x200
1405a6e65: call 0x1428d9000
1405a6e6a: jmp 0x1405a7040
1405a6e6f: mov r8, rsi
1405a6e72: mov qword ptr [rbx + 0x440], r8
1405a6e79: lea rax, [rip + 0x2458b00]
1405a6e80: mov qword ptr [rsp + 0x20], rax
1405a6e85: lea rcx, [rip + 0x2457b24]
1405a6e8c: lea r9, [rip + 0x2457afd]
1405a6e93: lea r8, [rbx + 0x440]
1405a6e9a: mov edx, 0x2b
1405a6e9f: call 0x1428d9760
1405a6ea4: jmp 0x1405a7040
1405a6ea9: mov qword ptr [rbx + 0xb00], rsi
1405a6eb0: lea rax, [rip + 0x2458b21]
1405a6eb7: mov qword ptr [rsp + 0x20], rax
1405a6ebc: lea rcx, [rip + 0x2457aed]
1405a6ec3: lea r9, [rip + 0x2457a46]
1405a6eca: lea r8, [rbx + 0xb00]
1405a6ed1: mov edx, 0x2b
1405a6ed6: call 0x1428d9760
1405a6edb: jmp 0x1405a7040
1405a6ee0: mov qword ptr [rbx + 0x440], rdx
1405a6ee7: lea rax, [rip + 0x2458b22]
1405a6eee: mov qword ptr [rsp + 0x20], rax
1405a6ef3: lea rcx, [rip + 0x2457ab6]
1405a6efa: lea r9, [rip + 0x2457a0f]
1405a6f01: lea r8, [rbx + 0x440]
1405a6f08: mov edx, 0x2b
1405a6f0d: call 0x1428d9760
1405a6f12: jmp 0x1405a7040
1405a6f17: mov qword ptr [rbx + 0x440], r15
1405a6f1e: lea rax, [rip + 0x2458b43]
1405a6f25: mov qword ptr [rsp + 0x20], rax
1405a6f2a: lea rcx, [rip + 0x2457a7f]
1405a6f31: lea r9, [rip + 0x24579d8]
1405a6f38: lea r8, [rbx + 0x440]
1405a6f3f: mov edx, 0x2b
1405a6f44: call 0x1428d9760
1405a6f49: jmp 0x1405a7040
1405a6f4e: lea rax, [rip + 0x23a98bb]
1405a6f55: mov qword ptr [rbx + 0xb00], rax
1405a6f5c: mov qword ptr [rbx + 0xb08], 1
1405a6f67: mov qword ptr [rbx + 0xb10], 8
1405a6f72: xorps xmm0, xmm0
1405a6f75: movups xmmword ptr [rbx + 0xb18], xmm0
1405a6f7c: mov byte ptr [rbx + 0xb3], 1
1405a6f83: lea rdx, [rip + 0x2458bf6]
1405a6f8a: lea rcx, [rbx + 0xb00]
1405a6f91: call 0x1428d9390
1405a6f96: jmp 0x1405a7040
1405a6f9b: lea rax, [rip + 0x23a986e]
1405a6fa2: mov qword ptr [rbx + 0xb00], rax
1405a6fa9: mov qword ptr [rbx + 0xb08], 1
1405a6fb4: mov qword ptr [rbx + 0xb10], 8
1405a6fbf: xorps xmm0, xmm0
1405a6fc2: movups xmmword ptr [rbx + 0xb18], xmm0
1405a6fc9: mov byte ptr [rbx + 0xb3], 1
1405a6fd0: lea rdx, [rip + 0x2458bc1]
1405a6fd7: lea rcx, [rbx + 0xb00]
1405a6fde: call 0x1428d9390
1405a6fe3: jmp 0x1405a7040
1405a6fe5: lea r8, [rip + 0x2457aa4]
1405a6fec: mov ecx, 1
1405a6ff1: mov edx, 0xa
1405a6ff6: call 0x1428d8fe3
1405a6ffb: jmp 0x1405a7040
1405a6ffd: mov eax, 0x48
1405a7002: lea rcx, [rip + 0x2457832]
1405a7009: jmp 0x1405a65bd
1405a700e: mov qword ptr [rbx + 0x440], rax
1405a7015: lea rax, [rip + 0x2458a1c]
1405a701c: mov qword ptr [rsp + 0x20], rax
1405a7021: lea rcx, [rip + 0x2457988]
1405a7028: lea r9, [rip + 0x2457961]
1405a702f: lea r8, [rbx + 0x440]
1405a7036: mov edx, 0x2b
1405a703b: call 0x1428d9760
1405a7040: ud2 
1405a7042: mov ecx, 7
1405a7047: int 0x29
1405a7049: ud2 