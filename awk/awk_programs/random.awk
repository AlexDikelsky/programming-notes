awk ' #random number generator
BEGIN { for (i=1; i<=200; i++)
    print int(101*rand())
}
'