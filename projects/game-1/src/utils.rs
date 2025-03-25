use bevy::math::Vec2;
use bevy::prelude::{Camera, GlobalTransform, Single, Window};
use bevy::utils::HashSet;
use std::collections::VecDeque;

pub fn bfs(grid: &Vec<Vec<usize>>, start: (i32, i32), end: (i32, i32)) -> Vec<(i32, i32)> {
    let rows = grid.len();
    let cols = grid[0].len();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();

    q.push_back((start, vec![start]));
    visited.insert(start);

    while !q.is_empty() {
        let (curr, path) = q.pop_front().unwrap();
        if curr == end {
            return path;
        }

        for (dx, dy) in directions.iter() {
            let new_x = curr.0 + dx;
            let new_y = curr.1 + dy;

            if new_x < 0 || new_x >= cols as i32 || new_y < 0 || new_y >= rows as i32 {
                continue;
            }

            if visited.contains(&(new_x, new_y)) || grid[new_y as usize][new_x as usize] == 1 {
                continue;
            }

            visited.insert((new_x, new_y));
            let mut x = path.clone();
            x.push((new_x, new_y));
            q.push_back(((new_x, new_y), x));
        }
    }
    Vec::new()
}

pub fn get_ray_vec(camera: Single<(&Camera, &GlobalTransform)>, window: Single<&Window>) -> Vec2 {
    let (camera, camera_transform) = *camera;
    if let Some(pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        return pos;
    }
    Vec2::ZERO
}

pub fn vec_to_nearest(pos: Vec2, size: f32) -> Vec2 {
    let x = (pos.x / size).round() * size;
    let y = (pos.y / size).round() * size;
    Vec2::new(x, y)
}
