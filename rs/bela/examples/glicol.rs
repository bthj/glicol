extern crate glicol;
use glicol::Engine;
extern crate bela;
use bela::*;

const BLOCK_SIZE: usize = 16;

fn main() {
    run("a: sin 440".to_owned()).unwrap();
    // run(String::from(

    //     "&a: adc 2
        
    //     &m: adc 1 >> mul 900 >> add 1000
                
    //     ~a: sin &m >> mul &a"

    // )).unwrap();
}

fn run(code: String) -> Result<(), error::Error> {
    let mut setup = |context: &mut Context, engine: &mut Engine<BLOCK_SIZE>| -> Result<(), error::Error> {
        
        // engine.make_adc_node(context.analog_in_channels());
        // engine.parse();
        println!("adc frames {}", context.analog_frames());
        println!("adc chan {}", context.analog_in_channels());
        println!("adc {:?} len {}", context.analog_in(), context.analog_in().len());
        println!("audio frames {}", context.audio_frames());
        println!("{}", code);
        engine.update_with_code(&code);
        Ok(())
    };
    let mut cleanup = |_context: &mut Context, _user_data: &mut Engine<BLOCK_SIZE>| {
        println!("Cleaning up");
    };

    let mut render = |context: &mut Context, engine: &mut Engine<BLOCK_SIZE>| {
        // engine.set_adc_node_buffer(&context.analog_in(), 8, BLOCK_SIZE, false);
        let buf = engine.next_block().0;
        for i in 0..BLOCK_SIZE {
            (*context.audio_out())[i] = buf[0][i];
            (*context.audio_out())[i + BLOCK_SIZE] = buf[1][i];
        }
    };

    let engine = Engine::<BLOCK_SIZE>::new();
    let user_data = AppData::new(engine, &mut render, Some(&mut setup), Some(&mut cleanup));
    let mut bela_app = Bela::new(user_data);
    let mut settings = InitSettings::default();
    settings.set_period_size(BLOCK_SIZE);
    settings.set_uniform_sample_rate(true);
    settings.set_interleave(false);
    bela_app.run(&mut settings)
}