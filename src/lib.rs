use proc_macro::TokenStream;
use quote::quote;
use rusttype::{point, Font, Scale};
use std::fs::File;
use std::io::Read;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, LitStr, Result, Token,
};

// 宏参数结构
struct RenderTextArgs {
    text: LitStr,
    size: Option<usize>,
}

impl Parse for RenderTextArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let text = input.parse::<LitStr>()?;

        let size = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            Some(input.parse::<syn::LitInt>()?.base10_parse::<usize>()?)
        } else {
            None
        };

        Ok(RenderTextArgs { text, size })
    }
}

#[proc_macro]
pub fn render_text(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as RenderTextArgs);
    let text = args.text.value();
    let size = args.size.unwrap_or(32);

    // 生成点阵数据
    let glyphs_data = generate_bitmap_data_sized(&text, size);

    // 为每个字符生成一个32行的数组
    let glyph_arrays = glyphs_data.iter().map(|glyph| {
        let rows = glyph.iter().map(|&row| {
            quote! { #row }
        });
        quote! { [#(#rows),*] }
    });

    // 生成最终的静态数组
    let expanded = quote! {
        {
            const GLYPHS: &'static [[u32; 32]] = &[
                #(#glyph_arrays),*
            ];
            GLYPHS
        }
    };

    TokenStream::from(expanded)
}

fn generate_bitmap_data_sized(text: &str, font_size: usize) -> Vec<[u32; 32]> {
    // 嵌入默认字体或从环境变量获取路径
    let mut file = File::open("./WenQuanYiMicroHei.ttf").unwrap();
    let buffer = &mut Vec::new();
    file.read_to_end(buffer).expect("Failed to read font file");
    let font = Font::try_from_bytes(buffer).expect("Failed to load font");

    let scale = Scale::uniform(font_size as f32);
    let v_metrics = font.v_metrics(scale);

    let mut result = Vec::new();

    for ch in text.chars() {
        let mut bitmap = [0u32; 32];

        // 设置起始位置，使字形居中
        let offset = point(0.0, v_metrics.ascent);

        let glyph = font.glyph(ch).scaled(scale).positioned(offset);

        // 渲染字形
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                // 计算在32x32网格中的位置
                let px = (bb.min.x + x as i32) as usize;
                let py = (bb.min.y + y as i32) as usize;

                if px < 32 && py < 32 {
                    // v是覆盖度，范围0.0到1.0
                    if v > 0.55 {
                        // 设置对应的位
                        bitmap[py] |= 1 << (31 - px);
                    }
                }
            });
        }

        result.push(bitmap);
    }

    result
}

// 用于调试的辅助宏
#[proc_macro]
pub fn render_text_debug(input: TokenStream) -> TokenStream {
    let lit_str = parse_macro_input!(input as LitStr);
    let text = lit_str.value();

    let glyphs_data = generate_bitmap_data_sized(&text, 24);

    // 生成可打印的调试信息
    let debug_strings: Vec<String> = glyphs_data
        .iter()
        .enumerate()
        .map(|(idx, glyph)| {
            let ch = text.chars().nth(idx).unwrap_or('?');
            let mut output = format!("Character '{}' bitmap:\n", ch);
            for row in glyph.iter() {
                for bit in (0..32).rev() {
                    if (row >> bit) & 1 == 1 {
                        output.push('█');
                        output.push('█');
                    } else {
                        output.push(' ');
                        output.push(' ');
                    }
                }
                output.push('\n');
            }
            output
        })
        .collect();

    let arrays = glyphs_data.iter().map(|glyph| {
        let rows = glyph.iter().map(|row| {
            quote! { #row }
        });
        quote! { [#(#rows),*] }
    });

    let expanded = quote! {
        {
            #(println!("{}", #debug_strings);)*

            const GLYPHS: &'static [[u32; 32]] = &[
                #(#arrays),*
            ];
            GLYPHS
        }
    };

    TokenStream::from(expanded)
}
