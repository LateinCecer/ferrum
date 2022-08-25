use ferrum::lang::expr::*;
use ferrum::lang::expr::Expr::Identifier;

fn main() -> Result<(), String> {
    let input = r#"
        {
            let alpha = (1.0 + 2.0_f64) / 3.0_e-2_f32 * if true { .2 } else { 3u8 };
            let a = { 0 + 1 };

            if a > 0 && true {
                let v = 32;
                4 + 2
            } else if true {
                let b = 42;
                2 + 3
            }

            let b = if a < 2 { 42 } else { 32 };

            let data = [0; 32];
            for i in 0..data.len() {
                print(data[i]);
            }


            let msg = "Hello World!";
            let some_char = '@';
            std::print(msg);

            let mut counter = 0;
            while true {
                counter += 1;
                print(counter);

                if (counter > 1000) && (counter % 3 == 0) {
                    std::io.print("Hello World!")?;
                    break;
                }
            }

            (a + 1 << 2, (-3).pow(3));
            let r = &counter;
            let c = 4 & 4;
        }
    "#;

    let fn_declare = r#"
        fn foo(a: f32, mut b: &mut f32) -> (std::string::String, u32) {
            *b *= a;
            let len = *b;
            let ty = MyStruct { name: "Some", len };

            match ty {
                MyStruct("Hello World!", Some((num, 32))) => print("Hello World!")
                MyStruct(name, 13) => print("{} = 13", name)
                a => print("Anything goes {}", a)
            };

            (ty.to_string(), 3)
        }
    "#;

    let trait_declare = r#"
        trait Mul {
            fn mul(lhs: f32, rhs: f32) -> f32;
            fn mul_assign(lhs: &mut f32, rhs: f32);
        }
    "#;

    let struct_declare = r#"
        struct MyData {
            name: String,
            len: u32,
        }
    "#;

    let enum_declare = r#"
        enum Option {
            Some(Data),
            None
        }
    "#;

    // let block = parser::fn_block(input).map_err(|e| e.to_string())?;
    // println!("Block: {:#?}", block);

    let fn_header = parser::function(fn_declare).map_err(|e| e.to_string());
    println!("Function: {:#?}", fn_header);

    // let trait_declare = parser::tr(trait_declare).map_err(|e| e.to_string());
    // println!("Trait: {:#?}", trait_declare);

    // let struct_declare = parser::struct_def(struct_declare).map_err(|e| e.to_string());
    // println!("Struct: {:#?}", struct_declare);

    // let enum_declare = parser::enum_def(enum_declare).map_err(|e| e.to_string());
    // println!("Enum: {:#?}", enum_declare);


    Ok(())
}