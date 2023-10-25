<?php
$libExtension = (PHP_OS_FAMILY == "Darwin" ? 'dylib' : (PHP_OS_FAMILY == "Windows"?'dll':'so'));

$lib = "target/debug/double_input.$libExtension";

$ffi = FFI::cdef(<<<HEADER
    char * text_generate(char *s,unsigned char n);
    void text_free(char *s);
HEADER,
$lib);
//传入字符串和数字
$rstr = $ffi->text_generate("5555阿里巴巴國美電器，哈哈",5);
//转成php字符串
$result = FFI::string($rstr);

var_dump($result);
?>
