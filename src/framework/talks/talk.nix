fw: talkId:

fw.components.page
{
  title = fw.content.talks.${talkId}.title;
  layout = "talk";
} ''
  ${
    fw.components.talkItem {
      inherit talkId;
    }
  }
''
