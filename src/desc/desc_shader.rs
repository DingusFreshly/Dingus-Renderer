pub enum ShaderSource<'a> {
    Wgsl(&'a str),
    Handle()
}