use ansi_term::Colour::{Fixed, RGB};
use ansi_term::Style;

pub fn print_logo() {
    println!("{}", generate_banner_1());
}

fn generate_banner_1() -> String {
    let colors = [RGB(255, 0, 0), RGB(0, 255, 0), RGB(0, 0, 255)];
    let mut banner = String::new();
    
    let text = r#"
    ██╗  ██╗ █████╗ ███╗   ██╗████████╗██╗
    ╚██╗██╔╝██╔══██╗████╗  ██║╚══██╔══╝██║
     ╚███╔╝ ███████║██╔██╗ ██║   ██║   ██║
     ██╔██╗ ██╔══██║██║╚██╗██║   ██║   ██║
    ██╔╝ ██╗██║  ██║██║ ╚████║   ██║   ██║
    ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝   ╚═╝   ╚═╝
    "#;
    
    for (i, line) in text.lines().enumerate() {
        let color = colors[i % colors.len()];
        banner.push_str(&format!("{}\n", color.paint(line)));
    }
    
    banner
}
