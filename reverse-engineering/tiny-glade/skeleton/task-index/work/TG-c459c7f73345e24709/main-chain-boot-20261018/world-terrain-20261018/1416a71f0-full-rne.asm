1416a71f0: push rbp
1416a71f1: push r15
1416a71f3: push r14
1416a71f5: push r13
1416a71f7: push r12
1416a71f9: push rsi
1416a71fa: push rdi
1416a71fb: push rbx
1416a71fc: sub rsp, 0x418
1416a7203: lea rbp, [rsp + 0x80]
1416a720b: mov qword ptr [rbp + 0x390], 0xfffffffffffffffe
1416a7216: mov qword ptr [rbp + 0x378], r8
1416a721d: mov rbx, rdx
1416a7220: mov rsi, rcx
1416a7223: mov rdx, qword ptr [rbp + 0x400]
1416a722a: mov rax, qword ptr [rbp + 0x408]
1416a7231: mov rcx, qword ptr [rax + 0x10]
1416a7235: mov qword ptr [rbp + 0x1f0], rcx
1416a723c: movups xmm0, xmmword ptr [rax]
1416a723f: movaps xmmword ptr [rbp + 0x1e0], xmm0
1416a7246: mov eax, dword ptr [rdx]
1416a7248: test eax, eax
1416a724a: je 0x1416a794b
1416a7250: mov r14, r9
1416a7253: mov rdi, qword ptr [rbp + 0x1e0]
1416a725a: test rdi, rdi
1416a725d: je 0x1416a7264
1416a725f: or eax, 2
1416a7262: mov dword ptr [rdx], eax
1416a7264: test rdi, rdi
1416a7267: setne r8b
1416a726b: lea rcx, [rbp + 0x78]
1416a726f: call 0x140bd5ed0 ; rhapsody::raw::texture::get_image_create_info
1416a7274: cmp dword ptr [rbp + 0x94], 0
1416a727b: je 0x1416a7963
1416a7281: cmp dword ptr [rbp + 0x98], 0
1416a7288: je 0x1416a797b
1416a728e: cmp dword ptr [rbp + 0x9c], 0
1416a7295: je 0x1416a7993
1416a729b: mov rcx, qword ptr [rbx + 0x640]
1416a72a2: lea rdx, [rbp + 0x78]
1416a72a6: lea r9, [rbp + 0x270]
1416a72ad: xor r8d, r8d
1416a72b0: call qword ptr [rbx + 0x1a0]
1416a72b6: test eax, eax
1416a72b8: jne 0x1416a79d4
1416a72be: mov r12, qword ptr [rbp + 0x270]
1416a72c5: mov rcx, rbx
1416a72c8: call 0x140bf8960 ; rhapsody::raw::device::GraphicsDevice::debug_utils
1416a72cd: test rax, rax
1416a72d0: je 0x1416a737f
1416a72d6: mov r13, rax
1416a72d9: mov r8, r14
1416a72dc: mov qword ptr [rbp + 0x380], rsi
1416a72e3: mov rsi, r12
1416a72e6: lea rcx, [rbp + 0xd0]
1416a72ed: mov rdx, qword ptr [rbp + 0x378]
1416a72f4: call 0x140d797d0 ; alloc::ffi::c_str::impl$1::new::impl$2::spec_new_impl
1416a72f9: xor eax, eax
1416a72fb: cmp rax, qword ptr [rbp + 0xd0]
1416a7302: jno 0x1416a7a42
1416a7308: mov r12, qword ptr [rbp + 0xd8]
1416a730f: mov r15, qword ptr [rbp + 0xe0]
1416a7316: mov dword ptr [rbp + 0x270], 0x3b9cbe00
1416a7320: mov qword ptr [rbp + 0x278], 0
1416a732b: mov dword ptr [rbp + 0x280], 0xa
1416a7335: mov qword ptr [rbp + 0x288], rsi
1416a733c: mov qword ptr [rbp + 0x290], r12
1416a7343: mov rcx, qword ptr [r13 + 0x40]
1416a7347: lea rdx, [rbp + 0x270]
1416a734e: call qword ptr [r13]
1416a7352: test eax, eax
1416a7354: jne 0x1416a7a8b
1416a735a: mov byte ptr [r12], 0
1416a735f: test r15, r15
1416a7362: mov rcx, r12
1416a7365: mov r12, rsi
1416a7368: mov rsi, qword ptr [rbp + 0x380]
1416a736f: je 0x1416a737f
1416a7371: mov r8d, 1
1416a7377: mov rdx, r15
1416a737a: call 0x140613c20 ; ?
1416a737f: mov r15, r14
1416a7382: mov rcx, qword ptr [rbx + 0x640]
1416a7389: lea r8, [rbp + 0x350]
1416a7390: mov rdx, r12
1416a7393: call qword ptr [rbx + 0xf0]
1416a7399: mov r13, qword ptr [rbx + 0x678]
1416a73a0: lea r14, [r13 + 0x10]
1416a73a4: mov cl, 1
1416a73a6: xor eax, eax
1416a73a8: lock cmpxchg byte ptr [r13 + 0x10], cl
1416a73ae: jne 0x1416a79ab
1416a73b4: mov qword ptr [rbp + 0x368], r14
1416a73bb: mov rcx, r14
1416a73be: call 0x140c81c70 ; parking_lot_core::parking_lot::deadlock_impl::acquire_resource
1416a73c3: add r13, 0x18
1416a73c7: mov rax, qword ptr [rbp + 0x378]
1416a73ce: mov qword ptr [rbp + 0xe0], rax
1416a73d5: mov r14, r15
1416a73d8: mov qword ptr [rbp + 0xe8], r15
1416a73df: movups xmm0, xmmword ptr [rbp + 0x350]
1416a73e6: movups xmmword ptr [rbp + 0xf0], xmm0
1416a73ed: mov rax, qword ptr [rbp + 0x360]
1416a73f4: mov qword ptr [rbp + 0x100], rax
1416a73fb: mov word ptr [rbp + 0x108], 0x100
1416a7404: mov qword ptr [rbp + 0xd0], 2
1416a740f: lea rcx, [rbp + 0x270]
1416a7416: lea r8, [rbp + 0xd0]
1416a741d: mov rdx, r13
1416a7420: call 0x140c0da40 ; rhapsody::raw::allocator::GpuAllocator::allocate
1416a7425: mov qword ptr [rbp + 0x268], rdi
1416a742c: cmp byte ptr [rbp + 0x2b4], 2
1416a7433: mov rax, qword ptr [rbp + 0x270]
1416a743a: mov qword ptr [rbp + 0x380], rax
1416a7441: mov r13, qword ptr [rbp + 0x288]
1416a7448: jne 0x1416a74e8
1416a744e: mov rax, qword ptr [rbp + 0x278]
1416a7455: mov qword ptr [rbp + 0x370], rax
1416a745c: mov r15, qword ptr [rbp + 0x280]
1416a7463: movzx eax, byte ptr [rip + 0x1f8d79f] ; 0x143634c09 b''
1416a746a: mov edi, 0xf
1416a746f: mov ecx, 0xf
1416a7474: mov edx, 1
1416a7479: call 0x140613c10 ; ?
1416a747e: test rax, rax
1416a7481: je 0x1416a7acc
1416a7487: movabs rcx, 0x6572757478657420
1416a7491: mov qword ptr [rax + 7], rcx
1416a7495: movabs rcx, 0x20796c6e4f757047
1416a749f: mov qword ptr [rax], rcx
1416a74a2: mov rcx, qword ptr [rbp + 0x360]
1416a74a9: mov qword ptr [rbp + 0x1a0], rcx
1416a74b0: movups xmm0, xmmword ptr [rbp + 0x350]
1416a74b7: movaps xmmword ptr [rbp + 0x190], xmm0
1416a74be: mov rcx, qword ptr [rbp + 0x380]
1416a74c5: cmp rcx, 0xb
1416a74c9: jne 0x1416a7863
1416a74cf: mov rcx, r13
1416a74d2: mov rdx, qword ptr [rbp + 0x370]
1416a74d9: mov qword ptr [rbp + 0x380], rdx
1416a74e0: mov r13d, 0xf
1416a74e6: jmp 0x1416a7520
1416a74e8: mov r15, qword ptr [rbp + 0x278]
1416a74ef: mov rcx, qword ptr [rbp + 0x280]
1416a74f6: mov rax, qword ptr [rbp + 0x290]
1416a74fd: mov rdi, qword ptr [rbp + 0x298]
1416a7504: movups xmm0, xmmword ptr [rbp + 0x2a0]
1416a750b: movaps xmmword ptr [rbp + 0x190], xmm0
1416a7512: mov rdx, qword ptr [rbp + 0x2b0]
1416a7519: mov qword ptr [rbp + 0x1a0], rdx
1416a7520: mov rdx, qword ptr [rbp + 0x1a0]
1416a7527: mov qword ptr [rbp + 0x210], rdx
1416a752e: movaps xmm0, xmmword ptr [rbp + 0x190]
1416a7535: movaps xmmword ptr [rbp + 0x200], xmm0
1416a753c: mov r8, qword ptr [rbp + 0x380]
1416a7543: mov qword ptr [rbp + 0x220], r8
1416a754a: mov qword ptr [rbp + 0x228], r15
1416a7551: mov qword ptr [rbp + 0x230], rcx
1416a7558: movups xmmword ptr [rbp + 0x250], xmm0
1416a755f: mov qword ptr [rbp + 0x260], rdx
1416a7566: mov qword ptr [rbp + 0x238], r13
1416a756d: mov qword ptr [rbp + 0x240], rax
1416a7574: mov qword ptr [rbp + 0x248], rdi
1416a757b: mov byte ptr [rbp + 0x38f], 1
1416a7582: mov rdi, qword ptr [rbp + 0x368]
1416a7589: mov rcx, rdi
1416a758c: call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
1416a7591: xor ecx, ecx
1416a7593: mov al, 1
1416a7595: lock cmpxchg byte ptr [rdi], cl
1416a7599: jne 0x1416a79be
1416a759f: mov byte ptr [rbp + 0x38f], 1
1416a75a6: lea rcx, [rbp + 0x220]
1416a75ad: call 0x1406d0cb0 ; samply_symbols::windows::impl$5::function_count
1416a75b2: mov r13, rax
1416a75b5: mov byte ptr [rbp + 0x38f], 1
1416a75bc: lea rcx, [rbp + 0x220]
1416a75c3: call 0x1406868b0 ; toolbar::mei::core::NodeContext::id
1416a75c8: mov rcx, qword ptr [rbx + 0x640]
1416a75cf: mov rdx, r12
1416a75d2: mov r8, r13
1416a75d5: mov r9, rax
1416a75d8: call qword ptr [rbx + 0xe0]
1416a75de: test eax, eax
1416a75e0: mov rdx, qword ptr [rbp + 0x400]
1416a75e7: jne 0x1416a7a05
1416a75ed: lea rax, [rbx + 0x78]
1416a75f1: mov r13, qword ptr [rbx + 0x648]
1416a75f8: mov rcx, qword ptr [rbp + 0x260]
1416a75ff: mov qword ptr [rbp + 0x1d0], rcx
1416a7606: movups xmm0, xmmword ptr [rbp + 0x220]
1416a760d: movups xmm1, xmmword ptr [rbp + 0x230]
1416a7614: movups xmm2, xmmword ptr [rbp + 0x240]
1416a761b: movups xmm3, xmmword ptr [rbp + 0x250]
1416a7622: movaps xmmword ptr [rbp + 0x1c0], xmm3
1416a7629: movaps xmmword ptr [rbp + 0x1b0], xmm2
1416a7630: movaps xmmword ptr [rbp + 0x1a0], xmm1
1416a7637: movaps xmmword ptr [rbp + 0x190], xmm0
1416a763e: movups xmm0, xmmword ptr [rdx]
1416a7641: movups xmm1, xmmword ptr [rdx + 0x10]
1416a7645: movaps xmmword ptr [rbp + 0x210], xmm1
1416a764c: movaps xmmword ptr [rbp + 0x200], xmm0
1416a7653: mov byte ptr [rbp + 0x38f], 0
1416a765a: lea rcx, [rbp + 0x200]
1416a7661: mov qword ptr [rsp + 0x30], rcx
1416a7666: lea rcx, [rbp + 0x190]
1416a766d: mov qword ptr [rsp + 0x28], rcx
1416a7672: mov qword ptr [rsp + 0x20], rax
1416a7677: lea rcx, [rbp + 0x270]
1416a767e: mov rdx, qword ptr [rbp + 0x378]
1416a7685: mov r8, r14
1416a7688: mov r9, r12
1416a768b: call 0x140bdc2d0 ; rhapsody::raw::managed::texture::ManagedTexture::new
1416a7690: movzx edi, byte ptr [rbp + 0x32c]
1416a7697: cmp dil, 3
1416a769b: jne 0x1416a76fb
1416a769d: movups xmm0, xmmword ptr [rbp + 0x2b0]
1416a76a4: movaps xmmword ptr [rbp + 0x110], xmm0
1416a76ab: movups xmm1, xmmword ptr [rbp + 0x270]
1416a76b2: movups xmm2, xmmword ptr [rbp + 0x280]
1416a76b9: movups xmm3, xmmword ptr [rbp + 0x290]
1416a76c0: movups xmm4, xmmword ptr [rbp + 0x2a0]
1416a76c7: movaps xmmword ptr [rbp + 0x100], xmm4
1416a76ce: movaps xmmword ptr [rbp + 0xf0], xmm3
1416a76d5: movaps xmmword ptr [rbp + 0xe0], xmm2
1416a76dc: movaps xmmword ptr [rbp + 0xd0], xmm1
1416a76e3: movups xmmword ptr [rsi + 0x40], xmm0
1416a76e7: movups xmmword ptr [rsi + 0x30], xmm4
1416a76eb: movups xmmword ptr [rsi + 0x20], xmm3
1416a76ef: movups xmmword ptr [rsi + 0x10], xmm2
1416a76f3: movups xmmword ptr [rsi], xmm1
1416a76f6: jmp 0x1416a7934
1416a76fb: lea r14, [rbp + 0xd0]
1416a7702: lea rdx, [rbp + 0x270]
1416a7709: mov r8d, 0xbc
1416a770f: mov rcx, r14
1416a7712: call 0x142923c00 ; ?
1416a7717: movzx eax, word ptr [rbp + 0x32d]
1416a771e: mov word ptr [rbp + 0x75], ax
1416a7722: movzx eax, byte ptr [rbp + 0x32f]
1416a7729: mov byte ptr [rbp + 0x77], al
1416a772c: lea r15, [rbp - 0x48]
1416a7730: mov r8d, 0xbc
1416a7736: mov rcx, r15
1416a7739: mov rdx, r14
1416a773c: call 0x142923c00 ; ?
1416a7741: mov byte ptr [rbp + 0x74], dil
1416a7745: mov byte ptr [rbp + 0x38f], 0
1416a774c: mov rcx, r13
1416a774f: mov rdx, r15
1416a7752: call 0x140bdbd50 ; rhapsody::raw::managed::resources::impl$6::allocate
1416a7757: mov ecx, dword ptr [rax + 0xc]
1416a775a: movzx edx, word ptr [r13 + 0xa50]
1416a7762: mov r8, qword ptr [r13]
1416a7766: mov qword ptr [rbp + 0x338], r8
1416a776d: mov r8, rax
1416a7770: add r8, 0x10
1416a7774: mov qword ptr [rbp + 0x340], rax
1416a777b: mov dword ptr [rbp + 0x348], ecx
1416a7781: mov word ptr [rbp + 0x34c], dx
1416a7788: mov byte ptr [rbp + 0x34e], 0
1416a778f: mov qword ptr [rbp + 0x330], r8
1416a7796: cmp qword ptr [rbp + 0x268], 0
1416a779e: mov rdi, qword ptr [rbp + 0x400]
1416a77a5: je 0x1416a77f6
1416a77a7: movups xmm0, xmmword ptr [rdi]
1416a77aa: movups xmm1, xmmword ptr [rdi + 0x10]
1416a77ae: movaps xmmword ptr [rbp + 0xe0], xmm1
1416a77b5: movaps xmmword ptr [rbp + 0xd0], xmm0
1416a77bc: lea rax, [rbp + 0x330]
1416a77c3: mov qword ptr [rsp + 0x20], rax
1416a77c8: lea rcx, [rbp + 0x270]
1416a77cf: lea r8, [rbp + 0xd0]
1416a77d6: lea r9, [rbp + 0x1e0]
1416a77dd: mov rdx, rbx
1416a77e0: call 0x140bfe270 ; rhapsody::raw::device::GraphicsDevice::init_texture_data
1416a77e5: mov rax, qword ptr [rbp + 0x270]
1416a77ec: cmp rax, 0xb
1416a77f0: jne 0x1416a78e7
1416a77f6: mov rax, qword ptr [rbp + 0x348]
1416a77fd: mov qword ptr [rbp + 0x280], rax
1416a7804: movups xmm0, xmmword ptr [rbp + 0x338]
1416a780b: movaps xmmword ptr [rbp + 0x270], xmm0
1416a7812: movups xmm1, xmmword ptr [rdi]
1416a7815: movups xmm2, xmmword ptr [rdi + 0x10]
1416a7819: movups xmmword ptr [rbp + 0x298], xmm2
1416a7820: movups xmmword ptr [rbp + 0x288], xmm1
1416a7827: movups xmmword ptr [rsi + 8], xmm0
1416a782b: mov rax, qword ptr [rbp + 0x2a0]
1416a7832: mov qword ptr [rsi + 0x38], rax
1416a7836: movaps xmm0, xmmword ptr [rbp + 0x290]
1416a783d: movups xmmword ptr [rsi + 0x28], xmm0
1416a7841: mov rax, qword ptr [rbp + 0x280]
1416a7848: mov qword ptr [rsi + 0x18], rax
1416a784c: mov rax, qword ptr [rbp + 0x288]
1416a7853: mov qword ptr [rsi + 0x20], rax
1416a7857: mov qword ptr [rsi], 0xb
1416a785e: jmp 0x1416a7934
1416a7863: mov rdx, rcx
1416a7866: mov rcx, qword ptr [rbp + 0x360]
1416a786d: mov qword ptr [rbp + 0x210], rcx
1416a7874: movups xmm0, xmmword ptr [rbp + 0x350]
1416a787b: movaps xmmword ptr [rbp + 0x200], xmm0
1416a7882: mov rcx, qword ptr [rbp + 0x370]
1416a7889: mov qword ptr [rsi + 8], rcx
1416a788d: mov qword ptr [rsi + 0x10], r15
1416a7891: mov qword ptr [rsi + 0x18], r13
1416a7895: movups xmm0, xmmword ptr [rbp + 0x350]
1416a789c: movups xmmword ptr [rsi + 0x38], xmm0
1416a78a0: mov rcx, qword ptr [rbp + 0x360]
1416a78a7: mov qword ptr [rsi + 0x48], rcx
1416a78ab: mov qword ptr [rsi], rdx
1416a78ae: mov qword ptr [rsi + 0x20], 0xf
1416a78b6: mov qword ptr [rsi + 0x28], rax
1416a78ba: mov qword ptr [rsi + 0x30], 0xf
1416a78c2: mov rdi, qword ptr [rbp + 0x368]
1416a78c9: mov rcx, rdi
1416a78cc: call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
1416a78d1: xor ecx, ecx
1416a78d3: mov al, 1
1416a78d5: lock cmpxchg byte ptr [rdi], cl
1416a78d9: je 0x1416a7934
1416a78db: mov rcx, rdi
1416a78de: xor edx, edx
1416a78e0: call 0x1428cdb90 ; parking_lot::raw_mutex::RawMutex::unlock_slow
1416a78e5: jmp 0x1416a7934
1416a78e7: mov rcx, qword ptr [rbp + 0x2b8]
1416a78ee: mov qword ptr [rsi + 0x48], rcx
1416a78f2: movups xmm0, xmmword ptr [rbp + 0x278]
1416a78f9: movups xmm1, xmmword ptr [rbp + 0x288]
1416a7900: movups xmm2, xmmword ptr [rbp + 0x298]
1416a7907: movups xmm3, xmmword ptr [rbp + 0x2a8]
1416a790e: movups xmmword ptr [rsi + 0x38], xmm3
1416a7912: movups xmmword ptr [rsi + 0x28], xmm2
1416a7916: movups xmmword ptr [rsi + 0x18], xmm1
1416a791a: movups xmmword ptr [rsi + 8], xmm0
1416a791e: mov qword ptr [rsi], rax
1416a7921: mov byte ptr [rbp + 0x38f], 0
1416a7928: lea rcx, [rbp + 0x338]
1416a792f: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1416a7934: mov rax, rsi
1416a7937: add rsp, 0x418
1416a793e: pop rbx
1416a793f: pop rdi
1416a7940: pop rsi
1416a7941: pop r12
1416a7943: pop r13
1416a7945: pop r14
1416a7947: pop r15
1416a7949: pop rbp
1416a794a: ret 
1416a794b: lea rcx, [rip + 0x15e1488] ; 0x142c88dda b'assertion failed: desc.u'
1416a7952: lea r8, [rip + 0x15e14bf] ; 0x142c88e18 b'\x88\x8d\xc8B\x01\x00\x00\x00R\x00\x00\x00\x00\x00\x00\x002\x02\x00\x00\t\x00\x00\x00'
1416a7959: mov edx, 0x3c
1416a795e: call 0x1428d9430 ; core::panicking::panic
1416a7963: lea rcx, [rip + 0x15e14c6] ; 0x142c88e30 b'assertion failed: create'
1416a796a: lea r8, [rip + 0x15e14ef] ; 0x142c88e60 b'\x88\x8d\xc8B\x01\x00\x00\x00R\x00\x00\x00\x00\x00\x00\x00<\x02\x00\x00\t\x00\x00\x00'
1416a7971: mov edx, 0x2e
1416a7976: call 0x1428d9430 ; core::panicking::panic
1416a797b: lea rcx, [rip + 0x15e14f6] ; 0x142c88e78 b'assertion failed: create'
1416a7982: lea r8, [rip + 0x15e151f] ; 0x142c88ea8 b'\x88\x8d\xc8B\x01\x00\x00\x00R\x00\x00\x00\x00\x00\x00\x00=\x02\x00\x00\t\x00\x00\x00'
1416a7989: mov edx, 0x2f
1416a798e: call 0x1428d9430 ; core::panicking::panic
1416a7993: lea rcx, [rip + 0x15e1526] ; 0x142c88ec0 b'assertion failed: create'
1416a799a: lea r8, [rip + 0x15e154f] ; 0x142c88ef0 b'\x88\x8d\xc8B\x01\x00\x00\x00R\x00\x00\x00\x00\x00\x00\x00>\x02\x00\x00\t\x00\x00\x00'
1416a79a1: mov edx, 0x2e
1416a79a6: call 0x1428d9430 ; core::panicking::panic
1416a79ab: mov rcx, r14
1416a79ae: mov r8d, 0x3b9aca00
1416a79b4: call 0x1428cd210 ; parking_lot::raw_mutex::RawMutex::lock_slow
1416a79b9: jmp 0x1416a73b4
1416a79be: mov byte ptr [rbp + 0x38f], 1
1416a79c5: mov rcx, rdi
1416a79c8: xor edx, edx
1416a79ca: call 0x1428cdb90 ; parking_lot::raw_mutex::RawMutex::unlock_slow
1416a79cf: jmp 0x1416a759f
1416a79d4: mov dword ptr [rbp + 0x270], eax
1416a79da: lea rax, [rip + 0x15e1537] ; 0x142c88f18 b'\x88\x8d\xc8B\x01\x00\x00\x00R\x00\x00\x00\x00\x00\x00\x00D\x02\x00\x00\x12\x00\x00\x00'
1416a79e1: mov qword ptr [rsp + 0x20], rax
1416a79e6: lea rcx, [rip + 0x15e151b] ; 0x142c88f08 b'create_image\x00\x00\x00\x00\x88\x8d\xc8B\x01\x00\x00\x00'
1416a79ed: lea r9, [rip + 0x15e0cd4] ; 0x142c886c8 b'\x00\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00'
1416a79f4: lea r8, [rbp + 0x270]
1416a79fb: mov edx, 0xc
1416a7a00: call 0x1428d9760 ; core::result::unwrap_failed
1416a7a05: mov dword ptr [rbp + 0x270], eax
1416a7a0b: mov byte ptr [rbp + 0x38f], 1
1416a7a12: lea rax, [rip + 0x15e157f] ; 0x142c88f98 b'\x88\x8d\xc8B\x01\x00\x00\x00R\x00\x00\x00\x00\x00\x00\x00j\x02\x00\x00\x12\x00\x00\x00'
1416a7a19: mov qword ptr [rsp + 0x20], rax
1416a7a1e: lea rcx, [rip + 0x15e155b] ; 0x142c88f80 b'bind_image_memory\x00\x00\x00\x00\x00\x00\x00'
1416a7a25: lea r9, [rip + 0x15e0c9c] ; 0x142c886c8 b'\x00\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00'
1416a7a2c: lea r8, [rbp + 0x270]
1416a7a33: mov edx, 0x11
1416a7a38: call 0x1428d9760 ; core::result::unwrap_failed
1416a7a3d: jmp 0x1416a7ae9
1416a7a42: movups xmm0, xmmword ptr [rbp + 0xd0]
1416a7a49: movups xmm1, xmmword ptr [rbp + 0xe0]
1416a7a50: movaps xmmword ptr [rbp + 0x280], xmm1
1416a7a57: movaps xmmword ptr [rbp + 0x270], xmm0
1416a7a5e: lea rax, [rip + 0x15e14cb] ; 0x142c88f30 b'\x88\x8d\xc8B\x01\x00\x00\x00R\x00\x00\x00\x00\x00\x00\x00G\x02\x00\x00/\x00\x00\x00'
1416a7a65: mov qword ptr [rsp + 0x20], rax
1416a7a6a: lea rcx, [rip + 0x15e0c97] ; 0x142c88708 b'called `Result::unwrap()'
1416a7a71: lea r9, [rip + 0x15e0c70] ; 0x142c886e8 b'\xf0E]@\x01\x00\x00\x00 \x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00'
1416a7a78: lea r8, [rbp + 0x270]
1416a7a7f: mov edx, 0x2b
1416a7a84: call 0x1428d9760 ; core::result::unwrap_failed
1416a7a89: jmp 0x1416a7ae9
1416a7a8b: mov qword ptr [rbp + 0x380], r12
1416a7a92: mov qword ptr [rbp + 0x378], r15
1416a7a99: mov dword ptr [rbp + 0xd0], eax
1416a7a9f: lea rax, [rip + 0x15e14c2] ; 0x142c88f68 b'\x88\x8d\xc8B\x01\x00\x00\x00R\x00\x00\x00\x00\x00\x00\x00M\x02\x00\x00\x12\x00\x00\x00'
1416a7aa6: mov qword ptr [rsp + 0x20], rax
1416a7aab: lea rcx, [rip + 0x15e1496] ; 0x142c88f48 b'set_debug_utils_object_n'
1416a7ab2: lea r9, [rip + 0x15e0c0f] ; 0x142c886c8 b'\x00\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00'
1416a7ab9: lea r8, [rbp + 0xd0]
1416a7ac0: mov edx, 0x1b
1416a7ac5: call 0x1428d9760 ; core::result::unwrap_failed
1416a7aca: jmp 0x1416a7ae9
1416a7acc: mov qword ptr [rbp + 0x378], r15
1416a7ad3: lea r8, [rip + 0x15e0ca6] ; 0x142c88780 b'3\x87\xc8B\x01\x00\x00\x00J\x00\x00\x00\x00\x00\x00\x00\xa2\x00\x00\x00\x19\x00\x00\x00'
1416a7ada: mov ecx, 1
1416a7adf: mov edx, 0xf
1416a7ae4: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1416a7ae9: ud2 
1416a7aeb: nop dword ptr [rax + rax]
1416a7af0: mov qword ptr [rsp + 0x10], rdx
1416a7af5: push rbp
1416a7af6: push r15
1416a7af8: push r14
1416a7afa: push r13
1416a7afc: push r12
1416a7afe: push rsi
1416a7aff: push rdi
1416a7b00: push rbx
1416a7b01: sub rsp, 0x38
1416a7b05: lea rbp, [rdx + 0x80]
1416a7b0c: mov rdx, qword ptr [rbp + 0x270]
1416a7b13: test rdx, rdx
1416a7b16: je 0x1416a7b2a
1416a7b18: mov rcx, qword ptr [rbp + 0x278]
1416a7b1f: mov r8d, 1
1416a7b25: call 0x140613c20 ; ?
1416a7b2a: nop 
1416a7b2b: add rsp, 0x38
1416a7b2f: pop rbx
1416a7b30: pop rdi
1416a7b31: pop rsi
1416a7b32: pop r12
1416a7b34: pop r13
1416a7b36: pop r14
1416a7b38: pop r15
1416a7b3a: pop rbp
1416a7b3b: ret 
1416a7b3c: nop dword ptr [rax]
1416a7b40: mov qword ptr [rsp + 0x10], rdx
1416a7b45: push rbp
1416a7b46: push r15
1416a7b48: push r14
1416a7b4a: push r13
1416a7b4c: push r12
1416a7b4e: push rsi
1416a7b4f: push rdi
1416a7b50: push rbx
1416a7b51: sub rsp, 0x38
1416a7b55: lea rbp, [rdx + 0x80]
1416a7b5c: mov rax, qword ptr [rbp + 0x380]
1416a7b63: mov byte ptr [rax], 0
1416a7b66: cmp qword ptr [rbp + 0x378], 0
1416a7b6e: je 0x1416a7b89
1416a7b70: mov r8d, 1
1416a7b76: mov rcx, qword ptr [rbp + 0x380]
1416a7b7d: mov rdx, qword ptr [rbp + 0x378]
1416a7b84: call 0x140613c20 ; ?
1416a7b89: nop 
1416a7b8a: add rsp, 0x38
1416a7b8e: pop rbx
1416a7b8f: pop rdi
1416a7b90: pop rsi
1416a7b91: pop r12
1416a7b93: pop r13
1416a7b95: pop r14
1416a7b97: pop r15
1416a7b99: pop rbp
1416a7b9a: ret 
1416a7b9b: nop dword ptr [rax + rax]
1416a7ba0: mov qword ptr [rsp + 0x10], rdx
1416a7ba5: push rbp
1416a7ba6: push r15
1416a7ba8: push r14
1416a7baa: push r13
1416a7bac: push r12
1416a7bae: push rsi
1416a7baf: push rdi
1416a7bb0: push rbx
1416a7bb1: sub rsp, 0x38
1416a7bb5: lea rbp, [rdx + 0x80]
1416a7bbc: cmp qword ptr [rbp + 0x380], 5
1416a7bc4: ja 0x1416a7bfb
1416a7bc6: mov eax, 0x32
1416a7bcb: mov rcx, qword ptr [rbp + 0x380]
1416a7bd2: bt rax, rcx
1416a7bd6: jae 0x1416a7bfb
1416a7bd8: cmp qword ptr [rbp + 0x370], 0
1416a7be0: je 0x1416a7bfb
1416a7be2: mov r8d, 1
1416a7be8: mov rcx, qword ptr [rbp + 0x378]
1416a7bef: mov rdx, qword ptr [rbp + 0x370]
1416a7bf6: call 0x140613c20 ; ?
1416a7bfb: nop 
1416a7bfc: add rsp, 0x38
1416a7c00: pop rbx
1416a7c01: pop rdi
1416a7c02: pop rsi
1416a7c03: pop r12
1416a7c05: pop r13
1416a7c07: pop r14
1416a7c09: pop r15
1416a7c0b: pop rbp
1416a7c0c: ret 
1416a7c0d: nop dword ptr [rax]
1416a7c10: mov qword ptr [rsp + 0x10], rdx
1416a7c15: push rbp
1416a7c16: push r15
1416a7c18: push r14
1416a7c1a: push r13
1416a7c1c: push r12
1416a7c1e: push rsi
1416a7c1f: push rdi
1416a7c20: push rbx
1416a7c21: sub rsp, 0x38
1416a7c25: lea rbp, [rdx + 0x80]
1416a7c2c: lea rcx, [rbp + 0x338]
1416a7c33: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1416a7c38: mov byte ptr [rbp + 0x38f], 0
1416a7c3f: add rsp, 0x38
1416a7c43: pop rbx
1416a7c44: pop rdi
1416a7c45: pop rsi
1416a7c46: pop r12
1416a7c48: pop r13
1416a7c4a: pop r14
1416a7c4c: pop r15
1416a7c4e: pop rbp
1416a7c4f: ret 
1416a7c50: mov qword ptr [rsp + 0x10], rdx
1416a7c55: push rbp
1416a7c56: push r15
1416a7c58: push r14
1416a7c5a: push r13
1416a7c5c: push r12
1416a7c5e: push rsi
1416a7c5f: push rdi
1416a7c60: push rbx
1416a7c61: sub rsp, 0x38
1416a7c65: lea rbp, [rdx + 0x80]
1416a7c6c: cmp byte ptr [rbp + 0x38f], 0
1416a7c73: jne 0x1416a7c86
1416a7c75: add rsp, 0x38
1416a7c79: pop rbx
1416a7c7a: pop rdi
1416a7c7b: pop rsi
1416a7c7c: pop r12
1416a7c7e: pop r13
1416a7c80: pop r14
1416a7c82: pop r15
1416a7c84: pop rbp
1416a7c85: ret 
1416a7c86: lea rcx, [rbp + 0x220]
1416a7c8d: call 0x140888ce0 ; core::ptr::drop_in_place<gpu_allocator::vulkan::Allocation>
1416a7c92: jmp 0x1416a7c75
1416a7c94: nop word ptr cs:[rax + rax]
1416a7ca0: mov qword ptr [rsp + 0x10], rdx
1416a7ca5: push rbp
1416a7ca6: push r15
1416a7ca8: push r14
1416a7caa: push r13
1416a7cac: push r12
1416a7cae: push rsi
1416a7caf: push rdi
1416a7cb0: push rbx
1416a7cb1: sub rsp, 0x38
1416a7cb5: lea rbp, [rdx + 0x80]
1416a7cbc: mov rcx, qword ptr [rbp + 0x368]
1416a7cc3: call 0x140161c30 ; core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo> >
1416a7cc8: nop 
1416a7cc9: add rsp, 0x38
1416a7ccd: pop rbx
1416a7cce: pop rdi
1416a7ccf: pop rsi
1416a7cd0: pop r12
1416a7cd2: pop r13
1416a7cd4: pop r14
1416a7cd6: pop r15
1416a7cd8: pop rbp
1416a7cd9: ret 
