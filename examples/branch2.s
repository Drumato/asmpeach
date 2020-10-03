	.globl	main
	.type	main, @function
main:
	pushq	%rbp
	movq	%rsp, %rbp
	movq	$0, -4(%rbp)
	jmp	.L2
.L3:
    movq    $1, %rax
	addq	%rax, -4(%rbp)
.L2:
    movq    -4(%rbp), %rax
	cmpq	$10, %rax
	jle	.L3
	movq	-4(%rbp), %rax
	popq	%rbp
	ret
