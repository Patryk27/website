{
  title = "Trace Me a River (v2): Computing stuff on GPU using Rust";
  tags = [ "rust" "gpu" ];
  when = { y = 2024; m = 1; d = 11; };
  where = "Rust Wrocław";
  link = "https://youtu.be/RfAx_pkQFTY?t=4192";

  resources = [
    { label = "code"; link = "https://github.com/Patryk27/sdf-playground"; }
    { label = "slides"; link = "https://files.pwy.io/2024-trace-me-a-river.pdf"; }
  ];

  description = ''
    <p>
      GPUs are known for their abilities of generating pretty images pretty
      fast - in this talk we'll see what makes GPUs different from CPUs and
      we'll see how you can code them in Rust.
    </p>
  '';
}
