        .globl  main
        .type   main, @function
main:
        endbr64
        pushq   %rbp
        movq    %rsp, %rbp
        movb    $1, -15(%rbp)
        movw    $2, -14(%rbp)
        movl    $3, -12(%rbp)
        movq    $4, -8(%rbp)
        movl    $0, %eax
        popq    %rbp
        ret
