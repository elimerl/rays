use glam::Vec4;

pub fn reflect(incident: Vec4, normal: Vec4) -> Vec4 {
    incident - 2.0 * incident.dot(normal) * normal
}
