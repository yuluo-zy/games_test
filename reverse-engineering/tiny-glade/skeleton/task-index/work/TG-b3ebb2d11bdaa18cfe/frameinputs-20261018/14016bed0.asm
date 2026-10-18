14016bed0: push r15
14016bed2: push r14
14016bed4: push r13
14016bed6: push r12
14016bed8: push rsi
14016bed9: push rdi
14016beda: push rbx
14016bedb: sub rsp, 0x20
14016bedf: mov rsi, rdx
14016bee2: mov rdi, rcx
14016bee5: mov r14, qword ptr [rcx]
14016bee8: test r14, r14
14016beeb: je 0x14016bf47
14016beed: mov rax, qword ptr [rdi + 8]
14016bef1: mov rbx, qword ptr [rdi + 0x20]
14016bef5: cmp r14, rax
14016bef8: je 0x14016bf09
14016befa: sub rax, r14
14016befd: add rax, -0x10
14016bf01: and rax, 0xfffffffffffffff0
14016bf05: mov rsi, qword ptr [r14 + rax]
14016bf09: test rbx, rbx
14016bf0c: je 0x14016bf47
14016bf0e: mov r15, qword ptr [rdi + 0x10]
14016bf12: mov rdx, qword ptr [rdi + 0x18]
14016bf16: mov r12, qword ptr [r15 + 0x10]
14016bf1a: cmp rdx, r12
14016bf1d: je 0x14016bf40
14016bf1f: mov rax, qword ptr [r15 + 8]
14016bf23: shl rdx, 4
14016bf27: add rdx, rax
14016bf2a: mov rcx, r12
14016bf2d: shl rcx, 4
14016bf31: add rcx, rax
14016bf34: mov r8, rbx
14016bf37: shl r8, 4
14016bf3b: call 0x142923c10
14016bf40: add r12, rbx
14016bf43: mov qword ptr [r15 + 0x10], r12
14016bf47: mov rbx, qword ptr [rdi + 0x28]
14016bf4b: test rbx, rbx
14016bf4e: je 0x14016bfc7
14016bf50: mov rcx, qword ptr [rdi + 0x30]
14016bf54: mov r8, rbx
14016bf57: cmp rbx, rcx
14016bf5a: je 0x14016bf76
14016bf5c: mov rax, rcx
14016bf5f: sub rax, rbx
14016bf62: add rax, -0x10
14016bf66: and rax, 0xfffffffffffffff0
14016bf6a: lea r8, [rbx + rax]
14016bf6e: add r8, 0x10
14016bf72: mov rsi, qword ptr [rbx + rax]
14016bf76: mov rax, qword ptr [rdi + 0x38]
14016bf7a: mov rdx, qword ptr [rdi + 0x40]
14016bf7e: mov r15, qword ptr [rdi + 0x48]
14016bf82: cmp rcx, r8
14016bf85: test r15, r15
14016bf88: je 0x14016bfc7
14016bf8a: lea r12, [rax + 0x10]
14016bf8e: mov r13, qword ptr [rax + 0x10]
14016bf92: cmp rdx, r13
14016bf95: je 0x14016bfbb
14016bf97: mov rax, qword ptr [rax + 8]
14016bf9b: shl rdx, 4
14016bf9f: add rdx, rax
14016bfa2: mov rcx, r13
14016bfa5: shl rcx, 4
14016bfa9: add rcx, rax
14016bfac: mov r8, r15
14016bfaf: shl r8, 4
14016bfb3: call 0x142923c10
14016bfb8: mov rdx, r13
14016bfbb: add rdx, r15
14016bfbe: mov qword ptr [r12], rdx
14016bfc2: mov rax, qword ptr [rdi]
14016bfc5: jmp 0x14016bfca
14016bfc7: mov rax, r14
14016bfca: test r14, r14
14016bfcd: sete cl
14016bfd0: test rax, rax
14016bfd3: setne dl
14016bfd6: test cl, dl
14016bfd8: je 0x14016c035
14016bfda: mov qword ptr [rdi], 8
14016bfe1: mov rcx, qword ptr [rdi + 8]
14016bfe5: mov r14, qword ptr [rdi + 0x20]
14016bfe9: mov qword ptr [rdi + 8], 8
14016bff1: cmp rcx, rax
14016bff4: test r14, r14
14016bff7: je 0x14016c035
14016bff9: mov rax, qword ptr [rdi + 0x10]
14016bffd: mov rdx, qword ptr [rdi + 0x18]
14016c001: lea r15, [rax + 0x10]
14016c005: mov r12, qword ptr [rax + 0x10]
14016c009: cmp rdx, r12
14016c00c: je 0x14016c02f
14016c00e: mov rax, qword ptr [rax + 8]
14016c012: shl rdx, 4
14016c016: add rdx, rax
14016c019: mov rcx, r12
14016c01c: shl rcx, 4
14016c020: add rcx, rax
14016c023: mov r8, r14
14016c026: shl r8, 4
14016c02a: call 0x142923c10
14016c02f: add r12, r14
14016c032: mov qword ptr [r15], r12
14016c035: test rbx, rbx
14016c038: setne cl
14016c03b: mov rax, qword ptr [rdi + 0x28]
14016c03f: test rax, rax
14016c042: sete dl
14016c045: or dl, cl
14016c047: jne 0x14016c0a5
14016c049: mov qword ptr [rdi + 0x28], 8
14016c051: mov rcx, qword ptr [rdi + 0x30]
14016c055: mov rbx, qword ptr [rdi + 0x48]
14016c059: mov qword ptr [rdi + 0x30], 8
14016c061: cmp rcx, rax
14016c064: test rbx, rbx
14016c067: je 0x14016c0a5
14016c069: mov rax, qword ptr [rdi + 0x38]
14016c06d: mov rdx, qword ptr [rdi + 0x40]
14016c071: lea rdi, [rax + 0x10]
14016c075: mov r14, qword ptr [rax + 0x10]
14016c079: cmp rdx, r14
14016c07c: je 0x14016c09f
14016c07e: mov rax, qword ptr [rax + 8]
14016c082: shl rdx, 4
14016c086: add rdx, rax
14016c089: mov rcx, r14
14016c08c: shl rcx, 4
14016c090: add rcx, rax
14016c093: mov r8, rbx
14016c096: shl r8, 4
14016c09a: call 0x142923c10
14016c09f: add r14, rbx
14016c0a2: mov qword ptr [rdi], r14
14016c0a5: mov rax, rsi
14016c0a8: add rsp, 0x20
14016c0ac: pop rbx
14016c0ad: pop rdi
14016c0ae: pop rsi
14016c0af: pop r12
14016c0b1: pop r13
14016c0b3: pop r14
14016c0b5: pop r15
14016c0b7: ret 
