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

# 違い
* DB a,b,...  a,bには16進数表記しか認めない
* RESB n - $  nには16進数表記で4桁記述するようにした
