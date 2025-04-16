{
  title = "Marching Events: What does iCalendar have to do with ray marching?";
  body = builtins.readFile ./body.html;
  assets = ./assets;

  tags = [
    "ical"
    "rust"
  ];

  publishedAt = {
    y = 2025;
    m = 4;
    d = 16;
  };
}
