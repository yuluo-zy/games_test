140cdd670: push rsi
140cdd671: push rdi
140cdd672: push rbx
140cdd673: sub rsp, 0x50
140cdd677: mov rdi, r8
140cdd67a: mov rbx, rdx
140cdd67d: mov rsi, rcx
140cdd680: lea rcx, [rsp + 0x20]
140cdd685: call qword ptr [r8 + 0x20]
140cdd689: cmp qword ptr [rsp + 0x20], 0
140cdd68f: jne 0x140cdd6f0
140cdd691: mov qword ptr [rsi + 0x60], rbx
140cdd695: mov qword ptr [rsi + 0x68], rdi
140cdd699: mov qword ptr [rsi], 0
140cdd6a0: mov qword ptr [rsi + 8], 8
140cdd6a8: xorps xmm0, xmm0
140cdd6ab: movups xmmword ptr [rsi + 0x10], xmm0
140cdd6af: mov qword ptr [rsi + 0x20], 8
140cdd6b7: mov qword ptr [rsi + 0x28], 0
140cdd6bf: movabs rax, 0x8000000000000000
140cdd6c9: mov qword ptr [rsi + 0x30], rax
140cdd6cd: mov qword ptr [rsi + 0x48], 0
140cdd6d5: mov qword ptr [rsi + 0x50], 8
140cdd6dd: mov qword ptr [rsi + 0x58], 0
140cdd6e5: mov rax, rsi
140cdd6e8: add rsp, 0x50
140cdd6ec: pop rbx
140cdd6ed: pop rdi
140cdd6ee: pop rsi
140cdd6ef: ret 
140cdd6f0: lea rax, [rip + 0x1e5a0a1]
140cdd6f7: mov qword ptr [rsp + 0x20], rax
140cdd6fc: mov qword ptr [rsp + 0x28], 1
140cdd705: mov qword ptr [rsp + 0x30], 8
140cdd70e: xorps xmm0, xmm0
140cdd711: movups xmmword ptr [rsp + 0x38], xmm0
140cdd716: lea rdx, [rip + 0x1e5a0f3]
140cdd71d: lea rcx, [rsp + 0x20]
140cdd722: call 0x1428d9390
140cdd727: int3 
