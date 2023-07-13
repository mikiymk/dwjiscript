# JavaScript のリテラル

## null リテラル

null リテラルは値の `null` を表します。

```ebnf
null_literal = "null";
```

## 真偽値リテラル

真偽値リテラルは真偽値を表します。

```ebnf
boolean_literal =
    | "true"
    | "false";
```

`"true"` が真、`"false"` が偽を表します。

## 数値リテラル

数値リテラルは数値を表します。

```ebnf
numeric_literal =
    | integer_literal [ "n" ]
    | float_literal
    | legacy_number_literal;

integer_literal =
    | non_zero_decimal_digits
    | "0b" binary_digits
    | "0o" octal_digits
    | "0x" hex_digits;

float_literal =
    | non_zero_decimal_digits "." [ decimal_digits ] [ float_exponent ]
    | non_zero_decimal_digits float_exponent
    | "." decimal_digits [ float_exponent ];

float_exponent = ( "e" | "E" ) [ "+" | "-" ] decimal_digits;

legacy_number_literal =
    | "0" legacy_octal_digits
    | "0" legacy_decimal_digits
    | "0" legacy_decimal_digits "." [ decimal_digits ] [ float_exponent ]
    | "0" legacy_decimal_digits float_exponent

non_zero_decimal_digits =
    | "0"
    | non_zero_decimal_digit [ [ "_" ] decimal_digits ];

legacy_decimal_digits = [ legacy_octal_digits ] non_octal_digit { decimal_digit };

decimal_digits = decimal_digit { decimal_digit } [ "_" decimal_digits ];

binary_digits = binary_digit { binary_digit } [ "_" binary_digits ];

legacy_octal_digits = octal_digit { octal_digit };

octal_digits = octal_digit { octal_digit } [ "_" octal_digits ];

hex_digits = hex_digit { hex_digit } [ "_" hex_digits ];

binary_digit =
    | "0"
    | "1";

octal_digit =
    | binary_digit
    | "2"
    | "3"
    | "4"
    | "5"
    | "6"
    | "7";

non_octal_digit =
    | "8"
    | "9";

decimal_digit =
    | octal_digit
    | non_octal_digit;

non_zero_decimal_digit =
    | "1"
    | "2"
    | "3"
    | "4"
    | "5"
    | "6"
    | "7"
    | "8"
    | "9";

hex_digit =
    | decimal_digit
    | "a" | "A"
    | "b" | "B"
    | "c" | "C"
    | "d" | "D"
    | "e" | "E"
    | "f" | "F";
```

数値は2進数、8進数、10進数、16進数の整数と10進数の浮動小数点数を表すことができます。

## 文字列リテラル
## 配列リテラル
## オブジェクトリテラル
## テンプレートリテラル
