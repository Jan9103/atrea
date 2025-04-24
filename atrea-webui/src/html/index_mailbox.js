window.onmessage=(event)=>{
  let msg=JSON.parse(event.data);
  console.log("MAILBOX: "+event.data);
  switch(msg["action"]){
    case "view_channel":
      new WinBox("Channel: "+msg["name"], {
        url: "./box_channel.html?login="+encodeURIComponent(msg["login"]),
        background: "#660",
      });
      break;
    case "show_recs":
      new WinBox("Recommendations ("+msg["alg"]+")", {
        url: "./box_recs.html?algo="+encodeURIComponent(msg["alg"]),
        background: "#066",
      });
      break;
    case "open_help":
      if(!msg["site"].match(/^[a-z]+$/)) {
        console.error("ERROR: invalid help page: "+event.data);
        return;
      }
      new WinBox("Help: "+msg["title"], {
        url: "./box_help_"+msg["site"]+".html",
        background: "#060",
      });
      break;
    case "open_known_viewers":
      new WinBox("Known Viewers", {
        url: "./box_known_viewers.html",
        background: "#606",
      });
      break;
    case "open_liked_channels":
      new WinBox("Liked Channels", {
        url: "./box_recs.html?algo=liked_channels",
        background: "#066",
      });
      break;
    case "show_rel_graph":
      new WinBox("Force Graph", {
        url: "./box_rel_graph.html",
        background: "#006",
        width: "100%",
        height: "100%",
      });
      break;
    default:
      console.error("ERROR: invalid incomming message (unknown action): "+event.data);
      return;
  };
};
