pub fn mat4_identity() -> [[f32; 4]; 4] {
    [[1.,0.,0.,0.],[0.,1.,0.,0.],[0.,0.,1.,0.],[0.,0.,0.,1.]]
}
pub fn vec3_norm(v: [f32;3]) -> [f32;3] {
    let l = (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt();
    if l < 1e-8 { [0.,1.,0.] } else { [v[0]/l, v[1]/l, v[2]/l] }
}
pub fn vec3_sub(a: [f32;3], b: [f32;3]) -> [f32;3] { [a[0]-b[0], a[1]-b[1], a[2]-b[2]] }

pub fn vec3_dot(a: [f32;3], b: [f32;3]) -> f32 { a[0]*b[0]+a[1]*b[1]+a[2]*b[2] }
pub fn vec3_cross(a: [f32;3], b: [f32;3]) -> [f32;3] {
    [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
}
/// What matrix do you need to reverse this translation
pub fn mat4_inverse(m: [[f32;4];4]) -> [[f32;4];4] {

    let (m00,m01,m02,m03) = (m[0][0],m[0][1],m[0][2],m[0][3]);
    let (m10,m11,m12,m13) = (m[1][0],m[1][1],m[1][2],m[1][3]);
    let (m20,m21,m22,m23) = (m[2][0],m[2][1],m[2][2],m[2][3]);
    let (m30,m31,m32,m33) = (m[3][0],m[3][1],m[3][2],m[3][3]);
    // Lord save me.
    let c00 = m11*(m22*m33-m23*m32) - m12*(m21*m33-m23*m31) + m13*(m21*m32-m22*m31);
    let c01 =-(m10*(m22*m33-m23*m32) - m12*(m20*m33-m23*m30) + m13*(m20*m32-m22*m30));
    let c02 = m10*(m21*m33-m23*m31) - m11*(m20*m33-m23*m30) + m13*(m20*m31-m21*m30);
    let c03 =-(m10*(m21*m32-m22*m31) - m11*(m20*m32-m22*m30) + m12*(m20*m31-m21*m30));
    let c10 =-(m01*(m22*m33-m23*m32) - m02*(m21*m33-m23*m31) + m03*(m21*m32-m22*m31));
    let c11 = m00*(m22*m33-m23*m32) - m02*(m20*m33-m23*m30) + m03*(m20*m32-m22*m30);
    let c12 =-(m00*(m21*m33-m23*m31) - m01*(m20*m33-m23*m30) + m03*(m20*m31-m21*m30));
    let c13 = m00*(m21*m32-m22*m31) - m01*(m20*m32-m22*m30) + m02*(m20*m31-m21*m30);
    let c20 = m01*(m12*m33-m13*m32) - m02*(m11*m33-m13*m31) + m03*(m11*m32-m12*m31);
    let c21 =-(m00*(m12*m33-m13*m32) - m02*(m10*m33-m13*m30) + m03*(m10*m32-m12*m30));
    let c22 = m00*(m11*m33-m13*m31) - m01*(m10*m33-m13*m30) + m03*(m10*m31-m11*m30);
    let c23 =-(m00*(m11*m32-m12*m31) - m01*(m10*m32-m12*m30) + m02*(m10*m31-m11*m30));
    let c30 =-(m01*(m12*m23-m13*m22) - m02*(m11*m23-m13*m21) + m03*(m11*m22-m12*m21));
    let c31 = m00*(m12*m23-m13*m22) - m02*(m10*m23-m13*m20) + m03*(m10*m22-m12*m20);
    let c32 =-(m00*(m11*m23-m13*m21) - m01*(m10*m23-m13*m20) + m03*(m10*m21-m11*m20));
    let c33 = m00*(m11*m22-m12*m21) - m01*(m10*m22-m12*m20) + m02*(m10*m21-m11*m20);

    let det = m00*c00 + m01*c01 + m02*c02 + m03*c03;
    let inv_det = if det.abs() < 1e-8 { 0.0 } else { 1.0 / det };

    [
        [c00*inv_det, c10*inv_det, c20*inv_det, c30*inv_det],
        [c01*inv_det, c11*inv_det, c21*inv_det, c31*inv_det],
        [c02*inv_det, c12*inv_det, c22*inv_det, c32*inv_det],
        [c03*inv_det, c13*inv_det, c23*inv_det, c33*inv_det],
    ]
}
pub fn mat4_mul(a: [[f32;4];4], b: [[f32;4];4]) -> [[f32;4];4] {
    let mut r = [[0f32;4];4];
    for i in 0..4 { for j in 0..4 { for k in 0..4 { r[i][j] += a[i][k]*b[k][j]; } } }
    r
}