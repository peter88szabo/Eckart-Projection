fn smat_element(which: String, mass: &[f64], q: &[f64]) -> f64 {

    let nat = mass.len();

   // Sign of the mat element is neglected here!!!!!!!!
    let mut mat_elem = 0.0;
    for i in 0..nat {
        let jx = 3 * i; 
        let jy = 3 * i + 1; 
        let jz = 3 * i + 2; 

        match which.as_str() {
            "x"     => mat_elem += mass[i] *  q[jx],
            "y"     => mat_elem += mass[i] *  q[jy],
            "z"     => mat_elem += mass[i] *  q[jz],
            "xy"    => mat_elem += mass[i] *  q[jx] * q[jy],
            "xz"    => mat_elem += mass[i] *  q[jx] * q[jz],
            "yz"    => mat_elem += mass[i] *  q[jy] * q[jz],
            "x2+y2" => mat_elem += mass[i] * (q[jx] * q[jx] + q[jy] * q[jy]),
            "x2+z2" => mat_elem += mass[i] * (q[jx] * q[jx] + q[jz] * q[jz]),
            "y2+z2" => mat_elem += mass[i] * (q[jy] * q[jy] + q[jz] * q[jz]),
             _      => println!("Wrong mode used in smat_element function)"),
        }
    }

    return mat_elem;
}

pub fn build_smat(mass: &[f64], q: &[f64]) -> Vec<Vec<f64>> {
    let mut smat: Vec<Vec<f64>> = vec![vec![0.0; 6]; 6];
    let tot_mass: f64 = mass.iter().sum();

  //S matrix from J. W. McIver J. Chern. Phys., Vol. 88, No.2, 15 January 1988
  //Diagonal elements
    smat[0][0] = tot_mass;
    smat[1][1] = tot_mass;
    smat[2][2] = tot_mass;
    smat[3][3] = smat_element("y2+z2".to_string(), mass, q);
    smat[4][4] = smat_element("x2+z2".to_string(), mass, q);
    smat[5][5] = smat_element("x2+y2".to_string(), mass, q);

  //Off-Diagonals (upper-half matrix):
    smat[0][4] = -1.0 * smat_element("z".to_string(), mass, q);
    smat[0][4] = -1.0 * smat_element("z".to_string(), mass, q);
    smat[0][5] =        smat_element("y".to_string(), mass, q);

    smat[1][3] =        smat_element("z".to_string(), mass, q);
    smat[1][5] = -1.0 * smat_element("x".to_string(), mass, q);

    smat[2][3] = -1.0 * smat_element("y".to_string(), mass, q);
    smat[2][4] =        smat_element("x".to_string(), mass, q);

    smat[3][4] = -1.0 * smat_element("xy".to_string(), mass, q);
    smat[3][5] = -1.0 * smat_element("xz".to_string(), mass, q);

    smat[4][5] = -1.0 * smat_element("yz".to_string(), mass, q);

    return smat;
}

pub fn build_bmat(mass: &[f64], q: &[f64]) -> Vec<Vec<f64>> {
    let ndim = q.len();
    let natom = ndim/3;
    println!("natom = {}", natom);

    let mut bmat: Vec<Vec<f64>> = vec![vec![0.0; 6]; ndim];

   //b(1-6) vectors from J. W. McIver J. Chern. Phys., Vol. 88, No.2, 15 January 1988
    for i in 0..natom{
        bmat[3*i][0]   = mass[i].sqrt(); //b1
        bmat[3*i+1][1] = mass[i].sqrt(); //b2
        bmat[3*i+2][2] = mass[i].sqrt(); //b3

        bmat[3*i+1][3] =  (mass[i].sqrt()) * q[3*i+2]; //b4
        bmat[3*i+2][3] = -(mass[i].sqrt()) * q[3*i+1]; //b4

        bmat[3*i][4]   = -(mass[i].sqrt()) * q[3*i+2]; //b5
        bmat[3*i+2][4] =  (mass[i].sqrt()) * q[3*i];   //b5

        bmat[3*i][5]   =  (mass[i].sqrt()) * q[3*i+1]; //b6
        bmat[3*i+1][5] = -(mass[i].sqrt()) * q[3*i];   //b6
    }

    return bmat;
}
