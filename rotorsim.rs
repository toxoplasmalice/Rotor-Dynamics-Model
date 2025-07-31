//Rotordynamics simulation

use nalgebra::{Matrix4};

let tau: f64 = 6.283185307179586; // tau is 2*Pi
let g: f64 = 9.81; // acceleration due to gravity m/s^2
let 


fn compute_model(){

}

#[derive(Debug, Clone, Copy)]
pub struct CrossSection {
    pub area: f64,            // A
    pub area_moment_inertia_y: f64, // Iy (for bending about y-axis)
    pub area_moment_inertia_z: f64, // Iz (for bending about z-axis)
    pub polar_moment_inertia: f64, // J (for torsion)
    pub shear_correction_factor_y: f64, // kappa_y
    pub shear_correction_factor_z: f64, // kappa_z
}

#[derive(Debug, Clone, Copy)]
pub struct material{
    pub young_modulus: f64, // E
    pub shear_modulus: f64, // G
    pub density: f64,       // rho
}

#[derive(Debug, Clone, Copy)]
pub struct node{
    pub id: usize,
    pub x: f64, // meters
    pub y: f64, // meters
    pub z: f64, // meters
}

#[derive(Debug, Clone, Copy)]
pub struct element {
    pub id: usize,
    pub node_i: usize, // Start node ID
    pub node_j: usize, // End node ID
    pub length: f64, // meters
    pub material: Material,
    pub cross_section: CrossSection,
}

let circular_cs = CrossSection {
    area: std::f64::consts::PI * 0.025_f64.powi(2), // Radius 2.5 cm
    area_moment_inertia_y: std::f64::consts::PI / 4.0 * 0.025_f64.powi(4),
    area_moment_inertia_z: std::f64::consts::PI / 4.0 * 0.025_f64.powi(4),
    polar_moment_inertia: std::f64::consts::PI / 2.0 * 0.025_f64.powi(4),
    shear_correction_factor_y: 10.0 / 9.0, // Common for circular cross-section
    shear_correction_factor_z: 10.0 / 9.0,
};

let 

impl beam_element_2d_timoshenko{
    fn calculate_stiffness_matrix_2d(&self)->{
        let e = self.material.young_modulus;
        let g = self.material.shear_modulus;
        let l = self.length;
        let iy = self.cross_section.area_moment_inertia_y;
        let a = self.cross_section.area;
        let kappa_y = self.cross_section.shear_correction_factor_y;

        // Parameter to account for shear deformation
        let phi = (12.0 * e * iy) / (kappa_y * g * a * l * l);

        let c1 = e * iy / ((1.0 + phi) * l.powi(3));
        let c2 = e * iy / ((1.0 + phi) * l.powi(2));
        let c3 = e * iy / ((1.0 + phi) * l);

        #[allow(unused_mut)]
        let mut k_e = Matrix4::zeros();

        // This is a highly simplified and conceptual representation.
        // The actual Timoshenko stiffness matrix elements are quite involved.
        // Example for bending in Y-Z plane (v_y, theta_z)
        k_e[(0,0)] = 12.0 * c1;
        k_e[(0,1)] = 6.0 * c2;
        k_e[(0,2)] = -12.0 * c1;
        k_e[(0,3)] = 6.0 * c2;

        k_e[(1,0)] = 6.0 * c2;
        k_e[(1,1)] = (4.0 + phi) * c3;
        k_e[(1,2)] = -6.0 * c2;
        k_e[(1,3)] = (2.0 - phi) * c3;

        k_e[(2,0)] = -12.0 * c1;
        k_e[(2,1)] = -6.0 * c2;
        k_e[(2,2)] = 12.0 * c1;
        k_e[(2,3)] = -6.0 * c2;

        k_e[(3,0)] = 6.0 * c2;
        k_e[(3,1)] = (2.0 - phi) * c3;
        k_e[(3,2)] = -6.0 * c2;
        k_e[(3,3)] = (4.0 + phi) * c3;

        k_e
    }

    fn calculate_mass_matrix_2d(&self){
        let rho = self.material.density;
        let a = self.cross_section.area;
        let l = self.length;
        let iy = self.cross_section.area_moment_inertia_y; // Rotational inertia

        let m = rho * a * l / 420.0;
        let i = rho * iy * l / 30.0; // Simplified rotational inertia term

        #[allow(unused_mut)]
        let mut m_e = Matrix4::zeros();

        // Simplified consistent mass matrix for 2D Timoshenko beam
        // These values need to be derived properly.
        m_e[(0,0)] = 156.0 * m;
        m_e[(0,1)] = 22.0 * l * m;
        m_e[(0,2)] = 54.0 * m;
        m_e[(0,3)] = -13.0 * l * m;

        m_e[(1,0)] = 22.0 * l * m;
        m_e[(1,1)] = (4.0 * l.powi(2) * m) + (36.0 * i);
        m_e[(1,2)] = 13.0 * l * m;
        m_e[(1,3)] = (-3.0 * l.powi(2) * m) + (-18.0 * i);

        m_e[(2,0)] = 54.0 * m;
        m_e[(2,1)] = 13.0 * l * m;
        m_e[(2,2)] = 156.0 * m;
        m_e[(2,3)] = -22.0 * l * m;

        m_e[(3,0)] = -13.0 * l * m;
        m_e[(3,1)] = (-3.0 * l.powi(2) * m) + (-18.0 * i);
        m_e[(3,2)] = -22.0 * l * m;
        m_e[(3,3)] = (4.0 * l.powi(2) * m) + (36.0 * i);
    }

    // Gyroscopic matrix (simplified for 2D). For 3D, this is more complex.
    // This matrix couples rotational velocities.
    // DoF order: [v_y_i, theta_z_i, v_y_j, theta_z_j]
    pub fn calculate_gyroscopic_matrix_2d(&self) -> Matrix4<f64> {
        let rho = self.material.density;
        let iy = self.cross_section.area_moment_inertia_y; // Moment of inertia for gyroscopic effect
        let l = self.length;

        let g_val = rho * iy * l / 30.0; // Proportional to rotational inertia

        #[allow(unused_mut)]
        let mut g_e = Matrix4::zeros();

        // Gyroscopic terms for theta_z (rotation about Z-axis for Y-Z bending)
        // Note: The gyroscopic matrix for a single beam element is not typically
        // calculated this way in simplified 2D. It's more relevant when considering
        // the 3D coupling between bending planes.
        // For a full rotor, you'd consider the rotational speed Omega.
        // If we consider a 2D gyroscopic effect linking rotations about Z axis
        // and its effect on Y-motion:
        // This is a highly conceptual and likely incorrect simplified 2D gyroscopic matrix.
        // In full 3D, gyroscopic effects couple rotations about perpendicular axes.
        // It's often expressed as proportional to the rotational inertia and angular velocity.

        g_e[(1,0)] = -g_val; // This would cause coupling
        g_e[(0,1)] = g_val;

        g_e[(3,2)] = -g_val;
        g_e[(2,3)] = g_val;
        // In a real 3D rotor, gyroscopic terms are far more intricate, coupling
        // translational and rotational DoFs across different axes.
        // Example: G_ij = Omega * (I_z - I_y) for certain DoF combinations.

        g_e
    }
}

fn gen_

fn gen_A_matrix(){
    println!("{}",,)

}

fn gen_B_matrix(){

}

fn gen_C_matrix(){

}

fn gen_G_matrix(){

}