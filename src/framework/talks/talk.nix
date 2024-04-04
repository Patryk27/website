fw: talkId:

fw.components.page {
  title = fw.content.talks.${talkId}.title;
  layout = "talk";

  body = fw.components.talkItem {
    inherit talkId;
  };
}
