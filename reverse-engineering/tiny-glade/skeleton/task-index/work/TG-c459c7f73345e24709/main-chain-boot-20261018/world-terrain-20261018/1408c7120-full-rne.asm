1408c7120: push rbp
1408c7121: push r15
1408c7123: push r14
1408c7125: push r13
1408c7127: push r12
1408c7129: push rsi
1408c712a: push rdi
1408c712b: push rbx
1408c712c: sub rsp, 0x5c8
1408c7133: lea rbp, [rsp + 0x80]
1408c713b: movaps xmmword ptr [rbp + 0x530], xmm6
1408c7142: mov qword ptr [rbp + 0x528], 0xfffffffffffffffe
1408c714d: mov rsi, rdx
1408c7150: mov r15, rcx
1408c7153: lea rbx, [rdx + 0x38]
1408c7157: movabs rdx, 0x47c94cb78283a72e
1408c7161: movabs r8, 0x8c173ded97323e9d
1408c716b: mov rcx, rbx
1408c716e: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1408c7173: test rax, rax
1408c7176: je 0x1408c8aeb
1408c717c: cmp qword ptr [rsi + 0x218], rdx
1408c7183: jbe 0x1408c8aeb
1408c7189: mov rax, qword ptr [rsi + 0x210]
1408c7190: mov rcx, qword ptr [rax + rdx*8]
1408c7194: test rcx, rcx
1408c7197: je 0x1408c8aeb
1408c719d: mov rax, qword ptr [rsi + 0x1e0]
1408c71a4: not rcx
1408c71a7: lea rcx, [rcx + rcx*2]
1408c71ab: shl rcx, 5
1408c71af: cmp qword ptr [rax + rcx + 0x20], 0
1408c71b5: je 0x1408c8aeb
1408c71bb: add rax, rcx
1408c71be: mov r12, qword ptr [rax + 0x10]
1408c71c2: movabs rdx, 0x1e38c16726d0fb8d
1408c71cc: movabs r8, 0xec8924b33809588b
1408c71d6: mov rcx, rbx
1408c71d9: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1408c71de: test rax, rax
1408c71e1: je 0x1408c8af7
1408c71e7: cmp qword ptr [rsi + 0x218], rdx
1408c71ee: jbe 0x1408c8af7
1408c71f4: mov rax, qword ptr [rsi + 0x210]
1408c71fb: mov rcx, qword ptr [rax + rdx*8]
1408c71ff: test rcx, rcx
1408c7202: je 0x1408c8af7
1408c7208: mov rax, qword ptr [rsi + 0x1e0]
1408c720f: not rcx
1408c7212: lea rcx, [rcx + rcx*2]
1408c7216: shl rcx, 5
1408c721a: cmp qword ptr [rax + rcx + 0x20], 0
1408c7220: je 0x1408c8af7
1408c7226: add rax, rcx
1408c7229: mov rcx, qword ptr [rax + 0x10]
1408c722d: mov qword ptr [rbp + 0x4c8], rcx
1408c7234: mov ecx, dword ptr [rsi + 0x5b0]
1408c723a: mov dword ptr [rax + 0x5c], ecx
1408c723d: movabs rdx, 0xcda6af2f6afb130e
1408c7247: movabs r8, 0x36e88a927bd70aa
1408c7251: mov rcx, rbx
1408c7254: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1408c7259: test rax, rax
1408c725c: je 0x1408c8b03
1408c7262: cmp qword ptr [rsi + 0x218], rdx
1408c7269: jbe 0x1408c8b03
1408c726f: mov rax, qword ptr [rsi + 0x210]
1408c7276: mov rcx, qword ptr [rax + rdx*8]
1408c727a: test rcx, rcx
1408c727d: je 0x1408c8b03
1408c7283: mov rax, qword ptr [rsi + 0x1e0]
1408c728a: not rcx
1408c728d: lea rcx, [rcx + rcx*2]
1408c7291: shl rcx, 5
1408c7295: cmp qword ptr [rax + rcx + 0x20], 0
1408c729b: je 0x1408c8b03
1408c72a1: add rax, rcx
1408c72a4: mov rax, qword ptr [rax + 0x10]
1408c72a8: mov rcx, qword ptr [rax + 0x30]
1408c72ac: or rcx, qword ptr [rax + 0x10]
1408c72b0: jne 0x1408c72c3
1408c72b2: mov rax, qword ptr [rbp + 0x4c8]
1408c72b9: cmp byte ptr [rax + 0x40], 1
1408c72bd: je 0x1408c8988
1408c72c3: mov qword ptr [rbp + 0x4d8], rsi
1408c72ca: mov qword ptr [rbp + 0x4f8], 0
1408c72d5: mov qword ptr [rbp + 0x500], 8
1408c72e0: mov qword ptr [rbp + 0x508], 0
1408c72eb: mov qword ptr [rbp + 0x4e0], 0
1408c72f6: mov qword ptr [rbp + 0x4e8], 8
1408c7301: mov qword ptr [rbp + 0x4f0], 0
1408c730c: mov byte ptr [rbp + 0x527], 1
1408c7313: call 0x140c16a00 ; puffin::are_scopes_on
1408c7318: test al, al
1408c731a: je 0x1408c73b5
1408c7320: lea r8, [rip + 0x21dbbe0]
1408c7327: lea rbx, [rip + 0x21dbb83]
1408c732e: nop 
1408c7330: movsx eax, byte ptr [r8 - 1]
1408c7335: test eax, eax
1408c7337: js 0x1408c7350
1408c7339: dec r8
1408c733c: cmp eax, 0x5c
1408c733f: jne 0x1408c73a4
1408c7341: jmp 0x1408c73bc
1408c7343: nop word ptr cs:[rax + rax]
1408c7350: movzx ecx, byte ptr [r8 - 2]
1408c7355: cmp cl, 0xc0
1408c7358: jge 0x1408c737d
1408c735a: movzx edx, byte ptr [r8 - 3]
1408c735f: cmp dl, 0xc0
1408c7362: jge 0x1408c7386
1408c7364: movzx r9d, byte ptr [r8 - 4]
1408c7369: add r8, -4
1408c736d: and r9d, 7
1408c7371: shl r9d, 6
1408c7375: and edx, 0x3f
1408c7378: or edx, r9d
1408c737b: jmp 0x1408c738d
1408c737d: add r8, -2
1408c7381: and ecx, 0x1f
1408c7384: jmp 0x1408c7395
1408c7386: add r8, -3
1408c738a: and edx, 0xf
1408c738d: shl edx, 6
1408c7390: and ecx, 0x3f
1408c7393: or ecx, edx
1408c7395: shl ecx, 6
1408c7398: and al, 0x3f
1408c739a: movzx eax, al
1408c739d: or eax, ecx
1408c739f: cmp eax, 0x5c
1408c73a2: je 0x1408c73bc
1408c73a4: cmp eax, 0x2f
1408c73a7: je 0x1408c73bc
1408c73a9: cmp r8, rbx
1408c73ac: jne 0x1408c7330
1408c73ae: mov esi, 0x56
1408c73b3: jmp 0x1408c7414
1408c73b5: xor eax, eax
1408c73b7: jmp 0x1408c74ab
1408c73bc: lea rbx, [rip + 0x21dbaee]
1408c73c3: sub r8, rbx
1408c73c6: inc r8
1408c73c9: je 0x1408c7409
1408c73cb: cmp r8, 0x56
1408c73cf: jb 0x1408c7402
1408c73d1: je 0x1408c7409
1408c73d3: mov byte ptr [rbp + 0x527], 1
1408c73da: lea rax, [rip + 0x21db9d7]
1408c73e1: mov qword ptr [rsp + 0x20], rax
1408c73e6: lea rcx, [rip + 0x21dbac4]
1408c73ed: mov edx, 0x56
1408c73f2: mov r9d, 0x56
1408c73f8: call 0x1428d9940 ; core::str::slice_error_fail
1408c73fd: jmp 0x1408c8b97
1408c7402: cmp byte ptr [r8 + rbx], 0xbf
1408c7407: jle 0x1408c73d3
1408c7409: mov esi, 0x56
1408c740e: sub rsi, r8
1408c7411: add rbx, r8
1408c7414: call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
1408c7419: mov rcx, rax
1408c741c: mov rax, qword ptr [rax]
1408c741f: test rax, rax
1408c7422: je 0x1408c7433
1408c7424: cmp eax, 1
1408c7427: jne 0x1408c8b24
1408c742d: add rcx, 8
1408c7431: jmp 0x1408c744d
1408c7433: mov byte ptr [rbp + 0x527], 1
1408c743a: xor edx, edx
1408c743c: call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
1408c7441: mov rcx, rax
1408c7444: test rax, rax
1408c7447: je 0x1408c8b24
1408c744d: cmp qword ptr [rcx], 0
1408c7451: jne 0x1408c8a1c
1408c7457: mov qword ptr [rcx], 0xffffffffffffffff
1408c745e: mov qword ptr [rbp + 0x510], rcx
1408c7465: add rcx, 8
1408c7469: mov qword ptr [rsp + 0x20], rsi
1408c746e: mov qword ptr [rsp + 0x30], 0
1408c7477: mov qword ptr [rsp + 0x28], 1
1408c7480: lea rdx, [rip + 0x21dbac9]
1408c7487: mov r8d, 0x16
1408c748d: mov r9, rbx
1408c7490: call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
1408c7495: mov rcx, qword ptr [rbp + 0x510]
1408c749c: inc qword ptr [rcx]
1408c749f: mov qword ptr [rbp + 0x4c0], rax
1408c74a6: mov eax, 1
1408c74ab: mov qword ptr [rbp + 0x4b8], rax
1408c74b2: mov rax, qword ptr [rbp + 0x4c8]
1408c74b9: mov r13, qword ptr [rax + 8]
1408c74bd: mov rbx, qword ptr [rax + 0x20]
1408c74c1: movdqa xmm0, xmmword ptr [r13]
1408c74c7: pmovmskb eax, xmm0
1408c74cb: not eax
1408c74cd: lea rsi, [r13 + 0x10]
1408c74d1: lea rcx, [r12 + 8]
1408c74d6: mov qword ptr [rbp + 0x188], rcx
1408c74dd: movss xmm6, dword ptr [rip + 0x209918f]
1408c74e5: mov qword ptr [rbp + 0x158], r15
1408c74ec: mov qword ptr [rbp + 0x518], r12
1408c74f3: jmp 0x1408c7549
1408c74f5: nop word ptr cs:[rax + rax]
1408c7500: mov rax, qword ptr [rbp + 0x500]
1408c7507: mov rcx, rdi
1408c750a: shl rcx, 5
1408c750e: movdqu xmm0, xmmword ptr [rbp + 0x160]
1408c7516: movups xmm1, xmmword ptr [rbp + 0x170]
1408c751d: movups xmmword ptr [rax + rcx + 0x10], xmm1
1408c7522: movdqu xmmword ptr [rax + rcx], xmm0
1408c7527: inc rdi
1408c752a: mov qword ptr [rbp + 0x508], rdi
1408c7531: mov eax, r14d
1408c7534: mov r15, qword ptr [rbp + 0x158]
1408c753b: mov r12, qword ptr [rbp + 0x518]
1408c7542: mov rbx, qword ptr [rbp + 0x4d0]
1408c7549: test rbx, rbx
1408c754c: je 0x1408c7baf
1408c7552: test ax, ax
1408c7555: je 0x1408c7560
1408c7557: lea r14d, [rax - 1]
1408c755b: and r14d, eax
1408c755e: jmp 0x1408c7589
1408c7560: movdqa xmm0, xmmword ptr [rsi]
1408c7564: pmovmskb eax, xmm0
1408c7568: add r13, -0x300
1408c756f: add rsi, 0x10
1408c7573: cmp eax, 0xffff
1408c7578: je 0x1408c7560
1408c757a: mov ecx, 0xfffffffe
1408c757f: sub ecx, eax
1408c7581: not eax
1408c7583: mov r14d, eax
1408c7586: and r14d, ecx
1408c7589: dec rbx
1408c758c: tzcnt eax, eax
1408c7590: neg rax
1408c7593: lea rcx, [rax + rax*2]
1408c7597: shl rcx, 4
1408c759b: ucomiss xmm6, dword ptr [r13 + rcx - 8]
1408c75a1: mov eax, r14d
1408c75a4: jae 0x1408c7549
1408c75a6: lea rdi, [rcx + r13]
1408c75aa: cmp qword ptr [rdi - 0x18], 0
1408c75af: mov eax, r14d
1408c75b2: je 0x1408c7549
1408c75b4: cmp qword ptr [rbp + 0x4f0], 0x3fff
1408c75bf: ja 0x1408c7939
1408c75c5: mov eax, dword ptr [r13 + rcx - 0x30]
1408c75ca: mov dword ptr [rbp + 0x510], eax
1408c75d0: xor eax, eax
1408c75d2: mov cl, 1
1408c75d4: lock cmpxchg byte ptr [r12], cl
1408c75da: jne 0x1408c790c
1408c75e0: mov rcx, r12
1408c75e3: call 0x140c81c70 ; parking_lot_core::parking_lot::deadlock_impl::acquire_resource
1408c75e8: mov qword ptr [rbp + 0x4d0], rbx
1408c75ef: mov r8, qword ptr [r15 + 0x190]
1408c75f6: mov r9, qword ptr [rdi - 0x18]
1408c75fa: inc r9
1408c75fd: movabs rax, 0x1ffffffffffffffe
1408c7607: and r9, rax
1408c760a: cmp r9, 0x400
1408c7611: mov eax, 0x400
1408c7616: cmovae r9, rax
1408c761a: shl r9d, 3
1408c761e: mov qword ptr [rsp + 0x28], 0x10
1408c7627: mov qword ptr [rsp + 0x20], 1
1408c7630: lea rcx, [rbp + 0x190]
1408c7637: mov rdx, qword ptr [rbp + 0x188]
1408c763e: call 0x140c02740 ; rhapsody::raw::dynamic_constants::DynamicConstants::allocate
1408c7643: mov byte ptr [rbp + 0x526], 1
1408c764a: mov rcx, qword ptr [rbp + 0x518]
1408c7651: call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
1408c7656: mov al, 1
1408c7658: mov rcx, qword ptr [rbp + 0x518]
1408c765f: xor edx, edx
1408c7661: lock cmpxchg byte ptr [rcx], dl
1408c7665: jne 0x1408c791f
1408c766b: mov rax, qword ptr [rbp + 0x190]
1408c7672: test al, 3
1408c7674: jne 0x1408c8ac6
1408c767a: mov rcx, qword ptr [rbp + 0x198]
1408c7681: test cl, 7
1408c7684: jne 0x1408c8acb
1408c768a: mov rdx, qword ptr [rdi - 0x18]
1408c768e: shr rcx, 3
1408c7692: cmp rdx, rcx
1408c7695: cmovb rcx, rdx
1408c7699: test rcx, rcx
1408c769c: je 0x1408c77da
1408c76a2: mov rdx, qword ptr [rdi - 0x20]
1408c76a6: cmp rcx, 6
1408c76aa: jb 0x1408c76ca
1408c76ac: lea r8, [rcx + rcx*2]
1408c76b0: lea r8, [rdx + r8*4]
1408c76b4: cmp rax, r8
1408c76b7: jae 0x1408c7753
1408c76bd: lea r8, [rax + rcx*8]
1408c76c1: cmp rdx, r8
1408c76c4: jae 0x1408c7753
1408c76ca: xor r8d, r8d
1408c76cd: mov r9, r8
1408c76d0: or r9, 1
1408c76d4: test cl, 1
1408c76d7: je 0x1408c76fa
1408c76d9: lea r10, [r8 + r8*2]
1408c76dd: movd xmm0, dword ptr [rdx + r10*4]
1408c76e3: movss xmm1, dword ptr [rdx + r10*4 + 8]
1408c76ea: movd dword ptr [rax + r8*8], xmm0
1408c76f0: movss dword ptr [rax + r8*8 + 4], xmm1
1408c76f7: mov r8, r9
1408c76fa: cmp rcx, r9
1408c76fd: je 0x1408c77d6
1408c7703: lea r9, [r8 + r8*2]
1408c7707: lea rdx, [rdx + r9*4]
1408c770b: add rdx, 0x14
1408c770f: nop 
1408c7710: movss xmm0, dword ptr [rdx - 0x14]
1408c7715: movss xmm1, dword ptr [rdx - 0xc]
1408c771a: movss dword ptr [rax + r8*8], xmm0
1408c7720: movss dword ptr [rax + r8*8 + 4], xmm1
1408c7727: movd xmm0, dword ptr [rdx - 8]
1408c772c: movss xmm1, dword ptr [rdx]
1408c7730: movd dword ptr [rax + r8*8 + 8], xmm0
1408c7737: movss dword ptr [rax + r8*8 + 0xc], xmm1
1408c773e: lea r9, [r8 + 2]
1408c7742: add rdx, 0x18
1408c7746: mov r8, r9
1408c7749: cmp rcx, r9
1408c774c: jne 0x1408c7710
1408c774e: jmp 0x1408c77d6
1408c7753: mov r8, rcx
1408c7756: movabs r9, 0x1ffffffffffffffc
1408c7760: and r8, r9
1408c7763: lea r9, [rdx + 0x2c]
1408c7767: xor r10d, r10d
1408c776a: nop word ptr [rax + rax]
1408c7770: movd xmm0, dword ptr [r9 - 0x2c]
1408c7776: movd xmm1, dword ptr [r9 - 0x24]
1408c777c: punpckldq xmm0, xmm1
1408c7780: movd xmm1, dword ptr [r9 - 0x20]
1408c7786: movd xmm2, dword ptr [r9 - 0x18]
1408c778c: punpckldq xmm1, xmm2
1408c7790: punpcklqdq xmm0, xmm1
1408c7794: movss xmm1, dword ptr [r9 - 0xc]
1408c779a: movss xmm2, dword ptr [r9 - 0x14]
1408c77a0: unpcklps xmm2, xmm1
1408c77a3: movss xmm1, dword ptr [r9]
1408c77a8: movss xmm3, dword ptr [r9 - 8]
1408c77ae: unpcklps xmm3, xmm1
1408c77b1: movlhps xmm2, xmm3
1408c77b4: movdqu xmmword ptr [rax + r10*8], xmm0
1408c77ba: movups xmmword ptr [rax + r10*8 + 0x10], xmm2
1408c77c0: add r10, 4
1408c77c4: add r9, 0x30
1408c77c8: cmp r8, r10
1408c77cb: jne 0x1408c7770
1408c77cd: cmp rcx, r8
1408c77d0: jne 0x1408c76cd
1408c77d6: mov rdx, qword ptr [rdi - 0x18]
1408c77da: cmp rdx, 0x400
1408c77e1: mov ebx, 0x400
1408c77e6: cmovb rbx, rdx
1408c77ea: test rdx, rdx
1408c77ed: je 0x1408c89a3
1408c77f3: mov byte ptr [rbp + 0x526], 1
1408c77fa: lea rcx, [rbp + 0x190]
1408c7801: call 0x140c02280 ; rhapsody::raw::dynamic_constants::DynamicConstantsChunk::device_address
1408c7806: mov r12, rax
1408c7809: movss xmm0, dword ptr [rdi - 0x10]
1408c780e: movss xmm1, dword ptr [rdi - 8]
1408c7813: mov byte ptr [rbp + 0x526], 1
1408c781a: call 0x140c90df0 ; utils::pack_2x16f_uint
1408c781f: mov edi, eax
1408c7821: mov r15, qword ptr [rbp + 0x4f0]
1408c7828: cmp r15, qword ptr [rbp + 0x4e0]
1408c782f: jne 0x1408c784b
1408c7831: mov byte ptr [rbp + 0x526], 1
1408c7838: lea rcx, [rbp + 0x4e0]
1408c783f: lea rdx, [rip + 0x21db762]
1408c7846: call 0x140562630 ; alloc::raw_vec::RawVec::grow_one<tuple$<u64,ref$<system_clutter::autoclutter::AutoClutterObject> >,alloc::alloc::Global>
1408c784b: and ebx, 0x3ff
1408c7851: mov edx, dword ptr [rbp + 0x510]
1408c7857: shl edx, 0xa
1408c785a: or edx, ebx
1408c785c: mov rax, qword ptr [rbp + 0x4e8]
1408c7863: mov rcx, r15
1408c7866: shl rcx, 4
1408c786a: mov qword ptr [rax + rcx], r12
1408c786e: mov dword ptr [rax + rcx + 8], edx
1408c7872: mov dword ptr [rax + rcx + 0xc], edi
1408c7876: inc r15
1408c7879: mov qword ptr [rbp + 0x4f0], r15
1408c7880: mov rax, qword ptr [rbp + 0x1d0]
1408c7887: mov qword ptr [rbp + 0x360], rax
1408c788e: movups xmm0, xmmword ptr [rbp + 0x190]
1408c7895: movups xmm1, xmmword ptr [rbp + 0x1a0]
1408c789c: movups xmm2, xmmword ptr [rbp + 0x1b0]
1408c78a3: movups xmm3, xmmword ptr [rbp + 0x1c0]
1408c78aa: movaps xmmword ptr [rbp + 0x350], xmm3
1408c78b1: movaps xmmword ptr [rbp + 0x340], xmm2
1408c78b8: movaps xmmword ptr [rbp + 0x330], xmm1
1408c78bf: movaps xmmword ptr [rbp + 0x320], xmm0
1408c78c6: mov byte ptr [rbp + 0x526], 0
1408c78cd: lea rcx, [rbp + 0x160]
1408c78d4: lea rdx, [rbp + 0x320]
1408c78db: call 0x140c02290 ; rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_gpu_lifetime_tracker
1408c78e0: mov rdi, qword ptr [rbp + 0x508]
1408c78e7: cmp rdi, qword ptr [rbp + 0x4f8]
1408c78ee: jne 0x1408c7500
1408c78f4: lea rcx, [rbp + 0x4f8]
1408c78fb: lea rdx, [rip + 0x21db6be]
1408c7902: call 0x140562c30 ; alloc::raw_vec::RawVec::grow_one<tuple$<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global> >,alloc::alloc::Global>
1408c7907: jmp 0x1408c7500
1408c790c: mov rcx, r12
1408c790f: mov r8d, 0x3b9aca00
1408c7915: call 0x1428cd210 ; parking_lot::raw_mutex::RawMutex::lock_slow
1408c791a: jmp 0x1408c75e0
1408c791f: mov byte ptr [rbp + 0x526], 1
1408c7926: mov rcx, qword ptr [rbp + 0x518]
1408c792d: xor edx, edx
1408c792f: call 0x1428cdb90 ; parking_lot::raw_mutex::RawMutex::unlock_slow
1408c7934: jmp 0x1408c766b
1408c7939: mov eax, dword ptr [rip + 0x2d6d379]
1408c793f: cmp eax, 3
1408c7942: jne 0x1408c8a34
1408c7948: mov rdx, qword ptr [rip + 0x2d6d361]
1408c794f: lea rcx, [rbp + 0x320]
1408c7956: call 0x140542890 ; std::sync::poison::mutex::Mutex::lock<winit::dpi::PhysicalSize<u32> >
1408c795b: cmp dword ptr [rbp + 0x320], 1
1408c7962: je 0x1408c8b39
1408c7968: mov rax, qword ptr [rbp + 0x328]
1408c796f: mov qword ptr [rbp + 0x510], rax
1408c7976: movzx eax, byte ptr [rbp + 0x330]
1408c797d: mov byte ptr [rbp + 0x4d0], al
1408c7983: movzx eax, byte ptr [rip + 0x2d6d27f]
1408c798a: mov ecx, 0xf
1408c798f: mov edx, 1
1408c7994: call 0x140613c10 ; ?
1408c7999: test rax, rax
1408c799c: je 0x1408c8b81
1408c79a2: movabs rcx, 0x2928202168746170
1408c79ac: mov qword ptr [rax + 7], rcx
1408c79b0: movabs rcx, 0x705f656c75646f6d
1408c79ba: mov qword ptr [rax], rcx
1408c79bd: mov qword ptr [rbp + 0x320], 0xf
1408c79c8: mov qword ptr [rbp + 0x328], rax
1408c79cf: mov qword ptr [rbp + 0x330], 0xf
1408c79da: mov qword ptr [rsp + 0x20], 1
1408c79e3: lea rcx, [rbp + 0x320]
1408c79ea: mov edx, 0xf
1408c79ef: mov r8d, 0x18
1408c79f5: mov r9d, 1
1408c79fb: call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1408c7a00: mov rax, qword ptr [rbp + 0x328]
1408c7a07: mov rdx, qword ptr [rbp + 0x330]
1408c7a0e: movups xmm0, xmmword ptr [rip + 0x21db5ee]
1408c7a15: movups xmmword ptr [rax + rdx], xmm0
1408c7a19: movabs rcx, 0x726f727245203a3a
1408c7a23: mov qword ptr [rax + rdx + 0x10], rcx
1408c7a28: add rdx, 0x18
1408c7a2c: mov qword ptr [rbp + 0x1a0], rdx
1408c7a33: movdqu xmm0, xmmword ptr [rbp + 0x320]
1408c7a3b: movdqa xmmword ptr [rbp + 0x190], xmm0
1408c7a43: mov rax, qword ptr [rbp + 0x190]
1408c7a4a: sub rax, rdx
1408c7a4d: cmp rax, 0x1e
1408c7a51: jbe 0x1408c8a76
1408c7a57: mov rcx, qword ptr [rbp + 0x510]
1408c7a5e: mov rax, qword ptr [rbp + 0x198]
1408c7a65: movups xmm0, xmmword ptr [rip + 0x21db5be]
1408c7a6c: movups xmmword ptr [rax + rdx + 0xf], xmm0
1408c7a71: movups xmm0, xmmword ptr [rip + 0x21db5a3]
1408c7a78: movups xmmword ptr [rax + rdx], xmm0
1408c7a7c: add rdx, 0x1f
1408c7a80: mov qword ptr [rbp + 0x1a0], rdx
1408c7a87: mov qword ptr [rbp + 0x330], rdx
1408c7a8e: movdqa xmm0, xmmword ptr [rbp + 0x190]
1408c7a96: movdqa xmmword ptr [rbp + 0x320], xmm0
1408c7a9e: add rcx, 8
1408c7aa2: lea rdx, [rbp + 0x320]
1408c7aa9: call 0x14143b8b0 ; alloc::collections::btree::map::BTreeMap::insert<alloc::string::String,alloc::collections::btree::set_val::SetValZST,alloc::alloc::Global>
1408c7aae: test al, al
1408c7ab0: jne 0x1408c7b7d
1408c7ab6: mov rax, qword ptr [rip + 0x2d648eb]
1408c7abd: mov rax, qword ptr [rax]
1408c7ac0: test rax, rax
1408c7ac3: je 0x1408c7b7d
1408c7ac9: lea rcx, [rip + 0x21db508]
1408c7ad0: call 0x14067dd80 ; utils::swizzlers::impl$5::as_vec2
1408c7ad5: movdqu xmm0, xmmword ptr [rax]
1408c7ad9: mov eax, dword ptr [rax + 0x10]
1408c7adc: mov qword ptr [rbp + 0x350], 1
1408c7ae7: lea r8, [rip + 0x21db562]
1408c7aee: mov qword ptr [rbp + 0x358], r8
1408c7af5: mov qword ptr [rbp + 0x360], 0x4c
1408c7b00: lea rdx, [rip + 0x21db539]
1408c7b07: mov qword ptr [rbp + 0x370], rdx
1408c7b0e: mov qword ptr [rbp + 0x378], 1
1408c7b19: lea rcx, [rbp + 0x190]
1408c7b20: mov qword ptr [rbp + 0x380], rcx
1408c7b27: mov qword ptr [rbp + 0x320], 0
1408c7b32: xorps xmm1, xmm1
1408c7b35: movups xmmword ptr [rbp + 0x388], xmm1
1408c7b3c: mov qword ptr [rbp + 0x328], r8
1408c7b43: mov qword ptr [rbp + 0x330], 0x4c
1408c7b4e: mov qword ptr [rbp + 0x338], 0
1408c7b59: movdqu xmmword ptr [rbp + 0x340], xmm0
1408c7b61: mov dword ptr [rbp + 0x368], 1
1408c7b6b: mov dword ptr [rbp + 0x36c], eax
1408c7b71: lea rdx, [rbp + 0x320]
1408c7b78: call 0x140d4a330 ; log::__private_api::impl$2::log
1408c7b7d: cmp byte ptr [rbp + 0x4d0], 0
1408c7b84: mov rsi, qword ptr [rbp + 0x510]
1408c7b8b: jne 0x1408c7ba3
1408c7b8d: mov rax, qword ptr [rip + 0x2d6488c]
1408c7b94: mov rax, qword ptr [rax]
1408c7b97: shl rax, 1
1408c7b9a: test rax, rax
1408c7b9d: jne 0x1408c8ab0
1408c7ba3: xor eax, eax
1408c7ba5: xchg byte ptr [rsi], al
1408c7ba7: cmp al, 2
1408c7ba9: je 0x1408c8aa3
1408c7baf: mov cl, 1
1408c7bb1: xor eax, eax
1408c7bb3: lock cmpxchg byte ptr [r12], cl
1408c7bb9: jne 0x1408c89c7
1408c7bbf: mov rcx, r12
1408c7bc2: call 0x140c81c70 ; parking_lot_core::parking_lot::deadlock_impl::acquire_resource
1408c7bc7: mov r8, qword ptr [r15 + 0x190]
1408c7bce: mov r14, qword ptr [rbp + 0x4e8]
1408c7bd5: mov rdi, qword ptr [rbp + 0x4f0]
1408c7bdc: shl rdi, 4
1408c7be0: mov qword ptr [rsp + 0x20], 0
1408c7be9: lea rcx, [rbp + 0x320]
1408c7bf0: mov rdx, qword ptr [rbp + 0x188]
1408c7bf7: mov r9, rdi
1408c7bfa: call 0x140c02740 ; rhapsody::raw::dynamic_constants::DynamicConstants::allocate
1408c7bff: mov rcx, qword ptr [rbp + 0x328]
1408c7c06: cmp rcx, rdi
1408c7c09: jne 0x1408c89da
1408c7c0f: mov rcx, qword ptr [rbp + 0x320]
1408c7c16: mov rdx, r14
1408c7c19: mov r8, rdi
1408c7c1c: call 0x142923c00 ; ?
1408c7c21: movdqu xmm0, xmmword ptr [rbp + 0x320]
1408c7c29: movups xmm1, xmmword ptr [rbp + 0x330]
1408c7c30: movups xmm2, xmmword ptr [rbp + 0x340]
1408c7c37: movups xmm3, xmmword ptr [rbp + 0x350]
1408c7c3e: movdqa xmmword ptr [rbp + 0x190], xmm0
1408c7c46: movaps xmmword ptr [rbp + 0x1a0], xmm1
1408c7c4d: movaps xmmword ptr [rbp + 0x1b0], xmm2
1408c7c54: movaps xmmword ptr [rbp + 0x1c0], xmm3
1408c7c5b: mov rax, qword ptr [rbp + 0x360]
1408c7c62: mov qword ptr [rbp + 0x1d0], rax
1408c7c69: mov byte ptr [rbp + 0x525], 1
1408c7c70: mov rsi, qword ptr [rbp + 0x518]
1408c7c77: mov rcx, rsi
1408c7c7a: call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
1408c7c7f: xor ecx, ecx
1408c7c81: mov al, 1
1408c7c83: lock cmpxchg byte ptr [rsi], cl
1408c7c87: jne 0x1408c89ee
1408c7c8d: mov byte ptr [rbp + 0x525], 1
1408c7c94: lea rcx, [rbp + 0x190]
1408c7c9b: call 0x140c02280 ; rhapsody::raw::dynamic_constants::DynamicConstantsChunk::device_address
1408c7ca0: mov r14, rax
1408c7ca3: mov rax, qword ptr [rbp + 0x1d0]
1408c7caa: mov qword ptr [rbp + 0x360], rax
1408c7cb1: movdqa xmm0, xmmword ptr [rbp + 0x190]
1408c7cb9: movaps xmm1, xmmword ptr [rbp + 0x1a0]
1408c7cc0: movaps xmm2, xmmword ptr [rbp + 0x1b0]
1408c7cc7: movaps xmm3, xmmword ptr [rbp + 0x1c0]
1408c7cce: movaps xmmword ptr [rbp + 0x350], xmm3
1408c7cd5: movaps xmmword ptr [rbp + 0x340], xmm2
1408c7cdc: movaps xmmword ptr [rbp + 0x330], xmm1
1408c7ce3: movdqa xmmword ptr [rbp + 0x320], xmm0
1408c7ceb: mov byte ptr [rbp + 0x525], 0
1408c7cf2: lea rbx, [rbp + 0x160]
1408c7cf9: lea rdx, [rbp + 0x320]
1408c7d00: mov rcx, rbx
1408c7d03: call 0x140c02290 ; rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_gpu_lifetime_tracker
1408c7d08: mov rsi, qword ptr [rbp + 0x508]
1408c7d0f: cmp rsi, qword ptr [rbp + 0x4f8]
1408c7d16: jne 0x1408c7d2b
1408c7d18: lea rdx, [rip + 0x21db381]
1408c7d1f: lea rcx, [rbp + 0x4f8]
1408c7d26: call 0x140562c30 ; alloc::raw_vec::RawVec::grow_one<tuple$<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global> >,alloc::alloc::Global>
1408c7d2b: mov rax, qword ptr [rbp + 0x500]
1408c7d32: mov rcx, rsi
1408c7d35: shl rcx, 5
1408c7d39: movdqu xmm0, xmmword ptr [rbp + 0x160]
1408c7d41: movups xmm1, xmmword ptr [rbp + 0x170]
1408c7d48: movups xmmword ptr [rax + rcx + 0x10], xmm1
1408c7d4d: movdqu xmmword ptr [rax + rcx], xmm0
1408c7d52: inc rsi
1408c7d55: mov qword ptr [rbp + 0x508], rsi
1408c7d5c: cmp qword ptr [rbp + 0x4b8], 0
1408c7d64: je 0x1408c7dd1
1408c7d66: call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
1408c7d6b: mov rcx, rax
1408c7d6e: mov rax, qword ptr [rax]
1408c7d71: test rax, rax
1408c7d74: je 0x1408c7d85
1408c7d76: cmp eax, 1
1408c7d79: jne 0x1408c8b0f
1408c7d7f: add rcx, 8
1408c7d83: jmp 0x1408c7d9f
1408c7d85: mov byte ptr [rbp + 0x527], 1
1408c7d8c: xor edx, edx
1408c7d8e: call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
1408c7d93: mov rcx, rax
1408c7d96: test rax, rax
1408c7d99: je 0x1408c8b0f
1408c7d9f: cmp qword ptr [rcx], 0
1408c7da3: jne 0x1408c8a04
1408c7da9: mov qword ptr [rcx], 0xffffffffffffffff
1408c7db0: mov qword ptr [rbp + 0x518], rcx
1408c7db7: add rcx, 8
1408c7dbb: mov rdx, qword ptr [rbp + 0x4c0]
1408c7dc2: call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
1408c7dc7: mov rax, qword ptr [rbp + 0x518]
1408c7dce: inc qword ptr [rax]
1408c7dd1: mov byte ptr [rbp + 0x527], 1
1408c7dd8: mov qword ptr [rsp + 0x30], r15
1408c7ddd: mov rax, qword ptr [rbp + 0x4d8]
1408c7de4: mov qword ptr [rsp + 0x28], rax
1408c7de9: mov qword ptr [rsp + 0x20], 0x31
1408c7df2: lea rdx, [rip + 0x21db2bf]
1408c7df9: lea r9, [rip + 0x21db2d0]
1408c7e00: lea rcx, [rbp + 0x320]
1408c7e07: mov r8d, 0x18
1408c7e0d: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
1408c7e12: cmp byte ptr [rbp + 0x456], 8
1408c7e19: je 0x1408c7e44
1408c7e1b: mov r9, qword ptr [rbp + 0x4f0]
1408c7e22: lea rcx, [rbp + 0x430]
1408c7e29: mov edx, 0xe8
1408c7e2e: add rdx, qword ptr [rbp + 0x458]
1408c7e35: movabs r8, 0x5efaa1884a48985d
1408c7e3f: call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
1408c7e44: lea rcx, [rbp + 0x190]
1408c7e4b: lea rdx, [rbp + 0x320]
1408c7e52: mov r8d, 0x190
1408c7e58: call 0x142923c00 ; ?
1408c7e5d: cmp byte ptr [rbp + 0x2c6], 8
1408c7e64: je 0x1408c7e8b
1408c7e66: lea rcx, [rbp + 0x2a0]
1408c7e6d: mov edx, 0xe8
1408c7e72: add rdx, qword ptr [rbp + 0x2c8]
1408c7e79: movabs r8, 0x497d31a6f59bd1e5
1408c7e83: mov r9, r14
1408c7e86: call 0x14096b760 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<array$<u32,2> >,array$<u32,2> >
1408c7e8b: lea rcx, [rbp + 0x320]
1408c7e92: lea rdx, [rbp + 0x190]
1408c7e99: mov r8d, 0x190
1408c7e9f: call 0x142923c00 ; ?
1408c7ea4: cmp byte ptr [rbp + 0x456], 8
1408c7eab: je 0x1408c7ed7
1408c7ead: lea rcx, [rbp + 0x430]
1408c7eb4: mov edx, 0xe8
1408c7eb9: add rdx, qword ptr [rbp + 0x458]
1408c7ec0: movabs r8, 0x8a5c96e47fe18f3a
1408c7eca: movss xmm3, dword ptr [rip + 0x205d9fe]
1408c7ed2: call 0x14096ba30 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<f32>,f32>
1408c7ed7: lea rdi, [rbp + 0x190]
1408c7ede: lea r14, [rbp + 0x320]
1408c7ee5: mov r8d, 0x190
1408c7eeb: mov rcx, rdi
1408c7eee: mov rdx, r14
1408c7ef1: call 0x142923c00 ; ?
1408c7ef6: mov byte ptr [rbp - 0x40], 9
1408c7efa: mov dword ptr [rbp + 0x320], 2
1408c7f04: mov qword ptr [rsp + 0x20], r14
1408c7f09: lea rcx, [rbp - 0x40]
1408c7f0d: movabs r8, 0xb981790202418bc8
1408c7f17: mov rdx, rdi
1408c7f1a: mov r9b, 0x32
1408c7f1d: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408c7f22: lea rdi, [rbp + 0x320]
1408c7f29: lea r14, [rbp + 0x190]
1408c7f30: mov r8d, 0x190
1408c7f36: mov rcx, rdi
1408c7f39: mov rdx, r14
1408c7f3c: call 0x142923c00 ; ?
1408c7f41: mov byte ptr [rbp - 0x40], 0xb
1408c7f45: mov dword ptr [rbp + 0x190], 2
1408c7f4f: mov qword ptr [rsp + 0x20], r14
1408c7f54: lea rcx, [rbp - 0x40]
1408c7f58: movabs r8, 0xe3659c0fbde7bf03
1408c7f62: mov rdx, rdi
1408c7f65: mov r9b, 0x32
1408c7f68: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408c7f6d: lea rdi, [rbp + 0x190]
1408c7f74: lea r14, [rbp + 0x320]
1408c7f7b: mov r8d, 0x190
1408c7f81: mov rcx, rdi
1408c7f84: mov rdx, r14
1408c7f87: call 0x142923c00 ; ?
1408c7f8c: mov byte ptr [rbp - 0x40], 0x13
1408c7f90: mov dword ptr [rbp + 0x320], 2
1408c7f9a: mov qword ptr [rsp + 0x20], r14
1408c7f9f: lea rcx, [rbp - 0x40]
1408c7fa3: movabs r8, 0x6ad63ccd18c8f041
1408c7fad: mov rdx, rdi
1408c7fb0: mov r9b, 0x19
1408c7fb3: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408c7fb8: lea rdi, [rbp + 0x320]
1408c7fbf: lea r14, [rbp + 0x190]
1408c7fc6: mov r8d, 0x190
1408c7fcc: mov rcx, rdi
1408c7fcf: mov rdx, r14
1408c7fd2: call 0x142923c00 ; ?
1408c7fd7: mov byte ptr [rbp - 0x40], 0xe
1408c7fdb: mov dword ptr [rbp + 0x190], 2
1408c7fe5: mov qword ptr [rsp + 0x20], r14
1408c7fea: lea rcx, [rbp - 0x40]
1408c7fee: movabs r8, 0x4ee098b38612d2b9
1408c7ff8: mov rdx, rdi
1408c7ffb: mov r9b, 0x32
1408c7ffe: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408c8003: lea rdi, [rbp + 0x190]
1408c800a: lea r14, [rbp + 0x320]
1408c8011: mov r8d, 0x190
1408c8017: mov rcx, rdi
1408c801a: mov rdx, r14
1408c801d: call 0x142923c00 ; ?
1408c8022: mov byte ptr [rbp - 0x40], 0xd
1408c8026: mov dword ptr [rbp + 0x320], 2
1408c8030: mov qword ptr [rsp + 0x20], r14
1408c8035: lea rcx, [rbp - 0x40]
1408c8039: movabs r8, 0xab24287344aba8c1
1408c8043: mov rdx, rdi
1408c8046: mov r9b, 0x32
1408c8049: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408c804e: lea rdi, [rbp + 0x320]
1408c8055: lea rdx, [rbp + 0x190]
1408c805c: mov r8d, 0x190
1408c8062: mov rcx, rdi
1408c8065: call 0x142923c00 ; ?
1408c806a: mov byte ptr [rbp + 0x527], 1
1408c8071: lea rcx, [rbp + 0x190]
1408c8078: movabs r8, 0xd6623460fa508d3e
1408c8082: mov rdx, rdi
1408c8085: call 0x141607210 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::render::normals_debug::NormalDebugSsbo>
1408c808a: mov rax, qword ptr [rbp + 0x508]
1408c8091: mov qword ptr [rbp + 0x330], rax
1408c8098: movdqu xmm0, xmmword ptr [rbp + 0x4f8]
1408c80a0: movdqa xmmword ptr [rbp + 0x320], xmm0
1408c80a8: mov byte ptr [rbp + 0x527], 0
1408c80af: lea rcx, [rbp + 0x190]
1408c80b6: lea rdx, [rbp + 0x320]
1408c80bd: call 0x141609e20 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_with<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke::closure_env$1>
1408c80c2: mov byte ptr [rbp + 0x527], 0
1408c80c9: lea rcx, [rbp - 0x40]
1408c80cd: movabs rdx, 0x20000000200
1408c80d7: mov r8d, 0x6d
1408c80dd: call 0x140bd52a0 ; rhapsody::raw::texture::TextureDesc::new_2d
1408c80e2: movdqu xmm0, xmmword ptr [rbp - 0x40]
1408c80e7: movups xmm1, xmmword ptr [rbp - 0x30]
1408c80eb: movaps xmmword ptr [rbp + 0x1a0], xmm1
1408c80f2: movdqa xmmword ptr [rbp + 0x190], xmm0
1408c80fa: lea rcx, [rbp + 0x330]
1408c8101: mov byte ptr [rbp + 0x527], 0
1408c8108: lea rdx, [rbp + 0x190]
1408c810f: call 0x140bd6ce0 ; rhapsody::rg::resource::impl$4::from
1408c8114: lea rax, [rip + 0x21dafe6]
1408c811b: mov qword ptr [rbp + 0x320], rax
1408c8122: mov qword ptr [rbp + 0x328], 9
1408c812d: mov byte ptr [rbp + 0x527], 0
1408c8134: lea rdx, [rbp + 0x320]
1408c813b: mov rcx, r15
1408c813e: call 0x140be94d0 ; rhapsody::rg::graph::RenderGraph::create_raw_resource
1408c8143: movdqu xmm0, xmmword ptr [rbp - 0x40]
1408c8148: movups xmm1, xmmword ptr [rbp - 0x30]
1408c814c: movdqu xmmword ptr [rbp + 0x164], xmm0
1408c8154: movups xmmword ptr [rbp + 0x174], xmm1
1408c815b: mov dword ptr [rbp + 0x160], eax
1408c8161: lea rax, [rip + 0x21dafa2]
1408c8168: mov qword ptr [rbp + 0x190], rax
1408c816f: mov qword ptr [rbp + 0x198], 7
1408c817a: mov byte ptr [rbp + 0x1a0], 1
1408c8181: mov byte ptr [rbp + 0x527], 0
1408c8188: mov qword ptr [rsp + 0x28], r15
1408c818d: mov rax, qword ptr [rbp + 0x4d8]
1408c8194: mov qword ptr [rsp + 0x20], rax
1408c8199: lea rdx, [rip + 0x21daf71]
1408c81a0: lea rcx, [rbp + 0x320]
1408c81a7: lea r9, [rbp + 0x190]
1408c81ae: mov r8d, 0x13
1408c81b4: call 0x141602990 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::GaussBlur>
1408c81b9: cmp byte ptr [rbp + 0x456], 8
1408c81c0: je 0x1408c81e7
1408c81c2: lea rcx, [rbp + 0x430]
1408c81c9: mov edx, 0xe8
1408c81ce: add rdx, qword ptr [rbp + 0x458]
1408c81d5: movabs r8, 0xd91ff8372094bc22
1408c81df: mov r9b, 1
1408c81e2: call 0x14096b850 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<bool>,bool>
1408c81e7: lea rcx, [rbp + 0x190]
1408c81ee: lea rdx, [rbp + 0x320]
1408c81f5: mov r8d, 0x190
1408c81fb: call 0x142923c00 ; ?
1408c8200: cmp byte ptr [rbp + 0x2c6], 8
1408c8207: je 0x1408c8233
1408c8209: lea rcx, [rbp + 0x2a0]
1408c8210: mov edx, 0xe8
1408c8215: add rdx, qword ptr [rbp + 0x2c8]
1408c821c: movabs r8, 0xdc022c4f3092a794
1408c8226: movss xmm3, dword ptr [rip + 0x20664c2]
1408c822e: call 0x14096ba30 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<f32>,f32>
1408c8233: lea r14, [rbp + 0x190]
1408c823a: mov r8d, 0x190
1408c8240: mov rcx, rdi
1408c8243: mov rdx, r14
1408c8246: call 0x142923c00 ; ?
1408c824b: mov byte ptr [rbp - 0x40], 9
1408c824f: mov dword ptr [rbp + 0x190], 2
1408c8259: mov qword ptr [rsp + 0x20], r14
1408c825e: lea rcx, [rbp - 0x40]
1408c8262: movabs r8, 0xd82d7a0adb9164c2
1408c826c: mov rdx, rdi
1408c826f: mov r9b, 0x19
1408c8272: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408c8277: lea rdi, [rbp + 0x190]
1408c827e: lea r14, [rbp + 0x320]
1408c8285: mov r8d, 0x190
1408c828b: mov rcx, rdi
1408c828e: mov rdx, r14
1408c8291: call 0x142923c00 ; ?
1408c8296: mov qword ptr [rbp - 0x40], rbx
1408c829a: mov dword ptr [rbp + 0x320], 2
1408c82a4: mov qword ptr [rsp + 0x20], r14
1408c82a9: lea rcx, [rbp - 0x40]
1408c82ad: movabs r8, 0x1ca51a73373208db
1408c82b7: mov rdx, rdi
1408c82ba: mov r9b, 0x32
1408c82bd: call 0x1408edd00 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >
1408c82c2: lea rcx, [rbp + 0x320]
1408c82c9: lea rdx, [rbp + 0x190]
1408c82d0: mov r8d, 0x190
1408c82d6: call 0x142923c00 ; ?
1408c82db: movups xmm0, xmmword ptr [rbp + 0x3c0]
1408c82e2: movaps xmmword ptr [rbp], xmm0
1408c82e6: movups xmm0, xmmword ptr [rbp + 0x3b0]
1408c82ed: movaps xmmword ptr [rbp - 0x10], xmm0
1408c82f1: movups xmm0, xmmword ptr [rbp + 0x3a0]
1408c82f8: movaps xmmword ptr [rbp - 0x20], xmm0
1408c82fc: movups xmm0, xmmword ptr [rbp + 0x390]
1408c8303: movaps xmmword ptr [rbp - 0x30], xmm0
1408c8307: movups xmm0, xmmword ptr [rbp + 0x380]
1408c830e: movaps xmmword ptr [rbp - 0x40], xmm0
1408c8312: mov rax, qword ptr [rbp + 0x458]
1408c8319: mov rcx, qword ptr [rbp + 0x4a0]
1408c8320: mov rdx, qword ptr [rbp + 0x330]
1408c8327: mov qword ptr [rbp + 0x1a0], rdx
1408c832e: movups xmm0, xmmword ptr [rbp + 0x320]
1408c8335: movaps xmmword ptr [rbp + 0x190], xmm0
1408c833c: movups xmm0, xmmword ptr [rbp + 0x338]
1408c8343: movaps xmmword ptr [rbp + 0x1c0], xmm0
1408c834a: mov rdx, qword ptr [rbp + 0x348]
1408c8351: mov qword ptr [rbp + 0x1d0], rdx
1408c8358: movups xmm0, xmmword ptr [rbp + 0x350]
1408c835f: movups xmmword ptr [rbp + 0x1a8], xmm0
1408c8366: mov rdx, qword ptr [rbp + 0x360]
1408c836d: mov qword ptr [rbp + 0x1b8], rdx
1408c8374: movups xmm0, xmmword ptr [rbp + 0x3d0]
1408c837b: movups xmm1, xmmword ptr [rbp + 0x3e0]
1408c8382: movups xmm2, xmmword ptr [rbp + 0x3f0]
1408c8389: movups xmm3, xmmword ptr [rbp + 0x400]
1408c8390: movups xmmword ptr [rbp + 0x208], xmm0
1408c8397: movups xmmword ptr [rbp + 0x218], xmm1
1408c839e: movups xmmword ptr [rbp + 0x228], xmm2
1408c83a5: movups xmmword ptr [rbp + 0x238], xmm3
1408c83ac: movups xmm0, xmmword ptr [rbp + 0x410]
1408c83b3: movups xmmword ptr [rbp + 0x248], xmm0
1408c83ba: movups xmm0, xmmword ptr [rbp + 0x420]
1408c83c1: movups xmmword ptr [rbp + 0x258], xmm0
1408c83c8: mov rdx, qword ptr [rbp + 0x378]
1408c83cf: mov qword ptr [rbp + 0x1e8], rdx
1408c83d6: movdqu xmm0, xmmword ptr [rbp + 0x368]
1408c83de: movdqu xmmword ptr [rbp + 0x1d8], xmm0
1408c83e6: mov qword ptr [rbp + 0x270], rcx
1408c83ed: mov qword ptr [rbp + 0x268], rax
1408c83f4: movaps xmm6, xmmword ptr [rip + 0x21da105]
1408c83fb: movaps xmmword ptr [rbp + 0x1f0], xmm6
1408c8402: lea rcx, [rbp - 0x40]
1408c8406: lea rdx, [rbp + 0x190]
1408c840d: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
1408c8412: cmp byte ptr [rbp + 0x456], 8
1408c8419: je 0x1408c842e
1408c841b: lea rcx, [rbp + 0x440]
1408c8422: mov byte ptr [rbp + 0x527], 0
1408c8429: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1408c842e: lea rax, [rip + 0x21dacef]
1408c8435: mov qword ptr [rbp + 0x190], rax
1408c843c: mov qword ptr [rbp + 0x198], 3
1408c8447: mov byte ptr [rbp + 0x1a0], 0
1408c844e: mov byte ptr [rbp + 0x527], 0
1408c8455: mov qword ptr [rsp + 0x28], r15
1408c845a: mov rax, qword ptr [rbp + 0x4d8]
1408c8461: mov qword ptr [rsp + 0x20], rax
1408c8466: lea rdx, [rip + 0x21daca4]
1408c846d: lea rcx, [rbp + 0x320]
1408c8474: lea r9, [rbp + 0x190]
1408c847b: mov r8d, 0x13
1408c8481: call 0x141602990 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::GaussBlur>
1408c8486: cmp byte ptr [rbp + 0x456], 8
1408c848d: je 0x1408c84b4
1408c848f: lea rcx, [rbp + 0x430]
1408c8496: mov edx, 0xe8
1408c849b: add rdx, qword ptr [rbp + 0x458]
1408c84a2: movabs r8, 0xd91ff8372094bc22
1408c84ac: xor r9d, r9d
1408c84af: call 0x14096b850 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<bool>,bool>
1408c84b4: lea rcx, [rbp + 0x190]
1408c84bb: lea rdx, [rbp + 0x320]
1408c84c2: mov r8d, 0x190
1408c84c8: call 0x142923c00 ; ?
1408c84cd: cmp byte ptr [rbp + 0x2c6], 8
1408c84d4: je 0x1408c8500
1408c84d6: lea rcx, [rbp + 0x2a0]
1408c84dd: mov edx, 0xe8
1408c84e2: add rdx, qword ptr [rbp + 0x2c8]
1408c84e9: movabs r8, 0xdc022c4f3092a794
1408c84f3: movss xmm3, dword ptr [rip + 0x20661f5]
1408c84fb: call 0x14096ba30 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<f32>,f32>
1408c8500: lea rdi, [rbp + 0x320]
1408c8507: lea r14, [rbp + 0x190]
1408c850e: mov r8d, 0x190
1408c8514: mov rcx, rdi
1408c8517: mov rdx, r14
1408c851a: call 0x142923c00 ; ?
1408c851f: mov qword ptr [rbp - 0x40], rbx
1408c8523: mov dword ptr [rbp + 0x190], 2
1408c852d: mov qword ptr [rsp + 0x20], r14
1408c8532: lea rcx, [rbp - 0x40]
1408c8536: movabs r8, 0xd82d7a0adb9164c2
1408c8540: mov rdx, rdi
1408c8543: mov r9b, 0x19
1408c8546: call 0x1408ee240 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >
1408c854b: lea rdi, [rbp + 0x190]
1408c8552: lea rbx, [rbp + 0x320]
1408c8559: mov r8d, 0x190
1408c855f: mov rcx, rdi
1408c8562: mov rdx, rbx
1408c8565: call 0x142923c00 ; ?
1408c856a: mov byte ptr [rbp - 0x40], 9
1408c856e: mov dword ptr [rbp + 0x320], 2
1408c8578: mov qword ptr [rsp + 0x20], rbx
1408c857d: lea rcx, [rbp - 0x40]
1408c8581: movabs r8, 0x1ca51a73373208db
1408c858b: mov rdx, rdi
1408c858e: mov r9b, 0x32
1408c8591: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408c8596: lea rdx, [rbp + 0x190]
1408c859d: mov r8d, 0x190
1408c85a3: mov rcx, rbx
1408c85a6: call 0x142923c00 ; ?
1408c85ab: movups xmm0, xmmword ptr [rbp + 0x3c0]
1408c85b2: movaps xmmword ptr [rbp], xmm0
1408c85b6: movups xmm0, xmmword ptr [rbp + 0x3b0]
1408c85bd: movaps xmmword ptr [rbp - 0x10], xmm0
1408c85c1: movups xmm0, xmmword ptr [rbp + 0x3a0]
1408c85c8: movaps xmmword ptr [rbp - 0x20], xmm0
1408c85cc: movups xmm0, xmmword ptr [rbp + 0x390]
1408c85d3: movaps xmmword ptr [rbp - 0x30], xmm0
1408c85d7: movups xmm0, xmmword ptr [rbp + 0x380]
1408c85de: movaps xmmword ptr [rbp - 0x40], xmm0
1408c85e2: mov rax, qword ptr [rbp + 0x458]
1408c85e9: mov rcx, qword ptr [rbp + 0x4a0]
1408c85f0: mov rdx, qword ptr [rbp + 0x330]
1408c85f7: mov qword ptr [rbp + 0x1a0], rdx
1408c85fe: movups xmm0, xmmword ptr [rbp + 0x320]
1408c8605: movaps xmmword ptr [rbp + 0x190], xmm0
1408c860c: movups xmm0, xmmword ptr [rbp + 0x338]
1408c8613: movaps xmmword ptr [rbp + 0x1c0], xmm0
1408c861a: mov rdx, qword ptr [rbp + 0x348]
1408c8621: mov qword ptr [rbp + 0x1d0], rdx
1408c8628: movups xmm0, xmmword ptr [rbp + 0x350]
1408c862f: movups xmmword ptr [rbp + 0x1a8], xmm0
1408c8636: mov rdx, qword ptr [rbp + 0x360]
1408c863d: mov qword ptr [rbp + 0x1b8], rdx
1408c8644: movups xmm0, xmmword ptr [rbp + 0x3d0]
1408c864b: movups xmm1, xmmword ptr [rbp + 0x3e0]
1408c8652: movups xmm2, xmmword ptr [rbp + 0x3f0]
1408c8659: movups xmm3, xmmword ptr [rbp + 0x400]
1408c8660: movups xmmword ptr [rbp + 0x208], xmm0
1408c8667: movups xmmword ptr [rbp + 0x218], xmm1
1408c866e: movups xmmword ptr [rbp + 0x228], xmm2
1408c8675: movups xmmword ptr [rbp + 0x238], xmm3
1408c867c: movups xmm0, xmmword ptr [rbp + 0x410]
1408c8683: movups xmmword ptr [rbp + 0x248], xmm0
1408c868a: movups xmm0, xmmword ptr [rbp + 0x420]
1408c8691: movups xmmword ptr [rbp + 0x258], xmm0
1408c8698: mov rdx, qword ptr [rbp + 0x378]
1408c869f: mov qword ptr [rbp + 0x1e8], rdx
1408c86a6: movdqu xmm0, xmmword ptr [rbp + 0x368]
1408c86ae: movdqu xmmword ptr [rbp + 0x1d8], xmm0
1408c86b6: mov qword ptr [rbp + 0x270], rcx
1408c86bd: mov qword ptr [rbp + 0x268], rax
1408c86c4: movaps xmmword ptr [rbp + 0x1f0], xmm6
1408c86cb: lea rcx, [rbp - 0x40]
1408c86cf: lea rdx, [rbp + 0x190]
1408c86d6: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
1408c86db: cmp byte ptr [rbp + 0x456], 8
1408c86e2: je 0x1408c86f7
1408c86e4: lea rcx, [rbp + 0x440]
1408c86eb: mov byte ptr [rbp + 0x527], 0
1408c86f2: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1408c86f7: mov byte ptr [rbp + 0x527], 0
1408c86fe: mov qword ptr [rsp + 0x30], r15
1408c8703: mov rax, qword ptr [rbp + 0x4d8]
1408c870a: mov qword ptr [rsp + 0x28], rax
1408c870f: mov qword ptr [rsp + 0x20], 0x1b
1408c8718: lea rdx, [rip + 0x21daa08]
1408c871f: lea r9, [rip + 0x21daa13]
1408c8726: lea rcx, [rbp + 0x190]
1408c872d: mov r8d, 0x12
1408c8733: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
1408c8738: mov byte ptr [rbp - 0x40], 9
1408c873c: mov dword ptr [rbp + 0x320], 2
1408c8746: mov qword ptr [rsp + 0x20], rbx
1408c874b: lea rsi, [rbp - 0x40]
1408c874f: lea rdx, [rbp + 0x190]
1408c8756: movabs r8, 0x61b08b5e0470c5f2
1408c8760: mov rcx, rsi
1408c8763: mov r9b, 0x19
1408c8766: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408c876b: lea rdi, [rbp + 0x320]
1408c8772: lea rdx, [rbp + 0x190]
1408c8779: mov r8d, 0x190
1408c877f: mov rcx, rdi
1408c8782: call 0x142923c00 ; ?
1408c8787: mov byte ptr [rbp + 0x4b8], 0xa
1408c878e: mov dword ptr [rbp - 0x40], 2
1408c8795: mov qword ptr [rsp + 0x20], rsi
1408c879a: lea rcx, [rbp + 0x4b8]
1408c87a1: movabs r8, 0x297d1e4cdae8f657
1408c87ab: mov rdx, rdi
1408c87ae: mov r9b, 0x19
1408c87b1: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408c87b6: mov r8d, 0x190
1408c87bc: mov rcx, rsi
1408c87bf: mov rdx, rdi
1408c87c2: call 0x142923c00 ; ?
1408c87c7: mov byte ptr [rbp + 0x190], 0xe
1408c87ce: mov dword ptr [rbp + 0x320], 2
1408c87d8: mov qword ptr [rsp + 0x20], rdi
1408c87dd: lea rcx, [rbp + 0x190]
1408c87e4: movabs r8, 0x4ee098b38612d2b9
1408c87ee: mov rdx, rsi
1408c87f1: mov r9b, 0x32
1408c87f4: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408c87f9: lea rcx, [rbp + 0x320]
1408c8800: lea rdx, [rbp - 0x40]
1408c8804: mov r8d, 0x190
1408c880a: call 0x142923c00 ; ?
1408c880f: movups xmm0, xmmword ptr [rbp + 0x3c0]
1408c8816: movaps xmmword ptr [rbp], xmm0
1408c881a: movups xmm0, xmmword ptr [rbp + 0x3b0]
1408c8821: movaps xmmword ptr [rbp - 0x10], xmm0
1408c8825: movups xmm0, xmmword ptr [rbp + 0x3a0]
1408c882c: movaps xmmword ptr [rbp - 0x20], xmm0
1408c8830: movups xmm0, xmmword ptr [rbp + 0x390]
1408c8837: movaps xmmword ptr [rbp - 0x30], xmm0
1408c883b: movups xmm0, xmmword ptr [rbp + 0x380]
1408c8842: movaps xmmword ptr [rbp - 0x40], xmm0
1408c8846: mov rax, qword ptr [rbp + 0x458]
1408c884d: mov rcx, qword ptr [rbp + 0x4a0]
1408c8854: mov rdx, qword ptr [rbp + 0x330]
1408c885b: mov qword ptr [rbp + 0x1a0], rdx
1408c8862: movups xmm0, xmmword ptr [rbp + 0x320]
1408c8869: movaps xmmword ptr [rbp + 0x190], xmm0
1408c8870: movups xmm0, xmmword ptr [rbp + 0x338]
1408c8877: movaps xmmword ptr [rbp + 0x1c0], xmm0
1408c887e: mov rdx, qword ptr [rbp + 0x348]
1408c8885: mov qword ptr [rbp + 0x1d0], rdx
1408c888c: movups xmm0, xmmword ptr [rbp + 0x350]
1408c8893: movups xmmword ptr [rbp + 0x1a8], xmm0
1408c889a: mov rdx, qword ptr [rbp + 0x360]
1408c88a1: mov qword ptr [rbp + 0x1b8], rdx
1408c88a8: movups xmm0, xmmword ptr [rbp + 0x3d0]
1408c88af: movups xmm1, xmmword ptr [rbp + 0x3e0]
1408c88b6: movups xmm2, xmmword ptr [rbp + 0x3f0]
1408c88bd: movups xmm3, xmmword ptr [rbp + 0x400]
1408c88c4: movups xmmword ptr [rbp + 0x208], xmm0
1408c88cb: movups xmmword ptr [rbp + 0x218], xmm1
1408c88d2: movups xmmword ptr [rbp + 0x228], xmm2
1408c88d9: movups xmmword ptr [rbp + 0x238], xmm3
1408c88e0: movups xmm0, xmmword ptr [rbp + 0x410]
1408c88e7: movups xmmword ptr [rbp + 0x248], xmm0
1408c88ee: movups xmm0, xmmword ptr [rbp + 0x420]
1408c88f5: movups xmmword ptr [rbp + 0x258], xmm0
1408c88fc: mov rdx, qword ptr [rbp + 0x378]
1408c8903: mov qword ptr [rbp + 0x1e8], rdx
1408c890a: movdqu xmm0, xmmword ptr [rbp + 0x368]
1408c8912: movdqu xmmword ptr [rbp + 0x1d8], xmm0
1408c891a: mov qword ptr [rbp + 0x270], rcx
1408c8921: mov qword ptr [rbp + 0x268], rax
1408c8928: movaps xmmword ptr [rbp + 0x1f0], xmm6
1408c892f: lea rcx, [rbp - 0x40]
1408c8933: lea rdx, [rbp + 0x190]
1408c893a: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
1408c893f: cmp byte ptr [rbp + 0x456], 8
1408c8946: je 0x1408c895b
1408c8948: lea rcx, [rbp + 0x440]
1408c894f: mov byte ptr [rbp + 0x527], 0
1408c8956: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1408c895b: mov rax, qword ptr [rbp + 0x4c8]
1408c8962: mov byte ptr [rax + 0x40], 1
1408c8966: mov rdx, qword ptr [rbp + 0x4e0]
1408c896d: test rdx, rdx
1408c8970: je 0x1408c8988
1408c8972: mov rcx, qword ptr [rbp + 0x4e8]
1408c8979: shl rdx, 4
1408c897d: mov r8d, 8
1408c8983: call 0x140613c20 ; ?
1408c8988: movaps xmm6, xmmword ptr [rbp + 0x530]
1408c898f: add rsp, 0x5c8
1408c8996: pop rbx
1408c8997: pop rdi
1408c8998: pop rsi
1408c8999: pop r12
1408c899b: pop r13
1408c899d: pop r14
1408c899f: pop r15
1408c89a1: pop rbp
1408c89a2: ret 
1408c89a3: mov byte ptr [rbp + 0x526], 1
1408c89aa: lea rcx, [rip + 0x21da5b5]
1408c89b1: lea r8, [rip + 0x21da5d8]
1408c89b8: mov edx, 0x29
1408c89bd: call 0x1428d9430 ; core::panicking::panic
1408c89c2: jmp 0x1408c8b97
1408c89c7: mov rcx, r12
1408c89ca: mov r8d, 0x3b9aca00
1408c89d0: call 0x1428cd210 ; parking_lot::raw_mutex::RawMutex::lock_slow
1408c89d5: jmp 0x1408c7bbf
1408c89da: lea r8, [rip + 0x21cccdf]
1408c89e1: mov rdx, rdi
1408c89e4: call 0x1428d98e0 ; core::slice::impl$0::copy_from_slice::len_mismatch_fail
1408c89e9: jmp 0x1408c8b97
1408c89ee: mov byte ptr [rbp + 0x525], 1
1408c89f5: mov rcx, rsi
1408c89f8: xor edx, edx
1408c89fa: call 0x1428cdb90 ; parking_lot::raw_mutex::RawMutex::unlock_slow
1408c89ff: jmp 0x1408c7c8d
1408c8a04: mov byte ptr [rbp + 0x527], 1
1408c8a0b: lea rcx, [rip + 0x237215e]
1408c8a12: call 0x1428d9250 ; core::cell::panic_already_borrowed
1408c8a17: jmp 0x1408c8b97
1408c8a1c: mov byte ptr [rbp + 0x527], 1
1408c8a23: lea rcx, [rip + 0x2372146]
1408c8a2a: call 0x1428d9250 ; core::cell::panic_already_borrowed
1408c8a2f: jmp 0x1408c8b97
1408c8a34: mov byte ptr [rbp + 0x190], 1
1408c8a3b: lea rax, [rbp + 0x190]
1408c8a42: mov qword ptr [rbp + 0x320], rax
1408c8a49: lea rax, [rip + 0x21da588]
1408c8a50: mov qword ptr [rsp + 0x20], rax
1408c8a55: lea rcx, [rip + 0x2d6c25c]
1408c8a5c: lea r9, [rip + 0x21da1d5]
1408c8a63: lea r8, [rbp + 0x320]
1408c8a6a: xor edx, edx
1408c8a6c: call 0x1428d8820 ; std::sys::sync::once::futex::Once::call
1408c8a71: jmp 0x1408c7948
1408c8a76: mov qword ptr [rsp + 0x20], 1
1408c8a7f: lea rcx, [rbp + 0x190]
1408c8a86: mov r8d, 0x1f
1408c8a8c: mov r9d, 1
1408c8a92: call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1408c8a97: mov rdx, qword ptr [rbp + 0x1a0]
1408c8a9e: jmp 0x1408c7a57
1408c8aa3: mov rcx, rsi
1408c8aa6: call 0x1428d8810 ; std::sys::sync::mutex::futex::Mutex::wake
1408c8aab: jmp 0x1408c7baf
1408c8ab0: call 0x1428d86b0 ; std::panicking::panic_count::is_zero_slow_path
1408c8ab5: test al, al
1408c8ab7: jne 0x1408c7ba3
1408c8abd: mov byte ptr [rsi + 1], 1
1408c8ac1: jmp 0x1408c7ba3
1408c8ac6: xor r8d, r8d
1408c8ac9: jmp 0x1408c8ace
1408c8acb: mov r8b, 1
1408c8ace: mov byte ptr [rbp + 0x526], 1
1408c8ad5: lea rcx, [rip + 0x21ccb75]
1408c8adc: mov edx, 0xe
1408c8ae1: call 0x1428c9e00 ; bytemuck::internal::something_went_wrong<bytemuck::PodCastError>
1408c8ae6: jmp 0x1408c8b97
1408c8aeb: lea rcx, [rip + 0x21da416]
1408c8af2: call 0x1428d9310 ; core::option::unwrap_failed
1408c8af7: lea rcx, [rip + 0x21da422]
1408c8afe: call 0x1428d9310 ; core::option::unwrap_failed
1408c8b03: lea rcx, [rip + 0x21da42e]
1408c8b0a: call 0x1428d9310 ; core::option::unwrap_failed
1408c8b0f: mov byte ptr [rbp + 0x527], 1
1408c8b16: lea rcx, [rip + 0x2371f83]
1408c8b1d: call 0x1428d80d0 ; std::thread::local::panic_access_error
1408c8b22: jmp 0x1408c8b97
1408c8b24: mov byte ptr [rbp + 0x527], 1
1408c8b2b: lea rcx, [rip + 0x2371f6e]
1408c8b32: call 0x1428d80d0 ; std::thread::local::panic_access_error
1408c8b37: jmp 0x1408c8b97
1408c8b39: mov rax, qword ptr [rbp + 0x328]
1408c8b40: movzx ecx, byte ptr [rbp + 0x330]
1408c8b47: mov qword ptr [rbp + 0x190], rax
1408c8b4e: mov byte ptr [rbp + 0x198], cl
1408c8b54: lea rax, [rip + 0x21da47d]
1408c8b5b: mov qword ptr [rsp + 0x20], rax
1408c8b60: lea rcx, [rip + 0x21da489]
1408c8b67: lea r9, [rip + 0x21da162]
1408c8b6e: lea r8, [rbp + 0x190]
1408c8b75: mov edx, 0x13
1408c8b7a: call 0x1428d9760 ; core::result::unwrap_failed
1408c8b7f: jmp 0x1408c8b97
1408c8b81: lea r8, [rip + 0x21da1b8]
1408c8b88: mov ecx, 1
1408c8b8d: mov edx, 0xf
1408c8b92: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1408c8b97: ud2 
1408c8b99: nop dword ptr [rax]
1408c8ba0: mov qword ptr [rsp + 0x10], rdx
1408c8ba5: push rbp
1408c8ba6: push r15
1408c8ba8: push r14
1408c8baa: push r13
1408c8bac: push r12
1408c8bae: push rsi
1408c8baf: push rdi
1408c8bb0: push rbx
1408c8bb1: sub rsp, 0x48
1408c8bb5: lea rbp, [rdx + 0x80]
1408c8bbc: movaps xmmword ptr [rsp + 0x30], xmm6
1408c8bc1: mov rdx, qword ptr [rbp + 0x190]
1408c8bc8: test rdx, rdx
1408c8bcb: je 0x1408c8bdf
1408c8bcd: mov rcx, qword ptr [rbp + 0x198]
1408c8bd4: mov r8d, 1
1408c8bda: call 0x140613c20 ; ?
1408c8bdf: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c8be4: add rsp, 0x48
1408c8be8: pop rbx
1408c8be9: pop rdi
1408c8bea: pop rsi
1408c8beb: pop r12
1408c8bed: pop r13
1408c8bef: pop r14
1408c8bf1: pop r15
1408c8bf3: pop rbp
1408c8bf4: ret 
1408c8bf5: nop word ptr cs:[rax + rax]
1408c8c00: mov qword ptr [rsp + 0x10], rdx
1408c8c05: push rbp
1408c8c06: push r15
1408c8c08: push r14
1408c8c0a: push r13
1408c8c0c: push r12
1408c8c0e: push rsi
1408c8c0f: push rdi
1408c8c10: push rbx
1408c8c11: sub rsp, 0x48
1408c8c15: lea rbp, [rdx + 0x80]
1408c8c1c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c8c21: mov rdx, qword ptr [rbp + 0x320]
1408c8c28: test rdx, rdx
1408c8c2b: je 0x1408c8c3f
1408c8c2d: mov rcx, qword ptr [rbp + 0x328]
1408c8c34: mov r8d, 1
1408c8c3a: call 0x140613c20 ; ?
1408c8c3f: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c8c44: add rsp, 0x48
1408c8c48: pop rbx
1408c8c49: pop rdi
1408c8c4a: pop rsi
1408c8c4b: pop r12
1408c8c4d: pop r13
1408c8c4f: pop r14
1408c8c51: pop r15
1408c8c53: pop rbp
1408c8c54: ret 
1408c8c55: nop word ptr cs:[rax + rax]
1408c8c60: mov qword ptr [rsp + 0x10], rdx
1408c8c65: push rbp
1408c8c66: push r15
1408c8c68: push r14
1408c8c6a: push r13
1408c8c6c: push r12
1408c8c6e: push rsi
1408c8c6f: push rdi
1408c8c70: push rbx
1408c8c71: sub rsp, 0x48
1408c8c75: lea rbp, [rdx + 0x80]
1408c8c7c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c8c81: lea rcx, [rbp + 0x190]
1408c8c88: call 0x1405d2870 ; core::ptr::drop_in_place<std::sync::poison::mutex::MutexGuard<winit::platform_impl::platform::window_state::WindowState> >
1408c8c8d: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c8c92: add rsp, 0x48
1408c8c96: pop rbx
1408c8c97: pop rdi
1408c8c98: pop rsi
1408c8c99: pop r12
1408c8c9b: pop r13
1408c8c9d: pop r14
1408c8c9f: pop r15
1408c8ca1: pop rbp
1408c8ca2: ret 
1408c8ca3: nop word ptr cs:[rax + rax]
1408c8cb0: mov qword ptr [rsp + 0x10], rdx
1408c8cb5: push rbp
1408c8cb6: push r15
1408c8cb8: push r14
1408c8cba: push r13
1408c8cbc: push r12
1408c8cbe: push rsi
1408c8cbf: push rdi
1408c8cc0: push rbx
1408c8cc1: sub rsp, 0x48
1408c8cc5: lea rbp, [rdx + 0x80]
1408c8ccc: movaps xmmword ptr [rsp + 0x30], xmm6
1408c8cd1: mov byte ptr [rbp + 0x527], 0
1408c8cd8: lea rcx, [rbp + 0x190]
1408c8cdf: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408c8ce4: mov byte ptr [rbp + 0x527], 0
1408c8ceb: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c8cf0: add rsp, 0x48
1408c8cf4: pop rbx
1408c8cf5: pop rdi
1408c8cf6: pop rsi
1408c8cf7: pop r12
1408c8cf9: pop r13
1408c8cfb: pop r14
1408c8cfd: pop r15
1408c8cff: pop rbp
1408c8d00: ret 
1408c8d01: nop word ptr cs:[rax + rax]
1408c8d10: mov qword ptr [rsp + 0x10], rdx
1408c8d15: push rbp
1408c8d16: push r15
1408c8d18: push r14
1408c8d1a: push r13
1408c8d1c: push r12
1408c8d1e: push rsi
1408c8d1f: push rdi
1408c8d20: push rbx
1408c8d21: sub rsp, 0x48
1408c8d25: lea rbp, [rdx + 0x80]
1408c8d2c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c8d31: mov byte ptr [rbp + 0x527], 0
1408c8d38: lea rcx, [rbp + 0x320]
1408c8d3f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408c8d44: mov byte ptr [rbp + 0x527], 0
1408c8d4b: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c8d50: add rsp, 0x48
1408c8d54: pop rbx
1408c8d55: pop rdi
1408c8d56: pop rsi
1408c8d57: pop r12
1408c8d59: pop r13
1408c8d5b: pop r14
1408c8d5d: pop r15
1408c8d5f: pop rbp
1408c8d60: ret 
1408c8d61: nop word ptr cs:[rax + rax]
1408c8d70: mov qword ptr [rsp + 0x10], rdx
1408c8d75: push rbp
1408c8d76: push r15
1408c8d78: push r14
1408c8d7a: push r13
1408c8d7c: push r12
1408c8d7e: push rsi
1408c8d7f: push rdi
1408c8d80: push rbx
1408c8d81: sub rsp, 0x48
1408c8d85: lea rbp, [rdx + 0x80]
1408c8d8c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c8d91: mov byte ptr [rbp + 0x527], 0
1408c8d98: lea rcx, [rbp + 0x190]
1408c8d9f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408c8da4: mov byte ptr [rbp + 0x527], 0
1408c8dab: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c8db0: add rsp, 0x48
1408c8db4: pop rbx
1408c8db5: pop rdi
1408c8db6: pop rsi
1408c8db7: pop r12
1408c8db9: pop r13
1408c8dbb: pop r14
1408c8dbd: pop r15
1408c8dbf: pop rbp
1408c8dc0: ret 
1408c8dc1: nop word ptr cs:[rax + rax]
1408c8dd0: mov qword ptr [rsp + 0x10], rdx
1408c8dd5: push rbp
1408c8dd6: push r15
1408c8dd8: push r14
1408c8dda: push r13
1408c8ddc: push r12
1408c8dde: push rsi
1408c8ddf: push rdi
1408c8de0: push rbx
1408c8de1: sub rsp, 0x48
1408c8de5: lea rbp, [rdx + 0x80]
1408c8dec: movaps xmmword ptr [rsp + 0x30], xmm6
1408c8df1: mov byte ptr [rbp + 0x527], 0
1408c8df8: lea rcx, [rbp + 0x320]
1408c8dff: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408c8e04: mov byte ptr [rbp + 0x527], 0
1408c8e0b: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c8e10: add rsp, 0x48
1408c8e14: pop rbx
1408c8e15: pop rdi
1408c8e16: pop rsi
1408c8e17: pop r12
1408c8e19: pop r13
1408c8e1b: pop r14
1408c8e1d: pop r15
1408c8e1f: pop rbp
1408c8e20: ret 
1408c8e21: nop word ptr cs:[rax + rax]
1408c8e30: mov qword ptr [rsp + 0x10], rdx
1408c8e35: push rbp
1408c8e36: push r15
1408c8e38: push r14
1408c8e3a: push r13
1408c8e3c: push r12
1408c8e3e: push rsi
1408c8e3f: push rdi
1408c8e40: push rbx
1408c8e41: sub rsp, 0x48
1408c8e45: lea rbp, [rdx + 0x80]
1408c8e4c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c8e51: mov byte ptr [rbp + 0x527], 1
1408c8e58: lea rcx, [rbp + 0x320]
1408c8e5f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408c8e64: mov byte ptr [rbp + 0x527], 1
1408c8e6b: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c8e70: add rsp, 0x48
1408c8e74: pop rbx
1408c8e75: pop rdi
1408c8e76: pop rsi
1408c8e77: pop r12
1408c8e79: pop r13
1408c8e7b: pop r14
1408c8e7d: pop r15
1408c8e7f: pop rbp
1408c8e80: ret 
1408c8e81: nop word ptr cs:[rax + rax]
1408c8e90: mov qword ptr [rsp + 0x10], rdx
1408c8e95: push rbp
1408c8e96: push r15
1408c8e98: push r14
1408c8e9a: push r13
1408c8e9c: push r12
1408c8e9e: push rsi
1408c8e9f: push rdi
1408c8ea0: push rbx
1408c8ea1: sub rsp, 0x48
1408c8ea5: lea rbp, [rdx + 0x80]
1408c8eac: movaps xmmword ptr [rsp + 0x30], xmm6
1408c8eb1: mov byte ptr [rbp + 0x527], 1
1408c8eb8: lea rcx, [rbp + 0x190]
1408c8ebf: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408c8ec4: mov byte ptr [rbp + 0x527], 1
1408c8ecb: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c8ed0: add rsp, 0x48
1408c8ed4: pop rbx
1408c8ed5: pop rdi
1408c8ed6: pop rsi
1408c8ed7: pop r12
1408c8ed9: pop r13
1408c8edb: pop r14
1408c8edd: pop r15
1408c8edf: pop rbp
1408c8ee0: ret 
1408c8ee1: nop word ptr cs:[rax + rax]
1408c8ef0: mov qword ptr [rsp + 0x10], rdx
1408c8ef5: push rbp
1408c8ef6: push r15
1408c8ef8: push r14
1408c8efa: push r13
1408c8efc: push r12
1408c8efe: push rsi
1408c8eff: push rdi
1408c8f00: push rbx
1408c8f01: sub rsp, 0x48
1408c8f05: lea rbp, [rdx + 0x80]
1408c8f0c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c8f11: mov byte ptr [rbp + 0x527], 1
1408c8f18: lea rcx, [rbp + 0x320]
1408c8f1f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408c8f24: mov byte ptr [rbp + 0x527], 1
1408c8f2b: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c8f30: add rsp, 0x48
1408c8f34: pop rbx
1408c8f35: pop rdi
1408c8f36: pop rsi
1408c8f37: pop r12
1408c8f39: pop r13
1408c8f3b: pop r14
1408c8f3d: pop r15
1408c8f3f: pop rbp
1408c8f40: ret 
1408c8f41: nop word ptr cs:[rax + rax]
1408c8f50: mov qword ptr [rsp + 0x10], rdx
1408c8f55: push rbp
1408c8f56: push r15
1408c8f58: push r14
1408c8f5a: push r13
1408c8f5c: push r12
1408c8f5e: push rsi
1408c8f5f: push rdi
1408c8f60: push rbx
1408c8f61: sub rsp, 0x48
1408c8f65: lea rbp, [rdx + 0x80]
1408c8f6c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c8f71: movdqu xmm0, xmmword ptr [rbp + 0x160]
1408c8f79: movups xmm1, xmmword ptr [rbp + 0x170]
1408c8f80: movaps xmmword ptr [rbp + 0x330], xmm1
1408c8f87: movdqa xmmword ptr [rbp + 0x320], xmm0
1408c8f8f: mov byte ptr [rbp + 0x525], 0
1408c8f96: lea rcx, [rbp + 0x320]
1408c8f9d: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1408c8fa2: mov byte ptr [rbp + 0x525], 0
1408c8fa9: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c8fae: add rsp, 0x48
1408c8fb2: pop rbx
1408c8fb3: pop rdi
1408c8fb4: pop rsi
1408c8fb5: pop r12
1408c8fb7: pop r13
1408c8fb9: pop r14
1408c8fbb: pop r15
1408c8fbd: pop rbp
1408c8fbe: ret 
1408c8fbf: nop 
1408c8fc0: mov qword ptr [rsp + 0x10], rdx
1408c8fc5: push rbp
1408c8fc6: push r15
1408c8fc8: push r14
1408c8fca: push r13
1408c8fcc: push r12
1408c8fce: push rsi
1408c8fcf: push rdi
1408c8fd0: push rbx
1408c8fd1: sub rsp, 0x48
1408c8fd5: lea rbp, [rdx + 0x80]
1408c8fdc: movaps xmmword ptr [rsp + 0x30], xmm6
1408c8fe1: mov rax, qword ptr [rbp + 0x510]
1408c8fe8: inc qword ptr [rax]
1408c8feb: mov byte ptr [rbp + 0x527], 1
1408c8ff2: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c8ff7: add rsp, 0x48
1408c8ffb: pop rbx
1408c8ffc: pop rdi
1408c8ffd: pop rsi
1408c8ffe: pop r12
1408c9000: pop r13
1408c9002: pop r14
1408c9004: pop r15
1408c9006: pop rbp
1408c9007: ret 
1408c9008: nop dword ptr [rax + rax]
1408c9010: mov qword ptr [rsp + 0x10], rdx
1408c9015: push rbp
1408c9016: push r15
1408c9018: push r14
1408c901a: push r13
1408c901c: push r12
1408c901e: push rsi
1408c901f: push rdi
1408c9020: push rbx
1408c9021: sub rsp, 0x48
1408c9025: lea rbp, [rdx + 0x80]
1408c902c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c9031: mov rax, qword ptr [rbp + 0x518]
1408c9038: inc qword ptr [rax]
1408c903b: mov byte ptr [rbp + 0x527], 1
1408c9042: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c9047: add rsp, 0x48
1408c904b: pop rbx
1408c904c: pop rdi
1408c904d: pop rsi
1408c904e: pop r12
1408c9050: pop r13
1408c9052: pop r14
1408c9054: pop r15
1408c9056: pop rbp
1408c9057: ret 
1408c9058: nop dword ptr [rax + rax]
1408c9060: mov qword ptr [rsp + 0x10], rdx
1408c9065: push rbp
1408c9066: push r15
1408c9068: push r14
1408c906a: push r13
1408c906c: push r12
1408c906e: push rsi
1408c906f: push rdi
1408c9070: push rbx
1408c9071: sub rsp, 0x48
1408c9075: lea rbp, [rdx + 0x80]
1408c907c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c9081: mov rcx, qword ptr [rbp + 0x510]
1408c9088: movzx edx, byte ptr [rbp + 0x4d0]
1408c908f: call 0x140043000 ; core::ptr::drop_in_place<std::sync::poison::mutex::MutexGuard<winit::platform_impl::platform::window_state::WindowState> >
1408c9094: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c9099: add rsp, 0x48
1408c909d: pop rbx
1408c909e: pop rdi
1408c909f: pop rsi
1408c90a0: pop r12
1408c90a2: pop r13
1408c90a4: pop r14
1408c90a6: pop r15
1408c90a8: pop rbp
1408c90a9: ret 
1408c90aa: nop word ptr [rax + rax]
1408c90b0: mov qword ptr [rsp + 0x10], rdx
1408c90b5: push rbp
1408c90b6: push r15
1408c90b8: push r14
1408c90ba: push r13
1408c90bc: push r12
1408c90be: push rsi
1408c90bf: push rdi
1408c90c0: push rbx
1408c90c1: sub rsp, 0x48
1408c90c5: lea rbp, [rdx + 0x80]
1408c90cc: movaps xmmword ptr [rsp + 0x30], xmm6
1408c90d1: cmp byte ptr [rbp + 0x456], 8
1408c90d8: je 0x1408c90ed
1408c90da: lea rcx, [rbp + 0x440]
1408c90e1: mov byte ptr [rbp + 0x527], 0
1408c90e8: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1408c90ed: mov byte ptr [rbp + 0x527], 0
1408c90f4: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c90f9: add rsp, 0x48
1408c90fd: pop rbx
1408c90fe: pop rdi
1408c90ff: pop rsi
1408c9100: pop r12
1408c9102: pop r13
1408c9104: pop r14
1408c9106: pop r15
1408c9108: pop rbp
1408c9109: ret 
1408c910a: nop word ptr [rax + rax]
1408c9110: mov qword ptr [rsp + 0x10], rdx
1408c9115: push rbp
1408c9116: push r15
1408c9118: push r14
1408c911a: push r13
1408c911c: push r12
1408c911e: push rsi
1408c911f: push rdi
1408c9120: push rbx
1408c9121: sub rsp, 0x48
1408c9125: lea rbp, [rdx + 0x80]
1408c912c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c9131: mov byte ptr [rbp + 0x527], 0
1408c9138: lea rcx, [rbp - 0x40]
1408c913c: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408c9141: mov byte ptr [rbp + 0x527], 0
1408c9148: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c914d: add rsp, 0x48
1408c9151: pop rbx
1408c9152: pop rdi
1408c9153: pop rsi
1408c9154: pop r12
1408c9156: pop r13
1408c9158: pop r14
1408c915a: pop r15
1408c915c: pop rbp
1408c915d: ret 
1408c915e: nop 
1408c9160: mov qword ptr [rsp + 0x10], rdx
1408c9165: push rbp
1408c9166: push r15
1408c9168: push r14
1408c916a: push r13
1408c916c: push r12
1408c916e: push rsi
1408c916f: push rdi
1408c9170: push rbx
1408c9171: sub rsp, 0x48
1408c9175: lea rbp, [rdx + 0x80]
1408c917c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c9181: mov byte ptr [rbp + 0x527], 0
1408c9188: lea rcx, [rbp + 0x320]
1408c918f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408c9194: mov byte ptr [rbp + 0x527], 0
1408c919b: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c91a0: add rsp, 0x48
1408c91a4: pop rbx
1408c91a5: pop rdi
1408c91a6: pop rsi
1408c91a7: pop r12
1408c91a9: pop r13
1408c91ab: pop r14
1408c91ad: pop r15
1408c91af: pop rbp
1408c91b0: ret 
1408c91b1: nop word ptr cs:[rax + rax]
1408c91c0: mov qword ptr [rsp + 0x10], rdx
1408c91c5: push rbp
1408c91c6: push r15
1408c91c8: push r14
1408c91ca: push r13
1408c91cc: push r12
1408c91ce: push rsi
1408c91cf: push rdi
1408c91d0: push rbx
1408c91d1: sub rsp, 0x48
1408c91d5: lea rbp, [rdx + 0x80]
1408c91dc: movaps xmmword ptr [rsp + 0x30], xmm6
1408c91e1: mov byte ptr [rbp + 0x527], 0
1408c91e8: lea rcx, [rbp + 0x190]
1408c91ef: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408c91f4: mov byte ptr [rbp + 0x527], 0
1408c91fb: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c9200: add rsp, 0x48
1408c9204: pop rbx
1408c9205: pop rdi
1408c9206: pop rsi
1408c9207: pop r12
1408c9209: pop r13
1408c920b: pop r14
1408c920d: pop r15
1408c920f: pop rbp
1408c9210: ret 
1408c9211: nop word ptr cs:[rax + rax]
1408c9220: mov qword ptr [rsp + 0x10], rdx
1408c9225: push rbp
1408c9226: push r15
1408c9228: push r14
1408c922a: push r13
1408c922c: push r12
1408c922e: push rsi
1408c922f: push rdi
1408c9230: push rbx
1408c9231: sub rsp, 0x48
1408c9235: lea rbp, [rdx + 0x80]
1408c923c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c9241: cmp byte ptr [rbp + 0x456], 8
1408c9248: je 0x1408c925d
1408c924a: lea rcx, [rbp + 0x440]
1408c9251: mov byte ptr [rbp + 0x527], 0
1408c9258: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1408c925d: mov byte ptr [rbp + 0x527], 0
1408c9264: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c9269: add rsp, 0x48
1408c926d: pop rbx
1408c926e: pop rdi
1408c926f: pop rsi
1408c9270: pop r12
1408c9272: pop r13
1408c9274: pop r14
1408c9276: pop r15
1408c9278: pop rbp
1408c9279: ret 
1408c927a: nop word ptr [rax + rax]
1408c9280: mov qword ptr [rsp + 0x10], rdx
1408c9285: push rbp
1408c9286: push r15
1408c9288: push r14
1408c928a: push r13
1408c928c: push r12
1408c928e: push rsi
1408c928f: push rdi
1408c9290: push rbx
1408c9291: sub rsp, 0x48
1408c9295: lea rbp, [rdx + 0x80]
1408c929c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c92a1: mov byte ptr [rbp + 0x527], 0
1408c92a8: lea rcx, [rbp + 0x190]
1408c92af: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408c92b4: mov byte ptr [rbp + 0x527], 0
1408c92bb: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c92c0: add rsp, 0x48
1408c92c4: pop rbx
1408c92c5: pop rdi
1408c92c6: pop rsi
1408c92c7: pop r12
1408c92c9: pop r13
1408c92cb: pop r14
1408c92cd: pop r15
1408c92cf: pop rbp
1408c92d0: ret 
1408c92d1: nop word ptr cs:[rax + rax]
1408c92e0: mov qword ptr [rsp + 0x10], rdx
1408c92e5: push rbp
1408c92e6: push r15
1408c92e8: push r14
1408c92ea: push r13
1408c92ec: push r12
1408c92ee: push rsi
1408c92ef: push rdi
1408c92f0: push rbx
1408c92f1: sub rsp, 0x48
1408c92f5: lea rbp, [rdx + 0x80]
1408c92fc: movaps xmmword ptr [rsp + 0x30], xmm6
1408c9301: mov byte ptr [rbp + 0x527], 0
1408c9308: lea rcx, [rbp + 0x320]
1408c930f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408c9314: mov byte ptr [rbp + 0x527], 0
1408c931b: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c9320: add rsp, 0x48
1408c9324: pop rbx
1408c9325: pop rdi
1408c9326: pop rsi
1408c9327: pop r12
1408c9329: pop r13
1408c932b: pop r14
1408c932d: pop r15
1408c932f: pop rbp
1408c9330: ret 
1408c9331: nop word ptr cs:[rax + rax]
1408c9340: mov qword ptr [rsp + 0x10], rdx
1408c9345: push rbp
1408c9346: push r15
1408c9348: push r14
1408c934a: push r13
1408c934c: push r12
1408c934e: push rsi
1408c934f: push rdi
1408c9350: push rbx
1408c9351: sub rsp, 0x48
1408c9355: lea rbp, [rdx + 0x80]
1408c935c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c9361: cmp byte ptr [rbp + 0x456], 8
1408c9368: je 0x1408c937d
1408c936a: lea rcx, [rbp + 0x440]
1408c9371: mov byte ptr [rbp + 0x527], 0
1408c9378: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1408c937d: mov byte ptr [rbp + 0x527], 0
1408c9384: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c9389: add rsp, 0x48
1408c938d: pop rbx
1408c938e: pop rdi
1408c938f: pop rsi
1408c9390: pop r12
1408c9392: pop r13
1408c9394: pop r14
1408c9396: pop r15
1408c9398: pop rbp
1408c9399: ret 
1408c939a: nop word ptr [rax + rax]
1408c93a0: mov qword ptr [rsp + 0x10], rdx
1408c93a5: push rbp
1408c93a6: push r15
1408c93a8: push r14
1408c93aa: push r13
1408c93ac: push r12
1408c93ae: push rsi
1408c93af: push rdi
1408c93b0: push rbx
1408c93b1: sub rsp, 0x48
1408c93b5: lea rbp, [rdx + 0x80]
1408c93bc: movaps xmmword ptr [rsp + 0x30], xmm6
1408c93c1: mov byte ptr [rbp + 0x527], 0
1408c93c8: lea rcx, [rbp + 0x190]
1408c93cf: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408c93d4: mov byte ptr [rbp + 0x527], 0
1408c93db: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c93e0: add rsp, 0x48
1408c93e4: pop rbx
1408c93e5: pop rdi
1408c93e6: pop rsi
1408c93e7: pop r12
1408c93e9: pop r13
1408c93eb: pop r14
1408c93ed: pop r15
1408c93ef: pop rbp
1408c93f0: ret 
1408c93f1: nop word ptr cs:[rax + rax]
1408c9400: mov qword ptr [rsp + 0x10], rdx
1408c9405: push rbp
1408c9406: push r15
1408c9408: push r14
1408c940a: push r13
1408c940c: push r12
1408c940e: push rsi
1408c940f: push rdi
1408c9410: push rbx
1408c9411: sub rsp, 0x48
1408c9415: lea rbp, [rdx + 0x80]
1408c941c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c9421: mov byte ptr [rbp + 0x527], 0
1408c9428: lea rcx, [rbp + 0x320]
1408c942f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408c9434: mov byte ptr [rbp + 0x527], 0
1408c943b: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c9440: add rsp, 0x48
1408c9444: pop rbx
1408c9445: pop rdi
1408c9446: pop rsi
1408c9447: pop r12
1408c9449: pop r13
1408c944b: pop r14
1408c944d: pop r15
1408c944f: pop rbp
1408c9450: ret 
1408c9451: nop word ptr cs:[rax + rax]
1408c9460: mov qword ptr [rsp + 0x10], rdx
1408c9465: push rbp
1408c9466: push r15
1408c9468: push r14
1408c946a: push r13
1408c946c: push r12
1408c946e: push rsi
1408c946f: push rdi
1408c9470: push rbx
1408c9471: sub rsp, 0x48
1408c9475: lea rbp, [rdx + 0x80]
1408c947c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c9481: mov byte ptr [rbp + 0x527], 1
1408c9488: lea rcx, [rbp + 0x190]
1408c948f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408c9494: mov byte ptr [rbp + 0x527], 1
1408c949b: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c94a0: add rsp, 0x48
1408c94a4: pop rbx
1408c94a5: pop rdi
1408c94a6: pop rsi
1408c94a7: pop r12
1408c94a9: pop r13
1408c94ab: pop r14
1408c94ad: pop r15
1408c94af: pop rbp
1408c94b0: ret 
1408c94b1: nop word ptr cs:[rax + rax]
1408c94c0: mov qword ptr [rsp + 0x10], rdx
1408c94c5: push rbp
1408c94c6: push r15
1408c94c8: push r14
1408c94ca: push r13
1408c94cc: push r12
1408c94ce: push rsi
1408c94cf: push rdi
1408c94d0: push rbx
1408c94d1: sub rsp, 0x48
1408c94d5: lea rbp, [rdx + 0x80]
1408c94dc: movaps xmmword ptr [rsp + 0x30], xmm6
1408c94e1: mov byte ptr [rbp + 0x527], 1
1408c94e8: lea rcx, [rbp + 0x320]
1408c94ef: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408c94f4: mov byte ptr [rbp + 0x527], 1
1408c94fb: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c9500: add rsp, 0x48
1408c9504: pop rbx
1408c9505: pop rdi
1408c9506: pop rsi
1408c9507: pop r12
1408c9509: pop r13
1408c950b: pop r14
1408c950d: pop r15
1408c950f: pop rbp
1408c9510: ret 
1408c9511: nop word ptr cs:[rax + rax]
1408c9520: mov qword ptr [rsp + 0x10], rdx
1408c9525: push rbp
1408c9526: push r15
1408c9528: push r14
1408c952a: push r13
1408c952c: push r12
1408c952e: push rsi
1408c952f: push rdi
1408c9530: push rbx
1408c9531: sub rsp, 0x48
1408c9535: lea rbp, [rdx + 0x80]
1408c953c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c9541: mov byte ptr [rbp + 0x527], 1
1408c9548: lea rcx, [rbp + 0x190]
1408c954f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408c9554: mov byte ptr [rbp + 0x527], 1
1408c955b: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c9560: add rsp, 0x48
1408c9564: pop rbx
1408c9565: pop rdi
1408c9566: pop rsi
1408c9567: pop r12
1408c9569: pop r13
1408c956b: pop r14
1408c956d: pop r15
1408c956f: pop rbp
1408c9570: ret 
1408c9571: nop word ptr cs:[rax + rax]
1408c9580: mov qword ptr [rsp + 0x10], rdx
1408c9585: push rbp
1408c9586: push r15
1408c9588: push r14
1408c958a: push r13
1408c958c: push r12
1408c958e: push rsi
1408c958f: push rdi
1408c9590: push rbx
1408c9591: sub rsp, 0x48
1408c9595: lea rbp, [rdx + 0x80]
1408c959c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c95a1: mov byte ptr [rbp + 0x527], 1
1408c95a8: lea rcx, [rbp + 0x320]
1408c95af: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408c95b4: mov byte ptr [rbp + 0x527], 1
1408c95bb: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c95c0: add rsp, 0x48
1408c95c4: pop rbx
1408c95c5: pop rdi
1408c95c6: pop rsi
1408c95c7: pop r12
1408c95c9: pop r13
1408c95cb: pop r14
1408c95cd: pop r15
1408c95cf: pop rbp
1408c95d0: ret 
1408c95d1: nop word ptr cs:[rax + rax]
1408c95e0: mov qword ptr [rsp + 0x10], rdx
1408c95e5: push rbp
1408c95e6: push r15
1408c95e8: push r14
1408c95ea: push r13
1408c95ec: push r12
1408c95ee: push rsi
1408c95ef: push rdi
1408c95f0: push rbx
1408c95f1: sub rsp, 0x48
1408c95f5: lea rbp, [rdx + 0x80]
1408c95fc: movaps xmmword ptr [rsp + 0x30], xmm6
1408c9601: mov byte ptr [rbp + 0x527], 1
1408c9608: lea rcx, [rbp + 0x190]
1408c960f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408c9614: mov byte ptr [rbp + 0x527], 1
1408c961b: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c9620: add rsp, 0x48
1408c9624: pop rbx
1408c9625: pop rdi
1408c9626: pop rsi
1408c9627: pop r12
1408c9629: pop r13
1408c962b: pop r14
1408c962d: pop r15
1408c962f: pop rbp
1408c9630: ret 
1408c9631: nop word ptr cs:[rax + rax]
1408c9640: mov qword ptr [rsp + 0x10], rdx
1408c9645: push rbp
1408c9646: push r15
1408c9648: push r14
1408c964a: push r13
1408c964c: push r12
1408c964e: push rsi
1408c964f: push rdi
1408c9650: push rbx
1408c9651: sub rsp, 0x48
1408c9655: lea rbp, [rdx + 0x80]
1408c965c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c9661: cmp byte ptr [rbp + 0x525], 0
1408c9668: je 0x1408c96a1
1408c966a: movdqa xmm0, xmmword ptr [rbp + 0x1b0]
1408c9672: movaps xmm1, xmmword ptr [rbp + 0x1c0]
1408c9679: movaps xmmword ptr [rbp - 0x30], xmm1
1408c967d: movdqa xmmword ptr [rbp - 0x40], xmm0
1408c9682: lea rcx, [rbp - 0x40]
1408c9686: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1408c968b: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c9690: add rsp, 0x48
1408c9694: pop rbx
1408c9695: pop rdi
1408c9696: pop rsi
1408c9697: pop r12
1408c9699: pop r13
1408c969b: pop r14
1408c969d: pop r15
1408c969f: pop rbp
1408c96a0: ret 
1408c96a1: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c96a6: add rsp, 0x48
1408c96aa: pop rbx
1408c96ab: pop rdi
1408c96ac: pop rsi
1408c96ad: pop r12
1408c96af: pop r13
1408c96b1: pop r14
1408c96b3: pop r15
1408c96b5: pop rbp
1408c96b6: ret 
1408c96b7: nop word ptr [rax + rax]
1408c96c0: mov qword ptr [rsp + 0x10], rdx
1408c96c5: push rbp
1408c96c6: push r15
1408c96c8: push r14
1408c96ca: push r13
1408c96cc: push r12
1408c96ce: push rsi
1408c96cf: push rdi
1408c96d0: push rbx
1408c96d1: sub rsp, 0x48
1408c96d5: lea rbp, [rdx + 0x80]
1408c96dc: movaps xmmword ptr [rsp + 0x30], xmm6
1408c96e1: movups xmm0, xmmword ptr [rbp + 0x160]
1408c96e8: movups xmm1, xmmword ptr [rbp + 0x170]
1408c96ef: movaps xmmword ptr [rbp + 0x330], xmm1
1408c96f6: movaps xmmword ptr [rbp + 0x320], xmm0
1408c96fd: mov byte ptr [rbp + 0x526], 0
1408c9704: lea rcx, [rbp + 0x320]
1408c970b: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1408c9710: mov byte ptr [rbp + 0x526], 0
1408c9717: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c971c: add rsp, 0x48
1408c9720: pop rbx
1408c9721: pop rdi
1408c9722: pop rsi
1408c9723: pop r12
1408c9725: pop r13
1408c9727: pop r14
1408c9729: pop r15
1408c972b: pop rbp
1408c972c: ret 
1408c972d: nop dword ptr [rax]
1408c9730: mov qword ptr [rsp + 0x10], rdx
1408c9735: push rbp
1408c9736: push r15
1408c9738: push r14
1408c973a: push r13
1408c973c: push r12
1408c973e: push rsi
1408c973f: push rdi
1408c9740: push rbx
1408c9741: sub rsp, 0x48
1408c9745: lea rbp, [rdx + 0x80]
1408c974c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c9751: mov rcx, qword ptr [rbp + 0x518]
1408c9758: call 0x140161c30 ; core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo> >
1408c975d: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c9762: add rsp, 0x48
1408c9766: pop rbx
1408c9767: pop rdi
1408c9768: pop rsi
1408c9769: pop r12
1408c976b: pop r13
1408c976d: pop r14
1408c976f: pop r15
1408c9771: pop rbp
1408c9772: ret 
1408c9773: nop word ptr cs:[rax + rax]
1408c9780: mov qword ptr [rsp + 0x10], rdx
1408c9785: push rbp
1408c9786: push r15
1408c9788: push r14
1408c978a: push r13
1408c978c: push r12
1408c978e: push rsi
1408c978f: push rdi
1408c9790: push rbx
1408c9791: sub rsp, 0x48
1408c9795: lea rbp, [rdx + 0x80]
1408c979c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c97a1: movdqu xmm0, xmmword ptr [rbp + 0x340]
1408c97a9: movups xmm1, xmmword ptr [rbp + 0x350]
1408c97b0: movaps xmmword ptr [rbp + 0x1a0], xmm1
1408c97b7: movdqa xmmword ptr [rbp + 0x190], xmm0
1408c97bf: lea rcx, [rbp + 0x190]
1408c97c6: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1408c97cb: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c97d0: add rsp, 0x48
1408c97d4: pop rbx
1408c97d5: pop rdi
1408c97d6: pop rsi
1408c97d7: pop r12
1408c97d9: pop r13
1408c97db: pop r14
1408c97dd: pop r15
1408c97df: pop rbp
1408c97e0: ret 
1408c97e1: nop word ptr cs:[rax + rax]
1408c97f0: mov qword ptr [rsp + 0x10], rdx
1408c97f5: push rbp
1408c97f6: push r15
1408c97f8: push r14
1408c97fa: push r13
1408c97fc: push r12
1408c97fe: push rsi
1408c97ff: push rdi
1408c9800: push rbx
1408c9801: sub rsp, 0x48
1408c9805: lea rbp, [rdx + 0x80]
1408c980c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c9811: mov rcx, qword ptr [rbp + 0x518]
1408c9818: call 0x140161c30 ; core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo> >
1408c981d: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c9822: add rsp, 0x48
1408c9826: pop rbx
1408c9827: pop rdi
1408c9828: pop rsi
1408c9829: pop r12
1408c982b: pop r13
1408c982d: pop r14
1408c982f: pop r15
1408c9831: pop rbp
1408c9832: ret 
1408c9833: nop word ptr cs:[rax + rax]
1408c9840: mov qword ptr [rsp + 0x10], rdx
1408c9845: push rbp
1408c9846: push r15
1408c9848: push r14
1408c984a: push r13
1408c984c: push r12
1408c984e: push rsi
1408c984f: push rdi
1408c9850: push rbx
1408c9851: sub rsp, 0x48
1408c9855: lea rbp, [rdx + 0x80]
1408c985c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c9861: cmp byte ptr [rbp + 0x526], 0
1408c9868: je 0x1408c98a1
1408c986a: movdqu xmm0, xmmword ptr [rbp + 0x1b0]
1408c9872: movups xmm1, xmmword ptr [rbp + 0x1c0]
1408c9879: movaps xmmword ptr [rbp - 0x30], xmm1
1408c987d: movdqa xmmword ptr [rbp - 0x40], xmm0
1408c9882: lea rcx, [rbp - 0x40]
1408c9886: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1408c988b: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c9890: add rsp, 0x48
1408c9894: pop rbx
1408c9895: pop rdi
1408c9896: pop rsi
1408c9897: pop r12
1408c9899: pop r13
1408c989b: pop r14
1408c989d: pop r15
1408c989f: pop rbp
1408c98a0: ret 
1408c98a1: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c98a6: add rsp, 0x48
1408c98aa: pop rbx
1408c98ab: pop rdi
1408c98ac: pop rsi
1408c98ad: pop r12
1408c98af: pop r13
1408c98b1: pop r14
1408c98b3: pop r15
1408c98b5: pop rbp
1408c98b6: ret 
1408c98b7: nop word ptr [rax + rax]
1408c98c0: mov qword ptr [rsp + 0x10], rdx
1408c98c5: push rbp
1408c98c6: push r15
1408c98c8: push r14
1408c98ca: push r13
1408c98cc: push r12
1408c98ce: push rsi
1408c98cf: push rdi
1408c98d0: push rbx
1408c98d1: sub rsp, 0x48
1408c98d5: lea rbp, [rdx + 0x80]
1408c98dc: movaps xmmword ptr [rsp + 0x30], xmm6
1408c98e1: cmp qword ptr [rbp + 0x4b8], 0
1408c98e9: je 0x1408c9914
1408c98eb: lea rax, [rbp + 0x4c0]
1408c98f2: mov qword ptr [rbp + 0x150], rax
1408c98f9: lea rax, [rbp + 0x150]
1408c9900: mov qword ptr [rbp - 0x40], rax
1408c9904: lea rcx, [rip + 0x21d944d]
1408c990b: lea rdx, [rbp - 0x40]
1408c990f: call 0x14132f2d0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
1408c9914: mov byte ptr [rbp + 0x527], 1
1408c991b: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c9920: add rsp, 0x48
1408c9924: pop rbx
1408c9925: pop rdi
1408c9926: pop rsi
1408c9927: pop r12
1408c9929: pop r13
1408c992b: pop r14
1408c992d: pop r15
1408c992f: pop rbp
1408c9930: ret 
1408c9931: nop word ptr cs:[rax + rax]
1408c9940: mov qword ptr [rsp + 0x10], rdx
1408c9945: push rbp
1408c9946: push r15
1408c9948: push r14
1408c994a: push r13
1408c994c: push r12
1408c994e: push rsi
1408c994f: push rdi
1408c9950: push rbx
1408c9951: sub rsp, 0x48
1408c9955: lea rbp, [rdx + 0x80]
1408c995c: movaps xmmword ptr [rsp + 0x30], xmm6
1408c9961: movzx ebx, byte ptr [rbp + 0x527]
1408c9968: mov rdx, qword ptr [rbp + 0x4e0]
1408c996f: test rdx, rdx
1408c9972: je 0x1408c998a
1408c9974: mov rcx, qword ptr [rbp + 0x4e8]
1408c997b: shl rdx, 4
1408c997f: mov r8d, 8
1408c9985: call 0x140613c20 ; ?
1408c998a: test bl, 1
1408c998d: jne 0x1408c99a5
1408c998f: movaps xmm6, xmmword ptr [rsp + 0x30]
1408c9994: add rsp, 0x48
1408c9998: pop rbx
1408c9999: pop rdi
1408c999a: pop rsi
1408c999b: pop r12
1408c999d: pop r13
1408c999f: pop r14
1408c99a1: pop r15
1408c99a3: pop rbp
1408c99a4: ret 
1408c99a5: lea rcx, [rbp + 0x4f8]
1408c99ac: call 0x1408826b0 ; core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::raw::managed::handle::GpuOwned<tuple$<>,rhapsody::raw::managed::handle::ManagedHandle<rhapsody::raw::managed::buffer::ManagedBuffer> >,alloc::alloc::Global> >
1408c99b1: jmp 0x1408c998f
