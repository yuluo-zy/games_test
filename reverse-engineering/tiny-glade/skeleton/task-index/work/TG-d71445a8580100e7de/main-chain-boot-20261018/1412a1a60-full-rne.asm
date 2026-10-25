1412a1a60: push rbp
1412a1a61: push r15
1412a1a63: push r14
1412a1a65: push r13
1412a1a67: push r12
1412a1a69: push rsi
1412a1a6a: push rdi
1412a1a6b: push rbx
1412a1a6c: sub rsp, 0xe8
1412a1a73: lea rbp, [rsp + 0x80]
1412a1a7b: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412a1a83: mov rsi, r8
1412a1a86: mov r14, rdx
1412a1a89: mov rbx, rcx
1412a1a8c: mov rdi, qword ptr [rcx]
1412a1a8f: mov r12, qword ptr [rdi + 0x30]
1412a1a93: lea r15, [r12 + 0x10]
1412a1a98: mov cl, 1
1412a1a9a: xor eax, eax
1412a1a9c: lock cmpxchg byte ptr [r12 + 0x10], cl
1412a1aa3: jne 0x1412a1c5c
1412a1aa9: mov r13, qword ptr [rip + 0x238a970]
1412a1ab0: mov rax, qword ptr [r13]
1412a1ab4: shl rax, 1
1412a1ab7: test rax, rax
1412a1aba: jne 0x1412a1c69
1412a1ac0: xor eax, eax
1412a1ac2: movzx ecx, byte ptr [r12 + 0x11]
1412a1ac8: test cl, cl
1412a1aca: jne 0x1412a1c7e
1412a1ad0: cmp qword ptr [r12 + 0x30], 0
1412a1ad6: je 0x1412a1bf6
1412a1adc: test al, al
1412a1ade: jne 0x1412a1af0
1412a1ae0: mov rax, qword ptr [r13]
1412a1ae4: shl rax, 1
1412a1ae7: test rax, rax
1412a1aea: jne 0x1412a1cd3
1412a1af0: xor eax, eax
1412a1af2: xchg byte ptr [r15], al
1412a1af5: cmp al, 2
1412a1af7: je 0x1412a1caf
1412a1afd: xor ecx, ecx
1412a1aff: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
1412a1b04: test rax, rax
1412a1b07: je 0x1412a1cc7
1412a1b0d: movups xmm0, xmmword ptr [rax]
1412a1b10: inc qword ptr [rax]
1412a1b13: movaps xmmword ptr [rbp + 0x40], xmm0
1412a1b17: movups xmm0, xmmword ptr [rip + 0x19ca3e2]
1412a1b1e: movaps xmmword ptr [rbp + 0x20], xmm0
1412a1b22: movups xmm0, xmmword ptr [rip + 0x19ca3e7]
1412a1b29: movaps xmmword ptr [rbp + 0x30], xmm0
1412a1b2d: mov eax, dword ptr [r14 + 0x1c]
1412a1b31: mov rcx, qword ptr [r14 + 0x10]
1412a1b35: mov dword ptr [rcx], eax
1412a1b37: mov rdx, qword ptr [r14]
1412a1b3a: mov eax, dword ptr [rbx + 0x1c]
1412a1b3d: mov rcx, qword ptr [rbx + 0x10]
1412a1b41: mov dword ptr [rcx], eax
1412a1b43: mov byte ptr [rbp + 0x5f], 1
1412a1b47: lea r9, [rbp + 0x20]
1412a1b4b: mov cl, 1
1412a1b4d: mov r8, rdi
1412a1b50: call 0x1412a1df0 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a1b55: cmp qword ptr [rbp + 0x38], 0
1412a1b5a: je 0x1412a1c2e
1412a1b60: movaps xmm0, xmmword ptr [rbp + 0x20]
1412a1b64: movaps xmm1, xmmword ptr [rbp + 0x30]
1412a1b68: movaps xmm2, xmmword ptr [rbp + 0x40]
1412a1b6c: movaps xmmword ptr [rbp - 0x30], xmm2
1412a1b70: movaps xmmword ptr [rbp - 0x40], xmm1
1412a1b74: movaps xmmword ptr [rbp - 0x50], xmm0
1412a1b78: mov eax, dword ptr [rsi + 0x1c]
1412a1b7b: mov rcx, qword ptr [rsi + 0x10]
1412a1b7f: mov dword ptr [rcx], eax
1412a1b81: mov rsi, qword ptr [rsi]
1412a1b84: mov rax, qword ptr [rsi + 0x40]
1412a1b88: mov qword ptr [rbp + 0x10], rax
1412a1b8c: movaps xmm0, xmmword ptr [rbp - 0x50]
1412a1b90: movaps xmm1, xmmword ptr [rbp - 0x40]
1412a1b94: movaps xmm2, xmmword ptr [rbp - 0x30]
1412a1b98: movaps xmmword ptr [rbp], xmm2
1412a1b9c: movaps xmmword ptr [rbp - 0x10], xmm1
1412a1ba0: movaps xmmword ptr [rbp - 0x20], xmm0
1412a1ba4: mov rdi, qword ptr [rsi + 0x30]
1412a1ba8: cmp rdi, qword ptr [rsi + 0x20]
1412a1bac: jne 0x1412a1bbe
1412a1bae: lea rcx, [rsi + 0x20]
1412a1bb2: lea rdx, [rip + 0x199858f]
1412a1bb9: call 0x1406319d0 ; alloc::raw_vec::RawVec::grow_one<utils::curve::Curve<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>,alloc::alloc::Global>
1412a1bbe: mov rax, qword ptr [rsi + 0x28]
1412a1bc2: imul rcx, rdi, 0x38
1412a1bc6: movaps xmm0, xmmword ptr [rbp - 0x20]
1412a1bca: movaps xmm1, xmmword ptr [rbp - 0x10]
1412a1bce: movaps xmm2, xmmword ptr [rbp]
1412a1bd2: mov rdx, qword ptr [rbp + 0x10]
1412a1bd6: mov qword ptr [rax + rcx + 0x30], rdx
1412a1bdb: movups xmmword ptr [rax + rcx + 0x20], xmm2
1412a1be0: movups xmmword ptr [rax + rcx + 0x10], xmm1
1412a1be5: movups xmmword ptr [rax + rcx], xmm0
1412a1be9: inc rdi
1412a1bec: mov qword ptr [rsi + 0x30], rdi
1412a1bf0: inc qword ptr [rsi + 0x40]
1412a1bf4: jmp 0x1412a1c47 ; country_core::startup::load_prefabs::reload_prefabs
1412a1bf6: test al, al
1412a1bf8: jne 0x1412a1c0a
1412a1bfa: mov rax, qword ptr [r13]
1412a1bfe: shl rax, 1
1412a1c01: test rax, rax
1412a1c04: jne 0x1412a1ceb
1412a1c0a: xor eax, eax
1412a1c0c: xchg byte ptr [r15], al
1412a1c0f: cmp al, 2
1412a1c11: jne 0x1412a1c47
1412a1c13: mov rcx, r15
1412a1c16: add rsp, 0xe8
1412a1c1d: pop rbx
1412a1c1e: pop rdi
1412a1c1f: pop rsi
1412a1c20: pop r12
1412a1c22: pop r13
1412a1c24: pop r14
1412a1c26: pop r15
1412a1c28: pop rbp
1412a1c29: jmp 0x1428d8810 ; std::sys::sync::mutex::futex::Mutex::wake
1412a1c2e: lea rcx, [rbp + 0x20]
1412a1c32: mov r8d, 0x10
1412a1c38: mov r9d, 0x10
1412a1c3e: lea rdx, [rbp + 0x40]
1412a1c42: call 0x140b73f00 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<asset::prefab::PrefabName,tuple$<> >,alloc::alloc::Global>
1412a1c47: nop 
1412a1c48: add rsp, 0xe8
1412a1c4f: pop rbx
1412a1c50: pop rdi
1412a1c51: pop rsi
1412a1c52: pop r12
1412a1c54: pop r13
1412a1c56: pop r14
1412a1c58: pop r15
1412a1c5a: pop rbp
1412a1c5b: ret 
1412a1c5c: mov rcx, r15
1412a1c5f: call 0x1428d8750 ; std::sys::sync::mutex::futex::Mutex::lock_contended
1412a1c64: jmp 0x1412a1aa9 ; country_core::startup::load_prefabs::reload_prefabs
1412a1c69: call 0x1428d86b0 ; std::panicking::panic_count::is_zero_slow_path
1412a1c6e: xor al, 1
1412a1c70: movzx ecx, byte ptr [r12 + 0x11]
1412a1c76: test cl, cl
1412a1c78: je 0x1412a1ad0
1412a1c7e: mov qword ptr [rbp - 0x20], r15
1412a1c82: mov byte ptr [rbp - 0x18], al
1412a1c85: lea rax, [rip + 0x19888ec]
1412a1c8c: mov qword ptr [rsp + 0x20], rax
1412a1c91: lea rcx, [rip + 0x1988380]
1412a1c98: lea r9, [rip + 0x19883c9]
1412a1c9f: lea r8, [rbp - 0x20]
1412a1ca3: mov edx, 0x2b
1412a1ca8: call 0x1428d9760 ; core::result::unwrap_failed
1412a1cad: ud2 
1412a1caf: mov rcx, r15
1412a1cb2: call 0x1428d8810 ; std::sys::sync::mutex::futex::Mutex::wake
1412a1cb7: xor ecx, ecx
1412a1cb9: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
1412a1cbe: test rax, rax
1412a1cc1: jne 0x1412a1b0d
1412a1cc7: lea rcx, [rip + 0x1998dd2]
1412a1cce: call 0x1428d80d0 ; std::thread::local::panic_access_error
1412a1cd3: call 0x1428d86b0 ; std::panicking::panic_count::is_zero_slow_path
1412a1cd8: test al, al
1412a1cda: jne 0x1412a1af0
1412a1ce0: mov byte ptr [r12 + 0x11], 1
1412a1ce6: jmp 0x1412a1af0 ; country_core::startup::load_prefabs::reload_prefabs
1412a1ceb: call 0x1428d86b0 ; std::panicking::panic_count::is_zero_slow_path
1412a1cf0: test al, al
1412a1cf2: jne 0x1412a1c0a
1412a1cf8: mov byte ptr [r12 + 0x11], 1
1412a1cfe: jmp 0x1412a1c0a ; country_core::startup::load_prefabs::reload_prefabs
1412a1d03: nop word ptr cs:[rax + rax]
1412a1d10: mov qword ptr [rsp + 0x10], rdx
1412a1d15: push rbp
1412a1d16: push r15
1412a1d18: push r14
1412a1d1a: push r13
1412a1d1c: push r12
1412a1d1e: push rsi
1412a1d1f: push rdi
1412a1d20: push rbx
1412a1d21: sub rsp, 0x28
1412a1d25: lea rbp, [rdx + 0x80]
1412a1d2c: cmp byte ptr [rbp + 0x5f], 0
1412a1d30: je 0x1412a1d4b
1412a1d32: lea rcx, [rbp + 0x20]
1412a1d36: mov r8d, 0x10
1412a1d3c: mov r9d, 0x10
1412a1d42: lea rdx, [rbp + 0x40]
1412a1d46: call 0x140b73f00 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<asset::prefab::PrefabName,tuple$<> >,alloc::alloc::Global>
1412a1d4b: nop 
1412a1d4c: add rsp, 0x28
1412a1d50: pop rbx
1412a1d51: pop rdi
1412a1d52: pop rsi
1412a1d53: pop r12
1412a1d55: pop r13
1412a1d57: pop r14
1412a1d59: pop r15
1412a1d5b: pop rbp
1412a1d5c: ret 
1412a1d5d: nop dword ptr [rax]
1412a1d60: mov qword ptr [rsp + 0x10], rdx
1412a1d65: push rbp
1412a1d66: push r15
1412a1d68: push r14
1412a1d6a: push r13
1412a1d6c: push r12
1412a1d6e: push rsi
1412a1d6f: push rdi
1412a1d70: push rbx
1412a1d71: sub rsp, 0x28
1412a1d75: lea rbp, [rdx + 0x80]
1412a1d7c: lea rcx, [rbp - 0x20]
1412a1d80: call 0x1405d2870 ; core::ptr::drop_in_place<std::sync::poison::mutex::MutexGuard<winit::platform_impl::platform::window_state::WindowState> >
1412a1d85: nop 
1412a1d86: add rsp, 0x28
1412a1d8a: pop rbx
1412a1d8b: pop rdi
1412a1d8c: pop rsi
1412a1d8d: pop r12
1412a1d8f: pop r13
1412a1d91: pop r14
1412a1d93: pop r15
1412a1d95: pop rbp
1412a1d96: ret 
1412a1d97: nop word ptr [rax + rax]
1412a1da0: mov qword ptr [rsp + 0x10], rdx
1412a1da5: push rbp
1412a1da6: push r15
1412a1da8: push r14
1412a1daa: push r13
1412a1dac: push r12
1412a1dae: push rsi
1412a1daf: push rdi
1412a1db0: push rbx
1412a1db1: sub rsp, 0x28
1412a1db5: lea rbp, [rdx + 0x80]
1412a1dbc: lea rdx, [rbp]
1412a1dc0: mov byte ptr [rbp + 0x5f], 0
1412a1dc4: lea rcx, [rbp - 0x20]
1412a1dc8: mov r8d, 0x10
1412a1dce: mov r9d, 0x10
1412a1dd4: call 0x140b73f00 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<asset::prefab::PrefabName,tuple$<> >,alloc::alloc::Global>
1412a1dd9: mov byte ptr [rbp + 0x5f], 0
1412a1ddd: add rsp, 0x28
1412a1de1: pop rbx
1412a1de2: pop rdi
1412a1de3: pop rsi
1412a1de4: pop r12
1412a1de6: pop r13
1412a1de8: pop r14
1412a1dea: pop r15
1412a1dec: pop rbp
1412a1ded: ret 
