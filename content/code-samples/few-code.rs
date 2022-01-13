pub trait ReadMessage<M> {
    fn read_message(&mut self) -> Result<M>;
}

pub trait WriteMessage<M> {
    fn write_message(&mut self, message: &M) -> Result<()>;
}
