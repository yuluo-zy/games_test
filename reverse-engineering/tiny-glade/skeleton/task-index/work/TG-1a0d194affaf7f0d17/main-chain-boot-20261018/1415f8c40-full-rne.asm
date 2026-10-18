1415f8c40: push rsi
1415f8c41: sub rsp, 0x30
1415f8c45: cmp r8, 5
1415f8c49: je 0x1415f8c8a
1415f8c4b: cmp r8, 3
1415f8c4f: jne 0x1415f8ceb
1415f8c55: movzx eax, word ptr [rdx]
1415f8c58: xor eax, 0x3349
1415f8c5d: movzx r9d, byte ptr [rdx + 2]
1415f8c62: xor r9d, 0x32
1415f8c66: or r9w, ax
1415f8c6a: je 0x1415f8ccf
1415f8c6c: movzx eax, word ptr [rdx]
1415f8c6f: xor eax, 0x3346
1415f8c74: movzx r9d, byte ptr [rdx + 2]
1415f8c79: xor r9d, 0x32
1415f8c7d: or r9w, ax
1415f8c81: jne 0x1415f8ceb
1415f8c83: mov word ptr [rcx], 0x132
1415f8c88: jmp 0x1415f8ce2 ; country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
1415f8c8a: mov eax, 0x78323346
1415f8c8f: mov r9d, dword ptr [rdx]
1415f8c92: xor r9d, eax
1415f8c95: movzx r10d, byte ptr [rdx + 4]
1415f8c9a: xor r10d, 0x32
1415f8c9e: or r10d, r9d
1415f8ca1: je 0x1415f8cd6
1415f8ca3: xor eax, dword ptr [rdx]
1415f8ca5: movzx r9d, byte ptr [rdx + 4]
1415f8caa: xor r9d, 0x33
1415f8cae: or r9d, eax
1415f8cb1: je 0x1415f8cdd
1415f8cb3: mov eax, 0x78323346
1415f8cb8: xor eax, dword ptr [rdx]
1415f8cba: movzx r9d, byte ptr [rdx + 4]
1415f8cbf: xor r9d, 0x34
1415f8cc3: or r9d, eax
1415f8cc6: jne 0x1415f8ceb
1415f8cc8: mov word ptr [rcx], 0x432
1415f8ccd: jmp 0x1415f8ce2 ; country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
1415f8ccf: mov word ptr [rcx], 0x32
1415f8cd4: jmp 0x1415f8ce2 ; country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
1415f8cd6: mov word ptr [rcx], 0x232
1415f8cdb: jmp 0x1415f8ce2 ; country_core::startup::load_mesh_atlases::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
1415f8cdd: mov word ptr [rcx], 0x332
1415f8ce2: mov rax, rcx
1415f8ce5: add rsp, 0x30
1415f8ce9: pop rsi
1415f8cea: ret 
1415f8ceb: mov qword ptr [rsp + 0x20], 5
1415f8cf4: lea r9, [rip + 0x168492d]
1415f8cfb: mov rsi, rcx
1415f8cfe: call 0x1428ca640 ; ron::error::impl$5::unknown_variant
1415f8d03: mov rax, rsi
1415f8d06: add rsp, 0x30
1415f8d0a: pop rsi
1415f8d0b: ret 
