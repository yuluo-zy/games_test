1412c6000: push rbp
1412c6001: push rsi
1412c6002: push rdi
1412c6003: sub rsp, 0x140
1412c600a: lea rbp, [rsp + 0x80]
1412c6012: mov qword ptr [rbp + 0xb8], 0xfffffffffffffffe
1412c601d: mov rsi, rcx
1412c6020: lea rcx, [rbp - 0x58]
1412c6024: xor r8d, r8d
1412c6027: call 0x1412e5170
1412c602c: movzx eax, byte ptr [rip + 0x236ebd6]
1412c6033: mov ecx, 0x90
1412c6038: mov edx, 8
1412c603d: call 0x140613c10
1412c6042: test rax, rax
1412c6045: je 0x1412c6112
1412c604b: mov rdi, rax
1412c604e: lea rdx, [rbp - 0x58]
1412c6052: mov r8d, 0x90
1412c6058: mov rcx, rax
1412c605b: call 0x142923c00
1412c6060: lea rax, [rip + 0x17d7dd2]
1412c6067: mov qword ptr [rbp + 0x68], rax
1412c606b: mov qword ptr [rbp + 0x70], 0x3d
1412c6073: mov qword ptr [rbp + 0x78], rdi
1412c6077: lea rcx, [rip + 0x19672ca]
1412c607e: mov qword ptr [rbp + 0x80], rcx
1412c6085: mov qword ptr [rbp + 0x98], 0
1412c6090: mov byte ptr [rbp + 0xae], 5
1412c6097: mov qword ptr [rbp + 0x88], rax
1412c609e: mov qword ptr [rbp + 0x90], 0x3d
1412c60a9: movzx eax, byte ptr [rip + 0x236eb59]
1412c60b0: mov ecx, 0x40
1412c60b5: mov edx, 8
1412c60ba: call 0x140613c10
1412c60bf: test rax, rax
1412c60c2: je 0x1412c6123
1412c60c4: movups xmm0, xmmword ptr [rbp + 0x78]
1412c60c8: movups xmm1, xmmword ptr [rbp + 0x88]
1412c60cf: movups xmm2, xmmword ptr [rbp + 0x98]
1412c60d6: movups xmm3, xmmword ptr [rbp + 0xa8]
1412c60dd: movups xmmword ptr [rax + 0x30], xmm3
1412c60e1: movups xmmword ptr [rax + 0x20], xmm2
1412c60e5: movups xmmword ptr [rax + 0x10], xmm1
1412c60e9: movups xmmword ptr [rax], xmm0
1412c60ec: movabs rdx, 0x153e2eb3e8651534
1412c60f6: mov rcx, rsi
1412c60f9: mov r8, rax
1412c60fc: call 0x140969bf0
1412c6101: cmp rax, 1
1412c6105: je 0x1412c6134
1412c6107: add rsp, 0x140
1412c610e: pop rdi
1412c610f: pop rsi
1412c6110: pop rbp
1412c6111: ret 
1412c6112: mov ecx, 8
1412c6117: mov edx, 0x90
1412c611c: call 0x1428d9000
1412c6121: jmp 0x1412c6132
1412c6123: mov ecx, 8
1412c6128: mov edx, 0x40
1412c612d: call 0x1428d9000
1412c6132: ud2 
1412c6134: mov rax, rdx
1412c6137: add rdx, 0x10
1412c613b: mov rsi, rdx
1412c613e: cmp qword ptr [rax + 0x18], 0x3d
1412c6143: jne 0x1412c61d2
1412c6149: mov rdx, qword ptr [rsi]
1412c614c: lea rcx, [rip + 0x17d7ce6]
1412c6153: mov r8d, 0x3d
1412c6159: call 0x142923bf0
1412c615e: test eax, eax
1412c6160: jne 0x1412c61d2
1412c6162: lea rax, [rip + 0x1967cbf]
1412c6169: mov qword ptr [rbp + 0x38], rax
1412c616d: lea rax, [rip - 0xd928d4]
1412c6174: mov qword ptr [rbp + 0x40], rax
1412c6178: lea rax, [rbp + 0x68]
1412c617c: mov qword ptr [rbp + 0x48], rax
1412c6180: lea rax, [rip - 0x11bc2f7]
1412c6187: mov qword ptr [rbp + 0x50], rax
1412c618b: lea rax, [rip + 0x1967496]
1412c6192: mov qword ptr [rbp + 0x78], rax
1412c6196: mov qword ptr [rbp + 0x80], 3
1412c61a1: mov qword ptr [rbp + 0x98], 0
1412c61ac: lea rax, [rbp + 0x38]
1412c61b0: mov qword ptr [rbp + 0x88], rax
1412c61b7: mov qword ptr [rbp + 0x90], 2
1412c61c2: lea rdx, [rip + 0x196748f]
1412c61c9: lea rcx, [rbp + 0x78]
1412c61cd: call 0x1428d9390
1412c61d2: lea rax, [rip + 0x1967c4f]
1412c61d9: mov qword ptr [rbp + 0x78], rax
1412c61dd: lea rax, [rip - 0xd92944]
1412c61e4: mov qword ptr [rbp + 0x80], rax
1412c61eb: mov qword ptr [rbp + 0x88], rsi
1412c61f2: lea rax, [rip - 0x11bc369]
1412c61f9: mov qword ptr [rbp + 0x90], rax
1412c6200: lea rcx, [rbp + 0x68]
1412c6204: mov qword ptr [rbp + 0x98], rcx
1412c620b: mov qword ptr [rbp + 0xa0], rax
1412c6212: lea rax, [rip + 0x196734f]
1412c6219: mov qword ptr [rbp + 0x38], rax
1412c621d: mov qword ptr [rbp + 0x40], 3
1412c6225: mov qword ptr [rbp + 0x58], 0
1412c622d: lea rax, [rbp + 0x78]
1412c6231: mov qword ptr [rbp + 0x48], rax
1412c6235: mov qword ptr [rbp + 0x50], 3
1412c623d: lea rdx, [rip + 0x196738c]
1412c6244: lea rcx, [rbp + 0x38]
1412c6248: call 0x1428d9390
1412c624d: int3 
1412c624e: nop 
1412c6250: mov qword ptr [rsp + 0x10], rdx
1412c6255: push rbp
1412c6256: push rsi
1412c6257: push rdi
1412c6258: sub rsp, 0x20
1412c625c: lea rbp, [rdx + 0x80]
1412c6263: lea rcx, [rbp - 0x58]
1412c6267: call 0x14087f110
1412c626c: nop 
1412c626d: add rsp, 0x20
1412c6271: pop rdi
1412c6272: pop rsi
1412c6273: pop rbp
1412c6274: ret 
1412c6275: nop word ptr cs:[rax + rax]
1412c6280: mov qword ptr [rsp + 0x10], rdx
1412c6285: push rbp
1412c6286: push rsi
1412c6287: push rdi
1412c6288: sub rsp, 0x20
1412c628c: lea rbp, [rdx + 0x80]
1412c6293: lea rcx, [rbp + 0x78]
1412c6297: call 0x1400c04b0
1412c629c: nop 
1412c629d: add rsp, 0x20
1412c62a1: pop rdi
1412c62a2: pop rsi
1412c62a3: pop rbp
1412c62a4: ret 
