SECTIONS
{
  /*textセクションのメモリ上での開始位置*/
  . = 0x280000;
  .text : 
  {
    /*textセクションを全部*/
    *(.text);
    /*textのサイズをシンボルに代入*/
    _b_tsize = SIZEOF(.text);
  }

  /*dataセクションのメモリ上での開始位置*/
  .data 0x310000 :

  /*dataセクションのファイル上の開始位置*/
  AT ( ADDR(.text) + SIZEOF(.text))
  {
    /*.dataのスタートアドレスをシンボルに代入*/
    _b_dstart = SIZEOF(.text) ;
    /*.dataセクションを全部*/
    *(.data);
    /*.dataのサイズをシンボルに代入*/
    _b_dsize = SIZEOF(.data);
  }

  /*bssセクションのメモリ上での開始位置
  ディフォルトではdataの終端から*/
  .bss ( 0x310000 + SIZEOF (.data) ) :
  {
    *(.bss);
    _b_bsssize = SIZEOF(.bss);
  }
}
