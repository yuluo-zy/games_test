
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14200fa70: 41 57                       	push	r15
14200fa72: 41 56                       	push	r14
14200fa74: 41 55                       	push	r13
14200fa76: 41 54                       	push	r12
14200fa78: 56                          	push	rsi
14200fa79: 57                          	push	rdi
14200fa7a: 55                          	push	rbp
14200fa7b: 53                          	push	rbx
14200fa7c: 48 83 ec 78                 	sub	rsp, 0x78
14200fa80: 4c 89 ce                    	mov	rsi, r9
14200fa83: 4c 89 c7                    	mov	rdi, r8
14200fa86: 49 89 cc                    	mov	r12, rcx
14200fa89: 48 8b 94 24 18 01 00 00     	mov	rdx, qword ptr [rsp + 0x118]
14200fa91: 48 8b 8c 24 e0 00 00 00     	mov	rcx, qword ptr [rsp + 0xe0]
14200fa99: 48 8b 42 30                 	mov	rax, qword ptr [rdx + 0x30]
14200fa9d: 4c 8b 72 38                 	mov	r14, qword ptr [rdx + 0x38]
14200faa1: 48 89 44 24 40              	mov	qword ptr [rsp + 0x40], rax
14200faa6: 49 8b 04 24                 	mov	rax, qword ptr [r12]
14200faaa: 0f b6 09                    	movzx	ecx, byte ptr [rcx]
14200faad: 4d 8b 68 40                 	mov	r13, qword ptr [r8 + 0x40]
14200fab1: 49 39 c5                    	cmp	r13, rax
14200fab4: 76 5f                       	jbe	0x14200fb15 <.text+0x200eb15>
14200fab6: 48 8b 57 38                 	mov	rdx, qword ptr [rdi + 0x38]
14200faba: 48 8b 14 c2                 	mov	rdx, qword ptr [rdx + 8*rax]
14200fabe: 48 85 d2                    	test	rdx, rdx
14200fac1: 74 52                       	je	0x14200fb15 <.text+0x200eb15>
14200fac3: 48 f7 d2                    	not	rdx
14200fac6: 4c 8d 3c 52                 	lea	r15, [rdx + 2*rdx]
14200faca: 49 c1 e7 04                 	shl	r15, 0x4
14200face: 4c 03 7f 18                 	add	r15, qword ptr [rdi + 0x18]
14200fad2: 8b 9c 24 10 01 00 00        	mov	ebx, dword ptr [rsp + 0x110]
14200fad9: 8b ac 24 08 01 00 00        	mov	ebp, dword ptr [rsp + 0x108]
14200fae0: 84 c9                       	test	cl, cl
14200fae2: 75 46                       	jne	0x14200fb2a <.text+0x200eb2a>
14200fae4: 4c 89 6c 24 38              	mov	qword ptr [rsp + 0x38], r13
14200fae9: 4d 89 f5                    	mov	r13, r14
14200faec: 4d 89 e6                    	mov	r14, r12
14200faef: 41 89 ec                    	mov	r12d, ebp
14200faf2: 4d 8b 47 08                 	mov	r8, qword ptr [r15 + 0x8]
14200faf6: 4c 89 c1                    	mov	rcx, r8
14200faf9: 49 0f af cc                 	imul	rcx, r12
14200fafd: 49 03 4f 10                 	add	rcx, qword ptr [r15 + 0x10]
14200fb01: 48 8d 54 24 40              	lea	rdx, [rsp + 0x40]
14200fb06: e8 05 41 91 00              	call	0x142923c10 <.text+0x2922c10>
14200fb0b: 49 8b 47 20                 	mov	rax, qword ptr [r15 + 0x20]
14200fb0f: 42 89 1c a0                 	mov	dword ptr [rax + 4*r12], ebx
14200fb13: eb 71                       	jmp	0x14200fb86 <.text+0x200eb86>
14200fb15: 45 31 ff                    	xor	r15d, r15d
14200fb18: 8b 9c 24 10 01 00 00        	mov	ebx, dword ptr [rsp + 0x110]
14200fb1f: 8b ac 24 08 01 00 00        	mov	ebp, dword ptr [rsp + 0x108]
14200fb26: 84 c9                       	test	cl, cl
14200fb28: 74 ba                       	je	0x14200fae4 <.text+0x200eae4>
14200fb2a: 80 bc 24 20 01 00 00 00     	cmp	byte ptr [rsp + 0x120], 0x0
14200fb32: 74 34                       	je	0x14200fb68 <.text+0x200eb68>
14200fb34: 49 39 c5                    	cmp	r13, rax
14200fb37: 76 60                       	jbe	0x14200fb99 <.text+0x200eb99>
14200fb39: 48 8b 4f 38                 	mov	rcx, qword ptr [rdi + 0x38]
14200fb3d: 48 8b 04 c1                 	mov	rax, qword ptr [rcx + 8*rax]
14200fb41: 48 85 c0                    	test	rax, rax
14200fb44: 74 53                       	je	0x14200fb99 <.text+0x200eb99>
14200fb46: 48 8b 4f 18                 	mov	rcx, qword ptr [rdi + 0x18]
14200fb4a: 48 f7 d0                    	not	rax
14200fb4d: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14200fb51: 48 c1 e0 04                 	shl	rax, 0x4
14200fb55: 48 8b 44 01 18              	mov	rax, qword ptr [rcx + rax + 0x18]
14200fb5a: 48 85 c0                    	test	rax, rax
14200fb5d: 74 3a                       	je	0x14200fb99 <.text+0x200eb99>
14200fb5f: 48 8d 4c 24 40              	lea	rcx, [rsp + 0x40]
14200fb64: ff d0                       	call	rax
14200fb66: eb 31                       	jmp	0x14200fb99 <.text+0x200eb99>
14200fb68: 4c 89 6c 24 38              	mov	qword ptr [rsp + 0x38], r13
14200fb6d: 4d 89 f5                    	mov	r13, r14
14200fb70: 4d 89 e6                    	mov	r14, r12
14200fb73: 41 89 ec                    	mov	r12d, ebp
14200fb76: 4c 8d 44 24 40              	lea	r8, [rsp + 0x40]
14200fb7b: 4c 89 f9                    	mov	rcx, r15
14200fb7e: 4c 89 e2                    	mov	rdx, r12
14200fb81: e8 7a f8 ce fe              	call	0x140cff400 <.text+0xcfe400>
14200fb86: 49 8b 47 28                 	mov	rax, qword ptr [r15 + 0x28]
14200fb8a: 42 89 1c a0                 	mov	dword ptr [rax + 4*r12], ebx
14200fb8e: 4d 89 f4                    	mov	r12, r14
14200fb91: 4d 89 ee                    	mov	r14, r13
14200fb94: 4c 8b 6c 24 38              	mov	r13, qword ptr [rsp + 0x38]
14200fb99: 4c 89 74 24 40              	mov	qword ptr [rsp + 0x40], r14
14200fb9e: 49 8b 44 24 08              	mov	rax, qword ptr [r12 + 0x8]
14200fba3: 48 8b 8c 24 e0 00 00 00     	mov	rcx, qword ptr [rsp + 0xe0]
14200fbab: 0f b6 49 01                 	movzx	ecx, byte ptr [rcx + 0x1]
14200fbaf: 49 39 c5                    	cmp	r13, rax
14200fbb2: 76 4e                       	jbe	0x14200fc02 <.text+0x200ec02>
14200fbb4: 48 8b 57 38                 	mov	rdx, qword ptr [rdi + 0x38]
14200fbb8: 48 8b 14 c2                 	mov	rdx, qword ptr [rdx + 8*rax]
14200fbbc: 48 85 d2                    	test	rdx, rdx
14200fbbf: 74 41                       	je	0x14200fc02 <.text+0x200ec02>
14200fbc1: 48 f7 d2                    	not	rdx
14200fbc4: 4c 8d 3c 52                 	lea	r15, [rdx + 2*rdx]
14200fbc8: 49 c1 e7 04                 	shl	r15, 0x4
14200fbcc: 4c 03 7f 18                 	add	r15, qword ptr [rdi + 0x18]
14200fbd0: 84 c9                       	test	cl, cl
14200fbd2: 75 35                       	jne	0x14200fc09 <.text+0x200ec09>
14200fbd4: 41 89 ee                    	mov	r14d, ebp
14200fbd7: 4d 8b 47 08                 	mov	r8, qword ptr [r15 + 0x8]
14200fbdb: 4c 89 c1                    	mov	rcx, r8
14200fbde: 49 0f af ce                 	imul	rcx, r14
14200fbe2: 49 03 4f 10                 	add	rcx, qword ptr [r15 + 0x10]
14200fbe6: 48 8d 54 24 40              	lea	rdx, [rsp + 0x40]
14200fbeb: e8 20 40 91 00              	call	0x142923c10 <.text+0x2922c10>
14200fbf0: 49 8b 47 20                 	mov	rax, qword ptr [r15 + 0x20]
14200fbf4: 42 89 1c b0                 	mov	dword ptr [rax + 4*r14], ebx
14200fbf8: 49 8b 47 28                 	mov	rax, qword ptr [r15 + 0x28]
14200fbfc: 42 89 1c b0                 	mov	dword ptr [rax + 4*r14], ebx
14200fc00: eb 6e                       	jmp	0x14200fc70 <.text+0x200ec70>
14200fc02: 45 31 ff                    	xor	r15d, r15d
14200fc05: 84 c9                       	test	cl, cl
14200fc07: 74 cb                       	je	0x14200fbd4 <.text+0x200ebd4>
14200fc09: 80 bc 24 20 01 00 00 00     	cmp	byte ptr [rsp + 0x120], 0x0
14200fc11: 74 3c                       	je	0x14200fc4f <.text+0x200ec4f>
14200fc13: 49 39 c5                    	cmp	r13, rax
14200fc16: 4c 8b b4 24 18 01 00 00     	mov	r14, qword ptr [rsp + 0x118]
14200fc1e: 76 58                       	jbe	0x14200fc78 <.text+0x200ec78>
14200fc20: 48 8b 4f 38                 	mov	rcx, qword ptr [rdi + 0x38]
14200fc24: 48 8b 04 c1                 	mov	rax, qword ptr [rcx + 8*rax]
14200fc28: 48 85 c0                    	test	rax, rax
14200fc2b: 74 4b                       	je	0x14200fc78 <.text+0x200ec78>
14200fc2d: 48 8b 4f 18                 	mov	rcx, qword ptr [rdi + 0x18]
14200fc31: 48 f7 d0                    	not	rax
14200fc34: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14200fc38: 48 c1 e0 04                 	shl	rax, 0x4
14200fc3c: 48 8b 44 01 18              	mov	rax, qword ptr [rcx + rax + 0x18]
14200fc41: 48 85 c0                    	test	rax, rax
14200fc44: 74 32                       	je	0x14200fc78 <.text+0x200ec78>
14200fc46: 48 8d 4c 24 40              	lea	rcx, [rsp + 0x40]
14200fc4b: ff d0                       	call	rax
14200fc4d: eb 29                       	jmp	0x14200fc78 <.text+0x200ec78>
14200fc4f: 4d 89 e6                    	mov	r14, r12
14200fc52: 41 89 ec                    	mov	r12d, ebp
14200fc55: 4c 8d 44 24 40              	lea	r8, [rsp + 0x40]
14200fc5a: 4c 89 f9                    	mov	rcx, r15
14200fc5d: 4c 89 e2                    	mov	rdx, r12
14200fc60: e8 9b f7 ce fe              	call	0x140cff400 <.text+0xcfe400>
14200fc65: 49 8b 47 28                 	mov	rax, qword ptr [r15 + 0x28]
14200fc69: 42 89 1c a0                 	mov	dword ptr [rax + 4*r12], ebx
14200fc6d: 4d 89 f4                    	mov	r12, r14
14200fc70: 4c 8b b4 24 18 01 00 00     	mov	r14, qword ptr [rsp + 0x118]
14200fc78: 41 0f 28 06                 	movaps	xmm0, xmmword ptr [r14]
14200fc7c: 41 0f 28 4e 10              	movaps	xmm1, xmmword ptr [r14 + 0x10]
14200fc81: 41 0f 28 56 20              	movaps	xmm2, xmmword ptr [r14 + 0x20]
14200fc86: 0f 29 54 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm2
14200fc8b: 0f 29 4c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm1
14200fc90: 0f 29 44 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm0
14200fc95: 49 8b 44 24 10              	mov	rax, qword ptr [r12 + 0x10]
14200fc9a: 48 8b 8c 24 e0 00 00 00     	mov	rcx, qword ptr [rsp + 0xe0]
14200fca2: 0f b6 49 02                 	movzx	ecx, byte ptr [rcx + 0x2]
14200fca6: 49 39 c5                    	cmp	r13, rax
14200fca9: 76 4e                       	jbe	0x14200fcf9 <.text+0x200ecf9>
14200fcab: 48 8b 57 38                 	mov	rdx, qword ptr [rdi + 0x38]
14200fcaf: 48 8b 14 c2                 	mov	rdx, qword ptr [rdx + 8*rax]
14200fcb3: 48 85 d2                    	test	rdx, rdx
14200fcb6: 74 41                       	je	0x14200fcf9 <.text+0x200ecf9>
14200fcb8: 48 f7 d2                    	not	rdx
14200fcbb: 4c 8d 3c 52                 	lea	r15, [rdx + 2*rdx]
14200fcbf: 49 c1 e7 04                 	shl	r15, 0x4
14200fcc3: 4c 03 7f 18                 	add	r15, qword ptr [rdi + 0x18]
14200fcc7: 84 c9                       	test	cl, cl
14200fcc9: 75 35                       	jne	0x14200fd00 <.text+0x200ed00>
14200fccb: 41 89 ee                    	mov	r14d, ebp
14200fcce: 4d 8b 47 08                 	mov	r8, qword ptr [r15 + 0x8]
14200fcd2: 4c 89 c1                    	mov	rcx, r8
14200fcd5: 49 0f af ce                 	imul	rcx, r14
14200fcd9: 49 03 4f 10                 	add	rcx, qword ptr [r15 + 0x10]
14200fcdd: 48 8d 54 24 40              	lea	rdx, [rsp + 0x40]
14200fce2: e8 29 3f 91 00              	call	0x142923c10 <.text+0x2922c10>
14200fce7: 49 8b 47 20                 	mov	rax, qword ptr [r15 + 0x20]
14200fceb: 42 89 1c b0                 	mov	dword ptr [rax + 4*r14], ebx
14200fcef: 49 8b 47 28                 	mov	rax, qword ptr [r15 + 0x28]
14200fcf3: 42 89 1c b0                 	mov	dword ptr [rax + 4*r14], ebx
14200fcf7: eb 66                       	jmp	0x14200fd5f <.text+0x200ed5f>
14200fcf9: 45 31 ff                    	xor	r15d, r15d
14200fcfc: 84 c9                       	test	cl, cl
14200fcfe: 74 cb                       	je	0x14200fccb <.text+0x200eccb>
14200fd00: 80 bc 24 20 01 00 00 00     	cmp	byte ptr [rsp + 0x120], 0x0
14200fd08: 74 34                       	je	0x14200fd3e <.text+0x200ed3e>
14200fd0a: 49 39 c5                    	cmp	r13, rax
14200fd0d: 76 50                       	jbe	0x14200fd5f <.text+0x200ed5f>
14200fd0f: 48 8b 4f 38                 	mov	rcx, qword ptr [rdi + 0x38]
14200fd13: 48 8b 04 c1                 	mov	rax, qword ptr [rcx + 8*rax]
14200fd17: 48 85 c0                    	test	rax, rax
14200fd1a: 74 43                       	je	0x14200fd5f <.text+0x200ed5f>
14200fd1c: 48 8b 4f 18                 	mov	rcx, qword ptr [rdi + 0x18]
14200fd20: 48 f7 d0                    	not	rax
14200fd23: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14200fd27: 48 c1 e0 04                 	shl	rax, 0x4
14200fd2b: 48 8b 44 01 18              	mov	rax, qword ptr [rcx + rax + 0x18]
14200fd30: 48 85 c0                    	test	rax, rax
14200fd33: 74 2a                       	je	0x14200fd5f <.text+0x200ed5f>
14200fd35: 48 8d 4c 24 40              	lea	rcx, [rsp + 0x40]
14200fd3a: ff d0                       	call	rax
14200fd3c: eb 21                       	jmp	0x14200fd5f <.text+0x200ed5f>
14200fd3e: 4d 89 e6                    	mov	r14, r12
14200fd41: 41 89 ec                    	mov	r12d, ebp
14200fd44: 4c 8d 44 24 40              	lea	r8, [rsp + 0x40]
14200fd49: 4c 89 f9                    	mov	rcx, r15
14200fd4c: 4c 89 e2                    	mov	rdx, r12
14200fd4f: e8 ac f6 ce fe              	call	0x140cff400 <.text+0xcfe400>
14200fd54: 49 8b 47 28                 	mov	rax, qword ptr [r15 + 0x28]
14200fd58: 42 89 1c a0                 	mov	dword ptr [rax + 4*r12], ebx
14200fd5c: 4d 89 f4                    	mov	r12, r14
14200fd5f: 49 8b 44 24 18              	mov	rax, qword ptr [r12 + 0x18]
14200fd64: 48 8b 8c 24 e0 00 00 00     	mov	rcx, qword ptr [rsp + 0xe0]
14200fd6c: 0f b6 49 03                 	movzx	ecx, byte ptr [rcx + 0x3]
14200fd70: 49 39 c5                    	cmp	r13, rax
14200fd73: 76 4e                       	jbe	0x14200fdc3 <.text+0x200edc3>
14200fd75: 48 8b 57 38                 	mov	rdx, qword ptr [rdi + 0x38]
14200fd79: 48 8b 14 c2                 	mov	rdx, qword ptr [rdx + 8*rax]
14200fd7d: 48 85 d2                    	test	rdx, rdx
14200fd80: 74 41                       	je	0x14200fdc3 <.text+0x200edc3>
14200fd82: 48 f7 d2                    	not	rdx
14200fd85: 4c 8d 3c 52                 	lea	r15, [rdx + 2*rdx]
14200fd89: 49 c1 e7 04                 	shl	r15, 0x4
14200fd8d: 4c 03 7f 18                 	add	r15, qword ptr [rdi + 0x18]
14200fd91: 84 c9                       	test	cl, cl
14200fd93: 75 35                       	jne	0x14200fdca <.text+0x200edca>
14200fd95: 41 89 ee                    	mov	r14d, ebp
14200fd98: 4d 8b 47 08                 	mov	r8, qword ptr [r15 + 0x8]
14200fd9c: 4c 89 c1                    	mov	rcx, r8
14200fd9f: 49 0f af ce                 	imul	rcx, r14
14200fda3: 49 03 4f 10                 	add	rcx, qword ptr [r15 + 0x10]
14200fda7: 48 8d 54 24 40              	lea	rdx, [rsp + 0x40]
14200fdac: e8 5f 3e 91 00              	call	0x142923c10 <.text+0x2922c10>
14200fdb1: 49 8b 47 20                 	mov	rax, qword ptr [r15 + 0x20]
14200fdb5: 42 89 1c b0                 	mov	dword ptr [rax + 4*r14], ebx
14200fdb9: 49 8b 47 28                 	mov	rax, qword ptr [r15 + 0x28]
14200fdbd: 42 89 1c b0                 	mov	dword ptr [rax + 4*r14], ebx
14200fdc1: eb 66                       	jmp	0x14200fe29 <.text+0x200ee29>
14200fdc3: 45 31 ff                    	xor	r15d, r15d
14200fdc6: 84 c9                       	test	cl, cl
14200fdc8: 74 cb                       	je	0x14200fd95 <.text+0x200ed95>
14200fdca: 80 bc 24 20 01 00 00 00     	cmp	byte ptr [rsp + 0x120], 0x0
14200fdd2: 74 34                       	je	0x14200fe08 <.text+0x200ee08>
14200fdd4: 49 39 c5                    	cmp	r13, rax
14200fdd7: 76 50                       	jbe	0x14200fe29 <.text+0x200ee29>
14200fdd9: 48 8b 4f 38                 	mov	rcx, qword ptr [rdi + 0x38]
14200fddd: 48 8b 04 c1                 	mov	rax, qword ptr [rcx + 8*rax]
14200fde1: 48 85 c0                    	test	rax, rax
14200fde4: 74 43                       	je	0x14200fe29 <.text+0x200ee29>
14200fde6: 48 8b 4f 18                 	mov	rcx, qword ptr [rdi + 0x18]
14200fdea: 48 f7 d0                    	not	rax
14200fded: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14200fdf1: 48 c1 e0 04                 	shl	rax, 0x4
14200fdf5: 48 8b 44 01 18              	mov	rax, qword ptr [rcx + rax + 0x18]
14200fdfa: 48 85 c0                    	test	rax, rax
14200fdfd: 74 2a                       	je	0x14200fe29 <.text+0x200ee29>
14200fdff: 48 8d 4c 24 40              	lea	rcx, [rsp + 0x40]
14200fe04: ff d0                       	call	rax
14200fe06: eb 21                       	jmp	0x14200fe29 <.text+0x200ee29>
14200fe08: 4d 89 e6                    	mov	r14, r12
14200fe0b: 41 89 ec                    	mov	r12d, ebp
14200fe0e: 4c 8d 44 24 40              	lea	r8, [rsp + 0x40]
14200fe13: 4c 89 f9                    	mov	rcx, r15
14200fe16: 4c 89 e2                    	mov	rdx, r12
14200fe19: e8 e2 f5 ce fe              	call	0x140cff400 <.text+0xcfe400>
14200fe1e: 49 8b 47 28                 	mov	rax, qword ptr [r15 + 0x28]
14200fe22: 42 89 1c a0                 	mov	dword ptr [rax + 4*r12], ebx
14200fe26: 4d 89 f4                    	mov	r12, r14
14200fe29: 49 8b 44 24 20              	mov	rax, qword ptr [r12 + 0x20]
14200fe2e: 48 8b 8c 24 e0 00 00 00     	mov	rcx, qword ptr [rsp + 0xe0]
14200fe36: 0f b6 51 04                 	movzx	edx, byte ptr [rcx + 0x4]
14200fe3a: 49 39 c5                    	cmp	r13, rax
14200fe3d: 76 67                       	jbe	0x14200fea6 <.text+0x200eea6>
14200fe3f: 48 8b 4f 38                 	mov	rcx, qword ptr [rdi + 0x38]
14200fe43: 48 8b 0c c1                 	mov	rcx, qword ptr [rcx + 8*rax]
14200fe47: 48 85 c9                    	test	rcx, rcx
14200fe4a: 74 5a                       	je	0x14200fea6 <.text+0x200eea6>
14200fe4c: 48 f7 d1                    	not	rcx
14200fe4f: 4c 8d 34 49                 	lea	r14, [rcx + 2*rcx]
14200fe53: 49 c1 e6 04                 	shl	r14, 0x4
14200fe57: 4c 03 77 18                 	add	r14, qword ptr [rdi + 0x18]
14200fe5b: 4c 8b a4 24 f8 00 00 00     	mov	r12, qword ptr [rsp + 0xf8]
14200fe63: 48 8b 8c 24 f0 00 00 00     	mov	rcx, qword ptr [rsp + 0xf0]
14200fe6b: 84 d2                       	test	dl, dl
14200fe6d: 75 4e                       	jne	0x14200febd <.text+0x200eebd>
14200fe6f: 41 89 ed                    	mov	r13d, ebp
14200fe72: 4d 8b 46 08                 	mov	r8, qword ptr [r14 + 0x8]
14200fe76: 4c 89 c0                    	mov	rax, r8
14200fe79: 49 0f af c5                 	imul	rax, r13
14200fe7d: 49 03 46 10                 	add	rax, qword ptr [r14 + 0x10]
14200fe81: 48 8d 54 24 40              	lea	rdx, [rsp + 0x40]
14200fe86: 49 89 cf                    	mov	r15, rcx
14200fe89: 48 89 c1                    	mov	rcx, rax
14200fe8c: e8 7f 3d 91 00              	call	0x142923c10 <.text+0x2922c10>
14200fe91: 4c 89 f9                    	mov	rcx, r15
14200fe94: 49 8b 46 20                 	mov	rax, qword ptr [r14 + 0x20]
14200fe98: 42 89 1c a8                 	mov	dword ptr [rax + 4*r13], ebx
14200fe9c: 49 8b 46 28                 	mov	rax, qword ptr [r14 + 0x28]
14200fea0: 42 89 1c a8                 	mov	dword ptr [rax + 4*r13], ebx
14200fea4: eb 7f                       	jmp	0x14200ff25 <.text+0x200ef25>
14200fea6: 45 31 f6                    	xor	r14d, r14d
14200fea9: 4c 8b a4 24 f8 00 00 00     	mov	r12, qword ptr [rsp + 0xf8]
14200feb1: 48 8b 8c 24 f0 00 00 00     	mov	rcx, qword ptr [rsp + 0xf0]
14200feb9: 84 d2                       	test	dl, dl
14200febb: 74 b2                       	je	0x14200fe6f <.text+0x200ee6f>
14200febd: 80 bc 24 20 01 00 00 00     	cmp	byte ptr [rsp + 0x120], 0x0
14200fec5: 74 3d                       	je	0x14200ff04 <.text+0x200ef04>
14200fec7: 49 39 c5                    	cmp	r13, rax
14200feca: 76 59                       	jbe	0x14200ff25 <.text+0x200ef25>
14200fecc: 48 8b 57 38                 	mov	rdx, qword ptr [rdi + 0x38]
14200fed0: 48 8b 04 c2                 	mov	rax, qword ptr [rdx + 8*rax]
14200fed4: 48 85 c0                    	test	rax, rax
14200fed7: 74 4c                       	je	0x14200ff25 <.text+0x200ef25>
14200fed9: 48 8b 57 18                 	mov	rdx, qword ptr [rdi + 0x18]
14200fedd: 48 f7 d0                    	not	rax
14200fee0: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14200fee4: 48 c1 e0 04                 	shl	rax, 0x4
14200fee8: 48 8b 54 02 18              	mov	rdx, qword ptr [rdx + rax + 0x18]
14200feed: 48 85 d2                    	test	rdx, rdx
14200fef0: 74 33                       	je	0x14200ff25 <.text+0x200ef25>
14200fef2: 48 8d 44 24 40              	lea	rax, [rsp + 0x40]
14200fef7: 49 89 ce                    	mov	r14, rcx
14200fefa: 48 89 c1                    	mov	rcx, rax
14200fefd: ff d2                       	call	rdx
14200feff: 4c 89 f1                    	mov	rcx, r14
14200ff02: eb 21                       	jmp	0x14200ff25 <.text+0x200ef25>
14200ff04: 41 89 ef                    	mov	r15d, ebp
14200ff07: 4c 8d 44 24 40              	lea	r8, [rsp + 0x40]
14200ff0c: 49 89 cd                    	mov	r13, rcx
14200ff0f: 4c 89 f1                    	mov	rcx, r14
14200ff12: 4c 89 fa                    	mov	rdx, r15
14200ff15: e8 e6 f4 ce fe              	call	0x140cff400 <.text+0xcfe400>
14200ff1a: 4c 89 e9                    	mov	rcx, r13
14200ff1d: 49 8b 46 28                 	mov	rax, qword ptr [r14 + 0x28]
14200ff21: 42 89 1c b8                 	mov	dword ptr [rax + 4*r15], ebx
14200ff25: 4c 39 e1                    	cmp	rcx, r12
14200ff28: 74 39                       	je	0x14200ff63 <.text+0x200ef63>
14200ff2a: 4c 8b b4 24 00 01 00 00     	mov	r14, qword ptr [rsp + 0x100]
14200ff32: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
14200ff40: 4c 8d 79 10                 	lea	r15, [rcx + 0x10]
14200ff44: 4c 89 74 24 28              	mov	qword ptr [rsp + 0x28], r14
14200ff49: 89 6c 24 20                 	mov	dword ptr [rsp + 0x20], ebp
14200ff4d: 48 89 fa                    	mov	rdx, rdi
14200ff50: 49 89 f0                    	mov	r8, rsi
14200ff53: 41 89 d9                    	mov	r9d, ebx
14200ff56: e8 35 09 d3 fe              	call	0x140d40890 <.text+0xd3f890>
14200ff5b: 4c 89 f9                    	mov	rcx, r15
14200ff5e: 4d 39 e7                    	cmp	r15, r12
14200ff61: 75 dd                       	jne	0x14200ff40 <.text+0x200ef40>
14200ff63: 48 83 c4 78                 	add	rsp, 0x78
14200ff67: 5b                          	pop	rbx
14200ff68: 5d                          	pop	rbp
14200ff69: 5f                          	pop	rdi
14200ff6a: 5e                          	pop	rsi
14200ff6b: 41 5c                       	pop	r12
14200ff6d: 41 5d                       	pop	r13
14200ff6f: 41 5e                       	pop	r14
14200ff71: 41 5f                       	pop	r15
14200ff73: c3                          	ret
