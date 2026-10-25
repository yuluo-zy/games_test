000000014085ebc0: push rbp
000000014085ebc1: push rsi
000000014085ebc2: sub rsp, 0xe8
000000014085ebc9: lea rbp, [rsp + 0x80]
000000014085ebd1: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
000000014085ebd9: movzx eax, byte ptr [rdx]
000000014085ebdc: lea r8, [rip + 0x22370d5] ; 142a95cb8 7590dcfd348fdcfd7b90dcfd438fdcfde68fdcfd908fdcfda698dcfdc199dcfd1699dcfd349adcfd579adcfdd29adcfd6099dcfde099dcfd0b9adcfd929adcfd6c9adcfdd29adcfdd29adcfdd29adcfd b'u\x90\xdc\xfd4\x8f\xdc\xfd{\x90\xdc\xfdC\x8f\xdc\xfd\xe6\x8f\xdc\xfd\x90\x8f\xdc\xfd\xa6\x98\xdc\xfd\xc1\x99\xdc\xfd\x16\x99\xdc\xfd4\x9a\xdc\xfdW\x9a\xdc\xfd\xd2\x9a\xdc\xfd`\x99\xdc\xfd\xe0\x99\xdc\xfd\x0b\x9a\xdc\xfd\x92\x9a\xdc\xfdl\x9a\xdc\xfd\xd2\x9a\xdc\xfd\xd2\x9a\xdc\xfd\xd2\x9a\xdc\xfd'
000000014085ebe3: movsxd rax, dword ptr [r8 + rax*4]
000000014085ebe7: add rax, r8
000000014085ebea: jmp rax
000000014085ebec: movzx eax, byte ptr [rdx + 1]
000000014085ebf0: mov byte ptr [rcx + 1], al
000000014085ebf3: mov byte ptr [rcx], 0
000000014085ebf6: jmp 0x14085ed22
000000014085ebfb: mov r9, qword ptr [rdx + 8]
000000014085ebff: mov r10, qword ptr [rdx + 0x10]
000000014085ec03: mov rax, qword ptr [rdx + 0x18]
000000014085ec07: cmp rax, 5
000000014085ec0b: je 0x14085ecb9
000000014085ec11: cmp rax, 0xf
000000014085ec15: jne 0x14085ecd9
000000014085ec1b: movabs rdx, 0x746957656e6f7453
000000014085ec25: xor rdx, qword ptr [r10]
000000014085ec28: movabs r8, 0x737472696b536874
000000014085ec32: xor r8, qword ptr [r10 + 7]
000000014085ec36: or r8, rdx
000000014085ec39: jne 0x14085ed97
000000014085ec3f: mov byte ptr [rcx + 1], 1
000000014085ec43: jmp 0x14085ed08
000000014085ec48: mov rsi, rcx
000000014085ec4b: mov rax, qword ptr [rdx + 0x18]
000000014085ec4f: mov qword ptr [rbp - 0x30], rax
000000014085ec53: movups xmm0, xmmword ptr [rdx + 8]
000000014085ec57: movaps xmmword ptr [rbp - 0x40], xmm0
000000014085ec5b: lea rcx, [rbp - 0x20]
000000014085ec5f: lea rdx, [rbp - 0x40]
000000014085ec63: call 0x140b880d0 ; serde_json::value::de::MapDeserializer::new
000000014085ec68: mov byte ptr [rbp - 0x58], 0xb
000000014085ec6c: lea r8, [rip + 0x22369bd] ; 142a95630 000000000000000000000000000000000100000000000000b0d98540010000000a636173745f736c6963655f6d75742f686f6d652f68332f636f64652f706f756e63656c696768742f636f756e747279 b'\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\xb0\xd9\x85@\x01\x00\x00\x00\ncast_slice_mut/home/h3/code/pouncelight/country'
000000014085ec73: lea rcx, [rbp - 0x58]
000000014085ec77: lea rdx, [rbp + 0x5f]
000000014085ec7b: call 0x1428c9a00 ; serde_json::error::impl$8::invalid_type
000000014085ec80: mov qword ptr [rsi + 8], rax
000000014085ec84: mov byte ptr [rsi], 1
000000014085ec87: lea rcx, [rbp - 0x20]
000000014085ec8b: call 0x140887df0 ; core::ptr::drop_in_place<serde_json::map::IntoIter>
000000014085ec90: lea rcx, [rbp + 0x28]
000000014085ec94: call 0x14088e430 ; core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<serde_json::value::Value> > > >
000000014085ec99: jmp 0x14085ed22
000000014085ec9e: mov rax, qword ptr [rdx + 0x18]
000000014085eca2: mov qword ptr [rbp - 0x10], rax
000000014085eca6: movups xmm0, xmmword ptr [rdx + 8]
000000014085ecaa: movaps xmmword ptr [rbp - 0x20], xmm0
000000014085ecae: lea rdx, [rbp - 0x20]
000000014085ecb2: call 0x1415abad0 ; serde_json::value::de::visit_array<country_core::resources::walls::inner_wall_state::deserialize_wall_style::WallStyleVisitor>
000000014085ecb7: jmp 0x14085ed22
000000014085ecb9: mov edx, 0x6e6f7453
000000014085ecbe: xor edx, dword ptr [r10]
000000014085ecc1: movzx r8d, byte ptr [r10 + 4]
000000014085ecc6: xor r8d, 0x65
000000014085ecca: or r8d, edx
000000014085eccd: jne 0x14085ed97
000000014085ecd3: mov byte ptr [rcx + 1], 0
000000014085ecd7: jmp 0x14085ed08
000000014085ecd9: cmp rax, 0xc
000000014085ecdd: jne 0x14085ed97
000000014085ece3: movabs rdx, 0x626d6954666c6148
000000014085eced: xor rdx, qword ptr [r10]
000000014085ecf0: mov r8d, dword ptr [r10 + 8]
000000014085ecf4: xor r8, 0x64657265
000000014085ecfb: or r8, rdx
000000014085ecfe: jne 0x14085ed97
000000014085ed04: mov byte ptr [rcx + 1], 2
000000014085ed08: xor eax, eax
000000014085ed0a: mov byte ptr [rcx], al
000000014085ed0c: test r9, r9
000000014085ed0f: je 0x14085ed22
000000014085ed11: mov r8d, 1
000000014085ed17: mov rcx, r10
000000014085ed1a: mov rdx, r9
000000014085ed1d: call 0x140613c20 ; 140613c20
000000014085ed22: nop 
000000014085ed23: add rsp, 0xe8
000000014085ed2a: pop rsi
000000014085ed2b: pop rbp
000000014085ed2c: ret 
000000014085ed2d: mov byte ptr [rbp - 0x20], 7
000000014085ed31: jmp 0x14085ed5b
000000014085ed33: mov r8, qword ptr [rdx + 8]
000000014085ed37: mov rax, qword ptr [rdx + 0x10]
000000014085ed3b: test r8, r8
000000014085ed3e: je 0x14085ed53
000000014085ed40: mov rsi, rcx
000000014085ed43: cmp r8, 1
000000014085ed47: jne 0x14085ed72
000000014085ed49: mov qword ptr [rbp - 0x18], rax
000000014085ed4d: mov byte ptr [rbp - 0x20], 2
000000014085ed51: jmp 0x14085ed7a
000000014085ed53: mov qword ptr [rbp - 0x18], rax
000000014085ed57: mov byte ptr [rbp - 0x20], 1
000000014085ed5b: lea r8, [rip + 0x22368ce] ; 142a95630 000000000000000000000000000000000100000000000000b0d98540010000000a636173745f736c6963655f6d75742f686f6d652f68332f636f64652f706f756e63656c696768742f636f756e747279 b'\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\xb0\xd9\x85@\x01\x00\x00\x00\ncast_slice_mut/home/h3/code/pouncelight/country'
000000014085ed62: lea rax, [rbp - 0x20]
000000014085ed66: lea rdx, [rbp + 0x5f]
000000014085ed6a: mov rsi, rcx
000000014085ed6d: mov rcx, rax
000000014085ed70: jmp 0x14085ed89
000000014085ed72: mov qword ptr [rbp - 0x18], rax
000000014085ed76: mov byte ptr [rbp - 0x20], 3
000000014085ed7a: lea r8, [rip + 0x22368af] ; 142a95630 000000000000000000000000000000000100000000000000b0d98540010000000a636173745f736c6963655f6d75742f686f6d652f68332f636f64652f706f756e63656c696768742f636f756e747279 b'\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\xb0\xd9\x85@\x01\x00\x00\x00\ncast_slice_mut/home/h3/code/pouncelight/country'
000000014085ed81: lea rcx, [rbp - 0x20]
000000014085ed85: lea rdx, [rbp + 0x5f]
000000014085ed89: call 0x1428c9a00 ; serde_json::error::impl$8::invalid_type
000000014085ed8e: mov qword ptr [rsi + 8], rax
000000014085ed92: mov byte ptr [rsi], 1
000000014085ed95: jmp 0x14085ed22
000000014085ed97: mov qword ptr [rbp + 0x50], r9
000000014085ed9b: mov rsi, rcx
000000014085ed9e: mov qword ptr [rbp + 0x48], r10
000000014085eda2: mov qword ptr [rbp - 0x18], r10
000000014085eda6: mov qword ptr [rbp - 0x10], rax
000000014085edaa: mov byte ptr [rbp - 0x20], 5
000000014085edae: lea r8, [rip + 0x223687b] ; 142a95630 000000000000000000000000000000000100000000000000b0d98540010000000a636173745f736c6963655f6d75742f686f6d652f68332f636f64652f706f756e63656c696768742f636f756e747279 b'\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\xb0\xd9\x85@\x01\x00\x00\x00\ncast_slice_mut/home/h3/code/pouncelight/country'
000000014085edb5: lea rcx, [rbp - 0x20]
000000014085edb9: lea rdx, [rbp + 0x5f]
000000014085edbd: call 0x1428f9b40 ; serde::de::Error::invalid_value<serde_json::error::Error>
000000014085edc2: mov rcx, rsi
000000014085edc5: mov qword ptr [rsi + 8], rax
000000014085edc9: mov al, 1
000000014085edcb: mov r9, qword ptr [rbp + 0x50]
000000014085edcf: mov r10, qword ptr [rbp + 0x48]
000000014085edd3: mov byte ptr [rcx], al
000000014085edd5: test r9, r9
000000014085edd8: jne 0x14085ed11
000000014085edde: jmp 0x14085ed22
000000014085ede3: nop word ptr cs:[rax + rax]
000000014085edf0: mov qword ptr [rsp + 0x10], rdx
000000014085edf5: push rbp
000000014085edf6: push rsi
000000014085edf7: sub rsp, 0x28
000000014085edfb: lea rbp, [rdx + 0x80]
000000014085ee02: cmp qword ptr [rbp + 0x50], 0
000000014085ee07: je 0x14085ee1c
000000014085ee09: mov r8d, 1
000000014085ee0f: mov rcx, qword ptr [rbp + 0x48]
000000014085ee13: mov rdx, qword ptr [rbp + 0x50]
000000014085ee17: call 0x140613c20 ; 140613c20
000000014085ee1c: nop 
000000014085ee1d: add rsp, 0x28
000000014085ee21: pop rsi
000000014085ee22: pop rbp
000000014085ee23: ret 
000000014085ee24: nop word ptr cs:[rax + rax]
000000014085ee30: mov qword ptr [rsp + 0x10], rdx
000000014085ee35: push rbp
000000014085ee36: push rsi
000000014085ee37: sub rsp, 0x28
000000014085ee3b: lea rbp, [rdx + 0x80]
000000014085ee42: lea rcx, [rbp - 0x20]
000000014085ee46: call 0x14088a500 ; core::ptr::drop_in_place<serde_json::value::de::MapDeserializer>
000000014085ee4b: nop 
000000014085ee4c: add rsp, 0x28
000000014085ee50: pop rsi
000000014085ee51: pop rbp
000000014085ee52: ret 
000000014085ee53: nop word ptr cs:[rax + rax]
000000014085ee60: mov qword ptr [rsp + 0x10], rdx
000000014085ee65: push rbp
000000014085ee66: push rsi
000000014085ee67: sub rsp, 0x28
000000014085ee6b: lea rbp, [rdx + 0x80]
000000014085ee72: lea rcx, [rbp + 0x28]
000000014085ee76: call 0x14088e430 ; core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<serde_json::value::Value> > > >
000000014085ee7b: nop 
000000014085ee7c: add rsp, 0x28
000000014085ee80: pop rsi
000000014085ee81: pop rbp
000000014085ee82: ret 
