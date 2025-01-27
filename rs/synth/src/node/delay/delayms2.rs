use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};
use hashbrown::HashMap;
use dasp_ring_buffer as ring_buffer;
// use dasp_signal::{self as signal, Signal};
// use dasp_interpolate::{
    // Interpolator,
    // sinc::Sinc,
    // linear::Linear,
// };
type Fixed = ring_buffer::Fixed<Vec<f32>>;
// type Bounded = ring_buffer::Bounded<Vec<f32>>;

// enum RingBuffer {
//     Fix(Fixed),
//     Bound(Bounded)
// }

#[derive(Debug, Clone)]
pub struct DelayMs2 {
    buf: Fixed,
    buf2: Fixed,
    sr: usize,
    input_order: Vec<usize>
    // delay_n: usize,
}

impl DelayMs2 {
    pub fn new() -> Self {
        Self { buf: Fixed::from(vec![0.0]), buf2: Fixed::from(vec![0.0]), sr: 44100, input_order: vec![] }
    }
    pub fn delay(self, delay: f32) -> Self {
        let buf; let buf2; let delay_n;
        if delay == 0.0 {
            let maxdelay = 2.;
            delay_n = (maxdelay / 1000. * self.sr as f32 ) as usize;
            buf = Fixed::from(vec![0.0; delay_n]);
            buf2 = Fixed::from(vec![0.0; delay_n]);
        } else {
            delay_n = (delay / 1000. * self.sr as f32) as usize;
            buf = Fixed::from(vec![0.0; delay_n]);
            buf2 = Fixed::from(vec![0.0; delay_n]);
        };
        Self { buf, buf2, ..self}
    }
    
    pub fn sr(self, sr:usize) -> Self {
        Self {sr, ..self}
    }

    impl_to_boxed_nodedata!();
}


impl<const N: usize> Node<N> for DelayMs2 {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        match inputs.len() {
            1 => {
                let main_input = inputs.values_mut().next().unwrap();
                for i in 0..N {
                    output[0][i] = self.buf.push(main_input.buffers()[0][i]);
                    // output[1][i] = self.buf2.push(main_input.buffers()[1][i]);
                }
            },
            2 => {
                let main_input = &inputs[&self.input_order[0]]; // can panic if there is no id
                let ref_input = &inputs[&self.input_order[1]]; // can panic if there is no id

                let mod_buf = &mut ref_input.buffers();
                for i in 0..N {
                    let mut pos = - mod_buf[0][i] / 1000. * self.sr as f32;
                    while pos < 0. {
                        pos += self.buf.len() as f32;
                    };
                    let pos_int = pos.floor() as usize;
                    let pos_frac = pos.fract();
                    output[0][i] = self.buf.get(pos_int) * pos_frac + self.buf.get(pos_int+1) * (1.-pos_frac);
                    output[1][i] = self.buf2.get(pos_int) * pos_frac + self.buf2.get(pos_int+1) * (1.-pos_frac);
                    self.buf.push(main_input.buffers()[0][i]);
                    self.buf2.push(main_input.buffers()[1][i]);
                }
            }
            _ => {return ()}
        }
    }

    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToNumber(pos, value) => {
                match pos {
                    0 => {
                        let delay_n = (value / 1000. * self.sr as f32) as usize;
                        // buf = Fixed::from(vec![0.0; delay_n]);
                        // buf2 = Fixed::from(vec![0.0; delay_n]);
                        self.buf.set_first(delay_n);
                        self.buf2.set_first(delay_n);
                    },
                    _ => {}
                }
            },
            Message::Index(i) => {
                self.input_order.push(i)
            },
            Message::IndexOrder(pos, index) => {
                self.input_order.insert(pos, index)
            },
            Message::ResetOrder => {
                self.input_order.clear();
            },
            _ => {}
        }
    }
}