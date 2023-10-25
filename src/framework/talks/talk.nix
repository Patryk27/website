fw: talkId:

fw.components.page
{
  title = "~/talks/${talkId}";
  layout = "talk";
} ''
  ${
    fw.components.talkItem {
      inherit talkId;
    }
  }
''
