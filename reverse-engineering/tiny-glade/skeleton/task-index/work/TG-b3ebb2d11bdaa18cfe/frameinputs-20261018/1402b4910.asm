1402b4910: push rsi
1402b4911: sub rsp, 0x90
1402b4918: mov rsi, rcx
1402b491b: add rcx, 0x38
1402b491f: movabs rdx, 0x6ecb4684a4c9e1f9
1402b4929: movabs r8, 0xfae2e21abac81ddc
1402b4933: call 0x1400dd6f0
1402b4938: test rax, rax
1402b493b: je 0x1402b4a24
1402b4941: cmp qword ptr [rsi + 0x218], rdx
1402b4948: jbe 0x1402b4a24
1402b494e: mov rax, qword ptr [rsi + 0x210]
1402b4955: mov rcx, qword ptr [rax + rdx*8]
1402b4959: test rcx, rcx
1402b495c: je 0x1402b4a24
1402b4962: mov rax, qword ptr [rsi + 0x1e0]
1402b4969: not rcx
1402b496c: lea rcx, [rcx + rcx*2]
1402b4970: shl rcx, 5
1402b4974: cmp qword ptr [rax + rcx + 0x20], 0
1402b497a: je 0x1402b4a24
1402b4980: add rax, rcx
1402b4983: mov rcx, qword ptr [rax + 0x10]
1402b4987: mov edx, dword ptr [rsi + 0x5b0]
1402b498d: mov dword ptr [rax + 0x5c], edx
1402b4990: mov rax, qword ptr [rcx + 0x40]
1402b4994: mov qword ptr [rcx + 0x18], rax
1402b4998: lea rdx, [rcx + 0x20]
1402b499c: mov qword ptr [rcx + 0x38], rax
1402b49a0: mov rax, qword ptr [rcx + 8]
1402b49a4: mov r8, qword ptr [rcx + 0x10]
1402b49a8: mov qword ptr [rcx + 0x10], 0
1402b49b0: mov r9, r8
1402b49b3: shl r9, 4
1402b49b7: add r9, rax
1402b49ba: mov r10, qword ptr [rcx + 0x30]
1402b49be: mov qword ptr [rcx + 0x30], 0
1402b49c6: mov r11, qword ptr [rcx + 0x28]
1402b49ca: mov rsi, r10
1402b49cd: shl rsi, 4
1402b49d1: add rsi, r11
1402b49d4: mov qword ptr [rsp + 0x20], rax
1402b49d9: mov qword ptr [rsp + 0x28], r9
1402b49de: mov qword ptr [rsp + 0x30], rcx
1402b49e3: mov qword ptr [rsp + 0x38], r8
1402b49e8: mov qword ptr [rsp + 0x40], 0
1402b49f1: mov qword ptr [rsp + 0x48], r11
1402b49f6: mov qword ptr [rsp + 0x50], rsi
1402b49fb: mov qword ptr [rsp + 0x58], rdx
1402b4a00: mov qword ptr [rsp + 0x60], r10
1402b4a05: mov qword ptr [rsp + 0x68], 0
1402b4a0e: lea rcx, [rsp + 0x20]
1402b4a13: xor edx, edx
1402b4a15: call 0x14016bed0
1402b4a1a: nop 
1402b4a1b: add rsp, 0x90
1402b4a22: pop rsi
1402b4a23: ret 
1402b4a24: lea rax, [rip + 0x27483bd]
1402b4a2b: mov qword ptr [rsp + 0x80], rax
1402b4a33: mov qword ptr [rsp + 0x88], 0x42
1402b4a3f: lea rax, [rsp + 0x80]
1402b4a47: mov qword ptr [rsp + 0x70], rax
1402b4a4c: lea rax, [rip + 0x27ee4d]
1402b4a53: mov qword ptr [rsp + 0x78], rax
1402b4a58: lea rax, [rip + 0x26f9801]
1402b4a5f: mov qword ptr [rsp + 0x20], rax
1402b4a64: mov qword ptr [rsp + 0x28], 1
1402b4a6d: mov qword ptr [rsp + 0x40], 0
1402b4a76: lea rax, [rsp + 0x70]
1402b4a7b: mov qword ptr [rsp + 0x30], rax
1402b4a80: mov qword ptr [rsp + 0x38], 1
1402b4a89: lea rdx, [rip + 0x26f97e0]
1402b4a90: lea rcx, [rsp + 0x20]
1402b4a95: call 0x1428d9390
1402b4a9a: int3 
