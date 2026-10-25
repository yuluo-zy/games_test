14145b1a0: push rsi
14145b1a1: push rdi
14145b1a2: push rbx
14145b1a3: sub rsp, 0x70
14145b1a7: movabs rax, 0x736f6d6570736575
14145b1b1: mov qword ptr [rsp + 0x28], rax
14145b1b6: movabs rax, 0x6c7967656e657261
14145b1c0: mov qword ptr [rsp + 0x30], rax
14145b1c5: movabs rax, 0x646f72616e646f6d
14145b1cf: mov qword ptr [rsp + 0x38], rax
14145b1d4: movabs rax, 0x7465646279746573
14145b1de: mov qword ptr [rsp + 0x40], rax
14145b1e3: xorps xmm0, xmm0
14145b1e6: movups xmmword ptr [rsp + 0x48], xmm0
14145b1eb: movups xmmword ptr [rsp + 0x58], xmm0
14145b1f0: mov qword ptr [rsp + 0x68], 0
14145b1f9: mov rsi, qword ptr [rcx]
14145b1fc: mov rdi, qword ptr [rcx + 8]
14145b200: mov qword ptr [rsp + 0x20], rdi
14145b205: lea rbx, [rsp + 0x28]
14145b20a: lea rdx, [rsp + 0x20]
14145b20f: mov r8d, 8
14145b215: mov rcx, rbx
14145b218: call 0x140106160
14145b21d: mov rcx, rbx
14145b220: mov rdx, rsi
14145b223: mov r8, rdi
14145b226: call 0x140106160
14145b22b: mov r9, qword ptr [rsp + 0x58]
14145b230: shl r9, 0x38
14145b234: or r9, qword ptr [rsp + 0x60]
14145b239: mov rdx, qword ptr [rsp + 0x38]
14145b23e: mov rcx, qword ptr [rsp + 0x40]
14145b243: xor rcx, r9
14145b246: mov rax, qword ptr [rsp + 0x28]
14145b24b: add rax, rdx
14145b24e: mov r8, qword ptr [rsp + 0x30]
14145b253: rol rdx, 0xd
14145b257: add r8, rcx
14145b25a: xor rdx, rax
14145b25d: rol rcx, 0x10
14145b261: xor rcx, r8
14145b264: rol rax, 0x20
14145b268: add r8, rdx
14145b26b: add rax, rcx
14145b26e: rol rdx, 0x11
14145b272: xor rdx, r8
14145b275: rol rcx, 0x15
14145b279: rol r8, 0x20
14145b27d: xor rcx, rax
14145b280: xor rax, r9
14145b283: xor r8, 0xff
14145b28a: add rax, rdx
14145b28d: add r8, rcx
14145b290: rol rdx, 0xd
14145b294: xor rdx, rax
14145b297: rol rcx, 0x10
14145b29b: rol rax, 0x20
14145b29f: xor rcx, r8
14145b2a2: add r8, rdx
14145b2a5: rol rdx, 0x11
14145b2a9: add rax, rcx
14145b2ac: xor rdx, r8
14145b2af: rol rcx, 0x15
14145b2b3: xor rcx, rax
14145b2b6: rol r8, 0x20
14145b2ba: add rax, rdx
14145b2bd: add r8, rcx
14145b2c0: rol rdx, 0xd
14145b2c4: xor rdx, rax
14145b2c7: rol rcx, 0x10
14145b2cb: rol rax, 0x20
14145b2cf: xor rcx, r8
14145b2d2: add r8, rdx
14145b2d5: rol rdx, 0x11
14145b2d9: add rax, rcx
14145b2dc: xor rdx, r8
14145b2df: rol rcx, 0x15
14145b2e3: xor rcx, rax
14145b2e6: rol r8, 0x20
14145b2ea: add rax, rdx
14145b2ed: add r8, rcx
14145b2f0: rol rdx, 0xd
14145b2f4: xor rdx, rax
14145b2f7: rol rcx, 0x10
14145b2fb: xor rcx, r8
14145b2fe: add r8, rdx
14145b301: rol rdx, 0x11
14145b305: rol rcx, 0x15
14145b309: mov rax, r8
14145b30c: rol rax, 0x20
14145b310: xor rax, rdx
14145b313: xor rax, rcx
14145b316: xor rax, r8
14145b319: add rsp, 0x70
14145b31d: pop rbx
14145b31e: pop rdi
14145b31f: pop rsi
14145b320: ret 
