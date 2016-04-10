use memory::frame::*;

pub mod area;

pub trait FrameAlloc {
    fn alloc(&mut self) -> Option<Frame>;
    fn dealloc(&mut self, frame: Frame);
}
