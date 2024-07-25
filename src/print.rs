pub fn print_matrix(matrix: &[Vec<f64>]) {
    for row in matrix {
        for &element in row {
            //print!("{:11.5e}  ", element);
            print!("{:11.5}  ", element);
        }
        println!(); // Move to the next row
    }
}


pub fn print_trajectory(natom: usize, atom: &[String], q: &[f64]) {
    const C1: f64 = 0.5291772; //[Bohr]*C1 = [Angstrom]

    println!("{}",natom);
    println!("Current strucure in [Å]");
    for j in 0..natom {
        println!("{}  {:>12.5} {:>12.5} {:>12.5}", atom[j], q[3*j]*C1, q[3*j+1]*C1,  q[3*j+2]*C1);
    }
}
