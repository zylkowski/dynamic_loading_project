pub struct Host<'a> {
    pub funcs: &'a mut Vec<fn() -> i32>,
}
