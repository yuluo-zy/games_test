
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b46920: 48 89 c8                    	movq	%rcx, %rax
140b46923: 0f 10 02                    	movups	(%rdx), %xmm0
140b46926: 0f 11 41 20                 	movups	%xmm0, 0x20(%rcx)
140b4692a: 48 b9 02 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffe, %rcx # imm = 0x8000000000000002
140b46934: 48 89 48 18                 	movq	%rcx, 0x18(%rax)
140b46938: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
140b4693f: 48 c7 40 08 08 00 00 00     	movq	$0x8, 0x8(%rax)
140b46947: 48 c7 40 10 00 00 00 00     	movq	$0x0, 0x10(%rax)
140b4694f: c6 80 98 00 00 00 00        	movb	$0x0, 0x98(%rax)
140b46956: c3                          	retq
140b46957: cc                          	int3
140b46958: cc                          	int3
140b46959: cc                          	int3
140b4695a: cc                          	int3
140b4695b: cc                          	int3
140b4695c: cc                          	int3
140b4695d: cc                          	int3
140b4695e: cc                          	int3
140b4695f: cc                          	int3
140b46960: 48 81 ec a8 00 00 00        	subq	$0xa8, %rsp
140b46967: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140b4696f: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
140b46977: 48 89 c8                    	movq	%rcx, %rax
140b4697a: 0f 10 42 50                 	movups	0x50(%rdx), %xmm0
140b4697e: 0f 29 44 24 50              	movaps	%xmm0, 0x50(%rsp)
140b46983: 0f 10 4a 40                 	movups	0x40(%rdx), %xmm1
140b46987: 0f 29 4c 24 40              	movaps	%xmm1, 0x40(%rsp)
140b4698c: 0f 10 12                    	movups	(%rdx), %xmm2
140b4698f: 0f 10 5a 10                 	movups	0x10(%rdx), %xmm3
140b46993: 0f 10 62 20                 	movups	0x20(%rdx), %xmm4
140b46997: 0f 10 6a 30                 	movups	0x30(%rdx), %xmm5
140b4699b: 0f 29 6c 24 30              	movaps	%xmm5, 0x30(%rsp)
140b469a0: 0f 29 64 24 20              	movaps	%xmm4, 0x20(%rsp)
140b469a5: 0f 29 5c 24 10              	movaps	%xmm3, 0x10(%rsp)
140b469aa: 0f 29 14 24                 	movaps	%xmm2, (%rsp)
140b469ae: 41 0f 10 30                 	movups	(%r8), %xmm6
140b469b2: 41 0f 10 78 10              	movups	0x10(%r8), %xmm7
140b469b7: 0f 29 74 24 60              	movaps	%xmm6, 0x60(%rsp)
140b469bc: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
140b469c1: 0f 11 b9 88 00 00 00        	movups	%xmm7, 0x88(%rcx)
140b469c8: 0f 11 71 78                 	movups	%xmm6, 0x78(%rcx)
140b469cc: 0f 11 41 68                 	movups	%xmm0, 0x68(%rcx)
140b469d0: 0f 11 49 58                 	movups	%xmm1, 0x58(%rcx)
140b469d4: 0f 11 69 48                 	movups	%xmm5, 0x48(%rcx)
140b469d8: 0f 11 61 38                 	movups	%xmm4, 0x38(%rcx)
140b469dc: 0f 11 59 28                 	movups	%xmm3, 0x28(%rcx)
140b469e0: 0f 11 51 18                 	movups	%xmm2, 0x18(%rcx)
140b469e4: 48 c7 01 00 00 00 00        	movq	$0x0, (%rcx)
140b469eb: 48 c7 41 08 08 00 00 00     	movq	$0x8, 0x8(%rcx)
140b469f3: 48 c7 41 10 00 00 00 00     	movq	$0x0, 0x10(%rcx)
140b469fb: c6 81 98 00 00 00 00        	movb	$0x0, 0x98(%rcx)
140b46a02: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
140b46a0a: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140b46a12: 48 81 c4 a8 00 00 00        	addq	$0xa8, %rsp
140b46a19: c3                          	retq
