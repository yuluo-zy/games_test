140aff8e0: push rbp
140aff8e1: push r14
140aff8e3: push rsi
140aff8e4: push rdi
140aff8e5: push rbx
140aff8e6: sub rsp, 0xe0
140aff8ed: lea rbp, [rsp + 0x80]
140aff8f5: mov qword ptr [rbp + 0x58], 0xfffffffffffffffe
140aff8fd: mov rsi, rcx
140aff900: mov dword ptr [rbp - 0x58], edx
140aff903: mov qword ptr [rbp + 8], 0
140aff90b: mov qword ptr [rbp + 0x10], 1
140aff913: mov qword ptr [rbp + 0x18], 0
140aff91b: mov qword ptr [rbp - 0x38], 0
140aff923: mov qword ptr [rbp - 0x28], 0
140aff92b: mov qword ptr [rbp - 0x18], 0x20
140aff933: mov byte ptr [rbp - 0x10], 3
140aff937: lea rax, [rbp + 8]
140aff93b: mov qword ptr [rbp - 8], rax
140aff93f: lea rax, [rip + 0x1fe6cca]
140aff946: mov qword ptr [rbp], rax
140aff94a: lea rbx, [rbp - 0x58]
140aff94e: lea rdi, [rbp - 0x38]
140aff952: mov rcx, rbx
140aff955: mov rdx, rdi
140aff958: call 0x140d8d560
140aff95d: test al, al
140aff95f: jne 0x140aff9f4
140aff965: mov rax, qword ptr [rbp + 8]
140aff969: mov qword ptr [rbp + 0x48], rax
140aff96d: mov r8, qword ptr [rbp + 0x10]
140aff971: mov r14, qword ptr [rbp + 0x18]
140aff975: lea rcx, [rip + 0x1fe6c7c]
140aff97c: mov qword ptr [rbp - 0x48], rcx
140aff980: mov qword ptr [rbp - 0x40], 0xa
140aff988: mov qword ptr [rbp - 0x58], r8
140aff98c: mov qword ptr [rbp - 0x50], r14
140aff990: mov byte ptr [rbp + 0x57], 0
140aff994: mov edx, 0xa
140aff999: mov qword ptr [rbp + 0x40], r8
140aff99d: mov r9, r14
140aff9a0: call 0x140d6c690
140aff9a5: test rax, rax
140aff9a8: jne 0x140affa1e
140aff9aa: mov rax, qword ptr [rbp + 0x48]
140aff9ae: mov qword ptr [rbp - 0x38], rax
140aff9b2: mov rax, qword ptr [rbp + 0x40]
140aff9b6: mov qword ptr [rbp - 0x30], rax
140aff9ba: mov qword ptr [rbp - 0x28], r14
140aff9be: mov byte ptr [rbp + 0x57], 1
140aff9c2: lea rcx, [rip + 0x1fe6c39]
140aff9c9: lea r8, [rbp - 0x38]
140aff9cd: mov edx, 0xb
140aff9d2: call 0x1412e6c30
140aff9d7: mov byte ptr [rbp + 0x57], 1
140aff9db: mov rcx, rsi
140aff9de: call 0x140b859f0
140aff9e3: mov rax, rsi
140aff9e6: add rsp, 0xe0
140aff9ed: pop rbx
140aff9ee: pop rdi
140aff9ef: pop rsi
140aff9f0: pop r14
140aff9f2: pop rbp
140aff9f3: ret 
140aff9f4: lea rax, [rip + 0x1fe6ccd]
140aff9fb: mov qword ptr [rsp + 0x20], rax
140affa00: lea rcx, [rip + 0x1fe6c39]
140affa07: lea r9, [rip + 0x1fe6cd2]
140affa0e: lea r8, [rbp - 0x48]
140affa12: mov edx, 0x37
140affa17: call 0x1428d9760
140affa1c: jmp 0x140affa87
140affa1e: mov qword ptr [rbp + 0x38], rax
140affa22: lea rax, [rbp - 0x48]
140affa26: mov qword ptr [rbp - 0x38], rax
140affa2a: lea rax, [rip - 0xad1161]
140affa31: mov qword ptr [rbp - 0x30], rax
140affa35: mov qword ptr [rbp - 0x28], rbx
140affa39: mov qword ptr [rbp - 0x20], rax
140affa3d: lea rax, [rbp + 0x38]
140affa41: mov qword ptr [rbp - 0x18], rax
140affa45: lea rax, [rip + 0x25b5c4]
140affa4c: mov qword ptr [rbp - 0x10], rax
140affa50: lea rax, [rip + 0x212f439]
140affa57: mov qword ptr [rbp + 8], rax
140affa5b: mov qword ptr [rbp + 0x10], 3
140affa63: mov qword ptr [rbp + 0x28], 0
140affa6b: mov qword ptr [rbp + 0x18], rdi
140affa6f: mov qword ptr [rbp + 0x20], 3
140affa77: lea rdx, [rip + 0x212f48a]
140affa7e: lea rcx, [rbp + 8]
140affa82: call 0x1428d9390
140affa87: ud2 
140affa89: nop dword ptr [rax]
140affa90: mov qword ptr [rsp + 0x10], rdx
140affa95: push rbp
140affa96: push r14
140affa98: push rsi
140affa99: push rdi
140affa9a: push rbx
140affa9b: sub rsp, 0x30
140affa9f: lea rbp, [rdx + 0x80]
140affaa6: mov rdx, qword ptr [rbp + 8]
140affaaa: test rdx, rdx
140affaad: je 0x140affabe
140affaaf: mov rcx, qword ptr [rbp + 0x10]
140affab3: mov r8d, 1
140affab9: call 0x140613c20
140affabe: nop 
140affabf: add rsp, 0x30
140affac3: pop rbx
140affac4: pop rdi
140affac5: pop rsi
140affac6: pop r14
140affac8: pop rbp
140affac9: ret 
140affaca: nop word ptr [rax + rax]
140affad0: mov qword ptr [rsp + 0x10], rdx
140affad5: push rbp
140affad6: push r14
140affad8: push rsi
140affad9: push rdi
140affada: push rbx
140affadb: sub rsp, 0x30
140affadf: lea rbp, [rdx + 0x80]
140affae6: cmp qword ptr [rbp + 0x48], 0
140affaeb: sete al
140affaee: or al, byte ptr [rbp + 0x57]
140affaf1: test al, 1
140affaf3: jne 0x140affb08
140affaf5: mov r8d, 1
140affafb: mov rcx, qword ptr [rbp + 0x40]
140affaff: mov rdx, qword ptr [rbp + 0x48]
140affb03: call 0x140613c20
140affb08: nop 
140affb09: add rsp, 0x30
140affb0d: pop rbx
140affb0e: pop rdi
140affb0f: pop rsi
140affb10: pop r14
140affb12: pop rbp
140affb13: ret 
140affb14: nop word ptr cs:[rax + rax]
140affb20: mov qword ptr [rsp + 0x10], rdx
140affb25: push rbp
140affb26: push r14
140affb28: push rsi
140affb29: push rdi
140affb2a: push rbx
140affb2b: sub rsp, 0x30
140affb2f: lea rbp, [rdx + 0x80]
140affb36: mov byte ptr [rbp + 0x57], 0
140affb3a: lea rcx, [rbp + 0x38]
140affb3e: call 0x1405e7a00
140affb43: mov byte ptr [rbp + 0x57], 0
140affb47: add rsp, 0x30
140affb4b: pop rbx
140affb4c: pop rdi
140affb4d: pop rsi
140affb4e: pop r14
140affb50: pop rbp
140affb51: ret 
