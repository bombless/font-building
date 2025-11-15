use font_macro::{render_text, render_text_debug};

fn main() {
    // 基本使用
    let glyphs: &'static [[u32; 32]] = render_text!("大字");

    // 带大小参数
    let glyphs_small: &'static [[u32; 32]] = render_text!("16号字", 16);

    // 调试版本（会打印点阵）
    render_text_debug!("我要开始发力了");

    for g in glyphs.iter().chain(STATIC_TEXT) {
        for row in g {
            for bit in (0..32).rev() {
                if (row >> bit) & 1 == 1 {
                    print!("██");
                } else {
                    print!("  ");
                }
            }
            println!();
        }
    }

    for row_index in 2..14 {
        for word_index in 0..glyphs_small.len() {
            let row = glyphs_small[word_index][row_index];
            for bit in (18..32).rev() {
                if (row >> bit) & 1 == 1 {
                    print!("██");
                } else {
                    print!("  ");
                }
            }
        }
        println!();
    }
}
// 也可以在const上下文中使用
const STATIC_TEXT: &'static [[u32; 32]] = render_text!("静态文字");
