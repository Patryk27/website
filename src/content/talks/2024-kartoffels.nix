{
  title = "kartoffels: Emulating thousands of RISC V CPUs For Fun";

  description = ''
    <p>
      kartoffels is a game where you're given a potato and your job is to
      implement a firmware for it - it's a side project I've been developing for
      a couple of months now and in this talk I'll show you how it works, what
      troubles I've stumbled upon (lots!), what I think went great and why
      I ditched WebAssembly and went for RISC-V instead.
    </p>
  '';

  tags = [
    "game"
    "riscv"
    "rust"
    "webassembly"
  ];

  when = {
    y = 2024;
    m = 11;
    d = 25;
  };

  where = "code::dive";
  link = "https://www.youtube.com/watch?v=vJD4qq8a6QA";

  resources = [
    {
      label = "code";
      link = "https://github.com/Patryk27/kartoffels/";
    }
  ];
}
