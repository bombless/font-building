use font_macro::{render_text, render_text_debug};

fn main() {
    // 基本使用
    let glyphs: &'static [[u32; 32]] = render_text!("新闻来了");

    // 带大小参数
    let glyphs_small: &'static [[u32; 32]] = render_text!("你想干嘛", 16);

    // 调试版本（会打印点阵）
    let glyphs_debug: &'static [[u32; 32]] = render_text_debug!("我要开始发力了");

    // 打印第一个字符的点阵
    println!("First character bitmap:");
    for row in &glyphs[0] {
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

// 也可以在const上下文中使用
const STATIC_TEXT: &'static [[u32; 32]] = render_text!("STATIC");
