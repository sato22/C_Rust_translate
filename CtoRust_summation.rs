
struct Function{
    // C言語コードを格納する構造体
	label:  String,                // 関数名　String型
	args:   Vec<Variable>,     // 引数　Variable型のベクタ
	body:   String,                // 関数の中身　String型
	returntype:  Option<String>,                // 返り値の型　Noneを扱うためOpriton<String>型 
}

struct Variable{
    // C言語コードを格納する構造体
	label: String,           // 変数名　String型
	vartype:  String,        // 変数の型　String型
}

fn new_function (label: String, args: Vec<Variable>, body: String, returntype: Option<String>) -> Function {
	// Function構造体の初期化
	let f = Function{
				label: label,
				args: args,
				body: body, 
				returntype: returntype
			};
	return f;
}

fn new_variable (label: String, vartype: String) -> Variable {
	// Variable構造体の初期化
	let v = Variable{
				label: label,
				vartype: vartype
			};
	return v;
}

struct Rfunction{
    // Rustコードに変換用の構造体
	label:  String,                  // 関数名　String型
	args:   Vec<Variable>,      // 引数　Variable型のベクタ
	body:   String,                  // 関数の中身　String型
	returntype:  Option<String>,                  // 返り値の型　String型
}

fn new_rfunction(f: Function) -> Rfunction {
	// Rfunction構造体の初期化
	let r = Rfunction{
				label: f.label,
				args: f.args,
				body: f.body, 
				returntype: f.returntype
			};
	return r;
}


fn to_rust(mut f: Rfunction) {
	// C → Rustへ変換後表示

	// 引数の型名をintからi32に変更
	for i in 0..f.args.len() {
		if f.args[i].vartype == "int" {
			f.args[i].vartype = "i32".to_string()
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

	/* 
		返り値の型
		返り値が存在しない場合は処理を行わない
	*/
	if f.returntype != None {
		
		// 戻り値の型名をintからi32に変更
		if f.returntype == Some("int".to_string()) {
			f.returntype = Some("i32".to_string())
		}
		
		print!(" -> {}", f.returntype.unwrap());
	}

	// body（関数の中身）の変換
	print!("{{\n");

	let mut v = Variable{
					label: "initial".to_string(),
					vartype: "initial".to_string(),
				};


	for word in f.body.split_whitespace() {
		// 空白で一単語ずつ区切って処理
        match word {
            "int" => {
            	v.vartype = "i32".to_string()
            	// 構造体にいったん格納→構造体の中身が両方"initial"じゃなくなったら print!("let mut {}: {}",v.label,v.vartype)で表示？
            	// 次の変数名(例の場合はsum)をどうやってとるか
            },
            ";" => {
            	print!(";\n");			// 一文終わったら改行
            },
            /*
            	for文
            	・昇順
            	C:		for (i = 0; i < value; i++)
            	Rust:	for i in 0..value
            	・降順
            	C:		for (i = value; i > 0; i--)
            	Rust:	for i in (0..value+1).rev()		
            	rev()によってイテレータを逆にする。この際，範囲は value-1 value-2 ... となるため(0..value+1)と表記
            	
            	if文
            	()を記述しない
            */
            _ => {
                print!("{}",word);		// CとRustで形式が変わらなければそのまま出力
            }
        }
    }


	print!("}}\n");
	
}

fn main() {
	let a = new_variable("a".to_string(), "int".to_string());    // 変数aを定義
	let b = new_variable("b".to_string(), "int".to_string());    // 変数bを定義
	let c = new_variable("c".to_string(), "int".to_string());
	let variables: Vec<Variable> = vec![a,b,c];
	let func = new_function("summation".to_string(), variables, "int sum ; sum = a + b ; return sum ;".to_string(), None);
	let rfunc = new_rfunction(func);
	to_rust(rfunc);
}
