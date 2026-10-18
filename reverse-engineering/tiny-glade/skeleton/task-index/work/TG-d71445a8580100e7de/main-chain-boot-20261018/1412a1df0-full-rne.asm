1412a1df0: push rbp
1412a1df1: push r15
1412a1df3: push r14
1412a1df5: push r13
1412a1df7: push r12
1412a1df9: push rsi
1412a1dfa: push rdi
1412a1dfb: push rbx
1412a1dfc: sub rsp, 0x5f8
1412a1e03: lea rbp, [rsp + 0x80]
1412a1e0b: movdqa xmmword ptr [rbp + 0x560], xmm13
1412a1e14: movdqa xmmword ptr [rbp + 0x550], xmm12
1412a1e1d: movaps xmmword ptr [rbp + 0x540], xmm11
1412a1e25: movaps xmmword ptr [rbp + 0x530], xmm10
1412a1e2d: movdqa xmmword ptr [rbp + 0x520], xmm9
1412a1e36: movaps xmmword ptr [rbp + 0x510], xmm8
1412a1e3e: movaps xmmword ptr [rbp + 0x500], xmm7
1412a1e45: movaps xmmword ptr [rbp + 0x4f0], xmm6
1412a1e4c: mov qword ptr [rbp + 0x4e8], 0xfffffffffffffffe
1412a1e57: mov qword ptr [rbp + 0x300], r9
1412a1e5e: mov r14, r8
1412a1e61: mov r15, rdx
1412a1e64: mov bl, cl
1412a1e66: mov byte ptr [rbp + 0x42f], cl
1412a1e6c: lea rdx, [rip + 0x198871d]
1412a1e73: lea rcx, [rbp + 0x20]
1412a1e77: mov r8d, 0xe
1412a1e7d: call 0x140d61b00 ; std::sys::os_str::wtf8::Slice::to_owned
1412a1e82: xor ecx, ecx
1412a1e84: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
1412a1e89: test rax, rax
1412a1e8c: je 0x1412a5c13
1412a1e92: movdqu xmm0, xmmword ptr [rax]
1412a1e96: inc qword ptr [rax]
1412a1e99: movdqa xmmword ptr [rbp + 0x60], xmm0
1412a1e9e: movups xmm6, xmmword ptr [rip + 0x19ca05b]
1412a1ea5: movaps xmmword ptr [rbp + 0x40], xmm6
1412a1ea9: movups xmm7, xmmword ptr [rip + 0x19ca060]
1412a1eb0: movaps xmmword ptr [rbp + 0x50], xmm7
1412a1eb4: test bl, bl
1412a1eb6: je 0x1412a1f93
1412a1ebc: test r14, r14
1412a1ebf: je 0x1412a1fbe
1412a1ec5: mov rsi, qword ptr [r14 + 0x30]
1412a1ec9: lea rbx, [rsi + 0x10]
1412a1ecd: mov cl, 1
1412a1ecf: xor eax, eax
1412a1ed1: lock cmpxchg byte ptr [rsi + 0x10], cl
1412a1ed6: jne 0x1412a5486
1412a1edc: mov rdi, qword ptr [rip + 0x238a53d]
1412a1ee3: mov rax, qword ptr [rdi]
1412a1ee6: shl rax, 1
1412a1ee9: test rax, rax
1412a1eec: jne 0x1412a549a
1412a1ef2: mov dword ptr [rbp + 0x4c8], 0
1412a1efc: mov qword ptr [rbp + 0x3a0], rbx
1412a1f03: movzx eax, byte ptr [rsi + 0x11]
1412a1f07: test al, al
1412a1f09: jne 0x1412a54c1
1412a1f0f: xor ecx, ecx
1412a1f11: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
1412a1f16: test rax, rax
1412a1f19: je 0x1412a640d
1412a1f1f: movdqu xmm0, xmmword ptr [rax]
1412a1f23: inc qword ptr [rax]
1412a1f26: movdqu xmm1, xmmword ptr [rsi + 0x18]
1412a1f2b: movdqu xmm2, xmmword ptr [rsi + 0x28]
1412a1f30: movups xmm3, xmmword ptr [rsi + 0x38]
1412a1f34: movaps xmmword ptr [rbp + 0x150], xmm3
1412a1f3b: movdqa xmmword ptr [rbp + 0x140], xmm2
1412a1f43: movdqa xmmword ptr [rbp + 0x130], xmm1
1412a1f4b: movups xmmword ptr [rsi + 0x18], xmm6
1412a1f4f: movups xmmword ptr [rsi + 0x28], xmm7
1412a1f53: movdqu xmmword ptr [rsi + 0x38], xmm0
1412a1f58: cmp byte ptr [rbp + 0x4c8], 0
1412a1f5f: mov rbx, qword ptr [rbp + 0x3a0]
1412a1f66: jne 0x1412a1f77
1412a1f68: mov rax, qword ptr [rdi]
1412a1f6b: shl rax, 1
1412a1f6e: test rax, rax
1412a1f71: jne 0x1412a550b
1412a1f77: xor eax, eax
1412a1f79: xchg byte ptr [rbx], al
1412a1f7b: mov sil, 1
1412a1f7e: cmp al, 2
1412a1f80: jne 0x1412a1ff5
1412a1f82: mov byte ptr [rbp + 0x4e6], 1
1412a1f89: mov rcx, rbx
1412a1f8c: call 0x1428d8810 ; std::sys::sync::mutex::futex::Mutex::wake
1412a1f91: jmp 0x1412a1ff5 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a1f93: movdqa xmm0, xmmword ptr [rbp + 0x40]
1412a1f98: movdqa xmm1, xmmword ptr [rbp + 0x50]
1412a1f9d: movdqa xmm2, xmmword ptr [rbp + 0x60]
1412a1fa2: movdqa xmmword ptr [rbp + 0x150], xmm2
1412a1faa: movdqa xmmword ptr [rbp + 0x140], xmm1
1412a1fb2: movdqa xmmword ptr [rbp + 0x130], xmm0
1412a1fba: xor esi, esi
1412a1fbc: jmp 0x1412a1ff5 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a1fbe: mov byte ptr [rbp + 0x4e6], 1
1412a1fc5: xor ecx, ecx
1412a1fc7: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
1412a1fcc: test rax, rax
1412a1fcf: je 0x1412a67ac
1412a1fd5: movdqu xmm0, xmmword ptr [rax]
1412a1fd9: inc qword ptr [rax]
1412a1fdc: movdqa xmmword ptr [rbp + 0x150], xmm0
1412a1fe4: movaps xmmword ptr [rbp + 0x130], xmm6
1412a1feb: movaps xmmword ptr [rbp + 0x140], xmm7
1412a1ff2: mov sil, 1
1412a1ff5: mov qword ptr [rbp + 0x210], r14
1412a1ffc: mov dword ptr [rbp + 0x2f4], esi
1412a2002: lea rax, [rbp + 0x42f]
1412a2009: mov qword ptr [rbp + 8], rax
1412a200d: mov qword ptr [rbp + 0x10], r15
1412a2011: lea rax, [rbp + 0x130]
1412a2018: mov qword ptr [rbp + 0x18], rax
1412a201c: lea rcx, [rbp + 0x20]
1412a2020: call 0x140d76ed0 ; std::sys::os_str::wtf8::Buf::as_mut_slice
1412a2025: mov r8, rdx
1412a2028: lea rcx, [rbp + 0x430]
1412a202f: mov rdx, rax
1412a2032: call 0x14200a1b0 ; asset::prefab::scan::scan_asset_tree
1412a2037: cmp qword ptr [rbp + 0x430], 0
1412a203f: je 0x1412a5c24
1412a2045: movups xmm0, xmmword ptr [rbp + 0x480]
1412a204c: movaps xmmword ptr [rbp + 0x120], xmm0
1412a2053: movups xmm0, xmmword ptr [rbp + 0x470]
1412a205a: movaps xmmword ptr [rbp + 0x110], xmm0
1412a2061: movdqu xmm0, xmmword ptr [rbp + 0x430]
1412a2069: movdqu xmm1, xmmword ptr [rbp + 0x440]
1412a2071: movdqu xmm2, xmmword ptr [rbp + 0x450]
1412a2079: movups xmm3, xmmword ptr [rbp + 0x460]
1412a2080: movaps xmmword ptr [rbp + 0x100], xmm3
1412a2087: movdqa xmmword ptr [rbp + 0xf0], xmm2
1412a208f: movdqa xmmword ptr [rbp + 0xe0], xmm1
1412a2097: movdqa xmmword ptr [rbp + 0xd0], xmm0
1412a209f: mov rsi, qword ptr [rbp + 0xe8]
1412a20a6: xor ebx, ebx
1412a20a8: mov ecx, 0x28
1412a20ad: mov rax, rsi
1412a20b0: mul rcx
1412a20b3: mov rdi, rax
1412a20b6: jo 0x1412a20ea
1412a20b8: movabs rax, 0x7ffffffffffffff8
1412a20c2: cmp rdi, rax
1412a20c5: ja 0x1412a20ea
1412a20c7: test rdi, rdi
1412a20ca: je 0x1412a2108
1412a20cc: movzx eax, byte ptr [rip + 0x2392b36]
1412a20d3: mov ebx, 8
1412a20d8: mov edx, 8
1412a20dd: mov rcx, rdi
1412a20e0: call 0x140613c10
1412a20e5: test rax, rax
1412a20e8: jne 0x1412a210f
1412a20ea: mov byte ptr [rbp + 0x4d8], 1
1412a20f1: lea r8, [rip + 0x19884c0]
1412a20f8: mov rcx, rbx
1412a20fb: mov rdx, rdi
1412a20fe: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1412a2103: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a2108: mov eax, 8
1412a210d: xor esi, esi
1412a210f: mov qword ptr [rbp + 0x248], rsi
1412a2116: mov qword ptr [rbp + 0x250], rax
1412a211d: mov qword ptr [rbp + 0x258], 0
1412a2128: call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1412a212d: mov rsi, rax
1412a2130: test byte ptr [rax], 1
1412a2133: mov qword ptr [rbp + 0x268], r15
1412a213a: je 0x1412a214d
1412a213c: mov rcx, rsi
1412a213f: add rcx, 8
1412a2143: mov rax, qword ptr [rsi + 8]
1412a2147: mov rdx, qword ptr [rsi + 0x10]
1412a214b: jmp 0x1412a2176 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a214d: mov byte ptr [rbp + 0x4dc], 1
1412a2154: mov byte ptr [rbp + 0x4db], 1
1412a215b: call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1412a2160: mov rcx, rsi
1412a2163: add rcx, 8
1412a2167: mov qword ptr [rsi], 1
1412a216e: mov qword ptr [rsi + 8], rax
1412a2172: mov qword ptr [rsi + 0x10], rdx
1412a2176: lea r8, [rax + 1]
1412a217a: mov qword ptr [rcx], r8
1412a217d: movups xmm6, xmmword ptr [rip + 0x1987f7c]
1412a2184: movaps xmmword ptr [rbp + 0x170], xmm6
1412a218b: movups xmm7, xmmword ptr [rip + 0x1987f5e]
1412a2192: movaps xmmword ptr [rbp + 0x160], xmm7
1412a2199: mov qword ptr [rbp + 0x180], rax
1412a21a0: mov qword ptr [rbp + 0x188], rdx
1412a21a7: mov rax, qword ptr [rbp + 0xd0]
1412a21ae: mov rcx, qword ptr [rbp + 0xd8]
1412a21b5: lea rcx, [rax + rcx + 1]
1412a21ba: movdqa xmm0, xmmword ptr [rax]
1412a21be: pmovmskb edx, xmm0
1412a21c2: not edx
1412a21c4: mov r8, qword ptr [rbp + 0xe8]
1412a21cb: mov qword ptr [rbp + 0x1e0], rax
1412a21d2: add rax, 0x10
1412a21d6: mov qword ptr [rbp + 0x1e8], rax
1412a21dd: mov qword ptr [rbp + 0x1f0], rcx
1412a21e4: mov word ptr [rbp + 0x1f8], dx
1412a21eb: mov qword ptr [rbp + 0x200], r8
1412a21f2: lea rdi, [rbp + 0x208]
1412a21f9: lea rax, [rbp + 8]
1412a21fd: mov qword ptr [rbp + 0x208], rax
1412a2204: lea rbx, [rbp + 0x2d8]
1412a220b: movaps xmm8, xmmword ptr [rip + 0x168371d]
1412a2213: pcmpeqd xmm9, xmm9
1412a2218: xorps xmm10, xmm10
1412a221c: movss xmm11, dword ptr [rip + 0x16836df]
1412a2225: mov qword ptr [rbp + 0x3b0], rdi
1412a222c: nop dword ptr [rax]
1412a2230: mov rax, qword ptr [rbp + 0x200]
1412a2237: test rax, rax
1412a223a: je 0x1412a4e16
1412a2240: movzx ecx, word ptr [rbp + 0x1f8]
1412a2247: mov r15, qword ptr [rbp + 0x1e0]
1412a224e: test cx, cx
1412a2251: je 0x1412a2280
1412a2253: lea edx, [rcx - 1]
1412a2256: and edx, ecx
1412a2258: mov word ptr [rbp + 0x1f8], dx
1412a225f: dec rax
1412a2262: mov qword ptr [rbp + 0x200], rax
1412a2269: test r15, r15
1412a226c: jne 0x1412a22da
1412a226e: jmp 0x1412a4e16 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a2273: nop word ptr cs:[rax + rax]
1412a2280: mov rdx, qword ptr [rbp + 0x1e8]
1412a2287: nop word ptr [rax + rax]
1412a2290: movdqa xmm0, xmmword ptr [rdx]
1412a2294: pmovmskb ecx, xmm0
1412a2298: add r15, -0x400
1412a229f: add rdx, 0x10
1412a22a3: cmp ecx, 0xffff
1412a22a9: je 0x1412a2290
1412a22ab: mov qword ptr [rbp + 0x1e8], rdx
1412a22b2: mov qword ptr [rbp + 0x1e0], r15
1412a22b9: mov edx, 0xfffffffe
1412a22be: sub edx, ecx
1412a22c0: not ecx
1412a22c2: mov r8d, ecx
1412a22c5: and r8d, edx
1412a22c8: mov word ptr [rbp + 0x1f8], r8w
1412a22d0: dec rax
1412a22d3: mov qword ptr [rbp + 0x200], rax
1412a22da: tzcnt eax, ecx
1412a22de: shl eax, 6
1412a22e1: sub r15, rax
1412a22e4: lea rax, [r15 - 0x40]
1412a22e8: lea rcx, [r15 - 0x30]
1412a22ec: mov qword ptr [rbp + 0x430], rax
1412a22f3: mov qword ptr [rbp + 0x438], rcx
1412a22fa: mov byte ptr [rbp + 0x4e5], 1
1412a2301: mov byte ptr [rbp + 0x4e4], 1
1412a2308: lea rcx, [rbp + 0x3b0]
1412a230f: lea rdx, [rbp + 0x430]
1412a2316: call 0x140a04cf0 ; core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<ref$<asset::prefab::PrefabName>,ref$<asset::prefab::config::PrefabConfig> > > >,country_core::startup::load_prefabs::load_prefabs_impl::closure_env$2>
1412a231b: test al, al
1412a231d: je 0x1412a2230
1412a2323: mov qword ptr [rbp + 0x230], 0
1412a232e: mov qword ptr [rbp + 0x238], 8
1412a2339: mov qword ptr [rbp + 0x240], 0
1412a2344: mov r13, qword ptr [r15 - 0x30]
1412a2348: mov rsi, qword ptr [r15 - 0x18]
1412a234c: movdqa xmm0, xmmword ptr [r13]
1412a2352: pmovmskb eax, xmm0
1412a2356: not eax
1412a2358: lea r14, [r13 + 0x10]
1412a235c: nop dword ptr [rax]
1412a2360: mov rdx, rsi
1412a2363: sub rsi, 1
1412a2367: jb 0x1412a4d40
1412a236d: test ax, ax
1412a2370: je 0x1412a2380
1412a2372: lea ecx, [rax - 1]
1412a2375: and ecx, eax
1412a2377: mov dword ptr [rbp + 0x3a0], ecx
1412a237d: jmp 0x1412a23b1 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a237f: nop 
1412a2380: movdqa xmm0, xmmword ptr [r14]
1412a2385: pmovmskb eax, xmm0
1412a2389: add r13, -0xd80
1412a2390: add r14, 0x10
1412a2394: cmp eax, 0xffff
1412a2399: je 0x1412a2380
1412a239b: mov ecx, 0xfffffffe
1412a23a0: sub ecx, eax
1412a23a2: not eax
1412a23a4: mov r8d, eax
1412a23a7: and r8d, ecx
1412a23aa: mov dword ptr [rbp + 0x3a0], r8d
1412a23b1: tzcnt eax, eax
1412a23b5: neg rax
1412a23b8: imul rax, rax, 0xd8
1412a23bf: mov r9, rax
1412a23c2: movzx ecx, byte ptr [r13 + rax - 4]
1412a23c8: lea eax, [rcx - 2]
1412a23cb: movzx eax, al
1412a23ce: cmp al, 2
1412a23d0: mov r8d, 2
1412a23d6: cmovae eax, r8d
1412a23da: cmp al, 2
1412a23dc: jne 0x1412a5528
1412a23e2: mov eax, dword ptr [rbp + 0x3a0]
1412a23e8: test cl, 1
1412a23eb: jne 0x1412a2360
1412a23f1: add r9, r13
1412a23f4: mov qword ptr [rbp + 0x3a8], r9
1412a23fb: mov rax, qword ptr [r9 - 0xc0]
1412a2402: movabs rcx, 0x7ffffffffffffffe
1412a240c: add rcx, rax
1412a240f: cmp rcx, 2
1412a2413: mov r8d, 2
1412a2419: cmovae rcx, r8
1412a241d: cmp rcx, 2
1412a2421: mov qword ptr [rbp + 0x220], rdx
1412a2428: jne 0x1412a5887
1412a242e: movabs rcx, 0x8000000000000000
1412a2438: xor rax, rcx
1412a243b: cmp rax, 2
1412a243f: cmovae rax, r8
1412a2443: cmp rax, 2
1412a2447: jne 0x1412a591b
1412a244d: mov rdi, qword ptr [rbp + 0x3a8]
1412a2454: lea rdx, [rdi - 0xc0]
1412a245b: lea rcx, [rbp + 0x190]
1412a2462: call 0x140d7bd90 ; alloc::string::impl$6::clone
1412a2467: mov rax, qword ptr [rdi - 0x90]
1412a246e: movabs rcx, 0x7fffffffffffffff
1412a2478: add rcx, rax
1412a247b: cmp rcx, 2
1412a247f: mov edx, 2
1412a2484: cmovae rcx, rdx
1412a2488: cmp rcx, 2
1412a248c: jne 0x1412a59af
1412a2492: neg rax
1412a2495: mov qword ptr [rbp + 0xa0], rsi
1412a249c: jno 0x1412a24c0
1412a249e: movabs rax, 0x8000000000000000
1412a24a8: mov qword ptr [rbp + 0x1c8], rax
1412a24af: jmp 0x1412a24e1 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a24b1: nop word ptr cs:[rax + rax]
1412a24c0: mov rax, qword ptr [rbp + 0x3a8]
1412a24c7: lea rdx, [rax - 0x90]
1412a24ce: mov byte ptr [rbp + 0x4e1], 1
1412a24d5: lea rcx, [rbp + 0x1c8]
1412a24dc: call 0x140d7bd90 ; alloc::string::impl$6::clone
1412a24e1: mov byte ptr [rbp + 0x4d7], 1
1412a24e8: mov byte ptr [rbp + 0x4d6], 1
1412a24ef: lea rcx, [rbp + 0x430]
1412a24f6: lea rsi, [rbp + 0x190]
1412a24fd: mov rdx, rsi
1412a2500: call 0x140d7bd90 ; alloc::string::impl$6::clone
1412a2505: mov rax, qword ptr [rbp + 0x430]
1412a250c: mov qword ptr [rbp + 0x318], rax
1412a2513: mov rax, qword ptr [rbp + 0x438]
1412a251a: mov qword ptr [rbp + 0x218], rax
1412a2521: mov rdi, qword ptr [rbp + 0x440]
1412a2528: mov byte ptr [rbp + 0x4d5], 1
1412a252f: mov byte ptr [rbp + 0x4d4], 1
1412a2536: mov byte ptr [rbp + 0x4d3], 1
1412a253d: lea rcx, [rbp + 0x3b0]
1412a2544: mov rdx, rsi
1412a2547: call 0x140d7bd90 ; alloc::string::impl$6::clone
1412a254c: xor eax, eax
1412a254e: cmp rax, qword ptr [rbp + 0x1c8]
1412a2555: jno 0x1412a2570
1412a2557: movabs rax, 0x8000000000000000
1412a2561: mov qword ptr [rbp + 0x430], rax
1412a2568: jmp 0x1412a2583 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a256a: nop word ptr [rax + rax]
1412a2570: lea rcx, [rbp + 0x430]
1412a2577: lea rdx, [rbp + 0x1c8]
1412a257e: call 0x140d7bd90 ; alloc::string::impl$6::clone
1412a2583: mov rax, qword ptr [rbp + 0x3c0]
1412a258a: mov qword ptr [rbp + 0x2d0], rax
1412a2591: movdqu xmm0, xmmword ptr [rbp + 0x3b0]
1412a2599: movdqa xmmword ptr [rbp + 0x2c0], xmm0
1412a25a1: mov rax, qword ptr [rbp + 0x440]
1412a25a8: mov qword ptr [rbx + 0x10], rax
1412a25ac: mov rax, qword ptr [rbp + 0x430]
1412a25b3: mov qword ptr [rbx], rax
1412a25b6: mov rax, qword ptr [rbp + 0x438]
1412a25bd: mov qword ptr [rbx + 8], rax
1412a25c1: cmp qword ptr [rbp + 0x178], 0
1412a25c9: je 0x1412a2860
1412a25cf: mov byte ptr [rbp + 0x4e0], 1
1412a25d6: mov byte ptr [rbp + 0x4df], 1
1412a25dd: mov byte ptr [rbp + 0x4de], 1
1412a25e4: lea rcx, [rbp + 0x180]
1412a25eb: lea rdx, [rbp + 0x2c0]
1412a25f2: call 0x1412e7e00 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > > > >
1412a25f7: mov rcx, qword ptr [rbp + 0x160]
1412a25fe: mov rdx, qword ptr [rbp + 0x168]
1412a2605: mov r9, rdx
1412a2608: and r9, rax
1412a260b: shr rax, 0x39
1412a260f: movd xmm0, eax
1412a2613: punpcklbw xmm0, xmm0
1412a2617: pshuflw xmm0, xmm0, 0
1412a261c: pshufd xmm12, xmm0, 0
1412a2622: lea r10, [rcx - 0x38]
1412a2626: mov rax, qword ptr [rbp + 0x2c8]
1412a262d: mov qword ptr [rbp + 0x4c8], rax
1412a2634: mov r8, qword ptr [rbp + 0x2d0]
1412a263b: mov r11, qword ptr [rbp + 0x2d8]
1412a2642: mov rax, qword ptr [rbp + 0x2e8]
1412a2649: mov qword ptr [rbp + 0x2f8], rax
1412a2650: mov rax, qword ptr [rbp + 0x2e0]
1412a2657: mov qword ptr [rbp + 0x270], rax
1412a265e: mov qword ptr [rbp + 0x278], 0
1412a2669: mov qword ptr [rbp + 0x358], rcx
1412a2670: movdqu xmm13, xmmword ptr [rcx + r9]
1412a2676: movdqa xmm0, xmm13
1412a267b: pcmpeqb xmm0, xmm12
1412a2680: pmovmskb ecx, xmm0
1412a2684: test ecx, ecx
1412a2686: je 0x1412a272f
1412a268c: mov rax, r11
1412a268f: neg rax
1412a2692: mov qword ptr [rbp + 0x280], rdi
1412a2699: mov qword ptr [rbp + 0x288], rdx
1412a26a0: mov qword ptr [rbp + 0x290], r9
1412a26a7: mov qword ptr [rbp + 0x298], r11
1412a26ae: jno 0x1412a2770
1412a26b4: mov qword ptr [rbp + 0x310], rcx
1412a26bb: tzcnt edi, ecx
1412a26bf: add rdi, r9
1412a26c2: and rdi, rdx
1412a26c5: neg rdi
1412a26c8: imul rsi, rdi, 0x38
1412a26cc: cmp r8, qword ptr [r10 + rsi + 0x10]
1412a26d1: jne 0x1412a2702
1412a26d3: add rsi, r10
1412a26d6: mov rdx, qword ptr [rsi + 8]
1412a26da: mov rcx, qword ptr [rbp + 0x4c8]
1412a26e1: mov rbx, r10
1412a26e4: mov r12, r8
1412a26e7: call 0x142923bf0
1412a26ec: mov r8, r12
1412a26ef: mov r10, rbx
1412a26f2: test eax, eax
1412a26f4: jne 0x1412a2702
1412a26f6: xor eax, eax
1412a26f8: cmp rax, qword ptr [rsi + 0x18]
1412a26fc: jo 0x1412a281a
1412a2702: mov rcx, qword ptr [rbp + 0x310]
1412a2709: lea eax, [rcx - 1]
1412a270c: and ax, cx
1412a270f: mov ecx, eax
1412a2711: mov rdi, qword ptr [rbp + 0x280]
1412a2718: mov rdx, qword ptr [rbp + 0x288]
1412a271f: mov r9, qword ptr [rbp + 0x290]
1412a2726: mov r11, qword ptr [rbp + 0x298]
1412a272d: jne 0x1412a26b4
1412a272f: pcmpeqb xmm13, xmm9
1412a2734: pmovmskb eax, xmm13
1412a2739: test eax, eax
1412a273b: jne 0x1412a2860
1412a2741: mov rax, qword ptr [rbp + 0x278]
1412a2748: add r9, rax
1412a274b: add r9, 0x10
1412a274f: add rax, 0x10
1412a2753: mov qword ptr [rbp + 0x278], rax
1412a275a: and r9, rdx
1412a275d: mov rcx, qword ptr [rbp + 0x358]
1412a2764: jmp 0x1412a2670 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a2769: nop dword ptr [rax]
1412a2770: mov qword ptr [rbp + 0x310], rcx
1412a2777: tzcnt edi, ecx
1412a277b: add rdi, r9
1412a277e: and rdi, rdx
1412a2781: neg rdi
1412a2784: imul rsi, rdi, 0x38
1412a2788: cmp r8, qword ptr [r10 + rsi + 0x10]
1412a278d: jne 0x1412a27ba
1412a278f: add rsi, r10
1412a2792: mov rdx, qword ptr [rsi + 8]
1412a2796: mov rcx, qword ptr [rbp + 0x4c8]
1412a279d: mov rbx, r10
1412a27a0: mov r12, r8
1412a27a3: call 0x142923bf0
1412a27a8: mov r8, r12
1412a27ab: mov r10, rbx
1412a27ae: test eax, eax
1412a27b0: jne 0x1412a27ba
1412a27b2: xor eax, eax
1412a27b4: cmp rax, qword ptr [rsi + 0x18]
1412a27b8: jno 0x1412a27ec
1412a27ba: mov rcx, qword ptr [rbp + 0x310]
1412a27c1: lea eax, [rcx - 1]
1412a27c4: and ax, cx
1412a27c7: mov ecx, eax
1412a27c9: mov rdi, qword ptr [rbp + 0x280]
1412a27d0: mov rdx, qword ptr [rbp + 0x288]
1412a27d7: mov r9, qword ptr [rbp + 0x290]
1412a27de: mov r11, qword ptr [rbp + 0x298]
1412a27e5: jne 0x1412a2770
1412a27e7: jmp 0x1412a272f ; country_core::startup::load_prefabs::load_prefabs_impl
1412a27ec: mov rax, qword ptr [rbp + 0x2f8]
1412a27f3: cmp rax, qword ptr [rsi + 0x28]
1412a27f7: jne 0x1412a27ba
1412a27f9: mov rdx, qword ptr [rsi + 0x20]
1412a27fd: mov rcx, qword ptr [rbp + 0x270]
1412a2804: mov r8, qword ptr [rbp + 0x2f8]
1412a280b: call 0x142923bf0
1412a2810: mov r8, r12
1412a2813: mov r10, rbx
1412a2816: test eax, eax
1412a2818: jne 0x1412a27ba
1412a281a: imul rax, rdi, 0x38
1412a281e: mov rcx, qword ptr [rbp + 0x358]
1412a2825: mov r12, qword ptr [rcx + rax - 8]
1412a282a: lock inc qword ptr [r12]
1412a282f: jle 0x1412a689d
1412a2835: xor esi, esi
1412a2837: mov byte ptr [rbp + 0x270], 1
1412a283e: mov rdx, qword ptr [rbp + 0x2c0]
1412a2845: test rdx, rdx
1412a2848: jne 0x1412a4406
1412a284e: jmp 0x1412a4418 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a2853: nop word ptr cs:[rax + rax]
1412a2860: mov rax, qword ptr [rbp + 0x318]
1412a2867: mov qword ptr [rbp + 0x3b0], rax
1412a286e: mov rax, qword ptr [rbp + 0x218]
1412a2875: mov qword ptr [rbp + 0x3b8], rax
1412a287c: mov qword ptr [rbp + 0x3c0], rdi
1412a2883: movabs rax, 0x8000000000000000
1412a288d: mov qword ptr [rbp + 0x3c8], rax
1412a2894: mov byte ptr [rbp + 0x4e0], 1
1412a289b: mov byte ptr [rbp + 0x4df], 1
1412a28a2: mov byte ptr [rbp + 0x4de], 0
1412a28a9: lea rcx, [rbp + 0x430]
1412a28b0: lea rdx, [rbp + 0x160]
1412a28b7: lea r8, [rbp + 0x3b0]
1412a28be: call 0x1415244d0 ; hashbrown::map::HashMap::rustc_entry<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>
1412a28c3: mov rax, qword ptr [rbp + 0x430]
1412a28ca: mov rdx, qword ptr [rbp + 0x438]
1412a28d1: mov rcx, rax
1412a28d4: neg rcx
1412a28d7: jo 0x1412a2bac
1412a28dd: mov rcx, qword ptr [rbp + 0x440]
1412a28e4: lea r9, [rbp + 0x448]
1412a28eb: mov r8, qword ptr [r9 + 0x20]
1412a28ef: lea r10, [rbp + 0x378]
1412a28f6: mov qword ptr [r10 + 0x20], r8
1412a28fa: movdqu xmm0, xmmword ptr [r9]
1412a28ff: movdqu xmm1, xmmword ptr [r9 + 0x10]
1412a2905: movdqu xmmword ptr [r10 + 0x10], xmm1
1412a290b: movdqu xmmword ptr [r10], xmm0
1412a2910: mov qword ptr [rbp + 0x360], rax
1412a2917: mov qword ptr [rbp + 0x368], rdx
1412a291e: mov qword ptr [rbp + 0x370], rcx
1412a2925: lea rax, [rbp + 0x190]
1412a292c: mov qword ptr [rbp + 0x3b0], rax
1412a2933: lea rax, [rip - 0x1266e6a]
1412a293a: mov qword ptr [rbp + 0x3b8], rax
1412a2941: lea rax, [rip + 0x1987f98]
1412a2948: mov qword ptr [rbp + 0x430], rax
1412a294f: mov qword ptr [rbp + 0x438], 2
1412a295a: lea rax, [rbp + 0x3b0]
1412a2961: mov qword ptr [rbp + 0x440], rax
1412a2968: mov qword ptr [rbp + 0x448], 1
1412a2973: mov qword ptr [rbp + 0x450], 0
1412a297e: lea rsi, [rbp + 0x320]
1412a2985: mov rcx, rsi
1412a2988: lea rdx, [rbp + 0x430]
1412a298f: call 0x140d79f90 ; alloc::fmt::format::format_inner
1412a2994: mov byte ptr [rbp + 0x338], 1
1412a299b: mov byte ptr [rbp + 0x4e3], 1
1412a29a2: mov rcx, rsi
1412a29a5: call 0x140d76ed0 ; std::sys::os_str::wtf8::Buf::as_mut_slice
1412a29aa: mov r9, qword ptr [r15 - 0x40]
1412a29ae: mov rcx, qword ptr [r15 - 0x38]
1412a29b2: lock inc qword ptr [r9]
1412a29b6: jle 0x1412a689d
1412a29bc: mov r8, rdx
1412a29bf: mov byte ptr [rbp + 0x4e3], 1
1412a29c6: mov qword ptr [rsp + 0x20], rcx
1412a29cb: lea rcx, [rbp + 0x100]
1412a29d2: mov rdx, rax
1412a29d5: call 0x142009df0 ; asset::prefab::scan::PrefabDependencies::add_file_to_prefab_dependency
1412a29da: movdqu xmm0, xmmword ptr [rbp + 0x320]
1412a29e2: movdqu xmm1, xmmword ptr [rbp + 0x330]
1412a29ea: movdqa xmmword ptr [rbp + 0x440], xmm1
1412a29f2: movdqa xmmword ptr [rbp + 0x430], xmm0
1412a29fa: mov byte ptr [rbp + 0x4e3], 0
1412a2a01: lea rcx, [rbp + 0x3b0]
1412a2a08: lea rdx, [rbp + 0x430]
1412a2a0f: call 0x140b0fe60 ; country_core::utils::load_json::load_json_as_mesh<std::path::PathBuf>
1412a2a14: xor eax, eax
1412a2a16: cmp rax, qword ptr [rbp + 0x3b0]
1412a2a1d: jo 0x1412a67c4
1412a2a23: mov rax, qword ptr [rbp + 0x420]
1412a2a2a: lea rcx, [rbp + 0x448]
1412a2a31: mov qword ptr [rcx + 0x68], rax
1412a2a35: movups xmm0, xmmword ptr [rbp + 0x410]
1412a2a3c: movups xmmword ptr [rcx + 0x58], xmm0
1412a2a40: movups xmm0, xmmword ptr [rbp + 0x400]
1412a2a47: movups xmmword ptr [rcx + 0x48], xmm0
1412a2a4b: movups xmm0, xmmword ptr [rbp + 0x3f0]
1412a2a52: movups xmmword ptr [rcx + 0x38], xmm0
1412a2a56: movdqu xmm0, xmmword ptr [rbp + 0x3b0]
1412a2a5e: movdqu xmm1, xmmword ptr [rbp + 0x3c0]
1412a2a66: movdqu xmm2, xmmword ptr [rbp + 0x3d0]
1412a2a6e: movups xmm3, xmmword ptr [rbp + 0x3e0]
1412a2a75: movups xmmword ptr [rcx + 0x28], xmm3
1412a2a79: movdqu xmmword ptr [rcx + 0x18], xmm2
1412a2a7e: movdqu xmmword ptr [rcx + 8], xmm1
1412a2a83: movdqu xmmword ptr [rcx - 8], xmm0
1412a2a88: mov qword ptr [rbp + 0x430], 1
1412a2a93: mov qword ptr [rbp + 0x438], 1
1412a2a9e: movzx eax, byte ptr [rip + 0x2392164]
1412a2aa5: mov ecx, 0x88
1412a2aaa: mov edx, 8
1412a2aaf: call 0x140613c10
1412a2ab4: test rax, rax
1412a2ab7: je 0x1412a6802
1412a2abd: mov rsi, rax
1412a2ac0: mov r8d, 0x88
1412a2ac6: mov rcx, rax
1412a2ac9: lea rdx, [rbp + 0x430]
1412a2ad0: call 0x142923c00
1412a2ad5: mov rdx, qword ptr [rbp + 0x398]
1412a2adc: movups xmm0, xmmword ptr [rbp + 0x360]
1412a2ae3: movdqu xmm1, xmmword ptr [rbp + 0x370]
1412a2aeb: movdqu xmm2, xmmword ptr [rbp + 0x380]
1412a2af3: movdqa xmmword ptr [rbp + 0x450], xmm2
1412a2afb: movdqa xmmword ptr [rbp + 0x440], xmm1
1412a2b03: movaps xmmword ptr [rbp + 0x430], xmm0
1412a2b0a: mov rax, qword ptr [rbp + 0x390]
1412a2b11: mov rcx, qword ptr [rax]
1412a2b14: mov r8, qword ptr [rax + 8]
1412a2b18: mov r9, r8
1412a2b1b: and r9, rdx
1412a2b1e: movdqu xmm0, xmmword ptr [rcx + r9]
1412a2b24: pmovmskb r10d, xmm0
1412a2b29: test r10d, r10d
1412a2b2c: je 0x1412a4cb9
1412a2b32: tzcnt r10d, r10d
1412a2b37: add r10, r9
1412a2b3a: and r10, r8
1412a2b3d: movzx r9d, byte ptr [rcx + r10]
1412a2b42: test r9b, r9b
1412a2b45: jns 0x1412a4cdf
1412a2b4b: shr rdx, 0x39
1412a2b4f: lea r11, [r10 - 0x10]
1412a2b53: and r11, r8
1412a2b56: mov byte ptr [rcx + r10], dl
1412a2b5a: mov byte ptr [rcx + r11 + 0x10], dl
1412a2b5f: neg r10
1412a2b62: and r9b, 1
1412a2b66: movzx edx, r9b
1412a2b6a: sub qword ptr [rax + 0x10], rdx
1412a2b6e: imul r8, r10, 0x38
1412a2b72: lea rdx, [rcx + r8]
1412a2b76: movdqa xmm0, xmmword ptr [rbp + 0x430]
1412a2b7e: movdqa xmm1, xmmword ptr [rbp + 0x440]
1412a2b86: movdqa xmm2, xmmword ptr [rbp + 0x450]
1412a2b8e: movdqu xmmword ptr [rcx + r8 - 0x18], xmm2
1412a2b95: movdqu xmmword ptr [rcx + r8 - 0x28], xmm1
1412a2b9c: movdqu xmmword ptr [rcx + r8 - 0x38], xmm0
1412a2ba3: mov qword ptr [rcx + r8 - 8], rsi
1412a2ba8: inc qword ptr [rax + 0x18]
1412a2bac: mov r12, qword ptr [rdx - 8]
1412a2bb0: lock inc qword ptr [r12]
1412a2bb5: jle 0x1412a689d
1412a2bbb: mov qword ptr [rbp + 0x260], r12
1412a2bc2: xor eax, eax
1412a2bc4: cmp rax, qword ptr [rbp + 0x1c8]
1412a2bcb: seto bl
1412a2bce: mov byte ptr [rbp + 0x270], bl
1412a2bd4: jo 0x1412a43f7
1412a2bda: mov rax, qword ptr [rbp + 0x1d8]
1412a2be1: mov qword ptr [rbp + 0x2b0], rax
1412a2be8: movdqu xmm0, xmmword ptr [rbp + 0x1c8]
1412a2bf0: movdqa xmmword ptr [rbp + 0x2a0], xmm0
1412a2bf8: lea rcx, [rbp + 0x2a0]
1412a2bff: mov qword ptr [rbp + 0x90], rcx
1412a2c06: mov r8, qword ptr [rbp + 0x2a8]
1412a2c0d: cmp rax, 0xd
1412a2c11: je 0x1412a2da7
1412a2c17: cmp rax, 0xe
1412a2c1b: jne 0x1412a6734
1412a2c21: mov rax, qword ptr [r8]
1412a2c24: movabs rcx, 0x5f74636172747865
1412a2c2e: xor rax, rcx
1412a2c31: mov rcx, qword ptr [r8 + 6]
1412a2c35: movabs rdx, 0x79706f6e61635f74
1412a2c3f: xor rcx, rdx
1412a2c42: or rcx, rax
1412a2c45: jne 0x1412a6734
1412a2c4b: add r12, 0x10
1412a2c4f: mov byte ptr [rbp + 0x4e2], 1
1412a2c56: mov r9d, 0xe
1412a2c5c: lea rcx, [rbp + 0x430]
1412a2c63: mov rdx, r12
1412a2c66: call 0x140a08680 ; country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1>
1412a2c6b: cmp qword ptr [rbp + 0x480], 0
1412a2c73: je 0x1412a2f3c
1412a2c79: mov rsi, qword ptr [rbp + 0x468]
1412a2c80: movdqa xmm0, xmmword ptr [rsi]
1412a2c84: pmovmskb ecx, xmm0
1412a2c88: mov rax, rsi
1412a2c8b: cmp ecx, 0xffff
1412a2c91: jne 0x1412a2cba
1412a2c93: lea rdx, [rsi + 0x10]
1412a2c97: mov rax, rsi
1412a2c9a: nop word ptr [rax + rax]
1412a2ca0: movdqa xmm0, xmmword ptr [rdx]
1412a2ca4: pmovmskb ecx, xmm0
1412a2ca8: add rax, -0x380
1412a2cae: add rdx, 0x10
1412a2cb2: cmp ecx, 0xffff
1412a2cb8: je 0x1412a2ca0
1412a2cba: not ecx
1412a2cbc: tzcnt ecx, ecx
1412a2cc0: neg rcx
1412a2cc3: imul rcx, rcx, 0x38
1412a2cc7: mov rax, qword ptr [rax + rcx - 8]
1412a2ccc: mov qword ptr [rbp + 0x308], rax
1412a2cd3: mov r8d, 0xb
1412a2cd9: lea rcx, [rbp + 0x488]
1412a2ce0: lea rdx, [rip + 0x1987c69]
1412a2ce7: call 0x1405b1a70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<str$> >
1412a2cec: mov rdi, qword ptr [rbp + 0x470]
1412a2cf3: mov rcx, rdi
1412a2cf6: and rcx, rax
1412a2cf9: shr rax, 0x39
1412a2cfd: movd xmm0, eax
1412a2d01: punpcklbw xmm0, xmm0
1412a2d05: pshuflw xmm0, xmm0, 0
1412a2d0a: pshufd xmm0, xmm0, 0
1412a2d0f: lea rbx, [rsi - 0x38]
1412a2d13: xor eax, eax
1412a2d15: movdqu xmm1, xmmword ptr [rsi + rcx]
1412a2d1a: movdqa xmm2, xmm1
1412a2d1e: pcmpeqb xmm2, xmm0
1412a2d22: pmovmskb edx, xmm2
1412a2d26: test edx, edx
1412a2d28: je 0x1412a2d83
1412a2d2a: tzcnt r8d, edx
1412a2d2f: add r8, rcx
1412a2d32: and r8, rdi
1412a2d35: neg r8
1412a2d38: imul r8, r8, 0x38
1412a2d3c: cmp qword ptr [rbx + r8 + 0x10], 0xb
1412a2d42: jne 0x1412a2d76
1412a2d44: lea r9, [rbx + r8]
1412a2d48: mov r9, qword ptr [r9 + 8]
1412a2d4c: mov r10, qword ptr [r9]
1412a2d4f: movabs r11, 0x6e65635f6d697270
1412a2d59: xor r10, r11
1412a2d5c: mov r9, qword ptr [r9 + 3]
1412a2d60: movabs r11, 0x7265746e65635f6d
1412a2d6a: xor r9, r11
1412a2d6d: or r9, r10
1412a2d70: je 0x1412a2e51
1412a2d76: lea r8d, [rdx - 1]
1412a2d7a: and r8w, dx
1412a2d7e: mov edx, r8d
1412a2d81: jne 0x1412a2d2a
1412a2d83: pcmpeqb xmm1, xmm9
1412a2d88: pmovmskb edx, xmm1
1412a2d8c: test edx, edx
1412a2d8e: jne 0x1412a39ad
1412a2d94: add rcx, rax
1412a2d97: add rcx, 0x10
1412a2d9b: add rax, 0x10
1412a2d9f: and rcx, rdi
1412a2da2: jmp 0x1412a2d15 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a2da7: mov rax, qword ptr [r8]
1412a2daa: movabs rcx, 0x5f74636172747865
1412a2db4: xor rax, rcx
1412a2db7: mov rcx, qword ptr [r8 + 5]
1412a2dbb: movabs rdx, 0x6b6e7572745f7463
1412a2dc5: xor rcx, rdx
1412a2dc8: or rcx, rax
1412a2dcb: jne 0x1412a6734
1412a2dd1: add r12, 0x10
1412a2dd5: mov byte ptr [rbp + 0x4e2], 1
1412a2ddc: mov r9d, 0xd
1412a2de2: lea rcx, [rbp + 0x430]
1412a2de9: mov rdx, r12
1412a2dec: call 0x140a08d30 ; country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2>
1412a2df1: cmp qword ptr [rbp + 0x480], 0
1412a2df9: je 0x1412a2f4c
1412a2dff: mov rax, qword ptr [rbp + 0x468]
1412a2e06: movdqa xmm0, xmmword ptr [rax]
1412a2e0a: pmovmskb ecx, xmm0
1412a2e0e: cmp ecx, 0xffff
1412a2e14: jne 0x1412a2e3a
1412a2e16: lea rdx, [rax + 0x10]
1412a2e1a: nop word ptr [rax + rax]
1412a2e20: movdqa xmm0, xmmword ptr [rdx]
1412a2e24: pmovmskb ecx, xmm0
1412a2e28: add rax, -0x380
1412a2e2e: add rdx, 0x10
1412a2e32: cmp ecx, 0xffff
1412a2e38: je 0x1412a2e20
1412a2e3a: not ecx
1412a2e3c: tzcnt ecx, ecx
1412a2e40: neg rcx
1412a2e43: imul rcx, rcx, 0x38
1412a2e47: mov rbx, qword ptr [rax + rcx - 8]
1412a2e4c: jmp 0x1412a2f4e ; country_core::startup::load_prefabs::load_prefabs_impl
1412a2e51: cmp dword ptr [rsi + r8 - 0x20], 3
1412a2e57: jne 0x1412a5608
1412a2e5d: add r8, rsi
1412a2e60: mov rax, qword ptr [r8 - 0x10]
1412a2e64: mov qword ptr [rbp + 0x288], rax
1412a2e6b: mov rax, qword ptr [r8 - 8]
1412a2e6f: mov qword ptr [rbp + 0x228], rax
1412a2e76: mov r8d, 0xf
1412a2e7c: lea rcx, [rbp + 0x488]
1412a2e83: lea rdx, [rip + 0x19874d7]
1412a2e8a: call 0x1405b1a70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<str$> >
1412a2e8f: mov rcx, rax
1412a2e92: shr rcx, 0x39
1412a2e96: movd xmm0, ecx
1412a2e9a: punpcklbw xmm0, xmm0
1412a2e9e: pshuflw xmm0, xmm0, 0
1412a2ea3: pshufd xmm0, xmm0, 0
1412a2ea8: xor ecx, ecx
1412a2eaa: and rax, rdi
1412a2ead: movdqu xmm1, xmmword ptr [rsi + rax]
1412a2eb2: movdqa xmm2, xmm1
1412a2eb6: pcmpeqb xmm2, xmm0
1412a2eba: pmovmskb edx, xmm2
1412a2ebe: test edx, edx
1412a2ec0: je 0x1412a2f1b
1412a2ec2: tzcnt r8d, edx
1412a2ec7: add r8, rax
1412a2eca: and r8, rdi
1412a2ecd: neg r8
1412a2ed0: imul r8, r8, 0x38
1412a2ed4: cmp qword ptr [rbx + r8 + 0x10], 0xf
1412a2eda: jne 0x1412a2f0e
1412a2edc: lea r9, [rbx + r8]
1412a2ee0: mov r9, qword ptr [r9 + 8]
1412a2ee4: mov r10, qword ptr [r9]
1412a2ee7: movabs r11, 0x505f786574726556
1412a2ef1: xor r10, r11
1412a2ef4: mov r9, qword ptr [r9 + 7]
1412a2ef8: movabs r11, 0x6e6f697469736f50
1412a2f02: xor r9, r11
1412a2f05: or r9, r10
1412a2f08: je 0x1412a2ff5
1412a2f0e: lea r8d, [rdx - 1]
1412a2f12: and r8w, dx
1412a2f16: mov edx, r8d
1412a2f19: jne 0x1412a2ec2
1412a2f1b: pcmpeqb xmm1, xmm9
1412a2f20: pmovmskb edx, xmm1
1412a2f24: test edx, edx
1412a2f26: jne 0x1412a544c
1412a2f2c: add rax, rcx
1412a2f2f: add rax, 0x10
1412a2f33: add rcx, 0x10
1412a2f37: jmp 0x1412a2eaa ; country_core::startup::load_prefabs::load_prefabs_impl
1412a2f3c: mov qword ptr [rbp + 0x308], 0
1412a2f47: jmp 0x1412a39ad ; country_core::startup::load_prefabs::load_prefabs_impl
1412a2f4c: xor ebx, ebx
1412a2f4e: mov r8d, 2
1412a2f54: lea rcx, [rbp + 0x488]
1412a2f5b: lea rsi, [rip + 0x19878ee]
1412a2f62: mov rdx, rsi
1412a2f65: call 0x1405b1a70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<str$> >
1412a2f6a: mov qword ptr [rsp + 0x20], 2
1412a2f73: lea rcx, [rbp + 0x3b0]
1412a2f7a: lea rdx, [rbp + 0x468]
1412a2f81: mov r8, rax
1412a2f84: mov r9, rsi
1412a2f87: call 0x140b78d80 ; hashbrown::raw::RawTable::remove_entry<tuple$<alloc::string::String,enum2$<country_core::render::mesh::VertexAttributeValues> >,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<str$,alloc::string::String,enum2$<country_core::render::mesh::VertexAttributeValues> > >
1412a2f8c: mov r12, qword ptr [rbp + 0x3c8]
1412a2f93: cmp r12, 5
1412a2f97: je 0x1412a3a91
1412a2f9d: mov rdx, qword ptr [rbp + 0x3b0]
1412a2fa4: mov rsi, qword ptr [rbp + 0x3d0]
1412a2fab: mov rdi, qword ptr [rbp + 0x3d8]
1412a2fb2: test rdx, rdx
1412a2fb5: je 0x1412a2fc9
1412a2fb7: mov rcx, qword ptr [rbp + 0x3b8]
1412a2fbe: mov r8d, 1
1412a2fc4: call 0x140613c20
1412a2fc9: cmp r12, 3
1412a2fcd: ja 0x1412a3a77
1412a2fd3: lea rcx, [rip + 0x1987b1e]
1412a2fda: movsxd rax, dword ptr [rcx + r12*4]
1412a2fde: add rax, rcx
1412a2fe1: jmp rax
1412a2fe3: test rsi, rsi
1412a2fe6: je 0x1412a3a91
1412a2fec: shl rsi, 2
1412a2ff0: jmp 0x1412a3a80 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a2ff5: cmp dword ptr [rsi + r8 - 0x20], 3
1412a2ffb: jne 0x1412a5650
1412a3001: add rsi, r8
1412a3004: mov rbx, qword ptr [rsi - 0x10]
1412a3008: mov rax, qword ptr [rsi - 8]
1412a300c: mov qword ptr [rbp + 0x4c8], rax
1412a3013: call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1412a3018: mov rsi, rax
1412a301b: test byte ptr [rax], 1
1412a301e: je 0x1412a3031
1412a3020: mov rcx, rsi
1412a3023: add rcx, 8
1412a3027: mov rax, qword ptr [rsi + 8]
1412a302b: mov rdx, qword ptr [rsi + 0x10]
1412a302f: jmp 0x1412a304c ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3031: call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1412a3036: mov rcx, rsi
1412a3039: add rcx, 8
1412a303d: mov qword ptr [rsi], 1
1412a3044: mov qword ptr [rsi + 8], rax
1412a3048: mov qword ptr [rsi + 0x10], rdx
1412a304c: lea r8, [rax + 1]
1412a3050: mov qword ptr [rcx], r8
1412a3053: movaps xmmword ptr [rbp + 0x370], xmm6
1412a305a: movaps xmmword ptr [rbp + 0x360], xmm7
1412a3061: mov qword ptr [rbp + 0x380], rax
1412a3068: mov qword ptr [rbp + 0x388], rdx
1412a306f: cmp qword ptr [rbp + 0x480], 0
1412a3077: je 0x1412a5469
1412a307d: mov r8d, 0xd
1412a3083: lea rcx, [rbp + 0x488]
1412a308a: lea rdx, [rip + 0x19878e7]
1412a3091: call 0x1405b1a70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<str$> >
1412a3096: mov r9, qword ptr [rbp + 0x468]
1412a309d: mov rcx, qword ptr [rbp + 0x470]
1412a30a4: mov rdx, rcx
1412a30a7: and rdx, rax
1412a30aa: shr rax, 0x39
1412a30ae: movd xmm0, eax
1412a30b2: punpcklbw xmm0, xmm0
1412a30b6: pshuflw xmm0, xmm0, 0
1412a30bb: pshufd xmm0, xmm0, 0
1412a30c0: lea rax, [r9 - 0x38]
1412a30c4: xor r8d, r8d
1412a30c7: mov r12, r9
1412a30ca: movdqu xmm1, xmmword ptr [r9 + rdx]
1412a30d0: movdqa xmm2, xmm1
1412a30d4: pcmpeqb xmm2, xmm0
1412a30d8: pmovmskb r9d, xmm2
1412a30dd: test r9d, r9d
1412a30e0: je 0x1412a3137
1412a30e2: tzcnt r10d, r9d
1412a30e7: add r10, rdx
1412a30ea: and r10, rcx
1412a30ed: neg r10
1412a30f0: imul r10, r10, 0x38
1412a30f4: cmp qword ptr [rax + r10 + 0x10], 0xd
1412a30fa: jne 0x1412a312a
1412a30fc: lea r11, [rax + r10]
1412a3100: mov r11, qword ptr [r11 + 8]
1412a3104: mov rsi, qword ptr [r11]
1412a3107: movabs rdi, 0x4e5f786574726556
1412a3111: xor rsi, rdi
1412a3114: mov r11, qword ptr [r11 + 5]
1412a3118: movabs rdi, 0x6c616d726f4e5f78
1412a3122: xor r11, rdi
1412a3125: or r11, rsi
1412a3128: je 0x1412a3160
1412a312a: lea r10d, [r9 - 1]
1412a312e: and r10w, r9w
1412a3132: mov r9d, r10d
1412a3135: jne 0x1412a30e2
1412a3137: pcmpeqb xmm1, xmm9
1412a313c: pmovmskb r9d, xmm1
1412a3141: test r9d, r9d
1412a3144: jne 0x1412a5469
1412a314a: add rdx, r8
1412a314d: add rdx, 0x10
1412a3151: add r8, 0x10
1412a3155: and rdx, rcx
1412a3158: mov r9, r12
1412a315b: jmp 0x1412a30c7 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3160: cmp dword ptr [r12 + r10 - 0x20], 3
1412a3166: jne 0x1412a5698
1412a316c: add r12, r10
1412a316f: mov rax, qword ptr [r12 - 0x10]
1412a3174: mov qword ptr [rbp + 0x298], rax
1412a317b: mov qword ptr [rbp + 0xc8], r12
1412a3182: mov rcx, qword ptr [r12 - 8]
1412a3187: mov rax, qword ptr [rbp + 0x228]
1412a318e: cmp rcx, rax
1412a3191: cmovae rcx, rax
1412a3195: mov qword ptr [rbp + 0x290], rcx
1412a319c: mov qword ptr [rbp + 0x358], 0
1412a31a7: xor eax, eax
1412a31a9: jmp 0x1412a31f9 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a31ab: nop dword ptr [rax + rax]
1412a31b0: mov rax, qword ptr [rsi - 0x20]
1412a31b4: mov rcx, qword ptr [rbp + 0x358]
1412a31bb: mov dword ptr [rax + r12*4], ecx
1412a31bf: inc r12
1412a31c2: mov qword ptr [rsi - 0x18], r12
1412a31c6: movss xmm0, dword ptr [rdi + 8]
1412a31cb: movsd xmm1, qword ptr [rdi]
1412a31cf: movsd xmm2, qword ptr [rsi - 0x10]
1412a31d4: addps xmm2, xmm1
1412a31d7: movlps qword ptr [rsi - 0x10], xmm2
1412a31db: addss xmm0, dword ptr [rsi - 8]
1412a31e0: movss dword ptr [rsi - 8], xmm0
1412a31e5: mov eax, dword ptr [rbp + 0x278]
1412a31eb: mov qword ptr [rbp + 0x358], rax
1412a31f2: mov rax, qword ptr [rbp + 0x2f8]
1412a31f9: cmp rax, qword ptr [rbp + 0x290]
1412a3200: jae 0x1412a34e4
1412a3206: mov qword ptr [rbp + 0x2f8], rax
1412a320d: lea rax, [rax*4]
1412a3215: lea rdi, [rax + rax*2]
1412a3219: mov rcx, qword ptr [rbp + 0x288]
1412a3220: mov eax, dword ptr [rcx + rdi + 8]
1412a3224: mov rcx, qword ptr [rcx + rdi]
1412a3228: mov qword ptr [rbp + 0x3b0], rcx
1412a322f: mov dword ptr [rbp + 0x3b8], eax
1412a3235: lea rcx, [rbp + 0x380]
1412a323c: lea rdx, [rbp + 0x3b0]
1412a3243: call 0x1412e7c80 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<array$<u32,3> > >
1412a3248: mov rsi, rax
1412a324b: inc qword ptr [rbp + 0x2f8]
1412a3252: add rdi, qword ptr [rbp + 0x298]
1412a3259: mov rax, qword ptr [rbp + 0x358]
1412a3260: inc eax
1412a3262: mov dword ptr [rbp + 0x278], eax
1412a3268: mov rcx, rsi
1412a326b: shr rcx, 0x39
1412a326f: mov rax, qword ptr [rbp + 0x360]
1412a3276: mov r8, qword ptr [rbp + 0x368]
1412a327d: mov rdx, r8
1412a3280: and rdx, rsi
1412a3283: mov qword ptr [rbp + 0x280], rcx
1412a328a: movd xmm0, ecx
1412a328e: punpcklbw xmm0, xmm0
1412a3292: pshuflw xmm0, xmm0, 0
1412a3297: pshufd xmm0, xmm0, 0
1412a329c: lea r11, [rax - 0x38]
1412a32a0: mov r10d, dword ptr [rbp + 0x3b8]
1412a32a7: mov rcx, qword ptr [rbp + 0x3b0]
1412a32ae: mov qword ptr [rbp + 0x310], rcx
1412a32b5: xor r9d, r9d
1412a32b8: mov r12, rdx
1412a32bb: movdqu xmm1, xmmword ptr [rax + r12]
1412a32c1: movdqa xmm2, xmm1
1412a32c5: pcmpeqb xmm2, xmm0
1412a32c9: pmovmskb ecx, xmm2
1412a32cd: test ecx, ecx
1412a32cf: je 0x1412a3355
1412a32d5: mov qword ptr [rbp + 0xa8], rdx
1412a32dc: mov qword ptr [rbp + 0xb0], rsi
1412a32e3: mov qword ptr [rbp + 0xb8], r9
1412a32ea: mov qword ptr [rbp + 0xc0], r12
1412a32f1: mov r9, rbx
1412a32f4: tzcnt edx, ecx
1412a32f8: add rdx, r12
1412a32fb: and rdx, r8
1412a32fe: neg rdx
1412a3301: imul rdx, rdx, 0x38
1412a3305: mov esi, dword ptr [r11 + rdx + 8]
1412a330a: mov r12, r8
1412a330d: mov r8, qword ptr [r11 + rdx]
1412a3311: xor r8, qword ptr [rbp + 0x310]
1412a3318: mov rbx, r10
1412a331b: mov r10d, ebx
1412a331e: xor r10, rsi
1412a3321: or r10, r8
1412a3324: je 0x1412a3380
1412a3326: lea edx, [rcx - 1]
1412a3329: and dx, cx
1412a332c: mov ecx, edx
1412a332e: mov r10, rbx
1412a3331: mov rbx, r9
1412a3334: mov r8, r12
1412a3337: mov rdx, qword ptr [rbp + 0xa8]
1412a333e: mov rsi, qword ptr [rbp + 0xb0]
1412a3345: mov r9, qword ptr [rbp + 0xb8]
1412a334c: mov r12, qword ptr [rbp + 0xc0]
1412a3353: jne 0x1412a32f1
1412a3355: pcmpeqb xmm1, xmm9
1412a335a: pmovmskb ecx, xmm1
1412a335e: test ecx, ecx
1412a3360: jne 0x1412a339c
1412a3362: add r12, r9
1412a3365: add r12, 0x10
1412a3369: add r9, 0x10
1412a336d: and r12, r8
1412a3370: jmp 0x1412a32bb ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3375: nop word ptr cs:[rax + rax]
1412a3380: add rax, rdx
1412a3383: mov rsi, rax
1412a3386: mov rbx, r9
1412a3389: mov r12, qword ptr [rsi - 0x18]
1412a338d: cmp r12, qword ptr [rsi - 0x28]
1412a3391: jne 0x1412a31b0
1412a3397: jmp 0x1412a344d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a339c: cmp qword ptr [rbp + 0x370], 0
1412a33a4: je 0x1412a3485
1412a33aa: mov r11, r10
1412a33ad: movdqu xmm0, xmmword ptr [rax + rdx]
1412a33b2: pmovmskb ecx, xmm0
1412a33b6: test ecx, ecx
1412a33b8: mov r12, qword ptr [rbp + 0x310]
1412a33bf: je 0x1412a3462
1412a33c5: tzcnt ecx, ecx
1412a33c9: add rcx, rdx
1412a33cc: and rcx, r8
1412a33cf: movzx r9d, byte ptr [rax + rcx]
1412a33d4: test r9b, r9b
1412a33d7: jns 0x1412a34ce
1412a33dd: lea rdx, [rcx - 0x10]
1412a33e1: and rdx, r8
1412a33e4: mov r8, qword ptr [rbp + 0x280]
1412a33eb: mov byte ptr [rax + rcx], r8b
1412a33ef: mov byte ptr [rax + rdx + 0x10], r8b
1412a33f4: neg rcx
1412a33f7: imul rcx, rcx, 0x38
1412a33fb: lea rsi, [rax + rcx]
1412a33ff: and r9b, 1
1412a3403: movzx edx, r9b
1412a3407: sub qword ptr [rbp + 0x370], rdx
1412a340e: mov qword ptr [rax + rcx - 0x38], r12
1412a3413: mov dword ptr [rax + rcx - 0x30], r11d
1412a3418: mov qword ptr [rax + rcx - 0x28], 0
1412a3421: mov qword ptr [rax + rcx - 0x20], 4
1412a342a: movups xmmword ptr [rax + rcx - 0x18], xmm10
1412a3430: mov dword ptr [rax + rcx - 8], 0
1412a3438: inc qword ptr [rbp + 0x378]
1412a343f: mov r12, qword ptr [rsi - 0x18]
1412a3443: cmp r12, qword ptr [rsi - 0x28]
1412a3447: jne 0x1412a31b0
1412a344d: lea rcx, [rsi - 0x28]
1412a3451: lea rdx, [rip + 0x19875e8]
1412a3458: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
1412a345d: jmp 0x1412a31b0 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3462: mov r10d, 0x10
1412a3468: add rdx, r10
1412a346b: and rdx, r8
1412a346e: movdqu xmm0, xmmword ptr [rax + rdx]
1412a3473: pmovmskb ecx, xmm0
1412a3477: add r10, 0x10
1412a347b: test ecx, ecx
1412a347d: jne 0x1412a33c5
1412a3483: jmp 0x1412a3468 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3485: mov edx, 1
1412a348a: lea rcx, [rbp + 0x360]
1412a3491: lea r8, [rbp + 0x380]
1412a3498: mov r9b, 1
1412a349b: call 0x1428c22a0 ; hashbrown::raw::RawTable::reserve_rehash<tuple$<array$<u32,3>,country_core::startup::load_prefabs::recalculate_canopy_quad_attribs::Quad>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<array$<u32,3>,country_core::startup::load_prefabs::recalculate_canopy_quad_attribs::Quad,std::hash::random::RandomState> >
1412a34a0: mov rax, qword ptr [rbp + 0x3b0]
1412a34a7: mov qword ptr [rbp + 0x310], rax
1412a34ae: mov r10d, dword ptr [rbp + 0x3b8]
1412a34b5: mov rax, qword ptr [rbp + 0x360]
1412a34bc: mov r8, qword ptr [rbp + 0x368]
1412a34c3: and rsi, r8
1412a34c6: mov rdx, rsi
1412a34c9: jmp 0x1412a33aa ; country_core::startup::load_prefabs::load_prefabs_impl
1412a34ce: movdqa xmm0, xmmword ptr [rax]
1412a34d2: pmovmskb ecx, xmm0
1412a34d6: tzcnt ecx, ecx
1412a34da: movzx r9d, byte ptr [rax + rcx]
1412a34df: jmp 0x1412a33dd ; country_core::startup::load_prefabs::load_prefabs_impl
1412a34e4: mov rax, qword ptr [rbp + 0x378]
1412a34eb: test rax, rax
1412a34ee: je 0x1412a3619
1412a34f4: mov rcx, qword ptr [rbp + 0x360]
1412a34fb: lea rdx, [rcx + 0x10]
1412a34ff: movdqa xmm0, xmmword ptr [rcx]
1412a3503: pmovmskb r9d, xmm0
1412a3508: not r9d
1412a350b: jmp 0x1412a354d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a350d: xorps xmm0, xmm0
1412a3510: movsldup xmm2, xmm2
1412a3514: mulps xmm1, xmm2
1412a3517: movzx r10d, r10b
1412a351b: movd xmm2, r10d
1412a3520: pshufd xmm2, xmm2, 0x50
1412a3525: pslld xmm2, 0x1f
1412a352a: psrad xmm2, 0x1f
1412a352f: pand xmm2, xmm1
1412a3533: movq qword ptr [rcx + r9 - 0x10], xmm2
1412a353a: movss dword ptr [rcx + r9 - 8], xmm0
1412a3541: mov r9d, r8d
1412a3544: dec rax
1412a3547: je 0x1412a3619
1412a354d: test r9w, r9w
1412a3551: je 0x1412a3560
1412a3553: lea r8d, [r9 - 1]
1412a3557: and r8d, r9d
1412a355a: jmp 0x1412a358f ; country_core::startup::load_prefabs::load_prefabs_impl
1412a355c: nop dword ptr [rax]
1412a3560: movdqa xmm0, xmmword ptr [rdx]
1412a3564: pmovmskb r9d, xmm0
1412a3569: add rcx, -0x380
1412a3570: add rdx, 0x10
1412a3574: cmp r9d, 0xffff
1412a357b: je 0x1412a3560
1412a357d: mov r10d, 0xfffffffe
1412a3583: sub r10d, r9d
1412a3586: not r9d
1412a3589: mov r8d, r9d
1412a358c: and r8d, r10d
1412a358f: tzcnt r9d, r9d
1412a3594: neg r9
1412a3597: imul r9, r9, 0x38
1412a359b: movss xmm0, dword ptr [rcx + r9 - 8]
1412a35a2: movaps xmm2, xmm0
1412a35a5: mulss xmm2, xmm0
1412a35a9: movsd xmm1, qword ptr [rcx + r9 - 0x10]
1412a35b0: movaps xmm3, xmm1
1412a35b3: mulps xmm3, xmm1
1412a35b6: movshdup xmm4, xmm3
1412a35ba: addss xmm4, xmm3
1412a35be: addss xmm4, xmm2
1412a35c2: xorps xmm3, xmm3
1412a35c5: sqrtss xmm3, xmm4
1412a35c9: movaps xmm2, xmm11
1412a35cd: divss xmm2, xmm3
1412a35d1: movd r10d, xmm2
1412a35d6: test r10d, r10d
1412a35d9: setns r11b
1412a35dd: mov esi, r10d
1412a35e0: and esi, 0x7fffffff
1412a35e6: add esi, 0xff800000
1412a35ec: cmp esi, 0x7f000000
1412a35f2: setb sil
1412a35f6: and sil, r11b
1412a35f9: dec r10d
1412a35fc: cmp r10d, 0x7fffff
1412a3603: setb r10b
1412a3607: or r10b, sil
1412a360a: je 0x1412a350d
1412a3610: mulss xmm0, xmm2
1412a3614: jmp 0x1412a3510 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3619: mov qword ptr [rbp + 0x80], 0
1412a3624: mov dword ptr [rbp + 0x88], 0
1412a362e: mov dword ptr [rbp + 0x3b8], 0
1412a3638: mov qword ptr [rbp + 0x3b0], 0
1412a3643: lea rcx, [rbp + 0x1b0]
1412a364a: lea rdx, [rbp + 0x3b0]
1412a3651: mov r8, qword ptr [rbp + 0x228]
1412a3658: lea r9, [rip + 0x1987341]
1412a365f: call 0x1412f7260 ; alloc::vec::from_elem<array$<f32,3> >
1412a3664: mov rdx, qword ptr [rbp + 0x1c0]
1412a366b: mov qword ptr [rbp + 0x78], rdx
1412a366f: mov rax, qword ptr [rbp + 0xc8]
1412a3676: cmp dword ptr [rax - 0x20], 3
1412a367a: jne 0x1412a56e0
1412a3680: mov rax, qword ptr [rax - 8]
1412a3684: mov qword ptr [rbp + 0x320], rax
1412a368b: cmp rdx, rax
1412a368e: jne 0x1412a572f
1412a3694: mov rax, qword ptr [rbp + 0x378]
1412a369b: test rax, rax
1412a369e: je 0x1412a3796
1412a36a4: mov r8, qword ptr [rbp + 0x360]
1412a36ab: lea r9, [r8 + 0x10]
1412a36af: movdqa xmm0, xmmword ptr [r8]
1412a36b4: pmovmskb r11d, xmm0
1412a36b9: not r11d
1412a36bc: mov r10, qword ptr [rbp + 0x1b8]
1412a36c3: jmp 0x1412a36e0 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a36c5: nop word ptr cs:[rax + rax]
1412a36d0: mov rax, qword ptr [rbp + 0x358]
1412a36d7: dec rax
1412a36da: je 0x1412a3796
1412a36e0: test r11w, r11w
1412a36e4: mov qword ptr [rbp + 0x358], rax
1412a36eb: je 0x1412a3700
1412a36ed: lea ecx, [r11 - 1]
1412a36f1: and ecx, r11d
1412a36f4: jmp 0x1412a372e ; country_core::startup::load_prefabs::load_prefabs_impl
1412a36f6: nop word ptr cs:[rax + rax]
1412a3700: movdqa xmm0, xmmword ptr [r9]
1412a3705: pmovmskb r11d, xmm0
1412a370a: add r8, -0x380
1412a3711: add r9, 0x10
1412a3715: cmp r11d, 0xffff
1412a371c: je 0x1412a3700
1412a371e: mov eax, 0xfffffffe
1412a3723: sub eax, r11d
1412a3726: not r11d
1412a3729: mov ecx, r11d
1412a372c: and ecx, eax
1412a372e: mov eax, r11d
1412a3731: tzcnt eax, eax
1412a3735: mov r11d, ecx
1412a3738: neg rax
1412a373b: imul rax, rax, 0x38
1412a373f: mov rsi, qword ptr [r8 + rax - 0x18]
1412a3744: test rsi, rsi
1412a3747: je 0x1412a36d0
1412a3749: lea rdi, [r8 + rax]
1412a374d: mov r12, qword ptr [r8 + rax - 0x20]
1412a3752: shl rsi, 2
1412a3756: xor eax, eax
1412a3758: nop dword ptr [rax + rax]
1412a3760: mov ecx, dword ptr [r12 + rax]
1412a3764: cmp rdx, rcx
1412a3767: jbe 0x1412a576b
1412a376d: movd xmm0, dword ptr [rdi - 8]
1412a3772: lea rcx, [rcx + rcx*2]
1412a3776: movq xmm1, qword ptr [rdi - 0x10]
1412a377b: movq qword ptr [r10 + rcx*4], xmm1
1412a3781: movd dword ptr [r10 + rcx*4 + 8], xmm0
1412a3788: add rax, 4
1412a378c: cmp rsi, rax
1412a378f: jne 0x1412a3760
1412a3791: jmp 0x1412a36d0 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3796: mov byte ptr [rbp + 0x4dd], 1
1412a379d: lea rcx, [rbp + 0x320]
1412a37a4: lea rdx, [rbp + 0x80]
1412a37ab: mov r8, qword ptr [rbp + 0x228]
1412a37b2: lea r9, [rip + 0x1987217]
1412a37b9: call 0x1412f7260 ; alloc::vec::from_elem<array$<f32,3> >
1412a37be: mov rax, qword ptr [rbp + 0x378]
1412a37c5: test rax, rax
1412a37c8: je 0x1412a38fa
1412a37ce: mov r8, qword ptr [rbp + 0x360]
1412a37d5: lea r9, [r8 + 0x10]
1412a37d9: movdqa xmm0, xmmword ptr [r8]
1412a37de: pmovmskb r11d, xmm0
1412a37e3: not r11d
1412a37e6: mov r10, qword ptr [rbp + 0x328]
1412a37ed: mov rdx, qword ptr [rbp + 0x330]
1412a37f4: jmp 0x1412a3810 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a37f6: nop word ptr cs:[rax + rax]
1412a3800: mov rax, qword ptr [rbp + 0x358]
1412a3807: dec rax
1412a380a: je 0x1412a38fa
1412a3810: test r11w, r11w
1412a3814: mov qword ptr [rbp + 0x358], rax
1412a381b: je 0x1412a3830
1412a381d: lea eax, [r11 - 1]
1412a3821: and eax, r11d
1412a3824: jmp 0x1412a385e ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3826: nop word ptr cs:[rax + rax]
1412a3830: movdqa xmm0, xmmword ptr [r9]
1412a3835: pmovmskb r11d, xmm0
1412a383a: add r8, -0x380
1412a3841: add r9, 0x10
1412a3845: cmp r11d, 0xffff
1412a384c: je 0x1412a3830
1412a384e: mov ecx, 0xfffffffe
1412a3853: sub ecx, r11d
1412a3856: not r11d
1412a3859: mov eax, r11d
1412a385c: and eax, ecx
1412a385e: mov ecx, r11d
1412a3861: tzcnt ecx, ecx
1412a3865: mov r11d, eax
1412a3868: neg rcx
1412a386b: imul rax, rcx, 0x38
1412a386f: mov r12, qword ptr [r8 + rax - 0x18]
1412a3874: test r12, r12
1412a3877: je 0x1412a3800
1412a3879: mov rsi, qword ptr [r8 + rax - 0x20]
1412a387e: lea rdi, [r12*4]
1412a3886: pxor xmm0, xmm0
1412a388a: pxor xmm1, xmm1
1412a388e: xor eax, eax
1412a3890: mov ecx, dword ptr [rsi + rax]
1412a3893: cmp qword ptr [rbp + 0x4c8], rcx
1412a389a: jbe 0x1412a5783
1412a38a0: lea rcx, [rcx + rcx*2]
1412a38a4: movsd xmm2, qword ptr [rbx + rcx*4]
1412a38a9: addps xmm0, xmm2
1412a38ac: addss xmm1, dword ptr [rbx + rcx*4 + 8]
1412a38b2: add rax, 4
1412a38b6: cmp rdi, rax
1412a38b9: jne 0x1412a3890
1412a38bb: xorps xmm2, xmm2
1412a38be: cvtsi2ss xmm2, r12
1412a38c3: movsldup xmm3, xmm2
1412a38c7: divps xmm0, xmm3
1412a38ca: divss xmm1, xmm2
1412a38ce: xor eax, eax
1412a38d0: mov ecx, dword ptr [rsi + rax]
1412a38d3: cmp rdx, rcx
1412a38d6: jbe 0x1412a57a9
1412a38dc: lea rcx, [rcx + rcx*2]
1412a38e0: movlps qword ptr [r10 + rcx*4], xmm0
1412a38e5: movss dword ptr [r10 + rcx*4 + 8], xmm1
1412a38ec: add rax, 4
1412a38f0: cmp rdi, rax
1412a38f3: jne 0x1412a38d0
1412a38f5: jmp 0x1412a3800 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a38fa: mov rax, qword ptr [rbp + 0x1c0]
1412a3901: mov qword ptr [rbp + 0x3c0], rax
1412a3908: movups xmm0, xmmword ptr [rbp + 0x1b0]
1412a390f: movaps xmmword ptr [rbp + 0x3b0], xmm0
1412a3916: mov byte ptr [rbp + 0x4da], 1
1412a391d: mov byte ptr [rbp + 0x4d9], 0
1412a3924: mov r8d, 0xc
1412a392a: lea rcx, [rbp + 0x430]
1412a3931: lea rdx, [rip + 0x19870b0]
1412a3938: lea r9, [rbp + 0x3b0]
1412a393f: call 0x140a34c90 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global> >
1412a3944: mov rax, qword ptr [rbp + 0x330]
1412a394b: mov qword ptr [rbp + 0x3c0], rax
1412a3952: movups xmm0, xmmword ptr [rbp + 0x320]
1412a3959: movaps xmmword ptr [rbp + 0x3b0], xmm0
1412a3960: mov byte ptr [rbp + 0x4da], 0
1412a3967: mov byte ptr [rbp + 0x4d9], 0
1412a396e: mov r8d, 0xb
1412a3974: lea rcx, [rbp + 0x430]
1412a397b: lea rdx, [rip + 0x1986fce]
1412a3982: lea r9, [rbp + 0x3b0]
1412a3989: call 0x140a34c90 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global> >
1412a398e: mov r8d, 0x38
1412a3994: mov r9d, 0x10
1412a399a: lea rcx, [rbp + 0x360]
1412a39a1: lea rdx, [rbp + 0x380]
1412a39a8: call 0x140b74f50 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<array$<u32,3>,country_core::startup::load_prefabs::recalculate_canopy_quad_attribs::Quad>,alloc::alloc::Global>
1412a39ad: mov r8d, 2
1412a39b3: lea rcx, [rbp + 0x488]
1412a39ba: lea rsi, [rip + 0x1986e8f]
1412a39c1: mov rdx, rsi
1412a39c4: call 0x1405b1a70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<str$> >
1412a39c9: mov qword ptr [rsp + 0x20], 2
1412a39d2: lea rcx, [rbp + 0x3b0]
1412a39d9: lea rdx, [rbp + 0x468]
1412a39e0: mov r8, rax
1412a39e3: mov r9, rsi
1412a39e6: call 0x140b78d80 ; hashbrown::raw::RawTable::remove_entry<tuple$<alloc::string::String,enum2$<country_core::render::mesh::VertexAttributeValues> >,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<str$,alloc::string::String,enum2$<country_core::render::mesh::VertexAttributeValues> > >
1412a39eb: mov rbx, qword ptr [rbp + 0x3c8]
1412a39f2: cmp rbx, 5
1412a39f6: je 0x1412a3ddc
1412a39fc: mov rdx, qword ptr [rbp + 0x3b0]
1412a3a03: mov rsi, qword ptr [rbp + 0x3d0]
1412a3a0a: mov rdi, qword ptr [rbp + 0x3d8]
1412a3a11: test rdx, rdx
1412a3a14: je 0x1412a3a28
1412a3a16: mov rcx, qword ptr [rbp + 0x3b8]
1412a3a1d: mov r8d, 1
1412a3a23: call 0x140613c20
1412a3a28: cmp rbx, 3
1412a3a2c: ja 0x1412a3dc2
1412a3a32: lea rcx, [rip + 0x19870cf]
1412a3a39: movsxd rax, dword ptr [rcx + rbx*4]
1412a3a3d: add rax, rcx
1412a3a40: jmp rax
1412a3a42: test rsi, rsi
1412a3a45: je 0x1412a3ddc
1412a3a4b: shl rsi, 2
1412a3a4f: jmp 0x1412a3dcb ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3a54: test rsi, rsi
1412a3a57: je 0x1412a3a91
1412a3a59: shl rsi, 3
1412a3a5d: jmp 0x1412a3a80 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3a5f: test rsi, rsi
1412a3a62: je 0x1412a3a91
1412a3a64: shl rsi, 2
1412a3a68: lea rdx, [rsi + rsi*2]
1412a3a6c: mov r8d, 4
1412a3a72: mov rcx, rdi
1412a3a75: jmp 0x1412a3a8c ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3a77: test rsi, rsi
1412a3a7a: je 0x1412a3a91
1412a3a7c: shl rsi, 4
1412a3a80: mov r8d, 4
1412a3a86: mov rcx, rdi
1412a3a89: mov rdx, rsi
1412a3a8c: call 0x140613c20
1412a3a91: test rbx, rbx
1412a3a94: je 0x1412a4121
1412a3a9a: cmp qword ptr [rbp + 0x480], 0
1412a3aa2: je 0x1412a3b7f
1412a3aa8: mov r8d, 0xa
1412a3aae: lea rcx, [rbp + 0x488]
1412a3ab5: lea rdx, [rip + 0x1986d96]
1412a3abc: call 0x1405b1a70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<str$> >
1412a3ac1: mov rcx, qword ptr [rbp + 0x468]
1412a3ac8: mov rdx, qword ptr [rbp + 0x470]
1412a3acf: mov r8, rdx
1412a3ad2: and r8, rax
1412a3ad5: shr rax, 0x39
1412a3ad9: movd xmm0, eax
1412a3add: punpcklbw xmm0, xmm0
1412a3ae1: pshuflw xmm0, xmm0, 0
1412a3ae6: pshufd xmm0, xmm0, 0
1412a3aeb: lea rax, [rcx - 0x38]
1412a3aef: xor r9d, r9d
1412a3af2: movdqu xmm1, xmmword ptr [rcx + r8]
1412a3af8: movdqa xmm2, xmm1
1412a3afc: pcmpeqb xmm2, xmm0
1412a3b00: pmovmskb r10d, xmm2
1412a3b05: test r10d, r10d
1412a3b08: je 0x1412a3b5d
1412a3b0a: tzcnt r11d, r10d
1412a3b0f: add r11, r8
1412a3b12: and r11, rdx
1412a3b15: neg r11
1412a3b18: imul r11, r11, 0x38
1412a3b1c: cmp qword ptr [rax + r11 + 0x10], 0xa
1412a3b22: jne 0x1412a3b50
1412a3b24: add r11, rax
1412a3b27: mov r11, qword ptr [r11 + 8]
1412a3b2b: mov rsi, qword ptr [r11]
1412a3b2e: movabs rdi, 0x705f726165707061
1412a3b38: xor rsi, rdi
1412a3b3b: movzx r11d, word ptr [r11 + 8]
1412a3b40: xor r11, 0x736f
1412a3b47: or r11, rsi
1412a3b4a: je 0x1412a3bdf
1412a3b50: lea r11d, [r10 - 1]
1412a3b54: and r11w, r10w
1412a3b58: mov r10d, r11d
1412a3b5b: jne 0x1412a3b0a
1412a3b5d: pcmpeqb xmm1, xmm9
1412a3b62: pmovmskb r10d, xmm1
1412a3b67: test r10d, r10d
1412a3b6a: jne 0x1412a3b7f
1412a3b6c: add r8, r9
1412a3b6f: add r8, 0x10
1412a3b73: add r9, 0x10
1412a3b77: and r8, rdx
1412a3b7a: jmp 0x1412a3af2 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3b7f: mov dword ptr [rbp + 0x368], 0
1412a3b89: mov qword ptr [rbp + 0x360], 0
1412a3b94: lea rcx, [rbp + 0x3b0]
1412a3b9b: lea rdx, [rbp + 0x360]
1412a3ba2: mov r8, rbx
1412a3ba5: lea r9, [rip + 0x1986cb4]
1412a3bac: call 0x1412f7260 ; alloc::vec::from_elem<array$<f32,3> >
1412a3bb1: mov r8d, 0xa
1412a3bb7: lea rcx, [rbp + 0x430]
1412a3bbe: lea rdx, [rip + 0x1986c8d]
1412a3bc5: lea r9, [rbp + 0x3b0]
1412a3bcc: call 0x140a34c90 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global> >
1412a3bd1: cmp qword ptr [rbp + 0x480], 0
1412a3bd9: je 0x1412a3cae
1412a3bdf: mov r8d, 3
1412a3be5: lea rcx, [rbp + 0x488]
1412a3bec: lea rdx, [rip + 0x1986c85]
1412a3bf3: call 0x1405b1a70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<str$> >
1412a3bf8: mov rcx, qword ptr [rbp + 0x468]
1412a3bff: mov rdx, qword ptr [rbp + 0x470]
1412a3c06: mov r8, rdx
1412a3c09: and r8, rax
1412a3c0c: shr rax, 0x39
1412a3c10: movd xmm0, eax
1412a3c14: punpcklbw xmm0, xmm0
1412a3c18: pshuflw xmm0, xmm0, 0
1412a3c1d: pshufd xmm0, xmm0, 0
1412a3c22: lea rax, [rcx - 0x38]
1412a3c26: xor r9d, r9d
1412a3c29: movdqu xmm1, xmmword ptr [rcx + r8]
1412a3c2f: movdqa xmm2, xmm1
1412a3c33: pcmpeqb xmm2, xmm0
1412a3c37: pmovmskb r10d, xmm2
1412a3c3c: test r10d, r10d
1412a3c3f: je 0x1412a3c8c
1412a3c41: tzcnt r11d, r10d
1412a3c46: add r11, r8
1412a3c49: and r11, rdx
1412a3c4c: neg r11
1412a3c4f: imul r11, r11, 0x38
1412a3c53: cmp qword ptr [rax + r11 + 0x10], 3
1412a3c59: jne 0x1412a3c7f
1412a3c5b: add r11, rax
1412a3c5e: mov r11, qword ptr [r11 + 8]
1412a3c62: movzx esi, word ptr [r11]
1412a3c66: xor esi, 0x6761
1412a3c6c: movzx r11d, byte ptr [r11 + 2]
1412a3c71: xor r11d, 0x65
1412a3c75: or r11w, si
1412a3c79: je 0x1412a4121
1412a3c7f: lea r11d, [r10 - 1]
1412a3c83: and r11w, r10w
1412a3c87: mov r10d, r11d
1412a3c8a: jne 0x1412a3c41
1412a3c8c: pcmpeqb xmm1, xmm9
1412a3c91: pmovmskb r10d, xmm1
1412a3c96: test r10d, r10d
1412a3c99: jne 0x1412a3cae
1412a3c9b: add r8, r9
1412a3c9e: add r8, 0x10
1412a3ca2: add r9, 0x10
1412a3ca6: and r8, rdx
1412a3ca9: jmp 0x1412a3c29 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3cae: lea rsi, [rbx*4]
1412a3cb6: mov rax, rbx
1412a3cb9: shr rax, 0x3e
1412a3cbd: jne 0x1412a6889
1412a3cc3: mov edi, 0
1412a3cc8: movabs rax, 0x7ffffffffffffffc
1412a3cd2: cmp rsi, rax
1412a3cd5: ja 0x1412a688b
1412a3cdb: movzx eax, byte ptr [rip + 0x2390f27]
1412a3ce2: mov edi, 4
1412a3ce7: mov edx, 4
1412a3cec: mov rcx, rsi
1412a3cef: call 0x140613c10
1412a3cf4: test rax, rax
1412a3cf7: je 0x1412a688b
1412a3cfd: mov rcx, rax
1412a3d00: cmp rbx, 1
1412a3d04: je 0x1412a3d5f
1412a3d06: mov edx, 1
1412a3d0b: mov rcx, rax
1412a3d0e: cmp rbx, 9
1412a3d12: jb 0x1412a3d49
1412a3d14: lea r8, [rbx - 1]
1412a3d18: mov r9, r8
1412a3d1b: and r9, 0xfffffffffffffff8
1412a3d1f: lea rcx, [rax + r9*4]
1412a3d23: lea rdx, [r9 + 1]
1412a3d27: xor r10d, r10d
1412a3d2a: nop word ptr [rax + rax]
1412a3d30: movups xmmword ptr [rax + r10*4], xmm8
1412a3d35: movups xmmword ptr [rax + r10*4 + 0x10], xmm8
1412a3d3b: add r10, 8
1412a3d3f: cmp r9, r10
1412a3d42: jne 0x1412a3d30
1412a3d44: cmp r8, r9
1412a3d47: je 0x1412a3d5f
1412a3d49: mov r8, rbx
1412a3d4c: sub r8, rdx
1412a3d4f: nop 
1412a3d50: mov dword ptr [rcx], 0x3f800000
1412a3d56: add rcx, 4
1412a3d5a: dec r8
1412a3d5d: jne 0x1412a3d50
1412a3d5f: mov dword ptr [rcx], 0x3f800000
1412a3d65: mov qword ptr [rbp + 0x3b0], rbx
1412a3d6c: mov qword ptr [rbp + 0x3b8], rax
1412a3d73: mov qword ptr [rbp + 0x3c0], rbx
1412a3d7a: mov r8d, 3
1412a3d80: lea rcx, [rbp + 0x430]
1412a3d87: lea rdx, [rip + 0x1986aea]
1412a3d8e: lea r9, [rbp + 0x3b0]
1412a3d95: call 0x140a35b80 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<f32,alloc::alloc::Global> >
1412a3d9a: jmp 0x1412a4121 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3d9f: test rsi, rsi
1412a3da2: je 0x1412a3ddc
1412a3da4: shl rsi, 3
1412a3da8: jmp 0x1412a3dcb ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3daa: test rsi, rsi
1412a3dad: je 0x1412a3ddc
1412a3daf: shl rsi, 2
1412a3db3: lea rdx, [rsi + rsi*2]
1412a3db7: mov r8d, 4
1412a3dbd: mov rcx, rdi
1412a3dc0: jmp 0x1412a3dd7 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3dc2: test rsi, rsi
1412a3dc5: je 0x1412a3ddc
1412a3dc7: shl rsi, 4
1412a3dcb: mov r8d, 4
1412a3dd1: mov rcx, rdi
1412a3dd4: mov rdx, rsi
1412a3dd7: call 0x140613c20
1412a3ddc: cmp qword ptr [rbp + 0x480], 0
1412a3de4: je 0x1412a3ec1
1412a3dea: mov r8d, 0xa
1412a3df0: lea rcx, [rbp + 0x488]
1412a3df7: lea rdx, [rip + 0x1986a54]
1412a3dfe: call 0x1405b1a70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<str$> >
1412a3e03: mov rcx, qword ptr [rbp + 0x468]
1412a3e0a: mov rdx, qword ptr [rbp + 0x470]
1412a3e11: mov r8, rdx
1412a3e14: and r8, rax
1412a3e17: shr rax, 0x39
1412a3e1b: movd xmm0, eax
1412a3e1f: punpcklbw xmm0, xmm0
1412a3e23: pshuflw xmm0, xmm0, 0
1412a3e28: pshufd xmm0, xmm0, 0
1412a3e2d: lea rax, [rcx - 0x38]
1412a3e31: xor r9d, r9d
1412a3e34: movdqu xmm1, xmmword ptr [rcx + r8]
1412a3e3a: movdqa xmm2, xmm1
1412a3e3e: pcmpeqb xmm2, xmm0
1412a3e42: pmovmskb r10d, xmm2
1412a3e47: test r10d, r10d
1412a3e4a: je 0x1412a3e9f
1412a3e4c: tzcnt r11d, r10d
1412a3e51: add r11, r8
1412a3e54: and r11, rdx
1412a3e57: neg r11
1412a3e5a: imul r11, r11, 0x38
1412a3e5e: cmp qword ptr [rax + r11 + 0x10], 0xa
1412a3e64: jne 0x1412a3e92
1412a3e66: add r11, rax
1412a3e69: mov r11, qword ptr [r11 + 8]
1412a3e6d: mov rsi, qword ptr [r11]
1412a3e70: movabs rdi, 0x705f726165707061
1412a3e7a: xor rsi, rdi
1412a3e7d: movzx r11d, word ptr [r11 + 8]
1412a3e82: xor r11, 0x736f
1412a3e89: or r11, rsi
1412a3e8c: je 0x1412a3f25
1412a3e92: lea r11d, [r10 - 1]
1412a3e96: and r11w, r10w
1412a3e9a: mov r10d, r11d
1412a3e9d: jne 0x1412a3e4c
1412a3e9f: pcmpeqb xmm1, xmm9
1412a3ea4: pmovmskb r10d, xmm1
1412a3ea9: test r10d, r10d
1412a3eac: jne 0x1412a3ec1
1412a3eae: add r8, r9
1412a3eb1: add r8, 0x10
1412a3eb5: add r9, 0x10
1412a3eb9: and r8, rdx
1412a3ebc: jmp 0x1412a3e34 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3ec1: mov dword ptr [rbp + 0x368], 0
1412a3ecb: mov qword ptr [rbp + 0x360], 0
1412a3ed6: lea rcx, [rbp + 0x3b0]
1412a3edd: lea rdx, [rbp + 0x360]
1412a3ee4: mov r8, qword ptr [rbp + 0x308]
1412a3eeb: lea r9, [rip + 0x19869a6]
1412a3ef2: call 0x1412f7260 ; alloc::vec::from_elem<array$<f32,3> >
1412a3ef7: mov r8d, 0xa
1412a3efd: lea rcx, [rbp + 0x430]
1412a3f04: lea rdx, [rip + 0x1986947]
1412a3f0b: lea r9, [rbp + 0x3b0]
1412a3f12: call 0x140a34c90 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global> >
1412a3f17: cmp qword ptr [rbp + 0x480], 0
1412a3f1f: je 0x1412a3ff4
1412a3f25: mov r8d, 3
1412a3f2b: lea rcx, [rbp + 0x488]
1412a3f32: lea rdx, [rip + 0x198693f]
1412a3f39: call 0x1405b1a70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<str$> >
1412a3f3e: mov rcx, qword ptr [rbp + 0x468]
1412a3f45: mov rdx, qword ptr [rbp + 0x470]
1412a3f4c: mov r8, rdx
1412a3f4f: and r8, rax
1412a3f52: shr rax, 0x39
1412a3f56: movd xmm0, eax
1412a3f5a: punpcklbw xmm0, xmm0
1412a3f5e: pshuflw xmm0, xmm0, 0
1412a3f63: pshufd xmm0, xmm0, 0
1412a3f68: lea rax, [rcx - 0x38]
1412a3f6c: xor r9d, r9d
1412a3f6f: movdqu xmm1, xmmword ptr [rcx + r8]
1412a3f75: movdqa xmm2, xmm1
1412a3f79: pcmpeqb xmm2, xmm0
1412a3f7d: pmovmskb r10d, xmm2
1412a3f82: test r10d, r10d
1412a3f85: je 0x1412a3fd2
1412a3f87: tzcnt r11d, r10d
1412a3f8c: add r11, r8
1412a3f8f: and r11, rdx
1412a3f92: neg r11
1412a3f95: imul r11, r11, 0x38
1412a3f99: cmp qword ptr [rax + r11 + 0x10], 3
1412a3f9f: jne 0x1412a3fc5
1412a3fa1: add r11, rax
1412a3fa4: mov r11, qword ptr [r11 + 8]
1412a3fa8: movzx esi, word ptr [r11]
1412a3fac: xor esi, 0x6761
1412a3fb2: movzx r11d, byte ptr [r11 + 2]
1412a3fb7: xor r11d, 0x65
1412a3fbb: or r11w, si
1412a3fbf: je 0x1412a4121
1412a3fc5: lea r11d, [r10 - 1]
1412a3fc9: and r11w, r10w
1412a3fcd: mov r10d, r11d
1412a3fd0: jne 0x1412a3f87
1412a3fd2: pcmpeqb xmm1, xmm9
1412a3fd7: pmovmskb r10d, xmm1
1412a3fdc: test r10d, r10d
1412a3fdf: jne 0x1412a3ff4
1412a3fe1: add r8, r9
1412a3fe4: add r8, 0x10
1412a3fe8: add r9, 0x10
1412a3fec: and r8, rdx
1412a3fef: jmp 0x1412a3f6f ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3ff4: mov r8, qword ptr [rbp + 0x308]
1412a3ffb: lea rsi, [r8*4]
1412a4003: mov rax, r8
1412a4006: shr rax, 0x3e
1412a400a: jne 0x1412a6873
1412a4010: mov edi, 0
1412a4015: movabs rax, 0x7ffffffffffffffc
1412a401f: cmp rsi, rax
1412a4022: ja 0x1412a6875
1412a4028: test rsi, rsi
1412a402b: je 0x1412a4070
1412a402d: movzx eax, byte ptr [rip + 0x2390bd5]
1412a4034: mov edi, 4
1412a4039: mov edx, 4
1412a403e: mov rcx, rsi
1412a4041: call 0x140613c10
1412a4046: mov r8, qword ptr [rbp + 0x308]
1412a404d: test rax, rax
1412a4050: je 0x1412a6875
1412a4056: mov rcx, r8
1412a4059: cmp r8, 2
1412a405d: jb 0x1412a407d
1412a405f: cmp r8, 8
1412a4063: ja 0x1412a4087
1412a4065: mov r8d, 1
1412a406b: mov rdx, rax
1412a406e: jmp 0x1412a40b9 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4070: mov eax, 4
1412a4075: xor ecx, ecx
1412a4077: cmp r8, 2
1412a407b: jae 0x1412a405f
1412a407d: mov rdx, rax
1412a4080: test r8, r8
1412a4083: jne 0x1412a40df
1412a4085: jmp 0x1412a40e5 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4087: lea r9, [r8 - 1]
1412a408b: mov r10, r9
1412a408e: and r10, 0xfffffffffffffff8
1412a4092: lea rdx, [rax + r10*4]
1412a4096: lea r8, [r10 + 1]
1412a409a: xor r11d, r11d
1412a409d: nop dword ptr [rax]
1412a40a0: movups xmmword ptr [rax + r11*4], xmm8
1412a40a5: movups xmmword ptr [rax + r11*4 + 0x10], xmm8
1412a40ab: add r11, 8
1412a40af: cmp r10, r11
1412a40b2: jne 0x1412a40a0
1412a40b4: cmp r9, r10
1412a40b7: je 0x1412a40df
1412a40b9: mov r9, qword ptr [rbp + 0x308]
1412a40c0: sub r9, r8
1412a40c3: nop word ptr cs:[rax + rax]
1412a40d0: mov dword ptr [rdx], 0x3f800000
1412a40d6: add rdx, 4
1412a40da: dec r9
1412a40dd: jne 0x1412a40d0
1412a40df: mov dword ptr [rdx], 0x3f800000
1412a40e5: mov qword ptr [rbp + 0x3b0], rcx
1412a40ec: mov qword ptr [rbp + 0x3b8], rax
1412a40f3: mov rax, qword ptr [rbp + 0x308]
1412a40fa: mov qword ptr [rbp + 0x3c0], rax
1412a4101: mov r8d, 3
1412a4107: lea rcx, [rbp + 0x430]
1412a410e: lea rdx, [rip + 0x1986763]
1412a4115: lea r9, [rbp + 0x3b0]
1412a411c: call 0x140a35b80 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<f32,alloc::alloc::Global> >
1412a4121: mov rax, qword ptr [rbp + 0x4a0]
1412a4128: mov qword ptr [rbp + 0x420], rax
1412a412f: movups xmm0, xmmword ptr [rbp + 0x490]
1412a4136: movaps xmmword ptr [rbp + 0x410], xmm0
1412a413d: movups xmm0, xmmword ptr [rbp + 0x480]
1412a4144: movaps xmmword ptr [rbp + 0x400], xmm0
1412a414b: movups xmm0, xmmword ptr [rbp + 0x470]
1412a4152: movaps xmmword ptr [rbp + 0x3f0], xmm0
1412a4159: movups xmm0, xmmword ptr [rbp + 0x430]
1412a4160: movdqu xmm1, xmmword ptr [rbp + 0x440]
1412a4168: movdqu xmm2, xmmword ptr [rbp + 0x450]
1412a4170: movups xmm3, xmmword ptr [rbp + 0x460]
1412a4177: movaps xmmword ptr [rbp + 0x3e0], xmm3
1412a417e: movdqa xmmword ptr [rbp + 0x3d0], xmm2
1412a4186: movdqa xmmword ptr [rbp + 0x3c0], xmm1
1412a418e: movaps xmmword ptr [rbp + 0x3b0], xmm0
1412a4195: mov rax, qword ptr [rbp + 0x1a0]
1412a419c: mov qword ptr [rbp + 0x330], rax
1412a41a3: movups xmm0, xmmword ptr [rbp + 0x190]
1412a41aa: movaps xmmword ptr [rbp + 0x320], xmm0
1412a41b1: mov rax, qword ptr [rbp + 0x2b0]
1412a41b8: lea rcx, [rbp + 0x338]
1412a41bf: mov qword ptr [rcx + 0x10], rax
1412a41c3: movdqa xmm0, xmmword ptr [rbp + 0x2a0]
1412a41cb: movdqu xmmword ptr [rcx], xmm0
1412a41cf: mov byte ptr [rbp + 0x4d2], 1
1412a41d6: lea rcx, [rbp + 0x430]
1412a41dd: lea rdx, [rbp + 0x160]
1412a41e4: lea r8, [rbp + 0x320]
1412a41eb: call 0x1415244d0 ; hashbrown::map::HashMap::rustc_entry<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>
1412a41f0: mov rax, qword ptr [rbp + 0x430]
1412a41f7: mov rcx, rax
1412a41fa: neg rcx
1412a41fd: jo 0x1412a6816
1412a4203: lea rdx, [rbp + 0x448]
1412a420a: mov rcx, qword ptr [rdx + 0x20]
1412a420e: lea r8, [rbp + 0x378]
1412a4215: mov qword ptr [r8 + 0x20], rcx
1412a4219: movups xmm0, xmmword ptr [rdx]
1412a421c: movups xmm1, xmmword ptr [rdx + 0x10]
1412a4220: movups xmmword ptr [r8 + 0x10], xmm1
1412a4225: movups xmmword ptr [r8], xmm0
1412a4229: movups xmm0, xmmword ptr [rbp + 0x438]
1412a4230: mov qword ptr [rbp + 0x360], rax
1412a4237: movups xmmword ptr [rbp + 0x368], xmm0
1412a423e: mov rax, qword ptr [rbp + 0x420]
1412a4245: mov qword ptr [rdx + 0x68], rax
1412a4249: movaps xmm0, xmmword ptr [rbp + 0x410]
1412a4250: movups xmmword ptr [rdx + 0x58], xmm0
1412a4254: movaps xmm0, xmmword ptr [rbp + 0x400]
1412a425b: movups xmmword ptr [rdx + 0x48], xmm0
1412a425f: movaps xmm0, xmmword ptr [rbp + 0x3f0]
1412a4266: movups xmmword ptr [rdx + 0x38], xmm0
1412a426a: movdqa xmm0, xmmword ptr [rbp + 0x3b0]
1412a4272: movdqa xmm1, xmmword ptr [rbp + 0x3c0]
1412a427a: movdqa xmm2, xmmword ptr [rbp + 0x3d0]
1412a4282: movaps xmm3, xmmword ptr [rbp + 0x3e0]
1412a4289: movups xmmword ptr [rdx + 0x28], xmm3
1412a428d: movdqu xmmword ptr [rdx + 0x18], xmm2
1412a4292: movdqu xmmword ptr [rdx + 8], xmm1
1412a4297: movdqu xmmword ptr [rdx - 8], xmm0
1412a429c: mov qword ptr [rbp + 0x430], 1
1412a42a7: mov qword ptr [rbp + 0x438], 1
1412a42b2: movzx eax, byte ptr [rip + 0x2390950]
1412a42b9: mov ecx, 0x88
1412a42be: mov edx, 8
1412a42c3: call 0x140613c10
1412a42c8: test rax, rax
1412a42cb: je 0x1412a6862
1412a42d1: mov rsi, rax
1412a42d4: mov r8d, 0x88
1412a42da: mov rcx, rax
1412a42dd: lea rdx, [rbp + 0x430]
1412a42e4: call 0x142923c00
1412a42e9: mov rax, qword ptr [rbp + 0x390]
1412a42f0: mov rdx, qword ptr [rbp + 0x398]
1412a42f7: movups xmm0, xmmword ptr [rbp + 0x360]
1412a42fe: movdqu xmm1, xmmword ptr [rbp + 0x370]
1412a4306: movdqu xmm2, xmmword ptr [rbp + 0x380]
1412a430e: movdqa xmmword ptr [rbp + 0x450], xmm2
1412a4316: movdqa xmmword ptr [rbp + 0x440], xmm1
1412a431e: movaps xmmword ptr [rbp + 0x430], xmm0
1412a4325: mov rcx, qword ptr [rax]
1412a4328: mov r8, qword ptr [rax + 8]
1412a432c: mov r9, r8
1412a432f: and r9, rdx
1412a4332: movdqu xmm0, xmmword ptr [rcx + r9]
1412a4338: pmovmskb r10d, xmm0
1412a433d: test r10d, r10d
1412a4340: je 0x1412a4cf7
1412a4346: tzcnt r10d, r10d
1412a434b: add r10, r9
1412a434e: and r10, r8
1412a4351: movzx r9d, byte ptr [rcx + r10]
1412a4356: test r9b, r9b
1412a4359: jns 0x1412a4d1d
1412a435f: shr rdx, 0x39
1412a4363: lea r11, [r10 - 0x10]
1412a4367: and r11, r8
1412a436a: mov byte ptr [rcx + r10], dl
1412a436e: mov byte ptr [rcx + r11 + 0x10], dl
1412a4373: and r9b, 1
1412a4377: movzx edx, r9b
1412a437b: sub qword ptr [rax + 0x10], rdx
1412a437f: neg r10
1412a4382: imul rdx, r10, 0x38
1412a4386: movdqa xmm0, xmmword ptr [rbp + 0x430]
1412a438e: movdqa xmm1, xmmword ptr [rbp + 0x440]
1412a4396: movdqa xmm2, xmmword ptr [rbp + 0x450]
1412a439e: movdqu xmmword ptr [rcx + rdx - 0x18], xmm2
1412a43a4: movdqu xmmword ptr [rcx + rdx - 0x28], xmm1
1412a43aa: movdqu xmmword ptr [rcx + rdx - 0x38], xmm0
1412a43b0: mov qword ptr [rcx + rdx - 8], rsi
1412a43b5: inc qword ptr [rax + 0x18]
1412a43b9: mov r12, qword ptr [rcx + rdx - 8]
1412a43be: lock inc qword ptr [r12]
1412a43c3: jle 0x1412a689d
1412a43c9: mov rax, qword ptr [rbp + 0x260]
1412a43d0: lock dec qword ptr [rax]
1412a43d4: jne 0x1412a43f7
1412a43d6: mov byte ptr [rbp + 0x4e0], 0
1412a43dd: mov byte ptr [rbp + 0x4df], 0
1412a43e4: mov byte ptr [rbp + 0x4de], 0
1412a43eb: lea rcx, [rbp + 0x260]
1412a43f2: call 0x140b5c830 ; alloc::sync::Arc::drop_slow<country_core::render::mesh::Mesh,alloc::alloc::Global>
1412a43f7: mov sil, 1
1412a43fa: mov rdx, qword ptr [rbp + 0x2c0]
1412a4401: test rdx, rdx
1412a4404: je 0x1412a4418
1412a4406: mov rcx, qword ptr [rbp + 0x2c8]
1412a440d: mov r8d, 1
1412a4413: call 0x140613c20
1412a4418: mov rdx, qword ptr [rbp + 0x2d8]
1412a441f: mov rax, rdx
1412a4422: neg rax
1412a4425: mov rdi, qword ptr [rbp + 0x318]
1412a442c: jo 0x1412a4442
1412a442e: jae 0x1412a4442
1412a4430: mov rcx, qword ptr [rbp + 0x2e0]
1412a4437: mov r8d, 1
1412a443d: call 0x140613c20
1412a4442: test rdi, rdi
1412a4445: sete al
1412a4448: or sil, al
1412a444b: cmp sil, 1
1412a444f: je 0x1412a4466
1412a4451: mov r8d, 1
1412a4457: mov rcx, qword ptr [rbp + 0x218]
1412a445e: mov rdx, rdi
1412a4461: call 0x140613c20
1412a4466: mov rdx, qword ptr [rbp + 0x1c8]
1412a446d: mov rax, rdx
1412a4470: neg rax
1412a4473: movzx eax, byte ptr [rbp + 0x270]
1412a447a: jno 0x1412a4490
1412a447c: test al, al
1412a447e: jne 0x1412a44ab
1412a4480: jmp 0x1412a44c9 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4482: nop word ptr cs:[rax + rax]
1412a4490: test al, al
1412a4492: je 0x1412a44c9
1412a4494: test rdx, rdx
1412a4497: je 0x1412a44ab
1412a4499: mov rcx, qword ptr [rbp + 0x1d0]
1412a44a0: mov r8d, 1
1412a44a6: call 0x140613c20
1412a44ab: mov rdx, qword ptr [rbp + 0x190]
1412a44b2: test rdx, rdx
1412a44b5: je 0x1412a44c9
1412a44b7: mov rcx, qword ptr [rbp + 0x198]
1412a44be: mov r8d, 1
1412a44c4: call 0x140613c20
1412a44c9: mov qword ptr [rbp + 0x2a0], r12
1412a44d0: mov rdx, qword ptr [rbp + 0x3a8]
1412a44d7: mov rax, qword ptr [rdx - 0xa8]
1412a44de: movabs rcx, 0x8000000000000000
1412a44e8: xor rax, rcx
1412a44eb: cmp rax, 2
1412a44ef: mov ecx, 2
1412a44f4: cmovae rax, rcx
1412a44f8: cmp rax, 2
1412a44fc: jne 0x1412a5a4a
1412a4502: lea rax, [rdx - 0xa8]
1412a4509: mov qword ptr [rbp + 0x98], rax
1412a4510: lock inc qword ptr [r12]
1412a4515: jle 0x1412a689d
1412a451b: lea rax, [rdx - 0xd8]
1412a4522: mov qword ptr [rbp + 0x358], rax
1412a4529: mov qword ptr [rbp + 0x4c8], r12
1412a4530: mov qword ptr [rbp + 0x360], r12
1412a4537: mov rdi, qword ptr [rdx - 0xa0]
1412a453e: mov rbx, qword ptr [rdx - 0x98]
1412a4545: xor r12d, r12d
1412a4548: nop dword ptr [rax + rax]
1412a4550: mov esi, r12d
1412a4553: mov eax, 0xa
1412a4558: movzx ecx, r12b
1412a455c: lea rdx, [rip + 0x19865b5]
1412a4563: movsxd rcx, dword ptr [rdx + rcx*4]
1412a4567: add rcx, rdx
1412a456a: jmp rcx
1412a456c: lea rcx, [rip + 0x1815c7d]
1412a4573: mov eax, 0xb
1412a4578: mov r12b, 1
1412a457b: cmp rax, rbx
1412a457e: jne 0x1412a4550
1412a4580: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4585: lea rcx, [rip + 0x1815da4]
1412a458c: mov eax, 0x1e
1412a4591: mov r12b, 0x11
1412a4594: cmp rax, rbx
1412a4597: jne 0x1412a4550
1412a4599: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a459e: lea rcx, [rip + 0x1815d61]
1412a45a5: mov eax, 0x15
1412a45aa: mov r12b, 0xf
1412a45ad: cmp rax, rbx
1412a45b0: jne 0x1412a4550
1412a45b2: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a45b7: lea rcx, [rip + 0x1815d23]
1412a45be: mov eax, 0xb
1412a45c3: mov r12b, 0xc
1412a45c6: cmp rax, rbx
1412a45c9: jne 0x1412a4550
1412a45cb: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a45d0: lea rcx, [rip + 0x1815c65]
1412a45d7: mov eax, 0x1e
1412a45dc: mov r12b, 5
1412a45df: cmp rax, rbx
1412a45e2: jne 0x1412a4550
1412a45e8: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a45ed: lea rcx, [rip + 0x1815cf8]
1412a45f4: mov eax, 9
1412a45f9: mov r12b, 0xd
1412a45fc: cmp rax, rbx
1412a45ff: jne 0x1412a4550
1412a4605: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a460a: lea rcx, [rip + 0x1815cb7]
1412a4611: mov eax, 0xf
1412a4616: mov r12b, 0xa
1412a4619: cmp rax, rbx
1412a461c: jne 0x1412a4550
1412a4622: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4627: lea rcx, [rip + 0x1815d8e]
1412a462e: mov eax, 0x12
1412a4633: mov r12b, 0x1a
1412a4636: cmp rax, rbx
1412a4639: jne 0x1412a4550
1412a463f: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4644: lea rcx, [rip + 0x1815d5b]
1412a464b: mov eax, 0x16
1412a4650: mov r12b, 0x19
1412a4653: cmp rax, rbx
1412a4656: jne 0x1412a4550
1412a465c: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4661: lea rcx, [rip + 0x1815b9d]
1412a4668: mov eax, 0x20
1412a466d: mov r12b, 3
1412a4670: cmp rax, rbx
1412a4673: jne 0x1412a4550
1412a4679: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a467e: lea rcx, [rip + 0x1815c96]
1412a4685: mov eax, 0x15
1412a468a: mov r12b, 0x10
1412a468d: cmp rax, rbx
1412a4690: jne 0x1412a4550
1412a4696: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a469b: lea rcx, [rip + 0x1815b83]
1412a46a2: mov eax, 0x17
1412a46a7: mov r12b, 4
1412a46aa: cmp rax, rbx
1412a46ad: jne 0x1412a4550
1412a46b3: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a46b8: lea rcx, [rip + 0x1815bdd]
1412a46bf: mov eax, 0x19
1412a46c4: mov r12b, 8
1412a46c7: cmp rax, rbx
1412a46ca: jne 0x1412a4550
1412a46d0: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a46d5: mov r12b, 2
1412a46d8: lea rcx, [rip + 0x1815b1c]
1412a46df: cmp rax, rbx
1412a46e2: jne 0x1412a4550
1412a46e8: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a46ed: lea rcx, [rip + 0x1815c5a]
1412a46f4: mov eax, 9
1412a46f9: mov r12b, 0x12
1412a46fc: cmp rax, rbx
1412a46ff: jne 0x1412a4550
1412a4705: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a470a: lea rcx, [rip + 0x1815c67]
1412a4711: mov r12b, 0x15
1412a4714: cmp rax, rbx
1412a4717: jne 0x1412a4550
1412a471d: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4722: lea rcx, [rip + 0x1815b31]
1412a4729: mov eax, 0x24
1412a472e: mov r12b, 6
1412a4731: cmp rax, rbx
1412a4734: jne 0x1412a4550
1412a473a: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a473f: lea rcx, [rip + 0x1815c3c]
1412a4746: mov r12b, 0x16
1412a4749: cmp rax, rbx
1412a474c: jne 0x1412a4550
1412a4752: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4757: lea rcx, [rip + 0x1815b97]
1412a475e: mov eax, 0x11
1412a4763: mov r12b, 0xe
1412a4766: cmp rax, rbx
1412a4769: jne 0x1412a4550
1412a476f: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4774: lea rcx, [rip + 0x1815b03]
1412a477b: mov eax, 0x1e
1412a4780: mov r12b, 7
1412a4783: cmp rax, rbx
1412a4786: jne 0x1412a4550
1412a478c: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4791: lea rcx, [rip + 0x1815bbf]
1412a4798: mov eax, 0x15
1412a479d: mov r12b, 0x13
1412a47a0: cmp rax, rbx
1412a47a3: jne 0x1412a4550
1412a47a9: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a47ae: lea rcx, [rip + 0x1815b22]
1412a47b5: mov r12b, 0xb
1412a47b8: cmp rax, rbx
1412a47bb: jne 0x1412a4550
1412a47c1: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a47c6: lea rcx, [rip + 0x1815ae8]
1412a47cd: mov eax, 0x13
1412a47d2: mov r12b, 9
1412a47d5: cmp rax, rbx
1412a47d8: jne 0x1412a4550
1412a47de: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a47e0: lea rcx, [rip + 0x1815ba5]
1412a47e7: mov eax, 0x11
1412a47ec: mov r12b, 0x17
1412a47ef: cmp rax, rbx
1412a47f2: jne 0x1412a4550
1412a47f8: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a47fa: lea rcx, [rip + 0x1815b6b]
1412a4801: mov eax, 0xc
1412a4806: mov r12b, 0x14
1412a4809: cmp rax, rbx
1412a480c: jne 0x1412a4550
1412a4812: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4814: lea rcx, [rip + 0x1815b82]
1412a481b: mov eax, 9
1412a4820: mov r12b, 0x18
1412a4823: cmp rax, rbx
1412a4826: jne 0x1412a4550
1412a482c: jmp 0x1412a4850 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a482e: lea rcx, [rip + 0x1815b99]
1412a4835: mov eax, 0x11
1412a483a: mov r12b, 0x1b
1412a483d: cmp rax, rbx
1412a4840: jne 0x1412a4550
1412a4846: nop word ptr cs:[rax + rax]
1412a4850: mov rdx, rdi
1412a4853: mov r8, rbx
1412a4856: call 0x142923bf0
1412a485b: test eax, eax
1412a485d: jne 0x1412a4550
1412a4863: mov r12, qword ptr [rbp + 0x4c8]
1412a486a: mov qword ptr [rbp + 0x1b0], r12
1412a4871: mov byte ptr [rbp + 0x1b8], sil
1412a4878: mov rax, qword ptr [rbp + 0x3a8]
1412a487f: mov eax, dword ptr [rax - 0x48]
1412a4882: lea edx, [rax - 4]
1412a4885: cmp edx, 2
1412a4888: mov ecx, 2
1412a488d: cmovb ecx, edx
1412a4890: cmp ecx, 2
1412a4893: jne 0x1412a5ade
1412a4899: mov bl, 2
1412a489b: cmp eax, 3
1412a489e: je 0x1412a49f2
1412a48a4: mov rax, qword ptr [rbp + 0x3a8]
1412a48ab: movzx eax, byte ptr [rax - 8]
1412a48af: cmp eax, 2
1412a48b2: jne 0x1412a5d41
1412a48b8: mov rdx, qword ptr [rbp + 0x2a0]
1412a48bf: add rdx, 0x10
1412a48c3: mov byte ptr [rbp + 0x4e7], 1
1412a48ca: lea rcx, [rbp + 0x360]
1412a48d1: call 0x140a07870 ; country_core::render::mesh::Mesh::aabb_bounds
1412a48d6: mov rdx, qword ptr [rbp + 0x3a8]
1412a48dd: mov eax, dword ptr [rdx - 0x48]
1412a48e0: cmp eax, 2
1412a48e3: jne 0x1412a5ddb
1412a48e9: movd xmm0, dword ptr [rdx - 0x3c]
1412a48ee: movq xmm1, qword ptr [rdx - 0x44]
1412a48f3: movq qword ptr [rbp + 0x3b0], xmm1
1412a48fb: movd dword ptr [rbp + 0x3b8], xmm0
1412a4903: mov byte ptr [rbp + 0x4e7], 1
1412a490a: lea rcx, [rbp + 0x430]
1412a4911: lea rdi, [rbp + 0x360]
1412a4918: lea r8, [rbp + 0x3b0]
1412a491f: mov rbx, rdx
1412a4922: mov rdx, rdi
1412a4925: call 0x141240490 ; utils::geometry::aabb::Aabb3::scaled
1412a492a: mov eax, dword ptr [rbx - 0x38]
1412a492d: cmp eax, 2
1412a4930: jne 0x1412a5e75
1412a4936: mov eax, dword ptr [rbx - 0x2c]
1412a4939: lea rdx, [rbp + 0x448]
1412a4940: mov dword ptr [rdx + 8], eax
1412a4943: mov rax, qword ptr [rbx - 0x34]
1412a4947: mov qword ptr [rdx], rax
1412a494a: mov eax, dword ptr [rbx - 0x28]
1412a494d: cmp eax, 2
1412a4950: jne 0x1412a5f08
1412a4956: mov rcx, rbx
1412a4959: mov eax, dword ptr [rbx - 0x1c]
1412a495c: mov dword ptr [rdx + 0x14], eax
1412a495f: mov rax, qword ptr [rbx - 0x24]
1412a4963: mov qword ptr [rdx + 0xc], rax
1412a4967: mov eax, dword ptr [rbx - 0x18]
1412a496a: cmp eax, 2
1412a496d: mov r12, qword ptr [rbp + 0x4c8]
1412a4974: jne 0x1412a5f9b
1412a497a: mov eax, dword ptr [rcx - 0xc]
1412a497d: mov dword ptr [rdx + 0x20], eax
1412a4980: mov rax, qword ptr [rcx - 0x14]
1412a4984: mov qword ptr [rdx + 0x18], rax
1412a4988: movzx ebx, byte ptr [rcx - 7]
1412a498c: lea eax, [rbx - 2]
1412a498f: movzx ecx, al
1412a4992: cmp cl, 2
1412a4995: mov eax, 2
1412a499a: cmovb eax, ecx
1412a499d: cmp al, 2
1412a499f: jne 0x1412a602e
1412a49a5: mov byte ptr [rbp + 0x46c], bl
1412a49ab: movdqu xmm0, xmmword ptr [rbp + 0x430]
1412a49b3: movdqu xmm1, xmmword ptr [rbp + 0x440]
1412a49bb: movdqu xmm2, xmmword ptr [rbp + 0x450]
1412a49c3: movups xmm3, xmmword ptr [rbp + 0x45c]
1412a49ca: movdqa xmmword ptr [rbp - 0x40], xmm0
1412a49cf: movdqa xmmword ptr [rbp - 0x30], xmm1
1412a49d4: movdqa xmmword ptr [rbp - 0x20], xmm2
1412a49d9: movups xmmword ptr [rbp - 0x14], xmm3
1412a49dd: movzx eax, word ptr [rdx + 0x25]
1412a49e1: mov word ptr [rbp + 0x354], ax
1412a49e8: movzx eax, byte ptr [rdx + 0x27]
1412a49ec: mov byte ptr [rbp + 0x356], al
1412a49f2: mov rax, qword ptr [rbp + 0x3a8]
1412a49f9: mov rax, qword ptr [rax - 0x78]
1412a49fd: movabs rdx, 0x7ffffffffffffffb
1412a4a07: add rdx, rax
1412a4a0a: cmp rdx, 2
1412a4a0e: mov ecx, 2
1412a4a13: cmovb rcx, rdx
1412a4a17: cmp rcx, 2
1412a4a1b: jne 0x1412a5b78
1412a4a21: movabs rcx, 0x8000000000000004
1412a4a2b: movabs rdx, 0x8000000000000000
1412a4a35: mov qword ptr [rbp + 0x318], rdx
1412a4a3c: cmp rax, rcx
1412a4a3f: je 0x1412a4b75
1412a4a45: movabs rcx, 0x7ffffffffffffffe
1412a4a4f: add rax, rcx
1412a4a52: cmp rax, 2
1412a4a56: mov ecx, 2
1412a4a5b: cmovb rcx, rax
1412a4a5f: cmp rcx, 2
1412a4a63: jne 0x1412a60c4
1412a4a69: mov rax, qword ptr [rbp + 0x3a8]
1412a4a70: movzx eax, byte ptr [rax - 0x50]
1412a4a74: cmp eax, 2
1412a4a77: jne 0x1412a615f
1412a4a7d: mov rdx, qword ptr [rbp + 0x2a0]
1412a4a84: add rdx, 0x10
1412a4a88: mov byte ptr [rbp + 0x4e7], 1
1412a4a8f: lea rcx, [rbp + 0x360]
1412a4a96: call 0x140a07870 ; country_core::render::mesh::Mesh::aabb_bounds
1412a4a9b: mov r12, qword ptr [rbp + 0x3a8]
1412a4aa2: mov eax, dword ptr [r12 - 0x60]
1412a4aa7: cmp eax, 2
1412a4aaa: jne 0x1412a61f9
1412a4ab0: movd xmm0, dword ptr [r12 - 0x54]
1412a4ab7: movq xmm1, qword ptr [r12 - 0x5c]
1412a4abe: movq qword ptr [rbp + 0x430], xmm1
1412a4ac6: movd dword ptr [rbp + 0x438], xmm0
1412a4ace: mov byte ptr [rbp + 0x4e7], 1
1412a4ad5: lea rdi, [rbp + 0x360]
1412a4adc: lea r8, [rbp + 0x430]
1412a4ae3: lea rcx, [rbp + 0x3c8]
1412a4aea: mov rdx, rdi
1412a4aed: call 0x141240490 ; utils::geometry::aabb::Aabb3::scaled
1412a4af2: add r12, -0x78
1412a4af6: movabs rcx, 0x8000000000000000
1412a4b00: xor rcx, qword ptr [r12]
1412a4b04: cmp rcx, 2
1412a4b08: mov eax, 2
1412a4b0d: cmovb rax, rcx
1412a4b11: cmp rax, 2
1412a4b15: jne 0x1412a6293
1412a4b1b: mov rdx, r12
1412a4b1e: mov byte ptr [rbp + 0x4e7], 1
1412a4b25: lea rcx, [rbp + 0x3b0]
1412a4b2c: call 0x140d7bd90 ; alloc::string::impl$6::clone
1412a4b31: mov rax, qword ptr [rbp + 0x3b0]
1412a4b38: mov qword ptr [rbp + 0x318], rax
1412a4b3f: mov rax, qword ptr [rbp + 0x3b8]
1412a4b46: mov qword ptr [rbp + 0x1a8], rax
1412a4b4d: lea rax, [rbp + 0x3c8]
1412a4b54: movdqu xmm0, xmmword ptr [rax - 8]
1412a4b59: movdqu xmm1, xmmword ptr [rax + 8]
1412a4b5e: movdqa xmmword ptr [rbp + 0x360], xmm0
1412a4b66: movdqa xmmword ptr [rbp + 0x370], xmm1
1412a4b6e: mov r12, qword ptr [rbp + 0x4c8]
1412a4b75: mov byte ptr [rbp + 0x4d1], 1
1412a4b7c: lea rcx, [rbp + 0x3b0]
1412a4b83: mov rdx, qword ptr [rbp + 0x358]
1412a4b8a: call 0x140d7bd90 ; alloc::string::impl$6::clone
1412a4b8f: mov rax, qword ptr [rbp + 0x318]
1412a4b96: mov qword ptr [rbp + 0x448], rax
1412a4b9d: mov rax, qword ptr [rbp + 0x1a8]
1412a4ba4: mov qword ptr [rbp + 0x450], rax
1412a4bab: movaps xmm0, xmmword ptr [rbp + 0x360]
1412a4bb2: movaps xmm1, xmmword ptr [rbp + 0x370]
1412a4bb9: lea rcx, [rbp + 0x448]
1412a4bc0: movups xmmword ptr [rcx + 0x20], xmm1
1412a4bc4: movups xmmword ptr [rcx + 0x10], xmm0
1412a4bc8: movups xmm0, xmmword ptr [rbp + 0x3b0]
1412a4bcf: movaps xmmword ptr [rbp + 0x430], xmm0
1412a4bd6: mov rax, qword ptr [rbp + 0x3c0]
1412a4bdd: mov qword ptr [rbp + 0x440], rax
1412a4be4: mov qword ptr [rbp + 0x4b8], r12
1412a4beb: mov byte ptr [rbp + 0x4c0], sil
1412a4bf2: movups xmm0, xmmword ptr [rbp - 0x14]
1412a4bf6: movups xmmword ptr [rcx + 0x5c], xmm0
1412a4bfa: movdqa xmm0, xmmword ptr [rbp - 0x40]
1412a4bff: movdqa xmm1, xmmword ptr [rbp - 0x30]
1412a4c04: movdqa xmm2, xmmword ptr [rbp - 0x20]
1412a4c09: movdqu xmmword ptr [rcx + 0x50], xmm2
1412a4c0e: movdqu xmmword ptr [rcx + 0x40], xmm1
1412a4c13: movdqu xmmword ptr [rcx + 0x30], xmm0
1412a4c18: mov byte ptr [rbp + 0x4b4], bl
1412a4c1e: movzx eax, byte ptr [rbp + 0x356]
1412a4c25: mov byte ptr [rcx + 0x6f], al
1412a4c28: movzx eax, word ptr [rbp + 0x354]
1412a4c2f: mov word ptr [rcx + 0x6d], ax
1412a4c33: mov rsi, qword ptr [rbp + 0x240]
1412a4c3a: cmp rsi, qword ptr [rbp + 0x230]
1412a4c41: jne 0x1412a4c56
1412a4c43: lea rdx, [rip + 0x1985a16]
1412a4c4a: lea rcx, [rbp + 0x230]
1412a4c51: call 0x140562ff0 ; alloc::raw_vec::RawVec::grow_one<tiny_glade::screenshot::ActiveScreenshotRequest,alloc::alloc::Global>
1412a4c56: imul rcx, rsi, 0x98
1412a4c5d: add rcx, qword ptr [rbp + 0x238]
1412a4c64: lea rdx, [rbp + 0x430]
1412a4c6b: mov r8d, 0x98
1412a4c71: call 0x142923c10
1412a4c76: inc rsi
1412a4c79: mov qword ptr [rbp + 0x240], rsi
1412a4c80: mov rax, qword ptr [rbp + 0x2a0]
1412a4c87: lock dec qword ptr [rax]
1412a4c8b: lea rdi, [rbp + 0x208]
1412a4c92: lea rbx, [rbp + 0x2d8]
1412a4c99: mov rsi, qword ptr [rbp + 0xa0]
1412a4ca0: jne 0x1412a4cae
1412a4ca2: lea rcx, [rbp + 0x2a0]
1412a4ca9: call 0x140b5c830 ; alloc::sync::Arc::drop_slow<country_core::render::mesh::Mesh,alloc::alloc::Global>
1412a4cae: mov eax, dword ptr [rbp + 0x3a0]
1412a4cb4: jmp 0x1412a235c ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4cb9: mov r11d, 0x10
1412a4cbf: add r9, r11
1412a4cc2: and r9, r8
1412a4cc5: movdqu xmm0, xmmword ptr [rcx + r9]
1412a4ccb: pmovmskb r10d, xmm0
1412a4cd0: add r11, 0x10
1412a4cd4: test r10d, r10d
1412a4cd7: jne 0x1412a2b32
1412a4cdd: jmp 0x1412a4cbf ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4cdf: movdqa xmm0, xmmword ptr [rcx]
1412a4ce3: pmovmskb r9d, xmm0
1412a4ce8: tzcnt r10d, r9d
1412a4ced: movzx r9d, byte ptr [rcx + r10]
1412a4cf2: jmp 0x1412a2b4b ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4cf7: mov r11d, 0x10
1412a4cfd: add r9, r11
1412a4d00: and r9, r8
1412a4d03: movdqu xmm0, xmmword ptr [rcx + r9]
1412a4d09: pmovmskb r10d, xmm0
1412a4d0e: add r11, 0x10
1412a4d12: test r10d, r10d
1412a4d15: jne 0x1412a4346
1412a4d1b: jmp 0x1412a4cfd ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4d1d: movdqa xmm0, xmmword ptr [rcx]
1412a4d21: pmovmskb r9d, xmm0
1412a4d26: tzcnt r10d, r9d
1412a4d2b: movzx r9d, byte ptr [rcx + r10]
1412a4d30: jmp 0x1412a435f ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4d35: nop word ptr cs:[rax + rax]
1412a4d40: mov qword ptr [rbp + 0x220], rdx
1412a4d47: mov rax, qword ptr [r15 - 0x40]
1412a4d4b: mov rcx, qword ptr [r15 - 0x38]
1412a4d4f: lock inc qword ptr [rax]
1412a4d53: jle 0x1412a689d
1412a4d59: mov rdx, qword ptr [rbp + 0x240]
1412a4d60: lea r8, [rbp + 0x448]
1412a4d67: mov qword ptr [r8 + 8], rdx
1412a4d6b: movups xmm0, xmmword ptr [rbp + 0x230]
1412a4d72: movups xmmword ptr [r8 - 8], xmm0
1412a4d77: mov qword ptr [rbp + 0x430], rax
1412a4d7e: mov qword ptr [rbp + 0x438], rcx
1412a4d85: mov rsi, qword ptr [rbp + 0x258]
1412a4d8c: cmp rsi, qword ptr [rbp + 0x248]
1412a4d93: jne 0x1412a4da8
1412a4d95: lea rcx, [rbp + 0x248]
1412a4d9c: lea rdx, [rip + 0x198582d]
1412a4da3: call 0x140562ab0 ; alloc::raw_vec::RawVec::grow_one<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,ref$<glam::f32::vec3::Vec3>,glam::f32::vec2::Vec2,f32,country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,enum2$<country_core::resources::walls::decorator::DecoratorType> >,alloc::alloc::Global>
1412a4da8: lea rax, [rsi + 1]
1412a4dac: mov rcx, qword ptr [rbp + 0x250]
1412a4db3: lea rdx, [rsi + rsi*4]
1412a4db7: movdqu xmm0, xmmword ptr [rbp + 0x430]
1412a4dbf: movdqu xmm1, xmmword ptr [rbp + 0x440]
1412a4dc7: mov r8, qword ptr [rbp + 0x450]
1412a4dce: mov qword ptr [rcx + rdx*8 + 0x20], r8
1412a4dd3: movdqu xmmword ptr [rcx + rdx*8 + 0x10], xmm1
1412a4dd9: movdqu xmmword ptr [rcx + rdx*8], xmm0
1412a4dde: mov qword ptr [rbp + 0x258], rax
1412a4de5: cmp qword ptr [rbp + 0x300], 0
1412a4ded: je 0x1412a2225
1412a4df3: mov rdx, qword ptr [r15 - 0x40]
1412a4df7: mov r8, qword ptr [r15 - 0x38]
1412a4dfb: lock inc qword ptr [rdx]
1412a4dff: jle 0x1412a689d
1412a4e05: mov rcx, qword ptr [rbp + 0x300]
1412a4e0c: call 0x14152b320 ; hashbrown::map::HashMap::insert<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global>
1412a4e11: jmp 0x1412a2225 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4e16: mov r13, qword ptr [rbp + 0x248]
1412a4e1d: mov r12, qword ptr [rbp + 0x250]
1412a4e24: mov rax, qword ptr [rbp + 0x258]
1412a4e2b: lea rcx, [rax + rax*4]
1412a4e2f: lea rdi, [r12 + rcx*8]
1412a4e33: mov qword ptr [rbp + 0x430], r12
1412a4e3a: mov qword ptr [rbp + 0x438], r12
1412a4e41: mov qword ptr [rbp + 0x440], r13
1412a4e48: mov qword ptr [rbp + 0x448], rdi
1412a4e4f: mov r9, r12
1412a4e52: test rax, rax
1412a4e55: je 0x1412a4ecf
1412a4e57: lea rbx, [rbp - 0x50]
1412a4e5b: lea r9, [r12 + 0x28]
1412a4e60: shl rax, 3
1412a4e64: lea r14, [rax + rax*4]
1412a4e68: lea rsi, [rbp - 0x58]
1412a4e6c: mov r15, qword ptr [rbp + 0x268]
1412a4e73: nop word ptr cs:[rax + rax]
1412a4e80: mov rax, qword ptr [r9 - 0x18]
1412a4e84: mov rcx, rax
1412a4e87: neg rcx
1412a4e8a: jo 0x1412a4ec8
1412a4e8c: mov rdx, qword ptr [r9 - 0x28]
1412a4e90: mov r8, qword ptr [r9 - 0x20]
1412a4e94: mov qword ptr [rbp - 0x58], rax
1412a4e98: movdqu xmm0, xmmword ptr [r9 - 0x10]
1412a4e9e: movdqu xmmword ptr [rbx], xmm0
1412a4ea2: mov rcx, r15
1412a4ea5: mov qword ptr [rbp + 0x3a8], r9
1412a4eac: mov r9, rsi
1412a4eaf: call 0x1408e2540 ; country_core::resources::prefab::PrefabLibrary::add
1412a4eb4: mov r9, qword ptr [rbp + 0x3a8]
1412a4ebb: add r9, 0x28
1412a4ebf: add r14, -0x28
1412a4ec3: jne 0x1412a4e80
1412a4ec5: mov r9, rdi
1412a4ec8: mov qword ptr [rbp + 0x438], r9
1412a4ecf: mov qword ptr [rbp + 0x300], r13
1412a4ed6: mov qword ptr [rbp + 0x268], r12
1412a4edd: sub rdi, r9
1412a4ee0: shr rdi, 3
1412a4ee4: movabs rax, 0xcccccccccccccccd
1412a4eee: imul rax, rdi
1412a4ef2: mov qword ptr [rbp + 0x4c8], rax
1412a4ef9: mov qword ptr [rbp + 0x3a0], 0
1412a4f04: mov qword ptr [rbp + 0x3a8], r9
1412a4f0b: mov rcx, r9
1412a4f0e: mov rdi, qword ptr [rbp + 0x210]
1412a4f15: nop word ptr cs:[rax + rax]
1412a4f20: mov rax, qword ptr [rbp + 0x3a0]
1412a4f27: cmp qword ptr [rbp + 0x4c8], rax
1412a4f2e: je 0x1412a4f48
1412a4f30: inc rax
1412a4f33: mov qword ptr [rbp + 0x3a0], rax
1412a4f3a: lea rsi, [rcx + 0x28]
1412a4f3e: call 0x1408989d0 ; core::ptr::drop_in_place<tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab> >
1412a4f43: mov rcx, rsi
1412a4f46: jmp 0x1412a4f20 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4f48: mov rax, qword ptr [rbp + 0x300]
1412a4f4f: test rax, rax
1412a4f52: je 0x1412a4f6e
1412a4f54: shl rax, 3
1412a4f58: lea rdx, [rax + rax*4]
1412a4f5c: mov r8d, 8
1412a4f62: mov rcx, qword ptr [rbp + 0x268]
1412a4f69: call 0x140613c20
1412a4f6e: test rdi, rdi
1412a4f71: je 0x1412a4fe8
1412a4f73: lea rax, [rbp + 0x100]
1412a4f7a: movdqu xmm0, xmmword ptr [rax]
1412a4f7e: movdqu xmm1, xmmword ptr [rax + 0x10]
1412a4f83: movdqu xmm2, xmmword ptr [rax + 0x20]
1412a4f88: movdqa xmmword ptr [rbp + 0x380], xmm2
1412a4f90: movdqa xmmword ptr [rbp + 0x370], xmm1
1412a4f98: movdqa xmmword ptr [rbp + 0x360], xmm0
1412a4fa0: mov rax, qword ptr [rdi]
1412a4fa3: xor esi, esi
1412a4fa5: cmp rax, 3
1412a4fa9: cmovne rsi, rdi
1412a4fad: cmp eax, 3
1412a4fb0: jne 0x1412a5058
1412a4fb6: lea rdx, [rbp + 0x380]
1412a4fbd: mov byte ptr [rbp + 0x4e5], 0
1412a4fc4: mov byte ptr [rbp + 0x4e4], 0
1412a4fcb: lea rcx, [rbp + 0x360]
1412a4fd2: mov r8d, 0x38
1412a4fd8: mov r9d, 0x10
1412a4fde: call 0x140b74970 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global> >,alloc::alloc::Global>
1412a4fe3: jmp 0x1412a534c ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4fe8: mov byte ptr [rbp + 0x4dc], 1
1412a4fef: mov byte ptr [rbp + 0x4db], 0
1412a4ff6: lea rcx, [rbp + 0x160]
1412a4ffd: mov r8d, 0x38
1412a5003: mov r9d, 0x10
1412a5009: lea rdx, [rbp + 0x180]
1412a5010: call 0x140b73b80 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >,alloc::alloc::Global>
1412a5015: lea rdx, [rbp + 0xf0]
1412a501c: lea rcx, [rbp + 0xd0]
1412a5023: mov r8d, 0x40
1412a5029: mov r9d, 0x10
1412a502f: call 0x140b75590 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,alloc::alloc::Global>
1412a5034: lea rdx, [rbp + 0x120]
1412a503b: mov r8d, 0x38
1412a5041: mov r9d, 0x10
1412a5047: lea rcx, [rbp + 0x100]
1412a504e: call 0x140b74970 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global> >,alloc::alloc::Global>
1412a5053: jmp 0x1412a5398 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5058: mov r12, qword ptr [rbp + 0x100]
1412a505f: mov r8, qword ptr [rbp + 0x108]
1412a5066: mov r13, qword ptr [rbp + 0x118]
1412a506d: lea rcx, [r8 + 1]
1412a5071: movdqa xmm0, xmmword ptr [r12]
1412a5077: test r8, r8
1412a507a: je 0x1412a50a5
1412a507c: mov edx, 0x38
1412a5081: mov rax, rcx
1412a5084: mul rdx
1412a5087: add rax, 0xf
1412a508b: and rax, 0xfffffffffffffff0
1412a508f: lea rdx, [r8 + rax]
1412a5093: add rdx, 0x11
1412a5097: mov r8, r12
1412a509a: sub r8, rax
1412a509d: mov r9d, 0x10
1412a50a3: jmp 0x1412a50a8 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a50a5: xor r9d, r9d
1412a50a8: lea rdi, [r12 + 0x10]
1412a50ad: pmovmskb eax, xmm0
1412a50b1: not eax
1412a50b3: add rcx, r12
1412a50b6: mov qword ptr [rbp + 0x448], r12
1412a50bd: mov qword ptr [rbp + 0x450], rdi
1412a50c4: mov qword ptr [rbp + 0x458], rcx
1412a50cb: mov word ptr [rbp + 0x460], ax
1412a50d2: mov qword ptr [rbp + 0x468], r13
1412a50d9: mov qword ptr [rbp + 0x3a0], r9
1412a50e0: mov qword ptr [rbp + 0x430], r9
1412a50e7: mov qword ptr [rbp + 0x438], rdx
1412a50ee: mov qword ptr [rbp + 0x440], r8
1412a50f5: lea r14, [rbp + 0x2c8]
1412a50fc: lea r15, [rbp + 0x320]
1412a5103: jmp 0x1412a5112 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5105: nop word ptr cs:[rax + rax]
1412a5110: mov eax, ebx
1412a5112: sub r13, 1
1412a5116: jb 0x1412a5264
1412a511c: test ax, ax
1412a511f: je 0x1412a5130
1412a5121: lea ebx, [rax - 1]
1412a5124: and ebx, eax
1412a5126: jmp 0x1412a5165 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5128: nop dword ptr [rax + rax]
1412a5130: movdqa xmm0, xmmword ptr [rdi]
1412a5134: pmovmskb eax, xmm0
1412a5138: add r12, -0x380
1412a513f: add rdi, 0x10
1412a5143: cmp eax, 0xffff
1412a5148: je 0x1412a5130
1412a514a: mov qword ptr [rbp + 0x450], rdi
1412a5151: mov qword ptr [rbp + 0x448], r12
1412a5158: mov ecx, 0xfffffffe
1412a515d: sub ecx, eax
1412a515f: not eax
1412a5161: mov ebx, eax
1412a5163: and ebx, ecx
1412a5165: mov word ptr [rbp + 0x460], bx
1412a516c: mov qword ptr [rbp + 0x468], r13
1412a5173: tzcnt eax, eax
1412a5177: neg rax
1412a517a: imul rcx, rax, 0x38
1412a517e: mov rax, qword ptr [r12 + rcx - 0x38]
1412a5183: movups xmm0, xmmword ptr [r12 + rcx - 0x10]
1412a5189: movaps xmmword ptr [rbp + 0x3d0], xmm0
1412a5190: movups xmm0, xmmword ptr [r12 + rcx - 0x20]
1412a5196: movaps xmmword ptr [rbp + 0x3c0], xmm0
1412a519d: movups xmm0, xmmword ptr [r12 + rcx - 0x30]
1412a51a3: movaps xmmword ptr [rbp + 0x3b0], xmm0
1412a51aa: mov rcx, rax
1412a51ad: neg rcx
1412a51b0: jo 0x1412a5270
1412a51b6: mov qword ptr [rbp + 0x2c0], rax
1412a51bd: mov rax, qword ptr [rbp + 0x3c0]
1412a51c4: mov qword ptr [r14 + 0x10], rax
1412a51c8: movaps xmm0, xmmword ptr [rbp + 0x3b0]
1412a51cf: movups xmmword ptr [r14], xmm0
1412a51d3: mov rax, qword ptr [rbp + 0x210]
1412a51da: mov rax, qword ptr [rax + 0x30]
1412a51de: lock inc qword ptr [rax]
1412a51e2: jle 0x1412a689d
1412a51e8: mov qword ptr [rbp + 0x338], rax
1412a51ef: lea rax, [rbp + 0x3c8]
1412a51f6: movdqu xmm0, xmmword ptr [rax]
1412a51fa: movdqa xmmword ptr [rbp + 0x320], xmm0
1412a5202: mov rax, qword ptr [rax + 0x10]
1412a5206: mov qword ptr [rbp + 0x330], rax
1412a520d: lea rcx, [rbp + 0x1e0]
1412a5214: mov rdx, rsi
1412a5217: lea r8, [rbp + 0x2c0]
1412a521e: mov r9, r15
1412a5221: call 0x14091d060 ; hotwatch::GenericHotwatch::watch<notify::windows::ReadDirectoryChangesWatcher,ref$<std::path::PathBuf>,country_core::startup::load_prefabs::impl$1::watch_prefab::closure_env$0>
1412a5226: movabs rax, 0x8000000000000004
1412a5230: cmp qword ptr [rbp + 0x1e0], rax
1412a5237: jne 0x1412a57c8
1412a523d: mov rdx, qword ptr [rbp + 0x2c0]
1412a5244: test rdx, rdx
1412a5247: je 0x1412a5110
1412a524d: mov rcx, qword ptr [rbp + 0x2c8]
1412a5254: mov r8d, 1
1412a525a: call 0x140613c20
1412a525f: jmp 0x1412a5110 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5264: mov r8, qword ptr [rbp + 0x3a0]
1412a526b: jmp 0x1412a532f ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5270: test r13, r13
1412a5273: mov r8, qword ptr [rbp + 0x3a0]
1412a527a: je 0x1412a532f
1412a5280: test bx, bx
1412a5283: je 0x1412a5290
1412a5285: lea r14d, [rbx - 1]
1412a5289: and r14d, ebx
1412a528c: jmp 0x1412a52c8 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a528e: nop 
1412a5290: movdqa xmm0, xmmword ptr [rdi]
1412a5294: pmovmskb ebx, xmm0
1412a5298: add r12, -0x380
1412a529f: add rdi, 0x10
1412a52a3: cmp ebx, 0xffff
1412a52a9: je 0x1412a5290
1412a52ab: mov qword ptr [rbp + 0x450], rdi
1412a52b2: mov qword ptr [rbp + 0x448], r12
1412a52b9: mov eax, 0xfffffffe
1412a52be: sub eax, ebx
1412a52c0: not ebx
1412a52c2: mov r14d, ebx
1412a52c5: and r14d, eax
1412a52c8: mov word ptr [rbp + 0x460], r14w
1412a52d0: dec r13
1412a52d3: mov qword ptr [rbp + 0x468], r13
1412a52da: tzcnt eax, ebx
1412a52de: neg rax
1412a52e1: imul rax, rax, 0x38
1412a52e5: lea rsi, [r12 + rax]
1412a52e9: mov rdx, qword ptr [r12 + rax - 0x38]
1412a52ee: test rdx, rdx
1412a52f1: je 0x1412a5302
1412a52f3: mov rcx, qword ptr [rsi - 0x30]
1412a52f7: mov r8d, 1
1412a52fd: call 0x140613c20
1412a5302: add rsi, -0x18
1412a5306: mov byte ptr [rbp + 0x4e5], 0
1412a530d: mov byte ptr [rbp + 0x4e4], 0
1412a5314: mov rcx, rsi
1412a5317: call 0x14088d2c0 ; core::ptr::drop_in_place<alloc::vec::Vec<regex::dfa::State,alloc::alloc::Global> >
1412a531c: mov ebx, r14d
1412a531f: test r13, r13
1412a5322: jne 0x1412a5280
1412a5328: mov r8, qword ptr [rbp + 0x430]
1412a532f: test r8, r8
1412a5332: je 0x1412a534c
1412a5334: mov rdx, qword ptr [rbp + 0x438]
1412a533b: test rdx, rdx
1412a533e: je 0x1412a534c
1412a5340: mov rcx, qword ptr [rbp + 0x440]
1412a5347: call 0x140613c20
1412a534c: mov byte ptr [rbp + 0x4dc], 0
1412a5353: mov byte ptr [rbp + 0x4db], 0
1412a535a: lea rcx, [rbp + 0x160]
1412a5361: mov r8d, 0x38
1412a5367: mov r9d, 0x10
1412a536d: lea rdx, [rbp + 0x180]
1412a5374: call 0x140b73b80 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >,alloc::alloc::Global>
1412a5379: lea rdx, [rbp + 0xf0]
1412a5380: lea rcx, [rbp + 0xd0]
1412a5387: mov r8d, 0x40
1412a538d: mov r9d, 0x10
1412a5393: call 0x140b75590 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,alloc::alloc::Global>
1412a5398: lea rdx, [rbp + 0x150]
1412a539f: mov esi, dword ptr [rbp + 0x2f4]
1412a53a5: mov byte ptr [rbp + 0x4e6], sil
1412a53ac: lea rcx, [rbp + 0x130]
1412a53b3: mov r8d, 0x10
1412a53b9: mov r9d, 0x10
1412a53bf: call 0x140b73f00 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<asset::prefab::PrefabName,tuple$<> >,alloc::alloc::Global>
1412a53c4: test sil, sil
1412a53c7: je 0x1412a53e2
1412a53c9: lea rcx, [rbp + 0x40]
1412a53cd: mov r8d, 0x10
1412a53d3: mov r9d, 0x10
1412a53d9: lea rdx, [rbp + 0x60]
1412a53dd: call 0x140b73f00 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<asset::prefab::PrefabName,tuple$<> >,alloc::alloc::Global>
1412a53e2: mov rdx, qword ptr [rbp + 0x20]
1412a53e6: test rdx, rdx
1412a53e9: je 0x1412a53fa
1412a53eb: mov rcx, qword ptr [rbp + 0x28]
1412a53ef: mov r8d, 1
1412a53f5: call 0x140613c20
1412a53fa: movaps xmm6, xmmword ptr [rbp + 0x4f0]
1412a5401: movaps xmm7, xmmword ptr [rbp + 0x500]
1412a5408: movaps xmm8, xmmword ptr [rbp + 0x510]
1412a5410: movaps xmm9, xmmword ptr [rbp + 0x520]
1412a5418: movaps xmm10, xmmword ptr [rbp + 0x530]
1412a5420: movaps xmm11, xmmword ptr [rbp + 0x540]
1412a5428: movaps xmm12, xmmword ptr [rbp + 0x550]
1412a5430: movaps xmm13, xmmword ptr [rbp + 0x560]
1412a5438: add rsp, 0x5f8
1412a543f: pop rbx
1412a5440: pop rdi
1412a5441: pop rsi
1412a5442: pop r12
1412a5444: pop r13
1412a5446: pop r14
1412a5448: pop r15
1412a544a: pop rbp
1412a544b: ret 
1412a544c: lea rcx, [rip + 0x1984a95]
1412a5453: lea r8, [rip + 0x1985506]
1412a545a: mov edx, 0x16
1412a545f: call 0x1428d9330 ; core::option::expect_failed
1412a5464: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5469: lea rcx, [rip + 0x1984a78]
1412a5470: lea r8, [rip + 0x1985511]
1412a5477: mov edx, 0x16
1412a547c: call 0x1428d9330 ; core::option::expect_failed
1412a5481: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5486: mov byte ptr [rbp + 0x4e6], 1
1412a548d: mov rcx, rbx
1412a5490: call 0x1428d8750 ; std::sys::sync::mutex::futex::Mutex::lock_contended
1412a5495: jmp 0x1412a1edc ; country_core::startup::load_prefabs::load_prefabs_impl
1412a549a: mov byte ptr [rbp + 0x4e6], 1
1412a54a1: call 0x1428d86b0 ; std::panicking::panic_count::is_zero_slow_path
1412a54a6: xor al, 1
1412a54a8: mov dword ptr [rbp + 0x4c8], eax
1412a54ae: mov qword ptr [rbp + 0x3a0], rbx
1412a54b5: movzx eax, byte ptr [rsi + 0x11]
1412a54b9: test al, al
1412a54bb: je 0x1412a1f0f
1412a54c1: mov rax, qword ptr [rbp + 0x3a0]
1412a54c8: mov qword ptr [rbp + 0x430], rax
1412a54cf: mov eax, dword ptr [rbp + 0x4c8]
1412a54d5: mov byte ptr [rbp + 0x438], al
1412a54db: lea rax, [rip + 0x1985196]
1412a54e2: mov qword ptr [rsp + 0x20], rax
1412a54e7: lea rcx, [rip + 0x1984b2a]
1412a54ee: lea r9, [rip + 0x1984b73]
1412a54f5: lea r8, [rbp + 0x430]
1412a54fc: mov edx, 0x2b
1412a5501: call 0x1428d9760 ; core::result::unwrap_failed
1412a5506: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a550b: mov byte ptr [rbp + 0x4e6], 1
1412a5512: call 0x1428d86b0 ; std::panicking::panic_count::is_zero_slow_path
1412a5517: test al, al
1412a5519: jne 0x1412a1f77
1412a551f: mov byte ptr [rsi + 0x11], 1
1412a5523: jmp 0x1412a1f77 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5528: mov qword ptr [rbp + 0x220], rdx
1412a552f: movzx eax, al
1412a5532: test eax, eax
1412a5534: jne 0x1412a557e
1412a5536: lea rax, [rip + 0x1832343]
1412a553d: mov qword ptr [rbp + 0x430], rax
1412a5544: mov qword ptr [rbp + 0x438], 1
1412a554f: mov qword ptr [rbp + 0x440], 8
1412a555a: pxor xmm0, xmm0
1412a555e: movdqu xmmword ptr [rbp + 0x448], xmm0
1412a5566: lea rdx, [rip + 0x198507b]
1412a556d: lea rcx, [rbp + 0x430]
1412a5574: call 0x1428d9390 ; core::panicking::panic_fmt
1412a5579: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a557e: lea rax, [rip + 0x17f8ad4]
1412a5585: mov qword ptr [rbp + 0x3b0], rax
1412a558c: mov qword ptr [rbp + 0x3b8], 4
1412a5597: lea rax, [rbp + 0x3b0]
1412a559e: mov qword ptr [rbp + 0x360], rax
1412a55a5: lea rax, [rip - 0x119b71c]
1412a55ac: mov qword ptr [rbp + 0x368], rax
1412a55b3: lea rax, [rip + 0x183230e]
1412a55ba: mov qword ptr [rbp + 0x430], rax
1412a55c1: mov qword ptr [rbp + 0x438], 2
1412a55cc: mov qword ptr [rbp + 0x450], 0
1412a55d7: lea rax, [rbp + 0x360]
1412a55de: mov qword ptr [rbp + 0x440], rax
1412a55e5: mov qword ptr [rbp + 0x448], 1
1412a55f0: lea rdx, [rip + 0x1984ff1]
1412a55f7: lea rcx, [rbp + 0x430]
1412a55fe: call 0x1428d9390 ; core::panicking::panic_fmt
1412a5603: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5608: lea rax, [rip + 0x1821a09]
1412a560f: mov qword ptr [rbp + 0x3b0], rax
1412a5616: mov qword ptr [rbp + 0x3b8], 1
1412a5621: mov qword ptr [rbp + 0x3c0], 8
1412a562c: pxor xmm0, xmm0
1412a5630: movdqu xmmword ptr [rbp + 0x3c8], xmm0
1412a5638: lea rdx, [rip + 0x1821a01]
1412a563f: lea rcx, [rbp + 0x3b0]
1412a5646: call 0x1428d9390 ; core::panicking::panic_fmt
1412a564b: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5650: lea rax, [rip + 0x18219c1]
1412a5657: mov qword ptr [rbp + 0x3b0], rax
1412a565e: mov qword ptr [rbp + 0x3b8], 1
1412a5669: mov qword ptr [rbp + 0x3c0], 8
1412a5674: pxor xmm0, xmm0
1412a5678: movdqu xmmword ptr [rbp + 0x3c8], xmm0
1412a5680: lea rdx, [rip + 0x18219b9]
1412a5687: lea rcx, [rbp + 0x3b0]
1412a568e: call 0x1428d9390 ; core::panicking::panic_fmt
1412a5693: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5698: lea rax, [rip + 0x1821979]
1412a569f: mov qword ptr [rbp + 0x3b0], rax
1412a56a6: mov qword ptr [rbp + 0x3b8], 1
1412a56b1: mov qword ptr [rbp + 0x3c0], 8
1412a56bc: pxor xmm0, xmm0
1412a56c0: movdqu xmmword ptr [rbp + 0x3c8], xmm0
1412a56c8: lea rdx, [rip + 0x1821971]
1412a56cf: lea rcx, [rbp + 0x3b0]
1412a56d6: call 0x1428d9390 ; core::panicking::panic_fmt
1412a56db: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a56e0: lea rax, [rip + 0x1821931]
1412a56e7: mov qword ptr [rbp + 0x3b0], rax
1412a56ee: mov qword ptr [rbp + 0x3b8], 1
1412a56f9: mov qword ptr [rbp + 0x3c0], 8
1412a5704: pxor xmm0, xmm0
1412a5708: movdqu xmmword ptr [rbp + 0x3c8], xmm0
1412a5710: mov byte ptr [rbp + 0x4dd], 1
1412a5717: lea rdx, [rip + 0x1821922]
1412a571e: lea rcx, [rbp + 0x3b0]
1412a5725: call 0x1428d9390 ; core::panicking::panic_fmt
1412a572a: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a572f: mov qword ptr [rbp + 0x3b0], 0
1412a573a: mov byte ptr [rbp + 0x4dd], 1
1412a5741: lea rax, [rip + 0x1985270]
1412a5748: mov qword ptr [rsp + 0x20], rax
1412a574d: lea rdx, [rbp + 0x78]
1412a5751: lea r8, [rbp + 0x320]
1412a5758: lea r9, [rbp + 0x3b0]
1412a575f: xor ecx, ecx
1412a5761: call 0x1428d9596 ; core::panicking::assert_failed<usize,usize>
1412a5766: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a576b: mov byte ptr [rbp + 0x4dd], 1
1412a5772: lea r8, [rip + 0x19852af]
1412a5779: call 0x1428d9518 ; core::panicking::panic_bounds_check
1412a577e: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5783: mov byte ptr [rbp + 0x4da], 1
1412a578a: mov byte ptr [rbp + 0x4d9], 1
1412a5791: lea r8, [rip + 0x1985278]
1412a5798: mov rdx, qword ptr [rbp + 0x4c8]
1412a579f: call 0x1428d9518 ; core::panicking::panic_bounds_check
1412a57a4: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a57a9: mov byte ptr [rbp + 0x4da], 1
1412a57b0: mov byte ptr [rbp + 0x4d9], 1
1412a57b7: lea r8, [rip + 0x198523a]
1412a57be: call 0x1428d9518 ; core::panicking::panic_bounds_check
1412a57c3: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a57c8: mov rax, qword ptr [rbp + 0x1f0]
1412a57cf: mov qword ptr [rbp + 0x330], rax
1412a57d6: movdqu xmm0, xmmword ptr [rbp + 0x1e0]
1412a57de: movdqa xmmword ptr [rbp + 0x320], xmm0
1412a57e6: lea rax, [rip + 0x19852c3]
1412a57ed: mov qword ptr [rsp + 0x20], rax
1412a57f2: lea rcx, [rip + 0x1985297]
1412a57f9: lea r9, [rip + 0x19847d8]
1412a5800: lea r8, [rbp + 0x320]
1412a5807: mov edx, 0x1e
1412a580c: call 0x1428d9760 ; core::result::unwrap_failed
1412a5811: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5816: lea rax, [rbp + 0x98]
1412a581d: mov qword ptr [rbp + 0x3b0], rax
1412a5824: lea rax, [rip - 0x1251f4b]
1412a582b: mov qword ptr [rbp + 0x3b8], rax
1412a5832: lea rax, [rip + 0x1984e87]
1412a5839: mov qword ptr [rbp + 0x430], rax
1412a5840: mov qword ptr [rbp + 0x438], 1
1412a584b: mov qword ptr [rbp + 0x450], 0
1412a5856: lea rax, [rbp + 0x3b0]
1412a585d: mov qword ptr [rbp + 0x440], rax
1412a5864: mov qword ptr [rbp + 0x448], 1
1412a586f: lea rdx, [rip + 0x1984e5a]
1412a5876: lea rcx, [rbp + 0x430]
1412a587d: call 0x1428d9390 ; core::panicking::panic_fmt
1412a5882: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5887: cmp rcx, 1
1412a588b: jne 0x1412a5c62
1412a5891: lea rax, [rip + 0x17f8310]
1412a5898: mov qword ptr [rbp + 0x3b0], rax
1412a589f: mov qword ptr [rbp + 0x3b8], 0x29
1412a58aa: lea rax, [rbp + 0x3b0]
1412a58b1: mov qword ptr [rbp + 0x360], rax
1412a58b8: lea rax, [rip - 0x119ba2f]
1412a58bf: mov qword ptr [rbp + 0x368], rax
1412a58c6: lea rax, [rip + 0x1831ffb]
1412a58cd: mov qword ptr [rbp + 0x430], rax
1412a58d4: mov qword ptr [rbp + 0x438], 2
1412a58df: mov qword ptr [rbp + 0x450], 0
1412a58ea: lea rax, [rbp + 0x360]
1412a58f1: mov qword ptr [rbp + 0x440], rax
1412a58f8: mov qword ptr [rbp + 0x448], 1
1412a5903: lea rdx, [rip + 0x1984cf6]
1412a590a: lea rcx, [rbp + 0x430]
1412a5911: call 0x1428d9390 ; core::panicking::panic_fmt
1412a5916: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a591b: cmp rax, 1
1412a591f: jne 0x1412a5caa
1412a5925: lea rax, [rip + 0x17f91b0]
1412a592c: mov qword ptr [rbp + 0x3b0], rax
1412a5933: mov qword ptr [rbp + 0x3b8], 0x15
1412a593e: lea rax, [rbp + 0x3b0]
1412a5945: mov qword ptr [rbp + 0x360], rax
1412a594c: lea rax, [rip - 0x119bac3]
1412a5953: mov qword ptr [rbp + 0x368], rax
1412a595a: lea rax, [rip + 0x1831f67]
1412a5961: mov qword ptr [rbp + 0x430], rax
1412a5968: mov qword ptr [rbp + 0x438], 2
1412a5973: mov qword ptr [rbp + 0x450], 0
1412a597e: lea rax, [rbp + 0x360]
1412a5985: mov qword ptr [rbp + 0x440], rax
1412a598c: mov qword ptr [rbp + 0x448], 1
1412a5997: lea rdx, [rip + 0x1984e3a]
1412a599e: lea rcx, [rbp + 0x430]
1412a59a5: call 0x1428d9390 ; core::panicking::panic_fmt
1412a59aa: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a59af: cmp rcx, 1
1412a59b3: jne 0x1412a5cf2
1412a59b9: lea rax, [rip + 0x17f79f8]
1412a59c0: mov qword ptr [rbp + 0x3b0], rax
1412a59c7: mov qword ptr [rbp + 0x3b8], 0x40
1412a59d2: lea rax, [rbp + 0x3b0]
1412a59d9: mov qword ptr [rbp + 0x360], rax
1412a59e0: lea rax, [rip - 0x119bb57]
1412a59e7: mov qword ptr [rbp + 0x368], rax
1412a59ee: lea rax, [rip + 0x1831ed3]
1412a59f5: mov qword ptr [rbp + 0x430], rax
1412a59fc: mov qword ptr [rbp + 0x438], 2
1412a5a07: mov qword ptr [rbp + 0x450], 0
1412a5a12: lea rax, [rbp + 0x360]
1412a5a19: mov qword ptr [rbp + 0x440], rax
1412a5a20: mov qword ptr [rbp + 0x448], 1
1412a5a2b: mov byte ptr [rbp + 0x4e1], 1
1412a5a32: lea rdx, [rip + 0x1984db7]
1412a5a39: lea rcx, [rbp + 0x430]
1412a5a40: call 0x1428d9390 ; core::panicking::panic_fmt
1412a5a45: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5a4a: cmp rax, 1
1412a5a4e: jne 0x1412a6327
1412a5a54: lea rax, [rip + 0x17f9081]
1412a5a5b: mov qword ptr [rbp + 0x3b0], rax
1412a5a62: mov qword ptr [rbp + 0x3b8], 0x15
1412a5a6d: lea rax, [rbp + 0x3b0]
1412a5a74: mov qword ptr [rbp + 0x360], rax
1412a5a7b: lea rax, [rip - 0x119bbf2]
1412a5a82: mov qword ptr [rbp + 0x368], rax
1412a5a89: lea rax, [rip + 0x1831e38]
1412a5a90: mov qword ptr [rbp + 0x430], rax
1412a5a97: mov qword ptr [rbp + 0x438], 2
1412a5aa2: mov qword ptr [rbp + 0x450], 0
1412a5aad: lea rax, [rbp + 0x360]
1412a5ab4: mov qword ptr [rbp + 0x440], rax
1412a5abb: mov qword ptr [rbp + 0x448], 1
1412a5ac6: lea rdx, [rip + 0x1984b4b]
1412a5acd: lea rcx, [rbp + 0x430]
1412a5ad4: call 0x1428d9390 ; core::panicking::panic_fmt
1412a5ad9: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5ade: cmp ecx, 1
1412a5ae1: jne 0x1412a636f
1412a5ae7: lea rax, [rip + 0x17f7998]
1412a5aee: mov qword ptr [rbp + 0x3b0], rax
1412a5af5: mov qword ptr [rbp + 0x3b8], 0x3b
1412a5b00: lea rax, [rbp + 0x3b0]
1412a5b07: mov qword ptr [rbp + 0x360], rax
1412a5b0e: lea rax, [rip - 0x119bc85]
1412a5b15: mov qword ptr [rbp + 0x368], rax
1412a5b1c: lea rax, [rip + 0x1831da5]
1412a5b23: mov qword ptr [rbp + 0x430], rax
1412a5b2a: mov qword ptr [rbp + 0x438], 2
1412a5b35: mov qword ptr [rbp + 0x450], 0
1412a5b40: lea rax, [rbp + 0x360]
1412a5b47: mov qword ptr [rbp + 0x440], rax
1412a5b4e: mov qword ptr [rbp + 0x448], 1
1412a5b59: mov byte ptr [rbp + 0x4e7], 1
1412a5b60: lea rdx, [rip + 0x1984ac9]
1412a5b67: lea rcx, [rbp + 0x430]
1412a5b6e: call 0x1428d9390 ; core::panicking::panic_fmt
1412a5b73: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5b78: cmp rcx, 1
1412a5b7c: jne 0x1412a63be
1412a5b82: lea rax, [rip + 0x17f8b5f]
1412a5b89: mov qword ptr [rbp + 0x3b0], rax
1412a5b90: mov qword ptr [rbp + 0x3b8], 0x3e
1412a5b9b: lea rax, [rbp + 0x3b0]
1412a5ba2: mov qword ptr [rbp + 0x360], rax
1412a5ba9: lea rax, [rip - 0x119bd20]
1412a5bb0: mov qword ptr [rbp + 0x368], rax
1412a5bb7: lea rax, [rip + 0x1831d0a]
1412a5bbe: mov qword ptr [rbp + 0x430], rax
1412a5bc5: mov qword ptr [rbp + 0x438], 2
1412a5bd0: mov qword ptr [rbp + 0x450], 0
1412a5bdb: lea rax, [rbp + 0x360]
1412a5be2: mov qword ptr [rbp + 0x440], rax
1412a5be9: mov qword ptr [rbp + 0x448], 1
1412a5bf4: mov byte ptr [rbp + 0x4e7], 1
1412a5bfb: lea rdx, [rip + 0x1984a46]
1412a5c02: lea rcx, [rbp + 0x430]
1412a5c09: call 0x1428d9390 ; core::panicking::panic_fmt
1412a5c0e: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5c13: lea rcx, [rip + 0x1994e86]
1412a5c1a: call 0x1428d80d0 ; std::thread::local::panic_access_error
1412a5c1f: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5c24: mov rax, qword ptr [rbp + 0x438]
1412a5c2b: mov qword ptr [rbp + 0x3b0], rax
1412a5c32: lea rax, [rip + 0x1984967]
1412a5c39: mov qword ptr [rsp + 0x20], rax
1412a5c3e: lea rcx, [rip + 0x19843d3]
1412a5c45: lea r9, [rip + 0x19843ac]
1412a5c4c: lea r8, [rbp + 0x3b0]
1412a5c53: mov edx, 0x2b
1412a5c58: call 0x1428d9760 ; core::result::unwrap_failed
1412a5c5d: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5c62: lea rax, [rip + 0x1831c17]
1412a5c69: mov qword ptr [rbp + 0x430], rax
1412a5c70: mov qword ptr [rbp + 0x438], 1
1412a5c7b: mov qword ptr [rbp + 0x440], 8
1412a5c86: pxor xmm0, xmm0
1412a5c8a: movdqu xmmword ptr [rbp + 0x448], xmm0
1412a5c92: lea rdx, [rip + 0x1984967]
1412a5c99: lea rcx, [rbp + 0x430]
1412a5ca0: call 0x1428d9390 ; core::panicking::panic_fmt
1412a5ca5: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5caa: lea rax, [rip + 0x1831bcf]
1412a5cb1: mov qword ptr [rbp + 0x430], rax
1412a5cb8: mov qword ptr [rbp + 0x438], 1
1412a5cc3: mov qword ptr [rbp + 0x440], 8
1412a5cce: pxor xmm0, xmm0
1412a5cd2: movdqu xmmword ptr [rbp + 0x448], xmm0
1412a5cda: lea rdx, [rip + 0x1984af7]
1412a5ce1: lea rcx, [rbp + 0x430]
1412a5ce8: call 0x1428d9390 ; core::panicking::panic_fmt
1412a5ced: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5cf2: lea rax, [rip + 0x1831b87]
1412a5cf9: mov qword ptr [rbp + 0x430], rax
1412a5d00: mov qword ptr [rbp + 0x438], 1
1412a5d0b: mov qword ptr [rbp + 0x440], 8
1412a5d16: pxor xmm0, xmm0
1412a5d1a: movdqu xmmword ptr [rbp + 0x448], xmm0
1412a5d22: mov byte ptr [rbp + 0x4e1], 1
1412a5d29: lea rdx, [rip + 0x1984ac0]
1412a5d30: lea rcx, [rbp + 0x430]
1412a5d37: call 0x1428d9390 ; core::panicking::panic_fmt
1412a5d3c: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5d41: cmp eax, 1
1412a5d44: jne 0x1412a641e
1412a5d4a: lea rax, [rip + 0x17f8faa]
1412a5d51: mov qword ptr [rbp + 0x360], rax
1412a5d58: mov qword ptr [rbp + 0x368], 0x23
1412a5d63: lea rax, [rbp + 0x360]
1412a5d6a: mov qword ptr [rbp + 0x320], rax
1412a5d71: lea rax, [rip - 0x119bee8]
1412a5d78: mov qword ptr [rbp + 0x328], rax
1412a5d7f: lea rax, [rip + 0x1831b42]
1412a5d86: mov qword ptr [rbp + 0x3b0], rax
1412a5d8d: mov qword ptr [rbp + 0x3b8], 2
1412a5d98: mov qword ptr [rbp + 0x3d0], 0
1412a5da3: lea rax, [rbp + 0x320]
1412a5daa: mov qword ptr [rbp + 0x3c0], rax
1412a5db1: mov qword ptr [rbp + 0x3c8], 1
1412a5dbc: mov byte ptr [rbp + 0x4e7], 1
1412a5dc3: lea rdx, [rip + 0x198491e]
1412a5dca: lea rcx, [rbp + 0x3b0]
1412a5dd1: call 0x1428d9390 ; core::panicking::panic_fmt
1412a5dd6: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5ddb: cmp eax, 1
1412a5dde: jne 0x1412a646d
1412a5de4: lea rax, [rip + 0x17f8fb9]
1412a5deb: mov qword ptr [rbp + 0x320], rax
1412a5df2: mov qword ptr [rbp + 0x328], 8
1412a5dfd: lea rax, [rbp + 0x320]
1412a5e04: mov qword ptr [rbp + 0x2c0], rax
1412a5e0b: lea rax, [rip - 0x119bf82]
1412a5e12: mov qword ptr [rbp + 0x2c8], rax
1412a5e19: lea rax, [rip + 0x1831aa8]
1412a5e20: mov qword ptr [rbp + 0x3b0], rax
1412a5e27: mov qword ptr [rbp + 0x3b8], 2
1412a5e32: mov qword ptr [rbp + 0x3d0], 0
1412a5e3d: lea rax, [rbp + 0x2c0]
1412a5e44: mov qword ptr [rbp + 0x3c0], rax
1412a5e4b: mov qword ptr [rbp + 0x3c8], 1
1412a5e56: mov byte ptr [rbp + 0x4e7], 1
1412a5e5d: lea rdx, [rip + 0x198489c]
1412a5e64: lea rcx, [rbp + 0x3b0]
1412a5e6b: call 0x1428d9390 ; core::panicking::panic_fmt
1412a5e70: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5e75: cmp eax, 1
1412a5e78: jne 0x1412a64bc
1412a5e7e: lea rax, [rip + 0x17f8f1f]
1412a5e85: mov qword ptr [rbp + 0x360], rax
1412a5e8c: mov qword ptr [rbp + 0x368], 8
1412a5e97: mov qword ptr [rbp + 0x320], rdi
1412a5e9e: lea rax, [rip - 0x119c015]
1412a5ea5: mov qword ptr [rbp + 0x328], rax
1412a5eac: lea rax, [rip + 0x1831a15]
1412a5eb3: mov qword ptr [rbp + 0x3b0], rax
1412a5eba: mov qword ptr [rbp + 0x3b8], 2
1412a5ec5: mov qword ptr [rbp + 0x3d0], 0
1412a5ed0: lea rax, [rbp + 0x320]
1412a5ed7: mov qword ptr [rbp + 0x3c0], rax
1412a5ede: mov qword ptr [rbp + 0x3c8], 1
1412a5ee9: mov byte ptr [rbp + 0x4e7], 1
1412a5ef0: lea rdx, [rip + 0x1984821]
1412a5ef7: lea rcx, [rbp + 0x3b0]
1412a5efe: call 0x1428d9390 ; core::panicking::panic_fmt
1412a5f03: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5f08: cmp eax, 1
1412a5f0b: jne 0x1412a650b
1412a5f11: lea rax, [rip + 0x17f8e8c]
1412a5f18: mov qword ptr [rbp + 0x360], rax
1412a5f1f: mov qword ptr [rbp + 0x368], 8
1412a5f2a: mov qword ptr [rbp + 0x320], rdi
1412a5f31: lea rax, [rip - 0x119c0a8]
1412a5f38: mov qword ptr [rbp + 0x328], rax
1412a5f3f: lea rax, [rip + 0x1831982]
1412a5f46: mov qword ptr [rbp + 0x3b0], rax
1412a5f4d: mov qword ptr [rbp + 0x3b8], 2
1412a5f58: mov qword ptr [rbp + 0x3d0], 0
1412a5f63: lea rax, [rbp + 0x320]
1412a5f6a: mov qword ptr [rbp + 0x3c0], rax
1412a5f71: mov qword ptr [rbp + 0x3c8], 1
1412a5f7c: mov byte ptr [rbp + 0x4e7], 1
1412a5f83: lea rdx, [rip + 0x19847a6]
1412a5f8a: lea rcx, [rbp + 0x3b0]
1412a5f91: call 0x1428d9390 ; core::panicking::panic_fmt
1412a5f96: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a5f9b: cmp eax, 1
1412a5f9e: jne 0x1412a655a
1412a5fa4: lea rax, [rip + 0x17f8df9]
1412a5fab: mov qword ptr [rbp + 0x360], rax
1412a5fb2: mov qword ptr [rbp + 0x368], 8
1412a5fbd: mov qword ptr [rbp + 0x320], rdi
1412a5fc4: lea rax, [rip - 0x119c13b]
1412a5fcb: mov qword ptr [rbp + 0x328], rax
1412a5fd2: lea rax, [rip + 0x18318ef]
1412a5fd9: mov qword ptr [rbp + 0x3b0], rax
1412a5fe0: mov qword ptr [rbp + 0x3b8], 2
1412a5feb: mov qword ptr [rbp + 0x3d0], 0
1412a5ff6: lea rax, [rbp + 0x320]
1412a5ffd: mov qword ptr [rbp + 0x3c0], rax
1412a6004: mov qword ptr [rbp + 0x3c8], 1
1412a600f: mov byte ptr [rbp + 0x4e7], 1
1412a6016: lea rdx, [rip + 0x198472b]
1412a601d: lea rcx, [rbp + 0x3b0]
1412a6024: call 0x1428d9390 ; core::panicking::panic_fmt
1412a6029: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a602e: movzx eax, al
1412a6031: cmp eax, 1
1412a6034: jne 0x1412a65a9
1412a603a: lea rax, [rip + 0x17f7dd3]
1412a6041: mov qword ptr [rbp + 0x360], rax
1412a6048: mov qword ptr [rbp + 0x368], 0x25
1412a6053: mov qword ptr [rbp + 0x320], rdi
1412a605a: lea rax, [rip - 0x119c1d1]
1412a6061: mov qword ptr [rbp + 0x328], rax
1412a6068: lea rax, [rip + 0x1831859]
1412a606f: mov qword ptr [rbp + 0x3b0], rax
1412a6076: mov qword ptr [rbp + 0x3b8], 2
1412a6081: mov qword ptr [rbp + 0x3d0], 0
1412a608c: lea rax, [rbp + 0x320]
1412a6093: mov qword ptr [rbp + 0x3c0], rax
1412a609a: mov qword ptr [rbp + 0x3c8], 1
1412a60a5: mov byte ptr [rbp + 0x4e7], 1
1412a60ac: lea rdx, [rip + 0x19846ad]
1412a60b3: lea rcx, [rbp + 0x3b0]
1412a60ba: call 0x1428d9390 ; core::panicking::panic_fmt
1412a60bf: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a60c4: cmp rcx, 1
1412a60c8: jne 0x1412a65f8
1412a60ce: lea rax, [rip + 0x17f7fbe]
1412a60d5: mov qword ptr [rbp + 0x360], rax
1412a60dc: mov qword ptr [rbp + 0x368], 0x29
1412a60e7: lea rax, [rbp + 0x360]
1412a60ee: mov qword ptr [rbp + 0x320], rax
1412a60f5: lea rax, [rip - 0x119c26c]
1412a60fc: mov qword ptr [rbp + 0x328], rax
1412a6103: lea rax, [rip + 0x18317be]
1412a610a: mov qword ptr [rbp + 0x430], rax
1412a6111: mov qword ptr [rbp + 0x438], 2
1412a611c: mov qword ptr [rbp + 0x450], 0
1412a6127: lea rax, [rbp + 0x320]
1412a612e: mov qword ptr [rbp + 0x440], rax
1412a6135: mov qword ptr [rbp + 0x448], 1
1412a6140: mov byte ptr [rbp + 0x4e7], 1
1412a6147: lea rdx, [rip + 0x198462a]
1412a614e: lea rcx, [rbp + 0x430]
1412a6155: call 0x1428d9390 ; core::panicking::panic_fmt
1412a615a: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a615f: cmp eax, 1
1412a6162: jne 0x1412a6647
1412a6168: lea rax, [rip + 0x17f7a99]
1412a616f: mov qword ptr [rbp + 0x360], rax
1412a6176: mov qword ptr [rbp + 0x368], 0x28
1412a6181: lea rax, [rbp + 0x360]
1412a6188: mov qword ptr [rbp + 0x320], rax
1412a618f: lea rax, [rip - 0x119c306]
1412a6196: mov qword ptr [rbp + 0x328], rax
1412a619d: lea rax, [rip + 0x1831724]
1412a61a4: mov qword ptr [rbp + 0x430], rax
1412a61ab: mov qword ptr [rbp + 0x438], 2
1412a61b6: mov qword ptr [rbp + 0x450], 0
1412a61c1: lea rax, [rbp + 0x320]
1412a61c8: mov qword ptr [rbp + 0x440], rax
1412a61cf: mov qword ptr [rbp + 0x448], 1
1412a61da: mov byte ptr [rbp + 0x4e7], 1
1412a61e1: lea rdx, [rip + 0x19845a8]
1412a61e8: lea rcx, [rbp + 0x430]
1412a61ef: call 0x1428d9390 ; core::panicking::panic_fmt
1412a61f4: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a61f9: cmp eax, 1
1412a61fc: jne 0x1412a6696
1412a6202: lea rax, [rip + 0x17f8b9b]
1412a6209: mov qword ptr [rbp + 0x320], rax
1412a6210: mov qword ptr [rbp + 0x328], 8
1412a621b: lea rax, [rbp + 0x320]
1412a6222: mov qword ptr [rbp + 0x2c0], rax
1412a6229: lea rax, [rip - 0x119c3a0]
1412a6230: mov qword ptr [rbp + 0x2c8], rax
1412a6237: lea rax, [rip + 0x183168a]
1412a623e: mov qword ptr [rbp + 0x430], rax
1412a6245: mov qword ptr [rbp + 0x438], 2
1412a6250: mov qword ptr [rbp + 0x450], 0
1412a625b: lea rax, [rbp + 0x2c0]
1412a6262: mov qword ptr [rbp + 0x440], rax
1412a6269: mov qword ptr [rbp + 0x448], 1
1412a6274: mov byte ptr [rbp + 0x4e7], 1
1412a627b: lea rdx, [rip + 0x1984526]
1412a6282: lea rcx, [rbp + 0x430]
1412a6289: call 0x1428d9390 ; core::panicking::panic_fmt
1412a628e: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a6293: cmp rax, 1
1412a6297: jne 0x1412a66e5
1412a629d: lea rax, [rip + 0x17f8838]
1412a62a4: mov qword ptr [rbp + 0x360], rax
1412a62ab: mov qword ptr [rbp + 0x368], 0x15
1412a62b6: mov qword ptr [rbp + 0x320], rdi
1412a62bd: lea rax, [rip - 0x119c434]
1412a62c4: mov qword ptr [rbp + 0x328], rax
1412a62cb: lea rax, [rip + 0x18315f6]
1412a62d2: mov qword ptr [rbp + 0x430], rax
1412a62d9: mov qword ptr [rbp + 0x438], 2
1412a62e4: mov qword ptr [rbp + 0x450], 0
1412a62ef: lea rax, [rbp + 0x320]
1412a62f6: mov qword ptr [rbp + 0x440], rax
1412a62fd: mov qword ptr [rbp + 0x448], 1
1412a6308: mov byte ptr [rbp + 0x4e7], 1
1412a630f: lea rdx, [rip + 0x19844aa]
1412a6316: lea rcx, [rbp + 0x430]
1412a631d: call 0x1428d9390 ; core::panicking::panic_fmt
1412a6322: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a6327: lea rax, [rip + 0x1831552]
1412a632e: mov qword ptr [rbp + 0x430], rax
1412a6335: mov qword ptr [rbp + 0x438], 1
1412a6340: mov qword ptr [rbp + 0x440], 8
1412a634b: pxor xmm0, xmm0
1412a634f: movdqu xmmword ptr [rbp + 0x448], xmm0
1412a6357: lea rdx, [rip + 0x19842ba]
1412a635e: lea rcx, [rbp + 0x430]
1412a6365: call 0x1428d9390 ; core::panicking::panic_fmt
1412a636a: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a636f: lea rax, [rip + 0x183150a]
1412a6376: mov qword ptr [rbp + 0x430], rax
1412a637d: mov qword ptr [rbp + 0x438], 1
1412a6388: mov qword ptr [rbp + 0x440], 8
1412a6393: pxor xmm0, xmm0
1412a6397: movdqu xmmword ptr [rbp + 0x448], xmm0
1412a639f: mov byte ptr [rbp + 0x4e7], 1
1412a63a6: lea rdx, [rip + 0x1984283]
1412a63ad: lea rcx, [rbp + 0x430]
1412a63b4: call 0x1428d9390 ; core::panicking::panic_fmt
1412a63b9: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a63be: lea rax, [rip + 0x18314bb]
1412a63c5: mov qword ptr [rbp + 0x430], rax
1412a63cc: mov qword ptr [rbp + 0x438], 1
1412a63d7: mov qword ptr [rbp + 0x440], 8
1412a63e2: pxor xmm0, xmm0
1412a63e6: movdqu xmmword ptr [rbp + 0x448], xmm0
1412a63ee: mov byte ptr [rbp + 0x4e7], 1
1412a63f5: lea rdx, [rip + 0x198424c]
1412a63fc: lea rcx, [rbp + 0x430]
1412a6403: call 0x1428d9390 ; core::panicking::panic_fmt
1412a6408: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a640d: lea rcx, [rip + 0x199468c]
1412a6414: call 0x1428d80d0 ; std::thread::local::panic_access_error
1412a6419: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a641e: lea rax, [rip + 0x183145b]
1412a6425: mov qword ptr [rbp + 0x3b0], rax
1412a642c: mov qword ptr [rbp + 0x3b8], 1
1412a6437: mov qword ptr [rbp + 0x3c0], 8
1412a6442: pxor xmm0, xmm0
1412a6446: movdqu xmmword ptr [rbp + 0x3c8], xmm0
1412a644e: mov byte ptr [rbp + 0x4e7], 1
1412a6455: lea rdx, [rip + 0x198428c]
1412a645c: lea rcx, [rbp + 0x3b0]
1412a6463: call 0x1428d9390 ; core::panicking::panic_fmt
1412a6468: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a646d: lea rax, [rip + 0x183140c]
1412a6474: mov qword ptr [rbp + 0x3b0], rax
1412a647b: mov qword ptr [rbp + 0x3b8], 1
1412a6486: mov qword ptr [rbp + 0x3c0], 8
1412a6491: pxor xmm0, xmm0
1412a6495: movdqu xmmword ptr [rbp + 0x3c8], xmm0
1412a649d: mov byte ptr [rbp + 0x4e7], 1
1412a64a4: lea rdx, [rip + 0x1984255]
1412a64ab: lea rcx, [rbp + 0x3b0]
1412a64b2: call 0x1428d9390 ; core::panicking::panic_fmt
1412a64b7: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a64bc: lea rax, [rip + 0x18313bd]
1412a64c3: mov qword ptr [rbp + 0x3b0], rax
1412a64ca: mov qword ptr [rbp + 0x3b8], 1
1412a64d5: mov qword ptr [rbp + 0x3c0], 8
1412a64e0: pxor xmm0, xmm0
1412a64e4: movdqu xmmword ptr [rbp + 0x3c8], xmm0
1412a64ec: mov byte ptr [rbp + 0x4e7], 1
1412a64f3: lea rdx, [rip + 0x198421e]
1412a64fa: lea rcx, [rbp + 0x3b0]
1412a6501: call 0x1428d9390 ; core::panicking::panic_fmt
1412a6506: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a650b: lea rax, [rip + 0x183136e]
1412a6512: mov qword ptr [rbp + 0x3b0], rax
1412a6519: mov qword ptr [rbp + 0x3b8], 1
1412a6524: mov qword ptr [rbp + 0x3c0], 8
1412a652f: pxor xmm0, xmm0
1412a6533: movdqu xmmword ptr [rbp + 0x3c8], xmm0
1412a653b: mov byte ptr [rbp + 0x4e7], 1
1412a6542: lea rdx, [rip + 0x19841e7]
1412a6549: lea rcx, [rbp + 0x3b0]
1412a6550: call 0x1428d9390 ; core::panicking::panic_fmt
1412a6555: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a655a: lea rax, [rip + 0x183131f]
1412a6561: mov qword ptr [rbp + 0x3b0], rax
1412a6568: mov qword ptr [rbp + 0x3b8], 1
1412a6573: mov qword ptr [rbp + 0x3c0], 8
1412a657e: pxor xmm0, xmm0
1412a6582: movdqu xmmword ptr [rbp + 0x3c8], xmm0
1412a658a: mov byte ptr [rbp + 0x4e7], 1
1412a6591: lea rdx, [rip + 0x19841b0]
1412a6598: lea rcx, [rbp + 0x3b0]
1412a659f: call 0x1428d9390 ; core::panicking::panic_fmt
1412a65a4: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a65a9: lea rax, [rip + 0x18312d0]
1412a65b0: mov qword ptr [rbp + 0x3b0], rax
1412a65b7: mov qword ptr [rbp + 0x3b8], 1
1412a65c2: mov qword ptr [rbp + 0x3c0], 8
1412a65cd: pxor xmm0, xmm0
1412a65d1: movdqu xmmword ptr [rbp + 0x3c8], xmm0
1412a65d9: mov byte ptr [rbp + 0x4e7], 1
1412a65e0: lea rdx, [rip + 0x1984179]
1412a65e7: lea rcx, [rbp + 0x3b0]
1412a65ee: call 0x1428d9390 ; core::panicking::panic_fmt
1412a65f3: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a65f8: lea rax, [rip + 0x1831281]
1412a65ff: mov qword ptr [rbp + 0x430], rax
1412a6606: mov qword ptr [rbp + 0x438], 1
1412a6611: mov qword ptr [rbp + 0x440], 8
1412a661c: pxor xmm0, xmm0
1412a6620: movdqu xmmword ptr [rbp + 0x448], xmm0
1412a6628: mov byte ptr [rbp + 0x4e7], 1
1412a662f: lea rdx, [rip + 0x1984142]
1412a6636: lea rcx, [rbp + 0x430]
1412a663d: call 0x1428d9390 ; core::panicking::panic_fmt
1412a6642: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a6647: lea rax, [rip + 0x1831232]
1412a664e: mov qword ptr [rbp + 0x430], rax
1412a6655: mov qword ptr [rbp + 0x438], 1
1412a6660: mov qword ptr [rbp + 0x440], 8
1412a666b: pxor xmm0, xmm0
1412a666f: movdqu xmmword ptr [rbp + 0x448], xmm0
1412a6677: mov byte ptr [rbp + 0x4e7], 1
1412a667e: lea rdx, [rip + 0x198410b]
1412a6685: lea rcx, [rbp + 0x430]
1412a668c: call 0x1428d9390 ; core::panicking::panic_fmt
1412a6691: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a6696: lea rax, [rip + 0x18311e3]
1412a669d: mov qword ptr [rbp + 0x430], rax
1412a66a4: mov qword ptr [rbp + 0x438], 1
1412a66af: mov qword ptr [rbp + 0x440], 8
1412a66ba: pxor xmm0, xmm0
1412a66be: movdqu xmmword ptr [rbp + 0x448], xmm0
1412a66c6: mov byte ptr [rbp + 0x4e7], 1
1412a66cd: lea rdx, [rip + 0x19840d4]
1412a66d4: lea rcx, [rbp + 0x430]
1412a66db: call 0x1428d9390 ; core::panicking::panic_fmt
1412a66e0: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a66e5: lea rax, [rip + 0x1831194]
1412a66ec: mov qword ptr [rbp + 0x430], rax
1412a66f3: mov qword ptr [rbp + 0x438], 1
1412a66fe: mov qword ptr [rbp + 0x440], 8
1412a6709: pxor xmm0, xmm0
1412a670d: movdqu xmmword ptr [rbp + 0x448], xmm0
1412a6715: mov byte ptr [rbp + 0x4e7], 1
1412a671c: lea rdx, [rip + 0x198409d]
1412a6723: lea rcx, [rbp + 0x430]
1412a672a: call 0x1428d9390 ; core::panicking::panic_fmt
1412a672f: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a6734: lea rax, [rbp + 0x90]
1412a673b: mov qword ptr [rbp + 0x3b0], rax
1412a6742: lea rax, [rip - 0x1252e69]
1412a6749: mov qword ptr [rbp + 0x3b8], rax
1412a6750: lea rax, [rip + 0x19840d1]
1412a6757: mov qword ptr [rbp + 0x430], rax
1412a675e: mov qword ptr [rbp + 0x438], 1
1412a6769: mov qword ptr [rbp + 0x450], 0
1412a6774: lea rax, [rbp + 0x3b0]
1412a677b: mov qword ptr [rbp + 0x440], rax
1412a6782: mov qword ptr [rbp + 0x448], 1
1412a678d: mov byte ptr [rbp + 0x4e2], 1
1412a6794: lea rdx, [rip + 0x198409d]
1412a679b: lea rcx, [rbp + 0x430]
1412a67a2: call 0x1428d9390 ; core::panicking::panic_fmt
1412a67a7: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a67ac: mov byte ptr [rbp + 0x4e6], 1
1412a67b3: lea rcx, [rip + 0x19942e6]
1412a67ba: call 0x1428d80d0 ; std::thread::local::panic_access_error
1412a67bf: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a67c4: mov rax, qword ptr [rbp + 0x3b8]
1412a67cb: mov qword ptr [rbp + 0x430], rax
1412a67d2: lea rax, [rip + 0x1984127]
1412a67d9: mov qword ptr [rsp + 0x20], rax
1412a67de: lea rcx, [rip + 0x1983833]
1412a67e5: lea r9, [rip + 0x198380c]
1412a67ec: lea r8, [rbp + 0x430]
1412a67f3: mov edx, 0x2b
1412a67f8: call 0x1428d9760 ; core::result::unwrap_failed
1412a67fd: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a6802: mov ecx, 8
1412a6807: mov edx, 0x88
1412a680c: call 0x1428d9000 ; alloc::alloc::handle_alloc_error
1412a6811: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a6816: lea rax, [rip + 0x198410b]
1412a681d: mov qword ptr [rbp + 0x430], rax
1412a6824: mov qword ptr [rbp + 0x438], 1
1412a682f: mov qword ptr [rbp + 0x440], 8
1412a683a: pxor xmm0, xmm0
1412a683e: movdqu xmmword ptr [rbp + 0x448], xmm0
1412a6846: mov byte ptr [rbp + 0x4d2], 1
1412a684d: lea rdx, [rip + 0x19840e4]
1412a6854: lea rcx, [rbp + 0x430]
1412a685b: call 0x1428d9390 ; core::panicking::panic_fmt
1412a6860: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a6862: mov ecx, 8
1412a6867: mov edx, 0x88
1412a686c: call 0x1428d9000 ; alloc::alloc::handle_alloc_error
1412a6871: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a6873: xor edi, edi
1412a6875: lea r8, [rip + 0x1984034]
1412a687c: mov rcx, rdi
1412a687f: mov rdx, rsi
1412a6882: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1412a6887: jmp 0x1412a689d ; country_core::startup::load_prefabs::load_prefabs_impl
1412a6889: xor edi, edi
1412a688b: lea r8, [rip + 0x1983fee]
1412a6892: mov rcx, rdi
1412a6895: mov rdx, rsi
1412a6898: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1412a689d: ud2 
1412a689f: nop 
1412a68a0: mov qword ptr [rsp + 0x10], rdx
1412a68a5: push rbp
1412a68a6: push r15
1412a68a8: push r14
1412a68aa: push r13
1412a68ac: push r12
1412a68ae: push rsi
1412a68af: push rdi
1412a68b0: push rbx
1412a68b1: sub rsp, 0xa8
1412a68b8: lea rbp, [rdx + 0x80]
1412a68bf: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a68c6: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a68cd: movaps xmmword ptr [rsp + 0x40], xmm11
1412a68d3: movaps xmmword ptr [rsp + 0x50], xmm10
1412a68d9: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a68e0: movaps xmmword ptr [rsp + 0x70], xmm8
1412a68e6: movaps xmmword ptr [rsp + 0x80], xmm7
1412a68ee: movaps xmmword ptr [rsp + 0x90], xmm6
1412a68f6: mov rdx, qword ptr [rbp + 0x20]
1412a68fa: test rdx, rdx
1412a68fd: je 0x1412a690e
1412a68ff: mov rcx, qword ptr [rbp + 0x28]
1412a6903: mov r8d, 1
1412a6909: call 0x140613c20
1412a690e: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a6916: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a691e: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a6924: movaps xmm9, xmmword ptr [rsp + 0x60]
1412a692a: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a6930: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a6936: movaps xmm12, xmmword ptr [rsp + 0x30]
1412a693c: movaps xmm13, xmmword ptr [rsp + 0x20]
1412a6942: add rsp, 0xa8
1412a6949: pop rbx
1412a694a: pop rdi
1412a694b: pop rsi
1412a694c: pop r12
1412a694e: pop r13
1412a6950: pop r14
1412a6952: pop r15
1412a6954: pop rbp
1412a6955: ret 
1412a6956: nop word ptr cs:[rax + rax]
1412a6960: mov qword ptr [rsp + 0x10], rdx
1412a6965: push rbp
1412a6966: push r15
1412a6968: push r14
1412a696a: push r13
1412a696c: push r12
1412a696e: push rsi
1412a696f: push rdi
1412a6970: push rbx
1412a6971: sub rsp, 0xa8
1412a6978: lea rbp, [rdx + 0x80]
1412a697f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a6986: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a698d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a6993: movaps xmmword ptr [rsp + 0x50], xmm10
1412a6999: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a69a0: movaps xmmword ptr [rsp + 0x70], xmm8
1412a69a6: movaps xmmword ptr [rsp + 0x80], xmm7
1412a69ae: movaps xmmword ptr [rsp + 0x90], xmm6
1412a69b6: mov byte ptr [rbp + 0x4e6], 1
1412a69bd: mov rcx, qword ptr [rbp + 0x3a0]
1412a69c4: mov edx, dword ptr [rbp + 0x4c8]
1412a69ca: call 0x140043000 ; core::ptr::drop_in_place<std::sync::poison::mutex::MutexGuard<winit::platform_impl::platform::window_state::WindowState> >
1412a69cf: mov byte ptr [rbp + 0x4e6], 1
1412a69d6: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a69de: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a69e6: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a69ec: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a69f3: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a69f9: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a69ff: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a6a06: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a6a0d: add rsp, 0xa8
1412a6a14: pop rbx
1412a6a15: pop rdi
1412a6a16: pop rsi
1412a6a17: pop r12
1412a6a19: pop r13
1412a6a1b: pop r14
1412a6a1d: pop r15
1412a6a1f: pop rbp
1412a6a20: ret 
1412a6a21: nop word ptr cs:[rax + rax]
1412a6a30: mov qword ptr [rsp + 0x10], rdx
1412a6a35: push rbp
1412a6a36: push r15
1412a6a38: push r14
1412a6a3a: push r13
1412a6a3c: push r12
1412a6a3e: push rsi
1412a6a3f: push rdi
1412a6a40: push rbx
1412a6a41: sub rsp, 0xa8
1412a6a48: lea rbp, [rdx + 0x80]
1412a6a4f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a6a56: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a6a5d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a6a63: movaps xmmword ptr [rsp + 0x50], xmm10
1412a6a69: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a6a70: movaps xmmword ptr [rsp + 0x70], xmm8
1412a6a76: movaps xmmword ptr [rsp + 0x80], xmm7
1412a6a7e: movaps xmmword ptr [rsp + 0x90], xmm6
1412a6a86: lea rdx, [rbp + 0x150]
1412a6a8d: lea rcx, [rbp + 0x130]
1412a6a94: mov r8d, 0x10
1412a6a9a: mov r9d, 0x10
1412a6aa0: call 0x140b73f00 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<asset::prefab::PrefabName,tuple$<> >,alloc::alloc::Global>
1412a6aa5: mov eax, dword ptr [rbp + 0x2f4]
1412a6aab: mov byte ptr [rbp + 0x4e6], al
1412a6ab1: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a6ab9: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a6ac1: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a6ac7: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a6ace: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a6ad4: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a6ada: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a6ae1: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a6ae8: add rsp, 0xa8
1412a6aef: pop rbx
1412a6af0: pop rdi
1412a6af1: pop rsi
1412a6af2: pop r12
1412a6af4: pop r13
1412a6af6: pop r14
1412a6af8: pop r15
1412a6afa: pop rbp
1412a6afb: ret 
1412a6afc: nop dword ptr [rax]
1412a6b00: mov qword ptr [rsp + 0x10], rdx
1412a6b05: push rbp
1412a6b06: push r15
1412a6b08: push r14
1412a6b0a: push r13
1412a6b0c: push r12
1412a6b0e: push rsi
1412a6b0f: push rdi
1412a6b10: push rbx
1412a6b11: sub rsp, 0xa8
1412a6b18: lea rbp, [rdx + 0x80]
1412a6b1f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a6b26: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a6b2d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a6b33: movaps xmmword ptr [rsp + 0x50], xmm10
1412a6b39: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a6b40: movaps xmmword ptr [rsp + 0x70], xmm8
1412a6b46: movaps xmmword ptr [rsp + 0x80], xmm7
1412a6b4e: movaps xmmword ptr [rsp + 0x90], xmm6
1412a6b56: lea rcx, [rbp + 0x3b0]
1412a6b5d: call 0x140d4cc30 ; anyhow::error::impl$6::drop
1412a6b62: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a6b6a: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a6b72: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a6b78: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a6b7f: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a6b85: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a6b8b: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a6b92: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a6b99: add rsp, 0xa8
1412a6ba0: pop rbx
1412a6ba1: pop rdi
1412a6ba2: pop rsi
1412a6ba3: pop r12
1412a6ba5: pop r13
1412a6ba7: pop r14
1412a6ba9: pop r15
1412a6bab: pop rbp
1412a6bac: ret 
1412a6bad: nop dword ptr [rax]
1412a6bb0: mov qword ptr [rsp + 0x10], rdx
1412a6bb5: push rbp
1412a6bb6: push r15
1412a6bb8: push r14
1412a6bba: push r13
1412a6bbc: push r12
1412a6bbe: push rsi
1412a6bbf: push rdi
1412a6bc0: push rbx
1412a6bc1: sub rsp, 0xa8
1412a6bc8: lea rbp, [rdx + 0x80]
1412a6bcf: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a6bd6: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a6bdd: movaps xmmword ptr [rsp + 0x40], xmm11
1412a6be3: movaps xmmword ptr [rsp + 0x50], xmm10
1412a6be9: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a6bf0: movaps xmmword ptr [rsp + 0x70], xmm8
1412a6bf6: movaps xmmword ptr [rsp + 0x80], xmm7
1412a6bfe: movaps xmmword ptr [rsp + 0x90], xmm6
1412a6c06: movzx ebx, byte ptr [rbp + 0x4e4]
1412a6c0d: movzx esi, byte ptr [rbp + 0x4e5]
1412a6c14: lea rcx, [rbp + 0x160]
1412a6c1b: mov r8d, 0x38
1412a6c21: mov r9d, 0x10
1412a6c27: lea rdx, [rbp + 0x180]
1412a6c2e: call 0x140b73b80 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >,alloc::alloc::Global>
1412a6c33: mov byte ptr [rbp + 0x4dc], sil
1412a6c3a: mov byte ptr [rbp + 0x4db], bl
1412a6c40: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a6c48: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a6c50: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a6c56: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a6c5d: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a6c63: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a6c69: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a6c70: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a6c77: add rsp, 0xa8
1412a6c7e: pop rbx
1412a6c7f: pop rdi
1412a6c80: pop rsi
1412a6c81: pop r12
1412a6c83: pop r13
1412a6c85: pop r14
1412a6c87: pop r15
1412a6c89: pop rbp
1412a6c8a: ret 
1412a6c8b: nop dword ptr [rax + rax]
1412a6c90: mov qword ptr [rsp + 0x10], rdx
1412a6c95: push rbp
1412a6c96: push r15
1412a6c98: push r14
1412a6c9a: push r13
1412a6c9c: push r12
1412a6c9e: push rsi
1412a6c9f: push rdi
1412a6ca0: push rbx
1412a6ca1: sub rsp, 0xa8
1412a6ca8: lea rbp, [rdx + 0x80]
1412a6caf: movaps xmmword ptr [rsp + 0x20], xmm13
1412a6cb5: movaps xmmword ptr [rsp + 0x30], xmm12
1412a6cbb: movaps xmmword ptr [rsp + 0x40], xmm11
1412a6cc1: movaps xmmword ptr [rsp + 0x50], xmm10
1412a6cc7: movaps xmmword ptr [rsp + 0x60], xmm9
1412a6ccd: movaps xmmword ptr [rsp + 0x70], xmm8
1412a6cd3: movaps xmmword ptr [rsp + 0x80], xmm7
1412a6cdb: movaps xmmword ptr [rsp + 0x90], xmm6
1412a6ce3: cmp qword ptr [rbp + 0x300], 0
1412a6ceb: je 0x1412a6d0e
1412a6ced: mov rax, qword ptr [rbp + 0x300]
1412a6cf4: shl rax, 3
1412a6cf8: lea rdx, [rax + rax*4]
1412a6cfc: mov r8d, 8
1412a6d02: mov rcx, qword ptr [rbp + 0x268]
1412a6d09: call 0x140613c20
1412a6d0e: mov byte ptr [rbp + 0x4e5], 1
1412a6d15: mov byte ptr [rbp + 0x4e4], 0
1412a6d1c: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a6d24: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a6d2c: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a6d32: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a6d39: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a6d3f: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a6d45: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a6d4c: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a6d53: add rsp, 0xa8
1412a6d5a: pop rbx
1412a6d5b: pop rdi
1412a6d5c: pop rsi
1412a6d5d: pop r12
1412a6d5f: pop r13
1412a6d61: pop r14
1412a6d63: pop r15
1412a6d65: pop rbp
1412a6d66: ret 
1412a6d67: nop word ptr [rax + rax]
1412a6d70: mov qword ptr [rsp + 0x10], rdx
1412a6d75: push rbp
1412a6d76: push r15
1412a6d78: push r14
1412a6d7a: push r13
1412a6d7c: push r12
1412a6d7e: push rsi
1412a6d7f: push rdi
1412a6d80: push rbx
1412a6d81: sub rsp, 0xa8
1412a6d88: lea rbp, [rdx + 0x80]
1412a6d8f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a6d96: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a6d9d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a6da3: movaps xmmword ptr [rsp + 0x50], xmm10
1412a6da9: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a6db0: movaps xmmword ptr [rsp + 0x70], xmm8
1412a6db6: movaps xmmword ptr [rsp + 0x80], xmm7
1412a6dbe: movaps xmmword ptr [rsp + 0x90], xmm6
1412a6dc6: mov rdi, qword ptr [rbp + 0x4c8]
1412a6dcd: mov rbx, qword ptr [rbp + 0x3a0]
1412a6dd4: cmp rdi, rbx
1412a6dd7: mov rsi, qword ptr [rbp + 0x3a8]
1412a6dde: je 0x1412a6df5
1412a6de0: lea rax, [rbx + rbx*4]
1412a6de4: lea rcx, [rsi + rax*8]
1412a6de8: call 0x1408989d0 ; core::ptr::drop_in_place<tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab> >
1412a6ded: inc rbx
1412a6df0: cmp rbx, rdi
1412a6df3: jne 0x1412a6de0
1412a6df5: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a6dfd: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a6e05: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a6e0b: movaps xmm9, xmmword ptr [rsp + 0x60]
1412a6e11: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a6e17: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a6e1d: movaps xmm12, xmmword ptr [rsp + 0x30]
1412a6e23: movaps xmm13, xmmword ptr [rsp + 0x20]
1412a6e29: add rsp, 0xa8
1412a6e30: pop rbx
1412a6e31: pop rdi
1412a6e32: pop rsi
1412a6e33: pop r12
1412a6e35: pop r13
1412a6e37: pop r14
1412a6e39: pop r15
1412a6e3b: pop rbp
1412a6e3c: ret 
1412a6e3d: nop dword ptr [rax]
1412a6e40: mov qword ptr [rsp + 0x10], rdx
1412a6e45: push rbp
1412a6e46: push r15
1412a6e48: push r14
1412a6e4a: push r13
1412a6e4c: push r12
1412a6e4e: push rsi
1412a6e4f: push rdi
1412a6e50: push rbx
1412a6e51: sub rsp, 0xa8
1412a6e58: lea rbp, [rdx + 0x80]
1412a6e5f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a6e66: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a6e6d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a6e73: movaps xmmword ptr [rsp + 0x50], xmm10
1412a6e79: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a6e80: movaps xmmword ptr [rsp + 0x70], xmm8
1412a6e86: movaps xmmword ptr [rsp + 0x80], xmm7
1412a6e8e: movaps xmmword ptr [rsp + 0x90], xmm6
1412a6e96: mov rdx, qword ptr [rbp + 0x2c0]
1412a6e9d: test rdx, rdx
1412a6ea0: je 0x1412a6eb4
1412a6ea2: mov rcx, qword ptr [rbp + 0x2c8]
1412a6ea9: mov r8d, 1
1412a6eaf: call 0x140613c20
1412a6eb4: mov byte ptr [rbp + 0x4e5], 0
1412a6ebb: mov byte ptr [rbp + 0x4e4], 0
1412a6ec2: lea rcx, [rbp + 0x430]
1412a6ec9: call 0x14087de20 ; core::ptr::drop_in_place<std::collections::hash::map::IntoIter<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global> > >
1412a6ece: mov byte ptr [rbp + 0x4e5], 0
1412a6ed5: mov byte ptr [rbp + 0x4e4], 0
1412a6edc: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a6ee4: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a6eec: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a6ef2: movaps xmm9, xmmword ptr [rsp + 0x60]
1412a6ef8: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a6efe: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a6f04: movaps xmm12, xmmword ptr [rsp + 0x30]
1412a6f0a: movaps xmm13, xmmword ptr [rsp + 0x20]
1412a6f10: add rsp, 0xa8
1412a6f17: pop rbx
1412a6f18: pop rdi
1412a6f19: pop rsi
1412a6f1a: pop r12
1412a6f1c: pop r13
1412a6f1e: pop r14
1412a6f20: pop r15
1412a6f22: pop rbp
1412a6f23: ret 
1412a6f24: nop word ptr cs:[rax + rax]
1412a6f30: mov qword ptr [rsp + 0x10], rdx
1412a6f35: push rbp
1412a6f36: push r15
1412a6f38: push r14
1412a6f3a: push r13
1412a6f3c: push r12
1412a6f3e: push rsi
1412a6f3f: push rdi
1412a6f40: push rbx
1412a6f41: sub rsp, 0xa8
1412a6f48: lea rbp, [rdx + 0x80]
1412a6f4f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a6f56: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a6f5d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a6f63: movaps xmmword ptr [rsp + 0x50], xmm10
1412a6f69: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a6f70: movaps xmmword ptr [rsp + 0x70], xmm8
1412a6f76: movaps xmmword ptr [rsp + 0x80], xmm7
1412a6f7e: movaps xmmword ptr [rsp + 0x90], xmm6
1412a6f86: cmp qword ptr [rbp + 0x210], 0
1412a6f8e: jne 0x1412a6ffa
1412a6f90: lea rdx, [rbp + 0x120]
1412a6f97: mov r8d, 0x38
1412a6f9d: mov r9d, 0x10
1412a6fa3: lea rcx, [rbp + 0x100]
1412a6faa: call 0x140b74970 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global> >,alloc::alloc::Global>
1412a6faf: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a6fb7: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a6fbf: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a6fc5: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a6fcc: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a6fd2: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a6fd8: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a6fdf: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a6fe6: add rsp, 0xa8
1412a6fed: pop rbx
1412a6fee: pop rdi
1412a6fef: pop rsi
1412a6ff0: pop r12
1412a6ff2: pop r13
1412a6ff4: pop r14
1412a6ff6: pop r15
1412a6ff8: pop rbp
1412a6ff9: ret 
1412a6ffa: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a7002: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a700a: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a7010: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a7017: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a701d: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a7023: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a702a: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a7031: add rsp, 0xa8
1412a7038: pop rbx
1412a7039: pop rdi
1412a703a: pop rsi
1412a703b: pop r12
1412a703d: pop r13
1412a703f: pop r14
1412a7041: pop r15
1412a7043: pop rbp
1412a7044: ret 
1412a7045: nop word ptr cs:[rax + rax]
1412a7050: mov qword ptr [rsp + 0x10], rdx
1412a7055: push rbp
1412a7056: push r15
1412a7058: push r14
1412a705a: push r13
1412a705c: push r12
1412a705e: push rsi
1412a705f: push rdi
1412a7060: push rbx
1412a7061: sub rsp, 0xa8
1412a7068: lea rbp, [rdx + 0x80]
1412a706f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a7076: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a707d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a7083: movaps xmmword ptr [rsp + 0x50], xmm10
1412a7089: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a7090: movaps xmmword ptr [rsp + 0x70], xmm8
1412a7096: movaps xmmword ptr [rsp + 0x80], xmm7
1412a709e: movaps xmmword ptr [rsp + 0x90], xmm6
1412a70a6: lea rcx, [rbp + 0x430]
1412a70ad: call 0x140b70cc0 ; core::ptr::drop_in_place<tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab> >
1412a70b2: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a70ba: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a70c2: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a70c8: movaps xmm9, xmmword ptr [rsp + 0x60]
1412a70ce: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a70d4: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a70da: movaps xmm12, xmmword ptr [rsp + 0x30]
1412a70e0: movaps xmm13, xmmword ptr [rsp + 0x20]
1412a70e6: add rsp, 0xa8
1412a70ed: pop rbx
1412a70ee: pop rdi
1412a70ef: pop rsi
1412a70f0: pop r12
1412a70f2: pop r13
1412a70f4: pop r14
1412a70f6: pop r15
1412a70f8: pop rbp
1412a70f9: ret 
1412a70fa: nop word ptr [rax + rax]
1412a7100: mov qword ptr [rsp + 0x10], rdx
1412a7105: push rbp
1412a7106: push r15
1412a7108: push r14
1412a710a: push r13
1412a710c: push r12
1412a710e: push rsi
1412a710f: push rdi
1412a7110: push rbx
1412a7111: sub rsp, 0xa8
1412a7118: lea rbp, [rdx + 0x80]
1412a711f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a7126: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a712d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a7133: movaps xmmword ptr [rsp + 0x50], xmm10
1412a7139: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a7140: movaps xmmword ptr [rsp + 0x70], xmm8
1412a7146: movaps xmmword ptr [rsp + 0x80], xmm7
1412a714e: movaps xmmword ptr [rsp + 0x90], xmm6
1412a7156: mov rdx, qword ptr [rbp + 0x3b0]
1412a715d: test rdx, rdx
1412a7160: je 0x1412a7174
1412a7162: mov rcx, qword ptr [rbp + 0x3b8]
1412a7169: mov r8d, 1
1412a716f: call 0x140613c20
1412a7174: mov byte ptr [rbp + 0x4d5], 1
1412a717b: mov byte ptr [rbp + 0x4d4], 1
1412a7182: mov byte ptr [rbp + 0x4d3], 1
1412a7189: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a7191: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a7199: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a719f: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a71a6: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a71ac: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a71b2: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a71b9: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a71c0: add rsp, 0xa8
1412a71c7: pop rbx
1412a71c8: pop rdi
1412a71c9: pop rsi
1412a71ca: pop r12
1412a71cc: pop r13
1412a71ce: pop r14
1412a71d0: pop r15
1412a71d2: pop rbp
1412a71d3: ret 
1412a71d4: nop word ptr cs:[rax + rax]
1412a71e0: mov qword ptr [rsp + 0x10], rdx
1412a71e5: push rbp
1412a71e6: push r15
1412a71e8: push r14
1412a71ea: push r13
1412a71ec: push r12
1412a71ee: push rsi
1412a71ef: push rdi
1412a71f0: push rbx
1412a71f1: sub rsp, 0xa8
1412a71f8: lea rbp, [rdx + 0x80]
1412a71ff: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a7206: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a720d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a7213: movaps xmmword ptr [rsp + 0x50], xmm10
1412a7219: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a7220: movaps xmmword ptr [rsp + 0x70], xmm8
1412a7226: movaps xmmword ptr [rsp + 0x80], xmm7
1412a722e: movaps xmmword ptr [rsp + 0x90], xmm6
1412a7236: movzx ebx, byte ptr [rbp + 0x4de]
1412a723d: movzx esi, byte ptr [rbp + 0x4df]
1412a7244: movzx edi, byte ptr [rbp + 0x4e0]
1412a724b: mov rdx, qword ptr [rbp + 0x2c0]
1412a7252: test rdx, rdx
1412a7255: je 0x1412a7269
1412a7257: mov rcx, qword ptr [rbp + 0x2c8]
1412a725e: mov r8d, 1
1412a7264: call 0x140613c20
1412a7269: mov rdx, qword ptr [rbp + 0x2d8]
1412a7270: lea rax, [rdx*2]
1412a7278: test rax, rax
1412a727b: je 0x1412a728f
1412a727d: mov rcx, qword ptr [rbp + 0x2e0]
1412a7284: mov r8d, 1
1412a728a: call 0x140613c20
1412a728f: mov byte ptr [rbp + 0x4d5], dil
1412a7296: and sil, 1
1412a729a: mov byte ptr [rbp + 0x4d4], sil
1412a72a1: and bl, 1
1412a72a4: mov byte ptr [rbp + 0x4d3], bl
1412a72aa: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a72b2: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a72ba: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a72c0: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a72c7: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a72cd: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a72d3: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a72da: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a72e1: add rsp, 0xa8
1412a72e8: pop rbx
1412a72e9: pop rdi
1412a72ea: pop rsi
1412a72eb: pop r12
1412a72ed: pop r13
1412a72ef: pop r14
1412a72f1: pop r15
1412a72f3: pop rbp
1412a72f4: ret 
1412a72f5: nop word ptr cs:[rax + rax]
1412a7300: mov qword ptr [rsp + 0x10], rdx
1412a7305: push rbp
1412a7306: push r15
1412a7308: push r14
1412a730a: push r13
1412a730c: push r12
1412a730e: push rsi
1412a730f: push rdi
1412a7310: push rbx
1412a7311: sub rsp, 0xa8
1412a7318: lea rbp, [rdx + 0x80]
1412a731f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a7326: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a732d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a7333: movaps xmmword ptr [rsp + 0x50], xmm10
1412a7339: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a7340: movaps xmmword ptr [rsp + 0x70], xmm8
1412a7346: movaps xmmword ptr [rsp + 0x80], xmm7
1412a734e: movaps xmmword ptr [rsp + 0x90], xmm6
1412a7356: mov byte ptr [rbp + 0x4e3], 0
1412a735d: lea rcx, [rbp + 0x430]
1412a7364: call 0x140893000 ; core::ptr::drop_in_place<alloc::sync::ArcInner<country_core::render::mesh::Mesh> >
1412a7369: mov byte ptr [rbp + 0x4e3], 0
1412a7370: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a7378: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a7380: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a7386: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a738d: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a7393: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a7399: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a73a0: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a73a7: add rsp, 0xa8
1412a73ae: pop rbx
1412a73af: pop rdi
1412a73b0: pop rsi
1412a73b1: pop r12
1412a73b3: pop r13
1412a73b5: pop r14
1412a73b7: pop r15
1412a73b9: pop rbp
1412a73ba: ret 
1412a73bb: nop dword ptr [rax + rax]
1412a73c0: mov qword ptr [rsp + 0x10], rdx
1412a73c5: push rbp
1412a73c6: push r15
1412a73c8: push r14
1412a73ca: push r13
1412a73cc: push r12
1412a73ce: push rsi
1412a73cf: push rdi
1412a73d0: push rbx
1412a73d1: sub rsp, 0xa8
1412a73d8: lea rbp, [rdx + 0x80]
1412a73df: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a73e6: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a73ed: movaps xmmword ptr [rsp + 0x40], xmm11
1412a73f3: movaps xmmword ptr [rsp + 0x50], xmm10
1412a73f9: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a7400: movaps xmmword ptr [rsp + 0x70], xmm8
1412a7406: movaps xmmword ptr [rsp + 0x80], xmm7
1412a740e: movaps xmmword ptr [rsp + 0x90], xmm6
1412a7416: cmp byte ptr [rbp + 0x4e3], 0
1412a741d: je 0x1412a743d
1412a741f: mov rdx, qword ptr [rbp + 0x320]
1412a7426: test rdx, rdx
1412a7429: je 0x1412a743d
1412a742b: mov rcx, qword ptr [rbp + 0x328]
1412a7432: mov r8d, 1
1412a7438: call 0x140613c20
1412a743d: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a7445: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a744d: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a7453: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a745a: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a7460: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a7466: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a746d: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a7474: add rsp, 0xa8
1412a747b: pop rbx
1412a747c: pop rdi
1412a747d: pop rsi
1412a747e: pop r12
1412a7480: pop r13
1412a7482: pop r14
1412a7484: pop r15
1412a7486: pop rbp
1412a7487: ret 
1412a7488: nop dword ptr [rax + rax]
1412a7490: mov qword ptr [rsp + 0x10], rdx
1412a7495: push rbp
1412a7496: push r15
1412a7498: push r14
1412a749a: push r13
1412a749c: push r12
1412a749e: push rsi
1412a749f: push rdi
1412a74a0: push rbx
1412a74a1: sub rsp, 0xa8
1412a74a8: lea rbp, [rdx + 0x80]
1412a74af: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a74b6: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a74bd: movaps xmmword ptr [rsp + 0x40], xmm11
1412a74c3: movaps xmmword ptr [rsp + 0x50], xmm10
1412a74c9: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a74d0: movaps xmmword ptr [rsp + 0x70], xmm8
1412a74d6: movaps xmmword ptr [rsp + 0x80], xmm7
1412a74de: movaps xmmword ptr [rsp + 0x90], xmm6
1412a74e6: mov byte ptr [rbp + 0x4e0], 1
1412a74ed: mov byte ptr [rbp + 0x4df], 1
1412a74f4: mov byte ptr [rbp + 0x4de], 0
1412a74fb: lea rcx, [rbp + 0x360]
1412a7502: call 0x14014b4d0 ; core::ptr::drop_in_place<tuple$<alloc::string::String,epaint::text::fonts::FontData> >
1412a7507: mov byte ptr [rbp + 0x4e0], 1
1412a750e: mov byte ptr [rbp + 0x4df], 1
1412a7515: mov byte ptr [rbp + 0x4de], 0
1412a751c: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a7524: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a752c: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a7532: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a7539: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a753f: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a7545: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a754c: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a7553: add rsp, 0xa8
1412a755a: pop rbx
1412a755b: pop rdi
1412a755c: pop rsi
1412a755d: pop r12
1412a755f: pop r13
1412a7561: pop r14
1412a7563: pop r15
1412a7565: pop rbp
1412a7566: ret 
1412a7567: nop word ptr [rax + rax]
1412a7570: mov qword ptr [rsp + 0x10], rdx
1412a7575: push rbp
1412a7576: push r15
1412a7578: push r14
1412a757a: push r13
1412a757c: push r12
1412a757e: push rsi
1412a757f: push rdi
1412a7580: push rbx
1412a7581: sub rsp, 0xa8
1412a7588: lea rbp, [rdx + 0x80]
1412a758f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a7596: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a759d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a75a3: movaps xmmword ptr [rsp + 0x50], xmm10
1412a75a9: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a75b0: movaps xmmword ptr [rsp + 0x70], xmm8
1412a75b6: movaps xmmword ptr [rsp + 0x80], xmm7
1412a75be: movaps xmmword ptr [rsp + 0x90], xmm6
1412a75c6: lea rcx, [rbp + 0x360]
1412a75cd: mov r8d, 0x38
1412a75d3: mov r9d, 0x10
1412a75d9: lea rdx, [rbp + 0x380]
1412a75e0: call 0x140b74f50 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<array$<u32,3>,country_core::startup::load_prefabs::recalculate_canopy_quad_attribs::Quad>,alloc::alloc::Global>
1412a75e5: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a75ed: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a75f5: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a75fb: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a7602: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a7608: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a760e: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a7615: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a761c: add rsp, 0xa8
1412a7623: pop rbx
1412a7624: pop rdi
1412a7625: pop rsi
1412a7626: pop r12
1412a7628: pop r13
1412a762a: pop r14
1412a762c: pop r15
1412a762e: pop rbp
1412a762f: ret 
1412a7630: mov qword ptr [rsp + 0x10], rdx
1412a7635: push rbp
1412a7636: push r15
1412a7638: push r14
1412a763a: push r13
1412a763c: push r12
1412a763e: push rsi
1412a763f: push rdi
1412a7640: push rbx
1412a7641: sub rsp, 0xa8
1412a7648: lea rbp, [rdx + 0x80]
1412a764f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a7656: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a765d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a7663: movaps xmmword ptr [rsp + 0x50], xmm10
1412a7669: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a7670: movaps xmmword ptr [rsp + 0x70], xmm8
1412a7676: movaps xmmword ptr [rsp + 0x80], xmm7
1412a767e: movaps xmmword ptr [rsp + 0x90], xmm6
1412a7686: movzx ebx, byte ptr [rbp + 0x4d9]
1412a768d: cmp byte ptr [rbp + 0x4da], 0
1412a7694: je 0x1412a7710
1412a7696: mov rax, qword ptr [rbp + 0x320]
1412a769d: test rax, rax
1412a76a0: je 0x1412a76bc
1412a76a2: mov rcx, qword ptr [rbp + 0x328]
1412a76a9: shl rax, 2
1412a76ad: lea rdx, [rax + rax*2]
1412a76b1: mov r8d, 4
1412a76b7: call 0x140613c20
1412a76bc: and bl, 1
1412a76bf: mov byte ptr [rbp + 0x4dd], bl
1412a76c5: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a76cd: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a76d5: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a76db: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a76e2: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a76e8: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a76ee: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a76f5: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a76fc: add rsp, 0xa8
1412a7703: pop rbx
1412a7704: pop rdi
1412a7705: pop rsi
1412a7706: pop r12
1412a7708: pop r13
1412a770a: pop r14
1412a770c: pop r15
1412a770e: pop rbp
1412a770f: ret 
1412a7710: and bl, 1
1412a7713: mov byte ptr [rbp + 0x4dd], bl
1412a7719: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a7721: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a7729: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a772f: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a7736: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a773c: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a7742: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a7749: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a7750: add rsp, 0xa8
1412a7757: pop rbx
1412a7758: pop rdi
1412a7759: pop rsi
1412a775a: pop r12
1412a775c: pop r13
1412a775e: pop r14
1412a7760: pop r15
1412a7762: pop rbp
1412a7763: ret 
1412a7764: nop word ptr cs:[rax + rax]
1412a7770: mov qword ptr [rsp + 0x10], rdx
1412a7775: push rbp
1412a7776: push r15
1412a7778: push r14
1412a777a: push r13
1412a777c: push r12
1412a777e: push rsi
1412a777f: push rdi
1412a7780: push rbx
1412a7781: sub rsp, 0xa8
1412a7788: lea rbp, [rdx + 0x80]
1412a778f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a7796: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a779d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a77a3: movaps xmmword ptr [rsp + 0x50], xmm10
1412a77a9: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a77b0: movaps xmmword ptr [rsp + 0x70], xmm8
1412a77b6: movaps xmmword ptr [rsp + 0x80], xmm7
1412a77be: movaps xmmword ptr [rsp + 0x90], xmm6
1412a77c6: cmp byte ptr [rbp + 0x4dd], 0
1412a77cd: je 0x1412a7840
1412a77cf: mov rax, qword ptr [rbp + 0x1b0]
1412a77d6: test rax, rax
1412a77d9: je 0x1412a77f5
1412a77db: mov rcx, qword ptr [rbp + 0x1b8]
1412a77e2: shl rax, 2
1412a77e6: lea rdx, [rax + rax*2]
1412a77ea: mov r8d, 4
1412a77f0: call 0x140613c20
1412a77f5: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a77fd: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a7805: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a780b: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a7812: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a7818: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a781e: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a7825: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a782c: add rsp, 0xa8
1412a7833: pop rbx
1412a7834: pop rdi
1412a7835: pop rsi
1412a7836: pop r12
1412a7838: pop r13
1412a783a: pop r14
1412a783c: pop r15
1412a783e: pop rbp
1412a783f: ret 
1412a7840: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a7848: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a7850: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a7856: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a785d: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a7863: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a7869: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a7870: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a7877: add rsp, 0xa8
1412a787e: pop rbx
1412a787f: pop rdi
1412a7880: pop rsi
1412a7881: pop r12
1412a7883: pop r13
1412a7885: pop r14
1412a7887: pop r15
1412a7889: pop rbp
1412a788a: ret 
1412a788b: nop dword ptr [rax + rax]
1412a7890: mov qword ptr [rsp + 0x10], rdx
1412a7895: push rbp
1412a7896: push r15
1412a7898: push r14
1412a789a: push r13
1412a789c: push r12
1412a789e: push rsi
1412a789f: push rdi
1412a78a0: push rbx
1412a78a1: sub rsp, 0xa8
1412a78a8: lea rbp, [rdx + 0x80]
1412a78af: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a78b6: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a78bd: movaps xmmword ptr [rsp + 0x40], xmm11
1412a78c3: movaps xmmword ptr [rsp + 0x50], xmm10
1412a78c9: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a78d0: movaps xmmword ptr [rsp + 0x70], xmm8
1412a78d6: movaps xmmword ptr [rsp + 0x80], xmm7
1412a78de: movaps xmmword ptr [rsp + 0x90], xmm6
1412a78e6: lea rcx, [rbp + 0x430]
1412a78ed: call 0x140893000 ; core::ptr::drop_in_place<alloc::sync::ArcInner<country_core::render::mesh::Mesh> >
1412a78f2: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a78fa: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a7902: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a7908: movaps xmm9, xmmword ptr [rsp + 0x60]
1412a790e: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a7914: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a791a: movaps xmm12, xmmword ptr [rsp + 0x30]
1412a7920: movaps xmm13, xmmword ptr [rsp + 0x20]
1412a7926: add rsp, 0xa8
1412a792d: pop rbx
1412a792e: pop rdi
1412a792f: pop rsi
1412a7930: pop r12
1412a7932: pop r13
1412a7934: pop r14
1412a7936: pop r15
1412a7938: pop rbp
1412a7939: ret 
1412a793a: nop word ptr [rax + rax]
1412a7940: mov qword ptr [rsp + 0x10], rdx
1412a7945: push rbp
1412a7946: push r15
1412a7948: push r14
1412a794a: push r13
1412a794c: push r12
1412a794e: push rsi
1412a794f: push rdi
1412a7950: push rbx
1412a7951: sub rsp, 0xa8
1412a7958: lea rbp, [rdx + 0x80]
1412a795f: movaps xmmword ptr [rsp + 0x20], xmm13
1412a7965: movaps xmmword ptr [rsp + 0x30], xmm12
1412a796b: movaps xmmword ptr [rsp + 0x40], xmm11
1412a7971: movaps xmmword ptr [rsp + 0x50], xmm10
1412a7977: movaps xmmword ptr [rsp + 0x60], xmm9
1412a797d: movaps xmmword ptr [rsp + 0x70], xmm8
1412a7983: movaps xmmword ptr [rsp + 0x80], xmm7
1412a798b: movaps xmmword ptr [rsp + 0x90], xmm6
1412a7993: lea rcx, [rbp + 0x360]
1412a799a: call 0x14014b4d0 ; core::ptr::drop_in_place<tuple$<alloc::string::String,epaint::text::fonts::FontData> >
1412a799f: mov byte ptr [rbp + 0x4d2], 0
1412a79a6: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a79ae: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a79b6: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a79bc: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a79c3: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a79c9: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a79cf: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a79d6: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a79dd: add rsp, 0xa8
1412a79e4: pop rbx
1412a79e5: pop rdi
1412a79e6: pop rsi
1412a79e7: pop r12
1412a79e9: pop r13
1412a79eb: pop r14
1412a79ed: pop r15
1412a79ef: pop rbp
1412a79f0: ret 
1412a79f1: nop word ptr cs:[rax + rax]
1412a7a00: mov qword ptr [rsp + 0x10], rdx
1412a7a05: push rbp
1412a7a06: push r15
1412a7a08: push r14
1412a7a0a: push r13
1412a7a0c: push r12
1412a7a0e: push rsi
1412a7a0f: push rdi
1412a7a10: push rbx
1412a7a11: sub rsp, 0xa8
1412a7a18: lea rbp, [rdx + 0x80]
1412a7a1f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a7a26: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a7a2d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a7a33: movaps xmmword ptr [rsp + 0x50], xmm10
1412a7a39: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a7a40: movaps xmmword ptr [rsp + 0x70], xmm8
1412a7a46: movaps xmmword ptr [rsp + 0x80], xmm7
1412a7a4e: movaps xmmword ptr [rsp + 0x90], xmm6
1412a7a56: cmp byte ptr [rbp + 0x4d2], 0
1412a7a5d: je 0x1412a7abd
1412a7a5f: lea rcx, [rbp + 0x3b0]
1412a7a66: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
1412a7a6b: mov byte ptr [rbp + 0x4e2], 0
1412a7a72: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a7a7a: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a7a82: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a7a88: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a7a8f: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a7a95: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a7a9b: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a7aa2: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a7aa9: add rsp, 0xa8
1412a7ab0: pop rbx
1412a7ab1: pop rdi
1412a7ab2: pop rsi
1412a7ab3: pop r12
1412a7ab5: pop r13
1412a7ab7: pop r14
1412a7ab9: pop r15
1412a7abb: pop rbp
1412a7abc: ret 
1412a7abd: mov byte ptr [rbp + 0x4e2], 0
1412a7ac4: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a7acc: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a7ad4: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a7ada: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a7ae1: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a7ae7: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a7aed: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a7af4: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a7afb: add rsp, 0xa8
1412a7b02: pop rbx
1412a7b03: pop rdi
1412a7b04: pop rsi
1412a7b05: pop r12
1412a7b07: pop r13
1412a7b09: pop r14
1412a7b0b: pop r15
1412a7b0d: pop rbp
1412a7b0e: ret 
1412a7b0f: nop 
1412a7b10: mov qword ptr [rsp + 0x10], rdx
1412a7b15: push rbp
1412a7b16: push r15
1412a7b18: push r14
1412a7b1a: push r13
1412a7b1c: push r12
1412a7b1e: push rsi
1412a7b1f: push rdi
1412a7b20: push rbx
1412a7b21: sub rsp, 0xa8
1412a7b28: lea rbp, [rdx + 0x80]
1412a7b2f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a7b36: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a7b3d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a7b43: movaps xmmword ptr [rsp + 0x50], xmm10
1412a7b49: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a7b50: movaps xmmword ptr [rsp + 0x70], xmm8
1412a7b56: movaps xmmword ptr [rsp + 0x80], xmm7
1412a7b5e: movaps xmmword ptr [rsp + 0x90], xmm6
1412a7b66: movzx ebx, byte ptr [rbp + 0x4e2]
1412a7b6d: test bl, 1
1412a7b70: je 0x1412a7b90
1412a7b72: mov rdx, qword ptr [rbp + 0x2a0]
1412a7b79: test rdx, rdx
1412a7b7c: je 0x1412a7b90
1412a7b7e: mov rcx, qword ptr [rbp + 0x2a8]
1412a7b85: mov r8d, 1
1412a7b8b: call 0x140613c20
1412a7b90: mov rax, qword ptr [rbp + 0x260]
1412a7b97: lock dec qword ptr [rax]
1412a7b9b: jne 0x1412a7ba9
1412a7b9d: lea rcx, [rbp + 0x260]
1412a7ba4: call 0x140b5c830 ; alloc::sync::Arc::drop_slow<country_core::render::mesh::Mesh,alloc::alloc::Global>
1412a7ba9: mov byte ptr [rbp + 0x4e0], bl
1412a7baf: mov byte ptr [rbp + 0x4df], 0
1412a7bb6: mov byte ptr [rbp + 0x4de], 0
1412a7bbd: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a7bc5: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a7bcd: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a7bd3: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a7bda: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a7be0: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a7be6: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a7bed: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a7bf4: add rsp, 0xa8
1412a7bfb: pop rbx
1412a7bfc: pop rdi
1412a7bfd: pop rsi
1412a7bfe: pop r12
1412a7c00: pop r13
1412a7c02: pop r14
1412a7c04: pop r15
1412a7c06: pop rbp
1412a7c07: ret 
1412a7c08: nop dword ptr [rax + rax]
1412a7c10: mov qword ptr [rsp + 0x10], rdx
1412a7c15: push rbp
1412a7c16: push r15
1412a7c18: push r14
1412a7c1a: push r13
1412a7c1c: push r12
1412a7c1e: push rsi
1412a7c1f: push rdi
1412a7c20: push rbx
1412a7c21: sub rsp, 0xa8
1412a7c28: lea rbp, [rdx + 0x80]
1412a7c2f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a7c36: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a7c3d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a7c43: movaps xmmword ptr [rsp + 0x50], xmm10
1412a7c49: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a7c50: movaps xmmword ptr [rsp + 0x70], xmm8
1412a7c56: movaps xmmword ptr [rsp + 0x80], xmm7
1412a7c5e: movaps xmmword ptr [rsp + 0x90], xmm6
1412a7c66: movzx ebx, byte ptr [rbp + 0x4d4]
1412a7c6d: movzx esi, byte ptr [rbp + 0x4d5]
1412a7c74: cmp byte ptr [rbp + 0x4d3], 0
1412a7c7b: je 0x1412a7cf6
1412a7c7d: mov rdx, qword ptr [rbp + 0x318]
1412a7c84: test rdx, rdx
1412a7c87: je 0x1412a7c9b
1412a7c89: mov r8d, 1
1412a7c8f: mov rcx, qword ptr [rbp + 0x218]
1412a7c96: call 0x140613c20
1412a7c9b: mov byte ptr [rbp + 0x4d7], sil
1412a7ca2: and bl, 1
1412a7ca5: mov byte ptr [rbp + 0x4d6], bl
1412a7cab: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a7cb3: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a7cbb: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a7cc1: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a7cc8: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a7cce: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a7cd4: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a7cdb: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a7ce2: add rsp, 0xa8
1412a7ce9: pop rbx
1412a7cea: pop rdi
1412a7ceb: pop rsi
1412a7cec: pop r12
1412a7cee: pop r13
1412a7cf0: pop r14
1412a7cf2: pop r15
1412a7cf4: pop rbp
1412a7cf5: ret 
1412a7cf6: mov byte ptr [rbp + 0x4d7], sil
1412a7cfd: and bl, 1
1412a7d00: mov byte ptr [rbp + 0x4d6], bl
1412a7d06: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a7d0e: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a7d16: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a7d1c: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a7d23: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a7d29: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a7d2f: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a7d36: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a7d3d: add rsp, 0xa8
1412a7d44: pop rbx
1412a7d45: pop rdi
1412a7d46: pop rsi
1412a7d47: pop r12
1412a7d49: pop r13
1412a7d4b: pop r14
1412a7d4d: pop r15
1412a7d4f: pop rbp
1412a7d50: ret 
1412a7d51: nop word ptr cs:[rax + rax]
1412a7d60: mov qword ptr [rsp + 0x10], rdx
1412a7d65: push rbp
1412a7d66: push r15
1412a7d68: push r14
1412a7d6a: push r13
1412a7d6c: push r12
1412a7d6e: push rsi
1412a7d6f: push rdi
1412a7d70: push rbx
1412a7d71: sub rsp, 0xa8
1412a7d78: lea rbp, [rdx + 0x80]
1412a7d7f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a7d86: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a7d8d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a7d93: movaps xmmword ptr [rsp + 0x50], xmm10
1412a7d99: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a7da0: movaps xmmword ptr [rsp + 0x70], xmm8
1412a7da6: movaps xmmword ptr [rsp + 0x80], xmm7
1412a7dae: movaps xmmword ptr [rsp + 0x90], xmm6
1412a7db6: movzx ebx, byte ptr [rbp + 0x4d7]
1412a7dbd: mov rdx, qword ptr [rbp + 0x1c8]
1412a7dc4: mov rax, rdx
1412a7dc7: neg rax
1412a7dca: jno 0x1412a7e1d
1412a7dcc: mov byte ptr [rbp + 0x4e1], bl
1412a7dd2: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a7dda: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a7de2: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a7de8: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a7def: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a7df5: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a7dfb: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a7e02: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a7e09: add rsp, 0xa8
1412a7e10: pop rbx
1412a7e11: pop rdi
1412a7e12: pop rsi
1412a7e13: pop r12
1412a7e15: pop r13
1412a7e17: pop r14
1412a7e19: pop r15
1412a7e1b: pop rbp
1412a7e1c: ret 
1412a7e1d: test byte ptr [rbp + 0x4d6], 1
1412a7e24: je 0x1412a7e8e
1412a7e26: test rdx, rdx
1412a7e29: je 0x1412a7e3d
1412a7e2b: mov rcx, qword ptr [rbp + 0x1d0]
1412a7e32: mov r8d, 1
1412a7e38: call 0x140613c20
1412a7e3d: mov byte ptr [rbp + 0x4e1], bl
1412a7e43: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a7e4b: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a7e53: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a7e59: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a7e60: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a7e66: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a7e6c: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a7e73: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a7e7a: add rsp, 0xa8
1412a7e81: pop rbx
1412a7e82: pop rdi
1412a7e83: pop rsi
1412a7e84: pop r12
1412a7e86: pop r13
1412a7e88: pop r14
1412a7e8a: pop r15
1412a7e8c: pop rbp
1412a7e8d: ret 
1412a7e8e: mov byte ptr [rbp + 0x4e1], bl
1412a7e94: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a7e9c: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a7ea4: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a7eaa: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a7eb1: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a7eb7: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a7ebd: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a7ec4: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a7ecb: add rsp, 0xa8
1412a7ed2: pop rbx
1412a7ed3: pop rdi
1412a7ed4: pop rsi
1412a7ed5: pop r12
1412a7ed7: pop r13
1412a7ed9: pop r14
1412a7edb: pop r15
1412a7edd: pop rbp
1412a7ede: ret 
1412a7edf: nop 
1412a7ee0: mov qword ptr [rsp + 0x10], rdx
1412a7ee5: push rbp
1412a7ee6: push r15
1412a7ee8: push r14
1412a7eea: push r13
1412a7eec: push r12
1412a7eee: push rsi
1412a7eef: push rdi
1412a7ef0: push rbx
1412a7ef1: sub rsp, 0xa8
1412a7ef8: lea rbp, [rdx + 0x80]
1412a7eff: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a7f06: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a7f0d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a7f13: movaps xmmword ptr [rsp + 0x50], xmm10
1412a7f19: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a7f20: movaps xmmword ptr [rsp + 0x70], xmm8
1412a7f26: movaps xmmword ptr [rsp + 0x80], xmm7
1412a7f2e: movaps xmmword ptr [rsp + 0x90], xmm6
1412a7f36: test byte ptr [rbp + 0x4e1], 1
1412a7f3d: je 0x1412a7f5d
1412a7f3f: mov rdx, qword ptr [rbp + 0x190]
1412a7f46: test rdx, rdx
1412a7f49: je 0x1412a7f5d
1412a7f4b: mov rcx, qword ptr [rbp + 0x198]
1412a7f52: mov r8d, 1
1412a7f58: call 0x140613c20
1412a7f5d: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a7f65: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a7f6d: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a7f73: movaps xmm9, xmmword ptr [rsp + 0x60]
1412a7f79: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a7f7f: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a7f85: movaps xmm12, xmmword ptr [rsp + 0x30]
1412a7f8b: movaps xmm13, xmmword ptr [rsp + 0x20]
1412a7f91: add rsp, 0xa8
1412a7f98: pop rbx
1412a7f99: pop rdi
1412a7f9a: pop rsi
1412a7f9b: pop r12
1412a7f9d: pop r13
1412a7f9f: pop r14
1412a7fa1: pop r15
1412a7fa3: pop rbp
1412a7fa4: ret 
1412a7fa5: nop word ptr cs:[rax + rax]
1412a7fb0: mov qword ptr [rsp + 0x10], rdx
1412a7fb5: push rbp
1412a7fb6: push r15
1412a7fb8: push r14
1412a7fba: push r13
1412a7fbc: push r12
1412a7fbe: push rsi
1412a7fbf: push rdi
1412a7fc0: push rbx
1412a7fc1: sub rsp, 0xa8
1412a7fc8: lea rbp, [rdx + 0x80]
1412a7fcf: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a7fd6: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a7fdd: movaps xmmword ptr [rsp + 0x40], xmm11
1412a7fe3: movaps xmmword ptr [rsp + 0x50], xmm10
1412a7fe9: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a7ff0: movaps xmmword ptr [rsp + 0x70], xmm8
1412a7ff6: movaps xmmword ptr [rsp + 0x80], xmm7
1412a7ffe: movaps xmmword ptr [rsp + 0x90], xmm6
1412a8006: mov rax, qword ptr [rbp + 0x2a0]
1412a800d: lock dec qword ptr [rax]
1412a8011: jne 0x1412a801f
1412a8013: lea rcx, [rbp + 0x2a0]
1412a801a: call 0x140b5c830 ; alloc::sync::Arc::drop_slow<country_core::render::mesh::Mesh,alloc::alloc::Global>
1412a801f: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a8027: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a802f: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a8035: movaps xmm9, xmmword ptr [rsp + 0x60]
1412a803b: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a8041: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a8047: movaps xmm12, xmmword ptr [rsp + 0x30]
1412a804d: movaps xmm13, xmmword ptr [rsp + 0x20]
1412a8053: add rsp, 0xa8
1412a805a: pop rbx
1412a805b: pop rdi
1412a805c: pop rsi
1412a805d: pop r12
1412a805f: pop r13
1412a8061: pop r14
1412a8063: pop r15
1412a8065: pop rbp
1412a8066: ret 
1412a8067: nop word ptr [rax + rax]
1412a8070: mov qword ptr [rsp + 0x10], rdx
1412a8075: push rbp
1412a8076: push r15
1412a8078: push r14
1412a807a: push r13
1412a807c: push r12
1412a807e: push rsi
1412a807f: push rdi
1412a8080: push rbx
1412a8081: sub rsp, 0xa8
1412a8088: lea rbp, [rdx + 0x80]
1412a808f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a8096: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a809d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a80a3: movaps xmmword ptr [rsp + 0x50], xmm10
1412a80a9: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a80b0: movaps xmmword ptr [rsp + 0x70], xmm8
1412a80b6: movaps xmmword ptr [rsp + 0x80], xmm7
1412a80be: movaps xmmword ptr [rsp + 0x90], xmm6
1412a80c6: mov rax, qword ptr [rbp + 0x4c8]
1412a80cd: lock dec qword ptr [rax]
1412a80d1: jne 0x1412a80df
1412a80d3: lea rcx, [rbp + 0x360]
1412a80da: call 0x140b5c830 ; alloc::sync::Arc::drop_slow<country_core::render::mesh::Mesh,alloc::alloc::Global>
1412a80df: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a80e7: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a80ef: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a80f5: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a80fc: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a8102: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a8108: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a810f: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a8116: add rsp, 0xa8
1412a811d: pop rbx
1412a811e: pop rdi
1412a811f: pop rsi
1412a8120: pop r12
1412a8122: pop r13
1412a8124: pop r14
1412a8126: pop r15
1412a8128: pop rbp
1412a8129: ret 
1412a812a: nop word ptr [rax + rax]
1412a8130: mov qword ptr [rsp + 0x10], rdx
1412a8135: push rbp
1412a8136: push r15
1412a8138: push r14
1412a813a: push r13
1412a813c: push r12
1412a813e: push rsi
1412a813f: push rdi
1412a8140: push rbx
1412a8141: sub rsp, 0xa8
1412a8148: lea rbp, [rdx + 0x80]
1412a814f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a8156: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a815d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a8163: movaps xmmword ptr [rsp + 0x50], xmm10
1412a8169: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a8170: movaps xmmword ptr [rsp + 0x70], xmm8
1412a8176: movaps xmmword ptr [rsp + 0x80], xmm7
1412a817e: movaps xmmword ptr [rsp + 0x90], xmm6
1412a8186: movzx ebx, byte ptr [rbp + 0x4d1]
1412a818d: test bl, 1
1412a8190: je 0x1412a8210
1412a8192: mov rax, qword ptr [rbp + 0x318]
1412a8199: lea rax, [rax*2]
1412a81a1: test rax, rax
1412a81a4: je 0x1412a81bf
1412a81a6: mov r8d, 1
1412a81ac: mov rcx, qword ptr [rbp + 0x1a8]
1412a81b3: mov rdx, qword ptr [rbp + 0x318]
1412a81ba: call 0x140613c20
1412a81bf: mov byte ptr [rbp + 0x4e7], bl
1412a81c5: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a81cd: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a81d5: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a81db: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a81e2: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a81e8: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a81ee: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a81f5: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a81fc: add rsp, 0xa8
1412a8203: pop rbx
1412a8204: pop rdi
1412a8205: pop rsi
1412a8206: pop r12
1412a8208: pop r13
1412a820a: pop r14
1412a820c: pop r15
1412a820e: pop rbp
1412a820f: ret 
1412a8210: mov byte ptr [rbp + 0x4e7], bl
1412a8216: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a821e: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a8226: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a822c: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a8233: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a8239: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a823f: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a8246: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a824d: add rsp, 0xa8
1412a8254: pop rbx
1412a8255: pop rdi
1412a8256: pop rsi
1412a8257: pop r12
1412a8259: pop r13
1412a825b: pop r14
1412a825d: pop r15
1412a825f: pop rbp
1412a8260: ret 
1412a8261: nop word ptr cs:[rax + rax]
1412a8270: mov qword ptr [rsp + 0x10], rdx
1412a8275: push rbp
1412a8276: push r15
1412a8278: push r14
1412a827a: push r13
1412a827c: push r12
1412a827e: push rsi
1412a827f: push rdi
1412a8280: push rbx
1412a8281: sub rsp, 0xa8
1412a8288: lea rbp, [rdx + 0x80]
1412a828f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a8296: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a829d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a82a3: movaps xmmword ptr [rsp + 0x50], xmm10
1412a82a9: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a82b0: movaps xmmword ptr [rsp + 0x70], xmm8
1412a82b6: movaps xmmword ptr [rsp + 0x80], xmm7
1412a82be: movaps xmmword ptr [rsp + 0x90], xmm6
1412a82c6: test byte ptr [rbp + 0x4e7], 1
1412a82cd: je 0x1412a8333
1412a82cf: mov rax, qword ptr [rbp + 0x1b0]
1412a82d6: lock dec qword ptr [rax]
1412a82da: jne 0x1412a82e8
1412a82dc: lea rcx, [rbp + 0x1b0]
1412a82e3: call 0x140b5c830 ; alloc::sync::Arc::drop_slow<country_core::render::mesh::Mesh,alloc::alloc::Global>
1412a82e8: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a82f0: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a82f8: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a82fe: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a8305: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a830b: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a8311: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a8318: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a831f: add rsp, 0xa8
1412a8326: pop rbx
1412a8327: pop rdi
1412a8328: pop rsi
1412a8329: pop r12
1412a832b: pop r13
1412a832d: pop r14
1412a832f: pop r15
1412a8331: pop rbp
1412a8332: ret 
1412a8333: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a833b: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a8343: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a8349: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a8350: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a8356: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a835c: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a8363: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a836a: add rsp, 0xa8
1412a8371: pop rbx
1412a8372: pop rdi
1412a8373: pop rsi
1412a8374: pop r12
1412a8376: pop r13
1412a8378: pop r14
1412a837a: pop r15
1412a837c: pop rbp
1412a837d: ret 
1412a837e: nop 
1412a8380: mov qword ptr [rsp + 0x10], rdx
1412a8385: push rbp
1412a8386: push r15
1412a8388: push r14
1412a838a: push r13
1412a838c: push r12
1412a838e: push rsi
1412a838f: push rdi
1412a8390: push rbx
1412a8391: sub rsp, 0xa8
1412a8398: lea rbp, [rdx + 0x80]
1412a839f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a83a6: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a83ad: movaps xmmword ptr [rsp + 0x40], xmm11
1412a83b3: movaps xmmword ptr [rsp + 0x50], xmm10
1412a83b9: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a83c0: movaps xmmword ptr [rsp + 0x70], xmm8
1412a83c6: movaps xmmword ptr [rsp + 0x80], xmm7
1412a83ce: movaps xmmword ptr [rsp + 0x90], xmm6
1412a83d6: cmp qword ptr [rbp + 0x220], 0
1412a83de: jne 0x1412a8439
1412a83e0: mov byte ptr [rbp + 0x4e5], 1
1412a83e7: mov byte ptr [rbp + 0x4e4], 1
1412a83ee: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a83f6: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a83fe: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a8404: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a840b: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a8411: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a8417: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a841e: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a8425: add rsp, 0xa8
1412a842c: pop rbx
1412a842d: pop rdi
1412a842e: pop rsi
1412a842f: pop r12
1412a8431: pop r13
1412a8433: pop r14
1412a8435: pop r15
1412a8437: pop rbp
1412a8438: ret 
1412a8439: mov rsi, qword ptr [rbp + 0x238]
1412a8440: mov rdi, qword ptr [rbp + 0x240]
1412a8447: xor ebx, ebx
1412a8449: mov rcx, rsi
1412a844c: cmp rdi, rbx
1412a844f: je 0x1412a8477
1412a8451: nop word ptr cs:[rax + rax]
1412a8460: inc rbx
1412a8463: lea r14, [rcx + 0x98]
1412a846a: call 0x14088bca0 ; core::ptr::drop_in_place<country_core::resources::prefab::PrefabPart>
1412a846f: mov rcx, r14
1412a8472: cmp rdi, rbx
1412a8475: jne 0x1412a8460
1412a8477: mov rax, qword ptr [rbp + 0x230]
1412a847e: test rax, rax
1412a8481: je 0x1412a8498
1412a8483: imul rdx, rax, 0x98
1412a848a: mov r8d, 8
1412a8490: mov rcx, rsi
1412a8493: call 0x140613c20
1412a8498: mov byte ptr [rbp + 0x4e5], 1
1412a849f: mov byte ptr [rbp + 0x4e4], 1
1412a84a6: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a84ae: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a84b6: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a84bc: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a84c3: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a84c9: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a84cf: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a84d6: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a84dd: add rsp, 0xa8
1412a84e4: pop rbx
1412a84e5: pop rdi
1412a84e6: pop rsi
1412a84e7: pop r12
1412a84e9: pop r13
1412a84eb: pop r14
1412a84ed: pop r15
1412a84ef: pop rbp
1412a84f0: ret 
1412a84f1: nop word ptr cs:[rax + rax]
1412a8500: mov qword ptr [rsp + 0x10], rdx
1412a8505: push rbp
1412a8506: push r15
1412a8508: push r14
1412a850a: push r13
1412a850c: push r12
1412a850e: push rsi
1412a850f: push rdi
1412a8510: push rbx
1412a8511: sub rsp, 0xa8
1412a8518: lea rbp, [rdx + 0x80]
1412a851f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a8526: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a852d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a8533: movaps xmmword ptr [rsp + 0x50], xmm10
1412a8539: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a8540: movaps xmmword ptr [rsp + 0x70], xmm8
1412a8546: movaps xmmword ptr [rsp + 0x80], xmm7
1412a854e: movaps xmmword ptr [rsp + 0x90], xmm6
1412a8556: movzx ebx, byte ptr [rbp + 0x4dc]
1412a855d: cmp byte ptr [rbp + 0x4db], 0
1412a8564: je 0x1412a861a
1412a856a: mov rsi, qword ptr [rbp + 0x250]
1412a8571: mov rdi, qword ptr [rbp + 0x258]
1412a8578: xor r14d, r14d
1412a857b: mov rcx, rsi
1412a857e: cmp rdi, r14
1412a8581: je 0x1412a85a4
1412a8583: nop word ptr cs:[rax + rax]
1412a8590: inc r14
1412a8593: lea r15, [rcx + 0x28]
1412a8597: call 0x1408989d0 ; core::ptr::drop_in_place<tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab> >
1412a859c: mov rcx, r15
1412a859f: cmp rdi, r14
1412a85a2: jne 0x1412a8590
1412a85a4: mov rax, qword ptr [rbp + 0x248]
1412a85ab: test rax, rax
1412a85ae: je 0x1412a85c6
1412a85b0: shl rax, 3
1412a85b4: lea rdx, [rax + rax*4]
1412a85b8: mov r8d, 8
1412a85be: mov rcx, rsi
1412a85c1: call 0x140613c20
1412a85c6: and bl, 1
1412a85c9: mov byte ptr [rbp + 0x4d8], bl
1412a85cf: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a85d7: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a85df: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a85e5: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a85ec: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a85f2: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a85f8: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a85ff: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a8606: add rsp, 0xa8
1412a860d: pop rbx
1412a860e: pop rdi
1412a860f: pop rsi
1412a8610: pop r12
1412a8612: pop r13
1412a8614: pop r14
1412a8616: pop r15
1412a8618: pop rbp
1412a8619: ret 
1412a861a: and bl, 1
1412a861d: mov byte ptr [rbp + 0x4d8], bl
1412a8623: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a862b: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a8633: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a8639: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a8640: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a8646: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a864c: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a8653: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a865a: add rsp, 0xa8
1412a8661: pop rbx
1412a8662: pop rdi
1412a8663: pop rsi
1412a8664: pop r12
1412a8666: pop r13
1412a8668: pop r14
1412a866a: pop r15
1412a866c: pop rbp
1412a866d: ret 
1412a866e: nop 
1412a8670: mov qword ptr [rsp + 0x10], rdx
1412a8675: push rbp
1412a8676: push r15
1412a8678: push r14
1412a867a: push r13
1412a867c: push r12
1412a867e: push rsi
1412a867f: push rdi
1412a8680: push rbx
1412a8681: sub rsp, 0xa8
1412a8688: lea rbp, [rdx + 0x80]
1412a868f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a8696: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a869d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a86a3: movaps xmmword ptr [rsp + 0x50], xmm10
1412a86a9: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a86b0: movaps xmmword ptr [rsp + 0x70], xmm8
1412a86b6: movaps xmmword ptr [rsp + 0x80], xmm7
1412a86be: movaps xmmword ptr [rsp + 0x90], xmm6
1412a86c6: lea rdx, [rbp + 0xf0]
1412a86cd: movzx ebx, byte ptr [rbp + 0x4d8]
1412a86d4: lea rcx, [rbp + 0xd0]
1412a86db: mov r8d, 0x40
1412a86e1: mov r9d, 0x10
1412a86e7: call 0x140b75590 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<asset::prefab::PrefabName,asset::prefab::config::PrefabConfig>,alloc::alloc::Global>
1412a86ec: test bl, bl
1412a86ee: je 0x1412a875a
1412a86f0: lea rcx, [rbp + 0x100]
1412a86f7: lea rdx, [rbp + 0x120]
1412a86fe: mov r8d, 0x38
1412a8704: mov r9d, 0x10
1412a870a: call 0x140b74970 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<std::path::PathBuf,alloc::vec::Vec<asset::prefab::PrefabName,alloc::alloc::Global> >,alloc::alloc::Global>
1412a870f: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a8717: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a871f: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a8725: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a872c: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a8732: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a8738: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a873f: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a8746: add rsp, 0xa8
1412a874d: pop rbx
1412a874e: pop rdi
1412a874f: pop rsi
1412a8750: pop r12
1412a8752: pop r13
1412a8754: pop r14
1412a8756: pop r15
1412a8758: pop rbp
1412a8759: ret 
1412a875a: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a8762: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a876a: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a8770: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a8777: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a877d: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a8783: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a878a: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a8791: add rsp, 0xa8
1412a8798: pop rbx
1412a8799: pop rdi
1412a879a: pop rsi
1412a879b: pop r12
1412a879d: pop r13
1412a879f: pop r14
1412a87a1: pop r15
1412a87a3: pop rbp
1412a87a4: ret 
1412a87a5: nop word ptr cs:[rax + rax]
1412a87b0: mov qword ptr [rsp + 0x10], rdx
1412a87b5: push rbp
1412a87b6: push r15
1412a87b8: push r14
1412a87ba: push r13
1412a87bc: push r12
1412a87be: push rsi
1412a87bf: push rdi
1412a87c0: push rbx
1412a87c1: sub rsp, 0xa8
1412a87c8: lea rbp, [rdx + 0x80]
1412a87cf: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a87d6: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a87dd: movaps xmmword ptr [rsp + 0x40], xmm11
1412a87e3: movaps xmmword ptr [rsp + 0x50], xmm10
1412a87e9: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a87f0: movaps xmmword ptr [rsp + 0x70], xmm8
1412a87f6: movaps xmmword ptr [rsp + 0x80], xmm7
1412a87fe: movaps xmmword ptr [rsp + 0x90], xmm6
1412a8806: test byte ptr [rbp + 0x4e6], 1
1412a880d: je 0x1412a8873
1412a880f: lea rcx, [rbp + 0x40]
1412a8813: mov r8d, 0x10
1412a8819: mov r9d, 0x10
1412a881f: lea rdx, [rbp + 0x60]
1412a8823: call 0x140b73f00 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<asset::prefab::PrefabName,tuple$<> >,alloc::alloc::Global>
1412a8828: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a8830: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a8838: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a883e: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a8845: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a884b: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a8851: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a8858: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a885f: add rsp, 0xa8
1412a8866: pop rbx
1412a8867: pop rdi
1412a8868: pop rsi
1412a8869: pop r12
1412a886b: pop r13
1412a886d: pop r14
1412a886f: pop r15
1412a8871: pop rbp
1412a8872: ret 
1412a8873: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a887b: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a8883: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a8889: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a8890: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a8896: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a889c: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a88a3: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a88aa: add rsp, 0xa8
1412a88b1: pop rbx
1412a88b2: pop rdi
1412a88b3: pop rsi
1412a88b4: pop r12
1412a88b6: pop r13
1412a88b8: pop r14
1412a88ba: pop r15
1412a88bc: pop rbp
1412a88bd: ret 
1412a88be: nop 
1412a88c0: mov qword ptr [rsp + 0x10], rdx
1412a88c5: push rbp
1412a88c6: push r15
1412a88c8: push r14
1412a88ca: push r13
1412a88cc: push r12
1412a88ce: push rsi
1412a88cf: push rdi
1412a88d0: push rbx
1412a88d1: sub rsp, 0xa8
1412a88d8: lea rbp, [rdx + 0x80]
1412a88df: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a88e6: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a88ed: movaps xmmword ptr [rsp + 0x40], xmm11
1412a88f3: movaps xmmword ptr [rsp + 0x50], xmm10
1412a88f9: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a8900: movaps xmmword ptr [rsp + 0x70], xmm8
1412a8906: movaps xmmword ptr [rsp + 0x80], xmm7
1412a890e: movaps xmmword ptr [rsp + 0x90], xmm6
1412a8916: mov byte ptr [rbp + 0x4e6], 1
1412a891d: lea rcx, [rbp + 0x430]
1412a8924: call 0x1405d2870 ; core::ptr::drop_in_place<std::sync::poison::mutex::MutexGuard<winit::platform_impl::platform::window_state::WindowState> >
1412a8929: mov byte ptr [rbp + 0x4e6], 1
1412a8930: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a8938: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a8940: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a8946: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a894d: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a8953: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a8959: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a8960: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a8967: add rsp, 0xa8
1412a896e: pop rbx
1412a896f: pop rdi
1412a8970: pop rsi
1412a8971: pop r12
1412a8973: pop r13
1412a8975: pop r14
1412a8977: pop r15
1412a8979: pop rbp
1412a897a: ret 
1412a897b: nop dword ptr [rax + rax]
1412a8980: mov qword ptr [rsp + 0x10], rdx
1412a8985: push rbp
1412a8986: push r15
1412a8988: push r14
1412a898a: push r13
1412a898c: push r12
1412a898e: push rsi
1412a898f: push rdi
1412a8990: push rbx
1412a8991: sub rsp, 0xa8
1412a8998: lea rbp, [rdx + 0x80]
1412a899f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a89a6: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a89ad: movaps xmmword ptr [rsp + 0x40], xmm11
1412a89b3: movaps xmmword ptr [rsp + 0x50], xmm10
1412a89b9: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a89c0: movaps xmmword ptr [rsp + 0x70], xmm8
1412a89c6: movaps xmmword ptr [rsp + 0x80], xmm7
1412a89ce: movaps xmmword ptr [rsp + 0x90], xmm6
1412a89d6: mov rax, qword ptr [rbp + 0x3a8]
1412a89dd: mov qword ptr [rbp + 0x438], rax
1412a89e4: lea rcx, [rbp + 0x430]
1412a89eb: call 0x14087dcd0 ; core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<asset::prefab::PrefabName,country_core::resources::prefab::Prefab>,alloc::alloc::Global> >
1412a89f0: mov byte ptr [rbp + 0x4e5], 1
1412a89f7: mov byte ptr [rbp + 0x4e4], 0
1412a89fe: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a8a06: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a8a0e: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a8a14: movaps xmm9, xmmword ptr [rsp + 0x60]
1412a8a1a: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a8a20: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a8a26: movaps xmm12, xmmword ptr [rsp + 0x30]
1412a8a2c: movaps xmm13, xmmword ptr [rsp + 0x20]
1412a8a32: add rsp, 0xa8
1412a8a39: pop rbx
1412a8a3a: pop rdi
1412a8a3b: pop rsi
1412a8a3c: pop r12
1412a8a3e: pop r13
1412a8a40: pop r14
1412a8a42: pop r15
1412a8a44: pop rbp
1412a8a45: ret 
1412a8a46: nop word ptr cs:[rax + rax]
1412a8a50: mov qword ptr [rsp + 0x10], rdx
1412a8a55: push rbp
1412a8a56: push r15
1412a8a58: push r14
1412a8a5a: push r13
1412a8a5c: push r12
1412a8a5e: push rsi
1412a8a5f: push rdi
1412a8a60: push rbx
1412a8a61: sub rsp, 0xa8
1412a8a68: lea rbp, [rdx + 0x80]
1412a8a6f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a8a76: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a8a7d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a8a83: movaps xmmword ptr [rsp + 0x50], xmm10
1412a8a89: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a8a90: movaps xmmword ptr [rsp + 0x70], xmm8
1412a8a96: movaps xmmword ptr [rsp + 0x80], xmm7
1412a8a9e: movaps xmmword ptr [rsp + 0x90], xmm6
1412a8aa6: lea rcx, [rbp + 0x320]
1412a8aad: call 0x1408b9aa0 ; core::ptr::drop_in_place<enum2$<hotwatch::Error> >
1412a8ab2: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a8aba: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a8ac2: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a8ac8: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a8acf: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a8ad5: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a8adb: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a8ae2: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a8ae9: add rsp, 0xa8
1412a8af0: pop rbx
1412a8af1: pop rdi
1412a8af2: pop rsi
1412a8af3: pop r12
1412a8af5: pop r13
1412a8af7: pop r14
1412a8af9: pop r15
1412a8afb: pop rbp
1412a8afc: ret 
1412a8afd: nop dword ptr [rax]
1412a8b00: mov qword ptr [rsp + 0x10], rdx
1412a8b05: push rbp
1412a8b06: push r15
1412a8b08: push r14
1412a8b0a: push r13
1412a8b0c: push r12
1412a8b0e: push rsi
1412a8b0f: push rdi
1412a8b10: push rbx
1412a8b11: sub rsp, 0xa8
1412a8b18: lea rbp, [rdx + 0x80]
1412a8b1f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a8b26: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a8b2d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a8b33: movaps xmmword ptr [rsp + 0x50], xmm10
1412a8b39: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a8b40: movaps xmmword ptr [rsp + 0x70], xmm8
1412a8b46: movaps xmmword ptr [rsp + 0x80], xmm7
1412a8b4e: movaps xmmword ptr [rsp + 0x90], xmm6
1412a8b56: mov byte ptr [rbp + 0x4e3], 0
1412a8b5d: lea rcx, [rbp + 0x430]
1412a8b64: call 0x140d4cc30 ; anyhow::error::impl$6::drop
1412a8b69: mov byte ptr [rbp + 0x4e3], 0
1412a8b70: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a8b78: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a8b80: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a8b86: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a8b8d: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a8b93: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a8b99: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a8ba0: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a8ba7: add rsp, 0xa8
1412a8bae: pop rbx
1412a8baf: pop rdi
1412a8bb0: pop rsi
1412a8bb1: pop r12
1412a8bb3: pop r13
1412a8bb5: pop r14
1412a8bb7: pop r15
1412a8bb9: pop rbp
1412a8bba: ret 
1412a8bbb: nop dword ptr [rax + rax]
1412a8bc0: mov qword ptr [rsp + 0x10], rdx
1412a8bc5: push rbp
1412a8bc6: push r15
1412a8bc8: push r14
1412a8bca: push r13
1412a8bcc: push r12
1412a8bce: push rsi
1412a8bcf: push rdi
1412a8bd0: push rbx
1412a8bd1: sub rsp, 0xa8
1412a8bd8: lea rbp, [rdx + 0x80]
1412a8bdf: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a8be6: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a8bed: movaps xmmword ptr [rsp + 0x40], xmm11
1412a8bf3: movaps xmmword ptr [rsp + 0x50], xmm10
1412a8bf9: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a8c00: movaps xmmword ptr [rsp + 0x70], xmm8
1412a8c06: movaps xmmword ptr [rsp + 0x80], xmm7
1412a8c0e: movaps xmmword ptr [rsp + 0x90], xmm6
1412a8c16: lea rcx, [rbp + 0x430]
1412a8c1d: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
1412a8c22: mov byte ptr [rbp + 0x4e2], 1
1412a8c29: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a8c31: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a8c39: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a8c3f: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a8c46: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a8c4c: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a8c52: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a8c59: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a8c60: add rsp, 0xa8
1412a8c67: pop rbx
1412a8c68: pop rdi
1412a8c69: pop rsi
1412a8c6a: pop r12
1412a8c6c: pop r13
1412a8c6e: pop r14
1412a8c70: pop r15
1412a8c72: pop rbp
1412a8c73: ret 
1412a8c74: nop word ptr cs:[rax + rax]
1412a8c80: mov qword ptr [rsp + 0x10], rdx
1412a8c85: push rbp
1412a8c86: push r15
1412a8c88: push r14
1412a8c8a: push r13
1412a8c8c: push r12
1412a8c8e: push rsi
1412a8c8f: push rdi
1412a8c90: push rbx
1412a8c91: sub rsp, 0xa8
1412a8c98: lea rbp, [rdx + 0x80]
1412a8c9f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a8ca6: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a8cad: movaps xmmword ptr [rsp + 0x40], xmm11
1412a8cb3: movaps xmmword ptr [rsp + 0x50], xmm10
1412a8cb9: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a8cc0: movaps xmmword ptr [rsp + 0x70], xmm8
1412a8cc6: movaps xmmword ptr [rsp + 0x80], xmm7
1412a8cce: movaps xmmword ptr [rsp + 0x90], xmm6
1412a8cd6: lea rcx, [rbp + 0x430]
1412a8cdd: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
1412a8ce2: mov byte ptr [rbp + 0x4e2], 1
1412a8ce9: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a8cf1: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a8cf9: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a8cff: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a8d06: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a8d0c: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a8d12: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a8d19: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a8d20: add rsp, 0xa8
1412a8d27: pop rbx
1412a8d28: pop rdi
1412a8d29: pop rsi
1412a8d2a: pop r12
1412a8d2c: pop r13
1412a8d2e: pop r14
1412a8d30: pop r15
1412a8d32: pop rbp
1412a8d33: ret 
1412a8d34: nop word ptr cs:[rax + rax]
1412a8d40: mov qword ptr [rsp + 0x10], rdx
1412a8d45: push rbp
1412a8d46: push r15
1412a8d48: push r14
1412a8d4a: push r13
1412a8d4c: push r12
1412a8d4e: push rsi
1412a8d4f: push rdi
1412a8d50: push rbx
1412a8d51: sub rsp, 0xa8
1412a8d58: lea rbp, [rdx + 0x80]
1412a8d5f: movdqa xmmword ptr [rsp + 0x20], xmm13
1412a8d66: movdqa xmmword ptr [rsp + 0x30], xmm12
1412a8d6d: movaps xmmword ptr [rsp + 0x40], xmm11
1412a8d73: movaps xmmword ptr [rsp + 0x50], xmm10
1412a8d79: movdqa xmmword ptr [rsp + 0x60], xmm9
1412a8d80: movaps xmmword ptr [rsp + 0x70], xmm8
1412a8d86: movaps xmmword ptr [rsp + 0x80], xmm7
1412a8d8e: movaps xmmword ptr [rsp + 0x90], xmm6
1412a8d96: mov byte ptr [rbp + 0x4d1], 0
1412a8d9d: lea rcx, [rbp + 0x430]
1412a8da4: call 0x14088bca0 ; core::ptr::drop_in_place<country_core::resources::prefab::PrefabPart>
1412a8da9: mov byte ptr [rbp + 0x4d1], 0
1412a8db0: movaps xmm6, xmmword ptr [rsp + 0x90]
1412a8db8: movaps xmm7, xmmword ptr [rsp + 0x80]
1412a8dc0: movaps xmm8, xmmword ptr [rsp + 0x70]
1412a8dc6: movdqa xmm9, xmmword ptr [rsp + 0x60]
1412a8dcd: movaps xmm10, xmmword ptr [rsp + 0x50]
1412a8dd3: movaps xmm11, xmmword ptr [rsp + 0x40]
1412a8dd9: movdqa xmm12, xmmword ptr [rsp + 0x30]
1412a8de0: movdqa xmm13, xmmword ptr [rsp + 0x20]
1412a8de7: add rsp, 0xa8
1412a8dee: pop rbx
1412a8def: pop rdi
1412a8df0: pop rsi
1412a8df1: pop r12
1412a8df3: pop r13
1412a8df5: pop r14
1412a8df7: pop r15
1412a8df9: pop rbp
1412a8dfa: ret 
