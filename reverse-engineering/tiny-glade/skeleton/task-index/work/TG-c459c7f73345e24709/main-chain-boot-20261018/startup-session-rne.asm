000000014166e7d0: push rbp
000000014166e7d1: push r15
000000014166e7d3: push r14
000000014166e7d5: push r13
000000014166e7d7: push r12
000000014166e7d9: push rsi
000000014166e7da: push rdi
000000014166e7db: push rbx
000000014166e7dc: sub rsp, 0xed8
000000014166e7e3: lea rbp, [rsp + 0x80]
000000014166e7eb: movaps xmmword ptr [rbp + 0xe40], xmm6
000000014166e7f2: mov qword ptr [rbp + 0xe38], 0xfffffffffffffffe
000000014166e7fd: mov rdi, r9
000000014166e800: mov rsi, r8
000000014166e803: mov qword ptr [rbp + 0x908], rdx
000000014166e80a: mov rbx, rcx
000000014166e80d: mov rax, qword ptr [rbp + 0xf00]
000000014166e814: mov r14, qword ptr [rax + 8]
000000014166e818: test r14, r14
000000014166e81b: jne 0x14166e821
000000014166e81d: mov r14, qword ptr [rax + 0x10]
000000014166e821: mov r12, qword ptr [rbp + 0xec0]
000000014166e828: mov rax, qword ptr [r14]
000000014166e82b: mov r15, qword ptr [r14 + 0x10]
000000014166e82f: sub rax, r15
000000014166e832: cmp rax, 7
000000014166e836: jbe 0x14167161f
000000014166e83c: mov rax, qword ptr [r14 + 8]
000000014166e840: lea rcx, [rip - 0x394d27]
000000014166e847: mov qword ptr [rax + r15], rcx
000000014166e84b: add r15, 8
000000014166e84f: mov qword ptr [r14 + 0x10], r15
000000014166e853: mov eax, dword ptr [r12 + 0x1c]
000000014166e858: mov rcx, qword ptr [r12 + 0x10]
000000014166e85d: mov dword ptr [rcx], eax
000000014166e85f: mov r14, qword ptr [r12]
000000014166e863: lea rcx, [rbp + 0x320]
000000014166e86a: mov r8d, 0xa0
000000014166e870: mov rdx, r14
000000014166e873: call 0x142923c00
000000014166e878: mov qword ptr [r14], 0
000000014166e87f: mov qword ptr [r14 + 8], 8
000000014166e887: mov qword ptr [r14 + 0x10], 0
000000014166e88f: movabs r12, 0x8000000000000000
000000014166e899: mov qword ptr [r14 + 0x18], r12
000000014166e89d: mov byte ptr [r14 + 0x98], 0
000000014166e8a5: movzx edx, byte ptr [rbp + 0x3b8]
000000014166e8ac: mov qword ptr [rbp + 0xd60], 0
000000014166e8b7: xorps xmm0, xmm0
000000014166e8ba: movups xmmword ptr [rbp + 0xd70], xmm0
000000014166e8c1: mov qword ptr [rbp + 0xd68], 8
000000014166e8cc: mov rax, qword ptr [rbp + 0x338]
000000014166e8d3: mov rcx, qword ptr [rbx]
000000014166e8d6: cmp qword ptr [rcx + 0x10], 0
000000014166e8db: mov byte ptr [rbp + 0xe07], dl
000000014166e8e1: je 0x14166e902
000000014166e8e3: mov rcx, qword ptr [rcx + 8]
000000014166e8e7: mov rdx, qword ptr [rcx + 0x10]
000000014166e8eb: mov qword ptr [rbp + 0x3e0], rdx
000000014166e8f2: movups xmm0, xmmword ptr [rcx]
000000014166e8f5: movaps xmmword ptr [rbp + 0x3d0], xmm0
000000014166e8fc: movzx edx, byte ptr [rcx + 0x18]
000000014166e900: jmp 0x14166e904
000000014166e902: mov dl, 3
000000014166e904: lea rcx, [rbp + 0x340]
000000014166e90b: mov r8, rax
000000014166e90e: xor r8, r12
000000014166e911: cmp r8, 4
000000014166e915: mov r9d, 4
000000014166e91b: cmovb r9, r8
000000014166e91f: lea r8, [rip + 0x16163fe]
000000014166e926: movsxd r9, dword ptr [r8 + r9*4]
000000014166e92a: add r9, r8
000000014166e92d: movabs r13, 0x8000000000000001
000000014166e937: jmp r9
000000014166e93a: cmp dl, 3
000000014166e93d: je 0x14166ead6
000000014166e943: movaps xmm0, xmmword ptr [rbp + 0x3d0]
000000014166e94a: movaps xmmword ptr [rbp + 0x950], xmm0
000000014166e951: cmp rax, r13
000000014166e954: je 0x14166e96e
000000014166e956: jmp 0x14166eb22
000000014166e95b: movups xmm0, xmmword ptr [rcx]
000000014166e95e: movaps xmmword ptr [rbp + 0x950], xmm0
000000014166e965: cmp rax, r13
000000014166e968: jne 0x14166eb22
000000014166e96e: mov rdx, qword ptr [rdi]
000000014166e971: mov byte ptr [rbp + 0xe35], 0
000000014166e978: lea rdi, [rbp + 0x950]
000000014166e97f: mov rcx, rdi
000000014166e982: call 0x140a9fa80
000000014166e987: test al, al
000000014166e989: je 0x14166eb22
000000014166e98f: mov rax, qword ptr [rip + 0x1fbda12]
000000014166e996: mov rax, qword ptr [rax]
000000014166e999: cmp rax, 3
000000014166e99d: jb 0x14166ea8b
000000014166e9a3: mov qword ptr [rbp + 0x660], rdi
000000014166e9aa: lea rax, [rip + 0x4a7f]
000000014166e9b1: mov qword ptr [rbp + 0x668], rax
000000014166e9b8: mov byte ptr [rbp + 0xe35], 0
000000014166e9bf: lea rcx, [rip + 0x1615d12]
000000014166e9c6: call 0x14067dd80
000000014166e9cb: movups xmm0, xmmword ptr [rax]
000000014166e9ce: mov eax, dword ptr [rax + 0x10]
000000014166e9d1: mov qword ptr [rbp + 0xb00], 3
000000014166e9dc: lea rcx, [rip + 0x1615d0d]
000000014166e9e3: mov qword ptr [rbp + 0xb08], rcx
000000014166e9ea: mov qword ptr [rbp + 0xb10], 0x26
000000014166e9f5: lea rdx, [rip + 0x1615cbc]
000000014166e9fc: mov qword ptr [rbp + 0xb20], rdx
000000014166ea03: mov qword ptr [rbp + 0xb28], 2
000000014166ea0e: lea rdx, [rbp + 0x660]
000000014166ea15: mov qword ptr [rbp + 0xb30], rdx
000000014166ea1c: mov qword ptr [rbp + 0xb38], 1
000000014166ea27: mov qword ptr [rbp + 0xb40], 0
000000014166ea32: mov qword ptr [rbp + 0xad0], 0
000000014166ea3d: mov qword ptr [rbp + 0xad8], rcx
000000014166ea44: mov qword ptr [rbp + 0xae0], 0x26
000000014166ea4f: mov qword ptr [rbp + 0xae8], 0
000000014166ea5a: movups xmmword ptr [rbp + 0xaf0], xmm0
000000014166ea61: mov dword ptr [rbp + 0xb18], 1
000000014166ea6b: mov dword ptr [rbp + 0xb1c], eax
000000014166ea71: mov byte ptr [rbp + 0xe35], 0
000000014166ea78: lea rcx, [rbp + 0xe30]
000000014166ea7f: lea rdx, [rbp + 0xad0]
000000014166ea86: call 0x140d4a330
000000014166ea8b: mov rax, qword ptr [rbp + 0xef8]
000000014166ea92: mov ecx, dword ptr [rax + 0x1c]
000000014166ea95: mov rdx, qword ptr [rax + 0x10]
000000014166ea99: mov dword ptr [rdx], ecx
000000014166ea9b: mov rdi, qword ptr [rax]
000000014166ea9e: mov rbx, qword ptr [rdi + 0x30]
000000014166eaa2: mov r14, qword ptr [rdi + 0x40]
000000014166eaa6: cmp rbx, qword ptr [rdi + 0x20]
000000014166eaaa: jne 0x14166eac3
000000014166eaac: lea rcx, [rdi + 0x20]
000000014166eab0: mov byte ptr [rbp + 0xe35], 0
000000014166eab7: lea rdx, [rip + 0x15cb68a]
000000014166eabe: call 0x140562870
000000014166eac3: mov rax, qword ptr [rdi + 0x28]
000000014166eac7: mov qword ptr [rax + rbx*8], r14
000000014166eacb: inc rbx
000000014166eace: mov qword ptr [rdi + 0x30], rbx
000000014166ead2: inc qword ptr [rdi + 0x40]
000000014166ead6: mov qword ptr [rbp + 0xa08], 0
000000014166eae1: mov qword ptr [rbp + 0xa10], 1
000000014166eaec: mov qword ptr [rbp + 0xa18], 0
000000014166eaf7: movabs rax, 0x8000000000000000
000000014166eb01: mov qword ptr [rbp + 0xa20], rax
000000014166eb08: mov byte ptr [rbp + 0xa60], 0
000000014166eb0f: mov qword ptr [rbp + 0xa00], r13
000000014166eb16: mov qword ptr [rbp + 0xa68], rax
000000014166eb1d: jmp 0x14166f4a3
000000014166eb22: mov rax, qword ptr [rip + 0x1fbd87f]
000000014166eb29: mov rax, qword ptr [rax]
000000014166eb2c: cmp rax, 3
000000014166eb30: jb 0x14166ec25
000000014166eb36: lea rax, [rbp + 0x950]
000000014166eb3d: mov qword ptr [rbp + 0x660], rax
000000014166eb44: lea rax, [rip + 0x48e5]
000000014166eb4b: mov qword ptr [rbp + 0x668], rax
000000014166eb52: mov byte ptr [rbp + 0xe35], 0
000000014166eb59: lea rcx, [rip + 0x1615be0]
000000014166eb60: call 0x14067dd80
000000014166eb65: movups xmm0, xmmword ptr [rax]
000000014166eb68: mov eax, dword ptr [rax + 0x10]
000000014166eb6b: mov qword ptr [rbp + 0xb00], 3
000000014166eb76: lea rcx, [rip + 0x1615b73]
000000014166eb7d: mov qword ptr [rbp + 0xb08], rcx
000000014166eb84: mov qword ptr [rbp + 0xb10], 0x26
000000014166eb8f: lea rdx, [rip + 0x1615b9a]
000000014166eb96: mov qword ptr [rbp + 0xb20], rdx
000000014166eb9d: mov qword ptr [rbp + 0xb28], 1
000000014166eba8: lea rdx, [rbp + 0x660]
000000014166ebaf: mov qword ptr [rbp + 0xb30], rdx
000000014166ebb6: mov qword ptr [rbp + 0xb38], 1
000000014166ebc1: mov qword ptr [rbp + 0xb40], 0
000000014166ebcc: mov qword ptr [rbp + 0xad0], 0
000000014166ebd7: mov qword ptr [rbp + 0xad8], rcx
000000014166ebde: mov qword ptr [rbp + 0xae0], 0x26
000000014166ebe9: mov qword ptr [rbp + 0xae8], 0
000000014166ebf4: movups xmmword ptr [rbp + 0xaf0], xmm0
000000014166ebfb: mov dword ptr [rbp + 0xb18], 1
000000014166ec05: mov dword ptr [rbp + 0xb1c], eax
000000014166ec0b: mov byte ptr [rbp + 0xe35], 0
000000014166ec12: lea rcx, [rbp + 0xe30]
000000014166ec19: lea rdx, [rbp + 0xad0]
000000014166ec20: call 0x140d4a330
000000014166ec25: mov rax, qword ptr [rbp + 0xef0]
000000014166ec2c: mov ecx, dword ptr [rax + 0x1c]
000000014166ec2f: mov rdx, qword ptr [rax + 0x10]
000000014166ec33: mov dword ptr [rdx], ecx
000000014166ec35: mov rdi, qword ptr [rax]
000000014166ec38: mov rbx, qword ptr [rdi + 0x30]
000000014166ec3c: mov r14, qword ptr [rdi + 0x40]
000000014166ec40: cmp rbx, qword ptr [rdi + 0x20]
000000014166ec44: jne 0x14166ec5d
000000014166ec46: lea rcx, [rdi + 0x20]
000000014166ec4a: mov byte ptr [rbp + 0xe35], 0
000000014166ec51: lea rdx, [rip + 0x161a420]
000000014166ec58: call 0x140562870
000000014166ec5d: mov rax, qword ptr [rdi + 0x28]
000000014166ec61: mov qword ptr [rax + rbx*8], r14
000000014166ec65: inc rbx
000000014166ec68: mov qword ptr [rdi + 0x30], rbx
000000014166ec6c: inc qword ptr [rdi + 0x40]
000000014166ec70: movaps xmm0, xmmword ptr [rbp + 0x950]
000000014166ec77: movaps xmmword ptr [rbp - 0x20], xmm0
000000014166ec7b: mov byte ptr [rbp + 0xe35], 0
000000014166ec82: lea rcx, [rbp + 0x3f0]
000000014166ec89: lea rdx, [rbp - 0x20]
000000014166ec8d: call 0x140b46000
000000014166ec92: lea rcx, [rbp + 0x3f0]
000000014166ec99: call 0x140d76ed0
000000014166ec9e: mov r8, rdx
000000014166eca1: mov qword ptr [rsp + 0x20], 0xc
000000014166ecaa: lea r9, [rip + 0x1615936]
000000014166ecb1: lea rcx, [rbp + 0x660]
000000014166ecb8: mov rdx, rax
000000014166ecbb: call 0x140d62810
000000014166ecc0: movups xmm0, xmmword ptr [rbp + 0x660]
000000014166ecc7: movups xmm1, xmmword ptr [rbp + 0x670]
000000014166ecce: movaps xmmword ptr [rbp + 0xd0], xmm1
000000014166ecd5: movaps xmmword ptr [rbp + 0xc0], xmm0
000000014166ecdc: movaps xmm0, xmmword ptr [rbp - 0x20]
000000014166ece0: movups xmmword ptr [rbp + 0xae8], xmm0
000000014166ece7: mov rdx, qword ptr [rbp + 0x3f0]
000000014166ecee: test rdx, rdx
000000014166ecf1: je 0x14166ed05
000000014166ecf3: mov rcx, qword ptr [rbp + 0x3f8]
000000014166ecfa: mov r8d, 1
000000014166ed00: call 0x140613c20
000000014166ed05: movaps xmm0, xmmword ptr [rbp + 0xc0]
000000014166ed0c: movaps xmm1, xmmword ptr [rbp + 0xd0]
000000014166ed13: movaps xmmword ptr [rbp + 0xa10], xmm0
000000014166ed1a: movaps xmmword ptr [rbp + 0xa20], xmm1
000000014166ed21: movups xmm0, xmmword ptr [rbp + 0xad0]
000000014166ed28: movups xmm1, xmmword ptr [rbp + 0xae0]
000000014166ed2f: movups xmmword ptr [rbp + 0xa98], xmm0
000000014166ed36: movups xmmword ptr [rbp + 0xaa8], xmm1
000000014166ed3d: mov rax, qword ptr [rbp + 0xaf0]
000000014166ed44: mov qword ptr [rbp + 0xab8], rax
000000014166ed4b: movabs rax, 0x8000000000000000
000000014166ed55: mov qword ptr [rbp + 0xa00], rax
000000014166ed5c: mov qword ptr [rbp + 0xa08], 0
000000014166ed67: mov qword ptr [rbp + 0xa30], rax
000000014166ed6e: mov qword ptr [rbp + 0xa90], rax
000000014166ed75: mov word ptr [rbp + 0xac0], 0
000000014166ed7e: mov qword ptr [rbp + 0xad8], 0
000000014166ed89: mov qword ptr [rbp + 0xae0], 1
000000014166ed94: mov qword ptr [rbp + 0xae8], 0
000000014166ed9f: mov qword ptr [rbp + 0xaf0], rax
000000014166eda6: mov byte ptr [rbp + 0xb30], 0
000000014166edad: mov qword ptr [rbp + 0xb38], rax
000000014166edb4: mov qword ptr [rbp + 0xad0], r13
000000014166edbb: mov rax, qword ptr [rbp + 0xd60]
000000014166edc2: mov rdi, qword ptr [rbp + 0xd78]
000000014166edc9: cmp rdi, rax
000000014166edcc: jne 0x14166edef
000000014166edce: lea rdx, [rip + 0x1615983]
000000014166edd5: lea rcx, [rbp + 0xd60]
000000014166eddc: call 0x140b5ac80
000000014166ede1: mov rax, qword ptr [rbp + 0xd60]
000000014166ede8: mov rdi, qword ptr [rbp + 0xd78]
000000014166edef: mov rcx, qword ptr [rbp + 0xd70]
000000014166edf6: add rcx, rdi
000000014166edf9: xor edx, edx
000000014166edfb: cmp rcx, rax
000000014166edfe: cmovae rdx, rax
000000014166ee02: sub rcx, rdx
000000014166ee05: imul rcx, rcx, 0xc8
000000014166ee0c: add rcx, qword ptr [rbp + 0xd68]
000000014166ee13: lea rdx, [rbp + 0xad0]
000000014166ee1a: mov r8d, 0xc8
000000014166ee20: call 0x142923c10
000000014166ee25: inc rdi
000000014166ee28: mov qword ptr [rbp + 0xd78], rdi
000000014166ee2f: jmp 0x14166f4a3
000000014166ee34: lea rdx, [rbp + 0x360]
000000014166ee3b: mov qword ptr [rbp + 0x660], rax
000000014166ee42: movups xmm0, xmmword ptr [rcx]
000000014166ee45: movups xmmword ptr [rbp + 0x668], xmm0
000000014166ee4c: movups xmm0, xmmword ptr [rcx + 0x10]
000000014166ee50: movups xmmword ptr [rbp + 0x678], xmm0
000000014166ee57: mov rax, qword ptr [rdx + 0x30]
000000014166ee5b: mov qword ptr [rbp + 0x6b8], rax
000000014166ee62: movups xmm0, xmmword ptr [rdx]
000000014166ee65: movups xmm1, xmmword ptr [rdx + 0x10]
000000014166ee69: movups xmm2, xmmword ptr [rdx + 0x20]
000000014166ee6d: movups xmmword ptr [rbp + 0x6a8], xmm2
000000014166ee74: movups xmmword ptr [rbp + 0x698], xmm1
000000014166ee7b: movups xmmword ptr [rbp + 0x688], xmm0
000000014166ee82: mov rax, qword ptr [rbp + 0x398]
000000014166ee89: mov qword ptr [rbp + 0xe20], rax
000000014166ee90: mov rax, qword ptr [rbp + 0x3a0]
000000014166ee97: mov rcx, qword ptr [rbp + 0x670]
000000014166ee9e: test rcx, rcx
000000014166eea1: mov qword ptr [rbp + 0xe18], rax
000000014166eea8: js 0x14167188b
000000014166eeae: mov rdi, qword ptr [rbp + 0x3a8]
000000014166eeb5: movzx r15d, byte ptr [rbp + 0x3b0]
000000014166eebd: movzx r14d, byte ptr [rbp + 0x6b8]
000000014166eec5: je 0x14166f0e6
000000014166eecb: mov byte ptr [rbp + 0xe08], r14b
000000014166eed2: mov r14, rsi
000000014166eed5: mov esi, r15d
000000014166eed8: mov r13, qword ptr [rbp + 0x668]
000000014166eedf: movzx eax, byte ptr [rip + 0x1fc5d23]
000000014166eee6: mov ebx, 1
000000014166eeeb: mov edx, 1
000000014166eef0: mov r15, rcx
000000014166eef3: call 0x140613c10
000000014166eef8: test rax, rax
000000014166eefb: je 0x141671890
000000014166ef01: lea r9, [r15 - 1]
000000014166ef05: cmp r15, r9
000000014166ef08: cmovb r9, r15
000000014166ef0c: cmp r9, 0x20
000000014166ef10: setb cl
000000014166ef13: mov rdx, rax
000000014166ef16: sub rdx, r13
000000014166ef19: cmp rdx, 0x20
000000014166ef1d: setb dl
000000014166ef20: or dl, cl
000000014166ef22: mov rcx, r15
000000014166ef25: je 0x14166f199
000000014166ef2b: xor r9d, r9d
000000014166ef2e: mov r10, rcx
000000014166ef31: mov rdx, r13
000000014166ef34: mov r15d, esi
000000014166ef37: jmp 0x14166f1e0
000000014166ef3c: movups xmm0, xmmword ptr [rcx + 0x10]
000000014166ef40: movaps xmmword ptr [rbp + 0x930], xmm0
000000014166ef47: movups xmm0, xmmword ptr [rcx]
000000014166ef4a: movaps xmmword ptr [rbp + 0x8e0], xmm0
000000014166ef51: mov rax, qword ptr [rip + 0x1fbd450]
000000014166ef58: mov rax, qword ptr [rax]
000000014166ef5b: cmp rax, 3
000000014166ef5f: jb 0x14166f029
000000014166ef65: mov byte ptr [rbp + 0xe17], 1
000000014166ef6c: lea rcx, [rip + 0x1615835]
000000014166ef73: call 0x14067dd80
000000014166ef78: movups xmm0, xmmword ptr [rax]
000000014166ef7b: mov eax, dword ptr [rax + 0x10]
000000014166ef7e: mov qword ptr [rbp + 0xb00], 3
000000014166ef89: lea rcx, [rip + 0x1615760]
000000014166ef90: mov qword ptr [rbp + 0xb08], rcx
000000014166ef97: mov qword ptr [rbp + 0xb10], 0x26
000000014166efa2: lea rdx, [rip + 0x16157ef]
000000014166efa9: mov qword ptr [rbp + 0xb20], rdx
000000014166efb0: mov qword ptr [rbp + 0xb28], 1
000000014166efbb: mov qword ptr [rbp + 0xb30], 8
000000014166efc6: mov qword ptr [rbp + 0xad0], 0
000000014166efd1: xorps xmm1, xmm1
000000014166efd4: movups xmmword ptr [rbp + 0xb38], xmm1
000000014166efdb: mov qword ptr [rbp + 0xad8], rcx
000000014166efe2: mov qword ptr [rbp + 0xae0], 0x26
000000014166efed: mov qword ptr [rbp + 0xae8], 0
000000014166eff8: movups xmmword ptr [rbp + 0xaf0], xmm0
000000014166efff: mov dword ptr [rbp + 0xb18], 1
000000014166f009: mov dword ptr [rbp + 0xb1c], eax
000000014166f00f: mov byte ptr [rbp + 0xe17], 1
000000014166f016: lea rcx, [rbp + 0xe30]
000000014166f01d: lea rdx, [rbp + 0xad0]
000000014166f024: call 0x140d4a330
000000014166f029: lea rcx, [rbp + 0xad0]
000000014166f030: lea rdx, [rbp + 0x8e0]
000000014166f037: call 0x140b46000
000000014166f03c: lea rcx, [rbp + 0xad0]
000000014166f043: call 0x140d76ed0
000000014166f048: mov r8, rdx
000000014166f04b: mov qword ptr [rsp + 0x20], 9
000000014166f054: lea r9, [rip + 0x1480493]
000000014166f05b: lea rcx, [rbp + 0x660]
000000014166f062: mov rdx, rax
000000014166f065: call 0x140d62810
000000014166f06a: mov rdx, qword ptr [rbp + 0xad0]
000000014166f071: test rdx, rdx
000000014166f074: je 0x14166f088
000000014166f076: mov rcx, qword ptr [rbp + 0xad8]
000000014166f07d: mov r8d, 1
000000014166f083: call 0x140613c20
000000014166f088: movaps xmm0, xmmword ptr [rbp + 0x930]
000000014166f08f: movups xmm1, xmmword ptr [rbp + 0x660]
000000014166f096: movups xmm2, xmmword ptr [rbp + 0x670]
000000014166f09d: movaps xmmword ptr [rbp + 0xa00], xmm1
000000014166f0a4: movaps xmmword ptr [rbp + 0xa10], xmm2
000000014166f0ab: movaps xmmword ptr [rbp + 0xa20], xmm0
000000014166f0b2: movaps xmm0, xmmword ptr [rbp + 0x8e0]
000000014166f0b9: movaps xmmword ptr [rbp + 0xab0], xmm0
000000014166f0c0: movabs rax, 0x8000000000000000
000000014166f0ca: mov qword ptr [rbp + 0xa30], rax
000000014166f0d1: mov qword ptr [rbp + 0xa90], rax
000000014166f0d8: mov word ptr [rbp + 0xac0], 0
000000014166f0e1: jmp 0x14166f4a3
000000014166f0e6: mov eax, 1
000000014166f0eb: xor edx, edx
000000014166f0ed: cmp rdx, qword ptr [rbp + 0x678]
000000014166f0f4: jo 0x14166f232
000000014166f0fa: mov qword ptr [rbp + 0xdf8], rax
000000014166f101: mov qword ptr [rbp + 0xe08], rcx
000000014166f108: lea rdx, [rbp + 0x678]
000000014166f10f: mov bl, byte ptr [rbp + 0x6a0]
000000014166f115: lea rcx, [rbp + 0x3f0]
000000014166f11c: call 0x140d7bd90
000000014166f121: mov byte ptr [rbp + 0x418], bl
000000014166f127: movups xmm0, xmmword ptr [rbp + 0x690]
000000014166f12e: movups xmmword ptr [rbp + 0x408], xmm0
000000014166f135: movsd xmm1, qword ptr [rbp + 0x6a8]
000000014166f13d: movss xmm0, dword ptr [rbp + 0x6b0]
000000014166f145: movzx r8d, byte ptr [rbp + 0x6b4]
000000014166f14d: movzx r9d, byte ptr [rbp + 0x6b5]
000000014166f155: mov r12, qword ptr [rbp + 0x3f0]
000000014166f15c: movups xmm2, xmmword ptr [rbp + 0x3f8]
000000014166f163: movaps xmmword ptr [rbp + 0xc0], xmm2
000000014166f16a: movups xmm2, xmmword ptr [rbp + 0x408]
000000014166f171: movaps xmmword ptr [rbp + 0xd0], xmm2
000000014166f178: mov rdx, qword ptr [rbp + 0x418]
000000014166f17f: mov qword ptr [rbp + 0xe0], rdx
000000014166f186: mov rcx, qword ptr [rbp + 0xe08]
000000014166f18d: mov rax, qword ptr [rbp + 0xdf8]
000000014166f194: jmp 0x14166f232
000000014166f199: inc r9
000000014166f19c: mov r8d, r9d
000000014166f19f: and r8d, 0x1f
000000014166f1a3: mov edx, 0x20
000000014166f1a8: cmovne rdx, r8
000000014166f1ac: sub r9, rdx
000000014166f1af: mov r10, rcx
000000014166f1b2: sub r10, r9
000000014166f1b5: lea rdx, [r9 + r13]
000000014166f1b9: xor r8d, r8d
000000014166f1bc: mov r15d, esi
000000014166f1bf: nop 
000000014166f1c0: movups xmm0, xmmword ptr [r13 + r8]
000000014166f1c6: movups xmm1, xmmword ptr [r13 + r8 + 0x10]
000000014166f1cc: movups xmmword ptr [rax + r8], xmm0
000000014166f1d1: movups xmmword ptr [rax + r8 + 0x10], xmm1
000000014166f1d7: add r8, 0x20
000000014166f1db: cmp r9, r8
000000014166f1de: jne 0x14166f1c0
000000014166f1e0: mov rsi, r14
000000014166f1e3: add r13, rcx
000000014166f1e6: mov r8, rax
000000014166f1e9: add r8, r9
000000014166f1ec: xor r11d, r11d
000000014166f1ef: movzx r14d, byte ptr [rbp + 0xe08]
000000014166f1f7: nop word ptr [rax + rax]
000000014166f200: lea r9, [rdx + r11]
000000014166f204: cmp r9, r13
000000014166f207: je 0x14166f219
000000014166f209: movzx r9d, byte ptr [r9]
000000014166f20d: mov byte ptr [r8 + r11], r9b
000000014166f211: inc r11
000000014166f214: cmp r10, r11
000000014166f217: jne 0x14166f200
000000014166f219: movabs r13, 0x8000000000000001
000000014166f223: xor edx, edx
000000014166f225: cmp rdx, qword ptr [rbp + 0x678]
000000014166f22c: jno 0x14166f0fa
000000014166f232: mov byte ptr [rbp + 0xb28], r14b
000000014166f239: mov qword ptr [rbp + 0xad0], rcx
000000014166f240: mov qword ptr [rbp + 0xad8], rax
000000014166f247: mov qword ptr [rbp + 0xae0], rcx
000000014166f24e: mov qword ptr [rbp + 0xae8], r12
000000014166f255: movaps xmm2, xmmword ptr [rbp + 0xc0]
000000014166f25c: movaps xmm3, xmmword ptr [rbp + 0xd0]
000000014166f263: movups xmmword ptr [rbp + 0xaf0], xmm2
000000014166f26a: movups xmmword ptr [rbp + 0xb00], xmm3
000000014166f271: mov rdx, qword ptr [rbp + 0xe0]
000000014166f278: mov qword ptr [rbp + 0xb10], rdx
000000014166f27f: movlps qword ptr [rbp + 0xb18], xmm1
000000014166f286: movss dword ptr [rbp + 0xb20], xmm0
000000014166f28e: mov byte ptr [rbp + 0xb24], r8b
000000014166f295: mov byte ptr [rbp + 0xb25], r9b
000000014166f29c: mov r12, qword ptr [rbp + 0xe20]
000000014166f2a3: mov rax, r12
000000014166f2a6: neg rax
000000014166f2a9: jno 0x14166f388
000000014166f2af: mov eax, dword ptr [rbp + 0xb20]
000000014166f2b5: mov dword ptr [rbp + 0xa58], eax
000000014166f2bb: movzx eax, byte ptr [rbp + 0xb24]
000000014166f2c2: mov byte ptr [rbp + 0xa5c], al
000000014166f2c8: movzx eax, byte ptr [rbp + 0xb25]
000000014166f2cf: mov byte ptr [rbp + 0xa5d], al
000000014166f2d5: movzx eax, word ptr [rbp + 0xb26]
000000014166f2dc: mov word ptr [rbp + 0xa5e], ax
000000014166f2e3: movzx eax, byte ptr [rbp + 0xb28]
000000014166f2ea: mov byte ptr [rbp + 0xa60], al
000000014166f2f0: mov eax, dword ptr [rbp + 0xb29]
000000014166f2f6: mov dword ptr [rbp + 0xa61], eax
000000014166f2fc: movzx eax, word ptr [rbp + 0xb2d]
000000014166f303: mov word ptr [rbp + 0xa65], ax
000000014166f30a: movzx eax, byte ptr [rbp + 0xb2f]
000000014166f311: mov byte ptr [rbp + 0xa67], al
000000014166f317: movups xmm0, xmmword ptr [rbp + 0xb10]
000000014166f31e: movups xmmword ptr [rbp + 0xa48], xmm0
000000014166f325: movups xmm0, xmmword ptr [rbp + 0xad0]
000000014166f32c: movups xmm1, xmmword ptr [rbp + 0xae0]
000000014166f333: movups xmm2, xmmword ptr [rbp + 0xaf0]
000000014166f33a: movups xmm3, xmmword ptr [rbp + 0xb00]
000000014166f341: movups xmmword ptr [rbp + 0xa38], xmm3
000000014166f348: movups xmmword ptr [rbp + 0xa28], xmm2
000000014166f34f: movups xmmword ptr [rbp + 0xa18], xmm1
000000014166f356: movups xmmword ptr [rbp + 0xa08], xmm0
000000014166f35d: mov qword ptr [rbp + 0xa78], rdi
000000014166f364: mov byte ptr [rbp + 0xa80], r15b
000000014166f36b: mov qword ptr [rbp + 0xa00], r13
000000014166f372: movabs rax, 0x8000000000000000
000000014166f37c: mov qword ptr [rbp + 0xa68], rax
000000014166f383: jmp 0x14166f462
000000014166f388: test rdi, rdi
000000014166f38b: js 0x1416718a4
000000014166f391: je 0x14166f3bb
000000014166f393: movzx eax, byte ptr [rip + 0x1fc586f]
000000014166f39a: mov r14d, 1
000000014166f3a0: mov edx, 1
000000014166f3a5: mov rcx, rdi
000000014166f3a8: call 0x140613c10
000000014166f3ad: test rax, rax
000000014166f3b0: je 0x1416718a7
000000014166f3b6: mov rbx, rax
000000014166f3b9: jmp 0x14166f3c0
000000014166f3bb: mov ebx, 1
000000014166f3c0: mov rcx, rbx
000000014166f3c3: mov r14, qword ptr [rbp + 0xe18]
000000014166f3ca: mov rdx, r14
000000014166f3cd: mov r8, rdi
000000014166f3d0: call 0x142923c00
000000014166f3d5: movups xmm0, xmmword ptr [rbp + 0xad0]
000000014166f3dc: movups xmm1, xmmword ptr [rbp + 0xae0]
000000014166f3e3: movups xmm2, xmmword ptr [rbp + 0xaf0]
000000014166f3ea: movups xmm3, xmmword ptr [rbp + 0xb00]
000000014166f3f1: movups xmmword ptr [rbp + 0xa08], xmm0
000000014166f3f8: movups xmmword ptr [rbp + 0xa18], xmm1
000000014166f3ff: movups xmmword ptr [rbp + 0xa28], xmm2
000000014166f406: movups xmmword ptr [rbp + 0xa38], xmm3
000000014166f40d: movups xmm0, xmmword ptr [rbp + 0xb10]
000000014166f414: movups xmmword ptr [rbp + 0xa48], xmm0
000000014166f41b: movups xmm0, xmmword ptr [rbp + 0xb20]
000000014166f422: movups xmmword ptr [rbp + 0xa58], xmm0
000000014166f429: mov qword ptr [rbp + 0xa70], rbx
000000014166f430: mov qword ptr [rbp + 0xa78], rdi
000000014166f437: mov byte ptr [rbp + 0xa80], r15b
000000014166f43e: mov qword ptr [rbp + 0xa00], r13
000000014166f445: mov qword ptr [rbp + 0xa68], rdi
000000014166f44c: test r12, r12
000000014166f44f: je 0x14166f462
000000014166f451: mov r8d, 1
000000014166f457: mov rcx, r14
000000014166f45a: mov rdx, r12
000000014166f45d: call 0x140613c20
000000014166f462: mov rdx, qword ptr [rbp + 0x660]
000000014166f469: test rdx, rdx
000000014166f46c: je 0x14166f480
000000014166f46e: mov rcx, qword ptr [rbp + 0x668]
000000014166f475: mov r8d, 1
000000014166f47b: call 0x140613c20
000000014166f480: mov rdx, qword ptr [rbp + 0x678]
000000014166f487: mov rax, rdx
000000014166f48a: neg rax
000000014166f48d: jo 0x14166f4a3
000000014166f48f: jae 0x14166f4a3
000000014166f491: mov rcx, qword ptr [rbp + 0x680]
000000014166f498: mov r8d, 1
000000014166f49e: call 0x140613c20
000000014166f4a3: mov rax, qword ptr [rbp + 0xee8]
000000014166f4aa: mov rcx, qword ptr [rbp + 0x330]
000000014166f4b1: mov qword ptr [rbp + 0xd90], rcx
000000014166f4b8: movups xmm0, xmmword ptr [rbp + 0x320]
000000014166f4bf: movaps xmmword ptr [rbp + 0xd80], xmm0
000000014166f4c6: mov qword ptr [rbp + 0x320], 0
000000014166f4d1: mov qword ptr [rbp + 0x328], 8
000000014166f4dc: mov qword ptr [rbp + 0x330], 0
000000014166f4e7: mov rcx, qword ptr [rsi]
000000014166f4ea: mov qword ptr [rbp + 0x9b8], rcx
000000014166f4f1: lea rsi, [rbp + 0xa08]
000000014166f4f8: mov ecx, dword ptr [rax + 0x1c]
000000014166f4fb: mov dword ptr [rbp + 0xacc], ecx
000000014166f501: mov rcx, qword ptr [rax]
000000014166f504: mov rax, qword ptr [rax + 0x10]
000000014166f508: mov qword ptr [rbp + 0x8f8], rax
000000014166f50f: xorps xmm6, xmm6
000000014166f512: mov qword ptr [rbp + 0xde8], rcx
000000014166f519: lea rax, [rcx + 0x20]
000000014166f51d: mov qword ptr [rbp + 0x900], rax
000000014166f524: lea r15, [rbp - 0x20]
000000014166f528: lea rbx, [rbp + 0x660]
000000014166f52f: lea r14, [rbp + 0x3f0]
000000014166f536: nop word ptr cs:[rax + rax]
000000014166f540: mov r8d, 0xc8
000000014166f546: mov rcx, r15
000000014166f549: lea rdx, [rbp + 0xa00]
000000014166f550: call 0x142923c00
000000014166f555: movabs rcx, 0x8000000000000002
000000014166f55f: mov qword ptr [rbp + 0xa00], rcx
000000014166f566: mov rax, qword ptr [rbp - 0x20]
000000014166f56a: cmp rax, rcx
000000014166f56d: je 0x141671680
000000014166f573: mov rcx, qword ptr [rbp - 0x18]
000000014166f577: cmp rax, r13
000000014166f57a: jne 0x14166f900
000000014166f580: mov qword ptr [rbp + 0x950], rcx
000000014166f587: movups xmm0, xmmword ptr [rsi + 8]
000000014166f58b: movups xmm1, xmmword ptr [rsi + 0x18]
000000014166f58f: lea rcx, [rbp + 0x958]
000000014166f596: movups xmmword ptr [rcx + 0x10], xmm1
000000014166f59a: movups xmmword ptr [rcx], xmm0
000000014166f59d: mov rax, qword ptr [rsi + 0x58]
000000014166f5a1: mov qword ptr [rcx + 0x50], rax
000000014166f5a5: movups xmm0, xmmword ptr [rsi + 0x28]
000000014166f5a9: movups xmm1, xmmword ptr [rsi + 0x38]
000000014166f5ad: movups xmm2, xmmword ptr [rsi + 0x48]
000000014166f5b1: movups xmmword ptr [rcx + 0x40], xmm2
000000014166f5b5: movups xmmword ptr [rcx + 0x30], xmm1
000000014166f5b9: movups xmmword ptr [rcx + 0x20], xmm0
000000014166f5bd: mov rcx, qword ptr [rbp + 0xa68]
000000014166f5c4: mov rax, qword ptr [rbp + 0xa70]
000000014166f5cb: mov qword ptr [rbp + 0xde0], rax
000000014166f5d2: movups xmm0, xmmword ptr [rsi + 0x70]
000000014166f5d6: movaps xmmword ptr [rbp + 0x3c0], xmm0
000000014166f5dd: mov rax, qword ptr [rip + 0x1fbcdc4]
000000014166f5e4: mov rax, qword ptr [rax]
000000014166f5e7: cmp rax, 3
000000014166f5eb: mov qword ptr [rbp + 0xe20], rcx
000000014166f5f2: jb 0x14166f6e6
000000014166f5f8: mov qword ptr [rbp + 0xdf0], rcx
000000014166f5ff: mov byte ptr [rbp + 0xe37], 1
000000014166f606: mov byte ptr [rbp + 0xe36], 1
000000014166f60d: lea rcx, [rip + 0x1615314]
000000014166f614: call 0x14067dd80
000000014166f619: movups xmm0, xmmword ptr [rax]
000000014166f61c: mov eax, dword ptr [rax + 0x10]
000000014166f61f: mov qword ptr [rbp + 0xb00], 3
000000014166f62a: lea rdx, [rip + 0x16150bf]
000000014166f631: mov qword ptr [rbp + 0xb08], rdx
000000014166f638: mov qword ptr [rbp + 0xb10], 0x26
000000014166f643: lea rcx, [rip + 0x16152ce]
000000014166f64a: mov qword ptr [rbp + 0xb20], rcx
000000014166f651: mov qword ptr [rbp + 0xb28], 1
000000014166f65c: mov qword ptr [rbp + 0xb30], 8
000000014166f667: mov qword ptr [rbp + 0xad0], 0
000000014166f672: lea rcx, [rbp + 0xb38]
000000014166f679: movups xmmword ptr [rcx], xmm6
000000014166f67c: mov qword ptr [rbp + 0xad8], rdx
000000014166f683: mov qword ptr [rbp + 0xae0], 0x26
000000014166f68e: mov qword ptr [rbp + 0xae8], 0
000000014166f699: movups xmmword ptr [rbp + 0xaf0], xmm0
000000014166f6a0: mov dword ptr [rbp + 0xb18], 1
000000014166f6aa: mov dword ptr [rbp + 0xb1c], eax
000000014166f6b0: mov rax, qword ptr [rbp + 0xe20]
000000014166f6b7: mov qword ptr [rbp + 0xdf0], rax
000000014166f6be: mov byte ptr [rbp + 0xe37], 1
000000014166f6c5: mov byte ptr [rbp + 0xe36], 1
000000014166f6cc: lea rcx, [rbp + 0xe30]
000000014166f6d3: lea rdx, [rbp + 0xad0]
000000014166f6da: call 0x140d4a330
000000014166f6df: mov rcx, qword ptr [rbp + 0xe20]
000000014166f6e6: mov qword ptr [rbp + 0xdf0], rcx
000000014166f6ed: mov byte ptr [rbp + 0xe37], 1
000000014166f6f4: mov byte ptr [rbp + 0xe36], 1
000000014166f6fb: lea rcx, [rbp + 0xad0]
000000014166f702: call 0x140c102e0
000000014166f707: movups xmm0, xmmword ptr [rbp + 0xad0]
000000014166f70e: movaps xmmword ptr [rbp + 0x940], xmm0
000000014166f715: mov rax, qword ptr [rbp + 0xe20]
000000014166f71c: mov qword ptr [rbp + 0xdf0], rax
000000014166f723: mov byte ptr [rbp + 0xe37], 1
000000014166f72a: mov byte ptr [rbp + 0xe36], 1
000000014166f731: lea rcx, [rbp + 0xad0]
000000014166f738: lea rdx, [rbp + 0x940]
000000014166f73f: call 0x140b46000
000000014166f744: mov r12, rbx
000000014166f747: mov byte ptr [rbp + 0x660], 1
000000014166f74e: lea rsi, [rbp + 0xad0]
000000014166f755: mov rcx, rsi
000000014166f758: call 0x140d76ed0
000000014166f75d: mov r8, rdx
000000014166f760: mov rcx, r12
000000014166f763: mov rdx, rax
000000014166f766: call 0x140d5a150
000000014166f76b: mov rbx, rax
000000014166f76e: mov rdx, qword ptr [rbp + 0xad0]
000000014166f775: test rdx, rdx
000000014166f778: je 0x14166f78c
000000014166f77a: mov rcx, qword ptr [rbp + 0xad8]
000000014166f781: mov r8d, 1
000000014166f787: call 0x140613c20
000000014166f78c: test rbx, rbx
000000014166f78f: jne 0x141671706
000000014166f795: movaps xmm0, xmmword ptr [rbp + 0x940]
000000014166f79c: movaps xmmword ptr [rbp + 0xad0], xmm0
000000014166f7a3: mov rax, qword ptr [rbp + 0xe20]
000000014166f7aa: mov qword ptr [rbp + 0xdf0], rax
000000014166f7b1: mov byte ptr [rbp + 0xe37], 1
000000014166f7b8: mov byte ptr [rbp + 0xe36], 1
000000014166f7bf: lea rcx, [rbp + 0x950]
000000014166f7c6: mov rdx, rsi
000000014166f7c9: call 0x140b463c0
000000014166f7ce: test rax, rax
000000014166f7d1: jne 0x1416716cf
000000014166f7d7: mov rbx, r12
000000014166f7da: mov eax, dword ptr [rbp + 0xacc]
000000014166f7e0: mov rcx, qword ptr [rbp + 0x8f8]
000000014166f7e7: mov dword ptr [rcx], eax
000000014166f7e9: mov rax, qword ptr [rbp + 0xde8]
000000014166f7f0: mov rsi, qword ptr [rax + 0x30]
000000014166f7f4: mov rdi, qword ptr [rax + 0x40]
000000014166f7f8: cmp rsi, qword ptr [rax + 0x20]
000000014166f7fc: jne 0x14166f82d
000000014166f7fe: mov rax, qword ptr [rbp + 0xe20]
000000014166f805: mov qword ptr [rbp + 0xdf0], rax
000000014166f80c: mov byte ptr [rbp + 0xe37], 1
000000014166f813: mov byte ptr [rbp + 0xe36], 1
000000014166f81a: mov rcx, qword ptr [rbp + 0x900]
000000014166f821: lea rdx, [rip + 0x1619850]
000000014166f828: call 0x140562870
000000014166f82d: mov rcx, qword ptr [rbp + 0xde8]
000000014166f834: mov rax, qword ptr [rcx + 0x28]
000000014166f838: mov qword ptr [rax + rsi*8], rdi
000000014166f83c: inc rsi
000000014166f83f: mov qword ptr [rcx + 0x30], rsi
000000014166f843: inc qword ptr [rcx + 0x40]
000000014166f847: mov rcx, qword ptr [rbp + 0xe20]
000000014166f84e: mov rax, rcx
000000014166f851: neg rax
000000014166f854: seto byte ptr [rbp + 0xe06]
000000014166f85b: jno 0x14166fc70
000000014166f861: cmp byte ptr [rip + 0x1fc5528], 0
000000014166f868: lea rdx, [rip + 0x1615141]
000000014166f86f: lea rax, [rip + 0x1615155]
000000014166f876: cmovne rdx, rax
000000014166f87a: mov qword ptr [rbp + 0x8d0], rdx
000000014166f881: mov qword ptr [rbp + 0x8d8], 0x1b
000000014166f88c: mov rax, qword ptr [rbp + 0x9b8]
000000014166f893: cmp qword ptr [rax + 0x1d8], 0
000000014166f89b: je 0x14166fd2a
000000014166f8a1: movabs rax, 0x8000000000000000
000000014166f8ab: mov qword ptr [rbp + 0xdf0], rax
000000014166f8b2: mov byte ptr [rbp + 0xe37], 1
000000014166f8b9: mov byte ptr [rbp + 0xe36], 1
000000014166f8c0: mov r8d, 0x1b
000000014166f8c6: lea rcx, [rbp + 0xad0]
000000014166f8cd: call 0x140d68f80
000000014166f8d2: cmp dword ptr [rbp + 0xad0], 2
000000014166f8d9: jne 0x14166fd70
000000014166f8df: mov rax, qword ptr [rbp + 0xad8]
000000014166f8e6: mov qword ptr [rbp + 0x668], rax
000000014166f8ed: mov qword ptr [rbp + 0x660], 2
000000014166f8f8: jmp 0x14166fd85
000000014166f8fd: nop dword ptr [rax]
000000014166f900: movzx edi, byte ptr [rbp + 0xa0]
000000014166f907: movzx r12d, byte ptr [rbp + 0xa1]
000000014166f90f: movups xmm0, xmmword ptr [rsi + 0x78]
000000014166f913: movaps xmmword ptr [rbp + 0x110], xmm0
000000014166f91a: movups xmm0, xmmword ptr [rsi + 0x68]
000000014166f91e: movaps xmmword ptr [rbp + 0x100], xmm0
000000014166f925: movups xmm0, xmmword ptr [rsi + 0x28]
000000014166f929: movups xmm1, xmmword ptr [rsi + 0x38]
000000014166f92d: movups xmm2, xmmword ptr [rsi + 0x48]
000000014166f931: movups xmm3, xmmword ptr [rsi + 0x58]
000000014166f935: movaps xmmword ptr [rbp + 0xf0], xmm3
000000014166f93c: movaps xmmword ptr [rbp + 0xe0], xmm2
000000014166f943: movaps xmmword ptr [rbp + 0xd0], xmm1
000000014166f94a: movaps xmmword ptr [rbp + 0xc0], xmm0
000000014166f951: movups xmm0, xmmword ptr [rsi + 0xa8]
000000014166f958: movaps xmmword ptr [rbp + 0x920], xmm0
000000014166f95f: mov rdx, qword ptr [rbp + 0xa90]
000000014166f966: mov qword ptr [rbp + 0xe18], rdx
000000014166f96d: mov rdx, qword ptr [rbp + 0xa98]
000000014166f974: mov qword ptr [rbp + 0xe08], rdx
000000014166f97b: movups xmm0, xmmword ptr [rsi + 0x98]
000000014166f982: movaps xmmword ptr [rbp + 0x9c0], xmm0
000000014166f989: mov rdx, rax
000000014166f98c: neg rdx
000000014166f98f: jno 0x14167066c
000000014166f995: test cl, 1
000000014166f998: jne 0x1416708df
000000014166f99e: movups xmm0, xmmword ptr [rsi + 8]
000000014166f9a2: movups xmm1, xmmword ptr [rsi + 0x18]
000000014166f9a6: movaps xmmword ptr [rbp + 0x670], xmm1
000000014166f9ad: movaps xmmword ptr [rbp + 0x660], xmm0
000000014166f9b4: mov rcx, rbx
000000014166f9b7: call 0x140d76ed0
000000014166f9bc: lea rsi, [rbp + 0xad0]
000000014166f9c3: mov r8, rdx
000000014166f9c6: mov rcx, rsi
000000014166f9c9: mov rdx, rax
000000014166f9cc: call 0x140d6a710
000000014166f9d1: cmp dword ptr [rbp + 0xad0], 2
000000014166f9d8: jne 0x1416708f7
000000014166f9de: mov rax, qword ptr [rbp + 0xad8]
000000014166f9e5: mov ecx, eax
000000014166f9e7: and ecx, 3
000000014166f9ea: cmp ecx, 1
000000014166f9ed: jne 0x14166fa58
000000014166f9ef: lea rcx, [rax - 1]
000000014166f9f3: mov qword ptr [rbp + 0xde0], rcx
000000014166f9fa: mov rcx, qword ptr [rax - 1]
000000014166f9fe: mov qword ptr [rbp + 0xe20], rcx
000000014166fa05: mov rax, qword ptr [rax + 7]
000000014166fa09: mov qword ptr [rbp + 0xdf8], rax
000000014166fa10: mov rax, qword ptr [rax]
000000014166fa13: test rax, rax
000000014166fa16: je 0x14166fa21
000000014166fa18: mov rcx, qword ptr [rbp + 0xe20]
000000014166fa1f: call rax
000000014166fa21: mov rcx, qword ptr [rbp + 0xe20]
000000014166fa28: mov rax, qword ptr [rbp + 0xdf8]
000000014166fa2f: mov rdx, qword ptr [rax + 8]
000000014166fa33: test rdx, rdx
000000014166fa36: je 0x14166fa41
000000014166fa38: mov r8, qword ptr [rax + 0x10]
000000014166fa3c: call 0x140613c20
000000014166fa41: mov edx, 0x18
000000014166fa46: mov r8d, 8
000000014166fa4c: mov rcx, qword ptr [rbp + 0xde0]
000000014166fa53: call 0x140613c20
000000014166fa58: movaps xmm0, xmmword ptr [rbp + 0x660]
000000014166fa5f: movaps xmm1, xmmword ptr [rbp + 0x670]
000000014166fa66: movaps xmmword ptr [rbp + 0x400], xmm1
000000014166fa6d: movaps xmmword ptr [rbp + 0x3f0], xmm0
000000014166fa74: movaps xmmword ptr [rbp + 0x670], xmm1
000000014166fa7b: movaps xmmword ptr [rbp + 0x660], xmm0
000000014166fa82: mov rax, qword ptr [rip + 0x1fbc91f]
000000014166fa89: mov rax, qword ptr [rax]
000000014166fa8c: test rax, rax
000000014166fa8f: je 0x14166fb6b
000000014166fa95: mov qword ptr [rbp + 0xdc0], rbx
000000014166fa9c: lea rax, [rip - 0x90dfc3]
000000014166faa3: mov qword ptr [rbp + 0xdc8], rax
000000014166faaa: lea rcx, [rip + 0x1614d67]
000000014166fab1: call 0x14067dd80
000000014166fab6: movups xmm0, xmmword ptr [rax]
000000014166fab9: mov eax, dword ptr [rax + 0x10]
000000014166fabc: mov qword ptr [rbp + 0xb00], 1
000000014166fac7: lea rdx, [rip + 0x1614c22]
000000014166face: mov qword ptr [rbp + 0xb08], rdx
000000014166fad5: mov qword ptr [rbp + 0xb10], 0x26
000000014166fae0: lea rcx, [rip + 0x1614d11]
000000014166fae7: mov qword ptr [rbp + 0xb20], rcx
000000014166faee: mov qword ptr [rbp + 0xb28], 2
000000014166faf9: lea rcx, [rbp + 0xdc0]
000000014166fb00: mov qword ptr [rbp + 0xb30], rcx
000000014166fb07: mov qword ptr [rbp + 0xb38], 1
000000014166fb12: mov qword ptr [rbp + 0xb40], 0
000000014166fb1d: mov qword ptr [rbp + 0xad0], 0
000000014166fb28: mov qword ptr [rbp + 0xad8], rdx
000000014166fb2f: mov qword ptr [rbp + 0xae0], 0x26
000000014166fb3a: mov qword ptr [rbp + 0xae8], 0
000000014166fb45: movups xmmword ptr [rbp + 0xaf0], xmm0
000000014166fb4c: mov dword ptr [rbp + 0xb18], 1
000000014166fb56: mov dword ptr [rbp + 0xb1c], eax
000000014166fb5c: lea rcx, [rbp + 0xe30]
000000014166fb63: mov rdx, rsi
000000014166fb66: call 0x140d4a330
000000014166fb6b: mov rax, qword ptr [rbp + 0xd78]
000000014166fb72: movabs rdi, 0x8000000000000002
000000014166fb7c: test rax, rax
000000014166fb7f: je 0x14166fbe1
000000014166fb81: mov rcx, qword ptr [rbp + 0xd70]
000000014166fb88: lea rdx, [rcx + 1]
000000014166fb8c: mov r8, qword ptr [rbp + 0xd60]
000000014166fb93: mov r9, qword ptr [rbp + 0xd68]
000000014166fb9a: cmp rdx, r8
000000014166fb9d: mov edx, 0
000000014166fba2: cmovae rdx, r8
000000014166fba6: neg rdx
000000014166fba9: add rdx, rcx
000000014166fbac: inc rdx
000000014166fbaf: mov qword ptr [rbp + 0xd70], rdx
000000014166fbb6: dec rax
000000014166fbb9: mov qword ptr [rbp + 0xd78], rax
000000014166fbc0: imul rax, rcx, 0xc8
000000014166fbc7: mov rdi, qword ptr [r9 + rax]
000000014166fbcb: lea rdx, [r9 + rax]
000000014166fbcf: add rdx, 8
000000014166fbd3: mov r8d, 0xc0
000000014166fbd9: mov rcx, rsi
000000014166fbdc: call 0x142923c00
000000014166fbe1: mov qword ptr [rbp + 0xe20], rdi
000000014166fbe8: lea rcx, [rbp + 0xa00]
000000014166fbef: call 0x14166e5b0
000000014166fbf4: mov rax, qword ptr [rbp + 0xe20]
000000014166fbfb: mov qword ptr [rbp + 0xa00], rax
000000014166fc02: mov r8d, 0xc0
000000014166fc08: lea rsi, [rbp + 0xa08]
000000014166fc0f: mov rcx, rsi
000000014166fc12: lea rdx, [rbp + 0xad0]
000000014166fc19: call 0x142923c00
000000014166fc1e: mov rdx, qword ptr [rbp + 0x660]
000000014166fc25: test rdx, rdx
000000014166fc28: je 0x14166fc3c
000000014166fc2a: mov rcx, qword ptr [rbp + 0x668]
000000014166fc31: mov r8d, 1
000000014166fc37: call 0x140613c20
000000014166fc3c: mov rdx, qword ptr [rbp + 0xe18]
000000014166fc43: lea rax, [rdx*2]
000000014166fc4b: test rax, rax
000000014166fc4e: jne 0x14166fcc1
000000014166fc50: mov rdx, qword ptr [rbp + 0xc0]
000000014166fc57: mov rax, rdx
000000014166fc5a: neg rax
000000014166fc5d: jo 0x14166f540
000000014166fc63: jmp 0x14166fce6
000000014166fc68: nop dword ptr [rax + rax]
000000014166fc70: mov qword ptr [rbp + 0x9c0], rcx
000000014166fc77: mov rax, qword ptr [rbp + 0xde0]
000000014166fc7e: mov qword ptr [rbp + 0x9c8], rax
000000014166fc85: movaps xmm0, xmmword ptr [rbp + 0x3c0]
000000014166fc8c: lea rax, [rbp + 0x9d0]
000000014166fc93: movups xmmword ptr [rax], xmm0
000000014166fc96: lea r15, [rbp - 0x20]
000000014166fc9a: lea rsi, [rbp + 0xad0]
000000014166fca1: lea r12, [rbp + 0x9c0]
000000014166fca8: mov rax, qword ptr [rip + 0x1fbc6f9]
000000014166fcaf: mov rax, qword ptr [rax]
000000014166fcb2: cmp rax, 3
000000014166fcb6: jae 0x141670338
000000014166fcbc: jmp 0x141670415
000000014166fcc1: mov r8d, 1
000000014166fcc7: mov rcx, qword ptr [rbp + 0xe08]
000000014166fcce: call 0x140613c20
000000014166fcd3: mov rdx, qword ptr [rbp + 0xc0]
000000014166fcda: mov rax, rdx
000000014166fcdd: neg rax
000000014166fce0: jo 0x14166f540
000000014166fce6: jae 0x14166fcfa
000000014166fce8: mov rcx, qword ptr [rbp + 0xc8]
000000014166fcef: mov r8d, 1
000000014166fcf5: call 0x140613c20
000000014166fcfa: mov rdx, qword ptr [rbp + 0xd8]
000000014166fd01: mov rax, rdx
000000014166fd04: neg rax
000000014166fd07: jo 0x14166f540
000000014166fd0d: jae 0x14166f540
000000014166fd13: mov rcx, qword ptr [rbp + 0xe0]
000000014166fd1a: mov r8d, 1
000000014166fd20: call 0x140613c20
000000014166fd25: jmp 0x14166f540
000000014166fd2a: movabs rax, 0x8000000000000000
000000014166fd34: mov qword ptr [rbp + 0xdf0], rax
000000014166fd3b: mov byte ptr [rbp + 0xe37], 1
000000014166fd42: mov byte ptr [rbp + 0xe36], 1
000000014166fd49: mov qword ptr [rsp + 0x20], 5
000000014166fd52: mov r8d, 0x1b
000000014166fd58: lea rcx, [rbp + 0xda0]
000000014166fd5f: lea r9, [rip + 0x1614c80]
000000014166fd66: call 0x140d62810
000000014166fd6b: jmp 0x1416702ca
000000014166fd70: mov r8d, 0x270
000000014166fd76: mov rcx, rbx
000000014166fd79: lea rdx, [rbp + 0xad0]
000000014166fd80: call 0x142923c00
000000014166fd85: movabs rax, 0x8000000000000000
000000014166fd8f: mov qword ptr [rbp + 0xdf0], rax
000000014166fd96: mov byte ptr [rbp + 0xe37], 1
000000014166fd9d: mov byte ptr [rbp + 0xe36], 1
000000014166fda4: mov rcx, r14
000000014166fda7: mov rdx, rbx
000000014166fdaa: lea r8, [rbp + 0x8d0]
000000014166fdb1: call 0x1415a5b60
000000014166fdb6: cmp dword ptr [rbp + 0x3f0], 2
000000014166fdbd: je 0x14167173d
000000014166fdc3: mov r8d, 0x270
000000014166fdc9: lea rsi, [rbp + 0xad0]
000000014166fdd0: mov rcx, rsi
000000014166fdd3: mov rdx, r14
000000014166fdd6: call 0x142923c00
000000014166fddb: mov qword ptr [rbp + 0xd48], 0
000000014166fde6: mov qword ptr [rbp + 0xd50], 8
000000014166fdf1: mov qword ptr [rbp + 0xd58], 0
000000014166fdfc: lea rdi, [rbp + 0x668]
000000014166fe03: jmp 0x14166fe27
000000014166fe05: nop word ptr cs:[rax + rax]
000000014166fe10: mov edx, 0x18
000000014166fe15: mov r8d, 8
000000014166fe1b: mov rcx, qword ptr [rbp + 0xdf8]
000000014166fe22: call 0x140613c20
000000014166fe27: mov rcx, rbx
000000014166fe2a: mov rdx, rsi
000000014166fe2d: call 0x140d59e80
000000014166fe32: test byte ptr [rbp + 0x660], 1
000000014166fe39: je 0x141670108
000000014166fe3f: mov r8d, 0x258
000000014166fe45: lea rcx, [rbp + 0xc0]
000000014166fe4c: mov rdx, rdi
000000014166fe4f: call 0x142923c00
000000014166fe54: cmp qword ptr [rbp + 0xc0], 0
000000014166fe5c: je 0x141670071
000000014166fe62: mov r8d, 0x258
000000014166fe68: mov rcx, r14
000000014166fe6b: mov rdx, rdi
000000014166fe6e: call 0x142923c00
000000014166fe73: lea r15, [rbp + 0xdc0]
000000014166fe7a: mov rcx, r15
000000014166fe7d: mov rdx, r14
000000014166fe80: call 0x140d59f10
000000014166fe85: mov byte ptr [rbp + 0xe2e], 1
000000014166fe8c: mov rcx, r15
000000014166fe8f: call 0x140d76ed0
000000014166fe94: mov byte ptr [rbp + 0xe2e], 1
000000014166fe9b: mov rcx, rax
000000014166fe9e: call 0x140d62e10
000000014166fea3: test al, al
000000014166fea5: je 0x141670030
000000014166feab: mov byte ptr [rbp + 0xe2e], 1
000000014166feb2: lea rcx, [rbp + 0xdc0]
000000014166feb9: call 0x140d76ed0
000000014166febe: mov byte ptr [rbp + 0xe2e], 1
000000014166fec5: mov rcx, rax
000000014166fec8: call 0x140d61e00
000000014166fecd: test rax, rax
000000014166fed0: je 0x141671668
000000014166fed6: mov r8, rdx
000000014166fed9: mov byte ptr [rbp + 0xe2e], 1
000000014166fee0: lea rcx, [rbp + 0x9e0]
000000014166fee7: mov rdx, rax
000000014166feea: call 0x140d58e70
000000014166feef: test byte ptr [rbp + 0x9e0], 1
000000014166fef6: jne 0x14166ff21
000000014166fef8: cmp qword ptr [rbp + 0x9f0], 5
000000014166ff00: jne 0x14166ff21
000000014166ff02: mov rax, qword ptr [rbp + 0x9e8]
000000014166ff09: mov ecx, dword ptr [rax]
000000014166ff0b: mov edx, 0x72746e69
000000014166ff10: xor ecx, edx
000000014166ff12: movzx eax, byte ptr [rax + 4]
000000014166ff16: xor eax, 0x6f
000000014166ff19: or eax, ecx
000000014166ff1b: je 0x1416700e1
000000014166ff21: mov rax, qword ptr [rbp + 0xdc0]
000000014166ff28: mov qword ptr [rbp + 0xe18], rax
000000014166ff2f: mov rax, qword ptr [rbp + 0xdc8]
000000014166ff36: mov qword ptr [rbp + 0xe08], rax
000000014166ff3d: lea rax, [rbp + 0xdd0]
000000014166ff44: movups xmm0, xmmword ptr [rax]
000000014166ff47: movaps xmmword ptr [rbp + 0x9e0], xmm0
000000014166ff4e: mov rsi, qword ptr [rbp + 0xd58]
000000014166ff55: cmp rsi, qword ptr [rbp + 0xd48]
000000014166ff5c: jne 0x14166ff71
000000014166ff5e: lea rcx, [rbp + 0xd48]
000000014166ff65: lea rdx, [rip + 0x1614b2c]
000000014166ff6c: call 0x140562c30
000000014166ff71: mov rax, qword ptr [rbp + 0xd50]
000000014166ff78: mov rcx, rsi
000000014166ff7b: shl rcx, 5
000000014166ff7f: mov rdx, qword ptr [rbp + 0xe18]
000000014166ff86: mov qword ptr [rax + rcx], rdx
000000014166ff8a: mov rdx, qword ptr [rbp + 0xe08]
000000014166ff91: mov qword ptr [rax + rcx + 8], rdx
000000014166ff96: movaps xmm0, xmmword ptr [rbp + 0x9e0]
000000014166ff9d: movups xmmword ptr [rax + rcx + 0x10], xmm0
000000014166ffa2: inc rsi
000000014166ffa5: mov qword ptr [rbp + 0xd58], rsi
000000014166ffac: lea rsi, [rbp + 0xad0]
000000014166ffb3: mov rax, qword ptr [rbp + 0x3f0]
000000014166ffba: lock dec qword ptr [rax]
000000014166ffbe: jne 0x14166ffc8
000000014166ffc0: mov rcx, r14
000000014166ffc3: call 0x140d53bc0
000000014166ffc8: cmp qword ptr [rbp + 0xc0], 0
000000014166ffd0: jne 0x14166fe27
000000014166ffd6: mov rax, qword ptr [rbp + 0xc8]
000000014166ffdd: mov ecx, eax
000000014166ffdf: and ecx, 3
000000014166ffe2: cmp ecx, 1
000000014166ffe5: jne 0x14166fe27
000000014166ffeb: lea rcx, [rax - 1]
000000014166ffef: mov qword ptr [rbp + 0xdf8], rcx
000000014166fff6: mov rcx, qword ptr [rax - 1]
000000014166fffa: mov qword ptr [rbp + 0xe18], rcx
0000000141670001: mov rax, qword ptr [rax + 7]
0000000141670005: mov qword ptr [rbp + 0xe08], rax
000000014167000c: mov rax, qword ptr [rax]
000000014167000f: test rax, rax
0000000141670012: je 0x1416700b8
0000000141670018: mov rcx, qword ptr [rbp + 0xe18]
000000014167001f: call rax
0000000141670021: jmp 0x1416700b8
0000000141670026: nop word ptr cs:[rax + rax]
0000000141670030: mov rdx, qword ptr [rbp + 0xdc0]
0000000141670037: test rdx, rdx
000000014167003a: je 0x14167004e
000000014167003c: mov rcx, qword ptr [rbp + 0xdc8]
0000000141670043: mov r8d, 1
0000000141670049: call 0x140613c20
000000014167004e: mov rax, qword ptr [rbp + 0x3f0]
0000000141670055: lock dec qword ptr [rax]
0000000141670059: jne 0x141670063
000000014167005b: mov rcx, r14
000000014167005e: call 0x140d53bc0
0000000141670063: cmp qword ptr [rbp + 0xc0], 0
000000014167006b: jne 0x14166fe27
0000000141670071: mov rax, qword ptr [rbp + 0xc8]
0000000141670078: mov ecx, eax
000000014167007a: and ecx, 3
000000014167007d: cmp ecx, 1
0000000141670080: jne 0x14166fe27
0000000141670086: lea rcx, [rax - 1]
000000014167008a: mov qword ptr [rbp + 0xdf8], rcx
0000000141670091: mov rcx, qword ptr [rax - 1]
0000000141670095: mov qword ptr [rbp + 0xe18], rcx
000000014167009c: mov rax, qword ptr [rax + 7]
00000001416700a0: mov qword ptr [rbp + 0xe08], rax
00000001416700a7: mov rax, qword ptr [rax]
00000001416700aa: test rax, rax
00000001416700ad: je 0x1416700b8
00000001416700af: mov rcx, qword ptr [rbp + 0xe18]
00000001416700b6: call rax
00000001416700b8: mov rcx, qword ptr [rbp + 0xe18]
00000001416700bf: mov rax, qword ptr [rbp + 0xe08]
00000001416700c6: mov rdx, qword ptr [rax + 8]
00000001416700ca: test rdx, rdx
00000001416700cd: je 0x14166fe10
00000001416700d3: mov r8, qword ptr [rax + 0x10]
00000001416700d7: call 0x140613c20
00000001416700dc: jmp 0x14166fe10
00000001416700e1: mov rdx, qword ptr [rbp + 0xdc0]
00000001416700e8: test rdx, rdx
00000001416700eb: je 0x14166ffb3
00000001416700f1: mov rcx, qword ptr [rbp + 0xdc8]
00000001416700f8: mov r8d, 1
00000001416700fe: call 0x140613c20
0000000141670103: jmp 0x14166ffb3
0000000141670108: cmp qword ptr [rbp + 0xad0], 0
0000000141670110: je 0x14167011e
0000000141670112: lea rcx, [rbp + 0xad8]
0000000141670119: call 0x140d685b0
000000014167011e: mov rax, qword ptr [rbp + 0xae0]
0000000141670125: lock dec qword ptr [rax]
0000000141670129: jne 0x14167013e
000000014167012b: mov byte ptr [rbp + 0xe2f], 1
0000000141670132: lea rcx, [rbp + 0xae0]
0000000141670139: call 0x140d53bc0
000000014167013e: mov rbx, qword ptr [rbp + 0xd58]
0000000141670145: test rbx, rbx
0000000141670148: je 0x14167177b
000000014167014e: cmp rbx, 1
0000000141670152: jne 0x141670616
0000000141670158: mov rcx, qword ptr [rbp + 0xd58]
000000014167015f: test rcx, rcx
0000000141670162: je 0x1416717d9
0000000141670168: mov rax, qword ptr [rbp + 0x9b8]
000000014167016f: mov rax, qword ptr [rax + 0x1d8]
0000000141670176: dec rax
0000000141670179: mov r13, qword ptr [rbp + 0xd48]
0000000141670180: mov rbx, qword ptr [rbp + 0xd50]
0000000141670187: mov rdi, rcx
000000014167018a: shl rdi, 5
000000014167018e: add rdi, rbx
0000000141670191: mov qword ptr [rbp + 0xad0], rbx
0000000141670198: mov qword ptr [rbp + 0xae0], r13
000000014167019f: mov qword ptr [rbp + 0xae8], rdi
00000001416701a6: mov rdx, rax
00000001416701a9: or rdx, rcx
00000001416701ac: shr rdx, 0x20
00000001416701b0: je 0x1416701c1
00000001416701b2: xor edx, edx
00000001416701b4: div rcx
00000001416701b7: mov rsi, rdx
00000001416701ba: test rsi, rsi
00000001416701bd: jne 0x1416701cc
00000001416701bf: jmp 0x141670202
00000001416701c1: xor edx, edx
00000001416701c3: div ecx
00000001416701c5: mov esi, edx
00000001416701c7: test rsi, rsi
00000001416701ca: je 0x141670202
00000001416701cc: lea r15, [rbx + 8]
00000001416701d0: mov r14, rsi
00000001416701d3: jmp 0x1416701e9
00000001416701d5: nop word ptr cs:[rax + rax]
00000001416701e0: add r15, 0x20
00000001416701e4: dec r14
00000001416701e7: je 0x141670202
00000001416701e9: mov rdx, qword ptr [r15 - 8]
00000001416701ed: test rdx, rdx
00000001416701f0: je 0x1416701e0
00000001416701f2: mov rcx, qword ptr [r15]
00000001416701f5: mov r8d, 1
00000001416701fb: call 0x140613c20
0000000141670200: jmp 0x1416701e0
0000000141670202: shl rsi, 5
0000000141670206: add rsi, rbx
0000000141670209: lea r15, [rsi + 0x20]
000000014167020d: mov qword ptr [rbp + 0xad8], r15
0000000141670214: mov rax, qword ptr [rsi]
0000000141670217: mov rcx, qword ptr [rsi + 0x18]
000000014167021b: mov qword ptr [rbp + 0x670], rcx
0000000141670222: movups xmm0, xmmword ptr [rsi + 8]
0000000141670226: movaps xmmword ptr [rbp + 0x660], xmm0
000000014167022d: mov rcx, rax
0000000141670230: neg rcx
0000000141670233: jo 0x1416717c8
0000000141670239: mov qword ptr [rbp + 0xda0], rax
0000000141670240: mov rax, qword ptr [rbp + 0x670]
0000000141670247: lea rcx, [rbp + 0xda8]
000000014167024e: mov qword ptr [rcx + 0x10], rax
0000000141670252: movaps xmm0, xmmword ptr [rbp + 0x660]
0000000141670259: movups xmmword ptr [rcx], xmm0
000000014167025c: sub rdi, r15
000000014167025f: je 0x14167029c
0000000141670261: shr rdi, 5
0000000141670265: xor r14d, r14d
0000000141670268: jmp 0x141670278
000000014167026a: nop word ptr [rax + rax]
0000000141670270: inc r14
0000000141670273: dec rdi
0000000141670276: je 0x14167029c
0000000141670278: mov rax, r14
000000014167027b: shl rax, 5
000000014167027f: mov rdx, qword ptr [r15 + rax]
0000000141670283: test rdx, rdx
0000000141670286: je 0x141670270
0000000141670288: add rax, r15
000000014167028b: mov rcx, qword ptr [rax + 8]
000000014167028f: mov r8d, 1
0000000141670295: call 0x140613c20
000000014167029a: jmp 0x141670270
000000014167029c: test r13, r13
000000014167029f: je 0x1416702b6
00000001416702a1: shl r13, 5
00000001416702a5: mov r8d, 8
00000001416702ab: mov rcx, rbx
00000001416702ae: mov rdx, r13
00000001416702b1: call 0x140613c20
00000001416702b6: movabs r13, 0x8000000000000001
00000001416702c0: lea r14, [rbp + 0x3f0]
00000001416702c7: mov rbx, r12
00000001416702ca: lea rcx, [rbp + 0xda0]
00000001416702d1: call 0x140d76ed0
00000001416702d6: lea r12, [rbp + 0x9c0]
00000001416702dd: mov r8, rdx
00000001416702e0: mov qword ptr [rsp + 0x20], 0xc
00000001416702e9: mov rcx, r12
00000001416702ec: mov rdx, rax
00000001416702ef: lea r9, [rip + 0x16142f1]
00000001416702f6: call 0x140d62810
00000001416702fb: mov rdx, qword ptr [rbp + 0xda0]
0000000141670302: test rdx, rdx
0000000141670305: je 0x141670319
0000000141670307: mov rcx, qword ptr [rbp + 0xda8]
000000014167030e: mov r8d, 1
0000000141670314: call 0x140613c20
0000000141670319: lea r15, [rbp - 0x20]
000000014167031d: lea rsi, [rbp + 0xad0]
0000000141670324: mov rax, qword ptr [rip + 0x1fbc07d]
000000014167032b: mov rax, qword ptr [rax]
000000014167032e: cmp rax, 3
0000000141670332: jb 0x141670415
0000000141670338: mov qword ptr [rbp + 0x660], r12
000000014167033f: lea rax, [rip - 0x90e866]
0000000141670346: mov qword ptr [rbp + 0x668], rax
000000014167034d: mov byte ptr [rbp + 0xe31], 1
0000000141670354: lea rcx, [rip + 0x1614775]
000000014167035b: call 0x14067dd80
0000000141670360: movups xmm0, xmmword ptr [rax]
0000000141670363: mov eax, dword ptr [rax + 0x10]
0000000141670366: mov qword ptr [rbp + 0xb00], 3
0000000141670371: lea rdx, [rip + 0x1614378]
0000000141670378: mov qword ptr [rbp + 0xb08], rdx
000000014167037f: mov qword ptr [rbp + 0xb10], 0x26
000000014167038a: lea rcx, [rip + 0x161472f]
0000000141670391: mov qword ptr [rbp + 0xb20], rcx
0000000141670398: mov qword ptr [rbp + 0xb28], 1
00000001416703a3: mov qword ptr [rbp + 0xb30], rbx
00000001416703aa: mov qword ptr [rbp + 0xb38], 1
00000001416703b5: mov qword ptr [rbp + 0xb40], 0
00000001416703c0: mov qword ptr [rbp + 0xad0], 0
00000001416703cb: mov qword ptr [rbp + 0xad8], rdx
00000001416703d2: mov qword ptr [rbp + 0xae0], 0x26
00000001416703dd: mov qword ptr [rbp + 0xae8], 0
00000001416703e8: movups xmmword ptr [rbp + 0xaf0], xmm0
00000001416703ef: mov dword ptr [rbp + 0xb18], 1
00000001416703f9: mov dword ptr [rbp + 0xb1c], eax
00000001416703ff: mov byte ptr [rbp + 0xe31], 1
0000000141670406: lea rcx, [rbp + 0xe30]
000000014167040d: mov rdx, rsi
0000000141670410: call 0x140d4a330
0000000141670415: mov byte ptr [rbp + 0xe31], 1
000000014167041c: mov rcx, r12
000000014167041f: call 0x140d76ed0
0000000141670424: mov r8, rdx
0000000141670427: mov byte ptr [rbp + 0xe31], 1
000000014167042e: mov rcx, rsi
0000000141670431: mov rdx, rax
0000000141670434: call 0x140d6a710
0000000141670439: cmp dword ptr [rbp + 0xad0], 2
0000000141670440: je 0x141670698
0000000141670446: mov byte ptr [rbp + 0xe31], 1
000000014167044d: mov rcx, r12
0000000141670450: call 0x140d76ed0
0000000141670455: mov byte ptr [rbp + 0xe31], 1
000000014167045c: mov rcx, rax
000000014167045f: call 0x140d61cd0
0000000141670464: test rax, rax
0000000141670467: je 0x141670490
0000000141670469: mov r8, rdx
000000014167046c: mov byte ptr [rbp + 0xe31], 1
0000000141670473: mov qword ptr [rsp + 0x20], 9
000000014167047c: mov rcx, rbx
000000014167047f: mov rdx, rax
0000000141670482: lea r9, [rip + 0x161442f]
0000000141670489: call 0x140d62810
000000014167048e: jmp 0x1416704a1
0000000141670490: movabs rax, 0x8000000000000000
000000014167049a: mov qword ptr [rbp + 0x660], rax
00000001416704a1: mov rax, qword ptr [rbp + 0xe20]
00000001416704a8: neg rax
00000001416704ab: seto byte ptr [rbp + 0xe18]
00000001416704b2: movups xmm0, xmmword ptr [rbp + 0x9c0]
00000001416704b9: movups xmm1, xmmword ptr [rbp + 0x9d0]
00000001416704c0: movaps xmmword ptr [rbp + 0x400], xmm1
00000001416704c7: movaps xmmword ptr [rbp + 0x3f0], xmm0
00000001416704ce: movups xmm0, xmmword ptr [rbp + 0x950]
00000001416704d5: movups xmm1, xmmword ptr [rbp + 0x960]
00000001416704dc: movups xmm2, xmmword ptr [rbp + 0x970]
00000001416704e3: movups xmm3, xmmword ptr [rbp + 0x980]
00000001416704ea: movaps xmmword ptr [rbp + 0xad0], xmm0
00000001416704f1: movaps xmmword ptr [rbp + 0xae0], xmm1
00000001416704f8: movaps xmmword ptr [rbp + 0xaf0], xmm2
00000001416704ff: movaps xmmword ptr [rbp + 0xb00], xmm3
0000000141670506: movups xmm0, xmmword ptr [rbp + 0x990]
000000014167050d: movaps xmmword ptr [rbp + 0xb10], xmm0
0000000141670514: movups xmm0, xmmword ptr [rbp + 0x9a0]
000000014167051b: movaps xmmword ptr [rbp + 0xb20], xmm0
0000000141670522: movaps xmm0, xmmword ptr [rbp + 0x940]
0000000141670529: movaps xmmword ptr [rbp + 0xdc0], xmm0
0000000141670530: movups xmm0, xmmword ptr [rbp + 0x660]
0000000141670537: movups xmm1, xmmword ptr [rbp + 0x670]
000000014167053e: movaps xmmword ptr [rbp + 0xd0], xmm1
0000000141670545: movaps xmmword ptr [rbp + 0xc0], xmm0
000000014167054c: lea rcx, [rbp + 0xa00]
0000000141670553: call 0x14166e5b0
0000000141670558: movabs rax, 0x8000000000000000
0000000141670562: mov qword ptr [rbp + 0xa00], rax
0000000141670569: mov qword ptr [rbp + 0xa08], 1
0000000141670574: movaps xmm0, xmmword ptr [rbp + 0x3f0]
000000014167057b: movaps xmm1, xmmword ptr [rbp + 0x400]
0000000141670582: lea rsi, [rbp + 0xa08]
0000000141670589: movups xmmword ptr [rsi + 0x18], xmm1
000000014167058d: movups xmmword ptr [rsi + 8], xmm0
0000000141670591: movaps xmm0, xmmword ptr [rbp + 0xb20]
0000000141670598: movups xmmword ptr [rsi + 0x78], xmm0
000000014167059c: movaps xmm0, xmmword ptr [rbp + 0xb10]
00000001416705a3: movups xmmword ptr [rsi + 0x68], xmm0
00000001416705a7: movaps xmm0, xmmword ptr [rbp + 0xad0]
00000001416705ae: movaps xmm1, xmmword ptr [rbp + 0xae0]
00000001416705b5: movaps xmm2, xmmword ptr [rbp + 0xaf0]
00000001416705bc: movaps xmm3, xmmword ptr [rbp + 0xb00]
00000001416705c3: movups xmmword ptr [rsi + 0x58], xmm3
00000001416705c7: movups xmmword ptr [rsi + 0x48], xmm2
00000001416705cb: movups xmmword ptr [rsi + 0x38], xmm1
00000001416705cf: movups xmmword ptr [rsi + 0x28], xmm0
00000001416705d3: movaps xmm0, xmmword ptr [rbp + 0xc0]
00000001416705da: movaps xmm1, xmmword ptr [rbp + 0xd0]
00000001416705e1: movups xmmword ptr [rsi + 0x98], xmm1
00000001416705e8: movups xmmword ptr [rsi + 0x88], xmm0
00000001416705ef: movaps xmm0, xmmword ptr [rbp + 0xdc0]
00000001416705f6: movups xmmword ptr [rsi + 0xa8], xmm0
00000001416705fd: movzx eax, byte ptr [rbp + 0xe18]
0000000141670604: mov byte ptr [rbp + 0xac0], al
000000014167060a: mov byte ptr [rbp + 0xac1], 1
0000000141670611: jmp 0x14166f540
0000000141670616: mov r14, qword ptr [rbp + 0xd50]
000000014167061d: cmp rbx, 0x15
0000000141670621: jae 0x14167064e
0000000141670623: shl rbx, 5
0000000141670627: add rbx, r14
000000014167062a: lea r13, [r14 + 0x20]
000000014167062e: mov byte ptr [rbp + 0xe2f], 1
0000000141670635: mov rcx, r14
0000000141670638: mov rdx, r13
000000014167063b: call 0x1413320d0
0000000141670640: add r13, 0x20
0000000141670644: cmp r13, rbx
0000000141670647: jne 0x14167062e
0000000141670649: jmp 0x141670158
000000014167064e: mov byte ptr [rbp + 0xe2f], 1
0000000141670655: mov rcx, r14
0000000141670658: mov rdx, rbx
000000014167065b: lea r8, [rbp + 0xe30]
0000000141670662: call 0x140b4c390
0000000141670667: jmp 0x141670158
000000014167066c: movaps xmm0, xmmword ptr [rbp + 0xa20]
0000000141670673: movups xmmword ptr [rbp + 0x970], xmm0
000000014167067a: movups xmm0, xmmword ptr [rsi + 8]
000000014167067e: movups xmmword ptr [rbp + 0x960], xmm0
0000000141670685: mov qword ptr [rbp + 0x950], rax
000000014167068c: mov qword ptr [rbp + 0x958], rcx
0000000141670693: jmp 0x141670936
0000000141670698: mov rax, qword ptr [rbp + 0xad8]
000000014167069f: mov ecx, eax
00000001416706a1: and ecx, 3
00000001416706a4: cmp ecx, 1
00000001416706a7: jne 0x141670712
00000001416706a9: lea rcx, [rax - 1]
00000001416706ad: mov qword ptr [rbp + 0xdf8], rcx
00000001416706b4: mov rcx, qword ptr [rax - 1]
00000001416706b8: mov qword ptr [rbp + 0xe18], rcx
00000001416706bf: mov rax, qword ptr [rax + 7]
00000001416706c3: mov qword ptr [rbp + 0xe08], rax
00000001416706ca: mov rax, qword ptr [rax]
00000001416706cd: test rax, rax
00000001416706d0: je 0x1416706db
00000001416706d2: mov rcx, qword ptr [rbp + 0xe18]
00000001416706d9: call rax
00000001416706db: mov rcx, qword ptr [rbp + 0xe18]
00000001416706e2: mov rax, qword ptr [rbp + 0xe08]
00000001416706e9: mov rdx, qword ptr [rax + 8]
00000001416706ed: test rdx, rdx
00000001416706f0: je 0x1416706fb
00000001416706f2: mov r8, qword ptr [rax + 0x10]
00000001416706f6: call 0x140613c20
00000001416706fb: mov edx, 0x18
0000000141670700: mov r8d, 8
0000000141670706: mov rcx, qword ptr [rbp + 0xdf8]
000000014167070d: call 0x140613c20
0000000141670712: mov rcx, qword ptr [rbp + 0xd80]
0000000141670719: mov rax, qword ptr [rbp + 0xd88]
0000000141670720: mov rdx, qword ptr [rbp + 0xd90]
0000000141670727: movabs r8, 0x8000000000000000
0000000141670731: mov qword ptr [rbp + 0xd80], r8
0000000141670738: mov r8, rcx
000000014167073b: neg r8
000000014167073e: jo 0x141671849
0000000141670744: mov r12, rdx
0000000141670747: shl r12, 5
000000014167074b: add r12, rax
000000014167074e: mov qword ptr [rbp + 0x660], rax
0000000141670755: mov qword ptr [rbp + 0x668], rax
000000014167075c: mov qword ptr [rbp + 0x670], rcx
0000000141670763: mov qword ptr [rbp + 0x678], r12
000000014167076a: test rdx, rdx
000000014167076d: je 0x141670843
0000000141670773: mov rcx, qword ptr [rbp + 0xee0]
000000014167077a: mov r14d, dword ptr [rcx + 0x1c]
000000014167077e: mov rsi, qword ptr [rcx]
0000000141670781: mov r15, qword ptr [rcx + 0x10]
0000000141670785: lea rdi, [rbp + 0xad0]
000000014167078c: lea rbx, [rip + 0x15ea6a5]
0000000141670793: jmp 0x1416707ba
0000000141670795: nop word ptr cs:[rax + rax]
00000001416707a0: mov r13, qword ptr [rbp + 0x668]
00000001416707a7: mov r12, qword ptr [rbp + 0x678]
00000001416707ae: mov rax, r13
00000001416707b1: cmp r13, r12
00000001416707b4: je 0x141670843
00000001416707ba: lea r13, [rax + 0x20]
00000001416707be: mov qword ptr [rbp + 0x668], r13
00000001416707c5: mov rdx, qword ptr [rax]
00000001416707c8: mov rcx, qword ptr [rax + 8]
00000001416707cc: mov r8, rdx
00000001416707cf: neg r8
00000001416707d2: jo 0x141670820
00000001416707d4: movabs r8, 0x8000000000000001
00000001416707de: cmp rdx, r8
00000001416707e1: je 0x141670843
00000001416707e3: movups xmm0, xmmword ptr [rax + 0x10]
00000001416707e7: movaps xmmword ptr [rbp + 0x3f0], xmm0
00000001416707ee: mov dword ptr [r15], r14d
00000001416707f1: mov qword ptr [rbp + 0xad0], rdx
00000001416707f8: mov qword ptr [rbp + 0xad8], rcx
00000001416707ff: movaps xmm0, xmmword ptr [rbp + 0x3f0]
0000000141670806: lea rax, [rbp + 0xae0]
000000014167080d: movups xmmword ptr [rax], xmm0
0000000141670810: mov rcx, rsi
0000000141670813: mov rdx, rdi
0000000141670816: mov r8, rbx
0000000141670819: call 0x140b5a180
000000014167081e: jmp 0x1416707ae
0000000141670820: mov qword ptr [rbp + 0xad0], rcx
0000000141670827: xor eax, eax
0000000141670829: xchg byte ptr [rcx + 0x10], al
000000014167082c: lock dec qword ptr [rcx]
0000000141670830: jne 0x1416707a0
0000000141670836: mov rcx, rdi
0000000141670839: call 0x1402e5d80
000000014167083e: jmp 0x1416707a0
0000000141670843: mov byte ptr [rbp + 0xe31], 1
000000014167084a: lea rcx, [rbp + 0x660]
0000000141670851: call 0x140878540
0000000141670856: movups xmm0, xmmword ptr [rbp + 0x950]
000000014167085d: movups xmm1, xmmword ptr [rbp + 0x960]
0000000141670864: movups xmm2, xmmword ptr [rbp + 0x970]
000000014167086b: movups xmm3, xmmword ptr [rbp + 0x980]
0000000141670872: movaps xmmword ptr [rbp + 0xad0], xmm0
0000000141670879: movaps xmmword ptr [rbp + 0xae0], xmm1
0000000141670880: movaps xmmword ptr [rbp + 0xaf0], xmm2
0000000141670887: movaps xmmword ptr [rbp + 0xb00], xmm3
000000014167088e: movups xmm0, xmmword ptr [rbp + 0x990]
0000000141670895: movaps xmmword ptr [rbp + 0xb10], xmm0
000000014167089c: movups xmm0, xmmword ptr [rbp + 0x9a0]
00000001416708a3: movaps xmmword ptr [rbp + 0xb20], xmm0
00000001416708aa: movaps xmm0, xmmword ptr [rbp + 0x940]
00000001416708b1: movaps xmmword ptr [rbp + 0x910], xmm0
00000001416708b8: mov rdx, qword ptr [rbp + 0x9c0]
00000001416708bf: test rdx, rdx
00000001416708c2: je 0x1416711d7
00000001416708c8: mov rcx, qword ptr [rbp + 0x9c8]
00000001416708cf: mov r8d, 1
00000001416708d5: call 0x140613c20
00000001416708da: jmp 0x1416711d7
00000001416708df: movups xmm0, xmmword ptr [rsi + 8]
00000001416708e3: movups xmm1, xmmword ptr [rsi + 0x18]
00000001416708e7: movaps xmmword ptr [rbp + 0x400], xmm1
00000001416708ee: movaps xmmword ptr [rbp + 0x3f0], xmm0
00000001416708f5: jmp 0x141670913
00000001416708f7: movaps xmm0, xmmword ptr [rbp + 0x660]
00000001416708fe: movaps xmm1, xmmword ptr [rbp + 0x670]
0000000141670905: movaps xmmword ptr [rbp + 0x3f0], xmm0
000000014167090c: movaps xmmword ptr [rbp + 0x400], xmm1
0000000141670913: movaps xmm0, xmmword ptr [rbp + 0x3f0]
000000014167091a: movaps xmm1, xmmword ptr [rbp + 0x400]
0000000141670921: movups xmmword ptr [rbp + 0x968], xmm1
0000000141670928: movups xmmword ptr [rbp + 0x958], xmm0
000000014167092f: mov qword ptr [rbp + 0x950], r13
0000000141670936: mov rdx, qword ptr [rbp + 0xed0]
000000014167093d: mov eax, dword ptr [rdx + 0x1c]
0000000141670940: mov rcx, qword ptr [rdx + 0x10]
0000000141670944: mov dword ptr [rcx], eax
0000000141670946: mov rax, qword ptr [rdx]
0000000141670949: mov dword ptr [rax], 0xe
000000014167094f: movzx eax, byte ptr [rip + 0x1fc42b3]
0000000141670956: mov ecx, 0x20
000000014167095b: mov edx, 8
0000000141670960: call 0x140613c10
0000000141670965: test rax, rax
0000000141670968: je 0x1416717f1
000000014167096e: mov dword ptr [rax + 8], 0
0000000141670975: movabs r8, 0x8000000000000000
000000014167097f: mov qword ptr [rax], r8
0000000141670982: mov qword ptr [rbp + 0xda0], 1
000000014167098d: mov qword ptr [rbp + 0xda8], rax
0000000141670994: mov qword ptr [rbp + 0xdb0], 1
000000014167099f: mov rcx, qword ptr [rbp + 0xd80]
00000001416709a6: mov r9, qword ptr [rbp + 0xd88]
00000001416709ad: mov rdx, qword ptr [rbp + 0xd90]
00000001416709b4: mov qword ptr [rbp + 0xd80], r8
00000001416709bb: mov r8, rcx
00000001416709be: neg r8
00000001416709c1: jo 0x14167181a
00000001416709c7: mov r14, rdx
00000001416709ca: shl r14, 5
00000001416709ce: add r14, r9
00000001416709d1: mov qword ptr [rbp + 0x660], r9
00000001416709d8: mov qword ptr [rbp + 0x668], r9
00000001416709df: mov qword ptr [rbp + 0x670], rcx
00000001416709e6: mov qword ptr [rbp + 0x678], r14
00000001416709ed: test rdx, rdx
00000001416709f0: je 0x141670b25
00000001416709f6: mov byte ptr [rbp + 0xdf8], r12b
00000001416709fd: add r9, 0x20
0000000141670a01: mov r15d, 1
0000000141670a07: movabs r12, 0x8000000000000002
0000000141670a11: mov r8, r9
0000000141670a14: lea rsi, [rbp + 0xda0]
0000000141670a1b: lea rbx, [rip + 0x1613ea6]
0000000141670a22: jmp 0x141670a7b
0000000141670a24: nop word ptr cs:[rax + rax]
0000000141670a30: mov rax, qword ptr [rbp + 0xda8]
0000000141670a37: mov rcx, r15
0000000141670a3a: shl rcx, 5
0000000141670a3e: movups xmm0, xmmword ptr [rbp + 0xad0]
0000000141670a45: movups xmm1, xmmword ptr [rbp + 0xae0]
0000000141670a4c: movups xmmword ptr [rax + rcx + 0x10], xmm1
0000000141670a51: movups xmmword ptr [rax + rcx], xmm0
0000000141670a55: inc r15
0000000141670a58: mov qword ptr [rbp + 0xdb0], r15
0000000141670a5f: mov r8, qword ptr [rbp + 0xe20]
0000000141670a66: lea rcx, [r8 - 0x20]
0000000141670a6a: add r8, 0x20
0000000141670a6e: add rcx, 0x20
0000000141670a72: cmp rcx, r14
0000000141670a75: je 0x141670b16
0000000141670a7b: mov rdx, qword ptr [r8 - 0x20]
0000000141670a7f: mov qword ptr [rbp + 0xe20], r8
0000000141670a86: mov rcx, qword ptr [r8 - 0x18]
0000000141670a8a: mov r8, rdx
0000000141670a8d: neg r8
0000000141670a90: jo 0x141670ae0
0000000141670a92: cmp rdx, r13
0000000141670a95: je 0x141670b0f
0000000141670a97: mov r8, qword ptr [rbp + 0xe20]
0000000141670a9e: movups xmm0, xmmword ptr [r8 - 0x10]
0000000141670aa3: movaps xmmword ptr [rbp + 0xb0], xmm0
0000000141670aaa: mov qword ptr [rbp + 0xad0], rdx
0000000141670ab1: mov qword ptr [rbp + 0xad8], rcx
0000000141670ab8: lea rcx, [rbp + 0xae0]
0000000141670abf: movups xmmword ptr [rcx], xmm0
0000000141670ac2: cmp r15, qword ptr [rbp + 0xda0]
0000000141670ac9: jne 0x141670a37
0000000141670acf: mov rcx, rsi
0000000141670ad2: mov rdx, rbx
0000000141670ad5: call 0x140562c30
0000000141670ada: jmp 0x141670a30
0000000141670adf: nop 
0000000141670ae0: mov qword ptr [rbp + 0xad8], rcx
0000000141670ae7: mov qword ptr [rbp + 0xad0], r12
0000000141670aee: cmp r15, qword ptr [rbp + 0xda0]
0000000141670af5: jne 0x141670a37
0000000141670afb: mov rcx, rsi
0000000141670afe: lea rdx, [rip + 0x1613ddb]
0000000141670b05: call 0x140562c30
0000000141670b0a: jmp 0x141670a30
0000000141670b0f: mov r14, qword ptr [rbp + 0xe20]
0000000141670b16: mov qword ptr [rbp + 0x668], r14
0000000141670b1d: movzx r12d, byte ptr [rbp + 0xdf8]
0000000141670b25: mov byte ptr [rbp + 0xe34], 1
0000000141670b2c: mov byte ptr [rbp + 0xe33], 1
0000000141670b33: mov byte ptr [rbp + 0xe32], 1
0000000141670b3a: lea rcx, [rbp + 0x660]
0000000141670b41: call 0x140878540
0000000141670b46: mov rax, qword ptr [rip + 0x1fbb85b]
0000000141670b4d: mov rax, qword ptr [rax]
0000000141670b50: cmp rax, 4
0000000141670b54: jb 0x141670c3a
0000000141670b5a: mov byte ptr [rbp + 0xe34], 1
0000000141670b61: mov byte ptr [rbp + 0xe33], 1
0000000141670b68: mov byte ptr [rbp + 0xe32], 1
0000000141670b6f: lea rcx, [rip + 0x1613d2a]
0000000141670b76: call 0x14067dd80
0000000141670b7b: movups xmm0, xmmword ptr [rax]
0000000141670b7e: mov eax, dword ptr [rax + 0x10]
0000000141670b81: mov qword ptr [rbp + 0xb00], 4
0000000141670b8c: lea rdx, [rip + 0x1613b5d]
0000000141670b93: mov qword ptr [rbp + 0xb08], rdx
0000000141670b9a: mov qword ptr [rbp + 0xb10], 0x26
0000000141670ba5: lea rcx, [rip + 0x1613ce4]
0000000141670bac: mov qword ptr [rbp + 0xb20], rcx
0000000141670bb3: mov qword ptr [rbp + 0xb28], 1
0000000141670bbe: mov qword ptr [rbp + 0xb30], 8
0000000141670bc9: mov qword ptr [rbp + 0xad0], 0
0000000141670bd4: xorps xmm1, xmm1
0000000141670bd7: movups xmmword ptr [rbp + 0xb38], xmm1
0000000141670bde: mov qword ptr [rbp + 0xad8], rdx
0000000141670be5: mov qword ptr [rbp + 0xae0], 0x26
0000000141670bf0: mov qword ptr [rbp + 0xae8], 0
0000000141670bfb: movups xmmword ptr [rbp + 0xaf0], xmm0
0000000141670c02: mov dword ptr [rbp + 0xb18], 1
0000000141670c0c: mov dword ptr [rbp + 0xb1c], eax
0000000141670c12: mov byte ptr [rbp + 0xe34], 1
0000000141670c19: mov byte ptr [rbp + 0xe33], 1
0000000141670c20: mov byte ptr [rbp + 0xe32], 1
0000000141670c27: lea rcx, [rbp + 0xe30]
0000000141670c2e: lea rdx, [rbp + 0xad0]
0000000141670c35: call 0x140d4a330
0000000141670c3a: movups xmm0, xmmword ptr [rbp + 0x950]
0000000141670c41: movups xmm1, xmmword ptr [rbp + 0x960]
0000000141670c48: movups xmm2, xmmword ptr [rbp + 0x970]
0000000141670c4f: movups xmmword ptr [rbp + 0xb08], xmm2
0000000141670c56: movups xmmword ptr [rbp + 0xaf8], xmm1
0000000141670c5d: movups xmmword ptr [rbp + 0xae8], xmm0
0000000141670c64: and dil, 1
0000000141670c68: mov byte ptr [rbp + 0xde8], dil
0000000141670c6f: and r12b, 1
0000000141670c73: mov byte ptr [rbp + 0xdf8], r12b
0000000141670c7a: movups xmm0, xmmword ptr [rbp + 0xda0]
0000000141670c81: movaps xmmword ptr [rbp + 0xad0], xmm0
0000000141670c88: mov rax, qword ptr [rbp + 0xdb0]
0000000141670c8f: mov qword ptr [rbp + 0xae0], rax
0000000141670c96: mov rdx, qword ptr [rbp + 0xec8]
0000000141670c9d: mov eax, dword ptr [rdx + 0x1c]
0000000141670ca0: mov rcx, qword ptr [rdx + 0x10]
0000000141670ca4: mov dword ptr [rcx], eax
0000000141670ca6: mov rcx, qword ptr [rdx]
0000000141670ca9: mov qword ptr [rbp + 0xe20], rcx
0000000141670cb0: call 0x140890130
0000000141670cb5: mov rax, qword ptr [rbp + 0xb10]
0000000141670cbc: mov rcx, qword ptr [rbp + 0xe20]
0000000141670cc3: mov qword ptr [rcx + 0x40], rax
0000000141670cc7: movaps xmm0, xmmword ptr [rbp + 0xad0]
0000000141670cce: movaps xmm1, xmmword ptr [rbp + 0xae0]
0000000141670cd5: movaps xmm2, xmmword ptr [rbp + 0xaf0]
0000000141670cdc: movaps xmm3, xmmword ptr [rbp + 0xb00]
0000000141670ce3: movups xmmword ptr [rcx + 0x30], xmm3
0000000141670ce7: movups xmmword ptr [rcx + 0x20], xmm2
0000000141670ceb: movups xmmword ptr [rcx + 0x10], xmm1
0000000141670cef: movups xmmword ptr [rcx], xmm0
0000000141670cf2: mov byte ptr [rcx + 0x48], 0
0000000141670cf6: movzx eax, byte ptr [rbp + 0xe07]
0000000141670cfd: mov byte ptr [rcx + 0x49], al
0000000141670d00: movzx eax, byte ptr [rbp + 0xde8]
0000000141670d07: mov byte ptr [rcx + 0x4a], al
0000000141670d0a: movzx eax, byte ptr [rbp + 0xdf8]
0000000141670d11: mov byte ptr [rcx + 0x4b], al
0000000141670d14: mov qword ptr [rcx + 0x50], 0
0000000141670d1c: xor eax, eax
0000000141670d1e: cmp rax, qword ptr [rbp + 0xc0]
0000000141670d25: seto bl
0000000141670d28: jno 0x141670faf
0000000141670d2e: movaps xmm0, xmmword ptr [rbp + 0x920]
0000000141670d35: movaps xmmword ptr [rbp + 0xdc0], xmm0
0000000141670d3c: mov byte ptr [rbp + 0xe34], 1
0000000141670d43: mov byte ptr [rbp + 0xe33], 0
0000000141670d4a: mov byte ptr [rbp + 0xe32], 1
0000000141670d51: lea rcx, [rbp + 0x3f0]
0000000141670d58: lea rsi, [rbp + 0xdc0]
0000000141670d5f: mov rdx, rsi
0000000141670d62: call 0x140b46000
0000000141670d67: lea rdi, [rbp + 0x3f0]
0000000141670d6e: mov rcx, rdi
0000000141670d71: call 0x140d76ed0
0000000141670d76: mov r8, rdx
0000000141670d79: mov qword ptr [rsp + 0x20], 9
0000000141670d82: lea r9, [rip + 0x1613edf]
0000000141670d89: lea rcx, [rbp + 0xad0]
0000000141670d90: mov rdx, rax
0000000141670d93: call 0x140d62810
0000000141670d98: lea rcx, [rbp + 0xad0]
0000000141670d9f: call 0x140d76ed0
0000000141670da4: mov r8, rdx
0000000141670da7: lea rcx, [rbp + 0x660]
0000000141670dae: mov rdx, rax
0000000141670db1: call 0x140b46480
0000000141670db6: mov rdx, qword ptr [rbp + 0xad0]
0000000141670dbd: test rdx, rdx
0000000141670dc0: je 0x141670dd4
0000000141670dc2: mov rcx, qword ptr [rbp + 0xad8]
0000000141670dc9: mov r8d, 1
0000000141670dcf: call 0x140613c20
0000000141670dd4: mov rdx, qword ptr [rbp + 0x3f0]
0000000141670ddb: test rdx, rdx
0000000141670dde: je 0x141670df2
0000000141670de0: mov rcx, qword ptr [rbp + 0x3f8]
0000000141670de7: mov r8d, 1
0000000141670ded: call 0x140613c20
0000000141670df2: xor eax, eax
0000000141670df4: cmp rax, qword ptr [rbp + 0x660]
0000000141670dfb: jno 0x141670fe9
0000000141670e01: mov rax, qword ptr [rbp + 0x668]
0000000141670e08: mov qword ptr [rbp + 0x9e0], rax
0000000141670e0f: mov rax, qword ptr [rip + 0x1fbb592]
0000000141670e16: mov rax, qword ptr [rax]
0000000141670e19: cmp rax, 1
0000000141670e1d: jbe 0x141670f3d
0000000141670e23: lea rax, [rbp + 0x9e0]
0000000141670e2a: mov qword ptr [rbp + 0xdc0], rax
0000000141670e31: lea rax, [rip - 0x924228]
0000000141670e38: mov qword ptr [rbp + 0xdc8], rax
0000000141670e3f: mov qword ptr [rbp + 0x3f0], 2
0000000141670e4a: mov qword ptr [rbp + 0x400], 2
0000000141670e55: mov qword ptr [rbp + 0x410], 0
0000000141670e60: movabs rax, 0x400000020
0000000141670e6a: mov qword ptr [rbp + 0x418], rax
0000000141670e71: mov byte ptr [rbp + 0x420], 3
0000000141670e78: lea rcx, [rip + 0x1613dd1]
0000000141670e7f: call 0x14067dd80
0000000141670e84: movups xmm0, xmmword ptr [rax]
0000000141670e87: mov eax, dword ptr [rax + 0x10]
0000000141670e8a: mov qword ptr [rbp + 0xb00], 2
0000000141670e95: lea rdx, [rip + 0x1613854]
0000000141670e9c: mov qword ptr [rbp + 0xb08], rdx
0000000141670ea3: mov qword ptr [rbp + 0xb10], 0x26
0000000141670eae: lea rcx, [rip + 0x1613d8b]
0000000141670eb5: mov qword ptr [rbp + 0xb20], rcx
0000000141670ebc: mov qword ptr [rbp + 0xb28], 1
0000000141670ec7: mov qword ptr [rbp + 0xb30], rsi
0000000141670ece: mov qword ptr [rbp + 0xb38], 1
0000000141670ed9: mov qword ptr [rbp + 0xb40], rdi
0000000141670ee0: mov qword ptr [rbp + 0xb48], 1
0000000141670eeb: mov qword ptr [rbp + 0xad0], 0
0000000141670ef6: mov qword ptr [rbp + 0xad8], rdx
0000000141670efd: mov qword ptr [rbp + 0xae0], 0x26
0000000141670f08: mov qword ptr [rbp + 0xae8], 0
0000000141670f13: movups xmmword ptr [rbp + 0xaf0], xmm0
0000000141670f1a: mov dword ptr [rbp + 0xb18], 1
0000000141670f24: mov dword ptr [rbp + 0xb1c], eax
0000000141670f2a: lea rcx, [rbp + 0xe30]
0000000141670f31: lea rdx, [rbp + 0xad0]
0000000141670f38: call 0x140d4a330
0000000141670f3d: mov byte ptr [rbp + 0x448], 0
0000000141670f44: mov qword ptr [rbp + 0x3f0], 0
0000000141670f4f: mov qword ptr [rbp + 0x3f8], 1
0000000141670f5a: mov qword ptr [rbp + 0x400], 0
0000000141670f65: movabs rax, 0x8000000000000000
0000000141670f6f: mov qword ptr [rbp + 0x408], rax
0000000141670f76: mov byte ptr [rbp + 0xe34], 1
0000000141670f7d: mov byte ptr [rbp + 0xe33], 0
0000000141670f84: mov byte ptr [rbp + 0xe32], 1
0000000141670f8b: lea rcx, [rbp + 0x9e0]
0000000141670f92: call 0x140d4cc30
0000000141670f97: mov rcx, qword ptr [rbp + 0xe18]
0000000141670f9e: mov rax, rcx
0000000141670fa1: neg rax
0000000141670fa4: jno 0x141671050
0000000141670faa: jmp 0x141671175
0000000141670faf: movaps xmm0, xmmword ptr [rbp + 0x110]
0000000141670fb6: movaps xmmword ptr [rbp + 0x440], xmm0
0000000141670fbd: movaps xmm0, xmmword ptr [rbp + 0x100]
0000000141670fc4: movaps xmmword ptr [rbp + 0x430], xmm0
0000000141670fcb: movaps xmm0, xmmword ptr [rbp + 0xc0]
0000000141670fd2: movaps xmm1, xmmword ptr [rbp + 0xd0]
0000000141670fd9: movaps xmm2, xmmword ptr [rbp + 0xe0]
0000000141670fe0: movaps xmm3, xmmword ptr [rbp + 0xf0]
0000000141670fe7: jmp 0x141671021
0000000141670fe9: movups xmm0, xmmword ptr [rbp + 0x6b0]
0000000141670ff0: movaps xmmword ptr [rbp + 0x440], xmm0
0000000141670ff7: movups xmm0, xmmword ptr [rbp + 0x6a0]
0000000141670ffe: movaps xmmword ptr [rbp + 0x430], xmm0
0000000141671005: movups xmm0, xmmword ptr [rbp + 0x660]
000000014167100c: movups xmm1, xmmword ptr [rbp + 0x670]
0000000141671013: movups xmm2, xmmword ptr [rbp + 0x680]
000000014167101a: movups xmm3, xmmword ptr [rbp + 0x690]
0000000141671021: movaps xmmword ptr [rbp + 0x420], xmm3
0000000141671028: movaps xmmword ptr [rbp + 0x410], xmm2
000000014167102f: movaps xmmword ptr [rbp + 0x400], xmm1
0000000141671036: movaps xmmword ptr [rbp + 0x3f0], xmm0
000000014167103d: mov rcx, qword ptr [rbp + 0xe18]
0000000141671044: mov rax, rcx
0000000141671047: neg rax
000000014167104a: jo 0x141671175
0000000141671050: mov byte ptr [rbp + 0xdf8], bl
0000000141671056: mov qword ptr [rbp + 0xdc0], rcx
000000014167105d: mov rax, qword ptr [rbp + 0xe08]
0000000141671064: mov qword ptr [rbp + 0xdc8], rax
000000014167106b: movaps xmm0, xmmword ptr [rbp + 0x9c0]
0000000141671072: movups xmmword ptr [rbp + 0xdd0], xmm0
0000000141671079: lea rcx, [rbp + 0xad0]
0000000141671080: lea rdx, [rbp + 0x920]
0000000141671087: call 0x140b46000
000000014167108c: mov byte ptr [rbp + 0xe13], 1
0000000141671093: lea rcx, [rbp + 0xad0]
000000014167109a: call 0x140d76ed0
000000014167109f: mov r8, rdx
00000001416710a2: mov byte ptr [rbp + 0xe13], 1
00000001416710a9: mov qword ptr [rsp + 0x20], 9
00000001416710b2: lea r9, [rip + 0x16137ff]
00000001416710b9: lea rcx, [rbp + 0x660]
00000001416710c0: mov rdx, rax
00000001416710c3: call 0x140d62810
00000001416710c8: mov byte ptr [rbp + 0xe13], 0
00000001416710cf: lea rcx, [rbp + 0xdc0]
00000001416710d6: lea rdx, [rbp + 0x660]
00000001416710dd: call 0x141740c00
00000001416710e2: mov ecx, eax
00000001416710e4: and ecx, 3
00000001416710e7: cmp ecx, 1
00000001416710ea: jne 0x141671157
00000001416710ec: mov rcx, rax
00000001416710ef: dec rcx
00000001416710f2: mov qword ptr [rbp + 0xde0], rcx
00000001416710f9: mov rcx, qword ptr [rax - 1]
00000001416710fd: mov qword ptr [rbp + 0xe20], rcx
0000000141671104: mov rax, qword ptr [rax + 7]
0000000141671108: mov qword ptr [rbp + 0xde8], rax
000000014167110f: mov rax, qword ptr [rax]
0000000141671112: test rax, rax
0000000141671115: je 0x141671120
0000000141671117: mov rcx, qword ptr [rbp + 0xe20]
000000014167111e: call rax
0000000141671120: mov rcx, qword ptr [rbp + 0xe20]
0000000141671127: mov rax, qword ptr [rbp + 0xde8]
000000014167112e: mov rdx, qword ptr [rax + 8]
0000000141671132: test rdx, rdx
0000000141671135: je 0x141671140
0000000141671137: mov r8, qword ptr [rax + 0x10]
000000014167113b: call 0x140613c20
0000000141671140: mov edx, 0x18
0000000141671145: mov r8d, 8
000000014167114b: mov rcx, qword ptr [rbp + 0xde0]
0000000141671152: call 0x140613c20
0000000141671157: mov rdx, qword ptr [rbp + 0xad0]
000000014167115e: test rdx, rdx
0000000141671161: je 0x141671175
0000000141671163: mov rcx, qword ptr [rbp + 0xad8]
000000014167116a: mov r8d, 1
0000000141671170: call 0x140613c20
0000000141671175: movaps xmm0, xmmword ptr [rbp + 0x440]
000000014167117c: movaps xmmword ptr [rbp + 0xb20], xmm0
0000000141671183: movaps xmm0, xmmword ptr [rbp + 0x430]
000000014167118a: movaps xmmword ptr [rbp + 0xb10], xmm0
0000000141671191: movaps xmm0, xmmword ptr [rbp + 0x3f0]
0000000141671198: movaps xmm1, xmmword ptr [rbp + 0x400]
000000014167119f: movaps xmm2, xmmword ptr [rbp + 0x410]
00000001416711a6: movaps xmm3, xmmword ptr [rbp + 0x420]
00000001416711ad: movaps xmmword ptr [rbp + 0xb00], xmm3
00000001416711b4: movaps xmmword ptr [rbp + 0xaf0], xmm2
00000001416711bb: movaps xmmword ptr [rbp + 0xae0], xmm1
00000001416711c2: movaps xmmword ptr [rbp + 0xad0], xmm0
00000001416711c9: movaps xmm0, xmmword ptr [rbp + 0x920]
00000001416711d0: movaps xmmword ptr [rbp + 0x910], xmm0
00000001416711d7: movaps xmm0, xmmword ptr [rbp + 0xb20]
00000001416711de: movaps xmmword ptr [rbp + 0x6b0], xmm0
00000001416711e5: movaps xmm0, xmmword ptr [rbp + 0xb10]
00000001416711ec: movaps xmmword ptr [rbp + 0x6a0], xmm0
00000001416711f3: movaps xmm0, xmmword ptr [rbp + 0xad0]
00000001416711fa: movaps xmm1, xmmword ptr [rbp + 0xae0]
0000000141671201: movaps xmm2, xmmword ptr [rbp + 0xaf0]
0000000141671208: movaps xmm3, xmmword ptr [rbp + 0xb00]
000000014167120f: movaps xmmword ptr [rbp + 0x690], xmm3
0000000141671216: movaps xmmword ptr [rbp + 0x680], xmm2
000000014167121d: movaps xmmword ptr [rbp + 0x670], xmm1
0000000141671224: movaps xmmword ptr [rbp + 0x660], xmm0
000000014167122b: xor eax, eax
000000014167122d: cmp rax, qword ptr [rbp + 0xd80]
0000000141671234: jno 0x141671644
000000014167123a: movzx eax, byte ptr [rbp + 0x6b8]
0000000141671241: cmp byte ptr [rip + 0x1fc3b48], 0
0000000141671248: je 0x141671383
000000014167124e: test al, al
0000000141671250: je 0x14167136e
0000000141671256: mov rax, qword ptr [rip + 0x1fbb14b]
000000014167125d: mov rax, qword ptr [rax]
0000000141671260: cmp rax, 3
0000000141671264: jb 0x14167136e
000000014167126a: lea rax, [rbp + 0x6b8]
0000000141671271: mov qword ptr [rbp + 0x3f0], rax
0000000141671278: lea rax, [rip + 0x2181]
000000014167127f: mov qword ptr [rbp + 0x3f8], rax
0000000141671286: lea rcx, [rip + 0x16138b3]
000000014167128d: mov qword ptr [rbp + 0x400], rcx
0000000141671294: mov qword ptr [rbp + 0x408], rax
000000014167129b: mov byte ptr [rbp + 0xe2d], 1
00000001416712a2: lea rcx, [rip + 0x16138cf]
00000001416712a9: call 0x14067dd80
00000001416712ae: movups xmm0, xmmword ptr [rax]
00000001416712b1: mov eax, dword ptr [rax + 0x10]
00000001416712b4: mov qword ptr [rbp + 0xb00], 3
00000001416712bf: lea rdx, [rip + 0x161342a]
00000001416712c6: mov qword ptr [rbp + 0xb08], rdx
00000001416712cd: mov qword ptr [rbp + 0xb10], 0x26
00000001416712d8: lea rcx, [rip + 0x1613879]
00000001416712df: mov qword ptr [rbp + 0xb20], rcx
00000001416712e6: mov qword ptr [rbp + 0xb28], 2
00000001416712f1: lea rcx, [rbp + 0x3f0]
00000001416712f8: mov qword ptr [rbp + 0xb30], rcx
00000001416712ff: mov qword ptr [rbp + 0xb38], 2
000000014167130a: mov qword ptr [rbp + 0xb40], 0
0000000141671315: mov qword ptr [rbp + 0xad0], 0
0000000141671320: mov qword ptr [rbp + 0xad8], rdx
0000000141671327: mov qword ptr [rbp + 0xae0], 0x26
0000000141671332: mov qword ptr [rbp + 0xae8], 0
000000014167133d: movups xmmword ptr [rbp + 0xaf0], xmm0
0000000141671344: mov dword ptr [rbp + 0xb18], 1
000000014167134e: mov dword ptr [rbp + 0xb1c], eax
0000000141671354: mov byte ptr [rbp + 0xe2d], 1
000000014167135b: lea rcx, [rbp + 0xe30]
0000000141671362: lea rdx, [rbp + 0xad0]
0000000141671369: call 0x140d4a330
000000014167136e: mov byte ptr [rbp + 0x6b8], 0
0000000141671375: mov eax, 6
000000014167137a: lea r9, [rip + 0x147e14f]
0000000141671381: jmp 0x14167139c
0000000141671383: movzx eax, al
0000000141671386: lea rcx, [rip + 0x1613933]
000000014167138d: mov r9, qword ptr [rcx + rax*8]
0000000141671391: lea rcx, [rip + 0x1613950]
0000000141671398: mov rax, qword ptr [rcx + rax*8]
000000014167139c: mov byte ptr [rbp + 0xe2d], 1
00000001416713a3: mov qword ptr [rsp + 0x20], rax
00000001416713a8: lea rdx, [rip + 0x147e16b]
00000001416713af: lea rcx, [rbp + 0x3f0]
00000001416713b6: mov r8d, 0xc
00000001416713bc: call 0x140d62810
00000001416713c1: mov byte ptr [rbp + 0xe2d], 1
00000001416713c8: lea rcx, [rbp + 0xad0]
00000001416713cf: lea rdx, [rbp + 0x3f0]
00000001416713d6: call 0x1408bbbc0
00000001416713db: mov rdx, qword ptr [rbp + 0xed8]
00000001416713e2: mov eax, dword ptr [rdx + 0x1c]
00000001416713e5: mov rcx, qword ptr [rdx + 0x10]
00000001416713e9: mov dword ptr [rcx], eax
00000001416713eb: mov rdx, qword ptr [rdx]
00000001416713ee: mov r8, qword ptr [rbp + 0xaf0]
00000001416713f5: mov qword ptr [rdx + 0x20], r8
00000001416713f9: movups xmm0, xmmword ptr [rbp + 0xad0]
0000000141671400: movups xmm1, xmmword ptr [rbp + 0xae0]
0000000141671407: movups xmmword ptr [rdx + 0x10], xmm1
000000014167140b: movups xmmword ptr [rdx], xmm0
000000014167140e: cmp byte ptr [rip + 0x1fc397b], 0
0000000141671415: je 0x141671435
0000000141671417: mov qword ptr [rdx + 0x10], 8
000000014167141f: mov dword ptr [rdx + 0x18], 0
0000000141671426: mov word ptr [rdx + 0x1d], 0
000000014167142c: mov dword ptr [rcx], eax
000000014167142e: mov qword ptr [rdx], 0
0000000141671435: mov rcx, qword ptr [rbp + 0x908]
000000014167143c: mov rsi, qword ptr [rcx]
000000014167143f: test rsi, rsi
0000000141671442: je 0x141671538
0000000141671448: mov rax, qword ptr [rcx + 0x10]
000000014167144c: mov ecx, dword ptr [rcx + 0x1c]
000000014167144f: movaps xmm0, xmmword ptr [rbp + 0x6b0]
0000000141671456: movaps xmmword ptr [rbp + 0xb20], xmm0
000000014167145d: movaps xmm0, xmmword ptr [rbp + 0x6a0]
0000000141671464: movaps xmmword ptr [rbp + 0xb10], xmm0
000000014167146b: movaps xmm0, xmmword ptr [rbp + 0x660]
0000000141671472: movaps xmm1, xmmword ptr [rbp + 0x670]
0000000141671479: movaps xmm2, xmmword ptr [rbp + 0x680]
0000000141671480: movaps xmm3, xmmword ptr [rbp + 0x690]
0000000141671487: movaps xmmword ptr [rbp + 0xb00], xmm3
000000014167148e: movaps xmmword ptr [rbp + 0xaf0], xmm2
0000000141671495: movaps xmmword ptr [rbp + 0xae0], xmm1
000000014167149c: movaps xmmword ptr [rbp + 0xad0], xmm0
00000001416714a3: movaps xmm0, xmmword ptr [rbp + 0x910]
00000001416714aa: movaps xmmword ptr [rbp + 0xb30], xmm0
00000001416714b1: mov dword ptr [rax], ecx
00000001416714b3: mov rdx, qword ptr [rsi]
00000001416714b6: test rdx, rdx
00000001416714b9: je 0x1416714ca
00000001416714bb: mov rcx, qword ptr [rsi + 8]
00000001416714bf: mov r8d, 1
00000001416714c5: call 0x140613c20
00000001416714ca: mov rdx, qword ptr [rsi + 0x18]
00000001416714ce: mov rax, rdx
00000001416714d1: neg rax
00000001416714d4: jo 0x1416714e7
00000001416714d6: jae 0x1416714e7
00000001416714d8: mov rcx, qword ptr [rsi + 0x20]
00000001416714dc: mov r8d, 1
00000001416714e2: call 0x140613c20
00000001416714e7: movaps xmm0, xmmword ptr [rbp + 0xb30]
00000001416714ee: movups xmmword ptr [rsi + 0x60], xmm0
00000001416714f2: movaps xmm0, xmmword ptr [rbp + 0xb20]
00000001416714f9: movups xmmword ptr [rsi + 0x50], xmm0
00000001416714fd: movaps xmm0, xmmword ptr [rbp + 0xb10]
0000000141671504: movups xmmword ptr [rsi + 0x40], xmm0
0000000141671508: movaps xmm0, xmmword ptr [rbp + 0xad0]
000000014167150f: movaps xmm1, xmmword ptr [rbp + 0xae0]
0000000141671516: movaps xmm2, xmmword ptr [rbp + 0xaf0]
000000014167151d: movaps xmm3, xmmword ptr [rbp + 0xb00]
0000000141671524: movups xmmword ptr [rsi + 0x30], xmm3
0000000141671528: movups xmmword ptr [rsi + 0x20], xmm2
000000014167152c: movups xmmword ptr [rsi + 0x10], xmm1
0000000141671530: movups xmmword ptr [rsi], xmm0
0000000141671533: jmp 0x1416715bb
0000000141671538: movaps xmm0, xmmword ptr [rbp + 0x6b0]
000000014167153f: movaps xmmword ptr [rbp + 0xb20], xmm0
0000000141671546: movaps xmm0, xmmword ptr [rbp + 0x6a0]
000000014167154d: movaps xmmword ptr [rbp + 0xb10], xmm0
0000000141671554: movaps xmm0, xmmword ptr [rbp + 0x660]
000000014167155b: movaps xmm1, xmmword ptr [rbp + 0x670]
0000000141671562: movaps xmm2, xmmword ptr [rbp + 0x680]
0000000141671569: movaps xmm3, xmmword ptr [rbp + 0x690]
0000000141671570: movaps xmmword ptr [rbp + 0xb00], xmm3
0000000141671577: movaps xmmword ptr [rbp + 0xaf0], xmm2
000000014167157e: movaps xmmword ptr [rbp + 0xae0], xmm1
0000000141671585: movaps xmmword ptr [rbp + 0xad0], xmm0
000000014167158c: movaps xmm0, xmmword ptr [rbp + 0x910]
0000000141671593: movaps xmmword ptr [rbp + 0xb30], xmm0
000000014167159a: mov byte ptr [rbp + 0xe2d], 0
00000001416715a1: lea r8, [rip + 0x16135e8]
00000001416715a8: lea rdx, [rbp + 0xad0]
00000001416715af: mov rcx, qword ptr [rbp + 0xf00]
00000001416715b6: call 0x1412e0810
00000001416715bb: xor eax, eax
00000001416715bd: cmp rax, qword ptr [rbp + 0xd80]
00000001416715c4: jo 0x1416715d9
00000001416715c6: mov byte ptr [rbp + 0xe35], 1
00000001416715cd: lea rcx, [rbp + 0xd80]
00000001416715d4: call 0x140898020
00000001416715d9: mov byte ptr [rbp + 0xe05], 1
00000001416715e0: lea rcx, [rbp + 0xd60]
00000001416715e7: call 0x14087cb00
00000001416715ec: lea rcx, [rbp + 0xa00]
00000001416715f3: call 0x14166e5b0
00000001416715f8: lea rcx, [rbp + 0x320]
00000001416715ff: call 0x140898020
0000000141671604: movaps xmm6, xmmword ptr [rbp + 0xe40]
000000014167160b: add rsp, 0xed8
0000000141671612: pop rbx
0000000141671613: pop rdi
0000000141671614: pop rsi
0000000141671615: pop r12
0000000141671617: pop r13
0000000141671619: pop r14
000000014167161b: pop r15
000000014167161d: pop rbp
000000014167161e: ret 
000000014167161f: mov qword ptr [rsp + 0x20], 1
0000000141671628: mov r8d, 8
000000014167162e: mov r9d, 1
0000000141671634: mov rcx, r14
0000000141671637: mov rdx, r15
000000014167163a: call 0x1428a9a60
000000014167163f: jmp 0x14166e83c
0000000141671644: mov byte ptr [rbp + 0xe2d], 1
000000014167164b: lea rcx, [rip + 0x16134ae]
0000000141671652: lea r8, [rip + 0x16134cf]
0000000141671659: mov edx, 0x28
000000014167165e: call 0x1428d9430
0000000141671663: jmp 0x1416718b9
0000000141671668: mov byte ptr [rbp + 0xe2e], 1
000000014167166f: lea rcx, [rip + 0x161340a]
0000000141671676: call 0x1428d9310
000000014167167b: jmp 0x1416718b9
0000000141671680: lea rcx, [rbp - 0x20]
0000000141671684: call 0x14166e5b0
0000000141671689: lea rax, [rip + 0x1613550]
0000000141671690: mov qword ptr [rbp + 0xad0], rax
0000000141671697: mov qword ptr [rbp + 0xad8], 1
00000001416716a2: mov qword ptr [rbp + 0xae0], 8
00000001416716ad: xorps xmm0, xmm0
00000001416716b0: movups xmmword ptr [rbp + 0xae8], xmm0
00000001416716b7: lea rdx, [rip + 0x1613532]
00000001416716be: lea rcx, [rbp + 0xad0]
00000001416716c5: call 0x1428d9390
00000001416716ca: jmp 0x1416718b9
00000001416716cf: mov qword ptr [rbp + 0xad0], rax
00000001416716d6: lea rax, [rip + 0x16132bb]
00000001416716dd: mov qword ptr [rsp + 0x20], rax
00000001416716e2: lea rcx, [rip + 0x161328f]
00000001416716e9: lea r9, [rip + 0x1612e80]
00000001416716f0: lea r8, [rbp + 0xad0]
00000001416716f7: mov edx, 0x1d
00000001416716fc: call 0x1428d9760
0000000141671701: jmp 0x1416718b9
0000000141671706: mov qword ptr [rbp + 0xad0], rbx
000000014167170d: lea rax, [rip + 0x161324c]
0000000141671714: mov qword ptr [rsp + 0x20], rax
0000000141671719: lea rcx, [rip + 0x1613220]
0000000141671720: lea r9, [rip + 0x1612e69]
0000000141671727: lea r8, [rbp + 0xad0]
000000014167172e: mov edx, 0x20
0000000141671733: call 0x1428d9760
0000000141671738: jmp 0x1416718b9
000000014167173d: mov rax, qword ptr [rbp + 0x3f8]
0000000141671744: mov qword ptr [rbp + 0xad0], rax
000000014167174b: lea rax, [rip + 0x161329e]
0000000141671752: mov qword ptr [rsp + 0x20], rax
0000000141671757: lea rcx, [rip + 0x1612e52]
000000014167175e: lea r9, [rip + 0x1612e0b]
0000000141671765: lea r8, [rbp + 0xad0]
000000014167176c: mov edx, 0x2b
0000000141671771: call 0x1428d9760
0000000141671776: jmp 0x1416718b9
000000014167177b: lea rax, [rip + 0x16132d6]
0000000141671782: mov qword ptr [rbp + 0xad0], rax
0000000141671789: mov qword ptr [rbp + 0xad8], 1
0000000141671794: mov qword ptr [rbp + 0xae0], 8
000000014167179f: xorps xmm0, xmm0
00000001416717a2: movups xmmword ptr [rbp + 0xae8], xmm0
00000001416717a9: mov byte ptr [rbp + 0xe2f], 1
00000001416717b0: lea rdx, [rip + 0x16132b1]
00000001416717b7: lea rcx, [rbp + 0xad0]
00000001416717be: call 0x1428d9390
00000001416717c3: jmp 0x1416718b9
00000001416717c8: lea rcx, [rip + 0x1613251]
00000001416717cf: call 0x1428d9310
00000001416717d4: jmp 0x1416718b9
00000001416717d9: mov byte ptr [rbp + 0xe2f], 1
00000001416717e0: lea rcx, [rip + 0x1613221]
00000001416717e7: call 0x1428d9a20
00000001416717ec: jmp 0x1416718b9
00000001416717f1: mov byte ptr [rbp + 0xe16], 1
00000001416717f8: mov byte ptr [rbp + 0xe15], 1
00000001416717ff: mov byte ptr [rbp + 0xe14], 1
0000000141671806: mov ecx, 8
000000014167180b: mov edx, 0x20
0000000141671810: call 0x1428d9000
0000000141671815: jmp 0x1416718b9
000000014167181a: mov byte ptr [rbp + 0xe34], 1
0000000141671821: mov byte ptr [rbp + 0xe33], 1
0000000141671828: mov byte ptr [rbp + 0xe32], 1
000000014167182f: lea rcx, [rip + 0x1612ffa]
0000000141671836: lea r8, [rip + 0x161301b]
000000014167183d: mov edx, 0x25
0000000141671842: call 0x1428d9330
0000000141671847: jmp 0x1416718b9
0000000141671849: mov byte ptr [rbp + 0xe31], 1
0000000141671850: lea rcx, [rip + 0x1612fd9]
0000000141671857: lea r8, [rip + 0x161328a]
000000014167185e: mov edx, 0x25
0000000141671863: call 0x1428d9330
0000000141671868: jmp 0x1416718b9
000000014167186a: mov byte ptr [rbp + 0xe35], 0
0000000141671871: lea rcx, [rip + 0x1612dad]
0000000141671878: lea r8, [rip + 0x1612dd1]
000000014167187f: mov edx, 0x28
0000000141671884: call 0x1428d9430
0000000141671889: jmp 0x1416718b9
000000014167188b: xor ebx, ebx
000000014167188d: mov r15, rcx
0000000141671890: lea r8, [rip + 0x15c72b1]
0000000141671897: mov rcx, rbx
000000014167189a: mov rdx, r15
000000014167189d: call 0x1428d8fe3
00000001416718a2: jmp 0x1416718b9
00000001416718a4: xor r14d, r14d
00000001416718a7: lea r8, [rip + 0x15c72ca]
00000001416718ae: mov rcx, r14
00000001416718b1: mov rdx, rdi
00000001416718b4: call 0x1428d8fe3
00000001416718b9: ud2 
00000001416718bb: nop dword ptr [rax + rax]
00000001416718c0: mov qword ptr [rsp + 0x10], rdx
00000001416718c5: push rbp
00000001416718c6: push r15
00000001416718c8: push r14
00000001416718ca: push r13
00000001416718cc: push r12
00000001416718ce: push rsi
00000001416718cf: push rdi
00000001416718d0: push rbx
00000001416718d1: sub rsp, 0x38
00000001416718d5: lea rbp, [rdx + 0x80]
00000001416718dc: movaps xmmword ptr [rsp + 0x20], xmm6
00000001416718e1: cmp qword ptr [rbp + 0xe08], 0
00000001416718e9: je 0x141671904
00000001416718eb: mov r8d, 1
00000001416718f1: mov rcx, qword ptr [rbp + 0xdf8]
00000001416718f8: mov rdx, qword ptr [rbp + 0xe08]
00000001416718ff: call 0x140613c20
0000000141671904: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141671909: add rsp, 0x38
000000014167190d: pop rbx
000000014167190e: pop rdi
000000014167190f: pop rsi
0000000141671910: pop r12
0000000141671912: pop r13
0000000141671914: pop r14
0000000141671916: pop r15
0000000141671918: pop rbp
0000000141671919: ret 
000000014167191a: nop word ptr [rax + rax]
0000000141671920: mov qword ptr [rsp + 0x10], rdx
0000000141671925: push rbp
0000000141671926: push r15
0000000141671928: push r14
000000014167192a: push r13
000000014167192c: push r12
000000014167192e: push rsi
000000014167192f: push rdi
0000000141671930: push rbx
0000000141671931: sub rsp, 0x38
0000000141671935: lea rbp, [rdx + 0x80]
000000014167193c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141671941: movzx ebx, byte ptr [rbp + 0xe35]
0000000141671948: lea rcx, [rbp + 0xd60]
000000014167194f: call 0x14087cb00
0000000141671954: mov byte ptr [rbp + 0xe05], bl
000000014167195a: movaps xmm6, xmmword ptr [rsp + 0x20]
000000014167195f: add rsp, 0x38
0000000141671963: pop rbx
0000000141671964: pop rdi
0000000141671965: pop rsi
0000000141671966: pop r12
0000000141671968: pop r13
000000014167196a: pop r14
000000014167196c: pop r15
000000014167196e: pop rbp
000000014167196f: ret 
0000000141671970: mov qword ptr [rsp + 0x10], rdx
0000000141671975: push rbp
0000000141671976: push r15
0000000141671978: push r14
000000014167197a: push r13
000000014167197c: push r12
000000014167197e: push rsi
000000014167197f: push rdi
0000000141671980: push rbx
0000000141671981: sub rsp, 0x38
0000000141671985: lea rbp, [rdx + 0x80]
000000014167198c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141671991: mov rdx, qword ptr [rbp + 0x3f0]
0000000141671998: test rdx, rdx
000000014167199b: je 0x1416719af
000000014167199d: mov rcx, qword ptr [rbp + 0x3f8]
00000001416719a4: mov r8d, 1
00000001416719aa: call 0x140613c20
00000001416719af: mov byte ptr [rbp + 0xe35], 0
00000001416719b6: movaps xmm6, xmmword ptr [rsp + 0x20]
00000001416719bb: add rsp, 0x38
00000001416719bf: pop rbx
00000001416719c0: pop rdi
00000001416719c1: pop rsi
00000001416719c2: pop r12
00000001416719c4: pop r13
00000001416719c6: pop r14
00000001416719c8: pop r15
00000001416719ca: pop rbp
00000001416719cb: ret 
00000001416719cc: nop dword ptr [rax]
00000001416719d0: mov qword ptr [rsp + 0x10], rdx
00000001416719d5: push rbp
00000001416719d6: push r15
00000001416719d8: push r14
00000001416719da: push r13
00000001416719dc: push r12
00000001416719de: push rsi
00000001416719df: push rdi
00000001416719e0: push rbx
00000001416719e1: sub rsp, 0x38
00000001416719e5: lea rbp, [rdx + 0x80]
00000001416719ec: movaps xmmword ptr [rsp + 0x20], xmm6
00000001416719f1: mov byte ptr [rbp + 0xe35], 1
00000001416719f8: lea rcx, [rbp + 0xad0]
00000001416719ff: call 0x140b5a5d0
0000000141671a04: mov byte ptr [rbp + 0xe35], 1
0000000141671a0b: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141671a10: add rsp, 0x38
0000000141671a14: pop rbx
0000000141671a15: pop rdi
0000000141671a16: pop rsi
0000000141671a17: pop r12
0000000141671a19: pop r13
0000000141671a1b: pop r14
0000000141671a1d: pop r15
0000000141671a1f: pop rbp
0000000141671a20: ret 
0000000141671a21: nop word ptr cs:[rax + rax]
0000000141671a30: mov qword ptr [rsp + 0x10], rdx
0000000141671a35: push rbp
0000000141671a36: push r15
0000000141671a38: push r14
0000000141671a3a: push r13
0000000141671a3c: push r12
0000000141671a3e: push rsi
0000000141671a3f: push rdi
0000000141671a40: push rbx
0000000141671a41: sub rsp, 0x38
0000000141671a45: lea rbp, [rdx + 0x80]
0000000141671a4c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141671a51: mov rdx, qword ptr [rbp + 0xad0]
0000000141671a58: test rdx, rdx
0000000141671a5b: je 0x141671a6f
0000000141671a5d: mov rcx, qword ptr [rbp + 0xad8]
0000000141671a64: mov r8d, 1
0000000141671a6a: call 0x140613c20
0000000141671a6f: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141671a74: add rsp, 0x38
0000000141671a78: pop rbx
0000000141671a79: pop rdi
0000000141671a7a: pop rsi
0000000141671a7b: pop r12
0000000141671a7d: pop r13
0000000141671a7f: pop r14
0000000141671a81: pop r15
0000000141671a83: pop rbp
0000000141671a84: ret 
0000000141671a85: nop word ptr cs:[rax + rax]
0000000141671a90: mov qword ptr [rsp + 0x10], rdx
0000000141671a95: push rbp
0000000141671a96: push r15
0000000141671a98: push r14
0000000141671a9a: push r13
0000000141671a9c: push r12
0000000141671a9e: push rsi
0000000141671a9f: push rdi
0000000141671aa0: push rbx
0000000141671aa1: sub rsp, 0x38
0000000141671aa5: lea rbp, [rdx + 0x80]
0000000141671aac: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141671ab1: lea rcx, [rbp + 0x930]
0000000141671ab8: call 0x14088fa00
0000000141671abd: mov byte ptr [rbp + 0xe17], 0
0000000141671ac4: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141671ac9: add rsp, 0x38
0000000141671acd: pop rbx
0000000141671ace: pop rdi
0000000141671acf: pop rsi
0000000141671ad0: pop r12
0000000141671ad2: pop r13
0000000141671ad4: pop r14
0000000141671ad6: pop r15
0000000141671ad8: pop rbp
0000000141671ad9: ret 
0000000141671ada: nop word ptr [rax + rax]
0000000141671ae0: mov qword ptr [rsp + 0x10], rdx
0000000141671ae5: push rbp
0000000141671ae6: push r15
0000000141671ae8: push r14
0000000141671aea: push r13
0000000141671aec: push r12
0000000141671aee: push rsi
0000000141671aef: push rdi
0000000141671af0: push rbx
0000000141671af1: sub rsp, 0x38
0000000141671af5: lea rbp, [rdx + 0x80]
0000000141671afc: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141671b01: cmp byte ptr [rbp + 0xe17], 0
0000000141671b08: je 0x141671b33
0000000141671b0a: lea rcx, [rbp + 0x930]
0000000141671b11: call 0x14088fa00
0000000141671b16: mov byte ptr [rbp + 0xe35], 0
0000000141671b1d: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141671b22: add rsp, 0x38
0000000141671b26: pop rbx
0000000141671b27: pop rdi
0000000141671b28: pop rsi
0000000141671b29: pop r12
0000000141671b2b: pop r13
0000000141671b2d: pop r14
0000000141671b2f: pop r15
0000000141671b31: pop rbp
0000000141671b32: ret 
0000000141671b33: mov byte ptr [rbp + 0xe35], 0
0000000141671b3a: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141671b3f: add rsp, 0x38
0000000141671b43: pop rbx
0000000141671b44: pop rdi
0000000141671b45: pop rsi
0000000141671b46: pop r12
0000000141671b48: pop r13
0000000141671b4a: pop r14
0000000141671b4c: pop r15
0000000141671b4e: pop rbp
0000000141671b4f: ret 
0000000141671b50: mov qword ptr [rsp + 0x10], rdx
0000000141671b55: push rbp
0000000141671b56: push r15
0000000141671b58: push r14
0000000141671b5a: push r13
0000000141671b5c: push r12
0000000141671b5e: push rsi
0000000141671b5f: push rdi
0000000141671b60: push rbx
0000000141671b61: sub rsp, 0x38
0000000141671b65: lea rbp, [rdx + 0x80]
0000000141671b6c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141671b71: mov rax, qword ptr [rbp + 0xe20]
0000000141671b78: lea rax, [rax*2]
0000000141671b80: test rax, rax
0000000141671b83: je 0x141671b9e
0000000141671b85: mov r8d, 1
0000000141671b8b: mov rcx, qword ptr [rbp + 0xe18]
0000000141671b92: mov rdx, qword ptr [rbp + 0xe20]
0000000141671b99: call 0x140613c20
0000000141671b9e: lea rcx, [rbp + 0x660]
0000000141671ba5: call 0x14014b4d0
0000000141671baa: mov byte ptr [rbp + 0xe35], 0
0000000141671bb1: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141671bb6: add rsp, 0x38
0000000141671bba: pop rbx
0000000141671bbb: pop rdi
0000000141671bbc: pop rsi
0000000141671bbd: pop r12
0000000141671bbf: pop r13
0000000141671bc1: pop r14
0000000141671bc3: pop r15
0000000141671bc5: pop rbp
0000000141671bc6: ret 
0000000141671bc7: nop word ptr [rax + rax]
0000000141671bd0: mov qword ptr [rsp + 0x10], rdx
0000000141671bd5: push rbp
0000000141671bd6: push r15
0000000141671bd8: push r14
0000000141671bda: push r13
0000000141671bdc: push r12
0000000141671bde: push rsi
0000000141671bdf: push rdi
0000000141671be0: push rbx
0000000141671be1: sub rsp, 0x38
0000000141671be5: lea rbp, [rdx + 0x80]
0000000141671bec: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141671bf1: xor eax, eax
0000000141671bf3: cmp rax, qword ptr [rbp + 0xd80]
0000000141671bfa: jo 0x141671c08
0000000141671bfc: lea rcx, [rbp + 0xd80]
0000000141671c03: call 0x140898020
0000000141671c08: mov byte ptr [rbp + 0xe35], 1
0000000141671c0f: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141671c14: add rsp, 0x38
0000000141671c18: pop rbx
0000000141671c19: pop rdi
0000000141671c1a: pop rsi
0000000141671c1b: pop r12
0000000141671c1d: pop r13
0000000141671c1f: pop r14
0000000141671c21: pop r15
0000000141671c23: pop rbp
0000000141671c24: ret 
0000000141671c25: nop word ptr cs:[rax + rax]
0000000141671c30: mov qword ptr [rsp + 0x10], rdx
0000000141671c35: push rbp
0000000141671c36: push r15
0000000141671c38: push r14
0000000141671c3a: push r13
0000000141671c3c: push r12
0000000141671c3e: push rsi
0000000141671c3f: push rdi
0000000141671c40: push rbx
0000000141671c41: sub rsp, 0x38
0000000141671c45: lea rbp, [rdx + 0x80]
0000000141671c4c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141671c51: mov rdx, qword ptr [rbp + 0x660]
0000000141671c58: test rdx, rdx
0000000141671c5b: je 0x141671c6f
0000000141671c5d: mov rcx, qword ptr [rbp + 0x668]
0000000141671c64: mov r8d, 1
0000000141671c6a: call 0x140613c20
0000000141671c6f: mov byte ptr [rbp + 0xe2c], 1
0000000141671c76: mov byte ptr [rbp + 0xe2b], 1
0000000141671c7d: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141671c82: add rsp, 0x38
0000000141671c86: pop rbx
0000000141671c87: pop rdi
0000000141671c88: pop rsi
0000000141671c89: pop r12
0000000141671c8b: pop r13
0000000141671c8d: pop r14
0000000141671c8f: pop r15
0000000141671c91: pop rbp
0000000141671c92: ret 
0000000141671c93: nop word ptr cs:[rax + rax]
0000000141671ca0: mov qword ptr [rsp + 0x10], rdx
0000000141671ca5: push rbp
0000000141671ca6: push r15
0000000141671ca8: push r14
0000000141671caa: push r13
0000000141671cac: push r12
0000000141671cae: push rsi
0000000141671caf: push rdi
0000000141671cb0: push rbx
0000000141671cb1: sub rsp, 0x38
0000000141671cb5: lea rbp, [rdx + 0x80]
0000000141671cbc: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141671cc1: mov rcx, qword ptr [rbp + 0xe20]
0000000141671cc8: mov rax, qword ptr [rbp + 0xdf8]
0000000141671ccf: mov rdx, qword ptr [rax + 8]
0000000141671cd3: test rdx, rdx
0000000141671cd6: je 0x141671ce8
0000000141671cd8: mov rax, qword ptr [rbp + 0xdf8]
0000000141671cdf: mov r8, qword ptr [rax + 0x10]
0000000141671ce3: call 0x140613c20
0000000141671ce8: mov edx, 0x18
0000000141671ced: mov r8d, 8
0000000141671cf3: mov rcx, qword ptr [rbp + 0xde0]
0000000141671cfa: call 0x140613c20
0000000141671cff: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141671d04: add rsp, 0x38
0000000141671d08: pop rbx
0000000141671d09: pop rdi
0000000141671d0a: pop rsi
0000000141671d0b: pop r12
0000000141671d0d: pop r13
0000000141671d0f: pop r14
0000000141671d11: pop r15
0000000141671d13: pop rbp
0000000141671d14: ret 
0000000141671d15: nop word ptr cs:[rax + rax]
0000000141671d20: mov qword ptr [rsp + 0x10], rdx
0000000141671d25: push rbp
0000000141671d26: push r15
0000000141671d28: push r14
0000000141671d2a: push r13
0000000141671d2c: push r12
0000000141671d2e: push rsi
0000000141671d2f: push rdi
0000000141671d30: push rbx
0000000141671d31: sub rsp, 0x38
0000000141671d35: lea rbp, [rdx + 0x80]
0000000141671d3c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141671d41: mov rdx, qword ptr [rbp + 0x660]
0000000141671d48: test rdx, rdx
0000000141671d4b: je 0x141671d5f
0000000141671d4d: mov rcx, qword ptr [rbp + 0x668]
0000000141671d54: mov r8d, 1
0000000141671d5a: call 0x140613c20
0000000141671d5f: mov byte ptr [rbp + 0xe2c], 1
0000000141671d66: mov byte ptr [rbp + 0xe2b], 1
0000000141671d6d: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141671d72: add rsp, 0x38
0000000141671d76: pop rbx
0000000141671d77: pop rdi
0000000141671d78: pop rsi
0000000141671d79: pop r12
0000000141671d7b: pop r13
0000000141671d7d: pop r14
0000000141671d7f: pop r15
0000000141671d81: pop rbp
0000000141671d82: ret 
0000000141671d83: nop word ptr cs:[rax + rax]
0000000141671d90: mov qword ptr [rsp + 0x10], rdx
0000000141671d95: push rbp
0000000141671d96: push r15
0000000141671d98: push r14
0000000141671d9a: push r13
0000000141671d9c: push r12
0000000141671d9e: push rsi
0000000141671d9f: push rdi
0000000141671da0: push rbx
0000000141671da1: sub rsp, 0x38
0000000141671da5: lea rbp, [rdx + 0x80]
0000000141671dac: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141671db1: mov rdx, qword ptr [rbp + 0x3f0]
0000000141671db8: test rdx, rdx
0000000141671dbb: je 0x141671dcf
0000000141671dbd: mov rcx, qword ptr [rbp + 0x3f8]
0000000141671dc4: mov r8d, 1
0000000141671dca: call 0x140613c20
0000000141671dcf: mov byte ptr [rbp + 0xe34], 1
0000000141671dd6: mov byte ptr [rbp + 0xe33], 0
0000000141671ddd: mov byte ptr [rbp + 0xe32], 1
0000000141671de4: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141671de9: add rsp, 0x38
0000000141671ded: pop rbx
0000000141671dee: pop rdi
0000000141671def: pop rsi
0000000141671df0: pop r12
0000000141671df2: pop r13
0000000141671df4: pop r14
0000000141671df6: pop r15
0000000141671df8: pop rbp
0000000141671df9: ret 
0000000141671dfa: nop word ptr [rax + rax]
0000000141671e00: mov qword ptr [rsp + 0x10], rdx
0000000141671e05: push rbp
0000000141671e06: push r15
0000000141671e08: push r14
0000000141671e0a: push r13
0000000141671e0c: push r12
0000000141671e0e: push rsi
0000000141671e0f: push rdi
0000000141671e10: push rbx
0000000141671e11: sub rsp, 0x38
0000000141671e15: lea rbp, [rdx + 0x80]
0000000141671e1c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141671e21: mov rdx, qword ptr [rbp + 0xad0]
0000000141671e28: test rdx, rdx
0000000141671e2b: je 0x141671e3f
0000000141671e2d: mov rcx, qword ptr [rbp + 0xad8]
0000000141671e34: mov r8d, 1
0000000141671e3a: call 0x140613c20
0000000141671e3f: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141671e44: add rsp, 0x38
0000000141671e48: pop rbx
0000000141671e49: pop rdi
0000000141671e4a: pop rsi
0000000141671e4b: pop r12
0000000141671e4d: pop r13
0000000141671e4f: pop r14
0000000141671e51: pop r15
0000000141671e53: pop rbp
0000000141671e54: ret 
0000000141671e55: nop word ptr cs:[rax + rax]
0000000141671e60: mov qword ptr [rsp + 0x10], rdx
0000000141671e65: push rbp
0000000141671e66: push r15
0000000141671e68: push r14
0000000141671e6a: push r13
0000000141671e6c: push r12
0000000141671e6e: push rsi
0000000141671e6f: push rdi
0000000141671e70: push rbx
0000000141671e71: sub rsp, 0x38
0000000141671e75: lea rbp, [rdx + 0x80]
0000000141671e7c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141671e81: mov byte ptr [rbp + 0xe34], 1
0000000141671e88: mov byte ptr [rbp + 0xe33], 0
0000000141671e8f: mov byte ptr [rbp + 0xe32], 1
0000000141671e96: lea rcx, [rbp + 0x9e0]
0000000141671e9d: call 0x140d4cc30
0000000141671ea2: mov byte ptr [rbp + 0xe34], 1
0000000141671ea9: mov byte ptr [rbp + 0xe33], 0
0000000141671eb0: mov byte ptr [rbp + 0xe32], 1
0000000141671eb7: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141671ebc: add rsp, 0x38
0000000141671ec0: pop rbx
0000000141671ec1: pop rdi
0000000141671ec2: pop rsi
0000000141671ec3: pop r12
0000000141671ec5: pop r13
0000000141671ec7: pop r14
0000000141671ec9: pop r15
0000000141671ecb: pop rbp
0000000141671ecc: ret 
0000000141671ecd: nop dword ptr [rax]
0000000141671ed0: mov qword ptr [rsp + 0x10], rdx
0000000141671ed5: push rbp
0000000141671ed6: push r15
0000000141671ed8: push r14
0000000141671eda: push r13
0000000141671edc: push r12
0000000141671ede: push rsi
0000000141671edf: push rdi
0000000141671ee0: push rbx
0000000141671ee1: sub rsp, 0x38
0000000141671ee5: lea rbp, [rdx + 0x80]
0000000141671eec: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141671ef1: cmp qword ptr [rbp + 0xe18], 0
0000000141671ef9: je 0x141671f14
0000000141671efb: mov r8d, 1
0000000141671f01: mov rcx, qword ptr [rbp + 0xe08]
0000000141671f08: mov rdx, qword ptr [rbp + 0xe18]
0000000141671f0f: call 0x140613c20
0000000141671f14: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141671f19: add rsp, 0x38
0000000141671f1d: pop rbx
0000000141671f1e: pop rdi
0000000141671f1f: pop rsi
0000000141671f20: pop r12
0000000141671f22: pop r13
0000000141671f24: pop r14
0000000141671f26: pop r15
0000000141671f28: pop rbp
0000000141671f29: ret 
0000000141671f2a: nop word ptr [rax + rax]
0000000141671f30: mov qword ptr [rsp + 0x10], rdx
0000000141671f35: push rbp
0000000141671f36: push r15
0000000141671f38: push r14
0000000141671f3a: push r13
0000000141671f3c: push r12
0000000141671f3e: push rsi
0000000141671f3f: push rdi
0000000141671f40: push rbx
0000000141671f41: sub rsp, 0x38
0000000141671f45: lea rbp, [rdx + 0x80]
0000000141671f4c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141671f51: mov rcx, qword ptr [rbp + 0xe20]
0000000141671f58: mov rax, qword ptr [rbp + 0xde8]
0000000141671f5f: mov rdx, qword ptr [rax + 8]
0000000141671f63: test rdx, rdx
0000000141671f66: je 0x141671f78
0000000141671f68: mov rax, qword ptr [rbp + 0xde8]
0000000141671f6f: mov r8, qword ptr [rax + 0x10]
0000000141671f73: call 0x140613c20
0000000141671f78: mov edx, 0x18
0000000141671f7d: mov r8d, 8
0000000141671f83: mov rcx, qword ptr [rbp + 0xde0]
0000000141671f8a: call 0x140613c20
0000000141671f8f: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141671f94: add rsp, 0x38
0000000141671f98: pop rbx
0000000141671f99: pop rdi
0000000141671f9a: pop rsi
0000000141671f9b: pop r12
0000000141671f9d: pop r13
0000000141671f9f: pop r14
0000000141671fa1: pop r15
0000000141671fa3: pop rbp
0000000141671fa4: ret 
0000000141671fa5: nop word ptr cs:[rax + rax]
0000000141671fb0: mov qword ptr [rsp + 0x10], rdx
0000000141671fb5: push rbp
0000000141671fb6: push r15
0000000141671fb8: push r14
0000000141671fba: push r13
0000000141671fbc: push r12
0000000141671fbe: push rsi
0000000141671fbf: push rdi
0000000141671fc0: push rbx
0000000141671fc1: sub rsp, 0x38
0000000141671fc5: lea rbp, [rdx + 0x80]
0000000141671fcc: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141671fd1: mov rdx, qword ptr [rbp + 0xad0]
0000000141671fd8: test rdx, rdx
0000000141671fdb: je 0x141671fef
0000000141671fdd: mov rcx, qword ptr [rbp + 0xad8]
0000000141671fe4: mov r8d, 1
0000000141671fea: call 0x140613c20
0000000141671fef: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141671ff4: add rsp, 0x38
0000000141671ff8: pop rbx
0000000141671ff9: pop rdi
0000000141671ffa: pop rsi
0000000141671ffb: pop r12
0000000141671ffd: pop r13
0000000141671fff: pop r14
0000000141672001: pop r15
0000000141672003: pop rbp
0000000141672004: ret 
0000000141672005: nop word ptr cs:[rax + rax]
0000000141672010: mov qword ptr [rsp + 0x10], rdx
0000000141672015: push rbp
0000000141672016: push r15
0000000141672018: push r14
000000014167201a: push r13
000000014167201c: push r12
000000014167201e: push rsi
000000014167201f: push rdi
0000000141672020: push rbx
0000000141672021: sub rsp, 0x38
0000000141672025: lea rbp, [rdx + 0x80]
000000014167202c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672031: lea rcx, [rbp + 0x3f0]
0000000141672038: call 0x14014b4d0
000000014167203d: mov byte ptr [rbp + 0xe34], 0
0000000141672044: mov byte ptr [rbp + 0xe33], 0
000000014167204b: movzx eax, byte ptr [rbp + 0xdf8]
0000000141672052: mov byte ptr [rbp + 0xe32], al
0000000141672058: movaps xmm6, xmmword ptr [rsp + 0x20]
000000014167205d: add rsp, 0x38
0000000141672061: pop rbx
0000000141672062: pop rdi
0000000141672063: pop rsi
0000000141672064: pop r12
0000000141672066: pop r13
0000000141672068: pop r14
000000014167206a: pop r15
000000014167206c: pop rbp
000000014167206d: ret 
000000014167206e: nop 
0000000141672070: mov qword ptr [rsp + 0x10], rdx
0000000141672075: push rbp
0000000141672076: push r15
0000000141672078: push r14
000000014167207a: push r13
000000014167207c: push r12
000000014167207e: push rsi
000000014167207f: push rdi
0000000141672080: push rbx
0000000141672081: sub rsp, 0x38
0000000141672085: lea rbp, [rdx + 0x80]
000000014167208c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672091: cmp byte ptr [rbp + 0xe13], 0
0000000141672098: je 0x1416720ce
000000014167209a: mov rdx, qword ptr [rbp + 0xdc0]
00000001416720a1: test rdx, rdx
00000001416720a4: je 0x1416720b8
00000001416720a6: mov rcx, qword ptr [rbp + 0xdc8]
00000001416720ad: mov r8d, 1
00000001416720b3: call 0x140613c20
00000001416720b8: movaps xmm6, xmmword ptr [rsp + 0x20]
00000001416720bd: add rsp, 0x38
00000001416720c1: pop rbx
00000001416720c2: pop rdi
00000001416720c3: pop rsi
00000001416720c4: pop r12
00000001416720c6: pop r13
00000001416720c8: pop r14
00000001416720ca: pop r15
00000001416720cc: pop rbp
00000001416720cd: ret 
00000001416720ce: movaps xmm6, xmmword ptr [rsp + 0x20]
00000001416720d3: add rsp, 0x38
00000001416720d7: pop rbx
00000001416720d8: pop rdi
00000001416720d9: pop rsi
00000001416720da: pop r12
00000001416720dc: pop r13
00000001416720de: pop r14
00000001416720e0: pop r15
00000001416720e2: pop rbp
00000001416720e3: ret 
00000001416720e4: nop word ptr cs:[rax + rax]
00000001416720f0: mov qword ptr [rsp + 0x10], rdx
00000001416720f5: push rbp
00000001416720f6: push r15
00000001416720f8: push r14
00000001416720fa: push r13
00000001416720fc: push r12
00000001416720fe: push rsi
00000001416720ff: push rdi
0000000141672100: push rbx
0000000141672101: sub rsp, 0x38
0000000141672105: lea rbp, [rdx + 0x80]
000000014167210c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672111: mov rax, qword ptr [rbp + 0xe20]
0000000141672118: mov qword ptr [rbp + 0x668], rax
000000014167211f: lea rcx, [rbp + 0xad0]
0000000141672126: call 0x14166e790
000000014167212b: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672130: add rsp, 0x38
0000000141672134: pop rbx
0000000141672135: pop rdi
0000000141672136: pop rsi
0000000141672137: pop r12
0000000141672139: pop r13
000000014167213b: pop r14
000000014167213d: pop r15
000000014167213f: pop rbp
0000000141672140: ret 
0000000141672141: nop word ptr cs:[rax + rax]
0000000141672150: mov qword ptr [rsp + 0x10], rdx
0000000141672155: push rbp
0000000141672156: push r15
0000000141672158: push r14
000000014167215a: push r13
000000014167215c: push r12
000000014167215e: push rsi
000000014167215f: push rdi
0000000141672160: push rbx
0000000141672161: sub rsp, 0x38
0000000141672165: lea rbp, [rdx + 0x80]
000000014167216c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672171: mov rax, qword ptr [rbp + 0xe20]
0000000141672178: mov qword ptr [rbp + 0x668], rax
000000014167217f: lea rcx, [rbp + 0xad0]
0000000141672186: call 0x14166e790
000000014167218b: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672190: add rsp, 0x38
0000000141672194: pop rbx
0000000141672195: pop rdi
0000000141672196: pop rsi
0000000141672197: pop r12
0000000141672199: pop r13
000000014167219b: pop r14
000000014167219d: pop r15
000000014167219f: pop rbp
00000001416721a0: ret 
00000001416721a1: nop word ptr cs:[rax + rax]
00000001416721b0: mov qword ptr [rsp + 0x10], rdx
00000001416721b5: push rbp
00000001416721b6: push r15
00000001416721b8: push r14
00000001416721ba: push r13
00000001416721bc: push r12
00000001416721be: push rsi
00000001416721bf: push rdi
00000001416721c0: push rbx
00000001416721c1: sub rsp, 0x38
00000001416721c5: lea rbp, [rdx + 0x80]
00000001416721cc: movaps xmmword ptr [rsp + 0x20], xmm6
00000001416721d1: movzx ebx, byte ptr [rbp + 0xe32]
00000001416721d8: movzx esi, byte ptr [rbp + 0xe33]
00000001416721df: movzx edi, byte ptr [rbp + 0xe34]
00000001416721e6: test sil, 1
00000001416721ea: je 0x141672229
00000001416721ec: lea rcx, [rbp + 0xda0]
00000001416721f3: call 0x140876a30
00000001416721f8: and dil, 1
00000001416721fc: mov byte ptr [rbp + 0xe16], dil
0000000141672203: mov byte ptr [rbp + 0xe15], sil
000000014167220a: and bl, 1
000000014167220d: mov byte ptr [rbp + 0xe14], bl
0000000141672213: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672218: add rsp, 0x38
000000014167221c: pop rbx
000000014167221d: pop rdi
000000014167221e: pop rsi
000000014167221f: pop r12
0000000141672221: pop r13
0000000141672223: pop r14
0000000141672225: pop r15
0000000141672227: pop rbp
0000000141672228: ret 
0000000141672229: and dil, 1
000000014167222d: mov byte ptr [rbp + 0xe16], dil
0000000141672234: mov byte ptr [rbp + 0xe15], sil
000000014167223b: and bl, 1
000000014167223e: mov byte ptr [rbp + 0xe14], bl
0000000141672244: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672249: add rsp, 0x38
000000014167224d: pop rbx
000000014167224e: pop rdi
000000014167224f: pop rsi
0000000141672250: pop r12
0000000141672252: pop r13
0000000141672254: pop r14
0000000141672256: pop r15
0000000141672258: pop rbp
0000000141672259: ret 
000000014167225a: nop word ptr [rax + rax]
0000000141672260: mov qword ptr [rsp + 0x10], rdx
0000000141672265: push rbp
0000000141672266: push r15
0000000141672268: push r14
000000014167226a: push r13
000000014167226c: push r12
000000014167226e: push rsi
000000014167226f: push rdi
0000000141672270: push rbx
0000000141672271: sub rsp, 0x38
0000000141672275: lea rbp, [rdx + 0x80]
000000014167227c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672281: movzx ebx, byte ptr [rbp + 0xe14]
0000000141672288: movzx esi, byte ptr [rbp + 0xe16]
000000014167228f: test byte ptr [rbp + 0xe15], 1
0000000141672296: je 0x1416722ce
0000000141672298: lea rcx, [rbp + 0x950]
000000014167229f: call 0x14146ca50
00000001416722a4: and sil, 1
00000001416722a8: mov byte ptr [rbp + 0xe2c], sil
00000001416722af: and bl, 1
00000001416722b2: mov byte ptr [rbp + 0xe2b], bl
00000001416722b8: movaps xmm6, xmmword ptr [rsp + 0x20]
00000001416722bd: add rsp, 0x38
00000001416722c1: pop rbx
00000001416722c2: pop rdi
00000001416722c3: pop rsi
00000001416722c4: pop r12
00000001416722c6: pop r13
00000001416722c8: pop r14
00000001416722ca: pop r15
00000001416722cc: pop rbp
00000001416722cd: ret 
00000001416722ce: and sil, 1
00000001416722d2: mov byte ptr [rbp + 0xe2c], sil
00000001416722d9: and bl, 1
00000001416722dc: mov byte ptr [rbp + 0xe2b], bl
00000001416722e2: movaps xmm6, xmmword ptr [rsp + 0x20]
00000001416722e7: add rsp, 0x38
00000001416722eb: pop rbx
00000001416722ec: pop rdi
00000001416722ed: pop rsi
00000001416722ee: pop r12
00000001416722f0: pop r13
00000001416722f2: pop r14
00000001416722f4: pop r15
00000001416722f6: pop rbp
00000001416722f7: ret 
00000001416722f8: nop dword ptr [rax + rax]
0000000141672300: mov qword ptr [rsp + 0x10], rdx
0000000141672305: push rbp
0000000141672306: push r15
0000000141672308: push r14
000000014167230a: push r13
000000014167230c: push r12
000000014167230e: push rsi
000000014167230f: push rdi
0000000141672310: push rbx
0000000141672311: sub rsp, 0x38
0000000141672315: lea rbp, [rdx + 0x80]
000000014167231c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672321: movzx eax, byte ptr [rbp + 0xe2b]
0000000141672328: mov byte ptr [rbp + 0xe20], al
000000014167232e: mov rax, qword ptr [rbp + 0xe18]
0000000141672335: neg rax
0000000141672338: jno 0x141672350
000000014167233a: movaps xmm6, xmmword ptr [rsp + 0x20]
000000014167233f: add rsp, 0x38
0000000141672343: pop rbx
0000000141672344: pop rdi
0000000141672345: pop rsi
0000000141672346: pop r12
0000000141672348: pop r13
000000014167234a: pop r14
000000014167234c: pop r15
000000014167234e: pop rbp
000000014167234f: ret 
0000000141672350: test byte ptr [rbp + 0xe2c], 1
0000000141672357: je 0x141672392
0000000141672359: cmp qword ptr [rbp + 0xe18], 0
0000000141672361: je 0x14167237c
0000000141672363: mov r8d, 1
0000000141672369: mov rcx, qword ptr [rbp + 0xe08]
0000000141672370: mov rdx, qword ptr [rbp + 0xe18]
0000000141672377: call 0x140613c20
000000014167237c: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672381: add rsp, 0x38
0000000141672385: pop rbx
0000000141672386: pop rdi
0000000141672387: pop rsi
0000000141672388: pop r12
000000014167238a: pop r13
000000014167238c: pop r14
000000014167238e: pop r15
0000000141672390: pop rbp
0000000141672391: ret 
0000000141672392: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672397: add rsp, 0x38
000000014167239b: pop rbx
000000014167239c: pop rdi
000000014167239d: pop rsi
000000014167239e: pop r12
00000001416723a0: pop r13
00000001416723a2: pop r14
00000001416723a4: pop r15
00000001416723a6: pop rbp
00000001416723a7: ret 
00000001416723a8: nop dword ptr [rax + rax]
00000001416723b0: mov qword ptr [rsp + 0x10], rdx
00000001416723b5: push rbp
00000001416723b6: push r15
00000001416723b8: push r14
00000001416723ba: push r13
00000001416723bc: push r12
00000001416723be: push rsi
00000001416723bf: push rdi
00000001416723c0: push rbx
00000001416723c1: sub rsp, 0x38
00000001416723c5: lea rbp, [rdx + 0x80]
00000001416723cc: movaps xmmword ptr [rsp + 0x20], xmm6
00000001416723d1: xor eax, eax
00000001416723d3: cmp rax, qword ptr [rbp + 0xc0]
00000001416723da: jo 0x141672407
00000001416723dc: test byte ptr [rbp + 0xe20], 1
00000001416723e3: je 0x14167241d
00000001416723e5: lea rcx, [rbp + 0xc0]
00000001416723ec: call 0x14014b4d0
00000001416723f1: movaps xmm6, xmmword ptr [rsp + 0x20]
00000001416723f6: add rsp, 0x38
00000001416723fa: pop rbx
00000001416723fb: pop rdi
00000001416723fc: pop rsi
00000001416723fd: pop r12
00000001416723ff: pop r13
0000000141672401: pop r14
0000000141672403: pop r15
0000000141672405: pop rbp
0000000141672406: ret 
0000000141672407: movaps xmm6, xmmword ptr [rsp + 0x20]
000000014167240c: add rsp, 0x38
0000000141672410: pop rbx
0000000141672411: pop rdi
0000000141672412: pop rsi
0000000141672413: pop r12
0000000141672415: pop r13
0000000141672417: pop r14
0000000141672419: pop r15
000000014167241b: pop rbp
000000014167241c: ret 
000000014167241d: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672422: add rsp, 0x38
0000000141672426: pop rbx
0000000141672427: pop rdi
0000000141672428: pop rsi
0000000141672429: pop r12
000000014167242b: pop r13
000000014167242d: pop r14
000000014167242f: pop r15
0000000141672431: pop rbp
0000000141672432: ret 
0000000141672433: nop word ptr cs:[rax + rax]
0000000141672440: mov qword ptr [rsp + 0x10], rdx
0000000141672445: push rbp
0000000141672446: push r15
0000000141672448: push r14
000000014167244a: push r13
000000014167244c: push r12
000000014167244e: push rsi
000000014167244f: push rdi
0000000141672450: push rbx
0000000141672451: sub rsp, 0x38
0000000141672455: lea rbp, [rdx + 0x80]
000000014167245c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672461: movzx ebx, byte ptr [rbp + 0xe36]
0000000141672468: mov rdx, qword ptr [rbp + 0xdf0]
000000014167246f: lea rax, [rdx*2]
0000000141672477: test rax, rax
000000014167247a: je 0x141672497
000000014167247c: test byte ptr [rbp + 0xe37], 1
0000000141672483: je 0x141672497
0000000141672485: mov r8d, 1
000000014167248b: mov rcx, qword ptr [rbp + 0xde0]
0000000141672492: call 0x140613c20
0000000141672497: test bl, 1
000000014167249a: je 0x1416724be
000000014167249c: lea rcx, [rbp + 0x950]
00000001416724a3: call 0x14014b4d0
00000001416724a8: movaps xmm6, xmmword ptr [rsp + 0x20]
00000001416724ad: add rsp, 0x38
00000001416724b1: pop rbx
00000001416724b2: pop rdi
00000001416724b3: pop rsi
00000001416724b4: pop r12
00000001416724b6: pop r13
00000001416724b8: pop r14
00000001416724ba: pop r15
00000001416724bc: pop rbp
00000001416724bd: ret 
00000001416724be: movaps xmm6, xmmword ptr [rsp + 0x20]
00000001416724c3: add rsp, 0x38
00000001416724c7: pop rbx
00000001416724c8: pop rdi
00000001416724c9: pop rsi
00000001416724ca: pop r12
00000001416724cc: pop r13
00000001416724ce: pop r14
00000001416724d0: pop r15
00000001416724d2: pop rbp
00000001416724d3: ret 
00000001416724d4: nop word ptr cs:[rax + rax]
00000001416724e0: mov qword ptr [rsp + 0x10], rdx
00000001416724e5: push rbp
00000001416724e6: push r15
00000001416724e8: push r14
00000001416724ea: push r13
00000001416724ec: push r12
00000001416724ee: push rsi
00000001416724ef: push rdi
00000001416724f0: push rbx
00000001416724f1: sub rsp, 0x38
00000001416724f5: lea rbp, [rdx + 0x80]
00000001416724fc: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672501: mov rdx, qword ptr [rbp + 0xad0]
0000000141672508: test rdx, rdx
000000014167250b: je 0x14167251f
000000014167250d: mov rcx, qword ptr [rbp + 0xad8]
0000000141672514: mov r8d, 1
000000014167251a: call 0x140613c20
000000014167251f: mov rax, qword ptr [rbp + 0xe20]
0000000141672526: mov qword ptr [rbp + 0xdf0], rax
000000014167252d: mov byte ptr [rbp + 0xe37], 1
0000000141672534: mov byte ptr [rbp + 0xe36], 1
000000014167253b: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672540: add rsp, 0x38
0000000141672544: pop rbx
0000000141672545: pop rdi
0000000141672546: pop rsi
0000000141672547: pop r12
0000000141672549: pop r13
000000014167254b: pop r14
000000014167254d: pop r15
000000014167254f: pop rbp
0000000141672550: ret 
0000000141672551: nop word ptr cs:[rax + rax]
0000000141672560: mov qword ptr [rsp + 0x10], rdx
0000000141672565: push rbp
0000000141672566: push r15
0000000141672568: push r14
000000014167256a: push r13
000000014167256c: push r12
000000014167256e: push rsi
000000014167256f: push rdi
0000000141672570: push rbx
0000000141672571: sub rsp, 0x38
0000000141672575: lea rbp, [rdx + 0x80]
000000014167257c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672581: mov rsi, qword ptr [rbp + 0xe20]
0000000141672588: mov qword ptr [rbp + 0xdf0], rsi
000000014167258f: mov byte ptr [rbp + 0xe37], 1
0000000141672596: mov byte ptr [rbp + 0xe36], 1
000000014167259d: lea rcx, [rbp + 0xad0]
00000001416725a4: call 0x1405e7a00
00000001416725a9: mov qword ptr [rbp + 0xdf0], rsi
00000001416725b0: mov byte ptr [rbp + 0xe37], 1
00000001416725b7: mov byte ptr [rbp + 0xe36], 1
00000001416725be: movaps xmm6, xmmword ptr [rsp + 0x20]
00000001416725c3: add rsp, 0x38
00000001416725c7: pop rbx
00000001416725c8: pop rdi
00000001416725c9: pop rsi
00000001416725ca: pop r12
00000001416725cc: pop r13
00000001416725ce: pop r14
00000001416725d0: pop r15
00000001416725d2: pop rbp
00000001416725d3: ret 
00000001416725d4: nop word ptr cs:[rax + rax]
00000001416725e0: mov qword ptr [rsp + 0x10], rdx
00000001416725e5: push rbp
00000001416725e6: push r15
00000001416725e8: push r14
00000001416725ea: push r13
00000001416725ec: push r12
00000001416725ee: push rsi
00000001416725ef: push rdi
00000001416725f0: push rbx
00000001416725f1: sub rsp, 0x38
00000001416725f5: lea rbp, [rdx + 0x80]
00000001416725fc: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672601: mov rax, qword ptr [rbp + 0xae0]
0000000141672608: lock dec qword ptr [rax]
000000014167260c: jne 0x141672621
000000014167260e: mov byte ptr [rbp + 0xe2f], 1
0000000141672615: lea rcx, [rbp + 0xae0]
000000014167261c: call 0x140d53bc0
0000000141672621: mov byte ptr [rbp + 0xe2f], 1
0000000141672628: movaps xmm6, xmmword ptr [rsp + 0x20]
000000014167262d: add rsp, 0x38
0000000141672631: pop rbx
0000000141672632: pop rdi
0000000141672633: pop rsi
0000000141672634: pop r12
0000000141672636: pop r13
0000000141672638: pop r14
000000014167263a: pop r15
000000014167263c: pop rbp
000000014167263d: ret 
000000014167263e: nop 
0000000141672640: mov qword ptr [rsp + 0x10], rdx
0000000141672645: push rbp
0000000141672646: push r15
0000000141672648: push r14
000000014167264a: push r13
000000014167264c: push r12
000000014167264e: push rsi
000000014167264f: push rdi
0000000141672650: push rbx
0000000141672651: sub rsp, 0x38
0000000141672655: lea rbp, [rdx + 0x80]
000000014167265c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672661: mov rdx, qword ptr [rbp + 0xda0]
0000000141672668: test rdx, rdx
000000014167266b: je 0x14167267f
000000014167266d: mov rcx, qword ptr [rbp + 0xda8]
0000000141672674: mov r8d, 1
000000014167267a: call 0x140613c20
000000014167267f: movabs rax, 0x8000000000000000
0000000141672689: mov qword ptr [rbp + 0xdf0], rax
0000000141672690: mov byte ptr [rbp + 0xe37], 1
0000000141672697: mov byte ptr [rbp + 0xe36], 1
000000014167269e: movaps xmm6, xmmword ptr [rsp + 0x20]
00000001416726a3: add rsp, 0x38
00000001416726a7: pop rbx
00000001416726a8: pop rdi
00000001416726a9: pop rsi
00000001416726aa: pop r12
00000001416726ac: pop r13
00000001416726ae: pop r14
00000001416726b0: pop r15
00000001416726b2: pop rbp
00000001416726b3: ret 
00000001416726b4: nop word ptr cs:[rax + rax]
00000001416726c0: mov qword ptr [rsp + 0x10], rdx
00000001416726c5: push rbp
00000001416726c6: push r15
00000001416726c8: push r14
00000001416726ca: push r13
00000001416726cc: push r12
00000001416726ce: push rsi
00000001416726cf: push rdi
00000001416726d0: push rbx
00000001416726d1: sub rsp, 0x38
00000001416726d5: lea rbp, [rdx + 0x80]
00000001416726dc: movaps xmmword ptr [rsp + 0x20], xmm6
00000001416726e1: mov rax, qword ptr [rbp + 0x3f0]
00000001416726e8: lock dec qword ptr [rax]
00000001416726ec: jne 0x1416726fa
00000001416726ee: lea rcx, [rbp + 0x3f0]
00000001416726f5: call 0x140d53bc0
00000001416726fa: movaps xmm6, xmmword ptr [rsp + 0x20]
00000001416726ff: add rsp, 0x38
0000000141672703: pop rbx
0000000141672704: pop rdi
0000000141672705: pop rsi
0000000141672706: pop r12
0000000141672708: pop r13
000000014167270a: pop r14
000000014167270c: pop r15
000000014167270e: pop rbp
000000014167270f: ret 
0000000141672710: mov qword ptr [rsp + 0x10], rdx
0000000141672715: push rbp
0000000141672716: push r15
0000000141672718: push r14
000000014167271a: push r13
000000014167271c: push r12
000000014167271e: push rsi
000000014167271f: push rdi
0000000141672720: push rbx
0000000141672721: sub rsp, 0x38
0000000141672725: lea rbp, [rdx + 0x80]
000000014167272c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672731: cmp qword ptr [rbp + 0xe18], 0
0000000141672739: je 0x141672754
000000014167273b: mov r8d, 1
0000000141672741: mov rcx, qword ptr [rbp + 0xe08]
0000000141672748: mov rdx, qword ptr [rbp + 0xe18]
000000014167274f: call 0x140613c20
0000000141672754: mov byte ptr [rbp + 0xe2e], 0
000000014167275b: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672760: add rsp, 0x38
0000000141672764: pop rbx
0000000141672765: pop rdi
0000000141672766: pop rsi
0000000141672767: pop r12
0000000141672769: pop r13
000000014167276b: pop r14
000000014167276d: pop r15
000000014167276f: pop rbp
0000000141672770: ret 
0000000141672771: nop word ptr cs:[rax + rax]
0000000141672780: mov qword ptr [rsp + 0x10], rdx
0000000141672785: push rbp
0000000141672786: push r15
0000000141672788: push r14
000000014167278a: push r13
000000014167278c: push r12
000000014167278e: push rsi
000000014167278f: push rdi
0000000141672790: push rbx
0000000141672791: sub rsp, 0x38
0000000141672795: lea rbp, [rdx + 0x80]
000000014167279c: movaps xmmword ptr [rsp + 0x20], xmm6
00000001416727a1: mov rcx, qword ptr [rbp + 0xe18]
00000001416727a8: mov rax, qword ptr [rbp + 0xe08]
00000001416727af: mov rdx, qword ptr [rax + 8]
00000001416727b3: test rdx, rdx
00000001416727b6: je 0x1416727c8
00000001416727b8: mov rax, qword ptr [rbp + 0xe08]
00000001416727bf: mov r8, qword ptr [rax + 0x10]
00000001416727c3: call 0x140613c20
00000001416727c8: mov edx, 0x18
00000001416727cd: mov r8d, 8
00000001416727d3: mov rcx, qword ptr [rbp + 0xdf8]
00000001416727da: call 0x140613c20
00000001416727df: movaps xmm6, xmmword ptr [rsp + 0x20]
00000001416727e4: add rsp, 0x38
00000001416727e8: pop rbx
00000001416727e9: pop rdi
00000001416727ea: pop rsi
00000001416727eb: pop r12
00000001416727ed: pop r13
00000001416727ef: pop r14
00000001416727f1: pop r15
00000001416727f3: pop rbp
00000001416727f4: ret 
00000001416727f5: nop word ptr cs:[rax + rax]
0000000141672800: mov qword ptr [rsp + 0x10], rdx
0000000141672805: push rbp
0000000141672806: push r15
0000000141672808: push r14
000000014167280a: push r13
000000014167280c: push r12
000000014167280e: push rsi
000000014167280f: push rdi
0000000141672810: push rbx
0000000141672811: sub rsp, 0x38
0000000141672815: lea rbp, [rdx + 0x80]
000000014167281c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672821: cmp byte ptr [rbp + 0xe2e], 0
0000000141672828: je 0x14167285e
000000014167282a: mov rdx, qword ptr [rbp + 0xdc0]
0000000141672831: test rdx, rdx
0000000141672834: je 0x141672848
0000000141672836: mov rcx, qword ptr [rbp + 0xdc8]
000000014167283d: mov r8d, 1
0000000141672843: call 0x140613c20
0000000141672848: movaps xmm6, xmmword ptr [rsp + 0x20]
000000014167284d: add rsp, 0x38
0000000141672851: pop rbx
0000000141672852: pop rdi
0000000141672853: pop rsi
0000000141672854: pop r12
0000000141672856: pop r13
0000000141672858: pop r14
000000014167285a: pop r15
000000014167285c: pop rbp
000000014167285d: ret 
000000014167285e: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672863: add rsp, 0x38
0000000141672867: pop rbx
0000000141672868: pop rdi
0000000141672869: pop rsi
000000014167286a: pop r12
000000014167286c: pop r13
000000014167286e: pop r14
0000000141672870: pop r15
0000000141672872: pop rbp
0000000141672873: ret 
0000000141672874: nop word ptr cs:[rax + rax]
0000000141672880: mov qword ptr [rsp + 0x10], rdx
0000000141672885: push rbp
0000000141672886: push r15
0000000141672888: push r14
000000014167288a: push r13
000000014167288c: push r12
000000014167288e: push rsi
000000014167288f: push rdi
0000000141672890: push rbx
0000000141672891: sub rsp, 0x38
0000000141672895: lea rbp, [rdx + 0x80]
000000014167289c: movaps xmmword ptr [rsp + 0x20], xmm6
00000001416728a1: cmp qword ptr [rbp + 0xc0], 0
00000001416728a9: jne 0x1416728cd
00000001416728ab: lea rcx, [rbp + 0xc0]
00000001416728b2: call 0x1406180e0
00000001416728b7: movaps xmm6, xmmword ptr [rsp + 0x20]
00000001416728bc: add rsp, 0x38
00000001416728c0: pop rbx
00000001416728c1: pop rdi
00000001416728c2: pop rsi
00000001416728c3: pop r12
00000001416728c5: pop r13
00000001416728c7: pop r14
00000001416728c9: pop r15
00000001416728cb: pop rbp
00000001416728cc: ret 
00000001416728cd: movaps xmm6, xmmword ptr [rsp + 0x20]
00000001416728d2: add rsp, 0x38
00000001416728d6: pop rbx
00000001416728d7: pop rdi
00000001416728d8: pop rsi
00000001416728d9: pop r12
00000001416728db: pop r13
00000001416728dd: pop r14
00000001416728df: pop r15
00000001416728e1: pop rbp
00000001416728e2: ret 
00000001416728e3: nop word ptr cs:[rax + rax]
00000001416728f0: mov qword ptr [rsp + 0x10], rdx
00000001416728f5: push rbp
00000001416728f6: push r15
00000001416728f8: push r14
00000001416728fa: push r13
00000001416728fc: push r12
00000001416728fe: push rsi
00000001416728ff: push rdi
0000000141672900: push rbx
0000000141672901: sub rsp, 0x38
0000000141672905: lea rbp, [rdx + 0x80]
000000014167290c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672911: mov rcx, qword ptr [rbp + 0xe18]
0000000141672918: mov rax, qword ptr [rbp + 0xe08]
000000014167291f: mov rdx, qword ptr [rax + 8]
0000000141672923: test rdx, rdx
0000000141672926: je 0x141672938
0000000141672928: mov rax, qword ptr [rbp + 0xe08]
000000014167292f: mov r8, qword ptr [rax + 0x10]
0000000141672933: call 0x140613c20
0000000141672938: mov edx, 0x18
000000014167293d: mov r8d, 8
0000000141672943: mov rcx, qword ptr [rbp + 0xdf8]
000000014167294a: call 0x140613c20
000000014167294f: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672954: add rsp, 0x38
0000000141672958: pop rbx
0000000141672959: pop rdi
000000014167295a: pop rsi
000000014167295b: pop r12
000000014167295d: pop r13
000000014167295f: pop r14
0000000141672961: pop r15
0000000141672963: pop rbp
0000000141672964: ret 
0000000141672965: nop word ptr cs:[rax + rax]
0000000141672970: mov qword ptr [rsp + 0x10], rdx
0000000141672975: push rbp
0000000141672976: push r15
0000000141672978: push r14
000000014167297a: push r13
000000014167297c: push r12
000000014167297e: push rsi
000000014167297f: push rdi
0000000141672980: push rbx
0000000141672981: sub rsp, 0x38
0000000141672985: lea rbp, [rdx + 0x80]
000000014167298c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672991: cmp byte ptr [rbp + 0xe2f], 0
0000000141672998: je 0x1416729db
000000014167299a: lea rcx, [rbp + 0xd48]
00000001416729a1: call 0x1405d8fc0
00000001416729a6: movabs rax, 0x8000000000000000
00000001416729b0: mov qword ptr [rbp + 0xdf0], rax
00000001416729b7: mov byte ptr [rbp + 0xe37], 1
00000001416729be: mov byte ptr [rbp + 0xe36], 1
00000001416729c5: movaps xmm6, xmmword ptr [rsp + 0x20]
00000001416729ca: add rsp, 0x38
00000001416729ce: pop rbx
00000001416729cf: pop rdi
00000001416729d0: pop rsi
00000001416729d1: pop r12
00000001416729d3: pop r13
00000001416729d5: pop r14
00000001416729d7: pop r15
00000001416729d9: pop rbp
00000001416729da: ret 
00000001416729db: movabs rax, 0x8000000000000000
00000001416729e5: mov qword ptr [rbp + 0xdf0], rax
00000001416729ec: mov byte ptr [rbp + 0xe37], 1
00000001416729f3: mov byte ptr [rbp + 0xe36], 1
00000001416729fa: movaps xmm6, xmmword ptr [rsp + 0x20]
00000001416729ff: add rsp, 0x38
0000000141672a03: pop rbx
0000000141672a04: pop rdi
0000000141672a05: pop rsi
0000000141672a06: pop r12
0000000141672a08: pop r13
0000000141672a0a: pop r14
0000000141672a0c: pop r15
0000000141672a0e: pop rbp
0000000141672a0f: ret 
0000000141672a10: mov qword ptr [rsp + 0x10], rdx
0000000141672a15: push rbp
0000000141672a16: push r15
0000000141672a18: push r14
0000000141672a1a: push r13
0000000141672a1c: push r12
0000000141672a1e: push rsi
0000000141672a1f: push rdi
0000000141672a20: push rbx
0000000141672a21: sub rsp, 0x38
0000000141672a25: lea rbp, [rdx + 0x80]
0000000141672a2c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672a31: mov rcx, qword ptr [rbp + 0xe18]
0000000141672a38: mov rax, qword ptr [rbp + 0xe08]
0000000141672a3f: mov rdx, qword ptr [rax + 8]
0000000141672a43: test rdx, rdx
0000000141672a46: je 0x141672a58
0000000141672a48: mov rax, qword ptr [rbp + 0xe08]
0000000141672a4f: mov r8, qword ptr [rax + 0x10]
0000000141672a53: call 0x140613c20
0000000141672a58: mov edx, 0x18
0000000141672a5d: mov r8d, 8
0000000141672a63: mov rcx, qword ptr [rbp + 0xdf8]
0000000141672a6a: call 0x140613c20
0000000141672a6f: mov byte ptr [rbp + 0xe31], 1
0000000141672a76: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672a7b: add rsp, 0x38
0000000141672a7f: pop rbx
0000000141672a80: pop rdi
0000000141672a81: pop rsi
0000000141672a82: pop r12
0000000141672a84: pop r13
0000000141672a86: pop r14
0000000141672a88: pop r15
0000000141672a8a: pop rbp
0000000141672a8b: ret 
0000000141672a8c: nop dword ptr [rax]
0000000141672a90: mov qword ptr [rsp + 0x10], rdx
0000000141672a95: push rbp
0000000141672a96: push r15
0000000141672a98: push r14
0000000141672a9a: push r13
0000000141672a9c: push r12
0000000141672a9e: push rsi
0000000141672a9f: push rdi
0000000141672aa0: push rbx
0000000141672aa1: sub rsp, 0x38
0000000141672aa5: lea rbp, [rdx + 0x80]
0000000141672aac: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672ab1: cmp byte ptr [rbp + 0xe2d], 0
0000000141672ab8: je 0x141672adc
0000000141672aba: lea rcx, [rbp + 0x660]
0000000141672ac1: call 0x14014b4d0
0000000141672ac6: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672acb: add rsp, 0x38
0000000141672acf: pop rbx
0000000141672ad0: pop rdi
0000000141672ad1: pop rsi
0000000141672ad2: pop r12
0000000141672ad4: pop r13
0000000141672ad6: pop r14
0000000141672ad8: pop r15
0000000141672ada: pop rbp
0000000141672adb: ret 
0000000141672adc: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672ae1: add rsp, 0x38
0000000141672ae5: pop rbx
0000000141672ae6: pop rdi
0000000141672ae7: pop rsi
0000000141672ae8: pop r12
0000000141672aea: pop r13
0000000141672aec: pop r14
0000000141672aee: pop r15
0000000141672af0: pop rbp
0000000141672af1: ret 
0000000141672af2: nop word ptr cs:[rax + rax]
0000000141672b00: mov qword ptr [rsp + 0x10], rdx
0000000141672b05: push rbp
0000000141672b06: push r15
0000000141672b08: push r14
0000000141672b0a: push r13
0000000141672b0c: push r12
0000000141672b0e: push rsi
0000000141672b0f: push rdi
0000000141672b10: push rbx
0000000141672b11: sub rsp, 0x38
0000000141672b15: lea rbp, [rdx + 0x80]
0000000141672b1c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672b21: movzx ebx, byte ptr [rbp + 0xe31]
0000000141672b28: test bl, 1
0000000141672b2b: je 0x141672b82
0000000141672b2d: mov rdx, qword ptr [rbp + 0x9c0]
0000000141672b34: test rdx, rdx
0000000141672b37: je 0x141672b4b
0000000141672b39: mov rcx, qword ptr [rbp + 0x9c8]
0000000141672b40: mov r8d, 1
0000000141672b46: call 0x140613c20
0000000141672b4b: mov rax, qword ptr [rbp + 0xe20]
0000000141672b52: mov qword ptr [rbp + 0xdf0], rax
0000000141672b59: movzx eax, byte ptr [rbp + 0xe06]
0000000141672b60: mov byte ptr [rbp + 0xe37], al
0000000141672b66: mov byte ptr [rbp + 0xe36], bl
0000000141672b6c: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672b71: add rsp, 0x38
0000000141672b75: pop rbx
0000000141672b76: pop rdi
0000000141672b77: pop rsi
0000000141672b78: pop r12
0000000141672b7a: pop r13
0000000141672b7c: pop r14
0000000141672b7e: pop r15
0000000141672b80: pop rbp
0000000141672b81: ret 
0000000141672b82: mov rax, qword ptr [rbp + 0xe20]
0000000141672b89: mov qword ptr [rbp + 0xdf0], rax
0000000141672b90: movzx eax, byte ptr [rbp + 0xe06]
0000000141672b97: mov byte ptr [rbp + 0xe37], al
0000000141672b9d: mov byte ptr [rbp + 0xe36], bl
0000000141672ba3: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672ba8: add rsp, 0x38
0000000141672bac: pop rbx
0000000141672bad: pop rdi
0000000141672bae: pop rsi
0000000141672baf: pop r12
0000000141672bb1: pop r13
0000000141672bb3: pop r14
0000000141672bb5: pop r15
0000000141672bb7: pop rbp
0000000141672bb8: ret 
0000000141672bb9: nop dword ptr [rax]
0000000141672bc0: mov qword ptr [rsp + 0x10], rdx
0000000141672bc5: push rbp
0000000141672bc6: push r15
0000000141672bc8: push r14
0000000141672bca: push r13
0000000141672bcc: push r12
0000000141672bce: push rsi
0000000141672bcf: push rdi
0000000141672bd0: push rbx
0000000141672bd1: sub rsp, 0x38
0000000141672bd5: lea rbp, [rdx + 0x80]
0000000141672bdc: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672be1: cmp byte ptr [rbp + 0xe05], 0
0000000141672be8: je 0x141672c0c
0000000141672bea: lea rcx, [rbp + 0xa00]
0000000141672bf1: call 0x14166e5b0
0000000141672bf6: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672bfb: add rsp, 0x38
0000000141672bff: pop rbx
0000000141672c00: pop rdi
0000000141672c01: pop rsi
0000000141672c02: pop r12
0000000141672c04: pop r13
0000000141672c06: pop r14
0000000141672c08: pop r15
0000000141672c0a: pop rbp
0000000141672c0b: ret 
0000000141672c0c: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672c11: add rsp, 0x38
0000000141672c15: pop rbx
0000000141672c16: pop rdi
0000000141672c17: pop rsi
0000000141672c18: pop r12
0000000141672c1a: pop r13
0000000141672c1c: pop r14
0000000141672c1e: pop r15
0000000141672c20: pop rbp
0000000141672c21: ret 
0000000141672c22: nop word ptr cs:[rax + rax]
0000000141672c30: mov qword ptr [rsp + 0x10], rdx
0000000141672c35: push rbp
0000000141672c36: push r15
0000000141672c38: push r14
0000000141672c3a: push r13
0000000141672c3c: push r12
0000000141672c3e: push rsi
0000000141672c3f: push rdi
0000000141672c40: push rbx
0000000141672c41: sub rsp, 0x38
0000000141672c45: lea rbp, [rdx + 0x80]
0000000141672c4c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672c51: lea rcx, [rbp + 0x320]
0000000141672c58: call 0x140898020
0000000141672c5d: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672c62: add rsp, 0x38
0000000141672c66: pop rbx
0000000141672c67: pop rdi
0000000141672c68: pop rsi
0000000141672c69: pop r12
0000000141672c6b: pop r13
0000000141672c6d: pop r14
0000000141672c6f: pop r15
0000000141672c71: pop rbp
0000000141672c72: ret 
0000000141672c73: nop word ptr cs:[rax + rax]
0000000141672c80: mov qword ptr [rsp + 0x10], rdx
0000000141672c85: push rbp
0000000141672c86: push r15
0000000141672c88: push r14
0000000141672c8a: push r13
0000000141672c8c: push r12
0000000141672c8e: push rsi
0000000141672c8f: push rdi
0000000141672c90: push rbx
0000000141672c91: sub rsp, 0x38
0000000141672c95: lea rbp, [rdx + 0x80]
0000000141672c9c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672ca1: lea rcx, [rbp + 0xad0]
0000000141672ca8: call 0x14014b4d0
0000000141672cad: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672cb2: add rsp, 0x38
0000000141672cb6: pop rbx
0000000141672cb7: pop rdi
0000000141672cb8: pop rsi
0000000141672cb9: pop r12
0000000141672cbb: pop r13
0000000141672cbd: pop r14
0000000141672cbf: pop r15
0000000141672cc1: pop rbp
0000000141672cc2: ret 
0000000141672cc3: nop word ptr cs:[rax + rax]
0000000141672cd0: mov qword ptr [rsp + 0x10], rdx
0000000141672cd5: push rbp
0000000141672cd6: push r15
0000000141672cd8: push r14
0000000141672cda: push r13
0000000141672cdc: push r12
0000000141672cde: push rsi
0000000141672cdf: push rdi
0000000141672ce0: push rbx
0000000141672ce1: sub rsp, 0x38
0000000141672ce5: lea rbp, [rdx + 0x80]
0000000141672cec: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672cf1: mov rax, qword ptr [rbp + 0xe20]
0000000141672cf8: mov qword ptr [rbp + 0xa00], rax
0000000141672cff: lea rdx, [rbp + 0xad0]
0000000141672d06: mov r8d, 0xc0
0000000141672d0c: lea rcx, [rbp + 0xa08]
0000000141672d13: call 0x142923c00
0000000141672d18: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672d1d: add rsp, 0x38
0000000141672d21: pop rbx
0000000141672d22: pop rdi
0000000141672d23: pop rsi
0000000141672d24: pop r12
0000000141672d26: pop r13
0000000141672d28: pop r14
0000000141672d2a: pop r15
0000000141672d2c: pop rbp
0000000141672d2d: ret 
0000000141672d2e: nop 
0000000141672d30: mov qword ptr [rsp + 0x10], rdx
0000000141672d35: push rbp
0000000141672d36: push r15
0000000141672d38: push r14
0000000141672d3a: push r13
0000000141672d3c: push r12
0000000141672d3e: push rsi
0000000141672d3f: push rdi
0000000141672d40: push rbx
0000000141672d41: sub rsp, 0x38
0000000141672d45: lea rbp, [rdx + 0x80]
0000000141672d4c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672d51: lea rcx, [rbp + 0x660]
0000000141672d58: call 0x140878540
0000000141672d5d: mov byte ptr [rbp + 0xe34], 1
0000000141672d64: mov byte ptr [rbp + 0xe33], 1
0000000141672d6b: mov byte ptr [rbp + 0xe32], 1
0000000141672d72: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672d77: add rsp, 0x38
0000000141672d7b: pop rbx
0000000141672d7c: pop rdi
0000000141672d7d: pop rsi
0000000141672d7e: pop r12
0000000141672d80: pop r13
0000000141672d82: pop r14
0000000141672d84: pop r15
0000000141672d86: pop rbp
0000000141672d87: ret 
0000000141672d88: nop dword ptr [rax + rax]
0000000141672d90: mov qword ptr [rsp + 0x10], rdx
0000000141672d95: push rbp
0000000141672d96: push r15
0000000141672d98: push r14
0000000141672d9a: push r13
0000000141672d9c: push r12
0000000141672d9e: push rsi
0000000141672d9f: push rdi
0000000141672da0: push rbx
0000000141672da1: sub rsp, 0x38
0000000141672da5: lea rbp, [rdx + 0x80]
0000000141672dac: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672db1: mov rax, qword ptr [rbp + 0xb10]
0000000141672db8: mov rcx, qword ptr [rbp + 0xe20]
0000000141672dbf: mov qword ptr [rcx + 0x40], rax
0000000141672dc3: movaps xmm0, xmmword ptr [rbp + 0xad0]
0000000141672dca: movaps xmm1, xmmword ptr [rbp + 0xae0]
0000000141672dd1: movaps xmm2, xmmword ptr [rbp + 0xaf0]
0000000141672dd8: movaps xmm3, xmmword ptr [rbp + 0xb00]
0000000141672ddf: movups xmmword ptr [rcx + 0x30], xmm3
0000000141672de3: movups xmmword ptr [rcx + 0x20], xmm2
0000000141672de7: movups xmmword ptr [rcx + 0x10], xmm1
0000000141672deb: movups xmmword ptr [rcx], xmm0
0000000141672dee: mov byte ptr [rcx + 0x48], 0
0000000141672df2: movzx eax, byte ptr [rbp + 0xe07]
0000000141672df9: mov byte ptr [rcx + 0x49], al
0000000141672dfc: movzx eax, byte ptr [rbp + 0xde8]
0000000141672e03: mov byte ptr [rcx + 0x4a], al
0000000141672e06: movzx eax, byte ptr [rbp + 0xdf8]
0000000141672e0d: mov byte ptr [rcx + 0x4b], al
0000000141672e10: mov qword ptr [rcx + 0x50], 0
0000000141672e18: mov byte ptr [rbp + 0xe34], 1
0000000141672e1f: mov byte ptr [rbp + 0xe33], 0
0000000141672e26: mov byte ptr [rbp + 0xe32], 1
0000000141672e2d: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672e32: add rsp, 0x38
0000000141672e36: pop rbx
0000000141672e37: pop rdi
0000000141672e38: pop rsi
0000000141672e39: pop r12
0000000141672e3b: pop r13
0000000141672e3d: pop r14
0000000141672e3f: pop r15
0000000141672e41: pop rbp
0000000141672e42: ret 
0000000141672e43: nop word ptr cs:[rax + rax]
0000000141672e50: mov qword ptr [rsp + 0x10], rdx
0000000141672e55: push rbp
0000000141672e56: push r15
0000000141672e58: push r14
0000000141672e5a: push r13
0000000141672e5c: push r12
0000000141672e5e: push rsi
0000000141672e5f: push rdi
0000000141672e60: push rbx
0000000141672e61: sub rsp, 0x38
0000000141672e65: lea rbp, [rdx + 0x80]
0000000141672e6c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672e71: mov rsi, qword ptr [rbp + 0xe20]
0000000141672e78: mov qword ptr [rbp + 0xdf0], rsi
0000000141672e7f: mov byte ptr [rbp + 0xe37], 1
0000000141672e86: mov byte ptr [rbp + 0xe36], 1
0000000141672e8d: lea rcx, [rbp + 0xad0]
0000000141672e94: call 0x140d4cc30
0000000141672e99: mov qword ptr [rbp + 0xdf0], rsi
0000000141672ea0: mov byte ptr [rbp + 0xe37], 1
0000000141672ea7: mov byte ptr [rbp + 0xe36], 1
0000000141672eae: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672eb3: add rsp, 0x38
0000000141672eb7: pop rbx
0000000141672eb8: pop rdi
0000000141672eb9: pop rsi
0000000141672eba: pop r12
0000000141672ebc: pop r13
0000000141672ebe: pop r14
0000000141672ec0: pop r15
0000000141672ec2: pop rbp
0000000141672ec3: ret 
0000000141672ec4: nop word ptr cs:[rax + rax]
0000000141672ed0: mov qword ptr [rsp + 0x10], rdx
0000000141672ed5: push rbp
0000000141672ed6: push r15
0000000141672ed8: push r14
0000000141672eda: push r13
0000000141672edc: push r12
0000000141672ede: push rsi
0000000141672edf: push rdi
0000000141672ee0: push rbx
0000000141672ee1: sub rsp, 0x38
0000000141672ee5: lea rbp, [rdx + 0x80]
0000000141672eec: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672ef1: movabs rsi, 0x8000000000000000
0000000141672efb: mov qword ptr [rbp + 0xdf0], rsi
0000000141672f02: mov byte ptr [rbp + 0xe37], 1
0000000141672f09: mov byte ptr [rbp + 0xe36], 1
0000000141672f10: lea rcx, [rbp + 0xad0]
0000000141672f17: call 0x140d4cc30
0000000141672f1c: mov qword ptr [rbp + 0xdf0], rsi
0000000141672f23: mov byte ptr [rbp + 0xe37], 1
0000000141672f2a: mov byte ptr [rbp + 0xe36], 1
0000000141672f31: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672f36: add rsp, 0x38
0000000141672f3a: pop rbx
0000000141672f3b: pop rdi
0000000141672f3c: pop rsi
0000000141672f3d: pop r12
0000000141672f3f: pop r13
0000000141672f41: pop r14
0000000141672f43: pop r15
0000000141672f45: pop rbp
0000000141672f46: ret 
0000000141672f47: nop word ptr [rax + rax]
0000000141672f50: mov qword ptr [rsp + 0x10], rdx
0000000141672f55: push rbp
0000000141672f56: push r15
0000000141672f58: push r14
0000000141672f5a: push r13
0000000141672f5c: push r12
0000000141672f5e: push rsi
0000000141672f5f: push rdi
0000000141672f60: push rbx
0000000141672f61: sub rsp, 0x38
0000000141672f65: lea rbp, [rdx + 0x80]
0000000141672f6c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672f71: lea rcx, [rbp + 0xad0]
0000000141672f78: call 0x1405e69f0
0000000141672f7d: mov byte ptr [rbp + 0xe2f], 1
0000000141672f84: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672f89: add rsp, 0x38
0000000141672f8d: pop rbx
0000000141672f8e: pop rdi
0000000141672f8f: pop rsi
0000000141672f90: pop r12
0000000141672f92: pop r13
0000000141672f94: pop r14
0000000141672f96: pop r15
0000000141672f98: pop rbp
0000000141672f99: ret 
0000000141672f9a: nop word ptr [rax + rax]
0000000141672fa0: mov qword ptr [rsp + 0x10], rdx
0000000141672fa5: push rbp
0000000141672fa6: push r15
0000000141672fa8: push r14
0000000141672faa: push r13
0000000141672fac: push r12
0000000141672fae: push rsi
0000000141672faf: push rdi
0000000141672fb0: push rbx
0000000141672fb1: sub rsp, 0x38
0000000141672fb5: lea rbp, [rdx + 0x80]
0000000141672fbc: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141672fc1: lea rcx, [rbp + 0xad0]
0000000141672fc8: call 0x1405dd0b0
0000000141672fcd: mov byte ptr [rbp + 0xe2f], 0
0000000141672fd4: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141672fd9: add rsp, 0x38
0000000141672fdd: pop rbx
0000000141672fde: pop rdi
0000000141672fdf: pop rsi
0000000141672fe0: pop r12
0000000141672fe2: pop r13
0000000141672fe4: pop r14
0000000141672fe6: pop r15
0000000141672fe8: pop rbp
0000000141672fe9: ret 
0000000141672fea: nop word ptr [rax + rax]
0000000141672ff0: mov qword ptr [rsp + 0x10], rdx
0000000141672ff5: push rbp
0000000141672ff6: push r15
0000000141672ff8: push r14
0000000141672ffa: push r13
0000000141672ffc: push r12
0000000141672ffe: push rsi
0000000141672fff: push rdi
0000000141673000: push rbx
0000000141673001: sub rsp, 0x38
0000000141673005: lea rbp, [rdx + 0x80]
000000014167300c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141673011: lea rcx, [rbp + 0x660]
0000000141673018: call 0x140878540
000000014167301d: mov byte ptr [rbp + 0xe31], 1
0000000141673024: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141673029: add rsp, 0x38
000000014167302d: pop rbx
000000014167302e: pop rdi
000000014167302f: pop rsi
0000000141673030: pop r12
0000000141673032: pop r13
0000000141673034: pop r14
0000000141673036: pop r15
0000000141673038: pop rbp
0000000141673039: ret 
000000014167303a: nop word ptr [rax + rax]
0000000141673040: mov qword ptr [rsp + 0x10], rdx
0000000141673045: push rbp
0000000141673046: push r15
0000000141673048: push r14
000000014167304a: push r13
000000014167304c: push r12
000000014167304e: push rsi
000000014167304f: push rdi
0000000141673050: push rbx
0000000141673051: sub rsp, 0x38
0000000141673055: lea rbp, [rdx + 0x80]
000000014167305c: movaps xmmword ptr [rsp + 0x20], xmm6
0000000141673061: movabs rax, 0x8000000000000000
000000014167306b: mov qword ptr [rbp + 0xa00], rax
0000000141673072: mov qword ptr [rbp + 0xa08], 1
000000014167307d: movaps xmm0, xmmword ptr [rbp + 0x3f0]
0000000141673084: movaps xmm1, xmmword ptr [rbp + 0x400]
000000014167308b: lea rax, [rbp + 0xa08]
0000000141673092: movups xmmword ptr [rax + 0x18], xmm1
0000000141673096: movups xmmword ptr [rax + 8], xmm0
000000014167309a: movaps xmm0, xmmword ptr [rbp + 0xb20]
00000001416730a1: movups xmmword ptr [rax + 0x78], xmm0
00000001416730a5: movaps xmm0, xmmword ptr [rbp + 0xb10]
00000001416730ac: movups xmmword ptr [rax + 0x68], xmm0
00000001416730b0: movaps xmm0, xmmword ptr [rbp + 0xad0]
00000001416730b7: movaps xmm1, xmmword ptr [rbp + 0xae0]
00000001416730be: movaps xmm2, xmmword ptr [rbp + 0xaf0]
00000001416730c5: movaps xmm3, xmmword ptr [rbp + 0xb00]
00000001416730cc: movups xmmword ptr [rax + 0x58], xmm3
00000001416730d0: movups xmmword ptr [rax + 0x48], xmm2
00000001416730d4: movups xmmword ptr [rax + 0x38], xmm1
00000001416730d8: movups xmmword ptr [rax + 0x28], xmm0
00000001416730dc: movaps xmm0, xmmword ptr [rbp + 0xc0]
00000001416730e3: movaps xmm1, xmmword ptr [rbp + 0xd0]
00000001416730ea: movups xmmword ptr [rax + 0x98], xmm1
00000001416730f1: movups xmmword ptr [rax + 0x88], xmm0
00000001416730f8: movaps xmm0, xmmword ptr [rbp + 0xdc0]
00000001416730ff: movups xmmword ptr [rax + 0xa8], xmm0
0000000141673106: movzx eax, byte ptr [rbp + 0xe18]
000000014167310d: mov byte ptr [rbp + 0xac0], al
0000000141673113: mov byte ptr [rbp + 0xac1], 1
000000014167311a: mov byte ptr [rbp + 0xe31], 0
0000000141673121: movaps xmm6, xmmword ptr [rsp + 0x20]
0000000141673126: add rsp, 0x38
000000014167312a: pop rbx
000000014167312b: pop rdi
000000014167312c: pop rsi
000000014167312d: pop r12
000000014167312f: pop r13
0000000141673131: pop r14
0000000141673133: pop r15
0000000141673135: pop rbp
0000000141673136: ret 
