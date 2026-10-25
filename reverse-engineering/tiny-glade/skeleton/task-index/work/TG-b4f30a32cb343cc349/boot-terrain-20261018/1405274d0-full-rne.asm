1405274d0: push rbp
1405274d1: push r15
1405274d3: push r14
1405274d5: push r12
1405274d7: push rsi
1405274d8: push rdi
1405274d9: push rbx
1405274da: sub rsp, 0x280
1405274e1: lea rbp, [rsp + 0x80]
1405274e9: mov qword ptr [rbp + 0x1f8], 0xfffffffffffffffe
1405274f4: mov rsi, rdx
1405274f7: movdqu xmm0, xmmword ptr [rcx]
1405274fb: movdqu xmm1, xmmword ptr [rcx + 0x10]
140527500: movdqu xmm2, xmmword ptr [rcx + 0x20]
140527505: movdqa xmmword ptr [rbp + 0x160], xmm2
14052750d: movdqa xmmword ptr [rbp + 0x150], xmm1
140527515: movdqa xmmword ptr [rbp + 0x140], xmm0
14052751d: lea rdi, [rdx + 0x38]
140527521: add rdx, 0x5a8
140527528: mov qword ptr [rbp + 0x1d8], rdi
14052752f: mov qword ptr [rbp + 0x1e0], rdx
140527536: cmp qword ptr [rsi + 0x88], 0
14052753e: movabs rax, 0xe6222c4af0fcfd90
140527548: movabs rcx, 0x1e9ac9b6648cf0a6
140527552: je 0x1405275ea
140527558: mov r8, qword ptr [rsi + 0x70]
14052755c: mov r9, qword ptr [rsi + 0x78]
140527560: mov r10, r9
140527563: and r10, rax
140527566: xor r11d, r11d
140527569: movdqa xmm0, xmmword ptr [rip + 0x243a17f]
140527571: pcmpeqd xmm1, xmm1
140527575: movdqu xmm2, xmmword ptr [r8 + r10]
14052757b: movdqa xmm3, xmm2
14052757f: pcmpeqb xmm3, xmm0
140527583: pmovmskb ebx, xmm3
140527587: test ebx, ebx
140527589: jne 0x1405275bd
14052758b: pcmpeqb xmm2, xmm1
14052758f: pmovmskb ebx, xmm2
140527593: test ebx, ebx
140527595: jne 0x1405275ea
140527597: add r10, r11
14052759a: add r10, 0x10
14052759e: add r11, 0x10
1405275a2: and r10, r9
1405275a5: jmp 0x140527575 ; bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::resources::stairs::stairs_top_down::StairsTopDownMesh>,tuple$<> >
1405275a7: nop word ptr [rax + rax]
1405275b0: lea r14d, [rbx - 1]
1405275b4: and r14w, bx
1405275b8: mov ebx, r14d
1405275bb: je 0x14052758b
1405275bd: tzcnt r14d, ebx
1405275c2: add r14, r10
1405275c5: and r14, r9
1405275c8: neg r14
1405275cb: lea r14, [r14 + r14*2]
1405275cf: cmp qword ptr [r8 + r14*8 - 0x18], rcx
1405275d4: jne 0x1405275b0
1405275d6: cmp qword ptr [r8 + r14*8 - 0x10], rax
1405275db: jne 0x1405275b0
1405275dd: lea rax, [r8 + r14*8]
1405275e1: mov rbx, qword ptr [rax - 8]
1405275e5: jmp 0x1405278e4 ; bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::resources::stairs::stairs_top_down::StairsTopDownMesh>,tuple$<> >
1405275ea: movzx r8d, byte ptr [rsi + 0x98]
1405275f2: mov r8, qword ptr [rsi + 0xd8]
1405275f9: mov r10, qword ptr [rsi + 0xe0]
140527600: mov r11, r10
140527603: and r11, rax
140527606: xor ebx, ebx
140527608: movdqa xmm0, xmmword ptr [rip + 0x243a0e0]
140527610: pcmpeqd xmm1, xmm1
140527614: movdqu xmm2, xmmword ptr [r8 + r11]
14052761a: movdqa xmm3, xmm2
14052761e: pcmpeqb xmm3, xmm0
140527622: pmovmskb r14d, xmm3
140527627: test r14d, r14d
14052762a: jne 0x14052765d
14052762c: pcmpeqb xmm2, xmm1
140527630: pmovmskb r9d, xmm2
140527635: test r9d, r9d
140527638: jne 0x14052772d
14052763e: add r11, rbx
140527641: add r11, 0x10
140527645: add rbx, 0x10
140527649: and r11, r10
14052764c: jmp 0x140527614 ; bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::resources::stairs::stairs_top_down::StairsTopDownMesh>,tuple$<> >
14052764e: nop 
140527650: lea r9d, [r14 - 1]
140527654: and r9w, r14w
140527658: mov r14d, r9d
14052765b: je 0x14052762c
14052765d: tzcnt r15d, r14d
140527662: add r15, r11
140527665: and r15, r10
140527668: mov r9, r15
14052766b: neg r9
14052766e: lea r12, [r15*8]
140527676: shl r9, 7
14052767a: sub r9, r12
14052767d: cmp qword ptr [r8 + r9 - 0x88], rcx
140527685: jne 0x140527650
140527687: cmp qword ptr [r8 + r9 - 0x80], rax
14052768c: jne 0x140527650
14052768e: mov r11, r15
140527691: shl r11, 7
140527695: lea rbx, [r11 + r15*8]
140527699: sar rbx, 3
14052769d: movabs r11, 0xf0f0f0f0f0f0f0f1
1405276a7: imul r11, rbx
1405276ab: lea rbx, [r11 - 0x10]
1405276af: and rbx, r10
1405276b2: movdqu xmm0, xmmword ptr [r8 + rbx]
1405276b8: pcmpeqd xmm1, xmm1
1405276bc: pcmpeqb xmm0, xmm1
1405276c0: pmovmskb r14d, xmm0
1405276c5: movdqu xmm0, xmmword ptr [r8 + r11]
1405276cb: pcmpeqb xmm0, xmm1
1405276cf: pmovmskb r10d, xmm0
1405276d4: test r14d, r14d
1405276d7: je 0x1405276e4
1405276d9: bsr r14w, r14w
1405276de: xor r14d, 0xf
1405276e2: jmp 0x1405276e9 ; bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::resources::stairs::stairs_top_down::StairsTopDownMesh>,tuple$<> >
1405276e4: mov r14w, 0x10
1405276e9: add r9, r8
1405276ec: or r10d, 0x10000
1405276f3: tzcnt r10d, r10d
1405276f8: add r14d, r10d
1405276fb: mov r10b, 0x80
1405276fe: cmp r14w, 0xf
140527703: ja 0x14052770f
140527705: inc qword ptr [rsi + 0xe8]
14052770c: mov r10b, 0xff
14052770f: mov byte ptr [r8 + r11], r10b
140527713: mov byte ptr [r8 + rbx + 0x10], r10b
140527718: dec qword ptr [rsi + 0xf0]
14052771f: mov r8, qword ptr [r9 - 0x78]
140527723: cmp r8, 2
140527727: jne 0x14052786d
14052772d: mov rbx, qword ptr [rdx]
140527730: lea r14, [rbx + 1]
140527734: mov qword ptr [rdx], r14
140527737: movabs rdx, 0x8000000000000000
140527741: mov qword ptr [rbp + 0x1b0], rdx
140527748: lea rdx, [rip + 0x242b095]
14052774f: mov qword ptr [rbp + 0x1b8], rdx
140527756: mov qword ptr [rbp + 0x1c0], 0x43
140527761: mov word ptr [rbp + 0x1d0], 0x100
14052776a: mov qword ptr [rbp + 0x178], 1
140527775: mov qword ptr [rbp + 0x180], rcx
14052777c: mov qword ptr [rbp + 0x188], rax
140527783: mov qword ptr [rbp + 0x1a0], 8
14052778e: mov qword ptr [rbp + 0x1a8], 0x30
140527799: lea rax, [rip - 0x3dc0a0]
1405277a0: mov qword ptr [rbp + 0x1c8], rax
1405277a7: mov byte ptr [rbp + 0x1d2], 1
1405277ae: mov qword ptr [rbp + 0x190], 0
1405277b9: mov byte ptr [rbp + 0x1f7], 1
1405277c0: lea rcx, [rbp - 0x60]
1405277c4: lea r8, [rbp + 0x178]
1405277cb: mov rdx, rbx
1405277ce: call 0x140d3f870 ; bevy_ecs::component::ComponentInfo::new
1405277d3: cmp qword ptr [rsi + 0x48], r14
1405277d7: jae 0x1405277f2
1405277d9: mov byte ptr [rbp + 0x1f6], 1
1405277e0: lea r8, [rip + 0x24ca7d1]
1405277e7: mov rcx, rdi
1405277ea: mov rdx, r14
1405277ed: call 0x1402b5690 ; alloc::vec::Vec::resize_with<enum2$<core::option::Option<bevy_ecs::component::ComponentInfo> >,alloc::alloc::Global,bevy_ecs::component::impl$19::register_component_inner::closure_env$0>
1405277f2: mov r14, qword ptr [rsi + 0x40]
1405277f6: imul r15, rbx, 0xd0
1405277fd: lea rax, [r14 + r15]
140527801: mov qword ptr [rbp + 0x1e8], rax
140527808: lea rcx, [rbp + 0x70]
14052780c: lea rdx, [rbp - 0x60]
140527810: mov r8d, 0xd0
140527816: call 0x142923c00 ; ?
14052781b: cmp dword ptr [r14 + r15], 2
140527820: je 0x14052782e
140527822: mov rcx, qword ptr [rbp + 0x1e8]
140527829: call 0x1405f0b50 ; core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo>
14052782e: lea rdx, [rbp + 0x70]
140527832: mov r8d, 0xd0
140527838: mov rcx, qword ptr [rbp + 0x1e8]
14052783f: call 0x142923c00 ; ?
140527844: lea rcx, [rsi + 0x70]
140527848: mov byte ptr [rbp + 0x1f7], 1
14052784f: movabs rdx, 0x1e9ac9b6648cf0a6
140527859: movabs r8, 0xe6222c4af0fcfd90
140527863: mov r9, rbx
140527866: call 0x140082c20 ; hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global>
14052786b: jmp 0x1405278e4 ; bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::resources::stairs::stairs_top_down::StairsTopDownMesh>,tuple$<> >
14052786d: movups xmm0, xmmword ptr [r9 - 0x10]
140527872: movups xmmword ptr [rbp + 0xd8], xmm0
140527879: movups xmm0, xmmword ptr [r9 - 0x20]
14052787e: movups xmmword ptr [rbp + 0xc8], xmm0
140527885: movups xmm0, xmmword ptr [r9 - 0x30]
14052788a: movups xmmword ptr [rbp + 0xb8], xmm0
140527891: movdqu xmm0, xmmword ptr [r9 - 0x70]
140527897: movdqu xmm1, xmmword ptr [r9 - 0x60]
14052789d: movdqu xmm2, xmmword ptr [r9 - 0x50]
1405278a3: movdqu xmm3, xmmword ptr [r9 - 0x40]
1405278a9: movdqu xmmword ptr [rbp + 0xa8], xmm3
1405278b1: movdqu xmmword ptr [rbp + 0x98], xmm2
1405278b9: movdqu xmmword ptr [rbp + 0x88], xmm1
1405278c1: movdqu xmmword ptr [rbp + 0x78], xmm0
1405278c6: mov qword ptr [rbp + 0x70], r8
1405278ca: mov byte ptr [rbp + 0x1f7], 1
1405278d1: lea rcx, [rbp + 0x70]
1405278d5: lea rdx, [rbp + 0x1d8]
1405278dc: call 0x140d3fa40 ; bevy_ecs::component::QueuedRegistration::register
1405278e1: mov rbx, rax
1405278e4: movdqa xmm0, xmmword ptr [rbp + 0x140]
1405278ec: movdqa xmm1, xmmword ptr [rbp + 0x150]
1405278f4: movdqa xmm2, xmmword ptr [rbp + 0x160]
1405278fc: movdqa xmmword ptr [rbp + 0x90], xmm2
140527904: movdqa xmmword ptr [rbp + 0x80], xmm1
14052790c: movdqa xmmword ptr [rbp + 0x70], xmm0
140527911: mov r14d, dword ptr [rsi + 0x5b0]
140527918: mov byte ptr [rbp + 0x1f7], 0
14052791f: mov rcx, rsi
140527922: call 0x140cf8250 ; bevy_ecs::world::World::flush_components
140527927: lea rax, [rsi + 0xf8]
14052792e: mov qword ptr [rbp + 0x178], rbx
140527935: add rsi, 0x1d8
14052793c: mov qword ptr [rbp - 0x60], rdi
140527940: lea rcx, [rbp + 0x178]
140527947: mov qword ptr [rbp - 0x58], rcx
14052794b: mov qword ptr [rbp - 0x50], rax
14052794f: mov byte ptr [rbp + 0x1f7], 0
140527956: lea r8, [rbp - 0x60]
14052795a: mov rcx, rsi
14052795d: mov rdx, rbx
140527960: call 0x140509180 ; bevy_ecs::storage::sparse_set::SparseSet::get_or_insert_with<bevy_ecs::component::ComponentId,bevy_ecs::storage::resource::ResourceData<true>,bevy_ecs::storage::resource::impl$2::initialize_with::closure_env$0<true,bevy_ecs::world::impl$2::initialize_resource_internal::closure_env$0> >
140527965: mov rsi, rax
140527968: cmp qword ptr [rax + 0x20], 0
14052796d: je 0x140527986
14052796f: mov byte ptr [rbp + 0x1f7], 0
140527976: lea r8, [rbp + 0x70]
14052797a: mov rcx, rsi
14052797d: xor edx, edx
14052797f: call 0x140cff670 ; bevy_ecs::storage::blob_vec::BlobVec::replace_unchecked
140527984: jmp 0x1405279b0 ; bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::resources::stairs::stairs_top_down::StairsTopDownMesh>,tuple$<> >
140527986: cmp qword ptr [rsi + 0x18], 0
14052798b: je 0x1405279c6
14052798d: xor ecx, ecx
14052798f: lea rax, [rcx + 1]
140527993: mov qword ptr [rsi + 0x20], rax
140527997: mov r8, qword ptr [rsi + 8]
14052799b: imul rcx, r8
14052799f: add rcx, qword ptr [rsi + 0x10]
1405279a3: lea rdx, [rbp + 0x70]
1405279a7: call 0x142923c00 ; ?
1405279ac: mov dword ptr [rsi + 0x58], r14d
1405279b0: mov dword ptr [rsi + 0x5c], r14d
1405279b4: add rsp, 0x280
1405279bb: pop rbx
1405279bc: pop rdi
1405279bd: pop rsi
1405279be: pop r12
1405279c0: pop r14
1405279c2: pop r15
1405279c4: pop rbp
1405279c5: ret 
1405279c6: mov byte ptr [rbp + 0x1f7], 0
1405279cd: mov edx, 1
1405279d2: mov rcx, rsi
1405279d5: call 0x1428cec40 ; bevy_ecs::storage::blob_vec::impl$1::reserve::do_reserve
1405279da: mov rcx, qword ptr [rsi + 0x20]
1405279de: jmp 0x14052798f ; bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::system::commands::command::insert_resource::closure_env$0<country_core::resources::stairs::stairs_top_down::StairsTopDownMesh>,tuple$<> >
1405279e0: mov qword ptr [rsp + 0x10], rdx
1405279e5: push rbp
1405279e6: push r15
1405279e8: push r14
1405279ea: push r12
1405279ec: push rsi
1405279ed: push rdi
1405279ee: push rbx
1405279ef: sub rsp, 0x20
1405279f3: lea rbp, [rdx + 0x80]
1405279fa: lea rdx, [rbp + 0x70]
1405279fe: mov r8d, 0xd0
140527a04: mov rcx, qword ptr [rbp + 0x1e8]
140527a0b: call 0x142923c00 ; ?
140527a10: mov byte ptr [rbp + 0x1f6], 0
140527a17: add rsp, 0x20
140527a1b: pop rbx
140527a1c: pop rdi
140527a1d: pop rsi
140527a1e: pop r12
140527a20: pop r14
140527a22: pop r15
140527a24: pop rbp
140527a25: ret 
140527a26: nop word ptr cs:[rax + rax]
140527a30: mov qword ptr [rsp + 0x10], rdx
140527a35: push rbp
140527a36: push r15
140527a38: push r14
140527a3a: push r12
140527a3c: push rsi
140527a3d: push rdi
140527a3e: push rbx
140527a3f: sub rsp, 0x20
140527a43: lea rbp, [rdx + 0x80]
140527a4a: cmp byte ptr [rbp + 0x1f6], 0
140527a51: je 0x140527a63
140527a53: mov byte ptr [rbp + 0x1f7], 1
140527a5a: lea rcx, [rbp - 0x60]
140527a5e: call 0x1405f0b50 ; core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo>
140527a63: mov byte ptr [rbp + 0x1f7], 1
140527a6a: add rsp, 0x20
140527a6e: pop rbx
140527a6f: pop rdi
140527a70: pop rsi
140527a71: pop r12
140527a73: pop r14
140527a75: pop r15
140527a77: pop rbp
140527a78: ret 
140527a79: nop dword ptr [rax]
140527a80: mov qword ptr [rsp + 0x10], rdx
140527a85: push rbp
140527a86: push r15
140527a88: push r14
140527a8a: push r12
140527a8c: push rsi
140527a8d: push rdi
140527a8e: push rbx
140527a8f: sub rsp, 0x20
140527a93: lea rbp, [rdx + 0x80]
140527a9a: cmp byte ptr [rbp + 0x1f7], 0
140527aa1: je 0x140527aaf
140527aa3: lea rcx, [rbp + 0x140]
140527aaa: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140527aaf: nop 
140527ab0: add rsp, 0x20
140527ab4: pop rbx
140527ab5: pop rdi
140527ab6: pop rsi
140527ab7: pop r12
140527ab9: pop r14
140527abb: pop r15
140527abd: pop rbp
140527abe: ret 
