1412c5600: push rbp
1412c5601: push rsi
1412c5602: sub rsp, 0xe8
1412c5609: lea rbp, [rsp + 0x80]
1412c5611: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412c5619: mov rsi, rcx
1412c561c: lea rcx, [rbp - 0x58]
1412c5620: xor r8d, r8d
1412c5623: call 0x14161f660
1412c5628: movzx eax, byte ptr [rip + 0x236f5da]
1412c562f: mov ecx, 0x38
1412c5634: mov edx, 8
1412c5639: call 0x140613c10
1412c563e: test rax, rax
1412c5641: je 0x1412c56ff
1412c5647: mov rcx, qword ptr [rbp - 0x28]
1412c564b: mov qword ptr [rax + 0x30], rcx
1412c564f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c5653: movups xmm1, xmmword ptr [rbp - 0x48]
1412c5657: movups xmm2, xmmword ptr [rbp - 0x38]
1412c565b: movups xmmword ptr [rax + 0x20], xmm2
1412c565f: movups xmmword ptr [rax + 0x10], xmm1
1412c5663: movups xmmword ptr [rax], xmm0
1412c5666: lea rcx, [rip + 0x17d92d4]
1412c566d: mov qword ptr [rbp + 0x10], rcx
1412c5671: mov qword ptr [rbp + 0x18], 0x34
1412c5679: mov qword ptr [rbp + 0x20], rax
1412c567d: lea rax, [rip + 0x1967be4]
1412c5684: mov qword ptr [rbp + 0x28], rax
1412c5688: mov qword ptr [rbp + 0x40], 0
1412c5690: mov byte ptr [rbp + 0x56], 5
1412c5694: mov qword ptr [rbp + 0x30], rcx
1412c5698: mov qword ptr [rbp + 0x38], 0x34
1412c56a0: movzx eax, byte ptr [rip + 0x236f562]
1412c56a7: mov ecx, 0x40
1412c56ac: mov edx, 8
1412c56b1: call 0x140613c10
1412c56b6: test rax, rax
1412c56b9: je 0x1412c5710
1412c56bb: movups xmm0, xmmword ptr [rbp + 0x20]
1412c56bf: movups xmm1, xmmword ptr [rbp + 0x30]
1412c56c3: movups xmm2, xmmword ptr [rbp + 0x40]
1412c56c7: movups xmm3, xmmword ptr [rbp + 0x50]
1412c56cb: movups xmmword ptr [rax + 0x30], xmm3
1412c56cf: movups xmmword ptr [rax + 0x20], xmm2
1412c56d3: movups xmmword ptr [rax + 0x10], xmm1
1412c56d7: movups xmmword ptr [rax], xmm0
1412c56da: movabs rdx, 0x6fd8113d23595a99
1412c56e4: mov rcx, rsi
1412c56e7: mov r8, rax
1412c56ea: call 0x140969bf0
1412c56ef: cmp rax, 1
1412c56f3: je 0x1412c5721
1412c56f5: add rsp, 0xe8
1412c56fc: pop rsi
1412c56fd: pop rbp
1412c56fe: ret 
1412c56ff: mov ecx, 8
1412c5704: mov edx, 0x38
1412c5709: call 0x1428d9000
1412c570e: jmp 0x1412c571f
1412c5710: mov ecx, 8
1412c5715: mov edx, 0x40
1412c571a: call 0x1428d9000
1412c571f: ud2 
1412c5721: mov rax, rdx
1412c5724: add rdx, 0x10
1412c5728: mov rsi, rdx
1412c572b: cmp qword ptr [rax + 0x18], 0x34
1412c5730: jne 0x1412c57af
1412c5732: mov rdx, qword ptr [rsi]
1412c5735: lea rcx, [rip + 0x17d9205]
1412c573c: mov r8d, 0x34
1412c5742: call 0x142923bf0
1412c5747: test eax, eax
1412c5749: jne 0x1412c57af
1412c574b: lea rax, [rip + 0x196892e]
1412c5752: mov qword ptr [rbp - 0x20], rax
1412c5756: lea rax, [rip - 0xd91ebd]
1412c575d: mov qword ptr [rbp - 0x18], rax
1412c5761: lea rax, [rbp + 0x10]
1412c5765: mov qword ptr [rbp - 0x10], rax
1412c5769: lea rax, [rip - 0x11bb8e0]
1412c5770: mov qword ptr [rbp - 8], rax
1412c5774: lea rax, [rip + 0x1967ead]
1412c577b: mov qword ptr [rbp + 0x20], rax
1412c577f: mov qword ptr [rbp + 0x28], 3
1412c5787: mov qword ptr [rbp + 0x40], 0
1412c578f: lea rax, [rbp - 0x20]
1412c5793: mov qword ptr [rbp + 0x30], rax
1412c5797: mov qword ptr [rbp + 0x38], 2
1412c579f: lea rdx, [rip + 0x1967eb2]
1412c57a6: lea rcx, [rbp + 0x20]
1412c57aa: call 0x1428d9390
1412c57af: lea rax, [rip + 0x19688ca]
1412c57b6: mov qword ptr [rbp + 0x20], rax
1412c57ba: lea rax, [rip - 0xd91f21]
1412c57c1: mov qword ptr [rbp + 0x28], rax
1412c57c5: mov qword ptr [rbp + 0x30], rsi
1412c57c9: lea rax, [rip - 0x11bb940]
1412c57d0: mov qword ptr [rbp + 0x38], rax
1412c57d4: lea rcx, [rbp + 0x10]
1412c57d8: mov qword ptr [rbp + 0x40], rcx
1412c57dc: mov qword ptr [rbp + 0x48], rax
1412c57e0: lea rax, [rip + 0x1967d81]
1412c57e7: mov qword ptr [rbp - 0x20], rax
1412c57eb: mov qword ptr [rbp - 0x18], 3
1412c57f3: mov qword ptr [rbp], 0
1412c57fb: lea rax, [rbp + 0x20]
1412c57ff: mov qword ptr [rbp - 0x10], rax
1412c5803: mov qword ptr [rbp - 8], 3
1412c580b: lea rdx, [rip + 0x1967dbe]
1412c5812: lea rcx, [rbp - 0x20]
1412c5816: call 0x1428d9390
1412c581b: int3 
1412c581c: nop dword ptr [rax]
1412c5820: mov qword ptr [rsp + 0x10], rdx
1412c5825: push rbp
1412c5826: push rsi
1412c5827: sub rsp, 0x28
1412c582b: lea rbp, [rdx + 0x80]
1412c5832: lea rcx, [rbp - 0x48]
1412c5836: call 0x140be24b0
1412c583b: nop 
1412c583c: add rsp, 0x28
1412c5840: pop rsi
1412c5841: pop rbp
1412c5842: ret 
1412c5843: nop word ptr cs:[rax + rax]
1412c5850: mov qword ptr [rsp + 0x10], rdx
1412c5855: push rbp
1412c5856: push rsi
1412c5857: sub rsp, 0x28
1412c585b: lea rbp, [rdx + 0x80]
1412c5862: lea rcx, [rbp + 0x20]
1412c5866: call 0x1400c04b0
1412c586b: nop 
1412c586c: add rsp, 0x28
1412c5870: pop rsi
1412c5871: pop rbp
1412c5872: ret 
