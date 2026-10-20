0x14128c8e0: push rsi
0x14128c8e1: push rbx
0x14128c8e2: sub rsp, 0x78
0x14128c8e6: movabs rax, 0x736f6d6570736575
0x14128c8f0: mov qword ptr [rsp + 0x30], rax
0x14128c8f5: movabs rax, 0x6c7967656e657261
0x14128c8ff: mov qword ptr [rsp + 0x38], rax
0x14128c904: movabs rax, 0x646f72616e646f6d
0x14128c90e: mov qword ptr [rsp + 0x40], rax
0x14128c913: movabs rax, 0x7465646279746573
0x14128c91d: mov qword ptr [rsp + 0x48], rax
0x14128c922: xorps xmm0, xmm0
0x14128c925: movups xmmword ptr [rsp + 0x50], xmm0
0x14128c92a: movups xmmword ptr [rsp + 0x60], xmm0
0x14128c92f: mov qword ptr [rsp + 0x70], 0
0x14128c938: mov rax, qword ptr [rcx]
0x14128c93b: movzx ebx, byte ptr [rcx + 8]
0x14128c93f: mov qword ptr [rsp + 0x28], rax
0x14128c944: lea rsi, [rsp + 0x30]
0x14128c949: lea rdx, [rsp + 0x28]
0x14128c94e: mov r8d, 8
0x14128c954: mov rcx, rsi
0x14128c957: call 0x140106160
0x14128c95c: mov byte ptr [rsp + 0x28], bl
0x14128c960: lea rdx, [rsp + 0x28]
0x14128c965: mov r8d, 1
0x14128c96b: mov rcx, rsi
0x14128c96e: call 0x140106160
0x14128c973: mov r9, qword ptr [rsp + 0x60]
0x14128c978: shl r9, 0x38
0x14128c97c: or r9, qword ptr [rsp + 0x68]
0x14128c981: mov rdx, qword ptr [rsp + 0x40]
0x14128c986: mov rcx, qword ptr [rsp + 0x48]
0x14128c98b: xor rcx, r9
0x14128c98e: mov rax, qword ptr [rsp + 0x30]
0x14128c993: add rax, rdx
0x14128c996: mov r8, qword ptr [rsp + 0x38]
0x14128c99b: rol rdx, 0xd
0x14128c99f: add r8, rcx
0x14128c9a2: xor rdx, rax
0x14128c9a5: rol rcx, 0x10
0x14128c9a9: xor rcx, r8
0x14128c9ac: rol rax, 0x20
0x14128c9b0: add r8, rdx
0x14128c9b3: add rax, rcx
0x14128c9b6: rol rdx, 0x11
0x14128c9ba: xor rdx, r8
0x14128c9bd: rol rcx, 0x15
0x14128c9c1: rol r8, 0x20
0x14128c9c5: xor rcx, rax
0x14128c9c8: xor rax, r9
0x14128c9cb: xor r8, 0xff
0x14128c9d2: add rax, rdx
0x14128c9d5: add r8, rcx
0x14128c9d8: rol rdx, 0xd
0x14128c9dc: xor rdx, rax
0x14128c9df: rol rcx, 0x10
0x14128c9e3: rol rax, 0x20
0x14128c9e7: xor rcx, r8
0x14128c9ea: add r8, rdx
0x14128c9ed: rol rdx, 0x11
0x14128c9f1: add rax, rcx
0x14128c9f4: xor rdx, r8
0x14128c9f7: rol rcx, 0x15
0x14128c9fb: xor rcx, rax
0x14128c9fe: rol r8, 0x20
0x14128ca02: add rax, rdx
0x14128ca05: add r8, rcx
0x14128ca08: rol rdx, 0xd
0x14128ca0c: xor rdx, rax
0x14128ca0f: rol rcx, 0x10
0x14128ca13: rol rax, 0x20
0x14128ca17: xor rcx, r8
0x14128ca1a: add r8, rdx
0x14128ca1d: rol rdx, 0x11
0x14128ca21: add rax, rcx
0x14128ca24: xor rdx, r8
0x14128ca27: rol rcx, 0x15
0x14128ca2b: xor rcx, rax
0x14128ca2e: rol r8, 0x20
0x14128ca32: add rax, rdx
0x14128ca35: add r8, rcx
0x14128ca38: rol rdx, 0xd
0x14128ca3c: xor rdx, rax
0x14128ca3f: rol rcx, 0x10
0x14128ca43: xor rcx, r8
0x14128ca46: add r8, rdx
0x14128ca49: rol rdx, 0x11
0x14128ca4d: rol rcx, 0x15
0x14128ca51: mov rax, r8
0x14128ca54: rol rax, 0x20
0x14128ca58: xor rax, rdx
0x14128ca5b: xor rax, rcx
0x14128ca5e: xor rax, r8
0x14128ca61: add rsp, 0x78
0x14128ca65: pop rbx
0x14128ca66: pop rsi
0x14128ca67: ret 
