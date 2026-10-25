14206ba60: 55 push rbp
14206ba61: 56 push rsi
14206ba62: 57 push rdi
14206ba63: 4881ec50040000 sub rsp, 0x450
14206ba6a: 488dac2480000000 lea rbp, [rsp + 0x80]
14206ba72: 48c785c8030000feffffff mov qword ptr [rbp + 0x3c8], 0xfffffffffffffffe
14206ba7d: 4889ce mov rsi, rcx
14206ba80: 0f57c0 xorps xmm0, xmm0
14206ba83: 0f1145c8 movups xmmword ptr [rbp - 0x38], xmm0
14206ba87: 0f1145e0 movups xmmword ptr [rbp - 0x20], xmm0
14206ba8b: 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
14206ba8f: 0f114510 movups xmmword ptr [rbp + 0x10], xmm0
14206ba93: 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
14206ba97: c7453800000000 mov dword ptr [rbp + 0x38], 0
14206ba9e: 0f114560 movups xmmword ptr [rbp + 0x60], xmm0
14206baa2: 0f114578 movups xmmword ptr [rbp + 0x78], xmm0
14206baa6: 0f118590000000 movups xmmword ptr [rbp + 0x90], xmm0
14206baad: 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
14206bab4: 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
14206babb: c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
14206bac5: 48c745a800000000 mov qword ptr [rbp - 0x58], 0
14206bacd: 48c745b008000000 mov qword ptr [rbp - 0x50], 8
14206bad5: 48c745b800000000 mov qword ptr [rbp - 0x48], 0
14206badd: 48c745c010000000 mov qword ptr [rbp - 0x40], 0x10
14206bae5: 48c745d810000000 mov qword ptr [rbp - 0x28], 0x10
14206baed: 48c745f010000000 mov qword ptr [rbp - 0x10], 0x10
14206baf5: 48c7450810000000 mov qword ptr [rbp + 8], 0x10
14206bafd: 48c7452010000000 mov qword ptr [rbp + 0x20], 0x10
14206bb05: 48b80000000000000080 movabs rax, 0x8000000000000000
14206bb0f: 48894540 mov qword ptr [rbp + 0x40], rax
14206bb13: 488d0d0b6df600 lea rcx, [rip + 0xf66d0b]
14206bb1a: 48894d48 mov qword ptr [rbp + 0x48], rcx
14206bb1e: 48c745503c000000 mov qword ptr [rbp + 0x50], 0x3c
14206bb26: 48c7455810000000 mov qword ptr [rbp + 0x58], 0x10
14206bb2e: 48c7457010000000 mov qword ptr [rbp + 0x70], 0x10
14206bb36: 48c7858800000010000000 mov qword ptr [rbp + 0x88], 0x10
14206bb41: 48c785a000000010000000 mov qword ptr [rbp + 0xa0], 0x10
14206bb4c: 48c785b800000010000000 mov qword ptr [rbp + 0xb8], 0x10
14206bb57: c785d800000000000000 mov dword ptr [rbp + 0xd8], 0
14206bb61: 66c785dc0000000100 mov word ptr [rbp + 0xdc], 1
14206bb6a: 488985d0010000 mov qword ptr [rbp + 0x1d0], rax
14206bb71: c785c003000000000000 mov dword ptr [rbp + 0x3c0], 0
14206bb7b: 0fb60587905c01 movzx eax, byte ptr [rip + 0x15c9087]
14206bb82: b920040000 mov ecx, 0x420
14206bb87: ba08000000 mov edx, 8
14206bb8c: e87f805afe call 0x140613c10 ; unknown
14206bb91: 4885c0 test rax, rax
14206bb94: 7435 je 0x14206bbcb
14206bb96: 4889c7 mov rdi, rax
14206bb99: 488d55a8 lea rdx, [rbp - 0x58]
14206bb9d: 41b820040000 mov r8d, 0x420
14206bba3: 4889c1 mov rcx, rax
14206bba6: e855808b00 call 0x142923c00 ; unknown
14206bbab: 4c8d053e69f600 lea r8, [rip + 0xf6693e]
14206bbb2: 4889f1 mov rcx, rsi
14206bbb5: 4889fa mov rdx, rdi
14206bbb8: e8538fc7fe call 0x140ce4b10 ; bevy_ecs::schedule::config::impl$0::into_config
14206bbbd: 4889f0 mov rax, rsi
14206bbc0: 4881c450040000 add rsp, 0x450
14206bbc7: 5f pop rdi
14206bbc8: 5e pop rsi
14206bbc9: 5d pop rbp
14206bbca: c3 ret 
14206bbcb: b908000000 mov ecx, 8
14206bbd0: ba20040000 mov edx, 0x420
14206bbd5: e826d48600 call 0x1428d9000 ; alloc::alloc::handle_alloc_error
14206bbda: 0f0b ud2 
14206bbdc: 0f1f4000 nop dword ptr [rax]
14206bbe0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14206bbe5: 55 push rbp
14206bbe6: 56 push rsi
14206bbe7: 57 push rdi
14206bbe8: 4883ec20 sub rsp, 0x20
14206bbec: 488daa80000000 lea rbp, [rdx + 0x80]
14206bbf3: 488d4da8 lea rcx, [rbp - 0x58]
14206bbf7: e824fc80fe call 0x14087b820 ; core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$
14206bbfc: 90 nop 
14206bbfd: 4883c420 add rsp, 0x20
14206bc01: 5f pop rdi
14206bc02: 5e pop rsi
14206bc03: 5d pop rbp
14206bc04: c3 ret 
