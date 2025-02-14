fw: talkId:

fw.components.page {
  id = "talk-${talkId}";
  title = fw.content.talks.${talkId}.title;
  layout = "talk";

  body = fw.components.talkItem {
    inherit talkId;
  };
}
