mod bmat_smat;
mod print;

use print::{print_matrix, print_trajectory};
use bmat_smat::{build_bmat, build_smat};

fn main() {

    const C1: f64 = 0.5291772; //[Bohr]*C1 = [Angstrom]
    const AMU_TO_AU: f64 = 1838.6836605;

    let natom = 3;
    let ndim = 3 * natom;

    let mut mass: Vec<f64> = vec![16.0, 1.0, 1.0];
    //mass.iter_mut().for_each(|element| *element *= AMU_TO_AU  ); //from Atomic mass unit (g/mol) to
    
    let tot_mass: f64 = mass.iter().sum();
    println!("tot mass {}", tot_mass);
    println!();

    

    let atom: Vec<String> = vec!["O".to_string(),
                                 "H".to_string(),
                                 "H".to_string()];


    let mut q = vec![0.000000,     0.000000,     0.000000,
                     0.000000,     0.000000,     0.950000,
                     0.895670,     0.000000,    -0.316663];

    //q.iter_mut().for_each(|element| *element /= C1 ); //from Angstrom to Bohr

    let bmat = build_bmat(&mass, &q); 

    println!("b vectors as matrix:");
    print_matrix(&bmat);

    println!();
    print_trajectory(natom, &atom, &q);

    let smat = build_smat(&mass, &q);

    println!();
    println!("S matrix:");
    print_matrix(&smat);
}
