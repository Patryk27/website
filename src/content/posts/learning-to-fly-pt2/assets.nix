fw:

fw.utils.linkFarmEx "assets" [
  {
    name = "sketches";

    path = fw.utils.renderSketches {
      src = ./assets/sketches.pdf;

      labels = [
        "coding-propagate-1"
        "coding-propagate-2"
        "coding-propagate-3"
        "coding-propagate-4"
        "coding-propagate-5"
        "coding-new-1"
        "coding-new-2"
      ];
    };
  }
] [
  ./assets/rand.webm
]
