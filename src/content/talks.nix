{
  "2024-trace-me-a-river-v2" = {
    title = "Trace Me a River (v2): Computing stuff on GPU using Rust";
    when = { y = 2024; m = 1; d = 11; };
    where = "Rust Wrocław";
    link = "https://youtu.be/RfAx_pkQFTY?t=4192";

    resources = [
      { label = "code"; link = "https://github.com/Patryk27/sdf-playground"; }
    ];

    summary = ''
      <p>
        GPUs are known for their abilities of generating pretty images pretty
        fast - in this talk we'll see what makes GPUs different from CPUs and
        we'll see how you can code them in Rust.
      </p>
    '';
  };

  "2023-trace-me-a-river" = {
    title = "Trace Me a River: Computing stuff on GPU using Rust";
    when = { y = 2023; m = 11; d = 16; };
    where = "code::dive";
    link = "https://www.youtube.com/watch?v=npOI15aSAL8";

    resources = [
      { label = "code"; link = "https://github.com/Patryk27/sdf-playground"; }
    ];

    summary = ''
      <p>
        <i>(older version of my <code>Trace Me a River</code> talk)</i>
      </p>
    '';
  };

  "2023-wait-why-pt2" = {
    title = "wait, why (??): Surprising corners of Rust - part 2";
    when = { y = 2023; m = 10; d = 19; };
    where = "Rust Wrocław";
    link = "https://www.youtube.com/watch?v=FRki8MIMgis&t=4720s";

    summary = ''
      <p>
        Even though Rust strives for simplicity, it's got a couple of surprising
        corners and edge cases.
      </p>
      <p>
        In this talk I'm going to show you what makes <code>impl Drop</code>
        super-special, what's the difference between using <code>Self</code>
        and the type's name, what's the deal with <code>#[derive]</code> and
        trait bounds, and many others things that I've stumbled upon.
      </p>
    '';
  };

  "2023-emacs" = {
    title = "(r)IDE on a (t)IDE with my IDE: Why Emacs + Rust?";
    when = { y = 2023; m = 6; d = 22; };
    where = "Rust Wrocław";
    link = "https://www.youtube.com/watch?v=7_oh3QfgVbo&t=5064s";

    summary = ''
      <p>
        Since I spent most of my days looking at a text editor, I've had the
        chance of going through lots of programs, plugins, IDEs and ideas - some
        better, some worse.
      </p>
      <p>
        In this talk I'm going to show you what I've learned and how a typical
        day in my editor looks like. I'll also show you lots of tricks that make
        working with code, Git and filesystem easier, less error-prone and
        more comfortable.
      </p>
    '';
  };

  "2023-wait-why-pt1" = {
    title = "wait, why (??): Surprising corners of Rust - part 1";
    when = { y = 2023; m = 4; d = 13; };
    where = "Rust Wrocław";
    link = "https://www.youtube.com/watch?v=8Q3xKQGEDKA&t=3925s";

    summary = ''
      <p>
        Even though Rust strives for simplicity, it's got a couple of surprising
        corners and edge cases.
      </p>
      <p>
        In this talk I'm going to show you the difference between
        <code>.filter_map()</code> and <code>.flat_map()</code>, what's the deal
        with <code>const FOO: AtomicUsize</code> and many others things that
        I've stumbled upon.
      </p>
    '';
  };

  "2022-doome" = {
    title = "Doomé: What we didn’t know we didn’t know";
    when = { y = 2022; m = 12; d = 1; };
    where = "Rust Wrocław";
    link = "https://www.youtube.com/watch?v=S85Tw0dVtmw&t=5306s";

    resources = [
      { label = "game"; link = "https://dzejkop.itch.io/doome"; }
      { label = "code"; link = "https://github.com/patryk27/doome"; }
    ];

    summary = ''
      <p>
        Together with a friend we've created a game for GitHub's Game Off 2022
        which utilizes software ray-tracing - in this talk we're going through
        our game's internals, describing its most curious & cursed internals.
      </p>
    '';
  };

  "2022-ast-virtual-machine" = {
    title = "Let's implement a virtual machine! -- from AST to JIT in one hour";
    when = { y = 2022; m = 10; d = 6; };
    where = "Rust Wrocław";
    link = "https://www.youtube.com/watch?v=ryrOZS-CLyo&t=110s";

    resources = [
      { label = "code (1)"; link = "https://github.com/Patryk27/rast-jit-vm-simple"; }
      { label = "code (2)"; link = "https://github.com/Patryk27/rast-jit-vm"; }
    ];

    summary = ''
      <p>
        Have you ever wanted to create your own programming language?
      </p>
      <p>
        In this live-coding session I'll show you what is an abstract syntax
        tree and how, based on it, you can create a relatively fast, unsafe-free
        virtual machine that executes your own code.
      </p>
    '';
  };

  "2022-microdosing-rust" = {
    title = "Microdosing Rust: Why & How to Get Started with AVR?";
    when = { y = 2022; m = 6; d = 23; };
    where = "Rust Wrocław";
    link = "https://youtube.com/watch?v=3o_lzQMLU5Q";

    resources = [
      { label = "slides"; link = "https://pwy.io/talks/2022-microdosing-rust.pdf"; }
      { label = "codes"; link = "https://github.com/Patryk27/talks/tree/main/2022-microdosing-rust/codes"; }
    ];

    summary = ''
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
  };

  "2020-cant-hack-this" = {
    title = "Can’t Hack This: A hard-headed introduction to Nix";
    when = { y = 2020; m = 7; d = 3; };
    where = "Functional Programming Wrocław";
    link = "https://www.youtube.com/watch?v=LBrWwZOjsQ4";

    resources = [
      { label = "slides"; link = "https://pwy.io/talks/2020-cant-hack-this.pdf"; }
    ];
  };

  "2020-scary-acronyms" = {
    title = "Scary Acronyms (and Super Creeps): OIBITs, HRTBs and others";
    when = { y = 2020; m = 5; d = 27; };
    where = "Rust Wrocław";
    link = "https://www.youtube.com/watch?v=6Qi5-VU-kS0";

    resources = [
      { label = "slides"; link = "https://pwy.io/talks/2020-scary-acronyms.pdf"; }
    ];
  };

  "2020-fantastic-actors" = {
    title = "Fantastic Actors and Where to Find Them: Actor system from scratch";
    when = { y = 2020; m = 2; d = 27; };
    where = "Rust Wrocław";

    resources = [
      { label = "slides"; link = "https://pwy.io/talks/2020-fantastic-actors.pdf"; }
      { label = "codes"; link = "https://github.com/Patryk27/talks/tree/main/2020-fantastic-actors/codes"; }
    ];
  };
}
