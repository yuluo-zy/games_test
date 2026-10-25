14128ca70: 56 push rsi
14128ca71: 57 push rdi
14128ca72: 4883ec78 sub rsp, 0x78
14128ca76: 4889ce mov rsi, rcx
14128ca79: 48b875657370656d6f73 movabs rax, 0x736f6d6570736575
14128ca83: 4889442430 mov qword ptr [rsp + 0x30], rax
14128ca88: 48b86172656e6567796c movabs rax, 0x6c7967656e657261
14128ca92: 4889442438 mov qword ptr [rsp + 0x38], rax
14128ca97: 48b86d6f646e61726f64 movabs rax, 0x646f72616e646f6d
14128caa1: 4889442440 mov qword ptr [rsp + 0x40], rax
14128caa6: 48b87365747962646574 movabs rax, 0x7465646279746573
14128cab0: 4889442448 mov qword ptr [rsp + 0x48], rax
14128cab5: 0f57c0 xorps xmm0, xmm0
14128cab8: 0f11442450 movups xmmword ptr [rsp + 0x50], xmm0
14128cabd: 0f11442460 movups xmmword ptr [rsp + 0x60], xmm0
14128cac2: 48c744247000000000 mov qword ptr [rsp + 0x70], 0
14128cacb: 8b4110 mov eax, dword ptr [rcx + 0x10]
14128cace: 89442428 mov dword ptr [rsp + 0x28], eax
14128cad2: 488d7c2430 lea rdi, [rsp + 0x30]
14128cad7: 488d542428 lea rdx, [rsp + 0x28]
14128cadc: 41b804000000 mov r8d, 4
14128cae2: 4889f9 mov rcx, rdi
14128cae5: e87696e7fe call 0x140106160
14128caea: 488b16 mov rdx, qword ptr [rsi]
14128caed: 4c8b4608 mov r8, qword ptr [rsi + 8]
14128caf1: 4889f9 mov rcx, rdi
14128caf4: e86796e7fe call 0x140106160
14128caf9: c6442428ff mov byte ptr [rsp + 0x28], 0xff
14128cafe: 488d542428 lea rdx, [rsp + 0x28]
14128cb03: 41b801000000 mov r8d, 1
14128cb09: 4889f9 mov rcx, rdi
14128cb0c: e84f96e7fe call 0x140106160
14128cb11: 488b4618 mov rax, qword ptr [rsi + 0x18]
14128cb15: 4889442428 mov qword ptr [rsp + 0x28], rax
14128cb1a: 488d542428 lea rdx, [rsp + 0x28]
14128cb1f: 41b808000000 mov r8d, 8
14128cb25: 4889f9 mov rcx, rdi
14128cb28: e83396e7fe call 0x140106160
14128cb2d: 4c8b4c2460 mov r9, qword ptr [rsp + 0x60]
14128cb32: 49c1e138 shl r9, 0x38
14128cb36: 4c0b4c2468 or r9, qword ptr [rsp + 0x68]
14128cb3b: 488b542440 mov rdx, qword ptr [rsp + 0x40]
14128cb40: 488b4c2448 mov rcx, qword ptr [rsp + 0x48]
14128cb45: 4c31c9 xor rcx, r9
14128cb48: 488b442430 mov rax, qword ptr [rsp + 0x30]
14128cb4d: 4801d0 add rax, rdx
14128cb50: 4c8b442438 mov r8, qword ptr [rsp + 0x38]
14128cb55: 48c1c20d rol rdx, 0xd
14128cb59: 4901c8 add r8, rcx
14128cb5c: 4831c2 xor rdx, rax
14128cb5f: 48c1c110 rol rcx, 0x10
14128cb63: 4c31c1 xor rcx, r8
14128cb66: 48c1c020 rol rax, 0x20
14128cb6a: 4901d0 add r8, rdx
14128cb6d: 4801c8 add rax, rcx
14128cb70: 48c1c211 rol rdx, 0x11
14128cb74: 4c31c2 xor rdx, r8
14128cb77: 48c1c115 rol rcx, 0x15
14128cb7b: 49c1c020 rol r8, 0x20
14128cb7f: 4831c1 xor rcx, rax
14128cb82: 4c31c8 xor rax, r9
14128cb85: 4981f0ff000000 xor r8, 0xff
14128cb8c: 4801d0 add rax, rdx
14128cb8f: 4901c8 add r8, rcx
14128cb92: 48c1c20d rol rdx, 0xd
14128cb96: 4831c2 xor rdx, rax
14128cb99: 48c1c110 rol rcx, 0x10
14128cb9d: 48c1c020 rol rax, 0x20
14128cba1: 4c31c1 xor rcx, r8
14128cba4: 4901d0 add r8, rdx
14128cba7: 48c1c211 rol rdx, 0x11
14128cbab: 4801c8 add rax, rcx
14128cbae: 4c31c2 xor rdx, r8
14128cbb1: 48c1c115 rol rcx, 0x15
14128cbb5: 4831c1 xor rcx, rax
14128cbb8: 49c1c020 rol r8, 0x20
14128cbbc: 4801d0 add rax, rdx
14128cbbf: 4901c8 add r8, rcx
14128cbc2: 48c1c20d rol rdx, 0xd
14128cbc6: 4831c2 xor rdx, rax
14128cbc9: 48c1c110 rol rcx, 0x10
14128cbcd: 48c1c020 rol rax, 0x20
14128cbd1: 4c31c1 xor rcx, r8
14128cbd4: 4901d0 add r8, rdx
14128cbd7: 48c1c211 rol rdx, 0x11
14128cbdb: 4801c8 add rax, rcx
14128cbde: 4c31c2 xor rdx, r8
14128cbe1: 48c1c115 rol rcx, 0x15
14128cbe5: 4831c1 xor rcx, rax
14128cbe8: 49c1c020 rol r8, 0x20
14128cbec: 4801d0 add rax, rdx
14128cbef: 4901c8 add r8, rcx
14128cbf2: 48c1c20d rol rdx, 0xd
14128cbf6: 4831c2 xor rdx, rax
14128cbf9: 48c1c110 rol rcx, 0x10
14128cbfd: 4c31c1 xor rcx, r8
14128cc00: 4901d0 add r8, rdx
14128cc03: 48c1c211 rol rdx, 0x11
14128cc07: 48c1c115 rol rcx, 0x15
14128cc0b: 4c89c0 mov rax, r8
14128cc0e: 48c1c020 rol rax, 0x20
14128cc12: 4831d0 xor rax, rdx
14128cc15: 4831c8 xor rax, rcx
14128cc18: 4c31c0 xor rax, r8
14128cc1b: 4883c478 add rsp, 0x78
14128cc1f: 5f pop rdi
14128cc20: 5e pop rsi
14128cc21: c3 ret 
