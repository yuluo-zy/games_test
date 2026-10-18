14161ad70: push rbp
14161ad71: push r15
14161ad73: push r14
14161ad75: push r12
14161ad77: push rsi
14161ad78: push rdi
14161ad79: push rbx
14161ad7a: sub rsp, 0x110
14161ad81: lea rbp, [rsp + 0x80]
14161ad89: mov qword ptr [rbp + 0x88], 0xfffffffffffffffe
14161ad94: mov rdi, rdx
14161ad97: mov rsi, rcx
14161ad9a: test r8, r8
14161ad9d: je 0x14161adb7
14161ad9f: mov rbx, r9
14161ada2: cmp r9, 0x40001
14161ada9: jae 0x14161af7b
14161adaf: mov r15, r8
14161adb2: xor r12d, r12d
14161adb5: jmp 0x14161adf1
14161adb7: movzx eax, byte ptr [rip + 0x2019e4b]
14161adbe: mov ebx, 0x40000
14161adc3: mov ecx, 0x40000
14161adc8: mov edx, 1
14161adcd: call 0x140613c10
14161add2: test rax, rax
14161add5: je 0x14161afe5
14161addb: mov r15, rax
14161adde: mov r8d, 0x40000
14161ade4: mov rcx, rax
14161ade7: xor edx, edx
14161ade9: call 0x142923c20
14161adee: mov r12b, 1
14161adf1: mov qword ptr [rbp + 0x78], r15
14161adf5: mov byte ptr [rbp + 0x87], r12b
14161adfc: lea rcx, [rbp + 0x20]
14161ae00: mov edx, 0x40000
14161ae05: mov r8d, 0x22
14161ae0b: mov r9b, 2
14161ae0e: call 0x140bd4de0
14161ae13: mov qword ptr [rbp + 0x78], r15
14161ae17: mov byte ptr [rbp + 0x87], r12b
14161ae1e: lea r14, [rbp + 8]
14161ae22: lea rdx, [rbp + 0x20]
14161ae26: mov r8d, 1
14161ae2c: mov rcx, r14
14161ae2f: call 0x140bd4f20
14161ae34: mov qword ptr [rbp + 0x78], r15
14161ae38: mov byte ptr [rbp + 0x87], r12b
14161ae3f: mov qword ptr [rsp + 0x30], rbx
14161ae44: mov qword ptr [rsp + 0x28], r15
14161ae49: mov qword ptr [rsp + 0x20], r14
14161ae4e: lea r8, [rip + 0x16651fb]
14161ae55: lea rcx, [rbp - 0x48]
14161ae59: mov r9d, 0x1d
14161ae5f: mov rdx, rdi
14161ae62: call 0x1416a2dc0
14161ae67: cmp dword ptr [rbp - 0x48], 0xb
14161ae6b: jne 0x14161af0d
14161ae71: mov rax, qword ptr [rbp - 0x20]
14161ae75: mov qword ptr [rbp + 0x40], rax
14161ae79: movups xmm0, xmmword ptr [rbp - 0x40]
14161ae7d: movups xmm1, xmmword ptr [rbp - 0x30]
14161ae81: movaps xmmword ptr [rbp + 0x30], xmm1
14161ae85: movaps xmmword ptr [rbp + 0x20], xmm0
14161ae89: test r12b, r12b
14161ae8c: je 0x14161aea1
14161ae8e: mov edx, 0x40000
14161ae93: mov r8d, 1
14161ae99: mov rcx, r15
14161ae9c: call 0x140613c20
14161aea1: lea rcx, [rbp + 0x20]
14161aea5: mov rdx, rdi
14161aea8: call 0x140bd4cd0
14161aead: test rax, rax
14161aeb0: je 0x14161af6a
14161aeb6: cmp rdx, qword ptr [rbp + 0x38]
14161aeba: jb 0x14161aef0
14161aebc: mov rcx, qword ptr [rbp + 0x40]
14161aec0: mov qword ptr [rsi + 0x30], rcx
14161aec4: movaps xmm0, xmmword ptr [rbp + 0x20]
14161aec8: movaps xmm1, xmmword ptr [rbp + 0x30]
14161aecc: movups xmmword ptr [rsi + 0x20], xmm1
14161aed0: movups xmmword ptr [rsi + 0x10], xmm0
14161aed4: mov qword ptr [rsi], rax
14161aed7: mov qword ptr [rsi + 8], rdx
14161aedb: mov rax, rsi
14161aede: add rsp, 0x110
14161aee5: pop rbx
14161aee6: pop rdi
14161aee7: pop rsi
14161aee8: pop r12
14161aeea: pop r14
14161aeec: pop r15
14161aeee: pop rbp
14161aeef: ret 
14161aef0: lea rcx, [rip + 0x16652f9]
14161aef7: lea r8, [rip + 0x166532a]
14161aefe: mov edx, 0x37
14161af03: call 0x1428d9430
14161af08: jmp 0x14161b002
14161af0d: mov dword ptr [rbp + 0x74], r12d
14161af11: mov qword ptr [rbp + 0x18], r15
14161af15: movups xmm0, xmmword ptr [rbp - 8]
14161af19: movaps xmmword ptr [rbp + 0x60], xmm0
14161af1d: movups xmm0, xmmword ptr [rbp - 0x48]
14161af21: movups xmm1, xmmword ptr [rbp - 0x38]
14161af25: movups xmm2, xmmword ptr [rbp - 0x28]
14161af29: movups xmm3, xmmword ptr [rbp - 0x18]
14161af2d: movaps xmmword ptr [rbp + 0x50], xmm3
14161af31: movaps xmmword ptr [rbp + 0x40], xmm2
14161af35: movaps xmmword ptr [rbp + 0x30], xmm1
14161af39: movaps xmmword ptr [rbp + 0x20], xmm0
14161af3d: lea rax, [rip + 0x1665264]
14161af44: mov qword ptr [rsp + 0x20], rax
14161af49: lea rcx, [rip + 0x1663b98]
14161af50: lea r9, [rip + 0x1663b71]
14161af57: lea r8, [rbp + 0x20]
14161af5b: mov edx, 0x2b
14161af60: call 0x1428d9760
14161af65: jmp 0x14161b002
14161af6a: lea rcx, [rip + 0x1665267]
14161af71: call 0x1428d9310
14161af76: jmp 0x14161b002
14161af7b: lea rax, [rip + 0x1482647]
14161af82: mov qword ptr [rbp - 0x48], rax
14161af86: mov qword ptr [rbp - 0x40], 0x49
14161af8e: lea rax, [rbp - 0x48]
14161af92: mov qword ptr [rbp + 8], rax
14161af96: lea rax, [rip - 0x151110d]
14161af9d: mov qword ptr [rbp + 0x10], rax
14161afa1: lea rax, [rip + 0x1664820]
14161afa8: mov qword ptr [rbp + 0x20], rax
14161afac: mov qword ptr [rbp + 0x28], 1
14161afb4: mov qword ptr [rbp + 0x40], 0
14161afbc: lea rax, [rbp + 8]
14161afc0: mov qword ptr [rbp + 0x30], rax
14161afc4: mov qword ptr [rbp + 0x38], 1
14161afcc: mov byte ptr [rbp + 0x87], 0
14161afd3: lea rdx, [rip + 0x16647fe]
14161afda: lea rcx, [rbp + 0x20]
14161afde: call 0x1428d9390
14161afe3: jmp 0x14161b002
14161afe5: mov byte ptr [rbp + 0x87], 0
14161afec: lea r8, [rip + 0x1663abd]
14161aff3: mov ecx, 1
14161aff8: mov edx, 0x40000
14161affd: call 0x1428d8fe3
14161b002: ud2 
14161b004: nop word ptr cs:[rax + rax]
14161b010: mov qword ptr [rsp + 0x10], rdx
14161b015: push rbp
14161b016: push r15
14161b018: push r14
14161b01a: push r12
14161b01c: push rsi
14161b01d: push rdi
14161b01e: push rbx
14161b01f: sub rsp, 0x40
14161b023: lea rbp, [rdx + 0x80]
14161b02a: test byte ptr [rbp + 0x87], 1
14161b031: je 0x14161b047
14161b033: mov rcx, qword ptr [rbp + 0x78]
14161b037: mov edx, 0x40000
14161b03c: mov r8d, 1
14161b042: call 0x140613c20
14161b047: nop 
14161b048: add rsp, 0x40
14161b04c: pop rbx
14161b04d: pop rdi
14161b04e: pop rsi
14161b04f: pop r12
14161b051: pop r14
14161b053: pop r15
14161b055: pop rbp
14161b056: ret 
14161b057: nop word ptr [rax + rax]
14161b060: mov qword ptr [rsp + 0x10], rdx
14161b065: push rbp
14161b066: push r15
14161b068: push r14
14161b06a: push r12
14161b06c: push rsi
14161b06d: push rdi
14161b06e: push rbx
14161b06f: sub rsp, 0x40
14161b073: lea rbp, [rdx + 0x80]
14161b07a: lea rcx, [rbp + 0x20]
14161b07e: call 0x140be24b0
14161b083: nop 
14161b084: add rsp, 0x40
14161b088: pop rbx
14161b089: pop rdi
14161b08a: pop rsi
14161b08b: pop r12
14161b08d: pop r14
14161b08f: pop r15
14161b091: pop rbp
14161b092: ret 
14161b093: nop word ptr cs:[rax + rax]
14161b0a0: mov qword ptr [rsp + 0x10], rdx
14161b0a5: push rbp
14161b0a6: push r15
14161b0a8: push r14
14161b0aa: push r12
14161b0ac: push rsi
14161b0ad: push rdi
14161b0ae: push rbx
14161b0af: sub rsp, 0x40
14161b0b3: lea rbp, [rdx + 0x80]
14161b0ba: mov rsi, qword ptr [rbp + 0x18]
14161b0be: mov qword ptr [rbp + 0x78], rsi
14161b0c2: mov edi, dword ptr [rbp + 0x74]
14161b0c5: mov byte ptr [rbp + 0x87], dil
14161b0cc: lea rcx, [rbp + 0x20]
14161b0d0: call 0x1407cea70
14161b0d5: mov qword ptr [rbp + 0x78], rsi
14161b0d9: mov byte ptr [rbp + 0x87], dil
14161b0e0: add rsp, 0x40
14161b0e4: pop rbx
14161b0e5: pop rdi
14161b0e6: pop rsi
14161b0e7: pop r12
14161b0e9: pop r14
14161b0eb: pop r15
14161b0ed: pop rbp
14161b0ee: ret 
