#![cfg_attr(not(test), no_std)]

pub struct Stack<T: Copy, const N: usize>
{
  data: [Option<T>; N],
  head: usize
}

impl<T: Copy, const N: usize> Stack<T, N>
{
  pub fn new() -> Self
  {
    Self
    {
      data: [None; N],
      head: 0
    }
  }

  pub fn push(&mut self, item: T) -> Option<T>
  {
    if self.head == N
    {
      Some(item)
    }
    else
    {
      self.data[self.head] = Some(item);
      self.head = self.head + 1;
      None
    }
  }

  pub fn pop(&mut self) -> Option<T>
  {
    if self.head == 0
    {
      None
    }
    else
    {
      self.head = self.head - 1;
      self.data[self.head]
    }
  }

  pub fn peek(&self) -> Option<T>
  {
    if self.head == 0
    {
      None
    }
    else
    {
      self.data[self.head - 1]
    }
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn push()
  {
    let mut stack: Stack<_, 8> = Stack::new();
    assert_eq!(stack.push(1), None);
    assert_eq!(stack.push(2), None);
    assert_eq!(stack.push(3), None);
    assert_eq!(stack.push(4), None);
    assert_eq!(stack.push(5), None);
    assert_eq!(stack.push(6), None);
    assert_eq!(stack.push(7), None);
    assert_eq!(stack.push(8), None);
    assert_eq!(stack.push(9), Some(9));
  }

  #[test]
  fn pop()
  {
    let mut stack: Stack<_, 3> = Stack::new();
    assert_eq!(stack.push(1), None);
    assert_eq!(stack.push(2), None);
    assert_eq!(stack.push(3), None);
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
  }

  #[test]
  fn pop_and_push_dry()
  {
    let mut stack: Stack<_, 3> = Stack::new();
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.push(1), None);
    assert_eq!(stack.push(2), None);
    assert_eq!(stack.push(3), None);
    assert_eq!(stack.push(4), Some(4));
  }

  #[test]
  fn peek()
  {
    let mut stack: Stack<_, 3> = Stack::new();
    assert_eq!(stack.peek(), None);
    assert_eq!(stack.push(1), None);
    assert_eq!(stack.peek(), Some(1));
    assert_eq!(stack.push(2), None);
    assert_eq!(stack.peek(), Some(2));
    assert_eq!(stack.push(3), None);
    assert_eq!(stack.peek(), Some(3));
    assert_eq!(stack.push(4), Some(4));
    assert_eq!(stack.peek(), Some(3));
  }
}
