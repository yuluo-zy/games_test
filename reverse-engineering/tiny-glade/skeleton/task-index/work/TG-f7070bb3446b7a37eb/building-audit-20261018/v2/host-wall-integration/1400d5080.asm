0x1400d5080 55 push rbp
0x1400d5081 56 push rsi
0x1400d5082 57 push rdi
0x1400d5083 4881ec80010000 sub rsp, 0x180
0x1400d508a 488dac2480000000 lea rbp, [rsp + 0x80]
0x1400d5092 48c785f8000000feffffff mov qword ptr [rbp + 0xf8], 0xfffffffffffffffe
0x1400d509d 4889d7 mov rdi, rdx
0x1400d50a0 4889ce mov rsi, rcx
0x1400d50a3 0f104260 movups xmm0, xmmword ptr [rdx + 0x60]
0x1400d50a7 0f294570 movaps xmmword ptr [rbp + 0x70], xmm0
0x1400d50ab 0f104250 movups xmm0, xmmword ptr [rdx + 0x50]
0x1400d50af 0f294560 movaps xmmword ptr [rbp + 0x60], xmm0
0x1400d50b3 0f104240 movups xmm0, xmmword ptr [rdx + 0x40]
0x1400d50b7 0f294550 movaps xmmword ptr [rbp + 0x50], xmm0
0x1400d50bb 0f1002 movups xmm0, xmmword ptr [rdx]
0x1400d50be 0f104a10 movups xmm1, xmmword ptr [rdx + 0x10]
0x1400d50c2 0f105220 movups xmm2, xmmword ptr [rdx + 0x20]
0x1400d50c6 0f105a30 movups xmm3, xmmword ptr [rdx + 0x30]
0x1400d50ca 0f295d40 movaps xmmword ptr [rbp + 0x40], xmm3
0x1400d50ce 0f295530 movaps xmmword ptr [rbp + 0x30], xmm2
0x1400d50d2 0f294d20 movaps xmmword ptr [rbp + 0x20], xmm1
0x1400d50d6 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
0x1400d50da 0f1082d0000000 movups xmm0, xmmword ptr [rdx + 0xd0]
0x1400d50e1 0f294500 movaps xmmword ptr [rbp], xmm0
0x1400d50e5 0f1082c0000000 movups xmm0, xmmword ptr [rdx + 0xc0]
0x1400d50ec 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
0x1400d50f0 0f1082b0000000 movups xmm0, xmmword ptr [rdx + 0xb0]
0x1400d50f7 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
0x1400d50fb 0f104270 movups xmm0, xmmword ptr [rdx + 0x70]
0x1400d50ff 0f108a80000000 movups xmm1, xmmword ptr [rdx + 0x80]
0x1400d5106 0f109290000000 movups xmm2, xmmword ptr [rdx + 0x90]
0x1400d510d 0f109aa0000000 movups xmm3, xmmword ptr [rdx + 0xa0]
0x1400d5114 0f295dd0 movaps xmmword ptr [rbp - 0x30], xmm3
0x1400d5118 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
0x1400d511c 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
0x1400d5120 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x1400d5124 0fb605defa5503 movzx eax, byte ptr [rip + 0x355fade]
0x1400d512b b9e0000000 mov ecx, 0xe0
0x1400d5130 ba08000000 mov edx, 8
0x1400d5135 e8d6ea5300 call 0x140613c10
0x1400d513a 4885c0 test rax, rax
0x1400d513d 0f8437010000 je 0x1400d527a
0x1400d5143 488d4f70 lea rcx, [rdi + 0x70]
0x1400d5147 0f104760 movups xmm0, xmmword ptr [rdi + 0x60]
0x1400d514b 0f114060 movups xmmword ptr [rax + 0x60], xmm0
0x1400d514f 0f104750 movups xmm0, xmmword ptr [rdi + 0x50]
0x1400d5153 0f114050 movups xmmword ptr [rax + 0x50], xmm0
0x1400d5157 0f104740 movups xmm0, xmmword ptr [rdi + 0x40]
0x1400d515b 0f114040 movups xmmword ptr [rax + 0x40], xmm0
0x1400d515f 0f1007 movups xmm0, xmmword ptr [rdi]
0x1400d5162 0f104f10 movups xmm1, xmmword ptr [rdi + 0x10]
0x1400d5166 0f105720 movups xmm2, xmmword ptr [rdi + 0x20]
0x1400d516a 0f105f30 movups xmm3, xmmword ptr [rdi + 0x30]
0x1400d516e 0f115830 movups xmmword ptr [rax + 0x30], xmm3
0x1400d5172 0f115020 movups xmmword ptr [rax + 0x20], xmm2
0x1400d5176 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x1400d517a 0f1100 movups xmmword ptr [rax], xmm0
0x1400d517d 0f1001 movups xmm0, xmmword ptr [rcx]
0x1400d5180 0f104910 movups xmm1, xmmword ptr [rcx + 0x10]
0x1400d5184 0f105120 movups xmm2, xmmword ptr [rcx + 0x20]
0x1400d5188 0f105930 movups xmm3, xmmword ptr [rcx + 0x30]
0x1400d518c 0f114070 movups xmmword ptr [rax + 0x70], xmm0
0x1400d5190 0f118880000000 movups xmmword ptr [rax + 0x80], xmm1
0x1400d5197 0f119090000000 movups xmmword ptr [rax + 0x90], xmm2
0x1400d519e 0f1198a0000000 movups xmmword ptr [rax + 0xa0], xmm3
0x1400d51a5 0f104140 movups xmm0, xmmword ptr [rcx + 0x40]
0x1400d51a9 0f1180b0000000 movups xmmword ptr [rax + 0xb0], xmm0
0x1400d51b0 0f104150 movups xmm0, xmmword ptr [rcx + 0x50]
0x1400d51b4 0f1180c0000000 movups xmmword ptr [rax + 0xc0], xmm0
0x1400d51bb 0f104160 movups xmm0, xmmword ptr [rcx + 0x60]
0x1400d51bf 0f1180d0000000 movups xmmword ptr [rax + 0xd0], xmm0
0x1400d51c6 48c7859000000002000000 mov qword ptr [rbp + 0x90], 2
0x1400d51d1 48898598000000 mov qword ptr [rbp + 0x98], rax
0x1400d51d8 48c785a000000002000000 mov qword ptr [rbp + 0xa0], 2
0x1400d51e3 48c785a800000000000000 mov qword ptr [rbp + 0xa8], 0
0x1400d51ee 48c785b000000008000000 mov qword ptr [rbp + 0xb0], 8
0x1400d51f9 488d8dc0000000 lea rcx, [rbp + 0xc0]
0x1400d5200 0f57c0 xorps xmm0, xmm0
0x1400d5203 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
0x1400d520a 48b80000000000000080 movabs rax, 0x8000000000000000
0x1400d5214 48898588000000 mov qword ptr [rbp + 0x88], rax
0x1400d521b e8b0b30900 call 0x1401705d0
0x1400d5220 0f1085e8000000 movups xmm0, xmmword ptr [rbp + 0xe8]
0x1400d5227 0f114660 movups xmmword ptr [rsi + 0x60], xmm0
0x1400d522b 0f1085d8000000 movups xmm0, xmmword ptr [rbp + 0xd8]
0x1400d5232 0f114650 movups xmmword ptr [rsi + 0x50], xmm0
0x1400d5236 0f1085c8000000 movups xmm0, xmmword ptr [rbp + 0xc8]
0x1400d523d 0f114640 movups xmmword ptr [rsi + 0x40], xmm0
0x1400d5241 0f108588000000 movups xmm0, xmmword ptr [rbp + 0x88]
0x1400d5248 0f108d98000000 movups xmm1, xmmword ptr [rbp + 0x98]
0x1400d524f 0f1095a8000000 movups xmm2, xmmword ptr [rbp + 0xa8]
0x1400d5256 0f109db8000000 movups xmm3, xmmword ptr [rbp + 0xb8]
0x1400d525d 0f115e30 movups xmmword ptr [rsi + 0x30], xmm3
0x1400d5261 0f115620 movups xmmword ptr [rsi + 0x20], xmm2
0x1400d5265 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
0x1400d5269 0f1106 movups xmmword ptr [rsi], xmm0
0x1400d526c 4889f0 mov rax, rsi
0x1400d526f 4881c480010000 add rsp, 0x180
0x1400d5276 5f pop rdi
0x1400d5277 5e pop rsi
0x1400d5278 5d pop rbp
0x1400d5279 c3 ret 
0x1400d527a b908000000 mov ecx, 8
0x1400d527f bae0000000 mov edx, 0xe0
0x1400d5284 e8773d8002 call 0x1428d9000
0x1400d5289 0f0b ud2 
0x1400d528b 0f1f440000 nop dword ptr [rax + rax]
0x1400d5290 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1400d5295 55 push rbp
0x1400d5296 56 push rsi
0x1400d5297 57 push rdi
0x1400d5298 4883ec20 sub rsp, 0x20
0x1400d529c 488daa80000000 lea rbp, [rdx + 0x80]
0x1400d52a3 488d4da0 lea rcx, [rbp - 0x60]
0x1400d52a7 e8d445f5ff call 0x140029880
0x1400d52ac 488d4d10 lea rcx, [rbp + 0x10]
0x1400d52b0 e8cb45f5ff call 0x140029880
0x1400d52b5 90 nop 
0x1400d52b6 4883c420 add rsp, 0x20
0x1400d52ba 5f pop rdi
0x1400d52bb 5e pop rsi
0x1400d52bc 5d pop rbp
0x1400d52bd c3 ret 
0x1400d52be 6690 nop 
0x1400d52c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1400d52c5 55 push rbp
0x1400d52c6 56 push rsi
0x1400d52c7 57 push rdi
0x1400d52c8 4883ec20 sub rsp, 0x20
0x1400d52cc 488daa80000000 lea rbp, [rdx + 0x80]
0x1400d52d3 488d8d88000000 lea rcx, [rbp + 0x88]
0x1400d52da e8a145f5ff call 0x140029880
0x1400d52df 90 nop 
0x1400d52e0 4883c420 add rsp, 0x20
0x1400d52e4 5f pop rdi
0x1400d52e5 5e pop rsi
0x1400d52e6 5d pop rbp
0x1400d52e7 c3 ret 
