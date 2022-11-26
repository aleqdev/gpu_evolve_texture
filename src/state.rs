#[derive(Default, Eq, PartialEq)]
pub enum EvolveTextureState {
    #[default]
    Loading,
    Init,
    Process,
}
