#[macro_use]
extern crate proc_macro_hack;

#[macro_use]
extern crate quote;

extern crate guid_parser;
extern crate chomp;
extern crate syn;

use guid_parser::chunks;
use chomp::parse_only;
use syn::{Expr, ExprLit, Lit};

proc_macro_expr_impl! {
    pub fn guid_parts_impl(input: &str) -> String {
        let expr = syn::parse_str::<Expr>(input).unwrap();
        if let &Expr::Lit(ExprLit { lit: Lit::Str(ref lit), .. }) = &expr {
            let parts = parse_only(chunks, lit.value().as_bytes()).unwrap().to_parts();

            let data1: u32 = parts.0;
            let data2: u16 = parts.1;
            let data3: u16 = parts.2;

            let data4_0: u8 = parts.3[0];
            let data4_1: u8 = parts.3[1];
            let data4_2: u8 = parts.3[2];
            let data4_3: u8 = parts.3[3];
            let data4_4: u8 = parts.3[4];
            let data4_5: u8 = parts.3[5];
            let data4_6: u8 = parts.3[6];
            let data4_7: u8 = parts.3[7];

            (quote! {
                (#data1 as u32,
                 #data2 as u16,
                 #data3 as u16,
                 [ #data4_0 as u8,
                   #data4_1 as u8,
                   #data4_2 as u8,
                   #data4_3 as u8,
                   #data4_4 as u8,
                   #data4_5 as u8,
                   #data4_6 as u8,
                   #data4_7 as u8 ])
            }).to_string()
        } else {
            panic!("illegal guid expr (expected string literal)");
        }
        
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
