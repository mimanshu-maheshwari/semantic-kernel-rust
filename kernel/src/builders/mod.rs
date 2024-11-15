pub trait SemanticKernelBuilder<T> {
    fn build(&self) -> T;
}
