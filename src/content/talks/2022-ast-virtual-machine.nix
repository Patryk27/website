{
  title = "Let's implement a virtual machine! -- from AST to JIT in one hour";

  description = ''
    <p>
      Have you ever wanted to create your own programming language?
    </p>
    <p>
      In this live-coding session I'll show you what is an abstract syntax
      tree and how, based on it, you can create a relatively fast, unsafe-free
      virtual machine that executes your own code.
    </p>
  '';

  tags = [
    "rust"
  ];

  when = {
    y = 2022;
    m = 10;
    d = 6;
  };

  where = "Rust Wrocław";
  link = "https://www.youtube.com/watch?v=ryrOZS-CLyo&t=110s";

  resources = [
    {
      label = "code (1)";
      link = "https://github.com/Patryk27/rast-jit-vm-simple";
    }
    {
      label = "code (2)";
      link = "https://github.com/Patryk27/rast-jit-vm";
    }
  ];
}
