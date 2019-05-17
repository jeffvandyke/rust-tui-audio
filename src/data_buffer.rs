use std::collections::VecDeque;

/// A buffer to hold audio data meant for display on the terminal.
#[derive(Debug, PartialEq)]
pub struct DataBuffer {
    buffer: VecDeque<f32>,
}

impl DataBuffer {
    /// Makes a zero-filled circular data buffer of the given size.
    pub fn new(len: usize) -> DataBuffer {
        DataBuffer {
            buffer: VecDeque::from(vec![0.; len]),
        }
    }

    /// Adds the data to the queue.
    ///
    /// The latest data from `buf_data` is pushed to the end of the DataBuffer. If buf_data is
    /// larger than the DataBuffer, only available samples will be used. if buf_data is smaller,
    /// the remaining space is filled with the previous most recent.
    pub fn push_latest_data(&mut self, buf_data: &[f32]) {
        if buf_data.len() < self.buffer.len() {
            let diff = self.buffer.len() - buf_data.len();

            // Shift the preserved end data to the beginning
            for index in 0..diff {
                self.buffer[index] = self.buffer[index + buf_data.len()];
            }

            // fill the remaining data from the buf_data
            for (index, item) in buf_data.iter().enumerate() {
                self.buffer[index + diff] = *item;
            }
        } else {
            let diff = buf_data.len() - self.buffer.len();

            // Fill the latest available data that will fit.

            // TODO: Complicatedness below avoids a for loop lint. Nice experiment, but maybe find
            // a better way to solve?
            let (left, right) = self.buffer.as_mut_slices();
            let buf_data_source = &buf_data[diff..];
            left.copy_from_slice(&buf_data_source[..left.len()]);
            right.copy_from_slice(&buf_data_source[left.len()..]);
        }
    }

    /// Returns the length of the buffer
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Returns an iter from the underlying VecDeque
    pub fn iter(&self) -> std::collections::vec_deque::Iter<f32> {
        self.buffer.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Additional test function
    impl DataBuffer {
        /// Creates a new Vec by copying all the elements in the current vector
        pub fn clone_vec(&self) -> Vec<f32> {
            self.buffer.iter().cloned().collect()
        }
    }

    #[test]
    fn it_initializes_with_zeros() {
        assert_eq!(&DataBuffer::new(4).clone_vec(), &[0., 0., 0., 0.]);
    }

    #[test]
    fn it_pushes_short_data() {
        let mut data = DataBuffer::new(6);
        data.push_latest_data(&[2., 3., 4.]);
        assert_eq!(&data.clone_vec(), &[0., 0., 0., 2., 3., 4.]);
        data.push_latest_data(&[5., 6.]);
        assert_eq!(&data.clone_vec(), &[0., 2., 3., 4., 5., 6.]);
    }

    #[test]
    fn it_pushes_long_data() {
        let mut data = DataBuffer::new(4);
        data.push_latest_data(&[2., 3., 4.]);
        assert_eq!(&data.clone_vec(), &[0., 2., 3., 4.]);
        data.push_latest_data(&[11., 12., 13., 14., 15., 16., 17.]);
        assert_eq!(&data.clone_vec(), &[14., 15., 16., 17.]);
    }
}
