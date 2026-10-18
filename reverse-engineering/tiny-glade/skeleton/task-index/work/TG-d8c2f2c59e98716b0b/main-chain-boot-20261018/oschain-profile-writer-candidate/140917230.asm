140917230: push rbp
140917231: sub rsp, 0xd0
140917238: lea rbp, [rsp + 0x80]
140917240: mov qword ptr [rbp + 0x48], 0xfffffffffffffffe
140917248: mov rcx, qword ptr [rcx]
14091724b: call 0x140917360
140917250: test rax, rax
140917253: je 0x14091731b
140917259: mov qword ptr [rbp + 0x38], rax
14091725d: mov rax, qword ptr [rip + 0x2d15144]
140917264: mov rax, qword ptr [rax]
140917267: test rax, rax
14091726a: je 0x140917312
140917270: lea rax, [rbp + 0x38]
140917274: mov qword ptr [rbp + 0x28], rax
140917278: lea rax, [rip + 0x4359a1]
14091727f: mov qword ptr [rbp + 0x30], rax
140917283: lea rcx, [rip + 0x2195966]
14091728a: call 0x14067dd80
14091728f: movups xmm0, xmmword ptr [rax]
140917292: mov eax, dword ptr [rax + 0x10]
140917295: mov qword ptr [rbp - 0x28], 1
14091729d: lea rcx, [rip + 0x2195804]
1409172a4: mov qword ptr [rbp - 0x20], rcx
1409172a8: mov qword ptr [rbp - 0x18], 0x1a
1409172b0: lea rdx, [rip + 0x2195911]
1409172b7: mov qword ptr [rbp - 8], rdx
1409172bb: mov qword ptr [rbp], 1
1409172c3: lea rdx, [rbp + 0x28]
1409172c7: mov qword ptr [rbp + 8], rdx
1409172cb: mov qword ptr [rbp + 0x10], 1
1409172d3: mov qword ptr [rbp + 0x18], 0
1409172db: mov qword ptr [rbp - 0x58], 0
1409172e3: mov qword ptr [rbp - 0x50], rcx
1409172e7: mov qword ptr [rbp - 0x48], 0x1a
1409172ef: mov qword ptr [rbp - 0x40], 0
1409172f7: movups xmmword ptr [rbp - 0x38], xmm0
1409172fb: mov dword ptr [rbp - 0x10], 1
140917302: mov dword ptr [rbp - 0xc], eax
140917305: lea rcx, [rbp + 0x47]
140917309: lea rdx, [rbp - 0x58]
14091730d: call 0x140d4a330
140917312: lea rcx, [rbp + 0x38]
140917316: call 0x140d4cc30
14091731b: nop 
14091731c: add rsp, 0xd0
140917323: pop rbp
140917324: ret 
140917325: nop word ptr cs:[rax + rax]
140917330: mov qword ptr [rsp + 0x10], rdx
140917335: push rbp
140917336: sub rsp, 0x20
14091733a: lea rbp, [rdx + 0x80]
140917341: lea rcx, [rbp + 0x38]
140917345: call 0x140d4cc30
14091734a: nop 
14091734b: add rsp, 0x20
14091734f: pop rbp
140917350: ret 
