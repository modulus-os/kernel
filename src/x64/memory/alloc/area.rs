use memory::alloc::*;

/// Represents a memory area
pub struct Area {
    pub min: u64,
    pub max: u64,
}

impl Area {
    pub fn new(min: u64, max: u64) -> Area {
        Area {
            min: min,
            max: max,
        }
    }
}

/// Allocates page frames for a specific memory area
pub struct AreaFrameAlloc {
    next: Frame,
    area: Area,
}

impl AreaFrameAlloc {
    pub fn new(area: Area) -> AreaFrameAlloc {
        AreaFrameAlloc {
            next: Frame::from_address(area.min),
            area: area,
        }
    }

    pub fn set_area(&mut self, area: Area) {
        self.next = Frame::from_address(area.min);
        self.area = area;
    }

    pub fn first(&self) -> Frame {
        Frame::from_address(self.area.min)
    }

    pub fn last(&self) -> Frame {
        Frame::from_address(self.area.max)
    }
}

impl FrameAlloc for AreaFrameAlloc {
    fn alloc(&mut self) -> Option<Frame> {
        let frame = Frame::from_number(self.next.number);

        if self.next.number < self.last().number {
            self.next.number += 1;
            Some(frame)
        } else {
            None
        }
    }

    fn dealloc(&mut self, frame: Frame) {
        assert!(frame.number > self.first().number);
    }
}
