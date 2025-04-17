pub fn lerp
<V: std::ops::Add<Output = V> + std::ops::Sub<Output = V> + std::ops::Mul<f32, Output = V> + Clone>
(u: V, v: V, t: f32) -> V {
    let result = u.clone() + (v.clone() - u.clone()) * t.clone();
    result
}
