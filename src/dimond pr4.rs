fn main() {
    const SIZE: usize = 5; 

    
    let mut diamond = String::new();

    
    for i in 0..SIZE {
        let spaces = SIZE - i - 1;
        let stars = 2 * i + 1;
        diamond += &" ".repeat(spaces);
        diamond += &"*".repeat(stars);
        diamond += "\n";
    }

    
    for i in (0..SIZE - 1).rev() {
        let spaces = SIZE - i - 1;
        let stars = 2 * i + 1;
        diamond += &" ".repeat(spaces);
        diamond += &"*".repeat(stars);
        diamond += "\n";
    }

   
    print!("{}", diamond);
}
