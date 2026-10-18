1415f9210 55 push rbp
1415f9211 4157 push r15
1415f9213 4156 push r14
1415f9215 4155 push r13
1415f9217 4154 push r12
1415f9219 56 push rsi
1415f921a 57 push rdi
1415f921b 53 push rbx
1415f921c 4883ec68 sub rsp, 0x68
1415f9220 488d6c2460 lea rbp, [rsp + 0x60]
1415f9225 48c74500feffffff mov qword ptr [rbp], 0xfffffffffffffffe
1415f922d 488955e0 mov qword ptr [rbp - 0x20], rdx
1415f9231 4c8945e8 mov qword ptr [rbp - 0x18], r8
1415f9235 488b01 mov rax, qword ptr [rcx]
1415f9238 488945f8 mov qword ptr [rbp - 8], rax
1415f923c 488b4108 mov rax, qword ptr [rcx + 8]
1415f9240 488945f0 mov qword ptr [rbp - 0x10], rax
1415f9244 488b4110 mov rax, qword ptr [rcx + 0x10]
1415f9248 4885c0 test rax, rax
1415f924b 747d je 0x1415f92ca
1415f924d 488d0480 lea rax, [rax + rax*4]
1415f9251 4c8b65f0 mov r12, qword ptr [rbp - 0x10]
1415f9255 498d1c84 lea rbx, [r12 + rax*4]
1415f9259 4c8d75d4 lea r14, [rbp - 0x2c]
1415f925d 488d75cc lea rsi, [rbp - 0x34]
1415f9261 49bf0100000001000000 movabs r15, 0x100000001
1415f926b 488d7de0 lea rdi, [rbp - 0x20]
1415f926f 90 nop 
1415f9270 418b0424 mov eax, dword ptr [r12]
1415f9274 458b6c2404 mov r13d, dword ptr [r12 + 4]
1415f9279 418b4c2410 mov ecx, dword ptr [r12 + 0x10]
1415f927e 894dc8 mov dword ptr [rbp - 0x38], ecx
1415f9281 498b542408 mov rdx, qword ptr [r12 + 8]
1415f9286 488955c0 mov qword ptr [rbp - 0x40], rdx
1415f928a 8945cc mov dword ptr [rbp - 0x34], eax
1415f928d 44896dd0 mov dword ptr [rbp - 0x30], r13d
1415f9291 41894e08 mov dword ptr [r14 + 8], ecx
1415f9295 498916 mov qword ptr [r14], rdx
1415f9298 488b4de0 mov rcx, qword ptr [rbp - 0x20]
1415f929c 488b55e8 mov rdx, qword ptr [rbp - 0x18]
1415f92a0 41b914000000 mov r9d, 0x14
1415f92a6 4989f0 mov r8, rsi
1415f92a9 e8a2e70f00 call 0x1416f7a50 ; rhapsody::rg::pass_api::RenderPassApi::push_constants
1415f92ae 44896dcc mov dword ptr [rbp - 0x34], r13d
1415f92b2 4c897dd0 mov qword ptr [rbp - 0x30], r15
1415f92b6 4889f9 mov rcx, rdi
1415f92b9 4889f2 mov rdx, rsi
1415f92bc e8df5c37ff call 0x14096efa0 ; country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi::dispatch_3d
1415f92c1 4983c414 add r12, 0x14
1415f92c5 4939dc cmp r12, rbx
1415f92c8 75a6 jne 0x1415f9270
1415f92ca 488b45f8 mov rax, qword ptr [rbp - 8]
1415f92ce 4885c0 test rax, rax
1415f92d1 7417 je 0x1415f92ea
1415f92d3 48c1e002 shl rax, 2
1415f92d7 488d1480 lea rdx, [rax + rax*4]
1415f92db 41b804000000 mov r8d, 4
1415f92e1 488b4df0 mov rcx, qword ptr [rbp - 0x10]
1415f92e5 e836a901ff call 0x140613c20 ; 
1415f92ea 90 nop 
1415f92eb 4883c468 add rsp, 0x68
1415f92ef 5b pop rbx
1415f92f0 5f pop rdi
1415f92f1 5e pop rsi
1415f92f2 415c pop r12
1415f92f4 415d pop r13
1415f92f6 415e pop r14
1415f92f8 415f pop r15
1415f92fa 5d pop rbp
1415f92fb c3 ret 
1415f92fc 0f1f4000 nop dword ptr [rax]
1415f9300 4889542410 mov qword ptr [rsp + 0x10], rdx
1415f9305 55 push rbp
1415f9306 4157 push r15
1415f9308 4156 push r14
1415f930a 4155 push r13
1415f930c 4154 push r12
1415f930e 56 push rsi
1415f930f 57 push rdi
1415f9310 53 push rbx
1415f9311 4883ec28 sub rsp, 0x28
1415f9315 488d6a60 lea rbp, [rdx + 0x60]
1415f9319 48837df800 cmp qword ptr [rbp - 8], 0
1415f931e 741b je 0x1415f933b
1415f9320 488b45f8 mov rax, qword ptr [rbp - 8]
1415f9324 48c1e002 shl rax, 2
1415f9328 488d1480 lea rdx, [rax + rax*4]
1415f932c 41b804000000 mov r8d, 4
1415f9332 488b4df0 mov rcx, qword ptr [rbp - 0x10]
1415f9336 e8e5a801ff call 0x140613c20 ; 
1415f933b 90 nop 
1415f933c 4883c428 add rsp, 0x28
1415f9340 5b pop rbx
1415f9341 5f pop rdi
1415f9342 5e pop rsi
1415f9343 415c pop r12
1415f9345 415d pop r13
1415f9347 415e pop r14
1415f9349 415f pop r15
1415f934b 5d pop rbp
1415f934c c3 ret 
