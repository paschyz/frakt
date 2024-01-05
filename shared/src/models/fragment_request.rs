#[derive(Debug, Clone)]
pub struct FragmentRequest {
    pub worker_name: String,
    pub maximal_work_load: u32,
}
