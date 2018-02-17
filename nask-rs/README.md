# 仕様
## DB n 
* prefix(0x), hex, 1bytes 
*             dec, 1bytes
* 文字列: 空文字列有、"で囲む、"の中はa-z,A-Z, など
## DW n
* prefix(0x), hex, 2bytes 
*             dec, 2bytes
## DD n
* prefix(0x), hex, 4bytes
*             dec, 4bytes
## RESB n
* dec, 4bytes
## RESB n - $ 
* n: prefix(0x), hex, 2bytes
* $: 先頭から何bytes目か


# TODO
* コメント
