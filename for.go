package main

import (
	"fmt"
	"strings"
)

type Function struct {
	label string     // 関数名　string型
	args  []Variable // 変数　Variable型のスライス
	body  string     // 関数の中身　string型
	rtype string     // 戻り値の型　string型
}

type Variable struct {
	label   string // 変数名　string型
	vartype string // 変数の型　string型
}

func NewFunction(label string, args []Variable, body string, rtype string) Function {
	// Function構造体の初期化
	f := Function{}
	f.label = label
	f.args = args
	f.body = body
	f.rtype = rtype
	return f
}

func NewVariable(label string, vartype string) Variable {
	// Variable構造体の初期化
	v := Variable{}
	v.label = label
	v.vartype = vartype
	return v

}

func ToRust(f Function) {
	// go → Rustへ変換後表示

	// 戻り値の型名をintからi32に変更
	if f.rtype == "int" {
		f.rtype = "i32"
	}

	// 引数の型名をintからi32に変更
	for i, s := range f.args {
		if s.vartype == "int" {
			f.args[i].vartype = "i32"
		}
	}

	// for文
	if strings.Contains(f.body, "(i="){
	    strings.Replace(f.body, "(i=", "i in ", -1)
	}
	if strings.Contains(f.body, "i<="){
	    strings.Replace(f.body, "i<=", ".. ", -1)
	}
	if strings.Contains(f.body, "i++"){
	    strings.Replace(f.body, "i++", "", -1)
	}

	// 関数名
	fmt.Printf("fn %s", f.label)

	// 引数
	for i := 0; i < len(f.args); i++ {
		if i == 0 {
			fmt.Printf("(")
		}
		if i == len(f.args)-1{
		    fmt.Printf("%s: %s", f.args[i].label, f.args[i].vartype)
		} else {
		    fmt.Printf("%s: %s,", f.args[i].label, f.args[i].vartype)
		}
		if i == len(f.args)-1 {
			fmt.Printf(")")
		} 
	}

	// 戻り値
	// if f.rtype != nil {
	fmt.Printf(" -> %s", f.rtype)
	// }

	// 関数の中身
	fmt.Printf("{\n  %s\n}\n", f.body)
}

func main() {
	max := NewVariable("max", "int") // 変数maxを定義
	variables := []Variable{max}
	f := NewFunction("sample01", variables, "    int i;\n      int tmp=0;\n    for (i=1;i<=max;i++){\n           tmp = tmp+i;\n  }\n    return tmp;", "int")
	ToRust(f)
}
