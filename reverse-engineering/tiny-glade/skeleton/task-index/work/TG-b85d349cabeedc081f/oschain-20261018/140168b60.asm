140168b60: push rsi
140168b61: sub rsp, 0x70
140168b65: mov rsi, rcx
140168b68: lea rcx, [rsp + 0x28]
140168b6d: call 0x14037ec50
140168b72: movzx eax, byte ptr [rsp + 0x28]
140168b77: cmp al, 0x32
140168b79: jne 0x140168baf
140168b7b: movzx eax, byte ptr [rsp + 0x30]
140168b80: mov rdx, qword ptr [rsp + 0x38]
140168b85: lea rcx, [rip + 0x27f5b3c]
140168b8c: movsxd rax, dword ptr [rcx + rax*4]
140168b90: add rax, rcx
140168b93: jmp rax
140168b95: lea r8, [rip + 0x27f582c]
140168b9c: mov r9d, 2
140168ba2: mov rcx, rsi
140168ba5: call 0x140381190
140168baa: jmp 0x140168dbb
140168baf: mov ecx, dword ptr [rsp + 0x29]
140168bb3: mov edx, dword ptr [rsp + 0x2c]
140168bb7: mov dword ptr [rsi + 4], edx
140168bba: mov dword ptr [rsi + 1], ecx
140168bbd: movzx ecx, byte ptr [rsp + 0x30]
140168bc2: mov edx, dword ptr [rsp + 0x34]
140168bc6: mov dword ptr [rsi + 0xc], edx
140168bc9: mov edx, dword ptr [rsp + 0x31]
140168bcd: mov dword ptr [rsi + 9], edx
140168bd0: mov rdx, qword ptr [rsp + 0x38]
140168bd5: movups xmm0, xmmword ptr [rsp + 0x50]
140168bda: movups xmmword ptr [rsi + 0x28], xmm0
140168bde: movups xmm0, xmmword ptr [rsp + 0x60]
140168be3: movups xmmword ptr [rsi + 0x38], xmm0
140168be7: movups xmm0, xmmword ptr [rsp + 0x40]
140168bec: movups xmmword ptr [rsi + 0x18], xmm0
140168bf0: mov byte ptr [rsi], al
140168bf2: mov byte ptr [rsi + 8], cl
140168bf5: mov qword ptr [rsi + 0x10], rdx
140168bf9: jmp 0x140168dbb
140168bfe: movabs rax, 0x8000000000000001
140168c08: add rax, 0xf
140168c0c: jmp 0x140168db4
140168c11: lea r8, [rip + 0x27f5830]
140168c18: mov r9d, 1
140168c1e: mov rcx, rsi
140168c21: call 0x140385100
140168c26: jmp 0x140168dbb
140168c2b: movabs rax, 0x8000000000000001
140168c35: or rax, 0xa
140168c39: jmp 0x140168db4
140168c3e: movabs rax, 0x8000000000000001
140168c48: add rax, 3
140168c4c: jmp 0x140168db4
140168c51: lea r8, [rip + 0x27f5800]
140168c58: mov r9d, 1
140168c5e: mov rcx, rsi
140168c61: call 0x140380580
140168c66: jmp 0x140168dbb
140168c6b: lea r8, [rip + 0x27f57d6]
140168c72: mov r9d, 1
140168c78: mov rcx, rsi
140168c7b: call 0x140382e60
140168c80: jmp 0x140168dbb
140168c85: movabs rax, 0x8000000000000001
140168c8f: inc rax
140168c92: jmp 0x140168db4
140168c97: lea r8, [rip + 0x27f57aa]
140168c9e: mov r9d, 1
140168ca4: mov rcx, rsi
140168ca7: call 0x14037f920
140168cac: jmp 0x140168dbb
140168cb1: lea rcx, [rsp + 0x28]
140168cb6: call 0x1403874d0
140168cbb: cmp byte ptr [rsp + 0x28], 0x32
140168cc0: jne 0x140168d3a
140168cc2: mov rax, qword ptr [rsp + 0x30]
140168cc7: movabs rcx, 0x8000000000000001
140168cd1: or rcx, 2
140168cd5: jmp 0x140168d30
140168cd7: movabs rax, 0x8000000000000001
140168ce1: or rax, 6
140168ce5: jmp 0x140168db4
140168cea: movabs rax, 0x8000000000000001
140168cf4: jmp 0x140168db4
140168cf9: movabs rax, 0x8000000000000001
140168d03: or rax, 0x10
140168d07: jmp 0x140168db4
140168d0c: lea rcx, [rsp + 0x28]
140168d11: call 0x1403874d0
140168d16: cmp byte ptr [rsp + 0x28], 0x32
140168d1b: jne 0x140168d3a
140168d1d: mov rax, qword ptr [rsp + 0x30]
140168d22: movabs rcx, 0x8000000000000001
140168d2c: or rcx, 4
140168d30: mov qword ptr [rsi + 8], rcx
140168d34: mov qword ptr [rsi + 0x10], rax
140168d38: jmp 0x140168db8
140168d3a: mov rax, qword ptr [rsp + 0x68]
140168d3f: mov qword ptr [rsi + 0x40], rax
140168d43: movups xmm0, xmmword ptr [rsp + 0x28]
140168d48: movups xmm1, xmmword ptr [rsp + 0x38]
140168d4d: movups xmm2, xmmword ptr [rsp + 0x48]
140168d52: movups xmm3, xmmword ptr [rsp + 0x58]
140168d57: movups xmmword ptr [rsi + 0x30], xmm3
140168d5b: movups xmmword ptr [rsi + 0x20], xmm2
140168d5f: movups xmmword ptr [rsi + 0x10], xmm1
140168d63: movups xmmword ptr [rsi], xmm0
140168d66: jmp 0x140168dbb
140168d68: lea r8, [rip + 0x27f5701]
140168d6f: mov r9d, 2
140168d75: mov rcx, rsi
140168d78: call 0x140385d60
140168d7d: jmp 0x140168dbb
140168d7f: lea r8, [rip + 0x27f5672]
140168d86: mov r9d, 1
140168d8c: mov rcx, rsi
140168d8f: call 0x140384640
140168d94: jmp 0x140168dbb
140168d96: movabs rax, 0x8000000000000001
140168da0: add rax, 0x11
140168da4: jmp 0x140168db4
140168da6: movabs rax, 0x8000000000000001
140168db0: add rax, 9
140168db4: mov qword ptr [rsi + 8], rax
140168db8: mov byte ptr [rsi], 0x32
140168dbb: mov rax, rsi
140168dbe: add rsp, 0x70
140168dc2: pop rsi
140168dc3: ret 
140168dc4: lea r8, [rip + 0x27f5655]
140168dcb: mov r9d, 2
140168dd1: mov rcx, rsi
140168dd4: call 0x1403820c0
140168dd9: jmp 0x140168dbb
140168ddb: lea r8, [rip + 0x27f56b6]
140168de2: mov r9d, 1
140168de8: mov rcx, rsi
140168deb: call 0x140383ac0
140168df0: jmp 0x140168dbb
