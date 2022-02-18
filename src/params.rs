#[derive(Copy, Clone, Debug, Default)]
pub struct Params {
    pub mean_attraction: f32,
    pub std_attraction: f32,
    pub min_radius_lower: f32,
    pub min_radius_upper: f32,
    pub max_radius_lower: f32,
    pub max_radius_upper: f32,
    pub friction: f32,
    pub wrap: bool,
}
