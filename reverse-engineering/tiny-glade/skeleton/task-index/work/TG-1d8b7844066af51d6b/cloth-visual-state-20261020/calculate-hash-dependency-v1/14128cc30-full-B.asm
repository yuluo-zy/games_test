14128cc30: 56 push rsi
14128cc31: 57 push rdi
14128cc32: 4883ec78 sub rsp, 0x78
14128cc36: 4889ce mov rsi, rcx
14128cc39: 48b875657370656d6f73 movabs rax, 0x736f6d6570736575
14128cc43: 4889442430 mov qword ptr [rsp + 0x30], rax
14128cc48: 48b86172656e6567796c movabs rax, 0x6c7967656e657261
14128cc52: 4889442438 mov qword ptr [rsp + 0x38], rax
14128cc57: 48b86d6f646e61726f64 movabs rax, 0x646f72616e646f6d
14128cc61: 4889442440 mov qword ptr [rsp + 0x40], rax
14128cc66: 48b87365747962646574 movabs rax, 0x7465646279746573
14128cc70: 4889442448 mov qword ptr [rsp + 0x48], rax
14128cc75: 0f57c0 xorps xmm0, xmm0
14128cc78: 0f11442450 movups xmmword ptr [rsp + 0x50], xmm0
14128cc7d: 0f11442460 movups xmmword ptr [rsp + 0x60], xmm0
14128cc82: 48c744247000000000 mov qword ptr [rsp + 0x70], 0
14128cc8b: 8b01 mov eax, dword ptr [rcx]
14128cc8d: 8944242c mov dword ptr [rsp + 0x2c], eax
14128cc91: 488d7c2430 lea rdi, [rsp + 0x30]
14128cc96: 488d54242c lea rdx, [rsp + 0x2c]
14128cc9b: 41b804000000 mov r8d, 4
14128cca1: 4889f9 mov rcx, rdi
14128cca4: e8b794e7fe call 0x140106160
14128cca9: 488b5608 mov rdx, qword ptr [rsi + 8]
14128ccad: 4c8b4610 mov r8, qword ptr [rsi + 0x10]
14128ccb1: 4889f9 mov rcx, rdi
14128ccb4: e8a794e7fe call 0x140106160
14128ccb9: c644242cff mov byte ptr [rsp + 0x2c], 0xff
14128ccbe: 488d54242c lea rdx, [rsp + 0x2c]
14128ccc3: 41b801000000 mov r8d, 1
14128ccc9: 4889f9 mov rcx, rdi
14128cccc: e88f94e7fe call 0x140106160
14128ccd1: 4c8b4c2460 mov r9, qword ptr [rsp + 0x60]
14128ccd6: 49c1e138 shl r9, 0x38
14128ccda: 4c0b4c2468 or r9, qword ptr [rsp + 0x68]
14128ccdf: 488b542440 mov rdx, qword ptr [rsp + 0x40]
14128cce4: 488b4c2448 mov rcx, qword ptr [rsp + 0x48]
14128cce9: 4c31c9 xor rcx, r9
14128ccec: 488b442430 mov rax, qword ptr [rsp + 0x30]
14128ccf1: 4801d0 add rax, rdx
14128ccf4: 4c8b442438 mov r8, qword ptr [rsp + 0x38]
14128ccf9: 48c1c20d rol rdx, 0xd
14128ccfd: 4901c8 add r8, rcx
14128cd00: 4831c2 xor rdx, rax
14128cd03: 48c1c110 rol rcx, 0x10
14128cd07: 4c31c1 xor rcx, r8
14128cd0a: 48c1c020 rol rax, 0x20
14128cd0e: 4901d0 add r8, rdx
14128cd11: 4801c8 add rax, rcx
14128cd14: 48c1c211 rol rdx, 0x11
14128cd18: 4c31c2 xor rdx, r8
14128cd1b: 48c1c115 rol rcx, 0x15
14128cd1f: 49c1c020 rol r8, 0x20
14128cd23: 4831c1 xor rcx, rax
14128cd26: 4c31c8 xor rax, r9
14128cd29: 4981f0ff000000 xor r8, 0xff
14128cd30: 4801d0 add rax, rdx
14128cd33: 4901c8 add r8, rcx
14128cd36: 48c1c20d rol rdx, 0xd
14128cd3a: 4831c2 xor rdx, rax
14128cd3d: 48c1c110 rol rcx, 0x10
14128cd41: 48c1c020 rol rax, 0x20
14128cd45: 4c31c1 xor rcx, r8
14128cd48: 4901d0 add r8, rdx
14128cd4b: 48c1c211 rol rdx, 0x11
14128cd4f: 4801c8 add rax, rcx
14128cd52: 4c31c2 xor rdx, r8
14128cd55: 48c1c115 rol rcx, 0x15
14128cd59: 4831c1 xor rcx, rax
14128cd5c: 49c1c020 rol r8, 0x20
14128cd60: 4801d0 add rax, rdx
14128cd63: 4901c8 add r8, rcx
14128cd66: 48c1c20d rol rdx, 0xd
14128cd6a: 4831c2 xor rdx, rax
14128cd6d: 48c1c110 rol rcx, 0x10
14128cd71: 48c1c020 rol rax, 0x20
14128cd75: 4c31c1 xor rcx, r8
14128cd78: 4901d0 add r8, rdx
14128cd7b: 48c1c211 rol rdx, 0x11
14128cd7f: 4801c8 add rax, rcx
14128cd82: 4c31c2 xor rdx, r8
14128cd85: 48c1c115 rol rcx, 0x15
14128cd89: 4831c1 xor rcx, rax
14128cd8c: 49c1c020 rol r8, 0x20
14128cd90: 4801d0 add rax, rdx
14128cd93: 4901c8 add r8, rcx
14128cd96: 48c1c20d rol rdx, 0xd
14128cd9a: 4831c2 xor rdx, rax
14128cd9d: 48c1c110 rol rcx, 0x10
14128cda1: 4c31c1 xor rcx, r8
14128cda4: 4901d0 add r8, rdx
14128cda7: 48c1c211 rol rdx, 0x11
14128cdab: 48c1c115 rol rcx, 0x15
14128cdaf: 4c89c0 mov rax, r8
14128cdb2: 48c1c020 rol rax, 0x20
14128cdb6: 4831d0 xor rax, rdx
14128cdb9: 4831c8 xor rax, rcx
14128cdbc: 4c31c0 xor rax, r8
14128cdbf: 4883c478 add rsp, 0x78
14128cdc3: 5f pop rdi
14128cdc4: 5e pop rsi
14128cdc5: c3 ret 
