mod gamedata;
mod renderer;


fn main() {
    let mut gc = crate::renderer::Renderer::new(0.075);
    gc.render().expect("oops");
}
