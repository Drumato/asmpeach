	.globl	main
	.type	main, @function
main:
	pushq	%rbp
	movq	%rsp, %rbp
	movq	$3, -24(%rbp)
	movq	$5, -16(%rbp)
	movq	-24(%rbp), %rdx
	movq	-16(%rbp), %rax
	addq	%rdx, %rax
	movq	%rax, -8(%rbp)
	movq	-8(%rbp), %rax
	popq	%rbp
	ret