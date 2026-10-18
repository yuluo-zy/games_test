140b3b630: push rsi
140b3b631: push rdi
140b3b632: push rbx
140b3b633: sub rsp, 0x20
140b3b637: mov rax, rcx
140b3b63a: lea rcx, [r8 - 4]
140b3b63e: cmp rcx, 9
140b3b642: ja 0x140b3b908
140b3b648: lea r9, [rip + 0x1fb2e11]
140b3b64f: movsxd rcx, dword ptr [r9 + rcx*4]
140b3b653: add rcx, r9
140b3b656: jmp rcx
140b3b658: cmp dword ptr [rdx], 0x68746150
140b3b65e: je 0x140b3b8fc
140b3b664: lea r9, [rip + 0x1fb2abc]
140b3b66b: mov rcx, rdx
140b3b66e: mov rdi, rdx
140b3b671: mov rdx, r9
140b3b674: mov rsi, r8
140b3b677: mov rbx, rax
140b3b67a: call 0x142923bf0
140b3b67f: mov edx, eax
140b3b681: mov rax, rbx
140b3b684: test edx, edx
140b3b686: je 0x140b3b8e4
140b3b68c: mov rcx, rdi
140b3b68f: mov r8, rsi
140b3b692: lea rdx, [rip + 0x1fb2a92]
140b3b699: call 0x142923bf0
140b3b69e: test eax, eax
140b3b6a0: je 0x140b3b940
140b3b6a6: lea rdx, [rip + 0x1fb2a9e]
140b3b6ad: mov rcx, rdi
140b3b6b0: mov r8, rsi
140b3b6b3: call 0x142923bf0
140b3b6b8: mov rdx, rdi
140b3b6bb: mov r8, rsi
140b3b6be: test eax, eax
140b3b6c0: mov rax, rbx
140b3b6c3: jne 0x140b3b908
140b3b6c9: mov byte ptr [rax + 1], 0xc
140b3b6cd: jmp 0x140b3b947
140b3b6d2: movabs rcx, 0x6f7461726f636544
140b3b6dc: xor rcx, qword ptr [rdx]
140b3b6df: movzx r9d, byte ptr [rdx + 8]
140b3b6e4: xor r9, 0x72
140b3b6e8: or r9, rcx
140b3b6eb: je 0x140b3b888
140b3b6f1: jmp 0x140b3b908
140b3b6f6: movabs rcx, 0x7473656c62626f43
140b3b700: mov r9, qword ptr [rdx]
140b3b703: xor r9, rcx
140b3b706: mov r10d, dword ptr [rdx + 8]
140b3b70a: xor r10, 0x31656e6f
140b3b711: or r10, r9
140b3b714: je 0x140b3b8f0
140b3b71a: mov r9, qword ptr [rdx]
140b3b71d: xor r9, rcx
140b3b720: mov r10d, dword ptr [rdx + 8]
140b3b724: xor r10, 0x32656e6f
140b3b72b: or r10, r9
140b3b72e: je 0x140b3b934
140b3b734: xor rcx, qword ptr [rdx]
140b3b737: mov r9d, dword ptr [rdx + 8]
140b3b73b: xor r9, 0x33656e6f
140b3b742: or r9, rcx
140b3b745: je 0x140b3b93a
140b3b74b: lea r9, [rip + 0x1fb29ed]
140b3b752: mov rcx, rdx
140b3b755: mov rbx, rdx
140b3b758: mov rdx, r9
140b3b75b: mov rsi, r8
140b3b75e: mov rdi, rax
140b3b761: call 0x142923bf0
140b3b766: mov rdx, rbx
140b3b769: mov r8, rsi
140b3b76c: mov ecx, eax
140b3b76e: mov rax, rdi
140b3b771: test ecx, ecx
140b3b773: je 0x140b3b888
140b3b779: jmp 0x140b3b908
140b3b77e: mov ecx, 0x72726554
140b3b783: xor ecx, dword ptr [rdx]
140b3b785: mov r9d, 0x6e696172
140b3b78b: xor r9d, dword ptr [rdx + 3]
140b3b78f: or r9d, ecx
140b3b792: jne 0x140b3b908
140b3b798: mov byte ptr [rax + 1], 0
140b3b79c: jmp 0x140b3b947
140b3b7a1: movabs rcx, 0x6f7461726f636544
140b3b7ab: xor rcx, qword ptr [rdx]
140b3b7ae: movabs r9, 0x3556726f7461726f
140b3b7b8: xor r9, qword ptr [rdx + 3]
140b3b7bc: or r9, rcx
140b3b7bf: je 0x140b3b8ea
140b3b7c5: lea r9, [rip + 0x1fb2983]
140b3b7cc: mov rcx, rdx
140b3b7cf: mov rbx, rdx
140b3b7d2: mov rdx, r9
140b3b7d5: mov rsi, r8
140b3b7d8: mov rdi, rax
140b3b7db: call 0x142923bf0
140b3b7e0: mov rdx, rbx
140b3b7e3: mov r8, rsi
140b3b7e6: mov ecx, eax
140b3b7e8: mov rax, rdi
140b3b7eb: test ecx, ecx
140b3b7ed: jne 0x140b3b908
140b3b7f3: mov byte ptr [rax + 1], 0xd
140b3b7f7: jmp 0x140b3b947
140b3b7fc: mov ecx, 0x64726147
140b3b801: xor ecx, dword ptr [rdx]
140b3b803: movzx r9d, word ptr [rdx + 4]
140b3b808: xor r9d, 0x6e65
140b3b80f: or r9d, ecx
140b3b812: je 0x140b3b8f6
140b3b818: lea r9, [rip + 0x1fb293b]
140b3b81f: mov rcx, rdx
140b3b822: mov rbx, rdx
140b3b825: mov rdx, r9
140b3b828: mov rsi, r8
140b3b82b: mov rdi, rax
140b3b82e: call 0x142923bf0
140b3b833: mov rdx, rbx
140b3b836: mov r8, rsi
140b3b839: mov ecx, eax
140b3b83b: mov rax, rdi
140b3b83e: test ecx, ecx
140b3b840: jne 0x140b3b908
140b3b846: mov byte ptr [rax + 1], 0xe
140b3b84a: jmp 0x140b3b947
140b3b84f: movabs rcx, 0x6f7461726f636544
140b3b859: mov r9, qword ptr [rdx]
140b3b85c: xor r9, rcx
140b3b85f: movzx r10d, word ptr [rdx + 8]
140b3b864: xor r10, 0x3572
140b3b86b: or r10, r9
140b3b86e: je 0x140b3b8ea
140b3b870: xor rcx, qword ptr [rdx]
140b3b873: movzx r9d, word ptr [rdx + 8]
140b3b878: xor r9, 0x3472
140b3b87f: or r9, rcx
140b3b882: jne 0x140b3b908
140b3b888: mov byte ptr [rax + 1], 0xb
140b3b88c: jmp 0x140b3b947
140b3b891: mov ecx, 0x65746157
140b3b896: xor ecx, dword ptr [rdx]
140b3b898: movzx r9d, byte ptr [rdx + 4]
140b3b89d: xor r9d, 0x72
140b3b8a1: or r9d, ecx
140b3b8a4: je 0x140b3b902
140b3b8a6: mov ecx, 0x6f6c6f43
140b3b8ab: xor ecx, dword ptr [rdx]
140b3b8ad: movzx r9d, byte ptr [rdx + 4]
140b3b8b2: xor r9d, 0x72
140b3b8b6: or r9d, ecx
140b3b8b9: jne 0x140b3b908
140b3b8bb: mov byte ptr [rax + 1], 9
140b3b8bf: jmp 0x140b3b947
140b3b8c4: movabs rcx, 0x5377654e6c6c6157
140b3b8ce: xor rcx, qword ptr [rdx]
140b3b8d1: movabs r9, 0x6d65747379537765
140b3b8db: xor r9, qword ptr [rdx + 5]
140b3b8df: or r9, rcx
140b3b8e2: jne 0x140b3b908
140b3b8e4: mov byte ptr [rax + 1], 7
140b3b8e8: jmp 0x140b3b947
140b3b8ea: mov byte ptr [rax + 1], 0xa
140b3b8ee: jmp 0x140b3b947
140b3b8f0: mov byte ptr [rax + 1], 2
140b3b8f4: jmp 0x140b3b947
140b3b8f6: mov byte ptr [rax + 1], 6
140b3b8fa: jmp 0x140b3b947
140b3b8fc: mov byte ptr [rax + 1], 1
140b3b900: jmp 0x140b3b947
140b3b902: mov byte ptr [rax + 1], 5
140b3b906: jmp 0x140b3b947
140b3b908: lea r10, [rip + 0x1fb2851]
140b3b90f: mov r9d, 0xf
140b3b915: mov rcx, rdx
140b3b918: mov rdx, r8
140b3b91b: mov r8, r10
140b3b91e: mov rsi, rax
140b3b921: call 0x1428f9e20
140b3b926: mov rcx, rax
140b3b929: mov rax, rsi
140b3b92c: mov qword ptr [rsi + 8], rcx
140b3b930: mov cl, 1
140b3b932: jmp 0x140b3b949
140b3b934: mov byte ptr [rax + 1], 3
140b3b938: jmp 0x140b3b947
140b3b93a: mov byte ptr [rax + 1], 4
140b3b93e: jmp 0x140b3b947
140b3b940: mov rax, rbx
140b3b943: mov byte ptr [rbx + 1], 8
140b3b947: xor ecx, ecx
140b3b949: mov byte ptr [rax], cl
140b3b94b: add rsp, 0x20
140b3b94f: pop rbx
140b3b950: pop rdi
140b3b951: pop rsi
140b3b952: ret 
