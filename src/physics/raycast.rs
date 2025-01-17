use glam::Vec2;

pub struct RaycastHit {
    pub point: Vec2,
    pub normal: Vec2,
    pub distance: f32,
}

pub fn raycast(
    origin: Vec2,
    direction: Vec2,
    max_distance: f32,
    pos: Vec2,
    size: Vec2,
) -> Option<RaycastHit> {
    let dir_norm = direction.normalize();
    let half_size = size * 0.5;
    let min = pos - half_size;
    let max = pos + half_size;

    let t1 = (min - origin) / dir_norm;
    let t2 = (max - origin) / dir_norm;

    let tmin = t1.min(t2);
    let tmax = t1.max(t2);

    let enter_time = tmin.x.max(tmin.y);
    let exit_time = tmax.x.min(tmax.y);

    if enter_time > exit_time || enter_time > max_distance || exit_time < 0.0 {
        return None;
    }

    let hit_point = origin + dir_norm * enter_time;
    let normal = if enter_time == tmin.x {
        Vec2::new(if dir_norm.x < 0.0 { 1.0 } else { -1.0 }, 0.0)
    } else {
        Vec2::new(0.0, if dir_norm.y < 0.0 { 1.0 } else { -1.0 })
    };

    Some(RaycastHit {
        point: hit_point,
        normal,
        distance: enter_time,
    })
}
