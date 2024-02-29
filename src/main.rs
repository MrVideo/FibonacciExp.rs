use std::env;

fn main() {
    let magic_matrix: Vec<Vec<i32>> = vec![
        vec![1, 1],
        vec![1, 0]
    ];

    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        println!("Usage: main n\twhere n is the required index of a Fibonacci number");
        return
    }

    let n: i32 = args[1].parse().unwrap();

    print!("Fibonacci sequence @ {}: ", n);

    if n == 0 {
        print!("{}", 0);
    } else if n == 1 {
        print!("{}", 1);
    } else {
        let mut mat: Vec<Vec<i32>> = vec![
            vec![1, 1],
            vec![1, 0]
        ];

        for _ in 0..n-2 {
            mat = mat_mul(&magic_matrix, &mat);
        }
        
        print!("{}", mat[0][0]);
    }

    println!();
}

fn mat_mul(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![vec![0; 2]; 2];

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                res[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    res
}

