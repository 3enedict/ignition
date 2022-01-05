use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug, Default)]
pub struct VglRendererParameters {
    #[derivative(Default(value = "false"))]
    pub test: bool,
}
