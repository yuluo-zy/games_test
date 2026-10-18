140b1aca0: push rbp
140b1aca1: push r14
140b1aca3: push rsi
140b1aca4: push rdi
140b1aca5: push rbx
140b1aca6: sub rsp, 0x160
140b1acad: lea rbp, [rsp + 0x80]
140b1acb5: mov qword ptr [rbp + 0xd8], 0xfffffffffffffffe
140b1acc0: mov rsi, r8
140b1acc3: mov rdi, rdx
140b1acc6: mov rdx, rcx
140b1acc9: mov byte ptr [rbp + 0xd7], 1
140b1acd0: mov rcx, r9
140b1acd3: mov qword ptr [rbp + 0xc8], rdx
140b1acda: call 0x1408ba6e0 ; country_core::asset_libraries::meshwatch::MeshWatch::watch
140b1acdf: mov rcx, qword ptr [rbp + 0xc8]
140b1ace6: mov rax, qword ptr [rcx + 0x70]
140b1acea: mov qword ptr [rbp + 0xa0], rax
140b1acf1: movups xmm0, xmmword ptr [rcx + 0x60]
140b1acf5: movaps xmmword ptr [rbp + 0x90], xmm0
140b1acfc: movups xmm0, xmmword ptr [rcx + 0x50]
140b1ad00: movaps xmmword ptr [rbp + 0x80], xmm0
140b1ad07: movups xmm0, xmmword ptr [rcx + 0x40]
140b1ad0b: movaps xmmword ptr [rbp + 0x70], xmm0
140b1ad0f: movups xmm0, xmmword ptr [rcx]
140b1ad12: movups xmm1, xmmword ptr [rcx + 0x10]
140b1ad16: movups xmm2, xmmword ptr [rcx + 0x20]
140b1ad1a: movups xmm3, xmmword ptr [rcx + 0x30]
140b1ad1e: movaps xmmword ptr [rbp + 0x60], xmm3
140b1ad22: movaps xmmword ptr [rbp + 0x50], xmm2
140b1ad26: movaps xmmword ptr [rbp + 0x40], xmm1
140b1ad2a: movaps xmmword ptr [rbp + 0x30], xmm0
140b1ad2e: movabs rax, 0x8000000000000000
140b1ad38: mov qword ptr [rbp + 0xa8], rax
140b1ad3f: test rsi, rsi
140b1ad42: js 0x140b1add4
140b1ad48: je 0x140b1ad6e
140b1ad4a: movzx eax, byte ptr [rip + 0x2b19eb8]
140b1ad51: mov r14d, 1
140b1ad57: mov edx, 1
140b1ad5c: mov rcx, rsi
140b1ad5f: call 0x140613c10
140b1ad64: test rax, rax
140b1ad67: je 0x140b1add7
140b1ad69: mov rbx, rax
140b1ad6c: jmp 0x140b1ad73
140b1ad6e: mov ebx, 1
140b1ad73: mov r14, qword ptr [rbp + 0x130]
140b1ad7a: mov rcx, rbx
140b1ad7d: mov rdx, rdi
140b1ad80: mov r8, rsi
140b1ad83: call 0x142923c00
140b1ad88: mov qword ptr [rbp + 0xa8], rsi
140b1ad8f: mov qword ptr [rbp + 0xb0], rbx
140b1ad96: mov qword ptr [rbp + 0xb8], rsi
140b1ad9d: lea rsi, [rbp - 0x60]
140b1ada1: lea rdx, [rbp + 0x30]
140b1ada5: mov r8d, 0x90
140b1adab: mov rcx, rsi
140b1adae: call 0x142923c00
140b1adb3: mov byte ptr [rbp + 0xd7], 0
140b1adba: mov rcx, r14
140b1adbd: mov rdx, rsi
140b1adc0: call 0x14092cc60 ; country_core::asset_libraries::mesh_library::AssetMeshLibrary::add
140b1adc5: nop 
140b1adc6: add rsp, 0x160
140b1adcd: pop rbx
140b1adce: pop rdi
140b1adcf: pop rsi
140b1add0: pop r14
140b1add2: pop rbp
140b1add3: ret 
140b1add4: xor r14d, r14d
140b1add7: lea r8, [rip + 0x21141da]
140b1adde: mov rcx, r14
140b1ade1: mov rdx, rsi
140b1ade4: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140b1ade9: ud2 
140b1adeb: nop dword ptr [rax + rax]
140b1adf0: mov qword ptr [rsp + 0x10], rdx
140b1adf5: push rbp
140b1adf6: push r14
140b1adf8: push rsi
140b1adf9: push rdi
140b1adfa: push rbx
140b1adfb: sub rsp, 0x20
140b1adff: lea rbp, [rdx + 0x80]
140b1ae06: cmp byte ptr [rbp + 0xd7], 0
140b1ae0d: jne 0x140b1ae1a
140b1ae0f: add rsp, 0x20
140b1ae13: pop rbx
140b1ae14: pop rdi
140b1ae15: pop rsi
140b1ae16: pop r14
140b1ae18: pop rbp
140b1ae19: ret 
140b1ae1a: mov rcx, qword ptr [rbp + 0xc8]
140b1ae21: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
140b1ae26: jmp 0x140b1ae0f
140b1ae28: nop dword ptr [rax + rax]
140b1ae30: mov qword ptr [rsp + 0x10], rdx
140b1ae35: push rbp
140b1ae36: push r14
140b1ae38: push rsi
140b1ae39: push rdi
140b1ae3a: push rbx
140b1ae3b: sub rsp, 0x20
140b1ae3f: lea rbp, [rdx + 0x80]
140b1ae46: mov byte ptr [rbp + 0xd7], 0
140b1ae4d: lea rcx, [rbp + 0x30]
140b1ae51: call 0x140898d40 ; core::ptr::drop_in_place<country_core::asset_libraries::Asset<country_core::render::mesh::Mesh> >
140b1ae56: mov byte ptr [rbp + 0xd7], 0
140b1ae5d: add rsp, 0x20
140b1ae61: pop rbx
140b1ae62: pop rdi
140b1ae63: pop rsi
140b1ae64: pop r14
140b1ae66: pop rbp
140b1ae67: ret 
