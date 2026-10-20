1400d52f0 push rbp
1400d52f1 push rsi
1400d52f2 push rdi
1400d52f3 push rbx
1400d52f4 sub rsp, 0xac8
1400d52fb lea rbp, [rsp + 0x80]
1400d5303 mov qword ptr [rbp + 0xa40], 0xfffffffffffffffe
1400d530e mov rdi, rdx
1400d5311 mov rsi, rcx
1400d5314 movups xmm0, xmmword ptr [rdx + 0x60]
1400d5318 movaps xmmword ptr [rbp + 0x70], xmm0
1400d531c movups xmm0, xmmword ptr [rdx + 0x50]
1400d5320 movaps xmmword ptr [rbp + 0x60], xmm0
1400d5324 movups xmm0, xmmword ptr [rdx + 0x40]
1400d5328 movaps xmmword ptr [rbp + 0x50], xmm0
1400d532c movups xmm0, xmmword ptr [rdx]
1400d532f movups xmm1, xmmword ptr [rdx + 0x10]
1400d5333 movups xmm2, xmmword ptr [rdx + 0x20]
1400d5337 movups xmm3, xmmword ptr [rdx + 0x30]
1400d533b movaps xmmword ptr [rbp + 0x40], xmm3
1400d533f movaps xmmword ptr [rbp + 0x30], xmm2
1400d5343 movaps xmmword ptr [rbp + 0x20], xmm1
1400d5347 movaps xmmword ptr [rbp + 0x10], xmm0
1400d534b movups xmm0, xmmword ptr [rdx + 0xd0]
1400d5352 movaps xmmword ptr [rbp], xmm0
1400d5356 movups xmm0, xmmword ptr [rdx + 0xc0]
1400d535d movaps xmmword ptr [rbp - 0x10], xmm0
1400d5361 movups xmm0, xmmword ptr [rdx + 0xb0]
1400d5368 movaps xmmword ptr [rbp - 0x20], xmm0
1400d536c movups xmm0, xmmword ptr [rdx + 0x70]
1400d5370 movups xmm1, xmmword ptr [rdx + 0x80]
1400d5377 movups xmm2, xmmword ptr [rdx + 0x90]
1400d537e movups xmm3, xmmword ptr [rdx + 0xa0]
1400d5385 movaps xmmword ptr [rbp - 0x30], xmm3
1400d5389 movaps xmmword ptr [rbp - 0x40], xmm2
1400d538d movaps xmmword ptr [rbp - 0x50], xmm1
1400d5391 movaps xmmword ptr [rbp - 0x60], xmm0
1400d5395 movups xmm0, xmmword ptr [rdx + 0x140]
1400d539c movaps xmmword ptr [rbp + 0x930], xmm0
1400d53a3 movups xmm0, xmmword ptr [rdx + 0x130]
1400d53aa movaps xmmword ptr [rbp + 0x920], xmm0
1400d53b1 movups xmm0, xmmword ptr [rdx + 0x120]
1400d53b8 movaps xmmword ptr [rbp + 0x910], xmm0
1400d53bf movups xmm0, xmmword ptr [rdx + 0xe0]
1400d53c6 movups xmm1, xmmword ptr [rdx + 0xf0]
1400d53cd movups xmm2, xmmword ptr [rdx + 0x100]
1400d53d4 movups xmm3, xmmword ptr [rdx + 0x110]
1400d53db movaps xmmword ptr [rbp + 0x900], xmm3
1400d53e2 movaps xmmword ptr [rbp + 0x8f0], xmm2
1400d53e9 movaps xmmword ptr [rbp + 0x8e0], xmm1
1400d53f0 movaps xmmword ptr [rbp + 0x8d0], xmm0
1400d53f7 add rdx, 0x150
1400d53fe lea rcx, [rbp + 0x630]
1400d5405 mov r8d, 0xe0
1400d540b call 0x142923c00
1400d5410 movzx eax, byte ptr [rip + 0x355f7f2]
1400d5417 mov ecx, 0x850
1400d541c mov edx, 8
1400d5421 call 0x140613c10
1400d5426 mov qword ptr [rbp + 0xa28], rax
1400d542d test rax, rax
1400d5430 je 0x1400d6004
1400d5436 lea rax, [rdi + 0x70]
1400d543a movups xmm0, xmmword ptr [rdi + 0x60]
1400d543e movaps xmmword ptr [rbp + 0x8c0], xmm0
1400d5445 movups xmm0, xmmword ptr [rdi + 0x50]
1400d5449 movaps xmmword ptr [rbp + 0x8b0], xmm0
1400d5450 movups xmm0, xmmword ptr [rdi + 0x40]
1400d5454 movaps xmmword ptr [rbp + 0x8a0], xmm0
1400d545b movups xmm0, xmmword ptr [rdi]
1400d545e movups xmm1, xmmword ptr [rdi + 0x10]
1400d5462 movups xmm2, xmmword ptr [rdi + 0x20]
1400d5466 movups xmm3, xmmword ptr [rdi + 0x30]
1400d546a movaps xmmword ptr [rbp + 0x890], xmm3
1400d5471 movaps xmmword ptr [rbp + 0x880], xmm2
1400d5478 movaps xmmword ptr [rbp + 0x870], xmm1
1400d547f movaps xmmword ptr [rbp + 0x860], xmm0
1400d5486 movups xmm0, xmmword ptr [rax + 0x60]
1400d548a movaps xmmword ptr [rbp + 0x850], xmm0
1400d5491 movups xmm0, xmmword ptr [rax + 0x50]
1400d5495 movaps xmmword ptr [rbp + 0x840], xmm0
1400d549c movups xmm0, xmmword ptr [rax + 0x40]
1400d54a0 movaps xmmword ptr [rbp + 0x830], xmm0
1400d54a7 movups xmm0, xmmword ptr [rax]
1400d54aa movups xmm1, xmmword ptr [rax + 0x10]
1400d54ae movups xmm2, xmmword ptr [rax + 0x20]
1400d54b2 movups xmm3, xmmword ptr [rax + 0x30]
1400d54b6 movaps xmmword ptr [rbp + 0x820], xmm3
1400d54bd movaps xmmword ptr [rbp + 0x810], xmm2
1400d54c4 movaps xmmword ptr [rbp + 0x800], xmm1
1400d54cb movaps xmmword ptr [rbp + 0x7f0], xmm0
1400d54d2 mov byte ptr [rbp + 0xa3d], 1
1400d54d9 mov byte ptr [rbp + 0xa3c], 1
1400d54e0 lea rcx, [rbp + 0x5c0]
1400d54e7 call 0x1400c6200
1400d54ec movaps xmm0, xmmword ptr [rbp + 0x930]
1400d54f3 movaps xmmword ptr [rbp + 0x7e0], xmm0
1400d54fa movaps xmm0, xmmword ptr [rbp + 0x920]
1400d5501 movaps xmmword ptr [rbp + 0x7d0], xmm0
1400d5508 movaps xmm0, xmmword ptr [rbp + 0x910]
1400d550f movaps xmmword ptr [rbp + 0x7c0], xmm0
1400d5516 movaps xmm0, xmmword ptr [rbp + 0x8d0]
1400d551d movaps xmm1, xmmword ptr [rbp + 0x8e0]
1400d5524 movaps xmm2, xmmword ptr [rbp + 0x8f0]
1400d552b movaps xmm3, xmmword ptr [rbp + 0x900]
1400d5532 movaps xmmword ptr [rbp + 0x7b0], xmm3
1400d5539 movaps xmmword ptr [rbp + 0x7a0], xmm2
1400d5540 movaps xmmword ptr [rbp + 0x790], xmm1
1400d5547 movaps xmmword ptr [rbp + 0x780], xmm0
1400d554e mov byte ptr [rbp + 0xa3b], 1
1400d5555 lea rcx, [rbp + 0x550]
1400d555c call 0x1403e9f50
1400d5561 mov byte ptr [rbp + 0xa3a], 1
1400d5568 lea rcx, [rbp + 0x4e0]
1400d556f call 0x1403d6580
1400d5574 mov byte ptr [rbp + 0xa39], 1
1400d557b lea rcx, [rbp + 0x470]
1400d5582 call 0x140529c20
1400d5587 mov byte ptr [rbp + 0xa38], 1
1400d558e lea rcx, [rbp + 0x400]
1400d5595 call 0x1403d6a80
1400d559a mov byte ptr [rbp + 0xa37], 1
1400d55a1 lea rcx, [rbp + 0x390]
1400d55a8 call 0x140254d90
1400d55ad mov byte ptr [rbp + 0xa36], 1
1400d55b4 lea rcx, [rbp + 0x320]
1400d55bb call 0x1403d68d0
1400d55c0 mov byte ptr [rbp + 0xa35], 1
1400d55c7 lea rcx, [rbp + 0x2b0]
1400d55ce call 0x140104f60
1400d55d3 mov byte ptr [rbp + 0xa34], 1
1400d55da lea rcx, [rbp + 0x240]
1400d55e1 call 0x140285ee0
1400d55e6 mov byte ptr [rbp + 0xa33], 1
1400d55ed lea rcx, [rbp + 0x1d0]
1400d55f4 call 0x140105110
1400d55f9 mov byte ptr [rbp + 0xa32], 1
1400d5600 lea rcx, [rbp + 0x160]
1400d5607 call 0x140489a80
1400d560c movups xmm0, xmmword ptr [rbp + 0x690]
1400d5613 movaps xmmword ptr [rbp + 0x9a0], xmm0
1400d561a movups xmm0, xmmword ptr [rbp + 0x680]
1400d5621 movaps xmmword ptr [rbp + 0x990], xmm0
1400d5628 movups xmm0, xmmword ptr [rbp + 0x670]
1400d562f movaps xmmword ptr [rbp + 0x980], xmm0
1400d5636 movups xmm0, xmmword ptr [rbp + 0x630]
1400d563d movups xmm1, xmmword ptr [rbp + 0x640]
1400d5644 movups xmm2, xmmword ptr [rbp + 0x650]
1400d564b movups xmm3, xmmword ptr [rbp + 0x660]
1400d5652 movaps xmmword ptr [rbp + 0x970], xmm3
1400d5659 movaps xmmword ptr [rbp + 0x960], xmm2
1400d5660 movaps xmmword ptr [rbp + 0x950], xmm1
1400d5667 movaps xmmword ptr [rbp + 0x940], xmm0
1400d566e movups xmm0, xmmword ptr [rbp + 0x700]
1400d5675 movaps xmmword ptr [rbp + 0xa10], xmm0
1400d567c movups xmm0, xmmword ptr [rbp + 0x6f0]
1400d5683 movaps xmmword ptr [rbp + 0xa00], xmm0
1400d568a movups xmm0, xmmword ptr [rbp + 0x6e0]
1400d5691 movaps xmmword ptr [rbp + 0x9f0], xmm0
1400d5698 movups xmm0, xmmword ptr [rbp + 0x6d0]
1400d569f movaps xmmword ptr [rbp + 0x9e0], xmm0
1400d56a6 movups xmm0, xmmword ptr [rbp + 0x6c0]
1400d56ad movaps xmmword ptr [rbp + 0x9d0], xmm0
1400d56b4 movups xmm0, xmmword ptr [rbp + 0x6b0]
1400d56bb movaps xmmword ptr [rbp + 0x9c0], xmm0
1400d56c2 movups xmm0, xmmword ptr [rbp + 0x6a0]
1400d56c9 movaps xmmword ptr [rbp + 0x9b0], xmm0
1400d56d0 movzx eax, byte ptr [rip + 0x355f532]
1400d56d7 mov ecx, 0xe0
1400d56dc mov edx, 8
1400d56e1 call 0x140613c10
1400d56e6 test rax, rax
1400d56e9 je 0x1400d6023
1400d56ef lea rcx, [rbp + 0x6a0]
1400d56f6 movups xmm0, xmmword ptr [rbp + 0x690]
1400d56fd movups xmmword ptr [rax + 0x60], xmm0
1400d5701 movups xmm0, xmmword ptr [rbp + 0x680]
1400d5708 movups xmmword ptr [rax + 0x50], xmm0
1400d570c movups xmm0, xmmword ptr [rbp + 0x670]
1400d5713 movups xmmword ptr [rax + 0x40], xmm0
1400d5717 movups xmm0, xmmword ptr [rbp + 0x630]
1400d571e movups xmm1, xmmword ptr [rbp + 0x640]
1400d5725 movups xmm2, xmmword ptr [rbp + 0x650]
1400d572c movups xmm3, xmmword ptr [rbp + 0x660]
1400d5733 movups xmmword ptr [rax + 0x30], xmm3
1400d5737 movups xmmword ptr [rax + 0x20], xmm2
1400d573b movups xmmword ptr [rax + 0x10], xmm1
1400d573f movups xmmword ptr [rax], xmm0
1400d5742 movups xmm0, xmmword ptr [rcx]
1400d5745 movups xmm1, xmmword ptr [rcx + 0x10]
1400d5749 movups xmm2, xmmword ptr [rcx + 0x20]
1400d574d movups xmm3, xmmword ptr [rcx + 0x30]
1400d5751 movups xmmword ptr [rax + 0x70], xmm0
1400d5755 movups xmmword ptr [rax + 0x80], xmm1
1400d575c movups xmmword ptr [rax + 0x90], xmm2
1400d5763 movups xmmword ptr [rax + 0xa0], xmm3
1400d576a movups xmm0, xmmword ptr [rcx + 0x40]
1400d576e movups xmmword ptr [rax + 0xb0], xmm0
1400d5775 movups xmm0, xmmword ptr [rcx + 0x50]
1400d5779 movups xmmword ptr [rax + 0xc0], xmm0
1400d5780 movups xmm0, xmmword ptr [rcx + 0x60]
1400d5784 movups xmmword ptr [rax + 0xd0], xmm0
1400d578b mov qword ptr [rbp + 0x718], 2
1400d5796 mov qword ptr [rbp + 0x720], rax
1400d579d mov qword ptr [rbp + 0x728], 2
1400d57a8 mov qword ptr [rbp + 0x730], 0
1400d57b3 mov qword ptr [rbp + 0x738], 8
1400d57be xorps xmm0, xmm0
1400d57c1 movups xmmword ptr [rbp + 0x740], xmm0
1400d57c8 movabs rdi, 0x8000000000000000
1400d57d2 mov qword ptr [rbp + 0x710], rdi
1400d57d9 lea rcx, [rbp + 0xf0]
1400d57e0 call 0x1402de9d0
1400d57e5 lea rcx, [rbp + 0x80]
1400d57ec call 0x140254d90
1400d57f1 lea rcx, [rbp + 0x940]
1400d57f8 call 0x1400e5010
1400d57fd lea rcx, [rbp + 0x9b0]
1400d5804 call 0x1403c3860
1400d5809 movaps xmm0, xmmword ptr [rbp + 0x8c0]
1400d5810 mov rax, qword ptr [rbp + 0xa28]
1400d5817 movups xmmword ptr [rax + 0x60], xmm0
1400d581b movaps xmm0, xmmword ptr [rbp + 0x8b0]
1400d5822 movups xmmword ptr [rax + 0x50], xmm0
1400d5826 movaps xmm0, xmmword ptr [rbp + 0x8a0]
1400d582d movups xmmword ptr [rax + 0x40], xmm0
1400d5831 movaps xmm0, xmmword ptr [rbp + 0x860]
1400d5838 movaps xmm1, xmmword ptr [rbp + 0x870]
1400d583f movaps xmm2, xmmword ptr [rbp + 0x880]
1400d5846 movaps xmm3, xmmword ptr [rbp + 0x890]
1400d584d movups xmmword ptr [rax + 0x30], xmm3
1400d5851 movups xmmword ptr [rax + 0x20], xmm2
1400d5855 movups xmmword ptr [rax + 0x10], xmm1
1400d5859 movups xmmword ptr [rax], xmm0
1400d585c movaps xmm0, xmmword ptr [rbp + 0x7f0]
1400d5863 movaps xmm1, xmmword ptr [rbp + 0x800]
1400d586a movaps xmm2, xmmword ptr [rbp + 0x810]
1400d5871 movaps xmm3, xmmword ptr [rbp + 0x820]
1400d5878 movups xmmword ptr [rax + 0x70], xmm0
1400d587c movups xmmword ptr [rax + 0x80], xmm1
1400d5883 movups xmmword ptr [rax + 0x90], xmm2
1400d588a movups xmmword ptr [rax + 0xa0], xmm3
1400d5891 movaps xmm0, xmmword ptr [rbp + 0x830]
1400d5898 movups xmmword ptr [rax + 0xb0], xmm0
1400d589f movaps xmm0, xmmword ptr [rbp + 0x840]
1400d58a6 movups xmmword ptr [rax + 0xc0], xmm0
1400d58ad movaps xmm0, xmmword ptr [rbp + 0x850]
1400d58b4 movups xmmword ptr [rax + 0xd0], xmm0
1400d58bb movups xmm0, xmmword ptr [rbp + 0x620]
1400d58c2 movups xmmword ptr [rax + 0x140], xmm0
1400d58c9 movups xmm0, xmmword ptr [rbp + 0x610]
1400d58d0 movups xmmword ptr [rax + 0x130], xmm0
1400d58d7 movups xmm0, xmmword ptr [rbp + 0x600]
1400d58de movups xmmword ptr [rax + 0x120], xmm0
1400d58e5 movups xmm0, xmmword ptr [rbp + 0x5c0]
1400d58ec movups xmm1, xmmword ptr [rbp + 0x5d0]
1400d58f3 movups xmm2, xmmword ptr [rbp + 0x5e0]
1400d58fa movups xmm3, xmmword ptr [rbp + 0x5f0]
1400d5901 movups xmmword ptr [rax + 0x110], xmm3
1400d5908 movups xmmword ptr [rax + 0x100], xmm2
1400d590f movups xmmword ptr [rax + 0xf0], xmm1
1400d5916 movups xmmword ptr [rax + 0xe0], xmm0
1400d591d movaps xmm0, xmmword ptr [rbp + 0x780]
1400d5924 movaps xmm1, xmmword ptr [rbp + 0x790]
1400d592b movaps xmm2, xmmword ptr [rbp + 0x7a0]
1400d5932 movaps xmm3, xmmword ptr [rbp + 0x7b0]
1400d5939 movups xmmword ptr [rax + 0x150], xmm0
1400d5940 movups xmmword ptr [rax + 0x160], xmm1
1400d5947 movups xmmword ptr [rax + 0x170], xmm2
1400d594e movups xmmword ptr [rax + 0x180], xmm3
1400d5955 movaps xmm0, xmmword ptr [rbp + 0x7c0]
1400d595c movups xmmword ptr [rax + 0x190], xmm0
1400d5963 movaps xmm0, xmmword ptr [rbp + 0x7d0]
1400d596a movups xmmword ptr [rax + 0x1a0], xmm0
1400d5971 movaps xmm0, xmmword ptr [rbp + 0x7e0]
1400d5978 movups xmmword ptr [rax + 0x1b0], xmm0
1400d597f movups xmm0, xmmword ptr [rbp + 0x550]
1400d5986 movups xmm1, xmmword ptr [rbp + 0x560]
1400d598d movups xmm2, xmmword ptr [rbp + 0x570]
1400d5994 movups xmm3, xmmword ptr [rbp + 0x580]
1400d599b movups xmmword ptr [rax + 0x1c0], xmm0
1400d59a2 movups xmmword ptr [rax + 0x1d0], xmm1
1400d59a9 movups xmmword ptr [rax + 0x1e0], xmm2
1400d59b0 movups xmmword ptr [rax + 0x1f0], xmm3
1400d59b7 movups xmm0, xmmword ptr [rbp + 0x590]
1400d59be movups xmmword ptr [rax + 0x200], xmm0
1400d59c5 movups xmm0, xmmword ptr [rbp + 0x5b0]
1400d59cc movups xmmword ptr [rax + 0x220], xmm0
1400d59d3 movups xmm0, xmmword ptr [rbp + 0x5a0]
1400d59da movups xmmword ptr [rax + 0x210], xmm0
1400d59e1 movups xmm0, xmmword ptr [rbp + 0x540]
1400d59e8 movups xmmword ptr [rax + 0x290], xmm0
1400d59ef movups xmm0, xmmword ptr [rbp + 0x530]
1400d59f6 movups xmmword ptr [rax + 0x280], xmm0
1400d59fd movups xmm0, xmmword ptr [rbp + 0x520]
1400d5a04 movups xmmword ptr [rax + 0x270], xmm0
1400d5a0b movups xmm0, xmmword ptr [rbp + 0x4e0]
1400d5a12 movups xmm1, xmmword ptr [rbp + 0x4f0]
1400d5a19 movups xmm2, xmmword ptr [rbp + 0x500]
1400d5a20 movups xmm3, xmmword ptr [rbp + 0x510]
1400d5a27 movups xmmword ptr [rax + 0x260], xmm3
1400d5a2e movups xmmword ptr [rax + 0x250], xmm2
1400d5a35 movups xmmword ptr [rax + 0x240], xmm1
1400d5a3c movups xmmword ptr [rax + 0x230], xmm0
1400d5a43 movups xmm0, xmmword ptr [rbp + 0x470]
1400d5a4a movups xmm1, xmmword ptr [rbp + 0x480]
1400d5a51 movups xmm2, xmmword ptr [rbp + 0x490]
1400d5a58 movups xmm3, xmmword ptr [rbp + 0x4a0]
1400d5a5f movups xmmword ptr [rax + 0x2a0], xmm0
1400d5a66 movups xmmword ptr [rax + 0x2b0], xmm1
1400d5a6d movups xmmword ptr [rax + 0x2c0], xmm2
1400d5a74 movups xmmword ptr [rax + 0x2d0], xmm3
1400d5a7b movups xmm0, xmmword ptr [rbp + 0x4b0]
1400d5a82 movups xmmword ptr [rax + 0x2e0], xmm0
1400d5a89 movups xmm0, xmmword ptr [rbp + 0x4c0]
1400d5a90 movups xmmword ptr [rax + 0x2f0], xmm0
1400d5a97 movups xmm0, xmmword ptr [rbp + 0x4d0]
1400d5a9e movups xmmword ptr [rax + 0x300], xmm0
1400d5aa5 movups xmm0, xmmword ptr [rbp + 0x400]
1400d5aac movups xmm1, xmmword ptr [rbp + 0x410]
1400d5ab3 movups xmm2, xmmword ptr [rbp + 0x420]
1400d5aba movups xmm3, xmmword ptr [rbp + 0x430]
1400d5ac1 movups xmmword ptr [rax + 0x310], xmm0
1400d5ac8 movups xmmword ptr [rax + 0x320], xmm1
1400d5acf movups xmmword ptr [rax + 0x330], xmm2
1400d5ad6 movups xmmword ptr [rax + 0x340], xmm3
1400d5add movups xmm0, xmmword ptr [rbp + 0x440]
1400d5ae4 movups xmmword ptr [rax + 0x350], xmm0
1400d5aeb movups xmm0, xmmword ptr [rbp + 0x450]
1400d5af2 movups xmmword ptr [rax + 0x360], xmm0
1400d5af9 movups xmm0, xmmword ptr [rbp + 0x460]
1400d5b00 movups xmmword ptr [rax + 0x370], xmm0
1400d5b07 movups xmm0, xmmword ptr [rbp + 0x390]
1400d5b0e movups xmm1, xmmword ptr [rbp + 0x3a0]
1400d5b15 movups xmm2, xmmword ptr [rbp + 0x3b0]
1400d5b1c movups xmm3, xmmword ptr [rbp + 0x3c0]
1400d5b23 movups xmmword ptr [rax + 0x380], xmm0
1400d5b2a movups xmmword ptr [rax + 0x390], xmm1
1400d5b31 movups xmmword ptr [rax + 0x3a0], xmm2
1400d5b38 movups xmmword ptr [rax + 0x3b0], xmm3
1400d5b3f movups xmm0, xmmword ptr [rbp + 0x3d0]
1400d5b46 movups xmmword ptr [rax + 0x3c0], xmm0
1400d5b4d movups xmm0, xmmword ptr [rbp + 0x3e0]
1400d5b54 movups xmmword ptr [rax + 0x3d0], xmm0
1400d5b5b movups xmm0, xmmword ptr [rbp + 0x3f0]
1400d5b62 movups xmmword ptr [rax + 0x3e0], xmm0
1400d5b69 movups xmm0, xmmword ptr [rbp + 0x320]
1400d5b70 movups xmm1, xmmword ptr [rbp + 0x330]
1400d5b77 movups xmm2, xmmword ptr [rbp + 0x340]
1400d5b7e movups xmm3, xmmword ptr [rbp + 0x350]
1400d5b85 movups xmmword ptr [rax + 0x3f0], xmm0
1400d5b8c movups xmmword ptr [rax + 0x400], xmm1
1400d5b93 movups xmmword ptr [rax + 0x410], xmm2
1400d5b9a movups xmmword ptr [rax + 0x420], xmm3
1400d5ba1 movups xmm0, xmmword ptr [rbp + 0x360]
1400d5ba8 movups xmmword ptr [rax + 0x430], xmm0
1400d5baf movups xmm0, xmmword ptr [rbp + 0x370]
1400d5bb6 movups xmmword ptr [rax + 0x440], xmm0
1400d5bbd movups xmm0, xmmword ptr [rbp + 0x380]
1400d5bc4 movups xmmword ptr [rax + 0x450], xmm0
1400d5bcb movups xmm0, xmmword ptr [rbp + 0x2b0]
1400d5bd2 movups xmm1, xmmword ptr [rbp + 0x2c0]
1400d5bd9 movups xmm2, xmmword ptr [rbp + 0x2d0]
1400d5be0 movups xmm3, xmmword ptr [rbp + 0x2e0]
1400d5be7 movups xmmword ptr [rax + 0x460], xmm0
1400d5bee movups xmmword ptr [rax + 0x470], xmm1
1400d5bf5 movups xmmword ptr [rax + 0x480], xmm2
1400d5bfc movups xmmword ptr [rax + 0x490], xmm3
1400d5c03 movups xmm0, xmmword ptr [rbp + 0x2f0]
1400d5c0a movups xmmword ptr [rax + 0x4a0], xmm0
1400d5c11 movups xmm0, xmmword ptr [rbp + 0x300]
1400d5c18 movups xmmword ptr [rax + 0x4b0], xmm0
1400d5c1f movups xmm0, xmmword ptr [rbp + 0x310]
1400d5c26 movups xmmword ptr [rax + 0x4c0], xmm0
1400d5c2d movups xmm0, xmmword ptr [rbp + 0x240]
1400d5c34 movups xmm1, xmmword ptr [rbp + 0x250]
1400d5c3b movups xmm2, xmmword ptr [rbp + 0x260]
1400d5c42 movups xmm3, xmmword ptr [rbp + 0x270]
1400d5c49 movups xmmword ptr [rax + 0x4d0], xmm0
1400d5c50 movups xmmword ptr [rax + 0x4e0], xmm1
1400d5c57 movups xmmword ptr [rax + 0x4f0], xmm2
1400d5c5e movups xmmword ptr [rax + 0x500], xmm3
1400d5c65 movups xmm0, xmmword ptr [rbp + 0x280]
1400d5c6c movups xmmword ptr [rax + 0x510], xmm0
1400d5c73 movups xmm0, xmmword ptr [rbp + 0x290]
1400d5c7a movups xmmword ptr [rax + 0x520], xmm0
1400d5c81 movups xmm0, xmmword ptr [rbp + 0x2a0]
1400d5c88 movups xmmword ptr [rax + 0x530], xmm0
1400d5c8f movups xmm0, xmmword ptr [rbp + 0x1d0]
1400d5c96 movups xmm1, xmmword ptr [rbp + 0x1e0]
1400d5c9d movups xmm2, xmmword ptr [rbp + 0x1f0]
1400d5ca4 movups xmm3, xmmword ptr [rbp + 0x200]
1400d5cab movups xmmword ptr [rax + 0x540], xmm0
1400d5cb2 movups xmmword ptr [rax + 0x550], xmm1
1400d5cb9 movups xmmword ptr [rax + 0x560], xmm2
1400d5cc0 movups xmmword ptr [rax + 0x570], xmm3
1400d5cc7 movups xmm0, xmmword ptr [rbp + 0x210]
1400d5cce movups xmmword ptr [rax + 0x580], xmm0
1400d5cd5 movups xmm0, xmmword ptr [rbp + 0x220]
1400d5cdc movups xmmword ptr [rax + 0x590], xmm0
1400d5ce3 movups xmm0, xmmword ptr [rbp + 0x230]
1400d5cea movups xmmword ptr [rax + 0x5a0], xmm0
1400d5cf1 movups xmm0, xmmword ptr [rbp + 0x160]
1400d5cf8 movups xmm1, xmmword ptr [rbp + 0x170]
1400d5cff movups xmm2, xmmword ptr [rbp + 0x180]
1400d5d06 movups xmm3, xmmword ptr [rbp + 0x190]
1400d5d0d movups xmmword ptr [rax + 0x5b0], xmm0
1400d5d14 movups xmmword ptr [rax + 0x5c0], xmm1
1400d5d1b movups xmmword ptr [rax + 0x5d0], xmm2
1400d5d22 movups xmmword ptr [rax + 0x5e0], xmm3
1400d5d29 movups xmm0, xmmword ptr [rbp + 0x1a0]
1400d5d30 movups xmmword ptr [rax + 0x5f0], xmm0
1400d5d37 movups xmm0, xmmword ptr [rbp + 0x1b0]
1400d5d3e movups xmmword ptr [rax + 0x600], xmm0
1400d5d45 movups xmm0, xmmword ptr [rbp + 0x1c0]
1400d5d4c movups xmmword ptr [rax + 0x610], xmm0
1400d5d53 movups xmm0, xmmword ptr [rbp + 0x710]
1400d5d5a movups xmm1, xmmword ptr [rbp + 0x720]
1400d5d61 movups xmm2, xmmword ptr [rbp + 0x730]
1400d5d68 movups xmm3, xmmword ptr [rbp + 0x740]
1400d5d6f movups xmmword ptr [rax + 0x620], xmm0
1400d5d76 movups xmmword ptr [rax + 0x630], xmm1
1400d5d7d movups xmmword ptr [rax + 0x640], xmm2
1400d5d84 movups xmmword ptr [rax + 0x650], xmm3
1400d5d8b movups xmm0, xmmword ptr [rbp + 0x750]
1400d5d92 movups xmmword ptr [rax + 0x660], xmm0
1400d5d99 movups xmm0, xmmword ptr [rbp + 0x760]
1400d5da0 movups xmmword ptr [rax + 0x670], xmm0
1400d5da7 movups xmm0, xmmword ptr [rbp + 0x770]
1400d5dae movups xmmword ptr [rax + 0x680], xmm0
1400d5db5 movups xmm0, xmmword ptr [rbp + 0xf0]
1400d5dbc movups xmm1, xmmword ptr [rbp + 0x100]
1400d5dc3 movups xmm2, xmmword ptr [rbp + 0x110]
1400d5dca movups xmm3, xmmword ptr [rbp + 0x120]
1400d5dd1 movups xmmword ptr [rax + 0x690], xmm0
1400d5dd8 movups xmmword ptr [rax + 0x6a0], xmm1
1400d5ddf movups xmmword ptr [rax + 0x6b0], xmm2
1400d5de6 movups xmmword ptr [rax + 0x6c0], xmm3
1400d5ded movups xmm0, xmmword ptr [rbp + 0x130]
1400d5df4 movups xmmword ptr [rax + 0x6d0], xmm0
1400d5dfb movups xmm0, xmmword ptr [rbp + 0x140]
1400d5e02 movups xmmword ptr [rax + 0x6e0], xmm0
1400d5e09 movups xmm0, xmmword ptr [rbp + 0x150]
1400d5e10 movups xmmword ptr [rax + 0x6f0], xmm0
1400d5e17 movups xmm0, xmmword ptr [rbp + 0x80]
1400d5e1e movups xmm1, xmmword ptr [rbp + 0x90]
1400d5e25 movups xmm2, xmmword ptr [rbp + 0xa0]
1400d5e2c movups xmm3, xmmword ptr [rbp + 0xb0]
1400d5e33 movups xmmword ptr [rax + 0x700], xmm0
1400d5e3a movups xmmword ptr [rax + 0x710], xmm1
1400d5e41 movups xmmword ptr [rax + 0x720], xmm2
1400d5e48 movups xmmword ptr [rax + 0x730], xmm3
1400d5e4f movups xmm0, xmmword ptr [rbp + 0xc0]
1400d5e56 movups xmmword ptr [rax + 0x740], xmm0
1400d5e5d movups xmm0, xmmword ptr [rbp + 0xd0]
1400d5e64 movups xmmword ptr [rax + 0x750], xmm0
1400d5e6b movups xmm0, xmmword ptr [rbp + 0xe0]
1400d5e72 movups xmmword ptr [rax + 0x760], xmm0
1400d5e79 movups xmm0, xmmword ptr [rbp + 0x940]
1400d5e80 movups xmm1, xmmword ptr [rbp + 0x950]
1400d5e87 movups xmm2, xmmword ptr [rbp + 0x960]
1400d5e8e movups xmm3, xmmword ptr [rbp + 0x970]
1400d5e95 movups xmmword ptr [rax + 0x770], xmm0
1400d5e9c movups xmmword ptr [rax + 0x780], xmm1
1400d5ea3 movups xmmword ptr [rax + 0x790], xmm2
1400d5eaa movups xmmword ptr [rax + 0x7a0], xmm3
1400d5eb1 movups xmm0, xmmword ptr [rbp + 0x980]
1400d5eb8 movups xmmword ptr [rax + 0x7b0], xmm0
1400d5ebf movups xmm0, xmmword ptr [rbp + 0x990]
1400d5ec6 movups xmmword ptr [rax + 0x7c0], xmm0
1400d5ecd movups xmm0, xmmword ptr [rbp + 0x9a0]
1400d5ed4 movups xmmword ptr [rax + 0x7d0], xmm0
1400d5edb movups xmm0, xmmword ptr [rbp + 0x9b0]
1400d5ee2 movups xmm1, xmmword ptr [rbp + 0x9c0]
1400d5ee9 movups xmm2, xmmword ptr [rbp + 0x9d0]
1400d5ef0 movups xmm3, xmmword ptr [rbp + 0x9e0]
1400d5ef7 movups xmmword ptr [rax + 0x7e0], xmm0
1400d5efe movups xmmword ptr [rax + 0x7f0], xmm1
1400d5f05 movups xmmword ptr [rax + 0x800], xmm2
1400d5f0c movups xmmword ptr [rax + 0x810], xmm3
1400d5f13 movups xmm0, xmmword ptr [rbp + 0x9f0]
1400d5f1a movups xmmword ptr [rax + 0x820], xmm0
1400d5f21 movups xmm0, xmmword ptr [rbp + 0xa00]
1400d5f28 movups xmmword ptr [rax + 0x830], xmm0
1400d5f2f movups xmm0, xmmword ptr [rbp + 0xa10]
1400d5f36 movups xmmword ptr [rax + 0x840], xmm0
1400d5f3d mov qword ptr [rbp + 0x9b8], 0x13
1400d5f48 mov qword ptr [rbp + 0x9c0], rax
1400d5f4f mov qword ptr [rbp + 0x9c8], 0x13
1400d5f5a mov qword ptr [rbp + 0x9d0], 0
1400d5f65 mov qword ptr [rbp + 0x9d8], 8
1400d5f70 lea rcx, [rbp + 0x9e8]
1400d5f77 xorps xmm0, xmm0
1400d5f7a movups xmmword ptr [rbp + 0x9e0], xmm0
1400d5f81 mov rax, qword ptr [rbp + 0x950]
1400d5f88 mov qword ptr [rbp + 0xa00], rax
1400d5f8f movups xmm0, xmmword ptr [rbp + 0x940]
1400d5f96 movups xmmword ptr [rbp + 0x9f0], xmm0
1400d5f9d mov qword ptr [rbp + 0x9b0], rdi
1400d5fa4 call 0x140d2fa20
1400d5fa9 movups xmm0, xmmword ptr [rbp + 0xa10]
1400d5fb0 movups xmmword ptr [rsi + 0x60], xmm0
1400d5fb4 movups xmm0, xmmword ptr [rbp + 0xa00]
1400d5fbb movups xmmword ptr [rsi + 0x50], xmm0
1400d5fbf movups xmm0, xmmword ptr [rbp + 0x9f0]
1400d5fc6 movups xmmword ptr [rsi + 0x40], xmm0
1400d5fca movups xmm0, xmmword ptr [rbp + 0x9b0]
1400d5fd1 movups xmm1, xmmword ptr [rbp + 0x9c0]
1400d5fd8 movups xmm2, xmmword ptr [rbp + 0x9d0]
1400d5fdf movups xmm3, xmmword ptr [rbp + 0x9e0]
1400d5fe6 movups xmmword ptr [rsi + 0x30], xmm3
1400d5fea movups xmmword ptr [rsi + 0x20], xmm2
1400d5fee movups xmmword ptr [rsi + 0x10], xmm1
1400d5ff2 movups xmmword ptr [rsi], xmm0
1400d5ff5 mov rax, rsi
1400d5ff8 add rsp, 0xac8
1400d5fff pop rbx
1400d6000 pop rdi
1400d6001 pop rsi
1400d6002 pop rbp
1400d6003 ret 
1400d6004 mov byte ptr [rbp + 0xa3f], 1
1400d600b mov byte ptr [rbp + 0xa3e], 1
1400d6012 mov ecx, 8
1400d6017 mov edx, 0x850
1400d601c call 0x1428d9000
1400d6021 jmp 0x1400d6032
1400d6023 mov ecx, 8
1400d6028 mov edx, 0xe0
1400d602d call 0x1428d9000
1400d6032 ud2 
1400d6034 nop word ptr cs:[rax + rax]
1400d6040 mov qword ptr [rsp + 0x10], rdx
1400d6045 push rbp
1400d6046 push rsi
1400d6047 push rdi
1400d6048 push rbx
1400d6049 sub rsp, 0x28
1400d604d lea rbp, [rdx + 0x80]
1400d6054 movzx ebx, byte ptr [rbp + 0xa3c]
1400d605b movzx esi, byte ptr [rbp + 0xa3d]
1400d6062 lea rcx, [rbp + 0x7f0]
1400d6069 call 0x140029880
1400d606e lea rcx, [rbp + 0x860]
1400d6075 call 0x140029880
1400d607a mov edx, 0x850
1400d607f mov r8d, 8
1400d6085 mov rcx, qword ptr [rbp + 0xa28]
1400d608c call 0x140613c20
1400d6091 mov byte ptr [rbp + 0xa3f], sil
1400d6098 mov byte ptr [rbp + 0xa3e], bl
1400d609e add rsp, 0x28
1400d60a2 pop rbx
1400d60a3 pop rdi
1400d60a4 pop rsi
1400d60a5 pop rbp
1400d60a6 ret 
1400d60a7 nop word ptr [rax + rax]
1400d60b0 mov qword ptr [rsp + 0x10], rdx
1400d60b5 push rbp
1400d60b6 push rsi
1400d60b7 push rdi
1400d60b8 push rbx
1400d60b9 sub rsp, 0x28
1400d60bd lea rbp, [rdx + 0x80]
1400d60c4 lea rcx, [rbp + 0x9b0]
1400d60cb call 0x140029880
1400d60d0 lea rcx, [rbp + 0x940]
1400d60d7 call 0x140029880
1400d60dc nop 
1400d60dd add rsp, 0x28
1400d60e1 pop rbx
1400d60e2 pop rdi
1400d60e3 pop rsi
1400d60e4 pop rbp
1400d60e5 ret 
1400d60e6 nop word ptr cs:[rax + rax]
1400d60f0 mov qword ptr [rsp + 0x10], rdx
1400d60f5 push rbp
1400d60f6 push rsi
1400d60f7 push rdi
1400d60f8 push rbx
1400d60f9 sub rsp, 0x28
1400d60fd lea rbp, [rdx + 0x80]
1400d6104 lea rcx, [rbp + 0x160]
1400d610b call 0x140029880
1400d6110 mov byte ptr [rbp + 0xa32], 0
1400d6117 add rsp, 0x28
1400d611b pop rbx
1400d611c pop rdi
1400d611d pop rsi
1400d611e pop rbp
1400d611f ret 
1400d6120 mov qword ptr [rsp + 0x10], rdx
1400d6125 push rbp
1400d6126 push rsi
1400d6127 push rdi
1400d6128 push rbx
1400d6129 sub rsp, 0x28
1400d612d lea rbp, [rdx + 0x80]
1400d6134 lea rcx, [rbp + 0x710]
1400d613b call 0x140029880
1400d6140 nop 
1400d6141 add rsp, 0x28
1400d6145 pop rbx
1400d6146 pop rdi
1400d6147 pop rsi
1400d6148 pop rbp
1400d6149 ret 
1400d614a nop word ptr [rax + rax]
1400d6150 mov qword ptr [rsp + 0x10], rdx
1400d6155 push rbp
1400d6156 push rsi
1400d6157 push rdi
1400d6158 push rbx
1400d6159 sub rsp, 0x28
1400d615d lea rbp, [rdx + 0x80]
1400d6164 lea rcx, [rbp + 0x940]
1400d616b call 0x140029880
1400d6170 nop 
1400d6171 add rsp, 0x28
1400d6175 pop rbx
1400d6176 pop rdi
1400d6177 pop rsi
1400d6178 pop rbp
1400d6179 ret 
1400d617a nop word ptr [rax + rax]
1400d6180 mov qword ptr [rsp + 0x10], rdx
1400d6185 push rbp
1400d6186 push rsi
1400d6187 push rdi
1400d6188 push rbx
1400d6189 sub rsp, 0x28
1400d618d lea rbp, [rdx + 0x80]
1400d6194 movzx ebx, byte ptr [rbp + 0xa3e]
1400d619b cmp byte ptr [rbp + 0xa3f], 0
1400d61a2 jne 0x1400d61bc
1400d61a4 test bl, 1
1400d61a7 jne 0x1400d61cd
1400d61a9 cmp qword ptr [rbp + 0xa28], 0
1400d61b1 je 0x1400d61e3
1400d61b3 add rsp, 0x28
1400d61b7 pop rbx
1400d61b8 pop rdi
1400d61b9 pop rsi
1400d61ba pop rbp
1400d61bb ret 
1400d61bc lea rcx, [rbp + 0x630]
1400d61c3 call 0x1400d4d40
1400d61c8 test bl, 1
1400d61cb je 0x1400d61a9
1400d61cd lea rcx, [rbp + 0x8d0]
1400d61d4 call 0x140029880
1400d61d9 cmp qword ptr [rbp + 0xa28], 0
1400d61e1 jne 0x1400d61b3
1400d61e3 lea rcx, [rbp - 0x60]
1400d61e7 call 0x140029880
1400d61ec lea rcx, [rbp + 0x10]
1400d61f0 call 0x140029880
1400d61f5 jmp 0x1400d61b3
1400d61f7 nop word ptr [rax + rax]
1400d6200 mov qword ptr [rsp + 0x10], rdx
1400d6205 push rbp
1400d6206 push rsi
1400d6207 push rdi
1400d6208 push rbx
1400d6209 sub rsp, 0x28
1400d620d lea rbp, [rdx + 0x80]
1400d6214 lea rcx, [rbp + 0x9b0]
1400d621b call 0x140029880
1400d6220 nop 
1400d6221 add rsp, 0x28
1400d6225 pop rbx
1400d6226 pop rdi
1400d6227 pop rsi
1400d6228 pop rbp
1400d6229 ret 
1400d622a nop word ptr [rax + rax]
1400d6230 mov qword ptr [rsp + 0x10], rdx
1400d6235 push rbp
1400d6236 push rsi
1400d6237 push rdi
1400d6238 push rbx
1400d6239 sub rsp, 0x28
1400d623d lea rbp, [rdx + 0x80]
1400d6244 movzx ebx, byte ptr [rbp + 0xa3b]
1400d624b lea rcx, [rbp + 0x780]
1400d6252 call 0x140029880
1400d6257 lea rcx, [rbp + 0x5c0]
1400d625e call 0x140029880
1400d6263 mov byte ptr [rbp + 0xa3d], bl
1400d6269 mov byte ptr [rbp + 0xa3c], 0
1400d6270 add rsp, 0x28
1400d6274 pop rbx
1400d6275 pop rdi
1400d6276 pop rsi
1400d6277 pop rbp
1400d6278 ret 
1400d6279 nop dword ptr [rax]
1400d6280 mov qword ptr [rsp + 0x10], rdx
1400d6285 push rbp
1400d6286 push rsi
1400d6287 push rdi
1400d6288 push rbx
1400d6289 sub rsp, 0x28
1400d628d lea rbp, [rdx + 0x80]
1400d6294 movzx ebx, byte ptr [rbp + 0xa3a]
1400d629b lea rcx, [rbp + 0x550]
1400d62a2 call 0x140029880
1400d62a7 mov byte ptr [rbp + 0xa3b], bl
1400d62ad add rsp, 0x28
1400d62b1 pop rbx
1400d62b2 pop rdi
1400d62b3 pop rsi
1400d62b4 pop rbp
1400d62b5 ret 
1400d62b6 nop word ptr cs:[rax + rax]
1400d62c0 mov qword ptr [rsp + 0x10], rdx
1400d62c5 push rbp
1400d62c6 push rsi
1400d62c7 push rdi
1400d62c8 push rbx
1400d62c9 sub rsp, 0x28
1400d62cd lea rbp, [rdx + 0x80]
1400d62d4 movzx ebx, byte ptr [rbp + 0xa39]
1400d62db lea rcx, [rbp + 0x4e0]
1400d62e2 call 0x140029880
1400d62e7 mov byte ptr [rbp + 0xa3a], bl
1400d62ed add rsp, 0x28
1400d62f1 pop rbx
1400d62f2 pop rdi
1400d62f3 pop rsi
1400d62f4 pop rbp
1400d62f5 ret 
1400d62f6 nop word ptr cs:[rax + rax]
1400d6300 mov qword ptr [rsp + 0x10], rdx
1400d6305 push rbp
1400d6306 push rsi
1400d6307 push rdi
1400d6308 push rbx
1400d6309 sub rsp, 0x28
1400d630d lea rbp, [rdx + 0x80]
1400d6314 movzx ebx, byte ptr [rbp + 0xa38]
1400d631b lea rcx, [rbp + 0x470]
1400d6322 call 0x140029880
1400d6327 mov byte ptr [rbp + 0xa39], bl
1400d632d add rsp, 0x28
1400d6331 pop rbx
1400d6332 pop rdi
1400d6333 pop rsi
1400d6334 pop rbp
1400d6335 ret 
1400d6336 nop word ptr cs:[rax + rax]
1400d6340 mov qword ptr [rsp + 0x10], rdx
1400d6345 push rbp
1400d6346 push rsi
1400d6347 push rdi
1400d6348 push rbx
1400d6349 sub rsp, 0x28
1400d634d lea rbp, [rdx + 0x80]
1400d6354 movzx ebx, byte ptr [rbp + 0xa37]
1400d635b lea rcx, [rbp + 0x400]
1400d6362 call 0x140029880
1400d6367 mov byte ptr [rbp + 0xa38], bl
1400d636d add rsp, 0x28
1400d6371 pop rbx
1400d6372 pop rdi
1400d6373 pop rsi
1400d6374 pop rbp
1400d6375 ret 
1400d6376 nop word ptr cs:[rax + rax]
1400d6380 mov qword ptr [rsp + 0x10], rdx
1400d6385 push rbp
1400d6386 push rsi
1400d6387 push rdi
1400d6388 push rbx
1400d6389 sub rsp, 0x28
1400d638d lea rbp, [rdx + 0x80]
1400d6394 movzx ebx, byte ptr [rbp + 0xa36]
1400d639b lea rcx, [rbp + 0x390]
1400d63a2 call 0x140029880
1400d63a7 mov byte ptr [rbp + 0xa37], bl
1400d63ad add rsp, 0x28
1400d63b1 pop rbx
1400d63b2 pop rdi
1400d63b3 pop rsi
1400d63b4 pop rbp
1400d63b5 ret 
1400d63b6 nop word ptr cs:[rax + rax]
1400d63c0 mov qword ptr [rsp + 0x10], rdx
1400d63c5 push rbp
1400d63c6 push rsi
1400d63c7 push rdi
1400d63c8 push rbx
1400d63c9 sub rsp, 0x28
1400d63cd lea rbp, [rdx + 0x80]
1400d63d4 movzx ebx, byte ptr [rbp + 0xa35]
1400d63db lea rcx, [rbp + 0x320]
1400d63e2 call 0x140029880
1400d63e7 mov byte ptr [rbp + 0xa36], bl
1400d63ed add rsp, 0x28
1400d63f1 pop rbx
1400d63f2 pop rdi
1400d63f3 pop rsi
1400d63f4 pop rbp
1400d63f5 ret 
1400d63f6 nop word ptr cs:[rax + rax]
1400d6400 mov qword ptr [rsp + 0x10], rdx
1400d6405 push rbp
1400d6406 push rsi
1400d6407 push rdi
1400d6408 push rbx
1400d6409 sub rsp, 0x28
1400d640d lea rbp, [rdx + 0x80]
1400d6414 movzx ebx, byte ptr [rbp + 0xa34]
1400d641b lea rcx, [rbp + 0x2b0]
1400d6422 call 0x140029880
1400d6427 mov byte ptr [rbp + 0xa35], bl
1400d642d add rsp, 0x28
1400d6431 pop rbx
1400d6432 pop rdi
1400d6433 pop rsi
1400d6434 pop rbp
1400d6435 ret 
1400d6436 nop word ptr cs:[rax + rax]
1400d6440 mov qword ptr [rsp + 0x10], rdx
1400d6445 push rbp
1400d6446 push rsi
1400d6447 push rdi
1400d6448 push rbx
1400d6449 sub rsp, 0x28
1400d644d lea rbp, [rdx + 0x80]
1400d6454 movzx ebx, byte ptr [rbp + 0xa33]
1400d645b lea rcx, [rbp + 0x240]
1400d6462 call 0x140029880
1400d6467 mov byte ptr [rbp + 0xa34], bl
1400d646d add rsp, 0x28
1400d6471 pop rbx
1400d6472 pop rdi
1400d6473 pop rsi
1400d6474 pop rbp
1400d6475 ret 
1400d6476 nop word ptr cs:[rax + rax]
1400d6480 mov qword ptr [rsp + 0x10], rdx
1400d6485 push rbp
1400d6486 push rsi
1400d6487 push rdi
1400d6488 push rbx
1400d6489 sub rsp, 0x28
1400d648d lea rbp, [rdx + 0x80]
1400d6494 movzx ebx, byte ptr [rbp + 0xa32]
1400d649b lea rcx, [rbp + 0x1d0]
1400d64a2 call 0x140029880
1400d64a7 mov byte ptr [rbp + 0xa33], bl
1400d64ad add rsp, 0x28
1400d64b1 pop rbx
1400d64b2 pop rdi
1400d64b3 pop rsi
1400d64b4 pop rbp
1400d64b5 ret 
1400d64b6 nop word ptr cs:[rax + rax]
1400d64c0 mov qword ptr [rsp + 0x10], rdx
1400d64c5 push rbp
1400d64c6 push rsi
1400d64c7 push rdi
1400d64c8 push rbx
1400d64c9 sub rsp, 0x28
1400d64cd lea rbp, [rdx + 0x80]
1400d64d4 lea rcx, [rbp + 0xf0]
1400d64db call 0x140029880
1400d64e0 nop 
1400d64e1 add rsp, 0x28
1400d64e5 pop rbx
1400d64e6 pop rdi
1400d64e7 pop rsi
1400d64e8 pop rbp
1400d64e9 ret 
1400d64ea nop word ptr [rax + rax]
1400d64f0 mov qword ptr [rsp + 0x10], rdx
1400d64f5 push rbp
1400d64f6 push rsi
1400d64f7 push rdi
1400d64f8 push rbx
1400d64f9 sub rsp, 0x28
1400d64fd lea rbp, [rdx + 0x80]
1400d6504 lea rcx, [rbp + 0x80]
1400d650b call 0x140029880
1400d6510 nop 
1400d6511 add rsp, 0x28
1400d6515 pop rbx
1400d6516 pop rdi
1400d6517 pop rsi
1400d6518 pop rbp
1400d6519 ret 