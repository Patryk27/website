fw:

fw.pkgs.linkFarm "assets" [
  {
    name = "sketches";

    path = fw.utils.renderSketches {
      src = ./assets/sketches.pdf;

      labels = [
        "intro-1"
        "coding-selection-1"
        "coding-selection-2"
        "coding-selection-3"
        "coding-selection-4"
        "coding-crossover-1"
        "coding-crossover-2"
        "coding-crossover-3"
        "coding-crossover-4"
        "coding-crossover-5"
        "coding-crossover-6"
        "coding-mutation-1"
        "the-test-1"
      ];
    };
  }
]