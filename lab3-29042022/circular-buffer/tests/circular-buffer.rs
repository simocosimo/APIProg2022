use circular_buffer::{CircularBuffer, MyCycle, Error};
use std::rc::Rc;

#[test]
fn error_on_read_empty_buffer() {
    let mut buffer = CircularBuffer::<char>::new(1);
    assert_eq!(Err(Error::EmptyBuffer), buffer.read());
}

#[test]
#[ignore]
fn can_read_item_just_written() {
    let mut buffer = CircularBuffer::new(1);
    assert!(buffer.write('1').is_ok());
    assert_eq!(Ok('1'), buffer.read());
}

#[test]
#[ignore]
fn each_item_may_only_be_read_once() {
    let mut buffer = CircularBuffer::new(1);
    assert!(buffer.write('1').is_ok());
    assert_eq!(Ok('1'), buffer.read());
    assert_eq!(Err(Error::EmptyBuffer), buffer.read());
}

#[test]
#[ignore]
fn items_are_read_in_the_order_they_are_written() {
    let mut buffer = CircularBuffer::new(2);
    assert!(buffer.write('1').is_ok());
    assert!(buffer.write('2').is_ok());
    assert_eq!(Ok('1'), buffer.read());
    assert_eq!(Ok('2'), buffer.read());
    assert_eq!(Err(Error::EmptyBuffer), buffer.read());
}

#[test]
#[ignore]
fn full_buffer_cant_be_written_to() {
    let mut buffer = CircularBuffer::new(1);
    assert!(buffer.write('1').is_ok());
    assert_eq!(Err(Error::FullBuffer), buffer.write('2'));
}

#[test]
#[ignore]
fn read_frees_up_capacity_for_another_write() {
    let mut buffer = CircularBuffer::new(1);
    assert!(buffer.write('1').is_ok());
    assert_eq!(Ok('1'), buffer.read());
    assert!(buffer.write('2').is_ok());
    assert_eq!(Ok('2'), buffer.read());
}

#[test]
#[ignore]
fn read_position_is_maintained_even_across_multiple_writes() {
    let mut buffer = CircularBuffer::new(3);
    assert!(buffer.write('1').is_ok());
    assert!(buffer.write('2').is_ok());
    assert_eq!(Ok('1'), buffer.read());
    assert!(buffer.write('3').is_ok());
    assert_eq!(Ok('2'), buffer.read());
    assert_eq!(Ok('3'), buffer.read());
}

#[test]
#[ignore]
fn items_cleared_out_of_buffer_cant_be_read() {
    let mut buffer = CircularBuffer::new(1);
    assert!(buffer.write('1').is_ok());
    buffer.clear();
    assert_eq!(Err(Error::EmptyBuffer), buffer.read());
}

#[test]
#[ignore]
fn clear_frees_up_capacity_for_another_write() {
    let mut buffer = CircularBuffer::new(1);
    assert!(buffer.write('1').is_ok());
    buffer.clear();
    assert!(buffer.write('2').is_ok());
    assert_eq!(Ok('2'), buffer.read());
}

#[test]
#[ignore]
fn clear_does_nothing_on_empty_buffer() {
    let mut buffer = CircularBuffer::new(1);
    buffer.clear();
    assert!(buffer.write('1').is_ok());
    assert_eq!(Ok('1'), buffer.read());
}

#[test]
#[ignore]
fn clear_actually_frees_up_its_elements() {
    let mut buffer = CircularBuffer::new(1);
    let element = Rc::new(());
    assert!(buffer.write(Rc::clone(&element)).is_ok());
    assert_eq!(Rc::strong_count(&element), 2);
    buffer.clear();
    assert_eq!(Rc::strong_count(&element), 1);
}

#[test]
#[ignore]
fn overwrite_acts_like_write_on_non_full_buffer() {
    let mut buffer = CircularBuffer::new(2);
    assert!(buffer.write('1').is_ok());
    buffer.overwrite('2');
    assert_eq!(Ok('1'), buffer.read());
    assert_eq!(Ok('2'), buffer.read());
    assert_eq!(Err(Error::EmptyBuffer), buffer.read());
}

#[test]
#[ignore]
fn overwrite_replaces_the_oldest_item_on_full_buffer() {
    let mut buffer = CircularBuffer::new(2);
    assert!(buffer.write('1').is_ok());
    assert!(buffer.write('2').is_ok());
    buffer.overwrite('A');
    assert_eq!(Ok('2'), buffer.read());
    assert_eq!(Ok('A'), buffer.read());
}

#[test]
#[ignore]
fn overwrite_replaces_the_oldest_item_remaining_in_buffer_following_a_read() {
    let mut buffer = CircularBuffer::new(3);
    assert!(buffer.write('1').is_ok());
    assert!(buffer.write('2').is_ok());
    assert!(buffer.write('3').is_ok());
    assert_eq!(Ok('1'), buffer.read());
    assert!(buffer.write('4').is_ok());
    buffer.overwrite('5');
    assert_eq!(Ok('3'), buffer.read());
    assert_eq!(Ok('4'), buffer.read());
    assert_eq!(Ok('5'), buffer.read());
}

#[test]
#[ignore]
fn integer_buffer() {
    let mut buffer = CircularBuffer::new(2);
    assert!(buffer.write(1).is_ok());
    assert!(buffer.write(2).is_ok());
    assert_eq!(Ok(1), buffer.read());
    assert!(buffer.write(-1).is_ok());
    assert_eq!(Ok(2), buffer.read());
    assert_eq!(Ok(-1), buffer.read());
    assert_eq!(Err(Error::EmptyBuffer), buffer.read());
}

#[test]
#[ignore]
fn string_buffer() {
    let mut buffer = CircularBuffer::new(2);
    buffer.write("".to_string()).unwrap();
    buffer.write("Testing".to_string()).unwrap();
    assert_eq!(0, buffer.read().unwrap().len());
    assert_eq!(Ok("Testing".to_string()), buffer.read());
}

#[test]
#[ignore]
fn empty_iterator() {
    let empty = Vec::<i32>::new();
    let cycle= MyCycle::new(empty.iter(), 4);
    assert_eq!(0, cycle.count());
}

#[test]
#[ignore]
fn recursive_cycle() {
    let v = vec![1, 2, 3, 4];
    let cycle1 = MyCycle::new(v.iter(), 4);
    let cycle2 = MyCycle::new(cycle1, 2);
    assert_eq!((v.len() * 4) * 2, cycle2.count());
}

#[test]
#[ignore]
fn chained_cycles() {
    let v1 = vec![1, 2, 3, 4];
    let v2 = vec![1, 2, 3, 4, 5, 6, 7];
    let cycle1 = MyCycle::new(v1.iter(), 4);
    let cycle2 = MyCycle::new(v2.iter(), 2);
    assert_eq!(v1.len() * 4 + v2.len() * 2, cycle2.chain(cycle1).count());
}

#[test]
#[ignore]
fn zip_cycles() {
    let v1 = vec![1, 3, 5, 7];
    let v2 = vec![2, 4, 6, 8];
    let cycle1 = MyCycle::new(v1.iter(), 2);
    let cycle2 = MyCycle::new(v2.iter(), 2);
    let mut res= Vec::new();
    for (x, y) in cycle1.zip(cycle2) { res.push(*x); res.push(*y); }
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8], res);
}