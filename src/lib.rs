#![no_std]
#![feature(maybe_uninit_uninit_array)]
use core::mem::MaybeUninit;

pub struct Queue<T, const N: usize> {
    buffer: [MaybeUninit<T>; N],
    head: usize,
    tail: usize,
}

impl<T, const N: usize> Queue<T, N> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            buffer: MaybeUninit::uninit_array(),
            head: 0,
            tail: 0,
        }
    }

    #[inline]
    pub fn enqueue(&mut self, item: T) {
        self.buffer[self.tail].write(item);
        self.tail = (self.tail + 1) % N;
    }

    #[inline]
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let head = self.head;
        self.head = (self.head + 1) % N;
        Some(unsafe { self.buffer[head].assume_init_read() })
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }
}

impl<T, const N: usize> Default for Queue<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut queue: Queue<u32, 4> = Queue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.dequeue(), None);

        queue.enqueue(4711);
        queue.enqueue(7690);
        assert_eq!(queue.dequeue(), Some(4711));
        assert_eq!(queue.dequeue(), Some(7690));
        assert_eq!(queue.dequeue(), None);

        queue.enqueue(34129);
        queue.enqueue(33833);
        assert_eq!(queue.dequeue(), Some(34129));
        assert_eq!(queue.dequeue(), Some(33833));
        assert_eq!(queue.dequeue(), None);

        queue.enqueue(41272);
        queue.enqueue(26343);
        assert_eq!(queue.dequeue(), Some(41272));
        assert_eq!(queue.dequeue(), Some(26343));
        assert_eq!(queue.dequeue(), None);

        queue.enqueue(45354);
        assert_eq!(queue.dequeue(), Some(45354));
        assert_eq!(queue.dequeue(), None);

        queue.enqueue(24541);
        queue.enqueue(55154);
        queue.enqueue(38290);
        assert_eq!(queue.dequeue(), Some(24541));
        assert_eq!(queue.dequeue(), Some(55154));
        assert_eq!(queue.dequeue(), Some(38290));
        assert_eq!(queue.dequeue(), None);

        queue.enqueue(5996);
        assert_eq!(queue.dequeue(), Some(5996));
        assert_eq!(queue.dequeue(), None);

        queue.enqueue(26769);
        queue.enqueue(64004);
        queue.enqueue(63460);
        assert_eq!(queue.dequeue(), Some(26769));
        assert_eq!(queue.dequeue(), Some(64004));
        assert_eq!(queue.dequeue(), Some(63460));
        assert_eq!(queue.dequeue(), None);
    }
}
