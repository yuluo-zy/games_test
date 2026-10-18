140018040: push rbp
140018041: push r15
140018043: push r14
140018045: push r13
140018047: push r12
140018049: push rsi
14001804a: push rdi
14001804b: push rbx
14001804c: sub rsp, 0x948
140018053: lea rbp, [rsp + 0x80]
14001805b: movdqa xmmword ptr [rbp + 0x8b0], xmm15
140018064: movaps xmmword ptr [rbp + 0x8a0], xmm14
14001806c: movdqa xmmword ptr [rbp + 0x890], xmm13
140018075: movdqa xmmword ptr [rbp + 0x880], xmm12
14001807e: movaps xmmword ptr [rbp + 0x870], xmm11
140018086: movaps xmmword ptr [rbp + 0x860], xmm10
14001808e: movaps xmmword ptr [rbp + 0x850], xmm9
140018096: movaps xmmword ptr [rbp + 0x840], xmm8
14001809e: movaps xmmword ptr [rbp + 0x830], xmm7
1400180a5: movaps xmmword ptr [rbp + 0x820], xmm6
1400180ac: mov qword ptr [rbp + 0x818], 0xfffffffffffffffe
1400180b7: mov r15, r9
1400180ba: mov qword ptr [rbp + 0x368], r8
1400180c1: mov qword ptr [rbp + 0x268], rdx
1400180c8: mov rdi, rcx
1400180cb: mov rax, qword ptr [rbp + 0x930]
1400180d2: mov rcx, qword ptr [rax]
1400180d5: mov qword ptr [rbp + 0x7b0], rcx
1400180dc: mov r12, qword ptr [rax + 0x10]
1400180e0: mov r13d, dword ptr [rax + 0x1c]
1400180e4: mov rcx, qword ptr [rax + 0x20]
1400180e8: mov qword ptr [rbp + 0x2e0], rcx
1400180ef: mov rcx, qword ptr [rax + 0x30]
1400180f3: mov qword ptr [rbp + 0x90], rcx
1400180fa: mov ecx, dword ptr [rax + 0x3c]
1400180fd: mov dword ptr [rbp + 0x238], ecx
140018103: mov rcx, qword ptr [rax + 0x40]
140018107: mov qword ptr [rbp + 0x360], rcx
14001810e: mov rcx, qword ptr [rax + 0x50]
140018112: mov qword ptr [rbp + 0x330], rcx
140018119: mov ecx, dword ptr [rax + 0x5c]
14001811c: mov dword ptr [rbp + 0x534], ecx
140018122: mov rcx, qword ptr [rax + 0x60]
140018126: mov qword ptr [rbp + 0x338], rcx
14001812d: mov rcx, qword ptr [rax + 0x70]
140018131: mov qword ptr [rbp + 0x258], rcx
140018138: mov ecx, dword ptr [rax + 0x7c]
14001813b: mov dword ptr [rbp + 0x424], ecx
140018141: mov rcx, qword ptr [rax + 0x80]
140018148: mov qword ptr [rbp + 0x240], rcx
14001814f: mov rax, qword ptr [rax + 0xa0]
140018156: mov qword ptr [rbp + 0x328], rax
14001815d: mov rax, qword ptr [rbp + 0x960]
140018164: mov rcx, qword ptr [rax]
140018167: mov qword ptr [rbp + 0xe8], rcx
14001816e: mov rcx, qword ptr [rax + 0x20]
140018172: mov qword ptr [rbp + 0x260], rcx
140018179: mov rax, qword ptr [rax + 0x28]
14001817d: mov qword ptr [rbp + 0x250], rax
140018184: call 0x140c16a00 ; puffin::are_scopes_on
140018189: mov byte ptr [rbp + 0x79f], al
14001818f: test al, al
140018191: je 0x1400183c7
140018197: mov r14d, 0x4f
14001819d: mov ecx, 0x4f
1400181a2: call 0x1400299b0 ; core::str::impl$0::rfind<ref$<str$> >
1400181a7: lea rsi, [rip + 0x290f202]
1400181ae: cmp rax, 1
1400181b2: jne 0x140018259
1400181b8: mov r9, rdx
1400181bb: test rdx, rdx
1400181be: je 0x1400181f6
1400181c0: cmp r9, 0x4f
1400181c4: jae 0x1400181f4
1400181c6: lea rax, [rip + 0x290f1e3]
1400181cd: cmp byte ptr [r9 + rax], 0xbf
1400181d2: jg 0x1400181f6
1400181d4: lea rax, [rip + 0x290f29d]
1400181db: mov qword ptr [rsp + 0x20], rax
1400181e0: lea rcx, [rip + 0x290f1c9]
1400181e7: mov edx, 0x4f
1400181ec: xor r8d, r8d
1400181ef: call 0x1428d9940 ; core::str::slice_error_fail
1400181f4: jne 0x1400181d4
1400181f6: mov rcx, r9
1400181f9: call 0x1400299b0 ; core::str::impl$0::rfind<ref$<str$> >
1400181fe: cmp rax, 1
140018202: jne 0x140018259
140018204: mov r8, rdx
140018207: add r8, 2
14001820b: je 0x140018246
14001820d: cmp r8, 0x4f
140018211: jae 0x140018244
140018213: lea rax, [rip + 0x290f196]
14001821a: cmp byte ptr [r8 + rax], 0xbf
14001821f: jg 0x140018246
140018221: lea rax, [rip + 0x290f268]
140018228: mov qword ptr [rsp + 0x20], rax
14001822d: lea rcx, [rip + 0x290f17c]
140018234: mov edx, 0x4f
140018239: mov r9d, 0x4f
14001823f: call 0x1428d9940 ; core::str::slice_error_fail
140018244: jne 0x140018221
140018246: mov r14d, 0x4d
14001824c: sub r14, rdx
14001824f: lea rsi, [rip + 0x290f15a]
140018256: add rsi, r8
140018259: mov qword ptr [rbp + 0x808], r15
140018260: lea r8, [rip + 0x290e241]
140018267: lea r15, [rip + 0x290e1fa]
14001826e: nop 
140018270: movsx eax, byte ptr [r8 - 1]
140018275: test eax, eax
140018277: js 0x140018290
140018279: dec r8
14001827c: cmp eax, 0x5c
14001827f: jne 0x1400182e4
140018281: jmp 0x1400182fc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140018283: nop word ptr cs:[rax + rax]
140018290: movzx ecx, byte ptr [r8 - 2]
140018295: cmp cl, 0xc0
140018298: jge 0x1400182bd
14001829a: movzx edx, byte ptr [r8 - 3]
14001829f: cmp dl, 0xc0
1400182a2: jge 0x1400182c6
1400182a4: movzx r9d, byte ptr [r8 - 4]
1400182a9: add r8, -4
1400182ad: and r9d, 7
1400182b1: shl r9d, 6
1400182b5: and edx, 0x3f
1400182b8: or edx, r9d
1400182bb: jmp 0x1400182cd ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400182bd: add r8, -2
1400182c1: and ecx, 0x1f
1400182c4: jmp 0x1400182d5 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400182c6: add r8, -3
1400182ca: and edx, 0xf
1400182cd: shl edx, 6
1400182d0: and ecx, 0x3f
1400182d3: or ecx, edx
1400182d5: shl ecx, 6
1400182d8: and al, 0x3f
1400182da: movzx eax, al
1400182dd: or eax, ecx
1400182df: cmp eax, 0x5c
1400182e2: je 0x1400182fc
1400182e4: cmp eax, 0x2f
1400182e7: je 0x1400182fc
1400182e9: cmp r8, r15
1400182ec: jne 0x140018270
1400182ee: mov ebx, r13d
1400182f1: mov r13, r12
1400182f4: mov r12d, 0x40
1400182fa: jmp 0x14001834f ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400182fc: lea r15, [rip + 0x290e165]
140018303: sub r8, r15
140018306: inc r8
140018309: je 0x14001833d
14001830b: cmp r8, 0x40
14001830f: jae 0x14001833b
140018311: cmp byte ptr [r8 + r15], 0xbf
140018316: jg 0x14001833d
140018318: lea rax, [rip + 0x290f139]
14001831f: mov qword ptr [rsp + 0x20], rax
140018324: lea rcx, [rip + 0x290e13d]
14001832b: mov edx, 0x40
140018330: mov r9d, 0x40
140018336: call 0x1428d9940 ; core::str::slice_error_fail
14001833b: jne 0x140018318
14001833d: mov ebx, r13d
140018340: mov r13, r12
140018343: mov r12d, 0x40
140018349: sub r12, r8
14001834c: add r15, r8
14001834f: call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
140018354: mov rcx, rax
140018357: mov rax, qword ptr [rax]
14001835a: cmp rax, 1
14001835e: jne 0x14002045e
140018364: add rcx, 8
140018368: cmp qword ptr [rcx], 0
14001836c: jne 0x1400206e0
140018372: mov qword ptr [rcx], 0xffffffffffffffff
140018379: mov qword ptr [rbp + 0x7a0], rcx
140018380: add rcx, 8
140018384: mov qword ptr [rsp + 0x20], r12
140018389: mov qword ptr [rsp + 0x30], 0
140018392: mov qword ptr [rsp + 0x28], 1
14001839b: mov rdx, rsi
14001839e: mov r8, r14
1400183a1: mov r9, r15
1400183a4: call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
1400183a9: mov qword ptr [rbp + 0x270], rax
1400183b0: mov rax, qword ptr [rbp + 0x7a0]
1400183b7: inc qword ptr [rax]
1400183ba: mov r12, r13
1400183bd: mov r13d, ebx
1400183c0: mov r15, qword ptr [rbp + 0x808]
1400183c7: lea rdx, [rip + 0x2995df2]
1400183ce: mov rcx, rdi
1400183d1: call 0x1405ca870 ; frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>
1400183d6: mov rbx, rax
1400183d9: test rax, rax
1400183dc: je 0x140020a55
1400183e2: lea rax, [rip + 0x29e4ac8]
1400183e9: mov qword ptr [rbp + 0x6c0], rax
1400183f0: mov qword ptr [rbp + 0x6c8], 0x3a
1400183fb: mov r14, rbx
1400183fe: add r14, 0x10
140018402: cmp qword ptr [rbx + 0x18], 0x3a
140018407: jne 0x1400205f4
14001840d: mov rdx, qword ptr [r14]
140018410: lea rcx, [rip + 0x29e4a9a]
140018417: mov r8d, 0x3a
14001841d: call 0x142923bf0 ; ?
140018422: test eax, eax
140018424: jne 0x1400205f4
14001842a: mov rcx, qword ptr [rbx]
14001842d: mov rax, qword ptr [rbx + 8]
140018431: call qword ptr [rax + 0x18]
140018434: mov qword ptr [rbp + 0x120], rax
14001843b: cmp qword ptr [rbp + 0x120], 0
140018443: je 0x140020a55
140018449: lea rdx, [rip + 0x2995d68]
140018450: mov rcx, rdi
140018453: call 0x1405ca870 ; frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>
140018458: mov rbx, rax
14001845b: test rax, rax
14001845e: je 0x140020a66
140018464: lea rax, [rip + 0x29e47ea]
14001846b: mov qword ptr [rbp + 0x6c0], rax
140018472: mov qword ptr [rbp + 0x6c8], 0x3d
14001847d: mov r14, rbx
140018480: add r14, 0x10
140018484: cmp qword ptr [rbx + 0x18], 0x3d
140018489: jne 0x140020628
14001848f: mov rdx, qword ptr [r14]
140018492: lea rcx, [rip + 0x29e47bc]
140018499: mov r8d, 0x3d
14001849f: call 0x142923bf0 ; ?
1400184a4: test eax, eax
1400184a6: jne 0x140020628
1400184ac: mov rcx, qword ptr [rbx]
1400184af: mov rax, qword ptr [rbx + 8]
1400184b3: call qword ptr [rax + 0x18]
1400184b6: mov qword ptr [rbp + 0x128], rax
1400184bd: cmp qword ptr [rbp + 0x128], 0
1400184c5: je 0x140020a66
1400184cb: lea rdx, [rip + 0x2995cbe]
1400184d2: mov rcx, rdi
1400184d5: call 0x1405ca870 ; frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>
1400184da: mov rdi, rax
1400184dd: test rax, rax
1400184e0: je 0x140020a77
1400184e6: lea rax, [rip + 0x29e473b]
1400184ed: mov qword ptr [rbp + 0x6c0], rax
1400184f4: mov qword ptr [rbp + 0x6c8], 0x2d
1400184ff: mov rbx, rdi
140018502: add rbx, 0x10
140018506: cmp qword ptr [rdi + 0x18], 0x2d
14001850b: jne 0x14002065c
140018511: mov rdx, qword ptr [rbx]
140018514: lea rcx, [rip + 0x29e470d]
14001851b: mov r8d, 0x2d
140018521: call 0x142923bf0 ; ?
140018526: test eax, eax
140018528: jne 0x14002065c
14001852e: mov rcx, qword ptr [rdi]
140018531: mov rax, qword ptr [rdi + 8]
140018535: call qword ptr [rax + 0x18]
140018538: mov qword ptr [rbp + 0x130], rax
14001853f: cmp qword ptr [rbp + 0x130], 0
140018547: je 0x140020a77
14001854d: mov rax, qword ptr [rbp + 0x948]
140018554: mov rcx, qword ptr [rbp + 0x940]
14001855b: mov dword ptr [r12], r13d
14001855f: mov r13, rcx
140018562: mov rcx, qword ptr [rbp + 0x7b0]
140018569: lea rdx, [rcx + 0x198]
140018570: mov qword ptr [rbp + 0x2d0], rdx
140018577: mov rbx, qword ptr [rcx + 8]
14001857b: imul rdx, qword ptr [rcx + 0x10], 0x150
140018583: add rdx, rbx
140018586: mov qword ptr [rbp + 0xf0], rdx
14001858d: mov edx, dword ptr [rax + 0x1c]
140018590: mov dword ptr [rbp + 0x23c], edx
140018596: mov rdx, qword ptr [rax]
140018599: mov qword ptr [rbp + 0x98], rdx
1400185a0: mov rax, qword ptr [rax + 0x10]
1400185a4: mov qword ptr [rbp + 0xa0], rax
1400185ab: mov eax, dword ptr [r15 + 0x1c]
1400185af: mov dword ptr [rbp + 0x324], eax
1400185b5: mov rdx, qword ptr [r15]
1400185b8: mov rax, qword ptr [r15 + 0x10]
1400185bc: mov qword ptr [rbp + 0x110], rax
1400185c3: mov rax, qword ptr [rbp + 0x2e0]
1400185ca: movzx eax, byte ptr [rax + 0x30]
1400185ce: mov byte ptr [rbp + 0x790], al
1400185d4: lea rax, [rcx + 0x18]
1400185d8: mov qword ptr [rbp + 0x248], rax
1400185df: lea rax, [rcx + 0x1b8]
1400185e6: mov qword ptr [rbp + 0xc0], rax
1400185ed: lea rax, [rdx + 0x60]
1400185f1: mov qword ptr [rbp + 0xb0], rax
1400185f8: mov qword ptr [rbp + 0xb8], rdx
1400185ff: lea rax, [rdx + 0x30]
140018603: mov qword ptr [rbp + 0xa8], rax
14001860a: mov rax, qword ptr [rbp + 0x328]
140018611: add rax, 0x20
140018615: mov qword ptr [rbp + 0x108], rax
14001861c: lea rax, [rcx + 0x60]
140018620: mov qword ptr [rbp + 0x340], rax
140018627: xor r14d, r14d
14001862a: lea r12, [rbp + 0x590]
140018631: jmp 0x14001fa40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140018636: nop word ptr cs:[rax + rax]
140018640: movdqu xmm1, xmmword ptr [r8 + r10]
140018646: movdqa xmm2, xmm1
14001864a: pcmpeqb xmm2, xmm0
14001864e: pmovmskb r11d, xmm2
140018653: test r11d, r11d
140018656: je 0x1400186a0
140018658: mov rdx, qword ptr [rbp + 0x7e0]
14001865f: mov ecx, dword ptr [rdx + 0x128]
140018665: mov edx, dword ptr [rdx + 0x12c]
14001866b: tzcnt edi, r11d
140018670: add rdi, r10
140018673: and rdi, r9
140018676: shl rdi, 3
14001867a: mov rsi, r8
14001867d: sub rsi, rdi
140018680: cmp ecx, dword ptr [rsi - 8]
140018683: jne 0x14001868a
140018685: cmp edx, dword ptr [rsi - 4]
140018688: je 0x1400186c7
14001868a: lea esi, [r11 - 1]
14001868e: and si, r11w
140018692: mov r11d, esi
140018695: jne 0x14001866b
140018697: nop word ptr [rax + rax]
1400186a0: pcmpeqb xmm1, xmmword ptr [rip + 0x290d2e8]
1400186a8: pmovmskb ecx, xmm1
1400186ac: test ecx, ecx
1400186ae: jne 0x14001fa40
1400186b4: add r10, rax
1400186b7: add r10, 0x10
1400186bb: add rax, 0x10
1400186bf: and r10, r9
1400186c2: jmp 0x140018640 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400186c7: call 0x140959e70 ; country_core::resources::stairs::stairs_assembly_params::impl$2::from
1400186cc: mov qword ptr [rbp + 0x438], rax
1400186d3: lea rdi, [rbp + 0x460]
1400186da: mov rcx, qword ptr [rbp + 0x438]
1400186e1: call 0x140c96270 ; utils::rng::init_rng_u64
1400186e6: mov qword ptr [rbp + 0x428], rax
1400186ed: mov rdx, qword ptr [rbp + 0x7e0]
1400186f4: cmp r14, qword ptr [rdx + 0x98]
1400186fb: jo 0x140020a20
140018701: mov rax, qword ptr [rdx + 0xa0]
140018708: mov qword ptr [rbp + 0x648], rax
14001870f: mov rax, qword ptr [rdx + 0xa8]
140018716: mov qword ptr [rbp + 0x780], rax
14001871d: mov rax, qword ptr [rbp + 0x958]
140018724: mov rcx, qword ptr [rax]
140018727: mov edx, dword ptr [rdx + 0x128]
14001872d: call 0x140a1c930 ; country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info
140018732: test rax, rax
140018735: je 0x14001873c
140018737: mov eax, dword ptr [rax + 0x28]
14001873a: jmp 0x140018741 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001873c: mov eax, 0x186a3
140018741: mov qword ptr [rbp + 0x788], rax
140018748: mov rax, qword ptr [rbp + 0x958]
14001874f: mov rcx, qword ptr [rax]
140018752: mov rax, qword ptr [rbp + 0x670]
140018759: mov edx, dword ptr [rax]
14001875b: call 0x140a1c930 ; country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info
140018760: mov dword ptr [rbp + 0x640], 0
14001876a: test rax, rax
14001876d: je 0x140018789
14001876f: add rax, 0x30
140018773: mov rcx, rax
140018776: call 0x140839490 ; country_core::resources::walls::decorator::DecoratorExtraInfo::try_as_stair
14001877b: test rax, rax
14001877e: je 0x140018789
140018780: mov eax, dword ptr [rax + 8]
140018783: mov dword ptr [rbp + 0x640], eax
140018789: mov rax, qword ptr [rbp + 0x958]
140018790: mov rcx, qword ptr [rax]
140018793: mov rax, qword ptr [rbp + 0x670]
14001879a: mov edx, dword ptr [rax]
14001879c: call 0x140a1c930 ; country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info
1400187a1: test rax, rax
1400187a4: je 0x1400187e8
1400187a6: add rax, 0x30
1400187aa: mov rcx, rax
1400187ad: call 0x140839490 ; country_core::resources::walls::decorator::DecoratorExtraInfo::try_as_stair
1400187b2: test rax, rax
1400187b5: je 0x1400187e8
1400187b7: movzx eax, byte ptr [rax + 0x10]
1400187bb: mov dword ptr [rbp + 0x6a8], eax
1400187c1: xor r14d, r14d
1400187c4: mov rax, qword ptr [rbp + 0x328]
1400187cb: cmp qword ptr [rax + 0x18], 0
1400187d0: jne 0x140018803
1400187d2: mov dword ptr [rbp + 0x530], 0
1400187dc: mov rsi, qword ptr [rbp + 0x7e0]
1400187e3: jmp 0x1400188ef ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400187e8: mov dword ptr [rbp + 0x6a8], 0
1400187f2: xor r14d, r14d
1400187f5: mov rax, qword ptr [rbp + 0x328]
1400187fc: cmp qword ptr [rax + 0x18], 0
140018801: je 0x1400187d2
140018803: mov rcx, qword ptr [rbp + 0x108]
14001880a: mov rdx, qword ptr [rbp + 0x670]
140018811: call 0x1405b1be0 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::resources::stairs::stairs_state::StairGraphEdge> >
140018816: mov rdx, qword ptr [rbp + 0x328]
14001881d: mov rcx, qword ptr [rdx]
140018820: mov rdx, qword ptr [rdx + 8]
140018824: mov r8, rdx
140018827: and r8, rax
14001882a: shr rax, 0x39
14001882e: movd xmm0, eax
140018832: punpcklbw xmm0, xmm0
140018836: pshuflw xmm0, xmm0, 0
14001883b: pshufd xmm0, xmm0, 0
140018840: xor eax, eax
140018842: mov rsi, qword ptr [rbp + 0x7e0]
140018849: movdqu xmm1, xmmword ptr [rcx + r8]
14001884f: movdqa xmm2, xmm1
140018853: pcmpeqb xmm2, xmm0
140018857: pmovmskb r9d, xmm2
14001885c: test r9d, r9d
14001885f: je 0x1400188c0
140018861: mov r10, qword ptr [rbp + 0x670]
140018868: movq xmm2, qword ptr [r10]
14001886d: tzcnt r10d, r9d
140018872: add r10, r8
140018875: and r10, rdx
140018878: shl r10, 3
14001887c: mov r11, rcx
14001887f: sub r11, r10
140018882: movq xmm3, qword ptr [r11 - 8]
140018888: pcmpeqd xmm3, xmm2
14001888c: pshufd xmm3, xmm3, 0x50
140018891: movmskpd r10d, xmm3
140018896: mov r11b, 1
140018899: mov dword ptr [rbp + 0x530], r11d
1400188a0: cmp r10d, 3
1400188a4: je 0x1400188ef
1400188a6: lea r10d, [r9 - 1]
1400188aa: and r10w, r9w
1400188ae: mov r9d, r10d
1400188b1: jne 0x14001886d
1400188b3: nop word ptr cs:[rax + rax]
1400188c0: pcmpeqb xmm1, xmmword ptr [rip + 0x290d0c8]
1400188c8: pmovmskb r9d, xmm1
1400188cd: test r9d, r9d
1400188d0: jne 0x1400188e5
1400188d2: add r8, rax
1400188d5: add r8, 0x10
1400188d9: add rax, 0x10
1400188dd: and r8, rdx
1400188e0: jmp 0x140018849 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400188e5: mov dword ptr [rbp + 0x530], 0
1400188ef: lea rax, [rsi + 0xd0]
1400188f6: mov qword ptr [rbp + 0x2d8], rax
1400188fd: cmp dword ptr [rax], 1
140018900: jne 0x140018918
140018902: movss xmm0, dword ptr [rsi + 0xd4]
14001890a: movss dword ptr [rbp + 0x548], xmm0
140018912: mov r14d, 1
140018918: xor eax, eax
14001891a: cmp dword ptr [rsi + 0xf0], 1
140018921: jne 0x140018938
140018923: movss xmm0, dword ptr [rsi + 0xf4]
14001892b: movss dword ptr [rbp + 0x370], xmm0
140018933: mov eax, 1
140018938: mov dword ptr [rbp + 0x540], eax
14001893e: mov rax, qword ptr [rbp + 0x90]
140018945: mov ecx, dword ptr [rbp + 0x238]
14001894b: mov dword ptr [rax], ecx
14001894d: mov r8d, dword ptr [rsi + 0x128]
140018954: mov r9d, dword ptr [rsi + 0x12c]
14001895b: mov rax, qword ptr [rbp + 0x128]
140018962: mov qword ptr [rsp + 0x38], rax
140018967: mov rax, qword ptr [rbp + 0x130]
14001896e: mov qword ptr [rsp + 0x30], rax
140018973: mov rax, qword ptr [rbp + 0x120]
14001897a: mov qword ptr [rsp + 0x28], rax
14001897f: mov qword ptr [rsp + 0x20], r13
140018984: lea rcx, [rbp + 0x140]
14001898b: mov rdx, qword ptr [rbp + 0x2e0]
140018992: call 0x140aa8a60 ; country_core::resources::stairs::visual::StairsVisualState::reset_edge
140018997: mov dword ptr [rbp + 0x500], r14d
14001899e: mov r14, qword ptr [rbp + 0x7e0]
1400189a5: lea r15, [r14 + 0x130]
1400189ac: lea rax, [r14 + 0xf0]
1400189b3: mov qword ptr [rbp + 0x118], rax
1400189ba: mov r9, qword ptr [rbp + 0x780]
1400189c1: lea rax, [r9 + r9*2]
1400189c5: mov r10, qword ptr [rbp + 0x648]
1400189cc: lea rax, [r10 + rax*4]
1400189d0: mov rdx, qword ptr [r14 + 0x58]
1400189d4: mov rcx, qword ptr [r14 + 0x60]
1400189d8: lea r8, [rdx + rcx*4]
1400189dc: cmp rcx, r9
1400189df: cmovae rcx, r9
1400189e3: mov qword ptr [rbp + 0x6c0], r10
1400189ea: mov qword ptr [rbp + 0x6c8], rax
1400189f1: mov qword ptr [rbp + 0x6d0], rdx
1400189f8: mov qword ptr [rbp + 0x6d8], r8
1400189ff: mov qword ptr [rbp + 0x6e8], rcx
140018a06: mov qword ptr [rbp + 0x6f0], r9
140018a0d: mov qword ptr [rbp + 0x6f8], 0
140018a18: xor eax, eax
140018a1a: jmp 0x140018a2e ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140018a1c: nop dword ptr [rax]
140018a20: mov rax, qword ptr [rbp + 0x6e0]
140018a27: mov rcx, qword ptr [rbp + 0x6e8]
140018a2e: cmp rax, rcx
140018a31: jae 0x140018cff
140018a37: lea rcx, [rax + 1]
140018a3b: mov qword ptr [rbp + 0x6e0], rcx
140018a42: lea rcx, [rax + rax*2]
140018a46: shl rcx, 2
140018a4a: add rcx, qword ptr [rbp + 0x6c0]
140018a51: shl rax, 2
140018a55: add rax, qword ptr [rbp + 0x6d0]
140018a5c: cmp qword ptr [rbp + 0x6f8], 0
140018a64: je 0x140018aa0
140018a66: mov rdx, qword ptr [rbp + 0x708]
140018a6d: mov r8, qword ptr [rbp + 0x710]
140018a74: mov qword ptr [rbp + 0x708], rcx
140018a7b: mov qword ptr [rbp + 0x710], rax
140018a82: mov qword ptr [rbp + 0x6f8], rdx
140018a89: mov qword ptr [rbp + 0x700], r8
140018a90: jmp 0x140018b15 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140018a95: nop word ptr cs:[rax + rax]
140018aa0: mov qword ptr [rbp + 0x460], 1
140018aab: mov qword ptr [rbp + 0x468], rcx
140018ab2: mov qword ptr [rbp + 0x470], rax
140018ab9: lea rax, [rbp + 0x6c0]
140018ac0: mov qword ptr [rbp + 0x478], rax
140018ac7: mov rcx, r12
140018aca: mov rdx, rdi
140018acd: call 0x1402e13b0 ; itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > >,ref_mut$<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> > > > >
140018ad2: movups xmm0, xmmword ptr [rbp + 0x590]
140018ad9: movdqu xmm1, xmmword ptr [rbp + 0x5a0]
140018ae1: lea rax, [rbp + 0x6f8]
140018ae8: movdqu xmmword ptr [rax + 0x10], xmm1
140018aed: movups xmmword ptr [rax], xmm0
140018af0: mov rdx, qword ptr [rbp + 0x6f8]
140018af7: test rdx, rdx
140018afa: je 0x140018cff
140018b00: mov r8, qword ptr [rbp + 0x700]
140018b07: mov rcx, qword ptr [rbp + 0x708]
140018b0e: mov rax, qword ptr [rbp + 0x710]
140018b15: movd xmm0, dword ptr [rdx]
140018b19: movss xmm1, dword ptr [rdx + 8]
140018b1e: movd xmm2, dword ptr [rcx]
140018b22: movss xmm3, dword ptr [rcx + 8]
140018b27: movss xmm4, dword ptr [r8]
140018b2c: movss xmm5, dword ptr [rax]
140018b30: movaps xmm6, xmm4
140018b33: cmpunordss xmm6, xmm4
140018b38: movaps xmm7, xmm6
140018b3b: andps xmm7, xmm5
140018b3e: maxss xmm5, xmm4
140018b42: andnps xmm6, xmm5
140018b45: orps xmm6, xmm7
140018b48: mulss xmm6, dword ptr [rip + 0x290cd20]
140018b50: mov rax, qword ptr [rbp + 0x1f8]
140018b57: mov qword ptr [rbp + 0x460], rax
140018b5e: mulss xmm6, dword ptr [rip + 0x290cd0e]
140018b66: addss xmm6, dword ptr [rip + 0x290cd0a]
140018b6e: movd dword ptr [rbp + 0x590], xmm0
140018b76: movss dword ptr [rbp + 0x594], xmm1
140018b7e: movd dword ptr [rbp + 0x598], xmm2
140018b86: movss dword ptr [rbp + 0x59c], xmm3
140018b8e: xor eax, eax
140018b90: xor esi, esi
140018b92: nop word ptr cs:[rax + rax]
140018ba0: cmp rsi, 2
140018ba4: je 0x140018a20
140018baa: lea rcx, [rsi + 1]
140018bae: movd xmm0, dword ptr [rbp + rsi*8 + 0x590]
140018bb7: movd xmm2, dword ptr [rbp + rsi*8 + 0x594]
140018bc0: test al, 1
140018bc2: je 0x140018be0
140018bc4: mov rsi, rcx
140018bc7: movdqa xmm12, xmm0
140018bcc: movdqa xmm13, xmm11
140018bd1: movdqa xmm0, xmm1
140018bd5: movdqa xmm11, xmm2
140018bda: jmp 0x140018c13 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140018bdc: nop dword ptr [rax]
140018be0: cmp rcx, 2
140018be4: je 0x140018a20
140018bea: mov qword ptr [rbp + 0x5a0], 2
140018bf5: movd xmm12, dword ptr [rbp + rsi*8 + 0x598]
140018bff: movd xmm11, dword ptr [rbp + rsi*8 + 0x59c]
140018c09: mov esi, 2
140018c0e: movdqa xmm13, xmm2
140018c13: mov byte ptr [rbp + 0x380], 0
140018c1a: movss xmm7, dword ptr [rip + 0x290cc52]
140018c22: movaps xmm1, xmm7
140018c25: movd xmm8, dword ptr [rip + 0x290cc4e]
140018c2e: movdqa xmm2, xmm8
140018c33: call 0x142923ef0 ; ?
140018c38: movdqa xmm14, xmm0
140018c3d: movdqa xmm0, xmm13
140018c42: movaps xmm1, xmm7
140018c45: movdqa xmm2, xmm8
140018c4a: call 0x142923ef0 ; ?
140018c4f: movd ecx, xmm14
140018c54: movd eax, xmm0
140018c58: shl rax, 0x20
140018c5c: or rcx, rax
140018c5f: call 0x140b84e40 ; conservative_line_raster::impl$0::from
140018c64: movdqa xmm13, xmm0
140018c69: movaps xmm14, xmm1
140018c6d: movdqa xmm0, xmm12
140018c72: movss xmm7, dword ptr [rip + 0x290cbfa]
140018c7a: movaps xmm1, xmm7
140018c7d: movss xmm8, dword ptr [rip + 0x290cbf6]
140018c86: movaps xmm2, xmm8
140018c8a: call 0x142923ef0 ; ?
140018c8f: movdqa xmm15, xmm0
140018c94: movdqa xmm0, xmm11
140018c99: movaps xmm1, xmm7
140018c9c: movaps xmm2, xmm8
140018ca0: call 0x142923ef0 ; ?
140018ca5: movd ecx, xmm15
140018caa: movd eax, xmm0
140018cae: shl rax, 0x20
140018cb2: or rcx, rax
140018cb5: call 0x140b84e40 ; conservative_line_raster::impl$0::from
140018cba: movdqa xmm2, xmm0
140018cbe: movaps xmm3, xmm1
140018cc1: lea rax, [rbp + 0x380]
140018cc8: mov qword ptr [rsp + 0x30], rax
140018ccd: mov qword ptr [rsp + 0x28], rdi
140018cd2: movss dword ptr [rsp + 0x20], xmm6
140018cd8: movdqa xmm0, xmm13
140018cdd: movaps xmm1, xmm14
140018ce1: call 0x1402582d0 ; conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >
140018ce6: mov al, 1
140018ce8: cmp byte ptr [rbp + 0x380], 1
140018cef: movdqa xmm1, xmm12
140018cf4: jne 0x140018ba0
140018cfa: jmp 0x140018a20 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140018cff: mov eax, dword ptr [rbp + 0x23c]
140018d05: mov rcx, qword ptr [rbp + 0xa0]
140018d0c: mov dword ptr [rcx], eax
140018d0e: mov edx, 1
140018d13: mov rcx, qword ptr [rbp + 0x98]
140018d1a: mov r8, qword ptr [rbp + 0x438]
140018d21: call 0x1400cce50 ; country_core::resources::rt::RtWorld::get_mut<enum2$<country_core::resources::rt::RtMeshHandle> >
140018d26: mov qword ptr [rbp + 0x6a0], rax
140018d2d: mov rax, qword ptr [rbp + 0x6a0]
140018d34: mov qword ptr [rax + 0x50], 0
140018d3c: mov rax, qword ptr [rbp + 0x788]
140018d43: add eax, 0xfffe7960
140018d48: cmp eax, 0xb
140018d4b: ja 0x140018d7c
140018d4d: lea rcx, [rip + 0x290eb6c]
140018d54: movsxd rax, dword ptr [rcx + rax*4]
140018d58: add rax, rcx
140018d5b: jmp rax
140018d5d: movsd xmm0, qword ptr [rip + 0x290cb5b]
140018d65: movsd qword ptr [rbp + 0x590], xmm0
140018d6d: mov dword ptr [rbp + 0x598], 0x3e2e147b
140018d77: jmp 0x140018e01 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140018d7c: mov dword ptr [rbp + 0x598], 0
140018d86: mov qword ptr [rbp + 0x590], 0
140018d91: jmp 0x140018e01 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140018d93: movsd xmm0, qword ptr [rip + 0x290cb05]
140018d9b: movsd qword ptr [rbp + 0x590], xmm0
140018da3: mov dword ptr [rbp + 0x598], 0x3e051eb8
140018dad: jmp 0x140018e01 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140018daf: movsd xmm0, qword ptr [rip + 0x290caf9]
140018db7: movsd qword ptr [rbp + 0x590], xmm0
140018dbf: mov dword ptr [rbp + 0x598], 0x3e0f5c29
140018dc9: jmp 0x140018e01 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140018dcb: movsd xmm0, qword ptr [rip + 0x290cabd]
140018dd3: movsd qword ptr [rbp + 0x590], xmm0
140018ddb: mov dword ptr [rbp + 0x598], 0x3e570a3d
140018de5: jmp 0x140018e01 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140018de7: movsd xmm0, qword ptr [rip + 0x290ca91]
140018def: movsd qword ptr [rbp + 0x590], xmm0
140018df7: mov dword ptr [rbp + 0x598], 0x3e2147ae
140018e01: lea rcx, [rbp + 0x6c0]
140018e08: mov rdx, r12
140018e0b: call 0x1408bc460 ; country_core::resources::rt::impl$1::from
140018e10: test byte ptr [rbp + 0x6c0], 1
140018e17: je 0x140018e36
140018e19: mov rax, qword ptr [rbp + 0x6c8]
140018e20: mov qword ptr [rbp + 0x460], rax
140018e27: mov rcx, rdi
140018e2a: call 0x1400885d0 ; minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction
140018e2f: or eax, 0x1000000
140018e34: jmp 0x140018e3c ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140018e36: mov eax, dword ptr [rbp + 0x6c4]
140018e3c: mov rcx, qword ptr [rbp + 0x6a0]
140018e43: mov dword ptr [rcx + 0x58], eax
140018e46: mov qword ptr [rbp + 0x440], 0
140018e51: xorps xmm0, xmm0
140018e54: lea rax, [rbp + 0x450]
140018e5b: movups xmmword ptr [rax], xmm0
140018e5e: mov qword ptr [rbp + 0x448], 8
140018e69: mov qword ptr [rbp + 0x278], 0
140018e74: mov qword ptr [rbp + 0x280], 0x10
140018e7f: mov qword ptr [rbp + 0x288], 0
140018e8a: mov rax, qword ptr [r14 + 0x140]
140018e91: cmp rax, 2
140018e95: jb 0x140018ea5
140018e97: mov r15, qword ptr [r14 + 0x130]
140018e9e: mov rax, qword ptr [r14 + 0x138]
140018ea5: add qword ptr [rbp + 0x6a0], 0x40
140018ead: lea rax, [r15 + rax*4]
140018eb1: mov qword ptr [rbp + 0x538], rax
140018eb8: mov rax, qword ptr [rbp + 0x780]
140018ebf: dec rax
140018ec2: mov qword ptr [rbp + 0x580], rax
140018ec9: mov r8, r15
140018ecc: xor edi, edi
140018ece: mov qword ptr [rbp + 0x88], rbx
140018ed5: jmp 0x140018efc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140018ed7: nop word ptr [rax + rax]
140018ee0: mov r14, qword ptr [rbp + 0x7e0]
140018ee7: mov r8, qword ptr [rbp + 0x560]
140018eee: mov rdi, qword ptr [rbp + 0x698]
140018ef5: lea r12, [rbp + 0x590]
140018efc: cmp r8, qword ptr [rbp + 0x538]
140018f03: je 0x14001d368
140018f09: movzx r9d, word ptr [r8 + 2]
140018f0e: lea r10, [rdi + r9]
140018f12: test rdi, rdi
140018f15: sete bl
140018f18: cmp r10, qword ptr [rbp + 0x580]
140018f1f: sete r11b
140018f23: cmp r10, -1
140018f27: je 0x1400203d9
140018f2d: lea rcx, [r10 + 1]
140018f31: mov rax, rcx
140018f34: sub rax, rdi
140018f37: jb 0x1400203f1
140018f3d: cmp r10, qword ptr [rbp + 0x780]
140018f44: jae 0x14002040f
140018f4a: mov rdx, qword ptr [r14 + 0x78]
140018f4e: cmp r10, rdx
140018f51: jae 0x14002042e
140018f57: mov rdx, qword ptr [r14 + 0x60]
140018f5b: cmp r10, rdx
140018f5e: jae 0x140020446
140018f64: mov qword ptr [rbp + 0x808], rax
140018f6b: lea rax, [r8 + 4]
140018f6f: mov qword ptr [rbp + 0x560], rax
140018f76: lea rax, [rdi + rdi*2]
140018f7a: mov rdx, qword ptr [rbp + 0x648]
140018f81: lea r15, [rdx + rax*4]
140018f85: lea rax, [rdi*8]
140018f8d: add rax, qword ptr [r14 + 0x70]
140018f91: mov qword ptr [rbp + 0x7a0], rax
140018f98: lea r13, [rdi*4]
140018fa0: mov rax, qword ptr [r14 + 0x58]
140018fa4: add rax, r13
140018fa7: mov qword ptr [rbp + 0x6b0], rax
140018fae: mov rdx, qword ptr [r14 + 0x90]
140018fb5: test rdx, rdx
140018fb8: mov qword ptr [rbp + 0x698], r10
140018fbf: mov byte ptr [rbp + 0x588], r11b
140018fc6: mov byte ptr [rbp + 0x678], bl
140018fcc: mov qword ptr [rbp + 0x430], rdi
140018fd3: je 0x140019050
140018fd5: cmp r10, rdx
140018fd8: pxor xmm1, xmm1
140018fdc: jae 0x1400204a9
140018fe2: add r13, qword ptr [r14 + 0x88]
140018fe9: movzx eax, byte ptr [r8]
140018fed: movzx ecx, byte ptr [r8 + 1]
140018ff2: mov byte ptr [rbp + 0x806], al
140018ff8: mov byte ptr [rbp + 0x807], cl
140018ffe: cmp qword ptr [rbp + 0x808], 0
140019006: je 0x14001906c
140019008: movzx ecx, al
14001900b: lea rdx, [r9*4 + 4]
140019013: xor r8d, r8d
140019016: nop word ptr cs:[rax + rax]
140019020: cmp rdx, r8
140019023: je 0x14001907c
140019025: movss xmm0, dword ptr [r13 + r8]
14001902c: add r8, 4
140019030: ucomiss xmm0, xmm1
140019033: ja 0x140019020
140019035: mov qword ptr [rbp + 0x7d8], 0
140019040: jmp 0x140019100 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140019045: nop word ptr cs:[rax + rax]
140019050: mov r13, qword ptr [r14 + 0x88]
140019057: movzx eax, byte ptr [r8]
14001905b: movzx ecx, byte ptr [r8 + 1]
140019060: mov byte ptr [rbp + 0x806], al
140019066: mov byte ptr [rbp + 0x807], cl
14001906c: mov cl, 1
14001906e: mov qword ptr [rbp + 0x7d8], rcx
140019075: xor edx, edx
140019077: jmp 0x140019107 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001907c: lea edx, [rax - 2]
14001907f: dec rcx
140019082: xor r8d, r8d
140019085: cmp dl, 3
140019088: mov edx, 0
14001908d: mov qword ptr [rbp + 0x7d8], rdx
140019094: cmovae rcx, r8
140019098: test rcx, rcx
14001909b: je 0x1400190a3
14001909d: cmp rcx, 3
1400190a1: jne 0x140019100
1400190a3: mov byte ptr [rbp + 0x806], 2
1400190aa: lea rax, [rbp + 0x806]
1400190b1: mov qword ptr [rbp + 0x290], rax
1400190b8: mov rax, qword ptr [rbp + 0x958]
1400190bf: mov qword ptr [rbp + 0x298], rax
1400190c6: mov rax, qword ptr [rbp + 0x670]
1400190cd: mov qword ptr [rbp + 0x2a0], rax
1400190d4: mov rax, qword ptr [rbp + 0x248]
1400190db: mov qword ptr [rbp + 0x2a8], rax
1400190e2: mov qword ptr [rbp + 0x7d8], 0
1400190ed: mov rax, qword ptr [rbp + 0x808]
1400190f4: mov qword ptr [rbp + 0x7f0], rax
1400190fb: jmp 0x14001914e ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400190fd: nop dword ptr [rax]
140019100: mov rdx, qword ptr [rbp + 0x808]
140019107: lea rcx, [rbp + 0x806]
14001910e: mov qword ptr [rbp + 0x290], rcx
140019115: mov rcx, qword ptr [rbp + 0x958]
14001911c: mov qword ptr [rbp + 0x298], rcx
140019123: mov rcx, qword ptr [rbp + 0x670]
14001912a: mov qword ptr [rbp + 0x2a0], rcx
140019131: mov rcx, qword ptr [rbp + 0x248]
140019138: mov qword ptr [rbp + 0x2a8], rcx
14001913f: cmp al, 3
140019141: mov qword ptr [rbp + 0x7f0], rdx
140019148: je 0x140019bba
14001914e: mov rax, qword ptr [rbp + 0x200]
140019155: mov byte ptr [rbp + 0x817], 1
14001915c: mov qword ptr [rsp + 0x48], rax
140019161: mov rax, qword ptr [rbp + 0x6a0]
140019168: mov qword ptr [rsp + 0x40], rax
14001916d: lea rax, [rbp + 0x278]
140019174: mov qword ptr [rsp + 0x38], rax
140019179: mov rsi, qword ptr [rbp + 0x808]
140019180: mov qword ptr [rsp + 0x28], rsi
140019185: mov rax, qword ptr [rbp + 0x6b0]
14001918c: mov qword ptr [rsp + 0x20], rax
140019191: mov qword ptr [rsp + 0x30], 1
14001919a: mov rcx, r15
14001919d: mov rdx, rsi
1400191a0: mov r8, qword ptr [rbp + 0x7a0]
1400191a7: mov r9, rsi
1400191aa: call 0x1407de8b0 ; system_decorator::decorator_visual::stairs_assemble::gen_stairs_collider_and_rt_geo
1400191af: movzx eax, byte ptr [rbp + 0x806]
1400191b6: mov byte ptr [rbp + 0x7d0], al
1400191bc: mov rax, qword ptr [rbp + 0x670]
1400191c3: mov rax, qword ptr [rax]
1400191c6: mov qword ptr [rbp + 0x568], rax
1400191cd: mov rax, rsi
1400191d0: mov ecx, 0xc
1400191d5: mul rcx
1400191d8: mov rbx, rax
1400191db: jo 0x14002083d
1400191e1: mov r12d, 0
1400191e7: movabs rax, 0x7ffffffffffffffc
1400191f1: cmp rbx, rax
1400191f4: ja 0x140020840
1400191fa: test rbx, rbx
1400191fd: je 0x140019240
1400191ff: movzx eax, byte ptr [rip + 0x361ba03]
140019206: mov r12d, 4
14001920c: mov edx, 4
140019211: mov rcx, rbx
140019214: call 0x140613c10 ; ?
140019219: test rax, rax
14001921c: je 0x140020840
140019222: mov rsi, rax
140019225: mov r14, qword ptr [rbp + 0x808]
14001922c: mov rdi, r14
14001922f: jmp 0x14001924e ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140019231: nop word ptr cs:[rax + rax]
140019240: mov esi, 4
140019245: xor edi, edi
140019247: mov r14, qword ptr [rbp + 0x808]
14001924e: mov rcx, rsi
140019251: mov rdx, r15
140019254: mov r8, rbx
140019257: call 0x142923c00 ; ?
14001925c: mov qword ptr [rbp + 0x590], rdi
140019263: mov qword ptr [rbp + 0x598], rsi
14001926a: mov qword ptr [rbp + 0x5a0], r14
140019271: mov byte ptr [rbp + 0x817], 1
140019278: lea rcx, [rbp + 0x6c0]
14001927f: lea rdx, [rbp + 0x590]
140019286: mov r8d, 1
14001928c: call 0x140153640 ; utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
140019291: xor eax, eax
140019293: cmp rax, qword ptr [rbp + 0x6c0]
14001929a: jo 0x140018ee0
1400192a0: mov rax, qword ptr [rbp + 0x6f0]
1400192a7: mov qword ptr [rbp + 0x3b0], rax
1400192ae: movups xmm0, xmmword ptr [rbp + 0x6c0]
1400192b5: movups xmm1, xmmword ptr [rbp + 0x6d0]
1400192bc: movdqu xmm2, xmmword ptr [rbp + 0x6e0]
1400192c4: movdqa xmmword ptr [rbp + 0x3a0], xmm2
1400192cc: movaps xmmword ptr [rbp + 0x390], xmm1
1400192d3: movaps xmmword ptr [rbp + 0x380], xmm0
1400192da: mov rbx, qword ptr [rbp + 0x390]
1400192e1: mov rax, rbx
1400192e4: mov ecx, 0xc
1400192e9: mul rcx
1400192ec: mov r12, rax
1400192ef: jo 0x140020894
1400192f5: mov esi, 0
1400192fa: movabs rax, 0x7ffffffffffffffc
140019304: cmp r12, rax
140019307: ja 0x140020896
14001930d: mov r14, qword ptr [rbp + 0x388]
140019314: test r12, r12
140019317: mov qword ptr [rbp + 0x528], rbx
14001931e: je 0x14001934e
140019320: movzx eax, byte ptr [rip + 0x361b8e2]
140019327: mov esi, 4
14001932c: mov edx, 4
140019331: mov rcx, r12
140019334: call 0x140613c10 ; ?
140019339: test rax, rax
14001933c: je 0x140020896
140019342: mov rdi, rax
140019345: mov qword ptr [rbp + 0x790], rbx
14001934c: jmp 0x14001935e ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001934e: mov edi, 4
140019353: mov qword ptr [rbp + 0x790], 0
14001935e: mov rbx, r13
140019361: mov rcx, rdi
140019364: mov rdx, r14
140019367: mov r8, r12
14001936a: call 0x142923c00 ; ?
14001936f: mov r15, qword ptr [rbp + 0x3a8]
140019376: lea r12, [r15*4]
14001937e: mov rax, r15
140019381: shr rax, 0x3e
140019385: jne 0x14002091e
14001938b: mov r14d, 0
140019391: movabs rax, 0x7ffffffffffffffc
14001939b: cmp r12, rax
14001939e: ja 0x140020921
1400193a4: mov rdx, qword ptr [rbp + 0x3a0]
1400193ab: test r12, r12
1400193ae: je 0x1400193e4
1400193b0: mov r13, rdx
1400193b3: movzx eax, byte ptr [rip + 0x361b84f]
1400193ba: mov r14d, 4
1400193c0: mov edx, 4
1400193c5: mov rcx, r12
1400193c8: call 0x140613c10 ; ?
1400193cd: test rax, rax
1400193d0: je 0x140020921
1400193d6: mov rsi, rax
1400193d9: mov r14, rdi
1400193dc: mov rdi, r15
1400193df: mov rdx, r13
1400193e2: jmp 0x1400193ee ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400193e4: mov r14, rdi
1400193e7: mov esi, 4
1400193ec: xor edi, edi
1400193ee: mov r13, rbx
1400193f1: mov rcx, rsi
1400193f4: mov r8, r12
1400193f7: call 0x142923c00 ; ?
1400193fc: movss xmm0, dword ptr [rbp + 0x3b0]
140019404: mov rax, qword ptr [rbp + 0x790]
14001940b: mov qword ptr [rbp + 0x6c0], rax
140019412: mov qword ptr [rbp + 0x6c8], r14
140019419: mov rax, qword ptr [rbp + 0x528]
140019420: mov qword ptr [rbp + 0x6d0], rax
140019427: mov qword ptr [rbp + 0x6d8], rdi
14001942e: mov qword ptr [rbp + 0x6e0], rsi
140019435: mov qword ptr [rbp + 0x6e8], r15
14001943c: movss dword ptr [rbp + 0x6f0], xmm0
140019444: lea rcx, [rbp + 0x590]
14001944b: lea rdx, [rbp + 0x6c0]
140019452: movd xmm2, dword ptr [rip + 0x290c416]
14001945a: mov r9d, 1
140019460: call 0x140152b00 ; utils::curve::Curve::try_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
140019465: mov r14, qword ptr [rbp + 0x7e0]
14001946c: mov rax, qword ptr [rbp + 0x590]
140019473: mov rcx, rax
140019476: neg rcx
140019479: jno 0x140019520
14001947f: mov rax, qword ptr [rbp + 0x6c0]
140019486: test rax, rax
140019489: je 0x1400194a5
14001948b: mov rcx, qword ptr [rbp + 0x6c8]
140019492: shl rax, 2
140019496: lea rdx, [rax + rax*2]
14001949a: mov r8d, 4
1400194a0: call 0x140613c20 ; ?
1400194a5: mov rdx, qword ptr [rbp + 0x6d8]
1400194ac: test rdx, rdx
1400194af: je 0x1400194c7
1400194b1: mov rcx, qword ptr [rbp + 0x6e0]
1400194b8: shl rdx, 2
1400194bc: mov r8d, 4
1400194c2: call 0x140613c20 ; ?
1400194c7: mov rax, qword ptr [rbp + 0x380]
1400194ce: test rax, rax
1400194d1: je 0x1400194ed
1400194d3: mov rcx, qword ptr [rbp + 0x388]
1400194da: shl rax, 2
1400194de: lea rdx, [rax + rax*2]
1400194e2: mov r8d, 4
1400194e8: call 0x140613c20 ; ?
1400194ed: mov rdx, qword ptr [rbp + 0x398]
1400194f4: test rdx, rdx
1400194f7: je 0x140018ee7
1400194fd: mov rcx, qword ptr [rbp + 0x3a0]
140019504: shl rdx, 2
140019508: mov r8d, 4
14001950e: call 0x140613c20 ; ?
140019513: jmp 0x140018ee7 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140019518: nop dword ptr [rax + rax]
140019520: lea rcx, [rbp + 0x598]
140019527: movups xmm0, xmmword ptr [rcx]
14001952a: movups xmm1, xmmword ptr [rcx + 0x10]
14001952e: movdqu xmm2, xmmword ptr [rcx + 0x20]
140019533: lea rcx, [rbp + 0x468]
14001953a: movdqu xmmword ptr [rcx + 0x20], xmm2
14001953f: movups xmmword ptr [rcx + 0x10], xmm1
140019543: movups xmmword ptr [rcx], xmm0
140019546: mov qword ptr [rbp + 0x460], rax
14001954d: mov rax, qword ptr [rbp + 0x6c0]
140019554: test rax, rax
140019557: je 0x140019573
140019559: mov rcx, qword ptr [rbp + 0x6c8]
140019560: shl rax, 2
140019564: lea rdx, [rax + rax*2]
140019568: mov r8d, 4
14001956e: call 0x140613c20 ; ?
140019573: mov rdx, qword ptr [rbp + 0x6d8]
14001957a: test rdx, rdx
14001957d: je 0x140019595
14001957f: mov rcx, qword ptr [rbp + 0x6e0]
140019586: shl rdx, 2
14001958a: mov r8d, 4
140019590: call 0x140613c20 ; ?
140019595: cmp byte ptr [rbp + 0x7d8], 0
14001959c: jne 0x1400208ad
1400195a2: mov rdi, qword ptr [rbp + 0x470]
1400195a9: test rdi, rdi
1400195ac: mov qword ptr [rbp + 0x558], r13
1400195b3: je 0x1400196d5
1400195b9: lea rsi, [rdi*4]
1400195c1: movzx eax, byte ptr [rip + 0x361b641]
1400195c8: mov edx, 4
1400195cd: mov rcx, rsi
1400195d0: call 0x140613c10 ; ?
1400195d5: test rax, rax
1400195d8: je 0x14002093f
1400195de: mov rbx, qword ptr [rbp + 0x470]
1400195e5: mov qword ptr [rbp + 0x650], rdi
1400195ec: mov qword ptr [rbp + 0x658], rax
1400195f3: mov qword ptr [rbp + 0x660], 0
1400195fe: test rbx, rbx
140019601: je 0x1400196f6
140019607: lea rsi, [rbx*8]
14001960f: movzx eax, byte ptr [rip + 0x361b5f3]
140019616: mov edx, 4
14001961b: mov rcx, rsi
14001961e: call 0x140613c10 ; ?
140019623: test rax, rax
140019626: je 0x1400209ba
14001962c: mov rdi, qword ptr [rbp + 0x470]
140019633: mov qword ptr [rbp + 0x680], rbx
14001963a: mov qword ptr [rbp + 0x688], rax
140019641: mov qword ptr [rbp + 0x690], 0
14001964c: test rdi, rdi
14001964f: je 0x140019717
140019655: lea rax, [rdi*4]
14001965d: lea rsi, [rax + rax*2]
140019661: movzx eax, byte ptr [rip + 0x361b5a1]
140019668: mov edx, 4
14001966d: mov rcx, rsi
140019670: call 0x140613c10 ; ?
140019675: test rax, rax
140019678: je 0x1400209ee
14001967e: mov rbx, qword ptr [rbp + 0x470]
140019685: mov qword ptr [rbp + 0x508], rdi
14001968c: mov qword ptr [rbp + 0x510], rax
140019693: mov qword ptr [rbp + 0x518], 0
14001969e: test rbx, rbx
1400196a1: je 0x140019738
1400196a7: lea rsi, [rbx*4]
1400196af: movzx eax, byte ptr [rip + 0x361b553]
1400196b6: mov edx, 4
1400196bb: mov rcx, rsi
1400196be: call 0x140613c10 ; ?
1400196c3: test rax, rax
1400196c6: je 0x140020a07
1400196cc: mov r14, qword ptr [rbp + 0x470]
1400196d3: jmp 0x140019742 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400196d5: mov qword ptr [rbp + 0x650], 0
1400196e0: mov qword ptr [rbp + 0x658], 4
1400196eb: mov qword ptr [rbp + 0x660], 0
1400196f6: mov qword ptr [rbp + 0x680], 0
140019701: mov qword ptr [rbp + 0x688], 4
14001970c: mov qword ptr [rbp + 0x690], 0
140019717: mov qword ptr [rbp + 0x508], 0
140019722: mov qword ptr [rbp + 0x510], 4
14001972d: mov qword ptr [rbp + 0x518], 0
140019738: mov eax, 4
14001973d: xor ebx, ebx
14001973f: xor r14d, r14d
140019742: mov qword ptr [rbp + 0x590], rbx
140019749: mov qword ptr [rbp + 0x598], rax
140019750: mov qword ptr [rbp + 0x5a0], 0
14001975b: mov rax, qword ptr [rbp + 0x468]
140019762: mov qword ptr [rbp + 0x7d8], rax
140019769: mov rax, qword ptr [rbp + 0x480]
140019770: mov qword ptr [rbp + 0x790], rax
140019777: mov rax, qword ptr [rbp + 0x488]
14001977e: cmp rax, r14
140019781: cmovb r14, rax
140019785: xor r12d, r12d
140019788: xor r13d, r13d
14001978b: jmp 0x1400197b8 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001978d: nop dword ptr [rax]
140019790: inc r13
140019793: subss xmm7, xmm6
140019797: mulss xmm11, xmm7
14001979c: addss xmm11, xmm6
1400197a1: mov rax, qword ptr [rbp + 0x598]
1400197a8: movss dword ptr [rax + r12*4], xmm11
1400197ae: inc r12
1400197b1: mov qword ptr [rbp + 0x5a0], r12
1400197b8: cmp r13, r14
1400197bb: jae 0x1400199f0
1400197c1: lea rax, [r13*2]
1400197c9: add rax, r13
1400197cc: mov rcx, qword ptr [rbp + 0x7d8]
1400197d3: mov r15d, dword ptr [rcx + rax*4]
1400197d7: movss xmm6, dword ptr [rcx + rax*4 + 4]
1400197dd: mov esi, dword ptr [rcx + rax*4 + 8]
1400197e1: mov rax, qword ptr [rbp + 0x790]
1400197e8: movss xmm1, dword ptr [rax + r13*4]
1400197ee: lea rcx, [rbp + 0x380]
1400197f5: call 0x140153380 ; utils::curve::Curve::get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> >
1400197fa: mov r12d, eax
1400197fd: mov rdx, qword ptr [rbp + 0x808]
140019804: cmp rdx, r12
140019807: jbe 0x1400206cc
14001980d: lea rbx, [r12 + 1]
140019812: cmp rbx, rdx
140019815: jae 0x1400206a4
14001981b: movaps xmm11, xmm0
14001981f: mov rax, qword ptr [rbp + 0x7a0]
140019826: movsd xmm7, qword ptr [rax + r12*8]
14001982c: movsd xmm8, qword ptr [rax + r12*8 + 8]
140019833: mov rax, qword ptr [rbp + 0x6b0]
14001983a: movss xmm10, dword ptr [rax + r12*4]
140019840: movss xmm9, dword ptr [rax + r12*4 + 4]
140019847: cmp byte ptr [rbp + 0x7d0], 2
14001984e: jae 0x1400198b5
140019850: movd xmm1, r15d
140019855: movd xmm2, esi
140019859: mov rdx, qword ptr [rbp + 0x240]
140019860: mov rax, qword ptr [rdx + 8]
140019864: mov rcx, qword ptr [rdx + 0x10]
140019868: add rcx, rcx
14001986b: mov qword ptr [rbp + 0x6c0], rax
140019872: mov qword ptr [rbp + 0x6c8], rcx
140019879: movsd xmm0, qword ptr [rdx + 0x50]
14001987e: movsd qword ptr [rbp + 0x6d0], xmm0
140019886: lea rcx, [rbp + 0x6c0]
14001988d: call 0x140257ba0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>
140019892: addss xmm0, dword ptr [rip + 0x290c086]
14001989a: movaps xmm1, xmm6
14001989d: cmpunordss xmm1, xmm6
1400198a2: movaps xmm2, xmm1
1400198a5: andps xmm2, xmm0
1400198a8: maxss xmm0, xmm6
1400198ac: andnps xmm1, xmm0
1400198af: orps xmm1, xmm2
1400198b2: movaps xmm6, xmm1
1400198b5: mov rdi, qword ptr [rbp + 0x660]
1400198bc: cmp rdi, qword ptr [rbp + 0x650]
1400198c3: jne 0x1400198d8
1400198c5: lea rcx, [rbp + 0x650]
1400198cc: lea rdx, [rip + 0x290cad5]
1400198d3: call 0x140650050 ; alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>
1400198d8: subss xmm9, xmm10
1400198dd: mulss xmm9, xmm11
1400198e2: addss xmm9, xmm10
1400198e7: mov rax, qword ptr [rbp + 0x658]
1400198ee: movss dword ptr [rax + rdi*4], xmm9
1400198f4: inc rdi
1400198f7: mov qword ptr [rbp + 0x660], rdi
1400198fe: mov rdi, qword ptr [rbp + 0x690]
140019905: cmp rdi, qword ptr [rbp + 0x680]
14001990c: jne 0x140019921
14001990e: lea rcx, [rbp + 0x680]
140019915: lea rdx, [rip + 0x290caa4]
14001991c: call 0x140b81f90 ; alloc::raw_vec::RawVec::grow_one<tuple$<u8,u32>,alloc::alloc::Global>
140019921: subps xmm8, xmm7
140019925: movsldup xmm0, xmm11
14001992a: mulps xmm0, xmm8
14001992e: addps xmm0, xmm7
140019931: mov rax, qword ptr [rbp + 0x688]
140019938: movlps qword ptr [rax + rdi*8], xmm0
14001993c: inc rdi
14001993f: mov qword ptr [rbp + 0x690], rdi
140019946: mov rdi, qword ptr [rbp + 0x518]
14001994d: cmp rdi, qword ptr [rbp + 0x508]
140019954: jne 0x140019969
140019956: lea rcx, [rbp + 0x508]
14001995d: lea rdx, [rip + 0x290ca74]
140019964: call 0x140b9f180 ; alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>
140019969: mov rax, qword ptr [rbp + 0x510]
140019970: lea rcx, [rdi + rdi*2]
140019974: mov dword ptr [rax + rcx*4], r15d
140019978: movss dword ptr [rax + rcx*4 + 4], xmm6
14001997e: mov dword ptr [rax + rcx*4 + 8], esi
140019982: inc rdi
140019985: mov qword ptr [rbp + 0x518], rdi
14001998c: mov rdx, qword ptr [rbp + 0x7f0]
140019993: cmp rdx, r12
140019996: jbe 0x140020690
14001999c: cmp rbx, rdx
14001999f: jae 0x1400206b8
1400199a5: mov rax, qword ptr [rbp + 0x558]
1400199ac: movss xmm6, dword ptr [rax + r12*4]
1400199b2: movss xmm7, dword ptr [rax + r12*4 + 4]
1400199b9: mov r12, qword ptr [rbp + 0x5a0]
1400199c0: cmp r12, qword ptr [rbp + 0x590]
1400199c7: jne 0x140019790
1400199cd: lea rcx, [rbp + 0x590]
1400199d4: lea rdx, [rip + 0x290ca15]
1400199db: call 0x140650050 ; alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>
1400199e0: jmp 0x140019790 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400199e5: nop word ptr cs:[rax + rax]
1400199f0: mov rsi, qword ptr [rbp + 0x650]
1400199f7: lea rax, [rbp + 0x658]
1400199fe: movups xmm0, xmmword ptr [rax]
140019a01: movaps xmmword ptr [rbp + 0x6c0], xmm0
140019a08: mov rax, qword ptr [rbp + 0x690]
140019a0f: lea rcx, [rbp + 0x6f8]
140019a16: mov qword ptr [rcx - 0x18], rax
140019a1a: movups xmm0, xmmword ptr [rbp + 0x680]
140019a21: movups xmmword ptr [rcx - 0x28], xmm0
140019a25: movups xmm0, xmmword ptr [rbp + 0x508]
140019a2c: movaps xmmword ptr [rbp + 0x300], xmm0
140019a33: mov rax, qword ptr [rbp + 0x518]
140019a3a: mov qword ptr [rbp + 0x310], rax
140019a41: mov rbx, qword ptr [rbp + 0x590]
140019a48: mov r15, qword ptr [rbp + 0x598]
140019a4f: movaps xmm0, xmmword ptr [rbp + 0x6c0]
140019a56: movaps xmm1, xmmword ptr [rbp + 0x6d0]
140019a5d: movaps xmmword ptr [rbp + 0x3e0], xmm0
140019a64: movaps xmmword ptr [rbp + 0x3f0], xmm1
140019a6b: mov rax, qword ptr [rbp + 0x6e0]
140019a72: mov qword ptr [rbp + 0x400], rax
140019a79: mov rax, qword ptr [rbp + 0x460]
140019a80: test rax, rax
140019a83: je 0x140019a9f
140019a85: mov rcx, qword ptr [rbp + 0x468]
140019a8c: shl rax, 2
140019a90: lea rdx, [rax + rax*2]
140019a94: mov r8d, 4
140019a9a: call 0x140613c20 ; ?
140019a9f: mov rdx, qword ptr [rbp + 0x478]
140019aa6: test rdx, rdx
140019aa9: mov edi, 0
140019aae: mov r14, qword ptr [rbp + 0x7e0]
140019ab5: je 0x140019acd
140019ab7: mov rcx, qword ptr [rbp + 0x480]
140019abe: shl rdx, 2
140019ac2: mov r8d, 4
140019ac8: call 0x140613c20 ; ?
140019acd: mov rax, qword ptr [rbp + 0x380]
140019ad4: test rax, rax
140019ad7: je 0x140019af3
140019ad9: mov rcx, qword ptr [rbp + 0x388]
140019ae0: shl rax, 2
140019ae4: lea rdx, [rax + rax*2]
140019ae8: mov r8d, 4
140019aee: call 0x140613c20 ; ?
140019af3: mov rdx, qword ptr [rbp + 0x398]
140019afa: test rdx, rdx
140019afd: je 0x140019b15
140019aff: mov rcx, qword ptr [rbp + 0x3a0]
140019b06: shl rdx, 2
140019b0a: mov r8d, 4
140019b10: call 0x140613c20 ; ?
140019b15: mov rax, rsi
140019b18: neg rax
140019b1b: jo 0x140018ee7
140019b21: movaps xmm0, xmmword ptr [rbp + 0x3e0]
140019b28: lea rax, [rbp + 0x570]
140019b2f: movups xmmword ptr [rax], xmm0
140019b32: lea rcx, [rbp + 0x3f0]
140019b39: mov rax, qword ptr [rcx + 0x10]
140019b3d: mov qword ptr [rbp + 0x660], rax
140019b44: movups xmm0, xmmword ptr [rcx]
140019b47: movaps xmmword ptr [rbp + 0x650], xmm0
140019b4e: mov rax, qword ptr [rbp + 0x310]
140019b55: mov qword ptr [rbp + 0x690], rax
140019b5c: movaps xmm0, xmmword ptr [rbp + 0x300]
140019b63: movaps xmmword ptr [rbp + 0x680], xmm0
140019b6a: mov qword ptr [rbp + 0x568], rsi
140019b71: movzx esi, byte ptr [rbp + 0x806]
140019b78: lea eax, [rsi - 2]
140019b7b: mov rcx, rsi
140019b7e: dec rcx
140019b81: cmp al, 3
140019b83: cmovae rcx, rdi
140019b87: lea rdx, [rip + 0x290dd62]
140019b8e: movsxd rax, dword ptr [rdx + rcx*4]
140019b92: add rax, rdx
140019b95: mov qword ptr [rbp + 0x7d0], rbx
140019b9c: mov qword ptr [rbp + 0x7f0], r15
140019ba3: jmp rax
140019ba5: cmp sil, 4
140019ba9: jne 0x14001a1f9
140019baf: xor r14d, r14d
140019bb2: xor r15d, r15d
140019bb5: jmp 0x14001a5f4 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140019bba: mov rax, qword ptr [rbp + 0x808]
140019bc1: mov qword ptr [rbp + 0x78], rax
140019bc5: cmp rax, 2
140019bc9: jne 0x1400205b8
140019bcf: movsd xmm15, qword ptr [r15]
140019bd4: movss xmm12, dword ptr [r15 + 8]
140019bda: movsd xmm13, qword ptr [r15 + 0xc]
140019be0: movss xmm14, dword ptr [r15 + 0x14]
140019be6: mov rax, qword ptr [rbp + 0x950]
140019bed: mov rax, qword ptr [rax]
140019bf0: mov ecx, dword ptr [r14 + 0x128]
140019bf7: mov edx, dword ptr [r14 + 0x12c]
140019bfe: movsd qword ptr [rbp + 0x590], xmm15
140019c07: movss dword ptr [rbp + 0x598], xmm12
140019c10: movsd qword ptr [rbp + 0x6c0], xmm13
140019c19: movss dword ptr [rbp + 0x6c8], xmm14
140019c22: mov byte ptr [rbp + 0x817], 1
140019c29: mov dword ptr [rsp + 0x38], edx
140019c2d: mov dword ptr [rsp + 0x30], ecx
140019c31: mov rcx, qword ptr [rbp + 0x2d0]
140019c38: mov qword ptr [rsp + 0x28], rcx
140019c3d: mov qword ptr [rsp + 0x20], rax
140019c42: lea rcx, [rbp + 0x650]
140019c49: mov rdx, qword ptr [rbp + 0x2d8]
140019c50: mov r8, r12
140019c53: lea r9, [rbp + 0x6c0]
140019c5a: call 0x1407d8fe0 ; system_decorator::decorator_visual::stairs_assemble::calculate_adjusted_ladder_positions
140019c5f: mov r10, qword ptr [rbp + 0x7e0]
140019c66: mov r8d, dword ptr [r10 + 0x128]
140019c6d: mov r9d, dword ptr [r10 + 0x12c]
140019c74: mov dword ptr [rbp + 0x590], r8d
140019c7b: mov dword ptr [rbp + 0x594], r9d
140019c82: mov rax, qword ptr [rbp + 0x7b0]
140019c89: mov rdi, qword ptr [rax + 0x70]
140019c8d: test rdi, rdi
140019c90: lea rsi, [rbp + 0x590]
140019c97: je 0x140019da2
140019c9d: cmp rdi, 1
140019ca1: jne 0x140019cc6
140019ca3: mov rax, qword ptr [rbp + 0x7b0]
140019caa: mov rax, qword ptr [rax + 0x68]
140019cae: cmp dword ptr [rax + 8], r8d
140019cb2: jne 0x140019da2
140019cb8: cmp dword ptr [rax + 0xc], r9d
140019cbc: jne 0x140019da2
140019cc2: xor edx, edx
140019cc4: jmp 0x140019d1e ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140019cc6: mov rax, qword ptr [rbp + 0x7b0]
140019ccd: mov rcx, qword ptr [rax + 0x98]
140019cd4: mov rdx, qword ptr [rax + 0xa0]
140019cdb: call 0x140106840 ; indexmap::map::IndexMap::hash<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,country_core::resources::stairs::stairs_assembly_params::AdjustedStairEdge,std::hash::random::RandomState,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >
140019ce0: mov byte ptr [rbp + 0x817], 1
140019ce7: mov rcx, qword ptr [rbp + 0x340]
140019cee: mov rdx, rax
140019cf1: mov r8, rsi
140019cf4: call 0x1403ab1c0 ; indexmap::map::core::IndexMapCore::get_index_of<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2$<country_core::resources::stairs::stairs_assembly_params::AlignKind>,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >
140019cf9: cmp rax, 1
140019cfd: mov r10, qword ptr [rbp + 0x7e0]
140019d04: jne 0x140019da2
140019d0a: cmp rdx, rdi
140019d0d: jae 0x140020b82
140019d13: mov rax, qword ptr [rbp + 0x7b0]
140019d1a: mov rax, qword ptr [rax + 0x68]
140019d1e: lea rcx, [rdx + rdx*4]
140019d22: movups xmm0, xmmword ptr [rax + rcx*8 + 0x10]
140019d27: movaps xmmword ptr [rbp + 0x460], xmm0
140019d2e: mov eax, dword ptr [rax + rcx*8 + 0x20]
140019d32: mov dword ptr [rbp + 0x470], eax
140019d38: movzx ecx, byte ptr [rbp + 0x460]
140019d3f: mov byte ptr [rbp + 0x34c], 2
140019d46: mov eax, dword ptr [rbp + 0x35c]
140019d4c: mov dword ptr [rbp + 0x5a0], eax
140019d52: movzx eax, byte ptr [rbp + 0x34c]
140019d59: mov byte ptr [rbp + 0x590], al
140019d5f: mov rax, qword ptr [rbp + 0x34d]
140019d66: mov qword ptr [rbp + 0x591], rax
140019d6d: mov eax, dword ptr [rbp + 0x355]
140019d73: mov dword ptr [rbp + 0x599], eax
140019d79: movzx eax, word ptr [rbp + 0x359]
140019d80: mov word ptr [rbp + 0x59d], ax
140019d87: movzx eax, byte ptr [rbp + 0x35b]
140019d8e: mov byte ptr [rbp + 0x59f], al
140019d94: lea rax, [rbp + 0x460]
140019d9b: cmp cl, 3
140019d9e: je 0x140019dfe
140019da0: jmp 0x140019e01 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140019da2: mov byte ptr [rbp + 0x460], 3
140019da9: mov byte ptr [rbp + 0x34c], 2
140019db0: mov eax, dword ptr [rbp + 0x35c]
140019db6: mov dword ptr [rbp + 0x5a0], eax
140019dbc: movzx eax, byte ptr [rbp + 0x34c]
140019dc3: mov byte ptr [rbp + 0x590], al
140019dc9: mov rax, qword ptr [rbp + 0x34d]
140019dd0: mov qword ptr [rbp + 0x591], rax
140019dd7: mov eax, dword ptr [rbp + 0x355]
140019ddd: mov dword ptr [rbp + 0x599], eax
140019de3: movzx eax, word ptr [rbp + 0x359]
140019dea: mov word ptr [rbp + 0x59d], ax
140019df1: movzx eax, byte ptr [rbp + 0x35b]
140019df8: mov byte ptr [rbp + 0x59f], al
140019dfe: mov rax, rsi
140019e01: mov ecx, dword ptr [rax + 0x10]
140019e04: mov dword ptr [rbp + 0x390], ecx
140019e0a: movups xmm0, xmmword ptr [rax]
140019e0d: movaps xmmword ptr [rbp + 0x380], xmm0
140019e14: mov r9d, dword ptr [r10 + 0x128]
140019e1b: mov r8d, dword ptr [r10 + 0x12c]
140019e22: mov dword ptr [rbp + 0x590], r8d
140019e29: mov dword ptr [rbp + 0x594], r9d
140019e30: mov rax, qword ptr [rbp + 0x7b0]
140019e37: mov rdi, qword ptr [rax + 0x70]
140019e3b: test rdi, rdi
140019e3e: je 0x140019eeb
140019e44: cmp rdi, 1
140019e48: jne 0x140019e6d
140019e4a: mov rax, qword ptr [rbp + 0x7b0]
140019e51: mov rax, qword ptr [rax + 0x68]
140019e55: cmp dword ptr [rax + 8], r8d
140019e59: jne 0x140019eeb
140019e5f: cmp dword ptr [rax + 0xc], r9d
140019e63: jne 0x140019eeb
140019e69: xor edx, edx
140019e6b: jmp 0x140019eba ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140019e6d: mov rax, qword ptr [rbp + 0x7b0]
140019e74: mov rcx, qword ptr [rax + 0x98]
140019e7b: mov rdx, qword ptr [rax + 0xa0]
140019e82: call 0x140106840 ; indexmap::map::IndexMap::hash<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,country_core::resources::stairs::stairs_assembly_params::AdjustedStairEdge,std::hash::random::RandomState,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >
140019e87: mov byte ptr [rbp + 0x817], 1
140019e8e: mov rcx, qword ptr [rbp + 0x340]
140019e95: mov rdx, rax
140019e98: mov r8, rsi
140019e9b: call 0x1403ab1c0 ; indexmap::map::core::IndexMapCore::get_index_of<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2$<country_core::resources::stairs::stairs_assembly_params::AlignKind>,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >
140019ea0: cmp rax, 1
140019ea4: jne 0x140019eeb
140019ea6: cmp rdx, rdi
140019ea9: jae 0x140020ba0
140019eaf: mov rax, qword ptr [rbp + 0x7b0]
140019eb6: mov rax, qword ptr [rax + 0x68]
140019eba: lea rcx, [rdx + rdx*4]
140019ebe: movups xmm0, xmmword ptr [rax + rcx*8 + 0x10]
140019ec3: movaps xmmword ptr [rbp + 0x590], xmm0
140019eca: mov eax, dword ptr [rax + rcx*8 + 0x20]
140019ece: cmp byte ptr [rbp + 0x590], 3
140019ed5: mov dword ptr [rbp + 0x5a0], eax
140019edb: mov rax, rsi
140019ede: lea rcx, [rbp + 0x34c]
140019ee5: cmove rax, rcx
140019ee9: jmp 0x140019ef9 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140019eeb: mov byte ptr [rbp + 0x590], 3
140019ef2: lea rax, [rbp + 0x34c]
140019ef9: lea rcx, [rbp + 0x460]
140019f00: mov r8d, dword ptr [rax + 0x10]
140019f04: lea rdx, [rbp + 0x6f8]
140019f0b: mov dword ptr [rdx - 0x14], r8d
140019f0f: movups xmm0, xmmword ptr [rax]
140019f12: movups xmmword ptr [rdx - 0x24], xmm0
140019f16: movaps xmm0, xmmword ptr [rbp + 0x380]
140019f1d: movaps xmmword ptr [rbp + 0x6c0], xmm0
140019f24: mov eax, dword ptr [rbp + 0x390]
140019f2a: mov dword ptr [rbp + 0x6d0], eax
140019f30: movlps qword ptr [rbp + 0x460], xmm15
140019f38: movss dword ptr [rbp + 0x468], xmm12
140019f41: movlps qword ptr [rbp + 0x590], xmm13
140019f49: movss dword ptr [rbp + 0x598], xmm14
140019f52: mov byte ptr [rbp + 0x817], 1
140019f59: mov rdx, rsi
140019f5c: mov r8, qword ptr [rbp + 0x2d8]
140019f63: lea r9, [rbp + 0x6c0]
140019f6a: call 0x1407de800 ; system_decorator::decorator_visual::stairs_assemble::determine_ladder_support
140019f6f: mov byte ptr [rbp + 0x804], al
140019f75: mov rax, qword ptr [rbp + 0x7a0]
140019f7c: movsd xmm6, qword ptr [rax]
140019f80: movsd xmm11, qword ptr [rax + 8]
140019f86: movups xmm0, xmmword ptr [rax]
140019f89: movaps xmmword ptr [rbp + 0x300], xmm0
140019f90: mov rax, qword ptr [rbp + 0x6b0]
140019f97: movsd xmm0, qword ptr [rax]
140019f9b: movsd qword ptr [rbp + 0x550], xmm0
140019fa3: mov rax, qword ptr [rbp + 0x7f0]
140019faa: cmp rax, 1
140019fae: je 0x140020b12
140019fb4: test rax, rax
140019fb7: je 0x140020af6
140019fbd: movsd xmm10, qword ptr [r13]
140019fc3: cmp byte ptr [rbp + 0x530], 0
140019fca: je 0x14001c469
140019fd0: mov byte ptr [rbp + 0x804], 0
140019fd7: movzx eax, byte ptr [rip + 0x361ac2b]
140019fde: mov ecx, 0x18
140019fe3: mov edx, 4
140019fe8: call 0x140613c10 ; ?
140019fed: test rax, rax
140019ff0: lea rcx, [rbp + 0x590]
140019ff7: je 0x140020b60
140019ffd: mov rdx, qword ptr [rbp + 0x660]
14001a004: mov qword ptr [rax + 0x10], rdx
14001a008: movups xmm0, xmmword ptr [rbp + 0x650]
14001a00f: movups xmmword ptr [rax], xmm0
14001a012: mov qword ptr [rbp + 0x6c0], 2
14001a01d: mov qword ptr [rbp + 0x6c8], rax
14001a024: mov qword ptr [rbp + 0x6d0], 2
14001a02f: mov byte ptr [rbp + 0x817], 1
14001a036: lea rdx, [rbp + 0x6c0]
14001a03d: xor r8d, r8d
14001a040: call 0x140153640 ; utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14001a045: xor esi, esi
14001a047: cmp rsi, qword ptr [rbp + 0x590]
14001a04e: jo 0x14001c469
14001a054: mov rax, qword ptr [rbp + 0x5c0]
14001a05b: mov qword ptr [rbp + 0x6f0], rax
14001a062: movups xmm0, xmmword ptr [rbp + 0x590]
14001a069: movups xmm1, xmmword ptr [rbp + 0x5a0]
14001a070: movups xmm2, xmmword ptr [rbp + 0x5b0]
14001a077: movaps xmmword ptr [rbp + 0x6e0], xmm2
14001a07e: movaps xmmword ptr [rbp + 0x6d0], xmm1
14001a085: movaps xmmword ptr [rbp + 0x6c0], xmm0
14001a08c: lea rcx, [rbp + 0x460]
14001a093: lea rdx, [rbp + 0x6c0]
14001a09a: movd xmm2, dword ptr [rip + 0x290b7ce]
14001a0a2: mov r9d, 1
14001a0a8: call 0x140152b00 ; utils::curve::Curve::try_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14001a0ad: mov rax, qword ptr [rbp + 0x6c0]
14001a0b4: test rax, rax
14001a0b7: mov r13, qword ptr [rbp + 0x338]
14001a0be: movaps xmm9, xmmword ptr [rip + 0x290b86a]
14001a0c6: je 0x14001a0e2
14001a0c8: mov rcx, qword ptr [rbp + 0x6c8]
14001a0cf: shl rax, 2
14001a0d3: lea rdx, [rax + rax*2]
14001a0d7: mov r8d, 4
14001a0dd: call 0x140613c20 ; ?
14001a0e2: mov rdx, qword ptr [rbp + 0x6d8]
14001a0e9: test rdx, rdx
14001a0ec: je 0x14001a104
14001a0ee: mov rcx, qword ptr [rbp + 0x6e0]
14001a0f5: shl rdx, 2
14001a0f9: mov r8d, 4
14001a0ff: call 0x140613c20 ; ?
14001a104: cmp rsi, qword ptr [rbp + 0x460]
14001a10b: jo 0x14001c469
14001a111: movups xmm0, xmmword ptr [rbp + 0x460]
14001a118: movups xmm1, xmmword ptr [rbp + 0x470]
14001a11f: movups xmm2, xmmword ptr [rbp + 0x480]
14001a126: movaps xmmword ptr [rbp + 0x5a0], xmm1
14001a12d: movaps xmmword ptr [rbp + 0x5b0], xmm2
14001a134: mov rax, qword ptr [rbp + 0x490]
14001a13b: mov qword ptr [rbp + 0x5c0], rax
14001a142: movaps xmmword ptr [rbp + 0x590], xmm0
14001a149: mov rbx, qword ptr [rbp + 0x5a0]
14001a150: mov rax, qword ptr [rbp + 0x5b8]
14001a157: cmp rax, rbx
14001a15a: cmovb rbx, rax
14001a15e: lea rdi, [rbx*8]
14001a166: mov rax, rbx
14001a169: shr rax, 0x3d
14001a16d: jne 0x140020d2d
14001a173: mov esi, 0
14001a178: movabs rax, 0x7ffffffffffffffc
14001a182: cmp rdi, rax
14001a185: movss xmm8, dword ptr [rip + 0x290b776]
14001a18e: ja 0x140020d2f
14001a194: mov r14, qword ptr [rbp + 0x5b0]
14001a19b: test rdi, rdi
14001a19e: je 0x14001a22d
14001a1a4: movzx eax, byte ptr [rip + 0x361aa5e]
14001a1ab: mov esi, 4
14001a1b0: mov edx, 4
14001a1b5: mov rcx, rdi
14001a1b8: call 0x140613c10 ; ?
14001a1bd: test rax, rax
14001a1c0: je 0x140020d2f
14001a1c6: mov qword ptr [rbp + 0x808], rbx
14001a1cd: test rbx, rbx
14001a1d0: jne 0x14001a246
14001a1d2: jmp 0x14001a313 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001a1d7: cmp byte ptr [rbp + 0x6a8], 0
14001a1de: je 0x14001ad53
14001a1e4: cmp sil, 4
14001a1e8: jne 0x14001a972
14001a1ee: xor r14d, r14d
14001a1f1: xor r15d, r15d
14001a1f4: jmp 0x14001ac65 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001a1f9: cmp qword ptr [rbp + 0x430], 0
14001a201: je 0x14001a5cb
14001a207: xor eax, eax
14001a209: mov rcx, qword ptr [rbp + 0x698]
14001a210: cmp rcx, qword ptr [rbp + 0x580]
14001a217: jne 0x14001a5ea
14001a21d: cmp byte ptr [r14 + 0x108], 1
14001a225: sete cl
14001a228: jmp 0x14001a5ec ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001a22d: mov eax, 4
14001a232: mov qword ptr [rbp + 0x808], 0
14001a23d: test rbx, rbx
14001a240: je 0x14001a313
14001a246: cmp rbx, 8
14001a24a: jae 0x14001a401
14001a250: xor r8d, r8d
14001a253: mov rcx, r8
14001a256: or rcx, 1
14001a25a: test bl, 1
14001a25d: je 0x14001a296
14001a25f: movss xmm0, dword ptr [r14 + r8*4]
14001a265: movaps xmm1, xmm8
14001a269: subss xmm1, xmm0
14001a26d: movsd xmm2, qword ptr [rbp + 0x300]
14001a275: movsd xmm3, qword ptr [rbp + 0x308]
14001a27d: movsldup xmm1, xmm1
14001a281: mulps xmm1, xmm2
14001a284: movsldup xmm0, xmm0
14001a288: mulps xmm0, xmm3
14001a28b: addps xmm0, xmm1
14001a28e: movlps qword ptr [rax + r8*8], xmm0
14001a293: mov r8, rcx
14001a296: cmp rbx, rcx
14001a299: je 0x14001a313
14001a29f: nop 
14001a2a0: movss xmm0, dword ptr [r14 + r8*4]
14001a2a6: movaps xmm1, xmm8
14001a2aa: subss xmm1, xmm0
14001a2ae: movsd xmm2, qword ptr [rbp + 0x300]
14001a2b6: movsd xmm3, qword ptr [rbp + 0x308]
14001a2be: movsldup xmm1, xmm1
14001a2c2: mulps xmm1, xmm2
14001a2c5: movsldup xmm0, xmm0
14001a2c9: mulps xmm0, xmm3
14001a2cc: addps xmm0, xmm1
14001a2cf: movlps qword ptr [rax + r8*8], xmm0
14001a2d4: movss xmm0, dword ptr [r14 + r8*4 + 4]
14001a2db: movaps xmm1, xmm8
14001a2df: subss xmm1, xmm0
14001a2e3: movsd xmm2, qword ptr [rbp + 0x300]
14001a2eb: movsd xmm3, qword ptr [rbp + 0x308]
14001a2f3: movsldup xmm1, xmm1
14001a2f7: mulps xmm1, xmm2
14001a2fa: movsldup xmm0, xmm0
14001a2fe: mulps xmm0, xmm3
14001a301: addps xmm0, xmm1
14001a304: movlps qword ptr [rax + r8*8 + 8], xmm0
14001a30a: add r8, 2
14001a30e: cmp rbx, r8
14001a311: jne 0x14001a2a0
14001a313: mov qword ptr [rbp + 0x7a0], rax
14001a31a: mov r14, qword ptr [rbp + 0x5a0]
14001a321: mov rax, qword ptr [rbp + 0x5b8]
14001a328: cmp rax, r14
14001a32b: cmovb r14, rax
14001a32f: lea rdi, [r14*4]
14001a337: mov rax, r14
14001a33a: shr rax, 0x3e
14001a33e: jne 0x140020d4d
14001a344: mov esi, 0
14001a349: movabs rax, 0x7ffffffffffffffc
14001a353: cmp rdi, rax
14001a356: ja 0x140020d4f
14001a35c: mov r15, qword ptr [rbp + 0x5b0]
14001a363: test rdi, rdi
14001a366: je 0x14001a39b
14001a368: movzx eax, byte ptr [rip + 0x361a89a]
14001a36f: mov esi, 4
14001a374: mov edx, 4
14001a379: mov rcx, rdi
14001a37c: call 0x140613c10 ; ?
14001a381: test rax, rax
14001a384: je 0x140020d4f
14001a38a: mov qword ptr [rbp + 0x6b0], r14
14001a391: test r14, r14
14001a394: jne 0x14001a3b4
14001a396: jmp 0x14001c19f ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001a39b: mov eax, 4
14001a3a0: mov qword ptr [rbp + 0x6b0], 0
14001a3ab: test r14, r14
14001a3ae: je 0x14001c19f
14001a3b4: cmp r14, 8
14001a3b8: jae 0x14001a514
14001a3be: xor r8d, r8d
14001a3c1: mov rcx, r8
14001a3c4: or rcx, 1
14001a3c8: test r14b, 1
14001a3cc: je 0x14001c13b
14001a3d2: movss xmm0, dword ptr [r15 + r8*4]
14001a3d8: movaps xmm1, xmm8
14001a3dc: subss xmm1, xmm0
14001a3e0: movsd xmm2, qword ptr [rbp + 0x550]
14001a3e8: unpcklps xmm1, xmm0
14001a3eb: mulps xmm1, xmm2
14001a3ee: movshdup xmm0, xmm1
14001a3f2: addss xmm0, xmm1
14001a3f6: movss dword ptr [rax + r8*4], xmm0
14001a3fc: jmp 0x14001c19a ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001a401: lea rcx, [rax + rbx*8]
14001a405: add rcx, -8
14001a409: cmp rcx, rax
14001a40c: jb 0x14001a250
14001a412: lea rdx, [rax + 4]
14001a416: lea rcx, [rax + rbx*8]
14001a41a: add rcx, -4
14001a41e: cmp rcx, rdx
14001a421: jb 0x14001a250
14001a427: lea rcx, [rbx - 1]
14001a42b: shr rcx, 0x3d
14001a42f: mov r8d, 0
14001a435: jne 0x14001a253
14001a43b: add rdi, rax
14001a43e: lea rcx, [r14 + rbx*4]
14001a442: cmp rcx, rax
14001a445: seta r9b
14001a449: cmp r14, rdi
14001a44c: setb r8b
14001a450: lea rcx, [rbp + 0x310]
14001a457: cmp rcx, rax
14001a45a: seta cl
14001a45d: lea rdx, [rbp + 0x300]
14001a464: cmp rdx, rdi
14001a467: setb dl
14001a46a: test r9b, r8b
14001a46d: jne 0x14001a250
14001a473: mov r8d, 0
14001a479: and cl, dl
14001a47b: jne 0x14001a253
14001a481: movaps xmmword ptr [rbp + 0x7a0], xmm6
14001a488: mov r8, rbx
14001a48b: movabs rcx, 0x1ffffffffffffffc
14001a495: and r8, rcx
14001a498: movaps xmm0, xmmword ptr [rbp + 0x300]
14001a49f: movaps xmm1, xmm0
14001a4a2: shufps xmm1, xmm0, 0xff
14001a4a6: movaps xmm2, xmm0
14001a4a9: shufps xmm2, xmm0, 0xaa
14001a4ad: movaps xmm3, xmm0
14001a4b0: shufps xmm3, xmm0, 0x55
14001a4b4: shufps xmm0, xmm0, 0
14001a4b8: xor ecx, ecx
14001a4ba: nop word ptr [rax + rax]
14001a4c0: movups xmm4, xmmword ptr [r14 + rcx*4]
14001a4c5: movaps xmm5, xmm9
14001a4c9: subps xmm5, xmm4
14001a4cc: movaps xmm6, xmm5
14001a4cf: mulps xmm6, xmm0
14001a4d2: mulps xmm5, xmm3
14001a4d5: movaps xmm7, xmm4
14001a4d8: mulps xmm7, xmm2
14001a4db: addps xmm7, xmm6
14001a4de: mulps xmm4, xmm1
14001a4e1: addps xmm4, xmm5
14001a4e4: movaps xmm5, xmm7
14001a4e7: unpcklps xmm5, xmm4
14001a4ea: unpckhps xmm7, xmm4
14001a4ed: movups xmmword ptr [rax + rcx*8 + 0x10], xmm7
14001a4f2: movups xmmword ptr [rax + rcx*8], xmm5
14001a4f6: add rcx, 4
14001a4fa: cmp r8, rcx
14001a4fd: jne 0x14001a4c0
14001a4ff: cmp rbx, r8
14001a502: movaps xmm6, xmmword ptr [rbp + 0x7a0]
14001a509: jne 0x14001a253
14001a50f: jmp 0x14001a313 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001a514: lea rdx, [rax + rdi]
14001a518: add rdi, r15
14001a51b: lea rcx, [rbp + 0x558]
14001a522: cmp rcx, rax
14001a525: seta r8b
14001a529: lea rcx, [rbp + 0x550]
14001a530: cmp rcx, rdx
14001a533: setb r9b
14001a537: cmp rdi, rax
14001a53a: seta cl
14001a53d: cmp r15, rdx
14001a540: setb dl
14001a543: test r8b, r9b
14001a546: jne 0x14001a3be
14001a54c: mov r8d, 0
14001a552: and cl, dl
14001a554: jne 0x14001a3c1
14001a55a: mov r8, r14
14001a55d: movabs rcx, 0x3ffffffffffffff8
14001a567: and r8, rcx
14001a56a: movaps xmm0, xmmword ptr [rbp + 0x550]
14001a571: movaps xmm1, xmm0
14001a574: shufps xmm1, xmm0, 0
14001a578: shufps xmm0, xmm0, 0x55
14001a57c: xor ecx, ecx
14001a57e: nop 
14001a580: movups xmm2, xmmword ptr [r15 + rcx*4]
14001a585: movups xmm3, xmmword ptr [r15 + rcx*4 + 0x10]
14001a58b: movaps xmm4, xmm9
14001a58f: subps xmm4, xmm2
14001a592: movaps xmm5, xmm9
14001a596: subps xmm5, xmm3
14001a599: mulps xmm4, xmm1
14001a59c: mulps xmm5, xmm1
14001a59f: mulps xmm2, xmm0
14001a5a2: addps xmm2, xmm4
14001a5a5: mulps xmm3, xmm0
14001a5a8: addps xmm3, xmm5
14001a5ab: movups xmmword ptr [rax + rcx*4], xmm2
14001a5af: movups xmmword ptr [rax + rcx*4 + 0x10], xmm3
14001a5b4: add rcx, 8
14001a5b8: cmp r8, rcx
14001a5bb: jne 0x14001a580
14001a5bd: cmp r14, r8
14001a5c0: jne 0x14001a3c1
14001a5c6: jmp 0x14001c19f ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001a5cb: cmp byte ptr [r14 + 0xe8], 1
14001a5d3: sete al
14001a5d6: mov rcx, qword ptr [rbp + 0x698]
14001a5dd: cmp rcx, qword ptr [rbp + 0x580]
14001a5e4: je 0x14001a21d
14001a5ea: xor ecx, ecx
14001a5ec: movzx r14d, al
14001a5f0: movzx r15d, cl
14001a5f4: mov rax, qword ptr [rbp + 0x938]
14001a5fb: mov rdi, qword ptr [rax]
14001a5fe: mov bl, byte ptr [rbp + 0x807]
14001a604: mov byte ptr [rbp + 0x816], 1
14001a60b: mov byte ptr [rbp + 0x815], 1
14001a612: lea rcx, [rbp + 0x290]
14001a619: call 0x1400284b0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$8<true>
14001a61e: movzx ecx, r15w
14001a622: shl ecx, 8
14001a625: movzx r8d, r14w
14001a629: or r8d, ecx
14001a62c: mov ecx, dword ptr [rbp + 0x500]
14001a632: mov dword ptr [rbp + 0x6c0], ecx
14001a638: movss xmm0, dword ptr [rbp + 0x548]
14001a640: movss dword ptr [rbp + 0x6c4], xmm0
14001a648: mov ecx, dword ptr [rbp + 0x540]
14001a64e: mov dword ptr [rbp + 0x6c8], ecx
14001a654: movss xmm0, dword ptr [rbp + 0x370]
14001a65c: movss dword ptr [rbp + 0x6cc], xmm0
14001a664: mov byte ptr [rbp + 0x816], 1
14001a66b: mov byte ptr [rbp + 0x815], 1
14001a672: and dl, 1
14001a675: mov byte ptr [rsp + 0x68], dl
14001a679: and al, 1
14001a67b: mov byte ptr [rsp + 0x60], al
14001a67f: mov byte ptr [rsp + 0x58], bl
14001a683: mov qword ptr [rsp + 0x80], r12
14001a68b: mov rax, qword ptr [rbp + 0x7f0]
14001a692: mov qword ptr [rsp + 0x78], rax
14001a697: lea rax, [rbp + 0x6c0]
14001a69e: mov qword ptr [rsp + 0x70], rax
14001a6a3: mov byte ptr [rsp + 0x50], sil
14001a6a8: mov eax, dword ptr [rbp + 0x6a8]
14001a6ae: mov byte ptr [rsp + 0x48], al
14001a6b2: mov word ptr [rsp + 0x40], r8w
14001a6b8: mov qword ptr [rsp + 0x38], rdi
14001a6bd: mov eax, dword ptr [rbp + 0x640]
14001a6c3: mov dword ptr [rsp + 0x30], eax
14001a6c7: mov rax, qword ptr [rbp + 0x788]
14001a6ce: mov dword ptr [rsp + 0x28], eax
14001a6d2: lea rax, [rbp + 0x428]
14001a6d9: mov qword ptr [rsp + 0x20], rax
14001a6de: lea rcx, [rbp + 0x140]
14001a6e5: lea rdx, [rbp + 0x568]
14001a6ec: lea r8, [rbp + 0x650]
14001a6f3: lea r9, [rbp + 0x680]
14001a6fa: call 0x1407f98e0 ; system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings
14001a6ff: mov byte ptr [rbp + 0x816], 1
14001a706: mov byte ptr [rbp + 0x815], 1
14001a70d: mov rcx, qword ptr [rbp + 0x268]
14001a714: call 0x140987ca0 ; country_core::render::impl$0::deref
14001a719: mov rdx, qword ptr [rbp + 0x570]
14001a720: mov r8, qword ptr [rbp + 0x578]
14001a727: mov r9, qword ptr [rbp + 0x658]
14001a72e: mov rcx, qword ptr [rbp + 0x660]
14001a735: movups xmm0, xmmword ptr [rbp + 0x688]
14001a73c: mov r10d, dword ptr [rbp + 0x500]
14001a743: mov dword ptr [rbp + 0x6c0], r10d
14001a74a: movss xmm1, dword ptr [rbp + 0x548]
14001a752: movss dword ptr [rbp + 0x6c4], xmm1
14001a75a: mov r10d, dword ptr [rbp + 0x540]
14001a761: mov dword ptr [rbp + 0x6c8], r10d
14001a768: movss xmm1, dword ptr [rbp + 0x370]
14001a770: movss dword ptr [rbp + 0x6cc], xmm1
14001a778: mov byte ptr [rbp + 0x816], 1
14001a77f: mov byte ptr [rbp + 0x815], 1
14001a786: mov qword ptr [rsp + 0x60], r12
14001a78b: mov r10, qword ptr [rbp + 0x7f0]
14001a792: mov qword ptr [rsp + 0x58], r10
14001a797: lea r10, [rbp + 0x6c0]
14001a79e: mov qword ptr [rsp + 0x50], r10
14001a7a3: mov r10, qword ptr [rbp + 0x788]
14001a7aa: mov dword ptr [rsp + 0x48], r10d
14001a7af: mov r10, qword ptr [rbp + 0x940]
14001a7b6: mov qword ptr [rsp + 0x40], r10
14001a7bb: lea r10, [rbp + 0x140]
14001a7c2: mov qword ptr [rsp + 0x38], r10
14001a7c7: movups xmmword ptr [rsp + 0x28], xmm0
14001a7cc: mov qword ptr [rsp + 0x20], rcx
14001a7d1: mov rcx, rax
14001a7d4: call 0x1407dc170 ; system_decorator::decorator_visual::stairs_assemble::gen_stair_floors
14001a7d9: mov rax, qword ptr [rbp + 0x7e0]
14001a7e0: cmp byte ptr [rax + 0xe8], 0
14001a7e7: je 0x14001a939
14001a7ed: xor eax, eax
14001a7ef: mov r10, qword ptr [rbp + 0x7f0]
14001a7f6: movd xmm0, dword ptr [rip + 0x290b0d6]
14001a7fe: test al, al
14001a800: jne 0x14001a80a
14001a802: movd xmm0, dword ptr [rip + 0x290b0ce]
14001a80a: mov rcx, qword ptr [rbp + 0x570]
14001a811: mov rdx, qword ptr [rbp + 0x578]
14001a818: mov r8, qword ptr [rbp + 0x658]
14001a81f: mov r9, qword ptr [rbp + 0x660]
14001a826: movups xmm1, xmmword ptr [rbp + 0x688]
14001a82d: mov rax, qword ptr [rbp + 0x938]
14001a834: mov rax, qword ptr [rax]
14001a837: mov byte ptr [rbp + 0x816], 1
14001a83e: mov byte ptr [rbp + 0x815], 1
14001a845: mov qword ptr [rsp + 0x78], r12
14001a84a: mov qword ptr [rsp + 0x70], r10
14001a84f: mov r10, qword ptr [rbp + 0x240]
14001a856: mov qword ptr [rsp + 0x68], r10
14001a85b: movd dword ptr [rsp + 0x60], xmm0
14001a861: mov r10d, dword ptr [rbp + 0x6a8]
14001a868: mov byte ptr [rsp + 0x58], r10b
14001a86d: mov qword ptr [rsp + 0x48], rax
14001a872: mov rax, qword ptr [rbp + 0x788]
14001a879: mov dword ptr [rsp + 0x40], eax
14001a87d: lea rax, [rbp + 0x140]
14001a884: mov qword ptr [rsp + 0x38], rax
14001a889: lea rax, [rbp + 0x428]
14001a890: mov qword ptr [rsp + 0x30], rax
14001a895: movups xmmword ptr [rsp + 0x20], xmm1
14001a89a: mov byte ptr [rsp + 0x80], 1
14001a8a2: mov word ptr [rsp + 0x50], 0
14001a8a9: call 0x1407da0b0 ; system_decorator::decorator_visual::stairs_assemble::construct_stair_steps
14001a8ae: cmp byte ptr [rbp + 0x6a8], 0
14001a8b5: je 0x14001af32
14001a8bb: movzx ebx, byte ptr [rbp + 0x806]
14001a8c2: cmp bl, 1
14001a8c5: ja 0x14001a95f
14001a8cb: movzx esi, byte ptr [rbp + 0x807]
14001a8d2: mov r10, qword ptr [rbp + 0x7e0]
14001a8d9: mov r8d, dword ptr [r10 + 0x128]
14001a8e0: mov r9d, dword ptr [r10 + 0x12c]
14001a8e7: mov dword ptr [rbp + 0x6c0], r8d
14001a8ee: mov dword ptr [rbp + 0x6c4], r9d
14001a8f5: mov rax, qword ptr [rbp + 0x7b0]
14001a8fc: mov rdi, qword ptr [rax + 0x70]
14001a900: test rdi, rdi
14001a903: je 0x14001aa14
14001a909: cmp rdi, 1
14001a90d: jne 0x14001a9a6
14001a913: mov rax, qword ptr [rbp + 0x7b0]
14001a91a: mov rax, qword ptr [rax + 0x68]
14001a91e: cmp dword ptr [rax + 8], r8d
14001a922: jne 0x14001aa14
14001a928: cmp dword ptr [rax + 0xc], r9d
14001a92c: jne 0x14001aa14
14001a932: xor edx, edx
14001a934: jmp 0x14001aa05 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001a939: cmp byte ptr [rax + 0x108], 0
14001a940: sete al
14001a943: mov r10, qword ptr [rbp + 0x7f0]
14001a94a: movd xmm0, dword ptr [rip + 0x290af82]
14001a952: test al, al
14001a954: je 0x14001a802
14001a95a: jmp 0x14001a80a ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001a95f: mov r14, qword ptr [rbp + 0x7e0]
14001a966: mov r15, qword ptr [rbp + 0x7d0]
14001a96d: jmp 0x14001b19c ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001a972: cmp qword ptr [rbp + 0x430], 0
14001a97a: mov rcx, qword ptr [rbp + 0x698]
14001a981: je 0x14001ac43
14001a987: xor eax, eax
14001a989: cmp rcx, qword ptr [rbp + 0x580]
14001a990: jne 0x14001ac5b
14001a996: cmp byte ptr [r14 + 0x108], 1
14001a99e: sete cl
14001a9a1: jmp 0x14001ac5d ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001a9a6: mov rax, qword ptr [rbp + 0x7b0]
14001a9ad: mov rcx, qword ptr [rax + 0x98]
14001a9b4: mov rdx, qword ptr [rax + 0xa0]
14001a9bb: call 0x140106840 ; indexmap::map::IndexMap::hash<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,country_core::resources::stairs::stairs_assembly_params::AdjustedStairEdge,std::hash::random::RandomState,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >
14001a9c0: mov byte ptr [rbp + 0x816], 1
14001a9c7: mov byte ptr [rbp + 0x815], 1
14001a9ce: mov rcx, qword ptr [rbp + 0x340]
14001a9d5: mov rdx, rax
14001a9d8: lea r8, [rbp + 0x6c0]
14001a9df: call 0x1403ab1c0 ; indexmap::map::core::IndexMapCore::get_index_of<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2$<country_core::resources::stairs::stairs_assembly_params::AlignKind>,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >
14001a9e4: cmp rax, 1
14001a9e8: mov r10, qword ptr [rbp + 0x7e0]
14001a9ef: jne 0x14001aa14
14001a9f1: cmp rdx, rdi
14001a9f4: jae 0x140020f67
14001a9fa: mov rax, qword ptr [rbp + 0x7b0]
14001aa01: mov rax, qword ptr [rax + 0x68]
14001aa05: lea rcx, [rdx + rdx*4]
14001aa09: cmp byte ptr [rax + rcx*8 + 0x10], 0
14001aa0e: je 0x14001aad8
14001aa14: mov r9d, dword ptr [r10 + 0x128]
14001aa1b: mov r8d, dword ptr [r10 + 0x12c]
14001aa22: mov dword ptr [rbp + 0x6c0], r8d
14001aa29: mov dword ptr [rbp + 0x6c4], r9d
14001aa30: mov rax, qword ptr [rbp + 0x7b0]
14001aa37: mov rdi, qword ptr [rax + 0x70]
14001aa3b: test rdi, rdi
14001aa3e: je 0x14001ab5c
14001aa44: cmp rdi, 1
14001aa48: jne 0x14001aa6d
14001aa4a: mov rax, qword ptr [rbp + 0x7b0]
14001aa51: mov rax, qword ptr [rax + 0x68]
14001aa55: cmp dword ptr [rax + 8], r8d
14001aa59: jne 0x14001ab5c
14001aa5f: cmp dword ptr [rax + 0xc], r9d
14001aa63: jne 0x14001ab5c
14001aa69: xor edx, edx
14001aa6b: jmp 0x14001aac9 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001aa6d: mov rax, qword ptr [rbp + 0x7b0]
14001aa74: mov rcx, qword ptr [rax + 0x98]
14001aa7b: mov rdx, qword ptr [rax + 0xa0]
14001aa82: call 0x140106840 ; indexmap::map::IndexMap::hash<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,country_core::resources::stairs::stairs_assembly_params::AdjustedStairEdge,std::hash::random::RandomState,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >
14001aa87: mov byte ptr [rbp + 0x816], 1
14001aa8e: mov byte ptr [rbp + 0x815], 1
14001aa95: mov rcx, qword ptr [rbp + 0x340]
14001aa9c: mov rdx, rax
14001aa9f: lea r8, [rbp + 0x6c0]
14001aaa6: call 0x1403ab1c0 ; indexmap::map::core::IndexMapCore::get_index_of<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2$<country_core::resources::stairs::stairs_assembly_params::AlignKind>,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >
14001aaab: cmp rax, 1
14001aaaf: jne 0x14001ab5c
14001aab5: cmp rdx, rdi
14001aab8: jae 0x140020faa
14001aabe: mov rax, qword ptr [rbp + 0x7b0]
14001aac5: mov rax, qword ptr [rax + 0x68]
14001aac9: lea rcx, [rdx + rdx*4]
14001aacd: cmp byte ptr [rax + rcx*8 + 0x10], 0
14001aad2: jne 0x14001ab5c
14001aad8: lea rax, [rax + rcx*8]
14001aadc: add rax, 0x10
14001aae0: mov rcx, qword ptr [rbp + 0x938]
14001aae7: mov rcx, qword ptr [rcx]
14001aaea: movdqu xmm0, xmmword ptr [rbp + 0x570]
14001aaf2: movups xmm1, xmmword ptr [rbp + 0x658]
14001aaf9: movdqu xmm2, xmmword ptr [rbp + 0x688]
14001ab01: movss xmm3, dword ptr [rax + 4]
14001ab06: movss xmm4, dword ptr [rax + 8]
14001ab0b: mov byte ptr [rbp + 0x816], 1
14001ab12: mov byte ptr [rbp + 0x815], 1
14001ab19: movss dword ptr [rsp + 0x68], xmm4
14001ab1f: movss dword ptr [rsp + 0x60], xmm3
14001ab25: mov byte ptr [rsp + 0x58], sil
14001ab2a: and bl, 1
14001ab2d: mov byte ptr [rsp + 0x50], bl
14001ab31: movdqu xmmword ptr [rsp + 0x40], xmm2
14001ab37: movups xmmword ptr [rsp + 0x30], xmm1
14001ab3c: movdqu xmmword ptr [rsp + 0x20], xmm0
14001ab42: mov rdx, qword ptr [rbp + 0x788]
14001ab49: mov r8d, dword ptr [rbp + 0x6a8]
14001ab50: lea r9, [rbp + 0x140]
14001ab57: call 0x1407febb0 ; system_decorator::decorator_visual::stairs_railings_assemble::add_corner_stone_railing_piece
14001ab5c: xor edx, edx
14001ab5e: mov rax, qword ptr [rbp + 0x698]
14001ab65: cmp rax, qword ptr [rbp + 0x580]
14001ab6c: sete dl
14001ab6f: xor eax, eax
14001ab71: cmp qword ptr [rbp + 0x430], 0
14001ab79: sete al
14001ab7c: mov rcx, qword ptr [rbp + 0x950]
14001ab83: mov rcx, qword ptr [rcx]
14001ab86: movdqu xmm0, xmmword ptr [rbp + 0x570]
14001ab8e: movups xmm1, xmmword ptr [rbp + 0x658]
14001ab95: movdqu xmm2, xmmword ptr [rbp + 0x688]
14001ab9d: mov r8, qword ptr [rbp + 0x368]
14001aba4: mov r8, qword ptr [r8]
14001aba7: mov r9, qword ptr [rbp + 0x958]
14001abae: mov r9, qword ptr [r9]
14001abb1: mov r10, qword ptr [rbp + 0x938]
14001abb8: mov r10, qword ptr [r10]
14001abbb: shl edx, 8
14001abbe: or eax, edx
14001abc0: mov byte ptr [rbp + 0x816], 1
14001abc7: mov byte ptr [rbp + 0x815], 1
14001abce: mov qword ptr [rsp + 0x80], r10
14001abd6: mov qword ptr [rsp + 0x78], r9
14001abdb: mov qword ptr [rsp + 0x70], r8
14001abe0: movdqu xmmword ptr [rsp + 0x60], xmm2
14001abe6: movups xmmword ptr [rsp + 0x50], xmm1
14001abeb: movdqu xmmword ptr [rsp + 0x40], xmm0
14001abf1: lea rdx, [rbp + 0x140]
14001abf8: mov qword ptr [rsp + 0x38], rdx
14001abfd: mov edx, dword ptr [rbp + 0x6a8]
14001ac03: mov byte ptr [rsp + 0x30], dl
14001ac07: mov edx, dword ptr [rbp + 0x640]
14001ac0d: mov dword ptr [rsp + 0x28], edx
14001ac11: mov rdx, qword ptr [rbp + 0x788]
14001ac18: mov dword ptr [rsp + 0x20], edx
14001ac1c: mov word ptr [rsp + 0x88], ax
14001ac24: mov rdx, qword ptr [rbp + 0x7b0]
14001ac2b: mov r8, qword ptr [rbp + 0x670]
14001ac32: lea r9, [rbp + 0x428]
14001ac39: call 0x1407fcc80 ; system_decorator::decorator_visual::stairs_railings_assemble::railings_tips
14001ac3e: jmp 0x14001af32 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001ac43: cmp byte ptr [r14 + 0xe8], 1
14001ac4b: sete al
14001ac4e: cmp rcx, qword ptr [rbp + 0x580]
14001ac55: je 0x14001a996
14001ac5b: xor ecx, ecx
14001ac5d: movzx r14d, al
14001ac61: movzx r15d, cl
14001ac65: mov rax, qword ptr [rbp + 0x938]
14001ac6c: mov rdi, qword ptr [rax]
14001ac6f: mov bl, byte ptr [rbp + 0x807]
14001ac75: mov byte ptr [rbp + 0x816], 1
14001ac7c: mov byte ptr [rbp + 0x815], 1
14001ac83: lea rcx, [rbp + 0x290]
14001ac8a: call 0x1400284b0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$8<true>
14001ac8f: mov dword ptr [rbp + 0x6c0], 0
14001ac99: mov dword ptr [rbp + 0x6c8], 0
14001aca3: movzx ecx, r15w
14001aca7: shl ecx, 8
14001acaa: movzx r8d, r14w
14001acae: or r8d, ecx
14001acb1: mov byte ptr [rbp + 0x816], 1
14001acb8: mov byte ptr [rbp + 0x815], 1
14001acbf: and dl, 1
14001acc2: mov byte ptr [rsp + 0x68], dl
14001acc6: and al, 1
14001acc8: mov byte ptr [rsp + 0x60], al
14001accc: mov byte ptr [rsp + 0x58], bl
14001acd0: mov qword ptr [rsp + 0x80], r12
14001acd8: mov rax, qword ptr [rbp + 0x7f0]
14001acdf: mov qword ptr [rsp + 0x78], rax
14001ace4: lea rax, [rbp + 0x6c0]
14001aceb: mov qword ptr [rsp + 0x70], rax
14001acf0: mov byte ptr [rsp + 0x50], sil
14001acf5: mov eax, dword ptr [rbp + 0x6a8]
14001acfb: mov byte ptr [rsp + 0x48], al
14001acff: mov word ptr [rsp + 0x40], r8w
14001ad05: mov qword ptr [rsp + 0x38], rdi
14001ad0a: mov eax, dword ptr [rbp + 0x640]
14001ad10: mov dword ptr [rsp + 0x30], eax
14001ad14: mov rax, qword ptr [rbp + 0x788]
14001ad1b: mov dword ptr [rsp + 0x28], eax
14001ad1f: lea rax, [rbp + 0x428]
14001ad26: mov qword ptr [rsp + 0x20], rax
14001ad2b: lea rcx, [rbp + 0x140]
14001ad32: lea rdx, [rbp + 0x568]
14001ad39: lea r8, [rbp + 0x650]
14001ad40: lea r9, [rbp + 0x680]
14001ad47: call 0x1407f98e0 ; system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings
14001ad4c: mov r14, qword ptr [rbp + 0x7e0]
14001ad53: cmp byte ptr [r14 + 0xe8], 0
14001ad5b: je 0x14001ad69
14001ad5d: mov dword ptr [rbp + 0x7a0], 0
14001ad67: jmp 0x14001ad7a ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001ad69: cmp byte ptr [r14 + 0x108], 0
14001ad71: sete al
14001ad74: mov dword ptr [rbp + 0x7a0], eax
14001ad7a: mov rax, qword ptr [rbp + 0x570]
14001ad81: mov qword ptr [rbp + 0x808], rax
14001ad88: mov rbx, qword ptr [rbp + 0x578]
14001ad8f: mov r15, qword ptr [rbp + 0x658]
14001ad96: mov rdi, qword ptr [rbp + 0x660]
14001ad9d: mov rsi, qword ptr [rbp + 0x688]
14001ada4: mov r14, qword ptr [rbp + 0x690]
14001adab: mov rax, qword ptr [rbp + 0x938]
14001adb2: mov r13, qword ptr [rax]
14001adb5: mov byte ptr [rbp + 0x816], 1
14001adbc: mov byte ptr [rbp + 0x815], 1
14001adc3: mov rcx, qword ptr [rbp + 0x2d8]
14001adca: call 0x140959ef0 ; country_core::resources::stairs::stairs_assembly_params::StairNodeAnchor::try_as_wall
14001adcf: test rax, rax
14001add2: je 0x14001adff
14001add4: cmp qword ptr [rbp + 0x578], 0
14001addc: je 0x140020db4
14001ade2: mov rax, qword ptr [rbp + 0x570]
14001ade9: movss xmm0, dword ptr [rip + 0x290aadf]
14001adf1: ucomiss xmm0, dword ptr [rax]
14001adf4: setae al
14001adf7: mov dword ptr [rbp + 0x6b0], eax
14001adfd: jmp 0x14001ae09 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001adff: mov dword ptr [rbp + 0x6b0], 0
14001ae09: mov byte ptr [rbp + 0x816], 1
14001ae10: mov byte ptr [rbp + 0x815], 1
14001ae17: mov rcx, qword ptr [rbp + 0x118]
14001ae1e: call 0x140959ef0 ; country_core::resources::stairs::stairs_assembly_params::StairNodeAnchor::try_as_wall
14001ae23: test rax, rax
14001ae26: je 0x14001ae7c
14001ae28: mov rax, qword ptr [rbp + 0x578]
14001ae2f: test rax, rax
14001ae32: je 0x140020dd7
14001ae38: mov rcx, qword ptr [rbp + 0x570]
14001ae3f: lea rax, [rcx + rax*4]
14001ae43: add rax, -4
14001ae47: test rax, rax
14001ae4a: je 0x140020dd7
14001ae50: movd xmm0, dword ptr [rip + 0x290aa7c]
14001ae58: cmp byte ptr [rbp + 0x7a0], 0
14001ae5f: jne 0x14001ae69
14001ae61: movd xmm0, dword ptr [rip + 0x290aa6f]
14001ae69: movss xmm1, dword ptr [rip + 0x290aa5f]
14001ae71: ucomiss xmm1, dword ptr [rax]
14001ae74: jb 0x14001ae95
14001ae76: mov ax, 0x100
14001ae7a: jmp 0x14001ae97 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001ae7c: movd xmm0, dword ptr [rip + 0x290aa50]
14001ae84: cmp byte ptr [rbp + 0x7a0], 0
14001ae8b: jne 0x14001ae95
14001ae8d: movd xmm0, dword ptr [rip + 0x290aa43]
14001ae95: xor eax, eax
14001ae97: movzx ecx, byte ptr [rbp + 0x6b0]
14001ae9e: movzx eax, ax
14001aea1: or eax, ecx
14001aea3: mov byte ptr [rbp + 0x816], 1
14001aeaa: mov byte ptr [rbp + 0x815], 1
14001aeb1: mov word ptr [rsp + 0x50], ax
14001aeb6: mov qword ptr [rsp + 0x48], r13
14001aebb: mov qword ptr [rsp + 0x28], r14
14001aec0: mov qword ptr [rsp + 0x20], rsi
14001aec5: mov qword ptr [rsp + 0x78], r12
14001aeca: mov rax, qword ptr [rbp + 0x7f0]
14001aed1: mov qword ptr [rsp + 0x70], rax
14001aed6: mov rax, qword ptr [rbp + 0x240]
14001aedd: mov qword ptr [rsp + 0x68], rax
14001aee2: movd dword ptr [rsp + 0x60], xmm0
14001aee8: mov eax, dword ptr [rbp + 0x6a8]
14001aeee: mov byte ptr [rsp + 0x58], al
14001aef2: mov rax, qword ptr [rbp + 0x788]
14001aef9: mov dword ptr [rsp + 0x40], eax
14001aefd: lea rax, [rbp + 0x140]
14001af04: mov qword ptr [rsp + 0x38], rax
14001af09: lea rax, [rbp + 0x428]
14001af10: mov qword ptr [rsp + 0x30], rax
14001af15: mov byte ptr [rsp + 0x80], 0
14001af1d: mov r8, r15
14001af20: mov r9, rdi
14001af23: mov rcx, qword ptr [rbp + 0x808]
14001af2a: mov rdx, rbx
14001af2d: call 0x1407da0b0 ; system_decorator::decorator_visual::stairs_assemble::construct_stair_steps
14001af32: movzx ebx, byte ptr [rbp + 0x806]
14001af39: cmp bl, 2
14001af3c: mov r14, qword ptr [rbp + 0x7e0]
14001af43: mov r15, qword ptr [rbp + 0x7d0]
14001af4a: jae 0x14001b19c
14001af50: mov r8d, dword ptr [r14 + 0x128]
14001af57: mov r9d, dword ptr [r14 + 0x12c]
14001af5e: mov dword ptr [rbp + 0x6c0], r8d
14001af65: mov dword ptr [rbp + 0x6c4], r9d
14001af6c: mov rax, qword ptr [rbp + 0x7b0]
14001af73: mov rdi, qword ptr [rax + 0x70]
14001af77: test rdi, rdi
14001af7a: je 0x14001b195
14001af80: cmp rdi, 1
14001af84: jne 0x14001afa9
14001af86: mov rax, qword ptr [rbp + 0x7b0]
14001af8d: mov rax, qword ptr [rax + 0x68]
14001af91: cmp dword ptr [rax + 8], r8d
14001af95: jne 0x14001b195
14001af9b: cmp dword ptr [rax + 0xc], r9d
14001af9f: jne 0x14001b195
14001afa5: xor edx, edx
14001afa7: jmp 0x14001b013 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001afa9: mov rax, qword ptr [rbp + 0x7b0]
14001afb0: mov rcx, qword ptr [rax + 0x98]
14001afb7: mov rdx, qword ptr [rax + 0xa0]
14001afbe: call 0x140106840 ; indexmap::map::IndexMap::hash<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,country_core::resources::stairs::stairs_assembly_params::AdjustedStairEdge,std::hash::random::RandomState,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >
14001afc3: mov byte ptr [rbp + 0x816], 1
14001afca: mov byte ptr [rbp + 0x815], 1
14001afd1: mov rcx, qword ptr [rbp + 0x340]
14001afd8: mov rdx, rax
14001afdb: lea r8, [rbp + 0x6c0]
14001afe2: call 0x1403ab1c0 ; indexmap::map::core::IndexMapCore::get_index_of<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2$<country_core::resources::stairs::stairs_assembly_params::AlignKind>,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >
14001afe7: cmp rax, 1
14001afeb: mov r14, qword ptr [rbp + 0x7e0]
14001aff2: mov r15, qword ptr [rbp + 0x7d0]
14001aff9: jne 0x14001b195
14001afff: cmp rdx, rdi
14001b002: jae 0x140020eb7
14001b008: mov rax, qword ptr [rbp + 0x7b0]
14001b00f: mov rax, qword ptr [rax + 0x68]
14001b013: lea rcx, [rdx + rdx*4]
14001b017: cmp byte ptr [rax + rcx*8 + 0x10], 0
14001b01c: jne 0x14001b195
14001b022: movzx edx, byte ptr [rax + rcx*8 + 0x12]
14001b027: or dl, byte ptr [rax + rcx*8 + 0x11]
14001b02b: test dl, 1
14001b02e: je 0x14001b195
14001b034: mov ecx, dword ptr [r14 + 0x128]
14001b03b: mov edx, dword ptr [r14 + 0x12c]
14001b042: mov byte ptr [rbp + 0x816], 1
14001b049: mov byte ptr [rbp + 0x815], 1
14001b050: call 0x140959e70 ; country_core::resources::stairs::stairs_assembly_params::impl$2::from
14001b055: mov rdi, rax
14001b058: mov qword ptr [rbp + 0x6c0], rax
14001b05f: mov rax, qword ptr [rbp + 0x7b0]
14001b066: cmp qword ptr [rax + 0x1b0], 0
14001b06e: mov r15, qword ptr [rbp + 0x7d0]
14001b075: je 0x14001b18e
14001b07b: mov rcx, qword ptr [rbp + 0xc0]
14001b082: lea rdx, [rbp + 0x6c0]
14001b089: call 0x1405b0d70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId> >
14001b08e: mov rdx, qword ptr [rbp + 0x7b0]
14001b095: mov rcx, qword ptr [rdx + 0x198]
14001b09c: mov rdx, qword ptr [rdx + 0x1a0]
14001b0a3: mov r8, rdx
14001b0a6: and r8, rax
14001b0a9: shr rax, 0x39
14001b0ad: movd xmm0, eax
14001b0b1: punpcklbw xmm0, xmm0
14001b0b5: pshuflw xmm0, xmm0, 0
14001b0ba: pshufd xmm0, xmm0, 0
14001b0bf: lea rax, [rcx - 0x20]
14001b0c3: xor r9d, r9d
14001b0c6: movdqu xmm1, xmmword ptr [rcx + r8]
14001b0cc: movdqa xmm2, xmm1
14001b0d0: pcmpeqb xmm2, xmm0
14001b0d4: pmovmskb r10d, xmm2
14001b0d9: test r10d, r10d
14001b0dc: je 0x14001b108
14001b0de: tzcnt r11d, r10d
14001b0e3: add r11, r8
14001b0e6: and r11, rdx
14001b0e9: mov rsi, r11
14001b0ec: shl rsi, 5
14001b0f0: mov rbx, rax
14001b0f3: sub rbx, rsi
14001b0f6: cmp rdi, qword ptr [rbx]
14001b0f9: je 0x14001b12a
14001b0fb: lea r11d, [r10 - 1]
14001b0ff: and r11w, r10w
14001b103: mov r10d, r11d
14001b106: jne 0x14001b0de
14001b108: pcmpeqb xmm1, xmmword ptr [rip + 0x290a880]
14001b110: pmovmskb r10d, xmm1
14001b115: test r10d, r10d
14001b118: jne 0x14001b18e
14001b11a: add r8, r9
14001b11d: add r8, 0x10
14001b121: add r9, 0x10
14001b125: and r8, rdx
14001b128: jmp 0x14001b0c6 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001b12a: neg r11
14001b12d: shl r11, 5
14001b131: mov rax, qword ptr [rcx + r11 - 8]
14001b136: cmp rax, 2
14001b13a: jb 0x14001b148
14001b13c: mov rdx, qword ptr [rcx + r11 - 0x18]
14001b141: mov rax, qword ptr [rcx + r11 - 0x10]
14001b146: jmp 0x14001b150 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001b148: lea rdx, [rcx + r11]
14001b14c: add rdx, -0x18
14001b150: shl rax, 2
14001b154: xor ecx, ecx
14001b156: nop word ptr cs:[rax + rax]
14001b160: cmp rax, rcx
14001b163: je 0x14001b18e
14001b165: cmp byte ptr [rdx + rcx], 3
14001b169: lea rcx, [rcx + 4]
14001b16d: jne 0x14001b160
14001b16f: mov sil, 1
14001b172: mov r14, qword ptr [rbp + 0x7e0]
14001b179: test r15, r15
14001b17c: lea r12, [rbp + 0x590]
14001b183: jne 0x14001b6fa
14001b189: jmp 0x14001b713 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001b18e: mov r14, qword ptr [rbp + 0x7e0]
14001b195: movzx ebx, byte ptr [rbp + 0x806]
14001b19c: cmp bl, 4
14001b19f: jne 0x14001b674
14001b1a5: mov rcx, qword ptr [rbp + 0x578]
14001b1ac: test rcx, rcx
14001b1af: je 0x140020bbe
14001b1b5: mov rax, qword ptr [rbp + 0x570]
14001b1bc: lea rcx, [rax + rcx*4]
14001b1c0: add rcx, -4
14001b1c4: je 0x140020be1
14001b1ca: movss xmm0, dword ptr [rax]
14001b1ce: movss xmm1, dword ptr [rcx]
14001b1d2: movaps xmm2, xmm0
14001b1d5: cmpunordss xmm2, xmm0
14001b1da: movaps xmm3, xmm2
14001b1dd: andps xmm3, xmm1
14001b1e0: maxss xmm1, xmm0
14001b1e4: andnps xmm2, xmm1
14001b1e7: orps xmm2, xmm3
14001b1ea: ucomiss xmm2, dword ptr [rip + 0x290a6df]
14001b1f1: setbe al
14001b1f4: or al, byte ptr [rbp + 0x530]
14001b1fa: mov sil, 1
14001b1fd: jne 0x14001b6ee
14001b203: mov qword ptr [rbp + 0x460], 0
14001b20e: mov qword ptr [rbp + 0x468], 1
14001b219: movq xmm0, qword ptr [rbp + 0x690]
14001b221: pshufd xmm0, xmm0, 0x44
14001b226: paddq xmm0, xmmword ptr [rip + 0x290a6e2]
14001b22e: movdqu xmmword ptr [rbp + 0x470], xmm0
14001b236: xor r14d, r14d
14001b239: xor esi, esi
14001b23b: nop dword ptr [rax + rax]
14001b240: cmp r14, 0x20
14001b244: je 0x14001b795
14001b24a: mov r8, qword ptr [rbp + r14 + 0x468]
14001b252: cmp r8, -1
14001b256: je 0x14002054e
14001b25c: mov rcx, qword ptr [rbp + r14 + 0x460]
14001b264: lea r9, [r8 + 1]
14001b268: mov rdi, r9
14001b26b: mov qword ptr [rbp + 0x808], rcx
14001b272: sub rdi, rcx
14001b275: jb 0x14002056d
14001b27b: mov rdx, qword ptr [rbp + 0x690]
14001b282: cmp r8, rdx
14001b285: jae 0x140020596
14001b28b: mov qword ptr [rbp + 0x7a0], r8
14001b292: mov qword ptr [rbp + 0x6b0], r9
14001b299: mov rax, rdi
14001b29c: mov ecx, 0xc
14001b2a1: mul rcx
14001b2a4: jo 0x140020ac8
14001b2aa: mov r13, rsi
14001b2ad: mov esi, 0
14001b2b2: movabs rcx, 0x7ffffffffffffffc
14001b2bc: cmp rax, rcx
14001b2bf: mov rbx, qword ptr [rbp + 0x100]
14001b2c6: ja 0x140020ad1
14001b2cc: mov r15, qword ptr [rbp + 0x688]
14001b2d3: test rax, rax
14001b2d6: je 0x14001b310
14001b2d8: movzx ecx, byte ptr [rip + 0x361992a]
14001b2df: mov esi, 4
14001b2e4: mov edx, 4
14001b2e9: mov rcx, rax
14001b2ec: mov rbx, rax
14001b2ef: call 0x140613c10 ; ?
14001b2f4: test rax, rax
14001b2f7: je 0x140020ad1
14001b2fd: mov rsi, rdi
14001b300: mov rbx, rax
14001b303: jmp 0x14001b317 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001b305: nop word ptr cs:[rax + rax]
14001b310: mov ebx, 4
14001b315: xor esi, esi
14001b317: mov rax, qword ptr [rbp + 0x808]
14001b31e: lea rax, [rax + rax*2]
14001b322: lea rdx, [r15 + rax*4]
14001b326: lea r15, [rdi*4]
14001b32e: lea r8, [r15 + r15*2]
14001b332: mov rcx, rbx
14001b335: call 0x142923c00 ; ?
14001b33a: mov qword ptr [rbp + 0x6c0], rsi
14001b341: mov qword ptr [rbp + 0x100], rbx
14001b348: mov qword ptr [rbp + 0x6c8], rbx
14001b34f: mov qword ptr [rbp + 0x6d0], rdi
14001b356: mov byte ptr [rbp + 0x816], 1
14001b35d: mov byte ptr [rbp + 0x815], 1
14001b364: lea rcx, [rbp + 0x18]
14001b368: lea rdx, [rbp + 0x6c0]
14001b36f: mov r8d, 1
14001b375: call 0x140153640 ; utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14001b37a: add r14, 0x10
14001b37e: lea rsi, [r13 + 1]
14001b382: xor eax, eax
14001b384: cmp rax, qword ptr [rbp + 0x18]
14001b388: jo 0x14001b240
14001b38e: mov rax, qword ptr [rbp + 0x330]
14001b395: mov ecx, dword ptr [rbp + 0x534]
14001b39b: mov dword ptr [rax], ecx
14001b39d: mov rax, qword ptr [rbp + 0x48]
14001b3a1: mov qword ptr [rbp + 0x5c0], rax
14001b3a8: movups xmm0, xmmword ptr [rbp + 0x18]
14001b3ac: movups xmm1, xmmword ptr [rbp + 0x28]
14001b3b0: movdqu xmm2, xmmword ptr [rbp + 0x38]
14001b3b5: movdqa xmmword ptr [rbp + 0x5b0], xmm2
14001b3bd: movaps xmmword ptr [rbp + 0x5a0], xmm1
14001b3c4: movaps xmmword ptr [rbp + 0x590], xmm0
14001b3cb: mov rdx, qword ptr [rbp + 0x578]
14001b3d2: cmp qword ptr [rbp + 0x7a0], rdx
14001b3d9: jae 0x140020796
14001b3df: mov rbx, r13
14001b3e2: mov qword ptr [rbp + 0x558], r14
14001b3e9: mov rax, qword ptr [rbp + 0x808]
14001b3f0: lea r14, [rax*4]
14001b3f8: add r14, qword ptr [rbp + 0x570]
14001b3ff: test rdi, rdi
14001b402: je 0x14001b42a
14001b404: movzx eax, byte ptr [rip + 0x36197fe]
14001b40b: mov edx, 4
14001b410: mov rcx, r15
14001b413: call 0x140613c10 ; ?
14001b418: test rax, rax
14001b41b: je 0x140020e9e
14001b421: mov qword ptr [rbp + 0x7d8], rdi
14001b428: jmp 0x14001b43a ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001b42a: mov eax, 4
14001b42f: mov qword ptr [rbp + 0x7d8], 0
14001b43a: mov qword ptr [rbp + 0x138], rax
14001b441: mov rcx, rax
14001b444: mov rdx, r14
14001b447: mov r8, r15
14001b44a: call 0x142923c00 ; ?
14001b44f: mov rdx, qword ptr [rbp + 0x660]
14001b456: cmp qword ptr [rbp + 0x7a0], rdx
14001b45d: jae 0x1400207ae
14001b463: mov rax, rdi
14001b466: shr rax, 0x3c
14001b46a: jne 0x140020cff
14001b470: mov rax, qword ptr [rbp + 0x808]
14001b477: lea r13, [rax*8]
14001b47f: add r13, qword ptr [rbp + 0x658]
14001b486: lea r14, [rdi*8]
14001b48e: test rdi, rdi
14001b491: je 0x14001b4b9
14001b493: movzx eax, byte ptr [rip + 0x361976f]
14001b49a: mov edx, 4
14001b49f: mov rcx, r14
14001b4a2: call 0x140613c10 ; ?
14001b4a7: test rax, rax
14001b4aa: je 0x140020e61
14001b4b0: mov qword ptr [rbp + 0x790], rdi
14001b4b7: jmp 0x14001b4c9 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001b4b9: mov eax, 4
14001b4be: mov qword ptr [rbp + 0x790], 0
14001b4c9: mov qword ptr [rbp + 0x2f8], rax
14001b4d0: mov rcx, rax
14001b4d3: mov rdx, r13
14001b4d6: mov r8, r14
14001b4d9: call 0x142923c00 ; ?
14001b4de: cmp qword ptr [rbp + 0x7a0], r12
14001b4e5: jae 0x1400207c6
14001b4eb: test rdi, rdi
14001b4ee: je 0x14001b515
14001b4f0: movzx eax, byte ptr [rip + 0x3619712]
14001b4f7: mov edx, 4
14001b4fc: mov rcx, r15
14001b4ff: call 0x140613c10 ; ?
14001b504: test rax, rax
14001b507: je 0x140020e85
14001b50d: mov r13, rax
14001b510: mov r14, rdi
14001b513: jmp 0x14001b51e ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001b515: mov r13d, 4
14001b51b: xor r14d, r14d
14001b51e: mov rax, qword ptr [rbp + 0x7f0]
14001b525: mov rcx, qword ptr [rbp + 0x808]
14001b52c: lea rdx, [rax + rcx*4]
14001b530: mov rcx, r13
14001b533: mov r8, r15
14001b536: call 0x142923c00 ; ?
14001b53b: cmp rbx, 1
14001b53f: mov al, 2
14001b541: sbb al, 0
14001b543: movaps xmm0, xmmword ptr [rbp + 0x590]
14001b54a: movaps xmm1, xmmword ptr [rbp + 0x5a0]
14001b551: movdqa xmm2, xmmword ptr [rbp + 0x5b0]
14001b559: movaps xmmword ptr [rbp + 0x6c0], xmm0
14001b560: movaps xmmword ptr [rbp + 0x6d0], xmm1
14001b567: movdqa xmmword ptr [rbp + 0x6e0], xmm2
14001b56f: mov rcx, qword ptr [rbp + 0x5c0]
14001b576: mov qword ptr [rbp + 0x6f0], rcx
14001b57d: mov rcx, qword ptr [rbp + 0x790]
14001b584: mov qword ptr [rbp + 0x6f8], rcx
14001b58b: mov rcx, qword ptr [rbp + 0x2f8]
14001b592: mov qword ptr [rbp + 0x700], rcx
14001b599: mov qword ptr [rbp + 0x708], rdi
14001b5a0: mov rcx, qword ptr [rbp + 0x7d8]
14001b5a7: mov qword ptr [rbp + 0x710], rcx
14001b5ae: mov rcx, qword ptr [rbp + 0x138]
14001b5b5: mov qword ptr [rbp + 0x718], rcx
14001b5bc: mov qword ptr [rbp + 0x720], rdi
14001b5c3: mov qword ptr [rbp + 0x728], r14
14001b5ca: mov qword ptr [rbp + 0x730], r13
14001b5d1: mov qword ptr [rbp + 0x738], rdi
14001b5d8: mov rcx, qword ptr [rbp + 0x438]
14001b5df: mov qword ptr [rbp + 0x740], rcx
14001b5e6: mov word ptr [rbp + 0x754], 0
14001b5ef: mov rcx, qword ptr [rbp + 0x788]
14001b5f6: mov dword ptr [rbp + 0x750], ecx
14001b5fc: mov rcx, qword ptr [rbp + 0x670]
14001b603: mov rcx, qword ptr [rcx]
14001b606: mov qword ptr [rbp + 0x748], rcx
14001b60d: mov byte ptr [rbp + 0x75b], al
14001b613: mov dword ptr [rbp + 0x756], 0
14001b61d: mov byte ptr [rbp + 0x75a], 0
14001b624: mov rbx, qword ptr [rbp + 0x360]
14001b62b: mov rdi, qword ptr [rbx + 0x10]
14001b62f: cmp rdi, qword ptr [rbx]
14001b632: jne 0x14001b643
14001b634: mov rcx, rbx
14001b637: lea rdx, [rip + 0x290b46a]
14001b63e: call 0x140563480 ; alloc::raw_vec::RawVec::grow_one<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams,alloc::alloc::Global>
14001b643: lea rcx, [rdi + rdi*4]
14001b647: shl rcx, 5
14001b64b: add rcx, qword ptr [rbx + 8]
14001b64f: mov r8d, 0xa0
14001b655: lea rdx, [rbp + 0x6c0]
14001b65c: call 0x142923c10 ; ?
14001b661: inc rdi
14001b664: mov qword ptr [rbx + 0x10], rdi
14001b668: mov r14, qword ptr [rbp + 0x558]
14001b66f: jmp 0x14001b240 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001b674: cmp byte ptr [r14 + 0xe8], 0
14001b67c: lea rcx, [rbp + 0x460]
14001b683: jne 0x14001b693
14001b685: cmp byte ptr [r14 + 0x108], 0
14001b68d: je 0x14001ba77
14001b693: mov rax, qword ptr [rbp + 0x690]
14001b69a: mov qword ptr [rbp + 0x6d0], rax
14001b6a1: movaps xmm0, xmmword ptr [rbp + 0x680]
14001b6a8: movaps xmmword ptr [rbp + 0x6c0], xmm0
14001b6af: mov byte ptr [rbp + 0x816], 0
14001b6b6: mov byte ptr [rbp + 0x815], 1
14001b6bd: lea rdx, [rbp + 0x6c0]
14001b6c4: mov r8d, 1
14001b6ca: call 0x140153640 ; utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14001b6cf: xor eax, eax
14001b6d1: cmp rax, qword ptr [rbp + 0x460]
14001b6d8: jno 0x14001b7bb
14001b6de: xor esi, esi
14001b6e0: mov r14, qword ptr [rbp + 0x7e0]
14001b6e7: mov r15, qword ptr [rbp + 0x7d0]
14001b6ee: test r15, r15
14001b6f1: lea r12, [rbp + 0x590]
14001b6f8: je 0x14001b713
14001b6fa: shl r15, 2
14001b6fe: mov r8d, 4
14001b704: mov rcx, qword ptr [rbp + 0x7f0]
14001b70b: mov rdx, r15
14001b70e: call 0x140613c20 ; ?
14001b713: test sil, sil
14001b716: mov rdi, qword ptr [rbp + 0x698]
14001b71d: je 0x14001b745
14001b71f: mov rax, qword ptr [rbp + 0x680]
14001b726: test rax, rax
14001b729: je 0x14001b745
14001b72b: mov rcx, qword ptr [rbp + 0x688]
14001b732: shl rax, 2
14001b736: lea rdx, [rax + rax*2]
14001b73a: mov r8d, 4
14001b740: call 0x140613c20 ; ?
14001b745: mov rdx, qword ptr [rbp + 0x650]
14001b74c: test rdx, rdx
14001b74f: je 0x14001b767
14001b751: mov rcx, qword ptr [rbp + 0x658]
14001b758: shl rdx, 3
14001b75c: mov r8d, 4
14001b762: call 0x140613c20 ; ?
14001b767: mov rdx, qword ptr [rbp + 0x568]
14001b76e: test rdx, rdx
14001b771: je 0x14001b789
14001b773: mov rcx, qword ptr [rbp + 0x570]
14001b77a: shl rdx, 2
14001b77e: mov r8d, 4
14001b784: call 0x140613c20 ; ?
14001b789: mov r8, qword ptr [rbp + 0x560]
14001b790: jmp 0x140018efc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001b795: mov r14, qword ptr [rbp + 0x7e0]
14001b79c: mov r15, qword ptr [rbp + 0x7d0]
14001b7a3: mov sil, 1
14001b7a6: test r15, r15
14001b7a9: lea r12, [rbp + 0x590]
14001b7b0: jne 0x14001b6fa
14001b7b6: jmp 0x14001b713 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001b7bb: mov rax, qword ptr [rbp + 0x490]
14001b7c2: mov qword ptr [rbp + 0x5c0], rax
14001b7c9: movups xmm0, xmmword ptr [rbp + 0x460]
14001b7d0: movups xmm1, xmmword ptr [rbp + 0x470]
14001b7d7: movdqu xmm2, xmmword ptr [rbp + 0x480]
14001b7df: movdqa xmmword ptr [rbp + 0x5b0], xmm2
14001b7e7: movaps xmmword ptr [rbp + 0x5a0], xmm1
14001b7ee: movaps xmmword ptr [rbp + 0x590], xmm0
14001b7f5: mov rax, qword ptr [rbp + 0x568]
14001b7fc: mov qword ptr [rbp + 0x808], rax
14001b803: mov rcx, qword ptr [rbp + 0x570]
14001b80a: mov rsi, qword ptr [rbp + 0x578]
14001b811: mov rax, qword ptr [rbp + 0x650]
14001b818: mov qword ptr [rbp + 0x7a0], rax
14001b81f: mov rdx, qword ptr [rbp + 0x658]
14001b826: mov r14, qword ptr [rbp + 0x660]
14001b82d: mov bl, 1
14001b82f: mov rax, qword ptr [rbp + 0x7e0]
14001b836: cmp byte ptr [rax + 0xe8], 1
14001b83d: mov dil, 1
14001b840: mov qword ptr [rbp + 0x6b0], rcx
14001b847: mov qword ptr [rbp + 0x7d8], rdx
14001b84e: je 0x14001b888
14001b850: mov r8d, dword ptr [rax + 0x128]
14001b857: mov r9d, dword ptr [rax + 0x12c]
14001b85e: mov rcx, qword ptr [rbp + 0x950]
14001b865: mov rdx, qword ptr [rbp + 0x2d0]
14001b86c: call 0x1400275b0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$9<true>
14001b871: mov edi, eax
14001b873: mov rax, qword ptr [rbp + 0x7e0]
14001b87a: mov rcx, qword ptr [rbp + 0x6b0]
14001b881: mov rdx, qword ptr [rbp + 0x7d8]
14001b888: cmp byte ptr [rax + 0x108], 1
14001b88f: je 0x14001b8c9
14001b891: mov rax, qword ptr [rbp + 0x7e0]
14001b898: mov r9d, dword ptr [rax + 0x128]
14001b89f: mov r8d, dword ptr [rax + 0x12c]
14001b8a6: mov rcx, qword ptr [rbp + 0x950]
14001b8ad: mov rdx, qword ptr [rbp + 0x2d0]
14001b8b4: call 0x1400275b0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$9<true>
14001b8b9: mov ebx, eax
14001b8bb: mov rcx, qword ptr [rbp + 0x6b0]
14001b8c2: mov rdx, qword ptr [rbp + 0x7d8]
14001b8c9: mov rax, qword ptr [rbp + 0x5c0]
14001b8d0: mov qword ptr [rbp + 0x6f0], rax
14001b8d7: movaps xmm0, xmmword ptr [rbp + 0x590]
14001b8de: movaps xmm1, xmmword ptr [rbp + 0x5a0]
14001b8e5: movdqa xmm2, xmmword ptr [rbp + 0x5b0]
14001b8ed: movdqa xmmword ptr [rbp + 0x6e0], xmm2
14001b8f5: movaps xmmword ptr [rbp + 0x6d0], xmm1
14001b8fc: movaps xmmword ptr [rbp + 0x6c0], xmm0
14001b903: mov rax, qword ptr [rbp + 0x7a0]
14001b90a: mov qword ptr [rbp + 0x6f8], rax
14001b911: mov qword ptr [rbp + 0x700], rdx
14001b918: mov qword ptr [rbp + 0x708], r14
14001b91f: mov rax, qword ptr [rbp + 0x808]
14001b926: mov qword ptr [rbp + 0x710], rax
14001b92d: mov qword ptr [rbp + 0x718], rcx
14001b934: mov qword ptr [rbp + 0x720], rsi
14001b93b: mov rax, qword ptr [rbp + 0x7d0]
14001b942: mov qword ptr [rbp + 0x728], rax
14001b949: mov rax, qword ptr [rbp + 0x7f0]
14001b950: mov qword ptr [rbp + 0x730], rax
14001b957: mov qword ptr [rbp + 0x738], r12
14001b95e: mov rax, qword ptr [rbp + 0x438]
14001b965: mov qword ptr [rbp + 0x740], rax
14001b96c: mov rax, qword ptr [rbp + 0x670]
14001b973: mov rax, qword ptr [rax]
14001b976: mov qword ptr [rbp + 0x748], rax
14001b97d: mov rax, qword ptr [rbp + 0x788]
14001b984: mov dword ptr [rbp + 0x750], eax
14001b98a: mov byte ptr [rbp + 0x754], dil
14001b991: mov byte ptr [rbp + 0x755], bl
14001b997: mov word ptr [rbp + 0x756], 0
14001b9a0: movzx eax, byte ptr [rbp + 0x678]
14001b9a7: mov byte ptr [rbp + 0x758], al
14001b9ad: movzx eax, byte ptr [rbp + 0x588]
14001b9b4: mov byte ptr [rbp + 0x759], al
14001b9ba: mov word ptr [rbp + 0x75a], 0x300
14001b9c3: mov rax, qword ptr [rbp + 0x430]
14001b9ca: mov qword ptr [rbp + 0x760], rax
14001b9d1: mov rdi, qword ptr [rbp + 0x698]
14001b9d8: mov qword ptr [rbp + 0x768], rdi
14001b9df: mov byte ptr [rbp + 0x770], 0
14001b9e6: mov rax, qword ptr [rbp + 0x440]
14001b9ed: mov rsi, qword ptr [rbp + 0x458]
14001b9f4: cmp rsi, rax
14001b9f7: lea r12, [rbp + 0x590]
14001b9fe: mov r14, qword ptr [rbp + 0x7e0]
14001ba05: jne 0x14001ba28
14001ba07: lea rcx, [rbp + 0x440]
14001ba0e: lea rdx, [rip + 0x290afeb]
14001ba15: call 0x1405114a0 ; alloc::collections::vec_deque::VecDeque::grow<system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::ArchSegment,alloc::alloc::Global>
14001ba1a: mov rax, qword ptr [rbp + 0x440]
14001ba21: mov rsi, qword ptr [rbp + 0x458]
14001ba28: mov rcx, qword ptr [rbp + 0x450]
14001ba2f: add rcx, rsi
14001ba32: cmp rcx, rax
14001ba35: mov edx, 0
14001ba3a: cmovae rdx, rax
14001ba3e: sub rcx, rdx
14001ba41: imul rcx, rcx, 0xb8
14001ba48: add rcx, qword ptr [rbp + 0x448]
14001ba4f: mov r8d, 0xb8
14001ba55: lea rdx, [rbp + 0x6c0]
14001ba5c: call 0x142923c10 ; ?
14001ba61: inc rsi
14001ba64: mov qword ptr [rbp + 0x458], rsi
14001ba6b: mov r8, qword ptr [rbp + 0x560]
14001ba72: jmp 0x140018efc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001ba77: mov rsi, qword ptr [rbp + 0x688]
14001ba7e: mov rax, qword ptr [rbp + 0x690]
14001ba85: test rax, rax
14001ba88: sete cl
14001ba8b: lea rax, [rax + rax*2]
14001ba8f: lea rdi, [rsi + rax*4]
14001ba93: cmp rdi, 0xc
14001ba97: sete al
14001ba9a: or al, cl
14001ba9c: jne 0x14001bf40
14001baa2: mov rdx, qword ptr [rbp + 0x660]
14001baa9: cmp rdx, 1
14001baad: jbe 0x140020efe
14001bab3: movss xmm6, dword ptr [rsi + 8]
14001bab8: movss xmm0, dword ptr [rip + 0x2909db0]
14001bac0: mulss xmm6, xmm0
14001bac4: movsd xmm8, qword ptr [rsi]
14001bac9: movsd xmm7, qword ptr [rdi - 0xc]
14001bace: movss xmm9, dword ptr [rdi - 4]
14001bad4: mulss xmm9, xmm0
14001bad9: mov rax, qword ptr [rbp + 0x658]
14001bae0: movss xmm0, dword ptr [rax + 8]
14001bae5: movss xmm1, dword ptr [rax + 0xc]
14001baea: xorps xmm1, xmmword ptr [rip + 0x2909dff]
14001baf1: movss dword ptr [rbp + 0x590], xmm1
14001baf9: movss dword ptr [rbp + 0x594], xmm0
14001bb01: mov byte ptr [rbp + 0x816], 1
14001bb08: mov byte ptr [rbp + 0x815], 1
14001bb0f: lea rcx, [rbp + 0x6c0]
14001bb16: lea rdx, [rbp + 0x590]
14001bb1d: call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
14001bb22: mov rdx, qword ptr [rbp + 0x578]
14001bb29: cmp rdx, 1
14001bb2d: jbe 0x140020f22
14001bb33: mov rax, qword ptr [rbp + 0x570]
14001bb3a: movss xmm12, dword ptr [rax + 4]
14001bb40: mulss xmm12, dword ptr [rip + 0x2909d27]
14001bb49: addss xmm12, dword ptr [rip + 0x2909dae]
14001bb52: mov rdx, qword ptr [rbp + 0x660]
14001bb59: cmp rdx, 1
14001bb5d: jbe 0x140020f46
14001bb63: movsd xmm10, qword ptr [rbp + 0x6c0]
14001bb6c: movss xmm11, dword ptr [rbp + 0x6c8]
14001bb75: mulss xmm11, xmm12
14001bb7a: mov rax, qword ptr [rbp + 0x658]
14001bb81: movss xmm0, dword ptr [rax + 8]
14001bb86: movss xmm1, dword ptr [rax + 0xc]
14001bb8b: xorps xmm1, xmmword ptr [rip + 0x2909d5e]
14001bb92: movss dword ptr [rbp + 0x590], xmm1
14001bb9a: movss dword ptr [rbp + 0x594], xmm0
14001bba2: mov byte ptr [rbp + 0x816], 1
14001bba9: mov byte ptr [rbp + 0x815], 1
14001bbb0: lea rcx, [rbp + 0x6c0]
14001bbb7: lea rdx, [rbp + 0x590]
14001bbbe: call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
14001bbc3: mov rdx, qword ptr [rbp + 0x578]
14001bbca: cmp rdx, 1
14001bbce: jbe 0x140020f89
14001bbd4: mov rax, qword ptr [rbp + 0x570]
14001bbdb: movss xmm14, dword ptr [rax + 4]
14001bbe1: movsd xmm13, qword ptr [rbp + 0x6c0]
14001bbea: movss xmm15, dword ptr [rbp + 0x6c8]
14001bbf3: movzx eax, byte ptr [rip + 0x361900f]
14001bbfa: mov ecx, 0x24
14001bbff: mov edx, 4
14001bc04: call 0x140613c10 ; ?
14001bc09: test rax, rax
14001bc0c: je 0x140020edc
14001bc12: movaps xmm0, xmmword ptr [rip + 0x2909cc7]
14001bc19: mulps xmm8, xmm0
14001bc1d: mulps xmm7, xmm0
14001bc20: addps xmm7, xmm8
14001bc24: addss xmm6, xmm9
14001bc29: movsldup xmm0, xmm12
14001bc2e: mulps xmm10, xmm0
14001bc32: addps xmm10, xmm7
14001bc36: addss xmm11, xmm6
14001bc3b: mulss xmm14, dword ptr [rip + 0x2909c2c]
14001bc44: addss xmm14, dword ptr [rip + 0x2909cb3]
14001bc4d: mulss xmm15, xmm14
14001bc52: movlps qword ptr [rax], xmm10
14001bc56: movss dword ptr [rax + 8], xmm11
14001bc5c: movlps qword ptr [rax + 0xc], xmm7
14001bc60: movss dword ptr [rax + 0x14], xmm6
14001bc65: subss xmm6, xmm15
14001bc6a: movsldup xmm0, xmm14
14001bc6f: mulps xmm13, xmm0
14001bc73: subps xmm7, xmm13
14001bc77: movlps qword ptr [rax + 0x18], xmm7
14001bc7b: movss dword ptr [rax + 0x20], xmm6
14001bc80: movss xmm6, dword ptr [rsi]
14001bc84: subss xmm6, dword ptr [rdi - 0xc]
14001bc89: movss xmm7, dword ptr [rsi + 8]
14001bc8e: subss xmm7, dword ptr [rdi - 4]
14001bc93: mov qword ptr [rbp + 0x6c0], 3
14001bc9e: mov qword ptr [rbp + 0x6c8], rax
14001bca5: mov qword ptr [rbp + 0x6d0], 3
14001bcb0: mov byte ptr [rbp + 0x816], 1
14001bcb7: mov byte ptr [rbp + 0x815], 1
14001bcbe: lea rcx, [rbp + 0x460]
14001bcc5: lea rdx, [rbp + 0x6c0]
14001bccc: mov r8d, 1
14001bcd2: call 0x140153640 ; utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14001bcd7: xor eax, eax
14001bcd9: cmp rax, qword ptr [rbp + 0x460]
14001bce0: jo 0x14001bf40
14001bce6: mov rax, qword ptr [rbp + 0x490]
14001bced: mov qword ptr [rbp + 0x6f0], rax
14001bcf4: movups xmm0, xmmword ptr [rbp + 0x460]
14001bcfb: movups xmm1, xmmword ptr [rbp + 0x470]
14001bd02: movups xmm2, xmmword ptr [rbp + 0x480]
14001bd09: movaps xmmword ptr [rbp + 0x6e0], xmm2
14001bd10: movaps xmmword ptr [rbp + 0x6d0], xmm1
14001bd17: movaps xmmword ptr [rbp + 0x6c0], xmm0
14001bd1e: mov r8d, 3
14001bd24: lea rcx, [rbp + 0x508]
14001bd2b: movss xmm1, dword ptr [rip + 0x2909bd5]
14001bd33: lea r9, [rip + 0x290ac36]
14001bd3a: call 0x1402b6ad0 ; alloc::vec::from_elem<f32>
14001bd3f: test r12, r12
14001bd42: movss xmm8, dword ptr [rip + 0x2909bb9]
14001bd4b: movzx ebx, byte ptr [rbp + 0x588]
14001bd52: movzx esi, byte ptr [rbp + 0x678]
14001bd59: je 0x140020fcc
14001bd5f: mov rax, qword ptr [rbp + 0x7f0]
14001bd66: movss xmm1, dword ptr [rax]
14001bd6a: mov r8d, 3
14001bd70: lea rcx, [rbp + 0x3e0]
14001bd77: lea r9, [rip + 0x290ac22]
14001bd7e: call 0x1402b6ad0 ; alloc::vec::from_elem<f32>
14001bd83: movaps xmm1, xmm7
14001bd86: xorps xmm1, xmmword ptr [rip + 0x2909b63]
14001bd8d: mulss xmm7, xmm7
14001bd91: movaps xmm0, xmm6
14001bd94: mulss xmm0, xmm6
14001bd98: addss xmm0, xmm7
14001bd9c: sqrtss xmm0, xmm0
14001bda0: movaps xmm2, xmm8
14001bda4: divss xmm2, xmm0
14001bda8: mulss xmm1, xmm2
14001bdac: mulss xmm2, xmm6
14001bdb0: lea rax, [rip + 0x290ac01]
14001bdb7: mov qword ptr [rsp + 0x20], rax
14001bdbc: mov r9d, 3
14001bdc2: lea rcx, [rbp + 0x380]
14001bdc9: call 0x1402b6c60 ; alloc::vec::from_elem<glam::f32::vec2::Vec2>
14001bdce: mov rax, qword ptr [rbp + 0x6f0]
14001bdd5: mov qword ptr [rbp + 0x5c0], rax
14001bddc: movaps xmm0, xmmword ptr [rbp + 0x6c0]
14001bde3: movaps xmm1, xmmword ptr [rbp + 0x6d0]
14001bdea: movaps xmm2, xmmword ptr [rbp + 0x6e0]
14001bdf1: movaps xmmword ptr [rbp + 0x5b0], xmm2
14001bdf8: movaps xmmword ptr [rbp + 0x5a0], xmm1
14001bdff: movaps xmmword ptr [rbp + 0x590], xmm0
14001be06: mov rax, qword ptr [rbp + 0x390]
14001be0d: lea rcx, [rbp + 0x598]
14001be14: mov qword ptr [rcx + 0x40], rax
14001be18: movups xmm0, xmmword ptr [rbp + 0x380]
14001be1f: movups xmmword ptr [rcx + 0x30], xmm0
14001be23: mov rax, qword ptr [rbp + 0x518]
14001be2a: mov qword ptr [rcx + 0x58], rax
14001be2e: movups xmm0, xmmword ptr [rbp + 0x508]
14001be35: movups xmmword ptr [rcx + 0x48], xmm0
14001be39: mov rax, qword ptr [rbp + 0x3f0]
14001be40: mov qword ptr [rcx + 0x70], rax
14001be44: movups xmm0, xmmword ptr [rbp + 0x3e0]
14001be4b: movups xmmword ptr [rcx + 0x60], xmm0
14001be4f: movaps xmm0, xmmword ptr [rbp + 0x590]
14001be56: movaps xmm1, xmmword ptr [rbp + 0x5a0]
14001be5d: movdqa xmm2, xmmword ptr [rbp + 0x5b0]
14001be65: movapd xmm3, xmmword ptr [rbp + 0x5c0]
14001be6d: movaps xmmword ptr [rbp + 0x6c0], xmm0
14001be74: movaps xmmword ptr [rbp + 0x6d0], xmm1
14001be7b: movdqa xmmword ptr [rbp + 0x6e0], xmm2
14001be83: movapd xmmword ptr [rbp + 0x6f0], xmm3
14001be8b: movaps xmm0, xmmword ptr [rbp + 0x5d0]
14001be92: movaps xmmword ptr [rbp + 0x700], xmm0
14001be99: movaps xmm0, xmmword ptr [rbp + 0x5e0]
14001bea0: movaps xmmword ptr [rbp + 0x710], xmm0
14001bea7: movaps xmm0, xmmword ptr [rbp + 0x5f0]
14001beae: movaps xmmword ptr [rbp + 0x720], xmm0
14001beb5: movdqa xmm0, xmmword ptr [rbp + 0x600]
14001bebd: movdqa xmmword ptr [rbp + 0x730], xmm0
14001bec5: mov rax, qword ptr [rbp + 0x438]
14001becc: mov qword ptr [rbp + 0x740], rax
14001bed3: mov rax, qword ptr [rbp + 0x670]
14001beda: mov rax, qword ptr [rax]
14001bedd: mov qword ptr [rbp + 0x748], rax
14001bee4: mov rax, qword ptr [rbp + 0x788]
14001beeb: mov dword ptr [rbp + 0x750], eax
14001bef1: mov dword ptr [rbp + 0x754], 0
14001befb: mov byte ptr [rbp + 0x758], sil
14001bf02: mov byte ptr [rbp + 0x759], bl
14001bf08: mov word ptr [rbp + 0x75a], 0x301
14001bf11: mov byte ptr [rbp + 0x770], 2
14001bf18: mov byte ptr [rbp + 0x816], 1
14001bf1f: mov byte ptr [rbp + 0x815], 1
14001bf26: lea rcx, [rbp + 0x440]
14001bf2d: lea rdx, [rbp + 0x6c0]
14001bf34: lea r8, [rip + 0x290aa95]
14001bf3b: call 0x140511a10 ; alloc::collections::vec_deque::VecDeque::push_back<system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::ArchSegment,alloc::alloc::Global>
14001bf40: mov rax, qword ptr [rbp + 0x690]
14001bf47: mov qword ptr [rbp + 0x6d0], rax
14001bf4e: movaps xmm0, xmmword ptr [rbp + 0x680]
14001bf55: movaps xmmword ptr [rbp + 0x6c0], xmm0
14001bf5c: mov byte ptr [rbp + 0x816], 0
14001bf63: mov byte ptr [rbp + 0x815], 1
14001bf6a: lea rcx, [rbp + 0x460]
14001bf71: lea rdx, [rbp + 0x6c0]
14001bf78: mov r8d, 1
14001bf7e: call 0x140153640 ; utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14001bf83: xor eax, eax
14001bf85: cmp rax, qword ptr [rbp + 0x460]
14001bf8c: jo 0x14001b6de
14001bf92: mov rax, qword ptr [rbp + 0x490]
14001bf99: mov qword ptr [rbp + 0x5c0], rax
14001bfa0: movups xmm0, xmmword ptr [rbp + 0x460]
14001bfa7: movups xmm1, xmmword ptr [rbp + 0x470]
14001bfae: movups xmm2, xmmword ptr [rbp + 0x480]
14001bfb5: movaps xmmword ptr [rbp + 0x5b0], xmm2
14001bfbc: movaps xmmword ptr [rbp + 0x5a0], xmm1
14001bfc3: movaps xmmword ptr [rbp + 0x590], xmm0
14001bfca: mov rax, qword ptr [rbp + 0x578]
14001bfd1: lea rcx, [rbp + 0x598]
14001bfd8: mov qword ptr [rcx + 0x58], rax
14001bfdc: movups xmm0, xmmword ptr [rbp + 0x568]
14001bfe3: movups xmmword ptr [rcx + 0x48], xmm0
14001bfe7: mov rax, qword ptr [rbp + 0x660]
14001bfee: mov qword ptr [rcx + 0x40], rax
14001bff2: movaps xmm0, xmmword ptr [rbp + 0x650]
14001bff9: movups xmmword ptr [rcx + 0x30], xmm0
14001bffd: movaps xmm0, xmmword ptr [rbp + 0x590]
14001c004: movaps xmm1, xmmword ptr [rbp + 0x5a0]
14001c00b: movdqa xmm2, xmmword ptr [rbp + 0x5b0]
14001c013: mov rax, qword ptr [rbp + 0x5c0]
14001c01a: mov rcx, qword ptr [rbp + 0x5c8]
14001c021: movaps xmmword ptr [rbp + 0x6c0], xmm0
14001c028: movaps xmmword ptr [rbp + 0x6d0], xmm1
14001c02f: movdqa xmmword ptr [rbp + 0x6e0], xmm2
14001c037: mov qword ptr [rbp + 0x6f0], rax
14001c03e: mov qword ptr [rbp + 0x6f8], rcx
14001c045: movaps xmm0, xmmword ptr [rbp + 0x5d0]
14001c04c: movaps xmmword ptr [rbp + 0x700], xmm0
14001c053: movaps xmm0, xmmword ptr [rbp + 0x5e0]
14001c05a: movaps xmmword ptr [rbp + 0x710], xmm0
14001c061: mov rax, qword ptr [rbp + 0x5f0]
14001c068: mov qword ptr [rbp + 0x720], rax
14001c06f: mov rax, qword ptr [rbp + 0x7d0]
14001c076: mov qword ptr [rbp + 0x728], rax
14001c07d: mov rax, qword ptr [rbp + 0x7f0]
14001c084: mov qword ptr [rbp + 0x730], rax
14001c08b: mov qword ptr [rbp + 0x738], r12
14001c092: mov rax, qword ptr [rbp + 0x438]
14001c099: mov qword ptr [rbp + 0x740], rax
14001c0a0: mov rax, qword ptr [rbp + 0x670]
14001c0a7: mov rax, qword ptr [rax]
14001c0aa: mov qword ptr [rbp + 0x748], rax
14001c0b1: mov rax, qword ptr [rbp + 0x788]
14001c0b8: mov dword ptr [rbp + 0x750], eax
14001c0be: mov dword ptr [rbp + 0x754], 0
14001c0c8: movzx eax, byte ptr [rbp + 0x678]
14001c0cf: mov byte ptr [rbp + 0x758], al
14001c0d5: movzx eax, byte ptr [rbp + 0x588]
14001c0dc: mov byte ptr [rbp + 0x759], al
14001c0e2: mov word ptr [rbp + 0x75a], 0x301
14001c0eb: mov byte ptr [rbp + 0x770], 2
14001c0f2: mov byte ptr [rbp + 0x816], 0
14001c0f9: mov byte ptr [rbp + 0x815], 0
14001c100: lea rcx, [rbp + 0x440]
14001c107: lea rdx, [rbp + 0x6c0]
14001c10e: lea r8, [rip + 0x290a8d3]
14001c115: call 0x140511a10 ; alloc::collections::vec_deque::VecDeque::push_back<system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::ArchSegment,alloc::alloc::Global>
14001c11a: lea r12, [rbp + 0x590]
14001c121: mov r14, qword ptr [rbp + 0x7e0]
14001c128: mov rdi, qword ptr [rbp + 0x698]
14001c12f: mov r8, qword ptr [rbp + 0x560]
14001c136: jmp 0x140018efc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001c13b: cmp r14, rcx
14001c13e: je 0x14001c19f
14001c140: movss xmm0, dword ptr [r15 + r8*4]
14001c146: movaps xmm1, xmm8
14001c14a: subss xmm1, xmm0
14001c14e: movsd xmm2, qword ptr [rbp + 0x550]
14001c156: unpcklps xmm1, xmm0
14001c159: mulps xmm1, xmm2
14001c15c: movshdup xmm0, xmm1
14001c160: addss xmm0, xmm1
14001c164: movss dword ptr [rax + r8*4], xmm0
14001c16a: movss xmm0, dword ptr [r15 + r8*4 + 4]
14001c171: movaps xmm1, xmm8
14001c175: subss xmm1, xmm0
14001c179: movsd xmm2, qword ptr [rbp + 0x550]
14001c181: unpcklps xmm1, xmm0
14001c184: mulps xmm1, xmm2
14001c187: movshdup xmm0, xmm1
14001c18b: addss xmm0, xmm1
14001c18f: movss dword ptr [rax + r8*4 + 4], xmm0
14001c196: lea rcx, [r8 + 2]
14001c19a: mov r8, rcx
14001c19d: jmp 0x14001c13b ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001c19f: mov qword ptr [rbp + 0x7f0], rax
14001c1a6: mov r15, qword ptr [rbp + 0x5a0]
14001c1ad: mov rax, qword ptr [rbp + 0x5b8]
14001c1b4: cmp rax, r15
14001c1b7: cmovb r15, rax
14001c1bb: lea rdi, [r15*4]
14001c1c3: mov rax, r15
14001c1c6: shr rax, 0x3e
14001c1ca: jne 0x140020d6d
14001c1d0: mov esi, 0
14001c1d5: movabs rax, 0x7ffffffffffffffc
14001c1df: cmp rdi, rax
14001c1e2: ja 0x140020d6f
14001c1e8: mov r12, qword ptr [rbp + 0x5b0]
14001c1ef: test rdi, rdi
14001c1f2: je 0x14001c223
14001c1f4: movzx eax, byte ptr [rip + 0x3618a0e]
14001c1fb: mov esi, 4
14001c200: mov edx, 4
14001c205: mov rcx, rdi
14001c208: call 0x140613c10 ; ?
14001c20d: test rax, rax
14001c210: je 0x140020d6f
14001c216: mov rcx, r15
14001c219: test r15, r15
14001c21c: jne 0x14001c233
14001c21e: jmp 0x14001c350 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001c223: mov eax, 4
14001c228: xor ecx, ecx
14001c22a: test r15, r15
14001c22d: je 0x14001c350
14001c233: cmp r15, 8
14001c237: jb 0x14001c2c9
14001c23d: mov r8, rax
14001c240: sub r8, r12
14001c243: mov edx, 0
14001c248: cmp r8, 0x20
14001c24c: jb 0x14001c2cb
14001c24e: mov rdx, r15
14001c251: movabs r8, 0x3ffffffffffffff8
14001c25b: and rdx, r8
14001c25e: movaps xmm0, xmm10
14001c262: shufps xmm0, xmm10, 0
14001c267: movaps xmm1, xmm10
14001c26b: shufps xmm1, xmm10, 0x55
14001c270: xor r8d, r8d
14001c273: nop word ptr cs:[rax + rax]
14001c280: movups xmm2, xmmword ptr [r12 + r8*4]
14001c285: movups xmm3, xmmword ptr [r12 + r8*4 + 0x10]
14001c28b: movaps xmm4, xmm9
14001c28f: subps xmm4, xmm2
14001c292: movaps xmm5, xmm9
14001c296: subps xmm5, xmm3
14001c299: mulps xmm4, xmm0
14001c29c: mulps xmm5, xmm0
14001c29f: mulps xmm2, xmm1
14001c2a2: addps xmm2, xmm4
14001c2a5: mulps xmm3, xmm1
14001c2a8: addps xmm3, xmm5
14001c2ab: movups xmmword ptr [rax + r8*4], xmm2
14001c2b0: movups xmmword ptr [rax + r8*4 + 0x10], xmm3
14001c2b6: add r8, 8
14001c2ba: cmp rdx, r8
14001c2bd: jne 0x14001c280
14001c2bf: cmp r15, rdx
14001c2c2: jne 0x14001c2cb
14001c2c4: jmp 0x14001c350 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001c2c9: xor edx, edx
14001c2cb: mov r8, rdx
14001c2ce: or r8, 1
14001c2d2: test r15b, 1
14001c2d6: je 0x14001c2fc
14001c2d8: movss xmm0, dword ptr [r12 + rdx*4]
14001c2de: movaps xmm1, xmm8
14001c2e2: subss xmm1, xmm0
14001c2e6: unpcklps xmm1, xmm0
14001c2e9: mulps xmm1, xmm10
14001c2ed: movshdup xmm0, xmm1
14001c2f1: addss xmm0, xmm1
14001c2f5: movss dword ptr [rax + rdx*4], xmm0
14001c2fa: jmp 0x14001c34b ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001c2fc: cmp r15, r8
14001c2ff: je 0x14001c350
14001c301: movss xmm0, dword ptr [r12 + rdx*4]
14001c307: movaps xmm1, xmm8
14001c30b: subss xmm1, xmm0
14001c30f: unpcklps xmm1, xmm0
14001c312: mulps xmm1, xmm10
14001c316: movshdup xmm0, xmm1
14001c31a: addss xmm0, xmm1
14001c31e: movss dword ptr [rax + rdx*4], xmm0
14001c323: movss xmm0, dword ptr [r12 + rdx*4 + 4]
14001c32a: movaps xmm1, xmm8
14001c32e: subss xmm1, xmm0
14001c332: unpcklps xmm1, xmm0
14001c335: mulps xmm1, xmm10
14001c339: movshdup xmm0, xmm1
14001c33d: addss xmm0, xmm1
14001c341: movss dword ptr [rax + rdx*4 + 4], xmm0
14001c347: lea r8, [rdx + 2]
14001c34b: mov rdx, r8
14001c34e: jmp 0x14001c2fc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001c350: mov rdx, qword ptr [rbp + 0x258]
14001c357: mov r8d, dword ptr [rbp + 0x424]
14001c35e: mov dword ptr [rdx], r8d
14001c361: mov rdx, qword ptr [rbp + 0x5c0]
14001c368: mov qword ptr [rbp + 0x6f0], rdx
14001c36f: movaps xmm0, xmmword ptr [rbp + 0x590]
14001c376: movaps xmm1, xmmword ptr [rbp + 0x5a0]
14001c37d: movaps xmm2, xmmword ptr [rbp + 0x5b0]
14001c384: movaps xmmword ptr [rbp + 0x6e0], xmm2
14001c38b: movaps xmmword ptr [rbp + 0x6d0], xmm1
14001c392: movaps xmmword ptr [rbp + 0x6c0], xmm0
14001c399: mov rdx, qword ptr [rbp + 0x808]
14001c3a0: mov qword ptr [rbp + 0x6f8], rdx
14001c3a7: mov rdx, qword ptr [rbp + 0x7a0]
14001c3ae: mov qword ptr [rbp + 0x700], rdx
14001c3b5: mov qword ptr [rbp + 0x708], rbx
14001c3bc: mov rdx, qword ptr [rbp + 0x6b0]
14001c3c3: mov qword ptr [rbp + 0x728], rdx
14001c3ca: mov rdx, qword ptr [rbp + 0x7f0]
14001c3d1: mov qword ptr [rbp + 0x730], rdx
14001c3d8: mov qword ptr [rbp + 0x738], r14
14001c3df: mov qword ptr [rbp + 0x710], rcx
14001c3e6: mov qword ptr [rbp + 0x718], rax
14001c3ed: mov qword ptr [rbp + 0x720], r15
14001c3f4: mov rax, qword ptr [rbp + 0x788]
14001c3fb: mov dword ptr [rbp + 0x760], eax
14001c401: mov rax, qword ptr [rbp + 0x670]
14001c408: mov rax, qword ptr [rax]
14001c40b: mov qword ptr [rbp + 0x758], rax
14001c412: mov word ptr [rbp + 0x764], 0
14001c41b: movabs rax, 0x8000000000000000
14001c425: mov qword ptr [rbp + 0x740], rax
14001c42c: mov rsi, qword ptr [r13 + 0x10]
14001c430: cmp rsi, qword ptr [r13]
14001c434: jne 0x14001c445
14001c436: mov rcx, r13
14001c439: lea rdx, [rip + 0x290a3e8]
14001c440: call 0x1405636c0 ; alloc::raw_vec::RawVec::grow_one<country_core::resources::stairs_walls::StairsBracketAssemblyParams,alloc::alloc::Global>
14001c445: imul rcx, rsi, 0xa8
14001c44c: add rcx, qword ptr [r13 + 8]
14001c450: mov r8d, 0xa8
14001c456: lea rdx, [rbp + 0x6c0]
14001c45d: call 0x142923c10 ; ?
14001c462: inc rsi
14001c465: mov qword ptr [r13 + 0x10], rsi
14001c469: mov rax, qword ptr [rbp + 0x200]
14001c470: mov byte ptr [rbp + 0x817], 1
14001c477: mov qword ptr [rsp + 0x48], rax
14001c47c: mov rax, qword ptr [rbp + 0x6a0]
14001c483: mov qword ptr [rsp + 0x40], rax
14001c488: lea rax, [rbp + 0x278]
14001c48f: mov qword ptr [rsp + 0x38], rax
14001c494: lea rax, [rbp + 0x550]
14001c49b: mov qword ptr [rsp + 0x20], rax
14001c4a0: mov qword ptr [rsp + 0x30], 1
14001c4a9: mov qword ptr [rsp + 0x28], 2
14001c4b2: mov edx, 2
14001c4b7: mov r9d, 2
14001c4bd: lea rcx, [rbp + 0x650]
14001c4c4: lea r8, [rbp + 0x300]
14001c4cb: call 0x1407de8b0 ; system_decorator::decorator_visual::stairs_assemble::gen_stairs_collider_and_rt_geo
14001c4d0: mov byte ptr [rbp + 0x817], 1
14001c4d7: lea rax, [rbp + 0x428]
14001c4de: mov qword ptr [rsp + 0x38], rax
14001c4e3: mov eax, dword ptr [rbp + 0x640]
14001c4e9: mov dword ptr [rsp + 0x30], eax
14001c4ed: lea rax, [rbp + 0x804]
14001c4f4: mov qword ptr [rsp + 0x28], rax
14001c4f9: mov qword ptr [rsp + 0x20], 2
14001c502: mov r8d, 2
14001c508: lea rcx, [rbp + 0x198]
14001c50f: lea rdx, [rbp + 0x650]
14001c516: lea r9, [rbp + 0x550]
14001c51d: call 0x1407db3e0 ; system_decorator::decorator_visual::stairs_assemble::construct_ladder
14001c522: movzx eax, byte ptr [rbp + 0x804]
14001c529: cmp eax, 1
14001c52c: lea r12, [rbp + 0x590]
14001c533: jbe 0x14001d34e
14001c539: cmp eax, 3
14001c53c: je 0x14001c63d
14001c542: cmp eax, 4
14001c545: movss xmm3, dword ptr [rip + 0x29093b7]
14001c54d: jne 0x14001c674
14001c553: movaps xmm0, xmm14
14001c557: movlhps xmm0, xmm13
14001c55b: shufps xmm0, xmm13, 0xe2
14001c560: movaps xmm1, xmm12
14001c564: movlhps xmm1, xmm15
14001c568: shufps xmm1, xmm15, 0xe2
14001c56d: subps xmm0, xmm1
14001c570: movaps xmm1, xmm0
14001c573: mulps xmm1, xmm0
14001c576: movshdup xmm2, xmm1
14001c57a: addss xmm2, xmm1
14001c57e: xorps xmm1, xmm1
14001c581: sqrtss xmm1, xmm2
14001c585: movaps xmm2, xmm3
14001c588: divss xmm2, xmm1
14001c58c: movd eax, xmm2
14001c590: test eax, eax
14001c592: setns cl
14001c595: mov edx, eax
14001c597: and edx, 0x7fffffff
14001c59d: add edx, 0xff800000
14001c5a3: cmp edx, 0x7f000000
14001c5a9: setb dl
14001c5ac: and dl, cl
14001c5ae: dec eax
14001c5b0: cmp eax, 0x7fffff
14001c5b5: setb al
14001c5b8: or al, dl
14001c5ba: movsldup xmm1, xmm2
14001c5be: mulps xmm1, xmm0
14001c5c1: movzx eax, al
14001c5c4: movd xmm0, eax
14001c5c8: pshufd xmm11, xmm0, 0x50
14001c5ce: pslld xmm11, 0x1f
14001c5d4: psrad xmm11, 0x1f
14001c5da: pand xmm11, xmm1
14001c5df: movq qword ptr [rbp + 0x6c0], xmm11
14001c5e8: mov byte ptr [rbp + 0x817], 1
14001c5ef: lea rcx, [rbp + 0x68]
14001c5f3: lea rdx, [rbp + 0x6c0]
14001c5fa: call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
14001c5ff: movsd xmm0, qword ptr [rbp + 0x68]
14001c604: mulps xmm0, xmmword ptr [rip + 0x2909335]
14001c60b: movss xmm1, dword ptr [rbp + 0x70]
14001c610: mulss xmm1, dword ptr [rip + 0x2909338]
14001c618: subss xmm12, xmm1
14001c61d: subps xmm15, xmm0
14001c621: addps xmm13, xmm0
14001c625: addss xmm14, xmm1
14001c62a: mov rbx, qword ptr [rbp + 0x550]
14001c631: lea rcx, [rbp + 0x460]
14001c638: jmp 0x14001c8bd ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001c63d: movaps xmm0, xmm15
14001c641: cmpltps xmm0, xmm13
14001c646: pextrw eax, xmm0, 2
14001c64b: test al, 1
14001c64d: movss xmm3, dword ptr [rip + 0x29092af]
14001c655: je 0x14001c686
14001c657: mov esi, dword ptr [rbp + 0x550]
14001c65d: movaps xmm1, xmm6
14001c660: mulps xmm1, xmm6
14001c663: movshdup xmm0, xmm1
14001c667: addss xmm0, xmm1
14001c66b: mov dil, 1
14001c66e: movaps xmm11, xmm6
14001c672: jmp 0x14001c69e ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001c674: movaps xmm0, xmm13
14001c678: cmpltps xmm0, xmm15
14001c67d: pextrw eax, xmm0, 2
14001c682: test al, 1
14001c684: jne 0x14001c657
14001c686: mov esi, dword ptr [rbp + 0x554]
14001c68c: movaps xmm1, xmm11
14001c690: mulps xmm1, xmm11
14001c694: movshdup xmm0, xmm1
14001c698: addss xmm0, xmm1
14001c69c: xor edi, edi
14001c69e: sqrtss xmm0, xmm0
14001c6a2: movaps xmm1, xmm3
14001c6a5: divss xmm1, xmm0
14001c6a9: movd eax, xmm1
14001c6ad: test eax, eax
14001c6af: setns cl
14001c6b2: mov edx, eax
14001c6b4: and edx, 0x7fffffff
14001c6ba: add edx, 0xff800000
14001c6c0: cmp edx, 0x7f000000
14001c6c6: setb dl
14001c6c9: and dl, cl
14001c6cb: dec eax
14001c6cd: cmp eax, 0x7fffff
14001c6d2: setb al
14001c6d5: or al, dl
14001c6d7: movsldup xmm0, xmm1
14001c6db: mulps xmm0, xmm11
14001c6df: movzx eax, al
14001c6e2: movd xmm1, eax
14001c6e6: pshufd xmm1, xmm1, 0x50
14001c6eb: pslld xmm1, 0x1f
14001c6f0: psrad xmm1, 0x1f
14001c6f5: pand xmm1, xmm0
14001c6f9: movq qword ptr [rbp + 0x590], xmm1
14001c701: movshdup xmm0, xmm15
14001c706: movshdup xmm1, xmm13
14001c70b: movaps xmm11, xmm0
14001c70f: cmpunordss xmm11, xmm0
14001c715: movaps xmm2, xmm11
14001c719: andps xmm2, xmm1
14001c71c: minss xmm1, xmm0
14001c720: andnps xmm11, xmm1
14001c724: orps xmm11, xmm2
14001c728: movss dword ptr [rbp + 0x6c0], xmm15
14001c731: movss dword ptr [rbp + 0x6c4], xmm12
14001c73a: mov byte ptr [rbp + 0x817], 1
14001c741: lea rcx, [rbp + 0x228]
14001c748: lea rdx, [rbp + 0x6c0]
14001c74f: movaps xmm2, xmm11
14001c753: call 0x140c9cde0 ; utils::swizzlers::impl$1::xvy
14001c758: movss dword ptr [rbp + 0x6c0], xmm13
14001c761: movss dword ptr [rbp + 0x6c4], xmm14
14001c76a: mov byte ptr [rbp + 0x817], 1
14001c771: lea rcx, [rbp + 0x218]
14001c778: lea rdx, [rbp + 0x6c0]
14001c77f: movaps xmm2, xmm11
14001c783: call 0x140c9cde0 ; utils::swizzlers::impl$1::xvy
14001c788: test dil, dil
14001c78b: je 0x14001c7e9
14001c78d: mov byte ptr [rbp + 0x817], 1
14001c794: lea rcx, [rbp + 0x6c0]
14001c79b: lea rdx, [rbp + 0x590]
14001c7a2: call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
14001c7a7: movsd xmm13, qword ptr [rbp + 0x6c0]
14001c7b0: movss xmm14, dword ptr [rbp + 0x6c8]
14001c7b9: cmp byte ptr [rbp + 0x804], 3
14001c7c0: jne 0x14001c85b
14001c7c6: mulps xmm13, xmmword ptr [rip + 0x2909112]
14001c7ce: mulss xmm14, dword ptr [rip + 0x2909099]
14001c7d7: movsd xmm0, qword ptr [rbp + 0x228]
14001c7df: movss xmm1, dword ptr [rbp + 0x230]
14001c7e7: jmp 0x14001c83f ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001c7e9: mov byte ptr [rbp + 0x817], 1
14001c7f0: lea rcx, [rbp + 0x6c0]
14001c7f7: lea rdx, [rbp + 0x590]
14001c7fe: call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
14001c803: movsd xmm13, qword ptr [rbp + 0x6c0]
14001c80c: movss xmm14, dword ptr [rbp + 0x6c8]
14001c815: cmp byte ptr [rbp + 0x804], 3
14001c81c: jne 0x14001c878
14001c81e: mulps xmm13, xmmword ptr [rip + 0x29090ba]
14001c826: mulss xmm14, dword ptr [rip + 0x2909041]
14001c82f: movsd xmm0, qword ptr [rbp + 0x218]
14001c837: movss xmm1, dword ptr [rbp + 0x220]
14001c83f: movaps xmm15, xmm0
14001c843: subps xmm15, xmm13
14001c847: movaps xmm12, xmm1
14001c84b: subss xmm12, xmm14
14001c850: addps xmm13, xmm0
14001c854: addss xmm14, xmm1
14001c859: jmp 0x14001c8a1 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001c85b: movsd xmm15, qword ptr [rbp + 0x228]
14001c864: movss xmm12, dword ptr [rbp + 0x230]
14001c86d: addps xmm13, xmm15
14001c871: addss xmm14, xmm12
14001c876: jmp 0x14001c8a1 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001c878: movsd xmm0, qword ptr [rbp + 0x218]
14001c880: movss xmm1, dword ptr [rbp + 0x220]
14001c888: movaps xmm15, xmm0
14001c88c: subps xmm15, xmm13
14001c890: movaps xmm12, xmm1
14001c894: subss xmm12, xmm14
14001c899: movaps xmm14, xmm1
14001c89d: movaps xmm13, xmm0
14001c8a1: lea rcx, [rbp + 0x460]
14001c8a8: movq xmm11, qword ptr [rbp + 0x590]
14001c8b1: mov eax, esi
14001c8b3: mov rbx, rax
14001c8b6: shl rbx, 0x20
14001c8ba: or rbx, rax
14001c8bd: movlps qword ptr [rbp + 0x680], xmm15
14001c8c5: movss dword ptr [rbp + 0x688], xmm12
14001c8ce: movlps qword ptr [rbp + 0x68c], xmm13
14001c8d6: movss dword ptr [rbp + 0x694], xmm14
14001c8df: movshdup xmm0, xmm15
14001c8e4: minss xmm0, dword ptr [rip + 0x2909068]
14001c8ec: movshdup xmm1, xmm13
14001c8f1: minss xmm1, xmm0
14001c8f5: movss dword ptr [rbp + 0x568], xmm1
14001c8fd: lea rax, [rbp + 0x680]
14001c904: mov qword ptr [rbp + 0x6c0], rax
14001c90b: lea rax, [rbp + 0x698]
14001c912: mov qword ptr [rbp + 0x6c8], rax
14001c919: lea rax, [rbp + 0x568]
14001c920: mov qword ptr [rbp + 0x6d0], rax
14001c927: mov byte ptr [rbp + 0x817], 1
14001c92e: lea rdx, [rbp + 0x6c0]
14001c935: lea r8, [rip + 0x290a9f4]
14001c93c: call 0x1402befa0 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$16<false> > >
14001c941: mov byte ptr [rbp + 0x817], 1
14001c948: lea rcx, [rbp + 0x590]
14001c94f: lea rdx, [rbp + 0x460]
14001c956: xor r8d, r8d
14001c959: call 0x140153640 ; utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14001c95e: xor esi, esi
14001c960: cmp rsi, qword ptr [rbp + 0x590]
14001c967: jo 0x14001d34e
14001c96d: mov rax, qword ptr [rbp + 0x5c0]
14001c974: mov qword ptr [rbp + 0x6f0], rax
14001c97b: movups xmm0, xmmword ptr [rbp + 0x590]
14001c982: movups xmm1, xmmword ptr [rbp + 0x5a0]
14001c989: movups xmm2, xmmword ptr [rbp + 0x5b0]
14001c990: movaps xmmword ptr [rbp + 0x6e0], xmm2
14001c997: movaps xmmword ptr [rbp + 0x6d0], xmm1
14001c99e: movaps xmmword ptr [rbp + 0x6c0], xmm0
14001c9a5: lea rcx, [rbp + 0x380]
14001c9ac: lea rdx, [rbp + 0x6c0]
14001c9b3: movd xmm2, dword ptr [rip + 0x2908eb5]
14001c9bb: mov r9d, 1
14001c9c1: call 0x140152b00 ; utils::curve::Curve::try_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14001c9c6: mov rax, qword ptr [rbp + 0x6c0]
14001c9cd: test rax, rax
14001c9d0: je 0x14001c9ec
14001c9d2: mov rcx, qword ptr [rbp + 0x6c8]
14001c9d9: shl rax, 2
14001c9dd: lea rdx, [rax + rax*2]
14001c9e1: mov r8d, 4
14001c9e7: call 0x140613c20 ; ?
14001c9ec: mov rdx, qword ptr [rbp + 0x6d8]
14001c9f3: test rdx, rdx
14001c9f6: je 0x14001ca0e
14001c9f8: mov rcx, qword ptr [rbp + 0x6e0]
14001c9ff: shl rdx, 2
14001ca03: mov r8d, 4
14001ca09: call 0x140613c20 ; ?
14001ca0e: cmp rsi, qword ptr [rbp + 0x380]
14001ca15: jo 0x14001d34e
14001ca1b: mov rax, qword ptr [rbp + 0x3b0]
14001ca22: mov qword ptr [rbp + 0x490], rax
14001ca29: movupd xmm0, xmmword ptr [rbp + 0x380]
14001ca31: movups xmm1, xmmword ptr [rbp + 0x390]
14001ca38: movups xmm2, xmmword ptr [rbp + 0x3a0]
14001ca3f: movaps xmmword ptr [rbp + 0x480], xmm2
14001ca46: movaps xmmword ptr [rbp + 0x470], xmm1
14001ca4d: movapd xmmword ptr [rbp + 0x460], xmm0
14001ca55: mov rdi, qword ptr [rbp + 0x470]
14001ca5c: test rdi, rdi
14001ca5f: je 0x14001cb1b
14001ca65: lea rsi, [rdi*8]
14001ca6d: movzx eax, byte ptr [rip + 0x3618195]
14001ca74: mov edx, 4
14001ca79: mov rcx, rsi
14001ca7c: call 0x140613c10 ; ?
14001ca81: test rax, rax
14001ca84: je 0x140020c7d
14001ca8a: mov r8d, 1
14001ca90: mov rcx, rax
14001ca93: cmp rdi, 1
14001ca97: movss xmm7, dword ptr [rip + 0x2908e65]
14001ca9f: movaps xmm8, xmmword ptr [rip + 0x2908e89]
14001caa7: je 0x14001cb14
14001caa9: mov edx, 1
14001caae: mov rcx, rax
14001cab1: cmp rdi, 4
14001cab5: jbe 0x14001cb00
14001cab7: lea r9, [rdi - 1]
14001cabb: mov r10, r9
14001cabe: and r10, 0xfffffffffffffffc
14001cac2: lea rcx, [rax + r10*8]
14001cac6: lea rdx, [r10 + 1]
14001caca: movddup xmm0, xmm11
14001cacf: xor r8d, r8d
14001cad2: nop word ptr cs:[rax + rax]
14001cae0: movupd xmmword ptr [rax + r8*8], xmm0
14001cae6: movupd xmmword ptr [rax + r8*8 + 0x10], xmm0
14001caed: add r8, 4
14001caf1: cmp r10, r8
14001caf4: jne 0x14001cae0
14001caf6: mov r8, rdi
14001caf9: cmp r9, r10
14001cafc: je 0x14001cb14
14001cafe: nop 
14001cb00: inc rdx
14001cb03: movlpd qword ptr [rcx], xmm11
14001cb08: add rcx, 8
14001cb0c: cmp rdi, rdx
14001cb0f: jne 0x14001cb00
14001cb11: mov r8, rdi
14001cb14: movlpd qword ptr [rcx], xmm11
14001cb19: jmp 0x14001cb33 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001cb1b: mov eax, 4
14001cb20: xor r8d, r8d
14001cb23: movss xmm7, dword ptr [rip + 0x2908dd9]
14001cb2b: movaps xmm8, xmmword ptr [rip + 0x2908dfd]
14001cb33: mov qword ptr [rbp + 0x508], r8
14001cb3a: mov qword ptr [rbp + 0x510], rax
14001cb41: mov r15, qword ptr [rbp + 0x470]
14001cb48: mov rax, qword ptr [rbp + 0x488]
14001cb4f: cmp rax, r15
14001cb52: cmovb r15, rax
14001cb56: mov qword ptr [rbp + 0x518], rdi
14001cb5d: lea rdi, [r15*4]
14001cb65: mov rax, r15
14001cb68: shr rax, 0x3e
14001cb6c: jne 0x140020d8d
14001cb72: mov esi, 0
14001cb77: movabs rax, 0x7ffffffffffffffc
14001cb81: cmp rdi, rax
14001cb84: ja 0x140020d8f
14001cb8a: mov r14, qword ptr [rbp + 0x480]
14001cb91: test rdi, rdi
14001cb94: je 0x14001cbc5
14001cb96: movzx eax, byte ptr [rip + 0x361806c]
14001cb9d: mov esi, 4
14001cba2: mov edx, 4
14001cba7: mov rcx, rdi
14001cbaa: call 0x140613c10 ; ?
14001cbaf: test rax, rax
14001cbb2: je 0x140020d8f
14001cbb8: mov rcx, r15
14001cbbb: test r15, r15
14001cbbe: jne 0x14001cbd5
14001cbc0: jmp 0x14001ccfa ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001cbc5: mov eax, 4
14001cbca: xor ecx, ecx
14001cbcc: test r15, r15
14001cbcf: je 0x14001ccfa
14001cbd5: mov rdx, rbx
14001cbd8: shr rdx, 0x20
14001cbdc: movd xmm1, ebx
14001cbe0: movd xmm0, edx
14001cbe4: punpckldq xmm0, xmm1
14001cbe8: cmp r15, 8
14001cbec: jb 0x14001cc79
14001cbf2: mov r8, rax
14001cbf5: sub r8, r14
14001cbf8: mov edx, 0
14001cbfd: cmp r8, 0x20
14001cc01: jb 0x14001cc7b
14001cc03: mov rdx, r15
14001cc06: movabs r8, 0x3ffffffffffffff8
14001cc10: and rdx, r8
14001cc13: movdqa xmm1, xmm0
14001cc17: shufps xmm1, xmm0, 0x55
14001cc1b: movaps xmm2, xmm0
14001cc1e: shufps xmm2, xmm0, 0
14001cc22: xor r8d, r8d
14001cc25: nop word ptr cs:[rax + rax]
14001cc30: movups xmm3, xmmword ptr [r14 + r8*4]
14001cc35: movups xmm4, xmmword ptr [r14 + r8*4 + 0x10]
14001cc3b: movaps xmm5, xmm8
14001cc3f: subps xmm5, xmm3
14001cc42: movaps xmm6, xmm8
14001cc46: subps xmm6, xmm4
14001cc49: mulps xmm5, xmm1
14001cc4c: mulps xmm6, xmm1
14001cc4f: mulps xmm3, xmm2
14001cc52: addps xmm3, xmm5
14001cc55: mulps xmm4, xmm2
14001cc58: addps xmm4, xmm6
14001cc5b: movups xmmword ptr [rax + r8*4], xmm3
14001cc60: movups xmmword ptr [rax + r8*4 + 0x10], xmm4
14001cc66: add r8, 8
14001cc6a: cmp rdx, r8
14001cc6d: jne 0x14001cc30
14001cc6f: cmp r15, rdx
14001cc72: jne 0x14001cc7b
14001cc74: jmp 0x14001ccfa ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001cc79: xor edx, edx
14001cc7b: mov r8, rdx
14001cc7e: or r8, 1
14001cc82: test r15b, 1
14001cc86: je 0x14001ccaa
14001cc88: movss xmm1, dword ptr [r14 + rdx*4]
14001cc8e: movaps xmm2, xmm7
14001cc91: subss xmm2, xmm1
14001cc95: unpcklps xmm1, xmm2
14001cc98: mulps xmm1, xmm0
14001cc9b: movshdup xmm2, xmm1
14001cc9f: addss xmm2, xmm1
14001cca3: movss dword ptr [rax + rdx*4], xmm2
14001cca8: jmp 0x14001ccf5 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001ccaa: cmp r15, r8
14001ccad: je 0x14001ccfa
14001ccaf: movss xmm1, dword ptr [r14 + rdx*4]
14001ccb5: movaps xmm2, xmm7
14001ccb8: subss xmm2, xmm1
14001ccbc: unpcklps xmm1, xmm2
14001ccbf: mulps xmm1, xmm0
14001ccc2: movshdup xmm2, xmm1
14001ccc6: addss xmm2, xmm1
14001ccca: movss dword ptr [rax + rdx*4], xmm2
14001cccf: movss xmm1, dword ptr [r14 + rdx*4 + 4]
14001ccd6: movaps xmm2, xmm7
14001ccd9: subss xmm2, xmm1
14001ccdd: unpcklps xmm1, xmm2
14001cce0: mulps xmm1, xmm0
14001cce3: movshdup xmm2, xmm1
14001cce7: addss xmm2, xmm1
14001cceb: movss dword ptr [rax + rdx*4 + 4], xmm2
14001ccf1: lea r8, [rdx + 2]
14001ccf5: mov rdx, r8
14001ccf8: jmp 0x14001ccaa ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001ccfa: mov qword ptr [rbp + 0x3e0], rcx
14001cd01: mov qword ptr [rbp + 0x3e8], rax
14001cd08: mov r14, qword ptr [rbp + 0x470]
14001cd0f: mov rax, qword ptr [rbp + 0x488]
14001cd16: cmp rax, r14
14001cd19: cmovb r14, rax
14001cd1d: mov qword ptr [rbp + 0x3f0], r15
14001cd24: lea rdi, [r14*4]
14001cd2c: mov rax, r14
14001cd2f: shr rax, 0x3e
14001cd33: jne 0x140020d06
14001cd39: mov esi, 0
14001cd3e: movabs rax, 0x7ffffffffffffffc
14001cd48: cmp rdi, rax
14001cd4b: ja 0x140020d08
14001cd51: mov rbx, qword ptr [rbp + 0x480]
14001cd58: test rdi, rdi
14001cd5b: je 0x14001cd90
14001cd5d: movzx eax, byte ptr [rip + 0x3617ea5]
14001cd64: mov esi, 4
14001cd69: mov edx, 4
14001cd6e: mov rcx, rdi
14001cd71: call 0x140613c10 ; ?
14001cd76: test rax, rax
14001cd79: je 0x140020d08
14001cd7f: mov qword ptr [rbp + 0x7a0], r14
14001cd86: test r14, r14
14001cd89: jne 0x14001cda9
14001cd8b: jmp 0x14001ceb6 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001cd90: mov eax, 4
14001cd95: mov qword ptr [rbp + 0x7a0], 0
14001cda0: test r14, r14
14001cda3: je 0x14001ceb6
14001cda9: cmp r14, 8
14001cdad: jb 0x14001ce35
14001cdb3: mov rcx, rax
14001cdb6: sub rcx, rbx
14001cdb9: mov edx, 0
14001cdbe: cmp rcx, 0x20
14001cdc2: jb 0x14001ce37
14001cdc4: mov rdx, r14
14001cdc7: movabs rcx, 0x3ffffffffffffff8
14001cdd1: and rdx, rcx
14001cdd4: movaps xmm0, xmm10
14001cdd8: shufps xmm0, xmm10, 0
14001cddd: movaps xmm1, xmm10
14001cde1: shufps xmm1, xmm10, 0x55
14001cde6: xor ecx, ecx
14001cde8: nop dword ptr [rax + rax]
14001cdf0: movups xmm2, xmmword ptr [rbx + rcx*4]
14001cdf4: movups xmm3, xmmword ptr [rbx + rcx*4 + 0x10]
14001cdf9: movaps xmm4, xmm8
14001cdfd: subps xmm4, xmm2
14001ce00: movaps xmm5, xmm8
14001ce04: subps xmm5, xmm3
14001ce07: mulps xmm4, xmm0
14001ce0a: mulps xmm5, xmm0
14001ce0d: mulps xmm2, xmm1
14001ce10: addps xmm2, xmm4
14001ce13: mulps xmm3, xmm1
14001ce16: addps xmm3, xmm5
14001ce19: movups xmmword ptr [rax + rcx*4], xmm2
14001ce1d: movups xmmword ptr [rax + rcx*4 + 0x10], xmm3
14001ce22: add rcx, 8
14001ce26: cmp rdx, rcx
14001ce29: jne 0x14001cdf0
14001ce2b: cmp r14, rdx
14001ce2e: jne 0x14001ce37
14001ce30: jmp 0x14001ceb6 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001ce35: xor edx, edx
14001ce37: mov rcx, rdx
14001ce3a: or rcx, 1
14001ce3e: test r14b, 1
14001ce42: je 0x14001ce66
14001ce44: movss xmm0, dword ptr [rbx + rdx*4]
14001ce49: movaps xmm1, xmm7
14001ce4c: subss xmm1, xmm0
14001ce50: unpcklps xmm1, xmm0
14001ce53: mulps xmm1, xmm10
14001ce57: movshdup xmm0, xmm1
14001ce5b: addss xmm0, xmm1
14001ce5f: movss dword ptr [rax + rdx*4], xmm0
14001ce64: jmp 0x14001ceb1 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001ce66: cmp r14, rcx
14001ce69: je 0x14001ceb6
14001ce6b: movss xmm0, dword ptr [rbx + rdx*4]
14001ce70: movaps xmm1, xmm7
14001ce73: subss xmm1, xmm0
14001ce77: unpcklps xmm1, xmm0
14001ce7a: mulps xmm1, xmm10
14001ce7e: movshdup xmm0, xmm1
14001ce82: addss xmm0, xmm1
14001ce86: movss dword ptr [rax + rdx*4], xmm0
14001ce8b: movss xmm0, dword ptr [rbx + rdx*4 + 4]
14001ce91: movaps xmm1, xmm7
14001ce94: subss xmm1, xmm0
14001ce98: unpcklps xmm1, xmm0
14001ce9b: mulps xmm1, xmm10
14001ce9f: movshdup xmm0, xmm1
14001cea3: addss xmm0, xmm1
14001cea7: movss dword ptr [rax + rdx*4 + 4], xmm0
14001cead: lea rcx, [rdx + 2]
14001ceb1: mov rdx, rcx
14001ceb4: jmp 0x14001ce66 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001ceb6: mov qword ptr [rbp + 0x808], rax
14001cebd: mov byte ptr [rbp + 0x812], 1
14001cec4: mov byte ptr [rbp + 0x811], 1
14001cecb: mov rcx, qword ptr [rbp + 0x268]
14001ced2: call 0x140987ca0 ; country_core::render::impl$0::deref
14001ced7: mov rdi, rax
14001ceda: mov r12, qword ptr [rbp + 0x3e8]
14001cee1: mov r8, qword ptr [rbp + 0x3f0]
14001cee8: mov r13, qword ptr [rbp + 0x510]
14001ceef: mov r11, qword ptr [rbp + 0x518]
14001cef6: mov rsi, qword ptr [rbp + 0x468]
14001cefd: mov rbx, qword ptr [rbp + 0x470]
14001cf04: test r8, r8
14001cf07: je 0x14001cf5f
14001cf09: mov qword ptr [rbp + 0x7f0], rbx
14001cf10: mov rbx, r11
14001cf13: mov r15, r8
14001cf16: lea rcx, [r8*4]
14001cf1e: movzx eax, byte ptr [rip + 0x3617ce4]
14001cf25: mov edx, 4
14001cf2a: mov qword ptr [rbp + 0x7d0], rcx
14001cf31: call 0x140613c40 ; ?
14001cf36: test rax, rax
14001cf39: je 0x140020c39
14001cf3f: mov r10, rax
14001cf42: mov ecx, dword ptr [rbp + 0x540]
14001cf48: movss xmm1, dword ptr [rbp + 0x370]
14001cf50: mov r8, r15
14001cf53: mov r11, rbx
14001cf56: mov rbx, qword ptr [rbp + 0x7f0]
14001cf5d: jmp 0x14001cf73 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001cf5f: mov r10d, 4
14001cf65: mov ecx, dword ptr [rbp + 0x540]
14001cf6b: movss xmm1, dword ptr [rbp + 0x370]
14001cf73: mov qword ptr [rbp + 0x6b0], r8
14001cf7a: mov eax, dword ptr [rbp + 0x500]
14001cf80: mov dword ptr [rbp + 0x6c0], eax
14001cf86: movss xmm0, dword ptr [rbp + 0x548]
14001cf8e: movss dword ptr [rbp + 0x6c4], xmm0
14001cf96: mov dword ptr [rbp + 0x6c8], ecx
14001cf9c: movss dword ptr [rbp + 0x6cc], xmm1
14001cfa4: mov qword ptr [rsp + 0x60], r8
14001cfa9: mov qword ptr [rbp + 0x7f0], r10
14001cfb0: mov qword ptr [rsp + 0x58], r10
14001cfb5: lea rax, [rbp + 0x6c0]
14001cfbc: mov qword ptr [rsp + 0x50], rax
14001cfc1: mov rax, qword ptr [rbp + 0x788]
14001cfc8: mov dword ptr [rsp + 0x48], eax
14001cfcc: mov rax, qword ptr [rbp + 0x940]
14001cfd3: mov qword ptr [rsp + 0x40], rax
14001cfd8: lea rax, [rbp + 0x140]
14001cfdf: mov qword ptr [rsp + 0x38], rax
14001cfe4: mov qword ptr [rsp + 0x30], rbx
14001cfe9: mov qword ptr [rsp + 0x28], rsi
14001cfee: mov qword ptr [rsp + 0x20], r11
14001cff3: mov rcx, rdi
14001cff6: mov rdx, r12
14001cff9: mov r9, r13
14001cffc: call 0x1407dc170 ; system_decorator::decorator_visual::stairs_assemble::gen_stair_floors
14001d001: mov rdx, qword ptr [rbp + 0x6b0]
14001d008: test rdx, rdx
14001d00b: lea r12, [rbp + 0x590]
14001d012: je 0x14001d02a
14001d014: shl rdx, 2
14001d018: mov r8d, 4
14001d01e: mov rcx, qword ptr [rbp + 0x7f0]
14001d025: call 0x140613c20 ; ?
14001d02a: mov rax, qword ptr [rbp + 0x938]
14001d031: mov rsi, qword ptr [rax]
14001d034: mov bl, byte ptr [rbp + 0x806]
14001d03a: mov dil, byte ptr [rbp + 0x807]
14001d041: mov byte ptr [rbp + 0x812], 1
14001d048: mov byte ptr [rbp + 0x811], 1
14001d04f: lea rcx, [rbp + 0x290]
14001d056: call 0x1400284b0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$8<true>
14001d05b: mov dword ptr [rbp + 0x6c0], 0
14001d065: mov dword ptr [rbp + 0x6c8], 0
14001d06f: mov byte ptr [rbp + 0x812], 1
14001d076: mov byte ptr [rbp + 0x811], 1
14001d07d: mov qword ptr [rsp + 0x80], r14
14001d085: mov rcx, qword ptr [rbp + 0x808]
14001d08c: mov qword ptr [rsp + 0x78], rcx
14001d091: lea rcx, [rbp + 0x6c0]
14001d098: mov qword ptr [rsp + 0x70], rcx
14001d09d: and dl, 1
14001d0a0: mov byte ptr [rsp + 0x68], dl
14001d0a4: and al, 1
14001d0a6: mov byte ptr [rsp + 0x60], al
14001d0aa: mov byte ptr [rsp + 0x58], dil
14001d0af: mov byte ptr [rsp + 0x50], bl
14001d0b3: mov eax, dword ptr [rbp + 0x6a8]
14001d0b9: mov byte ptr [rsp + 0x48], al
14001d0bd: mov qword ptr [rsp + 0x38], rsi
14001d0c2: mov eax, dword ptr [rbp + 0x640]
14001d0c8: mov dword ptr [rsp + 0x30], eax
14001d0cc: mov rax, qword ptr [rbp + 0x788]
14001d0d3: mov dword ptr [rsp + 0x28], eax
14001d0d7: lea rax, [rbp + 0x428]
14001d0de: mov qword ptr [rsp + 0x20], rax
14001d0e3: mov word ptr [rsp + 0x40], 0x101
14001d0ea: lea rcx, [rbp + 0x140]
14001d0f1: lea rdx, [rbp + 0x3e0]
14001d0f8: lea r8, [rbp + 0x508]
14001d0ff: lea r9, [rbp + 0x460]
14001d106: call 0x1407f98e0 ; system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings
14001d10b: mov rax, qword ptr [rbp + 0x490]
14001d112: mov qword ptr [rbp + 0x6f0], rax
14001d119: movaps xmm0, xmmword ptr [rbp + 0x460]
14001d120: movaps xmm1, xmmword ptr [rbp + 0x470]
14001d127: movaps xmm2, xmmword ptr [rbp + 0x480]
14001d12e: movaps xmmword ptr [rbp + 0x6e0], xmm2
14001d135: movaps xmmword ptr [rbp + 0x6d0], xmm1
14001d13c: movaps xmmword ptr [rbp + 0x6c0], xmm0
14001d143: mov rdi, qword ptr [rbp + 0x3f0]
14001d14a: test rdi, rdi
14001d14d: je 0x14001d175
14001d14f: lea rsi, [rdi*4]
14001d157: movzx eax, byte ptr [rip + 0x3617aab]
14001d15e: mov edx, 4
14001d163: mov rcx, rsi
14001d166: call 0x140613c40 ; ?
14001d16b: test rax, rax
14001d16e: jne 0x14001d17a
14001d170: jmp 0x140020c64 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001d175: mov eax, 4
14001d17a: mov rcx, qword ptr [rbp + 0x3f0]
14001d181: lea rdx, [rbp + 0x598]
14001d188: mov qword ptr [rdx + 0x58], rcx
14001d18c: movups xmm0, xmmword ptr [rbp + 0x3e0]
14001d193: movups xmmword ptr [rdx + 0x48], xmm0
14001d197: mov rcx, qword ptr [rbp + 0x518]
14001d19e: mov qword ptr [rdx + 0x40], rcx
14001d1a2: movups xmm0, xmmword ptr [rbp + 0x508]
14001d1a9: movups xmmword ptr [rdx + 0x30], xmm0
14001d1ad: mov rcx, qword ptr [rbp + 0x6f0]
14001d1b4: mov qword ptr [rbp + 0x5c0], rcx
14001d1bb: movaps xmm0, xmmword ptr [rbp + 0x6c0]
14001d1c2: movaps xmm1, xmmword ptr [rbp + 0x6d0]
14001d1c9: movaps xmm2, xmmword ptr [rbp + 0x6e0]
14001d1d0: movaps xmmword ptr [rbp + 0x590], xmm0
14001d1d7: movaps xmmword ptr [rbp + 0x5a0], xmm1
14001d1de: movaps xmmword ptr [rbp + 0x5b0], xmm2
14001d1e5: movaps xmmword ptr [rbp + 0x6c0], xmm0
14001d1ec: movaps xmmword ptr [rbp + 0x6d0], xmm1
14001d1f3: movaps xmmword ptr [rbp + 0x6e0], xmm2
14001d1fa: movaps xmm0, xmmword ptr [rbp + 0x5d0]
14001d201: movaps xmm1, xmmword ptr [rbp + 0x5e0]
14001d208: movaps xmmword ptr [rbp + 0x700], xmm0
14001d20f: movaps xmmword ptr [rbp + 0x710], xmm1
14001d216: mov rcx, qword ptr [rbp + 0x5f0]
14001d21d: mov qword ptr [rbp + 0x720], rcx
14001d224: mov rcx, qword ptr [rbp + 0x5c0]
14001d22b: mov qword ptr [rbp + 0x6f0], rcx
14001d232: mov rcx, qword ptr [rbp + 0x5c8]
14001d239: mov qword ptr [rbp + 0x6f8], rcx
14001d240: mov qword ptr [rbp + 0x728], rdi
14001d247: mov qword ptr [rbp + 0x730], rax
14001d24e: mov qword ptr [rbp + 0x738], rdi
14001d255: mov rax, qword ptr [rbp + 0x438]
14001d25c: mov qword ptr [rbp + 0x740], rax
14001d263: mov rax, qword ptr [rbp + 0x670]
14001d26a: mov rax, qword ptr [rax]
14001d26d: mov qword ptr [rbp + 0x748], rax
14001d274: mov rax, qword ptr [rbp + 0x788]
14001d27b: mov dword ptr [rbp + 0x750], eax
14001d281: mov dword ptr [rbp + 0x754], 0x101
14001d28b: movzx eax, byte ptr [rbp + 0x678]
14001d292: mov byte ptr [rbp + 0x758], al
14001d298: movzx eax, byte ptr [rbp + 0x588]
14001d29f: mov byte ptr [rbp + 0x759], al
14001d2a5: mov word ptr [rbp + 0x75a], 0x300
14001d2ae: mov byte ptr [rbp + 0x770], 2
14001d2b5: mov rax, qword ptr [rbp + 0x440]
14001d2bc: mov rsi, qword ptr [rbp + 0x458]
14001d2c3: cmp rsi, rax
14001d2c6: jne 0x14001d2e9
14001d2c8: lea rcx, [rbp + 0x440]
14001d2cf: lea rdx, [rip + 0x29095b2]
14001d2d6: call 0x1405114a0 ; alloc::collections::vec_deque::VecDeque::grow<system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::ArchSegment,alloc::alloc::Global>
14001d2db: mov rax, qword ptr [rbp + 0x440]
14001d2e2: mov rsi, qword ptr [rbp + 0x458]
14001d2e9: mov rcx, qword ptr [rbp + 0x450]
14001d2f0: add rcx, rsi
14001d2f3: cmp rcx, rax
14001d2f6: mov edx, 0
14001d2fb: cmovae rdx, rax
14001d2ff: sub rcx, rdx
14001d302: imul rcx, rcx, 0xb8
14001d309: add rcx, qword ptr [rbp + 0x448]
14001d310: mov r8d, 0xb8
14001d316: lea rdx, [rbp + 0x6c0]
14001d31d: call 0x142923c10 ; ?
14001d322: inc rsi
14001d325: mov qword ptr [rbp + 0x458], rsi
14001d32c: mov rdx, qword ptr [rbp + 0x7a0]
14001d333: test rdx, rdx
14001d336: je 0x14001d34e
14001d338: shl rdx, 2
14001d33c: mov r8d, 4
14001d342: mov rcx, qword ptr [rbp + 0x808]
14001d349: call 0x140613c20 ; ?
14001d34e: mov r14, qword ptr [rbp + 0x7e0]
14001d355: mov r8, qword ptr [rbp + 0x560]
14001d35c: mov rdi, qword ptr [rbp + 0x698]
14001d363: jmp 0x140018efc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001d368: mov eax, dword ptr [rbp + 0x324]
14001d36e: mov rcx, qword ptr [rbp + 0x110]
14001d375: mov dword ptr [rcx], eax
14001d377: mov rdx, qword ptr [rbp + 0x368]
14001d37e: mov eax, dword ptr [rdx + 0x1c]
14001d381: mov rcx, qword ptr [rdx + 0x10]
14001d385: mov dword ptr [rcx], eax
14001d387: mov rdx, qword ptr [rdx]
14001d38a: mov r9d, dword ptr [r14 + 0x128]
14001d391: mov eax, dword ptr [r14 + 0x12c]
14001d398: mov byte ptr [rbp + 0x817], 1
14001d39f: mov dword ptr [rsp + 0x20], eax
14001d3a3: lea rcx, [rbp + 0x6c0]
14001d3aa: mov r8, qword ptr [rbp + 0xa8]
14001d3b1: call 0x140252ad0 ; country_core::systems::collision::world::RaycastWorld::remove_collider<country_core::resources::stairs::stairs_state::StairGraphEdge>
14001d3b6: cmp byte ptr [rbp + 0x6e0], 0x11
14001d3bd: je 0x14001d3df
14001d3bf: mov rax, qword ptr [rbp + 0x6f8]
14001d3c6: lock dec qword ptr [rax]
14001d3ca: jne 0x14001d3df
14001d3cc: mov byte ptr [rbp + 0x817], 1
14001d3d3: lea rcx, [rbp + 0x6f8]
14001d3da: call 0x140b5c620 ; alloc::sync::Arc::drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
14001d3df: mov rdx, qword ptr [rbp + 0x368]
14001d3e6: mov eax, dword ptr [rdx + 0x1c]
14001d3e9: mov rcx, qword ptr [rdx + 0x10]
14001d3ed: mov dword ptr [rcx], eax
14001d3ef: mov rdx, qword ptr [rdx]
14001d3f2: mov rax, qword ptr [rbp + 0x7e0]
14001d3f9: mov r9d, dword ptr [rax + 0x128]
14001d400: mov eax, dword ptr [rax + 0x12c]
14001d406: mov byte ptr [rbp + 0x817], 1
14001d40d: mov dword ptr [rsp + 0x20], eax
14001d411: lea rcx, [rbp + 0x6c0]
14001d418: mov r8, qword ptr [rbp + 0xb0]
14001d41f: call 0x140252ad0 ; country_core::systems::collision::world::RaycastWorld::remove_collider<country_core::resources::stairs::stairs_state::StairGraphEdge>
14001d424: cmp byte ptr [rbp + 0x6e0], 0x11
14001d42b: je 0x14001d44d
14001d42d: mov rax, qword ptr [rbp + 0x6f8]
14001d434: lock dec qword ptr [rax]
14001d438: jne 0x14001d44d
14001d43a: mov byte ptr [rbp + 0x817], 1
14001d441: lea rcx, [rbp + 0x6f8]
14001d448: call 0x140b5c620 ; alloc::sync::Arc::drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
14001d44d: cmp qword ptr [rbp + 0x288], 0
14001d455: je 0x14001d55e
14001d45b: mov rax, qword ptr [rbp + 0x288]
14001d462: mov qword ptr [rbp + 0x470], rax
14001d469: movups xmm0, xmmword ptr [rbp + 0x278]
14001d470: movaps xmmword ptr [rbp + 0x460], xmm0
14001d477: movaps xmm0, xmmword ptr [rip + 0x2909152]
14001d47e: movaps xmmword ptr [rbp + 0x5a0], xmm0
14001d485: movaps xmm0, xmmword ptr [rip + 0x2909134]
14001d48c: movaps xmmword ptr [rbp + 0x590], xmm0
14001d493: mov byte ptr [rbp + 0x817], 0
14001d49a: lea rcx, [rbp + 0x6d0]
14001d4a1: lea rdx, [rbp + 0x460]
14001d4a8: lea r8, [rbp + 0x590]
14001d4af: call 0x1408d0230 ; country_core::systems::collision::world::RaycastMeshData::new_compound
14001d4b4: mov qword ptr [rbp + 0x6c0], 1
14001d4bf: mov qword ptr [rbp + 0x6c8], 1
14001d4ca: movzx eax, byte ptr [rip + 0x3617738]
14001d4d1: mov ecx, 0x90
14001d4d6: mov edx, 0x10
14001d4db: call 0x140613c10 ; ?
14001d4e0: test rax, rax
14001d4e3: je 0x140020b34
14001d4e9: mov rsi, rax
14001d4ec: mov r8d, 0x90
14001d4f2: mov rcx, rax
14001d4f5: lea rdi, [rbp + 0x6c0]
14001d4fc: mov rdx, rdi
14001d4ff: call 0x142923c00 ; ?
14001d504: mov rax, qword ptr [rbp + 0x670]
14001d50b: mov rax, qword ptr [rax]
14001d50e: mov qword ptr [rbp + 0x6c4], rax
14001d515: mov byte ptr [rbp + 0x6c0], 0xd
14001d51c: mov byte ptr [rbp + 0x817], 0
14001d523: lea rcx, [rbp + 0x508]
14001d52a: mov rdx, rsi
14001d52d: mov r8, rdi
14001d530: call 0x1408ceb10 ; country_core::systems::collision::world::RaycastMesh::new
14001d535: xor esi, esi
14001d537: lea r12, [rbp + 0x590]
14001d53e: mov rbx, qword ptr [rbp + 0x7e0]
14001d545: cmp byte ptr [rbp + 0x508], 0x11
14001d54c: lea rdi, [rbp + 0x508]
14001d553: cmove rdi, rsi
14001d557: jne 0x14001d5b6
14001d559: jmp 0x14001dff8 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001d55e: mov rdx, qword ptr [rbp + 0x278]
14001d565: mov rcx, qword ptr [rbp + 0x280]
14001d56c: mov byte ptr [rbp + 0x508], 0x11
14001d573: test rdx, rdx
14001d576: mov esi, 0
14001d57b: lea r12, [rbp + 0x590]
14001d582: mov rbx, qword ptr [rbp + 0x7e0]
14001d589: je 0x14001dff8
14001d58f: shl rdx, 7
14001d593: mov r8d, 0x10
14001d599: call 0x140613c20 ; ?
14001d59e: cmp byte ptr [rbp + 0x508], 0x11
14001d5a5: lea rdi, [rbp + 0x508]
14001d5ac: cmove rdi, rsi
14001d5b0: je 0x14001dff8
14001d5b6: mov byte ptr [rbp + 0x805], 1
14001d5bd: xor ecx, ecx
14001d5bf: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
14001d5c4: test rax, rax
14001d5c7: je 0x140020b48
14001d5cd: movups xmm0, xmmword ptr [rax]
14001d5d0: inc qword ptr [rax]
14001d5d3: movaps xmmword ptr [rbp + 0x480], xmm0
14001d5da: movups xmm0, xmmword ptr [rip + 0x2992a3f]
14001d5e1: movaps xmmword ptr [rbp + 0x460], xmm0
14001d5e8: movups xmm0, xmmword ptr [rip + 0x2992a41]
14001d5ef: movaps xmmword ptr [rbp + 0x470], xmm0
14001d5f6: mov rax, qword ptr [rbp + 0x368]
14001d5fd: mov rbx, qword ptr [rax]
14001d600: movsd xmm0, qword ptr [rip + 0x2908358]
14001d608: movsd qword ptr [rbp + 0x6c0], xmm0
14001d610: mov dword ptr [rbp + 0x6c8], 0
14001d61a: mov byte ptr [rbp + 0x7ef], 1
14001d621: mov rcx, r12
14001d624: lea rdx, [rbp + 0x6c0]
14001d62b: call 0x140c91b30 ; utils::IsoTransform::from_translation
14001d630: mov byte ptr [rbp + 0x7ef], 1
14001d637: lea rcx, [rbp + 0x508]
14001d63e: call 0x1408ceb30 ; country_core::systems::collision::world::RaycastMesh::data
14001d643: movaps xmm0, xmmword ptr [rbp + 0x590]
14001d64a: movaps xmm1, xmmword ptr [rbp + 0x5a0]
14001d651: movaps xmmword ptr [rbp + 0x6d0], xmm1
14001d658: movaps xmmword ptr [rbp + 0x6c0], xmm0
14001d65f: mov qword ptr [rbp + 0x6e0], rax
14001d666: mov byte ptr [rbp + 0x7ef], 1
14001d66d: lea rax, [rbp + 0x460]
14001d674: mov qword ptr [rsp + 0x20], rax
14001d679: mov rcx, rbx
14001d67c: lea rdx, [rbp + 0x6c0]
14001d683: mov r8d, 2
14001d689: xor r9d, r9d
14001d68c: call 0x140254aa0 ; country_core::systems::collision::world::RaycastWorld::iter_shape_overlaps<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<false> >
14001d691: mov r14, qword ptr [rbp + 0x460]
14001d698: mov rcx, qword ptr [rbp + 0x468]
14001d69f: test rcx, rcx
14001d6a2: mov qword ptr [rbp + 0x6a0], rcx
14001d6a9: je 0x14001d6e0
14001d6ab: lea rax, [rcx*8 + 0x17]
14001d6b3: and rax, 0xfffffffffffffff0
14001d6b7: add rcx, rax
14001d6ba: add rcx, 0x11
14001d6be: mov qword ptr [rbp + 0x780], rcx
14001d6c5: mov rcx, r14
14001d6c8: sub rcx, rax
14001d6cb: mov qword ptr [rbp + 0x538], rcx
14001d6d2: mov eax, 0x10
14001d6d7: mov qword ptr [rbp + 0x648], rax
14001d6de: jmp 0x14001d6eb ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001d6e0: mov qword ptr [rbp + 0x648], 0
14001d6eb: mov rax, qword ptr [rbp + 0x478]
14001d6f2: mov qword ptr [rbp + 0x560], rax
14001d6f9: movdqa xmm0, xmmword ptr [r14]
14001d6fe: lea r13, [r14 + 0x10]
14001d702: pmovmskb eax, xmm0
14001d706: not eax
14001d708: jmp 0x14001d716 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001d70a: nop word ptr [rax + rax]
14001d710: mov eax, dword ptr [rbp + 0x698]
14001d716: cmp qword ptr [rbp + 0x560], 0
14001d71e: je 0x14001dfc1
14001d724: test ax, ax
14001d727: je 0x14001d740
14001d729: lea ecx, [rax - 1]
14001d72c: and ecx, eax
14001d72e: mov dword ptr [rbp + 0x698], ecx
14001d734: jmp 0x14001d76c ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001d736: nop word ptr cs:[rax + rax]
14001d740: movdqa xmm0, xmmword ptr [r13]
14001d746: pmovmskb eax, xmm0
14001d74a: add r14, -0x80
14001d74e: add r13, 0x10
14001d752: cmp eax, 0xffff
14001d757: je 0x14001d740
14001d759: mov ecx, 0xfffffffe
14001d75e: sub ecx, eax
14001d760: not eax
14001d762: mov edx, eax
14001d764: and edx, ecx
14001d766: mov dword ptr [rbp + 0x698], edx
14001d76c: tzcnt eax, eax
14001d770: shl eax, 3
14001d773: mov rcx, r14
14001d776: sub rcx, rax
14001d779: mov rdx, qword ptr [rcx - 8]
14001d77d: mov rcx, qword ptr [rbp + 0xe8]
14001d784: mov qword ptr [rbp + 0x790], rdx
14001d78b: lea r8, [rip + 0x2908dfe]
14001d792: call 0x140a91ef0 ; country_core::resources::walls::public_walls::PublicWalls::get
14001d797: mov r15, rax
14001d79a: dec qword ptr [rbp + 0x560]
14001d7a1: mov eax, dword ptr [rbp + 0x698]
14001d7a7: test r15, r15
14001d7aa: je 0x14001d716
14001d7b0: mov rcx, qword ptr [rbp + 0xe8]
14001d7b7: mov rdx, qword ptr [rbp + 0x790]
14001d7be: call 0x140a922b0 ; country_core::resources::walls::public_walls::PublicWalls::roof_entity
14001d7c3: mov rcx, rax
14001d7c6: shr rcx, 0x20
14001d7ca: je 0x14001d8c0
14001d7d0: mov eax, eax
14001d7d2: mov rdx, qword ptr [rbp + 0x250]
14001d7d9: cmp rax, qword ptr [rdx + 0x10]
14001d7dd: jae 0x14001d8c0
14001d7e3: mov rdx, qword ptr [rbp + 0x250]
14001d7ea: mov rdx, qword ptr [rdx + 8]
14001d7ee: lea rax, [rax + rax*4]
14001d7f2: cmp dword ptr [rdx + rax*4], ecx
14001d7f5: jne 0x14001d8c0
14001d7fb: lea rax, [rdx + rax*4]
14001d7ff: mov ecx, dword ptr [rax + 4]
14001d802: mov edx, 0xffffffff
14001d807: cmp rcx, rdx
14001d80a: je 0x14001d8c0
14001d810: mov rdx, qword ptr [rbp + 0x260]
14001d817: cmp qword ptr [rdx + 0x38], rcx
14001d81b: jbe 0x14001d8c0
14001d821: mov rdx, qword ptr [rbp + 0x260]
14001d828: mov rdx, qword ptr [rdx + 0x28]
14001d82c: mov r8d, ecx
14001d82f: shr r8d, 6
14001d833: mov rdx, qword ptr [rdx + r8*8]
14001d837: bt rdx, rcx
14001d83b: jae 0x14001d8c0
14001d841: mov ecx, dword ptr [rax + 0xc]
14001d844: mov rdx, qword ptr [rbp + 0x250]
14001d84b: mov rdx, qword ptr [rdx + 0x1a8]
14001d852: lea r8, [rcx + rcx*8]
14001d856: mov rcx, qword ptr [rbp + 0x260]
14001d85d: mov rcx, qword ptr [rcx + 0x110]
14001d864: cmp qword ptr [rdx + r8*8 + 0x40], rcx
14001d869: jbe 0x14001d892
14001d86b: lea rdx, [rdx + r8*8]
14001d86f: mov r8, qword ptr [rdx + 0x38]
14001d873: mov rcx, qword ptr [r8 + rcx*8]
14001d877: test rcx, rcx
14001d87a: je 0x14001d892
14001d87c: mov rdx, qword ptr [rdx + 0x18]
14001d880: not rcx
14001d883: lea rcx, [rcx + rcx*2]
14001d887: shl rcx, 4
14001d88b: mov rcx, qword ptr [rdx + rcx + 0x10]
14001d890: jmp 0x14001d894 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001d892: xor ecx, ecx
14001d894: mov eax, dword ptr [rax + 0x10]
14001d897: imul rax, rax, 0x58
14001d89b: mov edx, dword ptr [rcx + rax + 0x40]
14001d89f: xor esi, esi
14001d8a1: cmp dl, 3
14001d8a4: cmove edx, esi
14001d8a7: cmp dl, 2
14001d8aa: je 0x14001dfb5
14001d8b0: movzx eax, dl
14001d8b3: cmp eax, 1
14001d8b6: je 0x14001d8ce
14001d8b8: nop dword ptr [rax + rax]
14001d8c0: lea rcx, [r15 + 0x299]
14001d8c7: call 0x1408c5670 ; country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered
14001d8cc: mov esi, eax
14001d8ce: mov rcx, r15
14001d8d1: call 0x140b2a600 ; country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape
14001d8d6: mov r12, rax
14001d8d9: mov eax, dword ptr [rbp + 0x698]
14001d8df: test r12, r12
14001d8e2: je 0x14001d716
14001d8e8: test byte ptr [r12], 1
14001d8ed: je 0x14001d979
14001d8f3: mov rcx, r15
14001d8f6: call 0x140b2a6e0 ; country_core::resources::walls::public_wall_state::PublicWallState::flat_roof_y
14001d8fb: lea rdx, [r12 + 4]
14001d900: addss xmm0, dword ptr [rip + 0x2907ff8]
14001d908: lea rcx, [rbp + 0x6c0]
14001d90f: movaps xmm2, xmm0
14001d912: call 0x140c98b90 ; utils::geometry::rectangle::Rectangle2d::as_points3
14001d917: movzx eax, byte ptr [rip + 0x36172eb]
14001d91e: mov ecx, 0x30
14001d923: mov edx, 4
14001d928: call 0x140613c10 ; ?
14001d92d: test rax, rax
14001d930: je 0x1400209d3
14001d936: movups xmm0, xmmword ptr [rbp + 0x6c0]
14001d93d: movups xmm1, xmmword ptr [rbp + 0x6d0]
14001d944: movups xmm2, xmmword ptr [rbp + 0x6e0]
14001d94b: movups xmmword ptr [rax + 0x20], xmm2
14001d94f: movups xmmword ptr [rax + 0x10], xmm1
14001d953: mov qword ptr [rbp + 0x7a0], rax
14001d95a: movups xmmword ptr [rax], xmm0
14001d95d: lea rbx, [r12 + 0xc]
14001d962: add r12, 0x10
14001d966: mov eax, 4
14001d96b: mov qword ptr [rbp + 0x808], rax
14001d972: mov ecx, 4
14001d977: jmp 0x14001d9dd ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001d979: mov rcx, r15
14001d97c: call 0x140b2a6e0 ; country_core::resources::walls::public_wall_state::PublicWallState::flat_roof_y
14001d981: lea rbx, [r12 + 4]
14001d986: addss xmm0, dword ptr [rip + 0x2907f72]
14001d98e: lea rcx, [rbp + 0x6c0]
14001d995: mov rdx, rbx
14001d998: movss xmm2, dword ptr [rip + 0x2907fd0]
14001d9a0: movaps xmm3, xmm0
14001d9a3: call 0x140c97c50 ; utils::geometry::circle::Circle2d::as_points3
14001d9a8: lea rcx, [rbp + 0x50]
14001d9ac: lea rdx, [rbp + 0x6c0]
14001d9b3: lea r8, [rip + 0x29c06ce]
14001d9ba: call 0x1402bd6b0 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> >
14001d9bf: add r12, 8
14001d9c3: mov rax, qword ptr [rbp + 0x50]
14001d9c7: mov qword ptr [rbp + 0x808], rax
14001d9ce: mov rax, qword ptr [rbp + 0x58]
14001d9d2: mov qword ptr [rbp + 0x7a0], rax
14001d9d9: mov rcx, qword ptr [rbp + 0x60]
14001d9dd: mov dword ptr [rbp + 0x6b0], esi
14001d9e3: movss xmm0, dword ptr [r12]
14001d9e9: movss xmm1, dword ptr [rbx]
14001d9ed: movss dword ptr [rbp + 0x680], xmm1
14001d9f5: movss dword ptr [rbp + 0x684], xmm0
14001d9fd: lea rax, [rcx*4]
14001da05: lea rbx, [rax + rax*2]
14001da09: mov qword ptr [rbp + 0x7f0], rcx
14001da10: test rcx, rcx
14001da13: je 0x14001da37
14001da15: movzx eax, byte ptr [rip + 0x36171ed]
14001da1c: mov edx, 4
14001da21: mov rcx, rbx
14001da24: call 0x140613c10 ; ?
14001da29: test rax, rax
14001da2c: je 0x1400209a1
14001da32: mov rsi, rax
14001da35: jmp 0x14001da3c ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001da37: mov esi, 4
14001da3c: mov r12, qword ptr [rbp + 0x7a0]
14001da43: lea rax, [r12 + rbx]
14001da47: mov qword ptr [rbp + 0x7d0], rax
14001da4e: mov rcx, rsi
14001da51: mov rdx, r12
14001da54: mov r8, rbx
14001da57: call 0x142923c00 ; ?
14001da5c: add rbx, rsi
14001da5f: mov rax, qword ptr [rbp + 0x7f0]
14001da66: mov qword ptr [rbp + 0x6c0], rax
14001da6d: mov qword ptr [rbp + 0x6c8], rsi
14001da74: mov qword ptr [rbp + 0x6d0], rsi
14001da7b: mov qword ptr [rbp + 0x6d8], rax
14001da82: mov qword ptr [rbp + 0x6e0], rbx
14001da89: mov qword ptr [rbp + 0x6e8], r12
14001da90: mov qword ptr [rbp + 0x6f0], r12
14001da97: mov rax, qword ptr [rbp + 0x808]
14001da9e: mov qword ptr [rbp + 0x6f8], rax
14001daa5: mov rax, qword ptr [rbp + 0x7d0]
14001daac: mov qword ptr [rbp + 0x700], rax
14001dab3: mov dword ptr [rbp + 0x708], 0
14001dabd: movss xmm0, dword ptr [rip + 0x2907eaf]
14001dac5: movss dword ptr [rbp + 0x558], xmm0
14001dacd: test byte ptr [rbp + 0x6b0], 1
14001dad4: jne 0x14001dae6
14001dad6: movss xmm0, dword ptr [rip + 0x2907e9a]
14001dade: movss dword ptr [rbp + 0x558], xmm0
14001dae6: xorps xmm0, xmm0
14001dae9: jmp 0x14001db44 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001daeb: nop dword ptr [rax + rax]
14001daf0: movss xmm0, dword ptr [rbp + 0x7d8]
14001daf8: subss xmm0, xmm7
14001dafc: mov rax, qword ptr [r12 + 8]
14001db01: lea rcx, [rsi + rsi*2]
14001db05: mov rdx, qword ptr [rbp + 0x790]
14001db0c: mov qword ptr [rax + rcx*8], rdx
14001db10: movss dword ptr [rax + rcx*8 + 8], xmm12
14001db17: movss dword ptr [rax + rcx*8 + 0xc], xmm11
14001db1e: movss dword ptr [rax + rcx*8 + 0x10], xmm0
14001db24: mov dword ptr [rax + rcx*8 + 0x14], 0x41200000
14001db2c: inc rsi
14001db2f: mov qword ptr [r12 + 0x10], rsi
14001db34: movss xmm0, dword ptr [rbp + 0x7f0]
14001db3c: addss xmm0, dword ptr [rbp + 0x7d0]
14001db44: movss dword ptr [rbp + 0x7f0], xmm0
14001db4c: lea rcx, [rbp + 0x380]
14001db53: lea rdx, [rbp + 0x6c0]
14001db5a: call 0x1404c2650 ; itertools::tuple_impl::impl$10::next<alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >
14001db5f: test byte ptr [rbp + 0x380], 1
14001db66: je 0x14001df60
14001db6c: movss xmm0, dword ptr [rbp + 0x38c]
14001db74: movss dword ptr [rbp + 0x808], xmm0
14001db7c: movsd xmm0, qword ptr [rbp + 0x384]
14001db84: movaps xmmword ptr [rbp + 0x7a0], xmm0
14001db8b: movsd xmm0, qword ptr [rbp + 0x390]
14001db93: movaps xmmword ptr [rbp + 0x6b0], xmm0
14001db9a: movss xmm8, dword ptr [rbp + 0x398]
14001dba3: lea rcx, [rbp + 0x590]
14001dbaa: lea rdx, [rbp + 0x680]
14001dbb1: call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
14001dbb6: movss xmm11, dword ptr [rbp + 0x808]
14001dbbf: addss xmm11, xmm8
14001dbc4: mulss xmm11, dword ptr [rip + 0x2907ca3]
14001dbcd: subss xmm11, dword ptr [rbp + 0x598]
14001dbd6: movaps xmm1, xmm11
14001dbda: mulss xmm1, xmm11
14001dbdf: movaps xmm0, xmmword ptr [rbp + 0x7a0]
14001dbe6: addps xmm0, xmmword ptr [rbp + 0x6b0]
14001dbed: mulps xmm0, xmmword ptr [rip + 0x2907cec]
14001dbf4: movsd xmm2, qword ptr [rbp + 0x590]
14001dbfc: subps xmm0, xmm2
14001dbff: movaps xmm2, xmm0
14001dc02: mulps xmm2, xmm0
14001dc05: movshdup xmm3, xmm2
14001dc09: addss xmm3, xmm2
14001dc0d: addss xmm3, xmm1
14001dc11: xorps xmm2, xmm2
14001dc14: sqrtss xmm2, xmm3
14001dc18: movss xmm1, dword ptr [rip + 0x2907ce4]
14001dc20: divss xmm1, xmm2
14001dc24: movd eax, xmm1
14001dc28: test eax, eax
14001dc2a: setns cl
14001dc2d: mov edx, eax
14001dc2f: and edx, 0x7fffffff
14001dc35: add edx, 0xff800000
14001dc3b: cmp edx, 0x7f000000
14001dc41: setb dl
14001dc44: and dl, cl
14001dc46: dec eax
14001dc48: cmp eax, 0x7fffff
14001dc4d: setb al
14001dc50: or al, dl
14001dc52: jne 0x14001dc60
14001dc54: xorps xmm11, xmm11
14001dc58: jmp 0x14001dc65 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001dc5a: nop word ptr [rax + rax]
14001dc60: mulss xmm11, xmm1
14001dc65: lea rcx, [rbp + 0x590]
14001dc6c: movaps xmm4, xmmword ptr [rbp + 0x6b0]
14001dc73: movaps xmm9, xmm4
14001dc77: movaps xmm12, xmmword ptr [rbp + 0x7a0]
14001dc7f: subps xmm9, xmm12
14001dc83: movaps xmm6, xmm8
14001dc87: movss xmm10, dword ptr [rbp + 0x808]
14001dc90: subss xmm6, xmm10
14001dc95: movaps xmm2, xmm9
14001dc99: mulps xmm2, xmm9
14001dc9d: movshdup xmm3, xmm2
14001dca1: addss xmm3, xmm2
14001dca5: movaps xmm2, xmm6
14001dca8: mulss xmm2, xmm6
14001dcac: addss xmm2, xmm3
14001dcb0: sqrtss xmm2, xmm2
14001dcb4: movss dword ptr [rbp + 0x7d0], xmm2
14001dcbc: movsldup xmm1, xmm1
14001dcc0: mulps xmm0, xmm1
14001dcc3: movzx eax, al
14001dcc6: movd xmm1, eax
14001dcca: pshufd xmm14, xmm1, 0x50
14001dcd0: pslld xmm14, 0x1f
14001dcd6: psrad xmm14, 0x1f
14001dcdc: pand xmm14, xmm0
14001dce1: mov qword ptr [rbp + 0x298], 2
14001dcec: movabs rax, 0x3e99999a00000000
14001dcf6: mov qword ptr [rbp + 0x2a0], rax
14001dcfd: subps xmm12, xmm4
14001dd01: subss xmm10, xmm8
14001dd06: xor esi, esi
14001dd08: movss xmm7, dword ptr [rip + 0x2907c70]
14001dd10: movss xmm0, dword ptr [rip + 0x2907c64]
14001dd18: movss dword ptr [rbp + 0x7d8], xmm0
14001dd20: mov rax, rsi
14001dd23: cmp rax, 2
14001dd27: je 0x14001dec0
14001dd2d: nop dword ptr [rax]
14001dd30: lea rsi, [rax + 1]
14001dd34: mov qword ptr [rbp + 0x290], rsi
14001dd3b: movss xmm15, dword ptr [rbp + rax*4 + 0x2a0]
14001dd45: movsldup xmm13, xmm15
14001dd4a: mulps xmm13, xmm14
14001dd4e: mulss xmm15, xmm11
14001dd53: movss xmm0, dword ptr [rbp + 0x808]
14001dd5b: addss xmm0, xmm15
14001dd60: movaps xmm1, xmmword ptr [rbp + 0x7a0]
14001dd67: addps xmm1, xmm13
14001dd6b: movlps qword ptr [rbp + 0x3e0], xmm1
14001dd72: movss dword ptr [rbp + 0x3e8], xmm0
14001dd7a: movlps qword ptr [rbp + 0x3ec], xmm9
14001dd82: movss dword ptr [rbp + 0x3f4], xmm6
14001dd8a: mov dword ptr [rsp + 0x20], 0x3f800000
14001dd92: mov rdx, rdi
14001dd95: lea r8, [rip + 0x2908824]
14001dd9c: lea r9, [rbp + 0x3e0]
14001dda3: call 0x1408ceb40 ; country_core::systems::collision::world::RaycastMesh::raycast
14001dda8: cmp dword ptr [rbp + 0x5c4], 2
14001ddaf: je 0x14001dde4
14001ddb1: movss xmm0, dword ptr [rbp + 0x5c0]
14001ddb9: mulss xmm0, dword ptr [rbp + 0x7d0]
14001ddc1: addss xmm0, dword ptr [rbp + 0x7f0]
14001ddc9: movaps xmm1, xmm7
14001ddcc: cmpunordss xmm1, xmm7
14001ddd1: movaps xmm2, xmm1
14001ddd4: andps xmm2, xmm0
14001ddd7: minss xmm0, xmm7
14001dddb: andnps xmm1, xmm0
14001ddde: orps xmm1, xmm2
14001dde1: movaps xmm7, xmm1
14001dde4: lea rcx, [rbp + 0x590]
14001ddeb: addss xmm15, xmm8
14001ddf0: addps xmm13, xmmword ptr [rbp + 0x6b0]
14001ddf8: movlps qword ptr [rbp + 0x3e0], xmm13
14001de00: movss dword ptr [rbp + 0x3e8], xmm15
14001de09: movlps qword ptr [rbp + 0x3ec], xmm12
14001de11: movss dword ptr [rbp + 0x3f4], xmm10
14001de1a: mov dword ptr [rsp + 0x20], 0x3f800000
14001de22: mov rdx, rdi
14001de25: lea r8, [rip + 0x2908794]
14001de2c: lea r9, [rbp + 0x3e0]
14001de33: call 0x1408ceb40 ; country_core::systems::collision::world::RaycastMesh::raycast
14001de38: cmp dword ptr [rbp + 0x5c4], 2
14001de3f: jne 0x14001de57
14001de41: mov rax, rsi
14001de44: lea rcx, [rbp + 0x590]
14001de4b: cmp rax, 2
14001de4f: jne 0x14001dd30
14001de55: jmp 0x14001dec0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001de57: movss xmm0, dword ptr [rip + 0x2907aa5]
14001de5f: subss xmm0, dword ptr [rbp + 0x5c0]
14001de67: mulss xmm0, dword ptr [rbp + 0x7d0]
14001de6f: addss xmm0, dword ptr [rbp + 0x7f0]
14001de77: movss xmm3, dword ptr [rbp + 0x7d8]
14001de7f: movaps xmm1, xmm3
14001de82: cmpunordss xmm1, xmm3
14001de87: movaps xmm2, xmm1
14001de8a: andps xmm2, xmm0
14001de8d: maxss xmm0, xmm3
14001de91: andnps xmm1, xmm0
14001de94: orps xmm1, xmm2
14001de97: movss dword ptr [rbp + 0x7d8], xmm1
14001de9f: lea rcx, [rbp + 0x590]
14001dea6: mov rax, rsi
14001dea9: cmp rax, 2
14001dead: jne 0x14001dd30
14001deb3: nop word ptr cs:[rax + rax]
14001dec0: movaps xmm0, xmm7
14001dec3: addss xmm0, dword ptr [rip + 0x2907ab9]
14001decb: movss xmm1, dword ptr [rbp + 0x7d8]
14001ded3: ucomiss xmm1, xmm0
14001ded6: jbe 0x14001db34
14001dedc: mov rcx, r15
14001dedf: call 0x140b2a620 ; country_core::resources::walls::public_wall_state::PublicWallState::max_y
14001dee4: movaps xmm11, xmm0
14001dee8: mov r12, qword ptr [rbp + 0x208]
14001deef: mov rcx, r15
14001def2: call 0x140b2a740 ; country_core::resources::walls::public_wall_state::PublicWallState::wall_space
14001def7: subss xmm11, dword ptr [rbp + 0x558]
14001df00: addss xmm11, dword ptr [rip + 0x2907a7f]
14001df09: movaps xmm1, xmm7
14001df0c: addss xmm1, dword ptr [rbp + 0x7d8]
14001df14: mulss xmm1, dword ptr [rip + 0x2907954]
14001df1c: mov dword ptr [rbp + 0x590], eax
14001df22: movss dword ptr [rbp + 0x594], xmm0
14001df2a: lea rcx, [rbp + 0x590]
14001df31: call 0x140c91eb0 ; utils::wall_space::impl$1::project_clamp
14001df36: movaps xmm12, xmm0
14001df3a: mov rsi, qword ptr [r12 + 0x10]
14001df3f: cmp rsi, qword ptr [r12]
14001df43: jne 0x14001daf0
14001df49: mov rcx, r12
14001df4c: lea rdx, [rip + 0x2908655]
14001df53: call 0x140562db0 ; alloc::raw_vec::RawVec::grow_one<tuple$<ref$<str$>,enum2$<country_core::resources::gui::SettingsMenuState> >,alloc::alloc::Global>
14001df58: jmp 0x14001daf0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001df5d: nop dword ptr [rax]
14001df60: mov rax, qword ptr [rbp + 0x6d8]
14001df67: test rax, rax
14001df6a: je 0x14001df86
14001df6c: mov rcx, qword ptr [rbp + 0x6c8]
14001df73: shl rax, 2
14001df77: lea rdx, [rax + rax*2]
14001df7b: mov r8d, 4
14001df81: call 0x140613c20 ; ?
14001df86: mov rax, qword ptr [rbp + 0x6f8]
14001df8d: test rax, rax
14001df90: je 0x14001d710
14001df96: mov rcx, qword ptr [rbp + 0x6e8]
14001df9d: shl rax, 2
14001dfa1: lea rdx, [rax + rax*2]
14001dfa5: mov r8d, 4
14001dfab: call 0x140613c20 ; ?
14001dfb0: jmp 0x14001d710 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001dfb5: add rcx, rax
14001dfb8: movzx esi, byte ptr [rcx + 0x41]
14001dfbc: jmp 0x14001d8ce ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001dfc1: cmp qword ptr [rbp + 0x6a0], 0
14001dfc9: lea r12, [rbp + 0x590]
14001dfd0: mov rdx, qword ptr [rbp + 0x780]
14001dfd7: je 0x14001dff1
14001dfd9: test rdx, rdx
14001dfdc: je 0x14001dff1
14001dfde: mov rcx, qword ptr [rbp + 0x538]
14001dfe5: mov r8, qword ptr [rbp + 0x648]
14001dfec: call 0x140613c20 ; ?
14001dff1: mov rbx, qword ptr [rbp + 0x7e0]
14001dff8: mov rdx, qword ptr [rbp + 0x368]
14001dfff: mov eax, dword ptr [rdx + 0x1c]
14001e002: mov rcx, qword ptr [rdx + 0x10]
14001e006: mov dword ptr [rcx], eax
14001e008: mov rcx, qword ptr [rdx]
14001e00b: mov eax, dword ptr [rbp + 0x324]
14001e011: mov rdx, qword ptr [rbp + 0x110]
14001e018: mov dword ptr [rdx], eax
14001e01a: movzx eax, byte ptr [rbp + 0x508]
14001e021: mov rdx, qword ptr [rbp + 0x509]
14001e028: mov r8d, dword ptr [rbp + 0x511]
14001e02f: movzx r9d, word ptr [rbp + 0x515]
14001e037: movzx r10d, byte ptr [rbp + 0x517]
14001e03f: movups xmm0, xmmword ptr [rbp + 0x518]
14001e046: movaps xmmword ptr [rbp + 0x6d0], xmm0
14001e04d: mov byte ptr [rbp + 0x6c0], al
14001e053: mov qword ptr [rbp + 0x6c1], rdx
14001e05a: mov dword ptr [rbp + 0x6c9], r8d
14001e061: mov word ptr [rbp + 0x6cd], r9w
14001e069: mov byte ptr [rbp + 0x6cf], r10b
14001e070: mov r9d, dword ptr [rbx + 0x128]
14001e077: mov eax, dword ptr [rbx + 0x12c]
14001e07d: mov byte ptr [rbp + 0x805], 0
14001e084: mov dword ptr [rsp + 0x20], eax
14001e088: mov rdx, qword ptr [rbp + 0xb8]
14001e08f: lea r8, [rbp + 0x6c0]
14001e096: call 0x1407deb60 ; system_decorator::decorator_visual::stairs_assemble::create_stairs_collider
14001e09b: mov rax, qword ptr [rbp + 0x458]
14001e0a2: test rax, rax
14001e0a5: mov r13, qword ptr [rbp + 0x940]
14001e0ac: je 0x14001e10a
14001e0ae: mov rcx, qword ptr [rbp + 0x440]
14001e0b5: mov rdx, qword ptr [rbp + 0x450]
14001e0bc: lea r8, [rdx + 1]
14001e0c0: cmp r8, rcx
14001e0c3: mov r8d, 0
14001e0c9: cmovae r8, rcx
14001e0cd: neg r8
14001e0d0: lea rcx, [rdx + r8]
14001e0d4: inc rcx
14001e0d7: mov qword ptr [rbp + 0x450], rcx
14001e0de: dec rax
14001e0e1: mov qword ptr [rbp + 0x458], rax
14001e0e8: imul rdx, rdx, 0xb8
14001e0ef: add rdx, qword ptr [rbp + 0x448]
14001e0f6: mov r8d, 0xb8
14001e0fc: lea rcx, [rbp + 0x6c0]
14001e103: call 0x142923c00 ; ?
14001e108: jmp 0x14001e140 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001e10a: movabs rax, 0x8000000000000000
14001e114: mov qword ptr [rbp + 0x6c0], rax
14001e11b: jmp 0x14001e140 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001e11d: imul rcx, rsi, 0xa8
14001e124: add rcx, qword ptr [rdi + 8]
14001e128: mov r8d, 0xa8
14001e12e: mov rdx, r12
14001e131: call 0x142923c10 ; ?
14001e136: inc rsi
14001e139: mov qword ptr [rdi + 0x10], rsi
14001e13d: nop dword ptr [rax]
14001e140: mov rcx, qword ptr [rbp + 0x6c0]
14001e147: movabs rax, 0x8000000000000000
14001e151: mov qword ptr [rbp + 0x6c0], rax
14001e158: mov qword ptr [rbp + 0x808], rcx
14001e15f: mov rax, rcx
14001e162: neg rax
14001e165: jo 0x14001f9cb
14001e16b: mov rax, qword ptr [rbp + 0x6c8]
14001e172: mov qword ptr [rbp + 0x6b0], rax
14001e179: mov r14, qword ptr [rbp + 0x6d0]
14001e180: mov r15, qword ptr [rbp + 0x6d8]
14001e187: mov rax, qword ptr [rbp + 0x6e0]
14001e18e: mov qword ptr [rbp + 0x430], rax
14001e195: lea rcx, [rbp + 0x6f8]
14001e19c: movdqu xmm0, xmmword ptr [rcx - 0x10]
14001e1a1: movdqa xmmword ptr [rbp + 0x300], xmm0
14001e1a9: mov r8, qword ptr [rbp + 0x6f8]
14001e1b0: mov rax, qword ptr [rbp + 0x700]
14001e1b7: mov qword ptr [rbp + 0x7f0], rax
14001e1be: mov rbx, qword ptr [rbp + 0x708]
14001e1c5: mov rax, qword ptr [rbp + 0x710]
14001e1cc: mov qword ptr [rbp + 0x7a0], rax
14001e1d3: mov rax, qword ptr [rbp + 0x718]
14001e1da: mov qword ptr [rbp + 0x7d0], rax
14001e1e1: mov rdi, qword ptr [rbp + 0x720]
14001e1e8: mov rdx, qword ptr [rbp + 0x728]
14001e1ef: mov rax, qword ptr [rbp + 0x730]
14001e1f6: mov qword ptr [rbp + 0x7d8], rax
14001e1fd: movzx eax, word ptr [rcx + 0x66]
14001e201: mov word ptr [rbp + 0x56c], ax
14001e208: mov eax, dword ptr [rcx + 0x62]
14001e20b: mov dword ptr [rbp + 0x568], eax
14001e211: mov rsi, qword ptr [rbp + 0x738]
14001e218: movss xmm6, dword ptr [rbp + 0x754]
14001e220: mov rax, qword ptr [rbp + 0x740]
14001e227: mov qword ptr [rbp + 0x678], rax
14001e22e: mov eax, dword ptr [rbp + 0x748]
14001e234: mov dword ptr [rbp + 0x588], eax
14001e23a: mov r10d, dword ptr [rbp + 0x74c]
14001e241: mov r9d, dword ptr [rbp + 0x750]
14001e248: movzx r11d, word ptr [rbp + 0x758]
14001e250: mov rcx, qword ptr [rbp + 0x458]
14001e257: test rcx, rcx
14001e25a: mov qword ptr [rbp + 0x558], r15
14001e261: mov qword ptr [rbp + 0x698], rdx
14001e268: mov qword ptr [rbp + 0x790], r8
14001e26f: je 0x14001ea07
14001e275: mov dword ptr [rbp + 0x640], r10d
14001e27c: mov dword ptr [rbp + 0x528], r9d
14001e283: mov rdx, qword ptr [rbp + 0x450]
14001e28a: mov r8, qword ptr [rbp + 0x440]
14001e291: mov rax, qword ptr [rbp + 0x448]
14001e298: cmp rdx, r8
14001e29b: mov r9d, 0
14001e2a1: cmovae r9, r8
14001e2a5: mov r10, rdx
14001e2a8: sub r10, r9
14001e2ab: imul r9, r10, 0xb8
14001e2b2: cmp byte ptr [rax + r9 + 0xb0], 2
14001e2bb: je 0x14001e96e
14001e2c1: cmp byte ptr [rbp + 0x770], 2
14001e2c8: je 0x14001e96e
14001e2ce: mov r10, qword ptr [rbp + 0x768]
14001e2d5: add r9, rax
14001e2d8: cmp r10, qword ptr [r9 + 0xa0]
14001e2df: jne 0x14001e96e
14001e2e5: mov r10, qword ptr [rbp + 0x760]
14001e2ec: mov qword ptr [rbp + 0x500], r10
14001e2f3: mov r9, qword ptr [r9 + 0xa8]
14001e2fa: mov qword ptr [rbp + 0x540], r9
14001e301: lea r9, [rdx + 1]
14001e305: cmp r9, r8
14001e308: mov r9d, 0
14001e30e: cmovae r9, r8
14001e312: neg r9
14001e315: lea r8, [rdx + r9]
14001e319: inc r8
14001e31c: mov qword ptr [rbp + 0x450], r8
14001e323: dec rcx
14001e326: mov qword ptr [rbp + 0x458], rcx
14001e32d: imul r9, rdx, 0xb8
14001e334: mov rcx, qword ptr [rax + r9]
14001e338: mov rdx, rcx
14001e33b: neg rdx
14001e33e: jo 0x1400207e1
14001e344: mov word ptr [rbp + 0x548], r11w
14001e34c: mov rdx, qword ptr [rax + r9 + 8]
14001e351: mov r8, qword ptr [rax + r9 + 0x10]
14001e356: mov r10, qword ptr [rax + r9 + 0x18]
14001e35b: mov qword ptr [rbp + 0x648], r10
14001e362: mov r10, qword ptr [rax + r9 + 0x20]
14001e367: mov qword ptr [rbp + 0x6a8], r10
14001e36e: mov r10, qword ptr [rax + r9 + 0x38]
14001e373: mov qword ptr [rbp + 0x560], r10
14001e37a: mov r10, qword ptr [rax + r9 + 0x40]
14001e37f: mov qword ptr [rbp + 0x6a0], r10
14001e386: mov r13, qword ptr [rax + r9 + 0x48]
14001e38b: mov r10, qword ptr [rax + r9 + 0x50]
14001e390: mov qword ptr [rbp + 0x7e0], r10
14001e397: mov r10, qword ptr [rax + r9 + 0x58]
14001e39c: mov qword ptr [rbp + 0x780], r10
14001e3a3: mov r12, qword ptr [rax + r9 + 0x60]
14001e3a8: mov r10, qword ptr [rax + r9 + 0x68]
14001e3ad: mov qword ptr [rbp + 0x538], r10
14001e3b4: mov r10, qword ptr [rax + r9 + 0x70]
14001e3b9: mov qword ptr [rbp + 0x580], r10
14001e3c0: mov r15, qword ptr [rax + r9 + 0x78]
14001e3c5: mov rax, qword ptr [rbp + 0x808]
14001e3cc: mov qword ptr [rbp + 0x650], rax
14001e3d3: mov rax, qword ptr [rbp + 0x6b0]
14001e3da: mov qword ptr [rbp + 0x658], rax
14001e3e1: mov qword ptr [rbp + 0x660], r14
14001e3e8: test r14, r14
14001e3eb: je 0x14001e3f7
14001e3ed: dec r14
14001e3f0: mov qword ptr [rbp + 0x660], r14
14001e3f7: lea rax, [r8 + r8*2]
14001e3fb: lea rax, [rdx + rax*4]
14001e3ff: mov qword ptr [rbp + 0x590], rdx
14001e406: mov qword ptr [rbp + 0x598], rdx
14001e40d: mov qword ptr [rbp + 0x5a0], rcx
14001e414: mov qword ptr [rbp + 0x5a8], rax
14001e41b: mov qword ptr [rbp + 0x5b0], 1
14001e426: mov byte ptr [rbp + 0x7cd], 1
14001e42d: mov byte ptr [rbp + 0x7cc], 1
14001e434: mov byte ptr [rbp + 0x7cb], 1
14001e43b: mov byte ptr [rbp + 0x7ca], 1
14001e442: lea rcx, [rbp + 0x650]
14001e449: lea rdx, [rbp + 0x590]
14001e450: lea r8, [rip + 0x29081a1]
14001e457: call 0x1402b9e00 ; alloc::vec::spec_extend::impl$0::spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::skip::Skip<alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global> >,alloc::alloc::Global>
14001e45c: mov rax, qword ptr [rbp + 0x790]
14001e463: mov qword ptr [rbp + 0x680], rax
14001e46a: mov rax, qword ptr [rbp + 0x7f0]
14001e471: mov qword ptr [rbp + 0x688], rax
14001e478: mov qword ptr [rbp + 0x690], rbx
14001e47f: test rbx, rbx
14001e482: mov r14d, 0
14001e488: je 0x14001e494
14001e48a: dec rbx
14001e48d: mov qword ptr [rbp + 0x690], rbx
14001e494: mov rcx, qword ptr [rbp + 0x6a0]
14001e49b: lea rax, [rcx + r13*8]
14001e49f: mov qword ptr [rbp + 0x590], rcx
14001e4a6: mov qword ptr [rbp + 0x598], rcx
14001e4ad: mov rcx, qword ptr [rbp + 0x560]
14001e4b4: mov qword ptr [rbp + 0x5a0], rcx
14001e4bb: mov qword ptr [rbp + 0x5a8], rax
14001e4c2: mov qword ptr [rbp + 0x5b0], 1
14001e4cd: mov byte ptr [rbp + 0x7c9], 1
14001e4d4: mov byte ptr [rbp + 0x7c8], 1
14001e4db: mov byte ptr [rbp + 0x7c7], 1
14001e4e2: mov byte ptr [rbp + 0x7c6], 1
14001e4e9: lea rcx, [rbp + 0x680]
14001e4f0: lea rdx, [rbp + 0x590]
14001e4f7: lea r8, [rip + 0x2908112]
14001e4fe: call 0x1402ba000 ; alloc::vec::spec_extend::impl$0::spec_extend<glam::f32::vec2::Vec2,core::iter::adapters::skip::Skip<alloc::vec::into_iter::IntoIter<glam::f32::vec2::Vec2,alloc::alloc::Global> >,alloc::alloc::Global>
14001e503: mov rax, qword ptr [rbp + 0x7a0]
14001e50a: mov qword ptr [rbp + 0x290], rax
14001e511: mov rax, qword ptr [rbp + 0x7d0]
14001e518: mov qword ptr [rbp + 0x298], rax
14001e51f: mov qword ptr [rbp + 0x2a0], rdi
14001e526: test rdi, rdi
14001e529: mov r13, qword ptr [rbp + 0x940]
14001e530: je 0x14001e53c
14001e532: dec rdi
14001e535: mov qword ptr [rbp + 0x2a0], rdi
14001e53c: mov rcx, qword ptr [rbp + 0x780]
14001e543: lea rax, [rcx + r12*4]
14001e547: mov qword ptr [rbp + 0x590], rcx
14001e54e: mov qword ptr [rbp + 0x598], rcx
14001e555: mov rcx, qword ptr [rbp + 0x7e0]
14001e55c: mov qword ptr [rbp + 0x5a0], rcx
14001e563: mov qword ptr [rbp + 0x5a8], rax
14001e56a: mov qword ptr [rbp + 0x5b0], 1
14001e575: mov byte ptr [rbp + 0x7c5], 1
14001e57c: mov byte ptr [rbp + 0x7c4], 1
14001e583: mov byte ptr [rbp + 0x7c3], 1
14001e58a: lea rcx, [rbp + 0x290]
14001e591: lea r12, [rbp + 0x590]
14001e598: mov rdx, r12
14001e59b: lea r8, [rip + 0x2908086]
14001e5a2: call 0x1402b9c50 ; alloc::vec::spec_extend::impl$0::spec_extend<f32,core::iter::adapters::skip::Skip<alloc::vec::into_iter::IntoIter<f32,alloc::alloc::Global> >,alloc::alloc::Global>
14001e5a7: mov rax, qword ptr [rbp + 0x698]
14001e5ae: mov qword ptr [rbp + 0x3e0], rax
14001e5b5: mov rax, qword ptr [rbp + 0x7d8]
14001e5bc: mov qword ptr [rbp + 0x3e8], rax
14001e5c3: mov qword ptr [rbp + 0x3f0], rsi
14001e5ca: test rsi, rsi
14001e5cd: je 0x14001e5d9
14001e5cf: dec rsi
14001e5d2: mov qword ptr [rbp + 0x3f0], rsi
14001e5d9: mov rcx, qword ptr [rbp + 0x580]
14001e5e0: lea rax, [rcx + r15*4]
14001e5e4: mov qword ptr [rbp + 0x590], rcx
14001e5eb: mov qword ptr [rbp + 0x598], rcx
14001e5f2: mov rcx, qword ptr [rbp + 0x538]
14001e5f9: mov qword ptr [rbp + 0x5a0], rcx
14001e600: mov qword ptr [rbp + 0x5a8], rax
14001e607: mov qword ptr [rbp + 0x5b0], 1
14001e612: mov byte ptr [rbp + 0x7c2], 1
14001e619: mov byte ptr [rbp + 0x7c1], 1
14001e620: lea rcx, [rbp + 0x3e0]
14001e627: mov rdx, r12
14001e62a: lea r8, [rip + 0x290800f]
14001e631: call 0x1402b9c50 ; alloc::vec::spec_extend::impl$0::spec_extend<f32,core::iter::adapters::skip::Skip<alloc::vec::into_iter::IntoIter<f32,alloc::alloc::Global> >,alloc::alloc::Global>
14001e636: mov rax, qword ptr [rbp + 0x660]
14001e63d: mov qword ptr [rbp + 0x5a0], rax
14001e644: movdqu xmm0, xmmword ptr [rbp + 0x650]
14001e64c: movdqa xmmword ptr [rbp + 0x590], xmm0
14001e654: mov byte ptr [rbp + 0x7c2], 1
14001e65b: mov byte ptr [rbp + 0x7c1], 0
14001e662: lea rcx, [rbp + 0x380]
14001e669: mov rdx, r12
14001e66c: mov r8d, 1
14001e672: call 0x140153640 ; utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14001e677: cmp r14, qword ptr [rbp + 0x380]
14001e67e: jno 0x14001e710
14001e684: mov rdx, qword ptr [rbp + 0x3e0]
14001e68b: test rdx, rdx
14001e68e: je 0x14001e6a6
14001e690: mov rcx, qword ptr [rbp + 0x3e8]
14001e697: shl rdx, 2
14001e69b: mov r8d, 4
14001e6a1: call 0x140613c20 ; ?
14001e6a6: mov rdx, qword ptr [rbp + 0x290]
14001e6ad: test rdx, rdx
14001e6b0: mov rsi, qword ptr [rbp + 0x558]
14001e6b7: mov rdi, qword ptr [rbp + 0x648]
14001e6be: je 0x14001e6d6
14001e6c0: mov rcx, qword ptr [rbp + 0x298]
14001e6c7: shl rdx, 2
14001e6cb: mov r8d, 4
14001e6d1: call 0x140613c20 ; ?
14001e6d6: mov rdx, qword ptr [rbp + 0x680]
14001e6dd: test rdx, rdx
14001e6e0: je 0x14001e929
14001e6e6: mov rcx, qword ptr [rbp + 0x688]
14001e6ed: shl rdx, 3
14001e6f1: mov r8d, 4
14001e6f7: call 0x140613c20 ; ?
14001e6fc: jmp 0x14001e929 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001e701: nop word ptr cs:[rax + rax]
14001e710: movaps xmmword ptr [rbp + 0x370], xmm6
14001e717: mov rax, qword ptr [rbp + 0x3b0]
14001e71e: mov qword ptr [rbp + 0x5c0], rax
14001e725: movups xmm0, xmmword ptr [rbp + 0x380]
14001e72c: movups xmm1, xmmword ptr [rbp + 0x390]
14001e733: movups xmm2, xmmword ptr [rbp + 0x3a0]
14001e73a: movaps xmmword ptr [rbp + 0x5b0], xmm2
14001e741: movaps xmmword ptr [rbp + 0x5a0], xmm1
14001e748: movaps xmmword ptr [rbp + 0x590], xmm0
14001e74f: mov rax, qword ptr [rbp + 0x690]
14001e756: lea rcx, [rbp + 0x598]
14001e75d: mov qword ptr [rcx + 0x40], rax
14001e761: movups xmm0, xmmword ptr [rbp + 0x680]
14001e768: movups xmmword ptr [rcx + 0x30], xmm0
14001e76c: mov rax, qword ptr [rbp + 0x2a0]
14001e773: mov qword ptr [rcx + 0x58], rax
14001e777: movups xmm0, xmmword ptr [rbp + 0x290]
14001e77e: movups xmmword ptr [rcx + 0x48], xmm0
14001e782: mov rax, qword ptr [rbp + 0x3f0]
14001e789: mov qword ptr [rcx + 0x70], rax
14001e78d: movups xmm0, xmmword ptr [rbp + 0x3e0]
14001e794: movups xmmword ptr [rcx + 0x60], xmm0
14001e798: movaps xmm0, xmmword ptr [rbp + 0x590]
14001e79f: movaps xmm1, xmmword ptr [rbp + 0x5a0]
14001e7a6: movaps xmm2, xmmword ptr [rbp + 0x5b0]
14001e7ad: movaps xmm3, xmmword ptr [rbp + 0x5c0]
14001e7b4: movaps xmmword ptr [rbp + 0x460], xmm0
14001e7bb: movaps xmmword ptr [rbp + 0x470], xmm1
14001e7c2: movaps xmmword ptr [rbp + 0x480], xmm2
14001e7c9: movaps xmmword ptr [rbp + 0x490], xmm3
14001e7d0: movaps xmm0, xmmword ptr [rbp + 0x5d0]
14001e7d7: movaps xmmword ptr [rbp + 0x4a0], xmm0
14001e7de: movaps xmm0, xmmword ptr [rbp + 0x5e0]
14001e7e5: movaps xmmword ptr [rbp + 0x4b0], xmm0
14001e7ec: movaps xmm0, xmmword ptr [rbp + 0x5f0]
14001e7f3: movaps xmmword ptr [rbp + 0x4c0], xmm0
14001e7fa: movaps xmm0, xmmword ptr [rbp + 0x600]
14001e801: movaps xmmword ptr [rbp + 0x4d0], xmm0
14001e808: cmp r14, qword ptr [rbp + 0x6c0]
14001e80f: jo 0x14001e81d
14001e811: lea rcx, [rbp + 0x6c0]
14001e818: call 0x140609eb0 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams>
14001e81d: movaps xmm0, xmmword ptr [rbp + 0x4d0]
14001e824: movaps xmmword ptr [rbp + 0x730], xmm0
14001e82b: movaps xmm0, xmmword ptr [rbp + 0x4c0]
14001e832: movaps xmmword ptr [rbp + 0x720], xmm0
14001e839: movaps xmm0, xmmword ptr [rbp + 0x4b0]
14001e840: movaps xmmword ptr [rbp + 0x710], xmm0
14001e847: movaps xmm0, xmmword ptr [rbp + 0x4a0]
14001e84e: movaps xmmword ptr [rbp + 0x700], xmm0
14001e855: movaps xmm0, xmmword ptr [rbp + 0x460]
14001e85c: movdqa xmm1, xmmword ptr [rbp + 0x470]
14001e864: movdqa xmm2, xmmword ptr [rbp + 0x480]
14001e86c: movapd xmm3, xmmword ptr [rbp + 0x490]
14001e874: movapd xmmword ptr [rbp + 0x6f0], xmm3
14001e87c: movdqa xmmword ptr [rbp + 0x6e0], xmm2
14001e884: movdqa xmmword ptr [rbp + 0x6d0], xmm1
14001e88c: movaps xmmword ptr [rbp + 0x6c0], xmm0
14001e893: mov rax, qword ptr [rbp + 0x678]
14001e89a: mov qword ptr [rbp + 0x740], rax
14001e8a1: mov eax, dword ptr [rbp + 0x588]
14001e8a7: mov dword ptr [rbp + 0x748], eax
14001e8ad: mov eax, dword ptr [rbp + 0x640]
14001e8b3: mov dword ptr [rbp + 0x74c], eax
14001e8b9: mov eax, dword ptr [rbp + 0x528]
14001e8bf: mov dword ptr [rbp + 0x750], eax
14001e8c5: movdqa xmm0, xmmword ptr [rbp + 0x370]
14001e8cd: movd eax, xmm0
14001e8d1: mov word ptr [rbp + 0x754], ax
14001e8d8: mov word ptr [rbp + 0x756], 0
14001e8e1: movzx eax, word ptr [rbp + 0x548]
14001e8e8: mov word ptr [rbp + 0x758], ax
14001e8ef: mov word ptr [rbp + 0x75a], 0x300
14001e8f8: mov rax, qword ptr [rbp + 0x500]
14001e8ff: mov qword ptr [rbp + 0x760], rax
14001e906: mov rax, qword ptr [rbp + 0x540]
14001e90d: mov qword ptr [rbp + 0x768], rax
14001e914: mov byte ptr [rbp + 0x770], 0
14001e91b: mov rsi, qword ptr [rbp + 0x558]
14001e922: mov rdi, qword ptr [rbp + 0x648]
14001e929: test rdi, rdi
14001e92c: je 0x14001e947
14001e92e: shl rdi, 2
14001e932: mov r8d, 4
14001e938: mov rcx, qword ptr [rbp + 0x6a8]
14001e93f: mov rdx, rdi
14001e942: call 0x140613c20 ; ?
14001e947: test rsi, rsi
14001e94a: je 0x14001e140
14001e950: shl rsi, 2
14001e954: mov r8d, 4
14001e95a: mov rcx, qword ptr [rbp + 0x430]
14001e961: mov rdx, rsi
14001e964: call 0x140613c20 ; ?
14001e969: jmp 0x14001e140 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001e96e: lea r9, [rdx + 1]
14001e972: cmp r9, r8
14001e975: mov r9d, 0
14001e97b: cmovae r9, r8
14001e97f: neg r9
14001e982: lea r8, [rdx + r9]
14001e986: inc r8
14001e989: mov qword ptr [rbp + 0x450], r8
14001e990: dec rcx
14001e993: mov qword ptr [rbp + 0x458], rcx
14001e99a: imul rcx, rdx, 0xb8
14001e9a1: mov r15, qword ptr [rax + rcx]
14001e9a5: lea rdx, [rax + rcx]
14001e9a9: add rdx, 8
14001e9ad: mov r8d, 0xb0
14001e9b3: mov rcx, r12
14001e9b6: mov r13d, r11d
14001e9b9: call 0x142923c00 ; ?
14001e9be: mov qword ptr [rbp + 0x6c0], r15
14001e9c5: mov r15, qword ptr [rbp + 0x558]
14001e9cc: mov r8d, 0xb0
14001e9d2: lea rcx, [rbp + 0x6c8]
14001e9d9: mov rdx, r12
14001e9dc: call 0x142923c00 ; ?
14001e9e1: mov r11d, r13d
14001e9e4: mov r13, qword ptr [rbp + 0x940]
14001e9eb: mov rdx, qword ptr [rbp + 0x698]
14001e9f2: mov r8, qword ptr [rbp + 0x790]
14001e9f9: mov r9d, dword ptr [rbp + 0x528]
14001ea00: mov r10d, dword ptr [rbp + 0x640]
14001ea07: mov rax, qword ptr [rbp + 0x808]
14001ea0e: mov qword ptr [rbp + 0x460], rax
14001ea15: mov rax, qword ptr [rbp + 0x6b0]
14001ea1c: mov qword ptr [rbp + 0x468], rax
14001ea23: mov qword ptr [rbp + 0x470], r14
14001ea2a: mov qword ptr [rbp + 0x478], r15
14001ea31: mov rax, qword ptr [rbp + 0x430]
14001ea38: mov qword ptr [rbp + 0x480], rax
14001ea3f: movaps xmm0, xmmword ptr [rbp + 0x300]
14001ea46: lea rcx, [rbp + 0x468]
14001ea4d: movups xmmword ptr [rcx + 0x20], xmm0
14001ea51: mov qword ptr [rbp + 0x498], r8
14001ea58: mov rax, qword ptr [rbp + 0x7f0]
14001ea5f: mov qword ptr [rbp + 0x4a0], rax
14001ea66: mov qword ptr [rbp + 0x4a8], rbx
14001ea6d: mov rax, qword ptr [rbp + 0x7a0]
14001ea74: mov qword ptr [rbp + 0x4b0], rax
14001ea7b: mov r8, qword ptr [rbp + 0x7d0]
14001ea82: mov qword ptr [rbp + 0x4b8], r8
14001ea89: mov qword ptr [rbp + 0x4c0], rdi
14001ea90: mov qword ptr [rbp + 0x4c8], rdx
14001ea97: mov rax, qword ptr [rbp + 0x7d8]
14001ea9e: mov qword ptr [rbp + 0x4d0], rax
14001eaa5: mov qword ptr [rbp + 0x4d8], rsi
14001eaac: mov rax, qword ptr [rbp + 0x678]
14001eab3: mov qword ptr [rbp + 0x4e0], rax
14001eaba: mov eax, dword ptr [rbp + 0x588]
14001eac0: mov dword ptr [rbp + 0x4e8], eax
14001eac6: mov dword ptr [rbp + 0x4ec], r10d
14001eacd: mov dword ptr [rbp + 0x4f0], r9d
14001ead4: movss dword ptr [rbp + 0x4f4], xmm6
14001eadc: mov word ptr [rbp + 0x4f8], r11w
14001eae4: movzx eax, word ptr [rbp + 0x56c]
14001eaeb: mov word ptr [rcx + 0x96], ax
14001eaf2: mov eax, dword ptr [rbp + 0x568]
14001eaf8: mov dword ptr [rcx + 0x92], eax
14001eafe: test rdi, rdi
14001eb01: je 0x140020958
14001eb07: lea rax, [r8 + rdi*4]
14001eb0b: add rax, -4
14001eb0f: je 0x140020970
14001eb15: cmp byte ptr [rbp + 0x530], 0
14001eb1c: je 0x14001ec06
14001eb22: mov rax, qword ptr [rbp + 0x258]
14001eb29: mov ecx, dword ptr [rbp + 0x424]
14001eb2f: mov dword ptr [rax], ecx
14001eb31: mov rax, qword ptr [rbp + 0x490]
14001eb38: mov qword ptr [rbp + 0x5c0], rax
14001eb3f: movups xmm0, xmmword ptr [rbp + 0x460]
14001eb46: movups xmm1, xmmword ptr [rbp + 0x470]
14001eb4d: movups xmm2, xmmword ptr [rbp + 0x480]
14001eb54: movaps xmmword ptr [rbp + 0x5b0], xmm2
14001eb5b: movaps xmmword ptr [rbp + 0x5a0], xmm1
14001eb62: movaps xmmword ptr [rbp + 0x590], xmm0
14001eb69: lea rdx, [rbp + 0x468]
14001eb70: mov rax, qword ptr [rdx + 0x40]
14001eb74: lea rcx, [rbp + 0x598]
14001eb7b: mov qword ptr [rcx + 0x40], rax
14001eb7f: movups xmm0, xmmword ptr [rdx + 0x30]
14001eb83: movups xmmword ptr [rcx + 0x30], xmm0
14001eb87: mov rax, qword ptr [rdx + 0x58]
14001eb8b: mov qword ptr [rcx + 0x70], rax
14001eb8f: movups xmm0, xmmword ptr [rdx + 0x48]
14001eb93: movups xmmword ptr [rcx + 0x60], xmm0
14001eb97: mov rax, qword ptr [rdx + 0x70]
14001eb9b: mov qword ptr [rcx + 0x58], rax
14001eb9f: movups xmm0, xmmword ptr [rdx + 0x60]
14001eba3: movups xmmword ptr [rcx + 0x48], xmm0
14001eba7: mov rax, qword ptr [rbp + 0x788]
14001ebae: mov dword ptr [rbp + 0x630], eax
14001ebb4: mov rax, qword ptr [rbp + 0x670]
14001ebbb: mov rax, qword ptr [rax]
14001ebbe: mov qword ptr [rbp + 0x628], rax
14001ebc5: mov word ptr [rbp + 0x634], r11w
14001ebcd: movabs rax, 0x8000000000000000
14001ebd7: mov qword ptr [rbp + 0x610], rax
14001ebde: mov rdi, qword ptr [rbp + 0x338]
14001ebe5: mov rsi, qword ptr [rdi + 0x10]
14001ebe9: cmp rsi, qword ptr [rdi]
14001ebec: jne 0x14001e11d
14001ebf2: mov rcx, rdi
14001ebf5: lea rdx, [rip + 0x2907b94]
14001ebfc: call 0x1405636c0 ; alloc::raw_vec::RawVec::grow_one<country_core::resources::stairs_walls::StairsBracketAssemblyParams,alloc::alloc::Global>
14001ec01: jmp 0x14001e11d ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001ec06: movss xmm0, dword ptr [r8]
14001ec0b: movss xmm1, dword ptr [rax]
14001ec0f: movaps xmm2, xmm0
14001ec12: cmpunordss xmm2, xmm0
14001ec17: movaps xmm3, xmm2
14001ec1a: andps xmm3, xmm1
14001ec1d: maxss xmm1, xmm0
14001ec21: andnps xmm2, xmm1
14001ec24: orps xmm2, xmm3
14001ec27: movss xmm0, dword ptr [rip + 0x2906ca1]
14001ec2f: ucomiss xmm0, xmm2
14001ec32: jb 0x14001ec6d
14001ec34: mov byte ptr [rbp + 0x814], 1
14001ec3b: mov rcx, qword ptr [rbp + 0x2d8]
14001ec42: call 0x140959ef0 ; country_core::resources::stairs::stairs_assembly_params::StairNodeAnchor::try_as_wall
14001ec47: mov rdi, rax
14001ec4a: mov byte ptr [rbp + 0x814], 1
14001ec51: mov rcx, qword ptr [rbp + 0x118]
14001ec58: call 0x140959ef0 ; country_core::resources::stairs::stairs_assembly_params::StairNodeAnchor::try_as_wall
14001ec5d: test rdi, rdi
14001ec60: sete cl
14001ec63: test rax, rax
14001ec66: sete dl
14001ec69: or dl, cl
14001ec6b: je 0x14001ecc7
14001ec6d: mov rax, qword ptr [rbp + 0x330]
14001ec74: mov ecx, dword ptr [rbp + 0x534]
14001ec7a: mov dword ptr [rax], ecx
14001ec7c: mov r8d, 0xa0
14001ec82: mov rcx, r12
14001ec85: lea rdx, [rbp + 0x460]
14001ec8c: call 0x142923c00 ; ?
14001ec91: mov rdi, qword ptr [rbp + 0x360]
14001ec98: mov rsi, qword ptr [rdi + 0x10]
14001ec9c: cmp rsi, qword ptr [rdi]
14001ec9f: jne 0x14001ecb0
14001eca1: mov rcx, rdi
14001eca4: lea rdx, [rip + 0x2907acd]
14001ecab: call 0x140563480 ; alloc::raw_vec::RawVec::grow_one<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams,alloc::alloc::Global>
14001ecb0: lea rcx, [rsi + rsi*4]
14001ecb4: shl rcx, 5
14001ecb8: add rcx, qword ptr [rdi + 8]
14001ecbc: mov r8d, 0xa0
14001ecc2: jmp 0x14001e12e ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001ecc7: mov rcx, qword ptr [rdi]
14001ecca: cmp rcx, qword ptr [rax]
14001eccd: jne 0x14001ecdc
14001eccf: movzx r11d, word ptr [rbp + 0x4f8]
14001ecd7: jmp 0x14001eb22 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001ecdc: mov byte ptr [rbp + 0x814], 1
14001ece3: mov qword ptr [rsp + 0x20], rdi
14001ece8: mov rcx, r12
14001eceb: mov rdx, qword ptr [rbp + 0x368]
14001ecf2: mov r8, qword ptr [rbp + 0x938]
14001ecf9: lea r9, [rbp + 0x460]
14001ed00: call 0x1407d94a0 ; system_decorator::decorator_visual::stairs_assemble::split_segment_into_supported_and_unsupported_subsegments
14001ed05: mov rax, qword ptr [rbp + 0x590]
14001ed0c: mov qword ptr [rbp + 0x648], rax
14001ed13: mov r13, qword ptr [rbp + 0x598]
14001ed1a: mov rcx, qword ptr [rbp + 0x5a0]
14001ed21: mov rax, qword ptr [rbp + 0x5a8]
14001ed28: mov qword ptr [rbp + 0x640], rax
14001ed2f: mov rax, qword ptr [rbp + 0x5b0]
14001ed36: mov qword ptr [rbp + 0x540], rax
14001ed3d: mov rax, qword ptr [rbp + 0x5b8]
14001ed44: mov qword ptr [rbp + 0x538], rax
14001ed4b: mov rax, qword ptr [rbp + 0x5c0]
14001ed52: mov qword ptr [rbp + 0x528], rax
14001ed59: mov rax, qword ptr [rbp + 0x5c8]
14001ed60: mov qword ptr [rbp + 0x6a8], rax
14001ed67: mov rax, qword ptr [rbp + 0x5d0]
14001ed6e: mov qword ptr [rbp + 0x500], rax
14001ed75: shl rcx, 4
14001ed79: add rcx, r13
14001ed7c: mov qword ptr [rbp + 0x7e0], rcx
14001ed83: mov qword ptr [rbp + 0x580], r13
14001ed8a: nop word ptr [rax + rax]
14001ed90: mov r12, r13
14001ed93: cmp r12, qword ptr [rbp + 0x7e0]
14001ed9a: mov qword ptr [rbp + 0x560], r12
14001eda1: je 0x14001f263
14001eda7: mov r14, qword ptr [r12 + 8]
14001edac: cmp r14, -1
14001edb0: je 0x1400204c1
14001edb6: mov r13, qword ptr [r12]
14001edba: lea rax, [r14 + 1]
14001edbe: mov rdi, rax
14001edc1: sub rdi, r13
14001edc4: mov r15, qword ptr [rbp + 0x80]
14001edcb: jb 0x1400204d2
14001edd1: mov rdx, qword ptr [rbp + 0x470]
14001edd8: cmp r14, rdx
14001eddb: jae 0x1400204e9
14001ede1: mov qword ptr [rbp + 0x780], rax
14001ede8: mov rax, rdi
14001edeb: mov ecx, 0xc
14001edf0: mul rcx
14001edf3: jo 0x140020988
14001edf9: mov esi, 0
14001edfe: movabs rcx, 0x7ffffffffffffffc
14001ee08: cmp rax, rcx
14001ee0b: ja 0x14002098a
14001ee11: mov rbx, qword ptr [rbp + 0x468]
14001ee18: test rax, rax
14001ee1b: je 0x14001ee50
14001ee1d: movzx ecx, byte ptr [rip + 0x3615de5]
14001ee24: mov esi, 4
14001ee29: mov edx, 4
14001ee2e: mov rcx, rax
14001ee31: mov r15, rax
14001ee34: call 0x140613c10 ; ?
14001ee39: test rax, rax
14001ee3c: je 0x14002098a
14001ee42: mov rsi, rdi
14001ee45: mov r15, rax
14001ee48: jmp 0x14001ee58 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001ee4a: nop word ptr [rax + rax]
14001ee50: mov r15d, 4
14001ee56: xor esi, esi
14001ee58: mov qword ptr [rbp + 0x6a0], r13
14001ee5f: lea rax, [r13*2]
14001ee67: add rax, r13
14001ee6a: lea rdx, [rbx + rax*4]
14001ee6e: lea r12, [rdi*4]
14001ee76: lea r8, [r12 + r12*2]
14001ee7a: mov rcx, r15
14001ee7d: call 0x142923c00 ; ?
14001ee82: mov qword ptr [rbp + 0x380], rsi
14001ee89: mov qword ptr [rbp + 0x388], r15
14001ee90: mov qword ptr [rbp + 0x390], rdi
14001ee97: lea rcx, [rbp + 0x590]
14001ee9e: lea rdx, [rbp + 0x380]
14001eea5: mov r8d, 1
14001eeab: call 0x140153640 ; utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14001eeb0: mov qword ptr [rbp + 0x80], r15
14001eeb7: mov rax, qword ptr [rbp + 0x560]
14001eebe: lea r13, [rax + 0x10]
14001eec2: mov rax, qword ptr [rbp + 0x590]
14001eec9: mov rcx, rax
14001eecc: neg rcx
14001eecf: jo 0x14001ed90
14001eed5: mov rcx, qword ptr [rbp + 0x598]
14001eedc: mov r15, qword ptr [rbp + 0x5a8]
14001eee3: mov rbx, qword ptr [rbp + 0x5b0]
14001eeea: movss xmm0, dword ptr [rbp + 0x5c0]
14001eef2: ucomiss xmm0, dword ptr [rip + 0x2906a93]
14001eef9: ja 0x14001ef39
14001eefb: test rax, rax
14001eefe: je 0x14001ef13
14001ef00: shl rax, 2
14001ef04: lea rdx, [rax + rax*2]
14001ef08: mov r8d, 4
14001ef0e: call 0x140613c20 ; ?
14001ef13: mov r12, r13
14001ef16: test r15, r15
14001ef19: je 0x14001ed93
14001ef1f: shl r15, 2
14001ef23: mov r8d, 4
14001ef29: mov rcx, rbx
14001ef2c: mov rdx, r15
14001ef2f: call 0x140613c20 ; ?
14001ef34: jmp 0x14001ed90 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001ef39: mov rdx, qword ptr [rbp + 0x5a0]
14001ef40: mov r8, qword ptr [rbp + 0x5b8]
14001ef47: mov r9d, dword ptr [rbp + 0x5c4]
14001ef4e: mov r10, qword ptr [rbp + 0x330]
14001ef55: mov r11d, dword ptr [rbp + 0x534]
14001ef5c: mov dword ptr [r10], r11d
14001ef5f: mov qword ptr [rbp + 0x380], rax
14001ef66: mov qword ptr [rbp + 0x388], rcx
14001ef6d: mov qword ptr [rbp + 0x390], rdx
14001ef74: mov qword ptr [rbp + 0x398], r15
14001ef7b: mov qword ptr [rbp + 0x3a0], rbx
14001ef82: mov qword ptr [rbp + 0x3a8], r8
14001ef89: movss dword ptr [rbp + 0x3b0], xmm0
14001ef91: mov dword ptr [rbp + 0x3b4], r9d
14001ef98: mov rdx, qword ptr [rbp + 0x4a8]
14001ef9f: cmp r14, rdx
14001efa2: jae 0x1400206ec
14001efa8: mov rax, rdi
14001efab: shr rax, 0x3c
14001efaf: jne 0x140020c00
14001efb5: mov rax, qword ptr [rbp + 0x6a0]
14001efbc: lea r15, [rax*8]
14001efc4: add r15, qword ptr [rbp + 0x4a0]
14001efcb: lea rbx, [rdi*8]
14001efd3: test rdi, rdi
14001efd6: je 0x14001f003
14001efd8: movzx eax, byte ptr [rip + 0x3615c2a]
14001efdf: mov esi, 4
14001efe4: mov edx, 4
14001efe9: mov rcx, rbx
14001efec: call 0x140613c10 ; ?
14001eff1: test rax, rax
14001eff4: je 0x140020c9d
14001effa: mov qword ptr [rbp + 0x588], rdi
14001f001: jmp 0x14001f013 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001f003: mov eax, 4
14001f008: mov qword ptr [rbp + 0x588], 0
14001f013: mov qword ptr [rbp + 0x2e8], rax
14001f01a: mov rcx, rax
14001f01d: mov rdx, r15
14001f020: mov r8, rbx
14001f023: call 0x142923c00 ; ?
14001f028: mov rdx, qword ptr [rbp + 0x4c0]
14001f02f: cmp r14, rdx
14001f032: jae 0x140020704
14001f038: mov rax, qword ptr [rbp + 0x6a0]
14001f03f: lea rsi, [rax*4]
14001f047: add rsi, qword ptr [rbp + 0x4b8]
14001f04e: test rdi, rdi
14001f051: je 0x14001f0cb
14001f053: movzx eax, byte ptr [rip + 0x3615baf]
14001f05a: mov edx, 4
14001f05f: mov rcx, r12
14001f062: call 0x140613c10 ; ?
14001f067: test rax, rax
14001f06a: je 0x140020ce6
14001f070: mov rbx, rax
14001f073: mov rcx, rax
14001f076: mov rdx, rsi
14001f079: mov r8, r12
14001f07c: call 0x142923c00 ; ?
14001f081: mov rdx, qword ptr [rbp + 0x4d8]
14001f088: cmp r14, rdx
14001f08b: jae 0x140020817
14001f091: mov rsi, rbx
14001f094: mov rbx, qword ptr [rbp + 0x4d0]
14001f09b: movzx eax, byte ptr [rip + 0x3615b67]
14001f0a2: mov edx, 4
14001f0a7: mov rcx, r12
14001f0aa: call 0x140613c10 ; ?
14001f0af: test rax, rax
14001f0b2: je 0x140020cbf
14001f0b8: mov r15, rax
14001f0bb: mov qword ptr [rbp + 0x780], rdi
14001f0c2: mov qword ptr [rbp + 0x678], rsi
14001f0c9: jmp 0x14001f114 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001f0cb: mov ebx, 4
14001f0d0: mov ecx, 4
14001f0d5: mov rdx, rsi
14001f0d8: mov r8, r12
14001f0db: call 0x142923c00 ; ?
14001f0e0: mov rdx, qword ptr [rbp + 0x4d8]
14001f0e7: cmp r14, rdx
14001f0ea: jae 0x140020815
14001f0f0: mov eax, 4
14001f0f5: mov qword ptr [rbp + 0x678], rax
14001f0fc: mov rbx, qword ptr [rbp + 0x4d0]
14001f103: mov qword ptr [rbp + 0x780], 0
14001f10e: mov r15d, 4
14001f114: mov rsi, qword ptr [rbp + 0x6a0]
14001f11b: lea rdx, [rbx + rsi*4]
14001f11f: mov rcx, r15
14001f122: mov r8, r12
14001f125: call 0x142923c00 ; ?
14001f12a: mov rax, qword ptr [rbp + 0x470]
14001f131: dec rax
14001f134: test rsi, rsi
14001f137: movups xmm0, xmmword ptr [rbp + 0x380]
14001f13e: movups xmm1, xmmword ptr [rbp + 0x390]
14001f145: movups xmm2, xmmword ptr [rbp + 0x3a0]
14001f14c: movaps xmmword ptr [rbp + 0x590], xmm0
14001f153: movaps xmmword ptr [rbp + 0x5a0], xmm1
14001f15a: movaps xmmword ptr [rbp + 0x5b0], xmm2
14001f161: mov rcx, qword ptr [rbp + 0x3b0]
14001f168: mov qword ptr [rbp + 0x5c0], rcx
14001f16f: mov rcx, qword ptr [rbp + 0x588]
14001f176: mov qword ptr [rbp + 0x5c8], rcx
14001f17d: mov rcx, qword ptr [rbp + 0x2e8]
14001f184: mov qword ptr [rbp + 0x5d0], rcx
14001f18b: mov qword ptr [rbp + 0x5d8], rdi
14001f192: mov rcx, qword ptr [rbp + 0x780]
14001f199: mov qword ptr [rbp + 0x5e0], rcx
14001f1a0: mov rdx, qword ptr [rbp + 0x678]
14001f1a7: mov qword ptr [rbp + 0x5e8], rdx
14001f1ae: mov qword ptr [rbp + 0x5f0], rdi
14001f1b5: mov qword ptr [rbp + 0x5f8], rcx
14001f1bc: mov qword ptr [rbp + 0x600], r15
14001f1c3: mov qword ptr [rbp + 0x608], rdi
14001f1ca: mov rcx, qword ptr [rbp + 0x438]
14001f1d1: mov qword ptr [rbp + 0x610], rcx
14001f1d8: mov word ptr [rbp + 0x624], 0x101
14001f1e1: mov rcx, qword ptr [rbp + 0x788]
14001f1e8: mov dword ptr [rbp + 0x620], ecx
14001f1ee: mov rcx, qword ptr [rbp + 0x670]
14001f1f5: mov rcx, qword ptr [rcx]
14001f1f8: setne byte ptr [rbp + 0x626]
14001f1ff: cmp r14, rax
14001f202: mov qword ptr [rbp + 0x618], rcx
14001f209: setne byte ptr [rbp + 0x627]
14001f210: mov dword ptr [rbp + 0x628], 0x3000000
14001f21a: mov rdi, qword ptr [rbp + 0x360]
14001f221: mov rsi, qword ptr [rdi + 0x10]
14001f225: cmp rsi, qword ptr [rdi]
14001f228: jne 0x14001f239
14001f22a: mov rcx, rdi
14001f22d: lea rdx, [rip + 0x290752c]
14001f234: call 0x140563480 ; alloc::raw_vec::RawVec::grow_one<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams,alloc::alloc::Global>
14001f239: lea rcx, [rsi + rsi*4]
14001f23d: shl rcx, 5
14001f241: add rcx, qword ptr [rdi + 8]
14001f245: mov r8d, 0xa0
14001f24b: lea rdx, [rbp + 0x590]
14001f252: call 0x142923c10 ; ?
14001f257: inc rsi
14001f25a: mov qword ptr [rdi + 0x10], rsi
14001f25e: jmp 0x14001ed90 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001f263: mov rdx, qword ptr [rbp + 0x648]
14001f26a: test rdx, rdx
14001f26d: mov rsi, qword ptr [rbp + 0x500]
14001f274: je 0x14001f28c
14001f276: shl rdx, 4
14001f27a: mov r8d, 8
14001f280: mov rcx, qword ptr [rbp + 0x580]
14001f287: call 0x140613c20 ; ?
14001f28c: mov rdx, qword ptr [rbp + 0x538]
14001f293: shl rdx, 4
14001f297: mov rcx, qword ptr [rbp + 0x540]
14001f29e: add rdx, rcx
14001f2a1: lea rax, [rsi + rsi*2]
14001f2a5: mov r13, qword ptr [rbp + 0x6a8]
14001f2ac: lea r8, [rax*8]
14001f2b4: add r8, r13
14001f2b7: xorps xmm0, xmm0
14001f2ba: lea rax, [rbp + 0x3c0]
14001f2c1: movups xmmword ptr [rax], xmm0
14001f2c4: mov qword ptr [rax + 0x10], 0
14001f2cc: mov qword ptr [rbp + 0x380], rcx
14001f2d3: mov qword ptr [rbp + 0x388], rcx
14001f2da: mov rax, qword ptr [rbp + 0x640]
14001f2e1: mov qword ptr [rbp + 0x390], rax
14001f2e8: mov qword ptr [rbp + 0x538], rdx
14001f2ef: mov qword ptr [rbp + 0x398], rdx
14001f2f6: mov qword ptr [rbp + 0x3a0], r13
14001f2fd: mov qword ptr [rbp + 0x3a8], r13
14001f304: mov rax, qword ptr [rbp + 0x528]
14001f30b: mov qword ptr [rbp + 0x3b0], rax
14001f312: mov qword ptr [rbp + 0x580], r8
14001f319: mov qword ptr [rbp + 0x3b8], r8
14001f320: mov r12, r13
14001f323: mov r14, rcx
14001f326: nop word ptr cs:[rax + rax]
14001f330: mov rax, r14
14001f333: cmp rax, qword ptr [rbp + 0x538]
14001f33a: je 0x14001f842
14001f340: lea r13, [rax + 0x10]
14001f344: mov qword ptr [rbp + 0x388], r13
14001f34b: cmp r12, qword ptr [rbp + 0x580]
14001f352: je 0x14001f83b
14001f358: mov rbx, qword ptr [rax]
14001f35b: mov r15, qword ptr [rax + 8]
14001f35f: lea rdx, [r12 + 0x18]
14001f364: mov qword ptr [rbp + 0x3a8], rdx
14001f36b: mov rcx, qword ptr [r12]
14001f36f: mov rax, rcx
14001f372: neg rax
14001f375: jo 0x14001f9ac
14001f37b: mov rax, qword ptr [r12 + 8]
14001f380: mov qword ptr [rbp + 0x648], rax
14001f387: cmp r15, -1
14001f38b: mov qword ptr [rbp + 0x6a0], rcx
14001f392: je 0x1400204fd
14001f398: mov qword ptr [rbp + 0x780], rdx
14001f39f: lea rax, [r15 + 1]
14001f3a3: mov rdi, rax
14001f3a6: sub rdi, rbx
14001f3a9: jb 0x140020515
14001f3af: mov rdx, qword ptr [rbp + 0x470]
14001f3b6: cmp r15, rdx
14001f3b9: jae 0x140020533
14001f3bf: mov qword ptr [rbp + 0x678], rax
14001f3c6: mov rax, rdi
14001f3c9: mov ecx, 0xc
14001f3ce: mul rcx
14001f3d1: jo 0x140020a31
14001f3d7: mov esi, 0
14001f3dc: movabs rcx, 0x7ffffffffffffffc
14001f3e6: cmp rax, rcx
14001f3e9: ja 0x140020a33
14001f3ef: mov rcx, qword ptr [r12 + 0x10]
14001f3f4: mov qword ptr [rbp + 0x588], rcx
14001f3fb: mov r12, qword ptr [rbp + 0x468]
14001f402: test rax, rax
14001f405: je 0x14001f440
14001f407: movzx ecx, byte ptr [rip + 0x36157fb]
14001f40e: mov esi, 4
14001f413: mov edx, 4
14001f418: mov rcx, rax
14001f41b: mov r14, rax
14001f41e: call 0x140613c10 ; ?
14001f423: mov qword ptr [rbp + 0xf8], r14
14001f42a: test rax, rax
14001f42d: je 0x140020a33
14001f433: mov rsi, rdi
14001f436: mov r14, rax
14001f439: jmp 0x14001f448 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001f43b: nop dword ptr [rax + rax]
14001f440: mov r14d, 4
14001f446: xor esi, esi
14001f448: lea rax, [rbx + rbx*2]
14001f44c: lea rdx, [r12 + rax*4]
14001f450: lea r12, [rdi*4]
14001f458: lea r8, [r12 + r12*2]
14001f45c: mov rcx, r14
14001f45f: call 0x142923c00 ; ?
14001f464: mov qword ptr [rbp + 0x3e0], rsi
14001f46b: mov qword ptr [rbp + 0xf8], r14
14001f472: mov qword ptr [rbp + 0x3e8], r14
14001f479: mov qword ptr [rbp + 0x3f0], rdi
14001f480: mov byte ptr [rbp + 0x813], 1
14001f487: lea rcx, [rbp + 0x590]
14001f48e: lea rdx, [rbp + 0x3e0]
14001f495: mov r8d, 1
14001f49b: call 0x140153640 ; utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14001f4a0: xor eax, eax
14001f4a2: cmp rax, qword ptr [rbp + 0x590]
14001f4a9: mov r14, r13
14001f4ac: jno 0x14001f4e9
14001f4ae: mov rsi, qword ptr [rbp + 0x780]
14001f4b5: mov r12, rsi
14001f4b8: mov rax, r14
14001f4bb: mov rdx, qword ptr [rbp + 0x6a0]
14001f4c2: test rdx, rdx
14001f4c5: je 0x14001f333
14001f4cb: shl rdx, 2
14001f4cf: mov r8d, 4
14001f4d5: mov rcx, qword ptr [rbp + 0x648]
14001f4dc: call 0x140613c20 ; ?
14001f4e1: mov r12, rsi
14001f4e4: jmp 0x14001f330 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001f4e9: mov rax, qword ptr [rbp + 0x5c0]
14001f4f0: mov qword ptr [rbp + 0x2c0], rax
14001f4f7: movups xmm0, xmmword ptr [rbp + 0x590]
14001f4fe: movups xmm1, xmmword ptr [rbp + 0x5a0]
14001f505: movups xmm2, xmmword ptr [rbp + 0x5b0]
14001f50c: movaps xmmword ptr [rbp + 0x2b0], xmm2
14001f513: movaps xmmword ptr [rbp + 0x2a0], xmm1
14001f51a: movaps xmmword ptr [rbp + 0x290], xmm0
14001f521: mov rax, qword ptr [rbp + 0x258]
14001f528: mov ecx, dword ptr [rbp + 0x424]
14001f52e: mov dword ptr [rax], ecx
14001f530: mov rax, qword ptr [rbp + 0x2c0]
14001f537: mov qword ptr [rbp + 0x410], rax
14001f53e: movaps xmm0, xmmword ptr [rbp + 0x290]
14001f545: movaps xmm1, xmmword ptr [rbp + 0x2a0]
14001f54c: movaps xmm2, xmmword ptr [rbp + 0x2b0]
14001f553: movaps xmmword ptr [rbp + 0x400], xmm2
14001f55a: movaps xmmword ptr [rbp + 0x3f0], xmm1
14001f561: movaps xmmword ptr [rbp + 0x3e0], xmm0
14001f568: mov rdx, qword ptr [rbp + 0x4a8]
14001f56f: cmp r15, rdx
14001f572: jae 0x140020766
14001f578: mov rax, rdi
14001f57b: shr rax, 0x3c
14001f57f: jne 0x140020c32
14001f585: lea r13, [rbx*8]
14001f58d: add r13, qword ptr [rbp + 0x4a0]
14001f594: lea r8, [rdi*8]
14001f59c: test rdi, rdi
14001f59f: je 0x14001f5cd
14001f5a1: movzx eax, byte ptr [rip + 0x3615661]
14001f5a8: mov edx, 4
14001f5ad: mov rsi, r8
14001f5b0: mov rcx, r8
14001f5b3: call 0x140613c10 ; ?
14001f5b8: test rax, rax
14001f5bb: je 0x140020df6
14001f5c1: mov qword ptr [rbp + 0x548], rdi
14001f5c8: mov r8, rsi
14001f5cb: jmp 0x14001f5dd ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001f5cd: mov eax, 4
14001f5d2: mov qword ptr [rbp + 0x548], 0
14001f5dd: mov qword ptr [rbp + 0x2f0], rax
14001f5e4: mov rcx, rax
14001f5e7: mov rdx, r13
14001f5ea: call 0x142923c00 ; ?
14001f5ef: mov rdx, qword ptr [rbp + 0x4c0]
14001f5f6: cmp r15, rdx
14001f5f9: jae 0x14002077e
14001f5ff: lea rsi, [rbx*4]
14001f607: add rsi, qword ptr [rbp + 0x4b8]
14001f60e: test rdi, rdi
14001f611: je 0x14001f689
14001f613: mov r13, qword ptr [rbp + 0x548]
14001f61a: movzx eax, byte ptr [rip + 0x36155e8]
14001f621: mov edx, 4
14001f626: mov rcx, r12
14001f629: call 0x140613c10 ; ?
14001f62e: test rax, rax
14001f631: je 0x140020e48
14001f637: mov qword ptr [rbp + 0x370], rax
14001f63e: mov rcx, rax
14001f641: mov rdx, rsi
14001f644: mov r8, r12
14001f647: call 0x142923c00 ; ?
14001f64c: mov rdx, qword ptr [rbp + 0x4d8]
14001f653: cmp r15, rdx
14001f656: jae 0x140020867
14001f65c: mov rsi, qword ptr [rbp + 0x4d0]
14001f663: movzx eax, byte ptr [rip + 0x361559f]
14001f66a: mov edx, 4
14001f66f: mov rcx, r12
14001f672: call 0x140613c10 ; ?
14001f677: test rax, rax
14001f67a: je 0x140020e1a
14001f680: mov qword ptr [rbp + 0x678], rdi
14001f687: jmp 0x14001f6ce ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001f689: mov r13, qword ptr [rbp + 0x548]
14001f690: mov ecx, 4
14001f695: mov rdx, rsi
14001f698: mov r8, r12
14001f69b: call 0x142923c00 ; ?
14001f6a0: mov rdx, qword ptr [rbp + 0x4d8]
14001f6a7: mov eax, 4
14001f6ac: cmp r15, rdx
14001f6af: jae 0x14002085e
14001f6b5: mov qword ptr [rbp + 0x370], rax
14001f6bc: mov rsi, qword ptr [rbp + 0x4d0]
14001f6c3: mov qword ptr [rbp + 0x678], 0
14001f6ce: lea rdx, [rsi + rbx*4]
14001f6d2: mov rsi, rax
14001f6d5: mov rcx, rax
14001f6d8: mov r8, r12
14001f6db: call 0x142923c00 ; ?
14001f6e0: xor eax, eax
14001f6e2: mov ecx, 0
14001f6e7: test rbx, rbx
14001f6ea: jne 0x14001f6f2
14001f6ec: mov ecx, dword ptr [rbp + 0x4f8]
14001f6f2: mov rdx, qword ptr [rbp + 0x470]
14001f6f9: dec rdx
14001f6fc: cmp r15, rdx
14001f6ff: movzx edx, byte ptr [rbp + 0x4f9]
14001f706: cmove eax, edx
14001f709: mov rdx, qword ptr [rbp + 0x6a0]
14001f710: mov qword ptr [rbp + 0x610], rdx
14001f717: mov rdx, qword ptr [rbp + 0x648]
14001f71e: mov qword ptr [rbp + 0x618], rdx
14001f725: mov rdx, qword ptr [rbp + 0x588]
14001f72c: mov qword ptr [rbp + 0x620], rdx
14001f733: mov rdx, qword ptr [rbp + 0x410]
14001f73a: mov qword ptr [rbp + 0x5c0], rdx
14001f741: movaps xmm0, xmmword ptr [rbp + 0x3e0]
14001f748: movaps xmm1, xmmword ptr [rbp + 0x3f0]
14001f74f: movaps xmm2, xmmword ptr [rbp + 0x400]
14001f756: movaps xmmword ptr [rbp + 0x590], xmm0
14001f75d: movaps xmmword ptr [rbp + 0x5a0], xmm1
14001f764: movaps xmmword ptr [rbp + 0x5b0], xmm2
14001f76b: mov qword ptr [rbp + 0x5c8], r13
14001f772: mov rdx, qword ptr [rbp + 0x2f0]
14001f779: mov qword ptr [rbp + 0x5d0], rdx
14001f780: mov qword ptr [rbp + 0x5d8], rdi
14001f787: mov rdx, qword ptr [rbp + 0x678]
14001f78e: mov qword ptr [rbp + 0x5e0], rdx
14001f795: mov qword ptr [rbp + 0x5e8], rsi
14001f79c: mov qword ptr [rbp + 0x5f0], rdi
14001f7a3: mov qword ptr [rbp + 0x5f8], rdx
14001f7aa: mov rdx, qword ptr [rbp + 0x370]
14001f7b1: mov qword ptr [rbp + 0x600], rdx
14001f7b8: mov qword ptr [rbp + 0x608], rdi
14001f7bf: mov rdx, qword ptr [rbp + 0x788]
14001f7c6: mov dword ptr [rbp + 0x630], edx
14001f7cc: mov rdx, qword ptr [rbp + 0x670]
14001f7d3: mov rdx, qword ptr [rdx]
14001f7d6: mov qword ptr [rbp + 0x628], rdx
14001f7dd: mov byte ptr [rbp + 0x634], cl
14001f7e3: mov byte ptr [rbp + 0x635], al
14001f7e9: mov rdi, qword ptr [rbp + 0x338]
14001f7f0: mov rsi, qword ptr [rdi + 0x10]
14001f7f4: cmp rsi, qword ptr [rdi]
14001f7f7: lea rbx, [rbp + 0x590]
14001f7fe: jne 0x14001f80f
14001f800: mov rcx, rdi
14001f803: lea rdx, [rip + 0x2906ede]
14001f80a: call 0x1405636c0 ; alloc::raw_vec::RawVec::grow_one<country_core::resources::stairs_walls::StairsBracketAssemblyParams,alloc::alloc::Global>
14001f80f: imul rcx, rsi, 0xa8
14001f816: add rcx, qword ptr [rdi + 8]
14001f81a: mov r8d, 0xa8
14001f820: mov rdx, rbx
14001f823: call 0x142923c10 ; ?
14001f828: inc rsi
14001f82b: mov qword ptr [rdi + 0x10], rsi
14001f82f: mov r12, qword ptr [rbp + 0x780]
14001f836: jmp 0x14001f330 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001f83b: mov r12, qword ptr [rbp + 0x580]
14001f842: mov rdx, qword ptr [rbp + 0x640]
14001f849: test rdx, rdx
14001f84c: mov r13, qword ptr [rbp + 0x940]
14001f853: je 0x14001f86b
14001f855: shl rdx, 4
14001f859: mov r8d, 8
14001f85f: mov rcx, qword ptr [rbp + 0x540]
14001f866: call 0x140613c20 ; ?
14001f86b: mov r8, qword ptr [rbp + 0x580]
14001f872: mov rax, r8
14001f875: sub rax, r12
14001f878: movabs rcx, 0xaaaaaaaaaaaaaaab
14001f882: mul rcx
14001f885: cmp r8, r12
14001f888: je 0x14001f8c8
14001f88a: mov rdi, rdx
14001f88d: shr rdi, 4
14001f891: add r12, 8
14001f895: jmp 0x14001f8a9 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001f897: nop word ptr [rax + rax]
14001f8a0: add r12, 0x18
14001f8a4: dec rdi
14001f8a7: je 0x14001f8c8
14001f8a9: mov rdx, qword ptr [r12 - 8]
14001f8ae: test rdx, rdx
14001f8b1: je 0x14001f8a0
14001f8b3: mov rcx, qword ptr [r12]
14001f8b7: shl rdx, 2
14001f8bb: mov r8d, 4
14001f8c1: call 0x140613c20 ; ?
14001f8c6: jmp 0x14001f8a0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001f8c8: mov rax, qword ptr [rbp + 0x528]
14001f8cf: test rax, rax
14001f8d2: je 0x14001f8ee
14001f8d4: shl rax, 3
14001f8d8: lea rdx, [rax + rax*2]
14001f8dc: mov r8d, 8
14001f8e2: mov rcx, qword ptr [rbp + 0x6a8]
14001f8e9: call 0x140613c20 ; ?
14001f8ee: mov rax, qword ptr [rbp + 0x460]
14001f8f5: test rax, rax
14001f8f8: lea r12, [rbp + 0x590]
14001f8ff: je 0x14001f91b
14001f901: mov rcx, qword ptr [rbp + 0x468]
14001f908: shl rax, 2
14001f90c: lea rdx, [rax + rax*2]
14001f910: mov r8d, 4
14001f916: call 0x140613c20 ; ?
14001f91b: mov rdx, qword ptr [rbp + 0x478]
14001f922: test rdx, rdx
14001f925: je 0x14001f93d
14001f927: mov rcx, qword ptr [rbp + 0x480]
14001f92e: shl rdx, 2
14001f932: mov r8d, 4
14001f938: call 0x140613c20 ; ?
14001f93d: mov rdx, qword ptr [rbp + 0x498]
14001f944: test rdx, rdx
14001f947: je 0x14001f95f
14001f949: mov rcx, qword ptr [rbp + 0x4a0]
14001f950: shl rdx, 3
14001f954: mov r8d, 4
14001f95a: call 0x140613c20 ; ?
14001f95f: mov rdx, qword ptr [rbp + 0x4b0]
14001f966: test rdx, rdx
14001f969: je 0x14001f981
14001f96b: mov rcx, qword ptr [rbp + 0x4b8]
14001f972: shl rdx, 2
14001f976: mov r8d, 4
14001f97c: call 0x140613c20 ; ?
14001f981: mov rdx, qword ptr [rbp + 0x4c8]
14001f988: test rdx, rdx
14001f98b: je 0x14001e140
14001f991: mov rcx, qword ptr [rbp + 0x4d0]
14001f998: shl rdx, 2
14001f99c: mov r8d, 4
14001f9a2: call 0x140613c20 ; ?
14001f9a7: jmp 0x14001e140 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001f9ac: mov r12, rdx
14001f9af: mov rdx, qword ptr [rbp + 0x640]
14001f9b6: test rdx, rdx
14001f9b9: mov r13, qword ptr [rbp + 0x940]
14001f9c0: jne 0x14001f855
14001f9c6: jmp 0x14001f86b ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001f9cb: lea rcx, [rbp + 0x440]
14001f9d2: call 0x1405dec70 ; core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::ArchSegment,alloc::alloc::Global> >
14001f9d7: mov rbx, qword ptr [rbp + 0x88]
14001f9de: xor r14d, r14d
14001f9e1: mov rsi, qword ptr [rbp + 0x160]
14001f9e8: add rsi, 0x18
14001f9ec: mov rcx, rsi
14001f9ef: call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
14001f9f4: mov al, 1
14001f9f6: lock cmpxchg byte ptr [rsi], r14b
14001f9fb: jne 0x14001fa22
14001f9fd: mov rsi, qword ptr [rbp + 0x1b8]
14001fa04: add rsi, 0x18
14001fa08: mov rcx, rsi
14001fa0b: call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
14001fa10: mov al, 1
14001fa12: lock cmpxchg byte ptr [rsi], r14b
14001fa17: jne 0x14001fa2e
14001fa19: mov byte ptr [rbp + 0x790], 1
14001fa20: jmp 0x14001fa40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001fa22: mov rcx, rsi
14001fa25: xor edx, edx
14001fa27: call 0x1428cdb90 ; parking_lot::raw_mutex::RawMutex::unlock_slow
14001fa2c: jmp 0x14001f9fd ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001fa2e: mov rcx, rsi
14001fa31: xor edx, edx
14001fa33: call 0x1428cdb90 ; parking_lot::raw_mutex::RawMutex::unlock_slow
14001fa38: jmp 0x14001fa19 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001fa3a: nop word ptr [rax + rax]
14001fa40: cmp rbx, qword ptr [rbp + 0xf0]
14001fa47: je 0x14001fac5
14001fa49: mov rcx, rbx
14001fa4c: add rbx, 0x150
14001fa53: cmp byte ptr [rcx + 0x148], 1
14001fa5a: jne 0x14001fa40
14001fa5c: cmp r14, qword ptr [rcx + 0x98]
14001fa63: jo 0x14001fa40
14001fa65: mov rax, qword ptr [rbp + 0x328]
14001fa6c: cmp qword ptr [rax + 0x18], 0
14001fa71: je 0x14001fa40
14001fa73: mov qword ptr [rbp + 0x7e0], rcx
14001fa7a: lea rdx, [rcx + 0x128]
14001fa81: mov rcx, qword ptr [rbp + 0x108]
14001fa88: mov qword ptr [rbp + 0x670], rdx
14001fa8f: call 0x1405b1be0 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::resources::stairs::stairs_state::StairGraphEdge> >
14001fa94: mov rcx, qword ptr [rbp + 0x328]
14001fa9b: mov r8, qword ptr [rcx]
14001fa9e: mov r9, qword ptr [rcx + 8]
14001faa2: mov r10, r9
14001faa5: and r10, rax
14001faa8: shr rax, 0x39
14001faac: movd xmm0, eax
14001fab0: punpcklbw xmm0, xmm0
14001fab4: pshuflw xmm0, xmm0, 0
14001fab9: pshufd xmm0, xmm0, 0
14001fabe: xor eax, eax
14001fac0: jmp 0x140018640 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001fac5: mov rax, qword ptr [rbp + 0x360]
14001facc: mov rsi, qword ptr [rax + 8]
14001fad0: mov rax, qword ptr [rax + 0x10]
14001fad4: lea rdi, [rax + rax*4]
14001fad8: shl rdi, 5
14001fadc: add rdi, rsi
14001fadf: add qword ptr [rbp + 0x7b0], 0x168
14001faea: nop word ptr [rax + rax]
14001faf0: cmp rsi, rdi
14001faf3: je 0x14001fb21
14001faf5: mov ecx, dword ptr [rsi + 0x88]
14001fafb: mov edx, dword ptr [rsi + 0x8c]
14001fb01: call 0x140959e70 ; country_core::resources::stairs::stairs_assembly_params::impl$2::from
14001fb06: add rsi, 0xa0
14001fb0d: mov rcx, qword ptr [rbp + 0x7b0]
14001fb14: mov rdx, rax
14001fb17: mov r8b, 1
14001fb1a: call 0x1402c8dc0 ; hashbrown::map::HashMap::insert<country_core::resources::stairs::stairs_assembly_params::StairGraphEdgeIdentity,country_core::resources::stairs::stairs_assembly_params::StairArchWallSupportType,std::hash::random::RandomState,alloc::alloc::Global>
14001fb1f: jmp 0x14001faf0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001fb21: mov rax, qword ptr [rbp + 0x338]
14001fb28: mov rsi, qword ptr [rax + 8]
14001fb2c: imul rdi, qword ptr [rax + 0x10], 0xa8
14001fb34: add rdi, rsi
14001fb37: nop word ptr [rax + rax]
14001fb40: cmp rsi, rdi
14001fb43: je 0x14001fb71
14001fb45: mov ecx, dword ptr [rsi + 0x98]
14001fb4b: mov edx, dword ptr [rsi + 0x9c]
14001fb51: call 0x140959e70 ; country_core::resources::stairs::stairs_assembly_params::impl$2::from
14001fb56: add rsi, 0xa8
14001fb5d: mov rcx, qword ptr [rbp + 0x7b0]
14001fb64: mov rdx, rax
14001fb67: xor r8d, r8d
14001fb6a: call 0x1402c8dc0 ; hashbrown::map::HashMap::insert<country_core::resources::stairs::stairs_assembly_params::StairGraphEdgeIdentity,country_core::resources::stairs::stairs_assembly_params::StairArchWallSupportType,std::hash::random::RandomState,alloc::alloc::Global>
14001fb6f: jmp 0x14001fb40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001fb71: mov rax, qword ptr [rbp + 0x338]
14001fb78: mov r12, qword ptr [rax + 8]
14001fb7c: imul r14, qword ptr [rax + 0x10], 0xa8
14001fb84: add r14, r12
14001fb87: lea rsi, [rbp + 0x460]
14001fb8e: lea rdi, [rbp + 0x6c0]
14001fb95: lea rbx, [rbp + 0x380]
14001fb9c: xor r15d, r15d
14001fb9f: cmp r12, r14
14001fba2: jne 0x14001fbc0
14001fba4: jmp 0x1400200f5 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001fba9: nop dword ptr [rax]
14001fbb0: add r12, 0xa8
14001fbb7: cmp r12, r14
14001fbba: je 0x1400200f5
14001fbc0: lea rax, [r12 + 0x68]
14001fbc5: lea rcx, [r12 + 0x38]
14001fbca: mov rdx, qword ptr [rbp + 0x950]
14001fbd1: mov qword ptr [rbp + 0x460], rdx
14001fbd8: mov rdx, qword ptr [rbp + 0x7b0]
14001fbdf: mov qword ptr [rbp + 0x468], rdx
14001fbe6: mov rdx, qword ptr [rbp + 0x248]
14001fbed: mov qword ptr [rbp + 0x470], rdx
14001fbf4: mov rdx, qword ptr [rbp + 0x2d0]
14001fbfb: mov qword ptr [rbp + 0x478], rdx
14001fc02: mov qword ptr [rbp + 0x380], r12
14001fc09: mov qword ptr [rbp + 0x388], rax
14001fc10: mov qword ptr [rbp + 0x390], rcx
14001fc17: cmp byte ptr [r12 + 0xa4], 0
14001fc20: je 0x14001fe80
14001fc26: mov edx, dword ptr [r12 + 0x98]
14001fc2e: mov r8d, dword ptr [r12 + 0x9c]
14001fc36: mov rcx, rsi
14001fc39: mov r9b, 1
14001fc3c: call 0x1400286d0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$24<false>
14001fc41: test al, al
14001fc43: je 0x14001fe80
14001fc49: mov rcx, rdi
14001fc4c: mov rdx, rbx
14001fc4f: mov r8b, 1
14001fc52: call 0x140027840 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$25<false>
14001fc57: cmp r15, qword ptr [rbp + 0x6c0]
14001fc5e: jo 0x14001fe80
14001fc64: mov rax, qword ptr [rbp + 0x710]
14001fc6b: mov qword ptr [rbp + 0x808], rax
14001fc72: mov rax, qword ptr [rbp + 0x718]
14001fc79: mov qword ptr [rbp + 0x6b0], rax
14001fc80: mov rdx, qword ptr [rbp + 0x720]
14001fc87: mov rax, qword ptr [rbp + 0x330]
14001fc8e: mov ecx, dword ptr [rbp + 0x534]
14001fc94: mov dword ptr [rax], ecx
14001fc96: mov rax, qword ptr [rbp + 0x6f0]
14001fc9d: mov qword ptr [rbp + 0x5c0], rax
14001fca4: movups xmm0, xmmword ptr [rbp + 0x6c0]
14001fcab: movdqu xmm1, xmmword ptr [rbp + 0x6d0]
14001fcb3: movdqu xmm2, xmmword ptr [rbp + 0x6e0]
14001fcbb: movdqa xmmword ptr [rbp + 0x5b0], xmm2
14001fcc3: movdqa xmmword ptr [rbp + 0x5a0], xmm1
14001fccb: movaps xmmword ptr [rbp + 0x590], xmm0
14001fcd2: mov rax, qword ptr [rbp + 0x6f8]
14001fcd9: mov qword ptr [rbp + 0x7a0], rax
14001fce0: mov rax, qword ptr [rbp + 0x700]
14001fce7: mov qword ptr [rbp + 0x7d0], rax
14001fcee: mov r15, qword ptr [rbp + 0x708]
14001fcf5: test rdx, rdx
14001fcf8: mov qword ptr [rbp + 0x7f0], rdx
14001fcff: je 0x14001fd27
14001fd01: lea r13, [rdx*4]
14001fd09: movzx eax, byte ptr [rip + 0x3614ef9]
14001fd10: mov edx, 4
14001fd15: mov rcx, r13
14001fd18: call 0x140613c40 ; ?
14001fd1d: test rax, rax
14001fd20: jne 0x14001fd2c
14001fd22: jmp 0x140020a88 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001fd27: mov eax, 4
14001fd2c: mov qword ptr [rbp + 0x7d8], rax
14001fd33: mov ecx, dword ptr [r12 + 0x98]
14001fd3b: mov edx, dword ptr [r12 + 0x9c]
14001fd43: call 0x140959e70 ; country_core::resources::stairs::stairs_assembly_params::impl$2::from
14001fd48: mov ecx, dword ptr [r12 + 0xa0]
14001fd50: mov rdx, qword ptr [rbp + 0x5c0]
14001fd57: mov qword ptr [rbp + 0x170], rdx
14001fd5e: movaps xmm0, xmmword ptr [rbp + 0x590]
14001fd65: movdqa xmm1, xmmword ptr [rbp + 0x5a0]
14001fd6d: movdqa xmm2, xmmword ptr [rbp + 0x5b0]
14001fd75: movdqa xmmword ptr [rbp + 0x160], xmm2
14001fd7d: movdqa xmmword ptr [rbp + 0x150], xmm1
14001fd85: movaps xmmword ptr [rbp + 0x140], xmm0
14001fd8c: mov rdx, qword ptr [rbp + 0x7a0]
14001fd93: mov qword ptr [rbp + 0x178], rdx
14001fd9a: mov rdx, qword ptr [rbp + 0x7d0]
14001fda1: mov qword ptr [rbp + 0x180], rdx
14001fda8: mov qword ptr [rbp + 0x188], r15
14001fdaf: mov rdx, qword ptr [rbp + 0x808]
14001fdb6: mov qword ptr [rbp + 0x190], rdx
14001fdbd: mov rdx, qword ptr [rbp + 0x6b0]
14001fdc4: mov qword ptr [rbp + 0x198], rdx
14001fdcb: mov rdx, qword ptr [rbp + 0x7f0]
14001fdd2: mov qword ptr [rbp + 0x1a0], rdx
14001fdd9: mov qword ptr [rbp + 0x1a8], rdx
14001fde0: mov r8, qword ptr [rbp + 0x7d8]
14001fde7: mov qword ptr [rbp + 0x1b0], r8
14001fdee: mov qword ptr [rbp + 0x1b8], rdx
14001fdf5: mov qword ptr [rbp + 0x1c0], rax
14001fdfc: mov word ptr [rbp + 0x1d4], 0
14001fe05: mov dword ptr [rbp + 0x1d0], ecx
14001fe0b: mov rax, qword ptr [r12 + 0x98]
14001fe13: mov qword ptr [rbp + 0x1c8], rax
14001fe1a: mov dword ptr [rbp + 0x1d6], 0
14001fe24: mov word ptr [rbp + 0x1da], 0x100
14001fe2d: mov r13, qword ptr [rbp + 0x360]
14001fe34: mov r15, qword ptr [r13 + 0x10]
14001fe38: cmp r15, qword ptr [r13]
14001fe3c: jne 0x14001fe4d
14001fe3e: mov rcx, r13
14001fe41: lea rdx, [rip + 0x29066e8]
14001fe48: call 0x140563480 ; alloc::raw_vec::RawVec::grow_one<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams,alloc::alloc::Global>
14001fe4d: lea rcx, [r15 + r15*4]
14001fe51: shl rcx, 5
14001fe55: add rcx, qword ptr [r13 + 8]
14001fe59: mov r8d, 0xa0
14001fe5f: lea rdx, [rbp + 0x140]
14001fe66: call 0x142923c10 ; ?
14001fe6b: inc r15
14001fe6e: mov qword ptr [r13 + 0x10], r15
14001fe72: mov r13, qword ptr [rbp + 0x940]
14001fe79: xor r15d, r15d
14001fe7c: nop dword ptr [rax]
14001fe80: cmp byte ptr [r12 + 0xa5], 0
14001fe89: je 0x14001fbb0
14001fe8f: mov r8d, dword ptr [r12 + 0x98]
14001fe97: mov edx, dword ptr [r12 + 0x9c]
14001fe9f: mov rcx, rsi
14001fea2: xor r9d, r9d
14001fea5: call 0x1400286d0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$24<false>
14001feaa: test al, al
14001feac: je 0x14001fbb0
14001feb2: mov rcx, rdi
14001feb5: mov rdx, rbx
14001feb8: xor r8d, r8d
14001febb: call 0x140027840 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$25<false>
14001fec0: cmp r15, qword ptr [rbp + 0x6c0]
14001fec7: jo 0x14001fbb0
14001fecd: mov rax, qword ptr [rbp + 0x710]
14001fed4: mov qword ptr [rbp + 0x808], rax
14001fedb: mov rax, qword ptr [rbp + 0x718]
14001fee2: mov qword ptr [rbp + 0x6b0], rax
14001fee9: mov rdx, qword ptr [rbp + 0x720]
14001fef0: mov rax, qword ptr [rbp + 0x330]
14001fef7: mov ecx, dword ptr [rbp + 0x534]
14001fefd: mov dword ptr [rax], ecx
14001feff: mov rax, qword ptr [rbp + 0x6f0]
14001ff06: mov qword ptr [rbp + 0x5c0], rax
14001ff0d: movups xmm0, xmmword ptr [rbp + 0x6c0]
14001ff14: movdqu xmm1, xmmword ptr [rbp + 0x6d0]
14001ff1c: movdqu xmm2, xmmword ptr [rbp + 0x6e0]
14001ff24: movdqa xmmword ptr [rbp + 0x5b0], xmm2
14001ff2c: movdqa xmmword ptr [rbp + 0x5a0], xmm1
14001ff34: movaps xmmword ptr [rbp + 0x590], xmm0
14001ff3b: mov rax, qword ptr [rbp + 0x6f8]
14001ff42: mov qword ptr [rbp + 0x7a0], rax
14001ff49: mov rax, qword ptr [rbp + 0x700]
14001ff50: mov qword ptr [rbp + 0x7d0], rax
14001ff57: mov r15, qword ptr [rbp + 0x708]
14001ff5e: test rdx, rdx
14001ff61: mov qword ptr [rbp + 0x7f0], rdx
14001ff68: je 0x14001ff90
14001ff6a: lea r13, [rdx*4]
14001ff72: movzx eax, byte ptr [rip + 0x3614c90]
14001ff79: mov edx, 4
14001ff7e: mov rcx, r13
14001ff81: call 0x140613c40 ; ?
14001ff86: test rax, rax
14001ff89: jne 0x14001ff95
14001ff8b: jmp 0x140020aa8 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14001ff90: mov eax, 4
14001ff95: mov qword ptr [rbp + 0x7d8], rax
14001ff9c: mov ecx, dword ptr [r12 + 0x98]
14001ffa4: mov edx, dword ptr [r12 + 0x9c]
14001ffac: call 0x140959e70 ; country_core::resources::stairs::stairs_assembly_params::impl$2::from
14001ffb1: mov ecx, dword ptr [r12 + 0xa0]
14001ffb9: mov rdx, qword ptr [rbp + 0x5c0]
14001ffc0: mov qword ptr [rbp + 0x170], rdx
14001ffc7: movaps xmm0, xmmword ptr [rbp + 0x590]
14001ffce: movdqa xmm1, xmmword ptr [rbp + 0x5a0]
14001ffd6: movdqa xmm2, xmmword ptr [rbp + 0x5b0]
14001ffde: movdqa xmmword ptr [rbp + 0x160], xmm2
14001ffe6: movdqa xmmword ptr [rbp + 0x150], xmm1
14001ffee: movaps xmmword ptr [rbp + 0x140], xmm0
14001fff5: mov rdx, qword ptr [rbp + 0x7a0]
14001fffc: mov qword ptr [rbp + 0x178], rdx
140020003: mov rdx, qword ptr [rbp + 0x7d0]
14002000a: mov qword ptr [rbp + 0x180], rdx
140020011: mov qword ptr [rbp + 0x188], r15
140020018: mov rdx, qword ptr [rbp + 0x808]
14002001f: mov qword ptr [rbp + 0x190], rdx
140020026: mov rdx, qword ptr [rbp + 0x6b0]
14002002d: mov qword ptr [rbp + 0x198], rdx
140020034: mov rdx, qword ptr [rbp + 0x7f0]
14002003b: mov qword ptr [rbp + 0x1a0], rdx
140020042: mov qword ptr [rbp + 0x1a8], rdx
140020049: mov r8, qword ptr [rbp + 0x7d8]
140020050: mov qword ptr [rbp + 0x1b0], r8
140020057: mov qword ptr [rbp + 0x1b8], rdx
14002005e: mov qword ptr [rbp + 0x1c0], rax
140020065: mov word ptr [rbp + 0x1d4], 0
14002006e: mov dword ptr [rbp + 0x1d0], ecx
140020074: mov rax, qword ptr [r12 + 0x98]
14002007c: mov qword ptr [rbp + 0x1c8], rax
140020083: mov dword ptr [rbp + 0x1d6], 0
14002008d: mov word ptr [rbp + 0x1da], 0x200
140020096: mov r13, qword ptr [rbp + 0x360]
14002009d: mov r15, qword ptr [r13 + 0x10]
1400200a1: cmp r15, qword ptr [r13]
1400200a5: jne 0x1400200b6
1400200a7: mov rcx, r13
1400200aa: lea rdx, [rip + 0x29064af]
1400200b1: call 0x140563480 ; alloc::raw_vec::RawVec::grow_one<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams,alloc::alloc::Global>
1400200b6: lea rcx, [r15 + r15*4]
1400200ba: shl rcx, 5
1400200be: add rcx, qword ptr [r13 + 8]
1400200c2: mov r8d, 0xa0
1400200c8: lea rdx, [rbp + 0x140]
1400200cf: call 0x142923c10 ; ?
1400200d4: inc r15
1400200d7: mov qword ptr [r13 + 0x10], r15
1400200db: mov r13, qword ptr [rbp + 0x940]
1400200e2: xor r15d, r15d
1400200e5: add r12, 0xa8
1400200ec: cmp r12, r14
1400200ef: jne 0x14001fbc0
1400200f5: test byte ptr [rbp + 0x790], 1
1400200fc: je 0x14002026f
140020102: mov r8, qword ptr [rbp + 0x2e0]
140020109: mov rax, qword ptr [r8]
14002010c: mov rcx, qword ptr [r8 + 8]
140020110: add rcx, rax
140020113: inc rcx
140020116: movdqa xmm0, xmmword ptr [rax]
14002011a: pmovmskb edx, xmm0
14002011e: not edx
140020120: mov r8, qword ptr [r8 + 0x18]
140020124: mov qword ptr [rbp + 0x140], 0
14002012f: mov qword ptr [rbp + 0x150], 0
14002013a: mov qword ptr [rbp + 0x160], rax
140020141: add rax, 0x10
140020145: mov qword ptr [rbp + 0x168], rax
14002014c: mov qword ptr [rbp + 0x170], rcx
140020153: mov word ptr [rbp + 0x178], dx
14002015a: mov qword ptr [rbp + 0x180], r8
140020161: lea r8, [rip + 0x29071c8]
140020168: lea rcx, [rbp + 0x590]
14002016f: lea rdx, [rbp + 0x140]
140020176: call 0x1402bcd50 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::sse2::vec4::Vec4> >,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$26<false> > >
14002017b: mov rax, qword ptr [rbp + 0x598]
140020182: mov qword ptr [rbp + 0x808], rax
140020189: mov r14, qword ptr [rbp + 0x5a0]
140020190: test r14, r14
140020193: je 0x1400202bb
140020199: mov rcx, qword ptr [rbp + 0x268]
1400201a0: call 0x140987ca0 ; country_core::render::impl$0::deref
1400201a5: mov rsi, rax
1400201a8: mov rdi, r14
1400201ab: shl rdi, 4
1400201af: lea rbx, [rbp + 0x6c0]
1400201b6: mov rcx, rbx
1400201b9: mov rdx, rdi
1400201bc: mov r8d, 0x20
1400201c2: call 0x140bd4e20 ; rhapsody::raw::buffer::BufferDesc::new_gpu_only
1400201c7: mov qword ptr [rsp + 0x30], rdi
1400201cc: mov rax, qword ptr [rbp + 0x808]
1400201d3: mov qword ptr [rsp + 0x28], rax
1400201d8: mov qword ptr [rsp + 0x20], rbx
1400201dd: lea r8, [rip + 0x290630c]
1400201e4: lea rcx, [rbp + 0x140]
1400201eb: mov r9d, 0xf
1400201f1: mov rdx, rsi
1400201f4: call 0x1405b93b0 ; rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > >
1400201f9: cmp dword ptr [rbp + 0x140], 0xb
140020200: jne 0x1400202f5
140020206: mov rax, qword ptr [rbp + 0x168]
14002020d: mov qword ptr [rbp + 0x6e0], rax
140020214: movdqu xmm0, xmmword ptr [rbp + 0x148]
14002021c: movdqu xmm1, xmmword ptr [rbp + 0x158]
140020224: movdqa xmmword ptr [rbp + 0x6d0], xmm1
14002022c: movdqa xmmword ptr [rbp + 0x6c0], xmm0
140020234: mov dword ptr [rbp + 0x6e8], r14d
14002023b: lea r8, [rip + 0x29062be]
140020242: lea rdx, [rbp + 0x6c0]
140020249: mov rcx, r13
14002024c: call 0x14014dd30 ; bevy_ecs::system::commands::Commands::insert_resource<country_core::resources::stairs::stairs_top_down::StairsTopDownMesh>
140020251: cmp dword ptr [rbp + 0x140], 0xb
140020258: je 0x140020301
14002025e: lea rcx, [rbp + 0x140]
140020265: call 0x1400297d0 ; core::ptr::drop_in_place<enum2$<core::result::Result<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError> > > >
14002026a: jmp 0x140020301 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14002026f: mov rax, qword ptr [rbp + 0x2e0]
140020276: cmp qword ptr [rax + 0x18], 0
14002027b: jne 0x140020323
140020281: mov rsi, qword ptr [r13 + 8]
140020285: test rsi, rsi
140020288: jne 0x14002028e
14002028a: mov rsi, qword ptr [r13 + 0x10]
14002028e: mov rax, qword ptr [rsi]
140020291: mov rdi, qword ptr [rsi + 0x10]
140020295: sub rax, rdi
140020298: cmp rax, 7
14002029c: jbe 0x14002071c
1400202a2: mov rax, qword ptr [rsi + 8]
1400202a6: lea rcx, [rip + 0x12d313]
1400202ad: mov qword ptr [rax + rdi], rcx
1400202b1: add rdi, 8
1400202b5: mov qword ptr [rsi + 0x10], rdi
1400202b9: jmp 0x140020323 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400202bb: mov rsi, qword ptr [r13 + 8]
1400202bf: test rsi, rsi
1400202c2: jne 0x1400202c8
1400202c4: mov rsi, qword ptr [r13 + 0x10]
1400202c8: mov rax, qword ptr [rsi]
1400202cb: mov rdi, qword ptr [rsi + 0x10]
1400202cf: sub rax, rdi
1400202d2: cmp rax, 7
1400202d6: jbe 0x140020741
1400202dc: mov rax, qword ptr [rsi + 8]
1400202e0: lea rcx, [rip + 0x12d2d9]
1400202e7: mov qword ptr [rax + rdi], rcx
1400202eb: add rdi, 8
1400202ef: mov qword ptr [rsi + 0x10], rdi
1400202f3: jmp 0x140020301 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400202f5: lea rcx, [rbp + 0x140]
1400202fc: call 0x1400297d0 ; core::ptr::drop_in_place<enum2$<core::result::Result<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError> > > >
140020301: mov rdx, qword ptr [rbp + 0x590]
140020308: test rdx, rdx
14002030b: je 0x140020323
14002030d: shl rdx, 4
140020311: mov r8d, 0x10
140020317: mov rcx, qword ptr [rbp + 0x808]
14002031e: call 0x140613c20 ; ?
140020323: cmp byte ptr [rbp + 0x79f], 0
14002032a: je 0x140020377
14002032c: call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
140020331: mov rcx, rax
140020334: mov rax, qword ptr [rax]
140020337: cmp rax, 1
14002033b: jne 0x140020478
140020341: add rcx, 8
140020345: mov rdx, qword ptr [rbp + 0x270]
14002034c: cmp qword ptr [rcx], 0
140020350: jne 0x1400206e0
140020356: mov qword ptr [rcx], 0xffffffffffffffff
14002035d: mov qword ptr [rbp + 0x808], rcx
140020364: add rcx, 8
140020368: call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
14002036d: mov rax, qword ptr [rbp + 0x808]
140020374: inc qword ptr [rax]
140020377: movaps xmm6, xmmword ptr [rbp + 0x820]
14002037e: movaps xmm7, xmmword ptr [rbp + 0x830]
140020385: movaps xmm8, xmmword ptr [rbp + 0x840]
14002038d: movaps xmm9, xmmword ptr [rbp + 0x850]
140020395: movaps xmm10, xmmword ptr [rbp + 0x860]
14002039d: movaps xmm11, xmmword ptr [rbp + 0x870]
1400203a5: movaps xmm12, xmmword ptr [rbp + 0x880]
1400203ad: movaps xmm13, xmmword ptr [rbp + 0x890]
1400203b5: movaps xmm14, xmmword ptr [rbp + 0x8a0]
1400203bd: movaps xmm15, xmmword ptr [rbp + 0x8b0]
1400203c5: add rsp, 0x948
1400203cc: pop rbx
1400203cd: pop rdi
1400203ce: pop rsi
1400203cf: pop r12
1400203d1: pop r13
1400203d3: pop r14
1400203d5: pop r15
1400203d7: pop rbp
1400203d8: ret 
1400203d9: mov byte ptr [rbp + 0x817], 1
1400203e0: lea rcx, [rip + 0x29063c1]
1400203e7: call 0x1428d98a0 ; core::slice::index::slice_end_index_overflow_fail
1400203ec: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400203f1: mov byte ptr [rbp + 0x817], 1
1400203f8: lea r8, [rip + 0x29063a9]
1400203ff: mov rdx, rcx
140020402: mov rcx, rdi
140020405: call 0x1428d9850 ; core::slice::index::slice_index_order_fail
14002040a: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14002040f: mov byte ptr [rbp + 0x817], 1
140020416: lea r8, [rip + 0x290638b]
14002041d: mov rdx, qword ptr [rbp + 0x780]
140020424: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
140020429: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14002042e: mov byte ptr [rbp + 0x817], 1
140020435: lea r8, [rip + 0x2906384]
14002043c: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
140020441: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020446: mov byte ptr [rbp + 0x817], 1
14002044d: lea r8, [rip + 0x2906384]
140020454: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
140020459: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14002045e: test rax, rax
140020461: jne 0x14002049d
140020463: xor edx, edx
140020465: call 0x1428a1640 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
14002046a: mov rcx, rax
14002046d: test rax, rax
140020470: jne 0x140018368
140020476: jmp 0x14002049d ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020478: test rax, rax
14002047b: mov rax, qword ptr [rbp + 0x270]
140020482: jne 0x14002049d
140020484: mov rsi, rax
140020487: xor edx, edx
140020489: call 0x1428a1640 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
14002048e: mov rdx, rsi
140020491: mov rcx, rax
140020494: test rax, rax
140020497: jne 0x14002034c
14002049d: lea rcx, [rip + 0x291256c]
1400204a4: call 0x1428d80d0 ; std::thread::local::panic_access_error
1400204a9: mov byte ptr [rbp + 0x817], 1
1400204b0: lea r8, [rip + 0x2906339]
1400204b7: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
1400204bc: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400204c1: lea rcx, [rip + 0x2906238]
1400204c8: call 0x1428d98a0 ; core::slice::index::slice_end_index_overflow_fail
1400204cd: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400204d2: lea r8, [rip + 0x2906227]
1400204d9: mov rcx, r13
1400204dc: mov rdx, rax
1400204df: call 0x1428d9850 ; core::slice::index::slice_index_order_fail
1400204e4: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400204e9: lea r8, [rip + 0x2906210]
1400204f0: mov rcx, rax
1400204f3: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
1400204f8: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400204fd: mov byte ptr [rbp + 0x813], 1
140020504: lea rcx, [rip + 0x290617d]
14002050b: call 0x1428d98a0 ; core::slice::index::slice_end_index_overflow_fail
140020510: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020515: mov byte ptr [rbp + 0x813], 1
14002051c: lea r8, [rip + 0x2906165]
140020523: mov rcx, rbx
140020526: mov rdx, rax
140020529: call 0x1428d9850 ; core::slice::index::slice_index_order_fail
14002052e: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020533: mov byte ptr [rbp + 0x813], 1
14002053a: lea r8, [rip + 0x2906147]
140020541: mov rcx, rax
140020544: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
140020549: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14002054e: mov byte ptr [rbp + 0x816], 1
140020555: mov byte ptr [rbp + 0x815], 1
14002055c: lea rcx, [rip + 0x29064e5]
140020563: call 0x1428d98a0 ; core::slice::index::slice_end_index_overflow_fail
140020568: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14002056d: mov byte ptr [rbp + 0x816], 1
140020574: mov byte ptr [rbp + 0x815], 1
14002057b: lea r8, [rip + 0x29064c6]
140020582: mov rcx, qword ptr [rbp + 0x808]
140020589: mov rdx, r9
14002058c: call 0x1428d9850 ; core::slice::index::slice_index_order_fail
140020591: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020596: mov byte ptr [rbp + 0x816], 1
14002059d: mov byte ptr [rbp + 0x815], 1
1400205a4: lea r8, [rip + 0x290649d]
1400205ab: mov rcx, r9
1400205ae: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
1400205b3: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400205b8: mov qword ptr [rbp + 0x6c0], 0
1400205c3: mov byte ptr [rbp + 0x817], 1
1400205ca: lea rax, [rip + 0x290623f]
1400205d1: mov qword ptr [rsp + 0x20], rax
1400205d6: lea r8, [rip + 0x290622b]
1400205dd: lea rdx, [rbp + 0x78]
1400205e1: lea r9, [rbp + 0x6c0]
1400205e8: xor ecx, ecx
1400205ea: call 0x1428d9596 ; core::panicking::assert_failed<usize,usize>
1400205ef: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400205f4: mov qword ptr [rbp + 0x140], 0
1400205ff: lea rax, [rip + 0x298db92]
140020606: mov qword ptr [rsp + 0x20], rax
14002060b: lea rdx, [rbp + 0x6c0]
140020612: lea r9, [rbp + 0x140]
140020619: xor ecx, ecx
14002061b: mov r8, r14
14002061e: call 0x1428cab41 ; core::panicking::assert_failed<ref$<str$>,ref$<str$> >
140020623: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020628: mov qword ptr [rbp + 0x140], 0
140020633: lea rax, [rip + 0x298db5e]
14002063a: mov qword ptr [rsp + 0x20], rax
14002063f: lea rdx, [rbp + 0x6c0]
140020646: lea r9, [rbp + 0x140]
14002064d: xor ecx, ecx
14002064f: mov r8, r14
140020652: call 0x1428cab41 ; core::panicking::assert_failed<ref$<str$>,ref$<str$> >
140020657: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14002065c: mov qword ptr [rbp + 0x140], 0
140020667: lea rax, [rip + 0x298db2a]
14002066e: mov qword ptr [rsp + 0x20], rax
140020673: lea rdx, [rbp + 0x6c0]
14002067a: lea r9, [rbp + 0x140]
140020681: xor ecx, ecx
140020683: mov r8, rbx
140020686: call 0x1428cab41 ; core::panicking::assert_failed<ref$<str$>,ref$<str$> >
14002068b: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020690: lea r8, [rip + 0x2905d71]
140020697: mov rcx, r12
14002069a: call 0x1428d9518 ; core::panicking::panic_bounds_check
14002069f: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400206a4: lea r8, [rip + 0x2905ce5]
1400206ab: mov rcx, rbx
1400206ae: call 0x1428d9518 ; core::panicking::panic_bounds_check
1400206b3: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400206b8: lea r8, [rip + 0x2905d49]
1400206bf: mov rcx, rbx
1400206c2: call 0x1428d9518 ; core::panicking::panic_bounds_check
1400206c7: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400206cc: lea r8, [rip + 0x2905cbd]
1400206d3: mov rcx, r12
1400206d6: call 0x1428d9518 ; core::panicking::panic_bounds_check
1400206db: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400206e0: lea rcx, [rip + 0x29123d1]
1400206e7: call 0x1428d9250 ; core::cell::panic_already_borrowed
1400206ec: lea r8, [rip + 0x2906025]
1400206f3: mov rcx, qword ptr [rbp + 0x780]
1400206fa: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
1400206ff: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020704: lea r8, [rip + 0x2906025]
14002070b: mov rcx, qword ptr [rbp + 0x780]
140020712: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
140020717: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14002071c: mov qword ptr [rsp + 0x20], 1
140020725: mov r8d, 8
14002072b: mov r9d, 1
140020731: mov rcx, rsi
140020734: mov rdx, rdi
140020737: call 0x1428a29e0 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
14002073c: jmp 0x1400202a2 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020741: mov qword ptr [rsp + 0x20], 1
14002074a: mov r8d, 8
140020750: mov r9d, 1
140020756: mov rcx, rsi
140020759: mov rdx, rdi
14002075c: call 0x1428a29e0 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140020761: jmp 0x1400202dc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020766: lea r8, [rip + 0x2905f33]
14002076d: mov rcx, qword ptr [rbp + 0x678]
140020774: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
140020779: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14002077e: lea r8, [rip + 0x2905f33]
140020785: mov rcx, qword ptr [rbp + 0x678]
14002078c: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
140020791: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020796: lea r8, [rip + 0x29062c3]
14002079d: mov rcx, qword ptr [rbp + 0x6b0]
1400207a4: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
1400207a9: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400207ae: lea r8, [rip + 0x29062c3]
1400207b5: mov rcx, qword ptr [rbp + 0x6b0]
1400207bc: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
1400207c1: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400207c6: lea r8, [rip + 0x29062c3]
1400207cd: mov rcx, qword ptr [rbp + 0x6b0]
1400207d4: mov rdx, r12
1400207d7: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
1400207dc: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400207e1: mov byte ptr [rbp + 0x803], 1
1400207e8: mov byte ptr [rbp + 0x802], 1
1400207ef: mov byte ptr [rbp + 0x801], 1
1400207f6: mov byte ptr [rbp + 0x800], 0
1400207fd: mov byte ptr [rbp + 0x7ff], 0
140020804: lea rcx, [rip + 0x2905dd5]
14002080b: call 0x1428d9310 ; core::option::unwrap_failed
140020810: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020815: xor edi, edi
140020817: mov qword ptr [rbp + 0xd0], rbx
14002081e: mov qword ptr [rbp + 0xc8], rdi
140020825: lea r8, [rip + 0x2905f1c]
14002082c: mov rcx, qword ptr [rbp + 0x780]
140020833: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
140020838: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14002083d: xor r12d, r12d
140020840: mov byte ptr [rbp + 0x817], 1
140020847: lea r8, [rip + 0x29936e2]
14002084e: mov rcx, r12
140020851: mov rdx, rbx
140020854: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020859: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14002085e: mov qword ptr [rbp + 0x370], rax
140020865: xor edi, edi
140020867: mov rax, qword ptr [rbp + 0x370]
14002086e: mov qword ptr [rbp + 0xe0], rax
140020875: mov qword ptr [rbp + 0xd8], rdi
14002087c: lea r8, [rip + 0x2905e4d]
140020883: mov rcx, qword ptr [rbp + 0x678]
14002088a: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
14002088f: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020894: xor esi, esi
140020896: lea r8, [rip + 0x298db9b]
14002089d: mov rcx, rsi
1400208a0: mov rdx, r12
1400208a3: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1400208a8: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400208ad: lea rax, [rbp + 0x568]
1400208b4: mov qword ptr [rbp + 0x590], rax
1400208bb: lea rax, [rip + 0x967e]
1400208c2: mov qword ptr [rbp + 0x598], rax
1400208c9: lea rax, [rip + 0x2905b70]
1400208d0: mov qword ptr [rbp + 0x6c0], rax
1400208d7: mov qword ptr [rbp + 0x6c8], 1
1400208e2: mov qword ptr [rbp + 0x6e0], 0
1400208ed: lea rax, [rbp + 0x590]
1400208f4: mov qword ptr [rbp + 0x6d0], rax
1400208fb: mov qword ptr [rbp + 0x6d8], 1
140020906: lea rdx, [rip + 0x2905b43]
14002090d: lea rcx, [rbp + 0x6c0]
140020914: call 0x1428d9390 ; core::panicking::panic_fmt
140020919: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14002091e: xor r14d, r14d
140020921: mov qword ptr [rbp + 0x808], rdi
140020928: lea r8, [rip + 0x298db09]
14002092f: mov rcx, r14
140020932: mov rdx, r12
140020935: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
14002093a: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14002093f: lea r8, [rip + 0x29059ea]
140020946: mov ecx, 4
14002094b: mov rdx, rsi
14002094e: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020953: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020958: mov byte ptr [rbp + 0x814], 1
14002095f: lea rcx, [rip + 0x2905cf2]
140020966: call 0x1428d9310 ; core::option::unwrap_failed
14002096b: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020970: mov byte ptr [rbp + 0x814], 1
140020977: lea rcx, [rip + 0x2905cf2]
14002097e: call 0x1428d9310 ; core::option::unwrap_failed
140020983: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020988: xor esi, esi
14002098a: lea r8, [rip + 0x2906a07]
140020991: mov rcx, rsi
140020994: mov rdx, r15
140020997: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
14002099c: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400209a1: lea r8, [rip + 0x29b4270]
1400209a8: mov ecx, 4
1400209ad: mov rdx, rbx
1400209b0: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1400209b5: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400209ba: lea r8, [rip + 0x2905987]
1400209c1: mov ecx, 4
1400209c6: mov rdx, rsi
1400209c9: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1400209ce: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400209d3: lea r8, [rip + 0x29069be]
1400209da: mov ecx, 4
1400209df: mov edx, 0x30
1400209e4: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1400209e9: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400209ee: lea r8, [rip + 0x290596b]
1400209f5: mov ecx, 4
1400209fa: mov rdx, rsi
1400209fd: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020a02: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020a07: lea r8, [rip + 0x290596a]
140020a0e: mov ecx, 4
140020a13: mov rdx, rsi
140020a16: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020a1b: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020a20: lea rcx, [rip + 0x2905b51]
140020a27: call 0x1428d9310 ; core::option::unwrap_failed
140020a2c: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020a31: xor esi, esi
140020a33: mov byte ptr [rbp + 0x813], 1
140020a3a: lea r8, [rip + 0x2906957]
140020a41: mov rcx, rsi
140020a44: mov rdx, qword ptr [rbp + 0xf8]
140020a4b: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020a50: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020a55: lea rcx, [rip + 0x2905a4c]
140020a5c: call 0x1428d9310 ; core::option::unwrap_failed
140020a61: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020a66: lea rcx, [rip + 0x2905a53]
140020a6d: call 0x1428d9310 ; core::option::unwrap_failed
140020a72: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020a77: lea rcx, [rip + 0x2905a5a]
140020a7e: call 0x1428d9310 ; core::option::unwrap_failed
140020a83: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020a88: mov byte ptr [rbp + 0x79e], 1
140020a8f: lea r8, [rip + 0x2905a82]
140020a96: mov ecx, 4
140020a9b: mov rdx, r13
140020a9e: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020aa3: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020aa8: mov byte ptr [rbp + 0x79d], 1
140020aaf: lea r8, [rip + 0x2905a92]
140020ab6: mov ecx, 4
140020abb: mov rdx, r13
140020abe: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020ac3: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020ac8: xor esi, esi
140020aca: mov rbx, qword ptr [rbp + 0x100]
140020ad1: mov byte ptr [rbp + 0x816], 1
140020ad8: mov byte ptr [rbp + 0x815], 1
140020adf: lea r8, [rip + 0x29068b2]
140020ae6: mov rcx, rsi
140020ae9: mov rdx, rbx
140020aec: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020af1: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020af6: mov byte ptr [rbp + 0x817], 1
140020afd: lea r8, [rip + 0x2905fbc]
140020b04: xor ecx, ecx
140020b06: xor edx, edx
140020b08: call 0x1428d9518 ; core::panicking::panic_bounds_check
140020b0d: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020b12: mov byte ptr [rbp + 0x817], 1
140020b19: lea r8, [rip + 0x2905fb8]
140020b20: mov ecx, 1
140020b25: mov edx, 1
140020b2a: call 0x1428d9518 ; core::panicking::panic_bounds_check
140020b2f: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020b34: mov ecx, 0x10
140020b39: mov edx, 0x90
140020b3e: call 0x1428d9000 ; alloc::alloc::handle_alloc_error
140020b43: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020b48: mov byte ptr [rbp + 0x805], 1
140020b4f: lea rcx, [rip + 0x2911eba]
140020b56: call 0x1428d80d0 ; std::thread::local::panic_access_error
140020b5b: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020b60: mov byte ptr [rbp + 0x817], 1
140020b67: lea r8, [rip + 0x290682a]
140020b6e: mov ecx, 4
140020b73: mov edx, 0x18
140020b78: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020b7d: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020b82: mov byte ptr [rbp + 0x817], 1
140020b89: lea r8, [rip + 0x292d7f0]
140020b90: mov rcx, rdx
140020b93: mov rdx, rdi
140020b96: call 0x1428d9518 ; core::panicking::panic_bounds_check
140020b9b: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020ba0: mov byte ptr [rbp + 0x817], 1
140020ba7: lea r8, [rip + 0x292d7d2]
140020bae: mov rcx, rdx
140020bb1: mov rdx, rdi
140020bb4: call 0x1428d9518 ; core::panicking::panic_bounds_check
140020bb9: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020bbe: mov byte ptr [rbp + 0x816], 1
140020bc5: mov byte ptr [rbp + 0x815], 1
140020bcc: lea r8, [rip + 0x2905e45]
140020bd3: xor ecx, ecx
140020bd5: xor edx, edx
140020bd7: call 0x1428d9518 ; core::panicking::panic_bounds_check
140020bdc: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020be1: mov byte ptr [rbp + 0x816], 1
140020be8: mov byte ptr [rbp + 0x815], 1
140020bef: lea rcx, [rip + 0x2905e3a]
140020bf6: call 0x1428d9310 ; core::option::unwrap_failed
140020bfb: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020c00: xor esi, esi
140020c02: jmp 0x140020ca4 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020c07: mov byte ptr [rbp + 0x816], 1
140020c0e: mov byte ptr [rbp + 0x815], 1
140020c15: lea rcx, [rip + 0x2905cb4]
140020c1c: lea r8, [rip + 0x2905cd5]
140020c23: mov edx, 0x28
140020c28: call 0x1428d9430 ; core::panicking::panic
140020c2d: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020c32: xor ecx, ecx
140020c34: jmp 0x140020e02 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020c39: mov byte ptr [rbp + 0x812], 1
140020c40: mov byte ptr [rbp + 0x811], 1
140020c47: lea r8, [rip + 0x2905c0a]
140020c4e: mov ecx, 4
140020c53: mov rdx, qword ptr [rbp + 0x7d0]
140020c5a: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020c5f: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020c64: lea r8, [rip + 0x2905c05]
140020c6b: mov ecx, 4
140020c70: mov rdx, rsi
140020c73: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020c78: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020c7d: mov byte ptr [rbp + 0x7bd], 1
140020c84: lea r8, [rip + 0x2905bb5]
140020c8b: mov ecx, 4
140020c90: mov rdx, rsi
140020c93: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020c98: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020c9d: mov qword ptr [rbp + 0x2e8], rbx
140020ca4: lea r8, [rip + 0x29066ed]
140020cab: mov rcx, rsi
140020cae: mov rdx, qword ptr [rbp + 0x2e8]
140020cb5: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020cba: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020cbf: mov qword ptr [rbp + 0xd0], rsi
140020cc6: mov qword ptr [rbp + 0xc8], rdi
140020ccd: lea r8, [rip + 0x29066c4]
140020cd4: mov ecx, 4
140020cd9: mov rdx, r12
140020cdc: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020ce1: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020ce6: lea r8, [rip + 0x29066ab]
140020ced: mov ecx, 4
140020cf2: mov rdx, r12
140020cf5: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020cfa: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020cff: xor ecx, ecx
140020d01: jmp 0x140020e6d ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020d06: xor esi, esi
140020d08: mov byte ptr [rbp + 0x79c], 1
140020d0f: mov byte ptr [rbp + 0x79b], 1
140020d16: lea r8, [rip + 0x2906613]
140020d1d: mov rcx, rsi
140020d20: mov rdx, rdi
140020d23: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020d28: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020d2d: xor esi, esi
140020d2f: mov byte ptr [rbp + 0x7c0], 1
140020d36: lea r8, [rip + 0x29065f3]
140020d3d: mov rcx, rsi
140020d40: mov rdx, rdi
140020d43: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020d48: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020d4d: xor esi, esi
140020d4f: mov byte ptr [rbp + 0x7bf], 1
140020d56: lea r8, [rip + 0x29065d3]
140020d5d: mov rcx, rsi
140020d60: mov rdx, rdi
140020d63: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020d68: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020d6d: xor esi, esi
140020d6f: mov byte ptr [rbp + 0x7be], 1
140020d76: lea r8, [rip + 0x29065b3]
140020d7d: mov rcx, rsi
140020d80: mov rdx, rdi
140020d83: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020d88: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020d8d: xor esi, esi
140020d8f: mov byte ptr [rbp + 0x7bc], 1
140020d96: mov byte ptr [rbp + 0x7bb], 1
140020d9d: lea r8, [rip + 0x290658c]
140020da4: mov rcx, rsi
140020da7: mov rdx, rdi
140020daa: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020daf: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020db4: mov byte ptr [rbp + 0x816], 1
140020dbb: mov byte ptr [rbp + 0x815], 1
140020dc2: lea r8, [rip + 0x2905ad7]
140020dc9: xor ecx, ecx
140020dcb: xor edx, edx
140020dcd: call 0x1428d9518 ; core::panicking::panic_bounds_check
140020dd2: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020dd7: mov byte ptr [rbp + 0x816], 1
140020dde: mov byte ptr [rbp + 0x815], 1
140020de5: lea rcx, [rip + 0x2905acc]
140020dec: call 0x1428d9310 ; core::option::unwrap_failed
140020df1: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020df6: mov ecx, 4
140020dfb: mov qword ptr [rbp + 0x2f0], rsi
140020e02: lea r8, [rip + 0x290658f]
140020e09: mov rdx, qword ptr [rbp + 0x2f0]
140020e10: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020e15: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020e1a: mov rax, qword ptr [rbp + 0x370]
140020e21: mov qword ptr [rbp + 0xe0], rax
140020e28: mov qword ptr [rbp + 0xd8], rdi
140020e2f: lea r8, [rip + 0x2906562]
140020e36: mov ecx, 4
140020e3b: mov rdx, r12
140020e3e: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020e43: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020e48: lea r8, [rip + 0x2906549]
140020e4f: mov ecx, 4
140020e54: mov rdx, r12
140020e57: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020e5c: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020e61: mov ecx, 4
140020e66: mov qword ptr [rbp + 0x2f8], r14
140020e6d: lea r8, [rip + 0x2906524]
140020e74: mov rdx, qword ptr [rbp + 0x2f8]
140020e7b: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020e80: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020e85: lea r8, [rip + 0x290650c]
140020e8c: mov ecx, 4
140020e91: mov rdx, r15
140020e94: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020e99: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020e9e: lea r8, [rip + 0x29064f3]
140020ea5: mov ecx, 4
140020eaa: mov rdx, r15
140020ead: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140020eb2: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020eb7: mov byte ptr [rbp + 0x816], 1
140020ebe: mov byte ptr [rbp + 0x815], 1
140020ec5: lea r8, [rip + 0x292d4b4]
140020ecc: mov rcx, rdx
140020ecf: mov rdx, rdi
140020ed2: call 0x1428d9518 ; core::panicking::panic_bounds_check
140020ed7: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020edc: mov byte ptr [rbp + 0x816], 1
140020ee3: mov byte ptr [rbp + 0x815], 1
140020eea: mov ecx, 4
140020eef: mov edx, 0x24
140020ef4: call 0x1428d9000 ; alloc::alloc::handle_alloc_error
140020ef9: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020efe: mov byte ptr [rbp + 0x816], 1
140020f05: mov byte ptr [rbp + 0x815], 1
140020f0c: lea r8, [rip + 0x29059fd]
140020f13: mov ecx, 1
140020f18: call 0x1428d9518 ; core::panicking::panic_bounds_check
140020f1d: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020f22: mov byte ptr [rbp + 0x816], 1
140020f29: mov byte ptr [rbp + 0x815], 1
140020f30: lea r8, [rip + 0x29059f1]
140020f37: mov ecx, 1
140020f3c: call 0x1428d9518 ; core::panicking::panic_bounds_check
140020f41: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020f46: mov byte ptr [rbp + 0x816], 1
140020f4d: mov byte ptr [rbp + 0x815], 1
140020f54: lea r8, [rip + 0x29059e5]
140020f5b: mov ecx, 1
140020f60: call 0x1428d9518 ; core::panicking::panic_bounds_check
140020f65: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020f67: mov byte ptr [rbp + 0x816], 1
140020f6e: mov byte ptr [rbp + 0x815], 1
140020f75: lea r8, [rip + 0x292d404]
140020f7c: mov rcx, rdx
140020f7f: mov rdx, rdi
140020f82: call 0x1428d9518 ; core::panicking::panic_bounds_check
140020f87: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020f89: mov byte ptr [rbp + 0x816], 1
140020f90: mov byte ptr [rbp + 0x815], 1
140020f97: lea r8, [rip + 0x29059ba]
140020f9e: mov ecx, 1
140020fa3: call 0x1428d9518 ; core::panicking::panic_bounds_check
140020fa8: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020faa: mov byte ptr [rbp + 0x816], 1
140020fb1: mov byte ptr [rbp + 0x815], 1
140020fb8: lea r8, [rip + 0x292d3c1]
140020fbf: mov rcx, rdx
140020fc2: mov rdx, rdi
140020fc5: call 0x1428d9518 ; core::panicking::panic_bounds_check
140020fca: jmp 0x140020fdc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140020fcc: lea r8, [rip + 0x29059b5]
140020fd3: xor ecx, ecx
140020fd5: xor edx, edx
140020fd7: call 0x1428d9518 ; core::panicking::panic_bounds_check
140020fdc: ud2 
140020fde: nop 
140020fe0: mov qword ptr [rsp + 0x10], rdx
140020fe5: push rbp
140020fe6: push r15
140020fe8: push r14
140020fea: push r13
140020fec: push r12
140020fee: push rsi
140020fef: push rdi
140020ff0: push rbx
140020ff1: sub rsp, 0x138
140020ff8: lea rbp, [rdx + 0x80]
140020fff: movaps xmmword ptr [rsp + 0x90], xmm15
140021008: movaps xmmword ptr [rsp + 0xa0], xmm14
140021011: movaps xmmword ptr [rsp + 0xb0], xmm13
14002101a: movaps xmmword ptr [rsp + 0xc0], xmm12
140021023: movaps xmmword ptr [rsp + 0xd0], xmm11
14002102c: movaps xmmword ptr [rsp + 0xe0], xmm10
140021035: movaps xmmword ptr [rsp + 0xf0], xmm9
14002103e: movaps xmmword ptr [rsp + 0x100], xmm8
140021047: movaps xmmword ptr [rsp + 0x110], xmm7
14002104f: movaps xmmword ptr [rsp + 0x120], xmm6
140021057: mov rdx, qword ptr [rbp + 0x3e0]
14002105e: test rdx, rdx
140021061: je 0x140021079
140021063: mov rcx, qword ptr [rbp + 0x3e8]
14002106a: shl rdx, 2
14002106e: mov r8d, 4
140021074: call 0x140613c20 ; ?
140021079: movaps xmm6, xmmword ptr [rsp + 0x120]
140021081: movaps xmm7, xmmword ptr [rsp + 0x110]
140021089: movaps xmm8, xmmword ptr [rsp + 0x100]
140021092: movaps xmm9, xmmword ptr [rsp + 0xf0]
14002109b: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400210a4: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400210ad: movaps xmm12, xmmword ptr [rsp + 0xc0]
1400210b6: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400210bf: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400210c8: movaps xmm15, xmmword ptr [rsp + 0x90]
1400210d1: add rsp, 0x138
1400210d8: pop rbx
1400210d9: pop rdi
1400210da: pop rsi
1400210db: pop r12
1400210dd: pop r13
1400210df: pop r14
1400210e1: pop r15
1400210e3: pop rbp
1400210e4: ret 
1400210e5: nop word ptr cs:[rax + rax]
1400210f0: mov qword ptr [rsp + 0x10], rdx
1400210f5: push rbp
1400210f6: push r15
1400210f8: push r14
1400210fa: push r13
1400210fc: push r12
1400210fe: push rsi
1400210ff: push rdi
140021100: push rbx
140021101: sub rsp, 0x138
140021108: lea rbp, [rdx + 0x80]
14002110f: movaps xmmword ptr [rsp + 0x90], xmm15
140021118: movaps xmmword ptr [rsp + 0xa0], xmm14
140021121: movaps xmmword ptr [rsp + 0xb0], xmm13
14002112a: movaps xmmword ptr [rsp + 0xc0], xmm12
140021133: movaps xmmword ptr [rsp + 0xd0], xmm11
14002113c: movaps xmmword ptr [rsp + 0xe0], xmm10
140021145: movaps xmmword ptr [rsp + 0xf0], xmm9
14002114e: movaps xmmword ptr [rsp + 0x100], xmm8
140021157: movaps xmmword ptr [rsp + 0x110], xmm7
14002115f: movaps xmmword ptr [rsp + 0x120], xmm6
140021167: mov rdx, qword ptr [rbp + 0x508]
14002116e: test rdx, rdx
140021171: je 0x140021189
140021173: mov rcx, qword ptr [rbp + 0x510]
14002117a: shl rdx, 2
14002117e: mov r8d, 4
140021184: call 0x140613c20 ; ?
140021189: movaps xmm6, xmmword ptr [rsp + 0x120]
140021191: movaps xmm7, xmmword ptr [rsp + 0x110]
140021199: movaps xmm8, xmmword ptr [rsp + 0x100]
1400211a2: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400211ab: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400211b4: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400211bd: movaps xmm12, xmmword ptr [rsp + 0xc0]
1400211c6: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400211cf: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400211d8: movaps xmm15, xmmword ptr [rsp + 0x90]
1400211e1: add rsp, 0x138
1400211e8: pop rbx
1400211e9: pop rdi
1400211ea: pop rsi
1400211eb: pop r12
1400211ed: pop r13
1400211ef: pop r14
1400211f1: pop r15
1400211f3: pop rbp
1400211f4: ret 
1400211f5: nop word ptr cs:[rax + rax]
140021200: mov qword ptr [rsp + 0x10], rdx
140021205: push rbp
140021206: push r15
140021208: push r14
14002120a: push r13
14002120c: push r12
14002120e: push rsi
14002120f: push rdi
140021210: push rbx
140021211: sub rsp, 0x138
140021218: lea rbp, [rdx + 0x80]
14002121f: movaps xmmword ptr [rsp + 0x90], xmm15
140021228: movaps xmmword ptr [rsp + 0xa0], xmm14
140021231: movaps xmmword ptr [rsp + 0xb0], xmm13
14002123a: movaps xmmword ptr [rsp + 0xc0], xmm12
140021243: movaps xmmword ptr [rsp + 0xd0], xmm11
14002124c: movaps xmmword ptr [rsp + 0xe0], xmm10
140021255: movaps xmmword ptr [rsp + 0xf0], xmm9
14002125e: movaps xmmword ptr [rsp + 0x100], xmm8
140021267: movaps xmmword ptr [rsp + 0x110], xmm7
14002126f: movaps xmmword ptr [rsp + 0x120], xmm6
140021277: lea rcx, [rbp + 0x6c0]
14002127e: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140021283: mov byte ptr [rbp + 0x816], 1
14002128a: mov byte ptr [rbp + 0x815], 1
140021291: movaps xmm6, xmmword ptr [rsp + 0x120]
140021299: movaps xmm7, xmmword ptr [rsp + 0x110]
1400212a1: movaps xmm8, xmmword ptr [rsp + 0x100]
1400212aa: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400212b3: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400212bc: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400212c5: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400212cf: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400212d9: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400212e2: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400212ec: add rsp, 0x138
1400212f3: pop rbx
1400212f4: pop rdi
1400212f5: pop rsi
1400212f6: pop r12
1400212f8: pop r13
1400212fa: pop r14
1400212fc: pop r15
1400212fe: pop rbp
1400212ff: ret 
140021300: mov qword ptr [rsp + 0x10], rdx
140021305: push rbp
140021306: push r15
140021308: push r14
14002130a: push r13
14002130c: push r12
14002130e: push rsi
14002130f: push rdi
140021310: push rbx
140021311: sub rsp, 0x138
140021318: lea rbp, [rdx + 0x80]
14002131f: movdqa xmmword ptr [rsp + 0x90], xmm15
140021329: movaps xmmword ptr [rsp + 0xa0], xmm14
140021332: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002133c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140021346: movaps xmmword ptr [rsp + 0xd0], xmm11
14002134f: movaps xmmword ptr [rsp + 0xe0], xmm10
140021358: movaps xmmword ptr [rsp + 0xf0], xmm9
140021361: movaps xmmword ptr [rsp + 0x100], xmm8
14002136a: movaps xmmword ptr [rsp + 0x110], xmm7
140021372: movaps xmmword ptr [rsp + 0x120], xmm6
14002137a: mov byte ptr [rbp + 0x816], 0
140021381: mov byte ptr [rbp + 0x815], 0
140021388: lea rcx, [rbp + 0x6c0]
14002138f: call 0x140609eb0 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams>
140021394: mov byte ptr [rbp + 0x816], 0
14002139b: mov byte ptr [rbp + 0x815], 0
1400213a2: movaps xmm6, xmmword ptr [rsp + 0x120]
1400213aa: movaps xmm7, xmmword ptr [rsp + 0x110]
1400213b2: movaps xmm8, xmmword ptr [rsp + 0x100]
1400213bb: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400213c4: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400213cd: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400213d6: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400213e0: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400213ea: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400213f3: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400213fd: add rsp, 0x138
140021404: pop rbx
140021405: pop rdi
140021406: pop rsi
140021407: pop r12
140021409: pop r13
14002140b: pop r14
14002140d: pop r15
14002140f: pop rbp
140021410: ret 
140021411: nop word ptr cs:[rax + rax]
140021420: mov qword ptr [rsp + 0x10], rdx
140021425: push rbp
140021426: push r15
140021428: push r14
14002142a: push r13
14002142c: push r12
14002142e: push rsi
14002142f: push rdi
140021430: push rbx
140021431: sub rsp, 0x138
140021438: lea rbp, [rdx + 0x80]
14002143f: movdqa xmmword ptr [rsp + 0x90], xmm15
140021449: movaps xmmword ptr [rsp + 0xa0], xmm14
140021452: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002145c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140021466: movaps xmmword ptr [rsp + 0xd0], xmm11
14002146f: movaps xmmword ptr [rsp + 0xe0], xmm10
140021478: movaps xmmword ptr [rsp + 0xf0], xmm9
140021481: movaps xmmword ptr [rsp + 0x100], xmm8
14002148a: movaps xmmword ptr [rsp + 0x110], xmm7
140021492: movaps xmmword ptr [rsp + 0x120], xmm6
14002149a: mov byte ptr [rbp + 0x816], 1
1400214a1: mov byte ptr [rbp + 0x815], 1
1400214a8: lea rcx, [rbp + 0x6c0]
1400214af: call 0x140609eb0 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams>
1400214b4: mov byte ptr [rbp + 0x816], 1
1400214bb: mov byte ptr [rbp + 0x815], 1
1400214c2: movaps xmm6, xmmword ptr [rsp + 0x120]
1400214ca: movaps xmm7, xmmword ptr [rsp + 0x110]
1400214d2: movaps xmm8, xmmword ptr [rsp + 0x100]
1400214db: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400214e4: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400214ed: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400214f6: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140021500: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14002150a: movaps xmm14, xmmword ptr [rsp + 0xa0]
140021513: movdqa xmm15, xmmword ptr [rsp + 0x90]
14002151d: add rsp, 0x138
140021524: pop rbx
140021525: pop rdi
140021526: pop rsi
140021527: pop r12
140021529: pop r13
14002152b: pop r14
14002152d: pop r15
14002152f: pop rbp
140021530: ret 
140021531: nop word ptr cs:[rax + rax]
140021540: mov qword ptr [rsp + 0x10], rdx
140021545: push rbp
140021546: push r15
140021548: push r14
14002154a: push r13
14002154c: push r12
14002154e: push rsi
14002154f: push rdi
140021550: push rbx
140021551: sub rsp, 0x138
140021558: lea rbp, [rdx + 0x80]
14002155f: movdqa xmmword ptr [rsp + 0x90], xmm15
140021569: movaps xmmword ptr [rsp + 0xa0], xmm14
140021572: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002157c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140021586: movaps xmmword ptr [rsp + 0xd0], xmm11
14002158f: movaps xmmword ptr [rsp + 0xe0], xmm10
140021598: movaps xmmword ptr [rsp + 0xf0], xmm9
1400215a1: movaps xmmword ptr [rsp + 0x100], xmm8
1400215aa: movaps xmmword ptr [rsp + 0x110], xmm7
1400215b2: movaps xmmword ptr [rsp + 0x120], xmm6
1400215ba: mov byte ptr [rbp + 0x813], 0
1400215c1: lea rcx, [rbp + 0x590]
1400215c8: call 0x140607a30 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsBracketAssemblyParams>
1400215cd: mov byte ptr [rbp + 0x813], 0
1400215d4: movaps xmm6, xmmword ptr [rsp + 0x120]
1400215dc: movaps xmm7, xmmword ptr [rsp + 0x110]
1400215e4: movaps xmm8, xmmword ptr [rsp + 0x100]
1400215ed: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400215f6: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400215ff: movaps xmm11, xmmword ptr [rsp + 0xd0]
140021608: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140021612: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14002161c: movaps xmm14, xmmword ptr [rsp + 0xa0]
140021625: movdqa xmm15, xmmword ptr [rsp + 0x90]
14002162f: add rsp, 0x138
140021636: pop rbx
140021637: pop rdi
140021638: pop rsi
140021639: pop r12
14002163b: pop r13
14002163d: pop r14
14002163f: pop r15
140021641: pop rbp
140021642: ret 
140021643: nop word ptr cs:[rax + rax]
140021650: mov qword ptr [rsp + 0x10], rdx
140021655: push rbp
140021656: push r15
140021658: push r14
14002165a: push r13
14002165c: push r12
14002165e: push rsi
14002165f: push rdi
140021660: push rbx
140021661: sub rsp, 0x138
140021668: lea rbp, [rdx + 0x80]
14002166f: movdqa xmmword ptr [rsp + 0x90], xmm15
140021679: movaps xmmword ptr [rsp + 0xa0], xmm14
140021682: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002168c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140021696: movaps xmmword ptr [rsp + 0xd0], xmm11
14002169f: movaps xmmword ptr [rsp + 0xe0], xmm10
1400216a8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400216b1: movaps xmmword ptr [rsp + 0x100], xmm8
1400216ba: movaps xmmword ptr [rsp + 0x110], xmm7
1400216c2: movaps xmmword ptr [rsp + 0x120], xmm6
1400216ca: cmp qword ptr [rbp + 0x7d0], 0
1400216d2: jne 0x140021796
1400216d8: cmp qword ptr [rbp + 0x7a0], 0
1400216e0: jne 0x1400217c5
1400216e6: cmp qword ptr [rbp + 0x808], 0
1400216ee: je 0x14002170d
1400216f0: mov rdx, qword ptr [rbp + 0x808]
1400216f7: shl rdx, 2
1400216fb: mov r8d, 4
140021701: mov rcx, qword ptr [rbp + 0x6b0]
140021708: call 0x140613c20 ; ?
14002170d: lea rcx, [rbp + 0x590]
140021714: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140021719: mov byte ptr [rbp + 0x816], 0
140021720: mov byte ptr [rbp + 0x815], 0
140021727: movaps xmm6, xmmword ptr [rsp + 0x120]
14002172f: movaps xmm7, xmmword ptr [rsp + 0x110]
140021737: movaps xmm8, xmmword ptr [rsp + 0x100]
140021740: movaps xmm9, xmmword ptr [rsp + 0xf0]
140021749: movaps xmm10, xmmword ptr [rsp + 0xe0]
140021752: movaps xmm11, xmmword ptr [rsp + 0xd0]
14002175b: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140021765: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14002176f: movaps xmm14, xmmword ptr [rsp + 0xa0]
140021778: movdqa xmm15, xmmword ptr [rsp + 0x90]
140021782: add rsp, 0x138
140021789: pop rbx
14002178a: pop rdi
14002178b: pop rsi
14002178c: pop r12
14002178e: pop r13
140021790: pop r14
140021792: pop r15
140021794: pop rbp
140021795: ret 
140021796: mov rax, qword ptr [rbp + 0x7d0]
14002179d: lea rdx, [rax*4]
1400217a5: mov r8d, 4
1400217ab: mov rcx, qword ptr [rbp + 0x7f0]
1400217b2: call 0x140613c20 ; ?
1400217b7: cmp qword ptr [rbp + 0x7a0], 0
1400217bf: je 0x1400216e6
1400217c5: mov rdx, qword ptr [rbp + 0x7a0]
1400217cc: shl rdx, 3
1400217d0: mov r8d, 4
1400217d6: mov rcx, qword ptr [rbp + 0x7d8]
1400217dd: call 0x140613c20 ; ?
1400217e2: cmp qword ptr [rbp + 0x808], 0
1400217ea: jne 0x1400216f0
1400217f0: jmp 0x14002170d ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
1400217f5: nop word ptr cs:[rax + rax]
140021800: mov qword ptr [rsp + 0x10], rdx
140021805: push rbp
140021806: push r15
140021808: push r14
14002180a: push r13
14002180c: push r12
14002180e: push rsi
14002180f: push rdi
140021810: push rbx
140021811: sub rsp, 0x138
140021818: lea rbp, [rdx + 0x80]
14002181f: movaps xmmword ptr [rsp + 0x90], xmm15
140021828: movaps xmmword ptr [rsp + 0xa0], xmm14
140021831: movaps xmmword ptr [rsp + 0xb0], xmm13
14002183a: movaps xmmword ptr [rsp + 0xc0], xmm12
140021843: movaps xmmword ptr [rsp + 0xd0], xmm11
14002184c: movaps xmmword ptr [rsp + 0xe0], xmm10
140021855: movaps xmmword ptr [rsp + 0xf0], xmm9
14002185e: movaps xmmword ptr [rsp + 0x100], xmm8
140021867: movaps xmmword ptr [rsp + 0x110], xmm7
14002186f: movaps xmmword ptr [rsp + 0x120], xmm6
140021877: mov byte ptr [rbp + 0x7be], 0
14002187e: lea rcx, [rbp + 0x6c0]
140021885: call 0x140607a30 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsBracketAssemblyParams>
14002188a: mov byte ptr [rbp + 0x7be], 0
140021891: movaps xmm6, xmmword ptr [rsp + 0x120]
140021899: movaps xmm7, xmmword ptr [rsp + 0x110]
1400218a1: movaps xmm8, xmmword ptr [rsp + 0x100]
1400218aa: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400218b3: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400218bc: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400218c5: movaps xmm12, xmmword ptr [rsp + 0xc0]
1400218ce: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400218d7: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400218e0: movaps xmm15, xmmword ptr [rsp + 0x90]
1400218e9: add rsp, 0x138
1400218f0: pop rbx
1400218f1: pop rdi
1400218f2: pop rsi
1400218f3: pop r12
1400218f5: pop r13
1400218f7: pop r14
1400218f9: pop r15
1400218fb: pop rbp
1400218fc: ret 
1400218fd: nop dword ptr [rax]
140021900: mov qword ptr [rsp + 0x10], rdx
140021905: push rbp
140021906: push r15
140021908: push r14
14002190a: push r13
14002190c: push r12
14002190e: push rsi
14002190f: push rdi
140021910: push rbx
140021911: sub rsp, 0x138
140021918: lea rbp, [rdx + 0x80]
14002191f: movdqa xmmword ptr [rsp + 0x90], xmm15
140021929: movaps xmmword ptr [rsp + 0xa0], xmm14
140021932: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002193c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140021946: movaps xmmword ptr [rsp + 0xd0], xmm11
14002194f: movaps xmmword ptr [rsp + 0xe0], xmm10
140021958: movaps xmmword ptr [rsp + 0xf0], xmm9
140021961: movaps xmmword ptr [rsp + 0x100], xmm8
14002196a: movaps xmmword ptr [rsp + 0x110], xmm7
140021972: movaps xmmword ptr [rsp + 0x120], xmm6
14002197a: lea rcx, [rbp + 0x590]
140021981: call 0x140609eb0 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams>
140021986: movaps xmm6, xmmword ptr [rsp + 0x120]
14002198e: movaps xmm7, xmmword ptr [rsp + 0x110]
140021996: movaps xmm8, xmmword ptr [rsp + 0x100]
14002199f: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400219a8: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400219b1: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400219ba: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400219c4: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400219ce: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400219d7: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400219e1: add rsp, 0x138
1400219e8: pop rbx
1400219e9: pop rdi
1400219ea: pop rsi
1400219eb: pop r12
1400219ed: pop r13
1400219ef: pop r14
1400219f1: pop r15
1400219f3: pop rbp
1400219f4: ret 
1400219f5: nop word ptr cs:[rax + rax]
140021a00: mov qword ptr [rsp + 0x10], rdx
140021a05: push rbp
140021a06: push r15
140021a08: push r14
140021a0a: push r13
140021a0c: push r12
140021a0e: push rsi
140021a0f: push rdi
140021a10: push rbx
140021a11: sub rsp, 0x138
140021a18: lea rbp, [rdx + 0x80]
140021a1f: movaps xmmword ptr [rsp + 0x90], xmm15
140021a28: movaps xmmword ptr [rsp + 0xa0], xmm14
140021a31: movaps xmmword ptr [rsp + 0xb0], xmm13
140021a3a: movaps xmmword ptr [rsp + 0xc0], xmm12
140021a43: movaps xmmword ptr [rsp + 0xd0], xmm11
140021a4c: movaps xmmword ptr [rsp + 0xe0], xmm10
140021a55: movaps xmmword ptr [rsp + 0xf0], xmm9
140021a5e: movaps xmmword ptr [rsp + 0x100], xmm8
140021a67: movaps xmmword ptr [rsp + 0x110], xmm7
140021a6f: movaps xmmword ptr [rsp + 0x120], xmm6
140021a77: mov byte ptr [rbp + 0x812], 0
140021a7e: mov byte ptr [rbp + 0x811], 0
140021a85: lea rcx, [rbp + 0x6c0]
140021a8c: call 0x140609eb0 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams>
140021a91: mov byte ptr [rbp + 0x812], 0
140021a98: mov byte ptr [rbp + 0x811], 0
140021a9f: movaps xmm6, xmmword ptr [rsp + 0x120]
140021aa7: movaps xmm7, xmmword ptr [rsp + 0x110]
140021aaf: movaps xmm8, xmmword ptr [rsp + 0x100]
140021ab8: movaps xmm9, xmmword ptr [rsp + 0xf0]
140021ac1: movaps xmm10, xmmword ptr [rsp + 0xe0]
140021aca: movapd xmm11, xmmword ptr [rsp + 0xd0]
140021ad4: movaps xmm12, xmmword ptr [rsp + 0xc0]
140021add: movaps xmm13, xmmword ptr [rsp + 0xb0]
140021ae6: movaps xmm14, xmmword ptr [rsp + 0xa0]
140021aef: movaps xmm15, xmmword ptr [rsp + 0x90]
140021af8: add rsp, 0x138
140021aff: pop rbx
140021b00: pop rdi
140021b01: pop rsi
140021b02: pop r12
140021b04: pop r13
140021b06: pop r14
140021b08: pop r15
140021b0a: pop rbp
140021b0b: ret 
140021b0c: nop dword ptr [rax]
140021b10: mov qword ptr [rsp + 0x10], rdx
140021b15: push rbp
140021b16: push r15
140021b18: push r14
140021b1a: push r13
140021b1c: push r12
140021b1e: push rsi
140021b1f: push rdi
140021b20: push rbx
140021b21: sub rsp, 0x138
140021b28: lea rbp, [rdx + 0x80]
140021b2f: movdqa xmmword ptr [rsp + 0x90], xmm15
140021b39: movaps xmmword ptr [rsp + 0xa0], xmm14
140021b42: movdqa xmmword ptr [rsp + 0xb0], xmm13
140021b4c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140021b56: movaps xmmword ptr [rsp + 0xd0], xmm11
140021b5f: movaps xmmword ptr [rsp + 0xe0], xmm10
140021b68: movaps xmmword ptr [rsp + 0xf0], xmm9
140021b71: movaps xmmword ptr [rsp + 0x100], xmm8
140021b7a: movaps xmmword ptr [rsp + 0x110], xmm7
140021b82: movaps xmmword ptr [rsp + 0x120], xmm6
140021b8a: cmp dword ptr [rbp + 0x140], 0xb
140021b91: je 0x140021c0e
140021b93: lea rcx, [rbp + 0x140]
140021b9a: call 0x1400297d0 ; core::ptr::drop_in_place<enum2$<core::result::Result<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError> > > >
140021b9f: movaps xmm6, xmmword ptr [rsp + 0x120]
140021ba7: movaps xmm7, xmmword ptr [rsp + 0x110]
140021baf: movaps xmm8, xmmword ptr [rsp + 0x100]
140021bb8: movaps xmm9, xmmword ptr [rsp + 0xf0]
140021bc1: movaps xmm10, xmmword ptr [rsp + 0xe0]
140021bca: movaps xmm11, xmmword ptr [rsp + 0xd0]
140021bd3: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140021bdd: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140021be7: movaps xmm14, xmmword ptr [rsp + 0xa0]
140021bf0: movdqa xmm15, xmmword ptr [rsp + 0x90]
140021bfa: add rsp, 0x138
140021c01: pop rbx
140021c02: pop rdi
140021c03: pop rsi
140021c04: pop r12
140021c06: pop r13
140021c08: pop r14
140021c0a: pop r15
140021c0c: pop rbp
140021c0d: ret 
140021c0e: movaps xmm6, xmmword ptr [rsp + 0x120]
140021c16: movaps xmm7, xmmword ptr [rsp + 0x110]
140021c1e: movaps xmm8, xmmword ptr [rsp + 0x100]
140021c27: movaps xmm9, xmmword ptr [rsp + 0xf0]
140021c30: movaps xmm10, xmmword ptr [rsp + 0xe0]
140021c39: movaps xmm11, xmmword ptr [rsp + 0xd0]
140021c42: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140021c4c: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140021c56: movaps xmm14, xmmword ptr [rsp + 0xa0]
140021c5f: movdqa xmm15, xmmword ptr [rsp + 0x90]
140021c69: add rsp, 0x138
140021c70: pop rbx
140021c71: pop rdi
140021c72: pop rsi
140021c73: pop r12
140021c75: pop r13
140021c77: pop r14
140021c79: pop r15
140021c7b: pop rbp
140021c7c: ret 
140021c7d: nop dword ptr [rax]
140021c80: mov qword ptr [rsp + 0x10], rdx
140021c85: push rbp
140021c86: push r15
140021c88: push r14
140021c8a: push r13
140021c8c: push r12
140021c8e: push rsi
140021c8f: push rdi
140021c90: push rbx
140021c91: sub rsp, 0x138
140021c98: lea rbp, [rdx + 0x80]
140021c9f: movaps xmmword ptr [rsp + 0x90], xmm15
140021ca8: movaps xmmword ptr [rsp + 0xa0], xmm14
140021cb1: movaps xmmword ptr [rsp + 0xb0], xmm13
140021cba: movaps xmmword ptr [rsp + 0xc0], xmm12
140021cc3: movaps xmmword ptr [rsp + 0xd0], xmm11
140021ccc: movaps xmmword ptr [rsp + 0xe0], xmm10
140021cd5: movaps xmmword ptr [rsp + 0xf0], xmm9
140021cde: movaps xmmword ptr [rsp + 0x100], xmm8
140021ce7: movaps xmmword ptr [rsp + 0x110], xmm7
140021cef: movaps xmmword ptr [rsp + 0x120], xmm6
140021cf7: lea rcx, [rbp + 0x6c0]
140021cfe: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140021d03: mov byte ptr [rbp + 0x812], 1
140021d0a: mov byte ptr [rbp + 0x811], 0
140021d11: movaps xmm6, xmmword ptr [rsp + 0x120]
140021d19: movaps xmm7, xmmword ptr [rsp + 0x110]
140021d21: movaps xmm8, xmmword ptr [rsp + 0x100]
140021d2a: movaps xmm9, xmmword ptr [rsp + 0xf0]
140021d33: movaps xmm10, xmmword ptr [rsp + 0xe0]
140021d3c: movapd xmm11, xmmword ptr [rsp + 0xd0]
140021d46: movaps xmm12, xmmword ptr [rsp + 0xc0]
140021d4f: movaps xmm13, xmmword ptr [rsp + 0xb0]
140021d58: movaps xmm14, xmmword ptr [rsp + 0xa0]
140021d61: movaps xmm15, xmmword ptr [rsp + 0x90]
140021d6a: add rsp, 0x138
140021d71: pop rbx
140021d72: pop rdi
140021d73: pop rsi
140021d74: pop r12
140021d76: pop r13
140021d78: pop r14
140021d7a: pop r15
140021d7c: pop rbp
140021d7d: ret 
140021d7e: nop 
140021d80: mov qword ptr [rsp + 0x10], rdx
140021d85: push rbp
140021d86: push r15
140021d88: push r14
140021d8a: push r13
140021d8c: push r12
140021d8e: push rsi
140021d8f: push rdi
140021d90: push rbx
140021d91: sub rsp, 0x138
140021d98: lea rbp, [rdx + 0x80]
140021d9f: movaps xmmword ptr [rsp + 0x90], xmm15
140021da8: movaps xmmword ptr [rsp + 0xa0], xmm14
140021db1: movaps xmmword ptr [rsp + 0xb0], xmm13
140021dba: movaps xmmword ptr [rsp + 0xc0], xmm12
140021dc3: movaps xmmword ptr [rsp + 0xd0], xmm11
140021dcc: movaps xmmword ptr [rsp + 0xe0], xmm10
140021dd5: movaps xmmword ptr [rsp + 0xf0], xmm9
140021dde: movaps xmmword ptr [rsp + 0x100], xmm8
140021de7: movaps xmmword ptr [rsp + 0x110], xmm7
140021def: movaps xmmword ptr [rsp + 0x120], xmm6
140021df7: mov rax, qword ptr [rbp + 0x6b0]
140021dfe: test rax, rax
140021e01: je 0x140021e1c
140021e03: mov rdx, rax
140021e06: shl rdx, 2
140021e0a: mov r8d, 4
140021e10: mov rcx, qword ptr [rbp + 0x7f0]
140021e17: call 0x140613c20 ; ?
140021e1c: mov byte ptr [rbp + 0x812], 1
140021e23: mov byte ptr [rbp + 0x811], 1
140021e2a: movaps xmm6, xmmword ptr [rsp + 0x120]
140021e32: movaps xmm7, xmmword ptr [rsp + 0x110]
140021e3a: movaps xmm8, xmmword ptr [rsp + 0x100]
140021e43: movaps xmm9, xmmword ptr [rsp + 0xf0]
140021e4c: movaps xmm10, xmmword ptr [rsp + 0xe0]
140021e55: movapd xmm11, xmmword ptr [rsp + 0xd0]
140021e5f: movaps xmm12, xmmword ptr [rsp + 0xc0]
140021e68: movaps xmm13, xmmword ptr [rsp + 0xb0]
140021e71: movaps xmm14, xmmword ptr [rsp + 0xa0]
140021e7a: movaps xmm15, xmmword ptr [rsp + 0x90]
140021e83: add rsp, 0x138
140021e8a: pop rbx
140021e8b: pop rdi
140021e8c: pop rsi
140021e8d: pop r12
140021e8f: pop r13
140021e91: pop r14
140021e93: pop r15
140021e95: pop rbp
140021e96: ret 
140021e97: nop word ptr [rax + rax]
140021ea0: mov qword ptr [rsp + 0x10], rdx
140021ea5: push rbp
140021ea6: push r15
140021ea8: push r14
140021eaa: push r13
140021eac: push r12
140021eae: push rsi
140021eaf: push rdi
140021eb0: push rbx
140021eb1: sub rsp, 0x138
140021eb8: lea rbp, [rdx + 0x80]
140021ebf: movdqa xmmword ptr [rsp + 0x90], xmm15
140021ec9: movaps xmmword ptr [rsp + 0xa0], xmm14
140021ed2: movdqa xmmword ptr [rsp + 0xb0], xmm13
140021edc: movdqa xmmword ptr [rsp + 0xc0], xmm12
140021ee6: movaps xmmword ptr [rsp + 0xd0], xmm11
140021eef: movaps xmmword ptr [rsp + 0xe0], xmm10
140021ef8: movaps xmmword ptr [rsp + 0xf0], xmm9
140021f01: movaps xmmword ptr [rsp + 0x100], xmm8
140021f0a: movaps xmmword ptr [rsp + 0x110], xmm7
140021f12: movaps xmmword ptr [rsp + 0x120], xmm6
140021f1a: mov rax, qword ptr [rbp + 0x808]
140021f21: inc qword ptr [rax]
140021f24: movaps xmm6, xmmword ptr [rsp + 0x120]
140021f2c: movaps xmm7, xmmword ptr [rsp + 0x110]
140021f34: movaps xmm8, xmmword ptr [rsp + 0x100]
140021f3d: movaps xmm9, xmmword ptr [rsp + 0xf0]
140021f46: movaps xmm10, xmmword ptr [rsp + 0xe0]
140021f4f: movaps xmm11, xmmword ptr [rsp + 0xd0]
140021f58: movaps xmm12, xmmword ptr [rsp + 0xc0]
140021f61: movaps xmm13, xmmword ptr [rsp + 0xb0]
140021f6a: movaps xmm14, xmmword ptr [rsp + 0xa0]
140021f73: movaps xmm15, xmmword ptr [rsp + 0x90]
140021f7c: add rsp, 0x138
140021f83: pop rbx
140021f84: pop rdi
140021f85: pop rsi
140021f86: pop r12
140021f88: pop r13
140021f8a: pop r14
140021f8c: pop r15
140021f8e: pop rbp
140021f8f: ret 
140021f90: mov qword ptr [rsp + 0x10], rdx
140021f95: push rbp
140021f96: push r15
140021f98: push r14
140021f9a: push r13
140021f9c: push r12
140021f9e: push rsi
140021f9f: push rdi
140021fa0: push rbx
140021fa1: sub rsp, 0x138
140021fa8: lea rbp, [rdx + 0x80]
140021faf: movaps xmmword ptr [rsp + 0x90], xmm15
140021fb8: movaps xmmword ptr [rsp + 0xa0], xmm14
140021fc1: movaps xmmword ptr [rsp + 0xb0], xmm13
140021fca: movaps xmmword ptr [rsp + 0xc0], xmm12
140021fd3: movdqa xmmword ptr [rsp + 0xd0], xmm11
140021fdd: movaps xmmword ptr [rsp + 0xe0], xmm10
140021fe6: movaps xmmword ptr [rsp + 0xf0], xmm9
140021fef: movaps xmmword ptr [rsp + 0x100], xmm8
140021ff8: movaps xmmword ptr [rsp + 0x110], xmm7
140022000: movaps xmmword ptr [rsp + 0x120], xmm6
140022008: mov byte ptr [rbp + 0x817], 1
14002200f: lea rcx, [rbp + 0x6c0]
140022016: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
14002201b: mov byte ptr [rbp + 0x817], 1
140022022: movaps xmm6, xmmword ptr [rsp + 0x120]
14002202a: movaps xmm7, xmmword ptr [rsp + 0x110]
140022032: movaps xmm8, xmmword ptr [rsp + 0x100]
14002203b: movaps xmm9, xmmword ptr [rsp + 0xf0]
140022044: movaps xmm10, xmmword ptr [rsp + 0xe0]
14002204d: movaps xmm11, xmmword ptr [rsp + 0xd0]
140022056: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140022060: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14002206a: movaps xmm14, xmmword ptr [rsp + 0xa0]
140022073: movdqa xmm15, xmmword ptr [rsp + 0x90]
14002207d: add rsp, 0x138
140022084: pop rbx
140022085: pop rdi
140022086: pop rsi
140022087: pop r12
140022089: pop r13
14002208b: pop r14
14002208d: pop r15
14002208f: pop rbp
140022090: ret 
140022091: nop word ptr cs:[rax + rax]
1400220a0: mov qword ptr [rsp + 0x10], rdx
1400220a5: push rbp
1400220a6: push r15
1400220a8: push r14
1400220aa: push r13
1400220ac: push r12
1400220ae: push rsi
1400220af: push rdi
1400220b0: push rbx
1400220b1: sub rsp, 0x138
1400220b8: lea rbp, [rdx + 0x80]
1400220bf: movaps xmmword ptr [rsp + 0x90], xmm15
1400220c8: movaps xmmword ptr [rsp + 0xa0], xmm14
1400220d1: movaps xmmword ptr [rsp + 0xb0], xmm13
1400220da: movaps xmmword ptr [rsp + 0xc0], xmm12
1400220e3: movaps xmmword ptr [rsp + 0xd0], xmm11
1400220ec: movaps xmmword ptr [rsp + 0xe0], xmm10
1400220f5: movaps xmmword ptr [rsp + 0xf0], xmm9
1400220fe: movaps xmmword ptr [rsp + 0x100], xmm8
140022107: movaps xmmword ptr [rsp + 0x110], xmm7
14002210f: movaps xmmword ptr [rsp + 0x120], xmm6
140022117: mov byte ptr [rbp + 0x817], 1
14002211e: lea rcx, [rbp + 0x6c0]
140022125: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
14002212a: mov byte ptr [rbp + 0x817], 1
140022131: movaps xmm6, xmmword ptr [rsp + 0x120]
140022139: movaps xmm7, xmmword ptr [rsp + 0x110]
140022141: movaps xmm8, xmmword ptr [rsp + 0x100]
14002214a: movaps xmm9, xmmword ptr [rsp + 0xf0]
140022153: movaps xmm10, xmmword ptr [rsp + 0xe0]
14002215c: movaps xmm11, xmmword ptr [rsp + 0xd0]
140022165: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14002216f: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140022179: movaps xmm14, xmmword ptr [rsp + 0xa0]
140022182: movdqa xmm15, xmmword ptr [rsp + 0x90]
14002218c: add rsp, 0x138
140022193: pop rbx
140022194: pop rdi
140022195: pop rsi
140022196: pop r12
140022198: pop r13
14002219a: pop r14
14002219c: pop r15
14002219e: pop rbp
14002219f: ret 
1400221a0: mov qword ptr [rsp + 0x10], rdx
1400221a5: push rbp
1400221a6: push r15
1400221a8: push r14
1400221aa: push r13
1400221ac: push r12
1400221ae: push rsi
1400221af: push rdi
1400221b0: push rbx
1400221b1: sub rsp, 0x138
1400221b8: lea rbp, [rdx + 0x80]
1400221bf: movaps xmmword ptr [rsp + 0x90], xmm15
1400221c8: movaps xmmword ptr [rsp + 0xa0], xmm14
1400221d1: movaps xmmword ptr [rsp + 0xb0], xmm13
1400221da: movaps xmmword ptr [rsp + 0xc0], xmm12
1400221e3: movaps xmmword ptr [rsp + 0xd0], xmm11
1400221ec: movaps xmmword ptr [rsp + 0xe0], xmm10
1400221f5: movaps xmmword ptr [rsp + 0xf0], xmm9
1400221fe: movaps xmmword ptr [rsp + 0x100], xmm8
140022207: movaps xmmword ptr [rsp + 0x110], xmm7
14002220f: movaps xmmword ptr [rsp + 0x120], xmm6
140022217: movzx ebx, byte ptr [rbp + 0x7be]
14002221e: test bl, 1
140022221: je 0x1400222c0
140022227: cmp qword ptr [rbp + 0x6b0], 0
14002222f: je 0x14002224e
140022231: mov rdx, qword ptr [rbp + 0x6b0]
140022238: shl rdx, 2
14002223c: mov r8d, 4
140022242: mov rcx, qword ptr [rbp + 0x7f0]
140022249: call 0x140613c20 ; ?
14002224e: mov byte ptr [rbp + 0x7bf], bl
140022254: movaps xmm6, xmmword ptr [rsp + 0x120]
14002225c: movaps xmm7, xmmword ptr [rsp + 0x110]
140022264: movaps xmm8, xmmword ptr [rsp + 0x100]
14002226d: movaps xmm9, xmmword ptr [rsp + 0xf0]
140022276: movaps xmm10, xmmword ptr [rsp + 0xe0]
14002227f: movaps xmm11, xmmword ptr [rsp + 0xd0]
140022288: movaps xmm12, xmmword ptr [rsp + 0xc0]
140022291: movaps xmm13, xmmword ptr [rsp + 0xb0]
14002229a: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400222a3: movaps xmm15, xmmword ptr [rsp + 0x90]
1400222ac: add rsp, 0x138
1400222b3: pop rbx
1400222b4: pop rdi
1400222b5: pop rsi
1400222b6: pop r12
1400222b8: pop r13
1400222ba: pop r14
1400222bc: pop r15
1400222be: pop rbp
1400222bf: ret 
1400222c0: mov byte ptr [rbp + 0x7bf], bl
1400222c6: movaps xmm6, xmmword ptr [rsp + 0x120]
1400222ce: movaps xmm7, xmmword ptr [rsp + 0x110]
1400222d6: movaps xmm8, xmmword ptr [rsp + 0x100]
1400222df: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400222e8: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400222f1: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400222fa: movaps xmm12, xmmword ptr [rsp + 0xc0]
140022303: movaps xmm13, xmmword ptr [rsp + 0xb0]
14002230c: movaps xmm14, xmmword ptr [rsp + 0xa0]
140022315: movaps xmm15, xmmword ptr [rsp + 0x90]
14002231e: add rsp, 0x138
140022325: pop rbx
140022326: pop rdi
140022327: pop rsi
140022328: pop r12
14002232a: pop r13
14002232c: pop r14
14002232e: pop r15
140022330: pop rbp
140022331: ret 
140022332: nop word ptr cs:[rax + rax]
140022340: mov qword ptr [rsp + 0x10], rdx
140022345: push rbp
140022346: push r15
140022348: push r14
14002234a: push r13
14002234c: push r12
14002234e: push rsi
14002234f: push rdi
140022350: push rbx
140022351: sub rsp, 0x138
140022358: lea rbp, [rdx + 0x80]
14002235f: movdqa xmmword ptr [rsp + 0x90], xmm15
140022369: movaps xmmword ptr [rsp + 0xa0], xmm14
140022372: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002237c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140022386: movaps xmmword ptr [rsp + 0xd0], xmm11
14002238f: movaps xmmword ptr [rsp + 0xe0], xmm10
140022398: movaps xmmword ptr [rsp + 0xf0], xmm9
1400223a1: movaps xmmword ptr [rsp + 0x100], xmm8
1400223aa: movaps xmmword ptr [rsp + 0x110], xmm7
1400223b2: movaps xmmword ptr [rsp + 0x120], xmm6
1400223ba: mov rax, qword ptr [rbp + 0x7a0]
1400223c1: inc qword ptr [rax]
1400223c4: movaps xmm6, xmmword ptr [rsp + 0x120]
1400223cc: movaps xmm7, xmmword ptr [rsp + 0x110]
1400223d4: movaps xmm8, xmmword ptr [rsp + 0x100]
1400223dd: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400223e6: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400223ef: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400223f8: movaps xmm12, xmmword ptr [rsp + 0xc0]
140022401: movaps xmm13, xmmword ptr [rsp + 0xb0]
14002240a: movaps xmm14, xmmword ptr [rsp + 0xa0]
140022413: movaps xmm15, xmmword ptr [rsp + 0x90]
14002241c: add rsp, 0x138
140022423: pop rbx
140022424: pop rdi
140022425: pop rsi
140022426: pop r12
140022428: pop r13
14002242a: pop r14
14002242c: pop r15
14002242e: pop rbp
14002242f: ret 
140022430: mov qword ptr [rsp + 0x10], rdx
140022435: push rbp
140022436: push r15
140022438: push r14
14002243a: push r13
14002243c: push r12
14002243e: push rsi
14002243f: push rdi
140022440: push rbx
140022441: sub rsp, 0x138
140022448: lea rbp, [rdx + 0x80]
14002244f: movaps xmmword ptr [rsp + 0x90], xmm15
140022458: movaps xmmword ptr [rsp + 0xa0], xmm14
140022461: movaps xmmword ptr [rsp + 0xb0], xmm13
14002246a: movaps xmmword ptr [rsp + 0xc0], xmm12
140022473: movaps xmmword ptr [rsp + 0xd0], xmm11
14002247c: movaps xmmword ptr [rsp + 0xe0], xmm10
140022485: movaps xmmword ptr [rsp + 0xf0], xmm9
14002248e: movaps xmmword ptr [rsp + 0x100], xmm8
140022497: movaps xmmword ptr [rsp + 0x110], xmm7
14002249f: movaps xmmword ptr [rsp + 0x120], xmm6
1400224a7: movzx ebx, byte ptr [rbp + 0x7bf]
1400224ae: test bl, 1
1400224b1: je 0x140022550
1400224b7: cmp qword ptr [rbp + 0x808], 0
1400224bf: je 0x1400224de
1400224c1: mov rdx, qword ptr [rbp + 0x808]
1400224c8: shl rdx, 3
1400224cc: mov r8d, 4
1400224d2: mov rcx, qword ptr [rbp + 0x7a0]
1400224d9: call 0x140613c20 ; ?
1400224de: mov byte ptr [rbp + 0x7c0], bl
1400224e4: movaps xmm6, xmmword ptr [rsp + 0x120]
1400224ec: movaps xmm7, xmmword ptr [rsp + 0x110]
1400224f4: movaps xmm8, xmmword ptr [rsp + 0x100]
1400224fd: movaps xmm9, xmmword ptr [rsp + 0xf0]
140022506: movaps xmm10, xmmword ptr [rsp + 0xe0]
14002250f: movaps xmm11, xmmword ptr [rsp + 0xd0]
140022518: movaps xmm12, xmmword ptr [rsp + 0xc0]
140022521: movaps xmm13, xmmword ptr [rsp + 0xb0]
14002252a: movaps xmm14, xmmword ptr [rsp + 0xa0]
140022533: movaps xmm15, xmmword ptr [rsp + 0x90]
14002253c: add rsp, 0x138
140022543: pop rbx
140022544: pop rdi
140022545: pop rsi
140022546: pop r12
140022548: pop r13
14002254a: pop r14
14002254c: pop r15
14002254e: pop rbp
14002254f: ret 
140022550: mov byte ptr [rbp + 0x7c0], bl
140022556: movaps xmm6, xmmword ptr [rsp + 0x120]
14002255e: movaps xmm7, xmmword ptr [rsp + 0x110]
140022566: movaps xmm8, xmmword ptr [rsp + 0x100]
14002256f: movaps xmm9, xmmword ptr [rsp + 0xf0]
140022578: movaps xmm10, xmmword ptr [rsp + 0xe0]
140022581: movaps xmm11, xmmword ptr [rsp + 0xd0]
14002258a: movaps xmm12, xmmword ptr [rsp + 0xc0]
140022593: movaps xmm13, xmmword ptr [rsp + 0xb0]
14002259c: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400225a5: movaps xmm15, xmmword ptr [rsp + 0x90]
1400225ae: add rsp, 0x138
1400225b5: pop rbx
1400225b6: pop rdi
1400225b7: pop rsi
1400225b8: pop r12
1400225ba: pop r13
1400225bc: pop r14
1400225be: pop r15
1400225c0: pop rbp
1400225c1: ret 
1400225c2: nop word ptr cs:[rax + rax]
1400225d0: mov qword ptr [rsp + 0x10], rdx
1400225d5: push rbp
1400225d6: push r15
1400225d8: push r14
1400225da: push r13
1400225dc: push r12
1400225de: push rsi
1400225df: push rdi
1400225e0: push rbx
1400225e1: sub rsp, 0x138
1400225e8: lea rbp, [rdx + 0x80]
1400225ef: movdqa xmmword ptr [rsp + 0x90], xmm15
1400225f9: movaps xmmword ptr [rsp + 0xa0], xmm14
140022602: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002260c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140022616: movaps xmmword ptr [rsp + 0xd0], xmm11
14002261f: movaps xmmword ptr [rsp + 0xe0], xmm10
140022628: movaps xmmword ptr [rsp + 0xf0], xmm9
140022631: movaps xmmword ptr [rsp + 0x100], xmm8
14002263a: movaps xmmword ptr [rsp + 0x110], xmm7
140022642: movaps xmmword ptr [rsp + 0x120], xmm6
14002264a: mov rdx, qword ptr [rbp + 0x590]
140022651: test rdx, rdx
140022654: je 0x14002266c
140022656: shl rdx, 4
14002265a: mov r8d, 0x10
140022660: mov rcx, qword ptr [rbp + 0x808]
140022667: call 0x140613c20 ; ?
14002266c: movaps xmm6, xmmword ptr [rsp + 0x120]
140022674: movaps xmm7, xmmword ptr [rsp + 0x110]
14002267c: movaps xmm8, xmmword ptr [rsp + 0x100]
140022685: movaps xmm9, xmmword ptr [rsp + 0xf0]
14002268e: movaps xmm10, xmmword ptr [rsp + 0xe0]
140022697: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400226a0: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400226aa: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400226b4: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400226bd: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400226c7: add rsp, 0x138
1400226ce: pop rbx
1400226cf: pop rdi
1400226d0: pop rsi
1400226d1: pop r12
1400226d3: pop r13
1400226d5: pop r14
1400226d7: pop r15
1400226d9: pop rbp
1400226da: ret 
1400226db: nop dword ptr [rax + rax]
1400226e0: mov qword ptr [rsp + 0x10], rdx
1400226e5: push rbp
1400226e6: push r15
1400226e8: push r14
1400226ea: push r13
1400226ec: push r12
1400226ee: push rsi
1400226ef: push rdi
1400226f0: push rbx
1400226f1: sub rsp, 0x138
1400226f8: lea rbp, [rdx + 0x80]
1400226ff: movaps xmmword ptr [rsp + 0x90], xmm15
140022708: movaps xmmword ptr [rsp + 0xa0], xmm14
140022711: movaps xmmword ptr [rsp + 0xb0], xmm13
14002271a: movaps xmmword ptr [rsp + 0xc0], xmm12
140022723: movaps xmmword ptr [rsp + 0xd0], xmm11
14002272c: movaps xmmword ptr [rsp + 0xe0], xmm10
140022735: movaps xmmword ptr [rsp + 0xf0], xmm9
14002273e: movaps xmmword ptr [rsp + 0x100], xmm8
140022747: movaps xmmword ptr [rsp + 0x110], xmm7
14002274f: movaps xmmword ptr [rsp + 0x120], xmm6
140022757: test byte ptr [rbp + 0x7c0], 1
14002275e: je 0x1400227e6
140022764: lea rcx, [rbp + 0x590]
14002276b: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140022770: mov byte ptr [rbp + 0x817], 1
140022777: movaps xmm6, xmmword ptr [rsp + 0x120]
14002277f: movaps xmm7, xmmword ptr [rsp + 0x110]
140022787: movaps xmm8, xmmword ptr [rsp + 0x100]
140022790: movaps xmm9, xmmword ptr [rsp + 0xf0]
140022799: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400227a2: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400227ab: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400227b5: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400227bf: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400227c8: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400227d2: add rsp, 0x138
1400227d9: pop rbx
1400227da: pop rdi
1400227db: pop rsi
1400227dc: pop r12
1400227de: pop r13
1400227e0: pop r14
1400227e2: pop r15
1400227e4: pop rbp
1400227e5: ret 
1400227e6: mov byte ptr [rbp + 0x817], 1
1400227ed: movaps xmm6, xmmword ptr [rsp + 0x120]
1400227f5: movaps xmm7, xmmword ptr [rsp + 0x110]
1400227fd: movaps xmm8, xmmword ptr [rsp + 0x100]
140022806: movaps xmm9, xmmword ptr [rsp + 0xf0]
14002280f: movaps xmm10, xmmword ptr [rsp + 0xe0]
140022818: movaps xmm11, xmmword ptr [rsp + 0xd0]
140022821: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14002282b: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140022835: movaps xmm14, xmmword ptr [rsp + 0xa0]
14002283e: movdqa xmm15, xmmword ptr [rsp + 0x90]
140022848: add rsp, 0x138
14002284f: pop rbx
140022850: pop rdi
140022851: pop rsi
140022852: pop r12
140022854: pop r13
140022856: pop r14
140022858: pop r15
14002285a: pop rbp
14002285b: ret 
14002285c: nop dword ptr [rax]
140022860: mov qword ptr [rsp + 0x10], rdx
140022865: push rbp
140022866: push r15
140022868: push r14
14002286a: push r13
14002286c: push r12
14002286e: push rsi
14002286f: push rdi
140022870: push rbx
140022871: sub rsp, 0x138
140022878: lea rbp, [rdx + 0x80]
14002287f: movaps xmmword ptr [rsp + 0x90], xmm15
140022888: movaps xmmword ptr [rsp + 0xa0], xmm14
140022891: movaps xmmword ptr [rsp + 0xb0], xmm13
14002289a: movaps xmmword ptr [rsp + 0xc0], xmm12
1400228a3: movaps xmmword ptr [rsp + 0xd0], xmm11
1400228ac: movaps xmmword ptr [rsp + 0xe0], xmm10
1400228b5: movaps xmmword ptr [rsp + 0xf0], xmm9
1400228be: movaps xmmword ptr [rsp + 0x100], xmm8
1400228c7: movaps xmmword ptr [rsp + 0x110], xmm7
1400228cf: movaps xmmword ptr [rsp + 0x120], xmm6
1400228d7: movzx ebx, byte ptr [rbp + 0x811]
1400228de: movzx esi, byte ptr [rbp + 0x812]
1400228e5: cmp qword ptr [rbp + 0x7a0], 0
1400228ed: je 0x14002290c
1400228ef: mov rdx, qword ptr [rbp + 0x7a0]
1400228f6: shl rdx, 2
1400228fa: mov r8d, 4
140022900: mov rcx, qword ptr [rbp + 0x808]
140022907: call 0x140613c20 ; ?
14002290c: mov byte ptr [rbp + 0x79c], sil
140022913: and bl, 1
140022916: mov byte ptr [rbp + 0x79b], bl
14002291c: movaps xmm6, xmmword ptr [rsp + 0x120]
140022924: movaps xmm7, xmmword ptr [rsp + 0x110]
14002292c: movaps xmm8, xmmword ptr [rsp + 0x100]
140022935: movaps xmm9, xmmword ptr [rsp + 0xf0]
14002293e: movaps xmm10, xmmword ptr [rsp + 0xe0]
140022947: movapd xmm11, xmmword ptr [rsp + 0xd0]
140022951: movaps xmm12, xmmword ptr [rsp + 0xc0]
14002295a: movaps xmm13, xmmword ptr [rsp + 0xb0]
140022963: movaps xmm14, xmmword ptr [rsp + 0xa0]
14002296c: movaps xmm15, xmmword ptr [rsp + 0x90]
140022975: add rsp, 0x138
14002297c: pop rbx
14002297d: pop rdi
14002297e: pop rsi
14002297f: pop r12
140022981: pop r13
140022983: pop r14
140022985: pop r15
140022987: pop rbp
140022988: ret 
140022989: nop dword ptr [rax]
140022990: mov qword ptr [rsp + 0x10], rdx
140022995: push rbp
140022996: push r15
140022998: push r14
14002299a: push r13
14002299c: push r12
14002299e: push rsi
14002299f: push rdi
1400229a0: push rbx
1400229a1: sub rsp, 0x138
1400229a8: lea rbp, [rdx + 0x80]
1400229af: movdqa xmmword ptr [rsp + 0x90], xmm15
1400229b9: movaps xmmword ptr [rsp + 0xa0], xmm14
1400229c2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400229cc: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400229d6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400229df: movaps xmmword ptr [rsp + 0xe0], xmm10
1400229e8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400229f1: movaps xmmword ptr [rsp + 0x100], xmm8
1400229fa: movaps xmmword ptr [rsp + 0x110], xmm7
140022a02: movaps xmmword ptr [rsp + 0x120], xmm6
140022a0a: mov byte ptr [rbp + 0x817], 0
140022a11: lea rcx, [rbp + 0x6d0]
140022a18: call 0x140601be0 ; core::ptr::drop_in_place<country_core::systems::collision::world::RaycastMeshData>
140022a1d: mov byte ptr [rbp + 0x817], 0
140022a24: movaps xmm6, xmmword ptr [rsp + 0x120]
140022a2c: movaps xmm7, xmmword ptr [rsp + 0x110]
140022a34: movaps xmm8, xmmword ptr [rsp + 0x100]
140022a3d: movaps xmm9, xmmword ptr [rsp + 0xf0]
140022a46: movaps xmm10, xmmword ptr [rsp + 0xe0]
140022a4f: movaps xmm11, xmmword ptr [rsp + 0xd0]
140022a58: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140022a62: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140022a6c: movaps xmm14, xmmword ptr [rsp + 0xa0]
140022a75: movdqa xmm15, xmmword ptr [rsp + 0x90]
140022a7f: add rsp, 0x138
140022a86: pop rbx
140022a87: pop rdi
140022a88: pop rsi
140022a89: pop r12
140022a8b: pop r13
140022a8d: pop r14
140022a8f: pop r15
140022a91: pop rbp
140022a92: ret 
140022a93: nop word ptr cs:[rax + rax]
140022aa0: mov qword ptr [rsp + 0x10], rdx
140022aa5: push rbp
140022aa6: push r15
140022aa8: push r14
140022aaa: push r13
140022aac: push r12
140022aae: push rsi
140022aaf: push rdi
140022ab0: push rbx
140022ab1: sub rsp, 0x138
140022ab8: lea rbp, [rdx + 0x80]
140022abf: movdqa xmmword ptr [rsp + 0x90], xmm15
140022ac9: movaps xmmword ptr [rsp + 0xa0], xmm14
140022ad2: movdqa xmmword ptr [rsp + 0xb0], xmm13
140022adc: movdqa xmmword ptr [rsp + 0xc0], xmm12
140022ae6: movaps xmmword ptr [rsp + 0xd0], xmm11
140022aef: movaps xmmword ptr [rsp + 0xe0], xmm10
140022af8: movaps xmmword ptr [rsp + 0xf0], xmm9
140022b01: movaps xmmword ptr [rsp + 0x100], xmm8
140022b0a: movaps xmmword ptr [rsp + 0x110], xmm7
140022b12: movaps xmmword ptr [rsp + 0x120], xmm6
140022b1a: mov byte ptr [rbp + 0x814], 0
140022b21: lea rcx, [rbp + 0x590]
140022b28: call 0x140609eb0 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams>
140022b2d: mov byte ptr [rbp + 0x814], 0
140022b34: movaps xmm6, xmmword ptr [rsp + 0x120]
140022b3c: movaps xmm7, xmmword ptr [rsp + 0x110]
140022b44: movaps xmm8, xmmword ptr [rsp + 0x100]
140022b4d: movaps xmm9, xmmword ptr [rsp + 0xf0]
140022b56: movaps xmm10, xmmword ptr [rsp + 0xe0]
140022b5f: movaps xmm11, xmmword ptr [rsp + 0xd0]
140022b68: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140022b72: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140022b7c: movaps xmm14, xmmword ptr [rsp + 0xa0]
140022b85: movdqa xmm15, xmmword ptr [rsp + 0x90]
140022b8f: add rsp, 0x138
140022b96: pop rbx
140022b97: pop rdi
140022b98: pop rsi
140022b99: pop r12
140022b9b: pop r13
140022b9d: pop r14
140022b9f: pop r15
140022ba1: pop rbp
140022ba2: ret 
140022ba3: nop word ptr cs:[rax + rax]
140022bb0: mov qword ptr [rsp + 0x10], rdx
140022bb5: push rbp
140022bb6: push r15
140022bb8: push r14
140022bba: push r13
140022bbc: push r12
140022bbe: push rsi
140022bbf: push rdi
140022bc0: push rbx
140022bc1: sub rsp, 0x138
140022bc8: lea rbp, [rdx + 0x80]
140022bcf: movdqa xmmword ptr [rsp + 0x90], xmm15
140022bd9: movaps xmmword ptr [rsp + 0xa0], xmm14
140022be2: movdqa xmmword ptr [rsp + 0xb0], xmm13
140022bec: movdqa xmmword ptr [rsp + 0xc0], xmm12
140022bf6: movaps xmmword ptr [rsp + 0xd0], xmm11
140022bff: movaps xmmword ptr [rsp + 0xe0], xmm10
140022c08: movaps xmmword ptr [rsp + 0xf0], xmm9
140022c11: movaps xmmword ptr [rsp + 0x100], xmm8
140022c1a: movaps xmmword ptr [rsp + 0x110], xmm7
140022c22: movaps xmmword ptr [rsp + 0x120], xmm6
140022c2a: mov byte ptr [rbp + 0x7ce], 0
140022c31: lea rcx, [rbp + 0x140]
140022c38: call 0x140609eb0 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams>
140022c3d: mov byte ptr [rbp + 0x7ce], 0
140022c44: movaps xmm6, xmmword ptr [rsp + 0x120]
140022c4c: movaps xmm7, xmmword ptr [rsp + 0x110]
140022c54: movaps xmm8, xmmword ptr [rsp + 0x100]
140022c5d: movaps xmm9, xmmword ptr [rsp + 0xf0]
140022c66: movaps xmm10, xmmword ptr [rsp + 0xe0]
140022c6f: movaps xmm11, xmmword ptr [rsp + 0xd0]
140022c78: movaps xmm12, xmmword ptr [rsp + 0xc0]
140022c81: movaps xmm13, xmmword ptr [rsp + 0xb0]
140022c8a: movaps xmm14, xmmword ptr [rsp + 0xa0]
140022c93: movaps xmm15, xmmword ptr [rsp + 0x90]
140022c9c: add rsp, 0x138
140022ca3: pop rbx
140022ca4: pop rdi
140022ca5: pop rsi
140022ca6: pop r12
140022ca8: pop r13
140022caa: pop r14
140022cac: pop r15
140022cae: pop rbp
140022caf: ret 
140022cb0: mov qword ptr [rsp + 0x10], rdx
140022cb5: push rbp
140022cb6: push r15
140022cb8: push r14
140022cba: push r13
140022cbc: push r12
140022cbe: push rsi
140022cbf: push rdi
140022cc0: push rbx
140022cc1: sub rsp, 0x138
140022cc8: lea rbp, [rdx + 0x80]
140022ccf: movdqa xmmword ptr [rsp + 0x90], xmm15
140022cd9: movaps xmmword ptr [rsp + 0xa0], xmm14
140022ce2: movdqa xmmword ptr [rsp + 0xb0], xmm13
140022cec: movdqa xmmword ptr [rsp + 0xc0], xmm12
140022cf6: movaps xmmword ptr [rsp + 0xd0], xmm11
140022cff: movaps xmmword ptr [rsp + 0xe0], xmm10
140022d08: movaps xmmword ptr [rsp + 0xf0], xmm9
140022d11: movaps xmmword ptr [rsp + 0x100], xmm8
140022d1a: movaps xmmword ptr [rsp + 0x110], xmm7
140022d22: movaps xmmword ptr [rsp + 0x120], xmm6
140022d2a: mov byte ptr [rbp + 0x7cf], 0
140022d31: lea rcx, [rbp + 0x140]
140022d38: call 0x140609eb0 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams>
140022d3d: mov byte ptr [rbp + 0x7cf], 0
140022d44: movaps xmm6, xmmword ptr [rsp + 0x120]
140022d4c: movaps xmm7, xmmword ptr [rsp + 0x110]
140022d54: movaps xmm8, xmmword ptr [rsp + 0x100]
140022d5d: movaps xmm9, xmmword ptr [rsp + 0xf0]
140022d66: movaps xmm10, xmmword ptr [rsp + 0xe0]
140022d6f: movaps xmm11, xmmword ptr [rsp + 0xd0]
140022d78: movaps xmm12, xmmword ptr [rsp + 0xc0]
140022d81: movaps xmm13, xmmword ptr [rsp + 0xb0]
140022d8a: movaps xmm14, xmmword ptr [rsp + 0xa0]
140022d93: movaps xmm15, xmmword ptr [rsp + 0x90]
140022d9c: add rsp, 0x138
140022da3: pop rbx
140022da4: pop rdi
140022da5: pop rsi
140022da6: pop r12
140022da8: pop r13
140022daa: pop r14
140022dac: pop r15
140022dae: pop rbp
140022daf: ret 
140022db0: mov qword ptr [rsp + 0x10], rdx
140022db5: push rbp
140022db6: push r15
140022db8: push r14
140022dba: push r13
140022dbc: push r12
140022dbe: push rsi
140022dbf: push rdi
140022dc0: push rbx
140022dc1: sub rsp, 0x138
140022dc8: lea rbp, [rdx + 0x80]
140022dcf: movaps xmmword ptr [rsp + 0x90], xmm15
140022dd8: movaps xmmword ptr [rsp + 0xa0], xmm14
140022de1: movaps xmmword ptr [rsp + 0xb0], xmm13
140022dea: movaps xmmword ptr [rsp + 0xc0], xmm12
140022df3: movapd xmmword ptr [rsp + 0xd0], xmm11
140022dfd: movaps xmmword ptr [rsp + 0xe0], xmm10
140022e06: movaps xmmword ptr [rsp + 0xf0], xmm9
140022e0f: movaps xmmword ptr [rsp + 0x100], xmm8
140022e18: movaps xmmword ptr [rsp + 0x110], xmm7
140022e20: movaps xmmword ptr [rsp + 0x120], xmm6
140022e28: movzx ebx, byte ptr [rbp + 0x79b]
140022e2f: movzx esi, byte ptr [rbp + 0x79c]
140022e36: test sil, 1
140022e3a: je 0x140022edf
140022e40: mov rdx, qword ptr [rbp + 0x3e0]
140022e47: test rdx, rdx
140022e4a: je 0x140022e62
140022e4c: mov rcx, qword ptr [rbp + 0x3e8]
140022e53: shl rdx, 2
140022e57: mov r8d, 4
140022e5d: call 0x140613c20 ; ?
140022e62: mov byte ptr [rbp + 0x7bc], sil
140022e69: and bl, 1
140022e6c: mov byte ptr [rbp + 0x7bb], bl
140022e72: movaps xmm6, xmmword ptr [rsp + 0x120]
140022e7a: movaps xmm7, xmmword ptr [rsp + 0x110]
140022e82: movaps xmm8, xmmword ptr [rsp + 0x100]
140022e8b: movaps xmm9, xmmword ptr [rsp + 0xf0]
140022e94: movaps xmm10, xmmword ptr [rsp + 0xe0]
140022e9d: movapd xmm11, xmmword ptr [rsp + 0xd0]
140022ea7: movaps xmm12, xmmword ptr [rsp + 0xc0]
140022eb0: movaps xmm13, xmmword ptr [rsp + 0xb0]
140022eb9: movaps xmm14, xmmword ptr [rsp + 0xa0]
140022ec2: movaps xmm15, xmmword ptr [rsp + 0x90]
140022ecb: add rsp, 0x138
140022ed2: pop rbx
140022ed3: pop rdi
140022ed4: pop rsi
140022ed5: pop r12
140022ed7: pop r13
140022ed9: pop r14
140022edb: pop r15
140022edd: pop rbp
140022ede: ret 
140022edf: mov byte ptr [rbp + 0x7bc], sil
140022ee6: and bl, 1
140022ee9: mov byte ptr [rbp + 0x7bb], bl
140022eef: movaps xmm6, xmmword ptr [rsp + 0x120]
140022ef7: movaps xmm7, xmmword ptr [rsp + 0x110]
140022eff: movaps xmm8, xmmword ptr [rsp + 0x100]
140022f08: movaps xmm9, xmmword ptr [rsp + 0xf0]
140022f11: movaps xmm10, xmmword ptr [rsp + 0xe0]
140022f1a: movapd xmm11, xmmword ptr [rsp + 0xd0]
140022f24: movaps xmm12, xmmword ptr [rsp + 0xc0]
140022f2d: movaps xmm13, xmmword ptr [rsp + 0xb0]
140022f36: movaps xmm14, xmmword ptr [rsp + 0xa0]
140022f3f: movaps xmm15, xmmword ptr [rsp + 0x90]
140022f48: add rsp, 0x138
140022f4f: pop rbx
140022f50: pop rdi
140022f51: pop rsi
140022f52: pop r12
140022f54: pop r13
140022f56: pop r14
140022f58: pop r15
140022f5a: pop rbp
140022f5b: ret 
140022f5c: nop dword ptr [rax]
140022f60: mov qword ptr [rsp + 0x10], rdx
140022f65: push rbp
140022f66: push r15
140022f68: push r14
140022f6a: push r13
140022f6c: push r12
140022f6e: push rsi
140022f6f: push rdi
140022f70: push rbx
140022f71: sub rsp, 0x138
140022f78: lea rbp, [rdx + 0x80]
140022f7f: movaps xmmword ptr [rsp + 0x90], xmm15
140022f88: movaps xmmword ptr [rsp + 0xa0], xmm14
140022f91: movaps xmmword ptr [rsp + 0xb0], xmm13
140022f9a: movaps xmmword ptr [rsp + 0xc0], xmm12
140022fa3: movapd xmmword ptr [rsp + 0xd0], xmm11
140022fad: movaps xmmword ptr [rsp + 0xe0], xmm10
140022fb6: movaps xmmword ptr [rsp + 0xf0], xmm9
140022fbf: movaps xmmword ptr [rsp + 0x100], xmm8
140022fc8: movaps xmmword ptr [rsp + 0x110], xmm7
140022fd0: movaps xmmword ptr [rsp + 0x120], xmm6
140022fd8: movzx ebx, byte ptr [rbp + 0x7bb]
140022fdf: test byte ptr [rbp + 0x7bc], 1
140022fe6: je 0x140023084
140022fec: mov rdx, qword ptr [rbp + 0x508]
140022ff3: test rdx, rdx
140022ff6: je 0x14002300e
140022ff8: mov rcx, qword ptr [rbp + 0x510]
140022fff: shl rdx, 3
140023003: mov r8d, 4
140023009: call 0x140613c20 ; ?
14002300e: and bl, 1
140023011: mov byte ptr [rbp + 0x7bd], bl
140023017: movaps xmm6, xmmword ptr [rsp + 0x120]
14002301f: movaps xmm7, xmmword ptr [rsp + 0x110]
140023027: movaps xmm8, xmmword ptr [rsp + 0x100]
140023030: movaps xmm9, xmmword ptr [rsp + 0xf0]
140023039: movaps xmm10, xmmword ptr [rsp + 0xe0]
140023042: movapd xmm11, xmmword ptr [rsp + 0xd0]
14002304c: movaps xmm12, xmmword ptr [rsp + 0xc0]
140023055: movaps xmm13, xmmword ptr [rsp + 0xb0]
14002305e: movaps xmm14, xmmword ptr [rsp + 0xa0]
140023067: movaps xmm15, xmmword ptr [rsp + 0x90]
140023070: add rsp, 0x138
140023077: pop rbx
140023078: pop rdi
140023079: pop rsi
14002307a: pop r12
14002307c: pop r13
14002307e: pop r14
140023080: pop r15
140023082: pop rbp
140023083: ret 
140023084: and bl, 1
140023087: mov byte ptr [rbp + 0x7bd], bl
14002308d: movaps xmm6, xmmword ptr [rsp + 0x120]
140023095: movaps xmm7, xmmword ptr [rsp + 0x110]
14002309d: movaps xmm8, xmmword ptr [rsp + 0x100]
1400230a6: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400230af: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400230b8: movapd xmm11, xmmword ptr [rsp + 0xd0]
1400230c2: movaps xmm12, xmmword ptr [rsp + 0xc0]
1400230cb: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400230d4: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400230dd: movaps xmm15, xmmword ptr [rsp + 0x90]
1400230e6: add rsp, 0x138
1400230ed: pop rbx
1400230ee: pop rdi
1400230ef: pop rsi
1400230f0: pop r12
1400230f2: pop r13
1400230f4: pop r14
1400230f6: pop r15
1400230f8: pop rbp
1400230f9: ret 
1400230fa: nop word ptr [rax + rax]
140023100: mov qword ptr [rsp + 0x10], rdx
140023105: push rbp
140023106: push r15
140023108: push r14
14002310a: push r13
14002310c: push r12
14002310e: push rsi
14002310f: push rdi
140023110: push rbx
140023111: sub rsp, 0x138
140023118: lea rbp, [rdx + 0x80]
14002311f: movdqa xmmword ptr [rsp + 0x90], xmm15
140023129: movaps xmmword ptr [rsp + 0xa0], xmm14
140023132: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002313c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140023146: movaps xmmword ptr [rsp + 0xd0], xmm11
14002314f: movaps xmmword ptr [rsp + 0xe0], xmm10
140023158: movaps xmmword ptr [rsp + 0xf0], xmm9
140023161: movaps xmmword ptr [rsp + 0x100], xmm8
14002316a: movaps xmmword ptr [rsp + 0x110], xmm7
140023172: movaps xmmword ptr [rsp + 0x120], xmm6
14002317a: mov rcx, qword ptr [rbp + 0x1b8]
140023181: call 0x1405df260 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo> >
140023186: movaps xmm6, xmmword ptr [rsp + 0x120]
14002318e: movaps xmm7, xmmword ptr [rsp + 0x110]
140023196: movaps xmm8, xmmword ptr [rsp + 0x100]
14002319f: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400231a8: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400231b1: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400231ba: movaps xmm12, xmmword ptr [rsp + 0xc0]
1400231c3: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400231cc: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400231d5: movaps xmm15, xmmword ptr [rsp + 0x90]
1400231de: add rsp, 0x138
1400231e5: pop rbx
1400231e6: pop rdi
1400231e7: pop rsi
1400231e8: pop r12
1400231ea: pop r13
1400231ec: pop r14
1400231ee: pop r15
1400231f0: pop rbp
1400231f1: ret 
1400231f2: nop word ptr cs:[rax + rax]
140023200: mov qword ptr [rsp + 0x10], rdx
140023205: push rbp
140023206: push r15
140023208: push r14
14002320a: push r13
14002320c: push r12
14002320e: push rsi
14002320f: push rdi
140023210: push rbx
140023211: sub rsp, 0x138
140023218: lea rbp, [rdx + 0x80]
14002321f: movaps xmmword ptr [rsp + 0x90], xmm15
140023228: movaps xmmword ptr [rsp + 0xa0], xmm14
140023231: movaps xmmword ptr [rsp + 0xb0], xmm13
14002323a: movaps xmmword ptr [rsp + 0xc0], xmm12
140023243: movaps xmmword ptr [rsp + 0xd0], xmm11
14002324c: movaps xmmword ptr [rsp + 0xe0], xmm10
140023255: movaps xmmword ptr [rsp + 0xf0], xmm9
14002325e: movaps xmmword ptr [rsp + 0x100], xmm8
140023267: movaps xmmword ptr [rsp + 0x110], xmm7
14002326f: movaps xmmword ptr [rsp + 0x120], xmm6
140023277: cmp byte ptr [rbp + 0x7bd], 0
14002327e: je 0x140023306
140023284: lea rcx, [rbp + 0x460]
14002328b: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140023290: mov byte ptr [rbp + 0x817], 1
140023297: movaps xmm6, xmmword ptr [rsp + 0x120]
14002329f: movaps xmm7, xmmword ptr [rsp + 0x110]
1400232a7: movaps xmm8, xmmword ptr [rsp + 0x100]
1400232b0: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400232b9: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400232c2: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400232cb: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400232d5: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400232df: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400232e8: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400232f2: add rsp, 0x138
1400232f9: pop rbx
1400232fa: pop rdi
1400232fb: pop rsi
1400232fc: pop r12
1400232fe: pop r13
140023300: pop r14
140023302: pop r15
140023304: pop rbp
140023305: ret 
140023306: mov byte ptr [rbp + 0x817], 1
14002330d: movaps xmm6, xmmword ptr [rsp + 0x120]
140023315: movaps xmm7, xmmword ptr [rsp + 0x110]
14002331d: movaps xmm8, xmmword ptr [rsp + 0x100]
140023326: movaps xmm9, xmmword ptr [rsp + 0xf0]
14002332f: movaps xmm10, xmmword ptr [rsp + 0xe0]
140023338: movaps xmm11, xmmword ptr [rsp + 0xd0]
140023341: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14002334b: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140023355: movaps xmm14, xmmword ptr [rsp + 0xa0]
14002335e: movdqa xmm15, xmmword ptr [rsp + 0x90]
140023368: add rsp, 0x138
14002336f: pop rbx
140023370: pop rdi
140023371: pop rsi
140023372: pop r12
140023374: pop r13
140023376: pop r14
140023378: pop r15
14002337a: pop rbp
14002337b: ret 
14002337c: nop dword ptr [rax]
140023380: mov qword ptr [rsp + 0x10], rdx
140023385: push rbp
140023386: push r15
140023388: push r14
14002338a: push r13
14002338c: push r12
14002338e: push rsi
14002338f: push rdi
140023390: push rbx
140023391: sub rsp, 0x138
140023398: lea rbp, [rdx + 0x80]
14002339f: movdqa xmmword ptr [rsp + 0x90], xmm15
1400233a9: movaps xmmword ptr [rsp + 0xa0], xmm14
1400233b2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400233bc: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400233c6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400233cf: movaps xmmword ptr [rsp + 0xe0], xmm10
1400233d8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400233e1: movaps xmmword ptr [rsp + 0x100], xmm8
1400233ea: movaps xmmword ptr [rsp + 0x110], xmm7
1400233f2: movaps xmmword ptr [rsp + 0x120], xmm6
1400233fa: mov byte ptr [rbp + 0x814], 0
140023401: lea rcx, [rbp + 0x590]
140023408: call 0x140607a30 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsBracketAssemblyParams>
14002340d: mov byte ptr [rbp + 0x814], 0
140023414: movaps xmm6, xmmword ptr [rsp + 0x120]
14002341c: movaps xmm7, xmmword ptr [rsp + 0x110]
140023424: movaps xmm8, xmmword ptr [rsp + 0x100]
14002342d: movaps xmm9, xmmword ptr [rsp + 0xf0]
140023436: movaps xmm10, xmmword ptr [rsp + 0xe0]
14002343f: movaps xmm11, xmmword ptr [rsp + 0xd0]
140023448: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140023452: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14002345c: movaps xmm14, xmmword ptr [rsp + 0xa0]
140023465: movdqa xmm15, xmmword ptr [rsp + 0x90]
14002346f: add rsp, 0x138
140023476: pop rbx
140023477: pop rdi
140023478: pop rsi
140023479: pop r12
14002347b: pop r13
14002347d: pop r14
14002347f: pop r15
140023481: pop rbp
140023482: ret 
140023483: nop word ptr cs:[rax + rax]
140023490: mov qword ptr [rsp + 0x10], rdx
140023495: push rbp
140023496: push r15
140023498: push r14
14002349a: push r13
14002349c: push r12
14002349e: push rsi
14002349f: push rdi
1400234a0: push rbx
1400234a1: sub rsp, 0x138
1400234a8: lea rbp, [rdx + 0x80]
1400234af: movdqa xmmword ptr [rsp + 0x90], xmm15
1400234b9: movaps xmmword ptr [rsp + 0xa0], xmm14
1400234c2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400234cc: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400234d6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400234df: movaps xmmword ptr [rsp + 0xe0], xmm10
1400234e8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400234f1: movaps xmmword ptr [rsp + 0x100], xmm8
1400234fa: movaps xmmword ptr [rsp + 0x110], xmm7
140023502: movaps xmmword ptr [rsp + 0x120], xmm6
14002350a: cmp qword ptr [rbp + 0x808], 0
140023512: je 0x140023535
140023514: mov rax, qword ptr [rbp + 0x808]
14002351b: lea rdx, [rax*4]
140023523: mov r8d, 4
140023529: mov rcx, qword ptr [rbp + 0x6b0]
140023530: call 0x140613c20 ; ?
140023535: cmp qword ptr [rbp + 0x7f0], 0
14002353d: je 0x14002355c
14002353f: mov rdx, qword ptr [rbp + 0x7f0]
140023546: shl rdx, 2
14002354a: mov r8d, 4
140023550: mov rcx, qword ptr [rbp + 0x7d8]
140023557: call 0x140613c20 ; ?
14002355c: mov byte ptr [rbp + 0x79d], 0
140023563: movaps xmm6, xmmword ptr [rsp + 0x120]
14002356b: movaps xmm7, xmmword ptr [rsp + 0x110]
140023573: movaps xmm8, xmmword ptr [rsp + 0x100]
14002357c: movaps xmm9, xmmword ptr [rsp + 0xf0]
140023585: movaps xmm10, xmmword ptr [rsp + 0xe0]
14002358e: movaps xmm11, xmmword ptr [rsp + 0xd0]
140023597: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400235a1: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400235ab: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400235b4: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400235be: add rsp, 0x138
1400235c5: pop rbx
1400235c6: pop rdi
1400235c7: pop rsi
1400235c8: pop r12
1400235ca: pop r13
1400235cc: pop r14
1400235ce: pop r15
1400235d0: pop rbp
1400235d1: ret 
1400235d2: nop word ptr cs:[rax + rax]
1400235e0: mov qword ptr [rsp + 0x10], rdx
1400235e5: push rbp
1400235e6: push r15
1400235e8: push r14
1400235ea: push r13
1400235ec: push r12
1400235ee: push rsi
1400235ef: push rdi
1400235f0: push rbx
1400235f1: sub rsp, 0x138
1400235f8: lea rbp, [rdx + 0x80]
1400235ff: movdqa xmmword ptr [rsp + 0x90], xmm15
140023609: movaps xmmword ptr [rsp + 0xa0], xmm14
140023612: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002361c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140023626: movaps xmmword ptr [rsp + 0xd0], xmm11
14002362f: movaps xmmword ptr [rsp + 0xe0], xmm10
140023638: movaps xmmword ptr [rsp + 0xf0], xmm9
140023641: movaps xmmword ptr [rsp + 0x100], xmm8
14002364a: movaps xmmword ptr [rsp + 0x110], xmm7
140023652: movaps xmmword ptr [rsp + 0x120], xmm6
14002365a: cmp qword ptr [rbp + 0x808], 0
140023662: je 0x140023685
140023664: mov rax, qword ptr [rbp + 0x808]
14002366b: lea rdx, [rax*4]
140023673: mov r8d, 4
140023679: mov rcx, qword ptr [rbp + 0x6b0]
140023680: call 0x140613c20 ; ?
140023685: cmp qword ptr [rbp + 0x7f0], 0
14002368d: je 0x1400236ac
14002368f: mov rdx, qword ptr [rbp + 0x7f0]
140023696: shl rdx, 2
14002369a: mov r8d, 4
1400236a0: mov rcx, qword ptr [rbp + 0x7d8]
1400236a7: call 0x140613c20 ; ?
1400236ac: mov byte ptr [rbp + 0x79e], 0
1400236b3: movaps xmm6, xmmword ptr [rsp + 0x120]
1400236bb: movaps xmm7, xmmword ptr [rsp + 0x110]
1400236c3: movaps xmm8, xmmword ptr [rsp + 0x100]
1400236cc: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400236d5: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400236de: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400236e7: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400236f1: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400236fb: movaps xmm14, xmmword ptr [rsp + 0xa0]
140023704: movdqa xmm15, xmmword ptr [rsp + 0x90]
14002370e: add rsp, 0x138
140023715: pop rbx
140023716: pop rdi
140023717: pop rsi
140023718: pop r12
14002371a: pop r13
14002371c: pop r14
14002371e: pop r15
140023720: pop rbp
140023721: ret 
140023722: nop word ptr cs:[rax + rax]
140023730: mov qword ptr [rsp + 0x10], rdx
140023735: push rbp
140023736: push r15
140023738: push r14
14002373a: push r13
14002373c: push r12
14002373e: push rsi
14002373f: push rdi
140023740: push rbx
140023741: sub rsp, 0x138
140023748: lea rbp, [rdx + 0x80]
14002374f: movdqa xmmword ptr [rsp + 0x90], xmm15
140023759: movaps xmmword ptr [rsp + 0xa0], xmm14
140023762: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002376c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140023776: movaps xmmword ptr [rsp + 0xd0], xmm11
14002377f: movaps xmmword ptr [rsp + 0xe0], xmm10
140023788: movaps xmmword ptr [rsp + 0xf0], xmm9
140023791: movaps xmmword ptr [rsp + 0x100], xmm8
14002379a: movaps xmmword ptr [rsp + 0x110], xmm7
1400237a2: movaps xmmword ptr [rsp + 0x120], xmm6
1400237aa: movzx ebx, byte ptr [rbp + 0x79d]
1400237b1: cmp qword ptr [rbp + 0x7a0], 0
1400237b9: je 0x1400237d8
1400237bb: mov rdx, qword ptr [rbp + 0x7a0]
1400237c2: shl rdx, 3
1400237c6: mov r8d, 4
1400237cc: mov rcx, qword ptr [rbp + 0x7d0]
1400237d3: call 0x140613c20 ; ?
1400237d8: lea rcx, [rbp + 0x590]
1400237df: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
1400237e4: and bl, 1
1400237e7: mov byte ptr [rbp + 0x7ce], bl
1400237ed: movaps xmm6, xmmword ptr [rsp + 0x120]
1400237f5: movaps xmm7, xmmword ptr [rsp + 0x110]
1400237fd: movaps xmm8, xmmword ptr [rsp + 0x100]
140023806: movaps xmm9, xmmword ptr [rsp + 0xf0]
14002380f: movaps xmm10, xmmword ptr [rsp + 0xe0]
140023818: movaps xmm11, xmmword ptr [rsp + 0xd0]
140023821: movaps xmm12, xmmword ptr [rsp + 0xc0]
14002382a: movaps xmm13, xmmword ptr [rsp + 0xb0]
140023833: movaps xmm14, xmmword ptr [rsp + 0xa0]
14002383c: movaps xmm15, xmmword ptr [rsp + 0x90]
140023845: add rsp, 0x138
14002384c: pop rbx
14002384d: pop rdi
14002384e: pop rsi
14002384f: pop r12
140023851: pop r13
140023853: pop r14
140023855: pop r15
140023857: pop rbp
140023858: ret 
140023859: nop dword ptr [rax]
140023860: mov qword ptr [rsp + 0x10], rdx
140023865: push rbp
140023866: push r15
140023868: push r14
14002386a: push r13
14002386c: push r12
14002386e: push rsi
14002386f: push rdi
140023870: push rbx
140023871: sub rsp, 0x138
140023878: lea rbp, [rdx + 0x80]
14002387f: movdqa xmmword ptr [rsp + 0x90], xmm15
140023889: movaps xmmword ptr [rsp + 0xa0], xmm14
140023892: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002389c: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400238a6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400238af: movaps xmmword ptr [rsp + 0xe0], xmm10
1400238b8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400238c1: movaps xmmword ptr [rsp + 0x100], xmm8
1400238ca: movaps xmmword ptr [rsp + 0x110], xmm7
1400238d2: movaps xmmword ptr [rsp + 0x120], xmm6
1400238da: movzx ebx, byte ptr [rbp + 0x79e]
1400238e1: cmp qword ptr [rbp + 0x7a0], 0
1400238e9: je 0x140023908
1400238eb: mov rdx, qword ptr [rbp + 0x7a0]
1400238f2: shl rdx, 3
1400238f6: mov r8d, 4
1400238fc: mov rcx, qword ptr [rbp + 0x7d0]
140023903: call 0x140613c20 ; ?
140023908: lea rcx, [rbp + 0x590]
14002390f: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140023914: and bl, 1
140023917: mov byte ptr [rbp + 0x7cf], bl
14002391d: movaps xmm6, xmmword ptr [rsp + 0x120]
140023925: movaps xmm7, xmmword ptr [rsp + 0x110]
14002392d: movaps xmm8, xmmword ptr [rsp + 0x100]
140023936: movaps xmm9, xmmword ptr [rsp + 0xf0]
14002393f: movaps xmm10, xmmword ptr [rsp + 0xe0]
140023948: movaps xmm11, xmmword ptr [rsp + 0xd0]
140023951: movaps xmm12, xmmword ptr [rsp + 0xc0]
14002395a: movaps xmm13, xmmword ptr [rsp + 0xb0]
140023963: movaps xmm14, xmmword ptr [rsp + 0xa0]
14002396c: movaps xmm15, xmmword ptr [rsp + 0x90]
140023975: add rsp, 0x138
14002397c: pop rbx
14002397d: pop rdi
14002397e: pop rsi
14002397f: pop r12
140023981: pop r13
140023983: pop r14
140023985: pop r15
140023987: pop rbp
140023988: ret 
140023989: nop dword ptr [rax]
140023990: mov qword ptr [rsp + 0x10], rdx
140023995: push rbp
140023996: push r15
140023998: push r14
14002399a: push r13
14002399c: push r12
14002399e: push rsi
14002399f: push rdi
1400239a0: push rbx
1400239a1: sub rsp, 0x138
1400239a8: lea rbp, [rdx + 0x80]
1400239af: movaps xmmword ptr [rsp + 0x90], xmm15
1400239b8: movaps xmmword ptr [rsp + 0xa0], xmm14
1400239c1: movaps xmmword ptr [rsp + 0xb0], xmm13
1400239ca: movaps xmmword ptr [rsp + 0xc0], xmm12
1400239d3: movaps xmmword ptr [rsp + 0xd0], xmm11
1400239dc: movaps xmmword ptr [rsp + 0xe0], xmm10
1400239e5: movaps xmmword ptr [rsp + 0xf0], xmm9
1400239ee: movaps xmmword ptr [rsp + 0x100], xmm8
1400239f7: movaps xmmword ptr [rsp + 0x110], xmm7
1400239ff: movaps xmmword ptr [rsp + 0x120], xmm6
140023a07: cmp qword ptr [rbp + 0x808], 0
140023a0f: setne al
140023a12: test byte ptr [rbp + 0x7ce], al
140023a18: je 0x140023a37
140023a1a: mov rdx, qword ptr [rbp + 0x808]
140023a21: shl rdx, 2
140023a25: mov r8d, 4
140023a2b: mov rcx, qword ptr [rbp + 0x6b0]
140023a32: call 0x140613c20 ; ?
140023a37: movaps xmm6, xmmword ptr [rsp + 0x120]
140023a3f: movaps xmm7, xmmword ptr [rsp + 0x110]
140023a47: movaps xmm8, xmmword ptr [rsp + 0x100]
140023a50: movaps xmm9, xmmword ptr [rsp + 0xf0]
140023a59: movaps xmm10, xmmword ptr [rsp + 0xe0]
140023a62: movaps xmm11, xmmword ptr [rsp + 0xd0]
140023a6b: movaps xmm12, xmmword ptr [rsp + 0xc0]
140023a74: movaps xmm13, xmmword ptr [rsp + 0xb0]
140023a7d: movaps xmm14, xmmword ptr [rsp + 0xa0]
140023a86: movaps xmm15, xmmword ptr [rsp + 0x90]
140023a8f: add rsp, 0x138
140023a96: pop rbx
140023a97: pop rdi
140023a98: pop rsi
140023a99: pop r12
140023a9b: pop r13
140023a9d: pop r14
140023a9f: pop r15
140023aa1: pop rbp
140023aa2: ret 
140023aa3: nop word ptr cs:[rax + rax]
140023ab0: mov qword ptr [rsp + 0x10], rdx
140023ab5: push rbp
140023ab6: push r15
140023ab8: push r14
140023aba: push r13
140023abc: push r12
140023abe: push rsi
140023abf: push rdi
140023ac0: push rbx
140023ac1: sub rsp, 0x138
140023ac8: lea rbp, [rdx + 0x80]
140023acf: movaps xmmword ptr [rsp + 0x90], xmm15
140023ad8: movaps xmmword ptr [rsp + 0xa0], xmm14
140023ae1: movaps xmmword ptr [rsp + 0xb0], xmm13
140023aea: movaps xmmword ptr [rsp + 0xc0], xmm12
140023af3: movaps xmmword ptr [rsp + 0xd0], xmm11
140023afc: movaps xmmword ptr [rsp + 0xe0], xmm10
140023b05: movaps xmmword ptr [rsp + 0xf0], xmm9
140023b0e: movaps xmmword ptr [rsp + 0x100], xmm8
140023b17: movaps xmmword ptr [rsp + 0x110], xmm7
140023b1f: movaps xmmword ptr [rsp + 0x120], xmm6
140023b27: cmp qword ptr [rbp + 0x808], 0
140023b2f: setne al
140023b32: test byte ptr [rbp + 0x7cf], al
140023b38: je 0x140023b57
140023b3a: mov rdx, qword ptr [rbp + 0x808]
140023b41: shl rdx, 2
140023b45: mov r8d, 4
140023b4b: mov rcx, qword ptr [rbp + 0x6b0]
140023b52: call 0x140613c20 ; ?
140023b57: movaps xmm6, xmmword ptr [rsp + 0x120]
140023b5f: movaps xmm7, xmmword ptr [rsp + 0x110]
140023b67: movaps xmm8, xmmword ptr [rsp + 0x100]
140023b70: movaps xmm9, xmmword ptr [rsp + 0xf0]
140023b79: movaps xmm10, xmmword ptr [rsp + 0xe0]
140023b82: movaps xmm11, xmmword ptr [rsp + 0xd0]
140023b8b: movaps xmm12, xmmword ptr [rsp + 0xc0]
140023b94: movaps xmm13, xmmword ptr [rsp + 0xb0]
140023b9d: movaps xmm14, xmmword ptr [rsp + 0xa0]
140023ba6: movaps xmm15, xmmword ptr [rsp + 0x90]
140023baf: add rsp, 0x138
140023bb6: pop rbx
140023bb7: pop rdi
140023bb8: pop rsi
140023bb9: pop r12
140023bbb: pop r13
140023bbd: pop r14
140023bbf: pop r15
140023bc1: pop rbp
140023bc2: ret 
140023bc3: nop word ptr cs:[rax + rax]
140023bd0: mov qword ptr [rsp + 0x10], rdx
140023bd5: push rbp
140023bd6: push r15
140023bd8: push r14
140023bda: push r13
140023bdc: push r12
140023bde: push rsi
140023bdf: push rdi
140023be0: push rbx
140023be1: sub rsp, 0x138
140023be8: lea rbp, [rdx + 0x80]
140023bef: movdqa xmmword ptr [rsp + 0x90], xmm15
140023bf9: movaps xmmword ptr [rsp + 0xa0], xmm14
140023c02: movdqa xmmword ptr [rsp + 0xb0], xmm13
140023c0c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140023c16: movaps xmmword ptr [rsp + 0xd0], xmm11
140023c1f: movaps xmmword ptr [rsp + 0xe0], xmm10
140023c28: movaps xmmword ptr [rsp + 0xf0], xmm9
140023c31: movaps xmmword ptr [rsp + 0x100], xmm8
140023c3a: movaps xmmword ptr [rsp + 0x110], xmm7
140023c42: movaps xmmword ptr [rsp + 0x120], xmm6
140023c4a: cmp qword ptr [rbp + 0x808], 0
140023c52: je 0x140023c75
140023c54: mov rax, qword ptr [rbp + 0x808]
140023c5b: shl rax, 2
140023c5f: lea rdx, [rax + rax*2]
140023c63: mov r8d, 4
140023c69: mov rcx, qword ptr [rbp + 0x7a0]
140023c70: call 0x140613c20 ; ?
140023c75: movaps xmm6, xmmword ptr [rsp + 0x120]
140023c7d: movaps xmm7, xmmword ptr [rsp + 0x110]
140023c85: movaps xmm8, xmmword ptr [rsp + 0x100]
140023c8e: movaps xmm9, xmmword ptr [rsp + 0xf0]
140023c97: movaps xmm10, xmmword ptr [rsp + 0xe0]
140023ca0: movaps xmm11, xmmword ptr [rsp + 0xd0]
140023ca9: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140023cb3: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140023cbd: movaps xmm14, xmmword ptr [rsp + 0xa0]
140023cc6: movdqa xmm15, xmmword ptr [rsp + 0x90]
140023cd0: add rsp, 0x138
140023cd7: pop rbx
140023cd8: pop rdi
140023cd9: pop rsi
140023cda: pop r12
140023cdc: pop r13
140023cde: pop r14
140023ce0: pop r15
140023ce2: pop rbp
140023ce3: ret 
140023ce4: nop word ptr cs:[rax + rax]
140023cf0: mov qword ptr [rsp + 0x10], rdx
140023cf5: push rbp
140023cf6: push r15
140023cf8: push r14
140023cfa: push r13
140023cfc: push r12
140023cfe: push rsi
140023cff: push rdi
140023d00: push rbx
140023d01: sub rsp, 0x138
140023d08: lea rbp, [rdx + 0x80]
140023d0f: movdqa xmmword ptr [rsp + 0x90], xmm15
140023d19: movaps xmmword ptr [rsp + 0xa0], xmm14
140023d22: movdqa xmmword ptr [rsp + 0xb0], xmm13
140023d2c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140023d36: movaps xmmword ptr [rsp + 0xd0], xmm11
140023d3f: movaps xmmword ptr [rsp + 0xe0], xmm10
140023d48: movaps xmmword ptr [rsp + 0xf0], xmm9
140023d51: movaps xmmword ptr [rsp + 0x100], xmm8
140023d5a: movaps xmmword ptr [rsp + 0x110], xmm7
140023d62: movaps xmmword ptr [rsp + 0x120], xmm6
140023d6a: movaps xmm0, xmmword ptr [rbp + 0x4d0]
140023d71: movaps xmmword ptr [rbp + 0x730], xmm0
140023d78: movaps xmm0, xmmword ptr [rbp + 0x4c0]
140023d7f: movaps xmmword ptr [rbp + 0x720], xmm0
140023d86: movaps xmm0, xmmword ptr [rbp + 0x4b0]
140023d8d: movaps xmmword ptr [rbp + 0x710], xmm0
140023d94: movaps xmm0, xmmword ptr [rbp + 0x4a0]
140023d9b: movaps xmmword ptr [rbp + 0x700], xmm0
140023da2: movaps xmm0, xmmword ptr [rbp + 0x460]
140023da9: movdqa xmm1, xmmword ptr [rbp + 0x470]
140023db1: movdqa xmm2, xmmword ptr [rbp + 0x480]
140023db9: movapd xmm3, xmmword ptr [rbp + 0x490]
140023dc1: movapd xmmword ptr [rbp + 0x6f0], xmm3
140023dc9: movdqa xmmword ptr [rbp + 0x6e0], xmm2
140023dd1: movdqa xmmword ptr [rbp + 0x6d0], xmm1
140023dd9: movaps xmmword ptr [rbp + 0x6c0], xmm0
140023de0: mov rax, qword ptr [rbp + 0x678]
140023de7: mov qword ptr [rbp + 0x740], rax
140023dee: mov eax, dword ptr [rbp + 0x588]
140023df4: mov dword ptr [rbp + 0x748], eax
140023dfa: mov eax, dword ptr [rbp + 0x640]
140023e00: mov dword ptr [rbp + 0x74c], eax
140023e06: mov eax, dword ptr [rbp + 0x528]
140023e0c: mov dword ptr [rbp + 0x750], eax
140023e12: movdqa xmm0, xmmword ptr [rbp + 0x370]
140023e1a: movd eax, xmm0
140023e1e: mov word ptr [rbp + 0x754], ax
140023e25: mov word ptr [rbp + 0x756], 0
140023e2e: movzx eax, word ptr [rbp + 0x548]
140023e35: mov word ptr [rbp + 0x758], ax
140023e3c: mov word ptr [rbp + 0x75a], 0x300
140023e45: mov rax, qword ptr [rbp + 0x500]
140023e4c: mov qword ptr [rbp + 0x760], rax
140023e53: mov rax, qword ptr [rbp + 0x540]
140023e5a: mov qword ptr [rbp + 0x768], rax
140023e61: mov byte ptr [rbp + 0x770], 0
140023e68: mov byte ptr [rbp + 0x7c2], 0
140023e6f: mov byte ptr [rbp + 0x7c1], 0
140023e76: movaps xmm6, xmmword ptr [rsp + 0x120]
140023e7e: movaps xmm7, xmmword ptr [rsp + 0x110]
140023e86: movaps xmm8, xmmword ptr [rsp + 0x100]
140023e8f: movaps xmm9, xmmword ptr [rsp + 0xf0]
140023e98: movaps xmm10, xmmword ptr [rsp + 0xe0]
140023ea1: movaps xmm11, xmmword ptr [rsp + 0xd0]
140023eaa: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140023eb4: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140023ebe: movaps xmm14, xmmword ptr [rsp + 0xa0]
140023ec7: movdqa xmm15, xmmword ptr [rsp + 0x90]
140023ed1: add rsp, 0x138
140023ed8: pop rbx
140023ed9: pop rdi
140023eda: pop rsi
140023edb: pop r12
140023edd: pop r13
140023edf: pop r14
140023ee1: pop r15
140023ee3: pop rbp
140023ee4: ret 
140023ee5: nop word ptr cs:[rax + rax]
140023ef0: mov qword ptr [rsp + 0x10], rdx
140023ef5: push rbp
140023ef6: push r15
140023ef8: push r14
140023efa: push r13
140023efc: push r12
140023efe: push rsi
140023eff: push rdi
140023f00: push rbx
140023f01: sub rsp, 0x138
140023f08: lea rbp, [rdx + 0x80]
140023f0f: movdqa xmmword ptr [rsp + 0x90], xmm15
140023f19: movaps xmmword ptr [rsp + 0xa0], xmm14
140023f22: movdqa xmmword ptr [rsp + 0xb0], xmm13
140023f2c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140023f36: movaps xmmword ptr [rsp + 0xd0], xmm11
140023f3f: movaps xmmword ptr [rsp + 0xe0], xmm10
140023f48: movaps xmmword ptr [rsp + 0xf0], xmm9
140023f51: movaps xmmword ptr [rsp + 0x100], xmm8
140023f5a: movaps xmmword ptr [rsp + 0x110], xmm7
140023f62: movaps xmmword ptr [rsp + 0x120], xmm6
140023f6a: lea rcx, [rbp + 0x6c0]
140023f71: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140023f76: movaps xmm6, xmmword ptr [rsp + 0x120]
140023f7e: movaps xmm7, xmmword ptr [rsp + 0x110]
140023f86: movaps xmm8, xmmword ptr [rsp + 0x100]
140023f8f: movaps xmm9, xmmword ptr [rsp + 0xf0]
140023f98: movaps xmm10, xmmword ptr [rsp + 0xe0]
140023fa1: movaps xmm11, xmmword ptr [rsp + 0xd0]
140023faa: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140023fb4: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140023fbe: movaps xmm14, xmmword ptr [rsp + 0xa0]
140023fc7: movdqa xmm15, xmmword ptr [rsp + 0x90]
140023fd1: add rsp, 0x138
140023fd8: pop rbx
140023fd9: pop rdi
140023fda: pop rsi
140023fdb: pop r12
140023fdd: pop r13
140023fdf: pop r14
140023fe1: pop r15
140023fe3: pop rbp
140023fe4: ret 
140023fe5: nop word ptr cs:[rax + rax]
140023ff0: mov qword ptr [rsp + 0x10], rdx
140023ff5: push rbp
140023ff6: push r15
140023ff8: push r14
140023ffa: push r13
140023ffc: push r12
140023ffe: push rsi
140023fff: push rdi
140024000: push rbx
140024001: sub rsp, 0x138
140024008: lea rbp, [rdx + 0x80]
14002400f: movdqa xmmword ptr [rsp + 0x90], xmm15
140024019: movaps xmmword ptr [rsp + 0xa0], xmm14
140024022: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002402c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140024036: movaps xmmword ptr [rsp + 0xd0], xmm11
14002403f: movaps xmmword ptr [rsp + 0xe0], xmm10
140024048: movaps xmmword ptr [rsp + 0xf0], xmm9
140024051: movaps xmmword ptr [rsp + 0x100], xmm8
14002405a: movaps xmmword ptr [rsp + 0x110], xmm7
140024062: movaps xmmword ptr [rsp + 0x120], xmm6
14002406a: cmp qword ptr [rbp + 0x790], 0
140024072: je 0x140024095
140024074: mov rax, qword ptr [rbp + 0x790]
14002407b: shl rax, 2
14002407f: lea rdx, [rax + rax*2]
140024083: mov r8d, 4
140024089: mov rcx, qword ptr [rbp + 0x808]
140024090: call 0x140613c20 ; ?
140024095: movaps xmm6, xmmword ptr [rsp + 0x120]
14002409d: movaps xmm7, xmmword ptr [rsp + 0x110]
1400240a5: movaps xmm8, xmmword ptr [rsp + 0x100]
1400240ae: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400240b7: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400240c0: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400240c9: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400240d3: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400240dd: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400240e6: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400240f0: add rsp, 0x138
1400240f7: pop rbx
1400240f8: pop rdi
1400240f9: pop rsi
1400240fa: pop r12
1400240fc: pop r13
1400240fe: pop r14
140024100: pop r15
140024102: pop rbp
140024103: ret 
140024104: nop word ptr cs:[rax + rax]
140024110: mov qword ptr [rsp + 0x10], rdx
140024115: push rbp
140024116: push r15
140024118: push r14
14002411a: push r13
14002411c: push r12
14002411e: push rsi
14002411f: push rdi
140024120: push rbx
140024121: sub rsp, 0x138
140024128: lea rbp, [rdx + 0x80]
14002412f: movdqa xmmword ptr [rsp + 0x90], xmm15
140024139: movaps xmmword ptr [rsp + 0xa0], xmm14
140024142: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002414c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140024156: movaps xmmword ptr [rsp + 0xd0], xmm11
14002415f: movaps xmmword ptr [rsp + 0xe0], xmm10
140024168: movaps xmmword ptr [rsp + 0xf0], xmm9
140024171: movaps xmmword ptr [rsp + 0x100], xmm8
14002417a: movaps xmmword ptr [rsp + 0x110], xmm7
140024182: movaps xmmword ptr [rsp + 0x120], xmm6
14002418a: cmp qword ptr [rbp + 0x790], 0
140024192: je 0x1400241b1
140024194: mov rdx, qword ptr [rbp + 0x790]
14002419b: shl rdx, 3
14002419f: mov r8d, 4
1400241a5: mov rcx, qword ptr [rbp + 0x2f8]
1400241ac: call 0x140613c20 ; ?
1400241b1: movaps xmm6, xmmword ptr [rsp + 0x120]
1400241b9: movaps xmm7, xmmword ptr [rsp + 0x110]
1400241c1: movaps xmm8, xmmword ptr [rsp + 0x100]
1400241ca: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400241d3: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400241dc: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400241e5: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400241ef: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400241f9: movaps xmm14, xmmword ptr [rsp + 0xa0]
140024202: movdqa xmm15, xmmword ptr [rsp + 0x90]
14002420c: add rsp, 0x138
140024213: pop rbx
140024214: pop rdi
140024215: pop rsi
140024216: pop r12
140024218: pop r13
14002421a: pop r14
14002421c: pop r15
14002421e: pop rbp
14002421f: ret 
140024220: mov qword ptr [rsp + 0x10], rdx
140024225: push rbp
140024226: push r15
140024228: push r14
14002422a: push r13
14002422c: push r12
14002422e: push rsi
14002422f: push rdi
140024230: push rbx
140024231: sub rsp, 0x138
140024238: lea rbp, [rdx + 0x80]
14002423f: movdqa xmmword ptr [rsp + 0x90], xmm15
140024249: movaps xmmword ptr [rsp + 0xa0], xmm14
140024252: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002425c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140024266: movaps xmmword ptr [rsp + 0xd0], xmm11
14002426f: movaps xmmword ptr [rsp + 0xe0], xmm10
140024278: movaps xmmword ptr [rsp + 0xf0], xmm9
140024281: movaps xmmword ptr [rsp + 0x100], xmm8
14002428a: movaps xmmword ptr [rsp + 0x110], xmm7
140024292: movaps xmmword ptr [rsp + 0x120], xmm6
14002429a: mov rdx, qword ptr [rbp + 0xd8]
1400242a1: test rdx, rdx
1400242a4: je 0x1400242bc
1400242a6: mov rcx, qword ptr [rbp + 0xe0]
1400242ad: shl rdx, 2
1400242b1: mov r8d, 4
1400242b7: call 0x140613c20 ; ?
1400242bc: movaps xmm6, xmmword ptr [rsp + 0x120]
1400242c4: movaps xmm7, xmmword ptr [rsp + 0x110]
1400242cc: movaps xmm8, xmmword ptr [rsp + 0x100]
1400242d5: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400242de: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400242e7: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400242f0: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400242fa: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140024304: movaps xmm14, xmmword ptr [rsp + 0xa0]
14002430d: movdqa xmm15, xmmword ptr [rsp + 0x90]
140024317: add rsp, 0x138
14002431e: pop rbx
14002431f: pop rdi
140024320: pop rsi
140024321: pop r12
140024323: pop r13
140024325: pop r14
140024327: pop r15
140024329: pop rbp
14002432a: ret 
14002432b: nop dword ptr [rax + rax]
140024330: mov qword ptr [rsp + 0x10], rdx
140024335: push rbp
140024336: push r15
140024338: push r14
14002433a: push r13
14002433c: push r12
14002433e: push rsi
14002433f: push rdi
140024340: push rbx
140024341: sub rsp, 0x138
140024348: lea rbp, [rdx + 0x80]
14002434f: movdqa xmmword ptr [rsp + 0x90], xmm15
140024359: movaps xmmword ptr [rsp + 0xa0], xmm14
140024362: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002436c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140024376: movaps xmmword ptr [rsp + 0xd0], xmm11
14002437f: movaps xmmword ptr [rsp + 0xe0], xmm10
140024388: movaps xmmword ptr [rsp + 0xf0], xmm9
140024391: movaps xmmword ptr [rsp + 0x100], xmm8
14002439a: movaps xmmword ptr [rsp + 0x110], xmm7
1400243a2: movaps xmmword ptr [rsp + 0x120], xmm6
1400243aa: mov rdx, qword ptr [rbp + 0xc8]
1400243b1: test rdx, rdx
1400243b4: je 0x1400243cc
1400243b6: mov rcx, qword ptr [rbp + 0xd0]
1400243bd: shl rdx, 2
1400243c1: mov r8d, 4
1400243c7: call 0x140613c20 ; ?
1400243cc: movaps xmm6, xmmword ptr [rsp + 0x120]
1400243d4: movaps xmm7, xmmword ptr [rsp + 0x110]
1400243dc: movaps xmm8, xmmword ptr [rsp + 0x100]
1400243e5: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400243ee: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400243f7: movaps xmm11, xmmword ptr [rsp + 0xd0]
140024400: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14002440a: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140024414: movaps xmm14, xmmword ptr [rsp + 0xa0]
14002441d: movdqa xmm15, xmmword ptr [rsp + 0x90]
140024427: add rsp, 0x138
14002442e: pop rbx
14002442f: pop rdi
140024430: pop rsi
140024431: pop r12
140024433: pop r13
140024435: pop r14
140024437: pop r15
140024439: pop rbp
14002443a: ret 
14002443b: nop dword ptr [rax + rax]
140024440: mov qword ptr [rsp + 0x10], rdx
140024445: push rbp
140024446: push r15
140024448: push r14
14002444a: push r13
14002444c: push r12
14002444e: push rsi
14002444f: push rdi
140024450: push rbx
140024451: sub rsp, 0x138
140024458: lea rbp, [rdx + 0x80]
14002445f: movdqa xmmword ptr [rsp + 0x90], xmm15
140024469: movaps xmmword ptr [rsp + 0xa0], xmm14
140024472: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002447c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140024486: movaps xmmword ptr [rsp + 0xd0], xmm11
14002448f: movaps xmmword ptr [rsp + 0xe0], xmm10
140024498: movaps xmmword ptr [rsp + 0xf0], xmm9
1400244a1: movaps xmmword ptr [rsp + 0x100], xmm8
1400244aa: movaps xmmword ptr [rsp + 0x110], xmm7
1400244b2: movaps xmmword ptr [rsp + 0x120], xmm6
1400244ba: mov rdx, qword ptr [rbp + 0x7d8]
1400244c1: test rdx, rdx
1400244c4: je 0x1400244dc
1400244c6: shl rdx, 2
1400244ca: mov r8d, 4
1400244d0: mov rcx, qword ptr [rbp + 0x138]
1400244d7: call 0x140613c20 ; ?
1400244dc: movaps xmm6, xmmword ptr [rsp + 0x120]
1400244e4: movaps xmm7, xmmword ptr [rsp + 0x110]
1400244ec: movaps xmm8, xmmword ptr [rsp + 0x100]
1400244f5: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400244fe: movaps xmm10, xmmword ptr [rsp + 0xe0]
140024507: movaps xmm11, xmmword ptr [rsp + 0xd0]
140024510: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14002451a: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140024524: movaps xmm14, xmmword ptr [rsp + 0xa0]
14002452d: movdqa xmm15, xmmword ptr [rsp + 0x90]
140024537: add rsp, 0x138
14002453e: pop rbx
14002453f: pop rdi
140024540: pop rsi
140024541: pop r12
140024543: pop r13
140024545: pop r14
140024547: pop r15
140024549: pop rbp
14002454a: ret 
14002454b: nop dword ptr [rax + rax]
140024550: mov qword ptr [rsp + 0x10], rdx
140024555: push rbp
140024556: push r15
140024558: push r14
14002455a: push r13
14002455c: push r12
14002455e: push rsi
14002455f: push rdi
140024560: push rbx
140024561: sub rsp, 0x138
140024568: lea rbp, [rdx + 0x80]
14002456f: movdqa xmmword ptr [rsp + 0x90], xmm15
140024579: movaps xmmword ptr [rsp + 0xa0], xmm14
140024582: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002458c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140024596: movaps xmmword ptr [rsp + 0xd0], xmm11
14002459f: movaps xmmword ptr [rsp + 0xe0], xmm10
1400245a8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400245b1: movaps xmmword ptr [rsp + 0x100], xmm8
1400245ba: movaps xmmword ptr [rsp + 0x110], xmm7
1400245c2: movaps xmmword ptr [rsp + 0x120], xmm6
1400245ca: movzx ebx, byte ptr [rbp + 0x7c1]
1400245d1: movzx esi, byte ptr [rbp + 0x7c2]
1400245d8: test sil, 1
1400245dc: je 0x14002468a
1400245e2: mov rdx, qword ptr [rbp + 0x3e0]
1400245e9: test rdx, rdx
1400245ec: je 0x140024604
1400245ee: mov rcx, qword ptr [rbp + 0x3e8]
1400245f5: shl rdx, 2
1400245f9: mov r8d, 4
1400245ff: call 0x140613c20 ; ?
140024604: mov byte ptr [rbp + 0x7c5], sil
14002460b: and bl, 1
14002460e: mov byte ptr [rbp + 0x7c4], bl
140024614: mov byte ptr [rbp + 0x7c3], 0
14002461b: movaps xmm6, xmmword ptr [rsp + 0x120]
140024623: movaps xmm7, xmmword ptr [rsp + 0x110]
14002462b: movaps xmm8, xmmword ptr [rsp + 0x100]
140024634: movaps xmm9, xmmword ptr [rsp + 0xf0]
14002463d: movaps xmm10, xmmword ptr [rsp + 0xe0]
140024646: movaps xmm11, xmmword ptr [rsp + 0xd0]
14002464f: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140024659: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140024663: movaps xmm14, xmmword ptr [rsp + 0xa0]
14002466c: movdqa xmm15, xmmword ptr [rsp + 0x90]
140024676: add rsp, 0x138
14002467d: pop rbx
14002467e: pop rdi
14002467f: pop rsi
140024680: pop r12
140024682: pop r13
140024684: pop r14
140024686: pop r15
140024688: pop rbp
140024689: ret 
14002468a: mov byte ptr [rbp + 0x7c5], sil
140024691: and bl, 1
140024694: mov byte ptr [rbp + 0x7c4], bl
14002469a: mov byte ptr [rbp + 0x7c3], 0
1400246a1: movaps xmm6, xmmword ptr [rsp + 0x120]
1400246a9: movaps xmm7, xmmword ptr [rsp + 0x110]
1400246b1: movaps xmm8, xmmword ptr [rsp + 0x100]
1400246ba: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400246c3: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400246cc: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400246d5: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400246df: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400246e9: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400246f2: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400246fc: add rsp, 0x138
140024703: pop rbx
140024704: pop rdi
140024705: pop rsi
140024706: pop r12
140024708: pop r13
14002470a: pop r14
14002470c: pop r15
14002470e: pop rbp
14002470f: ret 
140024710: mov qword ptr [rsp + 0x10], rdx
140024715: push rbp
140024716: push r15
140024718: push r14
14002471a: push r13
14002471c: push r12
14002471e: push rsi
14002471f: push rdi
140024720: push rbx
140024721: sub rsp, 0x138
140024728: lea rbp, [rdx + 0x80]
14002472f: movdqa xmmword ptr [rsp + 0x90], xmm15
140024739: movaps xmmword ptr [rsp + 0xa0], xmm14
140024742: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002474c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140024756: movaps xmmword ptr [rsp + 0xd0], xmm11
14002475f: movaps xmmword ptr [rsp + 0xe0], xmm10
140024768: movaps xmmword ptr [rsp + 0xf0], xmm9
140024771: movaps xmmword ptr [rsp + 0x100], xmm8
14002477a: movaps xmmword ptr [rsp + 0x110], xmm7
140024782: movaps xmmword ptr [rsp + 0x120], xmm6
14002478a: mov rdx, qword ptr [rbp + 0x548]
140024791: test rdx, rdx
140024794: je 0x1400247ac
140024796: shl rdx, 3
14002479a: mov r8d, 4
1400247a0: mov rcx, qword ptr [rbp + 0x2f0]
1400247a7: call 0x140613c20 ; ?
1400247ac: movaps xmm6, xmmword ptr [rsp + 0x120]
1400247b4: movaps xmm7, xmmword ptr [rsp + 0x110]
1400247bc: movaps xmm8, xmmword ptr [rsp + 0x100]
1400247c5: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400247ce: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400247d7: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400247e0: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400247ea: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400247f4: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400247fd: movdqa xmm15, xmmword ptr [rsp + 0x90]
140024807: add rsp, 0x138
14002480e: pop rbx
14002480f: pop rdi
140024810: pop rsi
140024811: pop r12
140024813: pop r13
140024815: pop r14
140024817: pop r15
140024819: pop rbp
14002481a: ret 
14002481b: nop dword ptr [rax + rax]
140024820: mov qword ptr [rsp + 0x10], rdx
140024825: push rbp
140024826: push r15
140024828: push r14
14002482a: push r13
14002482c: push r12
14002482e: push rsi
14002482f: push rdi
140024830: push rbx
140024831: sub rsp, 0x138
140024838: lea rbp, [rdx + 0x80]
14002483f: movdqa xmmword ptr [rsp + 0x90], xmm15
140024849: movaps xmmword ptr [rsp + 0xa0], xmm14
140024852: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002485c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140024866: movaps xmmword ptr [rsp + 0xd0], xmm11
14002486f: movaps xmmword ptr [rsp + 0xe0], xmm10
140024878: movaps xmmword ptr [rsp + 0xf0], xmm9
140024881: movaps xmmword ptr [rsp + 0x100], xmm8
14002488a: movaps xmmword ptr [rsp + 0x110], xmm7
140024892: movaps xmmword ptr [rsp + 0x120], xmm6
14002489a: lea rcx, [rbp + 0x590]
1400248a1: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
1400248a6: mov byte ptr [rbp + 0x816], 1
1400248ad: mov byte ptr [rbp + 0x815], 1
1400248b4: movaps xmm6, xmmword ptr [rsp + 0x120]
1400248bc: movaps xmm7, xmmword ptr [rsp + 0x110]
1400248c4: movaps xmm8, xmmword ptr [rsp + 0x100]
1400248cd: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400248d6: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400248df: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400248e8: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400248f2: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400248fc: movaps xmm14, xmmword ptr [rsp + 0xa0]
140024905: movdqa xmm15, xmmword ptr [rsp + 0x90]
14002490f: add rsp, 0x138
140024916: pop rbx
140024917: pop rdi
140024918: pop rsi
140024919: pop r12
14002491b: pop r13
14002491d: pop r14
14002491f: pop r15
140024921: pop rbp
140024922: ret 
140024923: nop word ptr cs:[rax + rax]
140024930: mov qword ptr [rsp + 0x10], rdx
140024935: push rbp
140024936: push r15
140024938: push r14
14002493a: push r13
14002493c: push r12
14002493e: push rsi
14002493f: push rdi
140024940: push rbx
140024941: sub rsp, 0x138
140024948: lea rbp, [rdx + 0x80]
14002494f: movdqa xmmword ptr [rsp + 0x90], xmm15
140024959: movaps xmmword ptr [rsp + 0xa0], xmm14
140024962: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002496c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140024976: movaps xmmword ptr [rsp + 0xd0], xmm11
14002497f: movaps xmmword ptr [rsp + 0xe0], xmm10
140024988: movaps xmmword ptr [rsp + 0xf0], xmm9
140024991: movaps xmmword ptr [rsp + 0x100], xmm8
14002499a: movaps xmmword ptr [rsp + 0x110], xmm7
1400249a2: movaps xmmword ptr [rsp + 0x120], xmm6
1400249aa: movzx ebx, byte ptr [rbp + 0x7c3]
1400249b1: movzx esi, byte ptr [rbp + 0x7c4]
1400249b8: movzx edi, byte ptr [rbp + 0x7c5]
1400249bf: test dil, 1
1400249c3: je 0x140024a7c
1400249c9: mov rdx, qword ptr [rbp + 0x290]
1400249d0: test rdx, rdx
1400249d3: je 0x1400249eb
1400249d5: mov rcx, qword ptr [rbp + 0x298]
1400249dc: shl rdx, 2
1400249e0: mov r8d, 4
1400249e6: call 0x140613c20 ; ?
1400249eb: mov byte ptr [rbp + 0x7c9], dil
1400249f2: and sil, 1
1400249f6: mov byte ptr [rbp + 0x7c8], sil
1400249fd: and bl, 1
140024a00: mov byte ptr [rbp + 0x7c7], bl
140024a06: mov byte ptr [rbp + 0x7c6], 0
140024a0d: movaps xmm6, xmmword ptr [rsp + 0x120]
140024a15: movaps xmm7, xmmword ptr [rsp + 0x110]
140024a1d: movaps xmm8, xmmword ptr [rsp + 0x100]
140024a26: movaps xmm9, xmmword ptr [rsp + 0xf0]
140024a2f: movaps xmm10, xmmword ptr [rsp + 0xe0]
140024a38: movaps xmm11, xmmword ptr [rsp + 0xd0]
140024a41: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140024a4b: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140024a55: movaps xmm14, xmmword ptr [rsp + 0xa0]
140024a5e: movdqa xmm15, xmmword ptr [rsp + 0x90]
140024a68: add rsp, 0x138
140024a6f: pop rbx
140024a70: pop rdi
140024a71: pop rsi
140024a72: pop r12
140024a74: pop r13
140024a76: pop r14
140024a78: pop r15
140024a7a: pop rbp
140024a7b: ret 
140024a7c: mov byte ptr [rbp + 0x7c9], dil
140024a83: and sil, 1
140024a87: mov byte ptr [rbp + 0x7c8], sil
140024a8e: and bl, 1
140024a91: mov byte ptr [rbp + 0x7c7], bl
140024a97: mov byte ptr [rbp + 0x7c6], 0
140024a9e: movaps xmm6, xmmword ptr [rsp + 0x120]
140024aa6: movaps xmm7, xmmword ptr [rsp + 0x110]
140024aae: movaps xmm8, xmmword ptr [rsp + 0x100]
140024ab7: movaps xmm9, xmmword ptr [rsp + 0xf0]
140024ac0: movaps xmm10, xmmword ptr [rsp + 0xe0]
140024ac9: movaps xmm11, xmmword ptr [rsp + 0xd0]
140024ad2: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140024adc: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140024ae6: movaps xmm14, xmmword ptr [rsp + 0xa0]
140024aef: movdqa xmm15, xmmword ptr [rsp + 0x90]
140024af9: add rsp, 0x138
140024b00: pop rbx
140024b01: pop rdi
140024b02: pop rsi
140024b03: pop r12
140024b05: pop r13
140024b07: pop r14
140024b09: pop r15
140024b0b: pop rbp
140024b0c: ret 
140024b0d: nop dword ptr [rax]
140024b10: mov qword ptr [rsp + 0x10], rdx
140024b15: push rbp
140024b16: push r15
140024b18: push r14
140024b1a: push r13
140024b1c: push r12
140024b1e: push rsi
140024b1f: push rdi
140024b20: push rbx
140024b21: sub rsp, 0x138
140024b28: lea rbp, [rdx + 0x80]
140024b2f: movdqa xmmword ptr [rsp + 0x90], xmm15
140024b39: movaps xmmword ptr [rsp + 0xa0], xmm14
140024b42: movdqa xmmword ptr [rsp + 0xb0], xmm13
140024b4c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140024b56: movaps xmmword ptr [rsp + 0xd0], xmm11
140024b5f: movaps xmmword ptr [rsp + 0xe0], xmm10
140024b68: movaps xmmword ptr [rsp + 0xf0], xmm9
140024b71: movaps xmmword ptr [rsp + 0x100], xmm8
140024b7a: movaps xmmword ptr [rsp + 0x110], xmm7
140024b82: movaps xmmword ptr [rsp + 0x120], xmm6
140024b8a: lea rcx, [rbp + 0x3e0]
140024b91: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140024b96: mov byte ptr [rbp + 0x813], 1
140024b9d: movaps xmm6, xmmword ptr [rsp + 0x120]
140024ba5: movaps xmm7, xmmword ptr [rsp + 0x110]
140024bad: movaps xmm8, xmmword ptr [rsp + 0x100]
140024bb6: movaps xmm9, xmmword ptr [rsp + 0xf0]
140024bbf: movaps xmm10, xmmword ptr [rsp + 0xe0]
140024bc8: movaps xmm11, xmmword ptr [rsp + 0xd0]
140024bd1: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140024bdb: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140024be5: movaps xmm14, xmmword ptr [rsp + 0xa0]
140024bee: movdqa xmm15, xmmword ptr [rsp + 0x90]
140024bf8: add rsp, 0x138
140024bff: pop rbx
140024c00: pop rdi
140024c01: pop rsi
140024c02: pop r12
140024c04: pop r13
140024c06: pop r14
140024c08: pop r15
140024c0a: pop rbp
140024c0b: ret 
140024c0c: nop dword ptr [rax]
140024c10: mov qword ptr [rsp + 0x10], rdx
140024c15: push rbp
140024c16: push r15
140024c18: push r14
140024c1a: push r13
140024c1c: push r12
140024c1e: push rsi
140024c1f: push rdi
140024c20: push rbx
140024c21: sub rsp, 0x138
140024c28: lea rbp, [rdx + 0x80]
140024c2f: movdqa xmmword ptr [rsp + 0x90], xmm15
140024c39: movaps xmmword ptr [rsp + 0xa0], xmm14
140024c42: movdqa xmmword ptr [rsp + 0xb0], xmm13
140024c4c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140024c56: movaps xmmword ptr [rsp + 0xd0], xmm11
140024c5f: movaps xmmword ptr [rsp + 0xe0], xmm10
140024c68: movaps xmmword ptr [rsp + 0xf0], xmm9
140024c71: movaps xmmword ptr [rsp + 0x100], xmm8
140024c7a: movaps xmmword ptr [rsp + 0x110], xmm7
140024c82: movaps xmmword ptr [rsp + 0x120], xmm6
140024c8a: mov rdx, qword ptr [rbp + 0x588]
140024c91: test rdx, rdx
140024c94: je 0x140024cac
140024c96: shl rdx, 3
140024c9a: mov r8d, 4
140024ca0: mov rcx, qword ptr [rbp + 0x2e8]
140024ca7: call 0x140613c20 ; ?
140024cac: movaps xmm6, xmmword ptr [rsp + 0x120]
140024cb4: movaps xmm7, xmmword ptr [rsp + 0x110]
140024cbc: movaps xmm8, xmmword ptr [rsp + 0x100]
140024cc5: movaps xmm9, xmmword ptr [rsp + 0xf0]
140024cce: movaps xmm10, xmmword ptr [rsp + 0xe0]
140024cd7: movaps xmm11, xmmword ptr [rsp + 0xd0]
140024ce0: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140024cea: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140024cf4: movaps xmm14, xmmword ptr [rsp + 0xa0]
140024cfd: movdqa xmm15, xmmword ptr [rsp + 0x90]
140024d07: add rsp, 0x138
140024d0e: pop rbx
140024d0f: pop rdi
140024d10: pop rsi
140024d11: pop r12
140024d13: pop r13
140024d15: pop r14
140024d17: pop r15
140024d19: pop rbp
140024d1a: ret 
140024d1b: nop dword ptr [rax + rax]
140024d20: mov qword ptr [rsp + 0x10], rdx
140024d25: push rbp
140024d26: push r15
140024d28: push r14
140024d2a: push r13
140024d2c: push r12
140024d2e: push rsi
140024d2f: push rdi
140024d30: push rbx
140024d31: sub rsp, 0x138
140024d38: lea rbp, [rdx + 0x80]
140024d3f: movdqa xmmword ptr [rsp + 0x90], xmm15
140024d49: movaps xmmword ptr [rsp + 0xa0], xmm14
140024d52: movdqa xmmword ptr [rsp + 0xb0], xmm13
140024d5c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140024d66: movaps xmmword ptr [rsp + 0xd0], xmm11
140024d6f: movaps xmmword ptr [rsp + 0xe0], xmm10
140024d78: movaps xmmword ptr [rsp + 0xf0], xmm9
140024d81: movaps xmmword ptr [rsp + 0x100], xmm8
140024d8a: movaps xmmword ptr [rsp + 0x110], xmm7
140024d92: movaps xmmword ptr [rsp + 0x120], xmm6
140024d9a: movzx ebx, byte ptr [rbp + 0x7c6]
140024da1: movzx esi, byte ptr [rbp + 0x7c7]
140024da8: movzx edi, byte ptr [rbp + 0x7c8]
140024daf: test byte ptr [rbp + 0x7c9], 1
140024db6: je 0x140024e73
140024dbc: mov rdx, qword ptr [rbp + 0x680]
140024dc3: test rdx, rdx
140024dc6: je 0x140024dde
140024dc8: mov rcx, qword ptr [rbp + 0x688]
140024dcf: shl rdx, 3
140024dd3: mov r8d, 4
140024dd9: call 0x140613c20 ; ?
140024dde: and dil, 1
140024de2: mov byte ptr [rbp + 0x7cd], dil
140024de9: and sil, 1
140024ded: mov byte ptr [rbp + 0x7cc], sil
140024df4: and bl, 1
140024df7: mov byte ptr [rbp + 0x7cb], bl
140024dfd: mov byte ptr [rbp + 0x7ca], 0
140024e04: movaps xmm6, xmmword ptr [rsp + 0x120]
140024e0c: movaps xmm7, xmmword ptr [rsp + 0x110]
140024e14: movaps xmm8, xmmword ptr [rsp + 0x100]
140024e1d: movaps xmm9, xmmword ptr [rsp + 0xf0]
140024e26: movaps xmm10, xmmword ptr [rsp + 0xe0]
140024e2f: movaps xmm11, xmmword ptr [rsp + 0xd0]
140024e38: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140024e42: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140024e4c: movaps xmm14, xmmword ptr [rsp + 0xa0]
140024e55: movdqa xmm15, xmmword ptr [rsp + 0x90]
140024e5f: add rsp, 0x138
140024e66: pop rbx
140024e67: pop rdi
140024e68: pop rsi
140024e69: pop r12
140024e6b: pop r13
140024e6d: pop r14
140024e6f: pop r15
140024e71: pop rbp
140024e72: ret 
140024e73: and dil, 1
140024e77: mov byte ptr [rbp + 0x7cd], dil
140024e7e: and sil, 1
140024e82: mov byte ptr [rbp + 0x7cc], sil
140024e89: and bl, 1
140024e8c: mov byte ptr [rbp + 0x7cb], bl
140024e92: mov byte ptr [rbp + 0x7ca], 0
140024e99: movaps xmm6, xmmword ptr [rsp + 0x120]
140024ea1: movaps xmm7, xmmword ptr [rsp + 0x110]
140024ea9: movaps xmm8, xmmword ptr [rsp + 0x100]
140024eb2: movaps xmm9, xmmword ptr [rsp + 0xf0]
140024ebb: movaps xmm10, xmmword ptr [rsp + 0xe0]
140024ec4: movaps xmm11, xmmword ptr [rsp + 0xd0]
140024ecd: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140024ed7: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140024ee1: movaps xmm14, xmmword ptr [rsp + 0xa0]
140024eea: movdqa xmm15, xmmword ptr [rsp + 0x90]
140024ef4: add rsp, 0x138
140024efb: pop rbx
140024efc: pop rdi
140024efd: pop rsi
140024efe: pop r12
140024f00: pop r13
140024f02: pop r14
140024f04: pop r15
140024f06: pop rbp
140024f07: ret 
140024f08: nop dword ptr [rax + rax]
140024f10: mov qword ptr [rsp + 0x10], rdx
140024f15: push rbp
140024f16: push r15
140024f18: push r14
140024f1a: push r13
140024f1c: push r12
140024f1e: push rsi
140024f1f: push rdi
140024f20: push rbx
140024f21: sub rsp, 0x138
140024f28: lea rbp, [rdx + 0x80]
140024f2f: movdqa xmmword ptr [rsp + 0x90], xmm15
140024f39: movaps xmmword ptr [rsp + 0xa0], xmm14
140024f42: movdqa xmmword ptr [rsp + 0xb0], xmm13
140024f4c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140024f56: movaps xmmword ptr [rsp + 0xd0], xmm11
140024f5f: movaps xmmword ptr [rsp + 0xe0], xmm10
140024f68: movaps xmmword ptr [rsp + 0xf0], xmm9
140024f71: movaps xmmword ptr [rsp + 0x100], xmm8
140024f7a: movaps xmmword ptr [rsp + 0x110], xmm7
140024f82: movaps xmmword ptr [rsp + 0x120], xmm6
140024f8a: movzx ebx, byte ptr [rbp + 0x7ca]
140024f91: movzx esi, byte ptr [rbp + 0x7cb]
140024f98: movzx edi, byte ptr [rbp + 0x7cc]
140024f9f: cmp byte ptr [rbp + 0x7cd], 0
140024fa6: je 0x140024fce
140024fa8: mov rax, qword ptr [rbp + 0x650]
140024faf: test rax, rax
140024fb2: je 0x140024fce
140024fb4: mov rcx, qword ptr [rbp + 0x658]
140024fbb: shl rax, 2
140024fbf: lea rdx, [rax + rax*2]
140024fc3: mov r8d, 4
140024fc9: call 0x140613c20 ; ?
140024fce: mov rdx, qword ptr [rbp + 0x648]
140024fd5: test rdx, rdx
140024fd8: je 0x140024ff0
140024fda: shl rdx, 2
140024fde: mov r8d, 4
140024fe4: mov rcx, qword ptr [rbp + 0x6a8]
140024feb: call 0x140613c20 ; ?
140024ff0: mov rdx, qword ptr [rbp + 0x560]
140024ff7: test rdx, rdx
140024ffa: setne al
140024ffd: test bl, al
140024fff: je 0x140025017
140025001: shl rdx, 3
140025005: mov r8d, 4
14002500b: mov rcx, qword ptr [rbp + 0x6a0]
140025012: call 0x140613c20 ; ?
140025017: mov rdx, qword ptr [rbp + 0x7e0]
14002501e: test rdx, rdx
140025021: setne al
140025024: test sil, al
140025027: je 0x14002503f
140025029: shl rdx, 2
14002502d: mov r8d, 4
140025033: mov rcx, qword ptr [rbp + 0x780]
14002503a: call 0x140613c20 ; ?
14002503f: test dil, 1
140025043: je 0x140025103
140025049: mov rdx, qword ptr [rbp + 0x538]
140025050: test rdx, rdx
140025053: je 0x14002506b
140025055: shl rdx, 2
140025059: mov r8d, 4
14002505f: mov rcx, qword ptr [rbp + 0x580]
140025066: call 0x140613c20 ; ?
14002506b: mov byte ptr [rbp + 0x803], 1
140025072: and sil, 1
140025076: mov byte ptr [rbp + 0x802], sil
14002507d: and bl, 1
140025080: mov byte ptr [rbp + 0x801], bl
140025086: mov byte ptr [rbp + 0x800], 0
14002508d: mov byte ptr [rbp + 0x7ff], 1
140025094: movaps xmm6, xmmword ptr [rsp + 0x120]
14002509c: movaps xmm7, xmmword ptr [rsp + 0x110]
1400250a4: movaps xmm8, xmmword ptr [rsp + 0x100]
1400250ad: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400250b6: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400250bf: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400250c8: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400250d2: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400250dc: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400250e5: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400250ef: add rsp, 0x138
1400250f6: pop rbx
1400250f7: pop rdi
1400250f8: pop rsi
1400250f9: pop r12
1400250fb: pop r13
1400250fd: pop r14
1400250ff: pop r15
140025101: pop rbp
140025102: ret 
140025103: mov byte ptr [rbp + 0x803], 0
14002510a: and sil, 1
14002510e: mov byte ptr [rbp + 0x802], sil
140025115: and bl, 1
140025118: mov byte ptr [rbp + 0x801], bl
14002511e: mov byte ptr [rbp + 0x800], 0
140025125: mov byte ptr [rbp + 0x7ff], 1
14002512c: movaps xmm6, xmmword ptr [rsp + 0x120]
140025134: movaps xmm7, xmmword ptr [rsp + 0x110]
14002513c: movaps xmm8, xmmword ptr [rsp + 0x100]
140025145: movaps xmm9, xmmword ptr [rsp + 0xf0]
14002514e: movaps xmm10, xmmword ptr [rsp + 0xe0]
140025157: movaps xmm11, xmmword ptr [rsp + 0xd0]
140025160: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14002516a: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140025174: movaps xmm14, xmmword ptr [rsp + 0xa0]
14002517d: movdqa xmm15, xmmword ptr [rsp + 0x90]
140025187: add rsp, 0x138
14002518e: pop rbx
14002518f: pop rdi
140025190: pop rsi
140025191: pop r12
140025193: pop r13
140025195: pop r14
140025197: pop r15
140025199: pop rbp
14002519a: ret 
14002519b: nop dword ptr [rax + rax]
1400251a0: mov qword ptr [rsp + 0x10], rdx
1400251a5: push rbp
1400251a6: push r15
1400251a8: push r14
1400251aa: push r13
1400251ac: push r12
1400251ae: push rsi
1400251af: push rdi
1400251b0: push rbx
1400251b1: sub rsp, 0x138
1400251b8: lea rbp, [rdx + 0x80]
1400251bf: movdqa xmmword ptr [rsp + 0x90], xmm15
1400251c9: movaps xmmword ptr [rsp + 0xa0], xmm14
1400251d2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400251dc: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400251e6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400251ef: movaps xmmword ptr [rsp + 0xe0], xmm10
1400251f8: movaps xmmword ptr [rsp + 0xf0], xmm9
140025201: movaps xmmword ptr [rsp + 0x100], xmm8
14002520a: movaps xmmword ptr [rsp + 0x110], xmm7
140025212: movaps xmmword ptr [rsp + 0x120], xmm6
14002521a: lea rcx, [rbp + 0x380]
140025221: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140025226: movaps xmm6, xmmword ptr [rsp + 0x120]
14002522e: movaps xmm7, xmmword ptr [rsp + 0x110]
140025236: movaps xmm8, xmmword ptr [rsp + 0x100]
14002523f: movaps xmm9, xmmword ptr [rsp + 0xf0]
140025248: movaps xmm10, xmmword ptr [rsp + 0xe0]
140025251: movaps xmm11, xmmword ptr [rsp + 0xd0]
14002525a: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140025264: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14002526e: movaps xmm14, xmmword ptr [rsp + 0xa0]
140025277: movdqa xmm15, xmmword ptr [rsp + 0x90]
140025281: add rsp, 0x138
140025288: pop rbx
140025289: pop rdi
14002528a: pop rsi
14002528b: pop r12
14002528d: pop r13
14002528f: pop r14
140025291: pop r15
140025293: pop rbp
140025294: ret 
140025295: nop word ptr cs:[rax + rax]
1400252a0: mov qword ptr [rsp + 0x10], rdx
1400252a5: push rbp
1400252a6: push r15
1400252a8: push r14
1400252aa: push r13
1400252ac: push r12
1400252ae: push rsi
1400252af: push rdi
1400252b0: push rbx
1400252b1: sub rsp, 0x138
1400252b8: lea rbp, [rdx + 0x80]
1400252bf: movdqa xmmword ptr [rsp + 0x90], xmm15
1400252c9: movaps xmmword ptr [rsp + 0xa0], xmm14
1400252d2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400252dc: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400252e6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400252ef: movaps xmmword ptr [rsp + 0xe0], xmm10
1400252f8: movaps xmmword ptr [rsp + 0xf0], xmm9
140025301: movaps xmmword ptr [rsp + 0x100], xmm8
14002530a: movaps xmmword ptr [rsp + 0x110], xmm7
140025312: movaps xmmword ptr [rsp + 0x120], xmm6
14002531a: mov rdx, qword ptr [rbp + 0x590]
140025321: test rdx, rdx
140025324: je 0x14002533c
140025326: mov rcx, qword ptr [rbp + 0x598]
14002532d: shl rdx, 2
140025331: mov r8d, 4
140025337: call 0x140613c20 ; ?
14002533c: movaps xmm6, xmmword ptr [rsp + 0x120]
140025344: movaps xmm7, xmmword ptr [rsp + 0x110]
14002534c: movaps xmm8, xmmword ptr [rsp + 0x100]
140025355: movaps xmm9, xmmword ptr [rsp + 0xf0]
14002535e: movaps xmm10, xmmword ptr [rsp + 0xe0]
140025367: movaps xmm11, xmmword ptr [rsp + 0xd0]
140025370: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14002537a: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140025384: movaps xmm14, xmmword ptr [rsp + 0xa0]
14002538d: movdqa xmm15, xmmword ptr [rsp + 0x90]
140025397: add rsp, 0x138
14002539e: pop rbx
14002539f: pop rdi
1400253a0: pop rsi
1400253a1: pop r12
1400253a3: pop r13
1400253a5: pop r14
1400253a7: pop r15
1400253a9: pop rbp
1400253aa: ret 
1400253ab: nop dword ptr [rax + rax]
1400253b0: mov qword ptr [rsp + 0x10], rdx
1400253b5: push rbp
1400253b6: push r15
1400253b8: push r14
1400253ba: push r13
1400253bc: push r12
1400253be: push rsi
1400253bf: push rdi
1400253c0: push rbx
1400253c1: sub rsp, 0x138
1400253c8: lea rbp, [rdx + 0x80]
1400253cf: movdqa xmmword ptr [rsp + 0x90], xmm15
1400253d9: movaps xmmword ptr [rsp + 0xa0], xmm14
1400253e2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400253ec: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400253f6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400253ff: movaps xmmword ptr [rsp + 0xe0], xmm10
140025408: movaps xmmword ptr [rsp + 0xf0], xmm9
140025411: movaps xmmword ptr [rsp + 0x100], xmm8
14002541a: movaps xmmword ptr [rsp + 0x110], xmm7
140025422: movaps xmmword ptr [rsp + 0x120], xmm6
14002542a: mov rax, qword ptr [rbp + 0x508]
140025431: test rax, rax
140025434: je 0x140025450
140025436: mov rcx, qword ptr [rbp + 0x510]
14002543d: shl rax, 2
140025441: lea rdx, [rax + rax*2]
140025445: mov r8d, 4
14002544b: call 0x140613c20 ; ?
140025450: movaps xmm6, xmmword ptr [rsp + 0x120]
140025458: movaps xmm7, xmmword ptr [rsp + 0x110]
140025460: movaps xmm8, xmmword ptr [rsp + 0x100]
140025469: movaps xmm9, xmmword ptr [rsp + 0xf0]
140025472: movaps xmm10, xmmword ptr [rsp + 0xe0]
14002547b: movaps xmm11, xmmword ptr [rsp + 0xd0]
140025484: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14002548e: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140025498: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400254a1: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400254ab: add rsp, 0x138
1400254b2: pop rbx
1400254b3: pop rdi
1400254b4: pop rsi
1400254b5: pop r12
1400254b7: pop r13
1400254b9: pop r14
1400254bb: pop r15
1400254bd: pop rbp
1400254be: ret 
1400254bf: nop 
1400254c0: mov qword ptr [rsp + 0x10], rdx
1400254c5: push rbp
1400254c6: push r15
1400254c8: push r14
1400254ca: push r13
1400254cc: push r12
1400254ce: push rsi
1400254cf: push rdi
1400254d0: push rbx
1400254d1: sub rsp, 0x138
1400254d8: lea rbp, [rdx + 0x80]
1400254df: movdqa xmmword ptr [rsp + 0x90], xmm15
1400254e9: movaps xmmword ptr [rsp + 0xa0], xmm14
1400254f2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400254fc: movdqa xmmword ptr [rsp + 0xc0], xmm12
140025506: movaps xmmword ptr [rsp + 0xd0], xmm11
14002550f: movaps xmmword ptr [rsp + 0xe0], xmm10
140025518: movaps xmmword ptr [rsp + 0xf0], xmm9
140025521: movaps xmmword ptr [rsp + 0x100], xmm8
14002552a: movaps xmmword ptr [rsp + 0x110], xmm7
140025532: movaps xmmword ptr [rsp + 0x120], xmm6
14002553a: mov rdx, qword ptr [rbp + 0x680]
140025541: test rdx, rdx
140025544: je 0x14002555c
140025546: mov rcx, qword ptr [rbp + 0x688]
14002554d: shl rdx, 3
140025551: mov r8d, 4
140025557: call 0x140613c20 ; ?
14002555c: movaps xmm6, xmmword ptr [rsp + 0x120]
140025564: movaps xmm7, xmmword ptr [rsp + 0x110]
14002556c: movaps xmm8, xmmword ptr [rsp + 0x100]
140025575: movaps xmm9, xmmword ptr [rsp + 0xf0]
14002557e: movaps xmm10, xmmword ptr [rsp + 0xe0]
140025587: movaps xmm11, xmmword ptr [rsp + 0xd0]
140025590: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14002559a: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400255a4: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400255ad: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400255b7: add rsp, 0x138
1400255be: pop rbx
1400255bf: pop rdi
1400255c0: pop rsi
1400255c1: pop r12
1400255c3: pop r13
1400255c5: pop r14
1400255c7: pop r15
1400255c9: pop rbp
1400255ca: ret 
1400255cb: nop dword ptr [rax + rax]
1400255d0: mov qword ptr [rsp + 0x10], rdx
1400255d5: push rbp
1400255d6: push r15
1400255d8: push r14
1400255da: push r13
1400255dc: push r12
1400255de: push rsi
1400255df: push rdi
1400255e0: push rbx
1400255e1: sub rsp, 0x138
1400255e8: lea rbp, [rdx + 0x80]
1400255ef: movdqa xmmword ptr [rsp + 0x90], xmm15
1400255f9: movaps xmmword ptr [rsp + 0xa0], xmm14
140025602: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002560c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140025616: movaps xmmword ptr [rsp + 0xd0], xmm11
14002561f: movaps xmmword ptr [rsp + 0xe0], xmm10
140025628: movaps xmmword ptr [rsp + 0xf0], xmm9
140025631: movaps xmmword ptr [rsp + 0x100], xmm8
14002563a: movaps xmmword ptr [rsp + 0x110], xmm7
140025642: movaps xmmword ptr [rsp + 0x120], xmm6
14002564a: mov rdx, qword ptr [rbp + 0x650]
140025651: test rdx, rdx
140025654: je 0x14002566c
140025656: mov rcx, qword ptr [rbp + 0x658]
14002565d: shl rdx, 2
140025661: mov r8d, 4
140025667: call 0x140613c20 ; ?
14002566c: movaps xmm6, xmmword ptr [rsp + 0x120]
140025674: movaps xmm7, xmmword ptr [rsp + 0x110]
14002567c: movaps xmm8, xmmword ptr [rsp + 0x100]
140025685: movaps xmm9, xmmword ptr [rsp + 0xf0]
14002568e: movaps xmm10, xmmword ptr [rsp + 0xe0]
140025697: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400256a0: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400256aa: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400256b4: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400256bd: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400256c7: add rsp, 0x138
1400256ce: pop rbx
1400256cf: pop rdi
1400256d0: pop rsi
1400256d1: pop r12
1400256d3: pop r13
1400256d5: pop r14
1400256d7: pop r15
1400256d9: pop rbp
1400256da: ret 
1400256db: nop dword ptr [rax + rax]
1400256e0: mov qword ptr [rsp + 0x10], rdx
1400256e5: push rbp
1400256e6: push r15
1400256e8: push r14
1400256ea: push r13
1400256ec: push r12
1400256ee: push rsi
1400256ef: push rdi
1400256f0: push rbx
1400256f1: sub rsp, 0x138
1400256f8: lea rbp, [rdx + 0x80]
1400256ff: movdqa xmmword ptr [rsp + 0x90], xmm15
140025709: movaps xmmword ptr [rsp + 0xa0], xmm14
140025712: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002571c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140025726: movaps xmmword ptr [rsp + 0xd0], xmm11
14002572f: movaps xmmword ptr [rsp + 0xe0], xmm10
140025738: movaps xmmword ptr [rsp + 0xf0], xmm9
140025741: movaps xmmword ptr [rsp + 0x100], xmm8
14002574a: movaps xmmword ptr [rsp + 0x110], xmm7
140025752: movaps xmmword ptr [rsp + 0x120], xmm6
14002575a: lea rcx, [rbp + 0x460]
140025761: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140025766: movaps xmm6, xmmword ptr [rsp + 0x120]
14002576e: movaps xmm7, xmmword ptr [rsp + 0x110]
140025776: movaps xmm8, xmmword ptr [rsp + 0x100]
14002577f: movaps xmm9, xmmword ptr [rsp + 0xf0]
140025788: movaps xmm10, xmmword ptr [rsp + 0xe0]
140025791: movaps xmm11, xmmword ptr [rsp + 0xd0]
14002579a: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400257a4: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400257ae: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400257b7: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400257c1: add rsp, 0x138
1400257c8: pop rbx
1400257c9: pop rdi
1400257ca: pop rsi
1400257cb: pop r12
1400257cd: pop r13
1400257cf: pop r14
1400257d1: pop r15
1400257d3: pop rbp
1400257d4: ret 
1400257d5: nop word ptr cs:[rax + rax]
1400257e0: mov qword ptr [rsp + 0x10], rdx
1400257e5: push rbp
1400257e6: push r15
1400257e8: push r14
1400257ea: push r13
1400257ec: push r12
1400257ee: push rsi
1400257ef: push rdi
1400257f0: push rbx
1400257f1: sub rsp, 0x138
1400257f8: lea rbp, [rdx + 0x80]
1400257ff: movdqa xmmword ptr [rsp + 0x90], xmm15
140025809: movaps xmmword ptr [rsp + 0xa0], xmm14
140025812: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002581c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140025826: movaps xmmword ptr [rsp + 0xd0], xmm11
14002582f: movaps xmmword ptr [rsp + 0xe0], xmm10
140025838: movaps xmmword ptr [rsp + 0xf0], xmm9
140025841: movaps xmmword ptr [rsp + 0x100], xmm8
14002584a: movaps xmmword ptr [rsp + 0x110], xmm7
140025852: movaps xmmword ptr [rsp + 0x120], xmm6
14002585a: mov byte ptr [rbp + 0x817], 1
140025861: lea rcx, [rbp + 0x380]
140025868: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
14002586d: mov byte ptr [rbp + 0x817], 1
140025874: movaps xmm6, xmmword ptr [rsp + 0x120]
14002587c: movaps xmm7, xmmword ptr [rsp + 0x110]
140025884: movaps xmm8, xmmword ptr [rsp + 0x100]
14002588d: movaps xmm9, xmmword ptr [rsp + 0xf0]
140025896: movaps xmm10, xmmword ptr [rsp + 0xe0]
14002589f: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400258a8: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400258b2: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400258bc: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400258c5: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400258cf: add rsp, 0x138
1400258d6: pop rbx
1400258d7: pop rdi
1400258d8: pop rsi
1400258d9: pop r12
1400258db: pop r13
1400258dd: pop r14
1400258df: pop r15
1400258e1: pop rbp
1400258e2: ret 
1400258e3: nop word ptr cs:[rax + rax]
1400258f0: mov qword ptr [rsp + 0x10], rdx
1400258f5: push rbp
1400258f6: push r15
1400258f8: push r14
1400258fa: push r13
1400258fc: push r12
1400258fe: push rsi
1400258ff: push rdi
140025900: push rbx
140025901: sub rsp, 0x138
140025908: lea rbp, [rdx + 0x80]
14002590f: movdqa xmmword ptr [rsp + 0x90], xmm15
140025919: movaps xmmword ptr [rsp + 0xa0], xmm14
140025922: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002592c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140025936: movaps xmmword ptr [rsp + 0xd0], xmm11
14002593f: movaps xmmword ptr [rsp + 0xe0], xmm10
140025948: movaps xmmword ptr [rsp + 0xf0], xmm9
140025951: movaps xmmword ptr [rsp + 0x100], xmm8
14002595a: movaps xmmword ptr [rsp + 0x110], xmm7
140025962: movaps xmmword ptr [rsp + 0x120], xmm6
14002596a: movzx eax, byte ptr [rbp + 0x815]
140025971: movzx ecx, byte ptr [rbp + 0x816]
140025978: mov byte ptr [rbp + 0x808], al
14002597e: test al, al
140025980: mov byte ptr [rbp + 0x7a0], cl
140025986: je 0x140025a1a
14002598c: mov rdx, qword ptr [rbp + 0x7d0]
140025993: test rdx, rdx
140025996: je 0x1400259ae
140025998: shl rdx, 2
14002599c: mov r8d, 4
1400259a2: mov rcx, qword ptr [rbp + 0x7f0]
1400259a9: call 0x140613c20 ; ?
1400259ae: movaps xmm6, xmmword ptr [rsp + 0x120]
1400259b6: movaps xmm7, xmmword ptr [rsp + 0x110]
1400259be: movaps xmm8, xmmword ptr [rsp + 0x100]
1400259c7: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400259d0: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400259d9: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400259e2: movaps xmm12, xmmword ptr [rsp + 0xc0]
1400259eb: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400259f4: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400259fd: movaps xmm15, xmmword ptr [rsp + 0x90]
140025a06: add rsp, 0x138
140025a0d: pop rbx
140025a0e: pop rdi
140025a0f: pop rsi
140025a10: pop r12
140025a12: pop r13
140025a14: pop r14
140025a16: pop r15
140025a18: pop rbp
140025a19: ret 
140025a1a: movaps xmm6, xmmword ptr [rsp + 0x120]
140025a22: movaps xmm7, xmmword ptr [rsp + 0x110]
140025a2a: movaps xmm8, xmmword ptr [rsp + 0x100]
140025a33: movaps xmm9, xmmword ptr [rsp + 0xf0]
140025a3c: movaps xmm10, xmmword ptr [rsp + 0xe0]
140025a45: movaps xmm11, xmmword ptr [rsp + 0xd0]
140025a4e: movaps xmm12, xmmword ptr [rsp + 0xc0]
140025a57: movaps xmm13, xmmword ptr [rsp + 0xb0]
140025a60: movaps xmm14, xmmword ptr [rsp + 0xa0]
140025a69: movaps xmm15, xmmword ptr [rsp + 0x90]
140025a72: add rsp, 0x138
140025a79: pop rbx
140025a7a: pop rdi
140025a7b: pop rsi
140025a7c: pop r12
140025a7e: pop r13
140025a80: pop r14
140025a82: pop r15
140025a84: pop rbp
140025a85: ret 
140025a86: nop word ptr cs:[rax + rax]
140025a90: mov qword ptr [rsp + 0x10], rdx
140025a95: push rbp
140025a96: push r15
140025a98: push r14
140025a9a: push r13
140025a9c: push r12
140025a9e: push rsi
140025a9f: push rdi
140025aa0: push rbx
140025aa1: sub rsp, 0x138
140025aa8: lea rbp, [rdx + 0x80]
140025aaf: movaps xmmword ptr [rsp + 0x90], xmm15
140025ab8: movaps xmmword ptr [rsp + 0xa0], xmm14
140025ac1: movaps xmmword ptr [rsp + 0xb0], xmm13
140025aca: movaps xmmword ptr [rsp + 0xc0], xmm12
140025ad3: movaps xmmword ptr [rsp + 0xd0], xmm11
140025adc: movaps xmmword ptr [rsp + 0xe0], xmm10
140025ae5: movaps xmmword ptr [rsp + 0xf0], xmm9
140025aee: movaps xmmword ptr [rsp + 0x100], xmm8
140025af7: movaps xmmword ptr [rsp + 0x110], xmm7
140025aff: movaps xmmword ptr [rsp + 0x120], xmm6
140025b07: test byte ptr [rbp + 0x7a0], 1
140025b0e: je 0x140025ba6
140025b14: mov rax, qword ptr [rbp + 0x680]
140025b1b: test rax, rax
140025b1e: je 0x140025b3a
140025b20: mov rcx, qword ptr [rbp + 0x688]
140025b27: shl rax, 2
140025b2b: lea rdx, [rax + rax*2]
140025b2f: mov r8d, 4
140025b35: call 0x140613c20 ; ?
140025b3a: movaps xmm6, xmmword ptr [rsp + 0x120]
140025b42: movaps xmm7, xmmword ptr [rsp + 0x110]
140025b4a: movaps xmm8, xmmword ptr [rsp + 0x100]
140025b53: movaps xmm9, xmmword ptr [rsp + 0xf0]
140025b5c: movaps xmm10, xmmword ptr [rsp + 0xe0]
140025b65: movaps xmm11, xmmword ptr [rsp + 0xd0]
140025b6e: movaps xmm12, xmmword ptr [rsp + 0xc0]
140025b77: movaps xmm13, xmmword ptr [rsp + 0xb0]
140025b80: movaps xmm14, xmmword ptr [rsp + 0xa0]
140025b89: movaps xmm15, xmmword ptr [rsp + 0x90]
140025b92: add rsp, 0x138
140025b99: pop rbx
140025b9a: pop rdi
140025b9b: pop rsi
140025b9c: pop r12
140025b9e: pop r13
140025ba0: pop r14
140025ba2: pop r15
140025ba4: pop rbp
140025ba5: ret 
140025ba6: movaps xmm6, xmmword ptr [rsp + 0x120]
140025bae: movaps xmm7, xmmword ptr [rsp + 0x110]
140025bb6: movaps xmm8, xmmword ptr [rsp + 0x100]
140025bbf: movaps xmm9, xmmword ptr [rsp + 0xf0]
140025bc8: movaps xmm10, xmmword ptr [rsp + 0xe0]
140025bd1: movaps xmm11, xmmword ptr [rsp + 0xd0]
140025bda: movaps xmm12, xmmword ptr [rsp + 0xc0]
140025be3: movaps xmm13, xmmword ptr [rsp + 0xb0]
140025bec: movaps xmm14, xmmword ptr [rsp + 0xa0]
140025bf5: movaps xmm15, xmmword ptr [rsp + 0x90]
140025bfe: add rsp, 0x138
140025c05: pop rbx
140025c06: pop rdi
140025c07: pop rsi
140025c08: pop r12
140025c0a: pop r13
140025c0c: pop r14
140025c0e: pop r15
140025c10: pop rbp
140025c11: ret 
140025c12: nop word ptr cs:[rax + rax]
140025c20: mov qword ptr [rsp + 0x10], rdx
140025c25: push rbp
140025c26: push r15
140025c28: push r14
140025c2a: push r13
140025c2c: push r12
140025c2e: push rsi
140025c2f: push rdi
140025c30: push rbx
140025c31: sub rsp, 0x138
140025c38: lea rbp, [rdx + 0x80]
140025c3f: movaps xmmword ptr [rsp + 0x90], xmm15
140025c48: movaps xmmword ptr [rsp + 0xa0], xmm14
140025c51: movaps xmmword ptr [rsp + 0xb0], xmm13
140025c5a: movaps xmmword ptr [rsp + 0xc0], xmm12
140025c63: movaps xmmword ptr [rsp + 0xd0], xmm11
140025c6c: movaps xmmword ptr [rsp + 0xe0], xmm10
140025c75: movaps xmmword ptr [rsp + 0xf0], xmm9
140025c7e: movaps xmmword ptr [rsp + 0x100], xmm8
140025c87: movaps xmmword ptr [rsp + 0x110], xmm7
140025c8f: movaps xmmword ptr [rsp + 0x120], xmm6
140025c97: cmp byte ptr [rbp + 0x808], 0
140025c9e: je 0x140025d32
140025ca4: mov rdx, qword ptr [rbp + 0x650]
140025cab: test rdx, rdx
140025cae: je 0x140025cc6
140025cb0: mov rcx, qword ptr [rbp + 0x658]
140025cb7: shl rdx, 3
140025cbb: mov r8d, 4
140025cc1: call 0x140613c20 ; ?
140025cc6: movaps xmm6, xmmword ptr [rsp + 0x120]
140025cce: movaps xmm7, xmmword ptr [rsp + 0x110]
140025cd6: movaps xmm8, xmmword ptr [rsp + 0x100]
140025cdf: movaps xmm9, xmmword ptr [rsp + 0xf0]
140025ce8: movaps xmm10, xmmword ptr [rsp + 0xe0]
140025cf1: movaps xmm11, xmmword ptr [rsp + 0xd0]
140025cfa: movaps xmm12, xmmword ptr [rsp + 0xc0]
140025d03: movaps xmm13, xmmword ptr [rsp + 0xb0]
140025d0c: movaps xmm14, xmmword ptr [rsp + 0xa0]
140025d15: movaps xmm15, xmmword ptr [rsp + 0x90]
140025d1e: add rsp, 0x138
140025d25: pop rbx
140025d26: pop rdi
140025d27: pop rsi
140025d28: pop r12
140025d2a: pop r13
140025d2c: pop r14
140025d2e: pop r15
140025d30: pop rbp
140025d31: ret 
140025d32: movaps xmm6, xmmword ptr [rsp + 0x120]
140025d3a: movaps xmm7, xmmword ptr [rsp + 0x110]
140025d42: movaps xmm8, xmmword ptr [rsp + 0x100]
140025d4b: movaps xmm9, xmmword ptr [rsp + 0xf0]
140025d54: movaps xmm10, xmmword ptr [rsp + 0xe0]
140025d5d: movaps xmm11, xmmword ptr [rsp + 0xd0]
140025d66: movaps xmm12, xmmword ptr [rsp + 0xc0]
140025d6f: movaps xmm13, xmmword ptr [rsp + 0xb0]
140025d78: movaps xmm14, xmmword ptr [rsp + 0xa0]
140025d81: movaps xmm15, xmmword ptr [rsp + 0x90]
140025d8a: add rsp, 0x138
140025d91: pop rbx
140025d92: pop rdi
140025d93: pop rsi
140025d94: pop r12
140025d96: pop r13
140025d98: pop r14
140025d9a: pop r15
140025d9c: pop rbp
140025d9d: ret 
140025d9e: nop 
140025da0: mov qword ptr [rsp + 0x10], rdx
140025da5: push rbp
140025da6: push r15
140025da8: push r14
140025daa: push r13
140025dac: push r12
140025dae: push rsi
140025daf: push rdi
140025db0: push rbx
140025db1: sub rsp, 0x138
140025db8: lea rbp, [rdx + 0x80]
140025dbf: movaps xmmword ptr [rsp + 0x90], xmm15
140025dc8: movaps xmmword ptr [rsp + 0xa0], xmm14
140025dd1: movaps xmmword ptr [rsp + 0xb0], xmm13
140025dda: movaps xmmword ptr [rsp + 0xc0], xmm12
140025de3: movaps xmmword ptr [rsp + 0xd0], xmm11
140025dec: movaps xmmword ptr [rsp + 0xe0], xmm10
140025df5: movaps xmmword ptr [rsp + 0xf0], xmm9
140025dfe: movaps xmmword ptr [rsp + 0x100], xmm8
140025e07: movaps xmmword ptr [rsp + 0x110], xmm7
140025e0f: movaps xmmword ptr [rsp + 0x120], xmm6
140025e17: cmp byte ptr [rbp + 0x808], 0
140025e1e: je 0x140025ebc
140025e24: mov rdx, qword ptr [rbp + 0x568]
140025e2b: test rdx, rdx
140025e2e: je 0x140025e46
140025e30: mov rcx, qword ptr [rbp + 0x570]
140025e37: shl rdx, 2
140025e3b: mov r8d, 4
140025e41: call 0x140613c20 ; ?
140025e46: mov byte ptr [rbp + 0x817], 1
140025e4d: movaps xmm6, xmmword ptr [rsp + 0x120]
140025e55: movaps xmm7, xmmword ptr [rsp + 0x110]
140025e5d: movaps xmm8, xmmword ptr [rsp + 0x100]
140025e66: movaps xmm9, xmmword ptr [rsp + 0xf0]
140025e6f: movaps xmm10, xmmword ptr [rsp + 0xe0]
140025e78: movaps xmm11, xmmword ptr [rsp + 0xd0]
140025e81: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140025e8b: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140025e95: movaps xmm14, xmmword ptr [rsp + 0xa0]
140025e9e: movdqa xmm15, xmmword ptr [rsp + 0x90]
140025ea8: add rsp, 0x138
140025eaf: pop rbx
140025eb0: pop rdi
140025eb1: pop rsi
140025eb2: pop r12
140025eb4: pop r13
140025eb6: pop r14
140025eb8: pop r15
140025eba: pop rbp
140025ebb: ret 
140025ebc: mov byte ptr [rbp + 0x817], 1
140025ec3: movaps xmm6, xmmword ptr [rsp + 0x120]
140025ecb: movaps xmm7, xmmword ptr [rsp + 0x110]
140025ed3: movaps xmm8, xmmword ptr [rsp + 0x100]
140025edc: movaps xmm9, xmmword ptr [rsp + 0xf0]
140025ee5: movaps xmm10, xmmword ptr [rsp + 0xe0]
140025eee: movaps xmm11, xmmword ptr [rsp + 0xd0]
140025ef7: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140025f01: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140025f0b: movaps xmm14, xmmword ptr [rsp + 0xa0]
140025f14: movdqa xmm15, xmmword ptr [rsp + 0x90]
140025f1e: add rsp, 0x138
140025f25: pop rbx
140025f26: pop rdi
140025f27: pop rsi
140025f28: pop r12
140025f2a: pop r13
140025f2c: pop r14
140025f2e: pop r15
140025f30: pop rbp
140025f31: ret 
140025f32: nop word ptr cs:[rax + rax]
140025f40: mov qword ptr [rsp + 0x10], rdx
140025f45: push rbp
140025f46: push r15
140025f48: push r14
140025f4a: push r13
140025f4c: push r12
140025f4e: push rsi
140025f4f: push rdi
140025f50: push rbx
140025f51: sub rsp, 0x138
140025f58: lea rbp, [rdx + 0x80]
140025f5f: movdqa xmmword ptr [rsp + 0x90], xmm15
140025f69: movaps xmmword ptr [rsp + 0xa0], xmm14
140025f72: movdqa xmmword ptr [rsp + 0xb0], xmm13
140025f7c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140025f86: movaps xmmword ptr [rsp + 0xd0], xmm11
140025f8f: movaps xmmword ptr [rsp + 0xe0], xmm10
140025f98: movaps xmmword ptr [rsp + 0xf0], xmm9
140025fa1: movaps xmmword ptr [rsp + 0x100], xmm8
140025faa: movaps xmmword ptr [rsp + 0x110], xmm7
140025fb2: movaps xmmword ptr [rsp + 0x120], xmm6
140025fba: cmp byte ptr [rbp + 0x813], 0
140025fc1: je 0x140026055
140025fc7: mov rdx, qword ptr [rbp + 0x6a0]
140025fce: test rdx, rdx
140025fd1: je 0x140025fe9
140025fd3: shl rdx, 2
140025fd7: mov r8d, 4
140025fdd: mov rcx, qword ptr [rbp + 0x648]
140025fe4: call 0x140613c20 ; ?
140025fe9: movaps xmm6, xmmword ptr [rsp + 0x120]
140025ff1: movaps xmm7, xmmword ptr [rsp + 0x110]
140025ff9: movaps xmm8, xmmword ptr [rsp + 0x100]
140026002: movaps xmm9, xmmword ptr [rsp + 0xf0]
14002600b: movaps xmm10, xmmword ptr [rsp + 0xe0]
140026014: movaps xmm11, xmmword ptr [rsp + 0xd0]
14002601d: movaps xmm12, xmmword ptr [rsp + 0xc0]
140026026: movaps xmm13, xmmword ptr [rsp + 0xb0]
14002602f: movaps xmm14, xmmword ptr [rsp + 0xa0]
140026038: movaps xmm15, xmmword ptr [rsp + 0x90]
140026041: add rsp, 0x138
140026048: pop rbx
140026049: pop rdi
14002604a: pop rsi
14002604b: pop r12
14002604d: pop r13
14002604f: pop r14
140026051: pop r15
140026053: pop rbp
140026054: ret 
140026055: movaps xmm6, xmmword ptr [rsp + 0x120]
14002605d: movaps xmm7, xmmword ptr [rsp + 0x110]
140026065: movaps xmm8, xmmword ptr [rsp + 0x100]
14002606e: movaps xmm9, xmmword ptr [rsp + 0xf0]
140026077: movaps xmm10, xmmword ptr [rsp + 0xe0]
140026080: movaps xmm11, xmmword ptr [rsp + 0xd0]
140026089: movaps xmm12, xmmword ptr [rsp + 0xc0]
140026092: movaps xmm13, xmmword ptr [rsp + 0xb0]
14002609b: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400260a4: movaps xmm15, xmmword ptr [rsp + 0x90]
1400260ad: add rsp, 0x138
1400260b4: pop rbx
1400260b5: pop rdi
1400260b6: pop rsi
1400260b7: pop r12
1400260b9: pop r13
1400260bb: pop r14
1400260bd: pop r15
1400260bf: pop rbp
1400260c0: ret 
1400260c1: nop word ptr cs:[rax + rax]
1400260d0: mov qword ptr [rsp + 0x10], rdx
1400260d5: push rbp
1400260d6: push r15
1400260d8: push r14
1400260da: push r13
1400260dc: push r12
1400260de: push rsi
1400260df: push rdi
1400260e0: push rbx
1400260e1: sub rsp, 0x138
1400260e8: lea rbp, [rdx + 0x80]
1400260ef: movaps xmmword ptr [rsp + 0x90], xmm15
1400260f8: movaps xmmword ptr [rsp + 0xa0], xmm14
140026101: movaps xmmword ptr [rsp + 0xb0], xmm13
14002610a: movaps xmmword ptr [rsp + 0xc0], xmm12
140026113: movaps xmmword ptr [rsp + 0xd0], xmm11
14002611c: movaps xmmword ptr [rsp + 0xe0], xmm10
140026125: movaps xmmword ptr [rsp + 0xf0], xmm9
14002612e: movaps xmmword ptr [rsp + 0x100], xmm8
140026137: movaps xmmword ptr [rsp + 0x110], xmm7
14002613f: movaps xmmword ptr [rsp + 0x120], xmm6
140026147: lea rcx, [rbp + 0x380]
14002614e: call 0x1405e16f0 ; core::ptr::drop_in_place<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<tuple$<usize,usize>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<alloc::vec::Vec<f32,alloc::alloc::Global>,alloc::alloc::Global> > >
140026153: movaps xmm6, xmmword ptr [rsp + 0x120]
14002615b: movaps xmm7, xmmword ptr [rsp + 0x110]
140026163: movaps xmm8, xmmword ptr [rsp + 0x100]
14002616c: movaps xmm9, xmmword ptr [rsp + 0xf0]
140026175: movaps xmm10, xmmword ptr [rsp + 0xe0]
14002617e: movaps xmm11, xmmword ptr [rsp + 0xd0]
140026187: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140026191: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14002619b: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400261a4: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400261ae: add rsp, 0x138
1400261b5: pop rbx
1400261b6: pop rdi
1400261b7: pop rsi
1400261b8: pop r12
1400261ba: pop r13
1400261bc: pop r14
1400261be: pop r15
1400261c0: pop rbp
1400261c1: ret 
1400261c2: nop word ptr cs:[rax + rax]
1400261d0: mov qword ptr [rsp + 0x10], rdx
1400261d5: push rbp
1400261d6: push r15
1400261d8: push r14
1400261da: push r13
1400261dc: push r12
1400261de: push rsi
1400261df: push rdi
1400261e0: push rbx
1400261e1: sub rsp, 0x138
1400261e8: lea rbp, [rdx + 0x80]
1400261ef: movdqa xmmword ptr [rsp + 0x90], xmm15
1400261f9: movaps xmmword ptr [rsp + 0xa0], xmm14
140026202: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002620c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140026216: movaps xmmword ptr [rsp + 0xd0], xmm11
14002621f: movaps xmmword ptr [rsp + 0xe0], xmm10
140026228: movaps xmmword ptr [rsp + 0xf0], xmm9
140026231: movaps xmmword ptr [rsp + 0x100], xmm8
14002623a: movaps xmmword ptr [rsp + 0x110], xmm7
140026242: movaps xmmword ptr [rsp + 0x120], xmm6
14002624a: lea rcx, [rbp + 0x6c0]
140026251: call 0x1405e1b90 ; core::ptr::drop_in_place<itertools::tuple_impl::CircularTupleWindows<alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> > >
140026256: movaps xmm6, xmmword ptr [rsp + 0x120]
14002625e: movaps xmm7, xmmword ptr [rsp + 0x110]
140026266: movaps xmm8, xmmword ptr [rsp + 0x100]
14002626f: movaps xmm9, xmmword ptr [rsp + 0xf0]
140026278: movaps xmm10, xmmword ptr [rsp + 0xe0]
140026281: movaps xmm11, xmmword ptr [rsp + 0xd0]
14002628a: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140026294: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14002629e: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400262a7: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400262b1: add rsp, 0x138
1400262b8: pop rbx
1400262b9: pop rdi
1400262ba: pop rsi
1400262bb: pop r12
1400262bd: pop r13
1400262bf: pop r14
1400262c1: pop r15
1400262c3: pop rbp
1400262c4: ret 
1400262c5: nop word ptr cs:[rax + rax]
1400262d0: mov qword ptr [rsp + 0x10], rdx
1400262d5: push rbp
1400262d6: push r15
1400262d8: push r14
1400262da: push r13
1400262dc: push r12
1400262de: push rsi
1400262df: push rdi
1400262e0: push rbx
1400262e1: sub rsp, 0x138
1400262e8: lea rbp, [rdx + 0x80]
1400262ef: movdqa xmmword ptr [rsp + 0x90], xmm15
1400262f9: movaps xmmword ptr [rsp + 0xa0], xmm14
140026302: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002630c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140026316: movaps xmmword ptr [rsp + 0xd0], xmm11
14002631f: movaps xmmword ptr [rsp + 0xe0], xmm10
140026328: movaps xmmword ptr [rsp + 0xf0], xmm9
140026331: movaps xmmword ptr [rsp + 0x100], xmm8
14002633a: movaps xmmword ptr [rsp + 0x110], xmm7
140026342: movaps xmmword ptr [rsp + 0x120], xmm6
14002634a: cmp qword ptr [rbp + 0x6a0], 0
140026352: mov rdx, qword ptr [rbp + 0x780]
140026359: je 0x140026373
14002635b: test rdx, rdx
14002635e: je 0x140026373
140026360: mov rcx, qword ptr [rbp + 0x538]
140026367: mov r8, qword ptr [rbp + 0x648]
14002636e: call 0x140613c20 ; ?
140026373: mov byte ptr [rbp + 0x7ef], 0
14002637a: movaps xmm6, xmmword ptr [rsp + 0x120]
140026382: movaps xmm7, xmmword ptr [rsp + 0x110]
14002638a: movaps xmm8, xmmword ptr [rsp + 0x100]
140026393: movaps xmm9, xmmword ptr [rsp + 0xf0]
14002639c: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400263a5: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400263ae: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400263b8: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400263c2: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400263cb: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400263d5: add rsp, 0x138
1400263dc: pop rbx
1400263dd: pop rdi
1400263de: pop rsi
1400263df: pop r12
1400263e1: pop r13
1400263e3: pop r14
1400263e5: pop r15
1400263e7: pop rbp
1400263e8: ret 
1400263e9: nop dword ptr [rax]
1400263f0: mov qword ptr [rsp + 0x10], rdx
1400263f5: push rbp
1400263f6: push r15
1400263f8: push r14
1400263fa: push r13
1400263fc: push r12
1400263fe: push rsi
1400263ff: push rdi
140026400: push rbx
140026401: sub rsp, 0x138
140026408: lea rbp, [rdx + 0x80]
14002640f: movdqa xmmword ptr [rsp + 0x90], xmm15
140026419: movaps xmmword ptr [rsp + 0xa0], xmm14
140026422: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002642c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140026436: movaps xmmword ptr [rsp + 0xd0], xmm11
14002643f: movaps xmmword ptr [rsp + 0xe0], xmm10
140026448: movaps xmmword ptr [rsp + 0xf0], xmm9
140026451: movaps xmmword ptr [rsp + 0x100], xmm8
14002645a: movaps xmmword ptr [rsp + 0x110], xmm7
140026462: movaps xmmword ptr [rsp + 0x120], xmm6
14002646a: cmp byte ptr [rbp + 0x7ef], 0
140026471: je 0x1400264f9
140026477: lea rcx, [rbp + 0x460]
14002647e: call 0x140149f50 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>
140026483: mov byte ptr [rbp + 0x805], 1
14002648a: movaps xmm6, xmmword ptr [rsp + 0x120]
140026492: movaps xmm7, xmmword ptr [rsp + 0x110]
14002649a: movaps xmm8, xmmword ptr [rsp + 0x100]
1400264a3: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400264ac: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400264b5: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400264be: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400264c8: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400264d2: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400264db: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400264e5: add rsp, 0x138
1400264ec: pop rbx
1400264ed: pop rdi
1400264ee: pop rsi
1400264ef: pop r12
1400264f1: pop r13
1400264f3: pop r14
1400264f5: pop r15
1400264f7: pop rbp
1400264f8: ret 
1400264f9: mov byte ptr [rbp + 0x805], 1
140026500: movaps xmm6, xmmword ptr [rsp + 0x120]
140026508: movaps xmm7, xmmword ptr [rsp + 0x110]
140026510: movaps xmm8, xmmword ptr [rsp + 0x100]
140026519: movaps xmm9, xmmword ptr [rsp + 0xf0]
140026522: movaps xmm10, xmmword ptr [rsp + 0xe0]
14002652b: movaps xmm11, xmmword ptr [rsp + 0xd0]
140026534: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14002653e: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140026548: movaps xmm14, xmmword ptr [rsp + 0xa0]
140026551: movdqa xmm15, xmmword ptr [rsp + 0x90]
14002655b: add rsp, 0x138
140026562: pop rbx
140026563: pop rdi
140026564: pop rsi
140026565: pop r12
140026567: pop r13
140026569: pop r14
14002656b: pop r15
14002656d: pop rbp
14002656e: ret 
14002656f: nop 
140026570: mov qword ptr [rsp + 0x10], rdx
140026575: push rbp
140026576: push r15
140026578: push r14
14002657a: push r13
14002657c: push r12
14002657e: push rsi
14002657f: push rdi
140026580: push rbx
140026581: sub rsp, 0x138
140026588: lea rbp, [rdx + 0x80]
14002658f: movdqa xmmword ptr [rsp + 0x90], xmm15
140026599: movaps xmmword ptr [rsp + 0xa0], xmm14
1400265a2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400265ac: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400265b6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400265bf: movaps xmmword ptr [rsp + 0xe0], xmm10
1400265c8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400265d1: movaps xmmword ptr [rsp + 0x100], xmm8
1400265da: movaps xmmword ptr [rsp + 0x110], xmm7
1400265e2: movaps xmmword ptr [rsp + 0x120], xmm6
1400265ea: mov rdx, qword ptr [rbp + 0x648]
1400265f1: test rdx, rdx
1400265f4: je 0x14002660c
1400265f6: shl rdx, 4
1400265fa: mov r8d, 8
140026600: mov rcx, qword ptr [rbp + 0x580]
140026607: call 0x140613c20 ; ?
14002660c: movaps xmm6, xmmword ptr [rsp + 0x120]
140026614: movaps xmm7, xmmword ptr [rsp + 0x110]
14002661c: movaps xmm8, xmmword ptr [rsp + 0x100]
140026625: movaps xmm9, xmmword ptr [rsp + 0xf0]
14002662e: movaps xmm10, xmmword ptr [rsp + 0xe0]
140026637: movaps xmm11, xmmword ptr [rsp + 0xd0]
140026640: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14002664a: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140026654: movaps xmm14, xmmword ptr [rsp + 0xa0]
14002665d: movdqa xmm15, xmmword ptr [rsp + 0x90]
140026667: add rsp, 0x138
14002666e: pop rbx
14002666f: pop rdi
140026670: pop rsi
140026671: pop r12
140026673: pop r13
140026675: pop r14
140026677: pop r15
140026679: pop rbp
14002667a: ret 
14002667b: nop dword ptr [rax + rax]
140026680: mov qword ptr [rsp + 0x10], rdx
140026685: push rbp
140026686: push r15
140026688: push r14
14002668a: push r13
14002668c: push r12
14002668e: push rsi
14002668f: push rdi
140026690: push rbx
140026691: sub rsp, 0x138
140026698: lea rbp, [rdx + 0x80]
14002669f: movdqa xmmword ptr [rsp + 0x90], xmm15
1400266a9: movaps xmmword ptr [rsp + 0xa0], xmm14
1400266b2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400266bc: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400266c6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400266cf: movaps xmmword ptr [rsp + 0xe0], xmm10
1400266d8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400266e1: movaps xmmword ptr [rsp + 0x100], xmm8
1400266ea: movaps xmmword ptr [rsp + 0x110], xmm7
1400266f2: movaps xmmword ptr [rsp + 0x120], xmm6
1400266fa: mov rax, qword ptr [rbp + 0x560]
140026701: cmp rax, qword ptr [rbp + 0x7e0]
140026708: jne 0x140026780
14002670a: mov byte ptr [rbp + 0x814], 1
140026711: movaps xmm6, xmmword ptr [rsp + 0x120]
140026719: movaps xmm7, xmmword ptr [rsp + 0x110]
140026721: movaps xmm8, xmmword ptr [rsp + 0x100]
14002672a: movaps xmm9, xmmword ptr [rsp + 0xf0]
140026733: movaps xmm10, xmmword ptr [rsp + 0xe0]
14002673c: movaps xmm11, xmmword ptr [rsp + 0xd0]
140026745: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14002674f: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140026759: movaps xmm14, xmmword ptr [rsp + 0xa0]
140026762: movdqa xmm15, xmmword ptr [rsp + 0x90]
14002676c: add rsp, 0x138
140026773: pop rbx
140026774: pop rdi
140026775: pop rsi
140026776: pop r12
140026778: pop r13
14002677a: pop r14
14002677c: pop r15
14002677e: pop rbp
14002677f: ret 
140026780: mov r14, qword ptr [rbp + 0x500]
140026787: test r14, r14
14002678a: mov rdi, qword ptr [rbp + 0x528]
140026791: mov rbx, qword ptr [rbp + 0x640]
140026798: jne 0x140026851
14002679e: test rdi, rdi
1400267a1: je 0x1400267bd
1400267a3: shl rdi, 3
1400267a7: lea rdx, [rdi + rdi*2]
1400267ab: mov r8d, 8
1400267b1: mov rcx, qword ptr [rbp + 0x6a8]
1400267b8: call 0x140613c20 ; ?
1400267bd: test rbx, rbx
1400267c0: je 0x1400267db
1400267c2: shl rbx, 4
1400267c6: mov r8d, 8
1400267cc: mov rcx, qword ptr [rbp + 0x540]
1400267d3: mov rdx, rbx
1400267d6: call 0x140613c20 ; ?
1400267db: mov byte ptr [rbp + 0x814], 1
1400267e2: movaps xmm6, xmmword ptr [rsp + 0x120]
1400267ea: movaps xmm7, xmmword ptr [rsp + 0x110]
1400267f2: movaps xmm8, xmmword ptr [rsp + 0x100]
1400267fb: movaps xmm9, xmmword ptr [rsp + 0xf0]
140026804: movaps xmm10, xmmword ptr [rsp + 0xe0]
14002680d: movaps xmm11, xmmword ptr [rsp + 0xd0]
140026816: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140026820: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14002682a: movaps xmm14, xmmword ptr [rsp + 0xa0]
140026833: movdqa xmm15, xmmword ptr [rsp + 0x90]
14002683d: add rsp, 0x138
140026844: pop rbx
140026845: pop rdi
140026846: pop rsi
140026847: pop r12
140026849: pop r13
14002684b: pop r14
14002684d: pop r15
14002684f: pop rbp
140026850: ret 
140026851: mov rax, qword ptr [rbp + 0x6a8]
140026858: lea rsi, [rax + 8]
14002685c: jmp 0x14002686d ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14002685e: nop 
140026860: add rsi, 0x18
140026864: dec r14
140026867: je 0x14002679e
14002686d: mov rdx, qword ptr [rsi - 8]
140026871: test rdx, rdx
140026874: je 0x140026860
140026876: mov rcx, qword ptr [rsi]
140026879: shl rdx, 2
14002687d: mov r8d, 4
140026883: call 0x140613c20 ; ?
140026888: jmp 0x140026860 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
14002688a: nop word ptr [rax + rax]
140026890: mov qword ptr [rsp + 0x10], rdx
140026895: push rbp
140026896: push r15
140026898: push r14
14002689a: push r13
14002689c: push r12
14002689e: push rsi
14002689f: push rdi
1400268a0: push rbx
1400268a1: sub rsp, 0x138
1400268a8: lea rbp, [rdx + 0x80]
1400268af: movdqa xmmword ptr [rsp + 0x90], xmm15
1400268b9: movaps xmmword ptr [rsp + 0xa0], xmm14
1400268c2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400268cc: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400268d6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400268df: movaps xmmword ptr [rsp + 0xe0], xmm10
1400268e8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400268f1: movaps xmmword ptr [rsp + 0x100], xmm8
1400268fa: movaps xmmword ptr [rsp + 0x110], xmm7
140026902: movaps xmmword ptr [rsp + 0x120], xmm6
14002690a: cmp byte ptr [rbp + 0x814], 0
140026911: je 0x140026a0b
140026917: mov rax, qword ptr [rbp + 0x460]
14002691e: test rax, rax
140026921: jne 0x140026a9d
140026927: mov rdx, qword ptr [rbp + 0x478]
14002692e: test rdx, rdx
140026931: jne 0x140026ac7
140026937: mov rdx, qword ptr [rbp + 0x498]
14002693e: test rdx, rdx
140026941: jne 0x140026aed
140026947: mov rdx, qword ptr [rbp + 0x4b0]
14002694e: test rdx, rdx
140026951: jne 0x140026b13
140026957: mov rdx, qword ptr [rbp + 0x4c8]
14002695e: test rdx, rdx
140026961: je 0x140026979
140026963: mov rcx, qword ptr [rbp + 0x4d0]
14002696a: shl rdx, 2
14002696e: mov r8d, 4
140026974: call 0x140613c20 ; ?
140026979: mov byte ptr [rbp + 0x803], 0
140026980: mov byte ptr [rbp + 0x802], 0
140026987: mov byte ptr [rbp + 0x801], 0
14002698e: mov byte ptr [rbp + 0x800], 1
140026995: mov byte ptr [rbp + 0x7ff], 1
14002699c: movaps xmm6, xmmword ptr [rsp + 0x120]
1400269a4: movaps xmm7, xmmword ptr [rsp + 0x110]
1400269ac: movaps xmm8, xmmword ptr [rsp + 0x100]
1400269b5: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400269be: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400269c7: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400269d0: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400269da: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400269e4: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400269ed: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400269f7: add rsp, 0x138
1400269fe: pop rbx
1400269ff: pop rdi
140026a00: pop rsi
140026a01: pop r12
140026a03: pop r13
140026a05: pop r14
140026a07: pop r15
140026a09: pop rbp
140026a0a: ret 
140026a0b: mov byte ptr [rbp + 0x803], 0
140026a12: mov byte ptr [rbp + 0x802], 0
140026a19: mov byte ptr [rbp + 0x801], 0
140026a20: mov byte ptr [rbp + 0x800], 1
140026a27: mov byte ptr [rbp + 0x7ff], 1
140026a2e: movaps xmm6, xmmword ptr [rsp + 0x120]
140026a36: movaps xmm7, xmmword ptr [rsp + 0x110]
140026a3e: movaps xmm8, xmmword ptr [rsp + 0x100]
140026a47: movaps xmm9, xmmword ptr [rsp + 0xf0]
140026a50: movaps xmm10, xmmword ptr [rsp + 0xe0]
140026a59: movaps xmm11, xmmword ptr [rsp + 0xd0]
140026a62: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140026a6c: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140026a76: movaps xmm14, xmmword ptr [rsp + 0xa0]
140026a7f: movdqa xmm15, xmmword ptr [rsp + 0x90]
140026a89: add rsp, 0x138
140026a90: pop rbx
140026a91: pop rdi
140026a92: pop rsi
140026a93: pop r12
140026a95: pop r13
140026a97: pop r14
140026a99: pop r15
140026a9b: pop rbp
140026a9c: ret 
140026a9d: mov rcx, qword ptr [rbp + 0x468]
140026aa4: shl rax, 2
140026aa8: lea rdx, [rax + rax*2]
140026aac: mov r8d, 4
140026ab2: call 0x140613c20 ; ?
140026ab7: mov rdx, qword ptr [rbp + 0x478]
140026abe: test rdx, rdx
140026ac1: je 0x140026937
140026ac7: mov rcx, qword ptr [rbp + 0x480]
140026ace: shl rdx, 2
140026ad2: mov r8d, 4
140026ad8: call 0x140613c20 ; ?
140026add: mov rdx, qword ptr [rbp + 0x498]
140026ae4: test rdx, rdx
140026ae7: je 0x140026947
140026aed: mov rcx, qword ptr [rbp + 0x4a0]
140026af4: shl rdx, 3
140026af8: mov r8d, 4
140026afe: call 0x140613c20 ; ?
140026b03: mov rdx, qword ptr [rbp + 0x4b0]
140026b0a: test rdx, rdx
140026b0d: je 0x140026957
140026b13: mov rcx, qword ptr [rbp + 0x4b8]
140026b1a: shl rdx, 2
140026b1e: mov r8d, 4
140026b24: call 0x140613c20 ; ?
140026b29: mov rdx, qword ptr [rbp + 0x4c8]
140026b30: test rdx, rdx
140026b33: jne 0x140026963
140026b39: jmp 0x140026979 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140026b3e: nop 
140026b40: mov qword ptr [rsp + 0x10], rdx
140026b45: push rbp
140026b46: push r15
140026b48: push r14
140026b4a: push r13
140026b4c: push r12
140026b4e: push rsi
140026b4f: push rdi
140026b50: push rbx
140026b51: sub rsp, 0x138
140026b58: lea rbp, [rdx + 0x80]
140026b5f: movdqa xmmword ptr [rsp + 0x90], xmm15
140026b69: movaps xmmword ptr [rsp + 0xa0], xmm14
140026b72: movdqa xmmword ptr [rsp + 0xb0], xmm13
140026b7c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140026b86: movaps xmmword ptr [rsp + 0xd0], xmm11
140026b8f: movaps xmmword ptr [rsp + 0xe0], xmm10
140026b98: movaps xmmword ptr [rsp + 0xf0], xmm9
140026ba1: movaps xmmword ptr [rsp + 0x100], xmm8
140026baa: movaps xmmword ptr [rsp + 0x110], xmm7
140026bb2: movaps xmmword ptr [rsp + 0x120], xmm6
140026bba: movzx r14d, byte ptr [rbp + 0x800]
140026bc2: movzx edi, byte ptr [rbp + 0x801]
140026bc9: movzx esi, byte ptr [rbp + 0x802]
140026bd0: movzx ebx, byte ptr [rbp + 0x803]
140026bd7: cmp qword ptr [rbp + 0x808], 0
140026bdf: je 0x140026c0b
140026be1: test byte ptr [rbp + 0x7ff], 1
140026be8: jne 0x140026c0b
140026bea: mov rax, qword ptr [rbp + 0x808]
140026bf1: shl rax, 2
140026bf5: lea rdx, [rax + rax*2]
140026bf9: mov r8d, 4
140026bff: mov rcx, qword ptr [rbp + 0x6b0]
140026c06: call 0x140613c20 ; ?
140026c0b: mov rdx, qword ptr [rbp + 0x558]
140026c12: test rdx, rdx
140026c15: sete al
140026c18: or r14b, al
140026c1b: test r14b, 1
140026c1f: jne 0x140026c37
140026c21: shl rdx, 2
140026c25: mov r8d, 4
140026c2b: mov rcx, qword ptr [rbp + 0x430]
140026c32: call 0x140613c20 ; ?
140026c37: mov rdx, qword ptr [rbp + 0x790]
140026c3e: test rdx, rdx
140026c41: setne al
140026c44: test dil, al
140026c47: je 0x140026c5f
140026c49: shl rdx, 3
140026c4d: mov r8d, 4
140026c53: mov rcx, qword ptr [rbp + 0x7f0]
140026c5a: call 0x140613c20 ; ?
140026c5f: mov rdx, qword ptr [rbp + 0x7a0]
140026c66: test rdx, rdx
140026c69: setne al
140026c6c: test sil, al
140026c6f: je 0x140026c87
140026c71: shl rdx, 2
140026c75: mov r8d, 4
140026c7b: mov rcx, qword ptr [rbp + 0x7d0]
140026c82: call 0x140613c20 ; ?
140026c87: test bl, 1
140026c8a: je 0x140026d1e
140026c90: mov rdx, qword ptr [rbp + 0x698]
140026c97: test rdx, rdx
140026c9a: je 0x140026cb2
140026c9c: shl rdx, 2
140026ca0: mov r8d, 4
140026ca6: mov rcx, qword ptr [rbp + 0x7d8]
140026cad: call 0x140613c20 ; ?
140026cb2: movaps xmm6, xmmword ptr [rsp + 0x120]
140026cba: movaps xmm7, xmmword ptr [rsp + 0x110]
140026cc2: movaps xmm8, xmmword ptr [rsp + 0x100]
140026ccb: movaps xmm9, xmmword ptr [rsp + 0xf0]
140026cd4: movaps xmm10, xmmword ptr [rsp + 0xe0]
140026cdd: movaps xmm11, xmmword ptr [rsp + 0xd0]
140026ce6: movaps xmm12, xmmword ptr [rsp + 0xc0]
140026cef: movaps xmm13, xmmword ptr [rsp + 0xb0]
140026cf8: movaps xmm14, xmmword ptr [rsp + 0xa0]
140026d01: movaps xmm15, xmmword ptr [rsp + 0x90]
140026d0a: add rsp, 0x138
140026d11: pop rbx
140026d12: pop rdi
140026d13: pop rsi
140026d14: pop r12
140026d16: pop r13
140026d18: pop r14
140026d1a: pop r15
140026d1c: pop rbp
140026d1d: ret 
140026d1e: movaps xmm6, xmmword ptr [rsp + 0x120]
140026d26: movaps xmm7, xmmword ptr [rsp + 0x110]
140026d2e: movaps xmm8, xmmword ptr [rsp + 0x100]
140026d37: movaps xmm9, xmmword ptr [rsp + 0xf0]
140026d40: movaps xmm10, xmmword ptr [rsp + 0xe0]
140026d49: movaps xmm11, xmmword ptr [rsp + 0xd0]
140026d52: movaps xmm12, xmmword ptr [rsp + 0xc0]
140026d5b: movaps xmm13, xmmword ptr [rsp + 0xb0]
140026d64: movaps xmm14, xmmword ptr [rsp + 0xa0]
140026d6d: movaps xmm15, xmmword ptr [rsp + 0x90]
140026d76: add rsp, 0x138
140026d7d: pop rbx
140026d7e: pop rdi
140026d7f: pop rsi
140026d80: pop r12
140026d82: pop r13
140026d84: pop r14
140026d86: pop r15
140026d88: pop rbp
140026d89: ret 
140026d8a: nop word ptr [rax + rax]
140026d90: mov qword ptr [rsp + 0x10], rdx
140026d95: push rbp
140026d96: push r15
140026d98: push r14
140026d9a: push r13
140026d9c: push r12
140026d9e: push rsi
140026d9f: push rdi
140026da0: push rbx
140026da1: sub rsp, 0x138
140026da8: lea rbp, [rdx + 0x80]
140026daf: movaps xmmword ptr [rsp + 0x90], xmm15
140026db8: movaps xmmword ptr [rsp + 0xa0], xmm14
140026dc1: movaps xmmword ptr [rsp + 0xb0], xmm13
140026dca: movaps xmmword ptr [rsp + 0xc0], xmm12
140026dd3: movaps xmmword ptr [rsp + 0xd0], xmm11
140026ddc: movaps xmmword ptr [rsp + 0xe0], xmm10
140026de5: movaps xmmword ptr [rsp + 0xf0], xmm9
140026dee: movaps xmmword ptr [rsp + 0x100], xmm8
140026df7: movaps xmmword ptr [rsp + 0x110], xmm7
140026dff: movaps xmmword ptr [rsp + 0x120], xmm6
140026e07: xor eax, eax
140026e09: cmp rax, qword ptr [rbp + 0x6c0]
140026e10: jo 0x140026e1e
140026e12: lea rcx, [rbp + 0x6c0]
140026e19: call 0x140609eb0 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams>
140026e1e: mov byte ptr [rbp + 0x805], 0
140026e25: movaps xmm6, xmmword ptr [rsp + 0x120]
140026e2d: movaps xmm7, xmmword ptr [rsp + 0x110]
140026e35: movaps xmm8, xmmword ptr [rsp + 0x100]
140026e3e: movaps xmm9, xmmword ptr [rsp + 0xf0]
140026e47: movaps xmm10, xmmword ptr [rsp + 0xe0]
140026e50: movaps xmm11, xmmword ptr [rsp + 0xd0]
140026e59: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140026e63: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140026e6d: movaps xmm14, xmmword ptr [rsp + 0xa0]
140026e76: movdqa xmm15, xmmword ptr [rsp + 0x90]
140026e80: add rsp, 0x138
140026e87: pop rbx
140026e88: pop rdi
140026e89: pop rsi
140026e8a: pop r12
140026e8c: pop r13
140026e8e: pop r14
140026e90: pop r15
140026e92: pop rbp
140026e93: ret 
140026e94: nop word ptr cs:[rax + rax]
140026ea0: mov qword ptr [rsp + 0x10], rdx
140026ea5: push rbp
140026ea6: push r15
140026ea8: push r14
140026eaa: push r13
140026eac: push r12
140026eae: push rsi
140026eaf: push rdi
140026eb0: push rbx
140026eb1: sub rsp, 0x138
140026eb8: lea rbp, [rdx + 0x80]
140026ebf: movdqa xmmword ptr [rsp + 0x90], xmm15
140026ec9: movaps xmmword ptr [rsp + 0xa0], xmm14
140026ed2: movdqa xmmword ptr [rsp + 0xb0], xmm13
140026edc: movdqa xmmword ptr [rsp + 0xc0], xmm12
140026ee6: movaps xmmword ptr [rsp + 0xd0], xmm11
140026eef: movaps xmmword ptr [rsp + 0xe0], xmm10
140026ef8: movaps xmmword ptr [rsp + 0xf0], xmm9
140026f01: movaps xmmword ptr [rsp + 0x100], xmm8
140026f0a: movaps xmmword ptr [rsp + 0x110], xmm7
140026f12: movaps xmmword ptr [rsp + 0x120], xmm6
140026f1a: cmp byte ptr [rbp + 0x805], 0
140026f21: je 0x140026fbf
140026f27: cmp byte ptr [rbp + 0x508], 0x11
140026f2e: je 0x140026f49
140026f30: mov rax, qword ptr [rbp + 0x520]
140026f37: lock dec qword ptr [rax]
140026f3b: jne 0x140026f49
140026f3d: lea rcx, [rbp + 0x520]
140026f44: call 0x140b5c620 ; alloc::sync::Arc::drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
140026f49: mov byte ptr [rbp + 0x817], 0
140026f50: movaps xmm6, xmmword ptr [rsp + 0x120]
140026f58: movaps xmm7, xmmword ptr [rsp + 0x110]
140026f60: movaps xmm8, xmmword ptr [rsp + 0x100]
140026f69: movaps xmm9, xmmword ptr [rsp + 0xf0]
140026f72: movaps xmm10, xmmword ptr [rsp + 0xe0]
140026f7b: movaps xmm11, xmmword ptr [rsp + 0xd0]
140026f84: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140026f8e: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140026f98: movaps xmm14, xmmword ptr [rsp + 0xa0]
140026fa1: movdqa xmm15, xmmword ptr [rsp + 0x90]
140026fab: add rsp, 0x138
140026fb2: pop rbx
140026fb3: pop rdi
140026fb4: pop rsi
140026fb5: pop r12
140026fb7: pop r13
140026fb9: pop r14
140026fbb: pop r15
140026fbd: pop rbp
140026fbe: ret 
140026fbf: mov byte ptr [rbp + 0x817], 0
140026fc6: movaps xmm6, xmmword ptr [rsp + 0x120]
140026fce: movaps xmm7, xmmword ptr [rsp + 0x110]
140026fd6: movaps xmm8, xmmword ptr [rsp + 0x100]
140026fdf: movaps xmm9, xmmword ptr [rsp + 0xf0]
140026fe8: movaps xmm10, xmmword ptr [rsp + 0xe0]
140026ff1: movaps xmm11, xmmword ptr [rsp + 0xd0]
140026ffa: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140027004: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14002700e: movaps xmm14, xmmword ptr [rsp + 0xa0]
140027017: movdqa xmm15, xmmword ptr [rsp + 0x90]
140027021: add rsp, 0x138
140027028: pop rbx
140027029: pop rdi
14002702a: pop rsi
14002702b: pop r12
14002702d: pop r13
14002702f: pop r14
140027031: pop r15
140027033: pop rbp
140027034: ret 
140027035: nop word ptr cs:[rax + rax]
140027040: mov qword ptr [rsp + 0x10], rdx
140027045: push rbp
140027046: push r15
140027048: push r14
14002704a: push r13
14002704c: push r12
14002704e: push rsi
14002704f: push rdi
140027050: push rbx
140027051: sub rsp, 0x138
140027058: lea rbp, [rdx + 0x80]
14002705f: movdqa xmmword ptr [rsp + 0x90], xmm15
140027069: movaps xmmword ptr [rsp + 0xa0], xmm14
140027072: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002707c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140027086: movaps xmmword ptr [rsp + 0xd0], xmm11
14002708f: movaps xmmword ptr [rsp + 0xe0], xmm10
140027098: movaps xmmword ptr [rsp + 0xf0], xmm9
1400270a1: movaps xmmword ptr [rsp + 0x100], xmm8
1400270aa: movaps xmmword ptr [rsp + 0x110], xmm7
1400270b2: movaps xmmword ptr [rsp + 0x120], xmm6
1400270ba: cmp byte ptr [rbp + 0x817], 0
1400270c1: je 0x140027181
1400270c7: mov rsi, qword ptr [rbp + 0x280]
1400270ce: mov rdi, qword ptr [rbp + 0x288]
1400270d5: xor ebx, ebx
1400270d7: mov rcx, rsi
1400270da: cmp rdi, rbx
1400270dd: je 0x1400270f7
1400270df: nop 
1400270e0: inc rbx
1400270e3: lea r14, [rcx + 0x80]
1400270ea: call 0x140601be0 ; core::ptr::drop_in_place<country_core::systems::collision::world::RaycastMeshData>
1400270ef: mov rcx, r14
1400270f2: cmp rdi, rbx
1400270f5: jne 0x1400270e0
1400270f7: mov rdx, qword ptr [rbp + 0x278]
1400270fe: test rdx, rdx
140027101: je 0x140027115
140027103: shl rdx, 7
140027107: mov r8d, 0x10
14002710d: mov rcx, rsi
140027110: call 0x140613c20 ; ?
140027115: movaps xmm6, xmmword ptr [rsp + 0x120]
14002711d: movaps xmm7, xmmword ptr [rsp + 0x110]
140027125: movaps xmm8, xmmword ptr [rsp + 0x100]
14002712e: movaps xmm9, xmmword ptr [rsp + 0xf0]
140027137: movaps xmm10, xmmword ptr [rsp + 0xe0]
140027140: movaps xmm11, xmmword ptr [rsp + 0xd0]
140027149: movaps xmm12, xmmword ptr [rsp + 0xc0]
140027152: movaps xmm13, xmmword ptr [rsp + 0xb0]
14002715b: movaps xmm14, xmmword ptr [rsp + 0xa0]
140027164: movaps xmm15, xmmword ptr [rsp + 0x90]
14002716d: add rsp, 0x138
140027174: pop rbx
140027175: pop rdi
140027176: pop rsi
140027177: pop r12
140027179: pop r13
14002717b: pop r14
14002717d: pop r15
14002717f: pop rbp
140027180: ret 
140027181: movaps xmm6, xmmword ptr [rsp + 0x120]
140027189: movaps xmm7, xmmword ptr [rsp + 0x110]
140027191: movaps xmm8, xmmword ptr [rsp + 0x100]
14002719a: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400271a3: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400271ac: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400271b5: movaps xmm12, xmmword ptr [rsp + 0xc0]
1400271be: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400271c7: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400271d0: movaps xmm15, xmmword ptr [rsp + 0x90]
1400271d9: add rsp, 0x138
1400271e0: pop rbx
1400271e1: pop rdi
1400271e2: pop rsi
1400271e3: pop r12
1400271e5: pop r13
1400271e7: pop r14
1400271e9: pop r15
1400271eb: pop rbp
1400271ec: ret 
1400271ed: nop dword ptr [rax]
1400271f0: mov qword ptr [rsp + 0x10], rdx
1400271f5: push rbp
1400271f6: push r15
1400271f8: push r14
1400271fa: push r13
1400271fc: push r12
1400271fe: push rsi
1400271ff: push rdi
140027200: push rbx
140027201: sub rsp, 0x138
140027208: lea rbp, [rdx + 0x80]
14002720f: movaps xmmword ptr [rsp + 0x90], xmm15
140027218: movaps xmmword ptr [rsp + 0xa0], xmm14
140027221: movaps xmmword ptr [rsp + 0xb0], xmm13
14002722a: movaps xmmword ptr [rsp + 0xc0], xmm12
140027233: movaps xmmword ptr [rsp + 0xd0], xmm11
14002723c: movaps xmmword ptr [rsp + 0xe0], xmm10
140027245: movaps xmmword ptr [rsp + 0xf0], xmm9
14002724e: movaps xmmword ptr [rsp + 0x100], xmm8
140027257: movaps xmmword ptr [rsp + 0x110], xmm7
14002725f: movaps xmmword ptr [rsp + 0x120], xmm6
140027267: lea rcx, [rbp + 0x440]
14002726e: call 0x1405dec70 ; core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::ArchSegment,alloc::alloc::Global> >
140027273: movaps xmm6, xmmword ptr [rsp + 0x120]
14002727b: movaps xmm7, xmmword ptr [rsp + 0x110]
140027283: movaps xmm8, xmmword ptr [rsp + 0x100]
14002728c: movaps xmm9, xmmword ptr [rsp + 0xf0]
140027295: movaps xmm10, xmmword ptr [rsp + 0xe0]
14002729e: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400272a7: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400272b1: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400272bb: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400272c4: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400272ce: add rsp, 0x138
1400272d5: pop rbx
1400272d6: pop rdi
1400272d7: pop rsi
1400272d8: pop r12
1400272da: pop r13
1400272dc: pop r14
1400272de: pop r15
1400272e0: pop rbp
1400272e1: ret 
1400272e2: nop word ptr cs:[rax + rax]
1400272f0: mov qword ptr [rsp + 0x10], rdx
1400272f5: push rbp
1400272f6: push r15
1400272f8: push r14
1400272fa: push r13
1400272fc: push r12
1400272fe: push rsi
1400272ff: push rdi
140027300: push rbx
140027301: sub rsp, 0x138
140027308: lea rbp, [rdx + 0x80]
14002730f: movdqa xmmword ptr [rsp + 0x90], xmm15
140027319: movaps xmmword ptr [rsp + 0xa0], xmm14
140027322: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002732c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140027336: movaps xmmword ptr [rsp + 0xd0], xmm11
14002733f: movaps xmmword ptr [rsp + 0xe0], xmm10
140027348: movaps xmmword ptr [rsp + 0xf0], xmm9
140027351: movaps xmmword ptr [rsp + 0x100], xmm8
14002735a: movaps xmmword ptr [rsp + 0x110], xmm7
140027362: movaps xmmword ptr [rsp + 0x120], xmm6
14002736a: mov rdi, qword ptr [rbp + 0x160]
140027371: lea rsi, [rdi + 0x18]
140027375: mov rcx, rsi
140027378: call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
14002737d: xor ecx, ecx
14002737f: mov al, 1
140027381: lock cmpxchg byte ptr [rdi + 0x18], cl
140027386: jne 0x140027416
14002738c: mov rdi, qword ptr [rbp + 0x1b8]
140027393: lea rsi, [rdi + 0x18]
140027397: mov rcx, rsi
14002739a: call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
14002739f: xor ecx, ecx
1400273a1: mov al, 1
1400273a3: lock cmpxchg byte ptr [rdi + 0x18], cl
1400273a8: jne 0x140027425
1400273aa: movaps xmm6, xmmword ptr [rsp + 0x120]
1400273b2: movaps xmm7, xmmword ptr [rsp + 0x110]
1400273ba: movaps xmm8, xmmword ptr [rsp + 0x100]
1400273c3: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400273cc: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400273d5: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400273de: movaps xmm12, xmmword ptr [rsp + 0xc0]
1400273e7: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400273f0: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400273f9: movaps xmm15, xmmword ptr [rsp + 0x90]
140027402: add rsp, 0x138
140027409: pop rbx
14002740a: pop rdi
14002740b: pop rsi
14002740c: pop r12
14002740e: pop r13
140027410: pop r14
140027412: pop r15
140027414: pop rbp
140027415: ret 
140027416: mov rcx, rsi
140027419: xor edx, edx
14002741b: call 0x1428cdb90 ; parking_lot::raw_mutex::RawMutex::unlock_slow
140027420: jmp 0x14002738c ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140027425: mov rcx, rsi
140027428: xor edx, edx
14002742a: call 0x1428cdb90 ; parking_lot::raw_mutex::RawMutex::unlock_slow
14002742f: jmp 0x1400273aa ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false>
140027434: nop word ptr cs:[rax + rax]
140027440: mov qword ptr [rsp + 0x10], rdx
140027445: push rbp
140027446: push r15
140027448: push r14
14002744a: push r13
14002744c: push r12
14002744e: push rsi
14002744f: push rdi
140027450: push rbx
140027451: sub rsp, 0x138
140027458: lea rbp, [rdx + 0x80]
14002745f: movdqa xmmword ptr [rsp + 0x90], xmm15
140027469: movaps xmmword ptr [rsp + 0xa0], xmm14
140027472: movdqa xmmword ptr [rsp + 0xb0], xmm13
14002747c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140027486: movaps xmmword ptr [rsp + 0xd0], xmm11
14002748f: movaps xmmword ptr [rsp + 0xe0], xmm10
140027498: movaps xmmword ptr [rsp + 0xf0], xmm9
1400274a1: movaps xmmword ptr [rsp + 0x100], xmm8
1400274aa: movaps xmmword ptr [rsp + 0x110], xmm7
1400274b2: movaps xmmword ptr [rsp + 0x120], xmm6
1400274ba: cmp byte ptr [rbp + 0x79f], 0
1400274c1: je 0x140027500
1400274c3: call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
1400274c8: mov rsi, rax
1400274cb: mov rax, qword ptr [rax]
1400274ce: cmp rax, 1
1400274d2: mov rdx, qword ptr [rbp + 0x270]
1400274d9: jne 0x14002756c
1400274df: add rsi, 8
1400274e3: cmp qword ptr [rsi], 0
1400274e7: jne 0x140027599
1400274ed: mov qword ptr [rsi], 0xffffffffffffffff
1400274f4: lea rcx, [rsi + 8]
1400274f8: call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
1400274fd: inc qword ptr [rsi]
140027500: movaps xmm6, xmmword ptr [rsp + 0x120]
140027508: movaps xmm7, xmmword ptr [rsp + 0x110]
140027510: movaps xmm8, xmmword ptr [rsp + 0x100]
140027519: movaps xmm9, xmmword ptr [rsp + 0xf0]
140027522: movaps xmm10, xmmword ptr [rsp + 0xe0]
14002752b: movaps xmm11, xmmword ptr [rsp + 0xd0]
140027534: movaps xmm12, xmmword ptr [rsp + 0xc0]
14002753d: movaps xmm13, xmmword ptr [rsp + 0xb0]
140027546: movaps xmm14, xmmword ptr [rsp + 0xa0]
14002754f: movaps xmm15, xmmword ptr [rsp + 0x90]
140027558: add rsp, 0x138
14002755f: pop rbx
140027560: pop rdi
140027561: pop rsi
140027562: pop r12
140027564: pop r13
140027566: pop r14
140027568: pop r15
14002756a: pop rbp
14002756b: ret 
14002756c: test rax, rax
14002756f: jne 0x14002758d
140027571: mov rcx, rsi
140027574: mov rsi, rdx
140027577: xor edx, edx
140027579: call 0x1428a1640 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
14002757e: mov rdx, rsi
140027581: mov rsi, rax
140027584: test rax, rax
140027587: jne 0x1400274e3
14002758d: lea rcx, [rip + 0x290b47c]
140027594: call 0x1428d80d0 ; std::thread::local::panic_access_error
140027599: lea rcx, [rip + 0x290b518]
1400275a0: call 0x1428d9250 ; core::cell::panic_already_borrowed
1400275a5: int3 
