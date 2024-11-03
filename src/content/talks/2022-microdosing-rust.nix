{
  title = "Microdosing Rust: Why & How to Get Started with AVR?";

  description = ''
    <p>
      AVRs are charming microcontrollers which can survive more than 300 days
      on a single AA battery, lying on anything - from cold ice to sizzling
      stones.
    </p>
    <p>
      They can communicate with both high-level machines such as computers
      and low-level peripherals like humidity meters, which makes them neat,
      satisfying MCUs to play with -- and they can be programmed in Rust!
    </p>
  '';

  tags = [
    "avr"
    "microcontrollers"
    "rust"
  ];

  when = {
    y = 2022;
    m = 6;
    d = 23;
  };

  where = "Rust Wrocław";
  link = "https://youtube.com/watch?v=3o_lzQMLU5Q";

  resources = [
    {
      label = "slides";
      link = "https://files.pwy.io/2022-microdosing-rust.pdf";
    }
    {
      label = "codes";
      link = "https://github.com/Patryk27/talks/tree/main/2022-microdosing-rust/codes";
    }
  ];
}
