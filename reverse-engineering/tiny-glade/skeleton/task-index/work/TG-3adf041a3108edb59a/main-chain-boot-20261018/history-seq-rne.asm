1415a9780: push rbp
1415a9781: push r15
1415a9783: push r14
1415a9785: push r13
1415a9787: push r12
1415a9789: push rsi
1415a978a: push rdi
1415a978b: push rbx
1415a978c: sub rsp, 0x158
1415a9793: lea rbp, [rsp + 0x80]
1415a979b: mov qword ptr [rbp + 0xd0], 0xfffffffffffffffe
1415a97a6: mov rsi, rcx
1415a97a9: mov rbx, qword ptr [rdx + 0x10]
1415a97ad: lea rcx, [rbp + 0x70]
1415a97b1: call 0x140b88080 ; serde_json::value::de::SeqDeserializer::new
1415a97b6: mov r15, qword ptr [rbp + 0x78]
1415a97ba: mov rdi, qword ptr [rbp + 0x88]
1415a97c1: cmp r15, rdi
1415a97c4: je 0x1415a9819
1415a97c6: lea r14, [r15 + 0x20]
1415a97ca: mov qword ptr [rbp + 0x78], r14
1415a97ce: movzx eax, byte ptr [r15]
1415a97d2: cmp al, 6
1415a97d4: je 0x1415a981c
1415a97d6: mov byte ptr [rbp + 0x98], al
1415a97dc: movups xmm0, xmmword ptr [r15 + 1]
1415a97e1: movups xmm1, xmmword ptr [r15 + 0x10]
1415a97e6: movups xmmword ptr [rbp + 0x99], xmm0
1415a97ed: movups xmmword ptr [rbp + 0xa8], xmm1
1415a97f4: lea rcx, [rbp + 0xb8]
1415a97fb: lea rdx, [rbp + 0x98]
1415a9802: call 0x14159a8f0 ; serde_json::value::de::impl$2::deserialize_i32<serde::de::impls::impl$54::deserialize::PrimitiveVisitor>
1415a9807: cmp dword ptr [rbp + 0xb8], 1
1415a980e: jne 0x1415a988d
1415a9810: mov r15, qword ptr [rbp + 0xc0]
1415a9817: jmp 0x1415a9834
1415a9819: mov r14, r15
1415a981c: lea rdx, [rip + 0x16d1215]
1415a9823: lea r8, [rip + 0x16d0836]
1415a982a: xor ecx, ecx
1415a982c: call 0x1428f9d20 ; serde::de::Error::invalid_length<serde_json::error::Error>
1415a9831: mov r15, rax
1415a9834: mov qword ptr [rsi + 8], r15
1415a9838: movabs rax, 0x8000000000000000
1415a9842: mov qword ptr [rsi], rax
1415a9845: sub rdi, r14
1415a9848: shr rdi, 5
1415a984c: mov rcx, r14
1415a984f: mov rdx, rdi
1415a9852: call 0x140888d70 ; core::ptr::drop_in_place<slice2$<enum2$<serde_json::value::Value> > >
1415a9857: mov rdx, qword ptr [rbp + 0x80]
1415a985e: test rdx, rdx
1415a9861: je 0x1415a9876
1415a9863: mov rcx, qword ptr [rbp + 0x70]
1415a9867: shl rdx, 5
1415a986b: mov r8d, 8
1415a9871: call 0x140613c20 ; 0x140613c20
1415a9876: mov rax, rsi
1415a9879: add rsp, 0x158
1415a9880: pop rbx
1415a9881: pop rdi
1415a9882: pop rsi
1415a9883: pop r12
1415a9885: pop r13
1415a9887: pop r14
1415a9889: pop r15
1415a988b: pop rbp
1415a988c: ret 
1415a988d: mov r12d, dword ptr [rbp + 0xbc]
1415a9894: cmp r14, rdi
1415a9897: je 0x1415a98aa
1415a9899: lea r14, [r15 + 0x40]
1415a989d: mov qword ptr [rbp + 0x78], r14
1415a98a1: movzx eax, byte ptr [r15 + 0x20]
1415a98a6: cmp al, 6
1415a98a8: jne 0x1415a9920
1415a98aa: mov r15d, 0xffffffff
1415a98b0: cmp r14, rdi
1415a98b3: je 0x1415a996e
1415a98b9: lea r13, [r14 + 0x20]
1415a98bd: mov qword ptr [rbp + 0x78], r13
1415a98c1: movzx eax, byte ptr [r14]
1415a98c5: cmp al, 6
1415a98c7: je 0x1415a9971
1415a98cd: mov byte ptr [rbp + 0x98], al
1415a98d3: movups xmm0, xmmword ptr [r14 + 1]
1415a98d8: movups xmm1, xmmword ptr [r14 + 0x10]
1415a98dd: movups xmmword ptr [rbp + 0x99], xmm0
1415a98e4: movups xmmword ptr [rbp + 0xa8], xmm1
1415a98eb: lea rcx, [rbp + 0xb8]
1415a98f2: lea rdx, [rbp + 0x98]
1415a98f9: call 0x14159c190 ; serde_json::value::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::history::MultiEdit> >
1415a98fe: mov dword ptr [rbp + 0x90], r12d
1415a9905: mov r12, qword ptr [rbp + 0xb8]
1415a990c: mov rdx, qword ptr [rbp + 0xc0]
1415a9913: mov rax, r12
1415a9916: neg rax
1415a9919: jno 0x1415a9994
1415a991b: mov r15, rdx
1415a991e: jmp 0x1415a998c
1415a9920: mov byte ptr [rbp + 0x98], al
1415a9926: movups xmm0, xmmword ptr [r15 + 0x21]
1415a992b: movups xmm1, xmmword ptr [r15 + 0x30]
1415a9930: movups xmmword ptr [rbp + 0x99], xmm0
1415a9937: movups xmmword ptr [rbp + 0xa8], xmm1
1415a993e: lea rcx, [rbp + 0xb8]
1415a9945: lea rdx, [rbp + 0x98]
1415a994c: call 0x14159a8f0 ; serde_json::value::de::impl$2::deserialize_i32<serde::de::impls::impl$54::deserialize::PrimitiveVisitor>
1415a9951: cmp dword ptr [rbp + 0xb8], 1
1415a9958: je 0x1415a9810
1415a995e: mov r15d, dword ptr [rbp + 0xbc]
1415a9965: cmp r14, rdi
1415a9968: jne 0x1415a98b9
1415a996e: mov r13, r14
1415a9971: lea rdx, [rip + 0x16d10c0]
1415a9978: lea r8, [rip + 0x16d06e1]
1415a997f: mov ecx, 2
1415a9984: call 0x1428f9d20 ; serde::de::Error::invalid_length<serde_json::error::Error>
1415a9989: mov r15, rax
1415a998c: mov r14, r13
1415a998f: jmp 0x1415a9834
1415a9994: mov r8, qword ptr [rbp + 0xc8]
1415a999b: mov qword ptr [rbp - 0x30], r12
1415a999f: mov qword ptr [rbp - 0x28], rdx
1415a99a3: mov qword ptr [rbp - 0x20], r8
1415a99a7: cmp r13, rdi
1415a99aa: mov r13, rdi
1415a99ad: mov dword ptr [rbp + 0x94], r15d
1415a99b4: je 0x1415a99cb
1415a99b6: lea r13, [r14 + 0x40]
1415a99ba: mov qword ptr [rbp + 0x78], r13
1415a99be: movzx eax, byte ptr [r14 + 0x20]
1415a99c3: cmp al, 6
1415a99c5: jne 0x1415a9a91
1415a99cb: mov r15d, 8
1415a99d1: xor eax, eax
1415a99d3: xor ecx, ecx
1415a99d5: xorps xmm0, xmm0
1415a99d8: movups xmmword ptr [rbp - 0x40], xmm0
1415a99dc: mov qword ptr [rbp - 0x18], r12
1415a99e0: mov qword ptr [rbp - 0x10], rdx
1415a99e4: mov qword ptr [rbp - 8], r8
1415a99e8: mov qword ptr [rbp], rcx
1415a99ec: mov qword ptr [rbp + 8], r15
1415a99f0: mov qword ptr [rbp + 0x10], rax
1415a99f4: movabs r14, 0x8000000000000000
1415a99fe: mov qword ptr [rbp + 0x18], r14
1415a9a02: movups xmm1, xmmword ptr [rbp - 0x60]
1415a9a06: movups xmm2, xmmword ptr [rbp - 0x50]
1415a9a0a: movups xmmword ptr [rbp + 0x20], xmm1
1415a9a0e: movups xmmword ptr [rbp + 0x30], xmm2
1415a9a12: movups xmmword ptr [rbp + 0x40], xmm0
1415a9a16: mov eax, dword ptr [rbp + 0x90]
1415a9a1c: mov dword ptr [rbp + 0x50], eax
1415a9a1f: mov eax, dword ptr [rbp + 0x94]
1415a9a25: mov dword ptr [rbp + 0x54], eax
1415a9a28: mov dword ptr [rbp + 0x58], 0
1415a9a2f: cmp rdi, r13
1415a9a32: jne 0x1415a9b02
1415a9a38: mov rax, qword ptr [rbp + 0x58]
1415a9a3c: mov qword ptr [rsi + 0x70], rax
1415a9a40: mov rax, qword ptr [rbp + 0x48]
1415a9a44: mov qword ptr [rsi + 0x60], rax
1415a9a48: mov eax, dword ptr [rbp + 0x50]
1415a9a4b: mov dword ptr [rsi + 0x68], eax
1415a9a4e: mov eax, dword ptr [rbp + 0x54]
1415a9a51: mov dword ptr [rsi + 0x6c], eax
1415a9a54: movups xmm0, xmmword ptr [rbp + 0x38]
1415a9a58: movups xmmword ptr [rsi + 0x50], xmm0
1415a9a5c: movups xmm0, xmmword ptr [rbp + 0x28]
1415a9a60: movups xmmword ptr [rsi + 0x40], xmm0
1415a9a64: movups xmm0, xmmword ptr [rbp - 0x18]
1415a9a68: movups xmm1, xmmword ptr [rbp - 8]
1415a9a6c: movups xmm2, xmmword ptr [rbp + 8]
1415a9a70: movups xmm3, xmmword ptr [rbp + 0x18]
1415a9a74: movups xmmword ptr [rsi + 0x30], xmm3
1415a9a78: movups xmmword ptr [rsi + 0x20], xmm2
1415a9a7c: movups xmmword ptr [rsi + 0x10], xmm1
1415a9a80: movups xmmword ptr [rsi], xmm0
1415a9a83: lea rcx, [rbp + 0x70]
1415a9a87: call 0x14088a560 ; core::ptr::drop_in_place<serde_json::value::de::SeqDeserializer>
1415a9a8c: jmp 0x1415a9876
1415a9a91: mov qword ptr [rbp + 0x68], r8
1415a9a95: mov qword ptr [rbp + 0x60], rdx
1415a9a99: mov byte ptr [rbp + 0x98], al
1415a9a9f: movups xmm0, xmmword ptr [r14 + 0x21]
1415a9aa4: movups xmm1, xmmword ptr [r14 + 0x30]
1415a9aa9: movups xmmword ptr [rbp + 0x99], xmm0
1415a9ab0: movups xmmword ptr [rbp + 0xa8], xmm1
1415a9ab7: lea rcx, [rbp + 0xb8]
1415a9abe: lea rdx, [rbp + 0x98]
1415a9ac5: call 0x140b363e0 ; country_core::resources::history::impl$2::deserialize<enum2$<serde_json::value::Value> >
1415a9aca: mov rcx, qword ptr [rbp + 0xb8]
1415a9ad1: mov r15, qword ptr [rbp + 0xc0]
1415a9ad8: mov rax, rcx
1415a9adb: neg rax
1415a9ade: mov rdx, qword ptr [rbp + 0x60]
1415a9ae2: mov r8, qword ptr [rbp + 0x68]
1415a9ae6: jno 0x1415a9af6
1415a9ae8: lea rcx, [rbp - 0x30]
1415a9aec: call 0x140894d60 ; core::ptr::drop_in_place<country_core::resources::history::history_chain::HistoryEditChain>
1415a9af1: jmp 0x1415a998c
1415a9af6: mov rax, qword ptr [rbp + 0xc8]
1415a9afd: jmp 0x1415a99d5
1415a9b02: lea rdx, [rip + 0x16d0547]
1415a9b09: lea r8, [rip + 0x16d0550]
1415a9b10: mov rcx, rbx
1415a9b13: call 0x1428f9d20 ; serde::de::Error::invalid_length<serde_json::error::Error>
1415a9b18: mov qword ptr [rsi + 8], rax
1415a9b1c: mov qword ptr [rsi], r14
1415a9b1f: lea rcx, [rbp - 0x18]
1415a9b23: call 0x14088acf0 ; core::ptr::drop_in_place<country_core::resources::history::History>
1415a9b28: jmp 0x1415a9a83
1415a9b2d: nop dword ptr [rax]
1415a9b30: mov qword ptr [rsp + 0x10], rdx
1415a9b35: push rbp
1415a9b36: push r15
1415a9b38: push r14
1415a9b3a: push r13
1415a9b3c: push r12
1415a9b3e: push rsi
1415a9b3f: push rdi
1415a9b40: push rbx
1415a9b41: sub rsp, 0x28
1415a9b45: lea rbp, [rdx + 0x80]
1415a9b4c: lea rcx, [rbp - 0x18]
1415a9b50: call 0x14088acf0 ; core::ptr::drop_in_place<country_core::resources::history::History>
1415a9b55: nop 
1415a9b56: add rsp, 0x28
1415a9b5a: pop rbx
1415a9b5b: pop rdi
1415a9b5c: pop rsi
1415a9b5d: pop r12
1415a9b5f: pop r13
1415a9b61: pop r14
1415a9b63: pop r15
1415a9b65: pop rbp
1415a9b66: ret 
1415a9b67: nop word ptr [rax + rax]
1415a9b70: mov qword ptr [rsp + 0x10], rdx
1415a9b75: push rbp
1415a9b76: push r15
1415a9b78: push r14
1415a9b7a: push r13
1415a9b7c: push r12
1415a9b7e: push rsi
1415a9b7f: push rdi
1415a9b80: push rbx
1415a9b81: sub rsp, 0x28
1415a9b85: lea rbp, [rdx + 0x80]
1415a9b8c: lea rcx, [rbp - 0x30]
1415a9b90: call 0x140894d60 ; core::ptr::drop_in_place<country_core::resources::history::history_chain::HistoryEditChain>
1415a9b95: nop 
1415a9b96: add rsp, 0x28
1415a9b9a: pop rbx
1415a9b9b: pop rdi
1415a9b9c: pop rsi
1415a9b9d: pop r12
1415a9b9f: pop r13
1415a9ba1: pop r14
1415a9ba3: pop r15
1415a9ba5: pop rbp
1415a9ba6: ret 
1415a9ba7: nop word ptr [rax + rax]
1415a9bb0: mov qword ptr [rsp + 0x10], rdx
1415a9bb5: push rbp
1415a9bb6: push r15
1415a9bb8: push r14
1415a9bba: push r13
1415a9bbc: push r12
1415a9bbe: push rsi
1415a9bbf: push rdi
1415a9bc0: push rbx
1415a9bc1: sub rsp, 0x28
1415a9bc5: lea rbp, [rdx + 0x80]
1415a9bcc: mov rdx, qword ptr [rbp + 0x80]
1415a9bd3: test rdx, rdx
1415a9bd6: je 0x1415a9beb
1415a9bd8: mov rcx, qword ptr [rbp + 0x70]
1415a9bdc: shl rdx, 5
1415a9be0: mov r8d, 8
1415a9be6: call 0x140613c20 ; 0x140613c20
1415a9beb: nop 
1415a9bec: add rsp, 0x28
1415a9bf0: pop rbx
1415a9bf1: pop rdi
1415a9bf2: pop rsi
1415a9bf3: pop r12
1415a9bf5: pop r13
1415a9bf7: pop r14
1415a9bf9: pop r15
1415a9bfb: pop rbp
1415a9bfc: ret 
1415a9bfd: nop dword ptr [rax]
1415a9c00: mov qword ptr [rsp + 0x10], rdx
1415a9c05: push rbp
1415a9c06: push r15
1415a9c08: push r14
1415a9c0a: push r13
1415a9c0c: push r12
1415a9c0e: push rsi
1415a9c0f: push rdi
1415a9c10: push rbx
1415a9c11: sub rsp, 0x28
1415a9c15: lea rbp, [rdx + 0x80]
1415a9c1c: lea rcx, [rbp + 0x70]
1415a9c20: call 0x14088a560 ; core::ptr::drop_in_place<serde_json::value::de::SeqDeserializer>
1415a9c25: nop 
1415a9c26: add rsp, 0x28
1415a9c2a: pop rbx
1415a9c2b: pop rdi
1415a9c2c: pop rsi
1415a9c2d: pop r12
1415a9c2f: pop r13
1415a9c31: pop r14
1415a9c33: pop r15
1415a9c35: pop rbp
1415a9c36: ret 
