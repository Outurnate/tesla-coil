#![no_std]
#![no_main]

use libm::{powf, roundf};
use panic_halt as _;
use midly::{stream::MidiStream, MidiMessage, live::LiveEvent, num::{u7, u14}};
use arduino_hal::{DefaultClock, pac::tc1::tccr1b::CS1_A, clock::Clock};
use embedded_hal::serial::Read;
use stack::Stack;
use void::ResultVoidExt;
use core::{cmp::{min, max}, ops::{Sub, Div}};

// config parameters
const PITCH_BEND_NOTES_UP_OR_DOWN: f32 = 2_f32; // max pitch bend will be two notes up and down

// convert note number (0-127) to the fundamental frequency
fn frequency_for_note_number(note: f32) -> f32
{
  440_f32 * powf(2_f32, (note - 69_f32) / 12_f32)
}

// clamp value to range
fn clamp<T: Ord>(input: T, minimum: T, maximum: T) -> T
{
  max(min(input, maximum), minimum)
}

fn normalize<T: Sub<Output = T> + Div<Output = T> + Copy>(input: T, minimum: T, maximum: T) -> T
{
  let range = maximum - minimum;
  (input - minimum) / range
}

// calculate 16 bit register values
fn hz_and_dc_to_registers<CLOCK: Clock, const PRESCALE: u32>(frequency: f32, duty_cycle: f32) -> (u16, u16)
{
  let reg_a = ((CLOCK::FREQ as f32) / (frequency * (PRESCALE as f32))) - 1_f32;
  let reg_b = (duty_cycle * (reg_a + 1_f32)) - 1_f32;
  let peak = roundf(reg_a) as u16;
  (peak, clamp(roundf(reg_b) as u16, 1, peak - 1))
}

struct SquareWaveSynth<A: FnMut(u16), B: FnMut(u16), C: FnMut(bool), const N: usize>
{
  a: A,
  b: B,
  en: C,
  notes_pressed: Stack<u7, N>,
  bend: u14
}

impl<A: FnMut(u16), B: FnMut(u16), C: FnMut(bool), const N: usize> SquareWaveSynth<A, B, C, N>
{
  fn remove_note(&mut self, note: u7)
  {
    let mut temp_notes: Stack<u7, N> = Stack::new();
    // pop onto the temp stack until we either find the note we want
    // or we reach the bottom of the stack
    while let Some(current_note) = self.notes_pressed.pop()
    {
      if current_note == note
      {
        break;
      }
      else
      {
        temp_notes.push(current_note);
      }
    }

    // return temp notes back to main stack
    while let Some(temp_note) = temp_notes.pop()
    {
      // break early if the main stack is exhausted
      if self.notes_pressed.push(temp_note).is_none()
      {
        break;
      }
    }
  }

  fn update_timer(&mut self)
  {
    if let Some(note) = self.notes_pressed.peek()
    {
      let bend = (normalize(self.bend.as_int() as f32, 0_f32, u14::max_value().as_int() as f32) - 0.5_f32) * 2_f32 * PITCH_BEND_NOTES_UP_OR_DOWN;
      let frequency = frequency_for_note_number(note.as_int() as f32) + bend;
      let (a, b) = hz_and_dc_to_registers::<DefaultClock, 256>(frequency, 0.75_f32);
      (self.a)(a);
      (self.b)(b);
      (self.en)(true);
    }
    else
    {
      (self.en)(false);
    }
  }

  pub fn new(a: A, b: B, en: C) -> Self
  {
    Self
    {
      a,
      b,
      en,
      notes_pressed: Stack::new(),
      bend: u14::new(8192)
    }
  }

  pub fn note_on(&mut self, note: u7)
  {
    self.remove_note(note);
    self.notes_pressed.push(note);
    self.update_timer();
  }

  pub fn note_off(&mut self, note: u7)
  {
    self.remove_note(note);
    self.update_timer();
  }

  pub fn pitch_bend(&mut self, amount: u14)
  {
    self.bend = amount;
    self.update_timer();
  }
}

midly::stack_buffer! { struct LocalBuffer([u8; 1024]); }

#[arduino_hal::entry]
fn main() -> !
{
  let mut stream = MidiStream::with_buffer(LocalBuffer::new());

  let dp = arduino_hal::Peripherals::take().unwrap();
  let pins = arduino_hal::pins!(dp);
  let mut serial = arduino_hal::default_serial!(dp, pins, 115200);

  let timer = dp.TC1;
  // TODO check PWM mode
  timer.tccr1a.write(|w| w.wgm1().bits(0b01) );
  timer.tccr1b.write(|w|
  {
    w.cs1()
      .variant(CS1_A::PRESCALE_256)
      .wgm1().bits(0b10)
  });
  let mut d9 = pins.d9.into_output();
  let mut d10 = pins.d10.into_output();
  let mut synth: SquareWaveSynth<_, _, _, 16> = SquareWaveSynth::new(
    |x| timer.ocr1a.write(|w| unsafe { w.bits(x) }),
    |x| timer.ocr1b.write(|w| unsafe { w.bits(x) }),
    |x| if x
    {
      timer.tccr1a.modify(|_, w| w.com1a().match_clear());
      timer.tccr1a.modify(|_, w| w.com1b().match_clear());
      d9.set_high();
      d10.set_high();
    }
    else
    {
      timer.tccr1a.modify(|_, w| w.com1a().disconnected());
      timer.tccr1a.modify(|_, w| w.com1b().disconnected());
      d9.set_low();
      d10.set_low();
    });

  loop
  {
    let chunk = nb::block!(serial.read()).void_unwrap();
    stream.feed(&[chunk; 1], |event|
    {
      if let LiveEvent::Midi { message, .. } = event
      {
        match message
        {
          MidiMessage::NoteOff { key, .. } => { serial.write_byte(0); synth.note_off(key) },
          MidiMessage::NoteOn { key, vel } =>
          {
            if vel != 0
            {
              serial.write_byte(1);
              synth.note_on(key);
            }
            else
            {
              serial.write_byte(0);
              synth.note_off(key);
            }
          },
          MidiMessage::PitchBend { bend } => synth.pitch_bend(bend.0),
          MidiMessage::Controller { controller, value } => {},
          _ => {}
        }
      }
    });
  }
}
