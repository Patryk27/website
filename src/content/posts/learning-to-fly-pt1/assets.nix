fw:

fw.utils.linkFarmEx "assets" [
  {
    name = "sketches";

    path = fw.utils.renderSketches {
      src = ./assets/sketches.pdf;

      labels = [
        "design-1"
        "design-2"
        "design-3"
        "design-4"
        "brain-1"
        "brain-2"
        "brain-3"
        "nn-1"
        "nn-2"
        "nn-3"
        "nn-4"
        "nn-5"
        "nn-6"
        "nn-7"
        "nn-8"
        "nn-9"
        "nn-10"
        "ga-1"
        "ga-2"
        "ga-3"
        "ga-4"
        "ga-5"
        "ga-6"
        "ga-7"
        "ga-8"
      ];
    };
  }
] [
  ./assets/carrot.jpg
  ./assets/intro-outcome.png
]