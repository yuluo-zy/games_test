141593c80: 55  push rbp
141593c81: 4157  push r15
141593c83: 4156  push r14
141593c85: 4154  push r12
141593c87: 56  push rsi
141593c88: 57  push rdi
141593c89: 53  push rbx
141593c8a: 4881ec90000000  sub rsp, 0x90
141593c91: 488dac2480000000  lea rbp, [rsp + 0x80]
141593c99: 48c74508feffffff  mov qword ptr [rbp + 8], 0xfffffffffffffffe
141593ca1: 4d89c4  mov r12, r8
141593ca4: 4d85c0  test r8, r8
141593ca7: 0f8894010000  js 0x141593e41
141593cad: 4c89cf  mov rdi, r9
141593cb0: 4889ce  mov rsi, rcx
141593cb3: 7444  je 0x141593cf9
141593cb5: 4989d6  mov r14, rdx
141593cb8: 0fb6054a0f0a02  movzx eax, byte ptr [rip + 0x20a0f4a]
141593cbf: 41bf01000000  mov r15d, 1
141593cc5: ba01000000  mov edx, 1
141593cca: 4c89e1  mov rcx, r12
141593ccd: e83eff07ff  call 0x140613c10
141593cd2: 4885c0  test rax, rax
141593cd5: 0f8469010000  je 0x141593e44
141593cdb: 4889c3  mov rbx, rax
141593cde: 4889c1  mov rcx, rax
141593ce1: 4c89f2  mov rdx, r14
141593ce4: 4d89e0  mov r8, r12
141593ce7: e814ff3801  call 0x142923c00
141593cec: 488b16  mov rdx, qword ptr [rsi]
141593cef: 4889d0  mov rax, rdx
141593cf2: 48f7d8  neg rax
141593cf5: 7112  jno 0x141593d09
141593cf7: eb21  jmp 0x141593d1a
141593cf9: bb01000000  mov ebx, 1
141593cfe: 488b16  mov rdx, qword ptr [rsi]
141593d01: 4889d0  mov rax, rdx
141593d04: 48f7d8  neg rax
141593d07: 7011  jo 0x141593d1a
141593d09: 730f  jae 0x141593d1a
141593d0b: 488b4e08  mov rcx, qword ptr [rsi + 8]
141593d0f: 41b801000000  mov r8d, 1
141593d15: e806ff07ff  call 0x140613c20
141593d1a: 48895e08  mov qword ptr [rsi + 8], rbx
141593d1e: 4c896610  mov qword ptr [rsi + 0x10], r12
141593d22: 48b80000000000000080  movabs rax, 0x8000000000000000
141593d2c: 488906  mov qword ptr [rsi], rax
141593d2f: 488b4608  mov rax, qword ptr [rsi + 8]
141593d33: 488945d0  mov qword ptr [rbp - 0x30], rax
141593d37: 488b4610  mov rax, qword ptr [rsi + 0x10]
141593d3b: 488945d8  mov qword ptr [rbp - 0x28], rax
141593d3f: 4c8965c8  mov qword ptr [rbp - 0x38], r12
141593d43: 0fb61f  movzx ebx, byte ptr [rdi]
141593d46: 0fb605bc0e0a02  movzx eax, byte ptr [rip + 0x20a0ebc]
141593d4d: b905000000  mov ecx, 5
141593d52: ba01000000  mov edx, 1
141593d57: e8b4fe07ff  call 0x140613c10
141593d5c: 84db  test bl, bl
141593d5e: 741b  je 0x141593d7b
141593d60: 4885c0  test rax, rax
141593d63: 0f84ed000000  je 0x141593e56
141593d69: 0fb60dd5e36f01  movzx ecx, byte ptr [rip + 0x16fe3d5]
141593d70: 884804  mov byte ptr [rax + 4], cl
141593d73: 8b0dc8e36f01  mov ecx, dword ptr [rip + 0x16fe3c8]
141593d79: eb19  jmp 0x141593d94
141593d7b: 4885c0  test rax, rax
141593d7e: 0f84ee000000  je 0x141593e72
141593d84: 0fb60db5e36f01  movzx ecx, byte ptr [rip + 0x16fe3b5]
141593d8b: 884804  mov byte ptr [rax + 4], cl
141593d8e: 8b0da8e36f01  mov ecx, dword ptr [rip + 0x16fe3a8]
141593d94: 8908  mov dword ptr [rax], ecx
141593d96: c645a803  mov byte ptr [rbp - 0x58], 3
141593d9a: 48c745b005000000  mov qword ptr [rbp - 0x50], 5
141593da2: 488945b8  mov qword ptr [rbp - 0x48], rax
141593da6: 48c745c005000000  mov qword ptr [rbp - 0x40], 5
141593dae: 4883c618  add rsi, 0x18
141593db2: 488d4de0  lea rcx, [rbp - 0x20]
141593db6: 4c8d45c8  lea r8, [rbp - 0x38]
141593dba: 4c8d4da8  lea r9, [rbp - 0x58]
141593dbe: 4889f2  mov rdx, rsi
141593dc1: e86a7ceaff  call 0x14143ba30
141593dc6: 0fb645e0  movzx eax, byte ptr [rbp - 0x20]
141593dca: 4883f806  cmp rax, 6
141593dce: 7754  ja 0x141593e24
141593dd0: 488d0db1476e01  lea rcx, [rip + 0x16e47b1]
141593dd7: 48630481  movsxd rax, dword ptr [rcx + rax*4]
141593ddb: 4801c8  add rax, rcx
141593dde: ffe0  jmp rax
141593de0: 488b55e8  mov rdx, qword ptr [rbp - 0x18]
141593de4: 4885d2  test rdx, rdx
141593de7: 7444  je 0x141593e2d
141593de9: 488b4df0  mov rcx, qword ptr [rbp - 0x10]
141593ded: 41b801000000  mov r8d, 1
141593df3: eb28  jmp 0x141593e1d
141593df5: 488b4df0  mov rcx, qword ptr [rbp - 0x10]
141593df9: 488b55f8  mov rdx, qword ptr [rbp - 8]
141593dfd: 48894d00  mov qword ptr [rbp], rcx
141593e01: e86a4f2fff  call 0x140888d70
141593e06: 488b55e8  mov rdx, qword ptr [rbp - 0x18]
141593e0a: 4885d2  test rdx, rdx
141593e0d: 741e  je 0x141593e2d
141593e0f: 48c1e205  shl rdx, 5
141593e13: 41b808000000  mov r8d, 8
141593e19: 488b4d00  mov rcx, qword ptr [rbp]
141593e1d: e8fefd07ff  call 0x140613c20
141593e22: eb09  jmp 0x141593e2d
141593e24: 488d4de8  lea rcx, [rbp - 0x18]
141593e28: e8135130ff  call 0x140898f40
141593e2d: 31c0  xor eax, eax
141593e2f: 4881c490000000  add rsp, 0x90
141593e36: 5b  pop rbx
141593e37: 5f  pop rdi
141593e38: 5e  pop rsi
141593e39: 415c  pop r12
141593e3b: 415e  pop r14
141593e3d: 415f  pop r15
141593e3f: 5d  pop rbp
141593e40: c3  ret 
141593e41: 4531ff  xor r15d, r15d
141593e44: 4c8d058d436e01  lea r8, [rip + 0x16e438d]
141593e4b: 4c89f9  mov rcx, r15
141593e4e: 4c89e2  mov rdx, r12
141593e51: e88d513401  call 0x1428d8fe3
141593e56: 4c896500  mov qword ptr [rbp], r12
141593e5a: 4c8d056fdd6f01  lea r8, [rip + 0x16fdd6f]
141593e61: b901000000  mov ecx, 1
141593e66: ba05000000  mov edx, 5
141593e6b: e873513401  call 0x1428d8fe3
141593e70: eb1a  jmp 0x141593e8c
141593e72: 4c896500  mov qword ptr [rbp], r12
141593e76: 4c8d0553dd6f01  lea r8, [rip + 0x16fdd53]
141593e7d: b901000000  mov ecx, 1
141593e82: ba05000000  mov edx, 5
141593e87: e857513401  call 0x1428d8fe3
141593e8c: 0f0b  ud2 
141593e8e: 6690  nop 
141593e90: 4889542410  mov qword ptr [rsp + 0x10], rdx
141593e95: 55  push rbp
141593e96: 4157  push r15
141593e98: 4156  push r14
141593e9a: 4154  push r12
141593e9c: 56  push rsi
141593e9d: 57  push rdi
141593e9e: 53  push rbx
141593e9f: 4883ec20  sub rsp, 0x20
141593ea3: 488daa80000000  lea rbp, [rdx + 0x80]
141593eaa: 48837d0000  cmp qword ptr [rbp], 0
141593eaf: 7413  je 0x141593ec4
141593eb1: 488b4dd0  mov rcx, qword ptr [rbp - 0x30]
141593eb5: 41b801000000  mov r8d, 1
141593ebb: 488b5500  mov rdx, qword ptr [rbp]
141593ebf: e85cfd07ff  call 0x140613c20
141593ec4: 90  nop 
141593ec5: 4883c420  add rsp, 0x20
141593ec9: 5b  pop rbx
141593eca: 5f  pop rdi
141593ecb: 5e  pop rsi
141593ecc: 415c  pop r12
141593ece: 415e  pop r14
141593ed0: 415f  pop r15
141593ed2: 5d  pop rbp
141593ed3: c3  ret 
141593ed4: 6666662e0f1f840000000000  nop word ptr cs:[rax + rax]
141593ee0: 4889542410  mov qword ptr [rsp + 0x10], rdx
141593ee5: 55  push rbp
141593ee6: 4157  push r15
141593ee8: 4156  push r14
141593eea: 4154  push r12
141593eec: 56  push rsi
141593eed: 57  push rdi
141593eee: 53  push rbx
141593eef: 4883ec20  sub rsp, 0x20
141593ef3: 488daa80000000  lea rbp, [rdx + 0x80]
141593efa: 488b55e8  mov rdx, qword ptr [rbp - 0x18]
141593efe: 4885d2  test rdx, rdx
141593f01: 7413  je 0x141593f16
141593f03: 48c1e205  shl rdx, 5
141593f07: 41b808000000  mov r8d, 8
141593f0d: 488b4d00  mov rcx, qword ptr [rbp]
141593f11: e80afd07ff  call 0x140613c20
141593f16: 90  nop 
141593f17: 4883c420  add rsp, 0x20
141593f1b: 5b  pop rbx
141593f1c: 5f  pop rdi
141593f1d: 5e  pop rsi
141593f1e: 415c  pop r12
141593f20: 415e  pop r14
141593f22: 415f  pop r15
141593f24: 5d  pop rbp
141593f25: c3  ret 
