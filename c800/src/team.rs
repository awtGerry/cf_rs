use std::io;

pub fn _a() {
    let mut linea = String::new();
    std::io::stdin()
        .read_line(&mut linea)
        .unwrap();

    let num:u16 = linea.trim().parse().unwrap();

    let mut resultado = 0;

    for _ in 0..num {
        let mut nums = String::new();
        io::stdin().read_line(&mut nums).unwrap();
        let tmp:Vec<u32> = nums.trim().
            split(' ').into_iter().map(|e| e.parse().unwrap()).collect();

        let mut cont = 0;
        for i in tmp {
            cont+=i;
        }

        resultado+=(cont >= 2) as u16;
    }

    print!("{resultado}");
}
