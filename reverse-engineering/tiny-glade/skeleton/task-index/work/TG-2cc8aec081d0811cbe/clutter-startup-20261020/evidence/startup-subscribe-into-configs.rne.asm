141cedf80: 55 push rbp
141cedf81: 56 push rsi
141cedf82: 57 push rdi
141cedf83: 4881ecb0010000 sub rsp, 0x1b0
141cedf8a: 488dac2480000000 lea rbp, [rsp + 0x80]
141cedf92: 48c78528010000feffffff mov qword ptr [rbp + 0x128], 0xfffffffffffffffe
141cedf9d: 4889ce mov rsi, rcx
141cedfa0: 0f57c0 xorps xmm0, xmm0
141cedfa3: 0f114508 movups xmmword ptr [rbp + 8], xmm0
141cedfa7: 0f114520 movups xmmword ptr [rbp + 0x20], xmm0
141cedfab: 0f114538 movups xmmword ptr [rbp + 0x38], xmm0
141cedfaf: 0f114550 movups xmmword ptr [rbp + 0x50], xmm0
141cedfb3: 0f114568 movups xmmword ptr [rbp + 0x68], xmm0
141cedfb7: c7457800000000 mov dword ptr [rbp + 0x78], 0
141cedfbe: 0f1185a0000000 movups xmmword ptr [rbp + 0xa0], xmm0
141cedfc5: 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
141cedfcc: 0f1185d0000000 movups xmmword ptr [rbp + 0xd0], xmm0
141cedfd3: 0f1185e8000000 movups xmmword ptr [rbp + 0xe8], xmm0
141cedfda: 0f118500010000 movups xmmword ptr [rbp + 0x100], xmm0
141cedfe1: c7851001000000000000 mov dword ptr [rbp + 0x110], 0
141cedfeb: 48c745a000000000 mov qword ptr [rbp - 0x60], 0
141cedff3: 48c745e800000000 mov qword ptr [rbp - 0x18], 0
141cedffb: 48c745f008000000 mov qword ptr [rbp - 0x10], 8
141cee003: 48c745f800000000 mov qword ptr [rbp - 8], 0
141cee00b: 48c7450010000000 mov qword ptr [rbp], 0x10
141cee013: 48c7451810000000 mov qword ptr [rbp + 0x18], 0x10
141cee01b: 48c7453010000000 mov qword ptr [rbp + 0x30], 0x10
141cee023: 48c7454810000000 mov qword ptr [rbp + 0x48], 0x10
141cee02b: 48c7456010000000 mov qword ptr [rbp + 0x60], 0x10
141cee033: 48b80000000000000080 movabs rax, 0x8000000000000000
141cee03d: 48898580000000 mov qword ptr [rbp + 0x80], rax
141cee044: 488d058d952401 lea rax, [rip + 0x124958d]
141cee04b: 48898588000000 mov qword ptr [rbp + 0x88], rax
141cee052: 48c785900000005c000000 mov qword ptr [rbp + 0x90], 0x5c
141cee05d: 48c7859800000010000000 mov qword ptr [rbp + 0x98], 0x10
141cee068: 48c785b000000010000000 mov qword ptr [rbp + 0xb0], 0x10
141cee073: 48c785c800000010000000 mov qword ptr [rbp + 0xc8], 0x10
141cee07e: 48c785e000000010000000 mov qword ptr [rbp + 0xe0], 0x10
141cee089: 48c785f800000010000000 mov qword ptr [rbp + 0xf8], 0x10
141cee094: c7851801000000000000 mov dword ptr [rbp + 0x118], 0
141cee09e: 66c7851c0100000100 mov word ptr [rbp + 0x11c], 1
141cee0a7: c7852001000000000000 mov dword ptr [rbp + 0x120], 0
141cee0b1: 0fb605516b9401 movzx eax, byte ptr [rip + 0x1946b51]
141cee0b8: b988010000 mov ecx, 0x188
141cee0bd: ba08000000 mov edx, 8
141cee0c2: e8495b92fe call 0x140613c10
141cee0c7: 4885c0 test rax, rax
141cee0ca: 7435 je 0x141cee101
141cee0cc: 4889c7 mov rdi, rax
141cee0cf: 488d55a0 lea rdx, [rbp - 0x60]
141cee0d3: 41b888010000 mov r8d, 0x188
141cee0d9: 4889c1 mov rcx, rax
141cee0dc: e81f5bc300 call 0x142923c00
141cee0e1: 4c8d0580932401 lea r8, [rip + 0x1249380]
141cee0e8: 4889f1 mov rcx, rsi
141cee0eb: 4889fa mov rdx, rdi
141cee0ee: e81d6afffe call 0x140ce4b10
141cee0f3: 4889f0 mov rax, rsi
141cee0f6: 4881c4b0010000 add rsp, 0x1b0
141cee0fd: 5f pop rdi
141cee0fe: 5e pop rsi
141cee0ff: 5d pop rbp
141cee100: c3 ret 
141cee101: b908000000 mov ecx, 8
141cee106: ba88010000 mov edx, 0x188
141cee10b: e8f0aebe00 call 0x1428d9000
141cee110: 0f0b ud2 
141cee112: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
141cee120: 4889542410 mov qword ptr [rsp + 0x10], rdx
141cee125: 55 push rbp
141cee126: 56 push rsi
141cee127: 57 push rdi
141cee128: 4883ec20 sub rsp, 0x20
141cee12c: 488daa80000000 lea rbp, [rdx + 0x80]
141cee133: 488d4da0 lea rcx, [rbp - 0x60]
141cee137: e8142d8efe call 0x1405d0e50
141cee13c: 90 nop 
141cee13d: 4883c420 add rsp, 0x20
141cee141: 5f pop rdi
141cee142: 5e pop rsi
141cee143: 5d pop rbp
141cee144: c3 ret 
