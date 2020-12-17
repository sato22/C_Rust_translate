
struct Function<'a>{
    // C言語コードを格納する構造体
	label:  &'a str,                // 関数名　&str型
	args:   &'a [Variable<'a>],     // 引数　Variable型のスライス
	body:   &'a str,                // 関数の中身　&str型
	rtype:  &'a str,                // 戻り値の型　&str型
}

struct Variable<'b>{
    // C言語コードを格納する構造体
	label: &'b str,           // 変数名　&str型
	vartype:  &'b str,        // 変数の型　&str型
}

fn NewFunction<'a> (label: &'a str, args: &'a [Variable<'a>], body: &'a str, rtype: &'a str) -> Function<'a> {
	// Function構造体の初期化
	let f = Function{label: label,args: args,body: body, rtype: rtype};
	return f;
}

fn NewVariable<'b> (label: &'b str, vartype: &'b str) -> Variable<'b> {
	// Variable構造体の初期化
	let v = Variable{label: label,vartype: vartype};
	return v;
}

struct Rfunction<'c>{
    // Rustコードに変換用の構造体
	label:  String,                  // 関数名　String型
	args:   &'c [Variable<'c>],      // 引数　Variable型のスライス
	body:   String,                  // 関数の中身　String型
	rtype:  String,                  // 戻り値の型　String型
}

fn NewRfunction(f: Function) -> Rfunction {
	// Rfunction構造体の初期化
	let r = Rfunction{label: f.label.to_string(),args: f.args,body: f.body.to_string(), rtype: f.rtype.to_string()};
	return r;
}


fn ToRust(mut f: Rfunction) {
	// C → Rustへ変換後表示

	// 戻り値の型名をintからi32に変更
	if f.rtype == "int" {
		f.rtype = "i32".to_string();
	}

	// 引数の型名をintからi32に変更
	for i in 0..f.args.len() {
		if f.args[i].vartype == "int" {
			f.args[i].vartype = &"i32".to_string();
		}
	}

	// 関数名
	print!("fn {}", f.label);

	// 引数
	for i in 0..f.args.len(){
		if i == 0 {
			print!("(");
		}
		if i == f.args.len()-1 {
			print!("{}: {}", f.args[i].label, f.args[i].vartype);
		} else {
			print!("{}: {},", f.args[i].label, f.args[i].vartype);
		}
		if i == f.args.len()-1 {
			print!(")");
		}

	}

	// 戻り値
	// if f.rtype != nil {
	print!(" -> {}", f.rtype);
	// }

	// 関数の中身
	print!("{{\n  {}\n}}\n", f.body);
}

fn main() {
	let a = NewVariable("a", "int");    // 変数aを定義
	let b = NewVariable("b", "int");    // 変数bを定義
	let c = NewVariable("c", "int");
	let variables: &[Variable] = &[a,b,c];
	let func = NewFunction("summation", variables, "  int sum;\n    sum = a+b;\n    return sum;", "int");       // C言語コードを格納
	let rfunc = NewRfunction(func);
	ToRust(rfunc);
}