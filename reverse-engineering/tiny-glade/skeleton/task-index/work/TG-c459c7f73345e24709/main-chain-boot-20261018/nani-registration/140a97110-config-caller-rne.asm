140a97110: push rbp
140a97111: push rsi
140a97112: push rdi
140a97113: push rbx
140a97114: sub rsp, 0x2e8
140a9711b: lea rbp, [rsp + 0x80]
140a97123: mov qword ptr [rbp + 0x260], 0xfffffffffffffffe
140a9712e: mov rsi, rcx
140a97131: movzx eax, byte ptr [rip + 0x2b9dad1]
140a97138: mov ecx, 0x1c0
140a9713d: mov edx, 8
140a97142: call 0x140613c10
140a97147: test rax, rax
140a9714a: je 0x140a974f7
140a97150: mov qword ptr [rbp + 0x258], rax
140a97157: xorps xmm0, xmm0
140a9715a: movups xmmword ptr [rbp + 0x138], xmm0
140a97161: movups xmmword ptr [rbp + 0x150], xmm0
140a97168: movups xmmword ptr [rbp + 0x168], xmm0
140a9716f: movups xmmword ptr [rbp + 0x180], xmm0
140a97176: movups xmmword ptr [rbp + 0x198], xmm0
140a9717d: mov dword ptr [rbp + 0x1a8], 0
140a97187: movups xmmword ptr [rbp + 0x1d0], xmm0
140a9718e: movups xmmword ptr [rbp + 0x1e8], xmm0
140a97195: movups xmmword ptr [rbp + 0x200], xmm0
140a9719c: movups xmmword ptr [rbp + 0x218], xmm0
140a971a3: movups xmmword ptr [rbp + 0x230], xmm0
140a971aa: mov dword ptr [rbp + 0x240], 0
140a971b4: mov qword ptr [rbp + 0xf8], 0
140a971bf: mov qword ptr [rbp + 0x118], 0
140a971ca: mov qword ptr [rbp + 0x120], 8
140a971d5: mov qword ptr [rbp + 0x128], 0
140a971e0: mov qword ptr [rbp + 0x130], 0x10
140a971eb: mov qword ptr [rbp + 0x148], 0x10
140a971f6: mov qword ptr [rbp + 0x160], 0x10
140a97201: mov qword ptr [rbp + 0x178], 0x10
140a9720c: mov qword ptr [rbp + 0x190], 0x10
140a97217: movabs rbx, 0x8000000000000000
140a97221: mov qword ptr [rbp + 0x1b0], rbx
140a97228: lea rax, [rip + 0x2043d09]
140a9722f: mov qword ptr [rbp + 0x1b8], rax
140a97236: mov qword ptr [rbp + 0x1c0], 0x5b
140a97241: mov qword ptr [rbp + 0x1c8], 0x10
140a9724c: mov qword ptr [rbp + 0x1e0], 0x10
140a97257: mov qword ptr [rbp + 0x1f8], 0x10
140a97262: mov qword ptr [rbp + 0x210], 0x10
140a9726d: mov qword ptr [rbp + 0x228], 0x10
140a97278: mov dword ptr [rbp + 0x248], 0
140a97282: mov word ptr [rbp + 0x24c], 1
140a9728b: mov dword ptr [rbp + 0x250], 0
140a97295: movzx eax, byte ptr [rip + 0x2b9d96d]
140a9729c: mov ecx, 0x160
140a972a1: mov edx, 8
140a972a6: call 0x140613c10
140a972ab: test rax, rax
140a972ae: je 0x140a97506
140a972b4: mov rdi, rax
140a972b7: lea rdx, [rbp + 0xf8]
140a972be: mov r8d, 0x160
140a972c4: mov rcx, rax
140a972c7: call 0x142923c00
140a972cc: lea r8, [rip + 0x204367d]
140a972d3: lea rcx, [rbp + 0x88]
140a972da: mov rdx, rdi
140a972dd: call 0x140ce4b10
140a972e2: lea rcx, [rbp + 0x18]
140a972e6: call 0x140a2bf40
140a972eb: lea rcx, [rbp - 0x58]
140a972ef: call 0x1415f4750
140a972f4: lea rcx, [rbp + 0xf8]
140a972fb: call 0x1409c8600
140a97300: movups xmm0, xmmword ptr [rbp + 0xe8]
140a97307: mov rax, qword ptr [rbp + 0x258]
140a9730e: movups xmmword ptr [rax + 0x60], xmm0
140a97312: movups xmm0, xmmword ptr [rbp + 0xd8]
140a97319: movups xmmword ptr [rax + 0x50], xmm0
140a9731d: movups xmm0, xmmword ptr [rbp + 0xc8]
140a97324: movups xmmword ptr [rax + 0x40], xmm0
140a97328: movups xmm0, xmmword ptr [rbp + 0x88]
140a9732f: movups xmm1, xmmword ptr [rbp + 0x98]
140a97336: movups xmm2, xmmword ptr [rbp + 0xa8]
140a9733d: movups xmm3, xmmword ptr [rbp + 0xb8]
140a97344: movups xmmword ptr [rax + 0x30], xmm3
140a97348: movups xmmword ptr [rax + 0x20], xmm2
140a9734c: movups xmmword ptr [rax + 0x10], xmm1
140a97350: movups xmmword ptr [rax], xmm0
140a97353: movups xmm0, xmmword ptr [rbp + 0x18]
140a97357: movups xmm1, xmmword ptr [rbp + 0x28]
140a9735b: movups xmm2, xmmword ptr [rbp + 0x38]
140a9735f: movups xmm3, xmmword ptr [rbp + 0x48]
140a97363: movups xmmword ptr [rax + 0x70], xmm0
140a97367: movups xmmword ptr [rax + 0x80], xmm1
140a9736e: movups xmmword ptr [rax + 0x90], xmm2
140a97375: movups xmmword ptr [rax + 0xa0], xmm3
140a9737c: movups xmm0, xmmword ptr [rbp + 0x58]
140a97380: movups xmmword ptr [rax + 0xb0], xmm0
140a97387: movups xmm0, xmmword ptr [rbp + 0x68]
140a9738b: movups xmmword ptr [rax + 0xc0], xmm0
140a97392: movups xmm0, xmmword ptr [rbp + 0x78]
140a97396: movups xmmword ptr [rax + 0xd0], xmm0
140a9739d: movups xmm0, xmmword ptr [rbp + 8]
140a973a1: movups xmmword ptr [rax + 0x140], xmm0
140a973a8: movups xmm0, xmmword ptr [rbp - 8]
140a973ac: movups xmmword ptr [rax + 0x130], xmm0
140a973b3: movups xmm0, xmmword ptr [rbp - 0x18]
140a973b7: movups xmmword ptr [rax + 0x120], xmm0
140a973be: movups xmm0, xmmword ptr [rbp - 0x58]
140a973c2: movups xmm1, xmmword ptr [rbp - 0x48]
140a973c6: movups xmm2, xmmword ptr [rbp - 0x38]
140a973ca: movups xmm3, xmmword ptr [rbp - 0x28]
140a973ce: movups xmmword ptr [rax + 0x110], xmm3
140a973d5: movups xmmword ptr [rax + 0x100], xmm2
140a973dc: movups xmmword ptr [rax + 0xf0], xmm1
140a973e3: movups xmmword ptr [rax + 0xe0], xmm0
140a973ea: movups xmm0, xmmword ptr [rbp + 0xf8]
140a973f1: movups xmm1, xmmword ptr [rbp + 0x108]
140a973f8: movups xmm2, xmmword ptr [rbp + 0x118]
140a973ff: movups xmm3, xmmword ptr [rbp + 0x128]
140a97406: movups xmmword ptr [rax + 0x150], xmm0
140a9740d: movups xmmword ptr [rax + 0x160], xmm1
140a97414: movups xmmword ptr [rax + 0x170], xmm2
140a9741b: movups xmmword ptr [rax + 0x180], xmm3
140a97422: movups xmm0, xmmword ptr [rbp + 0x138]
140a97429: movups xmmword ptr [rax + 0x190], xmm0
140a97430: movups xmm0, xmmword ptr [rbp + 0x148]
140a97437: movups xmmword ptr [rax + 0x1a0], xmm0
140a9743e: movups xmm0, xmmword ptr [rbp + 0x158]
140a97445: movups xmmword ptr [rax + 0x1b0], xmm0
140a9744c: mov qword ptr [rbp + 0x100], 4
140a97457: mov qword ptr [rbp + 0x108], rax
140a9745e: mov qword ptr [rbp + 0x110], 4
140a97469: mov qword ptr [rbp + 0x118], 0
140a97474: mov qword ptr [rbp + 0x120], 8
140a9747f: xorps xmm0, xmm0
140a97482: movups xmmword ptr [rbp + 0x128], xmm0
140a97489: mov qword ptr [rbp + 0xf8], rbx
140a97490: lea rcx, [rbp + 0x130]
140a97497: call 0x140d2fa20
140a9749c: movups xmm0, xmmword ptr [rbp + 0x158]
140a974a3: movups xmmword ptr [rsi + 0x60], xmm0
140a974a7: movups xmm0, xmmword ptr [rbp + 0x148]
140a974ae: movups xmmword ptr [rsi + 0x50], xmm0
140a974b2: movups xmm0, xmmword ptr [rbp + 0x138]
140a974b9: movups xmmword ptr [rsi + 0x40], xmm0
140a974bd: movups xmm0, xmmword ptr [rbp + 0xf8]
140a974c4: movups xmm1, xmmword ptr [rbp + 0x108]
140a974cb: movups xmm2, xmmword ptr [rbp + 0x118]
140a974d2: movups xmm3, xmmword ptr [rbp + 0x128]
140a974d9: movups xmmword ptr [rsi + 0x30], xmm3
140a974dd: movups xmmword ptr [rsi + 0x20], xmm2
140a974e1: movups xmmword ptr [rsi + 0x10], xmm1
140a974e5: movups xmmword ptr [rsi], xmm0
140a974e8: mov rax, rsi
140a974eb: add rsp, 0x2e8
140a974f2: pop rbx
140a974f3: pop rdi
140a974f4: pop rsi
140a974f5: pop rbp
140a974f6: ret 
140a974f7: mov ecx, 8
140a974fc: mov edx, 0x1c0
140a97501: call 0x1428d9000
140a97506: mov ecx, 8
140a9750b: mov edx, 0x160
140a97510: call 0x1428d9000
140a97515: ud2 
140a97517: nop word ptr [rax + rax]
140a97520: mov qword ptr [rsp + 0x10], rdx
140a97525: push rbp
140a97526: push rsi
140a97527: push rdi
140a97528: push rbx
140a97529: sub rsp, 0x28
140a9752d: lea rbp, [rdx + 0x80]
140a97534: lea rcx, [rbp + 0xf8]
140a9753b: call 0x1405e5e70
140a97540: nop 
140a97541: add rsp, 0x28
140a97545: pop rbx
140a97546: pop rdi
140a97547: pop rsi
140a97548: pop rbp
140a97549: ret 
140a9754a: nop word ptr [rax + rax]
140a97550: mov qword ptr [rsp + 0x10], rdx
140a97555: push rbp
140a97556: push rsi
140a97557: push rdi
140a97558: push rbx
140a97559: sub rsp, 0x28
140a9755d: lea rbp, [rdx + 0x80]
140a97564: mov edx, 0x1c0
140a97569: mov r8d, 8
140a9756f: mov rcx, qword ptr [rbp + 0x258]
140a97576: call 0x140613c20
140a9757b: nop 
140a9757c: add rsp, 0x28
140a97580: pop rbx
140a97581: pop rdi
140a97582: pop rsi
140a97583: pop rbp
140a97584: ret 
140a97585: nop word ptr cs:[rax + rax]
140a97590: mov qword ptr [rsp + 0x10], rdx
140a97595: push rbp
140a97596: push rsi
140a97597: push rdi
140a97598: push rbx
140a97599: sub rsp, 0x28
140a9759d: lea rbp, [rdx + 0x80]
140a975a4: lea rcx, [rbp + 0x88]
140a975ab: call 0x140029880
140a975b0: nop 
140a975b1: add rsp, 0x28
140a975b5: pop rbx
140a975b6: pop rdi
140a975b7: pop rsi
140a975b8: pop rbp
140a975b9: ret 
140a975ba: nop word ptr [rax + rax]
140a975c0: mov qword ptr [rsp + 0x10], rdx
140a975c5: push rbp
140a975c6: push rsi
140a975c7: push rdi
140a975c8: push rbx
140a975c9: sub rsp, 0x28
140a975cd: lea rbp, [rdx + 0x80]
140a975d4: lea rcx, [rbp + 0x18]
140a975d8: call 0x140029880
140a975dd: nop 
140a975de: add rsp, 0x28
140a975e2: pop rbx
140a975e3: pop rdi
140a975e4: pop rsi
140a975e5: pop rbp
140a975e6: ret 
140a975e7: nop word ptr [rax + rax]
140a975f0: mov qword ptr [rsp + 0x10], rdx
140a975f5: push rbp
140a975f6: push rsi
140a975f7: push rdi
140a975f8: push rbx
140a975f9: sub rsp, 0x28
140a975fd: lea rbp, [rdx + 0x80]
140a97604: lea rcx, [rbp - 0x58]
140a97608: call 0x140029880
140a9760d: nop 
140a9760e: add rsp, 0x28
140a97612: pop rbx
140a97613: pop rdi
140a97614: pop rsi
140a97615: pop rbp
140a97616: ret 
140a97617: nop word ptr [rax + rax]
140a97620: mov qword ptr [rsp + 0x10], rdx
140a97625: push rbp
140a97626: push rsi
140a97627: push rdi
140a97628: push rbx
140a97629: sub rsp, 0x28
140a9762d: lea rbp, [rdx + 0x80]
140a97634: lea rcx, [rbp + 0xf8]
140a9763b: call 0x140029880
140a97640: nop 
140a97641: add rsp, 0x28
140a97645: pop rbx
140a97646: pop rdi
140a97647: pop rsi
140a97648: pop rbp
140a97649: ret 
